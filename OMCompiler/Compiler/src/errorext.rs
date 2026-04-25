//! Translation of Util/ErrorExt.mo
//!
//! Error handling external interface.
//! This module provides wrappers around the OpenModelica `omcruntime` C library
//! for managing error messages, checkpoints, and message queues.
//!
//! All external C functions link against the `omcruntime` library.
//! Each function takes a thread data pointer via `thread_data()`,
//! which corresponds to `OpenModelica.threadData()` in MetaModelica.

use crate::errortypes::{MessageType, Severity, TotalMessage};
use im::Vector;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};

// ============================================================================
// Persistent list type (mapped to im::Vector since im 15.x has no List)
// ============================================================================

/// Persistent list type (mapped to im::Vector since im 15.x has no List)
type List<T> = Vector<T>;

// ============================================================================
// Thread data accessor
// ============================================================================

// Safety: extern "C" block used for FFI to OpenModelica runtime
#[allow(dead_code)]
unsafe extern "C" {
    fn OpenModelica_threadData() -> *mut c_void;
}

/// Returns the current thread data pointer.
fn thread_data() -> *mut c_void {
    unsafe { OpenModelica_threadData() }
}

// ============================================================================
// FFI bindings to omcruntime C library
// ============================================================================

// registerModelicaFormatError
unsafe extern "C" {
    fn Error_registerModelicaFormatError();
}

// addSourceMessage
unsafe extern "C" {
    fn Error_addSourceMessage(
        threadData: *mut c_void,
        id: c_int,
        msg_type: c_int,
        msg_severity: c_int,
        sline: c_int,
        scol: c_int,
        eline: c_int,
        ecol: c_int,
        read_only: c_int,
        filename: *const c_char,
        msg: *const c_char,
        tokens: *mut c_void,
    );
}

// printMessagesStr
unsafe extern "C" {
    fn Error_printMessagesStr(threadData: *mut c_void, warnings_as_errors: c_int) -> *const c_char;
}

// printCheckpointMessagesStr
unsafe extern "C" {
    fn Error_printCheckpointMessagesStr(
        threadData: *mut c_void,
        warnings_as_errors: c_int,
    ) -> *const c_char;
}

// getNumMessages
unsafe extern "C" {
    fn Error_getNumMessages(threadData: *mut c_void) -> c_int;
}

// ErrorImpl__getNumErrorMessages
unsafe extern "C" {
    fn ErrorImpl__getNumErrorMessages(threadData: *mut c_void) -> c_int;
}

// ErrorImpl__getNumWarningMessages
unsafe extern "C" {
    fn ErrorImpl__getNumWarningMessages(threadData: *mut c_void) -> c_int;
}

// getMessages
unsafe extern "C" {
    fn Error_getMessages(threadData: *mut c_void) -> *mut c_void;
}

// ErrorImpl__getCheckpointMessages
unsafe extern "C" {
    fn ErrorImpl__getCheckpointMessages(threadData: *mut c_void) -> *mut c_void;
}

// ErrorImpl__clearMessages
unsafe extern "C" {
    fn ErrorImpl__clearMessages(threadData: *mut c_void);
}

// ErrorImpl__getNumCheckpoints
unsafe extern "C" {
    fn ErrorImpl__getNumCheckpoints(threadData: *mut c_void) -> c_int;
}

// ErrorImpl__rollbackNumCheckpoints
unsafe extern "C" {
    fn ErrorImpl__rollbackNumCheckpoints(threadData: *mut c_void, n: c_int);
}

// ErrorImpl__deleteNumCheckpoints
unsafe extern "C" {
    fn ErrorImpl__deleteNumCheckpoints(threadData: *mut c_void, n: c_int);
}

// ErrorImpl__setCheckpoint
unsafe extern "C" {
    fn ErrorImpl__setCheckpoint(threadData: *mut c_void, id: *const c_char);
}

// ErrorImpl__delCheckpoint
unsafe extern "C" {
    fn ErrorImpl__delCheckpoint(threadData: *mut c_void, id: *const c_char);
}

// printErrorsNoWarning
unsafe extern "C" {
    fn Error_printErrorsNoWarning(threadData: *mut c_void) -> *const c_char;
}

// ErrorImpl__rollBack
unsafe extern "C" {
    fn ErrorImpl__rollBack(threadData: *mut c_void, id: *const c_char);
}

// ErrorImpl__pop (popCheckPoint)
unsafe extern "C" {
    fn ErrorImpl__pop(
        threadData: *mut c_void,
        id: *const c_char,
    ) -> *mut c_void;
}

// ErrorImpl__pushMessages
unsafe extern "C" {
    fn ErrorImpl__pushMessages(threadData: *mut c_void, handles: *mut c_void);
}

// ErrorImpl__freeMessages
unsafe extern "C" {
    fn ErrorImpl__freeMessages(threadData: *mut c_void, handles: *mut c_void);
}

