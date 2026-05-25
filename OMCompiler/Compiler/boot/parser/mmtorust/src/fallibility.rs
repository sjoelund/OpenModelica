//! Fallibility analysis phase.
//!
//! Runs after [`crate::hierarchy::resolve_pass`] has converged and after
//! [`crate::hierarchy::detect_recursive_types`] / [`detect_types_containing_mutable`].
//!
//! Goal: classify every MetaModelica function (and every MetaModelica builtin
//! and every `external "C"` binding referenced from the sources) as
//! [`Fallibility::Fallible`] or [`Fallibility::Infallible`].  The result drives
//! codegen decisions later:
//!
//!   * Fallible function ⇒ Rust lowering returns `anyhow::Result<T>`, every
//!     call site appends `?` (or the surrounding [`crate::codegen::QMode`]
//!     equivalent).
//!   * Infallible function ⇒ Rust lowering returns the raw `T`, call sites
//!     emit a bare call expression — and when the function is referenced
//!     through a function-pointer type whose signature expects `Result<T>`,
//!     codegen wraps the value with a `fnptr!(f)` adapter so the types match.
//!
//! ## Why a dedicated phase
//!
//! Codegen used to assume every call is fallible and unconditionally appended
//! `?`. That bloats both compile times and runtime cost (every call materialises
//! a `Result`).  Determining fallibility precisely requires a global view —
//! a function is fallible iff one of its calls is fallible — so the natural
//! formulation is a fixed-point over the call graph, computed once before
//! code generation begins.  This file is the analogue of
//! [`crate::hierarchy::detect_recursive_types`] for fallibility.
//!
//! ## Definition of "fallible"
//!
//! A MetaModelica function `f` is *fallible* iff its body can fail at run time
//! via a code path that escapes the function. Concretely, `f` is fallible if
//! any of the following holds:
//!
//!  * `f` is `external "C" foo(...)` and the external `foo` is classified as
//!    [`Fallibility::Fallible`] in [`crate::external_c_calls`].
//!  * The body of `f` contains a call to `fail()` outside a `try`/`failure`
//!    boundary.
//!  * The body of `f` contains an unguarded `match` expression (a plain
//!    `match` without `matchcontinue` semantics can fall through if no case
//!    matches, which raises a MetaModelica failure).  `matchcontinue` is
//!    treated identically here — even though it locally recovers from a
//!    failing arm, exhausting all arms still raises a failure.
//!  * The body of `f` calls some other function `g` (builtin, external, or
//!    user-defined) that is itself fallible, again outside a `try`/`failure`
//!    boundary.
//!
//! `try ... else ... end try` and `failure(...)` blocks catch failures from
//! their body, so a fallible operation inside one does *not* propagate
//! upwards.  This is approximated, not implemented, in this first iteration —
//! see [`Walk::in_catch_depth`] — to keep the initial scaffold focused.
//!
//! ## What this iteration does NOT yet do
//!
//! * Cross-arm precision for `matchcontinue`: every match is conservatively
//!   marked fallible.
//! * Walking `for`/`while` loop bodies — these are walked uniformly with the
//!   surrounding scope (no special semantics needed for fallibility).
//! * Distinguishing `pure` external annotations.
//! * Identifying which uses of a function value are as a function pointer
//!   vs. as a direct call — that classification belongs to codegen, not to
//!   this purely-analytical phase.
//! * Refining the analysis from `Absyn::Exp`/`Absyn::Algorithm` to
//!   `typedexp::TypedExp`. The typed IR carries resolved call targets that
//!   would yield more precise results, but it is also expensive to compute
//!   (it currently runs once per function inside codegen). Re-using the typed
//!   IR here would duplicate that work; a future refactor should hoist the
//!   inference out of codegen and share the result.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use openmodelica_ast::Absyn;

use crate::external_c_calls::{self, Fallibility};
use crate::hierarchy::{InstanceHierarchy, NameNode, NodeKind, Ty};
use crate::typedexp::resolve_call_node;
use crate::MM;

// ── Builtin classification ───────────────────────────────────────────────────

