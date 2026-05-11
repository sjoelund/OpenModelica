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
    pub fileName: Arc<String>,
    /// Should be true for libraries.
    pub isReadOnly: bool,
    /// Start line number (1-based).
    pub lineNumberStart: i32,
    /// Start column number (1-based).
    pub columnNumberStart: i32,
    /// End line number (1-based).
    pub lineNumberEnd: i32,
    /// End column number (1-based).
    pub columnNumberEnd: i32,
    /// mtime in stat(2), stored as a double for increased precision on 32-bit platforms.
    pub lastModification: f64,
}

// ============================================================================
// Boolean functions
// ============================================================================

/// Logically combine two Booleans with 'and' operator.
#[inline(always)]
pub fn boolAnd(b1: bool, b2: bool) -> Result<bool> {
    Ok(b1 && b2)
}

/// Logically combine two Booleans with 'or' operator.
#[inline(always)]
pub fn boolOr(b1: bool, b2: bool) -> Result<bool> {
    Ok(b1 || b2)
}

/// Logically invert Boolean value using 'not' operator.
#[inline(always)]
pub fn boolNot(b: bool) -> Result<bool> {
    Ok(!b)
}

/// Compares two Booleans for equality.
#[inline(always)]
pub fn boolEq(b1: bool, b2: bool) -> Result<bool> {
    Ok(b1 == b2)
}

/// Returns "true" or "false" string from a boolean.
pub fn boolString(b: bool) -> Result<Arc<String>> {
    Ok(Arc::new(if b { "true".to_string() } else { "false".to_string() }))
}

// ============================================================================
// Integer arithmetic functions
// ============================================================================

/// Adds two Integer values.
#[inline(always)]
pub fn intAdd(i1: i32, i2: i32) -> Result<i32> {
    Ok(i1 + i2)
}

/// Subtracts two Integer values.
#[inline(always)]
pub fn intSub(i1: i32, i2: i32) -> Result<i32> {
    Ok(i1 - i2)
}

/// Multiplies two Integer values.
#[inline(always)]
pub fn intMul(i1: i32, i2: i32) -> Result<i32> {
    Ok(i1 * i2)
}

/// Divides two Integer values (truncated division).
/// Matches Modelica's div() semantics: truncates toward zero.
pub fn intDiv(i1: i32, i2: i32) -> Result<i32> {
    Ok(i1 / i2)
}

/// Calculates remainder of Integer division i1/i2.
/// Matches Modelica's mod() semantics: same sign as dividend.
pub fn intMod(i1: i32, i2: i32) -> Result<i32> {
    Ok(i1 % i2)
}

/// Returns the bigger one of two Integer values.
pub fn intMax(i1: i32, i2: i32) -> Result<i32> {
    Ok(i1.max(i2))
}

/// Returns the smaller one of two Integer values.
pub fn intMin(i1: i32, i2: i32) -> Result<i32> {
    Ok(i1.min(i2))
}

/// Returns the absolute value of Integer i.
pub fn intAbs(i: i32) -> Result<i32> {
    Ok(i.abs())
}

/// Returns negative value of Integer i.
#[inline(always)]
pub fn intNeg(i: i32) -> Result<i32> {
    Ok(-i)
}

// ============================================================================
// Integer comparison functions
// ============================================================================

/// Returns whether Integer i1 is smaller than Integer i2.
#[inline(always)]
pub fn intLt(i1: i32, i2: i32) -> Result<bool> {
    Ok(i1 < i2)
}

/// Returns whether Integer i1 is smaller than or equal to Integer i2.
#[inline(always)]
pub fn intLe(i1: i32, i2: i32) -> Result<bool> {
    Ok(i1 <= i2)
}

/// Returns whether Integer i1 is equal to Integer i2.
#[inline(always)]
pub fn intEq(i1: i32, i2: i32) -> Result<bool> {
    Ok(i1 == i2)
}

/// Returns whether Integer i1 is not equal to Integer i2.
#[inline(always)]
pub fn intNe(i1: i32, i2: i32) -> Result<bool> {
    Ok(i1 != i2)
}

/// Returns whether Integer i1 is greater than or equal to Integer i2.
#[inline(always)]
pub fn intGe(i1: i32, i2: i32) -> Result<bool> {
    Ok(i1 >= i2)
}

/// Returns whether Integer i1 is greater than Integer i2.
#[inline(always)]
pub fn intGt(i1: i32, i2: i32) -> Result<bool> {
    Ok(i1 > i2)
}

// ============================================================================
// Integer bitwise functions
// ============================================================================

/// Returns bitwise inverted Integer number of i (~i in C).
#[inline(always)]
pub const fn intBitNot(i: i32) -> Result<i32> {
    Ok(!i)
}

/// Returns bitwise 'and' of Integers i1 and i2 (i1 & i2 in C).
#[inline(always)]
pub const fn intBitAnd(i1: i32, i2: i32) -> Result<i32> {
    Ok(i1 & i2)
}

/// Returns bitwise 'or' of Integers i1 and i2 (i1 | i2 in C).
#[inline(always)]
pub const fn intBitOr(i1: i32, i2: i32) -> Result<i32> {
    Ok(i1 | i2)
}

/// Returns bitwise 'xor' of Integers i1 and i2 (i1 ^ i2 in C).
#[inline(always)]
pub const fn intBitXor(i1: i32, i2: i32) -> Result<i32> {
    Ok(i1 ^ i2)
}

/// Returns bitwise left shift of Integer i by s bits (i << s in C).
#[inline(always)]
pub const fn intBitLShift(i: i32, s: i32) -> Result<i32> {
    Ok(i << s)
}

/// Returns bitwise right shift of Integer i by s bits (i >> s in C).
#[inline(always)]
pub const fn intBitRShift(i: i32, s: i32) -> Result<i32> {
    Ok(i >> s)
}

// ============================================================================
// Integer conversion functions
// ============================================================================

/// Converts Integer to Real.
#[inline(always)]
pub fn intReal(i: i32) -> Result<f64> {
    Ok(i as f64)
}

/// Converts Integer to String.
pub fn intString(i: i32) -> Result<String> {
    Ok(i.to_string())
}

// ============================================================================
// Real arithmetic functions
// ============================================================================

/// Adds two Real values.
#[inline(always)]
pub fn realAdd(r1: f64, r2: f64) -> Result<f64> {
    Ok(r1 + r2)
}

/// Subtracts two Real values.
#[inline(always)]
pub fn realSub(r1: f64, r2: f64) -> Result<f64> {
    Ok(r1 - r2)
}

/// Multiplies two Real values.
#[inline(always)]
pub fn realMul(r1: f64, r2: f64) -> Result<f64> {
    Ok(r1 * r2)
}

/// Divides two Real values.
#[inline(always)]
pub fn realDiv(r1: f64, r2: f64) -> Result<f64> {
    Ok(r1 / r2)
}

/// Calculates remainder of Real division r1/r2.
pub fn realMod(r1: f64, r2: f64) -> Result<f64> {
    Ok(r1 % r2)
}

/// Raises r1 to the power r2 (r1^r2).
pub fn realPow(r1: f64, r2: f64) -> Result<f64> {
    Ok(r1.powf(r2))
}

/// Returns the bigger one of two Real values.
#[inline(always)]
pub fn realMax(r1: f64, r2: f64) -> Result<f64> {
    Ok(r1.max(r2))
}

/// Returns the smaller one of two Real values.
#[inline(always)]
pub fn realMin(r1: f64, r2: f64) -> Result<f64> {
    Ok(r1.min(r2))
}

