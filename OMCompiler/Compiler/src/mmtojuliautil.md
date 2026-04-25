# Assumptions and Notes for mmtojuliautil.rs

## Source
`Script/MMToJuliaUtil.mo` - Context types and utility functions for the MetaModelica-to-Julia code generator.

## Assumptions

### 1. Context uniontype mapping
The MetaModelica `uniontype Context` was translated to a Rust enum with struct variants matching each record. The variant names were kept in SCREAMING_SNAKE_CASE to match the original MetaModelica identifiers, with `#[allow(non_camel_case_types)]` suppressing the lint.

### 2. Absyn types
All `Absyn.*` type references (e.g., `Absyn.Exp`, `Absyn.Direction`, `Absyn.ElementSpec`, `Absyn.ClassPart`, `Absyn.ElementItem`, `Absyn.AlgorithmItem`) are imported from `crate::absyn`. This requires `mod absyn;` to be present in `main.rs`.

### 3. directionEqual and getDirection
The original `AbsynUtil.directionEqual` and `AbsynUtil.getDirection` functions were implemented directly in this module since AbsynUtil doesn't have a separate Rust translation. The logic matches the original MetaModelica exactly:
- `directionEqual` matches on pairs of directions (BIDIR, INPUT, OUTPUT, INPUT_OUTPUT).
- `getDirection` destructures an `ElementItem` to find a `Direction` inside the `ElementAttributes`, defaulting to `BIDIR` if not found.

### 4. filterOnDirection signature change
The MetaModelica version takes `list<Absyn.ElementItem>` and outputs a `list<ElementItem>`. The Rust version takes `&[absyn::ElementItem]` (a slice) and returns `Vec<absyn::ElementItem>` (a mutable vector) for idiomatic Rust. The caller can convert between `im::Vector` and `Vec` as needed.

### 5. explicitReturnInClassPart / algorithmItemsContainsReturn
Both functions were consolidated into a single public function (`explicit_return_in_class_part`) and one private helper (`algorithm_items_contains_return`). The original MetaModelica had the helper as a separate top-level function. The private helper was changed to take `&ClassPart` instead of `list<AlgorithmItem>` to avoid exposing an unnecessary abstraction layer.

### 6. mMKeywordToJLKeyword
The original function body is empty (`end mMKeywordToJLKeyword;`). No translation was generated for this function.

### 7. Commented-out functions
The MetaModelica file contains several fully commented-out functions (`getAllPartsExceptRecords`, `getPartsThatAreRecords`, `splitRecordsAndOtherElements`, `restrictionIsRecord`, `refactorNonStandardUniontypes`, etc.). These were not translated.

### 8. Potential issues
- The `filter_on_direction` function returns `Vec` instead of `im::Vector`. If the Julia generator expects persistent lists, an adapter may be needed.
- No error handling (`Result`) was needed since none of the functions in this module can fail.
