# Assumptions - ClockIndexes

## Translation of Util/ClockIndexes.mo to src/clockindexes.rs

### Datatypes
- `Integer` mapped to `i32` (as per CLAUDE.md)
- `list<Integer>` mapped to `im::Vector<i32>` via a `List<T>` type alias (im 15.x has no List, uses Vector)

### Design decisions
1. **`to_string` returns `&'static str`** - The original function returns a `String` output, but since all mappings are compile-time constants, returning a static string reference is more efficient. Callers can always convert with `.to_string()` if needed.

2. **`buildModelClocks` as a function** - The original code has this as a constant list, but Rust doesn't allow const construction of `im::Vector`. It's exposed as a function `build_model_clocks()` instead.

3. **`RT_NO_CLOCK` constant** - The original has `RT_NO_CLOCK = -1`. This constant is marked as unused since no other translated code currently references it.

### Things that might not work as expected
- **No fallthrough in Rust match** - The MetaModelica `match` statement is directly translated to a Rust `match`. Both have exhaustive matching, so the `else "ERR"` clause maps to `_ => "ERR"`. This is correct behavior.
- **Package encapsulation** - The original is wrapped in `package ClockIndexes`. Rust doesn't have package encapsulation, so all items are public `pub const` and `pub fn` at module level.
- **No `matchcontinue`** - The `toString` function uses a regular `match`, not `matchcontinue`, so no special handling or Result propagation is needed.