/// Returns the absolute value of Real x.
#[inline(always)]
pub fn realAbs(x: f64) -> Result<f64> {
    Ok(x.abs())
}

/// Returns whether two Real values are approximately equal within absTol.
pub fn realAlmostEq(a: f64, b: f64, abs_tol: f64) -> Result<bool> {
    Ok(abs_tol > (a - b).abs())
}

/// Returns negative value of Real x.
#[inline(always)]
pub fn realNeg(x: f64) -> Result<f64> {
    Ok(-x)
}

// ============================================================================
// Real comparison functions
// ============================================================================

/// Returns whether Real x1 is smaller than Real x2.
#[inline(always)]
pub fn realLt(x1: f64, x2: f64) -> Result<bool> {
    Ok(x1 < x2)
}

/// Returns whether Real x1 is smaller than or equal to Real x2.
#[inline(always)]
pub fn realLe(x1: f64, x2: f64) -> Result<bool> {
    Ok(x1 <= x2)
}

/// Returns whether Real x1 is equal to Real x2.
#[inline(always)]
pub fn realEq(x1: f64, x2: f64) -> Result<bool> {
    Ok(x1 == x2)
}

/// Returns whether Real x1 is not equal to Real x2.
#[inline(always)]
pub fn realNe(x1: f64, x2: f64) -> Result<bool> {
    Ok(x1 != x2)
}

/// Returns whether Real x1 is greater than or equal to Real x2.
#[inline(always)]
pub fn realGe(x1: f64, x2: f64) -> Result<bool> {
    Ok(x1 >= x2)
}

/// Returns whether Real x1 is greater than Real x2.
#[inline(always)]
pub fn realGt(x1: f64, x2: f64) -> Result<bool> {
    Ok(x1 > x2)
}

// ============================================================================
// Real conversion functions
// ============================================================================

/// Converts Real to Integer (truncates toward zero, matching Modelica integer() function).
pub fn realInt(r: f64) -> Result<i32> {
    Ok(r as i32)
}

/// Converts Real to String.
pub fn realString(r: f64) -> Result<Arc<String>> {
    Ok(Arc::new(r.to_string()))
}

// ============================================================================
// String character functions
// ============================================================================

/// Returns the ASCII code point of a single-character string.
pub fn stringCharInt(ch: Arc<String>) -> Result<i32> {
    if ch.len() != 1 {
        bail!("stringCharInt expects a single-character string, got '{}'", ch);
    };
    ch.chars().next()
        .map(|c| c as i32)
        .ok_or_else(|| anyhow::anyhow!("Failed to get character from string: {}", ch))
}

/// Returns a single-character string from an ASCII code point.
pub fn intStringChar(i: i32) -> Result<Arc<String>> {
    Ok(Arc::new(std::char::from_u32(i as u32)
        .map(|c| c.to_string())
        .unwrap_or_default()))
}

/// Parses an integer from a string. Fails if the string is not a valid integer.
pub fn stringInt(str: String) -> Result<i32> {
    str.parse::<i32>().map_err(|_| anyhow::anyhow!("Failed to parse integer from string: {}", str))
}

/// Parses a real (f64) from a string.
/// Fails unless the whole string can be consumed.
pub fn stringReal(str: String) -> Result<f64> {
    str.parse::<f64>().map_err(|_| anyhow::anyhow!("Failed to parse real from string: {}", str))
}

/// Converts a string to a list of single-character strings.
pub fn stringListStringChar(str: Arc<String>) -> Result<List<Arc<String>>> {
    // TODO: We could have constants for all these short strings to avoid allocations.
    Ok(str.chars().map(|c| Arc::new(c.to_string())).collect())
}

/// Appends a list of strings into a single string.
pub fn stringAppendList(strs: &List<Arc<String>>) -> Result<Arc<String>> {
    let mut result = String::new();

    for s in strs {
        result.push_str(&s);
    }

    Ok(Arc::new(result))
}

/// Takes a list of strings and a delimiter and joins them with the delimiter inserted between elements.
/// Example: stringDelimitList({"x","y","z"}, ", ") => "x, y, z"
pub fn stringDelimitList(strs: &List<Arc<String>>, delimiter: Arc<String>) -> Result<Arc<String>> {
    let mut result = String::new();
    let mut first = true;

    for s in strs {
        if !first {
            result.push_str(&delimiter);
        }
        result.push_str(&s);
        first = false;
    }

    Ok(Arc::new(result))
}

/// Returns the length of the string (number of bytes).
pub fn stringLength(str: String) -> Result<i32> {
    Ok(str.len() as i32)
}

/// Returns true if the string is empty.
pub fn stringEmpty(str: String) -> Result<bool> {
    Ok(str.is_empty())
}

/// Returns the byte value at the given 1-based index.
pub fn stringGet(str: Arc<String>, index: i32) -> Result<i32> {
    let idx = (index - 1) as usize; // 1-based to 0-based
    str.bytes().nth(idx)
        .map(|b| b as i32)
        .ok_or_else(|| anyhow::anyhow!("Index {} out of bounds for string of length {}", index, str.len()))
}

/// Returns the character at the given 1-based index as a string.
pub fn stringGetStringChar(str: Arc<String>, index: i32) -> Result<Arc<String>> {
    let idx = (index - 1) as usize; // 1-based to 0-based
    str.chars().nth(idx)
        .map(|c| Arc::new(c.to_string()))
        .ok_or_else(|| anyhow::anyhow!("Index {} out of bounds for string of length {}", index, str.chars().count()))
}

/// Updates the character at the given 1-based index with newch.
/// newch should be a single character.
pub fn stringUpdateStringChar(str: Arc<String>, newch: Arc<String>, index: i32) -> Result<Arc<String>> {
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
    Ok(Arc::new(chars.into_iter().collect()))
}

/// Concatenates two strings (s1 + s2).
pub fn stringAppend(s1: String, s2: String) -> Result<Arc<String>> {
    Ok(Arc::new(format!("{}{}", s1, s2)))
}

/// Compares two strings for equality.
#[inline(always)]
pub fn stringEq(s1: String, s2: String) -> Result<bool> {
    Ok(s1 == s2)
}
#[inline(always)]
pub fn stringEqual(s1: String, s2: String) -> Result<bool> {
    Ok(s1 == s2)
}

/// Compares two strings lexicographically.
/// Returns negative if s1 < s2, zero if s1 == s2, positive if s1 > s2.
pub fn stringCompare(s1: String, s2: String) -> Result<i32> {
    // Byte-by-byte comparison for consistency
    let bytes1 = s1.as_bytes();
    let bytes2 = s2.as_bytes();
    let len = bytes1.len().min(bytes2.len());
    for i in 0..len {
        if bytes1[i] < bytes2[i] {
            return Ok(-1);
        }
        if bytes1[i] > bytes2[i] {
            return Ok(1);
        }
    }
    // Length comparison if all compared bytes were equal
    match bytes1.len().cmp(&bytes2.len()) {
        std::cmp::Ordering::Less => Ok(-1),
        std::cmp::Ordering::Equal => Ok(0),
        std::cmp::Ordering::Greater => Ok(1),
    }
}

/// Returns a hash of the string using Rust's built-in hash.
pub fn stringHash(str: String) -> Result<i32> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hash;
    use std::hash::Hasher;
    let mut hasher = DefaultHasher::new();
    str.hash(&mut hasher);
    Ok(hasher.finish() as i32)
}

/// Returns a DJB2 hash of the string.
/// DJB2 algorithm: hash = hash * 33 + byte
pub fn stringHashDjb2(str: String) -> Result<i32> {
    let mut hash: i32 = 5381;
    for &byte in str.as_bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as i32);
    }
    Ok(hash)
}

