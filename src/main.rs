use structopt::StructOpt;

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(StructOpt)]
struct Opt {
    dependencies: Vec<String>,
    #[structopt(long, default_value = "./Cargo.toml")]
    manifest_path: String,
    #[structopt(long)]
    no_dev: bool,
}

fn main() -> Result {
    let opt = Opt::from_args();
    let metadata = cargo_metadata::MetadataCommand::new()
        .manifest_path(&opt.manifest_path)
        .exec()?;

    let resolve = metadata.resolve.as_ref().unwrap();
    let packages = dependencies(&metadata, &resolve.root.as_ref().unwrap())
        .unwrap()
        .iter()
        .filter(|x| !opt.no_dev || !dev_only(x))
        .map(|x| package(&metadata, &x.pkg).unwrap())
        .filter(|x| opt.dependencies.is_empty() || opt.dependencies.contains(&x.name))
        .collect::<Vec<_>>();

    if !opt.dependencies.is_empty() && packages.len() == 1 {
        display_one(&packages[0])
    } else {
        display_list(&packages)
    }
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
    dependency.dep_kinds.iter()
        .map(|x| x.kind)
        .collect::<Vec<_>>() == vec![cargo_metadata::DependencyKind::Development]
}

fn display_list(packages: &[&cargo_metadata::Package]) -> Result {
    use std::io::Write;

    let mut table = tabwriter::TabWriter::new(Vec::new());

    for package in packages {
        let row = format!(
            "{}\t{}\t{}\n",
            package.name,
            package.version,
            package.description.as_ref().unwrap_or(&"".to_string())
        );
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
    let features = package.features.keys().map(|x| x.clone()).collect::<Vec<_>>();
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
