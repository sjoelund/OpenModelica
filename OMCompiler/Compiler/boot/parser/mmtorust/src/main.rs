use mmwinnow::parse;
use mmwinnow::Grammar;
use mmwinnow::Absyn;

mod MM;
mod hierarchy;
mod codegen;
use rayon::prelude::*;

fn start_compilation(results: Vec<(&str, Absyn::Program)>) {
    let mut failures = 0;
    let mut all_classes: Vec<MM::Class> = Vec::new();
    for (path, program) in &results {
        match MM::from_program(program) {
            Ok(mm_program) => {
                all_classes.extend(mm_program);
            }
            Err(e) => {
                eprintln!("MM ERR {path}: {e}");
                failures += 1;
            }
        }
    }
    let mut hier = hierarchy::InstanceHierarchy::from_program(&all_classes);
    while hierarchy::resolve_pass(&mut hier) {}
    println!("{hier}");
    codegen::generate_all(&hier, "output").expect("code generation failed");
    println!("MM conversion: {} files, {} failures", results.len(), failures);
}

fn main() {
    rayon::ThreadPoolBuilder::new()
    .stack_size(16 * 1024 * 1024) // 16 MiB stack size, to avoid "thread stack overflow" on large files, especially on debug builds
    .num_threads(12)
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
