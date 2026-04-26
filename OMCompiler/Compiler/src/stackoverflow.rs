//! Translation of Util/StackOverflow.mo
//!
//! This module provides functions for handling stack overflow detection and
//! readable stacktrace generation, translated from MetaModelica.
//!
//! # Assumptions and Notes
//!
//! 1. **External C functions**: Functions like `mmc_do_stackoverflow`,
//!    `mmc_getStacktraceMessages_threadData`, `mmc_setStacktraceMessages_threadData`,
//!    `mmc_hasStacktraceMessages`, and `mmc_clearStacktraceMessages` are bound to
//!    external C functions. They are currently stubbed since the full C runtime
//!    is not available.
//! 2. **Testsuite.isRunning()**: The `Testsuite` module is not yet translated.
//!    `is_running()` is stubbed to return `false` so that normal execution
//!    proceeds.
//! 3. **System.regex**: MetaModelica's `System.regex` returns the number of
//!    matched groups and a list of captured strings. The Rust equivalent uses
//!    the `regex` crate with named capture groups.
//! 4. **OpenModelica.threadData()**: Returns a pointer to the current thread's
//!    data structure. This is a C runtime concept. The Rust stubs accept a
//!    dummy parameter.
//! 5. **List<T>**: Uses `im::Vector<T>` for immutable list semantics.
//! 6. **1-based indexing**: MetaModelica uses 1-based indexing for `substring`,
//!    but the Rust `substring` helper here uses 0-based indexing for compatibility
//!    with Rust byte slicing.
//! 7. **String(Integer(x))**: Converting `Integer` to `String` via `format!("{}", x)`.
//! 8. **listReverse**: Implemented as `list_reverse`.
//! 9. **matchcontinue**: Not used in this module.

use im::Vector;
use regex::RegexBuilder;

// ============================================================================
// Utility functions
// ============================================================================

/// Returns substring from `start` to `end` (0-based, inclusive).
/// Mirrors the Modelica `substring(str, start, end)` function.
fn substring(s: &str, start: usize, end: usize) -> String {
    let bytes = s.as_bytes();
    if start > end || end >= bytes.len() {
        return String::new();
    }
    String::from_utf8_lossy(&bytes[start..=end]).to_string()
}

/// Reverses an immutable vector (equivalent to listReverse).
fn list_reverse<T: Clone>(v: Vector<T>) -> Vector<T> {
    let mut result = Vector::new();
    for item in v.iter().rev() {
        result.push_back(item.clone());
    }
    result
}

/// Replaces all occurrences of `from` with `to` in `s`.
/// Equivalent to MetaModelica's `System.stringReplace(str, from, to)`.
fn string_replace(s: &str, from: &str, to: &str) -> String {
    s.replace(from, to)
}

// ============================================================================
// Testsuite stub
// ============================================================================

/// Stub for `Testsuite.isRunning()`.
/// Returns `false` since the Testsuite module is not yet translated.
fn is_running() -> bool {
    false
}

// ============================================================================
// External C function bindings
// ============================================================================

