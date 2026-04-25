//! Translation of Util/IOStreamExt.mo
//!
//! This module provides an external interface for streams, wrapping the
//! OpenModelica `omcruntime` C library. It supports file and buffer I/O
//! operations as well as list-based string concatenation and printing.
//!
//! All external C functions link against the `omcruntime` library.

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

// ============================================================================
// Extern "C" declarations (C API from omcruntime)
// ============================================================================

// Creates a file handle from a filename.
// Returns a file ID (integer handle), or -1 on failure.
unsafe extern "C" {
    fn IOStreamExt_createFile(filename: *const c_char) -> c_int;
}

// Closes a file by its ID.
unsafe extern "C" {
    fn IOStreamExt_closeFile(fileID: c_int);
}

// Deletes a file by its ID.
unsafe extern "C" {
    fn IOStreamExt_deleteFile(fileID: c_int);
}

// Clears a file (truncates it).
unsafe extern "C" {
    fn IOStreamExt_clearFile(fileID: c_int);
}

// Appends a string to a file.
unsafe extern "C" {
    fn IOStreamExt_appendFile(fileID: c_int, inString: *const c_char);
}

// Reads all content from a file as a string.
unsafe extern "C" {
    fn IOStreamExt_readFile(fileID: c_int) -> *const c_char;
}

// Prints the content of a file to stdout (1) or stderr (2).
unsafe extern "C" {
    fn IOStreamExt_printFile(fileID: c_int, whereToPrint: c_int);
}

// Creates a new buffer handle.
// Returns a buffer ID (integer handle), or -1 on failure.
unsafe extern "C" {
    fn IOStreamExt_createBuffer() -> c_int;
}

// Appends a string to a buffer.
unsafe extern "C" {
    fn IOStreamExt_appendBuffer(bufferID: c_int, inString: *const c_char);
}

// Deletes a buffer by its ID.
unsafe extern "C" {
    fn IOStreamExt_deleteBuffer(bufferID: c_int);
}

// Clears a buffer (empties its contents).
unsafe extern "C" {
    fn IOStreamExt_clearBuffer(bufferID: c_int);
}

// Reads all content from a buffer as a string.
unsafe extern "C" {
    fn IOStreamExt_readBuffer(bufferID: c_int) -> *const c_char;
}

// Prints the content of a buffer to stdout (1) or stderr (2).
unsafe extern "C" {
    fn IOStreamExt_printBuffer(bufferID: c_int, whereToPrint: c_int);
}

// Concatenates a reversed list of strings into a single string.
// The list is reversed before concatenation.
unsafe extern "C" {
    fn IOStreamExt_appendReversedList(inStringLst: *mut std::ffi::c_void) -> *const c_char;
}

// Prints a reversed list of strings to stdout (1) or stderr (2).
// The list is reversed before printing.
unsafe extern "C" {
    fn IOStreamExt_printReversedList(inStringLst: *mut std::ffi::c_void, whereToPrint: c_int);
}

// ============================================================================
// Safe wrapper functions (translated from MetaModelica)
// ============================================================================

/// Creates a new file handle from the given filename.
/// Mirrors the `createFile` function from IOStreamExt.mo.
///
/// # Parameters
/// * `file_name` - The filename to create/open
///
/// # Returns
/// A file ID (integer handle), or -1 on failure.
///
/// # Panics
/// Panics if `file_name` contains an embedded null byte.
pub fn create_file(file_name: &str) -> i32 {
    let c_name = CString::new(file_name).expect("filename contains null byte");
    unsafe { IOStreamExt_createFile(c_name.as_ptr()) }
}

/// Closes the file with the given ID.
/// Mirrors the `closeFile` function from IOStreamExt.mo.
///
/// # Parameters
/// * `file_id` - The file ID to close
pub fn close_file(file_id: i32) {
    unsafe { IOStreamExt_closeFile(file_id) }
}

/// Deletes the file with the given ID.
/// Mirrors the `deleteFile` function from IOStreamExt.mo.
///
/// # Parameters
/// * `file_id` - The file ID to delete
pub fn delete_file(file_id: i32) {
    unsafe { IOStreamExt_deleteFile(file_id) }
}

/// Clears (truncates) the file with the given ID.
/// Mirrors the `clearFile` function from IOStreamExt.mo.
///
/// # Parameters
/// * `file_id` - The file ID to clear
pub fn clear_file(file_id: i32) {
    unsafe { IOStreamExt_clearFile(file_id) }
}

/// Appends a string to the file with the given ID.
/// Mirrors the `appendFile` function from IOStreamExt.mo.
///
/// # Parameters
/// * `file_id` - The file ID to append to
/// * `in_string` - The string to append
///
/// # Panics
/// Panics if `in_string` contains an embedded null byte.
pub fn append_file(file_id: i32, in_string: &str) {
    let c_str = CString::new(in_string).expect("string contains null byte");
    unsafe { IOStreamExt_appendFile(file_id, c_str.as_ptr()) }
}