/// Fallibility classification for MetaModelica built-in functions implemented
/// in the `metamodelica` crate (see `metamodelica/src/lib.rs`).
///
/// Returns `None` if `name` is not recognised as a builtin; the caller is then
/// expected to look the name up as a user-defined or external function.
///
/// The classification mirrors what each function *actually* does today, not
/// what its current Rust signature claims: today every metamodelica function
/// returns `Result<T>` (a historical accident), but most are provably
/// infallible.  The codegen layer will eventually drop the `Result` from
/// infallible callees once this analysis is wired in.
pub fn builtin_fallibility(name: &str) -> Option<Fallibility> {
    use Fallibility::*;
    Some(match name {
        // ── Boolean — pure value logic, never fails ──────────────────────────
        "boolAnd" | "boolOr" | "boolNot" | "boolEq" | "boolString" => Infallible,

        // ── Integer arithmetic ───────────────────────────────────────────────
        // intDiv / intMod can in principle trap on divide-by-zero, but the
        // current implementation matches Modelica semantics by relying on the
        // host CPU's trap (we do not produce a `Result`). They are marked
        // Infallible to match the Rust signature; we may revisit if we
        // introduce explicit divide-by-zero checks.
        "intAdd" | "intSub" | "intMul" | "intDiv" | "intMod"
        | "intMax" | "intMin" | "intAbs" | "intNeg" => Infallible,
        "intLt" | "intLe" | "intEq" | "intNe" | "intGe" | "intGt" => Infallible,
        "intBitNot" | "intBitAnd" | "intBitOr" | "intBitXor"
        | "intBitLShift" | "intBitRShift" => Infallible,
        "intReal" | "intString" => Infallible,

        // ── Real arithmetic ──────────────────────────────────────────────────
        "realAdd" | "realSub" | "realMul" | "realDiv" | "realMod" | "realPow"
        | "realMax" | "realMin" | "realAbs" | "realNeg"
        | "realAlmostEq" => Infallible,
        "realLt" | "realLe" | "realEq" | "realNe" | "realGe" | "realGt" => Infallible,
        "realInt" | "realString" => Infallible,

        // ── String ───────────────────────────────────────────────────────────
        "stringCharInt" => Fallible,       // bails on non-singleton input
        "intStringChar" => Infallible,
        "stringInt" => Fallible,           // parse error
        "stringReal" => Fallible,          // parse error
        "stringListStringChar" => Infallible,
        "stringAppendList" | "stringDelimitList" => Infallible,
        "stringLength" | "stringEmpty" => Infallible,
        // `metamodelica::uriToFilename`: panics on malformed/unknown URIs
        // (matching the C `MMC_THROW`), but its Rust return type is plain
        // `ArcStr`, not `Result<ArcStr>` — so the caller must not emit `?`.
        "uriToFilename" => Infallible,
        "stringGet" => Fallible,           // index OOB
        "stringGetStringChar" => Fallible, // index OOB
        "stringUpdateStringChar" => Fallible, // bails on empty / OOB
        "stringAppend" => Infallible,
        "stringEq" | "stringEqual" | "stringCompare" => Infallible,
        "stringHash" | "stringHashDjb2" | "stringHashDjb2Continue"
        | "stringHashDjb2Mod" | "stringHashSdbm" => Infallible,
        "substring" => Fallible,           // bails on bogus range
        "listStringCharString" | "stringCharListString" => Infallible,

        // ── List ─────────────────────────────────────────────────────────────
        // listHead / listRest fail on Nil; the .get/.delete methods on Arc<List>
        // are bounds-checked. Plain `listAppend` / `listMember` / `listLength`
        // are total over `List<T>`.
        "listAppend" | "listMember" | "listLength" => Infallible,
        "listHead" | "listRest" => Fallible,

        // ── Array ────────────────────────────────────────────────────────────
        // arrayLength / arrayEmpty / arrayList / listArray / arrayCopy /
        // arrayAppend are total. arrayGet / arrayUpdate bounds-check.
        "arrayLength" | "arrayEmpty" | "arrayList" | "listArray"
        | "arrayCopy" | "arrayAppend" | "arrayCreate" => Infallible,
        "arrayGet" | "arrayUpdate" => Fallible,

        // ── Generic value / Option / misc ────────────────────────────────────
        "anyString" | "tick" | "clock"
        | "valueEq" | "valueCompare" | "referenceEq"
        | "referencePointerString" | "referenceDebugString"
        | "valueConstructor"
        | "isNone" | "isSome"
        | "setStackOverflowSignal" | "isPresent" => Infallible,

        // ── Explicit failure ─────────────────────────────────────────────────
        "fail" => Fallible,

        // ── MetaModelica::Dangerous — bounds-checked variants drop the check ─
        // The "no bounds checking" variants are infallible by construction.
        // listSetRest / listSetFirst bail on Nil — fallible.
        "arrayGetNoBoundsChecking"
        | "arrayUpdateNoBoundsChecking"
        | "arrayClearIndex"
        | "arrayCreateNoInit"
        | "stringGetNoBoundsChecking"
        | "listReverseInPlace" => Infallible,
        "listSetRest" | "listSetFirst" => Fallible,

        // ── Modelica language built-ins ──────────────────────────────────────
        // Declared as `external "C" name(...)` in
        // `OMCompiler/Compiler/FrontEnd/ModelicaBuiltin.mo` (and friends), but
        // they are NOT calls into the OpenModelica C runtime — the compiler
        // implements them directly (math intrinsics, array constructors,
        // signal operators). Classifying them here short-circuits the
        // [`crate::external_c_calls`] lookup so they don't need an entry in
        // that runtime-symbol registry.

        // Pure mathematical functions — total over the input domain. Some
        // (sqrt for negative input, log for non-positive) yield NaN/-inf at
        // runtime rather than raising, so they remain infallible.
        "sin" | "cos" | "tan"
        | "sinh" | "cosh" | "tanh"
        | "asin" | "acos" | "atan" | "atan2"
        | "exp" | "log" | "log10"
        | "sqrt" | "ceil" | "floor"
        | "sign" | "integer"
        | "abs" | "mod" | "div" | "rem" => Infallible,

        // Array constructors / reshape / projections.
        "ones" | "zeros" | "fill" | "identity" | "diagonal"
        | "vector" | "matrix" | "scalar" | "array"
        | "transpose" | "symmetric" | "skew"
        | "cross" | "outerProduct" | "linspace" => Infallible,

        // Reductions over arrays.
        "sum" | "product" | "min" | "max" => Infallible,

        // Continuous- / discrete-signal operators. Semantically these read
        // from solver state; they cannot fail at the language level.
        "pre" | "previous" | "der" | "edge" | "change"
        | "sample" | "hold" | "noEvent" | "smooth"
        | "semiLinear" | "reinit" | "delay"
        | "initial" | "terminal" => Infallible,

        // Synchronous (clocked) operators.
        "subSample" | "superSample" | "shiftSample" | "backSample"
        | "noClock" | "transition" | "ticksInState" | "timeInState"
        | "inStream" | "actualStream" | "getInstanceName"
        | "activeState" | "initialState" => Infallible,

        // Array shape / introspection.
        "size" | "ndims" => Infallible,

        // `cat(dim, A1, A2, ...)` concatenates arrays along dimension `dim`.
        // `classDirectory()` returns the source-file directory at the call
        // site — purely a compile-time query lowered to a constant.
        "cat" | "classDirectory" => Infallible,

        // Miscellaneous: connector cardinality, homotopy continuation,
        // distributed-parameter PDE primitive, pure-function marker.
        "cardinality" | "homotopy" | "spatialDistribution"
        | "promote" | "pure" => Infallible,

        // ── Failure-raising Modelica builtins ────────────────────────────────
        // `assert` throws when its condition is false; `terminate` ends the
        // simulation. Both propagate failure to the surrounding function.
        "assert" | "terminate" => Fallible,

        // ── I/O — `print` writes to stdout and never fails at this level. ──
        "print" => Infallible,

        _ => return None,
    })
}