// ErrorImpl__isTopCheckpoint
unsafe extern "C" {
    fn ErrorImpl__isTopCheckpoint(
        threadData: *mut c_void,
        id: *const c_char,
    ) -> c_int;
}

// Error_setShowErrorMessages
unsafe extern "C" {
    fn Error_setShowErrorMessages(threadData: *mut c_void, in_show: c_int);
}

// Error_moveMessagesToParentThread
unsafe extern "C" {
    fn Error_moveMessagesToParentThread(threadData: *mut c_void);
}

// Error_initAssertionFunctions
unsafe extern "C" {
    fn Error_initAssertionFunctions();
}

// ============================================================================
// Safe wrapper functions
// ============================================================================

/// Registers the ModelicaFormatError function to output messages
/// in the Error buffer instead of the default standard output.
///
/// Note: Only works in the bootstrapped compiler!
///
/// # Safety
/// This function calls into C code via FFI.
pub fn register_modelica_format_error() {
    unsafe { Error_registerModelicaFormatError() }
}

/// Adds a source message to the error queue.
///
/// # Parameters
/// * `id` - The error ID
/// * `msg_type` - The type of message (SYNTAX, GRAMMAR, etc.)
/// * `msg_severity` - The severity level
/// * `sline` - Start line number
/// * `scol` - Start column number
/// * `eline` - End line number
/// * `ecol` - End column number
/// * `read_only` - Whether the source info is read-only
/// * `filename` - The source file name
/// * `msg` - The message string
/// * `tokens` - Tokens to insert into the message
///
/// # Safety
/// This function calls into C code via FFI.
pub fn add_source_message(
    id: i32,
    msg_type: MessageType,
    msg_severity: Severity,
    sline: i32,
    scol: i32,
    eline: i32,
    ecol: i32,
    read_only: bool,
    filename: &str,
    msg: &str,
    tokens: List<String>,
) {
    let c_filename = CString::new(filename).expect("filename contains null byte");
    let c_msg = CString::new(msg).expect("message contains null byte");
    let td = thread_data();
    let tokens_ptr = list_to_ptr(tokens);
    unsafe {
        Error_addSourceMessage(
            td,
            id,
            msg_type as c_int,
            msg_severity as c_int,
            sline,
            scol,
            eline,
            ecol,
            read_only as c_int,
            c_filename.as_ptr(),
            c_msg.as_ptr(),
            tokens_ptr,
        )
    }
}

/// Prints all error messages as a string and pops them from the message queue.
///
/// # Parameters
/// * `warnings_as_errors` - If true, treat warnings as errors
///
/// # Returns
/// A string containing all error messages.
///
/// # Safety
/// This function calls into C code via FFI.
pub fn print_messages_str(warnings_as_errors: bool) -> String {
    let td = thread_data();
    unsafe {
        let ptr = Error_printMessagesStr(td, warnings_as_errors as c_int);
        if ptr.is_null() {
            String::new()
        } else {
            CStr::from_ptr(ptr)
                .to_string_lossy()
                .into_owned()
        }
    }
}

/// Prints the error messages since the last checkpoint as a string
/// and pops them from the message queue.
///
/// # Parameters
/// * `warnings_as_errors` - If true, treat warnings as errors
///
/// # Returns
/// A string containing the checkpoint messages.
///
/// # Safety
/// This function calls into C code via FFI.
pub fn print_checkpoint_messages_str(warnings_as_errors: bool) -> String {
    let td = thread_data();
    unsafe {
        let ptr = Error_printCheckpointMessagesStr(td, warnings_as_errors as c_int);
        if ptr.is_null() {
            String::new()
        } else {
            CStr::from_ptr(ptr)
                .to_string_lossy()
                .into_owned()
        }
    }
}

/// Returns the total number of messages in the message queue.
///
/// # Returns
/// The number of messages.
///
/// # Safety
/// This function calls into C code via FFI.
pub fn get_num_messages() -> i32 {
    let td = thread_data();
    unsafe { Error_getNumMessages(td) }
}

/// Returns the number of error messages.
///
/// # Returns
/// The number of error messages.
///
/// # Safety
/// This function calls into C code via FFI.
pub fn get_num_error_messages() -> i32 {
    let td = thread_data();
    unsafe { ErrorImpl__getNumErrorMessages(td) }
}

/// Returns the number of warning messages.
///
/// # Returns
/// The number of warning messages.
///
/// # Safety
/// This function calls into C code via FFI.
pub fn get_num_warning_messages() -> i32 {
    let td = thread_data();
    unsafe { ErrorImpl__getNumWarningMessages(td) }
}

