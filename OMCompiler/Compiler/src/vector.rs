//! Translation of Util/Vector.mo
//!
//! This module provides a generic dynamic array implementation translated from
//! MetaModelica. It uses `Mutable<T>` wrappers for the internal data and size,
//! matching the original design where these fields can be updated in-place.
//!
//! Note: MetaModelica uses 1-based indexing; all public functions that accept
//! indices expect 1-based values.

use anyhow::{bail, Result};
use crate::mutable::{Mutable, access, update};

// Persistent list type (mapped to im::Vector since im 15.x has no List)
type List<T> = im::Vector<T>;

// ============================================================================
// VECTOR record
// ============================================================================

/// A generic dynamic array, translated from the MetaModelica `Vector<T>` type.
///
/// Internally uses a `Vec<T>` wrapped in `Mutable` for the data and size.
/// The `size` field tracks the number of valid elements (logical size),
/// which may be less than the underlying capacity.
#[derive(Debug, Clone)]
pub struct VECTOR<T> {
    pub data: Mutable<Vec<T>>,
    pub size: Mutable<i32>,
}

impl<T> VECTOR<T> {
    /// Creates a new empty Vector with the given initial capacity.
    pub fn new(capacity: i32) -> Self {
        VECTOR {
            data: Mutable {
                data: Vec::with_capacity(capacity.max(0) as usize),
            },
            size: Mutable { data: 0 },
        }
    }
}

impl<T: Clone + Default> VECTOR<T> {
    // =========================================================================
    // Constructors
    // =========================================================================

    /// Creates a new Vector filled with the given value.
    pub fn new_fill(size: i32, value: T) -> Self {
        VECTOR {
            data: Mutable {
                data: vec![value; size.max(0) as usize],
            },
            size: Mutable { data: size.max(0) },
        }
    }

    /// Creates a Vector from a slice.
    pub fn from_array(arr: &[T]) -> Self {
        VECTOR {
            data: Mutable { data: arr.to_vec() },
            size: Mutable { data: arr.len() as i32 },
        }
    }

    /// Creates a Vector from a list.
    pub fn from_list(lst: &List<T>) -> Self {
        let data: Vec<T> = lst.iter().cloned().collect();
        VECTOR {
            data: Mutable { data },
            size: Mutable {
                data: lst.len() as i32,
            },
        }
    }

    // =========================================================================
    // Conversion
    // =========================================================================

    /// Converts a Vector to a Vec (array).
    pub fn to_array(&self) -> Vec<T> {
        let data = access(&self.data);
        let sz = access(&self.size);
        if *sz == data.len() as i32 {
            data.clone()
        } else {
            let mut arr = vec![T::default(); *sz as usize];
            for i in 0..*sz as usize {
                arr[i] = data[i].clone();
            }
            arr
        }
    }

    /// Converts a Vector to a list.
    pub fn to_list(&self) -> List<T> {
        let data = access(&self.data);
        let sz = access(&self.size);
        if *sz == data.len() as i32 {
            data.iter().cloned().collect()
        } else {
            let mut lst = im::vector![];
            for i in 0..*sz as usize {
                lst.push_back(data[i].clone());
            }
            lst
        }
    }

    // =========================================================================
    // Mutation
    // =========================================================================

    /// Appends a value to the end of the Vector.
    pub fn push(&mut self, value: T) {
        let mut sz = *access(&self.size);
        sz += 1;
        update(&mut self.size, sz);

        let data = reserve_capacity(&mut self.data, sz);
        data[sz as usize - 1] = value;
    }

    /// Inserts a value at the given 1-based index.
    /// Fails if the index is out of bounds, except for index == sz + 1 (append).
    pub fn insert(&mut self, value: T, index: i32) -> Result<()> {
        let sz = *access(&self.size);
        if index == sz + 1 {
            self.push(value);
            return Ok(());
        }
        if index < 1 || index > sz {
            bail!("insert index out of bounds");
        }

        let new_sz = sz + 1;
        update(&mut self.size, new_sz);
        let data = reserve_capacity(&mut self.data, new_sz);

        // Shift existing elements right by one position (1-based indexing)
        // From position sz down to index: data[i] = data[i-1]
        for i in (index..=sz).rev() {
            data[(i) as usize] = data[(i - 1) as usize].clone();
        }

        data[(index - 1) as usize] = value;
        Ok(())
    }

