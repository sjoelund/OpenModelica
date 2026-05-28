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

use crate::BackendDAE;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVariable;
use openmodelica_ast::Absyn;
use openmodelica_frontend::Algorithm;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionDump;
use openmodelica_frontend::ExpressionSimplify;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::Values;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

//--------------------------------
// collect for-loops
//--------------------------------
pub fn collectForLoops(mut varsIn: Arc<metamodelica::List<BackendDAE::Var>>, mut eqsIn: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut varsOut: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut eqsOut: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut arrayCrefs: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>> = metamodelica::nil();
    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut arrayVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut forEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut mixEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut nonArrEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    (varLst, arrayVars) = List::fold(varsIn.clone(), (std::sync::Arc::new(getArrayVars) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>)) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>)> + 'static>), (metamodelica::nil(), metamodelica::nil()));
    (arrayCrefs, _) = List::fold(arrayVars.clone(), (std::sync::Arc::new(getArrayVarCrefs) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<BackendDAE::Var>>)) -> Result<(Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<BackendDAE::Var>>)> + 'static>), (metamodelica::nil(), metamodelica::nil()));
    (forEqs, mixEqs, nonArrEqs) = List::fold1(eqsIn.clone(), (std::sync::Arc::new(dispatchLoopEquations) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> + 'static>), List::map(arrayCrefs.clone(), std::sync::Arc::new(fnptr!(Util::tuple31, _))), (metamodelica::nil(), metamodelica::nil(), metamodelica::nil()));
    forEqs = buildBackendDAEForEquations(forEqs.clone(), metamodelica::nil())?;
    mixEqs = List::fold(mixEqs.clone(), (std::sync::Arc::new(buildAccumExpInEquations) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> + 'static>), metamodelica::nil()).reverse();
    arrayVars = unexpandArrayVariables(arrayVars.clone(), metamodelica::nil())?;
    eqsOut = listAppend(forEqs.clone(), listAppend(mixEqs.clone(), nonArrEqs.clone()));
    varsOut = listAppend(arrayVars.clone(), varLst.clone());
    Ok((varsOut, eqsOut))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn unexpandArrayVariables(mut varsIn: Arc<metamodelica::List<BackendDAE::Var>>, mut foldIn: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut foldOut: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    foldOut = 'mc: {
        let __mc_input = varsIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(foldIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: var, tail: rest } => {
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut scalars: Arc<metamodelica::List</* ? */>> = metamodelica::nil();
                    let mut var = (*var).clone();
                    let mut rest = (*rest).clone();
                    cref = BackendVariable::varCref(var.clone())?;
                    let true = (ComponentReference::crefHaveSubs(cref.clone())?) else { bail!("pattern mismatch") };
                    (scalars, rest) = List::split1OnTrue(rest.clone(), (std::sync::Arc::new(varIsEqualCrefWithoutSubs) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), cref.clone());
                    cref = replaceFirstSubsInCref(cref.clone(), list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::RANGE { ty: BackendVariable::varType(var.clone())?, start: Arc::new(DAE::Exp::ICONST { integer: 1 }), step: None, stop: Arc::new(DAE::Exp::ICONST { integer: (scalars.clone().len() as i32) + 1 }) }) })])?;
                    var = BackendVariable::copyVarNewName(cref.clone(), var.clone());
                    Ok(unexpandArrayVariables(rest.clone(), cons(var.clone(), foldIn.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: var, tail: rest } => {
                    Ok(unexpandArrayVariables(rest.clone(), cons(var.clone(), foldIn.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(foldOut)
}

fn varIsEqualCrefWithoutSubs(mut varIn: BackendDAE::Var, mut crefIn: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut b: bool = false;
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    cref = BackendVariable::varCref(varIn.clone())?;
    b = ComponentReferenceBasics::crefEqualWithoutSubs(cref.clone(), crefIn.clone());
    Ok(b)
}

fn buildAccumExpInEquations(mut mixEq: Arc<BackendDAE::Equation>, mut foldIn: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut foldOut: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    foldOut = 'mc: {
        let __mc_input = mixEq.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::EQUATION { attr, source, scalar: lhs, exp: rhs } => {
                    let mut allTerms: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut minmaxTerms: Arc<metamodelica::List<(Arc<DAE::Exp>, i32, i32)>> = metamodelica::nil();
                    let mut lhs = (*lhs).clone();
                    let mut rhs = (*rhs).clone();
                    allTerms = Expression::allTerms(lhs.clone())?;
                    minmaxTerms = List::fold(allTerms.clone(), (std::sync::Arc::new(fnptr!(buildAccumExpInEquations1, Arc<DAE::Exp>, Arc<metamodelica::List<(Arc<DAE::Exp>, i32, i32)>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<(Arc<DAE::Exp>, i32, i32)>>) -> Result<Arc<metamodelica::List<(Arc<DAE::Exp>, i32, i32)>>> + 'static>), metamodelica::nil());
                    let __pa0 = ::match_deref::match_deref! { match &(buildAccumExpInEquations2(minmaxTerms.clone().reverse(), metamodelica::nil())?) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    lhs = __pa0.clone();
                    allTerms = Expression::allTerms(rhs.clone())?;
                    minmaxTerms = List::fold(allTerms.clone(), (std::sync::Arc::new(fnptr!(buildAccumExpInEquations1, Arc<DAE::Exp>, Arc<metamodelica::List<(Arc<DAE::Exp>, i32, i32)>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<(Arc<DAE::Exp>, i32, i32)>>) -> Result<Arc<metamodelica::List<(Arc<DAE::Exp>, i32, i32)>>> + 'static>), metamodelica::nil());
                    let __pa2 = ::match_deref::match_deref! { match &(buildAccumExpInEquations2(minmaxTerms.clone().reverse(), metamodelica::nil())?) {
                        Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil } => __pa2.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    rhs = __pa2.clone();
                    Ok(cons(Arc::new(BackendDAE::Equation::EQUATION { exp: rhs.clone(), scalar: lhs.clone(), source: source.clone(), attr: attr.clone() }), foldIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(cons(mixEq.clone(), foldIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(foldOut)
}

fn buildAccumExpInEquations1(mut termIn: Arc<DAE::Exp>, mut minmaxTermsIn: Arc<metamodelica::List<(Arc<DAE::Exp>, i32, i32)>>) -> Arc<metamodelica::List<(Arc<DAE::Exp>, i32, i32)>> {
    let mut minmaxTermsOut: Arc<metamodelica::List<(Arc<DAE::Exp>, i32, i32)>> = metamodelica::nil();
    let mut pos: i32 = 0;
    let mut idx: i32 = 0;
    let mut min: i32 = 0;
    let mut max: i32 = 0;
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut term: Arc<DAE::Exp>;
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(Expression::extractCrefsFromExp(termIn.clone()), '__try0)) {
            Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        cref = __pa1.clone();
        let true = (unwrap_break_err!(ComponentReference::crefHaveSubs(cref.clone()), '__try0)) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        pos = List::position1OnTrue(minmaxTermsIn.clone(), (std::sync::Arc::new(minmaxTermEqual) as std::sync::Arc<dyn ::std::ops::Fn((Arc<DAE::Exp>, i32, i32), Arc<DAE::Exp>) -> Result<bool> + 'static>), termIn.clone());
        if intEq(pos.clone(), -1) {
            let __pa3 = ::match_deref::match_deref! { match &(unwrap_break_err!(ComponentReferenceBasics::crefSubs(cref.clone()), '__try0)) {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: __pa3 } }, tail: Deref @ metamodelica::List::Nil } => __pa3.clone(),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            idx = __pa3.clone();
            minmaxTermsOut = cons((termIn.clone(), idx.clone(), idx.clone()), minmaxTermsIn.clone());
        } else {
            (term, min, max) = unwrap_break_err!((minmaxTermsIn.clone()).get(pos.clone()), '__try0);
            let __pa6 = ::match_deref::match_deref! { match &(unwrap_break_err!(ComponentReferenceBasics::crefSubs(cref.clone()), '__try0)) {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: __pa6 } }, tail: Deref @ metamodelica::List::Nil } => __pa6.clone(),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            idx = __pa6.clone();
            minmaxTermsOut = unwrap_break_err!(List::replaceAt((term.clone(), intMin(idx.clone(), min.clone()), intMax(idx.clone(), max.clone())), pos.clone(), minmaxTermsIn.clone()), '__try0);
        }
        Ok::<_, anyhow::Error>((minmaxTermsOut.clone(),))
    } {
        Ok((__try0_o0,)) => {
            minmaxTermsOut = __try0_o0;
        }
        Err(_) => {
            minmaxTermsOut = cons((termIn.clone(), -1, -1), minmaxTermsIn.clone());
        }
    }
    minmaxTermsOut
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn buildAccumExpInEquations2(mut minmaxTerm: Arc<metamodelica::List<(Arc<DAE::Exp>, i32, i32)>>, mut foldIn: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut foldOut: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let sumReductionInfo: Arc<DAE::ReductionInfo> = Arc::new(DAE::ReductionInfo { path: Arc::new(Absyn::Path::IDENT { name: (literal!("sum")).clone() }), iterType: openmodelica_ast::Absyn::ReductionIterType::COMBINE, exprType: DAE::T_REAL_DEFAULT().clone(), defaultValue: Some(Arc::new(Values::Value::REAL { real: metamodelica::OrderedFloat(0.0_f64) })), foldName: (literal!("$sumFold")).clone(), resultName: (literal!("$sumRes")).clone(), foldExp: Some(Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (literal!("$sumFold")).clone(), identType: DAE::T_REAL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_REAL_DEFAULT().clone() }), operator: DAE::Operator::ADD { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (literal!("$sumRes")).clone(), identType: DAE::T_REAL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_REAL_DEFAULT().clone() }) })) });
    let sumExp: Arc<DAE::Exp> = Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (literal!("$sumIter")).clone(), identType: DAE::T_REAL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_REAL_DEFAULT().clone() });
    foldOut = 'mc: {
        let __mc_input = (minmaxTerm.clone(), foldIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Cons { head: exp1, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(list![exp1.clone()])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (exp1, min, max), tail: rest }, Deref @ metamodelica::List::Nil) => {
                    let mut iter: Arc<DAE::Exp>;
                    let mut resExp: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut exp1 = (*exp1).clone();
                    let mut rest = (*rest).clone();
                    let true = (intNe(min.clone(), max.clone())) else { bail!("pattern mismatch") };
                    ::match_deref::match_deref! { match &(Expression::r#typeof(exp1.clone())?) {
                        Deref @ DAE::Type::T_REAL { .. } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    (_, rest) = List::split1OnTrue(rest.clone(), (std::sync::Arc::new(minmaxTermEqual) as std::sync::Arc<dyn ::std::ops::Fn((Arc<DAE::Exp>, i32, i32), Arc<DAE::Exp>) -> Result<bool> + 'static>), exp1.clone());
                    iter = Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (literal!("i")).clone(), identType: DAE::T_INTEGER_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_INTEGER_DEFAULT().clone() });
                    (exp1, _) = Expression::traverseExpBottomUp(exp1.clone(), (std::sync::Arc::new(replaceSubscriptInCrefExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Subscript>>>)> + 'static>), list![Arc::new(DAE::Subscript::INDEX { exp: iter.clone() })])?;
                    exp1 = Arc::new(DAE::Exp::REDUCTION { reductionInfo: sumReductionInfo.clone(), expr: sumExp.clone(), iterators: list![Arc::new(DAE::ReductionIterator { id: (literal!("$sumIter")).clone(), exp: Arc::new(DAE::Exp::RANGE { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: max.clone() - min.clone() })] }), start: Arc::new(DAE::Exp::ICONST { integer: min.clone() }), step: None, stop: Arc::new(DAE::Exp::ICONST { integer: max.clone() }) }), guardExp: None, ty: DAE::T_INTEGER_DEFAULT().clone() })] });
                    resExp = buildAccumExpInEquations2(rest.clone(), list![exp1.clone()])?;
                    Ok(resExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (exp1, min, max), tail: rest }, Deref @ metamodelica::List::Cons { head: exp0, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut iter: Arc<DAE::Exp>;
                    let mut resExp: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut exp1 = (*exp1).clone();
                    let mut rest = (*rest).clone();
                    let true = (intNe(min.clone(), max.clone())) else { bail!("pattern mismatch") };
                    ::match_deref::match_deref! { match &(Expression::r#typeof(exp1.clone())?) {
                        Deref @ DAE::Type::T_REAL { .. } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    (_, rest) = List::split1OnTrue(rest.clone(), (std::sync::Arc::new(minmaxTermEqual) as std::sync::Arc<dyn ::std::ops::Fn((Arc<DAE::Exp>, i32, i32), Arc<DAE::Exp>) -> Result<bool> + 'static>), exp1.clone());
                    iter = Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (literal!("i")).clone(), identType: DAE::T_INTEGER_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_INTEGER_DEFAULT().clone() });
                    (exp1, _) = Expression::traverseExpBottomUp(exp1.clone(), (std::sync::Arc::new(replaceSubscriptInCrefExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Subscript>>>)> + 'static>), list![Arc::new(DAE::Subscript::INDEX { exp: iter.clone() })])?;
                    exp1 = Arc::new(DAE::Exp::REDUCTION { reductionInfo: sumReductionInfo.clone(), expr: sumExp.clone(), iterators: list![Arc::new(DAE::ReductionIterator { id: (literal!("$sumIter")).clone(), exp: Arc::new(DAE::Exp::RANGE { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: max.clone() - min.clone() })] }), start: Arc::new(DAE::Exp::ICONST { integer: min.clone() }), step: None, stop: Arc::new(DAE::Exp::ICONST { integer: max.clone() }) }), guardExp: None, ty: DAE::T_INTEGER_DEFAULT().clone() })] });
                    resExp = buildAccumExpInEquations2(rest.clone(), list![Arc::new(DAE::Exp::BINARY { exp1: exp0.clone(), operator: DAE::Operator::ADD { ty: Expression::r#typeof(exp0.clone())? }, exp2: exp1.clone() })])?;
                    Ok(resExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (exp1, _, _), tail: rest }, Deref @ metamodelica::List::Nil) => {
                    let mut resExp: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    resExp = buildAccumExpInEquations2(rest.clone(), list![exp1.clone()])?;
                    Ok(resExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (exp1, _, _), tail: rest }, Deref @ metamodelica::List::Cons { head: exp0, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut resExp: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    resExp = buildAccumExpInEquations2(rest.clone(), list![Arc::new(DAE::Exp::BINARY { exp1: exp0.clone(), operator: DAE::Operator::ADD { ty: Expression::r#typeof(exp0.clone())? }, exp2: exp1.clone() })])?;
                    Ok(resExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(foldOut)
}

pub fn replaceSubscriptInCrefExp(mut expIn: Arc<DAE::Exp>, mut subsIn: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Subscript>>>)> {
    let mut expOut: Arc<DAE::Exp>;
    let mut subsOut: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
    (expOut, subsOut) = 'mc: {
        let __mc_input = expIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { ty, componentRef: cref } => {
                    let mut cref = (*cref).clone();
                    cref = replaceFirstSubsInCref(cref.clone(), subsIn.clone())?;
                    Ok((Arc::new(DAE::Exp::CREF { componentRef: cref.clone(), ty: ty.clone() }), subsIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((expIn.clone(), subsIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((expOut, subsOut))
}

fn minmaxTermEqual(mut minmaxTerm: (Arc<DAE::Exp>, i32, i32), mut term: Arc<DAE::Exp>) -> Result<bool> {
    let mut b: bool = false;
    let mut term0: Arc<DAE::Exp>;
    (term0, _, _) = minmaxTerm.clone();
    b = expEqualNoCrefSubs(term0.clone(), term.clone())?;
    Ok(b)
}

pub fn equationEqualNoCrefSubs(mut e1: Arc<BackendDAE::Equation>, mut e2: Arc<BackendDAE::Equation>) -> Result<bool> {
    let mut res: bool = false;
    res = 'mc: {
        let __mc_input = (e1.clone(), e2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let true = (referenceEq(&e1.clone(),&e2.clone())) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { scalar: e12, exp: e11, .. }, Deref @ BackendDAE::Equation::EQUATION { scalar: e22, exp: e21, .. }) => {
                    let mut terms1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut terms2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut crefs1: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut crefs2: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut commCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut res: bool = res.clone();
                    if boolAnd(expEqualNoCrefSubs(e11.clone(), e21.clone())?, expEqualNoCrefSubs(e12.clone(), e22.clone())?) {
                        res = true;
                    } else {
                        crefs1 = BackendEquation::equationCrefs(e1.clone())?;
                        crefs2 = BackendEquation::equationCrefs(e2.clone())?;
                        commCrefs = List::intersectionOnTrue(crefs1.clone(), crefs2.clone(), (std::sync::Arc::new(fnptr!(ComponentReferenceBasics::crefEqualWithoutSubs, Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>));
                        if intEq((crefs1.clone().len() as i32), (commCrefs.clone().len() as i32)) && intEq((crefs2.clone().len() as i32), (commCrefs.clone().len() as i32)) {
                            terms1 = listAppend(Expression::allTerms(e11.clone())?, Expression::allTerms(e12.clone())?);
                            terms2 = listAppend(Expression::allTerms(e21.clone())?, Expression::allTerms(e22.clone())?);
                            (_, terms1, terms2) = List::intersection1OnTrue(terms1.clone(), terms2.clone(), (std::sync::Arc::new(expEqualNoCrefSubs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>))?;
                            res = terms1.clone().is_empty() && terms2.clone().is_empty();
                        } else {
                            res = false;
                        }
                    }
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::ARRAY_EQUATION { right: e12, left: e11, .. }, Deref @ BackendDAE::Equation::ARRAY_EQUATION { right: e22, left: e21, .. }) => {
                    let mut res: bool = res.clone();
                    res = boolAnd(expEqualNoCrefSubs(e11.clone(), e21.clone())?, expEqualNoCrefSubs(e12.clone(), e22.clone())?);
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: e12, left: e11, .. }, Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: e22, left: e21, .. }) => {
                    let mut res: bool = res.clone();
                    res = boolAnd(expEqualNoCrefSubs(e11.clone(), e21.clone())?, expEqualNoCrefSubs(e12.clone(), e22.clone())?);
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::SOLVED_EQUATION { exp: exp1, componentRef: cr1, .. }, Deref @ BackendDAE::Equation::SOLVED_EQUATION { exp: exp2, componentRef: cr2, .. }) => {
                    let mut res: bool = res.clone();
                    res = boolAnd(ComponentReferenceBasics::crefEqualWithoutSubs(cr1.clone(), cr2.clone()), expEqualNoCrefSubs(exp1.clone(), exp2.clone())?);
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: exp1, .. }, Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: exp2, .. }) => {
                    let mut res: bool = res.clone();
                    res = expEqualNoCrefSubs(exp1.clone(), exp2.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::ALGORITHM { alg: alg1, .. }, Deref @ BackendDAE::Equation::ALGORITHM { alg: alg2, .. }) => {
                    let mut explst1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut explst2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut res: bool = res.clone();
                    explst1 = Algorithm::getAllExps(alg1.clone())?;
                    explst2 = Algorithm::getAllExps(alg2.clone())?;
                    res = List::isEqualOnTrue(explst1.clone(), explst2.clone(), (std::sync::Arc::new(expEqualNoCrefSubs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>));
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
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
pub fn expEqualNoCrefSubs(mut inExp1: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>) -> Result<bool> {
    let mut outEqual: bool = false;
    if referenceEq(&inExp1.clone(),&inExp2.clone()) {
        outEqual = true;
        return Ok(outEqual.clone());
    }
    if metamodelica::valueConstructor((&*inExp1.clone()))? != metamodelica::valueConstructor((&*inExp2.clone()))? {
        outEqual = false;
        return Ok(outEqual.clone());
    }
    outEqual = (::match_deref::match_deref! { match &(inExp1.clone()) {
        Deref @ DAE::Exp::ICONST { .. } => {
            let mut i: i32 = 0;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::ICONST { integer: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            i = __pa0.clone();
            var_field!((*inExp1).integer, DAE::Exp::ICONST).clone() == i.clone()
        },
        Deref @ DAE::Exp::RCONST { .. } => {
            let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let __pa0 = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::RCONST { real: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            r = __pa0.clone();
            var_field!((*inExp1).real, DAE::Exp::RCONST).clone() == r.clone()
        },
        Deref @ DAE::Exp::SCONST { .. } => {
            let mut s: ArcStr = arcstr::literal!("");
            let __pa0 = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::SCONST { string: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            s = __pa0.clone();
            var_field!((*inExp1).string, DAE::Exp::SCONST).clone() == s.clone()
        },
        Deref @ DAE::Exp::BCONST { .. } => {
            let mut b: bool = false;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::BCONST { bool: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            b = __pa0.clone();
            var_field!((*inExp1).bool, DAE::Exp::BCONST).clone() == b.clone()
        },
        Deref @ DAE::Exp::ENUM_LITERAL { .. } => {
            let mut p: Arc<Absyn::Path>;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::ENUM_LITERAL { name: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            p = __pa0.clone();
            AbsynUtil::pathEqual(var_field!((*inExp1).name, DAE::Exp::ENUM_LITERAL).clone(), p.clone())
        },
        Deref @ DAE::Exp::CREF { .. } => {
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let __pa0 = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::CREF { componentRef: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cr = __pa0.clone();
            ComponentReferenceBasics::crefEqualWithoutSubs(var_field!((*inExp1).componentRef, DAE::Exp::CREF).clone(), cr.clone())
        },
        Deref @ DAE::Exp::ARRAY { .. } => {
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::ARRAY { array: __pa0, ty: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            expl = __pa0.clone();
            ty = __pa1.clone();
            var_field!((*inExp1).ty, DAE::Exp::ARRAY).clone() == ty.clone() && expEqualNoCrefSubsList(var_field!((*inExp1).array, DAE::Exp::ARRAY).clone(), expl.clone())?
        },
        Deref @ DAE::Exp::MATRIX { .. } => {
            let mut mexpl: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::MATRIX { matrix: __pa0, ty: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            mexpl = __pa0.clone();
            ty = __pa1.clone();
            var_field!((*inExp1).ty, DAE::Exp::MATRIX).clone() == ty.clone() && expEqualNoCrefSubsListList(var_field!((*inExp1).matrix, DAE::Exp::MATRIX).clone(), mexpl.clone())?
        },
        Deref @ DAE::Exp::BINARY { .. } => {
            let mut e1: Arc<DAE::Exp>;
            let mut e2: Arc<DAE::Exp>;
            let mut op: DAE::Operator;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::BINARY { exp2: __pa0, operator: __pa1, exp1: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e2 = __pa0.clone();
            op = __pa1.clone();
            e1 = __pa2.clone();
            Expression::operatorEqual(var_field!((*inExp1).operator, DAE::Exp::BINARY).clone(), op.clone())? && expEqualNoCrefSubs(var_field!((*inExp1).exp1, DAE::Exp::BINARY).clone(), e1.clone())? && expEqualNoCrefSubs(var_field!((*inExp1).exp2, DAE::Exp::BINARY).clone(), e2.clone())?
        },
        Deref @ DAE::Exp::LBINARY { .. } => {
            let mut e1: Arc<DAE::Exp>;
            let mut e2: Arc<DAE::Exp>;
            let mut op: DAE::Operator;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::LBINARY { exp2: __pa0, operator: __pa1, exp1: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e2 = __pa0.clone();
            op = __pa1.clone();
            e1 = __pa2.clone();
            Expression::operatorEqual(var_field!((*inExp1).operator, DAE::Exp::LBINARY).clone(), op.clone())? && expEqualNoCrefSubs(var_field!((*inExp1).exp1, DAE::Exp::LBINARY).clone(), e1.clone())? && expEqualNoCrefSubs(var_field!((*inExp1).exp2, DAE::Exp::LBINARY).clone(), e2.clone())?
        },
        Deref @ DAE::Exp::UNARY { .. } => {
            let mut e: Arc<DAE::Exp>;
            let mut op: DAE::Operator;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::UNARY { operator: __pa0, exp: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            op = __pa0.clone();
            e = __pa1.clone();
            Expression::operatorEqual(var_field!((*inExp1).operator, DAE::Exp::UNARY).clone(), op.clone())? && expEqualNoCrefSubs(var_field!((*inExp1).exp, DAE::Exp::UNARY).clone(), e.clone())?
        },
        Deref @ DAE::Exp::LUNARY { .. } => {
            let mut e: Arc<DAE::Exp>;
            let mut op: DAE::Operator;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::LUNARY { operator: __pa0, exp: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            op = __pa0.clone();
            e = __pa1.clone();
            Expression::operatorEqual(var_field!((*inExp1).operator, DAE::Exp::LUNARY).clone(), op.clone())? && expEqualNoCrefSubs(var_field!((*inExp1).exp, DAE::Exp::LUNARY).clone(), e.clone())?
        },
        Deref @ DAE::Exp::RELATION { .. } => {
            let mut e1: Arc<DAE::Exp>;
            let mut e2: Arc<DAE::Exp>;
            let mut op: DAE::Operator;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::RELATION { exp2: __pa0, operator: __pa1, exp1: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e2 = __pa0.clone();
            op = __pa1.clone();
            e1 = __pa2.clone();
            Expression::operatorEqual(var_field!((*inExp1).operator, DAE::Exp::RELATION).clone(), op.clone())? && expEqualNoCrefSubs(var_field!((*inExp1).exp1, DAE::Exp::RELATION).clone(), e1.clone())? && expEqualNoCrefSubs(var_field!((*inExp1).exp2, DAE::Exp::RELATION).clone(), e2.clone())?
        },
        Deref @ DAE::Exp::IFEXP { .. } => {
            let mut e: Arc<DAE::Exp>;
            let mut e1: Arc<DAE::Exp>;
            let mut e2: Arc<DAE::Exp>;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::IFEXP { expElse: __pa0, expThen: __pa1, expCond: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e2 = __pa0.clone();
            e1 = __pa1.clone();
            e = __pa2.clone();
            expEqualNoCrefSubs(var_field!((*inExp1).expCond, DAE::Exp::IFEXP).clone(), e.clone())? && expEqualNoCrefSubs(var_field!((*inExp1).expThen, DAE::Exp::IFEXP).clone(), e1.clone())? && expEqualNoCrefSubs(var_field!((*inExp1).expElse, DAE::Exp::IFEXP).clone(), e2.clone())?
        },
        Deref @ DAE::Exp::CALL { .. } => {
            let mut p: Arc<Absyn::Path>;
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::CALL { expLst: __pa0, path: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            expl = __pa0.clone();
            p = __pa1.clone();
            AbsynUtil::pathEqual(var_field!((*inExp1).path, DAE::Exp::CALL).clone(), p.clone()) && expEqualNoCrefSubsList(var_field!((*inExp1).expLst, DAE::Exp::CALL).clone(), expl.clone())?
        },
        Deref @ DAE::Exp::RECORD { .. } => {
            let mut p: Arc<Absyn::Path>;
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::RECORD { exps: __pa0, path: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            expl = __pa0.clone();
            p = __pa1.clone();
            AbsynUtil::pathEqual(var_field!((*inExp1).path, DAE::Exp::RECORD).clone(), p.clone()) && expEqualNoCrefSubsList(var_field!((*inExp1).exps, DAE::Exp::RECORD).clone(), expl.clone())?
        },
        Deref @ DAE::Exp::PARTEVALFUNCTION { .. } => {
            let mut p: Arc<Absyn::Path>;
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::PARTEVALFUNCTION { expList: __pa0, path: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            expl = __pa0.clone();
            p = __pa1.clone();
            AbsynUtil::pathEqual(var_field!((*inExp1).path, DAE::Exp::PARTEVALFUNCTION).clone(), p.clone()) && expEqualNoCrefSubsList(var_field!((*inExp1).expList, DAE::Exp::PARTEVALFUNCTION).clone(), expl.clone())?
        },
        Deref @ DAE::Exp::RANGE { .. } => {
            let mut e1: Arc<DAE::Exp>;
            let mut e2: Arc<DAE::Exp>;
            let mut oe: Option<Arc<DAE::Exp>> = None;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::RANGE { stop: __pa0, step: __pa1, start: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e2 = __pa0.clone();
            oe = __pa1.clone();
            e1 = __pa2.clone();
            expEqualNoCrefSubs(var_field!((*inExp1).start, DAE::Exp::RANGE).clone(), e1.clone())? && expEqualNoCrefSubs(var_field!((*inExp1).stop, DAE::Exp::RANGE).clone(), e2.clone())? && expEqualNoCrefSubsOpt(var_field!((*inExp1).step, DAE::Exp::RANGE).clone(), oe.clone())?
        },
        Deref @ DAE::Exp::TUPLE { .. } => {
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let __pa0 = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::TUPLE { PR: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            expl = __pa0.clone();
            expEqualNoCrefSubsList(var_field!((*inExp1).PR, DAE::Exp::TUPLE).clone(), expl.clone())?
        },
        Deref @ DAE::Exp::CAST { .. } => {
            let mut e: Arc<DAE::Exp>;
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::CAST { exp: __pa0, ty: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e = __pa0.clone();
            ty = __pa1.clone();
            var_field!((*inExp1).ty, DAE::Exp::CAST).clone() == ty.clone() && expEqualNoCrefSubs(var_field!((*inExp1).exp, DAE::Exp::CAST).clone(), e.clone())?
        },
        Deref @ DAE::Exp::ASUB { .. } => {
            let mut e: Arc<DAE::Exp>;
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut expl2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
            expl = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut sub in (var_field!((*inExp1).sub, DAE::Exp::ASUB).clone()).into_iter().cloned() {
            let __x = Expression::getSubscriptExp(sub.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::ASUB { sub: __pa0, exp: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            subs = __pa0.clone();
            e = __pa1.clone();
            expl2 = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut sub in (subs.clone()).into_iter().cloned() {
            let __x = Expression::getSubscriptExp(sub.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            expEqualNoCrefSubs(var_field!((*inExp1).exp, DAE::Exp::ASUB).clone(), e.clone())? && expEqualNoCrefSubsList(expl.clone(), expl2.clone())?
        },
        Deref @ DAE::Exp::SIZE { .. } => {
            let mut e: Arc<DAE::Exp>;
            let mut oe: Option<Arc<DAE::Exp>> = None;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::SIZE { sz: __pa0, exp: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            oe = __pa0.clone();
            e = __pa1.clone();
            expEqualNoCrefSubs(var_field!((*inExp1).exp, DAE::Exp::SIZE).clone(), e.clone())? && expEqualNoCrefSubsOpt(var_field!((*inExp1).sz, DAE::Exp::SIZE).clone(), oe.clone())?
        },
        Deref @ DAE::Exp::REDUCTION { .. } => {
            inExp1.clone() == inExp2.clone()
        },
        Deref @ DAE::Exp::LIST { .. } => {
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let __pa0 = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::LIST { valList: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            expl = __pa0.clone();
            expEqualNoCrefSubsList(var_field!((*inExp1).valList, DAE::Exp::LIST).clone(), expl.clone())?
        },
        Deref @ DAE::Exp::CONS { .. } => {
            let mut e1: Arc<DAE::Exp>;
            let mut e2: Arc<DAE::Exp>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::CONS { cdr: __pa0, car: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e2 = __pa0.clone();
            e1 = __pa1.clone();
            expEqualNoCrefSubs(var_field!((*inExp1).car, DAE::Exp::CONS).clone(), e1.clone())? && expEqualNoCrefSubs(var_field!((*inExp1).cdr, DAE::Exp::CONS).clone(), e2.clone())?
        },
        Deref @ DAE::Exp::META_TUPLE { .. } => {
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let __pa0 = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::META_TUPLE { listExp: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            expl = __pa0.clone();
            expEqualNoCrefSubsList(var_field!((*inExp1).listExp, DAE::Exp::META_TUPLE).clone(), expl.clone())?
        },
        Deref @ DAE::Exp::META_OPTION { .. } => {
            let mut oe: Option<Arc<DAE::Exp>> = None;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::META_OPTION { exp: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            oe = __pa0.clone();
            expEqualNoCrefSubsOpt(var_field!((*inExp1).exp, DAE::Exp::META_OPTION).clone(), oe.clone())?
        },
        Deref @ DAE::Exp::METARECORDCALL { .. } => {
            let mut p: Arc<Absyn::Path>;
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::METARECORDCALL { args: __pa0, path: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            expl = __pa0.clone();
            p = __pa1.clone();
            AbsynUtil::pathEqual(var_field!((*inExp1).path, DAE::Exp::METARECORDCALL).clone(), p.clone()) && expEqualNoCrefSubsList(var_field!((*inExp1).args, DAE::Exp::METARECORDCALL).clone(), expl.clone())?
        },
        Deref @ DAE::Exp::MATCHEXPRESSION { .. } => {
            inExp1.clone() == inExp2.clone()
        },
        Deref @ DAE::Exp::BOX { .. } => {
            let mut e: Arc<DAE::Exp>;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::BOX { exp: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            e = __pa0.clone();
            expEqualNoCrefSubs(var_field!((*inExp1).exp, DAE::Exp::BOX).clone(), e.clone())?
        },
        Deref @ DAE::Exp::UNBOX { .. } => {
            let mut e: Arc<DAE::Exp>;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::UNBOX { exp: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            e = __pa0.clone();
            expEqualNoCrefSubs(var_field!((*inExp1).exp, DAE::Exp::UNBOX).clone(), e.clone())?
        },
        Deref @ DAE::Exp::SHARED_LITERAL { .. } => {
            let mut i: i32 = 0;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::SHARED_LITERAL { index: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            i = __pa0.clone();
            var_field!((*inExp1).index, DAE::Exp::SHARED_LITERAL).clone() == i.clone()
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEqual)
}

fn expEqualNoCrefSubsOpt(mut inExp1: Option<Arc<DAE::Exp>>, mut inExp2: Option<Arc<DAE::Exp>>) -> Result<bool> {
    let mut outEqual: bool = false;
    let mut e1: Arc<DAE::Exp>;
    let mut e2: Arc<DAE::Exp>;
    outEqual = (::match_deref::match_deref! { match &((inExp1.clone(), inExp2.clone())) {
        (None, None) => true,
        (Some(e1), Some(e2)) => expEqualNoCrefSubs(e1.clone(), e2.clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEqual)
}

fn expEqualNoCrefSubsList(mut inExpl1: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inExpl2: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<bool> {
    let mut outEqual: bool = false;
    let mut e2: Arc<DAE::Exp>;
    let mut rest_expl2: Arc<metamodelica::List<Arc<DAE::Exp>>> = inExpl2.clone();
    if (inExpl1.clone().len() as i32) != (inExpl2.clone().len() as i32) {
        outEqual = false;
        return Ok(outEqual.clone());
    }
    for mut e1 in &*inExpl1.clone() {
        let mut e1 = e1.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_expl2.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e2 = __pa0.clone();
        rest_expl2 = __pa1.clone();
        if !(expEqualNoCrefSubs(e1.clone(), e2.clone())?) {
            outEqual = false;
            return Ok(outEqual.clone());
        }
    }
    outEqual = true;
    Ok(outEqual)
}

fn expEqualNoCrefSubsListList(mut inExpl1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, mut inExpl2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>) -> Result<bool> {
    let mut outEqual: bool = false;
    let mut expl2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut rest_expl2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = inExpl2.clone();
    if (inExpl1.clone().len() as i32) != (inExpl2.clone().len() as i32) {
        outEqual = false;
        return Ok(outEqual.clone());
    }
    for mut expl1 in &*inExpl1.clone() {
        let mut expl1 = expl1.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_expl2.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        expl2 = __pa0.clone();
        rest_expl2 = __pa1.clone();
        if !(expEqualNoCrefSubsList(expl1.clone(), expl2.clone())?) {
            outEqual = false;
            return Ok(outEqual.clone());
        }
    }
    outEqual = true;
    Ok(outEqual)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn buildBackendDAEForEquations(mut classEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut foldIn: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut foldOut: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    foldOut = 'mc: {
        let __mc_input = classEqs.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(foldIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: eq, tail: rest } => {
                    let mut lhs: Arc<DAE::Exp>;
                    let mut rhs: Arc<DAE::Exp>;
                    let mut iterator: Arc<DAE::Exp>;
                    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
                    let mut attr: BackendDAE::EquationAttributes = <BackendDAE::EquationAttributes as ::std::default::Default>::default();
                    let mut similarEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut foldEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut eq = (*eq).clone();
                    let mut rest = (*rest).clone();
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(eq.clone()) {
                        Deref @ BackendDAE::Equation::EQUATION { attr: __pa0, source: __pa1, scalar: __pa2, exp: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    attr = __pa0.clone();
                    source = __pa1.clone();
                    rhs = __pa2.clone();
                    lhs = __pa3.clone();
                    let true = (ComponentReferenceBasics::crefEqualWithoutSubs(Expression::expCref(lhs.clone())?, Expression::expCref(rhs.clone())?)) else { bail!("pattern mismatch") };
                    (similarEqs, rest) = List::separate1OnTrue(classEqs.clone(), (std::sync::Arc::new(equationEqualNoCrefSubs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, Arc<BackendDAE::Equation>) -> Result<bool> + 'static>), eq.clone());
                    Expression::expCref(lhs.clone())?;
                    Expression::expCref(rhs.clone())?;
                    iterator = Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (literal!("i")).clone(), identType: DAE::T_INTEGER_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_INTEGER_DEFAULT().clone() });
                    eq = Arc::new(BackendDAE::Equation::FOR_EQUATION { iter: iterator.clone(), start: Arc::new(DAE::Exp::ICONST { integer: 1 }), stop: Arc::new(DAE::Exp::ICONST { integer: (similarEqs.clone().len() as i32) }), body: Arc::new(BackendDAE::Equation::EQUATION { exp: lhs.clone(), scalar: rhs.clone(), source: source.clone(), attr: attr.clone() }), source: source.clone(), attr: attr.clone() });
                    foldEqs = buildBackendDAEForEquations(rest.clone(), cons(eq.clone(), foldIn.clone()))?;
                    Ok(foldEqs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: eq, tail: rest } => {
                    let mut min: i32 = 0;
                    let mut max: i32 = 0;
                    let mut numCrefs: i32 = 0;
                    let mut lhs: Arc<DAE::Exp>;
                    let mut rhs: Arc<DAE::Exp>;
                    let mut iterator: Arc<DAE::Exp>;
                    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
                    let mut attr: BackendDAE::EquationAttributes = <BackendDAE::EquationAttributes as ::std::default::Default>::default();
                    let mut similarEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut foldEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut crefs2: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut crefMinMax: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32, i32)>> = metamodelica::nil();
                    let mut eq = (*eq).clone();
                    let mut rest = (*rest).clone();
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(eq.clone()) {
                        Deref @ BackendDAE::Equation::EQUATION { attr: __pa0, source: __pa1, scalar: __pa2, exp: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    attr = __pa0.clone();
                    source = __pa1.clone();
                    rhs = __pa2.clone();
                    lhs = __pa3.clone();
                    (similarEqs, rest) = List::separate1OnTrue(classEqs.clone(), (std::sync::Arc::new(equationEqualNoCrefSubs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, Arc<BackendDAE::Equation>) -> Result<bool> + 'static>), eq.clone());
                    crefs = BackendEquation::equationCrefs(eq.clone())?;
                    crefs2 = BackendEquation::equationCrefs((similarEqs.clone()).get(1)?)?;
                    (crefs2, crefs, _) = List::intersection1OnTrue(crefs.clone(), crefs2.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>))?;
                    numCrefs = (crefs.clone().len() as i32);
                    crefMinMax = List::thread3Map(crefs.clone().reverse(), List::fill(999999999, numCrefs.clone()), List::fill(0, numCrefs.clone()), std::sync::Arc::new(fnptr!(Util::make3Tuple, _, _, _)));
                    crefMinMax = List::fold1(similarEqs.clone(), (std::sync::Arc::new(getCrefIdcsForEquation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32, i32)>>) -> Result<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32, i32)>>> + 'static>), crefs2.clone(), crefMinMax.clone());
                    min = 1;
                    max = (similarEqs.clone().len() as i32);
                    iterator = Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (literal!("i")).clone(), identType: DAE::T_INTEGER_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_INTEGER_DEFAULT().clone() });
                    let (__pa4, __pa5) = ::match_deref::match_deref! { match &(BackendEquation::traverseExpsOfEquation(eq.clone(), (std::sync::Arc::new(setIteratorSubscriptCrefinEquation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32, i32)>>, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32, i32)>>, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>))> + 'static>), (crefMinMax.clone(), iterator.clone(), crefs2.clone()))?) {
                        (Deref @ BackendDAE::Equation::EQUATION { scalar: __pa4, exp: __pa5, .. }, _) => (__pa4.clone(), __pa5.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    rhs = __pa4.clone();
                    lhs = __pa5.clone();
                    eq = Arc::new(BackendDAE::Equation::FOR_EQUATION { iter: iterator.clone(), start: Arc::new(DAE::Exp::ICONST { integer: min.clone() }), stop: Arc::new(DAE::Exp::ICONST { integer: max.clone() }), body: Arc::new(BackendDAE::Equation::EQUATION { exp: lhs.clone(), scalar: rhs.clone(), source: source.clone(), attr: attr.clone() }), source: source.clone(), attr: attr.clone() });
                    foldEqs = buildBackendDAEForEquations(rest.clone(), cons(eq.clone(), foldIn.clone()))?;
                    Ok(foldEqs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(foldIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(foldOut)
}

fn getCrefIdcsForEquation(mut eq: Arc<BackendDAE::Equation>, mut constCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut crefMinMaxIn: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32, i32)>>) -> Result<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32, i32)>>> {
    let mut crefMinMaxOut: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32, i32)>> = metamodelica::nil();
    crefMinMaxOut = 'mc: {
        let __mc_input = (eq.clone(), crefMinMaxIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: _, .. }, crefMinMax) => {
                    let mut pos: i32 = 0;
                    let mut max: i32 = 0;
                    let mut min: i32 = 0;
                    let mut sub: i32 = 0;
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut refCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut refCrefMinMax: (Arc<DAE::ComponentRef>, i32, i32);
                    let mut eqCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut crefMinMax = (*crefMinMax).clone();
                    eqCrefs = BackendEquation::equationCrefs(eq.clone())?;
                    eqCrefs = List::filter1OnTrue(eqCrefs.clone(), (std::sync::Arc::new(fnptr!(ComponentReferenceBasics::crefNotInLst, Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<bool> + 'static>), constCrefs.clone());
                    for mut cref in &*eqCrefs.clone() {
                        let mut cref = cref.clone();
                        let __pa0 = ::match_deref::match_deref! { match &(ComponentReferenceBasics::crefSubs(cref.clone())?) {
                            Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: __pa0 } }, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        sub = __pa0.clone();
                        pos = 1;
                        for mut refCrefMinMax in &*crefMinMax.clone() {
                            let mut refCrefMinMax = refCrefMinMax.clone();
                            (refCref, min, max) = refCrefMinMax.clone();
                            if ComponentReferenceBasics::crefEqualWithoutSubs(refCref.clone(), cref.clone()) {
                                        max = intMax(max.clone(), sub.clone());
                                        min = intMin(min.clone(), sub.clone());
                                        crefMinMax = List::replaceAt((refCref.clone(), min.clone(), max.clone()), pos.clone(), crefMinMax.clone())?;
                            }
                            pos = pos.clone() + 1;
                        }
                    }
                    Ok(crefMinMax.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(crefMinMaxIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(crefMinMaxOut)
}

fn setIteratorSubscriptCrefinEquation(mut inExp: Arc<DAE::Exp>, mut tplIn: (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32, i32)>>, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32, i32)>>, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut tplOut: (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32, i32)>>, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>);
    (outExp, tplOut) = 'mc: {
        let __mc_input = (inExp.clone(), tplIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty, componentRef: cref }, (crefMinMax0, iterator, constCrefs)) => {
                    let mut min: i32 = 0;
                    let mut refCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut iterator1: Arc<DAE::Exp>;
                    let mut refCrefMinMax: (Arc<DAE::ComponentRef>, i32, i32);
                    let mut crefMinMax1: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32, i32)>> = metamodelica::nil();
                    let mut cref = (*cref).clone();
                    let true = (!(List::exist1(constCrefs.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), cref.clone()))) else { bail!("pattern mismatch") };
                    crefMinMax1 = metamodelica::nil();
                    for mut refCrefMinMax in &*crefMinMax0.clone() {
                        let mut refCrefMinMax = refCrefMinMax.clone();
                        (refCref, min, _) = refCrefMinMax.clone();
                        if ComponentReferenceBasics::crefEqualWithoutSubs(refCref.clone(), cref.clone()) {
                            (iterator1, _) = ExpressionSimplify::simplify(Arc::new(DAE::Exp::BINARY { exp1: iterator.clone(), operator: DAE::Operator::ADD { ty: DAE::T_INTEGER_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::ICONST { integer: min.clone() - 1 }) }))?;
                            cref = replaceFirstSubsInCref(cref.clone(), list![Arc::new(DAE::Subscript::INDEX { exp: iterator1.clone() })])?;
                        } else {
                            crefMinMax1 = cons(refCrefMinMax.clone(), crefMinMax1.clone());
                        }
                    }
                    Ok((Arc::new(DAE::Exp::CREF { componentRef: cref.clone(), ty: ty.clone() }), (crefMinMax1.clone(), iterator.clone(), constCrefs.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp2, operator: op, exp1 }, (crefMinMax0, iterator, constCrefs)) => {
                    let mut exp2 = (*exp2).clone();
                    let mut exp1 = (*exp1).clone();
                    let mut crefMinMax0 = (*crefMinMax0).clone();
                    let mut iterator = (*iterator).clone();
                    let mut constCrefs = (*constCrefs).clone();
                    let (__pa0, (__pa1, __pa2, __pa3)) = setIteratorSubscriptCrefinEquation(exp1.clone(), tplIn.clone())?;
                    exp1 = __pa0.clone();
                    crefMinMax0 = __pa1.clone();
                    iterator = __pa2.clone();
                    constCrefs = __pa3.clone();
                    let (__pa4, (__pa5, __pa6, __pa7)) = setIteratorSubscriptCrefinEquation(exp2.clone(), (crefMinMax0.clone(), iterator.clone(), constCrefs.clone()))?;
                    exp2 = __pa4.clone();
                    crefMinMax0 = __pa5.clone();
                    iterator = __pa6.clone();
                    constCrefs = __pa7.clone();
                    Ok((Arc::new(DAE::Exp::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() }), (crefMinMax0.clone(), iterator.clone(), constCrefs.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { exp: exp1, operator: op }, (crefMinMax0, iterator, constCrefs)) => {
                    let mut exp1 = (*exp1).clone();
                    let mut crefMinMax0 = (*crefMinMax0).clone();
                    let mut iterator = (*iterator).clone();
                    let mut constCrefs = (*constCrefs).clone();
                    let (__pa0, (__pa1, __pa2, __pa3)) = setIteratorSubscriptCrefinEquation(exp1.clone(), tplIn.clone())?;
                    exp1 = __pa0.clone();
                    crefMinMax0 = __pa1.clone();
                    iterator = __pa2.clone();
                    constCrefs = __pa3.clone();
                    Ok((Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: exp1.clone() }), (crefMinMax0.clone(), iterator.clone(), constCrefs.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { attr, expLst: eLst, path }, (crefMinMax0, iterator, constCrefs)) => {
                    let mut eLst = (*eLst).clone();
                    let mut crefMinMax0 = (*crefMinMax0).clone();
                    let mut iterator = (*iterator).clone();
                    let mut constCrefs = (*constCrefs).clone();
                    let (__pa0, (__pa1, __pa2, __pa3)) = List::mapFold(eLst.clone(), (std::sync::Arc::new(setIteratorSubscriptCrefinEquation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32, i32)>>, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32, i32)>>, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>))> + 'static>), tplIn.clone());
                    eLst = __pa0.clone();
                    crefMinMax0 = __pa1.clone();
                    iterator = __pa2.clone();
                    constCrefs = __pa3.clone();
                    Ok((Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: eLst.clone(), attr: attr.clone() }), (crefMinMax0.clone(), iterator.clone(), constCrefs.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), tplIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, tplOut))
}

fn getArrayVarCrefs(mut varIn: BackendDAE::Var, mut tplIn: (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<BackendDAE::Var>>)) -> Result<(Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<BackendDAE::Var>>)> {
    let mut tplOut: (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<BackendDAE::Var>>);
    tplOut = 'mc: {
        let __mc_input = (varIn.clone(), tplIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (BackendDAE::Var { varName: cref, .. }, (tplLst, arrVars)) => {
                    let mut idx: i32 = 0;
                    let mut crefHead: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut crefTailOpt: Option<Arc<DAE::ComponentRef>> = None;
                    let mut crefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut tpl: (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<BackendDAE::Var>>);
                    let mut tplLst = (*tplLst).clone();
                    let mut arrVars = (*arrVars).clone();
                    let true = (ComponentReference::isArrayElement(cref.clone())) else { bail!("pattern mismatch") };
                    (crefHead, idx, crefTailOpt) = ComponentReference::stripArrayCref(cref.clone())?;
                    if isSome(crefTailOpt.clone()) {
                        crefLst = list![Util::getOption(crefTailOpt.clone())?];
                    } else {
                        crefLst = metamodelica::nil();
                    }
                    (tplLst, arrVars) = addToArrayCrefLst(tplLst.clone(), varIn.clone(), (crefHead.clone(), idx.clone(), crefLst.clone()), metamodelica::nil(), arrVars.clone())?;
                    tpl = (tplLst.clone(), arrVars.clone());
                    Ok(tpl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(tplIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(tplOut)
}

fn addToArrayCrefLst(mut tplLstIn: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, mut varIn: BackendDAE::Var, mut tplRef: (Arc<DAE::ComponentRef>, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), mut tplLstFoldIn: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, mut varLstIn: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<(Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<BackendDAE::Var>>)> {
    let mut tplLstFoldOut: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>> = metamodelica::nil();
    let mut varLstOut: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    (tplLstFoldOut, varLstOut) = 'mc: {
        let __mc_input = (tplLstIn.clone(), tplRef.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (cref0, idx0, tailCrefs0), tail: rest }, (cref1, idx1, Deref @ metamodelica::List::Cons { head: crefTailRef, tail: Deref @ metamodelica::List::Nil })) => {
                    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut tplLst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>> = metamodelica::nil();
                    let mut tailCrefs0 = (*tailCrefs0).clone();
                    let true = (ComponentReferenceBasics::crefEqual(cref0.clone(), cref1.clone())?) else { bail!("pattern mismatch") };
                    if List::notMember(crefTailRef.clone(), tailCrefs0.clone()) {
                        tailCrefs0 = cons(crefTailRef.clone(), tailCrefs0.clone());
                        varLst = cons(varIn.clone(), varLstIn.clone());
                    } else {
                        varLst = varLstIn.clone();
                    }
                    tplLst = cons((cref0.clone(), intMax(idx0.clone(), idx1.clone()), tailCrefs0.clone()), rest.clone());
                    tplLst = List::append_reverse(tplLst.clone(), tplLstFoldIn.clone());
                    Ok((tplLst.clone(), varLst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (cref0, idx0, tailCrefs0), tail: rest }, (cref1, _, _)) => {
                    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut tplLst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>> = metamodelica::nil();
                    let false = (ComponentReferenceBasics::crefEqual(cref0.clone(), cref1.clone())?) else { bail!("pattern mismatch") };
                    (tplLst, varLst) = addToArrayCrefLst(rest.clone(), varIn.clone(), tplRef.clone(), cons((cref0.clone(), idx0.clone(), tailCrefs0.clone()), tplLstFoldIn.clone()), varLstIn.clone())?;
                    Ok((tplLst.clone(), varLst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, (cref1, idx1, tailCrefs1)) => {
                    let mut tplLst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>> = metamodelica::nil();
                    tplLst = cons((cref1.clone(), idx1.clone(), tailCrefs1.clone()), tplLstFoldIn.clone());
                    Ok((tplLst.clone(), cons(varIn.clone(), varLstIn.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((tplLstFoldOut, varLstOut))
}

fn getArrayVars(mut varIn: BackendDAE::Var, mut tplIn: (Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>)) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>)> {
    let mut tplOut: (Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>);
    tplOut = 'mc: {
        let __mc_input = (varIn.clone(), tplIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (BackendDAE::Var { varName: cref, .. }, (varLstIn, arrVarLstIn)) => {
                    let true = (ComponentReference::isArrayElement(cref.clone())) else { bail!("pattern mismatch") };
                    Ok((varLstIn.clone(), cons(varIn.clone(), arrVarLstIn.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (varLstIn, arrVarLstIn)) => {
                    Ok((cons(varIn.clone(), varLstIn.clone()), arrVarLstIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(tplOut)
}

fn dispatchLoopEquations(mut eqIn: Arc<BackendDAE::Equation>, mut arrayCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut tplIn: (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut tplOut: (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>);
    tplOut = (::match_deref::match_deref! { match &(tplIn.clone()) {
        (classEqs, mixEqs, nonArrEqs) => {
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut arrCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut nonArrCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut classEqs = (*classEqs).clone();
            let mut mixEqs = (*mixEqs).clone();
            let mut nonArrEqs = (*nonArrEqs).clone();
            crefs = BackendEquation::equationCrefs(eqIn.clone())?;
            (arrCrefs, nonArrCrefs) = List::separate1OnTrue(crefs.clone(), (std::sync::Arc::new(fnptr!(crefPartlyEqualToCrefs, Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<bool> + 'static>), arrayCrefs.clone());
            if nonArrCrefs.clone().is_empty() {
                classEqs = cons(eqIn.clone(), classEqs.clone());
            } else if arrCrefs.clone().is_empty() {
                nonArrEqs = cons(eqIn.clone(), nonArrEqs.clone());
            } else {
                mixEqs = cons(eqIn.clone(), mixEqs.clone());
            }
            (classEqs.clone(), mixEqs.clone(), nonArrEqs.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(tplOut)
}

fn crefPartlyEqualToCrefs(mut cref0: Arc<DAE::ComponentRef>, mut crefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> bool {
    let mut b: bool = false;
    b = List::exist1(crefLst.clone(), (std::sync::Arc::new(crefPartlyEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), cref0.clone());
    b
}

fn crefPartlyEqual(mut cref0: Arc<DAE::ComponentRef>, mut cref1: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut partlyEq: bool = false;
    partlyEq = 'mc: {
        let __mc_input = (cref0.clone(), cref1.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, Deref @ DAE::ComponentRef::CREF_IDENT { .. }) => {
                    Ok(var_field!((*cref0).ident, DAE::ComponentRef::CREF_IDENT).clone() == var_field!((*cref1).ident, DAE::ComponentRef::CREF_IDENT).clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cref01, .. }, Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cref11, .. }) => {
                    let mut b: bool = false;
                    if var_field!((*cref0).ident, DAE::ComponentRef::CREF_QUAL).clone() == var_field!((*cref1).ident, DAE::ComponentRef::CREF_QUAL).clone() {
                        b = crefPartlyEqual(cref01.clone(), cref11.clone())?;
                    } else {
                        b = false;
                    }
                    Ok(b.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { .. }, Deref @ DAE::ComponentRef::CREF_IDENT { .. }) => {
                    Ok(var_field!((*cref0).ident, DAE::ComponentRef::CREF_QUAL).clone() == var_field!((*cref1).ident, DAE::ComponentRef::CREF_IDENT).clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, Deref @ DAE::ComponentRef::CREF_QUAL { .. }) => {
                    Ok(var_field!((*cref0).ident, DAE::ComponentRef::CREF_IDENT).clone() == var_field!((*cref1).ident, DAE::ComponentRef::CREF_QUAL).clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(partlyEq)
}

pub fn reduceLoopExpressions(mut expIn: Arc<DAE::Exp>, mut maxSub: i32) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut expOut: Arc<DAE::Exp>;
    let mut notRemoved: bool = false;
    (expOut, notRemoved) = 'mc: {
        let __mc_input = expIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { componentRef: cref, .. } => {
                    let mut b: bool = false;
                    b = intLe(getIndexSubScript(listHead(ComponentReferenceBasics::crefSubs(cref.clone())?)?)?, maxSub.clone());
                    Ok((expIn.clone(), b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp2, operator: op, exp1 } => {
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    let mut exp: Arc<DAE::Exp>;
                    let mut exp2 = (*exp2).clone();
                    let mut exp1 = (*exp1).clone();
                    (exp1, b1) = reduceLoopExpressions(exp1.clone(), maxSub.clone())?;
                    (exp2, b2) = reduceLoopExpressions(exp2.clone(), maxSub.clone())?;
                    if b1.clone() && !(b2.clone()) {
                        exp = exp1.clone();
                    } else if b2.clone() && !(b1.clone()) {
                        exp = exp2.clone();
                    } else {
                        exp = Arc::new(DAE::Exp::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() });
                    }
                    Ok((exp.clone(), boolOr(b1.clone(), b2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::UNARY { exp, .. } => {
                    let mut b: bool = false;
                    let mut exp = (*exp).clone();
                    (exp, b) = reduceLoopExpressions(exp.clone(), maxSub.clone())?;
                    Ok((exp.clone(), b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((expIn.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((expOut, notRemoved))
}

pub fn insertSUMexp(mut expIn: Arc<DAE::Exp>, mut tplIn: (Arc<DAE::ComponentRef>, Arc<DAE::Exp>)) -> Result<(Arc<DAE::Exp>, (Arc<DAE::ComponentRef>, Arc<DAE::Exp>))> {
    let mut expOut: Arc<DAE::Exp>;
    let mut tplOut: (Arc<DAE::ComponentRef>, Arc<DAE::Exp>);
    (expOut, tplOut) = 'mc: {
        let __mc_input = (expIn.clone(), tplIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp2, operator: op, exp1 }, _) => {
                    let mut exp2 = (*exp2).clone();
                    let mut exp1 = (*exp1).clone();
                    (exp1, _) = insertSUMexp(exp1.clone(), tplIn.clone())?;
                    (exp2, _) = insertSUMexp(exp2.clone(), tplIn.clone())?;
                    Ok((Arc::new(DAE::Exp::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() }), tplIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { exp: exp1, operator: op }, _) => {
                    let mut exp1 = (*exp1).clone();
                    (exp1, _) = insertSUMexp(exp1.clone(), tplIn.clone())?;
                    Ok((Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: exp1.clone() }), tplIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, (cref0, repl)) => {
                    let true = (crefPartlyEqual(cref0.clone(), cref1.clone())?) else { bail!("pattern mismatch") };
                    Ok((repl.clone(), tplIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((expIn.clone(), tplIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((expOut, tplOut))
}

fn getIndexSubScript(mut sub: Arc<DAE::Subscript>) -> Result<i32> {
    let mut int: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: __pa0 } } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    int = __pa0.clone();
    Ok(int)
}

pub fn replaceFirstSubsInCref(mut crefIn: Arc<DAE::ComponentRef>, mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Arc<DAE::ComponentRef>> {
    let mut crefOut: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    crefOut = 'mc: {
        let __mc_input = crefIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cref, subscriptLst, identType, ident } => {
                    let mut cref = (*cref).clone();
                    let mut subscriptLst = (*subscriptLst).clone();
                    if List::hasOneElement(subscriptLst.clone()) {
                        subscriptLst = subs.clone();
                    }
                    cref = replaceFirstSubsInCref(cref.clone(), subs.clone())?;
                    Ok(Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (ident.clone()).clone(), identType: identType.clone(), subscriptLst: subscriptLst.clone(), componentRef: cref.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst, identType, ident } => {
                    let mut subscriptLst = (*subscriptLst).clone();
                    if List::hasOneElement(subscriptLst.clone()) {
                        subscriptLst = subs.clone();
                    }
                    Ok(Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (ident.clone()).clone(), identType: identType.clone(), subscriptLst: subscriptLst.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(crefIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(crefOut)
}

