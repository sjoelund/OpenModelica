#![allow(unused)]

use std::collections::HashMap;
use std::collections::BTreeMap;
use mmwinnow::Absyn;
use crate::MM;
use crate::hierarchy::{FunctionInput, NameNode, NodeKind, Ty, extract_default_exp, lookup_record_through_unions, collect_type_vars_in_ty, collect_type_vars_in_env};

// ── Literal values ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Lit {
    Int(i32),
    Real(String),
    Str(String),
    Bool(bool),
}

// ── Operator kinds ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOpKind {
    Add, Sub, Mul, Div, Pow,
    And, Or,
    Eq, NEq, Lt, LEq, Gt, GEq,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnOpKind { Neg, Not }

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MatchKind { Match, MatchContinue }

/// How multiple iterators in a reduction interact:
/// - `Combine`: cartesian product (the default; e.g. `f(e for i in xs, j in ys)`).
/// - `Thread`:  zip (introduced by the `threaded` keyword).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReductionIterKind { Combine, Thread }

/// One iterator in a reduction. `range` is the source collection, `guard` is an
/// optional Boolean filter expression evaluated per element.
#[derive(Debug, Clone)]
pub struct ReductionIter {
    pub name: String,
    pub range: TypedExp,
    pub guard: Option<TypedExp>,
    pub elem_ty: Ty,
}

// ── Typed expression IR ───────────────────────────────────────────────────────

/// One case in a match/matchcontinue expression.
#[derive(Debug, Clone)]
pub struct TypedCase {
    pub pattern: TypedPat,
    pub guard: Option<TypedExp>,
    /// Case-local declarations.  Each entry is `(name, type, default)` where
    /// `default` is the optional binding expression from the MetaModelica source
    /// (e.g. `list<list<T>> ol = {};`).  When present, codegen emits the local
    /// as `let mut <name>: <ty> = <default>;` so that the body may read it
    /// before any explicit assignment, matching MetaModelica semantics.
    pub locals: Vec<(String, Ty, Option<TypedExp>)>,
    pub stmts: Vec<TypedStmt>,
    pub result: TypedExp,
}

/// One segment of a structured component reference, carrying its subscripts.
/// e.g. `arr[1].field[2+i]` → `[Seg("arr",[1]), Seg("field",[2+i])]`
#[derive(Debug, Clone)]
pub struct CrefSegment {
    pub name: String,
    pub subscripts: Vec<TypedExp>,
}

#[derive(Debug, Clone)]
pub enum TypedExp {
    Lit(Lit),
    /// A variable reference or constant path.
    /// `name` is the dotted MM name (for lookup/compat).
    /// `segments` carries the structured parts with subscripts.
    Var { name: String, segments: Vec<CrefSegment>, ty: Ty },
    BinOp { op: BinOpKind, lhs: Box<TypedExp>, rhs: Box<TypedExp>, ty: Ty },
    UnOp { op: UnOpKind, operand: Box<TypedExp>, ty: Ty },
    /// A function call. `func` is the dotted MM name (e.g. "List.map", "SOME").
    Call { func: String, args: Vec<TypedExp>, named_args: Vec<(String, TypedExp)>, ty: Ty, sig_ty: Ty },
    /// A constructor/record literal. `name` is the dotted MM name.
    Constructor { name: String, args: Vec<TypedExp>, named_args: Vec<(String, TypedExp)>, ty: Ty, field_names: Vec<String> },
    /// Partial function application: `function f(arg1 = e1, arg2 = e2, ...)` —
    /// produces a callable value with the named/positional formals bound and the
    /// remaining formals still open. Lowers to a Rust closure that captures the
    /// bound expressions and forwards the unbound formals to `f`.
    ///
    /// `func` is the MM name of the underlying function. `args` are positional
    /// bindings (each binds the i-th formal); `named_args` are bindings keyed by
    /// formal name. `sig_ty` is the resolved `Ty::Function` of `func` (carrying
    /// formal names/types, needed by codegen to know which formals remain
    /// unbound). `ty` is the resulting function type — `sig_ty` with the bound
    /// formals removed from `inputs`.
    PartEval {
        func: String,
        args: Vec<TypedExp>,
        named_args: Vec<(String, TypedExp)>,
        sig_ty: Ty,
        ty: Ty,
    },
    If {
        cond: Box<TypedExp>,
        then_: Box<TypedExp>,
        elseif: Vec<(TypedExp, TypedExp)>,
        else_: Box<TypedExp>,
        ty: Ty,
    },
    Cons { head: Box<TypedExp>, tail: Box<TypedExp>, ty: Ty },
    Tuple(Vec<TypedExp>),
    /// An array/list literal. Empty array = empty list.
    Array { elems: Vec<TypedExp>, ty: Ty },
    Match { kind: MatchKind, input: Box<TypedExp>, cases: Vec<TypedCase>, ty: Ty },
    /// `start:stop` or `start:step:stop` — an arithmetic-progression iterator.
    Range { start: Box<TypedExp>, step: Option<Box<TypedExp>>, stop: Box<TypedExp>, elem_ty: Ty },
    /// A reduction expression `f(body for iter1 in r1, iter2 in r2, ...)` (or
    /// `threaded for ...` for zip semantics). The reduction is identified by
    /// `func` — either a builtin (`list`, `listReverse`, `sum`, `product`,
    /// `min`, `max`, `listAppend`) or a user-defined function whose signature
    /// must carry a `defaultValue` so the accumulator can be seeded.
    Reduction {
        func: String,
        body: Box<TypedExp>,
        iterators: Vec<ReductionIter>,
        iter_kind: ReductionIterKind,
        ty: Ty,
    },
    Todo(String),
}

impl TypedExp {
    pub fn ty(&self) -> Ty {
        match self {
            TypedExp::Lit(Lit::Int(_))  => Ty::I32,
            TypedExp::Lit(Lit::Real(_)) => Ty::F64,
            TypedExp::Lit(Lit::Str(_))  => Ty::Str,
            TypedExp::Lit(Lit::Bool(_)) => Ty::Bool,
            TypedExp::Var    { ty, .. }  => ty.clone(),
            TypedExp::BinOp  { ty, .. }  => ty.clone(),
            TypedExp::UnOp   { ty, .. }  => ty.clone(),
            TypedExp::Call   { ty, .. }  => ty.clone(),
            TypedExp::Constructor { ty, .. } => ty.clone(),
            TypedExp::If     { ty, .. }  => ty.clone(),
            TypedExp::Cons   { ty, .. }  => ty.clone(),
            TypedExp::Array  { ty, .. }  => ty.clone(),
            TypedExp::Match  { ty, .. }  => ty.clone(),
            TypedExp::Range  { elem_ty, .. } => Ty::Range(Box::new(elem_ty.clone())),
            TypedExp::Reduction { ty, .. } => ty.clone(),
            TypedExp::PartEval { ty, .. } => ty.clone(),
            TypedExp::Tuple(v) => Ty::Tuple(v.iter().map(|e| e.ty()).collect()),
            TypedExp::Todo(_)  => Ty::Unknown,
        }
    }
}

// ── Typed pattern IR ──────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum TypedPat {
    Wildcard,
    /// A binding variable introduced by this pattern.
    Var(String),
    Lit(Lit),
    EmptyList,
    Some_(Box<TypedPat>),
    None_,
    Cons { head: Box<TypedPat>, tail: Box<TypedPat> },
    Tuple(Vec<TypedPat>),
    /// A constructor/record pattern.
    /// `name` is the dotted MM name; `fields` are positional args; `named_fields` are named args.
    Constructor {
        name: String,
        fields: Vec<TypedPat>,
        named_fields: Vec<(String, TypedPat)>,
        ty: Ty,
    },
    /// `var as pat` — binds `var` to the whole value while also matching `pat`.
    As { var: String, pat: Box<TypedPat> },
    /// Array element access in pattern position (e.g. `arr[1]` on LHS of `:=`).
    Index { base: TypedExp, index: TypedExp },
    /// Field access on a local variable (e.g. `exarray.lastUsedIndex` where `exarray`
    /// is a variable). This must emit as `base.field` not as a let pattern.
    FieldAccess { base: Box<TypedPat>, field: String },
    Todo(String),
}

// ── Inference ─────────────────────────────────────────────────────────────────

/// Convert a ComponentRef to a dotted MetaModelica name (e.g. "List.map").
/// Deprecated: loses subscripts and structure. Use `extract_cref_segments` instead.
pub fn cref_to_dotted(cref: &Absyn::ComponentRef) -> String {
    let raw = match cref {
        Absyn::ComponentRef::CREF_IDENT { name, .. } => name.to_string(),
        Absyn::ComponentRef::CREF_QUAL { name, componentRef, .. } => {
            format!("{name}.{}", cref_to_dotted(componentRef))
        }
        Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef } => cref_to_dotted(componentRef),
        Absyn::ComponentRef::WILD | Absyn::ComponentRef::ALLWILD => "_".to_owned(),
    };
    match raw.as_str() {
        "MetaModelica.Dangerous.stringGetNoBoundsChecking" | "Dangerous.stringGetNoBoundsChecking" | "stringGetNoBoundsChecking" => "stringGet".to_owned(),
        "MetaModelica.Dangerous.arrayGetNoBoundsChecking" | "Dangerous.arrayGetNoBoundsChecking" | "arrayGetNoBoundsChecking" => "arrayGet".to_owned(),
        "MetaModelica.Dangerous.arrayUpdateNoBoundsChecking" | "Dangerous.arrayUpdateNoBoundsChecking" | "arrayUpdateNoBoundsChecking" => "arrayUpdate".to_owned(),
        // arrayCreateNoInit is kept distinct from arrayCreate: it lowers to
        // `metamodelica::Dangerous::arrayCreateNoInit(size)` which takes only the
        // size (the MetaModelica `dummy` second argument is a type witness only
        // and is dropped at codegen time).
        "MetaModelica.Dangerous.arrayCreateNoInit" | "Dangerous.arrayCreateNoInit" => "arrayCreateNoInit".to_owned(),
        "MetaModelica.Dangerous.listArrayLiteral" | "Dangerous.listArrayLiteral" | "listArrayLiteral" => "listArray".to_owned(),
        _ => raw,
    }
}

/// Extract structured segments (with subscripts) from a ComponentRef.
/// Returns (dotted_name, segments). The segments are in read-order:
/// `arr[1].field` → [("arr", [1]), ("field", [])]
fn extract_cref_segments<'a>(
    cref: &Absyn::ComponentRef,
    env: &HashMap<String, Ty>,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
    pkg_prefix: &str,
) -> (String, Vec<CrefSegment>) {
    // Collect segments in read-order (head -> tail).
    let mut segs: Vec<CrefSegment> = Vec::new();
    collect_cref_segments_rev(cref, env, top_level, pkg_prefix, &mut segs);

    let dotted: String = segs.iter().map(|s| s.name.clone()).collect::<Vec<_>>().join(".");
    (dotted, segs)
}

fn collect_cref_segments_rev<'a>(
    cref: &Absyn::ComponentRef,
    env: &HashMap<String, Ty>,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
    pkg_prefix: &str,
    acc: &mut Vec<CrefSegment>,
) {
    match cref {
        Absyn::ComponentRef::CREF_IDENT { name, subscripts } => {
            let subs: Vec<TypedExp> = (&**subscripts).into_iter()
                .filter_map(|s| {
                    if let Absyn::Subscript::SUBSCRIPT { subscript } = s.as_ref() {
                        Some(infer_exp(subscript, env, top_level, pkg_prefix, &[]))
                    } else {
                        None
                    }
                })
                .collect();
            acc.push(CrefSegment { name: name.to_string(), subscripts: subs });
        }
        Absyn::ComponentRef::CREF_QUAL { name, subscripts, componentRef } => {
            let subs: Vec<TypedExp> = (&**subscripts).into_iter()
                .filter_map(|s| {
                    if let Absyn::Subscript::SUBSCRIPT { subscript } = s.as_ref() {
                        Some(infer_exp(subscript, env, top_level, pkg_prefix, &[]))
                    } else {
                        None
                    }
                })
                .collect();
            acc.push(CrefSegment { name: name.to_string(), subscripts: subs });
            collect_cref_segments_rev(componentRef, env, top_level, pkg_prefix, acc);
        }
        Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef } => {
            collect_cref_segments_rev(componentRef, env, top_level, pkg_prefix, acc);
        }
        Absyn::ComponentRef::WILD | Absyn::ComponentRef::ALLWILD => {
            acc.push(CrefSegment { name: "_".to_owned(), subscripts: vec![] });
        }
    }
}

