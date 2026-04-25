# AvlSetString Translation Notes

## Assumptions

1. **String keys use Rust's `String` type** — MetaModelica `String` maps to Rust `String` (heap-allocated). All key comparisons use Rust's `Ord` trait for `String`.

2. **Generic BaseAvlSet in Rust** — The Rust translation of `BaseAvlSet` uses generics (`Tree<K: Ord + Clone>`) rather than the MetaModelica `replaceable` mechanism. This means `AvlSetString` doesn't need its own tree implementation — it reuses the generic one with `K = String`.

3. **keyStr identity mapping** — The MetaModelica `keyStr` function simply returns the input key as-is (`outString := inKey`). The Rust equivalent (`key_str_fn`) is an identity conversion.

4. **keyCompare uses Rust's string ordering** — The MetaModelica `keyCompare` calls `stringCompare(inKey1, inKey2)`, which returns -1/0/1 for less/equal/greater. The Rust `key_compare_fn` achieves the same using `str::cmp`.

## Things to watch out for

1. **Performance** — Using `String` (owned heap-allocated strings) as keys means every insertion requires cloning the string. For better performance in hot paths, consider using `&str` or interned strings later.

2. **No actual specialization needed** — Since the Rust BaseAvlSet is already generic, `AvlSetString` is essentially a re-export with String-specific convenience functions. The `Key` and `Tree` type aliases make the intended usage explicit.

3. **All base functions re-exported** — Functions like `add`, `list_keys`, `join`, `intersection` etc. are re-exported from `baseavlset` for API compatibility. They work identically since the generic implementation already handles `String` keys (via `Ord + Clone` bounds).

4. **The `List<T>` type from baseavlset uses `im::Vector`** — This maps correctly to MetaModelica `list<Key>` but the ordering is preserved through the AVL tree structure, not the list itself.

5. **matchcontinue not used** — This particular package doesn't use the `matchcontinue` construct, so no `Result`/`bail!` machinery is needed for the AvlSetString-specific code.