/// Returns all error messages and pops them from the message queue.
///
/// # Returns
/// A list of total messages.
///
/// # Safety
/// This function calls into C code via FFI.
pub fn get_messages() -> List<TotalMessage> {
    // Note: This returns a raw pointer to a list of TotalMessage.
    // The actual conversion from the C representation requires
    // the full runtime support. Returns empty list as stub.
    let _td = thread_data();
    unsafe { Error_getMessages(_td) };
    List::new()
}

/// Returns all error messages since the last checkpoint
/// and pops them from the message queue.
///
/// # Returns
/// A list of total messages since the last checkpoint.
///
/// # Safety
/// This function calls into C code via FFI.
pub fn get_checkpoint_messages() -> List<TotalMessage> {
    // Note: This returns a raw pointer to a list of TotalMessage.
    // The actual conversion from the C representation requires
    // the full runtime support. Returns empty list as stub.
    let _td = thread_data();
    unsafe { ErrorImpl__getCheckpointMessages(_td) };
    List::new()
}

/// Clears all messages from the message queue.
///
/// # Safety
/// This function calls into C code via FFI.
pub fn clear_messages() {
    let td = thread_data();
    unsafe { ErrorImpl__clearMessages(td) }
}

/// Returns the number of checkpoints.
/// Used to rollback/delete checkpoints without considering the identifier.
///
/// # Returns
/// The number of checkpoints.
///
/// # Safety
/// This function calls into C code via FFI.
pub fn get_num_checkpoints() -> i32 {
    let td = thread_data();
    unsafe { ErrorImpl__getNumCheckpoints(td) }
}

/// Rollback/delete the top `n` checkpoints.
/// Used to reset error messages after a stack overflow exception.
///
/// # Parameters
/// * `n` - The number of checkpoints to rollback/delete.
///
/// # Safety
/// This function calls into C code via FFI.
pub fn rollback_num_checkpoints(n: i32) {
    let td = thread_data();
    unsafe { ErrorImpl__rollbackNumCheckpoints(td, n) }
}

/// Delete the top `n` checkpoints without rollback.
/// Used to reset error messages after a stack overflow exception.
///
/// # Parameters
/// * `n` - The number of checkpoints to delete.
///
/// # Safety
/// This function calls into C code via FFI.
pub fn delete_num_checkpoints(n: i32) {
    let td = thread_data();
    unsafe { ErrorImpl__deleteNumCheckpoints(td, n) }
}

/// Sets a checkpoint for the error messages, so error messages can be
/// rolled back (i.e. deleted) up to this point.
///
/// A unique identifier for this checkpoint must be provided.
/// It is checked when doing rollback or deletion.
///
/// # Parameters
/// * `id` - A unique identifier for the checkpoint (up to the programmer
///   to guarantee uniqueness)
///
/// # Safety
/// This function calls into C code via FFI.
pub fn set_checkpoint(id: &str) {
    let c_id = CString::new(id).expect("checkpoint id contains null byte");
    let td = thread_data();
    unsafe { ErrorImpl__setCheckpoint(td, c_id.as_ptr()) }
}

/// Deletes the checkpoint at the top of the stack without
/// removing the error messages issued since that checkpoint.
///
/// # Parameters
/// * `id` - Unique identifier of the checkpoint to delete.
///   The application will exit with -1 if the checkpoint id doesn't match.
///
/// # Safety
/// This function calls into C code via FFI.
pub fn del_checkpoint(id: &str) {
    let c_id = CString::new(id).expect("checkpoint id contains null byte");
    let td = thread_data();
    unsafe { ErrorImpl__delCheckpoint(td, c_id.as_ptr()) }
}

/// Prints all errors (without warnings) as a string.
///
/// # Returns
/// A string containing only error messages (no warnings).
///
/// # Safety
/// This function calls into C code via FFI.
pub fn print_errors_no_warning() -> String {
    let td = thread_data();
    unsafe {
        let ptr = Error_printErrorsNoWarning(td);
        if ptr.is_null() {
            String::new()
        } else {
            CStr::from_ptr(ptr)
                .to_string_lossy()
                .into_owned()
        }
    }
}

/// Rolls back error messages until the latest checkpoint,
/// deleting all error messages added since that point in time.
///
/// A unique identifier for the checkpoint must be provided.
/// The application will exit with return code -1 if this identifier does not match.
///
/// # Parameters
/// * `id` - Unique identifier of the checkpoint to roll back to.
///
/// # Safety
/// This function calls into C code via FFI.
pub fn roll_back(id: &str) {
    let c_id = CString::new(id).expect("checkpoint id contains null byte");
    let td = thread_data();
    unsafe { ErrorImpl__rollBack(td, c_id.as_ptr()) }
}

