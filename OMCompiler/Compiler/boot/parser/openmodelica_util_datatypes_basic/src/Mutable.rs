// Manually written
#![allow(non_snake_case)]
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Mutable<T: Clone + PartialEq>(Arc<std::sync::Mutex<T>>);

impl<T: Clone + PartialEq> PartialEq for Mutable<T> {
    fn eq(&self, other: &Self) -> bool {
        let self_guard = self.0.lock().unwrap();
        let other_guard = other.0.lock().unwrap();
        *self_guard == *other_guard
    }
}

pub fn create<T: Clone + PartialEq>(data: T) -> Mutable<T> {
    Mutable(Arc::from(std::sync::Mutex::new(data)))
}

pub fn update<T: Clone + PartialEq>(mutable: Mutable<T>, data: T) {
    let mut guard = mutable.0.lock().unwrap();
    *guard = data;
}

pub fn access<T: Clone + PartialEq>(mutable: Mutable<T>) -> T {
    mutable.0.lock().unwrap().clone()
}
