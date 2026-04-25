//! Translation of Util/AvlTree.mo
//!
//! Generic AVL tree with type variables for Key and Val.
//! Supports custom key comparison, string conversion, and duplicate-item
//! update checking via callback functions stored in the Tree struct.
//!
//! MatchContinue from MetaModelica is translated to Rust with
//! `anyhow::Result` and `bail!` for backtracking across alternatives.
//!
//! Datatype mapping:
//!   Integer -> i32
//!   String -> String
//!   Boolean -> bool
//!   polymorphic<Any> -> generics with trait bounds
//!   List<T> -> im::Vector<T>
//!   Option<T> -> std::option::Option<T>

#![allow(dead_code)]

use anyhow::{bail, Result};
use std::fmt::Debug;

// ============================================================================
// Trait definitions for callback functions stored in Tree
// ============================================================================

/// Key comparison: returns -1 (less), 0 (equal), 1 (greater).
pub trait KeyCompare<Key> {
    fn call(&self, in_key1: &Key, in_key2: &Key) -> i32;
}

/// Key-to-string conversion.
pub trait KeyToStr<Key>: Debug {
    fn call(&self, key: &Key) -> String;
}

/// Value-to-string conversion.
pub trait ValToStr<Val>: Debug {
    fn call(&self, val: &Val) -> String;
}

/// Update check: returns true if update is allowed, false to reject.
pub trait ItemUpdateCheck<Key, Val>: Debug {
    fn call(&self, in_item_new: (&Key, &Val), in_item_old: (&Key, &Val)) -> bool;
}

// ============================================================================
// Node uniontype
// ============================================================================

/// The binary tree data structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node<K, V> {
    /// A node containing an item, height, and two children.
    NODE {
        item: Item<K, V>,
        height: i32,
        left: Box<Node<K, V>>,
        right: Box<Node<K, V>>,
    },
    /// Empty/NO_NODE - no node, represents an empty tree.
    NO_NODE,
}

// ============================================================================
// Item uniontype
// ============================================================================

/// Each node in the binary tree can have an item associated with it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item<K, V> {
    ITEM { key: K, val: V },
    NO_ITEM,
}

// ============================================================================
// Tree struct
// ============================================================================

/// An AVL tree with key comparison, printing, and update-check callbacks.
pub struct Tree<K, V> {
    root: Node<K, V>,
    key_compare_func: Box<dyn KeyCompare<K>>,
    key_str_func_opt: Option<Box<dyn KeyToStr<K>>>,
    val_str_func_opt: Option<Box<dyn ValToStr<V>>>,
    update_check_func_opt: Option<Box<dyn ItemUpdateCheck<K, V>>>,
    name: String,
}

// ============================================================================
// Node helper methods
// ============================================================================

impl<K, V> Node<K, V> {
    fn is_no_node(&self) -> bool {
        matches!(self, Node::NO_NODE)
    }

    fn is_empty_leaf(&self) -> bool {
        matches!(self, Node::NODE {
            item: Item::NO_ITEM,
            left,
            right,
            ..
        } if matches!(left.as_ref(), Node::NO_NODE) && matches!(right.as_ref(), Node::NO_NODE)
        )
    }
}

fn is_no_node_boxed(node: &Box<Node<(), ()>>) -> bool {
    matches!(node.as_ref(), Node::NO_NODE)
}

// ============================================================================
// Tree methods
// ============================================================================

impl<K: Clone + Debug, V: Clone + Debug> Tree<K, V> {
    /// Returns the name of the tree.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns true if printing functions (keyStr and valStr) have been set.
    pub fn has_printing_functions(&self) -> bool {
        self.key_str_func_opt.is_some() && self.val_str_func_opt.is_some()
    }

    /// Returns true if an update check function has been set.
    pub fn has_update_check_function(&self) -> bool {
        self.update_check_func_opt.is_some()
    }

    /// Get the root node.
    pub fn root(&self) -> &Node<K, V> {
        &self.root
    }

    /// Get the root node (owned clone).
    pub fn into_root(self) -> Node<K, V> {
        self.root
    }

    /// Returns the key compare function.
    pub fn get_key_compare_func(&self) -> &dyn KeyCompare<K> {
        &*self.key_compare_func
    }

