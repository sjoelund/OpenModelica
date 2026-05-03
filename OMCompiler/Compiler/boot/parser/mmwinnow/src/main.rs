use mmwinnow::parse;
use mmwinnow::Grammar;
use rayon::prelude::*;

fn main() {
    rayon::ThreadPoolBuilder::new().num_threads(12).build_global().unwrap();

    let sources_path = "/home/martin/dev/OpenModelica-rust/OMCompiler/Compiler/boot/parser/compilerSources.txt";
    let sources = std::fs::read_to_string(sources_path).expect("compilerSources.txt not found");
    let files: Vec<&str> = sources.lines().filter(|l| !l.trim().is_empty()).collect();

    let results: Vec<(&str, Result<(), String>)> = files
        .par_iter()
        .map(|path| {
            let result = std::fs::read_to_string(path)
                .map_err(|e| format!("read error: {e}"))
                .and_then(|code| parse(&code, Grammar::MetaModelica).map(|_| ()).map_err(|e| format!("{e}")));
            (*path, result)
        })
        .collect();

    let mut failures = 0;
    for (path, result) in &results {
        match result {
            Ok(()) => (), // println!("OK  {path}"),
            Err(e) => {
                eprintln!("ERR {path}: {e}");
                failures += 1;
            }
        }
    }

    println!("\n{} files, {} failures", results.len(), failures);
    if failures > 0 {
        std::process::exit(1);
    }
}