/// Convert an `Absyn::Path` to a dotted string (e.g. `"Pkg.Sub.Name"`).
fn path_to_dotted(path: &Absyn::Path) -> String {
    match path {
        Absyn::Path::IDENT { name } => name.to_string(),
        Absyn::Path::QUALIFIED { name, path } => format!("{name}.{}", path_to_dotted(path)),
        Absyn::Path::FULLYQUALIFIED { path } => path_to_dotted(path),
    }
}

/// Extract the target dotted path from any import (including unqualified/wildcard).
fn import_any_target_path(import: &Absyn::Import) -> Option<String> {
    match import {
        Absyn::Import::NAMED_IMPORT { path, .. }
        | Absyn::Import::QUAL_IMPORT { path }
        | Absyn::Import::UNQUAL_IMPORT { path } => {
            let d = path_to_dotted(path);
            if d.is_empty() { None } else { Some(d) }
        }
        _ => None,
    }
}

/// Extract the target dotted path from a named or qualified import statement.
/// Returns `None` for wildcard (`import Pkg.*`) and group imports.
fn import_target_path(import: &Absyn::Import) -> Option<String> {
    match import {
        Absyn::Import::NAMED_IMPORT { path, .. } | Absyn::Import::QUAL_IMPORT { path } => {
            let d = path_to_dotted(path);
            if d.is_empty() { None } else { Some(d) }
        }
        _ => None,
    }
}

/// Walk a dotted path through the hierarchy, transparently following import alias nodes.
///
/// For example, if `NFBuiltin` has `import LookupTree = NFLookupTree`, then
/// `walk_dotted_with_imports("NFBuiltin.LookupTree.Tree.EMPTY", top_level)` resolves
/// `LookupTree` → `NFLookupTree` and returns the node for `NFLookupTree.Tree.EMPTY`.
///
/// Also handles `Ty::AliasTo` type aliases (e.g. `type ParameterTree = NFCallParameterTree.Tree`).
///
/// Depth-limited to avoid infinite loops from mutually-recursive import aliases.
fn walk_dotted_with_imports<'a>(
    dotted: &str,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
    depth: u32,
) -> Option<(String, &'a NameNode<'a>)> {
    if depth > 8 {
        // Guard against pathological import alias cycles.
        return None;
    }

    // Fast path: direct lookup (also checks through intermediate uniontype nodes).
    if let Some(r) = lookup_record_through_unions(dotted, top_level)
        .or_else(|| lookup_node(dotted, top_level).map(|n| (dotted.to_owned(), n)))
    {
        return Some(r);
    }

    // Incremental walk: find the first prefix that exists in the hierarchy
    // and is an import or type-alias node, then substitute and retry.
    let parts: Vec<&str> = dotted.split('.').collect();
    for split in 1..parts.len() {
        let prefix = parts[..split].join(".");
        let Some(node) = lookup_node(&prefix, top_level) else { continue };

        let target: Option<String> = match &node.kind {
            NodeKind::Import(m) => import_target_path(&m.import),
            _ => None,
        }
        // Also follow type aliases recorded in the node's resolved type.
        .or_else(|| match &node.ty {
            Ty::AliasTo(t) => Some(t.clone()),
            Ty::RustEnum(t) | Ty::RustStruct(t) => Some(t.clone()),
            _ => None,
        });

        if let Some(target) = target {
            let rest = parts[split..].join(".");
            let resolved = if rest.is_empty() { target } else { format!("{target}.{rest}") };
            if let Some(r) = walk_dotted_with_imports(&resolved, top_level, depth + 1) {
                return Some(r);
            }
        }
    }
    None
}

/// Resolve a call-site function name to a `(canonical_dotted_name, node)` pair.
///
/// Resolution order:
/// 1. Direct top-level lookup, including through intermediate uniontype nodes.
/// 2. With `pkg_prefix` prepended (walking up the scope hierarchy from the most-specific
///    enclosing scope to the least-specific), to resolve names relative to the current package.
/// 3. At each candidate path, import-alias nodes (and `AliasTo` type aliases) are followed
///    transparently so that e.g. `LookupTree.Tree.EMPTY` inside `NFBuiltin` resolves to
///    `NFLookupTree.Tree.EMPTY` via the `import LookupTree = NFLookupTree;` declaration.
///
/// This function does NOT use heuristics (case/prefix); all decisions are based on the
/// hierarchy.
pub fn resolve_call_node<'a>(
    func: &str,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
    pkg_prefix: &str,
) -> Option<(String, &'a NameNode<'a>)> {
    // 1. Direct lookup (handles fully-qualified top-level names).
    if let Some(r) = walk_dotted_with_imports(func, top_level, 0) {
        return Some(r);
    }

    // 2. Qualify with each enclosing scope level (most-specific first).
    if !pkg_prefix.is_empty() {
        let mut parts: Vec<&str> = pkg_prefix.split('.').collect();
        loop {
            let prefixed = format!("{}.{func}", parts.join("."));
            if let Some(r) = walk_dotted_with_imports(&prefixed, top_level, 0) {
                return Some(r);
            }
            if parts.is_empty() {
                break;
            }
            parts.pop();
        }
    }

    // 3. For each scope level, scan all import children and try to resolve `func`
    //    against each import's target package. This handles:
    //    - bare names whose source type's package is only reachable via import alias
    //      (e.g. `MATCHING` in a scope with `import Matching = NBMatching;` — the
    //      record `MATCHING` lives in `NBMatching` and is used unqualified)
    //    - bare/dotted names reachable via wildcard imports
    //      (e.g. `Replaceable.NOT_REPLACEABLE` in a scope with `import NFPrefixes.*`)
    if !pkg_prefix.is_empty() {
        let mut parts: Vec<&str> = pkg_prefix.split('.').collect();
        loop {
            let scope_path = parts.join(".");
            if let Some(scope_node) = lookup_node(&scope_path, top_level) {
                for child in scope_node.children.values() {
                    if let NodeKind::Import(m) = &child.kind {
                        if let Some(target) = import_any_target_path(&m.import) {
                            let candidate = format!("{target}.{func}");
                            if let Some(r) = walk_dotted_with_imports(&candidate, top_level, 0) {
                                return Some(r);
                            }
                        }
                    }
                }
            }
            if parts.is_empty() {
                break;
            }
            parts.pop();
        }
    }

    None
}

fn lookup_ty_in_hierarchy<'a>(dotted: &str, top_level: &'a BTreeMap<String, NameNode<'a>>) -> Ty {
    let mut parts = dotted.split('.');
    let first = parts.next().unwrap_or("");
    let Some(mut node) = top_level.get(first) else { return Ty::Unknown };
    for part in parts {
        let Some(child) = node.children.get(part) else { return Ty::Unknown };
        node = child;
    }
    node.ty.clone()
}

/// Return the function signature for a MetaModelica built-in function used as a
/// first-class value (e.g. `valueEq` passed as a callback). Built-ins are not in
/// the hierarchy, so when a CREF resolves to `Ty::Unknown` we fall back here so
/// that codegen can treat the reference as a function pointer (no `.clone()`).
///
/// The signatures use `TypeVar("T")` as a stand-in for parameters whose actual
/// type is determined by the call site. The shape only needs to be `Ty::Function`
/// for codegen to skip the value clone; the inputs/output are informational —
/// EXCEPT for partial-application lowering (PARTEVALFUNCTION), which needs the
/// formal *names* to match what users write in `function f(name = value)`. So
/// where the MetaModelica builtin uses specific names (e.g. `realEq(x1, x2)`),
/// they are mirrored here.
pub fn builtin_function_ty(name: &str) -> Option<Ty> {
    let tv = |n: &str| Ty::TypeVar(n.to_owned());
    let inp = |name: &str, ty: Ty| FunctionInput { name: name.to_owned(), ty, default: None };
    let f = |inputs: Vec<FunctionInput>, output: Ty, type_vars: Vec<String>| -> Ty {
        Ty::Function { type_vars, inputs, output: Box::new(output), name: None }
    };
    match name {
        // Equality / comparison predicates: (T, T) -> Bool
        "valueEq" | "referenceEq" =>
            Some(f(vec![inp("a", tv("T")), inp("b", tv("T"))], Ty::Bool, vec!["T".to_owned()])),
        "intEq" | "intNe" | "intLt" | "intLe" | "intGt" | "intGe" =>
            Some(f(vec![inp("a", Ty::I32), inp("b", Ty::I32)], Ty::Bool, vec![])),
        "realEq" | "realLt" | "realLe" | "realGt" | "realGe" =>
            Some(f(vec![inp("x1", Ty::F64), inp("x2", Ty::F64)], Ty::Bool, vec![])),
        "stringEq" | "stringEqual" =>
            Some(f(vec![inp("a", Ty::Str), inp("b", Ty::Str)], Ty::Bool, vec![])),
        "boolEq" | "boolAnd" | "boolOr" =>
            Some(f(vec![inp("a", Ty::Bool), inp("b", Ty::Bool)], Ty::Bool, vec![])),
        "boolNot" =>
            Some(f(vec![inp("a", Ty::Bool)], Ty::Bool, vec![])),
        "isSome" | "isNone" =>
            Some(f(vec![inp("o", Ty::Option(Box::new(tv("T"))))], Ty::Bool, vec!["T".to_owned()])),
        "listEmpty" =>
            Some(f(vec![inp("l", Ty::List(Box::new(tv("T"))))], Ty::Bool, vec!["T".to_owned()])),
        "arrayEmpty" =>
            Some(f(vec![inp("a", Ty::Array(Box::new(tv("T"))))], Ty::Bool, vec!["T".to_owned()])),

        // Length-style: container -> Integer
        "listLength" =>
            Some(f(vec![inp("l", Ty::List(Box::new(tv("T"))))], Ty::I32, vec!["T".to_owned()])),
        "arrayLength" =>
            Some(f(vec![inp("a", Ty::Array(Box::new(tv("T"))))], Ty::I32, vec!["T".to_owned()])),
        "stringLength" =>
            Some(f(vec![inp("s", Ty::Str)], Ty::I32, vec![])),

        // Arithmetic: (T, T) -> T
        "intAdd" | "intSub" | "intMul" | "intDiv" | "intMod" | "intMax" | "intMin" =>
            Some(f(vec![inp("a", Ty::I32), inp("b", Ty::I32)], Ty::I32, vec![])),
        "realAdd" | "realSub" | "realMul" | "realDiv" | "realMax" | "realMin" =>
            Some(f(vec![inp("a", Ty::F64), inp("b", Ty::F64)], Ty::F64, vec![])),

        // Numeric coercions
        "intReal" =>
            Some(f(vec![inp("i", Ty::I32)], Ty::F64, vec![])),
        "realInt" =>
            Some(f(vec![inp("r", Ty::F64)], Ty::I32, vec![])),

        // String conversions/concat
        "intString" =>
            Some(f(vec![inp("i", Ty::I32)], Ty::Str, vec![])),
        "realString" =>
            Some(f(vec![inp("r", Ty::F64)], Ty::Str, vec![])),
        "boolString" =>
            Some(f(vec![inp("b", Ty::Bool)], Ty::Str, vec![])),
        "anyString" =>
            Some(f(vec![inp("v", tv("T"))], Ty::Str, vec!["T".to_owned()])),
        "stringAppend" =>
            Some(f(vec![inp("a", Ty::Str), inp("b", Ty::Str)], Ty::Str, vec![])),
        // String → number/boolean parsing
        "stringInt" =>
            Some(f(vec![inp("s", Ty::Str)], Ty::I32, vec![])),
        "stringReal" =>
            Some(f(vec![inp("s", Ty::Str)], Ty::F64, vec![])),
        "stringBool" =>
            Some(f(vec![inp("s", Ty::Str)], Ty::Bool, vec![])),

        _ => None,
    }
}

fn binop_ty(op: BinOpKind, lhs_ty: &Ty, rhs_ty: &Ty) -> Ty {
    match op {
        BinOpKind::Add | BinOpKind::Sub | BinOpKind::Mul | BinOpKind::Div => {
            match (lhs_ty, rhs_ty) {
                (Ty::F64, _) | (_, Ty::F64) => Ty::F64,
                (Ty::I32, _) | (_, Ty::I32) => Ty::I32,
                _ => lhs_ty.clone(),
            }
        }
        BinOpKind::Pow => Ty::F64,
        BinOpKind::And | BinOpKind::Or
        | BinOpKind::Eq | BinOpKind::NEq
        | BinOpKind::Lt | BinOpKind::LEq
        | BinOpKind::Gt | BinOpKind::GEq => Ty::Bool,
    }
}

