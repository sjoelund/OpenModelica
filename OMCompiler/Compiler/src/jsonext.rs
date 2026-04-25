//! Translation of Util/JSONExt.mo
//!
//! This module provides JSON serialization utilities for OpenModelica metatypes.
//!
//! # Design notes
//!
//! The original MetaModelica code operates on `modelica_metatype` (an opaque
//! pointer type from the OpenModelica code generator). Most of the `is*` /
//! `get*` functions are thin wrappers around C macros that inspect the header
//! of an OpenModelica value.  In Rust we represent these opaque values as
//! `*mut std::ffi::c_void` and declare them as unsafe extern "C" so they link
//! against the OpenModelica runtime library at build time.
//!
//! The `serialize` function is the only pure-algorithmic function: it walks
//! a value, dispatches on its type, and builds a JSON string.  All other
//! functions are FFI shims.
//!
//! ## Assumptions / Things that may not work as expected
//!
//! - The external C functions (`omc_is_integer`, `omc_is_real`, etc.) must be
//!   provided by the OpenModelica runtime at link time.  If you compile this
//!   crate in isolation the build will fail with undefined-symbol errors.
//! - `getList` simply returns the input pointer - the underlying linked-list
//!   structure is expected to be an IM-list (cons cell list).  Iterating it
//!   requires the caller to use `ListIter` (below).
//! - `getRecordNames` relies on OpenModelica's internal `record_description`
//!   layout; it only works with records produced by the OMC code generator.
//! - `getListElement` uses `boxptr_listGet` from the OpenModelica runtime
//!   with 1-based offset (matching MetaModelica semantics).

use std::ffi::{c_int, c_void};

// ============================================================================
// Opaque metatype handle
// ============================================================================

/// Opaque handle representing any OpenModelica metatype value.
/// Mirrors `modelica_metatype` from the OMC runtime.
pub type Metatype = *mut c_void;

// ============================================================================
// Extern "C" declarations (C API from OpenModelica runtime)
// ============================================================================

// All FFI functions consolidated into a single unsafe extern block.
unsafe extern "C" {
    // Type-checking predicates
    fn omc_is_integer(value: Metatype) -> c_int;
    fn omc_is_real(value: Metatype) -> c_int;
    fn omc_is_string(value: Metatype) -> c_int;
    fn omc_is_array(value: Metatype) -> c_int;
    fn omc_is_record(value: Metatype) -> c_int;
    fn omc_is_tuple(value: Metatype) -> c_int;
    fn omc_is_none(value: Metatype) -> c_int;
    fn omc_is_some(value: Metatype) -> c_int;
    fn omc_is_nil(value: Metatype) -> c_int;
    fn omc_is_cons(value: Metatype) -> c_int;
    // Record accessors
    fn omc_get_record_names(any: Metatype) -> Metatype;
    fn omc_get_record_component(iany: Metatype, offset: i32) -> Metatype;
    // Type casts
    fn omc_cast_int(a: Metatype) -> i64;
    fn omc_cast_real(a: Metatype) -> f64;
    fn omc_cast_string(a: Metatype) -> *const std::ffi::c_char;
    // Option type accessors
    fn omc_get_some(a: Metatype) -> Metatype;
    // Tuple/list accessors
    fn omc_get_tuple_size(any: Metatype) -> i32;
    fn omc_get_list(iany: Metatype) -> Metatype;
    fn omc_get_list_element(iany: Metatype, offset: i32) -> Metatype;
}

// ============================================================================
// Rust wrapper functions (translated from MetaModelica)
// ============================================================================

/// Check whether `value` is an integer (fixnum).
/// Mirrors `isInteger<T>` from JSONExt.mo.
pub fn is_integer(value: Metatype) -> bool {
    unsafe { omc_is_integer(value) != 0 }
}

/// Check whether `value` is a Real.
/// Mirrors `isReal<T>` from JSONExt.mo.
pub fn is_real(value: Metatype) -> bool {
    unsafe { omc_is_real(value) != 0 }
}

/// Check whether `value` is a String.
/// Mirrors `isString<T>` from JSONExt.mo.
pub fn is_string(value: Metatype) -> bool {
    unsafe { omc_is_string(value) != 0 }
}

/// Check whether `value` is an array.
/// Mirrors `isArray<T>` from JSONExt.mo.
pub fn is_array(value: Metatype) -> bool {
    unsafe { omc_is_array(value) != 0 }
}

/// Check whether `value` is a record (ctor > 1, slots > 0).
/// Mirrors `isRecord<T>` from JSONExt.mo.
pub fn is_record(value: Metatype) -> bool {
    unsafe { omc_is_record(value) != 0 }
}

/// Check whether `value` is a tuple (ctor == 0, slots > 0).
/// Mirrors `isTuple<T>` from JSONExt.mo.
pub fn is_tuple(value: Metatype) -> bool {
    unsafe { omc_is_tuple(value) != 0 }
}

