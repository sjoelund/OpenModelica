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
