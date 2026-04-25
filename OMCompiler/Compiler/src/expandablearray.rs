//! Translation of Util/ExpandableArray.mo
//!
//! This module provides a generic expandable array implementation translated from
//! MetaModelica. It behaves like an ordinary array with automatic resizing when
//! the capacity is exceeded. Elements can be accessed by index and deleted from
//! any position.
//!
//! # Indexing
//! All public functions use 1-based indexing to match MetaModelica conventions.
//! Internally, indices are converted to 0-based for Rust Vec access.

use anyhow::{bail, Result};

use crate::array;
use crate::mutable::{self, Mutable};

// Persistent list type
type List<T> = im::Vector<T>;

// ============================================================================
// ExpandableArray<T> uniontype
// ============================================================================

/// An expandable array that automatically resizes when capacity is exceeded.
///
/// Elements are accessed by 1-based index. When an element is deleted, gaps
/// are left in the underlying storage; `compress()` can be used to remove gaps.
#[derive(Debug, Clone)]
pub struct ExpandableArray<T>
where
    T: Clone,
{
    pub number_of_elements: Mutable<i32>,
    pub last_used_index: Mutable<i32>,
    pub capacity: Mutable<i32>,
    pub data: Mutable<Vec<Option<T>>>,
}

// ============================================================================
// new
// ============================================================================

/// O(n) - Creates a new empty ExpandableArray with the given capacity.
pub fn new<T>(capacity: i32, _dummy: &T) -> ExpandableArray<T>
where
    T: Clone,
{
    ExpandableArray {
        number_of_elements: mutable::create(0),
        last_used_index: mutable::create(0),
        capacity: mutable::create(capacity),
        data: mutable::create(vec![None; capacity as usize]),
    }
}

// ============================================================================
// clear
// ============================================================================

/// O(n) - Deletes all elements from the expandable array.
pub fn clear<T>(exarray: &mut ExpandableArray<T>)
where
    T: Clone,
{
    let number_of_elements = *mutable::access(&exarray.number_of_elements);
    let last_used_index = *mutable::access(&exarray.last_used_index);

    mutable::update(&mut exarray.number_of_elements, 0);
    mutable::update(&mut exarray.last_used_index, 0);

    let mut data = mutable::access_mut(&mut exarray.data);
    for i in 1..=last_used_index as usize {
        if data[i - 1].is_some() {
            data[i - 1] = None;
            if i >= number_of_elements as usize {
                return;
            }
        }
    }
}

// ============================================================================
// copy
// ============================================================================

/// Creates a deep copy of the expandable array.
pub fn copy<T>(in_exarray: &ExpandableArray<T>, dummy: &T) -> ExpandableArray<T>
where
    T: Clone,
{
    let cap = *mutable::access(&in_exarray.capacity);
    let mut out = new(cap, dummy);
    out.number_of_elements = in_exarray.number_of_elements.clone();
    out.last_used_index = in_exarray.last_used_index.clone();
    out.capacity = in_exarray.capacity.clone();
    let data_val = mutable::access(&in_exarray.data).clone();
    mutable::update(&mut out.data, data_val);
    out
}

// ============================================================================
// occupied
// ============================================================================

/// O(1) - Returns true if the element at the given index is occupied.
pub fn occupied<T: Clone>(index: i32, exarray: &ExpandableArray<T>) -> bool {
    let last_used_index = *mutable::access(&exarray.last_used_index);
    let data = mutable::access(&exarray.data);

    index >= 1
        && index <= last_used_index
        && (index as usize) <= data.len()
        && data[(index - 1) as usize].is_some()
}

// ============================================================================
// get
// ============================================================================

/// O(1) - Returns the value at the given index.
/// Fails if there is no value at the given index.
pub fn get<T: Clone>(index: i32, exarray: &ExpandableArray<T>) -> Result<T> {
    let data = mutable::access(&exarray.data);
    let last_used_index = *mutable::access(&exarray.last_used_index);

    if !(index >= 1 && index <= last_used_index) {
        bail!("index {} out of bounds (1..={})", index, last_used_index);
    }
    let idx = ((index - 1) as usize).min(data.len() - 1);
    match data[idx].as_ref() {
        Some(value) => Ok(value.clone()),
        None => bail!("no value at index {}", index),
    }
}

// ============================================================================
// expand_to_size
// ============================================================================

/// O(n) - Expands the array to the given minimum capacity, or does nothing
/// if the array is already large enough.
pub fn expand_to_size<T: Clone>(min_capacity: i32, exarray: &mut ExpandableArray<T>) {
    let capacity = *mutable::access(&exarray.capacity);
    if min_capacity > capacity {
        let data = mutable::access(&exarray.data).clone();
        let new_data = array::expand_to_size(min_capacity, &data, None);
        mutable::update(&mut exarray.capacity, min_capacity);
        mutable::update(&mut exarray.data, new_data);
    }
}

