//! Translation of Util/BaseHashSet.mo
//!
//! This module provides a generic hashset implementation.
//! See BaseAvlSet.rs for the usage pattern.
//!
//! The original MetaModelica code uses `replaceable type Key subtypeof Any`
//! which maps to Rust generics. The hash/equal/key-string functions are
//! passed as callbacks since Rust cannot infer them from the generic type.

use anyhow::{bail, Result};
use im::Vector;
use std::fmt::Debug;
use std::sync::Arc;

/// Persistent list type (maps from MetaModelica list<T>).
type List<T> = Vector<T>;

// ============================================================================
// Constants
// ============================================================================

/// Bucket size constants - pick based on expected set size.
/// These are prime numbers to minimize hash collisions.
pub const LOW_BUCKET_SIZE: i32 = 257;
pub const AVG_BUCKET_SIZE: i32 = 2053;
pub const BIG_BUCKET_SIZE: i32 = 4013;
pub const BIGGER_BUCKET_SIZE: i32 = 25343;
pub const HUGE_BUCKET_SIZE: i32 = 536870879; // 2^29 - 33 is prime :)
pub const DEFAULT_BUCKET_SIZE: i32 = AVG_BUCKET_SIZE;

// ============================================================================
// Type definitions
// ============================================================================

/// A boxed, Clone-able function type for computing a hash from a key.
pub struct FuncHash<K: Clone + Debug + Send + Sync + 'static> {
    inner: Arc<dyn Fn(&K) -> i32 + Send + Sync>,
}

impl<K: Clone + Debug + Send + Sync + 'static> Clone for FuncHash<K> {
    fn clone(&self) -> Self {
        FuncHash {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<K: Clone + Debug + Send + Sync + 'static> FuncHash<K> {
    pub fn new(f: impl Fn(&K) -> i32 + Send + Sync + 'static) -> Self {
        FuncHash {
            inner: Arc::new(f),
        }
    }
    pub fn call(&self, k: &K) -> i32 {
        (self.inner)(k)
    }
}

/// A boxed, Clone-able function type for comparing two keys for equality.
pub struct FuncEq<K: Clone + Debug + Send + Sync + 'static> {
    inner: Arc<dyn Fn(&K, &K) -> bool + Send + Sync>,
}

impl<K: Clone + Debug + Send + Sync + 'static> Clone for FuncEq<K> {
    fn clone(&self) -> Self {
        FuncEq {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<K: Clone + Debug + Send + Sync + 'static> FuncEq<K> {
    pub fn new(f: impl Fn(&K, &K) -> bool + Send + Sync + 'static) -> Self {
        FuncEq {
            inner: Arc::new(f),
        }
    }
    pub fn call(&self, a: &K, b: &K) -> bool {
        (self.inner)(a, b)
    }
}

/// A boxed, Clone-able function type for converting a key to a string.
pub struct FuncKeyString<K: Clone + Debug + Send + Sync + 'static> {
    inner: Arc<dyn Fn(&K) -> String + Send + Sync>,
}

impl<K: Clone + Debug + Send + Sync + 'static> Clone for FuncKeyString<K> {
    fn clone(&self) -> Self {
        FuncKeyString {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<K: Clone + Debug + Send + Sync + 'static> FuncKeyString<K> {
    pub fn new(f: impl Fn(&K) -> String + Send + Sync + 'static) -> Self {
        FuncKeyString {
            inner: Arc::new(f),
        }
    }
    pub fn call(&self, k: &K) -> String {
        (self.inner)(k)
    }
}

/// The functions tuple containing hash, equal, and key-string converters.
pub struct FuncsTuple<K: Clone + Debug + Send + Sync + 'static> {
    pub hash: FuncHash<K>,
    pub eq: FuncEq<K>,
    pub key_string: FuncKeyString<K>,
}

impl<K: Clone + Debug + Send + Sync + 'static> Clone for FuncsTuple<K> {
    fn clone(&self) -> Self {
        FuncsTuple {
            hash: self.hash.clone(),
            eq: self.eq.clone(),
            key_string: self.key_string.clone(),
        }
    }
}

