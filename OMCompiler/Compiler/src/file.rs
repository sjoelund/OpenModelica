//! Translation of Util/File.mo
//!
//! This module provides file I/O wrappers around the OpenModelica C runtime
//! functions for opening, reading, writing, seeking, and managing file handles.
//! It wraps the `omc_file_ext.h` C interface exposed via the `omcruntime` library.

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};

// ============================================================================
// File type - wraps an opaque C file handle (ExternalObject)
// ============================================================================

/// Opaque file handle, wrapping the OpenModelica file object.
/// Corresponds to the `File` class extending `ExternalObject` in MetaModelica.
#[derive(Debug, Clone, PartialEq)]
pub struct File {
    pub handle: *mut c_void,
}

impl File {
    /// Creates a new File handle (default constructor with null handle).
    pub fn new() -> Self {
        File {
            handle: std::ptr::null_mut(),
        }
    }
}

impl Default for File {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Mode enumeration
// ============================================================================

/// Mode in which a file is opened.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Mode {
    /// Read mode.
    Read,
    /// Write mode.
    Write,
}

impl Mode {
    /// Converts a Mode to the corresponding c_int (0=Read, 1=Write).
    pub fn to_c_int(&self) -> c_int {
        match self {
            Mode::Read => 0,
            Mode::Write => 1,
        }
    }
}

// ============================================================================
// Escape enumeration
// ============================================================================

/// Escape mode for string escaping during file writes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Escape {
    /// No escaping.
    None,
    /// Escapes C strings minimally: \\n and ".
    C,
    /// Escapes JSON strings (quotes and control characters).
    JSON,
    /// Escapes strings for XML text.
    XML,
}

impl Escape {
    /// Converts an Escape to the corresponding c_int (0=None, 1=C, 2=JSON, 3=XML).
    pub fn to_c_int(&self) -> c_int {
        match self {
            Escape::None => 0,
            Escape::C => 1,
            Escape::JSON => 2,
            Escape::XML => 3,
        }
    }
}

// ============================================================================
// Whence enumeration
// ============================================================================

/// Position for file seek operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Whence {
    /// SEEK_SET: Seek from the start of the file.
    Set,
    /// SEEK_CUR: Seek from the current position.
    Current,
    /// SEEK_END: Seek from the end of the file.
    End,
}

impl Whence {
    /// Converts a Whence to the corresponding c_int (0=Set, 1=Current, 2=End).
    pub fn to_c_int(&self) -> c_int {
        match self {
            Whence::Set => 0,
            Whence::Current => 1,
            Whence::End => 2,
        }
    }
}

// ============================================================================
// FFI bindings to omcruntime C library
// ============================================================================

// Creates a new File handle from an optional reference.
// Corresponds to `external "C" file=om_file_new(fromID)`.
// `from_id` should be None (no reference) or the reference from a previous File.
unsafe extern "C" {
    pub fn om_file_new(from_id: *const c_void) -> *mut c_void;
}

// Frees a File handle.
// Corresponds to `external "C" om_file_free(file)`.
unsafe extern "C" {
    pub fn om_file_free(file: *mut c_void);
}

// Opens a file with the given mode.
// Corresponds to `external "C" om_file_open(file, filename, mode)`.
unsafe extern "C" {
    pub fn om_file_open(file: *mut c_void, filename: *const c_char, mode: c_int);
}

// Writes a string to a file.
// Corresponds to `external "C" om_file_write(file, data)`.
unsafe extern "C" {
    pub fn om_file_write(file: *mut c_void, data: *const c_char);
}

// Writes an integer to a file with a format string.
// Corresponds to `external "C" om_file_write_int(file, data, format)`.
unsafe extern "C" {
    pub fn om_file_write_int(file: *mut c_void, data: i32, format: *const c_char);
}

// Writes a Real (float) to a file with a format string.
// Corresponds to `external "C" om_file_write_real(file, data, format)`.
unsafe extern "C" {
    pub fn om_file_write_real(file: *mut c_void, data: f64, format: *const c_char);
}

// Writes an escaped string to a file.
// Corresponds to `external "C" om_file_write_escape(file, data, escape)`.
unsafe extern "C" {
    pub fn om_file_write_escape(file: *mut c_void, data: *const c_char, escape: c_int);
}

