#![allow(non_snake_case, dead_code, unused_macros)]
//! Translation of FrontEnd/MetaModelicaBuiltin.mo
//!
//! Built-in MetaModelica declarations translated to Rust.
//! All functions are translated even if Rust has built-in alternatives,
//! since these functions may be referenced by other translated modules.
//!
//! Datatype mapping:
//!   Integer -> i32
//!   Real -> OrderedFloat<f64> (aliased as `metamodelica::Real`)
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
pub use ordered_float::OrderedFloat;
pub use num_traits::Float;

pub mod gc;

/// MetaModelica `array<T>`. See module-level docs for rationale.
pub type Array<A> = Rc<RefCell<Vec<A>>>;

/// MetaModelica `Real`. Wraps `f64` with `OrderedFloat` so that values
/// containing `Real` can implement `Ord` / `Eq` / `Hash` — required for
/// derived `valueCompare` on enums such as `DAE::Exp` and `DAE::Type`.
/// NaN ordering follows `ordered_float` semantics (`NaN` > any non-NaN).
pub type Real = OrderedFloat<f64>;

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
        // Rust versions: we accept up to 10 type arguments — enough for all
        // call sites we generate today (e.g. `NBJacobian.jacobianNone`, which
        // takes 9) — and the user gets a clear macro error otherwise.
        $crate::__fnptr_dispatch!($f $(, $t)+)
    }};
}

/// Internal helper for [`fnptr!`]: dispatches on arity (1..=10) without
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
    ($f:path, $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty) =>
        { |a1: $t1, a2: $t2, a3: $t3, a4: $t4, a5: $t5, a6: $t6, a7: $t7, a8: $t8, a9: $t9| -> ::anyhow::Result<_> { ::std::result::Result::Ok($f(a1, a2, a3, a4, a5, a6, a7, a8, a9)) } };
    ($f:path, $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty) =>
        { |a1: $t1, a2: $t2, a3: $t3, a4: $t4, a5: $t5, a6: $t6, a7: $t7, a8: $t8, a9: $t9, a10: $t10| -> ::anyhow::Result<_> { ::std::result::Result::Ok($f(a1, a2, a3, a4, a5, a6, a7, a8, a9, a10)) } };
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
            lastModification: $crate::OrderedFloat(0.0_f64),
        }
    };
    // With an explicit file: the original MetaModelica source path (the code
    // generator knows the `.mo` a function came from, but not the statement's
    // line — positions are zero, "unknown line in this file", which is also
    // exactly what the reference compiler prints under Testsuite.isRunning).
    ($file:literal) => {
        $crate::SourceInfo {
            fileName: ::arcstr::literal!($file),
            isReadOnly: false,
            lineNumberStart: 0,
            columnNumberStart: 0,
            lineNumberEnd: 0,
            columnNumberEnd: 0,
            lastModification: $crate::OrderedFloat(0.0_f64),
        }
    };
}

/// The Info attribute provides location information for elements and classes.
/// Mapped from the SOURCEINFO record in MetaModelicaBuiltin.mo.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    pub lastModification: Real,
}

// ============================================================================
// Boolean functions
// ============================================================================

/// Logically combine two Booleans with 'and' operator.
#[inline(always)]
pub fn boolAnd(b1: bool, b2: bool) -> bool {
    b1 && b2
}

/// Logically combine two Booleans with 'or' operator.
#[inline(always)]
pub fn boolOr(b1: bool, b2: bool) -> bool {
    b1 || b2
}

/// Logically invert Boolean value using 'not' operator.
#[inline(always)]
pub fn boolNot(b: bool) -> bool {
    !b
}

/// Compares two Booleans for equality.
#[inline(always)]
pub fn boolEq(b1: bool, b2: bool) -> bool {
    b1 == b2
}

/// Returns "true" or "false" string from a boolean.
pub fn boolString(b: bool) -> ArcStr {
    if b { literal!("true") } else { literal!("false") }
}

/// MetaModelica `print` builtin: writes the argument to stdout *without*
/// adding a trailing newline (matches the C runtime's `print`). This exists
/// alongside the inline `println!` lowering used at direct call sites so that
/// passing `print` as a value (e.g. `List.map_0(strs, print)`) resolves to a
/// real function item that can be wrapped by `fnptr!`. The codegen prefers the
/// inline lowering for direct calls because it avoids an extra trait-object
/// hop and keeps the formatting macro behaviour identical to the prior
/// generated code.
pub fn print(s: ArcStr) {
    // On wasm there is no OS stdout; route through the host hook (the JS host
    // binds it to console.log — see libopenmodelica_compiler). With no hook
    // bound the output is dropped, matching the prior wasm behaviour.
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(hook) = STDOUT_HOOK.get() {
            hook(s.as_str());
        }
        return;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::io::Write;
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(s.as_bytes()).ok();
    }
}

// Host output hooks. The native build writes straight to the process
// stdout/stderr; on wasm there is no OS console, so the JS host binds these
// (console.log / console.error) at startup. Mirrors the `ASSERT_HOOK` pattern.
static STDOUT_HOOK: std::sync::OnceLock<fn(&str)> = std::sync::OnceLock::new();
static STDERR_HOOK: std::sync::OnceLock<fn(&str)> = std::sync::OnceLock::new();

/// Bind the stdout sink used by [`print`] (first binding wins).
pub fn setStdoutHook(hook: fn(&str)) {
    let _ = STDOUT_HOOK.set(hook);
}

/// Bind the stderr sink used by [`host_eprint`] (first binding wins).
pub fn setStderrHook(hook: fn(&str)) {
    let _ = STDERR_HOOK.set(hook);
}

/// Write `s` to the host's stderr: the process stderr natively, or the bound
/// stderr hook on wasm. Use instead of a bare `eprintln!` wherever the output
/// must reach the JS console on wasm (e.g. the `showErrorMessages` echo).
pub fn host_eprint(s: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(hook) = STDERR_HOOK.get() {
            hook(s);
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        eprint!("{s}");
    }
}

// ============================================================================
// Runtime assertions (`omc_assert`)
// ============================================================================

/// Host hook invoked by [`omc_assert!`] before the assertion fails. This is
/// the analogue of the C runtime's `omc_assert` function pointer
/// (`SimulationRuntime/c/util/omc_error.c`): the compiler binds it (via
/// `ErrorExt.initAssertionFunctions`) to `omc_assert_compiler`, which appends a
/// `RUNTIME`/`Error` message to the error buffer, while the simulation runtime
/// would bind its own logger. When unbound the message survives only through
/// the returned error, matching a runtime that never installed a reporter.
static ASSERT_HOOK: std::sync::OnceLock<fn(&str)> = std::sync::OnceLock::new();

/// Bind the assertion reporter, mirroring the assignment of the `omc_assert`
/// function pointer in `Error_initAssertionFunctions`. The first binding wins,
/// matching the single init call performed at startup; later calls are ignored.
pub fn setAssertHook(hook: fn(&str)) {
    let _ = ASSERT_HOOK.set(hook);
}

/// Report an assertion message through the bound hook, if any. Called by the
/// [`omc_assert!`] macro immediately before it fails the surrounding
/// computation. Reading the `OnceLock` is lock-free after startup, so this is
/// safe to reach from any thread and adds no contention on the (cold) assert
/// path.
pub fn reportAssert(msg: &str) {
    if let Some(hook) = ASSERT_HOOK.get() {
        hook(msg);
    }
}

/// Runtime assertion mirroring the C `omc_assert(threadData, info, fmt, ...)`.
/// It reports the formatted message to the host error subsystem (see
/// [`setAssertHook`]) and then fails the current computation with that message
/// — the analogue of `omc_assert`'s `MMC_THROW`, which a surrounding
/// matchcontinue catches. Use it wherever a ported runtime function would call
/// `omc_assert`, instead of a bare `bail!`, so the message reaches
/// `getErrorString`.
#[macro_export]
macro_rules! omc_assert {
    ($($arg:tt)*) => {{
        let __msg: ::std::string::String = ::std::format!($($arg)*);
        $crate::reportAssert(&__msg);
        ::anyhow::bail!(__msg)
    }};
}

// ============================================================================
// Integer arithmetic functions
// ============================================================================

/// Adds two Integer values.
#[inline(always)]
pub fn intAdd(i1: i32, i2: i32) -> i32 {
    i1 + i2
}

/// Subtracts two Integer values.
#[inline(always)]
pub fn intSub(i1: i32, i2: i32) -> i32 {
    i1 - i2
}

/// Multiplies two Integer values.
#[inline(always)]
pub fn intMul(i1: i32, i2: i32) -> i32 {
    i1 * i2
}

/// Divides two Integer values (truncated division).
/// Matches Modelica's div() semantics: truncates toward zero.
pub fn intDiv(i1: i32, i2: i32) -> i32 {
    i1 / i2
}

/// Calculates `mod(i1, i2)` with Modelica semantics: the result has the
/// same sign as the divisor (Euclidean-style), not the dividend.
///
/// This mirrors `modelica_mod_integer` in the C runtime:
/// `let tmp = i1 % i2; if (i2>0 && tmp<0) || (i2<0 && tmp>0) { tmp + i2 } else { tmp }`.
/// Rust's `%` (like C's) returns the sign of the dividend, so a plain
/// `i1 % i2` is wrong for negative dividends — callers like the hash-set
/// bucket index `intMod(hash, bsize)` then dereference negative indices.
pub fn intMod(i1: i32, i2: i32) -> i32 {
    let tmp = i1 % i2;
    if (i2 > 0 && tmp < 0) || (i2 < 0 && tmp > 0) {
        tmp + i2
    } else {
        tmp
    }
}

/// Returns the bigger one of two Integer values.
pub fn intMax(i1: i32, i2: i32) -> i32 {
    i1.max(i2)
}

/// Returns the smaller one of two Integer values.
pub fn intMin(i1: i32, i2: i32) -> i32 {
    i1.min(i2)
}

/// Returns the absolute value of Integer i.
pub fn intAbs(i: i32) -> i32 {
    i.abs()
}

/// Returns negative value of Integer i.
#[inline(always)]
pub fn intNeg(i: i32) -> i32 {
    -i
}

// ============================================================================
// Integer comparison functions
// ============================================================================

/// Returns whether Integer i1 is smaller than Integer i2.
#[inline(always)]
pub fn intLt(i1: i32, i2: i32) -> bool {
    i1 < i2
}

/// Returns whether Integer i1 is smaller than or equal to Integer i2.
#[inline(always)]
pub fn intLe(i1: i32, i2: i32) -> bool {
    i1 <= i2
}

/// Returns whether Integer i1 is equal to Integer i2.
#[inline(always)]
pub fn intEq(i1: i32, i2: i32) -> bool {
    i1 == i2
}

/// Returns whether Integer i1 is not equal to Integer i2.
#[inline(always)]
pub fn intNe(i1: i32, i2: i32) -> bool {
    i1 != i2
}

/// Returns whether Integer i1 is greater than or equal to Integer i2.
#[inline(always)]
pub fn intGe(i1: i32, i2: i32) -> bool {
    i1 >= i2
}

/// Returns whether Integer i1 is greater than Integer i2.
#[inline(always)]
pub fn intGt(i1: i32, i2: i32) -> bool {
    i1 > i2
}

// ============================================================================
// Integer bitwise functions
// ============================================================================

/// Returns bitwise inverted Integer number of i (~i in C).
#[inline(always)]
pub const fn intBitNot(i: i32) -> i32 {
    !i
}

/// Returns bitwise 'and' of Integers i1 and i2 (i1 & i2 in C).
#[inline(always)]
pub const fn intBitAnd(i1: i32, i2: i32) -> i32 {
    i1 & i2
}

/// Returns bitwise 'or' of Integers i1 and i2 (i1 | i2 in C).
#[inline(always)]
pub const fn intBitOr(i1: i32, i2: i32) -> i32 {
    i1 | i2
}

