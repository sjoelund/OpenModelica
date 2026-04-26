//! Translation of Util/HashTableStringToProgram.mo
//!
//! This module provides a hash table mapping String keys to Absyn::Program values.
//! It is a specific instantiation of the generic BaseHashTable with:
//!   - Key = String
//!   - Value = Absyn::Program
//!
//! The hash function used is djb2, and equality is simple string comparison.
//!
//! # Assumptions
//! - The djb2 hash algorithm is implemented as: h = (h << 5).wrapping_add(h).wrapping_add(c)
//!   where c is the character code point.
//! - Absyn::Program is available from the absyn module.
//! - The FuncValString callback for Program->String conversion uses a dummy
//!   string since Absyn::Program has no natural string representation defined here.
//! - The BaseHashTable infrastructure (from basehashset.rs) is reused, but we add
//!   a FuncValString field since BaseHashTable uses a 4-tuple FuncsTuple.

use im::Vector;

// ============================================================================
// Re-exports from basehashset.rs (shared infrastructure)
// ============================================================================

use crate::basehashset;

// ============================================================================
// Constants
// ============================================================================

/// Default bucket size (same as BaseHashTable.defaultBucketSize).
pub const DEFAULT_BUCKET_SIZE: i32 = 2053;

// ============================================================================
// Type definitions
// ============================================================================

/// Key type - String keys.
pub type Key = String;

/// Value type - Absyn::Program values.
pub type Value = crate::absyn::Program;

/// Hash function callback for string keys (DJB2 algorithm).
pub type FuncHashCref = basehashset::FuncHash<Key>;

/// Equality comparison callback for string keys.
pub type FuncCrefEqual = basehashset::FuncEq<Key>;

/// Key-to-string conversion callback.
pub type FuncCrefStr = basehashset::FuncKeyString<Key>;

/// Type alias for value-to-string conversion callback.
pub type FuncExpStr = basehashset::FuncValString<Value>;

/// Type alias for the 4-field functions tuple (matching BaseHashTable.FuncsTuple).
/// Contains: (hash_func, equal_func, key_string_func, val_string_func)
pub type HashTableFuncs = basehashset::FuncsTuple4<Key, Value>;

/// The HashTable type, matching the MetaModelica tuple:
/// (HashVector, ValueArray, bucket_size, funcs_tuple)
///
/// HashVector = array<list<tuple<String, i32>>>
/// ValueArray = tuple<i32, i32, array<Option<tuple<String, Program>>>>
pub type HashTable = basehashset::BaseHashTable<Key, Value>;

// ============================================================================
// djb2 hash function
// ============================================================================

/// djb2 hash algorithm: h = (h * 33 + c) for each character c.
/// This matches the MetaModelica stringHashDjb2 built-in function.
pub fn string_hash_djb2(s: &str) -> i32 {
    let mut h: i32 = 5381;
    for c in s.chars() {
        // djb2: h = h * 33 + c  ==  h = (h << 5).wrapping_add(h).wrapping_add(c)
        h = h.wrapping_shl(5).wrapping_add(h).wrapping_add(c as i32);
    }
    h
}

// ============================================================================
// Dummy string function for Absyn::Program
// ============================================================================

/// Returns a dummy string for Absyn::Program values.
/// This is the Rust equivalent of the `dummyStr` function in the MO code.
pub fn dummy_str(_p: &Value) -> String {
    "<dummy Absyn::Program>".to_string()
}

// ============================================================================
// emptyHashTable
// ============================================================================

/// Returns an empty HashTable using the default bucket size.
/// Equivalent to the MetaModelica `emptyHashTable` function.
pub fn empty_hash_table() -> HashTable {
    empty_hash_table_sized(DEFAULT_BUCKET_SIZE)
}

// ============================================================================
// emptyHashTableSized
// ============================================================================

/// Returns an empty HashTable using the specified bucket size.
/// Equivalent to the MetaModelica `emptyHashTableSized` function.
///
/// Uses djb2 for hashing, string equality for key comparison,
/// identity for key string conversion (Util.id), and dummy_str for
/// value string conversion.
pub fn empty_hash_table_sized(size: i32) -> HashTable {
    // djb2 hash function
    let hash_func = FuncHashCref::new(move |s: &String| string_hash_djb2(s));

    // String equality function (stringEq)
    let eq_func = FuncCrefEqual::new(|a: &String, b: &String| a == b);

    // Identity function for key string conversion (Util.id)
    let key_string_func = FuncCrefStr::new(|s: &String| s.clone());

    // Dummy function for value string conversion (dummyStr)
    let val_string_func = FuncExpStr::new(|_| dummy_str(&Value::PROGRAM {
        classes: Vector::new(),
        within_: crate::absyn::Within::TOP,
    }));

    let funcs = HashTableFuncs {
        hash: hash_func,
        eq: eq_func,
        key_string: key_string_func,
        val_string: val_string_func,
    };

    // Use the BaseHashTable infrastructure with 4-field funcs tuple
    basehashset::empty_base_hash_table_work(size, funcs)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_hash_djb2() {
        // djb2("") should be 5381 (initial value)
        assert_eq!(string_hash_djb2(""), 5381);
        // Same input should always produce the same hash
        assert_eq!(string_hash_djb2("hello"), string_hash_djb2("hello"));
    }

    #[test]
    fn test_empty_hash_table() {
        let ht = empty_hash_table();
        // current_size = value_arr.0 (first field of value_array)
        assert_eq!(ht.value_arr.0, 0);
    }

    #[test]
    fn test_empty_hash_table_sized() {
        let ht = empty_hash_table_sized(100);
        assert_eq!(ht.value_arr.0, 0);
        assert_eq!(ht.bucket_size, 100);
    }

    #[test]
    fn test_hash_table_has_correct_funcs() {
        let ht = empty_hash_table();
        // Verify the funcs tuple is properly configured
        // The hash function should work
        let hash_a = ht.funcs.hash.call(&"a".to_string());
        let hash_b = ht.funcs.hash.call(&"b".to_string());
        // Both should be valid (not panicking)
        let _ = hash_a;
        let _ = hash_b;
    }
}
