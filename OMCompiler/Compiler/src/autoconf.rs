//! Translation of Autoconf.mo
//!
//! This module provides build configuration constants derived from the
//! Autoconf package, including OS detection, platform strings, linker flags,
//! and system library lists.

use im::Vector;

/// Persistent list type (mapped to im::Vector since im 15.x has no List)
type List<T> = Vector<T>;

// ============================================================================
// OS and platform detection
// ============================================================================

/// The configure command line used to build this project.
pub const CONFIGURE_COMMAND_LINE: &str =
    "Configured 2026-04-25 07:58:33 using arguments:  '--disable-option-checking' '--prefix=/home/martin/dev/OpenModelica-rust/build' '--with-ombuilddir=/home/martin/dev/OpenModelica-rust/build' '--cache-file=/dev/null' '--srcdir=.'";

/// Detected operating system: "linux".
pub const OS: &str = "linux";

/// Whether the target is a 64-bit platform.
pub const IS_64_BIT: bool = true;

/// Whether the target OS is Windows. Always `false` for this build.
/// (Computed from `OS == "Windows_NT"` but string comparison is not const in stable Rust.)
pub const IS_WINDOWS: bool = false;

/// Platform string: "Unix" on non-Windows, "WIN64"/"WIN32" on Windows.
pub const PLATFORM: &str = if IS_WINDOWS && IS_64_BIT {
    "WIN64"
} else if IS_WINDOWS {
    "WIN32"
} else {
    "Unix"
};

// ============================================================================
// Build tool strings
// ============================================================================

/// The make command name.
pub const MAKE: &str = "make";

/// The cmake command name.
pub const CMAKE: &str = "cmake";

/// File extension for executables: ".exe" on Windows, "" otherwise.
pub const EXE_EXT: &str = if IS_WINDOWS { ".exe" } else { "" };

/// Shared library extension: ".so".
pub const DLL_EXT: &str = ".so";

// ============================================================================
// Static/dynamic linking flags
// ============================================================================

/// Whether static linking flags are available.
pub const HAVE_BSTATIC: bool = true;

/// Linker flag to request static linking: "-Wl,-Bstatic" or "".
pub const BSTATIC: &str = if HAVE_BSTATIC { "-Wl,-Bstatic" } else { "" };

/// Linker flag to request dynamic linking: "-Wl,-Bdynamic" or "".
pub const BDYNAMIC: &str = if HAVE_BSTATIC { "-Wl,-Bdynamic" } else { "" };

// ============================================================================
// Path and delimiter strings
// ============================================================================

/// Path delimiter character for lists (e.g. library paths): ";" on Windows, ":" otherwise.
pub const GROUP_DELIMITER: &str = if IS_WINDOWS { ";" } else { ":" };

/// Path component delimiter: "/".
pub const PATH_DELIMITER: &str = "/";

// ============================================================================
// Linker flags
// ============================================================================

/// Runtime linker flags for the main runtime.
pub const LDFLAGS_RUNTIME: &str =
    " -Wl,--no-as-needed -Wl,--disable-new-dtags -lOpenModelicaRuntimeC -lopenblas -lm -lomcgc -lryu -lpthread -rdynamic";

/// Runtime linker flags for simulation.
pub const LDFLAGS_RUNTIME_SIM: &str =
    " -Wl,--no-as-needed -Wl,--disable-new-dtags -lSimulationRuntimeC -lopenblas -lm -lomcgc -lryu -lpthread -rdynamic -Wl,--no-undefined";

/// Runtime linker flags for FMU generation.
pub const LDFLAGS_RUNTIME_FMU: &str =
    " -Wl,--no-as-needed -Wl,--disable-new-dtags -lopenblas -lm -lpthread -lryu -rdynamic  -Wl,--no-undefined";

/// Runtime linker flags for static FMU generation.
pub const LDFLAGS_RUNTIME_FMU_STATIC: &str =
    " -Wl,-Bstatic -lSimulationRuntimeFMI  -Wl,--no-as-needed -Wl,--disable-new-dtags -lopenblas -Wl,-Bdynamic -lryu -lm -lpthread -rdynamic  -Wl,--no-undefined";

// ============================================================================
// System libraries
// ============================================================================

/// CORBA libraries (empty for this build).
pub const CORBA_LIBS: &str = "";

/// hwloc library flag (empty because condition `0 == 1` is false).
pub const HWLOC: &str = "";

/// Returns the list of system libraries to link against.
/// This is provided as a function because `im::Vector` construction
/// requires non-const initialization.
pub fn system_libs() -> List<&'static str> {
    let mut libs = List::new();
    libs.push_back("-lomcruntime");
    libs.push_back("-lexpat");
    libs.push_back("-lsqlite3");
    libs.push_back(CORBA_LIBS);
    libs.push_back("-lomcgc");
    libs.push_back(HWLOC);
    libs
}

/// Build triple target string (e.g. "x86_64-linux-gnu").
pub const TRIPLE: &str = "x86_64-linux-gnu";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_constants() {
        assert_eq!(OS, "linux");
        assert_eq!(IS_64_BIT, true);
        assert_eq!(IS_WINDOWS, false);
        assert_eq!(PLATFORM, "Unix");
        assert_eq!(EXE_EXT, "");
        assert_eq!(GROUP_DELIMITER, ":");
    }

    #[test]
    fn test_link_flags() {
        assert_eq!(BSTATIC, "-Wl,-Bstatic");
        assert_eq!(BDYNAMIC, "-Wl,-Bdynamic");
        assert!(LDFLAGS_RUNTIME.contains("OpenModelicaRuntimeC"));
        assert!(LDFLAGS_RUNTIME_SIM.contains("SimulationRuntimeC"));
    }

    #[test]
    fn test_system_libs() {
        let libs = system_libs();
        assert_eq!(libs.len(), 6);
        assert_eq!(libs.get(0).map(|s| *s), Some("-lomcruntime"));
        assert_eq!(libs.get(1).map(|s| *s), Some("-lexpat"));
        assert_eq!(libs.get(2).map(|s| *s), Some("-lsqlite3"));
        assert_eq!(libs.get(3).map(|s| *s), Some("")); // corbaLibs
        assert_eq!(libs.get(4).map(|s| *s), Some("-lomcgc"));
        assert_eq!(libs.get(5).map(|s| *s), Some("")); // hwloc
    }

    #[test]
    fn test_hwloc_disabled() {
        assert_eq!(HWLOC, "");
    }
}
