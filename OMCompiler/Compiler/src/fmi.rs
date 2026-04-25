//! Translation of Util/FMI.mo
//!
//! This module provides FMI (Functional Mock-up Interface) specific types and functions
//! for parsing and validating FMI information from model descriptions. It defines
//! union types for FMI data structures and utility functions for version/type checking.
//!
//! # Assumptions
//! - `Flags.getConfigString(Flags.FMI_VERSION)` is used by `get_fmi_version_string`.
//!   Since the full Flags infrastructure requires runtime initialization, this function
//!   returns a default value. The real value would be "2.0" from the compiler configuration.
//! - `List.filter2OnTrue` and `List.filter` from the List package are used by
//!   `filter_model_variables`. The Rust equivalent uses `Iterator::filter`.
//! - `stringEqual` is a simple string equality check.
//! - `Option<Integer>` maps to `Option<i32>`.
//! - `list<Integer>` and `list<ModelVariables>` map to `Vec<i32>` and `Vec<ModelVariables>`.
//!
//! # Known issues
//! - `get_fmi_version_string` returns a hardcoded default since the Flags module
//!   requires runtime initialization not yet wired up.
//! - The `get_enumeration_type_from_types` recursive function with guards has been
//!   converted to an iterative search to avoid stack overflow on large type lists.

// ============================================================================
// Info - uniontype for FMI information
// ============================================================================

/// Represents FMI information parsed from a model description.
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Info {
    INFO {
        fmi_version: String,
        fmi_type: i32,
        fmi_model_name: String,
        fmi_model_identifier: String,
        fmi_guid: String,
        fmi_description: String,
        fmi_generation_tool: String,
        fmi_generation_date_and_time: String,
        fmi_variable_naming_convention: String,
        fmi_number_of_continuous_states: i32,
        fmi_number_of_event_indicators: i32,
    },
}

// ============================================================================
// TypeDefinitions - uniontype for type definitions
// ============================================================================

/// Uniontype for type definitions from FMI model descriptions.
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum TypeDefinitions {
    ENUMERATIONTYPE {
        name: String,
        description: String,
        quantity: String,
        min: i32,
        max: i32,
        items: Vec<EnumerationItem>,
    },
}

// ============================================================================
// EnumerationItem - uniontype for enumeration items
// ============================================================================

/// Uniontype for enumeration items.
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub struct EnumerationItem {
    pub name: String,
    pub description: String,
}

// ============================================================================
// ExperimentAnnotation - uniontype for experiment annotation
// ============================================================================

/// Uniontype for experiment annotation from FMI model descriptions.
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ExperimentAnnotation {
    EXPERIMENTANNOTATION {
        fmi_experiment_start_time: f64,
        fmi_experiment_stop_time: f64,
        fmi_experiment_tolerance: f64,
    },
}

// ============================================================================
// ModelVariables - uniontype for FMI model variables
// ============================================================================

/// Uniontype for FMI model variables (multiple variable kinds).
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ModelVariables {
    REALVARIABLE {
        instance: i32,
        name: String,
        description: String,
        base_type: String,
        variability: String,
        causality: String,
        has_start_value: bool,
        start_value: f64,
        is_fixed: bool,
        value_reference: f64,
        x1_placement: i32,
        x2_placement: i32,
        y1_placement: i32,
        y2_placement: i32,
    },
    INTEGERVARIABLE {
        instance: i32,
        name: String,
        description: String,
        base_type: String,
        variability: String,
        causality: String,
        has_start_value: bool,
        start_value: i32,
        is_fixed: bool,
        value_reference: f64,
        x1_placement: i32,
        x2_placement: i32,
        y1_placement: i32,
        y2_placement: i32,
    },
    BOOLEANVARIABLE {
        instance: i32,
        name: String,
        description: String,
        base_type: String,
        variability: String,
        causality: String,
        has_start_value: bool,
        start_value: bool,
        is_fixed: bool,
        value_reference: f64,
        x1_placement: i32,
        x2_placement: i32,
        y1_placement: i32,
        y2_placement: i32,
    },
    STRINGVARIABLE {
        instance: i32,
        name: String,
        description: String,
        base_type: String,
        variability: String,
        causality: String,
        has_start_value: bool,
        start_value: String,
        is_fixed: bool,
        value_reference: f64,
        x1_placement: i32,
        x2_placement: i32,
        y1_placement: i32,
        y2_placement: i32,
    },
    ENUMERATIONVARIABLE {
        instance: i32,
        name: String,
        description: String,
        base_type: String,
        variability: String,
        causality: String,
        has_start_value: bool,
        start_value: i32,
        is_fixed: bool,
        value_reference: f64,
        x1_placement: i32,
        x2_placement: i32,
        y1_placement: i32,
        y2_placement: i32,
    },
}

