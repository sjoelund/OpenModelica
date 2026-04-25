# Gettext Translation Notes

## Assumptions

1. **Thread data access**: The translation uses `OpenModelica_threadData()` to get a thread data pointer, consistent with other translated modules like `print.rs`. The actual C function signature for `System_gettext` was inferred from the MetaModelica interface (`System.gettext(input String msgid; output String msgstr)`). The actual C binding may need adjustment based on the real `omcruntime` header definitions.

2. **FFI function name**: The C function is assumed to be named `System_gettext` (following the pattern of other `System` functions in the runtime). Verify the actual exported symbol name in the `omcruntime` library.

3. **Return type handling**: `System_gettext` returns `*const c_char`. If null, the original message ID is returned as-is (standard gettext behavior for missing translations).

4. **Locale initialization**: The MetaModelica code uses `System.gettextInit` to set the locale before translation. This Rust module does not wrap `gettextInit` - callers must ensure the locale is initialized before calling `gettext`.

## Potential Issues

1. **FFI signature mismatch**: The `System_gettext` C signature `(threadData, msgid) -> result` is assumed. The actual C runtime may use a different calling convention or signature. Check the `System.h` or `omcruntime` headers for the real signature.

2. **Thread safety**: The `OpenModelica_threadData()` function is called without any synchronization. In a multi-threaded environment, ensure the thread data pointer is valid on the calling thread.

3. **No `gettextInit` wrapper**: The `gettextInit` function from `System` (which initializes the locale) is not exposed in this module. Add a wrapper if locale initialization is needed.

4. **Empty string handling**: If `System_gettext` returns an empty string (valid translation), it will be returned as-is. This matches the MetaModelica behavior.

5. **No pluralization support**: The original Gettext package could potentially be extended with `ngettext` (plural forms). This translation only covers singular `gettext`.