fn call_ty(func: &str, args: &[TypedExp], top_level: &BTreeMap<String, NameNode<'_>>, pkg_prefix: &str) -> Ty {
    match func {
        "SOME" => Ty::Option(Box::new(args.first().map(|a| a.ty()).unwrap_or(Ty::Unknown))),
        "NONE" => Ty::Option(Box::new(Ty::Unknown)),
        "fail" => Ty::Unknown,
        "intAdd" | "intSub" | "intMul" | "intDiv" | "intMod" | "intAbs"
        | "intMax" | "intMin" | "intNeg" | "intBitAnd" | "intBitOr" | "intBitXor"
        | "intBitNot" | "intBitLShift" | "intBitRShift" | "intFromChar"
        | "stringLength" | "stringCompare" | "stringHash" | "stringHashDjb2"
        | "stringGet" | "stringInt" | "realInt"
        | "arrayLength" | "listLength" => Ty::I32,
        "realAdd" | "realSub" | "realMul" | "realDiv" | "realAbs"
        | "realMax" | "realMin" | "realNeg" | "realFloor" | "realCeil"
        | "realMod" | "realPow" | "intReal" | "stringReal" => Ty::F64,
        "stringBool" => Ty::Bool,
        "intString" | "realString" | "boolString" | "anyString"
        | "stringAppend" | "stringCharAt" | "stringGetStringChar" => Ty::Str,
        "stringEqual" | "stringEq" | "intEq" | "intLt" | "intLe" | "intGt" | "intGe"
        | "intNe" | "realEq" | "realLt" | "realLe" | "realGt" | "realGe"
        | "boolAnd" | "boolOr" | "boolNot" | "boolEq"
        | "referenceEq" | "valueEq" | "isEmpty" | "isSome" | "isNone"
        | "arrayEmpty" | "listEmpty" => Ty::Bool,
        "listHead" | "listFirst" => {
            match args.first().map(|a| a.ty()) {
                Some(Ty::List(inner)) => *inner,
                _ => Ty::Unknown,
            }
        }
        "listRest" | "listTail" | "listReverse" | "listAppend" | "listReverseInPlace" => {
            args.first().map(|a| a.ty()).unwrap_or(Ty::Unknown)
        }
        "arrayGet" => {
            match args.first().map(|a| a.ty()) {
                Some(Ty::Array(inner)) => *inner,
                _ => Ty::Unknown,
            }
        }
        "arrayUpdate" | "arrayCopy" => {
            args.first().map(|a| a.ty()).unwrap_or(Ty::Unknown)
        }
        "arrayCreate" => {
            Ty::Array(Box::new(args.get(1).map(|a| a.ty()).unwrap_or(Ty::Unknown)))
        }
        // arrayCreateNoInit(size, dummy): element type comes from the dummy
        // witness argument, same as arrayCreate. The dummy is dropped at
        // codegen time; here we still use it for type inference.
        "arrayCreateNoInit" => {
            Ty::Array(Box::new(args.get(1).map(|a| a.ty()).unwrap_or(Ty::Unknown)))
        }
        "listArray" => {
            match args.first().map(|a| a.ty()) {
                Some(Ty::List(inner)) => Ty::Array(inner),
                _ => Ty::Unknown,
            }
        }
        // MetaModelica builtin: `stringListStringChar(s)` → `List<String>` of one-char strings.
        // Declared in MetaModelicaBuiltin.mo (`output List<String> chars`); the metamodelica
        // runtime crate exposes it returning `Arc<List<ArcStr>>` to match the list convention.
        "stringListStringChar" => Ty::List(Box::new(Ty::Str)),
        // `listStringCharString` / `stringCharListString` invert that — list of one-char strings → String.
        "listStringCharString" | "stringCharListString" => Ty::Str,
        "arrayList" => {
            match args.first().map(|a| a.ty()) {
                Some(Ty::Array(inner)) => Ty::List(inner),
                _ => Ty::Unknown,
            }
        }
        _ => {
            // Resolve bare names against the current package scope so that calls
            // inside a module (e.g. `deleteMemberOnTrue` from inside `List.mo`)
            // find their canonical fully-qualified definition. Without this, the
            // hierarchy lookup would miss the function and return Ty::Unknown,
            // causing downstream Tuple-coercion logic to skip its tuple handling.
            let canonical = resolve_call_node(func, top_level, pkg_prefix)
                .map(|(q, _)| q)
                .unwrap_or_else(|| func.to_owned());
            match lookup_ty_in_hierarchy(&canonical, top_level) {
                Ty::Function { type_vars, inputs, output, .. } => {
                    // Unify the declared input types with the actual argument types
                    // so that any free type variables in the function signature get
                    // bound to concrete types from the call site. Without this step,
                    // calls like `Mutable.access<T>(mutable: Mutable<T>) -> T` invoked
                    // on a value of type `Mutable<list<X>>` would report their output
                    // as the raw `TypeVar("T")` instead of the concrete `list<X>`,
                    // breaking type-directed codegen (e.g. for-loop iterator handling,
                    // Arc-borrow decisions in pattern matching).
                    //
                    // The unification variables are every type-variable name that
                    // appears anywhere in the function's input or output types —
                    // not just `Ty::Function::type_vars`. Functions defined inside
                    // a generic class (e.g. `function access` inside `uniontype
                    // Mutable<T>`) inherit the enclosing class's type parameter
                    // without listing it in their own `type_vars` field.
                    let mut all_vars: Vec<String> = Vec::new();
                    for inp in inputs.iter() {
                        collect_type_vars_in_ty(&inp.ty, &mut all_vars);
                    }
                    collect_type_vars_in_ty(&output, &mut all_vars);
                    for v in &type_vars {
                        if !all_vars.contains(v) { all_vars.push(v.clone()); }
                    }
                    let mut subst: HashMap<String, Ty> = HashMap::new();
                    for (inp, arg) in inputs.iter().zip(args.iter()) {
                        unify_collect(&inp.ty, &arg.ty(), &all_vars, &mut subst);
                    }
                    apply_subst(&output, &subst)
                }
                other => other,
            }
        }
    }
}

/// Build a substitution map by structurally walking `sig` against `actual`.
/// Whenever `sig` is a `TypeVar` listed in `type_vars`, record the binding to
/// `actual` (first-binding wins; later inconsistent bindings are ignored — a
/// proper compiler pass would report the conflict, but for return-type
/// substitution alone any consistent witness suffices).
fn unify_collect(sig: &Ty, actual: &Ty, type_vars: &[String], subst: &mut HashMap<String, Ty>) {
    match (sig, actual) {
        (Ty::TypeVar(name), other) if type_vars.iter().any(|v| v == name) => {
            if !matches!(other, Ty::Unknown) {
                subst.entry(name.clone()).or_insert_with(|| other.clone());
            }
        }
        (Ty::Option(a), Ty::Option(b))
        | (Ty::List(a),   Ty::List(b))
        | (Ty::Array(a),  Ty::Array(b))
        | (Ty::Range(a),  Ty::Range(b)) => unify_collect(a, b, type_vars, subst),
        (Ty::Tuple(a), Ty::Tuple(b)) if a.len() == b.len() => {
            for (x, y) in a.iter().zip(b.iter()) {
                unify_collect(x, y, type_vars, subst);
            }
        }
        (Ty::Generic(na, aargs), Ty::Generic(nb, bargs))
            if na == nb && aargs.len() == bargs.len() =>
        {
            for (x, y) in aargs.iter().zip(bargs.iter()) {
                unify_collect(x, y, type_vars, subst);
            }
        }
        (Ty::Function { inputs: ai, output: ao, .. },
         Ty::Function { inputs: bi, output: bo, .. }) if ai.len() == bi.len() => {
            for (x, y) in ai.iter().zip(bi.iter()) {
                unify_collect(&x.ty, &y.ty, type_vars, subst);
            }
            unify_collect(ao, bo, type_vars, subst);
        }
        _ => {}
    }
}

/// Apply a type-variable substitution to a type, recursively.
fn apply_subst(ty: &Ty, subst: &HashMap<String, Ty>) -> Ty {
    if subst.is_empty() { return ty.clone(); }
    match ty {
        Ty::TypeVar(name) => subst.get(name).cloned().unwrap_or_else(|| ty.clone()),
        Ty::Option(inner) => Ty::Option(Box::new(apply_subst(inner, subst))),
        Ty::List(inner)   => Ty::List(Box::new(apply_subst(inner, subst))),
        Ty::Array(inner)  => Ty::Array(Box::new(apply_subst(inner, subst))),
        Ty::Range(inner)  => Ty::Range(Box::new(apply_subst(inner, subst))),
        Ty::Tuple(tys)    => Ty::Tuple(tys.iter().map(|t| apply_subst(t, subst)).collect()),
        Ty::Generic(name, args) =>
            Ty::Generic(name.clone(), args.iter().map(|t| apply_subst(t, subst)).collect()),
        Ty::Function { type_vars, inputs, output, name } => Ty::Function {
            type_vars: type_vars.clone(),
            inputs: inputs.iter()
                .map(|inp| FunctionInput { name: inp.name.clone(), ty: apply_subst(&inp.ty, subst), default: inp.default.clone() })
                .collect(),
            output: Box::new(apply_subst(output, subst)),
            name: name.clone(),
        },
        _ => ty.clone(),
    }
}

/// For a dotted name like `exarray.lastUsedIndex`, resolve the first segment in the
/// env to get its type, then walk through remaining segments as field accesses to get
/// the final field type. Returns `None` if the first segment isn't in env.
fn resolve_first_segment_type<'a>(
    dotted: &str,
    segments: &[CrefSegment],
    env: &HashMap<String, Ty>,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> Option<Ty> {
    let first_name = segments.first().map(|s| s.name.as_str()).unwrap_or(dotted);
    let mut ty = env.get(first_name)?.clone();

    // Apply subscripts on the first segment. Each scalar subscript peels off one
    // outer Array/List layer (e.g. `arr[i]` on an `Array<T>` yields `T`). Without
    // this, a reduction over `arr[i]` would type its body as `Array<T>` and the
    // accumulator declaration would be `List<Array<T>>` instead of `List<T>`.
    if let Some(seg) = segments.first() {
        for _ in &seg.subscripts {
            ty = match ty {
                Ty::Array(inner) | Ty::List(inner) => *inner,
                other => other,
            };
        }
    }

    // Walk remaining segments as field accesses to narrow the type, applying
    // each segment's subscripts the same way.
    //
    // Both plain structs (`Ty::RustStruct`) and generic instantiations
    // (`Ty::Generic` of a user-defined struct/uniontype) support field access.
    // For the generic case we look up the underlying type's field declarations
    // and apply the instantiation's type-argument substitution so that fields
    // are reported in the caller's instantiation rather than in the parameter
    // form. Without this, `delst.front` where `delst: MutableList<X>` would
    // return the parameter form `Ty::TypeVar("T")`/`Mutable<list<T>>` and
    // downstream type-directed codegen would fail (e.g. function-call
    // type-variable unification cannot tell what concrete type a `Mutable<...>`
    // wraps without it).
    for seg in segments.iter().skip(1) {
        let field_ty: Option<Ty> = match &ty {
            // Plain record / single-record uniontype rendered as a struct.
            // `record_field_tys` transparently handles the single-record uniontype
            // case by walking through to the sole record child.
            Ty::RustStruct(qname) | Ty::AliasTo(qname) => {
                let field_tys = record_field_tys(qname, top_level);
                field_tys.iter().find(|(n, _)| n == &seg.name).map(|(_, t)| t.clone())
            }
            // Multi-record uniontype rendered as a Rust enum. Field access on an
            // enum value is only legal in MetaModelica when the field exists in
            // the matched record-variant (the compiler is supposed to have
            // narrowed the value by pattern matching). At the type level we
            // don't carry the narrowing, so search all record variants for the
            // field — MetaModelica requires same-named fields across variants
            // to have the same type, so any matching record gives the answer.
            Ty::RustEnum(qname) => uniontype_variant_field_ty(qname, &seg.name, top_level),
            Ty::Generic(rust_name, args) => {
                // `rust_name` uses `::` separators; the hierarchy is dotted.
                let dotted = rust_name.replace("::", ".");
                let formal = class_type_param_names(&dotted, top_level);
                let mut subst: HashMap<String, Ty> = HashMap::new();
                for (name, actual) in formal.iter().zip(args.iter()) {
                    subst.insert(name.clone(), actual.clone());
                }
                let field_tys = record_field_tys(&dotted, top_level);
                field_tys.iter().find(|(n, _)| n == &seg.name).map(|(_, t)| apply_subst(t, &subst))
            }
            _ => None,
        };
        match field_ty {
            Some(t) => ty = t,
            None => break,
        }
        for _ in &seg.subscripts {
            ty = match ty {
                Ty::Array(inner) | Ty::List(inner) => *inner,
                other => other,
            };
        }
    }

    Some(ty)
}

