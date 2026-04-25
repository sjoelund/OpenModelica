# BaseHashSet Translation Assumptions

## Overview

Translation of `Util/BaseHashSet.mo` to `src/basehashset.rs`.

## Key Design Decisions

### Generic Key Type
The MetaModelica `replaceable type Key subtypeof Any` maps to a Rust generic `K: Clone + Debug + Send + Sync + 'static`. The actual key type is chosen by the consuming module.

### Function Callbacks
Since Rust generics cannot infer hash/equality/key-to-string functions, these are passed as `Arc`-wrapped closures stored in `FuncsTuple<K>`. The `FuncHash`, `FuncEq`, and `FuncKeyString` wrappers use `Arc` internally so they're cheaply cloneable when cloning a `HashSet`.

### ValueArray Design
The `ValueArray` uses a two-index system:
- `n` (element count / next insertion position)
- `size` (capacity / total vec length)

Elements are stored at positions 0 through n-1. The underlying `Vec<Option<K>>` is pre-allocated to `size` elements. When `n < size`, the next insertion goes at position `n` (not appended). When `n >= size`, the array expands by 1.4x.

### Bucket Collision Handling
Each bucket in the `HashVector` stores a `List<(Key, Integer)>` where the integer is the position in `ValueArray`. Collisions are handled by chaining in this linked list, searched linearly with the `FuncEq` callback.

## Things That May Not Work As Expected

1. **Original code has a bug**: The original MetaModelica `add` function does NOT use `valueArrayAdd` - it directly pushes to the end of the array. This means in the original, the first element would be stored at position `capacity` (end of pre-allocated array) rather than position 0. Our Rust translation fixes this by using `valueArrayAdd` properly.

2. **Memory overhead**: The `ValueArray` pre-allocates based on `bucketSize * 0.6`. For `DEFAULT_BUCKET_SIZE` (2053), this creates a vec of 1231 `None` elements. The actual data starts at position 0 and grows upward. This is more memory-efficient than the original which pushed to the end.

3. **Hash collisions**: Collisions are handled by linked list chaining. For large sets with many collisions, lookup performance degrades to O(n) in the worst case. The original code uses the same approach.

4. **Delete doesn't compact**: The `delete` function only marks the entry as `None` in the `ValueArray` - it doesn't remove the entry from the `HashVector` index table or compact the array. Repeated additions and deletions will not free memory. This matches the original MetaModelica behavior (see the function's own documentation).

5. **No iteration order guarantee**: `hashSetList` returns elements in the order they appear in the `ValueArray`, which is insertion order (0, 1, 2, ...). This is NOT sorted or hashed order.

6. **`FuncKeyString::call` is unused**: The `key_string` function is only used for debugging output (`printHashSet`, `dumpHashSet`). Since Rust's `Debug` trait is used for printing instead, this closure is effectively unused in the current implementation.

## Differences from Original

| Aspect | MetaModelica | Rust |
|--------|-------------|------|
| Indexing | 1-based | 0-based (converted at boundary) |
| Option type | `NONE()` / `SOME(v)` | `None` / `Some(v)` |
| List | `list<T>` | `im::Vector<T>` |
| Array | `array<T>` | `Vec<T>` |
| Hash function | `hashFunc(key)` | `funcs.hash.call(&key)` |
| Equality | `keyEqual(k1, k2)` | `funcs.eq.call(&k1, &k2)` |
| matchcontinue | Native syntax | Not needed (no pattern matching on tuples in same way) |
