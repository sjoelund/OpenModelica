//! Translation of Util/OMSimulatorExt.mo
//!
//! This module provides bindings to the OpenModelica Simulator runtime API via the
//! `omcruntime` C library. It exposes functions for creating/managing simulation
//! models, buses, connections, TLMSystems, configuring solvers, retrieving state,
//! and controlling simulation execution.
//!
//! All external C functions link against the `omcruntime` library.

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

// ============================================================================
// Extern "C" declarations (C API from omcruntime)
// ============================================================================

/// Loads the OM Simulator DLL / runtime.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_loadDLL() -> c_int;
}

/// Unloads the OM Simulator DLL / runtime.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_unloadDLL() -> c_int;
}

/// Returns the version string of this release.
/// Returns a pointer to a static version string.
unsafe extern "C" {
    fn OMSimulator_oms_getVersion() -> *const c_char;
}

// --- Bus management ---

/// Adds a bus with the given cref.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_addBus(cref: *const c_char) -> c_int;
}

/// Adds a connection between two crefs.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_addConnection(crefA: *const c_char, crefB: *const c_char) -> c_int;
}

/// Adds a connector to a model.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_addConnector(
        cref: *const c_char,
        causality: c_int,
        type_: c_int,
    ) -> c_int;
}

/// Adds a connector to a bus.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_addConnectorToBus(
        busCref: *const c_char,
        connectorCref: *const c_char,
    ) -> c_int;
}

/// Adds a connector to a TLM bus.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_addConnectorToTLMBus(
        busCref: *const c_char,
        connectorCref: *const c_char,
        type_: *const c_char,
    ) -> c_int;
}

/// Adds a dynamic value indicator.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_addDynamicValueIndicator(
        signal: *const c_char,
        lower: *const c_char,
        upper: *const c_char,
        stepSize: f64,
    ) -> c_int;
}

/// Adds an event indicator.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_addEventIndicator(signal: *const c_char) -> c_int;
}

/// Adds an external model.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_addExternalModel(
        cref: *const c_char,
        path: *const c_char,
        startscript: *const c_char,
    ) -> c_int;
}

/// Adds signals to results matching a regex.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_addSignalsToResults(
        cref: *const c_char,
        regex: *const c_char,
    ) -> c_int;
}

/// Adds a static value indicator.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_addStaticValueIndicator(
        signal: *const c_char,
        lower: f64,
        upper: f64,
        stepSize: f64,
    ) -> c_int;
}

/// Adds a submodel (FMU).
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_addSubModel(cref: *const c_char, fmuPath: *const c_char) -> c_int;
}

/// Adds a system.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_addSystem(cref: *const c_char, type_: c_int) -> c_int;
}

/// Adds a time indicator.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_addTimeIndicator(signal: *const c_char) -> c_int;
}

/// Adds a TLM bus.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_addTLMBus(
        cref: *const c_char,
        domain: c_int,
        dimensions: c_int,
        interpolation: c_int,
    ) -> c_int;
}

/// Adds a TLM connection.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_addTLMConnection(
        crefA: *const c_char,
        crefB: *const c_char,
        delay: f64,
        alpha: f64,
        linearimpedance: f64,
        angularimpedance: f64,
    ) -> c_int;
}

// --- Comparison ---

/// Compares two simulation result files for a variable.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_compareSimulationResults(
        filenameA: *const c_char,
        filenameB: *const c_char,
        var: *const c_char,
        relTol: f64,
        absTol: f64,
    ) -> c_int;
}

/// Copies a system from source to target.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_copySystem(source: *const c_char, target: *const c_char) -> c_int;
}

// --- Deletion ---

/// Deletes the entity with the given cref.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_delete(cref: *const c_char) -> c_int;
}

/// Deletes a connection between two crefs.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_deleteConnection(crefA: *const c_char, crefB: *const c_char) -> c_int;
}

/// Deletes a connector from a bus.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_deleteConnectorFromBus(
        busCref: *const c_char,
        connectorCref: *const c_char,
    ) -> c_int;
}

/// Deletes a connector from a TLM bus.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_deleteConnectorFromTLMBus(
        busCref: *const c_char,
        connectorCref: *const c_char,
    ) -> c_int;
}

// --- Export ---

/// Exports a model to a file.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_export(cref: *const c_char, filename: *const c_char) -> c_int;
}

/// Exports dependency graphs.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_exportDependencyGraphs(
        cref: *const c_char,
        initialization: *const c_char,
        event: *const c_char,
        simulation: *const c_char,
    ) -> c_int;
}

/// Exports the current snapshot.
/// Returns status code, writes snapshot contents to out pointer.
unsafe extern "C" {
    fn OMSimulator_oms_exportSnapshot(
        cref: *const c_char,
        contents: *mut *const c_char,
    ) -> c_int;
}

// --- Inspection ---

/// Extracts FMI kind from an FMU file.
/// Returns status code, writes kind to out pointer.
unsafe extern "C" {
    fn OMSimulator_oms_extractFMIKind(
        filename: *const c_char,
        kind: *mut c_int,
    ) -> c_int;
}

/// Gets a boolean value from a model.
/// Returns status code, writes value to out pointer.
unsafe extern "C" {
    fn OMSimulator_oms_getBoolean(cref: *const c_char, value: *mut c_int) -> c_int;
}

/// Gets the fixed step size.
/// Returns status code, writes stepSize to out pointer.
unsafe extern "C" {
    fn OMSimulator_oms_getFixedStepSize(cref: *const c_char, stepSize: *mut f64) -> c_int;
}

/// Gets an integer value from a model.
/// Returns status code, writes value to out pointer.
unsafe extern "C" {
    fn OMSimulator_oms_getInteger(cref: *const c_char, value: *mut c_int) -> c_int;
}

/// Gets the model state.
/// Returns status code, writes modelState to out pointer.
unsafe extern "C" {
    fn OMSimulator_oms_getModelState(cref: *const c_char, modelState: *mut c_int) -> c_int;
}

/// Gets a real (float) value from a model.
/// Returns status code, writes value to out pointer.
unsafe extern "C" {
    fn OMSimulator_oms_getReal(cref: *const c_char, value: *mut f64) -> c_int;
}

