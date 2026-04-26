# Assumptions and Notes for hashtablestringtoprogram.rs

## Translation of Util/HashTableStringToProgram.mo

This module provides a hash table mapping String keys to Absyn::Program values.

### Key Assumptions

1. **djb2 hash algorithm**: The `stringHashDjb2` MetaModelica built-in is implemented as:
   ```
   h = (h << 5).wrapping_add(h).wrapping_add(c)
   ```
   This matches the standard djb2 algorithm where `h = h * 33 + c`.

2. **Absyn::Program type**: The `Value` type alias refers to `crate::absyn::Program`. This type must be available from the absyn module.

3. **FuncValString callback**: Since `Absyn::Program` has no natural string representation defined in this module, a dummy string `"<dummy Absyn::Program>"` is used for the value-to-string conversion function (equivalent to the MetaModelica `dummyStr` function).

4. **BaseHashTable infrastructure**: The generic BaseHashTable infrastructure from basehashset.rs is reused, extended with a 4-field `FuncsTuple4` (hash, eq, key_string, val_string) since BaseHashTable uses a 4-tuple compared to BaseHashSet's 3-tuple.

### Additional Infrastructure Added

The following types were added to basehashset.rs to support the 4-field FuncsTuple required by BaseHashTable:
- `FuncValString<V>` - boxed function type for value-to-string conversion
- `FuncsTuple4<K, V>` - 4-field functions tuple for BaseHashTable
- `BaseHashTable<K, V>` - hash table struct with 4-field funcs tuple
- `empty_base_hash_table_work()` - constructor for empty BaseHashTable

### Not Translated

The partial functions (`FuncHashCref`, `FuncCrefEqual`, `FuncCrefStr`, `FuncExpStr`) in the MetaModelica code are type declarations for function callbacks. In Rust, these are represented as type aliases to the boxed function types from basehashset.rs.

### Potential Issues

1. **Send/Sync bounds**: The `Value` type (`Absyn::Program`) does not require `Send + Sync` because complex AST types may not implement these traits. This means the hash table is not safe to share across threads.

2. **No operations beyond empty creation**: The MetaModelica code also imports BaseHashTable operations (add, delete, get, etc.), but those are inherited from the generic BaseHashTable. Only the `emptyHashTable` and `emptyHashTableSized` functions are unique to this module.

3. **String equality**: Uses Rust's built-in `==` for string comparison, which is equivalent to the MetaModelica `stringEq` function (`s1 == s2`).
