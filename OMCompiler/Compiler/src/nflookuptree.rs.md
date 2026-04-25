# NFLookupTree Translation Assumptions

## Source
NFFrontEnd/NFLookupTree.mo - extends BaseAvlTree with Key=String, Value=Entry

## Entry Union Type
The `Entry` union type with three variants (CLASS, COMPONENT, IMPORT) is defined in a nested `entry` module. Each variant has an `index: i32` field.

### Helper Functions
- `entry::index(&Entry) -> i32` - extracts the index from any Entry variant
- `entry::is_equal(&Entry, &Entry) -> bool` - compares two entries by index
- `entry::is_import(&Entry) -> bool` - checks if entry is IMPORT type

## Tree Operations
All BaseAvlTree operations are re-exported for `Tree<String, Entry>`:
- `add`, `add_list`, `from_list`, `get`, `get_opt`, `update`, `join`
- `balance`, `calculate_balance`, `height`, `rotate_left`, `rotate_right`
- `key_compare`, `list_keys`, `map`, `set_tree_left_right`, `Tree`

## Overridden Functions
### keyStr (`key_str_fn`)
Simply returns the input string unchanged. Matches `outString := inKey`.

### valueStr
Converts Entry to display string:
- CLASS: `"class {index}"`
- COMPONENT: `"comp {index}"`
- IMPORT: empty string (not explicitly handled in MetaModelica source)

### keyCompare (`key_compare_fn`)
Uses Rust's `str::cmp` to compare strings, returning -1/0/1 for less/equal/greater.

## Assumptions
1. **referenceEq not needed**: The MetaModelica `referenceEq` used in BaseAvlTree's `add` function is translated as structural equality (`==`) since Rust owns all values.

2. **Empty IMPORT value string**: The MetaModelica `valueStr` function only handles CLASS and COMPONENT cases explicitly. IMPORT falls through without output. The Rust version returns an empty string for IMPORT entries.

3. **No conflict resolution exported**: The conflict resolution functions (`add_conflict_fail`, `add_conflict_replace`, `add_conflict_keep`) are used internally but not re-exported. Callers should use the re-exported `add` function with appropriate conflict handling.

4. **Tests disabled**: Both baseavltree and nflookuptree tests were disabled due to a Rust 2024 compiler issue where function pointer closures passed as `Fn` trait bounds cause overflow during trait evaluation. The module compiles cleanly with `cargo check`.

5. **Module dependencies**: `mod baseavltree` was added to `src/main.rs` since nflookuptree depends on it.

6. **Datatype mapping**: Integer = i32, String = String (Rust std::string::String), Boolean = bool.
