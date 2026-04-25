# Assumptions for avltreestring.rs

## Translation of AvlTreeString.mo

This module translates `Util/AvlTreeString.mo`, which is a specialization of
`BaseAvlTree` with `Key = String` and `Value = Integer`.

### Key Design Decisions

1. **No separate Tree type** - Instead of creating a new Tree type, this module
   uses type aliases pointing to the generic `baseavltree::Tree<String, i32>`.
   This avoids code duplication since `baseavltree.rs` already provides the
   full generic implementation.

2. **Re-exports from baseavltree** - All tree operations (add, get, to_list,
   etc.) are re-exported from `baseavltree` module so consumers can use them
   with `avltreestring::add(tree, key, value, conflict_fn)` syntax.

3. **`key_str_fn` takes `&str`** - The MetaModelica function takes a String key
   and returns it as a string. In Rust, accepting `&str` is more flexible.

4. **`key_compare_fn` returns -1/0/1** - Matches MetaModelica convention where
   `stringCompare` returns -1, 0, or 1 for less-than, equal, greater-than.

5. **`baseavlset` module does not exist** - The files `avlsetint.rs` and
   `avlsetstring.rs` reference `crate::baseavlset` which doesn't exist as a
   file. This module only depends on `baseavltree`.

### Things That Might Not Work As Expected

- **Performance**: The generic `baseavltree::Tree<String, i32>` clones strings
  on every operation. If performance is critical, a specialized implementation
  could avoid unnecessary clones.

- **Memory**: `String` keys allocate on the heap. For small keys, a `Box<str>`
  or `Rc<str>` could reduce allocations if keys are shared.

- **Conflict functions**: The re-exported conflict functions (add_conflict_fail,
  etc.) expect references to String/i32. Callers must provide the correct type.

- **`new()` function**: The BaseAvlSet package defines a `new` function that
  returns an empty tree. This is available via `Tree::new()` but not re-exported
  as a standalone function to avoid naming conflicts.
