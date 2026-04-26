# StackOverflow Module - Assumptions and Notes

## Assumptions

1. **External C functions**: The following functions are bound to external C symbols and are stubbed for test builds:
   - `mmc_getStacktraceMessages_threadData`
   - `mmc_setStacktraceMessages_threadData`
   - `mmc_hasStacktraceMessages`
   - `mmc_clearStacktraceMessages`
   - `mmc_do_stackoverflow`
   - `OpenModelica.threadData()`

   At runtime, these will link against the `omcruntime` C library. During `cargo test`, they are replaced with no-op stubs.

2. **Testsuite.isRunning()**: The `Testsuite` module is not yet translated. The `is_running()` function is stubbed to return `false`, meaning normal execution path is taken during tests.

3. **System.regex**: The MetaModelica `System.regex` function uses POSIX extended regular expressions. The Rust equivalent uses the `regex` crate. The regex patterns were adapted:
   - Linux format: `[[]` and `[]]` in MetaModelica (literal brackets) become `\[` and `\]` in Rust regex.
   - Both regexes are case-sensitive by default (matching MetaModelica behavior).

4. **OpenModelica.threadData()**: Returns a pointer to the current thread's data structure. This is a C runtime concept. The Rust stub returns `null`.

5. **List<T>**: Uses `im::Vector<T>` for immutable list semantics. The order is reversed when constructing via `push_front` and then reversed at the end via `list_reverse`.

6. **1-based indexing**: MetaModelica uses 1-based indexing for `substring`, but the Rust `substring` helper uses 0-based byte indexing for compatibility with Rust byte slicing.

7. **String(Integer(x))**: Converting `Integer` to `String` via `format!("{}", x)`.

8. **String(Real, significantDigits=n)**: Not used in this module.

## Things that might not work as expected

1. **Runtime C linkage**: The functions `mmc_*` require the `omcruntime` C library to be linked. Without it, these functions will fail at link time or runtime. The `#[cfg(not(test))]` / `#[cfg(test)]` conditional compilation handles the test case.

2. **Stack trace capture**: The stack trace capture mechanism depends on C runtime support. Without the full OpenModelica runtime, `get_stacktrace_messages()` returns an empty list.

3. **macOS regex pattern**: The OSX stack trace regex pattern may need tuning for different macOS versions or architectures. Test with real stack traces to verify.

4. **Thread safety**: The external C functions likely rely on thread-local storage in the C runtime. The Rust `thread_data()` stub returns `null`, so these functions will not work correctly without proper thread data.

5. **Concurrent access**: No synchronization is implemented for the stack trace state. If multiple threads access these functions concurrently, behavior may be undefined.

6. **Memory safety**: The `unsafe` blocks around FFI calls assume the C runtime maintains valid memory. Without proper synchronization and lifetime management, use after free is possible.
