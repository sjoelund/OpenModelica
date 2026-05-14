// Manually written
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;

pub type Mutable<T: Clone + PartialEq> = Arc<std::sync::Mutex<T>>;
pub fn create<T: Clone + PartialEq>(data: T) -> Result<Mutable<T>> {
    Ok(Arc::from(std::sync::Mutex::new(data)))
}

pub fn update<T: Clone + PartialEq>(mutable: Mutable<T>, data: T) -> Result<()> {
    let mut guard = mutable.lock().unwrap();
    *guard = data;
    Ok(())
}

pub fn access<T: Clone + PartialEq>(mutable: Mutable<T>) -> Result<T> {
    let mut guard = mutable.lock().unwrap();
    Ok(guard.clone())
}