// Seeks to a position in the file.
// Returns true on success, false on failure.
// Corresponds to `external "C" success = om_file_seek(file, offset, whence)`.
unsafe extern "C" {
    pub fn om_file_seek(file: *mut c_void, offset: i32, whence: c_int) -> c_int;
}

// Returns the current position in the file.
// Corresponds to `external "C" pos = om_file_tell(file)`.
unsafe extern "C" {
    pub fn om_file_tell(file: *mut c_void) -> i32;
}

// Gets the filename associated with a File handle.
// Corresponds to `external "C" fileName2 = om_file_get_filename(file)`.
unsafe extern "C" {
    pub fn om_file_get_filename(file: *const c_void) -> *const c_char;
}

// Returns NULL (an opaque pointer, not actually Option<Integer>).
// Corresponds to `external "C" reference = om_file_no_reference()`.
unsafe extern "C" {
    pub fn om_file_no_reference() -> *const c_void;
}

// Returns an opaque reference pointer from a File handle.
// Corresponds to `external "C" reference = om_file_get_reference(file)`.
unsafe extern "C" {
    pub fn om_file_get_reference(file: *const c_void) -> *const c_void;
}

// Releases a reference to a File handle.
// Corresponds to `external "C" om_file_release_reference(file)`.
unsafe extern "C" {
    pub fn om_file_release_reference(file: *mut c_void);
}

// ============================================================================
// Safe wrapper functions
// ============================================================================

/// Opens a file in the given mode.
///
/// # Parameters
/// * `file` - The file handle to populate
/// * `filename` - The filename to open
/// * `mode` - The mode (Read or Write)
///
/// # Panics
/// Panics if `filename` contains an embedded null byte.
pub fn open(file: &mut File, filename: &str, mode: Mode) {
    let c_filename = CString::new(filename).expect("filename contains null byte");
    unsafe {
        om_file_open(file.handle, c_filename.as_ptr(), mode.to_c_int());
    }
}

/// Writes a string to a file.
///
/// # Parameters
/// * `file` - The file handle to write to
/// * `data` - The string data to write
///
/// # Panics
/// Panics if `data` contains an embedded null byte.
pub fn write(file: &File, data: &str) {
    let c_data = CString::new(data).expect("data contains null byte");
    unsafe {
        om_file_write(file.handle, c_data.as_ptr());
    }
}

/// Writes an integer to a file with the default format "%d".
///
/// # Parameters
/// * `file` - The file handle to write to
/// * `data` - The integer to write
///
/// # Panics
/// Panics if `format` contains an embedded null byte.
pub fn write_int(file: &File, data: i32) {
    write_int_with_format(file, data, "%d")
}

/// Writes an integer to a file with a custom format string.
///
/// # Parameters
/// * `file` - The file handle to write to
/// * `data` - The integer to write
/// * `format` - The format string (e.g., "%d", "%08d")
///
/// # Panics
/// Panics if `format` contains an embedded null byte.
pub fn write_int_with_format(file: &File, data: i32, format: &str) {
    let c_format = CString::new(format).expect("format contains null byte");
    unsafe {
        om_file_write_int(file.handle, data, c_format.as_ptr());
    }
}

/// Writes a Real (float) to a file with the default format "%.15g".
///
/// # Parameters
/// * `file` - The file handle to write to
/// * `data` - The real number to write
///
/// # Panics
/// Panics if `format` contains an embedded null byte.
pub fn write_real(file: &File, data: f64) {
    write_real_with_format(file, data, "%.15g")
}

/// Writes a Real (float) to a file with a custom format string.
///
/// # Parameters
/// * `file` - The file handle to write to
/// * `data` - The real number to write
/// * `format` - The format string (e.g., "%.15g", "%.2f")
///
/// # Panics
/// Panics if `format` contains an embedded null byte.
pub fn write_real_with_format(file: &File, data: f64, format: &str) {
    let c_format = CString::new(format).expect("format contains null byte");
    unsafe {
        om_file_write_real(file.handle, data, c_format.as_ptr());
    }
}

