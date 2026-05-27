// Auto-generated from MetaModelica source
/*
 * This file is part of OpenModelica.
 *
 * Copyright (c) 1998-2026, Open Source Modelica Consortium (OSMC),
 * c/o Linköpings universitet, Department of Computer and Information Science,
 * SE-58183 Linköping, Sweden.
 *
 * All rights reserved.
 *
 * THIS PROGRAM IS PROVIDED UNDER THE TERMS OF AGPL VERSION 3 LICENSE OR
 * THIS OSMC PUBLIC LICENSE (OSMC-PL) VERSION 1.8.
 * ANY USE, REPRODUCTION OR DISTRIBUTION OF THIS PROGRAM CONSTITUTES
 * RECIPIENT'S ACCEPTANCE OF THE OSMC PUBLIC LICENSE OR THE GNU AGPL
 * VERSION 3, ACCORDING TO RECIPIENTS CHOICE.
 *
 * The OpenModelica software and the OSMC (Open Source Modelica Consortium)
 * Public License (OSMC-PL) are obtained from OSMC, either from the above
 * address, from the URLs:
 * http://www.openmodelica.org or
 * https://github.com/OpenModelica/ or
 * http://www.ida.liu.se/projects/OpenModelica,
 * and in the OpenModelica distribution.
 *
 * GNU AGPL version 3 is obtained from:
 * https://www.gnu.org/licenses/licenses.html#GPL
 *
 * This program is distributed WITHOUT ANY WARRANTY; without
 * even the implied warranty of MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE, EXCEPT AS EXPRESSLY SET FORTH
 * IN THE BY RECIPIENT SELECTED SUBSIDIARY LICENSE CONDITIONS OF OSMC-PL.
 *
 * See the full OSMC Public License conditions for more details.
 *
 */
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::ComponentReference;
use crate::DAEUtil;
use crate::Expression;
use crate::ExpressionDump;
use crate::ExpressionSimplifyTypes;
use crate::FCore;
use crate::FGraph;
use crate::Static;
use crate::Types;
use crate::ValuesUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::Values;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

// public imports
pub type ComponentRef = Arc<DAE::ComponentRef>;

pub type Ident = ArcStr;

pub type Operator = DAE::Operator;

pub type Type = Arc<DAE::Type>;

pub type Subscript = Arc<DAE::Subscript>;

// protected imports
pub static optionSimplifyOnly: std::sync::LazyLock<ExpressionSimplifyTypes::Evaluate> = std::sync::LazyLock::new(|| { ExpressionSimplifyTypes::optionSimplifyOnly.clone() });

pub fn simplify(mut inExp: Arc<DAE::Exp>) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut hasChanged: bool = false;
    (outExp, hasChanged) = simplifyWithOptions(inExp.clone(), optionSimplifyOnly.clone())?;
    Ok((outExp, hasChanged))
}

pub fn condsimplify(mut cond: bool, mut ioExp: Arc<DAE::Exp>) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut ioExp: Arc<DAE::Exp> = ioExp;
    let mut hasChanged: bool = false;
    if cond.clone() {
        (ioExp, hasChanged) = simplifyWithOptions(ioExp.clone(), optionSimplifyOnly.clone())?;
    }
    Ok((ioExp, hasChanged))
}

