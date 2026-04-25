# Assumptions for HpcOmSchedulerExt.rs

## Translated Functions

| MetaModelica | Rust |
|---|---|
| `readScheduleFromGraphMl(String) -> list<Integer>` | `read_schedule_from_graph_ml(&str) -> *mut c_void` |
| `scheduleMetis(array<Integer>, array<Integer>, array<Integer>, array<Integer>, Integer) -> list<Integer>` | `schedule_metis(*mut c_void, *mut c_void, *mut c_void, *mut c_void, i32) -> *mut c_void` |
| `schedulehMetis(array<Integer>, array<Integer>, array<Integer>, array<Integer>, Integer) -> list<Integer>` | `schedule_h_metis(*mut c_void, *mut c_void, *mut c_void, *mut c_void, i32) -> *mut c_void` |

## Assumptions & Potential Issues

1. **List return type**: The `list<Integer>` return type is mapped to `*mut c_void` because the C API from `omcruntime` returns a pointer to a C linked list (MMC runtime structure). Converting to a Rust `im::List<i32>` would require additional MMC runtime bindings.

2. **Array parameters**: The `array<Integer>` input parameters are mapped to `*mut c_void` since the exact C signature is not known. If the C API accepts raw `int*` pointers, these may need to change to `*mut i32`.

3. **External C linkage**: All functions depend on the `omcruntime` shared library. The program will fail to link or run if the library is not available at runtime.

4. **String handling**: The `fileName` parameter is converted to `CString` for the FFI call, which panics on NUL bytes. This is safe for typical file paths.

5. **Safety**: All functions use `unsafe` blocks around FFI calls. The safety is delegated to the `omcruntime` C implementation.
