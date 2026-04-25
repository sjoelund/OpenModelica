# Assumptions for Autoconf.mo -> autoconf.rs Translation

## Overview

The `Autoconf` package is an encapsulated package containing only constants and no functions. It represents build-time configuration values that were determined during the autoconf configuration step of the OpenModelica build process.

## Assumptions and Notes

### 1. String comparison in constants

**MetaModelica:** `constant Boolean isWindows = os == "Windows_NT";`

**Rust:** `pub const IS_WINDOWS: bool = false;`

Rust stable does not yet support `const` string comparison (issue #143874). Since `OS` is `"linux"`, `IS_WINDOWS` is hardcoded to `false`. If the build target changes to Windows, these constants would need to be re-evaluated.

### 2. `list<String>` mapping to `im::Vector`

**MetaModelica:** `constant list<String> systemLibs = {"-lomcruntime", ...};`

**Rust:** `pub fn system_libs() -> List<&'static str>`

The `im::List` type alias maps to `im::Vector` (the `im` crate v15.x renamed `List` to `Vector`). Since `im::Vector` cannot be constructed in a `const` context, it is provided as a function. The original MetaModelica code has `systemLibs` as a constant, but the equivalent Rust implementation must use a function.

### 3. Dead code warnings

All constants in this module are `pub` and intended for use by other modules. The current build (with only `main.rs`) will produce dead code warnings for all constants. This is expected and will resolve once other modules import from `autoconf`.

### 4. Compile-time evaluation of conditionals

All conditional expressions in the original `.mo` file evaluate to compile-time constants in Rust:
- `isWindows` → `false` (on this build)
- `platform` → `"Unix"`
- `exeExt` → `""`
- `bstatic` / `bdynamic` → `"-Wl,-Bstatic"` / `"-Wl,-Bdynamic"`
- `groupDelimiter` → `":"`
- `hwloc` → `""` (condition `0 == 1` is false)

### 5. Empty strings vs empty lists

The `corbaLibs` constant maps to an empty `&str`, and `hwloc` also maps to an empty `&str`. These are included in the `system_libs()` list as empty strings rather than being filtered out, to preserve the original structure.

### 6. `DLL_EXT` naming

The original MetaModelica constant `dllExt` maps to `.so` (shared library extension), not a DLL extension. This follows the naming convention from the original code but may be misleading on non-Windows platforms.
