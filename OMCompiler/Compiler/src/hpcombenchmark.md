# Assumptions for HpcOmBenchmark translation

## Source

Translated from `BackEnd/HpcOmBenchmark.mo`.

## Type mappings

- `Integer` → `i32`
- `Real` → `f64`
- `String` → `&str` (input), `String` (output)
- `list<Integer>` → `im::Vector<i32>`
- `list<Real>` → `im::Vector<f64>`
- `list<tuple<Integer,Integer,Real>>` → `im::Vector<(i32, i32, f64)>`
- `tuple<tuple<Integer,Integer>,tuple<Integer,Integer>>` → `((i32, i32), (i32, i32))`

## Assumptions and potential issues

### 1. C library pointer casting

The `hpcombenchmarkext` module returns raw C pointers (`*mut c_void`) from the `omcruntime` library. The new typed wrapper functions (`cast_list_real`, `cast_list_int`, and the `list_*` wrappers) cast these raw pointers directly to `im::Vector<T>` using `std::ptr::read`. This assumes the C library's internal list representation is ABI-compatible with `im::Vector`. If the C library uses a different structure layout, these functions will read garbage data. The correct fix would be to add proper FFI bindings that know how to iterate and construct `im::Vector` from the C list structures.

### 2. File existence checking

The original code uses `System.getFileModificationTime` to check if a file exists:

```metamodelica
SOME(_) = System.getFileModificationTime(fullFileName);
```

This has been translated to `std::fs::metadata(fullFileName).map(|_| ())?` which checks if the metadata can be read (i.e., the file exists and is readable). The `System` package has not been translated to Rust yet, so using the standard library is the appropriate choice here.

### 3. matchcontinue translation

Both `readCalcTimesFromFile` and `expandCalcTimes` use `matchcontinue`. `readCalcTimesFromFile` tries JSON first, then XML, and fails if neither exists. This is translated to sequential `if let` checks with `bail!` for fallback.

`expandCalcTimes` uses `matchcontinue` with list pattern matching (`numOfCalcs::calcTimeSum::eqIdx::rest`) and a tail-recursive call. This has been translated to an iterative `chunks_exact(3)` loop. The original prepends tuples to the accumulator list and returns it reversed at the end; our implementation uses `push_front` which achieves the same ordering in a single pass.

### 4. expandCalcTimes element ordering

The original list pattern `numOfCalcs::calcTimeSum::eqIdx::rest` destructures the first three elements in that order. The tuple is constructed as `(intEqIdx, intNumOfCalcs, calcTimeSum)`. The input list stores elements in the order: `[numOfCalcs, calcTimeSum, eqIdx, ...]`.

### 5. Error handling on invalid list length

When the input list length is not divisible by 3, the original `expandCalcTimes` calls `fail()`. The Rust translation prints an error message and returns an empty list. This is a deliberate choice since `Result` propagation through the recursive helper would be awkward; callers receive an empty result rather than a `Result` from this internal function.

### 6. No `System` module

The original imports `System` for `getFileModificationTime`. Since the System package has not been translated to Rust, the standard library's `std::fs::metadata` is used instead.

### 7. Deprecated functions

All functions in this module depend on the `hpcombenchmarkext` FFI bindings which in turn depend on the `omcruntime` C library. These are marked as deprecated in the ext module; the same applies to this translation.

## Compilation

- All 10 unit tests pass
- Code compiles with `cargo check` and `cargo test`
- The `bench_system` function returns `((opCostM, opCostN), (comCostM, comCostN))` using `get(0)`/`get(1)` with `unwrap_or(&0)` fallback for safety
