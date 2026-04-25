//! Translation of HpcOmBenchmarkExt.mo
//!
//! This module provides bindings to the HpcOmBenchmarkExt adapter package
//! used for benchmark calculation in c/c++. The implementation relies on the
//! `omcruntime` C library.
//!
//! All functions are external C calls - no pure Rust implementation provided.
//! The `extern "C"` declarations mirror the C API from omcruntime.

use std::ffi::{CString, c_char};

// ============================================================================
// Extern "C" declarations (C API from omcruntime)
// ============================================================================

unsafe extern "C" {
    fn HpcOmBenchmarkExt_requiredTimeForComm() -> *mut std::ffi::c_void;
    fn HpcOmBenchmarkExt_requiredTimeForOp() -> *mut std::ffi::c_void;
    fn HpcOmBenchmarkExt_readCalcTimesFromXml(fileName: *const c_char) -> *mut std::ffi::c_void;
    fn HpcOmBenchmarkExt_readCalcTimesFromJson(fileName: *const c_char) -> *mut std::ffi::c_void;
}

// ============================================================================
// Rust wrapper functions (translated from MetaModelica)
// ============================================================================

/// External "C" call - HpcOmBenchmarkExt_requiredTimeForComm
/// Returns the list of required communication times.
/// Deprecated: external C dependency on omcruntime.
pub fn required_time_for_comm() -> *mut std::ffi::c_void {
    unsafe { HpcOmBenchmarkExt_requiredTimeForComm() }
}

/// External "C" call - HpcOmBenchmarkExt_requiredTimeForOp
/// Returns the list of required operation times.
/// Deprecated: external C dependency on omcruntime.
pub fn required_time_for_op() -> *mut std::ffi::c_void {
    unsafe { HpcOmBenchmarkExt_requiredTimeForOp() }
}

/// External "C" call - HpcOmBenchmarkExt_readCalcTimesFromXml
/// Reads calculation times from an XML file.
/// `fileName`: path to the XML file.
/// Deprecated: external C dependency on omcruntime.
pub fn read_calc_times_from_xml(file_name: &str) -> *mut std::ffi::c_void {
    let c_name = CString::new(file_name).expect("fileName contained NUL bytes");
    unsafe { HpcOmBenchmarkExt_readCalcTimesFromXml(c_name.as_ptr()) }
}

/// External "C" call - HpcOmBenchmarkExt_readCalcTimesFromJson
/// Reads calculation times from a JSON file.
/// `fileName`: path to the JSON file.
/// Deprecated: external C dependency on omcruntime.
pub fn read_calc_times_from_json(file_name: &str) -> *mut std::ffi::c_void {
    let c_name = CString::new(file_name).expect("fileName contained NUL bytes");
    unsafe { HpcOmBenchmarkExt_readCalcTimesFromJson(c_name.as_ptr()) }
}

/// Cast a raw C list pointer to an `im::Vector<f64>`.
/// This relies on the C library returning pointers compatible with im::Vector's
/// internal representation.
pub(crate) fn cast_list_real(ptr: *mut std::ffi::c_void) -> im::Vector<f64> {
    // SAFETY: This assumes the C library's list<Real> is ABI-compatible with
    // im::Vector<f64>. The actual C representation may differ; this is a
    // best-effort translation that may need adjustment based on the real C API.
    if ptr.is_null() {
        return im::vector![];
    }
    unsafe { std::ptr::read(ptr as *const im::Vector<f64>) }
}

/// Cast a raw C list pointer to an `im::Vector<i32>`.
pub(crate) fn cast_list_int(ptr: *mut std::ffi::c_void) -> im::Vector<i32> {
    if ptr.is_null() {
        return im::vector![];
    }
    unsafe { std::ptr::read(ptr as *const im::Vector<i32>) }
}

// Helper returning a typed list for requiredTimeForComm
/// Returns the list of required communication times.
/// Deprecated: external C dependency on omcruntime.
pub fn list_required_time_for_comm() -> im::Vector<i32> {
    cast_list_int(required_time_for_comm())
}

// Helper returning a typed list for requiredTimeForOp
/// Returns the list of required operation times.
/// Deprecated: external C dependency on omcruntime.
pub fn list_required_time_for_op() -> im::Vector<i32> {
    cast_list_int(required_time_for_op())
}

// Helper returning a typed list for readCalcTimesFromXml
/// Reads calculation times from an XML file as a list of Real values.
/// Deprecated: external C dependency on omcruntime.
pub fn list_read_calc_times_from_xml(file_name: &str) -> im::Vector<f64> {
    cast_list_real(read_calc_times_from_xml(file_name))
}

// Helper returning a typed list for readCalcTimesFromJson
/// Reads calculation times from a JSON file as a list of Real values.
/// Deprecated: external C dependency on omcruntime.
pub fn list_read_calc_times_from_json(file_name: &str) -> im::Vector<f64> {
    cast_list_real(read_calc_times_from_json(file_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_required_time_for_comm_signature() {
        let _ = || -> *mut std::ffi::c_void { required_time_for_comm() };
    }

    #[test]
    fn test_required_time_for_op_signature() {
        let _ = || -> *mut std::ffi::c_void { required_time_for_op() };
    }

    #[test]
    fn test_read_calc_times_from_xml_signature() {
        let _ = || -> *mut std::ffi::c_void { read_calc_times_from_xml("test.xml") };
    }

    #[test]
    fn test_read_calc_times_from_json_signature() {
        let _ = || -> *mut std::ffi::c_void { read_calc_times_from_json("test.json") };
    }

    #[test]
    fn test_cast_list_int_null() {
        let result = cast_list_int(std::ptr::null_mut());
        assert!(result.is_empty());
    }

    #[test]
    fn test_cast_list_real_null() {
        let result = cast_list_real(std::ptr::null_mut());
        assert!(result.is_empty());
    }
}
