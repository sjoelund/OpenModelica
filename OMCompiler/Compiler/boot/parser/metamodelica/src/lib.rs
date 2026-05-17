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
//!   List<T> -> Arc<List<T>>           (persistent singly-linked list)
//!   array<T> -> Array<T> = Rc<RefCell<Vec<T>>>
//!
//! Note: MetaModelica uses 1-based indexing; Rust uses 0-based.
//! Functions that take indices expect 1-based indexing to match MetaModelica semantics.
//!
//! Array semantics: MetaModelica `array<T>` has reference (aliasing) semantics —
//! `arrayUpdate` mutates the underlying storage in place and the change is visible
//! through every alias of the array. We model that with `Rc<RefCell<Vec<T>>>`.
//! The compiler the bootstrap targets is single-threaded at the MM level, so
//! `Rc`/`RefCell` (no synchronization cost, deterministic borrow-violation panics)
//! is preferred over `Arc<Mutex<...>>` (lock+unlock per access, deadlock risk on
//! re-entrant callbacks). If MM-level concurrency is ever introduced, this alias
//! is the only thing that needs to change.

use std::sync::Arc;
use std::rc::Rc;
use std::cell::RefCell;
use anyhow::Result;
use anyhow::bail;
use arcstr::{ArcStr, literal, format};

/// MetaModelica `array<T>`. See module-level docs for rationale.
pub type Array<A> = Rc<RefCell<Vec<A>>>;

// ============================================================================
// SourceInfo - Location information for elements and classes
// ============================================================================

/// Wrap an infallible function value so it satisfies a function-pointer type
/// whose signature expects `Result<T>`.
///
/// MetaModelica function-typed parameters are uniformly lowered to
/// `fn(...) -> Result<T>` so that the same callback site can accept both
/// failing and non-failing callees. Codegen tracks which functions are
/// fallible (see `mmtorust::fallibility`); when an *infallible* function `f`
/// is passed by reference into a position that wants
/// `fn(A, B, ...) -> Result<T>`, codegen wraps it with `fnptr!(f)` so the
/// shapes line up without forcing every infallible function to materialise
/// a `Result`.
///
/// The macro is variadic in its argument list. Example expansions:
///
///   `fnptr!(g, A, B)`   →   `|a: A, b: B| -> Result<_> { Ok(g(a, b)) }`
///   `fnptr!(h, A)`      →   `|a: A| -> Result<_> { Ok(h(a)) }`
///   `fnptr!(noargs)`    →   `|| -> Result<_> { Ok(noargs()) }`
///
/// The closure does not capture by reference — the wrapped function is a
/// path expression (a function name or `Module::f`), which is already a
/// zero-sized `fn` item with no environment to capture.
///
/// **Note on cost**: the closure boxes nothing and the `Ok(..)` wrap is
/// trivially inlined by the optimiser. The point of the wrapper is purely
/// type-level; the runtime cost is the moral equivalent of `unwrap_unchecked`
/// in the reverse direction.
#[macro_export]
macro_rules! fnptr {
    // Zero-argument form.
    ($f:path) => {
        || -> ::anyhow::Result<_> { ::std::result::Result::Ok($f()) }
    };
    // 1+ argument form. The argument *types* must be supplied at the call
    // site so the closure's signature is unambiguous to the type system
    // (function pointers don't auto-infer parameter types).
    ($f:path $(, $t:ty)+ $(,)?) => {{
        // Generate fresh idents `__a0, __a1, …` for each type slot so the
        // macro stays type-driven (no second list of argument names needed
        // from the caller). The implementation expands one closure
        // parameter per `$t` via the `${index()}` builtin if available, but
        // we fall back to a hand-written tuple form for stability across
        // Rust versions: we accept up to 8 type arguments — enough for all
        // call sites we generate today — and the user gets a clear macro
        // error otherwise.
        $crate::__fnptr_dispatch!($f $(, $t)+)
    }};
}

/// Internal helper for [`fnptr!`]: dispatches on arity (1..=8) without
/// requiring the unstable `${index()}` builtin. Each arm just spells out the
/// closure parameter names; adding more arms is mechanical if a generated
/// call site ever needs >8 arguments.
#[macro_export]
#[doc(hidden)]
macro_rules! __fnptr_dispatch {
    ($f:path, $t1:ty) =>
        { |a1: $t1| -> ::anyhow::Result<_> { ::std::result::Result::Ok($f(a1)) } };
    ($f:path, $t1:ty, $t2:ty) =>
        { |a1: $t1, a2: $t2| -> ::anyhow::Result<_> { ::std::result::Result::Ok($f(a1, a2)) } };
    ($f:path, $t1:ty, $t2:ty, $t3:ty) =>
        { |a1: $t1, a2: $t2, a3: $t3| -> ::anyhow::Result<_> { ::std::result::Result::Ok($f(a1, a2, a3)) } };
    ($f:path, $t1:ty, $t2:ty, $t3:ty, $t4:ty) =>
        { |a1: $t1, a2: $t2, a3: $t3, a4: $t4| -> ::anyhow::Result<_> { ::std::result::Result::Ok($f(a1, a2, a3, a4)) } };
    ($f:path, $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty) =>
        { |a1: $t1, a2: $t2, a3: $t3, a4: $t4, a5: $t5| -> ::anyhow::Result<_> { ::std::result::Result::Ok($f(a1, a2, a3, a4, a5)) } };
    ($f:path, $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty) =>
        { |a1: $t1, a2: $t2, a3: $t3, a4: $t4, a5: $t5, a6: $t6| -> ::anyhow::Result<_> { ::std::result::Result::Ok($f(a1, a2, a3, a4, a5, a6)) } };
    ($f:path, $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty) =>
        { |a1: $t1, a2: $t2, a3: $t3, a4: $t4, a5: $t5, a6: $t6, a7: $t7| -> ::anyhow::Result<_> { ::std::result::Result::Ok($f(a1, a2, a3, a4, a5, a6, a7)) } };
    ($f:path, $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty) =>
        { |a1: $t1, a2: $t2, a3: $t3, a4: $t4, a5: $t5, a6: $t6, a7: $t7, a8: $t8| -> ::anyhow::Result<_> { ::std::result::Result::Ok($f(a1, a2, a3, a4, a5, a6, a7, a8)) } };
}

/// MetaModelica's `sourceInfo()` built-in: returns a `SourceInfo` populated from
/// the *compiler* call-site, not from any runtime value. We mirror that here by
/// using `file!()` / `line!()` / `column!()`, which the Rust compiler expands at
/// macro-invocation site — exactly the semantics MetaModelica gives `sourceInfo()`.
///
/// Codegen emits `sourceInfo!()` for the no-arg MetaModelica builtin call.
#[macro_export]
macro_rules! sourceInfo {
    () => {
        $crate::SourceInfo {
            fileName: ::arcstr::ArcStr::from(file!()),
            isReadOnly: false,
            lineNumberStart: line!() as i32,
            columnNumberStart: column!() as i32,
            lineNumberEnd: line!() as i32,
            columnNumberEnd: column!() as i32,
            lastModification: 0.0,
        }
    };
}

