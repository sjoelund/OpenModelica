# SBInterval.rs - Assumptions and Notes

## Type Mapping

| MetaModelica | Rust |
|---|---|
| `Integer` | `i32` |
| `Real` | `f64` |
| `Boolean` | `bool` |
| `String` | `String` |
| `INTERVAL` record | `struct INTERVAL { lo: i32, step: i32, hi: i32 }` |
| `List<T>` / `UnorderedSet<T>` | `im::Vector<T>` (`List<T>`) |

## System Functions Mapped

- `System.intMaxLit()` → `i32::MAX`
- `System.intMaxLit() + 1` → overflow guard (in `new` function, checked against `i32::MAX`)
- `abs(x)` → `x.abs()` (via `.abs()` on integers)
- `mod(a, b)` → `a.rem_euclid(b)` (Euclidean remainder, matches MetaModelica semantics)
- `div(a, b)` → `a / b` (integer division; for positive operands this is floor)
- `ceil(x)` → `x.ceil()` (for `f64`)
- `floor(x)` → `x.floor()` (for `f64`)
- `integer(x)` → `x as i32` (float-to-int truncation, used on already-ceiled/floored values)
- `intReal(x)` → `(x as f64)` (int-to-float cast)
- `realInt(x)` → `x as i32` (float-to-int; in `cardinality` context not needed since we return `f64`)
- `intMin(a, b)` → `a.min(b)`
- `intMax(a, b)` → `a.max(b)`

## UnorderedSet → List Substitution

The original `complement` function returns `UnorderedSet<SBInterval>`. There is no Rust translation of `UnorderedSet` yet, so the return type was changed to `List<INTERVAL>` (`im::Vector<INTERVAL>`).

**Impact**: The returned collection is ordered and allows duplicates (though the algorithm doesn't produce duplicates). Consumers expecting set semantics (no duplicates, unordered) should handle this. The `hash` and `isEqual` callback functions passed to `UnorderedSet.new(hash, isEqual)` are dropped since `im::Vector` doesn't use them.

## crop Function Semantics

The original `crop` function takes `input output SBInterval int` and mutates `int.hi` in place. The Rust version takes `&INTERVAL` and returns a new `INTERVAL` (functional style), since Rust types implement `Clone` and the original mutation semantics are preserved through the new value.

## cardinality vs size

- `cardinality` returns `(hi - lo) as f64 / step as f64` (a real value). This matches the original MetaModelica which does `realInt(intReal(hi - lo) / intReal(step))` — note that the original casts to int *after* the division, but the return type annotation says `Integer` while the body says `realInt(...)` which truncates to int. The Rust version returns `f64` to preserve precision; use `(x as i32)` to match the original truncation if needed.
- `size` returns `(hi - lo) / step + 1` (integer count of elements).

## affine Function

The `gain` parameter is `f64` (mapped from `Real`). When `gain <= 0` and `offset <= 0`, the result is an empty interval. When `gain <= 0` but `offset > 0`, the result is a single-element interval `[offset, offset]` with step 1.

The intermediate calculations (`lo`, `step`, `hi`) are done in `f64` before being converted back to `i32` at the end.

## euclid Function (Protected)

The extended Euclidean algorithm returns a 4-tuple `(gcd, lcm, ua, vb)`. The `lcm` is computed as `abs(s2)` where `s2` is the Bézout coefficient for the second argument. This works because of the invariant maintained by the algorithm: `r2 = ua * a + vb * b` at each step, and at termination `r1 = gcd(a,b)` and `s2` happens to encode `lcm(a,b)`.

## Potential Issues

1. **Overflow**: The code uses `i32` throughout. Operations like `i2.lo + i * int1.step` in `complement` could overflow if `i` is large. The original MetaModelica code would also overflow here, but it's worth noting for safety-critical usage.

2. **Division by zero**: The `euclid` function is always called with `step > 0` (since intervals with `step == 0` are empty and handled before reaching `intersection`), but callers should ensure non-zero steps are passed.

3. **Negative dividends with `%`**: The original MetaModelica uses `mod(a, b)` which is the Euclidean modulo (always non-negative when `b > 0`). In Rust, `%` is the remainder operator (sign follows dividend), but we use `.rem_euclid()` everywhere, which matches MetaModelica semantics. The exception is the `euclid` function where we use `%` — this is safe because `euclid` operates on non-negative inputs (steps are always positive).