/// Gets the solver type.
/// Returns status code, writes solver to out pointer.
unsafe extern "C" {
    fn OMSimulator_oms_getSolver(cref: *const c_char, solver: *mut c_int) -> c_int;
}

/// Gets the start time.
/// Returns status code, writes startTime to out pointer.
unsafe extern "C" {
    fn OMSimulator_oms_getStartTime(cref: *const c_char, startTime: *mut f64) -> c_int;
}

/// Gets the stop time.
/// Returns status code, writes stopTime to out pointer.
unsafe extern "C" {
    fn OMSimulator_oms_getStopTime(cref: *const c_char, stopTime: *mut f64) -> c_int;
}

/// Gets the submodel path.
/// Returns status code, writes path to out pointer.
unsafe extern "C" {
    fn OMSimulator_oms_getSubModelPath(
        cref: *const c_char,
        path: *mut *const c_char,
    ) -> c_int;
}

/// Gets the system type.
/// Returns status code, writes type to out pointer.
unsafe extern "C" {
    fn OMSimulator_oms_getSystemType(cref: *const c_char, type_: *mut c_int) -> c_int;
}

/// Gets tolerance values.
/// Returns status code, writes tolerances to out pointers.
unsafe extern "C" {
    fn OMSimulator_oms_getTolerance(
        cref: *const c_char,
        absoluteTolerance: *mut f64,
        relativeTolerance: *mut f64,
    ) -> c_int;
}

/// Gets variable step size information.
/// Returns status code, writes step sizes to out pointers.
unsafe extern "C" {
    fn OMSimulator_oms_getVariableStepSize(
        cref: *const c_char,
        initialStepSize: *mut f64,
        minimumStepSize: *mut f64,
        maximumStepSize: *mut f64,
    ) -> c_int;
}

// --- Fault injection ---

/// Injects a fault into a signal.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_faultInjection(
        signal: *const c_char,
        faultType: c_int,
        faultValue: f64,
    ) -> c_int;
}

// --- Import ---

/// Imports a file, returns the cref.
/// Returns status code, writes cref to out pointer.
unsafe extern "C" {
    fn OMSimulator_oms_importFile(
        filename: *const c_char,
        cref: *mut *const c_char,
    ) -> c_int;
}

/// Imports a snapshot.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_importSnapshot(
        cref: *const c_char,
        snapshot: *const c_char,
    ) -> c_int;
}

// --- Initialization / execution ---

/// Initializes the model.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_initialize(cref: *const c_char) -> c_int;
}

/// Instantiates the model.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_instantiate(cref: *const c_char) -> c_int;
}

/// Lists contents of a model.
/// Returns status code, writes contents to out pointer.
unsafe extern "C" {
    fn OMSimulator_oms_list(cref: *const c_char, contents: *mut *const c_char) -> c_int;
}

/// Lists unconnected connectors.
/// Returns status code, writes contents to out pointer.
unsafe extern "C" {
    fn OMSimulator_oms_listUnconnectedConnectors(
        cref: *const c_char,
        contents: *mut *const c_char,
    ) -> c_int;
}

/// Loads a snapshot into a new cref.
/// Returns status code, writes newCref to out pointer.
unsafe extern "C" {
    fn OMSimulator_oms_loadSnapshot(
        cref: *const c_char,
        snapshot: *const c_char,
        newCref: *mut *const c_char,
    ) -> c_int;
}

/// Creates a new model.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_newModel(cref: *const c_char) -> c_int;
}

/// Removes signals from results matching a regex.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_removeSignalsFromResults(
        cref: *const c_char,
        regex: *const c_char,
    ) -> c_int;
}

/// Renames a model.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_rename(cref: *const c_char, newCref: *const c_char) -> c_int;
}

/// Resets the model.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_reset(cref: *const c_char) -> c_int;
}

/// Runs a simulation from a file.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_RunFile(filename: *const c_char) -> c_int;
}

// --- Configuration (setters) ---

/// Sets a boolean value in a model.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_setBoolean(cref: *const c_char, value: c_int) -> c_int;
}

/// Sets a command line option.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_setCommandLineOption(cmd: *const c_char) -> c_int;
}

/// Sets the fixed step size.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_setFixedStepSize(cref: *const c_char, stepSize: f64) -> c_int;
}

/// Sets an integer value in a model.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_setInteger(cref: *const c_char, value: c_int) -> c_int;
}

/// Sets the log file.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_setLogFile(filename: *const c_char) -> c_int;
}

/// Sets the logging interval.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_setLoggingInterval(cref: *const c_char, loggingInterval: f64) -> c_int;
}

/// Sets the logging level.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_setLoggingLevel(logLevel: c_int) -> c_int;
}

/// Sets a real (float) value in a model.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_setReal(cref: *const c_char, value: f64) -> c_int;
}

/// Sets real input derivative.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_setRealInputDerivative(cref: *const c_char, value: f64) -> c_int;
}

/// Sets the result file.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_setResultFile(
        cref: *const c_char,
        filename: *const c_char,
        bufferSize: c_int,
    ) -> c_int;
}

/// Sets the signal filter (regex).
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_setSignalFilter(cref: *const c_char, regex: *const c_char) -> c_int;
}

/// Sets the solver type.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_setSolver(cref: *const c_char, solver: c_int) -> c_int;
}

/// Sets the start time.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_setStartTime(cref: *const c_char, startTime: f64) -> c_int;
}

/// Sets the stop time.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_setStopTime(cref: *const c_char, stopTime: f64) -> c_int;
}

/// Sets the temporary directory.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_setTempDirectory(newTempDir: *const c_char) -> c_int;
}

/// Sets TLM position and orientation (12 matrix elements + 3 coordinates).
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_setTLMPositionAndOrientation(
        cref: *const c_char,
        x1: f64,
        x2: f64,
        x3: f64,
        A11: f64,
        A12: f64,
        A13: f64,
        A21: f64,
        A22: f64,
        A23: f64,
        A31: f64,
        A32: f64,
        A33: f64,
    ) -> c_int;
}

/// Sets TLM socket data.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_setTLMSocketData(
        cref: *const c_char,
        address: *const c_char,
        managerPort: c_int,
        monitorPort: c_int,
    ) -> c_int;
}