    /// Appends v2 to the end of v1.
    pub fn append(&mut self, other: &Self) {
        let sz1 = *access(&self.size);
        let data2 = access(&other.data);
        let sz2 = *access(&other.size);
        let new_sz = sz1 + sz2;

        let data1 = reserve_capacity(&mut self.data, new_sz);

        for i in 0..sz2 as usize {
            data1[(sz1 + i as i32) as usize] = data2[i].clone();
        }

        update(&mut self.size, new_sz);
    }

    /// Appends a list to the end of the Vector.
    pub fn append_list(&mut self, lst: &List<T>) {
        let sz = *access(&self.size);
        let lst_len = lst.len() as i32;
        let new_sz = sz + lst_len;

        let data = reserve_capacity(&mut self.data, new_sz);

        let mut iter = lst.iter();
        for i in (sz + 1)..=new_sz {
            let item = iter.next().unwrap();
            data[(i - 1) as usize] = item.clone();
        }

        update(&mut self.size, new_sz);
    }

    /// Appends an array to the end of the Vector.
    pub fn append_array(&mut self, arr: &[T]) {
        let sz = *access(&self.size);
        let arr_len = arr.len() as i32;
        let new_sz = sz + arr_len;

        let data = reserve_capacity(&mut self.data, new_sz);

        for i in 0..arr_len {
            data[(sz + i) as usize] = arr[i as usize].clone();
        }

        update(&mut self.size, new_sz);
    }

    /// Removes the last element. Fails if the Vector is empty.
    pub fn pop(&mut self) -> Result<()> {
        let sz = *access(&self.size);
        if sz == 0 {
            bail!("pop from empty vector");
        }

        let data = &mut self.data.data;
        data[(sz - 1) as usize] = T::default();
        update(&mut self.size, sz - 1);
        Ok(())
    }

    /// Removes all elements. Does not change capacity.
    pub fn clear(&mut self) {
        let data = &mut self.data.data;
        let sz = *access(&self.size);
        for i in 0..sz as usize {
            data[i] = T::default();
        }
        update(&mut self.size, 0);
    }

    /// Shrinks the Vector to newSize, or does nothing if newSize >= current size.
    pub fn shrink(&mut self, new_size: i32) -> Result<()> {
        let sz = *access(&self.size);
        if new_size < 0 {
            bail!("negative new size");
        }
        if new_size < sz {
            let data = &mut self.data.data;
            for i in new_size as usize..sz as usize {
                data[i] = T::default();
            }
            update(&mut self.size, new_size);
        }
        Ok(())
    }

    /// Grows the Vector to newSize, filling new elements with fill_value.
    pub fn grow(&mut self, new_size: i32, fill_value: T) -> Result<()> {
        let sz = *access(&self.size);
        if new_size <= sz {
            return Ok(());
        }

        let data = reserve_capacity(&mut self.data, new_size);

        for i in (sz + 1)..=new_size {
            data[(i - 1) as usize] = fill_value.clone();
        }

        update(&mut self.size, new_size);
        Ok(())
    }

    /// Resizes the Vector to newSize, filling new elements with fill_value if growing,
    /// or truncating if shrinking.
    pub fn resize(&mut self, new_size: i32, fill_value: T) -> Result<()> {
        let sz = *access(&self.size);
        if new_size < sz {
            self.shrink(new_size)?;
        } else if new_size > sz {
            self.grow(new_size, fill_value)?;
        }
        Ok(())
    }

