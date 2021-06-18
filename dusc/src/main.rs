use crate::meta::package::{Package, PackageKind};

use anyhow::Result;
use clap::{crate_authors, crate_version, App, Arg};
use std::path::PathBuf;

pub mod meta;
pub mod resolution;

fn main() {
    let exit_code = run().is_ok() as i32;
    std::process::exit(exit_code)
}

fn run() -> Result<()> {
    let matches = App::new("dusc")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Dusk compiler")
        .arg(
            Arg::with_name("file")
                .help("The module root or entry point")
                .required(true),
        )
        .arg(
            Arg::with_name("package-type")
                .help("The type of package this is. Binary packages are compiled to executables.")
                .long("package-type")
                .possible_values(&["library", "binary"])
                .default_value("binary"),
        )
        .arg(Arg::with_name("extern"))
        .get_matches();

    // Unwrapping here is safe since these arguments are required or have default values.
    let package_root = PathBuf::from(matches.value_of_os("file").unwrap());
    let package_type = match matches.value_of("package-type").unwrap() {
        "binary" => PackageKind::Binary,
        "library" => PackageKind::Library,
        _ => unreachable!(),
    };

    let mut package = Package::new(package_type);
    package.build_module_tree(package_root)?;

    Ok(())
}