/// Sets tolerance values.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_setTolerance(
        cref: *const c_char,
        absoluteTolerance: f64,
        relativeTolerance: f64,
    ) -> c_int;
}

/// Sets variable step size parameters.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_setVariableStepSize(
        cref: *const c_char,
        initialStepSize: f64,
        minimumStepSize: f64,
        maximumStepSize: f64,
    ) -> c_int;
}

/// Sets the working directory.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_setWorkingDirectory(newWorkingDir: *const c_char) -> c_int;
}

// --- Simulation control ---

/// Runs a simulation on a model.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_simulate(cref: *const c_char) -> c_int;
}

/// Steps the simulation until the given stop time.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_stepUntil(cref: *const c_char, stopTime: f64) -> c_int;
}

/// Terminates the model.
/// Returns status code.
unsafe extern "C" {
    fn OMSimulator_oms_terminate(cref: *const c_char) -> c_int;
}

// ============================================================================
// Safe wrapper functions (translated from MetaModelica)
// ============================================================================

/// Converts a status integer to its string representation.
/// Translated from the `statusToString` function in OMSimulatorExt.mo.
///
/// # Parameters
/// * `status` - The status code (0=ok, 1=warning, 2=discard, 3=error, 4=fatal, 5=pending)
///
/// # Returns
/// A string describing the status.
pub fn status_to_string(status: i32) -> &'static str {
    match status {
        0 => "ok",
        1 => "warning",
        2 => "discard",
        3 => "error",
        4 => "fatal",
        5 => "pending",
        _ => "unknown_status",
    }
}

/// Loads the OM Simulator DLL / runtime.
/// Mirrors the `loadOMSimulator` function from OMSimulatorExt.mo.
///
/// # Returns
/// Status code from the C function.
pub fn load_om_simulator() -> i32 {
    unsafe { OMSimulator_loadDLL() }
}

/// Unloads the OM Simulator DLL / runtime.
/// Mirrors the `unloadOMSimulator` function from OMSimulatorExt.mo.
///
/// # Returns
/// Status code from the C function.
pub fn unload_om_simulator() -> i32 {
    unsafe { OMSimulator_unloadDLL() }
}

/// Returns the version string of this release.
/// Mirrors the `oms_getVersion` function from OMSimulatorExt.mo.
///
/// # Returns
/// The version string, or an empty string on failure.
pub fn oms_get_version() -> String {
    let ptr = unsafe { OMSimulator_oms_getVersion() };
    if ptr.is_null() {
        String::new()
    } else {
        unsafe {
            CStr::from_ptr(ptr)
                .to_string_lossy()
                .into_owned()
        }
    }
}

// --- Bus management ---

/// Adds a bus with the given cref.
/// Mirrors the `oms_addBus` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The bus reference name
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_add_bus(cref: &str) -> i32 {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    unsafe { OMSimulator_oms_addBus(c_ref.as_ptr()) }
}

/// Adds a connection between two crefs.
/// Mirrors the `oms_addConnection` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref_a` - First connection reference
/// * `cref_b` - Second connection reference
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if `cref_a` or `cref_b` contains an embedded null byte.
pub fn oms_add_connection(cref_a: &str, cref_b: &str) -> i32 {
    let a = CString::new(cref_a).expect("cref_a contains null byte");
    let b = CString::new(cref_b).expect("cref_b contains null byte");
    unsafe { OMSimulator_oms_addConnection(a.as_ptr(), b.as_ptr()) }
}

/// Adds a connector to a model.
/// Mirrors the `oms_addConnector` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
/// * `causality` - The causality of the connector
/// * `type_` - The type of the connector
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_add_connector(cref: &str, causality: i32, type_: i32) -> i32 {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    unsafe { OMSimulator_oms_addConnector(c_ref.as_ptr(), causality, type_) }
}

/// Adds a connector to a bus.
/// Mirrors the `oms_addConnectorToBus` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `bus_cref` - The bus reference
/// * `connector_cref` - The connector reference
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if `bus_cref` or `connector_cref` contains an embedded null byte.
pub fn oms_add_connector_to_bus(bus_cref: &str, connector_cref: &str) -> i32 {
    let bus = CString::new(bus_cref).expect("bus_cref contains null byte");
    let conn = CString::new(connector_cref).expect("connector_cref contains null byte");
    unsafe { OMSimulator_oms_addConnectorToBus(bus.as_ptr(), conn.as_ptr()) }
}

/// Adds a connector to a TLM bus.
/// Mirrors the `oms_addConnectorToTLMBus` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `bus_cref` - The TLM bus reference
/// * `connector_cref` - The connector reference
/// * `type_` - The type string
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if any argument contains an embedded null byte.
pub fn oms_add_connector_to_tlm_bus(bus_cref: &str, connector_cref: &str, type_: &str) -> i32 {
    let bus = CString::new(bus_cref).expect("bus_cref contains null byte");
    let conn = CString::new(connector_cref).expect("connector_cref contains null byte");
    let typ = CString::new(type_).expect("type_ contains null byte");
    unsafe {
        OMSimulator_oms_addConnectorToTLMBus(bus.as_ptr(), conn.as_ptr(), typ.as_ptr())
    }
}

/// Adds a dynamic value indicator.
/// Mirrors the `oms_addDynamicValueIndicator` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `signal` - The signal name
/// * `lower` - The lower bound string
/// * `upper` - The upper bound string
/// * `step_size` - The step size
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if any string argument contains an embedded null byte.
pub fn oms_add_dynamic_value_indicator(
    signal: &str,
    lower: &str,
    upper: &str,
    step_size: f64,
) -> i32 {
    let s = CString::new(signal).expect("signal contains null byte");
    let l = CString::new(lower).expect("lower contains null byte");
    let u = CString::new(upper).expect("upper contains null byte");
    unsafe { OMSimulator_oms_addDynamicValueIndicator(s.as_ptr(), l.as_ptr(), u.as_ptr(), step_size) }
}

/// Adds an event indicator.
/// Mirrors the `oms_addEventIndicator` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `signal` - The signal name
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if `signal` contains an embedded null byte.
pub fn oms_add_event_indicator(signal: &str) -> i32 {
    let s = CString::new(signal).expect("signal contains null byte");
    unsafe { OMSimulator_oms_addEventIndicator(s.as_ptr()) }
}