pub fn simplifyBinaryExp(mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::BINARY { exp2: e2, operator: op, exp1: e1 } => {
            simplifyBinary(inExp.clone(), op.clone(), e1.clone(), e2.clone())?
        },
        _ => {
            inExp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn simplifyUnaryExp(mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::UNARY { operator: op, exp: e1 } => {
            simplifyUnary(inExp.clone(), op.clone(), e1.clone())?
        },
        _ => {
            inExp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

fn simplifyWithOptions(mut inExp: Arc<DAE::Exp>, mut options: ExpressionSimplifyTypes::Evaluate) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut hasChanged: bool = false;
    (outExp, hasChanged) = 'mc: {
        let __mc_input = (inExp.clone(), options.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, ExpressionSimplifyTypes::Evaluate::DO_EVAL) => {
                    let mut eNew: Arc<DAE::Exp>;
                    let mut b: bool = false;
                    (eNew, _) = simplify1WithOptions(e.clone(), options.clone())?;
                    Error::assertionOrAddSourceMessage(Expression::isConstValue(eNew.clone())?, Error::INTERNAL_ERROR.clone(), list![(literal!("eval exp failed")).clone()], Absyn::dummyInfo.clone())?;
                    b = !(ExpressionBasics::expEqual(e.clone(), eNew.clone())?);
                    Ok((eNew.clone(), b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, _) => {
                    let mut eNew: Arc<DAE::Exp>;
                    let mut b: bool = false;
                    let false = (Config::getNoSimplify()?) else { bail!("pattern mismatch") };
                    (eNew, _) = simplify1WithOptions(e.clone(), options.clone())?;
                    eNew = simplify2(eNew.clone(), true, true)?;
                    (eNew, _) = simplify1WithOptions(eNew.clone(), options.clone())?;
                    b = !(ExpressionBasics::expEqual(e.clone(), eNew.clone())?);
                    Ok((eNew.clone(), b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, _) => {
                    let mut eNew: Arc<DAE::Exp>;
                    let mut b: bool = false;
                    (eNew, b) = simplify1WithOptions(e.clone(), options.clone())?;
                    Ok((eNew.clone(), b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, hasChanged))
}

pub fn simplifyTraverseHelper<A: Clone + 'static>(mut inExp: Arc<DAE::Exp>, mut inA: A) -> Result<(Arc<DAE::Exp>, A)> {
    let mut exp: Arc<DAE::Exp>;
    let mut a: A;
    a = inA.clone();
    (exp, _) = simplify(inExp.clone())?;
    Ok((exp, a))
}

pub fn simplify1TraverseHelper<A: Clone + 'static>(mut inExp: Arc<DAE::Exp>, mut inA: A) -> Result<(Arc<DAE::Exp>, A)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut a: A;
    a = inA.clone();
    (outExp, _) = simplify1(inExp.clone())?;
    Ok((outExp, a))
}

pub fn simplify1time(mut e: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outE: Arc<DAE::Exp>;
    let mut t1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut t2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    t1 = clock();
    (outE, _) = simplify1(e.clone())?;
    t2 = clock();
    println!("{}", (if (t2.clone() - t1.clone() > metamodelica::OrderedFloat(0.01_f64)) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("simplify1 took ")); __mm_s.push_str(&*realString(t2.clone() - t1.clone())); __mm_s.push_str(&*literal!(" seconds for exp: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!(" \nsimplified to :")); __mm_s.push_str(&*ExpressionBasics::printExpStr(outE.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }} else {literal!("")}).clone());
    Ok(outE)
}

pub fn simplifyWork(mut inExp: Arc<DAE::Exp>, mut options: ExpressionSimplifyTypes::Evaluate) -> Result<(Arc<DAE::Exp>, ExpressionSimplifyTypes::Evaluate)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outOptions: ExpressionSimplifyTypes::Evaluate = ExpressionSimplifyTypes::Evaluate::DO_EVAL;
    (outExp, outOptions) = (::match_deref::match_deref! { match &((inExp.clone(), options.clone())) {
        (Deref @ DAE::Exp::SIZE { sz: oe, exp: e1 }, _) => {
            (simplifySize(inExp.clone(), e1.clone(), oe.clone())?, options.clone())
        },
        (Deref @ DAE::Exp::CAST { exp: e, ty: tp }, _) => {
            let mut e = (*e).clone();
            e = simplifyCast(inExp.clone(), e.clone(), tp.clone())?;
            (e.clone(), options.clone())
        },
        (Deref @ DAE::Exp::ASUB { sub: subs, exp: e }, _) => {
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut e = (*e).clone();
            expl = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut sub in (subs.clone()).into_iter().cloned() {
            let __x = Expression::getSubscriptExp(sub.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            e = simplifyAsubExp(inExp.clone(), e.clone(), expl.clone())?;
            (e.clone(), options.clone())
        },
        (Deref @ DAE::Exp::TSUB { .. }, _) => {
            (simplifyTSub(inExp.clone()), options.clone())
        },
        (Deref @ DAE::Exp::UNARY { exp: e1, operator: op }, _) => {
            let mut e: Arc<DAE::Exp>;
            e = simplifyUnary(inExp.clone(), op.clone(), e1.clone())?;
            (e.clone(), options.clone())
        },
        (Deref @ DAE::Exp::BINARY { exp2: e2, operator: op, exp1: e1 }, _) => {
            let mut e: Arc<DAE::Exp>;
            e = simplifyBinary(inExp.clone(), op.clone(), e1.clone(), e2.clone())?;
            (e.clone(), options.clone())
        },
        (Deref @ DAE::Exp::RELATION { optionExpisASUB: isExpisASUB, index: index_, exp2: e2, operator: op, exp1: e1 }, _) => {
            let mut e: Arc<DAE::Exp>;
            e = simplifyRelation(inExp.clone(), op.clone(), e1.clone(), e2.clone(), index_.clone(), isExpisASUB.clone())?;
            (e.clone(), options.clone())
        },
        (Deref @ DAE::Exp::LUNARY { exp: e1, operator: op }, _) => {
            let mut e: Arc<DAE::Exp>;
            e = simplifyUnary(inExp.clone(), op.clone(), e1.clone())?;
            (e.clone(), options.clone())
        },
        (Deref @ DAE::Exp::LBINARY { exp2: e2, operator: op, exp1: e1 }, _) => {
            let mut e: Arc<DAE::Exp>;
            e = simplifyLBinary(inExp.clone(), op.clone(), e1.clone(), e2.clone())?;
            (e.clone(), options.clone())
        },
        (Deref @ DAE::Exp::IFEXP { expElse: e3, expThen: e2, expCond: e1 }, _) => {
            let mut e: Arc<DAE::Exp>;
            e = simplifyIfExp(inExp.clone(), e1.clone(), e2.clone(), e3.clone())?;
            (e.clone(), options.clone())
        },
        (Deref @ DAE::Exp::CREF { ty: t, componentRef: c_1 }, _) => {
            let mut e: Arc<DAE::Exp>;
            e = simplifyCref(inExp.clone(), c_1.clone(), t.clone())?;
            (e.clone(), options.clone())
        },
        (Deref @ DAE::Exp::REDUCTION { reductionInfo, expr: e1, iterators: riters }, _) => {
            let mut exp1: Arc<DAE::Exp>;
            let mut b2: bool = false;
            let mut riters = (*riters).clone();
            (riters, b2) = simplifyReductionIterators(riters.clone(), metamodelica::nil(), false)?;
            exp1 = if (b2.clone()) {Arc::new(DAE::Exp::REDUCTION { reductionInfo: reductionInfo.clone(), expr: e1.clone(), iterators: riters.clone() })} else {inExp.clone()};
            (simplifyReduction(exp1.clone())?, options.clone())
        },
        (Deref @ DAE::Exp::CALL { .. }, _) => {
            (simplifyCall(inExp.clone())?, options.clone())
        },
        (Deref @ DAE::Exp::RSUB { .. }, _) => {
            (simplifyRSub(inExp.clone())?, options.clone())
        },
        (Deref @ DAE::Exp::MATCHEXPRESSION { .. }, _) => {
            (simplifyMatch(inExp.clone()), options.clone())
        },
        (Deref @ DAE::Exp::UNBOX { .. }, _) => {
            (simplifyUnbox(inExp.clone()), options.clone())
        },
        (Deref @ DAE::Exp::BOX { .. }, _) => {
            (simplifyUnbox(inExp.clone()), options.clone())
        },
        (Deref @ DAE::Exp::CONS { .. }, _) => {
            (simplifyCons(inExp.clone()), options.clone())
        },
        _ => {
            (inExp.clone(), options.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outOptions))
}

fn simplifyRSub(mut e: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut e: Arc<DAE::Exp> = e;
    e = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Exp::RSUB { ix: (-1), exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, .. } => {
            Arc::new(DAE::Exp::CREF { componentRef: ComponentReference::joinCrefs(cr.clone(), ComponentReferenceBasics::makeCrefIdent((var_field!((*e).fieldName, DAE::Exp::RSUB).clone()).clone(), var_field!((*e).ty, DAE::Exp::RSUB).clone(), metamodelica::nil()))?, ty: var_field!((*e).ty, DAE::Exp::RSUB).clone() })
        },
        Deref @ DAE::Exp::RSUB { ix: (-1), exp: Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_COMPLEX { varLst: vars, complexClassType: ClassInf::State::RECORD { path: p2 }, .. }, .. }, expLst: exps, path: p1 }, .. } if (AbsynUtil::pathEqual(p1.clone(), p2.clone())) => {
            (exps.clone()).get(List::position1OnTrue({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut v in (vars.clone()).into_iter().cloned() {
            let __x = v.name.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), (var_field!((*e).fieldName, DAE::Exp::RSUB).clone()).clone()))?
        },
        Deref @ DAE::Exp::RSUB { ix: (-1), exp: Deref @ DAE::Exp::RECORD { comp, exps, .. }, .. } => {
            (exps.clone()).get(List::position1OnTrue(comp.clone(), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), (var_field!((*e).fieldName, DAE::Exp::RSUB).clone()).clone()))?
        },
        _ => {
            e.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(e)
}

fn simplifyAsubExp(mut origExp: Arc<DAE::Exp>, mut inExp: Arc<DAE::Exp>, mut inSubs: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (origExp.clone(), inExp.clone(), inSubs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, e, Deref @ metamodelica::List::Nil) => {
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::CAST { ty: tp, exp: e }, _) => {
                    let mut tp = (*tp).clone();
                    let mut e = (*e).clone();
                    tp = Expression::unliftArray(tp.clone())?;
                    e = Arc::new(DAE::Exp::CAST { ty: tp.clone(), exp: Arc::new(DAE::Exp::ASUB { exp: e.clone(), sub: {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
        for mut s in (inSubs.clone()).into_iter().cloned() {
                    let __x = Expression::makeIndexSubscript(s.clone());
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } }) });
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::TUPLE { PR: eLst }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: sub }, tail: Deref @ metamodelica::List::Nil }) => {
                    if !((sub.clone() <= (eLst.clone().len() as i32))) { bail!("guard") }
                    Ok((eLst.clone()).get(sub.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    Ok(simplifyAsubSlicing(inExp.clone(), inSubs.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    for mut exp in &*inSubs.clone() {
                        let mut exp = exp.clone();
                        Expression::expInt(exp.clone())?;
                    }
                    Ok(List::foldr(inSubs.clone(), (std::sync::Arc::new(simplifyAsub) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), inExp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut istart: i32 = 0;
                    let mut istep: i32 = 0;
                    let mut istop: i32 = 0;
                    let mut e: Arc<DAE::Exp>;
                    let mut subs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut hasRange: bool = false;
                    let mut step: Option<Arc<DAE::Exp>> = None;
                    hasRange = false;
                    subs = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut exp in (inSubs.clone()).into_iter().cloned() {
                    let __x = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::RANGE { stop: Deref @ DAE::Exp::ICONST { integer: istop }, step, start: Deref @ DAE::Exp::ICONST { integer: istart }, .. } => {
                    e = Expression::makeArray({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut i in (simplifyRange(istart.clone(), (::match_deref::match_deref! { match &(step.clone()) {
        None => 1,
        Some(Deref @ DAE::Exp::ICONST { integer: istep }) => istep.clone(),
        _ => bail!("match: no arm matched"),
    } }), istop.clone())?).into_iter().cloned() {
                    let __x = Arc::new(DAE::Exp::ICONST { integer: i.clone() });
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, DAE::T_INTEGER_DEFAULT().clone(), true);
                    hasRange = true;
                    e.clone()
        },
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    let true = (hasRange.clone()) else { bail!("pattern mismatch") };
                    Ok(Arc::new(DAE::Exp::ASUB { exp: inExp.clone(), sub: {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
        for mut s in (subs.clone()).into_iter().cloned() {
            let __x = Expression::makeIndexSubscript(s.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(origExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

fn simplifyCall(mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: tp, .. }, expLst: Deref @ metamodelica::List::Cons { head: e @ Deref @ DAE::Exp::REDUCTION { iterators: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, reductionInfo: ri @ Deref @ DAE::ReductionInfo { path: Deref @ Absyn::Path::IDENT { name: Deref @ "array" }, .. }, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name } } => {
                    if !((listMember((name.clone()).clone(), list![(literal!("sum")).clone(), (literal!("product")).clone(), (literal!("min")).clone(), (literal!("max")).clone()]))) { bail!("guard") }
                    let mut e = (*e).clone();
                    todo!("unhandled field-assign shape: e.reductionInfo");
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "homotopy" }, .. } => {
                    if !((ExpressionBasics::expEqual(e1.clone(), e2.clone())?)) { bail!("guard") }
                    Ok(e1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "noEvent" }, .. } => {
                    let mut b2: bool = false;
                    b2 = Expression::isRelation(e.clone()) || Expression::isEventTriggeringFunctionExp(e.clone());
                    Ok(if (!(b2.clone())) {simplifyNoEvent(e.clone())?} else {inExp.clone()})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp }, exp: e1 @ Deref @ DAE::Exp::CREF { .. } }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" } } => {
                    Ok(Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp.clone() }, exp: Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("der")).clone() }), expLst: list![e1.clone()], attr: attr.clone() }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: tp }, exp: e1 @ Deref @ DAE::Exp::CREF { .. } }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" } } => {
                    Ok(Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: tp.clone() }, exp: Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("der")).clone() }), expLst: list![e1.clone()], attr: attr.clone() }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. } => {
                    Ok(inExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. } => {
                    Ok(inExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "change" }, .. } => {
                    Ok(inExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "edge" }, .. } => {
                    Ok(inExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e @ Deref @ DAE::Exp::ASUB { exp, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. } => {
                    let mut b2: bool = false;
                    b2 = Expression::isConst(exp.clone())?;
                    Ok(if (b2.clone()) {e.clone()} else {inExp.clone()})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e @ Deref @ DAE::Exp::ASUB { exp, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. } => {
                    let mut b2: bool = false;
                    b2 = Expression::isConst(exp.clone())?;
                    Ok(if (b2.clone()) {e.clone()} else {inExp.clone()})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ASUB { exp, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "change" }, .. } => {
                    let mut b2: bool = false;
                    b2 = Expression::isConst(exp.clone())?;
                    Ok(if (b2.clone()) {Arc::new(DAE::Exp::BCONST { bool: false })} else {inExp.clone()})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ASUB { exp, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "edge" }, .. } => {
                    let mut b2: bool = false;
                    b2 = Expression::isConst(exp.clone())?;
                    Ok(if (b2.clone()) {Arc::new(DAE::Exp::BCONST { bool: false })} else {inExp.clone()})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. } => {
                    let mut e = (*e).clone();
                    (e, _) = Expression::traverseExpTopDown(e.clone(), (std::sync::Arc::new(fnptr!(preCref, Arc<DAE::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool, bool)> + 'static>), false)?;
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. } => {
                    let mut e = (*e).clone();
                    (e, _) = Expression::traverseExpTopDown(e.clone(), (std::sync::Arc::new(fnptr!(previousCref, Arc<DAE::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool, bool)> + 'static>), false)?;
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "change" }, .. } => {
                    let mut e = (*e).clone();
                    (e, _) = Expression::traverseExpTopDown(e.clone(), (std::sync::Arc::new(fnptr!(changeCref, Arc<DAE::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool, bool)> + 'static>), false)?;
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "edge" }, .. } => {
                    let mut e = (*e).clone();
                    (e, _) = Expression::traverseExpTopDown(e.clone(), (std::sync::Arc::new(fnptr!(edgeCref, Arc<DAE::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool, bool)> + 'static>), false)?;
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { isImpure: false, .. }, expLst: expl, path: Deref @ Absyn::Path::IDENT { name: idn } } => {
                    if !((Expression::isConstWorkList(expl.clone())?)) { bail!("guard") }
                    Ok(simplifyBuiltinConstantCalls((idn.clone()).clone(), inExp.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { builtin: true, .. }, .. } => {
                    Ok(simplifyBuiltinCalls(inExp.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: n }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "identity" }, .. } => {
                    let mut matrix: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    matrix = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut j in (1..=n.clone()).into_iter() {
                    let __x = Arc::new(DAE::Exp::ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: n.clone() })] }), scalar: true, array: {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut i in (1..=n.clone()).into_iter() {
                    let __x = if (i.clone() == j.clone()) {Arc::new(DAE::Exp::ICONST { integer: 1 })} else {Arc::new(DAE::Exp::ICONST { integer: 0 })};
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } });
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    Ok(Arc::new(DAE::Exp::ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: n.clone() }), Arc::new(DAE::Dimension::DIM_INTEGER { integer: n.clone() })] }), scalar: false, array: matrix.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ARRAY { ty: tp, array: expl, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "diagonal" }, .. } => {
                    let mut zero: Arc<DAE::Exp>;
                    let mut matrix: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut n: i32 = 0;
                    let mut tp = (*tp).clone();
                    n = (expl.clone().len() as i32);
                    tp = Types::arrayElementType(tp.clone());
                    zero = Expression::makeConstZero(tp.clone());
                    matrix = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut j in (1..=n.clone()).into_iter() {
                    let __x = Arc::new(DAE::Exp::ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: tp.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: n.clone() })] }), scalar: true, array: {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut i in (1..=n.clone()).into_iter() {
                    let __x = if (i.clone() == j.clone()) {(expl.clone()).get(i.clone())?} else {zero.clone()};
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } });
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    Ok(Arc::new(DAE::Exp::ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: tp.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: n.clone() }), Arc::new(DAE::Dimension::DIM_INTEGER { integer: n.clone() })] }), scalar: false, array: matrix.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: idn2 }, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: idn }, .. } => {
                    if !((idn.clone() == literal!("tan") && idn2.clone() == literal!("atan"))) { bail!("guard") }
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::RCONST { real: r1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::RCONST { real: r2 }, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "mod" }, .. } => {
                    if !((r2.clone() != metamodelica::OrderedFloat(0.0_f64))) { bail!("guard") }
                    Ok(Arc::new(DAE::Exp::RCONST { real: realMod(r1.clone(), r2.clone()) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: i1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: i2 }, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "mod" }, .. } => {
                    if !((metamodelica::OrderedFloat((i2.clone()) as f64) != metamodelica::OrderedFloat(0.0_f64))) { bail!("guard") }
                    Ok(Arc::new(DAE::Exp::ICONST { integer: intMod(i1.clone(), i2.clone()) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::RCONST { real: r1 }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "integer" }, .. } => {
                    Ok(Arc::new(DAE::Exp::ICONST { integer: ((r1.clone()).0 as i32) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "acos" }, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sin" }, .. } => {
                    Ok(Expression::makePureBuiltinCall((literal!("sqrt")).clone(), list![Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((1) as f64) }), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e.clone(), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e.clone() }) })], DAE::T_REAL_DEFAULT().clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "asin" }, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "cos" }, .. } => {
                    Ok(Expression::makePureBuiltinCall((literal!("sqrt")).clone(), list![Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((1) as f64) }), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e.clone(), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e.clone() }) })], DAE::T_REAL_DEFAULT().clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "atan" }, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sin" }, .. } => {
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e.clone(), operator: DAE::Operator::DIV { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: Expression::makePureBuiltinCall((literal!("sqrt")).clone(), list![Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((1) as f64) }), operator: DAE::Operator::ADD { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e.clone(), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e.clone() }) })], DAE::T_REAL_DEFAULT().clone()) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "atan" }, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "cos" }, .. } => {
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((1) as f64) }), operator: DAE::Operator::DIV { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: Expression::makePureBuiltinCall((literal!("sqrt")).clone(), list![Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((1) as f64) }), operator: DAE::Operator::ADD { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e.clone(), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e.clone() }) })], DAE::T_REAL_DEFAULT().clone()) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "atan2" }, .. } => {
                    if !((Expression::isZero(e2.clone()))) { bail!("guard") }
                    let mut e: Arc<DAE::Exp>;
                    e = Expression::makePureBuiltinCall((literal!("sign")).clone(), list![e1.clone()], DAE::T_REAL_DEFAULT().clone());
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.570796326794896619231321691639751442_f64) }), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1 @ Deref @ DAE::Exp::RCONST { real: __rlit_0 }, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "atan2" }, .. } => {
                    if !(__rlit_0.eq(&metamodelica::OrderedFloat((0.0) as f64))) { bail!("guard") }
                    Ok(e1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::RCONST { real: r1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::RCONST { real: r2 }, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "atan2" }, .. } => {
                    Ok(Arc::new(DAE::Exp::RCONST { real: (r1.clone()).atan2(r2.clone()) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::UNARY { exp: e1, operator: DAE::Operator::UMINUS { ty: tp } }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "abs" }, .. } => {
                    let mut e: Arc<DAE::Exp>;
                    e = Expression::makePureBuiltinCall((literal!("abs")).clone(), list![e1.clone()], tp.clone());
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if !((Config::acceptMetaModelicaGrammar()?)) { bail!("guard") }
                    Ok(simplifyMetaModelicaCalls(inExp.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

fn preCref(mut ie: Arc<DAE::Exp>, mut ib: bool) -> (Arc<DAE::Exp>, bool, bool) {
    let mut oe: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut ob: bool = false;
    (oe, cont, ob) = (::match_deref::match_deref! { match &((ie.clone(), ib.clone())) {
        (e @ Deref @ DAE::Exp::CREF { ty, .. }, _) => {
            (Expression::makeBuiltinCall((literal!("pre")).clone(), list![e.clone()], ty.clone(), false), false, true)
        },
        (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, b) => {
            (e.clone(), false, b.clone())
        },
        (e, b) => {
            (e.clone(), !(b.clone()), b.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (oe, cont, ob)
}

fn previousCref(mut ie: Arc<DAE::Exp>, mut ib: bool) -> (Arc<DAE::Exp>, bool, bool) {
    let mut oe: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut ob: bool = false;
    (oe, cont, ob) = (::match_deref::match_deref! { match &((ie.clone(), ib.clone())) {
        (e @ Deref @ DAE::Exp::CREF { ty, .. }, _) => {
            (Expression::makeBuiltinCall((literal!("previous")).clone(), list![e.clone()], ty.clone(), false), false, true)
        },
        (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. }, b) => {
            (e.clone(), false, b.clone())
        },
        (e, b) => {
            (e.clone(), !(b.clone()), b.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (oe, cont, ob)
}

fn changeCref(mut ie: Arc<DAE::Exp>, mut ib: bool) -> (Arc<DAE::Exp>, bool, bool) {
    let mut oe: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut ob: bool = false;
    (oe, cont, ob) = (::match_deref::match_deref! { match &((ie.clone(), ib.clone())) {
        (e @ Deref @ DAE::Exp::CREF { ty, .. }, _) => {
            (Expression::makeBuiltinCall((literal!("change")).clone(), list![e.clone()], ty.clone(), false), false, true)
        },
        (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "change" }, .. }, b) => {
            (e.clone(), false, b.clone())
        },
        (e, b) => {
            (e.clone(), !(b.clone()), b.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (oe, cont, ob)
}

fn edgeCref(mut ie: Arc<DAE::Exp>, mut ib: bool) -> (Arc<DAE::Exp>, bool, bool) {
    let mut oe: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut ob: bool = false;
    (oe, cont, ob) = (::match_deref::match_deref! { match &((ie.clone(), ib.clone())) {
        (e @ Deref @ DAE::Exp::CREF { ty, .. }, _) => {
            (Expression::makeBuiltinCall((literal!("edge")).clone(), list![e.clone()], ty.clone(), false), false, true)
        },
        (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "edge" }, .. }, b) => {
            (e.clone(), false, b.clone())
        },
        (e, b) => {
            (e.clone(), !(b.clone()), b.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (oe, cont, ob)
}

pub fn simplify1(mut inExp: Arc<DAE::Exp>) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut hasChanged: bool = false;
    (outExp, hasChanged) = simplify1WithOptions(inExp.clone(), optionSimplifyOnly.clone())?;
    Ok((outExp, hasChanged))
}

pub fn simplify1o(mut inExp: Option<Arc<DAE::Exp>>) -> Result<Option<Arc<DAE::Exp>>> {
    let mut outExp: Option<Arc<DAE::Exp>> = None;
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Some(e) => {
            let mut e = (*e).clone();
            (e, _) = simplify1WithOptions(e.clone(), optionSimplifyOnly.clone())?;
            Some(e.clone())
        },
        _ => {
            inExp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn simplify1WithOptions(mut inExp: Arc<DAE::Exp>, mut options: ExpressionSimplifyTypes::Evaluate) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut hasChanged: bool = false;
    (outExp, hasChanged) = simplify1FixP(inExp.clone(), options.clone(), 100, true, false)?;
    checkSimplify(Flags::isSet(Flags::CHECK_SIMPLIFY.clone())?, inExp.clone(), outExp.clone())?;
    Ok((outExp, hasChanged))
}

fn checkSimplify(mut check: bool, mut before: Arc<DAE::Exp>, mut after: Arc<DAE::Exp>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((check.clone(), before.clone(), after.clone())) {
        (false, _, _) => {
            ()
        },
        (true, _, _) => {
            let mut c1: i32 = 0;
            let mut c2: i32 = 0;
            let mut b: bool = false;
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut s3: ArcStr = arcstr::literal!("");
            let mut s4: ArcStr = arcstr::literal!("");
            let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            ty1 = Expression::r#typeof(before.clone())?;
            ty2 = Expression::r#typeof(after.clone())?;
            b = ty1.clone() == ty2.clone();
            if !(b.clone()) {
                s1 = (ExpressionBasics::printExpStr(before.clone())?).clone();
                s2 = (ExpressionBasics::printExpStr(after.clone())?).clone();
                s3 = (TypesDump::unparseType(ty1.clone())?).clone();
                s4 = (TypesDump::unparseType(ty2.clone())?).clone();
                Error::addMessage(Error::SIMPLIFICATION_TYPE.clone(), list![(s1.clone()).clone(), (s2.clone()).clone(), (s3.clone()).clone(), (s4.clone()).clone()])?;
                bail!("fail");
            }
            c1 = Expression::complexity(before.clone())?;
            c2 = Expression::complexity(after.clone())?;
            b = c1.clone() < c2.clone();
            if b.clone() {
                s1 = (intString(c2.clone())).clone();
                s2 = (intString(c1.clone())).clone();
                s3 = (ExpressionBasics::printExpStr(before.clone())?).clone();
                s4 = (ExpressionBasics::printExpStr(after.clone())?).clone();
                Error::addMessage(Error::SIMPLIFICATION_COMPLEXITY.clone(), list![(s1.clone()).clone(), (s2.clone()).clone(), (s3.clone()).clone(), (s4.clone()).clone()])?;
                bail!("fail");
            }
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn simplify1FixP(mut inExp: Arc<DAE::Exp>, mut inOptions: ExpressionSimplifyTypes::Evaluate, mut n: i32, mut cont: bool, mut hasChanged: bool) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outHasChanged: bool = false;
    (outExp, outHasChanged) = (::match_deref::match_deref! { match &((inExp.clone(), inOptions.clone(), n.clone(), cont.clone(), hasChanged.clone())) {
        (exp, _, _, false, _) => {
            (exp.clone(), hasChanged.clone())
        },
        (exp, options, 0, _, _) => {
            let mut str1: ArcStr = arcstr::literal!("");
            let mut str2: ArcStr = arcstr::literal!("");
            let mut exp = (*exp).clone();
            str1 = (ExpressionBasics::printExpStr(exp.clone())?).clone();
            (exp, _) = Expression::traverseExpBottomUp(exp.clone(), (std::sync::Arc::new(simplifyWork) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ExpressionSimplifyTypes::Evaluate) -> Result<(Arc<DAE::Exp>, ExpressionSimplifyTypes::Evaluate)> + 'static>), options.clone())?;
            str2 = (ExpressionBasics::printExpStr(exp.clone())?).clone();
            Error::addMessage(Error::SIMPLIFY_FIXPOINT_MAXIMUM.clone(), list![(str1.clone()).clone(), (str2.clone()).clone()])?;
            (exp.clone(), hasChanged.clone())
        },
        (exp, options, _, true, _) => {
            let mut expAfterSimplify: Arc<DAE::Exp>;
            let mut b: bool = false;
            let mut options = (*options).clone();
            ErrorExt::setCheckpoint((literal!("ExpressionSimplify")).clone());
            (expAfterSimplify, options) = Expression::traverseExpBottomUp(exp.clone(), (std::sync::Arc::new(simplifyWork) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ExpressionSimplifyTypes::Evaluate) -> Result<(Arc<DAE::Exp>, ExpressionSimplifyTypes::Evaluate)> + 'static>), options.clone())?;
            b = !(referenceEq(&expAfterSimplify.clone(),&exp.clone()));
            if b.clone() {
                ErrorExt::rollBack((literal!("ExpressionSimplify")).clone());
            } else {
                ErrorExt::delCheckpoint((literal!("ExpressionSimplify")).clone());
            }
            (expAfterSimplify, b) = simplify1FixP(expAfterSimplify.clone(), options.clone(), n.clone() - 1, b.clone(), b.clone() || hasChanged.clone())?;
            (expAfterSimplify.clone(), b.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outExp, outHasChanged))
}

fn simplifyReductionIterators(mut inIters: Arc<metamodelica::List<Arc<DAE::ReductionIterator>>>, mut inAcc: Arc<metamodelica::List<Arc<DAE::ReductionIterator>>>, mut inChange: bool) -> Result<(Arc<metamodelica::List<Arc<DAE::ReductionIterator>>>, bool)> {
    let mut outIters: Arc<metamodelica::List<Arc<DAE::ReductionIterator>>> = metamodelica::nil();
    let mut outChange: bool = false;
    (outIters, outChange) = (::match_deref::match_deref! { match &((inIters.clone(), inAcc.clone(), inChange.clone())) {
        (Deref @ metamodelica::List::Nil, acc, change) => {
            (acc.clone().reverse(), change.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::ReductionIterator { id, exp, guardExp: Some(Deref @ DAE::Exp::BCONST { bool: true }), ty }, tail: iters }, acc, _) => {
            let mut change: bool = false;
            let mut iters = (*iters).clone();
            (iters, change) = simplifyReductionIterators(iters.clone(), cons(Arc::new(DAE::ReductionIterator { id: (id.clone()).clone(), exp: exp.clone(), guardExp: None, ty: ty.clone() }), acc.clone()), true)?;
            (iters.clone(), change.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::ReductionIterator { id, exp: _, guardExp: Some(Deref @ DAE::Exp::BCONST { bool: false }), ty }, tail: _ }, _, _) => {
            (list![Arc::new(DAE::ReductionIterator { id: (id.clone()).clone(), exp: Arc::new(DAE::Exp::LIST { valList: metamodelica::nil() }), guardExp: None, ty: ty.clone() })], true)
        },
        (Deref @ metamodelica::List::Cons { head: iter, tail: iters }, acc, change) => {
            let mut iters = (*iters).clone();
            let mut change = (*change).clone();
            (iters, change) = simplifyReductionIterators(iters.clone(), cons(iter.clone(), acc.clone()), change.clone())?;
            (iters.clone(), change.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outIters, outChange))
}

fn simplifyIfExp(mut origExp: Arc<DAE::Exp>, mut cond: Arc<DAE::Exp>, mut tb: Arc<DAE::Exp>, mut fb: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut exp: Arc<DAE::Exp>;
    exp = (::match_deref::match_deref! { match &((origExp.clone(), cond.clone(), tb.clone(), fb.clone())) {
        (_, Deref @ DAE::Exp::BCONST { bool: true }, _, _) => {
            tb.clone()
        },
        (_, Deref @ DAE::Exp::BCONST { bool: false }, _, _) => {
            fb.clone()
        },
        (_, exp, Deref @ DAE::Exp::BCONST { bool: true }, Deref @ DAE::Exp::BCONST { bool: false }) => {
            exp.clone()
        },
        (_, exp, Deref @ DAE::Exp::BCONST { bool: false }, Deref @ DAE::Exp::BCONST { bool: true }) => {
            let mut exp = (*exp).clone();
            exp = Arc::new(DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: DAE::T_BOOL_DEFAULT().clone() }, exp: exp.clone() });
            exp.clone()
        },
        (_, e, Deref @ DAE::Exp::BOX { exp: e1 }, Deref @ DAE::Exp::BOX { exp: e2 }) => {
            let mut e = (*e).clone();
            e = Arc::new(DAE::Exp::IFEXP { expCond: e.clone(), expThen: e1.clone(), expElse: e2.clone() });
            Arc::new(DAE::Exp::BOX { exp: e.clone() })
        },
        _ => {
            if (ExpressionBasics::expEqual(tb.clone(), fb.clone())?) {tb.clone()} else {origExp.clone()}
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn simplifyMetaModelicaCalls(mut exp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::LIST { valList: el }, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "listAppend" }, .. } => {
            let mut e: Arc<DAE::Exp>;
            e = List::fold(el.clone().reverse(), (std::sync::Arc::new(fnptr!(Expression::makeCons, Arc<DAE::Exp>, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), e2.clone());
            e.clone()
        },
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::LIST { valList: Deref @ metamodelica::List::Nil }, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "listAppend" }, .. } => {
            e1.clone()
        },
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: i }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "intString" }, .. } => {
            let mut s: ArcStr = arcstr::literal!("");
            s = (intString(i.clone())).clone();
            Arc::new(DAE::Exp::SCONST { string: (s.clone()).clone() })
        },
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::RCONST { real: r }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "realString" }, .. } => {
            let mut s: ArcStr = arcstr::literal!("");
            s = (realString(r.clone())).clone();
            Arc::new(DAE::Exp::SCONST { string: (s.clone()).clone() })
        },
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::BCONST { bool: b }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "boolString" }, .. } => {
            let mut s: ArcStr = arcstr::literal!("");
            s = (boolString(b.clone())).clone();
            Arc::new(DAE::Exp::SCONST { string: (s.clone()).clone() })
        },
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::LIST { valList: el }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "listReverse" }, .. } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut el = (*el).clone();
            el = el.clone().reverse();
            e1_1 = Arc::new(DAE::Exp::LIST { valList: el.clone() });
            e1_1.clone()
        },
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::REDUCTION { reductionInfo: Deref @ DAE::ReductionInfo { path: Deref @ Absyn::Path::IDENT { name: Deref @ "list" }, iterType: rit, exprType: ty, defaultValue: v, foldName, resultName, foldExp }, expr: e1, iterators: riters }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "listReverse" }, .. } => {
            let mut e1 = (*e1).clone();
            e1 = Arc::new(DAE::Exp::REDUCTION { reductionInfo: Arc::new(DAE::ReductionInfo { path: Arc::new(Absyn::Path::IDENT { name: (literal!("listReverse")).clone() }), iterType: rit.clone(), exprType: ty.clone(), defaultValue: v.clone(), foldName: (foldName.clone()).clone(), resultName: (resultName.clone()).clone(), foldExp: foldExp.clone() }), expr: e1.clone(), iterators: riters.clone() });
            e1.clone()
        },
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::REDUCTION { reductionInfo: Deref @ DAE::ReductionInfo { path: Deref @ Absyn::Path::IDENT { name: Deref @ "listReverse" }, iterType: rit, exprType: ty, defaultValue: v, foldName, resultName, foldExp }, expr: e1, iterators: riters }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "listReverse" }, .. } => {
            let mut e1 = (*e1).clone();
            e1 = Arc::new(DAE::Exp::REDUCTION { reductionInfo: Arc::new(DAE::ReductionInfo { path: Arc::new(Absyn::Path::IDENT { name: (literal!("list")).clone() }), iterType: rit.clone(), exprType: ty.clone(), defaultValue: v.clone(), foldName: (foldName.clone()).clone(), resultName: (resultName.clone()).clone(), foldExp: foldExp.clone() }), expr: e1.clone(), iterators: riters.clone() });
            e1.clone()
        },
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::LIST { valList: el }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "listLength" }, .. } => {
            let mut i: i32 = 0;
            i = (el.clone().len() as i32);
            Arc::new(DAE::Exp::ICONST { integer: i.clone() })
        },
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "mmc_mk_some" }, .. } => {
            Arc::new(DAE::Exp::META_OPTION { exp: Some(e.clone()) })
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sourceInfo" }, .. } => {
            println!("{}", (literal!("sourceInfo() - simplify?\n")).clone());
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

fn simplifyCons(mut inExp: Arc<DAE::Exp>) -> Arc<DAE::Exp> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CONS { car: e, cdr: Deref @ DAE::Exp::LIST { valList: es } } => {
            Arc::new(DAE::Exp::LIST { valList: cons(e.clone(), es.clone()) })
        },
        _ => {
            inExp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp
}

fn simplifyUnbox(mut exp: Arc<DAE::Exp>) -> Arc<DAE::Exp> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::UNBOX { exp: Deref @ DAE::Exp::BOX { exp: outExp }, .. } => outExp.clone(),
        Deref @ DAE::Exp::BOX { exp: Deref @ DAE::Exp::UNBOX { exp: outExp, .. } } => outExp.clone(),
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp
}

fn simplifyMatch(mut exp: Arc<DAE::Exp>) -> Arc<DAE::Exp> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::MATCHEXPRESSION { cases: Deref @ metamodelica::List::Cons { head: Deref @ DAE::MatchCase { result: Some(e), body: Deref @ metamodelica::List::Nil, localDecls: Deref @ metamodelica::List::Nil, patterns: Deref @ metamodelica::List::Nil, .. }, tail: Deref @ metamodelica::List::Nil }, localDecls: Deref @ metamodelica::List::Nil, et: ty, inputs: Deref @ metamodelica::List::Nil, .. } if (!(Types::isTuple(ty.clone()))) => {
            e.clone()
        },
        Deref @ DAE::Exp::MATCHEXPRESSION { cases: Deref @ metamodelica::List::Cons { head: Deref @ DAE::MatchCase { result: Some(e1), body: Deref @ metamodelica::List::Nil, localDecls: Deref @ metamodelica::List::Nil, patterns: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Pattern::PAT_CONSTANT { exp: Deref @ DAE::Exp::BCONST { bool: b1 }, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::MatchCase { result: Some(e2), body: Deref @ metamodelica::List::Nil, localDecls: Deref @ metamodelica::List::Nil, patterns: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Pattern::PAT_CONSTANT { exp: Deref @ DAE::Exp::BCONST { bool: b2 }, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, tail: Deref @ metamodelica::List::Nil } }, localDecls: Deref @ metamodelica::List::Nil, et: ty, inputs: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, .. } if (!(boolEq(b1.clone(), b2.clone())) && !(Types::isTuple(ty.clone()))) => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut e = (*e).clone();
            e1_1 = if (b1.clone()) {e1.clone()} else {e2.clone()};
            e2_1 = if (b1.clone()) {e2.clone()} else {e1.clone()};
            e = Arc::new(DAE::Exp::IFEXP { expCond: e.clone(), expThen: e1_1.clone(), expElse: e2_1.clone() });
            e.clone()
        },
        Deref @ DAE::Exp::MATCHEXPRESSION { cases: Deref @ metamodelica::List::Cons { head: Deref @ DAE::MatchCase { result: Some(e1), body: Deref @ metamodelica::List::Nil, localDecls: Deref @ metamodelica::List::Nil, patterns: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Pattern::PAT_CONSTANT { exp: Deref @ DAE::Exp::BCONST { bool: b1 }, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::MatchCase { result: Some(e2), body: Deref @ metamodelica::List::Nil, localDecls: Deref @ metamodelica::List::Nil, patterns: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Pattern::PAT_WILD, tail: Deref @ metamodelica::List::Nil }, .. }, tail: Deref @ metamodelica::List::Nil } }, localDecls: Deref @ metamodelica::List::Nil, inputs: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, et: ty, matchType: DAE::MatchType::MATCH { .. }, .. } if (!(Types::isTuple(ty.clone()))) => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut e = (*e).clone();
            e1_1 = if (b1.clone()) {e1.clone()} else {e2.clone()};
            e2_1 = if (b1.clone()) {e2.clone()} else {e1.clone()};
            e = Arc::new(DAE::Exp::IFEXP { expCond: e.clone(), expThen: e1_1.clone(), expElse: e2_1.clone() });
            e.clone()
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp
}

fn simplifyCast(mut origExp: Arc<DAE::Exp>, mut exp: Arc<DAE::Exp>, mut tp: Type) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &((origExp.clone(), exp.clone(), tp.clone())) {
        (_, Deref @ DAE::Exp::RCONST { real: r }, Deref @ DAE::Type::T_REAL { .. }) => {
            Arc::new(DAE::Exp::RCONST { real: r.clone() })
        },
        (_, Deref @ DAE::Exp::ICONST { integer: i }, Deref @ DAE::Type::T_REAL { .. }) => {
            let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            r = intReal(i.clone());
            Arc::new(DAE::Exp::RCONST { real: r.clone() })
        },
        (_, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e }, _) => {
            let mut e = (*e).clone();
            e = addCast(e.clone(), tp.clone());
            Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: tp.clone() }, exp: e.clone() })
        },
        (_, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: e }, _) => {
            let mut e = (*e).clone();
            e = addCast(e.clone(), tp.clone());
            Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp.clone() }, exp: e.clone() })
        },
        (_, Deref @ DAE::Exp::ARRAY { ty: _, scalar: b, array: exps }, _) => {
            let mut exps_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut tp_1: Type = Arc::new(DAE::Type::T_NORETCALL);
            tp_1 = Expression::unliftArray(tp.clone())?;
            exps_1 = List::map1(exps.clone(), (std::sync::Arc::new(fnptr!(addCast, Arc<DAE::Exp>, Arc<DAE::Type>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> + 'static>), tp_1.clone());
            Arc::new(DAE::Exp::ARRAY { ty: tp.clone(), scalar: b.clone(), array: exps_1.clone() })
        },
        (_, Deref @ DAE::Exp::RANGE { stop: e2, step: eo, start: e1, ty: Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_INTEGER { .. }, .. } }, Deref @ DAE::Type::T_ARRAY { ty: tp2 @ Deref @ DAE::Type::T_REAL { .. }, .. }) => {
            let mut e2 = (*e2).clone();
            let mut eo = (*eo).clone();
            let mut e1 = (*e1).clone();
            e1 = addCast(e1.clone(), tp2.clone());
            e2 = addCast(e2.clone(), tp2.clone());
            eo = Util::applyOption1(eo.clone(), (std::sync::Arc::new(fnptr!(addCast, Arc<DAE::Exp>, Arc<DAE::Type>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> + 'static>), tp2.clone());
            Arc::new(DAE::Exp::RANGE { ty: tp.clone(), start: e1.clone(), step: eo.clone(), stop: e2.clone() })
        },
        (_, Deref @ DAE::Exp::IFEXP { expCond: cond, expThen: e1, expElse: e2 }, _) => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            e1_1 = Arc::new(DAE::Exp::CAST { ty: tp.clone(), exp: e1.clone() });
            e2_1 = Arc::new(DAE::Exp::CAST { ty: tp.clone(), exp: e2.clone() });
            Arc::new(DAE::Exp::IFEXP { expCond: cond.clone(), expThen: e1_1.clone(), expElse: e2_1.clone() })
        },
        (_, Deref @ DAE::Exp::MATRIX { ty: _, integer: n, matrix: mexps }, _) => {
            let mut tp1: Type = Arc::new(DAE::Type::T_NORETCALL);
            let mut tp2: Type = Arc::new(DAE::Type::T_NORETCALL);
            let mut mexps_1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
            tp1 = Expression::unliftArray(tp.clone())?;
            tp2 = Expression::unliftArray(tp1.clone())?;
            mexps_1 = List::map1List(mexps.clone(), (std::sync::Arc::new(fnptr!(addCast, Arc<DAE::Exp>, Arc<DAE::Type>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> + 'static>), tp2.clone());
            Arc::new(DAE::Exp::MATRIX { ty: tp.clone(), integer: n.clone(), matrix: mexps_1.clone() })
        },
        (_, Deref @ DAE::Exp::CALL { path: p1, expLst: exps, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: p2 }, .. }, .. } }, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: p3 }, .. }) if (AbsynUtil::pathEqual(p1.clone(), p2.clone())) => {
            Arc::new(DAE::Exp::CALL { path: p3.clone(), expLst: exps.clone(), attr: Arc::new(DAE::CallAttributes { ty: tp.clone(), tuple_: false, builtin: false, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL }) })
        },
        (_, Deref @ DAE::Exp::RECORD { path: _, exps, comp: fieldNames, ty: _ }, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: p3 }, .. }) => {
            Arc::new(DAE::Exp::RECORD { path: p3.clone(), exps: exps.clone(), comp: fieldNames.clone(), ty: tp.clone() })
        },
        (_, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: exps }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "fill" }, .. }, _) => {
            let mut tp_1: Type = Arc::new(DAE::Type::T_NORETCALL);
            let mut e = (*e).clone();
            tp_1 = List::fold(exps.clone(), (std::sync::Arc::new(Expression::unliftArrayIgnoreFirst) as std::sync::Arc<dyn ::std::ops::Fn(_, Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>), tp.clone());
            e = Arc::new(DAE::Exp::CAST { ty: tp_1.clone(), exp: e.clone() });
            e = Expression::makePureBuiltinCall((literal!("fill")).clone(), cons(e.clone(), exps.clone()), tp.clone());
            e.clone()
        },
        (_, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e @ Deref @ DAE::Exp::ICONST { integer: n }, tail: exps }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "cat" }, .. }, Deref @ DAE::Type::T_ARRAY { dims, .. }) if (Expression::dimensionUnknown((dims.clone()).get(n.clone())?)) => {
            let mut exps = (*exps).clone();
            exps = List::map1(exps.clone(), (std::sync::Arc::new(fnptr!(addCast, Arc<DAE::Exp>, Arc<DAE::Type>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> + 'static>), tp.clone());
            Expression::makePureBuiltinCall((literal!("cat")).clone(), cons(e.clone(), exps.clone()), tp.clone())
        },
        (_, e, _) => {
            let mut t1: Type = Arc::new(DAE::Type::T_NORETCALL);
            let mut t2: Type = Arc::new(DAE::Type::T_NORETCALL);
            t1 = Expression::arrayEltType(tp.clone());
            t2 = Expression::arrayEltType(Expression::r#typeof(e.clone())?);
            if (t1.clone() == t2.clone()) {e.clone()} else {origExp.clone()}
        },
        _ => {
            origExp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

fn addCast(mut inExp: Arc<DAE::Exp>, mut inType: Type) -> Arc<DAE::Exp> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = Arc::new(DAE::Exp::CAST { ty: inType.clone(), exp: inExp.clone() });
    outExp
}

fn reductionDefaultValue(mut name: ArcStr, mut ty: Arc<DAE::Type>) -> Result<Arc<Values::Value>> {
    let mut defaultValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    defaultValue = (::match_deref::match_deref! { match &((name.clone(), ty.clone())) {
        (Deref @ "min", Deref @ DAE::Type::T_BOOL { .. }) => Arc::new(Values::Value::BOOL { boolean: true }),
        (Deref @ "min", Deref @ DAE::Type::T_INTEGER { .. }) => Arc::new(Values::Value::INTEGER { integer: System::intMaxLit() }),
        (Deref @ "min", Deref @ DAE::Type::T_REAL { .. }) => Arc::new(Values::Value::REAL { real: System::realMaxLit() }),
        (Deref @ "min", Deref @ DAE::Type::T_ENUMERATION { .. }) => Arc::new(Values::Value::ENUM_LITERAL { name: AbsynUtil::suffixPath(var_field!((*ty).path, DAE::Type::T_ENUMERATION).clone(), (List::last(var_field!((*ty).names, DAE::Type::T_ENUMERATION).clone())?).clone())?, index: (var_field!((*ty).names, DAE::Type::T_ENUMERATION).clone().len() as i32) }),
        (Deref @ "max", Deref @ DAE::Type::T_BOOL { .. }) => Arc::new(Values::Value::BOOL { boolean: false }),
        (Deref @ "max", Deref @ DAE::Type::T_INTEGER { .. }) => Arc::new(Values::Value::INTEGER { integer: intNeg(System::intMaxLit()) }),
        (Deref @ "max", Deref @ DAE::Type::T_REAL { .. }) => Arc::new(Values::Value::REAL { real: -(System::realMaxLit()) }),
        (Deref @ "max", Deref @ DAE::Type::T_ENUMERATION { .. }) => Arc::new(Values::Value::ENUM_LITERAL { name: AbsynUtil::suffixPath(var_field!((*ty).path, DAE::Type::T_ENUMERATION).clone(), (List::last(var_field!((*ty).names, DAE::Type::T_ENUMERATION).clone())?).clone())?, index: 1 }),
        (Deref @ "product", Deref @ DAE::Type::T_INTEGER { .. }) => Arc::new(Values::Value::INTEGER { integer: 1 }),
        (Deref @ "product", Deref @ DAE::Type::T_REAL { .. }) => Arc::new(Values::Value::REAL { real: metamodelica::OrderedFloat(1.0_f64) }),
        (Deref @ "sum", Deref @ DAE::Type::T_INTEGER { .. }) => Arc::new(Values::Value::INTEGER { integer: 0 }),
        (Deref @ "sum", Deref @ DAE::Type::T_REAL { .. }) => Arc::new(Values::Value::REAL { real: metamodelica::OrderedFloat(0.0_f64) }),
        _ => bail!("match: no arm matched"),
    } });
    Ok(defaultValue)
}

fn reductionExpression(mut name: ArcStr, mut ty: Arc<DAE::Type>, mut foldName: ArcStr, mut resultName: ArcStr) -> Result<Arc<DAE::Exp>> {
    let mut foldExp: Arc<DAE::Exp>;
    let mut foldNameExp: Arc<DAE::Exp>;
    let mut resultNameExp: Arc<DAE::Exp>;
    foldNameExp = Expression::makeCrefExp(ComponentReferenceBasics::makeCrefIdent((foldName.clone()).clone(), ty.clone(), metamodelica::nil()), ty.clone())?;
    resultNameExp = Expression::makeCrefExp(ComponentReferenceBasics::makeCrefIdent((resultName.clone()).clone(), ty.clone(), metamodelica::nil()), ty.clone())?;
    foldExp = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "min" => Expression::makeBuiltinCall((literal!("min")).clone(), list![foldNameExp.clone(), resultNameExp.clone()], ty.clone(), false),
        Deref @ "max" => Expression::makeBuiltinCall((literal!("max")).clone(), list![foldNameExp.clone(), resultNameExp.clone()], ty.clone(), false),
        Deref @ "product" => Arc::new(DAE::Exp::BINARY { exp1: foldNameExp.clone(), operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: resultNameExp.clone() }),
        Deref @ "sum" => Arc::new(DAE::Exp::BINARY { exp1: foldNameExp.clone(), operator: DAE::Operator::ADD { ty: ty.clone() }, exp2: resultNameExp.clone() }),
        _ => bail!("match: no arm matched"),
    } });
    Ok(foldExp)
}

fn simplifyBuiltinCalls(mut exp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = exp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: tp, .. }, expLst: Deref @ metamodelica::List::Cons { head: e @ Deref @ DAE::Exp::ARRAY { .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name } } => {
                    if !((name.clone() == literal!("max") || name.clone() == literal!("min"))) { bail!("guard") }
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut e1: Arc<DAE::Exp>;
                    expl = Expression::flattenArrayExpToList(e.clone())?;
                    e1 = Expression::makeScalarArray(expl.clone(), tp.clone());
                    let false = (ExpressionBasics::expEqual(e.clone(), e1.clone())?) else { bail!("pattern mismatch") };
                    Ok(Expression::makePureBuiltinCall((name.clone()).clone(), list![e1.clone()], tp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ARRAY { array: expl @ Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name }, .. } => {
                    if !((name.clone() == literal!("max") || name.clone() == literal!("min"))) { bail!("guard") }
                    let mut e = (*e).clone();
                    let mut exp: Arc<DAE::Exp> = exp.clone();
                    if Expression::isArrayType(Expression::r#typeof(e.clone())?) {
                        assign_variant_field!(exp => DAE::Exp::CALL; expLst = expl.clone());
                        e = exp.clone();
                    }
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "max" }, .. } => {
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: tp, .. }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ARRAY { array: es, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "max" } } => {
                    let mut e: Arc<DAE::Exp>;
                    let mut i1: i32 = 0;
                    let mut i2: i32 = 0;
                    let mut es = (*es).clone();
                    i1 = (es.clone().len() as i32);
                    es = List::union(es.clone(), metamodelica::nil());
                    i2 = (es.clone().len() as i32);
                    if i1.clone() == i2.clone() {
                        let __pa0 = ::match_deref::match_deref! { match &(List::fold(es.clone(), (std::sync::Arc::new(fnptr!(maxElement, Arc<DAE::Exp>, Option<Arc<DAE::Exp>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Option<Arc<DAE::Exp>>) -> Result<Option<Arc<DAE::Exp>>> + 'static>), None)) {
                            Some(__pa0) => __pa0.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        e = __pa0.clone();
                        es = List::select(es.clone(), (std::sync::Arc::new(fnptr!(removeMinMaxFoldableValues, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>));
                        es = cons(e.clone(), es.clone());
                        i2 = (es.clone().len() as i32);
                        let true = (i2.clone() < i1.clone()) else { bail!("pattern mismatch") };
                        e = Expression::makeScalarArray(es.clone(), tp.clone());
                    } else {
                        e = Expression::makeScalarArray(es.clone(), tp.clone());
                    }
                    Ok(Expression::makePureBuiltinCall((literal!("max")).clone(), list![e.clone()], tp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: tp, .. }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ARRAY { array: es, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "min" } } => {
                    let mut e: Arc<DAE::Exp>;
                    let mut i1: i32 = 0;
                    let mut i2: i32 = 0;
                    let mut es = (*es).clone();
                    i1 = (es.clone().len() as i32);
                    es = List::union(es.clone(), metamodelica::nil());
                    i2 = (es.clone().len() as i32);
                    if i1.clone() == i2.clone() {
                        let __pa0 = ::match_deref::match_deref! { match &(List::fold(es.clone(), (std::sync::Arc::new(fnptr!(minElement, Arc<DAE::Exp>, Option<Arc<DAE::Exp>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Option<Arc<DAE::Exp>>) -> Result<Option<Arc<DAE::Exp>>> + 'static>), None)) {
                            Some(__pa0) => __pa0.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        e = __pa0.clone();
                        es = List::select(es.clone(), (std::sync::Arc::new(fnptr!(removeMinMaxFoldableValues, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>));
                        es = cons(e.clone(), es.clone());
                        i2 = (es.clone().len() as i32);
                        let true = (i2.clone() < i1.clone()) else { bail!("pattern mismatch") };
                        e = Expression::makeScalarArray(es.clone(), tp.clone());
                    } else {
                        e = Expression::makeScalarArray(es.clone(), tp.clone());
                    }
                    Ok(Expression::makePureBuiltinCall((literal!("min")).clone(), list![e.clone()], tp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, .. }, tail: Deref @ metamodelica::List::Nil }, attr: Deref @ DAE::CallAttributes { ty: tp, .. }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "min" } } => {
                    let mut e: Arc<DAE::Exp>;
                    e = Expression::makePureBuiltinCall((literal!("min")).clone(), list![e1.clone(), e2.clone()], tp.clone());
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, .. }, tail: Deref @ metamodelica::List::Nil }, attr: Deref @ DAE::CallAttributes { ty: tp, .. }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "max" } } => {
                    let mut e: Arc<DAE::Exp>;
                    e = Expression::makePureBuiltinCall((literal!("max")).clone(), list![e1.clone(), e2.clone()], tp.clone());
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_BOOL { .. }, .. }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "min" } } => {
                    let mut e: Arc<DAE::Exp>;
                    e = Arc::new(DAE::Exp::LBINARY { exp1: e1.clone(), operator: DAE::Operator::AND { ty: DAE::T_BOOL_DEFAULT().clone() }, exp2: e2.clone() });
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_BOOL { .. }, .. }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "max" } } => {
                    let mut e: Arc<DAE::Exp>;
                    e = Arc::new(DAE::Exp::LBINARY { exp1: e1.clone(), operator: DAE::Operator::OR { ty: DAE::T_BOOL_DEFAULT().clone() }, exp2: e2.clone() });
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ARRAY { array: expl, .. }, tail: Deref @ metamodelica::List::Nil }, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_BOOL { .. }, .. }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "min" } } => {
                    let mut e: Arc<DAE::Exp>;
                    e = Expression::makeLBinary(expl.clone(), DAE::Operator::AND { ty: DAE::T_BOOL_DEFAULT().clone() })?;
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ARRAY { array: expl, .. }, tail: Deref @ metamodelica::List::Nil }, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_BOOL { .. }, .. }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "max" } } => {
                    let mut e: Arc<DAE::Exp>;
                    e = Expression::makeLBinary(expl.clone(), DAE::Operator::OR { ty: DAE::T_BOOL_DEFAULT().clone() })?;
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: tp, .. }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ARRAY { array: expl @ Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } }, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name } } => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut e2: Arc<DAE::Exp>;
                    let mut expl = (*expl).clone();
                    let true = (Config::scalarizeMinMax()?) else { bail!("pattern mismatch") };
                    let true = (stringEq((name.clone()).clone(), (literal!("max")).clone()) || stringEq((name.clone()).clone(), (literal!("min")).clone())) else { bail!("pattern mismatch") };
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(expl.clone().reverse()) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: __pa2 } } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1 = __pa0.clone();
                    e2 = __pa1.clone();
                    expl = __pa2.clone();
                    e1 = Expression::makePureBuiltinCall((name.clone()).clone(), list![e2.clone(), e1.clone()], tp.clone());
                    e1 = List::fold2(expl.clone(), (std::sync::Arc::new(fnptr!(makeNestedReduction, Arc<DAE::Exp>, ArcStr, Arc<DAE::Type>, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArcStr, Arc<DAE::Type>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), (name.clone()).clone(), tp.clone(), e1.clone());
                    Ok(e1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e @ Deref @ DAE::Exp::CALL { expLst: expl, path: Deref @ Absyn::Path::IDENT { name: Deref @ "cross" }, .. } => {
                    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut v1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut v2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut scalar: bool = false;
                    let mut expl = (*expl).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(expl.clone()) {
                        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ARRAY { array: __pa0, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ARRAY { array: __pa1, .. }, tail: Deref @ metamodelica::List::Nil } } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    v1 = __pa0.clone();
                    v2 = __pa1.clone();
                    expl = simplifyCross(v1.clone(), v2.clone())?;
                    tp = Expression::r#typeof(e.clone())?;
                    scalar = !(Expression::isArrayType(Expression::unliftArray(tp.clone())?));
                    Ok(Arc::new(DAE::Exp::ARRAY { ty: tp.clone(), scalar: scalar.clone(), array: expl.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ARRAY { array: v1, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "skew" }, .. } => {
                    let mut mexpl: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
                    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    mexpl = simplifySkew(v1.clone())?;
                    tp = Expression::r#typeof(e.clone())?;
                    Ok(Arc::new(DAE::Exp::MATRIX { ty: tp.clone(), integer: 3, matrix: mexpl.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: expl }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "fill" }, .. } => {
                    let mut valueLst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut outExp: Arc<DAE::Exp>;
                    valueLst = List::map(expl.clone(), (std::sync::Arc::new(ValuesUtil::expValue) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<Values::Value>> + 'static>));
                    (_, outExp, _) = Static::elabBuiltinFill2(FCore::noCache(), FGraph::empty(), e.clone(), Expression::r#typeof(e.clone())?, valueLst.clone(), openmodelica_frontend_types::DAE::Const::C_CONST, openmodelica_frontend_types::DAE::Prefix::NOPRE, metamodelica::nil(), Absyn::dummyInfo.clone())?;
                    Ok(outExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Cons { head: len_exp, tail: Deref @ metamodelica::List::Cons { head: just_exp, tail: Deref @ metamodelica::List::Nil } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "String" }, .. } => {
                    Ok(simplifyBuiltinStringFormat(e.clone(), len_exp.clone(), just_exp.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::LIST { valList: expl }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "stringAppendList" }, .. } => {
                    Ok(simplifyStringAppendList(expl.clone(), metamodelica::nil(), false)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { ty: Deref @ DAE::Type::T_REAL { .. } }, exp2: e2 }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, .. } => {
                    let mut e: Arc<DAE::Exp>;
                    e = Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::POW { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.5_f64) }), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e2.clone() }) });
                    Ok(Expression::makePureBuiltinCall((literal!("abs")).clone(), list![e.clone()], DAE::T_REAL_DEFAULT().clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, .. } => {
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::POW { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.25_f64) }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::RCONST { real: r1 }, operator: DAE::Operator::MUL { ty: tp }, exp2: e2 }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, .. } => {
                    let mut e: Arc<DAE::Exp>;
                    let mut e3: Arc<DAE::Exp>;
                    let true = (r1.clone() >= metamodelica::OrderedFloat(0.0_f64)) else { bail!("pattern mismatch") };
                    e = Expression::makePureBuiltinCall((literal!("sqrt")).clone(), list![e1.clone()], DAE::T_REAL_DEFAULT().clone());
                    e3 = Expression::makePureBuiltinCall((literal!("sqrt")).clone(), list![e2.clone()], DAE::T_REAL_DEFAULT().clone());
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: e3.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e1 }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }, .. } => {
                    let mut es: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut e: Arc<DAE::Exp>;
                    let mut e2: Arc<DAE::Exp>;
                    let mut e3: Arc<DAE::Exp>;
                    expl = Expression::expandFactors(e1.clone())?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(List::split1OnTrue(expl.clone(), (std::sync::Arc::new(fnptr!(Expression::isFunCall, Arc<DAE::Exp>, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArcStr) -> Result<bool> + 'static>), (literal!("log")).clone())) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    e2 = __pa0.clone();
                    es = __pa1.clone();
                    let __pa3 = ::match_deref::match_deref! { match &(e2.clone()) {
                        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Nil }, .. } => __pa3.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e = __pa3.clone();
                    e3 = Expression::makeProductLst(es.clone())?;
                    Ok(Expression::expPow(e.clone(), Expression::negate(e3.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }, .. } => {
                    let mut es: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut e: Arc<DAE::Exp>;
                    let mut e2: Arc<DAE::Exp>;
                    let mut e3: Arc<DAE::Exp>;
                    expl = Expression::expandFactors(e1.clone())?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(List::split1OnTrue(expl.clone(), (std::sync::Arc::new(fnptr!(Expression::isFunCall, Arc<DAE::Exp>, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArcStr) -> Result<bool> + 'static>), (literal!("log")).clone())) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    e2 = __pa0.clone();
                    es = __pa1.clone();
                    let __pa3 = ::match_deref::match_deref! { match &(e2.clone()) {
                        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Nil }, .. } => __pa3.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e = __pa3.clone();
                    e3 = Expression::makeProductLst(es.clone())?;
                    Ok(Expression::expPow(e.clone(), e3.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }, .. }, operator: DAE::Operator::POW { ty: Deref @ DAE::Type::T_REAL { .. } }, exp2: e2 } => {
                    let mut e3: Arc<DAE::Exp>;
                    e3 = Expression::expMul(e.clone(), e2.clone())?;
                    Ok(Expression::makePureBuiltinCall((literal!("exp")).clone(), list![e3.clone()], DAE::T_REAL_DEFAULT().clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { ty: Deref @ DAE::Type::T_REAL { .. } }, exp2: Deref @ DAE::Exp::RCONST { real: r1 } }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "log" }, .. } => {
                    let mut e3: Arc<DAE::Exp>;
                    let _ /* lit — guard not yet implemented */ = (realMod(r1.clone(), metamodelica::OrderedFloat(2.0_f64))) else { bail!("pattern mismatch") };
                    e3 = Expression::makePureBuiltinCall((literal!("log")).clone(), list![e1.clone()], DAE::T_REAL_DEFAULT().clone());
                    Ok(Expression::expMul(Arc::new(DAE::Exp::RCONST { real: r1.clone() }), e3.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::RCONST { real: __rlit_1 }, operator: DAE::Operator::DIV { ty: Deref @ DAE::Type::T_REAL { .. } }, exp2: e2 }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "log" }, .. } => {
                    if !(__rlit_1.eq(&metamodelica::OrderedFloat((1.0) as f64))) { bail!("guard") }
                    let mut e3: Arc<DAE::Exp>;
                    e3 = Expression::makePureBuiltinCall((literal!("log")).clone(), list![e2.clone()], DAE::T_REAL_DEFAULT().clone());
                    Ok(Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: DAE::T_REAL_DEFAULT().clone() }, exp: e3.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "log" }, .. } => {
                    let mut e3: Arc<DAE::Exp>;
                    e3 = Expression::makePureBuiltinCall((literal!("log")).clone(), list![e1.clone()], DAE::T_REAL_DEFAULT().clone());
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.5_f64) }), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e3.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "smooth" }, .. } => {
                    if !((Expression::isConst(e1.clone())?)) { bail!("guard") }
                    Ok(e1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "$_DF$DER" }, .. } => {
                    if !((Expression::isConst(e1.clone())?)) { bail!("guard") }
                    Ok(Expression::makeConstZeroE(e1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "delay" }, .. } => {
                    let mut e = (*e).clone();
                    let __owned_variant_expLst_0 = list![e1.clone(), e2.clone(), e2.clone()];
                    if let DAE::Exp::CALL { expLst, .. } = &mut e {
                        *expLst = __owned_variant_expLst_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than DAE::Exp::CALL"); }
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "delay" }, .. } => {
                    if !((Expression::isConst(e1.clone())?)) { bail!("guard") }
                    Ok(e1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: tp, .. }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 }, tail: Deref @ metamodelica::List::Cons { head: e3, tail: Deref @ metamodelica::List::Cons { head: e4, tail: Deref @ metamodelica::List::Nil } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "delay" } } => {
                    let mut e: Arc<DAE::Exp>;
                    let true = (Expression::isConst(e1.clone())?) else { bail!("pattern mismatch") };
                    e = Expression::makeImpureBuiltinCall((literal!("delay")).clone(), list![e2.clone(), e3.clone(), e4.clone()], tp.clone());
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op.clone(), exp2: e.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: tp, .. }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 }, tail: Deref @ metamodelica::List::Cons { head: e3, tail: Deref @ metamodelica::List::Cons { head: e4, tail: Deref @ metamodelica::List::Nil } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "delay" } } => {
                    let mut e: Arc<DAE::Exp>;
                    let true = (Expression::isConst(e2.clone())?) else { bail!("pattern mismatch") };
                    e = Expression::makeImpureBuiltinCall((literal!("delay")).clone(), list![e1.clone(), e3.clone(), e4.clone()], tp.clone());
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e.clone(), operator: op.clone(), exp2: e2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: tp, .. }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::UNARY { operator: op, exp: e }, tail: Deref @ metamodelica::List::Cons { head: e3, tail: Deref @ metamodelica::List::Cons { head: e4, tail: Deref @ metamodelica::List::Nil } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "delay" } } => {
                    let mut e = (*e).clone();
                    e = Expression::makeImpureBuiltinCall((literal!("delay")).clone(), list![e.clone(), e3.clone(), e4.clone()], tp.clone());
                    Ok(Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: e.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: tp1, .. }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Nil, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sum" } } => {
                    Ok(Expression::makeConstZero(tp1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: tp2, .. }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::MATRIX { matrix: mexpl, ty: tp1, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sum" } } => {
                    let mut es: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut e: Arc<DAE::Exp>;
                    let mut sc: bool = false;
                    let mut dim: i32 = 0;
                    let mut tp1 = (*tp1).clone();
                    es = List::flatten(mexpl.clone());
                    tp1 = Expression::unliftArray(Expression::unliftArray(tp1.clone())?)?;
                    sc = !(Expression::isArrayType(tp1.clone()));
                    tp1 = if (sc.clone()) {Expression::unliftArray(tp1.clone())?} else {tp1.clone()};
                    tp1 = if (sc.clone()) {Expression::liftArrayLeft(tp1.clone(), Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN))} else {tp1.clone()};
                    dim = (es.clone().len() as i32);
                    tp1 = Expression::liftArrayLeft(tp1.clone(), Arc::new(DAE::Dimension::DIM_INTEGER { integer: dim.clone() }));
                    e = Arc::new(DAE::Exp::ARRAY { ty: tp1.clone(), scalar: sc.clone(), array: es.clone() });
                    e = Expression::makePureBuiltinCall((literal!("sum")).clone(), list![e.clone()], tp2.clone());
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: tp2, .. }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ARRAY { scalar: false, ty: tp1, array: es }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sum" } } => {
                    let mut e: Arc<DAE::Exp>;
                    let mut sc: bool = false;
                    let mut dim: i32 = 0;
                    let mut tp1 = (*tp1).clone();
                    let mut es = (*es).clone();
                    es = simplifyCat(1, es.clone())?;
                    tp1 = Expression::unliftArray(tp1.clone())?;
                    sc = !(Expression::isArrayType(tp1.clone()));
                    tp1 = if (sc.clone()) {Expression::unliftArray(tp1.clone())?} else {tp1.clone()};
                    tp1 = if (sc.clone()) {Expression::liftArrayLeft(tp1.clone(), Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN))} else {tp1.clone()};
                    dim = (es.clone().len() as i32);
                    tp1 = Expression::liftArrayLeft(tp1.clone(), Arc::new(DAE::Dimension::DIM_INTEGER { integer: dim.clone() }));
                    e = Arc::new(DAE::Exp::ARRAY { ty: tp1.clone(), scalar: sc.clone(), array: es.clone() });
                    e = Expression::makePureBuiltinCall((literal!("sum")).clone(), list![e.clone()], tp2.clone());
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: tp2, .. }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ARRAY { scalar: false, array: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sum" } } => {
                    let mut e = (*e).clone();
                    e = Expression::makePureBuiltinCall((literal!("sum")).clone(), list![e.clone()], tp2.clone());
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ARRAY { scalar: true, array: es, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sum" }, .. } => {
                    let mut e: Arc<DAE::Exp>;
                    e = Expression::makeSum(es.clone())?;
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "cat" }, .. } => {
                    Ok(e1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: tp, .. }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: i }, tail: es }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "cat" } } => {
                    let mut e: Arc<DAE::Exp>;
                    let mut es = (*es).clone();
                    es = simplifyCat(i.clone(), es.clone())?;
                    e = Expression::makePureBuiltinCall((literal!("cat")).clone(), cons(Arc::new(DAE::Exp::ICONST { integer: i.clone() }), es.clone()), tp.clone());
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { .. }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: i }, tail: es }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "cat" } } => {
                    let mut e: Arc<DAE::Exp>;
                    let mut dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut es = (*es).clone();
                    (es, dims) = ExpressionBasics::evalCat(i.clone(), es.clone(), (std::sync::Arc::new(Expression::getArrayOrMatrixContents) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>))?;
                    e = Expression::listToArray(es.clone(), {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
        for mut d in (dims.clone()).into_iter().cloned() {
                    let __x = Arc::new(DAE::Dimension::DIM_INTEGER { integer: d.clone() });
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })?;
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: i }, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "promote" }, .. } => {
                    if !((Types::numberOfDimensions(Expression::r#typeof(e1.clone())?)? == i.clone())) { bail!("guard") }
                    Ok(e1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ARRAY { ty: tp1 @ Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, .. }, scalar: sc, array: es }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: 2 }, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "promote" }, .. } => {
                    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut i: i32 = 0;
                    let mut es = (*es).clone();
                    tp = Types::liftArray(Types::unliftArray(tp1.clone())?, Arc::new(DAE::Dimension::DIM_INTEGER { integer: 1 }));
                    es = List::map2(List::map(es.clone(), std::sync::Arc::new(fnptr!(List::create, _))), (std::sync::Arc::new(fnptr!(Expression::makeArray, Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<DAE::Type>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<DAE::Type>, bool) -> Result<Arc<DAE::Exp>> + 'static>), tp.clone(), sc.clone());
                    i = (es.clone().len() as i32);
                    tp = Expression::liftArrayLeft(tp.clone(), Arc::new(DAE::Dimension::DIM_INTEGER { integer: i.clone() }));
                    Ok(Arc::new(DAE::Exp::ARRAY { ty: tp.clone(), scalar: false, array: es.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: i }, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "promote" }, .. } => {
                    if !((!(Types::isArray(Expression::r#typeof(e1.clone())?)))) { bail!("guard") }
                    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut tp1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut e1 = (*e1).clone();
                    tp = Expression::r#typeof(e1.clone())?;
                    for mut j in 1..=i.clone() {
                        tp1 = Types::liftArray(tp.clone(), Arc::new(DAE::Dimension::DIM_INTEGER { integer: 1 }));
                        e1 = Expression::makeArray(list![e1.clone()], tp1.clone(), !(Types::isArray(tp.clone())));
                        tp = tp1.clone();
                    }
                    Ok(e1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { .. }, expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "transpose" } } => {
                    let mut e = (*e).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::transposeArray(e.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e = __pa0.clone();
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: tp, .. }, expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "symmetric" } } => {
                    let mut mexpl: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
                    let mut es: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut tp1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut marr: metamodelica::Array<metamodelica::Array<Arc<DAE::Exp>>>;
                    let mut e = (*e).clone();
                    mexpl = Expression::get2dArrayOrMatrixContent(e.clone())?;
                    e = (::match_deref::match_deref! { match &(mexpl.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Nil, tail: Deref @ metamodelica::List::Nil } => e.clone(),
        Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, tail: Deref @ metamodelica::List::Nil } => e.clone(),
        _ => {
                    marr = metamodelica::arrayFromVec(List::map(mexpl.clone(), Arc::new(listArray.clone())).into_iter().cloned().collect());
                    let true = ((marr.clone().borrow().len() as i32) == (marr.clone().borrow()[(1-1) as usize].clone().borrow().len() as i32)) else { bail!("pattern mismatch") };
                    let true = ((marr.clone().borrow().len() as i32) > 1) else { bail!("pattern mismatch") };
                    simplifySymmetric(marr.clone(), (marr.clone().borrow().len() as i32) - 1, (marr.clone().borrow().len() as i32))?;
                    mexpl = List::mapArray(marr.clone(), Arc::new(arrayList.clone()));
                    tp1 = Expression::unliftArray(tp.clone())?;
                    es = List::map2(mexpl.clone(), (std::sync::Arc::new(fnptr!(Expression::makeArray, Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<DAE::Type>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<DAE::Type>, bool) -> Result<Arc<DAE::Exp>> + 'static>), tp1.clone(), !(Types::isArray(tp1.clone())));
                    e = Expression::makeArray(es.clone(), tp.clone(), false);
                    e.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: tp, .. }, expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "scalar" } } => {
                    let mut e = (*e).clone();
                    e = simplifyScalar(e.clone(), tp.clone())?;
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_ARRAY { ty: tp, dims: _ }, .. }, expLst: es @ Deref @ metamodelica::List::Cons { head: e, tail: _ }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "vector" } } => {
                    let mut i: i32 = 0;
                    let mut tp = (*tp).clone();
                    let false = (Types::isArray(Expression::r#typeof(e.clone())?)) else { bail!("pattern mismatch") };
                    i = (es.clone().len() as i32);
                    tp = Arc::new(DAE::Type::T_ARRAY { ty: tp.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: i.clone() })] });
                    Ok(Arc::new(DAE::Exp::ARRAY { ty: tp.clone(), scalar: true, array: es.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { .. }, expLst: Deref @ metamodelica::List::Cons { head: e @ Deref @ DAE::Exp::ARRAY { scalar: true, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "vector" } } => {
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: tp, .. }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::MATRIX { matrix: mexpl, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "vector" } } => {
                    let mut es: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut e: Arc<DAE::Exp>;
                    es = List::flatten(mexpl.clone());
                    es = List::map1(es.clone(), (std::sync::Arc::new(fnptr!(Expression::makeVectorCall, Arc<DAE::Exp>, Arc<DAE::Type>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> + 'static>), tp.clone());
                    e = Expression::makePureBuiltinCall((literal!("cat")).clone(), cons(Arc::new(DAE::Exp::ICONST { integer: 1 }), es.clone()), tp.clone());
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: tp, .. }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ARRAY { array: es, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "vector" } } => {
                    let mut e: Arc<DAE::Exp>;
                    let mut es = (*es).clone();
                    es = List::map1(es.clone(), (std::sync::Arc::new(fnptr!(Expression::makeVectorCall, Arc<DAE::Exp>, Arc<DAE::Type>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> + 'static>), tp.clone());
                    e = Expression::makePureBuiltinCall((literal!("cat")).clone(), cons(Arc::new(DAE::Exp::ICONST { integer: 1 }), es.clone()), tp.clone());
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Nil, path: Deref @ Absyn::Path::IDENT { name: Deref @ "inferredClock" }, .. } => {
                    Ok(Arc::new(DAE::Exp::CLKCONST { clk: Arc::new(openmodelica_frontend_types::DAE::ClockKind::INFERRED_CLOCK) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "realClock" }, .. } => {
                    Ok(Arc::new(DAE::Exp::CLKCONST { clk: Arc::new(DAE::ClockKind::REAL_CLOCK { interval: e1.clone() }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "booleanClock" }, .. } => {
                    Ok(Arc::new(DAE::Exp::CLKCONST { clk: Arc::new(DAE::ClockKind::EVENT_CLOCK { condition: e1.clone(), startInterval: e2.clone() }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "rationalClock" }, .. } => {
                    Ok(Arc::new(DAE::Exp::CLKCONST { clk: Arc::new(DAE::ClockKind::RATIONAL_CLOCK { intervalCounter: e1.clone(), resolution: e2.clone() }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "solverClock" }, .. } => {
                    Ok(Arc::new(DAE::Exp::CLKCONST { clk: Arc::new(DAE::ClockKind::SOLVER_CLOCK { c: e1.clone(), solverMethod: e2.clone() }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::SCONST { string: s1 }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "OpenModelica_uriToFilename" }, .. } => {
                    let mut e: Arc<DAE::Exp>;
                    let mut s2: ArcStr = arcstr::literal!("");
                    s2 = uriToFilename((s1.clone()).clone());
                    if Flags::getConfigBool(Flags::BUILDING_FMU.clone())? {
                        e = Expression::makeImpureBuiltinCall((literal!("OpenModelica_fmuLoadResource")).clone(), list![Arc::new(DAE::Exp::SCONST { string: (s2.clone()).clone() })], DAE::T_STRING_DEFAULT().clone());
                    } else {
                        e = Arc::new(DAE::Exp::SCONST { string: (s2.clone()).clone() });
                    }
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

fn simplifyScalar(mut inExp: Arc<DAE::Exp>, mut tp: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut exp: Arc<DAE::Exp>;
    exp = (::match_deref::match_deref! { match &((inExp.clone(), tp.clone())) {
        (Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, .. }, _) => Expression::makePureBuiltinCall((literal!("scalar")).clone(), list![exp.clone()], tp.clone()),
        (Deref @ DAE::Exp::MATRIX { matrix: Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, tail: Deref @ metamodelica::List::Nil }, .. }, _) => Expression::makePureBuiltinCall((literal!("scalar")).clone(), list![exp.clone()], tp.clone()),
        (Deref @ DAE::Exp::SIZE { sz: None, exp }, _) => {
            ::match_deref::match_deref! { match &(TypesDump::flattenArrayType(Expression::r#typeof(inExp.clone())?)) {
                (_, Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }) => (),
                _ => bail!("pattern mismatch"),
            } };
            Arc::new(DAE::Exp::SIZE { exp: exp.clone(), sz: Some(Arc::new(DAE::Exp::ICONST { integer: 1 })) })
        },
        _ => {
            ::match_deref::match_deref! { match &(TypesDump::flattenArrayType(Expression::r#typeof(inExp.clone())?)) {
                (_, Deref @ metamodelica::List::Nil) => (),
                _ => bail!("pattern mismatch"),
            } };
            inExp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn makeNestedReduction(mut inExp: Arc<DAE::Exp>, mut inName: ArcStr, mut inType: Arc<DAE::Type>, mut inCall: Arc<DAE::Exp>) -> Arc<DAE::Exp> {
    let mut outCall: Arc<DAE::Exp>;
    outCall = Expression::makePureBuiltinCall((inName.clone()).clone(), list![inExp.clone(), inCall.clone()], inType.clone());
    outCall
}

fn simplifySymmetric(mut marr: metamodelica::Array<metamodelica::Array<Arc<DAE::Exp>>>, mut i1: i32, mut i2: i32) -> Result<()> {
    let _ = (match (marr.clone(), i1.clone(), i2.clone()) {
        (_, 0, 1) => {
            ()
        },
        _ => {
            let mut v1: metamodelica::Array<Arc<DAE::Exp>>;
            let mut v2: metamodelica::Array<Arc<DAE::Exp>>;
            let mut exp: Arc<DAE::Exp>;
            v1 = marr.clone().borrow()[(i1.clone()-1) as usize].clone();
            v2 = marr.clone().borrow()[(i2.clone()-1) as usize].clone();
            exp = v1.clone().borrow()[(i2.clone()-1) as usize].clone();
            {let _arr = v2.clone(); _arr.borrow_mut()[(i1.clone()-1) as usize] = exp.clone(); _arr};
            simplifySymmetric(marr.clone(), if (i1.clone() == 1) {i2.clone() - 2} else {i1.clone() - 1}, if (i1.clone() == 1) {i2.clone() - 1} else {i2.clone()})?;
            ()
        },
    });
    Ok(())
}

fn simplifyCat(mut inDim: i32, mut inExpList: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExpList: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    outExpList = (::match_deref::match_deref! { match &((inDim.clone(), inExpList.clone())) {
        (1, _) => {
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            expl = List::map(inExpList.clone(), (std::sync::Arc::new(simplifyCatArg) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>));
            simplifyCat2(inDim.clone(), expl.clone(), metamodelica::nil(), false)?
        },
        _ => {
            simplifyCat2(inDim.clone(), inExpList.clone(), metamodelica::nil(), false)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExpList)
}

fn simplifyCatArg(mut arg: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outArg: Arc<DAE::Exp>;
    outArg = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ DAE::Exp::MATRIX { .. } => {
            Expression::matrixToArray(arg.clone())?
        },
        Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: dim, tail: Deref @ metamodelica::List::Nil }, .. }, .. } if (Expression::dimensionKnown(dim.clone())) => {
            Arc::new(DAE::Exp::ARRAY { ty: var_field!((*arg).ty, DAE::Exp::CREF).clone(), scalar: true, array: Expression::expandExpression(arg.clone(), false)? })
        },
        _ => {
            arg.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outArg)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn simplifyCat2(mut dim: i32, mut ies: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut acc: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut changed: bool) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut oes: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    oes = 'mc: {
        let __mc_input = (dim.clone(), ies.clone(), acc.clone(), changed.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, _, true) => {
                    Ok(acc.clone().reverse())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (1, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { ty: etp, dims: Deref @ metamodelica::List::Cons { head: dim1, tail: dims } }, scalar: sc, array: es1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: dim2, tail: _ }, .. }, array: es2, .. }, tail: es } }, _, _) => {
                    let mut esn: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut e: Arc<DAE::Exp>;
                    let mut ndim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let mut etp = (*etp).clone();
                    esn = listAppend(es1.clone(), es2.clone());
                    ndim = Expression::addDimensions(dim1.clone(), dim2.clone())?;
                    etp = Arc::new(DAE::Type::T_ARRAY { ty: etp.clone(), dims: cons(ndim.clone(), dims.clone()) });
                    e = Arc::new(DAE::Exp::ARRAY { ty: etp.clone(), scalar: sc.clone(), array: esn.clone() });
                    Ok(simplifyCat2(dim.clone(), cons(e.clone(), es.clone()), acc.clone(), true)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (2, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::MATRIX { ty: Deref @ DAE::Type::T_ARRAY { ty: etp, dims: Deref @ metamodelica::List::Cons { head: dim11, tail: Deref @ metamodelica::List::Cons { head: dim1, tail: dims } } }, integer: i, matrix: ms1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::MATRIX { ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: dim2, tail: _ } }, .. }, matrix: ms2, .. }, tail: es } }, _, _) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut ndim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let mut mss: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
                    let mut etp = (*etp).clone();
                    mss = List::threadMap(ms1.clone(), ms2.clone(), Arc::new(listAppend.clone()));
                    ndim = Expression::addDimensions(dim1.clone(), dim2.clone())?;
                    etp = Arc::new(DAE::Type::T_ARRAY { ty: etp.clone(), dims: cons(dim11.clone(), cons(ndim.clone(), dims.clone())) });
                    e = Arc::new(DAE::Exp::MATRIX { ty: etp.clone(), integer: i.clone(), matrix: mss.clone() });
                    Ok(simplifyCat2(dim.clone(), cons(e.clone(), es.clone()), acc.clone(), true)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: e, tail: es }, _, _) => {
                    Ok(simplifyCat2(dim.clone(), es.clone(), cons(e.clone(), acc.clone()), changed.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oes)
}

fn simplifyBuiltinStringFormat(mut exp: Arc<DAE::Exp>, mut len_exp: Arc<DAE::Exp>, mut just_exp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &((exp.clone(), len_exp.clone(), just_exp.clone())) {
        (Deref @ DAE::Exp::ICONST { integer: i }, Deref @ DAE::Exp::ICONST { integer: len }, Deref @ DAE::Exp::BCONST { bool: just }) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (intString(i.clone())).clone();
            r#str = (cevalBuiltinStringFormat((r#str.clone()).clone(), ((r#str.clone()).clone().len() as i32), len.clone(), just.clone())).clone();
            Arc::new(DAE::Exp::SCONST { string: (r#str.clone()).clone() })
        },
        (Deref @ DAE::Exp::RCONST { real: r }, Deref @ DAE::Exp::ICONST { integer: len }, Deref @ DAE::Exp::BCONST { bool: just }) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (realString(r.clone())).clone();
            r#str = (cevalBuiltinStringFormat((r#str.clone()).clone(), ((r#str.clone()).clone().len() as i32), len.clone(), just.clone())).clone();
            Arc::new(DAE::Exp::SCONST { string: (r#str.clone()).clone() })
        },
        (Deref @ DAE::Exp::BCONST { bool: b }, Deref @ DAE::Exp::ICONST { integer: len }, Deref @ DAE::Exp::BCONST { bool: just }) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (boolString(b.clone())).clone();
            r#str = (cevalBuiltinStringFormat((r#str.clone()).clone(), ((r#str.clone()).clone().len() as i32), len.clone(), just.clone())).clone();
            Arc::new(DAE::Exp::SCONST { string: (r#str.clone()).clone() })
        },
        (Deref @ DAE::Exp::ENUM_LITERAL { name, .. }, Deref @ DAE::Exp::ICONST { integer: len }, Deref @ DAE::Exp::BCONST { bool: just }) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (AbsynUtil::pathLastIdent(name.clone())?).clone();
            r#str = (cevalBuiltinStringFormat((r#str.clone()).clone(), ((r#str.clone()).clone().len() as i32), len.clone(), just.clone())).clone();
            Arc::new(DAE::Exp::SCONST { string: (r#str.clone()).clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

pub fn cevalBuiltinStringFormat(mut inString: ArcStr, mut stringLength: i32, mut minLength: i32, mut leftJustified: bool) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = (if (stringLength.clone() >= minLength.clone()) {inString.clone()} else {if (leftJustified.clone()) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*inString.clone()); __mm_s.push_str(&*stringAppendList(List::fill((literal!(" ")).clone(), minLength.clone() - stringLength.clone()))); ArcStr::from(__mm_s) }} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*stringAppendList(List::fill((literal!(" ")).clone(), minLength.clone() - stringLength.clone()))); __mm_s.push_str(&*inString.clone()); ArcStr::from(__mm_s) }}}).clone();
    outString
}

fn simplifyStringAppendList(mut iexpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut iacc: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut ichange: bool) -> Result<Arc<DAE::Exp>> {
    let mut exp: Arc<DAE::Exp>;
    exp = (::match_deref::match_deref! { match &((iexpl.clone(), iacc.clone(), ichange.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _) => {
            Arc::new(DAE::Exp::SCONST { string: (literal!("")).clone() })
        },
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, _) => {
            exp.clone()
        },
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Cons { head: exp1, tail: Deref @ metamodelica::List::Cons { head: exp2, tail: Deref @ metamodelica::List::Nil } }, _) => {
            Arc::new(DAE::Exp::BINARY { exp1: exp2.clone(), operator: DAE::Operator::ADD { ty: DAE::T_STRING_DEFAULT().clone() }, exp2: exp1.clone() })
        },
        (Deref @ metamodelica::List::Nil, acc, true) => {
            let mut acc = (*acc).clone();
            acc = acc.clone().reverse();
            exp = Arc::new(DAE::Exp::LIST { valList: acc.clone() });
            Expression::makePureBuiltinCall((literal!("stringAppendList")).clone(), list![exp.clone()], DAE::T_STRING_DEFAULT().clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::SCONST { string: s1 }, tail: rest }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::SCONST { string: s2 }, tail: acc }, _) => {
            let mut s: ArcStr = arcstr::literal!("");
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*s1.clone()); ArcStr::from(__mm_s) }).clone();
            simplifyStringAppendList(rest.clone(), cons(Arc::new(DAE::Exp::SCONST { string: (s.clone()).clone() }), acc.clone()), true)?
        },
        (Deref @ metamodelica::List::Cons { head: exp, tail: rest }, acc, change) => {
            simplifyStringAppendList(rest.clone(), cons(exp.clone(), acc.clone()), change.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(exp)
}

fn simplifyBuiltinConstantCalls(mut name: ArcStr, mut exp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (name.clone(), exp.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "der", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, .. }) => {
                    let mut e1: Arc<DAE::Exp>;
                    e1 = simplifyBuiltinConstantDer(e.clone())?;
                    Ok(e1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "pre", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, .. }) => {
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "previous", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, .. }) => {
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "edge", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, .. }) => {
                    Ok(Arc::new(DAE::Exp::BCONST { bool: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "change", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, .. }) => {
                    Ok(Arc::new(DAE::Exp::BCONST { bool: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "sqrt", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, .. }) => {
                    let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    r = (Expression::toReal(e.clone())?).sqrt();
                    Ok(Arc::new(DAE::Exp::RCONST { real: r.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "abs", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::RCONST { real: r }, tail: Deref @ metamodelica::List::Nil }, .. }) => {
                    let mut r = (*r).clone();
                    r = r.clone().abs();
                    Ok(Arc::new(DAE::Exp::RCONST { real: r.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "abs", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: i }, tail: Deref @ metamodelica::List::Nil }, .. }) => {
                    let mut i = (*i).clone();
                    i = i.clone().abs();
                    Ok(Arc::new(DAE::Exp::ICONST { integer: i.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "sin", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, .. }) => {
                    let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    r = (Expression::toReal(e.clone())?).sin();
                    Ok(Arc::new(DAE::Exp::RCONST { real: r.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "cos", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, .. }) => {
                    let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    r = (Expression::toReal(e.clone())?).cos();
                    Ok(Arc::new(DAE::Exp::RCONST { real: r.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "asin", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, .. }) => {
                    let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    r = Expression::toReal(e.clone())?;
                    let true = (r.clone() >= metamodelica::OrderedFloat(-1.0_f64) && r.clone() <= metamodelica::OrderedFloat(1.0_f64)) else { bail!("pattern mismatch") };
                    r = (r.clone()).asin();
                    Ok(Arc::new(DAE::Exp::RCONST { real: r.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "acos", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, .. }) => {
                    let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    r = Expression::toReal(e.clone())?;
                    let true = (r.clone() >= metamodelica::OrderedFloat(-1.0_f64) && r.clone() <= metamodelica::OrderedFloat(1.0_f64)) else { bail!("pattern mismatch") };
                    r = (Expression::toReal(e.clone())?).acos();
                    Ok(Arc::new(DAE::Exp::RCONST { real: r.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "tan", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, .. }) => {
                    let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    r = (Expression::toReal(e.clone())?).tan();
                    Ok(Arc::new(DAE::Exp::RCONST { real: r.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "exp", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, .. }) => {
                    let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    r = (Expression::toReal(e.clone())?).exp();
                    Ok(Arc::new(DAE::Exp::RCONST { real: r.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "log", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, .. }) => {
                    let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    r = Expression::toReal(e.clone())?;
                    let true = (r.clone() > metamodelica::OrderedFloat((0) as f64)) else { bail!("pattern mismatch") };
                    r = (r.clone()).ln();
                    Ok(Arc::new(DAE::Exp::RCONST { real: r.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "log10", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, .. }) => {
                    let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    r = Expression::toReal(e.clone())?;
                    let true = (r.clone() > metamodelica::OrderedFloat((0) as f64)) else { bail!("pattern mismatch") };
                    r = (r.clone()).log10();
                    Ok(Arc::new(DAE::Exp::RCONST { real: r.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "min", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: i }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: j }, tail: Deref @ metamodelica::List::Nil } }, .. }) => {
                    let mut i = (*i).clone();
                    i = std::cmp::min(i.clone(), j.clone());
                    Ok(Arc::new(DAE::Exp::ICONST { integer: i.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "min", Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_REAL { .. }, .. }, expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil } }, .. }) => {
                    let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut v1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut v2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    v1 = Expression::toReal(e.clone())?;
                    v2 = Expression::toReal(e1.clone())?;
                    r = std::cmp::min(v1.clone(), v2.clone());
                    Ok(Arc::new(DAE::Exp::RCONST { real: r.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "min", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e @ Deref @ DAE::Exp::ENUM_LITERAL { index: i, .. }, tail: Deref @ metamodelica::List::Cons { head: e1 @ Deref @ DAE::Exp::ENUM_LITERAL { index: j, .. }, tail: Deref @ metamodelica::List::Nil } }, .. }) => {
                    let mut e2: Arc<DAE::Exp>;
                    e2 = if (i.clone() < j.clone()) {e.clone()} else {e1.clone()};
                    Ok(e2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "max", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: i }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: j }, tail: Deref @ metamodelica::List::Nil } }, .. }) => {
                    let mut i = (*i).clone();
                    i = std::cmp::max(i.clone(), j.clone());
                    Ok(Arc::new(DAE::Exp::ICONST { integer: i.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "max", Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_REAL { .. }, .. }, expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil } }, .. }) => {
                    let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut v1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut v2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    v1 = Expression::toReal(e.clone())?;
                    v2 = Expression::toReal(e1.clone())?;
                    r = std::cmp::max(v1.clone(), v2.clone());
                    Ok(Arc::new(DAE::Exp::RCONST { real: r.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "max", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e @ Deref @ DAE::Exp::ENUM_LITERAL { index: i, .. }, tail: Deref @ metamodelica::List::Cons { head: e1 @ Deref @ DAE::Exp::ENUM_LITERAL { index: j, .. }, tail: Deref @ metamodelica::List::Nil } }, .. }) => {
                    let mut e2: Arc<DAE::Exp>;
                    e2 = if (i.clone() > j.clone()) {e.clone()} else {e1.clone()};
                    Ok(e2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "sign", Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::RCONST { real: r }, tail: Deref @ metamodelica::List::Nil }, .. }) => {
                    let mut i: i32 = 0;
                    i = if (realEq(r.clone(), metamodelica::OrderedFloat(0.0_f64))) {0} else {if (realGt(r.clone(), metamodelica::OrderedFloat(0.0_f64))) {1} else {-1}};
                    Ok(Arc::new(DAE::Exp::ICONST { integer: i.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

fn simplifyCref(mut origExp: Arc<DAE::Exp>, mut inCREF: ComponentRef, mut inType: Type) -> Result<Arc<DAE::Exp>> {
    let mut exp: Arc<DAE::Exp>;
    exp = 'mc: {
        let __mc_input = inCREF.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_IDENT { ident: idn, identType: t2, subscriptLst: ssl @ Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::SLICE { exp: Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: _ } }, tail: _ } } => {
                    let mut cr: ComponentRef = Arc::new(DAE::ComponentRef::WILD);
                    let mut expCref: Arc<DAE::Exp>;
                    let mut exp: Arc<DAE::Exp>;
                    cr = ComponentReferenceBasics::makeCrefIdent((idn.clone()).clone(), t2.clone(), metamodelica::nil());
                    expCref = Expression::makeCrefExp(cr.clone(), inType.clone())?;
                    exp = simplifyCref2(expCref.clone(), ssl.clone())?;
                    Ok(exp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::SLICE { exp: Deref @ DAE::Exp::RANGE { .. } }, tail: _ }, .. } => {
                    let mut cr: ComponentRef = Arc::new(DAE::ComponentRef::WILD);
                    let mut expCref: Arc<DAE::Exp>;
                    cr = ComponentReference::crefStripSubs(inCREF.clone())?;
                    expCref = Expression::makeCrefExp(cr.clone(), inType.clone())?;
                    Ok(simplifyCref2(expCref.clone(), var_field!((*inCREF).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { ident: idn, identType: Deref @ DAE::Type::T_METATYPE { ty: t2 }, subscriptLst: ssl, componentRef: cr } => {
                    let mut exp: Arc<DAE::Exp>;
                    exp = simplifyCrefMM1((idn.clone()).clone(), t2.clone(), ssl.clone());
                    exp = simplifyCrefMM(exp.clone(), Expression::r#typeof(exp.clone())?, cr.clone())?;
                    Ok(exp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(origExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(exp)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn simplifyCref2(mut inExp: Arc<DAE::Exp>, mut inSsl: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (inExp.clone(), inSsl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp_1, Deref @ metamodelica::List::Nil) => {
                    Ok(exp_1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr @ Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, identType: _, subscriptLst: _ }, ty: t }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::SLICE { exp: Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: expl_1 } }, tail: ssl }) => {
                    let mut exp: Arc<DAE::Exp>;
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
                    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut dim: i32 = 0;
                    let mut t = (*t).clone();
                    subs = List::map(expl_1.clone(), (std::sync::Arc::new(fnptr!(Expression::makeIndexSubscript, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Subscript>> + 'static>));
                    crefs = List::map1r(List::map(subs.clone(), std::sync::Arc::new(fnptr!(List::create, _))), (std::sync::Arc::new(ComponentReference::subscriptCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Arc<DAE::ComponentRef>> + 'static>), cr.clone());
                    t = Types::unliftArray(t.clone())?;
                    expl = List::map1(crefs.clone(), (std::sync::Arc::new(Expression::makeCrefExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> + 'static>), t.clone());
                    dim = (expl.clone().len() as i32);
                    exp = simplifyCref2(Arc::new(DAE::Exp::ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: t.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: dim.clone() })] }), scalar: true, array: expl.clone() }), ssl.clone())?;
                    Ok(exp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr @ Deref @ DAE::ComponentRef::CREF_IDENT { .. }, ty: t }, Deref @ metamodelica::List::Cons { head: ss @ Deref @ DAE::Subscript::SLICE { exp: Deref @ DAE::Exp::RANGE { .. } }, tail: ssl }) => {
                    let mut exp: Arc<DAE::Exp>;
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
                    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut dim: i32 = 0;
                    let mut t = (*t).clone();
                    subs = Expression::expandSliceExp(var_field!((**ss).exp, DAE::Subscript::SLICE).clone())?;
                    crefs = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        for mut s in (subs.clone()).into_iter().cloned() {
                    let __x = ComponentReference::subscriptCref(cr.clone(), List::create(s.clone()))?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    t = Types::unliftArray(t.clone())?;
                    expl = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut cr in (crefs.clone()).into_iter().cloned() {
                    let __x = Expression::makeCrefExp(cr.clone(), t.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    dim = (expl.clone().len() as i32);
                    exp = Arc::new(DAE::Exp::ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: t.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: dim.clone() })] }), scalar: true, array: expl.clone() });
                    Ok(simplifyCref2(exp.clone(), ssl.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { ty: tp, scalar: sc, array: expl }, ssl) => {
                    let mut expl = (*expl).clone();
                    expl = List::map1(expl.clone(), (std::sync::Arc::new(simplifyCref2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Arc<DAE::Exp>> + 'static>), ssl.clone());
                    Ok(Arc::new(DAE::Exp::ARRAY { ty: tp.clone(), scalar: sc.clone(), array: expl.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

fn simplifyCrefMM_index(mut inExp: Arc<DAE::Exp>, mut ident: ArcStr, mut ty: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut exp: Arc<DAE::Exp>;
    let mut index: i32 = 0;
    let mut nty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut fields: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    fields = Types::getMetaRecordFields(ty.clone())?;
    index = Types::findVarIndex((ident.clone()).clone(), fields.clone()) + 1;
    let __pa0 = ::match_deref::match_deref! { match &((fields.clone()).get(index.clone())?) {
        Deref @ DAE::Var { ty: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    nty = __pa0.clone();
    exp = Arc::new(DAE::Exp::RSUB { exp: inExp.clone(), ix: index.clone(), fieldName: (ident.clone()).clone(), ty: nty.clone() });
    Ok(exp)
}

fn simplifyCrefMM(mut inExp: Arc<DAE::Exp>, mut inType: Arc<DAE::Type>, mut inCref: ComponentRef) -> Result<Arc<DAE::Exp>> {
    let mut exp: Arc<DAE::Exp>;
    exp = (::match_deref::match_deref! { match &(inCref.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { .. } => {
            exp = simplifyCrefMM_index(inExp.clone(), (var_field!((*inCref).ident, DAE::ComponentRef::CREF_IDENT).clone()).clone(), inType.clone())?;
            exp = if (var_field!((*inCref).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone().is_empty()) {exp.clone()} else {Arc::new(DAE::Exp::ASUB { exp: exp.clone(), sub: var_field!((*inCref).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone() })};
            exp.clone()
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { .. } => {
            exp = simplifyCrefMM_index(inExp.clone(), (var_field!((*inCref).ident, DAE::ComponentRef::CREF_QUAL).clone()).clone(), inType.clone())?;
            exp = if (var_field!((*inCref).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone().is_empty()) {exp.clone()} else {Arc::new(DAE::Exp::ASUB { exp: exp.clone(), sub: var_field!((*inCref).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone() })};
            exp = simplifyCrefMM(exp.clone(), Expression::r#typeof(exp.clone())?, var_field!((*inCref).componentRef, DAE::ComponentRef::CREF_QUAL).clone())?;
            exp.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(exp)
}

fn simplifyCrefMM1(mut ident: ArcStr, mut ty: Arc<DAE::Type>, mut ssl: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Arc<DAE::Exp> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(ssl.clone()) {
        Deref @ metamodelica::List::Nil => Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (ident.clone()).clone(), identType: ty.clone(), subscriptLst: metamodelica::nil() }), ty: ty.clone() }),
        _ => Arc::new(DAE::Exp::ASUB { exp: Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (ident.clone()).clone(), identType: ty.clone(), subscriptLst: metamodelica::nil() }), ty: ty.clone() }), sub: ssl.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn simplify2(mut inExp: Arc<DAE::Exp>, mut simplifyAddOrSub: bool, mut simplifyMulOrDiv: bool) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    ty = Expression::r#typeof(inExp.clone())?;
    if !(Expression::isIntegerOrReal(ty.clone())) {
        outExp = inExp.clone();
        return Ok(outExp);
    }
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::BINARY { operator: op, .. } if (simplifyAddOrSub.clone() && Expression::isAddOrSub(op.clone())) => {
            let mut exp_2: Arc<DAE::Exp>;
            let mut exp_3: Arc<DAE::Exp>;
            let mut resConst: Arc<DAE::Exp>;
            let mut lstConstExp: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut lstExp: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut hasConst: bool = false;
            lstExp = Expression::terms(inExp.clone())?;
            (lstConstExp, lstExp) = List::splitOnTrue(lstExp.clone(), (std::sync::Arc::new(Expression::isConstValue) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>));
            hasConst = !(lstConstExp.clone().is_empty());
            resConst = if (hasConst.clone()) {simplifyBinaryAddConstants(lstConstExp.clone())?} else {Expression::makeConstZero(ty.clone())};
            exp_2 = if (hasConst.clone()) {Expression::makeSum1(lstExp.clone(), false)?} else {inExp.clone()};
            exp_3 = simplifyBinaryCoeff(exp_2.clone())?;
            exp_3 = if (hasConst.clone()) {Expression::expAdd(resConst.clone(), simplify2(exp_3.clone(), false, true)?)?} else {simplify2(exp_3.clone(), false, true)?};
            exp_3.clone()
        },
        Deref @ DAE::Exp::BINARY { exp2: e2, operator: op, exp1: e1 } if (Expression::isAddOrSub(op.clone())) => {
            let mut e2 = (*e2).clone();
            let mut e1 = (*e1).clone();
            e1 = simplify2(e1.clone(), false, true)?;
            e2 = simplify2(e2.clone(), false, true)?;
            Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone() })
        },
        Deref @ DAE::Exp::BINARY { operator: op, .. } if (simplifyMulOrDiv.clone() && Expression::isMulOrDiv(op.clone())) => {
            let mut exp_2: Arc<DAE::Exp>;
            let mut exp_3: Arc<DAE::Exp>;
            let mut resConst: Arc<DAE::Exp>;
            let mut lstConstExp: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut lstExp: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            lstExp = Expression::factors(inExp.clone())?;
            (lstConstExp, lstExp) = List::splitOnTrue(lstExp.clone(), (std::sync::Arc::new(Expression::isConst) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>));
            if !(lstConstExp.clone().is_empty()) {
                resConst = simplifyBinaryMulConstants(lstConstExp.clone())?;
                exp_2 = Expression::makeProductLst(if (Types::isScalarReal(Expression::typeofOp(op.clone())?)) {simplifyMul(lstExp.clone())?} else {lstExp.clone()})?;
                if Expression::isConstOne(resConst.clone()) {
                    exp_3 = simplify2(exp_2.clone(), true, false)?;
                } else if Expression::isConstMinusOne(resConst.clone()) {
                    exp_3 = Expression::negate(simplify2(exp_2.clone(), true, false)?)?;
                } else {
                    exp_3 = Expression::expMul(resConst.clone(), simplify2(exp_2.clone(), true, false)?)?;
                }
            } else {
                exp_2 = simplifyBinaryCoeff(inExp.clone())?;
                exp_3 = simplify2(exp_2.clone(), true, false)?;
            }
            exp_3.clone()
        },
        Deref @ DAE::Exp::BINARY { exp2: e2, operator: op, exp1: e1 } if (Expression::isMulOrDiv(op.clone())) => {
            let mut e2 = (*e2).clone();
            let mut e1 = (*e1).clone();
            e1 = simplify2(e1.clone(), true, false)?;
            e2 = simplify2(e2.clone(), true, false)?;
            Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone() })
        },
        Deref @ DAE::Exp::BINARY { exp2: e2, operator: op, exp1: e1 } => {
            let mut e2 = (*e2).clone();
            let mut e1 = (*e1).clone();
            e1 = simplify2(e1.clone(), true, true)?;
            e2 = simplify2(e2.clone(), true, true)?;
            Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone() })
        },
        Deref @ DAE::Exp::UNARY { operator: op, exp: e1 } => {
            let mut e1 = (*e1).clone();
            e1 = simplify2(e1.clone(), true, true)?;
            Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: e1.clone() })
        },
        _ => {
            inExp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

fn simplifyBinaryArrayOp(mut inOperator: Operator) -> bool {
    let mut found: bool = false;
    found = (match inOperator.clone() {
        DAE::Operator::MUL_MATRIX_PRODUCT { .. } => true,
        DAE::Operator::ADD_ARR { .. } => true,
        DAE::Operator::SUB_ARR { .. } => true,
        DAE::Operator::MUL_ARR { .. } => true,
        DAE::Operator::DIV_ARR { .. } => true,
        DAE::Operator::POW_ARR { .. } => true,
        DAE::Operator::POW_ARR2 { .. } => true,
        DAE::Operator::MUL_ARRAY_SCALAR { .. } => true,
        DAE::Operator::ADD_ARRAY_SCALAR { .. } => true,
        DAE::Operator::DIV_ARRAY_SCALAR { .. } => true,
        DAE::Operator::POW_ARRAY_SCALAR { .. } => true,
        DAE::Operator::SUB_SCALAR_ARRAY { .. } => true,
        DAE::Operator::DIV_SCALAR_ARRAY { .. } => true,
        DAE::Operator::POW_SCALAR_ARRAY { .. } => true,
        DAE::Operator::MUL_SCALAR_PRODUCT { .. } => true,
        _ => false,
    });
    found
}

fn simplifyBinaryArray(mut inExp1: Arc<DAE::Exp>, mut inOperator2: Operator, mut inExp3: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (inExp1.clone(), inOperator2.clone(), inExp3.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, DAE::Operator::MUL_MATRIX_PRODUCT { .. }, e2) => {
                    let mut e_1: Arc<DAE::Exp>;
                    e_1 = simplifyMatrixProduct(e1.clone(), e2.clone())?;
                    Ok(e_1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, op @ DAE::Operator::ADD_ARR { .. }, e2) => {
                    let mut a1: Arc<DAE::Exp>;
                    a1 = simplifyVectorBinary0(e1.clone(), op.clone(), e2.clone())?;
                    Ok(a1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, op @ DAE::Operator::SUB_ARR { .. }, e2) => {
                    let mut a1: Arc<DAE::Exp>;
                    a1 = simplifyVectorBinary0(e1.clone(), op.clone(), e2.clone())?;
                    Ok(a1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, op @ DAE::Operator::MUL_ARR { .. }, e2) => {
                    let mut a1: Arc<DAE::Exp>;
                    a1 = simplifyVectorBinary(e1.clone(), op.clone(), e2.clone())?;
                    Ok(a1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, op @ DAE::Operator::DIV_ARR { .. }, e2) => {
                    let mut a1: Arc<DAE::Exp>;
                    a1 = simplifyVectorBinary(e1.clone(), op.clone(), e2.clone())?;
                    Ok(a1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, DAE::Operator::POW_ARR { .. }, e2) => {
                    let mut a1: Arc<DAE::Exp>;
                    let mut tp: Type = Arc::new(DAE::Type::T_NORETCALL);
                    tp = Expression::r#typeof(e1.clone())?;
                    a1 = simplifyMatrixPow(e1.clone(), tp.clone(), e2.clone())?;
                    Ok(a1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, DAE::Operator::POW_ARR2 { .. }, e2) => {
                    let mut a1: Arc<DAE::Exp>;
                    let mut tp: Type = Arc::new(DAE::Type::T_NORETCALL);
                    tp = Expression::r#typeof(e1.clone())?;
                    a1 = simplifyVectorBinary(e1.clone(), DAE::Operator::POW_ARR2 { ty: tp.clone() }, e2.clone())?;
                    Ok(a1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, DAE::Operator::SUB_ARR { ty: tp }, Deref @ DAE::Exp::UNARY { operator: _, exp: e2 }) => {
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::ADD_ARR { ty: tp.clone() }, exp2: e2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, DAE::Operator::ADD_ARR { ty: tp }, Deref @ DAE::Exp::UNARY { operator: _, exp: e2 }) => {
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::SUB_ARR { ty: tp.clone() }, exp2: e2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (a1, op, s1) => {
                    let mut op = (*op).clone();
                    let true = (Expression::isArrayScalarOp(op.clone())) else { bail!("pattern mismatch") };
                    op = unliftOperator(a1.clone(), op.clone())?;
                    Ok(simplifyVectorScalar(a1.clone(), op.clone(), s1.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (s1, op, a1) => {
                    let mut op = (*op).clone();
                    let true = (Expression::isScalarArrayOp(op.clone())) else { bail!("pattern mismatch") };
                    op = unliftOperator(a1.clone(), op.clone())?;
                    Ok(simplifyVectorScalar(s1.clone(), op.clone(), a1.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, DAE::Operator::MUL_SCALAR_PRODUCT { .. }, e2) => {
                    let mut res: Arc<DAE::Exp>;
                    res = simplifyScalarProduct(e1.clone(), e2.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, op @ DAE::Operator::ADD_ARR { .. }, e2) => {
                    let mut a1: Arc<DAE::Exp>;
                    a1 = simplifyMatrixBinary(e1.clone(), op.clone(), e2.clone())?;
                    Ok(a1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, op @ DAE::Operator::SUB_ARR { .. }, e2) => {
                    let mut a1: Arc<DAE::Exp>;
                    a1 = simplifyMatrixBinary(e1.clone(), op.clone(), e2.clone())?;
                    Ok(a1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, op @ DAE::Operator::MUL_ARR { .. }, e2) => {
                    let mut a1: Arc<DAE::Exp>;
                    a1 = simplifyMatrixBinary(e1.clone(), op.clone(), e2.clone())?;
                    Ok(a1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, op @ DAE::Operator::DIV_ARR { .. }, e2) => {
                    let mut a1: Arc<DAE::Exp>;
                    a1 = simplifyMatrixBinary(e1.clone(), op.clone(), e2.clone())?;
                    Ok(a1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, op @ DAE::Operator::POW_ARR2 { .. }, e2) => {
                    let mut a1: Arc<DAE::Exp>;
                    a1 = simplifyMatrixBinary(e1.clone(), op.clone(), e2.clone())?;
                    Ok(a1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::MUL_ARRAY_SCALAR { ty: tp }, e2) => {
                    let mut a1: Arc<DAE::Exp>;
                    let true = (Expression::isZero(e2.clone())) else { bail!("pattern mismatch") };
                    (a1, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
                    Ok(a1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, DAE::Operator::DIV_ARR { .. }, _) => {
                    let mut a1: Arc<DAE::Exp>;
                    let mut tp: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let true = (Expression::isZero(e1.clone())) else { bail!("pattern mismatch") };
                    tp = Expression::r#typeof(e1.clone())?;
                    (a1, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
                    Ok(a1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, DAE::Operator::DIV_ARRAY_SCALAR { .. }, _) => {
                    let mut a1: Arc<DAE::Exp>;
                    let mut tp: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let true = (Expression::isZero(e1.clone())) else { bail!("pattern mismatch") };
                    tp = Expression::r#typeof(e1.clone())?;
                    (a1, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
                    Ok(a1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

pub fn simplifyScalarProduct(mut inVector1: Arc<DAE::Exp>, mut inVector2: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outProduct: Arc<DAE::Exp>;
    outProduct = (::match_deref::match_deref! { match &((inVector1.clone(), inVector2.clone())) {
        (Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Nil, ty: tp, .. }, Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Nil, .. }) => {
            Expression::makeConstZero(tp.clone())
        },
        (Deref @ DAE::Exp::ARRAY { array: expl1, .. }, Deref @ DAE::Exp::ARRAY { array: expl2, .. }) => {
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut exp: Arc<DAE::Exp>;
            let true = (Expression::isVector(inVector1.clone()) && Expression::isVector(inVector2.clone())) else { bail!("pattern mismatch") };
            expl = List::threadMap(expl1.clone(), expl2.clone(), (std::sync::Arc::new(Expression::expMul) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>));
            exp = List::reduce(expl.clone(), (std::sync::Arc::new(Expression::expAdd) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?;
            exp.clone()
        },
        (Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, Deref @ DAE::Exp::CREF { componentRef: cr2, .. }) if (Config::simCodeTarget()? != literal!("Cpp")) => {
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut expl1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut expl2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut exp: Arc<DAE::Exp>;
            expl1 = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut c in (ComponentReference::expandCref(cr1.clone(), true)?).into_iter().cloned() {
            let __x = Expression::crefToExp(c.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            let true = ((expl1.clone().len() as i32) <= 3) else { bail!("pattern mismatch") };
            expl2 = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut c in (ComponentReference::expandCref(cr2.clone(), true)?).into_iter().cloned() {
            let __x = Expression::crefToExp(c.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            let true = ((expl1.clone().len() as i32) == (expl2.clone().len() as i32)) else { bail!("pattern mismatch") };
            expl = List::threadMap(expl1.clone(), expl2.clone(), (std::sync::Arc::new(Expression::expMul) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>));
            exp = List::reduce(expl.clone(), (std::sync::Arc::new(Expression::expAdd) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?;
            exp.clone()
        },
        (_, _) => {
            let true = (Expression::isZero(inVector1.clone()) || Expression::isZero(inVector2.clone())) else { bail!("pattern mismatch") };
            Expression::makeConstZero(DAE::T_REAL_DEFAULT().clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outProduct)
}

fn unliftOperator(mut inArray: Arc<DAE::Exp>, mut inOperator: Operator) -> Result<Operator> {
    let mut outOperator: Operator;
    outOperator = (::match_deref::match_deref! { match &((inArray.clone(), inOperator.clone())) {
        (Deref @ DAE::Exp::MATRIX { .. }, _) => Expression::unliftOperatorX(inOperator.clone(), 2)?,
        _ => Expression::unliftOperator(inOperator.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outOperator)
}

fn simplifyVectorScalar(mut inLhs: Arc<DAE::Exp>, mut inOperator: Operator, mut inRhs: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &((inLhs.clone(), inOperator.clone(), inRhs.clone())) {
        (_, _, Deref @ DAE::Exp::ARRAY { array: es, scalar: sc, ty: tp }) => {
            let mut es = (*es).clone();
            es = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut e in (es.clone()).into_iter().cloned() {
            let __x = Expression::makeBinaryExp(inLhs.clone(), inOperator.clone(), e.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            Arc::new(DAE::Exp::ARRAY { ty: tp.clone(), scalar: sc.clone(), array: es.clone() })
        },
        (s1, op, Deref @ DAE::Exp::MATRIX { ty: tp, integer: dims, matrix: mexpl }) => {
            let mut mexpl = (*mexpl).clone();
            mexpl = simplifyVectorScalarMatrix(mexpl.clone(), op.clone(), s1.clone(), false);
            Arc::new(DAE::Exp::MATRIX { ty: tp.clone(), integer: dims.clone(), matrix: mexpl.clone() })
        },
        (Deref @ DAE::Exp::ARRAY { array: es, scalar: sc, ty: tp }, _, _) => {
            let mut es = (*es).clone();
            es = List::map2(es.clone(), (std::sync::Arc::new(fnptr!(Expression::makeBinaryExp, Arc<DAE::Exp>, DAE::Operator, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, DAE::Operator, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), inOperator.clone(), inRhs.clone());
            Arc::new(DAE::Exp::ARRAY { ty: tp.clone(), scalar: sc.clone(), array: es.clone() })
        },
        (Deref @ DAE::Exp::MATRIX { ty: tp, integer: dims, matrix: mexpl }, op, s1) => {
            let mut mexpl = (*mexpl).clone();
            mexpl = simplifyVectorScalarMatrix(mexpl.clone(), op.clone(), s1.clone(), true);
            Arc::new(DAE::Exp::MATRIX { ty: tp.clone(), integer: dims.clone(), matrix: mexpl.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

fn simplifyVectorBinary0(mut e1: Arc<DAE::Exp>, mut op: Operator, mut e2: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut res: Arc<DAE::Exp>;
    res = 'mc: {
        let __mc_input = (e1.clone(), op.clone(), e2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut a1: Arc<DAE::Exp>;
                    a1 = simplifyVectorBinary(e1.clone(), op.clone(), e2.clone())?;
                    Ok(a1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::ADD { .. }, _) => {
                    let true = (Expression::isZero(e1.clone())) else { bail!("pattern mismatch") };
                    Ok(e2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::ADD_ARR { .. }, _) => {
                    let true = (Expression::isZero(e1.clone())) else { bail!("pattern mismatch") };
                    Ok(e2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::SUB_ARR { .. }, _) => {
                    let true = (Expression::isZero(e1.clone())) else { bail!("pattern mismatch") };
                    Ok(Expression::negate(e2.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::SUB { .. }, _) => {
                    let true = (Expression::isZero(e1.clone())) else { bail!("pattern mismatch") };
                    Ok(Expression::negate(e2.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Expression::isZero(e2.clone())) else { bail!("pattern mismatch") };
                    Ok(e1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(res)
}

fn simplifyVectorBinary(mut inLhs: Arc<DAE::Exp>, mut inOperator: Operator, mut inRhs: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outResult: Arc<DAE::Exp>;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut sc: bool = false;
    let mut lhs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut rhs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut res: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut op: Operator;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inLhs.clone()) {
        Deref @ DAE::Exp::ARRAY { array: __pa0, scalar: __pa1, ty: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    lhs = __pa0.clone();
    sc = __pa1.clone();
    ty = __pa2.clone();
    let __pa3 = ::match_deref::match_deref! { match &(inRhs.clone()) {
        Deref @ DAE::Exp::ARRAY { array: __pa3, .. } => __pa3.clone(),
        _ => bail!("pattern mismatch"),
    } };
    rhs = __pa3.clone();
    op = removeOperatorDimension(inOperator.clone())?;
    res = List::threadMap1(lhs.clone(), rhs.clone(), (std::sync::Arc::new(fnptr!(simplifyVectorBinary2, Arc<DAE::Exp>, Arc<DAE::Exp>, DAE::Operator)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>, DAE::Operator) -> Result<Arc<DAE::Exp>> + 'static>), op.clone());
    outResult = Arc::new(DAE::Exp::ARRAY { ty: ty.clone(), scalar: sc.clone(), array: res.clone() });
    Ok(outResult)
}

fn simplifyVectorBinary2(mut inLhs: Arc<DAE::Exp>, mut inRhs: Arc<DAE::Exp>, mut inOperator: Operator) -> Arc<DAE::Exp> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = Arc::new(DAE::Exp::BINARY { exp1: inLhs.clone(), operator: inOperator.clone(), exp2: inRhs.clone() });
    outExp
}

fn simplifyMatrixBinary(mut inLhs: Arc<DAE::Exp>, mut inOperator: Operator, mut inRhs: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outResult: Arc<DAE::Exp>;
    let mut lhs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
    let mut rhs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
    let mut res: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
    let mut op: Operator;
    let mut sz: i32 = 0;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    lhs = Expression::get2dArrayOrMatrixContent(inLhs.clone())?;
    rhs = Expression::get2dArrayOrMatrixContent(inRhs.clone())?;
    op = removeOperatorDimension(inOperator.clone())?;
    res = List::threadMap1(lhs.clone(), rhs.clone(), (std::sync::Arc::new(fnptr!(simplifyMatrixBinary1, Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<DAE::Exp>>>, DAE::Operator)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<DAE::Exp>>>, DAE::Operator) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>), op.clone());
    sz = (res.clone().len() as i32);
    ty = Expression::r#typeof(inLhs.clone())?;
    outResult = Arc::new(DAE::Exp::MATRIX { ty: ty.clone(), integer: sz.clone(), matrix: res.clone() });
    Ok(outResult)
}

fn simplifyMatrixBinary1(mut inLhs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inRhs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inOperator: Operator) -> Arc<metamodelica::List<Arc<DAE::Exp>>> {
    let mut outExpl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    outExpl = List::threadMap1(inLhs.clone(), inRhs.clone(), (std::sync::Arc::new(simplifyMatrixBinary2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>, DAE::Operator) -> Result<Arc<DAE::Exp>> + 'static>), inOperator.clone());
    outExpl
}

fn simplifyMatrixBinary2(mut inLhs: Arc<DAE::Exp>, mut inRhs: Arc<DAE::Exp>, mut inOperator: Operator) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let mut op: Operator;
    op = removeOperatorDimension(inOperator.clone())?;
    outExp = Arc::new(DAE::Exp::BINARY { exp1: inLhs.clone(), operator: op.clone(), exp2: inRhs.clone() });
    Ok(outExp)
}

fn simplifyMatrixPow(mut inExp1: Arc<DAE::Exp>, mut inType: Type, mut inExp2: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (inExp1.clone(), inType.clone(), inExp2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::MATRIX { integer: size1, ty: tp1, .. }, _, Deref @ DAE::Exp::ICONST { integer: i }) => {
                    let mut expl_1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
                    let mut expl2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
                    let mut el: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut range: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let 0 = (i.clone()) else { bail!("pattern mismatch") };
                    el = List::fill(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), size1.clone());
                    expl2 = List::fill(el.clone(), size1.clone());
                    range = List::intRange2(0, size1.clone() - 1);
                    expl_1 = simplifyMatrixPow1(range.clone(), expl2.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }))?;
                    Ok(Arc::new(DAE::Exp::MATRIX { ty: tp1.clone(), integer: size1.clone(), matrix: expl_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (m @ Deref @ DAE::Exp::MATRIX { .. }, _, Deref @ DAE::Exp::ICONST { integer: i }) => {
                    let 1 = (i.clone()) else { bail!("pattern mismatch") };
                    Ok(m.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (m @ Deref @ DAE::Exp::MATRIX { .. }, _, Deref @ DAE::Exp::ICONST { integer: i }) => {
                    let mut res: Arc<DAE::Exp>;
                    let 2 = (i.clone()) else { bail!("pattern mismatch") };
                    res = simplifyMatrixProduct(m.clone(), m.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (m @ Deref @ DAE::Exp::MATRIX { ty: tp1, .. }, _, Deref @ DAE::Exp::ICONST { integer: i }) => {
                    let mut i_1: i32 = 0;
                    let mut e: Arc<DAE::Exp>;
                    let mut res: Arc<DAE::Exp>;
                    let true = (i.clone() > 3) else { bail!("pattern mismatch") };
                    let 0 = (intMod(i.clone(), 2)) else { bail!("pattern mismatch") };
                    i_1 = intDiv(i.clone(), 2);
                    e = simplifyMatrixPow(m.clone(), tp1.clone(), Arc::new(DAE::Exp::ICONST { integer: i_1.clone() }))?;
                    res = simplifyMatrixProduct(e.clone(), e.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (m @ Deref @ DAE::Exp::MATRIX { ty: tp1, .. }, _, Deref @ DAE::Exp::ICONST { integer: i }) => {
                    let mut i_1: i32 = 0;
                    let mut e: Arc<DAE::Exp>;
                    let mut res: Arc<DAE::Exp>;
                    let true = (1 < i.clone()) else { bail!("pattern mismatch") };
                    i_1 = i.clone() - 1;
                    e = simplifyMatrixPow(m.clone(), tp1.clone(), Arc::new(DAE::Exp::ICONST { integer: i_1.clone() }))?;
                    res = simplifyMatrixProduct(m.clone(), e.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

fn simplifyMatrixPow1(mut inRange: Arc<metamodelica::List<i32>>, mut inMatrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, mut inValue: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>> {
    let mut outMatrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
    outMatrix = 'mc: {
        let __mc_input = (inRange.clone(), inMatrix.clone(), inValue.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: i, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: row, tail: Deref @ metamodelica::List::Nil }, e) => {
                    let mut row1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    row1 = List::replaceAt(e.clone(), i.clone() + 1, row.clone())?;
                    Ok(list![row1.clone()])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: i, tail: rr }, Deref @ metamodelica::List::Cons { head: row, tail: rm }, e) => {
                    let mut rm1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
                    let mut row1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    row1 = List::replaceAt(e.clone(), i.clone() + 1, row.clone())?;
                    rm1 = simplifyMatrixPow1(rr.clone(), rm.clone(), e.clone())?;
                    Ok(cons(row1.clone(), rm1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outMatrix)
}

fn simplifyMatrixProduct(mut inMatrix1: Arc<DAE::Exp>, mut inMatrix2: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outProduct: Arc<DAE::Exp>;
    let mut mat1: Arc<DAE::Exp>;
    let mut mat2: Arc<DAE::Exp>;
    mat1 = Expression::matrixToArray(inMatrix1.clone())?;
    mat2 = Expression::matrixToArray(inMatrix2.clone())?;
    (mat2, _) = Expression::transposeArray(mat2.clone())?;
    outProduct = simplifyMatrixProduct2(mat1.clone(), mat2.clone())?;
    Ok(outProduct)
}

fn simplifyMatrixProduct2(mut inMatrix1: Arc<DAE::Exp>, mut inMatrix2: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outProduct: Arc<DAE::Exp>;
    outProduct = 'mc: {
        let __mc_input = (inMatrix1.clone(), inMatrix2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { ty: ty @ Deref @ DAE::Type::T_ARRAY { dims, .. }, .. }, Deref @ DAE::Exp::ARRAY { .. }) => {
                    let mut zero: Arc<DAE::Exp>;
                    let mut dims = (*dims).clone();
                    let true = (Expression::arrayContainZeroDimension(dims.clone())) else { bail!("pattern mismatch") };
                    zero = Expression::makeConstZero(ty.clone());
                    dims = simplifyMatrixProduct4(inMatrix1.clone(), inMatrix2.clone())?;
                    Ok(Expression::arrayFill(dims.clone(), zero.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { array: expl1, ty: Deref @ DAE::Type::T_ARRAY { ty, dims: Deref @ metamodelica::List::Cons { head: n, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }, .. }, Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, .. }, .. }) => {
                    let mut expl1 = (*expl1).clone();
                    let mut ty = (*ty).clone();
                    expl1 = List::map1(expl1.clone(), (std::sync::Arc::new(simplifyScalarProduct) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), inMatrix2.clone());
                    ty = Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![n.clone()] });
                    Ok(Arc::new(DAE::Exp::ARRAY { ty: ty.clone(), scalar: true, array: expl1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, .. }, .. }, Deref @ DAE::Exp::ARRAY { array: expl2, ty: Deref @ DAE::Type::T_ARRAY { ty, dims: Deref @ metamodelica::List::Cons { head: m, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }, .. }) => {
                    let mut expl1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut ty = (*ty).clone();
                    expl1 = List::map1r(expl2.clone(), (std::sync::Arc::new(simplifyScalarProduct) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), inMatrix1.clone());
                    ty = Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![m.clone()] });
                    Ok(Arc::new(DAE::Exp::ARRAY { ty: ty.clone(), scalar: true, array: expl1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { array: expl1, ty: Deref @ DAE::Type::T_ARRAY { ty, dims: Deref @ metamodelica::List::Cons { head: n, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }, .. }, Deref @ DAE::Exp::ARRAY { array: expl2, ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: p, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, .. }, .. }) => {
                    let mut row_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
                    let mut expl1 = (*expl1).clone();
                    matrix = List::map1(expl1.clone(), (std::sync::Arc::new(fnptr!(simplifyMatrixProduct3, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Exp>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>), expl2.clone());
                    row_ty = Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![p.clone()] });
                    expl1 = List::map2(matrix.clone(), (std::sync::Arc::new(fnptr!(Expression::makeArray, Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<DAE::Type>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<DAE::Type>, bool) -> Result<Arc<DAE::Exp>> + 'static>), row_ty.clone(), true);
                    Ok(Arc::new(DAE::Exp::ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![n.clone(), p.clone()] }), scalar: false, array: expl1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outProduct)
}

fn simplifyMatrixProduct3(mut inRow: Arc<DAE::Exp>, mut inMatrix: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Arc<metamodelica::List<Arc<DAE::Exp>>> {
    let mut outRow: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    outRow = List::map1r(inMatrix.clone(), (std::sync::Arc::new(simplifyScalarProduct) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), inRow.clone());
    outRow
}

fn simplifyMatrixProduct4(mut inMatrix1: Arc<DAE::Exp>, mut inMatrix2: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Dimension>>>> {
    let mut outDimensions: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    outDimensions = (::match_deref::match_deref! { match &((inMatrix1.clone(), inMatrix2.clone())) {
        (Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: n, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, .. }, .. }, Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, .. }, .. }) => {
            list![n.clone()]
        },
        (Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, .. }, .. }, Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: m, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, .. }, .. }) => {
            list![m.clone()]
        },
        (Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: n, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, .. }, .. }, Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: p, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, .. }, .. }) => {
            list![n.clone(), p.clone()]
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outDimensions)
}

fn simplifyBinarySortConstants(mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e @ Deref @ DAE::Exp::BINARY { operator: DAE::Operator::MUL { .. }, .. } => {
                    let mut res: Arc<DAE::Exp>;
                    res = simplifyBinarySortConstantsMul(e.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::DIV { ty: tp }, exp1: e1 } => {
                    let mut e2 = (*e2).clone();
                    let mut e1 = (*e1).clone();
                    e1 = simplifyBinarySortConstantsMul(e1.clone())?;
                    e2 = simplifyBinarySortConstantsMul(e2.clone())?;
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: e2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e @ Deref @ DAE::Exp::BINARY { operator: DAE::Operator::ADD { .. }, .. } => {
                    let mut e_lst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut const_es1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut notconst_es1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut res: Arc<DAE::Exp>;
                    let mut res1: Arc<DAE::Exp>;
                    let mut res2: Arc<DAE::Exp>;
                    e_lst = Expression::terms(e.clone())?;
                    (const_es1, notconst_es1) = List::splitOnTrue(e_lst.clone(), (std::sync::Arc::new(Expression::isConstValue) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>));
                    if !(const_es1.clone().is_empty()) {
                        res1 = simplifyBinaryAddConstants(const_es1.clone())?;
                        res2 = Expression::makeSum1(notconst_es1.clone(), false)?;
                        res = Expression::expAdd(res1.clone(), res2.clone())?;
                    } else {
                        res = inExp.clone();
                    }
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

fn simplifyBinaryCoeff(mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e @ Deref @ DAE::Exp::BINARY { operator: DAE::Operator::MUL { ty: tp }, .. } => {
                    if !((Types::isScalarReal(tp.clone()))) { bail!("guard") }
                    let mut e_lst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut e_lst_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut res: Arc<DAE::Exp>;
                    e_lst = Expression::factors(e.clone())?;
                    e_lst_1 = simplifyMul(e_lst.clone())?;
                    res = Expression::makeProductLst(e_lst_1.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::DIV { .. }, exp1: e1 } => {
                    let mut e_lst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut e_lst_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut e1_lst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut e2_lst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut e2_lst_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut res: Arc<DAE::Exp>;
                    let false = (Expression::isZero(e2.clone())) else { bail!("pattern mismatch") };
                    e1_lst = Expression::factors(e1.clone())?;
                    e2_lst = Expression::factors(e2.clone())?;
                    e2_lst_1 = List::map(e2_lst.clone(), (std::sync::Arc::new(Expression::inverseFactors) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>));
                    e_lst = listAppend(e1_lst.clone(), e2_lst_1.clone());
                    e_lst_1 = simplifyMul(e_lst.clone())?;
                    res = Expression::makeProductLst(e_lst_1.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e @ Deref @ DAE::Exp::BINARY { operator: DAE::Operator::ADD { .. }, .. } => {
                    let mut e_lst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut e_lst_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut res: Arc<DAE::Exp>;
                    e_lst = Expression::terms(e.clone())?;
                    e_lst_1 = simplifyAdd(e_lst.clone())?;
                    res = Expression::makeSum(e_lst_1.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::SUB { .. }, exp1: e1 } => {
                    let mut e_lst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut e_lst_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut e1_lst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut e2_lst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut res: Arc<DAE::Exp>;
                    e1_lst = Expression::terms(e1.clone())?;
                    e2_lst = Expression::terms(e2.clone())?;
                    e2_lst = List::map(e2_lst.clone(), (std::sync::Arc::new(Expression::negate) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>));
                    e_lst = listAppend(e1_lst.clone(), e2_lst.clone());
                    e_lst_1 = simplifyAdd(e_lst.clone())?;
                    res = Expression::makeSum(e_lst_1.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

fn simplifyBinaryAddConstants(mut inExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let mut tp: Type = Arc::new(DAE::Type::T_NORETCALL);
    let mut es: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExpLst.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outExp = __pa0.clone();
    es = __pa1.clone();
    tp = Expression::r#typeof(outExp.clone())?;
    for mut e in &*es.clone() {
        let mut e = e.clone();
        outExp = simplifyBinaryConst(DAE::Operator::ADD { ty: tp.clone() }, outExp.clone(), e.clone())?;
    }
    Ok(outExp)
}

fn simplifyBinaryMulConstants(mut inExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let mut es: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut tp: Type = Arc::new(DAE::Type::T_NORETCALL);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExpLst.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outExp = __pa0.clone();
    es = __pa1.clone();
    tp = Expression::r#typeof(outExp.clone())?;
    for mut e in &*es.clone() {
        let mut e = e.clone();
        outExp = simplifyBinaryConst(DAE::Operator::MUL { ty: tp.clone() }, outExp.clone(), e.clone())?;
    }
    Ok(outExp)
}

fn simplifyMul(mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut exp_const: Arc<metamodelica::List<(Arc<DAE::Exp>, metamodelica::Real)>> = metamodelica::nil();
    let mut exp_const_1: Arc<metamodelica::List<(Arc<DAE::Exp>, metamodelica::Real)>> = metamodelica::nil();
    exp_const = List::map(expl.clone(), (std::sync::Arc::new(simplifyBinaryMulCoeff2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<(Arc<DAE::Exp>, metamodelica::Real)> + 'static>));
    exp_const_1 = simplifyMulJoinFactors(exp_const.clone())?;
    expl_1 = simplifyMulMakePow(exp_const_1.clone());
    Ok(expl_1)
}

fn simplifyMulJoinFactors(mut inTplExpRealLst: Arc<metamodelica::List<(Arc<DAE::Exp>, metamodelica::Real)>>) -> Result<Arc<metamodelica::List<(Arc<DAE::Exp>, metamodelica::Real)>>> {
    let mut outTplExpRealLst: Arc<metamodelica::List<(Arc<DAE::Exp>, metamodelica::Real)>> = metamodelica::nil();
    let mut tplExpRealLst: Arc<metamodelica::List<(Arc<DAE::Exp>, metamodelica::Real)>> = inTplExpRealLst.clone();
    let mut e: Arc<DAE::Exp>;
    let mut coeff: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut coeff2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    while !(tplExpRealLst.clone().is_empty()) {
        let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(tplExpRealLst.clone()) {
            Deref @ metamodelica::List::Cons { head: (__pa0, __pa1), tail: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa0.clone();
        coeff = __pa1.clone();
        tplExpRealLst = __pa2.clone();
        (coeff2, tplExpRealLst) = simplifyMulJoinFactorsFind(e.clone(), tplExpRealLst.clone())?;
        coeff = coeff.clone() + coeff2.clone();
        outTplExpRealLst = cons((e.clone(), coeff.clone()), outTplExpRealLst.clone());
    }
    Ok(outTplExpRealLst)
}

fn simplifyMulJoinFactorsFind(mut inExp: Arc<DAE::Exp>, mut inTplExpRealLst: Arc<metamodelica::List<(Arc<DAE::Exp>, metamodelica::Real)>>) -> Result<(metamodelica::Real, Arc<metamodelica::List<(Arc<DAE::Exp>, metamodelica::Real)>>)> {
    let mut outReal: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut outTplExpRealLst: Arc<metamodelica::List<(Arc<DAE::Exp>, metamodelica::Real)>> = metamodelica::nil();
    let mut tplExpReal: (Arc<DAE::Exp>, metamodelica::Real);
    for mut tplExpReal in &*inTplExpRealLst.clone() {
        let mut tplExpReal = tplExpReal.clone();
        (outReal, outTplExpRealLst) = (::match_deref::match_deref! { match &(tplExpReal.clone()) {
        (e2, coeff) if (ExpressionBasics::expEqual(inExp.clone(), e2.clone())?) => {
            (coeff.clone() + outReal.clone(), outTplExpRealLst.clone())
        },
        (Deref @ DAE::Exp::BINARY { exp2: e2, operator: op @ DAE::Operator::DIV { .. }, exp1: e1 }, coeff) if (if (Expression::isOne(e1.clone())) {ExpressionBasics::expEqual(inExp.clone(), e2.clone())?} else {ExpressionBasics::expEqual(inExp.clone(), Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op.clone(), exp2: e1.clone() }))?}) => {
            (outReal.clone() - coeff.clone(), outTplExpRealLst.clone())
        },
        _ => {
            (outReal.clone(), cons(tplExpReal.clone(), outTplExpRealLst.clone()))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    outTplExpRealLst = outTplExpRealLst.clone().reverse();
    Ok((outReal, outTplExpRealLst))
}

fn simplifyMulMakePow(mut inTplExpRealLst: Arc<metamodelica::List<(Arc<DAE::Exp>, metamodelica::Real)>>) -> Arc<metamodelica::List<Arc<DAE::Exp>>> {
    let mut outExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut tplExpReal: (Arc<DAE::Exp>, metamodelica::Real);
    let mut e: Arc<DAE::Exp>;
    let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    for mut tplExpReal in &*inTplExpRealLst.clone() {
        let mut tplExpReal = tplExpReal.clone();
        (e, r) = tplExpReal.clone();
        outExpLst = if (r.clone() == metamodelica::OrderedFloat(1.0_f64)) {cons(e.clone(), outExpLst.clone())} else {cons(Arc::new(DAE::Exp::BINARY { exp1: e.clone(), operator: DAE::Operator::POW { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: r.clone() }) }), outExpLst.clone())};
    }
    outExpLst
}

fn simplifyAdd(mut inExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut coeffs: Arc<metamodelica::List<(Arc<DAE::Exp>, metamodelica::Real)>> = metamodelica::nil();
    match '__try0: {
        coeffs = List::map(inExpLst.clone(), (std::sync::Arc::new(simplifyBinaryAddCoeff2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<(Arc<DAE::Exp>, metamodelica::Real)> + 'static>));
        coeffs = unwrap_break_err!(simplifyAddJoinTerms(coeffs.clone()), '__try0);
        outExpLst = unwrap_break_err!(simplifyAddMakeMul(coeffs.clone()), '__try0);
        Ok::<_, anyhow::Error>((coeffs.clone(), outExpLst.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            coeffs = __try0_o0;
            outExpLst = __try0_o1;
        }
        Err(_) => {
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::trace((literal!("- ExpressionSimplify.simplifyAdd failed\n")).clone())?;
            }
            bail!("fail");
        }
    }
    Ok(outExpLst)
}

fn simplifyAddJoinTerms(mut inTplExpRealLst: Arc<metamodelica::List<(Arc<DAE::Exp>, metamodelica::Real)>>) -> Result<Arc<metamodelica::List<(Arc<DAE::Exp>, metamodelica::Real)>>> {
    fn addCoeff(mut oldCoeff: Option<metamodelica::Real>, mut newCoeff: metamodelica::Real) -> Result<metamodelica::Real> {
        let mut coeff: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
        coeff = if (isSome(oldCoeff.clone())) {Util::getOption(oldCoeff.clone())? + newCoeff.clone()} else {newCoeff.clone()};
        Ok(coeff)
    }

    let mut outTplExpRealLst: Arc<metamodelica::List<(Arc<DAE::Exp>, metamodelica::Real)>> = metamodelica::nil();
    outTplExpRealLst = (::match_deref::match_deref! { match &(inTplExpRealLst.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } => {
            inTplExpRealLst.clone()
        },
        Deref @ metamodelica::List::Cons { head: (exp1, coeff1), tail: Deref @ metamodelica::List::Cons { head: (exp2, coeff2), tail: Deref @ metamodelica::List::Nil } } => {
            if (ExpressionBasics::expEqual(exp1.clone(), exp2.clone())?) {list![(exp1.clone(), coeff1.clone() + coeff2.clone())]} else {inTplExpRealLst.clone()}
        },
        Deref @ metamodelica::List::Cons { head: (exp1, coeff1), tail: Deref @ metamodelica::List::Cons { head: (exp2, coeff2), tail: Deref @ metamodelica::List::Cons { head: (exp3, coeff3), tail: Deref @ metamodelica::List::Nil } } } => {
            if ExpressionBasics::expEqual(exp1.clone(), exp2.clone())? {
                if ExpressionBasics::expEqual(exp1.clone(), exp3.clone())? {
                    outTplExpRealLst = list![(exp1.clone(), coeff1.clone() + coeff2.clone() + coeff3.clone())];
                } else {
                    outTplExpRealLst = list![(exp1.clone(), coeff1.clone() + coeff2.clone()), (exp3.clone(), coeff3.clone())];
                }
            } else if ExpressionBasics::expEqual(exp1.clone(), exp3.clone())? {
                outTplExpRealLst = list![(exp1.clone(), coeff1.clone() + coeff3.clone()), (exp2.clone(), coeff2.clone())];
            } else if ExpressionBasics::expEqual(exp2.clone(), exp3.clone())? {
                outTplExpRealLst = list![(exp1.clone(), coeff1.clone()), (exp2.clone(), coeff2.clone() + coeff3.clone())];
            } else {
                outTplExpRealLst = inTplExpRealLst.clone();
            }
            outTplExpRealLst.clone()
        },
        _ => {
            let mut exp1: Arc<DAE::Exp>;
            let mut coeff1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut coeff_map: Arc<UnorderedMap::UnorderedMap<Arc<DAE::Exp>, metamodelica::Real>>;
            coeff_map = UnorderedMap::new((std::sync::Arc::new(Expression::hashExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>), (std::sync::Arc::new(ExpressionBasics::expEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>), (inTplExpRealLst.clone().len() as i32));
            for mut tpl in &*inTplExpRealLst.clone() {
                let mut tpl = tpl.clone();
                (exp1, coeff1) = tpl.clone();
                UnorderedMap::addUpdate(exp1.clone(), Arc::new({ let __pe_b1 = coeff1.clone(); move |__pe_a0| addCoeff(__pe_a0, __pe_b1.clone()) }), coeff_map.clone())?;
            }
            UnorderedMap::toList(coeff_map.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outTplExpRealLst)
}

fn simplifyAddMakeMul(mut inTplExpRealLst: Arc<metamodelica::List<(Arc<DAE::Exp>, metamodelica::Real)>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    outExpLst = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut tplExpReal in (inTplExpRealLst.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(tplExpReal.clone()) {
        (e, __rlit_2) if __rlit_2.eq(&metamodelica::OrderedFloat((1.0) as f64)) => {
            e.clone()
        },
        (e, __rlit_3) if __rlit_3.eq(&metamodelica::OrderedFloat((-1.0) as f64)) => {
            Expression::negate(e.clone())?
        },
        (e, r) => {
            (::match_deref::match_deref! { match &(Expression::r#typeof(e.clone())?) {
        Deref @ DAE::Type::T_INTEGER { .. } => Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::ICONST { integer: ((r.clone()).0 as i32) }), operator: DAE::Operator::MUL { ty: DAE::T_INTEGER_DEFAULT().clone() }, exp2: e.clone() }),
        _ => Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: r.clone() }), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(outExpLst)
}

fn simplifyBinaryAddCoeff2(mut inExp: Arc<DAE::Exp>) -> Result<(Arc<DAE::Exp>, metamodelica::Real)> {
    let mut outRes: (Arc<DAE::Exp>, metamodelica::Real);
    outRes = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CREF { .. } => {
            (inExp.clone(), metamodelica::OrderedFloat(1.0_f64))
        },
        Deref @ DAE::Exp::UNARY { exp, operator: DAE::Operator::UMINUS { ty: Deref @ DAE::Type::T_REAL { .. } } } => {
            let mut coeff: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut exp = (*exp).clone();
            (exp, coeff) = simplifyBinaryAddCoeff2(exp.clone())?;
            coeff = (metamodelica::OrderedFloat(-1.0_f64)) * (coeff.clone());
            (exp.clone(), coeff.clone())
        },
        Deref @ DAE::Exp::BINARY { exp2: e1, operator: DAE::Operator::MUL { .. }, exp1: Deref @ DAE::Exp::RCONST { real: coeff } } => {
            (e1.clone(), coeff.clone())
        },
        Deref @ DAE::Exp::BINARY { exp2: Deref @ DAE::Exp::RCONST { real: coeff }, operator: DAE::Operator::MUL { .. }, exp1: e1 } => {
            (e1.clone(), coeff.clone())
        },
        Deref @ DAE::Exp::BINARY { exp2: Deref @ DAE::Exp::ICONST { integer: icoeff }, operator: DAE::Operator::MUL { .. }, exp1: e1 } => {
            let mut coeff_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            coeff_1 = intReal(icoeff.clone());
            (e1.clone(), coeff_1.clone())
        },
        Deref @ DAE::Exp::BINARY { exp2: e1, operator: DAE::Operator::MUL { .. }, exp1: Deref @ DAE::Exp::ICONST { integer: icoeff } } => {
            let mut coeff_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            coeff_1 = intReal(icoeff.clone());
            (e1.clone(), coeff_1.clone())
        },
        Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::ADD { .. }, exp1: e1 } if (ExpressionBasics::expEqual(e1.clone(), e2.clone())?) => {
            (e1.clone(), metamodelica::OrderedFloat(2.0_f64))
        },
        _ => {
            (inExp.clone(), metamodelica::OrderedFloat(1.0_f64))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outRes)
}

fn simplifyBinaryMulCoeff2(mut inExp: Arc<DAE::Exp>) -> Result<(Arc<DAE::Exp>, metamodelica::Real)> {
    let mut outRes: (Arc<DAE::Exp>, metamodelica::Real);
    outRes = (::match_deref::match_deref! { match &(inExp.clone()) {
        e @ Deref @ DAE::Exp::CREF { .. } => {
            (e.clone(), metamodelica::OrderedFloat(1.0_f64))
        },
        Deref @ DAE::Exp::BINARY { exp2: Deref @ DAE::Exp::RCONST { real: coeff }, operator: DAE::Operator::POW { .. }, exp1: e1 } => {
            (e1.clone(), coeff.clone())
        },
        Deref @ DAE::Exp::BINARY { exp2: Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::RCONST { real: coeff }, operator: DAE::Operator::UMINUS { .. } }, operator: DAE::Operator::POW { .. }, exp1: e1 } => {
            let mut coeff_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            coeff_1 = metamodelica::OrderedFloat(0.0_f64) - coeff.clone();
            (e1.clone(), coeff_1.clone())
        },
        Deref @ DAE::Exp::BINARY { exp2: Deref @ DAE::Exp::ICONST { integer: icoeff }, operator: DAE::Operator::POW { .. }, exp1: e1 } => {
            let mut coeff_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            coeff_1 = intReal(icoeff.clone());
            (e1.clone(), coeff_1.clone())
        },
        Deref @ DAE::Exp::BINARY { exp2: Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::ICONST { integer: icoeff }, operator: DAE::Operator::UMINUS { .. } }, operator: DAE::Operator::POW { .. }, exp1: e1 } => {
            let mut coeff_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            coeff_1 = metamodelica::OrderedFloat(0.0_f64) - intReal(icoeff.clone());
            (e1.clone(), coeff_1.clone())
        },
        Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::MUL { .. }, exp1: e1 } if (ExpressionBasics::expEqual(e1.clone(), e2.clone())?) => {
            (e1.clone(), metamodelica::OrderedFloat(2.0_f64))
        },
        _ => {
            (inExp.clone(), metamodelica::OrderedFloat(1.0_f64))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outRes)
}

pub fn simplifySumOperatorExpression(mut iSum: Arc<DAE::Exp>, mut iop: DAE::Operator, mut iExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut oExp: Arc<DAE::Exp>;
    let mut T: Arc<metamodelica::List<Arc<DAE::Exp>>> = Expression::termsExpandUnary(iSum.clone())?;
    let mut b: bool = false;
    let mut e: Arc<DAE::Exp>;
    let mut newE: Arc<DAE::Exp>;
    let mut sE: Arc<DAE::Exp>;
    let mut tp: Arc<DAE::Type> = Expression::typeofOp(iop.clone())?;
    oExp = Expression::makeConstZero(tp.clone());
    sE = oExp.clone();
    for mut elem in &*T.clone() {
        let mut elem = elem.clone();
        e = Arc::new(DAE::Exp::BINARY { exp1: elem.clone(), operator: iop.clone(), exp2: iExp.clone() });
        newE = simplifyBinaryCoeff(e.clone())?;
        b = !(ExpressionBasics::expEqual(e.clone(), newE.clone())?);
        if b.clone() {
            sE = Expression::expAdd(sE.clone(), newE.clone())?;
        } else {
            oExp = Expression::expAdd(oExp.clone(), elem.clone())?;
        }
    }
    e = Arc::new(DAE::Exp::BINARY { exp1: oExp.clone(), operator: iop.clone(), exp2: iExp.clone() });
    oExp = Expression::expAdd(sE.clone(), e.clone())?;
    Ok(oExp)
}

fn simplifyAsub0(mut ie: Arc<DAE::Exp>, mut sub: i32, mut inSubExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut res: Arc<DAE::Exp>;
    res = (::match_deref::match_deref! { match &(ie.clone()) {
        Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: exps } => {
            let mut exp: Arc<DAE::Exp>;
            exp = (exps.clone()).get(sub.clone())?;
            exp.clone()
        },
        Deref @ DAE::Exp::RANGE { stop: Deref @ DAE::Exp::BCONST { bool: bstop }, start: Deref @ DAE::Exp::BCONST { bool: bstart }, .. } => {
            let mut b: bool = false;
            b = (simplifyRangeBool(bstart.clone(), bstop.clone())).get(sub.clone())?;
            Arc::new(DAE::Exp::BCONST { bool: b.clone() })
        },
        Deref @ DAE::Exp::RANGE { stop: Deref @ DAE::Exp::ICONST { integer: istop }, step: None, start: Deref @ DAE::Exp::ICONST { integer: istart }, .. } => {
            let mut exp: Arc<DAE::Exp>;
            let mut ival: i32 = 0;
            ival = (simplifyRange(istart.clone(), 1, istop.clone())?).get(sub.clone())?;
            exp = Arc::new(DAE::Exp::ICONST { integer: ival.clone() });
            exp.clone()
        },
        Deref @ DAE::Exp::RANGE { stop: Deref @ DAE::Exp::ICONST { integer: istop }, step: Some(Deref @ DAE::Exp::ICONST { integer: istep }), start: Deref @ DAE::Exp::ICONST { integer: istart }, .. } => {
            let mut exp: Arc<DAE::Exp>;
            let mut ival: i32 = 0;
            ival = (simplifyRange(istart.clone(), istep.clone(), istop.clone())?).get(sub.clone())?;
            exp = Arc::new(DAE::Exp::ICONST { integer: ival.clone() });
            exp.clone()
        },
        Deref @ DAE::Exp::RANGE { stop: Deref @ DAE::Exp::RCONST { real: rstop }, step: None, start: Deref @ DAE::Exp::RCONST { real: rstart }, .. } => {
            let mut exp: Arc<DAE::Exp>;
            let mut rval: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            rval = (simplifyRangeReal(rstart.clone(), metamodelica::OrderedFloat(1.0_f64), rstop.clone())?).get(sub.clone())?;
            exp = Arc::new(DAE::Exp::RCONST { real: rval.clone() });
            exp.clone()
        },
        Deref @ DAE::Exp::RANGE { stop: Deref @ DAE::Exp::RCONST { real: rstop }, step: Some(Deref @ DAE::Exp::RCONST { real: rstep }), start: Deref @ DAE::Exp::RCONST { real: rstart }, .. } => {
            let mut exp: Arc<DAE::Exp>;
            let mut rval: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            rval = (simplifyRangeReal(rstart.clone(), rstep.clone(), rstop.clone())?).get(sub.clone())?;
            exp = Arc::new(DAE::Exp::RCONST { real: rval.clone() });
            exp.clone()
        },
        Deref @ DAE::Exp::MATRIX { ty: t, integer: _, matrix: mexps } => {
            let mut t1: Type = Arc::new(DAE::Type::T_NORETCALL);
            let mut mexpl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            t1 = Expression::unliftArray(t.clone())?;
            mexpl = (mexps.clone()).get(sub.clone())?;
            Arc::new(DAE::Exp::ARRAY { ty: t1.clone(), scalar: true, array: mexpl.clone() })
        },
        Deref @ DAE::Exp::IFEXP { expCond: cond, expThen: e1, expElse: e2 } => {
            let mut e: Arc<DAE::Exp>;
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            e1 = Expression::makeASUB(e1.clone(), list![inSubExp.clone()])?;
            e2 = Expression::makeASUB(e2.clone(), list![inSubExp.clone()])?;
            e = Arc::new(DAE::Exp::IFEXP { expCond: cond.clone(), expThen: e1.clone(), expElse: e2.clone() });
            e.clone()
        },
        Deref @ DAE::Exp::CREF { componentRef: c, ty: t } => {
            let mut exp: Arc<DAE::Exp>;
            let mut c_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut t = (*t).clone();
            let true = (Types::isArray(t.clone())) else { bail!("pattern mismatch") };
            t = Expression::unliftArray(t.clone())?;
            c_1 = simplifyAsubCref(c.clone(), inSubExp.clone())?;
            exp = Expression::makeCrefExp(c_1.clone(), t.clone())?;
            exp.clone()
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 } if (Expression::isMulOrDiv(op.clone()) || Expression::isAddOrSub(op.clone())) => {
            let mut e: Arc<DAE::Exp>;
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            e1 = Expression::makeASUB(e1.clone(), list![inSubExp.clone()])?;
            e2 = Expression::makeASUB(e2.clone(), list![inSubExp.clone()])?;
            e = if (Expression::isMul(op.clone())) {Expression::expMul(e1.clone(), e2.clone())?} else if (Expression::isDiv(op.clone())) {Expression::makeDiv(e1.clone(), e2.clone())?} else if (Expression::isAdd(op.clone())) {Expression::expAdd(e1.clone(), e2.clone())?} else {Expression::expSub(e1.clone(), e2.clone())?};
            e.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(res)
}

fn simplifyAsubCref(mut cr: Arc<DAE::ComponentRef>, mut sub: Arc<DAE::Exp>) -> Result<Arc<DAE::ComponentRef>> {
    let mut res: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    res = 'mc: {
        let __mc_input = (cr.clone(), sub.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: idn, identType: t2, subscriptLst: s }, _) => {
                    let mut c_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut s_1: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
                    s_1 = Expression::subscriptsAppend(s.clone(), sub.clone())?;
                    c_1 = ComponentReferenceBasics::makeCrefIdent((idn.clone()).clone(), t2.clone(), s_1.clone());
                    Ok(c_1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { ident: idn, identType: t2 @ Deref @ DAE::Type::T_ARRAY { dims, .. }, subscriptLst: s, componentRef: c }, _) => {
                    let mut c_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut s_1: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
                    let true = ((dims.clone().len() as i32) > (s.clone().len() as i32)) else { bail!("pattern mismatch") };
                    s_1 = Expression::subscriptsAppend(s.clone(), sub.clone())?;
                    c_1 = ComponentReferenceBasics::makeCrefQual((idn.clone()).clone(), t2.clone(), s_1.clone(), c.clone());
                    Ok(c_1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { ident: idn, identType: t2, subscriptLst: s, componentRef: c }, _) => {
                    let mut s = (*s).clone();
                    s = Expression::subscriptsReplaceSlice(s.clone(), Arc::new(DAE::Subscript::INDEX { exp: sub.clone() }))?;
                    Ok(ComponentReferenceBasics::makeCrefQual((idn.clone()).clone(), t2.clone(), s.clone(), c.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { ident: idn, identType: t2, subscriptLst: s, componentRef: c }, _) => {
                    let mut c_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    c_1 = simplifyAsubCref(c.clone(), sub.clone())?;
                    c_1 = ComponentReferenceBasics::makeCrefQual((idn.clone()).clone(), t2.clone(), s.clone(), c_1.clone());
                    Ok(c_1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(res)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn simplifyAsub(mut inExp: Arc<DAE::Exp>, mut inSub: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (inExp.clone(), inSub.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, sub) => {
                    let mut exp: Arc<DAE::Exp>;
                    exp = simplifyAsub0(e.clone(), Expression::expInt(sub.clone())?, inSub.clone())?;
                    Ok(exp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { exp: e, operator: DAE::Operator::UMINUS_ARR { .. } }, sub) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut exp: Arc<DAE::Exp>;
                    let mut t2: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut op2: Operator;
                    let mut b: bool = false;
                    e_1 = simplifyAsub(e.clone(), sub.clone())?;
                    t2 = Expression::r#typeof(e_1.clone())?;
                    b = DAEUtil::expTypeArray(t2.clone());
                    op2 = if (b.clone()) {DAE::Operator::UMINUS_ARR { ty: t2.clone() }} else {DAE::Operator::UMINUS { ty: t2.clone() }};
                    exp = Arc::new(DAE::Exp::UNARY { operator: op2.clone(), exp: e_1.clone() });
                    Ok(exp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LUNARY { exp: e, operator: DAE::Operator::NOT { .. } }, sub) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut exp: Arc<DAE::Exp>;
                    let mut t2: Type = Arc::new(DAE::Type::T_NORETCALL);
                    e_1 = simplifyAsub(e.clone(), sub.clone())?;
                    t2 = Expression::r#typeof(e_1.clone())?;
                    exp = Arc::new(DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: t2.clone() }, exp: e_1.clone() });
                    Ok(exp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::SUB_ARR { .. }, exp1: e1 }, sub) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut exp: Arc<DAE::Exp>;
                    let mut t2: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut op2: Operator;
                    let mut b: bool = false;
                    e1_1 = simplifyAsub(e1.clone(), sub.clone())?;
                    e2_1 = simplifyAsub(e2.clone(), sub.clone())?;
                    t2 = Expression::r#typeof(e1_1.clone())?;
                    b = DAEUtil::expTypeArray(t2.clone());
                    op2 = if (b.clone()) {DAE::Operator::SUB_ARR { ty: t2.clone() }} else {DAE::Operator::SUB { ty: t2.clone() }};
                    exp = Arc::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op2.clone(), exp2: e2_1.clone() });
                    Ok(exp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::MUL_ARRAY_SCALAR { .. }, exp1: e1 }, sub) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut exp: Arc<DAE::Exp>;
                    let mut t2: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut op: Operator;
                    let mut b: bool = false;
                    e1_1 = simplifyAsub(e1.clone(), sub.clone())?;
                    t2 = Expression::r#typeof(e1_1.clone())?;
                    b = DAEUtil::expTypeArray(t2.clone());
                    op = if (b.clone()) {DAE::Operator::MUL_ARRAY_SCALAR { ty: t2.clone() }} else {DAE::Operator::MUL { ty: t2.clone() }};
                    exp = Arc::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2.clone() });
                    Ok(exp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::ADD_ARRAY_SCALAR { .. }, exp1: e1 }, sub) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut exp: Arc<DAE::Exp>;
                    let mut t2: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut op: Operator;
                    let mut b: bool = false;
                    e1_1 = simplifyAsub(e1.clone(), sub.clone())?;
                    t2 = Expression::r#typeof(e1_1.clone())?;
                    b = DAEUtil::expTypeArray(t2.clone());
                    op = if (b.clone()) {DAE::Operator::ADD_ARRAY_SCALAR { ty: t2.clone() }} else {DAE::Operator::ADD { ty: t2.clone() }};
                    exp = Arc::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2.clone() });
                    Ok(exp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::SUB_SCALAR_ARRAY { .. }, exp1: e1 }, sub) => {
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut exp: Arc<DAE::Exp>;
                    let mut t2: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut op: Operator;
                    let mut b: bool = false;
                    e2_1 = simplifyAsub(e2.clone(), sub.clone())?;
                    t2 = Expression::r#typeof(e2_1.clone())?;
                    b = DAEUtil::expTypeArray(t2.clone());
                    op = if (b.clone()) {DAE::Operator::SUB_SCALAR_ARRAY { ty: t2.clone() }} else {DAE::Operator::SUB { ty: t2.clone() }};
                    exp = Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2_1.clone() });
                    Ok(exp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::MUL_MATRIX_PRODUCT { .. }, exp1: e1 }, sub) => {
                    let mut e: Arc<DAE::Exp>;
                    e = simplifyMatrixProduct(e1.clone(), e2.clone())?;
                    e = simplifyAsub(e.clone(), sub.clone())?;
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::DIV_SCALAR_ARRAY { .. }, exp1: e1 }, sub) => {
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut exp: Arc<DAE::Exp>;
                    let mut t2: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut op: Operator;
                    let mut b: bool = false;
                    e2_1 = simplifyAsub(e2.clone(), sub.clone())?;
                    t2 = Expression::r#typeof(e2_1.clone())?;
                    b = DAEUtil::expTypeArray(t2.clone());
                    op = if (b.clone()) {DAE::Operator::DIV_SCALAR_ARRAY { ty: t2.clone() }} else {DAE::Operator::DIV { ty: t2.clone() }};
                    exp = Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2_1.clone() });
                    Ok(exp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::DIV_ARRAY_SCALAR { .. }, exp1: e1 }, sub) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut exp: Arc<DAE::Exp>;
                    let mut t2: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut op: Operator;
                    let mut b: bool = false;
                    e1_1 = simplifyAsub(e1.clone(), sub.clone())?;
                    t2 = Expression::r#typeof(e1_1.clone())?;
                    b = DAEUtil::expTypeArray(t2.clone());
                    op = if (b.clone()) {DAE::Operator::DIV_ARRAY_SCALAR { ty: t2.clone() }} else {DAE::Operator::DIV { ty: t2.clone() }};
                    exp = Arc::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2.clone() });
                    Ok(exp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::POW_SCALAR_ARRAY { .. }, exp1: e1 }, sub) => {
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut exp: Arc<DAE::Exp>;
                    let mut t2: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut op: Operator;
                    let mut b: bool = false;
                    e2_1 = simplifyAsub(e2.clone(), sub.clone())?;
                    t2 = Expression::r#typeof(e2_1.clone())?;
                    b = DAEUtil::expTypeArray(t2.clone());
                    op = if (b.clone()) {DAE::Operator::POW_SCALAR_ARRAY { ty: t2.clone() }} else {DAE::Operator::POW { ty: t2.clone() }};
                    exp = Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2_1.clone() });
                    Ok(exp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::POW_ARRAY_SCALAR { .. }, exp1: e1 }, sub) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut exp: Arc<DAE::Exp>;
                    let mut t2: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut op: Operator;
                    let mut b: bool = false;
                    e1_1 = simplifyAsub(e1.clone(), sub.clone())?;
                    t2 = Expression::r#typeof(e1_1.clone())?;
                    b = DAEUtil::expTypeArray(t2.clone());
                    op = if (b.clone()) {DAE::Operator::POW_ARRAY_SCALAR { ty: t2.clone() }} else {DAE::Operator::POW { ty: t2.clone() }};
                    exp = Arc::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2.clone() });
                    Ok(exp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::ADD_ARR { .. }, exp1: e1 }, sub) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut exp: Arc<DAE::Exp>;
                    let mut t2: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut op2: Operator;
                    let mut b: bool = false;
                    e1_1 = simplifyAsub(e1.clone(), sub.clone())?;
                    e2_1 = simplifyAsub(e2.clone(), sub.clone())?;
                    t2 = Expression::r#typeof(e1_1.clone())?;
                    b = DAEUtil::expTypeArray(t2.clone());
                    op2 = if (b.clone()) {DAE::Operator::ADD_ARR { ty: t2.clone() }} else {DAE::Operator::ADD { ty: t2.clone() }};
                    exp = Arc::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op2.clone(), exp2: e2_1.clone() });
                    Ok(exp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::MUL_ARR { .. }, exp1: e1 }, sub) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut exp: Arc<DAE::Exp>;
                    let mut t2: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut op2: Operator;
                    let mut b: bool = false;
                    e1_1 = simplifyAsub(e1.clone(), sub.clone())?;
                    e2_1 = simplifyAsub(e2.clone(), sub.clone())?;
                    t2 = Expression::r#typeof(e1_1.clone())?;
                    b = DAEUtil::expTypeArray(t2.clone());
                    op2 = if (b.clone()) {DAE::Operator::MUL_ARR { ty: t2.clone() }} else {DAE::Operator::MUL { ty: t2.clone() }};
                    exp = Arc::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op2.clone(), exp2: e2_1.clone() });
                    Ok(exp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::DIV_ARR { .. }, exp1: e1 }, sub) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut exp: Arc<DAE::Exp>;
                    let mut t2: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut op2: Operator;
                    let mut b: bool = false;
                    e1_1 = simplifyAsub(e1.clone(), sub.clone())?;
                    e2_1 = simplifyAsub(e2.clone(), sub.clone())?;
                    t2 = Expression::r#typeof(e1_1.clone())?;
                    b = DAEUtil::expTypeArray(t2.clone());
                    op2 = if (b.clone()) {DAE::Operator::DIV_ARR { ty: t2.clone() }} else {DAE::Operator::DIV { ty: t2.clone() }};
                    exp = Arc::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op2.clone(), exp2: e2_1.clone() });
                    Ok(exp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::POW_ARR2 { .. }, exp1: e1 }, sub) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut exp: Arc<DAE::Exp>;
                    let mut t2: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut op2: Operator;
                    let mut b: bool = false;
                    e1_1 = simplifyAsub(e1.clone(), sub.clone())?;
                    e2_1 = simplifyAsub(e2.clone(), sub.clone())?;
                    t2 = Expression::r#typeof(e1_1.clone())?;
                    b = DAEUtil::expTypeArray(t2.clone());
                    op2 = if (b.clone()) {DAE::Operator::POW_ARR2 { ty: t2.clone() }} else {DAE::Operator::POW { ty: t2.clone() }};
                    exp = Arc::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op2.clone(), exp2: e2_1.clone() });
                    Ok(exp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LBINARY { exp2: e2, operator: op, exp1: e1 }, sub) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    let mut exp: Arc<DAE::Exp>;
                    let mut t2: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut op = (*op).clone();
                    e1_1 = simplifyAsub(e1.clone(), sub.clone())?;
                    e2_1 = simplifyAsub(e2.clone(), sub.clone())?;
                    t2 = Expression::r#typeof(e1_1.clone())?;
                    op = Expression::setOpType(op.clone(), t2.clone())?;
                    exp = Arc::new(DAE::Exp::LBINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone() });
                    Ok(exp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { array: exps, .. }, sub) => {
                    let mut exp: Arc<DAE::Exp>;
                    let mut indx: i32 = 0;
                    indx = Expression::expInt(sub.clone())?;
                    exp = (exps.clone()).get(indx.clone())?;
                    Ok(exp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::MATRIX { matrix: lstexps, ty: t, .. }, sub) => {
                    let mut t_1: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let mut indx: i32 = 0;
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    indx = Expression::expInt(sub.clone())?;
                    expl = (lstexps.clone()).get(indx.clone())?;
                    t_1 = Expression::unliftArray(t.clone())?;
                    Ok(Arc::new(DAE::Exp::ARRAY { ty: t_1.clone(), scalar: true, array: expl.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { expCond: cond, expThen: e1, expElse: e2 }, sub) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    e1_1 = simplifyAsub(e1.clone(), sub.clone())?;
                    e2_1 = simplifyAsub(e2.clone(), sub.clone())?;
                    Ok(Arc::new(DAE::Exp::IFEXP { expCond: cond.clone(), expThen: e1_1.clone(), expElse: e2_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::REDUCTION { reductionInfo: Deref @ DAE::ReductionInfo { iterType: Absyn::ReductionIterType::THREAD, path: Deref @ Absyn::Path::IDENT { name: Deref @ "array" }, .. }, expr: exp, iterators: iters }, sub) => {
                    let mut exp = (*exp).clone();
                    exp = List::fold1(iters.clone(), (std::sync::Arc::new(simplifyAsubArrayReduction) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ReductionIterator>, Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), sub.clone(), exp.clone());
                    Ok(exp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::REDUCTION { reductionInfo: Deref @ DAE::ReductionInfo { iterType: Absyn::ReductionIterType::COMBINE, path: Deref @ Absyn::Path::IDENT { name: Deref @ "array" }, .. }, expr: exp, iterators: Deref @ metamodelica::List::Cons { head: iter, tail: Deref @ metamodelica::List::Nil } }, sub) => {
                    let mut exp = (*exp).clone();
                    exp = simplifyAsubArrayReduction(iter.clone(), sub.clone(), exp.clone())?;
                    Ok(exp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

fn simplifyAsubArrayReduction(mut iter: Arc<DAE::ReductionIterator>, mut sub: Arc<DAE::Exp>, mut acc: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut res: Arc<DAE::Exp>;
    res = (::match_deref::match_deref! { match &((iter.clone(), sub.clone(), acc.clone())) {
        (Deref @ DAE::ReductionIterator { guardExp: None, exp, id, .. }, _, _) => {
            let mut exp = (*exp).clone();
            exp = Expression::makeASUB(exp.clone(), list![sub.clone()])?;
            exp = replaceIteratorWithExp(exp.clone(), acc.clone(), (id.clone()).clone())?;
            exp.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(res)
}

fn simplifyAsubOperator(mut inExp1: Arc<DAE::Exp>, mut inOperator2: Operator, mut inOperator3: Operator) -> Operator {
    let mut outOperator: Operator;
    outOperator = (::match_deref::match_deref! { match &(inExp1.clone()) {
        Deref @ DAE::Exp::ARRAY { .. } => inOperator3.clone(),
        Deref @ DAE::Exp::MATRIX { .. } => inOperator3.clone(),
        Deref @ DAE::Exp::RANGE { .. } => inOperator3.clone(),
        _ => inOperator2.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outOperator
}

fn simplifyAsubSlicing(mut inExp: Arc<DAE::Exp>, mut inSubscripts: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<DAE::Exp>> {
    let mut outAsubArray: Arc<DAE::Exp>;
    let mut indices: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
    let mut asubs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut es: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut sz: i32 = 0;
    let mut elem: Arc<DAE::Exp>;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut didSplit: bool = false;
    let mut b: bool = false;
    indices = {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
        for mut e in (inSubscripts.clone()).into_iter().cloned() {
            let __x = (match () {
        () => {
            (es, b) = Expression::splitArray((simplify1(e.clone())?).0)?;
            didSplit = didSplit.clone() || b.clone();
            es.clone()
        },
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    let true = (didSplit.clone()) else { bail!("pattern mismatch") };
    for mut is in &*indices.clone() {
        let mut is = is.clone();
        for mut i in &*is.clone() {
            let mut i = i.clone();
            let _ = (::match_deref::match_deref! { match &(Expression::r#typeof(i.clone())?) {
        Deref @ DAE::Type::T_INTEGER { .. } => (),
        Deref @ DAE::Type::T_BOOL { .. } => (),
        Deref @ DAE::Type::T_ENUMERATION { .. } => (),
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
    }
    asubs = List::combinationMap(indices.clone(), Arc::new({ let __pe_b1 = inExp.clone(); move |__pe_a0| simplifyAsubSlicing2(__pe_a0, __pe_b1.clone()) }));
    outAsubArray = Expression::makeScalarArray(asubs.clone(), Types::unliftArray(Expression::r#typeof(inExp.clone())?)?);
    Ok(outAsubArray)
}

fn simplifyAsubSlicing2(mut inSubscripts: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outAsub: Arc<DAE::Exp>;
    outAsub = Expression::makeASUB(inExp.clone(), inSubscripts.clone())?;
    Ok(outAsub)
}

fn simplifyBinaryConst(mut inOperator1: Operator, mut inExp2: Arc<DAE::Exp>, mut inExp3: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &((inOperator1.clone(), inExp2.clone(), inExp3.clone())) {
        (DAE::Operator::ADD { .. }, Deref @ DAE::Exp::ICONST { integer: ie1 }, Deref @ DAE::Exp::ICONST { integer: ie2 }) => {
            let mut val: Arc<DAE::Exp>;
            val = safeIntOp(ie1.clone(), ie2.clone(), crate::ExpressionSimplifyTypes::IntOp::ADDOP)?;
            val.clone()
        },
        (DAE::Operator::ADD { .. }, Deref @ DAE::Exp::RCONST { real: re1 }, Deref @ DAE::Exp::RCONST { real: re2 }) => {
            let mut re3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            re3 = re1.clone() + re2.clone();
            Arc::new(DAE::Exp::RCONST { real: re3.clone() })
        },
        (DAE::Operator::ADD { .. }, Deref @ DAE::Exp::RCONST { real: re1 }, Deref @ DAE::Exp::ICONST { integer: ie2 }) => {
            let mut e2_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut re3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            e2_1 = intReal(ie2.clone());
            re3 = re1.clone() + e2_1.clone();
            Arc::new(DAE::Exp::RCONST { real: re3.clone() })
        },
        (DAE::Operator::ADD { .. }, Deref @ DAE::Exp::ICONST { integer: ie1 }, Deref @ DAE::Exp::RCONST { real: re2 }) => {
            let mut e1_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut re3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            e1_1 = intReal(ie1.clone());
            re3 = e1_1.clone() + re2.clone();
            Arc::new(DAE::Exp::RCONST { real: re3.clone() })
        },
        (DAE::Operator::ADD { .. }, Deref @ DAE::Exp::SCONST { string: s1 }, Deref @ DAE::Exp::SCONST { string: s2 }) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*s2.clone()); ArcStr::from(__mm_s) }).clone();
            Arc::new(DAE::Exp::SCONST { string: (r#str.clone()).clone() })
        },
        (DAE::Operator::SUB { .. }, Deref @ DAE::Exp::ICONST { integer: ie1 }, Deref @ DAE::Exp::ICONST { integer: ie2 }) => {
            let mut val: Arc<DAE::Exp>;
            val = safeIntOp(ie1.clone(), ie2.clone(), crate::ExpressionSimplifyTypes::IntOp::SUBOP)?;
            val.clone()
        },
        (DAE::Operator::SUB { .. }, Deref @ DAE::Exp::RCONST { real: re1 }, Deref @ DAE::Exp::RCONST { real: re2 }) => {
            let mut re3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            re3 = re1.clone() - re2.clone();
            Arc::new(DAE::Exp::RCONST { real: re3.clone() })
        },
        (DAE::Operator::SUB { .. }, Deref @ DAE::Exp::RCONST { real: re1 }, Deref @ DAE::Exp::ICONST { integer: ie2 }) => {
            let mut e2_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut re3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            e2_1 = intReal(ie2.clone());
            re3 = re1.clone() - e2_1.clone();
            Arc::new(DAE::Exp::RCONST { real: re3.clone() })
        },
        (DAE::Operator::SUB { .. }, Deref @ DAE::Exp::ICONST { integer: ie1 }, Deref @ DAE::Exp::RCONST { real: re2 }) => {
            let mut e1_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut re3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            e1_1 = intReal(ie1.clone());
            re3 = e1_1.clone() - re2.clone();
            Arc::new(DAE::Exp::RCONST { real: re3.clone() })
        },
        (DAE::Operator::MUL { .. }, Deref @ DAE::Exp::ICONST { integer: ie1 }, Deref @ DAE::Exp::ICONST { integer: ie2 }) => {
            let mut val: Arc<DAE::Exp>;
            val = safeIntOp(ie1.clone(), ie2.clone(), crate::ExpressionSimplifyTypes::IntOp::MULOP)?;
            val.clone()
        },
        (DAE::Operator::MUL { .. }, Deref @ DAE::Exp::RCONST { real: re1 }, Deref @ DAE::Exp::RCONST { real: re2 }) => {
            let mut re3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            re3 = re1.clone() * re2.clone();
            Arc::new(DAE::Exp::RCONST { real: re3.clone() })
        },
        (DAE::Operator::MUL { .. }, Deref @ DAE::Exp::RCONST { real: re1 }, Deref @ DAE::Exp::ICONST { integer: ie2 }) => {
            let mut e2_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut re3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            e2_1 = intReal(ie2.clone());
            re3 = re1.clone() * e2_1.clone();
            Arc::new(DAE::Exp::RCONST { real: re3.clone() })
        },
        (DAE::Operator::MUL { .. }, Deref @ DAE::Exp::ICONST { integer: ie1 }, Deref @ DAE::Exp::RCONST { real: re2 }) => {
            let mut e1_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut re3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            e1_1 = intReal(ie1.clone());
            re3 = e1_1.clone() * re2.clone();
            Arc::new(DAE::Exp::RCONST { real: re3.clone() })
        },
        (DAE::Operator::DIV { .. }, Deref @ DAE::Exp::ICONST { integer: ie1 }, Deref @ DAE::Exp::ICONST { integer: ie2 }) => {
            let mut val: Arc<DAE::Exp>;
            val = safeIntOp(ie1.clone(), ie2.clone(), crate::ExpressionSimplifyTypes::IntOp::DIVOP)?;
            val.clone()
        },
        (DAE::Operator::DIV { .. }, Deref @ DAE::Exp::RCONST { real: re1 }, Deref @ DAE::Exp::RCONST { real: re2 }) => {
            let mut re3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            re3 = re1.clone() / re2.clone();
            Arc::new(DAE::Exp::RCONST { real: re3.clone() })
        },
        (DAE::Operator::DIV { .. }, Deref @ DAE::Exp::RCONST { real: re1 }, Deref @ DAE::Exp::ICONST { integer: ie2 }) => {
            let mut e2_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut re3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            e2_1 = intReal(ie2.clone());
            re3 = re1.clone() / e2_1.clone();
            Arc::new(DAE::Exp::RCONST { real: re3.clone() })
        },
        (DAE::Operator::DIV { .. }, Deref @ DAE::Exp::ICONST { integer: ie1 }, Deref @ DAE::Exp::RCONST { real: re2 }) => {
            let mut e1_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut re3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            e1_1 = intReal(ie1.clone());
            re3 = e1_1.clone() / re2.clone();
            Arc::new(DAE::Exp::RCONST { real: re3.clone() })
        },
        (DAE::Operator::POW { .. }, Deref @ DAE::Exp::RCONST { real: re1 }, Deref @ DAE::Exp::RCONST { real: re2 }) => {
            let mut re3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            re3 = (re1.clone()).powf(re2.clone());
            Arc::new(DAE::Exp::RCONST { real: re3.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

fn simplifyRelationConst(mut op: Operator, mut e1: Arc<DAE::Exp>, mut e2: Arc<DAE::Exp>) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &((op.clone(), e1.clone(), e2.clone())) {
        (DAE::Operator::LESS { .. }, Deref @ DAE::Exp::BCONST { bool: false }, Deref @ DAE::Exp::BCONST { bool: true }) => {
            true
        },
        (DAE::Operator::LESS { .. }, Deref @ DAE::Exp::BCONST { bool: _ }, Deref @ DAE::Exp::BCONST { bool: _ }) => {
            false
        },
        (DAE::Operator::LESS { .. }, _, _) => {
            let mut v1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut v2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            v1 = Expression::toReal(e1.clone())?;
            v2 = Expression::toReal(e2.clone())?;
            b = v1.clone() < v2.clone();
            b.clone()
        },
        (DAE::Operator::LESSEQ { .. }, Deref @ DAE::Exp::BCONST { bool: true }, Deref @ DAE::Exp::BCONST { bool: false }) => {
            false
        },
        (DAE::Operator::LESSEQ { .. }, Deref @ DAE::Exp::BCONST { bool: _ }, Deref @ DAE::Exp::BCONST { bool: _ }) => {
            true
        },
        (DAE::Operator::LESSEQ { .. }, _, _) => {
            let mut v1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut v2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            v1 = Expression::toReal(e1.clone())?;
            v2 = Expression::toReal(e2.clone())?;
            b = v1.clone() <= v2.clone();
            b.clone()
        },
        (DAE::Operator::EQUAL { .. }, Deref @ DAE::Exp::BCONST { bool: b1 }, Deref @ DAE::Exp::BCONST { bool: b2 }) => {
            boolEq(b1.clone(), b2.clone())
        },
        (DAE::Operator::EQUAL { .. }, Deref @ DAE::Exp::SCONST { string: s1 }, Deref @ DAE::Exp::SCONST { string: s2 }) => {
            stringEqual((s1.clone()).clone(), (s2.clone()).clone())
        },
        (DAE::Operator::EQUAL { .. }, _, _) => {
            let mut v1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut v2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            v1 = Expression::toReal(e1.clone())?;
            v2 = Expression::toReal(e2.clone())?;
            realEq(v1.clone(), v2.clone())
        },
        (DAE::Operator::GREATER { .. }, _, _) => {
            !(simplifyRelationConst(DAE::Operator::LESSEQ { ty: DAE::T_REAL_DEFAULT().clone() }, e1.clone(), e2.clone())?)
        },
        (DAE::Operator::GREATEREQ { .. }, _, _) => {
            !(simplifyRelationConst(DAE::Operator::LESS { ty: DAE::T_REAL_DEFAULT().clone() }, e1.clone(), e2.clone())?)
        },
        (DAE::Operator::GREATER { .. }, Deref @ DAE::Exp::BCONST { bool: false }, Deref @ DAE::Exp::BCONST { bool: true }) => {
            !(simplifyRelationConst(DAE::Operator::LESSEQ { ty: DAE::T_REAL_DEFAULT().clone() }, e1.clone(), e2.clone())?)
        },
        (DAE::Operator::GREATEREQ { .. }, Deref @ DAE::Exp::BCONST { bool: false }, Deref @ DAE::Exp::BCONST { bool: true }) => {
            !(simplifyRelationConst(DAE::Operator::LESS { ty: DAE::T_REAL_DEFAULT().clone() }, e1.clone(), e2.clone())?)
        },
        (DAE::Operator::NEQUAL { .. }, _, _) => {
            !(simplifyRelationConst(DAE::Operator::EQUAL { ty: DAE::T_REAL_DEFAULT().clone() }, e1.clone(), e2.clone())?)
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(b)
}

pub fn safeIntOp(mut val1: i32, mut val2: i32, mut op: ExpressionSimplifyTypes::IntOp) -> Result<Arc<DAE::Exp>> {
    let mut outv: Arc<DAE::Exp>;
    outv = (match (val1.clone(), val2.clone(), op.clone()) {
        (_, _, ExpressionSimplifyTypes::IntOp::MULOP) => {
            let mut rv1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rv2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rv3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            rv1 = intReal(val1.clone());
            rv2 = intReal(val2.clone());
            rv3 = rv1.clone() * rv2.clone();
            outv = Expression::realToIntIfPossible(rv3.clone());
            outv.clone()
        },
        (_, _, ExpressionSimplifyTypes::IntOp::DIVOP) => {
            let mut ires: i32 = 0;
            ires = intDiv(val1.clone(), val2.clone());
            Arc::new(DAE::Exp::ICONST { integer: ires.clone() })
        },
        (_, _, ExpressionSimplifyTypes::IntOp::SUBOP) => {
            let mut rv1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rv2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rv3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            rv1 = intReal(val1.clone());
            rv2 = intReal(val2.clone());
            rv3 = rv1.clone() - rv2.clone();
            outv = Expression::realToIntIfPossible(rv3.clone());
            outv.clone()
        },
        (_, _, ExpressionSimplifyTypes::IntOp::ADDOP) => {
            let mut rv1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rv2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rv3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            rv1 = intReal(val1.clone());
            rv2 = intReal(val2.clone());
            rv3 = rv1.clone() + rv2.clone();
            outv = Expression::realToIntIfPossible(rv3.clone());
            outv.clone()
        },
        (_, _, ExpressionSimplifyTypes::IntOp::POWOP) => {
            let mut rv1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rv2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rv3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            rv1 = intReal(val1.clone());
            rv2 = intReal(val2.clone());
            rv3 = realPow(rv1.clone(), rv2.clone());
            outv = Expression::realToIntIfPossible(rv3.clone());
            outv.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outv)
}

fn simplifyBinaryCommutativeWork(mut op: Operator, mut lhs: Arc<DAE::Exp>, mut rhs: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut exp: Arc<DAE::Exp>;
    exp = 'mc: {
        let __mc_input = (op.clone(), lhs.clone(), rhs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL { ty: _ }, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sin" }, .. }, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "cos" }, .. }) => {
                    if !((ExpressionBasics::expEqual(e1.clone(), e2.clone())?)) { bail!("guard") }
                    let mut e: Arc<DAE::Exp>;
                    let mut op1: Operator;
                    op1 = DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() };
                    e = Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }), operator: op1.clone(), exp2: e1.clone() });
                    e = Expression::makePureBuiltinCall((literal!("sin")).clone(), list![e.clone()], DAE::T_REAL_DEFAULT().clone());
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.5_f64) }), operator: op1.clone(), exp2: e.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::ADD { ty: _ }, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sin" }, .. }, operator: DAE::Operator::POW { ty: Deref @ DAE::Type::T_REAL { .. } }, exp2: Deref @ DAE::Exp::RCONST { real: __rlit_4 } }, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "cos" }, .. }, operator: DAE::Operator::POW { ty: Deref @ DAE::Type::T_REAL { .. } }, exp2: Deref @ DAE::Exp::RCONST { real: __rlit_5 } }) => {
                    if !(__rlit_4.eq(&metamodelica::OrderedFloat((2.0) as f64)) && __rlit_5.eq(&metamodelica::OrderedFloat((2.0) as f64)) && (ExpressionBasics::expEqual(e1.clone(), e2.clone())?)) { bail!("guard") }
                    Ok(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL { ty: tp }, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "tan" }, .. }, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "cos" }, .. }) => {
                    if !((ExpressionBasics::expEqual(e1.clone(), e2.clone())?)) { bail!("guard") }
                    Ok(Expression::makePureBuiltinCall((literal!("sin")).clone(), list![e1.clone()], tp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::ADD { ty: _ }, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "cosh" }, .. }, operator: DAE::Operator::POW { ty: Deref @ DAE::Type::T_REAL { .. } }, exp2: Deref @ DAE::Exp::RCONST { real: __rlit_6 } }, Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sinh" }, .. }, operator: DAE::Operator::POW { ty: Deref @ DAE::Type::T_REAL { .. } }, exp2: Deref @ DAE::Exp::RCONST { real: __rlit_7 } }, operator: DAE::Operator::UMINUS { .. } }) => {
                    if !(__rlit_6.eq(&metamodelica::OrderedFloat((2.0) as f64)) && __rlit_7.eq(&metamodelica::OrderedFloat((2.0) as f64)) && (ExpressionBasics::expEqual(e1.clone(), e2.clone())?)) { bail!("guard") }
                    Ok(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL { ty: tp }, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "tanh" }, .. }, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "cosh" }, .. }) => {
                    if !((ExpressionBasics::expEqual(e1.clone(), e2.clone())?)) { bail!("guard") }
                    Ok(Expression::makePureBuiltinCall((literal!("sinh")).clone(), list![e1.clone()], tp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::ADD { ty: tp }, e1, Deref @ DAE::Exp::UNARY { exp: e2, operator: DAE::Operator::UMINUS { .. } }) => {
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::SUB { ty: tp.clone() }, exp2: e2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::ADD { ty: tp }, e1, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { exp: e2, operator: DAE::Operator::UMINUS { .. } }, operator: op2, exp2: e3 }) => {
                    if !((Expression::isMulOrDiv(op2.clone()))) { bail!("guard") }
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::SUB { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op2.clone(), exp2: e3.clone() }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::ADD { .. }, e1, e2) => {
                    if !((Expression::isZero(e1.clone()))) { bail!("guard") }
                    Ok(e2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL { ty: tp }, e1, Deref @ DAE::Exp::BINARY { exp2: e3, operator: DAE::Operator::DIV { ty: tp2 }, exp1: e2 }) => {
                    let mut e: Arc<DAE::Exp>;
                    let __pa0 = ::match_deref::match_deref! { match &(simplify1(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: e2.clone() }))?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e = __pa0.clone();
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e.clone(), operator: DAE::Operator::DIV { ty: tp2.clone() }, exp2: e3.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL { .. }, _, e2) => {
                    if !((Expression::isZero(e2.clone()))) { bail!("guard") }
                    Ok(e2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL { .. }, e1, e2) => {
                    if !((Expression::isConstOne(e2.clone()))) { bail!("guard") }
                    Ok(e1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL { ty }, e1, e2) => {
                    if !((Expression::isConstMinusOne(e2.clone()))) { bail!("guard") }
                    Ok(Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: ty.clone() }, exp: e1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL { .. }, Deref @ DAE::Exp::BINARY { exp1: e2, operator: op1 @ DAE::Operator::MUL { ty }, exp2: e3 }, e1) => {
                    if !((Types::isScalarReal(ty.clone()) && ExpressionBasics::expEqual(e2.clone(), e1.clone())?)) { bail!("guard") }
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e3.clone(), operator: op1.clone(), exp2: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::POW { ty: ty.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }) }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL { .. }, e1, Deref @ DAE::Exp::BINARY { exp1: e2, operator: op1 @ DAE::Operator::MUL { ty }, exp2: e3 }) => {
                    if !((Types::isScalarReal(ty.clone()) && ExpressionBasics::expEqual(e1.clone(), e3.clone())?)) { bail!("guard") }
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op1.clone(), exp2: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::POW { ty: ty.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }) }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL { .. }, Deref @ DAE::Exp::RCONST { real: r1 }, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::RCONST { real: r2 }, operator: DAE::Operator::MUL { ty: Deref @ DAE::Type::T_REAL { .. } }, exp2: e2 }) => {
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: r1.clone() * r2.clone() }), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL { .. }, Deref @ DAE::Exp::RCONST { real: r1 }, Deref @ DAE::Exp::BINARY { exp1: e2, operator: DAE::Operator::MUL { ty: Deref @ DAE::Type::T_REAL { .. } }, exp2: Deref @ DAE::Exp::RCONST { real: r2 } }) => {
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: r1.clone() * r2.clone() }), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::ADD { .. }, Deref @ DAE::Exp::RCONST { real: r1 }, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::RCONST { real: r2 }, operator: DAE::Operator::SUB { ty }, exp2: e1 @ Deref @ DAE::Exp::CREF { componentRef: _, .. } }) => {
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: (r1.clone()) + (r2.clone()) }), operator: DAE::Operator::SUB { ty: ty.clone() }, exp2: e1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::DIV { ty }, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "abs" }, .. }, e2) => {
                    if !((ExpressionBasics::expEqual(e1.clone(), e2.clone())?)) { bail!("guard") }
                    Ok(Expression::makePureBuiltinCall((literal!("sign")).clone(), list![e1.clone()], ty.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (op1 @ DAE::Operator::ADD { ty }, e1, Deref @ DAE::Exp::BINARY { exp1: e2, operator: op2 @ DAE::Operator::MUL { .. }, exp2: e3 }) => {
                    if !((!(Expression::isConstValue(e1.clone())?))) { bail!("guard") }
                    let mut exp: Arc<DAE::Exp>;
                    if ExpressionBasics::expEqual(e1.clone(), e3.clone())? {
                        exp = Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op2.clone(), exp2: Arc::new(DAE::Exp::BINARY { exp1: Expression::makeConstOne(ty.clone()), operator: op1.clone(), exp2: e2.clone() }) });
                    } else {
                        if ExpressionBasics::expEqual(e1.clone(), e2.clone())? {
                            exp = Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op2.clone(), exp2: Arc::new(DAE::Exp::BINARY { exp1: Expression::makeConstOne(ty.clone()), operator: op1.clone(), exp2: e3.clone() }) });
                        } else {
                            bail!("fail");
                        }
                    }
                    Ok(exp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL { .. }, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, .. }, e2) => {
                    if !((ExpressionBasics::expEqual(e1.clone(), e2.clone())?)) { bail!("guard") }
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::POW { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.5_f64) }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL { .. }, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, .. }, Deref @ DAE::Exp::BINARY { exp1: e2, operator: DAE::Operator::POW { .. }, exp2: e }) => {
                    if !((ExpressionBasics::expEqual(e1.clone(), e2.clone())?)) { bail!("guard") }
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::POW { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e.clone(), operator: DAE::Operator::ADD { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.5_f64) }) }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Operator::MUL { .. }, e1, Deref @ DAE::Exp::BINARY { exp1: e3, operator: op1 @ DAE::Operator::POW { ty: tp }, exp2: e4 }) => {
                    if !((ExpressionBasics::expEqual(e1.clone(), e3.clone())?)) { bail!("guard") }
                    let mut e: Arc<DAE::Exp>;
                    e = Expression::makeConstOne(tp.clone());
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op1.clone(), exp2: Arc::new(DAE::Exp::BINARY { exp1: e.clone(), operator: DAE::Operator::ADD { ty: tp.clone() }, exp2: e4.clone() }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(exp)
}

fn simplifyBinary(mut origExp: Arc<DAE::Exp>, mut inOperator2: Operator, mut lhs: Arc<DAE::Exp>, mut rhs: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let mut lhsIsConstValue: bool = Expression::isConstValue(lhs.clone())?;
    let mut rhsIsConstValue: bool = Expression::isConstValue(rhs.clone())?;
    outExp = 'mc: {
        let __mc_input = (origExp.clone(), inOperator2.clone(), lhs.clone(), rhs.clone(), lhsIsConstValue.clone(), rhsIsConstValue.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, op, e1, e2, _, _) => {
                    if !((simplifyBinaryArrayOp(op.clone()))) { bail!("guard") }
                    Ok(simplifyBinaryArray(e1.clone(), op.clone(), e2.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, op, e1, e2, _, _) => {
                    Ok(simplifyBinaryCommutativeWork(op.clone(), e1.clone(), e2.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, op, e1, e2, _, _) => {
                    Ok(simplifyBinaryCommutativeWork(op.clone(), e2.clone(), e1.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, oper, e1, e2, true, true) => {
                    let mut e3: Arc<DAE::Exp>;
                    e3 = simplifyBinaryConst(oper.clone(), e1.clone(), e2.clone())?;
                    Ok(e3.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, oper, Deref @ DAE::Exp::BINARY { exp1: e1, operator: op1, exp2: e2 }, Deref @ DAE::Exp::BINARY { exp1: e3, operator: op2, exp2: e4 }, _, _) => {
                    Ok(simplifyTwoBinaryExpressions(e1.clone(), op1.clone(), e2.clone(), oper.clone(), e3.clone(), op2.clone(), e4.clone(), ExpressionBasics::expEqual(e1.clone(), e3.clone())?, ExpressionBasics::expEqual(e1.clone(), e4.clone())?, ExpressionBasics::expEqual(e2.clone(), e3.clone())?, ExpressionBasics::expEqual(e2.clone(), e4.clone())?, Expression::isConstValue(e1.clone())?, Expression::isConstValue(e2.clone())?, Expression::isConstValue(e3.clone())?, Expression::operatorEqual(op1.clone(), op2.clone())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, oper, e1, e2, _, _) => {
                    let true = (Expression::isConstZeroLength(e1.clone()) || Expression::isConstZeroLength(e2.clone())) else { bail!("pattern mismatch") };
                    checkZeroLengthArrayOp(oper.clone())?;
                    Ok(e1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::MUL { .. }, e1, Deref @ DAE::Exp::BINARY { exp2: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e3 }, operator: op1 @ DAE::Operator::POW { ty: ty2 }, exp1: e2 }, _, _) => {
                    let mut res: Arc<DAE::Exp>;
                    res = Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::DIV { ty: ty2.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op1.clone(), exp2: e3.clone() }) });
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::DIV { .. }, e1, Deref @ DAE::Exp::BINARY { exp2: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e3 }, operator: op1 @ DAE::Operator::POW { ty: ty2 }, exp1: e2 }, _, _) => {
                    let mut res: Arc<DAE::Exp>;
                    res = Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL { ty: ty2.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op1.clone(), exp2: e3.clone() }) });
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::MUL { .. }, e1, Deref @ DAE::Exp::BINARY { exp2: Deref @ DAE::Exp::RCONST { real: r }, operator: op1 @ DAE::Operator::POW { ty: ty2 }, exp1: e2 }, _, _) => {
                    let mut res: Arc<DAE::Exp>;
                    let mut r = (*r).clone();
                    let true = (realLt(r.clone(), metamodelica::OrderedFloat(0.0_f64))) else { bail!("pattern mismatch") };
                    r = -(r.clone());
                    res = Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::DIV { ty: ty2.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op1.clone(), exp2: Arc::new(DAE::Exp::RCONST { real: r.clone() }) }) });
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::DIV { .. }, e1, Deref @ DAE::Exp::BINARY { exp2: Deref @ DAE::Exp::RCONST { real: r }, operator: op1 @ DAE::Operator::POW { ty: ty2 }, exp1: e2 }, _, _) => {
                    let mut res: Arc<DAE::Exp>;
                    let mut r = (*r).clone();
                    let true = (realLt(r.clone(), metamodelica::OrderedFloat(0.0_f64))) else { bail!("pattern mismatch") };
                    r = -(r.clone());
                    res = Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL { ty: ty2.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op1.clone(), exp2: Arc::new(DAE::Exp::RCONST { real: r.clone() }) }) });
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::DIV { ty: _ }, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { ty: _ }, exp2: e2 }, operator: op1, exp2: e3 }, e4, _, _) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut res: Arc<DAE::Exp>;
                    let true = (Expression::isAddOrSub(op1.clone())) else { bail!("pattern mismatch") };
                    let true = (ExpressionBasics::expEqual(e2.clone(), e4.clone())?) else { bail!("pattern mismatch") };
                    e = Expression::makeDiv(e3.clone(), e4.clone())?;
                    res = Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op1.clone(), exp2: e.clone() });
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::DIV { ty: _ }, Deref @ DAE::Exp::BINARY { exp1: e3, operator: op1, exp2: Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { ty: _ }, exp2: e2 } }, e4, _, _) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut res: Arc<DAE::Exp>;
                    let true = (Expression::isAddOrSub(op1.clone())) else { bail!("pattern mismatch") };
                    let true = (ExpressionBasics::expEqual(e2.clone(), e4.clone())?) else { bail!("pattern mismatch") };
                    e = Expression::makeDiv(e3.clone(), e4.clone())?;
                    res = Arc::new(DAE::Exp::BINARY { exp1: e.clone(), operator: op1.clone(), exp2: e1.clone() });
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::DIV { ty: _ }, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::BINARY { exp1: e1, operator: op2 @ DAE::Operator::MUL { .. }, exp2: Deref @ DAE::Exp::BINARY { exp1: e2, operator: DAE::Operator::MUL { ty: _ }, exp2: e3 } }, operator: op1, exp2: e4 }, e5, _, _) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e: Arc<DAE::Exp>;
                    let mut res: Arc<DAE::Exp>;
                    let true = (Expression::isAddOrSub(op1.clone())) else { bail!("pattern mismatch") };
                    let true = (ExpressionBasics::expEqual(e3.clone(), e5.clone())?) else { bail!("pattern mismatch") };
                    e = Expression::makeDiv(e4.clone(), e3.clone())?;
                    e1_1 = Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op2.clone(), exp2: e2.clone() });
                    res = Arc::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op1.clone(), exp2: e.clone() });
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, op2, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "abs" }, .. }, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "abs" }, .. }, _, _) => {
                    let mut res: Arc<DAE::Exp>;
                    let mut ty: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let true = (Expression::isMulOrDiv(op2.clone())) else { bail!("pattern mismatch") };
                    ty = Expression::r#typeof(e1.clone())?;
                    res = Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op2.clone(), exp2: e2.clone() });
                    Ok(Expression::makePureBuiltinCall((literal!("abs")).clone(), list![res.clone()], ty.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::DIV { ty }, e1, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }, .. }, _, _) => {
                    let mut e3: Arc<DAE::Exp>;
                    let mut e: Arc<DAE::Exp>;
                    let mut res: Arc<DAE::Exp>;
                    e = Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: ty.clone() }, exp: e2.clone() });
                    (e, _) = simplify1(e.clone())?;
                    e3 = Expression::makePureBuiltinCall((literal!("exp")).clone(), list![e.clone()], ty.clone());
                    res = Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: e3.clone() });
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::MUL { ty }, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }, .. }, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }, .. }, _, _) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut res: Arc<DAE::Exp>;
                    let false = (Expression::isConstValue(e1.clone())? || Expression::isConstValue(e2.clone())?) else { bail!("pattern mismatch") };
                    e = Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::ADD { ty: ty.clone() }, exp2: e2.clone() });
                    res = Expression::makePureBuiltinCall((literal!("exp")).clone(), list![e.clone()], ty.clone());
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, op1 @ DAE::Operator::DIV { .. }, Deref @ DAE::Exp::BINARY { exp2: e2, operator: op2 @ DAE::Operator::ADD { .. }, exp1: e1 }, e3, _, true) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut e4: Arc<DAE::Exp>;
                    let mut b: bool = false;
                    let mut b2: bool = false;
                    (e, b) = simplify1(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op1.clone(), exp2: e3.clone() }))?;
                    (e4, b2) = simplify1(Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op1.clone(), exp2: e3.clone() }))?;
                    let true = (b.clone() || b2.clone()) else { bail!("pattern mismatch") };
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e.clone(), operator: op2.clone(), exp2: e4.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, op1 @ DAE::Operator::DIV { .. }, Deref @ DAE::Exp::BINARY { exp2: e2, operator: op2 @ DAE::Operator::SUB { .. }, exp1: e1 }, e3, _, true) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut e4: Arc<DAE::Exp>;
                    let mut b: bool = false;
                    let mut b2: bool = false;
                    (e, b) = simplify1(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op1.clone(), exp2: e3.clone() }))?;
                    (e4, b2) = simplify1(Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op1.clone(), exp2: e3.clone() }))?;
                    let true = (b.clone() || b2.clone()) else { bail!("pattern mismatch") };
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e.clone(), operator: op2.clone(), exp2: e4.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::DIV { ty: tp }, e1, Deref @ DAE::Exp::BINARY { exp2: e3, operator: op2 @ DAE::Operator::DIV { .. }, exp1: e2 }, _, _) => {
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: e3.clone() }), operator: op2.clone(), exp2: e2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::DIV { ty: tp }, Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::DIV { ty: tp2 }, exp1: e1 }, e3, _, _) => {
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::DIV { ty: tp2.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: e3.clone() }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::DIV { .. }, e1, Deref @ DAE::Exp::BINARY { exp2: e3, operator: DAE::Operator::MUL { ty: tp2 }, exp1: e2 }, _, _) => {
                    let true = (ExpressionBasics::expEqual(e1.clone(), e3.clone())?) else { bail!("pattern mismatch") };
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), operator: DAE::Operator::DIV { ty: tp2.clone() }, exp2: e2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::DIV { .. }, e1, Deref @ DAE::Exp::BINARY { exp2: e3, operator: DAE::Operator::MUL { ty: tp2 }, exp1: e2 }, _, _) => {
                    let true = (ExpressionBasics::expEqual(e1.clone(), e2.clone())?) else { bail!("pattern mismatch") };
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), operator: DAE::Operator::DIV { ty: tp2.clone() }, exp2: e3.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::DIV { .. }, Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::MUL { .. }, exp1: e1 }, e3, _, _) => {
                    let true = (ExpressionBasics::expEqual(e1.clone(), e3.clone())?) else { bail!("pattern mismatch") };
                    Ok(e2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::DIV { .. }, Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::MUL { .. }, exp1: e1 }, e3, _, _) => {
                    let true = (ExpressionBasics::expEqual(e2.clone(), e3.clone())?) else { bail!("pattern mismatch") };
                    Ok(e1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::DIV { .. }, Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::MUL { .. }, exp1: Deref @ DAE::Exp::UNARY { exp: e1, operator: DAE::Operator::UMINUS { .. } } }, e3, _, _) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut tp2: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let true = (ExpressionBasics::expEqual(e1.clone(), e3.clone())?) else { bail!("pattern mismatch") };
                    tp2 = Expression::r#typeof(e2.clone())?;
                    e = Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp2.clone() }, exp: e2.clone() });
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::DIV { .. }, Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::MUL { .. }, exp1: Deref @ DAE::Exp::UNARY { exp: e1, operator: DAE::Operator::UMINUS { .. } } }, e3, _, _) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut tp2: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let true = (ExpressionBasics::expEqual(e2.clone(), e3.clone())?) else { bail!("pattern mismatch") };
                    tp2 = Expression::r#typeof(e1.clone())?;
                    e = Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp2.clone() }, exp: e1.clone() });
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::DIV { .. }, Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::MUL { .. }, exp1: e1 }, Deref @ DAE::Exp::UNARY { exp: e3, operator: DAE::Operator::UMINUS { .. } }, _, _) => {
                    let mut tp2: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let true = (ExpressionBasics::expEqual(e2.clone(), e3.clone())?) else { bail!("pattern mismatch") };
                    tp2 = Expression::r#typeof(e1.clone())?;
                    Ok(Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp2.clone() }, exp: e1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::DIV { .. }, Deref @ DAE::Exp::BINARY { exp2: e2, operator: DAE::Operator::MUL { .. }, exp1: e1 }, Deref @ DAE::Exp::UNARY { exp: e3, operator: DAE::Operator::UMINUS { .. } }, _, _) => {
                    let mut tp2: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let true = (ExpressionBasics::expEqual(e1.clone(), e3.clone())?) else { bail!("pattern mismatch") };
                    tp2 = Expression::r#typeof(e2.clone())?;
                    Ok(Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp2.clone() }, exp: e2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::SUB { ty }, e1, e2, true, _) => {
                    let true = (Expression::isZero(e1.clone())) else { bail!("pattern mismatch") };
                    Ok(Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: ty.clone() }, exp: e2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::SUB { .. }, e1, e2, _, true) => {
                    let true = (Expression::isZero(e2.clone())) else { bail!("pattern mismatch") };
                    Ok(e1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::SUB { ty }, e1, e2, _, _) => {
                    let true = (ExpressionBasics::expEqual(e1.clone(), e2.clone())?) else { bail!("pattern mismatch") };
                    Ok(Expression::makeConstZero(ty.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::ADD { ty }, e1, e2, _, _) => {
                    let mut e: Arc<DAE::Exp>;
                    let true = (Types::isRealOrSubTypeReal(ty.clone())?) else { bail!("pattern mismatch") };
                    let true = (ExpressionBasics::expEqual(e1.clone(), e2.clone())?) else { bail!("pattern mismatch") };
                    e = Expression::makeConstNumber(ty.clone(), 2);
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e.clone(), operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: e1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::SUB { ty }, e1, Deref @ DAE::Exp::UNARY { exp: e2, operator: DAE::Operator::UMINUS { .. } }, _, _) => {
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::ADD { ty: ty.clone() }, exp2: e2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::SUB { ty }, e1, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { exp: e2, operator: DAE::Operator::UMINUS { .. } }, operator: op1 @ DAE::Operator::MUL { ty: _ }, exp2: e3 }, _, _) => {
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::ADD { ty: ty.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op1.clone(), exp2: e3.clone() }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::SUB { ty }, e1, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { exp: e2, operator: DAE::Operator::UMINUS { .. } }, operator: op1 @ DAE::Operator::DIV { ty: _ }, exp2: e3 }, _, _) => {
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::ADD { ty: ty.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op1.clone(), exp2: e3.clone() }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::DIV { .. }, e1, e2, true, false) => {
                    let true = (Expression::isZero(e1.clone())) else { bail!("pattern mismatch") };
                    let false = (Expression::isZero(e2.clone())) else { bail!("pattern mismatch") };
                    Ok(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::DIV { .. }, e1, e2, false, true) => {
                    let true = (Expression::isConstOne(e2.clone())) else { bail!("pattern mismatch") };
                    Ok(e1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::DIV { ty }, e1, e2, _, _) => {
                    let mut e: Arc<DAE::Exp>;
                    let true = (Expression::isConstMinusOne(e2.clone())) else { bail!("pattern mismatch") };
                    e = Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: ty.clone() }, exp: e1.clone() });
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::DIV { ty }, e1, e2, _, _) => {
                    let mut res: Arc<DAE::Exp>;
                    let false = (Expression::isZero(e2.clone())) else { bail!("pattern mismatch") };
                    let true = (ExpressionBasics::expEqual(e1.clone(), e2.clone())?) else { bail!("pattern mismatch") };
                    res = Expression::makeConstOne(ty.clone());
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::MUL { ty }, e1, e2, _, _) => {
                    let mut res: Arc<DAE::Exp>;
                    let false = (Expression::isZero(e2.clone())) else { bail!("pattern mismatch") };
                    let true = (Types::isRealOrSubTypeReal(ty.clone())?) else { bail!("pattern mismatch") };
                    let true = (ExpressionBasics::expEqual(e1.clone(), e2.clone())?) else { bail!("pattern mismatch") };
                    res = Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::POW { ty: ty.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }) });
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::DIV { ty: tp }, e1, Deref @ DAE::Exp::RCONST { real: r1 }, _, _) => {
                    let mut e3: Arc<DAE::Exp>;
                    let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut r1 = (*r1).clone();
                    let true = (realAbs(r1.clone()) > metamodelica::OrderedFloat(0.0_f64)) else { bail!("pattern mismatch") };
                    r = metamodelica::OrderedFloat(1.0_f64) / r1.clone();
                    r1 = metamodelica::OrderedFloat(1e12_f64) * r.clone();
                    let _ /* lit — guard not yet implemented */ = (realMod(r1.clone(), metamodelica::OrderedFloat(1.0_f64))) else { bail!("pattern mismatch") };
                    e3 = Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: r.clone() }), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: e1.clone() });
                    Ok(e3.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, op2 @ DAE::Operator::DIV { ty: tp }, e1, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::RCONST { real: r1 }, operator: DAE::Operator::MUL { ty: _ }, exp2: e3 }, _, _) => {
                    let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut r1 = (*r1).clone();
                    let true = (realAbs(r1.clone()) > metamodelica::OrderedFloat(0.0_f64)) else { bail!("pattern mismatch") };
                    r = metamodelica::OrderedFloat(1.0_f64) / r1.clone();
                    r1 = metamodelica::OrderedFloat(1e12_f64) * r.clone();
                    let _ /* lit — guard not yet implemented */ = (realMod(r1.clone(), metamodelica::OrderedFloat(1.0_f64))) else { bail!("pattern mismatch") };
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: r.clone() }), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: e1.clone() }), operator: op2.clone(), exp2: e3.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, op1 @ DAE::Operator::DIV { .. }, Deref @ DAE::Exp::UNARY { exp: e1, operator: DAE::Operator::UMINUS { .. } }, Deref @ DAE::Exp::UNARY { exp: e2, operator: DAE::Operator::UMINUS { .. } }, _, _) => {
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op1.clone(), exp2: e2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, op2 @ DAE::Operator::MUL { .. }, Deref @ DAE::Exp::UNARY { exp: e1, operator: DAE::Operator::UMINUS { .. } }, Deref @ DAE::Exp::BINARY { exp1: e2, operator: op1 @ DAE::Operator::SUB { ty: _ }, exp2: e3 }, _, _) => {
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op2.clone(), exp2: Arc::new(DAE::Exp::BINARY { exp1: e3.clone(), operator: op1.clone(), exp2: e2.clone() }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, op2 @ DAE::Operator::DIV { .. }, Deref @ DAE::Exp::UNARY { exp: e1, operator: DAE::Operator::UMINUS { .. } }, Deref @ DAE::Exp::BINARY { exp1: e2, operator: op1 @ DAE::Operator::SUB { ty: _ }, exp2: e3 }, _, _) => {
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op2.clone(), exp2: Arc::new(DAE::Exp::BINARY { exp1: e3.clone(), operator: op1.clone(), exp2: e2.clone() }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, op2 @ DAE::Operator::MUL { .. }, e1, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { exp: e2, operator: op3 @ DAE::Operator::UMINUS { .. } }, operator: DAE::Operator::SUB { ty }, exp2: e3 }, _, _) => {
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::UNARY { operator: op3.clone(), exp: e1.clone() }), operator: op2.clone(), exp2: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::ADD { ty: ty.clone() }, exp2: e3.clone() }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, op2 @ DAE::Operator::DIV { .. }, e1, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { exp: e2, operator: op3 @ DAE::Operator::UMINUS { .. } }, operator: DAE::Operator::SUB { ty }, exp2: e3 }, _, _) => {
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::UNARY { operator: op3.clone(), exp: e1.clone() }), operator: op2.clone(), exp2: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::ADD { ty: ty.clone() }, exp2: e3.clone() }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, op2 @ DAE::Operator::POW { .. }, Deref @ DAE::Exp::UNARY { exp: e1, operator: DAE::Operator::UMINUS { .. } }, e2 @ Deref @ DAE::Exp::RCONST { real: __rlit_8 }, _, _) => {
                    if !(__rlit_8.eq(&metamodelica::OrderedFloat((2.0) as f64))) { bail!("guard") }
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op2.clone(), exp2: e2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, op1 @ DAE::Operator::DIV { ty }, e1, Deref @ DAE::Exp::UNARY { exp: e2, operator: DAE::Operator::UMINUS { .. } }, _, _) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    e1_1 = Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: ty.clone() }, exp: e1.clone() });
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op1.clone(), exp2: e2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, op1 @ DAE::Operator::DIV { .. }, Deref @ DAE::Exp::BINARY { exp2: e3, operator: op2 @ DAE::Operator::MUL { .. }, exp1: e2 }, e1, _, true) => {
                    let mut e: Arc<DAE::Exp>;
                    let true = (Expression::isConstValue(e3.clone())?) else { bail!("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(simplify1(Arc::new(DAE::Exp::BINARY { exp1: e3.clone(), operator: op1.clone(), exp2: e1.clone() }))?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e = __pa0.clone();
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e.clone(), operator: op2.clone(), exp2: e2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, op1 @ DAE::Operator::DIV { .. }, Deref @ DAE::Exp::BINARY { exp2: e3, operator: op2 @ DAE::Operator::MUL { .. }, exp1: e2 }, e1, _, true) => {
                    let mut e: Arc<DAE::Exp>;
                    let true = (Expression::isConstValue(e2.clone())?) else { bail!("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(simplify1(Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op1.clone(), exp2: e1.clone() }))?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e = __pa0.clone();
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e.clone(), operator: op2.clone(), exp2: e3.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::POW { .. }, e1, e, _, true) => {
                    let true = (Expression::isConstOne(e.clone())) else { bail!("pattern mismatch") };
                    Ok(e1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::POW { ty: tp }, e2, e, _, _) => {
                    let mut one: Arc<DAE::Exp>;
                    let true = (Expression::isConstMinusOne(e.clone())) else { bail!("pattern mismatch") };
                    one = Expression::makeConstOne(tp.clone());
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: one.clone(), operator: DAE::Operator::DIV { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::POW { .. }, e1, e, _, true) => {
                    let mut tp: Type = Arc::new(DAE::Type::T_NORETCALL);
                    let true = (Expression::isZero(e.clone())) else { bail!("pattern mismatch") };
                    tp = Expression::r#typeof(e1.clone())?;
                    Ok(Expression::makeConstOne(tp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::POW { .. }, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, .. }, Deref @ DAE::Exp::RCONST { real: __rlit_9 }, _, _) => {
                    if !(__rlit_9.eq(&metamodelica::OrderedFloat((2.0) as f64))) { bail!("guard") }
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, oper @ DAE::Operator::POW { .. }, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, .. }, e, _, _) => {
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: oper.clone(), exp2: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.5_f64) }), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e.clone() }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::DIV { .. }, e1, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, .. }, _, _) => {
                    let true = (ExpressionBasics::expEqual(e1.clone(), e2.clone())?) else { bail!("pattern mismatch") };
                    Ok(Expression::makePureBuiltinCall((literal!("sqrt")).clone(), list![e1.clone()], DAE::T_REAL_DEFAULT().clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::DIV { .. }, Deref @ DAE::Exp::BINARY { exp1: e1, operator: op1 @ DAE::Operator::POW { ty }, exp2: e2 }, e3, _, _) => {
                    let mut e4: Arc<DAE::Exp>;
                    let true = (ExpressionBasics::expEqual(e1.clone(), e3.clone())?) else { bail!("pattern mismatch") };
                    e4 = Expression::makeConstOne(ty.clone());
                    e4 = Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::SUB { ty: ty.clone() }, exp2: e4.clone() });
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op1.clone(), exp2: e4.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::DIV { .. }, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::BINARY { exp1: e1, operator: op1 @ DAE::Operator::POW { ty }, exp2: e2 }, operator: op2 @ DAE::Operator::MUL { ty: _ }, exp2: e5 }, e3, _, _) => {
                    let mut e4: Arc<DAE::Exp>;
                    let true = (ExpressionBasics::expEqual(e1.clone(), e3.clone())?) else { bail!("pattern mismatch") };
                    e4 = Expression::makeConstOne(ty.clone());
                    e4 = Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::SUB { ty: ty.clone() }, exp2: e4.clone() });
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op1.clone(), exp2: e4.clone() }), operator: op2.clone(), exp2: e5.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::DIV { .. }, e3, Deref @ DAE::Exp::BINARY { exp1: e1, operator: op1 @ DAE::Operator::POW { ty }, exp2: e2 }, _, _) => {
                    let mut e4: Arc<DAE::Exp>;
                    let true = (ExpressionBasics::expEqual(e1.clone(), e3.clone())?) else { bail!("pattern mismatch") };
                    e4 = Expression::makeConstOne(ty.clone());
                    e4 = Arc::new(DAE::Exp::BINARY { exp1: e4.clone(), operator: DAE::Operator::SUB { ty: ty.clone() }, exp2: e2.clone() });
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op1.clone(), exp2: e4.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, op2 @ DAE::Operator::POW { .. }, Deref @ DAE::Exp::BINARY { exp1: e1, operator: op1 @ DAE::Operator::DIV { ty: _ }, exp2: e2 }, Deref @ DAE::Exp::RCONST { real: r }, _, _) => {
                    let mut r = (*r).clone();
                    let true = (realLt(r.clone(), metamodelica::OrderedFloat(0.0_f64))) else { bail!("pattern mismatch") };
                    r = -(r.clone());
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op1.clone(), exp2: e1.clone() }), operator: op2.clone(), exp2: Arc::new(DAE::Exp::RCONST { real: r.clone() }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::POW { .. }, e1, _, true, _) => {
                    let true = (Expression::isConstOne(e1.clone())) else { bail!("pattern mismatch") };
                    Ok(e1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, op1, Deref @ DAE::Exp::IFEXP { expElse: e3, expThen: e2, expCond: e1 }, Deref @ DAE::Exp::IFEXP { expElse: e6, expThen: e5, expCond: e4 }, _, _) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut res: Arc<DAE::Exp>;
                    let true = (ExpressionBasics::expEqual(e1.clone(), e4.clone())?) else { bail!("pattern mismatch") };
                    e = Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op1.clone(), exp2: e5.clone() });
                    res = Arc::new(DAE::Exp::BINARY { exp1: e3.clone(), operator: op1.clone(), exp2: e6.clone() });
                    Ok(Arc::new(DAE::Exp::IFEXP { expCond: e1.clone(), expThen: e.clone(), expElse: res.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::SUB { ty }, Deref @ DAE::Exp::BINARY { exp1: e @ Deref @ DAE::Exp::UNARY { exp: e1, operator: DAE::Operator::UMINUS { .. } }, operator: op2 @ DAE::Operator::MUL { .. }, exp2: e2 }, Deref @ DAE::Exp::BINARY { exp1: e3, operator: DAE::Operator::MUL { .. }, exp2: e4 }, false, false) => {
                    let mut res: Arc<DAE::Exp>;
                    let true = (ExpressionBasics::expEqual(e1.clone(), e3.clone())?) else { bail!("pattern mismatch") };
                    res = Arc::new(DAE::Exp::BINARY { exp1: e.clone(), operator: op2.clone(), exp2: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::ADD { ty: ty.clone() }, exp2: e4.clone() }) });
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::SUB { ty }, Deref @ DAE::Exp::BINARY { exp1: e @ Deref @ DAE::Exp::UNARY { exp: e1, operator: DAE::Operator::UMINUS { .. } }, operator: DAE::Operator::DIV { .. }, exp2: e2 }, Deref @ DAE::Exp::BINARY { exp1: e3, operator: DAE::Operator::DIV { .. }, exp2: e4 }, false, false) => {
                    let mut res: Arc<DAE::Exp>;
                    let true = (ExpressionBasics::expEqual(e1.clone(), e3.clone())?) else { bail!("pattern mismatch") };
                    res = Arc::new(DAE::Exp::BINARY { exp1: e.clone(), operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: Expression::inverseFactors(e2.clone())?, operator: DAE::Operator::ADD { ty: ty.clone() }, exp2: Expression::inverseFactors(e4.clone())? }) });
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, op1, Deref @ DAE::Exp::BINARY { exp1: e1, operator: oper @ DAE::Operator::MUL { ty: _ }, exp2: Deref @ DAE::Exp::BINARY { exp1: e2, operator: op2, exp2: e3 } }, Deref @ DAE::Exp::BINARY { exp1: e4, operator: DAE::Operator::MUL { ty: _ }, exp2: Deref @ DAE::Exp::BINARY { exp1: e5, operator: op3, exp2: e6 } }, false, false) => {
                    let true = (Expression::isAddOrSub(op1.clone())) else { bail!("pattern mismatch") };
                    let true = (Expression::isMulOrDiv(op2.clone())) else { bail!("pattern mismatch") };
                    let true = (Expression::isMulOrDiv(op3.clone())) else { bail!("pattern mismatch") };
                    let true = (ExpressionBasics::expEqual(e2.clone(), e5.clone())?) else { bail!("pattern mismatch") };
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e5.clone(), operator: oper.clone(), exp2: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op2.clone(), exp2: e3.clone() }), operator: op1.clone(), exp2: Arc::new(DAE::Exp::BINARY { exp1: e4.clone(), operator: op3.clone(), exp2: e6.clone() }) }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, op1, Deref @ DAE::Exp::BINARY { exp1: e1, operator: oper @ DAE::Operator::MUL { ty: _ }, exp2: e2 }, Deref @ DAE::Exp::BINARY { exp1: e4, operator: DAE::Operator::MUL { ty: _ }, exp2: Deref @ DAE::Exp::BINARY { exp1: e5, operator: op3, exp2: e6 } }, false, false) => {
                    let true = (Expression::isAddOrSub(op1.clone())) else { bail!("pattern mismatch") };
                    let true = (Expression::isMulOrDiv(op3.clone())) else { bail!("pattern mismatch") };
                    let true = (ExpressionBasics::expEqual(e2.clone(), e5.clone())?) else { bail!("pattern mismatch") };
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e5.clone(), operator: oper.clone(), exp2: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op1.clone(), exp2: Arc::new(DAE::Exp::BINARY { exp1: e4.clone(), operator: op3.clone(), exp2: e6.clone() }) }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, op1, Deref @ DAE::Exp::BINARY { exp1: e1, operator: oper @ DAE::Operator::MUL { ty: _ }, exp2: Deref @ DAE::Exp::BINARY { exp1: e2, operator: op2, exp2: e3 } }, Deref @ DAE::Exp::BINARY { exp1: e4, operator: DAE::Operator::MUL { .. }, exp2: e5 }, false, false) => {
                    let mut outExp: Arc<DAE::Exp>;
                    let true = (Expression::isAddOrSub(op1.clone())) else { bail!("pattern mismatch") };
                    let true = (Expression::isMulOrDiv(op2.clone())) else { bail!("pattern mismatch") };
                    if ExpressionBasics::expEqual(e2.clone(), e5.clone())? {
                        outExp = Arc::new(DAE::Exp::BINARY { exp1: e5.clone(), operator: oper.clone(), exp2: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op2.clone(), exp2: e3.clone() }), operator: op1.clone(), exp2: e4.clone() }) });
                    } else {
                        if ExpressionBasics::expEqual(e2.clone(), e4.clone())? {
                            outExp = Arc::new(DAE::Exp::BINARY { exp1: e4.clone(), operator: oper.clone(), exp2: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op2.clone(), exp2: e3.clone() }), operator: op1.clone(), exp2: e5.clone() }) });
                        } else {
                            bail!("fail");
                        }
                    }
                    Ok(outExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, op1, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::BINARY { exp1: e1, operator: oper @ DAE::Operator::MUL { ty: _ }, exp2: e2 }, operator: op2, exp2: e3 }, Deref @ DAE::Exp::BINARY { exp1: e4, operator: DAE::Operator::MUL { .. }, exp2: e5 }, false, false) => {
                    let mut outExp: Arc<DAE::Exp>;
                    let true = (Expression::isAddOrSub(op1.clone())) else { bail!("pattern mismatch") };
                    let true = (Expression::isMulOrDiv(op2.clone())) else { bail!("pattern mismatch") };
                    if ExpressionBasics::expEqual(e2.clone(), e5.clone())? {
                        outExp = Arc::new(DAE::Exp::BINARY { exp1: e5.clone(), operator: oper.clone(), exp2: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op2.clone(), exp2: e3.clone() }), operator: op1.clone(), exp2: e4.clone() }) });
                    } else {
                        if ExpressionBasics::expEqual(e2.clone(), e4.clone())? {
                            outExp = Arc::new(DAE::Exp::BINARY { exp1: e4.clone(), operator: oper.clone(), exp2: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op2.clone(), exp2: e3.clone() }), operator: op1.clone(), exp2: e5.clone() }) });
                        } else {
                            bail!("fail");
                        }
                    }
                    Ok(outExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::POW { .. }, e1, e2 @ Deref @ DAE::Exp::RCONST { real: r }, _, true) => {
                    if !((r.clone() != intReal(((r.clone()).0 as i32)))) { bail!("guard") }
                    let mut e: Arc<DAE::Exp>;
                    let mut exp_lst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut exp_lst_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut outExp: Arc<DAE::Exp>;
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::factors(e1.clone())?) {
                        __pa0 @ Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } } } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    exp_lst = __pa0.clone();
                    let true = (List::any(exp_lst.clone(), (std::sync::Arc::new(fnptr!(Expression::isEvaluatedConst, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
                    (exp_lst, exp_lst_1) = List::splitOnTrue(exp_lst.clone(), (std::sync::Arc::new(Expression::isPositiveOrZero) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>));
                    exp_lst = simplifyBinaryDistributePow(exp_lst.clone(), e2.clone())?;
                    e = Expression::makeProductLst(exp_lst_1.clone())?;
                    e = Arc::new(DAE::Exp::BINARY { exp1: e.clone(), operator: inOperator2.clone(), exp2: e2.clone() });
                    outExp = Expression::makeProductLst(cons(e.clone(), exp_lst.clone()))?;
                    Ok(outExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::POW { .. }, e1, e2, _, true) => {
                    if !((Expression::isEvaluatedConst(e2.clone()))) { bail!("guard") }
                    let mut exp_lst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut exp_lst_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::factors(e1.clone())?) {
                        __pa0 @ Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } } } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    exp_lst = __pa0.clone();
                    let true = (List::any(exp_lst.clone(), (std::sync::Arc::new(fnptr!(Expression::isEvaluatedConst, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
                    exp_lst_1 = simplifyBinaryDistributePow(exp_lst.clone(), e2.clone())?;
                    Ok(Expression::makeProductLst(exp_lst_1.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::POW { .. }, Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { .. }, exp2: e2 }, e3, _, _) => {
                    if !((Expression::isEven(e2.clone()))) { bail!("guard") }
                    let mut e: Arc<DAE::Exp>;
                    if Expression::isEvaluatedConst(e3.clone()) {
                        e = simplifyBinaryConst(DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, e2.clone(), e3.clone())?;
                        let false = (Expression::isEven(e.clone())) else { bail!("pattern mismatch") };
                    } else {
                        e = Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e3.clone() });
                    }
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: Expression::makePureBuiltinCall((literal!("abs")).clone(), list![e1.clone()], Expression::r#typeof(e1.clone())?), operator: DAE::Operator::POW { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::POW { .. }, Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { .. }, exp2: e2 }, e3, _, _) => {
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::POW { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e3.clone() }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::DIV { ty }, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sin" }, .. }, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "cos" }, .. }, _, _) => {
                    let true = (ExpressionBasics::expEqual(e1.clone(), e2.clone())?) else { bail!("pattern mismatch") };
                    Ok(Expression::makePureBuiltinCall((literal!("tan")).clone(), list![e1.clone()], ty.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, op2 @ DAE::Operator::DIV { ty }, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "tan" }, .. }, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sin" }, .. }, _, _) => {
                    let mut e3: Arc<DAE::Exp>;
                    let mut e: Arc<DAE::Exp>;
                    let mut e4: Arc<DAE::Exp>;
                    let true = (ExpressionBasics::expEqual(e1.clone(), e2.clone())?) else { bail!("pattern mismatch") };
                    e3 = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) });
                    e4 = Expression::makePureBuiltinCall((literal!("cos")).clone(), list![e2.clone()], ty.clone());
                    e = Arc::new(DAE::Exp::BINARY { exp1: e3.clone(), operator: op2.clone(), exp2: e4.clone() });
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::DIV { ty }, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sin" }, .. }, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "tan" }, .. }, _, _) => {
                    let mut e: Arc<DAE::Exp>;
                    let true = (ExpressionBasics::expEqual(e1.clone(), e2.clone())?) else { bail!("pattern mismatch") };
                    e = Expression::makePureBuiltinCall((literal!("cos")).clone(), list![e2.clone()], ty.clone());
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, op2 @ DAE::Operator::DIV { ty }, e1, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "tan" }, .. }, _, _) => {
                    let mut e3: Arc<DAE::Exp>;
                    let mut e: Arc<DAE::Exp>;
                    let mut e4: Arc<DAE::Exp>;
                    e3 = Expression::makePureBuiltinCall((literal!("sin")).clone(), list![e2.clone()], ty.clone());
                    e4 = Expression::makePureBuiltinCall((literal!("cos")).clone(), list![e2.clone()], ty.clone());
                    e = Arc::new(DAE::Exp::BINARY { exp1: e4.clone(), operator: op2.clone(), exp2: e3.clone() });
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: e.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::DIV { ty }, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sinh" }, .. }, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "cosh" }, .. }, _, _) => {
                    let true = (ExpressionBasics::expEqual(e1.clone(), e2.clone())?) else { bail!("pattern mismatch") };
                    Ok(Expression::makePureBuiltinCall((literal!("tanh")).clone(), list![e1.clone()], ty.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, op2 @ DAE::Operator::DIV { ty }, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "tanh" }, .. }, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sinh" }, .. }, _, _) => {
                    let mut e3: Arc<DAE::Exp>;
                    let mut e: Arc<DAE::Exp>;
                    let mut e4: Arc<DAE::Exp>;
                    let true = (ExpressionBasics::expEqual(e1.clone(), e2.clone())?) else { bail!("pattern mismatch") };
                    e3 = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) });
                    e4 = Expression::makePureBuiltinCall((literal!("cosh")).clone(), list![e2.clone()], ty.clone());
                    e = Arc::new(DAE::Exp::BINARY { exp1: e3.clone(), operator: op2.clone(), exp2: e4.clone() });
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::DIV { ty }, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sinh" }, .. }, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "tanh" }, .. }, _, _) => {
                    let mut e: Arc<DAE::Exp>;
                    let true = (ExpressionBasics::expEqual(e1.clone(), e2.clone())?) else { bail!("pattern mismatch") };
                    e = Expression::makePureBuiltinCall((literal!("cosh")).clone(), list![e2.clone()], ty.clone());
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::MUL { ty }, e1, Deref @ DAE::Exp::UNARY { exp: e2, operator: DAE::Operator::UMINUS { .. } }, _, _) => {
                    let mut e1_1: Arc<DAE::Exp>;
                    e1_1 = Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: ty.clone() }, exp: e1.clone() });
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: e2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::ADD { .. }, Deref @ DAE::Exp::RANGE { stop: e2, step: oexp, start: e1, ty }, _, _, _) => {
                    let mut e2 = (*e2).clone();
                    let mut e1 = (*e1).clone();
                    e1 = simplifyBinary(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: inOperator2.clone(), exp2: rhs.clone() }), inOperator2.clone(), e1.clone(), rhs.clone())?;
                    e2 = simplifyBinary(Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: inOperator2.clone(), exp2: rhs.clone() }), inOperator2.clone(), e2.clone(), rhs.clone())?;
                    Ok(Arc::new(DAE::Exp::RANGE { ty: ty.clone(), start: e1.clone(), step: oexp.clone(), stop: e2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::ADD { .. }, _, Deref @ DAE::Exp::RANGE { stop: e2, step: oexp, start: e1, ty }, _, _) => {
                    let mut e2 = (*e2).clone();
                    let mut e1 = (*e1).clone();
                    e1 = simplifyBinary(Arc::new(DAE::Exp::BINARY { exp1: lhs.clone(), operator: inOperator2.clone(), exp2: e1.clone() }), inOperator2.clone(), lhs.clone(), e1.clone())?;
                    e2 = simplifyBinary(Arc::new(DAE::Exp::BINARY { exp1: lhs.clone(), operator: inOperator2.clone(), exp2: e1.clone() }), inOperator2.clone(), lhs.clone(), e2.clone())?;
                    Ok(Arc::new(DAE::Exp::RANGE { ty: ty.clone(), start: e1.clone(), step: oexp.clone(), stop: e2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::SUB { .. }, Deref @ DAE::Exp::RANGE { stop: e2, step: oexp, start: e1, ty }, _, _, _) => {
                    let mut e2 = (*e2).clone();
                    let mut e1 = (*e1).clone();
                    e1 = simplifyBinary(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: inOperator2.clone(), exp2: rhs.clone() }), inOperator2.clone(), e1.clone(), rhs.clone())?;
                    e2 = simplifyBinary(Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: inOperator2.clone(), exp2: rhs.clone() }), inOperator2.clone(), e2.clone(), rhs.clone())?;
                    Ok(Arc::new(DAE::Exp::RANGE { ty: ty.clone(), start: e1.clone(), step: oexp.clone(), stop: e2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::SUB { .. }, _, Deref @ DAE::Exp::RANGE { stop: e2, step: oexp, start: e1, ty }, _, _) => {
                    let mut e2 = (*e2).clone();
                    let mut e1 = (*e1).clone();
                    e1 = simplifyBinary(Arc::new(DAE::Exp::BINARY { exp1: lhs.clone(), operator: inOperator2.clone(), exp2: e1.clone() }), inOperator2.clone(), lhs.clone(), e1.clone())?;
                    e2 = simplifyBinary(Arc::new(DAE::Exp::BINARY { exp1: lhs.clone(), operator: inOperator2.clone(), exp2: e1.clone() }), inOperator2.clone(), lhs.clone(), e2.clone())?;
                    Ok(Arc::new(DAE::Exp::RANGE { ty: ty.clone(), start: e1.clone(), step: oexp.clone(), stop: e2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(origExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

fn simplifyTwoBinaryExpressions(mut e1: Arc<DAE::Exp>, mut lhsOperator: Operator, mut e2: Arc<DAE::Exp>, mut mainOperator: Operator, mut e3: Arc<DAE::Exp>, mut rhsOperator: Operator, mut e4: Arc<DAE::Exp>, mut expEqual_e1_e3: bool, mut expEqual_e1_e4: bool, mut expEqual_e2_e3: bool, mut expEqual_e2_e4: bool, mut isConst_e1: bool, mut isConst_e2: bool, mut isConst_e3: bool, mut operatorEqualLhsRhs: bool) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &((e1.clone(), lhsOperator.clone(), e2.clone(), mainOperator.clone(), e3.clone(), rhsOperator.clone(), e4.clone(), expEqual_e1_e3.clone(), expEqual_e1_e4.clone(), expEqual_e2_e3.clone(), expEqual_e2_e4.clone(), isConst_e1.clone(), isConst_e2.clone(), isConst_e3.clone(), operatorEqualLhsRhs.clone())) {
        (_, op2 @ DAE::Operator::MUL { .. }, _, op1 @ DAE::Operator::ADD { .. }, _, DAE::Operator::MUL { .. }, _, true, _, _, _, _, _, _, _) => {
            Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op2.clone(), exp2: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op1.clone(), exp2: e4.clone() }) })
        },
        (_, op2 @ DAE::Operator::MUL { .. }, _, op1 @ DAE::Operator::ADD { .. }, _, DAE::Operator::MUL { .. }, _, _, true, _, _, _, _, _, _) => {
            Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op2.clone(), exp2: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op1.clone(), exp2: e3.clone() }) })
        },
        (_, op2 @ DAE::Operator::MUL { .. }, _, op1 @ DAE::Operator::ADD { .. }, _, DAE::Operator::MUL { .. }, _, _, _, true, _, _, _, _, _) => {
            Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op2.clone(), exp2: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op1.clone(), exp2: e4.clone() }) })
        },
        (_, op2 @ DAE::Operator::MUL { .. }, _, op1 @ DAE::Operator::ADD { .. }, _, DAE::Operator::MUL { .. }, _, _, _, _, true, _, _, _, _) => {
            Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op2.clone(), exp2: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op1.clone(), exp2: e3.clone() }) })
        },
        (_, DAE::Operator::POW { .. }, _, DAE::Operator::MUL { .. }, _, DAE::Operator::POW { .. }, _, _, _, _, true, _, _, _, _) => {
            let mut res: Arc<DAE::Exp>;
            res = Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: mainOperator.clone(), exp2: e3.clone() }), operator: lhsOperator.clone(), exp2: e2.clone() });
            res.clone()
        },
        (_, DAE::Operator::POW { .. }, _, DAE::Operator::DIV { .. }, _, DAE::Operator::POW { .. }, _, _, _, _, true, _, _, _, _) => {
            let mut res: Arc<DAE::Exp>;
            res = Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: mainOperator.clone(), exp2: e3.clone() }), operator: lhsOperator.clone(), exp2: e2.clone() });
            res.clone()
        },
        (_, DAE::Operator::POW { .. }, _, DAE::Operator::MUL { .. }, _, DAE::Operator::POW { .. }, _, true, _, _, _, _, _, _, _) => {
            let mut res: Arc<DAE::Exp>;
            res = Expression::expAdd(e2.clone(), e4.clone())?;
            res = Expression::expPow(e1.clone(), res.clone())?;
            res.clone()
        },
        (_, DAE::Operator::POW { .. }, _, DAE::Operator::DIV { .. }, _, DAE::Operator::POW { .. }, _, true, _, _, _, _, _, _, _) => {
            let mut res: Arc<DAE::Exp>;
            res = Expression::expSub(e2.clone(), e4.clone())?;
            res = Expression::expPow(e1.clone(), res.clone())?;
            res.clone()
        },
        (_, op2, _, op1, _, _, _, _, _, _, true, _, false, _, true) if (Expression::isAddOrSub(op1.clone()) && Expression::isMulOrDiv(op2.clone())) => {
            Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op1.clone(), exp2: e3.clone() }), operator: op2.clone(), exp2: e4.clone() })
        },
        (_, op @ DAE::Operator::MUL { ty }, _, op1, _, DAE::Operator::DIV { ty: _ }, _, true, _, _, _, false, _, _, _) if (Expression::isAddOrSub(op1.clone())) => {
            let mut e: Arc<DAE::Exp>;
            let mut one: Arc<DAE::Exp>;
            one = Expression::makeConstOne(ty.clone());
            e = Expression::makeDiv(one.clone(), e4.clone())?;
            Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op1.clone(), exp2: e.clone() }), operator: op.clone(), exp2: e1.clone() })
        },
        (_, DAE::Operator::DIV { ty }, _, op1, _, DAE::Operator::MUL { ty: _ }, _, true, _, _, _, false, _, _, _) if (Expression::isAddOrSub(op1.clone())) => {
            let mut e: Arc<DAE::Exp>;
            let mut one: Arc<DAE::Exp>;
            one = Expression::makeConstOne(ty.clone());
            e = Expression::makeDiv(one.clone(), e2.clone())?;
            Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e.clone(), operator: op1.clone(), exp2: e4.clone() }), operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: e1.clone() })
        },
        (e1_1, op2, e_3, op1, e, _, _, _, _, _, true, _, false, _, true) if (Expression::isAddOrSub(op1.clone()) && Expression::isMulOrDiv(op2.clone())) => {
            let mut res: Arc<DAE::Exp>;
            res = Arc::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op1.clone(), exp2: e.clone() });
            Arc::new(DAE::Exp::BINARY { exp1: res.clone(), operator: op2.clone(), exp2: e_3.clone() })
        },
        (Deref @ DAE::Exp::BINARY { exp1: e_1, operator: op2, exp2: e_2 }, op @ DAE::Operator::MUL { ty: _ }, e_3, op1, e, op3, e_6, _, _, _, _, _, _, _, _) if (!(Expression::isConstValue(e_2.clone())?) && ExpressionBasics::expEqual(e_2.clone(), e_6.clone())? && Expression::operatorEqual(op2.clone(), op3.clone())? && Expression::isAddOrSub(op1.clone()) && Expression::isMulOrDiv(op2.clone())) => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut res: Arc<DAE::Exp>;
            e1_1 = Arc::new(DAE::Exp::BINARY { exp1: e_1.clone(), operator: op.clone(), exp2: e_3.clone() });
            res = Arc::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op1.clone(), exp2: e.clone() });
            Arc::new(DAE::Exp::BINARY { exp1: res.clone(), operator: op2.clone(), exp2: e_2.clone() })
        },
        (Deref @ DAE::Exp::BINARY { exp1: e_1, operator: op2, exp2: e_2 }, op @ DAE::Operator::MUL { ty: _ }, e_3, op1, Deref @ DAE::Exp::BINARY { exp1: e_4, operator: op3, exp2: e_5 }, DAE::Operator::MUL { ty: _ }, e_6, _, _, _, _, _, _, _, _) if (!(Expression::isConstValue(e_2.clone())?) && ExpressionBasics::expEqual(e_2.clone(), e_5.clone())? && Expression::operatorEqual(op2.clone(), op3.clone())? && Expression::isAddOrSub(op1.clone()) && Expression::isMulOrDiv(op2.clone())) => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e: Arc<DAE::Exp>;
            let mut res: Arc<DAE::Exp>;
            e1_1 = Arc::new(DAE::Exp::BINARY { exp1: e_1.clone(), operator: op.clone(), exp2: e_3.clone() });
            e = Arc::new(DAE::Exp::BINARY { exp1: e_4.clone(), operator: op.clone(), exp2: e_6.clone() });
            res = Arc::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op1.clone(), exp2: e.clone() });
            Arc::new(DAE::Exp::BINARY { exp1: res.clone(), operator: op2.clone(), exp2: e_2.clone() })
        },
        (e_1, op2, e_3, op1, Deref @ DAE::Exp::BINARY { exp1: e_4, operator: op3, exp2: e_5 }, op @ DAE::Operator::MUL { ty: _ }, e_6, _, _, _, _, _, false, _, _) if (ExpressionBasics::expEqual(e_3.clone(), e_5.clone())? && Expression::operatorEqual(op2.clone(), op3.clone())? && Expression::isAddOrSub(op1.clone()) && Expression::isMulOrDiv(op2.clone())) => {
            let mut e: Arc<DAE::Exp>;
            let mut res: Arc<DAE::Exp>;
            e = Arc::new(DAE::Exp::BINARY { exp1: e_4.clone(), operator: op.clone(), exp2: e_6.clone() });
            res = Arc::new(DAE::Exp::BINARY { exp1: e_1.clone(), operator: op1.clone(), exp2: e.clone() });
            Arc::new(DAE::Exp::BINARY { exp1: res.clone(), operator: op2.clone(), exp2: e_3.clone() })
        },
        (_, DAE::Operator::MUL { .. }, _, DAE::Operator::SUB { .. }, _, DAE::Operator::MUL { .. }, _, true, _, _, _, _, _, _, _) => {
            Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: lhsOperator.clone(), exp2: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: mainOperator.clone(), exp2: e4.clone() }) })
        },
        (_, DAE::Operator::MUL { .. }, _, DAE::Operator::SUB { .. }, _, DAE::Operator::MUL { .. }, _, _, true, _, _, _, _, _, _) => {
            Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: lhsOperator.clone(), exp2: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: mainOperator.clone(), exp2: e3.clone() }) })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

