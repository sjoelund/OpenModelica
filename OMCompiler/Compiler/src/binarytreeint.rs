//! Translation of BackEnd/BinaryTreeInt.mo
//!
//! Binary tree implementation for key-value pairs where both Key and Value are Integer (i32).
//! Supports insertion, lookup, and list conversion.

use anyhow::{bail, Result};
use im::Vector;

// ============================================================================
// Persistent list type
// ============================================================================

/// Persistent list type (maps from MetaModelica list<T>).
type List<T> = Vector<T>;

// ============================================================================
// Type definitions
// ============================================================================

/// Key type (Integer in MetaModelica).
pub type Key = i32;

/// Value type (Integer in MetaModelica).
pub type Value = i32;

/// Tree value stored in a node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TREEVALUE {
    pub key: Key,
    pub value: Value,
}

/// Optional tree value (Option<TreeValue>).
pub type OptTreeValue = Option<TREEVALUE>;

/// Optional binary tree (Option<BinTree>).
pub type OptBinTree = Option<Box<BinTree>>;

/// Binary tree node record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TREENODE {
    pub value: OptTreeValue,
    pub leftSubTree: OptBinTree,
    pub rightSubTree: OptBinTree,
}

/// Binary tree union type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinTree {
    TREENODE(TREENODE),
    NONE,
}

impl BinTree {
    fn new_empty() -> Self {
        BinTree::TREENODE(TREENODE {
            value: None,
            leftSubTree: None,
            rightSubTree: None,
        })
    }

    fn is_empty(&self) -> bool {
        matches!(self, BinTree::NONE)
            || matches!(self, BinTree::TREENODE(TREENODE {
                value: None,
                leftSubTree: None,
                rightSubTree: None,
            }))
    }
}

/// Empty binary tree constant.
pub fn empty_bin_tree() -> BinTree {
    BinTree::TREENODE(TREENODE {
        value: None,
        leftSubTree: None,
        rightSubTree: None,
    })
}

// ============================================================================
// keyCmp - compare two keys
// ============================================================================

/// Returns -1 if keya < keyb, 0 if equal, 1 if keya > keyb.
/// Matches: `cmp := Util.intSign(keya-keyb)`
pub fn key_cmp(keya: Key, keyb: Key) -> i32 {
    let diff = keya - keyb;
    if diff == 0 {
        0
    } else if diff > 0 {
        1
    } else {
        -1
    }
}

// ============================================================================
// treeGet - get a value by key from the tree
// ============================================================================

/// Public API: look up a value by key in the binary tree.
/// Matches: `v := treeGet3(bt, key, treeGet2(bt, key))`
pub fn tree_get(bt: &BinTree, key: Key) -> Result<Value> {
    let comp_result = tree_get2(bt, key)?;
    tree_get3(bt, key, comp_result)
}

/// Helper: compare the key at the current tree node with the target key.
/// Returns -1, 0, or 1. Only matches if the current node has a value.
fn tree_get2(in_bin_tree: &BinTree, ikey: Key) -> Result<i32> {
    match in_bin_tree {
        BinTree::TREENODE(TREENODE {
            value: Some(tree_value),
            ..
        }) => {
            let key = tree_value.key.clone();
            Ok(key_cmp(key, ikey))
        }
        _ => bail!("treeGet2: node has no value"),
    }
}

/// Helper: recursively look up a value based on comparison result.
/// 0 -> found, 1 -> search right, -1 -> search left.
fn tree_get3(in_bin_tree: &BinTree, ikey: Key, in_comp_result: i32) -> Result<Value> {
    match in_bin_tree {
        // Found it (comparison = 0)
        BinTree::TREENODE(TREENODE {
            value: Some(tree_value),
            ..
        }) if in_comp_result == 0 => Ok(tree_value.value.clone()),

        // Search right subtree (comparison = 1)
        BinTree::TREENODE(TREENODE {
            rightSubTree: Some(right),
            ..
        }) if in_comp_result == 1 => {
            let comp_result = tree_get2(right, ikey)?;
            tree_get3(right, ikey, comp_result)
        }

        // Search left subtree (comparison = -1)
        BinTree::TREENODE(TREENODE {
            leftSubTree: Some(left),
            ..
        }) if in_comp_result == -1 => {
            let comp_result = tree_get2(left, ikey)?;
            tree_get3(left, ikey, comp_result)
        }

        // Not found
        _ => bail!("treeGet3: key not found in tree"),
    }
}

// ============================================================================
// treeAddList - add a list of keys to the tree
// ============================================================================

/// Adds each key from the list to the tree (with default value 0).
/// Recursively processes the list, adding each key one by one.
pub fn tree_add_list(in_bin_tree: &BinTree, in_key_lst: List<Key>) -> BinTree {
    match in_key_lst.is_empty() {
        true => in_bin_tree.clone(),
        false => {
            let mut iter = in_key_lst.into_iter();
            let key = iter.next().unwrap();
            let res: List<Key> = iter.collect();
            let bt_1 = tree_add(in_bin_tree, key, 0).unwrap();
            tree_add_list(&bt_1, res)
        }
    }
}

