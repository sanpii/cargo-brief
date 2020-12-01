use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    dependencies: Vec<String>,
    #[structopt(long, default_value = "./Cargo.toml")]
    manifest_path: String,
    #[structopt(long)]
    no_dev: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;

    let opt = Opt::from_args();
    let metadata = cargo_metadata::MetadataCommand::new()
        .manifest_path(&opt.manifest_path)
        .exec()?;

    let mut table = tabwriter::TabWriter::new(Vec::new());

    let resolve = metadata.resolve.as_ref().unwrap();
    let dependencies = dependencies(&metadata, &resolve.root.as_ref().unwrap()).unwrap();

    for dependency in dependencies {
        if opt.no_dev && dev_only(&dependency) {
            continue;
        }

        let package = package(&metadata, &dependency.pkg).unwrap();

        if !opt.dependencies.is_empty() && !opt.dependencies.contains(&package.name) {
            continue;
        }

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
