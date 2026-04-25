//! Translation of Util/Print.mo
//!
//! This module provides buffered printing functions that wrap the
//! OpenModelica `omcruntime` C library. It includes functions for saving
//! and restoring print buffers, writing to error buffers, and managing
//! buffer content.
//!
//! All external C functions link against the `omcruntime` library.
//! Each function takes a thread data pointer via `thread_data()`,
//! which corresponds to `OpenModelica.threadData()` in MetaModelica.

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};

// ============================================================================
// Thread data accessor
// ============================================================================

// Safety: extern "C" block used for FFI to OpenModelica runtime
#[allow(dead_code)]
unsafe extern "C" {
    fn OpenModelica_threadData() -> *mut c_void;
}

/// Returns the current thread data pointer.
fn thread_data() -> *mut c_void {
    unsafe { OpenModelica_threadData() }
}

// ============================================================================
// FFI bindings to omcruntime C library
// ============================================================================

// saveAndClearBuf - saves and clears the buffer, returns a handle
unsafe extern "C" {
    fn Print_saveAndClearBuf(threadData: *mut c_void) -> c_int;
}

// restoreBuf - restores a previously saved buffer by handle
unsafe extern "C" {
    fn Print_restoreBuf(threadData: *mut c_void, handle: c_int);
}

// printErrorBuf - prints a string to the error buffer
unsafe extern "C" {
    fn Print_printErrorBuf(threadData: *mut c_void, inString: *const c_char);
}

// clearErrorBuf - clears the error buffer
unsafe extern "C" {
    fn Print_clearErrorBuf(threadData: *mut c_void);
}

// getErrorString - returns the error buffer content as a string
unsafe extern "C" {
    fn Print_getErrorString(threadData: *mut c_void) -> *const c_char;
}

// printBufLen - prints a string to the print buffer with explicit length
unsafe extern "C" {
    fn Print_printBufLen(threadData: *mut c_void, inString: *const c_char, length: c_int);
}

// clearBuf - clears the print buffer
unsafe extern "C" {
    fn Print_clearBuf(threadData: *mut c_void);
}

// getString - returns the print buffer content as a string (without clearing)
unsafe extern "C" {
    fn Print_getString(threadData: *mut c_void) -> *const c_char;
}

// writeBuf - writes the buffer content to a file
unsafe extern "C" {
    fn Print_writeBuf(threadData: *mut c_void, filename: *const c_char);
}

// writeBufConvertLines - writes buffer with modelica line directives converted to C preprocessor #line macros
unsafe extern "C" {
    fn Print_writeBufConvertLines(threadData: *mut c_void, filename: *const c_char);
}

// getBufLength - returns the filled length of the print buffer
unsafe extern "C" {
    fn Print_getBufLength(threadData: *mut c_void) -> c_int;
}

// printBufSpace - prints the given number of spaces to the print buffer
unsafe extern "C" {
    fn Print_printBufSpace(threadData: *mut c_void, inNumOfSpaces: c_int);
}

// printBufNewLine - prints one newline character to the print buffer
unsafe extern "C" {
    fn Print_printBufNewLine(threadData: *mut c_void);
}

// hasBufNewLineAtEnd - tests if the last character in the buffer is a newline
unsafe extern "C" {
    fn Print_hasBufNewLineAtEnd(threadData: *mut c_void) -> c_int;
}

// ============================================================================
// Safe wrapper functions
// ============================================================================

/// Saves and clears the content of the print buffer.
/// Returns a handle that can be used to restore the buffer later via `restore_buf`.
///
/// # Returns
/// An integer handle to the saved buffer state.
///
/// # Safety
/// This function calls into C code via FFI.
pub fn save_and_clear_buf() -> i32 {
    let td = thread_data();
    unsafe { Print_saveAndClearBuf(td) }
}

/// Restores a previously saved buffer.
/// Restores the buffer to the state saved by `save_and_clear_buf`.
///
/// # Parameters
/// * `handle` - The handle returned by `save_and_clear_buf`
///
/// # Safety
/// This function calls into C code via FFI.
pub fn restore_buf(handle: i32) {
    let td = thread_data();
    unsafe { Print_restoreBuf(td, handle) }
}

/// Prints a string to the error buffer.
///
/// # Parameters
/// * `in_string` - The string to print to the error buffer
///
/// # Panics
/// Panics if `in_string` contains an embedded null byte.
///
/// # Safety
/// This function calls into C code via FFI.
pub fn print_error_buf(in_string: &str) {
    let c_str = CString::new(in_string).expect("string contains null byte");
    let td = thread_data();
    unsafe { Print_printErrorBuf(td, c_str.as_ptr()) }
}

