//! Translation of Script/CevalScriptOMSimulator.mo
//!
//! This module provides a function dispatch mechanism for calling OpenModelica
//! Simulator operations through a unified interface. It receives a function name
//! and a list of Value arguments, then dispatches to the appropriate wrapper
//! function in the om_simoulator module.
//!
//! Input:
//!   - Function name (String)
//!   - Arguments (list of Values.Value)
//! Output:
//!   - Result Value

use crate::values::Value;
use crate::omsimulatorext as om_simoulator;
use anyhow::{Result, bail};
use im::Vector;

/// Helper: extract a STRING value from a Value.
fn unwrap_string<'a>(v: &'a Value) -> Option<&'a str> {
    match v {
        Value::STRING { string } => Some(string.as_str()),
        _ => None,
    }
}

/// Helper: extract an INTEGER value from a Value.
fn unwrap_integer(v: &Value) -> Option<i32> {
    match v {
        Value::INTEGER { integer } => Some(*integer),
        _ => None,
    }
}

/// Helper: extract a REAL value from a Value.
fn unwrap_real(v: &Value) -> Option<f64> {
    match v {
        Value::REAL { real } => Some(*real),
        _ => None,
    }
}

/// Helper: extract a BOOL value from a Value.
fn unwrap_bool(v: &Value) -> Option<bool> {
    match v {
        Value::BOOL { boolean } => Some(*boolean),
        _ => None,
    }
}

/// Helper: extract the index from an ENUM_LITERAL value.
fn unwrap_enum_index(v: &Value) -> Option<i32> {
    match v {
        Value::ENUM_LITERAL { index, .. } => Some(*index),
        _ => None,
    }
}

/// Helper: create a TUPLE Value from a list of Values.
fn make_tuple(vals: Vec<Value>) -> Value {
    Value::TUPLE {
        value_lst: vals.into_iter().collect(),
    }
}

/// Helper: create an INTEGER Value from an i32.
fn make_integer(v: i32) -> Value {
    Value::INTEGER { integer: v }
}

/// Helper: create a STRING Value from a &str.
fn make_string(v: impl Into<String>) -> Value {
    Value::STRING { string: v.into() }
}

