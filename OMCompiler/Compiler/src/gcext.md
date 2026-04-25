# GCExt Translation Assumptions

## Source File
`Util/GCExt.mo` from the OpenModelica project.

## Datatype Mappings
- `Integer` → `i32` (for parameters), `i64` (for ProfStats fields, matching 64-bit modelica integers)
- `Real` → `f64`
- `Boolean` → `bool`
- `String` → `&str`
- `uniontype/record` → `struct`

## Assumptions

### 1. GC Library ABI Compatibility
The `extern "C"` declarations assume the `omcgc` library exports the standard Boehm-GC functions with C calling conventions. The following FFI functions are declared:
- `GC_gcollect`, `GC_gcollect_and_unmap`, `GC_enable`, `GC_disable` - standard Boehm-GC API
- `GC_expand_hp` - takes `f64`, returns `int` (boolean)
- `GC_set_free_space_divisor` - takes `int`
- `GC_get_force_unmap_on_gcollect` - returns `int` (boolean)
- `GC_set_force_unmap_on_gcollect` - takes `int` (boolean)
- `omc_GC_set_max_heap_size` - takes `usize` (size_t)

**Risk:** If the actual `omcgc` library has different function signatures or naming conventions, these FFI bindings will fail at runtime.

### 2. GC Profiling Stats (`getProfStats`)
The `GC_get_prof_stats_modelica_rust` FFI function is declared but its actual C implementation is not available in this translation. The original uses a complex inline C function that:
- Calls `GC_get_prof_stats` from libgc
- Only works when `GC_VERSION_MAJOR == 7 && GC_VERSION_MINOR >= 5`, or `GC_VERSION_MAJOR >= 8`
- Falls back to zeroed stats when the struct is unavailable

**Risk:** This function returns zeroed stats by default since the actual C symbol is not linked. The C library must export `GC_get_prof_stats_modelica_rust` with 10 `i64*` pointer parameters for this to work.

### 3. Generic `free<T>` Function
The generic `free<T>(data: *mut T)` function wraps `omc_GC_free_ext` which calls `GC_free`. The original MetaModelica uses inline C with `GC_free(MMC_UNTAGPTR(data))`.

**Risk:** `MMTAG_UNTAGPTR` is an OpenModelica-specific macro. The Rust version simply casts the pointer. If the data pointer is tagged in the OpenModelica runtime sense, this could free the wrong address.

### 4. ProfStats to String
The `to_string_with` method replaces the original `match` expression on `PROFSTATS()`. It concatenates all 10 fields with a header and delimiter, and computes `total_allocd_bytes` as `bytes_allocd_since_gc + allocd_bytes_before_gc`.

### 5. Thread Safety
None of the GC functions are thread-safe wrappers. The underlying GC library must handle its own thread safety.

### 6. What Might Not Work
- **Compilation against a system that lacks `omcgc`**: All functions will fail to link at runtime since this is a static compilation with no actual `omcgc` library dependency declared in `Cargo.toml`.
- **`get_prof_stats`**: Returns zeroed stats unless the C symbol is linked.
- **`free`**: Safety depends on the caller ensuring the pointer is a valid single-object GC allocation (not a multi-element list allocation).
