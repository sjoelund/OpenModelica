# Assumptions for values.rs (translation of FrontEnd/Values.mo)

## Type mappings
- `Integer` -> `i32`
- `Real` -> `f64`
- `String` -> `String`
- `Boolean` -> `bool`
- `list<T>` -> `im::Vector<T>` (aliased as `List<T>`)
- `Option<T>` -> `std::option::Option<T>`
- `Absyn.Path` -> `crate::absyn::Path`
- `Absyn.CodeNode` -> `crate::absyn::CodeNode`

## Notes
- The `Value` enum contains all the variant names in the original CamelCase (e.g., `INTEGER`, `REAL`), matching the original MetaModelica record names.
- The `RECORD` variant uses `record_` as the field name to avoid a Rust keyword conflict.
- The `CODE` variant uses field name `a` (matching the original `A` field in MetaModelica, lowercased to `a` per naming convention).
- `IntRealOp` variants are all unit structs with the original names preserved (e.g., `MULOP`, `DIVOP`).
- Multidimensional arrays are represented as arrays of arrays in MetaModelica; the Rust representation uses `List<Value>` for `value_lst` which captures this nesting.
- `META_FAIL` and `NORETCALL` are unit variants with no fields.
- The `EMPTY` variant has four fields: `scope`, `name`, `ty`, and `ty_str` (lowercased from `tyStr`).

## Boxing for recursive types
MetaModelica handles recursive types natively, but Rust requires explicit indirection.
The following fields are boxed with `Box<Value>` to break infinite recursion:
- `OPTION.some`: `Option<Box<Value>>` (original: `Option<Value>`)
- `META_BOX.value`: `Box<Value>` (original: `Value`)
- `EMPTY.ty`: `Box<Value>` (original: `Value`)

These boxing decisions ensure the type is finite-sized in Rust while preserving the same semantic meaning as the original MetaModelica code.
