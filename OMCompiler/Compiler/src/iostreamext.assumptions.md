# Assumptions for iostreamext.rs (translated from IOStreamExt.mo)

## Assumptions and Potential Issues

### 1. C Library Linking
- **Assumption**: The `omcruntime` C library provides the `IOStreamExt_*` functions.
- **Risk**: The `runtime/IOStreamExt.c` source file is a stub (49 lines, only includes). The actual implementations may be in a separate compiled library (`libomcruntime.a` or `libomcruntime.so`). If the library is not linked at build time, the program will fail at link time.
- **Mitigation**: The `Cargo.toml` needs a `cargo:rustc-link-lib=omcruntime` directive (via `build.rs`) to link the runtime library. Without this, the FFI declarations will cause unresolved symbol errors.

### 2. List Handling for appendReversedList / printReversedList
- **Assumption**: The `list<String>` parameter from MetaModelica is represented as an opaque pointer (`*mut c_void`) in the C interface. The actual list type is the OpenModelica internal list representation.
- **Risk**: The `*mut c_void` type is a placeholder. The actual C signature may expect a specific type (e.g., `modelica_metatype`). If callers need to pass Rust `im::List<String>`, additional conversion code will be needed to bridge the gap.
- **Mitigation**: If the list functions are used, the list type needs to be mapped to the correct C-compatible representation.

### 3. Memory Management for String Returns
- **Assumption**: The C functions `IOStreamExt_readFile` and `IOStreamExt_readBuffer` return `const char*` pointers owned by the C runtime.
- **Risk**: The returned pointer may need to be freed by the caller, or it may be a static allocation. The current Rust code copies the string immediately via `to_string_lossy()`, which avoids leaks but may not handle non-UTF8 data correctly (falls back to replacement character).
- **Mitigation**: If the C functions allocate memory that must be freed, a corresponding free function (e.g., `IOStreamExt_freeString`) would be needed. None is visible in the current interface.

### 4. Integer File/Buffer IDs
- **Assumption**: File IDs and buffer IDs are `c_int` (32-bit signed integers) matching the MetaModelica `Integer` type.
- **Risk**: If the C implementation uses a different integer width or handles IDs as pointers, the mapping would be incorrect.

### 5. No Error Handling
- **Assumption**: The original MetaModelica functions do not raise exceptions on failure. Failure modes (e.g., invalid file ID) may silently do nothing or return -1.
- **Risk**: No Rust `Result` types are used because the original C API appears to use error codes rather than exceptions. The caller must check return values.

### 6. whereToPrint Enum Mapping
- **Assumption**: The `whereToPrint` integer maps as: 1 = stdout, 2 = stderr. This matches the annotation in the original MO file.

### 7. Buffer Implementation
- **Assumption**: Buffers are managed internally by the C runtime via integer IDs. The `createBuffer()` function returns an ID that can be used for subsequent operations. The Rust side does not track buffer lifetime.
- **Risk**: There is no RAII wrapper for buffers in this translation. Buffers must be explicitly deleted via `delete_buffer()` or they may leak.

### 8. Thread Safety
- **Assumption**: The C runtime functions are assumed to be thread-safe if the underlying library supports it. No explicit synchronization is added in the Rust bindings.

### 9. No Buffer/File Type Abstraction
- **Assumption**: Unlike `File.mo` which provides a `File` struct wrapping an opaque handle, this module uses raw integer IDs. This is consistent with the original MetaModelica interface which uses `Integer` for both file and buffer handles.
