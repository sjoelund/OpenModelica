//! Translation of Util/Array.mo
//!
//! This module provides array functions translated from MetaModelica to Rust.
//! It includes mapping, folding, reducing, sorting, copying, searching,
//! and comparison operations on arrays.
//!
//! Note: MetaModelica uses 1-based indexing; Rust uses 0-based indexing.
//! All functions here use 0-based indexing for Rust arrays.

use anyhow::{bail, Result};

// Persistent list type (mapped to im::Vector since im 15.x has no List)
type List<T> = im::Vector<T>;

// ============================================================================
// mapNoCopy<T>
// ============================================================================

/// Applies a function to each element of the array, modifying the array in place.
/// Since it will update the array values the returned array must have the same type,
/// and thus the applied function must also return the same type.
pub fn map_no_copy<T: Clone, F>(arr: &mut Vec<T>, mut f: F)
where
    F: FnMut(T) -> T,
{
    for i in 0..arr.len() {
        arr[i] = f(arr[i].clone());
    }
}

// ============================================================================
// mapNoCopy_1<T, ArgT>
// ============================================================================

/// Same as map_no_copy, but with an additional argument that's updated for each call.
pub fn map_no_copy_1<T, ArgT, F>(arr: &mut Vec<T>, mut f: F, mut arg: ArgT) -> ArgT
where
    T: Clone,
    F: FnMut(T, ArgT) -> (T, ArgT),
{
    for i in 0..arr.len() {
        let (new_val, new_arg) = f(arr[i].clone(), arg);
        arr[i] = new_val;
        arg = new_arg;
    }
    arg
}

// ============================================================================
// downheap (protected)
// ============================================================================

/// Helper function for heap sort. Maintains the min-heap property.
/// This is a direct translation of the 1-based indexing logic from MetaModelica.
fn downheap(arr: &mut Vec<i32>, n: i32, mut v: i32) -> Result<()> {
    let mut w = 2 * v + 1;
    let mut tmp;
    while w < n {
        if w + 1 < n {
            // arr[w+2] > arr[w+1] in 1-based indexing
            // In 0-based: arr[w+1] > arr[w]
            if arr[(w + 1) as usize] > arr[w as usize] {
                w += 1;
            }
        }
        // arr[v+1] >= arr[w+1] in 1-based indexing
        // In 0-based: arr[v] >= arr[w]
        if arr[v as usize] >= arr[w as usize] {
            return Ok(());
        }
        tmp = arr[v as usize];
        arr[v as usize] = arr[w as usize];
        arr[w as usize] = tmp;
        v = w;
        w = 2 * v + 1;
    }
    Ok(())
}

// ============================================================================
// heapSort
// ============================================================================

/// Sorts an array of integers in ascending order using heap sort.
pub fn heap_sort(arr: &mut Vec<i32>) -> Result<()> {
    let n = arr.len() as i32;
    // Build heap: for v in (n/2-1):-1:0 loop (1-based)
    // In 0-based: from (n/2-1) down to 0
    let start = if n > 0 { (n / 2) - 1 } else { 0 };
    for v in (0..=start).rev() {
        downheap(arr, n, v)?;
    }
    // Extract elements
    for v in (2..=n).rev() {
        let tmp = arr[0];
        arr[0] = arr[(v - 1) as usize];
        arr[(v - 1) as usize] = tmp;
        downheap(arr, v - 1, 0)?;
    }
    Ok(())
}

// ============================================================================
// findFirstOnTrue<T>
// ============================================================================

/// Finds the first element for which the predicate returns true.
/// Returns None if no element satisfies the predicate.
pub fn find_first_on_true<T, F>(arr: &[T], mut pred: F) -> Option<T>
where
    T: Clone,
    F: FnMut(&T) -> bool,
{
    for e in arr {
        if pred(e) {
            return Some(e.clone());
        }
    }
    None
}

// ============================================================================
// findFirstOnTrueWithIdx<T>
// ============================================================================

/// Finds the first element for which the predicate returns true,
/// along with its 0-based index. Returns (None, -1) if no match.
pub fn find_first_on_true_with_idx<T, F>(arr: &[T], mut pred: F) -> (Option<T>, i32)
where
    T: Clone,
    F: FnMut(&T) -> bool,
{
    for (idx, e) in arr.iter().enumerate() {
        if pred(e) {
            return (Some(e.clone()), idx as i32);
        }
    }
    (None, -1)
}

// ============================================================================
// select<T>
// ============================================================================

/// Selects elements from an array by index. Will panic if any index is out of bounds.
/// Note: MetaModelica uses 1-based indexing for the indices parameter.
/// This Rust version expects 0-based indices to match Rust conventions.
pub fn select<T>(arr: &[T], indices: &[i32]) -> Vec<T>
where
    T: Clone,
{
    let mut out = Vec::with_capacity(indices.len());
    for &idx in indices {
        out.push(arr[idx as usize].clone());
    }
    out
}

