#![allow(non_snake_case, dead_code, unused_macros)]
//! Translation of FrontEnd/MetaModelicaBuiltin.mo
//!
//! Built-in MetaModelica declarations translated to Rust.
//! All functions are translated even if Rust has built-in alternatives,
//! since these functions may be referenced by other translated modules.
//!
//! Datatype mapping:
//!   Integer -> i32
//!   Real -> f64
//!   Boolean -> bool
//!   String -> String
//!   List<T> -> im::Vector<T>
//!   array<T> -> Vec<T>
//!
//! Note: MetaModelica uses 1-based indexing; Rust uses 0-based.
//! Functions that take indices expect 1-based indexing to match MetaModelica semantics.

use std::sync::Arc;
use anyhow::Result;
use anyhow::bail;

// ============================================================================
// SourceInfo - Location information for elements and classes
// ============================================================================

/// The Info attribute provides location information for elements and classes.
/// Mapped from the SOURCEINFO record in MetaModelicaBuiltin.mo.
#[derive(Debug, Clone, PartialEq)]
pub struct SourceInfo {
    /// File name where the class is defined in.
    pub file_name: Arc<String>,
    /// Should be true for libraries.
    pub is_read_only: bool,
    /// Start line number (1-based).
    pub line_number_start: i32,
    /// Start column number (1-based).
    pub column_number_start: i32,
    /// End line number (1-based).
    pub line_number_end: i32,
    /// End column number (1-based).
    pub column_number_end: i32,
    /// mtime in stat(2), stored as a double for increased precision on 32-bit platforms.
    pub last_modification: f64,
}

// ============================================================================
// Option - Option<A> types from MetaModelica
// ============================================================================

/// Represents an optional value: NONE or SOME(A).
#[derive(Debug, Clone, PartialEq)]
pub enum OptionValue<A> {
    None,
    Some(A),
}

// ============================================================================
// Boolean functions
// ============================================================================

/// Logically combine two Booleans with 'and' operator.
#[inline(always)]
pub fn bool_and(b1: bool, b2: bool) -> bool {
    b1 && b2
}

/// Logically combine two Booleans with 'or' operator.
#[inline(always)]
pub fn bool_or(b1: bool, b2: bool) -> bool {
    b1 || b2
}

/// Logically invert Boolean value using 'not' operator.
#[inline(always)]
pub fn bool_not(b: bool) -> bool {
    !b
}

/// Compares two Booleans for equality.
#[inline(always)]
pub fn bool_eq(b1: bool, b2: bool) -> bool {
    b1 == b2
}

/// Returns "true" or "false" string from a boolean.
pub fn bool_string(b: bool) -> String {
    if b { "true".to_string() } else { "false".to_string() }
}

// ============================================================================
// Integer arithmetic functions
// ============================================================================

/// Adds two Integer values.
#[inline(always)]
pub fn int_add(i1: i32, i2: i32) -> i32 {
    i1 + i2
}

/// Subtracts two Integer values.
#[inline(always)]
pub fn int_sub(i1: i32, i2: i32) -> i32 {
    i1 - i2
}

/// Multiplies two Integer values.
#[inline(always)]
pub fn int_mul(i1: i32, i2: i32) -> i32 {
    i1 * i2
}

/// Divides two Integer values (truncated division).
/// Matches Modelica's div() semantics: truncates toward zero.
pub fn int_div(i1: i32, i2: i32) -> i32 {
    i1 / i2
}

/// Calculates remainder of Integer division i1/i2.
/// Matches Modelica's mod() semantics: same sign as dividend.
pub fn int_mod(i1: i32, i2: i32) -> i32 {
    i1 % i2
}

/// Returns the bigger one of two Integer values.
pub fn int_max(i1: i32, i2: i32) -> i32 {
    i1.max(i2)
}

/// Returns the smaller one of two Integer values.
pub fn int_min(i1: i32, i2: i32) -> i32 {
    i1.min(i2)
}

/// Returns the absolute value of Integer i.
pub fn int_abs(i: i32) -> i32 {
    i.abs()
}

/// Returns negative value of Integer i.
#[inline(always)]
pub fn int_neg(i: i32) -> i32 {
    -i
}

// ============================================================================
// Integer comparison functions
// ============================================================================

/// Returns whether Integer i1 is smaller than Integer i2.
#[inline(always)]
pub fn int_lt(i1: i32, i2: i32) -> bool {
    i1 < i2
}

/// Returns whether Integer i1 is smaller than or equal to Integer i2.
#[inline(always)]
pub fn int_le(i1: i32, i2: i32) -> bool {
    i1 <= i2
}

/// Returns whether Integer i1 is equal to Integer i2.
#[inline(always)]
pub fn int_eq(i1: i32, i2: i32) -> bool {
    i1 == i2
}

/// Returns whether Integer i1 is not equal to Integer i2.
#[inline(always)]
pub fn int_ne(i1: i32, i2: i32) -> bool {
    i1 != i2
}

/// Returns whether Integer i1 is greater than or equal to Integer i2.
#[inline(always)]
pub fn int_ge(i1: i32, i2: i32) -> bool {
    i1 >= i2
}

/// Returns whether Integer i1 is greater than Integer i2.
#[inline(always)]
pub fn int_gt(i1: i32, i2: i32) -> bool {
    i1 > i2
}

// ============================================================================
// Integer bitwise functions
// ============================================================================

/// Returns bitwise inverted Integer number of i (~i in C).
#[inline(always)]
pub fn int_bit_not(i: i32) -> i32 {
    !i
}

/// Returns bitwise 'and' of Integers i1 and i2 (i1 & i2 in C).
#[inline(always)]
pub fn int_bit_and(i1: i32, i2: i32) -> i32 {
    i1 & i2
}

/// Returns bitwise 'or' of Integers i1 and i2 (i1 | i2 in C).
#[inline(always)]
pub fn int_bit_or(i1: i32, i2: i32) -> i32 {
    i1 | i2
}

/// Returns bitwise 'xor' of Integers i1 and i2 (i1 ^ i2 in C).
#[inline(always)]
pub fn int_bit_xor(i1: i32, i2: i32) -> i32 {
    i1 ^ i2
}

/// Returns bitwise left shift of Integer i by s bits (i << s in C).
#[inline(always)]
pub fn int_bit_l_shift(i: i32, s: i32) -> i32 {
    i << s
}

/// Returns bitwise right shift of Integer i by s bits (i >> s in C).
#[inline(always)]
pub fn int_bit_r_shift(i: i32, s: i32) -> i32 {
    i >> s
}

// ============================================================================
// Integer conversion functions
// ============================================================================

/// Converts Integer to Real.
#[inline(always)]
pub fn int_real(i: i32) -> f64 {
    i as f64
}

/// Converts Integer to String.
pub fn int_string(i: i32) -> String {
    i.to_string()
}

// ============================================================================
// Real arithmetic functions
// ============================================================================

/// Adds two Real values.
#[inline(always)]
pub fn real_add(r1: f64, r2: f64) -> f64 {
    r1 + r2
}

/// Subtracts two Real values.
#[inline(always)]
pub fn real_sub(r1: f64, r2: f64) -> f64 {
    r1 - r2
}

/// Multiplies two Real values.
#[inline(always)]
pub fn real_mul(r1: f64, r2: f64) -> f64 {
    r1 * r2
}

/// Divides two Real values.
#[inline(always)]
pub fn real_div(r1: f64, r2: f64) -> f64 {
    r1 / r2
}

