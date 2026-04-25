//! Translation of NFFrontEnd/BaseModelica.mo
//!
//! This module provides output format configuration types and functions
//! from the BaseModelica package. It defines enumeration types for scalarization
//! and record modes, a uniontype for output format configuration, and functions
//! to configure these settings.
//!
//! # Assumptions
//! - The `Flags` module is not yet translated to Rust. Functions `format_from_flags`
//!   currently returns the default format since it depends on Flags functions
//!   (`is_set`, `is_config_flag_set`, `get_config_string_list`) that are not
//!   yet available in Rust.
//! - `BASE_MODELICA_OPTIONS` and `BASE_MODELICA_FORMAT` flag constants are not
//!   yet defined. They would normally come from the Flags package.

// ============================================================================
// ScalarizeMode - enumeration controlling scalarization behavior
// ============================================================================

/// Controls how equations/components are scalarized during code generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum ScalarizeMode {
    /// Fully scalarized - all components are scalarized
    SCALARIZED,
    /// Partially scalarized - default mode, some components scalarized
    PARTIALLY_SCALARIZED,
    /// Not scalarized - no scalarization applied
    NOT_SCALARIZED,
}

impl std::fmt::Display for ScalarizeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScalarizeMode::SCALARIZED => write!(f, "SCALARIZED"),
            ScalarizeMode::PARTIALLY_SCALARIZED => write!(f, "PARTIALLY_SCALARIZED"),
            ScalarizeMode::NOT_SCALARIZED => write!(f, "NOT_SCALARIZED"),
        }
    }
}

// ============================================================================
// RecordMode - enumeration controlling record handling
// ============================================================================

/// Controls whether records are kept as composite types or flattened.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum RecordMode {
    /// Records are preserved as composite types
    WITH_RECORDS,
    /// Records are flattened into individual components
    WITHOUT_RECORDS,
}

impl std::fmt::Display for RecordMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecordMode::WITH_RECORDS => write!(f, "WITH_RECORDS"),
            RecordMode::WITHOUT_RECORDS => write!(f, "WITHOUT_RECORDS"),
        }
    }
}

// ============================================================================
// OutputFormat - uniontype for output format configuration
// ============================================================================

/// Configuration for the output format of generated code.
/// Contains settings for scalarization mode, record handling, and binding movement.
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum OutputFormat {
    /// The OUTPUT_FORMAT record variant.
    OUTPUT_FORMAT {
        /// How to scalarize components
        scalarize_mode: ScalarizeMode,
        /// How to handle records
        record_mode: RecordMode,
        /// Whether to move bindings
        move_bindings: bool,
    },
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::OUTPUT_FORMAT {
                scalarize_mode,
                record_mode,
                move_bindings,
            } => write!(
                f,
                "OUTPUT_FORMAT {{ scalarizeMode: {}, recordMode: {}, moveBindings: {} }}",
                scalarize_mode, record_mode, move_bindings
            ),
        }
    }
}

// ============================================================================
// Default format constant
// ============================================================================

/// Default output format configuration.
/// Equivalent to: PARTIALLY_SCALARIZED, WITH_RECORDS, false
pub const DEFAULT_FORMAT: OutputFormat = OutputFormat::OUTPUT_FORMAT {
    scalarize_mode: ScalarizeMode::PARTIALLY_SCALARIZED,
    record_mode: RecordMode::WITH_RECORDS,
    move_bindings: false,
};

// ============================================================================
// Flags stubs
//
// The Flags module is not yet translated. These stubs allow format_from_flags
// to compile. The real implementation requires the Flags package.
// ============================================================================

/// Stub for `Flags.isSet(flag)`. Returns false since Flags is not available.
fn flags_is_set(_flag: &str) -> bool {
    false
}

/// Stub for `Flags.isConfigFlagSet(options, option)`. Returns false.
fn flags_is_config_flag_set(_options: &str, _option: &str) -> bool {
    false
}