// Safety: These are FFI boundaries into the C runtime.
#[cfg(not(test))]
unsafe extern "C" {
    fn mmc_getStacktraceMessages_threadData(_thread_data: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void;
    fn mmc_setStacktraceMessages_threadData(
        _thread_data: *mut std::os::raw::c_void,
        _num_skip: i32,
        _num_frames: i32,
    );
    fn mmc_hasStacktraceMessages(_thread_data: *mut std::os::raw::c_void) -> i32;
    fn mmc_clearStacktraceMessages(_thread_data: *mut std::os::raw::c_void);
    fn mmc_do_stackoverflow(_thread_data: *mut std::os::raw::c_void);
}

#[cfg(test)]
mod ffi {
    use super::*;

    /// Test-time stubs for the external C functions.
    /// These are only used during `cargo test` when the C runtime is not available.
    pub unsafe fn mmc_getStacktraceMessages_threadData(_thread_data: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void {
        std::ptr::null_mut()
    }

    pub unsafe fn mmc_setStacktraceMessages_threadData(
        _thread_data: *mut std::os::raw::c_void,
        _num_skip: i32,
        _num_frames: i32,
    ) {
        // no-op
    }

    pub unsafe fn mmc_hasStacktraceMessages(_thread_data: *mut std::os::raw::c_void) -> i32 {
        0
    }

    pub unsafe fn mmc_clearStacktraceMessages(_thread_data: *mut std::os::raw::c_void) {
        // no-op
    }

    pub unsafe fn mmc_do_stackoverflow(_thread_data: *mut std::os::raw::c_void) {
        // no-op
    }
}

/// Returns the thread data pointer (stub).
fn thread_data() -> *mut std::os::raw::c_void {
    std::ptr::null_mut()
}

// ============================================================================
// unmangle
// ============================================================================

/// Converts a mangled C symbol name back to a human-readable form.
///
/// If the symbol starts with `"omc_"`, it:
/// 1. Strips the `"omc_"` prefix (characters 0-3, keeping from index 4 onwards)
/// 2. Replaces `"__"` with `"#"`
/// 3. Replaces `"_"` with `"."`
/// 4. Replaces `"#"` with `"_"`
///
/// Otherwise, returns the symbol unchanged.
pub fn unmangle(in_symbol: &str) -> String {
    let mut out_symbol = in_symbol.to_string();
    if in_symbol.starts_with("omc_") {
        // substring(outSymbol, 5, stringLength(outSymbol)) - MM uses 1-based indexing
        // so position 5 = 0-based index 4
        out_symbol = substring(&out_symbol, 4, out_symbol.chars().count() - 1);

        out_symbol = string_replace(&out_symbol, "__", "#");
        out_symbol = string_replace(&out_symbol, "_", ".");
        out_symbol = string_replace(&out_symbol, "#", "_");
    }
    out_symbol
}

// ============================================================================
// stripAddresses
// ============================================================================

/// Strips memory addresses from a stack trace symbol string, replacing them
/// with the unmangled function name.
///
/// Handles two formats:
/// - **Linux**: `symbol(func+offset) [0xaddress]`
/// - **macOS**: `0xaddress func [+- offset]`
///
/// Returns the formatted string with unmangled function names.
pub fn strip_addresses(in_symbol: &str) -> String {
    // Regex for Linux messages:
    // "^([^(]*)[(]([^+]*[^+]*)[+][^)]*[)] *[[]0x[0-9a-fA-F]*[]]$"
    // In Rust regex, [[] means literal '[' but Rust uses \[ instead.
    // Group 1: symbol name before '('
    // Group 2: function name before '+'
    let linux_regex = RegexBuilder::new("^([^(]*)[(]([^+]*[^+]*)[+][^)]*[)] *\\[0x[0-9a-fA-F]*\\]$")
        .unicode(true)
        .build()
        .unwrap();

    if let Some(caps) = linux_regex.captures(in_symbol) {
        if caps.len() == 3 {
            let so = caps.get(1).map(|m| m.as_str()).unwrap_or("");
            let fun = caps.get(2).map(|m| m.as_str()).unwrap_or("");
            let unmangled = unmangle(fun);
            return format!("{}({})", so, unmangled);
        }
    }

    // Regex for OSX messages:
    // "^[0-9 ]*([A-Za-z0-9.]*) *0x[0-9a-fA-F]* ([A-Za-z0-9_]*) *[+] *[0-9]*$"
    // Group 1: library/object name
    // Group 2: function name
    let osx_regex = RegexBuilder::new("^[0-9 ]*([A-Za-z0-9.]*) *0x[0-9a-fA-F]* ([A-Za-z0-9_]*) *[+] *[0-9]*$")
        .unicode(true)
        .build()
        .unwrap();

    if let Some(caps) = osx_regex.captures(in_symbol) {
        if caps.len() == 3 {
            let so = caps.get(1).map(|m| m.as_str()).unwrap_or("");
            let fun = caps.get(2).map(|m| m.as_str()).unwrap_or("");
            let unmangled = unmangle(fun);
            return format!("{}({})", so, unmangled);
        }
    }

    in_symbol.to_string()
}

// ============================================================================
// triggerStackOverflow
// ============================================================================

/// Fakes a stack overflow (useful for debugging; forces earlier exit
/// since most functions do not catch stack overflow, and gives you a
/// stacktrace of the position you triggered this from).
pub fn trigger_stack_overflow() {
    // Safety: FFI call into the C runtime.
    #[cfg(not(test))]
    unsafe {
        mmc_do_stackoverflow(thread_data());
    }
    #[cfg(test)]
    unsafe {
        ffi::mmc_do_stackoverflow(thread_data());
    }
}

// ============================================================================
// generateReadableMessage
// ============================================================================

/// Generates a readable message from the current stack trace.
///
/// # Parameters
/// * `num_frames` - Maximum number of frames to include (default: 1000)
/// * `num_skip` - Number of frames to skip from the top (default: 4)
/// * `delimiter` - Delimiter between messages (default: "\n")
///
/// # Returns
/// A formatted string containing the readable stacktrace messages.
pub fn generate_readable_message(
    num_frames: i32,
    num_skip: i32,
    delimiter: &str,
) -> String {
    set_stacktrace_messages(num_skip, num_frames);
    get_readable_message(delimiter)
}

// ============================================================================
// getReadableMessage
// ============================================================================

/// Returns a single readable string from the stacktrace messages.
///
/// # Parameters
/// * `delimiter` - Delimiter between messages (default: "\n")
///
/// # Returns
/// A formatted string containing the readable stacktrace messages.
pub fn get_readable_message(delimiter: &str) -> String {
    string_delimit_list(readable_stacktrace_messages(), delimiter)
}

// ============================================================================
// readableStacktraceMessages
// ============================================================================

/// Produces a list of human-readable stacktrace messages.
/// Compresses consecutive duplicate symbols into ranges like
/// `[bt] #12...15 func_name`.
///
/// # Returns
/// A list of formatted stacktrace symbol strings, in stack-bottom-to-top order.
pub fn readable_stacktrace_messages() -> Vector<String> {
    if is_running() {
        let mut symbols = Vector::new();
        symbols.push_back(
            "[bt] [Symbols are not generated when running the test suite]".to_string()
        );
        return symbols;
    }

    let mut symbols = Vector::new();
    let mut prev: String = String::new();
    let mut n: i32 = 1;
    let mut prev_n: i32 = 1;

    // Get stacktrace messages and strip addresses
    let messages = get_stacktrace_messages();
    let stripped: Vector<String> = messages.iter()
        .map(|s| strip_addresses(&s))
        .collect();

    for symbol in stripped.iter() {
        if prev.is_empty() {
            // First symbol - do nothing (empty branch in MM)
        } else if *symbol != prev {
            let range_str = if n != prev_n {
                format!("...{}", n)
            } else {
                String::new()
            };
            let entry = format!("[bt] #{}{} {}", prev_n, range_str, prev);
            symbols.push_front(entry);
            n += 1;
            prev_n = n;
        } else {
            n += 1;
        }
        prev = symbol.clone();
    }

    // Add the last entry (only if we processed at least one symbol)
    if !prev.is_empty() {
        let range_str = if n != prev_n {
            format!("...{}", n)
        } else {
            String::new()
        };
        let entry = format!("[bt] #{}{} {}", prev_n, range_str, prev);
        symbols.push_front(entry);
    }

    list_reverse(symbols)
}

// ============================================================================
// readableStacktraceMessages (convenience wrapper with Vector)
// ============================================================================

/// Wrapper that returns the readable stacktrace as a `Vector<String>`.
/// Calls the internal implementation.
fn readable_stacktrace_messages_vec() -> Vector<String> {
    readable_stacktrace_messages()
}

// ============================================================================
// Helper: stringDelimitList
// ============================================================================

/// Joins a list of strings with the given delimiter.
/// Equivalent to MetaModelica's `stringDelimitList(list, delimiter)`.
fn string_delimit_list(list: Vector<String>, delimiter: &str) -> String {
    let items: Vec<&str> = list.iter().map(|s| s.as_str()).collect();
    items.join(delimiter)
}

// ============================================================================
// getStacktraceMessages
// ============================================================================

/// Returns a list of symbol names to print in error messages.
/// Calls into the C runtime to retrieve captured stack trace messages.
///
/// # Returns
/// A list of string symbols from the captured stack trace.
pub fn get_stacktrace_messages() -> Vector<String> {
    // In the real implementation, this would call mmc_getStacktraceMessages_threadData
    // and convert the C list to a Rust Vector. For now, return an empty list as a stub.
    #[cfg(not(test))]
    unsafe {
        let _raw = mmc_getStacktraceMessages_threadData(thread_data());
        // Convert C list to Rust Vector here.
        Vector::new()
    }
    #[cfg(test)]
    {
        unsafe {
            let _raw = ffi::mmc_getStacktraceMessages_threadData(thread_data());
            // Convert C list to Rust Vector here.
            Vector::new()
        }
    }
}

// ============================================================================
// setStacktraceMessages
// ============================================================================

/// Generate a stacktrace at the current position of code.
///
/// # Parameters
/// * `num_skip` - Number of frames to skip
/// * `num_frames` - Maximum number of frames to capture
pub fn set_stacktrace_messages(num_skip: i32, num_frames: i32) {
    // Safety: FFI call into the C runtime.
    #[cfg(not(test))]
    unsafe {
        mmc_setStacktraceMessages_threadData(thread_data(), num_skip, num_frames);
    }
    #[cfg(test)]
    unsafe {
        ffi::mmc_setStacktraceMessages_threadData(thread_data(), num_skip, num_frames);
    }
}

// ============================================================================
// hasStacktraceMessages
// ============================================================================

/// Returns true if a stack overflow has occurred.
///
/// # Returns
/// `true` if stacktrace messages are available, `false` otherwise.
pub fn has_stacktrace_messages() -> bool {
    // Safety: FFI call into the C runtime.
    #[cfg(not(test))]
    let result = unsafe { mmc_hasStacktraceMessages(thread_data()) };
    #[cfg(test)]
    let result = unsafe { ffi::mmc_hasStacktraceMessages(thread_data()) };
    result != 0
}

// ============================================================================
// clearStacktraceMessages
// ============================================================================

/// Clears the stacktrace from a stack overflow.
pub fn clear_stacktrace_messages() {
    // Safety: FFI call into the C runtime.
    #[cfg(not(test))]
    unsafe {
        mmc_clearStacktraceMessages(thread_data());
    }
    #[cfg(test)]
    unsafe {
        ffi::mmc_clearStacktraceMessages(thread_data());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unmangle_simple() {
        assert_eq!(unmangle("someSymbol"), "someSymbol");
    }

    #[test]
    fn test_unmangle_with_prefix() {
        // "omc_TestFunc" -> strip "omc_" -> "TestFunc"
        // No underscores or double underscores, so just return as-is
        assert_eq!(unmangle("omc_TestFunc"), "TestFunc");
    }

    #[test]
    fn test_unmangle_with_underscores() {
        // "omc_test__func" -> strip "omc_" -> "test__func"
        // Replace "__" with "#" -> "test#func"
        // Replace "_" with "." -> "test#func" (no more _)
        // Replace "#" with "_" -> "test_func"
        assert_eq!(unmangle("omc_test__func"), "test_func");
    }

    #[test]
    fn test_unmangle_with_underscores_and_dots() {
        // "omc_test__func_name" -> "test__func_name" -> "test#func_name" -> "test#func.name" -> "test_func.name"
        assert_eq!(unmangle("omc_test__func_name"), "test_func.name");
    }

    #[test]
    fn test_string_replace() {
        assert_eq!(string_replace("hello_world", "_", "."), "hello.world");
        assert_eq!(string_replace("hello__world", "__", "#"), "hello#world");
    }

    #[test]
    fn test_strip_addresses_linux() {
        let input = "someLib(func+0x10) [0x7f]";
        let result = strip_addresses(input);
        assert_eq!(result, "someLib(func)");
    }

    #[test]
    fn test_strip_addresses_linux_with_unmangle() {
        let input = "omcTestLib(omc_test__func+0x10) [0x7f]";
        let result = strip_addresses(input);
        // unmangle("omc_test__func") -> "test_func"
        assert_eq!(result, "omcTestLib(test_func)");
    }

    #[test]
    fn test_strip_addresses_osx() {
        let input = "  libsystem 0x1234 main + 0";
        let result = strip_addresses(input);
        assert_eq!(result, "libsystem(main)");
    }

    #[test]
    fn test_strip_addresses_no_match() {
        let input = "garbage data here";
        let result = strip_addresses(input);
        assert_eq!(result, "garbage data here");
    }

    #[test]
    fn test_list_reverse() {
        let mut v = Vector::new();
        v.push_back("a".to_string());
        v.push_back("b".to_string());
        v.push_back("c".to_string());
        let r = list_reverse(v);
        let items: Vec<&String> = r.iter().collect();
        assert_eq!(items[0], &"c".to_string());
        assert_eq!(items[1], &"b".to_string());
        assert_eq!(items[2], &"a".to_string());
    }

    #[test]
    fn test_string_delimit_list() {
        let mut v = Vector::new();
        v.push_back("a".to_string());
        v.push_back("b".to_string());
        v.push_back("c".to_string());
        assert_eq!(string_delimit_list(v, ", "), "a, b, c");
    }

    #[test]
    fn test_substring() {
        assert_eq!(substring("hello", 0, 4), "hello");
        assert_eq!(substring("hello", 1, 3), "ell");
    }

    #[test]
    fn test_get_readable_message_calls_stubbed_functions() {
        // Verify the function compiles and can be called.
        // The result will be empty since get_stacktrace_messages is a stub.
        let result = get_readable_message("\n");
        assert_eq!(result, "");
    }
}