fn simplifyLBinary(mut origExp: Arc<DAE::Exp>, mut inOperator2: Operator, mut inExp3: Arc<DAE::Exp>, mut inExp4: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &((origExp.clone(), inOperator2.clone(), inExp3.clone(), inExp4.clone())) {
        (_, _, Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Nil, .. }, _) => {
            origExp.clone()
        },
        (_, _, _, Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Nil, .. }) => {
            origExp.clone()
        },
        (_, DAE::Operator::AND { ty: Deref @ DAE::Type::T_BOOL { .. } }, e1, Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e2 }) if (ExpressionBasics::expEqual(e1.clone(), e2.clone())?) => {
            Arc::new(DAE::Exp::BCONST { bool: false })
        },
        (_, DAE::Operator::AND { ty: Deref @ DAE::Type::T_BOOL { .. } }, Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e1 }, e2) if (ExpressionBasics::expEqual(e1.clone(), e2.clone())?) => {
            Arc::new(DAE::Exp::BCONST { bool: false })
        },
        (_, DAE::Operator::OR { ty: Deref @ DAE::Type::T_BOOL { .. } }, e1, Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e2 }) if (ExpressionBasics::expEqual(e1.clone(), e2.clone())?) => {
            Arc::new(DAE::Exp::BCONST { bool: true })
        },
        (_, DAE::Operator::OR { ty: Deref @ DAE::Type::T_BOOL { .. } }, Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e1 }, e2) if (ExpressionBasics::expEqual(e1.clone(), e2.clone())?) => {
            Arc::new(DAE::Exp::BCONST { bool: true })
        },
        (_, DAE::Operator::AND { ty: _ }, e1 @ Deref @ DAE::Exp::BCONST { bool: b }, e2) => {
            if (b.clone()) {e2.clone()} else {e1.clone()}
        },
        (_, DAE::Operator::AND { ty: _ }, e1, e2 @ Deref @ DAE::Exp::BCONST { bool: b }) => {
            if (b.clone()) {e1.clone()} else {e2.clone()}
        },
        (_, DAE::Operator::OR { ty: _ }, e1 @ Deref @ DAE::Exp::BCONST { bool: b }, e2) => {
            if (b.clone()) {e1.clone()} else {e2.clone()}
        },
        (_, DAE::Operator::OR { ty: _ }, e1, e2 @ Deref @ DAE::Exp::BCONST { bool: b }) => {
            if (b.clone()) {e2.clone()} else {e1.clone()}
        },
        (_, DAE::Operator::AND { ty: _ }, e1, e2) if (ExpressionBasics::expEqual(e1.clone(), e2.clone())?) => {
            e1.clone()
        },
        (_, DAE::Operator::OR { ty: _ }, e1, e2) if (ExpressionBasics::expEqual(e1.clone(), e2.clone())?) => {
            e1.clone()
        },
        _ => {
            origExp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

fn simplifyRelation(mut origExp: Arc<DAE::Exp>, mut inOperator2: Operator, mut inExp3: Arc<DAE::Exp>, mut inExp4: Arc<DAE::Exp>, mut index: i32, mut optionExpisASUB: Option<(Arc<DAE::Exp>, i32, i32)>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (origExp.clone(), inOperator2.clone(), inExp3.clone(), inExp4.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, oper, e1, e2) => {
                    let mut b: bool = false;
                    let true = (Expression::isConstValue(e1.clone())?) else { bail!("pattern mismatch") };
                    let true = (Expression::isConstValue(e2.clone())?) else { bail!("pattern mismatch") };
                    b = simplifyRelationConst(oper.clone(), e1.clone(), e2.clone())?;
                    Ok(Arc::new(DAE::Exp::BCONST { bool: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::EQUAL { ty: _ }, Deref @ DAE::Exp::CREF { componentRef: cr1, ty: _ }, Deref @ DAE::Exp::CREF { componentRef: cr2, ty: _ }) => {
                    let true = (ComponentReferenceBasics::crefEqual(cr1.clone(), cr2.clone())?) else { bail!("pattern mismatch") };
                    Ok(Arc::new(DAE::Exp::BCONST { bool: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::NEQUAL { ty: _ }, Deref @ DAE::Exp::CREF { componentRef: cr1, ty: _ }, Deref @ DAE::Exp::CREF { componentRef: cr2, ty: _ }) => {
                    let true = (ComponentReferenceBasics::crefEqual(cr1.clone(), cr2.clone())?) else { bail!("pattern mismatch") };
                    Ok(Arc::new(DAE::Exp::BCONST { bool: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::GREATEREQ { .. }, _, _) => {
                    Ok(simplifyRelation2(origExp.clone(), inOperator2.clone(), inExp3.clone(), inExp4.clone(), index.clone(), optionExpisASUB.clone(), (std::sync::Arc::new(Expression::isPositiveOrZero) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::GREATER { .. }, _, _) => {
                    Ok(simplifyRelation2(origExp.clone(), inOperator2.clone(), inExp3.clone(), inExp4.clone(), index.clone(), optionExpisASUB.clone(), (std::sync::Arc::new(Expression::isPositiveOrZero) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::LESSEQ { .. }, _, _) => {
                    Ok(simplifyRelation2(origExp.clone(), inOperator2.clone(), inExp4.clone(), inExp3.clone(), index.clone(), optionExpisASUB.clone(), (std::sync::Arc::new(Expression::isPositiveOrZero) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::LESS { .. }, _, _) => {
                    Ok(simplifyRelation2(origExp.clone(), inOperator2.clone(), inExp4.clone(), inExp3.clone(), index.clone(), optionExpisASUB.clone(), (std::sync::Arc::new(Expression::isPositiveOrZero) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(origExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

fn simplifyRelation2(mut origExp: Arc<DAE::Exp>, mut inOp: Operator, mut lhs: Arc<DAE::Exp>, mut rhs: Arc<DAE::Exp>, mut index: i32, mut optionExpisASUB: Option<(Arc<DAE::Exp>, i32, i32)>, mut isPositive: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>) -> Result<Arc<DAE::Exp>> {
    pub type Fun = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>;

    let mut oExp: Arc<DAE::Exp>;
    let mut b: bool = false;
    let mut tp: Type = Arc::new(DAE::Type::T_NORETCALL);
    oExp = Expression::expSub(lhs.clone(), rhs.clone())?;
    (oExp, b) = simplify(oExp.clone())?;
    if Expression::isGreatereqOrLesseq(inOp.clone()) && isPositive(oExp.clone())? {
        oExp = Arc::new(DAE::Exp::BCONST { bool: true });
    } else {
        if Expression::isGreatereqOrLesseq(inOp.clone()) {
            oExp = origExp.clone();
        } else {
            oExp = Expression::negate(oExp.clone())?;
            (oExp, _) = simplify(oExp.clone())?;
            oExp = if (isPositive(oExp.clone())?) {Arc::new(DAE::Exp::BCONST { bool: false })} else {origExp.clone()};
        }
    }
    Ok(oExp)
}

fn simplifyBinaryDistributePow(mut inExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inExp: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    outExpLst = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut e in (inExpLst.clone()).into_iter().cloned() {
            if !(!(Expression::isConstOne(e.clone()))) { continue; }
            let __x = Arc::new(DAE::Exp::BINARY { exp1: e.clone(), operator: DAE::Operator::POW { ty: Expression::r#typeof(e.clone())? }, exp2: inExp.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(outExpLst)
}

fn simplifyUnary(mut origExp: Arc<DAE::Exp>, mut inOperator2: Operator, mut inExp3: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (origExp.clone(), inOperator2.clone(), inExp3.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::NOT { .. }, e1) => {
                    let mut b1: bool = false;
                    b1 = Expression::toBool(e1.clone())?;
                    b1 = !(b1.clone());
                    Ok(Arc::new(DAE::Exp::BCONST { bool: b1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::NOT { ty: _ }, Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e1 }) => {
                    Ok(e1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::UMINUS { .. }, Deref @ DAE::Exp::ICONST { integer: i }) => {
                    let mut i_1: i32 = 0;
                    i_1 = intNeg(i.clone());
                    Ok(Arc::new(DAE::Exp::ICONST { integer: i_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::UMINUS { .. }, Deref @ DAE::Exp::RCONST { real: r }) => {
                    let mut r_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    r_1 = -(r.clone());
                    Ok(Arc::new(DAE::Exp::RCONST { real: r_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, op2 @ DAE::Operator::UMINUS { .. }, Deref @ DAE::Exp::BINARY { exp2: e2, operator: op @ DAE::Operator::MUL { .. }, exp1: e1 }) => {
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::UNARY { operator: op2.clone(), exp: e1.clone() }), operator: op.clone(), exp2: e2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, op2 @ DAE::Operator::UMINUS_ARR { .. }, Deref @ DAE::Exp::BINARY { exp2: e2, operator: op @ DAE::Operator::MUL_ARR { .. }, exp1: e1 }) => {
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::UNARY { operator: op2.clone(), exp: e1.clone() }), operator: op.clone(), exp2: e2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::UMINUS { .. }, e1) => {
                    if !((Expression::isZero(e1.clone()))) { bail!("guard") }
                    Ok(e1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::UMINUS_ARR { .. }, e1) => {
                    if !((Expression::isZero(e1.clone()))) { bail!("guard") }
                    Ok(e1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::UMINUS { .. }, Deref @ DAE::Exp::BINARY { exp2: e2, operator: op @ DAE::Operator::SUB { .. }, exp1: e1 }) => {
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op.clone(), exp2: e1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::UMINUS_ARR { .. }, Deref @ DAE::Exp::BINARY { exp2: e2, operator: op @ DAE::Operator::SUB_ARR { .. }, exp1: e1 }) => {
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op.clone(), exp2: e1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, op2 @ DAE::Operator::UMINUS { .. }, Deref @ DAE::Exp::BINARY { exp2: e2, operator: op @ DAE::Operator::ADD { .. }, exp1: e1 }) => {
                    let mut e_1: Arc<DAE::Exp>;
                    let __pa0 = ::match_deref::match_deref! { match &(simplify1(Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::UNARY { operator: op2.clone(), exp: e1.clone() }), operator: op.clone(), exp2: Arc::new(DAE::Exp::UNARY { operator: op2.clone(), exp: e2.clone() }) }))?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e_1 = __pa0.clone();
                    Ok(e_1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, op2 @ DAE::Operator::UMINUS_ARR { .. }, Deref @ DAE::Exp::BINARY { exp2: e2, operator: op @ DAE::Operator::ADD_ARR { .. }, exp1: e1 }) => {
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::UNARY { operator: op2.clone(), exp: e1.clone() }), operator: op.clone(), exp2: Arc::new(DAE::Exp::UNARY { operator: op2.clone(), exp: e2.clone() }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, op2 @ DAE::Operator::UMINUS { .. }, Deref @ DAE::Exp::BINARY { exp2: e2, operator: op @ DAE::Operator::DIV { .. }, exp1: e1 }) => {
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::UNARY { operator: op2.clone(), exp: e1.clone() }), operator: op.clone(), exp2: e2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, op2 @ DAE::Operator::UMINUS_ARR { .. }, Deref @ DAE::Exp::BINARY { exp2: e2, operator: op @ DAE::Operator::DIV_ARR { .. }, exp1: e1 }) => {
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::UNARY { operator: op2.clone(), exp: e1.clone() }), operator: op.clone(), exp2: e2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::UMINUS { .. }, Deref @ DAE::Exp::UNARY { exp: e1, operator: DAE::Operator::UMINUS { .. } }) => {
                    Ok(e1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::UMINUS_ARR { .. }, Deref @ DAE::Exp::UNARY { exp: e1, operator: DAE::Operator::UMINUS_ARR { .. } }) => {
                    Ok(e1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::UMINUS { .. }, Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::UNARY { exp: e1, .. }, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Cons { head: e3, tail: Deref @ metamodelica::List::Nil } } }, path: path @ Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" } }) => {
                    Ok(Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: list![e1.clone(), e3.clone(), e2.clone()], attr: attr.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::UMINUS_ARR { .. }, Deref @ DAE::Exp::ARRAY { ty: ty1, scalar: b1, array: expl }) => {
                    let mut expl = (*expl).clone();
                    expl = List::map(expl.clone(), (std::sync::Arc::new(Expression::negate) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>));
                    Ok(Arc::new(DAE::Exp::ARRAY { ty: ty1.clone(), scalar: b1.clone(), array: expl.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Operator::UMINUS_ARR { .. }, Deref @ DAE::Exp::MATRIX { ty: ty1, integer: i, matrix: mat }) => {
                    let mut mat = (*mat).clone();
                    mat = List::mapList(mat.clone(), (std::sync::Arc::new(Expression::negate) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>));
                    Ok(Arc::new(DAE::Exp::MATRIX { ty: ty1.clone(), integer: i.clone(), matrix: mat.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(origExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

fn simplifyVectorScalarMatrix(mut imexpl: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, mut op: Operator, mut s1: Arc<DAE::Exp>, mut arrayScalar: bool) -> Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> {
    let mut outExp: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
    outExp = if (arrayScalar.clone()) {{
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
        for mut row in (imexpl.clone()).into_iter().cloned() {
            let __x = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut e in (row.clone()).into_iter().cloned() {
            let __x = Arc::new(DAE::Exp::BINARY { exp1: e.clone(), operator: op.clone(), exp2: s1.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }} else {{
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
        for mut row in (imexpl.clone()).into_iter().cloned() {
            let __x = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut e in (row.clone()).into_iter().cloned() {
            let __x = Arc::new(DAE::Exp::BINARY { exp1: s1.clone(), operator: op.clone(), exp2: e.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }};
    outExp
}

fn simplifyBinarySortConstantsMul(mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let mut e_lst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut e_lst_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut const_es1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut const_es1_1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut notconst_es1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut res1: Arc<DAE::Exp>;
    let mut res2: Arc<DAE::Exp>;
    e_lst = Expression::factors(inExp.clone())?;
    (const_es1, notconst_es1) = List::splitOnTrue(e_lst.clone(), (std::sync::Arc::new(Expression::isConst) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>));
    if !(const_es1.clone().is_empty()) {
        res1 = simplifyBinaryMulConstants(const_es1.clone())?;
        (res1, _) = simplify1(res1.clone())?;
        res2 = Expression::makeProductLst(notconst_es1.clone())?;
        outExp = Expression::expMul(res1.clone(), res2.clone())?;
    } else {
        outExp = inExp.clone();
    }
    Ok(outExp)
}

fn simplifyBuiltinConstantDer(mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::RCONST { real: _ } => {
            Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) })
        },
        Deref @ DAE::Exp::ICONST { integer: _ } => {
            Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) })
        },
        Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { dims, ty: Deref @ DAE::Type::T_REAL { .. } }, .. } => {
            let mut e: Arc<DAE::Exp>;
            (e, _) = Expression::makeZeroExpression(dims.clone())?;
            e.clone()
        },
        Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { dims, ty: Deref @ DAE::Type::T_INTEGER { .. } }, .. } => {
            let mut e: Arc<DAE::Exp>;
            (e, _) = Expression::makeZeroExpression(dims.clone())?;
            e.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

fn removeOperatorDimension(mut inop: Operator) -> Result<Operator> {
    let mut outop: Operator;
    outop = (match inop.clone() {
        DAE::Operator::ADD_ARR { ty: ref ty1 } => {
            let mut ty2: Type = Arc::new(DAE::Type::T_NORETCALL);
            let mut b: bool = false;
            let mut op: Operator;
            ty2 = Expression::unliftArray(ty1.clone())?;
            b = DAEUtil::expTypeArray(ty2.clone());
            op = if (b.clone()) {DAE::Operator::ADD_ARR { ty: ty2.clone() }} else {DAE::Operator::ADD { ty: ty2.clone() }};
            op.clone()
        },
        DAE::Operator::SUB_ARR { ty: ref ty1 } => {
            let mut ty2: Type = Arc::new(DAE::Type::T_NORETCALL);
            let mut b: bool = false;
            let mut op: Operator;
            ty2 = Expression::unliftArray(ty1.clone())?;
            b = DAEUtil::expTypeArray(ty2.clone());
            op = if (b.clone()) {DAE::Operator::SUB_ARR { ty: ty2.clone() }} else {DAE::Operator::SUB { ty: ty2.clone() }};
            op.clone()
        },
        DAE::Operator::DIV_ARR { ty: ref ty1 } => {
            let mut ty2: Type = Arc::new(DAE::Type::T_NORETCALL);
            let mut b: bool = false;
            let mut op: Operator;
            ty2 = Expression::unliftArray(ty1.clone())?;
            b = DAEUtil::expTypeArray(ty2.clone());
            op = if (b.clone()) {DAE::Operator::DIV_ARR { ty: ty2.clone() }} else {DAE::Operator::DIV { ty: ty2.clone() }};
            op.clone()
        },
        DAE::Operator::MUL_ARR { ty: ref ty1 } => {
            let mut ty2: Type = Arc::new(DAE::Type::T_NORETCALL);
            let mut b: bool = false;
            let mut op: Operator;
            ty2 = Expression::unliftArray(ty1.clone())?;
            b = DAEUtil::expTypeArray(ty2.clone());
            op = if (b.clone()) {DAE::Operator::MUL_ARR { ty: ty2.clone() }} else {DAE::Operator::MUL { ty: ty2.clone() }};
            op.clone()
        },
        DAE::Operator::POW_ARR2 { ty: ref ty1 } => {
            let mut ty2: Type = Arc::new(DAE::Type::T_NORETCALL);
            let mut b: bool = false;
            let mut op: Operator;
            ty2 = Expression::unliftArray(ty1.clone())?;
            b = DAEUtil::expTypeArray(ty2.clone());
            op = if (b.clone()) {DAE::Operator::POW_ARR2 { ty: ty2.clone() }} else {DAE::Operator::POW { ty: ty2.clone() }};
            op.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outop)
}

pub fn simplifyRangeBool(mut inStart: bool, mut inStop: bool) -> Arc<metamodelica::List<bool>> {
    let mut outRange: Arc<metamodelica::List<bool>> = metamodelica::nil();
    outRange = if (inStart.clone()) {if (inStop.clone()) {list![true]} else {metamodelica::nil()}} else {if (inStop.clone()) {list![false, true]} else {list![false]}};
    outRange
}

pub fn simplifyRange(mut inStart: i32, mut inStep: i32, mut inStop: i32) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outValues: Arc<metamodelica::List<i32>> = metamodelica::nil();
    outValues = List::intRange3(inStart.clone(), inStep.clone(), inStop.clone())?;
    Ok(outValues)
}

pub fn simplifyRangeReal(mut inStart: metamodelica::Real, mut inStep: metamodelica::Real, mut inStop: metamodelica::Real) -> Result<Arc<metamodelica::List<metamodelica::Real>>> {
    let mut outValues: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    outValues = 'mc: {
        let __mc_input = (inStart.clone(), inStep.clone(), inStop.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut error_str: ArcStr = arcstr::literal!("");
            let true = (realAbs(inStep.clone()) <= metamodelica::OrderedFloat(1e-14_f64)) else { bail!("pattern mismatch") };
            error_str = stringDelimitList(List::map(list![inStart.clone(), inStep.clone(), inStop.clone()], (std::sync::Arc::new(fnptr!(realString, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real) -> Result<ArcStr> + 'static>)), (literal!(":")).clone());
            Error::addMessage(Error::ZERO_STEP_IN_ARRAY_CONSTRUCTOR.clone(), list![(error_str.clone()).clone()])?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _) = __mc_input.clone() else { bail!("nomatch") };
            if !((inStart.clone() == inStop.clone())) { bail!("guard") }
            Ok(list![inStart.clone()])
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut steps: i32 = 0;
            steps = Util::realRangeSize(inStart.clone(), inStep.clone(), inStop.clone()) - 1;
            Ok(simplifyRangeReal2(inStart.clone(), inStep.clone(), steps.clone(), metamodelica::nil()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outValues)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn simplifyRangeReal2(mut inStart: metamodelica::Real, mut inStep: metamodelica::Real, mut inSteps: i32, mut inValues: Arc<metamodelica::List<metamodelica::Real>>) -> Arc<metamodelica::List<metamodelica::Real>> {
    let mut outValues: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    outValues = (::match_deref::match_deref! { match &((inStart.clone(), inStep.clone(), inSteps.clone(), inValues.clone())) {
        (_, _, (-1), _) => {
            inValues.clone()
        },
        _ => {
            let mut next: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut vals: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            next = inStart.clone() + inStep.clone() * intReal(inSteps.clone());
            vals = cons(next.clone(), inValues.clone());
            simplifyRangeReal2(inStart.clone(), inStep.clone(), inSteps.clone() - 1, vals.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outValues
}

fn simplifyReduction(mut inReduction: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outValue: Arc<DAE::Exp>;
    outValue = 'mc: {
        let __mc_input = inReduction.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::REDUCTION { reductionInfo: Deref @ DAE::ReductionInfo { defaultValue: Some(v), .. }, iterators, .. } => {
                    let mut expr: Arc<DAE::Exp>;
                    let true = (hasZeroLengthIterator(iterators.clone())) else { bail!("pattern mismatch") };
                    expr = ValuesUtil::valueExp(v.clone(), None)?;
                    Ok(expr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::REDUCTION { iterators, .. } => {
                    let mut expr: Arc<DAE::Exp>;
                    let true = (hasZeroLengthIterator(iterators.clone())) else { bail!("pattern mismatch") };
                    expr = ValuesUtil::valueExp(Arc::new(openmodelica_frontend_types::Values::Value::META_FAIL), None)?;
                    Ok(expr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::REDUCTION { iterators: Deref @ metamodelica::List::Cons { head: Deref @ DAE::ReductionIterator { exp: range, guardExp: None, id: iter_name, .. }, tail: Deref @ metamodelica::List::Nil }, expr, reductionInfo: Deref @ DAE::ReductionInfo { defaultValue, exprType: ty, foldExp, resultName, foldName, path, .. } } => {
                    let mut values: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut ety: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut expr = (*expr).clone();
                    values = Expression::getArrayOrRangeContents(range.clone())?;
                    ety = Types::simplifyType(ty.clone())?;
                    values = List::map2(values.clone(), (std::sync::Arc::new(replaceIteratorWithExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>, ArcStr) -> Result<Arc<DAE::Exp>> + 'static>), expr.clone(), (iter_name.clone()).clone());
                    expr = simplifyReductionFoldPhase(path.clone(), foldExp.clone(), (foldName.clone()).clone(), (resultName.clone()).clone(), ety.clone(), values.clone(), defaultValue.clone())?;
                    Ok(expr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::REDUCTION { iterators, expr, reductionInfo: Deref @ DAE::ReductionInfo { defaultValue, foldExp, exprType: ty, resultName, foldName, iterType: Absyn::ReductionIterType::THREAD, path } } => {
                    let mut range: Arc<DAE::Exp>;
                    let mut iter_name: ArcStr = arcstr::literal!("");
                    let mut values: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut ety: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut iterators = (*iterators).clone();
                    let mut expr = (*expr).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(iterators.clone()) {
                        Deref @ metamodelica::List::Cons { head: Deref @ DAE::ReductionIterator { exp: __pa0, guardExp: None, id: __pa1, .. }, tail: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    range = __pa0.clone();
                    iter_name = __pa1.clone();
                    iterators = __pa2.clone();
                    values = Expression::getArrayOrRangeContents(range.clone())?;
                    ety = Types::simplifyType(ty.clone())?;
                    values = List::map2(values.clone(), (std::sync::Arc::new(replaceIteratorWithExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>, ArcStr) -> Result<Arc<DAE::Exp>> + 'static>), expr.clone(), (iter_name.clone()).clone());
                    values = List::fold(iterators.clone(), (std::sync::Arc::new(getIteratorValues) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ReductionIterator>, Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>), values.clone());
                    expr = simplifyReductionFoldPhase(path.clone(), foldExp.clone(), (foldName.clone()).clone(), (resultName.clone()).clone(), ety.clone(), values.clone(), defaultValue.clone())?;
                    Ok(expr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::REDUCTION { iterators: Deref @ metamodelica::List::Cons { head: iter, tail: iterators @ Deref @ metamodelica::List::Cons { head: _, tail: _ } }, expr, reductionInfo: Deref @ DAE::ReductionInfo { exprType: ty, resultName, foldName, iterType: Absyn::ReductionIterType::COMBINE, path: path @ Deref @ Absyn::Path::IDENT { name: Deref @ "array" }, .. } } => {
                    let mut ty1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut foldName2: ArcStr = arcstr::literal!("");
                    let mut resultName2: ArcStr = arcstr::literal!("");
                    let mut expr = (*expr).clone();
                    foldName2 = (Util::getTempVariableIndex()).clone();
                    resultName2 = (Util::getTempVariableIndex()).clone();
                    ty1 = Expression::unliftArray(ty.clone())?;
                    expr = Arc::new(DAE::Exp::REDUCTION { reductionInfo: Arc::new(DAE::ReductionInfo { path: path.clone(), iterType: openmodelica_ast::Absyn::ReductionIterType::COMBINE, exprType: ty1.clone(), defaultValue: None, foldName: (foldName2.clone()).clone(), resultName: (resultName2.clone()).clone(), foldExp: None }), expr: expr.clone(), iterators: list![iter.clone()] });
                    expr = Arc::new(DAE::Exp::REDUCTION { reductionInfo: Arc::new(DAE::ReductionInfo { path: path.clone(), iterType: openmodelica_ast::Absyn::ReductionIterType::COMBINE, exprType: ty.clone(), defaultValue: None, foldName: (foldName.clone()).clone(), resultName: (resultName.clone()).clone(), foldExp: None }), expr: expr.clone(), iterators: iterators.clone() });
                    Ok(expr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::REDUCTION { iterators: Deref @ metamodelica::List::Cons { head: iter, tail: iterators @ Deref @ metamodelica::List::Cons { head: _, tail: _ } }, expr, reductionInfo: Deref @ DAE::ReductionInfo { defaultValue, foldExp: None, exprType: ty, resultName, foldName, iterType: Absyn::ReductionIterType::COMBINE, path } } => {
                    let mut foldName2: ArcStr = arcstr::literal!("");
                    let mut resultName2: ArcStr = arcstr::literal!("");
                    let mut expr = (*expr).clone();
                    foldName2 = (Util::getTempVariableIndex()).clone();
                    resultName2 = (Util::getTempVariableIndex()).clone();
                    expr = Arc::new(DAE::Exp::REDUCTION { reductionInfo: Arc::new(DAE::ReductionInfo { path: path.clone(), iterType: openmodelica_ast::Absyn::ReductionIterType::COMBINE, exprType: ty.clone(), defaultValue: defaultValue.clone(), foldName: (foldName2.clone()).clone(), resultName: (resultName2.clone()).clone(), foldExp: None }), expr: expr.clone(), iterators: list![iter.clone()] });
                    expr = Arc::new(DAE::Exp::REDUCTION { reductionInfo: Arc::new(DAE::ReductionInfo { path: path.clone(), iterType: openmodelica_ast::Absyn::ReductionIterType::COMBINE, exprType: ty.clone(), defaultValue: defaultValue.clone(), foldName: (foldName.clone()).clone(), resultName: (resultName.clone()).clone(), foldExp: None }), expr: expr.clone(), iterators: iterators.clone() });
                    Ok(expr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::REDUCTION { iterators: Deref @ metamodelica::List::Cons { head: iter, tail: iterators @ Deref @ metamodelica::List::Cons { head: _, tail: _ } }, expr, reductionInfo: Deref @ DAE::ReductionInfo { defaultValue, foldExp: Some(foldExpr), exprType: ty, resultName, foldName, iterType: Absyn::ReductionIterType::COMBINE, path } } => {
                    let mut foldExpr2: Arc<DAE::Exp>;
                    let mut foldName2: ArcStr = arcstr::literal!("");
                    let mut resultName2: ArcStr = arcstr::literal!("");
                    let mut expr = (*expr).clone();
                    foldName2 = (Util::getTempVariableIndex()).clone();
                    resultName2 = (Util::getTempVariableIndex()).clone();
                    (foldExpr2, _) = Expression::traverseExpBottomUp(foldExpr.clone(), (std::sync::Arc::new(fnptr!(Expression::renameExpCrefIdent, Arc<DAE::Exp>, (ArcStr, ArcStr))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (ArcStr, ArcStr)) -> Result<(Arc<DAE::Exp>, (ArcStr, ArcStr))> + 'static>), (foldName.clone(), foldName2.clone()))?;
                    (foldExpr2, _) = Expression::traverseExpBottomUp(foldExpr2.clone(), (std::sync::Arc::new(fnptr!(Expression::renameExpCrefIdent, Arc<DAE::Exp>, (ArcStr, ArcStr))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (ArcStr, ArcStr)) -> Result<(Arc<DAE::Exp>, (ArcStr, ArcStr))> + 'static>), (resultName.clone(), resultName2.clone()))?;
                    expr = Arc::new(DAE::Exp::REDUCTION { reductionInfo: Arc::new(DAE::ReductionInfo { path: path.clone(), iterType: openmodelica_ast::Absyn::ReductionIterType::COMBINE, exprType: ty.clone(), defaultValue: defaultValue.clone(), foldName: (foldName2.clone()).clone(), resultName: (resultName2.clone()).clone(), foldExp: Some(foldExpr2.clone()) }), expr: expr.clone(), iterators: list![iter.clone()] });
                    expr = Arc::new(DAE::Exp::REDUCTION { reductionInfo: Arc::new(DAE::ReductionInfo { path: path.clone(), iterType: openmodelica_ast::Absyn::ReductionIterType::COMBINE, exprType: ty.clone(), defaultValue: defaultValue.clone(), foldName: (foldName.clone()).clone(), resultName: (resultName.clone()).clone(), foldExp: Some(foldExpr.clone()) }), expr: expr.clone(), iterators: iterators.clone() });
                    Ok(expr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inReduction.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outValue)
}

fn getIteratorValues(mut iter: Arc<DAE::ReductionIterator>, mut inValues: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut values: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut iter_name: ArcStr = arcstr::literal!("");
    let mut range: Arc<DAE::Exp>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(iter.clone()) {
        Deref @ DAE::ReductionIterator { exp: __pa0, guardExp: None, id: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    range = __pa0.clone();
    iter_name = __pa1.clone();
    values = Expression::getArrayOrRangeContents(range.clone())?;
    values = List::threadMap1(values.clone(), inValues.clone(), (std::sync::Arc::new(replaceIteratorWithExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>, ArcStr) -> Result<Arc<DAE::Exp>> + 'static>), (iter_name.clone()).clone());
    Ok(values)
}

fn replaceIteratorWithExp(mut iterExp: Arc<DAE::Exp>, mut exp: Arc<DAE::Exp>, mut name: ArcStr) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let __pa0 = ::match_deref::match_deref! { match &(Expression::traverseExpBottomUp(exp.clone(), (std::sync::Arc::new(replaceIteratorWithExpTraverser) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (ArcStr, Arc<DAE::Exp>, bool)) -> Result<(Arc<DAE::Exp>, (ArcStr, Arc<DAE::Exp>, bool))> + 'static>), (name.clone(), iterExp.clone(), true))?) {
        (__pa0, (_, _, true)) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outExp = __pa0.clone();
    Ok(outExp)
}

fn replaceIteratorWithExpTraverser(mut inExp: Arc<DAE::Exp>, mut inTpl: (ArcStr, Arc<DAE::Exp>, bool)) -> Result<(Arc<DAE::Exp>, (ArcStr, Arc<DAE::Exp>, bool))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (ArcStr, Arc<DAE::Exp>, bool);
    (outExp, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (_, _, false)) => {
                    Ok((inExp.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: _, subscriptLst: Deref @ metamodelica::List::Nil }, ty: _ }, tpl @ (name, iterExp, _)) => {
                    if !((stringEq((name.clone()).clone(), (id.clone()).clone()))) { bail!("guard") }
                    Ok((iterExp.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, .. }, .. }, (name, iterExp, _)) => {
                    if !((stringEq((name.clone()).clone(), (id.clone()).clone()))) { bail!("guard") }
                    Ok((exp.clone(), (name.clone(), iterExp.clone(), false)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: ty1, subscriptLst: ss, componentRef: cr }, ty }, tpl @ (name, Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::List::Nil, ident: replName, .. }, .. }, _)) => {
                    if !((stringEq((name.clone()).clone(), (id.clone()).clone()))) { bail!("guard") }
                    Ok((Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (replName.clone()).clone(), identType: ty1.clone(), subscriptLst: ss.clone(), componentRef: cr.clone() }), ty: ty.clone() }), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: ty1, subscriptLst: Deref @ metamodelica::List::Nil, componentRef: cr }, ty }, tpl @ (name, Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: ss, ident: replName, .. }, .. }, _)) => {
                    if !((stringEq((name.clone()).clone(), (id.clone()).clone()))) { bail!("guard") }
                    Ok((Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (replName.clone()).clone(), identType: ty1.clone(), subscriptLst: ss.clone(), componentRef: cr.clone() }), ty: ty.clone() }), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: _, subscriptLst: Deref @ metamodelica::List::Nil, componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: id2, identType: _, subscriptLst: Deref @ metamodelica::List::Nil } }, .. }, tpl @ (name, Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: recordPath }, varLst, .. }, .. }, path: callPath, expLst: exps }, _)) => {
                    let mut exp: Arc<DAE::Exp>;
                    let mut i: i32 = 0;
                    let true = (stringEq((name.clone()).clone(), (id.clone()).clone())) else { bail!("pattern mismatch") };
                    let true = (AbsynUtil::pathEqual(callPath.clone(), recordPath.clone())) else { bail!("pattern mismatch") };
                    let true = ((varLst.clone().len() as i32) == (exps.clone().len() as i32)) else { bail!("pattern mismatch") };
                    i = List::position1OnTrue(varLst.clone(), (std::sync::Arc::new(DAEUtil::typeVarIdentEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>, ArcStr) -> Result<bool> + 'static>), (id2.clone()).clone());
                    exp = (exps.clone()).get(i.clone())?;
                    Ok((exp.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, .. }, .. }, (name, iterExp, _)) => {
                    if !((stringEq((name.clone()).clone(), (id.clone()).clone()))) { bail!("guard") }
                    Ok((exp.clone(), (name.clone(), iterExp.clone(), false)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outTpl))
}

fn simplifyReductionFoldPhase(mut path: Arc<Absyn::Path>, mut optFoldExp: Option<Arc<DAE::Exp>>, mut foldName: ArcStr, mut resultName: ArcStr, mut ty: Arc<DAE::Type>, mut inExps: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut defaultValue: Option<Arc<Values::Value>>) -> Result<Arc<DAE::Exp>> {
    let mut exp: Arc<DAE::Exp>;
    let mut checkForSimplifications: bool = false;
    (exp, checkForSimplifications) = (::match_deref::match_deref! { match &((path.clone(), optFoldExp.clone(), foldName.clone(), resultName.clone(), ty.clone(), inExps.clone(), defaultValue.clone())) {
        (Deref @ Absyn::Path::IDENT { name: Deref @ "array" }, _, _, _, _, _, _) => {
            let mut aty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut ty2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut length: i32 = 0;
            aty = Types::unliftArray(Types::expTypetoTypesType(ty.clone())?)?;
            length = (inExps.clone().len() as i32);
            ty2 = Types::liftArray(aty.clone(), Arc::new(DAE::Dimension::DIM_INTEGER { integer: length.clone() }));
            exp = Expression::makeArray(inExps.clone(), ty2.clone(), !(Types::isArray(aty.clone())));
            (exp.clone(), false)
        },
        (_, _, _, _, _, Deref @ metamodelica::List::Nil, Some(val)) => {
            (ValuesUtil::valueExp(val.clone(), None)?, false)
        },
        (_, _, _, _, _, Deref @ metamodelica::List::Nil, None) => {
            bail!("fail")
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "min" }, _, _, _, _, _, _) => {
            let mut arr_exp: Arc<DAE::Exp>;
            arr_exp = Expression::makeScalarArray(inExps.clone(), ty.clone());
            (Expression::makePureBuiltinCall((literal!("min")).clone(), list![arr_exp.clone()], ty.clone()), true)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "max" }, _, _, _, _, _, _) => {
            let mut arr_exp: Arc<DAE::Exp>;
            arr_exp = Expression::makeScalarArray(inExps.clone(), ty.clone());
            (Expression::makePureBuiltinCall((literal!("max")).clone(), list![arr_exp.clone()], ty.clone()), true)
        },
        (_, Some(_), _, _, _, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, _) => {
            (exp.clone(), false)
        },
        (_, Some(foldExp), _, _, _, Deref @ metamodelica::List::Cons { head: exp, tail: exps }, _) => {
            let mut exp = (*exp).clone();
            exp = simplifyReductionFoldPhase2(exps.clone(), foldExp.clone(), (foldName.clone()).clone(), (resultName.clone()).clone(), exp.clone())?;
            (exp.clone(), false)
        },
        _ => bail!("match: no arm matched"),
    } });
    if checkForSimplifications.clone() {
        let __pa0 = ::match_deref::match_deref! { match &(simplify1(exp.clone())?) {
            (__pa0, true) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        exp = __pa0.clone();
    }
    Ok(exp)
}

fn simplifyReductionFoldPhase2(mut inExps: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut foldExp: Arc<DAE::Exp>, mut foldName: ArcStr, mut resultName: ArcStr, mut acc: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut exp: Arc<DAE::Exp>;
    exp = (::match_deref::match_deref! { match &((inExps.clone(), foldExp.clone(), foldName.clone(), resultName.clone(), acc.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _) => {
            acc.clone()
        },
        (Deref @ metamodelica::List::Cons { head: exp, tail: exps }, _, _, _, _) => {
            let mut exp = (*exp).clone();
            exp = replaceIteratorWithExp(exp.clone(), foldExp.clone(), (foldName.clone()).clone())?;
            exp = replaceIteratorWithExp(acc.clone(), exp.clone(), (resultName.clone()).clone())?;
            simplifyReductionFoldPhase2(exps.clone(), foldExp.clone(), (foldName.clone()).clone(), (resultName.clone()).clone(), exp.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(exp)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn hasZeroLengthIterator(mut inIters: Arc<metamodelica::List<Arc<DAE::ReductionIterator>>>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(inIters.clone()) {
        Deref @ metamodelica::List::Nil => {
            false
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::ReductionIterator { guardExp: Some(Deref @ DAE::Exp::BCONST { bool: false }), .. }, tail: _ } => {
            true
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::ReductionIterator { exp: Deref @ DAE::Exp::LIST { valList: Deref @ metamodelica::List::Nil }, .. }, tail: _ } => {
            true
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::ReductionIterator { exp: Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Nil, .. }, .. }, tail: _ } => {
            true
        },
        Deref @ metamodelica::List::Cons { head: _, tail: iters } => {
            hasZeroLengthIterator(iters.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn simplifyList(mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExpl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    outExpl = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut exp in (expl.clone()).into_iter().cloned() {
            let __x = (simplify1(exp.clone())?).0;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(outExpl)
}

pub fn simplifyList1(mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<bool>>)> {
    let mut outExpl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut outBool: Arc<metamodelica::List<bool>> = metamodelica::nil();
    outExpl = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut exp in (expl.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(exp.clone()) {
        _ => {
            let mut e: Arc<DAE::Exp>;
            let mut b2: bool = false;
            (e, b2) = simplify(exp.clone())?;
            outBool = cons(b2.clone(), outBool.clone());
            e.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outBool = Dangerous::listReverseInPlace(outBool.clone());
    Ok((outExpl, outBool))
}

pub fn condsimplifyList1(mut blst: Arc<metamodelica::List<bool>>, mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<bool>>)> {
    let mut outExpl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut outBool: Arc<metamodelica::List<bool>> = metamodelica::nil();
    let mut rest_expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = expl.clone();
    let mut exp: Arc<DAE::Exp>;
    let mut b2: bool = false;
    for mut b in &*blst.clone() {
        let mut b = b.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_expl.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        exp = __pa0.clone();
        rest_expl = __pa1.clone();
        (exp, b2) = condsimplify(b.clone(), exp.clone())?;
        outExpl = cons(exp.clone(), outExpl.clone());
        outBool = cons(b2.clone(), outBool.clone());
    }
    outExpl = Dangerous::listReverseInPlace(outExpl.clone());
    outBool = Dangerous::listReverseInPlace(outBool.clone());
    Ok((outExpl, outBool))
}

fn checkZeroLengthArrayOp(mut op: DAE::Operator) -> Result<()> {
    let _ = (match op.clone() {
        DAE::Operator::ADD_ARR { .. } => (),
        DAE::Operator::SUB_ARR { .. } => (),
        DAE::Operator::MUL_ARR { .. } => (),
        DAE::Operator::DIV_ARR { .. } => (),
        DAE::Operator::POW_ARR { .. } => (),
        DAE::Operator::POW_ARR2 { .. } => (),
        DAE::Operator::MUL_ARRAY_SCALAR { .. } => (),
        DAE::Operator::ADD_ARRAY_SCALAR { .. } => (),
        DAE::Operator::DIV_ARRAY_SCALAR { .. } => (),
        DAE::Operator::SUB_SCALAR_ARRAY { .. } => (),
        DAE::Operator::DIV_SCALAR_ARRAY { .. } => (),
        DAE::Operator::MUL_MATRIX_PRODUCT { .. } => (),
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

pub fn simplifyAddSymbolicOperation(mut exp: Arc<DAE::EquationExp>, mut source: Arc<DAE::ElementSource>) -> Result<(Arc<DAE::EquationExp>, Arc<DAE::ElementSource>)> {
    let mut outExp: Arc<DAE::EquationExp>;
    let mut outSource: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    (outExp, outSource) = (::match_deref::match_deref! { match &((exp.clone(), source.clone())) {
        (Deref @ DAE::EquationExp::PARTIAL_EQUATION { exp: e }, _) => {
            let mut changed: bool = false;
            let mut e = (*e).clone();
            (e, changed) = simplify(e.clone())?;
            outExp = if (changed.clone()) {Arc::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e.clone() })} else {exp.clone()};
            outSource = ElementSource::condAddSymbolicTransformation(changed.clone(), source.clone(), Arc::new(DAE::SymbolicOperation::SIMPLIFY { before: exp.clone(), after: outExp.clone() }))?;
            (outExp.clone(), outSource.clone())
        },
        (Deref @ DAE::EquationExp::RESIDUAL_EXP { exp: e }, _) => {
            let mut changed: bool = false;
            let mut e = (*e).clone();
            (e, changed) = simplify(e.clone())?;
            outExp = if (changed.clone()) {Arc::new(DAE::EquationExp::RESIDUAL_EXP { exp: e.clone() })} else {exp.clone()};
            outSource = ElementSource::condAddSymbolicTransformation(changed.clone(), source.clone(), Arc::new(DAE::SymbolicOperation::SIMPLIFY { before: exp.clone(), after: outExp.clone() }))?;
            (outExp.clone(), outSource.clone())
        },
        (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: e1, rhs: e2 }, _) => {
            let mut changed: bool = false;
            let mut changed1: bool = false;
            let mut changed2: bool = false;
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            (e1, changed1) = simplify(e1.clone())?;
            (e2, changed2) = simplify(e2.clone())?;
            changed = changed1.clone() || changed2.clone();
            outExp = if (changed.clone()) {Arc::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1.clone(), rhs: e2.clone() })} else {exp.clone()};
            outSource = ElementSource::condAddSymbolicTransformation(changed.clone(), source.clone(), Arc::new(DAE::SymbolicOperation::SIMPLIFY { before: exp.clone(), after: outExp.clone() }))?;
            (outExp.clone(), outSource.clone())
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("ExpressionSimplify.simplifyAddSymbolicOperation failed")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outSource))
}

pub fn condSimplifyAddSymbolicOperation(mut cond: bool, mut exp: Arc<DAE::EquationExp>, mut source: Arc<DAE::ElementSource>) -> Result<(Arc<DAE::EquationExp>, Arc<DAE::ElementSource>)> {
    let mut exp: Arc<DAE::EquationExp> = exp;
    let mut source: Arc<DAE::ElementSource> = source;
    if cond.clone() {
        (exp, source) = simplifyAddSymbolicOperation(exp.clone(), source.clone())?;
    }
    Ok((exp, source))
}

fn simplifySize(mut origExp: Arc<DAE::Exp>, mut exp: Arc<DAE::Exp>, mut optDim: Option<Arc<DAE::Exp>>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (origExp.clone(), exp.clone(), optDim.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Some(dimExp)) => {
                    let mut i: i32 = 0;
                    let mut n: i32 = 0;
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut dim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    i = Expression::expInt(dimExp.clone())?;
                    t = Expression::r#typeof(exp.clone())?;
                    dims = Expression::arrayDimension(t.clone());
                    dim = (dims.clone()).get(i.clone())?;
                    n = Expression::dimensionSize(dim.clone())?;
                    Ok(Arc::new(DAE::Exp::ICONST { integer: n.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(origExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

fn simplifyTSub(mut origExp: Arc<DAE::Exp>) -> Arc<DAE::Exp> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(origExp.clone()) {
        Deref @ DAE::Exp::TSUB { ix: i, exp: Deref @ DAE::Exp::CAST { exp: Deref @ DAE::Exp::TUPLE { PR: expl }, .. }, .. } => {
            (expl.clone()).get(i.clone()).unwrap()
        },
        Deref @ DAE::Exp::TSUB { ix: i, exp: Deref @ DAE::Exp::TUPLE { PR: expl }, .. } => {
            (expl.clone()).get(i.clone()).unwrap()
        },
        Deref @ DAE::Exp::TSUB { exp: e @ Deref @ DAE::Exp::RCONST { .. }, .. } => {
            e.clone()
        },
        _ => {
            origExp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp
}

fn simplifyNoEvent(mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut e: Arc<DAE::Exp>;
    e = Expression::addNoEventToEventTriggeringFunctions(Expression::addNoEventToRelations(Expression::stripNoEvent(inExp.clone())?)?)?;
    Ok(e)
}

fn maxElement(mut e1: Arc<DAE::Exp>, mut e2: Option<Arc<DAE::Exp>>) -> Option<Arc<DAE::Exp>> {
    let mut elt: Option<Arc<DAE::Exp>> = None;
    elt = (::match_deref::match_deref! { match &((e1.clone(), e2.clone())) {
        (Deref @ DAE::Exp::RCONST { real: _ }, None) => {
            Some(e1.clone())
        },
        (Deref @ DAE::Exp::ICONST { integer: _ }, None) => {
            Some(e1.clone())
        },
        (Deref @ DAE::Exp::BCONST { bool: _ }, None) => {
            Some(e1.clone())
        },
        (Deref @ DAE::Exp::RCONST { real: r1 }, Some(Deref @ DAE::Exp::RCONST { real: r2 })) => {
            if (r1.clone() > r2.clone()) {Some(e1.clone())} else {e2.clone()}
        },
        (Deref @ DAE::Exp::ICONST { integer: i1 }, Some(Deref @ DAE::Exp::ICONST { integer: i2 })) => {
            if (intGt(i1.clone(), i2.clone())) {Some(e1.clone())} else {e2.clone()}
        },
        (Deref @ DAE::Exp::BCONST { bool: b1 }, Some(Deref @ DAE::Exp::BCONST { bool: b2 })) => {
            if (b2.clone() || b1.clone() == b2.clone()) {e2.clone()} else {Some(Arc::new(DAE::Exp::BCONST { bool: b1.clone() }))}
        },
        _ => {
            e2.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    elt
}

fn minElement(mut e1: Arc<DAE::Exp>, mut e2: Option<Arc<DAE::Exp>>) -> Option<Arc<DAE::Exp>> {
    let mut elt: Option<Arc<DAE::Exp>> = None;
    elt = (::match_deref::match_deref! { match &((e1.clone(), e2.clone())) {
        (Deref @ DAE::Exp::RCONST { real: _ }, None) => {
            Some(e1.clone())
        },
        (Deref @ DAE::Exp::ICONST { integer: _ }, None) => {
            Some(e1.clone())
        },
        (Deref @ DAE::Exp::BCONST { bool: _ }, None) => {
            Some(e1.clone())
        },
        (Deref @ DAE::Exp::RCONST { real: r1 }, Some(Deref @ DAE::Exp::RCONST { real: r2 })) => {
            if (r1.clone() < r2.clone()) {Some(e1.clone())} else {e2.clone()}
        },
        (Deref @ DAE::Exp::ICONST { integer: i1 }, Some(Deref @ DAE::Exp::ICONST { integer: i2 })) => {
            if (intLt(i1.clone(), i2.clone())) {Some(e1.clone())} else {e2.clone()}
        },
        (Deref @ DAE::Exp::BCONST { bool: b1 }, Some(Deref @ DAE::Exp::BCONST { bool: b2 })) => {
            if (!(b2.clone()) || b1.clone() == b2.clone()) {e2.clone()} else {Some(Arc::new(DAE::Exp::BCONST { bool: b1.clone() }))}
        },
        _ => {
            e2.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    elt
}

fn removeMinMaxFoldableValues(mut e: Arc<DAE::Exp>) -> bool {
    let mut filter: bool = false;
    filter = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Exp::RCONST { real: _ } => false,
        Deref @ DAE::Exp::ICONST { integer: _ } => false,
        Deref @ DAE::Exp::BCONST { bool: _ } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    filter
}

pub fn simplifySkew(mut v1: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>> {
    let mut res: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
    let mut x1: Arc<DAE::Exp>;
    let mut x2: Arc<DAE::Exp>;
    let mut x3: Arc<DAE::Exp>;
    let mut zero: Arc<DAE::Exp>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(v1.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    x1 = __pa0.clone();
    x2 = __pa1.clone();
    x3 = __pa2.clone();
    zero = Expression::makeConstZero(Expression::r#typeof(x1.clone())?);
    res = list![list![zero.clone(), Expression::negate(x3.clone())?, x2.clone()], list![x3.clone(), zero.clone(), Expression::negate(x1.clone())?], list![Expression::negate(x2.clone())?, x1.clone(), zero.clone()]];
    Ok(res)
}

pub fn simplifyCross(mut v1: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut v2: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut res: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut x1: Arc<DAE::Exp>;
    let mut x2: Arc<DAE::Exp>;
    let mut x3: Arc<DAE::Exp>;
    let mut y1: Arc<DAE::Exp>;
    let mut y2: Arc<DAE::Exp>;
    let mut y3: Arc<DAE::Exp>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(v1.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    x1 = __pa0.clone();
    x2 = __pa1.clone();
    x3 = __pa2.clone();
    let (__pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(v2.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Nil } } } => (__pa4.clone(), __pa5.clone(), __pa6.clone()),
        _ => bail!("pattern mismatch"),
    } };
    y1 = __pa4.clone();
    y2 = __pa5.clone();
    y3 = __pa6.clone();
    res = list![Expression::expSub(Expression::makeProduct(x2.clone(), y3.clone())?, Expression::makeProduct(x3.clone(), y2.clone())?)?, Expression::expSub(Expression::makeProduct(x3.clone(), y1.clone())?, Expression::makeProduct(x1.clone(), y3.clone())?)?, Expression::expSub(Expression::makeProduct(x1.clone(), y2.clone())?, Expression::makeProduct(x2.clone(), y1.clone())?)?];
    Ok(res)
}