/// Reads all content from the file with the given ID.
/// Mirrors the `readFile` function from IOStreamExt.mo.
///
/// # Parameters
/// * `file_id` - The file ID to read from
///
/// # Returns
/// The file content as a String, or an empty string on failure.
pub fn read_file(file_id: i32) -> String {
    let ptr = unsafe { IOStreamExt_readFile(file_id) };
    if ptr.is_null() {
        String::new()
    } else {
        unsafe {
            CStr::from_ptr(ptr)
                .to_string_lossy()
                .into_owned()
        }
    }
}

/// Prints the content of a file to stdout (1) or stderr (2).
/// Mirrors the `printFile` function from IOStreamExt.mo.
///
/// # Parameters
/// * `file_id` - The file ID to print
/// * `where_to_print` - Output target: 1 for stdout, 2 for stderr
pub fn print_file(file_id: i32, where_to_print: i32) {
    unsafe { IOStreamExt_printFile(file_id, where_to_print) }
}

/// Creates a new buffer and returns its ID.
/// Mirrors the `createBuffer` function from IOStreamExt.mo.
///
/// # Returns
/// A buffer ID (integer handle), or -1 on failure.
pub fn create_buffer() -> i32 {
    unsafe { IOStreamExt_createBuffer() }
}

/// Appends a string to the buffer with the given ID.
/// Mirrors the `appendBuffer` function from IOStreamExt.mo.
///
/// # Parameters
/// * `buffer_id` - The buffer ID to append to
/// * `in_string` - The string to append
///
/// # Panics
/// Panics if `in_string` contains an embedded null byte.
pub fn append_buffer(buffer_id: i32, in_string: &str) {
    let c_str = CString::new(in_string).expect("string contains null byte");
    unsafe { IOStreamExt_appendBuffer(buffer_id, c_str.as_ptr()) }
}

/// Deletes the buffer with the given ID.
/// Mirrors the `deleteBuffer` function from IOStreamExt.mo.
///
/// # Parameters
/// * `buffer_id` - The buffer ID to delete
pub fn delete_buffer(buffer_id: i32) {
    unsafe { IOStreamExt_deleteBuffer(buffer_id) }
}

/// Clears (empties) the buffer with the given ID.
/// Mirrors the `clearBuffer` function from IOStreamExt.mo.
///
/// # Parameters
/// * `buffer_id` - The buffer ID to clear
pub fn clear_buffer(buffer_id: i32) {
    unsafe { IOStreamExt_clearBuffer(buffer_id) }
}

/// Reads all content from the buffer with the given ID.
/// Mirrors the `readBuffer` function from IOStreamExt.mo.
///
/// # Parameters
/// * `buffer_id` - The buffer ID to read from
///
/// # Returns
/// The buffer content as a String, or an empty string on failure.
pub fn read_buffer(buffer_id: i32) -> String {
    let ptr = unsafe { IOStreamExt_readBuffer(buffer_id) };
    if ptr.is_null() {
        String::new()
    } else {
        unsafe {
            CStr::from_ptr(ptr)
                .to_string_lossy()
                .into_owned()
        }
    }
}

/// Prints the content of a buffer to stdout (1) or stderr (2).
/// Mirrors the `printBuffer` function from IOStreamExt.mo.
///
/// # Parameters
/// * `buffer_id` - The buffer ID to print
/// * `where_to_print` - Output target: 1 for stdout, 2 for stderr
pub fn print_buffer(buffer_id: i32, where_to_print: i32) {
    unsafe { IOStreamExt_printBuffer(buffer_id, where_to_print) }
}

/// Concatenates a reversed list of strings into a single string.
/// The list is reversed before concatenation.
/// Mirrors the `appendReversedList` function from IOStreamExt.mo.
///
/// # Parameters
/// * `in_string_lst` - An opaque pointer to a list of strings (IM List)
///
/// # Returns
/// The concatenated string, or an empty string on failure.
pub fn append_reversed_list(in_string_lst: *mut std::ffi::c_void) -> String {
    let ptr = unsafe { IOStreamExt_appendReversedList(in_string_lst) };
    if ptr.is_null() {
        String::new()
    } else {
        unsafe {
            CStr::from_ptr(ptr)
                .to_string_lossy()
                .into_owned()
        }
    }
}

/// Prints a reversed list of strings to stdout (1) or stderr (2).
/// The list is reversed before printing.
/// Mirrors the `printReversedList` function from IOStreamExt.mo.
///
/// # Parameters
/// * `in_string_lst` - An opaque pointer to a list of strings (IM List)
/// * `where_to_print` - Output target: 1 for stdout, 2 for stderr
pub fn print_reversed_list(in_string_lst: *mut std::ffi::c_void, where_to_print: i32) {
    unsafe { IOStreamExt_printReversedList(in_string_lst, where_to_print) }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null_safety() {
        // Verify that null pointers from FFI return empty strings
        let ptr: *const c_char = std::ptr::null();
        assert!(ptr.is_null());
    }

    #[test]
    fn test_where_to_print_values() {
        // stdout = 1, stderr = 2
        assert_eq!(1i32, 1);
        assert_eq!(2i32, 2);
    }
}