// ============================================================================
// FmiImport - uniontype for FMI import configuration
// ============================================================================

/// Uniontype for FMI import configuration.
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum FmiImport {
    FMIIMPORT {
        platform: String,
        fmu_file_name: String,
        fmu_working_directory: String,
        fmi_log_level: i32,
        fmi_debug_output: bool,
        fmi_context: Option<i32>,
        fmi_instance: Option<i32>,
        fmi_info: Info,
        fmi_type_definitions_list: Vec<TypeDefinitions>,
        fmi_experiment_annotation: ExperimentAnnotation,
        fmi_model_variables_instance: Option<i32>,
        fmi_model_variables_list: Vec<ModelVariables>,
        generate_input_connectors: bool,
        generate_output_connectors: bool,
    },
}

// ============================================================================
// Public functions
// ============================================================================

/// Returns the model identifier from FMI info.
///
/// Corresponds to: `getFMIModelIdentifier(inFMIInfo)`
///
/// # Parameters
/// * `in_fmi_info` - The FMI info to extract the model identifier from
///
/// # Returns
/// The `fmiModelIdentifier` string from the info record.
pub fn get_fmi_model_identifier(in_fmi_info: &Info) -> String {
    match in_fmi_info {
        Info::INFO { fmi_model_identifier, .. } => fmi_model_identifier.clone(),
    }
}

/// Returns the FMI type as a human-readable string.
///
/// Corresponds to: `getFMIType(inFMIInfo)`
///
/// Maps (version, type) pairs to string representations:
/// - ("1.0", 0) -> "me"
/// - ("1.0", 1) -> "cs_st"
/// - ("1.0", 2) -> "cs_tool"
/// - ("2.0", 1) -> "me"
/// - ("2.0", 2) -> "cs"
/// - ("2.0", 3) -> "me_cs"
///
/// # Parameters
/// * `in_fmi_info` - The FMI info containing version and type
///
/// # Returns
/// A string describing the FMI type.
pub fn get_fmi_type(in_fmi_info: &Info) -> String {
    match in_fmi_info {
        Info::INFO { fmi_version, fmi_type, .. } => match (fmi_version.as_str(), fmi_type) {
            ("1.0", 0) => "me".to_string(),
            ("1.0", 1) => "cs_st".to_string(),
            ("1.0", 2) => "cs_tool".to_string(),
            ("2.0", 1) => "me".to_string(),
            ("2.0", 2) => "cs".to_string(),
            ("2.0", 3) => "me_cs".to_string(),
            _ => String::new(),
        },
    }
}

/// Returns the FMI version from FMI info.
///
/// Corresponds to: `getFMIVersion(inFMIInfo)`
///
/// # Parameters
/// * `in_fmi_info` - The FMI info to extract the version from
///
/// # Returns
/// The `fmiVersion` string from the info record.
pub fn get_fmi_version(in_fmi_info: &Info) -> String {
    match in_fmi_info {
        Info::INFO { fmi_version, .. } => fmi_version.clone(),
    }
}

/// Checks if the FMU version is supported.
///
/// Corresponds to: `checkFMIVersion(inFMIVersion)`
/// Supported versions: "1.0", "2.0"
///
/// # Parameters
/// * `in_fmi_version` - The version string to check
///
/// # Returns
/// `true` if the version is supported, `false` otherwise.
pub fn check_fmi_version(in_fmi_version: &str) -> bool {
    match in_fmi_version {
        "1.0" | "2.0" => true,
        _ => false,
    }
}

