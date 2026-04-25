# Assumptions and Notes for AvlTree.mo -> avltree.rs

## Type System

- **Generic types**: `Key` and `Val` are generic type parameters on `Tree<K, V>`, `Node<K, V>`, and `Item<K, V>`.
  The original MetaModelica uses polymorphic types (`polymorphic<Any>`), which in Rust translates to generics with trait bounds.

- **Callback functions**: The Mo code stores function pointers (`FuncTypeKeyCompare`, `FuncTypeKeyToStr`, etc.) as fields
  in the `Tree` struct. These are translated to Rust trait objects (`Box<dyn KeyCompare<K>>`, etc.) stored on the heap.
  The traits require `Debug` for the string conversion traits to allow debugging output.

- **Option types**: `Option<FuncTypeKeyToStr>` in Mo becomes `Option<Box<dyn KeyToStr<K>>>` in Rust.
  When `NONE()`, the corresponding functionality (key-to-string, val-to-string, update check) is unavailable.

## Node Semantics

- **Node::NO_NODE**: Represents an empty subtree (analogous to `null` in other languages).
- **Node::NODE with Item::NO_ITEM**: A placeholder node with no actual data. This is used for the initial empty tree
  root and in `empty_node_if_no_node()`.
- **Box<Node<K,V>>**: Children are heap-allocated via `Box` to handle the recursive type. This is necessary because
  Rust requires all enum variants and struct fields to have known sizes at compile time.

## matchcontinue Translation

The Mo `matchcontinue` keyword is translated to sequential `if` statements with early `return`. Each case in the
`matchcontinue` becomes a guarded `if` block. If a case matches and executes a `then` branch, it returns immediately.
If it doesn't match, execution continues to the next case.

Example translation:
```rust
// Mo: matchcontinue x { case 1 then A; case 2 then B; case 2 then C; else D; }
// Rust:
if x == 1 { return Ok(A); }
if let Ok(v) = match x { 2 => check_B(), _ => bail!("") } { return Ok(v); }
if let Ok(v) = match x { 2 => check_C(), _ => bail!("") } { return Ok(v); }
return D;
```

## add / addNode / addNodeDispatch

- `add()` destructures the Tree, calls `add_node()` on the root, then reconstructs the Tree.
- `add_node()` handles empty nodes and dispatches to `add_node_dispatch()` based on key comparison.
- `add_node_dispatch()` has the most complex logic with `matchcontinue`:
  - If key equals existing key and no update check function: allow replacement
  - If key equals existing key and update check function says yes: allow replacement
  - If key equals existing key and update check function says no: return unchanged node
  - If key > existing: insert into right subtree
  - If key < existing: insert into left subtree

## balance and Rotations

- AVL balancing follows the standard algorithm: check height difference, apply single or double rotations.
- `do_balance3()` and `do_balance4()` implement the "double rotation" cases (right-left and left-right).
- `difference_in_height()` returns `left_height - right_height`. Balance is needed when |difference| > 1.

## replace vs add

- `replace()` does NOT use the update check function. It directly replaces the value for an existing key.
- `replace()` traverses to the node by key and swaps the value, without rebalancing.

## add_unique

- `add_unique()` returns both the tree and the item that was inserted or already existed.
- Unlike `add()`, `add_unique()` never updates an existing key. If the key exists, it returns the existing item.
- No update check function is used in `add_unique()`.

## Printing Functions

- `pretty_print_tree_str()` and `print_tree_str()` check `has_printing_functions()` first.
  If printing functions are not set, returns an error string.
- `print_item_str()` calls the key and value string conversion functions stored in the tree.
  If these are not set, this will panic (unwrap on None).

## getKeyOfVal

- Uses `matchcontinue` pattern: tries current node first, then left subtree, then right subtree.
- Returns the first key found with the matching value (DFS traversal).
- If no key is found, returns an error.

## Potential Issues

1. **Trait object overhead**: Storing callbacks as `Box<dyn Trait>` adds heap allocation overhead.
   For performance-critical code, consider using generics with trait bounds instead.

2. **Panic on missing printing functions**: `get_key_to_str_func()` and `get_val_to_str_func()` use `.unwrap()`.
   Callers must ensure printing functions are set before calling `print_item_str()` directly.
   The `print_tree_str()` and `pretty_print_tree_str()` public functions check for this first.

3. **No PartialEq on K and V in some functions**: Some internal functions like `balance()` don't require
   `PartialEq` bounds, but public functions like `add()` and `replace()` do.

4. **Clone-heavy operations**: Many functions clone nodes throughout the tree. For large trees, this may
   be a performance concern. A future optimization could use interior mutability (`RefCell`, `Cell`)
   or a persistent tree structure.

5. **Error handling**: Most functions return `Result<T>` with `bail!` for error cases.
   The Mo `fail()` statement translates to `bail!("...")`.

6. **Naming conventions**: Enum variants `NO_NODE`, `NO_ITEM`, `ITEM`, `NODE`, and `TREE` follow
   the original Mo naming. They would normally be `NoNode`, `NoItem`, `Item`, `Node`, `Tree`
   in idiomatic Rust.
