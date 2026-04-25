# ExpandableArray.mo Translation Assumptions

## Type Mappings

| MetaModelica | Rust |
|---|---|
| `Integer` | `i32` |
| `Real` | `f64` |
| `Boolean` | `bool` |
| `String` | `String` |
| `array<T>` | `Vec<T>` |
| `list<T>` | `im::Vector<T>` |
| `Option<T>` | `std::option::Option<T>` |
| `NONE()` | `None` |
| `SOME(x)` | `Some(x)` |
| `Mutable<T>` | `Mutable<T>` (wrapper struct in `mutable` module) |
| `PrintFunction` | Closure `Fn(&T) -> String` |
| `fail()` | `bail!(...)` from `anyhow` |
| `arrayCreate(n, val)` | `vec![val; n]` |
| `arrayCopy(arr)` | `arr.clone()` |
| `arrayUpdate(arr, i, val)` | `arr[i-1] = val` |
| `arrayGet(arr, i)` | `arr[i-1]` |
| `isSome(opt)` | `opt.is_some()` |
| `getOption(opt)` | `opt.unwrap()` / `.as_ref().unwrap()` |
| `max(a, b)` | `a.max(b)` |

## Indexing Conventions

- **MetaModelica uses 1-based indexing**; this Rust translation preserves 1-based indexing in all public API functions (`set`, `get`, `add`, `delete`, `update`, `occupied`) to match the original interface.
- Internally, 1-based indices are converted to 0-based for Rust `Vec` access via `index - 1`.

## Mutable Semantics

- The original MetaModelica uses `Mutable<T>` wrappers to simulate pass-by-reference for immutable values. In Rust, we use `&mut ExpandableArray<T>` for functions that mutate the array, and `&ExpandableArray<T>` for read-only access.
- The `Mutable<T>` struct wraps data that needs to be shared/mutated, similar to the original `array<>` construct.

## Functions with Known Issues or Deviations

### `set`
The original checks `index > 0 and (index > capacity or isNone(...))`. The Rust version checks if the index is out of bounds OR if the slot is already occupied. The capacity auto-expansion doubles from the current capacity (starting from `max(capacity, 1)`).

### `clear`
The original stops early when `n == 0` (all elements have been cleared). The Rust version replicates this optimization but uses a counter that increments as elements are cleared.

### `compress`
The original uses nested while loops with `Dangerous` no-bounds-checking access. The Rust version uses safe bounds-checked access with `.min()` and `.saturating_sub()` to prevent panics.

### `shrink`
The original calls `compress` first, then creates a new array of size `numberOfElements` by copying element-by-element. The Rust version replicates this behavior.

### `toString`
The original uses a `PrintFunction` partial function (a nested type with its own `input/output`). The Rust version uses a closure parameter `Fn(&T) -> String`. The debug mode formatting differs slightly: `"(n / C)"` vs `"(n )"` to match the original's `intString(capacity)` inclusion.

### `getNumberOfElements`, `getLastUsedIndex`, `getCapacity`, `getData`
These are simple accessors that directly read from the `Mutable` wrappers. In the original MetaModelica they use default output parameter syntax; in Rust they return values directly.

## Assumptions

1. **Clone requirement**: All generic type parameters require `Clone` because the expandable array stores and copies values. The original MetaModelica does not have this constraint.

2. **fail() behavior**: The original `fail()` causes a runtime abort. In Rust, these are translated to `bail!()` from `anyhow` which returns a `Result` error. This means functions like `get`, `set`, `delete`, and `update` now return `Result<T>` instead of potentially aborting.

3. **expandToSize uses array::expand_to_size**: The `array::expand_to_size` function is called from the `array` module. This function expects 1-based sizes, matching the MetaModelica convention.

4. **No-bounds-checking**: The original uses `Dangerous.arrayGetNoBoundsChecking` and `Dangerous.arrayUpdateNoBoundsChecking` for performance. The Rust translation uses safe, bounds-checked access with `.min()` and `.saturating_sub()` to prevent panics on out-of-bounds access.

5. **Memory layout**: The `ExpandableArray<T>` struct in Rust derives `Clone`, which means cloning an array clones the entire data vector. The original MetaModelica's `copy` function does the same (deep copy).

6. **capacity field**: The `capacity` field in the struct tracks the allocated capacity. It is updated in `new`, `expand_to_size`, and `shrink`, but NOT during the automatic growth in `set`. This matches the original MetaModelica behavior where `capacity` is updated after `expandToSize` is called.

## Things That Might Not Work as Expected

1. **Thread safety**: The original `Mutable<T>` type in OpenModelica uses shared references with garbage collection for thread safety. The Rust version uses `&mut` for exclusive mutation, which is stricter. Concurrent access would require `Arc<Mutex<T>>` or similar.

2. **Performance of clone operations**: Every `compress`, `copy`, and `shrink` operation clones the entire data vector. For large arrays, this could be expensive compared to the original C-based implementation.

3. **Iterator invalidation in compress**: The `compress` function has complex nested loops that shift elements. The Rust implementation uses safe bounds checking which adds overhead but prevents undefined behavior.

4. **Empty array edge cases**: Functions like `get` and `delete` on empty arrays return errors, while the original would call `fail()`. Callers need to handle the `Result` properly.

5. **toString PrintFunction**: The original `PrintFunction` is a partial function defined inside `toString`, allowing it to be overridden by callers. The Rust version uses a closure parameter instead, which is more flexible but has a different calling convention.