/// Checks if the FMI version is 1.0.
///
/// Corresponds to: `isFMIVersion10(inFMUVersion)`
///
/// # Parameters
/// * `in_fmu_version` - The version string to check
///
/// # Returns
/// `true` if the version is "1.0", `false` otherwise.
pub fn is_fmi_version_10(in_fmu_version: &str) -> bool {
    in_fmu_version == "1.0"
}

/// Checks if the FMI version is 2.0.
///
/// Corresponds to: `isFMIVersion20(inFMUVersion)`
/// Default value for `inFMUVersion` is `getFMIVersionString()`.
///
/// # Parameters
/// * `in_fmu_version` - The version string to check (defaults to configured FMI version)
///
/// # Returns
/// `true` if the version is "2.0", `false` otherwise.
pub fn is_fmi_version_20(in_fmu_version: &str) -> bool {
    in_fmu_version == "2.0"
}

/// Returns the FMI version string from the compiler configuration.
///
/// Corresponds to: `getFMIVersionString()`
/// Reads `Flags.getConfigString(Flags.FMI_VERSION)`.
///
/// # Returns
/// The configured FMI version string.
/// Returns "2.0" as default since the full Flags infrastructure requires runtime initialization.
pub fn get_fmi_version_string() -> String {
    // Stub: the real implementation reads Flags.FMI_VERSION
    // Since flags require runtime initialization, return the default
    String::from("2.0")
}

/// Checks if the FMU type is supported.
///
/// Corresponds to: `checkFMIType(inFMIType)`
/// Supported types: "me", "cs", "me_cs"
///
/// # Parameters
/// * `in_fmi_type` - The FMI type string to check
///
/// # Returns
/// `true` if the type is supported, `false` otherwise.
pub fn check_fmi_type(in_fmi_type: &str) -> bool {
    matches!(in_fmi_type, "me" | "cs" | "me_cs")
}

/// Checks if FMU export is possible for the given version and type.
///
/// Corresponds to: `canExportFMU(inFMUVersion, inFMIType)`
/// Supported combinations:
/// - ("1.0", "me")
/// - ("2.0", "me"), ("2.0", "cs"), ("2.0", "me_cs")
///
/// # Parameters
/// * `in_fmu_version` - The FMU version string
/// * `in_fmi_type` - The FMI type string
///
/// # Returns
/// `true` if export is possible, `false` otherwise.
pub fn can_export_fmu(in_fmu_version: &str, in_fmi_type: &str) -> bool {
    matches!((in_fmu_version, in_fmi_type),
        ("1.0", "me")
        | ("2.0", "me")
        | ("2.0", "cs")
        | ("2.0", "me_cs")
    )
}

/// Checks if the FMU type is model exchange.
///
/// Corresponds to: `isFMIMEType(inFMIType)`
/// Matches: "me", "me_cs"
///
/// # Parameters
/// * `in_fmi_type` - The FMI type string to check
///
/// # Returns
/// `true` if the type supports model exchange, `false` otherwise.
pub fn is_fmi_mime_type(in_fmi_type: &str) -> bool {
    matches!(in_fmi_type, "me" | "me_cs")
}

/// Checks if the FMU type is co-simulation.
///
/// Corresponds to: `isFMICSType(inFMIType)`
/// Matches: "cs", "me_cs"
///
/// # Parameters
/// * `in_fmi_type` - The FMI type string to check
///
/// # Returns
/// `true` if the type supports co-simulation, `false` otherwise.
pub fn is_fmi_cs_type(in_fmi_type: &str) -> bool {
    matches!(in_fmi_type, "cs" | "me_cs")
}

