//! Translation of Util/AvlSetString.mo
//!
//! AvlSetString is a specialization of BaseAvlSet where Key = String.
//! It redeclares keyStr to output the key directly and keyCompare
//! to use Rust's string comparison.
//!
//! Since BaseAvlSet is already generic in Rust, this module re-exports
//! the base module and provides String-specific helpers.

use crate::baseavlset;
pub use baseavlset::{add, add_list, balance, calculate_balance, height, intersection,
                      is_empty, key_compare, list_keys, list_keys_reverse,
                      new, print_tree_str, print_tree_str2,
                      rotate_left, rotate_right, set_tree_left_right, smallest_key};

/// Type alias for the key type (String in MetaModelica).
pub type Key = String;

/// Type alias for Tree<Key>.
pub type Tree = baseavlset::Tree<Key>;

/// Empty tree constructor (matches `new` in MetaModelica).
pub fn empty() -> Tree {
    Tree::EMPTY
}

/// keyStr: convert a String key to a String.
/// Matches: `outString := inKey`
pub fn key_str_fn(in_key: &str) -> String {
    in_key.to_string()
}

/// keyCompare: compare two String keys.
/// Returns -1 if key1 < key2, 0 if equal, 1 if key1 > key2.
/// Matches: `outResult := stringCompare(inKey1, inKey2)`
pub fn key_compare_fn(in_key1: &str, in_key2: &str) -> i32 {
    match in_key1.cmp(in_key2) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_str_fn() {
        assert_eq!(key_str_fn("hello"), "hello");
        assert_eq!(key_str_fn(""), "");
    }

    #[test]
    fn test_key_compare_fn() {
        assert_eq!(key_compare_fn("a", "b"), -1);
        assert_eq!(key_compare_fn("b", "a"), 1);
        assert_eq!(key_compare_fn("x", "x"), 0);
    }

    #[test]
    fn test_basic_operations() {
        let t: Tree = new();
        assert!(is_empty(&t));

        let t = add(t, "world".to_string());
        assert!(!is_empty(&t));
        assert!(t.has_key(&"world".to_string()));
        assert!(!t.has_key(&"hello".to_string()));

        let t = add(t, "hello".to_string());
        let t = add(t, "foo".to_string());
        assert!(t.has_key(&"hello".to_string()));
        assert!(t.has_key(&"world".to_string()));
        assert!(t.has_key(&"foo".to_string()));
    }

    #[test]
    fn test_list_keys_sorted() {
        let mut t = Tree::new();
        t = add(t, "c".to_string());
        t = add(t, "a".to_string());
        t = add(t, "b".to_string());
        let keys = list_keys(&t);
        let actual: Vec<String> = keys.into_iter().collect();
        assert_eq!(actual, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_intersection() {
        let t1 = add(add(add(new(), "a".to_string()), "b".to_string()), "c".to_string());
        let t2 = add(add(add(new(), "b".to_string()), "c".to_string()), "d".to_string());
        let (intersect, rest1, rest2) = intersection(&t1, &t2);
        assert!(intersect.has_key(&"b".to_string()));
        assert!(intersect.has_key(&"c".to_string()));
        assert!(!intersect.has_key(&"a".to_string()));
        assert!(!intersect.has_key(&"d".to_string()));
        assert!(rest1.has_key(&"a".to_string()));
        assert!(rest2.has_key(&"d".to_string()));
    }
}
