# Assumptions and Notes for src/zeromq.rs

## Assumptions

1. **`Option<Integer>` type**: Mapped to `Option<i32>`. In MetaModelica, `Option` is a variant type where `NONE` and `SOME(value)` wrap an `Integer`. The C function `ZeroMQ_initialize` returns an integer handle (likely a file descriptor or pointer cast to int), where a negative/invalid value means `NONE`. The other functions accept an optional socket handle.

2. **Return value of `ZeroMQ_initialize`**: Assumed to return a non-negative integer for `SOME(fd)` and a negative value for `NONE`. If the C API differs (e.g., returns `NULL`/`0` for none), adjust the `initialize` function accordingly.

3. **String ownership from C**: `ZeroMQ_handleRequest` returns a C-allocated `char*`. The Rust code calls `CStr::from_ptr` and converts to `String`, but **does not free the C allocation**. The caller is responsible for calling a C free function (e.g., `free()` or a custom deallocator) if the C API provides one.

4. **`Option<Integer>` parameter for request-handling functions**: The C functions `ZeroMQ_handleRequest`, `ZeroMQ_sendReply`, and `ZeroMQ_close` likely accept a raw pointer or handle. In the wrapper, `Option<i32>` is cast to `*mut c_void` when passed to C. If the C API expects an `int` instead of a pointer, change the extern declarations and casts.

5. **`Boolean` mapping**: `Boolean` in MetaModelica maps to `bool` in Rust. The `listenToAll` parameter is passed as `1` or `0` to the C function (as `c_int`).

6. **`String` parameter to `sendReply`**: The C function expects a `const char*`. The Rust code converts via `CString`. If the string could contain NUL bytes, `CString::new` will error - this is unlikely for protocol strings but worth noting.

## Things That Might Not Work as Expected

- **No C library linkage**: This module requires linking against `omcruntime` which contains the actual C implementations (`ZeroMQ_initialize`, etc.). Without this library, the extern declarations will fail at link time.
- **Thread safety**: The C functions may or may not be thread-safe. No synchronization is added in the Rust wrappers.
- **Error handling**: The C functions do not return `Result` types in the C API. The Rust wrappers return `String` or `Option<i32>` without propagating C-level errors (e.g., `errno`). For production use, consider checking `errno` or adding error-returning variants of the C functions.
- **Memory leaks**: The string returned from `handleRequest` is not freed. If the C allocation must be freed, add a corresponding `ZeroMQ_freeString` extern declaration or use a custom free function.
