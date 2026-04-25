# OMSimulatorExt Translation Assumptions

## Summary
Translated `Util/OMSimulatorExt.mo` into `src/omsimulatorext.rs`, providing safe Rust wrappers around the `omcruntime` C library.

## Assumptions

1. **C function signatures**: The original MO file uses `external "C"` annotations with `annotation(Library = "omcruntime")` but does not specify the exact C function signatures. I inferred the following:
   - Functions returning `output Integer status` (with no other outputs) map to C functions returning `int`.
   - Functions returning `output String outString` (alone) map to C functions returning `const char*`.
   - Functions with both `output Integer status` and `output String/Integer/Real` map to C functions returning `int` status, with the other outputs passed as `*mut` pointers.
   - C function names follow the pattern `OMSimulator_oms_<functionName>` (e.g., `OMSimulator_oms_addBus`).

2. **String ownership**: Strings returned from C are assumed to be either static or heap-allocated with a lifetime long enough to be copied. The caller must not free them. If the C library returns dynamically allocated strings that the caller must free, memory leaks will occur.

3. **Boolean parameters**: The MO `Boolean` type is mapped to `c_int` in C (0 = false, non-zero = true). The Rust wrappers use `bool` for input parameters for safety.

4. **Type mapping**:
   - `Integer` in MetaModelica maps to `i32` in Rust / `c_int` in C.
   - `Real` in MetaModelica maps to `f64` in Rust / `double` in C.
   - `String` in MetaModelica maps to `&str` (input) / `String` (output) in Rust / `*const c_char` in C.

5. **Error handling**: All wrapper functions that call C functions are marked with `unsafe` blocks. The public API is safe and uses `CString` for string conversion with `expect` panics on null bytes.

6. **`loadOMSimulator` / `unloadOMSimulator`**: These functions have no input parameters and return `Integer status`. They are assumed to call `OMSimulator_loadDLL()` and `OMSimulator_unloadDLL()` respectively, both returning `int`.

7. **`statusToString`**: This function is implemented purely in Rust (no external C binding) as an `if-else` chain translated directly from the MO algorithm section.

## Potential Issues

1. **C function name conventions**: If the actual C library uses different function naming conventions (e.g., no `OMSimulator_` prefix, or a different prefix entirely), the FFI calls will fail at link time.

2. **String lifetime and memory management**: If the C library returns dynamically allocated strings that must be freed (e.g., via `free()`), the current implementation will leak memory. A corresponding `free_string` FFI declaration would be needed.

3. **Threading**: The C library may or may not be thread-safe. No synchronization is added in the Rust wrappers.

4. **`oms_RunFile`**: The original MO function name uses camelCase (`oms_RunFile`). The Rust wrapper is named `oms_run_file` (snake_case), consistent with Rust naming conventions.

5. **`oms_getBoolean` and `oms_getInteger`**: The C declarations for functions that return both a status and an output value assume the output is written via a pointer parameter. If the C library uses a different convention (e.g., struct return), this will not work.