/// Adds an external model.
/// Mirrors the `oms_addExternalModel` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
/// * `path` - The model path
/// * `startscript` - The start script
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if any argument contains an embedded null byte.
pub fn oms_add_external_model(cref: &str, path: &str, startscript: &str) -> i32 {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    let p = CString::new(path).expect("path contains null byte");
    let s = CString::new(startscript).expect("startscript contains null byte");
    unsafe { OMSimulator_oms_addExternalModel(c_ref.as_ptr(), p.as_ptr(), s.as_ptr()) }
}

/// Adds signals to results matching a regex.
/// Mirrors the `oms_addSignalsToResults` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
/// * `regex` - The regex pattern
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if any argument contains an embedded null byte.
pub fn oms_add_signals_to_results(cref: &str, regex: &str) -> i32 {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    let r = CString::new(regex).expect("regex contains null byte");
    unsafe { OMSimulator_oms_addSignalsToResults(c_ref.as_ptr(), r.as_ptr()) }
}

/// Adds a static value indicator.
/// Mirrors the `oms_addStaticValueIndicator` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `signal` - The signal name
/// * `lower` - The lower bound
/// * `upper` - The upper bound
/// * `step_size` - The step size
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if `signal` contains an embedded null byte.
pub fn oms_add_static_value_indicator(signal: &str, lower: f64, upper: f64, step_size: f64) -> i32 {
    let s = CString::new(signal).expect("signal contains null byte");
    unsafe { OMSimulator_oms_addStaticValueIndicator(s.as_ptr(), lower, upper, step_size) }
}

/// Adds a submodel (FMU).
/// Mirrors the `oms_addSubModel` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
/// * `fmu_path` - The path to the FMU file
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if any argument contains an embedded null byte.
pub fn oms_add_sub_model(cref: &str, fmu_path: &str) -> i32 {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    let p = CString::new(fmu_path).expect("fmu_path contains null byte");
    unsafe { OMSimulator_oms_addSubModel(c_ref.as_ptr(), p.as_ptr()) }
}

/// Adds a system.
/// Mirrors the `oms_addSystem` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The system reference
/// * `type_` - The system type
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_add_system(cref: &str, type_: i32) -> i32 {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    unsafe { OMSimulator_oms_addSystem(c_ref.as_ptr(), type_) }
}

/// Adds a time indicator.
/// Mirrors the `oms_addTimeIndicator` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `signal` - The signal name
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if `signal` contains an embedded null byte.
pub fn oms_add_time_indicator(signal: &str) -> i32 {
    let s = CString::new(signal).expect("signal contains null byte");
    unsafe { OMSimulator_oms_addTimeIndicator(s.as_ptr()) }
}

/// Adds a TLM bus.
/// Mirrors the `oms_addTLMBus` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The bus reference
/// * `domain` - The domain
/// * `dimensions` - The number of dimensions
/// * `interpolation` - The interpolation type
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_add_tlm_bus(cref: &str, domain: i32, dimensions: i32, interpolation: i32) -> i32 {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    unsafe { OMSimulator_oms_addTLMBus(c_ref.as_ptr(), domain, dimensions, interpolation) }
}

/// Adds a TLM connection.
/// Mirrors the `oms_addTLMConnection` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref_a` - First connection reference
/// * `cref_b` - Second connection reference
/// * `delay` - The delay
/// * `alpha` - The alpha value
/// * `linearimpedance` - The linear impedance
/// * `angularimpedance` - The angular impedance
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if `cref_a` or `cref_b` contains an embedded null byte.
pub fn oms_add_tlm_connection(
    cref_a: &str,
    cref_b: &str,
    delay: f64,
    alpha: f64,
    linearimpedance: f64,
    angularimpedance: f64,
) -> i32 {
    let a = CString::new(cref_a).expect("cref_a contains null byte");
    let b = CString::new(cref_b).expect("cref_b contains null byte");
    unsafe {
        OMSimulator_oms_addTLMConnection(
            a.as_ptr(), b.as_ptr(), delay, alpha, linearimpedance, angularimpedance,
        )
    }
}

// --- Comparison ---

/// Compares two simulation result files for a variable.
/// Mirrors the `oms_compareSimulationResults` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `filename_a` - First result file
/// * `filename_b` - Second result file
/// * `var` - Variable name to compare
/// * `rel_tol` - Relative tolerance
/// * `abs_tol` - Absolute tolerance
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if any argument contains an embedded null byte.
pub fn oms_compare_simulation_results(
    filename_a: &str,
    filename_b: &str,
    var: &str,
    rel_tol: f64,
    abs_tol: f64,
) -> i32 {
    let a = CString::new(filename_a).expect("filename_a contains null byte");
    let b = CString::new(filename_b).expect("filename_b contains null byte");
    let v = CString::new(var).expect("var contains null byte");
    unsafe {
        OMSimulator_oms_compareSimulationResults(
            a.as_ptr(), b.as_ptr(), v.as_ptr(), rel_tol, abs_tol,
        )
    }
}

/// Copies a system from source to target.
/// Mirrors the `oms_copySystem` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `source` - Source system reference
/// * `target` - Target system reference
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if any argument contains an embedded null byte.
pub fn oms_copy_system(source: &str, target: &str) -> i32 {
    let s = CString::new(source).expect("source contains null byte");
    let t = CString::new(target).expect("target contains null byte");
    unsafe { OMSimulator_oms_copySystem(s.as_ptr(), t.as_ptr()) }
}

// --- Deletion ---

/// Deletes the entity with the given cref.
/// Mirrors the `oms_delete` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The reference to delete
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_delete(cref: &str) -> i32 {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    unsafe { OMSimulator_oms_delete(c_ref.as_ptr()) }
}

/// Deletes a connection between two crefs.
/// Mirrors the `oms_deleteConnection` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref_a` - First connection reference
/// * `cref_b` - Second connection reference
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if any argument contains an embedded null byte.
pub fn oms_delete_connection(cref_a: &str, cref_b: &str) -> i32 {
    let a = CString::new(cref_a).expect("cref_a contains null byte");
    let b = CString::new(cref_b).expect("cref_b contains null byte");
    unsafe { OMSimulator_oms_deleteConnection(a.as_ptr(), b.as_ptr()) }
}

