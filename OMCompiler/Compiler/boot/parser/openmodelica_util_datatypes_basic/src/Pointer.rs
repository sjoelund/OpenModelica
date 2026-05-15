// Manually written
#![allow(non_snake_case)]
use std::sync::{Arc, Mutex};
use anyhow::{Result, bail};
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

impl<T: PartialEq> PartialEq for Pointer<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Pointer::Mutable(a), Pointer::Mutable(b)) => Arc::ptr_eq(a, b),
            (Pointer::Immutable(a), Pointer::Immutable(b)) => Arc::ptr_eq(a, b),
            _ => false,
        }
    }
}

pub fn create<T: Clone + PartialEq>(data: T) -> Result<Pointer<T>> {
    Ok(Pointer::Mutable(Arc::new(Mutex::new(data))))
}

pub fn createImmutable<T: Clone + PartialEq>(data: T) -> Result<Pointer<T>> {
    Ok(Pointer::Immutable(Arc::new(data)))
}

pub fn update<T: Clone + PartialEq>(mutable: Pointer<T>, data: T) -> Result<()> {
    match mutable {
        Pointer::Mutable(cell) => {
            let mut guard = cell.lock().expect("Pointer.update: mutex poisoned");
            *guard = data;
            Ok(())
        }
        Pointer::Immutable(_) => bail!("Pointer.update: tried to update an immutable Pointer"),
    }
}

pub fn access<T: Clone + PartialEq>(mutable: Pointer<T>) -> Result<T> {
    match mutable {
        Pointer::Mutable(cell) => {
            let guard = cell.lock().expect("Pointer.access: mutex poisoned");
            Ok((*guard).clone())
        }
        Pointer::Immutable(a) => Ok((*a).clone()),
    }
}

pub fn clone<T: Clone + PartialEq>(mutable: Pointer<T>) -> Result<Pointer<T>> {
    Ok(mutable)
}

pub fn apply<T: Clone + PartialEq>(mutable: Pointer<T>, func: fn(T) -> Result<T>) -> Result<Pointer<T>> {
    let new = func(access(mutable.clone())?)?;
    if !(referenceEq(&new, &access(mutable.clone())?)?) {
        update(mutable.clone(), new)?;
    }
    Ok(mutable)
}