// ============================================================================
// map<TI, TO>
// ============================================================================

/// Applies a function to each element of the array, creating a new array.
/// The original array is left unchanged.
pub fn map<TI, TO, F>(arr: &[TI], mut f: F) -> Vec<TO>
where
    TI: Clone,
    F: FnMut(&TI) -> TO,
{
    if arr.is_empty() {
        return Vec::new();
    }
    let len = arr.len();
    let mut out = Vec::with_capacity(len);
    out.push(f(&arr[0]));
    for i in 1..len {
        out.push(f(&arr[i]));
    }
    out
}

// ============================================================================
// map1<TI, TO, ArgT>
// ============================================================================

/// Applies a function to each element with an extra argument, creating a new array.
pub fn map1<TI, TO, ArgT: Clone, F>(arr: &[TI], mut f: F, arg: ArgT) -> Vec<TO>
where
    TI: Clone,
    F: FnMut(&TI, ArgT) -> TO,
{
    if arr.is_empty() {
        return Vec::new();
    }
    let len = arr.len();
    let mut out = Vec::with_capacity(len);
    out.push(f(&arr[0], arg.clone()));
    for i in 1..len {
        out.push(f(&arr[i], arg.clone()));
    }
    out
}

// ============================================================================
// map1Ind<TI, TO, ArgT>
// ============================================================================

/// Applies a function to each element with an extra argument and 1-based index.
pub fn map1_ind<TI, TO, ArgT: Clone, F>(arr: &[TI], mut f: F, arg: ArgT) -> Vec<TO>
where
    TI: Clone,
    F: FnMut(&TI, i32, ArgT) -> TO,
{
    if arr.is_empty() {
        return Vec::new();
    }
    let len = arr.len();
    let mut out = Vec::with_capacity(len);
    // 1-based index: index 1
    out.push(f(&arr[0], 1, arg.clone()));
    for i in 1..len {
        // 1-based index: i+1
        out.push(f(&arr[i], (i + 1) as i32, arg.clone()));
    }
    out
}

// ============================================================================
// mapList<TI, TO>
// ============================================================================