/// Main dispatch function: routes function name + args to the appropriate operation.
/// Translated from the ceval function in CevalScriptOMSimulator.mo.
///
/// # Arguments
/// * `function_name` - The name of the function to call
/// * `args` - The list of argument Values
///
/// # Returns
/// The result as a Value, or a TUPLE of values for multi-return functions.
pub fn ceval(function_name: String, args: Vector<Value>) -> Result<Value> {
    let args = &args;

    macro_rules! arg {
        ($idx:expr, $unwrap:expr, $expected:expr) => {{
            let val = args
                .get($idx)
                .ok_or_else(|| anyhow::anyhow!("{}: expected argument at index {}", function_name, $idx))?;
            $unwrap(val)
                .ok_or_else(|| anyhow::anyhow!("{}: expected {} argument", function_name, $expected))?
        }};
    }

    match function_name.as_str() {
        "loadOMSimulator" => {
            ensure_arg_count(args, 0, "loadOMSimulator")?;
            Ok(make_integer(om_simoulator::load_om_simulator()))
        }

        "unloadOMSimulator" => {
            ensure_arg_count(args, 0, "unloadOMSimulator")?;
            Ok(make_integer(om_simoulator::unload_om_simulator()))
        }

        "oms_addBus" => {
            ensure_arg_count(args, 1, "oms_addBus")?;
            let cref = arg!(0, unwrap_string, "STRING");
            Ok(make_integer(om_simoulator::oms_add_bus(cref)))
        }

        "oms_addConnection" => {
            ensure_arg_count(args, 2, "oms_addConnection")?;
            let a = arg!(0, unwrap_string, "STRING");
            let b = arg!(1, unwrap_string, "STRING");
            Ok(make_integer(om_simoulator::oms_add_connection(a, b)))
        }

        "oms_addConnector" => {
            ensure_arg_count(args, 3, "oms_addConnector")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let causality = arg!(1, unwrap_enum_index, "ENUM_LITERAL") - 1;
            let type_ = arg!(2, unwrap_enum_index, "ENUM_LITERAL") - 1;
            Ok(make_integer(om_simoulator::oms_add_connector(cref, causality, type_)))
        }

        "oms_addConnectorToBus" => {
            ensure_arg_count(args, 2, "oms_addConnectorToBus")?;
            let bus = arg!(0, unwrap_string, "STRING");
            let conn = arg!(1, unwrap_string, "STRING");
            Ok(make_integer(om_simoulator::oms_add_connector_to_bus(bus, conn)))
        }

        "oms_addConnectorToTLMBus" => {
            ensure_arg_count(args, 3, "oms_addConnectorToTLMBus")?;
            let bus = arg!(0, unwrap_string, "STRING");
            let conn = arg!(1, unwrap_string, "STRING");
            let stype = arg!(2, unwrap_string, "STRING");
            Ok(make_integer(om_simoulator::oms_add_connector_to_tlm_bus(bus, conn, stype)))
        }

        "oms_addDynamicValueIndicator" => {
            ensure_arg_count(args, 4, "oms_addDynamicValueIndicator")?;
            let signal = arg!(0, unwrap_string, "STRING");
            let s_lower = arg!(1, unwrap_string, "STRING");
            let s_upper = arg!(2, unwrap_string, "STRING");
            let step_size = arg!(3, unwrap_real, "REAL");
            Ok(make_integer(om_simoulator::oms_add_dynamic_value_indicator(signal, s_lower, s_upper, step_size)))
        }

        "oms_addEventIndicator" => {
            ensure_arg_count(args, 1, "oms_addEventIndicator")?;
            let signal = arg!(0, unwrap_string, "STRING");
            Ok(make_integer(om_simoulator::oms_add_event_indicator(signal)))
        }

        "oms_addExternalModel" => {
            ensure_arg_count(args, 3, "oms_addExternalModel")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let path = arg!(1, unwrap_string, "STRING");
            let startscript = arg!(2, unwrap_string, "STRING");
            Ok(make_integer(om_simoulator::oms_add_external_model(cref, path, startscript)))
        }

        "oms_addSignalsToResults" => {
            ensure_arg_count(args, 2, "oms_addSignalsToResults")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let regex = arg!(1, unwrap_string, "STRING");
            Ok(make_integer(om_simoulator::oms_add_signals_to_results(cref, regex)))
        }

        "oms_addStaticValueIndicator" => {
            ensure_arg_count(args, 4, "oms_addStaticValueIndicator")?;
            let signal = arg!(0, unwrap_string, "STRING");
            let lower = arg!(1, unwrap_real, "REAL");
            let upper = arg!(2, unwrap_real, "REAL");
            let step_size = arg!(3, unwrap_real, "REAL");
            Ok(make_integer(om_simoulator::oms_add_static_value_indicator(signal, lower, upper, step_size)))
        }

        "oms_addSubModel" => {
            ensure_arg_count(args, 2, "oms_addSubModel")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let fmu_path = arg!(1, unwrap_string, "STRING");
            Ok(make_integer(om_simoulator::oms_add_sub_model(cref, fmu_path)))
        }

        "oms_addSystem" => {
            ensure_arg_count(args, 2, "oms_addSystem")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let type_ = arg!(1, unwrap_enum_index, "ENUM_LITERAL") - 1;
            Ok(make_integer(om_simoulator::oms_add_system(cref, type_)))
        }

        "oms_addTimeIndicator" => {
            ensure_arg_count(args, 1, "oms_addTimeIndicator")?;
            let signal = arg!(0, unwrap_string, "STRING");
            Ok(make_integer(om_simoulator::oms_add_time_indicator(signal)))
        }

        "oms_addTLMBus" => {
            ensure_arg_count(args, 4, "oms_addTLMBus")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let domain = arg!(1, unwrap_enum_index, "ENUM_LITERAL") - 1;
            let dimensions = arg!(2, unwrap_integer, "INTEGER");
            let interpolation = arg!(3, unwrap_enum_index, "ENUM_LITERAL") - 1;
            Ok(make_integer(om_simoulator::oms_add_tlm_bus(cref, domain, dimensions, interpolation)))
        }

        "oms_addTLMConnection" => {
            ensure_arg_count(args, 6, "oms_addTLMConnection")?;
            let a = arg!(0, unwrap_string, "STRING");
            let b = arg!(1, unwrap_string, "STRING");
            let delay = arg!(2, unwrap_real, "REAL");
            let alpha = arg!(3, unwrap_real, "REAL");
            let linearimpedance = arg!(4, unwrap_real, "REAL");
            let angularimpedance = arg!(5, unwrap_real, "REAL");
            Ok(make_integer(om_simoulator::oms_add_tlm_connection(
                a, b, delay, alpha, linearimpedance, angularimpedance,
            )))
        }

        "oms_compareSimulationResults" => {
            ensure_arg_count(args, 5, "oms_compareSimulationResults")?;
            let filename_a = arg!(0, unwrap_string, "STRING");
            let filename_b = arg!(1, unwrap_string, "STRING");
            let var = arg!(2, unwrap_string, "STRING");
            let rel_tol = arg!(3, unwrap_real, "REAL");
            let abs_tol = arg!(4, unwrap_real, "REAL");
            Ok(make_integer(om_simoulator::oms_compare_simulation_results(filename_a, filename_b, var, rel_tol, abs_tol)))
        }

        "oms_copySystem" => {
            ensure_arg_count(args, 2, "oms_copySystem")?;
            let source = arg!(0, unwrap_string, "STRING");
            let target = arg!(1, unwrap_string, "STRING");
            Ok(make_integer(om_simoulator::oms_copy_system(source, target)))
        }

        "oms_delete" => {
            ensure_arg_count(args, 1, "oms_delete")?;
            let cref = arg!(0, unwrap_string, "STRING");
            Ok(make_integer(om_simoulator::oms_delete(cref)))
        }

        "oms_deleteConnection" => {
            ensure_arg_count(args, 2, "oms_deleteConnection")?;
            let a = arg!(0, unwrap_string, "STRING");
            let b = arg!(1, unwrap_string, "STRING");
            Ok(make_integer(om_simoulator::oms_delete_connection(a, b)))
        }

        "oms_deleteConnectorFromBus" => {
            ensure_arg_count(args, 2, "oms_deleteConnectorFromBus")?;
            let bus = arg!(0, unwrap_string, "STRING");
            let conn = arg!(1, unwrap_string, "STRING");
            Ok(make_integer(om_simoulator::oms_delete_connector_from_bus(bus, conn)))
        }

        "oms_deleteConnectorFromTLMBus" => {
            ensure_arg_count(args, 2, "oms_deleteConnectorFromTLMBus")?;
            let bus = arg!(0, unwrap_string, "STRING");
            let conn = arg!(1, unwrap_string, "STRING");
            Ok(make_integer(om_simoulator::oms_delete_connector_from_tlm_bus(bus, conn)))
        }

        "oms_export" => {
            ensure_arg_count(args, 2, "oms_export")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let filename = arg!(1, unwrap_string, "STRING");
            Ok(make_integer(om_simoulator::oms_export(cref, filename)))
        }

        "oms_exportDependencyGraphs" => {
            ensure_arg_count(args, 4, "oms_exportDependencyGraphs")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let initialization = arg!(1, unwrap_string, "STRING");
            let event = arg!(2, unwrap_string, "STRING");
            let simulation = arg!(3, unwrap_string, "STRING");
            Ok(make_integer(om_simoulator::oms_export_dependency_graphs(cref, initialization, event, simulation)))
        }

        "oms_exportSnapshot" => {
            ensure_arg_count(args, 1, "oms_exportSnapshot")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let (status, contents) = om_simoulator::oms_export_snapshot(cref);
            Ok(make_tuple(vec![make_string(contents), make_integer(status)]))
        }

        "oms_extractFMIKind" => {
            ensure_arg_count(args, 1, "oms_extractFMIKind")?;
            let filename = arg!(0, unwrap_string, "STRING");
            let (status, kind) = om_simoulator::oms_extract_fmi_kind(filename);
            Ok(make_tuple(vec![make_integer(kind), make_integer(status)]))
        }

        "oms_getBoolean" => {
            ensure_arg_count(args, 1, "oms_getBoolean")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let (status, value) = om_simoulator::oms_get_boolean(cref);
            Ok(make_tuple(vec![Value::BOOL { boolean: value != 0 }, make_integer(status)]))
        }

        "oms_getFixedStepSize" => {
            ensure_arg_count(args, 1, "oms_getFixedStepSize")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let (status, value) = om_simoulator::oms_get_fixed_step_size(cref);
            Ok(make_tuple(vec![Value::REAL { real: value }, make_integer(status)]))
        }

        "oms_getInteger" => {
            ensure_arg_count(args, 1, "oms_getInteger")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let (status, value) = om_simoulator::oms_get_integer(cref);
            Ok(make_tuple(vec![make_integer(value), make_integer(status)]))
        }

        "oms_getModelState" => {
            ensure_arg_count(args, 1, "oms_getModelState")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let (status, value) = om_simoulator::oms_get_model_state(cref);
            Ok(make_tuple(vec![make_integer(value), make_integer(status)]))
        }

        "oms_getReal" => {
            ensure_arg_count(args, 1, "oms_getReal")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let (status, value) = om_simoulator::oms_get_real(cref);
            Ok(make_tuple(vec![Value::REAL { real: value }, make_integer(status)]))
        }

        "oms_getSolver" => {
            ensure_arg_count(args, 1, "oms_getSolver")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let (status, value) = om_simoulator::oms_get_solver(cref);
            Ok(make_tuple(vec![make_integer(value), make_integer(status)]))
        }

        "oms_getStartTime" => {
            ensure_arg_count(args, 1, "oms_getStartTime")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let (status, value) = om_simoulator::oms_get_start_time(cref);
            Ok(make_tuple(vec![Value::REAL { real: value }, make_integer(status)]))
        }

        "oms_getStopTime" => {
            ensure_arg_count(args, 1, "oms_getStopTime")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let (status, value) = om_simoulator::oms_get_stop_time(cref);
            Ok(make_tuple(vec![Value::REAL { real: value }, make_integer(status)]))
        }

        "oms_getSubModelPath" => {
            ensure_arg_count(args, 1, "oms_getSubModelPath")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let (status, path) = om_simoulator::oms_get_sub_model_path(cref);
            Ok(make_tuple(vec![make_string(path), make_integer(status)]))
        }

        "oms_getSystemType" => {
            ensure_arg_count(args, 1, "oms_getSystemType")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let (status, value) = om_simoulator::oms_get_system_type(cref);
            Ok(make_tuple(vec![make_integer(value), make_integer(status)]))
        }

        "oms_getTolerance" => {
            ensure_arg_count(args, 1, "oms_getTolerance")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let (status, abs_tol, rel_tol) = om_simoulator::oms_get_tolerance(cref);
            Ok(make_tuple(vec![
                Value::REAL { real: abs_tol },
                Value::REAL { real: rel_tol },
                make_integer(status),
            ]))
        }

        "oms_getVariableStepSize" => {
            ensure_arg_count(args, 1, "oms_getVariableStepSize")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let (status, initial, minimum, maximum) = om_simoulator::oms_get_variable_step_size(cref);
            Ok(make_tuple(vec![
                Value::REAL { real: initial },
                Value::REAL { real: minimum },
                Value::REAL { real: maximum },
                make_integer(status),
            ]))
        }

        "oms_faultInjection" => {
            ensure_arg_count(args, 3, "oms_faultInjection")?;
            let signal = arg!(0, unwrap_string, "STRING");
            let fault_type = arg!(1, unwrap_enum_index, "ENUM_LITERAL") - 1;
            let fault_value = arg!(2, unwrap_real, "REAL");
            Ok(make_integer(om_simoulator::oms_fault_injection(signal, fault_type, fault_value)))
        }

        "oms_importFile" => {
            ensure_arg_count(args, 1, "oms_importFile")?;
            let filename = arg!(0, unwrap_string, "STRING");
            let (status, cref) = om_simoulator::oms_import_file(filename);
            Ok(make_tuple(vec![make_string(cref), make_integer(status)]))
        }

        "oms_importSnapshot" => {
            ensure_arg_count(args, 2, "oms_importSnapshot")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let snapshot = arg!(1, unwrap_string, "STRING");
            Ok(make_integer(om_simoulator::oms_import_snapshot(cref, snapshot)))
        }

        "oms_initialize" => {
            ensure_arg_count(args, 1, "oms_initialize")?;
            let cref = arg!(0, unwrap_string, "STRING");
            Ok(make_integer(om_simoulator::oms_initialize(cref)))
        }

        "oms_instantiate" => {
            ensure_arg_count(args, 1, "oms_instantiate")?;
            let cref = arg!(0, unwrap_string, "STRING");
            Ok(make_integer(om_simoulator::oms_instantiate(cref)))
        }

        "oms_list" => {
            ensure_arg_count(args, 1, "oms_list")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let (status, contents) = om_simoulator::oms_list(cref);
            Ok(make_tuple(vec![make_string(contents), make_integer(status)]))
        }

        "oms_listUnconnectedConnectors" => {
            ensure_arg_count(args, 1, "oms_listUnconnectedConnectors")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let (status, contents) = om_simoulator::oms_list_unconnected_connectors(cref);
            Ok(make_tuple(vec![make_string(contents), make_integer(status)]))
        }

        "oms_loadSnapshot" => {
            ensure_arg_count(args, 2, "oms_loadSnapshot")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let snapshot = arg!(1, unwrap_string, "STRING");
            let (status, new_cref) = om_simoulator::oms_load_snapshot(cref, snapshot);
            Ok(make_tuple(vec![make_string(new_cref), make_integer(status)]))
        }

        "oms_newModel" => {
            ensure_arg_count(args, 1, "oms_newModel")?;
            let cref = arg!(0, unwrap_string, "STRING");
            Ok(make_integer(om_simoulator::oms_new_model(cref)))
        }

        "oms_removeSignalsFromResults" => {
            ensure_arg_count(args, 2, "oms_removeSignalsFromResults")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let regex = arg!(1, unwrap_string, "STRING");
            Ok(make_integer(om_simoulator::oms_remove_signals_from_results(cref, regex)))
        }

        "oms_rename" => {
            ensure_arg_count(args, 2, "oms_rename")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let new_cref = arg!(1, unwrap_string, "STRING");
            Ok(make_integer(om_simoulator::oms_rename(cref, new_cref)))
        }

        "oms_reset" => {
            ensure_arg_count(args, 1, "oms_reset")?;
            let cref = arg!(0, unwrap_string, "STRING");
            Ok(make_integer(om_simoulator::oms_reset(cref)))
        }

        "oms_RunFile" => {
            ensure_arg_count(args, 1, "oms_RunFile")?;
            let filename = arg!(0, unwrap_string, "STRING");
            Ok(make_integer(om_simoulator::oms_run_file(filename)))
        }

        "oms_setBoolean" => {
            ensure_arg_count(args, 2, "oms_setBoolean")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let b = arg!(1, unwrap_bool, "BOOL");
            Ok(make_integer(om_simoulator::oms_set_boolean(cref, b)))
        }

        "oms_setCommandLineOption" => {
            ensure_arg_count(args, 1, "oms_setCommandLineOption")?;
            let cmd = arg!(0, unwrap_string, "STRING");
            Ok(make_integer(om_simoulator::oms_set_command_line_option(cmd)))
        }

        "oms_setFixedStepSize" => {
            ensure_arg_count(args, 2, "oms_setFixedStepSize")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let step_size = arg!(1, unwrap_real, "REAL");
            Ok(make_integer(om_simoulator::oms_set_fixed_step_size(cref, step_size)))
        }

        "oms_setInteger" => {
            ensure_arg_count(args, 2, "oms_setInteger")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let ivalue = arg!(1, unwrap_integer, "INTEGER");
            Ok(make_integer(om_simoulator::oms_set_integer(cref, ivalue)))
        }

        "oms_setLogFile" => {
            ensure_arg_count(args, 1, "oms_setLogFile")?;
            let filename = arg!(0, unwrap_string, "STRING");
            Ok(make_integer(om_simoulator::oms_set_log_file(filename)))
        }

        "oms_setLoggingInterval" => {
            ensure_arg_count(args, 2, "oms_setLoggingInterval")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let logging_interval = arg!(1, unwrap_real, "REAL");
            Ok(make_integer(om_simoulator::oms_set_logging_interval(cref, logging_interval)))
        }

        "oms_setLoggingLevel" => {
            ensure_arg_count(args, 1, "oms_setLoggingLevel")?;
            let log_level = arg!(0, unwrap_integer, "INTEGER");
            Ok(make_integer(om_simoulator::oms_set_logging_level(log_level)))
        }

        "oms_setReal" => {
            ensure_arg_count(args, 2, "oms_setReal")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let rvalue = arg!(1, unwrap_real, "REAL");
            Ok(make_integer(om_simoulator::oms_set_real(cref, rvalue)))
        }

        "oms_setRealInputDerivative" => {
            ensure_arg_count(args, 2, "oms_setRealInputDerivative")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let value = arg!(1, unwrap_real, "REAL");
            Ok(make_integer(om_simoulator::oms_set_real_input_derivative(cref, value)))
        }

        "oms_setResultFile" => {
            ensure_arg_count(args, 3, "oms_setResultFile")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let filename = arg!(1, unwrap_string, "STRING");
            let buffer_size = arg!(2, unwrap_integer, "INTEGER");
            Ok(make_integer(om_simoulator::oms_set_result_file(cref, filename, buffer_size)))
        }

        "oms_setSignalFilter" => {
            ensure_arg_count(args, 2, "oms_setSignalFilter")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let regex = arg!(1, unwrap_string, "STRING");
            Ok(make_integer(om_simoulator::oms_set_signal_filter(cref, regex)))
        }

        "oms_setSolver" => {
            ensure_arg_count(args, 2, "oms_setSolver")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let solver = arg!(1, unwrap_enum_index, "ENUM_LITERAL") - 1;
            Ok(make_integer(om_simoulator::oms_set_solver(cref, solver)))
        }

        "oms_setStartTime" => {
            ensure_arg_count(args, 2, "oms_setStartTime")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let start_time = arg!(1, unwrap_real, "REAL");
            Ok(make_integer(om_simoulator::oms_set_start_time(cref, start_time)))
        }

        "oms_setStopTime" => {
            ensure_arg_count(args, 2, "oms_setStopTime")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let stop_time = arg!(1, unwrap_real, "REAL");
            Ok(make_integer(om_simoulator::oms_set_stop_time(cref, stop_time)))
        }

        "oms_setTempDirectory" => {
            ensure_arg_count(args, 1, "oms_setTempDirectory")?;
            let new_temp_dir = arg!(0, unwrap_string, "STRING");
            Ok(make_integer(om_simoulator::oms_set_temp_directory(new_temp_dir)))
        }

        "oms_setTLMPositionAndOrientation" => {
            ensure_arg_count(args, 14, "oms_setTLMPositionAndOrientation")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let x1 = arg!(1, unwrap_real, "REAL");
            let x2 = arg!(2, unwrap_real, "REAL");
            let x3 = arg!(3, unwrap_real, "REAL");
            let a11 = arg!(4, unwrap_real, "REAL");
            let a12 = arg!(5, unwrap_real, "REAL");
            let a13 = arg!(6, unwrap_real, "REAL");
            let a21 = arg!(7, unwrap_real, "REAL");
            let a22 = arg!(8, unwrap_real, "REAL");
            let a23 = arg!(9, unwrap_real, "REAL");
            let a31 = arg!(10, unwrap_real, "REAL");
            let a32 = arg!(11, unwrap_real, "REAL");
            let a33 = arg!(12, unwrap_real, "REAL");
            Ok(make_integer(om_simoulator::oms_set_tlm_position_and_orientation(
                cref, x1, x2, x3, a11, a12, a13, a21, a22, a23, a31, a32, a33,
            )))
        }

        "oms_setTLMSocketData" => {
            ensure_arg_count(args, 4, "oms_setTLMSocketData")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let address = arg!(1, unwrap_string, "STRING");
            let manager_port = arg!(2, unwrap_integer, "INTEGER");
            let monitor_port = arg!(3, unwrap_integer, "INTEGER");
            Ok(make_integer(om_simoulator::oms_set_tlm_socket_data(cref, address, manager_port, monitor_port)))
        }

        "oms_setTolerance" => {
            ensure_arg_count(args, 3, "oms_setTolerance")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let abs_tol = arg!(1, unwrap_real, "REAL");
            let rel_tol = arg!(2, unwrap_real, "REAL");
            Ok(make_integer(om_simoulator::oms_set_tolerance(cref, abs_tol, rel_tol)))
        }

        "oms_setVariableStepSize" => {
            ensure_arg_count(args, 4, "oms_setVariableStepSize")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let initial = arg!(1, unwrap_real, "REAL");
            let minimum = arg!(2, unwrap_real, "REAL");
            let maximum = arg!(3, unwrap_real, "REAL");
            Ok(make_integer(om_simoulator::oms_set_variable_step_size(cref, initial, minimum, maximum)))
        }

        "oms_setWorkingDirectory" => {
            ensure_arg_count(args, 1, "oms_setWorkingDirectory")?;
            let new_working_dir = arg!(0, unwrap_string, "STRING");
            Ok(make_integer(om_simoulator::oms_set_working_directory(new_working_dir)))
        }

        "oms_simulate" => {
            ensure_arg_count(args, 1, "oms_simulate")?;
            let cref = arg!(0, unwrap_string, "STRING");
            Ok(make_integer(om_simoulator::oms_simulate(cref)))
        }

        "oms_stepUntil" => {
            ensure_arg_count(args, 2, "oms_stepUntil")?;
            let cref = arg!(0, unwrap_string, "STRING");
            let stop_time = arg!(1, unwrap_real, "REAL");
            Ok(make_integer(om_simoulator::oms_step_until(cref, stop_time)))
        }

        "oms_terminate" => {
            ensure_arg_count(args, 1, "oms_terminate")?;
            let cref = arg!(0, unwrap_string, "STRING");
            Ok(make_integer(om_simoulator::oms_terminate(cref)))
        }

        "oms_getVersion" => {
            ensure_arg_count(args, 0, "oms_getVersion")?;
            Ok(make_string(om_simoulator::oms_get_version()))
        }

        _ => bail!("unknown function: {}", function_name),
    }
}

/// Validate that args has exactly the expected count.
fn ensure_arg_count(args: &Vector<Value>, expected: usize, func: &str) -> Result<()> {
    if args.len() != expected {
        bail!(
            "{}: expected {} arguments, got {}",
            func,
            expected,
            args.len()
        );
    }
    Ok(())
}
