use openmodelica_ast::parser::parse;
use openmodelica_ast::parser::Grammar;
use openmodelica_ast::Absyn;
use metamodelica::nil;

mod MM;
mod hierarchy;
mod typedexp;
mod codegen;
mod external_c_calls;
mod fallibility;
mod dep_analysis;
mod unused_functions;
use rayon::prelude::*;

fn start_compilation(results: Vec<Absyn::Program>) {
    let mut failures = 0;
    let t0 = std::time::Instant::now();
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
    println!("MM conversion: {} files, {} failures {:.2}s", results.len(), failures, t0.elapsed().as_secs_f64());
    let t0 = std::time::Instant::now();
    let mut hier = hierarchy::InstanceHierarchy::from_program(&all_classes);
    hierarchy::flatten_extends(&mut hier);
    let mut warnings = std::collections::BTreeSet::new();
    while hierarchy::resolve_pass(&mut hier, &mut warnings) {}
    println!("Hierarchy extends+resolve types: {:.2}s", t0.elapsed().as_secs_f64());
    for w in &warnings {
        eprintln!("{w}");
    }
    let t0 = std::time::Instant::now();
    hierarchy::detect_recursive_types(&mut hier);
    hierarchy::detect_types_containing_mutable(&mut hier);
    hierarchy::detect_types_containing_array(&mut hier);
    hierarchy::detect_types_containing_dyn_fn(&mut hier);
    println!("Hierarchy recursive+mutable detection: {:.2}s", t0.elapsed().as_secs_f64());
    // println!("{hier}");

    // Fallibility analysis: classify every user-defined function as fallible
    // (lowers to `-> Result<T>`) or infallible (lowers to `-> T`). The result
    // is consumed by codegen to decide whether each call site needs `?` and
    // whether function-pointer references need a `fnptr!`-style wrapper.
    let t0 = std::time::Instant::now();
    let info = fallibility::analyze(&hier);
    hier.fallible_functions = info.fallible_functions.clone();
    let infallible_count = info.total_functions.saturating_sub(info.fallible_functions.len());
    println!(
        "Fallibility analysis: {} functions ({} fallible, {} infallible), {} externals; {} ext registry entries; {:.2}s",
        info.total_functions,
        info.fallible_functions.len(),
        infallible_count,
        info.external_functions,
        external_c_calls::registered_count(),
        t0.elapsed().as_secs_f64(),
    );

    // PartialEq requirement analysis: for each user-defined function,
    // figure out which of its type parameters need a `+ PartialEq` bound
    // in the emitted Rust signature. This runs after fallibility so we
    // can use its results (though right now it only needs `top_level`).
    // Without this pass codegen would either over-require PartialEq
    // (breaking callbacks forwarded through generic helpers like
    // `List.map3`) or under-require it (breaking transitive callers of
    // `valueEq`/`listMember`).
    //
    // `analyze_default` is a sibling pass that computes which type
    // parameters need a `+ Default` bound, driven by
    // `arrayCreateNoInit(size, <unassigned dummy>)` call sites that lower
    // to `arrayCreateDefault(size)`. The two passes share no mutable state
    // and only read `hier.top_level`, so we run them concurrently via
    // `rayon::join` to overlap their costs.
    let t0 = std::time::Instant::now();
    let (partial_eq_required, default_required) = rayon::join(
        || codegen::analyze_partial_eq(&hier.top_level),
        || codegen::analyze_default(&hier.top_level),
    );
    hier.partial_eq_required = partial_eq_required;
    hier.default_required = default_required;
    let with_eq = hier.partial_eq_required.values().filter(|s| !s.is_empty()).count();
    let with_default = hier.default_required.values().filter(|s| !s.is_empty()).count();
    println!(
        "PartialEq + Default analysis: {} PartialEq-bounded, {} Default-bounded; {:.2}s",
        with_eq,
        with_default,
        t0.elapsed().as_secs_f64(),
    );

    let t0 = std::time::Instant::now();
    codegen::generate_all(&hier, "openmodelica/src").expect("code generation failed");
    println!("Code generation {:.2}s", t0.elapsed().as_secs_f64())
}