/// Look up a field on a multi-record uniontype by searching each record-variant.
///
/// Use case: in a match arm like `case Flags.ENUM_FLAG() then ... flag.validValues ...`,
/// the bound `flag` is typed as the uniontype (`Ty::RustEnum`) — narrowing isn't
/// tracked at the type level. To resolve `flag.validValues` we walk the
/// uniontype's record children and pick the first record that declares the
/// field. MetaModelica enforces that fields with the same name across variants
/// share a type, so the first hit is authoritative.
///
/// Returns `None` if `qname` doesn't name a uniontype or none of its records
/// declare a field with this name.
fn uniontype_variant_field_ty<'a>(
    qname: &str,
    field: &str,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> Option<Ty> {
    let node = lookup_node(qname, top_level)?;
    let NodeKind::Class(c) = &node.kind else { return None };
    if !matches!(c.restriction, Absyn::Restriction::R_UNIONTYPE) {
        return None;
    }
    for child in node.children.values() {
        let NodeKind::Class(rc) = &child.kind else { continue };
        if !matches!(rc.restriction, Absyn::Restriction::R_RECORD | Absyn::Restriction::R_METARECORD { .. }) {
            continue;
        }
        let rec_members: &[MM::ClassMember] = match &rc.body {
            MM::ClassDef::Parts { members, .. } | MM::ClassDef::ClassExtends { members, .. } => members,
            _ => continue,
        };
        for m in rec_members {
            let MM::ClassMember::Component(cm) = m else { continue };
            if cm.name == field {
                if let Some(comp_node) = child.children.get(&cm.name) {
                    return Some(comp_node.ty.clone());
                }
            }
        }
    }
    None
}

fn record_field_tys<'a>(
    qname: &str,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> Vec<(String, Ty)> {
    // Try the direct path first; fall back to looking through uniontype parents.
    let node = lookup_node(qname, top_level)
        .or_else(|| lookup_record_through_unions(qname, top_level).map(|(_, n)| n));
    let Some(node) = node else { return vec![] };
    let NodeKind::Class(c) = &node.kind else { return vec![] };
    let members: &[MM::ClassMember] = match &c.body {
        MM::ClassDef::Parts { members, .. } | MM::ClassDef::ClassExtends { members, .. } => members,
        _ => return vec![],
    };
    let direct: Vec<(String, Ty)> = members.iter().filter_map(|m| {
        let MM::ClassMember::Component(cm) = m else { return None };
        let child = node.children.get(&cm.name)?;
        Some((cm.name.clone(), child.ty.clone()))
    }).collect();
    if !direct.is_empty() {
        return direct;
    }
    // Single-record uniontype: hierarchy seeding emits the record under the
    // uniontype's own qname (no separate record struct + alias), so direct
    // field lookup on the uniontype node finds no components — the components
    // live on the sole record child. Forward to it.
    if matches!(c.restriction, Absyn::Restriction::R_UNIONTYPE) {
        let record_children: Vec<&NameNode> = node.children.values()
            .filter(|child| matches!(&child.kind, NodeKind::Class(cc)
                if matches!(cc.restriction, Absyn::Restriction::R_RECORD | Absyn::Restriction::R_METARECORD { .. })))
            .collect();
        if record_children.len() == 1 {
            let rec_node = record_children[0];
            if let NodeKind::Class(rc) = &rec_node.kind {
                let rec_members: &[MM::ClassMember] = match &rc.body {
                    MM::ClassDef::Parts { members, .. } | MM::ClassDef::ClassExtends { members, .. } => members,
                    _ => return vec![],
                };
                return rec_members.iter().filter_map(|m| {
                    let MM::ClassMember::Component(cm) = m else { return None };
                    let child = rec_node.children.get(&cm.name)?;
                    Some((cm.name.clone(), child.ty.clone()))
                }).collect();
            }
        }
    }
    direct
}

/// Return the formal type-parameter names declared on a user-defined class
/// (uniontype or record) identified by `qname`. For a uniontype like
/// `uniontype Mutable<T> ... end Mutable;` the result is `["T"]`. Used to
/// substitute a generic instantiation's type arguments into the parameter
/// form of its field declarations when walking field accesses.
fn class_type_param_names<'a>(
    qname: &str,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> Vec<String> {
    let node = lookup_node(qname, top_level)
        .or_else(|| lookup_record_through_unions(qname, top_level).map(|(_, n)| n));
    let Some(node) = node else { return vec![] };
    let NodeKind::Class(c) = &node.kind else { return vec![] };
    match &c.body {
        MM::ClassDef::Parts { type_vars, .. } => type_vars.clone(),
        _ => vec![],
    }
}

fn lookup_node<'a>(
    dotted: &str,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
) -> Option<&'a NameNode<'a>> {
    let mut parts = dotted.split('.');
    let first = parts.next().unwrap_or("");
    let mut node = top_level.get(first)?;
    for part in parts {
        let child = node.children.get(part)?;
        node = child;
    }
    Some(node)
}

/// Infer the type of a MetaModelica expression, building a typed expression tree.
/// `env` maps local variable names to their resolved types.
/// `type_vars` is the list of type-variable names in scope for the enclosing function
/// (e.g. `["Key"]` for a function with `replaceable type Key subtypeof Any`).
/// These are needed to resolve local variable type annotations that reference type params.
pub fn infer_exp<'a>(
    exp: &Absyn::Exp,
    env: &HashMap<String, Ty>,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
    pkg_prefix: &str,
    type_vars: &[String],
) -> TypedExp {
    match exp {
        Absyn::Exp::INTEGER { value } => TypedExp::Lit(Lit::Int(*value)),
        Absyn::Exp::REAL    { value } => TypedExp::Lit(Lit::Real(value.to_string())),
        Absyn::Exp::STRING  { value } => TypedExp::Lit(Lit::Str(value.to_string())),
        Absyn::Exp::BOOL    { value } => TypedExp::Lit(Lit::Bool(*value)),

        Absyn::Exp::CREF { componentRef } => {
            let (name, segments) = extract_cref_segments(componentRef, env, top_level, pkg_prefix);
            // Local env takes priority; fall back to hierarchy, then try qualifying
            // the bare name with the enclosing package prefix (for sibling references).
            // For dotted names like `exarray.lastUsedIndex`, the env key is just
            // the first segment. If it resolves to a record type and the remaining
            // segments are field accesses, use the record's field types.
            let ty = resolve_first_segment_type(&name, &segments, env, top_level).unwrap_or_else(|| {
                let first = segments.first().map(|s| s.name.as_str()).unwrap_or(&name);
                let ty = lookup_ty_in_hierarchy(first, top_level);
                if ty == Ty::Unknown && !pkg_prefix.is_empty() && !name.contains('.') {
                    lookup_ty_in_hierarchy(&format!("{pkg_prefix}.{name}"), top_level)
                } else {
                    ty
                }
            });
            // If the reference still resolves to Unknown and the name matches a known
            // built-in function (not in the hierarchy), treat it as a function pointer
            // so callers can pass it without `.clone()`.
            let ty = if ty == Ty::Unknown && segments.len() == 1 && !name.contains('.') {
                builtin_function_ty(&name).unwrap_or(Ty::Unknown)
            } else {
                ty
            };
            TypedExp::Var { name, segments, ty }
        }

        Absyn::Exp::BINARY  { exp1, op, exp2 }
        | Absyn::Exp::LBINARY  { exp1, op, exp2 }
        | Absyn::Exp::RELATION { exp1, op, exp2 } => {
            let lhs = infer_exp(exp1, env, top_level, pkg_prefix, type_vars);
            let rhs = infer_exp(exp2, env, top_level, pkg_prefix, type_vars);
            let bin_op = absyn_op_to_binop(op);
            let ty = binop_ty(bin_op, &lhs.ty(), &rhs.ty());
            TypedExp::BinOp { op: bin_op, lhs: Box::new(lhs), rhs: Box::new(rhs), ty }
        }

        Absyn::Exp::UNARY { op, exp } => {
            let operand = infer_exp(exp, env, top_level, pkg_prefix, type_vars);
            match op {
                Absyn::Operator::NOT => {
                    // Fold `!true` → `Lit(false)`, `!false` → `Lit(true)`.
                    if let TypedExp::Lit(Lit::Bool(v)) = &operand {
                        TypedExp::Lit(Lit::Bool(!v))
                    } else {
                        TypedExp::UnOp { op: UnOpKind::Not, operand: Box::new(operand), ty: Ty::Bool }
                    }
                }
                _ => {
                    // Fold `-1` → `Lit(Int(-1))`, `-"3.14"` → `Lit(Real("-3.14"))`.
                    match &operand {
                        TypedExp::Lit(Lit::Int(v)) => TypedExp::Lit(Lit::Int(-v)),
                        TypedExp::Lit(Lit::Real(v)) => TypedExp::Lit(Lit::Real(format!("-{v}"))),
                        _ => {
                            let ty = operand.ty();
                            TypedExp::UnOp { op: UnOpKind::Neg, operand: Box::new(operand), ty }
                        }
                    }
                }
            }
        }

        Absyn::Exp::LUNARY { exp, .. } => {
            let operand = infer_exp(exp, env, top_level, pkg_prefix, type_vars);
            // Fold `not true` → `Lit(false)`, `not false` → `Lit(true)`.
            if let TypedExp::Lit(Lit::Bool(v)) = &operand {
                TypedExp::Lit(Lit::Bool(!v))
            } else {
                TypedExp::UnOp { op: UnOpKind::Not, operand: Box::new(operand), ty: Ty::Bool }
            }
        }

        Absyn::Exp::IFEXP { ifExp, trueBranch, elseBranch, elseIfBranch } => {
            let cond  = infer_exp(ifExp, env, top_level, pkg_prefix, type_vars);
            let then_ = infer_exp(trueBranch, env, top_level, pkg_prefix, type_vars);
            let else_ = infer_exp(elseBranch, env, top_level, pkg_prefix, type_vars);
            let elseif: Vec<(TypedExp, TypedExp)> = (&**elseIfBranch).into_iter()
                .map(|(c, b)| (infer_exp(c.as_ref(), env, top_level, pkg_prefix, type_vars), infer_exp(b.as_ref(), env, top_level, pkg_prefix, type_vars)))
                .collect();
            let ty = if then_.ty() != Ty::Unknown { then_.ty() } else { else_.ty() };
            TypedExp::If { cond: Box::new(cond), then_: Box::new(then_), elseif, else_: Box::new(else_), ty }
        }

        Absyn::Exp::CALL { function_, functionArgs, .. } => {
            let func = cref_to_dotted(function_);
            // Detect reduction syntax `f(expr for it in range, ...)` and lower it
            // into a dedicated TypedExp::Reduction node rather than a Call with
            // missing arguments.
            if let Absyn::FunctionArgs::FOR_ITER_FARG { exp: body_exp, iterType, iterators } = functionArgs {
                let iter_kind = match iterType {
                    Absyn::ReductionIterType::COMBINE => ReductionIterKind::Combine,
                    Absyn::ReductionIterType::THREAD  => ReductionIterKind::Thread,
                };
                // Build iterators left-to-right; each iterator binds a name visible
                // to subsequent iterator ranges and the body. We thread `env` so
                // later iterators / body see those bindings.
                let mut iter_env = env.clone();
                let mut iters: Vec<ReductionIter> = Vec::new();
                for it in (&**iterators).into_iter() {
                    let Absyn::ForIterator::ITERATOR { name: it_name, guardExp, range } = it;
                    let range_e = match range {
                        Some(r) => infer_exp(r.as_ref(), &iter_env, top_level, pkg_prefix, type_vars),
                        // A reduction iterator without an explicit range is the implicit-array
                        // form (Modelica spec §3.4.4.2); not yet supported in the lowering.
                        None => TypedExp::Todo("reduction-iter-without-range".to_owned()),
                    };
                    let elem_ty = match range_e.ty() {
                        Ty::List(t) | Ty::Array(t) | Ty::Range(t) => *t,
                        _ => Ty::Unknown,
                    };
                    iter_env.insert(it_name.to_string(), elem_ty.clone());
                    let guard = guardExp.as_ref().map(|g| infer_exp(g.as_ref(), &iter_env, top_level, pkg_prefix, type_vars));
                    iters.push(ReductionIter { name: it_name.to_string(), range: range_e, guard, elem_ty });
                }
                let body = infer_exp(body_exp.as_ref(), &iter_env, top_level, pkg_prefix, type_vars);
                // The reduction's result type depends on `func`:
                //  - list / listReverse / listAppend → list<body_ty>
                //  - min / max → body_ty itself
                //  - sum / product → body_ty (numeric)
                //  - user function → the function's output type
                let body_ty = body.ty();
                let ty = match func.as_str() {
                    "list" | "listReverse" => Ty::List(Box::new(body_ty.clone())),
                    "listAppend" => body_ty.clone(),
                    "sum" | "product" | "min" | "max" => body_ty.clone(),
                    _ => match lookup_ty_in_hierarchy(&func, top_level) {
                        Ty::Function { output, .. } => *output,
                        _ => Ty::Unknown,
                    },
                };
                return TypedExp::Reduction {
                    func,
                    body: Box::new(body),
                    iterators: iters,
                    iter_kind,
                    ty,
                };
            }
            let (args, named_args) = extract_call_args(functionArgs, env, top_level, pkg_prefix, type_vars);
            let sig_ty = lookup_ty_in_hierarchy(&func, top_level);
            // Resolve the call node using import-aware lookup so that dotted names whose
            // first segment is an import alias (e.g. `LookupTree.Tree.EMPTY` where
            // `import LookupTree = NFLookupTree`) and names relative to the current
            // package (e.g. bare `LEAF` or dotted `Tree.EMPTY` inside `AvlSetInt`) all
            // resolve to their canonical fully-qualified path.
            let resolved: Option<(String, &NameNode)> = resolve_call_node(&func, top_level, pkg_prefix);
            let is_constructor = match &sig_ty {
                Ty::RustStruct(_) | Ty::RustEnum(_) => true,
                _ => {
                    if let Some((_, node)) = &resolved {
                        matches!(node.kind, NodeKind::Class(ref c) if matches!(c.restriction, Absyn::Restriction::R_RECORD | Absyn::Restriction::R_UNIONTYPE))
                    } else {
                        false
                    }
                }
            };
            if is_constructor {
                // Use the canonical (fully-qualified) name so downstream codegen can
                // look up fields, even when the call site used a shorter path.
                let canonical = resolved.as_ref().map(|(q, _)| q.clone()).unwrap_or(func.clone());
                let ty = match lookup_ty_in_hierarchy(&canonical, top_level) {
                    Ty::Function { output, .. } => *output,
                    other => other,
                };
                let field_names = match &sig_ty {
                    Ty::RustStruct(qname) | Ty::RustEnum(qname) => {
                        record_field_tys(qname, top_level).into_iter().map(|(n, _)| n).collect()
                    }
                    _ => {
                        record_field_tys(&canonical, top_level).into_iter().map(|(n, _)| n).collect()
                    }
                };
                TypedExp::Constructor { name: canonical, args, named_args, ty, field_names }
            } else {
                let ty = call_ty(&func, &args, top_level, pkg_prefix);
                TypedExp::Call { func, args, named_args, ty, sig_ty }
            }
        }

        Absyn::Exp::PARTEVALFUNCTION { function_, functionArgs } => {
            // `function f(arg = e, ...)`: partial application of `f` with the
            // specified arguments bound. The remaining formals stay open and
            // must be supplied at every later call site.
            let func = cref_to_dotted(function_);
            let (args, named_args) = extract_call_args(functionArgs, env, top_level, pkg_prefix, type_vars);
            // Resolve the underlying function's signature. We need formal
            // names and types (in order) so codegen can identify which
            // formals were bound positionally / by name and emit a closure
            // that forwards the remaining unbound formals.
            //
            // We use the same name-resolution path as a normal CALL site so
            // that bare references to sibling functions (e.g. `edge_finder`
            // inside its enclosing package) are found.  Built-ins (e.g.
            // `realEq`) live outside the hierarchy and are looked up in
            // `builtin_function_ty`.
            let sig_ty = match resolve_call_node(&func, top_level, pkg_prefix) {
                Some((_, node)) => node.ty.clone(),
                None => builtin_function_ty(&func).unwrap_or_else(|| lookup_ty_in_hierarchy(&func, top_level)),
            };
            let ty = match &sig_ty {
                Ty::Function { type_vars: tvs, inputs, output, .. } => {
                    let bound_pos = args.len();
                    let bound_named: std::collections::HashSet<&str> =
                        named_args.iter().map(|(n, _)| n.as_str()).collect();
                    let remaining: Vec<FunctionInput> = inputs.iter().enumerate()
                        .filter_map(|(i, inp)| {
                            if i < bound_pos { return None; }
                            if bound_named.contains(inp.name.as_str()) { return None; }
                            Some(inp.clone())
                        })
                        .collect();
                    Ty::Function {
                        type_vars: tvs.clone(),
                        inputs: remaining,
                        output: output.clone(),
                        // The result is no longer the original named alias —
                        // it's a freshly-shaped function whose arity differs.
                        name: None,
                    }
                }
                _ => Ty::Unknown,
            };
            TypedExp::PartEval { func, args, named_args, sig_ty, ty }
        }

        Absyn::Exp::TUPLE { expressions } => {
            let elems: Vec<TypedExp> = (&**expressions).into_iter()
                .map(|e| infer_exp(e.as_ref(), env, top_level, pkg_prefix, type_vars))
                .collect();
            TypedExp::Tuple(elems)
        }

        Absyn::Exp::ARRAY { arrayExp } => {
            let elems: Vec<TypedExp> = (&**arrayExp).into_iter()
                .map(|e| infer_exp(e.as_ref(), env, top_level, pkg_prefix, type_vars))
                .collect();
            let inner_ty = elems.first().map(|e| e.ty()).unwrap_or(Ty::Unknown);
            TypedExp::Array { elems, ty: Ty::List(Box::new(inner_ty)) }
        }

        Absyn::Exp::CONS { head, rest } => {
            let head_e = infer_exp(head, env, top_level, pkg_prefix, type_vars);
            let tail_e = infer_exp(rest, env, top_level, pkg_prefix, type_vars);
            let ty = tail_e.ty();
            TypedExp::Cons { head: Box::new(head_e), tail: Box::new(tail_e), ty }
        }

        Absyn::Exp::MATCHEXP { matchTy, inputExp, localDecls, cases, .. } => {
            let input = infer_exp(inputExp, env, top_level, pkg_prefix, type_vars);
            let kind = match matchTy {
                Absyn::MatchType::MATCH => MatchKind::Match,
                Absyn::MatchType::MATCHCONTINUE => MatchKind::MatchContinue,
            };
            // Process match-level local declarations: these are visible to all arms
            // and must be declared in each arm body. We add them to the environment
            // before inferring the case bodies so that their types are known inside arms.
            let match_locals_raw = infer_case_locals_standalone(localDecls, type_vars, top_level);
            let mut case_env = env.clone();
            for (n, t, _) in &match_locals_raw {
                case_env.insert(n.clone(), t.clone());
            }
            // Type-check default-binding expressions for match-level locals in
            // an environment where the surrounding scope and all match-level
            // locals are visible.  Pattern bindings are *not* — match-level
            // locals are evaluated once per arm entry, before patterns bind.
            let match_locals: Vec<(String, Ty, Option<TypedExp>)> = match_locals_raw.into_iter()
                .map(|(n, t, d)| {
                    let td = d.as_ref().map(|e| infer_exp(e, &case_env, top_level, pkg_prefix, type_vars));
                    (n, t, td)
                })
                .collect();
            let typed_cases: Vec<TypedCase> = (&**cases).into_iter()
                .map(|c| infer_case(c, &case_env, top_level, pkg_prefix, &match_locals, type_vars))
                .collect();
            let ty = typed_cases.iter()
                .map(|c| c.result.ty())
                .find(|t| *t != Ty::Unknown)
                .unwrap_or(Ty::Unknown);
            TypedExp::Match { kind, input: Box::new(input), cases: typed_cases, ty }
        }

        Absyn::Exp::RANGE { start, step, stop } => {
            let start_e = infer_exp(start, env, top_level, pkg_prefix, type_vars);
            let step_e = step.as_ref().map(|s| infer_exp(s, env, top_level, pkg_prefix, type_vars));
            let stop_e = infer_exp(stop, env, top_level, pkg_prefix, type_vars);
            let elem_ty = start_e.ty();
            TypedExp::Range {
                start: Box::new(start_e),
                step: step_e.map(Box::new),
                stop: Box::new(stop_e),
                elem_ty,
            }
        }

        other => TypedExp::Todo(format!("{other:?}").chars().take(80).collect()),
    }
}

