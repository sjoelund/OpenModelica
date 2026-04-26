# Assumptions and Notes for src/fmi.rs (Util/FMI.mo)

## Translation Summary

This file translates `Util/FMI.mo` which contains FMI (Functional Mock-up Interface) specific types and utility functions for parsing and validating FMI information.

## Types Translated

- **Info** - FMI information record (uniontype with `INFO` variant)
- **TypeDefinitions** - Type definitions (uniontype with `ENUMERATIONTYPE` variant)
- **EnumerationItem** - Enumeration item (struct)
- **ExperimentAnnotation** - Experiment annotation (uniontype with `EXPERIMENTANNOTATION` variant)
- **ModelVariables** - Model variables (uniontype with 5 variants: REALVARIABLE, INTEGERVARIABLE, BOOLEANVARIABLE, STRINGVARIABLE, ENUMERATIONVARIABLE)
- **FmiImport** - FMI import configuration (uniontype with `FMIIMPORT` variant)

## Functions Translated

| MetaModelica | Rust |
|---|---|
| `getFMIModelIdentifier` | `get_fmi_model_identifier` |
| `getFMIType` | `get_fmi_type` |
| `getFMIVersion` | `get_fmi_version` |
| `checkFMIVersion` | `check_fmi_version` |
| `isFMIVersion10` | `is_fmi_version_10` |
| `isFMIVersion20` | `is_fmi_version_20` |
| `getFMIVersionString` | `get_fmi_version_string` |
| `checkFMIType` | `check_fmi_type` |
| `canExportFMU` | `can_export_fmu` |
| `isFMIMEType` | `is_fmi_mime_type` |
| `isFMICSType` | `is_fmi_cs_type` |
| `getEnumerationTypeFromTypes` | `get_enumeration_type_from_types` |
| `filterModelVariables` | `filter_model_variables` |
| `filterModelVariable` | `filter_model_variable` |

## Assumptions

1. **Flags module integration**: `getFMIVersionString` in MetaModelica reads `Flags.getConfigString(Flags.FMI_VERSION)`. The Rust implementation currently returns `"2.0"` as a hardcoded default since the full Flags infrastructure requires runtime initialization not yet wired up. The real value comes from compiler configuration.

2. **List module integration**: `filterModelVariables` in MetaModelica uses `List.filter2OnTrue`. The Rust implementation uses standard `Iterator::filter` which provides the same semantics.

3. **stringEqual**: In MetaModelica, `stringEqual` is a simple string equality check. The Rust implementation uses `==` on `&str`.

4. **Option type**: `Option<Integer>` maps directly to Rust's `Option<i32>`.

5. **List types**: `list<Integer>` maps to `Vec<i32>` and `list<ModelVariables>` maps to `Vec<ModelVariables>`.

6. **1-based vs 0-based indexing**: MetaModelica uses 1-based indexing for some operations. The translated code follows Rust conventions (0-based) where applicable, since no explicit 1-based indexing is used in this module's functions.

7. **Guard conditions**: The `getEnumerationTypeFromTypes` function in MetaModelica uses recursive calls with guard conditions. This was converted to an iterative search to avoid potential stack overflow on very large type definition lists.

## Known Issues

1. **`get_fmi_version_string` returns default**: Since the Flags module requires runtime initialization, this function returns `"2.0"` as a hardcoded default rather than reading from the actual compiler configuration.

2. **No FMI type equality**: The current types implement `PartialEq` and `Clone` which is sufficient for most uses. If more sophisticated type comparison is needed in the future, additional trait implementations may be required.

3. **No Default implementations**: The types do not implement `Default` since most fields are semantically required. Adding defaults would require choosing sensible sentinel values.
