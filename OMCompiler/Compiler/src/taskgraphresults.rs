//! Translation of Util/TaskGraphResults.mo
//!
//! This module provides functions for checking if a given task graph or code graph
//! has the same structure as a reference graph and correct node and edge values.

use std::ffi::{c_char, CString};

// ============================================================================
// Extern "C" declarations (C API from omcruntime)
// ============================================================================

unsafe extern "C" {
    /// External "C" call - TaskGraphResults_checkTaskGraph
    /// Checks if the given taskGraph has the same structure as the reference
    /// graph and correct node and edge values.
    ///
    /// Returns a pointer to a C-string representing the result list.
    /// The caller is responsible for freeing the returned string.
    fn TaskGraphResults_checkTaskGraph(
        filename: *const c_char,
        reffilename: *const c_char,
    ) -> *mut c_char;

    /// External "C" call - TaskGraphResults_checkCodeGraph
    /// Checks if the given code graph has the same structure as the reference
    /// graph and correct node and edge values.
    ///
    /// Returns a pointer to a C-string representing the result list.
    /// The caller is responsible for freeing the returned string.
    fn TaskGraphResults_checkCodeGraph(
        graphfile: *const c_char,
        codefile: *const c_char,
    ) -> *mut c_char;
}

// ============================================================================
// Helper: parse a C string representing a MetaModelica list into a Vector
// ============================================================================

/// Parses a C string in the format `{"item1","item2",...}` or `{"item"}` into a
/// `Vec<String>`. Handles the typical MetaModelica list-to-string representation.
#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn parse_list_string(s: *mut c_char) -> Vec<String> {
    if s.is_null() {
        return Vec::new();
    }

    let c_str = CString::from_raw(s);
    let text = c_str.to_string_lossy().to_string();

    // MetaModelica list format: {"elem1","elem2",...}
    // Remove outer braces
    let trimmed = text.trim();
    if let Some(inner) = trimmed.strip_prefix('{').and_then(|s| s.strip_suffix('}')) {
        inner
            .split(',')
            .filter_map(|item| {
                let item = item.trim().trim_matches('"');
                if item.is_empty() {
                    None
                } else {
                    Some(item.to_string())
                }
            })
            .collect()
    } else {
        // If not in list format, treat as a single item
        let text = text.trim().to_string();
        if text.is_empty() {
            Vec::new()
        } else {
            vec![text]
        }
    }
}

// ============================================================================
// Helper: convert a Vec<String> to a MetaModelica list string for FFI return
// ============================================================================

/// Converts a `Vec<String>` into a C string in the MetaModelica list format
/// `{"item1","item2",...}`.
fn vec_to_list_string(v: &[String]) -> *mut c_char {
    let inner: Vec<String> = v
        .iter()
        .map(|s| format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\"")))
        .collect();
    let result = format!("{{{}}}", inner.join(","));
    CString::new(result).expect("CString allocation failed").into_raw()
}

// ============================================================================
// Functions
// ============================================================================

/// Checks if the given task graph has the same structure as the reference graph
/// and correct node and edge values.
///
/// Equivalent to the `checkTaskGraph` external C function in OpenModelica.
/// The `filename` and `reffilename` arguments are paths to graph files.
///
/// # Safety
///
/// This function calls an external C function from the `omcruntime` library.
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn check_task_graph(
    filename: &str,
    reffilename: &str,
) -> Result<Vec<String>, &'static str> {
    let filename_c =
        CString::new(filename).map_err(|_| "filename contains null bytes")?;
    let reffilename_c =
        CString::new(reffilename).map_err(|_| "reffilename contains null bytes")?;

    let result = TaskGraphResults_checkTaskGraph(
        filename_c.as_ptr(),
        reffilename_c.as_ptr(),
    );

    // CString::from_raw takes ownership; parse_list_string consumes it
    Ok(parse_list_string(result))
}

/// Checks if the given code graph has the same structure as the reference
/// graph and correct node and edge values.
///
/// Equivalent to the `checkCodeGraph` external C function in OpenModelica.
/// The `graphfile` and `codefile` arguments are paths to graph/code files.
///
/// # Safety
///
/// This function calls an external C function from the `omcruntime` library.
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn check_code_graph(
    graphfile: &str,
    codefile: &str,
) -> Result<Vec<String>, &'static str> {
    let graphfile_c =
        CString::new(graphfile).map_err(|_| "graphfile contains null bytes")?;
    let codefile_c =
        CString::new(codefile).map_err(|_| "codefile contains null bytes")?;

    let result = TaskGraphResults_checkCodeGraph(
        graphfile_c.as_ptr(),
        codefile_c.as_ptr(),
    );

    // CString::from_raw takes ownership; parse_list_string consumes it
    Ok(parse_list_string(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_list_string_empty() {
        let s = CString::new("{}").unwrap().into_raw();
        let result = unsafe { parse_list_string(s) };
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_parse_list_string_single() {
        let s = CString::new(r#"{ "hello" }"#).unwrap().into_raw();
        let result = unsafe { parse_list_string(s) };
        assert_eq!(result, vec!["hello"]);
    }

    #[test]
    fn test_parse_list_string_multiple() {
        let s = CString::new(r#"{ "a","b","c" }"#).unwrap().into_raw();
        let result = unsafe { parse_list_string(s) };
        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_parse_list_string_null() {
        let result = unsafe { parse_list_string(std::ptr::null_mut()) };
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_vec_to_list_string_empty() {
        let v: Vec<String> = Vec::new();
        let c_str = unsafe { CString::from_raw(vec_to_list_string(&v)) };
        assert_eq!(c_str.to_string_lossy(), "{}");
    }

    #[test]
    fn test_vec_to_list_string_items() {
        let v = vec!["hello".to_string(), "world".to_string()];
        let c_str = unsafe { CString::from_raw(vec_to_list_string(&v)) };
        let expected = r#"{"hello","world"}"#;
        assert_eq!(c_str.to_string_lossy(), expected);
    }

    #[test]
    fn test_roundtrip() {
        let items: Vec<String> = vec!["foo".to_string(), "bar".to_string(), "baz".to_string()];
        let expected = items.clone();
        let c_str = vec_to_list_string(&items);
        let parsed = unsafe { parse_list_string(c_str) };
        assert_eq!(parsed, expected);
    }
}