    /// Removes the element at the given 1-based index. Fails if out of bounds.
    pub fn remove(&mut self, index: i32) -> Result<()> {
        let sz = *access(&self.size);

        if index == sz {
            return self.pop();
        }
        if index < 0 || index > sz {
            bail!("remove index out of bounds");
        }

        let data = &mut self.data.data;

        // Shift elements left to fill the gap
        for i in index..sz {
            data[(i - 1) as usize] = data[i as usize].clone();
        }

        update(&mut self.size, sz - 1);
        Ok(())
    }

    /// Updates the element at the given 1-based index. Fails if out of bounds.
    pub fn update(&mut self, index: i32, value: T) -> Result<()> {
        let data = &mut self.data.data;
        let sz = *access(&self.size);
        if index <= 0 || index > sz {
            bail!("update index out of bounds");
        }
        data[(index - 1) as usize] = value;
        Ok(())
    }

    /// Updates the element at the given 1-based index without bounds checking.
    /// DANGEROUS - only use when the index is already known to be in bounds.
    pub fn update_no_bounds(&mut self, index: i32, value: T) {
        let data = &mut self.data.data;
        data[(index - 1) as usize] = value;
    }

    // =========================================================================
    // Accessors
    // =========================================================================

    /// Returns the element at the given 1-based index. Fails if out of bounds.
    pub fn get(&self, index: i32) -> Result<T> {
        let data = access(&self.data);
        let sz = *access(&self.size);
        if index <= 0 || index > sz {
            bail!("get index out of bounds");
        }
        Ok(data[(index - 1) as usize].clone())
    }

    /// Returns the element at the given 1-based index without bounds checking.
    /// DANGEROUS - only use when the index is already known to be in bounds.
    pub fn get_no_bounds(&self, index: i32) -> T {
        let data = access(&self.data);
        data[(index - 1) as usize].clone()
    }

    /// Returns the last element. Fails if the Vector is empty.
    pub fn last(&self) -> Result<T> {
        let data = access(&self.data);
        let sz = *access(&self.size);
        if sz == 0 {
            bail!("last from empty vector");
        }
        Ok(data[(sz - 1) as usize].clone())
    }

    /// Returns the number of elements in the Vector.
    pub fn size(&self) -> i32 {
        *access(&self.size)
    }

    /// Returns the capacity (number of elements the Vector can store without reallocating).
    pub fn capacity(&self) -> i32 {
        access(&self.data).capacity() as i32
    }

    /// Returns true if the Vector is empty.
    pub fn is_empty(&self) -> bool {
        *access(&self.size) == 0
    }

    /// Increases the capacity to the given amount. Does nothing if already large enough.
    pub fn reserve(&mut self, new_capacity: i32) {
        let data = &mut self.data.data;
        if new_capacity > data.capacity() as i32 {
            let mut new_data = Vec::with_capacity(new_capacity.max(0) as usize);
            new_data.extend(data.iter().cloned());
            *data = new_data;
        }
    }

   /// Shrinks the capacity to match the actual number of elements.
    pub fn trim(&mut self) {
        let data = &mut self.data.data;
        let sz = *access(&self.size);
        if sz < data.capacity() as i32 {
            data.shrink_to_fit();
        }
    }

    /// Fills the given 1-based interval [from, to] with the given value.
    /// Fails if any part of the interval is out of bounds.
    /// If from > to, does nothing.
    pub fn fill(&mut self, value: T, from: i32, to: i32) -> Result<()> {
        let sz = *access(&self.size);
        if from < 1 || to < 1 || from > sz || to > sz {
            bail!("fill interval out of bounds");
        }
        if from > to {
            return Ok(());
        }

        let data = &mut self.data.data;
        for i in from..=to {
            data[(i - 1) as usize] = value.clone();
        }
        Ok(())
    }

    // =========================================================================
    // Higher-order functions
    // =========================================================================