fn extract_call_args<'a>(
    function_args: &Absyn::FunctionArgs,
    env: &HashMap<String, Ty>,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
    pkg_prefix: &str,
    type_vars: &[String],
) -> (Vec<TypedExp>, Vec<(String, TypedExp)>) {
    match function_args {
        Absyn::FunctionArgs::FUNCTIONARGS { args, argNames } => {
            let pos: Vec<TypedExp> = (&**args).into_iter()
                .map(|a| infer_exp(a.as_ref(), env, top_level, pkg_prefix, type_vars))
                .collect();
            let named: Vec<(String, TypedExp)> = (&**argNames).into_iter()
                .map(|na| {
                    let Absyn::NamedArg::NAMEDARG { argName, argValue } = na.as_ref();
                    (argName.to_string(), infer_exp(argValue.as_ref(), env, top_level, pkg_prefix, type_vars))
                })
                .collect();
            (pos, named)
        }
        _ => (vec![], vec![]),
    }
}

fn infer_case<'a>(
    case: &Absyn::Case,
    env: &HashMap<String, Ty>,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
    pkg_prefix: &str,
    // Match-level locals already incorporated into `env` by the caller.
    extra_locals: &[(String, Ty, Option<TypedExp>)],
    type_vars: &[String],
) -> TypedCase {
    fn path_to_dotted(path: &Absyn::Path) -> String {
        match path {
            Absyn::Path::IDENT { name } => name.to_string(),
            Absyn::Path::QUALIFIED { name, path } => format!("{name}.{}", path_to_dotted(path)),
            Absyn::Path::FULLYQUALIFIED { path } => path_to_dotted(path),
        }
    }

    /// Resolve a single MetaModelica TypeSpec to a `Ty`.
    ///
    /// `type_vars` lists the type-variable names in scope for the enclosing function.
    /// This is required so that references to type variables like `Option<Key>` produce
    /// `Ty::Option(Ty::TypeVar("Key"))` rather than falling through to a failed hierarchy
    /// lookup and returning `Ty::Unknown`.
    ///
    /// Handling mirrors `hierarchy::resolve_type_spec`:
    /// - Primitives (Integer, Real, Boolean, String) → primitive Ty variants
    /// - Type variable names → `Ty::TypeVar(name)`
    /// - Option<T>, list<T>/List<T>, array<T>/Array<T>, tuple<...> → structured Ty variants
    /// - `polymorphic<T>` (the parser's representation of `replaceable type T subtypeof Any`) →
    ///   recurse into the inner spec (stripping the wrapper)
    /// - Everything else → hierarchy lookup
    fn typespec_to_ty(type_spec: &Absyn::TypeSpec, type_vars: &[String], top_level: &BTreeMap<String, NameNode<'_>>) -> Ty {
        match type_spec {
            Absyn::TypeSpec::TPATH { path, .. } => {
                let name = path_to_dotted(path);
                match name.as_str() {
                    "Integer" => Ty::I32,
                    "Real"    => Ty::F64,
                    "Boolean" => Ty::Bool,
                    "String"  => Ty::Str,
                    _ if type_vars.iter().any(|v| v == &name) => Ty::TypeVar(name),
                    _ => lookup_ty_in_hierarchy(&name, top_level),
                }
            }
            Absyn::TypeSpec::TCOMPLEX { path, typeSpecs, .. } => {
                let args: Vec<std::sync::Arc<Absyn::TypeSpec>> = (&**typeSpecs).into_iter().cloned().collect();
                let ctor = path_to_dotted(path);
                match ctor.as_str() {
                    "Option" if args.len() == 1 => {
                        Ty::Option(Box::new(typespec_to_ty(args[0].as_ref(), type_vars, top_level)))
                    }
                    "list" | "List" if args.len() == 1 => {
                        Ty::List(Box::new(typespec_to_ty(args[0].as_ref(), type_vars, top_level)))
                    }
                    "array" | "Array" if args.len() == 1 => {
                        Ty::Array(Box::new(typespec_to_ty(args[0].as_ref(), type_vars, top_level)))
                    }
                    "tuple" => {
                        let tys: Vec<Ty> = args.iter().map(|a| typespec_to_ty(a.as_ref(), type_vars, top_level)).collect();
                        Ty::Tuple(tys)
                    }
                    // `polymorphic<T>` is the parser's representation for `replaceable type T subtypeof Any`.
                    // Strip the wrapper and resolve the inner spec directly.
                    "polymorphic" if args.len() == 1 => {
                        typespec_to_ty(args[0].as_ref(), type_vars, top_level)
                    }
                    // Unknown generic — look up the base name in the hierarchy.
                    // The type arguments are not represented in Ty yet; this is a known limitation.
                    _ => lookup_ty_in_hierarchy(&ctor, top_level),
                }
            }
        }
    }

    fn infer_case_locals(local_decls: &std::sync::Arc<mmwinnow::List<std::sync::Arc<Absyn::ElementItem>>>, type_vars: &[String], top_level: &BTreeMap<String, NameNode<'_>>) -> Vec<(String, Ty, Option<Absyn::Exp>)> {
        let mut out = Vec::new();
        for item in (&**local_decls).into_iter() {
            let Absyn::ElementItem::ELEMENTITEM { element } = item.as_ref() else { continue };
            let Absyn::Element::ELEMENT { specification, .. } = element else { continue };
            let Absyn::ElementSpec::COMPONENTS { typeSpec, components, .. } = specification else { continue };
            let ty = typespec_to_ty(&typeSpec, type_vars, top_level);
            for comp_item in (&**components).into_iter() {
                let Absyn::ComponentItem::COMPONENTITEM { component, .. } = comp_item.as_ref();
                let Absyn::Component::COMPONENT { name, modification, .. } = component;
                let default = extract_default_exp(modification).cloned();
                out.push((name.to_string(), ty.clone(), default));
            }
        }
        out
    }

    fn infer_eq_item<'a>(
        item: &Absyn::EquationItem,
        env: &mut HashMap<String, Ty>,
        top_level: &'a BTreeMap<String, NameNode<'a>>,
        pkg_prefix: &str,
        type_vars: &[String],
    ) -> Option<TypedStmt> {
        let eq = match item {
            Absyn::EquationItem::EQUATIONITEM { equation_, .. } => equation_,
            Absyn::EquationItem::EQUATIONITEMCOMMENT { .. } => return None,
        };
        Some(match eq.as_ref() {
            Absyn::Equation::EQ_EQUALS { leftSide, rightSide } => {
                let rhs = infer_exp(rightSide, env, top_level, pkg_prefix, type_vars);
                let lhs = infer_pat(leftSide, env, top_level, pkg_prefix, type_vars);
                for (name, _ty) in pat_bindings(&lhs) {
                    env.insert(name, rhs.ty());
                }
                TypedStmt::Assign { lhs, rhs }
            }
            Absyn::Equation::EQ_NORETCALL { functionName, functionArgs } => {
                let func = cref_to_dotted(functionName);
                let (args, named_args) = extract_call_args(functionArgs, env, top_level, pkg_prefix, type_vars);
                let sig_ty = lookup_ty_in_hierarchy(&func, top_level);
                let resolved = resolve_call_node(&func, top_level, pkg_prefix);
                let is_constructor = match &sig_ty {
                    Ty::RustStruct(_) | Ty::RustEnum(_) => true,
                    _ => {
                        if let Some((_, node)) = &resolved {
                            matches!(node.kind, NodeKind::Class(ref c) if matches!(c.restriction, Absyn::Restriction::R_RECORD | Absyn::Restriction::R_UNIONTYPE))
                        } else {
                            false
                        }
                    }
                };
                let call = if is_constructor {
                    let canonical = resolved.as_ref().map(|(q, _)| q.clone()).unwrap_or(func.clone());
                    let ty = match lookup_ty_in_hierarchy(&canonical, top_level) {
                        Ty::Function { output, .. } => *output,
                        other => other,
                    };
                    let field_names = match &sig_ty {
                        Ty::RustStruct(qname) | Ty::RustEnum(qname) => {
                            record_field_tys(qname, top_level).into_iter().map(|(n, _)| n).collect()
                        }
                        _ => {
                            record_field_tys(&canonical, top_level).into_iter().map(|(n, _)| n).collect()
                        }
                    };
                    TypedExp::Constructor { name: canonical, args, named_args, ty, field_names }
                } else {
                    let ty = call_ty(&func, &args, top_level, pkg_prefix);
                    TypedExp::Call { func, args, named_args, ty, sig_ty }
                };
                TypedStmt::NoRetCall { call }
            }
            Absyn::Equation::EQ_IF { ifExp, equationTrueItems, elseIfBranches, equationElseItems } => {
                let cond = infer_exp(ifExp, env, top_level, pkg_prefix, type_vars);
                let then_ = infer_eq_items_list_arc(equationTrueItems, env, top_level, pkg_prefix, type_vars);
                let elseif: Vec<(TypedExp, Vec<TypedStmt>)> = (&**elseIfBranches).into_iter()
                    .map(|(c, b)| (
                        infer_exp(c, env, top_level, pkg_prefix, type_vars),
                        infer_eq_items_list_arc(b, env, top_level, pkg_prefix, type_vars),
                    ))
                    .collect();
                let else_ = infer_eq_items_list_arc(equationElseItems, env, top_level, pkg_prefix, type_vars);
                TypedStmt::If { cond, then_, elseif, else_ }
            }
            Absyn::Equation::EQ_FOR { iterators, forEquations } => {
                let iters: Vec<Absyn::ForIterator> = (&**iterators).into_iter().cloned().collect();
                if iters.len() == 1 {
                    let Absyn::ForIterator::ITERATOR { name, range, .. } = &iters[0];
                    let range_e = match range {
                        Some(r) => infer_exp(r.as_ref(), env, top_level, pkg_prefix, type_vars),
                        None => TypedExp::Todo("for-without-range".to_owned()),
                    };
                    let elem_ty = match range_e.ty() {
                        Ty::List(t) | Ty::Array(t) | Ty::Range(t) => *t,
                        _ => Ty::Unknown,
                    };
                    let mut inner = env.clone();
                    inner.insert(name.to_string(), elem_ty);
                    let body = infer_eq_items_list_arc(forEquations, &mut inner, top_level, pkg_prefix, type_vars);
                    TypedStmt::For { var: name.to_string(), range: range_e, body }
                } else {
                    TypedStmt::Todo("multi-iterator-for-eq".to_owned())
                }
            }
            Absyn::Equation::EQ_FAILURE { equ } => {
                let mut body = Vec::new();
                if let Some(s) = infer_eq_item(equ, env, top_level, pkg_prefix, type_vars) {
                    body.push(s);
                }
                TypedStmt::Failure { body }
            }
            other => TypedStmt::Todo(format!("{other:?}").chars().take(60).collect()),
        })
    }

    fn infer_eq_items_list<'a>(
        items: &std::sync::Arc<mmwinnow::List<Absyn::EquationItem>>,
        env: &mut HashMap<String, Ty>,
        top_level: &'a BTreeMap<String, NameNode<'a>>,
        pkg_prefix: &str,
        type_vars: &[String],
    ) -> Vec<TypedStmt> {
        let mut out = Vec::new();
        for it in (&**items).into_iter() {
            if let Some(s) = infer_eq_item(it, env, top_level, pkg_prefix, type_vars) {
                out.push(s);
            }
        }
        out
    }

    fn infer_eq_items_list_arc<'a>(
        items: &std::sync::Arc<mmwinnow::List<std::sync::Arc<Absyn::EquationItem>>>,
        env: &mut HashMap<String, Ty>,
        top_level: &'a BTreeMap<String, NameNode<'a>>,
        pkg_prefix: &str,
        type_vars: &[String],
    ) -> Vec<TypedStmt> {
        let mut out = Vec::new();
        for it in (&**items).into_iter() {
            if let Some(s) = infer_eq_item(it.as_ref(), env, top_level, pkg_prefix, type_vars) {
                out.push(s);
            }
        }
        out
    }

    fn infer_case_class_part<'a>(
        class_part: &Absyn::ClassPart,
        env: &mut HashMap<String, Ty>,
        top_level: &'a BTreeMap<String, NameNode<'a>>,
        pkg_prefix: &str,
        type_vars: &[String],
    ) -> Vec<TypedStmt> {
        match class_part {
            Absyn::ClassPart::ALGORITHMS { contents }
            | Absyn::ClassPart::INITIALALGORITHMS { contents } => {
                infer_stmts_list(contents, env, top_level, pkg_prefix, type_vars)
            }
            Absyn::ClassPart::EQUATIONS { contents }
            | Absyn::ClassPart::INITIALEQUATIONS { contents } => {
                infer_eq_items_list(contents, env, top_level, pkg_prefix, type_vars)
            }
            _ => vec![],
        }
    }

    match case {
        Absyn::Case::CASE { pattern, patternGuard, localDecls, classPart, result, .. } => {
            // Case-level locals (`local list<X> M;`) must be visible to
            // `infer_pat` so that a pattern reference like `node::M` resolves
            // `M` to the locally-declared variable rather than being
            // misclassified as a constructor by the uppercase heuristic in
            // `infer_pat`. Without this, an uppercase-named local appearing
            // in pattern position becomes `TypedPat::Constructor { name: "M" }`
            // and downstream codegen emits it as a bare ctor name, losing the
            // ref-binding through the surrounding `Cons.tail` Arc edge.
            let case_locals_pre = infer_case_locals(localDecls, type_vars, top_level);
            let mut pat_env = env.clone();
            for (n, t, _) in &case_locals_pre {
                pat_env.insert(n.clone(), t.clone());
            }
            let pat = infer_pat(pattern, &pat_env, top_level, pkg_prefix, type_vars);
            let mut inner_env = env.clone();
            inner_env.extend(pat_bindings(&pat));
            // Start with match-level locals (already in env), then add case-level locals.
            // Dedup: case-level locals shadow match-level ones with the same name.
            let mut locals: Vec<(String, Ty, Option<TypedExp>)> = extra_locals.to_vec();
            // Build the environment in which case-local default expressions are
            // type-checked: surrounding scope + pattern bindings + all
            // case-locals (so a later local can mention an earlier one).
            // This mirrors MetaModelica's case-local evaluation rules.
            let mut local_init_env = inner_env.clone();
            for (n, t, _) in &case_locals_pre {
                local_init_env.insert(n.clone(), t.clone());
            }
            for (n, t, default_exp) in &case_locals_pre {
                let typed_default = default_exp.as_ref()
                    .map(|e| infer_exp(e, &local_init_env, top_level, pkg_prefix, type_vars));
                if let Some(pos) = locals.iter().position(|(ln, _, _)| ln == n) {
                    locals[pos] = (n.clone(), t.clone(), typed_default); // case-level shadows match-level
                } else {
                    locals.push((n.clone(), t.clone(), typed_default));
                }
            }
            for (n, t, _) in &locals {
                inner_env.insert(n.clone(), t.clone());
            }
            let guard = patternGuard.as_ref().map(|g| infer_exp(g, &inner_env, top_level, pkg_prefix, type_vars));
            let mut case_env = inner_env.clone();
            let stmts = infer_case_class_part(classPart, &mut case_env, top_level, pkg_prefix, type_vars);
            // Discover any new variables first assigned inside the arm body (not declared
            // anywhere). These arise when the MetaModelica source omits explicit local
            // declarations for intermediate variables that are only assigned once.
            for (n, t) in case_env.iter() {
                if !inner_env.contains_key(n) && !locals.iter().any(|(ln, _, _)| ln == n) {
                    locals.push((n.clone(), t.clone(), None));
                }
            }
            TypedCase { pattern: pat, guard, locals, stmts, result: infer_exp(result, &case_env, top_level, pkg_prefix, type_vars) }
        }
        Absyn::Case::ELSE { localDecls, classPart, result, .. } => {
            let mut case_env = env.clone();
            let mut locals: Vec<(String, Ty, Option<TypedExp>)> = extra_locals.to_vec();
            let case_locals = infer_case_locals(localDecls, type_vars, top_level);
            // Build the environment in which case-local default expressions
            // are type-checked: surrounding scope + all case-locals.
            let mut local_init_env = env.clone();
            for (n, t, _) in &case_locals {
                local_init_env.insert(n.clone(), t.clone());
            }
            for (n, t, default_exp) in &case_locals {
                let typed_default = default_exp.as_ref()
                    .map(|e| infer_exp(e, &local_init_env, top_level, pkg_prefix, type_vars));
                if let Some(pos) = locals.iter().position(|(ln, _, _)| ln == n) {
                    locals[pos] = (n.clone(), t.clone(), typed_default);
                } else {
                    locals.push((n.clone(), t.clone(), typed_default));
                }
            }
            for (n, t, _) in &locals {
                case_env.insert(n.clone(), t.clone());
            }
            let stmts = infer_case_class_part(classPart, &mut case_env, top_level, pkg_prefix, type_vars);
            for (n, t) in case_env.iter() {
                if !env.contains_key(n) && !locals.iter().any(|(ln, _, _)| ln == n) {
                    locals.push((n.clone(), t.clone(), None));
                }
            }
            TypedCase { pattern: TypedPat::Wildcard, guard: None, locals, stmts, result: infer_exp(result, &case_env, top_level, pkg_prefix, type_vars) }
        }
    }
}

