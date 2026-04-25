# Assumptions for HpcOmBenchmarkExt translation

## Type mappings

- `list<Integer>` → `*mut std::ffi::c_void` (raw pointer to C list)
- `list<Real>` → `*mut std::ffi::c_void` (raw pointer to C list)
- `String` (input) → `&str` (wrapper), `*const c_char` (extern block)

## Assumptions and potential issues

1. **Return types as raw pointers**: The `list<Integer>` and `list<Real>` return types from the C library are represented as `*mut std::ffi::c_void`. The C library `omcruntime` likely returns pointers to its own list structures. Converting these to Rust `im::List` types would require additional bindings to the C list API (e.g., list creation, iteration, conversion).

2. **CString allocation**: `read_calc_times_from_xml` and `read_calc_times_from_json` allocate a `CString` from the input `&str`. If the C library stores the string pointer beyond the call, this would cause a dangling pointer. Verify that the C library copies the string internally.

3. **NUL bytes in fileName**: The `CString::new()` call will fail (panic via `expect`) if the file name contains NUL bytes. This is correct behavior as NUL bytes are invalid in C strings.

4. **Linking against omcruntime**: The `omcruntime` library must be available at link time. The Cargo project does not currently declare this dependency. You will need to add linker flags (e.g., `-L /path/to/omcruntime -l omcruntime`) via `RUSTFLAGS` or a `build.rs` script.

5. **No cleanup functions**: The original Modelica code does not expose any cleanup/free functions for the returned list pointers. If the C library uses its own allocator, the caller is responsible for freeing them. Verify the memory management contract of `omcruntime`.

6. **Deprecated markers**: All four functions are marked as `Deprecated` since they are thin wrappers around external C code. Future work should consider reimplementing this logic in Rust or providing higher-level abstractions.
