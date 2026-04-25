# TaskGraphResults Translation Notes

## Assumptions

### C Function Signatures
The original MO file declares two external "C" functions:
- `TaskGraphResults_checkTaskGraph(filename, reffilename)`
- `TaskGraphResults_checkCodeGraph(graphfile, codefile)`

The actual C signatures from `omcruntime` were not available (no header file found), so the FFI declarations assume:
- Both parameters are `*const c_char` (C strings)
- Both functions return `*mut c_char` (caller-allocated C string)
- The return value is a C string in MetaModelica list format: `{"item1","item2",...}`

### Return Value Format
The parser assumes the C function returns a NUL-terminated C string containing a list in the format `{"elem1","elem2",...}`. The `parse_list_string` helper strips the outer braces and splits on commas, removing surrounding quotes.

### Memory Management
The C-returned string is owned by the C side and must be freed by the Rust side. `CString::from_raw` is used to take ownership so the memory is properly freed when dropped.

### The C functions are linked to `omcruntime`
The `annotation(Library = "omcruntime")` in the MO code indicates these functions come from the OpenModelica C runtime. The Rust `extern "C"` block will only link correctly if `omcruntime` is available as a library on the linker path.

## Things That Might Not Work

1. **Missing C symbols**: If `omcruntime` is not linked (not on `LIBRARY_PATH` / not a dependency), the Rust binary will fail at runtime with undefined symbol errors.

2. **String encoding**: The parser assumes the C strings are UTF-8. If `omcruntime` uses a different encoding, parsing will produce garbage.

3. **List format changes**: The `parse_list_string` helper is a best-effort implementation. If `omcruntime` uses a different list format (e.g., comma-separated without quotes, or a different brace style), parsing will fail.

4. **Error handling**: If `omcruntime` returns `NULL` or an error string instead of a valid list, the parser may panic or produce incorrect results.

5. **Thread safety**: The external C functions may not be thread-safe. Concurrent calls from multiple Rust threads could cause data races in the `omcruntime` C code.
