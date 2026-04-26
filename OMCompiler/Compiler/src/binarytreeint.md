# BinaryTreeInt Translation Notes

## Assumptions

- **Key/Value types**: Both `Key` and `Value` are mapped to `i32` (Integer in MetaModelica).
- **Option type**: `Option<T>` in MetaModelica is mapped to Rust's standard `Option<T>`.
- **List type**: `list<Key>` from MetaModelica is mapped to `im::Vector<Key>` (persistent list).
- **Empty tree**: The `NONE()` in the MetaModelica `BinTree` type is represented both as `BinTree::NONE` variant and as a `TREENODE` with all `None` fields. The `is_empty()` method treats both as empty.
- **Box for recursion**: `Option<BinTree>` requires `Box` indirection in Rust to avoid infinite size, so the actual type is `Option<Box<BinTree>>`.

## matchcontinue Translation

The `matchcontinue` construct is translated to a chain of `if`/`match` statements with `bail!()` for non-matching cases. Each case that doesn't match falls through to the next via `bail!()`, and the final else case calls `bail!("BinaryTreeInt.treeAdd failed")`.

## Potential Issues

1. **treeAdd recursive empty-tree wrapper**: In cases 4 and 6 (creating new leaf nodes), the code wraps the new node in a `TREENODE(NONE(), NONE(), NONE())` before calling `tree_add`. This is necessary because `tree_add` expects a `&BinTree` reference. In practice, this is safe because an empty tree will always match the first case.

2. **bintreeToList2 bug**: The original MetaModelica code has a bug in the third case of `bintreeToList2` where it processes the `left` subtree twice instead of processing `left` and `right` separately. This is translated as-is and preserved as a comment.

3. **treeGet always checks current node first**: The `treeGet` implementation always checks the current node before recursing, even though the original MetaModelica code seems to imply that `treeGet2` would only match if the key was at the current position. This means for a tree with 3 nodes, looking up the left child requires checking 2 nodes first (right child and root) even though the tree structure might suggest a more direct path. The behavior is still correct but may be slightly less efficient than expected.

4. **Commented-out functions**: The following functions from the original are commented out and not translated:
   - `treeDelete2` - deletion from tree
   - `treeDeleteRightmostValue` - helper for deletion
   - `treePruneEmptyNodes` - helper for deletion
   - `bintreeDepth` - calculates tree depth (references `intMax` which is not defined in Util.mo)

5. **intMax not available**: The commented-out `bintreeDepth` function references `intMax` which is not defined in `Util.mo`. It exists in other packages (`FrontEnd/MetaModelicaBuiltin.mo`) but would require importing.

6. **Error.addMessage**: The `Error.addMessage` calls in the fail cases are replaced with `bail!()` since the error handling system is not fully integrated in the Rust translation.

7. **treeAddList default value**: The `treeAddList` function adds keys with a default value of `0` since the MetaModelica version only passes keys (not key-value pairs) to the function.
