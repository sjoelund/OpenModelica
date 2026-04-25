# Assumptions and Notes for src/pointer.rs

## Assumptions

1. **Pointer struct wrapper**: The original OpenModelica `Pointer<T>` is a uniontype with C-level boxing via `mmc_mk_box1`. In Rust, this is represented as a simple generic struct `Pointer<T>` with a single `data: T` field.

2. **`createImmutable`**: The original uses `mmc_mk_some(data)` as a builtin. This is translated to the same `Pointer { data }` constructor. In the original runtime, `mmc_mk_some` produces an immutable marker. This distinction is not enforced in Rust since Rust's type system doesn't have the same kind of runtime immutability markers.

3. **`referenceEq`**: The original OpenModelica uses `referenceEq` to check if two values refer to the same memory location (pointer comparison). In Rust, `reference_eq` uses `std::ptr::eq` for reference comparison when applicable, falling back to `PartialEq` value comparison for non-reference types.

4. **`apply` function**: The original `apply` creates a `new` value from `func(access(mutable))`, then checks `referenceEq(new, access(mutable))` before updating. In the Rust translation, we clone the original data, apply the function in-place, then compare with the cloned original. This is functionally equivalent but requires `T: Clone`.

5. **`update` immutability check**: The original C code checks `valueConstructor(ptr)!=0` and throws `MMC_THROW_INTERNAL()` if true. The Rust translation does not enforce this check — it trusts the caller not to pass immutable pointers to `update`.

6. **Thread data**: The original `update` takes `OpenModelica.threadData()` as a parameter for the C runtime. The Rust translation omits this since Rust doesn't use the same threading model.

7. **`clone` default parameter**: The original `clone` has `input output Pointer<T> mutable = create(access(mutable))` with a default parameter. In Rust, `clone` takes an immutable reference `&Pointer<T>` since it returns a new value rather than modifying in place.

8. **`Func` callback type**: The original uses a partial function type `Func` with `input output T value`. In Rust, this is represented as `impl FnOnce(&mut T)` — a closure that receives a mutable reference to the value.

## Things That Might Not Work as Expected

1. **No runtime immutability enforcement**: The distinction between `create` and `createImmutable` is not enforced at runtime in Rust. Both produce identical `Pointer` structs. If strict immutability is needed, consider using `std::sync::RwLock` or a custom marker type.

2. **Clone overhead**: The `apply` function requires `T: Clone` to compare before/after values. This may have performance implications for large types. An alternative would be to use a `Copy` bound where possible.

3. **No garbage collection**: The original runtime uses `mmc_mk_box1` which integrates with OpenModelica's garbage collector. The Rust translation uses plain heap-allocated structs. If GC integration is needed, consider using `Rc<Pointer<T>>`.

4. **No `matchcontinue` support**: This module does not use `matchcontinue`, so no special handling was needed.
