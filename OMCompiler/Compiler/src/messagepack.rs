//! Translation of Util/MessagePack.mo
//!
//! This module provides MessagePack serialization and deserialization utilities.
//! MessagePack is an efficient binary serialization format.
//!
//! # Design notes
//!
//! All classes that `extend ExternalObject` in the original MetaModelica code
//! are opaque pointers managed by the C `msgpackc` library (via the
//! `msgpack_modelica` C API shim). In Rust we represent them as `*mut c_void`
//! and wrap them in zero-sized newtype structs that implement `Drop` for
//! automatic cleanup.
//!
//! The external C functions declared here (`msgpack_modelica_*`) must be
//! provided by the `msgpackc` library at link time.
//!
//! # Assumptions / Things that may not work as expected
//!
//! - The C library `msgpackc` and the `msgpack-modelica.h` header must be
//!   available at link time.  This crate does not bundle them.
//! - `SimpleBuffer` and `Packer` are RAII wrappers — they auto-free on Drop.
//! - `Deserializer` and `Stream` are also RAII wrappers with Drop.
//! - `String` in the original maps to `*const c_char` (null-terminated UTF-8).
//! - `Boolean` maps to `c_int` (0 = false, non-zero = true).
//! - `Integer` in the original is a 64-bit value; we use `i64` for precision.

use std::ffi::{c_char, c_int, c_void};

// ============================================================================
// Opaque handles (ExternalObject wrappers)
// ============================================================================

/// Opaque handle for `Pack.SimpleBuffer.SimpleBuffer`.
/// Wraps an sbuffer_t* from the C API.
pub struct SimpleBuffer {
    ptr: *mut c_void,
}

impl SimpleBuffer {
    pub fn new() -> Self {
        let ptr = unsafe { msgpack_modelica_sbuffer_new() };
        Self { ptr }
    }
}

impl Drop for SimpleBuffer {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { msgpack_modelica_sbuffer_free(self.ptr) };
        }
    }
}

impl Default for SimpleBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// Opaque handle for `Pack.Packer`.
/// Wraps a msgpack_packer* from the C API.
pub struct Packer {
    ptr: *mut c_void,
}

impl Packer {
    pub fn new(buf: &SimpleBuffer) -> Self {
        let ptr = unsafe { msgpack_modelica_packer_new_sbuffer(buf.ptr) };
        Self { ptr }
    }
}

impl Drop for Packer {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { msgpack_modelica_packer_free(self.ptr) };
        }
    }
}

/// Opaque handle for `Unpack.Deserializer`.
/// Wraps a msgpack_unpacked* from the C API.
pub struct Deserializer {
    ptr: *mut c_void,
}

impl Deserializer {
    /// Load a MessagePack file from disk.
    /// Mirrors `Unpack.Deserializer` constructor.
    pub fn from_file(path: &str) -> Self {
        let c_str = std::ffi::CString::new(path).expect("path contains null bytes");
        let ptr = unsafe { msgpack_modelica_new_deserialiser(c_str.as_ptr()) };
        Self { ptr }
    }
}

impl Drop for Deserializer {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { msgpack_modelica_free_deserialiser(self.ptr) };
        }
    }
}

/// Opaque handle for `Utilities.Stream.Stream`.
/// Wraps a msgpack_modelica_stream* from the C API.
pub struct Stream {
    ptr: *mut c_void,
}

impl Stream {
    /// Create a new stream.
    /// If `path` is empty, creates an in-memory stream (use `to_string()` to read).
    /// Mirrors `Utilities.Stream.Stream` constructor.
    pub fn new(path: &str) -> Self {
        let c_str = std::ffi::CString::new(path).expect("path contains null bytes");
        let ptr = unsafe { msgpack_modelica_new_stream(c_str.as_ptr()) };
        Self { ptr }
    }
}

impl Drop for Stream {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { msgpack_modelica_free_stream(self.ptr) };
        }
    }
}

// ============================================================================
// Extern "C" declarations (msgpack_modelica C API)
// ============================================================================