/// Resolve match-level local declarations (from `MATCHEXP.localDecls`) to a list of
/// `(name, Ty)` pairs. This is the same resolution logic as `infer_case_locals`
/// (used for case-level locals) but exposed at the top level so `infer_exp` can
/// call it directly when processing a `MATCHEXP` node.
///
/// `type_vars` must be the function-level type variable names (e.g. `["Key"]`).
fn infer_case_locals_standalone(
    local_decls: &std::sync::Arc<mmwinnow::List<std::sync::Arc<Absyn::ElementItem>>>,
    type_vars: &[String],
    top_level: &BTreeMap<String, NameNode<'_>>,
) -> Vec<(String, Ty, Option<Absyn::Exp>)> {
    fn path_to_dotted(path: &Absyn::Path) -> String {
        match path {
            Absyn::Path::IDENT { name } => name.to_string(),
            Absyn::Path::QUALIFIED { name, path } => format!("{name}.{}", path_to_dotted(path)),
            Absyn::Path::FULLYQUALIFIED { path } => path_to_dotted(path),
        }
    }
    fn typespec_to_ty(type_spec: &Absyn::TypeSpec, type_vars: &[String], top_level: &BTreeMap<String, NameNode<'_>>) -> Ty {
        match type_spec {
            Absyn::TypeSpec::TPATH { path, .. } => {
                let name = path_to_dotted(path);
                match name.as_str() {
                    "Integer" => Ty::I32,
                    "Real"    => Ty::F64,
                    "Boolean" => Ty::Bool,
                    "String"  => Ty::Str,
                    _ if type_vars.iter().any(|v| v == &name) => Ty::TypeVar(name),
                    _ => lookup_ty_in_hierarchy(&name, top_level),
                }
            }
            Absyn::TypeSpec::TCOMPLEX { path, typeSpecs, .. } => {
                let args: Vec<std::sync::Arc<Absyn::TypeSpec>> = (&**typeSpecs).into_iter().cloned().collect();
                let ctor = path_to_dotted(path);
                match ctor.as_str() {
                    "Option" if args.len() == 1 => {
                        Ty::Option(Box::new(typespec_to_ty(args[0].as_ref(), type_vars, top_level)))
                    }
                    "list" | "List" if args.len() == 1 => {
                        Ty::List(Box::new(typespec_to_ty(args[0].as_ref(), type_vars, top_level)))
                    }
                    "array" | "Array" if args.len() == 1 => {
                        Ty::Array(Box::new(typespec_to_ty(args[0].as_ref(), type_vars, top_level)))
                    }
                    "tuple" => {
                        let tys: Vec<Ty> = args.iter().map(|a| typespec_to_ty(a.as_ref(), type_vars, top_level)).collect();
                        Ty::Tuple(tys)
                    }
                    "polymorphic" if args.len() == 1 => {
                        typespec_to_ty(args[0].as_ref(), type_vars, top_level)
                    }
                    _ => lookup_ty_in_hierarchy(&ctor, top_level),
                }
            }
        }
    }

    let mut out = Vec::new();
    for item in (&**local_decls).into_iter() {
        let Absyn::ElementItem::ELEMENTITEM { element } = item.as_ref() else { continue };
        let Absyn::Element::ELEMENT { specification, .. } = element else { continue };
        let Absyn::ElementSpec::COMPONENTS { typeSpec, components, .. } = specification else { continue };
        let ty = typespec_to_ty(&typeSpec, type_vars, top_level);
        for comp_item in (&**components).into_iter() {
            let Absyn::ComponentItem::COMPONENTITEM { component, .. } = comp_item.as_ref();
            let Absyn::Component::COMPONENT { name, modification, .. } = component;
            let default = extract_default_exp(modification).cloned();
            out.push((name.to_string(), ty.clone(), default));
        }
    }
    out
}

