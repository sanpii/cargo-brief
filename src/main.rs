use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    #[structopt(long, default_value = "./Cargo.toml")]
    manifest_path: String,
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

    for dependency_id in dependencies {
        let package = package(&metadata, &dependency_id).unwrap();

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
) -> Option<&'a Vec<cargo_metadata::PackageId>> {
    metadata
        .resolve
        .as_ref()
        .unwrap()
        .nodes
        .iter()
        .find(|node| &node.id == id)
        .map(|node| &node.dependencies)
}

fn package<'a>(
    metadata: &'a cargo_metadata::Metadata,
    id: &cargo_metadata::PackageId,
) -> Option<&'a cargo_metadata::Package> {
    metadata.packages.iter().find(|package| &package.id == id)
}
