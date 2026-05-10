#![allow(unused)]

use std::collections::HashMap;
use std::collections::BTreeMap;
use mmwinnow::Absyn;
use crate::hierarchy::{NameNode, NodeKind, Ty};

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
    Add, Sub, Mul, Div,
    And, Or,
    Eq, NEq, Lt, LEq, Gt, GEq,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnOpKind { Neg, Not }

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MatchKind { Match, MatchContinue }

// ── Typed expression IR ───────────────────────────────────────────────────────

/// One case in a match/matchcontinue expression.
#[derive(Debug, Clone)]
pub struct TypedCase {
    pub pattern: TypedPat,
    pub guard: Option<TypedExp>,
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
    /// A function/constructor call. `func` is the dotted MM name (e.g. "List.map", "SOME").
    Call { func: String, args: Vec<TypedExp>, named_args: Vec<(String, TypedExp)>, ty: Ty },
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
            TypedExp::If     { ty, .. }  => ty.clone(),
            TypedExp::Cons   { ty, .. }  => ty.clone(),
            TypedExp::Array  { ty, .. }  => ty.clone(),
            TypedExp::Match  { ty, .. }  => ty.clone(),
            TypedExp::Range  { elem_ty, .. } => Ty::List(Box::new(elem_ty.clone())),
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
    Todo(String),
}

// ── Inference ─────────────────────────────────────────────────────────────────