/// Check if a pattern is a "local base" — a variable or field-access chain that can be
/// used as the base of a field access expression (as opposed to a constructor/literal).
fn is_local_base(pat: &TypedPat, env: &HashMap<String, Ty>) -> bool {
    match pat {
        TypedPat::Var(name) => env.contains_key(name),
        TypedPat::FieldAccess { base, .. } => is_local_base(base, env),
        _ => false,
    }
}

/// Convert a local-base pattern back into a TypedExp for use as the base of field access.
fn pat_to_exp(pat: &TypedPat, top_level: &BTreeMap<String, NameNode<'_>>) -> TypedExp {
    match pat {
        TypedPat::Var(name) => TypedExp::Var {
            name: name.clone(),
            segments: vec![CrefSegment { name: name.clone(), subscripts: vec![] }],
            ty: lookup_ty_in_hierarchy(name, top_level),
        },
        TypedPat::FieldAccess { base, field } => {
            let base_exp = pat_to_exp(base, top_level);
            // Build a Var that represents the full dotted path, appending this field.
            let base_name = match base_exp {
                TypedExp::Var { name, .. } => name,
                _ => "_".to_owned(),
            };
            TypedExp::Var {
                name: format!("{base_name}.{field}"),
                segments: vec![],
                ty: Ty::Unknown,
            }
        },
        _ => TypedExp::Var { name: "_".into(), segments: vec![], ty: Ty::Unknown },
    }
}

/// Infer the pattern from an expression in case-pattern position.
/// `env` and `pkg_prefix` are needed for subscripted refs (Index patterns) in assignment LHS.
pub fn infer_pat<'a>(
    exp: &Absyn::Exp,
    env: &HashMap<String, Ty>,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
    pkg_prefix: &str,
    type_vars: &[String],
) -> TypedPat {
    match exp {
        Absyn::Exp::INTEGER { value } => TypedPat::Lit(Lit::Int(*value)),
        Absyn::Exp::REAL    { value } => TypedPat::Lit(Lit::Real(value.to_string())),
        Absyn::Exp::STRING  { value } => TypedPat::Lit(Lit::Str(value.to_string())),
        Absyn::Exp::BOOL    { value } => TypedPat::Lit(Lit::Bool(*value)),

        Absyn::Exp::CREF { componentRef } => {
            match componentRef.as_ref() {
                Absyn::ComponentRef::WILD | Absyn::ComponentRef::ALLWILD => TypedPat::Wildcard,
                Absyn::ComponentRef::CREF_IDENT { name, subscripts } if subscripts.is_empty() => {
                    if &**name == "_" {
                        TypedPat::Wildcard
                    } else if env.contains_key(&**name) {
                        // The name is bound in the current scope (function
                        // input/output/protected, match-level local, or
                        // case-level local). It must be a pattern variable —
                        // a local variable shadows any same-named constructor
                        // in scope, and crucially this prevents an uppercase
                        // local (e.g. `local list<X> M;` referenced as `node::M`)
                        // from being misclassified as a constructor.
                        TypedPat::Var(name.to_string())
                    } else if name.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                        // Uppercase identifiers in pattern position are constructors in
                        // MetaModelica (variants/records), not variable binders.
                        let ty = lookup_ty_in_hierarchy(name, top_level);
                        TypedPat::Constructor { name: name.to_string(), fields: vec![], named_fields: vec![], ty }
                    } else {
                        TypedPat::Var(name.to_string())
                    }
                }
                // Subscripted reference in pattern position (e.g. `arr[1]` on LHS of `:=`).
                Absyn::ComponentRef::CREF_IDENT { name, subscripts } => {
                    let sub = (&**subscripts).into_iter()
                        .filter_map(|s| {
                            if let Absyn::Subscript::SUBSCRIPT { subscript } = s.as_ref() {
                                Some(subscript.as_ref().clone())
                            } else {
                                None
                            }
                        })
                        .next();
                    if let Some(sub_exp) = sub {
                        let base_ty = env.get(&**name).cloned().unwrap_or_else(|| {
                            lookup_ty_in_hierarchy(name, top_level)
                        });
                        let base = TypedExp::Var { name: name.to_string(), segments: vec![], ty: base_ty };
                        TypedPat::Index {
                            base,
                            index: infer_exp(&sub_exp, env, top_level, pkg_prefix, type_vars),
                        }
                    } else {
                        TypedPat::Var(name.to_string())
                    }
                }
                // Qualified reference with subscripts (e.g. `a.b[1]` on LHS of `:=`).
                Absyn::ComponentRef::CREF_QUAL { name, subscripts, componentRef: rest } => {
                    let has_subs = (&**subscripts).into_iter().any(|s| matches!(s.as_ref(), Absyn::Subscript::SUBSCRIPT { .. }));
                    if has_subs {
                        let dotted = cref_to_dotted(rest);
                        // The subscripted part is the tail of the qualified ref.
                        // Build the full dotted name (without subscript info for type lookup)
                        // and emit as Index pattern.
                        let full_dotted = cref_to_dotted(componentRef);
                        let sub = (&**subscripts).into_iter()
                            .filter_map(|s| {
                                if let Absyn::Subscript::SUBSCRIPT { subscript } = s.as_ref() {
                                    Some(subscript.as_ref().clone())
                                } else {
                                    None
                                }
                            })
                            .next();
                        if let Some(sub_exp) = sub {
                            // Check if inner ref resolves to a local variable — if so, build
                            // the base expression as a chain of field accesses, not a package path.
                            let inner_pat = infer_pat(&Absyn::Exp::CREF { componentRef: rest.clone() }, env, top_level, pkg_prefix, type_vars);
                            let base = if is_local_base(&inner_pat, env) {
                                pat_to_exp(&inner_pat, top_level)
                            } else {
                                TypedExp::Var { name: dotted.clone(), segments: vec![], ty: lookup_ty_in_hierarchy(&dotted, top_level) }
                            };
                            TypedPat::Index {
                                base,
                                index: infer_exp(&sub_exp, env, top_level, pkg_prefix, type_vars),
                            }
                        } else {
                            let ty = lookup_ty_in_hierarchy(&full_dotted, top_level);
                            TypedPat::Constructor { name: full_dotted, fields: vec![], named_fields: vec![], ty }
                        }
                    } else {
                        // For non-subscripted qualified refs, preserve source order in local
                        // field chains (e.g. `exarray.lastUsedIndex`), otherwise treat as
                        // constructor path.
                        let full_dotted = cref_to_dotted(componentRef);
                        let mut parts = full_dotted.split('.');
                        let first = parts.next().unwrap_or("");
                        if !first.is_empty() && env.contains_key(first) {
                            let mut pat = TypedPat::Var(first.to_owned());
                            for field in parts {
                                pat = TypedPat::FieldAccess {
                                    base: Box::new(pat),
                                    field: field.to_owned(),
                                };
                            }
                            pat
                        } else {
                            let ty = lookup_ty_in_hierarchy(&full_dotted, top_level);
                            TypedPat::Constructor { name: full_dotted, fields: vec![], named_fields: vec![], ty }
                        }
                    }
                }
                _ => {
                    let dotted = cref_to_dotted(componentRef);
                    let ty = lookup_ty_in_hierarchy(&dotted, top_level);
                    TypedPat::Constructor { name: dotted, fields: vec![], named_fields: vec![], ty }
                }
            }
        }

        Absyn::Exp::CALL { function_, functionArgs, .. } => {
            let func = cref_to_dotted(function_);
            match func.as_str() {
                "SOME" => {
                    let inner = match functionArgs {
                        Absyn::FunctionArgs::FUNCTIONARGS { args, .. } => (&**args).into_iter().next()
                            .map(|a| infer_pat(a.as_ref(), env, top_level, pkg_prefix, type_vars))
                            .unwrap_or(TypedPat::Wildcard),
                        _ => TypedPat::Wildcard,
                    };
                    TypedPat::Some_(Box::new(inner))
                }
                "NONE" => TypedPat::None_,
                _ => {
                    let (fields, named_fields) = match functionArgs {
                        Absyn::FunctionArgs::FUNCTIONARGS { args, argNames } => {
                            let pos: Vec<TypedPat> = (&**args).into_iter()
                                .map(|a| infer_pat(a.as_ref(), env, top_level, pkg_prefix, type_vars))
                                .collect();
                            let named: Vec<(String, TypedPat)> = (&**argNames).into_iter()
                                .map(|na| {
                                    let Absyn::NamedArg::NAMEDARG { argName, argValue } = na.as_ref();
                                    (argName.to_string(), infer_pat(argValue.as_ref(), env, top_level, pkg_prefix, type_vars))
                                })
                                .collect();
                            (pos, named)
                        }
                        _ => (vec![], vec![]),
                    };
                    let ty = lookup_ty_in_hierarchy(&func, top_level);
                    TypedPat::Constructor { name: func, fields, named_fields, ty }
                }
            }
        }

        Absyn::Exp::TUPLE { expressions } => {
            TypedPat::Tuple((&**expressions).into_iter().map(|e| infer_pat(e.as_ref(), env, top_level, pkg_prefix, type_vars)).collect())
        }

        Absyn::Exp::ARRAY { arrayExp } => {
            // {} is the empty-list pattern; {a,b,...} builds a list via nested cons.
            let mut pats: Vec<TypedPat> = (&**arrayExp).into_iter()
                .map(|e| infer_pat(e.as_ref(), env, top_level, pkg_prefix, type_vars))
                .collect();
            if pats.is_empty() {
                TypedPat::EmptyList
            } else {
                let mut result = TypedPat::EmptyList;
                for p in pats.into_iter().rev() {
                    result = TypedPat::Cons { head: Box::new(p), tail: Box::new(result) };
                }
                result
            }
        }

        Absyn::Exp::CONS { head, rest } => {
            TypedPat::Cons {
                head: Box::new(infer_pat(head, env, top_level, pkg_prefix, type_vars)),
                tail: Box::new(infer_pat(rest, env, top_level, pkg_prefix, type_vars)),
            }
        }

        Absyn::Exp::AS { id, exp } => {
           TypedPat::As { var: id.to_string(), pat: Box::new(infer_pat(exp, env, top_level, pkg_prefix, type_vars)) }
        }

        // Negative literal in pattern position.
        Absyn::Exp::UNARY { op: Absyn::Operator::UMINUS | Absyn::Operator::UMINUS_EW, exp } => {
            match exp.as_ref() {
                Absyn::Exp::INTEGER { value } => TypedPat::Lit(Lit::Int(-value)),
                Absyn::Exp::REAL    { value } => TypedPat::Lit(Lit::Real(format!("-{value}"))),
                other => TypedPat::Todo(format!("{other:?}").chars().take(40).collect()),
            }
        }

        other => TypedPat::Todo(format!("{other:?}").chars().take(80).collect()),
    }
}

