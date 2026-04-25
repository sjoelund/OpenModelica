//! Translation of Util/Gettext.mo
//!
//! This module provides utilities for marking strings as translatable or
//! non-translatable, and for translating translatable content to strings.
//!
//! The `TranslatableContent` union type wraps either a `gettext` record
//! (marking a message ID for translation via `System.gettext`) or a `notrans`
//! record (a plain string that should not be translated).

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};

// ============================================================================
// Thread data accessor
// ============================================================================

#[allow(dead_code)]
unsafe extern "C" {
    fn OpenModelica_threadData() -> *mut c_void;
}

/// Returns the current thread data pointer.
#[allow(dead_code)]
fn thread_data() -> *mut c_void {
    unsafe { OpenModelica_threadData() }
}

// ============================================================================
// FFI binding to System.gettext
// ============================================================================

// Translates a message ID using the C gettext system.
// Safety: extern "C" block used for FFI to OpenModelica runtime.
// The System_gettext function is declared here so it can be called safely.
unsafe extern "C" {
    #[allow(dead_code)]
    fn System_gettext(threadData: *mut c_void, msgid: *const c_char) -> *const c_char;
}

/// Translates a message ID string using the underlying gettext system.
///
/// # Parameters
/// * `msgid` - The untranslated message ID
///
/// # Returns
/// The translated string, or the original `msgid` if no translation is available.
///
/// # Panics
/// Panics if `msgid` contains an embedded null byte.
///
/// # Safety
/// This function calls into C code via FFI.
#[allow(dead_code)]
pub fn gettext(msgid: &str) -> String {
    let c_str = CString::new(msgid).expect("msgid contains null byte");
    let td = thread_data();
    let ptr = unsafe { System_gettext(td, c_str.as_ptr()) };
    if ptr.is_null() {
        msgid.to_string()
    } else {
        unsafe {
            CStr::from_ptr(ptr)
                .to_string_lossy()
                .into_owned()
        }
    }
}

// ============================================================================
// TranslatableContent - union type for translatable or non-translatable strings
// ============================================================================

/// Represents content that is either a translatable message or a non-translatable string.
///
/// - `Gettext { msgid }`: A message ID to be translated.
/// - `NoTrans { str }`: A plain string that should not be translated (e.g., too generic).
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TranslatableContent {
    /// Used to mark messages as targets for translation
    Gettext { msgid: String },
    /// String cannot be translated; used for too generic messages
    NoTrans { str: String },
}

// ============================================================================
// translateContent
// ============================================================================

/// Translates `TranslatableContent` to a string.
/// For `Gettext` variants, the message ID is passed through `gettext()`.
/// For `NoTrans` variants, the string is returned as-is.
///
/// # Parameters
/// * `msg` - The translatable content to convert to a string
///
/// # Returns
/// The translated (or passthrough) string.
#[allow(dead_code)]
pub fn translate_content(msg: TranslatableContent) -> String {
    match msg {
        TranslatableContent::Gettext { msgid } => gettext(&msgid),
        TranslatableContent::NoTrans { str } => str,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_trans_returns_string_as_is() {
        let content = TranslatableContent::NoTrans {
            str: "hello world".to_string(),
        };
        assert_eq!(translate_content(content), "hello world");
    }

    #[test]
    fn test_gettext_wrapper_exists() {
        // Just verify the FFI wrapper compiles and can be called.
        // Actual translation depends on the C runtime and locale setup.
        let _result = gettext("test message");
    }
}