/// Deletes a connector from a bus.
/// Mirrors the `oms_deleteConnectorFromBus` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `bus_cref` - The bus reference
/// * `connector_cref` - The connector reference
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if any argument contains an embedded null byte.
pub fn oms_delete_connector_from_bus(bus_cref: &str, connector_cref: &str) -> i32 {
    let bus = CString::new(bus_cref).expect("bus_cref contains null byte");
    let conn = CString::new(connector_cref).expect("connector_cref contains null byte");
    unsafe { OMSimulator_oms_deleteConnectorFromBus(bus.as_ptr(), conn.as_ptr()) }
}

/// Deletes a connector from a TLM bus.
/// Mirrors the `oms_deleteConnectorFromTLMBus` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `bus_cref` - The TLM bus reference
/// * `connector_cref` - The connector reference
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if any argument contains an embedded null byte.
pub fn oms_delete_connector_from_tlm_bus(bus_cref: &str, connector_cref: &str) -> i32 {
    let bus = CString::new(bus_cref).expect("bus_cref contains null byte");
    let conn = CString::new(connector_cref).expect("connector_cref contains null byte");
    unsafe { OMSimulator_oms_deleteConnectorFromTLMBus(bus.as_ptr(), conn.as_ptr()) }
}

// --- Export ---

/// Exports a model to a file.
/// Mirrors the `oms_export` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
/// * `filename` - The output filename
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if any argument contains an embedded null byte.
pub fn oms_export(cref: &str, filename: &str) -> i32 {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    let f = CString::new(filename).expect("filename contains null byte");
    unsafe { OMSimulator_oms_export(c_ref.as_ptr(), f.as_ptr()) }
}

/// Exports dependency graphs.
/// Mirrors the `oms_exportDependencyGraphs` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
/// * `initialization` - Initialization graph filename
/// * `event` - Event graph filename
/// * `simulation` - Simulation graph filename
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if any argument contains an embedded null byte.
pub fn oms_export_dependency_graphs(
    cref: &str,
    initialization: &str,
    event: &str,
    simulation: &str,
) -> i32 {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    let i = CString::new(initialization).expect("initialization contains null byte");
    let e = CString::new(event).expect("event contains null byte");
    let s = CString::new(simulation).expect("simulation contains null byte");
    unsafe {
        OMSimulator_oms_exportDependencyGraphs(c_ref.as_ptr(), i.as_ptr(), e.as_ptr(), s.as_ptr())
    }
}

/// Exports the current snapshot.
/// Mirrors the `oms_exportSnapshot` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
///
/// # Returns
/// A tuple of (status_code, snapshot_contents_string).
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_export_snapshot(cref: &str) -> (i32, String) {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    let mut contents_ptr: *const c_char = std::ptr::null();
    let status = unsafe { OMSimulator_oms_exportSnapshot(c_ref.as_ptr(), &mut contents_ptr as *mut _) };
    let contents = if contents_ptr.is_null() {
        String::new()
    } else {
        unsafe {
            CStr::from_ptr(contents_ptr)
                .to_string_lossy()
                .into_owned()
        }
    };
    (status, contents)
}

// --- Inspection ---

/// Extracts FMI kind from an FMU file.
/// Mirrors the `oms_extractFMIKind` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `filename` - Path to the FMU file
///
/// # Returns
/// A tuple of (status_code, kind).
///
/// # Panics
/// Panics if `filename` contains an embedded null byte.
pub fn oms_extract_fmi_kind(filename: &str) -> (i32, i32) {
    let f = CString::new(filename).expect("filename contains null byte");
    let mut kind = 0;
    let status = unsafe { OMSimulator_oms_extractFMIKind(f.as_ptr(), &mut kind) };
    (status, kind)
}

/// Gets a boolean value from a model.
/// Mirrors the `oms_getBoolean` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
///
/// # Returns
/// A tuple of (status_code, value).
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_get_boolean(cref: &str) -> (i32, i32) {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    let mut value = 0;
    let status = unsafe { OMSimulator_oms_getBoolean(c_ref.as_ptr(), &mut value) };
    (status, value)
}

/// Gets the fixed step size.
/// Mirrors the `oms_getFixedStepSize` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
///
/// # Returns
/// A tuple of (status_code, step_size).
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_get_fixed_step_size(cref: &str) -> (i32, f64) {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    let mut step_size = 0.0;
    let status = unsafe { OMSimulator_oms_getFixedStepSize(c_ref.as_ptr(), &mut step_size) };
    (status, step_size)
}

/// Gets an integer value from a model.
/// Mirrors the `oms_getInteger` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
///
/// # Returns
/// A tuple of (status_code, value).
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_get_integer(cref: &str) -> (i32, i32) {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    let mut value = 0;
    let status = unsafe { OMSimulator_oms_getInteger(c_ref.as_ptr(), &mut value) };
    (status, value)
}

/// Gets the model state.
/// Mirrors the `oms_getModelState` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
///
/// # Returns
/// A tuple of (status_code, model_state).
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_get_model_state(cref: &str) -> (i32, i32) {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    let mut model_state = 0;
    let status = unsafe { OMSimulator_oms_getModelState(c_ref.as_ptr(), &mut model_state) };
    (status, model_state)
}

/// Gets a real (float) value from a model.
/// Mirrors the `oms_getReal` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
///
/// # Returns
/// A tuple of (status_code, value).
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_get_real(cref: &str) -> (i32, f64) {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    let mut value = 0.0;
    let status = unsafe { OMSimulator_oms_getReal(c_ref.as_ptr(), &mut value) };
    (status, value)
}

/// Gets the solver type.
/// Mirrors the `oms_getSolver` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
///
/// # Returns
/// A tuple of (status_code, solver).
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_get_solver(cref: &str) -> (i32, i32) {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    let mut solver = 0;
    let status = unsafe { OMSimulator_oms_getSolver(c_ref.as_ptr(), &mut solver) };
    (status, solver)
}

/// Gets the start time.
/// Mirrors the `oms_getStartTime` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
///
/// # Returns
/// A tuple of (status_code, start_time).
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_get_start_time(cref: &str) -> (i32, f64) {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    let mut start_time = 0.0;
    let status = unsafe { OMSimulator_oms_getStartTime(c_ref.as_ptr(), &mut start_time) };
    (status, start_time)
}