/// Maps a function over a list and returns an array.
pub fn map_list<TI, TO, F>(lst: &List<TI>, mut f: F) -> Vec<TO>
where
    TI: Clone,
    F: FnMut(&TI) -> TO,
{
    let len = lst.len();
    if len == 0 {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(len);
    // First element
    let first = lst.get(0).unwrap();
    out.push(f(first));
    // Rest
    for i in 1..len {
        let item = lst.get(i).unwrap();
        out.push(f(item));
    }
    out
}

// ============================================================================
// fold<T, FoldT>
// ============================================================================

/// Folds over an array, applying a function to each element with an accumulator.
pub fn fold<T, FoldT, F>(arr: &[T], mut f: F, mut acc: FoldT) -> FoldT
where
    T: Clone,
    F: FnMut(&T, FoldT) -> FoldT,
{
    for e in arr {
        acc = f(e, acc);
    }
    acc
}

// ============================================================================
// foldIndex<T, FoldT>
// ============================================================================

/// Folds over an array with index, applying a function to each element.
/// The index is 1-based (matching MetaModelica conventions).
pub fn fold_index<T, FoldT, F>(arr: &[T], mut f: F, mut acc: FoldT) -> FoldT
where
    T: Clone,
    F: FnMut(&T, i32, FoldT) -> FoldT,
{
    for (idx, e) in arr.iter().enumerate() {
        // 1-based index
        acc = f(e, (idx + 1) as i32, acc);
    }
    acc
}

// ============================================================================
// reduce<T>
// ============================================================================

/// Reduces an array to a single value by repeatedly applying a binary function.
/// Panics if the array is empty.
pub fn reduce<T, F>(arr: &[T], mut f: F) -> T
where
    T: Clone,
    F: FnMut(T, T) -> T,
{
    let mut result = arr[0].clone();
    for i in 1..arr.len() {
        result = f(result, arr[i].clone());
    }
    result
}

// ============================================================================
// updateIndexFirst<T>
// ============================================================================

/// Like array_update, but with the index first so it can be used with List.map.
pub fn update_index_first<T: Clone>(index: i32, value: T, arr: &mut Vec<T>) {
    arr[index as usize] = value;
}

// ============================================================================
// getIndexFirst<T>
// ============================================================================

/// Like array_get, but with the index first so it can be used with List.map.
pub fn get_index_first<T: Clone>(index: i32, arr: &[T]) -> T {
    arr[index as usize].clone()
}

// ============================================================================
// replaceAtWithFill<T>
// ============================================================================

/// Replaces the value at the given position, padding with fill value if out of range.
/// Note: inPos is expected to be 1-based (MetaModelica convention).
pub fn replace_at_with_fill<T: Clone>(in_pos: i32, replace: T, arr: &[T], fill: T) -> Vec<T> {
    let mut out = expand_to_size(in_pos, arr, fill.clone());
    out[(in_pos - 1) as usize] = replace;
    out
}

// ============================================================================
// expandToSize<T>
// ============================================================================

/// Expands an array to the given size, or does nothing if already large enough.
/// Note: newSize is 1-based (MetaModelica convention).
pub fn expand_to_size<T: Clone>(new_size: i32, arr: &[T], fill: T) -> Vec<T> {
    // new_size is 1-based MetaModelica size (same as actual vector length)
    if new_size <= arr.len() as i32 {
        return arr.to_vec();
    }
    let new_size_usize = new_size as usize;
    let mut out = vec![fill; new_size_usize];
    for (i, item) in arr.iter().enumerate() {
        out[i] = item.clone();
    }
    out
}

// ============================================================================
// expand<T>
// ============================================================================

/// Increases the number of elements of an array by in_n.
/// Each new element is assigned the value in_fill.
pub fn expand<T: Clone>(n: i32, arr: &[T], fill: T) -> Vec<T> {
    if n < 1 {
        return arr.to_vec();
    }
    let len = arr.len();
    let mut out = vec![fill.clone(); len + n as usize];
    for (i, item) in arr.iter().enumerate() {
        out[i] = item.clone();
    }
    out
}

// ============================================================================
// expandOnDemand<T>
// ============================================================================

/// Resizes an array with the given factor if the array is smaller than the requested size.
pub fn expand_on_demand<T: Clone>(new_size: i32, arr: &[T], expansion_factor: f64, fill: T) -> Vec<T> {
    let len = arr.len() as i32;
    if new_size <= len {
        return arr.to_vec();
    }
    let actual_new_size = (len as f64 * expansion_factor).floor() as i32;
    let mut out = vec![fill; actual_new_size as usize];
    for (i, item) in arr.iter().enumerate() {
        out[i] = item.clone();
    }
    out
}

// ============================================================================
// consToElement<T>
// ============================================================================

/// Concatenates an element to a list element of an array (prepend).
pub fn cons_to_element<T: Clone>(index: i32, element: T, arr: &mut Vec<List<T>>) {
    let mut new_list = im::vector![element];
    new_list.extend(arr[index as usize].iter().cloned());
    arr[index as usize] = new_list;
}

// ============================================================================
// appendToElement<T>
// ============================================================================

/// Appends a list to a list element of an array.
pub fn append_to_element<T: Clone>(index: i32, elements: &List<T>, arr: &mut Vec<List<T>>) {
    let mut new_list = arr[index as usize].clone();
    new_list.extend(elements.iter().cloned());
    arr[index as usize] = new_list;
}

// ============================================================================
// appendList<T>
// ============================================================================

/// Returns a new array with the list elements added to the end of the given array.
pub fn append_list<T: Clone>(arr: &[T], lst: &List<T>) -> Vec<T> {
    if lst.is_empty() {
        return arr.to_vec();
    }
    if arr.is_empty() {
        return lst.iter().cloned().collect();
    }
    let arr_len = arr.len();
    let lst_len = lst.len();
    let mut out = vec![arr[0].clone(); arr_len + lst_len];
    for (i, item) in arr.iter().enumerate() {
        out[i] = item.clone();
    }
    for (i, item) in lst.iter().enumerate() {
        out[arr_len + i] = item.clone();
    }
    out
}

// ============================================================================
// join<T>
// ============================================================================

/// Returns a new array consisting of the elements from both given arrays.
pub fn join<T: Clone>(arr1: &[T], arr2: &[T]) -> Vec<T> {
    if arr1.is_empty() {
        return arr2.to_vec();
    }
    if arr2.is_empty() {
        return arr1.to_vec();
    }
    let len1 = arr1.len();
    let len2 = arr2.len();
    let mut out = vec![arr1[0].clone(); len1 + len2];
    for (i, item) in arr1.iter().enumerate() {
        out[i] = item.clone();
    }
    for (i, item) in arr2.iter().enumerate() {
        out[len1 + i] = item.clone();
    }
    out
}

// ============================================================================
// copy<T>
// ============================================================================

/// Copies all values from src to dest. Fails if src is larger than dest.
pub fn copy<T: Clone>(src: &[T], dest: &mut Vec<T>) -> Result<()> {
    if src.len() > dest.len() {
        bail!("source array larger than destination");
    }
    for i in 0..src.len() {
        dest[i] = src[i].clone();
    }
    Ok(())
}

// ============================================================================
// copyN<T>
// ============================================================================

/// Copies the first in_n values from src to dest. Fails if in_n is too large.
/// Note: src_offset and dst_offset are 0-based (Rust convention).
pub fn copy_n<T: Clone>(
    src: &[T],
    dest: &mut Vec<T>,
    n: i32,
    src_offset: i32,
    dst_offset: i32,
) -> Result<()> {
    if n + dst_offset > dest.len() as i32 || n + src_offset > src.len() as i32 {
        bail!("copy range exceeds array bounds");
    }
    for i in 0..n {
        dest[(i + dst_offset) as usize] = src[(i + src_offset) as usize].clone();
    }
    Ok(())
}

// ============================================================================
// copyRange<T>
// ============================================================================

/// Copies a range of elements from one array to another.
/// All indices are 1-based (MetaModelica convention).
pub fn copy_range<T: Clone>(
    src: &[T],
    dst: &mut Vec<T>,
    src_first: i32,
    src_last: i32,
    dst_pos: i32,
) -> Result<()> {
    let src_first_usize = (src_first - 1) as usize; // 1-based to 0-based
    let src_last_usize = (src_last - 1) as usize;   // 1-based to 0-based
    let dst_pos_usize = (dst_pos - 1) as usize;     // 1-based to 0-based

    if src_first > src_last || src_last > src.len() as i32 {
        bail!("invalid source range");
    }
    if dst_pos_usize + src_last_usize >= dst.len() {
        bail!("destination range too small");
    }

    let offset = dst_pos_usize - src_first_usize;
    for i in src_first_usize..=src_last_usize {
        dst[offset + i] = src[i].clone();
    }
    Ok(())
}

// ============================================================================
// createIntRange
// ============================================================================

/// Creates an array of i32 with values 1..=in_len (1-based range).
pub fn create_int_range(in_len: i32) -> Vec<i32> {
    (1..=in_len).collect()
}

// ============================================================================
// setRange<T>
// ============================================================================

/// Sets the elements in positions in_start to in_end to in_value.
/// Indices are 1-based (MetaModelica convention).
pub fn set_range<T: Clone>(in_start: i32, in_end: i32, arr: &mut Vec<T>, in_value: T) -> Result<()> {
    if in_start > arr.len() as i32 {
        bail!("start index out of bounds");
    }
    let start = (in_start - 1) as usize; // 1-based to 0-based
    let end = (in_end - 1) as usize;     // 1-based to 0-based
    for i in start..=end {
        arr[i] = in_value.clone();
    }
    Ok(())
}

// ============================================================================
// getRange<T>
// ============================================================================

/// Gets the elements between in_start and in_end as a list.
/// Indices are 1-based (MetaModelica convention).
pub fn get_range<T: Clone>(in_start: i32, in_end: i32, arr: &[T]) -> Result<List<T>> {
    if in_start > arr.len() as i32 {
        bail!("start index out of bounds");
    }
    let start = (in_start - 1) as usize; // 1-based to 0-based
    let end = (in_end - 1) as usize;     // 1-based to 0-based
    let mut result = im::vector![];
    for i in start..=end {
        result.push_back(arr[i].clone());
    }
    Ok(result)
}

// ============================================================================
// position<T>
// ============================================================================

/// Returns the 0-based index of the given element in the array, or None if not found.
/// Uses PartialEq for comparison.
pub fn position<T: PartialEq>(arr: &[T], element: &T) -> Option<i32> {
    for (i, e) in arr.iter().enumerate() {
        if *e == *element {
            return Some(i as i32);
        }
    }
    None
}

// ============================================================================
// getMemberOnTrue<VT, ET>
// ============================================================================

/// Returns the first element for which the comparison function returns true,
/// along with that element's 0-based index.
pub fn get_member_on_true<VT, ET, F>(value: &VT, arr: &[ET], mut comp: F) -> Result<(ET, i32)>
where
    ET: Clone,
    F: FnMut(&VT, &ET) -> bool,
{
    for (i, e) in arr.iter().enumerate() {
        if comp(value, e) {
            return Ok((e.clone(), i as i32));
        }
    }
    bail!("no element matched the predicate");
}

// ============================================================================
// reverse<T>
// ============================================================================

/// Reverses the elements in an array.
pub fn reverse<T: Clone>(arr: &[T]) -> Vec<T> {
    let mut out = arr.to_vec();
    let size = out.len();
    for i in 0..size / 2 {
        let elem1 = out[i].clone();
        let elem2 = out[size - 1 - i].clone();
        out[i] = elem2;
        out[size - 1 - i] = elem1;
    }
    out
}

// ============================================================================
// toString<T>
// ============================================================================

/// Creates a string from an array and a function that maps an array element to a string.
/// Also takes several parameters that determine the formatting.
///
/// Example: `to_string_with_opts([1, 2, 3], |x| format!("{}", x), "nums", "[", ";", "]", true, 0)`
///   => `"nums[1;2;3]"`
pub fn to_string_with_opts<T, F>(
    arr: &[T],
    print_fn: F,
    name_str: &str,
    begin_str: &str,
    delimit_str: &str,
    end_str: &str,
    print_empty: bool,
    max_length: i32,
) -> String
where
    F: Fn(&T) -> String,
{
    // Handle truncation for maxLength
    let items: Vec<String> = if max_length > 0 && (arr.len() as i32) > max_length {
        let mut result = Vec::with_capacity(max_length as usize);
        for item in arr.iter().take(max_length as usize) {
            result.push(print_fn(item));
        }
        result
    } else {
        arr.iter().map(|e| print_fn(e)).collect()
    };

    let end_str = if max_length > 0 && (arr.len() as i32) > max_length {
        format!("{}...{}", delimit_str, end_str)
    } else {
        end_str.to_string()
    };

    // Empty list handling
    if items.is_empty() {
        if print_empty {
            return format!("{}{}{}", name_str, begin_str, end_str);
        } else {
            return name_str.to_string();
        }
    }

    // Non-empty: join items with delimiter
    let joined = items.join(delimit_str);
    format!("{}{}{}{}", name_str, begin_str, joined, end_str)
}

/// Convenience wrapper for toString with default options.
pub fn to_string<T, F>(arr: &[T], print_fn: F) -> String
where
    F: Fn(&T) -> String,
{
    to_string_with_opts(arr, print_fn, "", "[", ", ", "]", true, 0)
}

// ============================================================================
// isEqual<T>
// ============================================================================

/// Checks if two arrays are equal (using PartialEq).
pub fn is_equal<T: PartialEq>(arr1: &[T], arr2: &[T]) -> Result<bool> {
    if arr1.len() != arr2.len() {
        return Ok(false);
    }
    for i in 0..arr1.len() {
        if arr1[i] != arr2[i] {
            return Ok(false);
        }
    }
    Ok(true)
}

// ============================================================================
// isEqualOnTrue<T1, T2>
// ============================================================================

/// Returns whether the two arrays are equal, using the given predicate function
/// to check element equality.
pub fn is_equal_on_true<T1, T2, F>(arr1: &[T1], arr2: &[T2], mut pred: F) -> bool
where
    F: FnMut(&T1, &T2) -> bool,
{
    if arr1.len() != arr2.len() {
        return false;
    }
    for i in 0..arr1.len() {
        if !pred(&arr1[i], &arr2[i]) {
            return false;
        }
    }
    true
}

// ============================================================================
// allEqual<T>
// ============================================================================

/// Returns true if all elements in the array are equal according to the predicate.
pub fn all_equal<T, F>(arr: &[T], mut pred: F) -> bool
where
    F: FnMut(&T, &T) -> bool,
{
    if arr.is_empty() {
        return true;
    }
    let first = &arr[0];
    for i in 1..arr.len() {
        if !pred(first, &arr[i]) {
            return false;
        }
    }
    true
}

// ============================================================================
// isLess<T1, T2>
// ============================================================================

/// Returns true if arr1 is less than arr2 using a lexicographical comparison.
/// Note: The original MetaModelica code calls lessFn(e2, e1) which is a type error
/// when T1 != T2 (the LessFn type is LessFn(T1, T2)). We only use lessFn(e1, e2).
pub fn is_less<T1, T2, F>(arr1: &[T1], arr2: &[T2], mut less_fn: F) -> bool
where
    F: FnMut(&T1, &T2) -> bool,
{
    let len1 = arr1.len();
    let len2 = arr2.len();
    let min_len = len1.min(len2);

    for i in 0..min_len {
        if less_fn(&arr1[i], &arr2[i]) {
            return true;
        }
        // Note: original code calls lessFn(e2, e1) but LessFn is LessFn(T1, T2).
        // This is a type error in the original MetaModelica code.
        // We treat equality when less_fn(e1, e2) is false.
    }

    // arr1 < arr2 if arr1 is a prefix of arr2 and all elements are equal
    len1 < len2
}

// ============================================================================
// insertList<T>
// ============================================================================

/// Inserts a list of elements into an array starting at the given position.
/// startPos is 0-based (Rust convention).
pub fn insert_list<T: Clone>(arr: &mut Vec<T>, lst: &[T], start_pos: usize) {
    for (i, e) in lst.iter().enumerate() {
        arr[start_pos + i] = e.clone();
    }
}

// ============================================================================
// remove<T>
// ============================================================================

/// Returns a new array without the element at the given index.
/// index is 0-based (Rust convention).
pub fn remove<T: Clone>(arr: &[T], index: usize) -> Vec<T> {
    if arr.len() <= 1 {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(arr.len() - 1);
    for i in 0..index {
        out.push(arr[i].clone());
    }
    for i in index + 1..arr.len() {
        out.push(arr[i].clone());
    }
    out
}

// ============================================================================
// all<T>
// ============================================================================

/// Returns true if the given predicate function returns true for all elements.
pub fn all<T, F>(arr: &[T], mut pred: F) -> bool
where
    F: FnMut(&T) -> bool,
{
    for e in arr {
        if !pred(e) {
            return false;
        }
    }
    true
}

// ============================================================================
// any<T>
// ============================================================================

/// Returns true if the given predicate function returns true for any element.
pub fn any<T, F>(arr: &[T], mut pred: F) -> bool
where
    F: FnMut(&T) -> bool,
{
    for e in arr {
        if pred(e) {
            return true;
        }
    }
    false
}

// ============================================================================
// minElement<T>
// ============================================================================

/// Returns the smallest element in the array, or fails if the array is empty.
pub fn min_element<T, F>(arr: &[T], mut less_fn: F) -> Result<T>
where
    T: Clone,
    F: FnMut(&T, &T) -> bool,
{
    if arr.is_empty() {
        bail!("array is empty");
    }
    let mut res = arr[0].clone();
    for i in 1..arr.len() {
        if less_fn(&arr[i], &res) {
            res = arr[i].clone();
        }
    }
    Ok(res)
}

// ============================================================================
// maxElement<T>
// ============================================================================

/// Returns the largest element in the array, or fails if the array is empty.
pub fn max_element<T, F>(arr: &[T], mut less_fn: F) -> Result<T>
where
    T: Clone,
    F: FnMut(&T, &T) -> bool,
{
    if arr.is_empty() {
        bail!("array is empty");
    }
    let mut res = arr[0].clone();
    for i in 1..arr.len() {
        if less_fn(&res, &arr[i]) {
            res = arr[i].clone();
        }
    }
    Ok(res)
}

// ============================================================================
// compare<T1, T2>
// ============================================================================

/// Returns -1 if arr1 is shorter than arr2 or 1 if arr1 is longer than arr2.
/// If both arrays are of equal length, applies the compare function to each pair
/// of array elements and returns the first nonzero value, or 0 if no nonzero value.
pub fn compare<T1, T2, F>(arr1: &[T1], arr2: &[T2], mut comp_fn: F) -> i32
where
    F: FnMut(&T1, &T2) -> i32,
{
    let l1 = arr1.len() as i32;
    let l2 = arr2.len() as i32;

    if l1 != l2 {
        return if l1 > l2 { 1 } else { -1 };
    }

    for i in 0..l1 {
        let res = comp_fn(&arr1[i as usize], &arr2[i as usize]);
        if res != 0 {
            return res;
        }
    }
    0
}

// ============================================================================
// mapFold<TI, TO, ArgT>
// ============================================================================

/// Combines map and fold: applies a function to each element with an accumulator,
/// producing a new array and a final accumulator value.
pub fn map_fold<TI, TO, ArgT, F>(arr: &[TI], mut f: F, arg: ArgT) -> (Vec<TO>, ArgT)
where
    TI: Clone,
    F: FnMut(&TI, ArgT) -> (TO, ArgT),
{
    let len = arr.len();
    if len == 0 {
        return (Vec::new(), arg);
    }

    let mut out = Vec::with_capacity(len);
    let (res, mut acc) = f(&arr[0], arg);
    out.push(res);

    for i in 1..len {
        let (res, new_acc) = f(&arr[i], acc);
        out.push(res);
        acc = new_acc;
    }
    (out, acc)
}

// ============================================================================
// transpose<T>
// ============================================================================

/// Transposes a two-dimensional array.
pub fn transpose<T: Clone>(arr: &[Vec<T>]) -> Vec<Vec<T>> {
    if arr.is_empty() {
        return arr.to_vec();
    }

    let row = &arr[0];
    if row.is_empty() {
        return arr.to_vec();
    }

    let c_len = arr.len();
    let r_len = row.len();
    let val = row[0].clone();

    // outArray[r][c] = arr[c][r]
    let mut out: Vec<Vec<T>> = (0..r_len)
        .map(|_| vec![val.clone(); c_len])
        .collect();

    for r in 0..r_len {
        for c in 0..c_len {
            out[r][c] = arr[c][r].clone();
        }
    }
    out
}

// ============================================================================
// threadMap<T1, T2, TO>
// ============================================================================

/// Creates an array with the result from calling the given function on each pair
/// of elements in two arrays. Panics if arrays have different lengths.
pub fn thread_map<T1, T2, TO, F>(arr1: &[T1], arr2: &[T2], mut f: F) -> Vec<TO>
where
    T1: Clone,
    T2: Clone,
    F: FnMut(&T1, &T2) -> TO,
{
    if arr1.is_empty() {
        return Vec::new();
    }
    if arr1.len() != arr2.len() {
        panic!("array lengths must be equal for thread_map");
    }

    let len = arr1.len();
    let mut out = Vec::with_capacity(len);
    out.push(f(&arr1[0], &arr2[0]));
    for i in 1..len {
        out.push(f(&arr1[i], &arr2[i]));
    }
    out
}

// ============================================================================
// generate<T>
// ============================================================================

/// Generates an array of length n by calling the given generator function for each element.
pub fn generate<T, F>(n: i32, mut generator: F) -> Vec<T>
where
    F: FnMut() -> T,
{
    if n <= 0 {
        return Vec::new();
    }
    let mut arr = Vec::with_capacity(n as usize);
    let first = generator();
    arr.push(first);
    for _ in 1..n {
        arr.push(generator());
    }
    arr
}

// ============================================================================
// filter<T>
// ============================================================================

/// Filters elements from an array based on a predicate function.
/// Elements for which the function returns true are removed.
pub fn filter<T, F>(arr: &[T], mut f: F) -> Vec<T>
where
    T: Clone,
    F: FnMut(&T) -> bool,
{
    // Count elements where f returns true (elements to remove)
    let remove_count = arr.iter().filter(|e| f(e)).count();
    let new_size = arr.len() - remove_count;
    let mut new_arr = Vec::with_capacity(new_size);

    for e in arr {
        if !f(e) {
            new_arr.push(e.clone());
        }
    }
    new_arr
}

// ============================================================================
// Test
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn test_heap_sort() {
        let mut arr = vec![5, 3, 8, 1, 2, 9, 4, 7, 6];
        heap_sort(&mut arr).unwrap();
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_find_first_on_true() {
        let arr = vec![1, 4, 3, 7, 2];
        assert_eq!(find_first_on_true(&arr, |&x| x > 3), Some(4));
        assert_eq!(find_first_on_true(&arr, |&x| x > 10), None);
    }

    #[test]
    fn test_find_first_on_true_with_idx() {
        let arr = vec![1, 4, 3, 7, 2];
        let (elem, idx) = find_first_on_true_with_idx(&arr, |&x| x > 3);
        assert_eq!(elem, Some(4));
        assert_eq!(idx, 1);
    }

    #[test]
    fn test_map() {
        let arr = vec![1, 2, 3];
        let result = map(&arr, |&x| x * 2);
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[test]
    fn test_map_empty() {
        let arr: Vec<i32> = vec![];
        let result = map(&arr, |&x| x * 2);
        assert!(result.is_empty());
    }

    #[test]
    fn test_fold() {
        let arr = vec![1, 2, 3, 4];
        let result = fold(&arr, |&x, acc| acc + x, 0);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_reduce() {
        let arr = vec![1, 2, 3];
        let result = reduce(&arr, |a, b| a + b);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_reverse() {
        let arr = vec![1, 2, 3, 4, 5];
        let result = reverse(&arr);
        assert_eq!(result, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_join() {
        let a = vec![1, 2];
        let b = vec![3, 4];
        let result = join(&a, &b);
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_replace_at_with_fill() {
        let arr = vec!['a', 'b', 'c'];
        let result = replace_at_with_fill(5, 'A', &arr, 'x');
        // 1-based index 5 = 0-based index 4, size = 5
        assert_eq!(result, vec!['a', 'b', 'c', 'x', 'A']);
    }

    #[test]
    fn test_all_any() {
        let arr = vec![2, 4, 6, 8];
        assert!(all(&arr, |&x| x % 2 == 0));
        assert!(!all(&arr, |&x| x > 5));
        assert!(any(&arr, |&x| x > 5));
        assert!(!any(&arr, |&x| x > 10));
    }

    #[test]
    fn test_remove() {
        let arr = vec![1, 2, 3, 4, 5];
        let result = remove(&arr, 2); // remove index 2 (value 3)
        assert_eq!(result, vec![1, 2, 4, 5]);
    }

    #[test]
    fn test_min_max() {
        let arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        assert_eq!(min_element(&arr, |a, b| a < b).unwrap(), 1);
        assert_eq!(max_element(&arr, |a, b| a < b).unwrap(), 9);
    }

    #[test]
    fn test_create_int_range() {
        let result = create_int_range(5);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_is_equal() {
        let a = vec![1, 2, 3];
        let b = vec![1, 2, 3];
        let c = vec![1, 2, 4];
        assert_eq!(is_equal(&a, &b).unwrap(), true);
        assert_eq!(is_equal(&a, &c).unwrap(), false);
    }

    #[test]
    fn test_is_less() {
        let a = vec![1, 2, 3];
        let b = vec![1, 2, 4];
        assert!(is_less(&a, &b, |a, b| a < b));
    }

    #[test]
    fn test_compare() {
        let a: Vec<i32> = vec![1, 2];
        let b: Vec<i32> = vec![1, 2, 3];
        let c: Vec<i32> = vec![1, 2];
        assert_eq!(compare(&a, &b, |a, b| match a.cmp(b) { Ordering::Less => -1, Ordering::Equal => 0, Ordering::Greater => 1 }), -1);
        assert_eq!(compare(&a, &c, |a, b| match a.cmp(b) { Ordering::Less => -1, Ordering::Equal => 0, Ordering::Greater => 1 }), 0);
    }

    #[test]
    fn test_filter() {
        let arr = vec![1, 2, 3, 4, 5, 6];
        let result = filter(&arr, |&x| x % 2 != 0); // remove odd
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[test]
    fn test_generate() {
        let mut counter = 0i32;
        let arr = generate(5, || {
            counter += 1;
            counter
        });
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_transpose() {
        let arr = vec![
            vec![1, 2],
            vec![3, 4],
            vec![5, 6],
        ];
        let result = transpose(&arr);
        assert_eq!(result, vec![
            vec![1, 3, 5],
            vec![2, 4, 6],
        ]);
    }

    #[test]
    fn test_thread_map() {
        let a = vec![1, 2, 3];
        let b = vec![10, 20, 30];
        let result = thread_map(&a, &b, |x, y| x + y);
        assert_eq!(result, vec![11, 22, 33]);
    }

    #[test]
    fn test_to_string() {
        let arr = vec![1, 2, 3];
        let result = to_string(&arr, |x| format!("{}", x));
        assert_eq!(result, "[1, 2, 3]");
    }

    #[test]
    fn test_to_string_with_opts() {
        let arr = vec![1, 2, 3];
        let result = to_string_with_opts(&arr, |x| format!("{}", x), "nums", "[", ";", "]", true, 0);
        assert_eq!(result, "nums[1;2;3]");
    }

    #[test]
    fn test_to_string_truncation() {
        let arr = vec![1, 2, 3, 4, 5];
        let result = to_string_with_opts(&arr, |x| format!("{}", x), "", "[", ";", "]", true, 3);
        assert_eq!(result, "[1;2;3;...]");
    }

    #[test]
    fn test_to_string_empty() {
        let arr: Vec<i32> = vec![];
        let result = to_string_with_opts(&arr, |x| format!("{}", x), "empty", "[", ";", "]", true, 0);
        assert_eq!(result, "empty[]");
    }

    #[test]
    fn test_map_fold() {
        let arr = vec![1, 2, 3];
        let (result, acc) = map_fold(&arr, |&x, acc| (x + acc, acc + 1), 0);
        assert_eq!(result, vec![1, 3, 5]); // 1, 2+1, 3+2
        assert_eq!(acc, 3);
    }

    #[test]
    fn test_map1() {
        let arr = vec![1, 2, 3];
        let result = map1(&arr, |&x, y| x * y, 10);
        assert_eq!(result, vec![10, 20, 30]);
    }

    #[test]
    fn test_select() {
        let arr = vec![10, 20, 30, 40];
        let indices = vec![0, 2, 3];
        let result = select(&arr, &indices);
        assert_eq!(result, vec![10, 30, 40]);
    }

    #[test]
    fn test_append_list() {
        let arr = vec![1, 2];
        let lst = im::vector![3, 4, 5];
        let result = append_list(&arr, &lst);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_expand() {
        let arr = vec![1, 2, 3];
        let result = expand(2, &arr, 0);
        assert_eq!(result, vec![1, 2, 3, 0, 0]);
    }

    #[test]
    fn test_equal_on_true() {
        let a = vec![1, 2, 3];
        let b = vec![1, 2, 3];
        assert!(is_equal_on_true(&a, &b, |x, y| x == y));
        let c = vec![1, 2, 4];
        assert!(!is_equal_on_true(&a, &c, |x, y| x == y));
    }

    #[test]
    fn test_all_equal() {
        let arr = vec![3, 3, 3];
        assert!(all_equal(&arr, |a, b| a == b));
        let arr2 = vec![1, 2, 1];
        assert!(!all_equal(&arr2, |a, b| a == b));
    }
}
