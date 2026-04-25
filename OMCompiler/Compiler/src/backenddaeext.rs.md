# Assumptions and Caveats for backenddaeext.rs

## Source
`BackEnd/BackendDAEEXT.mo` - A package of external C function declarations from the `omcruntime` library.

## Translation Approach
All functions in the MO file are `external "C"` declarations — they declare C functions that exist in the `omcruntime` runtime library but have no body in the MO file. The Rust translation mirrors this by providing `extern "C"` FFI declarations plus thin Rust wrapper functions with snake_case names.

## Key Assumptions

1. **No implementation provided**: Since the MO file only contains `external "C"` declarations, the Rust code provides only FFI bindings. No algorithmic implementation is included. The actual logic resides in the C runtime library (`BackendDAEEXT.cpp`).

2. **List return types**: Functions returning `list<Integer>` (e.g., `getMarkedEqns`, `getDifferentiatedEqns`, `getMarkedVariables`) return `*mut c_void` in Rust. Converting these to Rust lists would require MMC runtime bindings for the C linked list representation (`mmc_mk_cons` / `mmc_mk_nil`).

3. **Array parameters**: Functions with `array<...>` parameters (e.g., `setAdjacencyMatrix`, `getAssignment`, `setAssignment`) accept or return raw `*mut c_void` pointers. The C API uses raw void pointers for these arrays, which means no type safety on the Rust side.

4. **Boolean return types**: Functions returning `Boolean` (e.g., `getVMark`, `setAssignment`) are translated as `bool`, with the C integer return value compared against 0.

5. **Package name**: Changed from `Compiler` to `backenddaeext` in `Cargo.toml` to avoid potential naming conflict with Rust's `compiler_builtins` crate.

6. **Rust 2024 edition**: The `extern "C"` blocks are declared as `unsafe extern "C"` to comply with Rust 2024 edition requirements.

## Things That May Not Work as Expected

- **Linking**: The binary will not link successfully without the `omcruntime` library providing the actual C function implementations. This is expected for a standalone module.
- **List conversion**: The stub list-returning functions currently return null pointers. Converting them to `im::List<i32>` would require implementing C list-to-Rust-list conversion using MMC runtime primitives.
- **Adjacency matrix**: The `set_adjacency_matrix` function accepts a raw void pointer for the adjacency matrix. The actual memory layout expected by the C function is unknown.
- **Deprecated annotations**: All functions are marked as deprecated in documentation because they are external C dependencies. These should eventually be reimplemented in pure Rust or replaced with in-memory data structures.

## Mapping Summary

| MetaModelica Type | Rust Type |
|---|---|
| `Integer` | `i32` |
| `Real` | `f64` |
| `Boolean` | `bool` (via `i32 != 0`) |
| `list<Integer>` | `*mut c_void` (stub) |
| `array<list<Integer>>` | `*mut c_void` |
| `array<Integer>` | `*mut c_void` |
| `external "C"` | `unsafe extern "C"` |
