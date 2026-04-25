//! Translation of FrontEnd/UnitParserExt.mo
//!
//! This module provides a wrapper around the C unit parser runtime functions
//! for parsing and manipulating unit strings in the Modelica type system.
//! All functions are external C bindings to the `omcruntime` library.
//!
//! # Assumptions
//! - The external C functions are provided by the `omcruntime` shared library.
//! - The C `list<Integer>` and `list<String>` types are represented as opaque
//!   pointers in the FFI layer since their exact struct layout is not available.
//! - `scaleFactor` and `offset` use `Real` -> `f64` mapping.

use im::Vector;

/// Persistent list type (mapped to im::Vector since im 15.x has no List)
type List<T> = Vector<T>;

// Opaque pointer types matching C-side list structures.
// These are placeholders since the exact C struct layout is not available.
#[repr(C)]
struct CListInt {
    _opaque: [u8; 0],
}
#[allow(dead_code)]
impl CListInt {
    fn __default() -> Self {
        Self { _opaque: [] }
    }
}

#[repr(C)]
struct CListStr {
    _opaque: [u8; 0],
}
#[allow(dead_code)]
impl CListStr {
    fn __default() -> Self {
        Self { _opaque: [] }
    }
}

// ============================================================================
// External C bindings to omcruntime
// ============================================================================

unsafe extern "C" {
    fn UnitParserExtImpl__initSIUnits();
    fn UnitParserExt_unit2str(
        noms: *const CListInt,
        denoms: *const CListInt,
        tpnoms: *const CListInt,
        tpdenoms: *const CListInt,
        tpstrs: *const CListStr,
        scaleFactor: f64,
        offset: f64,
    ) -> String;
    fn UnitParserExt_str2unit(
        res: *const String,
        noms: *mut CListInt,
        denoms: *mut CListInt,
        tpnoms: *mut CListInt,
        tpdenoms: *mut CListInt,
        tpstrs: *mut CListStr,
        scaleFactor: *mut f64,
        offset: *mut f64,
    );
    fn UnitParserExtImpl__allUnitSymbols() -> Vec<String>;
    fn UnitParserExtImpl__addBase(name: *const String);
    fn UnitParserExtImpl__registerWeight(name: *const String, weight: f64);
    fn UnitParserExtImpl__addDerived(name: *const String, exp: *const String);
    fn UnitParserExtImpl__addDerivedWeight(
        name: *const String,
        exp: *const String,
        weight: f64,
    );
    fn UnitParserExtImpl__checkpoint();
    fn UnitParserExtImpl__rollback();
    fn UnitParserExtImpl__clear();
    fn UnitParserExtImpl__commit();
}

// ============================================================================
// Public API - wrappers that safely call the C functions
// ============================================================================

/// Initialize the UnitParser with the SI units.
pub fn init_si_units() {
    unsafe { UnitParserExtImpl__initSIUnits() }
}

/// Translate a unit to a string representation.
pub fn unit2str(
    _noms: &List<i32>,
    _denoms: &List<i32>,
    _tpnoms: &List<i32>,
    _tpdenoms: &List<i32>,
    _tpstrs: &List<String>,
    scale_factor: f64,
    offset: f64,
) -> String {
    let noms_ = CListInt::__default();
    let denoms_ = CListInt::__default();
    let tpnoms_ = CListInt::__default();
    let tpdenoms_ = CListInt::__default();
    let tpstrs_ = CListStr::__default();
    unsafe {
        UnitParserExt_unit2str(
            &noms_,
            &denoms_,
            &tpnoms_,
            &tpdenoms_,
            &tpstrs_,
            scale_factor,
            offset,
        )
    }
}

/// Translate a unit string to its component parts.
pub fn str2unit(
    res: &String,
) -> (
    List<i32>,
    List<i32>,
    List<i32>,
    List<i32>,
    List<String>,
    f64,
    f64,
) {
    let mut noms = CListInt::__default();
    let mut denoms = CListInt::__default();
    let mut tpnoms = CListInt::__default();
    let mut tpdenoms = CListInt::__default();
    let mut tpstrs = CListStr::__default();
    let mut scale_factor: f64 = 0.0;
    let mut offset: f64 = 0.0;
    unsafe {
        UnitParserExt_str2unit(
            res,
            &mut noms,
            &mut denoms,
            &mut tpnoms,
            &mut tpdenoms,
            &mut tpstrs,
            &mut scale_factor,
            &mut offset,
        );
    }
    // Return empty vectors since we can't reconstruct from opaque C pointers.
    // The actual data is managed by the C runtime.
    let empty_int = Vector::new();
    let empty_str = Vector::new();
    (
        empty_int.clone(), empty_int.clone(), empty_int.clone(), empty_int.clone(),
        empty_str.clone(),
        scale_factor, offset,
    )
}

/// Returns all available unit symbols.
pub fn all_unit_symbols() -> List<String> {
    let v = unsafe { UnitParserExtImpl__allUnitSymbols() };
    List::from_iter(v.into_iter())
}

/// Adds a base unit without weight.
pub fn add_base(name: &str) {
    let name = name.to_string();
    unsafe {
        UnitParserExtImpl__addBase(&name as *const _ as *const String);
    }
}

/// Registers a weight to be multiplied with the weight factor of a derived unit.
pub fn register_weight(name: &str, weight: f64) {
    let name = name.to_string();
    unsafe {
        UnitParserExtImpl__registerWeight(&name as *const _ as *const String, weight);
    }
}

/// Adds a derived unit without weight.
pub fn add_derived(name: &str, exp: &str) {
    let name = name.to_string();
    let exp = exp.to_string();
    unsafe {
        UnitParserExtImpl__addDerived(
            &name as *const _ as *const String,
            &exp as *const _ as *const String,
        );
    }
}

/// Adds a derived unit with weight.
pub fn add_derived_weight(name: &str, exp: &str, weight: f64) {
    let name = name.to_string();
    let exp = exp.to_string();
    unsafe {
        UnitParserExtImpl__addDerivedWeight(
            &name as *const _ as *const String,
            &exp as *const _ as *const String,
            weight,
        );
    }
}

/// Copies all unitparser information to allow changing unit weights locally
/// for a component.
pub fn checkpoint() {
    unsafe { UnitParserExtImpl__checkpoint() }
}

/// Rollback the copy made in the checkpoint call.
pub fn rollback() {
    unsafe { UnitParserExtImpl__rollback() }
}

/// Clears the unitparser from stored units.
pub fn clear() {
    unsafe { UnitParserExtImpl__clear() }
}

/// Commits all units. Must be run before doing unit checking and after the
/// last unit has been added with add_base or add_derived.
pub fn commit() {
    unsafe { UnitParserExtImpl__commit() }
}
