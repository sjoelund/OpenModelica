//! Translation of Util/AvlSetPath.mo
//!
//! AvlSetPath is a specialization of BaseAvlSet where Key = Absyn.Path.
//! It redeclares keyStr to use AbsynUtil.pathString and keyCompare
//! to use AbsynUtil.pathCompare.
//!
//! Since BaseAvlSet is already generic in Rust, this module re-exports
//! the base module and provides Path-specific helpers.

use crate::baseavlset;
use crate::absyn::Path;
pub use baseavlset::{add, add_list, balance, calculate_balance, height, intersection,
                      is_empty, key_compare, list_keys, list_keys_reverse,
                      new, print_tree_str, print_tree_str2,
                      rotate_left, rotate_right, set_tree_left_right, smallest_key};

/// Type alias for the key type (Absyn.Path in MetaModelica).
pub type Key = Path;

/// Type alias for Tree<Key>.
pub type Tree = baseavlset::Tree<Key>;

/// Empty tree constructor (matches `new` in MetaModelica).
pub fn empty() -> Tree {
    Tree::EMPTY
}

/// keyStr: convert a Path to a String.
/// Matches: `outString := AbsynUtil.pathString(inKey)`
///
/// pathString with default parameters (delimiter=".", usefq=true, reverse=false)
/// formats a Path as a dot-separated string.
/// - IDENT(name) -> name
/// - QUALIFIED(name, path) -> pathString(path) + "." + name
/// - FULLYQUALIFIED(path) -> ".." + pathString(path)
pub fn key_str_fn(in_key: &Path) -> String {
    match in_key {
        Path::IDENT { name } => name.clone(),
        Path::QUALIFIED { name, path } => {
            let parent = key_str_fn(path);
            if parent.is_empty() {
                name.clone()
            } else {
                format!("{}.{}", parent, name)
            }
        }
        Path::FULLYQUALIFIED { path } => {
            let parent = key_str_fn(path);
            format!("..{}", parent)
        }
    }
}

/// keyCompare: compare two Paths.
/// Returns -1 if key1 < key2, 0 if equal, 1 if key1 > key2.
/// Matches: `outResult := AbsynUtil.pathCompare(inKey1, inKey2)`
///
/// This uses string comparison of the path representations.
/// Note: This is a simplified comparison. The actual AbsynUtil.pathCompare
/// may handle quoting and fully-qualified name resolution differently.
pub fn key_compare_fn(in_key1: &Path, in_key2: &Path) -> i32 {
    let s1 = key_str_fn(in_key1);
    let s2 = key_str_fn(in_key2);
    match s1.cmp(&s2) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ident(name: &str) -> Path {
        Path::IDENT { name: name.to_string() }
    }

    fn qualified(name: &str, path: Path) -> Path {
        Path::QUALIFIED { name: name.to_string(), path: Box::new(path) }
    }

    fn fully_qualified(path: Path) -> Path {
        Path::FULLYQUALIFIED { path: Box::new(path) }
    }

    #[test]
    fn test_key_str_fn_ident() {
        assert_eq!(key_str_fn(&ident("Foo")), "Foo");
        assert_eq!(key_str_fn(&ident("")), "");
    }

    #[test]
    fn test_key_str_fn_qualified() {
        assert_eq!(key_str_fn(&qualified("Bar", ident("Foo"))), "Foo.Bar");
        assert_eq!(key_str_fn(&qualified("C", qualified("B", ident("A")))), "A.B.C");
    }

    #[test]
    fn test_key_str_fn_fully_qualified() {
        assert_eq!(key_str_fn(&fully_qualified(ident("Foo"))), "..Foo");
        assert_eq!(key_str_fn(&fully_qualified(qualified("Bar", ident("Foo")))), "..Foo.Bar");
    }

    #[test]
    fn test_key_compare_fn() {
        assert_eq!(key_compare_fn(&ident("A"), &ident("B")), -1);
        assert_eq!(key_compare_fn(&ident("B"), &ident("A")), 1);
        assert_eq!(key_compare_fn(&ident("X"), &ident("X")), 0);
    }

    #[test]
    fn test_basic_operations() {
        let t: Tree = new();
        assert!(is_empty(&t));

        let t = add(t, ident("Foo"));
        assert!(!is_empty(&t));
        assert!(t.has_key(&ident("Foo")));
        assert!(!t.has_key(&ident("Bar")));

        let t = add(t, ident("Bar"));
        let t = add(t, ident("Baz"));
        assert!(t.has_key(&ident("Foo")));
        assert!(t.has_key(&ident("Bar")));
        assert!(t.has_key(&ident("Baz")));
    }

    #[test]
    fn test_list_keys_sorted() {
        let mut t = Tree::new();
        t = add(t, qualified("C", ident("B")));
        t = add(t, ident("A"));
        t = add(t, ident("B"));
        let keys = list_keys(&t);
        let actual: Vec<Path> = keys.into_iter().collect();
        assert_eq!(actual[0], ident("A"));
        assert_eq!(actual[1], ident("B"));
        assert_eq!(actual[2], qualified("C", ident("B")));
    }

    #[test]
    fn test_intersection() {
        let t1 = add(add(ident("a".to_string()), "b".to_string()), "c".to_string());
        let t2 = add(add(ident("b".to_string()), "c".to_string()), "d".to_string());
        let (intersect, rest1, rest2) = intersection(&t1, &t2);
        assert!(intersect.has_key(&ident("b".to_string())));
        assert!(intersect.has_key(&ident("c".to_string())));
        assert!(!intersect.has_key(&ident("a".to_string())));
        assert!(!intersect.has_key(&ident("d".to_string())));
        assert!(rest1.has_key(&ident("a".to_string())));
        assert!(rest2.has_key(&ident("d".to_string())));
    }
}
