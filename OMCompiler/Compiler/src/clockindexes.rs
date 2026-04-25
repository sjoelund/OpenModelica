//! Translation of Util/ClockIndexes.mo
//!
//! This module provides clock index constants used by the real-time clocks
//! in a separate package to ease customisation (different indexes depending
//! on back-end). Compiled as a utility package since Susan uses timers.

use im::Vector;

/// Persistent list type (mapped to im::Vector since im 15.x has no List)
type List<T> = Vector<T>;

// ============================================================================
// Constants
// ============================================================================

/// No clock index.
pub const RT_NO_CLOCK: i32 = -1;

/// Simulate total clock index.
pub const RT_CLOCK_SIMULATE_TOTAL: i32 = 8;

/// Simulate simulation clock index.
pub const RT_CLOCK_SIMULATE_SIMULATION: i32 = 9;

/// Build model clock index.
pub const RT_CLOCK_BUILD_MODEL: i32 = 10;

/// Execution statistics clock index.
pub const RT_CLOCK_EXECSTAT: i32 = 11;

/// Frontend clock index.
pub const RT_CLOCK_FRONTEND: i32 = 13;

/// Backend clock index.
pub const RT_CLOCK_BACKEND: i32 = 14;

/// Simcode clock index.
pub const RT_CLOCK_SIMCODE: i32 = 15;

/// Linearize clock index.
pub const RT_CLOCK_LINEARIZE: i32 = 16;

/// Templates clock index.
pub const RT_CLOCK_TEMPLATES: i32 = 17;

/// Uncertainties clock index.
pub const RT_CLOCK_UNCERTAINTIES: i32 = 18;

/// Profiler 0 clock index.
pub const RT_PROFILER0: i32 = 19;

/// Profiler 1 clock index.
pub const RT_PROFILER1: i32 = 20;

/// Profiler 2 clock index.
pub const RT_PROFILER2: i32 = 21;

/// Execution statistics Jacobians clock index.
pub const RT_CLOCK_EXECSTAT_JACOBIANS: i32 = 22;

/// User reserved clock index.
pub const RT_CLOCK_USER_RESERVED: i32 = 23;

/// Execution statistics HPCOM modules clock index.
pub const RT_CLOCK_EXECSTAT_HPCOM_MODULES: i32 = 24;

/// Show statement clock index.
pub const RT_CLOCK_SHOW_STATEMENT: i32 = 25;

/// FInst clock index.
pub const RT_CLOCK_FINST: i32 = 26;

/// New backend module clock index.
pub const RT_CLOCK_NEW_BACKEND_MODULE: i32 = 29;

/// New backend initialization clock index.
pub const RT_CLOCK_NEW_BACKEND_INITIALIZATION: i32 = 30;

// ============================================================================
// Clock index list
// ============================================================================

/// Returns the list of clocks used during build model.
/// Contains: RT_CLOCK_BUILD_MODEL, RT_CLOCK_SIMULATE_TOTAL,
/// RT_CLOCK_TEMPLATES, RT_CLOCK_LINEARIZE, RT_CLOCK_SIMCODE,
/// RT_CLOCK_BACKEND, RT_CLOCK_FRONTEND.
pub fn build_model_clocks() -> List<i32> {
    let mut clocks = List::new();
    clocks.push_back(RT_CLOCK_BUILD_MODEL);
    clocks.push_back(RT_CLOCK_SIMULATE_TOTAL);
    clocks.push_back(RT_CLOCK_TEMPLATES);
    clocks.push_back(RT_CLOCK_LINEARIZE);
    clocks.push_back(RT_CLOCK_SIMCODE);
    clocks.push_back(RT_CLOCK_BACKEND);
    clocks.push_back(RT_CLOCK_FRONTEND);
    clocks
}

// ============================================================================
// Functions
// ============================================================================

