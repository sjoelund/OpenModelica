// Mirror the reference omc binary's RUNPATH (`$ORIGIN/../lib/<triple>/omc:$ORIGIN`).
//
// External-function shared objects (e.g. ffi/libModelicaExternalC.so) have a
// DT_NEEDED on `libOpenModelicaRuntimeC.so`, which the reference resolves
// because omc links that library and ld.so finds it through this RUNPATH at
// process start, registering it under its basename. The Rust port doesn't
// link the C runtime; instead `openmodelica_util::dynload` dlopen()s it by
// basename before loading user libraries — and that basename lookup resolves
// through this same RUNPATH when the binary is installed as `<prefix>/bin/omc`.
//
// Windows note: DLL dependencies resolve via the executable's directory and
// PATH, so no equivalent is needed (and rpath args would not be accepted).
fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let triple = match target_os.as_str() {
        // Same per-OS layout as `Autoconf::triple` in openmodelica_util.
        "linux" => format!("{target_arch}-linux-gnu"),
        "macos" => format!("{target_arch}-apple-darwin"),
        _ => return,
    };
    println!("cargo:rustc-link-arg-bins=-Wl,-rpath,$ORIGIN/../lib/{triple}/omc");
    println!("cargo:rustc-link-arg-bins=-Wl,-rpath,$ORIGIN");
    // libomcruntime.so (dlopened by the `-d=gen` pipeline) resolves the
    // compiler callback `omc_Error_getCurrentComponent` against the host
    // executable — in the C omc it comes from the compiled Error module in
    // the binary. Export the Rust port's shim (DynLoadExt.rs) from the
    // dynamic symbol table; `-u` keeps the rlib object alive through the
    // link so there is a definition to export.
    println!("cargo:rustc-link-arg-bins=-Wl,-u,omc_Error_getCurrentComponent");
    println!("cargo:rustc-link-arg-bins=-Wl,--export-dynamic-symbol=omc_Error_getCurrentComponent");
}
