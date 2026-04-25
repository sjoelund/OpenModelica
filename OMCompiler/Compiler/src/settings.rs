//! Translation of Util/Settings.mo
//!
//! This module provides wrappers around the OpenModelica C runtime settings
//! functions exposed via the `omcruntime` library. It covers path configuration,
//! home directory lookup, and echo control.

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

// ============================================================================
// FFI bindings to omcruntime C library
// ============================================================================

// Returns the version number string of this release.
// Corresponds to `external "C" outString=Settings_getVersionNr()`.
#[allow(dead_code)]
unsafe extern "C" {
    pub fn Settings_getVersionNr() -> *const c_char;
}

// Sets the temporary directory path.
// Corresponds to `external "C" SettingsImpl__setTempDirectoryPath(inString)`.
#[allow(dead_code)]
unsafe extern "C" {
    pub fn SettingsImpl__setTempDirectoryPath(str_ptr: *const c_char);
}

// Returns the temporary directory path.
// Corresponds to `external "C" outString=Settings_getTempDirectoryPath()`.
// Note: The returned string is allocated by the C runtime.
#[allow(dead_code)]
unsafe extern "C" {
    pub fn Settings_getTempDirectoryPath() -> *const c_char;
}

// Sets the installation directory path (pass empty string to clear).
// Corresponds to `external "C" SettingsImpl__setInstallationDirectoryPath(inString)`.
#[allow(dead_code)]
unsafe extern "C" {
    pub fn SettingsImpl__setInstallationDirectoryPath(str_ptr: *const c_char);
}

// Returns the installation directory path.
// Corresponds to `external "C" outString=Settings_getInstallationDirectoryPath()`.
// Note: The returned string is allocated by the C runtime.
#[allow(dead_code)]
unsafe extern "C" {
    pub fn Settings_getInstallationDirectoryPath() -> *const c_char;
}

// Sets the Modelica path.
// Corresponds to `external "C" SettingsImpl__setModelicaPath(inString)`.
#[allow(dead_code)]
unsafe extern "C" {
    pub fn SettingsImpl__setModelicaPath(str_ptr: *const c_char);
}

// Returns the Modelica path, taking a `runningTestsuite` flag.
// Corresponds to `external "C" outString=Settings_getModelicaPath(runningTestsuite)`.
// Note: The returned string is allocated by the C runtime.
#[allow(dead_code)]
unsafe extern "C" {
    pub fn Settings_getModelicaPath(running_testsuite: c_int) -> *const c_char;
}

// Returns the home directory, taking a `runningTestsuite` flag.
// Corresponds to `external "C" outString=Settings_getHomeDir(runningTestsuite)`.
// Note: The returned string is allocated by the C runtime.
#[allow(dead_code)]
unsafe extern "C" {
    pub fn Settings_getHomeDir(running_testsuite: c_int) -> *const c_char;
}

// Returns the current echo setting.
// Corresponds to `external "C" echo=Settings_getEcho()`.
#[allow(dead_code)]
unsafe extern "C" {
    pub fn Settings_getEcho() -> c_int;
}

// Sets the echo setting.
// Corresponds to `external "C" Settings_setEcho(echo)`.
#[allow(dead_code)]
unsafe extern "C" {
    pub fn Settings_setEcho(echo: c_int);
}

// ============================================================================
// Safe wrapper functions
// ============================================================================

/// Returns the version number of this release.
/// Corresponds to `getVersionNr` in MetaModelica.
///
/// # Returns
/// The version string (e.g., "v6.0.0-dev").
pub fn get_version_nr() -> String {
    unsafe {
        let ptr = Settings_getVersionNr();
        if ptr.is_null() {
            String::new()
        } else {
            CStr::from_ptr(ptr)
                .to_string_lossy()
                .into_owned()
        }
    }
}

/// Sets the temporary directory path.
/// Corresponds to `setTempDirectoryPath` in MetaModelica.
///
/// # Parameters
/// * `path` - The temporary directory path to set.
///
/// # Panics
/// Panics if `path` contains an embedded null byte.
pub fn set_temp_directory_path(path: &str) {
    let c_path = CString::new(path).expect("path contains null byte");
    unsafe {
        SettingsImpl__setTempDirectoryPath(c_path.as_ptr());
    }
}

