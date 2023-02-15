#![warn(warnings)]

use clap::Parser;

type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("Package {0} not found")]
    NotFound(String),
    #[error("Unable to read cargo metadata: {0}")]
    Metadata(#[from] cargo_metadata::Error),
    #[error("{0}")]
    TabWriter(#[from] tabwriter::IntoInnerError<tabwriter::TabWriter<Vec<u8>>>),
    #[error("{0}")]
    Utf8(#[from] std::string::FromUtf8Error),
}

#[derive(Parser)]
#[command(name = "cargo", bin_name = "cargo")]
enum Opt {
    Brief(Info),
}

#[derive(Parser)]
struct Info {
    #[arg(default_value = "*")]
    package: String,
    #[arg(long, default_value = "./Cargo.toml")]
    manifest_path: std::path::PathBuf,
    #[arg(long)]
    no_dev: bool,
    #[arg(long, short)]
    recursive: bool,
}

fn main() -> Result {
    let Opt::Brief(opt) = Opt::parse();
    let metadata = cargo_metadata::MetadataCommand::new()
        .manifest_path(&opt.manifest_path)
        .exec()?;
    let mut total = 0;
    let root = root(&metadata).map(|x| vec![x.clone()]).unwrap_or_default();
    let recursive = opt.recursive || root.is_empty();

    let members = if recursive {
        &metadata.workspace_members
    } else {
        &root
    };

    for workspace_member in members {
        let packages = member(&opt, &metadata, workspace_member);

        if packages.is_empty() {
            continue;
        }

        total += packages.len();

        if recursive {
            println!("# {workspace_member}\n");
        }

        if recursive || packages.len() > 1 {
            display_list(&packages)?;
        } else {
            display_one(packages[0])?;
        }

        if recursive {
            println!();
        }
    }

    if total == 0 && opt.package != "*" {
        Err(Error::NotFound(opt.package))
    } else {
        Ok(())
    }
}

fn root(metadata: &cargo_metadata::Metadata) -> Option<&cargo_metadata::PackageId> {
    metadata.resolve.as_ref().unwrap().root.as_ref()
}

fn member<'a>(
    opt: &Info,
    metadata: &'a cargo_metadata::Metadata,
    root: &cargo_metadata::PackageId,
) -> Vec<&'a cargo_metadata::Package> {
    let wildmatch = wildmatch::WildMatch::new(&opt.package);

    let Some(dependencies) = dependencies(metadata, root) else {
        return Vec::new();
    };

    dependencies
        .iter()
        .filter(|x| !opt.no_dev || !dev_only(x))
        .map(|x| package(metadata, &x.pkg).unwrap())
        .filter(|x| wildmatch.matches(&x.name))
        .collect::<Vec<_>>()
}

fn dependencies<'a>(
    metadata: &'a cargo_metadata::Metadata,
    id: &cargo_metadata::PackageId,
) -> Option<&'a Vec<cargo_metadata::NodeDep>> {
    metadata
        .resolve
        .as_ref()
        .unwrap()
        .nodes
        .iter()
        .find(|node| &node.id == id)
        .map(|node| &node.deps)
}

fn package<'a>(
    metadata: &'a cargo_metadata::Metadata,
    id: &cargo_metadata::PackageId,
) -> Option<&'a cargo_metadata::Package> {
    metadata.packages.iter().find(|package| &package.id == id)
}

fn dev_only(dependency: &cargo_metadata::NodeDep) -> bool {
    dependency
        .dep_kinds
        .iter()
        .map(|x| x.kind)
        .collect::<Vec<_>>()
        == vec![cargo_metadata::DependencyKind::Development]
}

fn display_list(packages: &[&cargo_metadata::Package]) -> Result {
    use std::io::Write;

    let mut table = tabwriter::TabWriter::new(Vec::new());

    for package in packages {
        let summary = package
            .description
            .as_ref()
            .map(|x| {
                let mut lines = x.lines();
                let line = lines.next().unwrap_or_default();

                if lines.count() > 0 {
                    format!("{line}…")
                } else {
                    line.to_string()
                }
            })
            .unwrap_or_default();

        let row = format!("{}\t{}\t{}\n", package.name, package.version, summary,);
        table.write_all(row.as_bytes())?;
    }

    let s = String::from_utf8(table.into_inner()?)?;
    print!("{s}");

    Ok(())
}

fn display_one(package: &cargo_metadata::Package) -> Result {
    use std::io::Write;

    let mut table = tabwriter::TabWriter::new(Vec::new());

    table.write_all(&row("name", Some(&package.name)))?;
    table.write_all(&row("descrip.", package.description.as_ref()))?;
    table.write_all(&row("keywords", Some(&package.keywords.join(", "))))?;
    table.write_all(&row("categories", Some(&package.categories.join(", "))))?;
    table.write_all(&row("version", Some(&package.version)))?;
    table.write_all(&row("license", package.license.as_ref()))?;
    table.write_all(&row("homepage", package.homepage.as_ref()))?;
    table.write_all(&row("repository", package.repository.as_ref()))?;
    let features = package.features.keys().cloned().collect::<Vec<_>>();
    table.write_all(&row("features", Some(&features.join(", "))))?;

    let s = String::from_utf8(table.into_inner()?)?;
    print!("{s}");

    Ok(())
}

fn row<S: ToString>(key: &str, value: Option<&S>) -> Vec<u8> {
    format!(
        "{}\t: {}\n",
        ansi_term::Colour::Green.paint(key),
        value.map(ToString::to_string).unwrap_or_default(),
    )
    .as_bytes()
    .to_vec()
}
