# StringUtil.rs - Translation Assumptions and Notes

## Translation Summary

This module translates `Util/StringUtil.mo` from MetaModelica to Rust. All public functions and constants from the original MO file have been translated.

## Assumptions and Potential Issues

### 1. Indexing Convention
- **MetaModelica** uses 1-based string indexing; **Rust** uses 0-based indexing.
- All search functions (`find_char`, `rfind_char`, etc.) accept 1-based start/end positions matching the original MM semantics and return 1-based indices.
- Internally, positions are converted to 0-based for string access.

### 2. UTF-8 Handling
- `stringLength` returns character count, not byte count (using `.chars().count()`).
- The `word_wrap` function operates on character boundaries but does not handle multi-byte Unicode characters correctly for word wrapping purposes. This matches the original MM behavior which states it "does not handle UTF-8 strings correctly."
- Character-to-int conversions (e.g., `is_alpha`) assume ASCII values. Non-ASCII characters above 127 may behave unexpectedly.

### 3. `System.strtok(s, "\n")`
- Translated using Rust's `str::split('\n')`, which produces an iterator over string slices.
- Unlike MM's `strtok`, this preserves empty strings from consecutive delimiters.

### 4. `System.StringAllocator` (in `repeat`)
- The MM code uses `StringAllocator` for efficient string building.
- The Rust version uses `str::repeat(n)` which is the idiomatic equivalent and should have comparable performance.

### 5. `listReverseInPlace`
- Translated using `im::Vector` with a full reversal at the end.
- The MM version reverses in-place; the Rust version creates a new reversed vector. This is functionally equivalent but uses additional memory.

### 6. `String(Real, significantDigits=n)`
- Implemented using `format!("{:.precision$}", val)` with precision = n-1.
- Trailing zeros and decimal points are trimmed to match MM's `String()` formatting behavior.

### 7. `String(Integer)`
- Implemented using `format!("{}", val)`, which produces a decimal string representation.

### 8. `stringGetNoBoundsChecking`
- The original MM function accesses characters without bounds checking.
- The Rust version collects characters into a `Vec` and accesses by index with bounds checking (safer but slower).

### 9. `stringCharInt`
- Character to ASCII code conversion uses Rust's `char as i32`.
- Only valid for ASCII characters (0-127). Non-ASCII Unicode code points are cast to i32, which may differ from MM behavior for values > 127.

### 10. `realMul` / `realInt`
- `realMul(a, b)` -> `a * b` (f64 multiplication).
- `realInt(r)` -> `(r as i32)` (f64 to i32 conversion via truncation).

### 11. `substring`
- 1-based inclusive indexing: `substring(s, 1, n)` returns the first n characters.
- Out-of-bounds indices are clamped to the string length.

### 12. Unused Constant: `CHAR_DOT`
- Defined as 46 in the original MM but not used in any function. Kept for API compatibility.

### 13. Deprecated Functions
The following functions have Rust built-in equivalents and are marked as deprecated:
- `repeat_str` -> `str.repeat(n)`
- `starts_with_str` -> `str.starts_with()`
- `ends_with_str` -> `str.ends_with()`
- `ends_with_newline_str` -> `str.ends_with('\n')`

### 14. `stripBOM`
- The UTF-8 BOM is the byte sequence 0xEF 0xBB 0xBF.
- The function checks for these bytes and strips them if present.
- Returns the stripped BOM character (U+FEFF) or empty string if no BOM found.

### 15. `bytesToReadableUnit`
- Uses powers of 1024 (binary prefixes).
- `maxSizeInUnit` controls when units switch: if value exceeds 500 * MB, it shows as GB, etc.
- The default `maxSizeInUnit=500` means values up to 500 GB are shown in GB before switching to TB.

### 16. `word_wrap` Complexity
- The `word_wrap` function is complex with nested conditionals for hyphenation at dash boundaries.
- The hyphenation logic checks if both characters surrounding a dash are alphabetic before breaking at the dash.
- This function may have performance issues with very long strings due to repeated `substring` allocations.

### 17. `equalIgnoreSpace` Algorithm
- Compares non-space characters sequentially, skipping spaces in both strings.
- Uses a forward-only scan, which is O(n) but may not handle all edge cases identically to MM.

## Test Coverage

The `test_stringutil()` function tests:
- Constants (`NO_POS`, `CHAR_NEWLINE`, `CHAR_SPACE`)
- Headline functions (`headline_1` through `headline_4`)
- Character search (`find_char`, `rfind_char`, `find_char_not`, `rfind_char_not`)
- Character classification (`is_alpha`)
- String repetition (`repeat`, `repeat_str`)
- Quoting (`quote`)
- Prefix/suffix checks (`starts_with`, `ends_with`, `ends_with_newline`)
- String rest (`rest`)
- Space-ignorant equality (`equal_ignore_space`)
- Byte formatting (`bytes_to_readable_unit`)
- Hex conversion (`convert_char_non_ascii_to_hex`)
- BOM stripping (`strip_bom`)
- Extension stripping (`strip_file_extension`)
- Word wrapping (`word_wrap`)