    /// Returns the update check function.
    pub fn get_update_check_func(&self) -> &dyn ItemUpdateCheck<K, V> {
        &**self.update_check_func_opt.as_ref().unwrap()
    }

    /// Returns the key-to-string function.
    pub fn get_key_to_str_func(&self) -> &dyn KeyToStr<K> {
        &**self.key_str_func_opt.as_ref().unwrap()
    }

    /// Returns the val-to-string function.
    pub fn get_val_to_str_func(&self) -> &dyn ValToStr<V> {
        &**self.val_str_func_opt.as_ref().unwrap()
    }
}

/// Creates an empty AVL tree with the given parameters.
pub fn create<K: Clone + Debug, V: Clone + Debug>(
    name: String,
    key_compare_func: Box<dyn KeyCompare<K>>,
    key_str_func_opt: Option<Box<dyn KeyToStr<K>>>,
    val_str_func_opt: Option<Box<dyn ValToStr<V>>>,
    update_check_func_opt: Option<Box<dyn ItemUpdateCheck<K, V>>>,
) -> Tree<K, V> {
    Tree {
        root: Node::NODE {
            item: Item::NO_ITEM,
            height: 0,
            left: Box::new(Node::NO_NODE),
            right: Box::new(Node::NO_NODE),
        },
        key_compare_func,
        key_str_func_opt,
        val_str_func_opt,
        update_check_func_opt,
        name,
    }
}

// ============================================================================
// Protected helper: new_leaf_node
// ============================================================================

fn new_leaf_node<K: Clone, V: Clone>(item: Item<K, V>, height: i32) -> Node<K, V> {
    Node::NODE {
        item,
        height,
        left: Box::new(Node::NO_NODE),
        right: Box::new(Node::NO_NODE),
    }
}

// ============================================================================
// add
// ============================================================================

/// Inserts a new item into the tree.
pub fn add<K: Clone + PartialEq + Debug, V: Clone + Debug>(
    mut in_tree: Tree<K, V>,
    in_key: K,
    in_val: V,
) -> Result<Tree<K, V>> {
    let root_node = in_tree.root.clone();
    let new_root = add_node(&in_tree, root_node, in_key, in_val)?;
    in_tree.root = new_root;
    Ok(in_tree)
}

// ============================================================================
// add_node
// ============================================================================

fn add_node<K: Clone + PartialEq + Debug, V: Clone + Debug>(
    in_tree: &Tree<K, V>,
    in_node: Node<K, V>,
    in_key: K,
    in_val: V,
) -> Result<Node<K, V>> {
    match &in_node {
        // empty node
        Node::NO_NODE => {
            return Ok(new_leaf_node(Item::ITEM { key: in_key, val: in_val }, 1))
        }

        // empty node item
        Node::NODE {
            item: Item::NO_ITEM,
            left,
            right,
            ..
        } if matches!(left.as_ref(), Node::NO_NODE) && matches!(right.as_ref(), Node::NO_NODE) => {
            return Ok(new_leaf_node(Item::ITEM { key: in_key, val: in_val }, 1))
        }

        Node::NODE {
            item: Item::ITEM { key: rkey, .. },
            ..
        } => {
            let order = in_tree.get_key_compare_func().call(&in_key, rkey);
            let node = add_node_dispatch(in_tree, in_node, order, in_key, in_val)?;
            return Ok(balance(node));
        }

        _ => {}
    }
    bail!(
        "AvlTree.add_node name: {} failed!",
        in_tree.name()
    )
}

// ============================================================================
// add_node_dispatch
// ============================================================================