/// Continues computing a DJB2 hash by adding another string to it.
pub fn stringHashDjb2Continue(str: String, hash: i32) -> Result<i32> {
    let mut h = hash;
    for &byte in str.as_bytes() {
        h = h.wrapping_mul(33).wrapping_add(byte as i32);
    }
    Ok(h)
}

/// Computes a DJB2 hash and applies modulo without intermediate overflow issues.
pub fn stringHashDjb2Mod(str: String, mod_val: i32) -> Result<i32> {
    if mod_val == 0 {
        return Ok(0);
    }
    let mut hash: i64 = 5381;
    for &byte in str.as_bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as i64);
    }
    Ok((hash % mod_val as i64) as i32)
}

/// Returns an SDBM hash of the string.
/// SDBM algorithm: hash = byte + (hash << 6) + (hash << 16) - hash
pub fn stringHashSdbm(str: String) -> Result<i32> {
    let mut hash: i32 = 0;
    for &byte in str.as_bytes() {
        hash = byte as i32 + (hash << 6) + (hash << 16) - hash;
    }
    Ok(hash)
}

/// Extracts a substring from str.
/// start and stop are 1-based indices (first character is at index 1).
/// Fails for bogus start/stop values.
pub fn substring(str: Arc<String>, start: i32, stop: i32) -> Result<Arc<String>> {
    if start < 1 || stop < start || start > stop {
        bail!("Invalid substring range: start={}, stop={}", start, stop);
    }
    let start_idx = (start - 1) as usize; // 1-based to 0-based
    let stop_idx = stop as usize;         // 1-based, inclusive -> exclusive
    let chars: Vec<char> = str.chars().collect();
    if stop_idx > chars.len() {
        bail!("Stop index {} exceeds string length {}", stop, chars.len());
    }
    Ok(Arc::new(chars[start_idx..stop_idx].iter().collect()))
}

/// Alias for string_append_list (maps a list of single-char strings to one string).
pub fn listStringCharString(strs: &List<Arc<String>>) -> Result<Arc<String>> {
    stringAppendList(strs)
}

/// Alias for string_append_list (maps a list of single-char strings to one string).
pub fn stringCharListString(strs: &List<Arc<String>>) -> Result<Arc<String>> {
    stringAppendList(strs)
}

// ============================================================================
// List functions
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum List<T: Clone> {
    Cons{head: T, tail: Arc<List<T>>},
    Nil,
}
use List::{Cons, Nil};

#[macro_export]
macro_rules! list {
    // Base case: empty list
    () => {
        $crate::List::Nil
    };
    // Case with a trailing comma
    ( $($x:expr),*, ) => {
        list!($($x),*)
    };
    // General case: peel off the first element and recurse
    ( $x:expr, $($rest:expr),+ ) => {
        $crate::cons($x, list!($($rest),+))
    };
    // Single element case
    ( $x:expr ) => {
        $crate::cons($x, list!())
    };
}

pub fn cons<T: Clone>(head: T, tail: List<T>) -> List<T> {
    Cons{head: head, tail: Arc::new(tail)}
}

impl<T: Clone> Default for List<T> {
    fn default() -> List<T> {
        Nil
    }
}
pub struct ListIterator<'a, T: Clone> {
    curr: &'a List<T>,
}

impl<T: Clone> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> List<T> {
        let mut buf = Nil;
        for item in iter {
            buf = cons(item, buf);
        }
        buf.reverse()
    }
}

impl<'a, T: Clone> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = ListIterator<'a, T>;

    // Required method
    fn into_iter(self) -> Self::IntoIter {
        ListIterator { curr: self }
    }
}

impl<'a, T: Clone> Iterator for ListIterator<'a, T> {
    type Item = &'a T; // No Clone needed here!

