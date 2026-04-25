//! Translation of Util/AvlSetInt.mo
//!
//! AvlSetInt is a specialization of BaseAvlSet where Key = Integer (i32).
//! It redeclares keyStr and keyCompare for i32 keys.
//! Since BaseAvlSet is already generic in Rust, this module re-exports
//! the base module and provides i32-specific helpers.

use crate::baseavlset;
pub use baseavlset::{add, add_list, balance, calculate_balance, height, intersection,
                      is_empty, key_compare, list_keys, list_keys_reverse,
                      new, print_tree_str, print_tree_str2,
                      rotate_left, rotate_right, set_tree_left_right, smallest_key};

/// Type alias for the key type (Integer = i32 in MetaModelica).
pub type Key = i32;

/// Type alias for Tree<Key>.
pub type Tree = baseavlset::Tree<Key>;

/// Empty tree constructor (matches `new` in MetaModelica).
#[allow(dead_code)]
pub fn empty() -> Tree {
    Tree::EMPTY
}

/// keyStr: convert an i32 key to a String.
/// Matches: `outString := String(inKey)`
pub fn key_str_fn(in_key: Key) -> String {
    format!("{in_key}")
}

/// keyCompare: compare two i32 keys.
/// Returns -1 if key1 < key2, 0 if equal, 1 if key1 > key2.
/// Matches: `outResult := sign(inKey2 - inKey1)`
pub fn key_compare_fn(in_key1: Key, in_key2: Key) -> i32 {
    let diff = in_key2 - in_key1;
    if diff > 0 {
        1
    } else if diff < 0 {
        -1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_str_fn() {
        assert_eq!(key_str_fn(42), "42");
        assert_eq!(key_str_fn(-7), "-7");
        assert_eq!(key_str_fn(0), "0");
    }

    #[test]
    fn test_key_compare_fn() {
        assert_eq!(key_compare_fn(1, 2), 1);   // 2-1 > 0 -> 1
        assert_eq!(key_compare_fn(2, 1), -1);  // 1-2 < 0 -> -1
        assert_eq!(key_compare_fn(3, 3), 0);   // 3-3 == 0 -> 0
    }

    #[test]
    fn test_basic_operations() {
        let t: Tree = new();
        assert!(is_empty(&t));

        let t = add(t, 5);
        assert!(!is_empty(&t));
        assert!(t.has_key(&5));
        assert!(!t.has_key(&3));

        let t = add(t, 3);
        let t = add(t, 7);
        assert!(t.has_key(&3));
        assert!(t.has_key(&5));
        assert!(t.has_key(&7));
    }
}