fn add_node_dispatch<K: Clone + PartialEq + Debug, V: Clone + Debug>(
    in_tree: &Tree<K, V>,
    in_node: Node<K, V>,
    in_key_comp: i32,
    in_key: K,
    in_val: V,
) -> Result<Node<K, V>> {
    // case: key comp == 0, no update check function -> allow replacement
    if in_key_comp == 0 && !in_tree.has_update_check_function() {
        if let Node::NODE {
            item: _,
            height,
            left,
            right,
        } = in_node
        {
            return Ok(Node::NODE {
                item: Item::ITEM { key: in_key, val: in_val },
                height,
                left,
                right,
            });
        }
    }

    // case: key comp == 0, has update check function, update allowed
    if in_key_comp == 0 && in_tree.has_update_check_function() {
        if let Node::NODE {
            item,
            height,
            left,
            right,
        } = &in_node
        {
            let update_check = in_tree.get_update_check_func();
            if let Item::ITEM { key: ok, val: ov } = item {
                let new_item_key = in_key.clone();
                let new_item_val = in_val.clone();
                if update_check.call((&new_item_key, &new_item_val), (ok, ov)) {
                    return Ok(Node::NODE {
                        item: Item::ITEM { key: in_key, val: in_val },
                        height: *height,
                        left: left.clone(),
                        right: right.clone(),
                    });
                }
            }
        }
    }

    // case: key comp == 0, has update check function, update NOT allowed -> return same node
    if in_key_comp == 0 && in_tree.has_update_check_function() {
        if let Node::NODE {
            item,
            height,
            left,
            right,
        } = &in_node
        {
            let update_check = in_tree.get_update_check_func();
            if let Item::ITEM { key: ok, val: ov } = item {
                let new_item_key = in_key.clone();
                let new_item_val = in_val.clone();
                if !update_check.call((&new_item_key, &new_item_val), (ok, ov)) {
                    return Ok(Node::NODE {
                        item: item.clone(),
                        height: *height,
                        left: left.clone(),
                        right: right.clone(),
                    });
                }
            }
        }
    }

    // insert into right subtree (key > node key)
    if in_key_comp == 1 {
        if let Node::NODE {
            item,
            height,
            left,
            right,
        } = in_node
        {
            let n = empty_node_if_no_node(*right);
            let n = add_node(in_tree, n, in_key, in_val)?;
            return Ok(Node::NODE { item, height, left, right: Box::new(n) });
        }
    }

    // insert into left subtree (key < node key)
    if in_key_comp == -1 {
        if let Node::NODE {
            item,
            height,
            left,
            right,
        } = in_node
        {
            let n = empty_node_if_no_node(*left);
            let n = add_node(in_tree, n, in_key, in_val)?;
            return Ok(Node::NODE {
                item,
                height,
                left: Box::new(n),
                right,
            });
        }
    }

    bail!("AvlTree.add_node_dispatch failed")
}

// ============================================================================
// get
// ============================================================================

/// Get a value from the tree given a key.
pub fn get<K: Clone + PartialEq + Debug, V: Clone + Debug>(
    in_tree: &Tree<K, V>,
    in_key: &K,
) -> Result<V> {
    let node = &in_tree.root;
    get_node(in_tree, node.clone(), in_key.clone())
}

fn get_node<K: Clone + PartialEq + Debug, V: Clone + Debug>(
    in_tree: &Tree<K, V>,
    in_node: Node<K, V>,
    in_key: K,
) -> Result<V> {
    let rkey = match &in_node {
        Node::NODE {
            item: Item::ITEM { key, .. },
            ..
        } => key.clone(),
        _ => bail!("AvlTree.get_node: not a NODE with ITEM"),
    };
    let order = in_tree.get_key_compare_func().call(&in_key, &rkey);
    get_node_dispatch(in_tree, in_node, order, in_key)
}

fn get_node_dispatch<K: Clone + PartialEq + Debug, V: Clone + Debug>(
    in_tree: &Tree<K, V>,
    in_node: Node<K, V>,
    in_key_comp: i32,
    in_key: K,
) -> Result<V> {
    match (&in_node, in_key_comp) {
        // found match
        (Node::NODE { item: Item::ITEM { val, .. }, .. }, 0) => Ok(val.clone()),

        // search right
        (Node::NODE { right, .. }, 1) => {
            get_node(in_tree, *right.clone(), in_key)
        }

        // search left
        (Node::NODE { left, .. }, -1) => {
            get_node(in_tree, *left.clone(), in_key)
        }

        _ => bail!("AvlTree.get_node_dispatch: key not found"),
    }
}

// ============================================================================
// replace
// ============================================================================

/// Replaces the item of an already existing node in the tree.
pub fn replace<K: Clone + PartialEq + Debug, V: Clone + Debug>(
    mut in_tree: Tree<K, V>,
    in_key: K,
    in_val: V,
) -> Result<Tree<K, V>> {
    let node = in_tree.root.clone();
    let new_root = replace_node(&in_tree, node, in_key, in_val)?;
    in_tree.root = new_root;
    Ok(in_tree)
}