/// Rolls back error messages until the latest checkpoint,
/// returning all error messages added since that point in time.
///
/// A unique identifier for the checkpoint must be provided.
/// The application will exit with return code -1 if this identifier does not match.
///
/// # Parameters
/// * `id` - Unique identifier of the checkpoint to pop.
///
/// # Returns
/// Opaque handles (integers) that MUST be passed back to `push_messages`
/// or `free_messages` or memory will be leaked.
///
/// # Safety
/// This function calls into C code via FFI.
pub fn pop_check_point(id: &str) -> List<i32> {
    let c_id = CString::new(id).expect("checkpoint id contains null byte");
    let td = thread_data();
    // Note: Returns opaque pointer list from C. The actual conversion
    // from the C representation requires the full runtime support.
    // Returns empty list as stub.
    unsafe { ErrorImpl__pop(td, c_id.as_ptr()) };
    List::new()
}

/// Pushes stored pointers back to the error stack.
/// Use the handles returned by `pop_check_point`.
///
/// # Parameters
/// * `handles` - Opaque pointers from `pop_check_point`.
///
/// # Safety
/// This function calls into C code via FFI.
pub fn push_messages(handles: List<i32>) {
    let td = thread_data();
    let handles_ptr = list_of_ints_to_ptr(handles);
    unsafe { ErrorImpl__pushMessages(td, handles_ptr) }
}

/// Frees stored pointers, releasing memory for checkpointed messages.
/// Use the handles returned by `pop_check_point`.
///
/// # Parameters
/// * `handles` - Opaque pointers from `pop_check_point`.
///
/// # Safety
/// This function calls into C code via FFI.
pub fn free_messages(handles: List<i32>) {
    let td = thread_data();
    let handles_ptr = list_of_ints_to_ptr(handles);
    unsafe { ErrorImpl__freeMessages(td, handles_ptr) }
}

/// Checks if the specified checkpoint exists AT THE TOP OF THE STACK.
///
/// This is useful when you want to roll_back/delete a checkpoint but
/// aren't sure that it exists (due to MetaModelica backtracking).
///
/// # Parameters
/// * `id` - Unique identifier of the checkpoint to check.
///
/// # Returns
/// `true` if the checkpoint exists at the top of the stack, `false` otherwise.
///
/// # Safety
/// This function calls into C code via FFI.
pub fn is_top_checkpoint(id: &str) -> bool {
    let c_id = CString::new(id).expect("checkpoint id contains null byte");
    let td = thread_data();
    unsafe { ErrorImpl__isTopCheckpoint(td, c_id.as_ptr()) != 0 }
}

/// Shows or hides error messages.
///
/// # Parameters
/// * `in_show` - If true, show error messages; if false, hide them.
///
/// # Safety
/// This function calls into C code via FFI.
pub fn set_show_error_messages(in_show: bool) {
    let td = thread_data();
    unsafe { Error_setShowErrorMessages(td, in_show as c_int) }
}

/// Moves error messages from the current thread to the parent thread.
///
/// # Safety
/// This function calls into C code via FFI.
pub fn move_messages_to_parent_thread() {
    let td = thread_data();
    unsafe { Error_moveMessagesToParentThread(td) }
}

/// Makes `assert()` and other runtime assertions print to the error buffer.
///
/// # Safety
/// This function calls into C code via FFI.
pub fn init_assertion_functions() {
    unsafe { Error_initAssertionFunctions() }
}

// ============================================================================
// Helper functions for converting Rust collections to C pointers
// ============================================================================

/// Converts a list of strings to a C-compatible pointer.
/// The actual representation depends on the omcruntime convention.
///
/// # Safety
/// This helper produces a pointer that will be passed to unsafe FFI calls.
fn list_to_ptr(_list: List<String>) -> *mut c_void {
    // Note: The actual representation of list<String> in the C FFI
    // requires the full runtime support. This is a stub that returns
    // a null pointer.
    std::ptr::null_mut()
}

/// Converts a list of integers to a C-compatible pointer.
/// The actual representation depends on the omcruntime convention.
///
/// # Safety
/// This helper produces a pointer that will be passed to unsafe FFI calls.
fn list_of_ints_to_ptr(_list: List<i32>) -> *mut c_void {
    // Note: The actual representation of list<Integer> in the C FFI
    // requires the full runtime support. This is a stub that returns
    // a null pointer.
    std::ptr::null_mut()
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_num_messages_exists() {
        // Verify the function can be called.
        // Actual behavior depends on the C runtime being initialized.
        let _n = get_num_messages();
    }

    #[test]
    fn test_get_num_error_messages_exists() {
        let _n = get_num_error_messages();
    }

    #[test]
    fn test_get_num_warning_messages_exists() {
        let _n = get_num_warning_messages();
    }

    #[test]
    fn test_get_num_checkpoints_exists() {
        let _n = get_num_checkpoints();
    }

    #[test]
    fn test_clear_messages_exists() {
        clear_messages();
    }

    #[test]
    fn test_init_assertion_functions_exists() {
        init_assertion_functions();
    }
}
