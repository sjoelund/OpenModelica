# SCodeSimplify Translation Assumptions

## Files Translated
- `FrontEnd/SCodeSimplify.mo` -> `src/scodesimplify.rs`
- `FrontEnd/SCode.mo` types -> `src/scode.rs` (supporting type definitions)

## Assumptions and Notes

### Matchcontinue Semantics
The MetaModelica `matchcontinue` construct is simulated using iterative control flow with early returns. The `simplify_elements` function uses a `while let` loop with `continue` to simulate the MetaModelica `matchcontinue` behavior where non-matching cases fall through to the next case.

### Box for Recursive Types
Mutually recursive types (`Mod` <-> `Element`) and self-recursive types (`ClassDef::CLASS_EXTENDS.composition`) are wrapped in `Box<>` to prevent infinite size errors. This matches the pattern used in `absyn.rs`.

### Import Paths
Since these modules are part of the same crate (declared in `main.rs`), imports use `crate::absyn` and `crate::scode` rather than bare module names.

### Constants vs Functions
MetaModelica `constant` declarations that require non-const initialization (e.g., `Vector::new()`) cannot be translated to Rust `const` items. Instead, they are provided as `pub fn` functions (`default_var_attr`, `default_param_attr`, etc.).

### Unused Type Exports
The `scode.rs` module exports all types defined in the MetaModelica SCode package, even though only a subset are used by the simplification logic. This ensures the module is a complete translation and can serve as a reference for future work.

### pathContains
The `pathContains` helper function is implemented inline in `scodesimplify.rs`. It recursively traverses the `Path` enum to check if any component matches the given identifier string. This corresponds to `AbsynUtil.pathContains` from the MetaModelica interface.

### No Runtime Testing
The generated code compiles but has no associated tests. The simplification logic should be tested with actual SCode data to verify correct behavior.

### Potential Issues
1. **EXTENDS with "Icons" path**: The current implementation checks if the base class path contains "Icons" as a substring in any path component. This should match the MetaModelica behavior of `AbsynUtil.pathContains(bcp, "Icons")`.
2. **Element modification in place**: The `Element::EXTENDS.modifications` and `Element::COMPONENT.modifications` fields are `Box<Mod>` rather than `Mod`, which adds a small heap allocation overhead compared to the original MetaModelica.
3. **Vector operations**: The `im::Vector` type's `remove(0)` and `push_back` are used for list cons/decons operations. These are O(n) operations rather than O(1), which could impact performance on large element lists.