/// Returns bitwise 'xor' of Integers i1 and i2 (i1 ^ i2 in C).
#[inline(always)]
pub const fn intBitXor(i1: i32, i2: i32) -> i32 {
    i1 ^ i2
}

/// Returns bitwise left shift of Integer i by s bits (i << s in C).
#[inline(always)]
pub const fn intBitLShift(i: i32, s: i32) -> i32 {
    i << s
}

/// Returns bitwise right shift of Integer i by s bits (i >> s in C).
#[inline(always)]
pub const fn intBitRShift(i: i32, s: i32) -> i32 {
    i >> s
}

// ============================================================================
// Integer conversion functions
// ============================================================================

/// Converts Integer to Real.
#[inline(always)]
pub fn intReal(i: i32) -> Real {
    OrderedFloat(i as f64)
}

/// Converts Integer to String.
pub fn intString(i: i32) -> ArcStr {
    format!("{}", i)
}

// ============================================================================
// Real arithmetic functions
// ============================================================================

/// Adds two Real values.
#[inline(always)]
pub fn realAdd(r1: Real, r2: Real) -> Real {
    r1 + r2
}

/// Subtracts two Real values.
#[inline(always)]
pub fn realSub(r1: Real, r2: Real) -> Real {
    r1 - r2
}

/// Multiplies two Real values.
#[inline(always)]
pub fn realMul(r1: Real, r2: Real) -> Real {
    r1 * r2
}

/// Divides two Real values.
#[inline(always)]
pub fn realDiv(r1: Real, r2: Real) -> Real {
    r1 / r2
}

/// Real division `/` with MetaModelica failure semantics: dividing by exactly
/// zero is a *recoverable* MetaModelica failure, not `inf`/`nan`. The C runtime
/// generated for every `Real / Real` emits `if (denom == 0) goto fail;` before
/// the division, so a zero divisor fails the enclosing `match`/`matchcontinue`
/// arm rather than producing a non-finite constant. mmtorust lowers the `/`
/// operator (`BinOpKind::Div`) through this helper so the same observable
/// behaviour holds — e.g. `ExpressionSimplify.simplifyBinaryConst` must *fail*
/// on `1.0 / 0.0` (leaving the expression unsimplified) instead of folding it
/// to `RCONST(inf)`.
#[inline(always)]
pub fn real_div_checked(r1: Real, r2: Real) -> Result<Real> {
    if r2.0 == 0.0 {
        bail!("Real division by zero");
    }
    Ok(r1 / r2)
}

/// Calculates remainder of Real division r1/r2.
pub fn realMod(r1: Real, r2: Real) -> Real {
    OrderedFloat(r1.0 - (r1.0/r2.0).floor()*r2.0)
}

/// Raises r1 to the power r2 (r1^r2).
pub fn realPow(r1: Real, r2: Real) -> Real {
    OrderedFloat(r1.0.powf(r2.0))
}

/// Returns the bigger one of two Real values.
#[inline(always)]
pub fn realMax(r1: Real, r2: Real) -> Real {
    OrderedFloat(r1.0.max(r2.0))
}

/// Returns the smaller one of two Real values.
#[inline(always)]
pub fn realMin(r1: Real, r2: Real) -> Real {
    OrderedFloat(r1.0.min(r2.0))
}

/// Returns the absolute value of Real x.
#[inline(always)]
pub fn realAbs(x: Real) -> Real {
    OrderedFloat(x.0.abs())
}

/// Returns whether two Real values are approximately equal within absTol.
pub fn realAlmostEq(a: Real, b: Real, abs_tol: Real) -> bool {
    abs_tol.0 > (a.0 - b.0).abs()
}

/// Returns negative value of Real x.
#[inline(always)]
pub fn realNeg(x: Real) -> Real {
    -x
}

/// Modelica `sign(v)` builtin: the signum of a Real, returning an Integer
/// `-1`, `0`, or `1` (see ModelicaBuiltin.mo `function sign`). Note this is
/// NOT `f64::signum`, which returns `±1.0` for zero/negative-zero; Modelica
/// requires `sign(0) == 0`. Integer arguments are coerced to Real at the call
/// site (the builtin's formal is `Real v`), so a single Real overload suffices.
#[inline(always)]
pub fn sign(v: Real) -> i32 {
    if v.0 > 0.0 { 1 } else if v.0 < 0.0 { -1 } else { 0 }
}

// ============================================================================
// Real comparison functions
// ============================================================================

/// Returns whether Real x1 is smaller than Real x2.
#[inline(always)]
pub fn realLt(x1: Real, x2: Real) -> bool {
    x1 < x2
}

/// Returns whether Real x1 is smaller than or equal to Real x2.
#[inline(always)]
pub fn realLe(x1: Real, x2: Real) -> bool {
    x1 <= x2
}

/// Returns whether Real x1 is equal to Real x2.
#[inline(always)]
pub fn realEq(x1: Real, x2: Real) -> bool {
    x1 == x2
}

/// Returns whether Real x1 is not equal to Real x2.
#[inline(always)]
pub fn realNe(x1: Real, x2: Real) -> bool {
    x1 != x2
}

/// Returns whether Real x1 is greater than or equal to Real x2.
#[inline(always)]
pub fn realGe(x1: Real, x2: Real) -> bool {
    x1 >= x2
}

/// Returns whether Real x1 is greater than Real x2.
#[inline(always)]
pub fn realGt(x1: Real, x2: Real) -> bool {
    x1 > x2
}

// ============================================================================
// Real conversion functions
// ============================================================================

/// Converts Real to Integer (truncates toward zero, matching Modelica integer() function).
pub fn realInt(r: Real) -> i32 {
    r.0 as i32
}

/// Converts Real to String.
///
/// Mirrors the C runtime's `realString` (`SimulationRuntime/c/meta/realString.c`):
/// the shortest round-trip representation (ryu's `d2s`; Rust's `{:e}` is the
/// same shortest form) post-processed by `ryu_to_hr` with `real_output=true`,
/// so round values keep a trailing `.0` (`3.0` → `"3.0"`, never `"3"`).
pub fn realString(r: Real) -> ArcStr {
    let v = r.0;
    if v.is_infinite() {
        return if v < 0.0 { arcstr::literal!("-inf") } else { arcstr::literal!("inf") };
    }
    if v.is_nan() {
        return arcstr::literal!("NaN");
    }
    ArcStr::from(ryu_to_hr(&std::format!("{:e}", v), true))
}

/// Port of the C runtime's `ryu_hr_tdzp` (`3rdParty/ryu/ryu/om_format.c`): the
/// shortest round-trip representation in OMEdit's display variant (`ryu_to_hr`
/// with `real_output=false`). Used by OMEdit's `StringHandler::number` to format
/// numbers for display, exposed to it through the embedding cdylib.
pub fn ryu_hr_tdzp(d: f64) -> String {
    if d.is_infinite() {
        return if d < 0.0 { "-inf".to_owned() } else { "inf".to_owned() };
    }
    if d.is_nan() {
        return "NaN".to_owned();
    }
    ryu_to_hr(&std::format!("{:e}", d), false)
}

/// Port of `ryu_to_hr` from `3rdParty/ryu/ryu/om_format.c`: convert a
/// shortest-form scientific representation (`8.13e2`) to the minimal
/// decimal or exponential rendering omc uses everywhere it prints Reals.
///
/// * Exponents in `[-3, 5]` (and at most 3 trailing zeros before the
///   decimal point) print in decimal form, everything else stays
///   exponential (with a lowercase `e`).
/// * With `real_output`, round decimal values get a trailing `.0`.
/// * Without `real_output`, a mantissa with more than 12 decimals is
///   rounded to 12 when that removes at least 4 trailing zeros (the
///   OMEdit-style display variant, `ryu_hr_tdzp`).
fn ryu_to_hr(d2s_str: &str, real_output: bool) -> String {
    let Some(epos) = d2s_str.find(['e', 'E']) else {
        // Not in mantissa-exponent form (e.g. "NaN"); pass through.
        return d2s_str.replace('E', "e");
    };
    let mant_str = &d2s_str[..epos];
    let mut exp: i32 = d2s_str[epos + 1..].parse().unwrap_or(0);
    let (neg, mut digits) = match mant_str.strip_prefix('-') {
        Some(m) => (true, m.to_string()),
        None => (false, mant_str.to_string()),
    };
    // Number of digits after the decimal point in the mantissa.
    let mut ndec: i32 = if digits.contains('.') { digits.len() as i32 - 2 } else { 0 };
    // The exponential rendering used when the decimal form is unsuitable.
    let mut exp_repr: String = d2s_str.replace('E', "e");

    if ndec > 12 && !real_output {
        // Round the mantissa to 12 decimals; use it only if that removed at
        // least 4 trailing zeros (i.e. the long tail was an artifact).
        let mant: f64 = digits.parse().unwrap_or(0.0);
        let mut rounded = std::format!("{mant:.12}");
        // 9.999999999999999 rounds to 10.000000000000: renormalise.
        if rounded == "10.000000000000" {
            rounded = "1.000000000000".to_string();
            exp += 1;
        }
        let mut nz = 0;
        while rounded.ends_with('0') {
            rounded.pop();
            nz += 1;
        }
        if rounded.ends_with('.') {
            rounded.pop();
        }
        if nz > 3 {
            digits = rounded;
            ndec = if digits.contains('.') { digits.len() as i32 - 2 } else { 0 };
            exp_repr = std::format!("{}{digits}e{exp}", if neg { "-" } else { "" });
        }
    }

    if !(-3..=5).contains(&exp) || (exp > 0 && exp - ndec > 3) {
        return exp_repr;
    }

    // Decimal form. `digs` is the mantissa without its decimal point:
    // one leading digit followed by `ndec` decimals.
    let digs: Vec<char> = digits.chars().filter(|c| *c != '.').collect();
    let mut out = String::with_capacity(24);
    if neg {
        out.push('-');
    }
    if exp == 0 {
        out.push_str(&digits);
    } else if exp > 0 {
        // Move the decimal point `exp` places to the right.
        out.push(digs[0]);
        let take = ndec.min(exp) as usize;
        out.extend(&digs[1..1 + take]);
        if exp > ndec {
            for _ in 0..(exp - ndec) {
                out.push('0');
            }
        } else if exp < ndec {
            out.push('.');
            out.extend(&digs[1 + take..]);
        }
    } else {
        // exp < 0: the number starts with "0." and some zeros.
        out.push_str("0.");
        for _ in 0..(-exp - 1) {
            out.push('0');
        }
        out.extend(&digs);
    }
    if exp >= ndec && real_output {
        out.push_str(".0");
    }
    out
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
pub fn intStringChar(i: i32) -> ArcStr {
    format!("{}", std::char::from_u32(i as u32).unwrap())
}

/// Parses an integer from a string. Fails if the string is not a valid integer.
///
/// Mirrors `nobox_stringInt` (meta_modelica_builtin.c): `strtol` skips
/// leading whitespace, and the conversion fails unless everything after the
/// number has been consumed (so trailing whitespace is an error).
pub fn stringInt(str: ArcStr) -> Result<i32> {
    str.trim_start_matches(c_isspace)
        .parse::<i32>()
        .map_err(|_| anyhow::anyhow!("Failed to parse integer from string: {}", str))
}

/// The C-locale `isspace` set that `strtol`/`strtod` skip before a number.
fn c_isspace(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n' | '\x0b' | '\x0c' | '\r')
}

/// Parses a real from a string.
/// Fails unless the whole string can be consumed.
///
/// Mirrors `nobox_stringReal` (meta_modelica_builtin.c): `strtod` skips
/// leading whitespace — e.g. `" 2.10"` from a CSV column parses — and the
/// conversion fails unless everything after the number has been consumed.
/// (C99 hex-float syntax, which strtod would also accept, is not supported
/// by Rust's parser; no caller is known to rely on it.)
pub fn stringReal(str: ArcStr) -> Result<Real> {
    str.trim_start_matches(c_isspace)
        .parse::<f64>()
        .map(OrderedFloat)
        .map_err(|_| anyhow::anyhow!("Failed to parse real from string: {}", str))
}

/// Converts a string to a list of single-character strings.
pub fn stringListStringChar(str: ArcStr) -> Arc<List<ArcStr>> {
    // TODO: We could have constants for all these short strings to avoid allocations.
    Arc::new(str.chars().map(|c| format!("{}", c)).collect())
}

/// Appends a list of strings into a single string.
pub fn stringAppendList(strs: Arc<List<ArcStr>>) -> ArcStr {
    let mut len = 0;
    for s in &*strs {
        len += s.len();
    }
    let mut result = String::with_capacity(len);
    for s in &*strs {
        result.push_str(s);
    }
    result.into()
}

/// Takes a list of strings and a delimiter and joins them with the delimiter inserted between elements.
/// Example: stringDelimitList({"x","y","z"}, ", ") => "x, y, z"
pub fn stringDelimitList(strs: Arc<List<ArcStr>>, delimiter: ArcStr) -> ArcStr {
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
        result.push_str(s);
        first = false;
    }

    result.into()
}

