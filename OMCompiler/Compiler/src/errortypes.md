# ErrorTypes Translation Assumptions and Notes

## Assumptions

1. **SourceInfo definition** - The `SourceInfo` type is used in `TotalMessage` but is not defined in `ErrorTypes.mo`. It is defined as a built-in compiler type. I've defined it locally in `errortypes.rs` with the same fields observed from the `absyn.rs` file: `file_name`, `is_read_only`, `start_line`, `start_column`, `end_line`, `end_column`. This definition should be consistent with the one in `absyn.rs` if that module is loaded.

2. **Gettext.TranslatableContent** - The `TranslatableContent` type is imported from the `Gettext` package. Since there's no separate `gettext.rs` Rust module, I've defined `TranslatableContent` locally with two variants: `GETTEXT { msgid: String }` and `NOTRANS { str: String }`. The variant name `NOTRANS` uses a Rust keyword (`str` as a field name is valid, but the variant name `NOTRANS` avoids the `trans` keyword issue).

3. **list<String> mapping** - The `MessageTokens` type is `list<String>` in MetaModelica. The `im::List` type from the `im` crate has no `List` in version 15.x, so I used `im::Vector` as the underlying type (consistent with `absyn.rs`).

4. **Integer mapping** - `ErrorID` maps from MetaModelica `Integer` to Rust `i32` as per the CLAUDE.md conventions.

5. **uniontype mapping** - All MetaModelica `uniontype`s are translated to Rust `enum`s with struct variants (one variant per record). This is consistent with the `absyn.rs` translation.

6. **Display trait** - I added `Display` implementations for `Severity`, `MessageType`, and `TranslatableContent` to provide human-readable string representations. These are not in the original MetaModelica but are useful for debugging.

## Potential Issues

1. **SourceInfo consistency** - If `absyn.rs` is compiled separately and defines `SourceInfo` with different field names or types, there could be inconsistencies. Both definitions should match the C runtime's actual `SourceInfo` layout.

2. **TranslatableContent usage** - Any code that consumes `TranslatableContent` will need to handle the two variants. The `translateContent` function from the `Gettext` package is not translated here - if translation is needed at runtime, a separate `gettext.rs` module should be created.

3. **im::Vector vs im::List** - Using `im::Vector` instead of `im::List` means the memory layout and performance characteristics may differ from the original MetaModelica linked list. This should be fine for error handling where the number of tokens is small.

4. **Copy/Copy on enums** - `Severity` and `MessageType` implement `Copy`, which means they can be passed by value without allocation. This matches the semantics of the original union types which are essentially tagged integers.
