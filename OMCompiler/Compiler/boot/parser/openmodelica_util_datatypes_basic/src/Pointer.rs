// Manually written
#![allow(non_snake_case)]
use std::sync::{Arc, Mutex};
use metamodelica::*; // Built-in types and functions

// Mirrors the MetaModelica/C representation: `Mutable` corresponds to
// `mmc_mk_box1(0, data)` (ctor 0, in-place updatable) and `Immutable`
// corresponds to `mmc_mk_some(data)` (ctor 1, update rejected at runtime).
pub enum Pointer<T> {
    Mutable(Arc<Mutex<T>>),
    Immutable(Arc<T>),
}

impl<T> Clone for Pointer<T> {
    fn clone(&self) -> Self {
        match self {
            Pointer::Mutable(a) => Pointer::Mutable(Arc::clone(a)),
            Pointer::Immutable(a) => Pointer::Immutable(Arc::clone(a)),
        }
    }
}

impl<T> std::fmt::Debug for Pointer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pointer::Mutable(a) => write!(f, "Pointer::Mutable({:p})", Arc::as_ptr(a)),
            Pointer::Immutable(a) => write!(f, "Pointer::Immutable({:p})", Arc::as_ptr(a)),
        }
    }
}

impl<T: PartialEq> PartialEq for Pointer<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Pointer::Mutable(a), Pointer::Mutable(b)) => Arc::ptr_eq(a, b),
            (Pointer::Immutable(a), Pointer::Immutable(b)) => Arc::ptr_eq(a, b),
            _ => false,
        }
    }
}

/// `Eq` follows trivially from pointer equality — `Arc::ptr_eq` is
/// reflexive/symmetric/transitive — but we still gate it on `T: Eq` so
/// the bound stays in lockstep with `PartialEq` for callers.
impl<T: Eq> Eq for Pointer<T> {}

/// Total order by ctor tag (`Mutable` < `Immutable`) and then by the
/// pointer's numeric address. Used so types transitively containing a
/// `Pointer` (e.g. anything wrapping an `NFVariable`) can derive `Ord`
/// for hash-free containers like `BTreeMap` and the generated
/// `valueCompare` lowering. The ordering is stable across `clone()` of
/// the same `Pointer` because pointer-equal clones share the same
/// `Arc::as_ptr` address.
fn pointer_key<T>(p: &Pointer<T>) -> (u8, usize) {
    match p {
        Pointer::Mutable(a) => (0, Arc::as_ptr(a) as usize),
        Pointer::Immutable(a) => (1, Arc::as_ptr(a) as usize),
    }
}

impl<T: PartialOrd + PartialEq> PartialOrd for Pointer<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(pointer_key(self).cmp(&pointer_key(other)))
    }
}

impl<T: Ord> Ord for Pointer<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        pointer_key(self).cmp(&pointer_key(other))
    }
}

/// `Hash` mirrors the pointer-equality semantics: hash the ctor tag plus
/// the pointer address. Two pointers compare equal via [`PartialEq`] iff
/// they have the same tag and the same address, so this satisfies the
/// `k1 == k2 ⇒ hash(k1) == hash(k2)` contract.
impl<T> std::hash::Hash for Pointer<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        pointer_key(self).hash(state);
    }
}

pub fn create<T: Clone + PartialEq>(data: T) -> Pointer<T> {
    Pointer::Mutable(Arc::new(Mutex::new(data)))
}

pub fn createImmutable<T: Clone + PartialEq>(data: T) -> Pointer<T> {
    Pointer::Immutable(Arc::new(data))
}

// The MetaModelica/C runtime treats `update` as infallible: it writes through
// the box pointer without inspecting the ctor tag. Passing an immutable
// pointer is undefined behaviour at the C level; we surface that as a Rust
// panic rather than a `Result` because every caller would `unwrap`.
pub fn update<T: Clone + PartialEq>(mutable: Pointer<T>, data: T) {
    match mutable {
        Pointer::Mutable(cell) => {
            let mut guard = cell.lock().expect("Pointer.update: mutex poisoned");
            *guard = data;
        }
        Pointer::Immutable(_) => panic!("Pointer.update: tried to update an immutable Pointer"),
    }
}

pub fn access<T: Clone + PartialEq>(mutable: Pointer<T>) -> T {
    match mutable {
        Pointer::Mutable(cell) => {
            let guard = cell.lock().expect("Pointer.access: mutex poisoned");
            (*guard).clone()
        }
        Pointer::Immutable(a) => (*a).clone(),
    }
}

pub fn clone<T: Clone + PartialEq>(mutable: Pointer<T>) -> Pointer<T> {
    mutable
}

// `func` is a callback. Per the fallibility convention, function-pointer slots
// remain `fn(T) -> Result<T>` so the same slot can carry either fallible or
// infallible callees (codegen wraps infallible ones via `fnptr!`). The body
// itself is infallible — but it has to `?`-propagate failures from the
// callback. We can't, because we now return `T`. Resolve this by `unwrap()`ing
// the callback result: the MM analysis classified `Pointer.apply` infallible
// based on its callees, which is only sound when the callback itself never
// fails. Surface a misuse as a panic, consistent with the C runtime.
pub fn apply<T: Clone + PartialEq>(mutable: Pointer<T>, func: fn(T) -> anyhow::Result<T>) -> Pointer<T> {
    let new = func(access(mutable.clone())).unwrap();
    if !referenceEq(&new, &access(mutable.clone())) {
        update(mutable.clone(), new);
    }
    mutable
}
