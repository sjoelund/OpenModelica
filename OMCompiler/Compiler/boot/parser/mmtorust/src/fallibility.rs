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

use mmwinnow::Absyn;

use crate::external_c_calls::{self, Fallibility};
use crate::hierarchy::{InstanceHierarchy, NameNode, NodeKind};
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
}

impl Walk {
    fn scan_class(c: &MM::Class) -> Self {
        let mut w = Walk::default();
        let algorithms: &[Absyn::AlgorithmItem] = match &c.body {
            MM::ClassDef::Parts { algorithms, external, .. } => {
                if let Some(ext) = external {
                    w.external = Some(external_symbol_name(&ext.decl, &c.name));
                }
                algorithms
            }
            MM::ClassDef::ClassExtends { algorithms, .. } => algorithms,
            _ => return w,
        };
        for it in algorithms {
            w.scan_algorithm_item(it);
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
                self.scan_function_args(functionArgs);
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
                self.scan_function_args(functionArgs);
            }
            PARTEVALFUNCTION { function_, functionArgs } => {
                // Partial application produces a function value rather than
                // calling the function. It does NOT make the surrounding
                // function fallible on its own — but the bound argument
                // expressions are evaluated eagerly and therefore still need
                // to be walked.
                let _ = function_;
                self.scan_function_args(functionArgs);
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
            MATCHEXP { inputExp, cases, .. } => {
                // Plain `match` and `matchcontinue` can both raise a failure
                // when no case matches; mark the surrounding function fallible.
                self.has_match = true;
                self.scan_exp(inputExp);
                for case in &**cases {
                    match case {
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
                    let Absyn::NamedArg::NAMEDARG { argValue, .. } = &**na;
                    self.scan_exp(argValue);
                }
            }
            Absyn::FunctionArgs::FOR_ITER_FARG { exp, iterators, .. } => {
                self.scan_exp(exp);
                for it in &**iterators {
                    let Absyn::ForIterator::ITERATOR { range, guardExp, .. } = it;
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
    let Absyn::ExternalDecl::EXTERNALDECL { funcName, .. } = decl;
    match funcName.as_ref() {
        Some(n) if !n.is_empty() => n.to_string(),
        _ => fallback_fn_name.to_owned(),
    }
}

/// Walk the hierarchy and collect every R_FUNCTION class together with its
/// fully-qualified MM name. Mirrors the convention used throughout codegen
/// (dot-separated, top-level package first).
fn collect_functions<'a>(
    nodes: &BTreeMap<String, NameNode<'a>>,
    prefix: &str,
    out: &mut Vec<(String, &'a MM::Class)>,
) {
    for (name, node) in nodes {
        let qname = if prefix.is_empty() { name.clone() } else { format!("{prefix}.{name}") };
        if let NodeKind::Class(c) = &node.kind {
            if matches!(c.restriction, Absyn::Restriction::R_FUNCTION { .. }) {
                out.push((qname.clone(), *c));
            }
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
    // `caller_qname` includes the function's own name; strip it to get the
    // enclosing package prefix that `resolve_call_node` expects.
    let pkg_prefix = caller_qname.rsplit_once('.').map(|(p, _)| p).unwrap_or("");
    resolve_call_node(raw, top_level, pkg_prefix).map(|(q, _)| q)
}

// ── Driver ───────────────────────────────────────────────────────────────────

/// Run the full analysis pass.  Visits every function class in `hier`, scans
/// its body for calls / external bindings / match expressions, and computes
/// the fixed point of "is fallible".  Panics if it encounters an unlisted
/// external "C" symbol — see [`crate::external_c_calls::lookup_or_panic`].
pub fn analyze(hier: &InstanceHierarchy<'_>) -> FallibilityInfo {
    let mut functions: Vec<(String, &MM::Class)> = Vec::new();
    collect_functions(&hier.top_level, "", &mut functions);

    // Per-function scan results, keyed by FQN. Storing the Walk separately
    // from the fallibility set keeps the propagation loop allocation-free.
    let mut walks: BTreeMap<String, Walk> = BTreeMap::new();
    let mut external_count = 0usize;
    for (qname, class) in &functions {
        let w = Walk::scan_class(class);
        if w.external.is_some() {
            external_count += 1;
        }
        walks.insert(qname.clone(), w);
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
            // Builtin first: a known builtin classification short-circuits
            // both the hierarchy lookup and the fixed-point edge — the
            // verdict is decided right now.
            if let Some(b) = builtin_fallibility(raw) {
                if matches!(b, Fallibility::Fallible) {
                    fallible.insert(qname.clone());
                }
                continue;
            }
            // User-defined callee — record an edge so the fixed point can
            // propagate fallibility transitively.
            if let Some(target) = resolve_called_qname(raw, qname, &hier.top_level) {
                if walks.contains_key(&target) {
                    set.insert(target);
                }
                // If `target` resolves to a non-function node (record/type
                // constructor, partial-application reference, etc.), we drop
                // the edge — constructors never fail in our lowering.
            }
            // Unresolved names: conservatively ignored at this stage. They
            // typically correspond to user-supplied callback parameters
            // (function-typed arguments) whose target is only known at the
            // call site. A precise treatment requires the typed IR — see
            // module-level docs.
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
