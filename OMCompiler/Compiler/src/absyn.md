# absyn.rs - Translation Assumptions and Notes

## Source
FrontEnd/Absyn.mo from OpenModelica

## Type Mappings
- `Integer` → `i32`
- `Real` → `String` (stored as string representation, not f64, to preserve user's display format)
- `Boolean` → `bool`
- `String` → `String`
- `Path` → `Path` enum (with `Box` indirection for recursion)
- `list<T>` → `im::Vector<T>` (type alias `List<T>`)
- `Option<T>` → `Option<T>`
- `tuple<A, B>` → `(A, B)` Rust tuple
- `tuple<A, B, C>` → `(A, B, C)` Rust tuple

## Box Indirection
Several types have mutual recursion requiring `Box` indirection for Rust's size requirements:

- `Exp` ↔ `FunctionArgs`: `CALL.function_args` is `Box<FunctionArgs>`, `FOR_ITER_FARG.exp` is `Box<Exp>`
- `Exp` ↔ `CodeNode`: `CODE.code` is `Box<CodeNode>`, `C_EXPRESSION.exp` is `Box<Exp>`
- `Equation` ↔ `EquationItem`: `EQUATIONITEM.equation_` is `Box<Equation>`, `EQ_FAILURE.equ` is `Box<EquationItem>`
- `ComponentRef`: self-recursive via `CREF_FULLYQUALIFIED` and `CREF_QUAL`
- `Path`: self-recursive via `QUALIFIED` and `FULLYQUALIFIED`

## SourceInfo
`SourceInfo` is a built-in compiler type, not defined in Absyn.mo. The Rust struct is inferred from C runtime code and has the following fields:
- `file_name: String` - source file path
- `is_read_only: bool`
- `start_line: i32`, `start_column: i32` - start position
- `end_line: i32`, `end_column: i32` - end position

The exact field names and types may differ in the actual implementation.

## im::Vector vs im::List
The CLAUDE.md instructions say to use `im::List`, but `im` crate v15.1.0 does not export `List`. The available persistent collection types are `Vector`, `HashMap`, `HashSet`, `OrdMap`, `OrdSet`. Used `im::Vector<T>` as the list type. Note that `Vector` is a random-access persistent vector (O(log N) access), while `List` would be a cons-list (O(1) cons at front). This may have performance implications for code that relies on O(1) prepending.

## Variant Naming Convention
MO uniontype constructors use ALL_CAPS (e.g., `CLASS`, `WITHIN`, `R_CLASS`). Rust enum variants typically use PascalCase. Per the translation guidelines, ALL_CAPS names are preserved for traceability back to the original MO code. The `#[allow(non_camel_case_types)]` attribute suppresses the corresponding warning.

## Reserved Words
- `Case::ELSE_` uses trailing underscore since `else` is a Rust keyword. In Rust, `ELSE` as an enum variant name would actually work (enum variants are in a different namespace), but `ELSE_` is used for safety/clarity.

## No Functions
Absyn.mo contains only type definitions and two constants (`emptyMod` and `emptyFunctionArgs`). No functions were translated.

## Public Visibility
The MO file marks many types as `public` and others as private. In the Rust translation, all types are `pub` for simplicity. This is acceptable since the module is a private internal module.

## Operator Debug Implementation
`Operator` has a custom `Debug` implementation because it derives `Copy` (which is incompatible with `derive(Debug)` in some edge cases). The custom implementation prints operator names as uppercase strings (e.g., "ADD", "SUB", "LESS").

## Unchanged Types
The following types were translated directly with no special handling needed:
- `ForIterator`, `Program`, `Within`
- `Class`, `ClassDef`, `TypeSpec`, `EnumDef`, `EnumLiteral`
- `ClassPart`, `ElementItem`, `Element`, `ConstrainClass`, `ElementSpec`
- `InnerOuter`, `Import`, `GroupImport`
- `ComponentItem`, `Component`, `EquationItem`, `AlgorithmItem`
- `Algorithm`, `Modification`, `EqMod`, `ElementArg`
- `RedeclareKeywords`, `Each`, `ElementAttributes`, `IsField`
- `Parallelism`, `FlowStream`, `Variability`, `Direction`
- `Case`, `MatchType`, `ReductionIterType`, `NamedArg`
- `Subscript`, `Restriction`, `FunctionPurity`, `FunctionRestriction`
- `Annotation`, `Comment`, `ExternalDecl`, `Ref`, `Msg`