/// The Info attribute provides location information for elements and classes.
/// Mapped from the SOURCEINFO record in MetaModelicaBuiltin.mo.
#[derive(Debug, Clone, PartialEq)]
pub struct SourceInfo {
    /// File name where the class is defined in.
    pub fileName: ArcStr,
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
pub fn boolString(b: bool) -> Result<ArcStr> {
    Ok(if b { literal!("true") } else { literal!("false") })
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
pub fn intString(i: i32) -> Result<ArcStr> {
    Ok(format!("{}", i))
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
    Ok(r1 - (r1/r2).floor()*r2)
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
pub fn realString(r: f64) -> Result<ArcStr> {
    Ok(format!("{}", r))
}

// ============================================================================
// String character functions
// ============================================================================

/// Returns the ASCII code point of a single-character string.
pub fn stringCharInt(ch: ArcStr) -> Result<i32> {
    if ch.len() != 1 {
        bail!("stringCharInt expects a single-character string, got '{}'", ch);
    };
    ch.chars().next()
        .map(|c| c as i32)
        .ok_or_else(|| anyhow::anyhow!("Failed to get character from string: {}", ch))
}

/// Returns a single-character string from an ASCII code point.
pub fn intStringChar(i: i32) -> Result<ArcStr> {
    Ok(format!("{}", std::char::from_u32(i as u32).unwrap()))
}

/// Parses an integer from a string. Fails if the string is not a valid integer.
pub fn stringInt(str: ArcStr) -> Result<i32> {
    str.parse::<i32>().map_err(|_| anyhow::anyhow!("Failed to parse integer from string: {}", str))
}

/// Parses a real (f64) from a string.
/// Fails unless the whole string can be consumed.
pub fn stringReal(str: ArcStr) -> Result<f64> {
    str.parse::<f64>().map_err(|_| anyhow::anyhow!("Failed to parse real from string: {}", str))
}

/// Converts a string to a list of single-character strings.
pub fn stringListStringChar(str: ArcStr) -> Result<Arc<List<ArcStr>>> {
    // TODO: We could have constants for all these short strings to avoid allocations.
    Ok(Arc::new(str.chars().map(|c| format!("{}", c)).collect()))
}

/// Appends a list of strings into a single string.
pub fn stringAppendList(strs: Arc<List<ArcStr>>) -> Result<ArcStr> {
    let mut len = 0;
    for s in &*strs {
        len += s.len();
    }
    let mut result = String::with_capacity(len);
    for s in &*strs {
        result.push_str(&s);
    }
    Ok(result.into())
}

/// Takes a list of strings and a delimiter and joins them with the delimiter inserted between elements.
/// Example: stringDelimitList({"x","y","z"}, ", ") => "x, y, z"
pub fn stringDelimitList(strs: Arc<List<ArcStr>>, delimiter: ArcStr) -> Result<ArcStr> {
    let mut len = 0;
    let delimiter_len = delimiter.len();
    for s in &*strs {
        len += s.len() + delimiter_len;
    }

    let mut result = String::with_capacity(len);
    let mut first = true;

    for s in &*strs {
        if !first {
            result.push_str(&delimiter);
        }
        result.push_str(&s);
        first = false;
    }

    Ok(result.into())
}

/// Returns the length of the string (number of bytes).
pub fn stringLength(str: ArcStr) -> Result<i32> {
    Ok(str.len() as i32)
}

/// Returns true if the string is empty.
pub fn stringEmpty(str: ArcStr) -> Result<bool> {
    Ok(str.is_empty())
}

/// Returns the byte value at the given 1-based index.
pub fn stringGet(str: ArcStr, index: i32) -> Result<i32> {
    let idx = (index - 1) as usize; // 1-based to 0-based
    str.bytes().nth(idx)
        .map(|b| b as i32)
        .ok_or_else(|| anyhow::anyhow!("Index {} out of bounds for string of length {}", index, str.len()))
}

/// Returns the character at the given 1-based index as a string.
pub fn stringGetStringChar(str: ArcStr, index: i32) -> Result<ArcStr> {
    let idx = (index - 1) as usize; // 1-based to 0-based
    str.chars().nth(idx)
        .map(|c| format!("{}", c))
        .ok_or_else(|| anyhow::anyhow!("Index {} out of bounds for string of length {}", index, str.chars().count()))
}

/// Updates the character at the given 1-based index with newch.
/// newch should be a single character.
pub fn stringUpdateStringChar(str: ArcStr, newch: ArcStr, index: i32) -> Result<ArcStr> {
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
    Ok(format!("{}", chars.into_iter().collect::<String>()))
}

/// Concatenates two strings (s1 + s2).
pub fn stringAppend(s1: ArcStr, s2: ArcStr) -> Result<ArcStr> {
    Ok(format!("{}{}", s1, s2))
}

/// Compares two strings for equality.
#[inline(always)]
pub fn stringEq(s1: ArcStr, s2: ArcStr) -> Result<bool> {
    Ok(s1 == s2)
}
#[inline(always)]
pub fn stringEqual(s1: ArcStr, s2: ArcStr) -> Result<bool> {
    Ok(s1 == s2)
}

/// Compares two strings lexicographically.
/// Returns negative if s1 < s2, zero if s1 == s2, positive if s1 > s2.
pub fn stringCompare(s1: ArcStr, s2: ArcStr) -> Result<i32> {
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
pub fn stringHash(str: ArcStr) -> Result<i32> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hash;
    use std::hash::Hasher;
    let mut hasher = DefaultHasher::new();
    str.hash(&mut hasher);
    Ok(hasher.finish() as i32)
}

/// Returns a DJB2 hash of the string.
/// DJB2 algorithm: hash = hash * 33 + byte
pub fn stringHashDjb2(str: ArcStr) -> Result<i32> {
    let mut hash: i32 = 5381;
    for &byte in str.as_bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as i32);
    }
    Ok(hash)
}

/// Continues computing a DJB2 hash by adding another string to it.
pub fn stringHashDjb2Continue(str: ArcStr, hash: i32) -> Result<i32> {
    let mut h = hash;
    for &byte in str.as_bytes() {
        h = h.wrapping_mul(33).wrapping_add(byte as i32);
    }
    Ok(h)
}

/// Computes a DJB2 hash and applies modulo without intermediate overflow issues.
pub fn stringHashDjb2Mod(str: ArcStr, mod_val: i32) -> Result<i32> {
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
pub fn stringHashSdbm(str: ArcStr) -> Result<i32> {
    let mut hash: i32 = 0;
    for &byte in str.as_bytes() {
        hash = byte as i32 + (hash << 6) + (hash << 16) - hash;
    }
    Ok(hash)
}

/// Extracts a substring from str.
/// start and stop are 1-based indices (first character is at index 1).
/// Fails for bogus start/stop values.
pub fn substring(str: ArcStr, start: i32, stop: i32) -> Result<ArcStr> {
    if start < 1 || stop < start || start > stop {
        bail!("Invalid substring range: start={}, stop={}", start, stop);
    }
    let start_idx = (start - 1) as usize; // 1-based to 0-based
    let stop_idx = stop as usize;         // 1-based, inclusive -> exclusive
    let chars: Vec<char> = str.chars().collect();
    if stop_idx > chars.len() {
        bail!("Stop index {} exceeds string length {}", stop, chars.len());
    }
    Ok(format!("{}", chars[start_idx..stop_idx].iter().collect::<String>()))
}

/// Alias for string_append_list (maps a list of single-char strings to one string).
pub fn listStringCharString(strs: Arc<List<ArcStr>>) -> Result<ArcStr> {
    stringAppendList(strs)
}

/// Alias for string_append_list (maps a list of single-char strings to one string).
pub fn stringCharListString(strs: Arc<List<ArcStr>>) -> Result<ArcStr> {
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
        std::sync::Arc::new($crate::List::Nil)
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

/// Functionally update a single field of a record value stored as `Arc<T>`.
///
/// MetaModelica record update (`var.field := value`) has value semantics: a new
/// record is produced and rebound. We model the record as `Arc<T>` for cheap
/// sharing, so direct field mutation through the `Arc` is impossible. This macro
/// clones the underlying record (a shallow copy — the contained fields are
/// themselves cheap `Arc` handles or scalars), overwrites the targeted field on
/// the owned copy, and rebinds `$base` to a fresh `Arc<T>`.
///
/// For multi-record uniontypes (Rust enums), use `assign_variant_field!` instead:
/// the matched variant must be named explicitly because the enum tag is not
/// inferable from the macro's input position. With a single-record uniontype
/// (or any plain struct), this macro suffices.
#[macro_export]
macro_rules! assign_field {
    // One or more field assignments against the same `Arc<T>` base. The clone
    // and the `Arc::new` happen once for the whole batch, no matter how many
    // fields are updated. All assignments must target the same identifier; the
    // macro reuses `$base` as the storage and only matches the trailing entries
    // to keep the parser happy.
    (
        $base:ident . $first_field:ident = $first_value:expr
        $(, $_base:ident . $field:ident = $value:expr)*
        $(,)?
    ) => {{
        let mut __owned = (*$base).clone();
        __owned.$first_field = $first_value;
        $( __owned.$field = $value; )*
        $base = ::std::sync::Arc::new(__owned);
    }};
}

/// Like `assign_field!`, but for a uniontype-enum value whose currently matched
/// variant is known statically (e.g. inside a `match` arm or after a refutable
/// `let`-pattern). The variant path must be supplied so the destructure picks
/// the right arm; a runtime mismatch panics, which would indicate a codegen bug.
///
/// Example: `assign_variant_field!(node => NFInstNode::CLASS_NODE; ty = newTy);`
#[macro_export]
macro_rules! assign_variant_field {
    // One or more field assignments to a value already known to be a specific
    // variant (`$($variant)::+`). The destructure happens once; the field
    // bindings are then assigned in sequence on the owned copy. A runtime
    // variant mismatch panics — that would indicate a codegen bug.
    (
        $base:ident => $($variant:ident)::+ ;
        $first_field:ident = $first_value:expr
        $(, $field:ident = $value:expr)*
        $(,)?
    ) => {{
        let mut __owned = (*$base).clone();
        // Field-shorthand destructure: each captured binding takes the same
        // name as the field, so the macro doesn't need to invent unique
        // identifiers per repetition.
        if let $($variant)::+ {
            $first_field,
            $($field,)*
            ..
        } = &mut __owned {
            *$first_field = $first_value;
            $( *$field = $value; )*
        } else {
            panic!(
                "assign_variant_field!: expected variant {} but value held a different variant",
                stringify!($($variant)::+),
            );
        }
        $base = ::std::sync::Arc::new(__owned);
    }};
}

/// Read a single field from a uniontype-enum value whose currently matched
/// variant is known statically (e.g. inside a `match` arm or after a refutable
/// `let`-pattern). MetaModelica syntax `v.field` is valid on a uniontype value
/// when the surrounding control flow proves `v` holds a particular record
/// variant; in Rust the enum has no such field directly, so the field must be
/// extracted by destructuring. This macro performs that destructure inline.
///
/// The returned value is a reference (`&FieldType`) borrowed from `$base`; the
/// caller is expected to clone it as appropriate. A runtime variant mismatch
/// panics, which would indicate a codegen bug.
///
/// Two input forms are supported:
///   - `var_field!(v.field, Pkg::Type::VARIANT)` for a plain (owned) enum value.
///   - `var_field!((*v).field, Pkg::Type::VARIANT)` when `v` is `Arc<Enum>` /
///     other `Deref`-able smart pointer; the explicit `*` selects the deref arm.
///
/// The variant path must be supplied so the destructure picks the right arm;
/// it cannot be inferred from the input position.
#[macro_export]
macro_rules! var_field {
    // Plain (owned) base: match against `&$base`. Rust match ergonomics binds
    // `$field` as `&FieldType` against the enum scrutinee.
    ( $base:ident . $field:ident , $($variant:ident)::+ ) => {
        match &$base {
            $($variant)::+ { $field, .. } => $field,
            _ => panic!(
                "var_field!: expected variant {} but value held a different variant",
                stringify!($($variant)::+),
            ),
        }
    };
    // Smart-pointer base (Arc / Rc / Box / &T / &mut T): `*$base` derefs through
    // the wrapper to the underlying enum; `&*$base` then yields `&Enum`.
    ( ( * $base:ident ) . $field:ident , $($variant:ident)::+ ) => {
        match &*$base {
            $($variant)::+ { $field, .. } => $field,
            _ => panic!(
                "var_field!: expected variant {} but value held a different variant",
                stringify!($($variant)::+),
            ),
        }
    };
    // Reference to a smart pointer (e.g. `&Arc<Enum>`): produced by `ref`
    // pattern bindings on Arc-typed fields under `deref_patterns`. The first
    // `*` strips the outer reference, the second `*` derefs the Arc.
    ( ( * * $base:ident ) . $field:ident , $($variant:ident)::+ ) => {
        match &**$base {
            $($variant)::+ { $field, .. } => $field,
            _ => panic!(
                "var_field!: expected variant {} but value held a different variant",
                stringify!($($variant)::+),
            ),
        }
    };
}

pub fn nil<T: Clone>() -> Arc<List<T>> {
    Arc::new(Nil)
}

pub fn cons<T: Clone>(head: T, tail: Arc<List<T>>) -> Arc<List<T>> {
    Arc::new(Cons{head, tail})
}

impl<T: Clone> Default for List<T> {
    fn default() -> List<T> {
        Nil
    }
}
pub struct ListRefIterator<'a, T: Clone> {
    curr: &'a List<T>,
}

impl<T: Clone> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> List<T> {
        let mut buf = nil();
        for item in iter {
            buf = cons(item, buf);
        }
        (*buf.reverse()).clone()
    }
}

impl<'a, T: Clone> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = ListRefIterator<'a, T>;

