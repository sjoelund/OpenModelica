# AbsynToRust - Assumptions and Known Issues

## Overview

This template translates MetaModelica Absyn (abstract syntax) to Rust code. It is modeled after `AbsynToJulia.tpl` but adapted for Rust syntax and idioms.

## Assumptions

### Type Mappings

| MetaModelica | Rust |
|---|---|
| `Integer` | `i32` |
| `Real` | `f64` |
| `Boolean` | `bool` |
| `String` | `String` |
| `list<T>` | `im::Vector<T>` (aliased as `List<T>`) |
| `tuple<A, B, ...>` | `(A, B, ...)` (Rust native tuples) |
| `uniontype` | `enum` with `#[derive(Debug, Clone, PartialEq)]` |
| `record` | `struct` with `#[derive(Debug, Clone, PartialEq)]` |

### Pattern Matching

- MetaModelica `match` expressions map to Rust `match` expressions
- `matchcontinue` is NOT directly supported in Rust; the template generates regular `match` blocks
- Pattern guards use `if` syntax which matches Rust's `pattern if guard` syntax
- The `@` operator for as-patterns maps to Rust's `@` operator (e.g., `x @ Pattern`)

### Function Conventions

- All function names should be converted to `snake_case` in Rust (the template includes a placeholder for this via `System.stringReplace`)
- Functions return values directly (no explicit `return` needed in most cases)
- Functions that can fail should return `Result<T>` and use `?` for propagation

### String Handling

- String literals use `.to_string()` suffix
- The `escapeModelicaStringToRustString` function handles string escaping (must be implemented)
- String concatenation uses `+` operator (same as MetaModelica, unlike Julia which uses `*`)

### Import/Module System

- `import` maps to `use` in Rust
- `using` (unqualified import) maps to `use path::*;`
- Named imports map to `use Path as Name;`
- Group imports map to `use prefix::{group1, group2};`
- Qualified paths use `::` separator instead of `.`

### Error Handling

- `ALG_FAILURE` maps to `panic!()` -- this may need refinement for production code
- `ALG_TRY` maps to `std::panic::catch_unwind` -- this is a rough approximation
- Consider using `anyhow::Result` and `anyhow::bail!` for matchcontinue patterns

### Control Flow

- `if/elseif/else` expressions use Rust's `if/else if/else` syntax
- `for` loops use Rust's `for x in range` syntax
- `while` loops use Rust's `while condition` syntax
- `break` and `continue` map directly

### Lists and Collections

- `list(...)` constructs map to `vec![...]`
- `CONS(head, tail)` maps to `List::cons(head, tail)`
- Empty list maps to `vec![]`
- Array constructs also use `vec![...]`

### Matchcontinue Support

Per CLAUDE.md guidance, matchcontinue requires special handling:

```rust
fn matchcontinue(x: i32) -> Result<i32> {
    if x == 1 { return Ok(value1); }
    if let Ok(v) = match x { 2 => check(2), _ => bail!("") } { return Ok(v); }
    // ...
    bail!("no match");
}
```

This pattern requires `use anyhow::{Result, bail};`.

## Things That May Not Work

1. **Threaded iteration** (`FOR_ITER_FARG` with `THREAD`) -- marked with comment, no direct Rust equivalent
2. **Partial functions** -- forward-declared only, not fully implemented
3. **Equation sections** -- not supported, must be converted to algorithm sections
4. **When clauses** -- explicitly unsupported in the template
5. **`extends` keyword** -- not natively supported in Rust; generates a comment placeholder
6. **`constrainedby`** -- generates placeholder text, no Rust equivalent
7. **`redeclare`/`replaceable`** -- generates text but has no semantic meaning in Rust
8. **`final` prefix** -- generates text but Rust uses different const/immutable semantics
9. **`each` prefix** -- generates text but has no semantic meaning in Rust
10. **`inner`/`outer`** -- not handled in Rust output
11. **Function redefinitions** (Modelica-style) -- generates `pub type` alias, may not match semantics
12. **CamelCase to snake_case conversion** -- uses placeholder `System.stringReplace`, needs proper implementation
13. **1-based vs 0-based indexing** -- MetaModelica is 1-based, Rust is 0-based; off-by-one errors possible
14. **`$array` special function** -- mapped to `vec![...]` which may not match all semantics
15. **Code nodes** (`Code` expressions) -- rough mapping, may need refinement

## Dependencies

The generated Rust code requires:

- `im` crate (for persistent data structures via `im::Vector`)
- `anyhow` crate (for `Result` and `bail!` in matchcontinue patterns)
- Standard library features (`std::any::Any` for polymorphic types)

## Files

- `AbsynToRust.tpl` -- The template file containing all translation rules
- `AbsynToRustTV.mo` -- The type view/interface file declaring all types used by the template
- `AbsynToRust.md` -- This assumptions document
