# Assumptions and Notes for backenddaeext.mo.rs

## Overview
This file translates `BackEnd/BackendDAEEXT.mo` into Rust. The original Modelica
file declares functions that call into the `omcruntime` C library via
`external "C"` annotations.

## Key Assumptions

### External C Linkage
All functions are translated as wrappers around `extern "C"` declarations.
The actual C functions are declared but not implemented in Rust - they
must be linked at runtime from the `omcruntime` library.

### List Conversion Stubs
Three functions return `list<Integer>` types:
- `get_marked_eqns()`
- `get_differentiated_eqns()`
- `get_marked_variables()`

The C implementation uses MMC's linked list representation
(`mmc_mk_nil` / `mmc_mk_cons`), which requires the full MMC runtime
bindings to convert properly to `im::List<i32>`. These return empty
lists as stubs. **This will not work correctly without the MMC bindings.**

### array<list<Integer>> Parameter
The `set_adjacency_matrix` function takes an `array<list<Integer>> m`
parameter. In the C header this is `modelica_metatype` (a void pointer).
The Rust wrapper accepts `List<List<i32>>` but cannot actually pass it
to the C function without a conversion layer. The `m` parameter is
passed as `null` in the stub.

### array<Integer> Parameters
`get_assignment` and `set_assignment` take `array<Integer>` parameters,
which are passed as `*mut c_void` in Rust. These should be raw pointers
to C-style arrays.

### Unused Parameters
`init_marks` takes two integer parameters (`inInteger1`, `inInteger2`)
that are not used in the C implementation (see the C comment "Why are
the inputs not even used?"). They are preserved in the Rust wrapper
for API compatibility.

### Commented-out Functions
The following functions are commented out in the original Modelica file
with a TODO note ("Implement an external C function for bootstrapped omc
or remove me"). These are **not included** in the translation:
- `get_e_mark`
- `dump_marked_equations`
- `dump_marked_variables`
- `init_v`
- `init_f`
- `set_v`
- `get_v`
- `set_f`
- `get_f`
- `cheapmatching`

### Datatype Mappings
- `Integer` -> `i32`
- `Real` -> `f64`
- `Boolean` -> `bool` (C returns `int`, checked against `!= 0`)
- `list<Integer>` -> `im::List<i32>`
- `array<Integer>` -> raw pointer (`*mut c_void`)
- `array<list<Integer>>` -> raw pointer (`*mut c_void`)
- `list<Integer>` return -> raw pointer (`*mut c_void`)

### 1-Indexed Arrays
The C implementation uses 1-based indexing for `lowlink`, `number`,
`v`, and `f` arrays (subtracting 1 from the input index). The Rust
wrappers pass the index through unchanged, preserving the caller's
responsibility to use 1-based indices.

## Things That May Not Work As Expected

1. **List-returning functions**: `get_marked_eqns`, `get_differentiated_eqns`,
   and `get_marked_variables` return empty lists. These require MMC runtime
   bindings to properly convert C linked lists to Rust lists.

2. **Adjacency matrix**: `set_adjacency_matrix` passes a null pointer for
   the `m` parameter. The conversion from `im::List<List<i32>>` to the
   C representation is not implemented.

3. **Global mutable state**: The C implementation uses global `std::set`
   and `std::vector` variables. Calling these functions concurrently
   from multiple threads will cause data races.

4. **Link-time dependencies**: This module requires linking against the
   `omcruntime` library. Without it, the extern "C" functions will
   cause linker errors.

5. **No error handling**: Most functions are `fn()` with no `Result`
   return type. The C functions do not return error codes (except
   `set_assignment`).
