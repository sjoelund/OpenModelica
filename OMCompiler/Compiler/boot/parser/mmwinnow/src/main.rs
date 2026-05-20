use mmwinnow::parse;
use mmwinnow::Grammar;
use rayon::prelude::*;

fn parse_file_with_list(sources_path: &str, grammar: Grammar, test_name: &str) {
    let sources = std::fs::read_to_string(sources_path).expect("compilerSources.txt not found");
    let files: Vec<&str> = sources.lines().filter(|l| !l.trim().is_empty()).collect();

    let t0 = std::time::Instant::now();
    let results: Vec<(&str, Result<(), String>)> = files
        .par_iter()
        .map(|path| {
            let result = std::fs::read_to_string(path)
                .map_err(|e| format!("read error: {e}"))
                .and_then(|code: String| parse(&code, path, grammar).map(|_| ()).map_err(|e| format!("{e}")));
            (*path, result)
        })
        .collect();
    let elapsed = t0.elapsed();

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

    println!("{test_name}: {} files, {} failures, {:.2}s", results.len(), failures, elapsed.as_secs_f64());
    if failures > 0 {
        std::process::exit(1);
    };
}

fn main() {
    rayon::ThreadPoolBuilder::new().num_threads(12).build_global().unwrap();
    parse_file_with_list("/home/martin/dev/OpenModelica-rust/OMCompiler/Compiler/boot/parser/compilerSources.txt", Grammar::MetaModelica, "OpenModelica full sources");
    parse_file_with_list("/home/martin/dev/OpenModelica-rust/OMCompiler/Compiler/boot/parser/mslSources.txt", Grammar::Modelica3, "Modelica Standard Library");
}
