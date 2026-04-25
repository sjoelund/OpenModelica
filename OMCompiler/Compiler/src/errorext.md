# Assumptions and Notes for errorext.rs

## Source
Translated from `Util/ErrorExt.mo` (OpenModelica).

## Assumptions

### Thread Data
- `OpenModelica.threadData()` maps to an extern "C" function `OpenModelica_threadData() -> *mut c_void`.
  This function is expected to exist in the `omcruntime` C library at runtime.

### C Library
- All functions link against the `omcrruntime` C library. The C symbol names match the function names in the .mo file (e.g., `Error_addSourceMessage`, `ErrorImpl__clearMessages`).

### String Handling
- All `String` inputs are converted to `CString` before passing to FFI. This panics if the string contains an embedded null byte.
- String outputs from C functions (returning `*const c_char`) are converted via `CStr::from_ptr` with `to_string_lossy()`. If the returned pointer is null, an empty string is returned.

### Integer Types
- `Integer` in MetaModelica maps to `i32` in Rust.
- `Boolean` maps to `c_int` in FFI and `bool` in the safe wrappers.
- `ErrorTypes.ErrorID` maps to `i32` (consistent with `errortypes.rs`).
- `ErrorTypes.MessageType` and `ErrorTypes.Severity` are cast to `c_int` for FFI.

### List Types
- `list<String>` and `list<Integer>` inputs are passed as opaque `*mut c_void` pointers via helper functions (`list_to_ptr`, `list_of_ints_to_ptr`). These helpers currently return null pointers as stubs, since the actual C representation of lists in omcruntime is not fully known.
- `list<ErrorTypes.TotalMessage>` outputs (from `get_messages` and `get_checkpoint_messages`) currently return empty lists as stubs, since the actual C representation is not fully known.
- `list<Integer>` outputs (from `pop_check_point`) similarly return empty lists as stubs.

### Checkpoint Handles
- `pop_check_point` returns `List<i32>` as opaque handles. These MUST be passed back to `push_messages` or `free_messages` to avoid memory leaks, matching the MetaModelica documentation.

### Functions with No Output
- `register_modelica_format_error`, `clear_messages`, `set_checkpoint`, `del_checkpoint`, `roll_back`, `push_messages`, `free_messages`, `set_show_error_messages`, `move_messages_to_parent_thread`, and `init_assertion_functions` return `()` (nothing), matching their MetaModelica signatures.

### Deprecated / Known Limitations
- **List handling is a stub**: The `list_to_ptr` and `list_of_ints_to_ptr` helpers return null pointers. This means functions that pass lists as arguments (`add_source_message`, `push_messages`, `free_messages`) or return lists (`get_messages`, `get_checkpoint_messages`, `pop_check_point`) will not work correctly without proper list marshalling.
- **No actual omcruntime library linked**: At compile time, the C library symbols are unresolved. This will fail at link time unless `omcruntime` is provided.
- **Thread data may be null**: In test environments without the full OpenModelica runtime, `OpenModelica_threadData()` may return null, which could cause segfaults in the C functions.

### Tests
- The included unit tests verify that the functions can be called (compilation check) but do not validate runtime behavior, as this requires the full OpenModelica runtime.
