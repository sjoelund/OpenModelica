# Settings Module - Assumptions and Notes

## Overview

Translation of `Util/Settings.mo` into `src/settings.rs`.

## Assumptions

1. **C function naming convention**: The MetaModelica `external "C"` annotations reference functions like `Settings_getVersionNr()`, `SettingsImpl__setTempDirectoryPath()`, etc. These are assumed to be available in the `omcruntime` C library. The Rust FFI declarations mirror these exact names.

2. **String allocation ownership**: For getter functions that return strings (`get_version_nr`, `get_temp_directory_path`, etc.), the returned `*const c_char` is assumed to be allocated by the C runtime and should be treated as immutable. The caller copies the data into a Rust `String` via `to_string_lossy()`.

3. **Boolean to c_int mapping**: MetaModelica `Boolean` inputs map to `i32`/`c_int` in C (0=false, 1=true). The `runningTestsuite` parameter is mapped accordingly in `get_modelica_path` and `get_home_dir`.

4. **Integer type mapping**: MetaModelica `Integer` maps to `i32` in Rust, consistent with the project conventions.

5. **dumpSettings is commented out**: The `dumpSettings` function is commented out in the source MO file (marked as TODO). It is not translated.

6. **Library linkage**: All functions are declared as `extern "C"` but no actual linking to `omcruntime` is performed in this crate. The actual linking happens at the application level when the `omcruntime` shared library is available.

## Things that might not work as expected

1. **Missing C symbols**: If the `omcruntime` library is not linked at runtime, all FFI calls will result in link-time or runtime errors. This crate only provides the declarations and safe wrappers.

2. **String lifetime**: The getters return owned `String` values by copying from `*const c_char`. If the C runtime's returned pointers are invalid or become dangling, this will read from freed memory.

3. **No error handling**: The external C functions are assumed to always succeed. There is no `Result<T>` return type wrapping, since the original MO functions do not indicate failure modes.

4. **setTempDirectoryPath/setInstallationDirectoryPath/setModelicaPath**: The setter functions take a `*const c_char`. If the C side expects a mutable pointer (`*mut c_char`), this could cause issues.
