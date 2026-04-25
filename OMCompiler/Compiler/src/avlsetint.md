# AvlSetInt Translation Notes

## Assumptions

- `AvlSetInt` specializes `BaseAvlSet` with `Key = Integer` (`i32`). The generic `baseavlset` module already handles all tree operations, so `avlsetint` re-exports them and adds i32-specific helpers.
- `baseavlset` is declared as a module in `main.rs` before `avlsetint` so it can be imported via `crate::baseavlset`.
- `keyStr` in MetaModelica uses `String(inKey)` — in Rust this maps to `format!("{in_key}")`.
- `keyCompare` uses `sign(inKey2 - inKey1)` which returns -1, 0, or 1.

## Things That Might Not Work As Expected

- The `baseavlset` module is generic and its public functions are not directly used in tests (only the i32-specific `key_str_fn` and `key_compare_fn` are tested). All generic functionality is inherited through re-exports.
- `Tree<K>` uses `Box<Tree<K>>` for NODE children, which adds indirection compared to the original MetaModelica's recursive type. This is necessary in Rust due to the unknown size of recursive types.
- The `List` type alias in `baseavlset` (mapped from MetaModelica `list<T>`) is unused in this module — it remains in `baseavlset` for future use.

## Structure

- `Key` = `i32`
- `Tree` = `baseavlset::Tree<i32>` (type alias)
- `key_str_fn` = i32-specific `keyStr` (format integer to string)
- `key_compare_fn` = i32-specific `keyCompare` (sign-based comparison)
- All other operations are re-exported from `baseavlset` unchanged
