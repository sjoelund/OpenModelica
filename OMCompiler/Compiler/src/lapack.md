# Assumptions - lapack.rs

## Source
Translated from `Util/Lapack.mo` (16 external function declarations).

## Datatype Mapping
| MetaModelica | Rust |
|---|---|
| `Integer` | `c_int` (via `std::ffi::c_int`) |
| `Real` | `f64` |
| `String` | `*const c_char` |
| `list<list<Real>>` | `*const f64` / `*mut f64` |
| `list<Real>` | `*const f64` / `*mut f64` |

## Key Assumptions

### 1. FFI Function Signatures
The C functions are named `LapackImpl__<funcname>` and declared in an `extern "C"` block.
They accept C-style pointers (`*const f64`, `*mut f64`) for array parameters rather than
Rust `im::List` types, matching the LAPACK Fortran calling convention where arrays are
passed as pointers with leading dimensions.

### 2. Leading Dimensions (LDA, LDB, etc.)
All matrix functions include leading dimension parameters (`inLDA`, `inLDB`, `inLDVL`,
`inLDVR`, `inLDU`, `inLDVT`, `inLDH`, `inLDZ`, `inLDAB`). These are passed as `c_int`
and correspond to the row stride used by LAPACK routines for column-major storage.

### 3. String Parameters
String parameters (`inJOBVL`, `inJOBVR`, `inTRANS`, `inJOBU`, `inJOBVT`, `inCOMPZ`)
represent single-character LAPACK options (e.g., `"N"`, `"V"`, `"T"`, `"C"`). They are
declared as null-terminated C strings (`*const c_char`) to match Fortran calling conventions.

### 4. Output Parameters
The MetaModelica `output` keyword maps to `*mut` pointers in the FFI declarations.
All 16 functions follow the convention where outputs are passed as mutable pointers.

### 5. Return Value Convention
LAPACK functions use an `outINFO` integer parameter to indicate success/failure (0 = success,
negative = illegal argument, positive = algorithm did not converge). There is no return
value in the traditional sense.

### 6. No Rust Wrapper Functions
This file contains only FFI declarations. No Rust wrapper functions are provided because:
- The MetaModelica functions are purely `external "C"` declarations with no implementation body
- The actual C runtime (`omcruntime`/`Lapack`) handles all computation
- Any Rust wrappers would need to handle conversion between `im::List` and raw pointers,
  which depends on the calling convention of the C runtime

### 7. Linking
The functions are linked to the `omcruntime`/`Lapack` C library. This library must be
available at runtime. Without it, linking will fail.

## Things That Might Not Work

1. **Linking** - The binary will not link without the `omcruntime` LAPACK library.
   The library provides the actual `LapackImpl__*` implementations.

2. **Array Layout** - LAPACK uses column-major (Fortran) memory layout. If Rust code
   passes row-major arrays, results will be incorrect.

3. **String Terminology** - Single-character strings must be null-terminated
   (`b"N\0"` not `"N"`). Passing improperly terminated strings causes undefined behavior.

4. **WORK Array Size** - Each function has an `inLWORK` parameter that specifies the
   dimension of the WORK array. If too small, the C routine may overflow. Some routines
   require a query call with `LWORK=-1` to determine optimal size.

5. **Thread Safety** - LAPACK routines may not be thread-safe depending on the underlying
   implementation. Concurrent calls could cause data races.

6. **No Bounds Checking** - As FFI declarations, Rust cannot verify array bounds or
   pointer validity. Invalid pointers cause undefined behavior.