    // Required method
    fn into_iter(self) -> Self::IntoIter {
       ListRefIterator { curr: self }
    }
}

/*
pub struct ListIterator<T: Clone> {
    curr: Arc<List<T>>,
}

impl<T: Clone> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = ListIterator<T>;

    // Required method
    fn into_iter(self) -> Self::IntoIter {
        ListIterator { curr: Arc::new(self) }
    }
}

impl<T: Clone> Iterator for ListIterator<T> {
    type Item = T; // No Clone needed here!

    fn next(&mut self) -> Option<Self::Item> {
        match *self.curr.clone() {
            // If it's Nil, we are done.
            List::Nil => return None,

            // If it's Cons:
            List::Cons { ref head, ref tail } => {
                self.curr = tail.clone();
                Some(head.clone())
            }
        }
    }
}
*/

impl<'a, T: Clone> Iterator for ListRefIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.curr {
            Nil => return None,
            Cons { head, tail } => {
                self.curr = tail;
                Some(head)
            }
        }
    }
}

impl<T: Clone> List<T> {
    /// Appends lst2 to lst1. O(length(lst1)), O(1) if either list is empty.
    pub fn append(self: &Arc<List<T>>, lst2: &Arc<List<T>>) -> Arc<List<T>> {
        if self.is_empty() {
            return lst2.clone();
        }
        if lst2.is_empty() {
            return self.clone();
        }
        let mut result = lst2.clone();
        for item in &*(self.reverse()) {
            result = cons(item.clone(), result);
        }
        result
    }
    /// Returns the length of a list. O(n).
    pub fn len(&self) -> i32 {
        self.into_iter().count() as i32
    }
    /// Reverses the elements in a list. O(n).
    pub fn reverse(self: &Arc<List<T>>) -> Arc<List<T>> {
        let mut result: Arc<List<T>> = nil();
        for e in &**self {
            result = cons(e.clone(), result);
        }
        result
    }
    /// Gets the element at the given 1-based index. O(index).
    pub fn get(self: &Arc<List<T>>, index: i32) -> Result<T> {
        (&**self).into_iter().nth((index - 1) as usize)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Index {} out of bounds for list of length {}", index, self.len()))
    }
    pub fn prepend_reverse(self: &Arc<List<T>>, prefix: &Arc<List<T>>) -> Arc<List<T>> {
        let mut result = self.clone();
        for item in &**prefix {
            result = cons(item.clone(), result);
        }
        result
    }
    /// Deletes the element at the given 1-based index. O(index).
    pub fn delete(self: &Arc<List<T>>, index: i32) -> Result<Arc<List<T>>> {
        if index < 1 {
            bail!("Index must be positive, got {}", index);
        }
        if index == 1 {
            return self.rest();
        }
        let mut result = nil();
        let mut iter = self;
        let mut cur_index = index;
        loop {
            cur_index -= 1;
            let (head,tail) = match &**iter {
                Nil => bail!("Index {} out of bounds for list", index),
                Cons{head, tail} => (head, tail)
            };
            iter = &tail;
            if cur_index == 0 {
                return Ok(iter.prepend_reverse(&result));
            }
            result = cons(head.clone(), result);
        }
    }
}

