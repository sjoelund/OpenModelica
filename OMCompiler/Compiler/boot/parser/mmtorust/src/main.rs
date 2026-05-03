use mmwinnow::parse;
use mmwinnow::Grammar;
use mmwinnow::Absyn;
use rayon::prelude::*;

fn start_compilation(results: Vec<(&str, Absyn::Program)>) {
    for (path, program) in results {
        if path.ends_with("/Absyn.mo") {
            println!("{:?}", program);
        }
    }
}

fn main() {
    rayon::ThreadPoolBuilder::new()
    .stack_size(16 * 1024 * 1024) // 16 MiB stack size, to avoid "thread stack overflow" on large files, especially on debug builds
    .build_global()
    .unwrap();

    let source_path = "compilerSources.txt";
    let grammar = Grammar::MetaModelica;
    let sources = std::fs::read_to_string(source_path).expect("compilerSources.txt not found");
    let files: Vec<&str> = sources.lines().filter(|l| !l.trim().is_empty()).collect();

    let t0 = std::time::Instant::now();
    let results: Vec<(&str, Result<Absyn::Program, String>)> = files
        .par_iter()
        .map(|path| {
            let result = std::fs::read_to_string(path)
                .map_err(|e| format!("read error: {e}"))
                .and_then(|code: String| parse(&code, path, grammar.clone()).map_err(|e| format!("{e}")));
            (*path, result)
        })
        .collect();
    let elapsed = t0.elapsed();

    let mut failures = 0;
    for (path, result) in &results {
        match result {
            Ok(_) => (), // println!("OK  {path}"),
            Err(e) => {
                eprintln!("ERR {path}: {e}");
                failures += 1;
            }
        }
    }

    println!("OpenModelica: {} files, {} failures, {:.2}s", results.len(), failures, elapsed.as_secs_f64());
    if failures > 0 {
        std::process::exit(1);
    };
    start_compilation(results.into_iter().map(|(p, r)| (p, r.unwrap())).collect());
}