/// Collect all variable bindings introduced by a pattern, with Ty::Unknown for now.
/// Used to extend the type environment inside a match case body.
pub fn pat_bindings(pat: &TypedPat) -> Vec<(String, Ty)> {
    let mut out = Vec::new();
    collect_bindings(pat, &mut out);
    out
}

fn collect_bindings(pat: &TypedPat, out: &mut Vec<(String, Ty)>) {
    match pat {
        TypedPat::Var(name) => out.push((name.clone(), Ty::Unknown)),
        TypedPat::Some_(inner) => collect_bindings(inner, out),
        TypedPat::Cons { head, tail } => {
            collect_bindings(head, out);
            collect_bindings(tail, out);
        }
        TypedPat::Tuple(pats) => pats.iter().for_each(|p| collect_bindings(p, out)),
        TypedPat::Constructor { fields, named_fields, .. } => {
            fields.iter().for_each(|p| collect_bindings(p, out));
            named_fields.iter().for_each(|(_, p)| collect_bindings(p, out));
        }
        TypedPat::As { var, pat } => {
            out.push((var.clone(), Ty::Unknown));
            collect_bindings(pat, out);
        }
        _ => {}
    }
}

// ── Typed statement IR ────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum TypedStmt {
    /// `lhs := rhs;` — `lhs` may be any pattern (`x`, `(a,b)`, `SOME(x)`, `true`, …).
    Assign { lhs: TypedPat, rhs: TypedExp },
    /// A call statement with no return value (or value discarded).
    NoRetCall { call: TypedExp },
    If {
        cond: TypedExp,
        then_: Vec<TypedStmt>,
        elseif: Vec<(TypedExp, Vec<TypedStmt>)>,
        else_: Vec<TypedStmt>,
    },
    /// `for var in range loop body end for;` — single-iterator form only for now.
    For { var: String, range: TypedExp, body: Vec<TypedStmt> },
    While { cond: TypedExp, body: Vec<TypedStmt> },
    /// `try body else else_body end try;`
    Try { body: Vec<TypedStmt>, else_body: Vec<TypedStmt> },
    /// `failure(body)` — succeeds iff `body` fails.
    Failure { body: Vec<TypedStmt> },
    Return,
    Break,
    Continue,
    Todo(String),
}

/// Infer a list of algorithm items into typed statements, threading the env so that
/// each pattern-assign extends bindings visible to subsequent stmts.
pub fn infer_stmts<'a>(
    items: &[Absyn::AlgorithmItem],
    env: &mut HashMap<String, Ty>,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
    pkg_prefix: &str,
    type_vars: &[String],
) -> Vec<TypedStmt> {
    let mut out = Vec::new();
    for it in items {
        if let Some(s) = infer_stmt(it, env, top_level, pkg_prefix, type_vars) {
            out.push(s);
        }
    }
    out
}

fn infer_stmt<'a>(
    item: &Absyn::AlgorithmItem,
    env: &mut HashMap<String, Ty>,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
    pkg_prefix: &str,
    type_vars: &[String],
) -> Option<TypedStmt> {
    let alg = match item {
        Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_, .. } => algorithm_.as_ref(),
        Absyn::AlgorithmItem::ALGORITHMITEMCOMMENT { .. } => return None,
    };
    Some(match alg {
        Absyn::Algorithm::ALG_ASSIGN { assignComponent, value } => {
            let rhs = infer_exp(value, env, top_level, pkg_prefix, type_vars);
            // The LHS of `:=` is a pattern (in MetaModelica, patterns and expressions share syntax).
            let lhs = infer_pat(assignComponent, env, top_level, pkg_prefix, type_vars);
            // Extend env from any *new* bindings introduced by the LHS.
            // Declared locals (outputs/protected) already have their authoritative
            // type recorded in env from the function-prelude pass; we must not
            // overwrite that with `rhs.ty()`, because the RHS type can be a raw
            // function-output TypeVar (e.g. `Mutable.access<T>(Mutable<T>) -> T`
            // returns `Ty::TypeVar("T")` without per-call substitution) and we
            // would lose the local's structural type (e.g. `list<T>`), breaking
            // downstream type-directed codegen such as for-loop iteration.
            //
            // For pattern-introduced names not yet in env (e.g. `x :: rest := lst`)
            // we still need an entry. The exact type derivation from the scrutinee
            // is left to later work; insert `Ty::Unknown` so codegen at least sees
            // the binding exists without overriding declared types elsewhere.
            for (name, _ty) in pat_bindings(&lhs) {
                env.entry(name).or_insert(Ty::Unknown);
            }
            TypedStmt::Assign { lhs, rhs }
        }
        Absyn::Algorithm::ALG_NORETCALL { functionCall, functionArgs } => {
            let func = cref_to_dotted(functionCall);
            let (args, named_args) = extract_call_args(functionArgs, env, top_level, pkg_prefix, type_vars);
            let sig_ty = lookup_ty_in_hierarchy(&func, top_level);
            let resolved = resolve_call_node(&func, top_level, pkg_prefix);
            let is_constructor = match &sig_ty {
                Ty::RustStruct(_) | Ty::RustEnum(_) => true,
                _ => {
                    if let Some((_, node)) = &resolved {
                        matches!(node.kind, NodeKind::Class(ref c) if matches!(c.restriction, Absyn::Restriction::R_RECORD | Absyn::Restriction::R_UNIONTYPE))
                    } else {
                        false
                    }
                }
            };
            let call = if is_constructor {
                let canonical = resolved.as_ref().map(|(q, _)| q.clone()).unwrap_or(func.clone());
                let ty = match lookup_ty_in_hierarchy(&canonical, top_level) {
                    Ty::Function { output, .. } => *output,
                    other => other,
                };
                let field_names = match &sig_ty {
                    Ty::RustStruct(qname) | Ty::RustEnum(qname) => {
                        record_field_tys(qname, top_level).into_iter().map(|(n, _)| n).collect()
                    }
                    _ => {
                        record_field_tys(&canonical, top_level).into_iter().map(|(n, _)| n).collect()
                    }
                };
                TypedExp::Constructor { name: canonical, args, named_args, ty, field_names }
            } else {
                let ty = call_ty(&func, &args, top_level, pkg_prefix);
                TypedExp::Call { func, args, named_args, ty, sig_ty }
            };
            TypedStmt::NoRetCall { call }
        }
        Absyn::Algorithm::ALG_IF { ifExp, trueBranch, elseIfAlgorithmBranch, elseBranch } => {
            let cond = infer_exp(ifExp, env, top_level, pkg_prefix, type_vars);
            let then_ = infer_stmts_list(trueBranch, env, top_level, pkg_prefix, type_vars);
            let elseif: Vec<(TypedExp, Vec<TypedStmt>)> = (&**elseIfAlgorithmBranch).into_iter()
                .map(|(c, b)| (
                    infer_exp(c, env, top_level, pkg_prefix, type_vars),
                    infer_stmts_list(b, env, top_level, pkg_prefix, type_vars),
                ))
                .collect();
            let else_ = infer_stmts_list(elseBranch, env, top_level, pkg_prefix, type_vars);
            TypedStmt::If { cond, then_, elseif, else_ }
        }
        Absyn::Algorithm::ALG_FOR { iterators, forBody }
        | Absyn::Algorithm::ALG_PARFOR { iterators, parforBody: forBody } => {
            // Single-iterator form only.
            let iters: Vec<Absyn::ForIterator> = (&**iterators).into_iter().cloned().collect();
            if iters.len() == 1 {
                let Absyn::ForIterator::ITERATOR { name, range, .. } = &iters[0];
                let range_e = match range {
                    Some(r) => infer_exp(r.as_ref(), env, top_level, pkg_prefix, type_vars),
                    None => TypedExp::Todo("for-without-range".to_owned()),
                };
                // Element type from list/array.
                let elem_ty = match range_e.ty() {
                    Ty::List(t) | Ty::Array(t) | Ty::Range(t) => *t,
                    _ => Ty::Unknown,
                };
                let mut inner = env.clone();
                inner.insert(name.to_string(), elem_ty);
                let body = infer_stmts_list(forBody, &mut inner, top_level, pkg_prefix, type_vars);
                TypedStmt::For { var: name.to_string(), range: range_e, body }
            } else {
                TypedStmt::Todo("multi-iterator-for".to_owned())
            }
        }
        Absyn::Algorithm::ALG_WHILE { boolExpr, whileBody } => {
            let cond = infer_exp(boolExpr, env, top_level, pkg_prefix, type_vars);
            let body = infer_stmts_list(whileBody, env, top_level, pkg_prefix, type_vars);
            TypedStmt::While { cond, body }
        }
        Absyn::Algorithm::ALG_TRY { body, elseBody } => {
            let mut benv = env.clone();
            let body = infer_stmts_list(body, &mut benv, top_level, pkg_prefix, type_vars);
            let mut eenv = env.clone();
            let else_body = infer_stmts_list(elseBody, &mut eenv, top_level, pkg_prefix, type_vars);
            TypedStmt::Try { body, else_body }
        }
        Absyn::Algorithm::ALG_FAILURE { equ } => {
            let mut fenv = env.clone();
            let body = infer_stmts_list(equ, &mut fenv, top_level, pkg_prefix, type_vars);
            TypedStmt::Failure { body }
        }
        Absyn::Algorithm::ALG_RETURN   => TypedStmt::Return,
        Absyn::Algorithm::ALG_BREAK    => TypedStmt::Break,
        Absyn::Algorithm::ALG_CONTINUE => TypedStmt::Continue,
        other => TypedStmt::Todo(format!("{other:?}").chars().take(60).collect()),
    })
}

fn infer_stmts_list<'a>(
    items: &std::sync::Arc<mmwinnow::List<Absyn::AlgorithmItem>>,
    env: &mut HashMap<String, Ty>,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
    pkg_prefix: &str,
    type_vars: &[String],
) -> Vec<TypedStmt> {
    let mut out = Vec::new();
    for it in (&**items).into_iter() {
        if let Some(s) = infer_stmt(it, env, top_level, pkg_prefix, type_vars) {
            out.push(s);
        }
    }
    out
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn absyn_op_to_binop(op: &Absyn::Operator) -> BinOpKind {
    match op {
        Absyn::Operator::ADD | Absyn::Operator::ADD_EW => BinOpKind::Add,
        Absyn::Operator::SUB | Absyn::Operator::SUB_EW => BinOpKind::Sub,
        Absyn::Operator::MUL | Absyn::Operator::MUL_EW => BinOpKind::Mul,
        Absyn::Operator::DIV | Absyn::Operator::DIV_EW => BinOpKind::Div,
        Absyn::Operator::POW | Absyn::Operator::POW_EW => BinOpKind::Pow,
        Absyn::Operator::AND   => BinOpKind::And,
        Absyn::Operator::OR    => BinOpKind::Or,
        Absyn::Operator::EQUAL => BinOpKind::Eq,
        Absyn::Operator::NEQUAL   => BinOpKind::NEq,
        Absyn::Operator::LESS     => BinOpKind::Lt,
        Absyn::Operator::LESSEQ   => BinOpKind::LEq,
        Absyn::Operator::GREATER  => BinOpKind::Gt,
        Absyn::Operator::GREATEREQ => BinOpKind::GEq,
        _ => BinOpKind::Add,
    }
}