/// Converts a clock index to its short string representation.
///
/// # Parameters
/// * `clock_index` - The clock index to convert
///
/// # Returns
/// A string abbreviation for the clock, or "ERR" for unknown indices,
/// "NON" for RT_NO_CLOCK.
pub fn to_string(clock_index: i32) -> &'static str {
    match clock_index {
        RT_NO_CLOCK => "NON",
        RT_CLOCK_SIMULATE_TOTAL => "STO",
        RT_CLOCK_SIMULATE_SIMULATION => "SSI",
        RT_CLOCK_BUILD_MODEL => "BLD",
        RT_CLOCK_EXECSTAT => "EXS",
        RT_CLOCK_FRONTEND => "FRT",
        RT_CLOCK_BACKEND => "BCK",
        RT_CLOCK_SIMCODE => "SCD",
        RT_CLOCK_LINEARIZE => "LIN",
        RT_CLOCK_TEMPLATES => "TMP",
        RT_CLOCK_UNCERTAINTIES => "UNC",
        RT_PROFILER0 => "PR0",
        RT_PROFILER1 => "PR1",
        RT_PROFILER2 => "PR2",
        RT_CLOCK_EXECSTAT_JACOBIANS => "JAC",
        RT_CLOCK_USER_RESERVED => "RES",
        RT_CLOCK_EXECSTAT_HPCOM_MODULES => "HPC",
        RT_CLOCK_SHOW_STATEMENT => "STM",
        RT_CLOCK_FINST => "FIN",
        RT_CLOCK_NEW_BACKEND_MODULE => "SIM",
        RT_CLOCK_NEW_BACKEND_INITIALIZATION => "INI",
        _ => "ERR",
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_clock_constant() {
        assert_eq!(RT_NO_CLOCK, -1);
    }

    #[test]
    fn test_clock_constants() {
        assert_eq!(RT_CLOCK_SIMULATE_TOTAL, 8);
        assert_eq!(RT_CLOCK_BUILD_MODEL, 10);
        assert_eq!(RT_CLOCK_FRONTEND, 13);
        assert_eq!(RT_CLOCK_BACKEND, 14);
        assert_eq!(RT_CLOCK_SIMCODE, 15);
        assert_eq!(RT_CLOCK_LINEARIZE, 16);
        assert_eq!(RT_CLOCK_TEMPLATES, 17);
        assert_eq!(RT_CLOCK_UNCERTAINTIES, 18);
        assert_eq!(RT_CLOCK_FINST, 26);
        assert_eq!(RT_CLOCK_NEW_BACKEND_INITIALIZATION, 30);
    }

    #[test]
    fn test_profiler_constants() {
        assert_eq!(RT_PROFILER0, 19);
        assert_eq!(RT_PROFILER1, 20);
        assert_eq!(RT_PROFILER2, 21);
    }

    #[test]
    fn test_build_model_clocks_list() {
        let clocks = build_model_clocks();
        assert_eq!(clocks.len(), 7);
        assert_eq!(clocks.get(0).map(|v| *v), Some(RT_CLOCK_BUILD_MODEL));
        assert_eq!(clocks.get(1).map(|v| *v), Some(RT_CLOCK_SIMULATE_TOTAL));
        assert_eq!(clocks.get(2).map(|v| *v), Some(RT_CLOCK_TEMPLATES));
        assert_eq!(clocks.get(3).map(|v| *v), Some(RT_CLOCK_LINEARIZE));
        assert_eq!(clocks.get(4).map(|v| *v), Some(RT_CLOCK_SIMCODE));
        assert_eq!(clocks.get(5).map(|v| *v), Some(RT_CLOCK_BACKEND));
        assert_eq!(clocks.get(6).map(|v| *v), Some(RT_CLOCK_FRONTEND));
    }

    #[test]
    fn test_to_string_known_indices() {
        assert_eq!(to_string(RT_NO_CLOCK), "NON");
        assert_eq!(to_string(RT_CLOCK_BUILD_MODEL), "BLD");
        assert_eq!(to_string(RT_CLOCK_FRONTEND), "FRT");
        assert_eq!(to_string(RT_CLOCK_BACKEND), "BCK");
        assert_eq!(to_string(RT_CLOCK_SIMCODE), "SCD");
        assert_eq!(to_string(RT_CLOCK_LINEARIZE), "LIN");
        assert_eq!(to_string(RT_CLOCK_TEMPLATES), "TMP");
        assert_eq!(to_string(RT_CLOCK_UNCERTAINTIES), "UNC");
        assert_eq!(to_string(RT_PROFILER0), "PR0");
        assert_eq!(to_string(RT_PROFILER1), "PR1");
        assert_eq!(to_string(RT_PROFILER2), "PR2");
        assert_eq!(to_string(RT_CLOCK_EXECSTAT_JACOBIANS), "JAC");
        assert_eq!(to_string(RT_CLOCK_USER_RESERVED), "RES");
        assert_eq!(to_string(RT_CLOCK_EXECSTAT_HPCOM_MODULES), "HPC");
        assert_eq!(to_string(RT_CLOCK_SHOW_STATEMENT), "STM");
        assert_eq!(to_string(RT_CLOCK_FINST), "FIN");
        assert_eq!(to_string(RT_CLOCK_NEW_BACKEND_MODULE), "SIM");
        assert_eq!(to_string(RT_CLOCK_NEW_BACKEND_INITIALIZATION), "INI");
    }

    #[test]
    fn test_to_string_unknown() {
        assert_eq!(to_string(999), "ERR");
    }

    #[test]
    fn test_to_string_all_known() {
        assert_eq!(to_string(RT_CLOCK_SIMULATE_TOTAL), "STO");
        assert_eq!(to_string(RT_CLOCK_SIMULATE_SIMULATION), "SSI");
        assert_eq!(to_string(RT_CLOCK_EXECSTAT), "EXS");
    }
}
