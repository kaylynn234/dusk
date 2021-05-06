use clap::{crate_authors, crate_version, App, Arg};
use std::path::PathBuf;

pub mod symbols;

fn main() {
    let exit_code = match run() {
        Ok(_) => 0,
        Err(_) => 1,
    };

    std::process::exit(exit_code)
}

fn run() -> Result<(), ()> {
    let matches = App::new("dusc")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Dusk programming language compiler")
        .arg(
            Arg::with_name("file")
                .help("The module root or entry point")
                .required(true),
        )
        .arg(
            Arg::with_name("type")
                .help("The type of package this is. Binary packages are compiled to executables.")
                .short("t")
                .long("type")
                .possible_values(&["library", "binary"])
                .default_value("binary"),
        )
        .get_matches();

    // Unwrapping here is safe since these arguments are required or have default values.
    let mut package_root = PathBuf::from(matches.value_of_os("file").unwrap());
    let package_type = matches.value_of("package").unwrap();

    if package_root.is_dir() {
        package_root.push("/main.dusk");
    }

    if !package_root.exists() {
        eprintln!(
            "error: the referenced path (\"{}\") does not exist",
            package_root.to_string_lossy()
        );

        return Err(());
    }

    // TODO: Everything else. Need to build a module tree. Add that to the parser and then extract into a recursive
    // function here.

    Ok(())
}