fn replace_node<K: Clone + PartialEq + Debug, V: Clone + Debug>(
    in_tree: &Tree<K, V>,
    in_node: Node<K, V>,
    in_key: K,
    in_val: V,
) -> Result<Node<K, V>> {
    let rkey = match &in_node {
        Node::NODE {
            item: Item::ITEM { key, .. },
            ..
        } => key.clone(),
        _ => bail!("AvlTree.replace_node: not a NODE with ITEM"),
    };
    let order = in_tree.get_key_compare_func().call(&in_key, &rkey);
    replace_node_dispatch(in_tree, in_node, order, in_key, in_val)
}

fn replace_node_dispatch<K: Clone + PartialEq + Debug, V: Clone + Debug>(
    in_tree: &Tree<K, V>,
    in_node: Node<K, V>,
    in_key_comp: i32,
    in_key: K,
    in_val: V,
) -> Result<Node<K, V>> {
    match (&in_node, in_key_comp) {
        // replace this node
        (Node::NODE { item, height, left, right }, 0) => {
            if matches!(item, Item::ITEM { .. }) {
                return Ok(Node::NODE {
                    item: Item::ITEM { key: in_key, val: in_val },
                    height: *height,
                    left: left.clone(),
                    right: right.clone(),
                });
            }
        }

        // insert into right subtree
        (Node::NODE {
            item,
            height,
            left,
            right,
        }, 1) => {
            let n = empty_node_if_no_node(*right.clone());
            let n = replace_node(in_tree, n, in_key, in_val)?;
            return Ok(Node::NODE {
                item: item.clone(),
                height: *height,
                left: left.clone(),
                right: Box::new(n),
            });
        }

        // insert into left subtree
        (Node::NODE {
            item,
            height,
            left,
            right,
        }, -1) => {
            let n = empty_node_if_no_node(*left.clone());
            let n = replace_node(in_tree, n, in_key, in_val)?;
            return Ok(Node::NODE {
                item: item.clone(),
                height: *height,
                left: Box::new(n),
                right: right.clone(),
            });
        }

        _ => {}
    }
    bail!("AvlTree.replace_node_dispatch failed")
}

// ============================================================================
// empty_node_if_no_node
// ============================================================================

fn empty_node_if_no_node<K: Clone, V: Clone>(in_node: Node<K, V>) -> Node<K, V> {
    match in_node {
        Node::NO_NODE => Node::NODE {
            item: Item::NO_ITEM,
            height: 0,
            left: Box::new(Node::NO_NODE),
            right: Box::new(Node::NO_NODE),
        },
        n @ Node::NODE { .. } => n,
    }
}

// ============================================================================
// balance / do_balance / do_balance2 / do_balance3 / do_balance4
// ============================================================================

fn balance<K: Clone + PartialEq + Debug, V: Clone + Debug>(in_node: Node<K, V>) -> Node<K, V> {
    if in_node.is_no_node() {
        return in_node;
    }
    let d = difference_in_height(&in_node);
    do_balance(d, in_node)
}

fn do_balance<K: Clone + PartialEq + Debug, V: Clone + Debug>(
    difference: i32,
    in_node: Node<K, V>,
) -> Node<K, V> {
    match difference {
        -1 | 0 | 1 => compute_height(in_node),
        _ => do_balance2(difference < 0, in_node),
    }
}

fn do_balance2<K: Clone + PartialEq + Debug, V: Clone + Debug>(
    in_diff_is_negative: bool,
    in_node: Node<K, V>,
) -> Node<K, V> {
    if in_diff_is_negative {
        // negative balance factor: left heavy, right rotation
        let n = do_balance3(in_node);
        rotate_left(&n)
    } else {
        // positive balance factor: right heavy, left rotation
        let n = do_balance4(in_node);
        rotate_right(&n)
    }
}

fn do_balance3<K: Clone + PartialEq + Debug, V: Clone + Debug>(
    mut in_node: Node<K, V>,
) -> Node<K, V> {
    let r_n = right_node(&in_node);
    if difference_in_height(&r_n) > 0 {
        let rr = rotate_right(&r_n);
        in_node = set_right(in_node, rr);
    }
    in_node
}

