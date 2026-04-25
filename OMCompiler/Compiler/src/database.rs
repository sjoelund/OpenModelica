//! Translation of Util/Database.mo
//!
//! This module provides functionality for creating and using SQLite databases.
//! It is a wrapper to SQLite via the omcruntime C library.

use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_void};

// ============================================================================
// FFI bindings to omcruntime C library
// ============================================================================

// Opens a database with the given index and name.
// Returns 0 on success, non-zero error code on failure.
unsafe extern "C" {
    pub fn Database_open(index: c_int, name: *const c_char) -> c_int;
}

// Query a database with the given index.
unsafe extern "C" {
    pub fn Database_query(
        index: c_int,
        sql: *const c_char,
        result: *mut *mut c_void,
    ) -> c_int;
}

// ============================================================================
// Safe wrapper functions
// ============================================================================

/// Error codes returned by the C database functions.
pub const DATABASE_ERROR_DATABASE_INDEX_OVERFLOW: c_int = 500;
pub const DATABASE_ERROR_NOT_INITIALIZED: c_int = 501;

/// Error indicating the database index is out of range (>= 1024 or < 0).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseError {
    /// The database index is out of range.
    IndexOverflow,
    /// The database at this index has not been initialized.
    NotInitialized,
    /// The C function returned a non-zero error code.
    Other(c_int),
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseError::IndexOverflow => {
                write!(f, "database index out of range (max 1024)")
            },
            DatabaseError::NotInitialized => {
                write!(f, "database at this index is not initialized")
            },
            DatabaseError::Other(code) => {
                write!(f, "database error with code {}", code)
            },
        }
    }
}

impl std::error::Error for DatabaseError {}

/// Opens a database with the given index and name.
///
/// # Parameters
/// * `index` - The database index (max 1024)
/// * `name` - The name of the file, or `:memory:` for an in-memory database
///
/// # Returns
/// `Ok(())` on success, or a `DatabaseError` if it cannot open the database.
///
/// # Panics
/// Panics if `name` contains an embedded null byte.
pub fn open(index: i32, name: &str) -> Result<(), DatabaseError> {
    let c_name = CString::new(name).expect("name contains null byte");
    let rc = unsafe { Database_open(index as c_int, c_name.as_ptr()) };
    if rc == 0 {
        Ok(())
    } else if rc == DATABASE_ERROR_DATABASE_INDEX_OVERFLOW {
        Err(DatabaseError::IndexOverflow)
    } else if rc == DATABASE_ERROR_NOT_INITIALIZED {
        Err(DatabaseError::NotInitialized)
    } else {
        Err(DatabaseError::Other(rc))
    }
}

/// Query a database with the given index.
///
/// # Parameters
/// * `index` - The database index (previously opened via `open`)
/// * `sql` - The SQL query string
///
/// # Returns
/// `Ok(())` on success, or a `DatabaseError` if the query fails.
///
/// # Note
/// The `result` parameter is passed through to the underlying SQLite callback
/// mechanism. The actual result data is populated by the C library.
/// This wrapper currently only checks for query success/failure.
///
/// # Panics
/// Panics if `sql` contains an embedded null byte.
pub fn query(index: i32, sql: &str) -> Result<(), DatabaseError> {
    let c_sql = CString::new(sql).expect("sql contains null byte");
    let rc = unsafe { Database_query(index as c_int, c_sql.as_ptr(), std::ptr::null_mut()) };
    if rc == 0 {
        Ok(())
    } else if rc == DATABASE_ERROR_DATABASE_INDEX_OVERFLOW {
        Err(DatabaseError::IndexOverflow)
    } else if rc == DATABASE_ERROR_NOT_INITIALIZED {
        Err(DatabaseError::NotInitialized)
    } else {
        Err(DatabaseError::Other(rc))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_codes() {
        assert_eq!(DATABASE_ERROR_DATABASE_INDEX_OVERFLOW, 500);
        assert_eq!(DATABASE_ERROR_NOT_INITIALIZED, 501);
    }

    #[test]
    fn test_database_error_display() {
        assert_eq!(
            format!("{}", DatabaseError::IndexOverflow),
            "database index out of range (max 1024)"
        );
        assert_eq!(
            format!("{}", DatabaseError::NotInitialized),
            "database at this index is not initialized"
        );
        assert_eq!(
            format!("{}", DatabaseError::Other(42)),
            "database error with code 42"
        );
    }

    #[test]
    fn test_database_error_debug() {
        let err = DatabaseError::Other(501);
        assert!(format!("{:?}", err).contains("501"));
    }

    #[test]
    fn test_database_error_eq() {
        assert_eq!(DatabaseError::IndexOverflow, DatabaseError::IndexOverflow);
        assert_eq!(DatabaseError::NotInitialized, DatabaseError::NotInitialized);
        assert_eq!(DatabaseError::Other(1), DatabaseError::Other(1));
        assert_ne!(DatabaseError::Other(1), DatabaseError::Other(2));
    }
}
