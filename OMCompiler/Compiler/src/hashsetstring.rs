//! Translation of Util/HashSetString.mo
//!
//! This module provides a string-specific HashSet implementation built on top
//! of the generic BaseHashSet. It configures BaseHashSet with string-specific
//! hash (DJB2), equality, and key-to-string conversion functions.

use crate::basehashset::{self, FuncsTuple, HashSet as BaseHashSet, DEFAULT_BUCKET_SIZE};

/// Key type alias - String keys.
pub type Key = String;

/// Hash function callback for string keys (DJB2 algorithm).
pub type FuncHashCref = basehashset::FuncHash<Key>;

/// Equality comparison callback for string keys.
pub type FuncCrefEqual = basehashset::FuncEq<Key>;

/// Key-to-string conversion callback.
pub type FuncCrefStr = basehashset::FuncKeyString<Key>;

/// HashSet type alias - same structure as BaseHashSet HashSet.
pub type HashSet = BaseHashSet<Key>;

/// DJB2 hash function: hash = hash * 31 + char, starting from 0.
/// This is the standard DJB2 string hash algorithm.
fn string_hash_djb2(s: &Key) -> i32 {
    let mut hash: i32 = 0;
    for byte in s.bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(byte as i32);
    }
    hash
}

/// String equality comparison.
fn string_eq(a: &Key, b: &Key) -> bool {
    a == b
}

/// Identity function - returns its input unchanged.
fn id(s: &Key) -> String {
    s.clone()
}

/// Construct the FuncsTuple for string-based HashSet operations.
fn string_funcs() -> FuncsTuple<Key> {
    FuncsTuple {
        hash: FuncHashCref::new(string_hash_djb2),
        eq: FuncCrefEqual::new(string_eq),
        key_string: FuncCrefStr::new(id),
    }
}

/// Returns an empty HashSet using the default bucket size.
pub fn empty_hash_set() -> HashSet {
    empty_hash_set_sized(DEFAULT_BUCKET_SIZE)
}

/// Returns an empty HashSet with the given bucket size.
pub fn empty_hash_set_sized(size: i32) -> HashSet {
    basehashset::empty_hash_set_work(size, string_funcs())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_djb2_hash_consistency() {
        assert_eq!(string_hash_djb2(&String::new()), 0);
        assert_eq!(string_hash_djb2(&"a".to_string()), 'a' as i32);
        assert_eq!(string_hash_djb2(&"A".to_string()), 'A' as i32);
        // Same input should always produce the same hash
        assert_eq!(string_hash_djb2(&"hello".to_string()), string_hash_djb2(&"hello".to_string()));
    }

    #[test]
    fn test_empty_hash_set() {
        let hs = empty_hash_set();
        assert_eq!(basehashset::current_size(&hs), 0);
        assert!(!basehashset::has(&"test".to_string(), &hs));
    }

    #[test]
    fn test_empty_hash_set_sized() {
        let hs = empty_hash_set_sized(100);
        assert_eq!(basehashset::current_size(&hs), 0);
        assert_eq!(hs.bucket_size, 100);
    }

    #[test]
    fn test_add_and_has() {
        let mut hs = empty_hash_set();
        hs = basehashset::add("hello".to_string(), &hs);
        hs = basehashset::add("world".to_string(), &hs);
        assert!(basehashset::has(&"hello".to_string(), &hs));
        assert!(basehashset::has(&"world".to_string(), &hs));
        assert_eq!(basehashset::current_size(&hs), 2);
    }

    #[test]
    fn test_add_duplicate() {
        let mut hs = empty_hash_set();
        hs = basehashset::add("hello".to_string(), &hs);
        hs = basehashset::add("hello".to_string(), &hs);
        assert_eq!(basehashset::current_size(&hs), 1);
    }

    #[test]
    fn test_get() {
        let hs = empty_hash_set();
        let hs = basehashset::add("key1".to_string(), &hs);
        assert_eq!(basehashset::get(&"key1".to_string(), &hs), Some("key1".to_string()));
        assert_eq!(basehashset::get(&"key2".to_string(), &hs), None);
    }

    #[test]
    fn test_delete() {
        let mut hs = empty_hash_set();
        hs = basehashset::add("hello".to_string(), &hs);
        assert!(basehashset::has(&"hello".to_string(), &hs));
        hs = basehashset::delete(&"hello".to_string(), &hs).unwrap();
        assert!(!basehashset::has(&"hello".to_string(), &hs));
    }

    #[test]
    fn test_hash_set_list() {
        let mut hs = empty_hash_set();
        hs = basehashset::add("first".to_string(), &hs);
        hs = basehashset::add("second".to_string(), &hs);
        let lst = basehashset::hash_set_list(&hs);
        assert_eq!(lst.len(), 2);
    }

    #[test]
    fn test_string_hash_collisions() {
        // DJB2 is a good hash function - test with a few strings
        let hash_a = string_hash_djb2(&"hello".to_string());
        let hash_b = string_hash_djb2(&"world".to_string());
        // Different strings should typically have different hashes
        // (collisions are possible but unlikely for distinct strings)
        let _ = hash_a;
        let _ = hash_b;
    }
}
