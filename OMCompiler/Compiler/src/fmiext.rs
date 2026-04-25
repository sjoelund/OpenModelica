//! Translation of Util/FMIExt.mo
//!
//! This module provides FMI import-specific functions that wrap C implementations
//! from the OpenModelica runtime (`omcbackendruntime`, `omcruntime`, `fmilib`).
//! It provides functions for initializing and releasing FMI imports.

use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_void};

// ============================================================================
// FMI Types (from FMI.mo)
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

/// Uniontype for enumeration items.
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub struct EnumerationItem {
    pub name: String,
    pub description: String,
}

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
// Extern C declarations
// ============================================================================

// Direct extern "C" declaration for `FMIImpl__initializeFMIImport`.
// See `initialize_fmi_import` for the safe wrapper.
unsafe extern "C" {
    fn FMIImpl__initializeFMIImport(
        in_file_name: *const c_char,
        in_working_directory: *const c_char,
        in_fmi_log_level: c_int,
        in_input_connectors: c_int,
        in_output_connectors: c_int,
        in_is_model_description_import: c_int,
        out_fmi_context: *mut c_int,
        out_fmi_instance: *mut c_int,
        out_fmi_info: *mut c_void,
        out_type_definitions_list: *mut c_void,
        out_experiment_annotation: *mut c_void,
        out_model_variables_instance: *mut c_int,
        out_model_variables_list: *mut c_void,
    ) -> c_int;
}

// Direct extern "C" declaration for `FMIImpl__releaseFMIImport`.
// See `release_fmi_import` for the safe wrapper.
unsafe extern "C" {
    fn FMIImpl__releaseFMIImport(
        in_fmi_model_variables_instance: c_int,
        in_fmi_instance: c_int,
        in_fmi_context: c_int,
        in_fmi_version: *const c_char,
    );
}

// ============================================================================
// Public wrapper functions
// ============================================================================

/// FMI initialization result matching the MO interface.
#[derive(Debug, Clone)]
pub struct InitializeFMIImportResult {
    pub result: bool,
    pub out_fmi_context: Option<i32>,
    pub out_fmi_instance: Option<i32>,
    pub out_fmi_info: Info,
    pub out_type_definitions_list: Vec<TypeDefinitions>,
    pub out_experiment_annotation: ExperimentAnnotation,
    pub out_model_variables_instance: Option<i32>,
    pub out_model_variables_list: Vec<ModelVariables>,
}

