# Assumptions and Notes for src/file.rs

## Assumptions

1. **C library name**: The FFI bindings assume the functions (e.g., `om_file_new`, `om_file_free`, etc.) are available in a shared library linked at build time (likely `omcruntime`). The library name is not specified in the MO file and must be linked externally.

2. **Mode/Escape/Whence integer mappings**: The `Mode`, `Escape`, and `Whence` enumerations are mapped to sequential c_int values starting from 0:
   - `Mode::Read = 0`, `Mode::Write = 1`
   - `Escape::None = 0`, `C = 1`, `JSON = 2`, `XML = 3`
   - `Whence::Set = 0`, `Current = 1`, `End = 2`
   These values are assumed to match the C-side enumeration. If the C side uses different values, the `to_c_int()` methods need updating.

3. **File handle lifecycle**: The `File` struct wraps a raw pointer (`*mut c_void`). The Rust side does not automatically call `om_file_free` on drop. The caller must explicitly call `destructor()` to free the handle. A `Drop` implementation could be added for RAII safety.

4. **`Option<Integer>` in MetaModelica**: The MO file uses `Option<Integer>` for some parameters (e.g., `fromID` in the constructor, `file` in `getFilename`). In the Rust translation, these are represented as raw pointers (`*const c_void`), where `NULL` represents `NONE` and a valid pointer represents `SOME(value)`.

5. **`getFilename` return value**: The MO file declares `getFilename` as a function with an output parameter (`output String fileName2`). The Rust translation returns a `String` directly via `CStr::from_ptr().to_string_lossy()`. This assumes the C function returns a null-terminated C string that the caller should NOT free.

6. **`writeSpace` loop range**: The MO file uses `for i in 1:n loop`, meaning if `n=0`, the loop body executes 0 times (OpenModelica's 1-based for loops with `1:n` are equivalent to Rust's `1..=n` inclusive range). The Rust translation handles `n <= 0` as a no-op.

7. **ExternalObject base class**: The `File` class extends `ExternalObject`, which in the C runtime is typically an opaque pointer. The Rust `File` struct mirrors this with a `handle: *mut c_void` field.

## Things That Might Not Work As Expected

1. **No Drop implementation**: The `File` struct does not implement `Drop`, so file handles will not be automatically freed when the struct goes out of scope. This could lead to resource leaks. Consider adding a `Drop` impl that calls `destructor()`.

2. **No file close function**: The MO file does not define a `close` function. The only way to release a file is through `destructor()` (which calls `om_file_free`). Whether this properly flushes and closes the underlying OS file descriptor depends on the C implementation.

3. **Seek return value interpretation**: The C function `om_file_seek` returns `c_int` where the MO code maps non-zero to success (`success = om_file_seek(...)`). In C, `fseek` typically returns 0 on success and non-zero on failure. The Rust wrapper inverts this logic (`result != 0` returns `true`), which matches the MO semantics but contradicts standard C `fseek` behavior. If the C function is actually `fseek`-like, the return logic may be inverted.

4. **Error handling is minimal**: The `seek` function returns a `bool`, but other functions (`open`, `write`, `write_real`, etc.) have no error return. The C functions may set error indicators internally, but these are not exposed in the Rust API.

5. **Thread safety**: The `File` struct contains raw pointers and is not `Send`/`Sync` by default. Concurrent access to the same file handle from multiple threads is not supported.

6. **`om_file_get_filename` lifetime**: The returned `*const c_char` from `om_file_get_filename` may have a lifetime tied to the `File` handle. If the handle is freed before the returned `String` is dropped, this could cause a use-after-free. The `to_string_lossy()` call copies the data, mitigating this risk.