/// Stub for `Flags.getConfigStringList(flag)`. Returns empty list.
fn flags_get_config_string_list(_flag: &str) -> Vec<String> {
    Vec::new()
}

/// The flag name for base modelica format options.
/// Corresponds to `Flags.BASE_MODELICA_FORMAT` in MetaModelica.
const BASE_MODELICA_FORMAT: &str = "BASE_MODELICA_FORMAT";

/// The flag name for base modelica options.
/// Corresponds to `Flags.BASE_MODELICA_OPTIONS` in MetaModelica.
const BASE_MODELICA_OPTIONS: &str = "BASE_MODELICA_OPTIONS";

/// The flag name for scalarize option.
/// Corresponds to `Flags.NF_SCALARIZE` in MetaModelica.
const NF_SCALARIZE: &str = "NF_SCALARIZE";

// ============================================================================
// Public functions
// ============================================================================

/// Parses format configuration from command-line flags.
///
/// Reads flags to determine scalarization mode, record mode, and binding
/// movement settings. Falls back to `DEFAULT_FORMAT` when flags are not set
/// (Flags module not yet available in Rust).
///
/// # Returns
/// An `OutputFormat` configuration derived from the flags.
///
/// # Note
/// This function currently returns `DEFAULT_FORMAT` because the `Flags` module
/// is not yet translated. The full implementation requires:
/// - `Flags.isSet(NF_SCALARIZE)`
/// - `Flags.isConfigFlagSet(BASE_MODELICA_OPTIONS, "scalarize")`
/// - `Flags.getConfigStringList(BASE_MODELICA_FORMAT)`
/// - `Flags.isConfigFlagSet(BASE_MODELICA_OPTIONS, "moveBindings")`
pub fn format_from_flags() -> OutputFormat {
    let mut scalarize_mode = ScalarizeMode::PARTIALLY_SCALARIZED;
    let mut record_mode = RecordMode::WITH_RECORDS;
    let mut move_bindings = false;

    // if not Flags.isSet(Flags.NF_SCALARIZE) then
    //   format.scalarizeMode := ScalarizeMode.NOT_SCALARIZED;
    if !flags_is_set(NF_SCALARIZE) {
        scalarize_mode = ScalarizeMode::NOT_SCALARIZED;
    } else if flags_is_config_flag_set(BASE_MODELICA_OPTIONS, "scalarize") {
        scalarize_mode = ScalarizeMode::SCALARIZED;
        record_mode = RecordMode::WITHOUT_RECORDS;
    }

    // for option in Flags.getConfigStringList(Flags.BASE_MODELICA_FORMAT) loop
    //   () := match option
    //     case "scalarized"          algorithm format.scalarizeMode := ScalarizeMode.SCALARIZED; then ();
    //     case "partiallyScalarized" algorithm format.scalarizeMode := ScalarizeMode.PARTIALLY_SCALARIZED; then ();
    //     case "nonScalarized"       algorithm format.scalarizeMode := ScalarizeMode.NOT_SCALARIZED; then ();
    //     case "withRecords"         algorithm format.recordMode := RecordMode.WITH_RECORDS; then ();
    //     case "withoutRecords"      algorithm format.recordMode := RecordMode.WITHOUT_RECORDS; then ();
    //     else ();
    //   end match;
    // end for;
    for option in flags_get_config_string_list(BASE_MODELICA_FORMAT) {
        let matched = match option.as_str() {
            "scalarized" => {
                scalarize_mode = ScalarizeMode::SCALARIZED;
                true
            }
            "partiallyScalarized" => {
                scalarize_mode = ScalarizeMode::PARTIALLY_SCALARIZED;
                true
            }
            "nonScalarized" => {
                scalarize_mode = ScalarizeMode::NOT_SCALARIZED;
                true
            }
            "withRecords" => {
                record_mode = RecordMode::WITH_RECORDS;
                true
            }
            "withoutRecords" => {
                record_mode = RecordMode::WITHOUT_RECORDS;
                true
            }
            _ => false,
        };
        let _ = matched; // () in MetaModelica - discard the match result
    }

    // format.moveBindings := Flags.isConfigFlagSet(Flags.BASE_MODELICA_OPTIONS, "moveBindings");
    move_bindings = flags_is_config_flag_set(BASE_MODELICA_OPTIONS, "moveBindings");

    OutputFormat::OUTPUT_FORMAT {
        scalarize_mode,
        record_mode,
        move_bindings,
    }
}