/// Returns the length of the string (number of bytes).
pub fn stringLength(str: ArcStr) -> i32 {
    str.len() as i32
}

/// Returns true if the string is empty.
pub fn stringEmpty(str: ArcStr) -> bool {
    str.is_empty()
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
pub fn stringAppend(s1: ArcStr, s2: ArcStr) -> ArcStr {
    format!("{}{}", s1, s2)
}

/// Compares two strings for equality.
#[inline(always)]
pub fn stringEq(s1: ArcStr, s2: ArcStr) -> bool {
    s1 == s2
}
#[inline(always)]
pub fn stringEqual(s1: ArcStr, s2: ArcStr) -> bool {
    s1 == s2
}

/// Compares two strings lexicographically.
/// Returns negative if s1 < s2, zero if s1 == s2, positive if s1 > s2.
pub fn stringCompare(s1: ArcStr, s2: ArcStr) -> i32 {
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
pub fn stringHash(str: ArcStr) -> i32 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hash;
    use std::hash::Hasher;
    let mut hasher = DefaultHasher::new();
    str.hash(&mut hasher);
    hasher.finish() as i32
}

/// Reduces a 64-bit djb2 accumulator to the non-negative `Integer` (i32) the
/// MetaModelica builtins return. The C runtime (`meta_modelica_builtin.c`)
/// computes djb2 in a 64-bit word and returns `labs()` of it; mirror that by
/// taking the magnitude and keeping the low 31 bits, so the result is always
/// `>= 0`. A non-negative hash matters wherever it reaches a textual context —
/// e.g. `Util.hashFileNamePrefix` embeds `intString(hash)` in generated file
/// and directory names, and a leading '-' from a negative value breaks the
/// shell commands (`rm -rf -56.fmutmp`) the generated makefiles run on them.
#[inline]
fn djb2_to_integer(hash: i64) -> i32 {
    (hash.unsigned_abs() & i32::MAX as u64) as i32
}

/// Returns a DJB2 hash of the string.
/// DJB2 algorithm: hash = hash * 33 + byte
///
/// Accumulated in i64 to match the C runtime's 64-bit `unsigned long` recurrence
/// (the i32 port previously wrapped at 32 bits, diverging from C and collapsing
/// long strings), then returned as a non-negative i32 — see [`djb2_to_integer`].
pub fn stringHashDjb2(str: ArcStr) -> i32 {
    let mut hash: i64 = 5381;
    for &byte in str.as_bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as i64);
    }
    djb2_to_integer(hash)
}

/// Continues computing a DJB2 hash by adding another string to it. The incoming
/// `hash` is the (already non-negative) result of a previous step; accumulation
/// proceeds in i64 and the result is reduced back to a non-negative i32.
pub fn stringHashDjb2Continue(str: ArcStr, hash: i32) -> i32 {
    let mut h = hash as i64;
    for &byte in str.as_bytes() {
        h = h.wrapping_mul(33).wrapping_add(byte as i64);
    }
    djb2_to_integer(h)
}

/// Computes a DJB2 hash and applies modulo without intermediate overflow issues.
/// Mirrors the C runtime: the 64-bit hash is reduced modulo `mod_val` using
/// unsigned arithmetic, so the result lies in `[0, mod_val)`.
pub fn stringHashDjb2Mod(str: ArcStr, mod_val: i32) -> i32 {
    if mod_val == 0 {
        return 0;
    }
    let mut hash: i64 = 5381;
    for &byte in str.as_bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as i64);
    }
    ((hash as u64) % (mod_val as u32 as u64)) as i32
}

/// Returns an SDBM hash of the string.
/// SDBM algorithm: hash = byte + (hash << 6) + (hash << 16) - hash
pub fn stringHashSdbm(str: ArcStr) -> i32 {
    let mut hash: i32 = 0;
    for &byte in str.as_bytes() {
        hash = byte as i32 + (hash << 6) + (hash << 16) - hash;
    }
    hash
}

thread_local! {
    /// Class-name → source-directory mapping for `modelica://` URI
    /// resolution. The Rust analogue of the C runtime's
    /// `threadData->localRoots[LOCAL_ROOT_URI_LOOKUP]`: an interleaved
    /// `[name1, dir1, name2, dir2, ...]` array sorted by name, installed by
    /// `SymbolTable.updateUriMapping` (via `System.updateUriMapping`)
    /// whenever the loaded program changes.
    static URI_LOOKUP: RefCell<Array<ArcStr>> = RefCell::new(Default::default());
}

/// Install the `modelica://` URI lookup table. Port of
/// `OpenModelica_updateUriMapping` (`SimulationRuntime/c/util/utility.c`),
/// which stashes the array in a thread-local root for [`uriToFilename`] to
/// read later. No parsing happens here.
pub fn updateUriMapping(namesAndDirs: Array<ArcStr>) {
    URI_LOOKUP.with(|r| *r.borrow_mut() = namesAndDirs);
}

/// Port of `lookupDirectoryFromName`: binary-search the interleaved
/// `[name, dir, ...]` table for `name` and return its directory.
fn uri_lookup_directory(name: &str) -> Option<String> {
    URI_LOOKUP.with(|r| {
        let table = r.borrow();
        let table = table.borrow();
        // The table is sorted by name (AvlTreeStringString in-order), so a
        // binary search matches the C bsearch; a linear scan would also be
        // correct but the table can hold every loaded top-level class.
        let mut lo = 0usize;
        let mut hi = table.len() / 2;
        while lo < hi {
            let mid = (lo + hi) / 2;
            match name.cmp(table[2 * mid].as_str()) {
                std::cmp::Ordering::Equal => return Some(table[2 * mid + 1].to_string()),
                std::cmp::Ordering::Less => hi = mid,
                std::cmp::Ordering::Greater => lo = mid + 1,
            }
        }
        None
    })
}

/// Port of `OpenModelica_decode_uri_inplace`: `+` becomes a space and
/// `%XX` hex escapes are decoded; a `%` followed by invalid hex is copied
/// through literally.
fn decode_uri(src: &str) -> String {
    let bytes = src.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'+' => out.push(b' '),
            b'%' if i + 2 < bytes.len()
                && bytes[i + 1].is_ascii_hexdigit()
                && bytes[i + 2].is_ascii_hexdigit() =>
            {
                let hi = (bytes[i + 1] as char).to_digit(16).unwrap() as u8;
                let lo = (bytes[i + 2] as char).to_digit(16).unwrap() as u8;
                out.push(hi * 16 + lo);
                i += 2;
            }
            b => out.push(b),
        }
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// Converts a Modelica URI to an absolute filename.
///
/// Mirrors `OpenModelica_uriToFilename_impl` in
/// `OMCompiler/SimulationRuntime/c/util/utility.c` (with a NULL
/// `resourcesDir`, like the `OpenModelica_uriToFilename` builtin). The
/// MM-source-level `OpenModelica.Scripting.uriToFilename(uri)` lowers to
/// this when called from generated code (see the rewrite in
/// `mmtorust::typedexp::cref_to_dotted`).
///
/// Schemes handled:
/// * `modelica://Package.Sub/path` — look up `Package`'s source directory
///   in the table installed by [`updateUriMapping`], descend into nested
///   class subdirectories as long as they exist on disk, and append the
///   path part. Fails (like the C `MMC_THROW`) when the package is not in
///   the table.
/// * `file://path` — strip the prefix and treat as a regular path.
/// * Other `xxx://` URIs — fail, matching the C runtime's `MMC_THROW`.
/// * Plain paths — canonicalize through `std::fs::canonicalize` if the path
///   exists; otherwise return as-is when absolute, or prepend the current
///   working directory when relative.
pub fn uriToFilename(uri_om: ArcStr) -> Result<ArcStr> {
    let uri = &*uri_om;
    if uri.is_empty() {
        omc_assert!("Malformed URI (got an empty string)");
    }
    // Scheme matching is case-insensitive per the C implementation
    // (`strncasecmp`). Only the prefix is lowercased — paths on
    // case-sensitive filesystems must keep their original casing.
    let scheme_match = |prefix: &str| -> bool {
        uri.len() >= prefix.len()
            && uri[..prefix.len()].eq_ignore_ascii_case(prefix)
    };

    // Resolve the path the same way `uriToFilenameRegularPaths` does for
    // non-FMU calls (resourcesDir == NULL). `orig_uri` only feeds the
    // trailing-slash rule: a directory result keeps a trailing '/' when the
    // *original URI* ended with one.
    fn regular_paths(path: &str, orig_uri: &ArcStr) -> ArcStr {
        let p = std::path::Path::new(path);
        match std::fs::canonicalize(p) {
            Ok(canon) => {
                let mut s = canon.to_string_lossy().into_owned();
                if orig_uri.ends_with('/') && !s.ends_with('/') && p.is_dir() {
                    s.push('/');
                }
                ArcStr::from(s)
            }
            Err(_) => {
                // Path does not exist (yet). For absolute paths, return
                // as-is; for relative paths, prepend the current working
                // directory.
                let is_absolute = p.is_absolute()
                    || (path.len() >= 2 && path.as_bytes()[1] == b':'
                        && path.as_bytes()[0].is_ascii_alphabetic());
                if is_absolute {
                    ArcStr::from(path)
                } else if let Ok(cwd) = std::env::current_dir() {
                    let mut joined = cwd;
                    joined.push(path);
                    ArcStr::from(joined.to_string_lossy().into_owned())
                } else {
                    ArcStr::from(path)
                }
            }
        }
    }

    if scheme_match("modelica://") {
        let rest = &uri[11..];
        // getIdent: the class name runs up to the first '.' or '/'.
        let ident_end = rest.find(['.', '/']).unwrap_or(rest.len());
        let class_name = &rest[..ident_end];
        if class_name.is_empty() {
            omc_assert!("Malformed URI (couldn't get a class name): {uri}");
        }
        let mut dir = match uri_lookup_directory(class_name).filter(|d| !d.is_empty()) {
            Some(dir) => dir,
            None => omc_assert!("Failed to lookup URI (is the package loaded?) {uri}"),
        };
        let rest = decode_uri(&rest[ident_end..]);
        // Descend into nested class subdirectories: for each `.Ident`,
        // extend the directory while `<dir>/<Ident>` exists on disk. An
        // ident whose directory does not exist is consumed and *dropped* —
        // the resource path is relative to the innermost class that has
        // its own directory (same as the C loop).
        let mut pos = 0usize;
        while rest[pos..].starts_with('.') {
            pos += 1;
            let id_end = rest[pos..]
                .find(['.', '/'])
                .map(|i| pos + i)
                .unwrap_or(rest.len());
            if id_end == pos {
                if rest[id_end..].starts_with('.') {
                    omc_assert!("Malformed URI (double dot in class name): {uri}");
                }
                break; // '/' or end of string
            }
            let candidate = std::format!("{dir}/{}", &rest[pos..id_end]);
            pos = id_end;
            if !std::path::Path::new(&candidate).is_dir() {
                break;
            }
            dir = candidate;
        }
        // Skip to just past the next '/'; everything after it is the
        // resource path, re-joined to the directory including the '/'.
        let path = match rest[pos..].find('/') {
            None => dir,
            Some(i) if pos + i + 1 == rest.len() => dir,
            Some(i) => std::format!("{dir}{}", &rest[pos + i..]),
        };
        return Ok(regular_paths(&path, &uri_om));
    }

    let path: &str = if scheme_match("file://") {
        &uri[7..]
    } else if uri.contains("://") {
        omc_assert!("Unknown URI schema: {uri}");
    } else {
        uri
    };
    Ok(regular_paths(path, &uri_om))
}

