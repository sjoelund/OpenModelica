// Manually written
use std::sync::Arc;
use anyhow::{Result};

#[derive(Clone, Debug)]
pub struct Mutable<T: Clone + PartialEq>(Arc<std::sync::Mutex<T>>);

impl<T: Clone + PartialEq> PartialEq for Mutable<T> {
    fn eq(&self, other: &Self) -> bool {
        let self_guard = self.0.lock().unwrap();
        let other_guard = other.0.lock().unwrap();
        *self_guard == *other_guard
    }
}

pub fn create<T: Clone + PartialEq>(data: T) -> Result<Mutable<T>> {
    Ok(Mutable(Arc::from(std::sync::Mutex::new(data))))
}

pub fn update<T: Clone + PartialEq>(mutable: Mutable<T>, data: T) -> Result<()> {
    let mut guard = mutable.0.lock().unwrap();
    *guard = data;
    Ok(())
}

pub fn access<T: Clone + PartialEq>(mutable: Mutable<T>) -> Result<T> {
    let mut guard = mutable.0.lock().unwrap();
    Ok(guard.clone())
}