// ============================================================================
// BaseHashTable 4-tuple support (FuncValString + FuncsTuple4 + BaseHashTable)
// ============================================================================

/// A boxed, Clone-able function type for converting a Value to a String.
/// Used by BaseHashTable's 4-field FuncsTuple.
/// Note: V does not require Send + Sync since Value types like Program may not be Send.
pub struct FuncValString<V> {
    inner: Arc<dyn Fn(&V) -> String + 'static>,
}

impl<V: Clone + Debug + 'static> Clone for FuncValString<V> {
    fn clone(&self) -> Self {
        FuncValString {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<V: Clone + Debug + 'static> FuncValString<V> {
    pub fn new(f: impl Fn(&V) -> String + 'static) -> Self {
        FuncValString {
            inner: Arc::new(f),
        }
    }
    pub fn call(&self, val: &V) -> String {
        (self.inner)(val)
    }
}

/// The 4-field functions tuple for BaseHashTable: (hash, eq, key_string, val_string).
/// K requires Send + Sync for the hash/equal functions used in concurrent contexts.
/// V does not require Send + Sync since Value types like Program may not be Send.
pub struct FuncsTuple4<K: Clone + Debug + Send + Sync + 'static, V: Clone + Debug + 'static> {
    pub hash: FuncHash<K>,
    pub eq: FuncEq<K>,
    pub key_string: FuncKeyString<K>,
    pub val_string: FuncValString<V>,
}

impl<K: Clone + Debug + Send + Sync + 'static, V: Clone + Debug + 'static> Clone
    for FuncsTuple4<K, V>
{
    fn clone(&self) -> Self {
        FuncsTuple4 {
            hash: self.hash.clone(),
            eq: self.eq.clone(),
            key_string: self.key_string.clone(),
            val_string: self.val_string.clone(),
        }
    }
}

/// The BaseHashTable struct (4-field funcs tuple, matching BaseHashTable.mo).
pub struct BaseHashTable<K: Clone + Debug + Send + Sync + 'static, V: Clone + Debug + 'static> {
    pub hash_vec: HashVector<K>,
    pub value_arr: ValueArray<(K, V)>,
    pub bucket_size: i32,
    pub n: i32,
    pub funcs: FuncsTuple4<K, V>,
}

impl<K: Clone + Debug + Send + Sync + 'static, V: Clone + Debug + 'static> Clone
    for BaseHashTable<K, V>
{
    fn clone(&self) -> Self {
        BaseHashTable {
            hash_vec: self.hash_vec.clone(),
            value_arr: (
                self.value_arr.0,
                self.value_arr.1,
                self.value_arr.2.to_vec(),
            ),
            bucket_size: self.bucket_size,
            n: self.n,
            funcs: self.funcs.clone(),
        }
    }
}

/// Create an empty BaseHashTable with the given bucket size and 4-field funcs tuple.
pub fn empty_base_hash_table_work<K: Clone + Debug + Send + Sync + 'static, V: Clone + Debug + 'static>(
    sz_bucket: i32,
    funcs: FuncsTuple4<K, V>,
) -> BaseHashTable<K, V> {
    let arr: HashVector<K> = (0..sz_bucket).map(|_| List::new()).collect();
    let sz_arr = bucket_to_values_size(sz_bucket);
    let empty_arr: Vec<Option<(K, V)>> = vec![None; sz_arr as usize];
    let varr = (0, sz_arr, empty_arr);
    BaseHashTable {
        hash_vec: arr,
        value_arr: varr,
        bucket_size: sz_bucket,
        n: 0,
        funcs,
    }
}

/// Value array: (count, capacity, entries).
/// entries[i] is Some(key) if the slot is occupied, None otherwise.
/// Indexes are 0-based (matching Rust convention).
pub type ValueArray<K> = (i32, i32, Vec<Option<K>>);

/// A single bucket entry: (key, position_in_value_array).
type BucketEntry<K> = (K, i32);

/// Hash vector: array of linked lists of (key, index) pairs.
/// Each bucket stores all keys that hash to the same index.
pub type HashVector<K> = Vec<List<BucketEntry<K>>>;

/// The HashSet tuple:
/// (hash_vector, value_array, bucket_size, element_count, funcs).
pub struct HashSet<K: Clone + Debug + Send + Sync + 'static> {
    pub hash_vec: HashVector<K>,
    pub value_arr: ValueArray<K>,
    pub bucket_size: i32,
    pub n: i32,
    pub funcs: FuncsTuple<K>,
}

