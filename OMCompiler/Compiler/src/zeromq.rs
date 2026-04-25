//! Translation of Util/ZeroMQ.mo
//!
//! This module provides ZeroMQ communication interface for the OpenModelica
//! compiler. It wraps external C functions from the `omcruntime` library.
//!
//! This is the ZeroMQ connection module of the compiler, used in
//! interactive mode if omc is started with `-d=interactiveZMQ`.
//! The actual implementation is in `./runtime/zeromqimpl.c`.

// ============================================================================
// Extern "C" declarations (C API from omcruntime)
// ============================================================================

unsafe extern "C" {
    fn ZeroMQ_initialize(
        fileSuffix: *const std::ffi::c_char,
        listenToAll: std::ffi::c_int,
        port: i32,
    ) -> i32;
    fn ZeroMQ_handleRequest(zmqSocket: *mut std::ffi::c_void)
        -> *mut std::ffi::c_char;
    fn ZeroMQ_sendReply(zmqSocket: *mut std::ffi::c_void, reply: *const std::ffi::c_char);
    fn ZeroMQ_close(zmqSocket: *mut std::ffi::c_void);
}

// ============================================================================
// Rust wrapper functions (translated from MetaModelica)
// ============================================================================

/// External "C" call - ZeroMQ_initialize
/// Initializes a ZeroMQ socket connection.
/// Deprecated: external C dependency; requires omcruntime linkage.
pub fn initialize(
    file_suffix: &str,
    listen_to_all: bool,
    port: i32,
) -> Option<i32> {
    let c_suffix =
        std::ffi::CString::new(file_suffix).expect("NUL byte in fileSuffix");
    let listen = if listen_to_all { 1 } else { 0 };
    let result = unsafe { ZeroMQ_initialize(c_suffix.as_ptr(), listen, port) };
    if result < 0 {
        None
    } else {
        Some(result)
    }
}

/// External "C" call - ZeroMQ_handleRequest
/// Handles an incoming request on the given ZeroMQ socket and returns the
/// request string.
/// Deprecated: external C dependency; requires omcruntime linkage.
/// Returns an owned heap allocation from C that must be freed.
/// The caller is responsible for freeing the returned string.
pub fn handle_request(zmq_socket: Option<i32>) -> String {
    let ptr = zmq_socket
        .map(|fd| unsafe { ZeroMQ_handleRequest(fd as *mut std::ffi::c_void) });
    if let Some(ptr) = ptr {
        if ptr.is_null() {
            return String::new();
        }
        let c_str = unsafe { std::ffi::CStr::from_ptr(ptr) };
        c_str.to_string_lossy().into_owned()
    } else {
        String::new()
    }
}

/// External "C" call - ZeroMQ_sendReply
/// Sends a reply string back over the given ZeroMQ socket.
/// Deprecated: external C dependency; requires omcruntime linkage.
pub fn send_reply(zmq_socket: Option<i32>, reply: &str) {
    if let Some(fd) = zmq_socket {
        let c_str = std::ffi::CString::new(reply).expect("NUL byte in reply string");
        unsafe {
            ZeroMQ_sendReply(fd as *mut std::ffi::c_void, c_str.as_ptr());
        }
    }
}

/// External "C" call - ZeroMQ_close
/// Closes the given ZeroMQ socket.
/// Deprecated: external C dependency; requires omcruntime linkage.
pub fn close(zmq_socket: Option<i32>) {
    if let Some(fd) = zmq_socket {
        unsafe { ZeroMQ_close(fd as *mut std::ffi::c_void) }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    // NOTE: Most tests are compile-only checks that verify function
    // signatures without calling the external C functions.
    // Actual FFI calls require the omcruntime libraries.

    #[test]
    fn test_initialize_signature() {
        let _fn_ptr: fn(&str, bool, i32) -> Option<i32> = initialize;
    }

    #[test]
    fn test_handle_request_signature() {
        let _fn_ptr: fn(Option<i32>) -> String = handle_request;
    }

    #[test]
    fn test_send_reply_signature() {
        let _fn_ptr: fn(Option<i32>, &str) = send_reply;
    }

    #[test]
    fn test_close_signature() {
        let _fn_ptr: fn(Option<i32>) = close;
    }
}