/// Returns the temporary directory path.
/// Corresponds to `getTempDirectoryPath` in MetaModelica.
///
/// # Returns
/// The temporary directory path as a String.
pub fn get_temp_directory_path() -> String {
    unsafe {
        let ptr = Settings_getTempDirectoryPath();
        if ptr.is_null() {
            String::new()
        } else {
            CStr::from_ptr(ptr)
                .to_string_lossy()
                .into_owned()
        }
    }
}

/// Sets the installation directory path.
/// Pass an empty string to clear it.
/// Corresponds to `setInstallationDirectoryPath` in MetaModelica.
///
/// # Parameters
/// * `path` - The installation directory path (empty string clears it).
///
/// # Panics
/// Panics if `path` contains an embedded null byte.
pub fn set_installation_directory_path(path: &str) {
    let c_path = CString::new(path).expect("path contains null byte");
    unsafe {
        SettingsImpl__setInstallationDirectoryPath(c_path.as_ptr());
    }
}

/// Returns the installation directory path.
/// Corresponds to `getInstallationDirectoryPath` in MetaModelica.
///
/// # Returns
/// The installation directory path as a String.
pub fn get_installation_directory_path() -> String {
    unsafe {
        let ptr = Settings_getInstallationDirectoryPath();
        if ptr.is_null() {
            String::new()
        } else {
            CStr::from_ptr(ptr)
                .to_string_lossy()
                .into_owned()
        }
    }
}

/// Sets the Modelica path.
/// Corresponds to `setModelicaPath` in MetaModelica.
///
/// # Parameters
/// * `path` - The Modelica path to set.
///
/// # Panics
/// Panics if `path` contains an embedded null byte.
pub fn set_modelica_path(path: &str) {
    let c_path = CString::new(path).expect("path contains null byte");
    unsafe {
        SettingsImpl__setModelicaPath(c_path.as_ptr());
    }
}

/// Returns the Modelica path, taking a `running_testsuite` flag.
/// Corresponds to `getModelicaPath` in MetaModelica.
///
/// # Parameters
/// * `running_testsuite` - Whether the compiler is running in test mode.
///
/// # Returns
/// The Modelica path as a String.
pub fn get_modelica_path(running_testsuite: bool) -> String {
    let flag = if running_testsuite { 1 } else { 0 };
    unsafe {
        let ptr = Settings_getModelicaPath(flag);
        if ptr.is_null() {
            String::new()
        } else {
            CStr::from_ptr(ptr)
                .to_string_lossy()
                .into_owned()
        }
    }
}

/// Returns the home directory, taking a `running_testsuite` flag.
/// Corresponds to `getHomeDir` in MetaModelica.
///
/// # Parameters
/// * `running_testsuite` - Whether the compiler is running in test mode.
///
/// # Returns
/// The home directory path as a String.
pub fn get_home_dir(running_testsuite: bool) -> String {
    let flag = if running_testsuite { 1 } else { 0 };
    unsafe {
        let ptr = Settings_getHomeDir(flag);
        if ptr.is_null() {
            String::new()
        } else {
            CStr::from_ptr(ptr)
                .to_string_lossy()
                .into_owned()
        }
    }
}

/// Returns the current echo setting.
/// Corresponds to `getEcho` in MetaModelica.
///
/// # Returns
/// The echo setting as an i32.
pub fn get_echo() -> i32 {
    unsafe { Settings_getEcho() }
}

/// Sets the echo setting.
/// Corresponds to `setEcho` in MetaModelica.
///
/// # Parameters
/// * `echo` - The echo setting to use.
pub fn set_echo(echo: i32) {
    unsafe {
        Settings_setEcho(echo);
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_temp_directory_path_no_null() {
        // Verify that a path with no null bytes does not panic
        let result = std::panic::catch_unwind(|| {
            set_temp_directory_path("/tmp/test");
        });
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_installation_path_no_null() {
        let result = std::panic::catch_unwind(|| {
            set_installation_directory_path("");
        });
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_modelica_path_no_null() {
        let result = std::panic::catch_unwind(|| {
            set_modelica_path("/path/to/modelica");
        });
        assert!(result.is_ok());
    }
}