fn do_balance4<K: Clone + PartialEq + Debug, V: Clone + Debug>(
    mut in_node: Node<K, V>,
) -> Node<K, V> {
    let l_n = left_node(&in_node);
    if difference_in_height(&l_n) < 0 {
        let rl = rotate_left(&l_n);
        in_node = set_left(in_node, rl);
    }
    in_node
}

// ============================================================================
// set_right / set_left
// ============================================================================

fn set_right<K: Clone, V: Clone>(node: Node<K, V>, right: Node<K, V>) -> Node<K, V> {
    let Node::NODE { item, height, left, .. } = node else {
        return node;
    };
    Node::NODE {
        item,
        height,
        left,
        right: Box::new(right),
    }
}

fn set_left<K: Clone, V: Clone>(node: Node<K, V>, left: Node<K, V>) -> Node<K, V> {
    let Node::NODE { item, height, right, .. } = node else {
        return node;
    };
    Node::NODE {
        item,
        height,
        left: Box::new(left),
        right,
    }
}

// ============================================================================
// left_node / right_node
// ============================================================================

fn left_node<K: Clone, V: Clone>(node: &Node<K, V>) -> Node<K, V> {
    match node {
        Node::NODE { left, .. } => left.as_ref().clone(),
        _ => Node::NO_NODE,
    }
}

fn right_node<K: Clone, V: Clone>(node: &Node<K, V>) -> Node<K, V> {
    match node {
        Node::NODE { right, .. } => right.as_ref().clone(),
        _ => Node::NO_NODE,
    }
}

// ============================================================================
// exchange_left / exchange_right
// ============================================================================

fn exchange_left<K: Clone + PartialEq + Debug, V: Clone + Debug>(
    in_node: Node<K, V>,
    in_parent: Node<K, V>,
) -> Node<K, V> {
    let parent = set_right(in_parent, left_node(&in_node));
    let parent = balance(parent);
    let node = set_left(in_node, parent);
    balance(node)
}

fn exchange_right<K: Clone + PartialEq + Debug, V: Clone + Debug>(
    in_node: Node<K, V>,
    in_parent: Node<K, V>,
) -> Node<K, V> {
    let parent = set_left(in_parent, right_node(&in_node));
    let parent = balance(parent);
    let node = set_right(in_node, parent);
    balance(node)
}

// ============================================================================
// rotate_left / rotate_right
// ============================================================================

fn rotate_left<K: Clone + PartialEq + Debug, V: Clone + Debug>(node: &Node<K, V>) -> Node<K, V> {
    exchange_left(right_node(node), node.clone())
}

fn rotate_right<K: Clone + PartialEq + Debug, V: Clone + Debug>(node: &Node<K, V>) -> Node<K, V> {
    exchange_right(left_node(node), node.clone())
}

// ============================================================================
// difference_in_height / compute_height / get_height
// ============================================================================

fn difference_in_height<K: Clone, V: Clone>(node: &Node<K, V>) -> i32 {
    let (l, r) = match node {
        Node::NODE { left, right, .. } => (left.as_ref().clone(), right.as_ref().clone()),
        _ => (Node::NO_NODE, Node::NO_NODE),
    };
    get_height(&l) - get_height(&r)
}

fn compute_height<K: Clone, V: Clone>(in_node: Node<K, V>) -> Node<K, V> {
    let Node::NODE { item, left, right, .. } = in_node else {
        return in_node;
    };
    let hl = get_height(&left);
    let hr = get_height(&right);
    let height = hl.max(hr) + 1;
    Node::NODE {
        item,
        height,
        left,
        right,
    }
}

fn get_height<K, V>(bt: &Node<K, V>) -> i32 {
    match bt {
        Node::NO_NODE => 0,
        Node::NODE { height, .. } => *height,
    }
}

// ============================================================================
// pretty_print_tree_str
// ============================================================================