    /// Applies a function to each element and creates a new Vector from results.
    pub fn map<OT>(&self, mut fn_mut: impl FnMut(&T) -> OT, shrink: bool) -> VECTOR<OT>
    where
        OT: Default,
    {
        let data = access(&self.data);
        let sz = *access(&self.size);
        let len = if shrink { sz as usize } else { data.len() };

        let mut new_data = Vec::with_capacity(len);
        for i in 0..sz as usize {
            new_data.push(fn_mut(&data[i]));
        }

        VECTOR {
            data: Mutable { data: new_data },
            size: Mutable { data: sz },
        }
    }

    /// Applies a function to each element and creates a new list from results.
    pub fn map_to_list<OT>(&self, mut fn_mut: impl FnMut(&T) -> OT) -> List<OT>
    where
        OT: Clone,
    {
        let data = access(&self.data);
        let sz = *access(&self.size);

        let mut result = im::vector![];
        for i in (0..sz as usize).rev() {
            let item = fn_mut(&data[i]);
            result.push_front(item);
        }
        result
    }

    /// Applies the given function to each element, changing each element's value
    /// to the result of the call.
    pub fn apply(&mut self, mut fn_mut: impl FnMut(&T) -> T) {
        let sz = *access(&self.size);
        let data = &mut self.data.data;
        for i in 0..sz as usize {
            data[i] = fn_mut(&data[i]);
        }
    }

    /// Folds over the Vector, applying a function to each element with an accumulator.
    pub fn fold<FT>(&self, mut fn_mut: impl FnMut(&T, FT) -> FT, mut arg: FT) -> FT {
        let data = access(&self.data);
        let sz = *access(&self.size);
        for i in 0..sz as usize {
            arg = fn_mut(&data[i], arg);
        }
        arg
    }

    /// Returns the first element and its 1-based index for which the predicate is true,
    /// or (None, -1) if no match.
    pub fn find(&self, mut fn_mut: impl FnMut(&T) -> bool) -> (Option<T>, i32) {
        let data = access(&self.data);
        let sz = *access(&self.size);
        for i in 0..sz as usize {
            if fn_mut(&data[i]) {
                return (Some(data[i].clone()), (i + 1) as i32);
            }
        }
        (None, -1)
    }

    /// Returns the last element and its 1-based index for which the predicate is true,
    /// or (None, -1) if no match.
    pub fn find_last(&self, mut fn_mut: impl FnMut(&T) -> bool) -> (Option<T>, i32) {
        let data = access(&self.data);
        let sz = *access(&self.size);
        for i in (0..sz as usize).rev() {
            if fn_mut(&data[i]) {
                return (Some(data[i].clone()), (i + 1) as i32);
            }
        }
        (None, -1)
    }
}

/// Returns the first element and the index of that element for which the given
/// function returns true, but proceeds to check all other elements for better
/// solutions regarding an extra argument.
pub fn find_fold<T, FT, F>(v: &VECTOR<T>, arg: FT, mut pred_fn: F) -> (Option<T>, i32, FT)
where
    T: Clone,
    F: FnMut(&T, FT) -> (bool, FT),
{
    let data = access(&v.data);
    let sz = *access(&v.size);
    let mut oe: Option<T> = None;
    let mut index: i32 = -1;
    let mut arg = arg;

    for i in 0..sz as usize {
        let (res, new_arg) = pred_fn(&data[i], arg);
        arg = new_arg;
        if res && oe.is_none() {
            oe = Some(data[i].clone());
            index = (i + 1) as i32;
        }
    }

    (oe, index, arg)
}

// ============================================================================
// all, any, none predicates
// ============================================================================

/// Returns true if the given function returns true for all elements.
pub fn all<T, F>(v: &VECTOR<T>, mut pred_fn: F) -> bool
where
    T: Clone,
    F: FnMut(&T) -> bool,
{
    let data = access(&v.data);
    let sz = *access(&v.size);
    for i in 0..sz as usize {
        if !pred_fn(&data[i]) {
            return false;
        }
    }
    true
}

/// Returns true if the given function returns true for any element.
pub fn any<T, F>(v: &VECTOR<T>, mut pred_fn: F) -> bool
where
    T: Clone,
    F: FnMut(&T) -> bool,
{
    let data = access(&v.data);
    let sz = *access(&v.size);
    for i in 0..sz as usize {
        if pred_fn(&data[i]) {
            return true;
        }
    }
    false
}