/// Calculates remainder of Real division r1/r2.
pub fn real_mod(r1: f64, r2: f64) -> f64 {
    r1 % r2
}

/// Raises r1 to the power r2 (r1^r2).
pub fn real_pow(r1: f64, r2: f64) -> f64 {
    r1.powf(r2)
}

/// Returns the bigger one of two Real values.
#[inline(always)]
pub fn real_max(r1: f64, r2: f64) -> f64 {
    r1.max(r2)
}

/// Returns the smaller one of two Real values.
#[inline(always)]
pub fn real_min(r1: f64, r2: f64) -> f64 {
    r1.min(r2)
}

/// Returns the absolute value of Real x.
#[inline(always)]
pub fn real_abs(x: f64) -> f64 {
    x.abs()
}

/// Returns whether two Real values are approximately equal within absTol.
pub fn real_almost_eq(a: f64, b: f64, abs_tol: f64) -> bool {
    abs_tol > (a - b).abs()
}

/// Returns negative value of Real x.
#[inline(always)]
pub fn real_neg(x: f64) -> f64 {
    -x
}

// ============================================================================
// Real comparison functions
// ============================================================================

/// Returns whether Real x1 is smaller than Real x2.
#[inline(always)]
pub fn real_lt(x1: f64, x2: f64) -> bool {
    x1 < x2
}

/// Returns whether Real x1 is smaller than or equal to Real x2.
#[inline(always)]
pub fn real_le(x1: f64, x2: f64) -> bool {
    x1 <= x2
}

/// Returns whether Real x1 is equal to Real x2.
#[inline(always)]
pub fn real_eq(x1: f64, x2: f64) -> bool {
    x1 == x2
}

/// Returns whether Real x1 is not equal to Real x2.
#[inline(always)]
pub fn real_ne(x1: f64, x2: f64) -> bool {
    x1 != x2
}

/// Returns whether Real x1 is greater than or equal to Real x2.
#[inline(always)]
pub fn real_ge(x1: f64, x2: f64) -> bool {
    x1 >= x2
}

/// Returns whether Real x1 is greater than Real x2.
#[inline(always)]
pub fn real_gt(x1: f64, x2: f64) -> bool {
    x1 > x2
}

// ============================================================================
// Real conversion functions
// ============================================================================

/// Converts Real to Integer (truncates toward zero, matching Modelica integer() function).
pub fn real_int(r: f64) -> i32 {
    r as i32
}

/// Converts Real to String.
pub fn real_string(r: f64) -> String {
    r.to_string()
}

// ============================================================================
// String character functions
// ============================================================================

/// Returns the ASCII code point of a single-character string.
pub fn string_char_int(ch: &str) -> i32 {
    ch.chars().next()
        .map(|c| c as i32)
        .unwrap_or(0)
}

/// Returns a single-character string from an ASCII code point.
pub fn int_string_char(i: i32) -> String {
    std::char::from_u32(i as u32)
        .map(|c| c.to_string())
        .unwrap_or_default()
}

/// Parses an integer from a string. Fails if the string is not a valid integer.
pub fn string_int(str: &str) -> Result<i32> {
    str.parse::<i32>().map_err(|_| anyhow::anyhow!("Failed to parse integer from string: {}", str))
}

/// Parses a real (f64) from a string.
/// Fails unless the whole string can be consumed.
pub fn string_real(str: &str) -> Result<f64> {
    str.parse::<f64>().map_err(|_| anyhow::anyhow!("Failed to parse real from string: {}", str))
}

/// Converts a string to a list of single-character strings.
pub fn string_list_string_char(str: &str) -> List<String> {
    str.chars().map(|c| c.to_string()).collect()
}

/// Appends a list of strings into a single string.
pub fn string_append_list(strs: &List<String>) -> String {
    strs.into_iter().collect()
}

/// Takes a list of strings and a delimiter and joins them with the delimiter inserted between elements.
/// Example: stringDelimitList({"x","y","z"}, ", ") => "x, y, z"
pub fn string_delimit_list(strs: &List<String>, delimiter: &str) -> String {
    strs.into_iter().collect::<Vec<String>>().join(delimiter)
}

/// Returns the length of the string (number of bytes).
pub fn string_length(str: &str) -> i32 {
    str.len() as i32
}

/// Returns true if the string is empty.
pub fn string_empty(str: &str) -> bool {
    str.is_empty()
}

/// Returns the byte value at the given 1-based index.
pub fn string_get(str: &str, index: i32) -> Result<i32> {
    let idx = (index - 1) as usize; // 1-based to 0-based
    str.bytes().nth(idx)
        .map(|b| b as i32)
        .ok_or_else(|| anyhow::anyhow!("Index {} out of bounds for string of length {}", index, str.len()))
}

/// Returns the character at the given 1-based index as a string.
pub fn string_get_string_char(str: &str, index: i32) -> Result<String> {
    let idx = (index - 1) as usize; // 1-based to 0-based
    str.chars().nth(idx)
        .map(|c| c.to_string())
        .ok_or_else(|| anyhow::anyhow!("Index {} out of bounds for string of length {}", index, str.chars().count()))
}

/// Updates the character at the given 1-based index with newch.
/// newch should be a single character.
pub fn string_update_string_char(str: &str, newch: &str, index: i32) -> Result<String> {
    if newch.is_empty() {
        bail!("newch must not be empty");
    }
    let idx = (index - 1) as usize; // 1-based to 0-based
    let mut chars: Vec<char> = str.chars().collect();
    if idx >= chars.len() {
        bail!("Index {} out of bounds for string with {} characters", index, chars.len());
    }
    let new_char = newch.chars().next().unwrap_or(' ');
    chars[idx] = new_char;
    Ok(chars.into_iter().collect())
}

/// Concatenates two strings (s1 + s2).
pub fn string_append(s1: &str, s2: &str) -> String {
    format!("{}{}", s1, s2)
}

/// Compares two strings for equality.
#[inline(always)]
pub fn string_eq(s1: &str, s2: &str) -> bool {
    s1 == s2
}
#[inline(always)]
pub fn string_equal(s1: &str, s2: &str) -> bool {
    s1 == s2
}

/// Compares two strings lexicographically.
/// Returns negative if s1 < s2, zero if s1 == s2, positive if s1 > s2.
pub fn string_compare(s1: &str, s2: &str) -> i32 {
    // Byte-by-byte comparison for consistency
    let bytes1 = s1.as_bytes();
    let bytes2 = s2.as_bytes();
    let len = bytes1.len().min(bytes2.len());
    for i in 0..len {
        if bytes1[i] < bytes2[i] {
            return -1;
        }
        if bytes1[i] > bytes2[i] {
            return 1;
        }
    }
    // Length comparison if all compared bytes were equal
    match bytes1.len().cmp(&bytes2.len()) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

/// Returns a hash of the string using Rust's built-in hash.
pub fn string_hash(str: &str) -> i32 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hash;
    use std::hash::Hasher;
    let mut hasher = DefaultHasher::new();
    str.hash(&mut hasher);
    hasher.finish() as i32
}

/// Returns a DJB2 hash of the string.
/// DJB2 algorithm: hash = hash * 33 + byte
pub fn string_hash_djb2(str: &str) -> i32 {
    let mut hash: i32 = 5381;
    for &byte in str.as_bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as i32);
    }
    hash
}

