// Manually written
#![allow(non_snake_case)]
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Mutable<T: Clone>(Arc<std::sync::Mutex<T>>);

// `PartialEq` is a conditional impl rather than a struct-level bound so
// that `Mutable<T>` can store values whose `T` does not implement
// `PartialEq` (notably callbacks: `&impl Fn(...)` and similar). MM-level
// code only invokes structural equality on `Mutable<T>` when `T` itself
// is comparable.
impl<T: Clone + PartialEq> PartialEq for Mutable<T> {
    fn eq(&self, other: &Self) -> bool {
        let self_guard = self.0.lock().unwrap();
        let other_guard = other.0.lock().unwrap();
        *self_guard == *other_guard
    }
}

/// `Eq` mirrors `PartialEq` (both are content-based). Implemented as a
/// conditional impl on `T: Eq` so wrappers around non-`Eq` types still
/// compile; only callers that demand `Eq` on the wrapper pay the bound.
impl<T: Clone + Eq> Eq for Mutable<T> {}

/// Content-based ordering. Mirrors `PartialEq`'s "lock both, compare
/// inner values" pattern. Locks are always acquired self-then-other in
/// declaration order so the routine is deadlock-free against itself
/// (cross-thread Mutable comparisons assume no concurrent reordering of
/// the same pair of cells in opposite order, which the codegen never
/// generates).
impl<T: Clone + PartialOrd> PartialOrd for Mutable<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_guard = self.0.lock().unwrap();
        let other_guard = other.0.lock().unwrap();
        (*self_guard).partial_cmp(&*other_guard)
    }
}

impl<T: Clone + Ord> Ord for Mutable<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_guard = self.0.lock().unwrap();
        let other_guard = other.0.lock().unwrap();
        (*self_guard).cmp(&*other_guard)
    }
}

// `Hash` is deliberately NOT implemented. Like `std::cell::RefCell`, the
// interior value can change after a hash has been computed but before the
// hash is consumed; a hashable cell would silently corrupt hash containers
// when the contents mutated. Codegen excludes `Hash` from the derive set
// for any type that transitively contains a `Mutable` (in fact such types
// only carry `PartialEq` — see `derives_for` in `mmtorust/src/codegen.rs`).

/// `Default` is exposed so records containing a `Mutable<T>` can themselves
/// derive `Default` (used by the codegen lowering of `arrayCreateNoInit`
/// when the type-witness dummy is unassigned). The default `Mutable` holds a
/// freshly-locked `T::default()`.
impl<T: Clone + Default> Default for Mutable<T> {
    fn default() -> Self {
        Mutable(Arc::from(std::sync::Mutex::new(T::default())))
    }
}

pub fn create<T: Clone>(data: T) -> Mutable<T> {
    Mutable(Arc::from(std::sync::Mutex::new(data)))
}

pub fn update<T: Clone>(mutable: Mutable<T>, data: T) {
    let mut guard = mutable.0.lock().unwrap();
    *guard = data;
}

pub fn access<T: Clone>(mutable: Mutable<T>) -> T {
    mutable.0.lock().unwrap().clone()
}
