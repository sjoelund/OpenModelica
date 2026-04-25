//! Translation of Util/Curl.mo
//!
//! This module provides multi-download functionality using libcurl.
//! It wraps the `om_curl_multi_download` C function from the OpenModelica runtime
//! for downloading multiple files with parallel transfers and mirror retry support.

use std::ffi::{c_int, c_void};

// ============================================================================
// FFI bindings to the OpenModelica runtime curl functions
// ============================================================================

// Multi-download function from the OpenModelica C runtime.
//
// url_path_list: linked list of download entries. Each entry is a tuple
//   (urls, filename) where urls is a list of mirror URLs to try and
//   filename is the output file path.
// max_parallel: maximum number of concurrent transfers.
//
// Returns 1 on success, 0 on failure.
//
// Corresponds to the `external "C"` declaration in Curl.mo:
// `external "C" success = om_curl_multi_download(urlFileList, maxParallel);`
#[allow(dead_code)]
unsafe extern "C" {
    pub fn om_curl_multi_download(url_path_list: *mut c_void, max_parallel: c_int) -> c_int;
}

// ============================================================================
// Safe wrapper
// ============================================================================

/// Download multiple files from URLs with parallel transfers and mirror retry.
///
/// Each download entry is a tuple of `(urls, filename)` where `urls` is a list
/// of mirror URLs to try (in order) and `filename` is the output file path.
/// On download failure, the next mirror URL is automatically tried.
///
/// # Parameters
/// * `url_path_list` - Download entries as a linked list pointer matching
///   the MetaModelica `list<tuple<list<String>, String>>` layout
/// * `max_parallel` - Maximum number of concurrent transfers.
///   If 0, defaults to the number of available CPU cores.
///
/// # Returns
/// `true` if all downloads succeeded, `false` if any failed.
///
/// # Safety
/// The caller must ensure `url_path_list` points to valid linked list data
/// matching the MetaModelica MMC list representation (cons cells with
/// `MMC_CAR`/`MMC_CDR` structure). Each entry tuple contains
/// `(list<String> urls, String filename)`.
#[allow(dead_code)]
pub fn multi_download(url_path_list: *mut c_void, max_parallel: i32) -> bool {
    let max_p = if max_parallel > 0 { max_parallel } else { 1 };
    unsafe { om_curl_multi_download(url_path_list, max_p) != 0 }
}
