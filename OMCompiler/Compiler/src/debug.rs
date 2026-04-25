//! Translation of Util/Debug.mo
//!
//! Debug printing functions. These provide simple trace output capabilities
//! used for debug printing during execution.
//!
//! Both functions wrap `print_error_buf` from the print module, which writes
//! to the OpenModelica error buffer via the `omcruntime` C library.

use crate::print::print_error_buf;

/// Debug printing function.
/// Writes the given string to the error buffer.
///
/// # Parameters
/// * `s` - The string to print
///
/// # Panics
/// Panics if `s` contains an embedded null byte.
pub fn trace(s: &str) {
    print_error_buf(s);
}

/// Debug printing function with newline.
/// Writes the given string followed by a newline to the error buffer.
///
/// # Parameters
/// * `str` - The string to print
///
/// # Panics
/// Panics if `str` contains an embedded null byte.
pub fn traceln(string: &str) {
    print_error_buf(string);
    print_error_buf("\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trace_compiles() {
        // Verify the function can be called.
        // Actual behavior depends on the C runtime being initialized.
        trace("test");
    }

    #[test]
    fn test_traceln_compiles() {
        // Verify the function can be called.
        // Actual behavior depends on the C runtime being initialized.
        traceln("test");
    }
}
