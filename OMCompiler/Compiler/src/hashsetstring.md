# HashSetString.rs - Translation Notes

## Summary

Translates `Util/HashSetString.mo` into `src/hashsetstring.rs`. This module provides a string-specific HashSet built on top of the generic `BaseHashSet` module.

## Assumptions

1. **DJB2 Hash Algorithm**: The MetaModelica builtin `stringHashDjb2` is the standard DJB2 hash algorithm: `hash = hash * 31 + char`, starting from 0. Implemented using `wrapping_mul` and `wrapping_add` for overflow safety.

2. **String Equality**: `stringEq` maps directly to Rust's `==` operator for `String` types.

3. **Identity Function**: `Util.id` maps to a simple clone of the input string.

4. **Generic BaseHashSet**: The Rust `BaseHashSet` uses generics (`HashSet<K>`) rather than a tuple-based type, which is more idiomatic in Rust. `HashSetString` works with `K = String`.

5. **No matchcontinue Needed**: `HashSetString.mo` contains only two simple wrapper functions (`emptyHashSet` and `emptyHashSetSized`) that delegate to `BaseHashSet.emptyHashSetWork`. No `matchcontinue` semantics are required.

6. **1-based Indexing**: The MetaModelica HashSet uses 1-based indexing internally (array access at `hashvec[indx + 1]`). This is already handled in `BaseHashSet.rs` which uses 0-based indexing, so the Rust translation is already 0-based.

## Things That Might Not Work As Expected

1. **Hash Collisions**: The DJB2 hash implementation iterates over bytes (`s.bytes()`) rather than characters (`chars()`). For ASCII strings this is identical. For multi-byte UTF-8, this produces the same result as the MetaModelica implementation (which operates on byte values).

2. **Overflow Behavior**: Rust's `wrapping_mul` and `wrapping_add` match C's unsigned integer overflow semantics. The MetaModelica implementation uses 32-bit signed integers (`Integer`), so wrapping behavior should be consistent.

3. **Type Aliases Only**: The `Key`, `HashSet`, `FuncHashCref`, `FuncCrefEqual`, and `FuncCrefStr` type aliases are defined for API compatibility with the MetaModelica interface, but the actual HashSet type is `basehashset::HashSet<Key>`.

4. **No Clone of HashSet**: The `empty_hash_set_sized` function creates a fresh empty HashSet. Adding elements requires cloning the HashSet reference and modifying the clone (consistent with BaseHashSet's immutable semantics).

5. **FuncsTuple Construction**: Each call to `empty_hash_set_sized` creates new `FuncHash`, `FuncEq`, and `FuncKeyString` closures. For performance, consider creating a shared `FuncsTuple` constant if these functions are called frequently.

## API Reference

| MetaModelica | Rust |
|---|---|
| `emptyHashSet()` | `empty_hash_set()` |
| `emptyHashSetSized(size)` | `empty_hash_set_sized(size)` |
| `Key` (type alias) | `Key` (type alias = `String`) |
| `HashSet` (type) | `HashSet` (type alias = `BaseHashSet<Key>`) |
| `HashSetCrefFunctionsType` | Not separately exposed (part of `FuncsTuple`) |
| `FuncHashCref` | `FuncHashCref` (type alias) |
| `FuncCrefEqual` | `FuncCrefEqual` (type alias) |
| `FuncCrefStr` | `FuncCrefStr` (type alias) |
