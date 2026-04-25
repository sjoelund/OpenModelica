# Mutable.rs - Assumptions

## Assumptions Made

1. **Struct-based representation**: The `Mutable<T>` uniontype from OpenModelica is translated to a Rust struct. The original uses a garbage-collected box (`mmc_mk_box1`) but Rust's ownership system handles this natively.

2. **Direct field access**: The C implementation accesses `MMC_STRUCTDATA(mutable)[0]` - the first field of the struct. Our `data` field maps directly to this.

3. **Generic parameter**: The OpenModelica type defaults to `polymorphic<Any>` when `T` is unspecified. Rust requires the type parameter to be specified at use site (no default generics), so callers must write `Mutable<SomeType>` explicitly.

4. **Pass by reference for `update`**: The original C function takes a pointer to the mutable struct and modifies it in place. In Rust, we accept `&mut Mutable<T>` to express this mutability explicitly.

5. **Return reference for `access`**: Returns `&T` (immutable borrow) rather than cloning/copying the data, matching the semantics of the original which returns a pointer to the data.

## Things That Might Not Work as Expected

1. **Memory management differences**: The original uses OpenModelica's garbage collector (`mmc_mk_box1`). Rust uses ownership/borrowing. If the original code relies on the GC to manage lifetime, Rust's borrow checker may require structural changes.

2. **Type erasure**: The C code uses `void*` which provides runtime type erasure. Rust's generics are monomorphized at compile time - there's no direct equivalent to `void*` without using `Box<dyn Any>` (which requires `static` trait bounds).

3. **Clone semantics**: `Mutable<T>` derives `Clone`, which requires `T: Clone`. If `T` doesn't implement `Clone`, calling `clone()` on `Mutable<T>` will fail at compile time. The original C code simply copies a pointer.

4. **No unsafe operations**: The original C code performs direct memory access via macros. This translation uses safe Rust. If the original relies on specific memory layouts for interop with other C code, additional `#[repr(C)]` annotations would be needed.