/// Returns a string representation of the tree, or an error message if
/// no printing functions are attached.
pub fn pretty_print_tree_str<K: Clone + PartialEq + Debug, V: Clone + Debug>(
    in_tree: &Tree<K, V>,
) -> String {
    if !in_tree.has_printing_functions() {
        return format!(
            "TreePrintError<NO_PRINTING_FUNCTIONS_ATTACHED> name[{}]",
            in_tree.name()
        );
    }
    let node = &in_tree.root;
    if node.is_no_node() {
        return String::new();
    }
    pretty_print_node_str(in_tree, node.clone(), "")
}

fn pretty_print_node_str<K: Clone + PartialEq + Debug, V: Clone + Debug>(
    in_tree: &Tree<K, V>,
    in_node: Node<K, V>,
    in_indent: &str,
) -> String {
    match in_node {
        Node::NO_NODE => String::new(),

        Node::NODE {
            item: Item::NO_ITEM,
            left,
            right,
            ..
        } => {
            let indent = format!("{}  ", in_indent);
            let s1 = pretty_print_node_str(in_tree, *left, &indent);
            let s2 = pretty_print_node_str(in_tree, *right, &indent);
            format!("\n{}{}", s1, s2)
        }

        Node::NODE {
            item,
            left,
            right,
            ..
        } => {
            let indent = format!("{}  ", in_indent);
            let s1 = pretty_print_node_str(in_tree, *left, &indent);
            let s2 = pretty_print_node_str(in_tree, *right, &indent);
            let item_str = print_item_str(in_tree, item);
            format!("\n{}{}{}{}", in_indent, item_str, s1, s2)
        }
    }
}

// ============================================================================
// print_tree_str
// ============================================================================

/// Returns a compact string representation of the tree.
pub fn print_tree_str<K: Clone + PartialEq + Debug, V: Clone + Debug>(
    in_tree: &Tree<K, V>,
) -> String {
    if !in_tree.has_printing_functions() {
        return format!(
            "TreePrintError<NO_PRINTING_FUNCTIONS_ATTACHED> name[{}]",
            in_tree.name()
        );
    }
    let node = &in_tree.root;
    if node.is_no_node() {
        return String::new();
    }
    print_node_str(in_tree, node.clone())
}

fn print_node_str<K: Clone + PartialEq + Debug, V: Clone + Debug>(
    in_tree: &Tree<K, V>,
    in_node: Node<K, V>,
) -> String {
    match in_node {
        Node::NO_NODE => String::new(),
        Node::NODE {
            item: Item::NO_ITEM,
            ..
        } => String::new(),
        Node::NODE {
            item,
            left,
            right,
            ..
        } => {
            let left_str = print_node_str(in_tree, *left);
            let right_str = print_node_str(in_tree, *right);
            let item_str = print_item_str(in_tree, item);
            format!("i: {}, l: {}, r: {}", item_str, left_str, right_str)
        }
    }
}

// ============================================================================
// print_item_str
// ============================================================================

fn print_item_str<K: Clone + PartialEq + Debug, V: Clone + Debug>(
    in_tree: &Tree<K, V>,
    in_item: Item<K, V>,
) -> String {
    match in_item {
        Item::NO_ITEM => "[]".to_string(),
        Item::ITEM { key, val } => {
            let key_str = in_tree.get_key_to_str_func().call(&key);
            let val_str = in_tree.get_val_to_str_func().call(&val);
            format!("[{}, {}]", key_str, val_str)
        }
    }
}

// ============================================================================
// get_key_of_val
// ============================================================================

/// Search for a key that has val as value. Returns the first matching key
/// found via DFS (current, left, right).
pub fn get_key_of_val<K: Clone + PartialEq + Debug, V: Clone + Debug + PartialEq>(
    in_tree: &Tree<K, V>,
    in_val: &V,
) -> Result<K> {
    let node = &in_tree.root;
    if node.is_no_node() {
        bail!("AvlTree.get_key_of_val: tree has no root");
    }
    get_key_of_val_node(in_tree, node.clone(), in_val)
}

