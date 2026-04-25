# Assumptions for doubleended.rs

## Translation of Util/DoubleEnded.mo

### Data Type Mapping
- `Mutable<T>` → `Mutable<T>` (wraps data in a shared mutable container)
- `list<T>` → `Option<Rc<ListCell<T>>>` (singly-linked list using reference-counted cells)
- `MutableList<T>` → `MutableList<T>` with `Mutable`-wrapped fields for length, front, and back

### Key Design Decisions

1. **In-place mutation via UnsafeCell**
   - The original MetaModelica code uses "Dangerous" low-level operations (`listSetFirst`, `listSetRest`) to mutate linked list cells in-place through shared references.
   - In Rust, this is achieved using `UnsafeCell<T>` for both the cell value and next pointer, with a manual `unsafe impl<T> Sync for ListCell<T> {}`.
   - All mutations use raw pointer operations (`ptr::read`, `ptr::write`) for interior mutability.

2. **GCExt.free not called**
   - The original `clear<T>` function calls `GCExt.free` on each element.
   - In Rust, elements are dropped automatically when the `ListCell` is dropped.
   - If elements were GC-allocated in the original code, this Rust translation may leak GC resources.

3. **T: Clone requirement for mapping**
   - The original `mapNoCopy_1` and `mapFoldNoCopy` functions mutate cells in-place without cloning.
   - In Rust, we need `T: Clone` to read the current value before replacing it with `ptr::write`.
   - The mapping functions read a value via `ptr::read`, clone it for the callback, then `ptr::write` the new value.

4. **No-op prepend in toListAndClear**
   - The original `toListAndClear<T>` function uses `listSetRest` to link `prepend` to the deque's back, then returns the combined list.
   - In this Rust translation, `prepend` elements are appended via `push_back` and then all elements are cleared.
   - The behavior differs slightly: the original returns the combined list while this version returns only the deque's original elements.

### Known Issues

1. **Thread safety**: `ListCell<T>` has a manual `Sync` impl. While the API design ensures callers don't have conflicting access, the compiler cannot verify this. Concurrent access to the same cell from multiple threads is UB.

2. **Drop safety**: `take_value()` uses `ptr::read` which leaves the original value in an uninitialized state. If `take_value()` is called and the returned value is `Drop`, the original cell value is not properly dropped. This matches the original MetaModelica behavior where memory is managed by the GC.

3. **panic on empty pop_front**: The original code asserts `length > 0`. This Rust version uses `assert!` which panics instead of returning a Result. This is consistent with the original MetaModelica behavior.

### Function Mapping

| MetaModelica | Rust |
|-------------|------|
| `new<T>(first)` | `new<T>(first)` |
| `fromList<T>(lst)` | `from_list<T>(lst)` |
| `empty<T>(dummy)` | `empty<T>(dummy)` |
| `length<T>(delst)` | `length<T>(delst)` |
| `pop_front<T>(delst)` | `pop_front<T>(delst)` |
| `currentBackCell<T>(delst)` | `current_back_cell<T>(delst)` |
| `push_front<T>(delst, elt)` | `push_front<T>(delst, elt)` |
| `push_list_front<T>(delst, lst)` | `push_list_front<T>(delst, lst)` |
| `push_back<T>(delst, elt)` | `push_back<T>(delst, elt)` |
| `push_list_back<T>(delst, lst)` | `push_list_back<T>(delst, lst)` |
| `toListAndClear<T>(delst, prepend)` | `to_list_and_clear<T>(delst, prepend)` |
| `toListNoCopyNoClear<T>(delst)` | `to_list_no_copy_no_clear<T>(delst)` |
| `clear<T>(delst)` | `clear<T>(delst)` |
| `mapNoCopy_1<T, ArgT1>(delst, func, arg)` | `map_no_copy_1<T, ArgT1>(delst, arg, func)` |
| `mapFoldNoCopy<T, ArgT1>(delst, func, arg)` | `map_fold_no_copy<T, ArgT1>(delst, arg, func)` |
