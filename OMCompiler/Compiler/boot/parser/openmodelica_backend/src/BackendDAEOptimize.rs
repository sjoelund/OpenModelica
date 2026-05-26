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

use crate::AdjacencyMatrix;
use crate::AvlSetInt;
use crate::BackendDAE;
use crate::BackendDAEEXT;
use crate::BackendDAETransform;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendInline;
use crate::BackendVarTransform;
use crate::BackendVariable;
use crate::Differentiate;
use crate::ExpressionSolve;
use crate::HpcOmTaskGraph;
use crate::Matching;
use crate::RewriteRules;
use crate::SynchronousFeatures;
use crate::Tearing;
use openmodelica_ast::Absyn;
use openmodelica_frontend::Algorithm;
use openmodelica_frontend::CheckModel;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::DAEDump;
use openmodelica_frontend::DAEUtil;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionDump;
use openmodelica_frontend::ExpressionSimplify;
use openmodelica_frontend::FCore;
use openmodelica_frontend::HashSet;
use openmodelica_frontend::HashTable2;
use openmodelica_frontend::HashTableExpToIndex;
use openmodelica_frontend::Inline;
use openmodelica_frontend::Types;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::BaseHashSet;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::GCExt;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

pub fn simplifyAllExpressions(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = inDAE.clone();
    let mut removedEqsList: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared>;
    let _ = BackendDAEUtil::traverseBackendDAEExpsNoCopyWithUpdate(outDAE.clone(), Arc::new(ExpressionSimplify::simplify1TraverseHelper), 0)?;
    shared = outDAE.shared.clone();
    for mut eq in &*BackendEquation::equationList(shared.removedEqs.clone()) {
        let mut eq = eq.clone();
        removedEqsList = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ BackendDAE::Equation::ALGORITHM { alg: Deref @ DAE::Algorithm { statementLst: Deref @ metamodelica::List::Nil }, .. } => removedEqsList.clone(),
        _ => cons(eq.clone(), removedEqsList.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    assign_field!(shared.removedEqs = BackendEquation::listEquation(metamodelica::Dangerous::listReverseInPlace(removedEqsList.clone()))?);
    assign_field!(outDAE.shared = shared.clone());
    Ok(outDAE)
}

// =============================================================================
// simplifyInStream
//
// OM introduces $OMC$PositiveMax which can simplified using min or max attribute
// see Modelica spec for inStream
// author: Vitalij Ruge
// see. #3885, 4441, 5104
// =============================================================================
pub fn simplifyInStream(mut dae: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut dae: Arc<BackendDAE::BackendDAE> = dae;
    let mut shared: Arc<BackendDAE::Shared> = dae.shared.clone();
    let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = dae.eqs.clone();
    let mut vars: Arc<metamodelica::List<BackendDAE::Variables>> = {
        let mut __acc: Arc<metamodelica::List<BackendDAE::Variables>> = metamodelica::nil();
        for mut eq in (eqs.clone()).into_iter().cloned() {
            let __x = eq.orderedVars.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    vars = cons(shared.globalKnownVars.clone(), vars.clone());
    vars = cons(shared.localKnownVars.clone(), vars.clone());
    let _ = BackendDAEUtil::traverseBackendDAEExpsNoCopyWithUpdate(dae.clone(), Arc::new(simplifyInStreamWork), vars.clone())?;
    Ok(dae)
}

fn simplifyInStreamWork(mut inExp: Arc<DAE::Exp>, mut inVars: Arc<metamodelica::List<BackendDAE::Variables>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<BackendDAE::Variables>>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outVars: Arc<metamodelica::List<BackendDAE::Variables>> = inVars.clone();
    (outExp, _) = Expression::traverseExpBottomUp(inExp.clone(), Arc::new(simplifyInStreamWork2), outVars.clone())?;
    if !(ExpressionBasics::expEqual(outExp.clone(), inExp.clone())?) {
        (outExp, _) = ExpressionSimplify::simplify(outExp.clone())?;
    }
    Ok((outExp, outVars))
}

fn simplifyInStreamWork2(mut inExp: Arc<DAE::Exp>, mut inVars: Arc<metamodelica::List<BackendDAE::Variables>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<BackendDAE::Variables>>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outVars: Arc<metamodelica::List<BackendDAE::Variables>> = inVars.clone();
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Cons { head: expr, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "$OMC$PositiveMax" }, .. } => {
            let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut ret: Arc<DAE::Exp>;
            let mut eMin: Option<Arc<DAE::Exp>> = None;
            let mut eMax: Option<Arc<DAE::Exp>> = None;
            (eMin, eMax) = simplifyInStreamGetMinMaxAttributes(cr.clone(), outVars.clone());
            tp = ComponentReference::crefTypeFull(cr.clone())?;
            ret = if (Util::applyOptionOrDefault(eMax.clone(), Arc::new(Expression::isNegativeOrZero), false)) {Expression::createZeroExpression(tp.clone())?} else if (Util::applyOptionOrDefault(eMin.clone(), Arc::new({ let __pe_b1 = expr.clone(); move |__pe_a0| Ok(Expression::isGreaterOrEqual(__pe_a0, __pe_b1.clone())) }), false)) {e.clone()} else {Expression::makePureBuiltinCall((literal!("max")).clone(), list![e.clone(), expr.clone()], tp.clone())};
            ret.clone()
        },
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp }, exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. } }, tail: Deref @ metamodelica::List::Cons { head: expr, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "$OMC$PositiveMax" }, .. } => {
            let mut ret: Arc<DAE::Exp>;
            let mut eMin: Option<Arc<DAE::Exp>> = None;
            let mut eMax: Option<Arc<DAE::Exp>> = None;
            (eMin, eMax) = simplifyInStreamGetMinMaxAttributes(cr.clone(), outVars.clone());
            ret = if (Util::applyOptionOrDefault(eMin.clone(), Arc::new(Expression::isPositiveOrZero), false)) {Expression::createZeroExpression(tp.clone())?} else if (Util::applyOptionOrDefault(eMax.clone(), Arc::new({ let __pe_b0 = Expression::negate(expr.clone())?; move |__pe_a1| Ok(Expression::isGreaterOrEqual(__pe_b0.clone(), __pe_a1)) }), false)) {e.clone()} else {Expression::makePureBuiltinCall((literal!("max")).clone(), list![e.clone(), expr.clone()], tp.clone())};
            ret.clone()
        },
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Cons { head: expr, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "$OMC$PositiveMax" }, .. } => {
            Expression::makePureBuiltinCall((literal!("max")).clone(), list![e.clone(), expr.clone()], Expression::r#typeof(e.clone())?)
        },
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Cons { head: expr, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "$OMC$inStreamDiv" }, .. } => {
            let mut ret: Arc<DAE::Exp>;
            let mut e = (*e).clone();
            (e, _) = ExpressionSimplify::simplify(e.clone())?;
            ret = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Exp::BINARY { exp1: a, operator: DAE::Operator::DIV { .. }, exp2: b } if (Expression::isZero(a.clone()) && Expression::isZero(b.clone())) => {
            expr.clone()
        },
        _ => {
            e.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            ret.clone()
        },
        _ => {
            inExp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outVars))
}

fn simplifyInStreamGetMinMaxAttributes(mut cr: Arc<DAE::ComponentRef>, mut inVars: Arc<metamodelica::List<BackendDAE::Variables>>) -> (Option<Arc<DAE::Exp>>, Option<Arc<DAE::Exp>>) {
    let mut outMin: Option<Arc<DAE::Exp>> = None;
    let mut outMax: Option<Arc<DAE::Exp>> = None;
    let mut v: BackendDAE::Var;
    for mut vars in &*inVars.clone() {
        let mut vars = vars.clone();
        if '__try0: {
            (v, _) = unwrap_break_err!(BackendVariable::getVarSingle(cr.clone(), vars.clone()), '__try0);
            (outMin, outMax) = BackendVariable::getMinMaxAttribute(v.clone());
            break;
            Ok::<(), anyhow::Error>(())
        }.is_err() {
        }
    }
    (outMin, outMax)
}

// =============================================================================
// simplify time independent function calls
//
// public functions:
//   - simplifyTimeIndepFuncCalls
// =============================================================================
pub fn simplifyTimeIndepFuncCalls(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    (outDAE, _) = BackendDAEUtil::mapEqSystemAndFold(inDAE.clone(), Arc::new(simplifyTimeIndepFuncCalls0), false)?;
    outDAE = simplifyTimeIndepFuncCallsShared(outDAE.clone())?;
    Ok(outDAE)
}

fn simplifyTimeIndepFuncCalls0(mut isyst: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut inChanged: bool) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool)> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut outShared: Arc<BackendDAE::Shared>;
    let mut outChanged: bool = false;
    (osyst, outShared, outChanged) = 'mc: {
        let __mc_input = (isyst.clone(), inShared.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (syst, shared) => {
                    let (_, (_, _, true)) = (BackendDAEUtil::traverseBackendDAEExpsEqns(syst.orderedEqs.clone(), Arc::new(Expression::traverseSubexpressionsHelper), (traverserExpsimplifyTimeIndepFuncCalls, (shared.globalKnownVars.clone(), shared.aliasVars.clone(), false)))?) else { bail!("pattern mismatch") };
                    let (_, (_, _, true)) = (BackendDAEUtil::traverseBackendDAEExpsEqns(syst.removedEqs.clone(), Arc::new(Expression::traverseSubexpressionsHelper), (traverserExpsimplifyTimeIndepFuncCalls, (shared.globalKnownVars.clone(), shared.aliasVars.clone(), false)))?) else { bail!("pattern mismatch") };
                    Ok((isyst.clone(), inShared.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((isyst.clone(), inShared.clone(), inChanged.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((osyst, outShared, outChanged))
}

fn traverserExpsimplifyTimeIndepFuncCalls(mut inExp: Arc<DAE::Exp>, mut inTpl: (BackendDAE::Variables, BackendDAE::Variables, bool)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, bool))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (BackendDAE::Variables, BackendDAE::Variables, bool);
    (outExp, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { ty: tp, componentRef: cr }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, (globalKnownVars, aliasvars, _)) => {
                    let mut zero: Arc<DAE::Exp>;
                    let mut var: BackendDAE::Var;
                    (var, _) = BackendVariable::getVarSingle(cr.clone(), globalKnownVars.clone())?;
                    let false = (BackendVariable::isVarOnTopLevelAndInput(var.clone())) else { bail!("pattern mismatch") };
                    (zero, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
                    Ok((zero.clone(), (globalKnownVars.clone(), aliasvars.clone(), true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: idn }, .. }, (globalKnownVars, aliasvars, _)) => {
                    if !((idn.clone() == literal!("pre") || idn.clone() == literal!("previous"))) { bail!("guard") }
                    let mut var: BackendDAE::Var;
                    (var, _) = BackendVariable::getVarSingle(cr.clone(), globalKnownVars.clone())?;
                    let false = (BackendVariable::isInput(var.clone())) else { bail!("pattern mismatch") };
                    Ok((e.clone(), (globalKnownVars.clone(), aliasvars.clone(), true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e @ Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. }, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: idn }, .. }, (globalKnownVars, aliasvars, _)) => {
                    if !((idn.clone() == literal!("pre") || idn.clone() == literal!("previous"))) { bail!("guard") }
                    Ok((e.clone(), (globalKnownVars.clone(), aliasvars.clone(), true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. }, .. } }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: idn }, .. }, (globalKnownVars, aliasvars, _)) => {
                    if !((idn.clone() == literal!("pre") || idn.clone() == literal!("previous"))) { bail!("guard") }
                    Ok((e.clone(), (globalKnownVars.clone(), aliasvars.clone(), true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { ty: tp, componentRef: cr }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: idn } }, (globalKnownVars, aliasvars, _)) => {
                    if !((idn.clone() == literal!("pre") || idn.clone() == literal!("previous"))) { bail!("guard") }
                    let mut e: Arc<DAE::Exp>;
                    let mut var: BackendDAE::Var;
                    let mut negate: bool = false;
                    let mut cr = (*cr).clone();
                    (var, _) = BackendVariable::getVarSingle(cr.clone(), aliasvars.clone())?;
                    (cr, negate) = BackendVariable::getAlias(var.clone())?;
                    e = Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: tp.clone() });
                    e = if (negate.clone()) {Expression::negate(e.clone())?} else {e.clone()};
                    (e, _) = ExpressionSimplify::simplify(Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (idn.clone()).clone() }), expLst: list![e.clone()], attr: attr.clone() }))?;
                    (e, _) = Expression::traverseExpBottomUp(e.clone(), Arc::new(traverserExpsimplifyTimeIndepFuncCalls), (globalKnownVars.clone(), aliasvars.clone(), false))?;
                    Ok((e.clone(), (globalKnownVars.clone(), aliasvars.clone(), true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { ty: tp, componentRef: cr }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: idn }, .. }, (globalKnownVars, aliasvars, _)) => {
                    if !((idn.clone() == literal!("change") || idn.clone() == literal!("edge"))) { bail!("guard") }
                    let mut zero: Arc<DAE::Exp>;
                    ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), globalKnownVars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: _, tail: _ }, _) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    zero = Expression::arrayFill(Expression::arrayDimension(tp.clone()), Arc::new(DAE::Exp::BCONST { bool: false }))?;
                    Ok((zero.clone(), (globalKnownVars.clone(), aliasvars.clone(), true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { ty: tp, componentRef: cr }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: idn }, .. }, (globalKnownVars, aliasvars, _)) => {
                    if !((idn.clone() == literal!("change") || idn.clone() == literal!("edge"))) { bail!("guard") }
                    let mut zero: Arc<DAE::Exp>;
                    ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), aliasvars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: _, tail: _ }, _) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    zero = Expression::arrayFill(Expression::arrayDimension(tp.clone()), Arc::new(DAE::Exp::BCONST { bool: false }))?;
                    Ok((zero.clone(), (globalKnownVars.clone(), aliasvars.clone(), true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. }, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: idn }, .. }, (globalKnownVars, aliasvars, _)) => {
                    if !((idn.clone() == literal!("change") || idn.clone() == literal!("edge"))) { bail!("guard") }
                    Ok((Arc::new(DAE::Exp::BCONST { bool: false }), (globalKnownVars.clone(), aliasvars.clone(), true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { ty: tp, componentRef: cr }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: idn } }, (globalKnownVars, aliasvars, _)) => {
                    if !((idn.clone() == literal!("change") || idn.clone() == literal!("edge"))) { bail!("guard") }
                    let mut e: Arc<DAE::Exp>;
                    let mut var: BackendDAE::Var;
                    let mut negate: bool = false;
                    let mut cr = (*cr).clone();
                    (var, _) = BackendVariable::getVarSingle(cr.clone(), aliasvars.clone())?;
                    (cr, negate) = BackendVariable::getAlias(var.clone())?;
                    e = Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: tp.clone() });
                    e = if (negate.clone()) {Expression::negate(e.clone())?} else {e.clone()};
                    (e, _) = ExpressionSimplify::simplify(Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (idn.clone()).clone() }), expLst: list![e.clone()], attr: attr.clone() }))?;
                    (e, _) = Expression::traverseExpBottomUp(e.clone(), Arc::new(traverserExpsimplifyTimeIndepFuncCalls), (globalKnownVars.clone(), aliasvars.clone(), false))?;
                    Ok((e.clone(), (globalKnownVars.clone(), aliasvars.clone(), true)))
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

fn simplifyTimeIndepFuncCallsShared(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    let mut shared: Arc<BackendDAE::Shared>;
    shared = inDAE.shared.clone();
    BackendDAEUtil::traverseBackendDAEExpsVarsWithUpdate(shared.globalKnownVars.clone(), Arc::new(Expression::traverseSubexpressionsHelper), (traverserExpsimplifyTimeIndepFuncCalls, (shared.globalKnownVars.clone(), shared.aliasVars.clone(), false)))?;
    BackendDAEUtil::traverseBackendDAEExpsEqns(shared.initialEqs.clone(), Arc::new(Expression::traverseSubexpressionsHelper), (traverserExpsimplifyTimeIndepFuncCalls, (shared.globalKnownVars.clone(), shared.aliasVars.clone(), false)))?;
    BackendDAEUtil::traverseBackendDAEExpsEqns(shared.removedEqs.clone(), Arc::new(Expression::traverseSubexpressionsHelper), (traverserExpsimplifyTimeIndepFuncCalls, (shared.globalKnownVars.clone(), shared.aliasVars.clone(), false)))?;
    let (shared.eventInfo, _) = traverseEventInfoExps(shared.eventInfo.clone(), Arc::new(Expression::traverseSubexpressionsHelper), (traverserExpsimplifyTimeIndepFuncCalls, (shared.globalKnownVars.clone(), shared.aliasVars.clone(), false)))?;
    outDAE = BackendDAE::DAE(inDAE.eqs.clone(), shared.clone())?;
    Ok(outDAE)
}

fn traverseEventInfoExps<T: Clone + 'static>(mut eventInfo: BackendDAE::EventInfo, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, T)> + 'static>, mut arg: T) -> Result<(BackendDAE::EventInfo, T)> {
    pub type FuncExpType<T: Clone> = fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, T)>;

    let mut eventInfo: BackendDAE::EventInfo = eventInfo;
    let mut arg: T = arg;
    arg = DoubleEnded::mapFoldNoCopy(eventInfo.zeroCrossings.zc.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static> = func.clone(); move |__pe_a0, __pe_a2| traverseZeroCrossingExps(__pe_a0, __pe_b1.clone(), __pe_a2) }), arg.clone())?;
    arg = DoubleEnded::mapFoldNoCopy(eventInfo.samples.zc.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static> = func.clone(); move |__pe_a0, __pe_a2| traverseZeroCrossingExps(__pe_a0, __pe_b1.clone(), __pe_a2) }), arg.clone())?;
    arg = DoubleEnded::mapFoldNoCopy(eventInfo.relations.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static> = func.clone(); move |__pe_a0, __pe_a2| traverseZeroCrossingExps(__pe_a0, __pe_b1.clone(), __pe_a2) }), arg.clone())?;
    Ok((eventInfo, arg))
}

fn traverseZeroCrossingExps<T: Clone + 'static>(mut zc: BackendDAE::ZeroCrossing, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, T)> + 'static>, mut arg: T) -> Result<(BackendDAE::ZeroCrossing, T)> {
    pub type FuncExpType<T: Clone> = fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, T)>;

    let mut zc: BackendDAE::ZeroCrossing = zc;
    let mut arg: T = arg;
    let mut relation: Arc<DAE::Exp>;
    (relation, arg) = Expression::traverseExpBottomUp(zc.relation_.clone(), func.clone(), arg.clone())?;
    if !(referenceEq(&relation.clone(),&zc.relation_.clone())) {
        zc.relation_ = relation.clone();
    }
    Ok((zc, arg))
}

fn toplevelInputOrUnfixed(mut inVar: BackendDAE::Var) -> bool {
    let mut b: bool = false;
    b = BackendVariable::isVarOnTopLevelAndInput(inVar.clone()) || BackendVariable::isParam(inVar.clone()) && !(BackendVariable::varFixed(inVar.clone()));
    b
}