unsafe extern "C" {
    // --- SimpleBuffer ---
    fn msgpack_modelica_sbuffer_new() -> *mut c_void;
    fn msgpack_modelica_sbuffer_free(buf: *mut c_void);
    fn msgpack_modelica_sbuffer_to_file(buf: *mut c_void, file: *const c_char) -> c_int;
    fn msgpack_modelica_sbuffer_position(buf: *mut c_void) -> i64;

    // --- Packer ---
    fn msgpack_modelica_packer_new_sbuffer(buf: *mut c_void) -> *mut c_void;
    fn msgpack_modelica_packer_free(packer: *mut c_void);
    fn msgpack_modelica_pack_double(packer: *mut c_void, dbl: f64) -> c_int;
    fn msgpack_modelica_pack_int(packer: *mut c_void, i: i64) -> c_int;
    fn msgpack_modelica_pack_true(packer: *mut c_void) -> c_int;
    fn msgpack_modelica_pack_false(packer: *mut c_void) -> c_int;
    fn msgpack_modelica_pack_array(packer: *mut c_void, len: i64) -> c_int;
    fn msgpack_modelica_pack_map(packer: *mut c_void, len: i64) -> c_int;
    fn msgpack_modelica_pack_string(packer: *mut c_void, s: *const c_char) -> c_int;
    fn msgpack_modelica_pack_nil(packer: *mut c_void) -> c_int;

    // --- Deserializer / Unpack ---
    fn msgpack_modelica_new_deserialiser(file: *const c_char) -> *mut c_void;
    fn msgpack_modelica_free_deserialiser(deserializer: *mut c_void);
    fn msgpack_modelica_unpack_next(
        deserializer: *mut c_void,
        offset: i64,
        newoffset: *mut i64,
    ) -> c_int;
    fn msgpack_modelica_unpack_next_to_stream(
        deserializer: *mut c_void,
        stream: *mut c_void,
        offset: i64,
        newoffset: *mut i64,
    ) -> c_int;
    fn msgpack_modelica_unpack_int(
        deserializer: *mut c_void,
        offset: i64,
        newoffset: *mut i64,
        success: *mut c_int,
    ) -> i64;
    fn msgpack_modelica_unpack_string(
        deserializer: *mut c_void,
        offset: i64,
        newoffset: *mut i64,
        success: *mut c_int,
    ) -> *mut c_char;
    fn msgpack_modelica_get_unpacked_int(deserializer: *mut c_void) -> i64;

    // --- Stream ---
    fn msgpack_modelica_new_stream(file: *const c_char) -> *mut c_void;
    fn msgpack_modelica_free_stream(stream: *mut c_void);
    fn msgpack_modelica_stream_get(stream: *mut c_void) -> *mut c_char;
    fn msgpack_modelica_stream_append(stream: *mut c_void, str: *const c_char);
}

// ============================================================================
// Pack.SimpleBuffer — wrapper functions
// ============================================================================

/// Write the buffer contents to a file.
/// Mirrors `Pack.SimpleBuffer.writeFile`.
pub fn simple_buffer_write_file(buf: &SimpleBuffer, path: &str) -> bool {
    let c_str = std::ffi::CString::new(path).expect("path contains null bytes");
    unsafe { msgpack_modelica_sbuffer_to_file(buf.ptr, c_str.as_ptr()) != 0 }
}

/// Get the current position (length) of the buffer.
/// Mirrors `Pack.SimpleBuffer.position`.
pub fn simple_buffer_position(buf: &SimpleBuffer) -> i64 {
    unsafe { msgpack_modelica_sbuffer_position(buf.ptr) }
}

// ============================================================================
// Pack — packer functions
// ============================================================================

/// Pack a double (Real) value.
/// Returns true on success.
/// Mirrors `Pack.double`.
pub fn pack_double(packer: &Packer, value: f64) -> bool {
    unsafe { msgpack_modelica_pack_double(packer.ptr, value) != 0 }
}

/// Pack an integer value.
/// Returns true on success.
/// Mirrors `Pack.integer`.
pub fn pack_integer(packer: &Packer, value: i64) -> bool {
    unsafe { msgpack_modelica_pack_int(packer.ptr, value) != 0 }
}

/// Pack a boolean value.
/// Returns true on success.
/// Mirrors `Pack.bool`.
pub fn pack_bool(packer: &Packer, value: bool) -> bool {
    let result = if value {
        unsafe { msgpack_modelica_pack_true(packer.ptr) }
    } else {
        unsafe { msgpack_modelica_pack_false(packer.ptr) }
    };
    result != 0
}

