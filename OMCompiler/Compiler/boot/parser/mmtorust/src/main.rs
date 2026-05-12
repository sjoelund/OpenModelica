use mmwinnow::parse;
use mmwinnow::Grammar;
use mmwinnow::Absyn;

mod MM;
mod hierarchy;
mod typedexp;
mod codegen;
use rayon::prelude::*;

fn main() {
    let t0 = std::time::Instant::now();
    let start_compilation = |results: Vec<Absyn::Program>| {
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
        hierarchy::flatten_extends(&mut hier);
        println!("flatten_extends: {:?}", t0.elapsed());
        let mut warnings = std::collections::BTreeSet::new();
        while hierarchy::resolve_pass(&mut hier, &mut warnings) {}
        for w in &warnings {
            eprintln!("{w}");
        }
        hierarchy::detect_recursive_types(&mut hier);
        println!("detect_recursive_types: {:?}", t0.elapsed());
        // println!("{hier}");
        codegen::generate_all(&hier, "openmodelica/src").expect("code generation failed");
        println!("MM conversion: {} files, {} failures", results.len(), failures);
    };
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

    let programs: Vec<std::sync::Mutex<Option<Absyn::Program>>> =
        files.iter().map(|_| std::sync::Mutex::new(None)).collect();

    let results: Vec<Result<(), String>> = files
        .par_iter()
        .map(|(path, ix)| {
            let result = std::fs::read_to_string(path)
                .map_err(|e| format!("read error: {e}"))
                .and_then(|code: String| parse(&code, path, if path.contains("NFBuiltin.mo") {Grammar::Modelica3} else {grammar.clone()}).map_err(|e| format!("{e}")));
            match result {
                Ok(program) => {
                    *programs[*ix].lock().unwrap() = Some(program);
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
    // Move parsed programs out instead of cloning every AST, which can be very slow
    // for the full compilerSources set.
    let parsed_programs: Vec<Absyn::Program> = programs
        .into_iter()
        .map(|m| {
            m.into_inner()
                .expect("poisoned program mutex")
                .expect("missing parsed program")
        })
        .collect();
    start_compilation(parsed_programs);
    let elapsed = t0.elapsed();
    println!("After compilation {:.2}s", elapsed.as_secs_f64())
}