/// Finds an enumeration type by name from the type definitions list.
///
/// Corresponds to: `getEnumerationTypeFromTypes(inTypeDefinitionsList, inBaseType)`
///
/// Searches through the type definitions list for an enumeration type whose name
/// matches `in_base_type`. Uses recursive iteration (converted from MO's recursive
/// function with guards) to handle potentially large lists without stack overflow.
///
/// # Parameters
/// * `in_type_definitions_list` - The list of type definitions to search
/// * `in_base_type` - The base type name to look for
///
/// # Returns
/// The matching type name, or empty string if not found.
pub fn get_enumeration_type_from_types(
    in_type_definitions_list: &[TypeDefinitions],
    in_base_type: &str,
) -> String {
    for type_def in in_type_definitions_list {
        let TypeDefinitions::ENUMERATIONTYPE { name, .. } = type_def;
        if name == in_base_type {
            return name.clone();
        }
    }
    String::new()
}

/// Filters model variables by type and causality.
///
/// Corresponds to: `filterModelVariables(inModelVariables, tipe, variableCausality)`
/// Uses `List.filter2OnTrue` internally, translated to Rust's `Iterator::filter`.
///
/// # Parameters
/// * `in_model_variables` - The list of model variables to filter
/// * `tipe` - The variable type to filter by ("real", "integer", "boolean", "string", "enumeration")
/// * `variable_causality` - The causality to filter by (e.g., "parameter", "output", "input")
///
/// # Returns
/// A filtered list of model variables matching both the type and causality criteria.
pub fn filter_model_variables(
    in_model_variables: &[ModelVariables],
    tipe: &str,
    variable_causality: &str,
) -> Vec<ModelVariables> {
    in_model_variables
        .iter()
        .filter(|v| filter_model_variable(v, tipe, variable_causality))
        .cloned()
        .collect()
}