/// Pack the start of an array (sequence) with the given length.
/// Returns true on success.
/// Mirrors `Pack.sequence`.
pub fn pack_sequence(packer: &Packer, len: i64) -> bool {
    unsafe { msgpack_modelica_pack_array(packer.ptr, len) != 0 }
}

/// Pack the start of a map with the given length.
/// Returns true on success.
/// Mirrors `Pack.map`.
pub fn pack_map(packer: &Packer, len: i64) -> bool {
    unsafe { msgpack_modelica_pack_map(packer.ptr, len) != 0 }
}

/// Pack a string value.
/// Returns true on success.
/// Mirrors `Pack.string`.
pub fn pack_string(packer: &Packer, value: &str) -> bool {
    let c_str = std::ffi::CString::new(value).expect("string contains null bytes");
    unsafe { msgpack_modelica_pack_string(packer.ptr, c_str.as_ptr()) != 0 }
}

/// Pack a nil (null) value.
/// Returns true on success.
/// Mirrors `Pack.nil`.
pub fn pack_nil(packer: &Packer) -> bool {
    unsafe { msgpack_modelica_pack_nil(packer.ptr) != 0 }
}

// ============================================================================
// Unpack — deserializer functions
// ============================================================================

/// Try to advance to the next unpacked item.
/// Returns (success, new_offset).
/// Mirrors `Unpack.next`.
pub fn unpack_next(deserializer: &Deserializer, offset: i64) -> (bool, i64) {
    let mut new_offset = 0i64;
    let success = unsafe {
        msgpack_modelica_unpack_next(deserializer.ptr, offset, &mut new_offset) != 0
    };
    (success, new_offset)
}

/// Unpack an integer value at the current offset.
/// Returns (value, new_offset, success).
/// Mirrors `Unpack.integer`.
pub fn unpack_integer(deserializer: &Deserializer, offset: i64) -> (i64, i64, bool) {
    let mut new_offset = 0i64;
    let mut success = 0;
    let value = unsafe {
        msgpack_modelica_unpack_int(
            deserializer.ptr,
            offset,
            &mut new_offset,
            &mut success,
        )
    };
    (value, new_offset, success != 0)
}

/// Unpack a string value at the current offset.
/// Returns (value, new_offset, success).
/// The returned string may be None if success is false or the pointer is null.
/// Mirrors `Unpack.string`.
pub fn unpack_string(deserializer: &Deserializer, offset: i64) -> (Option<String>, i64, bool) {
    let mut new_offset = 0i64;
    let mut success = 0;
    let ptr = unsafe {
        msgpack_modelica_unpack_string(
            deserializer.ptr,
            offset,
            &mut new_offset,
            &mut success,
        )
    };
    let value = if ptr.is_null() {
        None
    } else {
        Some(
            unsafe { std::ffi::CStr::from_ptr(ptr) }
                .to_string_lossy()
                .into_owned(),
        )
    };
    (value, new_offset, success != 0)
}

/// Get an integer value from the deserializer without advancing.
/// Mirrors `Unpack.get_integer`.
pub fn get_integer(deserializer: &Deserializer) -> i64 {
    unsafe { msgpack_modelica_get_unpacked_int(deserializer.ptr) }
}

/// Unpack the next item and write it to a stream.
/// Returns (success, new_offset).
/// Mirrors `Unpack.toStream`.
pub fn unpack_next_to_stream(
    deserializer: &Deserializer,
    stream: &Stream,
    offset: i64,
) -> (bool, i64) {
    let mut new_offset = 0i64;
    let success = unsafe {
        msgpack_modelica_unpack_next_to_stream(
            deserializer.ptr,
            stream.ptr,
            offset,
            &mut new_offset,
        ) != 0
    };
    (success, new_offset)
}

// ============================================================================
// Utilities.Stream — stream functions
// ============================================================================