/// Initialize FMI import from a file.
///
/// Wraps the C function `FMIImpl__initializeFMIImport` from the OpenModelica runtime.
///
/// # Safety
///
/// - `in_file_name` must point to a valid, null-terminated C string.
/// - `in_working_directory` must point to a valid, null-terminated C string.
/// - The caller must ensure the C library (`omcbackendruntime`, `omcruntime`, `fmilib`) is linked.
pub unsafe fn initialize_fmi_import(
    in_file_name: *const c_char,
    in_working_directory: *const c_char,
    in_fmi_log_level: i32,
    in_input_connectors: bool,
    in_output_connectors: bool,
    in_is_model_description_import: bool,
) -> InitializeFMIImportResult {
    let mut out_fmi_context: c_int = 0;
    let mut out_fmi_instance: c_int = 0;
    let mut out_fmi_info: *mut c_void = std::ptr::null_mut();
    let mut out_type_defs: *mut c_void = std::ptr::null_mut();
    let mut out_exp_annot: *mut c_void = std::ptr::null_mut();
    let mut out_mv_instance: c_int = 0;
    let mut out_mv_list: *mut c_void = std::ptr::null_mut();

    let result = unsafe {
        FMIImpl__initializeFMIImport(
            in_file_name,
            in_working_directory,
            in_fmi_log_level,
            if in_input_connectors { 1 } else { 0 },
            if in_output_connectors { 1 } else { 0 },
            if in_is_model_description_import { 1 } else { 0 },
            &mut out_fmi_context,
            &mut out_fmi_instance,
            &mut out_fmi_info as *mut *mut c_void as *mut c_void,
            &mut out_type_defs as *mut *mut c_void as *mut c_void,
            &mut out_exp_annot as *mut *mut c_void as *mut c_void,
            &mut out_mv_instance,
            &mut out_mv_list as *mut *mut c_void as *mut c_void,
        )
    };

    let out_fmi_context = if out_fmi_context != 0 {
        Some(out_fmi_context)
    } else {
        None
    };

    let out_fmi_instance = if out_fmi_instance != 0 {
        Some(out_fmi_instance)
    } else {
        None
    };

    let out_model_variables_instance = if out_mv_instance != 0 {
        Some(out_mv_instance)
    } else {
        None
    };

    let result_bool = result != 0;

    // The complex types (Info, type definitions list, experiment annotation,
    // model variables list) are opaque pointer outputs from the C side.
    // In a complete translation, these would be deserialized from C structs.
    // For now, we return default values as placeholders.
    let fmi_info = Info::INFO {
        fmi_version: String::new(),
        fmi_type: 0,
        fmi_model_name: String::new(),
        fmi_model_identifier: String::new(),
        fmi_guid: String::new(),
        fmi_description: String::new(),
        fmi_generation_tool: String::new(),
        fmi_generation_date_and_time: String::new(),
        fmi_variable_naming_convention: String::new(),
        fmi_number_of_continuous_states: 0,
        fmi_number_of_event_indicators: 0,
    };

    let exp_annot = ExperimentAnnotation::EXPERIMENTANNOTATION {
        fmi_experiment_start_time: 0.0,
        fmi_experiment_stop_time: 0.0,
        fmi_experiment_tolerance: 0.0,
    };

    InitializeFMIImportResult {
        result: result_bool,
        out_fmi_context,
        out_fmi_instance,
        out_fmi_info: fmi_info,
        out_type_definitions_list: Vec::new(),
        out_experiment_annotation: exp_annot,
        out_model_variables_instance,
        out_model_variables_list: Vec::new(),
    }
}

/// Release FMI import resources.
///
/// Wraps the C function `FMIImpl__releaseFMIImport` from the OpenModelica runtime.
///
/// # Safety
///
/// - The caller must ensure the C library (`omcbackendruntime`, `omcruntime`, `fmilib`) is linked.
/// - The `in_fmi_version` must point to a valid, null-terminated C string.
pub unsafe fn release_fmi_import(
    in_fmi_model_variables_instance: Option<i32>,
    in_fmi_instance: Option<i32>,
    in_fmi_context: Option<i32>,
    in_fmi_version: *const c_char,
) {
    let mv_inst = in_fmi_model_variables_instance.unwrap_or(0);
    let fmi_inst = in_fmi_instance.unwrap_or(0);
    let fmi_ctx = in_fmi_context.unwrap_or(0);
    unsafe {
        FMIImpl__releaseFMIImport(mv_inst, fmi_inst, fmi_ctx, in_fmi_version);
    }
}

/// Initialize FMI import from a file (String-based convenience wrapper).
pub fn initialize_fmi_import_str(
    in_file_name: &str,
    in_working_directory: &str,
    in_fmi_log_level: i32,
    in_input_connectors: bool,
    in_output_connectors: bool,
    in_is_model_description_import: bool,
) -> InitializeFMIImportResult {
    let file_name_c = CString::new(in_file_name).unwrap_or_default();
    let working_dir_c = CString::new(in_working_directory).unwrap_or_default();

    unsafe {
        initialize_fmi_import(
            file_name_c.as_ptr(),
            working_dir_c.as_ptr(),
            in_fmi_log_level,
            in_input_connectors,
            in_output_connectors,
            in_is_model_description_import,
        )
    }
}

/// Release FMI import resources (String-based convenience wrapper).
pub fn release_fmi_import_str(
    in_fmi_model_variables_instance: Option<i32>,
    in_fmi_instance: Option<i32>,
    in_fmi_context: Option<i32>,
    in_fmi_version: &str,
) {
    let version_c = CString::new(in_fmi_version).ok();
    if let Some(c_str) = version_c {
        unsafe {
            release_fmi_import(
                in_fmi_model_variables_instance,
                in_fmi_instance,
                in_fmi_context,
                c_str.as_ptr(),
            );
        }
    }
}
