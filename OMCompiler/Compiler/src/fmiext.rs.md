# Assumptions and Notes for src/fmiext.rs

## Assumptions

1. **C function signatures**: The C functions `FMIImpl__initializeFMIImport` and `FMIImpl__releaseFMIImport` are assumed to exist in a linked C library (the MO file specifies `omcbackendruntime`, `omcruntime`, `fmilib`). The exact parameter types for the opaque pointer outputs (`outFMIInfo`, `outTypeDefinitionsList`, `outExperimentAnnotation`, `outModelVariablesList`) are unknown — in the Rust translation these are declared as `*mut c_void`. The actual deserialization of these opaque pointers into Rust types would require knowledge of the C-side struct layouts.

2. **`Option<Integer>` for pointers**: In the MO code, `Option<Integer>` is used to store C pointers that are "truncated to 32-bit". In the Rust translation, these are represented as `Option<i32>`, where `Some(value)` represents `SOME(pointer)` and `None` represents `NONE`. The wrapper functions handle the zero/non-zero distinction for the `out_fmi_context`, `out_fmi_instance`, and `out_model_variables_instance` outputs.

3. **FMI type definitions**: The FMI types (`Info`, `TypeDefinitions`, `ExperimentAnnotation`, `ModelVariables`) are translated as Rust enums matching the uniontype/record structure from `FMI.mo`. The inner record names (e.g., `INFO`, `EXPERIMENTANNOTATION`, `REALVARIABLE`) are used as variant names, following the existing convention in the codebase.

4. **`list<T>` mapping**: MetaModelica `list<T>` is mapped to `Vec<T>` in Rust. The `im::List` type from the existing codebase is used elsewhere, but `Vec<T>` is used here because the FMI functions return lists that are typically iterated once and not needed for persistent immutable operations.

5. **Boolean to C int conversion**: In the C FFI, booleans are passed as `c_int` (0 or 1). The wrapper converts Rust `bool` to `c_int` via `if cond { 1 } else { 0 }`.

6. **Return value interpretation**: The MO function `initializeFMIImport` returns `Boolean result` as an output parameter. The C function returns `c_int`, and the Rust wrapper interprets non-zero as `true`.

## Things That Might Not Work As Expected

1. **Opaque pointer outputs are placeholder**: The `outFMIInfo`, `outTypeDefinitionsList`, `outExperimentAnnotation`, and `outModelVariablesList` outputs from `initializeFMIImport` are opaque C pointers. The Rust translation returns default/empty values for these since the C-side struct layouts are not known. A complete translation would need the C struct definitions to properly deserialize these values.

2. **Library linking**: The extern declarations require the C functions to be available at link time. The Rust code does not declare which shared library to load — it must be linked externally (e.g., via linker flags or a `.cargo/config.toml`).

3. **No cleanup for allocated memory**: The C function may allocate memory for output structures. The Rust translation does not include any mechanism to free these allocations. A corresponding free function would need to be declared.

4. **Thread safety**: The FFI functions are not marked with any thread-safety guarantees. If the underlying C implementation uses global state, concurrent calls from multiple threads could cause data races.