// ============================================================================
// HashSet Clone implementation
// ============================================================================

impl<K: Clone + Debug + Send + Sync + 'static> Clone for HashSet<K> {
    fn clone(&self) -> Self {
        HashSet {
            hash_vec: self.hash_vec.clone(),
            value_arr: (
                self.value_arr.0,
                self.value_arr.1,
                self.value_arr.2.to_vec(),
            ),
            bucket_size: self.bucket_size,
            n: self.n,
            funcs: self.funcs.clone(),
        }
    }
}

// ============================================================================
// bucketToValuesSize
// ============================================================================

/// Calculate the values array size based on the bucket size.
/// Uses 60% of bucket size (realInt(realMul(intReal(szBucket), 0.6))).
pub fn bucket_to_values_size(sz_bucket: i32) -> i32 {
    (sz_bucket as f64 * 0.6) as i32
}

// ============================================================================
// emptyHashSetWork
// ============================================================================

/// Create an empty hashset with the given bucket size.
pub fn empty_hash_set_work<K: Clone + Debug + Send + Sync + 'static>(
    sz_bucket: i32,
    funcs: FuncsTuple<K>,
) -> HashSet<K> {
    let arr: HashVector<K> = (0..sz_bucket).map(|_| List::new()).collect();
    let sz_arr = bucket_to_values_size(sz_bucket);
    let empty_arr: Vec<Option<K>> = vec![None; sz_arr as usize];
    let varr = (0, sz_arr, empty_arr);
    HashSet {
        hash_vec: arr,
        value_arr: varr,
        bucket_size: sz_bucket,
        n: 0,
        funcs,
    }
}

// ============================================================================
// Helper: get1 (protected)
// ============================================================================

/// Helper function to look up a key. Returns (Option<Key>, index) where
/// index is the position in value_array if found.
pub fn get1<K: Clone + Debug + Send + Sync + 'static>(
    key: &K,
    hash_set: &HashSet<K>,
) -> (Option<K>, i32) {
    let hash_ind = hash_set.funcs.hash.call(key) % hash_set.bucket_size;
    let hash_ind = hash_ind.rem_euclid(hash_set.bucket_size);
    let indexes = &hash_set.hash_vec[hash_ind as usize];
    let key_eq = &hash_set.funcs.eq;

    // Search the linked list for a matching key
    let mut result_index: i32 = -1;
    for (k, idx) in indexes.iter() {
        if key_eq.call(key, k) {
            result_index = *idx;
            break;
        }
    }

    let value = if result_index >= 0 {
        let (_, _, arr) = &hash_set.value_arr;
        arr[result_index as usize].clone()
    } else {
        None
    };

    (value, result_index)
}

// ============================================================================
// Helper: get2 (protected)
// ============================================================================

/// Helper function to search a linked list of keys.
/// Returns (index, found).
pub fn get2<K, F>(key: &K, key_indices: &[(K, i32)], key_eq: &F) -> (i32, bool)
where
    K: Clone,
    F: Fn(&K, &K) -> bool,
{
    for (k, idx) in key_indices.iter() {
        if key_eq(key, k) {
            return (*idx, true);
        }
    }
    (-1, false)
}

// ============================================================================
// add
// ============================================================================