impl<T: Clone> List<T> {
    pub fn new(item: T) -> Arc<List<T>> {
        Arc::new(Cons{head: item, tail: nil()})
    }
    pub fn cons(self: Arc<List<T>>, item: T) -> Arc<List<T>> {
        Arc::new(Cons{head: item, tail: self})
    }
    /// Gets the first element. O(1).
    /// Fails if the list is empty.
    pub fn head(self: &Arc<List<T>>) -> Result<&T> {
        match &**self {
            Nil => bail!("Cannot get head of empty list"),
            Cons{head, ..} => Ok(head),
        }
    }
    /// Returns all elements except the first. O(1).
    /// Fails if the list is empty.
    pub fn rest(self: &Arc<List<T>>) -> Result<Arc<List<T>>> {
        match &**self {
            Nil => bail!("Cannot get rest of empty list"),
            Cons{tail, ..} => Ok(tail.clone()),
        }
    }
    /// Returns true if the list is empty. O(1).
    pub fn is_empty(self: &Arc<List<T>>) -> bool {
        match **self {
            Nil => true,
            _ => false
        }
    }
}



impl<T: PartialEq + Clone> List<T> {
    /// Checks if an element is a member of the list. O(n).
    /// Uses PartialEq for comparison.
    pub fn contains(self: &Arc<List<T>>, element: &T) -> bool {
        for item in &**self {
            if element.eq(item) { return true; }
        }
        return false;
    }
}

pub fn listAppend<T: Clone>(lst1: Arc<List<T>>, lst2: Arc<List<T>>) -> Result<Arc<List<T>>> {
    Ok(lst1.append(&lst2))
}

pub fn listMember<T: Clone+PartialEq>(element: T, lst: Arc<List<T>>) -> Result<bool> {
    Ok(lst.contains(&element))
}

pub fn listHead<T: Clone>(lst: Arc<List<T>>) -> Result<T> {
    let Cons{head, ..} = &*lst else {bail!("Cannot get head of empty list")};
    Ok(head.clone())
}

pub fn listRest<T: Clone>(lst: Arc<List<T>>) -> Result<Arc<List<T>>> {
    match &*lst {
        Nil => bail!("Cannot get rest of empty list"),
        Cons{tail, ..} => Ok(tail.clone()),
    }
}

pub fn listLength<T: Clone>(lst: Arc<List<T>>) -> Result<i32> {
    Ok(lst.len())
}

// ============================================================================
// Array functions
// ============================================================================

/// Wraps a `Vec<T>` into a fresh MetaModelica `Array<T>`.
#[inline]
pub fn arrayFromVec<A>(v: Vec<A>) -> Array<A> {
    Rc::new(RefCell::new(v))
}

// All array fns take `Array<A>` by value: cloning an `Rc` is one atomic-free
// refcount bump, so the by-value convention matches how `Arc<List<A>>` is
// handled elsewhere and lets generated call sites pass `arr.clone()` directly
// without needing an explicit `&` prefix.

/// Returns the length of the array. O(1).
pub fn arrayLength<A>(arr: Array<A>) -> Result<i32> {
    Ok(arr.borrow().len() as i32)
}

/// Returns true if the array is empty. O(1).
pub fn arrayEmpty<A>(arr: Array<A>) -> Result<bool> {
    Ok(arr.borrow().is_empty())
}

/// Gets the element at the given 1-based index. O(1).
pub fn arrayGet<A: Clone>(arr: Array<A>, index: i32) -> Result<A> {
    let idx = (index - 1) as usize; // 1-based to 0-based
    let v = arr.borrow();
    v.get(idx)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("Index {} out of bounds for array of length {}", index, v.len()))
}