fn traversingTimeEqnsFinder(mut inExp: Arc<DAE::Exp>, mut inTpl: (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool)) -> Result<(Arc<DAE::Exp>, bool, (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut outTpl: (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool);
    (outExp, cont, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::List::Nil, ident: Deref @ "time", .. }, ty: _ }, (_, vars, globalKnownVars, b1, b2)) => {
                    Ok((e.clone(), false, (true, vars.clone(), globalKnownVars.clone(), b1.clone(), b2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ }, (_, vars, globalKnownVars, b1, b2)) => {
                    let mut vlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), globalKnownVars.clone())?) {
                        (__pa0, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    vlst = __pa0.clone();
                    let false = (List::none(vlst.clone(), Arc::new(fnptr!(toplevelInputOrUnfixed, BackendDAE::Var)))) else { bail!("pattern mismatch") };
                    Ok((e.clone(), false, (true, vars.clone(), globalKnownVars.clone(), b1.clone(), b2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, .. }, (_, vars, globalKnownVars, b1, b2)) => {
                    Ok((e.clone(), false, (true, vars.clone(), globalKnownVars.clone(), b1.clone(), b2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, (_, vars, globalKnownVars, b1, b2)) => {
                    Ok((e.clone(), false, (true, vars.clone(), globalKnownVars.clone(), b1.clone(), b2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. }, (_, vars, globalKnownVars, b1, b2)) => {
                    Ok((e.clone(), false, (true, vars.clone(), globalKnownVars.clone(), b1.clone(), b2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "change" }, .. }, (_, vars, globalKnownVars, b1, b2)) => {
                    Ok((e.clone(), false, (true, vars.clone(), globalKnownVars.clone(), b1.clone(), b2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "edge" }, .. }, (_, vars, globalKnownVars, b1, b2)) => {
                    Ok((e.clone(), false, (true, vars.clone(), globalKnownVars.clone(), b1.clone(), b2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ }, (_, vars, globalKnownVars, true, b2)) => {
                    let mut var: BackendDAE::Var;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), globalKnownVars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: _ }, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    var = __pa0.clone();
                    let DAE::INPUT { .. } = (BackendVariable::getVarDirection(var.clone())) else { bail!("pattern mismatch") };
                    Ok((e.clone(), false, (true, vars.clone(), globalKnownVars.clone(), true, b2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ }, (_, vars, globalKnownVars, b1, true)) => {
                    ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: _, tail: _ }, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok((e.clone(), false, (true, vars.clone(), globalKnownVars.clone(), b1.clone(), true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, (b, _, _, _, _)) => {
                    Ok((e.clone(), !(b.clone()), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, cont, outTpl))
}

pub fn countSimpleEquations(mut inDlow: Arc<BackendDAE::BackendDAE>, mut inM: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<i32> {
    let mut outSimpleEqns: i32 = 0;
    outSimpleEqns = (::match_deref::match_deref! { match &((inDlow.clone(), inM.clone())) {
        (dlow, _) => {
            let mut n: i32 = 0;
            let (_, (_, __pa0)) = AdjacencyMatrix::traverseAdjacencyMatrix(inM.clone(), Arc::new(countSimpleEquationsFinder), (dlow.clone(), 0))?;
            n = __pa0.clone();
            n.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outSimpleEqns)
}

fn countSimpleEquationsFinder(mut elem: Arc<metamodelica::List<i32>>, mut pos: i32, mut inTpl: (Arc<BackendDAE::BackendDAE>, i32)) -> Result<(Arc<metamodelica::List<i32>>, (Arc<BackendDAE::BackendDAE>, i32))> {
    let mut outList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut outTpl: (Arc<BackendDAE::BackendDAE>, i32);
    (outList, outTpl) = 'mc: {
        let __mc_input = inTpl.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (dae @ Deref @ DAE { UNIQUEIO: metamodelica::List::Cons { head: syst, tail: Deref @ metamodelica::List::Nil }, derivativeNamePrefix: shared, .. }, n) => {
                    let mut l: i32 = 0;
                    let mut n_1: i32 = 0;
                    l = (elem.clone().len() as i32);
                    let true = (intLt(l.clone(), 3)) else { bail!("pattern mismatch") };
                    let true = (intGt(l.clone(), 0)) else { bail!("pattern mismatch") };
                    countsimpleEquation(elem.clone(), l.clone(), pos.clone(), syst.clone(), shared.clone())?;
                    n_1 = n.clone() + 1;
                    Ok((metamodelica::nil(), (dae.clone(), n_1.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((metamodelica::nil(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outList, outTpl))
}

fn countsimpleEquation(mut elem: Arc<metamodelica::List<i32>>, mut length: i32, mut pos: i32, mut syst: Arc<BackendDAE::EqSystem>, mut shared: Arc<BackendDAE::Shared>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (elem.clone(), shared.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: i, tail: Deref @ metamodelica::List::Nil }, Deref @ BackendDAE::Shared { backendDAEType: BackendDAE::BackendDAEType::JACOBIAN, .. }) => {
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut cre: Arc<DAE::Exp>;
                    let mut e1: Arc<DAE::Exp>;
                    let mut e2: Arc<DAE::Exp>;
                    let mut vars: BackendDAE::Variables;
                    let mut globalKnownVars: BackendDAE::Variables;
                    let mut var: BackendDAE::Var;
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    vars = BackendVariable::daeVars(syst.clone());
                    var = BackendVariable::getVarAt(vars.clone(), intAbs(i.clone()))?;
                    let false = (BackendVariable::isStateorStateDerVar(var.clone())) else { bail!("pattern mismatch") };
                    eqns = BackendEquation::getEqnsFromEqSystem(syst.clone());
                    eqn = BackendEquation::get(eqns.clone(), pos.clone());
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(eqn.clone()) {
                        Deref @ BackendDAE::Equation::EQUATION { scalar: __pa0, exp: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    e2 = __pa0.clone();
                    e1 = __pa1.clone();
                    globalKnownVars = BackendVariable::daeGlobalKnownVars(shared.clone());
                    ::match_deref::match_deref! { match &(Expression::traverseExpTopDown(e1.clone(), Arc::new(traversingTimeEqnsFinder), (false, vars.clone(), globalKnownVars.clone(), true, false))?) {
                        (_, (false, _, _, _, _)) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    ::match_deref::match_deref! { match &(Expression::traverseExpTopDown(e2.clone(), Arc::new(traversingTimeEqnsFinder), (false, vars.clone(), globalKnownVars.clone(), true, false))?) {
                        (_, (false, _, _, _, _)) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    cr = BackendVariable::varCref(var.clone())?;
                    cre = Expression::crefExp(cr.clone())?;
                    let (_, metamodelica::List::Nil) = (ExpressionSolve::solve(e1.clone(), e2.clone(), cre.clone(), None)?) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: i, tail: Deref @ metamodelica::List::Nil }, _) => {
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut cre: Arc<DAE::Exp>;
                    let mut e1: Arc<DAE::Exp>;
                    let mut e2: Arc<DAE::Exp>;
                    let mut vars: BackendDAE::Variables;
                    let mut globalKnownVars: BackendDAE::Variables;
                    let mut var: BackendDAE::Var;
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    vars = BackendVariable::daeVars(syst.clone());
                    var = BackendVariable::getVarAt(vars.clone(), intAbs(i.clone()))?;
                    let false = (BackendVariable::isStateorStateDerVar(var.clone())) else { bail!("pattern mismatch") };
                    eqns = BackendEquation::getEqnsFromEqSystem(syst.clone());
                    eqn = BackendEquation::get(eqns.clone(), pos.clone());
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(eqn.clone()) {
                        Deref @ BackendDAE::Equation::EQUATION { scalar: __pa0, exp: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    e2 = __pa0.clone();
                    e1 = __pa1.clone();
                    globalKnownVars = BackendVariable::daeGlobalKnownVars(shared.clone());
                    ::match_deref::match_deref! { match &(Expression::traverseExpTopDown(e1.clone(), Arc::new(traversingTimeEqnsFinder), (false, vars.clone(), globalKnownVars.clone(), false, false))?) {
                        (_, (false, _, _, _, _)) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    ::match_deref::match_deref! { match &(Expression::traverseExpTopDown(e2.clone(), Arc::new(traversingTimeEqnsFinder), (false, vars.clone(), globalKnownVars.clone(), false, false))?) {
                        (_, (false, _, _, _, _)) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    cr = BackendVariable::varCref(var.clone())?;
                    cre = Expression::crefExp(cr.clone())?;
                    let (_, metamodelica::List::Nil) = (ExpressionSolve::solve(e1.clone(), e2.clone(), cre.clone(), None)?) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, _) => {
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut vars: BackendDAE::Variables;
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    eqns = BackendEquation::getEqnsFromEqSystem(syst.clone());
                    eqn = BackendEquation::get(eqns.clone(), pos.clone());
                    (cr, _, _, _, _) = BackendEquation::derivativeEquation(eqn.clone())?;
                    vars = BackendVariable::daeVars(syst.clone());
                    ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: _, tail: _ }, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, _) => {
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    eqns = BackendEquation::getEqnsFromEqSystem(syst.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(BackendEquation::get(eqns.clone(), pos.clone())) {
                        __pa0 @ Deref @ BackendDAE::Equation::EQUATION { .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    eqn = __pa0.clone();
                    let _ = BackendEquation::aliasEquation(eqn.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

// =============================================================================
// remove parameters stuff
//
// =============================================================================
pub fn removeParameters(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    outDAE = (::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ DAE { UNIQUEIO: systs, derivativeNamePrefix: shared @ BackendDAE::Shared { globalKnownVars, .. }, .. } => {
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut systs = (*systs).clone();
            let mut shared = (*shared).clone();
            let mut globalKnownVars = (*globalKnownVars).clone();
            repl = BackendVarTransform::emptyReplacements();
            (repl, _) = BackendVariable::traverseBackendDAEVars(globalKnownVars.clone(), Arc::new(removeParametersFinder), (repl.clone(), globalKnownVars.clone()))?;
            (globalKnownVars, repl) = replaceFinalVars(1, globalKnownVars.clone(), repl.clone())?;
            (globalKnownVars, repl) = replaceFinalVars(1, globalKnownVars.clone(), repl.clone())?;
            if Flags::isSet(Flags::DUMP_PARAM_REPL.clone())? {
                BackendVarTransform::dumpReplacements(repl.clone())?;
            }
            systs = List::map1(systs.clone(), Arc::new(removeParameterswork), repl.clone());
            todo!("unhandled field-assign shape: shared.globalKnownVars");
            BackendDAE::DAE(systs.clone(), shared.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outDAE)
}

fn removeParameterswork(mut isyst: Arc<BackendDAE::EqSystem>, mut repl: BackendVarTransform::VariableReplacements) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    osyst = (::match_deref::match_deref! { match &(isyst.clone()) {
        syst @ Deref @ BackendDAE::EqSystem { orderedEqs: eqns, orderedVars: vars, .. } => {
            let mut lsteqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut syst = (*syst).clone();
            let mut vars = (*vars).clone();
            (vars, _) = replaceFinalVars(1, vars.clone(), repl.clone())?;
            (lsteqns, _) = BackendVarTransform::replaceEquations(BackendEquation::equationList(eqns.clone()), repl.clone(), None)?;
            assign_field!(
                syst.orderedVars = vars.clone(),
                syst.orderedEqs = BackendEquation::listEquation(lsteqns.clone())?,
                syst.m = None,
                syst.mT = None
            );
            syst.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(osyst)
}

fn removeParametersFinder(mut inVar: BackendDAE::Var, mut inTpl: (BackendVarTransform::VariableReplacements, BackendDAE::Variables)) -> Result<(BackendDAE::Var, (BackendVarTransform::VariableReplacements, BackendDAE::Variables))> {
    let mut outVar: BackendDAE::Var;
    let mut outTpl: (BackendVarTransform::VariableReplacements, BackendDAE::Variables);
    (outVar, outTpl) = 'mc: {
        let __mc_input = (inVar.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ BackendDAE::Var { bindExp: Some(exp), varKind: BackendDAE::VarKind::PARAM, varName, .. }, (repl, vars)) => {
                    let mut repl_1: BackendVarTransform::VariableReplacements;
                    let mut exp1: Arc<DAE::Exp>;
                    (exp1, _) = Expression::traverseExpBottomUp(exp.clone(), Arc::new(BackendDAEUtil::replaceCrefsWithValues), (vars.clone(), varName.clone()))?;
                    repl_1 = BackendVarTransform::addReplacement(repl.clone(), varName.clone(), exp1.clone(), None)?;
                    Ok((v.clone(), (repl_1.clone(), vars.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inVar.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVar, outTpl))
}

fn replaceFinalVars(mut inNumRepl: i32, mut inVars: BackendDAE::Variables, mut inRepl: BackendVarTransform::VariableReplacements) -> Result<(BackendDAE::Variables, BackendVarTransform::VariableReplacements)> {
    let mut outVars: BackendDAE::Variables;
    let mut outRepl: BackendVarTransform::VariableReplacements;
    (outVars, outRepl) = 'mc: {
        let __mc_input = (inNumRepl.clone(), inVars.clone(), inRepl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut numrepl, mut globalKnownVars, mut repl) = __mc_input.clone() else { bail!("nomatch") };
            let true = (intEq(0, numrepl.clone())) else { bail!("pattern mismatch") };
            Ok((globalKnownVars.clone(), repl.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, mut globalKnownVars, mut repl) = __mc_input.clone() else { bail!("nomatch") };
            let mut numrepl: i32 = 0;
            let mut globalKnownVars1: BackendDAE::Variables;
            let mut globalKnownVars2: BackendDAE::Variables;
            let mut repl1: BackendVarTransform::VariableReplacements;
            let mut repl2: BackendVarTransform::VariableReplacements;
            let (__pa0, (__pa1, __pa2)) = BackendVariable::traverseBackendDAEVarsWithUpdate(globalKnownVars.clone(), Arc::new(replaceFinalVarTraverser), (repl.clone(), 0))?;
            globalKnownVars1 = __pa0.clone();
            repl1 = __pa1.clone();
            numrepl = __pa2.clone();
            (globalKnownVars2, repl2) = replaceFinalVars(numrepl.clone(), globalKnownVars1.clone(), repl1.clone())?;
            Ok((globalKnownVars2.clone(), repl2.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVars, outRepl))
}

fn replaceFinalVarTraverser(mut inVar: BackendDAE::Var, mut inTpl: (BackendVarTransform::VariableReplacements, i32)) -> Result<(BackendDAE::Var, (BackendVarTransform::VariableReplacements, i32))> {
    let mut outVar: BackendDAE::Var;
    let mut outTpl: (BackendVarTransform::VariableReplacements, i32);
    (outVar, outTpl) = 'mc: {
        let __mc_input = (inVar.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (BackendDAE::Var { bindExp: Some(Deref @ DAE::Exp::CALL { .. }), .. }, _) => {
                    Ok((inVar.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ BackendDAE::Var { values: attr, bindExp: Some(e), varName: cr, .. }, (repl, numrepl)) => {
                    let mut v1: BackendDAE::Var;
                    let mut repl_1: BackendVarTransform::VariableReplacements;
                    let mut e1: Arc<DAE::Exp>;
                    let mut attr = (*attr).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVarTransform::replaceExp(e.clone(), repl.clone(), None)?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1 = __pa0.clone();
                    (e1, _) = ExpressionSimplify::simplify(e1.clone())?;
                    v1 = BackendVariable::setBindExp(v.clone(), Some(e1.clone()));
                    repl_1 = addConstExpReplacement(e1.clone(), cr.clone(), repl.clone())?;
                    (attr, repl_1) = BackendDAEUtil::traverseBackendDAEVarAttr(attr.clone(), Arc::new(traverseExpVisitorWrapper), repl_1.clone())?;
                    v1 = BackendVariable::setVarAttributes(v1.clone(), attr.clone());
                    Ok((v1.clone(), (repl_1.clone(), numrepl.clone() + 1)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ BackendDAE::Var { values: attr, .. }, (repl, numrepl)) => {
                    let mut v1: BackendDAE::Var;
                    let mut new_attr: Option<Arc<DAE::VariableAttributes>> = None;
                    let mut repl = (*repl).clone();
                    (new_attr, repl) = BackendDAEUtil::traverseBackendDAEVarAttr(attr.clone(), Arc::new(traverseExpVisitorWrapper), repl.clone())?;
                    v1 = BackendVariable::setVarAttributes(v.clone(), new_attr.clone());
                    Ok((v1.clone(), (repl.clone(), numrepl.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVar, outTpl))
}

fn addConstExpReplacement(mut inExp: Arc<DAE::Exp>, mut cr: Arc<DAE::ComponentRef>, mut inRepl: BackendVarTransform::VariableReplacements) -> Result<BackendVarTransform::VariableReplacements> {
    let mut outRepl: BackendVarTransform::VariableReplacements;
    outRepl = 'mc: {
        let __mc_input = inRepl.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (Expression::isConst(inExp.clone())?) else { bail!("pattern mismatch") };
            Ok(BackendVarTransform::addReplacement(inRepl.clone(), cr.clone(), inExp.clone(), None)?)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(inRepl.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outRepl)
}

fn traverseExpVisitorWrapper(mut inExp: Arc<DAE::Exp>, mut inRepl: BackendVarTransform::VariableReplacements) -> Result<(Arc<DAE::Exp>, BackendVarTransform::VariableReplacements)> {
    let mut exp: Arc<DAE::Exp>;
    let mut repl: BackendVarTransform::VariableReplacements;
    (exp, repl) = 'mc: {
        let __mc_input = (inExp.clone(), inRepl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ DAE::Exp::CREF { componentRef: _, ty: _ }, repl) => {
                    let mut exp = (*exp).clone();
                    (exp, _) = BackendVarTransform::replaceExp(exp.clone(), repl.clone(), None)?;
                    Ok((exp.clone(), repl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inRepl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((exp, repl))
}

// =============================================================================
// remove protected parameters stuff
//
// =============================================================================
pub fn removeProtectedParameters(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    outDAE = (::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ DAE { UNIQUEIO: systs, derivativeNamePrefix: shared @ BackendDAE::Shared { globalKnownVars, .. }, .. } => {
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut systs = (*systs).clone();
            let mut shared = (*shared).clone();
            repl = BackendVarTransform::emptyReplacements();
            repl = BackendVariable::traverseBackendDAEVars(globalKnownVars.clone(), Arc::new(protectedParametersFinder), repl.clone())?;
            if Flags::isSet(Flags::DUMP_PP_REPL.clone())? {
                BackendVarTransform::dumpReplacements(repl.clone())?;
            }
            systs = List::map1(systs.clone(), Arc::new(removeProtectedParameterswork), repl.clone());
            todo!("unhandled field-assign shape: shared.globalKnownVars");
            BackendDAE::DAE(systs.clone(), shared.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outDAE)
}

fn removeProtectedParameterswork(mut isyst: Arc<BackendDAE::EqSystem>, mut repl: BackendVarTransform::VariableReplacements) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    osyst = (::match_deref::match_deref! { match &(isyst.clone()) {
        syst @ Deref @ BackendDAE::EqSystem { orderedEqs: eqns, .. } => {
            let mut lsteqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut b: bool = false;
            let mut syst = (*syst).clone();
            lsteqns = BackendEquation::equationList(eqns.clone());
            (lsteqns, b) = BackendVarTransform::replaceEquations(lsteqns.clone(), repl.clone(), None)?;
            if b.clone() {
                assign_field!(syst.orderedEqs = BackendEquation::listEquation(lsteqns.clone())?);
                syst = BackendDAEUtil::clearEqSyst(syst.clone())?;
            }
            syst.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(osyst)
}

fn protectedParametersFinder(mut inVar: BackendDAE::Var, mut inRepl: BackendVarTransform::VariableReplacements) -> Result<(BackendDAE::Var, BackendVarTransform::VariableReplacements)> {
    let mut outVar: BackendDAE::Var;
    let mut outRepl: BackendVarTransform::VariableReplacements;
    (outVar, outRepl) = 'mc: {
        let __mc_input = (inVar.clone(), inRepl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ BackendDAE::Var { values, bindExp: Some(exp), varKind: BackendDAE::VarKind::PARAM, varName, .. }, repl) => {
                    let mut repl_1: BackendVarTransform::VariableReplacements;
                    let true = (DAEUtil::getProtectedAttr(values.clone())) else { bail!("pattern mismatch") };
                    repl_1 = BackendVarTransform::addReplacement(repl.clone(), varName.clone(), exp.clone(), None)?;
                    Ok((v.clone(), repl_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inVar.clone(), inRepl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVar, outRepl))
}

// =============================================================================
// remove equal function calls equations stuff
//
// =============================================================================
pub fn removeEqualRHS(mut dae: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut odae: Arc<BackendDAE::BackendDAE>;
    odae = BackendDAEUtil::mapEqSystem(dae.clone(), Arc::new(removeEqualFunctionCallsWork))?;
    Ok(odae)
}

fn removeEqualFunctionCallsWork(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    (osyst, oshared) = (::match_deref::match_deref! { match &(isyst.clone()) {
        syst @ Deref @ BackendDAE::EqSystem { orderedEqs: eqns, orderedVars: vars, .. } => {
            let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut changed: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut isInitial: bool = false;
            let mut funcs; // TODO: local with unresolved type
            let mut syst = (*syst).clone();
            isInitial = BackendDAEUtil::isInitializationDAE(ishared.clone());
            funcs = BackendDAEUtil::getFunctions(ishared.clone())?;
            (syst, m, mT) = BackendDAEUtil::getAdjacencyMatrixfromOption(syst.clone(), crate::BackendDAE::IndexType::NORMAL, Some(funcs.clone()), isInitial.clone())?;
            let (__pa0, (__pa1, _, _, __pa2, _)) = AdjacencyMatrix::traverseAdjacencyMatrix(m.clone(), Arc::new(removeEqualFunctionCallFinder), (mT.clone(), vars.clone(), eqns.clone(), metamodelica::nil(), isInitial.clone()))?;
            m = __pa0.clone();
            mT = __pa1.clone();
            changed = __pa2.clone();
            assign_field!(
                syst.m = Some(m.clone()),
                syst.mT = Some(mT.clone()),
                syst.matching = Arc::new(crate::BackendDAE::Matching::NO_MATCHING)
            );
            syst = BackendDAEUtil::updateAdjacencyMatrix(syst.clone(), crate::BackendDAE::IndexType::NORMAL, None, changed.clone(), isInitial.clone())?;
            (syst.clone(), ishared.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((osyst, oshared))
}

fn removeEqualFunctionCallFinder(mut elem: Arc<metamodelica::List<i32>>, mut pos: i32, mut inTpl: (metamodelica::Array<Arc<metamodelica::List<i32>>>, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<i32>>, bool)) -> Result<(Arc<metamodelica::List<i32>>, (metamodelica::Array<Arc<metamodelica::List<i32>>>, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<i32>>, bool))> {
    let mut outList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut outTpl: (metamodelica::Array<Arc<metamodelica::List<i32>>>, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<i32>>, bool);
    (outList, outTpl) = 'mc: {
        let __mc_input = inTpl.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mT, vars, eqns, changed, isInitial) => {
                    let mut eqns1: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut exp: Arc<DAE::Exp>;
                    let mut e1: Arc<DAE::Exp>;
                    let mut e2: Arc<DAE::Exp>;
                    let mut ecr: Arc<DAE::Exp>;
                    let mut expvars: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
                    let mut controleqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut expvars1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut expvarseqns: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut changed = (*changed).clone();
                    ::match_deref::match_deref! { match &(elem.clone()) {
                        Deref @ metamodelica::List::Cons { head: _, tail: _ } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(BackendEquation::get(eqns.clone(), pos.clone())) {
                        Deref @ BackendDAE::Equation::EQUATION { scalar: __pa0, exp: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    e2 = __pa0.clone();
                    e1 = __pa1.clone();
                    (ecr, exp) = functionCallEqn(e1.clone(), e2.clone(), vars.clone())?;
                    expvars = BackendDAEUtil::adjacencyRowExp(exp.clone(), vars.clone(), Arc::new(crate::AvlSetInt::Tree::EMPTY), None, crate::BackendDAE::IndexType::NORMAL, isInitial.clone())?;
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(List::map2(AvlSetInt::listKeys(expvars.clone(), metamodelica::nil()), Arc::new(fnptr!(varEqns, i32, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>)), pos.clone(), mT.clone())) {
                        Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    expvars1 = __pa2.clone();
                    expvarseqns = __pa3.clone();
                    controleqns = getControlEqns(expvars1.clone(), expvarseqns.clone())?;
                    (eqns1, changed) = removeEqualFunctionCall(controleqns.clone(), ecr.clone(), exp.clone(), eqns.clone(), changed.clone())?;
                    Ok((metamodelica::nil(), (mT.clone(), vars.clone(), eqns1.clone(), changed.clone(), isInitial.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((metamodelica::nil(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outList, outTpl))
}

fn functionCallEqn(mut ie1: Arc<DAE::Exp>, mut ie2: Arc<DAE::Exp>, mut inVars: BackendDAE::Variables) -> Result<(Arc<DAE::Exp>, Arc<DAE::Exp>)> {
    let mut outECr: Arc<DAE::Exp>;
    let mut outExp: Arc<DAE::Exp>;
    (outECr, outExp) = (::match_deref::match_deref! { match &((ie1.clone(), ie2.clone())) {
        (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::UMINUS { .. } }) => {
            bail!("fail")
        },
        (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::CREF { .. }) => {
            bail!("fail")
        },
        (Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::UMINUS { .. } }, Deref @ DAE::Exp::CREF { .. }) => {
            bail!("fail")
        },
        (e1 @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, Deref @ DAE::Exp::UNARY { exp: e2, operator: op @ DAE::Operator::UMINUS { .. } }) => {
            ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), inVars.clone())?) {
                (Deref @ metamodelica::List::Cons { head: _, tail: _ }, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => (),
                _ => bail!("pattern mismatch"),
            } };
            (Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: e1.clone() }), e2.clone())
        },
        (e1 @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, e2) => {
            ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), inVars.clone())?) {
                (Deref @ metamodelica::List::Cons { head: _, tail: _ }, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => (),
                _ => bail!("pattern mismatch"),
            } };
            (e1.clone(), e2.clone())
        },
        (Deref @ DAE::Exp::UNARY { exp: e1, operator: op @ DAE::Operator::UMINUS { .. } }, e2 @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }) => {
            ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), inVars.clone())?) {
                (Deref @ metamodelica::List::Cons { head: _, tail: _ }, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => (),
                _ => bail!("pattern mismatch"),
            } };
            (Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: e2.clone() }), e1.clone())
        },
        (e1, e2 @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }) => {
            ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), inVars.clone())?) {
                (Deref @ metamodelica::List::Cons { head: _, tail: _ }, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => (),
                _ => bail!("pattern mismatch"),
            } };
            (e2.clone(), e1.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outECr, outExp))
}

fn varEqns(mut v: i32, mut pos: i32, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Arc<metamodelica::List<i32>> {
    let mut outVarEqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut vareqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut vareqns1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    vareqns = mT.borrow()[(intAbs(v.clone())-1) as usize].clone();
    vareqns1 = List::map(vareqns.clone(), Arc::new(intAbs.clone()));
    outVarEqns = List::removeOnTrue(intAbs(pos.clone()), Arc::new(fnptr!(intEq, i32, i32)), vareqns1.clone());
    outVarEqns
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getControlEqns(mut inVarsEqn: Arc<metamodelica::List<i32>>, mut inVarsEqns: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outEqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    outEqns = (::match_deref::match_deref! { match &((inVarsEqn.clone(), inVarsEqns.clone())) {
        (a, Deref @ metamodelica::List::Nil) => {
            a.clone()
        },
        (a, Deref @ metamodelica::List::Cons { head: b, tail: rest }) => {
            let mut c: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut d: Arc<metamodelica::List<i32>> = metamodelica::nil();
            c = List::intersectionOnTrue(a.clone(), b.clone(), Arc::new(fnptr!(intEq, i32, i32)));
            d = getControlEqns(c.clone(), rest.clone())?;
            d.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outEqns)
}

fn removeEqualFunctionCall(mut inEqsLst: Arc<metamodelica::List<i32>>, mut inExp: Arc<DAE::Exp>, mut inECr: Arc<DAE::Exp>, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut ichanged: Arc<metamodelica::List<i32>>) -> Result<(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<i32>>)> {
    let mut outEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut outEqsLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (outEqns, outEqsLst) = 'mc: {
        let __mc_input = inEqsLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((inEqns.clone(), ichanged.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: pos, tail: rest } => {
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut eqn1: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut i: i32 = 0;
                    let mut changed: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    eqn = BackendEquation::get(inEqns.clone(), pos.clone());
                    let (__pa0, (_, _, __pa1)) = BackendDAETransform::traverseBackendDAEExpsEqnWithSymbolicOperation(eqn.clone(), Arc::new(replaceExp), (inECr.clone(), inExp.clone(), 0))?;
                    eqn1 = __pa0.clone();
                    i = __pa1.clone();
                    let true = (intGt(i.clone(), 0)) else { bail!("pattern mismatch") };
                    eqns = BackendEquation::setAtIndex(inEqns.clone(), pos.clone(), eqn1.clone())?;
                    changed = List::consOnTrue(!(listMember(pos.clone(), ichanged.clone())), pos.clone(), ichanged.clone());
                    (eqns, changed) = removeEqualFunctionCall(rest.clone(), inExp.clone(), inECr.clone(), eqns.clone(), changed.clone())?;
                    Ok((eqns.clone(), changed.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut changed: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    (eqns, changed) = removeEqualFunctionCall(rest.clone(), inExp.clone(), inECr.clone(), inEqns.clone(), ichanged.clone())?;
                    Ok((eqns.clone(), changed.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outEqns, outEqsLst))
}

fn replaceExp(mut inExp: Arc<DAE::Exp>, mut inTpl: (Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>, (Arc<DAE::Exp>, Arc<DAE::Exp>, i32))) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>, (Arc<DAE::Exp>, Arc<DAE::Exp>, i32)))> {
    let mut e1: Arc<DAE::Exp>;
    let mut outTpl: (Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>, (Arc<DAE::Exp>, Arc<DAE::Exp>, i32));
    let mut e: Arc<DAE::Exp>;
    let mut se: Arc<DAE::Exp>;
    let mut te: Arc<DAE::Exp>;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>> = metamodelica::nil();
    e = inExp.clone();
    let (__pa0, (__pa1, __pa2, __pa3)) = inTpl.clone();
    ops = __pa0.clone();
    se = __pa1.clone();
    te = __pa2.clone();
    i = __pa3.clone();
    (e1, j) = Expression::replaceExp(e.clone(), se.clone(), te.clone())?;
    ops = if (j.clone() > 0) {cons(Arc::new(DAE::SymbolicOperation::SUBSTITUTION { substitutions: list![e1.clone()], source: e.clone() }), ops.clone())} else {ops.clone()};
    outTpl = (ops.clone(), (se.clone(), te.clone(), i.clone() + j.clone()));
    Ok((e1, outTpl))
}

// =============================================================================
// remove unused parameter
//
// =============================================================================
pub fn removeUnusedParameter(mut inDlow: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDlow: Arc<BackendDAE::BackendDAE>;
    outDlow = (::match_deref::match_deref! { match &(inDlow.clone()) {
        Deref @ DAE { UNIQUEIO: eqs, derivativeNamePrefix: shared, .. } => {
            let mut globalKnownVars: BackendDAE::Variables;
            let mut globalKnownVars1: BackendDAE::Variables;
            let mut shared = (*shared).clone();
            globalKnownVars1 = BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone());
            globalKnownVars = shared.globalKnownVars.clone();
            globalKnownVars1 = BackendVariable::traverseBackendDAEVars(globalKnownVars.clone(), Arc::new(copyNonParamVariables), globalKnownVars1.clone())?;
            (_, globalKnownVars1) = List::fold1(eqs.clone(), Arc::new(BackendDAEUtil::traverseBackendDAEExpsEqSystem), checkUnusedVariables, (globalKnownVars.clone(), globalKnownVars1.clone()));
            (_, globalKnownVars1) = BackendDAEUtil::traverseBackendDAEExpsVars(globalKnownVars.clone(), Arc::new(checkUnusedParameter), (globalKnownVars.clone(), globalKnownVars1.clone()))?;
            (_, globalKnownVars1) = BackendDAEUtil::traverseBackendDAEExpsVars(shared.aliasVars.clone(), Arc::new(checkUnusedParameter), (globalKnownVars.clone(), globalKnownVars1.clone()))?;
            (_, globalKnownVars1) = BackendDAEUtil::traverseBackendDAEExpsEqns(shared.removedEqs.clone(), Arc::new(checkUnusedParameter), (globalKnownVars.clone(), globalKnownVars1.clone()))?;
            (_, globalKnownVars1) = BackendDAEUtil::traverseBackendDAEExpsEqns(shared.initialEqs.clone(), Arc::new(checkUnusedParameter), (globalKnownVars.clone(), globalKnownVars1.clone()))?;
            todo!("unhandled field-assign shape: shared.globalKnownVars");
            BackendDAE::DAE(eqs.clone(), shared.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outDlow)
}

fn copyNonParamVariables(mut inVar: BackendDAE::Var, mut inVars: BackendDAE::Variables) -> Result<(BackendDAE::Var, BackendDAE::Variables)> {
    let mut outVar: BackendDAE::Var;
    let mut outVars: BackendDAE::Variables;
    (outVar, outVars) = (match inVar.clone() {
        mut v @ BackendDAE::Var { varKind: BackendDAE::VarKind::PARAM, .. } => {
            (v.clone(), inVars.clone())
        },
        _ => {
            let mut vars1: BackendDAE::Variables;
            vars1 = BackendVariable::addVar(inVar.clone(), inVars.clone())?;
            (inVar.clone(), vars1.clone())
        },
    });
    Ok((outVar, outVars))
}

fn checkUnusedParameter(mut inExp: Arc<DAE::Exp>, mut inTpl: (BackendDAE::Variables, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (BackendDAE::Variables, BackendDAE::Variables);
    (outExp, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp, (vars, _)) => {
                    let mut vars1: BackendDAE::Variables;
                    let (_, (_, __pa0)) = Expression::traverseExpBottomUp(exp.clone(), Arc::new(checkUnusedParameterExp), inTpl.clone())?;
                    vars1 = __pa0.clone();
                    Ok((exp.clone(), (vars.clone(), vars1.clone())))
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

fn checkUnusedParameterExp(mut inExp: Arc<DAE::Exp>, mut inTuple: (BackendDAE::Variables, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTuple: (BackendDAE::Variables, BackendDAE::Variables);
    (outExp, outTuple) = 'mc: {
        let __mc_input = (inExp.clone(), inTuple.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. }, .. }, (_, _)) => {
                    Ok((e.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, varLst, .. }, componentRef: cr }, tp) => {
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut tp = (*tp).clone();
                    expl = List::map1(varLst.clone(), Arc::new(Expression::generateCrefsExpFromExpVar), cr.clone());
                    (_, tp) = Expression::traverseExpList(expl.clone(), Arc::new(checkUnusedParameterExp), tp.clone())?;
                    Ok((e.clone(), tp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { .. }, .. }, tp) => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut tp = (*tp).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::extendArrExp(e.clone(), false)?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1 = __pa0.clone();
                    (_, tp) = Expression::traverseExpBottomUp(e1.clone(), Arc::new(checkUnusedParameterExp), tp.clone())?;
                    Ok((e.clone(), tp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { .. }, .. }, _) => {
                    Ok((e.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (_, vars1)) => {
                    (_, _) = BackendVariable::getVar(cr.clone(), vars1.clone())?;
                    Ok((e.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, vars1)) => {
                    let mut var: BackendDAE::Var;
                    let mut vars1 = (*vars1).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: _ }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    var = __pa0.clone();
                    vars1 = BackendVariable::addVar(var.clone(), vars1.clone())?;
                    Ok((e.clone(), (vars.clone(), vars1.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outTuple))
}

// =============================================================================
// remove unused variables
//
// =============================================================================
pub fn removeUnusedVariables(mut inDlow: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDlow: Arc<BackendDAE::BackendDAE>;
    outDlow = (::match_deref::match_deref! { match &(inDlow.clone()) {
        Deref @ DAE { UNIQUEIO: eqs, derivativeNamePrefix: shared, .. } => {
            let mut globalKnownVars: BackendDAE::Variables;
            let mut globalKnownVars1: BackendDAE::Variables;
            let mut tpl: (BackendDAE::Variables, BackendDAE::Variables);
            let mut shared = (*shared).clone();
            globalKnownVars1 = BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone());
            globalKnownVars = shared.globalKnownVars.clone();
            tpl = List::fold1(eqs.clone(), Arc::new(BackendDAEUtil::traverseBackendDAEExpsEqSystem), checkUnusedVariables, (globalKnownVars.clone(), globalKnownVars1.clone()));
            tpl = BackendDAEUtil::traverseBackendDAEExpsVars(globalKnownVars.clone(), Arc::new(checkUnusedVariables), tpl.clone())?;
            tpl = BackendDAEUtil::traverseBackendDAEExpsVars(shared.aliasVars.clone(), Arc::new(checkUnusedVariables), tpl.clone())?;
            tpl = BackendDAEUtil::traverseBackendDAEExpsEqns(shared.removedEqs.clone(), Arc::new(checkUnusedVariables), tpl.clone())?;
            (_, globalKnownVars1) = BackendDAEUtil::traverseBackendDAEExpsEqns(shared.initialEqs.clone(), Arc::new(checkUnusedVariables), tpl.clone())?;
            todo!("unhandled field-assign shape: shared.globalKnownVars");
            BackendDAE::DAE(eqs.clone(), shared.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outDlow)
}

fn checkUnusedVariables(mut inExp: Arc<DAE::Exp>, mut inTpl: (BackendDAE::Variables, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (BackendDAE::Variables, BackendDAE::Variables);
    (outExp, outTpl) = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                exp => {
                    let mut tpl: (BackendDAE::Variables, BackendDAE::Variables);
                    (_, tpl) = Expression::traverseExpBottomUp(exp.clone(), Arc::new(checkUnusedVariablesExp), inTpl.clone())?;
                    Ok((exp.clone(), tpl.clone()))
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

fn checkUnusedVariablesExp(mut inExp: Arc<DAE::Exp>, mut inTuple: (BackendDAE::Variables, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTuple: (BackendDAE::Variables, BackendDAE::Variables);
    (outExp, outTuple) = 'mc: {
        let __mc_input = (inExp.clone(), inTuple.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. }, .. }, tp) => {
                    Ok((e.clone(), tp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, varLst, .. }, componentRef: cr }, tp) => {
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut tp = (*tp).clone();
                    expl = List::map1(varLst.clone(), Arc::new(Expression::generateCrefsExpFromExpVar), cr.clone());
                    (_, tp) = Expression::traverseExpList(expl.clone(), Arc::new(checkUnusedVariablesExp), tp.clone())?;
                    Ok((e.clone(), tp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { .. }, .. }, tp) => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut tp = (*tp).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::extendArrExp(e.clone(), false)?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1 = __pa0.clone();
                    (_, tp) = Expression::traverseExpBottomUp(e1.clone(), Arc::new(checkUnusedVariablesExp), tp.clone())?;
                    Ok((e.clone(), tp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { .. }, .. }, _) => {
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (_, vars1)) => {
                    (_, _) = BackendVariable::getVar(cr.clone(), vars1.clone())?;
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, vars1)) => {
                    let mut var: BackendDAE::Var;
                    let mut vars1 = (*vars1).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: _ }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    var = __pa0.clone();
                    vars1 = BackendVariable::addVar(var.clone(), vars1.clone())?;
                    Ok((inExp.clone(), (vars.clone(), vars1.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outTuple))
}

// =============================================================================
// remove unused functions
//
// =============================================================================
// =============================================================================
// parallel back end stuff (TLM)
//
// =============================================================================
pub fn collapseIndependentBlocks(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    let mut syst: Arc<BackendDAE::EqSystem>;
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared>;
    let mut sz: i32 = 0;
    let mut vars: BackendDAE::Variables;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ DAE { UNIQUEIO: __pa0, derivativeNamePrefix: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    systs = __pa0.clone();
    shared = __pa1.clone();
    vars = BackendVariable::emptyVarsSized(((metamodelica::OrderedFloat(({
        let mut __acc: i32 = 0;
        for mut s in (systs.clone()).into_iter().cloned() {
            let __x = BackendVariable::varsSize(s.orderedVars.clone())?;
            __acc += __x;
        }
        __acc
    }) as f64) * metamodelica::OrderedFloat(1.4_f64)).0.floor() as i32));
    syst = List::fold(systs.clone().reverse(), Arc::new(mergeIndependentBlocks), BackendDAEUtil::createEqSystem(vars.clone(), BackendEquation::emptyEqns(), metamodelica::nil(), crate::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns()));
    outDAE = BackendDAE::DAE(list![syst.clone()], shared.clone())?;
    Ok(outDAE)
}

fn mergeIndependentBlocks(mut syst1: Arc<BackendDAE::EqSystem>, mut syst2: Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut syst: Arc<BackendDAE::EqSystem>;
    let mut vars: BackendDAE::Variables;
    let mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut removedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut stateSets: Arc<metamodelica::List<BackendDAE::StateSet>> = metamodelica::nil();
    vars = BackendVariable::addVariables(syst1.orderedVars.clone(), syst2.orderedVars.clone())?;
    eqs = BackendEquation::addList(BackendEquation::equationList(syst1.orderedEqs.clone()), syst2.orderedEqs.clone())?;
    removedEqs = BackendEquation::addList(BackendEquation::equationList(syst1.removedEqs.clone()), syst2.removedEqs.clone())?;
    stateSets = listAppend(syst1.stateSets.clone(), syst2.stateSets.clone());
    syst = BackendDAEUtil::createEqSystem(vars.clone(), eqs.clone(), stateSets.clone(), crate::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, removedEqs.clone());
    Ok(syst)
}

pub fn partitionIndependentBlocks(mut dlow: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDlow: Arc<BackendDAE::BackendDAE>;
    outDlow = (::match_deref::match_deref! { match &(dlow.clone()) {
        Deref @ DAE { UNIQUEIO: metamodelica::List::Cons { head: syst, tail: Deref @ metamodelica::List::Nil }, derivativeNamePrefix: shared, .. } => {
            let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
            let mut shared = (*shared).clone();
            (systs, shared) = partitionIndependentBlocksHelper(syst.clone(), shared.clone(), Error::getNumErrorMessages(), false)?;
            BackendDAE::DAE(systs.clone(), shared.clone())?
        },
        _ => {
            let mut syst: Arc<BackendDAE::EqSystem>;
            let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
            let mut shared: Arc<BackendDAE::Shared>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(collapseIndependentBlocks(dlow.clone())?) {
                Deref @ DAE { UNIQUEIO: metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, derivativeNamePrefix: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            syst = __pa0.clone();
            shared = __pa1.clone();
            (systs, shared) = partitionIndependentBlocksHelper(syst.clone(), shared.clone(), Error::getNumErrorMessages(), false)?;
            BackendDAE::DAE(systs.clone(), shared.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDlow)
}

pub fn partitionIndependentBlocksHelper(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut numErrorMessages: i32, mut throwNoError: bool) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, Arc<BackendDAE::Shared>)> {
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut oshared: Arc<BackendDAE::Shared>;
    (systs, oshared) = 'mc: {
        let __mc_input = isyst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                syst => {
                    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut rm: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut rmT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut eqPartMap: metamodelica::Array<i32>;
                    let mut varPartMap: metamodelica::Array<i32>;
                    let mut rixs: metamodelica::Array<i32>;
                    let mut vars: metamodelica::Array<bool>;
                    let mut rvars: metamodelica::Array<bool>;
                    let mut b: bool = false;
                    let mut isInitial: bool = false;
                    let mut i: i32 = 0;
                    let mut funcs; // TODO: local with unresolved type
                    let mut syst = (*syst).clone();
                    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = systs.clone();
                    isInitial = BackendDAEUtil::isInitializationDAE(ishared.clone());
                    funcs = BackendDAEUtil::getFunctions(ishared.clone())?;
                    (syst, m, mT) = BackendDAEUtil::getAdjacencyMatrixfromOption(syst.clone(), crate::BackendDAE::IndexType::NORMAL, Some(funcs.clone()), isInitial.clone())?;
                    (rm, rmT) = BackendDAEUtil::removedAdjacencyMatrix(syst.clone(), crate::BackendDAE::IndexType::NORMAL, Some(funcs.clone()), isInitial.clone())?;
                    eqPartMap = arrayCreate((m.clone().borrow().len() as i32), 0);
                    varPartMap = arrayCreate((mT.clone().borrow().len() as i32), 0);
                    rixs = arrayCreate((rm.clone().borrow().len() as i32), 0);
                    vars = arrayCreate((mT.clone().borrow().len() as i32), false);
                    rvars = arrayCreate((rmT.clone().borrow().len() as i32), false);
                    i = SynchronousFeatures::partitionIndependentBlocks0(m.clone(), mT.clone(), rm.clone(), rmT.clone(), eqPartMap.clone(), varPartMap.clone(), rixs.clone(), vars.clone(), rvars.clone())?;
                    b = i.clone() > 1;
                    systs = if (b.clone()) {SynchronousFeatures::partitionIndependentBlocksSplitBlocks(i.clone(), syst.clone(), eqPartMap.clone(), rixs.clone(), mT.clone(), rmT.clone(), throwNoError.clone(), funcs.clone(), isInitial.clone())?} else {list![syst.clone()]};
                    GCExt::free(eqPartMap.clone());
                    GCExt::free(varPartMap.clone());
                    GCExt::free(rixs.clone());
                    Ok((systs.clone(), ishared.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::assertion(!(numErrorMessages.clone() == Error::getNumErrorMessages()), (literal!("BackendDAEOptimize.partitionIndependentBlocks failed without good error message")).clone(), Absyn::dummyInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((systs, oshared))
}

// =============================================================================
// residual stuff ... for whatever reason
//
// =============================================================================
pub fn residualForm(mut dlow: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut odlow: Arc<BackendDAE::BackendDAE>;
    odlow = BackendDAEUtil::mapEqSystem1(dlow.clone(), Arc::new(residualForm1), 1)?;
    Ok(odlow)
}

fn residualForm1(mut syst: Arc<BackendDAE::EqSystem>, mut i: i32, mut shared: Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> {
    let mut osyst: Arc<BackendDAE::EqSystem> = syst.clone();
    let mut oshared: Arc<BackendDAE::Shared> = shared.clone();
    let mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let __pa0 = ::match_deref::match_deref! { match &(syst.clone()) {
        Deref @ BackendDAE::EqSystem { orderedEqs: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    eqs = __pa0.clone();
    BackendEquation::traverseEquationArray_WithUpdate(eqs.clone(), Arc::new(residualForm2), 1)?;
    Ok((osyst, oshared))
}

fn residualForm2(mut inEq: Arc<BackendDAE::Equation>, mut ii: i32) -> Result<(Arc<BackendDAE::Equation>, i32)> {
    let mut outEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut oi: i32 = 0;
    (outEq, oi) = 'mc: {
        let __mc_input = inEq.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, source, attr: eqAttr } => {
                    let mut e: Arc<DAE::Exp>;
                    let mut source = (*source).clone();
                    ::match_deref::match_deref! { match &(Expression::r#typeof(e1.clone())?) {
                        Deref @ DAE::Type::T_REAL { .. } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    let false = (Expression::isZero(e1.clone()) || Expression::isZero(e2.clone())) else { bail!("pattern mismatch") };
                    e = Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT.clone() }, exp2: e2.clone() });
                    (e, _) = ExpressionSimplify::simplify(e.clone())?;
                    source = ElementSource::addSymbolicTransformation(source.clone(), Arc::new(DAE::SymbolicOperation::OP_RESIDUAL { e1: e1.clone(), e2: e2.clone(), e: e.clone() }))?;
                    Ok((Arc::new(BackendDAE::Equation::EQUATION { exp: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), scalar: e.clone(), source: source.clone(), attr: eqAttr.clone() }), ii.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inEq.clone(), ii.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outEq, oi))
}

// =============================================================================
// countOperations
//
// =============================================================================
pub fn countOperations(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    if Flags::isSet(Flags::COUNT_OPERATIONS.clone())? {
        (outDAE, _) = BackendDAEUtil::mapEqSystemAndFold(inDAE.clone(), Arc::new(countOperations0), false)?;
    } else {
        outDAE = inDAE.clone();
    }
    Ok(outDAE)
}

fn countOperations0(mut isyst: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut inChanged: bool) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool)> {
    let mut osyst: Arc<BackendDAE::EqSystem> = isyst.clone();
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut outChanged: bool = inChanged.clone();
    let mut compInfos: Arc<metamodelica::List<Arc<BackendDAE::CompInfo>>> = metamodelica::nil();
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(isyst.clone()) {
        Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa0, .. }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    comps = __pa0.clone();
    compInfos = countOperationstraverseComps(comps.clone(), isyst.clone(), inShared.clone(), metamodelica::nil())?;
    Ok((osyst, outShared, outChanged))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn countOperationstraverseComps(mut inComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut compInfosIn: Arc<metamodelica::List<Arc<BackendDAE::CompInfo>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::CompInfo>>>> {
    let mut compInfosOut: Arc<metamodelica::List<Arc<BackendDAE::CompInfo>>> = metamodelica::nil();
    compInfosOut = 'mc: {
        let __mc_input = (inComps.clone(), ishared.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(compInfosIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { eqn: eqIdx, .. }, tail: rest }, _) => {
                    let mut numAdd: i32 = 0;
                    let mut numMul: i32 = 0;
                    let mut numDiv: i32 = 0;
                    let mut numTrig: i32 = 0;
                    let mut numRel: i32 = 0;
                    let mut numOth: i32 = 0;
                    let mut numFuncs: i32 = 0;
                    let mut numLog: i32 = 0;
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut compInfo: Arc<BackendDAE::CompInfo>;
                    eqns = BackendEquation::getEqnsFromEqSystem(isyst.clone());
                    eqn = BackendEquation::get(eqns.clone(), eqIdx.clone());
                    let (_, (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7)) = BackendEquation::traverseExpsOfEquation(eqn.clone(), Arc::new({ let __pe_b1 = ishared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, __pe_b1.clone(), __pe_a2) }), (0, 0, 0, 0, 0, 0, 0, 0))?;
                    numAdd = __pa0.clone();
                    numMul = __pa1.clone();
                    numDiv = __pa2.clone();
                    numTrig = __pa3.clone();
                    numRel = __pa4.clone();
                    numLog = __pa5.clone();
                    numOth = __pa6.clone();
                    numFuncs = __pa7.clone();
                    compInfo = Arc::new(BackendDAE::CompInfo::COUNTER { comp: listHead(inComps.clone())?, numAdds: numAdd.clone(), numMul: numMul.clone(), numDiv: numDiv.clone(), numTrig: numTrig.clone(), numRelations: numRel.clone(), numLog: numLog.clone(), numOth: numOth.clone(), funcCalls: numFuncs.clone() });
                    if Flags::isSet(Flags::COUNT_OPERATIONS.clone())? {
                        BackendDump::dumpCompInfo(compInfo.clone())?;
                    }
                    Ok(countOperationstraverseComps(rest.clone(), isyst.clone(), ishared.clone(), cons(compInfo.clone(), compInfosIn.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { eqn: eqIdx, .. }, tail: rest }, _) => {
                    let mut numAdd: i32 = 0;
                    let mut numMul: i32 = 0;
                    let mut numDiv: i32 = 0;
                    let mut numTrig: i32 = 0;
                    let mut numRel: i32 = 0;
                    let mut numOth: i32 = 0;
                    let mut numFuncs: i32 = 0;
                    let mut numLog: i32 = 0;
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut compInfo: Arc<BackendDAE::CompInfo>;
                    eqn = BackendEquation::get(BackendEquation::getEqnsFromEqSystem(isyst.clone()), eqIdx.clone());
                    let (_, (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7)) = BackendEquation::traverseExpsOfEquation(eqn.clone(), Arc::new({ let __pe_b1 = ishared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, __pe_b1.clone(), __pe_a2) }), (0, 0, 0, 0, 0, 0, 0, 0))?;
                    numAdd = __pa0.clone();
                    numMul = __pa1.clone();
                    numDiv = __pa2.clone();
                    numTrig = __pa3.clone();
                    numRel = __pa4.clone();
                    numLog = __pa5.clone();
                    numOth = __pa6.clone();
                    numFuncs = __pa7.clone();
                    compInfo = Arc::new(BackendDAE::CompInfo::COUNTER { comp: listHead(inComps.clone())?, numAdds: numAdd.clone(), numMul: numMul.clone(), numDiv: numDiv.clone(), numTrig: numTrig.clone(), numRelations: numRel.clone(), numLog: numLog.clone() + 1, numOth: numOth.clone(), funcCalls: numFuncs.clone() });
                    if Flags::isSet(Flags::COUNT_OPERATIONS.clone())? {
                        BackendDump::dumpCompInfo(compInfo.clone())?;
                    }
                    Ok(countOperationstraverseComps(rest.clone(), isyst.clone(), ishared.clone(), cons(compInfo.clone(), compInfosIn.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: comp @ Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jacType: BackendDAE::JacobianType::JAC_LINEAR, jac, eqns: eqs, .. }, tail: rest }, _) => {
                    let mut size: i32 = 0;
                    let mut density: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut compInfo: Arc<BackendDAE::CompInfo>;
                    let mut allOps: Arc<BackendDAE::CompInfo>;
                    (_, _, _) = BackendDAETransform::getEquationAndSolvedVar(comp.clone(), BackendEquation::getEqnsFromEqSystem(isyst.clone()), BackendVariable::daeVars(isyst.clone()))?;
                    size = (eqs.clone().len() as i32);
                    density = realDiv(intReal(getNumJacEntries(jac.clone())), intReal(size.clone() * size.clone()));
                    allOps = Arc::new(BackendDAE::CompInfo::COUNTER { comp: comp.clone(), numAdds: 0, numMul: 0, numDiv: 0, numTrig: 0, numRelations: 0, numLog: 0, numOth: 0, funcCalls: 0 });
                    allOps = countOperationsJac(jac.clone(), ishared.clone(), allOps.clone())?;
                    compInfo = Arc::new(BackendDAE::CompInfo::SYSTEM { comp: comp.clone(), allOperations: allOps.clone(), size: size.clone(), density: density.clone() });
                    Ok(countOperationstraverseComps(rest.clone(), isyst.clone(), ishared.clone(), cons(compInfo.clone(), compInfosIn.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: comp @ Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jac, .. }, tail: rest }, _) => {
                    let mut numAdd: i32 = 0;
                    let mut numMul: i32 = 0;
                    let mut numDiv: i32 = 0;
                    let mut numTrig: i32 = 0;
                    let mut numRel: i32 = 0;
                    let mut numOth: i32 = 0;
                    let mut numFuncs: i32 = 0;
                    let mut numLog: i32 = 0;
                    let mut size: i32 = 0;
                    let mut jacEntries: i32 = 0;
                    let mut density: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut compInfo: Arc<BackendDAE::CompInfo>;
                    let mut allOps: Arc<BackendDAE::CompInfo>;
                    let mut eqnlst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    (eqnlst, _, _) = BackendDAETransform::getEquationAndSolvedVar(comp.clone(), BackendEquation::getEqnsFromEqSystem(isyst.clone()), BackendVariable::daeVars(isyst.clone()))?;
                    size = (eqnlst.clone().len() as i32);
                    (numAdd, numMul, numDiv, numTrig, numRel, numLog, numOth, numFuncs) = BackendDAEUtil::traverseBackendDAEExpsEqns(BackendEquation::listEquation(eqnlst.clone())?, Arc::new({ let __pe_b1 = ishared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, __pe_b1.clone(), __pe_a2) }), (0, 0, 0, 0, 0, 0, 0, 0))?;
                    allOps = Arc::new(BackendDAE::CompInfo::COUNTER { comp: comp.clone(), numAdds: numAdd.clone(), numMul: numMul.clone(), numDiv: numDiv.clone(), numTrig: numTrig.clone(), numRelations: numRel.clone(), numLog: numLog.clone(), numOth: numOth.clone(), funcCalls: numFuncs.clone() });
                    jacEntries = getNumJacEntries(jac.clone());
                    if intEq(jacEntries.clone(), -1) {
                        jacEntries = size.clone() * size.clone();
                    }
                    density = realDiv(intReal(jacEntries.clone()), intReal(size.clone() * size.clone()));
                    compInfo = Arc::new(BackendDAE::CompInfo::SYSTEM { comp: comp.clone(), allOperations: allOps.clone(), size: size.clone(), density: density.clone() });
                    Ok(countOperationstraverseComps(rest.clone(), isyst.clone(), ishared.clone(), cons(compInfo.clone(), compInfosIn.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: comp @ Deref @ BackendDAE::StrongComponent::SINGLEARRAY { eqn: eqIdx, .. }, tail: rest }, _) => {
                    let mut numAdd: i32 = 0;
                    let mut numMul: i32 = 0;
                    let mut numDiv: i32 = 0;
                    let mut numTrig: i32 = 0;
                    let mut numRel: i32 = 0;
                    let mut numOth: i32 = 0;
                    let mut numFuncs: i32 = 0;
                    let mut numLog: i32 = 0;
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut compInfo: Arc<BackendDAE::CompInfo>;
                    eqn = BackendEquation::get(BackendEquation::getEqnsFromEqSystem(isyst.clone()), eqIdx.clone());
                    let (_, (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7)) = BackendEquation::traverseExpsOfEquation(eqn.clone(), Arc::new({ let __pe_b1 = ishared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, __pe_b1.clone(), __pe_a2) }), (0, 0, 0, 0, 0, 0, 0, 0))?;
                    numAdd = __pa0.clone();
                    numMul = __pa1.clone();
                    numDiv = __pa2.clone();
                    numTrig = __pa3.clone();
                    numRel = __pa4.clone();
                    numLog = __pa5.clone();
                    numOth = __pa6.clone();
                    numFuncs = __pa7.clone();
                    compInfo = Arc::new(BackendDAE::CompInfo::COUNTER { comp: comp.clone(), numAdds: numAdd.clone(), numMul: numMul.clone(), numDiv: numDiv.clone(), numTrig: numTrig.clone(), numRelations: numRel.clone(), numLog: numLog.clone(), numOth: numOth.clone(), funcCalls: numFuncs.clone() });
                    Ok(countOperationstraverseComps(rest.clone(), isyst.clone(), ishared.clone(), cons(compInfo.clone(), compInfosIn.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: comp @ Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { eqn: eqIdx, .. }, tail: rest }, _) => {
                    let mut numAdd: i32 = 0;
                    let mut numMul: i32 = 0;
                    let mut numDiv: i32 = 0;
                    let mut numTrig: i32 = 0;
                    let mut numRel: i32 = 0;
                    let mut numOth: i32 = 0;
                    let mut numFuncs: i32 = 0;
                    let mut numLog: i32 = 0;
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut compInfo: Arc<BackendDAE::CompInfo>;
                    eqn = BackendEquation::get(BackendEquation::getEqnsFromEqSystem(isyst.clone()), eqIdx.clone());
                    let (_, (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7)) = BackendEquation::traverseExpsOfEquation(eqn.clone(), Arc::new({ let __pe_b1 = ishared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, __pe_b1.clone(), __pe_a2) }), (0, 0, 0, 0, 0, 0, 0, 0))?;
                    numAdd = __pa0.clone();
                    numMul = __pa1.clone();
                    numDiv = __pa2.clone();
                    numTrig = __pa3.clone();
                    numRel = __pa4.clone();
                    numLog = __pa5.clone();
                    numOth = __pa6.clone();
                    numFuncs = __pa7.clone();
                    compInfo = Arc::new(BackendDAE::CompInfo::COUNTER { comp: comp.clone(), numAdds: numAdd.clone(), numMul: numMul.clone(), numDiv: numDiv.clone(), numTrig: numTrig.clone(), numRelations: numRel.clone(), numLog: numLog.clone() + 1, numOth: numOth.clone(), funcCalls: numFuncs.clone() });
                    Ok(countOperationstraverseComps(rest.clone(), isyst.clone(), ishared.clone(), cons(compInfo.clone(), compInfosIn.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: comp @ Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { eqn: eqIdx, .. }, tail: rest }, _) => {
                    let mut numAdd: i32 = 0;
                    let mut numMul: i32 = 0;
                    let mut numDiv: i32 = 0;
                    let mut numTrig: i32 = 0;
                    let mut numRel: i32 = 0;
                    let mut numOth: i32 = 0;
                    let mut numFuncs: i32 = 0;
                    let mut numLog: i32 = 0;
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut compInfo: Arc<BackendDAE::CompInfo>;
                    eqn = BackendEquation::get(BackendEquation::getEqnsFromEqSystem(isyst.clone()), eqIdx.clone());
                    let (_, (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7)) = BackendEquation::traverseExpsOfEquation(eqn.clone(), Arc::new({ let __pe_b1 = ishared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, __pe_b1.clone(), __pe_a2) }), (0, 0, 0, 0, 0, 0, 0, 0))?;
                    numAdd = __pa0.clone();
                    numMul = __pa1.clone();
                    numDiv = __pa2.clone();
                    numTrig = __pa3.clone();
                    numRel = __pa4.clone();
                    numLog = __pa5.clone();
                    numOth = __pa6.clone();
                    numFuncs = __pa7.clone();
                    compInfo = Arc::new(BackendDAE::CompInfo::COUNTER { comp: comp.clone(), numAdds: numAdd.clone(), numMul: numMul.clone(), numDiv: numDiv.clone(), numTrig: numTrig.clone(), numRelations: numRel.clone(), numLog: numLog.clone() + 1, numOth: numOth.clone(), funcCalls: numFuncs.clone() });
                    Ok(countOperationstraverseComps(rest.clone(), isyst.clone(), ishared.clone(), cons(compInfo.clone(), compInfosIn.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: comp @ Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { eqn: eqIdx, .. }, tail: rest }, _) => {
                    let mut numAdd: i32 = 0;
                    let mut numMul: i32 = 0;
                    let mut numDiv: i32 = 0;
                    let mut numTrig: i32 = 0;
                    let mut numRel: i32 = 0;
                    let mut numOth: i32 = 0;
                    let mut numFuncs: i32 = 0;
                    let mut numLog: i32 = 0;
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut compInfo: Arc<BackendDAE::CompInfo>;
                    eqn = BackendEquation::get(BackendEquation::getEqnsFromEqSystem(isyst.clone()), eqIdx.clone());
                    let (_, (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7)) = BackendEquation::traverseExpsOfEquation(eqn.clone(), Arc::new({ let __pe_b1 = ishared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, __pe_b1.clone(), __pe_a2) }), (0, 0, 0, 0, 0, 0, 0, 0))?;
                    numAdd = __pa0.clone();
                    numMul = __pa1.clone();
                    numDiv = __pa2.clone();
                    numTrig = __pa3.clone();
                    numRel = __pa4.clone();
                    numLog = __pa5.clone();
                    numOth = __pa6.clone();
                    numFuncs = __pa7.clone();
                    compInfo = Arc::new(BackendDAE::CompInfo::COUNTER { comp: comp.clone(), numAdds: numAdd.clone(), numMul: numMul.clone(), numDiv: numDiv.clone(), numTrig: numTrig.clone(), numRelations: numRel.clone(), numLog: numLog.clone() + 1, numOth: numOth.clone(), funcCalls: numFuncs.clone() });
                    Ok(countOperationstraverseComps(rest.clone(), isyst.clone(), ishared.clone(), cons(compInfo.clone(), compInfosIn.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::TORNSYSTEM { linear: true, strictTearingSet: BackendDAE::TearingSet { innerEquations, residualequations: tornEqs, tearingvars: vlst, .. }, .. }, tail: rest }, Deref @ BackendDAE::Shared { functionTree: funcs, .. }) => {
                    let mut numAdd: i32 = 0;
                    let mut numMul: i32 = 0;
                    let mut numDiv: i32 = 0;
                    let mut numTrig: i32 = 0;
                    let mut numRel: i32 = 0;
                    let mut numOth: i32 = 0;
                    let mut numFuncs: i32 = 0;
                    let mut numLog: i32 = 0;
                    let mut otherEqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut comp: Arc<BackendDAE::StrongComponent>;
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut vars: BackendDAE::Variables;
                    let mut compInfo: Arc<BackendDAE::CompInfo>;
                    let mut torn: Arc<BackendDAE::CompInfo>;
                    let mut other: Arc<BackendDAE::CompInfo>;
                    let mut eqnlst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut explst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut vLstLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut vlst = (*vlst).clone();
                    comp = listHead(inComps.clone())?;
                    eqns = BackendEquation::getEqnsFromEqSystem(isyst.clone());
                    vars = BackendVariable::daeVars(isyst.clone());
                    eqnlst = BackendEquation::getList(tornEqs.clone(), eqns.clone());
                    varlst = List::map1(vlst.clone(), Arc::new(BackendVariable::getVarAtIndexFirst), vars.clone());
                    (explst, _) = BackendDAEUtil::getEqnSysRhs(BackendEquation::listEquation(eqnlst.clone())?, BackendVariable::listVar1(varlst.clone()), Some(funcs.clone()))?;
                    let (_, (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7)) = Expression::traverseExpList(explst.clone(), Arc::new({ let __pe_b1 = ishared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, __pe_b1.clone(), __pe_a2) }), (0, 0, 0, 0, 0, 0, 0, 0))?;
                    numAdd = __pa0.clone();
                    numMul = __pa1.clone();
                    numDiv = __pa2.clone();
                    numTrig = __pa3.clone();
                    numRel = __pa4.clone();
                    numLog = __pa5.clone();
                    numOth = __pa6.clone();
                    numFuncs = __pa7.clone();
                    torn = Arc::new(BackendDAE::CompInfo::COUNTER { comp: comp.clone(), numAdds: numAdd.clone(), numMul: numMul.clone(), numDiv: numDiv.clone(), numTrig: numTrig.clone(), numRelations: numRel.clone(), numLog: numLog.clone(), numOth: numOth.clone(), funcCalls: numFuncs.clone() });
                    (otherEqs, vLstLst, _) = List::map_3(innerEquations.clone(), Arc::new(BackendDAEUtil::getEqnAndVarsFromInnerEquation));
                    vlst = List::flatten(vLstLst.clone());
                    eqnlst = BackendEquation::getList(otherEqs.clone(), eqns.clone());
                    varlst = List::map1(vlst.clone(), Arc::new(BackendVariable::getVarAtIndexFirst), vars.clone());
                    (explst, _) = BackendDAEUtil::getEqnSysRhs(BackendEquation::listEquation(eqnlst.clone())?, BackendVariable::listVar1(varlst.clone()), Some(funcs.clone()))?;
                    let (_, (__pa8, __pa9, __pa10, __pa11, __pa12, __pa13, __pa14, __pa15)) = Expression::traverseExpList(explst.clone(), Arc::new({ let __pe_b1 = ishared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, __pe_b1.clone(), __pe_a2) }), (0, 0, 0, 0, 0, 0, 0, 0))?;
                    numAdd = __pa8.clone();
                    numMul = __pa9.clone();
                    numDiv = __pa10.clone();
                    numTrig = __pa11.clone();
                    numRel = __pa12.clone();
                    numLog = __pa13.clone();
                    numOth = __pa14.clone();
                    numFuncs = __pa15.clone();
                    other = Arc::new(BackendDAE::CompInfo::COUNTER { comp: comp.clone(), numAdds: numAdd.clone(), numMul: numMul.clone(), numDiv: numDiv.clone(), numTrig: numTrig.clone(), numRelations: numRel.clone(), numLog: numLog.clone(), numOth: numOth.clone(), funcCalls: numFuncs.clone() });
                    compInfo = Arc::new(BackendDAE::CompInfo::TORN_ANALYSE { comp: comp.clone(), tornEqs: torn.clone(), otherEqs: other.clone(), tornSize: (tornEqs.clone().len() as i32) });
                    Ok(countOperationstraverseComps(rest.clone(), isyst.clone(), ishared.clone(), cons(compInfo.clone(), compInfosIn.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::TORNSYSTEM { linear: false, strictTearingSet: BackendDAE::TearingSet { innerEquations, residualequations: tornEqs, .. }, .. }, tail: rest }, _) => {
                    let mut numAdd: i32 = 0;
                    let mut numMul: i32 = 0;
                    let mut numDiv: i32 = 0;
                    let mut numTrig: i32 = 0;
                    let mut numRel: i32 = 0;
                    let mut numOth: i32 = 0;
                    let mut numFuncs: i32 = 0;
                    let mut numLog: i32 = 0;
                    let mut otherEqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut comp: Arc<BackendDAE::StrongComponent>;
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut compInfo: Arc<BackendDAE::CompInfo>;
                    let mut torn: Arc<BackendDAE::CompInfo>;
                    let mut other: Arc<BackendDAE::CompInfo>;
                    let mut eqnlst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut explst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    comp = listHead(inComps.clone())?;
                    eqns = BackendEquation::getEqnsFromEqSystem(isyst.clone());
                    let _ = BackendVariable::daeVars(isyst.clone());
                    eqnlst = BackendEquation::getList(tornEqs.clone(), eqns.clone());
                    explst = List::map(eqnlst.clone(), Arc::new(BackendEquation::getEquationRHS));
                    let (_, (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7)) = Expression::traverseExpList(explst.clone(), Arc::new({ let __pe_b1 = ishared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, __pe_b1.clone(), __pe_a2) }), (0, 0, 0, 0, 0, 0, 0, 0))?;
                    numAdd = __pa0.clone();
                    numMul = __pa1.clone();
                    numDiv = __pa2.clone();
                    numTrig = __pa3.clone();
                    numRel = __pa4.clone();
                    numLog = __pa5.clone();
                    numOth = __pa6.clone();
                    numFuncs = __pa7.clone();
                    torn = Arc::new(BackendDAE::CompInfo::COUNTER { comp: comp.clone(), numAdds: numAdd.clone(), numMul: numMul.clone(), numDiv: numDiv.clone(), numTrig: numTrig.clone(), numRelations: numRel.clone(), numLog: numLog.clone(), numOth: numOth.clone(), funcCalls: numFuncs.clone() });
                    (otherEqs, _, _) = List::map_3(innerEquations.clone(), Arc::new(BackendDAEUtil::getEqnAndVarsFromInnerEquation));
                    eqnlst = BackendEquation::getList(otherEqs.clone(), eqns.clone());
                    explst = List::map(eqnlst.clone(), Arc::new(BackendEquation::getEquationRHS));
                    let (_, (__pa8, __pa9, __pa10, __pa11, __pa12, __pa13, __pa14, __pa15)) = Expression::traverseExpList(explst.clone(), Arc::new({ let __pe_b1 = ishared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, __pe_b1.clone(), __pe_a2) }), (0, 0, 0, 0, 0, 0, 0, 0))?;
                    numAdd = __pa8.clone();
                    numMul = __pa9.clone();
                    numDiv = __pa10.clone();
                    numTrig = __pa11.clone();
                    numRel = __pa12.clone();
                    numLog = __pa13.clone();
                    numOth = __pa14.clone();
                    numFuncs = __pa15.clone();
                    other = Arc::new(BackendDAE::CompInfo::COUNTER { comp: comp.clone(), numAdds: numAdd.clone(), numMul: numMul.clone(), numDiv: numDiv.clone(), numTrig: numTrig.clone(), numRelations: numRel.clone(), numLog: numLog.clone(), numOth: numOth.clone(), funcCalls: numFuncs.clone() });
                    compInfo = Arc::new(BackendDAE::CompInfo::TORN_ANALYSE { comp: comp.clone(), tornEqs: torn.clone(), otherEqs: other.clone(), tornSize: (tornEqs.clone().len() as i32) });
                    Ok(countOperationstraverseComps(rest.clone(), isyst.clone(), ishared.clone(), cons(compInfo.clone(), compInfosIn.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: comp, tail: rest }, _) => {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("not supported component: ")); __mm_s.push_str(&*BackendDump::strongComponentString(comp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(countOperationstraverseComps(rest.clone(), isyst.clone(), ishared.clone(), compInfosIn.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(compInfosOut)
}

fn getNumJacEntries(mut inJac: Arc<BackendDAE::Jacobian>) -> i32 {
    let mut numEntries: i32 = 0;
    numEntries = (::match_deref::match_deref! { match &(inJac.clone()) {
        Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: None } => {
            -1
        },
        Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: Some(jac) } => {
            (jac.clone().len() as i32)
        },
        Deref @ BackendDAE::Jacobian::EMPTY_JACOBIAN => {
            -1
        },
        Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: None, .. } => {
            -1
        },
        Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: Some((_, _, vars1, vars2, _, _)), .. } if ((vars1.clone().len() as i32) == (vars2.clone().len() as i32)) => {
            (vars1.clone().len() as i32)
        },
        _ => {
            println!("{}", (literal!("another JAC\n")).clone());
            -1
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    numEntries
}

fn countOperationsJac(mut inJac: Arc<BackendDAE::Jacobian>, mut shared: Arc<BackendDAE::Shared>, mut compInfoIn: Arc<BackendDAE::CompInfo>) -> Result<Arc<BackendDAE::CompInfo>> {
    let mut compInfoOut: Arc<BackendDAE::CompInfo>;
    compInfoOut = (::match_deref::match_deref! { match &((inJac.clone(), compInfoIn.clone())) {
        (Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: None }, _) => {
            compInfoIn.clone()
        },
        (Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: Some(jac) }, Deref @ BackendDAE::CompInfo::COUNTER { funcCalls: numFuncs, numOth, numLog, numRelations: numRel, numTrig, numDiv, numMul, numAdds: numAdd, comp }) => {
            let mut numFuncs = (*numFuncs).clone();
            let mut numOth = (*numOth).clone();
            let mut numLog = (*numLog).clone();
            let mut numRel = (*numRel).clone();
            let mut numTrig = (*numTrig).clone();
            let mut numDiv = (*numDiv).clone();
            let mut numMul = (*numMul).clone();
            let mut numAdd = (*numAdd).clone();
            (numAdd, numMul, numDiv, numTrig, numRel, numLog, numOth, numFuncs) = List::fold(jac.clone(), Arc::new({ let __pe_b1 = shared.clone(); move |__pe_a0, __pe_a2| countOperationsJac1(__pe_a0, __pe_b1.clone(), __pe_a2) }), (numAdd.clone(), numMul.clone(), numDiv.clone(), numOth.clone(), numTrig.clone(), numRel.clone(), numLog.clone(), numFuncs.clone()));
            Arc::new(BackendDAE::CompInfo::COUNTER { comp: comp.clone(), numAdds: numAdd.clone(), numMul: numMul.clone(), numDiv: numDiv.clone(), numTrig: numTrig.clone(), numRelations: numRel.clone(), numLog: numLog.clone(), numOth: numOth.clone(), funcCalls: numFuncs.clone() })
        },
        (_, _) => {
            compInfoIn.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(compInfoOut)
}

fn countOperationsJac1(mut inJac: (i32, i32, Arc<BackendDAE::Equation>), mut shared: Arc<BackendDAE::Shared>, mut inTpl: (i32, i32, i32, i32, i32, i32, i32, i32)) -> Result<(i32, i32, i32, i32, i32, i32, i32, i32)> {
    let mut outTpl: (i32, i32, i32, i32, i32, i32, i32, i32);
    (_, outTpl) = BackendEquation::traverseExpsOfEquation(Util::tuple33(inJac.clone()), Arc::new({ let __pe_b1 = shared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, __pe_b1.clone(), __pe_a2) }), inTpl.clone())?;
    Ok(outTpl)
}

pub fn countOperationsExp(mut inExp: Arc<DAE::Exp>, mut shared: Arc<BackendDAE::Shared>, mut inTpl: (i32, i32, i32, i32, i32, i32, i32, i32)) -> Result<(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (i32, i32, i32, i32, i32, i32, i32, i32);
    (outExp, outTpl) = Expression::traverseExpBottomUp(inExp.clone(), Arc::new({ let __pe_b1 = shared.clone(); move |__pe_a0, __pe_a2| traversecountOperationsExp(__pe_a0, __pe_b1.clone(), __pe_a2) }), inTpl.clone())?;
    Ok((outExp, outTpl))
}

fn traversecountOperationsExp(mut inExp: Arc<DAE::Exp>, mut shared: Arc<BackendDAE::Shared>, mut inTuple: (i32, i32, i32, i32, i32, i32, i32, i32)) -> Result<(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTuple: (i32, i32, i32, i32, i32, i32, i32, i32);
    (outExp, outTuple) = 'mc: {
        let __mc_input = (inExp.clone(), inTuple.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::BINARY { operator: op, .. }, _) => {
                    let mut tpl: (i32, i32, i32, i32, i32, i32, i32, i32);
                    tpl = countOperator(op.clone(), inTuple.clone())?;
                    Ok((e.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::UNARY { operator: op, .. }, _) => {
                    let mut tpl: (i32, i32, i32, i32, i32, i32, i32, i32);
                    tpl = countOperator(op.clone(), inTuple.clone())?;
                    Ok((e.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::LBINARY { operator: op, .. }, _) => {
                    let mut tpl: (i32, i32, i32, i32, i32, i32, i32, i32);
                    tpl = countOperator(op.clone(), inTuple.clone())?;
                    Ok((e.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::LUNARY { operator: op, .. }, _) => {
                    let mut tpl: (i32, i32, i32, i32, i32, i32, i32, i32);
                    tpl = countOperator(op.clone(), inTuple.clone())?;
                    Ok((e.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::RELATION { operator: op, .. }, _) => {
                    let mut tpl: (i32, i32, i32, i32, i32, i32, i32, i32);
                    tpl = countOperator(op.clone(), inTuple.clone())?;
                    Ok((e.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::IFEXP { expElse: exp2, expThen: exp1, expCond: cond }, _) => {
                    let mut i1: i32 = 0;
                    let mut i2: i32 = 0;
                    let mut i3: i32 = 0;
                    let mut i4: i32 = 0;
                    let mut i5: i32 = 0;
                    let mut i6: i32 = 0;
                    let mut i7: i32 = 0;
                    let mut i8: i32 = 0;
                    let mut tpl: (i32, i32, i32, i32, i32, i32, i32, i32);
                    (_, tpl) = traversecountOperationsExp(exp1.clone(), shared.clone(), inTuple.clone())?;
                    (_, tpl) = traversecountOperationsExp(exp2.clone(), shared.clone(), tpl.clone())?;
                    let (_, (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7)) = traversecountOperationsExp(cond.clone(), shared.clone(), tpl.clone())?;
                    i1 = __pa0.clone();
                    i2 = __pa1.clone();
                    i3 = __pa2.clone();
                    i4 = __pa3.clone();
                    i5 = __pa4.clone();
                    i6 = __pa5.clone();
                    i7 = __pa6.clone();
                    i8 = __pa7.clone();
                    Ok((e.clone(), (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone() + 1, i7.clone(), i8.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::RECORD { exps: expLst, .. }, _) => {
                    let mut tpl: (i32, i32, i32, i32, i32, i32, i32, i32);
                    (_, tpl) = Expression::traverseExpList(expLst.clone(), Arc::new({ let __pe_b1 = shared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, __pe_b1.clone(), __pe_a2) }), inTuple.clone())?;
                    Ok((e.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::ARRAY { array: expLst, .. }, _) => {
                    let mut tpl: (i32, i32, i32, i32, i32, i32, i32, i32);
                    (_, tpl) = Expression::traverseExpList(expLst.clone(), Arc::new({ let __pe_b1 = shared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, __pe_b1.clone(), __pe_a2) }), inTuple.clone())?;
                    Ok((e.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::TUPLE { PR: expLst }, _) => {
                    let mut tpl: (i32, i32, i32, i32, i32, i32, i32, i32);
                    (_, tpl) = Expression::traverseExpList(expLst.clone(), Arc::new({ let __pe_b1 = shared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, __pe_b1.clone(), __pe_a2) }), inTuple.clone())?;
                    Ok((e.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: opName }, .. }, (i1, i2, i3, i4, i5, i6, i7, i8)) => {
                    if !((stringEq((opName.clone()).clone(), (literal!("sin")).clone()) || stringEq((opName.clone()).clone(), (literal!("cos")).clone()) || stringEq((opName.clone()).clone(), (literal!("tan")).clone()))) { bail!("guard") }
                    Ok((e.clone(), (i1.clone(), i2.clone(), i3.clone(), i4.clone() + 1, i5.clone(), i6.clone(), i7.clone(), i8.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, (i1, i2, i3, i4, i5, i6, i7, i8)) => {
                    Ok((e.clone(), (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }, .. }, (i1, i2, i3, i4, i5, i6, i7, i8)) => {
                    Ok((e.clone(), (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone() + 1, i8.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, (i1, i2, i3, i4, i5, i6, i7, i8)) => {
                    Ok((e.clone(), (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone() + 1)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. }, (i1, i2, i3, i4, i5, i6, i7, i8)) => {
                    Ok((e.clone(), (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone() + 1)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path, .. }, _) => {
                    let mut func: DAE::Function;
                    let mut i1: i32 = 0;
                    let mut i2: i32 = 0;
                    let mut i3: i32 = 0;
                    let mut i4: i32 = 0;
                    let mut i5: i32 = 0;
                    let mut i6: i32 = 0;
                    let mut i7: i32 = 0;
                    let mut i8: i32 = 0;
                    let mut elemLst: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    func = DAEUtil::getNamedFunction(path.clone(), BackendDAEUtil::getFunctions(shared.clone())?)?;
                    elemLst = DAEUtil::getFunctionElements(func.clone())?;
                    (i1, i2, i3, i4, i5, i6, i7, i8) = countOperationsInFunction(elemLst.clone(), shared.clone(), inTuple.clone())?;
                    Ok((e.clone(), (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone() + 1)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outTuple))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn countOperationsInFunction(mut elemLst: Arc<metamodelica::List<Arc<DAE::Element>>>, mut shared: Arc<BackendDAE::Shared>, mut inTpl: (i32, i32, i32, i32, i32, i32, i32, i32)) -> Result<(i32, i32, i32, i32, i32, i32, i32, i32)> {
    let mut outTpl: (i32, i32, i32, i32, i32, i32, i32, i32);
    outTpl = 'mc: {
        let __mc_input = elemLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(inTpl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: stmts }, .. }, tail: rest } => {
                    let mut tpl: (i32, i32, i32, i32, i32, i32, i32, i32);
                    (_, tpl) = DAEUtil::traverseDAEEquationsStmts(stmts.clone(), Arc::new({ let __pe_b1 = shared.clone(); move |__pe_a0, __pe_a2| traversecountOperationsExp(__pe_a0, __pe_b1.clone(), __pe_a2) }), inTpl.clone());
                    Ok(countOperationsInFunction(rest.clone(), shared.clone(), tpl.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::EQUATION { scalar: exp2, exp: exp1, .. }, tail: rest } => {
                    let mut tpl: (i32, i32, i32, i32, i32, i32, i32, i32);
                    (_, tpl) = traversecountOperationsExp(exp1.clone(), shared.clone(), inTpl.clone())?;
                    (_, tpl) = traversecountOperationsExp(exp2.clone(), shared.clone(), tpl.clone())?;
                    Ok(countOperationsInFunction(rest.clone(), shared.clone(), tpl.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::COMPLEX_EQUATION { rhs: exp2, lhs: exp1, .. }, tail: rest } => {
                    let mut tpl: (i32, i32, i32, i32, i32, i32, i32, i32);
                    (_, tpl) = traversecountOperationsExp(exp1.clone(), shared.clone(), inTpl.clone())?;
                    (_, tpl) = traversecountOperationsExp(exp2.clone(), shared.clone(), tpl.clone())?;
                    Ok(countOperationsInFunction(rest.clone(), shared.clone(), tpl.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(countOperationsInFunction(rest.clone(), shared.clone(), inTpl.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTpl)
}

fn countOperator(mut op: DAE::Operator, mut inTpl: (i32, i32, i32, i32, i32, i32, i32, i32)) -> Result<(i32, i32, i32, i32, i32, i32, i32, i32)> {
    let mut outTpl: (i32, i32, i32, i32, i32, i32, i32, i32);
    outTpl = (match (op.clone(), inTpl.clone()) {
        (DAE::Operator::ADD { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone() + 1, i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::SUB { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone() + 1, i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::MUL { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone() + 1, i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::DIV { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone(), i3.clone() + 1, i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::POW { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone() + 1, i8.clone())
        },
        (DAE::Operator::UMINUS { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone() + 1, i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::UMINUS_ARR { ty: ref tp }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            let mut i: i32 = 0;
            i = Expression::sizeOf(tp.clone())?;
            (i1.clone() + i.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::ADD_ARR { ty: ref tp }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            let mut i: i32 = 0;
            i = Expression::sizeOf(tp.clone())?;
            (i1.clone() + i.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::SUB_ARR { ty: ref tp }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            let mut i: i32 = 0;
            i = Expression::sizeOf(tp.clone())?;
            (i1.clone() + i.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::MUL_ARR { ty: ref tp }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            let mut i: i32 = 0;
            i = Expression::sizeOf(tp.clone())?;
            (i1.clone(), i2.clone() + i.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::DIV_ARR { ty: ref tp }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            let mut i: i32 = 0;
            i = Expression::sizeOf(tp.clone())?;
            (i1.clone(), i2.clone(), i3.clone() + i.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::MUL_ARRAY_SCALAR { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone() + 1, i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::ADD_ARRAY_SCALAR { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone() + 1, i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::SUB_SCALAR_ARRAY { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone() + 1, i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::MUL_SCALAR_PRODUCT { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone() + 1, i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::MUL_MATRIX_PRODUCT { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone() + 1, i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::DIV_ARRAY_SCALAR { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone(), i3.clone() + 1, i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::DIV_SCALAR_ARRAY { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone(), i3.clone() + 1, i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::POW_ARRAY_SCALAR { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone() + 1, i8.clone())
        },
        (DAE::Operator::POW_SCALAR_ARRAY { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone() + 1, i8.clone())
        },
        (DAE::Operator::POW_ARR { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone() + 1, i8.clone())
        },
        (DAE::Operator::POW_ARR2 { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone() + 1, i8.clone())
        },
        (DAE::Operator::AND { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone() + 1, i7.clone(), i8.clone())
        },
        (DAE::Operator::OR { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone() + 1, i7.clone(), i8.clone())
        },
        (DAE::Operator::NOT { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone() + 1, i7.clone(), i8.clone())
        },
        (DAE::Operator::LESS { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone() + 1, i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::LESSEQ { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone() + 1, i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::GREATER { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone() + 1, i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::GREATEREQ { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone() + 1, i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::EQUAL { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone() + 1, i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::NEQUAL { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone() + 1, i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::USERDEFINED { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone() + 1, i7.clone(), i8.clone())
        },
        _ => {
            println!("{}", (literal!("not supported operator\n")).clone());
            inTpl.clone()
        },
    });
    Ok(outTpl)
}

// =============================================================================
// simplify if equations
//
// =============================================================================
pub fn simplifyIfEquations(mut dae: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut odae: Arc<BackendDAE::BackendDAE>;
    odae = BackendDAEUtil::mapEqSystem(dae.clone(), Arc::new(simplifyIfEquationsWork))?;
    Ok(odae)
}

fn simplifyIfEquationsWork(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    (osyst, oshared) = 'mc: {
        let __mc_input = (isyst.clone(), ishared.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (syst @ Deref @ BackendDAE::EqSystem { orderedEqs: eqns, .. }, shared @ Deref @ BackendDAE::Shared { initialEqs, globalKnownVars, .. }) => {
                    let mut eqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut asserts: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut initial_asserts: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut systChanged: bool = false;
                    let mut syst = (*syst).clone();
                    let mut shared = (*shared).clone();
                    eqnslst = BackendEquation::equationList(eqns.clone());
                    (eqnslst, asserts, systChanged) = List::fold31(eqnslst.clone().reverse(), Arc::new(simplifyIfEquationsFinder), globalKnownVars.clone(), metamodelica::nil(), metamodelica::nil(), false);
                    todo!("unhandled field-assign shape: syst.orderedEqs");
                    eqnslst = BackendEquation::equationList(initialEqs.clone());
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(List::fold31(eqnslst.clone().reverse(), Arc::new(simplifyIfEquationsFinder), globalKnownVars.clone(), metamodelica::nil(), metamodelica::nil(), systChanged.clone())) {
                        (__pa0, __pa1, true) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    eqnslst = __pa0.clone();
                    initial_asserts = __pa1.clone();
                    todo!("unhandled field-assign shape: shared.initialEqs");
                    syst = BackendDAEUtil::clearEqSyst(syst.clone())?;
                    syst = BackendEquation::requationsAddDAE(asserts.clone(), syst.clone())?;
                    Ok((syst.clone(), shared.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((isyst.clone(), ishared.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((osyst, oshared))
}

fn simplifyIfEquationsFinder(mut inElem: Arc<BackendDAE::Equation>, mut inConstArg: BackendDAE::Variables, mut acc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut asserts: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut b: bool) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool)> {
    let mut acc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = acc;
    let mut asserts: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = asserts;
    let mut b: bool = b;
    (acc, asserts, b) = 'mc: {
        let __mc_input = (inElem.clone(), inConstArg.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::IF_EQUATION { attr, source, eqnsfalse: eqnslst, eqnstrue: eqnslstlst, conditions: explst }, globalKnownVars) => {
                    let mut asserts1: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut explst = (*explst).clone();
                    let mut acc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = acc.clone();
                    (explst, _) = Expression::traverseExpList(explst.clone(), Arc::new(simplifyEvaluatedParamter), (globalKnownVars.clone(), false))?;
                    explst = ExpressionSimplify::simplifyList(explst.clone())?;
                    (acc, asserts1) = simplifyIfEquation(explst.clone(), eqnslstlst.clone(), eqnslst.clone(), metamodelica::nil(), metamodelica::nil(), source.clone(), globalKnownVars.clone(), acc.clone(), attr.clone())?;
                    asserts1 = listAppend(asserts.clone(), asserts1.clone());
                    Ok((acc.clone(), asserts1.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eqn, globalKnownVars) => {
                    let mut eqn = (*eqn).clone();
                    let mut b: bool = b.clone();
                    let (__pa0, (_, __pa1)) = BackendEquation::traverseExpsOfEquation(eqn.clone(), Arc::new(simplifyIfExpevaluatedParamter), (globalKnownVars.clone(), b.clone()))?;
                    eqn = __pa0.clone();
                    b = __pa1.clone();
                    Ok((cons(eqn.clone(), acc.clone()), asserts.clone(), b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((acc, asserts, b))
}

fn simplifyIfExpevaluatedParamter(mut inExp: Arc<DAE::Exp>, mut tpl1: (BackendDAE::Variables, bool)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, bool))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut tpl2: (BackendDAE::Variables, bool);
    (outExp, tpl2) = 'mc: {
        let __mc_input = (inExp.clone(), tpl1.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1 @ Deref @ DAE::Exp::IFEXP { expElse, expThen, expCond: cond }, (globalKnownVars, b)) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut b1: bool = false;
                    let mut cond = (*cond).clone();
                    let (__pa0, (_, __pa1)) = Expression::traverseExpBottomUp(cond.clone(), Arc::new(simplifyEvaluatedParamter), (globalKnownVars.clone(), false))?;
                    cond = __pa0.clone();
                    b1 = __pa1.clone();
                    e2 = if (b1.clone()) {Arc::new(DAE::Exp::IFEXP { expCond: cond.clone(), expThen: expThen.clone(), expElse: expElse.clone() })} else {e1.clone()};
                    (e2, _) = ExpressionSimplify::condsimplify(b1.clone(), e2.clone())?;
                    Ok((e2.clone(), (globalKnownVars.clone(), b.clone() || b1.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), tpl1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, tpl2))
}

fn simplifyEvaluatedParamter(mut inExp: Arc<DAE::Exp>, mut tpl1: (BackendDAE::Variables, bool)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, bool))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut tpl2: (BackendDAE::Variables, bool);
    (outExp, tpl2) = 'mc: {
        let __mc_input = (inExp.clone(), tpl1.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (globalKnownVars, _)) => {
                    let mut v: BackendDAE::Var;
                    let mut e: Arc<DAE::Exp>;
                    (v, _) = BackendVariable::getVarSingle(cr.clone(), globalKnownVars.clone())?;
                    let true = (BackendVariable::hasVarEvaluateAnnotationTrue(v.clone()) || Flags::getConfigBool(Flags::EVALUATE_FINAL_PARAMS.clone())? && BackendVariable::isFinalVar(v.clone()) || Flags::getConfigBool(Flags::EVALUATE_PROTECTED_PARAMS.clone())? && BackendVariable::isProtectedVar(v.clone())) else { bail!("pattern mismatch") };
                    e = BackendVariable::varBindExpStartValue(v.clone())?;
                    Ok((e.clone(), (globalKnownVars.clone(), true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), tpl1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, tpl2))
}

fn simplifyIfEquation(mut conditions: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut theneqns: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, mut elseenqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut conditions1: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut theneqns1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, mut source: Arc<DAE::ElementSource>, mut globalKnownVars: BackendDAE::Variables, mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inEqAttr: BackendDAE::EquationAttributes) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut outEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut outAsserts: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    (outEqns, outAsserts) = (::match_deref::match_deref! { match &((conditions.clone(), theneqns.clone(), conditions1.clone(), theneqns1.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut asserts: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            (eqns, asserts, _) = List::fold31(elseenqs.clone().reverse(), Arc::new(simplifyIfEquationsFinder), globalKnownVars.clone(), metamodelica::nil(), metamodelica::nil(), false);
            (listAppend(eqns.clone(), inEqns.clone()), asserts.clone())
        },
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            let mut explst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut eqnslst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>> = metamodelica::nil();
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut elseenqs1: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut asserts: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            explst = conditions1.clone().reverse();
            eqnslst = theneqns1.clone().reverse();
            (elseenqs1, asserts, _) = List::fold31(elseenqs.clone().reverse(), Arc::new(simplifyIfEquationsFinder), globalKnownVars.clone(), metamodelica::nil(), metamodelica::nil(), false);
            elseenqs1 = listAppend(elseenqs1.clone(), asserts.clone());
            (eqnslst, elseenqs1, asserts) = simplifyIfEquationAsserts(explst.clone(), eqnslst.clone(), elseenqs1.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil())?;
            eqns = simplifyIfEquation1(explst.clone(), eqnslst.clone(), elseenqs1.clone(), source.clone(), globalKnownVars.clone(), inEqns.clone(), inEqAttr.clone())?;
            (eqns.clone(), asserts.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::BCONST { bool: true }, tail: _ }, Deref @ metamodelica::List::Cons { head: eqns, tail: _ }, Deref @ metamodelica::List::Nil, _) => {
            let mut asserts: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut eqns = (*eqns).clone();
            (eqns, asserts, _) = List::fold31(eqns.clone().reverse(), Arc::new(simplifyIfEquationsFinder), globalKnownVars.clone(), metamodelica::nil(), metamodelica::nil(), false);
            (listAppend(eqns.clone(), inEqns.clone()), asserts.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::BCONST { bool: true }, tail: _ }, Deref @ metamodelica::List::Cons { head: eqns, tail: _ }, _, _) => {
            let mut explst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut eqnslst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>> = metamodelica::nil();
            let mut elseenqs1: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut asserts: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut eqns = (*eqns).clone();
            explst = conditions1.clone().reverse();
            eqnslst = theneqns1.clone().reverse();
            (elseenqs1, asserts, _) = List::fold31(eqns.clone().reverse(), Arc::new(simplifyIfEquationsFinder), globalKnownVars.clone(), metamodelica::nil(), metamodelica::nil(), false);
            elseenqs1 = listAppend(elseenqs1.clone(), asserts.clone());
            (eqnslst, elseenqs1, asserts) = simplifyIfEquationAsserts(explst.clone(), eqnslst.clone(), elseenqs1.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil())?;
            eqns = simplifyIfEquation1(explst.clone(), eqnslst.clone(), elseenqs1.clone(), source.clone(), globalKnownVars.clone(), inEqns.clone(), inEqAttr.clone())?;
            (eqns.clone(), asserts.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::BCONST { bool: false }, tail: explst }, Deref @ metamodelica::List::Cons { head: _, tail: eqnslst }, _, _) => {
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut asserts: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            (eqns, asserts) = simplifyIfEquation(explst.clone(), eqnslst.clone(), elseenqs.clone(), conditions1.clone(), theneqns1.clone(), source.clone(), globalKnownVars.clone(), inEqns.clone(), inEqAttr.clone())?;
            (eqns.clone(), asserts.clone())
        },
        (Deref @ metamodelica::List::Cons { head: e, tail: explst }, Deref @ metamodelica::List::Cons { head: eqns, tail: eqnslst }, _, _) => {
            let mut asserts: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut eqns = (*eqns).clone();
            (eqns, asserts, _) = List::fold31(eqns.clone().reverse(), Arc::new(simplifyIfEquationsFinder), globalKnownVars.clone(), metamodelica::nil(), metamodelica::nil(), false);
            eqns = listAppend(eqns.clone(), asserts.clone());
            (eqns, asserts) = simplifyIfEquation(explst.clone(), eqnslst.clone(), elseenqs.clone(), cons(e.clone(), conditions1.clone()), cons(eqns.clone(), theneqns1.clone()), source.clone(), globalKnownVars.clone(), inEqns.clone(), inEqAttr.clone())?;
            (eqns.clone(), asserts.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outEqns, outAsserts))
}

fn simplifyIfEquation1(mut conditions: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut theneqns: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, mut elseenqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut source: Arc<DAE::ElementSource>, mut globalKnownVars: BackendDAE::Variables, mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inEqAttr: BackendDAE::EquationAttributes) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut outEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    outEqns = 'mc: {
        let __mc_input = inEqAttr.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
            let mut crexplst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>> = metamodelica::nil();
            let _ = countEquationsInBranches(theneqns.clone(), elseenqs.clone(), source.clone())?;
            ht = HashTable2::emptyHashTable();
            ht = simplifySolvedIfEqnsElse(elseenqs.clone(), ht.clone())?;
            ht = simplifySolvedIfEqns(conditions.clone().reverse(), theneqns.clone().reverse(), ht.clone())?;
            crexplst = BaseHashTable::hashTableList(ht.clone());
            eqns = simplifySolvedIfEqns2(crexplst.clone(), inEqns.clone(), inEqAttr.clone())?;
            Ok(eqns.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut fbsExp: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut tbsExp: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let _ = countEquationsInBranches(theneqns.clone(), elseenqs.clone(), source.clone())?;
            fbsExp = makeEquationLstToResidualExpLst(elseenqs.clone())?;
            tbsExp = List::map(theneqns.clone(), Arc::new(makeEquationLstToResidualExpLst));
            eqns = makeEquationsFromResiduals(conditions.clone(), tbsExp.clone(), fbsExp.clone(), source.clone(), inEqAttr.clone())?;
            Ok(listAppend(eqns.clone(), inEqns.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(cons(Arc::new(BackendDAE::Equation::IF_EQUATION { conditions: conditions.clone(), eqnstrue: theneqns.clone(), eqnsfalse: elseenqs.clone(), source: source.clone(), attr: inEqAttr.clone() }), inEqns.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outEqns)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn simplifySolvedIfEqns2(mut crexplst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>, mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inEqAttr: BackendDAE::EquationAttributes) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut outEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    outEqns = (::match_deref::match_deref! { match &(crexplst.clone()) {
        Deref @ metamodelica::List::Nil => {
            inEqns.clone()
        },
        Deref @ metamodelica::List::Cons { head: (cr, e), tail: rest } => {
            let mut crexp: Arc<DAE::Exp>;
            crexp = Expression::crefExp(cr.clone())?;
            simplifySolvedIfEqns2(rest.clone(), cons(Arc::new(BackendDAE::Equation::EQUATION { exp: crexp.clone(), scalar: e.clone(), source: DAE::emptyElementSource.clone(), attr: inEqAttr.clone() }), inEqns.clone()), inEqAttr.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEqns)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn simplifySolvedIfEqns(mut conditions: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut theneqns: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, mut iHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr))> {
    let mut oHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
    oHt = (::match_deref::match_deref! { match &((conditions.clone(), theneqns.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            iHt.clone()
        },
        (Deref @ metamodelica::List::Cons { head: c, tail: explst }, Deref @ metamodelica::List::Cons { head: eqns, tail: rest }) => {
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
            ht = simplifySolvedIfEqns1(c.clone(), eqns.clone(), iHt.clone(), HashSet::emptyHashSet())?;
            simplifySolvedIfEqns(explst.clone(), rest.clone(), ht.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(oHt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn simplifySolvedIfEqns1(mut condition: Arc<DAE::Exp>, mut brancheqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut iHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr)), mut iHs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr))> {
    let mut oHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
    oHt = (::match_deref::match_deref! { match &((brancheqns.clone(), iHt.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            iHt.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::EQUATION { scalar: e, exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, .. }, tail: rest }, _) => {
            let mut exp: Arc<DAE::Exp>;
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
            let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let false = (Expression::expHasCref(e.clone(), cr.clone())?) else { bail!("pattern mismatch") };
            hs = BaseHashSet::addUnique(cr.clone(), iHs.clone())?;
            exp = BaseHashTable::get(cr.clone(), iHt.clone())?;
            exp = Arc::new(DAE::Exp::IFEXP { expCond: condition.clone(), expThen: e.clone(), expElse: exp.clone() });
            ht = BaseHashTable::add((cr.clone(), exp.clone()), iHt.clone())?;
            simplifySolvedIfEqns1(condition.clone(), rest.clone(), ht.clone(), hs.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::EQUATION { scalar: e, exp: Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, operator: DAE::Operator::UMINUS { .. } }, .. }, tail: rest }, _) => {
            let mut exp: Arc<DAE::Exp>;
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
            let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut e = (*e).clone();
            let false = (Expression::expHasCref(e.clone(), cr.clone())?) else { bail!("pattern mismatch") };
            hs = BaseHashSet::addUnique(cr.clone(), iHs.clone())?;
            exp = BaseHashTable::get(cr.clone(), iHt.clone())?;
            e = Expression::negate(e.clone())?;
            exp = Arc::new(DAE::Exp::IFEXP { expCond: condition.clone(), expThen: e.clone(), expElse: exp.clone() });
            ht = BaseHashTable::add((cr.clone(), exp.clone()), iHt.clone())?;
            simplifySolvedIfEqns1(condition.clone(), rest.clone(), ht.clone(), hs.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(oHt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn simplifySolvedIfEqnsElse(mut elseenqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut iHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr))> {
    let mut oHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
    oHt = (::match_deref::match_deref! { match &(elseenqs.clone()) {
        Deref @ metamodelica::List::Nil => {
            iHt.clone()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::EQUATION { scalar: e, exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, .. }, tail: rest } if (!(BaseHashTable::hasKey(cr.clone(), iHt.clone()))) => {
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
            let false = (Expression::expHasCref(e.clone(), cr.clone())?) else { bail!("pattern mismatch") };
            ht = BaseHashTable::add((cr.clone(), e.clone()), iHt.clone())?;
            simplifySolvedIfEqnsElse(rest.clone(), ht.clone())?
        },
        Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::EQUATION { scalar: e, exp: Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, operator: DAE::Operator::UMINUS { .. } }, .. }, tail: rest } if (!(BaseHashTable::hasKey(cr.clone(), iHt.clone()))) => {
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
            let mut e = (*e).clone();
            let false = (Expression::expHasCref(e.clone(), cr.clone())?) else { bail!("pattern mismatch") };
            e = Expression::negate(e.clone())?;
            ht = BaseHashTable::add((cr.clone(), e.clone()), iHt.clone())?;
            simplifySolvedIfEqnsElse(rest.clone(), ht.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(oHt)
}

fn simplifyIfEquationAsserts(mut conditions: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut theneqns: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, mut elseenqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut conditions1: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut theneqns1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut otheneqns: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>> = metamodelica::nil();
    let mut oelseenqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut outEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    (otheneqns, oelseenqs, outEqns) = (::match_deref::match_deref! { match &((conditions.clone(), theneqns.clone())) {
        (_, Deref @ metamodelica::List::Nil) => {
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut beqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            (beqns, eqns) = simplifyIfEquationAsserts1(elseenqs.clone(), None, conditions1.clone(), metamodelica::nil(), inEqns.clone())?;
            (theneqns1.clone().reverse(), beqns.clone(), eqns.clone())
        },
        (Deref @ metamodelica::List::Cons { head: e, tail: explst }, Deref @ metamodelica::List::Cons { head: eqns, tail: eqnslst }) => {
            let mut eqns1: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut beqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut eqnslst1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>> = metamodelica::nil();
            let mut eqns = (*eqns).clone();
            (beqns, eqns) = simplifyIfEquationAsserts1(eqns.clone(), Some(e.clone()), conditions1.clone(), metamodelica::nil(), inEqns.clone())?;
            (eqnslst1, eqns1, eqns) = simplifyIfEquationAsserts(explst.clone(), eqnslst.clone(), elseenqs.clone(), cons(e.clone(), conditions1.clone()), cons(beqns.clone(), theneqns1.clone()), eqns.clone())?;
            (eqnslst1.clone(), eqns1.clone(), eqns.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((otheneqns, oelseenqs, outEqns))
}

fn simplifyIfEquationAsserts1(mut brancheqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut condition: Option<Arc<DAE::Exp>>, mut conditions: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut brancheqns1: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut obrancheqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut outEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    (obrancheqns, outEqns) = (::match_deref::match_deref! { match &((brancheqns.clone(), condition.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            (brancheqns1.clone().reverse(), inEqns.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::ALGORITHM { attr: eqAttr, expand: crefExpand, source, alg: Deref @ DAE::Algorithm { statementLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_ASSERT { source: source1, level, msg, cond }, tail: Deref @ metamodelica::List::Nil } }, size }, tail: eqns }, None) => {
            let mut e: Arc<DAE::Exp>;
            let mut beqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut eqns = (*eqns).clone();
            e = List::fold(conditions.clone(), Arc::new(fnptr!(makeIfExp, Arc<DAE::Exp>, Arc<DAE::Exp>)), cond.clone());
            (beqns, eqns) = simplifyIfEquationAsserts1(eqns.clone(), condition.clone(), conditions.clone(), brancheqns1.clone(), cons(Arc::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: Arc::new(DAE::Algorithm { statementLst: list![Arc::new(DAE::Statement::STMT_ASSERT { cond: e.clone(), msg: msg.clone(), level: level.clone(), source: source1.clone() })] }), source: source.clone(), expand: crefExpand.clone(), attr: eqAttr.clone() }), inEqns.clone()))?;
            (beqns.clone(), eqns.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::ALGORITHM { attr: eqAttr, expand: crefExpand, source, alg: Deref @ DAE::Algorithm { statementLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_ASSERT { source: source1, level, msg, cond }, tail: Deref @ metamodelica::List::Nil } }, size }, tail: eqns }, Some(e)) => {
            let mut beqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut eqns = (*eqns).clone();
            let mut e = (*e).clone();
            e = Arc::new(DAE::Exp::IFEXP { expCond: e.clone(), expThen: cond.clone(), expElse: Arc::new(DAE::Exp::BCONST { bool: true }) });
            e = List::fold(conditions.clone(), Arc::new(fnptr!(makeIfExp, Arc<DAE::Exp>, Arc<DAE::Exp>)), e.clone());
            (beqns, eqns) = simplifyIfEquationAsserts1(eqns.clone(), condition.clone(), conditions.clone(), brancheqns1.clone(), cons(Arc::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: Arc::new(DAE::Algorithm { statementLst: list![Arc::new(DAE::Statement::STMT_ASSERT { cond: e.clone(), msg: msg.clone(), level: level.clone(), source: source1.clone() })] }), source: source.clone(), expand: crefExpand.clone(), attr: eqAttr.clone() }), inEqns.clone()))?;
            (beqns.clone(), eqns.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::ALGORITHM { attr: eqAttr, expand: crefExpand, source, alg: Deref @ DAE::Algorithm { statementLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_TERMINATE { source: source1, msg }, tail: Deref @ metamodelica::List::Nil } }, size }, tail: eqns }, None) => {
            let mut e: Arc<DAE::Exp>;
            let mut beqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut eqns = (*eqns).clone();
            e = List::fold(conditions.clone(), Arc::new(fnptr!(makeIfExp, Arc<DAE::Exp>, Arc<DAE::Exp>)), Arc::new(DAE::Exp::BCONST { bool: true }));
            (beqns, eqns) = simplifyIfEquationAsserts1(eqns.clone(), condition.clone(), conditions.clone(), brancheqns1.clone(), cons(Arc::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: Arc::new(DAE::Algorithm { statementLst: list![Arc::new(DAE::Statement::STMT_IF { exp: e.clone(), statementLst: list![Arc::new(DAE::Statement::STMT_TERMINATE { msg: msg.clone(), source: source1.clone() })], else_: Arc::new(openmodelica_frontend_types::DAE::Else::NOELSE), source: source1.clone() })] }), source: source.clone(), expand: crefExpand.clone(), attr: eqAttr.clone() }), inEqns.clone()))?;
            (beqns.clone(), eqns.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::ALGORITHM { attr: eqAttr, expand: crefExpand, source, alg: Deref @ DAE::Algorithm { statementLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_TERMINATE { source: source1, msg }, tail: Deref @ metamodelica::List::Nil } }, size }, tail: eqns }, Some(e)) => {
            let mut beqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut eqns = (*eqns).clone();
            let mut e = (*e).clone();
            e = List::fold(conditions.clone(), Arc::new(fnptr!(makeIfExp, Arc<DAE::Exp>, Arc<DAE::Exp>)), e.clone());
            (beqns, eqns) = simplifyIfEquationAsserts1(eqns.clone(), condition.clone(), conditions.clone(), brancheqns1.clone(), cons(Arc::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: Arc::new(DAE::Algorithm { statementLst: list![Arc::new(DAE::Statement::STMT_IF { exp: e.clone(), statementLst: list![Arc::new(DAE::Statement::STMT_TERMINATE { msg: msg.clone(), source: source1.clone() })], else_: Arc::new(openmodelica_frontend_types::DAE::Else::NOELSE), source: source1.clone() })] }), source: source.clone(), expand: crefExpand.clone(), attr: eqAttr.clone() }), inEqns.clone()))?;
            (beqns.clone(), eqns.clone())
        },
        (Deref @ metamodelica::List::Cons { head: eqn, tail: eqns }, _) => {
            let mut beqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut eqns = (*eqns).clone();
            (beqns, eqns) = simplifyIfEquationAsserts1(eqns.clone(), condition.clone(), conditions.clone(), cons(eqn.clone(), brancheqns1.clone()), inEqns.clone())?;
            (beqns.clone(), eqns.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((obrancheqns, outEqns))
}

fn makeIfExp(mut cond: Arc<DAE::Exp>, mut else_: Arc<DAE::Exp>) -> Arc<DAE::Exp> {
    let mut oExp: Arc<DAE::Exp>;
    oExp = Arc::new(DAE::Exp::IFEXP { expCond: cond.clone(), expThen: Arc::new(DAE::Exp::BCONST { bool: true }), expElse: else_.clone() });
    oExp
}

fn countEquationsInBranches(mut trueBranches: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, mut falseBranch: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut source: Arc<DAE::ElementSource>) -> Result<i32> {
    let mut nrOfEquations: i32 = 0;
    nrOfEquations = 'mc: {
        let __mc_input = falseBranch.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut b: Arc<metamodelica::List<bool>> = metamodelica::nil();
                    let mut nrOfEquationsBranches: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut nrOfEquations: i32 = nrOfEquations.clone();
                    nrOfEquations = BackendEquation::equationLstSize(falseBranch.clone())?;
                    nrOfEquationsBranches = List::map(trueBranches.clone(), Arc::new(BackendEquation::equationLstSize));
                    b = List::map1(nrOfEquationsBranches.clone(), Arc::new(fnptr!(intEq, i32, i32)), nrOfEquations.clone());
                    let true = (List::reduce(b.clone(), Arc::new(fnptr!(boolAnd, bool, bool)))?) else { bail!("pattern mismatch") };
                    Ok(nrOfEquations.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Error::addSourceMessage(Error::IF_EQUATION_MISSING_ELSE.clone(), metamodelica::nil(), ElementSource::getElementSourceFileInfo(source.clone()))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: _ } => {
                    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut eqstr: ArcStr = arcstr::literal!("");
                    let mut nrOfEquationsBranches: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut nrOfEquations: i32 = nrOfEquations.clone();
                    nrOfEquations = BackendEquation::equationLstSize(falseBranch.clone())?;
                    nrOfEquationsBranches = List::map(trueBranches.clone(), Arc::new(BackendEquation::equationLstSize));
                    eqstr = stringDelimitList(List::map(listAppend(trueBranches.clone(), list![falseBranch.clone()]), Arc::new(BackendDump::dumpEqnsStr)), (literal!("\n")).clone());
                    strs = List::map(nrOfEquationsBranches.clone(), Arc::new(fnptr!(intString, i32)));
                    r#str = stringDelimitList(strs.clone(), (literal!(",")).clone());
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*intString(nrOfEquations.clone())); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::IF_EQUATION_UNBALANCED_2.clone(), list![(r#str.clone()).clone(), (eqstr.clone()).clone()], ElementSource::getElementSourceFileInfo(source.clone()))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(nrOfEquations)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn makeEquationLstToResidualExpLst(mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut oExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    oExpLst = 'mc: {
        let __mc_input = eqLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: eq @ Deref @ BackendDAE::Equation::ALGORITHM { source, .. }, tail: rest } => {
                    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (BackendDump::equationString(eq.clone())?).clone();
                    r#str = (Util::stringReplaceChar((r#str.clone()).clone(), (literal!("\n")).clone(), (literal!("")).clone())?).clone();
                    Error::addSourceMessage(Error::IF_EQUATION_WARNING.clone(), list![(r#str.clone()).clone()], ElementSource::getElementSourceFileInfo(source.clone()))?;
                    exps = makeEquationLstToResidualExpLst(rest.clone())?;
                    Ok(exps.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: eq, tail: rest } => {
                    let mut exps1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut exps2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    exps1 = makeEquationToResidualExpLst(eq.clone())?;
                    exps2 = makeEquationLstToResidualExpLst(rest.clone())?;
                    exps = listAppend(exps1.clone(), exps2.clone());
                    Ok(exps.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oExpLst)
}

fn makeEquationToResidualExpLst(mut eq: Arc<BackendDAE::Equation>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut oExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    oExpLst = 'mc: {
        let __mc_input = eq.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::IF_EQUATION { eqnsfalse: fbs, eqnstrue: tbs, conditions: conds, .. } => {
                    let mut fbsExp: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut tbsExp: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
                    fbsExp = makeEquationLstToResidualExpLst(fbs.clone())?;
                    tbsExp = List::map(tbs.clone(), Arc::new(makeEquationLstToResidualExpLst));
                    exps = makeResidualIfExpLst(conds.clone(), tbsExp.clone(), fbsExp.clone())?;
                    Ok(exps.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                elt => {
                    let mut exp: Arc<DAE::Exp>;
                    exp = makeEquationToResidualExp(elt.clone())?;
                    Ok(list![exp.clone()])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oExpLst)
}

fn makeResidualIfExpLst(mut inExp1: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inExpLst2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, mut inExpLst3: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    outExpLst = (::match_deref::match_deref! { match &((inExp1.clone(), inExpLst2.clone(), inExpLst3.clone())) {
        (_, tbs, Deref @ metamodelica::List::Nil) => {
            let true = (List::all(tbs.clone(), Arc::new(listEmpty))) else { bail!("pattern mismatch") };
            metamodelica::nil()
        },
        (conds, tbs, Deref @ metamodelica::List::Cons { head: fb, tail: fbs }) => {
            let mut tbsRest: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
            let mut tbsFirst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut rest_res: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut ifexp: Arc<DAE::Exp>;
            tbsRest = List::map(tbs.clone(), Arc::new(listRest.clone()));
            rest_res = makeResidualIfExpLst(conds.clone(), tbsRest.clone(), fbs.clone())?;
            tbsFirst = List::map(tbs.clone(), Arc::new(listHead.clone()));
            ifexp = Expression::makeNestedIf(conds.clone(), tbsFirst.clone(), fb.clone())?;
            cons(ifexp.clone(), rest_res.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExpLst)
}

pub fn makeEquationToResidualExp(mut eq: Arc<BackendDAE::Equation>) -> Result<Arc<DAE::Exp>> {
    let mut oExp: Arc<DAE::Exp>;
    oExp = 'mc: {
        let __mc_input = eq.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::EQUATION { scalar: e2, exp: e1, .. } => {
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut oExp: Arc<DAE::Exp>;
                    ty = Expression::r#typeof(e1.clone())?;
                    let true = (Types::isIntegerOrRealOrSubTypeOfEither(ty.clone())?) else { bail!("pattern mismatch") };
                    oExp = Expression::expSub(e1.clone(), e2.clone())?;
                    Ok(oExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::ARRAY_EQUATION { right: e2, left: e1, .. } => {
                    let mut oExp: Arc<DAE::Exp>;
                    oExp = Expression::expSub(e1.clone(), e2.clone())?;
                    Ok(oExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::SOLVED_EQUATION { exp: e2, componentRef: cr1, .. } => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut oExp: Arc<DAE::Exp>;
                    e1 = Expression::crefExp(cr1.clone())?;
                    oExp = Expression::expSub(e1.clone(), e2.clone())?;
                    Ok(oExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: oExp, .. } => {
                    Ok(oExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: e2, left: Deref @ DAE::Exp::TUPLE { PR: expl }, .. } => {
                    let mut e: Arc<DAE::Exp>;
                    let mut expl1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut idx: i32 = 0;
                    let mut idxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut oExp: Arc<DAE::Exp>;
                    expl1 = metamodelica::nil();
                    idxs = metamodelica::nil();
                    idx = 1;
                    for mut elem in &*expl.clone() {
                        let mut elem = elem.clone();
                        if Expression::isNotWild(elem.clone()) {
                            idxs = cons(idx.clone(), idxs.clone());
                            expl1 = cons(elem.clone(), expl1.clone());
                        }
                        idx = idx.clone() + 1;
                    }
                    let __pa0 = ::match_deref::match_deref! { match &(expl1.clone()) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e = __pa0.clone();
                    let __pa2 = ::match_deref::match_deref! { match &(idxs.clone()) {
                        Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil } => __pa2.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    idx = __pa2.clone();
                    oExp = Expression::expSub(e.clone(), Arc::new(DAE::Exp::TSUB { exp: e2.clone(), ix: idx.clone(), ty: Expression::r#typeof(e.clone())? }))?;
                    Ok(oExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: e2, left: e1, .. } => {
                    let mut oExp: Arc<DAE::Exp>;
                    oExp = Expression::expSub(e1.clone(), e2.clone())?;
                    Ok(oExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- BackendDAEOptimize.makeEquationToResidualExp failed to transform equation: ")); __mm_s.push_str(&*BackendDump::equationString(eq.clone())?); __mm_s.push_str(&*literal!(" to residual form!")); ArcStr::from(__mm_s) }).clone();
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oExp)
}

fn makeEquationsFromResiduals(mut inExp1: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inExpLst2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, mut inExpLst3: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inSource: Arc<DAE::ElementSource>, mut inEqAttr: BackendDAE::EquationAttributes) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut outExpLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    outExpLst = (::match_deref::match_deref! { match &(inExpLst3.clone()) {
        Deref @ metamodelica::List::Nil => {
            let true = (List::all(inExpLst2.clone(), Arc::new(listEmpty))) else { bail!("pattern mismatch") };
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: fb, tail: fbs } => {
            let mut tbsRest: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
            let mut tbsFirst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut ifexp: Arc<DAE::Exp>;
            let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut rest_res: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut zeroExp: Arc<DAE::Exp>;
            let mut size: i32 = 0;
            size = Expression::sizeOf(Expression::r#typeof(fb.clone())?)?;
            tbsRest = List::map(inExpLst2.clone(), Arc::new(listRest.clone()));
            rest_res = makeEquationsFromResiduals(inExp1.clone(), tbsRest.clone(), fbs.clone(), inSource.clone(), inEqAttr.clone())?;
            tbsFirst = List::map(inExpLst2.clone(), Arc::new(listHead.clone()));
            ifexp = Expression::makeNestedIf(inExp1.clone(), tbsFirst.clone(), fb.clone())?;
            if size.clone() == 1 {
                eq = Arc::new(BackendDAE::Equation::EQUATION { exp: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), scalar: ifexp.clone(), source: inSource.clone(), attr: inEqAttr.clone() });
            } else {
                zeroExp = Expression::createZeroExpression(Expression::r#typeof(fb.clone())?)?;
                eq = Arc::new(BackendDAE::Equation::COMPLEX_EQUATION { size: size.clone(), left: zeroExp.clone(), right: ifexp.clone(), source: inSource.clone(), attr: inEqAttr.clone() });
            }
            cons(eq.clone(), rest_res.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExpLst)
}

// =============================================================================
// simplify semiLinear calls
//
// =============================================================================
pub fn simplifysemiLinear(mut dae: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut odae: Arc<BackendDAE::BackendDAE>;
    odae = BackendDAEUtil::mapEqSystem(dae.clone(), Arc::new(simplifysemiLinearWork))?;
    Ok(odae)
}

fn simplifysemiLinearWork(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    (osyst, oshared) = 'mc: {
        let __mc_input = isyst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                syst @ Deref @ BackendDAE::EqSystem { orderedEqs: eqns, .. } => {
                    let mut eqnslst: Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>> = metamodelica::nil();
                    let mut eqnsarray: metamodelica::Array<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>>;
                    let mut syst = (*syst).clone();
                    let mut eqns = (*eqns).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(BackendEquation::traverseEquationArray_WithUpdate(eqns.clone(), Arc::new(simplifysemiLinearFinder), (metamodelica::nil(), 0, false))?) {
                        (__pa0, (__pa1, _, true)) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    eqns = __pa0.clone();
                    eqnslst = __pa1.clone();
                    eqnsarray = semiLinearSort(eqnslst.clone(), HashTableExpToIndex::emptyHashTable(), 1, arrayCreate(5, metamodelica::nil()))?;
                    eqnsarray = semiLinearSort1(Arc::new(eqnsarray.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), 1, arrayCreate(5, metamodelica::nil()))?;
                    eqnslst = Array::fold(eqnsarray.clone(), Arc::new(semiLinearOptimize), metamodelica::nil());
                    todo!("unhandled field-assign shape: syst.orderedEqs");
                    Ok((BackendDAEUtil::clearEqSyst(syst.clone())?, ishared.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((isyst.clone(), ishared.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((osyst, oshared))
}

fn semiLinearReplaceEqns(mut iTpl: (Arc<BackendDAE::Equation>, i32), mut iEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> {
    let mut oEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut index: i32 = 0;
    (eqn, index) = iTpl.clone();
    if Flags::isSet(Flags::SEMILINEAR.clone())? {
        BackendDump::debugStrEqnStr((literal!("Replace with ")).clone(), eqn.clone(), (literal!("\n")).clone())?;
    }
    oEqns = BackendEquation::setAtIndex(iEqns.clone(), index.clone() + 1, eqn.clone())?;
    Ok(oEqns)
}

fn semiLinearOptimize(mut eqnslst: Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>, mut iAcc: Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>) -> Result<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>> {
    let mut oAcc: Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>> = metamodelica::nil();
    oAcc = 'mc: {
        let __mc_input = eqnslst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } => {
                    Ok(iAcc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
                    let mut ht1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
                    let mut eqnsarray: metamodelica::Array<(Arc<BackendDAE::Equation>, i32)>;
                    let mut explst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    ht = HashTableExpToIndex::emptyHashTable();
                    ht1 = HashTableExpToIndex::emptyHashTable();
                    (ht, ht1) = semiLinearOptimize1(eqnslst.clone(), 1, ht.clone(), ht1.clone())?;
                    explst = List::fold1(BaseHashTable::hashTableKeyList(ht.clone()), Arc::new(fnptr!(semiLinearGetSA, Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<metamodelica::List<Arc<DAE::Exp>>>)), ht1.clone(), metamodelica::nil());
                    eqnsarray = metamodelica::arrayFromVec(eqnslst.clone().into_iter().cloned().collect());
                    Ok(semiLinearOptimize2(explst.clone(), ht.clone(), eqnsarray.clone(), iAcc.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iAcc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oAcc)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn semiLinearOptimize2(mut saLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut iHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), mut IEqnsarray: metamodelica::Array<(Arc<BackendDAE::Equation>, i32)>, mut iAcc: Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>) -> Result<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>> {
    let mut oAcc: Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>> = metamodelica::nil();
    oAcc = 'mc: {
        let __mc_input = saLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(iAcc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: sa, tail: rest } => {
                    let mut sb: Arc<DAE::Exp>;
                    let mut s1: Arc<DAE::Exp>;
                    let mut y: Arc<DAE::Exp>;
                    let mut x: Arc<DAE::Exp>;
                    let mut explst: Arc<metamodelica::List<(Arc<DAE::Exp>, i32, Arc<DAE::ElementSource>)>> = metamodelica::nil();
                    let mut acc: Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>> = metamodelica::nil();
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut eqn1: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut i1: i32 = 0;
                    let mut index: i32 = 0;
                    let mut index1: i32 = 0;
                    let mut path: Arc<Absyn::Path>;
                    let mut attr: Arc<DAE::CallAttributes>;
                    let mut source: Arc<DAE::ElementSource>;
                    let mut source1: Arc<DAE::ElementSource>;
                    let mut eqAttr: BackendDAE::EquationAttributes;
                    i1 = BaseHashTable::get(sa.clone(), iHt.clone())?;
                    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(IEqnsarray.borrow()[(i1.clone()-1) as usize].clone()) {
                        (Deref @ BackendDAE::Equation::EQUATION { attr: __pa0, source: __pa1, scalar: Deref @ DAE::Exp::CALL { attr: __pa2, expLst: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Nil } } }, path: __pa5 }, exp: __pa6 }, __pa7) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    eqAttr = __pa0.clone();
                    source = __pa1.clone();
                    attr = __pa2.clone();
                    x = __pa3.clone();
                    s1 = __pa4.clone();
                    path = __pa5.clone();
                    y = __pa6.clone();
                    index = __pa7.clone();
                    (sb, source1, index1, explst) = semiLinearOptimize3(s1.clone(), source.clone(), index.clone(), iHt.clone(), IEqnsarray.clone(), metamodelica::nil())?;
                    eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: s1.clone(), scalar: Arc::new(DAE::Exp::IFEXP { expCond: Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("noEvent")).clone() }), expLst: list![Arc::new(DAE::Exp::RELATION { exp1: x.clone(), operator: DAE::Operator::GREATEREQ { ty: DAE::T_REAL_DEFAULT.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), index: -1, optionExpisASUB: None })], attr: DAE::callAttrBuiltinBool.clone() }), expThen: sa.clone(), expElse: sb.clone() }), source: source.clone(), attr: eqAttr.clone() });
                    eqn1 = Arc::new(BackendDAE::Equation::EQUATION { exp: y.clone(), scalar: Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: list![x.clone(), sa.clone(), sb.clone()], attr: attr.clone() }), source: source1.clone(), attr: eqAttr.clone() });
                    acc = semiLinearOptimize4(explst.clone(), cons((eqn1.clone(), index1.clone()), iAcc.clone()), eqAttr.clone())?;
                    Ok(semiLinearOptimize2(rest.clone(), iHt.clone(), IEqnsarray.clone(), cons((eqn.clone(), index.clone()), acc.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(semiLinearOptimize2(rest.clone(), iHt.clone(), IEqnsarray.clone(), iAcc.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oAcc)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn semiLinearOptimize4(mut explst: Arc<metamodelica::List<(Arc<DAE::Exp>, i32, Arc<DAE::ElementSource>)>>, mut iAcc: Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>, mut inEqAttr: BackendDAE::EquationAttributes) -> Result<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>> {
    let mut oAcc: Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>> = metamodelica::nil();
    oAcc = (::match_deref::match_deref! { match &(explst.clone()) {
        Deref @ metamodelica::List::Nil => {
            iAcc.clone()
        },
        Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } => {
            iAcc.clone()
        },
        Deref @ metamodelica::List::Cons { head: (s2, index, source), tail: rest @ Deref @ metamodelica::List::Cons { head: (s1, _, _), tail: _ } } => {
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: s2.clone(), scalar: s1.clone(), source: source.clone(), attr: inEqAttr.clone() });
            semiLinearOptimize4(rest.clone(), cons((eqn.clone(), index.clone()), iAcc.clone()), inEqAttr.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(oAcc)
}

fn semiLinearOptimize3(mut exp: Arc<DAE::Exp>, mut isource: Arc<DAE::ElementSource>, mut iIndex: i32, mut iHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), mut IEqnsarray: metamodelica::Array<(Arc<BackendDAE::Equation>, i32)>, mut iAcc: Arc<metamodelica::List<(Arc<DAE::Exp>, i32, Arc<DAE::ElementSource>)>>) -> Result<(Arc<DAE::Exp>, Arc<DAE::ElementSource>, i32, Arc<metamodelica::List<(Arc<DAE::Exp>, i32, Arc<DAE::ElementSource>)>>)> {
    let mut slast: Arc<DAE::Exp>;
    let mut osource: Arc<DAE::ElementSource>;
    let mut oIndex: i32 = 0;
    let mut oAcc: Arc<metamodelica::List<(Arc<DAE::Exp>, i32, Arc<DAE::ElementSource>)>> = metamodelica::nil();
    (slast, osource, oIndex, oAcc) = 'mc: {
        let __mc_input = ();
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            let mut sb: Arc<DAE::Exp>;
            let mut i: i32 = 0;
            let mut index: i32 = 0;
            let mut source: Arc<DAE::ElementSource>;
            let mut oAcc: Arc<metamodelica::List<(Arc<DAE::Exp>, i32, Arc<DAE::ElementSource>)>> = oAcc.clone();
            i = BaseHashTable::get(exp.clone(), iHt.clone())?;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(IEqnsarray.borrow()[(i.clone()-1) as usize].clone()) {
                (Deref @ BackendDAE::Equation::EQUATION { source: __pa0, scalar: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } } }, .. }, .. }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            source = __pa0.clone();
            sb = __pa1.clone();
            index = __pa2.clone();
            (sb, source, index, oAcc) = semiLinearOptimize3(sb.clone(), source.clone(), index.clone(), iHt.clone(), IEqnsarray.clone(), cons((exp.clone(), iIndex.clone(), source.clone()), iAcc.clone()))?;
            Ok((sb.clone(), source.clone(), index.clone(), oAcc.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            Ok((exp.clone(), isource.clone(), iIndex.clone(), iAcc.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((slast, osource, oIndex, oAcc))
}

fn semiLinearGetSA(mut key: Arc<DAE::Exp>, mut iHt1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), mut iAcc: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Arc<metamodelica::List<Arc<DAE::Exp>>> {
    let mut oAcc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    oAcc = if (BaseHashTable::hasKey(key.clone(), iHt1.clone())) {iAcc.clone()} else {cons(key.clone(), iAcc.clone())};
    oAcc
}

fn semiLinearOptimize1(mut eqnslst: Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>, mut i: i32, mut iHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), mut iHt1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr))) -> Result<((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)))> {
    let mut oHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
    let mut oHt1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
    (oHt, oHt1) = (::match_deref::match_deref! { match &(eqnslst.clone()) {
        Deref @ metamodelica::List::Nil => {
            (iHt.clone(), iHt1.clone())
        },
        Deref @ metamodelica::List::Cons { head: (Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: sa, tail: Deref @ metamodelica::List::Cons { head: sb, tail: Deref @ metamodelica::List::Nil } } }, .. }, .. }, _), tail: rest } => {
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
            let mut ht1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
            ht = BaseHashTable::add((sa.clone(), i.clone()), iHt.clone())?;
            ht1 = BaseHashTable::add((sb.clone(), i.clone()), iHt1.clone())?;
            (ht, ht1) = semiLinearOptimize1(rest.clone(), i.clone() + 1, ht.clone(), ht1.clone())?;
            (ht.clone(), ht1.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((oHt, oHt1))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn semiLinearSort(mut eqnslst: Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>, mut iHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), mut size: i32, mut iEqnsarray: metamodelica::Array<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>>> {
    let mut oEqnsarray: metamodelica::Array<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>>;
    oEqnsarray = 'mc: {
        let __mc_input = eqnslst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(iEqnsarray.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (eqn @ Deref @ BackendDAE::Equation::EQUATION { exp: y, .. }, index), tail: rest } => {
                    let mut i: i32 = 0;
                    let mut eqns: Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>> = metamodelica::nil();
                    let mut eqnsarray: metamodelica::Array<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>>;
                    i = BaseHashTable::get(y.clone(), iHt.clone())?;
                    eqns = iEqnsarray.borrow()[(i.clone()-1) as usize].clone();
                    eqnsarray = {let _arr = iEqnsarray.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = cons((eqn.clone(), index.clone()), eqns.clone()); _arr};
                    Ok(semiLinearSort(rest.clone(), iHt.clone(), size.clone(), eqnsarray.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (eqn @ Deref @ BackendDAE::Equation::EQUATION { exp: y, .. }, index), tail: rest } => {
                    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
                    let mut eqnsarray: metamodelica::Array<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>>;
                    ht = BaseHashTable::add((y.clone(), size.clone()), iHt.clone())?;
                    eqnsarray = if (intGt(size.clone(), (iEqnsarray.clone().borrow().len() as i32))) {Array::expand(5, iEqnsarray.clone(), metamodelica::nil())?} else {iEqnsarray.clone()};
                    eqnsarray = {let _arr = eqnsarray.clone(); _arr.borrow_mut()[(size.clone()-1) as usize] = list![(eqn.clone(), index.clone())]; _arr};
                    Ok(semiLinearSort(rest.clone(), ht.clone(), size.clone() + 1, eqnsarray.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oEqnsarray)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn semiLinearSort1(mut eqnslstlst: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>>>, mut size: i32, mut iEqnsarray: metamodelica::Array<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>>> {
    let mut oEqnsarray: metamodelica::Array<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>>;
    oEqnsarray = (::match_deref::match_deref! { match &(eqnslstlst.clone()) {
        Deref @ metamodelica::List::Nil => {
            iEqnsarray.clone()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Cons { head: tpl, tail: Deref @ metamodelica::List::Nil }, tail: rest } => {
            let mut eqnsarray: metamodelica::Array<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>>;
            eqnsarray = if (intGt(size.clone(), (iEqnsarray.clone().borrow().len() as i32))) {Array::expand(5, iEqnsarray.clone(), metamodelica::nil())?} else {iEqnsarray.clone()};
            eqnsarray = {let _arr = eqnsarray.clone(); _arr.borrow_mut()[(size.clone()-1) as usize] = list![tpl.clone()]; _arr};
            semiLinearSort1(rest.clone(), size.clone() + 1, eqnsarray.clone())?
        },
        Deref @ metamodelica::List::Cons { head: eqns, tail: rest } => {
            let mut size1: i32 = 0;
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
            let mut eqnsarray: metamodelica::Array<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>>;
            ht = HashTableExpToIndex::emptyHashTable();
            (size1, eqnsarray) = semiLinearSort2(eqns.clone(), ht.clone(), size.clone(), iEqnsarray.clone())?;
            semiLinearSort1(rest.clone(), size1.clone(), eqnsarray.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oEqnsarray)
}

fn semiLinearSort2(mut eqnslst: Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>, mut iHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), mut size: i32, mut iEqnsarray: metamodelica::Array<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>>) -> Result<(i32, metamodelica::Array<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>>)> {
    let mut osize: i32 = 0;
    let mut oEqnsarray: metamodelica::Array<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>>;
    (osize, oEqnsarray) = 'mc: {
        let __mc_input = (eqnslst.clone(), iHt.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok((size.clone(), iEqnsarray.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (eqn @ Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: x, tail: _ }, .. }, .. }, index), tail: rest }, _) => {
                    let mut i: i32 = 0;
                    let mut eqns: Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>> = metamodelica::nil();
                    let mut eqnsarray: metamodelica::Array<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>>;
                    i = BaseHashTable::get(x.clone(), iHt.clone())?;
                    eqns = iEqnsarray.borrow()[(i.clone()-1) as usize].clone();
                    eqnsarray = {let _arr = iEqnsarray.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = cons((eqn.clone(), index.clone()), eqns.clone()); _arr};
                    (i, eqnsarray) = semiLinearSort2(rest.clone(), iHt.clone(), size.clone(), eqnsarray.clone())?;
                    Ok((i.clone(), eqnsarray.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (eqn @ Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: x, tail: _ }, .. }, .. }, index), tail: rest }, _) => {
                    let mut i: i32 = 0;
                    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
                    let mut eqnsarray: metamodelica::Array<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>>;
                    ht = BaseHashTable::add((x.clone(), size.clone()), iHt.clone())?;
                    eqnsarray = if (intGt(size.clone(), (iEqnsarray.clone().borrow().len() as i32))) {Array::expand(5, iEqnsarray.clone(), metamodelica::nil())?} else {iEqnsarray.clone()};
                    eqnsarray = {let _arr = eqnsarray.clone(); _arr.borrow_mut()[(size.clone()-1) as usize] = list![(eqn.clone(), index.clone())]; _arr};
                    (i, eqnsarray) = semiLinearSort2(rest.clone(), ht.clone(), size.clone() + 1, eqnsarray.clone())?;
                    Ok((i.clone(), eqnsarray.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((osize, oEqnsarray))
}

fn simplifysemiLinearFinder(mut inEq: Arc<BackendDAE::Equation>, mut inTpl: (Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>, i32, bool)) -> Result<(Arc<BackendDAE::Equation>, (Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>, i32, bool))> {
    let mut outEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut outTpl: (Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>, i32, bool);
    (outEq, outTpl) = 'mc: {
        let __mc_input = (inEq.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { attr: eqAttr, source, scalar: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: x, tail: Deref @ metamodelica::List::Cons { head: sa, tail: Deref @ metamodelica::List::Cons { head: sb, tail: Deref @ metamodelica::List::Nil } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, .. }, exp: y }, (eqnslst, index, _)) => {
                    let true = (Expression::isZero(y.clone())) else { bail!("pattern mismatch") };
                    let true = (Expression::isZero(x.clone())) else { bail!("pattern mismatch") };
                    Ok((Arc::new(BackendDAE::Equation::EQUATION { exp: sa.clone(), scalar: sb.clone(), source: source.clone(), attr: eqAttr.clone() }), (eqnslst.clone(), index.clone() + 1, true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { attr: eqAttr, source, scalar: y, exp: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: x, tail: Deref @ metamodelica::List::Cons { head: sa, tail: Deref @ metamodelica::List::Cons { head: sb, tail: Deref @ metamodelica::List::Nil } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, .. } }, (eqnslst, index, _)) => {
                    let true = (Expression::isZero(y.clone())) else { bail!("pattern mismatch") };
                    let true = (Expression::isZero(x.clone())) else { bail!("pattern mismatch") };
                    Ok((Arc::new(BackendDAE::Equation::EQUATION { exp: sa.clone(), scalar: sb.clone(), source: source.clone(), attr: eqAttr.clone() }), (eqnslst.clone(), index.clone() + 1, true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { attr: eqAttr, source, scalar: Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: x, tail: Deref @ metamodelica::List::Cons { head: sa, tail: Deref @ metamodelica::List::Cons { head: sb, tail: Deref @ metamodelica::List::Nil } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, .. }, .. }, exp: y }, (eqnslst, index, _)) => {
                    let true = (Expression::isZero(y.clone())) else { bail!("pattern mismatch") };
                    let true = (Expression::isZero(x.clone())) else { bail!("pattern mismatch") };
                    Ok((Arc::new(BackendDAE::Equation::EQUATION { exp: sa.clone(), scalar: sb.clone(), source: source.clone(), attr: eqAttr.clone() }), (eqnslst.clone(), index.clone() + 1, true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { attr: eqAttr, source, scalar: y, exp: Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: x, tail: Deref @ metamodelica::List::Cons { head: sa, tail: Deref @ metamodelica::List::Cons { head: sb, tail: Deref @ metamodelica::List::Nil } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, .. }, .. } }, (eqnslst, index, _)) => {
                    let true = (Expression::isZero(y.clone())) else { bail!("pattern mismatch") };
                    let true = (Expression::isZero(x.clone())) else { bail!("pattern mismatch") };
                    Ok((Arc::new(BackendDAE::Equation::EQUATION { exp: sa.clone(), scalar: sb.clone(), source: source.clone(), attr: eqAttr.clone() }), (eqnslst.clone(), index.clone() + 1, true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { attr: eqAttr, source, scalar: Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::UNARY { exp: x, .. }, tail: Deref @ metamodelica::List::Cons { head: sb, tail: Deref @ metamodelica::List::Cons { head: sa, tail: Deref @ metamodelica::List::Nil } } }, path: path @ Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" } }, .. }, exp: y }, (eqnslst, index, _)) => {
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: y.clone(), scalar: Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: list![x.clone(), sa.clone(), sb.clone()], attr: attr.clone() }), source: source.clone(), attr: eqAttr.clone() });
                    if Flags::isSet(Flags::SEMILINEAR.clone())? {
                        BackendDump::debugStrEqnStr((literal!("Found semiLinear ")).clone(), eqn.clone(), (literal!("\n")).clone())?;
                    }
                    Ok((eqn.clone(), (cons((eqn.clone(), index.clone()), eqnslst.clone()), index.clone() + 1, true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { attr: eqAttr, source, scalar: y, exp: Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::UNARY { exp: x, .. }, tail: Deref @ metamodelica::List::Cons { head: sb, tail: Deref @ metamodelica::List::Cons { head: sa, tail: Deref @ metamodelica::List::Nil } } }, path: path @ Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" } }, .. } }, (eqnslst, index, _)) => {
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: y.clone(), scalar: Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: list![x.clone(), sa.clone(), sb.clone()], attr: attr.clone() }), source: source.clone(), attr: eqAttr.clone() });
                    if Flags::isSet(Flags::SEMILINEAR.clone())? {
                        BackendDump::debugStrEqnStr((literal!("Found semiLinear ")).clone(), eqn.clone(), (literal!("\n")).clone())?;
                    }
                    Ok((eqn.clone(), (cons((eqn.clone(), index.clone()), eqnslst.clone()), index.clone() + 1, true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { attr: eqAttr, source, scalar: Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::UNARY { exp: x, .. }, tail: Deref @ metamodelica::List::Cons { head: sb, tail: Deref @ metamodelica::List::Cons { head: sa, tail: Deref @ metamodelica::List::Nil } } }, path: path @ Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" } }, exp: Deref @ DAE::Exp::UNARY { exp: y, .. } }, (eqnslst, index, _)) => {
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: y.clone(), scalar: Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: list![x.clone(), sa.clone(), sb.clone()], attr: attr.clone() }), source: source.clone(), attr: eqAttr.clone() });
                    if Flags::isSet(Flags::SEMILINEAR.clone())? {
                        BackendDump::debugStrEqnStr((literal!("Found semiLinear ")).clone(), eqn.clone(), (literal!("\n")).clone())?;
                    }
                    Ok((eqn.clone(), (cons((eqn.clone(), index.clone()), eqnslst.clone()), index.clone() + 1, true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { attr: eqAttr, source, scalar: Deref @ DAE::Exp::UNARY { exp: y, .. }, exp: Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::UNARY { exp: x, .. }, tail: Deref @ metamodelica::List::Cons { head: sb, tail: Deref @ metamodelica::List::Cons { head: sa, tail: Deref @ metamodelica::List::Nil } } }, path: path @ Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" } } }, (eqnslst, index, _)) => {
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: y.clone(), scalar: Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: list![x.clone(), sa.clone(), sb.clone()], attr: attr.clone() }), source: source.clone(), attr: eqAttr.clone() });
                    if Flags::isSet(Flags::SEMILINEAR.clone())? {
                        BackendDump::debugStrEqnStr((literal!("Found semiLinear ")).clone(), eqn.clone(), (literal!("\n")).clone())?;
                    }
                    Ok((eqn.clone(), (cons((eqn.clone(), index.clone()), eqnslst.clone()), index.clone() + 1, true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { attr: eqAttr, source, scalar: Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::UNARY { exp: x, .. }, tail: Deref @ metamodelica::List::Cons { head: sb, tail: Deref @ metamodelica::List::Cons { head: sa, tail: Deref @ metamodelica::List::Nil } } }, path: path @ Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" } }, exp: y }, (eqnslst, index, _)) => {
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut y = (*y).clone();
                    y = Expression::negate(y.clone())?;
                    eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: y.clone(), scalar: Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: list![x.clone(), sa.clone(), sb.clone()], attr: attr.clone() }), source: source.clone(), attr: eqAttr.clone() });
                    if Flags::isSet(Flags::SEMILINEAR.clone())? {
                        BackendDump::debugStrEqnStr((literal!("Found semiLinear ")).clone(), eqn.clone(), (literal!("\n")).clone())?;
                    }
                    Ok((eqn.clone(), (cons((eqn.clone(), index.clone()), eqnslst.clone()), index.clone() + 1, true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { attr: eqAttr, source, scalar: y, exp: Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::UNARY { exp: x, .. }, tail: Deref @ metamodelica::List::Cons { head: sb, tail: Deref @ metamodelica::List::Cons { head: sa, tail: Deref @ metamodelica::List::Nil } } }, path: path @ Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" } } }, (eqnslst, index, _)) => {
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut y = (*y).clone();
                    y = Expression::negate(y.clone())?;
                    eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: y.clone(), scalar: Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: list![x.clone(), sa.clone(), sb.clone()], attr: attr.clone() }), source: source.clone(), attr: eqAttr.clone() });
                    if Flags::isSet(Flags::SEMILINEAR.clone())? {
                        BackendDump::debugStrEqnStr((literal!("Found semiLinear ")).clone(), eqn.clone(), (literal!("\n")).clone())?;
                    }
                    Ok((eqn.clone(), (cons((eqn.clone(), index.clone()), eqnslst.clone()), index.clone() + 1, true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eqn @ Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, .. }, .. }, (eqnslst, index, _)) => {
                    if Flags::isSet(Flags::SEMILINEAR.clone())? {
                        BackendDump::debugStrEqnStr((literal!("Found semiLinear ")).clone(), eqn.clone(), (literal!("\n")).clone())?;
                    }
                    Ok((eqn.clone(), (cons((eqn.clone(), index.clone()), eqnslst.clone()), index.clone() + 1, true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eqn @ Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, .. }, .. }, (eqnslst, index, _)) => {
                    if Flags::isSet(Flags::SEMILINEAR.clone())? {
                        BackendDump::debugStrEqnStr((literal!("Found semiLinear ")).clone(), eqn.clone(), (literal!("\n")).clone())?;
                    }
                    Ok((eqn.clone(), (cons((eqn.clone(), index.clone()), eqnslst.clone()), index.clone() + 1, true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { attr: eqAttr, source, scalar: Deref @ DAE::Exp::UNARY { exp: x @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, .. }, .. }, exp: y }, (eqnslst, index, _)) => {
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut y = (*y).clone();
                    y = Expression::negate(y.clone())?;
                    eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: y.clone(), scalar: x.clone(), source: source.clone(), attr: eqAttr.clone() });
                    if Flags::isSet(Flags::SEMILINEAR.clone())? {
                        BackendDump::debugStrEqnStr((literal!("Found semiLinear ")).clone(), eqn.clone(), (literal!("\n")).clone())?;
                    }
                    Ok((eqn.clone(), (cons((eqn.clone(), index.clone()), eqnslst.clone()), index.clone() + 1, true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { attr: eqAttr, source, scalar: y, exp: Deref @ DAE::Exp::UNARY { exp: x @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, .. }, .. } }, (eqnslst, index, _)) => {
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut y = (*y).clone();
                    y = Expression::negate(y.clone())?;
                    eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: y.clone(), scalar: x.clone(), source: source.clone(), attr: eqAttr.clone() });
                    if Flags::isSet(Flags::SEMILINEAR.clone())? {
                        BackendDump::debugStrEqnStr((literal!("Found semiLinear ")).clone(), eqn.clone(), (literal!("\n")).clone())?;
                    }
                    Ok((eqn.clone(), (cons((eqn.clone(), index.clone()), eqnslst.clone()), index.clone() + 1, true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eqn, (eqnslst, index, b)) => {
                    Ok((eqn.clone(), (eqnslst.clone(), index.clone() + 1, b.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outEq, outTpl))
}

// =============================================================================
// remove constants stuff
//
// =============================================================================
pub fn removeConstants(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    outDAE = (::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ DAE { UNIQUEIO: systs, derivativeNamePrefix: shared @ BackendDAE::Shared { globalKnownVars, .. }, .. } => {
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut lsteqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut b: bool = false;
            let mut systs = (*systs).clone();
            let mut shared = (*shared).clone();
            let mut globalKnownVars = (*globalKnownVars).clone();
            repl = BackendVarTransform::emptyReplacements();
            repl = BackendVariable::traverseBackendDAEVars(globalKnownVars.clone(), Arc::new(removeConstantsFinder), repl.clone())?;
            let (__pa0, (__pa1, _)) = BackendVariable::traverseBackendDAEVarsWithUpdate(globalKnownVars.clone(), Arc::new(replaceFinalVarTraverser), (repl.clone(), 0))?;
            globalKnownVars = __pa0.clone();
            repl = __pa1.clone();
            if Flags::isSet(Flags::DUMP_CONST_REPL.clone())? {
                BackendVarTransform::dumpReplacements(repl.clone())?;
            }
            lsteqns = BackendEquation::equationList(shared.initialEqs.clone());
            (lsteqns, b) = BackendVarTransform::replaceEquations(lsteqns.clone(), repl.clone(), None)?;
            todo!("unhandled field-assign shape: shared.initialEqs");
            lsteqns = BackendEquation::equationList(shared.removedEqs.clone());
            (lsteqns, b) = BackendVarTransform::replaceEquations(lsteqns.clone(), repl.clone(), None)?;
            todo!("unhandled field-assign shape: shared.removedEqs");
            systs = List::map1(systs.clone(), Arc::new(removeConstantsWork), repl.clone());
            BackendDAE::DAE(systs.clone(), shared.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outDAE)
}

fn removeConstantsWork(mut inEqSystem: Arc<BackendDAE::EqSystem>, mut repl: BackendVarTransform::VariableReplacements) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut outEqSystem: Arc<BackendDAE::EqSystem>;
    outEqSystem = (::match_deref::match_deref! { match &(inEqSystem.clone()) {
        syst @ Deref @ BackendDAE::EqSystem { orderedVars: vars, .. } => {
            let mut lsteqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut b: bool = false;
            let mut syst = (*syst).clone();
            BackendVariable::traverseBackendDAEVarsWithUpdate(vars.clone(), Arc::new(replaceFinalVarTraverser), (repl.clone(), 0))?;
            (lsteqns, b) = BackendVarTransform::replaceEquations(BackendEquation::equationList(syst.orderedEqs.clone()), repl.clone(), None)?;
            if b.clone() {
                assign_field!(syst.orderedEqs = BackendEquation::listEquation(lsteqns.clone())?);
                syst = BackendDAEUtil::clearEqSyst(syst.clone())?;
            }
            (lsteqns, b) = BackendVarTransform::replaceEquations(BackendEquation::equationList(syst.removedEqs.clone()), repl.clone(), None)?;
            if b.clone() {
                assign_field!(syst.removedEqs = BackendEquation::listEquation(lsteqns.clone())?);
            }
            syst.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outEqSystem)
}

fn removeConstantsFinder(mut inVar: BackendDAE::Var, mut inRepl: BackendVarTransform::VariableReplacements) -> Result<(BackendDAE::Var, BackendVarTransform::VariableReplacements)> {
    let mut outVar: BackendDAE::Var;
    let mut outRepl: BackendVarTransform::VariableReplacements;
    (outVar, outRepl) = 'mc: {
        let __mc_input = (inVar.clone(), inRepl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ BackendDAE::Var { bindExp: Some(exp), varKind: BackendDAE::VarKind::CONST, varName, .. }, repl) => {
                    let mut repl_1: BackendVarTransform::VariableReplacements;
                    repl_1 = BackendVarTransform::addReplacement(repl.clone(), varName.clone(), exp.clone(), None)?;
                    Ok((v.clone(), repl_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inVar.clone(), inRepl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVar, outRepl))
}

// =============================================================================
// reaplace edge and change with (b and not pre(b)) and (v <> pre(v))
//
// =============================================================================
pub fn replaceEdgeChange(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    (outDAE, _) = BackendDAEUtil::mapEqSystemAndFold(inDAE.clone(), Arc::new(replaceEdgeChange0), false)?;
    outDAE = replaceEdgeChangeShared(outDAE.clone())?;
    Ok(outDAE)
}

fn replaceEdgeChange0(mut isyst: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut inChanged: bool) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool)> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut outChanged: bool = false;
    (osyst, outChanged) = 'mc: {
        let __mc_input = isyst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::EqSystem { removedEqs, orderedEqs, .. } => {
                    BackendDAEUtil::traverseBackendDAEExpsEqns(orderedEqs.clone(), Arc::new(traverserreplaceEdgeChange), false)?;
                    BackendDAEUtil::traverseBackendDAEExpsEqns(removedEqs.clone(), Arc::new(traverserreplaceEdgeChange), false)?;
                    Ok((isyst.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((isyst.clone(), inChanged.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((osyst, outShared, outChanged))
}

fn traverserreplaceEdgeChange(mut e: Arc<DAE::Exp>, mut b: bool) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut oe: Arc<DAE::Exp>;
    let mut ob: bool = false;
    (oe, ob) = Expression::traverseExpBottomUp(e.clone(), Arc::new(traverserExpreplaceEdgeChange), b.clone())?;
    Ok((oe, ob))
}

fn traverserExpreplaceEdgeChange(mut inExp: Arc<DAE::Exp>, mut inB: bool) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outB: bool = false;
    (outExp, outB) = 'mc: {
        let __mc_input = (inExp.clone(), inB.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "change" }, .. }, _) => {
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    ty = Expression::r#typeof(e.clone())?;
                    Ok((Arc::new(DAE::Exp::RELATION { exp1: e.clone(), operator: DAE::Operator::NEQUAL { ty: ty.clone() }, exp2: Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("pre")).clone() }), expLst: list![e.clone()], attr: Arc::new(DAE::CallAttributes { ty: ty.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL }) }), index: -1, optionExpisASUB: None }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "edge" }, .. }, _) => {
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    ty = Expression::r#typeof(e.clone())?;
                    Ok((Arc::new(DAE::Exp::LBINARY { exp1: e.clone(), operator: DAE::Operator::AND { ty: ty.clone() }, exp2: Arc::new(DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: ty.clone() }, exp: Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("pre")).clone() }), expLst: list![e.clone()], attr: Arc::new(DAE::CallAttributes { ty: ty.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL }) }) }) }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inB.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outB))
}

fn replaceEdgeChangeShared(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    let mut remeqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared>;
    let (__pa0, __pa2, __pa1) = ::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ DAE { UNIQUEIO: __pa0, derivativeNamePrefix: __pa2 @ BackendDAE::Shared { removedEqs: __pa1, .. }, .. } => (__pa0.clone(), __pa2.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    systs = __pa0.clone();
    remeqns = __pa1.clone();
    shared = __pa2.clone();
    BackendDAEUtil::traverseBackendDAEExpsEqns(remeqns.clone(), Arc::new(traverserreplaceEdgeChange), false)?;
    outDAE = BackendDAE::DAE(systs.clone(), shared.clone())?;
    Ok(outDAE)
}

// =============================================================================
// section for preOptModule >>removeLocalKnownVars<<
//
// =============================================================================
pub fn removeLocalKnownVars(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    outDAE = BackendDAEUtil::mapEqSystem(inDAE.clone(), Arc::new(removeLocalKnownVars2))?;
    Ok(outDAE)
}

pub fn removeLocalKnownVars2(mut syst: Arc<BackendDAE::EqSystem>, mut shared: Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> {
    let mut syst: Arc<BackendDAE::EqSystem> = syst;
    let mut shared: Arc<BackendDAE::Shared> = shared;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut potentialLocalKnownVar: BackendDAE::Var;
    let mut potentialGlobalKnownEquation: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut orderedVars: BackendDAE::Variables = syst.orderedVars.clone();
    let mut orderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = syst.orderedEqs.clone();
    let mut lhs: Arc<DAE::Exp>;
    let mut rhs: Arc<DAE::Exp>;
    let mut crefExp: Arc<DAE::Exp>;
    let mut binding: Arc<DAE::Exp>;
    let mut localKnownVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut localKnownEqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eindex: i32 = 0;
    let mut vindex: i32 = 0;
    (_, m, _, _, _) = BackendDAEUtil::getAdjacencyMatrixScalar(syst.clone(), crate::BackendDAE::IndexType::NORMAL, None, BackendDAEUtil::isInitializationDAE(shared.clone()))?;
    m = Array::map(m.clone(), Arc::new(fnptr!(Tearing::deleteNegativeEntries, Arc<metamodelica::List<i32>>)));
    let __range0 = m.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut row in __range0 {
        eindex = eindex.clone() + 1;
        if (row.clone().len() as i32) == 1 {
            let __pa1 = ::match_deref::match_deref! { match &(row.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } => __pa1.clone(),
                _ => bail!("pattern mismatch"),
            } };
            vindex = __pa1.clone();
            potentialLocalKnownVar = BackendVariable::getVarAt(orderedVars.clone(), vindex.clone())?;
            potentialGlobalKnownEquation = BackendEquation::get(orderedEqs.clone(), eindex.clone());
            match '__try3: {
                let (__pa4, __pa5) = ::match_deref::match_deref! { match &(potentialGlobalKnownEquation.clone()) {
                    Deref @ BackendDAE::Equation::EQUATION { scalar: __pa4, exp: __pa5, .. } => (__pa4.clone(), __pa5.clone()),
                    _ => break '__try3 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                } };
                rhs = __pa4.clone();
                lhs = __pa5.clone();
                crefExp = unwrap_break_err!(BackendVariable::varExp(potentialLocalKnownVar.clone()), '__try3);
                (binding, _) = unwrap_break_err!(ExpressionSolve::solve(lhs.clone(), rhs.clone(), crefExp.clone(), None), '__try3);
                potentialLocalKnownVar = BackendVariable::setBindExp(potentialLocalKnownVar.clone(), Some(binding.clone()));
                localKnownVars = cons(vindex.clone(), localKnownVars.clone());
                localKnownEqns = cons(eindex.clone(), localKnownEqns.clone());
                assign_field!(shared.localKnownVars = unwrap_break_err!(BackendVariable::addVar(potentialLocalKnownVar.clone(), shared.localKnownVars.clone()), '__try3));
                Ok::<_, anyhow::Error>((binding.clone(), crefExp.clone(), lhs.clone(), localKnownEqns.clone(), localKnownVars.clone(), potentialLocalKnownVar.clone(), rhs.clone(), shared.clone()))
            } {
                Ok((__try3_o0, __try3_o1, __try3_o2, __try3_o3, __try3_o4, __try3_o5, __try3_o6, __try3_o7)) => {
                    binding = __try3_o0;
                    crefExp = __try3_o1;
                    lhs = __try3_o2;
                    localKnownEqns = __try3_o3;
                    localKnownVars = __try3_o4;
                    potentialLocalKnownVar = __try3_o5;
                    rhs = __try3_o6;
                    shared = __try3_o7;
                }
                Err(_) => {
                    bail!("try/else: outputs not set in else branch");
                }
            }
        }
    }
    localKnownVars = List::sort(localKnownVars.clone(), Arc::new(fnptr!(intLt, i32, i32)))?;
    localKnownEqns = localKnownEqns.clone().reverse();
    for mut var in &*localKnownVars.clone() {
        let mut var = var.clone();
        (orderedVars, _) = BackendVariable::removeVar(var.clone(), orderedVars.clone())?;
    }
    for mut eqn in &*localKnownEqns.clone() {
        let mut eqn = eqn.clone();
        orderedEqs = BackendEquation::delete(eqn.clone(), orderedEqs.clone())?;
    }
    assign_field!(
        syst.m = None,
        syst.mT = None,
        syst.matching = Arc::new(crate::BackendDAE::Matching::NO_MATCHING),
        syst.orderedVars = BackendVariable::listVar(BackendVariable::varList(orderedVars.clone())?),
        syst.orderedEqs = orderedEqs.clone()
    );
    Ok((syst, shared))
}

// =============================================================================
// section for postOptModule >>addInitialStmtsToAlgorithms<<
//
//   Real a[3];
// algorithm       -->  algorithm
//   a[1] := 1.0;         a[1] := $START.a[1];
//                        a[2] := $START.a[2];
//                        a[3] := $START.a[3];
//                        a[1] := 1.0;
// =============================================================================
pub fn addInitialStmtsToAlgorithms(mut inDAE: Arc<BackendDAE::BackendDAE>, mut isInitialSystem: bool) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    outDAE = BackendDAEUtil::mapEqSystem1(inDAE.clone(), Arc::new(addInitialStmtsToAlgorithms1), isInitialSystem.clone())?;
    Ok(outDAE)
}

fn addInitialStmtsToAlgorithms1(mut syst: Arc<BackendDAE::EqSystem>, mut isInitialSystem: bool, mut shared: Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> {
    let mut osyst: Arc<BackendDAE::EqSystem> = syst.clone();
    let mut oshared: Arc<BackendDAE::Shared> = shared.clone();
    let mut ordvars: BackendDAE::Variables;
    let mut allVars: BackendDAE::Variables;
    let mut ordeqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut initEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(osyst.clone()) {
        Deref @ BackendDAE::EqSystem { orderedEqs: __pa0, orderedVars: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ordeqns = __pa0.clone();
    ordvars = __pa1.clone();
    BackendEquation::traverseEquationArray_WithUpdate(ordeqns.clone(), Arc::new(eaddInitialStmtsToAlgorithms1Helper), (ordvars.clone(), isInitialSystem.clone()))?;
    Ok((osyst, oshared))
}

fn eaddInitialStmtsToAlgorithms1Helper(mut inEq: Arc<BackendDAE::Equation>, mut inTpl: (BackendDAE::Variables, bool)) -> Result<(Arc<BackendDAE::Equation>, (BackendDAE::Variables, bool))> {
    let mut outEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut outTpl: (BackendDAE::Variables, bool) = inTpl.clone();
    outEq = (::match_deref::match_deref! { match &((inEq.clone(), inTpl.clone())) {
        (Deref @ BackendDAE::Equation::ALGORITHM { attr, expand: crExpand, source, alg: alg @ Deref @ DAE::Algorithm { statementLst: statements }, size }, (vars, isInitialEquations)) => {
            let mut outputs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut statements = (*statements).clone();
            crlst = CheckModel::checkAndGetAlgorithmOutputs(alg.clone(), source.clone(), crExpand.clone())?;
            outputs = List::map(crlst.clone(), Arc::new(Expression::crefExp));
            statements = expandAlgorithmStmts(statements.clone(), outputs.clone(), vars.clone(), isInitialEquations.clone())?;
            Arc::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: Arc::new(DAE::Algorithm { statementLst: statements.clone() }), source: source.clone(), expand: crExpand.clone(), attr: attr.clone() })
        },
        _ => {
            inEq.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outEq, outTpl))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn expandAlgorithmStmts(mut inAlg: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inOutputs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inVars: BackendDAE::Variables, mut isInitialEquation: bool) -> Result<Arc<metamodelica::List<Arc<DAE::Statement>>>> {
    let mut outAlg: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    outAlg = (::match_deref::match_deref! { match &((inAlg.clone(), inOutputs.clone())) {
        (statements, Deref @ metamodelica::List::Nil) => {
            statements.clone()
        },
        (statements, Deref @ metamodelica::List::Cons { head: out, tail: rest }) => {
            let mut initExp: Arc<DAE::Exp>;
            let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut stmt: Arc<DAE::Statement>;
            let mut type_: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut statements = (*statements).clone();
            cref = Expression::expCref(out.clone())?;
            (vars, _) = BackendVariable::getVar(cref.clone(), inVars.clone())?;
            for mut v in &*vars.clone() {
                let mut v = v.clone();
                type_ = v.varType.clone();
                if BackendVariable::isVarDiscrete(v.clone()) && !(isInitialEquation.clone()) {
                    initExp = Expression::makePureBuiltinCall((literal!("pre")).clone(), list![Expression::crefExp(v.varName.clone())?], type_.clone());
                } else {
                    initExp = Expression::crefExp(ComponentReference::crefPrefixStart(v.varName.clone()))?;
                }
                stmt = Algorithm::makeAssignment(Arc::new(DAE::Exp::CREF { componentRef: v.varName.clone(), ty: type_.clone() }), DAE::Properties::PROP { type_: type_.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }, initExp.clone(), DAE::Properties::PROP { type_: type_.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }, DAE::dummyAttrVar.clone(), openmodelica_frontend_types::SCode::Initial::NON_INITIAL, DAE::emptyElementSource.clone())?;
                statements = cons(stmt.clone(), statements.clone());
            }
            expandAlgorithmStmts(statements.clone(), rest.clone(), inVars.clone(), isInitialEquation.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outAlg)
}

// =============================================================================
// section for expandDerOperator
//
// =============================================================================
pub fn expandDerOperator(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    outDAE = BackendDAEUtil::mapEqSystem(inDAE.clone(), Arc::new(expandDerOperatorWork))?;
    Ok(outDAE)
}

fn expandDerOperatorWork(mut syst: Arc<BackendDAE::EqSystem>, mut shared: Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> {
    let mut syst: Arc<BackendDAE::EqSystem> = syst;
    let mut shared: Arc<BackendDAE::Shared> = shared;
    (syst, shared) = (::match_deref::match_deref! { match &((syst.clone(), shared.clone())) {
        (syst @ Deref @ BackendDAE::EqSystem { orderedEqs: eqns, orderedVars: vars, .. }, Deref @ BackendDAE::Shared { initialEqs: inieqns, .. }) => {
            let mut shared_arr: Mutable::Mutable<Arc<BackendDAE::Shared>>;
            let mut syst = (*syst).clone();
            let mut vars = (*vars).clone();
            shared_arr = Mutable::create(shared.clone());
            (_, vars) = BackendEquation::traverseEquationArray_WithUpdate(eqns.clone(), Arc::new(todo!("PARTEVALFUNCTION of traverserexpandDerEquation: function signature not resolved")), vars.clone())?;
            (_, vars) = BackendEquation::traverseEquationArray_WithUpdate(inieqns.clone(), Arc::new(todo!("PARTEVALFUNCTION of traverserexpandDerEquation: function signature not resolved")), vars.clone())?;
            assign_field!(syst.orderedVars = vars.clone());
            (syst.clone(), Mutable::access(shared_arr.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((syst, shared))
}

fn expandDerExp(mut exp: Arc<DAE::Exp>, mut vars: BackendDAE::Variables, mut inShared: Mutable::Mutable<Arc<BackendDAE::Shared>>) -> Result<(Arc<DAE::Exp>, BackendDAE::Variables)> {
    let mut exp: Arc<DAE::Exp> = exp;
    let mut vars: BackendDAE::Variables = vars;
    let mut failed: bool = false;
    (exp, vars) = 'mc: {
        let __mc_input = exp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (ComponentReference::crefStr(cr.clone())?).clone();
                    r#str = stringAppendList(list![(literal!("The model includes derivatives of order > 1 for: ")).clone(), (r#str.clone()).clone(), (literal!(". That is not supported. Adding 'Real d")).clone(), (r#str.clone()).clone(), (literal!(" = der(")).clone(), (r#str.clone()).clone(), (literal!(");' *might* result in a solvable model")).clone()]);
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e1 @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { .. }, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut vars: BackendDAE::Variables = vars.clone();
                    let mut exp: Arc<DAE::Exp> = exp.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::extendArrExp(e1.clone(), false)?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e2 = __pa0.clone();
                    (exp, vars) = Expression::traverseExpBottomUp(e2.clone(), Arc::new({ let __pe_b2 = inShared.clone(); move |__pe_a0, __pe_a1| expandDerExp(__pe_a0, __pe_a1, __pe_b2.clone()) }), vars.clone())?;
                    Ok((exp.clone(), vars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e1 @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut exp: Arc<DAE::Exp> = exp.clone();
                    let mut vars: BackendDAE::Variables = vars.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::extendArrExp(e1.clone(), false)?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e2 = __pa0.clone();
                    (exp, vars) = Expression::traverseExpBottomUp(e2.clone(), Arc::new({ let __pe_b2 = inShared.clone(); move |__pe_a0, __pe_a1| expandDerExp(__pe_a0, __pe_a1, __pe_b2.clone()) }), vars.clone())?;
                    Ok((exp.clone(), vars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e1 @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } => {
                    let mut v: BackendDAE::Var;
                    let mut e1 = (*e1).clone();
                    let mut vars: BackendDAE::Variables = vars.clone();
                    let mut failed: bool = failed.clone();
                    (v, _) = BackendVariable::getVarSingle(cr.clone(), vars.clone())?;
                    if let Ok((__pa0, __pa1)) = updateStatesVar(vars.clone(), v.clone(), e1.clone()) {
                        vars = __pa0.clone();
                        e1 = __pa1.clone();
                    } else {
                        failed = true;
                        bail!("fail");
                    }
                    Ok((e1.clone(), vars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e1 @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } => {
                    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut vars: BackendDAE::Variables = vars.clone();
                    let false = (failed.clone()) else { bail!("pattern mismatch") };
                    (varlst, _) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    vars = updateStatesVars(vars.clone(), varlst.clone(), false)?;
                    Ok((e1.clone(), vars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut shared: Arc<BackendDAE::Shared>;
                    let mut vars: BackendDAE::Variables = vars.clone();
                    let false = (failed.clone()) else { bail!("pattern mismatch") };
                    (e2, shared) = Differentiate::differentiateExpTime(e1.clone(), vars.clone(), Mutable::access(inShared.clone()))?;
                    let false = (Expression::isZero(e2.clone())) else { bail!("pattern mismatch") };
                    Mutable::update(inShared.clone(), shared.clone());
                    (e2, _) = ExpressionSimplify::simplify(e2.clone())?;
                    (_, vars) = Expression::traverseExpBottomUp(e2.clone(), Arc::new(derCrefsExp), vars.clone())?;
                    Ok((e2.clone(), vars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((exp.clone(), vars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    if failed.clone() {
        bail!("fail");
    }
    Ok((exp, vars))
}

fn derCrefsExp(mut inExp: Arc<DAE::Exp>, mut inVars: BackendDAE::Variables) -> Result<(Arc<DAE::Exp>, BackendDAE::Variables)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outVars: BackendDAE::Variables;
    (outExp, outVars) = 'mc: {
        let __mc_input = (inExp.clone(), inVars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, vars) => {
                    let mut v: BackendDAE::Var;
                    let mut e = (*e).clone();
                    let mut vars = (*vars).clone();
                    (v, _) = BackendVariable::getVarSingle(cr.clone(), vars.clone())?;
                    (vars, e) = updateStatesVar(vars.clone(), v.clone(), e.clone())?;
                    Ok((e.clone(), vars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, vars) => {
                    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut vars = (*vars).clone();
                    (varlst, _) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    vars = updateStatesVars(vars.clone(), varlst.clone(), false)?;
                    Ok((e.clone(), vars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outVars))
}

fn updateStatesVar(mut inVars: BackendDAE::Variables, mut var: BackendDAE::Var, mut iExp: Arc<DAE::Exp>) -> Result<(BackendDAE::Variables, Arc<DAE::Exp>)> {
    let mut outVars: BackendDAE::Variables = inVars.clone();
    let mut oExp: Arc<DAE::Exp> = iExp.clone();
    let mut var1: BackendDAE::Var;
    let mut arg: Arc<DAE::Exp>;
    if BackendVariable::isVarNonDifferentiable(var.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(iExp.clone()) {
            Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, .. } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        arg = __pa0.clone();
        Error::addSourceMessageAndFail(Error::DER_OF_NONDIFFERENTIABLE_EXP.clone(), list![(ExpressionBasics::printExpStr(arg.clone())?).clone()], var.source.info.clone())?;
    } else if BackendVariable::isVarDiscrete(var.clone()) {
        oExp = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) });
    } else if !(BackendVariable::isStateVar(var.clone())) || BackendVariable::varStateSelectForced(var.clone()) {
        var1 = BackendVariable::setVarKind(var.clone(), BackendDAE::VarKind::STATE { index: 1, derName: None, natural: true })?;
        outVars = BackendVariable::addVar(var1.clone(), inVars.clone())?;
        oExp = iExp.clone();
    }
    Ok((outVars, oExp))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn updateStatesVars(mut inVars: BackendDAE::Variables, mut inNewStates: Arc<metamodelica::List<BackendDAE::Var>>, mut noStateFound: bool) -> Result<BackendDAE::Variables> {
    let mut outVars: BackendDAE::Variables;
    outVars = 'mc: {
        let __mc_input = (inNewStates.clone(), noStateFound.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, true) => {
                    Ok(inVars.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: var, tail: newStates }, _) => {
                    let mut vars: BackendDAE::Variables;
                    let mut var = (*var).clone();
                    let false = (BackendVariable::isVarDiscrete(var.clone())) else { bail!("pattern mismatch") };
                    let false = (BackendVariable::isStateVar(var.clone())) else { bail!("pattern mismatch") };
                    var = BackendVariable::setVarKind(var.clone(), BackendDAE::VarKind::STATE { index: 1, derName: None, natural: true })?;
                    vars = BackendVariable::addVar(var.clone(), inVars.clone())?;
                    vars = updateStatesVars(vars.clone(), newStates.clone(), true)?;
                    Ok(vars.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: newStates }, _) => {
                    let mut vars: BackendDAE::Variables;
                    vars = updateStatesVars(inVars.clone(), newStates.clone(), noStateFound.clone())?;
                    Ok(vars.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVars)
}

// =============================================================================
// section for addedScaledVars
//
// =============================================================================
pub fn addedScaledVars_states(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    let mut systlst: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut osystlst: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut vars: BackendDAE::Variables;
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut lst_states: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut tmpv: BackendDAE::Var;
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut norm: Arc<DAE::Exp>;
    let mut y_norm: Arc<DAE::Exp>;
    let mut y: Arc<DAE::Exp>;
    let mut lhs: Arc<DAE::Exp>;
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut syst: Arc<BackendDAE::EqSystem>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ DAE { UNIQUEIO: __pa0, derivativeNamePrefix: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    systlst = __pa0.clone();
    oshared = __pa1.clone();
    for mut syst in &*systlst.clone() {
        let mut syst = syst.clone();
        syst = (::match_deref::match_deref! { match &(syst.clone()) {
        syst1 @ Deref @ BackendDAE::EqSystem { orderedEqs: eqns, orderedVars: vars, .. } => {
            let mut syst1 = (*syst1).clone();
            let mut eqns = (*eqns).clone();
            let mut vars = (*vars).clone();
            lst_states = List::select(BackendVariable::varList(vars.clone())?, Arc::new(fnptr!(BackendVariable::isStateVar, BackendDAE::Var)));
            for mut v in &*lst_states.clone() {
                let mut v = v.clone();
                cref = BackendVariable::varCref(v.clone())?;
                tmpv = BackendVariable::createVar(cref.clone(), (literal!("__OMC$scaled_state")).clone())?;
                y = Expression::crefExp(cref.clone())?;
                norm = BackendVariable::getVarNominalValue(v.clone());
                y_norm = Expression::expDiv(y.clone(), norm.clone())?;
                (y_norm, _) = ExpressionSimplify::simplify(y_norm.clone())?;
                cref = BackendVariable::varCref(tmpv.clone())?;
                lhs = Expression::crefExp(cref.clone())?;
                eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: lhs.clone(), scalar: y_norm.clone(), source: DAE::emptyElementSource.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                eqns = BackendEquation::add(eqn.clone(), eqns.clone())?;
                vars = BackendVariable::addVar(tmpv.clone(), vars.clone())?;
            }
            assign_field!(
                syst1.orderedVars = vars.clone(),
                syst1.orderedEqs = eqns.clone()
            );
            BackendDAEUtil::clearEqSyst(syst1.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
        osystlst = cons(syst.clone(), osystlst.clone());
    }
    outDAE = BackendDAE::DAE(osystlst.clone(), oshared.clone())?;
    Ok(outDAE)
}

pub fn addedScaledVars_inputs(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    let mut systlst: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut osystlst: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut vars: BackendDAE::Variables;
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut kvarlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut lst_inputs: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut tmpv: BackendDAE::Var;
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut norm: Arc<DAE::Exp>;
    let mut y_norm: Arc<DAE::Exp>;
    let mut y: Arc<DAE::Exp>;
    let mut lhs: Arc<DAE::Exp>;
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut syst: Arc<BackendDAE::EqSystem>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ DAE { UNIQUEIO: __pa0, derivativeNamePrefix: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    systlst = __pa0.clone();
    oshared = __pa1.clone();
    kvarlst = BackendVariable::varList(oshared.globalKnownVars.clone())?;
    lst_inputs = List::select(kvarlst.clone(), Arc::new(fnptr!(BackendVariable::isVarOnTopLevelAndInputNoDerInput, BackendDAE::Var)));
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(systlst.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    syst = __pa2.clone();
    osystlst = __pa3.clone();
    syst = (::match_deref::match_deref! { match &(syst.clone()) {
        syst1 @ Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqns, .. } => {
            let mut syst1 = (*syst1).clone();
            let mut vars = (*vars).clone();
            let mut eqns = (*eqns).clone();
            for mut v in &*lst_inputs.clone() {
                let mut v = v.clone();
                cref = BackendVariable::varCref(v.clone())?;
                tmpv = BackendVariable::createVar(cref.clone(), (literal!("__OMC$scaled_input")).clone())?;
                y = Expression::crefExp(cref.clone())?;
                norm = BackendVariable::getVarNominalValue(v.clone());
                y_norm = Expression::expDiv(y.clone(), norm.clone())?;
                (y_norm, _) = ExpressionSimplify::simplify(y_norm.clone())?;
                cref = BackendVariable::varCref(tmpv.clone())?;
                lhs = Expression::crefExp(cref.clone())?;
                eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: lhs.clone(), scalar: y_norm.clone(), source: DAE::emptyElementSource.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                eqns = BackendEquation::add(eqn.clone(), eqns.clone())?;
                vars = BackendVariable::addVar(tmpv.clone(), vars.clone())?;
            }
            assign_field!(
                syst1.orderedEqs = eqns.clone(),
                syst1.orderedVars = vars.clone()
            );
            BackendDAEUtil::clearEqSyst(syst1.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    osystlst = cons(syst.clone(), osystlst.clone());
    outDAE = BackendDAE::DAE(osystlst.clone(), oshared.clone())?;
    Ok(outDAE)
}

// =============================================================================
// section for sortEqnsVars
//
// author: Vitalij Ruge
// =============================================================================
fn sortEqnsVarsWorkTpl(mut tplIndexWeight: Arc<metamodelica::List<(i32, i32)>>) -> Arc<metamodelica::List<i32>> {
    let mut outIndexs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    outIndexs = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut elem in (tplIndexWeight.clone()).into_iter().cloned() {
            let __x = Util::tuple21(elem.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outIndexs
}

fn sortEqnsVarsWeights(mut inW: metamodelica::Array<i32>, mut n: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> metamodelica::Array<i32> {
    let mut outW: metamodelica::Array<i32> = inW.clone();
    let mut i: i32 = 0;
    for mut i in 1..=n.clone() {
        {
            let __cell0 = (m.borrow()[(i.clone()-1) as usize].clone().len() as i32);
            outW.clone().borrow_mut()[(i.clone()-1) as usize] = __cell0;
        }
    }
    outW
}

// =============================================================================
// fix some bugs for complex function
//
// e.g. (a,-b) = f(.) -> (a,c) = f(.) with c = -b
//      (a,b) = (c,d) -> a=c and b = d
//      {a,b} = {c,d} -> a=c and b = d
//      (a,b) = f(a) fixed iterration var
// author: Vitalij Ruge
// =============================================================================
pub fn simplifyComplexFunction(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    outDAE = simplifyComplexFunction1(inDAE.clone(), true)?;
    Ok(outDAE)
}

pub fn simplifyComplexFunction1(mut inDAE: Arc<BackendDAE::BackendDAE>, mut withTmpVars: bool) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = inDAE.clone();
    let mut systlst: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared>;
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut vars: BackendDAE::Variables;
    let mut n: i32 = 0;
    let mut size: i32 = 0;
    let mut idx: i32 = 1;
    let mut m: i32 = 0;
    let mut j: i32 = 0;
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut eqn1: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut left: Arc<DAE::Exp>;
    let mut right: Arc<DAE::Exp>;
    let mut e1: Arc<DAE::Exp>;
    let mut e2: Arc<DAE::Exp>;
    let mut e: Arc<DAE::Exp>;
    let mut e3: Arc<DAE::Exp>;
    let mut e4: Arc<DAE::Exp>;
    let mut left_lst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut right_lst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut indRemove: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut source: Arc<DAE::ElementSource>;
    let mut attr: BackendDAE::EquationAttributes;
    let mut update: bool = false;
    let mut sc: bool = false;
    let mut path: Arc<Absyn::Path>;
    let mut arrayLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut arrayLst2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut cattr: Arc<DAE::CallAttributes>;
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut tmpvar: BackendDAE::Var;
    let mut tmpVarPrefix: ArcStr = arcstr::literal!("");
    shared = inDAE.shared.clone();
    tmpVarPrefix = ((::match_deref::match_deref! { match &(shared.clone()) {
        Deref @ BackendDAE::Shared { backendDAEType: BackendDAE::BackendDAEType::SIMULATION, .. } => literal!("$OMC$CF$sim"),
        Deref @ BackendDAE::Shared { backendDAEType: BackendDAE::BackendDAEType::INITIALSYSTEM, .. } => literal!("$OMC$CF$init"),
        _ => literal!("$OMC$CF$unknown"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    for mut syst in &*inDAE.eqs.clone() {
        let mut syst = syst.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(syst.clone()) {
            Deref @ BackendDAE::EqSystem { orderedEqs: __pa0, orderedVars: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        eqns = __pa0.clone();
        vars = __pa1.clone();
        n = ExpandableArray::getNumberOfElements(eqns.clone());
        update = false;
        indRemove = metamodelica::nil();
        for mut i in 1..=n.clone() {
            match '__try2: {
                eqn = BackendEquation::get(eqns.clone(), i.clone());
                Ok::<_, anyhow::Error>((eqn.clone(),))
            } {
                Ok((__try2_o0,)) => {
                    eqn = __try2_o0;
                }
                Err(_) => {
                    continue;
                }
            }
            if BackendEquation::isComplexEquation(eqn.clone()) || BackendEquation::isArrayEquation(eqn.clone()) {
                if BackendEquation::isComplexEquation(eqn.clone()) {
                    let (__pa3, __pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(eqn.clone()) {
                        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { source: __pa3, attr: __pa4, right: __pa5, left: __pa6, size: __pa7 } => (__pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    source = __pa3.clone();
                    attr = __pa4.clone();
                    right = __pa5.clone();
                    left = __pa6.clone();
                    size = __pa7.clone();
                } else {
                    let (__pa8, __pa9, __pa10, __pa11) = ::match_deref::match_deref! { match &(eqn.clone()) {
                        Deref @ BackendDAE::Equation::ARRAY_EQUATION { source: __pa8, attr: __pa9, right: __pa10, left: __pa11, .. } => (__pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    source = __pa8.clone();
                    attr = __pa9.clone();
                    right = __pa10.clone();
                    left = __pa11.clone();
                }
                if Expression::isTuple(left.clone()) && Expression::isTuple(right.clone()) {
                    let __pa12 = ::match_deref::match_deref! { match &(left.clone()) {
                        Deref @ DAE::Exp::TUPLE { PR: __pa12 } => __pa12.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    left_lst = __pa12.clone();
                    let __pa13 = ::match_deref::match_deref! { match &(right.clone()) {
                        Deref @ DAE::Exp::TUPLE { PR: __pa13 } => __pa13.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    right_lst = __pa13.clone();
                    update = true;
                    indRemove = cons(i.clone(), indRemove.clone());
                    for mut e1 in &*left_lst.clone() {
                        let mut e1 = e1.clone();
                        let (__pa14, __pa15) = ::match_deref::match_deref! { match &(right_lst.clone()) {
                            Deref @ metamodelica::List::Cons { head: __pa14, tail: __pa15 } => (__pa14.clone(), __pa15.clone()),
                            _ => bail!("pattern mismatch"),
                        } };
                        e2 = __pa14.clone();
                        right_lst = __pa15.clone();
                        if !(Expression::isWild(e1.clone())) {
                            if Expression::isScalar(e2.clone())? {
                                eqn1 = BackendEquation::generateEquation(e1.clone(), e2.clone(), source.clone(), attr.clone())?;
                                eqns = BackendEquation::add(eqn1.clone(), eqns.clone())?;
                            } else {
                                expLst = simplifyComplexFunction2(e1.clone());
                                arrayLst = simplifyComplexFunction2(e2.clone());
                                for mut e_asub in &*arrayLst.clone() {
                                    let mut e_asub = e_asub.clone();
                                    let (__pa16, __pa17) = ::match_deref::match_deref! { match &(expLst.clone()) {
                                        Deref @ metamodelica::List::Cons { head: __pa16, tail: __pa17 } => (__pa16.clone(), __pa17.clone()),
                                        _ => bail!("pattern mismatch"),
                                    } };
                                    e3 = __pa16.clone();
                                    expLst = __pa17.clone();
                                    eqn1 = BackendEquation::generateEquation(e_asub.clone(), e3.clone(), source.clone(), attr.clone())?;
                                    eqns = BackendEquation::add(eqn1.clone(), eqns.clone())?;
                                }
                            }
                        }
                    }
                } else if Expression::isArray(left.clone()) && Expression::isArray(right.clone()) {
                    match '__try18: {
                        left_lst = unwrap_break_err!(Expression::getArrayOrRangeContents(left.clone()), '__try18);
                        right_lst = unwrap_break_err!(Expression::getArrayOrRangeContents(right.clone()), '__try18);
                        update = true;
                        indRemove = cons(i.clone(), indRemove.clone());
                        for mut e1 in &*left_lst.clone() {
                            let mut e1 = e1.clone();
                            let (__pa19, __pa20) = ::match_deref::match_deref! { match &(right_lst.clone()) {
                                Deref @ metamodelica::List::Cons { head: __pa19, tail: __pa20 } => (__pa19.clone(), __pa20.clone()),
                                _ => break '__try18 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                            } };
                            e2 = __pa19.clone();
                            right_lst = __pa20.clone();
                            if !(Expression::isWild(e1.clone())) {
                                if unwrap_break_err!(Expression::isScalar(e2.clone()), '__try18) {
                                    eqn1 = unwrap_break_err!(BackendEquation::generateEquation(e1.clone(), e2.clone(), source.clone(), attr.clone()), '__try18);
                                    eqns = unwrap_break_err!(BackendEquation::add(eqn1.clone(), eqns.clone()), '__try18);
                                } else {
                                    expLst = simplifyComplexFunction2(e1.clone());
                                    arrayLst = simplifyComplexFunction2(e2.clone());
                                    for mut e_asub in &*arrayLst.clone() {
                                        let mut e_asub = e_asub.clone();
                                        let (__pa21, __pa22) = ::match_deref::match_deref! { match &(expLst.clone()) {
                                            Deref @ metamodelica::List::Cons { head: __pa21, tail: __pa22 } => (__pa21.clone(), __pa22.clone()),
                                            _ => break '__try18 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                                        } };
                                        e3 = __pa21.clone();
                                        expLst = __pa22.clone();
                                        eqn1 = unwrap_break_err!(BackendEquation::generateEquation(e_asub.clone(), e3.clone(), source.clone(), attr.clone()), '__try18);
                                        eqns = unwrap_break_err!(BackendEquation::add(eqn1.clone(), eqns.clone()), '__try18);
                                    }
                                }
                            }
                        }
                        Ok::<_, anyhow::Error>((indRemove.clone(), left_lst.clone(), right_lst.clone(), update.clone()))
                    } {
                        Ok((__try18_o0, __try18_o1, __try18_o2, __try18_o3)) => {
                            indRemove = __try18_o0;
                            left_lst = __try18_o1;
                            right_lst = __try18_o2;
                            update = __try18_o3;
                        }
                        Err(_) => {
                            continue;
                        }
                    }
                } else if withTmpVars.clone() && Expression::isTuple(left.clone()) && Expression::isCall(right.clone()) {
                    let __pa23 = ::match_deref::match_deref! { match &(left.clone()) {
                        Deref @ DAE::Exp::TUPLE { PR: __pa23 } => __pa23.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    left_lst = __pa23.clone();
                    let (__pa24, __pa25, __pa26) = ::match_deref::match_deref! { match &(right.clone()) {
                        Deref @ DAE::Exp::CALL { attr: __pa24, expLst: __pa25, path: __pa26 } => (__pa24.clone(), __pa25.clone(), __pa26.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cattr = __pa24.clone();
                    expLst = __pa25.clone();
                    path = __pa26.clone();
                    expLst = metamodelica::nil();
                    for mut e1 in &*left_lst.clone() {
                        let mut e1 = e1.clone();
                        if Expression::isCref(e1.clone()) {
                            let __pa27 = ::match_deref::match_deref! { match &(e1.clone()) {
                                Deref @ DAE::Exp::CREF { componentRef: __pa27, .. } => __pa27.clone(),
                                _ => bail!("pattern mismatch"),
                            } };
                            cr = __pa27.clone();
                            if Expression::expHasCrefNoPreOrStart(right.clone(), cr.clone())? {
                                update = true;
                                cr = ComponentReferenceBasics::makeCrefIdent(({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmpVarPrefix.clone()); __mm_s.push_str(&*intString(idx.clone())); ArcStr::from(__mm_s) }).clone(), Expression::r#typeof(e1.clone())?, metamodelica::nil());
                                idx = idx.clone() + 1;
                                e = Expression::crefExp(cr.clone())?;
                                tmpvar = BackendVariable::makeVar(cr.clone());
                                tmpvar = BackendVariable::setVarTS(tmpvar.clone(), Some(crate::BackendDAE::TearingSelect::AVOID));
                                vars = BackendVariable::addVar(tmpvar.clone(), vars.clone())?;
                                eqn1 = Arc::new(BackendDAE::Equation::EQUATION { exp: e.clone(), scalar: e1.clone(), source: DAE::emptyElementSource.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                                eqns = BackendEquation::add(eqn1.clone(), eqns.clone())?;
                            } else {
                                e = e1.clone();
                            }
                        } else if Expression::isUnaryCref(e1.clone()) {
                            update = true;
                            cr = ComponentReferenceBasics::makeCrefIdent(({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmpVarPrefix.clone()); __mm_s.push_str(&*intString(idx.clone())); ArcStr::from(__mm_s) }).clone(), Expression::r#typeof(e1.clone())?, metamodelica::nil());
                            idx = idx.clone() + 1;
                            e = Expression::crefExp(cr.clone())?;
                            tmpvar = BackendVariable::makeVar(cr.clone());
                            tmpvar = BackendVariable::setVarTS(tmpvar.clone(), Some(crate::BackendDAE::TearingSelect::AVOID));
                            vars = BackendVariable::addVar(tmpvar.clone(), vars.clone())?;
                            eqn1 = Arc::new(BackendDAE::Equation::EQUATION { exp: e.clone(), scalar: e1.clone(), source: DAE::emptyElementSource.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                            eqns = BackendEquation::add(eqn1.clone(), eqns.clone())?;
                        } else if Expression::isArray(e1.clone()) {
                            update = true;
                            let (__pa28, __pa29) = ::match_deref::match_deref! { match &(e1.clone()) {
                                Deref @ DAE::Exp::ARRAY { scalar: __pa28, array: __pa29, .. } => (__pa28.clone(), __pa29.clone()),
                                _ => bail!("pattern mismatch"),
                            } };
                            sc = __pa28.clone();
                            arrayLst = __pa29.clone();
                            m = (arrayLst.clone().len() as i32);
                            cr = ComponentReferenceBasics::makeCrefIdent(({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmpVarPrefix.clone()); __mm_s.push_str(&*intString(idx.clone())); ArcStr::from(__mm_s) }).clone(), Expression::r#typeof(e1.clone())?, metamodelica::nil());
                            idx = idx.clone() + 1;
                            e = Expression::crefExp(cr.clone())?;
                            tmpvar = BackendVariable::makeVar(cr.clone());
                            tmpvar = BackendVariable::setVarTS(tmpvar.clone(), Some(crate::BackendDAE::TearingSelect::AVOID));
                            tmpvar.arryDim = list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: m.clone() })];
                            arrayLst2 = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut k in (1..=m.clone()).into_iter() {
            let __x = Expression::makeAsubAddIndex(e.clone(), k.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                            j = 1;
                            for mut e2 in &*arrayLst2.clone() {
                                let mut e2 = e2.clone();
                                let (__pa30, __pa31) = ::match_deref::match_deref! { match &(arrayLst.clone()) {
                                    Deref @ metamodelica::List::Cons { head: __pa30, tail: __pa31 } => (__pa30.clone(), __pa31.clone()),
                                    _ => bail!("pattern mismatch"),
                                } };
                                e3 = __pa30.clone();
                                arrayLst = __pa31.clone();
                                eqn1 = Arc::new(BackendDAE::Equation::EQUATION { exp: e2.clone(), scalar: e3.clone(), source: DAE::emptyElementSource.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                                eqns = BackendEquation::add(eqn1.clone(), eqns.clone())?;
                                cr = ComponentReferenceBasics::makeCrefIdent(({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmpVarPrefix.clone()); __mm_s.push_str(&*intString(idx.clone() - 1)); ArcStr::from(__mm_s) }).clone(), Expression::r#typeof(e1.clone())?, list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: j.clone() }) })]);
                                j = j.clone() + 1;
                                tmpvar.varName = cr.clone();
                                vars = BackendVariable::addVar(tmpvar.clone(), vars.clone())?;
                            }
                        } else {
                            e = e1.clone();
                        }
                        expLst = cons(e.clone(), expLst.clone());
                    }
                    left = Arc::new(DAE::Exp::TUPLE { PR: metamodelica::Dangerous::listReverseInPlace(expLst.clone()) });
                    eqn = BackendEquation::generateEquation(left.clone(), right.clone(), source.clone(), attr.clone())?;
                    eqns = BackendEquation::setAtIndex(eqns.clone(), i.clone(), eqn.clone())?;
                }
            }
        }
        if update.clone() {
            for mut i in &*indRemove.clone().reverse() {
                let mut i = i.clone();
                eqns = BackendEquation::delete(i.clone(), eqns.clone())?;
            }
            eqns = BackendEquation::listEquation(BackendEquation::equationList(eqns.clone()))?;
            systlst = cons(BackendDAEUtil::createEqSystem(vars.clone(), eqns.clone(), syst.stateSets.clone(), syst.partitionKind.clone(), syst.removedEqs.clone()), systlst.clone());
        } else {
            systlst = cons(syst.clone(), systlst.clone());
        }
    }
    assign_field!(outDAE.eqs = systlst.clone());
    Ok(outDAE)
}

pub fn simplifyComplexFunction2(mut e1: Arc<DAE::Exp>) -> Arc<metamodelica::List<Arc<DAE::Exp>>> {
    let mut out_lst_e1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut lst_e: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    if '__try0: {
        if Expression::isArray(e1.clone()) || Expression::isArrayType(Expression::r#typeof(e1.clone()).unwrap()) {
            lst_e = unwrap_break_err!(Expression::getArrayOrRangeContents(e1.clone()), '__try0);
            for mut e in &*lst_e.clone() {
                let mut e = e.clone();
                out_lst_e1 = listAppend(simplifyComplexFunction2(e.clone()), out_lst_e1.clone());
            }
        } else if Expression::isRecord(e1.clone()) {
            lst_e = unwrap_break_err!(Expression::splitRecord(e1.clone(), Expression::r#typeof(e1.clone()).unwrap()), '__try0);
            for mut e in &*lst_e.clone() {
                let mut e = e.clone();
                out_lst_e1 = listAppend(simplifyComplexFunction2(e.clone()), out_lst_e1.clone());
            }
            out_lst_e1 = list![e1.clone()];
        } else {
            out_lst_e1 = list![e1.clone()];
        }
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        out_lst_e1 = list![e1.clone()];
    }
    out_lst_e1
}

// =============================================================================
// section for hets
//
// (h)euristic (e)quation (t)erms (s)ort
// heuristic sorting of terms for better numeric in equations(res, torn,...)
//
// author: Vitalij Ruge
// =============================================================================
pub fn hets(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    if Flags::getConfigString(Flags::HETS.clone())? != literal!("none") {
        outDAE = hetsWork(inDAE.clone())?;
    } else {
        outDAE = inDAE.clone();
    }
    Ok(outDAE)
}

fn hetsWork(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = inDAE.clone();
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut vars: BackendDAE::Variables;
    let mut matching: Arc<BackendDAE::Matching> = Arc::new(BackendDAE::Matching::NO_MATCHING);
    let mut stateSets: Arc<metamodelica::List<BackendDAE::StateSet>> = metamodelica::nil();
    let mut partitionKind: BackendDAE::BaseClockPartitionKind = BackendDAE::BaseClockPartitionKind::CONTINUOUS_TIME_PARTITION;
    let mut innerEquations: Arc<metamodelica::List<BackendDAE::InnerEquation>> = metamodelica::nil();
    let mut innerEquation: BackendDAE::InnerEquation;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut tvars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut teqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared>;
    shared = outDAE.shared.clone();
    for mut syst in &*outDAE.eqs.clone() {
        let mut syst = syst.clone();
        let (__pa0, __pa1, __pa3, __pa2, __pa4, __pa5) = ::match_deref::match_deref! { match &(syst.clone()) {
            Deref @ BackendDAE::EqSystem { partitionKind: __pa0, stateSets: __pa1, matching: __pa3 @ Deref @ BackendDAE::Matching::MATCHING { comps: __pa2, .. }, orderedEqs: __pa4, orderedVars: __pa5, .. } => (__pa0.clone(), __pa1.clone(), __pa3.clone(), __pa2.clone(), __pa4.clone(), __pa5.clone()),
            _ => bail!("pattern mismatch"),
        } };
        partitionKind = __pa0.clone();
        stateSets = __pa1.clone();
        comps = __pa2.clone();
        matching = __pa3.clone();
        eqns = __pa4.clone();
        vars = __pa5.clone();
        for mut comp in &*comps.clone() {
            let mut comp = comp.clone();
            if BackendEquation::isTornSystem(comp.clone()) {
                let (__pa7, __pa8, __pa9) = ::match_deref::match_deref! { match &(comp.clone()) {
                    Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { innerEquations: __pa7, residualequations: __pa8, tearingvars: __pa9, .. }, .. } => (__pa7.clone(), __pa8.clone(), __pa9.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                innerEquations = __pa7.clone();
                teqns = __pa8.clone();
                tvars = __pa9.clone();
                for mut innerEquation in &*innerEquations.clone() {
                    let mut innerEquation = innerEquation.clone();
                    if '__try10: {
                        let (__pa11, __pa12) = ::match_deref::match_deref! { match &(unwrap_break_err!(BackendDAEUtil::getEqnAndVarsFromInnerEquation(innerEquation.clone()), '__try10)) {
                            (__pa11, Deref @ metamodelica::List::Cons { head: __pa12, tail: Deref @ metamodelica::List::Nil }, _) => (__pa11.clone(), __pa12.clone()),
                            _ => break '__try10 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                        } };
                        i = __pa11.clone();
                        j = __pa12.clone();
                        eqn = BackendEquation::get(eqns.clone(), i.clone());
                        let BackendDAE::VAR { varName: __pa14, .. } = (unwrap_break_err!(BackendVariable::getVarAt(vars.clone(), j.clone()), '__try10)) else { break '__try10 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                        cr = __pa14.clone();
                        eqn = unwrap_break_err!(BackendEquation::solveEquation(eqn.clone(), Expression::crefExp(cr.clone())?, Some(shared.functionTree.clone())), '__try10);
                        eqn = unwrap_break_err!(hetsSplitRhs(eqn.clone()), '__try10);
                        eqns = unwrap_break_err!(BackendEquation::setAtIndex(eqns.clone(), i.clone(), eqn.clone()), '__try10);
                        Ok::<(), anyhow::Error>(())
                    }.is_err() {
                    }
                }
                for mut i in &*teqns.clone() {
                    let mut i = i.clone();
                    eqn = BackendEquation::get(eqns.clone(), i.clone());
                    eqn = hetsSplitRes(eqn.clone())?;
                    eqns = BackendEquation::setAtIndex(eqns.clone(), i.clone(), eqn.clone())?;
                }
            } else if BackendEquation::isEquationsSystem(comp.clone()) {
                let __pa15 = ::match_deref::match_deref! { match &(comp.clone()) {
                    Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: __pa15, .. } => __pa15.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                teqns = __pa15.clone();
                for mut i in &*teqns.clone() {
                    let mut i = i.clone();
                    eqn = BackendEquation::get(eqns.clone(), i.clone());
                    eqn = hetsSplitRes(eqn.clone())?;
                    eqns = BackendEquation::setAtIndex(eqns.clone(), i.clone(), eqn.clone())?;
                }
            }
        }
    }
    Ok(outDAE)
}

fn hetsSplitRes(mut iEqn: Arc<BackendDAE::Equation>) -> Result<Arc<BackendDAE::Equation>> {
    let mut oEqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    oEqn = (::match_deref::match_deref! { match &(iEqn.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { attr, source, scalar: e2, exp: e1 } => {
            let mut e: Arc<DAE::Exp>;
            e = Expression::createResidualExp(e1.clone(), e2.clone())?;
            e = hetsSplitExp(e.clone())?;
            Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: e.clone(), source: source.clone(), attr: attr.clone() })
        },
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, source, attr } => {
            let mut e = (*e).clone();
            e = hetsSplitExp(e.clone())?;
            Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: e.clone(), source: source.clone(), attr: attr.clone() })
        },
        _ => {
            iEqn.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oEqn)
}

fn hetsSplitRhs(mut iEqn: Arc<BackendDAE::Equation>) -> Result<Arc<BackendDAE::Equation>> {
    let mut oEqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    oEqn = (::match_deref::match_deref! { match &(iEqn.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { attr, source, scalar: e2, exp: e1 } => {
            let mut e2 = (*e2).clone();
            e2 = hetsSplitExp(e2.clone())?;
            Arc::new(BackendDAE::Equation::EQUATION { exp: e1.clone(), scalar: e2.clone(), source: source.clone(), attr: attr.clone() })
        },
        _ => {
            iEqn.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oEqn)
}

fn hetsSplitExp(mut iExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut oExp: Arc<DAE::Exp>;
    oExp = (::match_deref::match_deref! { match &(iExp.clone()) {
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 } if (Expression::isMulOrDiv(op.clone())) => {
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            e1 = hetsSplitExp(e1.clone())?;
            e2 = hetsSplitExp(e2.clone())?;
            Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone() })
        },
        e @ Deref @ DAE::Exp::BINARY { exp1: _, operator: op, exp2: _ } if (Expression::isAddOrSub(op.clone())) => {
            let mut terms: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut termsDer: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            terms = Expression::terms(e.clone())?;
            terms = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut t in (terms.clone()).into_iter().cloned() {
            let __x = hetsSplitExp(t.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            (termsDer, terms) = List::splitOnTrue(terms.clone(), Arc::new(Expression::expHasDer));
            Expression::expAdd(Expression::makeSum1(terms.clone(), false)?, Expression::makeSum1(termsDer.clone(), false)?)?
        },
        _ => {
            iExp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oExp)
}

// =============================================================================
// section inlineFunctionInLoops
// force inlining function of loop
// author: Vitalij Ruge
// motivation see #3997 library devs introduce annotation(Inline=true) for simplify loops
// =============================================================================
pub fn inlineFunctionInLoops(mut dae: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut dae: Arc<BackendDAE::BackendDAE> = dae;
    dae = inlineFunctionInLoopsMain(dae.clone())?;
    Ok(dae)
}

// =============================================================================
// section for simplifyLoops
//
// simplify(hopful) loops for simulation/optimization
// author: Vitalij Ruge
// =============================================================================
pub fn simplifyLoops(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    outDAE = if (Flags::getConfigInt(Flags::SIMPLIFY_LOOPS.clone())? > 0) {simplifyLoopsMain(inDAE.clone())?} else {inDAE.clone()};
    Ok(outDAE)
}

fn simplifyLoopsUpdateAss(mut inAss: metamodelica::Array<i32>, mut new_ass: Arc<metamodelica::List<i32>>, mut n: i32) -> metamodelica::Array<i32> {
    let mut outAss: metamodelica::Array<i32> = inAss.clone();
    let mut i: i32 = 1;
    for mut a in &*new_ass.clone() {
        let mut a = a.clone();
        {
            let __cell0 = a.clone();
            outAss.clone().borrow_mut()[(i.clone() + n.clone()-1) as usize] = __cell0;
        }
        i = i.clone() + 1;
    }
    outAss
}

fn simplifyLoopsUpdateComps(mut inComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut inAss1: Arc<metamodelica::List<i32>>, mut inAss2: Arc<metamodelica::List<i32>>, mut inCompOrders: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>> {
    let mut outComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = inComps.clone();
    let mut a1: i32 = 0;
    let mut a2: i32 = 0;
    let mut shift: i32 = 0;
    let mut o: i32 = 0;
    let mut comp: Arc<BackendDAE::StrongComponent>;
    let mut ass1: Arc<metamodelica::List<i32>> = inAss1.clone();
    let mut ass2: Arc<metamodelica::List<i32>> = inAss2.clone();
    let mut compOrders: Arc<metamodelica::List<i32>> = inCompOrders.clone();
    for mut a1 in &*ass1.clone() {
        let mut a1 = a1.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(compOrders.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        o = __pa0.clone();
        compOrders = __pa1.clone();
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(ass2.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        a2 = __pa2.clone();
        ass2 = __pa3.clone();
        comp = Arc::new(BackendDAE::StrongComponent::SINGLEEQUATION { eqn: a1.clone(), var: a2.clone() });
        outComps = List::insert(outComps.clone(), o.clone() + shift.clone(), comp.clone())?;
        shift = shift.clone() + 1;
    }
    Ok(outComps)
}

fn simplifyLoopsWork(mut inComp: Arc<BackendDAE::StrongComponent>, mut inIndx: i32, mut inVars: BackendDAE::Variables, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inShared: Arc<BackendDAE::Shared>, mut inUpdate: bool, mut ass1_: Arc<metamodelica::List<i32>>, mut ass2_: Arc<metamodelica::List<i32>>, mut simDAE: bool, mut ii: i32, mut inCompOrders: Arc<metamodelica::List<i32>>) -> Result<(i32, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Shared>, bool, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut outIndx: i32 = inIndx.clone();
    let mut outVars: BackendDAE::Variables = inVars.clone();
    let mut outEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = inEqns.clone();
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut outUpdate: bool = inUpdate.clone();
    let mut ass1: Arc<metamodelica::List<i32>> = ass1_.clone();
    let mut ass2: Arc<metamodelica::List<i32>> = ass2_.clone();
    let mut outCompOrders: Arc<metamodelica::List<i32>> = inCompOrders.clone();
    let mut eqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut vv: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut var_lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut update: bool = false;
    let mut linear: bool = false;
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut innerEquations: Arc<metamodelica::List<BackendDAE::InnerEquation>> = metamodelica::nil();
    let mut innerEquation: BackendDAE::InnerEquation;
    if BackendEquation::isEquationsSystem(inComp.clone()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inComp.clone()) {
            Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { vars: __pa0, eqns: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        vars = __pa0.clone();
        eqns = __pa1.clone();
        if BackendDAEUtil::isLinearEqSystemComp(inComp.clone()) {
            return Ok((outIndx, outVars, outEqns, outShared, outUpdate, ass1, ass2, outCompOrders));
        }
        if Flags::isSet(Flags::DUMP_SIMPLIFY_LOOPS.clone())? {
            println!("{}", (literal!("------ EquationsSystem ------\n")).clone());
        }
    } else {
        let (__pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(inComp.clone()) {
            Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { innerEquations: __pa2, residualequations: __pa3, tearingvars: __pa4, .. }, linear: __pa5, .. } => (__pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
            _ => bail!("pattern mismatch"),
        } };
        innerEquations = __pa2.clone();
        eqns = __pa3.clone();
        vars = __pa4.clone();
        linear = __pa5.clone();
        if linear.clone() {
            return Ok((outIndx, outVars, outEqns, outShared, outUpdate, ass1, ass2, outCompOrders));
        }
        if Flags::isSet(Flags::DUMP_SIMPLIFY_LOOPS.clone())? {
            println!("{}", (literal!("------ Tearing ------\n")).clone());
        }
        for mut innerEquation in &*innerEquations.clone() {
            let mut innerEquation = innerEquation.clone();
            (k, vv, _) = BackendDAEUtil::getEqnAndVarsFromInnerEquation(innerEquation.clone())?;
            eqns = cons(k.clone(), eqns.clone());
            vars = listAppend(vv.clone(), vars.clone());
        }
    }
    if Flags::isSet(Flags::DUMP_SIMPLIFY_LOOPS.clone())? {
        println!("{}", (literal!("------ loop-vars ------\n")).clone());
    }
    for mut i in &*vars.clone() {
        let mut i = i.clone();
        let BackendDAE::VAR { varName: __pa6, .. } = (BackendVariable::getVarAt(outVars.clone(), i.clone())?) else { bail!("pattern mismatch") };
        cr = __pa6.clone();
        var_lst = cons(cr.clone(), var_lst.clone());
        if Flags::isSet(Flags::DUMP_SIMPLIFY_LOOPS.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
    }
    if Flags::isSet(Flags::DUMP_SIMPLIFY_LOOPS.clone())? {
        println!("{}", (literal!("------------\n")).clone());
    }
    for mut i in &*eqns.clone() {
        let mut i = i.clone();
        if '__try7: {
            eqn = BackendEquation::get(outEqns.clone(), i.clone());
            if unwrap_break_err!(Flags::isSet(Flags::DUMP_SIMPLIFY_LOOPS.clone()), '__try7) {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("update eqn[")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!("]\n")); ArcStr::from(__mm_s) }).clone());
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*unwrap_break_err!(BackendDump::equationString(eqn.clone()), '__try7)); __mm_s.push_str(&*literal!("--old--\n")); ArcStr::from(__mm_s) }).clone());
            }
            (outIndx, outVars, outEqns, outShared, update, eqn, ass1, ass2, outCompOrders) = unwrap_break_err!(simplifyLoopEqn(outIndx.clone(), outVars.clone(), outEqns.clone(), outShared.clone(), var_lst.clone(), eqn.clone(), ass1.clone(), ass2.clone(), simDAE.clone(), ii.clone(), outCompOrders.clone()), '__try7);
            outUpdate = outUpdate.clone() || update.clone();
            outEqns = unwrap_break_err!(BackendEquation::setAtIndex(outEqns.clone(), i.clone(), eqn.clone()), '__try7);
            if unwrap_break_err!(Flags::isSet(Flags::DUMP_SIMPLIFY_LOOPS.clone()), '__try7) {
                println!("{}", (literal!("=> ")).clone());
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*unwrap_break_err!(BackendDump::equationString(eqn.clone()), '__try7)); __mm_s.push_str(&*literal!("--new--\n")); ArcStr::from(__mm_s) }).clone());
            }
            Ok::<(), anyhow::Error>(())
        }.is_err() {
        }
    }
    Ok((outIndx, outVars, outEqns, outShared, outUpdate, ass1, ass2, outCompOrders))
}

fn simplifyLoopEqn(mut inIndx: i32, mut inVars: BackendDAE::Variables, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inShared: Arc<BackendDAE::Shared>, mut var_lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inEqn: Arc<BackendDAE::Equation>, mut ass1_: Arc<metamodelica::List<i32>>, mut ass2_: Arc<metamodelica::List<i32>>, mut simDAE: bool, mut ii: i32, mut inCompOrders: Arc<metamodelica::List<i32>>) -> Result<(i32, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Shared>, bool, Arc<BackendDAE::Equation>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut outIndx: i32 = inIndx.clone();
    let mut outVars: BackendDAE::Variables = inVars.clone();
    let mut outEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = inEqns.clone();
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut outUpdate: bool = false;
    let mut outEqn: Arc<BackendDAE::Equation> = inEqn.clone();
    let mut ass1: Arc<metamodelica::List<i32>> = ass1_.clone();
    let mut ass2: Arc<metamodelica::List<i32>> = ass2_.clone();
    let mut outCompOrder: Arc<metamodelica::List<i32>> = inCompOrders.clone();
    let mut rhs: Arc<DAE::Exp>;
    let mut lhs: Arc<DAE::Exp>;
    let mut e: Arc<DAE::Exp>;
    let mut update_lhs: bool = false;
    let mut update_rhs: bool = false;
    let mut loopTerms_lhs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut noLoopTerms_lhs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut loopTerms_rhs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut noLoopTerms_rhs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut useTmpVars: bool = Flags::getConfigInt(Flags::SIMPLIFY_LOOPS.clone())? > 1;
    if BackendEquation::isAlgorithm(outEqn.clone()) {
        return Ok((outIndx, outVars, outEqns, outShared, outUpdate, outEqn, ass1, ass2, outCompOrder));
    }
    lhs = BackendEquation::getEquationLHS(outEqn.clone())?;
    if !(Types::isIntegerOrRealOrSubTypeOfEither(Expression::r#typeof(lhs.clone())?)?) {
        return Ok((outIndx, outVars, outEqns, outShared, outUpdate, outEqn, ass1, ass2, outCompOrder));
    }
    rhs = BackendEquation::getEquationRHS(outEqn.clone())?;
    (loopTerms_lhs, noLoopTerms_lhs) = simplifyLoops_SplitTerms(var_lst.clone(), lhs.clone())?;
    (loopTerms_rhs, noLoopTerms_rhs) = simplifyLoops_SplitTerms(var_lst.clone(), rhs.clone())?;
    if (loopTerms_lhs.clone().len() as i32) > (loopTerms_rhs.clone().len() as i32) {
        lhs = Expression::expSub(Expression::makeSum1(loopTerms_lhs.clone(), false)?, Expression::makeSum1(loopTerms_rhs.clone(), false)?)?;
        rhs = Expression::expSub(Expression::makeSum1(noLoopTerms_rhs.clone(), false)?, Expression::makeSum1(noLoopTerms_lhs.clone(), false)?)?;
    } else {
        lhs = Expression::expSub(Expression::makeSum1(loopTerms_rhs.clone(), false)?, Expression::makeSum1(loopTerms_lhs.clone(), false)?)?;
        rhs = Expression::expSub(Expression::makeSum1(noLoopTerms_lhs.clone(), false)?, Expression::makeSum1(noLoopTerms_rhs.clone(), false)?)?;
    }
    (lhs, rhs, _) = Expression::createResidualExp3(lhs.clone(), rhs.clone())?;
    (lhs, e) = Expression::makeFraction(lhs.clone())?;
    (lhs, _) = ExpressionSimplify::simplify(lhs.clone())?;
    (e, _) = ExpressionSimplify::simplify(e.clone())?;
    rhs = ExpressionSimplify::simplifySumOperatorExpression(rhs.clone(), DAE::Operator::MUL { ty: Expression::r#typeof(rhs.clone())? }, e.clone())?;
    (outIndx, outVars, outEqns, outShared, update_rhs, rhs, ass1, ass2, outCompOrder) = simplifyLoopExp(outIndx.clone(), outVars.clone(), outEqns.clone(), outShared.clone(), var_lst.clone(), rhs.clone(), ass1.clone(), ass2.clone(), simDAE.clone(), useTmpVars.clone(), ii.clone(), outCompOrder.clone(), (literal!("LOOP")).clone(), false)?;
    (outIndx, outVars, outEqns, outShared, update_lhs, lhs, ass1, ass2, outCompOrder) = simplifyLoopExp(outIndx.clone(), outVars.clone(), outEqns.clone(), outShared.clone(), var_lst.clone(), lhs.clone(), ass1.clone(), ass2.clone(), simDAE.clone(), useTmpVars.clone(), ii.clone(), outCompOrder.clone(), (literal!("LOOP")).clone(), false)?;
    outEqn = BackendEquation::setEquationLHS(outEqn.clone(), lhs.clone())?;
    outEqn = BackendEquation::setEquationRHS(outEqn.clone(), rhs.clone())?;
    outUpdate = outUpdate.clone() || update_rhs.clone() || update_lhs.clone();
    Ok((outIndx, outVars, outEqns, outShared, outUpdate, outEqn, ass1, ass2, outCompOrder))
}

pub fn simplifyLoopExp(mut inIndx: i32, mut inVars: BackendDAE::Variables, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inShared: Arc<BackendDAE::Shared>, mut var_lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inExp: Arc<DAE::Exp>, mut ass1_: Arc<metamodelica::List<i32>>, mut ass2_: Arc<metamodelica::List<i32>>, mut simDAE: bool, mut useTmpVars: bool, mut ii: i32, mut inCompOrders: Arc<metamodelica::List<i32>>, mut tmpVarName: ArcStr, mut noPara: bool) -> Result<(i32, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Shared>, bool, Arc<DAE::Exp>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut outIndx: i32 = inIndx.clone();
    let mut outVars: BackendDAE::Variables = inVars.clone();
    let mut outEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = inEqns.clone();
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut outUpdate: bool = false;
    let mut outExp: Arc<DAE::Exp> = inExp.clone();
    let mut ass1: Arc<metamodelica::List<i32>> = ass1_.clone();
    let mut ass2: Arc<metamodelica::List<i32>> = ass2_.clone();
    let mut outCompOrder: Arc<metamodelica::List<i32>> = inCompOrders.clone();
    let mut loopTerms: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut noLoopTerms: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut loopFactors: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut noLoopFactors: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut loopTermsUpdatedFactors: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut loopTerms2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut noLoopTerms2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut loopFacotrsUpdatedTerms: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut res: Arc<DAE::Exp>;
    let mut noLoopTerm: Arc<DAE::Exp>;
    let mut loopTerm: Arc<DAE::Exp>;
    let mut noLoopFactor: Arc<DAE::Exp>;
    let mut noLoopTerm2: Arc<DAE::Exp>;
    let mut loopTerm2: Arc<DAE::Exp>;
    let mut e1: Arc<DAE::Exp>;
    let mut e2: Arc<DAE::Exp>;
    let mut con: Arc<DAE::Exp>;
    let mut update: bool = false;
    let mut op: DAE::Operator;
    let mut para: bool = false;
    let mut ne: i32 = 0;
    let mut nv: i32 = 0;
    (loopTerms, noLoopTerms) = simplifyLoops_SplitTerms(var_lst.clone(), outExp.clone())?;
    (noLoopTerm, _) = ExpressionSimplify::simplify1(Expression::makeSum1(noLoopTerms.clone(), false)?)?;
    if useTmpVars.clone() && simDAE.clone() {
        (noLoopTerm, outEqns, outVars, outShared, update, para) = BackendEquation::makeTmpEqnForExp(noLoopTerm.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmpVarName.clone()); __mm_s.push_str(&*literal!("T")); ArcStr::from(__mm_s) }).clone(), System::tmpTickIndex(Global::tmpVariableIndex.clone()), outEqns.clone(), outVars.clone(), outShared.clone(), false)?;
        (outUpdate, ass1, ass2, outIndx, outCompOrder) = simplifyLoopExpHelper(update.clone(), outUpdate.clone(), para.clone(), ass1.clone(), ass2.clone(), outVars.clone(), outEqns.clone(), outIndx.clone(), ii.clone(), outCompOrder.clone())?;
    }
    loopTermsUpdatedFactors = metamodelica::nil();
    for mut factor in &*loopTerms.clone() {
        let mut factor = factor.clone();
        (loopFactors, noLoopFactors) = simplifyLoops_SplitFactors(var_lst.clone(), factor.clone())?;
        (noLoopFactor, _) = ExpressionSimplify::simplify1(Expression::makeProductLst(noLoopFactors.clone())?)?;
        if useTmpVars.clone() && simDAE.clone() {
            if (::match_deref::match_deref! { match &(noLoopFactor.clone()) {
        Deref @ DAE::Exp::BINARY { operator: DAE::Operator::DIV { .. }, .. } => true,
        Deref @ DAE::Exp::BINARY { operator: DAE::Operator::POW { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }) {
                let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(noLoopFactor.clone()) {
                    Deref @ DAE::Exp::BINARY { exp1: __pa0, operator: __pa1, exp2: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                e1 = __pa0.clone();
                op = __pa1.clone();
                e2 = __pa2.clone();
                (e1, outEqns, outVars, outShared, update, para) = BackendEquation::makeTmpEqnForExp(e1.clone(), (literal!("LOOPF")).clone(), if (simDAE.clone()) {outIndx.clone()} else {-(outIndx.clone())}, outEqns.clone(), outVars.clone(), outShared.clone(), noPara.clone())?;
                (outUpdate, ass1, ass2, outIndx, outCompOrder) = simplifyLoopExpHelper(update.clone(), outUpdate.clone(), para.clone(), ass1.clone(), ass2.clone(), outVars.clone(), outEqns.clone(), outIndx.clone(), ii.clone(), outCompOrder.clone())?;
                (e2, outEqns, outVars, outShared, update, para) = BackendEquation::makeTmpEqnForExp(e2.clone(), (literal!("LOOPF")).clone(), if (simDAE.clone()) {outIndx.clone()} else {-(outIndx.clone())}, outEqns.clone(), outVars.clone(), outShared.clone(), noPara.clone())?;
                (outUpdate, ass1, ass2, outIndx, outCompOrder) = simplifyLoopExpHelper(update.clone(), outUpdate.clone(), para.clone(), ass1.clone(), ass2.clone(), outVars.clone(), outEqns.clone(), outIndx.clone(), ii.clone(), outCompOrder.clone())?;
                noLoopFactor = Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone() });
            } else {
                (noLoopFactor, outEqns, outVars, outShared, update, para) = BackendEquation::makeTmpEqnForExp(noLoopFactor.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmpVarName.clone()); __mm_s.push_str(&*literal!("F")); ArcStr::from(__mm_s) }).clone(), if (simDAE.clone()) {outIndx.clone()} else {-(outIndx.clone())}, outEqns.clone(), outVars.clone(), outShared.clone(), false)?;
                (outUpdate, ass1, ass2, outIndx, outCompOrder) = simplifyLoopExpHelper(update.clone(), outUpdate.clone(), para.clone(), ass1.clone(), ass2.clone(), outVars.clone(), outEqns.clone(), outIndx.clone(), ii.clone(), outCompOrder.clone())?;
            }
        }
        loopFacotrsUpdatedTerms = metamodelica::nil();
        for mut term in &*loopFactors.clone() {
            let mut term = term.clone();
            res = term.clone();
            if Expression::isBinary(res.clone()) {
                let __pa3 = ::match_deref::match_deref! { match &(res.clone()) {
                    Deref @ DAE::Exp::BINARY { operator: __pa3, .. } => __pa3.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                op = __pa3.clone();
                if Expression::isAddOrSub(op.clone()) || Expression::isMulOrDiv(op.clone()) || Expression::isPow(op.clone()) {
                    if !(ExpressionBasics::expEqual(res.clone(), inExp.clone())?) {
                        if Expression::isDiv(op.clone()) || Expression::isPow(op.clone()) {
                            let (__pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(res.clone()) {
                                Deref @ DAE::Exp::BINARY { exp1: __pa4, operator: __pa5, exp2: __pa6 } => (__pa4.clone(), __pa5.clone(), __pa6.clone()),
                                _ => bail!("pattern mismatch"),
                            } };
                            e1 = __pa4.clone();
                            op = __pa5.clone();
                            e2 = __pa6.clone();
                            (outIndx, outVars, outEqns, outShared, update, e1, ass1, ass2, outCompOrder) = simplifyLoopExp(outIndx.clone(), outVars.clone(), outEqns.clone(), outShared.clone(), var_lst.clone(), e1.clone(), ass1.clone(), ass2.clone(), simDAE.clone(), useTmpVars.clone(), ii.clone(), outCompOrder.clone(), (literal!("LOOP")).clone(), false)?;
                            outUpdate = update.clone() || outUpdate.clone();
                            (outIndx, outVars, outEqns, outShared, update, e2, ass1, ass2, outCompOrder) = simplifyLoopExp(outIndx.clone(), outVars.clone(), outEqns.clone(), outShared.clone(), var_lst.clone(), e2.clone(), ass1.clone(), ass2.clone(), simDAE.clone(), useTmpVars.clone(), ii.clone(), outCompOrder.clone(), (literal!("LOOP")).clone(), false)?;
                            outUpdate = update.clone() || outUpdate.clone();
                            (e2, _) = ExpressionSimplify::simplify1(e2.clone())?;
                            res = Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone() });
                        } else {
                            (outIndx, outVars, outEqns, outShared, update, res, ass1, ass2, outCompOrder) = simplifyLoopExp(outIndx.clone(), outVars.clone(), outEqns.clone(), outShared.clone(), var_lst.clone(), res.clone(), ass1.clone(), ass2.clone(), simDAE.clone(), useTmpVars.clone(), ii.clone(), outCompOrder.clone(), (literal!("LOOP")).clone(), false)?;
                            outUpdate = update.clone() || outUpdate.clone();
                        }
                    }
                }
            }
            loopFacotrsUpdatedTerms = cons(res.clone(), loopFacotrsUpdatedTerms.clone());
        }
        loopTermsUpdatedFactors = cons(Expression::makeProductLst(cons(noLoopFactor.clone(), loopFacotrsUpdatedTerms.clone()))?, loopTermsUpdatedFactors.clone());
    }
    (outExp, _) = ExpressionSimplify::simplify(Expression::makeSum1(cons(noLoopTerm.clone(), loopTermsUpdatedFactors.clone()), true)?)?;
    Ok((outIndx, outVars, outEqns, outShared, outUpdate, outExp, ass1, ass2, outCompOrder))
}

fn simplifyLoopExpHelper(mut update: bool, mut update_: bool, mut para: bool, mut ass1_: Arc<metamodelica::List<i32>>, mut ass2_: Arc<metamodelica::List<i32>>, mut inVars: BackendDAE::Variables, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inIndex: i32, mut ii: i32, mut inCompOrders: Arc<metamodelica::List<i32>>) -> Result<(bool, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, i32, Arc<metamodelica::List<i32>>)> {
    let mut outUpdate: bool = update_.clone();
    let mut ass1: Arc<metamodelica::List<i32>> = ass1_.clone();
    let mut ass2: Arc<metamodelica::List<i32>> = ass2_.clone();
    let mut outIndx: i32 = inIndex.clone();
    let mut outCompOrder: Arc<metamodelica::List<i32>> = inCompOrders.clone();
    let mut ne: i32 = 0;
    let mut nv: i32 = 0;
    if update.clone() {
        outIndx = outIndx.clone() + 1;
        outUpdate = update.clone();
        if !(para.clone()) {
            ne = ExpandableArray::getNumberOfElements(inEqns.clone());
            let BackendDAE::VARIABLES { numberOfVars: __pa0, .. } = (inVars.clone()) else { bail!("pattern mismatch") };
            nv = __pa0.clone();
            ass1 = cons(ne.clone(), ass1.clone());
            ass2 = cons(nv.clone(), ass2.clone());
            outCompOrder = cons(ii.clone(), outCompOrder.clone());
        }
    }
    Ok((outUpdate, ass1, ass2, outIndx, outCompOrder))
}

pub fn simplifyLoops_SplitTerms(mut var_lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inExp: Arc<DAE::Exp>) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<DAE::Exp>>>)> {
    let mut loopTerms: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut noLoopTerms: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut tmp_loopTerms: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    noLoopTerms = Expression::terms(inExp.clone())?;
    for mut cr in &*var_lst.clone() {
        let mut cr = cr.clone();
        if noLoopTerms.clone().is_empty() {
            break;
        } else {
            (tmp_loopTerms, noLoopTerms) = List::split1OnTrue(noLoopTerms.clone(), Arc::new(Expression::expHasCrefNoPreOrStart), cr.clone());
            loopTerms = listAppend(tmp_loopTerms.clone(), loopTerms.clone());
        }
    }
    Ok((loopTerms, noLoopTerms))
}

fn simplifyLoops_SplitFactors(mut var_lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inExp: Arc<DAE::Exp>) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<DAE::Exp>>>)> {
    let mut loopTerms: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut noLoopTerms: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut tmp_loopTerms: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    noLoopTerms = Expression::factors(inExp.clone())?;
    for mut cr in &*var_lst.clone() {
        let mut cr = cr.clone();
        if noLoopTerms.clone().is_empty() {
            break;
        } else {
            (tmp_loopTerms, noLoopTerms) = List::split1OnTrue(noLoopTerms.clone(), Arc::new(Expression::expHasCrefNoPreOrStart), cr.clone());
            loopTerms = listAppend(tmp_loopTerms.clone(), loopTerms.clone());
        }
    }
    Ok((loopTerms, noLoopTerms))
}

// =============================================================================
// section for introduceDerAlias
//
// =============================================================================
pub fn introduceDerAlias(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    outDAE = BackendDAEUtil::mapEqSystem(inDAE.clone(), Arc::new(introduceDerAliasWork))?;
    Ok(outDAE)
}

fn introduceDerAliasWork(mut inSyst: Arc<BackendDAE::EqSystem>, mut shared: Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared> = shared.clone();
    let mut syst: Arc<BackendDAE::EqSystem>;
    let mut vars: BackendDAE::Variables;
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut eqnsList: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    syst = inSyst.clone();
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(syst.clone()) {
        Deref @ BackendDAE::EqSystem { orderedEqs: __pa0, orderedVars: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    eqns = __pa0.clone();
    vars = __pa1.clone();
    let (__pa2, (__pa3, __pa4, _, _)) = BackendEquation::traverseEquationArray_WithUpdate(eqns.clone(), Arc::new(traverserintroduceDerAliasEquation), (vars.clone(), metamodelica::nil(), shared.clone(), true))?;
    eqns = __pa2.clone();
    vars = __pa3.clone();
    eqnsList = __pa4.clone();
    eqns = BackendEquation::addList(eqnsList.clone(), eqns.clone())?;
    assign_field!(
        syst.orderedEqs = eqns.clone(),
        syst.orderedVars = vars.clone()
    );
    osyst = syst.clone();
    Ok((osyst, oshared))
}

fn traverserintroduceDerAliasEquation(mut inEq: Arc<BackendDAE::Equation>, mut tpl: (BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Shared>, bool)) -> Result<(Arc<BackendDAE::Equation>, (BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Shared>, bool))> {
    let mut outEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut outTpl: (BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Shared>, bool);
    let mut e: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut vars: BackendDAE::Variables;
    let mut b: bool = false;
    let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared>;
    let mut eqnLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    (vars, eqnLst, shared, b) = tpl.clone();
    let (__pa0, (__pa1, __pa2, __pa3, __pa4, _)) = BackendEquation::traverseExpsOfEquation(inEq.clone(), Arc::new(traverserintroduceDerAliasExp.clone()), (vars.clone(), eqnLst.clone(), shared.clone(), metamodelica::nil(), b.clone()))?;
    e = __pa0.clone();
    vars = __pa1.clone();
    eqnLst = __pa2.clone();
    shared = __pa3.clone();
    ops = __pa4.clone();
    outEq = List::foldr(ops.clone(), Arc::new(BackendEquation::addOperation), e.clone());
    outTpl = (vars.clone(), eqnLst.clone(), shared.clone(), b.clone());
    Ok((outEq, outTpl))
}

fn introDerAlias(mut inExp: Arc<DAE::Exp>, mut itpl: (BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Shared>, bool, bool)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Shared>, bool, bool))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut tpl: (BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Shared>, bool, bool);
    (outExp, tpl) = 'mc: {
        let __mc_input = (inExp.clone(), itpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { ty, componentRef: cr }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, (vars, eqnLst, shared, addVar, _)) => {
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut v: BackendDAE::Var;
                    let mut v1: BackendDAE::Var;
                    let mut numVars: i32 = 0;
                    let mut vars = (*vars).clone();
                    let mut eqnLst = (*eqnLst).clone();
                    let mut outExp: Arc<DAE::Exp>;
                    (v, _) = BackendVariable::getVarSingle(cr.clone(), vars.clone())?;
                    cref = BackendVariable::varCref(v.clone())?;
                    v1 = BackendVariable::createAliasDerVar(cref.clone())?;
                    v1 = BackendVariable::mergeNominalAttribute(v.clone(), v1.clone(), false)?;
                    cref = BackendVariable::varCref(v1.clone())?;
                    outExp = Arc::new(DAE::Exp::CREF { componentRef: cref.clone(), ty: ty.clone() });
                    if addVar.clone() {
                        numVars = BackendVariable::varsSize(vars.clone())?;
                        vars = BackendVariable::addVar(v1.clone(), vars.clone())?;
                        eqnLst = if (numVars.clone() < BackendVariable::varsSize(vars.clone())?) {cons(Arc::new(BackendDAE::Equation::EQUATION { exp: inExp.clone(), scalar: outExp.clone(), source: DAE::emptyElementSource.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() }), eqnLst.clone())} else {eqnLst.clone()};
                    }
                    Ok((outExp.clone(), (vars.clone(), eqnLst.clone(), shared.clone(), addVar.clone(), true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, (_, _, _, _, _)) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAEOptimize.introduceDerAlias failed for: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), itpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, tpl))
}

// =============================================================================
// section for replaceDerCall
//
// =============================================================================
pub fn replaceDerCalls(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    outDAE = BackendDAEUtil::mapEqSystem(inDAE.clone(), Arc::new(replaceDerCallWork))?;
    Ok(outDAE)
}

fn replaceDerCallWork(mut inSyst: Arc<BackendDAE::EqSystem>, mut shared: Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared> = shared.clone();
    osyst = (::match_deref::match_deref! { match &(inSyst.clone()) {
        syst @ Deref @ BackendDAE::EqSystem { orderedEqs: eqns, orderedVars: vars, .. } => {
            let mut localKnowns: BackendDAE::Variables;
            let mut syst = (*syst).clone();
            let mut eqns = (*eqns).clone();
            let mut vars = (*vars).clone();
            (eqns, vars) = BackendEquation::traverseEquationArray_WithUpdate(eqns.clone(), Arc::new(traverserreplaceDerCall), vars.clone())?;
            (localKnowns, vars) = BackendVariable::traverseBackendDAEVars(vars.clone(), Arc::new(moveStatesVariables), (oshared.localKnownVars.clone(), vars.clone()))?;
            assign_field!(oshared.localKnownVars = localKnowns.clone());
            assign_field!(
                syst.orderedEqs = eqns.clone(),
                syst.orderedVars = vars.clone()
            );
            syst.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((osyst, oshared))
}

fn traverserreplaceDerCall(mut inEq: Arc<BackendDAE::Equation>, mut inVars: BackendDAE::Variables) -> Result<(Arc<BackendDAE::Equation>, BackendDAE::Variables)> {
    let mut outEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut outVars: BackendDAE::Variables = inVars.clone();
    let mut e: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>> = metamodelica::nil();
    (e, ops) = BackendEquation::traverseExpsOfEquation(inEq.clone(), Arc::new(traverserreplaceDerCallExp), metamodelica::nil())?;
    outEq = List::foldr(ops.clone(), Arc::new(BackendEquation::addOperation), e.clone());
    Ok((outEq, outVars))
}

fn traverserreplaceDerCallExp(mut inExp: Arc<DAE::Exp>, mut tpl: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>> = metamodelica::nil();
    let mut e: Arc<DAE::Exp>;
    let mut e1: Arc<DAE::Exp>;
    let mut b: bool = false;
    e = inExp.clone();
    (e1, b) = Expression::traverseExpBottomUp(e.clone(), Arc::new(replaceDerCall), false)?;
    outTpl = List::consOnTrue(b.clone(), Arc::new(DAE::SymbolicOperation::SUBSTITUTION { substitutions: list![e1.clone()], source: e.clone() }), tpl.clone());
    outExp = e1.clone();
    Ok((outExp, outTpl))
}

fn replaceDerCall(mut inExp: Arc<DAE::Exp>, mut itpl: bool) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut tpl: bool = false;
    (outExp, tpl) = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { ty, componentRef: cr }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } => {
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut outExp: Arc<DAE::Exp>;
                    cref = ComponentReference::crefPrefixDer(cr.clone());
                    outExp = Arc::new(DAE::Exp::CREF { componentRef: cref.clone(), ty: ty.clone() });
                    Ok((outExp.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAEOptimize.replaceDerCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), itpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, tpl))
}

fn moveStatesVariables(mut inVar: BackendDAE::Var, mut inTpl: (BackendDAE::Variables, BackendDAE::Variables)) -> Result<(BackendDAE::Var, (BackendDAE::Variables, BackendDAE::Variables))> {
    let mut outVar: BackendDAE::Var = inVar.clone();
    let mut outTpl: (BackendDAE::Variables, BackendDAE::Variables) = inTpl.clone();
    let () = (match inVar.clone() {
        BackendDAE::Var { varName: ref cref, varKind: BackendDAE::VarKind::STATE { .. }, .. } => {
            let mut newVar: BackendDAE::Var;
            let mut localKnowns: BackendDAE::Variables;
            let mut newVars: BackendDAE::Variables;
            let mut cref = cref.clone();
            (localKnowns, newVars) = inTpl.clone();
            newVars = BackendVariable::deleteVar(cref.clone(), newVars.clone())?;
            localKnowns = BackendVariable::addVar(inVar.clone(), localKnowns.clone())?;
            cref = ComponentReference::crefPrefixDer(cref.clone());
            newVar = BackendVariable::copyVarNewName(cref.clone(), inVar.clone());
            newVar = BackendVariable::setVarKind(newVar.clone(), crate::BackendDAE::VarKind::STATE_DER)?;
            newVars = BackendVariable::addVar(newVar.clone(), newVars.clone())?;
            outTpl = (localKnowns.clone(), newVars.clone());
            ()
        },
        _ => {
            ()
        },
    });
    Ok((outVar, outTpl))
}

// =============================================================================
// replace expression with rewritten expression
//
// =============================================================================
pub fn applyRewriteRulesBackend(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    (outDAE, _) = BackendDAEUtil::mapEqSystemAndFold(inDAE.clone(), Arc::new(fnptr!(applyRewriteRulesBackend0, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool)), false)?;
    outDAE = applyRewriteRulesBackendShared(outDAE.clone())?;
    Ok(outDAE)
}

fn applyRewriteRulesBackend0(mut isyst: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut inChanged: bool) -> (Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool) {
    let mut osyst: Arc<BackendDAE::EqSystem> = isyst.clone();
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut outChanged: bool = false;
    match '__try0: {
        unwrap_break_err!(BackendDAEUtil::traverseBackendDAEExpsVarsWithUpdate(isyst.orderedVars.clone(), Arc::new(traverserapplyRewriteRulesBackend), false), '__try0);
        unwrap_break_err!(BackendDAEUtil::traverseBackendDAEExpsEqns(isyst.orderedEqs.clone(), Arc::new(traverserapplyRewriteRulesBackend), false), '__try0);
        unwrap_break_err!(BackendDAEUtil::traverseBackendDAEExpsEqns(isyst.removedEqs.clone(), Arc::new(traverserapplyRewriteRulesBackend), false), '__try0);
        outChanged = true;
        Ok::<_, anyhow::Error>((outChanged.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outChanged = __try0_o0;
        }
        Err(_) => {
            outChanged = false;
        }
    }
    (osyst, outShared, outChanged)
}

fn traverserapplyRewriteRulesBackend(mut inExp: Arc<DAE::Exp>, mut inB: bool) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outB: bool = false;
    (outExp, outB) = Expression::traverseExpBottomUp(inExp.clone(), Arc::new(traverserExpapplyRewriteRulesBackend), inB.clone())?;
    Ok((outExp, outB))
}

fn traverserExpapplyRewriteRulesBackend(mut inExp: Arc<DAE::Exp>, mut inB: bool) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outB: bool = false;
    (outExp, outB) = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e => {
                    let mut e = (*e).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(RewriteRules::rewriteBackEnd(e.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e = __pa0.clone();
                    Ok((e.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inB.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outB))
}

fn applyRewriteRulesBackendShared(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    let mut shared: Arc<BackendDAE::Shared>;
    shared = inDAE.shared.clone();
    BackendDAEUtil::traverseBackendDAEExpsVarsWithUpdate(shared.globalKnownVars.clone(), Arc::new(traverserapplyRewriteRulesBackend), false)?;
    BackendDAEUtil::traverseBackendDAEExpsEqns(shared.initialEqs.clone(), Arc::new(traverserapplyRewriteRulesBackend), false)?;
    BackendDAEUtil::traverseBackendDAEExpsEqns(shared.removedEqs.clone(), Arc::new(traverserapplyRewriteRulesBackend), false)?;
    outDAE = BackendDAE::DAE(inDAE.eqs.clone(), shared.clone())?;
    Ok(outDAE)
}

// =============================================================================
// generates a list with all iteration variables
//
// =============================================================================
pub fn listAllIterationVariables(mut inBackendDAE: Arc<BackendDAE::BackendDAE>) -> Result<()> {
    let mut backendDAEType: BackendDAE::BackendDAEType = BackendDAE::BackendDAEType::ALGEQSYSTEM;
    let mut warnings: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(inBackendDAE.clone()) {
        Deref @ DAE { shared: BackendDAE::Shared { backendDAEType: __pa0, .. }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    backendDAEType = __pa0.clone();
    (warnings, _) = listAllIterationVariables0(inBackendDAE.eqs.clone())?;
    Error::addCompilerNotification(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("List of all iteration variables (DAE kind: ")); __mm_s.push_str(&*BackendDump::printBackendDAEType2String(backendDAEType.clone())?); __mm_s.push_str(&*literal!(")\n")); __mm_s.push_str(&*stringDelimitList(warnings.clone(), (literal!("\n")).clone())); ArcStr::from(__mm_s) }).clone())?;
    Ok(())
}

pub fn listAllIterationVariables0(mut inEqs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut outWarnings: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outComponentRef: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut warnings: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut warnings_accum: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
    let mut crefs_accum: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>> = metamodelica::nil();
    for mut eq in &*inEqs.clone() {
        let mut eq = eq.clone();
        (warnings, crefs) = listAllIterationVariables1(eq.clone())?;
        warnings_accum = cons(warnings.clone(), warnings_accum.clone());
        crefs_accum = cons(crefs.clone(), crefs_accum.clone());
    }
    outWarnings = List::flattenReverse(warnings_accum.clone());
    outComponentRef = List::flattenReverse(crefs_accum.clone());
    Ok((outWarnings, outComponentRef))
}

fn listAllIterationVariables1(mut inEqSystem: Arc<BackendDAE::EqSystem>) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut outWarning: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outComponentRef: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut vars: BackendDAE::Variables;
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inEqSystem.clone()) {
        Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa0, .. }, orderedVars: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    comps = __pa0.clone();
    vars = __pa1.clone();
    (outWarning, outComponentRef) = listAllIterationVariables2(comps.clone(), vars.clone())?;
    Ok((outWarning, outComponentRef))
}

fn listAllIterationVariables2(mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut vars: BackendDAE::Variables) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut warnings: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut componentRefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut var_idxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut var_idxs2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let NONLINEAR_SYSTEM: ArcStr = literal!("Iteration variables of nonlinear equation system:\n");
    let ANALYTIC_JACOBIAN: ArcStr = literal!("Iteration variables of equation system with analytic Jacobian:\n");
    let NO_ANALYTIC_JACOBIAN: ArcStr = literal!("Iteration variables of equation system without analytic Jacobian:\n");
    let TORN_LINEAR: ArcStr = literal!("Iteration variables of torn linear equation system:\n");
    let TORN_NONLINEAR: ArcStr = literal!("Iteration variables of torn nonlinear equation system:\n");
    for mut comp in &*comps.clone().reverse() {
        let mut comp = comp.clone();
        (warnings, componentRefs) = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jacType: BackendDAE::JacobianType::JAC_NONLINEAR, .. } => listAllIterationVariables3(var_field!((*comp).vars, BackendDAE::StrongComponent::EQUATIONSYSTEM).clone(), vars.clone(), (NONLINEAR_SYSTEM.clone()).clone(), warnings.clone(), componentRefs.clone())?,
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jacType: BackendDAE::JacobianType::JAC_GENERIC, .. } => listAllIterationVariables3(var_field!((*comp).vars, BackendDAE::StrongComponent::EQUATIONSYSTEM).clone(), vars.clone(), (ANALYTIC_JACOBIAN.clone()).clone(), warnings.clone(), componentRefs.clone())?,
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jacType: BackendDAE::JacobianType::JAC_NO_ANALYTIC, .. } => listAllIterationVariables3(var_field!((*comp).vars, BackendDAE::StrongComponent::EQUATIONSYSTEM).clone(), vars.clone(), (NO_ANALYTIC_JACOBIAN.clone()).clone(), warnings.clone(), componentRefs.clone())?,
        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { casualTearingSet: None, strictTearingSet: BackendDAE::TearingSet { tearingvars: var_idxs, .. }, .. } => listAllIterationVariables3(var_idxs.clone(), vars.clone(), (if (var_field!((*comp).linear, BackendDAE::StrongComponent::TORNSYSTEM).clone()) {TORN_LINEAR.clone()} else {TORN_NONLINEAR.clone()}).clone(), warnings.clone(), componentRefs.clone())?,
        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { casualTearingSet: Some(BackendDAE::TearingSet { tearingvars: var_idxs2, .. }), strictTearingSet: BackendDAE::TearingSet { tearingvars: var_idxs, .. }, .. } => listAllIterationVariables3(List::union(var_idxs.clone(), var_idxs2.clone()), vars.clone(), (if (var_field!((*comp).linear, BackendDAE::StrongComponent::TORNSYSTEM).clone()) {TORN_LINEAR.clone()} else {TORN_NONLINEAR.clone()}).clone(), warnings.clone(), componentRefs.clone())?,
        _ => (warnings.clone(), componentRefs.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((warnings, componentRefs))
}

fn listAllIterationVariables3(mut varIndices: Arc<metamodelica::List<i32>>, mut allVars: BackendDAE::Variables, mut message: ArcStr, mut warnings: Arc<metamodelica::List<ArcStr>>, mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut warnings: Arc<metamodelica::List<ArcStr>> = warnings;
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = crefs;
    let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    if !(varIndices.clone().is_empty()) {
        vars = {
        let mut __acc: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
        for mut v in (varIndices.clone()).into_iter().cloned() {
            let __x = BackendVariable::getVarAt(allVars.clone(), v.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        crefs = List::append_reverse({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        for mut v in (vars.clone()).into_iter().cloned() {
            let __x = BackendVariable::varCref(v.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, crefs.clone());
        warnings = cons({ let mut __mm_s = String::new(); __mm_s.push_str(&*message.clone()); __mm_s.push_str(&*warnAboutVars(vars.clone())?); ArcStr::from(__mm_s) }, warnings.clone());
    }
    Ok((warnings, crefs))
}

fn warnAboutVars(mut vars: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut v in (vars.clone()).into_iter().cloned() {
            let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  ")); __mm_s.push_str(&*BackendDump::varString(v.clone())?); ArcStr::from(__mm_s) };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!("\n")).clone());
    Ok(r#str)
}

pub fn addTimeAsState(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = inDAE.clone();
    let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut eq: Arc<BackendDAE::EqSystem>;
    let mut shared: Arc<BackendDAE::Shared>;
    let mut orderedVars: BackendDAE::Variables;
    let mut orderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut var: BackendDAE::Var;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(BackendDAEUtil::mapEqSystemAndFold(inDAE.clone(), Arc::new(addTimeAsState1), 0)?) {
        (Deref @ DAE { UNIQUEIO: __pa0, derivativeNamePrefix: __pa1, .. }, _) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    eqs = __pa0.clone();
    shared = __pa1.clone();
    orderedVars = BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone());
    var = BackendDAE::Var { varName: DAE::crefTimeState.clone(), varKind: BackendDAE::VarKind::STATE { index: 1, derName: None, natural: true }, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_REAL_DEFAULT.clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource.clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: true, initNonlinear: false, encrypted: false };
    var = BackendVariable::setVarFixed(var.clone(), true)?;
    var = BackendVariable::setVarStartValue(var.clone(), Arc::new(DAE::Exp::CREF { componentRef: DAE::crefTime.clone(), ty: DAE::T_REAL_DEFAULT.clone() }))?;
    orderedVars = BackendVariable::addVar(var.clone(), orderedVars.clone())?;
    orderedEqs = BackendEquation::emptyEqnsSized(1);
    orderedEqs = BackendEquation::add(Arc::new(BackendDAE::Equation::EQUATION { exp: Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("der")).clone() }), expLst: list![Arc::new(DAE::Exp::CREF { componentRef: DAE::crefTimeState.clone(), ty: DAE::T_REAL_DEFAULT.clone() })], attr: DAE::callAttrBuiltinReal.clone() }), scalar: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), source: DAE::emptyElementSource.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() }), orderedEqs.clone())?;
    eq = BackendDAEUtil::createEqSystem(orderedVars.clone(), orderedEqs.clone(), metamodelica::nil(), crate::BackendDAE::BaseClockPartitionKind::CONTINUOUS_TIME_PARTITION, BackendEquation::emptyEqns());
    outDAE = BackendDAE::DAE(cons(eq.clone(), eqs.clone()), shared.clone())?;
    Ok(outDAE)
}

fn addTimeAsState1(mut inSystem: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut inFoo: i32) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32)> {
    let mut outSystem: Arc<BackendDAE::EqSystem>;
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut outFoo: i32 = inFoo.clone();
    outSystem = 'mc: {
        let __mc_input = inSystem.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                syst @ Deref @ BackendDAE::EqSystem { orderedEqs, .. } => {
                    BackendEquation::traverseEquationArray_WithUpdate(orderedEqs.clone(), Arc::new(addTimeAsState2), inFoo.clone())?;
                    Ok(syst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inSystem.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outSystem, outShared, outFoo))
}

fn addTimeAsState2(mut inEq: Arc<BackendDAE::Equation>, mut inFoo: i32) -> Result<(Arc<BackendDAE::Equation>, i32)> {
    let mut outEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut outFoo: i32 = inFoo.clone();
    (outEq, _) = BackendEquation::traverseExpsOfEquation(inEq.clone(), Arc::new(addTimeAsState3), inFoo.clone())?;
    Ok((outEq, outFoo))
}

fn addTimeAsState3(mut inExp: Arc<DAE::Exp>, mut inTuple: i32) -> Result<(Arc<DAE::Exp>, i32)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTuple: i32 = 0;
    (outExp, outTuple) = Expression::traverseExpTopDown(inExp.clone(), Arc::new(fnptr!(addTimeAsState4, Arc<DAE::Exp>, i32)), inTuple.clone())?;
    Ok((outExp, outTuple))
}

fn addTimeAsState4(mut inExp: Arc<DAE::Exp>, mut inTuple: i32) -> (Arc<DAE::Exp>, bool, i32) {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = true;
    let mut outTuple: i32 = inTuple.clone();
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CREF { ty, componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. } } => {
            Arc::new(DAE::Exp::CREF { componentRef: DAE::crefTimeState.clone(), ty: ty.clone() })
        },
        _ => {
            inExp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, cont, outTuple)
}

//-------------------------------------
//Evaluate Output Variables Only.
//-------------------------------------
fn stateVarIsNotVisited(mut idx: i32, mut varArr: metamodelica::Array<i32>) -> Result<bool> {
    let mut b: bool = false;
    b = intLt(varArr.clone().borrow()[(idx.clone()-1) as usize].clone(), 0);
    Ok(b)
}

fn replaceDerCallOutputsOnly(mut exp: Arc<DAE::Exp>, mut der_replacement: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Exp>>>) -> (Arc<DAE::Exp>, Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Exp>>>) {
    let mut exp: Arc<DAE::Exp> = exp;
    let mut der_replacement: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Exp>>> = der_replacement;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } => {
            UnorderedMap::getOrDefault(cr.clone(), der_replacement.clone(), exp.clone())
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (exp, der_replacement)
}

// =============================================================================
// section for initOptModule >>inlineHomotopy<<
//
// =============================================================================
pub fn inlineHomotopy(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = inDAE.clone();
    let mut orderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut orderedVars: BackendDAE::Variables;
    let mut foundHomotopy: bool = false;
    for mut syst in &*inDAE.eqs.clone() {
        let mut syst = syst.clone();
        orderedEqs = syst.orderedEqs.clone();
        (orderedEqs, foundHomotopy) = BackendEquation::traverseEquationArray_WithUpdate(orderedEqs.clone(), Arc::new(inlineHomotopy2), false)?;
        assign_field!(syst.orderedEqs = orderedEqs.clone());
    }
    Ok(outDAE)
}

fn inlineHomotopy2(mut inEq: Arc<BackendDAE::Equation>, mut inFoundHomotopy: bool) -> Result<(Arc<BackendDAE::Equation>, bool)> {
    let mut outEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut outFoundHomotopy: bool = inFoundHomotopy.clone();
    (outEq, outFoundHomotopy) = BackendEquation::traverseExpsOfEquation(inEq.clone(), Arc::new(inlineHomotopy3), inFoundHomotopy.clone())?;
    Ok((outEq, outFoundHomotopy))
}

fn inlineHomotopy3(mut inExp: Arc<DAE::Exp>, mut inFoundHomotopy: bool) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outFoundHomotopy: bool = inFoundHomotopy.clone();
    (outExp, outFoundHomotopy) = Expression::traverseExpTopDown(inExp.clone(), Arc::new(replaceHomotopyWithLambdaExpression), inFoundHomotopy.clone())?;
    Ok((outExp, outFoundHomotopy))
}

fn replaceHomotopyWithLambdaExpression(mut inExp: Arc<DAE::Exp>, mut inFoundHomotopy: bool) -> Result<(Arc<DAE::Exp>, bool, bool)> {
    let mut outExp: Arc<DAE::Exp> = inExp.clone();
    let mut cont: bool = true;
    let mut outFoundHomotopy: bool = false;
    outFoundHomotopy = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: actual, tail: Deref @ metamodelica::List::Cons { head: simplified, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "homotopy" }, .. } => {
            let mut lambda: Arc<DAE::Exp>;
            lambda = Expression::crefExp(ComponentReferenceBasics::makeCrefIdent((arcstr::literal!(BackendDAE::homotopyLambda)).clone(), DAE::T_REAL_DEFAULT.clone(), metamodelica::nil()))?;
            outExp = Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: simplified.clone(), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT.clone() }, exp2: lambda.clone() }) }), operator: DAE::Operator::ADD { ty: DAE::T_REAL_DEFAULT.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: actual.clone(), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT.clone() }, exp2: lambda.clone() }) });
            true
        },
        _ => {
            inFoundHomotopy.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outFoundHomotopy))
}

// =============================================================================
// section for initOptModule >>generateHomotopyComponents<<
//
// =============================================================================
pub fn generateHomotopyComponents(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = inDAE.clone();
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut newEqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    if Config::adaptiveHomotopy()? {
        for mut syst in &*outDAE.eqs.clone() {
            let mut syst = syst.clone();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(syst.matching.clone()) {
                Deref @ BackendDAE::Matching::MATCHING { comps: __pa0, ass2: __pa1, ass1: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            comps = __pa0.clone();
            ass2 = __pa1.clone();
            ass1 = __pa2.clone();
            if Config::globalHomotopy()? {
                (comps, syst) = traverseStrongComponentsForHomotopyLoop(comps.clone(), syst.clone())?;
            } else {
                (comps, syst) = traverseStrongComponentsAddLambda(comps.clone(), syst.clone())?;
            }
            assign_field!(syst.matching = Arc::new(BackendDAE::Matching::MATCHING { comps: comps.clone(), ass2: ass2.clone(), ass1: ass1.clone() }));
            newEqSystems = cons(syst.clone(), newEqSystems.clone());
        }
        assign_field!(outDAE.eqs = newEqSystems.clone().reverse());
    } else {
        Error::addCompilerWarning((literal!("InitOptModule generateHomotopyComponents is activated for an equidistant homotopy method and will therefore be ignored.")).clone())?;
    }
    Ok(outDAE)
}

fn traverseStrongComponentsForHomotopyLoop(mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut system: Arc<BackendDAE::EqSystem>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<BackendDAE::EqSystem>)> {
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = comps;
    let mut system: Arc<BackendDAE::EqSystem> = system;
    let mut nComps: i32 = 0;
    let mut compIndex: i32 = 0;
    let mut homotopyLoopBeginning: i32 = 0;
    let mut homotopyLoopEnd: i32 = 0;
    let mut preHomotopyComponents: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut homotopyComponents: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut postHomotopyComponents: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut homotopyComponent: Arc<BackendDAE::StrongComponent>;
    let mut lambda: BackendDAE::Var;
    let mut lambdaIdx: i32 = 0;
    nComps = (comps.clone().len() as i32);
    for mut comp in &*comps.clone() {
        let mut comp = comp.clone();
        compIndex = compIndex.clone() + 1;
        let _ = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { eqn: eqnIndex, .. } => {
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut hasHomotopy: bool = false;
            eqn = BackendEquation::get(system.orderedEqs.clone(), eqnIndex.clone());
            (_, hasHomotopy) = BackendEquation::traverseExpsOfEquation(eqn.clone(), Arc::new(BackendDAEUtil::containsHomotopyCall), false)?;
            if hasHomotopy.clone() {
                homotopyLoopEnd = compIndex.clone();
                if homotopyLoopBeginning.clone() == 0 {
                    homotopyLoopBeginning = compIndex.clone();
                }
            }
            ()
        },
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: eqnIndexes, .. } => {
            let mut eqnLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut hasHomotopy: bool = false;
            if homotopyLoopBeginning.clone() == 0 {
                eqnLst = BackendEquation::getList(eqnIndexes.clone(), system.orderedEqs.clone());
                (_, hasHomotopy) = BackendEquation::traverseExpsOfEquationList(eqnLst.clone(), Arc::new(BackendDAEUtil::containsHomotopyCall), false)?;
                if hasHomotopy.clone() {
                    homotopyLoopBeginning = compIndex.clone();
                    homotopyLoopEnd = compIndex.clone();
                }
            } else {
                homotopyLoopEnd = compIndex.clone();
            }
            ()
        },
        Deref @ BackendDAE::StrongComponent::SINGLEARRAY { eqn: eqnIndex, .. } => {
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut hasHomotopy: bool = false;
            eqn = BackendEquation::get(system.orderedEqs.clone(), eqnIndex.clone());
            (_, hasHomotopy) = BackendEquation::traverseExpsOfEquation(eqn.clone(), Arc::new(BackendDAEUtil::containsHomotopyCall), false)?;
            if hasHomotopy.clone() {
                homotopyLoopEnd = compIndex.clone();
                if homotopyLoopBeginning.clone() == 0 {
                    homotopyLoopBeginning = compIndex.clone();
                }
            }
            ()
        },
        Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { eqn: eqnIndex, .. } => {
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut hasHomotopy: bool = false;
            eqn = BackendEquation::get(system.orderedEqs.clone(), eqnIndex.clone());
            (_, hasHomotopy) = BackendEquation::traverseExpsOfEquation(eqn.clone(), Arc::new(BackendDAEUtil::containsHomotopyCall), false)?;
            if hasHomotopy.clone() {
                homotopyLoopEnd = compIndex.clone();
                if homotopyLoopBeginning.clone() == 0 {
                    homotopyLoopBeginning = compIndex.clone();
                }
            }
            ()
        },
        Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { eqn: eqnIndex, .. } => {
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut hasHomotopy: bool = false;
            eqn = BackendEquation::get(system.orderedEqs.clone(), eqnIndex.clone());
            (_, hasHomotopy) = BackendEquation::traverseExpsOfEquation(eqn.clone(), Arc::new(BackendDAEUtil::containsHomotopyCall), false)?;
            if hasHomotopy.clone() {
                homotopyLoopEnd = compIndex.clone();
                if homotopyLoopBeginning.clone() == 0 {
                    homotopyLoopBeginning = compIndex.clone();
                }
            }
            ()
        },
        Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { eqn: eqnIndex, .. } => {
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut hasHomotopy: bool = false;
            eqn = BackendEquation::get(system.orderedEqs.clone(), eqnIndex.clone());
            (_, hasHomotopy) = BackendEquation::traverseExpsOfEquation(eqn.clone(), Arc::new(BackendDAEUtil::containsHomotopyCall), false)?;
            if hasHomotopy.clone() {
                homotopyLoopEnd = compIndex.clone();
                if homotopyLoopBeginning.clone() == 0 {
                    homotopyLoopBeginning = compIndex.clone();
                }
            }
            ()
        },
        Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { eqn: eqnIndex, .. } => {
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut hasHomotopy: bool = false;
            eqn = BackendEquation::get(system.orderedEqs.clone(), eqnIndex.clone());
            (_, hasHomotopy) = BackendEquation::traverseExpsOfEquation(eqn.clone(), Arc::new(BackendDAEUtil::containsHomotopyCall), false)?;
            if hasHomotopy.clone() {
                homotopyLoopEnd = compIndex.clone();
                if homotopyLoopBeginning.clone() == 0 {
                    homotopyLoopBeginning = compIndex.clone();
                }
            }
            ()
        },
        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { innerEquations, residualequations: resEqnIndexes, .. }, .. } => {
            let mut innerEqnIndexes: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut eqnLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut hasHomotopy: bool = false;
            if homotopyLoopBeginning.clone() == 0 {
                eqnLst = BackendEquation::getList(resEqnIndexes.clone(), system.orderedEqs.clone());
                (_, hasHomotopy) = BackendEquation::traverseExpsOfEquationList(eqnLst.clone(), Arc::new(BackendDAEUtil::containsHomotopyCall), false)?;
                if !(hasHomotopy.clone()) {
                    (innerEqnIndexes, _, _) = List::map_3(innerEquations.clone(), Arc::new(BackendDAEUtil::getEqnAndVarsFromInnerEquation));
                    eqnLst = BackendEquation::getList(innerEqnIndexes.clone(), system.orderedEqs.clone());
                    (_, hasHomotopy) = BackendEquation::traverseExpsOfEquationList(eqnLst.clone(), Arc::new(BackendDAEUtil::containsHomotopyCall), false)?;
                }
                if hasHomotopy.clone() {
                    homotopyLoopBeginning = compIndex.clone();
                    homotopyLoopEnd = compIndex.clone();
                }
            } else {
                homotopyLoopEnd = compIndex.clone();
            }
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    }
    if homotopyLoopBeginning.clone() > 0 {
        lambda = BackendDAE::Var { varName: ComponentReferenceBasics::makeCrefIdent((arcstr::literal!(BackendDAE::homotopyLambda)).clone(), DAE::T_REAL_DEFAULT.clone(), metamodelica::nil()), varKind: crate::BackendDAE::VarKind::VARIABLE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_REAL_DEFAULT.clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource.clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: true, initNonlinear: false, encrypted: false };
        assign_field!(system.orderedVars = BackendVariable::addVar(lambda.clone(), system.orderedVars.clone())?);
        lambdaIdx = BackendVariable::varsSize(system.orderedVars.clone())?;
        (preHomotopyComponents, homotopyComponents, postHomotopyComponents) = getHomotopyComponents(List::intRange(nComps.clone()), comps.clone(), homotopyLoopBeginning.clone(), homotopyLoopEnd.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil())?;
        homotopyComponent = createOneHomotopyComponent(homotopyComponents.clone(), system.clone(), lambdaIdx.clone())?;
        comps = cons(homotopyComponent.clone(), postHomotopyComponents.clone());
        comps = listAppend(preHomotopyComponents.clone(), comps.clone());
    }
    Ok((comps, system))
}

fn getHomotopyComponents(mut componentIndexes: Arc<metamodelica::List<i32>>, mut components: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut homotopyLoopBeginning: i32, mut homotopyLoopEnd: i32, mut outPreHomotopyComponents: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut outHomotopyComponents: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut outPostHomotopyComponents: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>)> {
    let mut outPreHomotopyComponents: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = outPreHomotopyComponents;
    let mut outHomotopyComponents: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = outHomotopyComponents;
    let mut outPostHomotopyComponents: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = outPostHomotopyComponents;
    (outPreHomotopyComponents, outHomotopyComponents, outPostHomotopyComponents) = (::match_deref::match_deref! { match &((componentIndexes.clone(), components.clone())) {
        (Deref @ metamodelica::List::Cons { head: i, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: comp, tail: Deref @ metamodelica::List::Nil }) => {
            if intLt(i.clone(), homotopyLoopBeginning.clone()) {
                outPreHomotopyComponents = cons(comp.clone(), outPreHomotopyComponents.clone());
            } else if intGt(i.clone(), homotopyLoopEnd.clone()) {
                outPostHomotopyComponents = cons(comp.clone(), outPostHomotopyComponents.clone());
            } else {
                outHomotopyComponents = cons(comp.clone(), outHomotopyComponents.clone());
            }
            (outPreHomotopyComponents.clone().reverse(), outHomotopyComponents.clone().reverse(), outPostHomotopyComponents.clone().reverse())
        },
        (Deref @ metamodelica::List::Cons { head: i, tail: indexes }, Deref @ metamodelica::List::Cons { head: comp, tail: comps }) => {
            if intLt(i.clone(), homotopyLoopBeginning.clone()) {
                outPreHomotopyComponents = cons(comp.clone(), outPreHomotopyComponents.clone());
            } else if intGt(i.clone(), homotopyLoopEnd.clone()) {
                outPostHomotopyComponents = cons(comp.clone(), outPostHomotopyComponents.clone());
            } else {
                outHomotopyComponents = cons(comp.clone(), outHomotopyComponents.clone());
            }
            getHomotopyComponents(indexes.clone(), comps.clone(), homotopyLoopBeginning.clone(), homotopyLoopEnd.clone(), outPreHomotopyComponents.clone(), outHomotopyComponents.clone(), outPostHomotopyComponents.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outPreHomotopyComponents, outHomotopyComponents, outPostHomotopyComponents))
}

fn createOneHomotopyComponent(mut homotopyComponents: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut inSystem: Arc<BackendDAE::EqSystem>, mut lambdaIdx: i32) -> Result<Arc<BackendDAE::StrongComponent>> {
    let mut outHomotopyComponent: Arc<BackendDAE::StrongComponent>;
    let mut newInnerEquations: Arc<metamodelica::List<BackendDAE::InnerEquation>> = metamodelica::nil();
    let mut newResEquations: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut newIterationVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut isMixed: bool = false;
    for mut comp in &*homotopyComponents.clone() {
        let mut comp = comp.clone();
        (newInnerEquations, newResEquations, newIterationVars) = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { var: varIndex, eqn: eqnIndex } => {
            let mut newInnerEquation: BackendDAE::InnerEquation;
            newInnerEquation = BackendDAE::InnerEquation::INNEREQUATION { vars: list![varIndex.clone()], eqn: eqnIndex.clone() };
            (cons(newInnerEquation.clone(), newInnerEquations.clone()), newResEquations.clone(), newIterationVars.clone())
        },
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { mixedSystem, vars: varIndexes, eqns: eqnIndexes, .. } => {
            if mixedSystem.clone() {
                isMixed = true;
            }
            (newInnerEquations.clone(), listAppend(newResEquations.clone(), eqnIndexes.clone()), listAppend(newIterationVars.clone(), varIndexes.clone()))
        },
        Deref @ BackendDAE::StrongComponent::SINGLEARRAY { vars: varIndexes, eqn: eqnIndex } => {
            let mut newInnerEquation: BackendDAE::InnerEquation;
            newInnerEquation = BackendDAE::InnerEquation::INNEREQUATION { vars: varIndexes.clone(), eqn: eqnIndex.clone() };
            (cons(newInnerEquation.clone(), newInnerEquations.clone()), newResEquations.clone(), newIterationVars.clone())
        },
        Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { vars: varIndexes, eqn: eqnIndex } => {
            let mut newInnerEquation: BackendDAE::InnerEquation;
            newInnerEquation = BackendDAE::InnerEquation::INNEREQUATION { vars: varIndexes.clone(), eqn: eqnIndex.clone() };
            (cons(newInnerEquation.clone(), newInnerEquations.clone()), newResEquations.clone(), newIterationVars.clone())
        },
        Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { vars: varIndexes, eqn: eqnIndex } => {
            let mut newInnerEquation: BackendDAE::InnerEquation;
            newInnerEquation = BackendDAE::InnerEquation::INNEREQUATION { vars: varIndexes.clone(), eqn: eqnIndex.clone() };
            (cons(newInnerEquation.clone(), newInnerEquations.clone()), newResEquations.clone(), newIterationVars.clone())
        },
        Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { vars: varIndexes, eqn: eqnIndex } => {
            let mut newInnerEquation: BackendDAE::InnerEquation;
            newInnerEquation = BackendDAE::InnerEquation::INNEREQUATION { vars: varIndexes.clone(), eqn: eqnIndex.clone() };
            (cons(newInnerEquation.clone(), newInnerEquations.clone()), newResEquations.clone(), newIterationVars.clone())
        },
        Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { vars: varIndexes, eqn: eqnIndex } => {
            let mut newInnerEquation: BackendDAE::InnerEquation;
            newInnerEquation = BackendDAE::InnerEquation::INNEREQUATION { vars: varIndexes.clone(), eqn: eqnIndex.clone() };
            (cons(newInnerEquation.clone(), newInnerEquations.clone()), newResEquations.clone(), newIterationVars.clone())
        },
        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { mixedSystem, strictTearingSet: BackendDAE::TearingSet { innerEquations, tearingvars: tVarIndexes, residualequations: resEqnIndexes, .. }, .. } => {
            if mixedSystem.clone() {
                isMixed = true;
            }
            for mut innerEquation in &*innerEquations.clone() {
                let mut innerEquation = innerEquation.clone();
                newInnerEquations = cons(innerEquation.clone(), newInnerEquations.clone());
            }
            (newInnerEquations.clone(), listAppend(newResEquations.clone(), resEqnIndexes.clone()), listAppend(newIterationVars.clone(), tVarIndexes.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    }
    outHomotopyComponent = Arc::new(BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: listAppend(newIterationVars.clone(), list![lambdaIdx.clone()]), residualequations: newResEquations.clone(), innerEquations: newInnerEquations.clone().reverse(), jac: Arc::new(crate::BackendDAE::Jacobian::EMPTY_JACOBIAN) }, casualTearingSet: None, linear: false, mixedSystem: isMixed.clone() });
    Ok(outHomotopyComponent)
}

fn traverseStrongComponentsAddLambda(mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut system: Arc<BackendDAE::EqSystem>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<BackendDAE::EqSystem>)> {
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = comps;
    let mut system: Arc<BackendDAE::EqSystem> = system;
    let mut newComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut lambda: BackendDAE::Var;
    let mut lambdaIdx: i32 = 0;
    let mut hasAnyHomotopy: bool = false;
    lambdaIdx = BackendVariable::varsSize(system.orderedVars.clone())? + 1;
    for mut comp in &*comps.clone() {
        let mut comp = comp.clone();
        comp = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { mixedSystem, jacType, jac, vars: varIndexes, eqns: eqnIndexes } => {
            let mut eqnLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut hasHomotopy: bool = false;
            eqnLst = BackendEquation::getList(eqnIndexes.clone(), system.orderedEqs.clone());
            (_, hasHomotopy) = BackendEquation::traverseExpsOfEquationList(eqnLst.clone(), Arc::new(BackendDAEUtil::containsHomotopyCall), false)?;
            if hasHomotopy.clone() {
                hasAnyHomotopy = true;
                comp = Arc::new(BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: eqnIndexes.clone(), vars: cons(lambdaIdx.clone(), varIndexes.clone()), jac: jac.clone(), jacType: jacType.clone(), mixedSystem: mixedSystem.clone() });
            }
            comp.clone()
        },
        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { mixedSystem, linear, casualTearingSet, strictTearingSet: BackendDAE::TearingSet { jac, innerEquations, tearingvars: tVarIndexes, residualequations: resEqnIndexes } } => {
            let mut innerEqnIndexes: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut eqnLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut hasHomotopy: bool = false;
            eqnLst = BackendEquation::getList(resEqnIndexes.clone(), system.orderedEqs.clone());
            (_, hasHomotopy) = BackendEquation::traverseExpsOfEquationList(eqnLst.clone(), Arc::new(BackendDAEUtil::containsHomotopyCall), false)?;
            if !(hasHomotopy.clone()) {
                (innerEqnIndexes, _, _) = List::map_3(innerEquations.clone(), Arc::new(BackendDAEUtil::getEqnAndVarsFromInnerEquation));
                eqnLst = BackendEquation::getList(innerEqnIndexes.clone(), system.orderedEqs.clone());
                (_, hasHomotopy) = BackendEquation::traverseExpsOfEquationList(eqnLst.clone(), Arc::new(BackendDAEUtil::containsHomotopyCall), false)?;
            }
            if hasHomotopy.clone() {
                hasAnyHomotopy = true;
                comp = Arc::new(BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: listAppend(tVarIndexes.clone(), list![lambdaIdx.clone()]), residualequations: resEqnIndexes.clone(), innerEquations: innerEquations.clone(), jac: jac.clone() }, casualTearingSet: casualTearingSet.clone(), linear: linear.clone(), mixedSystem: mixedSystem.clone() });
            }
            comp.clone()
        },
        _ => {
            comp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        newComps = cons(comp.clone(), newComps.clone());
    }
    if hasAnyHomotopy.clone() {
        lambda = BackendDAE::Var { varName: ComponentReferenceBasics::makeCrefIdent((arcstr::literal!(BackendDAE::homotopyLambda)).clone(), DAE::T_REAL_DEFAULT.clone(), metamodelica::nil()), varKind: crate::BackendDAE::VarKind::VARIABLE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_REAL_DEFAULT.clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource.clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: true, initNonlinear: false, encrypted: false };
        assign_field!(system.orderedVars = BackendVariable::addVar(lambda.clone(), system.orderedVars.clone())?);
    }
    comps = newComps.clone().reverse();
    Ok((comps, system))
}