// ============================================================================
// set
// ============================================================================

/// Sets the element at the given index to the given value.
/// Fails if the index is already used (occupied).
/// Capacity is automatically doubled if needed.
pub fn set<T: Clone>(index: i32, value: T, exarray: &mut ExpandableArray<T>) -> Result<()> {
    let number_of_elements = *mutable::access(&exarray.number_of_elements);
    let last_used_index = *mutable::access(&exarray.last_used_index);
    let capacity = *mutable::access(&exarray.capacity);

    // Check if index is out of bounds or already occupied
    let out_of_bounds = index as usize > capacity as usize;
    let occupied = index as usize <= capacity as usize
        && mutable::access(&exarray.data)[(index - 1) as usize].is_some();

    if !(index > 0 && (out_of_bounds || !occupied)) {
        bail!("index {} is already occupied", index);
    }

    // Expand if needed
    if index as usize > capacity as usize {
        let mut cap = capacity.max(1);
        while index > cap {
            cap *= 2;
        }
        expand_to_size(cap, exarray);
    }

    // Update the data
    let mut data = mutable::access_mut(&mut exarray.data);
    data[(index - 1) as usize] = Some(value);
    mutable::update(&mut exarray.number_of_elements, number_of_elements + 1);

    if index > last_used_index {
        mutable::update(&mut exarray.last_used_index, index);
    }

    Ok(())
}

// ============================================================================
// add
// ============================================================================

/// Appends the value at the first unused index.
/// Returns the index where the value was added.
pub fn add<T: Clone>(value: T, exarray: &mut ExpandableArray<T>) -> Result<i32> {
    let last_used_index = *mutable::access(&exarray.last_used_index);
    let index = last_used_index + 1;
    set(index, value, exarray)?;
    Ok(index)
}

// ============================================================================
// delete
// ============================================================================

/// Deletes the value at the given index.
/// Fails if there is no value at the given index.
pub fn delete<T: Clone>(index: i32, exarray: &mut ExpandableArray<T>) -> Result<()> {
    let number_of_elements = *mutable::access(&exarray.number_of_elements);
    let data = mutable::access(&exarray.data);
    let last_used_index = *mutable::access(&exarray.last_used_index);

    if !(index >= 1
        && index <= last_used_index
        && (index as usize) <= data.len()
        && data[(index - 1) as usize].is_some())
    {
        bail!("no value at index {} to delete", index);
    }

    // Need to drop the immutable borrow before mutating
    let data_ref = &exarray.data;
    let data_inner = mutable::access(data_ref);
    let was_some = data_inner[(index - 1) as usize].is_some();
    // The borrow ends naturally here; no explicit drop needed

    if !was_some {
        bail!("no value at index {} to delete", index);
    }

    let mut data = mutable::access_mut(&mut exarray.data);
    data[(index - 1) as usize] = None;
    mutable::update(&mut exarray.number_of_elements, number_of_elements - 1);

    if index == last_used_index {
        let mut luidx = last_used_index - 1;
        while luidx > 0 {
            let data = mutable::access(&exarray.data);
            let idx = ((luidx - 1) as usize).min(data.len().saturating_sub(1));
            if data[idx].is_none() {
                luidx -= 1;
            } else {
                break;
            }
        }
        mutable::update(&mut exarray.last_used_index, luidx);
    }

    Ok(())
}

// ============================================================================
// update
// ============================================================================

/// Overrides the value at the given index.
/// Fails if there is no value at the given index.
pub fn update<T: Clone>(index: i32, value: T, exarray: &mut ExpandableArray<T>) -> Result<()> {
    let last_used_index = *mutable::access(&exarray.last_used_index);
    let data = mutable::access(&exarray.data);

    if !(index >= 1
        && index <= last_used_index
        && (index as usize) <= data.len()
        && data[(index - 1) as usize].is_some())
    {
        bail!("no value at index {} to update", index);
    }

    let mut data = mutable::access_mut(&mut exarray.data);
    data[(index - 1) as usize] = Some(value);
    Ok(())
}

// ============================================================================
// to_list
// ============================================================================

/// Converts the expandable array to a persistent list.
/// Only includes elements that have a value (not None).
pub fn to_list<T: Clone>(exarray: &ExpandableArray<T>) -> List<T> {
    let number_of_elements = *mutable::access(&exarray.number_of_elements);
    let last_used_index = *mutable::access(&exarray.last_used_index);
    let data = mutable::access(&exarray.data);

    if number_of_elements == 0 {
        return im::vector![];
    }

    let mut result: List<T> = im::vector![];
    for i in 1..=last_used_index {
        let idx = ((i - 1) as usize).min(data.len().saturating_sub(1));
        if let Some(ref val) = data[idx] {
            result.push_back(val.clone());
        }
    }
    result
}