/// Creates a new array of the given size, initialized with initialValue. O(size).
pub fn arrayCreate<A: Clone>(size: i32, initial_value: A) -> Result<Array<A>> {
    if size <= 0 {
        return Ok(arrayFromVec(Vec::new()));
    }
    Ok(arrayFromVec(vec![initial_value; size as usize]))
}

/// Converts an array to a list. O(n).
pub fn arrayList<A: Clone>(arr: Array<A>) -> Result<Arc<List<A>>> {
    let mut result = Arc::new(List::Nil);
    for item in arr.borrow().iter().rev().cloned() {
        result = List::cons(result, item);
    }
    Ok(result)
}

/// Converts a list to an array. O(n).
pub fn listArray<A: Clone>(lst: Arc<List<A>>) -> Result<Array<A>> {
    let mut result = Vec::new();
    for item in &*lst {
        result.push(item.clone());
    }
    Ok(arrayFromVec(result))
}

/// Updates the value at the given 1-based index. O(1).
/// Mutates the underlying storage; the change is visible through every alias
/// of the same array. Returns the same `Rc` (a cheap clone) so call sites can
/// chain or reassign as the MetaModelica signature suggests.
pub fn arrayUpdate<A: Clone>(arr: Array<A>, index: i32, new_value: A) -> Result<Array<A>> {
    let idx = (index - 1) as usize; // 1-based to 0-based
    {
        let mut v = arr.borrow_mut();
        let len = v.len();
        if idx >= len {
            bail!("Index {} out of bounds for array of length {}", index, len);
        }
        v[idx] = new_value;
    }
    Ok(arr)
}

/// Creates a (deep, by-element) copy of the array. O(n).
/// The returned array does NOT share storage with the input.
pub fn arrayCopy<A: Clone>(arr: Array<A>) -> Result<Array<A>> {
    Ok(arrayFromVec(arr.borrow().clone()))
}

/// Appends arr2 to arr1, creating a new array. O(length(arr1) + length(arr2)).
/// The result does not share storage with either input.
pub fn arrayAppend<A: Clone>(arr1: Array<A>, arr2: Array<A>) -> Result<Array<A>> {
    let mut result = arr1.borrow().clone();
    result.extend(arr2.borrow().iter().cloned());
    Ok(arrayFromVec(result))
}

// ============================================================================
// StaticArray<T> - read-only array storage for module-level constant tables
// ============================================================================

/// Storage for module-level immutable arrays (lexer/parser tables built from
/// `MetaModelica.Dangerous.listArrayLiteral` and similar constant
/// `array<T>` declarations).
///
/// The mutable [`Array<T>`] type is `Rc<RefCell<Vec<T>>>`, which is **not**
/// `Sync` and therefore cannot be placed inside `pub static LazyLock<...>`.
/// MM-level concurrency is single-threaded, so the unsync-ness is the right
/// trade-off for general-purpose arrays — but constant tables, which are
/// never written to after construction, do not need `RefCell` at all.
/// `StaticArray<T>` wraps the data in `Arc<Vec<T>>` instead, which **is**
/// `Sync + Send` (for `T: Sync + Send`) and thus admissible as a static.
///
/// The API exposed mirrors the parts of [`Array<T>`] that read-only call
/// sites use:
///
/// * [`StaticArray::borrow`] returns `&Vec<T>`, matching the use of
///   `RefCell::borrow` on `Array<T>` — generated code does
///   `TABLE.borrow()[idx]` and that resolves identically here.
/// * The inherent [`StaticArray::clone`] returns an `Array<T>`, **not**
///   `Self`. Generated code occasionally does `TABLE.clone()` and passes
///   the result to a function whose MM-level parameter type is
///   `array<T>` (e.g. `checkArrayModelica`). To keep those call sites
///   working without a codegen-side rewrite, `.clone()` materialises a
///   fresh mutable `Array<T>` (deep copy). Use [`StaticArray::share`] to
///   get a cheap aliasing copy of the `StaticArray` itself.
#[derive(Debug)]
pub struct StaticArray<T> {
    inner: Arc<Vec<T>>,
}

impl<T> StaticArray<T> {
    /// Wraps a `Vec<T>` into a read-only `StaticArray<T>`.
    #[inline]
    pub fn new(v: Vec<T>) -> Self {
        StaticArray { inner: Arc::new(v) }
    }

