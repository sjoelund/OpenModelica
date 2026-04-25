# DiffAlgorithm - Translation Assumptions

## Translated Package

`Util/DiffAlgorithm.mo` -> `src/diffalgorithm.rs`

## Assumptions and Notes

### 1. Print Buffer System
The original MetaModelica uses `Print.saveAndClearBuf()`, `Print.printBuf()`, and `Print.restoreBuf()` for buffered output, which depend on the C runtime. The Rust translation uses direct string building instead. This works for standalone use but differs from the original when integrated with the OpenModelica runtime.

### 2. Closure Ownership
MetaModelica passes functions as values that can be copied. Rust's `impl Fn` has ownership semantics, so we pass `&impl Fn` throughout the call chain. This works correctly but requires all callers to pass function references.

### 3. Matchcontinue Translation
The `matchcontinue` construct is translated to sequential `if let Some(...)` checks:
- `onlyAdditions` is tried first - returns `None` if the diff requires removals
- `onlyRemovals` is tried second - returns `None` if the diff requires additions
- `myersGreedyDiff` is the fallback

This is correct because these algorithms are mutually exclusive: if only additions are needed, `onlyAdditions` succeeds; if only removals, `onlyRemovals` succeeds; otherwise Myers' algorithm handles the general case.

### 4. Indexing
MetaModelica uses 1-based indexing. The Rust translation uses 0-based indexing for arrays/vectors. The `start1`, `end1`, `start2`, `end2` parameters in the internal `diff_seq` function use the same 0-based convention as the input arrays.

### 5. trimCommonPrefix / trimCommonSuffix Whitespace Optimization
These functions have a special optimization: when comparing `arr1[start1]` with `arr2[start2]` (or suffix equivalents), they skip over whitespace-only elements in `arr2` if the next element in `arr2` matches `arr1[start1]`. This handles cases like:

```
arr1: ["a", "b"]
arr2: ["a", " ", "\t", "b"]
```

This is an OpenModelica-specific optimization that may produce different results than a standard Myers diff.

### 6. Threaded For (all_equal_range)
The `min(equals(...) threaded for e in 1:len1)` construct from MetaModelica is translated to `Iterator::all()`, which short-circuits on the first false value.

### 7. DiffList Output Format
The output `DiffList<T>` is a `Vec<(Diff, List<T>)>` where:
- `Diff::Add` means the `List<T>` elements are additions (present in seq2, not in seq1)
- `Diff::Delete` means the `List<T>` elements are deletions (present in seq1, not in seq2)
- `Diff::Equal` means the `List<T>` elements are equal in both sequences

### 8. Public API Surface
- `diff<T>` - main diff function
- `print_diff_terminal_color<T>` - colored terminal output
- `print_diff_xml<T>` - XML-tagged output
- `print_actual<T>` - output showing only additions (hides deletions)
- `partial_print_diff<T>` - base print function with custom DiffStrings config
- `DiffStrings` - configuration struct for print functions
- `print_start_to_end<T>` - print array range

### 9. Potential Issues
- Empty input lists: `end1 = arr1.len() - 1` for empty arrays gives `saturating_sub(1) = 0`, which is handled correctly by the base cases.
- The `myers_greedy_diff` returns an empty `Vec` on failure (should never happen with valid input).
- Generic type parameter count: `diff` requires 5 generic parameters (T, E, W, WC, TS) which may be cumbersome to use. Consider wrapper functions for common types.