fn get_key_of_val_node<K: Clone + PartialEq + Debug, V: Clone + Debug + PartialEq>(
    in_tree: &Tree<K, V>,
    in_node: Node<K, V>,
    in_val: &V,
) -> Result<K> {
    // Try current node
    if let Node::NODE {
        item: Item::ITEM { key: k, val: v },
        ..
    } = &in_node
    {
        if v == in_val {
            return Ok(k.clone());
        }
    }

    // Search left
    if let Node::NODE {
        item: Item::ITEM { val: v, .. },
        left,
        ..
    } = &in_node
    {
        if v != in_val {
            if let Ok(k) = get_key_of_val_node(in_tree, left.as_ref().clone(), in_val) {
                return Ok(k);
            }
        }
    }

    // Search right
    if let Node::NODE {
        item: Item::ITEM { val: v, .. },
        right,
        ..
    } = &in_node
    {
        if v != in_val {
            if let Ok(k) = get_key_of_val_node(in_tree, right.as_ref().clone(), in_val) {
                return Ok(k);
            }
        }
    }

    bail!("Key not found with value")
}

// ============================================================================
// add_unique
// ============================================================================

/// Inserts a new item into the tree if the key is not already present.
/// Returns the tree and the item that was inserted or already existed.
pub fn add_unique<K: Clone + PartialEq + Debug, V: Clone + Debug>(
    mut in_tree: Tree<K, V>,
    in_key: K,
    in_val: V,
) -> Result<(Tree<K, V>, Item<K, V>)> {
    let root_node = in_tree.root.clone();
    let (node, item) = add_node_unique(&in_tree, root_node, in_key, in_val)?;
    in_tree.root = node;
    Ok((in_tree, item))
}

fn add_node_unique<K: Clone + PartialEq + Debug, V: Clone + Debug>(
    in_tree: &Tree<K, V>,
    in_node: Node<K, V>,
    in_key: K,
    in_val: V,
) -> Result<(Node<K, V>, Item<K, V>)> {
    match &in_node {
        // empty node
        Node::NO_NODE => {
            let item = Item::ITEM { key: in_key, val: in_val };
            let n = new_leaf_node(item.clone(), 1);
            return Ok((n, item));
        }

        // empty node item
        Node::NODE {
            item: Item::NO_ITEM,
            left,
            right,
            ..
        } if matches!(left.as_ref(), Node::NO_NODE) && matches!(right.as_ref(), Node::NO_NODE) => {
            let item = Item::ITEM { key: in_key, val: in_val };
            let n = new_leaf_node(item.clone(), 1);
            return Ok((n, item));
        }

        Node::NODE {
            item: Item::ITEM { key: rkey, .. },
            ..
        } => {
            let order = in_tree.get_key_compare_func().call(&in_key, rkey);
            let (n, item) = add_node_unique_dispatch(in_tree, in_node, order, in_key, in_val)?;
            let n = balance(n);
            return Ok((n, item));
        }

        _ => {}
    }
    bail!("AvlTree.add_node_unique name: {} failed!", in_tree.name())
}

fn add_node_unique_dispatch<K: Clone + PartialEq + Debug, V: Clone + Debug>(
    in_tree: &Tree<K, V>,
    in_node: Node<K, V>,
    in_key_comp: i32,
    in_key: K,
    in_val: V,
) -> Result<(Node<K, V>, Item<K, V>)> {
    match in_key_comp {
        // key already exists - return the existing node and item
        0 => {
            let existing = match &in_node {
                Node::NODE { item, .. } => item.clone(),
                _ => bail!("AvlTree.add_node_unique_dispatch: not a NODE with ITEM"),
            };
            return Ok((in_node, existing));
        }

        // insert into right subtree
        1 => {
            if let Node::NODE {
                item,
                height,
                left,
                right,
            } = in_node
            {
                let mut n = empty_node_if_no_node(*right);
                let (new_n, it) = add_node_unique(in_tree, n, in_key, in_val)?;
                n = new_n;
                return Ok((
                    Node::NODE {
                        item: item.clone(),
                        height,
                        left,
                        right: Box::new(n),
                    },
                    it,
                ));
            }
        }

        // insert into left subtree
        -1 => {
            if let Node::NODE {
                item,
                height,
                left,
                right,
            } = in_node
            {
                let mut n = empty_node_if_no_node(*left);
                let (new_n, it) = add_node_unique(in_tree, n, in_key, in_val)?;
                n = new_n;
                return Ok((
                    Node::NODE {
                        item: item.clone(),
                        height,
                        left: Box::new(n),
                        right,
                    },
                    it,
                ));
            }
        }

        _ => {}
    }
    bail!("AvlTree.add_node_unique_dispatch failed")
}