/// Returns true if the given function returns true for none of the elements.
pub fn none<T, F>(v: &VECTOR<T>, mut pred_fn: F) -> bool
where
    T: Clone,
    F: FnMut(&T) -> bool,
{
    let data = access(&v.data);
    let sz = *access(&v.size);
    for i in 0..sz as usize {
        if pred_fn(&data[i]) {
            return false;
        }
    }
    true
}

// ============================================================================
// copy, deepCopy, swap
// ============================================================================

/// Creates a copy of the given Vector.
pub fn copy<T: Clone>(v: &VECTOR<T>) -> VECTOR<T> {
    let data = access(&v.data);
    let sz = *access(&v.size);
    VECTOR {
        data: Mutable {
            data: data.clone(),
        },
        size: Mutable { data: sz },
    }
}

/// Creates a deep copy of the given Vector using the given copy function.
pub fn deep_copy<T, F>(v: &VECTOR<T>, mut copy_fn: F) -> VECTOR<T>
where
    T: Clone,
    F: FnMut(&mut T),
{
    let data = access(&v.data);
    let sz = *access(&v.size);

    let mut new_data: Vec<T> = data.iter().cloned().collect();
    for i in 0..new_data.len() {
        copy_fn(&mut new_data[i]);
    }

    VECTOR {
        data: Mutable { data: new_data },
        size: Mutable { data: sz },
    }
}

/// Swaps the contents of two Vectors.
pub fn swap<T>(v1: &mut VECTOR<T>, v2: &mut VECTOR<T>) {
    std::mem::swap(&mut v1.data.data, &mut v2.data.data);
    std::mem::swap(&mut v1.size.data, &mut v2.size.data);
}

/// Creates a string representation of the Vector.
pub fn to_string<T, F>(v: &VECTOR<T>, string_fn: F, begin: &str, delim: &str, end: &str) -> String
where
    T: Clone + Default,
    F: Fn(&T) -> String,
{
    let arr = v.to_array();
    let items: Vec<String> = arr.iter().map(|e| string_fn(e)).collect();
    let joined = items.join(delim);
    format!("{}{}{}", begin, joined, end)
}

// ============================================================================
// Protected helpers
// ============================================================================

/// Protected helper: resizes the internal array to the given size,
/// copying elements from the old array to the new one.
fn resize_array<T: Clone>(arr: &[T], new_size: usize) -> Vec<T> {
    let mut out = Vec::with_capacity(new_size);
    let len = arr.len().min(new_size);
    for i in 0..len {
        out.push(arr[i].clone());
    }
    out
}

