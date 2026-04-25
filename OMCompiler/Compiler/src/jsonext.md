# jsonext.rs — Assumptions and Notes

## Overview

Translation of `Util/JSONExt.mo` into Rust as `src/jsonext.rs`.

## Assumptions

### External C dependencies
All type-checking and accessor functions (`omc_is_integer`, `omc_is_real`, `omc_get_record_names`, etc.) call into the OpenModelica runtime via FFI. These functions **will not link** unless the OpenModelica runtime library is provided at link time. Compiling this crate in isolation will produce undefined-symbol errors.

### Opaque metatype handle
All values are represented as `Metatype` (`*mut c_void`), matching OpenModelica's `modelica_metatype`. This is an opaque pointer to the OMC code generator's internal value representation. The caller must ensure the pointer is valid and points to a properly constructed OMC value.

### List iteration
The `ListIter` iterator walks IM cons-cell lists by repeatedly calling `omc_get_list_element` at offsets 1 (head/element) and 2 (tail/next list). This matches the OpenModelica runtime's linked-list layout. Lists must be proper cons-cell lists terminated by `NIL`.

### Record field access
`get_record_component` uses 1-based offsets into the record's slots. Slot 1 is the record name (a string), and slots 2+ are the field values. This is reflected in the `serialize` function's record handling.

### String extraction
Strings are extracted via `omc_cast_string` which returns a `*const c_char` pointer to the internal string data. The string must be null-terminated for `CStr::from_ptr` to work correctly.

### Tuple size
`omc_get_tuple_size` returns the number of slots in a tuple's header. This is used to iterate over tuple elements in the `serialize` function.

## Things that may not work as expected

1. **Linking**: This module requires the OpenModelica runtime. It cannot be compiled or linked standalone.
2. **Record names**: The `get_record_names` function relies on OpenModelica's internal `record_description` C struct. If OMC changes its internal representation, this will break.
3. **Nil terminator**: The `ListIter` checks `is_nil` to terminate. If a list is malformed (not properly terminated), it will iterate past the intended end.
4. **Unknown types**: If a value doesn't match any of the recognized types (integer, real, string, record, nil, cons, none, some, tuple), `serialize` returns `"UNKNOWN(??)"`. The original code used `anyString(any)` for this case, which is not available in Rust.

## API summary

| MetaModelica | Rust | Notes |
|---|---|---|
| `isInteger<T>` | `is_integer(Metatype) -> bool` | FFI wrapper |
| `isReal<T>` | `is_real(Metatype) -> bool` | FFI wrapper |
| `isString<T>` | `is_string(Metatype) -> bool` | FFI wrapper |
| `isArray<T>` | `is_array(Metatype) -> bool` | FFI wrapper |
| `isRecord<T>` | `is_record(Metatype) -> bool` | FFI wrapper |
| `isTuple<T>` | `is_tuple(Metatype) -> bool` | FFI wrapper |
| `isNONE<T>` | `is_none(Metatype) -> bool` | FFI wrapper |
| `isSOME<T>` | `is_some(Metatype) -> bool` | FFI wrapper |
| `isNil<T>` | `is_nil(Metatype) -> bool` | FFI wrapper |
| `isCons<T>` | `is_cons(Metatype) -> bool` | FFI wrapper |
| `getRecordNames<T>` | `get_record_names(Metatype) -> Metatype` | Returns IM list |
| `getRecordComponent<TIN,TOUT>` | `get_record_component(Metatype, i32) -> Metatype` | 1-based offset |
| `getInteger<T>` | `get_integer(Metatype) -> i64` | |
| `getReal<T>` | `get_real(Metatype) -> f64` | |
| `getString<T>` | `get_string(Metatype) -> Option<String>` | Returns None if null |
| `getSome<TIN,TOUT>` | `get_some(Metatype) -> Metatype` | |
| `getTupleSize<T>` | `get_tuple_size(Metatype) -> i32` | |
| `getList<TIN,TOUT>` | `get_list(Metatype) -> Metatype` | Returns input |
| `getListElement<TIN,TOUT>` | `get_list_element(Metatype, i32) -> Metatype` | 1-based offset |
| `serialize<T>` | `serialize(Metatype, &[String]) -> String` | JSON output |
| `listMember` (used internally) | `is_list_member(&[String], &str) -> bool` | Helper |