// ============================================================================
// treeAdd - add a key-value pair to the tree (matchcontinue)
// ============================================================================

/// Insert a key-value pair into the binary tree.
/// Returns the updated tree, or fails if the tree structure is invalid.
pub fn tree_add(in_bin_tree: &BinTree, in_key: Key, in_value: Value) -> Result<BinTree> {
    // Case 1: Empty tree -> create new node
    if in_bin_tree.is_empty() {
        return Ok(BinTree::TREENODE(TREENODE {
            value: Some(TREEVALUE {
                key: in_key,
                value: in_value,
            }),
            leftSubTree: None,
            rightSubTree: None,
        }));
    }

    // Cases 2-7: Non-empty tree with a value
    if let BinTree::TREENODE(TREENODE {
        value: opt_tree_value,
        leftSubTree,
        rightSubTree,
    }) = in_bin_tree
    {
        if let Some(tree_value) = opt_tree_value {
            let rkey = tree_value.key;

            // Case 2: Key already exists -> replace value
            if key_cmp(rkey, in_key) == 0 {
                return Ok(BinTree::TREENODE(TREENODE {
                    value: Some(TREEVALUE {
                        key: rkey,
                        value: in_value,
                    }),
                    leftSubTree: leftSubTree.clone(),
                    rightSubTree: rightSubTree.clone(),
                }));
            }

            // Cases 3-4: Key > node key
            if key_cmp(rkey, in_key) == 1 {
                match rightSubTree.as_ref() {
                    // Case 3: Right subtree exists -> recurse
                    Some(right) => {
                        let t_1 = tree_add(right, in_key, in_value)?;
                        return Ok(BinTree::TREENODE(TREENODE {
                            value: Some(tree_value.clone()),
                            leftSubTree: leftSubTree.clone(),
                            rightSubTree: Some(Box::new(t_1)),
                        }));
                    }
                    // Case 4: Right subtree empty -> create new right node
                    None => {
                        let right_1 = tree_add(
                            &BinTree::TREENODE(TREENODE {
                                value: None,
                                leftSubTree: None,
                                rightSubTree: None,
                            }),
                            in_key,
                            in_value,
                        )?;
                        return Ok(BinTree::TREENODE(TREENODE {
                            value: Some(tree_value.clone()),
                            leftSubTree: leftSubTree.clone(),
                            rightSubTree: Some(Box::new(right_1)),
                        }));
                    }
                }
            }

            // Cases 5-6: Key < node key
            if key_cmp(rkey, in_key) == -1 {
                match leftSubTree.as_ref() {
                    // Case 5: Left subtree exists -> recurse left
                    Some(left) => {
                        let t_1 = tree_add(left, in_key, in_value)?;
                        return Ok(BinTree::TREENODE(TREENODE {
                            value: Some(tree_value.clone()),
                            leftSubTree: Some(Box::new(t_1)),
                            rightSubTree: rightSubTree.clone(),
                        }));
                    }
                    // Case 6: Left subtree empty -> create new left node
                    None => {
                        let left_1 = tree_add(
                            &BinTree::TREENODE(TREENODE {
                                value: None,
                                leftSubTree: None,
                                rightSubTree: None,
                            }),
                            in_key,
                            in_value,
                        )?;
                        return Ok(BinTree::TREENODE(TREENODE {
                            value: Some(tree_value.clone()),
                            leftSubTree: Some(Box::new(left_1)),
                            rightSubTree: rightSubTree.clone(),
                        }));
                    }
                }
            }
        }
    }

    // Case 7: else -> fail (should never happen with valid input)
    bail!("BinaryTreeInt.treeAdd failed")
}

// ============================================================================
// bintreeToList - convert tree to list of keys and values
// ============================================================================

/// Convert a binary tree to a list of keys and values.
/// Uses the helper function bintreeToList2 with empty accumulators.
pub fn bintree_to_list(in_bin_tree: &BinTree) -> Result<(List<Key>, List<Value>)> {
    match bintree_to_list2(in_bin_tree, List::new(), List::new()) {
        Ok(result) => Ok(result),
        Err(_) => bail!("BackendDAEUtil.bintreeToList failed"),
    }
}

// ============================================================================
// bintreeToList2 - helper for bintreeToList (matchcontinue)
// ============================================================================

