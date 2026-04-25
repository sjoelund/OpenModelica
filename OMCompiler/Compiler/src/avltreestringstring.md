# AvlTreeStringString - Assumptions & Notes

## Package Description
`AvlTreeStringString` is an AVL tree (key-value map) where both the key and value types are `String`. It is a specialization of `BaseAvlTree` that redeclares `keyStr`, `valueStr`, and `keyCompare` for string-to-string mapping.

## Translation Approach
Since `BaseAvlTree` is already generic in Rust (`Tree<K, V>`), this translation follows the same pattern as `AvlTreeString`: the base module provides the generic tree operations, and this module provides type aliases and String/String-specific helper functions.

## Assumptions
1. **Generic base** - The Rust `baseavltree` module already implements all tree operations (add, get, to_list, etc.) generically. This module re-exports them for `Tree<String, String>`.
2. **String comparison** - MetaModelica's `stringCompare` returns -1, 0, or 1. The Rust `key_compare_fn` uses the same ordering via `str::cmp` semantics.
3. **keyStr and valueStr are identity functions** - Both simply return the input as-is, since the types are already String.
4. **No addUpdate function in interface** - The interface.mo shows the public API only includes `keyStr`, `valueStr`, `keyCompare`, plus all inherited BaseAvlTree functions.

## Things That Might Not Work as Expected
1. **Performance** - Using `String` as both key and value means heap allocations on every tree operation. For performance-critical code, `&str` references could be used but would complicate the API.
2. **referenceEq** - The MetaModelica code uses `referenceEq` to check if two values are the same reference. In Rust, we use `PartialEq::eq` (structural equality) instead. This is a semantic difference but typically produces the same results for `String` values.
3. **Mutable output parameters** - The MetaModelica `input output Tree tree` pattern is translated to Rust by taking ownership of the tree and returning a new one (via `add`, `join`, etc.) rather than using mutable references.

## Tests
Unit tests are included in the `.rs` file covering:
- `key_str_fn` and `value_str_fn` (identity)
- `key_compare_fn` (ordering)
- Basic tree operations: add, get, get_opt, to_list, list_keys