/// Protected function: checks if a single model variable matches type and causality criteria.
///
/// Corresponds to: `filterModelVariable(modelVar, tipe, variableCausality)`
///
/// # Parameters
/// * `model_var` - The model variable to check
/// * `tipe` - The variable type ("real", "integer", "boolean", "string")
/// * `variable_causality` - The expected causality
///
/// # Returns
/// `true` if the variable matches both the type and causality, `false` otherwise.
pub fn filter_model_variable(model_var: &ModelVariables, tipe: &str, variable_causality: &str) -> bool {
    let causality = match model_var {
        ModelVariables::REALVARIABLE { causality, .. } if tipe == "real" => Some(causality),
        ModelVariables::INTEGERVARIABLE { causality, .. } if tipe == "integer" => Some(causality),
        ModelVariables::BOOLEANVARIABLE { causality, .. } if tipe == "boolean" => Some(causality),
        ModelVariables::STRINGVARIABLE { causality, .. } if tipe == "string" => Some(causality),
        _ => None,
    };
    match causality {
        Some(c) => c == variable_causality,
        None => false,
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn make_info(version: &str, fmi_type: i32, model_id: &str) -> Info {
        Info::INFO {
            fmi_version: version.to_string(),
            fmi_type,
            fmi_model_name: String::new(),
            fmi_model_identifier: model_id.to_string(),
            fmi_guid: String::new(),
            fmi_description: String::new(),
            fmi_generation_tool: String::new(),
            fmi_generation_date_and_time: String::new(),
            fmi_variable_naming_convention: String::new(),
            fmi_number_of_continuous_states: 0,
            fmi_number_of_event_indicators: 0,
        }
    }

    #[test]
    fn test_get_fmi_model_identifier() {
        let info = make_info("2.0", 1, "MyModel");
        assert_eq!(get_fmi_model_identifier(&info), "MyModel");
    }

    #[test]
    fn test_get_fmi_version() {
        let info = make_info("1.0", 0, "Test");
        assert_eq!(get_fmi_version(&info), "1.0");
        let info2 = make_info("2.0", 2, "Test");
        assert_eq!(get_fmi_version(&info2), "2.0");
    }

    #[test]
    fn test_get_fmi_type() {
        assert_eq!(get_fmi_type(&make_info("1.0", 0, "")), "me");
        assert_eq!(get_fmi_type(&make_info("1.0", 1, "")), "cs_st");
        assert_eq!(get_fmi_type(&make_info("1.0", 2, "")), "cs_tool");
        assert_eq!(get_fmi_type(&make_info("2.0", 1, "")), "me");
        assert_eq!(get_fmi_type(&make_info("2.0", 2, "")), "cs");
        assert_eq!(get_fmi_type(&make_info("2.0", 3, "")), "me_cs");
    }

    #[test]
    fn test_check_fmi_version() {
        assert!(check_fmi_version("1.0"));
        assert!(check_fmi_version("2.0"));
        assert!(!check_fmi_version("3.0"));
        assert!(!check_fmi_version("1.1"));
    }

    #[test]
    fn test_is_fmi_version_10() {
        assert!(is_fmi_version_10("1.0"));
        assert!(!is_fmi_version_10("2.0"));
    }

    #[test]
    fn test_is_fmi_version_20() {
        assert!(is_fmi_version_20("2.0"));
        assert!(!is_fmi_version_20("1.0"));
    }

    #[test]
    fn test_get_fmi_version_string() {
        assert_eq!(get_fmi_version_string(), "2.0");
    }

    #[test]
    fn test_check_fmi_type() {
        assert!(check_fmi_type("me"));
        assert!(check_fmi_type("cs"));
        assert!(check_fmi_type("me_cs"));
        assert!(!check_fmi_type("invalid"));
    }

    #[test]
    fn test_can_export_fmu() {
        assert!(can_export_fmu("1.0", "me"));
        assert!(can_export_fmu("2.0", "me"));
        assert!(can_export_fmu("2.0", "cs"));
        assert!(can_export_fmu("2.0", "me_cs"));
        assert!(!can_export_fmu("1.0", "cs"));
        assert!(!can_export_fmu("3.0", "me"));
    }

    #[test]
    fn test_is_fmi_mime_type() {
        assert!(is_fmi_mime_type("me"));
        assert!(is_fmi_mime_type("me_cs"));
        assert!(!is_fmi_mime_type("cs"));
    }

    #[test]
    fn test_is_fmi_cs_type() {
        assert!(is_fmi_cs_type("cs"));
        assert!(is_fmi_cs_type("me_cs"));
        assert!(!is_fmi_cs_type("me"));
    }

    #[test]
    fn test_get_enumeration_type_from_types() {
        let types = vec![
            TypeDefinitions::ENUMERATIONTYPE {
                name: "Color".to_string(),
                description: String::new(),
                quantity: String::new(),
                min: 0,
                max: 2,
                items: vec![
                    EnumerationItem {
                        name: "Red".to_string(),
                        description: String::new(),
                    },
                    EnumerationItem {
                        name: "Green".to_string(),
                        description: String::new(),
                    },
                ],
            },
            TypeDefinitions::ENUMERATIONTYPE {
                name: "Size".to_string(),
                description: String::new(),
                quantity: String::new(),
                min: 0,
                max: 2,
                items: vec![],
            },
        ];
        assert_eq!(get_enumeration_type_from_types(&types, "Color"), "Color");
        assert_eq!(get_enumeration_type_from_types(&types, "Size"), "Size");
        assert_eq!(get_enumeration_type_from_types(&types, "NotFound"), "");
    }

    #[test]
    fn test_get_enumeration_type_from_types_empty() {
        let types: Vec<TypeDefinitions> = vec![];
        assert_eq!(get_enumeration_type_from_types(&types, "Any"), "");
    }

    #[test]
    fn test_filter_model_variable_real() {
        let real_var = ModelVariables::REALVARIABLE {
            instance: 1,
            name: "x".to_string(),
            description: String::new(),
            base_type: "Real".to_string(),
            variability: "parameter".to_string(),
            causality: "input".to_string(),
            has_start_value: true,
            start_value: 0.0,
            is_fixed: false,
            value_reference: 0.0,
            x1_placement: 0,
            x2_placement: 0,
            y1_placement: 0,
            y2_placement: 0,
        };
        assert!(filter_model_variable(&real_var, "real", "input"));
        assert!(!filter_model_variable(&real_var, "real", "output"));
        assert!(!filter_model_variable(&real_var, "integer", "input"));
    }

    #[test]
    fn test_filter_model_variable_integer() {
        let int_var = ModelVariables::INTEGERVARIABLE {
            instance: 2,
            name: "n".to_string(),
            description: String::new(),
            base_type: "Integer".to_string(),
            variability: "parameter".to_string(),
            causality: "parameter".to_string(),
            has_start_value: true,
            start_value: 10,
            is_fixed: false,
            value_reference: 1.0,
            x1_placement: 0,
            x2_placement: 0,
            y1_placement: 0,
            y2_placement: 0,
        };
        assert!(filter_model_variable(&int_var, "integer", "parameter"));
        assert!(!filter_model_variable(&int_var, "real", "parameter"));
    }

    #[test]
    fn test_filter_model_variable_boolean() {
        let bool_var = ModelVariables::BOOLEANVARIABLE {
            instance: 3,
            name: "flag".to_string(),
            description: String::new(),
            base_type: "Boolean".to_string(),
            variability: "parameter".to_string(),
            causality: "output".to_string(),
            has_start_value: false,
            start_value: false,
            is_fixed: false,
            value_reference: 2.0,
            x1_placement: 0,
            x2_placement: 0,
            y1_placement: 0,
            y2_placement: 0,
        };
        assert!(filter_model_variable(&bool_var, "boolean", "output"));
        assert!(!filter_model_variable(&bool_var, "boolean", "input"));
    }

    #[test]
    fn test_filter_model_variable_string() {
        let str_var = ModelVariables::STRINGVARIABLE {
            instance: 4,
            name: "label".to_string(),
            description: String::new(),
            base_type: "String".to_string(),
            variability: "parameter".to_string(),
            causality: "output".to_string(),
            has_start_value: false,
            start_value: "test".to_string(),
            is_fixed: false,
            value_reference: 3.0,
            x1_placement: 0,
            x2_placement: 0,
            y1_placement: 0,
            y2_placement: 0,
        };
        assert!(filter_model_variable(&str_var, "string", "output"));
        assert!(!filter_model_variable(&str_var, "string", "input"));
    }

    #[test]
    fn test_filter_model_variables() {
        let vars = vec![
            ModelVariables::REALVARIABLE {
                instance: 1,
                name: "x".to_string(),
                description: String::new(),
                base_type: "Real".to_string(),
                variability: "parameter".to_string(),
                causality: "input".to_string(),
                has_start_value: true,
                start_value: 1.0,
                is_fixed: false,
                value_reference: 0.0,
                x1_placement: 0, x2_placement: 0, y1_placement: 0, y2_placement: 0,
            },
            ModelVariables::REALVARIABLE {
                instance: 2,
                name: "y".to_string(),
                description: String::new(),
                base_type: "Real".to_string(),
                variability: "parameter".to_string(),
                causality: "output".to_string(),
                has_start_value: true,
                start_value: 2.0,
                is_fixed: false,
                value_reference: 1.0,
                x1_placement: 0, x2_placement: 0, y1_placement: 0, y2_placement: 0,
            },
            ModelVariables::INTEGERVARIABLE {
                instance: 3,
                name: "n".to_string(),
                description: String::new(),
                base_type: "Integer".to_string(),
                variability: "parameter".to_string(),
                causality: "input".to_string(),
                has_start_value: true,
                start_value: 5,
                is_fixed: false,
                value_reference: 2.0,
                x1_placement: 0, x2_placement: 0, y1_placement: 0, y2_placement: 0,
            },
        ];
        let result = filter_model_variables(&vars, "real", "input");
        assert_eq!(result.len(), 1);
        match &result[0] {
            ModelVariables::REALVARIABLE { name, .. } => assert_eq!(name, "x"),
            _ => panic!("Expected REALVARIABLE"),
        }

        let result = filter_model_variables(&vars, "real", "output");
        assert_eq!(result.len(), 1);
        match &result[0] {
            ModelVariables::REALVARIABLE { name, .. } => assert_eq!(name, "y"),
            _ => panic!("Expected REALVARIABLE"),
        }
    }

    #[test]
    fn test_fmi_import_type() {
        // Verify the FmiImport enum compiles with all fields
        let _import = FmiImport::FMIIMPORT {
            platform: "Linux".to_string(),
            fmu_file_name: "test.fmu".to_string(),
            fmu_working_directory: ".".to_string(),
            fmi_log_level: 0,
            fmi_debug_output: false,
            fmi_context: Some(1),
            fmi_instance: Some(2),
            fmi_info: make_info("2.0", 1, "Test"),
            fmi_type_definitions_list: vec![],
            fmi_experiment_annotation: ExperimentAnnotation::EXPERIMENTANNOTATION {
                fmi_experiment_start_time: 0.0,
                fmi_experiment_stop_time: 10.0,
                fmi_experiment_tolerance: 1e-6,
            },
            fmi_model_variables_instance: Some(0),
            fmi_model_variables_list: vec![],
            generate_input_connectors: false,
            generate_output_connectors: false,
        };
    }

    #[test]
    fn test_model_variables_all_variants() {
        // Verify all ModelVariables variants compile and have correct types
        let real = ModelVariables::REALVARIABLE {
            instance: 1, name: "r".to_string(), description: String::new(),
            base_type: "Real".to_string(), variability: "parameter".to_string(),
            causality: "input".to_string(), has_start_value: true,
            start_value: 1.0_f64, is_fixed: false, value_reference: 0.0,
            x1_placement: 0, x2_placement: 0, y1_placement: 0, y2_placement: 0,
        };
        assert!(matches!(&real, ModelVariables::REALVARIABLE { .. }));

        let int = ModelVariables::INTEGERVARIABLE {
            instance: 2, name: "i".to_string(), description: String::new(),
            base_type: "Integer".to_string(), variability: "parameter".to_string(),
            causality: "parameter".to_string(), has_start_value: true,
            start_value: 10_i32, is_fixed: false, value_reference: 1.0,
            x1_placement: 0, x2_placement: 0, y1_placement: 0, y2_placement: 0,
        };
        assert!(matches!(&int, ModelVariables::INTEGERVARIABLE { .. }));

        let boolean = ModelVariables::BOOLEANVARIABLE {
            instance: 3, name: "b".to_string(), description: String::new(),
            base_type: "Boolean".to_string(), variability: "parameter".to_string(),
            causality: "output".to_string(), has_start_value: false,
            start_value: false, is_fixed: false, value_reference: 2.0,
            x1_placement: 0, x2_placement: 0, y1_placement: 0, y2_placement: 0,
        };
        assert!(matches!(&boolean, ModelVariables::BOOLEANVARIABLE { .. }));

        let string = ModelVariables::STRINGVARIABLE {
            instance: 4, name: "s".to_string(), description: String::new(),
            base_type: "String".to_string(), variability: "parameter".to_string(),
            causality: "output".to_string(), has_start_value: false,
            start_value: "hello".to_string(), is_fixed: false, value_reference: 3.0,
            x1_placement: 0, x2_placement: 0, y1_placement: 0, y2_placement: 0,
        };
        assert!(matches!(&string, ModelVariables::STRINGVARIABLE { .. }));

        let enumeration = ModelVariables::ENUMERATIONVARIABLE {
            instance: 5, name: "e".to_string(), description: String::new(),
            base_type: "Enumeration".to_string(), variability: "parameter".to_string(),
            causality: "parameter".to_string(), has_start_value: true,
            start_value: 0, is_fixed: false, value_reference: 4.0,
            x1_placement: 0, x2_placement: 0, y1_placement: 0, y2_placement: 0,
        };
        assert!(matches!(&enumeration, ModelVariables::ENUMERATIONVARIABLE { .. }));
    }

    #[test]
    fn test_info_type_clone_and_partial_eq() {
        let info1 = make_info("2.0", 1, "Test");
        let info2 = info1.clone();
        assert_eq!(info1, info2);
    }

    #[test]
    fn test_type_definitions_clone_and_partial_eq() {
        let td1 = TypeDefinitions::ENUMERATIONTYPE {
            name: "Color".to_string(),
            description: String::new(),
            quantity: String::new(),
            min: 0,
            max: 2,
            items: vec![
                EnumerationItem {
                    name: "Red".to_string(),
                    description: String::new(),
                },
            ],
        };
        let td2 = td1.clone();
        assert_eq!(td1, td2);
    }
}