/// Add a key to the hashset. If the key already exists, nothing happens
/// (the entry is kept up to date).
pub fn add<K: Clone + Debug + Send + Sync + 'static>(
    entry: K,
    hash_set: &HashSet<K>,
) -> HashSet<K> {
    let (fkey, indx) = get1(&entry, hash_set);

    if fkey.is_some() {
        // Key already exists - update value in value_array
        let mut new_arr = hash_set.value_arr.2.to_vec();
        new_arr[indx as usize] = Some(entry);
        HashSet {
            hash_vec: hash_set.hash_vec.clone(),
            value_arr: (
                hash_set.value_arr.0,
                hash_set.value_arr.1,
                new_arr,
            ),
            bucket_size: hash_set.bucket_size,
            n: hash_set.n,
            funcs: hash_set.funcs.clone(),
        }
    } else {
        // Key is new - add it
        let bsize = hash_set.bucket_size;
        let hash_func = &hash_set.funcs.hash;
        let indx_mod = (hash_func.call(&entry) % bsize).rem_euclid(bsize);

       let newpos = hash_set.value_arr.0;
        let varr = value_array_add(&hash_set.value_arr, entry.clone());

        let mut indexes = hash_set.hash_vec[indx_mod as usize].clone();
        indexes.push_front((entry, newpos));
        let mut hash_vec = hash_set.hash_vec.clone();
        hash_vec[indx_mod as usize] = indexes;
        let n_new = varr.0;

        HashSet {
            hash_vec,
            value_arr: varr,
            bucket_size: bsize,
            n: n_new,
            funcs: hash_set.funcs.clone(),
        }
    }
}

// ============================================================================
// addNoUpdCheck
// ============================================================================

/// Add a key to the hashset without checking if it already exists.
/// More efficient when you already know the key is not present.
pub fn add_no_upd_check<K: Clone + Debug + Send + Sync + 'static>(
    entry: K,
    hash_set: &HashSet<K>,
) -> HashSet<K> {
    let bsize = hash_set.bucket_size;
    let hash_func = &hash_set.funcs.hash;
    let indx = (hash_func.call(&entry) % bsize).rem_euclid(bsize);

    let newpos = hash_set.value_arr.0;
    let varr_1 = value_array_add(&hash_set.value_arr, entry.clone());

    let mut indexes = hash_set.hash_vec[indx as usize].clone();
    indexes.push_front((entry, newpos));
    let mut hash_vec = hash_set.hash_vec.clone();
    hash_vec[indx as usize] = indexes;
    let n_1 = varr_1.0;

    HashSet {
        hash_vec,
        value_arr: varr_1,
        bucket_size: bsize,
        n: n_1,
        funcs: hash_set.funcs.clone(),
    }
}

// ============================================================================
// addUnique
// ============================================================================

/// Add a key to the hashset. Fails if the key is already present.
pub fn add_unique<K: Clone + Debug + Send + Sync + 'static>(
    entry: K,
    hash_set: &HashSet<K>,
) -> Result<HashSet<K>> {
    if has(&entry, hash_set) {
        bail!("addUnique: key already present in hashset");
    }
    Ok(add_no_upd_check(entry, hash_set))
}

// ============================================================================
// delete
// ============================================================================

/// Delete a key from the hashset.
/// Note: This only clears the entry in the value array (sets to NONE).
/// It does not remove the entry from the index table (hash_vec).
/// A lot of deletions will not make the hashset more compact.
pub fn delete<K: Clone + Debug + Send + Sync + 'static>(
    key: &K,
    hash_set: &HashSet<K>,
) -> Result<HashSet<K>> {
    let (fkey, indx) = get1(key, hash_set);
    match fkey {
        Some(_) => {
            let mut arr_1 = hash_set.value_arr.2.to_vec();
            arr_1[indx as usize] = None;
            Ok(HashSet {
                hash_vec: hash_set.hash_vec.clone(),
                value_arr: (
                    hash_set.value_arr.0,
                    hash_set.value_arr.1,
                    arr_1,
                ),
                bucket_size: hash_set.bucket_size,
                n: hash_set.n,
                funcs: hash_set.funcs.clone(),
            })
        }
        None => bail!("delete: key not found in hashset"),
    }
}

// ============================================================================
// has
// ============================================================================

/// Returns true if the key is in the hashset.
pub fn has<K: Clone + Debug + Send + Sync + 'static>(key: &K, hash_set: &HashSet<K>) -> bool {
    // Empty set containing nothing
    if hash_set.value_arr.0 == 0 {
        return false;
    }
    let (opt, _) = get1(key, hash_set);
    opt.is_some()
}