// ── Public result type ───────────────────────────────────────────────────────

/// Output of [`analyze`]. Owned, cheap to clone — sets are typically small
/// compared to the size of the hierarchy.
#[derive(Debug, Default, Clone)]
pub struct FallibilityInfo {
    /// Fully-qualified MetaModelica names of every user-defined function
    /// classified as [`Fallibility::Fallible`].  Functions absent from this
    /// set are infallible.
    pub fallible_functions: BTreeSet<String>,
    /// Total number of user-defined function classes inspected.
    pub total_functions: usize,
    /// Number of distinct `external "C"` declarations encountered.
    pub external_functions: usize,
}

// ── Walk state ───────────────────────────────────────────────────────────────

/// Per-function call/feature accumulator. Built lazily by [`Walk::scan_class`]
/// and consumed by the fixed-point loop below.
#[derive(Debug, Default)]
struct Walk {
    /// `external "C"` binding for this function, if any. The first element is
    /// the C symbol name (with the MM-level name as a fallback when funcName
    /// is omitted, per Modelica external-function defaults).
    external: Option<String>,
    /// Names of all callees observed in the body — at this stage they are raw
    /// MM names from the source (e.g. "List.map", "foo", "intAdd"). They are
    /// resolved against the hierarchy in [`resolve_called_qname`].
    calls: BTreeSet<String>,
    /// True if the body contains a `match`/`matchcontinue` expression — a
    /// fail-on-no-match is observable to callers.
    has_match: bool,
    /// True if the body contains an explicit `fail()` call outside a catch
    /// boundary.  (Catch boundaries are not yet tracked; see module docs.)
    has_fail: bool,
    /// Function-level variable names (inputs/outputs/protected component
    /// declarations) that are visible as bindings inside any match in this
    /// function. Used by [`match_is_exhaustive`] / [`absyn_pat_is_irrefutable`]
    /// to decide whether a bare identifier in pattern position refers to a
    /// declared local (irrefutable variable binding) or a constructor
    /// (refutable). This is the scope-aware replacement for the historical
    /// case-sensitivity heuristic.
    outer_scope: BTreeSet<String>,
}

impl Walk {
    fn scan_class(c: &MM::Class) -> Self {
        let mut w = Walk::default();
        let (algorithms, members) = match &c.body {
            MM::ClassDef::Parts { algorithms, external, members, .. } => {
                if let Some(ext) = external {
                    w.external = Some(external_symbol_name(&ext.decl, &c.name));
                }
                (algorithms, members)
            }
            MM::ClassDef::ClassExtends { algorithms, members, .. } => (algorithms, members),
            _ => return w,
        };
        // Function-level variable declarations contribute to the binding
        // scope visible inside any nested match expression. Collect their
        // names up-front; per-match additions (matchExp.localDecls and
        // each case's localDecls) are layered on top inside
        // `match_is_exhaustive`.
        for m in members {
            if let MM::ClassMember::Component(cm) = m {
                w.outer_scope.insert(cm.name.clone());
            }
        }
        for it in algorithms {
            w.scan_algorithm_item(&**it);
        }
        w
    }