/// Helper: traverse tree in-order, accumulating keys and values in lists.
/// Note: This function has a bug in the original - it processes 'left' twice
/// instead of 'right' in the third case. Translated as-is.
fn bintree_to_list2(
    in_bin_tree: &BinTree,
    klst: List<Key>,
    vlst: List<Value>,
) -> Result<(List<Key>, List<Value>)> {
    // Case 1: Empty tree -> return accumulated lists
    if in_bin_tree.is_empty() {
        return Ok((klst, vlst));
    }

    // Case 2: Node with value -> add key/value to front, recurse into subtrees
    if let BinTree::TREENODE(TREENODE {
        value: Some(tree_value),
        leftSubTree,
        rightSubTree,
    }) = in_bin_tree
    {
        let key = tree_value.key;
        let value = tree_value.value;

        let (klst, vlst) = bintree_to_list_opt(leftSubTree, klst, vlst)?;
        let (klst, vlst) = bintree_to_list_opt(rightSubTree, klst, vlst)?;
        let klst = List::from_iter(Some(key).into_iter().chain(klst));
        let vlst = List::from_iter(Some(value).into_iter().chain(vlst));
        return Ok((klst, vlst));
    }

    // Case 3: Degenerate node (no value, has left subtree, no right subtree)
    // Note: Bug in original - processes left twice instead of left and right
    if let BinTree::TREENODE(TREENODE {
        value: None,
        leftSubTree: Some(_),
        rightSubTree: None,
    }) = in_bin_tree
    {
        // Need to re-borrow the full tree for recursive calls
        // This case has a bug in the original - it processes the same subtree twice
        if let BinTree::TREENODE(TREENODE { leftSubTree: opt, .. }) = in_bin_tree {
            let (klst, vlst) = bintree_to_list_opt(opt, klst, vlst)?;
            let (klst, vlst) = bintree_to_list_opt(opt, klst, vlst)?;
            return Ok((klst, vlst));
        }
    }

    bail!("bintreeToList2: no matching case")
}

// ============================================================================
// bintreeToListOpt - helper to handle Option<BinTree>
// ============================================================================

/// Helper: process an optional subtree.
/// None -> return accumulators unchanged.
/// Some(tree) -> recurse into it.
fn bintree_to_list_opt(
    in_bin_tree_opt: &OptBinTree,
    klst: List<Key>,
    vlst: List<Value>,
) -> Result<(List<Key>, List<Value>)> {
    match in_bin_tree_opt {
        None => Ok((klst, vlst)),
        Some(bt) => bintree_to_list2(bt, klst, vlst),
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_cmp() {
        assert_eq!(key_cmp(5, 5), 0);
        assert_eq!(key_cmp(5, 3), 1);
        assert_eq!(key_cmp(3, 5), -1);
    }

    #[test]
    fn test_empty_bin_tree() {
        let tree = empty_bin_tree();
        assert!(tree.is_empty());
    }

    #[test]
    fn test_tree_add_empty() {
        let empty = empty_bin_tree();
        let result = tree_add(&empty, 5, 10).unwrap();
        assert!(!result.is_empty());
    }

    #[test]
    fn test_tree_add_and_get() {
        let empty = empty_bin_tree();
        let tree = tree_add(&empty, 5, 10).unwrap();
        let val = tree_get(&tree, 5).unwrap();
        assert_eq!(val, 10);
    }

    #[test]
    fn test_tree_add_multiple() {
        let empty = empty_bin_tree();
        let t1 = tree_add(&empty, 5, 10).unwrap();
        let t2 = tree_add(&t1, 3, 20).unwrap();
        let t3 = tree_add(&t2, 7, 30).unwrap();

        assert_eq!(tree_get(&t3, 5).unwrap(), 10);
        assert_eq!(tree_get(&t3, 3).unwrap(), 20);
        assert_eq!(tree_get(&t3, 7).unwrap(), 30);
    }

    #[test]
    fn test_tree_add_replace() {
        let empty = empty_bin_tree();
        let t1 = tree_add(&empty, 5, 10).unwrap();
        let t2 = tree_add(&t1, 5, 99).unwrap();
        assert_eq!(tree_get(&t2, 5).unwrap(), 99);
    }

    #[test]
    fn test_tree_get_not_found() {
        let empty = empty_bin_tree();
        let t1 = tree_add(&empty, 5, 10).unwrap();
        assert!(tree_get(&t1, 99).is_err());
    }

    #[test]
    fn test_tree_add_list() {
        let empty = empty_bin_tree();
        let keys = List::from_iter(vec![1, 2, 3].into_iter());
        let result = tree_add_list(&empty, keys);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_bintree_to_list() {
        let empty = empty_bin_tree();
        let t1 = tree_add(&empty, 3, 30).unwrap();
        let t2 = tree_add(&t1, 1, 10).unwrap();
        let t3 = tree_add(&t2, 5, 50).unwrap();
        let (keys, values) = bintree_to_list(&t3).unwrap();
        assert_eq!(keys.len(), 3);
        assert_eq!(values.len(), 3);
    }

    #[test]
    fn test_bintree_to_list_empty() {
        let empty = empty_bin_tree();
        let (keys, values) = bintree_to_list(&empty).unwrap();
        assert_eq!(keys.len(), 0);
        assert_eq!(values.len(), 0);
    }
}