// ============================================================================
// hasAll
// ============================================================================

/// Returns true if all keys are in the hashset.
pub fn has_all<K: Clone + Debug + Send + Sync + 'static>(
    keys: &[K],
    hash_set: &HashSet<K>,
) -> bool {
    for key in keys {
        if !has(key, hash_set) {
            return false;
        }
    }
    true
}

// ============================================================================
// get
// ============================================================================

/// Returns the key from the hashset, or None if not present.
pub fn get<K: Clone + Debug + Send + Sync + 'static>(
    key: &K,
    hash_set: &HashSet<K>,
) -> Option<K> {
    let (opt, _) = get1(key, hash_set);
    opt
}

// ============================================================================
// printHashSet
// ============================================================================

/// Print the hashset as a list of string representations.
pub fn print_hash_set<K: Clone + Debug + Send + Sync + 'static>(
    hash_set: &HashSet<K>,
) {
    let lst = hash_set_list(hash_set);
    for (i, key) in lst.iter().enumerate() {
        if i > 0 {
            println!();
        }
        print!("{key:?}");
    }
}

// ============================================================================
// dumpHashSet
// ============================================================================

/// Dump the hashset for debugging.
pub fn dump_hash_set<K: Clone + Debug + Send + Sync + 'static>(
    hash_set: &HashSet<K>,
) {
    println!("HashSet:");
    print_hash_set(hash_set);
    println!();
}

// ============================================================================
// hashSetList
// ============================================================================

/// Returns the entries in the hashset as a list of keys.
pub fn hash_set_list<K: Clone + Debug + Send + Sync + 'static>(
    hash_set: &HashSet<K>,
) -> List<K> {
    value_array_list(&hash_set.value_arr)
}

// ============================================================================
// valueArrayList
// ============================================================================

/// Transform a ValueArray to a key list.
pub fn value_array_list<K: Clone + Debug + Send + Sync + 'static>(
    value_array: &ValueArray<K>,
) -> List<K> {
    let (size, _, arr) = value_array;
    let mut out_list: List<K> = List::new();

    for i in (0..*size).rev() {
        if let Some(ref e) = arr[i as usize] {
            out_list.push_front(e.clone());
        }
    }

    out_list
}

// ============================================================================
// currentSize
// ============================================================================

/// Returns the number of elements inserted into the table.
pub fn current_size<K: Clone + Debug + Send + Sync + 'static>(
    hash_set: &HashSet<K>,
) -> i32 {
    hash_set.value_arr.0
}

// ============================================================================
// valueArrayLength
// ============================================================================

/// Returns the number of elements in the ValueArray (current count).
pub fn value_array_length<K: Clone + Debug + Send + Sync + 'static>(
    value_array: &ValueArray<K>,
) -> i32 {
    value_array.0
}

// ============================================================================
// valueArrayAdd
// ============================================================================

/// Add an entry to the ValueArray, increasing array size if no space left
/// by factor 1.4.
pub fn value_array_add<K: Clone + Debug + Send + Sync + 'static>(
    value_array: &ValueArray<K>,
    entry: K,
) -> ValueArray<K> {
    let (n, size, arr) = value_array;

    if *n < *size {
        // Have space to add array element at position n.
        let mut arr_1 = arr.to_vec();
        arr_1[*n as usize] = Some(entry);
        (*n + 1, *size, arr_1)
    } else {
        // Do NOT have space. Expand with factor 1.4.
        let rsize = *size as f64;
        let rexpandsize = rsize * 0.4;
        let expandsize = rexpandsize as i32;
        let expandsize_1 = if expandsize < 1 { 1 } else { expandsize };
        let newsize = expandsize_1 + size;
        // Create new array and insert at position n
        let mut arr_2 = vec![None::<K>; newsize as usize];
        // Copy existing elements
        for i in 0..*n {
            arr_2[i as usize] = arr[i as usize].clone();
        }
        arr_2[*n as usize] = Some(entry);
        (*n + 1, newsize, arr_2)
    }
}

// ============================================================================
// valueArraySetnth
// ============================================================================