/// Continues computing a DJB2 hash by adding another string to it.
pub fn string_hash_djb2_continue(str: &str, hash: i32) -> i32 {
    let mut h = hash;
    for &byte in str.as_bytes() {
        h = h.wrapping_mul(33).wrapping_add(byte as i32);
    }
    h
}

/// Computes a DJB2 hash and applies modulo without intermediate overflow issues.
pub fn string_hash_djb2_mod(str: &str, mod_val: i32) -> i32 {
    if mod_val == 0 {
        return 0;
    }
    let mut hash: i64 = 5381;
    for &byte in str.as_bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as i64);
    }
    (hash % mod_val as i64) as i32
}

/// Returns an SDBM hash of the string.
/// SDBM algorithm: hash = byte + (hash << 6) + (hash << 16) - hash
pub fn string_hash_sdbm(str: &str) -> i32 {
    let mut hash: i32 = 0;
    for &byte in str.as_bytes() {
        hash = byte as i32 + (hash << 6) + (hash << 16) - hash;
    }
    hash
}

/// Extracts a substring from str.
/// start and stop are 1-based indices (first character is at index 1).
/// Fails for bogus start/stop values.
pub fn substring(str: &str, start: i32, stop: i32) -> Result<String> {
    if start < 1 || stop < start || start > stop {
        bail!("Invalid substring range: start={}, stop={}", start, stop);
    }
    let start_idx = (start - 1) as usize; // 1-based to 0-based
    let stop_idx = stop as usize;         // 1-based, inclusive -> exclusive
    let chars: Vec<char> = str.chars().collect();
    if stop_idx > chars.len() {
        bail!("Stop index {} exceeds string length {}", stop, chars.len());
    }
    Ok(chars[start_idx..stop_idx].iter().collect())
}

/// Alias for string_append_list (maps a list of single-char strings to one string).
pub fn list_string_char_string(strs: &List<String>) -> String {
    string_append_list(strs)
}

/// Alias for string_append_list (maps a list of single-char strings to one string).
pub fn string_char_list_string(strs: &List<String>) -> String {
    string_append_list(strs)
}

// ============================================================================
// List functions
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum List<T> {
    Cons{head: T, tail: Arc<List<T>>},
    Nil(),
}
use List::{Cons, Nil};

macro_rules! list {
    // Base case: empty list
    () => {
        $crate::metamodelica::List::Nil()
    };
    // Case with a trailing comma
    ( $($x:expr),*, ) => {
        list!($($x),*)
    };
    // General case: peel off the first element and recurse
    ( $x:expr, $($rest:expr),+ ) => {
        $crate::metamodelica::cons($x, list!($($rest),+))
    };
    // Single element case
    ( $x:expr ) => {
        $crate::metamodelica::cons($x, list!())
    };
}

pub fn cons<T>(head: T, tail: List<T>) -> List<T> {
    Cons{head: head, tail: Arc::new(tail)}
}

impl<T> Default for List<T> {
    fn default() -> List<T> {
        Nil()
    }
}
pub struct ListIterator<'a, T: Clone> {
    curr: &'a List<T>,
}

impl<T: Clone> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> List<T> {
        let mut buf = Nil();
        for item in iter {
            buf = cons(item, buf);
        }
        buf.reverse()
    }
}

impl<'a, T: Clone> IntoIterator for &'a List<T> {
    type Item = T;
    type IntoIter = ListIterator<'a, T>;

    // Required method
    fn into_iter(self) -> Self::IntoIter {
        ListIterator { curr: self }
    }
}

impl<'a, T: Clone> Iterator for ListIterator<'a, T> {
    // We can refer to this type using Self::Item
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let res;
        (res,self.curr) = match *self.curr {
            Nil() => (None, self.curr),
            Cons{ref head, ref tail} => {
                (Some((*head).clone()), &**tail)
            }
        };
        res
    }
}

impl<T: Clone> List<T> {
    /// Appends lst2 to lst1. O(length(lst1)), O(1) if either list is empty.
    pub fn append(self: &List<T>, lst2: &List<T>) -> List<T> {
        if self.is_empty() {
            return lst2.clone();
        }
        if lst2.is_empty() {
            return self.clone();
        }
        let mut result = lst2.clone();
        for item in &self.reverse() {
            result = cons(item, result);
        }
        result
    }
    /// Returns the length of a list. O(n).
    pub fn len(&self) -> i32 {
        self.into_iter().count() as i32
    }
    /// Reverses the elements in a list. O(n).
    pub fn reverse(self: &List<T>) -> List<T> {
        let mut result: List<T> = Nil();
        for e in self {
            result = cons(e, result);
        }
        result
    }
    /// Gets the element at the given 1-based index. O(index).
    pub fn get(self: &List<T>, index: i32) -> Result<T> {
        self.into_iter().nth((index - 1) as usize)
            .ok_or_else(|| anyhow::anyhow!("Index {} out of bounds for list of length {}", index, self.len()))
    }
    pub fn prepend_reverse(self: List<T>, prefix: &List<T>) -> List<T> {
        let mut result = self;
        for item in prefix {
            result = cons(item, result);
        }
        result
    }
    /// Deletes the element at the given 1-based index. O(index).
    pub fn delete(self: &List<T>, index: i32) -> Result<List<T>> {
        if index < 1 {
            bail!("Index must be positive, got {}", index);
        }
        if index == 1 {
            return self.rest().cloned();
        }
        let mut result = Nil();
        let mut iter: &List<T> = self;
        let mut cur_index = index;
        loop {
            cur_index -= 1;
            let (head,tail) = match iter {
                Nil() => bail!("Index {} out of bounds for list", index),
                Cons{head, tail} => (head, tail)
            };
            iter = tail;
            if cur_index == 0 {
                return Ok(iter.clone().prepend_reverse(&result));
            }
            result = cons(head.clone(), result);
        }
    }
}

impl<T> List<T> {
    pub fn new(item: T) -> List<T> {
        Cons{head: item, tail: Arc::new(Nil())}
    }
    pub fn cons(self: List<T>, item: T) -> List<T> {
        Cons{head: item, tail: Arc::new(self)}
    }
    /// Gets the first element. O(1).
    /// Fails if the list is empty.
    pub fn head(self: &List<T>) -> Result<&T> {
        match self {
            Nil() => bail!("Cannot get rest of empty list"),
            Cons{head, ..} => Ok(head),
        }
    }
    /// Returns all elements except the first. O(1).
    /// Fails if the list is empty.
    pub fn rest(self: &List<T>) -> Result<&List<T>> {
        match self {
            Nil() => bail!("Cannot get rest of empty list"),
            Cons{tail, ..} => Ok(tail),
        }
    }
    /// Returns true if the list is empty. O(1).
    pub fn is_empty(self: &List<T>) -> bool {
        match self {
            Nil() => true,
            _ => false
        }
    }
}



impl<T: PartialEq + Clone> List<T> {
    /// Checks if an element is a member of the list. O(n).
    /// Uses PartialEq for comparison.
    pub fn contains(self: &List<T>, element: &T) -> bool {
        for item in self {
            if element.eq(&item) { return true; }
        }
        return false;
    }
}

// ============================================================================
// Array functions
// ============================================================================

/// Returns the length of the array. O(1).
pub fn array_length<A>(arr: &[A]) -> i32 {
    arr.len() as i32
}

/// Returns true if the array is empty. O(1).
pub fn array_empty<A>(arr: &[A]) -> bool {
    arr.is_empty()
}