// ============================================================================
// compress
// ============================================================================

/// O(n) - Reorders elements to remove gaps.
/// Warning: This changes the indices of the elements.
pub fn compress<T: Clone>(exarray: &mut ExpandableArray<T>) {
    let number_of_elements = *mutable::access(&exarray.number_of_elements);
    let mut last_used_index = *mutable::access(&exarray.last_used_index);

    while last_used_index > number_of_elements && last_used_index > 0 {
        let mut found_none = false;
        let mut i: i32 = 1;

        while i <= last_used_index && last_used_index > number_of_elements {
            let data = mutable::access(&exarray.data);
            let idx = ((i - 1) as usize).min(data.len().saturating_sub(1));
            if data[idx].is_none() {
                found_none = true;

                // Find next non-None from the end
                let mut src = last_used_index;
                let data = mutable::access(&exarray.data);
                while src > 0 {
                    let src_idx = ((src - 1) as usize).min(data.len().saturating_sub(1));
                    if data[src_idx].is_some() {
                        break;
                    }
                    src -= 1;
                }

                if src > 0 {
                    let mut data = mutable::access_mut(&mut exarray.data);
                    let dst_idx = ((i - 1) as usize).min(data.len().saturating_sub(1));
                    let src_idx = ((src - 1) as usize).min(data.len().saturating_sub(1));
                    data[dst_idx] = data[src_idx].clone();
                    data[src_idx] = None;
                    last_used_index -= 1;
                }
            }
            i += 1;
        }

        if !found_none {
            break;
        }
    }

    mutable::update(&mut exarray.last_used_index, last_used_index);
}

// ============================================================================
// shrink
// ============================================================================

/// O(n) - Reduces the capacity to the number of elements.
/// Warning: This may change the indices of the elements.
pub fn shrink<T: Clone>(exarray: &mut ExpandableArray<T>) {
    let number_of_elements = *mutable::access(&exarray.number_of_elements);

    // Compress first to pack elements at the beginning
    compress(exarray);

    let new_cap = number_of_elements;

    // Create a new array of the right size
    let mut new_data = Vec::with_capacity(new_cap as usize);
    for i in 0..new_cap {
        let data = mutable::access(&exarray.data);
        if (i as usize) < data.len() {
            new_data.push(data[i as usize].clone());
        }
    }

    mutable::update(&mut exarray.capacity, new_cap);
    mutable::update(&mut exarray.data, new_data);
}

// ============================================================================
// to_string
// ============================================================================

/// O(n) - Dumps all elements with a given print function.
pub fn to_string<T, F>(exarray: &ExpandableArray<T>, header: &str, func: F, debug: bool) -> String
where
    T: Clone,
    F: Fn(&T) -> String,
{
    let number_of_elements = *mutable::access(&exarray.number_of_elements);
    let capacity = *mutable::access(&exarray.capacity);
    let data = mutable::access(&exarray.data);

    let str = if debug {
        format!("{} ({} / {})\n", header, number_of_elements, capacity)
    } else {
        format!("{} ({} )\n", header, number_of_elements)
    };

    let mut result = str.to_string();
    result.push_str("========================================\n");

    let mut remaining = number_of_elements;
    if remaining == 0 {
        result.push_str("<empty>\n");
    } else {
        for i in 1..=capacity {
            let idx = ((i - 1) as usize).min(data.len().saturating_sub(1));
            if let Some(ref value) = data[idx] {
                remaining -= 1;
                result.push_str(&format!("{}: {}\n", i, func(value)));
                if remaining == 0 {
                    break;
                }
            }
        }
    }

    result
}

// ============================================================================
// get_number_of_elements
// ============================================================================

/// Returns the number of elements currently in the array.
pub fn get_number_of_elements<T: Clone>(exarray: &ExpandableArray<T>) -> i32 {
    *mutable::access(&exarray.number_of_elements)
}

// ============================================================================
// get_last_used_index
// ============================================================================

/// Returns the last used index of the array.
pub fn get_last_used_index<T: Clone>(exarray: &ExpandableArray<T>) -> i32 {
    *mutable::access(&exarray.last_used_index)
}

// ============================================================================
// get_capacity
// ============================================================================

/// Returns the current capacity of the array.
pub fn get_capacity<T: Clone>(exarray: &ExpandableArray<T>) -> i32 {
    *mutable::access(&exarray.capacity)
}

// ============================================================================
// get_data
// ============================================================================