    fn scan_algorithm_item(&mut self, it: &Absyn::AlgorithmItem) {
        let alg = match it {
            Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_, .. } => &**algorithm_,
            Absyn::AlgorithmItem::ALGORITHMITEMCOMMENT { .. } => return,
        };
        match alg {
            Absyn::Algorithm::ALG_ASSIGN { assignComponent, value } => {
                // MetaModelica's `:=` is a *pattern* assignment: if the LHS is
                // anything other than a plain variable reference (or a tuple
                // of plain variable references), the match can fail at runtime
                // and the surrounding function therefore fallible. Codegen
                // lowers these to `let PAT = RHS else { bail!("pattern
                // mismatch") };`, which only typechecks when the function
                // returns `Result`. Examples:
                //   `Cons(h, t) := xs;`        — list cons pattern
                //   `SOME(x) := opt;`          — uniontype variant pattern
                //   `(a, SOME(b)) := pair;`    — tuple containing a refutable
                //                                sub-pattern
                if exp_is_refutable_lhs(assignComponent) {
                    self.has_fail = true;
                }
                self.scan_exp(assignComponent);
                self.scan_exp(value);
            }
            Absyn::Algorithm::ALG_IF { ifExp, trueBranch, elseIfAlgorithmBranch, elseBranch } => {
                self.scan_exp(ifExp);
                for it in &**trueBranch { self.scan_algorithm_item(it); }
                for (cond, branch) in &**elseIfAlgorithmBranch {
                    self.scan_exp(cond);
                    for it in &**branch { self.scan_algorithm_item(it); }
                }
                for it in &**elseBranch { self.scan_algorithm_item(it); }
            }
            Absyn::Algorithm::ALG_FOR { forBody, .. }
            | Absyn::Algorithm::ALG_PARFOR { parforBody: forBody, .. } => {
                for it in &**forBody { self.scan_algorithm_item(it); }
            }
            Absyn::Algorithm::ALG_WHILE { boolExpr, whileBody } => {
                self.scan_exp(boolExpr);
                for it in &**whileBody { self.scan_algorithm_item(it); }
            }
            Absyn::Algorithm::ALG_WHEN_A { boolExpr, whenBody, elseWhenAlgorithmBranch } => {
                self.scan_exp(boolExpr);
                for it in &**whenBody { self.scan_algorithm_item(it); }
                for (e, branch) in &**elseWhenAlgorithmBranch {
                    self.scan_exp(e);
                    for it in &**branch { self.scan_algorithm_item(it); }
                }
            }
            Absyn::Algorithm::ALG_NORETCALL { functionCall, functionArgs } => {
                self.record_call(&cref_to_dotted(functionCall));
                self.scan_function_args(&**functionArgs);
            }
            Absyn::Algorithm::ALG_FAILURE { equ: _ } => {
                // `failure(body)` *succeeds* iff `body` fails, which means it
                // *throws* whenever the body succeeds — so the construct
                // itself is unconditionally fallible from the enclosing
                // function's point of view. We do NOT need to inspect the
                // body: regardless of what it does, the failure clause can
                // raise the failure that escapes upward.
                self.has_fail = true;
            }
            Absyn::Algorithm::ALG_TRY { body: _, elseBody } => {
                // `try BODY else ELSE end try;` catches a failure raised by
                // BODY and runs ELSE instead. The only paths that can
                // propagate a failure *out* of the try clause are failures
                // inside ELSE (BODY's failures are caught and so do not
                // contribute to the enclosing function's fallibility).
                for it in &**elseBody { self.scan_algorithm_item(it); }
            }
            Absyn::Algorithm::ALG_RETURN
            | Absyn::Algorithm::ALG_BREAK
            | Absyn::Algorithm::ALG_CONTINUE => {}
        }
    }

    fn scan_exp(&mut self, e: &Absyn::Exp) {
        use Absyn::Exp::*;
        match e {
            INTEGER { .. } | REAL { .. } | STRING { .. } | BOOL { .. } | END | BREAK => {}
            CREF { .. } | CODE { .. } => {}
            BINARY { exp1, exp2, .. } | LBINARY { exp1, exp2, .. } | RELATION { exp1, exp2, .. } => {
                self.scan_exp(exp1); self.scan_exp(exp2);
            }
            UNARY { exp, .. } | LUNARY { exp, .. } => self.scan_exp(exp),
            IFEXP { ifExp, trueBranch, elseBranch, elseIfBranch } => {
                self.scan_exp(ifExp);
                self.scan_exp(trueBranch);
                self.scan_exp(elseBranch);
                for (c, t) in &**elseIfBranch { self.scan_exp(c); self.scan_exp(t); }
            }
            CALL { function_, functionArgs, .. } => {
                self.record_call(&cref_to_dotted(function_));
                self.scan_function_args(&**functionArgs);
            }
            PARTEVALFUNCTION { function_, functionArgs } => {
                // Partial application produces a function value rather than
                // calling the function. It does NOT make the surrounding
                // function fallible on its own — but the bound argument
                // expressions are evaluated eagerly and therefore still need
                // to be walked.
                let _ = function_;
                self.scan_function_args(&**functionArgs);
            }
            ARRAY { arrayExp } | LIST { exps: arrayExp } => {
                for e in &**arrayExp { self.scan_exp(e); }
            }
            MATRIX { matrix } => {
                for row in &**matrix {
                    for e in &**row { self.scan_exp(e); }
                }
            }
            RANGE { start, step, stop } => {
                self.scan_exp(start);
                if let Some(s) = step.as_deref() { self.scan_exp(s); }
                self.scan_exp(stop);
            }
            TUPLE { expressions } => {
                for e in &**expressions { self.scan_exp(e); }
            }
            AS { exp, .. } => self.scan_exp(exp),
            CONS { head, rest } => { self.scan_exp(head); self.scan_exp(rest); }
            MATCHEXP { matchTy, inputExp, localDecls, cases, .. } => {
                // A `match` raises a failure only when no arm matches the
                // scrutinee. If the patterns exhaustively cover every value
                // of the scrutinee's type, the match cannot fail, so the
                // surrounding function stays infallible. `matchcontinue` is
                // never considered exhaustive — any arm body may explicitly
                // `fail()` and fall through, exhausting all arms even with
                // full pattern coverage. See codegen `cases_exhaustive` for
                // the typed-IR counterpart; the two must agree.
                if !matches!(matchTy, Absyn::MatchType::MATCH)
                    || !match_is_exhaustive(&**cases, &**localDecls, &self.outer_scope)
                {
                    self.has_match = true;
                }
                self.scan_exp(inputExp);
                for case in &**cases {
                    match &**case {
                        Absyn::Case::CASE { pattern, patternGuard, classPart, result, .. } => {
                            self.scan_exp(pattern);
                            if let Some(g) = patternGuard.as_deref() { self.scan_exp(g); }
                            self.scan_class_part(classPart);
                            self.scan_exp(result);
                        }
                        Absyn::Case::ELSE { classPart, result, .. } => {
                            self.scan_class_part(classPart);
                            self.scan_exp(result);
                        }
                    }
                }
            }
            DOT { exp, index } => { self.scan_exp(exp); self.scan_exp(index); }
            EXPRESSIONCOMMENT { exp, .. } => self.scan_exp(exp),
            SUBSCRIPTED_EXP { exp, .. } => self.scan_exp(exp),
        }
    }

    fn scan_class_part(&mut self, part: &Absyn::ClassPart) {
        if let Absyn::ClassPart::ALGORITHMS { contents } = part {
            for it in &**contents { self.scan_algorithm_item(it); }
        }
        // EQUATIONS / EXTERNAL / etc. are not introduced inside match-case
        // class parts by the parser we use; if a future grammar revision
        // changes that, this match needs to grow.
    }

    fn scan_function_args(&mut self, fa: &Absyn::FunctionArgs) {
        match fa {
            Absyn::FunctionArgs::FUNCTIONARGS { args, argNames } => {
                for e in &**args { self.scan_exp(e); }
                for na in &**argNames {
                    let Absyn::NamedArg { argValue, .. } = &**na;
                    self.scan_exp(argValue);
                }
            }
            Absyn::FunctionArgs::FOR_ITER_FARG { exp, iterators, .. } => {
                self.scan_exp(exp);
                for it in &**iterators {
                    let Absyn::ForIterator { range, guardExp, .. } = &**it;
                    if let Some(r) = range.as_deref() { self.scan_exp(r); }
                    if let Some(g) = guardExp.as_deref() { self.scan_exp(g); }
                }
            }
        }
    }

    fn record_call(&mut self, name: &str) {
        if name == "fail" {
            self.has_fail = true;
        }
        self.calls.insert(name.to_owned());
    }
}