/// Clears the error buffer.
///
/// # Safety
/// This function calls into C code via FFI.
pub fn clear_error_buf() {
    let td = thread_data();
    unsafe { Print_clearErrorBuf(td) }
}

/// Returns the error buffer content as a string.
///
/// # Returns
/// The error buffer content, or an empty string if unavailable.
///
/// # Safety
/// This function calls into C code via FFI.
pub fn get_error_string() -> String {
    let td = thread_data();
    let ptr = unsafe { Print_getErrorString(td) };
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

/// Prints a string to the print buffer.
/// Corresponds to `Print_printBufLen` with explicit string length.
///
/// # Parameters
/// * `in_string` - The string to print to the buffer
///
/// # Panics
/// Panics if `in_string` contains an embedded null byte.
///
/// # Safety
/// This function calls into C code via FFI.
pub fn print_buf(in_string: &str) {
    let c_str = CString::new(in_string).expect("string contains null byte");
    let td = thread_data();
    let length = in_string.len() as c_int;
    unsafe { Print_printBufLen(td, c_str.as_ptr(), length) }
}

/// Clears the print buffer.
///
/// # Safety
/// This function calls into C code via FFI.
pub fn clear_buf() {
    let td = thread_data();
    unsafe { Print_clearBuf(td) }
}

/// Returns the print buffer content as a string.
/// Does NOT clear the buffer.
///
/// # Returns
/// The print buffer content, or an empty string if unavailable.
///
/// # Safety
/// This function calls into C code via FFI.
pub fn get_string() -> String {
    let td = thread_data();
    let ptr = unsafe { Print_getString(td) };
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

/// Writes the buffer content to a file.
///
/// # Parameters
/// * `filename` - The filename to write to
///
/// # Panics
/// Panics if `filename` contains an embedded null byte.
///
/// # Safety
/// This function calls into C code via FFI.
pub fn write_buf(filename: &str) {
    let c_filename = CString::new(filename).expect("filename contains null byte");
    let td = thread_data();
    unsafe { Print_writeBuf(td, c_filename.as_ptr()) }
}

/// Writes the buffer to a file, converting /*#modelicaLine...*/ directives
/// to C preprocessor #line macros.
///
/// # Parameters
/// * `filename` - The filename to write to
///
/// # Panics
/// Panics if `filename` contains an embedded null byte.
///
/// # Safety
/// This function calls into C code via FFI.
pub fn write_buf_convert_lines(filename: &str) {
    let c_filename = CString::new(filename).expect("filename contains null byte");
    let td = thread_data();
    unsafe { Print_writeBufConvertLines(td, c_filename.as_ptr()) }
}

/// Gets the actual length of the filled space in the print buffer.
///
/// # Returns
/// The number of characters currently in the buffer.
///
/// # Safety
/// This function calls into C code via FFI.
pub fn get_buf_length() -> i32 {
    let td = thread_data();
    unsafe { Print_getBufLength(td) }
}

/// Prints the given number of space characters to the print buffer.
///
/// # Parameters
/// * `num_of_spaces` - The number of spaces to print
///
/// # Safety
/// This function calls into C code via FFI.
pub fn print_buf_space(num_of_spaces: i32) {
    let td = thread_data();
    unsafe { Print_printBufSpace(td, num_of_spaces) }
}

/// Prints one newline character to the print buffer.
///
/// # Safety
/// This function calls into C code via FFI.
pub fn print_buf_new_line() {
    let td = thread_data();
    unsafe { Print_printBufNewLine(td) }
}

/// Tests if the last outputted character in the print buffer is a newline.
/// This is a (temporary) workaround to string_length's O(n) cost.
///
/// # Returns
/// `true` if the last character is a newline, `false` otherwise.
///
/// # Safety
/// This function calls into C code via FFI.
pub fn has_buf_new_line_at_end() -> bool {
    let td = thread_data();
    unsafe { Print_hasBufNewLineAtEnd(td) != 0 }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread_data_returns_pointer() {
        // The thread data pointer should not be null in a properly initialized runtime.
        // In a test environment without the full runtime, this may be null -
        // that's acceptable as long as the function returns a value.
        let _ptr = thread_data();
    }

    #[test]
    fn test_clear_error_buf_exists() {
        // Verify the function compiles and can be called.
        // Actual behavior depends on the C runtime being initialized.
        clear_error_buf();
    }
}
