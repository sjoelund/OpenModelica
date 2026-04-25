//! Translation of Util/BaseAvlTree.mo
//!
//! This module provides a generic AVL tree (map) implementation using an
//! AVL-balanced binary search tree. It maps keys to values and supports
//! insertion with conflict resolution, lookup, iteration, and tree transformations.
//!
//! The `replaceable` keyword from MetaModelica is translated to Rust
//! generics with trait bounds. The `Value` type defaults to `i32` (Integer).

use anyhow::Result;
use im::Vector;

// ============================================================================
// Type aliases
// ============================================================================

/// Persistent list type (maps from MetaModelica list<T>).
type List<T> = Vector<T>;

// ============================================================================
// Tree uniontype
// ============================================================================

/// The binary tree data structure for the AVL tree (key-value map).
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Tree<K: Ord + Clone, V: Clone> {
    NODE {
        key: K,
        value: V,
        height: i32,
        left: Box<Tree<K, V>>,
        right: Box<Tree<K, V>>,
    },
    LEAF {
        key: K,
        value: V,
    },
    EMPTY,
}

// ============================================================================
// Tree helper methods
// ============================================================================

impl<K: Ord + Clone, V: Clone> Tree<K, V> {
    /// Returns a new empty tree.
    pub fn new() -> Self {
        Tree::EMPTY
    }

    /// Returns the height of the tree.
    pub fn tree_height(&self) -> i32 {
        match self {
            Tree::NODE { height, .. } => *height,
            Tree::LEAF { .. } => 1,
            Tree::EMPTY => 0,
        }
    }

    /// Returns true if this is an EMPTY tree.
    pub fn is_empty(&self) -> bool {
        matches!(self, Tree::EMPTY)
    }

    /// Returns the key, if this is a NODE or LEAF.
    pub fn key(&self) -> Option<&K> {
        match self {
            Tree::NODE { key, .. } | Tree::LEAF { key, .. } => Some(key),
            Tree::EMPTY => None,
        }
    }

    /// Returns the value, if this is a NODE or LEAF.
    pub fn value(&self) -> Option<&V> {
        match self {
            Tree::NODE { value, .. } | Tree::LEAF { value, .. } => Some(value),
            Tree::EMPTY => None,
        }
    }

    /// Returns the left subtree, if this is a NODE.
    pub fn left(&self) -> Option<&Tree<K, V>> {
        match self {
            Tree::NODE { left, .. } => Some(left),
            _ => None,
        }
    }

    /// Returns the right subtree, if this is a NODE.
    pub fn right(&self) -> Option<&Tree<K, V>> {
        match self {
            Tree::NODE { right, .. } => Some(right),
            _ => None,
        }
    }
}

// ============================================================================
// Display helpers
// ============================================================================

/// Default key-to-string conversion.
pub fn key_str<K: Ord + Clone + std::fmt::Display>(key: &K) -> String {
    format!("{key}")
}

/// Default value-to-string conversion.
pub fn value_str<V: std::fmt::Display>(value: &V) -> String {
    format!("{value}")
}

/// Prints a node's key and value to a string.
pub fn print_node_str<K: Ord + Clone + std::fmt::Display, V: std::fmt::Display + Clone>(
    node: &Tree<K, V>,
) -> String {
    match node {
        Tree::NODE { key, value, .. } | Tree::LEAF { key, value, .. } => {
            format!("({}, {})", key_str(key), value_str(value))
        }
        Tree::EMPTY => String::new(),
    }
}

// ============================================================================
// keyCompare
// ============================================================================