/// Gets the stop time.
/// Mirrors the `oms_getStopTime` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
///
/// # Returns
/// A tuple of (status_code, stop_time).
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_get_stop_time(cref: &str) -> (i32, f64) {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    let mut stop_time = 0.0;
    let status = unsafe { OMSimulator_oms_getStopTime(c_ref.as_ptr(), &mut stop_time) };
    (status, stop_time)
}

/// Gets the submodel path.
/// Mirrors the `oms_getSubModelPath` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
///
/// # Returns
/// A tuple of (status_code, path_string).
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_get_sub_model_path(cref: &str) -> (i32, String) {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    let mut path_ptr: *const c_char = std::ptr::null();
    let status = unsafe { OMSimulator_oms_getSubModelPath(c_ref.as_ptr(), &mut path_ptr as *mut _) };
    let path = if path_ptr.is_null() {
        String::new()
    } else {
        unsafe {
            CStr::from_ptr(path_ptr)
                .to_string_lossy()
                .into_owned()
        }
    };
    (status, path)
}

/// Gets the system type.
/// Mirrors the `oms_getSystemType` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
///
/// # Returns
/// A tuple of (status_code, type_).
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_get_system_type(cref: &str) -> (i32, i32) {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    let mut type_ = 0;
    let status = unsafe { OMSimulator_oms_getSystemType(c_ref.as_ptr(), &mut type_) };
    (status, type_)
}

/// Gets tolerance values.
/// Mirrors the `oms_getTolerance` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
///
/// # Returns
/// A tuple of (status_code, absolute_tolerance, relative_tolerance).
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_get_tolerance(cref: &str) -> (i32, f64, f64) {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    let mut abs_tol = 0.0;
    let mut rel_tol = 0.0;
    let status = unsafe {
        OMSimulator_oms_getTolerance(c_ref.as_ptr(), &mut abs_tol, &mut rel_tol)
    };
    (status, abs_tol, rel_tol)
}

/// Gets variable step size information.
/// Mirrors the `oms_getVariableStepSize` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
///
/// # Returns
/// A tuple of (status_code, initial_step_size, minimum_step_size, maximum_step_size).
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_get_variable_step_size(cref: &str) -> (i32, f64, f64, f64) {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    let mut initial = 0.0;
    let mut minimum = 0.0;
    let mut maximum = 0.0;
    let status = unsafe {
        OMSimulator_oms_getVariableStepSize(
            c_ref.as_ptr(),
            &mut initial,
            &mut minimum,
            &mut maximum,
        )
    };
    (status, initial, minimum, maximum)
}

// --- Fault injection ---

/// Injects a fault into a signal.
/// Mirrors the `oms_faultInjection` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `signal` - The signal name
/// * `fault_type` - The fault type
/// * `fault_value` - The fault value
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if `signal` contains an embedded null byte.
pub fn oms_fault_injection(signal: &str, fault_type: i32, fault_value: f64) -> i32 {
    let s = CString::new(signal).expect("signal contains null byte");
    unsafe { OMSimulator_oms_faultInjection(s.as_ptr(), fault_type, fault_value) }
}

// --- Import ---

/// Imports a file, returns the cref.
/// Mirrors the `oms_importFile` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `filename` - The file to import
///
/// # Returns
/// A tuple of (status_code, cref_string).
///
/// # Panics
/// Panics if `filename` contains an embedded null byte.
pub fn oms_import_file(filename: &str) -> (i32, String) {
    let f = CString::new(filename).expect("filename contains null byte");
    let mut cref_ptr: *const c_char = std::ptr::null();
    let status = unsafe { OMSimulator_oms_importFile(f.as_ptr(), &mut cref_ptr as *mut _) };
    let cref = if cref_ptr.is_null() {
        String::new()
    } else {
        unsafe {
            CStr::from_ptr(cref_ptr)
                .to_string_lossy()
                .into_owned()
        }
    };
    (status, cref)
}

/// Imports a snapshot.
/// Mirrors the `oms_importSnapshot` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
/// * `snapshot` - The snapshot data
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if any argument contains an embedded null byte.
pub fn oms_import_snapshot(cref: &str, snapshot: &str) -> i32 {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    let s = CString::new(snapshot).expect("snapshot contains null byte");
    unsafe { OMSimulator_oms_importSnapshot(c_ref.as_ptr(), s.as_ptr()) }
}

// --- Initialization / execution ---

/// Initializes the model.
/// Mirrors the `oms_initialize` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_initialize(cref: &str) -> i32 {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    unsafe { OMSimulator_oms_initialize(c_ref.as_ptr()) }
}

/// Instantiates the model.
/// Mirrors the `oms_instantiate` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_instantiate(cref: &str) -> i32 {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    unsafe { OMSimulator_oms_instantiate(c_ref.as_ptr()) }
}

/// Lists contents of a model.
/// Mirrors the `oms_list` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
///
/// # Returns
/// A tuple of (status_code, contents_string).
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_list(cref: &str) -> (i32, String) {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    let mut contents_ptr: *const c_char = std::ptr::null();
    let status = unsafe { OMSimulator_oms_list(c_ref.as_ptr(), &mut contents_ptr as *mut _) };
    let contents = if contents_ptr.is_null() {
        String::new()
    } else {
        unsafe {
            CStr::from_ptr(contents_ptr)
                .to_string_lossy()
                .into_owned()
        }
    };
    (status, contents)
}

/// Lists unconnected connectors.
/// Mirrors the `oms_listUnconnectedConnectors` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
///
/// # Returns
/// A tuple of (status_code, contents_string).
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_list_unconnected_connectors(cref: &str) -> (i32, String) {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    let mut contents_ptr: *const c_char = std::ptr::null();
    let status =
        unsafe { OMSimulator_oms_listUnconnectedConnectors(c_ref.as_ptr(), &mut contents_ptr as *mut _) };
    let contents = if contents_ptr.is_null() {
        String::new()
    } else {
        unsafe {
            CStr::from_ptr(contents_ptr)
                .to_string_lossy()
                .into_owned()
        }
    };
    (status, contents)
}

