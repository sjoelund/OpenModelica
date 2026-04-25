//! Translation of Util/AvlTreeStringString.mo
//!
//! AvlTreeStringString is a specialization of BaseAvlTree where both Key and
//! Value are String. It redeclares keyStr, valueStr, and keyCompare
//! for string keys and string values.
//! Since BaseAvlTree is already generic in Rust, this module re-exports
//! the base module and provides String/String-specific helpers.

use crate::baseavltree;

// Re-export all the generic tree operations for String/String
pub use baseavltree::{
    add, add_conflict_fail, add_conflict_keep, add_conflict_replace, add_list, add_update,
    balance, calculate_balance, fold, fold_2, fold_cond, for_each, from_list, get, get_opt, height,
    join, key_compare, list_keys, list_values, map, map_fold, print_node_str, rotate_left,
    rotate_right, set_tree_left_right, to_list, update,
};

/// Type alias for the key type (String in MetaModelica).
pub type Key = String;

/// Type alias for the value type (String in MetaModelica).
pub type Value = String;

/// Type alias for Tree<Key, Value>.
pub type Tree = baseavltree::Tree<Key, Value>;

/// Empty tree constructor (matches `new` in MetaModelica).
pub fn new() -> Tree {
    Tree::new()
}

/// keyStr: convert a String key to a String (identity).
/// Matches: `outString := inKey`
pub fn key_str_fn(in_key: &str) -> String {
    in_key.to_string()
}

/// valueStr: convert a String value to a String (identity).
/// Matches: `outString := inValue`
pub fn value_str_fn(in_value: &str) -> String {
    in_value.to_string()
}

/// keyCompare: compare two string keys.
/// Returns -1 if key1 < key2, 0 if equal, 1 if key1 > key2.
/// Matches: `outResult := stringCompare(inKey1, inKey2)`
pub fn key_compare_fn(in_key1: &str, in_key2: &str) -> i32 {
    if in_key1 < in_key2 {
        -1
    } else if in_key1 > in_key2 {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_str_fn() {
        assert_eq!(key_str_fn("hello"), "hello");
        assert_eq!(key_str_fn(""), "");
        assert_eq!(key_str_fn("a"), "a");
    }

    #[test]
    fn test_value_str_fn() {
        assert_eq!(value_str_fn("hello"), "hello");
        assert_eq!(value_str_fn(""), "");
        assert_eq!(value_str_fn("world"), "world");
    }

    #[test]
    fn test_key_compare_fn() {
        assert_eq!(key_compare_fn("a", "b"), -1);
        assert_eq!(key_compare_fn("b", "a"), 1);
        assert_eq!(key_compare_fn("x", "x"), 0);
    }

    #[test]
    fn test_basic_operations() {
        let t = Tree::new();
        assert!(t.is_empty());

        let t = add(t, "key1".to_string(), "val1".to_string(), add_conflict_replace).unwrap();
        assert!(!t.is_empty());

        let t = add(t, "key2".to_string(), "val2".to_string(), add_conflict_replace).unwrap();
        let t = add(t, "key1".to_string(), "val1_updated".to_string(), add_conflict_replace).unwrap();

        // Updated value
        assert_eq!(
            get(&t, &"key1".to_string()).unwrap(),
            "val1_updated"
        );
        assert_eq!(get(&t, &"key2".to_string()).unwrap(), "val2");

        // getOpt for existing keys
        assert_eq!(
            get_opt(&t, &"key1".to_string()),
            Some("val1_updated".to_string())
        );
        assert_eq!(get_opt(&t, &"key3".to_string()), None);

        // toList
        let list = to_list(&t);
        assert_eq!(list.len(), 2);

        // listKeys
        let keys = list_keys(&t);
        assert_eq!(keys.len(), 2);
    }
}