/// Extracts a substring from str.
/// start and stop are 1-based indices (first character is at index 1).
/// Fails for bogus start/stop values.
pub fn substring(str: ArcStr, start: i32, stop: i32) -> Result<ArcStr> {
    if start < 1 || stop < start || start > stop {
        bail!("Invalid substring range: start={}, stop={}", start, stop);
    }
    // `substring` is byte-indexed to match the rest of the MetaModelica
    // string surface (stringLength returns bytes via `.len()`, stringGet
    // returns a byte value). Treating these as char-based here caused
    // bytes/chars mismatches when callers reach for the indices returned
    // by `stringLength` — e.g. `stripBOM` would error with
    // "Stop index 8 exceeds string length 6" on a UTF-8 BOM input
    // because the BOM is 1 char but 3 bytes.
    let start_idx = (start - 1) as usize; // 1-based to 0-based
    let stop_idx = stop as usize;         // 1-based, inclusive -> exclusive
    if stop_idx > str.len() {
        bail!("Stop index {} exceeds string length {}", stop, str.len());
    }
    match str.get(start_idx..stop_idx) {
        Some(slice) => Ok(ArcStr::from(slice)),
        // The byte range falls inside a multi-byte UTF-8 sequence — there is
        // no valid string to return. Surface this rather than silently
        // producing nonsense; the call site should be rewritten to use
        // codepoint indices if that's what it meant.
        None => bail!(
            "substring({}, {}) does not fall on UTF-8 character boundaries",
            start, stop
        ),
    }
}

/// Alias for string_append_list (maps a list of single-char strings to one string).
pub fn listStringCharString(strs: Arc<List<ArcStr>>) -> ArcStr {
    stringAppendList(strs)
}

/// Alias for string_append_list (maps a list of single-char strings to one string).
pub fn stringCharListString(strs: Arc<List<ArcStr>>) -> ArcStr {
    stringAppendList(strs)
}

// ============================================================================
// List functions
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Default)]
pub enum List<T: Clone> {
    Cons{head: T, tail: Arc<List<T>>},
    #[default]
    Nil,
}

// Hand-written instead of `#[derive(Default)]`: the derive emits
// `impl<T: Clone + Default> Default for List<T>`, but the empty list is a
// valid default for *any* element type. The spurious `T: Default` bound
// otherwise blocks defaulting containers like `DoubleEnded.MutableList<T>`
// (whose fields are `Mutable<Arc<List<T>>>`) at `T: Clone`.

// Without this, dropping a list is recursive in its length (each node's
// `Arc<List>` field drops the next node from inside its own drop call), so
// releasing a list of a few hundred thousand elements overflows the stack —
// the C runtime never had this problem because its GC frees cells without
// walking the list. Iteratively unlink instead: detach each uniquely-owned
// tail before its node is dropped, so every node drop sees a Nil tail.
impl<T: Clone> Drop for List<T> {
    fn drop(&mut self) {
        // Fast path: Nil, or a node whose tail is already Nil — in
        // particular every node the unlink loop below has detached, so the
        // nested drop it triggers neither allocates nor recurses.
        let List::Cons { tail, .. } = self else { return };
        if matches!(&**tail, List::Nil) {
            return;
        }
        // One shared Nil per unlink walk; replacing a tail with a clone of
        // it is just a refcount increment.
        let nil: Arc<List<T>> = Arc::new(List::Nil);
        let mut cur = std::mem::replace(tail, nil.clone());
        loop {
            match Arc::get_mut(&mut cur) {
                // Sole owner of this node (strong count 1, no weak refs):
                // nothing else can observe the detached tail. Take it, then
                // let the node drop through the fast path above.
                Some(List::Cons { tail, .. }) => {
                    let next = std::mem::replace(tail, nil.clone());
                    cur = next;
                }
                // Nil, or a node shared with another live list: dropping our
                // reference only decrements the refcount and must leave the
                // suffix intact, so stop here.
                _ => break,
            }
        }
    }
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
        $crate::list!($($x),*)
    };
    // General case: peel off the first element and recurse
    ( $x:expr, $($rest:expr),+ ) => {
        $crate::cons($x, $crate::list!($($rest),+))
    };
    // Single element case
    ( $x:expr ) => {
        $crate::cons($x, $crate::list!())
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
        $base:ident => $variant:path ;
        $first_field:ident = $first_value:expr
        $(, $field:ident = $value:expr)*
        $(,)?
    ) => {{
        let mut __owned = (*$base).clone();
        // Evaluate every value expression BEFORE entering an `if let` that
        // would introduce field-shorthand pattern bindings with the same name
        // as the field. Otherwise a call site like
        //   `assign_variant_field!(t => T::N; value = value.clone())`
        // would have `value.clone()` resolve to the &mut FieldType binding
        // produced by the destructure, not the outer local — silently turning
        // the assignment into a self-copy. We capture each value into `__v`
        // immediately before its assignment; `__v` is shadowed each iteration,
        // which is fine because it's consumed before the next `let __v = ...`.
        let __v = $first_value;
        if let $variant { $first_field, .. } = &mut __owned {
            *$first_field = __v;
        } else {
            panic!(
                "assign_variant_field!: expected variant {} but value held a different variant",
                stringify!($variant),
            );
        }
        $(
            let __v = $value;
            if let $variant { $field, .. } = &mut __owned {
                *$field = __v;
            } else {
                panic!(
                    "assign_variant_field!: expected variant {} but value held a different variant",
                    stringify!($variant),
                );
            }
        )*
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
            Nil => None,
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
            iter = tail;
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
        false
    }
}

pub fn listAppend<T: Clone>(lst1: Arc<List<T>>, lst2: Arc<List<T>>) -> Arc<List<T>> {
    lst1.append(&lst2)
}

/// Free-function form of the `listReverse` builtin (the `List::reverse` method).
/// Direct calls lower to `x.reverse()`, but when `listReverse` is used as a
/// first-class value (e.g. `Array.map(arr, listReverse)`) there must be a real
/// function path to reference — methods cannot be named as `fn` items. Codegen
/// emits `fnptr!(metamodelica::listReverse, _)` for that case.
pub fn listReverse<T: Clone>(lst: Arc<List<T>>) -> Arc<List<T>> {
    lst.reverse()
}

pub fn listMember<T: Clone+PartialEq>(element: T, lst: Arc<List<T>>) -> bool {
    lst.contains(&element)
}

pub fn listHead<T: Clone>(lst: Arc<List<T>>) -> Result<T> {
    let Cons{head, ..} = &*lst else {bail!("Cannot get head of empty list")};
    Ok(head.clone())
}

pub fn listGet<T: Clone>(lst: Arc<List<T>>, i: i32) -> Result<T> {
    lst.get(i)
}

pub fn listEmpty<T: Clone>(lst: Arc<List<T>>) -> bool {
    lst.is_empty()
}

pub fn listDelete<T: Clone>(lst: Arc<List<T>>, index: i32) -> Result<Arc<List<T>>> {
    lst.delete(index)
}

pub fn listRest<T: Clone>(lst: Arc<List<T>>) -> Result<Arc<List<T>>> {
    match &*lst {
        Nil => bail!("Cannot get rest of empty list"),
        Cons{tail, ..} => Ok(tail.clone()),
    }
}