/// Loads a snapshot into a new cref.
/// Mirrors the `oms_loadSnapshot` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
/// * `snapshot` - The snapshot data
///
/// # Returns
/// A tuple of (status_code, new_cref_string).
///
/// # Panics
/// Panics if any argument contains an embedded null byte.
pub fn oms_load_snapshot(cref: &str, snapshot: &str) -> (i32, String) {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    let s = CString::new(snapshot).expect("snapshot contains null byte");
    let mut new_cref_ptr: *const c_char = std::ptr::null();
    let status = unsafe {
        OMSimulator_oms_loadSnapshot(
            c_ref.as_ptr(),
            s.as_ptr(),
            &mut new_cref_ptr as *mut _,
        )
    };
    let new_cref = if new_cref_ptr.is_null() {
        String::new()
    } else {
        unsafe {
            CStr::from_ptr(new_cref_ptr)
                .to_string_lossy()
                .into_owned()
        }
    };
    (status, new_cref)
}

/// Creates a new model.
/// Mirrors the `oms_newModel` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The new model reference
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_new_model(cref: &str) -> i32 {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    unsafe { OMSimulator_oms_newModel(c_ref.as_ptr()) }
}

/// Removes signals from results matching a regex.
/// Mirrors the `oms_removeSignalsFromResults` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
/// * `regex` - The regex pattern
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if any argument contains an embedded null byte.
pub fn oms_remove_signals_from_results(cref: &str, regex: &str) -> i32 {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    let r = CString::new(regex).expect("regex contains null byte");
    unsafe { OMSimulator_oms_removeSignalsFromResults(c_ref.as_ptr(), r.as_ptr()) }
}

/// Renames a model.
/// Mirrors the `oms_rename` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - Current reference
/// * `new_cref` - New reference name
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if any argument contains an embedded null byte.
pub fn oms_rename(cref: &str, new_cref: &str) -> i32 {
    let c = CString::new(cref).expect("cref contains null byte");
    let n = CString::new(new_cref).expect("new_cref contains null byte");
    unsafe { OMSimulator_oms_rename(c.as_ptr(), n.as_ptr()) }
}

/// Resets the model.
/// Mirrors the `oms_reset` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_reset(cref: &str) -> i32 {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    unsafe { OMSimulator_oms_reset(c_ref.as_ptr()) }
}

/// Runs a simulation from a file.
/// Mirrors the `oms_RunFile` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `filename` - The simulation file
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if `filename` contains an embedded null byte.
pub fn oms_run_file(filename: &str) -> i32 {
    let f = CString::new(filename).expect("filename contains null byte");
    unsafe { OMSimulator_oms_RunFile(f.as_ptr()) }
}

// --- Configuration (setters) ---

/// Sets a boolean value in a model.
/// Mirrors the `oms_setBoolean` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
/// * `value` - The boolean value (0=false, non-zero=true)
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_set_boolean(cref: &str, value: bool) -> i32 {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    unsafe { OMSimulator_oms_setBoolean(c_ref.as_ptr(), if value { 1 } else { 0 }) }
}

/// Sets a command line option.
/// Mirrors the `oms_setCommandLineOption` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cmd` - The command line option string
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if `cmd` contains an embedded null byte.
pub fn oms_set_command_line_option(cmd: &str) -> i32 {
    let c = CString::new(cmd).expect("cmd contains null byte");
    unsafe { OMSimulator_oms_setCommandLineOption(c.as_ptr()) }
}

/// Sets the fixed step size.
/// Mirrors the `oms_setFixedStepSize` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
/// * `step_size` - The fixed step size
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_set_fixed_step_size(cref: &str, step_size: f64) -> i32 {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    unsafe { OMSimulator_oms_setFixedStepSize(c_ref.as_ptr(), step_size) }
}

/// Sets an integer value in a model.
/// Mirrors the `oms_setInteger` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
/// * `value` - The integer value
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_set_integer(cref: &str, value: i32) -> i32 {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    unsafe { OMSimulator_oms_setInteger(c_ref.as_ptr(), value) }
}

/// Sets the log file.
/// Mirrors the `oms_setLogFile` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `filename` - The log file path
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if `filename` contains an embedded null byte.
pub fn oms_set_log_file(filename: &str) -> i32 {
    let f = CString::new(filename).expect("filename contains null byte");
    unsafe { OMSimulator_oms_setLogFile(f.as_ptr()) }
}

/// Sets the logging interval.
/// Mirrors the `oms_setLoggingInterval` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
/// * `logging_interval` - The logging interval
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_set_logging_interval(cref: &str, logging_interval: f64) -> i32 {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    unsafe { OMSimulator_oms_setLoggingInterval(c_ref.as_ptr(), logging_interval) }
}

/// Sets the logging level.
/// Mirrors the `oms_setLoggingLevel` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `log_level` - The logging level (integer)
///
/// # Returns
/// Status code.
pub fn oms_set_logging_level(log_level: i32) -> i32 {
    unsafe { OMSimulator_oms_setLoggingLevel(log_level) }
}

/// Sets a real (float) value in a model.
/// Mirrors the `oms_setReal` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
/// * `value` - The real value
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_set_real(cref: &str, value: f64) -> i32 {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    unsafe { OMSimulator_oms_setReal(c_ref.as_ptr(), value) }
}

/// Sets real input derivative.
/// Mirrors the `oms_setRealInputDerivative` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
/// * `value` - The derivative value
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_set_real_input_derivative(cref: &str, value: f64) -> i32 {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    unsafe { OMSimulator_oms_setRealInputDerivative(c_ref.as_ptr(), value) }
}

/// Sets the result file.
/// Mirrors the `oms_setResultFile` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
/// * `filename` - The result file path
/// * `buffer_size` - The buffer size
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if any argument contains an embedded null byte.
pub fn oms_set_result_file(cref: &str, filename: &str, buffer_size: i32) -> i32 {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    let f = CString::new(filename).expect("filename contains null byte");
    unsafe { OMSimulator_oms_setResultFile(c_ref.as_ptr(), f.as_ptr(), buffer_size) }
}

/// Sets the signal filter (regex).
/// Mirrors the `oms_setSignalFilter` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
/// * `regex` - The regex pattern
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if any argument contains an embedded null byte.
pub fn oms_set_signal_filter(cref: &str, regex: &str) -> i32 {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    let r = CString::new(regex).expect("regex contains null byte");
    unsafe { OMSimulator_oms_setSignalFilter(c_ref.as_ptr(), r.as_ptr()) }
}

