# UnitParserExt - Assumptions and Notes

## Assumptions

1. **External C library**: All functions are external C bindings to the `omcruntime` library. The actual implementation lives in C code that is linked at runtime.

2. **C `String` type**: The C `String` parameter is treated as `*const String` in the FFI layer. In the actual C code this is likely `const char*`. The `String` type is not FFI-safe in Rust (it's a fat pointer), but this matches the pattern used by other modules in this project.

3. **C `list<Integer>` and `list<String>` types**: These are represented as opaque pointers (`CListInt` and `CListStr` structs) since the exact C struct layout is not available. The functions that use these types (unit2str, str2unit) create default placeholder objects rather than passing real data, since the Rust code has no way to construct the C-side list structures.

4. **str2unit return values**: Since the C function writes to output pointers that point to opaque C types, the Rust wrapper returns empty vectors for the list outputs and the correct scale_factor/offset values. The actual list data is managed by the C runtime and cannot be reconstructed from opaque pointers.

5. **allUnitSymbols return type**: The C function returns an array of strings. In Rust this is represented as `Vec<String>` which is then converted to `List<String>` (im::Vector).

6. **String parameters for addBase, addDerived, etc.**: These take Rust `&str` and are converted to owned `String` before being passed as raw pointers to the C function. The C function likely stores a copy of the string.

## Things That Might Not Work As Expected

1. **Calling the external functions**: The code compiles but the functions will segfault or return garbage at runtime unless the `omcruntime` C library is linked and the symbols are available (e.g., via `cargo build --extern omcruntime=libomcruntime.so` or similar).

2. **unit2str and str2unit**: These functions take placeholder opaque types since we cannot construct the C-side list structures from Rust. The actual data exchange would require either:
   - Re-implementing the C-side list types as `#[repr(C)]` Rust structs
   - Having a Rust-side implementation of the list-to-C conversion

3. **Memory management**: The C functions may allocate memory that the Rust code cannot free. If this module is used extensively, memory leaks could occur unless proper `free` functions are called.

4. **Thread safety**: The C functions likely use global state (the unit parser is a global resource). Calling these functions from multiple threads concurrently may cause data races.
