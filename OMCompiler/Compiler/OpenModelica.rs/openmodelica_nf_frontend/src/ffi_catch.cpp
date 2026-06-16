// C++ exception barrier for FFI.callFunction (see src/FFI.rs).
//
// The reference compiler wraps its ffi_call in `try { ... } catch (...) {
// MMC_THROW(); }` (Compiler/runtime/ffi_omc.cpp) so an external function
// that throws — e.g. the testsuite's `exception1_ext`, which does
// `throw std::runtime_error(...)` — turns into an ordinary MetaModelica
// failure. A foreign exception must never unwind through a Rust frame
// (that aborts the process), so the try/catch has to wrap the `ffi_call`
// itself in C++; Rust only sees the 0/1 result.
//
// `ffi_cif` is opaque here on purpose: declaring `ffi_call` ourselves
// avoids needing libffi's headers at build time — the symbol resolves
// against the libffi that the `libffi-sys` crate builds and links.

extern "C" void ffi_call(void *cif, void (*fn)(), void *rvalue, void **avalue);

extern "C" int omrs_ffi_call_catch(void *cif, void (*fn)(), void *rvalue, void **avalue)
{
  try {
    ffi_call(cif, fn, rvalue, avalue);
    return 0;
  } catch (...) {
    return 1;
  }
}