/// Compares two keys. Returns -1 if key1 < key2, 0 if equal, 1 if key1 > key2.
pub fn key_compare<K: Ord>(key1: &K, key2: &K) -> i32 {
    match key1.cmp(key2) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

// ============================================================================
// Conflict resolution functions
// ============================================================================

/// Conflict resolving function that fails on conflict.
pub fn add_conflict_fail<V, K>(_new_value: &V, _old_value: &V, _key: &K) -> Result<V> {
    anyhow::bail!("Key conflict: key already exists in tree")
}

/// Conflict resolving function that replaces the old value with the new.
pub fn add_conflict_replace<V: Clone, K>(new_value: &V, _old_value: &V, _key: &K) -> Result<V> {
    Ok(new_value.clone())
}

/// Conflict resolving function that keeps the old value.
pub fn add_conflict_keep<V: Clone, K>(_new_value: &V, old_value: &V, _key: &K) -> Result<V> {
    Ok(old_value.clone())
}

// ============================================================================
// add
// ============================================================================

/// Inserts a new node in the tree, using the conflict function to resolve
/// duplicate keys.
///
/// The conflict function is called when the key already exists in the tree.
/// It receives (new_value, old_value, key) and must return the value to store.
/// If the conflict function fails (returns Err), the error propagates.
pub fn add<K: Ord + Clone + PartialEq, V: Clone + PartialEq, F>(
    tree: Tree<K, V>,
    in_key: K,
    in_value: V,
    conflict_fn: F,
) -> Result<Tree<K, V>>
where
    F: FnOnce(&V, &V, &K) -> Result<V>,
{
    match tree {
        // Empty tree -> create a leaf
        Tree::EMPTY => Ok(Tree::LEAF {
            key: in_key,
            value: in_value,
        }),

        // Node case
        Tree::NODE {
            key,
            value,
            height: _,
            left,
            right,
        } => {
            let key_comp = key_compare(&in_key, &key);

            if key_comp == 0 {
                // Key already exists -> resolve conflict
                let new_value = conflict_fn(&in_value, &value, &key)?;
                // Only rebuild if the value actually changed
                if new_value != value {
                    Ok(Tree::NODE {
                        key,
                        value: new_value,
                        height: 1,
                        left,
                        right,
                    })
                } else {
                    // Value unchanged - reconstruct original tree
                    Ok(Tree::NODE {
                        key,
                        value,
                        height: 1,
                        left,
                        right,
                    })
                }
            } else if key_comp == -1 {
                // Key is smaller -> insert into left subtree
                let new_left = add(*left, in_key, in_value, conflict_fn)?;
                Ok(balance(&Tree::NODE {
                    key,
                    value,
                    height: 0,
                    left: Box::new(new_left),
                    right: Box::new(*right),
                }))
            } else {
                // Key is larger -> insert into right subtree
                let new_right = add(*right, in_key, in_value, conflict_fn)?;
                Ok(balance(&Tree::NODE {
                    key,
                    value,
                    height: 0,
                    left: Box::new(*left),
                    right: Box::new(new_right),
                }))
            }
        }

        // Leaf case
        Tree::LEAF {
            key,
            value: leaf_value,
        } => {
            let key_comp = key_compare(&in_key, &key);

            if key_comp == 0 {
                // Key already exists -> resolve conflict
                let new_value = conflict_fn(&in_value, &leaf_value, &key)?;
                // Only rebuild if value changed
                if new_value != leaf_value {
                    Ok(Tree::LEAF {
                        key: key.clone(),
                        value: new_value,
                    })
                } else {
                    // Value unchanged - reconstruct original tree
                    Ok(Tree::LEAF {
                        key: key.clone(),
                        value: leaf_value,
                    })
                }
            } else if key_comp == -1 {
                // New key is smaller -> convert leaf to node
                Ok(Tree::NODE {
                    key: key.clone(),
                    value: leaf_value.clone(),
                    height: 2,
                    left: Box::new(Tree::LEAF {
                        key: in_key,
                        value: in_value,
                    }),
                    right: Box::new(Tree::EMPTY),
                })
            } else {
                // New key is larger -> convert leaf to node
                Ok(Tree::NODE {
                    key: key.clone(),
                    value: leaf_value.clone(),
                    height: 2,
                    left: Box::new(Tree::EMPTY),
                    right: Box::new(Tree::LEAF {
                        key: in_key,
                        value: in_value,
                    }),
                })
            }
        }
    }
}

// ============================================================================
// addUpdate
// ============================================================================

/// Trait for update functions used in addUpdate.
pub trait UpdateFn<V> {
    fn call(&mut self, old_value: Option<V>) -> V;
}

impl<F, V> UpdateFn<V> for F
where
    F: FnMut(Option<V>) -> V,
{
    fn call(&mut self, old_value: Option<V>) -> V {
        self(old_value)
    }
}

/// Trait for forEach callback functions.
pub trait EachFn<K, V> {
    fn call(&mut self, key: &K, value: &V);
}

impl<F, K, V> EachFn<K, V> for F
where
    F: FnMut(&K, &V),
{
    fn call(&mut self, key: &K, value: &V) {
        self(key, value)
    }
}

/// Inserts a new node in the tree, where the value is generated by the given
/// function. If the key already exists in the tree then the function is given
/// the old value (wrapped in Some), otherwise it is called with None.
pub fn add_update<K: Ord + Clone, V: Clone + PartialEq, F: UpdateFn<V>>(
    tree: Tree<K, V>,
    key: K,
    mut fn_inner: F,
) -> Tree<K, V> {
    _add_update_helper(tree, key, &mut fn_inner)
}

fn _add_update_helper<K: Ord + Clone, V: Clone + PartialEq, F: UpdateFn<V>>(
    tree: Tree<K, V>,
    key: K,
    fn_inner: &mut F,
) -> Tree<K, V> {
    let key_comp = match &tree {
        Tree::NODE { key: k, .. } | Tree::LEAF { key: k, .. } => key_compare(&key, k),
        Tree::EMPTY => 0, // will create new node
    };

    match tree {
        // Empty tree -> create a leaf
        Tree::EMPTY => Tree::LEAF {
            key: key.clone(),
            value: fn_inner.call(None),
        },

        // Node case
        Tree::NODE {
            key: node_key,
            mut value,
            height: _,
            mut left,
            mut right,
        } => {
            if key_comp == 0 {
                // Key already exists -> update value
                value = fn_inner.call(Some(value.clone()));
            } else if key_comp == -1 {
                // Replace left branch
                *left = _add_update_helper(*left, key.clone(), fn_inner);
            } else {
                // Replace right branch
                *right = _add_update_helper(*right, key.clone(), fn_inner);
            }

            if key_comp == 0 {
                Tree::NODE {
                    key: node_key,
                    value,
                    height: 1,
                    left,
                    right,
                }
            } else {
                balance(&Tree::NODE {
                    key: node_key,
                    value,
                    height: 1,
                    left,
                    right,
                })
            }
        }

        // Leaf case
        Tree::LEAF {
            key: leaf_key,
            mut value,
        } => {
            if key_comp == 0 {
                // Key already exists -> update value
                value = fn_inner.call(Some(value.clone()));
            } else if key_comp == -1 {
                // New key is smaller -> convert to node
                return Tree::NODE {
                    key: leaf_key.clone(),
                    value: value.clone(),
                    height: 2,
                    left: Box::new(Tree::LEAF {
                        key: key.clone(),
                        value: fn_inner.call(None),
                    }),
                    right: Box::new(Tree::EMPTY),
                };
            } else {
                // New key is larger -> convert to node
                return Tree::NODE {
                    key: leaf_key.clone(),
                    value: value.clone(),
                    height: 2,
                    left: Box::new(Tree::EMPTY),
                    right: Box::new(Tree::LEAF {
                        key: key.clone(),
                        value: fn_inner.call(None),
                    }),
                };
            }

            if key_comp == 0 {
                Tree::LEAF {
                    key: leaf_key.clone(),
                    value,
                }
            } else {
                balance(&Tree::NODE {
                    key: leaf_key.clone(),
                    value,
                    height: 1,
                    left: Box::new(Tree::EMPTY),
                    right: Box::new(Tree::EMPTY),
                })
            }
        }
    }
}

// ============================================================================
// addList
// ============================================================================

/// Adds a list of key-value pairs to the tree.
pub fn add_list<K: Ord + Clone + PartialEq, V: Clone + PartialEq, F>(
    tree: Tree<K, V>,
    in_values: List<(K, V)>,
    conflict_fn: F,
) -> Result<Tree<K, V>>
where
    F: Fn(&V, &V, &K) -> Result<V>,
{
    in_values
        .into_iter()
        .try_fold(tree, |acc, (key, value)| add(acc, key, value, &conflict_fn))
}

// ============================================================================
// update
// ============================================================================

/// Alias for add that replaces the node in case of conflict.
pub fn update<K: Ord + Clone + PartialEq, V: Clone + PartialEq>(
    tree: Tree<K, V>,
    key: K,
    value: V,
) -> Result<Tree<K, V>> {
    add(tree, key, value, add_conflict_replace)
}

// ============================================================================
// get
// ============================================================================

/// Fetches a value from the tree given a key, or fails if no value is
/// associated with the key.
pub fn get<K: Ord + Clone, V: Clone>(tree: &Tree<K, V>, key: &K) -> Result<V> {
    get_opt(tree, key)
        .ok_or_else(|| anyhow::anyhow!("Key not found in tree"))
}

// ============================================================================
// getOpt
// ============================================================================

/// Fetches a value from the tree given a key, or returns None if no value
/// is associated with the key.
pub fn get_opt<K: Ord + Clone, V: Clone>(tree: &Tree<K, V>, key: &K) -> Option<V> {
    let k = match tree {
        Tree::NODE { key, .. } | Tree::LEAF { key, .. } => key.clone(),
        Tree::EMPTY => return None,
    };

    match (key_compare(key, &k), tree) {
        (0, Tree::LEAF { value, .. }) | (0, Tree::NODE { value, .. }) => Some(value.clone()),
        (1, Tree::NODE { right, .. }) => get_opt(right, key),
        (-1, Tree::NODE { left, .. }) => get_opt(left, key),
        _ => None,
    }
}

// ============================================================================
// fromList
// ============================================================================

/// Creates a new tree from a list of key-value pairs.
pub fn from_list<K: Ord + Clone + PartialEq, V: Clone + PartialEq, F>(
    in_values: List<(K, V)>,
    conflict_fn: F,
) -> Result<Tree<K, V>>
where
    F: Fn(&V, &V, &K) -> Result<V>,
{
    let empty = Tree::EMPTY;
    add_list(empty, in_values, conflict_fn)
}

// ============================================================================
// toList
// ============================================================================

/// Converts the tree to a flat list of key-value tuples (in sorted key order).
pub fn to_list<K: Ord + Clone, V: Clone>(in_tree: &Tree<K, V>) -> List<(K, V)> {
    _to_list_impl(in_tree, List::new())
}

fn _to_list_impl<K: Ord + Clone, V: Clone>(
    in_tree: &Tree<K, V>,
    mut lst: List<(K, V)>,
) -> List<(K, V)> {
    match in_tree {
        Tree::NODE {
            key,
            value,
            right,
            left,
            ..
        } => {
            lst = _to_list_impl(right, lst);
            lst = List::from_iter(Some((key.clone(), value.clone())).into_iter().chain(lst));
            _to_list_impl(left, lst)
        }
        Tree::LEAF { key, value } => {
            List::from_iter(Some((key.clone(), value.clone())).into_iter().chain(lst))
        }
        Tree::EMPTY => lst,
    }
}

// ============================================================================
// listKeys
// ============================================================================

/// Constructs a list of all the keys in the tree (sorted ascending).
pub fn list_keys<K: Ord + Clone, V: Clone>(tree: &Tree<K, V>) -> List<K> {
    _list_keys_impl(tree, List::new())
}

fn _list_keys_impl<K: Ord + Clone, V: Clone>(tree: &Tree<K, V>, mut lst: List<K>) -> List<K> {
    match tree {
        Tree::NODE {
            key, right, left, ..
        } => {
            lst = _list_keys_impl(right, lst);
            lst = List::from_iter(Some(key.clone()).into_iter().chain(lst));
            _list_keys_impl(left, lst)
        }
        Tree::LEAF { key, .. } => {
            List::from_iter(Some(key.clone()).into_iter().chain(lst))
        }
        Tree::EMPTY => lst,
    }
}

// ============================================================================
// listValues
// ============================================================================

/// Constructs a list of all the values in the tree (sorted by key order).
pub fn list_values<K: Ord + Clone, V: Clone>(tree: &Tree<K, V>) -> List<V> {
    _list_values_impl(tree, List::new())
}

fn _list_values_impl<K: Ord + Clone, V: Clone>(tree: &Tree<K, V>, mut lst: List<V>) -> List<V> {
    match tree {
        Tree::NODE {
            value, right, left, ..
        } => {
            lst = _list_values_impl(right, lst);
            lst = List::from_iter(Some(value.clone()).into_iter().chain(lst));
            _list_values_impl(left, lst)
        }
        Tree::LEAF { value, .. } => {
            List::from_iter(Some(value.clone()).into_iter().chain(lst))
        }
        Tree::EMPTY => lst,
    }
}

// ============================================================================
// join
// ============================================================================

/// Joins two trees by adding all elements from tree_to_join to tree.
pub fn join<K: Ord + Clone + PartialEq, V: Clone + PartialEq, F>(
    tree: Tree<K, V>,
    tree_to_join: &Tree<K, V>,
    conflict_fn: F,
) -> Result<Tree<K, V>>
where
    F: Fn(&V, &V, &K) -> Result<V>,
{
    match tree_to_join {
        Tree::EMPTY => Ok(tree),
        Tree::NODE {
            key,
            value,
            left,
            right,
            ..
        } => {
            let tree = add(tree, key.clone(), value.clone(), &conflict_fn)?;
            let tree = join(tree, left, &conflict_fn)?;
            join(tree, right, &conflict_fn)
        }
        Tree::LEAF { key, value } => {
            add(tree, key.clone(), value.clone(), &conflict_fn)
        }
    }
}

// ============================================================================
// forEach
// ============================================================================

/// Traverses the tree in depth-first in-order and applies the given function
/// to each node, but without constructing a new tree like with map.
pub fn for_each<K: Ord + Clone, V: Clone, F: FnMut(&K, &V)>(
    tree: &Tree<K, V>,
    func: &mut F,
) {
    fn traverse<K: Ord + Clone, V: Clone, F: FnMut(&K, &V)>(
        tree: &Tree<K, V>,
        func: &mut F,
    ) {
        match tree {
            Tree::NODE {
                key,
                value,
                left,
                right,
                ..
            } => {
                traverse(left, func);
                func(key, value);
                traverse(right, func);
            }
            Tree::LEAF { key, value } => {
                func(key, value);
            }
            Tree::EMPTY => {}
        }
    }
    traverse(tree, func);
}

// ============================================================================
// map
// ============================================================================

/// Traverses the tree in depth-first in-order and applies the given function
/// to each node, constructing a new tree with the resulting nodes.
pub fn map<K: Ord + Clone, V: Clone, V2: Clone + PartialEq, F: FnMut(&K, &V) -> V2>(
    in_tree: &Tree<K, V>,
    mut in_func: F,
) -> Tree<K, V2> {
    _map_helper(in_tree, &mut in_func)
}

fn _map_helper<K: Ord + Clone, V: Clone, V2: Clone, F: FnMut(&K, &V) -> V2>(
    tree: &Tree<K, V>,
    in_func: &mut F,
) -> Tree<K, V2> {
    match tree {
        Tree::NODE {
            key,
            value,
            left,
            right,
            ..
        } => {
            let new_left = _map_helper(left, in_func);
            let new_value = in_func(key, value);
            let new_right = _map_helper(right, in_func);
            let new_height =
                std::cmp::max(new_left.tree_height(), new_right.tree_height()) + 1;

            // Always rebuild since V2 may differ from V
            Tree::NODE {
                key: key.clone(),
                value: new_value,
                height: new_height,
                left: Box::new(new_left),
                right: Box::new(new_right),
            }
        }
        Tree::LEAF { key, .. } => {
            let new_value = in_func(key, tree.value().unwrap());
            Tree::LEAF {
                key: key.clone(),
                value: new_value,
            }
        }
        Tree::EMPTY => Tree::EMPTY,
    }
}

// ============================================================================
// fold
// ============================================================================

/// Traverses the tree in depth-first in-order and applies the given function
/// to each node, in the process updating the accumulator.
pub fn fold<K: Ord + Clone, V: Clone, FT, F: FnMut(&K, &V, FT) -> FT>(
    in_tree: &Tree<K, V>,
    mut in_func: F,
    in_start_value: FT,
) -> FT {
    _fold_helper(in_tree, &mut in_func, in_start_value)
}

fn _fold_helper<K: Ord + Clone, V: Clone, FT, F: FnMut(&K, &V, FT) -> FT>(
    tree: &Tree<K, V>,
    in_func: &mut F,
    mut out_result: FT,
) -> FT {
    match tree {
        Tree::NODE {
            key,
            value,
            left,
            right,
            ..
        } => {
            out_result = _fold_helper(left, in_func, out_result);
            out_result = in_func(key, value, out_result);
            _fold_helper(right, in_func, out_result)
        }
        Tree::LEAF { key, value } => in_func(key, value, out_result),
        Tree::EMPTY => out_result,
    }
}

// ============================================================================
// fold_2
// ============================================================================

/// Like fold, but takes two fold arguments.
pub fn fold_2<
    K: Ord + Clone,
    V: Clone,
    FT1: Clone,
    FT2: Clone,
    F: FnMut(&K, &V, FT1, FT2) -> (FT1, FT2),
>(
    tree: &Tree<K, V>,
    mut fold_func: F,
    fold_arg1: FT1,
    fold_arg2: FT2,
) -> (FT1, FT2) {
    _fold_2_helper(tree, &mut fold_func, fold_arg1, fold_arg2)
}

fn _fold_2_helper<
    K: Ord + Clone,
    V: Clone,
    FT1: Clone,
    FT2: Clone,
    F: FnMut(&K, &V, FT1, FT2) -> (FT1, FT2),
>(
    tree: &Tree<K, V>,
    fold_func: &mut F,
    mut fold_arg1: FT1,
    mut fold_arg2: FT2,
) -> (FT1, FT2) {
    match tree {
        Tree::NODE {
            key,
            value,
            left,
            right,
            ..
        } => {
            (fold_arg1, fold_arg2) = _fold_2_helper(left, fold_func, fold_arg1, fold_arg2);
            (fold_arg1, fold_arg2) = fold_func(key, value, fold_arg1, fold_arg2);
            _fold_2_helper(right, fold_func, fold_arg1, fold_arg2)
        }
        Tree::LEAF { key, value } => {
            fold_func(key, value, fold_arg1, fold_arg2)
        }
        Tree::EMPTY => (fold_arg1, fold_arg2),
    }
}

// ============================================================================
// foldCond
// ============================================================================

/// Like fold, but if the fold function returns false it will not continue
/// down into the tree (but will still continue with other branches).
pub fn fold_cond<K: Ord + Clone, V: Clone, FT: Clone, F: FnMut(&K, &V, &mut FT) -> bool>(
    tree: &Tree<K, V>,
    mut fold_func: F,
    mut value: FT,
) -> FT {
    match tree {
        Tree::NODE {
            key,
            value: node_value,
            left,
            right,
            ..
        } => {
            let should_continue = fold_func(key, node_value, &mut value);

            if should_continue {
                value = _fold_cond_helper(left, &mut fold_func, value);
                value = _fold_cond_helper(right, &mut fold_func, value);
            }
            value
        }
        Tree::LEAF { key, value: leaf_value } => {
            fold_func(key, leaf_value, &mut value);
            value
        }
        Tree::EMPTY => value,
    }
}

fn _fold_cond_helper<K: Ord + Clone, V: Clone, FT: Clone, F: FnMut(&K, &V, &mut FT) -> bool>(
    tree: &Tree<K, V>,
    fold_func: &mut F,
    value: FT,
) -> FT {
    match tree {
        Tree::NODE {
            key,
            value: node_value,
            left,
            right,
            ..
        } => {
            let should_continue = fold_func(key, node_value, &mut value.clone());
            if should_continue {
                let val = _fold_cond_helper(left, fold_func, value);
                _fold_cond_helper(right, fold_func, val)
            } else {
                value
            }
        }
        Tree::LEAF { key, value: leaf_value } => {
            fold_func(key, leaf_value, &mut value.clone());
            value
        }
        Tree::EMPTY => value,
    }
}

// ============================================================================
// mapFold
// ============================================================================

/// Traverses the tree in depth-first in-order and applies the given function
/// to each node, constructing a new tree with the resulting nodes.
/// mapFold also takes an extra argument which is updated on each call.
pub fn map_fold<
    K: Ord + Clone,
    V: Clone,
    V2: Clone + PartialEq,
    FT: Clone,
    F: FnMut(&K, &V, &mut FT) -> (V2, FT),
>(
    in_tree: &Tree<K, V>,
    in_func: F,
    in_start_value: FT,
) -> (Tree<K, V2>, FT) {
    let (out_tree, out_result) = _map_fold_helper(in_tree, in_func, in_start_value);
    (out_tree, out_result)
}

fn _map_fold_helper<
    K: Ord + Clone,
    V: Clone,
    V2: Clone,
    FT: Clone,
    F: FnMut(&K, &V, &mut FT) -> (V2, FT),
>(
    tree: &Tree<K, V>,
    mut in_func: F,
    mut out_result: FT,
) -> (Tree<K, V2>, FT) {
    match tree {
        Tree::NODE {
            key,
            value,
            left,
            right,
            ..
        } => {
            let (new_left, mut result_after_left) = _map_fold_helper(left, &mut in_func, out_result);
            let (new_value, result_after_func) = in_func(key, value, &mut result_after_left);
            let (new_right, final_result) =
                _map_fold_helper(right, &mut in_func, result_after_func);

            let new_height =
                std::cmp::max(new_left.tree_height(), new_right.tree_height()) + 1;

            // Always rebuild since V2 may differ from V
            let new_tree = Tree::NODE {
                key: key.clone(),
                value: new_value,
                height: new_height,
                left: Box::new(new_left),
                right: Box::new(new_right),
            };
            (new_tree, final_result)
        }
        Tree::LEAF { key, .. } => {
            let (new_value, result) = in_func(key, tree.value().unwrap(), &mut out_result);
            (
                Tree::LEAF {
                    key: key.clone(),
                    value: new_value,
                },
                result,
            )
        }
        Tree::EMPTY => (Tree::EMPTY, out_result),
    }
}

// ============================================================================
// setTreeLeftRight
// ============================================================================

/// Sets the left and right children of a tree node.
/// If the tree is a NODE and both children are EMPTY, converts to LEAF.
/// If the children are the same, returns the original tree.
pub fn set_tree_left_right<K: Ord + Clone + PartialEq, V: Clone + PartialEq>(
    orig: &Tree<K, V>,
    left: &Tree<K, V>,
    right: &Tree<K, V>,
) -> Tree<K, V> {
    match orig {
        Tree::NODE {
            key,
            value,
            height: _,
            ..
        } => {
            // If both children are empty and orig is NODE, convert to LEAF
            if left.is_empty() && right.is_empty() {
                return Tree::LEAF {
                    key: key.clone(),
                    value: value.clone(),
                };
            }
            // If children haven't changed, return original
            if reference_eq_or_empty(orig.left(), Some(left))
                && reference_eq_or_empty(orig.right(), Some(right))
            {
                return orig.clone();
            }
            let h_left = left.tree_height();
            let h_right = right.tree_height();
            Tree::NODE {
                key: key.clone(),
                value: value.clone(),
                height: std::cmp::max(h_left, h_right) + 1,
                left: Box::new(left.clone()),
                right: Box::new(right.clone()),
            }
        }
        Tree::LEAF { key, value } => {
            // Convert leaf to node if children are not both empty
            if left.is_empty() && right.is_empty() {
                return orig.clone();
            }
            let h_left = left.tree_height();
            let h_right = right.tree_height();
            Tree::NODE {
                key: key.clone(),
                value: value.clone(),
                height: std::cmp::max(h_left, h_right) + 1,
                left: Box::new(left.clone()),
                right: Box::new(right.clone()),
            }
        }
        Tree::EMPTY => orig.clone(),
    }
}

/// Helper: true if both trees are EMPTY, or they are structurally equal.
fn reference_eq_or_empty<K: Ord + Clone + PartialEq, V: Clone + PartialEq>(
    t1: Option<&Tree<K, V>>,
    t2: Option<&Tree<K, V>>,
) -> bool {
    match (t1, t2) {
        (Some(t1), Some(t2)) => t1 == t2,
        (None, None) => true,
        _ => false,
    }
}

// ============================================================================
// balance (re-export from baseavlset for internal use)
// ============================================================================

/// Balances an AVL tree.
pub fn balance<K: Ord + Clone + PartialEq, V: Clone + PartialEq>(in_tree: &Tree<K, V>) -> Tree<K, V> {
    match in_tree {
        Tree::LEAF { .. } => in_tree.clone(),
        Tree::NODE { .. } => {
            let lh = in_tree.left().unwrap().tree_height();
            let rh = in_tree.right().unwrap().tree_height();
            let diff = lh - rh;

            if diff < -1 {
                // Right heavy - left rotation needed
                if calculate_balance(in_tree.right().unwrap()) > 0 {
                    // Right-Left case: double rotation
                    let rotated_right = rotate_right(in_tree.right().unwrap());
                    let new_tree = set_tree_left_right(
                        in_tree,
                        in_tree.left().unwrap(),
                        &rotated_right,
                    );
                    rotate_left(&new_tree)
                } else {
                    // Right-Right case: single rotation
                    rotate_left(in_tree)
                }
            } else if diff > 1 {
                // Left heavy - right rotation needed
                if calculate_balance(in_tree.left().unwrap()) < 0 {
                    // Left-Right case: double rotation
                    let rotated_left = rotate_left(in_tree.left().unwrap());
                    let new_tree = set_tree_left_right(
                        in_tree,
                        &rotated_left,
                        in_tree.right().unwrap(),
                    );
                    rotate_right(&new_tree)
                } else {
                    // Left-Left case: single rotation
                    rotate_right(in_tree)
                }
            } else if in_tree.tree_height() != std::cmp::max(lh, rh) + 1 {
                // Height needs update
                Tree::NODE {
                    key: in_tree.key().unwrap().clone(),
                    value: in_tree.value().unwrap().clone(),
                    height: std::cmp::max(lh, rh) + 1,
                    left: in_tree.left().unwrap().clone().into(),
                    right: in_tree.right().unwrap().clone().into(),
                }
            } else {
                in_tree.clone()
            }
        }
        Tree::EMPTY => in_tree.clone(),
    }
}

/// Returns the height of a tree.
pub fn height<K: Ord + Clone, V: Clone>(in_node: &Tree<K, V>) -> i32 {
    in_node.tree_height()
}

/// Returns the balance factor: left_height - right_height.
pub fn calculate_balance<K: Ord + Clone, V: Clone>(in_node: &Tree<K, V>) -> i32 {
    match in_node {
        Tree::NODE { left, right, .. } => height(left) - height(right),
        Tree::LEAF { .. } => 0,
        Tree::EMPTY => 0,
    }
}

/// Performs an AVL left rotation on the given tree.
pub fn rotate_left<K: Ord + Clone + PartialEq, V: Clone + PartialEq>(in_node: &Tree<K, V>) -> Tree<K, V> {
    match in_node {
        Tree::NODE {
            left,
            right,
            ..
        } => {
            let child = right.as_ref();
            match child {
                Tree::NODE {
                    left: child_left,
                    right: child_right,
                    ..
                } => {
                    let node = set_tree_left_right(in_node, left, child_left);
                    set_tree_left_right(child, &node, child_right)
                }
                Tree::LEAF { .. } => {
                    let node = set_tree_left_right(in_node, left, &Tree::EMPTY);
                    set_tree_left_right(child, &node, &Tree::EMPTY)
                }
                Tree::EMPTY => in_node.clone(),
            }
        }
        _ => in_node.clone(),
    }
}

/// Performs an AVL right rotation on the given tree.
pub fn rotate_right<K: Ord + Clone + PartialEq, V: Clone + PartialEq>(in_node: &Tree<K, V>) -> Tree<K, V> {
    match in_node {
        Tree::NODE {
            left,
            right,
            ..
        } => {
            let child = left.as_ref();
            match child {
                Tree::NODE {
                    left: child_left,
                    right: child_right,
                    ..
                } => {
                    let node = set_tree_left_right(in_node, child_right, right);
                    set_tree_left_right(child, child_left, &node)
                }
                Tree::LEAF { .. } => {
                    let node = set_tree_left_right(in_node, &Tree::EMPTY, right);
                    set_tree_left_right(child, &Tree::EMPTY, &node)
                }
                Tree::EMPTY => in_node.clone(),
            }
        }
        _ => in_node.clone(),
    }
}