/// Returns a reference to the internal data array.
pub fn get_data<T: Clone>(exarray: &ExpandableArray<T>) -> &Vec<Option<T>> {
    mutable::access(&exarray.data)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_basic() {
        let arr = new(4, &0);
        assert_eq!(get_capacity(&arr), 4);
        assert_eq!(get_number_of_elements(&arr), 0);
        assert_eq!(get_last_used_index(&arr), 0);
    }

    #[test]
    fn test_add_and_get() {
        let mut arr = new(4, &0);
        let idx = add(10, &mut arr).unwrap();
        assert_eq!(idx, 1);
        assert_eq!(get(1, &arr).unwrap(), 10);
        assert_eq!(get_number_of_elements(&arr), 1);

        let idx = add(20, &mut arr).unwrap();
        assert_eq!(idx, 2);
        assert_eq!(get(2, &arr).unwrap(), 20);
    }

    #[test]
    fn test_set_and_update() {
        let mut arr = new(4, &0);
        set(1, 42, &mut arr).unwrap();
        assert_eq!(get(1, &arr).unwrap(), 42);

        update(1, 99, &mut arr).unwrap();
        assert_eq!(get(1, &arr).unwrap(), 99);
    }

    #[test]
    fn test_set_duplicate_fails() {
        let mut arr = new(4, &0);
        set(1, 10, &mut arr).unwrap();
        assert!(set(1, 20, &mut arr).is_err());
    }

    #[test]
    fn test_delete() {
        let mut arr = new(4, &0);
        add(10, &mut arr).unwrap();
        add(20, &mut arr).unwrap();
        delete(1, &mut arr).unwrap();
        assert_eq!(get_number_of_elements(&arr), 1);
        assert_eq!(get(2, &arr).unwrap(), 20);
    }

    #[test]
    fn test_occupied() {
        let mut arr = new(4, &0);
        assert!(!occupied(1, &arr));
        add(10, &mut arr).unwrap();
        assert!(occupied(1, &arr));
    }

    #[test]
    fn test_to_list() {
        let mut arr = new(4, &0);
        add(1, &mut arr).unwrap();
        add(2, &mut arr).unwrap();
        add(3, &mut arr).unwrap();
        let lst = to_list(&arr);
        assert_eq!(lst.len(), 3);
    }

    #[test]
    fn test_auto_expand() {
        let mut arr = new(2, &0);
        for i in 1..=10 {
            set(i, i * 10, &mut arr).unwrap();
        }
        assert_eq!(get_number_of_elements(&arr), 10);
        assert_eq!(get(5, &arr).unwrap(), 50);
    }

    #[test]
    fn test_copy() {
        let mut arr = new(4, &0);
        add(1, &mut arr).unwrap();
        add(2, &mut arr).unwrap();
        let copy = copy(&arr, &0);
        assert_eq!(get_number_of_elements(&copy), 2);
        assert_eq!(get(1, &copy).unwrap(), 1);
    }

    #[test]
    fn test_get_fails_on_none() {
        let arr = new(4, &0);
        assert!(get(1, &arr).is_err());
    }

    #[test]
    fn test_delete_fails_on_none() {
        let mut arr = new(4, &0);
        assert!(delete(1, &mut arr).is_err());
    }

    #[test]
    fn test_compress() {
        let mut arr = new(4, &0);
        add(1, &mut arr).unwrap();
        add(2, &mut arr).unwrap();
        add(3, &mut arr).unwrap();
        delete(2, &mut arr).unwrap();
        // Now we have [1, None, 3] with last_used_index=3
        compress(&mut arr);
        // After compress, elements should be packed: [1, 3, None]
        assert_eq!(get_number_of_elements(&arr), 2);
    }

    #[test]
    fn test_shrink() {
        let mut arr = new(8, &0);
        add(1, &mut arr).unwrap();
        add(2, &mut arr).unwrap();
        add(3, &mut arr).unwrap();
        shrink(&mut arr);
        assert_eq!(get_capacity(&arr), 3);
    }

    #[test]
    fn test_clear() {
        let mut arr = new(4, &0);
        add(1, &mut arr).unwrap();
        add(2, &mut arr).unwrap();
        clear(&mut arr);
        assert_eq!(get_number_of_elements(&arr), 0);
        assert_eq!(get_last_used_index(&arr), 0);
    }

    #[test]
    fn test_to_string() {
        let mut arr = new(4, &0);
        add(1, &mut arr).unwrap();
        add(2, &mut arr).unwrap();
        let s = to_string(&arr, "test", |x| format!("{}", x), false);
        assert!(s.contains("test"));
        assert!(s.contains("1"));
        assert!(s.contains("2"));
    }

    #[test]
    fn test_add_returns_index() {
        let mut arr = new(4, &0);
        let idx1 = add(10, &mut arr).unwrap();
        let idx2 = add(20, &mut arr).unwrap();
        assert_eq!(idx1, 1);
        assert_eq!(idx2, 2);
    }
}