/// True when an expression used on the LHS of a MetaModelica `:=` assignment
/// produces a *refutable* pattern — one whose match can fail at runtime, in
/// which case codegen emits `bail!("pattern mismatch")` to surface the failure
/// to the caller, making the surrounding function fallible.
///
/// Plain variables and tuples-of-plain-variables are irrefutable; anything
/// involving a constructor, cons-cell, literal, range, or destructuring
/// expression is refutable. Wildcards (`_`) are irrefutable but appear in
/// pattern position only inside a containing tuple.
///
/// Conservative: when in doubt, classify as refutable. A spurious "fallible"
/// classification just keeps a `Result<>` return where it wasn't needed, while
/// a spurious "infallible" classification produces uncompilable code.
fn exp_is_refutable_lhs(e: &Absyn::Exp) -> bool {
    use Absyn::Exp::*;
    match e {
        // A plain identifier on the LHS is an ordinary assignment.
        CREF { .. } => false,
        // `(a, b, c) := rhs` — only irrefutable if every component is itself
        // irrefutable on the LHS.
        TUPLE { expressions } => (&**expressions).into_iter().any(|e| exp_is_refutable_lhs(e)),
        // Every other Exp shape that can syntactically appear on the LHS of
        // `:=` denotes a refutable pattern match: constructor applications
        // (CALL), cons-cells (CONS), literal lists/arrays, as-patterns,
        // ranges, and even bare literals.
        _ => true,
    }
}

/// Collect the bare component names declared in a `localDecls` block of a
/// `match` expression or a `case`. Each declaration takes the shape
/// `ELEMENTITEM { element: ELEMENT { specification: COMPONENTS { components, .. } } }`
/// where each component is a `COMPONENTITEM { component: COMPONENT { name } }`.
///
/// Lexer comment / TEXT / DEFINEUNIT items are silently skipped — they
/// don't introduce variable bindings.
fn collect_local_decl_names(
    decls: &metamodelica::List<std::sync::Arc<Absyn::ElementItem>>,
    out: &mut BTreeSet<String>,
) {
    for item in decls {
        let Absyn::ElementItem::ELEMENTITEM { element } = item.as_ref() else { continue };
        let Absyn::Element::ELEMENT { specification, .. } = &**element else { continue };
        let Absyn::ElementSpec::COMPONENTS { components, .. } = &**specification else { continue };
        for ci in &**components {
            let Absyn::ComponentItem { component, .. } = ci.as_ref();
            let Absyn::Component { name, .. } = component;
            out.insert(name.to_string());
        }
    }
}

// ── Exhaustiveness on Absyn patterns ─────────────────────────────────────────
//
// This is the Absyn-IR counterpart to codegen's `cases_exhaustive` /
// `pats_cover_ty`. The two analyses run on different IRs (Absyn here,
// typedexp::TypedPat there) but MUST classify the same set of matches as
// exhaustive — otherwise the fallibility verdict for a function disagrees
// with whether codegen emits a `_ => bail!(...)` fallback, producing
// uncompilable lowered code.
//
// Conservative: we underapproximate exhaustiveness. A `false` here just
// keeps the surrounding function flagged fallible (the historical default);
// a spurious `true` would let codegen elide a needed fallback and break the
// build. Type info is not available at this phase, so we only recognise
// the type-independent shapes whose pattern coverage is decidable purely
// from the constructor names involved (List, Option, Bool).