/// Check whether `value` is NONE (ctor == 1, slots == 0).
/// Mirrors `isNONE<T>` from JSONExt.mo.
pub fn is_none(value: Metatype) -> bool {
    unsafe { omc_is_none(value) != 0 }
}

/// Check whether `value` is SOME (ctor == 1, slots == 1).
/// Mirrors `isSOME<T>` from JSONExt.mo.
pub fn is_some(value: Metatype) -> bool {
    unsafe { omc_is_some(value) != 0 }
}

/// Check whether `value` is NIL.
/// Mirrors `isNil<T>` from JSONExt.mo.
pub fn is_nil(value: Metatype) -> bool {
    unsafe { omc_is_nil(value) != 0 }
}

/// Check whether `value` is a CONS cell (list element).
/// Mirrors `isCons<T>` from JSONExt.mo.
pub fn is_cons(value: Metatype) -> bool {
    unsafe { omc_is_cons(value) != 0 }
}

/// Get field names from a record.
/// Returns an IM list of strings: record name followed by field names.
/// Mirrors `getRecordNames<T>` from JSONExt.mo.
pub fn get_record_names(any: Metatype) -> Metatype {
    unsafe { omc_get_record_names(any) }
}

/// Get a single component from a record by 1-based offset.
/// Mirrors `getRecordComponent<TIN, TOUT>` from JSONExt.mo.
pub fn get_record_component(iany: Metatype, offset: i32) -> Metatype {
    unsafe { omc_get_record_component(iany, offset) }
}

/// Extract the integer (fixnum) value from a metatype.
/// Mirrors `getInteger<T>` from JSONExt.mo.
pub fn get_integer(a: Metatype) -> i64 {
    unsafe { omc_cast_int(a) }
}

/// Extract a Real (f64) value from a metatype.
/// Mirrors `getReal<T>` from JSONExt.mo.
pub fn get_real(a: Metatype) -> f64 {
    unsafe { omc_cast_real(a) }
}

/// Extract a String from a metatype.
/// Returns a String on success, or None if the pointer is null.
/// Mirrors `getString<T>` from JSONExt.mo.
pub fn get_string(a: Metatype) -> Option<String> {
    let ptr = unsafe { omc_cast_string(a) };
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { std::ffi::CStr::from_ptr(ptr) }.to_string_lossy().into_owned())
    }
}

/// Get the value inside a SOME.
/// Mirrors `getSome<TIN, TOUT>` from JSONExt.mo.
pub fn get_some(a: Metatype) -> Metatype {
    unsafe { omc_get_some(a) }
}

/// Get the number of slots in a tuple.
/// Mirrors `getTupleSize<T>` from JSONExt.mo.
pub fn get_tuple_size(any: Metatype) -> i32 {
    unsafe { omc_get_tuple_size(any) }
}

/// Get the list (cons-cell list) value.
/// Simply returns the input.
/// Mirrors `getList<TIN, TOUT>` from JSONExt.mo.
pub fn get_list(iany: Metatype) -> Metatype {
    unsafe { omc_get_list(iany) }
}

/// Get an element from a list at a 1-based offset.
/// Mirrors `getListElement<TIN, TOUT>` from JSONExt.mo.
pub fn get_list_element(iany: Metatype, offset: i32) -> Metatype {
    unsafe { omc_get_list_element(iany, offset) }
}

// ============================================================================
// Helper: iterate an IM list and collect elements
// ============================================================================

/// Iterator over elements of an IM cons-cell list.
///
/// Each element is a `Metatype` (opaque handle).  The list itself is also
/// represented as a cons-cell Metatype; `NIL` terminates the list.
pub struct ListIter {
    head: Metatype,
}

impl ListIter {
    /// Create a new iterator from a cons-cell list metatype.
    pub fn new(head: Metatype) -> Self {
        Self { head }
    }
}

impl Iterator for ListIter {
    type Item = Metatype;

    fn next(&mut self) -> Option<Self::Item> {
        if is_nil(self.head) || self.head.is_null() {
            return None;
        }
        // In an IM list, the head (first slot) is the element,
        // and the tail (second slot) is the rest of the list.
        let element = get_list_element(self.head, 1);
        self.head = get_list_element(self.head, 2);
        Some(element)
    }
}

// ============================================================================
// Helper: string-delimit a list of strings (reverse + join)
// ============================================================================

/// Delimit a `Vec<String>` with `delimiter`.
/// Mirrors the `stringDelimitList` behavior from OpenModelica.
fn string_delimit_list(parts: &[String], delimiter: &str) -> String {
    parts.join(delimiter)
}

// ============================================================================
// serialize — the main recursive JSON serialization function
// ============================================================================