pub fn listLength<T: Clone>(lst: Arc<List<T>>) -> i32 {
    lst.len()
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
pub fn arrayLength<A>(arr: Array<A>) -> i32 {
    arr.borrow().len() as i32
}

/// Returns true if the array is empty. O(1).
pub fn arrayEmpty<A>(arr: Array<A>) -> bool {
    arr.borrow().is_empty()
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
pub fn arrayCreate<A: Clone>(size: i32, initial_value: A) -> Array<A> {
    if size <= 0 {
        return arrayFromVec(Vec::new());
    }
    arrayFromVec(vec![initial_value; size as usize])
}

/// Creates a new array of the given size, initialized with `A::default()`.
///
/// Used by codegen to lower `arrayCreateNoInit(size, dummy)` when the MM
/// dummy expression is a bare reference to a function-scope variable that
/// is never assigned at the call point (a common MM idiom: declare a
/// `protected SBInterval dummyi;` and pass it as the type witness only).
/// Such a reference cannot be forwarded as a Rust value, so we discard it
/// and rely on the `Default` impl for the element type instead. Types that
/// lack a `Default` impl will fail to compile at the use site — the fix is
/// to add a sensible `Default` for that type (often the "empty" or "first
/// variant" form).
pub fn arrayCreateDefault<A: Clone + Default>(size: i32) -> Array<A> {
    if size <= 0 {
        return arrayFromVec(Vec::new());
    }
    arrayFromVec(vec![A::default(); size as usize])
}

/// Converts an array to a list. O(n).
pub fn arrayList<A: Clone>(arr: Array<A>) -> Arc<List<A>> {
    let mut result = Arc::new(List::Nil);
    for item in arr.borrow().iter().rev().cloned() {
        result = List::cons(result, item);
    }
    result
}

/// Converts a list to an array. O(n).
pub fn listArray<A: Clone>(lst: Arc<List<A>>) -> Array<A> {
    let mut result = Vec::new();
    for item in &*lst {
        result.push(item.clone());
    }
    arrayFromVec(result)
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
pub fn arrayCopy<A: Clone>(arr: Array<A>) -> Array<A> {
    arrayFromVec(arr.borrow().clone())
}

/// Appends arr2 to arr1, creating a new array. O(length(arr1) + length(arr2)).
/// The result does not share storage with either input.
pub fn arrayAppend<A: Clone>(arr1: Array<A>, arr2: Array<A>) -> Array<A> {
    let mut result = arr1.borrow().clone();
    result.extend(arr2.borrow().iter().cloned());
    arrayFromVec(result)
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
pub fn anyString<A: std::fmt::Debug>(a: A) -> ArcStr {
    format!("{:?}", a)
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
pub fn valueEq<A: PartialEq>(a1: A, a2: A) -> bool {
    a1 == a2
}

/// Compares two Ord values.
/// Returns -1 if a1 < a2, 0 if a1 == a2, 1 if a1 > a2.
pub fn valueCompare<A: Ord>(a1: A, a2: A) -> i32 {
    match a1.cmp(&a2) {
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
pub fn referenceEq<A: ?Sized>(a1: &A, a2: &A) -> bool {
    // No `A: PartialEq` bound. The body only does pointer comparison, and
    // some MM types — e.g. anything transitively embedding an
    // `Arc<dyn Fn(...) + 'static>` callback (NF Type's
    // EvaluateSingletonType variant, and its containers down to
    // ComponentRef) — can no longer auto-derive `PartialEq`. Requiring
    // the bound here would lock those types out of every
    // `referenceEq(&a, &b)` site even though the bodies don't need it.
    //
    // `A: ?Sized` so the caller can deref through a shared-pointer handle
    // and compare the *pointee* — `referenceEq(&*s1, &*s2)` for `ArcStr`
    // arguments compares the `str` pointees. Two clones of the same handle
    // live at different stack addresses, so comparing the handles would
    // distinguish them even though they designate the same MM object; the
    // code generator therefore passes `&*v` for handle-represented values
    // (see mmtorust's `referenceEq` lowering). For unsized pointees
    // `std::ptr::eq` also compares the metadata (length / vtable), which is
    // identical for clones of the same handle.
    std::ptr::eq(a1 as *const A, a2 as *const A)
}

/// `#[derive(ReferenceEq)]` — emits a shallow field-wise [`ReferenceEq`]
/// impl; mmtorust adds it to every generated MM value type (see
/// `GenCtx::derives_for`).
pub use metamodelica_derive::ReferenceEq;

/// `#[derive(MetaCmp)]` — emits `PartialEq`/`PartialOrd`/`Ord` with an
/// `Arc::ptr_eq` fast path on every `Arc<…>` field, the faithful port of the
/// MMC runtime `valueCompare`'s pointer-identity short-circuit. mmtorust emits
/// it in place of the builtin `PartialEq`/`PartialOrd`/`Ord` derives (see
/// `GenCtx::derives_for`) so comparing the cyclic NF graphs terminates.
pub use metamodelica_derive::MetaCmp;

/// MetaModelica `referenceEq` as a trait, for *generic* contexts where the
/// operands' representation is opaque to the code generator (a bare type
/// parameter, or a type containing one). Concrete-typed sites are lowered
/// structurally instead — see `try_emit_reference_eq` in
/// mmtorust/src/codegen.rs; this trait's impls mirror that lowering exactly:
///
///   * scalars (Integer/Boolean/Real, MMC immediates or boxed-immutable)
///     compare by value;
///   * shared handles (`Arc`, `Rc` — lists, uniontype values, arrays,
///     callbacks) compare by allocation identity;
///   * `String` (`ArcStr`) compares the pointee `str` address, so clones of
///     one handle are identical but equal copies are not;
///   * options/tuples/records/enums compare component-wise (the derive).
///
/// The relation is *observational identity*: true whenever MMC's pointer
/// comparison would be true, and additionally true for distinct copies whose
/// components are pairwise identical — copies which exist only because this
/// port unboxes non-recursive records and clones values where MMC shares
/// pointers, and which no MM program can otherwise distinguish.
pub trait ReferenceEq {
    fn reference_eq(&self, other: &Self) -> bool;
}

impl ReferenceEq for i32 {
    fn reference_eq(&self, other: &Self) -> bool { self == other }
}
impl ReferenceEq for i64 {
    fn reference_eq(&self, other: &Self) -> bool { self == other }
}
impl ReferenceEq for bool {
    fn reference_eq(&self, other: &Self) -> bool { self == other }
}
impl ReferenceEq for () {
    fn reference_eq(&self, _other: &Self) -> bool { true }
}
/// MM `Real`. MMC boxes reals, so its referenceEq can distinguish equal
/// values in distinct boxes; an unboxed `f64` cannot. Value equality is the
/// observational-identity choice (`OrderedFloat` makes NaN equal itself,
/// so the relation stays reflexive like pointer identity is).
impl ReferenceEq for Real {
    fn reference_eq(&self, other: &Self) -> bool { self == other }
}
/// MM `String`: identity of the shared `str` allocation, like the
/// `referenceEq(&*s1, &*s2)` the concrete lowering emits.
impl ReferenceEq for ArcStr {
    fn reference_eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.as_str() as *const str, other.as_str() as *const str)
    }
}
/// Shared handles: allocation identity. Covers lists (`Arc<List<T>>`),
/// Arc-boxed uniontype values, and `Arc<dyn Fn(...)>` callbacks (`?Sized`).
impl<T: ?Sized> ReferenceEq for Arc<T> {
    fn reference_eq(&self, other: &Self) -> bool { Arc::ptr_eq(self, other) }
}
/// `Rc` handles: covers `Array<T>` (= `Rc<RefCell<Vec<T>>>`), whose MM
/// semantics are reference (aliasing) semantics — identity of the storage.
impl<T: ?Sized> ReferenceEq for Rc<T> {
    fn reference_eq(&self, other: &Self) -> bool { Rc::ptr_eq(self, other) }
}
/// `NONE()` is a runtime singleton in MMC, so two NONEs are identical;
/// SOME payloads compare recursively (same shape the concrete lowering
/// emits for `Option`).
impl<T: ReferenceEq> ReferenceEq for Option<T> {
    fn reference_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (None, None) => true,
            (Some(l), Some(r)) => l.reference_eq(r),
            _ => false,
        }
    }
}
/// MM tuples: component-wise.
macro_rules! impl_reference_eq_tuple {
    ($($name:ident : $idx:tt),+) => {
        impl<$($name: ReferenceEq),+> ReferenceEq for ($($name,)+) {
            fn reference_eq(&self, other: &Self) -> bool {
                true $(&& self.$idx.reference_eq(&other.$idx))+
            }
        }
    };
}
impl_reference_eq_tuple!(A: 0);
impl_reference_eq_tuple!(A: 0, B: 1);
impl_reference_eq_tuple!(A: 0, B: 1, C: 2);
impl_reference_eq_tuple!(A: 0, B: 1, C: 2, D: 3);
impl_reference_eq_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4);
impl_reference_eq_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5);
impl_reference_eq_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6);
impl_reference_eq_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7);
impl_reference_eq_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8);
impl_reference_eq_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9);
/// Bare `List<T>` (the usual MM representation is `Arc<List<T>>`, caught by
/// the `Arc` impl): shallow like the derive would emit — heads compare via
/// `ReferenceEq`, tails via `Arc` identity, so the comparison is O(1).
impl<T: Clone + ReferenceEq> ReferenceEq for List<T> {
    fn reference_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (List::Nil, List::Nil) => true,
            (List::Cons { head: lh, tail: lt }, List::Cons { head: rh, tail: rt }) => {
                lh.reference_eq(rh) && Arc::ptr_eq(lt, rt)
            }
            _ => false,
        }
    }
}
impl ReferenceEq for SourceInfo {
    fn reference_eq(&self, other: &Self) -> bool {
        self.fileName.reference_eq(&other.fileName)
            && self.isReadOnly == other.isReadOnly
            && self.lineNumberStart == other.lineNumberStart
            && self.columnNumberStart == other.columnNumberStart
            && self.lineNumberEnd == other.lineNumberEnd
            && self.columnNumberEnd == other.columnNumberEnd
            && self.lastModification == other.lastModification
    }
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

/// Returns the constructor tag for a value.
///
/// In MetaModelica `valueConstructor(v)` returns the variant index of a
/// boxed uniontype value (it is the *value* that matters, not its static
/// type — two values of the same uniontype but different records produce
/// different tags). In Rust we implement this using
/// [`std::mem::discriminant`], hashed into an `i32`.
///
/// For enums this yields a stable, distinct number per variant.  For
/// non-enum types `mem::discriminant` returns a single constant value (so
/// all instances hash to the same `i32`), which matches MetaModelica's
/// "records have a single constructor" semantics.
///
/// The caller is expected to pass `&value` — for `Arc<T>`-wrapped values
/// generated code must deref through the `Arc` (`&*arc`) so that the
/// inspected discriminant belongs to the inner enum, not to `Arc` itself.
pub fn valueConstructor<A>(value: &A) -> Result<i32> {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    std::mem::discriminant(value).hash(&mut hasher);
    Ok((hasher.finish() & 0x7FFF_FFFF) as i32)
}

/// Returns the current time in seconds relative to process start.
/// Not very accurate, intended for diff comparisons.
// `Instant::now` panics on wasm32-unknown-unknown (no clock); web-time backs it
// with the JS clock there.
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;
#[cfg(target_arch = "wasm32")]
use web_time::Instant;

fn getStartInstant() -> Instant {
    use std::sync::OnceLock;
    static START: OnceLock<Instant> = OnceLock::new();
    *START.get_or_init(Instant::now)
}

pub fn clock() -> Real {
    OrderedFloat(getStartInstant().elapsed().as_secs_f64())
}

// ============================================================================
// Option functions
// ============================================================================

/// Returns true if the Option is NONE.
pub fn isNone<A>(opt: Option<A>) -> bool {
    opt.is_none()
}

/// Returns true if the Option is SOME.
pub fn isSome<A>(opt: Option<A>) -> bool {
    opt.is_some()
}

// ============================================================================
// Misc builtin functions
// ============================================================================

/// Sets the stack overflow signal to the given value and returns the old one.
/// In this translation, simply returns the input value. Infallible (see the
/// `Infallible` classification in mmtorust's fallibility analysis), so it
/// returns a bare `bool` — call sites do not add `?`.
pub fn setStackOverflowSignal(in_signal: bool) -> bool {
    in_signal
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
// metamodelica::ext - Hand-written replacements for `external "C"` runtime
//                     functions that the MetaModelica source declares as
//                     FFI shims. The codegen consults
//                     `mmtorust::external_c_calls::external_c_impl_path`
//                     and emits delegating calls into this module.
// ============================================================================

#[allow(non_snake_case)]
pub mod ext {
    /// C `atof`-style parse: the longest valid floating-point prefix (after
    /// leading whitespace), `0.0` when there is none. Hand-written ports of
    /// runtime C code use this where the original called `atof`/`strtod` on
    /// machine-written text, so trailing junk must be tolerated rather than
    /// turned into an error (`str::parse` would reject it).
    pub fn c_atof(s: &str) -> f64 {
        let s = s.trim_start();
        let bytes = s.as_bytes();
        let mut end = 0;
        // optional sign
        if end < bytes.len() && (bytes[end] == b'+' || bytes[end] == b'-') {
            end += 1;
        }
        let mut seen_digit = false;
        while end < bytes.len() && bytes[end].is_ascii_digit() {
            end += 1;
            seen_digit = true;
        }
        if end < bytes.len() && bytes[end] == b'.' {
            end += 1;
            while end < bytes.len() && bytes[end].is_ascii_digit() {
                end += 1;
                seen_digit = true;
            }
        }
        if !seen_digit {
            return 0.0;
        }
        // optional exponent (only consumed when it has at least one digit)
        if end < bytes.len() && (bytes[end] == b'e' || bytes[end] == b'E') {
            let mut exp_end = end + 1;
            if exp_end < bytes.len() && (bytes[exp_end] == b'+' || bytes[exp_end] == b'-') {
                exp_end += 1;
            }
            let exp_digits_start = exp_end;
            while exp_end < bytes.len() && bytes[exp_end].is_ascii_digit() {
                exp_end += 1;
            }
            if exp_end > exp_digits_start {
                end = exp_end;
            }
        }
        s[..end].parse::<f64>().unwrap_or(0.0)
    }

    /// C `atoi`/`strtol(s, NULL, 10)`-style parse: the longest decimal-integer
    /// prefix (after leading whitespace), `0` when there is none.
    pub fn c_atol(s: &str) -> i64 {
        let s = s.trim_start();
        let bytes = s.as_bytes();
        let mut end = 0;
        if end < bytes.len() && (bytes[end] == b'+' || bytes[end] == b'-') {
            end += 1;
        }
        let digits_start = end;
        while end < bytes.len() && bytes[end].is_ascii_digit() {
            end += 1;
        }
        if end == digits_start {
            return 0;
        }
        s[..end].parse::<i64>().unwrap_or(0)
    }
}

// ============================================================================
// MetaModelica::Dangerous - Functions that skip bounds checking
// ============================================================================

pub mod Dangerous {
    pub use super::*;
    /// Unsafe array get without bounds checking.
    /// Panics in debug mode if index is out of bounds due to Rust's bounds checking on indexing.
    pub fn arrayGetNoBoundsChecking<A: Clone>(arr: Array<A>, index: i32) -> A {
        let idx = (index - 1) as usize; // 1-based to 0-based
        let v = arr.borrow();
        // SAFETY: Caller must ensure index is in bounds.
        unsafe { v.get_unchecked(idx).clone() }
    }

    /// Unsafe array update without bounds checking.
    /// Mutates the underlying storage in place; visible through every alias.
    pub fn arrayUpdateNoBoundsChecking<A: Clone>(arr: Array<A>, index: i32, new_value: A) -> Array<A> {
        let idx = (index - 1) as usize; // 1-based to 0-based
        {
            let mut v = arr.borrow_mut();
            // SAFETY: Caller must ensure index is in bounds.
            unsafe { *v.get_unchecked_mut(idx) = new_value; }
        }
        arr
    }

    /// Unsafe array clearing without bounds checking.
    /// Mutates the underlying storage in place; visible through every alias.
    ///
    /// This is intentionally a **no-op** in the Rust translation.
    ///
    /// In the original MetaModelica C/GC runtime the function nulled out the
    /// slot to release the GC reference early.  In Rust we rely on `Arc<T>`
    /// for lifetime management: the slot holds a valid, live `Arc<T>`, and it
    /// will be properly decremented when the slot is overwritten or when the
    /// backing `Vec<T>` is freed.  Calling `drop_in_place` here and then
    /// writing zero bytes would leave an invalid (null) `Arc<T>` in the slot;
    /// `Vec::drop` would later try to drop that zeroed value, which dereferences
    /// a null pointer → SIGSEGV.
    #[inline(always)]
    pub fn arrayClearIndex<A: Clone>(_arr: Array<A>, _index: i32) {}

    /// Write `val` into an uninitialised slot created by `arrayCreateNoInit`.
    ///
    /// Uses `std::ptr::write` so the garbage bytes that occupy the slot are
    /// **not** interpreted as a live `A` value (no drop is called on them).
    /// Returns the array so the call can be used as an expression, matching
    /// the shape of the regular `arrayUpdate` codegen.
    ///
    /// # Safety
    /// * `index` is 1-based and must be in bounds.
    /// * The slot at `index - 1` must be genuinely uninitialised — it must
    ///   never have been written via this function or via a regular assignment.
    ///   Writing into an already-initialised slot leaks the old value.
    pub unsafe fn arrayInitSlot<A>(arr: Array<A>, index: i32, val: A) -> Array<A> {
        {
            let mut borrow = arr.borrow_mut();
            // SAFETY: contract requires index to be in-bounds and the slot uninitialised.
            #[allow(unsafe_op_in_unsafe_fn)]
            let p = unsafe { borrow.get_unchecked_mut((index - 1) as usize) as *mut A };
            #[allow(unsafe_op_in_unsafe_fn)]
            unsafe { std::ptr::write(p, val) };
        }
        arr
    }

    /// Creates a new array with uninitialized elements.
    /// The MetaModelica signature takes a `dummy` argument purely as a type witness;
    /// the codegen drops it because Rust generics already carry the element type.
    pub fn arrayCreateNoInit<A: Clone>(size: i32) -> Array<A> {
        let mut v = Vec::with_capacity(size as usize);
        // SAFETY:
        // 1. We allocated capacity for `size` elements.
        // 2. Caller guarantees every element is initialized before being read.
        unsafe {
            v.set_len(size as usize);
        }
        arrayFromVec(v)
    }
    /// Unsafe string get without bounds checking.
    ///
    /// Mirrors `stringGet`'s `ArcStr` parameter (MetaModelica `String` values
    /// are `ArcStr` in the translation); `ArcStr` derefs to `str` so
    /// `as_bytes()` works directly. This is the *dangerous*, no-bounds-checking
    /// variant: it performs an unchecked read and therefore returns the raw
    /// `i32` byte value, never a `Result`. The caller is responsible for
    /// supplying an in-bounds index — matching the MetaModelica
    /// `MetaModelica.Dangerous.stringGetNoBoundsChecking` contract.
    pub fn stringGetNoBoundsChecking(str: ArcStr, index: i32) -> i32 {
        let idx = (index - 1) as usize; // 1-based to 0-based
        // SAFETY: Caller must ensure index is in bounds.
        unsafe { (*str.as_bytes().get_unchecked(idx)) as i32 }
    }
    /// Reverses a list in place, destructively.
    ///
    /// Walks the spine and repoints each `Cons` cell's `tail` at the cell that
    /// preceded it, mutating the cells through a raw pointer (the same
    /// dangerous mechanism as `listSetRest`). No new cells are allocated.
    ///
    /// SAFETY / semantics: this mirrors the MetaModelica runtime's destructive
    /// `listReverseInPlace`. Every other holder of a clone of these cons cells
    /// observes the reversal, and the input list head no longer denotes the
    /// same sequence. Only call on a freshly built list that is not shared and
    /// not read concurrently.
    pub fn listReverseInPlace<T: Clone>(list: Arc<List<T>>) -> Arc<List<T>> {
        let mut prev: Arc<List<T>> = nil();
        let mut curr: Arc<List<T>> = list;
        while let List::Cons { tail, .. } = &*curr {
            let next = tail.clone();
            // SAFETY: see the method doc — the caller guarantees the cells are
            // uniquely owned (freshly built) and not read concurrently.
            unsafe {
                let p = Arc::as_ptr(&curr) as *mut List<T>;
                if let List::Cons { tail, .. } = &mut *p {
                    *tail = prev;
                }
            }
            prev = curr;
            curr = next;
        }
        prev
    }
    /// Destructively appends `second` onto the end of `first`: walks to the last
    /// cons cell of `first` and repoints its `tail` at `second`. Allocates
    /// nothing. Mirrors the MetaModelica runtime's `listAppendDestroy`
    /// (`SimulationRuntime/c/meta/meta_modelica_builtin.c`).
    ///
    /// SAFETY: mutates `first`'s last cell through a raw pointer (the same
    /// mechanism as [`listSetRest`]), so every holder of a clone of that cell
    /// observes the splice. The MetaModelica contract is that `first` is
    /// "destroyed" — the caller must not keep using it as its original sequence,
    /// and the cells must not be read concurrently.
    pub fn listAppendDestroy<T: Clone>(first: Arc<List<T>>, second: Arc<List<T>>) -> Arc<List<T>> {
        // An empty first list has no cell to repoint; the result is `second`.
        if matches!(&*first, List::Nil) {
            return second;
        }
        // Walk to the last cons cell (the one whose tail is Nil).
        let mut lst = first.clone();
        loop {
            let next = match &*lst {
                List::Cons { tail, .. } if !matches!(&**tail, List::Nil) => tail.clone(),
                _ => break,
            };
            lst = next;
        }
        // SAFETY: see the doc comment — `first`'s cells are destroyed/uniquely
        // held by the caller and not read concurrently.
        unsafe {
            let p = Arc::as_ptr(&lst) as *mut List<T>;
            if let List::Cons { tail, .. } = &mut *p {
                *tail = second;
            }
        }
        first
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
            assert!(boolAnd(true, true));
            assert!(!boolAnd(true, false));
            assert!(!boolAnd(false, true));
            assert!(!boolAnd(false, false));
        }

        #[test]
        fn test_bool_or() {
            assert!(boolOr(true, false));
            assert!(boolOr(false, true));
            assert!(boolOr(true, true));
            assert!(!boolOr(false, false));
        }

        #[test]
        fn test_bool_not() {
            assert!(boolNot(false));
            assert!(!boolNot(true));
        }

        #[test]
        fn test_bool_eq() {
            assert!(boolEq(true, true));
            assert!(boolEq(false, false));
            assert!(!boolEq(true, false));
            assert!(!boolEq(false, true));
        }

        #[test]
        fn test_bool_string() {
            assert_eq!(&*boolString(true), "true");
            assert_eq!(&*boolString(false), "false");
        }
    }

    // =========================================================================
    // Integer arithmetic tests
    // =========================================================================

    mod int_arithmetic_tests {
        use super::*;

        #[test]
        fn test_int_add() {
            assert_eq!(intAdd(1, 2), 3);
            assert_eq!(intAdd(-1, 1), 0);
            assert_eq!(intAdd(-1, -2), -3);
        }

        #[test]
        fn test_int_sub() {
            assert_eq!(intSub(5, 3), 2);
            assert_eq!(intSub(3, 5), -2);
            assert_eq!(intSub(0, 0), 0);
        }

        #[test]
        fn test_int_mul() {
            assert_eq!(intMul(3, 4), 12);
            assert_eq!(intMul(-3, 4), -12);
            assert_eq!(intMul(-3, -4), 12);
            assert_eq!(intMul(0, 100), 0);
        }

        #[test]
        fn test_int_div() {
            assert_eq!(intDiv(10, 3), 3);
            assert_eq!(intDiv(10, -3), -3);
            assert_eq!(intDiv(-10, 3), -3);
            assert_eq!(intDiv(-10, -3), 3);
        }

        #[test]
        fn test_int_mod() {
            // Modelica mod: result has the same sign as the divisor.
            assert_eq!(intMod(10, 3), 1);
            assert_eq!(intMod(10, -3), -2);
            assert_eq!(intMod(-10, 3), 2);
            assert_eq!(intMod(-10, -3), -1);
        }

        #[test]
        fn test_int_max() {
            assert_eq!(intMax(1, 2), 2);
            assert_eq!(intMax(2, 1), 2);
            assert_eq!(intMax(5, 5), 5);
            assert_eq!(intMax(-1, -2), -1);
        }

        #[test]
        fn test_int_min() {
            assert_eq!(intMin(1, 2), 1);
            assert_eq!(intMin(2, 1), 1);
            assert_eq!(intMin(5, 5), 5);
            assert_eq!(intMin(-1, -2), -2);
        }

        #[test]
        fn test_int_abs() {
            assert_eq!(intAbs(-5), 5);
            assert_eq!(intAbs(5), 5);
            assert_eq!(intAbs(0), 0);
        }

        #[test]
        fn test_int_neg() {
            assert_eq!(intNeg(5), -5);
            assert_eq!(intNeg(-5), 5);
            assert_eq!(intNeg(0), 0);
        }
    }

    // =========================================================================
    // Integer comparison tests
    // =========================================================================

    mod int_comparison_tests {
        use super::*;

        #[test]
        fn test_int_lt() {
            assert!(intLt(1, 2));
            assert!(!intLt(2, 2));
            assert!(!intLt(2, 1));
        }

        #[test]
        fn test_int_le() {
            assert!(intLe(1, 2));
            assert!(intLe(2, 2));
            assert!(!intLe(2, 1));
        }

        #[test]
        fn test_int_eq() {
            assert!(intEq(5, 5));
            assert!(!intEq(5, 6));
        }

        #[test]
        fn test_int_ne() {
            assert!(intNe(5, 6));
            assert!(!intNe(5, 5));
        }

        #[test]
        fn test_int_ge() {
            assert!(intGe(2, 1));
            assert!(intGe(2, 2));
            assert!(!intGe(1, 2));
        }

        #[test]
        fn test_int_gt() {
            assert!(intGt(2, 1));
            assert!(!intGt(2, 2));
            assert!(!intGt(1, 2));
        }
    }

    // =========================================================================
    // Integer bitwise tests
    // =========================================================================

    mod int_bitwise_tests {
        use super::*;

        #[test]
        fn test_int_bit_not() {
            assert_eq!(intBitNot(0i32), -1);
            assert_eq!(intBitNot(-1i32), 0);
            assert_eq!(intBitNot(1), !1);
        }

        #[test]
        fn test_int_bit_and() {
            assert_eq!(intBitAnd(0b1100, 0b1010), 0b1000);
            assert_eq!(intBitAnd(0, 5), 0);
        }

        #[test]
        fn test_int_bit_or() {
            assert_eq!(intBitOr(0b1100, 0b1010), 0b1110);
            assert_eq!(intBitOr(0, 5), 5);
        }

        #[test]
        fn test_int_bit_xor() {
            assert_eq!(intBitXor(0b1100, 0b1010), 0b0110);
            assert_eq!(intBitXor(5, 5), 0);
        }

        #[test]
        fn test_int_bit_l_shift() {
            assert_eq!(intBitLShift(1, 3), 8);
            assert_eq!(intBitLShift(3, 1), 6);
        }

        #[test]
        fn test_int_bit_r_shift() {
            assert_eq!(intBitRShift(8, 3), 1);
            assert_eq!(intBitRShift(6, 1), 3);
        }
    }

    // =========================================================================
    // Integer conversion tests
    // =========================================================================

    mod int_conversion_tests {
        use super::*;

        #[test]
        fn test_int_real() {
            assert_eq!(intReal(42), OrderedFloat(42.0_f64));
            assert_eq!(intReal(-7), OrderedFloat(-7.0_f64));
        }

        #[test]
        fn test_int_string() {
            assert_eq!(&*intString(42), "42");
            assert_eq!(&*intString(-7), "-7");
            assert_eq!(&*intString(0), "0");
        }
    }

    // =========================================================================
    // Real arithmetic tests
    // =========================================================================

    mod real_arithmetic_tests {
        use super::*;
        fn r(x: f64) -> Real { OrderedFloat(x) }

        #[test]
        fn test_real_add() {
            assert_eq!(realAdd(r(1.5), r(2.5)), r(4.0));
            assert_eq!(realAdd(r(-1.0), r(1.0)), r(0.0));
        }

        #[test]
        fn test_real_sub() {
            assert_eq!(realSub(r(5.0), r(3.0)), r(2.0));
            assert_eq!(realSub(r(3.0), r(5.0)), r(-2.0));
        }

        #[test]
        fn test_real_mul() {
            assert_eq!(realMul(r(3.0), r(4.0)), r(12.0));
            assert_eq!(realMul(r(-3.0), r(4.0)), r(-12.0));
        }

        #[test]
        fn test_real_div() {
            assert_eq!(realDiv(r(10.0), r(3.0)), r(10.0 / 3.0));
            assert_eq!(realDiv(r(6.0), r(2.0)), r(3.0));
        }

        #[test]
        fn test_real_mod() {
            assert_eq!(realMod(r(10.0), r(3.0)), r(1.0));
            assert_eq!(realMod(r(10.5), r(3.0)), r(1.5));
        }

        #[test]
        fn test_real_pow() {
            assert_eq!(realPow(r(2.0), r(3.0)), r(8.0));
            assert_eq!(realPow(r(9.0), r(0.5)), r(3.0));
        }

        #[test]
        fn test_real_max() {
            assert_eq!(realMax(r(1.5), r(2.5)), r(2.5));
            assert_eq!(realMax(r(5.0), r(5.0)), r(5.0));
        }

        #[test]
        fn test_real_min() {
            assert_eq!(realMin(r(1.5), r(2.5)), r(1.5));
            assert_eq!(realMin(r(5.0), r(5.0)), r(5.0));
        }

        #[test]
        fn test_real_abs() {
            assert_eq!(realAbs(r(-5.5)), r(5.5));
            assert_eq!(realAbs(r(5.5)), r(5.5));
        }

        #[test]
        fn test_real_almost_eq() {
            assert!(realAlmostEq(r(1.0), r(1.0000001), r(1e-5)));
            assert!(!realAlmostEq(r(1.0), r(1.1), r(1e-5)));
            assert!(realAlmostEq(r(1.0), r(1.0), r(1e-6)));
        }

        #[test]
        fn test_real_neg() {
            assert_eq!(realNeg(r(5.5)), r(-5.5));
            assert_eq!(realNeg(r(-5.5)), r(5.5));
        }
    }

    // =========================================================================
    // Real comparison tests
    // =========================================================================

    mod real_comparison_tests {
        use super::*;
        fn r(x: f64) -> Real { OrderedFloat(x) }

        #[test]
        fn test_real_lt() {
            assert!(realLt(r(1.0), r(2.0)));
            assert!(!realLt(r(2.0), r(2.0)));
            assert!(!realLt(r(2.0), r(1.0)));
        }

        #[test]
        fn test_real_le() {
            assert!(realLe(r(1.0), r(2.0)));
            assert!(realLe(r(2.0), r(2.0)));
            assert!(!realLe(r(2.0), r(1.0)));
        }

        #[test]
        fn test_real_eq() {
            assert!(realEq(r(1.0), r(1.0)));
            assert!(!realEq(r(1.0), r(2.0)));
        }

        #[test]
        fn test_real_ne() {
            assert!(realNe(r(1.0), r(2.0)));
            assert!(!realNe(r(1.0), r(1.0)));
        }

        #[test]
        fn test_real_ge() {
            assert!(realGe(r(2.0), r(1.0)));
            assert!(realGe(r(2.0), r(2.0)));
            assert!(!realGe(r(1.0), r(2.0)));
        }

        #[test]
        fn test_real_gt() {
            assert!(realGt(r(2.0), r(1.0)));
            assert!(!realGt(r(2.0), r(2.0)));
            assert!(!realGt(r(1.0), r(2.0)));
        }
    }

    // =========================================================================
    // Real conversion tests
    // =========================================================================

    mod real_conversion_tests {
        use super::*;
        fn r(x: f64) -> Real { OrderedFloat(x) }

        #[test]
        fn test_real_int() {
            assert_eq!(realInt(r(3.7)), 3);
            assert_eq!(realInt(r(-3.7)), -3);
            assert_eq!(realInt(r(3.0)), 3);
        }

        #[test]
        fn test_real_string() {
            assert_eq!(&*realString(r(3.14)), "3.14");
            assert_eq!(&*realString(r(0.0)), "0.0");
            assert_eq!(&*realString(r(-1.5)), "-1.5");
            assert_eq!(&*realString(r(3.0)), "3.0");
            assert_eq!(&*realString(r(f64::INFINITY)), "inf");
            assert_eq!(&*realString(r(f64::NEG_INFINITY)), "-inf");
            assert_eq!(&*realString(r(f64::NAN)), "NaN");
        }

        /// The `real_output=1` vectors from om_format.c's TEST_RYU_TO_HR main.
        #[test]
        fn test_ryu_to_hr_real() {
            for (input, expected) in [
                ("8e5", "8e5"),
                ("8e4", "8e4"),
                ("8e3", "8000.0"),
                ("8e2", "800.0"),
                ("8e1", "80.0"),
                ("8e0", "8.0"),
                ("8e-1", "0.8"),
                ("8e-2", "0.08"),
                ("8e-3", "0.008"),
                ("8e-4", "8e-4"),
                ("8e-5", "8e-5"),
                ("8.13e6", "8.13e6"),
                ("8.13e5", "813000.0"),
                ("8.13e4", "81300.0"),
                ("8.13e3", "8130.0"),
                ("8.13e2", "813.0"),
                ("8.13e1", "81.3"),
                ("8.13e0", "8.13"),
                ("8.13e-1", "0.813"),
                ("8.13e-2", "0.0813"),
                ("8.13e-3", "0.00813"),
                ("8.13e-4", "8.13e-4"),
                ("8.13e-5", "8.13e-5"),
                ("8.1234567e6", "8.1234567e6"),
                ("8.1234567e5", "812345.67"),
                ("8.1234567e4", "81234.567"),
                ("8.1234567e0", "8.1234567"),
                ("8.1234567e-3", "0.0081234567"),
                ("8.1234567e-4", "8.1234567e-4"),
                ("-1.2e1", "-12.0"),
                ("-4.56e8", "-4.56e8"),
                ("1e-60", "1e-60"),
                ("1e80", "1e80"),
                ("NaN", "NaN"),
                ("Inf", "Inf"),
                ("-Inf", "-Inf"),
                ("9.499999999999999e2", "949.9999999999999"),
                ("1.000000000000002e0", "1.000000000000002"),
                ("-9.499999999999999e2", "-949.9999999999999"),
            ] {
                assert_eq!(ryu_to_hr(input, true), expected, "ryu_to_hr({input:?}, true)");
            }
        }

        /// The `real_output=0` vectors from om_format.c's TEST_RYU_TO_HR main.
        #[test]
        fn test_ryu_to_hr_plain() {
            for (input, expected) in [
                ("8e3", "8000"),
                ("8e0", "8"),
                ("8e-3", "0.008"),
                ("8e-4", "8e-4"),
                ("8.13e2", "813"),
                ("8.13e1", "81.3"),
                ("-1.2e1", "-12"),
                ("9.499999999999999e2", "950"),
                ("1.9999999999999998e8", "2e8"),
                ("1.9999999999999998e-6", "2e-6"),
                ("-9.499999999999999e2", "-950"),
                ("1.000000000000002e0", "1"),
                ("1.0000000000000022e0", "1"),
                ("9.99999999999999e-13", "1e-12"),
                ("9.99999999999999e-5", "1e-4"),
                ("9.99999999999999e-2", "0.1"),
                ("9.99999999999999e-1", "1"),
                ("9.99999999999999e0", "10"),
                ("9.99999999999999e1", "100"),
                ("9.99999999999999e5", "1e6"),
                ("9.99999999999999e11", "1e12"),
                ("-9.99999999999999e1", "-100"),
                ("1.234567890123456e6", "1.234567890123456e6"),
                ("1.2e6", "1.2e6"),
                ("1e6", "1e6"),
            ] {
                assert_eq!(ryu_to_hr(input, false), expected, "ryu_to_hr({input:?}, false)");
            }
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
            assert_eq!(&*intStringChar(65), "A");
            assert_eq!(&*intStringChar(97), "a");
            assert_eq!(&*intStringChar(48), "0");
            assert_eq!(&*intStringChar(0), "\0");
        }

        #[test]
        fn test_string_int() {
            assert_eq!(stringInt(literal!("42")).unwrap(), 42);
            assert_eq!(stringInt(literal!("-7")).unwrap(), -7);
            assert!(stringInt(literal!("not_a_number")).is_err());
            // strtol semantics: leading whitespace is skipped, trailing
            // junk (including whitespace) is an error.
            assert_eq!(stringInt(literal!("  42")).unwrap(), 42);
            assert!(stringInt(literal!("42 ")).is_err());
        }

        #[test]
        fn test_string_real() {
            assert_eq!(stringReal(literal!("3.14")).unwrap(), OrderedFloat(3.14));
            assert_eq!(stringReal(literal!("-2.5")).unwrap(), OrderedFloat(-2.5));
            assert!(stringReal(literal!("not_a_number")).is_err());
            // strtod semantics: leading whitespace is skipped (e.g. a CSV
            // column " 2.10"), trailing junk is an error.
            assert_eq!(stringReal(literal!(" 2.10")).unwrap(), OrderedFloat(2.10));
            assert!(stringReal(literal!("2.10 x")).is_err());
        }

        #[test]
        fn test_string_list_string_char() {
            let result = stringListStringChar(literal!("abc "));
            assert_eq!(&*result, &List::from_iter([literal!("a"), literal!("b"), literal!("c"), literal!(" ")]));
        }

        #[test]
        fn test_string_append_list() {
            let strs = list![literal!("hello"), literal!(" "), literal!("world")];
            assert_eq!(&*stringAppendList(strs), "hello world");
        }

        #[test]
        fn test_string_delimit_list() {
            let strs: Arc<List<ArcStr>> = list![literal!("x"), literal!("y"), literal!("z")];
            assert_eq!(stringDelimitList(strs, literal!(", ")), "x, y, z");
        }
    }

    // =========================================================================
    // String length and empty tests
    // =========================================================================

    mod string_length_tests {
        use super::*;

        #[test]
        fn test_string_length() {
            assert_eq!(stringLength("hello".into()), 5);
            assert_eq!(stringLength("".into()), 0);
        }

        #[test]
        fn test_string_empty() {
            assert!(stringEmpty("".into()));
            assert!(!stringEmpty("hello".into()));
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
            assert_eq!(stringAppend(literal!("hello"), literal!(" world")), literal!("hello world"));
            assert_eq!(stringAppend(literal!(""), literal!("hello")), literal!("hello"));
            assert_eq!(stringAppend(literal!("hello"), literal!("")), literal!("hello"));
        }

        #[test]
        fn test_string_eq() {
            assert!(stringEq(literal!("abc"), literal!("abc")));
            assert!(!stringEq(literal!("abc"), literal!("abd")));
            assert!(!stringEq(literal!(""), literal!("abc")));
        }

        #[test]
        fn test_string_equal() {
            assert!(stringEqual(literal!("abc"), literal!("abc")));
            assert!(!stringEqual(literal!("abc"), literal!("abd")));
        }
    }

    // =========================================================================
    // String compare test
    // =========================================================================

    mod string_compare_test {
        use super::*;

        #[test]
        fn test_string_compare() {
            assert!(stringCompare(literal!("abc"), literal!("abd")) < 0);
            assert_eq!(stringCompare(literal!("abc"), literal!("abc")), 0);
            assert!(stringCompare(literal!("abd"), literal!("abc")) > 0);
            assert!(stringCompare(literal!("ab"), literal!("abc")) < 0);
            assert!(stringCompare(literal!("abc"), literal!("ab")) > 0);
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
            assert_eq!(stringHashDjb2(literal!("a")), 5381_i32.wrapping_mul(33).wrapping_add(97));
            assert_eq!(stringHashDjb2(literal!("")), 5381);
        }

        #[test]
        fn test_string_hash_djb2_continue() {
            // Starting from the hash of "ab" and adding "c" gives the same as
            // hashing "abc" from scratch. The strings are kept short so every
            // intermediate value fits a non-negative i32: the `Continue` API
            // passes the running hash as an `Integer` (i32), so the chain
            // identity only holds while no intermediate has been reduced (it
            // matches the C runtime, whose `labs()`/64-bit width has the same
            // boundary limitation).
            let h1 = stringHashDjb2(literal!("ab"));
            let combined = stringHashDjb2Continue(literal!("c"), h1);
            assert_eq!(combined, stringHashDjb2(literal!("abc")));
        }

        #[test]
        fn test_string_hash_djb2_mod() {
            let h = stringHashDjb2Mod(literal!("hello"), 100);
            assert!(h >= 0 && h < 100);
            assert_eq!(stringHashDjb2Mod(literal!("hello"), 0), 0);
        }

        #[test]
        fn test_string_hash_sdbm() {
            // SDBM of "a" = 97 + 0 + 0 - 0 = 97
            assert_eq!(stringHashSdbm(literal!("a")), 97);
            assert_eq!(stringHashSdbm(literal!("")), 0);
        }

        #[test]
        fn test_string_hash_consistency() {
            // Same string should produce same hash
            assert_eq!(stringHash(literal!("test")), stringHash(literal!("test")));
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
            assert_eq!(&*listStringCharString(strs), "abc");
        }

        #[test]
        fn test_string_char_list_string() {
            let strs: Arc<List<ArcStr>> = list![literal!("a"), literal!("b"), literal!("c")];
            assert_eq!(&*stringCharListString(strs), "abc");
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
            assert_eq!(arrayLength(arr(vec![1, 2, 3])), 3);
            assert_eq!(arrayLength(arr::<i32>(vec![])), 0);
        }

        #[test]
        fn test_array_empty() {
            assert!(!arrayEmpty(arr(vec![1, 2, 3])));
            assert!(arrayEmpty(arr::<i32>(vec![])));
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
            let a = arrayCreate(5, 0);
            assert_eq!(*a.borrow(), vec![0, 0, 0, 0, 0]);
            let empty: Array<i32> = arrayCreate(0, 42);
            assert!(empty.borrow().is_empty());
        }

        #[test]
        fn test_array_list() {
            let a = arr(vec![1, 2, 3]);
            let lst = arrayList(a);
            assert_eq!(lst, list![1, 2, 3]);
        }

        #[test]
        fn test_list_array() {
            let lst = list![1, 2, 3];
            let a = listArray(lst);
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
            let copy = arrayCopy(a.clone());
            assert_eq!(*copy.borrow(), vec![1, 2, 3]);
            // arrayCopy must NOT share storage with the source.
            arrayUpdate(a, 1, 99).unwrap();
            assert_eq!(*copy.borrow(), vec![1, 2, 3]);
        }

        #[test]
        fn test_array_append() {
            let a = arr(vec![1, 2]);
            let b = arr(vec![3, 4]);
            assert_eq!(*arrayAppend(a.clone(), b.clone()).borrow(), vec![1, 2, 3, 4]);

            let empty: Array<i32> = arr(vec![]);
            assert_eq!(*arrayAppend(empty.clone(), b).borrow(), vec![3, 4]);
            assert_eq!(*arrayAppend(a, empty).borrow(), vec![1, 2]);
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
            assert_eq!(&*result, "42");

            let s = "hello";
            assert!(anyString(&s).contains("hello"));
        }

        #[test]
        fn test_tick() {
            let t1 = tick();
            let t2 = tick();
            assert_eq!(t2, t1+1);
        }

        #[test]
        fn test_value_eq() {
            let a = vec![1, 2, 3];
            let b = vec![1, 2, 3];
            let c = vec![1, 2, 4];
            assert!(valueEq(&a, &b));
            assert!(!valueEq(&a, &c));
        }

        #[test]
        fn test_value_compare() {
            assert_eq!(valueCompare(&1, &2), -1);
            assert_eq!(valueCompare(&2, &2), 0);
            assert_eq!(valueCompare(&3, &2), 1);

            assert_eq!(valueCompare(&"abc", &"abd"), -1);
            assert_eq!(valueCompare(&"abc", &"abc"), 0);
            assert_eq!(valueCompare(&"abd", &"abc"), 1);
        }

        #[test]
        fn test_reference_eq() {
            let a = 42;
            let b = 42;
            // Same reference should be equal
            assert!(referenceEq(&a, &a));
            // Different references with same value
            // reference_eq checks pointer equality, so different vars may not be equal
            assert!(referenceEq(&a, &b) || !referenceEq(&a, &b)); // either is valid
        }

        #[test]
        fn test_reference_arc() {
            let a = Arc::new(42);
            let b = a.clone();
            // Comparing the Arc *handles* distinguishes clones — they are two
            // distinct stack objects even though they share the pointee. This
            // is why generated code must not compare handles directly.
            assert!(!referenceEq(&a, &b));
            // Comparing the *pointees* (`&*v`, as the code generator emits for
            // handle-represented values) identifies clones of the same Arc.
            assert!(referenceEq(&*a, &*b));
            // A separate allocation with an equal value is NOT reference-equal.
            let c = Arc::new(42);
            assert!(!referenceEq(&*a, &*c));
        }

        #[test]
        fn test_reference_eq_str_pointee() {
            // `A: ?Sized` lets callers compare unsized pointees: clones of the
            // same ArcStr share storage (address + length both match)…
            let s1 = ArcStr::from("hello");
            let s2 = s1.clone();
            assert!(referenceEq(&*s1, &*s2));
            // …while an equal-valued but separately allocated string differs.
            let s3 = ArcStr::from("hello");
            assert!(!referenceEq(&*s1, &*s3));
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
            // MetaModelica semantics: same variant → same tag; different
            // variants of the same uniontype → different tags. Implemented
            // via `std::mem::discriminant`, so values of the same enum
            // variant compare equal (e.g. `Some(1)` and `Some(2)`), while
            // values of different variants compare unequal.
            #[allow(dead_code)]
            enum E { A(i32), B(i32), C }
            let a1 = valueConstructor(&E::A(1)).unwrap();
            let a2 = valueConstructor(&E::A(99)).unwrap();
            let b  = valueConstructor(&E::B(1)).unwrap();
            let c  = valueConstructor(&E::C).unwrap();
            assert_eq!(a1, a2);
            assert_ne!(a1, b);
            assert_ne!(a1, c);
            assert_ne!(b, c);
        }

        #[test]
        fn test_clock() {
            let t1 = clock();
            let t2 = clock();
            assert!(t1 >= OrderedFloat(0.0));
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
            assert!(setStackOverflowSignal(true));
            assert!(!setStackOverflowSignal(false));
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
                lastModification: OrderedFloat(1234567890.0),
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
            assert_eq!(arrayGetNoBoundsChecking(arr.clone(), 1), 10);
            assert_eq!(arrayGetNoBoundsChecking(arr.clone(), 2), 20);
            assert_eq!(arrayGetNoBoundsChecking(arr, 3), 30);
        }

        #[test]
        fn test_array_update_no_bounds_checking() {
            let arr = arrayFromVec(vec![1, 2, 3]);
            arrayUpdateNoBoundsChecking(arr.clone(), 2, 99);
            assert_eq!(*arr.borrow(), vec![1, 99, 3]);
        }

        #[test]
        fn test_array_create_no_init() {
            let arr: Array<i32> = arrayCreateNoInit(5);
            assert_eq!(arr.borrow().len(), 5);
        }

        #[test]
        fn test_string_get_no_bounds_checking() {
            let s = arcstr::literal!("hello");
            assert_eq!(stringGetNoBoundsChecking(s.clone(), 1), b'h' as i32);
            assert_eq!(stringGetNoBoundsChecking(s, 5), b'o' as i32);
        }

        #[test]
        fn test_list_reverse_in_place() {
            use super::{cons, nil};
            let l = cons(1, cons(2, cons(3, nil())));
            let r = listReverseInPlace(l);
            assert_eq!((&*r).into_iter().cloned().collect::<Vec<_>>(), vec![3, 2, 1]);
            // Empty and singleton edge cases.
            assert_eq!((&*listReverseInPlace(nil::<i32>())).into_iter().count(), 0);
            let single = listReverseInPlace(cons(42, nil()));
            assert_eq!((&*single).into_iter().cloned().collect::<Vec<_>>(), vec![42]);
        }

        #[test]
        fn test_list_append_destroy() {
            use super::{cons, nil};
            let collect = |l: &super::List<i32>| (&*l).into_iter().cloned().collect::<Vec<_>>();

            // Normal append: first ++ second.
            let first = cons(1, cons(2, cons(3, nil())));
            let second = cons(4, cons(5, nil()));
            let r = listAppendDestroy(first, second);
            assert_eq!(collect(&r), vec![1, 2, 3, 4, 5]);

            // Empty first → result is second (no cell to repoint).
            let r = listAppendDestroy(nil::<i32>(), cons(7, cons(8, nil())));
            assert_eq!(collect(&r), vec![7, 8]);

            // Empty second → first unchanged.
            let r = listAppendDestroy(cons(1, cons(2, nil())), nil::<i32>());
            assert_eq!(collect(&r), vec![1, 2]);

            // Both empty.
            assert_eq!((&*listAppendDestroy(nil::<i32>(), nil::<i32>())).into_iter().count(), 0);

            // Singleton first.
            let r = listAppendDestroy(cons(1, nil()), cons(2, cons(3, nil())));
            assert_eq!(collect(&r), vec![1, 2, 3]);

            // Destructive / zero-alloc: the splice mutates the last cell of
            // `first` in place, so a clone of `first`'s head taken *before* the
            // call observes the appended tail afterwards (the cells are shared,
            // not copied).
            let first = cons(1, cons(2, nil()));
            let alias = first.clone();
            let r = listAppendDestroy(first, cons(3, cons(4, nil())));
            assert_eq!(collect(&r), vec![1, 2, 3, 4]);
            assert_eq!(collect(&alias), vec![1, 2, 3, 4]);
        }
    }
}
