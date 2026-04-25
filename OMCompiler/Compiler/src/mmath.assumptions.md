# Assumptions and Caveats for mmath.rs

## Source
`FrontEnd/MMath.mo` - A package providing rational number arithmetic operations.

## Translation Approach
Direct translation of the MMath package from MetaModelica to Rust. The `Rational` uniontype with its `RATIONAL` record variant maps to a Rust enum with a struct variant. All arithmetic operations simplify results by dividing by GCD.

## Key Assumptions

1. **Integer type mapping**: `Integer` in MetaModelica maps to `i64` (not `i32`) to avoid overflow during intermediate calculations (e.g., `i1*i4 + i3*i2` for rational addition). The original MO code uses `Integer` throughout.

2. **GCD for negative numbers**: Uses `rem_euclid` for modulo operation, which always returns a non-negative remainder. This ensures `int_gcd` always returns a non-negative GCD regardless of input signs, which is then used for division to normalize signs correctly.

3. **normalizeZero behavior**: When the numerator is zero after arithmetic, the denominator is forced to 1. This ensures `0/6` normalizes to `0/1`, consistent with the MO source.

4. **is_greater_than uses float comparison**: The original MO code uses `realGt(r1.nom/r1.denom, r2.nom/r2.denom)` which converts to float for comparison. This can lose precision for very large rationals. The `equals` function uses exact cross-multiplication instead.

5. **intGcd recursion**: The Euclidean algorithm is translated as a recursive function. For typical inputs this is fine, but pathological inputs could cause stack overflow.

6. **Division by zero not guarded**: If r2 has a zero denominator, the division operation will panic (integer division by zero). The original MO code does not guard against this either.

## Things That May Not Work as Expected

- **Overflow**: Intermediate products in arithmetic (e.g., `i1*i4`) use `i64`. For very large rational numbers, overflow may occur silently. The original MO code likely had the same issue but with its native integer type.
- **matchcontinue translation**: The `testRational` function uses `matchcontinue` which is translated into a sequential check with error propagation via `anyhow::Result`. The semantics are preserved but the control flow structure differs.
- **RATIONAL constructor name**: The `RATIONAL` helper function in tests uses uppercase to match the MetaModelica constructor name. This violates Rust's `non_snake_case` convention but preserves the original naming.

## Mapping Summary

| MetaModelica Type | Rust Type |
|---|---|
| `Integer` | `i64` |
| `Real` | `f64` |
| `Boolean` | `bool` |
| `String` | `String` |
| `Rational` (uniontype) | `Rational` enum |
| `RATIONAL` (record) | `Rational::RATIONAL { nom, denom }` variant |
| `uniontype` | `enum` with struct variants |
| Recursive function | Direct Rust recursion |
| `matchcontinue` | Sequential checks + `Result` error propagation |
