use mmwinnow::parse;
use mmwinnow::Grammar;
use mmwinnow::Absyn;
use mmwinnow::List;

mod MM;
mod hierarchy;
mod codegen;
use rayon::prelude::*;
use std::sync::Arc;

fn start_compilation(results: Vec<Absyn::Program>) {
    let mut failures = 0;
    let mut all_classes: Vec<MM::Class> = Vec::new();
    for program in &results {
        match MM::from_program(program) {
            Ok(mm_program) => {
                all_classes.extend(mm_program);
            }
            Err(e) => {
                eprintln!("MM ERR: {e}");
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
    let t0 = std::time::Instant::now();
    rayon::ThreadPoolBuilder::new()
    .stack_size(16 * 1024 * 1024) // 16 MiB stack size, to avoid "thread stack overflow" on large files, especially on debug builds
    .num_threads(12)
    .build_global()
    .unwrap();

    let source_path = "compilerSources.txt";
    let grammar = Grammar::MetaModelica;
    let sources = std::fs::read_to_string(source_path).expect("compilerSources.txt not found");
    let mut i = 0;
    let files: Vec<(&str, usize)> = sources.lines().filter(|l| !l.trim().is_empty()).map(|f| {i=i+1;(f,i-1)}).collect();

    let programs: Vec<std::sync::Mutex<Absyn::Program>> = files.iter().map(|_| std::sync::Mutex::new(Absyn::Program::PROGRAM{classes: List::Nil{}, within_: Absyn::Within::TOP})).collect();

    let results: Vec<Result<(), String>> = files
        .par_iter()
        .map(|(path, ix)| {
            let result = std::fs::read_to_string(path)
                .map_err(|e| format!("read error: {e}"))
                .and_then(|code: String| parse(&code, path, grammar.clone()).map_err(|e| format!("{e}")));
            match result {
                Ok(program) => {
                    *programs[*ix].lock().unwrap() = program;
                    Ok(())
                },
                Err(e) => Err(e),
            }
        })
        .collect();
    let elapsed = t0.elapsed();

    let mut failures = 0;
    for result in &results {
        match result {
            Ok(_) => (), // println!("OK  {path}"),
            Err(e) => {
                eprintln!("ERR: {e}");
                failures += 1;
            }
        }
    }

    println!("OpenModelica: {} files, {} failures, {:.2}s", results.len(), failures, elapsed.as_secs_f64());
    start_compilation(programs.iter().map(|p| p.lock().unwrap().clone()).collect());
    let elapsed = t0.elapsed();
    println!("After compilation {:.2}s", elapsed.as_secs_f64())
}
