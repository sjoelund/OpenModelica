# BaseModelica.rs - Assumptions

## Overview
Translation of `NFFrontEnd/BaseModelica.mo` into Rust.

## Assumptions & Potential Issues

### 1. Flags Module Not Translated
The original MetaModelica code depends on the `Flags` package (`Flags.isSet`, `Flags.isConfigFlagSet`, `Flags.getConfigStringList`). These are not yet translated to Rust, so stub functions are provided that return default values. The `format_from_flags()` function currently returns a format with `NOT_SCALARIZED` mode (because `NF_SCALARIZE` flag check defaults to false via the stub).

**Impact**: When the Flags module is translated, `format_from_flags()` should be re-implemented to call the real Flag functions instead of the stubs.

### 2. Flag Constant Names
The constants `NF_SCALARIZE`, `BASE_MODELICA_OPTIONS`, and `BASE_MODELICA_FORMAT` are hardcoded as string literals. These should match the actual flag names used by the Flags module. If the flag names differ, the stub behavior will be incorrect.

### 3. Uniontype Representation
`OutputFormat` is a uniontype containing a single record variant (`OUTPUT_FORMAT`). In Rust, this is represented as an enum with one variant. If additional uniontype variants are added to the MetaModelica source, the enum must be extended and the Display implementation updated.

### 4. `for` Loop with `match` Pattern
The original `for` loop uses a `match` statement that has `algorithm` expressions within each case (MetaModelica's way of executing side effects). The translation uses a `match` on `option.as_str()` with boolean return values, discarding the result with `let _ = matched;`. This correctly captures the semantics: each case modifies mutable variables in scope.

### 5. `inlineFunctions` Function
This function simply returns the result of a Flags check. Currently it returns `false` due to the stub. No algorithm body to translate beyond the initialization from `Flags.isConfigFlagSet`.

### 6. Default Format Constant
`defaultFormat` is translated to a `const` with the same values: `PARTIALLY_SCALARIZED`, `WITH_RECORDS`, `false`. Since all fields are `Copy`, this is a zero-cost constant.

### 7. Protected Import of Flags
The `import Flags` is marked `protected` in MetaModelica, meaning it's only accessible within the package. The stub functions `flags_is_*` and related constants are private (no `pub`), which correctly reflects this protection.

### 8. Type Mappings
- `Boolean` → `bool`
- `enumeration` → `enum` with `#[derive(Copy, Clone, PartialEq, Eq, Hash)]`
- `record` fields → struct fields within enum variant
- `constant` → `const`
