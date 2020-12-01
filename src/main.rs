use structopt::StructOpt;

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

#[derive(StructOpt)]
#[structopt(name = "cargo", bin_name = "cargo")]
enum Opt {
    Info(Info),
}

#[derive(StructOpt)]
struct Info {
    #[structopt(default_value = "*")]
    package: String,
    #[structopt(long, default_value = "./Cargo.toml")]
    manifest_path: String,
    #[structopt(long)]
    no_dev: bool,
}

fn main() -> Result {
    let Opt::Info(opt) = Opt::from_args();
    let metadata = cargo_metadata::MetadataCommand::new()
        .manifest_path(&opt.manifest_path)
        .exec()?;
    let mut total = 0;

    for workspace_member in &metadata.workspace_members {
        let packages = member(&opt, &metadata, &workspace_member);

        if packages.is_empty() {
            continue;
        }

        total += packages.len();

        if metadata.workspace_members.len() > 1 {
            println!("# {}\n", workspace_member);
        }

        if packages.len() > 1 || metadata.workspace_members.len() > 1 {
            display_list(&packages)?;
        } else {
            display_one(&packages[0])?;
        }

        if metadata.workspace_members.len() > 1 {
            println!();
        }
    }

    if total == 0 {
        Err(Error::NotFound(opt.package))
    } else {
        Ok(())
    }
}

fn member<'a>(
    opt: &Info,
    metadata: &'a cargo_metadata::Metadata,
    root: &cargo_metadata::PackageId,
) -> Vec<&'a cargo_metadata::Package> {
    let wildmatch = wildmatch::WildMatch::new(&opt.package);

    let dependencies = match dependencies(&metadata, &root) {
        Some(dependencies) => dependencies,
        None => return Vec::new(),
    };

    dependencies
        .iter()
        .filter(|x| !opt.no_dev || !dev_only(x))
        .map(|x| package(&metadata, &x.pkg).unwrap())
        .filter(|x| wildmatch.is_match(&x.name))
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
                    format!("{}…", line)
                } else {
                    line.to_string()
                }
            })
            .unwrap_or_default();

        let row = format!("{}\t{}\t{}\n", package.name, package.version, summary,);
        table.write_all(row.as_bytes())?;
    }

    let s = String::from_utf8(table.into_inner()?)?;
    print!("{}", s);

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
    print!("{}", s);

    Ok(())
}

fn row<S: ToString>(key: &str, value: Option<&S>) -> Vec<u8> {
    format!(
        "{}\t: {}\n",
        ansi_term::Colour::Green.paint(key),
        value.map(|x| x.to_string()).unwrap_or_default(),
    )
    .as_bytes()
    .to_vec()
}
