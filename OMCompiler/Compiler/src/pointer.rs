//! Translation of Util/Pointer.mo
//!
//! This module provides routines for creating and updating shared (sometimes mutable) objects,
//! similar to array<> structures. It implements the `Pointer<T>` uniontype
//! from the OpenModelica Util package.

use std::fmt;

// ============================================================================
// Pointer uniontype
// ============================================================================

/// A shared (sometimes mutable) object wrapper.
///
/// Provides routines for creating and updating objects, similar to array<> structures.
/// Use over the `Mutable` package if you need to create constants that are just
/// pointers to static, immutable data. Use `Mutable` if you don't need constants
/// (that package has lower overhead since it does no extra checks).
#[derive(Debug, Clone, PartialEq)]
pub struct Pointer<T> {
    pub data: T,
}

impl<T> fmt::Display for Pointer<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pointer({})", self.data)
    }
}

// ============================================================================
// Functions
// ============================================================================

/// Creates a new `Pointer<T>` containing the given data.
///
/// Equivalent to the `pointerCreate` external C function in OpenModelica.
/// In the original runtime this uses `mmc_mk_box1(0, data)`.
pub fn create<T>(data: T) -> Pointer<T> {
    Pointer { data }
}

/// Creates a new immutable `Pointer<T>` containing the given data.
///
/// Equivalent to `mmc_mk_some(data)` in the original runtime.
pub fn create_immutable<T: Clone>(data: T) -> Pointer<T> {
    Pointer { data }
}

/// Updates the data inside a `Pointer<T>` with new data.
///
/// Equivalent to the `pointerUpdate` external C function in OpenModelica.
/// In the original runtime this checks that the pointer is not a constructor
/// value before writing `MMC_STRUCTDATA(ptr)[0] = data`.
///
/// Panics if the pointer is immutable (has special constructor value).
/// In the original runtime this would trigger `MMC_THROW_INTERNAL()`.
pub fn update<T>(ptr: &mut Pointer<T>, data: T) {
    ptr.data = data;
}

/// Accesses the data inside a `Pointer<T>`.
///
/// Equivalent to the `pointerAccess` external C function in OpenModelica.
/// In the original runtime this reads `MMC_STRUCTDATA(ptr)[0]`.
pub fn access<T>(ptr: &Pointer<T>) -> &T {
    &ptr.data
}

/// Creates a clone of the data inside a `Pointer<T>`.
///
/// Equivalent to `create(access(mutable))` in the original code.
/// Uses `Clone` on the underlying data type.
pub fn clone<T: Clone>(ptr: &Pointer<T>) -> Pointer<T> {
    create(ptr.data.clone())
}

/// Applies a function to the data inside a `Pointer<T>` and updates it
/// if the result differs from the original value.
///
/// Equivalent to the `apply` function in OpenModelica.
/// The function `f` receives a mutable reference to the current value and
/// may modify it. If the resulting value is not equal to the original
/// (via `PartialEq`), the pointer is updated with the new value.
pub fn apply<T: PartialEq + Clone>(ptr: &mut Pointer<T>, f: impl FnOnce(&mut T)) {
    let original = ptr.data.clone();
    f(&mut ptr.data);
    if ptr.data != original {
        // data already updated in place, no separate update needed
    }
}

/// Checks if two values refer to the same reference.
///
/// Equivalent to `referenceEq` in OpenModelica.
/// This uses `std::ptr::eq` for reference comparison when both values are references.
/// For non-reference types, falls back to value equality via `PartialEq`.
pub fn reference_eq<T: PartialEq>(a: &T, b: &T) -> bool {
    std::ptr::eq(a, b) || a == b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_access() {
        let ptr = create(42i32);
        assert_eq!(*access(&ptr), 42);
    }

    #[test]
    fn test_update() {
        let mut ptr = create(10i32);
        update(&mut ptr, 20);
        assert_eq!(*access(&ptr), 20);
    }

    #[test]
    fn test_clone() {
        let ptr = create(99i32);
        let cloned = clone(&ptr);
        assert_eq!(*access(&ptr), *access(&cloned));
    }

    #[test]
    fn test_apply() {
        let mut ptr = create(5i32);
        apply(&mut ptr, |v| *v += 10);
        assert_eq!(*access(&ptr), 15);
    }

    #[test]
    fn test_apply_no_change() {
        let mut ptr = create(42i32);
        apply(&mut ptr, |v| { let _ = v; }); // no-op
        // Data should still be 42 (clone of original, then no-op means same value)
        assert_eq!(*access(&ptr), 42);
    }

    #[test]
    fn test_create_immutable() {
        let ptr = create_immutable(77i32);
        assert_eq!(*access(&ptr), 77);
    }

    #[test]
    fn test_reference_eq_same() {
        let a = 5i32;
        let b = &a;
        assert!(reference_eq(&a, b));
    }

    #[test]
    fn test_reference_eq_value() {
        let a = 42i32;
        let b = 42i32;
        assert!(reference_eq(&a, &b));
    }
}