/// Set the nth variable in the ValueArray to value.
pub fn value_array_set_nth<K: Clone + Debug + Send + Sync + 'static>(
    value_array: &ValueArray<K>,
    pos: i32,
    entry: K,
) -> ValueArray<K> {
    let (n, size, arr) = value_array;
    if pos < *size {
        let mut arr_1 = arr.to_vec();
        arr_1[pos as usize] = Some(entry);
        (*n, *size, arr_1)
    } else {
        (*n, *size, arr.to_vec())
    }
}

// ============================================================================
// valueArrayClearnth
// ============================================================================

/// Clear the nth variable in the ValueArray (set to None).
pub fn value_array_clear_nth<K: Clone + Debug + Send + Sync + 'static>(
    value_array: &ValueArray<K>,
    pos: i32,
) -> ValueArray<K> {
    let (n, size, arr) = value_array;
    if pos < *size {
        let mut arr_1 = arr.to_vec();
        arr_1[pos as usize] = None;
        (*n, *size, arr_1)
    } else {
        (*n, *size, arr.to_vec())
    }
}

// ============================================================================
// valueArrayNth
// ============================================================================

/// Retrieve the nth value from ValueArray, index from 0..n-1.
pub fn value_array_nth<K: Clone + Debug + Send + Sync + 'static>(
    value_array: &ValueArray<K>,
    pos: i32,
) -> K {
    let (_, _, arr) = value_array;
    match &arr[pos as usize] {
        Some(k) => k.clone(),
        None => panic!("value_array_nth: index out of bounds"),
    }
}

// ============================================================================
// valueArrayNthT
// ============================================================================

/// Retrieve the nth value from ValueArray, index from 0..n-1.
/// Returns Option to avoid panics.
pub fn value_array_nth_t<K: Clone + Debug + Send + Sync + 'static>(
    value_array: &ValueArray<K>,
    pos: i32,
) -> Option<K> {
    let (_, _, arr) = value_array;
    if pos >= 0 && pos < value_array.1 {
        arr[pos as usize].clone()
    } else {
        None
    }
}

// ============================================================================
// Helper: expand_vec
// ============================================================================

