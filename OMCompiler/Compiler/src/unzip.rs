//! Translation of Util/Unzip.mo
//!
//! This module provides a wrapper for extracting files from ZIP archives
//! via the OpenModelica C runtime function `om_unzip`.

use std::ffi::CString;
use std::os::raw::c_char;

// ============================================================================
// FFI bindings to omcruntime C library
// ============================================================================

// FFI binding for the C function that extracts files from a ZIP archive.
// Corresponds to `external "C" success = om_unzip(fileName, pathToExtract, destinationPath)`
// in the MetaModelica source.
unsafe extern "C" {
    pub fn om_unzip(
        zip_file_name: *const c_char,
        path_to_extract: *const c_char,
        dest_path: *const c_char,
    ) -> i32;
}

// ============================================================================
// Safe wrapper
// ============================================================================

/// Extracts files from a ZIP archive to a destination path.
///
/// This wraps the `om_unzip` C function from the OpenModelica runtime.
///
/// # Parameters
/// * `file_name` - Path to the ZIP file to extract from
/// * `path_to_extract` - Sub-path within the ZIP to extract (empty string extracts all)
/// * `destination_path` - Directory to extract files into
///
/// # Returns
/// `true` if extraction succeeded, `false` otherwise.
///
/// # Panics
/// Panics if any argument contains an embedded null byte.
///
/// # Safety
/// This function is safe to call, but the underlying C function may fail
/// if the ZIP file does not exist, is corrupted, or the destination path
/// is invalid.
pub fn unzip_path(file_name: &str, path_to_extract: &str, destination_path: &str) -> bool {
    let c_file_name =
        CString::new(file_name).expect("file_name contains null byte");
    let c_path_to_extract =
        CString::new(path_to_extract).expect("path_to_extract contains null byte");
    let c_destination_path =
        CString::new(destination_path).expect("destination_path contains null byte");

    unsafe {
        om_unzip(
            c_file_name.as_ptr(),
            c_path_to_extract.as_ptr(),
            c_destination_path.as_ptr(),
        ) != 0
    }
}