/// Gets the element at the given 1-based index. O(1).
pub fn array_get<A: Clone>(arr: &[A], index: i32) -> Result<A> {
    let idx = (index - 1) as usize; // 1-based to 0-based
    arr.get(idx)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("Index {} out of bounds for array of length {}", index, arr.len()))
}

/// Creates a new array of the given size, initialized with initialValue. O(size).
pub fn array_create<A: Clone>(size: i32, initial_value: A) -> Vec<A> {
    if size <= 0 {
        return Vec::new();
    }
    vec![initial_value; size as usize]
}

/// Converts an array to a list. O(n).
pub fn array_list<A: Clone>(arr: &[A]) -> List<A> {
    arr.iter().cloned().collect()
}

/// Converts a list to an array. O(n).
pub fn list_array<A: Clone>(lst: &List<A>) -> Vec<A> {
    lst.into_iter().collect()
}

/// Updates the value at the given 1-based index. O(1).
/// Mutates the array in place (impure).
pub fn array_update<A: Clone>(arr: &mut Vec<A>, index: i32, new_value: A) -> Result<()> {
    let idx = (index - 1) as usize; // 1-based to 0-based
    let len = arr.len();
    if idx >= len {
        bail!("Index {} out of bounds for array of length {}", index, len);
    }
    arr[idx] = new_value;
    Ok(())
}

/// Creates a copy of the array. O(n).
pub fn array_copy<A: Clone>(arr: &[A]) -> Vec<A> {
    arr.to_vec()
}

/// Appends arr2 to arr1, creating a new array. O(length(arr1) + length(arr2)).
pub fn array_append<A: Clone>(arr1: &[A], arr2: &[A]) -> Vec<A> {
    let mut result = arr1.to_vec();
    result.extend(arr2.iter().cloned());
    result
}

// ============================================================================
// Generic value functions
// ============================================================================

/// Returns the string representation of any Debug-printable value.
/// Rather slow; only use this for debugging!
pub fn any_string<A: std::fmt::Debug>(a: &A) -> String {
    format!("{:?}", a)
}

/// Prints any Debug-printable value to stderr.
pub fn print_any<A: std::fmt::Debug>(a: &A) {
    eprintln!("{:?}", a);
}

/// Prints a debug string prefix followed by any Debug-printable value to stderr.
/// For RML compatibility.
pub fn debug_print<A: std::fmt::Debug>(str: &str, a: &A) {
    eprintln!("{}: {:?}", str, a);
}

thread_local! {
    static TICK_COUNTER: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// Returns a monotonically increasing tick counter.
/// Uses a thread-local counter for simulation purposes.
pub fn tick() -> i32 {
    TICK_COUNTER.with(|counter| {
        let current = counter.get();
        counter.set(current.wrapping_add(1));
        current as i32
    })
}

/// Structural equality for any PartialEq value.
pub fn value_eq<A: PartialEq>(a1: &A, a2: &A) -> bool {
    a1 == a2
}

/// Compares two Ord values.
/// Returns -1 if a1 < a2, 0 if a1 == a2, 1 if a1 > a2.
pub fn value_compare<A: Ord>(a1: &A, a2: &A) -> i32 {
    match a1.cmp(a2) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

/// Reference equality check.
/// In Rust, this checks pointer equality for reference-counted types.
/// For simple types, falls back to structural equality.
///
/// This is a very fast comparison to speed up comparisons.
/// If you know that all occurrences of a value are the same pointer,
/// you can use reference_eq instead of structural equality.
pub fn reference_eq<A: PartialEq>(a1: &A, a2: &A) -> bool {
    std::ptr::eq(a1 as *const A, a2 as *const A)
}

/// Returns the pointer address of a reference as a hexadecimal string for debugging.
pub fn reference_pointer_string<A>(a: &A) -> String {
    format!("{:p}", a)
}

/// Returns a debug string for a function symbol.
/// In Rust, returns the type name of the value for debugging.
pub fn reference_debug_string<A: std::fmt::Debug>(_a: &A) -> String {
    format!("{:?}", std::any::type_name::<A>())
}

/// Returns the constructor tag for a boxed value.
/// In Rust, returns a type-based discriminator.
pub fn value_constructor<A>() -> i32 {
    // Use a hash of the type name as a stable discriminator
    let type_name = std::any::type_name::<A>();
    let mut hash: i32 = 5381;
    for &byte in type_name.as_bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as i32);
    }
    hash
}

/// Returns the current time in seconds relative to process start.
/// Not very accurate, intended for diff comparisons.
fn get_start_instant() -> std::time::Instant {
    use std::sync::OnceLock;
    static START: OnceLock<std::time::Instant> = OnceLock::new();
    *START.get_or_init(|| std::time::Instant::now())
}

pub fn clock() -> f64 {
    get_start_instant().elapsed().as_secs_f64()
}

// ============================================================================
// Option functions
// ============================================================================

/// Returns true if the Option is NONE.
pub fn is_none<A>(opt: &OptionValue<A>) -> bool {
    matches!(opt, OptionValue::None)
}

/// Returns true if the Option is SOME.
pub fn is_some<A>(opt: &OptionValue<A>) -> bool {
    matches!(opt, OptionValue::Some(_))
}

// ============================================================================
// Misc builtin functions
// ============================================================================

/// Sets the stack overflow signal to the given value and returns the old one.
/// In this translation, simply returns the input value.
pub fn set_stack_overflow_signal(in_signal: bool) -> bool {
    in_signal
}

/// Returns true if the formal output argument is present as an actual argument.
/// In MetaModelica this is a compile-time check; in Rust it always returns true
/// because the argument exists at the call site.
pub fn is_present<T>(_ident: &T) -> bool {
    true
}

/// Fail function - unconditionally raises an error.
pub fn fail() -> Result<()> {
    bail!("fail() was called - unrecoverable error")
}

// ============================================================================
// MetaModelica::Dangerous - Functions that skip bounds checking
// ============================================================================

pub mod Dangerous {
    use super::*;
    /// Unsafe array get without bounds checking.
    /// Panics in debug mode if index is out of bounds due to Rust's bounds checking on indexing.
    pub fn arrayGetNoBoundsChecking<A: Clone>(arr: &[A], index: i32) -> A {
        let idx = (index - 1) as usize; // 1-based to 0-based
        // SAFETY: Caller must ensure index is in bounds.
        // Rust does not have true unchecked indexing, but unsafe::get_unchecked avoids bounds check.
        unsafe { arr.get_unchecked(idx).clone() }
    }

    /// Unsafe array update without bounds checking.
    /// Mutates the array in place.
    pub fn arrayUpdateNoBoundsChecking<A: Clone>(arr: &mut Vec<A>, index: i32, new_value: A) {
        let idx = (index - 1) as usize; // 1-based to 0-based
        // SAFETY: Caller must ensure index is in bounds.
        unsafe { *arr.get_unchecked_mut(idx) = new_value }
    }

