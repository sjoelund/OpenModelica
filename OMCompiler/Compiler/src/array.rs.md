# Array.mo Translation Assumptions

## Type Mappings

| MetaModelica | Rust |
|---|---|
| `Integer` | `i32` |
| `Real` | `f64` |
| `Boolean` | `bool` |
| `String` | `String` |
| `array<T>` | `Vec<T>` |
| `list<T>` | `im::Vector<T>` (via `List<T>` alias) |
| `Option<T>` | `std::option::Option<T>` |
| `NONE()` | `None` |
| `SOME(x)` | `Some(x)` |
| `tuple<A,B>` | `(A, B)` |
| `FuncType` (inline) | Closure parameter |
| `ReduceFunc` (inline) | Closure parameter |
| `FoldFunc` (inline) | Closure parameter |
| `CompFunc` (inline) | Closure parameter |
| `PredFunc` (inline) | Closure parameter |
| `LessFn` (inline) | Closure parameter |
| `MapFunc` (inline) | Closure parameter |
| `Generator` (inline) | Closure parameter |
| `filterFunc` (inline) | Closure parameter |

## Indexing Conventions

- **MetaModelica uses 1-based indexing**; Rust uses 0-based indexing.
- Functions that accept index parameters (e.g., `replace_at_with_fill`, `expand_to_size`, `set_range`, `get_range`) treat the index as a **1-based MetaModelica size** (i.e., the actual vector length), not a 0-based offset. This is because `arrayCreate(size, value)` creates a vector of `size` elements.
- The `replace_at_with_fill(pos, ...)` function uses 1-based `pos` to determine the target size, then converts to 0-based for the actual assignment.
- The `map1_ind` function passes **1-based indices** to the closure to match MetaModelica behavior.
- The `fold_index` function passes **1-based indices** to the closure.

## Functions with Known Issues or Deviations

### `is_less` / `isLess`
The original MetaModelica code calls `lessFn(e2, e1)` (swapping arguments) when T1 != T2, but the `LessFn` type is defined as `LessFn(T1, T2)`. This is a **type error** in the original code. The Rust translation only calls `lessFn(&arr1[i], &arr2[i])` and uses length comparison as the tiebreaker.

### `toString` / `toString<T>`
The original MetaModelica uses a `match` expression on the list with pattern matching on `({}, true)` and `({}, false)`. The Rust translation reimplements this using `if/else` conditional logic. The `stringDelimitList` and `stringAppendList` functions from MetaModelica are replaced with Rust's `String::join()` and `format!()`.

### `heapSort`
The original uses a **min-heap** sort (based on the comparison direction in `downheap`), which produces **ascending** order. The Rust translation preserves this behavior.

### `filter`
The original MetaModelica uses a list comprehension with `guard` syntax to count elements to remove:
```metamodelica
new_size := arrayLength(arr) - sum(1 for e guard fun(e) in arr);
```
The Rust translation uses `arr.iter().filter(|e| f(e)).count()` to achieve the same result. Note that in MetaModelica, `fun(e) == true` means "remove this element" (the predicate selects elements to exclude).

### `mapFold`
The accumulator is passed by value (cloned) for each call since Rust closures don't have the same reference semantics as MetaModelica. This may have performance implications for large accumulator types.

### `downheap` / `heapSort` (protected)
These are `pub` in Rust but intended to be `protected` (module-private) like the original. They are only used internally by `heap_sort`.

## Assumptions

1. **`fail()` behavior**: The original `fail()` function causes a runtime abort. In Rust, these are translated to `panic!()` (via `bail!()` from `anyhow`) with descriptive error messages.

2. **`copy` vs `arrayCopy`**: The MetaModelica doc comment notes that `arrayCopy` is a builtin for duplicating arrays. The Rust `copy` function copies element-by-element (not a shallow copy), matching the MetaModelica semantics.

3. **`expandOnDemand`**: The original computes `new_size := realInt(intReal(len) * inExpansionFactor)`. The Rust translation uses `floor()` which may differ from MetaModelica's `realInt` truncation behavior for negative numbers.

4. **`consToElement` and `appendToElement`**: These operate on `array<list<T>>`. In Rust, they use `im::Vector` for the inner lists. The `::` (cons) operator in MetaModelica is translated to prepending via `im::vector![element]`.

5. **`mapNoCopy` and `mapNoCopy_1`**: These modify the input array in-place. In Rust, the input is `&mut Vec<T>` which enforces exclusive mutation at compile time.

6. **`threadMap`**: The original `fail()` on mismatched lengths is replaced with `panic!()` in Rust.

7. **`insertList`**: The original uses `input output` parameter, meaning the array is modified in-place. Rust uses `&mut Vec<T>`.

## Things That Might Not Work as Expected

1. **Generic type constraints**: The original MetaModelica does not require `Clone` or `PartialEq` constraints on type parameters. The Rust translation adds these bounds where needed (e.g., `T: Clone` for copying elements). This means types that work in MetaModelica may need to implement `Clone` in Rust.

2. **Performance**: The Rust translation uses element-by-element copying where MetaModelica's `copy`, `copyRange`, and `copyN` functions might use bulk memory operations (e.g., `memcpy`). For performance-critical code, consider using `slice::copy_from_slice()` for `Copy` types.

3. **`mapList` behavior**: The MetaModelica `mapList` starts iterating from `listRest(inList)` with index 2 (1-based). The Rust translation uses `.skip(1)` which achieves the same result but the index semantics are implicit rather than explicit.

4. **`getRange` return order**: The MetaModelica code prepends elements to the result list in a loop (`outList := value::outList`), which reverses the order. The Rust translation appends elements in forward order.

5. **`isEqual` return value**: The MetaModelica function returns `true` by default and only sets `outIsEqual := false` when a mismatch is found. The Rust version returns `Ok(true)` or `Ok(false)`.

6. **`allEqual` on empty arrays**: The MetaModelica function returns (via `return`) without setting the output when the array is empty. In Rust, we explicitly return `true` for empty arrays, matching the default `= true` initialization.