/// Expand a Vec by adding extra elements.
/// This mimics the MetaModelica Array.expand function.
fn expand_vec<T: Clone>(extra: i32, existing: &[T], default: Option<T>) -> Vec<T> {
    let extra_usize = extra as usize;
    let mut new_vec = Vec::with_capacity(existing.len() + extra_usize);
    if let Some(ref val) = default {
        for _ in 0..extra_usize {
            new_vec.push(val.clone());
        }
    }
    for item in existing.iter() {
        new_vec.push(item.clone());
    }
    new_vec
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn make_string_funcs() -> FuncsTuple<String> {
        FuncsTuple {
            hash: FuncHash::new(|s: &String| {
                let mut h: i32 = 0;
                for c in s.chars() {
                    h = h.wrapping_mul(31).wrapping_add(c as i32);
                }
                h
            }),
            eq: FuncEq::new(|a: &String, b: &String| a == b),
            key_string: FuncKeyString::new(|s: &String| s.clone()),
        }
    }

    #[test]
    fn test_empty_hash_set() {
        let funcs = make_string_funcs();
        let hs = empty_hash_set_work(DEFAULT_BUCKET_SIZE, funcs);
        assert_eq!(current_size(&hs), 0);
        assert!(!has(&"hello".to_string(), &hs));
    }

    #[test]
    fn test_add_and_has() {
        let funcs = make_string_funcs();
        let mut hs = empty_hash_set_work(DEFAULT_BUCKET_SIZE, funcs);

        hs = add("hello".to_string(), &hs);
        assert!(has(&"hello".to_string(), &hs));
        assert_eq!(current_size(&hs), 1);

        hs = add("world".to_string(), &hs);
        assert!(has(&"world".to_string(), &hs));
        assert_eq!(current_size(&hs), 2);
    }

    #[test]
    fn test_add_duplicate() {
        let funcs = make_string_funcs();
        let mut hs = empty_hash_set_work(DEFAULT_BUCKET_SIZE, funcs);

        hs = add("hello".to_string(), &hs);
        hs = add("hello".to_string(), &hs); // duplicate
        assert_eq!(current_size(&hs), 1);
        assert!(has(&"hello".to_string(), &hs));
    }

    #[test]
    fn test_add_unique() {
        let funcs = make_string_funcs();
        let mut hs = empty_hash_set_work(DEFAULT_BUCKET_SIZE, funcs);

        hs = add_no_upd_check("hello".to_string(), &hs);
        assert!(add_unique("world".to_string(), &hs).is_ok());
        assert!(add_unique("hello".to_string(), &hs).is_err());
    }

    #[test]
    fn test_delete() {
        let funcs = make_string_funcs();
        let mut hs = empty_hash_set_work(DEFAULT_BUCKET_SIZE, funcs);

        hs = add("hello".to_string(), &hs);
        assert!(has(&"hello".to_string(), &hs));
        hs = delete(&"hello".to_string(), &hs).unwrap();
        assert!(!has(&"hello".to_string(), &hs));
    }

    #[test]
    fn test_get() {
        let funcs = make_string_funcs();
        let hs = empty_hash_set_work(DEFAULT_BUCKET_SIZE, funcs);
        let hs = add("hello".to_string(), &hs);

        assert_eq!(get(&"hello".to_string(), &hs), Some("hello".to_string()));
        assert_eq!(get(&"world".to_string(), &hs), None);
    }

    #[test]
    fn test_has_all() {
        let funcs = make_string_funcs();
        let mut hs = empty_hash_set_work(DEFAULT_BUCKET_SIZE, funcs);
        hs = add("a".to_string(), &hs);
        hs = add("b".to_string(), &hs);

        let keys = vec!["a".to_string(), "b".to_string()];
        assert!(has_all(&keys, &hs));

        let keys2 = vec!["a".to_string(), "c".to_string()];
        assert!(!has_all(&keys2, &hs));
    }

    #[test]
    fn test_hash_set_list() {
        let funcs = make_string_funcs();
        let mut hs = empty_hash_set_work(DEFAULT_BUCKET_SIZE, funcs);
        hs = add("first".to_string(), &hs);
        hs = add("second".to_string(), &hs);
        hs = add("third".to_string(), &hs);

        let lst = hash_set_list(&hs);
        assert_eq!(lst.len(), 3);
    }

    #[test]
    fn test_value_array_add_growth() {
        let funcs = make_string_funcs();
        let hs = empty_hash_set_work(3, funcs); // very small bucket to trigger growth

        let mut va = hs.value_arr;
        // Add more elements than the initial capacity to trigger growth
        for i in 0..10 {
            va = value_array_add(&va, format!("key_{i}"));
        }
        assert_eq!(value_array_length(&va), 10);
    }

    #[test]
    fn test_value_array_nth() {
        let funcs = make_string_funcs();
        let _hs = empty_hash_set_work(DEFAULT_BUCKET_SIZE, funcs);

        let va: ValueArray<String> = (0, 10, vec![None; 10]);
        let va = value_array_set_nth(&va, 0, "first".to_string());
        let va = value_array_set_nth(&va, 2, "third".to_string());

        assert_eq!(value_array_nth(&va, 0), "first");
        assert_eq!(value_array_nth(&va, 2), "third");
    }

    #[test]
    fn test_value_array_clear_nth() {
        let funcs = make_string_funcs();
        let _hs = empty_hash_set_work(DEFAULT_BUCKET_SIZE, funcs);
        let va: ValueArray<String> = (0, 10, vec![None; 10]);

        let va = value_array_set_nth(&va, 3, "test".to_string());
        assert_eq!(value_array_nth_t(&va, 3), Some("test".to_string()));

        let va = value_array_clear_nth(&va, 3);
        assert_eq!(value_array_nth_t(&va, 3), None);
    }

    #[test]
    fn test_bucket_to_values_size() {
        assert_eq!(bucket_to_values_size(257), 154); // 257 * 0.6 = 154.2
        assert_eq!(bucket_to_values_size(100), 60);
    }
}