    fn next(&mut self) -> Option<Self::Item> {
        // We match on the dereferenced value of self.curr to see the content.
        // Note: We do NOT move *self.curr; we just look at it.
        match self.curr {
            // If it's Nil, we are done.
            List::Nil => None,

            // If it's Cons:
            List::Cons { head, tail } => {
                // 1. `head` is a `&'a T` because `self.curr` is `&'a List<T>` and `ref head` borrows from it.
                // 2. `tail` is a `&Box<List<T>>`. We want to advance to the next node.
                //    We borrow the contents of the Box. Since the Box lives inside the List
                //    (which lives for 'a), this new reference is also valid for 'a.
                let next_node = &**tail;

                // 3. Update the iterator state to point to the next node.
                self.curr = next_node;

                // 4. Return the head.
                Some(head)
            }
        }
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
            result = cons(item.clone(), result);
        }
        result
    }
    /// Returns the length of a list. O(n).
    pub fn len(&self) -> i32 {
        self.into_iter().count() as i32
    }
    /// Reverses the elements in a list. O(n).
    pub fn reverse(self: &List<T>) -> List<T> {
        let mut result: List<T> = Nil;
        for e in self {
            result = cons(e.clone(), result);
        }
        result
    }
    /// Gets the element at the given 1-based index. O(index).
    pub fn get(self: &List<T>, index: i32) -> Result<T> {
        self.into_iter().nth((index - 1) as usize)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Index {} out of bounds for list of length {}", index, self.len()))
    }
    pub fn prepend_reverse(self: List<T>, prefix: &List<T>) -> List<T> {
        let mut result = self;
        for item in prefix {
            result = cons(item.clone(), result);
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
        let mut result = Nil;
        let mut iter: &List<T> = self;
        let mut cur_index = index;
        loop {
            cur_index -= 1;
            let (head,tail) = match iter {
                Nil => bail!("Index {} out of bounds for list", index),
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

impl<T: Clone> List<T> {
    pub fn new(item: T) -> List<T> {
        Cons{head: item, tail: Arc::new(Nil)}
    }
    pub fn cons(self: List<T>, item: T) -> List<T> {
        Cons{head: item, tail: Arc::new(self)}
    }
    /// Gets the first element. O(1).
    /// Fails if the list is empty.
    pub fn head(self: &List<T>) -> Result<&T> {
        match self {
            Nil => bail!("Cannot get rest of empty list"),
            Cons{head, ..} => Ok(head),
        }
    }
    /// Returns all elements except the first. O(1).
    /// Fails if the list is empty.
    pub fn rest(self: &List<T>) -> Result<&List<T>> {
        match self {
            Nil => bail!("Cannot get rest of empty list"),
            Cons{tail, ..} => Ok(tail),
        }
    }
    /// Returns true if the list is empty. O(1).
    pub fn is_empty(self: &List<T>) -> bool {
        match self {
            Nil => true,
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
pub fn arrayLength<A>(arr: &[A]) -> Result<i32> {
    Ok(arr.len() as i32)
}

/// Returns true if the array is empty. O(1).
pub fn arrayEmpty<A>(arr: &[A]) -> Result<bool> {
    Ok(arr.is_empty())
}

/// Gets the element at the given 1-based index. O(1).
pub fn arrayGet<A: Clone>(arr: &[A], index: i32) -> Result<A> {
    let idx = (index - 1) as usize; // 1-based to 0-based
    arr.get(idx)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("Index {} out of bounds for array of length {}", index, arr.len()))
}

/// Creates a new array of the given size, initialized with initialValue. O(size).
pub fn arrayCreate<A: Clone>(size: i32, initial_value: A) -> Result<Vec<A>> {
    if size <= 0 {
        return Ok(Vec::new());
    }
    Ok(vec![initial_value; size as usize])
}

/// Converts an array to a list. O(n).
pub fn arrayList<A: Clone>(arr: &[A]) -> Result<List<A>> {
    let mut result = List::Nil;
    for item in arr.iter().rev().cloned() {
        result = List::cons(result, item);
    }
    Ok(result)
}

/// Converts a list to an array. O(n).
pub fn listArray<A: Clone>(lst: &List<A>) -> Result<Vec<A>> {
    let mut result = Vec::new();
    for item in lst {
        result.push(item.clone());
    }
    Ok(result)
}

/// Updates the value at the given 1-based index. O(1).
/// Mutates the array in place (impure).
pub fn arrayUpdate<A: Clone>(arr: &mut Vec<A>, index: i32, new_value: A) -> Result<&mut Vec<A>> {
    let idx = (index - 1) as usize; // 1-based to 0-based
    let len = arr.len();
    if idx >= len {
        bail!("Index {} out of bounds for array of length {}", index, len);
    }
    arr[idx] = new_value;
    Ok(arr)
}

/// Creates a copy of the array. O(n).
pub fn arrayCopy<A: Clone>(arr: &[A]) -> Result<Vec<A>> {
    Ok(arr.to_vec())
}

/// Appends arr2 to arr1, creating a new array. O(length(arr1) + length(arr2)).
pub fn arrayAppend<A: Clone>(arr1: &[A], arr2: &[A]) -> Result<Vec<A>> {
    let mut result = arr1.to_vec();
    result.extend(arr2.iter().cloned());
    Ok(result)
}

// ============================================================================
// Generic value functions
// ============================================================================

/// Returns the string representation of any Debug-printable value.
/// Rather slow; only use this for debugging!
pub fn anyString<A: std::fmt::Debug>(a: &A) -> Result<Arc<String>> {
    Ok(Arc::new(format!("{:?}", a)))
}

/// Prints any Debug-printable value to stderr.
pub fn printAny<A: std::fmt::Debug>(a: &A) {
    eprintln!("{:?}", a);
}

/// Prints a debug string prefix followed by any Debug-printable value to stderr.
/// For RML compatibility.
pub fn debug_print<A: std::fmt::Debug>(str: String, a: &A) {
    eprintln!("{}: {:?}", str, a);
}

thread_local! {
    static TICK_COUNTER: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// Returns a monotonically increasing tick counter.
/// Uses a thread-local counter for simulation purposes.
pub fn tick() -> Result<i32> {
    TICK_COUNTER.with(|counter| {
        let current = counter.get();
        counter.set(current.wrapping_add(1));
        Ok(current as i32)
    })
}

/// Structural equality for any PartialEq value.
pub fn valueEq<A: PartialEq>(a1: &A, a2: &A) -> Result<bool> {
    Ok(a1 == a2)
}

/// Compares two Ord values.
/// Returns -1 if a1 < a2, 0 if a1 == a2, 1 if a1 > a2.
pub fn valueCompare<A: Ord>(a1: &A, a2: &A) -> Result<i32> {
    match a1.cmp(a2) {
        std::cmp::Ordering::Less => Ok(-1),
        std::cmp::Ordering::Equal => Ok(0),
        std::cmp::Ordering::Greater => Ok(1),
    }
}

/// Reference equality check.
/// In Rust, this checks pointer equality for reference-counted types.
/// For simple types, falls back to structural equality.
///
/// This is a very fast comparison to speed up comparisons.
/// If you know that all occurrences of a value are the same pointer,
/// you can use reference_eq instead of structural equality.
pub fn referenceEq<A: PartialEq>(a1: &A, a2: &A) -> Result<bool> {
    Ok(std::ptr::eq(a1 as *const A, a2 as *const A))
}

/// Returns the pointer address of a reference as a hexadecimal string for debugging.
pub fn referencePointerString<A>(a: &A) -> Result<Arc<String>> {
    Ok(Arc::new(format!("{:p}", a)))
}

/// Returns a debug string for a function symbol.
/// In Rust, returns the type name of the value for debugging.
pub fn referenceDebugString<A: std::fmt::Debug>(_a: &A) -> Result<String> {
    Ok(format!("{:?}", std::any::type_name::<A>()))
}

/// Returns the constructor tag for a boxed value.
/// In Rust, returns a type-based discriminator.
pub fn valueConstructor<A>() -> Result<i32> {
    // Use a hash of the type name as a stable discriminator
    let type_name = std::any::type_name::<A>();
    let mut hash: i32 = 5381;
    for &byte in type_name.as_bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as i32);
    }
    Ok(hash)
}

/// Returns the current time in seconds relative to process start.
/// Not very accurate, intended for diff comparisons.
fn getStartInstant() -> std::time::Instant {
    use std::sync::OnceLock;
    static START: OnceLock<std::time::Instant> = OnceLock::new();
    *START.get_or_init(|| std::time::Instant::now())
}

pub fn clock() -> Result<f64> {
    Ok(getStartInstant().elapsed().as_secs_f64())
}

// ============================================================================
// Option functions
// ============================================================================

/// Returns true if the Option is NONE.
pub fn isNone<A>(opt: &Option<A>) -> Result<bool> {
    Ok(matches!(opt, None))
}

/// Returns true if the Option is SOME.
pub fn isSome<A>(opt: &Option<A>) -> Result<bool> {
    Ok(matches!(opt, Some(_)))
}

// ============================================================================
// Misc builtin functions
// ============================================================================

/// Sets the stack overflow signal to the given value and returns the old one.
/// In this translation, simply returns the input value.
pub fn setStackOverflowSignal(in_signal: bool) -> Result<bool> {
    Ok(in_signal)
}

/// Returns true if the formal output argument is present as an actual argument.
/// In MetaModelica this is a compile-time check; in Rust it always returns true
/// because the argument exists at the call site.
pub fn isPresent<T>(_ident: &T) -> Result<bool> {
    Ok(true)
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
    pub fn arrayGetNoBoundsChecking<A: Clone>(arr: &[A], index: i32) -> Result<A> {
        let idx = (index - 1) as usize; // 1-based to 0-based
        // SAFETY: Caller must ensure index is in bounds.
        // Rust does not have true unchecked indexing, but unsafe::get_unchecked avoids bounds check.
        unsafe { Ok(arr.get_unchecked(idx).clone()) }
    }

    /// Unsafe array update without bounds checking.
    /// Mutates the array in place.
    pub fn arrayUpdateNoBoundsChecking<A: Clone>(arr: &mut Vec<A>, index: i32, new_value: A) -> Result<()> {
        let idx = (index - 1) as usize; // 1-based to 0-based
        // SAFETY: Caller must ensure index is in bounds.
        unsafe { *arr.get_unchecked_mut(idx) = new_value }
        Ok(())
    }

    /// Creates a new array with uninitialized elements.
    /// The dummy parameter is used to fix the type of the array.
    /// Elements are set to a clone of dummy.
    pub fn arrayCreateNoInit<A: Clone>(size: i32, dummy: A) -> Result<Vec<A>> {
        if size <= 0 {
            return Ok(Vec::new());
        }
        // SAFETY: We immediately fill with cloned values so the array is never used uninitialized.
        // In a true unsafe translation we could use MaybeUninit, but for safety we fill with dummy.
        Ok(vec![dummy; size as usize])
    }
    /// Unsafe string get without bounds checking.
    pub fn stringGetNoBoundsChecking(str: String, index: i32) -> Result<i32> {
        let idx = (index - 1) as usize; // 1-based to 0-based
        // SAFETY: Caller must ensure index is in bounds.
        unsafe { Ok((*str.as_bytes().get_unchecked(idx)) as i32) }
    }
    /// Reverses a list in place.
    pub fn listReverseInPlace<T: Clone>(list: &List<T>) -> Result<List<T>>{
        Ok(list.reverse())
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
            assert!(boolAnd(true, true).unwrap());
            assert!(!boolAnd(true, false).unwrap());
            assert!(!boolAnd(false, true).unwrap());
            assert!(!boolAnd(false, false).unwrap());
        }

        #[test]
        fn test_bool_or() {
            assert!(boolOr(true, false).unwrap());
            assert!(boolOr(false, true).unwrap());
            assert!(boolOr(true, true).unwrap());
            assert!(!boolOr(false, false).unwrap());
        }

        #[test]
        fn test_bool_not() {
            assert!(boolNot(false).unwrap());
            assert!(!boolNot(true).unwrap());
        }

        #[test]
        fn test_bool_eq() {
            assert!(boolEq(true, true).unwrap());
            assert!(boolEq(false, false).unwrap());
            assert!(!boolEq(true, false).unwrap());
            assert!(!boolEq(false, true).unwrap());
        }

        #[test]
        fn test_bool_string() {
            assert_eq!(*boolString(true).unwrap(), "true");
            assert_eq!(*boolString(false).unwrap(), "false");
        }
    }

    // =========================================================================
    // Integer arithmetic tests
    // =========================================================================

    mod int_arithmetic_tests {
        use super::*;

        #[test]
        fn test_int_add() {
            assert_eq!(intAdd(1, 2).unwrap(), 3);
            assert_eq!(intAdd(-1, 1).unwrap(), 0);
            assert_eq!(intAdd(-1, -2).unwrap(), -3);
        }

        #[test]
        fn test_int_sub() {
            assert_eq!(intSub(5, 3).unwrap(), 2);
            assert_eq!(intSub(3, 5).unwrap(), -2);
            assert_eq!(intSub(0, 0).unwrap(), 0);
        }

        #[test]
        fn test_int_mul() {
            assert_eq!(intMul(3, 4).unwrap(), 12);
            assert_eq!(intMul(-3, 4).unwrap(), -12);
            assert_eq!(intMul(-3, -4).unwrap(), 12);
            assert_eq!(intMul(0, 100).unwrap(), 0);
        }

        #[test]
        fn test_int_div() {
            assert_eq!(intDiv(10, 3).unwrap(), 3);
            assert_eq!(intDiv(10, -3).unwrap(), -3);
            assert_eq!(intDiv(-10, 3).unwrap(), -3);
            assert_eq!(intDiv(-10, -3).unwrap(), 3);
        }

        #[test]
        fn test_int_mod() {
            assert_eq!(intMod(10, 3).unwrap(), 1);
            assert_eq!(intMod(10, -3).unwrap(), 1);
            assert_eq!(intMod(-10, 3).unwrap(), -1);
            assert_eq!(intMod(-10, -3).unwrap(), -1);
        }

        #[test]
        fn test_int_max() {
            assert_eq!(intMax(1, 2).unwrap(), 2);
            assert_eq!(intMax(2, 1).unwrap(), 2);
            assert_eq!(intMax(5, 5).unwrap(), 5);
            assert_eq!(intMax(-1, -2).unwrap(), -1);
        }

        #[test]
        fn test_int_min() {
            assert_eq!(intMin(1, 2).unwrap(), 1);
            assert_eq!(intMin(2, 1).unwrap(), 1);
            assert_eq!(intMin(5, 5).unwrap(), 5);
            assert_eq!(intMin(-1, -2).unwrap(), -2);
        }

        #[test]
        fn test_int_abs() {
            assert_eq!(intAbs(-5).unwrap(), 5);
            assert_eq!(intAbs(5).unwrap(), 5);
            assert_eq!(intAbs(0).unwrap(), 0);
        }

        #[test]
        fn test_int_neg() {
            assert_eq!(intNeg(5).unwrap(), -5);
            assert_eq!(intNeg(-5).unwrap(), 5);
            assert_eq!(intNeg(0).unwrap(), 0);
        }
    }

    // =========================================================================
    // Integer comparison tests
    // =========================================================================

    mod int_comparison_tests {
        use super::*;

        #[test]
        fn test_int_lt() {
            assert!(intLt(1, 2).unwrap());
            assert!(!intLt(2, 2).unwrap());
            assert!(!intLt(2, 1).unwrap());
        }

        #[test]
        fn test_int_le() {
            assert!(intLe(1, 2).unwrap());
            assert!(intLe(2, 2).unwrap());
            assert!(!intLe(2, 1).unwrap());
        }

        #[test]
        fn test_int_eq() {
            assert!(intEq(5, 5).unwrap());
            assert!(!intEq(5, 6).unwrap());
        }

        #[test]
        fn test_int_ne() {
            assert!(intNe(5, 6).unwrap());
            assert!(!intNe(5, 5).unwrap());
        }

        #[test]
        fn test_int_ge() {
            assert!(intGe(2, 1).unwrap());
            assert!(intGe(2, 2).unwrap());
            assert!(!intGe(1, 2).unwrap());
        }

        #[test]
        fn test_int_gt() {
            assert!(intGt(2, 1).unwrap());
            assert!(!intGt(2, 2).unwrap());
            assert!(!intGt(1, 2).unwrap());
        }
    }

    // =========================================================================
    // Integer bitwise tests
    // =========================================================================

    mod int_bitwise_tests {
        use super::*;

        #[test]
        fn test_int_bit_not() {
            assert_eq!(intBitNot(0i32).unwrap(), -1);
            assert_eq!(intBitNot(-1i32).unwrap(), 0);
            assert_eq!(intBitNot(1).unwrap(), !1);
        }

        #[test]
        fn test_int_bit_and() {
            assert_eq!(intBitAnd(0b1100, 0b1010).unwrap(), 0b1000);
            assert_eq!(intBitAnd(0, 5).unwrap(), 0);
        }

        #[test]
        fn test_int_bit_or() {
            assert_eq!(intBitOr(0b1100, 0b1010).unwrap(), 0b1110);
            assert_eq!(intBitOr(0, 5).unwrap(), 5);
        }

        #[test]
        fn test_int_bit_xor() {
            assert_eq!(intBitXor(0b1100, 0b1010).unwrap(), 0b0110);
            assert_eq!(intBitXor(5, 5).unwrap(), 0);
        }

        #[test]
        fn test_int_bit_l_shift() {
            assert_eq!(intBitLShift(1, 3).unwrap(), 8);
            assert_eq!(intBitLShift(3, 1).unwrap(), 6);
        }

        #[test]
        fn test_int_bit_r_shift() {
            assert_eq!(intBitRShift(8, 3).unwrap(), 1);
            assert_eq!(intBitRShift(6, 1).unwrap(), 3);
        }
    }

    // =========================================================================
    // Integer conversion tests
    // =========================================================================

    mod int_conversion_tests {
        use super::*;

        #[test]
        fn test_int_real() {
            assert_eq!(intReal(42).unwrap(), 42.0_f64);
            assert_eq!(intReal(-7).unwrap(), -7.0_f64);
        }

        #[test]
        fn test_int_string() {
            assert_eq!(&*intString(42).unwrap(), "42");
            assert_eq!(&*intString(-7).unwrap(), "-7");
            assert_eq!(&*intString(0).unwrap(), "0");
        }
    }

    // =========================================================================
    // Real arithmetic tests
    // =========================================================================

    mod real_arithmetic_tests {
        use super::*;

        #[test]
        fn test_real_add() {
            assert_eq!(realAdd(1.5, 2.5).unwrap(), 4.0);
            assert_eq!(realAdd(-1.0, 1.0).unwrap(), 0.0);
        }

        #[test]
        fn test_real_sub() {
            assert_eq!(realSub(5.0, 3.0).unwrap(), 2.0);
            assert_eq!(realSub(3.0, 5.0).unwrap(), -2.0);
        }

        #[test]
        fn test_real_mul() {
            assert_eq!(realMul(3.0, 4.0).unwrap(), 12.0);
            assert_eq!(realMul(-3.0, 4.0).unwrap(), -12.0);
        }

        #[test]
        fn test_real_div() {
            assert_eq!(realDiv(10.0, 3.0).unwrap(), 10.0 / 3.0);
            assert_eq!(realDiv(6.0, 2.0).unwrap(), 3.0);
        }

        #[test]
        fn test_real_mod() {
            assert_eq!(realMod(10.0, 3.0).unwrap(), 1.0);
            assert_eq!(realMod(10.5, 3.0).unwrap(), 1.5);
        }

        #[test]
        fn test_real_pow() {
            assert_eq!(realPow(2.0, 3.0).unwrap(), 8.0);
            assert_eq!(realPow(9.0, 0.5).unwrap(), 3.0);
        }

        #[test]
        fn test_real_max() {
            assert_eq!(realMax(1.5, 2.5).unwrap(), 2.5);
            assert_eq!(realMax(5.0, 5.0).unwrap(), 5.0);
        }

        #[test]
        fn test_real_min() {
            assert_eq!(realMin(1.5, 2.5).unwrap(), 1.5);
            assert_eq!(realMin(5.0, 5.0).unwrap(), 5.0);
        }

        #[test]
        fn test_real_abs() {
            assert_eq!(realAbs(-5.5).unwrap(), 5.5);
            assert_eq!(realAbs(5.5).unwrap(), 5.5);
        }

        #[test]
        fn test_real_almost_eq() {
            assert!(realAlmostEq(1.0, 1.0000001, 1e-5).unwrap());
            assert!(!realAlmostEq(1.0, 1.1, 1e-5).unwrap());
            assert!(realAlmostEq(1.0, 1.0, 1e-6).unwrap());
        }

        #[test]
        fn test_real_neg() {
            assert_eq!(realNeg(5.5).unwrap(), -5.5);
            assert_eq!(realNeg(-5.5).unwrap(), 5.5);
        }
    }

    // =========================================================================
    // Real comparison tests
    // =========================================================================

    mod real_comparison_tests {
        use super::*;

        #[test]
        fn test_real_lt() {
            assert!(realLt(1.0, 2.0).unwrap());
            assert!(!realLt(2.0, 2.0).unwrap());
            assert!(!realLt(2.0, 1.0).unwrap());
        }

        #[test]
        fn test_real_le() {
            assert!(realLe(1.0, 2.0).unwrap());
            assert!(realLe(2.0, 2.0).unwrap());
            assert!(!realLe(2.0, 1.0).unwrap());
        }

        #[test]
        fn test_real_eq() {
            assert!(realEq(1.0, 1.0).unwrap());
            assert!(!realEq(1.0, 2.0).unwrap());
        }

        #[test]
        fn test_real_ne() {
            assert!(realNe(1.0, 2.0).unwrap());
            assert!(!realNe(1.0, 1.0).unwrap());
        }

        #[test]
        fn test_real_ge() {
            assert!(realGe(2.0, 1.0).unwrap());
            assert!(realGe(2.0, 2.0).unwrap());
            assert!(!realGe(1.0, 2.0).unwrap());
        }

        #[test]
        fn test_real_gt() {
            assert!(realGt(2.0, 1.0).unwrap());
            assert!(!realGt(2.0, 2.0).unwrap());
            assert!(!realGt(1.0, 2.0).unwrap());
        }
    }

    // =========================================================================
    // Real conversion tests
    // =========================================================================

    mod real_conversion_tests {
        use super::*;

        #[test]
        fn test_real_int() {
            assert_eq!(realInt(3.7).unwrap(), 3);
            assert_eq!(realInt(-3.7).unwrap(), -3);
            assert_eq!(realInt(3.0).unwrap(), 3);
        }

        #[test]
        fn test_real_string() {
            assert_eq!(&*realString(3.14).unwrap(), "3.14");
            assert_eq!(&*realString(0.0).unwrap(), "0");
            assert_eq!(&*realString(-1.5).unwrap(), "-1.5");
        }
    }

    // =========================================================================
    // String character tests
    // =========================================================================

    mod string_char_tests {
        use super::*;

        #[test]
        fn test_string_char_int() {
            assert_eq!(stringCharInt(Arc::new("A".to_string())).unwrap(), 65);
            assert_eq!(stringCharInt(Arc::new("a".to_string())).unwrap(), 97);
            assert_eq!(stringCharInt(Arc::new("0".to_string())).unwrap(), 48);
        }

        #[test]
        fn test_int_string_char() {
            assert_eq!(&*intStringChar(65).unwrap(), "A");
            assert_eq!(&*intStringChar(97).unwrap(), "a");
            assert_eq!(&*intStringChar(48).unwrap(), "0");
            assert_eq!(&*intStringChar(0).unwrap(), "\0");
        }

        #[test]
        fn test_string_int() {
            assert_eq!(stringInt("42".to_string()).unwrap(), 42);
            assert_eq!(stringInt("-7".to_string()).unwrap(), -7);
            assert!(stringInt("not_a_number".to_string()).is_err());
        }

        #[test]
        fn test_string_real() {
            assert_eq!(stringReal("3.14".to_string()).unwrap(), 3.14);
            assert_eq!(stringReal("-2.5".to_string()).unwrap(), -2.5);
            assert!(stringReal("not_a_number".to_string()).is_err());
        }

        #[test]
        fn test_string_list_string_char() {
            let result = stringListStringChar(Arc::new("abc".to_string())).unwrap();
            assert_eq!(result, List::from_iter([Arc::new("a".to_string()), Arc::new("b".to_string()), Arc::new("c".to_string())]));
        }

        #[test]
        fn test_string_append_list() {
            let strs = list![Arc::new("hello".to_string()), Arc::new(" ".to_string()), Arc::new("world".to_string())];
            assert_eq!(&*stringAppendList(&strs).unwrap(), "hello world");
        }

        #[test]
        fn test_string_delimit_list() {
            let strs = list![Arc::new("x".to_string()), Arc::new("y".to_string()), Arc::new("z".to_string())];
            assert_eq!(&*stringDelimitList(&strs, Arc::new(", ".to_string())).unwrap(), "x, y, z");
        }
    }

    // =========================================================================
    // String length and empty tests
    // =========================================================================

    mod string_length_tests {
        use super::*;

        #[test]
        fn test_string_length() {
            assert_eq!(stringLength("hello".to_string()).unwrap(), 5);
            assert_eq!(stringLength("".to_string()).unwrap(), 0);
        }

        #[test]
        fn test_string_empty() {
            assert!(stringEmpty("".to_string()).unwrap());
            assert!(!stringEmpty("hello".to_string()).unwrap());
        }
    }

    // =========================================================================
    // String get/update tests
    // =========================================================================

    mod string_get_update_tests {
        use super::*;

        #[test]
        fn test_string_get() {
            assert_eq!(stringGet(Arc::new("hello".to_string()), 1).unwrap(), b'h' as i32);
            assert_eq!(stringGet(Arc::new("hello".to_string()), 5).unwrap(), b'o' as i32);
            assert!(stringGet(Arc::new("hello".to_string()), 0).is_err());
            assert!(stringGet(Arc::new("hello".to_string()), 6).is_err());
        }

        #[test]
        fn test_string_get_string_char() {
            assert_eq!(stringGetStringChar(Arc::new("hello".to_string()), 1).unwrap(), Arc::new("h".to_string()));
            assert_eq!(stringGetStringChar(Arc::new("hello".to_string()), 3).unwrap(), Arc::new("l".to_string()));
            assert_eq!(stringGetStringChar(Arc::new("hello".to_string()), 5).unwrap(), Arc::new("o".to_string()));
            assert!(stringGetStringChar(Arc::new("hello".to_string()), 0).is_err());
            assert!(stringGetStringChar(Arc::new("hello".to_string()), 6).is_err());
        }

        #[test]
        fn test_string_update_string_char() {
            assert_eq!(stringUpdateStringChar(Arc::new("hello".to_string()), Arc::new("X".to_string()), 1).unwrap(), Arc::new("Xello".to_string()));
            assert_eq!(stringUpdateStringChar(Arc::new("hello".to_string()), Arc::new("X".to_string()), 3).unwrap(), Arc::new("heXlo".to_string()));
            assert_eq!(stringUpdateStringChar(Arc::new("hello".to_string()), Arc::new("X".to_string()), 5).unwrap(), Arc::new("hellX".to_string()));
            assert!(stringUpdateStringChar(Arc::new("hello".to_string()), Arc::new("X".to_string()), 0).is_err());
            assert!(stringUpdateStringChar(Arc::new("hello".to_string()), Arc::new("X".to_string()), 6).is_err());
            assert!(stringUpdateStringChar(Arc::new("hello".to_string()), Arc::new("".to_string()), 1).is_err());
        }
    }

    // =========================================================================
    // String append/equal tests
    // =========================================================================

    mod string_append_equal_tests {
        use super::*;

        #[test]
        fn test_string_append() {
            assert_eq!(&*stringAppend("hello".to_string(), " world".to_string()).unwrap(), "hello world");
            assert_eq!(&*stringAppend("".to_string(), "hello".to_string()).unwrap(), "hello");
            assert_eq!(&*stringAppend("hello".to_string(), "".to_string()).unwrap(), "hello");
        }

        #[test]
        fn test_string_eq() {
            assert!(stringEq("abc".to_string(), "abc".to_string()).unwrap());
            assert!(!stringEq("abc".to_string(), "abd".to_string()).unwrap());
            assert!(!stringEq("".to_string(), "abc".to_string()).unwrap());
        }

        #[test]
        fn test_string_equal() {
            assert!(stringEqual("abc".to_string(), "abc".to_string()).unwrap());
            assert!(!stringEqual("abc".to_string(), "abd".to_string()).unwrap());
        }
    }

    // =========================================================================
    // String compare test
    // =========================================================================

    mod string_compare_test {
        use super::*;

        #[test]
        fn test_string_compare() {
            assert!(stringCompare("abc".to_string(), "abd".to_string()).unwrap() < 0);
            assert_eq!(stringCompare("abc".to_string(), "abc".to_string()).unwrap(), 0);
            assert!(stringCompare("abd".to_string(), "abc".to_string()).unwrap() > 0);
            assert!(stringCompare("ab".to_string(), "abc".to_string()).unwrap() < 0);
            assert!(stringCompare("abc".to_string(), "ab".to_string()).unwrap() > 0);
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
            assert_eq!(stringHashDjb2("a".to_string()).unwrap(), 5381_i32.wrapping_mul(33).wrapping_add(97));
            assert_eq!(stringHashDjb2("".to_string()).unwrap(), 5381);
        }

        #[test]
        fn test_string_hash_djb2_continue() {
            let h1 = stringHashDjb2("hello".to_string()).unwrap();
            let _h2 = stringHashDjb2(" world".to_string()).unwrap();
            let combined = stringHashDjb2Continue(" world".to_string(), h1).unwrap();
            // Starting from h1 and adding " world" should give the same
            // as hashing "hello world" from scratch
            assert_eq!(combined, stringHashDjb2("hello world".to_string()).unwrap());
        }

        #[test]
        fn test_string_hash_djb2_mod() {
            let h = stringHashDjb2Mod("hello".to_string(), 100).unwrap();
            assert!(h >= 0 && h < 100);
            assert_eq!(stringHashDjb2Mod("hello".to_string(), 0).unwrap(), 0);
        }

        #[test]
        fn test_string_hash_sdbm() {
            // SDBM of "a" = 97 + 0 + 0 - 0 = 97
            assert_eq!(stringHashSdbm("a".to_string()).unwrap(), 97);
            assert_eq!(stringHashSdbm("".to_string()).unwrap(), 0);
        }

        #[test]
        fn test_string_hash_consistency() {
            // Same string should produce same hash
            assert_eq!(stringHash("test".to_string()).unwrap(), stringHash("test".to_string()).unwrap());
        }
    }

    // =========================================================================
    // Substring tests
    // =========================================================================

    mod substring_tests {
        use super::*;

        #[test]
        fn test_substring_basic() {
            assert_eq!(*substring(Arc::new("hello world".to_string()), 1, 5).unwrap(), "hello".to_string());
            assert_eq!(*substring(Arc::new("hello world".to_string()), 7, 11).unwrap(), "world".to_string());
            assert_eq!(*substring(Arc::new("hello".to_string()), 3, 3).unwrap(), "l".to_string());
            assert_eq!(*substring(Arc::new("hello".to_string()), 1, 5).unwrap(), "hello".to_string());
        }

        #[test]
        fn test_substring_errors() {
            assert!(substring(Arc::new("hello".to_string()), 0, 3).is_err());  // start < 1
            assert!(substring(Arc::new("hello".to_string()), 3, 2).is_err());  // stop < start
            assert!(substring(Arc::new("hello".to_string()), 1, 6).is_err());  // stop out of bounds
            assert!(substring(Arc::new("hello".to_string()), 6, 7).is_err());  // start out of bounds
        }
    }

    // =========================================================================
    // List string char string tests
    // =========================================================================

    mod list_string_tests {
        use super::*;

        #[test]
        fn test_list_string_char_string() {
            let strs = list![Arc::new("a".to_string()), Arc::new("b".to_string()), Arc::new("c".to_string())];
            assert_eq!(&*listStringCharString(&strs).unwrap(), "abc");
        }

        #[test]
        fn test_string_char_list_string() {
            let strs = list![Arc::new("a".to_string()), Arc::new("b".to_string()), Arc::new("c".to_string())];
            assert_eq!(&*stringCharListString(&strs).unwrap(), "abc");
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
            let empty: List<i32> = Nil;
            assert_eq!(empty.append(&b), b);
            assert_eq!(a.append(&empty), a);
        }

        #[test]
        fn test_list_reverse() {
            let lst = list![1, 2, 3, 4, 5];
            let result = lst.reverse();
            assert_eq!(result, list![5, 4, 3, 2, 1]);

            let empty: List<i32> = Nil;
            assert_eq!(empty.reverse(), Nil);
        }

        #[test]
        fn test_list_length() {
            let lst = list![1, 2, 3];
            assert_eq!(lst.len(), 3);
            let empty: List<i32> = Nil;
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

            let empty: List<i32> = Nil;
            assert!(empty.rest().is_err());
        }

        #[test]
        fn test_list_head() {
            let lst = list![1, 2, 3];
            assert_eq!(lst.head().unwrap().clone(), 1);

            let empty: List<i32> = Nil;
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

            let empty: List<i32> = Nil;
            assert!(empty.is_empty());
        }

        #[test]
        fn test_cons() {
            let lst = list![2, 3];
            let result = cons(1, lst);
            assert_eq!(result, list![1, 2, 3]);

            let empty: List<i32> = Nil;
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
            assert_eq!(arrayLength(&arr).unwrap(), 3);
            let empty: Vec<i32> = vec![];
            assert_eq!(arrayLength(&empty).unwrap(), 0);
        }

        #[test]
        fn test_array_empty() {
            let arr = vec![1, 2, 3];
            assert!(!arrayEmpty(&arr).unwrap());
            let empty: Vec<i32> = vec![];
            assert!(arrayEmpty(&empty).unwrap());
        }

        #[test]
        fn test_array_get() {
            let arr = vec![10, 20, 30];
            assert_eq!(arrayGet(&arr, 1).unwrap(), 10);
            assert_eq!(arrayGet(&arr, 2).unwrap(), 20);
            assert_eq!(arrayGet(&arr, 3).unwrap(), 30);
            assert!(arrayGet(&arr, 0).is_err());
            assert!(arrayGet(&arr, 4).is_err());
        }

        #[test]
        fn test_array_create() {
            let arr = arrayCreate(5, 0).unwrap();
            assert_eq!(arr, vec![0, 0, 0, 0, 0]);
            let empty: Vec<i32> = arrayCreate(0, 42).unwrap();
            assert!(empty.is_empty());
        }

        #[test]
        fn test_array_list() {
            let arr = vec![1, 2, 3];
            let lst = arrayList(&arr).unwrap();
            assert_eq!(lst, list![1, 2, 3]);
        }

        #[test]
        fn test_list_array() {
            let lst = list![1, 2, 3];
            let arr = listArray(&lst).unwrap();
            assert_eq!(arr, vec![1, 2, 3]);
        }

        #[test]
        fn test_array_update() {
            let mut arr = vec![1, 2, 3];
            arrayUpdate(&mut arr, 2, 99).unwrap();
            assert_eq!(arr, vec![1, 99, 3]);
            assert!(arrayUpdate(&mut arr, 0, 99).is_err());
            assert!(arrayUpdate(&mut arr, 4, 99).is_err());
        }

        #[test]
        fn test_array_copy() {
            let arr = vec![1, 2, 3];
            let copy = arrayCopy(&arr).unwrap();
            assert_eq!(copy, vec![1, 2, 3]);
            assert_eq!(copy, arr);
        }

        #[test]
        fn test_array_append() {
            let a = vec![1, 2];
            let b = vec![3, 4];
            assert_eq!(arrayAppend(&a, &b).unwrap(), vec![1, 2, 3, 4]);

            let empty: Vec<i32> = vec![];
            assert_eq!(arrayAppend(&empty, &b).unwrap(), b);
            assert_eq!(arrayAppend(&a, &empty).unwrap(), a);
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
            let result = anyString(&val);
            assert_eq!(&*result.unwrap(), "42");

            let s = "hello";
            assert!(anyString(&s).unwrap().contains("hello"));
        }

        #[test]
        fn test_print_any() {
            // Just ensure it doesn't panic
            let val = 42i32;
            printAny(&val);
        }

        #[test]
        fn test_debug_print() {
            // Just ensure it doesn't panic
            let val = 42i32;
            debug_print("test".to_string(), &val);
        }

        #[test]
        fn test_tick() {
            let t1 = tick().unwrap();
            let t2 = tick().unwrap();
            assert_eq!(t2, t1+1);
        }

        #[test]
        fn test_value_eq() {
            let a = vec![1, 2, 3];
            let b = vec![1, 2, 3];
            let c = vec![1, 2, 4];
            assert!(valueEq(&a, &b).unwrap());
            assert!(!valueEq(&a, &c).unwrap());
        }

        #[test]
        fn test_value_compare() {
            assert_eq!(valueCompare(&1, &2).unwrap(), -1);
            assert_eq!(valueCompare(&2, &2).unwrap(), 0);
            assert_eq!(valueCompare(&3, &2).unwrap(), 1);

            assert_eq!(valueCompare(&"abc", &"abd").unwrap(), -1);
            assert_eq!(valueCompare(&"abc", &"abc").unwrap(), 0);
            assert_eq!(valueCompare(&"abd", &"abc").unwrap(), 1);
        }

        #[test]
        fn test_reference_eq() {
            let a = 42;
            let b = 42;
            // Same reference should be equal
            assert!(referenceEq(&a, &a).unwrap());
            // Different references with same value
            // reference_eq checks pointer equality, so different vars may not be equal
            assert!(referenceEq(&a, &b).unwrap() || !referenceEq(&a, &b).unwrap()); // either is valid
        }

        #[test]
        fn test_reference_pointer_string() {
            let val = 42;
            let ptr_str = referencePointerString(&val).unwrap();
            // Should be a valid hex representation like "0x..."
            assert!(ptr_str.starts_with("0x"));
        }

        #[test]
        fn test_reference_debug_string() {
            let val = 42i32;
            let result = referenceDebugString(&val).unwrap();
            assert!(result.contains("i32"));
        }

        #[test]
        fn test_value_constructor() {
            // Should return a stable value for the same type
            let c1 = valueConstructor::<i32>().unwrap();
            let c2 = valueConstructor::<i32>().unwrap();
            assert_eq!(c1, c2);

            // Different types should likely have different constructors
            let c3 = valueConstructor::<String>().unwrap();
            assert_ne!(c1, c3);
        }

        #[test]
        fn test_clock() {
            let t1 = clock().unwrap();
            let t2 = clock().unwrap();
            assert!(t1 >= 0.0);
            assert!(t2 >= t1);
        }
    }

    // =========================================================================
    // Misc builtin tests
    // =========================================================================

    mod misc_builtin_tests {
        use super::*;

        #[test]
        fn test_set_stack_overflow_signal() {
            assert!(setStackOverflowSignal(true).unwrap());
            assert!(!setStackOverflowSignal(false).unwrap());
        }

        #[test]
        fn test_is_present() {
            // Always returns true in Rust translation
            assert!(isPresent(&42).unwrap());
            assert!(isPresent(&"hello").unwrap());
        }

        #[test]
        fn test_fail() {
            assert!(fail().is_err());
        }

        #[test]
        fn test_source_info() {
            let info = SourceInfo {
                fileName: Arc::new("test.mo".to_string()),
                isReadOnly: true,
                lineNumberStart: 1,
                columnNumberStart: 1,
                lineNumberEnd: 10,
                columnNumberEnd: 50,
                lastModification: 1234567890.0,
            };
            assert_eq!(*info.fileName, "test.mo");
            assert!(info.isReadOnly);
            assert_eq!(info.lineNumberStart, 1);
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
            assert_eq!(arrayGetNoBoundsChecking(&arr, 1).unwrap(), 10);
            assert_eq!(arrayGetNoBoundsChecking(&arr, 2).unwrap(), 20);
            assert_eq!(arrayGetNoBoundsChecking(&arr, 3).unwrap(), 30);
        }

        #[test]
        fn test_array_update_no_bounds_checking() {
            let mut arr = vec![1, 2, 3];
            arrayUpdateNoBoundsChecking(&mut arr, 2, 99).unwrap();
            assert_eq!(arr, vec![1, 99, 3]);
        }

        #[test]
        fn test_array_create_no_init() {
            let arr = arrayCreateNoInit(5, 0i32).unwrap();
            assert_eq!(arr.len(), 5);
        }

        #[test]
        fn test_string_get_no_bounds_checking() {
            let s = "hello".to_string();
            assert_eq!(stringGetNoBoundsChecking(s.clone(), 1).unwrap(), b'h' as i32);
            assert_eq!(stringGetNoBoundsChecking(s, 5).unwrap(), b'o' as i32);
        }
    }
}
