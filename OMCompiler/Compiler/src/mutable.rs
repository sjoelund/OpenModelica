//! Translation of Util/Mutable.mo
//!
//! This module provides routines for creating and updating mutable (shared) objects,
//! similar to array<> structures. It implements the `Mutable<T>` uniontype
//! from the OpenModelica Util package.

use std::fmt;

// ============================================================================
// Mutable uniontype
// ============================================================================

/// A mutable (shared) object wrapper.
///
/// Provides a generic container for mutable data, similar to array<> structures
/// in the original OpenModelica code.
#[derive(Debug, Clone, PartialEq)]
pub struct Mutable<T> {
    pub data: T,
}

impl<T> fmt::Display for Mutable<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Mutable({})", self.data)
    }
}

// ============================================================================
// Functions
// ============================================================================

/// Creates a new `Mutable<T>` wrapper containing the given data.
///
/// Equivalent to the `mutableCreate` external C function in OpenModelica.
/// In the original runtime this uses `mmc_mk_box1` for garbage-collected boxing.
pub fn create<T>(data: T) -> Mutable<T> {
    Mutable { data }
}

/// Updates the data inside a `Mutable<T>` with new data.
///
/// Equivalent to the `mutableUpdate` external C function in OpenModelica.
/// In the original runtime this accesses `MMC_STRUCTDATA(mutable)[0]` directly.
pub fn update<T>(mutable: &mut Mutable<T>, data: T) {
    mutable.data = data;
}

/// Accesses the data inside a `Mutable<T>`.
///
/// Equivalent to the `mutableAccess` external C function in OpenModelica.
/// In the original runtime this reads `MMC_STRUCTDATA(mutable)[0]`.
pub fn access<T>(mutable: &Mutable<T>) -> &T {
    &mutable.data
}

/// Mutably accesses the data inside a `Mutable<T>`.
pub fn access_mut<T>(mutable: &mut Mutable<T>) -> &mut T {
    &mut mutable.data
}