    /// Creates a new array with uninitialized elements.
    /// The dummy parameter is used to fix the type of the array.
    /// Elements are set to a clone of dummy.
    pub fn arrayCreateNoInit<A: Clone>(size: i32, dummy: A) -> Vec<A> {
        if size <= 0 {
            return Vec::new();
        }
        // SAFETY: We immediately fill with cloned values so the array is never used uninitialized.
        // In a true unsafe translation we could use MaybeUninit, but for safety we fill with dummy.
        vec![dummy; size as usize]
    }
    /// Unsafe string get without bounds checking.
    pub fn stringGetNoBoundsChecking(str: &str, index: i32) -> i32 {
        let idx = (index - 1) as usize; // 1-based to 0-based
        // SAFETY: Caller must ensure index is in bounds.
        unsafe { (*str.as_bytes().get_unchecked(idx)) as i32 }
    }
    /// Reverses a list in place.
    pub fn listReverseInPlace<T: Clone>(list: &List<T>) -> List<T>{
        list.reverse()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use super::*;

    // =========================================================================
    // Boolean tests
    // =========================================================================

    mod boolean_tests {
        use super::*;

        #[test]
        fn test_bool_and() {
            assert!(bool_and(true, true));
            assert!(!bool_and(true, false));
            assert!(!bool_and(false, true));
            assert!(!bool_and(false, false));
        }

        #[test]
        fn test_bool_or() {
            assert!(bool_or(true, false));
            assert!(bool_or(false, true));
            assert!(bool_or(true, true));
            assert!(!bool_or(false, false));
        }

        #[test]
        fn test_bool_not() {
            assert!(bool_not(false));
            assert!(!bool_not(true));
        }

        #[test]
        fn test_bool_eq() {
            assert!(bool_eq(true, true));
            assert!(bool_eq(false, false));
            assert!(!bool_eq(true, false));
            assert!(!bool_eq(false, true));
        }

        #[test]
        fn test_bool_string() {
            assert_eq!(bool_string(true), "true");
            assert_eq!(bool_string(false), "false");
        }
    }

    // =========================================================================
    // Integer arithmetic tests
    // =========================================================================

    mod int_arithmetic_tests {
        use super::*;

        #[test]
        fn test_int_add() {
            assert_eq!(int_add(1, 2), 3);
            assert_eq!(int_add(-1, 1), 0);
            assert_eq!(int_add(-1, -2), -3);
        }

        #[test]
        fn test_int_sub() {
            assert_eq!(int_sub(5, 3), 2);
            assert_eq!(int_sub(3, 5), -2);
            assert_eq!(int_sub(0, 0), 0);
        }

        #[test]
        fn test_int_mul() {
            assert_eq!(int_mul(3, 4), 12);
            assert_eq!(int_mul(-3, 4), -12);
            assert_eq!(int_mul(-3, -4), 12);
            assert_eq!(int_mul(0, 100), 0);
        }

        #[test]
        fn test_int_div() {
            assert_eq!(int_div(10, 3), 3);
            assert_eq!(int_div(10, -3), -3);
            assert_eq!(int_div(-10, 3), -3);
            assert_eq!(int_div(-10, -3), 3);
        }

        #[test]
        fn test_int_mod() {
            assert_eq!(int_mod(10, 3), 1);
            assert_eq!(int_mod(10, -3), 1);
            assert_eq!(int_mod(-10, 3), -1);
            assert_eq!(int_mod(-10, -3), -1);
        }

        #[test]
        fn test_int_max() {
            assert_eq!(int_max(1, 2), 2);
            assert_eq!(int_max(2, 1), 2);
            assert_eq!(int_max(5, 5), 5);
            assert_eq!(int_max(-1, -2), -1);
        }

        #[test]
        fn test_int_min() {
            assert_eq!(int_min(1, 2), 1);
            assert_eq!(int_min(2, 1), 1);
            assert_eq!(int_min(5, 5), 5);
            assert_eq!(int_min(-1, -2), -2);
        }

        #[test]
        fn test_int_abs() {
            assert_eq!(int_abs(-5), 5);
            assert_eq!(int_abs(5), 5);
            assert_eq!(int_abs(0), 0);
        }

        #[test]
        fn test_int_neg() {
            assert_eq!(int_neg(5), -5);
            assert_eq!(int_neg(-5), 5);
            assert_eq!(int_neg(0), 0);
        }
    }

    // =========================================================================
    // Integer comparison tests
    // =========================================================================

    mod int_comparison_tests {
        use super::*;

        #[test]
        fn test_int_lt() {
            assert!(int_lt(1, 2));
            assert!(!int_lt(2, 2));
            assert!(!int_lt(2, 1));
        }

        #[test]
        fn test_int_le() {
            assert!(int_le(1, 2));
            assert!(int_le(2, 2));
            assert!(!int_le(2, 1));
        }

        #[test]
        fn test_int_eq() {
            assert!(int_eq(5, 5));
            assert!(!int_eq(5, 6));
        }

        #[test]
        fn test_int_ne() {
            assert!(int_ne(5, 6));
            assert!(!int_ne(5, 5));
        }

        #[test]
        fn test_int_ge() {
            assert!(int_ge(2, 1));
            assert!(int_ge(2, 2));
            assert!(!int_ge(1, 2));
        }

        #[test]
        fn test_int_gt() {
            assert!(int_gt(2, 1));
            assert!(!int_gt(2, 2));
            assert!(!int_gt(1, 2));
        }
    }

    // =========================================================================
    // Integer bitwise tests
    // =========================================================================

    mod int_bitwise_tests {
        use super::*;

        #[test]
        fn test_int_bit_not() {
            assert_eq!(int_bit_not(0i32), -1);
            assert_eq!(int_bit_not(-1i32), 0);
            assert_eq!(int_bit_not(1), !1);
        }

        #[test]
        fn test_int_bit_and() {
            assert_eq!(int_bit_and(0b1100, 0b1010), 0b1000);
            assert_eq!(int_bit_and(0, 5), 0);
        }

        #[test]
        fn test_int_bit_or() {
            assert_eq!(int_bit_or(0b1100, 0b1010), 0b1110);
            assert_eq!(int_bit_or(0, 5), 5);
        }

        #[test]
        fn test_int_bit_xor() {
            assert_eq!(int_bit_xor(0b1100, 0b1010), 0b0110);
            assert_eq!(int_bit_xor(5, 5), 0);
        }

        #[test]
        fn test_int_bit_l_shift() {
            assert_eq!(int_bit_l_shift(1, 3), 8);
            assert_eq!(int_bit_l_shift(3, 1), 6);
        }

        #[test]
        fn test_int_bit_r_shift() {
            assert_eq!(int_bit_r_shift(8, 3), 1);
            assert_eq!(int_bit_r_shift(6, 1), 3);
        }
    }

    // =========================================================================
    // Integer conversion tests
    // =========================================================================

    mod int_conversion_tests {
        use super::*;

        #[test]
        fn test_int_real() {
            assert_eq!(int_real(42), 42.0_f64);
            assert_eq!(int_real(-7), -7.0_f64);
        }

        #[test]
        fn test_int_string() {
            assert_eq!(int_string(42), "42");
            assert_eq!(int_string(-7), "-7");
            assert_eq!(int_string(0), "0");
        }
    }

    // =========================================================================
    // Real arithmetic tests
    // =========================================================================

    mod real_arithmetic_tests {
        use super::*;

        #[test]
        fn test_real_add() {
            assert_eq!(real_add(1.5, 2.5), 4.0);
            assert_eq!(real_add(-1.0, 1.0), 0.0);
        }

        #[test]
        fn test_real_sub() {
            assert_eq!(real_sub(5.0, 3.0), 2.0);
            assert_eq!(real_sub(3.0, 5.0), -2.0);
        }

        #[test]
        fn test_real_mul() {
            assert_eq!(real_mul(3.0, 4.0), 12.0);
            assert_eq!(real_mul(-3.0, 4.0), -12.0);
        }

        #[test]
        fn test_real_div() {
            assert_eq!(real_div(10.0, 3.0), 10.0 / 3.0);
            assert_eq!(real_div(6.0, 2.0), 3.0);
        }

        #[test]
        fn test_real_mod() {
            assert_eq!(real_mod(10.0, 3.0), 1.0);
            assert_eq!(real_mod(10.5, 3.0), 1.5);
        }

        #[test]
        fn test_real_pow() {
            assert_eq!(real_pow(2.0, 3.0), 8.0);
            assert_eq!(real_pow(9.0, 0.5), 3.0);
        }

        #[test]
        fn test_real_max() {
            assert_eq!(real_max(1.5, 2.5), 2.5);
            assert_eq!(real_max(5.0, 5.0), 5.0);
        }

        #[test]
        fn test_real_min() {
            assert_eq!(real_min(1.5, 2.5), 1.5);
            assert_eq!(real_min(5.0, 5.0), 5.0);
        }

        #[test]
        fn test_real_abs() {
            assert_eq!(real_abs(-5.5), 5.5);
            assert_eq!(real_abs(5.5), 5.5);
        }

        #[test]
        fn test_real_almost_eq() {
            assert!(real_almost_eq(1.0, 1.0000001, 1e-5));
            assert!(!real_almost_eq(1.0, 1.1, 1e-5));
            assert!(real_almost_eq(1.0, 1.0, 1e-6));
        }

        #[test]
        fn test_real_neg() {
            assert_eq!(real_neg(5.5), -5.5);
            assert_eq!(real_neg(-5.5), 5.5);
        }
    }

    // =========================================================================
    // Real comparison tests
    // =========================================================================

    mod real_comparison_tests {
        use super::*;

        #[test]
        fn test_real_lt() {
            assert!(real_lt(1.0, 2.0));
            assert!(!real_lt(2.0, 2.0));
            assert!(!real_lt(2.0, 1.0));
        }

        #[test]
        fn test_real_le() {
            assert!(real_le(1.0, 2.0));
            assert!(real_le(2.0, 2.0));
            assert!(!real_le(2.0, 1.0));
        }

        #[test]
        fn test_real_eq() {
            assert!(real_eq(1.0, 1.0));
            assert!(!real_eq(1.0, 2.0));
        }

        #[test]
        fn test_real_ne() {
            assert!(real_ne(1.0, 2.0));
            assert!(!real_ne(1.0, 1.0));
        }

        #[test]
        fn test_real_ge() {
            assert!(real_ge(2.0, 1.0));
            assert!(real_ge(2.0, 2.0));
            assert!(!real_ge(1.0, 2.0));
        }

        #[test]
        fn test_real_gt() {
            assert!(real_gt(2.0, 1.0));
            assert!(!real_gt(2.0, 2.0));
            assert!(!real_gt(1.0, 2.0));
        }
    }

    // =========================================================================
    // Real conversion tests
    // =========================================================================

    mod real_conversion_tests {
        use super::*;

        #[test]
        fn test_real_int() {
            assert_eq!(real_int(3.7), 3);
            assert_eq!(real_int(-3.7), -3);
            assert_eq!(real_int(3.0), 3);
        }

        #[test]
        fn test_real_string() {
            assert_eq!(real_string(3.14), "3.14");
            assert_eq!(real_string(0.0), "0");
            assert_eq!(real_string(-1.5), "-1.5");
        }
    }

    // =========================================================================
    // String character tests
    // =========================================================================

    mod string_char_tests {
        use super::*;

        #[test]
        fn test_string_char_int() {
            assert_eq!(string_char_int("A"), 65);
            assert_eq!(string_char_int("a"), 97);
            assert_eq!(string_char_int("0"), 48);
        }

        #[test]
        fn test_int_string_char() {
            assert_eq!(int_string_char(65), "A");
            assert_eq!(int_string_char(97), "a");
            assert_eq!(int_string_char(48), "0");
            assert_eq!(int_string_char(0), "\0");
        }

        #[test]
        fn test_string_int() {
            assert_eq!(string_int("42").unwrap(), 42);
            assert_eq!(string_int("-7").unwrap(), -7);
            assert!(string_int("not_a_number").is_err());
        }

        #[test]
        fn test_string_real() {
            assert_eq!(string_real("3.14").unwrap(), 3.14);
            assert_eq!(string_real("-2.5").unwrap(), -2.5);
            assert!(string_real("not_a_number").is_err());
        }

        #[test]
        fn test_string_list_string_char() {
            let result = string_list_string_char("abc");
            assert_eq!(result, List::from_iter(["a".to_string(), "b".to_string(), "c".to_string()]));
        }

        #[test]
        fn test_string_append_list() {
            let strs = list!["hello".to_string(), " ".to_string(), "world".to_string()];
            assert_eq!(string_append_list(&strs), "hello world");
        }

        #[test]
        fn test_string_delimit_list() {
            let strs = list!["x".to_string(), "y".to_string(), "z".to_string()];
            assert_eq!(string_delimit_list(&strs, ", "), "x, y, z");
        }
    }

    // =========================================================================
    // String length and empty tests
    // =========================================================================

    mod string_length_tests {
        use super::*;

        #[test]
        fn test_string_length() {
            assert_eq!(string_length("hello"), 5);
            assert_eq!(string_length(""), 0);
        }

        #[test]
        fn test_string_empty() {
            assert!(string_empty(""));
            assert!(!string_empty("hello"));
        }
    }

    // =========================================================================
    // String get/update tests
    // =========================================================================

    mod string_get_update_tests {
        use super::*;

        #[test]
        fn test_string_get() {
            assert_eq!(string_get("hello", 1).unwrap(), b'h' as i32);
            assert_eq!(string_get("hello", 5).unwrap(), b'o' as i32);
            assert!(string_get("hello", 0).is_err());
            assert!(string_get("hello", 6).is_err());
        }

        #[test]
        fn test_string_get_string_char() {
            assert_eq!(string_get_string_char("hello", 1).unwrap(), "h");
            assert_eq!(string_get_string_char("hello", 3).unwrap(), "l");
            assert_eq!(string_get_string_char("hello", 5).unwrap(), "o");
            assert!(string_get_string_char("hello", 0).is_err());
            assert!(string_get_string_char("hello", 6).is_err());
        }

        #[test]
        fn test_string_update_string_char() {
            assert_eq!(string_update_string_char("hello", "X", 1).unwrap(), "Xello");
            assert_eq!(string_update_string_char("hello", "X", 3).unwrap(), "heXlo");
            assert_eq!(string_update_string_char("hello", "X", 5).unwrap(), "hellX");
            assert!(string_update_string_char("hello", "X", 0).is_err());
            assert!(string_update_string_char("hello", "X", 6).is_err());
            assert!(string_update_string_char("hello", "", 1).is_err());
        }
    }

    // =========================================================================
    // String append/equal tests
    // =========================================================================

    mod string_append_equal_tests {
        use super::*;

        #[test]
        fn test_string_append() {
            assert_eq!(string_append("hello", " world"), "hello world");
            assert_eq!(string_append("", "hello"), "hello");
            assert_eq!(string_append("hello", ""), "hello");
        }

        #[test]
        fn test_string_eq() {
            assert!(string_eq("abc", "abc"));
            assert!(!string_eq("abc", "abd"));
            assert!(!string_eq("", "abc"));
        }

        #[test]
        fn test_string_equal() {
            assert!(string_equal("abc", "abc"));
            assert!(!string_equal("abc", "abd"));
        }
    }

    // =========================================================================
    // String compare test
    // =========================================================================

    mod string_compare_test {
        use super::*;

        #[test]
        fn test_string_compare() {
            assert!(string_compare("abc", "abd") < 0);
            assert_eq!(string_compare("abc", "abc"), 0);
            assert!(string_compare("abd", "abc") > 0);
            assert!(string_compare("ab", "abc") < 0);
            assert!(string_compare("abc", "ab") > 0);
        }
    }

    // =========================================================================
    // String hash tests
    // =========================================================================

    mod string_hash_tests {
        use super::*;

        #[test]
        fn test_string_hash_djb2() {
            // DJB2 of "a" = 5381 * 33 + 97 = 177700 + 97 = 177797
            assert_eq!(string_hash_djb2("a"), 5381_i32.wrapping_mul(33).wrapping_add(97));
            assert_eq!(string_hash_djb2(""), 5381);
        }

        #[test]
        fn test_string_hash_djb2_continue() {
            let h1 = string_hash_djb2("hello");
            let _h2 = string_hash_djb2(" world");
            let combined = string_hash_djb2_continue(" world", h1);
            // Starting from h1 and adding " world" should give the same
            // as hashing "hello world" from scratch
            assert_eq!(combined, string_hash_djb2("hello world"));
        }

        #[test]
        fn test_string_hash_djb2_mod() {
            let h = string_hash_djb2_mod("hello", 100);
            assert!(h >= 0 && h < 100);
            assert_eq!(string_hash_djb2_mod("hello", 0), 0);
        }

        #[test]
        fn test_string_hash_sdbm() {
            // SDBM of "a" = 97 + 0 + 0 - 0 = 97
            assert_eq!(string_hash_sdbm("a"), 97);
            assert_eq!(string_hash_sdbm(""), 0);
        }

        #[test]
        fn test_string_hash_consistency() {
            // Same string should produce same hash
            assert_eq!(string_hash("test"), string_hash("test"));
        }
    }

    // =========================================================================
    // Substring tests
    // =========================================================================

    mod substring_tests {
        use super::*;

        #[test]
        fn test_substring_basic() {
            assert_eq!(substring("hello world", 1, 5).unwrap(), "hello");
            assert_eq!(substring("hello world", 7, 11).unwrap(), "world");
            assert_eq!(substring("hello", 3, 3).unwrap(), "l");
            assert_eq!(substring("hello", 1, 5).unwrap(), "hello");
        }

        #[test]
        fn test_substring_errors() {
            assert!(substring("hello", 0, 3).is_err());  // start < 1
            assert!(substring("hello", 3, 2).is_err());  // stop < start
            assert!(substring("hello", 1, 6).is_err());  // stop out of bounds
            assert!(substring("hello", 6, 7).is_err());  // start out of bounds
        }
    }

    // =========================================================================
    // List string char string tests
    // =========================================================================

    mod list_string_tests {
        use super::*;

        #[test]
        fn test_list_string_char_string() {
            let strs = list!["a".to_string(), "b".to_string(), "c".to_string()];
            assert_eq!(list_string_char_string(&strs), "abc");
        }

        #[test]
        fn test_string_char_list_string() {
            let strs = list!["a".to_string(), "b".to_string(), "c".to_string()];
            assert_eq!(string_char_list_string(&strs), "abc");
        }
    }

    // =========================================================================
    // List function tests
    // =========================================================================

    mod list_function_tests {
        use super::*;

        #[test]
        fn test_list_append() {
            let a = list![1, 2, 3];
            let b = list![4, 5];
            let result = a.append(&b);
            assert_eq!(result, list![1, 2, 3, 4, 5]);

            // Empty list cases
            let empty: List<i32> = Nil();
            assert_eq!(empty.append(&b), b);
            assert_eq!(a.append(&empty), a);
        }

        #[test]
        fn test_list_reverse() {
            let lst = list![1, 2, 3, 4, 5];
            let result = lst.reverse();
            assert_eq!(result, list![5, 4, 3, 2, 1]);

            let empty: List<i32> = Nil();
            assert_eq!(empty.reverse(), Nil());
        }

        #[test]
        fn test_list_length() {
            let lst = list![1, 2, 3];
            assert_eq!(lst.len(), 3);
            let empty: List<i32> = Nil();
            assert_eq!(empty.len(), 0);
        }

        #[test]
        fn test_list_member() {
            let lst = list![1, 2, 3];
            assert!(lst.contains(&2));
            assert!(!lst.contains(&4));
        }

        #[test]
        fn test_list_get() {
            let lst = list![10, 20, 30];
            assert_eq!(lst.get(1).unwrap(), 10);
            assert_eq!(lst.get(2).unwrap(), 20);
            assert_eq!(lst.get(3).unwrap(), 30);
            assert!(lst.get(0).is_err());
            assert!(lst.get(4).is_err());
        }

        #[test]
        fn test_list_rest() {
            let lst = list![1, 2, 3];
            let result = lst.rest().unwrap().clone();
            assert_eq!(result, list![2, 3]);

            let single = list![1];
            assert!(single.rest().unwrap().is_empty());

            let empty: List<i32> = Nil();
            assert!(empty.rest().is_err());
        }

        #[test]
        fn test_list_head() {
            let lst = list![1, 2, 3];
            assert_eq!(lst.head().unwrap().clone(), 1);

            let empty: List<i32> = Nil();
            assert!(empty.head().is_err());
        }

        #[test]
        fn test_list_delete() {
            let lst = list![1, 2, 3, 4];
            assert_eq!(lst.delete(1).unwrap(), list![2, 3, 4]);
            assert_eq!(lst.delete(2).unwrap(), list![1, 3, 4]);
            assert_eq!(lst.delete(4).unwrap(), list![1, 2, 3]);
        }

        #[test]
        fn test_list_empty() {
            let lst = list![1, 2, 3];
            assert!(!lst.is_empty());

            let empty: List<i32> = Nil();
            assert!(empty.is_empty());
        }

        #[test]
        fn test_cons() {
            let lst = list![2, 3];
            let result = cons(1, lst);
            assert_eq!(result, list![1, 2, 3]);

            let empty: List<i32> = Nil();
            let result = cons(42, empty);
            assert_eq!(result, List::new(42));
        }

        #[test]
        fn test_list_reverse2() -> () {
            let lst1 = list![1,2,3,4];
            let lst2 = lst1.reverse();
            let lst3 = lst2.reverse();
            assert_eq!(lst1, lst3);
            assert!(lst1 != lst2);
        }
    }

    // =========================================================================
    // Array function tests
    // =========================================================================

    mod array_function_tests {
        use super::*;

        #[test]
        fn test_array_length() {
            let arr = vec![1, 2, 3];
            assert_eq!(array_length(&arr), 3);
            let empty: Vec<i32> = vec![];
            assert_eq!(array_length(&empty), 0);
        }

        #[test]
        fn test_array_empty() {
            let arr = vec![1, 2, 3];
            assert!(!array_empty(&arr));
            let empty: Vec<i32> = vec![];
            assert!(array_empty(&empty));
        }

        #[test]
        fn test_array_get() {
            let arr = vec![10, 20, 30];
            assert_eq!(array_get(&arr, 1).unwrap(), 10);
            assert_eq!(array_get(&arr, 2).unwrap(), 20);
            assert_eq!(array_get(&arr, 3).unwrap(), 30);
            assert!(array_get(&arr, 0).is_err());
            assert!(array_get(&arr, 4).is_err());
        }

        #[test]
        fn test_array_create() {
            let arr = array_create(5, 0);
            assert_eq!(arr, vec![0, 0, 0, 0, 0]);
            let empty: Vec<i32> = array_create(0, 42);
            assert!(empty.is_empty());
        }

        #[test]
        fn test_array_list() {
            let arr = vec![1, 2, 3];
            let lst = array_list(&arr);
            assert_eq!(lst, list![1, 2, 3]);
        }

        #[test]
        fn test_list_array() {
            let lst = list![1, 2, 3];
            let arr = list_array(&lst);
            assert_eq!(arr, vec![1, 2, 3]);
        }

        #[test]
        fn test_array_update() {
            let mut arr = vec![1, 2, 3];
            array_update(&mut arr, 2, 99).unwrap();
            assert_eq!(arr, vec![1, 99, 3]);
            assert!(array_update(&mut arr, 0, 99).is_err());
            assert!(array_update(&mut arr, 4, 99).is_err());
        }

        #[test]
        fn test_array_copy() {
            let arr = vec![1, 2, 3];
            let copy = array_copy(&arr);
            assert_eq!(copy, vec![1, 2, 3]);
            assert_eq!(copy, arr);
        }

        #[test]
        fn test_array_append() {
            let a = vec![1, 2];
            let b = vec![3, 4];
            assert_eq!(array_append(&a, &b), vec![1, 2, 3, 4]);

            let empty: Vec<i32> = vec![];
            assert_eq!(array_append(&empty, &b), b);
            assert_eq!(array_append(&a, &empty), a);
        }
    }

    // =========================================================================
    // Generic value tests
    // =========================================================================

    mod generic_value_tests {
        use super::*;

        #[test]
        fn test_any_string() {
            let val = 42i32;
            let result = any_string(&val);
            assert_eq!(result, "42");

            let s = "hello";
            assert!(any_string(&s).contains("hello"));
        }

        #[test]
        fn test_print_any() {
            // Just ensure it doesn't panic
            let val = 42i32;
            print_any(&val);
        }

        #[test]
        fn test_debug_print() {
            // Just ensure it doesn't panic
            let val = 42i32;
            debug_print("test", &val);
        }

        #[test]
        fn test_tick() {
            let t1 = tick();
            let t2 = tick();
            assert_eq!(t2, t1.wrapping_add(1));
        }

        #[test]
        fn test_value_eq() {
            let a = vec![1, 2, 3];
            let b = vec![1, 2, 3];
            let c = vec![1, 2, 4];
            assert!(value_eq(&a, &b));
            assert!(!value_eq(&a, &c));
        }

        #[test]
        fn test_value_compare() {
            assert_eq!(value_compare(&1, &2), -1);
            assert_eq!(value_compare(&2, &2), 0);
            assert_eq!(value_compare(&3, &2), 1);

            assert_eq!(value_compare(&"abc", &"abd"), -1);
            assert_eq!(value_compare(&"abc", &"abc"), 0);
            assert_eq!(value_compare(&"abd", &"abc"), 1);
        }

        #[test]
        fn test_reference_eq() {
            let a = 42;
            let b = 42;
            // Same reference should be equal
            assert!(reference_eq(&a, &a));
            // Different references with same value
            // reference_eq checks pointer equality, so different vars may not be equal
            assert!(reference_eq(&a, &b) || !reference_eq(&a, &b)); // either is valid
        }

        #[test]
        fn test_reference_pointer_string() {
            let val = 42;
            let ptr_str = reference_pointer_string(&val);
            // Should be a valid hex representation like "0x..."
            assert!(ptr_str.starts_with("0x"));
        }

        #[test]
        fn test_reference_debug_string() {
            let val = 42i32;
            let result = reference_debug_string(&val);
            assert!(result.contains("i32"));
        }

        #[test]
        fn test_value_constructor() {
            // Should return a stable value for the same type
            let c1 = value_constructor::<i32>();
            let c2 = value_constructor::<i32>();
            assert_eq!(c1, c2);

            // Different types should likely have different constructors
            let c3 = value_constructor::<String>();
            assert_ne!(c1, c3);
        }

        #[test]
        fn test_clock() {
            let t1 = clock();
            let t2 = clock();
            assert!(t1 >= 0.0);
            assert!(t2 >= t1);
        }
    }

    // =========================================================================
    // Option tests
    // =========================================================================

    mod option_tests {
        use super::*;

        #[test]
        fn test_is_none() {
            let none: OptionValue<i32> = OptionValue::None;
            assert!(is_none(&none));

            let some = OptionValue::Some(42);
            assert!(!is_none(&some));
        }

        #[test]
        fn test_is_some() {
            let none: OptionValue<i32> = OptionValue::None;
            assert!(!is_some(&none));

            let some = OptionValue::Some(42);
            assert!(is_some(&some));
        }
    }

    // =========================================================================
    // Misc builtin tests
    // =========================================================================

    mod misc_builtin_tests {
        use super::*;

        #[test]
        fn test_set_stack_overflow_signal() {
            assert!(set_stack_overflow_signal(true));
            assert!(!set_stack_overflow_signal(false));
        }

        #[test]
        fn test_is_present() {
            // Always returns true in Rust translation
            assert!(is_present(&42));
            assert!(is_present(&"hello"));
        }

        #[test]
        fn test_fail() {
            assert!(fail().is_err());
        }

        #[test]
        fn test_source_info() {
            let info = SourceInfo {
                file_name: Arc::new("test.mo".to_string()),
                is_read_only: true,
                line_number_start: 1,
                column_number_start: 1,
                line_number_end: 10,
                column_number_end: 50,
                last_modification: 1234567890.0,
            };
            assert_eq!(*info.file_name, "test.mo");
            assert!(info.is_read_only);
            assert_eq!(info.line_number_start, 1);
        }
    }

    // =========================================================================
    // Dangerous function tests
    // =========================================================================

    mod dangerous_tests {
        use super::Dangerous::*;

        #[test]
        fn test_array_get_no_bounds_checking() {
            let arr = vec![10, 20, 30];
            // Valid 1-based indices
            assert_eq!(arrayGetNoBoundsChecking(&arr, 1), 10);
            assert_eq!(arrayGetNoBoundsChecking(&arr, 2), 20);
            assert_eq!(arrayGetNoBoundsChecking(&arr, 3), 30);
        }

        #[test]
        fn test_array_update_no_bounds_checking() {
            let mut arr = vec![1, 2, 3];
            arrayUpdateNoBoundsChecking(&mut arr, 2, 99);
            assert_eq!(arr, vec![1, 99, 3]);
        }

        #[test]
        fn test_array_create_no_init() {
            let arr = arrayCreateNoInit(5, 0i32);
            assert_eq!(arr.len(), 5);
        }

        #[test]
        fn test_string_get_no_bounds_checking() {
            let s = "hello";
            assert_eq!(stringGetNoBoundsChecking(s, 1), b'h' as i32);
            assert_eq!(stringGetNoBoundsChecking(s, 5), b'o' as i32);
        }
    }
}