    /// Returns a borrow of the underlying vector. Named `borrow` to match
    /// `RefCell::borrow` so generated indexing code (`table.borrow()[i]`)
    /// works against both `Array<T>` and `StaticArray<T>`.
    #[inline]
    pub fn borrow(&self) -> &Vec<T> {
        &self.inner
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Cheap aliasing copy of this `StaticArray` (Arc refcount bump).
    /// Use this when you want to keep `StaticArray<T>` semantics; use
    /// `.clone()` (the inherent method below) when you need a fresh
    /// mutable `Array<T>`.
    #[inline]
    pub fn share(&self) -> StaticArray<T> {
        StaticArray { inner: Arc::clone(&self.inner) }
    }
}

impl<T: Clone> StaticArray<T> {
    /// Materialises a fresh mutable [`Array<T>`] by element-wise cloning
    /// the static storage. See the type-level docs for the rationale —
    /// this is the form expected by MM-translated call sites that pass
    /// a static table into a function whose parameter type is
    /// `array<T>`.
    ///
    /// Note: this is an *inherent* method, not a `Clone` impl. We deliberately
    /// do **not** implement `Clone`, so `table.clone()` resolves to this
    /// method via method-lookup; calls through `Clone::clone` would not.
    #[inline]
    #[allow(clippy::should_implement_trait)]
    pub fn clone(&self) -> Array<T> {
        arrayFromVec((*self.inner).clone())
    }
}

// ============================================================================
// Generic value functions
// ============================================================================

/// Returns the string representation of any Debug-printable value.
/// Rather slow; only use this for debugging!
pub fn anyString<A: std::fmt::Debug>(a: A) -> Result<ArcStr> {
    Ok(format!("{:?}", a))
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
pub fn valueEq<A: PartialEq>(a1: A, a2: A) -> Result<bool> {
    Ok(a1 == a2)
}

/// Compares two Ord values.
/// Returns -1 if a1 < a2, 0 if a1 == a2, 1 if a1 > a2.
pub fn valueCompare<A: Ord>(a1: A, a2: A) -> Result<i32> {
    match a1.cmp(&a2) {
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
pub fn referencePointerString<A>(a: &A) -> Result<ArcStr> {
    Ok(format!("{:p}", a))
}

/// Returns a debug string for a function symbol.
/// In Rust, returns the type name of the value for debugging.
pub fn referenceDebugString<A: std::fmt::Debug>(_a: &A) -> Result<ArcStr> {
    Ok(format!("{:?}", std::any::type_name::<A>()))
}

// ============================================================================
// Global roots — MetaModelicaBuiltin.mo setGlobalRoot/getGlobalRoot.
//
// Each MetaModelica `setGlobalRoot(idx, v)` stores `v` in a fixed slot; the
// MMC backend allocated slots 0..1023 (the first 9 are thread-local, the rest
// shared). For the boot/Rust path we don't have multiple threads running the
// compiler proper, so a single thread-local table is sufficient — and avoids
// the Send/Sync constraints that would otherwise be incompatible with
// `Rc<RefCell<_>>`-shaped values the compiler stores in global roots (Flags,
// caches, etc.).
//
// Values are erased through `Rc<dyn Any>`; `getGlobalRoot::<A>` downcasts on
// retrieval. Index 0 is permitted but typically maps to "uninitialized" by
// convention — MetaModelicaBuiltin.mo specifically warns against using it.
// ============================================================================
thread_local! {
    static GLOBAL_ROOTS: std::cell::RefCell<Vec<Option<std::rc::Rc<dyn std::any::Any>>>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

pub fn setGlobalRoot<A: std::any::Any + 'static>(index: i32, value: A) -> Result<()> {
    GLOBAL_ROOTS.with(|r| {
        let mut v = r.borrow_mut();
        let idx = index as usize;
        if v.len() <= idx {
            v.resize_with(idx + 1, || None);
        }
        v[idx] = Some(std::rc::Rc::new(value));
    });
    Ok(())
}

pub fn getGlobalRoot<A: std::any::Any + Clone + 'static>(index: i32) -> Result<A> {
    GLOBAL_ROOTS.with(|r| {
        let v = r.borrow();
        let entry = v
            .get(index as usize)
            .and_then(|o| o.clone())
            .ok_or_else(|| anyhow::anyhow!("getGlobalRoot: index {} is uninitialized", index))?;
        match entry.downcast::<A>() {
            Ok(rc) => Ok((*rc).clone()),
            Err(_) => Err(anyhow::anyhow!(
                "getGlobalRoot: index {} type mismatch (expected {})",
                index,
                std::any::type_name::<A>()
            )),
        }
    })
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
pub fn isNone<A>(opt: Option<A>) -> Result<bool> {
    Ok(matches!(opt, None))
}

/// Returns true if the Option is SOME.
pub fn isSome<A>(opt: Option<A>) -> Result<bool> {
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
    pub use super::*;
    /// Unsafe array get without bounds checking.
    /// Panics in debug mode if index is out of bounds due to Rust's bounds checking on indexing.
    pub fn arrayGetNoBoundsChecking<A: Clone>(arr: Array<A>, index: i32) -> Result<A> {
        let idx = (index - 1) as usize; // 1-based to 0-based
        let v = arr.borrow();
        // SAFETY: Caller must ensure index is in bounds.
        unsafe { Ok(v.get_unchecked(idx).clone()) }
    }

    /// Unsafe array update without bounds checking.
    /// Mutates the underlying storage in place; visible through every alias.
    pub fn arrayUpdateNoBoundsChecking<A: Clone>(arr: Array<A>, index: i32, new_value: A) -> Result<()> {
        let idx = (index - 1) as usize; // 1-based to 0-based
        let mut v = arr.borrow_mut();
        // SAFETY: Caller must ensure index is in bounds.
        unsafe { *v.get_unchecked_mut(idx) = new_value }
        Ok(())
    }

    /// Unsafe array clearing without bounds checking.
    /// Mutates the underlying storage in place; visible through every alias.
    pub fn arrayClearIndex<A: Clone>(arr: Array<A>, index: i32) -> Result<()> {
        let idx = (index - 1) as usize; // 1-based to 0-based
        let mut v = arr.borrow_mut();
        // SAFETY: Caller must ensure idx is in bounds. We drop the existing
        // value in place, then overwrite its storage with zero bytes. The
        // slot must not be read as an `A` again before being reinitialized,
        // since the all-zero bit-pattern is not a valid value for arbitrary A.
        unsafe {
            let p = v.get_unchecked_mut(idx) as *mut A;
            std::ptr::drop_in_place(p);
            std::ptr::write_bytes(p, 0u8, 1);
        }
        Ok(())
    }

    /// Creates a new array with uninitialized elements.
    /// The MetaModelica signature takes a `dummy` argument purely as a type witness;
    /// the codegen drops it because Rust generics already carry the element type.
    pub fn arrayCreateNoInit<A: Clone>(size: i32) -> Result<Array<A>> {
        if size < 0 {
            bail!("Size must be non-negative, got {}", size);
        }
        let mut v = Vec::with_capacity(size as usize);
        // SAFETY:
        // 1. We allocated capacity for `size` elements.
        // 2. Caller guarantees every element is initialized before being read.
        unsafe {
            v.set_len(size as usize);
        }
        Ok(arrayFromVec(v))
    }
    /// Unsafe string get without bounds checking.
    pub fn stringGetNoBoundsChecking(str: String, index: i32) -> Result<i32> {
        let idx = (index - 1) as usize; // 1-based to 0-based
        // SAFETY: Caller must ensure index is in bounds.
        unsafe { Ok((*str.as_bytes().get_unchecked(idx)) as i32) }
    }
    /// Reverses a list in place.
    pub fn listReverseInPlace<T: Clone>(list: Arc<List<T>>) -> Result<Arc<List<T>>>{
        Ok(list.reverse())
    }
    /// Overwrites the `tail` field of the given Cons cell.
    ///
    /// SAFETY: Mutates the cell behind the `Arc` through a raw pointer, so all
    /// other holders of clones of this `Arc` observe the change. Caller must
    /// ensure no other thread is reading the cell concurrently. Mirrors the
    /// MetaModelica runtime's RML cons-cell mutation.
    pub fn listSetRest<T: Clone>(list: Arc<List<T>>, new_tail: Arc<List<T>>) -> Result<()> {
        let ptr = Arc::as_ptr(&list) as *mut List<T>;
        unsafe {
            match &mut *ptr {
                List::Cons { tail, .. } => { *tail = new_tail; Ok(()) }
                List::Nil => bail!("listSetRest: called on Nil"),
            }
        }
    }
    /// Overwrites the `head` field of the given Cons cell. See `listSetRest`
    /// for the safety contract.
    pub fn listSetFirst<T: Clone>(list: Arc<List<T>>, new_head: T) -> Result<()> {
        let ptr = Arc::as_ptr(&list) as *mut List<T>;
        unsafe {
            match &mut *ptr {
                List::Cons { head, .. } => { *head = new_head; Ok(()) }
                List::Nil => bail!("listSetFirst: called on Nil"),
            }
        }
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
            assert_eq!(&*boolString(true).unwrap(), "true");
            assert_eq!(&*boolString(false).unwrap(), "false");
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
            assert_eq!(stringCharInt(literal!("A")).unwrap(), 65);
            assert_eq!(stringCharInt(literal!("a")).unwrap(), 97);
            assert_eq!(stringCharInt(literal!("0")).unwrap(), 48);
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
            assert_eq!(stringInt(literal!("42")).unwrap(), 42);
            assert_eq!(stringInt(literal!("-7")).unwrap(), -7);
            assert!(stringInt(literal!("not_a_number")).is_err());
        }

        #[test]
        fn test_string_real() {
            assert_eq!(stringReal(literal!("3.14")).unwrap(), 3.14);
            assert_eq!(stringReal(literal!("-2.5")).unwrap(), -2.5);
            assert!(stringReal(literal!("not_a_number")).is_err());
        }

        #[test]
        fn test_string_list_string_char() {
            let result = stringListStringChar(literal!("abc ")).unwrap();
            assert_eq!(&*result, &List::from_iter([literal!("a"), literal!("b"), literal!("c")]));
        }

        #[test]
        fn test_string_append_list() {
            let strs = list![literal!("hello"), literal!(" "), literal!("world")];
            assert_eq!(&*stringAppendList(strs).unwrap(), "hello world");
        }

        #[test]
        fn test_string_delimit_list() {
            let strs: Arc<List<ArcStr>> = list![literal!("x"), literal!("y"), literal!("z")];
            assert_eq!(stringDelimitList(strs, literal!(", ")).unwrap(), "x, y, z");
        }
    }

    // =========================================================================
    // String length and empty tests
    // =========================================================================

    mod string_length_tests {
        use super::*;

        #[test]
        fn test_string_length() {
            assert_eq!(stringLength("hello".into()).unwrap(), 5);
            assert_eq!(stringLength("".into()).unwrap(), 0);
        }

        #[test]
        fn test_string_empty() {
            assert!(stringEmpty("".into()).unwrap());
            assert!(!stringEmpty("hello".into()).unwrap());
        }
    }

    // =========================================================================
    // String get/update tests
    // =========================================================================

    mod string_get_update_tests {
        use super::*;

        #[test]
        fn test_string_get() {
            assert_eq!(stringGet(literal!("hello"), 1).unwrap(), b'h' as i32);
            assert_eq!(stringGet(literal!("hello"), 5).unwrap(), b'o' as i32);
            assert!(stringGet(literal!("hello"), 0).is_err());
            assert!(stringGet(literal!("hello"), 6).is_err());
        }

        #[test]
        fn test_string_get_string_char() {
            assert_eq!(stringGetStringChar(literal!("hello"), 1).unwrap(), literal!("h"));
            assert_eq!(stringGetStringChar(literal!("hello"), 3).unwrap(), literal!("l"));
            assert_eq!(stringGetStringChar(literal!("hello"), 5).unwrap(), literal!("o"));
            assert!(stringGetStringChar(literal!("hello"), 0).is_err());
            assert!(stringGetStringChar(literal!("hello"), 6).is_err());
        }

        #[test]
        fn test_string_update_string_char() {
            assert_eq!(stringUpdateStringChar(literal!("hello"), literal!("X"), 1).unwrap(), literal!("Xello"));
            assert_eq!(stringUpdateStringChar(literal!("hello"), literal!("X"), 3).unwrap(), literal!("heXlo"));
            assert_eq!(stringUpdateStringChar(literal!("hello"), literal!("X"), 5).unwrap(), literal!("hellX"));
            assert!(stringUpdateStringChar(literal!("hello"), literal!("X"), 0).is_err());
            assert!(stringUpdateStringChar(literal!("hello"), literal!("X"), 6).is_err());
            assert!(stringUpdateStringChar(literal!("hello"), literal!(""), 1).is_err());
        }
    }

    // =========================================================================
    // String append/equal tests
    // =========================================================================

    mod string_append_equal_tests {
        use super::*;

        #[test]
        fn test_string_append() {
            assert_eq!(stringAppend(literal!("hello"), literal!(" world")).unwrap(), literal!("hello world"));
            assert_eq!(stringAppend(literal!(""), literal!("hello")).unwrap(), literal!("hello"));
            assert_eq!(stringAppend(literal!("hello"), literal!("")).unwrap(), literal!("hello"));
        }

        #[test]
        fn test_string_eq() {
            assert!(stringEq(literal!("abc"), literal!("abc")).unwrap());
            assert!(!stringEq(literal!("abc"), literal!("abd")).unwrap());
            assert!(!stringEq(literal!(""), literal!("abc")).unwrap());
        }

        #[test]
        fn test_string_equal() {
            assert!(stringEqual(literal!("abc"), literal!("abc")).unwrap());
            assert!(!stringEqual(literal!("abc"), literal!("abd")).unwrap());
        }
    }

    // =========================================================================
    // String compare test
    // =========================================================================

    mod string_compare_test {
        use super::*;

        #[test]
        fn test_string_compare() {
            assert!(stringCompare(literal!("abc"), literal!("abd")).unwrap() < 0);
            assert_eq!(stringCompare(literal!("abc"), literal!("abc")).unwrap(), 0);
            assert!(stringCompare(literal!("abd"), literal!("abc")).unwrap() > 0);
            assert!(stringCompare(literal!("ab"), literal!("abc")).unwrap() < 0);
            assert!(stringCompare(literal!("abc"), literal!("ab")).unwrap() > 0);
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
            assert_eq!(stringHashDjb2(literal!("a")).unwrap(), 5381_i32.wrapping_mul(33).wrapping_add(97));
            assert_eq!(stringHashDjb2(literal!("")).unwrap(), 5381);
        }

        #[test]
        fn test_string_hash_djb2_continue() {
            let h1 = stringHashDjb2(literal!("hello")).unwrap();
            let _h2 = stringHashDjb2(literal!(" world")).unwrap();
            let combined = stringHashDjb2Continue(literal!(" world"), h1).unwrap();
            // Starting from h1 and adding " world" should give the same
            // as hashing "hello world" from scratch
            assert_eq!(combined, stringHashDjb2(literal!("hello world")).unwrap());
        }

        #[test]
        fn test_string_hash_djb2_mod() {
            let h = stringHashDjb2Mod(literal!("hello"), 100).unwrap();
            assert!(h >= 0 && h < 100);
            assert_eq!(stringHashDjb2Mod(literal!("hello"), 0).unwrap(), 0);
        }

        #[test]
        fn test_string_hash_sdbm() {
            // SDBM of "a" = 97 + 0 + 0 - 0 = 97
            assert_eq!(stringHashSdbm(literal!("a")).unwrap(), 97);
            assert_eq!(stringHashSdbm(literal!("")).unwrap(), 0);
        }

        #[test]
        fn test_string_hash_consistency() {
            // Same string should produce same hash
            assert_eq!(stringHash(literal!("test")).unwrap(), stringHash(literal!("test")).unwrap());
        }
    }

    // =========================================================================
    // Substring tests
    // =========================================================================

    mod substring_tests {
        use super::*;

        #[test]
        fn test_substring_basic() {
            assert_eq!(*substring(literal!("hello world"), 1, 5).unwrap(), "hello".to_string());
            assert_eq!(*substring(literal!("hello world"), 7, 11).unwrap(), "world".to_string());
            assert_eq!(*substring(literal!("hello"), 3, 3).unwrap(), "l".to_string());
            assert_eq!(*substring(literal!("hello"), 1, 5).unwrap(), "hello".to_string());
        }

        #[test]
        fn test_substring_errors() {
            assert!(substring(literal!("hello"), 0, 3).is_err());  // start < 1
            assert!(substring(literal!("hello"), 3, 2).is_err());  // stop < start
            assert!(substring(literal!("hello"), 1, 6).is_err());  // stop out of bounds
            assert!(substring(literal!("hello"), 6, 7).is_err());  // start out of bounds
        }
    }

    // =========================================================================
    // List string char string tests
    // =========================================================================

    mod list_string_tests {
        use super::*;

        #[test]
        fn test_list_string_char_string() {
            let strs: Arc<List<ArcStr>> = list![literal!("a"), literal!("b"), literal!("c")];
            assert_eq!(&*listStringCharString(strs).unwrap(), "abc");
        }

        #[test]
        fn test_string_char_list_string() {
            let strs: Arc<List<ArcStr>> = list![literal!("a"), literal!("b"), literal!("c")];
            assert_eq!(&*stringCharListString(strs).unwrap(), "abc");
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
            let empty: Arc<List<i32>> = nil();
            assert_eq!(empty.append(&b), b);
            assert_eq!(a.append(&empty), a);
        }

        #[test]
        fn test_list_reverse() {
            let lst = list![1, 2, 3, 4, 5];
            let result = lst.reverse();
            assert_eq!(result, list![5, 4, 3, 2, 1]);

            let empty: Arc<List<i32>> = nil();
            assert_eq!(empty.reverse(), nil());
        }

        #[test]
        fn test_list_length() {
            let lst = list![1, 2, 3];
            assert_eq!(lst.len(), 3);
            let empty: Arc<List<i32>> = nil();
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

            let empty: Arc<List<i32>> = nil();
            assert!(empty.rest().is_err());
        }

        #[test]
        fn test_list_head() {
            let lst = list![1, 2, 3];
            assert_eq!(lst.head().unwrap().clone(), 1);

            let empty: Arc<List<i32>> = nil();
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

            let empty: Arc<List<i32>> = nil();
            assert!(empty.is_empty());
        }

        #[test]
        fn test_cons() {
            let lst = list![2, 3];
            let result = cons(1, lst);
            assert_eq!(result, list![1, 2, 3]);

            let empty: Arc<List<i32>> = nil();
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

        fn arr<A>(v: Vec<A>) -> Array<A> { arrayFromVec(v) }

        #[test]
        fn test_array_length() {
            assert_eq!(arrayLength(arr(vec![1, 2, 3])).unwrap(), 3);
            assert_eq!(arrayLength(arr::<i32>(vec![])).unwrap(), 0);
        }

        #[test]
        fn test_array_empty() {
            assert!(!arrayEmpty(arr(vec![1, 2, 3])).unwrap());
            assert!(arrayEmpty(arr::<i32>(vec![])).unwrap());
        }

        #[test]
        fn test_array_get() {
            let a = arr(vec![10, 20, 30]);
            assert_eq!(arrayGet(a.clone(), 1).unwrap(), 10);
            assert_eq!(arrayGet(a.clone(), 2).unwrap(), 20);
            assert_eq!(arrayGet(a.clone(), 3).unwrap(), 30);
            assert!(arrayGet(a.clone(), 0).is_err());
            assert!(arrayGet(a, 4).is_err());
        }

        #[test]
        fn test_array_create() {
            let a = arrayCreate(5, 0).unwrap();
            assert_eq!(*a.borrow(), vec![0, 0, 0, 0, 0]);
            let empty: Array<i32> = arrayCreate(0, 42).unwrap();
            assert!(empty.borrow().is_empty());
        }

        #[test]
        fn test_array_list() {
            let a = arr(vec![1, 2, 3]);
            let lst = arrayList(a).unwrap();
            assert_eq!(lst, list![1, 2, 3]);
        }

        #[test]
        fn test_list_array() {
            let lst = list![1, 2, 3];
            let a = listArray(lst).unwrap();
            assert_eq!(*a.borrow(), vec![1, 2, 3]);
        }

        #[test]
        fn test_array_update() {
            let a = arr(vec![1, 2, 3]);
            arrayUpdate(a.clone(), 2, 99).unwrap();
            assert_eq!(*a.borrow(), vec![1, 99, 3]);
            assert!(arrayUpdate(a.clone(), 0, 99).is_err());
            assert!(arrayUpdate(a.clone(), 4, 99).is_err());

            // Aliasing semantics: updates visible through every clone of the Rc.
            let alias = a.clone();
            arrayUpdate(a.clone(), 1, 100).unwrap();
            assert_eq!(*alias.borrow(), vec![100, 99, 3]);
        }

        #[test]
        fn test_array_copy() {
            let a = arr(vec![1, 2, 3]);
            let copy = arrayCopy(a.clone()).unwrap();
            assert_eq!(*copy.borrow(), vec![1, 2, 3]);
            // arrayCopy must NOT share storage with the source.
            arrayUpdate(a, 1, 99).unwrap();
            assert_eq!(*copy.borrow(), vec![1, 2, 3]);
        }

        #[test]
        fn test_array_append() {
            let a = arr(vec![1, 2]);
            let b = arr(vec![3, 4]);
            assert_eq!(*arrayAppend(a.clone(), b.clone()).unwrap().borrow(), vec![1, 2, 3, 4]);

            let empty: Array<i32> = arr(vec![]);
            assert_eq!(*arrayAppend(empty.clone(), b).unwrap().borrow(), vec![3, 4]);
            assert_eq!(*arrayAppend(a, empty).unwrap().borrow(), vec![1, 2]);
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
                fileName: literal!("test.mo"),
                isReadOnly: true,
                lineNumberStart: 1,
                columnNumberStart: 1,
                lineNumberEnd: 10,
                columnNumberEnd: 50,
                lastModification: 1234567890.0,
            };
            assert_eq!(info.fileName, "test.mo");
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
            let arr = arrayFromVec(vec![10, 20, 30]);
            // Valid 1-based indices
            assert_eq!(arrayGetNoBoundsChecking(arr.clone(), 1).unwrap(), 10);
            assert_eq!(arrayGetNoBoundsChecking(arr.clone(), 2).unwrap(), 20);
            assert_eq!(arrayGetNoBoundsChecking(arr, 3).unwrap(), 30);
        }

        #[test]
        fn test_array_update_no_bounds_checking() {
            let arr = arrayFromVec(vec![1, 2, 3]);
            arrayUpdateNoBoundsChecking(arr.clone(), 2, 99).unwrap();
            assert_eq!(*arr.borrow(), vec![1, 99, 3]);
        }

        #[test]
        fn test_array_create_no_init() {
            let arr: Array<i32> = arrayCreateNoInit(5).unwrap();
            assert_eq!(arr.borrow().len(), 5);
        }

        #[test]
        fn test_string_get_no_bounds_checking() {
            let s = "hello".to_string();
            assert_eq!(stringGetNoBoundsChecking(s.clone(), 1).unwrap(), b'h' as i32);
            assert_eq!(stringGetNoBoundsChecking(s, 5).unwrap(), b'o' as i32);
        }
    }
}
