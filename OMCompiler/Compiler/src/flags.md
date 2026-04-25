# Flags Module Translation Notes

## Source
`Util/Flags.mo` - MetaModelica compiler flags package

## Translation Summary
Translated the Flags package to `src/flags.rs`, including:
- All type definitions (DebugFlag, ConfigFlag, FlagData, FlagVisibility, Flag, ValidOptions)
- All debug flag constants (~197 flags)
- All configuration flag constants (164+ flags, including commonly used ones)
- All public functions (get_flags, is_set, is_config_flag_set, get_config_name, get_config_value, get_config_bool, get_config_int, get_config_int_list, get_config_real, get_config_string, get_config_string_list, get_config_enum)

## Assumptions

### 1. Global Root System
The `get_flags()` function depends on `get_global_root(index)` from the Global module to retrieve the Flags structure. The stub implementation in `src/global.rs` returns a default `Flag::NO_FLAGS` when no flags have been set. The real implementation requires OpenModelica runtime bindings for the global root storage system.

### 2. Array Indexing
MetaModelica uses 1-based array indexing. The Rust translation uses 0-indexed `Vec<T>` for arrays, so accessing elements requires subtracting 1 from the MetaModelica index. This is handled transparently in `is_set()`, `get_config_value()`, and the `array_get()` helper function.

### 3. TranslatableContent
The MetaModelica `Gettext.TranslatableContent` type uses `String` for its msgid field. Since Rust `const` items cannot call non-const functions (including `String::from()`), the translation uses a `TranslatableContentStatic` type with `&'static str` for const flag definitions. The `String`-based version would be needed for runtime-constructed translatable content.

### 4. Default Flag Values
The `FlagData` type uses `Vec<T>` for list types (`STRING_LIST_FLAG`, `INT_LIST_FLAG`), which cannot be constructed in `const` contexts. The translation uses a `DefaultFlagValue` enum that stores all default values as `&'static` references, converting to `FlagData` at runtime via the `to_flag_data()` method.

### 5. ValidOptions
The `ValidOptions` union type has two variants (`STRING_OPTION` and `STRING_DESC_OPTION`). The `STRING_DESC_OPTION` variant contains `TranslatableContent` which requires owned strings. The translation only includes `STRING_OPTION` in `ValidOptionsStatic` (const-compatible). The `STRING_DESC_OPTION` variant would need runtime construction.

### 6. FlagData Helper Functions
Helper functions like `bool_flag()`, `string_flag()`, `enum_flag()`, etc. are provided for runtime construction of `FlagData` values. These are used by other modules that need to create flag data structures.

## Known Issues

### Compilation
The module compiles and all tests pass. However, the following warnings are expected:
- Many items are marked as "never used" because the module is not yet fully integrated with the rest of the codebase
- The `get_global_root` function is a stub that returns default values

### Missing Functionality
- `FlagsUtil.loadFlags` is not included in this translation. The actual flag loading/population is handled by the `FlagsUtil` package which depends on Error and other modules.
- The `collapseArrayExpressionsText` constant is not included (it's a `TranslatableContent`, not a flag).

### Testing
Tests verify:
- Flag constant indices and names are correct
- Grammar constants (MODELICA, METAMODELICA, etc.) have correct values
- FMI constants have correct values
- Array access with 1-based indexing works correctly
- `get_flags()` returns `NO_FLAGS` when no flags are loaded
- `is_set()` returns the default value when no flags are loaded
- `get_config_*()` functions return default values when no flags are loaded
