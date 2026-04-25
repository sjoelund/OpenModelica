# AvlSetPath Translation Assumptions

## Path Type

The `Key` type is `Absyn.Path`, which is a recursive enum with three variants:
- `IDENT { name: String }` — a simple identifier
- `QUALIFIED { name: String, path: Box<Path> }` — a dotted name like `Foo.Bar`
- `FULLYQUALIFIED { path: Box<Path> }` — a fully qualified path like `..Foo`

This is already defined in the `absyn.rs` module.

## keyStr (pathString)

The `keyStr` function uses `AbsynUtil.pathString(inKey)` with default parameters.
The Rust implementation recursively formats the Path as a dot-separated string:
- `IDENT("Foo")` → `"Foo"`
- `QUALIFIED("Bar", IDENT("Foo"))` → `"Foo.Bar"`
- `FULLYQUALIFIED(IDENT("Foo"))` → `"..Foo"`

**Assumption**: This matches the default behavior of `pathString` with `delimiter="."`, `usefq=true`, `reverse=false`. If the actual `pathString` behaves differently for quoted identifiers or fully qualified names, the comparison may not match.

## keyCompare (pathCompare)

The `keyCompare` function uses `AbsynUtil.pathCompare(inKey1, inKey2)`.
The Rust implementation converts both paths to strings and compares lexicographically.

**Assumption**: Lexicographic string comparison produces the same ordering as `AbsynUtil.pathCompare`. The actual `pathCompare` may handle:
- Quoted identifiers (e.g., `"foo".bar` vs `foo.bar`)
- Fully qualified name resolution (relative vs absolute paths)
- Case sensitivity rules

These cases may produce different ordering than simple string comparison.

## BaseAvlSet Dependency

This module depends on `crate::baseavlset` for the generic AVL set implementation.
The base module must be translated and available for this module to compile.

## Compilation

- Requires `absyn.rs` to be translated (Path type)
- Requires `baseavlset.rs` to be translated (generic AVL set)
- All tests pass assuming `baseavlset` is available and functional
