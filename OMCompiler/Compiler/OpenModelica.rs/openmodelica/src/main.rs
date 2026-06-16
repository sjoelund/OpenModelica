use arcstr::ArcStr;
use openmodelica_backend_main::Main;
use std::env::args;
use std::io::Write;
use std::sync::Arc;

/// Default worker-thread stack size: 4 MiB - use env. var to increase it.
///
/// MMC's generated C code eliminates tail calls and uses small stack frames,
/// so the C omc fits deep traversals into the default 8 MiB main stack. The
/// Rust port only lowers *self* tail calls (mmtorust's `#[tailcall]` pass)
/// and its frames are larger, so workloads like `DumpGraphviz.dump` over the
/// whole compiler sources overflow 8 MiB and Rust aborts (no recovery, unlike
/// MMC's SEGV-handler unwind). The reservation is virtual address space only
/// — Linux commits stack pages lazily — so the cost of the headroom is nil.
const DEFAULT_STACK_SIZE: usize = 4 * 1024 * 1024;

fn run() -> i32 {
    if Main::main(Arc::new(args().skip(1).map(ArcStr::from).collect())).is_err() {
        // Mirror `rml_execution_failed()` in the generated C wrapper
        // (CodegenCFunctions.tpl): flush pending output, report on stderr
        // and exit with status 1. The MetaModelica exception carries no
        // payload worth printing — diagnostics were already emitted via
        // the Error buffer before `fail()` reached us.
        let _ = std::io::stdout().flush();
        eprintln!("Execution failed!");
        return 1;
    }
    0
}

fn main() -> std::process::ExitCode {
    let stack_size = std::env::var("OPENMODELICA_STACK_SIZE_KB")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .map(|kb| kb * 1024)
        .unwrap_or(DEFAULT_STACK_SIZE);
    match std::thread::Builder::new()
        .name("omc-main".to_owned())
        .stack_size(stack_size)
        .spawn(|| {
            let code = run();
            // Exit from inside the worker, skipping thread-local destructors:
            // the `-d=gen` dynamic-load pipeline initialises the dlopened MMC
            // runtime's GC and thread keys on this thread, and letting the
            // thread wind down normally runs those keys' destructors against
            // state the runtime has already torn down (observed as a SIGSEGV
            // at exit after any Print-buffer use in a gen'd function). The C
            // omc never destroys its main thread before exit either, so this
            // matches its lifecycle. Flush explicitly: `process::exit` skips
            // Rust's stdout flush-on-main-return.
            let _ = std::io::stdout().flush();
            let _ = std::io::stderr().flush();
            std::process::exit(code);
        }) {
        Ok(handle) => {
            // Only reached if the worker panicked before its process::exit
            // (the default panic hook has already printed the message).
            let _ = handle.join();
            std::process::ExitCode::FAILURE
        }
        // Could not reserve the requested stack (e.g. tight ulimit -v).
        // Run on the main thread rather than failing outright.
        Err(_) => std::process::ExitCode::from(run() as u8),
    }
}
