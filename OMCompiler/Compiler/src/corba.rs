//! Translation of Util/Corba.mo
//!
//! This module provides a CORBA communication interface for the OpenModelica
//! compiler. It wraps external C functions from the `omcruntime` /
//! `OpenModelicaCorba` libraries.
//!
//! OpenModelica does not include a complete CORBA implementation. You need
//! to download one (e.g. MICO from http://www.mico.org, or omniORB via
//! `--with-omniORB=/location/of/corba/library`).
//!
//! The actual implementation differs between Windows and Unix versions,
//! with C ifdefs providing platform-specific code.

// ============================================================================
// Extern "C" declarations (C API from omcruntime / OpenModelicaCorba)
// ============================================================================

unsafe extern "C" {
    fn Corba_haveCorba() -> i32;
    fn Corba_setObjectReferenceFilePath(in_string: *const std::ffi::c_char);
    fn Corba_setSessionName(in_string: *const std::ffi::c_char);
    fn Corba_initialize();
    fn Corba_waitForCommand() -> *mut std::ffi::c_char;
    fn Corba_sendreply(in_string: *const std::ffi::c_char);
    fn Corba_close();
}

// ============================================================================
// Rust wrapper functions (translated from MetaModelica)
// ============================================================================

/// External "C" call - Corba_haveCorba
/// Returns whether a CORBA implementation is available.
/// Deprecated: external C dependency; requires omcruntime linkage.
pub fn have_corba() -> bool {
    unsafe { Corba_haveCorba() != 0 }
}

/// External "C" call - Corba_setObjectReferenceFilePath
/// Sets the file path for the CORBA object reference.
/// Deprecated: external C dependency.
pub fn set_object_reference_file_path(in_string: &str) {
    let c_str = std::ffi::CString::new(in_string).expect("NUL byte in path");
    unsafe { Corba_setObjectReferenceFilePath(c_str.as_ptr()) }
}

/// External "C" call - Corba_setSessionName
/// Sets the CORBA session name.
/// Deprecated: external C dependency.
pub fn set_session_name(in_string: &str) {
    let c_str = std::ffi::CString::new(in_string).expect("NUL byte in session name");
    unsafe { Corba_setSessionName(c_str.as_ptr()) }
}

/// External "C" call - Corba_initialize
/// Initializes the CORBA connection.
/// Deprecated: external C dependency.
pub fn initialize() {
    unsafe { Corba_initialize() }
}

/// External "C" call - Corba_waitForCommand
/// Waits for a command from the CORBA server and returns it.
/// Deprecated: external C dependency.
/// Returns an owned heap allocation from C that must be freed.
/// The caller is responsible for freeing the returned string using
/// a corresponding C free function (not provided here).
/// Consider using `Option<String>` or a wrapper that handles freeing.
pub fn wait_for_command() -> String {
    let ptr = unsafe { Corba_waitForCommand() };
    if ptr.is_null() {
        return String::new();
    }
    let c_str = unsafe { std::ffi::CStr::from_ptr(ptr) };
    let rust_str = c_str.to_string_lossy().into_owned();
    // Note: the C side is responsible for freeing this allocation.
    // Without a matching free function exposed in FFI, this is a potential leak.
    rust_str
}

/// External "C" call - Corba_sendreply
/// Sends a reply string back to the CORBA client.
/// Deprecated: external C dependency.
pub fn sendreply(in_string: &str) {
    let c_str = std::ffi::CString::new(in_string).expect("NUL byte in reply string");
    unsafe { Corba_sendreply(c_str.as_ptr()) }
}

/// External "C" call - Corba_close
/// Closes the CORBA connection.
/// Deprecated: external C dependency.
pub fn close() {
    unsafe { Corba_close() }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    // NOTE: Most tests are compile-only checks that verify function
    // signatures without calling the external C functions.
    // Actual FFI calls require the omcruntime/OpenModelicaCorba libraries.

    #[test]
    fn test_have_corba_signature() {
        // Verifies the function signature: fn() -> bool
        let _fn_ptr: fn() -> bool = have_corba;
        let _closure: Box<dyn Fn() -> bool> = Box::new(|| have_corba());
    }

    #[test]
    fn test_set_object_reference_file_path_signature() {
        let _fn_ptr: fn(&str) = set_object_reference_file_path;
    }

    #[test]
    fn test_set_session_name_signature() {
        let _fn_ptr: fn(&str) = set_session_name;
    }

    #[test]
    fn test_initialize_signature() {
        let _fn_ptr: fn() = initialize;
    }

    #[test]
    fn test_wait_for_command_signature() {
        let _fn_ptr: fn() -> String = wait_for_command;
    }

    #[test]
    fn test_sendreply_signature() {
        let _fn_ptr: fn(&str) = sendreply;
    }

    #[test]
    fn test_close_signature() {
        let _fn_ptr: fn() = close;
    }
}