/// Sets the solver type.
/// Mirrors the `oms_setSolver` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
/// * `solver` - The solver type
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_set_solver(cref: &str, solver: i32) -> i32 {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    unsafe { OMSimulator_oms_setSolver(c_ref.as_ptr(), solver) }
}

/// Sets the start time.
/// Mirrors the `oms_setStartTime` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
/// * `start_time` - The start time
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_set_start_time(cref: &str, start_time: f64) -> i32 {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    unsafe { OMSimulator_oms_setStartTime(c_ref.as_ptr(), start_time) }
}

/// Sets the stop time.
/// Mirrors the `oms_setStopTime` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
/// * `stop_time` - The stop time
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_set_stop_time(cref: &str, stop_time: f64) -> i32 {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    unsafe { OMSimulator_oms_setStopTime(c_ref.as_ptr(), stop_time) }
}

/// Sets the temporary directory.
/// Mirrors the `oms_setTempDirectory` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `new_temp_dir` - The new temporary directory path
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if `new_temp_dir` contains an embedded null byte.
pub fn oms_set_temp_directory(new_temp_dir: &str) -> i32 {
    let d = CString::new(new_temp_dir).expect("new_temp_dir contains null byte");
    unsafe { OMSimulator_oms_setTempDirectory(d.as_ptr()) }
}

/// Sets TLM position and orientation.
/// Mirrors the `oms_setTLMPositionAndOrientation` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
/// * `x1`, `x2`, `x3` - Position coordinates
/// * `A11`..`A33` - 3x3 rotation matrix elements
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
#[allow(clippy::too_many_arguments)]
pub fn oms_set_tlm_position_and_orientation(
    cref: &str,
    x1: f64,
    x2: f64,
    x3: f64,
    a11: f64,
    a12: f64,
    a13: f64,
    a21: f64,
    a22: f64,
    a23: f64,
    a31: f64,
    a32: f64,
    a33: f64,
) -> i32 {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    unsafe {
        OMSimulator_oms_setTLMPositionAndOrientation(
            c_ref.as_ptr(), x1, x2, x3, a11, a12, a13, a21, a22, a23, a31, a32, a33,
        )
    }
}

/// Sets TLM socket data.
/// Mirrors the `oms_setTLMSocketData` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
/// * `address` - The socket address
/// * `manager_port` - The manager port
/// * `monitor_port` - The monitor port
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if any argument contains an embedded null byte.
pub fn oms_set_tlm_socket_data(cref: &str, address: &str, manager_port: i32, monitor_port: i32) -> i32 {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    let addr = CString::new(address).expect("address contains null byte");
    unsafe {
        OMSimulator_oms_setTLMSocketData(c_ref.as_ptr(), addr.as_ptr(), manager_port, monitor_port)
    }
}

/// Sets tolerance values.
/// Mirrors the `oms_setTolerance` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
/// * `absolute_tolerance` - The absolute tolerance
/// * `relative_tolerance` - The relative tolerance
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_set_tolerance(cref: &str, absolute_tolerance: f64, relative_tolerance: f64) -> i32 {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    unsafe {
        OMSimulator_oms_setTolerance(c_ref.as_ptr(), absolute_tolerance, relative_tolerance)
    }
}

/// Sets variable step size parameters.
/// Mirrors the `oms_setVariableStepSize` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
/// * `initial_step_size` - The initial step size
/// * `minimum_step_size` - The minimum step size
/// * `maximum_step_size` - The maximum step size
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_set_variable_step_size(
    cref: &str,
    initial_step_size: f64,
    minimum_step_size: f64,
    maximum_step_size: f64,
) -> i32 {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    unsafe {
        OMSimulator_oms_setVariableStepSize(
            c_ref.as_ptr(),
            initial_step_size,
            minimum_step_size,
            maximum_step_size,
        )
    }
}

/// Sets the working directory.
/// Mirrors the `oms_setWorkingDirectory` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `new_working_dir` - The new working directory path
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if `new_working_dir` contains an embedded null byte.
pub fn oms_set_working_directory(new_working_dir: &str) -> i32 {
    let d = CString::new(new_working_dir).expect("new_working_dir contains null byte");
    unsafe { OMSimulator_oms_setWorkingDirectory(d.as_ptr()) }
}

// --- Simulation control ---

/// Runs a simulation on a model.
/// Mirrors the `oms_simulate` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_simulate(cref: &str) -> i32 {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    unsafe { OMSimulator_oms_simulate(c_ref.as_ptr()) }
}

/// Steps the simulation until the given stop time.
/// Mirrors the `oms_stepUntil` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
/// * `stop_time` - The stop time to step until
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_step_until(cref: &str, stop_time: f64) -> i32 {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    unsafe { OMSimulator_oms_stepUntil(c_ref.as_ptr(), stop_time) }
}

/// Terminates the model.
/// Mirrors the `oms_terminate` function from OMSimulatorExt.mo.
///
/// # Parameters
/// * `cref` - The model reference
///
/// # Returns
/// Status code.
///
/// # Panics
/// Panics if `cref` contains an embedded null byte.
pub fn oms_terminate(cref: &str) -> i32 {
    let c_ref = CString::new(cref).expect("cref contains null byte");
    unsafe { OMSimulator_oms_terminate(c_ref.as_ptr()) }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_to_string_known() {
        assert_eq!(status_to_string(0), "ok");
        assert_eq!(status_to_string(1), "warning");
        assert_eq!(status_to_string(2), "discard");
        assert_eq!(status_to_string(3), "error");
        assert_eq!(status_to_string(4), "fatal");
        assert_eq!(status_to_string(5), "pending");
    }

    #[test]
    fn test_status_to_string_unknown() {
        assert_eq!(status_to_string(-1), "unknown_status");
        assert_eq!(status_to_string(100), "unknown_status");
    }

    #[test]
    fn test_boolean_conversion() {
        // Verify the boolean-to-int conversion logic matches
        assert_eq!(if true { 1 } else { 0 }, 1);
        assert_eq!(if false { 1 } else { 0 }, 0);
    }
}