/// Protected helper: ensures the vector has enough capacity and length for `new_size` elements.
/// Grows capacity by doubling if needed, extends length with defaults as necessary.
fn reserve_capacity<T: Clone + Default>(data: &mut Mutable<Vec<T>>, new_size: i32) -> &mut Vec<T> {
    let cap = data.data.capacity() as i32;
    let len = data.data.len() as i32;
    // Grow capacity if needed
    if new_size > cap {
        let mut cap = cap.max(1);
        while new_size > cap {
            cap *= 2;
        }
        let old_data = std::mem::replace(&mut data.data, Vec::with_capacity(cap as usize));
        let new_data = resize_array(&old_data, cap as usize);
        data.data = new_data;
    }
    // Extend length if needed
    if new_size > len {
        let data = &mut data.data;
        while data.len() < new_size as usize {
            data.push(T::default());
        }
    }
    &mut data.data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_size() {
        let v: VECTOR<i32> = VECTOR::new(10);
        assert_eq!(v.size(), 0);
        assert!(v.is_empty());
        assert_eq!(v.capacity(), 10);
    }

    #[test]
    fn test_new_fill() {
        let v: VECTOR<i32> = VECTOR::new_fill(5, 42);
        assert_eq!(v.size(), 5);
        assert_eq!(v.get(1).unwrap(), 42);
        assert_eq!(v.get(5).unwrap(), 42);
    }

    #[test]
    fn test_from_array() {
        let arr = vec![10, 20, 30];
        let v: VECTOR<i32> = VECTOR::from_array(&arr);
        assert_eq!(v.size(), 3);
        assert_eq!(v.get(1).unwrap(), 10);
        assert_eq!(v.get(3).unwrap(), 30);
    }

    #[test]
    fn test_to_array() {
        let arr = vec![1, 2, 3];
        let v: VECTOR<i32> = VECTOR::from_array(&arr);
        let result = v.to_array();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_push_and_pop() {
        let mut v: VECTOR<i32> = VECTOR::new(4);
        v.push(10);
        v.push(20);
        assert_eq!(v.size(), 2);
        assert_eq!(v.get(1).unwrap(), 10);
        assert_eq!(v.get(2).unwrap(), 20);

        v.pop().unwrap();
        assert_eq!(v.size(), 1);
    }

    #[test]
    fn test_insert() {
        let mut v: VECTOR<i32> = VECTOR::new(4);
        v.push(10);
        v.push(30);
        v.insert(20, 2).unwrap();
        assert_eq!(v.size(), 3);
        assert_eq!(v.get(1).unwrap(), 10);
        assert_eq!(v.get(2).unwrap(), 20);
        assert_eq!(v.get(3).unwrap(), 30);
    }

    #[test]
    fn test_insert_at_end_is_push() {
        let mut v: VECTOR<i32> = VECTOR::new(4);
        v.push(10);
        // Insert at position 2 (sz+1) should behave like push
        v.insert(20, 2).unwrap();
        assert_eq!(v.size(), 2);
        assert_eq!(v.get(2).unwrap(), 20);
    }

    #[test]
    fn test_insert_out_of_bounds() {
        let mut v: VECTOR<i32> = VECTOR::new(4);
        assert!(v.insert(10, 0).is_err());
        assert!(v.insert(10, 2).is_err()); // sz=0, only pos 1 is valid
    }

    #[test]
    fn test_remove() {
        let mut v: VECTOR<i32> = VECTOR::new(4);
        v.push(10);
        v.push(20);
        v.push(30);
        v.remove(2).unwrap();
        assert_eq!(v.size(), 2);
        assert_eq!(v.get(1).unwrap(), 10);
        assert_eq!(v.get(2).unwrap(), 30);
    }

    #[test]
    fn test_append() {
        let mut v1: VECTOR<i32> = VECTOR::new(4);
        v1.push(1);
        v1.push(2);
        let mut v2: VECTOR<i32> = VECTOR::new(4);
        v2.push(3);
        v2.push(4);
        v1.append(&v2);
        assert_eq!(v1.size(), 4);
        assert_eq!(v1.get(3).unwrap(), 3);
        assert_eq!(v1.get(4).unwrap(), 4);
    }

    #[test]
    fn test_clear() {
        let mut v: VECTOR<i32> = VECTOR::new(4);
        v.push(1);
        v.push(2);
        v.clear();
        assert_eq!(v.size(), 0);
        assert!(v.is_empty());
    }

    #[test]
    fn test_shrink() {
        let mut v: VECTOR<i32> = VECTOR::new(4);
        v.push(1);
        v.push(2);
        v.shrink(1).unwrap();
        assert_eq!(v.size(), 1);
    }

    #[test]
    fn test_grow() {
        let mut v: VECTOR<i32> = VECTOR::new(4);
        v.push(1);
        v.grow(5, 42).unwrap();
        assert_eq!(v.size(), 5);
        assert_eq!(v.get(3).unwrap(), 42);
        assert_eq!(v.get(5).unwrap(), 42);
    }

    #[test]
    fn test_resize() {
        let mut v: VECTOR<i32> = VECTOR::new(4);
        v.push(1);
        v.push(2);
        v.resize(5, 99).unwrap();
        assert_eq!(v.size(), 5);
        v.resize(1, 99).unwrap();
        assert_eq!(v.size(), 1);
    }

    #[test]
    fn test_get_failures() {
        let v: VECTOR<i32> = VECTOR::new(4);
        assert!(v.get(0).is_err());
        assert!(v.get(5).is_err());
    }

    #[test]
    fn test_last() {
        let mut v: VECTOR<i32> = VECTOR::new(4);
        v.push(10);
        v.push(20);
        assert_eq!(v.last().unwrap(), 20);
    }

    #[test]
    fn test_last_empty() {
        let v: VECTOR<i32> = VECTOR::new(4);
        assert!(v.last().is_err());
    }

    #[test]
    fn test_map() {
        let mut v: VECTOR<i32> = VECTOR::new(4);
        v.push(1);
        v.push(2);
        v.push(3);
        let result: VECTOR<i32> = v.map(|&x| x * 10, true);
        assert_eq!(result.size(), 3);
        assert_eq!(result.get(1).unwrap(), 10);
        assert_eq!(result.get(3).unwrap(), 30);
    }

    #[test]
    fn test_fold() {
        let mut v: VECTOR<i32> = VECTOR::new(4);
        v.push(1);
        v.push(2);
        v.push(3);
        let sum: i32 = v.fold(|&x, acc| acc + x, 0);
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_find() {
        let mut v: VECTOR<i32> = VECTOR::new(4);
        v.push(10);
        v.push(20);
        v.push(30);
        let (elem, idx) = v.find(|&x| x > 15);
        assert_eq!(elem, Some(20));
        assert_eq!(idx, 2);
    }

    #[test]
    fn test_find_none() {
        let mut v: VECTOR<i32> = VECTOR::new(4);
        v.push(1);
        let (elem, idx) = v.find(|&x| x > 100);
        assert_eq!(elem, None);
        assert_eq!(idx, -1);
    }

    #[test]
    fn test_find_last() {
        let mut v: VECTOR<i32> = VECTOR::new(4);
        v.push(10);
        v.push(20);
        v.push(30);
        let (elem, idx) = v.find_last(|&x| x > 5);
        assert_eq!(elem, Some(30));
        assert_eq!(idx, 3);
    }

    #[test]
    fn test_all() {
        let mut v: VECTOR<i32> = VECTOR::new(4);
        v.push(2);
        v.push(4);
        assert!(all(&v, |&x| x % 2 == 0));
    }

    #[test]
    fn test_any() {
        let mut v: VECTOR<i32> = VECTOR::new(4);
        v.push(1);
        v.push(2);
        assert!(any(&v, |&x| x > 1));
    }

    #[test]
    fn test_none() {
        let mut v: VECTOR<i32> = VECTOR::new(4);
        v.push(1);
        v.push(3);
        assert!(none(&v, |&x| x % 2 == 0));
    }

    #[test]
    fn test_copy() {
        let mut v: VECTOR<i32> = VECTOR::new(4);
        v.push(1);
        v.push(2);
        let c = copy(&v);
        assert_eq!(c.size(), 2);
        assert_eq!(c.get(1).unwrap(), 1);
    }

    #[test]
    fn test_swap() {
        let mut v1: VECTOR<i32> = VECTOR::new(4);
        let mut v2: VECTOR<i32> = VECTOR::new(4);
        v1.push(1);
        v2.push(2);
        swap(&mut v1, &mut v2);
        assert_eq!(v1.size(), 1);
        assert_eq!(v1.get(1).unwrap(), 2);
        assert_eq!(v2.size(), 1);
        assert_eq!(v2.get(1).unwrap(), 1);
    }

    #[test]
    fn test_apply() {
        let mut v: VECTOR<i32> = VECTOR::new(4);
        v.push(1);
        v.push(2);
        v.apply(|&x| x * 10);
        assert_eq!(v.get(1).unwrap(), 10);
        assert_eq!(v.get(2).unwrap(), 20);
    }

    #[test]
    fn test_to_string() {
        let mut v: VECTOR<i32> = VECTOR::new(4);
        v.push(1);
        v.push(2);
        let s = to_string(&v, |x| format!("{}", x), "[", ", ", "]");
        assert_eq!(s, "[1, 2]");
    }

    #[test]
    fn test_from_list_and_to_list() {
        let lst = im::vector![10, 20, 30];
        let v: VECTOR<i32> = VECTOR::from_list(&lst);
        assert_eq!(v.size(), 3);
        let lst2 = v.to_list();
        assert_eq!(lst2.len(), 3);
        assert_eq!(lst2.get(0), Some(&10));
    }

    #[test]
    fn test_append_list() {
        let mut v: VECTOR<i32> = VECTOR::new(4);
        v.push(1);
        let lst = im::vector![2, 3, 4];
        v.append_list(&lst);
        assert_eq!(v.size(), 4);
        assert_eq!(v.get(4).unwrap(), 4);
    }

    #[test]
    fn test_append_array() {
        let mut v: VECTOR<i32> = VECTOR::new(4);
        v.push(1);
        let arr = [2, 3];
        v.append_array(&arr);
        assert_eq!(v.size(), 3);
        assert_eq!(v.get(3).unwrap(), 3);
    }

    #[test]
    fn test_reserve() {
        let mut v: VECTOR<i32> = VECTOR::new(2);
        v.push(1);
        v.reserve(10);
        assert_eq!(v.capacity(), 10);
    }

    #[test]
    fn test_deep_copy() {
        let mut v: VECTOR<i32> = VECTOR::new(4);
        v.push(1);
        v.push(2);
        let c = deep_copy(&v, |x| *x *= 10);
        assert_eq!(c.get(1).unwrap(), 10);
        assert_eq!(c.get(2).unwrap(), 20);
    }

    #[test]
    fn test_find_fold() {
        let mut v: VECTOR<i32> = VECTOR::new(4);
        v.push(1);
        v.push(5);
        v.push(3);
        let (elem, idx, _) = find_fold(&v, 0i32, |&x, acc| {
            let gt = x > 2;
            (gt, acc + x)
        });
        assert_eq!(elem, Some(5));
        assert_eq!(idx, 2);
    }

    #[test]
    fn test_update() {
        let mut v: VECTOR<i32> = VECTOR::new(4);
        v.push(10);
        v.push(20);
        v.update(1, 99).unwrap();
        assert_eq!(v.get(1).unwrap(), 99);
    }

    #[test]
    fn test_update_no_bounds() {
        let mut v: VECTOR<i32> = VECTOR::new(4);
        v.push(10);
        v.update_no_bounds(1, 99);
        assert_eq!(v.get(1).unwrap(), 99);
    }

    #[test]
    fn test_fill() {
        let mut v: VECTOR<i32> = VECTOR::new(4);
        v.push(1);
        v.push(2);
        v.push(3);
        v.fill(99, 1, 2).unwrap();
        assert_eq!(v.get(1).unwrap(), 99);
        assert_eq!(v.get(2).unwrap(), 99);
        assert_eq!(v.get(3).unwrap(), 3);
    }

    #[test]
    fn test_map_to_list() {
        let mut v: VECTOR<i32> = VECTOR::new(4);
        v.push(1);
        v.push(2);
        let lst: im::Vector<i32> = v.map_to_list(|&x| x * 10);
        assert_eq!(lst.len(), 2);
        assert_eq!(lst.get(0), Some(&10));
        assert_eq!(lst.get(1), Some(&20));
    }

    #[test]
    fn test_trim() {
        let mut v: VECTOR<i32> = VECTOR::new(10);
        v.push(1);
        assert_eq!(v.capacity(), 10);
        v.trim();
        // After trim, capacity should be 1 (matching size of 1 element)
        assert_eq!(v.capacity(), 1);
    }

    #[test]
    fn test_get_no_bounds() {
        let mut v: VECTOR<i32> = VECTOR::new(4);
        v.push(42);
        assert_eq!(v.get_no_bounds(1), 42);
    }
}
