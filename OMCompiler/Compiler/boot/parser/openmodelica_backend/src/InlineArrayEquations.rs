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
use crate::BackendDAEUtil;
use crate::BackendEquation;
use openmodelica_frontend::DAEUtil;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionSimplify;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Debug;
use openmodelica_util::Flags;
use openmodelica_util_datatypes_basic::List;

// =============================================================================
// inline arrayeqns stuff
//
// public functions:
//   - inlineArrayEqn
//   - getScalarArrayEqns
// =============================================================================
pub fn inlineArrayEqn(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    (outDAE, _) = BackendDAEUtil::mapEqSystemAndFold(inDAE.clone(), (std::sync::Arc::new(inlineArrayEqn1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool)> + 'static>), false)?;
    Ok(outDAE)
}

fn inlineArrayEqn1(mut inEqSystem: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut inOptimized: bool) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool)> {
    let mut outEqSystem: Arc<BackendDAE::EqSystem>;
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut outOptimized: bool = false;
    (outEqSystem, outOptimized) = 'mc: {
        let __mc_input = inEqSystem.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::EqSystem { orderedEqs, .. } => {
                    let mut eqnLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut orderedEqs = (*orderedEqs).clone();
                    eqnLst = BackendEquation::equationList(orderedEqs.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(getScalarArrayEqns(eqnLst.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    eqnLst = __pa0.clone();
                    orderedEqs = BackendEquation::listEquation(eqnLst.clone())?;
                    Ok((BackendDAEUtil::clearEqSyst(BackendDAEUtil::setEqSystEqs(inEqSystem.clone(), orderedEqs.clone()))?, true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inEqSystem.clone(), inOptimized.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outEqSystem, outShared, outOptimized))
}

pub fn getScalarArrayEqns(mut inEqnLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool)> {
    let mut outEqnLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut outFound: bool = false;
    (outEqnLst, outFound) = getScalarArrayEqns0(inEqnLst.clone(), metamodelica::nil(), false)?;
    Ok((outEqnLst, outFound))
}

fn getScalarArrayEqns0(mut inEqnLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inAccEqnLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inFound: bool) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool)> {
    let mut outEqnLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut outFound: bool = false;
    (outEqnLst, outFound) = (::match_deref::match_deref! { match &((inEqnLst.clone(), inAccEqnLst.clone(), inFound.clone())) {
        (Deref @ metamodelica::List::Nil, _, _) => {
            (inAccEqnLst.clone().reverse(), inFound.clone())
        },
        (Deref @ metamodelica::List::Cons { head: eqn, tail: eqns }, _, _) => {
            let mut eqns1: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut b: bool = false;
            (eqns1, b) = getScalarArrayEqns1(eqn.clone(), inAccEqnLst.clone())?;
            (eqns1, b) = getScalarArrayEqns0(eqns.clone(), eqns1.clone(), b.clone() || inFound.clone())?;
            (eqns1.clone(), b.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outEqnLst, outFound))
}

fn getScalarArrayEqns1(mut inEqn: Arc<BackendDAE::Equation>, mut inAccEqnLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool)> {
    let mut outEqnLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut outFound: bool = false;
    (outEqnLst, outFound) = 'mc: {
        let __mc_input = inEqn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::ARRAY_EQUATION { attr, source, right: rhs, left: lhs, .. } => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut e2: Arc<DAE::Exp>;
                    let mut ea1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut ea2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    if Expression::isArray(lhs.clone()) || Expression::isMatrix(lhs.clone()) {
                        ea1 = Expression::flattenArrayExpToList(lhs.clone())?;
                    } else {
                        (e1, _) = Expression::extendArrExp(lhs.clone(), false)?;
                        (e1, _) = ExpressionSimplify::simplify(e1.clone())?;
                        let true = (Expression::isArray(e1.clone()) || Expression::isMatrix(e1.clone())) else { bail!("pattern mismatch") };
                        ea1 = Expression::flattenArrayExpToList(e1.clone())?;
                    }
                    if Expression::isArray(rhs.clone()) || Expression::isMatrix(rhs.clone()) {
                        ea2 = Expression::flattenArrayExpToList(rhs.clone())?;
                    } else {
                        (e2, _) = Expression::extendArrExp(rhs.clone(), false)?;
                        (e2, _) = ExpressionSimplify::simplify(e2.clone())?;
                        let true = (Expression::isArray(e2.clone()) || Expression::isMatrix(e2.clone())) else { bail!("pattern mismatch") };
                        ea2 = Expression::flattenArrayExpToList(e2.clone())?;
                    }
                    (_, eqns) = List::threadFold3(ea1.clone(), ea2.clone(), (std::sync::Arc::new(generateScalarArrayEqns2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>, Arc<DAE::ElementSource>, BackendDAE::EquationAttributes, Arc<DAE::EquationExp>, (i32, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)) -> Result<(i32, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> + 'static>), source.clone(), attr.clone(), Arc::new(DAE::EquationExp::EQUALITY_EXPS { lhs: lhs.clone(), rhs: rhs.clone() }), (1, inAccEqnLst.clone()))?;
                    Ok((eqns.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::COMPLEX_EQUATION { attr, source, right: rhs, left: lhs, .. } => {
                    let mut ea1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut ea2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    ea1 = Expression::splitRecord(lhs.clone(), Expression::r#typeof(lhs.clone())?)?;
                    ea2 = Expression::splitRecord(rhs.clone(), Expression::r#typeof(rhs.clone())?)?;
                    (_, eqns) = List::threadFold3(ea1.clone(), ea2.clone(), (std::sync::Arc::new(generateScalarArrayEqns2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>, Arc<DAE::ElementSource>, BackendDAE::EquationAttributes, Arc<DAE::EquationExp>, (i32, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)) -> Result<(i32, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> + 'static>), source.clone(), attr.clone(), Arc::new(DAE::EquationExp::EQUALITY_EXPS { lhs: lhs.clone(), rhs: rhs.clone() }), (1, inAccEqnLst.clone()))?;
                    Ok((eqns.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((cons(inEqn.clone(), inAccEqnLst.clone()), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outEqnLst, outFound))
}

fn generateScalarArrayEqns2(mut inExp1: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>, mut inSource: Arc<DAE::ElementSource>, mut eqAttr: BackendDAE::EquationAttributes, mut eqExp: Arc<DAE::EquationExp>, mut iEqns: (i32, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)) -> Result<(i32, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut oEqns: (i32, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>);
    oEqns = 'mc: {
        let __mc_input = iEqns.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (i, eqns) => {
                    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut size: i32 = 0;
                    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
                    tp = Expression::r#typeof(inExp1.clone())?;
                    let true = (DAEUtil::expTypeComplex(tp.clone())) else { bail!("pattern mismatch") };
                    size = Expression::sizeOf(tp.clone())?;
                    source = ElementSource::addSymbolicTransformation(inSource.clone(), Arc::new(DAE::SymbolicOperation::OP_SCALARIZE { before: eqExp.clone(), index: i.clone(), after: Arc::new(DAE::EquationExp::EQUALITY_EXPS { lhs: inExp1.clone(), rhs: inExp2.clone() }) }))?;
                    Ok((i.clone() + 1, cons(Arc::new(BackendDAE::Equation::COMPLEX_EQUATION { size: size.clone(), left: inExp1.clone(), right: inExp2.clone(), source: source.clone(), attr: eqAttr.clone() }), eqns.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (i, eqns) => {
                    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut recordSize: Option<i32> = None;
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut ds: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
                    tp = Expression::r#typeof(inExp1.clone())?;
                    let true = (DAEUtil::expTypeArray(tp.clone())) else { bail!("pattern mismatch") };
                    dims = Expression::arrayDimension(tp.clone());
                    tp = DAEUtil::expTypeElementType(tp.clone());
                    if DAEUtil::expTypeComplex(tp.clone()) {
                        recordSize = Some(Expression::sizeOf(tp.clone())?);
                    } else {
                        recordSize = None;
                    }
                    ds = Expression::dimensionsSizes(dims.clone());
                    source = ElementSource::addSymbolicTransformation(inSource.clone(), Arc::new(DAE::SymbolicOperation::OP_SCALARIZE { before: eqExp.clone(), index: i.clone(), after: Arc::new(DAE::EquationExp::EQUALITY_EXPS { lhs: inExp1.clone(), rhs: inExp2.clone() }) }))?;
                    Ok((i.clone() + 1, cons(Arc::new(BackendDAE::Equation::ARRAY_EQUATION { dimSize: ds.clone(), left: inExp1.clone(), right: inExp2.clone(), source: source.clone(), attr: eqAttr.clone(), recordSize: recordSize.clone() }), eqns.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (i, eqns) => {
                    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
                    tp = Expression::r#typeof(inExp1.clone())?;
                    b1 = DAEUtil::expTypeComplex(tp.clone());
                    b2 = DAEUtil::expTypeArray(tp.clone());
                    let false = (b1.clone() || b2.clone()) else { bail!("pattern mismatch") };
                    source = ElementSource::addSymbolicTransformation(inSource.clone(), Arc::new(DAE::SymbolicOperation::OP_SCALARIZE { before: eqExp.clone(), index: i.clone(), after: Arc::new(DAE::EquationExp::EQUALITY_EXPS { lhs: inExp1.clone(), rhs: inExp2.clone() }) }))?;
                    Ok((i.clone() + 1, cons(Arc::new(BackendDAE::Equation::EQUATION { exp: inExp1.clone(), scalar: inExp2.clone(), source: source.clone(), attr: eqAttr.clone() }), eqns.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- InlineArrayEquations.generateScalarArrayEqns2 failed on: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp1.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp2.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oEqns)
}