/// Is an Absyn-level pattern *irrefutable* — i.e. does it match every
/// possible value of whichever type the scrutinee turns out to have?
///
/// MetaModelica resolves bare identifiers in pattern position to either
/// "fresh variable binding" or "unit constructor reference" depending on
/// whether the name is declared as a local in the enclosing scope (match-
/// level or case-level `localDecls`, plus the surrounding function's
/// inputs/outputs/protected variables). `binding_names` carries that set
/// of names, gathered upstream by [`collect_match_binding_names`]. An
/// identifier in the set is treated as a variable binding (irrefutable);
/// any other identifier might be a constructor and is conservatively
/// classified refutable.
///
/// This is the sound replacement for the historical "first letter
/// uppercase ⇒ constructor" heuristic — we now consult the actual
/// declared scope.
fn absyn_pat_is_irrefutable(e: &Absyn::Exp, binding_names: &BTreeSet<String>) -> bool {
    use Absyn::Exp::*;
    match e {
        CREF { componentRef } => match componentRef.as_ref() {
            Absyn::ComponentRef::WILD | Absyn::ComponentRef::ALLWILD => true,
            Absyn::ComponentRef::CREF_IDENT { name, subscripts } if subscripts.is_empty() => {
                &**name == "_" || binding_names.contains(&**name as &str)
            }
            _ => false,
        }
        AS { exp, .. } => absyn_pat_is_irrefutable(exp, binding_names),
        TUPLE { expressions } => (&**expressions).into_iter().all(|e| absyn_pat_is_irrefutable(e, binding_names)),
        _ => false,
    }
}

/// A `SOME(<pat>)` pattern whose inner sub-pattern is itself irrefutable
/// covers every `SOME(_)` value. We recognise the Absyn-level shape: a
/// `CALL` whose callee dottifies to `SOME` with exactly one positional
/// argument that is irrefutable. (Named-argument forms or multi-arg
/// shapes are rejected as not-a-canonical-SOME-pattern.)
fn absyn_pat_is_full_some(e: &Absyn::Exp, binding_names: &BTreeSet<String>) -> bool {
    if let Absyn::Exp::CALL { function_, functionArgs, .. } = e {
        if cref_to_dotted(function_) != "SOME" { return false; }
        if let Absyn::FunctionArgs::FUNCTIONARGS { args, argNames } = &**functionArgs {
            let args_vec: Vec<&Absyn::Exp> = (&**args).into_iter().map(|a| a.as_ref()).collect();
            let names_empty = (&**argNames).into_iter().next().is_none();
            return names_empty
                && args_vec.len() == 1
                && absyn_pat_is_irrefutable(args_vec[0], binding_names);
        }
    }
    false
}

