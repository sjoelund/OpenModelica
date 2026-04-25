//! Translation of Util/Socket.mo
//!
//! This module provides socket communication interface for the OpenModelica
//! compiler. It wraps external C functions from the `omcruntime` library.
//!
//! This is the socket connection module of the compiler, used in interactive
//! mode when omc is started with `-d=interactive`. The actual implementation
//! is in `./runtime/socketimpl.c`. Not implemented in Win32 builds; use
//! `-d=interactiveCorba` instead.

// ============================================================================
// Extern "C" declarations (C API from omcruntime)
// ============================================================================

unsafe extern "C" {
    fn Socket_waitforconnect(inInteger: i32) -> i32;
    fn Socket_handlerequest(inInteger: i32) -> *mut std::ffi::c_char;
    fn Socket_sendreply(inInteger: i32, inString: *const std::ffi::c_char);
    fn Socket_close(inInteger: i32);
    fn Socket_cleanup();
}

// ============================================================================
// Rust wrapper functions (translated from MetaModelica)
// ============================================================================

/// External "C" call - Socket_waitforconnect
/// Waits for a connection on the given file descriptor.
/// Deprecated: external C dependency; requires omcruntime linkage.
pub fn wait_for_connect(in_integer: i32) -> i32 {
    unsafe { Socket_waitforconnect(in_integer) }
}

/// External "C" call - Socket_handlerequest
/// Handles an incoming request on the given connection descriptor
/// and returns the request string.
/// Deprecated: external C dependency; requires omcruntime linkage.
/// Returns an owned heap allocation from C that must be freed.
/// The caller is responsible for freeing the returned string.
pub fn handle_request(in_integer: i32) -> String {
    let ptr = unsafe { Socket_handlerequest(in_integer) };
    if ptr.is_null() {
        return String::new();
    }
    let c_str = unsafe { std::ffi::CStr::from_ptr(ptr) };
    c_str.to_string_lossy().into_owned()
}

/// External "C" call - Socket_sendreply
/// Sends a reply string back over the given connection descriptor.
/// Deprecated: external C dependency; requires omcruntime linkage.
pub fn send_reply(in_integer: i32, in_string: &str) {
    let c_str = std::ffi::CString::new(in_string).expect("NUL byte in reply string");
    unsafe { Socket_sendreply(in_integer, c_str.as_ptr()) }
}

/// External "C" call - Socket_close
/// Closes the given connection descriptor.
/// Deprecated: external C dependency; requires omcruntime linkage.
pub fn close(in_integer: i32) {
    unsafe { Socket_close(in_integer) }
}

/// External "C" call - Socket_cleanup
/// Cleans up all socket resources.
/// Deprecated: external C dependency; requires omcruntime linkage.
pub fn cleanup() {
    unsafe { Socket_cleanup() }
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
    fn test_wait_for_connect_signature() {
        let _fn_ptr: fn(i32) -> i32 = wait_for_connect;
        let _closure: Box<dyn Fn(i32) -> i32> = Box::new(|x| wait_for_connect(x));
    }

    #[test]
    fn test_handle_request_signature() {
        let _fn_ptr: fn(i32) -> String = handle_request;
    }

    #[test]
    fn test_send_reply_signature() {
        let _fn_ptr: fn(i32, &str) = send_reply;
    }

    #[test]
    fn test_close_signature() {
        let _fn_ptr: fn(i32) = close;
    }

    #[test]
    fn test_cleanup_signature() {
        let _fn_ptr: fn() = cleanup;
    }
}