/// Writes an escaped string to a file.
///
/// # Parameters
/// * `file` - The file handle to write to
/// * `data` - The string data to write
/// * `escape` - The escape mode (None, C, JSON, XML)
///
/// # Panics
/// Panics if `data` contains an embedded null byte.
pub fn write_escape(file: &File, data: &str, escape: Escape) {
    let c_data = CString::new(data).expect("data contains null byte");
    unsafe {
        om_file_write_escape(file.handle, c_data.as_ptr(), escape.to_c_int());
    }
}

/// Seeks to a position in the file.
///
/// # Parameters
/// * `file` - The file handle
/// * `offset` - The byte offset
/// * `whence` - The reference point (Set, Current, End)
///
/// # Returns
/// `true` if the seek was successful, `false` otherwise.
pub fn seek(file: &File, offset: i32, whence: Whence) -> bool {
    let result = unsafe { om_file_seek(file.handle, offset, whence.to_c_int()) };
    result != 0
}

/// Returns the current file position.
///
/// # Parameters
/// * `file` - The file handle
///
/// # Returns
/// The current byte position in the file.
pub fn tell(file: &File) -> i32 {
    unsafe { om_file_tell(file.handle) }
}

/// Gets the filename associated with a File handle.
///
/// # Parameters
/// * `file` - The file handle
///
/// # Returns
/// The filename as a String, or an empty string if unavailable.
pub fn get_filename(file: &File) -> String {
    let ptr = unsafe { om_file_get_filename(file.handle) };
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

/// Returns NULL (an opaque pointer, not actually Option<Integer>).
/// Corresponds to `noReference()` in MetaModelica.
///
/// # Returns
/// A null pointer (raw pointer to c_void).
pub fn no_reference() -> *const c_void {
    unsafe { om_file_no_reference() }
}

/// Returns an opaque reference pointer from a File handle.
/// Corresponds to `getReference(file)` in MetaModelica.
///
/// # Parameters
/// * `file` - The file handle
///
/// # Returns
/// The reference pointer, or null if the file is invalid.
pub fn get_reference(file: &File) -> *const c_void {
    unsafe { om_file_get_reference(file.handle) }
}

/// Releases a reference to a File handle.
/// Corresponds to `releaseReference(file)` in MetaModelica.
///
/// # Parameters
/// * `file` - The file handle
pub fn release_reference(file: &mut File) {
    unsafe {
        om_file_release_reference(file.handle);
    }
}

/// Frees a File handle.
///
/// # Parameters
/// * `file` - The file handle to free
pub fn destructor(file: &mut File) {
    unsafe {
        om_file_free(file.handle);
    }
    file.handle = std::ptr::null_mut();
}

/// Writes `n` space characters to a file.
/// This is a pure MetaModelica function (not external).
/// Corresponds to the `writeSpace` function in File.mo.
///
/// # Parameters
/// * `file` - The file handle to write to
/// * `n` - The number of space characters to write
pub fn write_space(file: &File, n: i32) {
    if n > 0 {
        let spaces = " ".repeat(n as usize);
        write(file, &spaces);
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_to_c_int() {
        assert_eq!(Mode::Read.to_c_int(), 0);
        assert_eq!(Mode::Write.to_c_int(), 1);
    }

    #[test]
    fn test_escape_to_c_int() {
        assert_eq!(Escape::None.to_c_int(), 0);
        assert_eq!(Escape::C.to_c_int(), 1);
        assert_eq!(Escape::JSON.to_c_int(), 2);
        assert_eq!(Escape::XML.to_c_int(), 3);
    }

    #[test]
    fn test_whence_to_c_int() {
        assert_eq!(Whence::Set.to_c_int(), 0);
        assert_eq!(Whence::Current.to_c_int(), 1);
        assert_eq!(Whence::End.to_c_int(), 2);
    }

    #[test]
    fn test_file_default() {
        let file = File::default();
        assert!(file.handle.is_null());
    }

    #[test]
    fn test_write_space_no_op() {
        // write_space with n <= 0 should be a no-op (no error)
        let file = File::new();
        write_space(&file, 0);
        write_space(&file, -1);
    }
}