/// Convert a ComponentRef to a dotted MetaModelica name (e.g. "List.map").
/// Deprecated: loses subscripts and structure. Use `extract_cref_segments` instead.
pub fn cref_to_dotted(cref: &Absyn::ComponentRef) -> String {
    match cref {
        Absyn::ComponentRef::CREF_IDENT { name, .. } => name.clone(),
        Absyn::ComponentRef::CREF_QUAL { name, componentRef, .. } => {
            format!("{name}.{}", cref_to_dotted(componentRef))
        }
        Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef } => cref_to_dotted(componentRef),
        Absyn::ComponentRef::WILD | Absyn::ComponentRef::ALLWILD => "_".to_owned(),
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
    // Collect segments in reverse (tail-first) then reverse at the end.
    let mut segs: Vec<CrefSegment> = Vec::new();
    collect_cref_segments_rev(cref, env, top_level, pkg_prefix, &mut segs);
    segs.reverse();

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
            let subs: Vec<TypedExp> = subscripts.into_iter()
                .filter_map(|s| {
                    if let Absyn::Subscript::SUBSCRIPT { subscript } = s.as_ref() {
                        Some(infer_exp(subscript, env, top_level, pkg_prefix))
                    } else {
                        None
                    }
                })
                .collect();
            acc.push(CrefSegment { name: name.clone(), subscripts: subs });
        }
        Absyn::ComponentRef::CREF_QUAL { name, subscripts, componentRef } => {
            let subs: Vec<TypedExp> = subscripts.into_iter()
                .filter_map(|s| {
                    if let Absyn::Subscript::SUBSCRIPT { subscript } = s.as_ref() {
                        Some(infer_exp(subscript, env, top_level, pkg_prefix))
                    } else {
                        None
                    }
                })
                .collect();
            acc.push(CrefSegment { name: name.clone(), subscripts: subs });
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

fn binop_ty(op: BinOpKind, lhs_ty: &Ty, rhs_ty: &Ty) -> Ty {
    match op {
        BinOpKind::Add | BinOpKind::Sub | BinOpKind::Mul | BinOpKind::Div => {
            match (lhs_ty, rhs_ty) {
                (Ty::F64, _) | (_, Ty::F64) => Ty::F64,
                (Ty::I32, _) | (_, Ty::I32) => Ty::I32,
                _ => lhs_ty.clone(),
            }
        }
        BinOpKind::And | BinOpKind::Or
        | BinOpKind::Eq | BinOpKind::NEq
        | BinOpKind::Lt | BinOpKind::LEq
        | BinOpKind::Gt | BinOpKind::GEq => Ty::Bool,
    }
}

fn call_ty(func: &str, args: &[TypedExp], top_level: &BTreeMap<String, NameNode<'_>>) -> Ty {
    match func {
        "SOME" => Ty::Option(Box::new(args.first().map(|a| a.ty()).unwrap_or(Ty::Unknown))),
        "NONE" => Ty::Option(Box::new(Ty::Unknown)),
        "fail" => Ty::Unknown,
        "intAdd" | "intSub" | "intMul" | "intDiv" | "intMod" | "intAbs"
        | "intMax" | "intMin" | "intNeg" | "intBitAnd" | "intBitOr" | "intBitXor"
        | "intBitLShift" | "intBitRShift" | "intFromChar" | "stringLength"
        | "stringCompare" | "stringHash" | "stringHashDjb2" => Ty::I32,
        "realAdd" | "realSub" | "realMul" | "realDiv" | "realAbs"
        | "realMax" | "realMin" | "realNeg" | "realFloor" | "realCeil" => Ty::F64,
        "intString" | "realString" | "boolString" | "anyString"
        | "stringAppend" | "stringCharAt" | "stringGetStringChar" => Ty::Str,
        "stringEqual" | "stringEq" | "intEq" | "intLt" | "intLe" | "intGt" | "intGe"
        | "intNe" | "realEq" | "realLt" | "realLe" | "realGt" | "realGe"
        | "boolAnd" | "boolOr" | "boolNot" | "boolEq"
        | "referenceEq" | "valueEq" | "isEmpty" | "isSome" | "isNone" => Ty::Bool,
        "listHead" | "listFirst" => {
            match args.first().map(|a| a.ty()) {
                Some(Ty::List(inner)) => *inner,
                _ => Ty::Unknown,
            }
        }
        "listRest" | "listTail" | "listReverse" | "listAppend" => {
            args.first().map(|a| a.ty()).unwrap_or(Ty::Unknown)
        }
        _ => {
            match lookup_ty_in_hierarchy(func, top_level) {
                Ty::Function { output, .. } => *output,
                other => other,
            }
        }
    }
}

/// Infer the type of a MetaModelica expression, building a typed expression tree.
/// `env` maps local variable names to their resolved types.
pub fn infer_exp<'a>(
    exp: &Absyn::Exp,
    env: &HashMap<String, Ty>,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
    pkg_prefix: &str,
) -> TypedExp {
    match exp {
        Absyn::Exp::INTEGER { value } => TypedExp::Lit(Lit::Int(*value)),
        Absyn::Exp::REAL    { value } => TypedExp::Lit(Lit::Real(value.clone())),
        Absyn::Exp::STRING  { value } => TypedExp::Lit(Lit::Str(value.clone())),
        Absyn::Exp::BOOL    { value } => TypedExp::Lit(Lit::Bool(*value)),

        Absyn::Exp::CREF { componentRef } => {
            let (name, segments) = extract_cref_segments(componentRef, env, top_level, pkg_prefix);
            // Local env takes priority; fall back to hierarchy, then try qualifying
            // the bare name with the enclosing package prefix (for sibling references).
            let ty = env.get(&name).cloned().unwrap_or_else(|| {
                let ty = lookup_ty_in_hierarchy(&name, top_level);
                if ty == Ty::Unknown && !pkg_prefix.is_empty() && !name.contains('.') {
                    lookup_ty_in_hierarchy(&format!("{pkg_prefix}.{name}"), top_level)
                } else {
                    ty
                }
            });
            TypedExp::Var { name, segments, ty }
        }

        Absyn::Exp::BINARY  { exp1, op, exp2 }
        | Absyn::Exp::LBINARY  { exp1, op, exp2 }
        | Absyn::Exp::RELATION { exp1, op, exp2 } => {
            let lhs = infer_exp(exp1, env, top_level, pkg_prefix);
            let rhs = infer_exp(exp2, env, top_level, pkg_prefix);
            let bin_op = absyn_op_to_binop(op);
            let ty = binop_ty(bin_op, &lhs.ty(), &rhs.ty());
            TypedExp::BinOp { op: bin_op, lhs: Box::new(lhs), rhs: Box::new(rhs), ty }
        }

        Absyn::Exp::UNARY { op, exp } => {
            let operand = infer_exp(exp, env, top_level, pkg_prefix);
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
            let operand = infer_exp(exp, env, top_level, pkg_prefix);
            // Fold `not true` → `Lit(false)`, `not false` → `Lit(true)`.
            if let TypedExp::Lit(Lit::Bool(v)) = &operand {
                TypedExp::Lit(Lit::Bool(!v))
            } else {
                TypedExp::UnOp { op: UnOpKind::Not, operand: Box::new(operand), ty: Ty::Bool }
            }
        }

        Absyn::Exp::IFEXP { ifExp, trueBranch, elseBranch, elseIfBranch } => {
            let cond  = infer_exp(ifExp, env, top_level, pkg_prefix);
            let then_ = infer_exp(trueBranch, env, top_level, pkg_prefix);
            let else_ = infer_exp(elseBranch, env, top_level, pkg_prefix);
            let elseif: Vec<(TypedExp, TypedExp)> = elseIfBranch.into_iter()
                .map(|(c, b)| (infer_exp(&c, env, top_level, pkg_prefix), infer_exp(&b, env, top_level, pkg_prefix)))
                .collect();
            let ty = if then_.ty() != Ty::Unknown { then_.ty() } else { else_.ty() };
            TypedExp::If { cond: Box::new(cond), then_: Box::new(then_), elseif, else_: Box::new(else_), ty }
        }

        Absyn::Exp::CALL { function_, functionArgs, .. } => {
            let func = cref_to_dotted(function_);
            let (args, named_args) = extract_call_args(functionArgs, env, top_level, pkg_prefix);
            let ty = call_ty(&func, &args, top_level);
            TypedExp::Call { func, args, named_args, ty }
        }

        Absyn::Exp::TUPLE { expressions } => {
            let elems: Vec<TypedExp> = expressions.into_iter()
                .map(|e| infer_exp(e.as_ref(), env, top_level, pkg_prefix))
                .collect();
            TypedExp::Tuple(elems)
        }

        Absyn::Exp::ARRAY { arrayExp } => {
            let elems: Vec<TypedExp> = arrayExp.into_iter()
                .map(|e| infer_exp(e.as_ref(), env, top_level, pkg_prefix))
                .collect();
            let inner_ty = elems.first().map(|e| e.ty()).unwrap_or(Ty::Unknown);
            TypedExp::Array { elems, ty: Ty::List(Box::new(inner_ty)) }
        }

        Absyn::Exp::CONS { head, rest } => {
            let head_e = infer_exp(head, env, top_level, pkg_prefix);
            let tail_e = infer_exp(rest, env, top_level, pkg_prefix);
            let ty = tail_e.ty();
            TypedExp::Cons { head: Box::new(head_e), tail: Box::new(tail_e), ty }
        }

        Absyn::Exp::MATCHEXP { matchTy, inputExp, cases, .. } => {
            let input = infer_exp(inputExp, env, top_level, pkg_prefix);
            let kind = match matchTy {
                Absyn::MatchType::MATCH => MatchKind::Match,
                Absyn::MatchType::MATCHCONTINUE => MatchKind::MatchContinue,
            };
            let typed_cases: Vec<TypedCase> = cases.into_iter()
                .map(|c| infer_case(&c, env, top_level, pkg_prefix))
                .collect();
            let ty = typed_cases.iter()
                .map(|c| c.result.ty())
                .find(|t| *t != Ty::Unknown)
                .unwrap_or(Ty::Unknown);
            TypedExp::Match { kind, input: Box::new(input), cases: typed_cases, ty }
        }

        Absyn::Exp::RANGE { start, step, stop } => {
            let start_e = infer_exp(start, env, top_level, pkg_prefix);
            let step_e = step.as_ref().map(|s| infer_exp(s, env, top_level, pkg_prefix));
            let stop_e = infer_exp(stop, env, top_level, pkg_prefix);
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
) -> (Vec<TypedExp>, Vec<(String, TypedExp)>) {
    match function_args {
        Absyn::FunctionArgs::FUNCTIONARGS { args, argNames } => {
            let pos: Vec<TypedExp> = args.into_iter()
                .map(|a| infer_exp(a.as_ref(), env, top_level, pkg_prefix))
                .collect();
            let named: Vec<(String, TypedExp)> = argNames.into_iter()
                .map(|na| {
                    let Absyn::NamedArg::NAMEDARG { argName, argValue } = na.as_ref();
                    (argName.clone(), infer_exp(argValue, env, top_level, pkg_prefix))
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
) -> TypedCase {
    match case {
        Absyn::Case::CASE { pattern, patternGuard, result, .. } => {
            let pat = infer_pat(pattern, top_level);
            let mut inner_env = env.clone();
            inner_env.extend(pat_bindings(&pat));
            let guard = patternGuard.as_ref().map(|g| infer_exp(g, &inner_env, top_level, pkg_prefix));
            TypedCase { pattern: pat, guard, result: infer_exp(result, &inner_env, top_level, pkg_prefix) }
        }
        Absyn::Case::ELSE { result, .. } => {
            TypedCase { pattern: TypedPat::Wildcard, guard: None, result: infer_exp(result, env, top_level, pkg_prefix) }
        }
    }
}

/// Infer the pattern from an expression in case-pattern position.
pub fn infer_pat(exp: &Absyn::Exp, top_level: &BTreeMap<String, NameNode<'_>>) -> TypedPat {
    match exp {
        Absyn::Exp::INTEGER { value } => TypedPat::Lit(Lit::Int(*value)),
        Absyn::Exp::REAL    { value } => TypedPat::Lit(Lit::Real(value.clone())),
        Absyn::Exp::STRING  { value } => TypedPat::Lit(Lit::Str(value.clone())),
        Absyn::Exp::BOOL    { value } => TypedPat::Lit(Lit::Bool(*value)),

        Absyn::Exp::CREF { componentRef } => {
            match componentRef.as_ref() {
                Absyn::ComponentRef::WILD | Absyn::ComponentRef::ALLWILD => TypedPat::Wildcard,
                Absyn::ComponentRef::CREF_IDENT { name, subscripts } if subscripts.is_empty() => {
                    if name == "_" {
                        TypedPat::Wildcard
                    } else if name.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                        // Uppercase: look up in hierarchy to distinguish unit-variant from binding.
                        let ty = lookup_ty_in_hierarchy(name, top_level);
                        match &ty {
                            Ty::RustUnitVariant | Ty::UnionTypeVariant(..) => {
                                TypedPat::Constructor { name: name.clone(), fields: vec![], named_fields: vec![], ty }
                            }
                            _ => TypedPat::Var(name.clone()),
                        }
                    } else {
                        TypedPat::Var(name.clone())
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
                        Absyn::FunctionArgs::FUNCTIONARGS { args, .. } => args.into_iter().next()
                            .map(|a| infer_pat(a.as_ref(), top_level))
                            .unwrap_or(TypedPat::Wildcard),
                        _ => TypedPat::Wildcard,
                    };
                    TypedPat::Some_(Box::new(inner))
                }
                "NONE" => TypedPat::None_,
                _ => {
                    let (fields, named_fields) = match functionArgs {
                        Absyn::FunctionArgs::FUNCTIONARGS { args, argNames } => {
                            let pos: Vec<TypedPat> = args.into_iter()
                                .map(|a| infer_pat(a.as_ref(), top_level))
                                .collect();
                            let named: Vec<(String, TypedPat)> = argNames.into_iter()
                                .map(|na| {
                                    let Absyn::NamedArg::NAMEDARG { argName, argValue } = na.as_ref();
                                    (argName.clone(), infer_pat(argValue, top_level))
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
            TypedPat::Tuple(expressions.into_iter().map(|e| infer_pat(e.as_ref(), top_level)).collect())
        }

        Absyn::Exp::ARRAY { arrayExp } => {
            // {} is the empty-list pattern; {a,b,...} builds a list via nested cons.
            let mut pats: Vec<TypedPat> = arrayExp.into_iter()
                .map(|e| infer_pat(e.as_ref(), top_level))
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
                head: Box::new(infer_pat(head, top_level)),
                tail: Box::new(infer_pat(rest, top_level)),
            }
        }

        Absyn::Exp::AS { id, exp } => {
            TypedPat::As { var: id.clone(), pat: Box::new(infer_pat(exp, top_level)) }
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
) -> Vec<TypedStmt> {
    let mut out = Vec::new();
    for it in items {
        if let Some(s) = infer_stmt(it, env, top_level, pkg_prefix) {
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
) -> Option<TypedStmt> {
    let alg = match item {
        Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_, .. } => algorithm_.as_ref(),
        Absyn::AlgorithmItem::ALGORITHMITEMCOMMENT { .. } => return None,
    };
    Some(match alg {
        Absyn::Algorithm::ALG_ASSIGN { assignComponent, value } => {
            let rhs = infer_exp(value, env, top_level, pkg_prefix);
            // The LHS of `:=` is a pattern (in MetaModelica, patterns and expressions share syntax).
            let lhs = infer_pat(assignComponent, top_level);
            // Extend env from any new bindings introduced by the LHS.
            for (name, _ty) in pat_bindings(&lhs) {
                // For now we don't know the binding type accurately without scrutinee threading;
                // record `Unknown`. The codegen can still emit the binding; fancier coercion later.
                env.insert(name, rhs.ty());
            }
            TypedStmt::Assign { lhs, rhs }
        }
        Absyn::Algorithm::ALG_NORETCALL { functionCall, functionArgs } => {
            let func = cref_to_dotted(functionCall);
            let (args, named_args) = extract_call_args(functionArgs, env, top_level, pkg_prefix);
            let ty = call_ty(&func, &args, top_level);
            TypedStmt::NoRetCall { call: TypedExp::Call { func, args, named_args, ty } }
        }
        Absyn::Algorithm::ALG_IF { ifExp, trueBranch, elseIfAlgorithmBranch, elseBranch } => {
            let cond = infer_exp(ifExp, env, top_level, pkg_prefix);
            let then_ = infer_stmts_list(trueBranch, env, top_level, pkg_prefix);
            let elseif: Vec<(TypedExp, Vec<TypedStmt>)> = elseIfAlgorithmBranch.into_iter()
                .map(|(c, b)| (
                    infer_exp(&c, env, top_level, pkg_prefix),
                    infer_stmts_list(&b, env, top_level, pkg_prefix),
                ))
                .collect();
            let else_ = infer_stmts_list(elseBranch, env, top_level, pkg_prefix);
            TypedStmt::If { cond, then_, elseif, else_ }
        }
        Absyn::Algorithm::ALG_FOR { iterators, forBody }
        | Absyn::Algorithm::ALG_PARFOR { iterators, parforBody: forBody } => {
            // Single-iterator form only.
            let iters: Vec<Absyn::ForIterator> = iterators.into_iter().collect();
            if iters.len() == 1 {
                let Absyn::ForIterator::ITERATOR { name, range, .. } = &iters[0];
                let range_e = match range {
                    Some(r) => infer_exp(r, env, top_level, pkg_prefix),
                    None => TypedExp::Todo("for-without-range".to_owned()),
                };
                // Element type from list/array.
                let elem_ty = match range_e.ty() {
                    Ty::List(t) | Ty::Array(t) => *t,
                    _ => Ty::Unknown,
                };
                let mut inner = env.clone();
                inner.insert(name.clone(), elem_ty);
                let body = infer_stmts_list(forBody, &mut inner, top_level, pkg_prefix);
                TypedStmt::For { var: name.clone(), range: range_e, body }
            } else {
                TypedStmt::Todo("multi-iterator-for".to_owned())
            }
        }
        Absyn::Algorithm::ALG_WHILE { boolExpr, whileBody } => {
            let cond = infer_exp(boolExpr, env, top_level, pkg_prefix);
            let body = infer_stmts_list(whileBody, env, top_level, pkg_prefix);
            TypedStmt::While { cond, body }
        }
        Absyn::Algorithm::ALG_TRY { body, elseBody } => {
            let mut benv = env.clone();
            let body = infer_stmts_list(body, &mut benv, top_level, pkg_prefix);
            let mut eenv = env.clone();
            let else_body = infer_stmts_list(elseBody, &mut eenv, top_level, pkg_prefix);
            TypedStmt::Try { body, else_body }
        }
        Absyn::Algorithm::ALG_FAILURE { equ } => {
            let mut fenv = env.clone();
            let body = infer_stmts_list(equ, &mut fenv, top_level, pkg_prefix);
            TypedStmt::Failure { body }
        }
        Absyn::Algorithm::ALG_RETURN   => TypedStmt::Return,
        Absyn::Algorithm::ALG_BREAK    => TypedStmt::Break,
        Absyn::Algorithm::ALG_CONTINUE => TypedStmt::Continue,
        other => TypedStmt::Todo(format!("{other:?}").chars().take(60).collect()),
    })
}

fn infer_stmts_list<'a>(
    items: &mmwinnow::List<Absyn::AlgorithmItem>,
    env: &mut HashMap<String, Ty>,
    top_level: &'a BTreeMap<String, NameNode<'a>>,
    pkg_prefix: &str,
) -> Vec<TypedStmt> {
    let mut out = Vec::new();
    for it in items.into_iter() {
        if let Some(s) = infer_stmt(&it, env, top_level, pkg_prefix) {
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