fn render_dot_if_available(dot_file: &str, svg_file: &str) {
    // Check whether `dot` (Graphviz) is on PATH by running `dot -V`.
    let available = std::process::Command::new("dot")
        .arg("-V")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .is_ok();

    if !available {
        println!("(dot not found on PATH; render manually: dot -Tsvg {dot_file} -o {svg_file})");
        return;
    }

    let status = std::process::Command::new("dot")
        .args(["-Tsvg", dot_file, "-o", svg_file])
        .status();

    match status {
        Ok(s) if s.success() => println!("Rendered: {svg_file}"),
        Ok(s) => eprintln!("dot exited with {s} for {dot_file}"),
        Err(e) => eprintln!("Failed to run dot for {dot_file}: {e}"),
    }
}

fn run_dep_analysis(programs: Vec<Absyn::Program>) {
    let t0 = std::time::Instant::now();
    let mut all_classes: Vec<MM::Class> = Vec::new();
    let mut failures = 0;
    for program in &programs {
        match MM::from_program(program) {
            Ok(mm_program) => all_classes.extend(mm_program),
            Err(e) => {
                eprintln!("MM ERR: {e}");
                failures += 1;
            }
        }
    }
    println!(
        "MM conversion: {} files, {} failures, {:.2}s",
        programs.len(),
        failures,
        t0.elapsed().as_secs_f64()
    );
    println!();

    let analysis = dep_analysis::DepAnalysis::build(&all_classes);
    dep_analysis::print_report(&analysis);

    let crate_dot = "dep_analysis_crates.dot";
    let pkg_dot = "dep_analysis_packages.dot";
    match dep_analysis::write_crate_dot(&analysis, crate_dot) {
        Ok(()) => println!("Wrote crate-level graph: {crate_dot}"),
        Err(e) => eprintln!("Failed to write {crate_dot}: {e}"),
    }
    match dep_analysis::write_package_dot(&analysis, pkg_dot) {
        Ok(()) => println!("Wrote package-level graph: {pkg_dot}"),
        Err(e) => eprintln!("Failed to write {pkg_dot}: {e}"),
    }
    println!();
    render_dot_if_available(crate_dot, "dep_analysis_crates.svg");
    render_dot_if_available(pkg_dot, "dep_analysis_packages.svg");
}

fn run_unused_functions(programs: Vec<Absyn::Program>) {
    let t0 = std::time::Instant::now();
    let mut all_classes: Vec<MM::Class> = Vec::new();
    let mut failures = 0;
    for program in &programs {
        match MM::from_program(program) {
            Ok(mm_program) => all_classes.extend(mm_program),
            Err(e) => {
                eprintln!("MM ERR: {e}");
                failures += 1;
            }
        }
    }
    println!(
        "MM conversion: {} files, {} failures, {:.2}s",
        programs.len(),
        failures,
        t0.elapsed().as_secs_f64()
    );

    let t0 = std::time::Instant::now();
    let mut hier = hierarchy::InstanceHierarchy::from_program(&all_classes);
    hierarchy::flatten_extends(&mut hier);
    let mut warnings = std::collections::BTreeSet::new();
    while hierarchy::resolve_pass(&mut hier, &mut warnings) {}
    println!(
        "Hierarchy extends+resolve types: {:.2}s",
        t0.elapsed().as_secs_f64()
    );

    let t0 = std::time::Instant::now();
    let report = unused_functions::analyze(&hier);
    println!(
        "Unused-function reachability: {:.2}s",
        t0.elapsed().as_secs_f64()
    );
    println!();
    unused_functions::print_report(&report);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let subcommand = args.get(1).map(|s| s.as_str());
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
    let files: Vec<(&str, usize)> = sources.lines().filter(|l| !l.trim().is_empty()).map(|f| {i += 1;(f,i-1)}).collect();

    let programs: Vec<std::sync::Mutex<Absyn::Program>> = files.iter().map(|_| std::sync::Mutex::new(Absyn::Program{classes: nil(), within_: Absyn::Within::TOP})).collect();

    let results: Vec<Result<(), String>> = files
        .par_iter()
        .map(|(path, ix)| {
            let result = std::fs::read_to_string(path)
                .map_err(|e| format!("read error: {e}"))
                .and_then(|code: String| parse(&code, path, if path.contains("NFBuiltin.mo") {Grammar::Modelica3} else {grammar}).map_err(|e| format!("{e}")));
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
    let parsed: Vec<Absyn::Program> = programs.iter().map(|p| p.lock().unwrap().clone()).collect();
    match subcommand {
        Some("dep-analysis") => run_dep_analysis(parsed),
        Some("unused-functions") => run_unused_functions(parsed),
        _ => start_compilation(parsed),
    }
}
