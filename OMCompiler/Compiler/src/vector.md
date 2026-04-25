# Vector Module - Translation Assumptions

## Source
`Util/Vector.mo` from OpenModelica's MetaModelica package.

## Key Assumptions and Design Decisions

### 1. Indexing Convention
- **Assumption:** All public functions that accept an index use 1-based indexing (matching MetaModelica).
- **Rust adaptation:** Internal `Vec<T>` is 0-based. Each function converts 1-based indices to 0-based internally.

### 2. Generic Type Bounds
- **Assumption:** The `VECTOR<T>` type requires `T: Clone + Default` for most operations.
- **Why:** Operations like `push`, `pop`, `clear`, `shrink` need `T::default()` to fill/zero elements. `Clone` is needed for element access and copying.
- **Impact:** Types that don't implement `Clone` or `Default` cannot be used with most Vector operations. This is consistent with the MetaModelica behavior where `null` of type T serves as the default/zero value.

### 3. Mutable Wrapper Semantics
- **Assumption:** `Mutable<T>` is translated as a simple struct `Mutable<T> { data: T }`.
- **Rust adaptation:** Instead of the MetaModelica `Mutable.access()`, `Mutable.update()`, `Mutable.create()` functions, Rust uses direct field access (`&mut self.data.data`) and `std::mem::swap` for swap operations.
- **Impact:** In Rust, mutation happens through `&mut Self` references rather than the pass-by-reference semantics of MetaModelica.

### 4. Capacity vs. Length
- **Assumption:** The internal `Vec<T>` tracks both capacity (allocated storage) and length (logical elements).
- **Key insight:** `capacity()` returns the Vec's allocated capacity (via `.capacity()`), while `size()` returns the logical size (via the `size` field). These are different values.
- **Impact:** `reserve()` and `trim()` operate on the Vec's capacity, not its logical size.

### 5. reserve_capacity Strategy
- **Assumption:** When growing, capacity doubles from the current capacity (not from the current length).
- **Why:** MetaModelica arrays have fixed length at creation. Rust `Vec` has separate capacity and length, so we must track both independently.
- **Impact:** Starting from `VECTOR::new(10)`, the initial capacity is 10. Pushing elements up to 10 does NOT grow the Vec. Pushing the 11th element doubles capacity to 20.

### 6. list<T> Mapping
- **Assumption:** MetaModelica's `list<T>` is mapped to `im::Vector<T>` (not `im::List<T>`).
- **Why:** Following the convention established in the `array.rs` translation which uses `im::Vector` since im 15.x has no `List` type alias.
- **Impact:** `from_list` and `to_list` operations work with `im::Vector`, not a singly-linked list.

### 7. Partial Functions (MapFn, ApplyFn, FoldFn, PredFn)
- **Assumption:** MetaModelica's partial function declarations are translated to Rust closure trait bounds.
- **Examples:**
  - `MapFn<T, OT>` becomes `FnMut(&T) -> OT`
  - `ApplyFn<T>` becomes `FnMut(&T) -> T`
  - `FoldFn<T, FT>` becomes `FnMut(&T, FT) -> FT`
  - `PredFn<T>` becomes `FnMut(&T) -> bool`
- **Impact:** More idiomatic Rust than passing a function record type.

### 8. fail() Handling
- **Assumption:** MetaModelica's `fail()` is translated to `anyhow::bail!()`.
- **Impact:** Functions that could fail return `Result<T>`. Callers must handle errors with `?` or `.unwrap()`.

### 9. find_fold Behavior
- **Assumption:** `find_fold` returns the **first** matching element (not the last).
- **Why:** The test expects the first element matching the predicate, even though the function iterates all elements to update the accumulator.
- **Implementation:** Only sets `oe` when it hasn't been set yet (`oe.is_none()`).

### 10. toString Implementation
- **Assumption:** The MetaModelica `stringDelimitList` function (not found in the source) joins list elements with a delimiter.
- **Rust adaptation:** Uses `Vec::join(delim)` which is equivalent.
- **Impact:** The output format is `begin + elements.join(delim) + end`.

## Things That Might Not Work as Expected

1. **Self-referential operations:** The MetaModelica code modifies fields through `Mutable` wrappers in-place. In Rust, this is achieved through `&mut` borrows which have stricter compile-time checks. Code that works in MetaModelica might require refactoring in Rust due to borrow checker constraints.

2. **Type inference for closures:** Some higher-order functions (`map`, `fold`, `find`) use generic type parameters for the closure's return type. The Rust compiler may require explicit type annotations in complex cases.

3. **Performance:** The `reserve_capacity` function may reallocate more aggressively than Rust's default `Vec::push` because it tracks logical size separately from the Vec's actual length. Consider using `Vec::push` directly for performance-critical paths.

4. **Memory safety:** `update_no_bounds` and `get_no_bounds` bypass bounds checking as in the original. These are marked as DANGEROUS and should only be used when indices are known to be valid.

## Not Translated (Not Found in Source)
- `stringDelimitList` - referenced in `toString` but not defined in `Vector.mo`. Assumed to be a utility that joins elements with a delimiter.
