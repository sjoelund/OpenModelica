# Debug Module Translation Assumptions

## Source
`Util/Debug.mo` from OpenModelica

## Assumptions

1. **Print.printErrorBuf mapping**: Both `trace` and `traceln` call `Print.printErrorBuf`, which maps to `crate::print::print_error_buf`. This writes to the OpenModelica error buffer via the `omcruntime` C library FFI.

2. **String parameter type**: The MetaModelica `String` type maps to `&str` (borrowed string slice) in Rust, following the convention used in `print.rs` for `print_error_buf`.

3. **No return value**: Both functions have no return value in MetaModelica, so they return `()` in Rust.

4. **Main module setup**: Added `mod print;` to `main.rs` since `debug.rs` depends on the print module. This was not previously present.

5. **No flag-controlled printing**: The `.mo` file description mentions "flag controlled printing" (via `-d-flag` runtime arguments), but the actual `.mo` source only contains `trace` and `traceln` functions with no flag logic. If flag-controlled variants are needed, they would require additional implementation.

## Things that might not work as expected

- **Runtime initialization**: Both functions depend on `OpenModelica_threadData()` from `print.rs`, which requires the full OpenModelica runtime to be initialized. Calling these functions outside of a properly initialized runtime context may result in null pointer dereferences.
- **Error buffer side effects**: These functions write to the error buffer, not stdout/stderr directly. Behavior depends on how the C library routes the error buffer output.