/// Does an Absyn case set exhaustively cover the scrutinee? Handles the
/// type-independent shapes:
///   * any case is `ELSE` or has an irrefutable pattern → exhaustive
///   * `{}` (Nil) + `_ :: _` with both subpatterns irrefutable → List
///   * `NONE()` + `SOME(_)` with irrefutable inner → Option
///   * boolean literals `true` and `false` → Bool
/// Cases with a guard never contribute coverage — a guard can fail.
fn match_is_exhaustive(
    cases: &metamodelica::List<Arc<Absyn::Case>>,
    match_local_decls: &metamodelica::List<std::sync::Arc<Absyn::ElementItem>>,
    outer_scope: &BTreeSet<String>,
) -> bool {
    // The full set of names in scope as variable bindings for any pattern
    // in this match: outer scope (function inputs/outputs/protected) ∕
    // match-level localDecls ∕ per-case localDecls.  Built once for the
    // match, augmented per-case below.
    let mut match_scope: BTreeSet<String> = outer_scope.clone();
    collect_local_decl_names(match_local_decls, &mut match_scope);

    // ELSE / leading irrefutable case → exhaustive regardless of type.
    for case in cases {
        match &**case {
            Absyn::Case::ELSE { .. } => return true,
            Absyn::Case::CASE { pattern, patternGuard, localDecls, .. } => {
                if patternGuard.is_none() {
                    let mut scope = match_scope.clone();
                    collect_local_decl_names(&**localDecls, &mut scope);
                    if absyn_pat_is_irrefutable(pattern.as_ref(), &scope) {
                        return true;
                    }
                }
            }
        }
    }

    // Collect un-guarded (pattern, per-case scope) pairs for the structural
    // checks below. Each case's pattern is checked against the union of the
    // match scope and that case's own localDecls.
    let pats: Vec<(&Absyn::Exp, BTreeSet<String>)> = cases.into_iter().filter_map(|c| match &**c {
        Absyn::Case::CASE { pattern, patternGuard, localDecls, .. } if patternGuard.is_none() => {
            let mut scope = match_scope.clone();
            collect_local_decl_names(&**localDecls, &mut scope);
            Some((&**pattern, scope))
        }
        _ => None,
    }).collect();

    // List: Nil + fully-irrefutable Cons.
    //
    // The parser surface for the empty list literal `{}` is currently
    // `Absyn::Exp::ARRAY { arrayExp: [] }` (the MetaModelica `{...}`
    // syntax always produces an ARRAY node; the dedicated LIST variant is
    // emitted for the `list(...)` builtin or list-comprehension forms).
    // We accept either shape so a future parser change to emit LIST for
    // `{}` continues to be recognised. A non-empty `{l}` literal would
    // desugar to a Cons chain in pattern position, but the parser keeps
    // the literal form here — `{l}` is therefore ARRAY/LIST with a single
    // element and does NOT contribute to Cons coverage.
    let is_empty_literal = |p: &Absyn::Exp| matches!(p,
        Absyn::Exp::ARRAY { arrayExp } if arrayExp.is_empty()
    ) || matches!(p,
        Absyn::Exp::LIST { exps } if exps.is_empty()
    );
    let has_nil = pats.iter().any(|(p, _)| is_empty_literal(p));
    let has_full_cons = pats.iter().any(|(p, scope)| match p {
        Absyn::Exp::CONS { head, rest } =>
            absyn_pat_is_irrefutable(&**head, scope) && absyn_pat_is_irrefutable(&**rest, scope),
        _ => false,
    });
    if has_nil && has_full_cons { return true; }

    // Option: NONE() + SOME(irrefutable).
    let has_none = pats.iter().any(|(p, _)| matches!(
        p,
        Absyn::Exp::CALL { function_, .. } if cref_to_dotted(function_.as_ref()) == "NONE"
    ));
    let has_full_some = pats.iter().any(|(p, scope)| absyn_pat_is_full_some(p, scope));
    if has_none && has_full_some { return true; }

    // Bool: both literals present.
    let has_true = pats.iter().any(|(p, _)| matches!(p, Absyn::Exp::BOOL { value: true }));
    let has_false = pats.iter().any(|(p, _)| matches!(p, Absyn::Exp::BOOL { value: false }));
    if has_true && has_false { return true; }

    // TODO: uniontype / record exhaustiveness — requires looking up the
    // scrutinee's type to enumerate constructors, which needs the typed IR.
    // See the typedexp::TypedPat counterpart in codegen for the analogous
    // gap.
    false
}

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Return the dotted MM-side name for a `ComponentRef` (e.g. `List.map`).
/// Mirrors `typedexp::cref_to_dotted` but kept private here so the analysis
/// is independent of the typed-IR module's surface.
fn cref_to_dotted(cref: &Absyn::ComponentRef) -> String {
    match cref {
        Absyn::ComponentRef::CREF_IDENT { name, .. } => name.to_string(),
        Absyn::ComponentRef::CREF_QUAL { name, componentRef, .. } => {
            format!("{name}.{}", cref_to_dotted(componentRef))
        }
        Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef } => {
            cref_to_dotted(componentRef)
        }
        Absyn::ComponentRef::WILD => "_".to_owned(),
        Absyn::ComponentRef::ALLWILD => "__".to_owned(),
    }
}

/// Pick the external C symbol used by an `external "C" ...` declaration.
///
/// Modelica allows omitting the explicit funcName, in which case the enclosing
/// MM function's name is the C symbol — see the Modelica spec, §12.9.1.3.
fn external_symbol_name(decl: &Absyn::ExternalDecl, fallback_fn_name: &str) -> String {
    let Absyn::ExternalDecl { funcName, .. } = decl;
    match funcName.as_ref() {
        Some(n) if !n.is_empty() => n.to_string(),
        _ => fallback_fn_name.to_owned(),
    }
}

/// Walk the hierarchy and collect every R_FUNCTION class together with its
/// fully-qualified MM name. Mirrors the convention used throughout codegen
/// (dot-separated, top-level package first).
///
/// Also records the resolved [`Ty::FunctionAlias`] base name (if any), so the
/// fallibility analysis can propagate fallibility through `function Foo = Bar`
/// aliases without re-resolving them later.
fn collect_functions<'a>(
    nodes: &BTreeMap<String, NameNode<'a>>,
    prefix: &str,
    out: &mut Vec<(String, &'a MM::Class, Option<String>)>,
) {
    for (name, node) in nodes {
        let qname = if prefix.is_empty() { name.clone() } else { format!("{prefix}.{name}") };
        if let NodeKind::Class(c) = &node.kind
            && matches!(c.restriction, Absyn::Restriction::R_FUNCTION { .. }) {
                let alias_base = match &node.ty {
                    Ty::FunctionAlias { base, .. } => Some(base.clone()),
                    _ => None,
                };
                out.push((qname.clone(), *c, alias_base));
            }
        collect_functions(&node.children, &qname, out);
    }
}

/// Resolve a raw callee name to its fully-qualified MM name relative to the
/// caller's enclosing package, using the same scoping rules codegen uses.
///
/// Returns `None` when the name doesn't resolve to anything in the hierarchy
/// (typical for builtins and external symbols, which are handled separately).
fn resolve_called_qname<'a>(
    raw: &str,
    caller_qname: &str,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> Option<String> {
    // Pass the *full* caller FQN as the scope prefix, not the enclosing
    // package. `resolve_call_node` walks the prefix outward one segment at
    // a time, so this lets it try `Mod.Caller.callee` first (catching
    // function-nested helper functions) before falling back to
    // `Mod.callee` and the bare top-level lookup. Stripping the function
    // name eagerly would skip the function-nested case and the
    // fallibility analysis would then see the call as unresolved.
    resolve_call_node(raw, top_level, caller_qname).map(|(q, _)| q)
}

// ── Driver ───────────────────────────────────────────────────────────────────