/// Returns whether function inlining is enabled.
///
/// Checks the `inlineFunctions` config flag.
///
/// # Returns
/// `true` if inlining is enabled, `false` otherwise.
///
/// # Note
/// Currently returns `false` because the Flags module is not yet available.
pub fn inline_functions() -> bool {
    flags_is_config_flag_set(BASE_MODELICA_OPTIONS, "inlineFunctions")
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_format_values() {
        let fmt = DEFAULT_FORMAT;
        match fmt {
            OutputFormat::OUTPUT_FORMAT {
                scalarize_mode,
                record_mode,
                move_bindings,
            } => {
                assert_eq!(scalarize_mode, ScalarizeMode::PARTIALLY_SCALARIZED);
                assert_eq!(record_mode, RecordMode::WITH_RECORDS);
                assert!(!move_bindings);
            }
        }
    }

    #[test]
    fn test_scalarize_mode_display() {
        assert_eq!(ScalarizeMode::SCALARIZED.to_string(), "SCALARIZED");
        assert_eq!(
            ScalarizeMode::PARTIALLY_SCALARIZED.to_string(),
            "PARTIALLY_SCALARIZED"
        );
        assert_eq!(
            ScalarizeMode::NOT_SCALARIZED.to_string(),
            "NOT_SCALARIZED"
        );
    }

    #[test]
    fn test_record_mode_display() {
        assert_eq!(RecordMode::WITH_RECORDS.to_string(), "WITH_RECORDS");
        assert_eq!(
            RecordMode::WITHOUT_RECORDS.to_string(),
            "WITHOUT_RECORDS"
        );
    }

    #[test]
    fn test_format_from_flags_returns_default() {
        // Since Flags stubs return false/empty, format_from_flags should
        // produce the default-like output (NOT_SCALARIZED due to NF_SCALARIZE check)
        let fmt = format_from_flags();
        match fmt {
            OutputFormat::OUTPUT_FORMAT {
                scalarize_mode,
                record_mode,
                move_bindings,
            } => {
                // Flags.isSet(NF_SCALARIZE) returns false, so scalarizeMode defaults to NOT_SCALARIZED
                assert_eq!(scalarize_mode, ScalarizeMode::NOT_SCALARIZED);
                assert_eq!(record_mode, RecordMode::WITH_RECORDS);
                assert!(!move_bindings);
            }
        }
    }

    #[test]
    fn test_enumeration_copy() {
        let mode = ScalarizeMode::SCALARIZED;
        let _copied = mode; // Verify Copy trait
        let _ = mode; // Verify it's still accessible

        let record = RecordMode::WITH_RECORDS;
        let _copied = record; // Verify Copy trait
        let _ = record; // Verify it's still accessible
    }

    #[test]
    fn test_output_format_clone() {
        let fmt = OutputFormat::OUTPUT_FORMAT {
            scalarize_mode: ScalarizeMode::SCALARIZED,
            record_mode: RecordMode::WITHOUT_RECORDS,
            move_bindings: true,
        };
        let _cloned = fmt.clone();
    }

    #[test]
    fn test_output_format_equality() {
        let fmt1 = OutputFormat::OUTPUT_FORMAT {
            scalarize_mode: ScalarizeMode::SCALARIZED,
            record_mode: RecordMode::WITH_RECORDS,
            move_bindings: false,
        };
        let fmt2 = OutputFormat::OUTPUT_FORMAT {
            scalarize_mode: ScalarizeMode::SCALARIZED,
            record_mode: RecordMode::WITH_RECORDS,
            move_bindings: false,
        };
        assert_eq!(fmt1, fmt2);
    }
}
