# Corba Module - Translation Assumptions and Notes

## Source
`Util/Corba.mo` - CORBA communication module for the OpenModelica compiler

## Translation Summary

All 6 public functions from the MetaModelica Corba package have been translated
to Rust wrapper functions that call the underlying C library (`omcruntime` /
`OpenModelicaCorba`).

## Assumptions

### External C Function Signatures
Since the C headers were not available, the following signatures were assumed:

- `Corba_haveCorba() -> int` - returns 0/1 for false/true
- `Corba_setObjectReferenceFilePath(char*)` - takes a C string
- `Corba_setSessionName(char*)` - takes a C string
- `Corba_initialize()` - no arguments, no return value
- `Corba_waitForCommand() -> char*` - returns an allocated C string
- `Corba_sendreply(char*)` - takes a C string
- `Corba_close()` - no arguments, no return value

### String Handling
- **Input strings** (`&str`): Converted to `CString` before passing to C.
  If the input contains a NUL byte, the function will panic with a descriptive
  message. This is the standard Rust pattern for FFI string passing.

- **Output strings** (`String` from `wait_for_command()`): The C function is
  assumed to return a heap-allocated C string. This string is read as a
  `CStr` and converted to a Rust `String`. However, the memory is never freed
  because no corresponding `free` function is declared.

- **Return type `Boolean`**: Mapped to `bool` in Rust. The C return value is
  compared against 0.

### Platform Differences
The original Moduleica code notes that the Windows and Unix implementations
differ via C ifdefs. This Rust translation provides a uniform interface that
works regardless of platform - the platform-specific behavior is entirely in
the C library.

## Things That Might Not Work

### Memory Leak in `wait_for_command()`
The most significant issue is that `wait_for_command()` returns a `String` but
never frees the underlying C allocation. The C side allocates the string but
no free function is exposed in the FFI interface. Potential solutions:
1. Add a `Corba_freeString(char*)` extern declaration if the C library provides one
2. Use `libc::free` if the C library uses standard malloc/free
3. Return `*mut c_char` and let the caller manage the lifetime

### No Error Handling
The original MetaModelica functions have no error handling - they are bare
external calls. The Rust translation matches this behavior. If the C library
throws exceptions or sets error states, those are not captured.

### No CORBA Initialization Check
Calling `initialize()` without first checking `have_corba()` is valid (matching
MetaModelica behavior), but may cause undefined behavior if no CORBA runtime
is available at all.

### Build Linking
The crate cannot be fully linked without the `omcruntime` and
`OpenModelicaCorba` libraries. `cargo check` (type checking) works, but
`cargo test` (linking) will fail unless these libraries are available.

### `sendreply` Function Name
The function `sendreply` was kept as-is (not renamed to `send_reply`) because
it follows the original MetaModelica naming and there's no clear indication
that it should be split. However, snake_case convention would suggest
`send_reply`.