/// Run the full analysis pass.  Visits every function class in `hier`, scans
/// its body for calls / external bindings / match expressions, and computes
/// the fixed point of "is fallible".  Panics if it encounters an unlisted
/// external "C" symbol — see [`crate::external_c_calls::lookup_or_panic`].
pub fn analyze(hier: &InstanceHierarchy<'_>) -> FallibilityInfo {
    let mut functions: Vec<(String, &MM::Class, Option<String>)> = Vec::new();
    collect_functions(&hier.top_level, "", &mut functions);

    // Per-function scan results, keyed by FQN. Storing the Walk separately
    // from the fallibility set keeps the propagation loop allocation-free.
    let mut walks: BTreeMap<String, Walk> = BTreeMap::new();
    let mut external_count = 0usize;
    // Function-alias edges, keyed by alias FQN → unresolved base name (as
    // written in `function Foo = Bar(...)`). Resolved to FQN below alongside
    // the rest of the call edges so the alias inherits its target's
    // fallibility classification.
    let mut alias_bases: BTreeMap<String, String> = BTreeMap::new();
    for (qname, class, alias_base) in &functions {
        let w = Walk::scan_class(class);
        if w.external.is_some() {
            external_count += 1;
        }
        walks.insert(qname.clone(), w);
        if let Some(base) = alias_base {
            alias_bases.insert(qname.clone(), base.clone());
        }
    }

    let mut fallible: BTreeSet<String> = BTreeSet::new();

    // Seed: every function whose immediate features (external/fail/match)
    // make it locally fallible.
    //
    // For functions with an `external` clause, look up the classification in
    // priority order:
    //   1. The MM-side bare name in [`builtin_fallibility`] — this is where
    //      Modelica language built-ins (`sin`, `cos`, `assert`, the array
    //      constructors, the signal operators, …) live. They are declared
    //      with `external "C"` in `ModelicaBuiltin.mo` but the compiler
    //      implements them directly; they are NOT calls into the OpenModelica
    //      C runtime, so they must not consult [`external_c_calls`].
    //   2. The C symbol in [`external_c_calls`] — the strict registry of
    //      genuine `OMCompiler/Compiler/runtime/*.c` symbols. Panics on
    //      unlisted entries (unless `MMTORUST_LENIENT_EXTERNALS=1`).
    for (qname, w) in &walks {
        let local = if let Some(c_name) = &w.external {
            let simple = qname.rsplit_once('.').map(|(_, s)| s).unwrap_or(qname.as_str());
            let f = builtin_fallibility(simple)
                .unwrap_or_else(|| external_c_calls::lookup_or_panic(c_name, qname));
            matches!(f, Fallibility::Fallible)
        } else {
            w.has_fail || w.has_match
        };
        if local {
            fallible.insert(qname.clone());
        }
    }

    // Build a forward call graph: caller_fqn → set of callee_fqn that are
    // user-defined functions reachable in the hierarchy.  Calls to bare
    // names are also resolved against builtins (handled inline below).
    let mut callees: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for (qname, w) in &walks {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for raw in &w.calls {
            // Resolve user-defined callee first: a user function shadows any
            // same-named builtin (e.g. `exp` inside `Template.TplMain`
            // refers to the AST-printer, not the math `exp` builtin).
            // If the name resolves to a node in the hierarchy, record the
            // edge for fixed-point propagation. Otherwise fall through to
            // the builtin table.
            if let Some(target) = resolve_called_qname(raw, qname, &hier.top_level) {
                if walks.contains_key(&target) {
                    set.insert(target);
                    continue;
                }
                // Target resolves to a non-function node (record/type
                // constructor, partial-application reference, etc.) — drop
                // the edge, constructors never fail in our lowering.
                continue;
            }
            // Unresolved as a user function: consult the builtin table.
            if let Some(b) = builtin_fallibility(raw) {
                if matches!(b, Fallibility::Fallible) {
                    fallible.insert(qname.clone());
                }
                continue;
            }
            // Unresolved names: conservatively ignored at this stage. They
            // typically correspond to user-supplied callback parameters
            // (function-typed arguments) whose target is only known at the
            // call site. A precise treatment requires the typed IR — see
            // module-level docs.
        }
        // `function Foo = Bar(...)` aliases have no body of their own; their
        // fallibility comes from the base function. Add the resolved edge so
        // the fixed-point loop propagates it. Unresolved bases fall through
        // to the builtin table — pathStringNoQual → pathString, for example.
        if let Some(base) = alias_bases.get(qname) {
            if let Some(target) = resolve_called_qname(base, qname, &hier.top_level) {
                if walks.contains_key(&target) {
                    set.insert(target);
                }
            } else if let Some(b) = builtin_fallibility(base) {
                if matches!(b, Fallibility::Fallible) {
                    fallible.insert(qname.clone());
                }
            }
        }
        callees.insert(qname.clone(), set);
    }

    // Fixed point: a function becomes fallible as soon as it can reach a
    // fallible callee.  Naive O(n·m·#iters) saturation — sufficient at
    // current scale (a few thousand functions) and easy to verify.
    loop {
        let mut changed = false;
        for (qname, edges) in &callees {
            if fallible.contains(qname) { continue; }
            if edges.iter().any(|t| fallible.contains(t)) {
                fallible.insert(qname.clone());
                changed = true;
            }
        }
        if !changed { break; }
    }

    FallibilityInfo {
        fallible_functions: fallible,
        total_functions: functions.len(),
        external_functions: external_count,
    }
}