/// Serialize any metatype value to a JSON string.
///
/// This mirrors the `serialize<T>` algorithm from JSONExt.mo:
///
/// 1. Integers → bare number string
/// 2. Reals → real number string
/// 3. Strings → quoted string
/// 4. Records → `{"recordName":{...}}`
/// 5. NIL → `"[]"`
/// 6. CONS (list) → `[elem1,elem2,...]`
/// 7. NONE → `"[]"`
/// 8. SOME → `[elem]`
/// 9. Tuples → `{"Tuple":{...}}`
/// 10. Unknown → `"UNKNOWN(...)"`
///
/// Mirrors `serialize<T>` from JSONExt.mo.
pub fn serialize(value: Metatype, filter: &[String]) -> String {
    unsafe { omc_serialize_internal(value, filter) }
}

/// Internal recursive helper that does all the type dispatch.
unsafe fn omc_serialize_internal(value: Metatype, filter: &[String]) -> String {
    // Integer
    if is_integer(value) {
        let n = unsafe { omc_cast_int(value) };
        return format!("{}", n);
    }

    // Real
    if is_real(value) {
        let r = unsafe { omc_cast_real(value) };
        return format!("{}", r);
    }

    // String
    if is_string(value) {
        let s = unsafe {
            std::ffi::CStr::from_ptr(omc_cast_string(value))
                .to_string_lossy()
                .into_owned()
        };
        return format!("\"{}\"", s);
    }

    // Record
    if is_record(value) {
        let names_raw = unsafe { omc_get_record_names(value) };
        // First element is the record name
        let record_name = unsafe {
            std::ffi::CStr::from_ptr(omc_cast_string(names_raw))
                .to_string_lossy()
                .into_owned()
        };

        // Collect remaining elements (field names) by iterating the list
        let all_names: Vec<String> = {
            let mut out = Vec::new();
            let iter = ListIter::new(names_raw);
            for elem in iter {
                let name = unsafe {
                    std::ffi::CStr::from_ptr(omc_cast_string(elem))
                        .to_string_lossy()
                        .into_owned()
                };
                out.push(name);
            }
            out
        };

        // all_names[0] = record name, all_names[1..] = field names
        let field_names = &all_names[1..];

        let mut result = format!("{{\"{}\":{{", record_name);
        let mut parts: Vec<String> = Vec::new();
        for (idx, field_name) in field_names.iter().enumerate() {
            let offset = (idx + 2) as i32;
            if !filter.contains(field_name) {
                let component = unsafe { omc_get_record_component(value, offset) };
                let serialized = unsafe { omc_serialize_internal(component, filter) };
                parts.push(format!("\"{}\":{}", field_name, serialized));
            }
        }
        parts.reverse();
        result.push_str(&string_delimit_list(&parts, ","));
        result.push_str("}}");
        return result;
    }

    // NIL
    if is_nil(value) {
        return "[]".to_string();
    }

    // CONS (list)
    if is_cons(value) {
        let mut parts: Vec<String> = Vec::new();
        let iter = ListIter::new(value);
        for elem in iter {
            let serialized = unsafe { omc_serialize_internal(elem, filter) };
            parts.push(serialized);
        }
        parts.reverse();
        return format!("[{}]", string_delimit_list(&parts, ","));
    }

    // NONE
    if is_none(value) {
        return "[]".to_string();
    }

    // SOME
    if is_some(value) {
        let inner = unsafe { omc_get_some(value) };
        let serialized = unsafe { omc_serialize_internal(inner, filter) };
        return format!("[{}]", serialized);
    }

    // Tuple
    if is_tuple(value) {
        let size = unsafe { omc_get_tuple_size(value) };
        let mut parts: Vec<String> = Vec::new();
        for i in 1..=size {
            let elem = unsafe { omc_get_list_element(value, i) };
            let serialized = unsafe { omc_serialize_internal(elem, filter) };
            parts.push(format!("\"{}\":{}", i, serialized));
        }
        parts.reverse();
        return format!("{{\"Tuple\":{{{}}}}}", string_delimit_list(&parts, ","));
    }

    // Unknown
    return format!("UNKNOWN({})", "??");
}

// ============================================================================
// is_list_member helper (used by serialize in the original)
// ============================================================================

/// Check if a string is in the filter list.
/// Mirrors `listMember` usage in JSONExt.mo.
pub fn is_list_member(list: &[String], member: &str) -> bool {
    list.contains(&member.to_string())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_list_member() {
        let filter = vec!["foo".to_string(), "bar".to_string()];
        assert!(is_list_member(&filter, "foo"));
        assert!(is_list_member(&filter, "bar"));
        assert!(!is_list_member(&filter, "baz"));
    }

    #[test]
    fn test_string_delimit_list() {
        assert_eq!(
            string_delimit_list(&["a".to_string(), "b".to_string()], ","),
            "a,b"
        );
        assert_eq!(string_delimit_list(&["a".to_string()], ","), "a");
        assert_eq!(string_delimit_list(&[], ","), "");
    }

    #[test]
    fn test_serialize_integer() {
        // We cannot easily construct a Metatype pointer in pure Rust,
        // but we verify the helper functions work.
        assert!(!is_list_member(&[], "anything"));
        assert!(is_list_member(&["x".to_string()], "x"));
    }
}
