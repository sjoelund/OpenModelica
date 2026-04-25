# Assumptions and Caveats for cevalscriptomsimulator.rs

## Source
`Script/CevalScriptOMSimulator.mo` - A package providing a unified function dispatch interface for OpenModelica Simulator operations.

## Translation Approach
The `ceval` function is a large `matchcontinue` that dispatches based on function name and argument patterns. Translated into a Rust `match` on `function_name.as_str()` with argument count validation and type checking. Each case calls the corresponding wrapper function from `om_simoulator` (which wraps `omsimulatorext`).

## Key Assumptions

1. **Value type mapping**: `Values.Value` variants map directly to `crate::values::Value` enum variants (e.g., `Values.INTEGER(x)` -> `Value::INTEGER { integer: x }`, `Values.STRING(s)` -> `Value::STRING { string: s }`).

2. **ENUM_LITERAL index adjustment**: The original MO code subtracts 1 from ENUM_LITERAL index values for several functions (e.g., `causality-1`, `type_-1`, `solver-1`). This is because MetaModelica uses 1-based indexing while the underlying C API uses 0-based indexing.

3. **Tuple return values**: Multi-return functions (e.g., `oms_getInteger` returning `(value, status)`) are wrapped in `Value::TUPLE` with the values in a `List<Value>`. The order matches the MO source: data value first, status code last.

4. **Argument validation**: Each case validates the argument count and types at runtime. If the wrong number or type of arguments is provided, an error is returned via `anyhow::Result`.

5. **Function name matching**: The function name matching is case-sensitive and exact. Unknown function names produce a descriptive error message.

6. **`loadOMSimulator` / `unloadOMSimulator`**: These use `om_simoulator::load_om_simulator()` and `om_simoulator::unload_om_simulator()` which wrap the C functions `OMSimulator_loadDLL` and `OMSimulator_unloadDLL`.

## Things That May Not Work as Expected

- **String safety**: The C functions expect null-terminated C strings. The `CString::new()` calls in `omsimulatorext` will panic if any string argument contains a null byte. This is the same behavior as the original MO code (which would fail at the C level).

- **Memory management for returned strings**: Functions returning strings (e.g., `oms_getSubModelPath`, `oms_list`) rely on the C API providing valid memory. The Rust wrapper uses `CStr::from_ptr` to convert to `String`, which assumes the string remains valid for the lifetime of the call.

- **Boolean conversion**: `oms_getBoolean` returns an `i32` from the C API. It is converted to a Rust `bool` with `value != 0`. This is consistent with the MO code which expects a boolean result.

- **Tuple ordering in multi-return values**: The tuple ordering follows the MO source exactly. For example, `oms_exportSnapshot` returns `(contents, status)` as a tuple. Any code depending on this function must respect this ordering.

- **Error propagation**: The original MO `matchcontinue` implicitly fails if no case matches. The Rust version explicitly returns an error via `bail!()` for unknown function names and type mismatches.

## Mapping Summary

| MetaModelica Type | Rust Type |
|---|---|
| `String` | `String` |
| `Integer` | `i32` |
| `Real` | `f64` |
| `Boolean` | `bool` |
| `list<Value>` | `im::Vector<Value>` |
| `Values.Value` | `Value` enum |
| `matchcontinue` | `match` + `if let Ok(...)` pattern |
| `Values.TUPLE({...})` | `Value::TUPLE { value_lst: ... }` |
| `Values.ENUM_LITERAL(index=i)` | `Value::ENUM_LITERAL { index: i, .. }` |