/// Read the content of an in-memory stream as a String.
/// Only works for in-memory streams (created with empty path).
/// Mirrors `Utilities.Stream.get`.
pub fn stream_get(stream: &Stream) -> String {
    let ptr = unsafe { msgpack_modelica_stream_get(stream.ptr) };
    if ptr.is_null() {
        String::new()
    } else {
        unsafe { std::ffi::CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned()
    }
}

/// Append a string to a stream.
/// Mirrors `Utilities.Stream.append`.
pub fn stream_append(stream: &Stream, value: &str) {
    let c_str = std::ffi::CString::new(value).expect("string contains null bytes");
    unsafe { msgpack_modelica_stream_append(stream.ptr, c_str.as_ptr()) }
}

// ============================================================================
// Utilities — high-level function
// ============================================================================

/// Deserialize a MessagePack binary file into a text file.
/// Each MessagePack item is written as a separate line.
///
/// Mirrors `Utilities.deserializeFileToFile`.
/// `separator` defaults to "\n".
pub fn deserialize_file_to_file(in_path: &str, out_path: &str, separator: &str) {
    let deserializer = Deserializer::from_file(in_path);
    let stream = Stream::new(out_path);

    let offset = 0i64;
    let (success, new_offset) = unpack_next_to_stream(&deserializer, &stream, offset);
    if success {
        stream_append(&stream, separator);
        deserialize_file_to_file_inner(&deserializer, &stream, new_offset, separator);
    }
}

/// Inner recursive helper for `deserialize_file_to_file`.
fn deserialize_file_to_file_inner(
    deserializer: &Deserializer,
    stream: &Stream,
    offset: i64,
    separator: &str,
) {
    let (success, new_offset) = unpack_next_to_stream(deserializer, stream, offset);
    if success {
        stream_append(stream, separator);
        deserialize_file_to_file_inner(deserializer, stream, new_offset, separator);
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_buffer_new() {
        let buf = SimpleBuffer::new();
        assert!(simple_buffer_position(&buf) >= 0);
    }

    #[test]
    fn test_pack_integer() {
        let buf = SimpleBuffer::new();
        let packer = Packer::new(&buf);

        // Pack an array of 2 integers
        assert!(pack_sequence(&packer, 2));
        assert!(pack_integer(&packer, 42));
        assert!(pack_integer(&packer, 99));

        assert_eq!(simple_buffer_position(&buf), 5); // 1 (array header) + 2*2 (fixintegers)
    }

    #[test]
    fn test_pack_string() {
        let buf = SimpleBuffer::new();
        let packer = Packer::new(&buf);

        assert!(pack_string(&packer, "hello"));
        // "hello" = 5 bytes + 1 byte for fixstr header
        assert_eq!(simple_buffer_position(&buf), 6);
    }

    #[test]
    fn test_pack_bool() {
        let buf = SimpleBuffer::new();
        let packer = Packer::new(&buf);

        assert!(pack_bool(&packer, true));
        assert!(pack_bool(&packer, false));
        assert_eq!(simple_buffer_position(&buf), 2); // 1 byte per boolean
    }

    #[test]
    fn test_pack_nil() {
        let buf = SimpleBuffer::new();
        let packer = Packer::new(&buf);

        assert!(pack_nil(&packer));
        assert_eq!(simple_buffer_position(&buf), 1); // fixnil = 1 byte
    }

    #[test]
    fn test_pack_double() {
        let buf = SimpleBuffer::new();
        let packer = Packer::new(&buf);

        assert!(pack_double(&packer, 3.14159));
        assert_eq!(simple_buffer_position(&buf), 9); // fixfloat = 1 (fixfloat header) + 8 bytes
    }

    #[test]
    fn test_pack_map() {
        let buf = SimpleBuffer::new();
        let packer = Packer::new(&buf);

        assert!(pack_map(&packer, 1));
        assert!(pack_string(&packer, "key"));
        assert!(pack_integer(&packer, 123));
        assert_eq!(simple_buffer_position(&buf), 10);
    }

    #[test]
    fn test_pack_array() {
        let buf = SimpleBuffer::new();
        let packer = Packer::new(&buf);

        assert!(pack_sequence(&packer, 3));
        assert!(pack_integer(&packer, 1));
        assert!(pack_integer(&packer, 2));
        assert!(pack_integer(&packer, 3));
    }

    #[test]
    fn test_stream_new_and_append() {
        let stream = Stream::new(""); // in-memory stream
        stream_append(&stream, "hello");
        stream_append(&stream, " world");
        let content = stream_get(&stream);
        assert_eq!(content, "hello world");
    }
}
