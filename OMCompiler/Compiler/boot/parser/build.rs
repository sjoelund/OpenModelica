// build.rs
use std::path::PathBuf;
use parol::build::Builder;

fn main() {
    if true { return ; }
    // 1. Define the input grammar file
    let grammar_file = PathBuf::from("src/metamodelica.par");

    // 2. Configure and run the builder
    Builder::with_cargo_script_output()
        .grammar_file(grammar_file)
        .generate_parser()
        .unwrap(); // Handle errors appropriately in production
}
