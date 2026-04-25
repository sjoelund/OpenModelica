# Unzip Module - Assumptions and Notes

## Assumptions

1. **C Runtime Availability**: This module depends on the `om_unzip` C function from the OpenModelica runtime library. The `omcruntime` library must be linked at build time for this module to be usable.

2. **ZIP Library**: The underlying `om_unzip` function depends on minizip (from the FMIL project). Ensure the runtime is compiled with minizip support.

3. **String Encoding**: The C function expects UTF-8 compatible C-strings. Non-UTF-8 paths may cause issues on some platforms.

4. **Path Handling**: The C function uses forward-slash (`/`) as a directory delimiter. On Windows, backslashes in paths may not work correctly.

5. **No Progress Tracking**: The C function returns only success/failure with no progress information for large archives.

## Things That Might Not Work As Expected

- **Empty ZIP files**: If the ZIP file has zero entries, the function may behave unexpectedly (though it should return success since no errors occur).
- **Very large ZIP files**: The function loads no data into memory beyond a read buffer, so memory usage should be manageable. However, disk space must be sufficient.
- **Nested extraction paths**: The C function creates directories as needed via `SystemImpl__createDirectory`, but if parent directories cannot be created, extraction will silently fail.
