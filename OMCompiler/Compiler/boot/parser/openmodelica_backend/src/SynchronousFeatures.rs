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
use crate::BackendDAEOptimize;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVariable;
use openmodelica_ast::Absyn;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::DAEDump;
use openmodelica_frontend::DAEUtil;
use openmodelica_frontend::Expression;
use openmodelica_frontend::HashTable;
use openmodelica_frontend::MMath;
use openmodelica_frontend::Types;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Error;
use openmodelica_util::ErrorTypes;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

// =============================================================================
// clock partitioning
//
// =============================================================================
pub fn clockPartitioning(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    outDAE = (::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ DAE { UNIQUEIO: metamodelica::List::Cons { head: syst, tail: Deref @ metamodelica::List::Nil }, derivativeNamePrefix: shared, .. } => {
            clockPartitioning1(syst.clone(), shared.clone())?
        },
        _ => {
            let mut syst: Arc<BackendDAE::EqSystem>;
            let mut shared: Arc<BackendDAE::Shared>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(BackendDAEOptimize::collapseIndependentBlocks(inDAE.clone())?) {
                Deref @ DAE { UNIQUEIO: metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, derivativeNamePrefix: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            syst = __pa0.clone();
            shared = __pa1.clone();
            clockPartitioning1(syst.clone(), shared.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDAE)
}

pub fn synchronousFeatures(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut contSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut clockedSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared>;
    (clockedSysts, contSysts) = List::splitOnTrue(inDAE.eqs.clone(), Arc::new(fnptr!(BackendDAEUtil::isClockedSyst, Arc<BackendDAE::EqSystem>)));
    if !(clockedSysts.clone().is_empty()) {
        shared = inDAE.shared.clone();
        (clockedSysts, shared) = treatClockedStates(clockedSysts.clone(), shared.clone())?;
        systs = listAppend(contSysts.clone(), clockedSysts.clone());
        outDAE = BackendDAE::DAE(systs.clone(), shared.clone())?;
        if Flags::isSet(Flags::DUMP_SYNCHRONOUS.clone())? {
            println!("{}", (literal!("synchronous features post-phase: synchronousFeatures\n\n")).clone());
            BackendDump::dumpEqSystems(systs.clone(), (literal!("clock partitioning")).clone());
            BackendDump::dumpBasePartitions(shared.partitionsInfo.basePartitions.clone(), (literal!("Base clocks")).clone())?;
            BackendDump::dumpSubPartitions(shared.partitionsInfo.subPartitions.clone(), (literal!("Sub clocks")).clone())?;
        }
    } else {
        outDAE = inDAE.clone();
    }
    Ok(outDAE)
}

pub fn contPartitioning(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut clockedSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut clockedSysts1: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared>;
    let mut syst: Arc<BackendDAE::EqSystem>;
    let mut unpartRemEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    (clockedSysts, systs) = List::splitOnTrue(inDAE.eqs.clone(), Arc::new(fnptr!(BackendDAEUtil::isClockedSyst, Arc<BackendDAE::EqSystem>)));
    shared = inDAE.shared.clone();
    if !(systs.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(BackendDAEOptimize::collapseIndependentBlocks(BackendDAE::DAE(systs.clone(), shared.clone())?)?) {
            Deref @ DAE { UNIQUEIO: metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, derivativeNamePrefix: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        syst = __pa0.clone();
        shared = __pa1.clone();
        (systs, clockedSysts1, unpartRemEqs) = baseClockPartitioning(syst.clone(), shared.clone())?;
        assert!(clockedSysts1.clone().is_empty(), "{}", &*(literal!("Get clocked system in SynchronousFeatures.addContVarsEqs")).clone());
        assign_field!(shared.removedEqs = BackendEquation::addList(unpartRemEqs.clone(), shared.removedEqs.clone())?);
    }
    outDAE = BackendDAE::DAE(listAppend(systs.clone(), clockedSysts.clone()), shared.clone())?;
    Ok(outDAE)
}

fn clockPartitioning1(mut inSyst: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    let mut syst: Arc<BackendDAE::EqSystem>;
    let mut contSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut clockedSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut holdComps: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut unpartRemEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    syst = substitutePartitionOpExps(inSyst.clone(), inShared.clone())?;
    (contSysts, clockedSysts, unpartRemEqs) = baseClockPartitioning(syst.clone(), shared.clone())?;
    (contSysts, holdComps) = removeHoldExpsSyst(contSysts.clone())?;
    (clockedSysts, shared) = subClockPartitioning1(clockedSysts.clone(), shared.clone(), holdComps.clone())?;
    unpartRemEqs = createBoolClockWhenClauses(shared.clone(), unpartRemEqs.clone());
    assign_field!(shared.removedEqs = BackendEquation::addList(unpartRemEqs.clone(), shared.removedEqs.clone())?);
    systs = listAppend(contSysts.clone(), clockedSysts.clone());
    outDAE = BackendDAE::DAE(systs.clone(), shared.clone())?;
    if !(clockedSysts.clone().is_empty()) {
        if Flags::isSet(Flags::DUMP_SYNCHRONOUS.clone())? {
            println!("{}", (literal!("synchronous features pre-phase: synchronousFeatures\n\n")).clone());
            BackendDump::dumpEqSystems(systs.clone(), (literal!("clock partitioning")).clone());
            BackendDump::dumpBasePartitions(shared.partitionsInfo.basePartitions.clone(), (literal!("Base clocks")).clone())?;
            BackendDump::dumpSubPartitions(shared.partitionsInfo.subPartitions.clone(), (literal!("Sub clocks")).clone())?;
        }
    }
    Ok(outDAE)
}

fn createBoolClockWhenClauses(mut inShared: Arc<BackendDAE::Shared>, mut inRemovedEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Arc<metamodelica::List<Arc<BackendDAE::Equation>>> {
    let mut outRemovedEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = inRemovedEqs.clone();
    let mut basePartition: BackendDAE::BasePartition;
    let __range0 = 1..=(inShared.partitionsInfo.basePartitions.clone().borrow().len() as i32);
    for mut i in __range0 {
        basePartition = inShared.partitionsInfo.basePartitions.borrow()[(i.clone()-1) as usize].clone();
        outRemovedEqs = (::match_deref::match_deref! { match &(basePartition.clock.clone()) {
        Deref @ DAE::ClockKind::EVENT_CLOCK { condition: c, startInterval: _ } => {
            let mut e: Arc<DAE::Exp>;
            let mut whenEq: Arc<BackendDAE::WhenEquation>;
            let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            e = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("$_clkfire")).clone() }), expLst: list![Arc::new(DAE::Exp::ICONST { integer: i.clone() })], attr: DAE::callAttrBuiltinOther.clone() });
            whenEq = Arc::new(BackendDAE::WhenEquation { condition: c.clone(), whenStmtLst: list![BackendDAE::WhenOperator::NORETCALL { exp: e.clone(), source: DAE::emptyElementSource.clone() }], elsewhenPart: None });
            eq = Arc::new(BackendDAE::Equation::WHEN_EQUATION { size: 0, whenEquation: whenEq.clone(), source: DAE::emptyElementSource.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
            cons(eq.clone(), outRemovedEqs.clone())
        },
        _ => {
            outRemovedEqs.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    outRemovedEqs
}

pub fn getBoolClockWhenClauses(mut eq: Arc<BackendDAE::Equation>, mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> (Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) {
    let mut eq: Arc<BackendDAE::Equation> = eq;
    let mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = eqLst;
    if hasBoolClockWhenClause(eq.clone()) {
        eqLst = cons(eq.clone(), eqLst.clone());
    }
    (eq, eqLst)
}

fn hasBoolClockWhenClause(mut eqn: Arc<BackendDAE::Equation>) -> bool {
    let mut hasBool: bool = false;
    let _ = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: Deref @ BackendDAE::WhenEquation { whenStmtLst: Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::NORETCALL { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "$_clkfire" }, .. }, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, size: 0, .. } => {
            hasBool = true;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    hasBool
}

fn treatClockedStates(mut inSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, mut inShared: Arc<BackendDAE::Shared>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, Arc<BackendDAE::Shared>)> {
    let mut outSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared> = inShared.clone();
    outSysts = {
        let mut __acc: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
        for mut syst in (inSysts.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(syst.clone()) {
        Deref @ BackendDAE::EqSystem { orderedEqs: eqs, .. } => {
            let mut idx: i32 = 0;
            let mut subPartition: BackendDAE::SubPartition;
            let mut solverMethod: ArcStr = arcstr::literal!("");
            let mut lstEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut derVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut var: BackendDAE::Var;
            let mut exp: Arc<DAE::Exp>;
            let mut exp2: Arc<DAE::Exp>;
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut eqs = (*eqs).clone();
            let BackendDAE::CLOCKED_PARTITION { subPartIdx: __pa0 } = (syst.partitionKind.clone()) else { bail!("pattern mismatch") };
            idx = __pa0.clone();
            subPartition = shared.partitionsInfo.subPartitions.borrow()[(idx.clone()-1) as usize].clone();
            solverMethod = (BackendDump::optionString(getSubClockSolverOpt(subPartition.clock.clone()))).clone();
            if StringUtil::startsWith((solverMethod.clone()).clone(), (literal!("Explicit")).clone()) {
                if solverMethod.clone() != literal!("ExplicitEuler") {
                    Error::addMessage(Error::CLOCK_SOLVERMETHOD.clone(), list![(literal!("ExplicitEuler")).clone(), (solverMethod.clone()).clone()])?;
                    solverMethod = (literal!("ExplicitEuler")).clone();
                }
            } else if ((solverMethod.clone()).clone().len() as i32) > 0 && solverMethod.clone() != literal!("ImplicitEuler") && solverMethod.clone() != literal!("SemiImplicitEuler") && solverMethod.clone() != literal!("ImplicitTrapezoid") {
                Error::addMessage(Error::CLOCK_SOLVERMETHOD.clone(), list![(literal!("ImplicitEuler")).clone(), (solverMethod.clone()).clone()])?;
                solverMethod = (literal!("ImplicitEuler")).clone();
            }
            for mut i in 1..=BackendEquation::getNumberOfEquations(eqs.clone()) {
                eq = BackendEquation::get(eqs.clone(), i.clone());
                let (__pa1, (__pa2, _)) = BackendEquation::traverseExpsOfEquation(eq.clone(), Arc::new(getDerVars1), (derVars.clone(), BackendEquation::getForEquationIterIdent(eq.clone())))?;
                eq = __pa1.clone();
                derVars = __pa2.clone();
                lstEqs = cons(eq.clone(), lstEqs.clone());
            }
            for mut derVar in &*derVars.clone() {
                let mut derVar = derVar.clone();
                var = ((BackendVariable::getVar(derVar.clone(), syst.orderedVars.clone())?).0).get(1)?;
                var = BackendDAE::Var { varName: ComponentReference::crefPrefixDer(derVar.clone()), varKind: crate::BackendDAE::VarKind::VARIABLE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: var.varType.clone(), bindExp: None, tplExp: None, arryDim: var.arryDim.clone(), source: DAE::emptyElementSource.clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
                assign_field!(syst.orderedVars = BackendVariable::addVar(var.clone(), syst.orderedVars.clone())?);
            }
            for mut derVar in &*derVars.clone() {
                let mut derVar = derVar.clone();
                var = ((BackendVariable::getVar(derVar.clone(), syst.orderedVars.clone())?).0).get(1)?;
                ty = var.varType.clone();
                derVar = (::match_deref::match_deref! { match &(var.varType.clone()) {
        Deref @ DAE::Type::T_ARRAY { ty, .. } => ComponentReference::crefApplySubs(derVar.clone(), list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (literal!("i")).clone(), identType: DAE::T_INTEGER_DEFAULT.clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_INTEGER_DEFAULT.clone() }) })])?,
        _ => derVar.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                exp = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("der")).clone() }), expLst: list![Arc::new(DAE::Exp::CREF { componentRef: derVar.clone(), ty: ty.clone() })], attr: DAE::callAttrBuiltinImpureReal.clone() });
                (exp, _) = substituteFiniteDifference(exp.clone(), metamodelica::nil());
                exp2 = Arc::new(DAE::Exp::CREF { componentRef: ComponentReference::crefPrefixDer(derVar.clone()), ty: ty.clone() });
                if solverMethod.clone() == literal!("ExplicitEuler") {
                    exp2 = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: list![exp2.clone()], attr: DAE::callAttrBuiltinImpureReal.clone() });
                } else if solverMethod.clone() == literal!("ImplicitTrapezoid") {
                    exp2 = Arc::new(DAE::Exp::BINARY { exp1: exp2.clone(), operator: DAE::Operator::ADD { ty: DAE::T_REAL_DEFAULT.clone() }, exp2: Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: list![exp2.clone()], attr: DAE::callAttrBuiltinImpureReal.clone() }) });
                    exp2 = Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.5_f64) }), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT.clone() }, exp2: exp2.clone() });
                }
                exp2 = Arc::new(DAE::Exp::IFEXP { expCond: Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("firstTick")).clone() }), expLst: metamodelica::nil(), attr: DAE::callAttrBuiltinImpureBool.clone() }), expThen: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((0) as f64) }), expElse: exp2.clone() });
                eq = (::match_deref::match_deref! { match &(var.varType.clone()) {
        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: dim, tail: Deref @ metamodelica::List::Nil }, .. } => {
            Arc::new(BackendDAE::Equation::FOR_EQUATION { iter: Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (literal!("i")).clone(), identType: DAE::T_INTEGER_DEFAULT.clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_INTEGER_DEFAULT.clone() }), start: Arc::new(DAE::Exp::ICONST { integer: 1 }), stop: DAEUtil::dimExp(dim.clone())?, body: Arc::new(BackendDAE::Equation::EQUATION { exp: exp.clone(), scalar: exp2.clone(), source: var.source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() }), source: var.source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() })
        },
        _ => {
            Arc::new(BackendDAE::Equation::EQUATION { exp: exp.clone(), scalar: exp2.clone(), source: var.source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                lstEqs = cons(eq.clone(), lstEqs.clone());
            }
            assign_field!(syst.orderedEqs = BackendEquation::listEquation(lstEqs.clone().reverse())?);
            if solverMethod.clone() == literal!("SemiImplicitEuler") {
                for mut i in 1..=BackendEquation::getNumberOfEquations(eqs.clone()) {
                    eq = BackendEquation::get(eqs.clone(), i.clone());
                    (eq, _) = BackendEquation::traverseExpsOfEquation(eq.clone(), Arc::new(shiftDerVars1), derVars.clone())?;
                    eqs = BackendEquation::setAtIndex(eqs.clone(), i.clone(), eq.clone())?;
                }
            }
            shared = markClockedStates(syst.clone(), shared.clone(), derVars.clone())?;
            BackendDAEUtil::clearEqSyst(syst.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok((outSysts, shared))
}

fn getDerVars1(mut inExp: Arc<DAE::Exp>, mut inDerVars: (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outDerVars: (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>);
    (outExp, outDerVars) = Expression::traverseExpBottomUp(inExp.clone(), Arc::new(fnptr!(getDerVars, Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>))), inDerVars.clone())?;
    Ok((outExp, outDerVars))
}

fn getDerVars(mut inExp: Arc<DAE::Exp>, mut inDerVars: (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>)) -> (Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>)) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outDerVars: (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>) = inDerVars.clone();
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { ty, componentRef: x }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } => {
            let mut derVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut optForIter: Option<ArcStr> = None;
            let mut forIter: ArcStr = arcstr::literal!("");
            let mut der_x: Arc<DAE::Exp>;
            der_x = Arc::new(DAE::Exp::CREF { componentRef: ComponentReference::crefPrefixDer(x.clone()), ty: ty.clone() });
            (derVars, optForIter) = inDerVars.clone();
            let _ = (match optForIter.clone() {
        Some(mut forIter) => {
            x = ComponentReference::crefStripIterSub(x.clone(), (forIter.clone()).clone());
            ()
        },
        _ => (),
    });
            if !(ComponentReferenceBasics::crefInLst(x.clone(), derVars.clone())) {
                derVars = cons(x.clone(), derVars.clone());
            }
            outDerVars = (derVars.clone(), optForIter.clone());
            der_x.clone()
        },
        _ => {
            inExp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outDerVars)
}

fn shiftDerVars1(mut inExp: Arc<DAE::Exp>, mut inDerVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outDerVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    (outExp, outDerVars) = Expression::traverseExpBottomUp(inExp.clone(), Arc::new(fnptr!(shiftDerVars, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)), inDerVars.clone())?;
    Ok((outExp, outDerVars))
}

fn shiftDerVars(mut inExp: Arc<DAE::Exp>, mut inDerVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> (Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outDerVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = inDerVars.clone();
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: x, .. } if (ComponentReferenceBasics::crefInLst(x.clone(), inDerVars.clone())) => {
            let mut exp: Arc<DAE::Exp>;
            exp = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: list![inExp.clone()], attr: DAE::callAttrBuiltinImpureReal.clone() });
            exp.clone()
        },
        Deref @ DAE::Exp::CALL { attr: attr @ Deref @ DAE::CallAttributes { .. }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CALL { expLst, path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" } } => {
            let mut exp: Arc<DAE::Exp>;
            exp = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("der")).clone() }), expLst: expLst.clone(), attr: attr.clone() });
            exp.clone()
        },
        Deref @ DAE::Exp::CALL { attr: attr @ Deref @ DAE::CallAttributes { .. }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CALL { expLst, path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" } } => {
            let mut exp: Arc<DAE::Exp>;
            exp = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: expLst.clone(), attr: attr.clone() });
            exp.clone()
        },
        _ => {
            inExp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outDerVars)
}

fn substituteFiniteDifference1(mut inExp: Arc<DAE::Exp>, mut inDerVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outDerVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    (outExp, outDerVars) = Expression::traverseExpBottomUp(inExp.clone(), Arc::new(fnptr!(substituteFiniteDifference, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)), inDerVars.clone())?;
    Ok((outExp, outDerVars))
}

fn substituteFiniteDifference(mut inExp: Arc<DAE::Exp>, mut inDerVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> (Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outDerVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    (outExp, outDerVars) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { attr: attr @ Deref @ DAE::CallAttributes { ty, .. }, expLst: expLst @ Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: x, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" } } => {
            let mut exp: Arc<DAE::Exp>;
            exp = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: expLst.clone(), attr: attr.clone() });
            exp = Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::CREF { componentRef: x.clone(), ty: ty.clone() }), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT.clone() }, exp2: exp.clone() });
            exp = Arc::new(DAE::Exp::BINARY { exp1: exp.clone(), operator: DAE::Operator::DIV { ty: DAE::T_REAL_DEFAULT.clone() }, exp2: Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("interval")).clone() }), expLst: metamodelica::nil(), attr: DAE::callAttrBuiltinImpureReal.clone() }) });
            (exp.clone(), cons(x.clone(), inDerVars.clone()))
        },
        _ => {
            (inExp.clone(), inDerVars.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outDerVars)
}

fn markClockedStates(mut inSyst: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut derVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<Arc<BackendDAE::Shared>> {
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut prevVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut isPrevVarArr: metamodelica::Array<bool>;
    let mut isDerVarArr: metamodelica::Array<bool>;
    let mut varIxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut var: BackendDAE::Var;
    let mut idx: i32 = 0;
    let mut subPartition: BackendDAE::SubPartition;
    let BackendDAE::CLOCKED_PARTITION { subPartIdx: __pa0 } = (inSyst.partitionKind.clone()) else { bail!("pattern mismatch") };
    idx = __pa0.clone();
    subPartition = outShared.partitionsInfo.subPartitions.borrow()[(idx.clone()-1) as usize].clone();
    isPrevVarArr = arrayCreate(BackendVariable::varsSize(inSyst.orderedVars.clone())?, false);
    isDerVarArr = arrayCreate(BackendVariable::varsSize(inSyst.orderedVars.clone())?, false);
    for mut cr in &*derVars.clone() {
        let mut cr = cr.clone();
        varIxs = getVarIxs(cr.clone(), inSyst.orderedVars.clone())?;
        for mut idx in &*varIxs.clone() {
            let mut idx = idx.clone();
            {let _arr = isDerVarArr.clone(); _arr.borrow_mut()[(idx.clone()-1) as usize] = true; _arr};
        }
    }
    for mut i in 1..=BackendEquation::getNumberOfEquations(inSyst.orderedEqs.clone()) {
        eq = BackendEquation::get(inSyst.orderedEqs.clone(), i.clone());
        let (_, (__pa1, _)) = BackendEquation::traverseExpsOfEquation(eq.clone(), Arc::new(collectPrevVars), (prevVars.clone(), BackendEquation::getForEquationIterIdent(eq.clone())))?;
        prevVars = __pa1.clone();
    }
    for mut i in 1..=BackendEquation::getNumberOfEquations(inSyst.removedEqs.clone()) {
        eq = BackendEquation::get(inSyst.removedEqs.clone(), i.clone());
        let (_, (__pa2, _)) = BackendEquation::traverseExpsOfEquation(eq.clone(), Arc::new(collectPrevVars), (prevVars.clone(), BackendEquation::getForEquationIterIdent(eq.clone())))?;
        prevVars = __pa2.clone();
    }
    if !(Flags::isSet(Flags::NF_SCALARIZE.clone())?) {
        prevVars = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        for mut cr in (prevVars.clone()).into_iter().cloned() {
            let __x = ComponentReferenceBasics::crefStripLastSubs(cr.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    }
    for mut cr in &*prevVars.clone() {
        let mut cr = cr.clone();
        varIxs = getVarIxs(cr.clone(), inSyst.orderedVars.clone())?;
        for mut idx in &*varIxs.clone() {
            let mut idx = idx.clone();
            {let _arr = isPrevVarArr.clone(); _arr.borrow_mut()[(idx.clone()-1) as usize] = true; _arr};
        }
    }
    prevVars = metamodelica::nil();
    let __range3 = 1..=(isPrevVarArr.clone().borrow().len() as i32);
    for mut i in __range3 {
        if isPrevVarArr.borrow()[(i.clone()-1) as usize].clone() {
            var = BackendVariable::getVarAt(inSyst.orderedVars.clone(), i.clone())?;
            var = BackendVariable::setVarKind(var.clone(), BackendDAE::VarKind::CLOCKED_STATE { isStartFixed: isDerVarArr.borrow()[(i.clone()-1) as usize].clone(), previousName: ComponentReference::crefPrefixPrevious(var.varName.clone()) })?;
            var = BackendVariable::setVarFixed(var.clone(), true)?;
            BackendVariable::setVarAt(inSyst.orderedVars.clone(), i.clone(), var.clone())?;
            prevVars = cons(var.varName.clone(), prevVars.clone());
        }
    }
    subPartition.prevVars = prevVars.clone();
    {let _arr = outShared.partitionsInfo.subPartitions.clone(); _arr.borrow_mut()[(idx.clone()-1) as usize] = subPartition.clone(); _arr};
    Ok(outShared)
}

fn collectPrevVars(mut inExp: Arc<DAE::Exp>, mut inPrevVars: (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outPrevVars: (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>);
    (outExp, outPrevVars) = Expression::traverseExpBottomUp(inExp.clone(), Arc::new(fnptr!(collectPrevVars1, Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>))), inPrevVars.clone())?;
    Ok((outExp, outPrevVars))
}

fn collectPrevVars1(mut inExp: Arc<DAE::Exp>, mut inPrevVars: (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>)) -> (Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>)) {
    let mut outExp: Arc<DAE::Exp> = inExp.clone();
    let mut outPrevVars: (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>);
    outPrevVars = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. } => {
            let mut inPrevCompRefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut inForIter: Option<ArcStr> = None;
            let mut forIter: ArcStr = arcstr::literal!("");
            (inPrevCompRefs, inForIter) = inPrevVars.clone();
            let _ = (match inForIter.clone() {
        Some(mut forIter) => {
            cr = ComponentReference::crefStripIterSub(cr.clone(), (forIter.clone()).clone());
            ()
        },
        _ => (),
    });
            (cons(cr.clone(), inPrevCompRefs.clone()), inForIter.clone())
        },
        _ => {
            inPrevVars.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outPrevVars)
}

fn subClockPartitioning1(mut inSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, mut inShared: Arc<BackendDAE::Shared>, mut inHoldComps: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, Arc<BackendDAE::Shared>)> {
    let mut outSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut baseClock: Arc<DAE::ClockKind> = Arc::new(DAE::ClockKind::INFERRED_CLOCK);
    let mut varsPartition: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut n: i32 = 0;
    let mut nBaseClocks: i32 = 0;
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut hasHoldOperator: metamodelica::Array<bool>;
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut lstSubClocks1: Arc<metamodelica::List<BackendDAE::SubClock>> = metamodelica::nil();
    let mut lstSubClocks: Arc<metamodelica::List<BackendDAE::SubClock>> = metamodelica::nil();
    let mut partitionsInfo: BackendDAE::PartitionsInfo;
    let mut basePartitions: metamodelica::Array<BackendDAE::BasePartition>;
    let mut subPartitions: metamodelica::Array<BackendDAE::SubPartition>;
    nBaseClocks = (inSysts.clone().len() as i32);
    basePartitions = arrayCreate(nBaseClocks.clone(), BackendDAE::BasePartition { clock: Arc::new(openmodelica_frontend_types::DAE::ClockKind::INFERRED_CLOCK), nSubClocks: 0 });
    varsPartition = HashTable::emptyHashTable();
    i = 0;
    j = 1;
    for mut syst in &*inSysts.clone() {
        let mut syst = syst.clone();
        (systs, baseClock, lstSubClocks1) = subClockPartitioning(syst.clone(), outShared.clone(), i.clone())?;
        n = (systs.clone().len() as i32);
        {let _arr = basePartitions.clone(); _arr.borrow_mut()[(j.clone()-1) as usize] = BackendDAE::BasePartition { clock: baseClock.clone(), nSubClocks: n.clone() }; _arr};
        outSysts = List::append_reverse(systs.clone(), outSysts.clone());
        lstSubClocks = List::append_reverse(lstSubClocks1.clone(), lstSubClocks.clone());
        i = i.clone() + n.clone();
        j = j.clone() + 1;
    }
    outSysts = outSysts.clone().reverse();
    lstSubClocks = lstSubClocks.clone().reverse();
    hasHoldOperator = arrayCreate((lstSubClocks.clone().len() as i32), false);
    i = 1;
    for mut syst in &*outSysts.clone() {
        let mut syst = syst.clone();
        for mut j in 1..=BackendVariable::varsSize(syst.orderedVars.clone())? {
            let BackendDAE::VAR { varName: __pa0, .. } = (BackendVariable::getVarAt(syst.orderedVars.clone(), j.clone())?) else { bail!("pattern mismatch") };
            cr = __pa0.clone();
            varsPartition = BaseHashTable::add((cr.clone(), i.clone()), varsPartition.clone())?;
        }
        i = i.clone() + 1;
    }
    for mut cr in &*inHoldComps.clone() {
        let mut cr = cr.clone();
        i = BaseHashTable::get(cr.clone(), varsPartition.clone())?;
        {let _arr = hasHoldOperator.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = true; _arr};
    }
    i = 1;
    subPartitions = arrayCreate((lstSubClocks.clone().len() as i32), BackendDAE::SubPartition { clock: BackendDAE::DEFAULT_SUBCLOCK.clone(), holdEvents: false, prevVars: metamodelica::nil() });
    for mut subclock in &*lstSubClocks.clone() {
        let mut subclock = subclock.clone();
        {let _arr = subPartitions.clone(); let _val = BackendDAE::SubPartition { clock: subclock.clone(), holdEvents: hasHoldOperator.borrow()[(i.clone()-1) as usize].clone(), prevVars: metamodelica::nil() }; _arr.borrow_mut()[(i.clone()-1) as usize] = _val; _arr};
        i = i.clone() + 1;
    }
    partitionsInfo = outShared.partitionsInfo.clone();
    partitionsInfo.basePartitions = basePartitions.clone();
    partitionsInfo.subPartitions = subPartitions.clone();
    assign_field!(outShared.partitionsInfo = partitionsInfo.clone());
    Ok((outSysts, outShared))
}

fn removeHoldExpsSyst(mut inSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut outSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut outHoldComps: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    for mut syst1 in &*inSysts.clone() {
        let mut syst1 = syst1.clone();
        syst1 = (::match_deref::match_deref! { match &(syst1.clone()) {
        syst @ Deref @ BackendDAE::EqSystem { orderedEqs: eqs, .. } => {
            let mut lstEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut i: i32 = 0;
            let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut syst = (*syst).clone();
            lstEqs = metamodelica::nil();
            for mut i in 1..=BackendEquation::getNumberOfEquations(eqs.clone()) {
                eq = BackendEquation::get(eqs.clone(), i.clone());
                (eq, outHoldComps) = BackendEquation::traverseExpsOfEquation(eq.clone(), Arc::new(removeHoldExp1), outHoldComps.clone())?;
                lstEqs = cons(eq.clone(), lstEqs.clone());
            }
            assign_field!(syst.orderedEqs = BackendEquation::listEquation(lstEqs.clone().reverse())?);
            syst.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
        outSysts = cons(BackendDAEUtil::clearEqSyst(syst1.clone())?, outSysts.clone());
    }
    Ok((outSysts, outHoldComps))
}

fn removeHoldExp1(mut inExp: Arc<DAE::Exp>, mut inComps: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outComps: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    (outExp, outComps) = Expression::traverseExpBottomUp(inExp.clone(), Arc::new(removeHoldExp), inComps.clone())?;
    Ok((outExp, outComps))
}

fn removeHoldExp(mut inExp: Arc<DAE::Exp>, mut inComps: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outComps: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    (outExp, outComps) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "hold" }, expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, attr: _ } => {
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let __pa0 = ::match_deref::match_deref! { match &(e.clone()) {
                Deref @ DAE::Exp::CREF { componentRef: __pa0, ty: _ } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cr = __pa0.clone();
            (substGetPartition(e.clone())?, cons(cr.clone(), inComps.clone()))
        },
        _ => {
            (inExp.clone(), inComps.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outComps))
}

fn getSubPartitionAdjacency(mut numPartitions: i32, mut baseClockEq: i32, mut subPartitionInterfaceEqs: Arc<metamodelica::List<i32>>, mut eqPartMap: metamodelica::Array<i32>, mut varPartMap: metamodelica::Array<i32>, mut clockedVarsMask: metamodelica::Array<bool>, mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut vars: BackendDAE::Variables) -> Result<(metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::SubClock)>>>, metamodelica::Array<i32>)> {
    let mut partAdjacency: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::SubClock)>>>;
    let mut order: metamodelica::Array<i32>;
    let mut infered: bool = false;
    let mut part: i32 = 0;
    let mut part1: i32 = 0;
    let mut part2: i32 = 0;
    let mut var1: i32 = 0;
    let mut var2: i32 = 0;
    let mut partLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut orderLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut subClk1: BackendDAE::SubClock = BackendDAE::SubClock::INFERED_SUBCLOCK;
    let mut subClk2: BackendDAE::SubClock = BackendDAE::SubClock::INFERED_SUBCLOCK;
    let mut partIsAssigned: metamodelica::Array<bool>;
    let mut adjParts: Arc<metamodelica::List<(i32, BackendDAE::SubClock)>> = metamodelica::nil();
    let mut partitionParents: metamodelica::Array<i32>;
    let mut partitionParentsVisited: metamodelica::Array<bool>;
    let mut partitionInterfacesClockVars: metamodelica::Array<bool>;
    partAdjacency = arrayCreate(numPartitions.clone(), metamodelica::nil());
    partitionParents = arrayCreate(numPartitions.clone(), -1);
    partitionInterfacesClockVars = arrayCreate(numPartitions.clone(), false);
    for mut subPartEq in &*subPartitionInterfaceEqs.clone() {
        let mut subPartEq = subPartEq.clone();
        (infered, part1, var1, subClk1, part2, var2, subClk2) = getConnectedSubPartitions(BackendEquation::get(eqs.clone(), subPartEq.clone()), varPartMap.clone(), vars.clone())?;
        if part1.clone() != 0 && part2.clone() != 0 {
            addPartAdjacencyEdge(part1.clone(), subClk1.clone(), part2.clone(), subClk2.clone(), partAdjacency.clone())?;
        }
        if partitionParents.borrow()[(part2.clone()-1) as usize].clone() == part1.clone() && partitionInterfacesClockVars.borrow()[(part2.clone()-1) as usize].clone() {
            {
                let __cell0 = -1;
                partitionParents.clone().borrow_mut()[(part2.clone()-1) as usize] = __cell0;
            }
        }
        {
            let __cell1 = !(clockedVarsMask.borrow()[(var1.clone()-1) as usize].clone() && clockedVarsMask.borrow()[(var2.clone()-1) as usize].clone());
            partitionInterfacesClockVars.clone().borrow_mut()[(part1.clone()-1) as usize] = __cell1;
        }
        if partitionParents.borrow()[(part2.clone()-1) as usize].clone() != part1.clone() {
            {
                let __cell2 = part2.clone();
                partitionParents.clone().borrow_mut()[(part1.clone()-1) as usize] = __cell2;
            }
        }
    }
    partLst = List::intRange(numPartitions.clone());
    partitionParentsVisited = arrayCreate(numPartitions.clone(), false);
    orderLst = metamodelica::nil();
    while !(partLst.clone().is_empty()) {
        let (__pa3, __pa4) = ::match_deref::match_deref! { match &(partLst.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
            _ => bail!("pattern mismatch"),
        } };
        part = __pa3.clone();
        partLst = __pa4.clone();
        if !(partitionParentsVisited.borrow()[(part.clone()-1) as usize].clone()) {
            if partitionParents.borrow()[(part.clone()-1) as usize].clone() == -1 || partitionParents.borrow()[(part.clone()-1) as usize].clone() == part.clone() {
                orderLst = cons(part.clone(), orderLst.clone());
                {
                    let __cell5 = true;
                    partitionParentsVisited.clone().borrow_mut()[(part.clone()-1) as usize] = __cell5;
                }
            } else if partitionParentsVisited.borrow()[(partitionParents.borrow()[(part.clone()-1) as usize].clone()-1) as usize].clone() {
                orderLst = cons(part.clone(), orderLst.clone());
                {
                    let __cell6 = true;
                    partitionParentsVisited.clone().borrow_mut()[(part.clone()-1) as usize] = __cell6;
                }
            } else {
                partLst = cons(part.clone(), partLst.clone());
                partLst = cons(partitionParents.borrow()[(part.clone()-1) as usize].clone(), partLst.clone());
            }
        }
    }
    order = metamodelica::arrayFromVec(orderLst.clone().reverse().into_iter().cloned().collect());
    Ok((partAdjacency, order))
}

fn getSubClockForClkConstructor(mut refClock: Arc<DAE::ClockKind>, mut clk: Arc<DAE::ClockKind>) -> Result<BackendDAE::SubClock> {
    let mut subClk: BackendDAE::SubClock = BackendDAE::SubClock::INFERED_SUBCLOCK;
    subClk = (::match_deref::match_deref! { match &((refClock.clone(), clk.clone())) {
        (Deref @ DAE::ClockKind::RATIONAL_CLOCK { intervalCounter: Deref @ DAE::Exp::ICONST { integer: i1 }, resolution: Deref @ DAE::Exp::ICONST { integer: i2 } }, Deref @ DAE::ClockKind::INFERRED_CLOCK) => {
            BackendDAE::SubClock::SUBCLOCK { factor: MMath::Rational { nom: i2.clone(), denom: i1.clone() }, shift: MMath::RAT0.clone(), solver: None }
        },
        (Deref @ DAE::ClockKind::RATIONAL_CLOCK { intervalCounter: Deref @ DAE::Exp::ICONST { integer: i1 }, resolution: Deref @ DAE::Exp::ICONST { integer: i2 } }, Deref @ DAE::ClockKind::RATIONAL_CLOCK { intervalCounter: Deref @ DAE::Exp::ICONST { integer: i3 }, resolution: Deref @ DAE::Exp::ICONST { integer: i4 } }) => {
            BackendDAE::SubClock::SUBCLOCK { factor: MMath::divRational(MMath::Rational { nom: i2.clone(), denom: i1.clone() }, MMath::Rational { nom: i4.clone(), denom: i3.clone() })?, shift: MMath::RAT0.clone(), solver: None }
        },
        (Deref @ DAE::ClockKind::REAL_CLOCK { interval: Deref @ DAE::Exp::RCONST { real: r1 } }, Deref @ DAE::ClockKind::INFERRED_CLOCK) => {
            BackendDAE::SubClock::SUBCLOCK { factor: MMath::Rational { nom: 1, denom: ((metamodelica::OrderedFloat(1.0_f64) / r1.clone()).0 as i32) }, shift: MMath::RAT0.clone(), solver: None }
        },
        (Deref @ DAE::ClockKind::REAL_CLOCK { interval: Deref @ DAE::Exp::RCONST { real: r1 } }, Deref @ DAE::ClockKind::REAL_CLOCK { interval: Deref @ DAE::Exp::RCONST { real: r2 } }) => {
            BackendDAE::SubClock::SUBCLOCK { factor: MMath::divRational(MMath::Rational { nom: 1, denom: ((metamodelica::OrderedFloat(1.0_f64) / r1.clone()).0 as i32) }, MMath::Rational { nom: 1, denom: ((metamodelica::OrderedFloat(1.0_f64) / r2.clone()).0 as i32) })?, shift: MMath::RAT0.clone(), solver: None }
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SynchronousFeatures.getSubClockForClkConstructor")); __mm_s.push_str(&*literal!(" failed.\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(subClk)
}

fn setSolverSubClock(mut baseClkIn: Arc<DAE::ClockKind>, mut inSubClock: BackendDAE::SubClock) -> (Arc<DAE::ClockKind>, BackendDAE::SubClock) {
    let mut baseClkOut: Arc<DAE::ClockKind> = Arc::new(DAE::ClockKind::INFERRED_CLOCK);
    let mut outSubClock: BackendDAE::SubClock = BackendDAE::SubClock::INFERED_SUBCLOCK;
    (baseClkOut, outSubClock) = (::match_deref::match_deref! { match &(baseClkIn.clone()) {
        Deref @ DAE::ClockKind::SOLVER_CLOCK { solverMethod: Deref @ DAE::Exp::SCONST { string: solver }, c: Deref @ DAE::Exp::CLKCONST { clk } } => {
            outSubClock = setSubClockSolver(inSubClock.clone(), if (solver.clone() == literal!("")) {None} else {Some((solver.clone()).clone())});
            (clk.clone(), outSubClock.clone())
        },
        _ => {
            (baseClkIn.clone(), inSubClock.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (baseClkOut, outSubClock)
}

fn findSubClocks(mut numPartitions: i32, mut baseClockEq: i32, mut baseClk: Arc<DAE::ClockKind>, mut baseClockConstructors: Arc<metamodelica::List<i32>>, mut subPartitionInterfaceEqs: Arc<metamodelica::List<i32>>, mut eqPartMap: metamodelica::Array<i32>, mut varPartMap: metamodelica::Array<i32>, mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut partAdjacency: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::SubClock)>>>) -> Result<(Arc<DAE::ClockKind>, metamodelica::Array<BackendDAE::SubClock>)> {
    let mut baseClkOut: Arc<DAE::ClockKind> = Arc::new(DAE::ClockKind::INFERRED_CLOCK);
    let mut outSubClocks: metamodelica::Array<BackendDAE::SubClock>;
    let mut part1: i32 = 0;
    let mut part2: i32 = 0;
    let mut ord: i32 = 0;
    let mut partLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut subClk1: BackendDAE::SubClock = BackendDAE::SubClock::INFERED_SUBCLOCK;
    let mut subClk2: BackendDAE::SubClock = BackendDAE::SubClock::INFERED_SUBCLOCK;
    let mut clk: Arc<DAE::ClockKind> = Arc::new(DAE::ClockKind::INFERRED_CLOCK);
    let mut partIsAssigned: metamodelica::Array<bool>;
    let mut adjParts: Arc<metamodelica::List<(i32, BackendDAE::SubClock)>> = metamodelica::nil();
    outSubClocks = arrayCreate(numPartitions.clone(), BackendDAE::DEFAULT_SUBCLOCK.clone());
    partIsAssigned = arrayCreate(numPartitions.clone(), false);
    for mut clockEq in &*baseClockConstructors.clone() {
        let mut clockEq = clockEq.clone();
        if !(intEq(baseClockEq.clone(), clockEq.clone())) && !(intEq(baseClockEq.clone(), -1)) {
            part1 = eqPartMap.clone().borrow()[(clockEq.clone()-1) as usize].clone();
            clk = getBaseClock(BackendEquation::get(eqs.clone(), clockEq.clone()));
            if !(isInferedBaseClock(clk.clone())) {
                subClk1 = getSubClockForClkConstructor(baseClk.clone(), clk.clone())?;
                {let _arr = outSubClocks.clone(); _arr.borrow_mut()[(part1.clone()-1) as usize] = subClk1.clone(); _arr};
                {let _arr = partIsAssigned.clone(); _arr.borrow_mut()[(part1.clone()-1) as usize] = true; _arr};
            }
        }
    }
    if isInferedBaseClock(baseClk.clone()) {
        baseClkOut = baseClk.clone();
        partLst = List::intRange(numPartitions.clone());
    } else {
        part1 = eqPartMap.clone().borrow()[(baseClockEq.clone()-1) as usize].clone();
        partLst = cons(part1.clone(), List::intRange(numPartitions.clone()));
        (baseClkOut, subClk1) = setSolverSubClock(baseClk.clone(), outSubClocks.borrow()[(part1.clone()-1) as usize].clone());
        {let _arr = outSubClocks.clone(); _arr.borrow_mut()[(part1.clone()-1) as usize] = subClk1.clone(); _arr};
        {let _arr = partIsAssigned.clone(); _arr.borrow_mut()[(part1.clone()-1) as usize] = true; _arr};
    }
    while !(partLst.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(partLst.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        part1 = __pa0.clone();
        partLst = __pa1.clone();
        adjParts = partAdjacency.clone().borrow()[(part1.clone()-1) as usize].clone();
        for mut adjPart in &*adjParts.clone() {
            let mut adjPart = adjPart.clone();
            part2 = Util::tuple21(adjPart.clone());
            if !(partIsAssigned.clone().borrow()[(part2.clone()-1) as usize].clone()) {
                subClk1 = outSubClocks.clone().borrow()[(part1.clone()-1) as usize].clone();
                subClk2 = Util::tuple22(adjPart.clone());
                subClk2 = computeAbsoluteSubClock(subClk1.clone(), subClk2.clone())?;
                if !(isInferedSubClock(subClk2.clone())) {
                    {let _arr = outSubClocks.clone(); _arr.borrow_mut()[(part2.clone()-1) as usize] = subClk2.clone(); _arr};
                    {let _arr = partIsAssigned.clone(); _arr.borrow_mut()[(part2.clone()-1) as usize] = true; _arr};
                    partLst = cons(part2.clone(), partLst.clone());
                }
            }
        }
    }
    Ok((baseClkOut, outSubClocks))
}

fn computeAbsoluteSubClock(mut preClock: BackendDAE::SubClock, mut subSeqClock: BackendDAE::SubClock) -> Result<BackendDAE::SubClock> {
    let mut subClk: BackendDAE::SubClock = BackendDAE::DEFAULT_SUBCLOCK.clone();
    subClk = (match (preClock.clone(), subSeqClock.clone()) {
        (BackendDAE::SubClock::SUBCLOCK { factor: mut f1, shift: mut s1, solver: mut solver1 }, BackendDAE::SubClock::SUBCLOCK { factor: mut f2, shift: mut s2, solver: mut solver2 }) => {
            solver1 = mergeSolver(solver1.clone(), solver2.clone())?;
            BackendDAE::SubClock::SUBCLOCK { factor: MMath::divRational(f1.clone(), f2.clone())?, shift: MMath::addRational(MMath::multRational(s1.clone(), f2.clone())?, s2.clone())?, solver: solver1.clone() }
        },
        (BackendDAE::SubClock::SUBCLOCK { factor: _, shift: _, solver: _ }, BackendDAE::SubClock::INFERED_SUBCLOCK) => {
            subSeqClock.clone()
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SynchronousFeatures.computeAbsoluteSubClock")); __mm_s.push_str(&*literal!(" failed.\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
    });
    Ok(subClk)
}

fn mergeSolver(mut solver1: Option<ArcStr>, mut solver2: Option<ArcStr>) -> Result<Option<ArcStr>> {
    let mut sOut: Option<ArcStr> = None;
    sOut = (match (solver1.clone(), solver2.clone()) {
        (None, Some(mut s2)) => {
            Some((s2.clone()).clone())
        },
        (Some(mut s1), None) => {
            Some((s1.clone()).clone())
        },
        (Some(mut s1), Some(mut s2)) => {
            if !(stringEq((s1.clone()).clone(), (s2.clone()).clone())) {
                Error::addCompilerNotification(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Infered sub clock partitions have different solvers:")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!(" <->")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!(".\n")); ArcStr::from(__mm_s) }).clone())?;
            }
            Some((s1.clone()).clone())
        },
        _ => {
            None
        },
    });
    Ok(sOut)
}

fn addPartAdjacencyEdge(mut part1: i32, mut sub1: BackendDAE::SubClock, mut part2: i32, mut sub2: BackendDAE::SubClock, mut partAdjacency: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::SubClock)>>>) -> Result<()> {
    let mut partEdges: Arc<metamodelica::List<(i32, BackendDAE::SubClock)>> = metamodelica::nil();
    if intGt(part1.clone(), 0) && intGt(part2.clone(), 0) {
        partEdges = partAdjacency.clone().borrow()[(part1.clone()-1) as usize].clone();
        for mut edge in &*partEdges.clone() {
            let mut edge = edge.clone();
            if intEq(Util::tuple21(edge.clone()), part2.clone()) {
            }
        }
        {let _arr = partAdjacency.clone(); _arr.borrow_mut()[(part1.clone()-1) as usize] = cons((part2.clone(), sub1.clone()), partEdges.clone()); _arr};
        partEdges = partAdjacency.clone().borrow()[(part2.clone()-1) as usize].clone();
        {let _arr = partAdjacency.clone(); _arr.borrow_mut()[(part2.clone()-1) as usize] = cons((part1.clone(), sub2.clone()), partEdges.clone()); _arr};
    }
    Ok(())
}

fn setSubClockFactor(mut subClk: BackendDAE::SubClock, mut factor: MMath::Rational) -> BackendDAE::SubClock {
    let mut subClkOut: BackendDAE::SubClock = BackendDAE::SubClock::INFERED_SUBCLOCK;
    subClkOut = (match subClk.clone() {
        BackendDAE::SubClock::SUBCLOCK { factor: _, shift: mut shift, solver: mut solver } => {
            BackendDAE::SubClock::SUBCLOCK { factor: factor.clone(), shift: shift.clone(), solver: solver.clone() }
        },
        _ => {
            subClk.clone()
        },
    });
    subClkOut
}

fn getSubClockFactor(mut subClk: BackendDAE::SubClock) -> MMath::Rational {
    let mut factor: MMath::Rational;
    factor = (match subClk.clone() {
        BackendDAE::SubClock::SUBCLOCK { factor: mut factor, shift: _, solver: _ } => {
            factor.clone()
        },
        _ => {
            MMath::RAT1.clone()
        },
    });
    factor
}

fn getSubClockShift(mut subClk: BackendDAE::SubClock) -> MMath::Rational {
    let mut shift: MMath::Rational;
    shift = (match subClk.clone() {
        BackendDAE::SubClock::SUBCLOCK { factor: _, shift: mut shift, solver: _ } => {
            shift.clone()
        },
        _ => {
            MMath::RAT0.clone()
        },
    });
    shift
}

fn getSubClockSolverOpt(mut subClk: BackendDAE::SubClock) -> Option<ArcStr> {
    let mut solver: Option<ArcStr> = None;
    solver = (match subClk.clone() {
        BackendDAE::SubClock::SUBCLOCK { factor: _, shift: _, solver: mut solver } => {
            solver.clone()
        },
        _ => {
            None
        },
    });
    solver
}

fn setSubClockShift(mut subClk: BackendDAE::SubClock, mut shift: MMath::Rational) -> BackendDAE::SubClock {
    let mut subClkOut: BackendDAE::SubClock = BackendDAE::SubClock::INFERED_SUBCLOCK;
    subClkOut = (match subClk.clone() {
        BackendDAE::SubClock::SUBCLOCK { factor: mut factor, shift: _, solver: mut solver } => {
            BackendDAE::SubClock::SUBCLOCK { factor: factor.clone(), shift: shift.clone(), solver: solver.clone() }
        },
        _ => {
            subClk.clone()
        },
    });
    subClkOut
}

fn setSubClockSolver(mut subClk: BackendDAE::SubClock, mut solver: Option<ArcStr>) -> BackendDAE::SubClock {
    let mut subClkOut: BackendDAE::SubClock = BackendDAE::SubClock::INFERED_SUBCLOCK;
    subClkOut = (match subClk.clone() {
        BackendDAE::SubClock::SUBCLOCK { factor: mut factor, shift: mut shift, solver: _ } => {
            BackendDAE::SubClock::SUBCLOCK { factor: factor.clone(), shift: shift.clone(), solver: solver.clone() }
        },
        _ => {
            subClk.clone()
        },
    });
    subClkOut
}

fn getConnectedSubPartitions(mut eq: Arc<BackendDAE::Equation>, mut varPartMap: metamodelica::Array<i32>, mut vars: BackendDAE::Variables) -> Result<(bool, i32, i32, BackendDAE::SubClock, i32, i32, BackendDAE::SubClock)> {
    let mut infered: bool = false;
    let mut part1: i32 = 0;
    let mut var1: i32 = -1;
    let mut sub1: BackendDAE::SubClock = BackendDAE::SubClock::INFERED_SUBCLOCK;
    let mut part2: i32 = 0;
    let mut var2: i32 = -1;
    let mut sub2: BackendDAE::SubClock = BackendDAE::SubClock::INFERED_SUBCLOCK;
    sub1 = BackendDAE::DEFAULT_SUBCLOCK.clone();
    sub2 = BackendDAE::DEFAULT_SUBCLOCK.clone();
    (part1, var1, part2, var2) = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref2, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: factor }, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "superSample" }, .. }, exp: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, .. } => {
            let mut v1: i32 = 0;
            let mut v2: i32 = 0;
            let mut p1: i32 = 0;
            let mut p2: i32 = 0;
            infered = intEq(factor.clone(), 0);
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref1.clone(), vars.clone())?) {
                (_, Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            v1 = __pa0.clone();
            p1 = varPartMap.borrow()[(v1.clone()-1) as usize].clone();
            let __pa2 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref2.clone(), vars.clone())?) {
                (_, Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil }) => __pa2.clone(),
                _ => bail!("pattern mismatch"),
            } };
            v2 = __pa2.clone();
            p2 = varPartMap.borrow()[(v2.clone()-1) as usize].clone();
            if infered.clone() {
                sub1 = crate::BackendDAE::SubClock::INFERED_SUBCLOCK;
                sub2 = crate::BackendDAE::SubClock::INFERED_SUBCLOCK;
            } else {
                sub1 = setSubClockFactor(sub1.clone(), MMath::divRational(MMath::RAT1.clone(), MMath::Rational { nom: factor.clone(), denom: 1 })?);
                sub2 = setSubClockFactor(sub2.clone(), MMath::Rational { nom: factor.clone(), denom: 1 });
            }
            (p1.clone(), v1.clone(), p2.clone(), v2.clone())
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref2, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: factor }, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "subSample" }, .. }, exp: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, .. } => {
            let mut v1: i32 = 0;
            let mut v2: i32 = 0;
            let mut p1: i32 = 0;
            let mut p2: i32 = 0;
            infered = intEq(factor.clone(), 0);
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref1.clone(), vars.clone())?) {
                (_, Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            v1 = __pa0.clone();
            p1 = varPartMap.borrow()[(v1.clone()-1) as usize].clone();
            let __pa2 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref2.clone(), vars.clone())?) {
                (_, Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil }) => __pa2.clone(),
                _ => bail!("pattern mismatch"),
            } };
            v2 = __pa2.clone();
            p2 = varPartMap.borrow()[(v2.clone()-1) as usize].clone();
            if infered.clone() {
                sub1 = crate::BackendDAE::SubClock::INFERED_SUBCLOCK;
                sub2 = crate::BackendDAE::SubClock::INFERED_SUBCLOCK;
            } else {
                sub1 = setSubClockFactor(sub1.clone(), MMath::Rational { nom: factor.clone(), denom: 1 });
                sub2 = setSubClockFactor(sub2.clone(), MMath::divRational(MMath::RAT1.clone(), MMath::Rational { nom: factor.clone(), denom: 1 })?);
            }
            (p1.clone(), v1.clone(), p2.clone(), v2.clone())
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref2, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: counter }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: resolution }, tail: Deref @ metamodelica::List::Nil } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "shiftSample" }, .. }, exp: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, .. } => {
            let mut v1: i32 = 0;
            let mut v2: i32 = 0;
            let mut p1: i32 = 0;
            let mut p2: i32 = 0;
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref1.clone(), vars.clone())?) {
                (_, Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            v1 = __pa0.clone();
            p1 = varPartMap.borrow()[(v1.clone()-1) as usize].clone();
            let __pa2 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref2.clone(), vars.clone())?) {
                (_, Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil }) => __pa2.clone(),
                _ => bail!("pattern mismatch"),
            } };
            v2 = __pa2.clone();
            p2 = varPartMap.borrow()[(v2.clone()-1) as usize].clone();
            sub1 = setSubClockShift(sub1.clone(), MMath::subRational(MMath::RAT0.clone(), MMath::Rational { nom: counter.clone(), denom: resolution.clone() })?);
            sub2 = setSubClockShift(sub2.clone(), MMath::Rational { nom: counter.clone(), denom: resolution.clone() });
            (p1.clone(), v1.clone(), p2.clone(), v2.clone())
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref2, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: counter }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: resolution }, tail: Deref @ metamodelica::List::Nil } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "backSample" }, .. }, exp: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, .. } => {
            let mut v1: i32 = 0;
            let mut v2: i32 = 0;
            let mut p1: i32 = 0;
            let mut p2: i32 = 0;
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref1.clone(), vars.clone())?) {
                (_, Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            v1 = __pa0.clone();
            p1 = varPartMap.borrow()[(v1.clone()-1) as usize].clone();
            let __pa2 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref2.clone(), vars.clone())?) {
                (_, Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil }) => __pa2.clone(),
                _ => bail!("pattern mismatch"),
            } };
            v2 = __pa2.clone();
            p2 = varPartMap.borrow()[(v2.clone()-1) as usize].clone();
            sub1 = setSubClockShift(sub1.clone(), MMath::Rational { nom: counter.clone(), denom: resolution.clone() });
            sub2 = setSubClockShift(sub2.clone(), MMath::subRational(MMath::RAT0.clone(), MMath::Rational { nom: counter.clone(), denom: resolution.clone() })?);
            (p1.clone(), v1.clone(), p2.clone(), v2.clone())
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::SOLVER_CLOCK { solverMethod: Deref @ DAE::Exp::SCONST { string: solver }, c: Deref @ DAE::Exp::CREF { componentRef: cref2, .. } } }, exp: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, .. } => {
            let mut v1: i32 = 0;
            let mut v2: i32 = 0;
            let mut p1: i32 = 0;
            let mut p2: i32 = 0;
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref1.clone(), vars.clone())?) {
                (_, Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            v1 = __pa0.clone();
            p1 = varPartMap.borrow()[(v1.clone()-1) as usize].clone();
            let __pa2 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref2.clone(), vars.clone())?) {
                (_, Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil }) => __pa2.clone(),
                _ => bail!("pattern mismatch"),
            } };
            v2 = __pa2.clone();
            p2 = varPartMap.borrow()[(v2.clone()-1) as usize].clone();
            sub1 = setSubClockSolver(sub1.clone(), Some((solver.clone()).clone()));
            sub2 = setSubClockSolver(sub2.clone(), Some((solver.clone()).clone()));
            (p1.clone(), v1.clone(), p2.clone(), v2.clone())
        },
        _ => {
            (-1, -1, -1, -1)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((infered, part1, var1, sub1, part2, var2, sub2))
}

fn chooseBaseClock(mut clockEqs: Arc<metamodelica::List<i32>>, mut numPartitions: i32, mut eqPartMap: metamodelica::Array<i32>, mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> (Arc<DAE::ClockKind>, i32) {
    let mut outBaseClock: Arc<DAE::ClockKind> = Arc::new(openmodelica_frontend_types::DAE::ClockKind::INFERRED_CLOCK);
    let mut baseClockEqIdx: i32 = -1;
    let mut subClkPartMap: metamodelica::Array<BackendDAE::SubClock>;
    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    subClkPartMap = arrayCreate(numPartitions.clone(), BackendDAE::DEFAULT_SUBCLOCK.clone());
    for mut clockEq in &*clockEqs.clone() {
        let mut clockEq = clockEq.clone();
        eq = BackendEquation::get(eqs.clone(), clockEq.clone());
        if isBaseClockEq(eq.clone()) {
            outBaseClock = getBaseClock(eq.clone());
            baseClockEqIdx = clockEq.clone();
        }
    }
    (outBaseClock, baseClockEqIdx)
}

fn isBaseClockEq(mut eq: Arc<BackendDAE::Equation>) -> bool {
    let mut isBaseClock: bool = false;
    isBaseClock = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::INFERRED_CLOCK }, exp: Deref @ DAE::Exp::CREF { .. }, .. } => {
            false
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CLKCONST { .. }, exp: Deref @ DAE::Exp::CREF { .. }, .. } => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isBaseClock
}

fn getBaseClock(mut eq: Arc<BackendDAE::Equation>) -> Arc<DAE::ClockKind> {
    let mut baseClk: Arc<DAE::ClockKind> = Arc::new(DAE::ClockKind::INFERRED_CLOCK);
    baseClk = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::INFERRED_CLOCK }, exp: Deref @ DAE::Exp::CREF { .. }, .. } => {
            Arc::new(openmodelica_frontend_types::DAE::ClockKind::INFERRED_CLOCK)
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CLKCONST { clk }, exp: Deref @ DAE::Exp::CREF { .. }, .. } => {
            clk.clone()
        },
        _ => {
            Arc::new(openmodelica_frontend_types::DAE::ClockKind::INFERRED_CLOCK)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    baseClk
}

fn removeEdge(mut eq: i32, mut var: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    let mut row: Arc<metamodelica::List<i32>> = metamodelica::nil();
    row = m.clone().borrow()[(eq.clone()-1) as usize].clone();
    (row, _) = List::deleteMemberOnTrue(var.clone(), row.clone(), Arc::new(fnptr!(intEq, i32, i32)))?;
    {let _arr = m.clone(); _arr.borrow_mut()[(eq.clone()-1) as usize] = row.clone(); _arr};
    row = mT.clone().borrow()[(var.clone()-1) as usize].clone();
    (row, _) = List::deleteMemberOnTrue(eq.clone(), row.clone(), Arc::new(fnptr!(intEq, i32, i32)))?;
    {let _arr = mT.clone(); _arr.borrow_mut()[(var.clone()-1) as usize] = row.clone(); _arr};
    Ok(())
}

fn findBaseClockInterfaces(mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut vars: BackendDAE::Variables, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut clockEqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut subClockInterfaceEqIdxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut subClockInterfaceEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut eqIdx: i32 = 0;
    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    for mut eqIdx in 1..=BackendEquation::getNumberOfEquations(eqs.clone()) {
        eq = BackendEquation::get(eqs.clone(), eqIdx.clone());
        (clockEqs, subClockInterfaceEqIdxs, subClockInterfaceEqs) = findBaseClockInterfaces1(eq.clone(), eqIdx.clone(), eqs.clone(), vars.clone(), m.clone(), mT.clone(), clockEqs.clone(), subClockInterfaceEqIdxs.clone(), subClockInterfaceEqs.clone())?;
    }
    Ok((clockEqs, subClockInterfaceEqIdxs, subClockInterfaceEqs))
}

fn findBaseClockInterfaces1(mut eq: Arc<BackendDAE::Equation>, mut eqIdx: i32, mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut vars: BackendDAE::Variables, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut clockEqsIn: Arc<metamodelica::List<i32>>, mut subClockInterfaceEqIdxsIn: Arc<metamodelica::List<i32>>, mut subClockInterfaceEqsIn: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut clockEqsOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut subClockInterfaceEqIdxsOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut subClockInterfaceEqsOut: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    (clockEqsOut, subClockInterfaceEqIdxsOut, subClockInterfaceEqsOut) = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::INFERRED_CLOCK }, .. } => {
            (cons(eqIdx.clone(), clockEqsIn.clone()), subClockInterfaceEqIdxsIn.clone(), subClockInterfaceEqsIn.clone())
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::RATIONAL_CLOCK { intervalCounter: _, .. } }, .. } => {
            (cons(eqIdx.clone(), clockEqsIn.clone()), subClockInterfaceEqIdxsIn.clone(), subClockInterfaceEqsIn.clone())
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::REAL_CLOCK { interval: _ } }, .. } => {
            (cons(eqIdx.clone(), clockEqsIn.clone()), subClockInterfaceEqIdxsIn.clone(), subClockInterfaceEqsIn.clone())
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::EVENT_CLOCK { condition: _, .. } }, .. } => {
            (cons(eqIdx.clone(), clockEqsIn.clone()), subClockInterfaceEqIdxsIn.clone(), subClockInterfaceEqsIn.clone())
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::SOLVER_CLOCK { c: Deref @ DAE::Exp::CREF { componentRef: _, .. }, solverMethod: _ } }, .. } => {
            (clockEqsIn.clone(), cons(eqIdx.clone(), subClockInterfaceEqIdxsIn.clone()), cons(eq.clone(), subClockInterfaceEqsIn.clone()))
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::SOLVER_CLOCK { c: Deref @ DAE::Exp::CLKCONST { clk: _ }, solverMethod: _ } }, .. } => {
            (cons(eqIdx.clone(), clockEqsIn.clone()), subClockInterfaceEqIdxsIn.clone(), subClockInterfaceEqsIn.clone())
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "superSample" }, .. }, .. } => {
            let mut varIdx: i32 = 0;
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref1.clone(), vars.clone())?) {
                (_, Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            varIdx = __pa0.clone();
            removeEdge(eqIdx.clone(), varIdx.clone(), m.clone(), mT.clone())?;
            (clockEqsIn.clone(), cons(eqIdx.clone(), subClockInterfaceEqIdxsIn.clone()), cons(eq.clone(), subClockInterfaceEqsIn.clone()))
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "subSample" }, .. }, .. } => {
            let mut varIdx: i32 = 0;
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref1.clone(), vars.clone())?) {
                (_, Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            varIdx = __pa0.clone();
            removeEdge(eqIdx.clone(), varIdx.clone(), m.clone(), mT.clone())?;
            (clockEqsIn.clone(), cons(eqIdx.clone(), subClockInterfaceEqIdxsIn.clone()), cons(eq.clone(), subClockInterfaceEqsIn.clone()))
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "shiftSample" }, .. }, .. } => {
            let mut varIdx: i32 = 0;
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref1.clone(), vars.clone())?) {
                (_, Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            varIdx = __pa0.clone();
            removeEdge(eqIdx.clone(), varIdx.clone(), m.clone(), mT.clone())?;
            (clockEqsIn.clone(), cons(eqIdx.clone(), subClockInterfaceEqIdxsIn.clone()), cons(eq.clone(), subClockInterfaceEqsIn.clone()))
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "shiftSample" }, .. }, .. } => {
            let mut varIdx: i32 = 0;
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref1.clone(), vars.clone())?) {
                (_, Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            varIdx = __pa0.clone();
            removeEdge(eqIdx.clone(), varIdx.clone(), m.clone(), mT.clone())?;
            (clockEqsIn.clone(), cons(eqIdx.clone(), subClockInterfaceEqIdxsIn.clone()), cons(eq.clone(), subClockInterfaceEqsIn.clone()))
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "backSample" }, .. }, .. } => {
            let mut varIdx: i32 = 0;
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref1.clone(), vars.clone())?) {
                (_, Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            varIdx = __pa0.clone();
            removeEdge(eqIdx.clone(), varIdx.clone(), m.clone(), mT.clone())?;
            (clockEqsIn.clone(), cons(eqIdx.clone(), subClockInterfaceEqIdxsIn.clone()), cons(eq.clone(), subClockInterfaceEqsIn.clone()))
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "backSample" }, .. }, .. } => {
            let mut varIdx: i32 = 0;
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref1.clone(), vars.clone())?) {
                (_, Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            varIdx = __pa0.clone();
            removeEdge(eqIdx.clone(), varIdx.clone(), m.clone(), mT.clone())?;
            (clockEqsIn.clone(), cons(eqIdx.clone(), subClockInterfaceEqIdxsIn.clone()), cons(eq.clone(), subClockInterfaceEqsIn.clone()))
        },
        Deref @ BackendDAE::Equation::EQUATION { .. } => {
            (clockEqsIn.clone(), subClockInterfaceEqIdxsIn.clone(), subClockInterfaceEqsIn.clone())
        },
        _ => {
            (clockEqsIn.clone(), subClockInterfaceEqIdxsIn.clone(), subClockInterfaceEqsIn.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((clockEqsOut, subClockInterfaceEqIdxsOut, subClockInterfaceEqsOut))
}

fn findHighestWhenPrefixIdx(mut inVar: BackendDAE::Var, mut idxIn: i32) -> Result<(BackendDAE::Var, i32)> {
    let mut outVar: BackendDAE::Var = inVar.clone();
    let mut idxOut: i32 = idxIn.clone();
    let mut name: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut chars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut chars1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut chars2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    name = inVar.varName.clone();
    chars = stringListStringChar((ComponentReference::crefStr(name.clone())?).clone());
    if intGt((chars.clone().len() as i32), 9) {
        (chars1, chars2) = List::split(chars.clone(), 8)?;
        if stringEq(stringDelimitList(chars1.clone(), (literal!("")).clone()), (arcstr::literal!(BackendDAE::WHENCLK_PRREFIX)).clone()) {
            idxOut = intMax(idxIn.clone(), stringInt(stringDelimitList(chars2.clone(), (literal!("")).clone()))?);
        }
    }
    Ok((outVar, idxOut))
}

fn replaceSampledClocks(mut eqsIn: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut varsIn: BackendDAE::Variables) -> Result<(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, BackendDAE::Variables)> {
    let mut eqsOut: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut varsOut: BackendDAE::Variables;
    let mut prefIdx: i32 = 0;
    let mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut newEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut newVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    prefIdx = BackendVariable::traverseBackendDAEVars(varsIn.clone(), Arc::new(findHighestWhenPrefixIdx), 1)?;
    let (__pa0, (_, _, __pa1, __pa2)) = BackendEquation::traverseEquationArray_WithUpdate(eqsIn.clone(), Arc::new(replaceSampledClocks1), (varsIn.clone(), prefIdx.clone() + 1, metamodelica::nil(), metamodelica::nil()))?;
    eqs = __pa0.clone();
    newEqs = __pa1.clone();
    newVars = __pa2.clone();
    eqsOut = BackendEquation::addList(newEqs.clone(), eqs.clone())?;
    varsOut = BackendVariable::addVars(newVars.clone(), varsIn.clone());
    Ok((eqsOut, varsOut))
}

fn replaceSampledClocks1(mut eqIn: Arc<BackendDAE::Equation>, mut tplIn: (BackendDAE::Variables, i32, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)) -> Result<(Arc<BackendDAE::Equation>, (BackendDAE::Variables, i32, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>))> {
    let mut eqOut: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut tplOut: (BackendDAE::Variables, i32, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>);
    (eqOut, tplOut) = (::match_deref::match_deref! { match &((eqIn.clone(), tplIn.clone())) {
        (Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, source, attr: BackendDAE::EquationAttributes { kind: BackendDAE::EquationKind::DYNAMIC_EQUATION, .. } }, (vars, suffixIdx0, newEqs, newVars)) => {
            let mut suffixIdx: i32 = 0;
            let mut attr: BackendDAE::EquationAttributes;
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            let mut newEqs = (*newEqs).clone();
            let mut newVars = (*newVars).clone();
            let (__pa0, (__pa1, __pa2, __pa3)) = Expression::traverseExpTopDown(e1.clone(), Arc::new(replaceSampledClocks2), (newEqs.clone(), newVars.clone(), suffixIdx0.clone()))?;
            e1 = __pa0.clone();
            newEqs = __pa1.clone();
            newVars = __pa2.clone();
            suffixIdx = __pa3.clone();
            let (__pa4, (__pa5, __pa6, __pa7)) = Expression::traverseExpTopDown(e2.clone(), Arc::new(replaceSampledClocks2), (newEqs.clone(), newVars.clone(), suffixIdx.clone()))?;
            e2 = __pa4.clone();
            newEqs = __pa5.clone();
            newVars = __pa6.clone();
            suffixIdx = __pa7.clone();
            if intEq(suffixIdx.clone() - suffixIdx0.clone(), 1) {
                attr = BackendEquation::defaultClockedEqAttr(suffixIdx0.clone());
            } else {
                attr = BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone();
            }
            (Arc::new(BackendDAE::Equation::EQUATION { exp: e1.clone(), scalar: e2.clone(), source: source.clone(), attr: attr.clone() }), (vars.clone(), suffixIdx.clone(), newEqs.clone(), newVars.clone()))
        },
        (Deref @ BackendDAE::Equation::ARRAY_EQUATION { dimSize, left: e1, right: e2, source, attr: BackendDAE::EquationAttributes { kind: BackendDAE::EquationKind::DYNAMIC_EQUATION, .. }, recordSize }, (vars, suffixIdx0, newEqs, newVars)) => {
            let mut suffixIdx: i32 = 0;
            let mut attr: BackendDAE::EquationAttributes;
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            let mut newEqs = (*newEqs).clone();
            let mut newVars = (*newVars).clone();
            let (__pa0, (__pa1, __pa2, __pa3)) = Expression::traverseExpTopDown(e1.clone(), Arc::new(replaceSampledClocks2), (newEqs.clone(), newVars.clone(), suffixIdx0.clone()))?;
            e1 = __pa0.clone();
            newEqs = __pa1.clone();
            newVars = __pa2.clone();
            suffixIdx = __pa3.clone();
            let (__pa4, (__pa5, __pa6, __pa7)) = Expression::traverseExpTopDown(e2.clone(), Arc::new(replaceSampledClocks2), (newEqs.clone(), newVars.clone(), suffixIdx.clone()))?;
            e2 = __pa4.clone();
            newEqs = __pa5.clone();
            newVars = __pa6.clone();
            suffixIdx = __pa7.clone();
            if intEq(suffixIdx.clone() - suffixIdx0.clone(), 1) {
                attr = BackendEquation::defaultClockedEqAttr(suffixIdx0.clone());
            } else {
                attr = BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone();
            }
            (Arc::new(BackendDAE::Equation::ARRAY_EQUATION { dimSize: dimSize.clone(), left: e1.clone(), right: e2.clone(), source: source.clone(), attr: attr.clone(), recordSize: recordSize.clone() }), (vars.clone(), suffixIdx.clone(), newEqs.clone(), newVars.clone()))
        },
        _ => {
            (eqIn.clone(), tplIn.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((eqOut, tplOut))
}

fn replaceSampledClocks2(mut inExp: Arc<DAE::Exp>, mut tplIn: (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut tplOut: (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32);
    (outExp, cont, tplOut) = (::match_deref::match_deref! { match &((inExp.clone(), tplIn.clone())) {
        (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: varExp @ Deref @ DAE::Exp::CREF { componentRef: _, .. }, tail: Deref @ metamodelica::List::Cons { head: clk @ Deref @ DAE::Exp::CLKCONST { clk: _ }, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, .. }, (newEqs, newVars, suffixIdx)) => {
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut addEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut addVar: BackendDAE::Var;
            cr = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(BackendDAE::WHENCLK_PRREFIX)); __mm_s.push_str(&*intString(suffixIdx.clone())); ArcStr::from(__mm_s) }).clone(), identType: DAE::T_CLOCK_DEFAULT.clone(), subscriptLst: metamodelica::nil() });
            addVar = BackendVariable::makeVar(cr.clone());
            addEq = Arc::new(BackendDAE::Equation::EQUATION { exp: Expression::crefToExp(cr.clone())?, scalar: clk.clone(), source: DAE::emptyElementSource.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
            (substGetPartition(varExp.clone())?, false, (cons(addEq.clone(), newEqs.clone()), cons(addVar.clone(), newVars.clone()), suffixIdx.clone() + 1))
        },
        _ => {
            (inExp.clone(), true, tplIn.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, tplOut))
}

fn orderSubPartitions(mut numParts: i32, mut subclocks: metamodelica::Array<BackendDAE::SubClock>, mut order: metamodelica::Array<i32>, mut eqPartMap: metamodelica::Array<i32>, mut varPartMap: metamodelica::Array<i32>, mut remEqPartMap: metamodelica::Array<i32>, mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut vars: BackendDAE::Variables, mut remEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut shared: Arc<BackendDAE::Shared>, mut partitionOffset: i32) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, Arc<metamodelica::List<BackendDAE::SubClock>>)> {
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut subClksOut: Arc<metamodelica::List<BackendDAE::SubClock>> = metamodelica::nil();
    let mut contMerge: bool = false;
    let mut considerRemovedEqs: bool = false;
    let mut part: i32 = 0;
    let mut mergedParts: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut partVarMap: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut partEqMap: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut partRemEqMap: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut sys: Arc<BackendDAE::EqSystem>;
    let mut clk: BackendDAE::SubClock = BackendDAE::SubClock::INFERED_SUBCLOCK;
    let mut clk2: BackendDAE::SubClock = BackendDAE::SubClock::INFERED_SUBCLOCK;
    let mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut remEqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut mergedOrder: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    considerRemovedEqs = intGe((remEqPartMap.clone().borrow().len() as i32), 1);
    partVarMap = arrayCreate(numParts.clone(), metamodelica::nil());
    let __range0 = 1..=(varPartMap.clone().borrow().len() as i32);
    for mut varIdx in __range0 {
        part = varPartMap.clone().borrow()[(varIdx.clone()-1) as usize].clone();
        if part.clone() > 0 {
            {let _arr = partVarMap.clone(); let _val = listAppend(partVarMap.borrow()[(part.clone()-1) as usize].clone(), list![varIdx.clone()]); _arr.borrow_mut()[(part.clone()-1) as usize] = _val; _arr};
        }
    }
    partEqMap = arrayCreate(numParts.clone(), metamodelica::nil());
    let __range1 = 1..=(eqPartMap.clone().borrow().len() as i32);
    for mut eqIdx in __range1 {
        part = eqPartMap.clone().borrow()[(eqIdx.clone()-1) as usize].clone();
        if part.clone() > 0 {
            {let _arr = partEqMap.clone(); let _val = listAppend(partEqMap.borrow()[(part.clone()-1) as usize].clone(), list![eqIdx.clone()]); _arr.borrow_mut()[(part.clone()-1) as usize] = _val; _arr};
        }
    }
    partRemEqMap = arrayCreate(numParts.clone(), metamodelica::nil());
    if considerRemovedEqs.clone() {
        let __range2 = 1..=(partRemEqMap.clone().borrow().len() as i32);
        for mut reqIdx in __range2 {
            part = remEqPartMap.clone().borrow()[(reqIdx.clone()-1) as usize].clone();
            if part.clone() > 0 {
                {let _arr = partRemEqMap.clone(); let _val = listAppend(partRemEqMap.borrow()[(part.clone()-1) as usize].clone(), list![reqIdx.clone()]); _arr.borrow_mut()[(part.clone()-1) as usize] = _val; _arr};
            }
        }
    }
    mergedOrder = metamodelica::nil();
    mergedParts = metamodelica::nil();
    clk = subclocks.clone().borrow()[(order.borrow()[(1-1) as usize].clone()-1) as usize].clone();
    let __range3 = order.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut part in __range3 {
        clk2 = subclocks.clone().borrow()[(part.clone()-1) as usize].clone();
        if subClkEqual(clk.clone(), clk2.clone())? {
            mergedParts = cons(part.clone(), mergedParts.clone());
        } else {
            mergedOrder = cons(mergedParts.clone().reverse(), mergedOrder.clone());
            mergedParts = list![part.clone()];
            clk = subclocks.clone().borrow()[(part.clone()-1) as usize].clone();
        }
    }
    mergedOrder = cons(mergedParts.clone().reverse(), mergedOrder.clone());
    mergedOrder = mergedOrder.clone().reverse();
    part = 1;
    for mut mergedParts in &*mergedOrder.clone() {
        let mut mergedParts = mergedParts.clone();
        eqLst = metamodelica::nil();
        varLst = metamodelica::nil();
        remEqLst = metamodelica::nil();
        for mut partIdx in &*mergedParts.clone() {
            let mut partIdx = partIdx.clone();
            let __range4 = &*partEqMap.clone().borrow()[(partIdx.clone()-1) as usize].clone();
            for mut e in __range4 {
                let mut e = e.clone();
                eqLst = cons(BackendEquation::get(eqs.clone(), e.clone()), eqLst.clone());
            }
            let __range5 = &*partVarMap.clone().borrow()[(partIdx.clone()-1) as usize].clone();
            for mut v in __range5 {
                let mut v = v.clone();
                varLst = cons(BackendVariable::getVarAt(vars.clone(), v.clone())?, varLst.clone());
            }
            let __range6 = &*partRemEqMap.clone().borrow()[(partIdx.clone()-1) as usize].clone();
            for mut r in __range6 {
                let mut r = r.clone();
                remEqLst = cons(BackendEquation::get(remEqs.clone(), r.clone()), remEqLst.clone());
            }
            clk = subclocks.clone().borrow()[(partIdx.clone()-1) as usize].clone();
        }
        if !(eqLst.clone().is_empty()) || !(remEqLst.clone().is_empty()) {
            let (__pa7, (_, _)) = createEqSystem(eqLst.clone().reverse(), varLst.clone().reverse(), remEqLst.clone(), (true, true))?;
            sys = __pa7.clone();
            assign_field!(sys.partitionKind = BackendDAE::BaseClockPartitionKind::CLOCKED_PARTITION { subPartIdx: partitionOffset.clone() + part.clone() });
            subClksOut = cons(clk.clone(), subClksOut.clone());
            systs = cons(sys.clone(), systs.clone());
            part = part.clone() + 1;
        }
    }
    systs = systs.clone().reverse();
    subClksOut = subClksOut.clone().reverse();
    Ok((systs, subClksOut))
}

fn isInferedSubClock(mut subClk: BackendDAE::SubClock) -> bool {
    let mut isInfered: bool = false;
    isInfered = (match subClk.clone() {
        BackendDAE::SubClock::INFERED_SUBCLOCK => true,
        _ => false,
    });
    isInfered
}

fn isInferedBaseClock(mut subClk: Arc<DAE::ClockKind>) -> bool {
    let mut isInfered: bool = false;
    isInfered = (::match_deref::match_deref! { match &(subClk.clone()) {
        Deref @ DAE::ClockKind::INFERRED_CLOCK => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isInfered
}

fn setFactor(mut oldVal: MMath::Rational, mut newVal: MMath::Rational) -> Result<MMath::Rational> {
    let mut outVal: MMath::Rational;
    outVal = (match (oldVal.clone(), newVal.clone()) {
        (MMath::Rational { nom: 1, denom: 1 }, _) => newVal.clone(),
        (_, MMath::Rational { nom: 1, denom: 1 }) => oldVal.clone(),
        _ => {
            if !(MMath::equals(oldVal.clone(), newVal.clone())?) {
                Error::addMessage(Error::SUBCLOCK_CONFLICT.clone(), list![(literal!("factor")).clone(), (MMath::rationalString(oldVal.clone())?).clone(), (MMath::rationalString(newVal.clone())?).clone()])?;
                bail!("fail");
            }
            newVal.clone()
        },
    });
    Ok(outVal)
}

fn setShift(mut oldVal: MMath::Rational, mut newVal: MMath::Rational) -> Result<MMath::Rational> {
    let mut outVal: MMath::Rational;
    outVal = (match (oldVal.clone(), newVal.clone()) {
        (MMath::Rational { nom: 0, denom: _ }, _) => newVal.clone(),
        (_, MMath::Rational { nom: 0, denom: _ }) => oldVal.clone(),
        _ => {
            if !(MMath::equals(oldVal.clone(), newVal.clone())?) {
                Error::addMessage(Error::SUBCLOCK_CONFLICT.clone(), list![(literal!("shift")).clone(), (MMath::rationalString(oldVal.clone())?).clone(), (MMath::rationalString(newVal.clone())?).clone()])?;
                bail!("fail");
            }
            newVal.clone()
        },
    });
    Ok(outVal)
}

fn collectSubclkInfoExp(mut inExp: Arc<DAE::Exp>, mut inTpl: (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, metamodelica::Array<Option<bool>>, SourceInfo, metamodelica::Array<i32>, i32, metamodelica::Array<i32>, BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, metamodelica::Array<Option<bool>>, SourceInfo, metamodelica::Array<i32>, i32, metamodelica::Array<i32>, BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, metamodelica::Array<Option<bool>>, SourceInfo, metamodelica::Array<i32>, i32, metamodelica::Array<i32>, BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>);
    let mut newEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut newVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut contPartitions: metamodelica::Array<Option<bool>>;
    let mut partitionIdx: i32 = 0;
    let mut partitions: metamodelica::Array<i32>;
    let mut vars: BackendDAE::Variables;
    let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut path: Arc<Absyn::Path>;
    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut attr: Arc<DAE::CallAttributes>;
    let mut clksCnt: metamodelica::Array<i32>;
    let mut clkCnt: i32 = 0;
    let mut source: SourceInfo;
    (newEqs, newVars, contPartitions, source, clksCnt, partitionIdx, partitions, vars, mT) = inTpl.clone();
    clkCnt = clksCnt.clone().borrow()[(partitionIdx.clone()-1) as usize].clone();
    (outExp, newEqs, newVars, clkCnt) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path, expLst, attr } => collectSubclkInfoCall(path.clone(), expLst.clone(), attr.clone(), newEqs.clone(), newVars.clone(), contPartitions.clone(), partitionIdx.clone(), clkCnt.clone(), partitions.clone(), vars.clone(), mT.clone(), source.clone())?,
        _ => (inExp.clone(), newEqs.clone(), newVars.clone(), clkCnt.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    {let _arr = clksCnt.clone(); _arr.borrow_mut()[(partitionIdx.clone()-1) as usize] = clkCnt.clone(); _arr};
    outTpl = (newEqs.clone(), newVars.clone(), contPartitions.clone(), source.clone(), clksCnt.clone(), partitionIdx.clone(), partitions.clone(), vars.clone(), mT.clone());
    Ok((outExp, outTpl))
}

fn createSubClockVar(mut inPartitionIdx: i32, mut inClkCnt: i32, mut inPath: Arc<Absyn::Path>, mut inExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inAttr: Arc<DAE::CallAttributes>, mut inPartitions: metamodelica::Array<i32>, mut inVars: BackendDAE::Variables, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<(BackendDAE::Var, Arc<BackendDAE::Equation>)> {
    let mut outVar: BackendDAE::Var;
    let mut outEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut varIxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut i: i32 = 0;
    let mut e: Arc<DAE::Exp>;
    let mut subclk: Arc<DAE::Exp>;
    let __pa0 = ::match_deref::match_deref! { match &(listHead(inExpLst.clone())?) {
        Deref @ DAE::Exp::CREF { componentRef: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    cr = __pa0.clone();
    (_, varIxs) = BackendVariable::getVar(cr.clone(), inVars.clone())?;
    i = listHead(varIxs.clone())?;
    i = listHead(mT.clone().borrow()[(i.clone()-1) as usize].clone())?;
    i = inPartitions.clone().borrow()[(i.clone()-1) as usize].clone();
    subclk = Arc::new(DAE::Exp::CREF { componentRef: getSubClkName(i.clone(), 1, DAE::T_CLOCK_DEFAULT.clone()), ty: DAE::T_CLOCK_DEFAULT.clone() });
    e = Arc::new(DAE::Exp::CALL { path: inPath.clone(), expLst: cons(subclk.clone(), listRest(inExpLst.clone())?), attr: inAttr.clone() });
    (outVar, outEq) = createSubClock(inPartitionIdx.clone(), inClkCnt.clone(), e.clone())?;
    Ok((outVar, outEq))
}

fn setContClockedPartition(mut inIsContClockedPartition: bool, mut inPartitionIdx: i32, mut inContPartitions: metamodelica::Array<Option<bool>>, mut source: SourceInfo) -> Result<()> {
    let mut isContClockedPartition: Option<bool> = None;
    let mut isContClockedPrevPartition: bool = false;
    isContClockedPartition = inContPartitions.clone().borrow()[(inPartitionIdx.clone()-1) as usize].clone();
    isContClockedPartition = (match isContClockedPartition.clone() {
        None => Some(inIsContClockedPartition.clone()),
        Some(mut isContClockedPrevPartition) => Some(inIsContClockedPartition.clone() || isContClockedPrevPartition.clone()),
    });
    {let _arr = inContPartitions.clone(); _arr.borrow_mut()[(inPartitionIdx.clone()-1) as usize] = isContClockedPartition.clone(); _arr};
    Ok(())
}

fn collectSubclkInfoCall(mut inPath: Arc<Absyn::Path>, mut inExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inAttr: Arc<DAE::CallAttributes>, mut inNewEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inNewVars: Arc<metamodelica::List<BackendDAE::Var>>, mut inContPartitions: metamodelica::Array<Option<bool>>, mut inPartitionIdx: i32, mut inClkCnt: i32, mut inPartitions: metamodelica::Array<i32>, mut inVars: BackendDAE::Variables, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut source: SourceInfo) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outNewEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut outNewVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut outClkCnt: i32 = 0;
    (outExp, outNewEqs, outNewVars, outClkCnt) = (::match_deref::match_deref! { match &((inPath.clone(), (inExpLst.clone().len() as i32))) {
        (Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, _) => {
            setContClockedPartition(true, inPartitionIdx.clone(), inContPartitions.clone(), source.clone())?;
            (Arc::new(DAE::Exp::CALL { path: inPath.clone(), expLst: inExpLst.clone(), attr: inAttr.clone() }), inNewEqs.clone(), inNewVars.clone(), inClkCnt.clone())
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "delay" }, _) => {
            setContClockedPartition(true, inPartitionIdx.clone(), inContPartitions.clone(), source.clone())?;
            (Arc::new(DAE::Exp::CALL { path: inPath.clone(), expLst: inExpLst.clone(), attr: inAttr.clone() }), inNewEqs.clone(), inNewVars.clone(), inClkCnt.clone())
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "spatialDistribution" }, _) => {
            setContClockedPartition(true, inPartitionIdx.clone(), inContPartitions.clone(), source.clone())?;
            (Arc::new(DAE::Exp::CALL { path: inPath.clone(), expLst: inExpLst.clone(), attr: inAttr.clone() }), inNewEqs.clone(), inNewVars.clone(), inClkCnt.clone())
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "initial" }, _) => {
            setContClockedPartition(true, inPartitionIdx.clone(), inContPartitions.clone(), source.clone())?;
            (Arc::new(DAE::Exp::CALL { path: inPath.clone(), expLst: inExpLst.clone(), attr: inAttr.clone() }), inNewEqs.clone(), inNewVars.clone(), inClkCnt.clone())
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "terminal" }, _) => {
            setContClockedPartition(true, inPartitionIdx.clone(), inContPartitions.clone(), source.clone())?;
            (Arc::new(DAE::Exp::CALL { path: inPath.clone(), expLst: inExpLst.clone(), attr: inAttr.clone() }), inNewEqs.clone(), inNewVars.clone(), inClkCnt.clone())
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "smooth" }, _) => {
            setContClockedPartition(true, inPartitionIdx.clone(), inContPartitions.clone(), source.clone())?;
            (Arc::new(DAE::Exp::CALL { path: inPath.clone(), expLst: inExpLst.clone(), attr: inAttr.clone() }), inNewEqs.clone(), inNewVars.clone(), inClkCnt.clone())
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, 3) => {
            setContClockedPartition(true, inPartitionIdx.clone(), inContPartitions.clone(), source.clone())?;
            (Arc::new(DAE::Exp::CALL { path: inPath.clone(), expLst: inExpLst.clone(), attr: inAttr.clone() }), inNewEqs.clone(), inNewVars.clone(), inClkCnt.clone())
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, _) => {
            setContClockedPartition(true, inPartitionIdx.clone(), inContPartitions.clone(), source.clone())?;
            (Arc::new(DAE::Exp::CALL { path: inPath.clone(), expLst: inExpLst.clone(), attr: inAttr.clone() }), inNewEqs.clone(), inNewVars.clone(), inClkCnt.clone())
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "edge" }, _) => {
            setContClockedPartition(true, inPartitionIdx.clone(), inContPartitions.clone(), source.clone())?;
            (Arc::new(DAE::Exp::CALL { path: inPath.clone(), expLst: inExpLst.clone(), attr: inAttr.clone() }), inNewEqs.clone(), inNewVars.clone(), inClkCnt.clone())
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "change" }, _) => {
            setContClockedPartition(true, inPartitionIdx.clone(), inContPartitions.clone(), source.clone())?;
            (Arc::new(DAE::Exp::CALL { path: inPath.clone(), expLst: inExpLst.clone(), attr: inAttr.clone() }), inNewEqs.clone(), inNewVars.clone(), inClkCnt.clone())
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "reinit" }, _) => {
            setContClockedPartition(true, inPartitionIdx.clone(), inContPartitions.clone(), source.clone())?;
            (Arc::new(DAE::Exp::CALL { path: inPath.clone(), expLst: inExpLst.clone(), attr: inAttr.clone() }), inNewEqs.clone(), inNewVars.clone(), inClkCnt.clone())
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, _) => {
            setContClockedPartition(false, inPartitionIdx.clone(), inContPartitions.clone(), source.clone())?;
            (Arc::new(DAE::Exp::CALL { path: inPath.clone(), expLst: inExpLst.clone(), attr: inAttr.clone() }), inNewEqs.clone(), inNewVars.clone(), inClkCnt.clone())
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "firstTick" }, _) => {
            setContClockedPartition(false, inPartitionIdx.clone(), inContPartitions.clone(), source.clone())?;
            (Arc::new(DAE::Exp::CALL { path: inPath.clone(), expLst: metamodelica::nil(), attr: inAttr.clone() }), inNewEqs.clone(), inNewVars.clone(), inClkCnt.clone())
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "interval" }, _) => {
            setContClockedPartition(false, inPartitionIdx.clone(), inContPartitions.clone(), source.clone())?;
            (Arc::new(DAE::Exp::CALL { path: inPath.clone(), expLst: metamodelica::nil(), attr: inAttr.clone() }), inNewEqs.clone(), inNewVars.clone(), inClkCnt.clone())
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, 2) => {
            let mut var: BackendDAE::Var;
            let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            (var, eq) = createSubClock(inPartitionIdx.clone(), inClkCnt.clone(), (inExpLst.clone()).get(2)?)?;
            (substGetPartition((inExpLst.clone()).get(1)?)?, cons(eq.clone(), inNewEqs.clone()), cons(var.clone(), inNewVars.clone()), inClkCnt.clone() + 1)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "subSample" }, 2) => {
            (substGetPartition((inExpLst.clone()).get(1)?)?, inNewEqs.clone(), inNewVars.clone(), inClkCnt.clone() + 1)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "superSample" }, 2) => {
            (substGetPartition((inExpLst.clone()).get(1)?)?, inNewEqs.clone(), inNewVars.clone(), inClkCnt.clone() + 1)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "shiftSample" }, 3) => {
            (substGetPartition((inExpLst.clone()).get(1)?)?, inNewEqs.clone(), inNewVars.clone(), inClkCnt.clone() + 1)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "backSample" }, 3) => {
            (substGetPartition((inExpLst.clone()).get(1)?)?, inNewEqs.clone(), inNewVars.clone(), inClkCnt.clone() + 1)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "noClock" }, 1) => {
            (substGetPartition((inExpLst.clone()).get(1)?)?, inNewEqs.clone(), inNewVars.clone(), inClkCnt.clone())
        },
        _ => {
            (Arc::new(DAE::Exp::CALL { path: inPath.clone(), expLst: inExpLst.clone(), attr: inAttr.clone() }), inNewEqs.clone(), inNewVars.clone(), inClkCnt.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outNewEqs, outNewVars, outClkCnt))
}

fn createSubClockVarFactor(mut inPartitionIdx: i32, mut inClkCnt: i32, mut inPath: Arc<Absyn::Path>, mut inExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inAttr: Arc<DAE::CallAttributes>, mut inPartitions: metamodelica::Array<i32>, mut inVars: BackendDAE::Variables, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inNewEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inNewVars: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outNewEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = inNewEqs.clone();
    let mut outNewVars: Arc<metamodelica::List<BackendDAE::Var>> = inNewVars.clone();
    let mut outClkCnt: i32 = inClkCnt.clone();
    let mut e: Arc<DAE::Exp>;
    outExp = substGetPartition(listHead(inExpLst.clone())?)?;
    Ok((outExp, outNewEqs, outNewVars, outClkCnt))
}

fn substGetPartition(mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let mut attrs: Arc<DAE::CallAttributes>;
    attrs = Arc::new(DAE::CallAttributes { ty: Expression::r#typeof(inExp.clone())?, tuple_: false, builtin: true, isImpure: true, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL });
    outExp = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("$getPart")).clone() }), expLst: list![inExp.clone()], attr: attrs.clone() });
    Ok(outExp)
}

fn getSubClkName(mut inPartitionIdx: i32, mut inClkIdx: i32, mut inTy: Arc<DAE::Type>) -> Arc<DAE::ComponentRef> {
    let mut outRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut name: ArcStr = arcstr::literal!("");
    name = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$subclk")); __mm_s.push_str(&*intString(inPartitionIdx.clone())); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(inClkIdx.clone())); ArcStr::from(__mm_s) }).clone();
    outRef = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (name.clone()).clone(), identType: inTy.clone(), subscriptLst: metamodelica::nil() });
    outRef
}

fn createSubClock(mut inPartitionIdx: i32, mut inCnt: i32, mut inExp: Arc<DAE::Exp>) -> Result<(BackendDAE::Var, Arc<BackendDAE::Equation>)> {
    let mut outVar: BackendDAE::Var;
    let mut outEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    ty = DAE::T_CLOCK_DEFAULT.clone();
    cr = getSubClkName(inPartitionIdx.clone(), inCnt.clone(), ty.clone());
    (outVar, outEq) = createEqVarPair(cr.clone(), ty.clone(), inExp.clone())?;
    Ok((outVar, outEq))
}

fn collectSubclkInfo(mut inEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inRemovedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inPartitionCnt: i32, mut inPartitions: metamodelica::Array<i32>, mut inReqsPartitions: metamodelica::Array<i32>, mut inVars: BackendDAE::Variables, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, metamodelica::Array<Option<bool>>, metamodelica::Array<i32>)> {
    let mut outNewEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut outNewVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut outContPartitions: metamodelica::Array<Option<bool>>;
    let mut oClksCnt: metamodelica::Array<i32>;
    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut cnt: i32 = 0;
    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut var: BackendDAE::Var;
    let mut partitionsWhenClocks: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    outContPartitions = arrayCreate(inPartitionCnt.clone(), None);
    partitionsWhenClocks = arrayCreate(inPartitionCnt.clone(), metamodelica::nil());
    oClksCnt = arrayCreate(inPartitionCnt.clone(), 1);
    (outNewEqs, outNewVars) = collectEquationArrayClocks(inEqs.clone(), inPartitionCnt.clone(), inPartitions.clone(), partitionsWhenClocks.clone(), oClksCnt.clone(), outContPartitions.clone(), inVars.clone(), mT.clone(), metamodelica::nil(), metamodelica::nil())?;
    (outNewEqs, outNewVars) = collectEquationArrayClocks(inRemovedEqs.clone(), inPartitionCnt.clone(), inReqsPartitions.clone(), partitionsWhenClocks.clone(), oClksCnt.clone(), outContPartitions.clone(), inVars.clone(), mT.clone(), outNewEqs.clone(), outNewVars.clone())?;
    for mut i in 1..=inPartitionCnt.clone() {
        let __range0 = &*partitionsWhenClocks.clone().borrow()[(i.clone()-1) as usize].clone();
        for mut j in __range0 {
            let mut j = j.clone();
            cnt = oClksCnt.clone().borrow()[(i.clone()-1) as usize].clone();
            cr = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(BackendDAE::WHENCLK_PRREFIX)); __mm_s.push_str(&*intString(j.clone())); ArcStr::from(__mm_s) }).clone(), identType: DAE::T_CLOCK_DEFAULT.clone(), subscriptLst: metamodelica::nil() });
            (var, eq) = createSubClock(i.clone(), cnt.clone(), Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: DAE::T_CLOCK_DEFAULT.clone() }))?;
            outNewEqs = cons(eq.clone(), outNewEqs.clone());
            outNewVars = cons(var.clone(), outNewVars.clone());
            {let _arr = oClksCnt.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = cnt.clone() + 1; _arr};
        }
        if oClksCnt.clone().borrow()[(i.clone()-1) as usize].clone() == 1 {
            (var, eq) = createSubClock(i.clone(), 1, Arc::new(DAE::Exp::CLKCONST { clk: Arc::new(openmodelica_frontend_types::DAE::ClockKind::INFERRED_CLOCK) }))?;
            outNewEqs = cons(eq.clone(), outNewEqs.clone());
            outNewVars = cons(var.clone(), outNewVars.clone());
            {let _arr = oClksCnt.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = 2; _arr};
        }
    }
    Ok((outNewEqs, outNewVars, outContPartitions, oClksCnt))
}

fn collectEquationArrayClocks(mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut partitionsCnt: i32, mut partitions: metamodelica::Array<i32>, mut partitionsWhenClocks: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut clksCnt: metamodelica::Array<i32>, mut contPartitions: metamodelica::Array<Option<bool>>, mut inVars: BackendDAE::Variables, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inNewEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inNewVars: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)> {
    let mut outNewEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = inNewEqs.clone();
    let mut outNewVars: Arc<metamodelica::List<BackendDAE::Var>> = inNewVars.clone();
    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut eqAttr: BackendDAE::EquationAttributes;
    let mut partitionIdx: i32 = 0;
    let mut source: SourceInfo;
    for mut i in 1..=BackendEquation::getNumberOfEquations(eqs.clone()) {
        eq = BackendEquation::get(eqs.clone(), i.clone());
        partitionIdx = partitions.clone().borrow()[(i.clone()-1) as usize].clone();
        let __pa0 = ::match_deref::match_deref! { match &(BackendEquation::equationSource(eq.clone())?) {
            Deref @ DAE::ElementSource { info: __pa0, .. } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        source = __pa0.clone();
        if partitionIdx.clone() != 0 {
            eqAttr = BackendEquation::getEquationAttributes(eq.clone())?;
            eqAttr = (match eqAttr.clone() {
        BackendDAE::EquationAttributes { kind: BackendDAE::EquationKind::CLOCKED_EQUATION { clk: mut whenIdx }, .. } => {
            let mut partitionsWhenClocksLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            partitionsWhenClocksLst = partitionsWhenClocks.borrow()[(partitionIdx.clone()-1) as usize].clone();
            if whenIdx.clone() != 0 && List::notMember(whenIdx.clone(), partitionsWhenClocksLst.clone()) {
                {let _arr = partitionsWhenClocks.clone(); _arr.borrow_mut()[(partitionIdx.clone()-1) as usize] = cons(whenIdx.clone(), partitionsWhenClocksLst.clone()); _arr};
            }
            eqAttr.kind = crate::BackendDAE::EquationKind::DYNAMIC_EQUATION;
            eqAttr.clone()
        },
        _ => {
            eqAttr.clone()
        },
    });
            eq = BackendEquation::setEquationAttributes(eq.clone(), eqAttr.clone())?;
            let (__pa1, (__pa2, __pa3, _, _, _, _, _, _, _)) = BackendEquation::traverseExpsOfEquation(eq.clone(), Arc::new(collectSubclkInfoExp1), (outNewEqs.clone(), outNewVars.clone(), contPartitions.clone(), source.clone(), clksCnt.clone(), partitionIdx.clone(), partitions.clone(), inVars.clone(), mT.clone()))?;
            eq = __pa1.clone();
            outNewEqs = __pa2.clone();
            outNewVars = __pa3.clone();
            BackendEquation::setAtIndex(eqs.clone(), i.clone(), eq.clone())?;
        }
    }
    Ok((outNewEqs, outNewVars))
}

fn collectSubclkInfoExp1(mut inExp: Arc<DAE::Exp>, mut inTpl: (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, metamodelica::Array<Option<bool>>, SourceInfo, metamodelica::Array<i32>, i32, metamodelica::Array<i32>, BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, metamodelica::Array<Option<bool>>, SourceInfo, metamodelica::Array<i32>, i32, metamodelica::Array<i32>, BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, metamodelica::Array<Option<bool>>, SourceInfo, metamodelica::Array<i32>, i32, metamodelica::Array<i32>, BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>);
    (outExp, outTpl) = Expression::traverseExpBottomUp(inExp.clone(), Arc::new(collectSubclkInfoExp), inTpl.clone())?;
    Ok((outExp, outTpl))
}

fn splitClockEqs(mut inEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, metamodelica::Array<bool>)> {
    let mut outClockEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut outClockEqsMask: metamodelica::Array<bool>;
    let mut clockEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut i: i32 = 0;
    outClockEqsMask = arrayCreate(BackendEquation::getNumberOfEquations(inEqs.clone()), true);
    for mut i in 1..=BackendEquation::getNumberOfEquations(inEqs.clone()) {
        eq = BackendEquation::get(inEqs.clone(), i.clone());
        if isClockEquation(eq.clone())? {
            clockEqs = cons(eq.clone(), clockEqs.clone());
            {let _arr = outClockEqsMask.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = false; _arr};
        }
    }
    outClockEqs = BackendEquation::listEquation(clockEqs.clone())?;
    Ok((outClockEqs, outClockEqsMask))
}

fn splitClockVars(mut inVars: BackendDAE::Variables) -> Result<(BackendDAE::Variables, metamodelica::Array<bool>)> {
    let mut outClockVars: BackendDAE::Variables;
    let mut outClockVarsMask: metamodelica::Array<bool>;
    let mut clockVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut var: BackendDAE::Var;
    outClockVarsMask = arrayCreate(BackendVariable::varsSize(inVars.clone())?, true);
    for mut i in 1..=BackendVariable::varsSize(inVars.clone())? {
        var = BackendVariable::getVarAt(inVars.clone(), i.clone())?;
        if Types::isClockOrSubTypeClock(var.varType.clone())? {
            clockVars = cons(var.clone(), clockVars.clone());
            {let _arr = outClockVarsMask.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = false; _arr};
        }
    }
    outClockVars = BackendVariable::listVar(clockVars.clone());
    Ok((outClockVars, outClockVarsMask))
}

fn substitutePartitionOpExps(mut inSyst: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut outSyst: Arc<BackendDAE::EqSystem> = inSyst.clone();
    let mut newEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut newVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut cnt: i32 = 1;
    for mut eq in &*BackendEquation::equationList(inSyst.orderedEqs.clone()) {
        let mut eq = eq.clone();
        let (__pa0, (__pa1, __pa2, __pa3, _)) = BackendEquation::traverseExpsOfEquation(eq.clone(), Arc::new(substitutePartitionOpExp), (newEqs.clone(), newVars.clone(), cnt.clone(), inShared.clone()))?;
        eq = __pa0.clone();
        newEqs = __pa1.clone();
        newVars = __pa2.clone();
        cnt = __pa3.clone();
        newEqs = cons(eq.clone(), newEqs.clone());
    }
    assign_field!(
        outSyst.orderedEqs = BackendEquation::listEquation(newEqs.clone().reverse())?,
        outSyst.orderedVars = BackendVariable::addVars(newVars.clone(), inSyst.orderedVars.clone())
    );
    outSyst = BackendDAEUtil::clearEqSyst(outSyst.clone())?;
    Ok(outSyst)
}

fn substitutePartitionOpExp(mut inExp: Arc<DAE::Exp>, mut inTpl: (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32, Arc<BackendDAE::Shared>)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32, Arc<BackendDAE::Shared>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32, Arc<BackendDAE::Shared>);
    (outExp, outTpl) = Expression::traverseExpBottomUp(inExp.clone(), Arc::new(substitutePartitionOpExp1), inTpl.clone())?;
    Ok((outExp, outTpl))
}

fn substitutePartitionOpExp1(mut inExp: Arc<DAE::Exp>, mut inTpl: (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32, Arc<BackendDAE::Shared>)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32, Arc<BackendDAE::Shared>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32, Arc<BackendDAE::Shared>);
    let mut path: Arc<Absyn::Path>;
    let mut shared: Arc<BackendDAE::Shared>;
    let mut attr: Arc<DAE::CallAttributes>;
    let mut clk: Arc<DAE::ClockKind> = Arc::new(DAE::ClockKind::INFERRED_CLOCK);
    let mut cnt: i32 = 0;
    let mut newEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut newVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    (newEqs, newVars, cnt, shared) = inTpl.clone();
    (outExp, outTpl) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CLKCONST { clk } => {
            let mut clk = (*clk).clone();
            (clk, newEqs, newVars, cnt) = substClock(clk.clone(), newEqs.clone(), newVars.clone(), cnt.clone(), shared.clone())?;
            (Arc::new(DAE::Exp::CLKCONST { clk: clk.clone() }), (newEqs.clone(), newVars.clone(), cnt.clone(), shared.clone()))
        },
        Deref @ DAE::Exp::CALL { attr, expLst: exps, path } => substituteExpsCall(path.clone(), exps.clone(), attr.clone(), newEqs.clone(), newVars.clone(), cnt.clone(), shared.clone())?,
        _ => (inExp.clone(), inTpl.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outTpl))
}

fn substClock(mut inClk: Arc<DAE::ClockKind>, mut inNewEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inNewVars: Arc<metamodelica::List<BackendDAE::Var>>, mut inCnt: i32, mut inShared: Arc<BackendDAE::Shared>) -> Result<(Arc<DAE::ClockKind>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32)> {
    let mut outClk: Arc<DAE::ClockKind> = Arc::new(DAE::ClockKind::INFERRED_CLOCK);
    let mut outNewEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut outNewVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut outCnt: i32 = 0;
    let mut e: Arc<DAE::Exp>;
    let mut i: Arc<DAE::Exp>;
    let mut f: Arc<DAE::Exp>;
    let mut cnt: i32 = 0;
    let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    (outClk, outNewEqs, outNewVars, outCnt) = (::match_deref::match_deref! { match &(inClk.clone()) {
        Deref @ DAE::ClockKind::EVENT_CLOCK { condition: e, startInterval: f } => {
            let mut e = (*e).clone();
            let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(substExp(list![e.clone()], inNewEqs.clone(), inNewVars.clone(), inCnt.clone())?) {
                (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, __pa1, __pa2, __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e = __pa0.clone();
            eqs = __pa1.clone();
            vars = __pa2.clone();
            cnt = __pa3.clone();
            (Arc::new(DAE::ClockKind::EVENT_CLOCK { condition: e.clone(), startInterval: f.clone() }), eqs.clone(), vars.clone(), cnt.clone())
        },
        Deref @ DAE::ClockKind::REAL_CLOCK { interval: e } => {
            let mut e = (*e).clone();
            (e, eqs, vars, cnt) = substClockExp(e.clone(), inNewEqs.clone(), inNewVars.clone(), inCnt.clone(), inShared.clone())?;
            (Arc::new(DAE::ClockKind::REAL_CLOCK { interval: e.clone() }), eqs.clone(), vars.clone(), cnt.clone())
        },
        Deref @ DAE::ClockKind::RATIONAL_CLOCK { intervalCounter: e, resolution: i } => {
            let mut e = (*e).clone();
            (e, eqs, vars, cnt) = substClockExp(e.clone(), inNewEqs.clone(), inNewVars.clone(), inCnt.clone(), inShared.clone())?;
            (Arc::new(DAE::ClockKind::RATIONAL_CLOCK { intervalCounter: e.clone(), resolution: i.clone() }), eqs.clone(), vars.clone(), cnt.clone())
        },
        _ => (inClk.clone(), inNewEqs.clone(), inNewVars.clone(), inCnt.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outClk, outNewEqs, outNewVars, outCnt))
}

fn isKnownOrConstantExp(mut inExp: Arc<DAE::Exp>, mut inKnownVars: BackendDAE::Variables) -> Result<bool> {
    let mut outKnown: bool = false;
    let (_, (__pa0, _)) = Expression::traverseExpTopDown(inExp.clone(), Arc::new(fnptr!(isKnownOrConstantExp_traverser, Arc<DAE::Exp>, (bool, BackendDAE::Variables))), (true, inKnownVars.clone()))?;
    outKnown = __pa0.clone();
    Ok(outKnown)
}

fn isKnownOrConstantExp_traverser(mut inExp: Arc<DAE::Exp>, mut inTpl: (bool, BackendDAE::Variables)) -> (Arc<DAE::Exp>, bool, (bool, BackendDAE::Variables)) {
    let mut outExp: Arc<DAE::Exp> = inExp.clone();
    let mut outContinue: bool = false;
    let mut outTpl: (bool, BackendDAE::Variables);
    let mut globalKnownVars: BackendDAE::Variables;
    let mut isKnown: bool = false;
    (isKnown, globalKnownVars) = inTpl.clone();
    isKnown = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { .. } => {
            false
        },
        Deref @ DAE::Exp::CREF { componentRef, .. } => {
            BackendVariable::containsCref(componentRef.clone(), globalKnownVars.clone())
        },
        _ => {
            isKnown.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outTpl = (isKnown.clone(), globalKnownVars.clone());
    outContinue = isKnown.clone();
    (outExp, outContinue, outTpl)
}

fn substClockExp(mut inExp: Arc<DAE::Exp>, mut inNewEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inNewVars: Arc<metamodelica::List<BackendDAE::Var>>, mut inCnt: i32, mut inShared: Arc<BackendDAE::Shared>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outNewEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut outNewVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut outCnt: i32 = 0;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    if isKnownOrConstantExp(inExp.clone(), inShared.globalKnownVars.clone())? {
        outExp = inExp.clone();
        outNewEqs = inNewEqs.clone();
        outNewVars = inNewVars.clone();
        outCnt = inCnt.clone();
    } else {
        let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(substExp(list![inExp.clone()], inNewEqs.clone(), inNewVars.clone(), inCnt.clone())?) {
            (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, __pa1, __pa2, __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        outExp = __pa0.clone();
        outNewEqs = __pa1.clone();
        outNewVars = __pa2.clone();
        outCnt = __pa3.clone();
    }
    Ok((outExp, outNewEqs, outNewVars, outCnt))
}

fn substituteExpsCall(mut inPath: Arc<Absyn::Path>, mut inExps: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inAttr: Arc<DAE::CallAttributes>, mut inEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inVars: Arc<metamodelica::List<BackendDAE::Var>>, mut inCnt: i32, mut inShared: Arc<BackendDAE::Shared>) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32, Arc<BackendDAE::Shared>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32, Arc<BackendDAE::Shared>);
    let mut replace: bool = false;
    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut cnt: i32 = 0;
    replace = (::match_deref::match_deref! { match &((inPath.clone(), (inExps.clone().len() as i32))) {
        (Deref @ Absyn::Path::IDENT { name: Deref @ "hold" }, 1) => true,
        (Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, 2) => true,
        (Deref @ Absyn::Path::IDENT { name: Deref @ "subSample" }, 2) => true,
        (Deref @ Absyn::Path::IDENT { name: Deref @ "superSample" }, 2) => true,
        (Deref @ Absyn::Path::IDENT { name: Deref @ "shiftSample" }, 3) => true,
        (Deref @ Absyn::Path::IDENT { name: Deref @ "backSample" }, 3) => true,
        (Deref @ Absyn::Path::IDENT { name: Deref @ "noClock" }, 1) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (exps, eqs, vars, cnt) = if (replace.clone()) {substExp(inExps.clone(), inEqs.clone(), inVars.clone(), inCnt.clone())?} else {(inExps.clone(), inEqs.clone(), inVars.clone(), inCnt.clone())};
    outExp = Arc::new(DAE::Exp::CALL { path: inPath.clone(), expLst: exps.clone(), attr: inAttr.clone() });
    outTpl = (eqs.clone(), vars.clone(), cnt.clone(), inShared.clone());
    Ok((outExp, outTpl))
}

fn createVar(mut inComp: Arc<DAE::ComponentRef>, mut inType: Arc<DAE::Type>) -> Result<BackendDAE::Var> {
    let mut outVar: BackendDAE::Var;
    outVar = BackendDAE::Var { encrypted: false, initNonlinear: false, unreplaceable: false, innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), comment: None, hideResult: None, tearingSelectOption: Some(crate::BackendDAE::TearingSelect::DEFAULT), values: DAEUtil::setProtectedAttr(DAEUtil::getEmptyVarAttr(inType.clone()), true)?, source: DAE::emptyElementSource.clone(), arryDim: metamodelica::nil(), tplExp: None, bindExp: None, varType: inType.clone(), varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varKind: crate::BackendDAE::VarKind::VARIABLE, varName: inComp.clone() };
    Ok(outVar)
}

fn createEqVarPair(mut inComp: Arc<DAE::ComponentRef>, mut inType: Arc<DAE::Type>, mut inExp: Arc<DAE::Exp>) -> Result<(BackendDAE::Var, Arc<BackendDAE::Equation>)> {
    let mut outVar: BackendDAE::Var;
    let mut outEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    outVar = createVar(inComp.clone(), inType.clone())?;
    outEq = Arc::new(BackendDAE::Equation::EQUATION { attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(), source: DAE::emptyElementSource.clone(), scalar: inExp.clone(), exp: Arc::new(DAE::Exp::CREF { ty: inType.clone(), componentRef: inComp.clone() }) });
    Ok((outVar, outEq))
}

fn substExp(mut inExps: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inVars: Arc<metamodelica::List<BackendDAE::Var>>, mut inCnt: i32) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32)> {
    let mut outTpl: (Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32);
    let mut create: bool = false;
    let mut e: Arc<DAE::Exp>;
    e = listHead(inExps.clone())?;
    create = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Exp::CREF { .. } => false,
        Deref @ DAE::Exp::RCONST { .. } => false,
        Deref @ DAE::Exp::SCONST { .. } => false,
        Deref @ DAE::Exp::BCONST { .. } => false,
        Deref @ DAE::Exp::ENUM_LITERAL { .. } => false,
        Deref @ DAE::Exp::CLKCONST { .. } => true,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outTpl = (match create.clone() {
        true => {
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut var: BackendDAE::Var;
            ty = Expression::r#typeof(e.clone())?;
            cr = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$var")); __mm_s.push_str(&*intString(inCnt.clone())); ArcStr::from(__mm_s) }).clone(), identType: ty.clone(), subscriptLst: metamodelica::nil() });
            (var, eq) = createEqVarPair(cr.clone(), ty.clone(), e.clone())?;
            (cons(Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() }), listRest(inExps.clone())?), cons(eq.clone(), inEqs.clone()), cons(var.clone(), inVars.clone()), inCnt.clone() + 1)
        },
        false => {
            (inExps.clone(), inEqs.clone(), inVars.clone(), inCnt.clone())
        },
    });
    Ok(outTpl)
}

fn getVarIxs(mut inComp: Arc<DAE::ComponentRef>, mut inVariables: BackendDAE::Variables) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outIntegerLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    outIntegerLst = 'mc: {
        let __mc_input = inComp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut ixs: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    (_, ixs) = BackendVariable::getVar(inComp.clone(), inVariables.clone())?;
                    Ok(ixs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outIntegerLst)
}

fn isClockExp(mut inExp: Arc<DAE::Exp>) -> Result<bool> {
    let mut out: bool = false;
    out = Types::isClockOrSubTypeClock(Expression::r#typeof(inExp.clone())?)?;
    Ok(out)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn isClockEquation(mut inEq: Arc<BackendDAE::Equation>) -> Result<bool> {
    let mut out: bool = false;
    out = (::match_deref::match_deref! { match &(inEq.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { scalar: e, .. } => {
            isClockExp(e.clone())?
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { right: e, .. } => {
            isClockExp(e.clone())?
        },
        Deref @ BackendDAE::Equation::FOR_EQUATION { body: eq, .. } => {
            isClockEquation(eq.clone())?
        },
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { exp: e, .. } => {
            isClockExp(e.clone())?
        },
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. } => {
            isClockExp(e.clone())?
        },
        Deref @ BackendDAE::Equation::ALGORITHM { .. } => {
            false
        },
        Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: Deref @ BackendDAE::WhenEquation { whenStmtLst: Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { right: e, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, .. } => {
            let mut info: SourceInfo;
            if isClockExp(e.clone())? {
                let __pa0 = ::match_deref::match_deref! { match &(BackendEquation::equationSource(inEq.clone())?) {
                    Deref @ DAE::ElementSource { info: __pa0, .. } => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                info = __pa0.clone();
                Error::addSourceMessageAndFail(Error::INVALID_CLOCK_EQUATION.clone(), metamodelica::nil(), info.clone())?;
            }
            false
        },
        Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: Deref @ BackendDAE::WhenEquation { whenStmtLst: Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::REINIT { value: e, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, .. } => {
            let mut info: SourceInfo;
            if isClockExp(e.clone())? {
                let __pa0 = ::match_deref::match_deref! { match &(BackendEquation::equationSource(inEq.clone())?) {
                    Deref @ DAE::ElementSource { info: __pa0, .. } => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                info = __pa0.clone();
                Error::addSourceMessageAndFail(Error::INVALID_CLOCK_EQUATION.clone(), metamodelica::nil(), info.clone())?;
            }
            false
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: e, .. } => {
            isClockExp(e.clone())?
        },
        Deref @ BackendDAE::Equation::IF_EQUATION { eqnsfalse: falseEqs, eqnstrue: trueEqs, .. } => {
            let mut listEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut info: SourceInfo;
            for mut listEqs in &*trueEqs.clone() {
                let mut listEqs = listEqs.clone();
                for mut eq in &*listEqs.clone() {
                    let mut eq = eq.clone();
                    if isClockEquation(eq.clone())? {
                        let __pa0 = ::match_deref::match_deref! { match &(BackendEquation::equationSource(eq.clone())?) {
                            Deref @ DAE::ElementSource { info: __pa0, .. } => __pa0.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        info = __pa0.clone();
                        Error::addSourceMessageAndFail(Error::INVALID_CLOCK_EQUATION.clone(), metamodelica::nil(), info.clone())?;
                    }
                }
            }
            for mut eq in &*falseEqs.clone() {
                let mut eq = eq.clone();
                if isClockEquation(eq.clone())? {
                    let __pa1 = ::match_deref::match_deref! { match &(BackendEquation::equationSource(eq.clone())?) {
                        Deref @ DAE::ElementSource { info: __pa1, .. } => __pa1.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    info = __pa1.clone();
                    Error::addSourceMessageAndFail(Error::INVALID_CLOCK_EQUATION.clone(), metamodelica::nil(), info.clone())?;
                }
            }
            false
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SynchronousFeatures.isClockEquation")); __mm_s.push_str(&*literal!(" failed.\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out)
}

fn detectEqPartition(mut inEq: Arc<BackendDAE::Equation>) -> Result<(Option<bool>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, bool)>>)> {
    let mut outPartitionType: Option<bool> = None;
    let mut refsInfo: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, bool)>> = metamodelica::nil();
    let mut partitionType: Option<bool> = None;
    let mut isClockEq: bool = false;
    let mut info: SourceInfo;
    partitionType = (match BackendEquation::getEquationAttributes(inEq.clone())? {
        BackendDAE::EquationAttributes { kind: BackendDAE::EquationKind::CLOCKED_EQUATION { .. }, .. } => Some(true),
        _ => None,
    });
    info = BackendEquation::equationInfo(inEq.clone())?;
    let (_, (__pa0, __pa1, _)) = BackendEquation::traverseExpsOfEquation(inEq.clone(), Arc::new(detectEqPartitionExp), (partitionType.clone(), metamodelica::nil(), info.clone()))?;
    partitionType = __pa0.clone();
    refsInfo = __pa1.clone();
    isClockEq = isClockEquation(inEq.clone())?;
    outPartitionType = if (isClockEq.clone()) {setClockedPartition(Some(true), partitionType.clone(), None, info.clone())?} else {partitionType.clone()};
    Ok((outPartitionType, refsInfo))
}

fn printPartitionType(mut isClockedPartition: Option<bool>) -> ArcStr {
    let mut out: ArcStr = arcstr::literal!("");
    out = ((match isClockedPartition.clone() {
        Some(false) => literal!("CONT_PARTITION"),
        Some(true) => literal!("CLOCKED_PARTITION"),
        _ => literal!("UNSPECIFIED_PARTITION"),
    })).clone();
    out
}

fn detectEqPartitionExp(mut inExp: Arc<DAE::Exp>, mut inTpl: (Option<bool>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, bool)>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, (Option<bool>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, bool)>>, SourceInfo))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (Option<bool>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, bool)>>, SourceInfo);
    (outExp, outTpl) = Expression::traverseExpTopDown(inExp.clone(), Arc::new(detectEqPartitionExp1), inTpl.clone())?;
    Ok((outExp, outTpl))
}

fn detectEqPartitionExp1(mut inExp: Arc<DAE::Exp>, mut inTpl: (Option<bool>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, bool)>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, bool, (Option<bool>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, bool)>>, SourceInfo))> {
    let mut outExp: Arc<DAE::Exp> = inExp.clone();
    let mut cont: bool = false;
    let mut outTpl: (Option<bool>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, bool)>>, SourceInfo);
    let mut refs: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, bool)>> = metamodelica::nil();
    let mut partition: Option<bool> = None;
    let mut info: SourceInfo;
    (partition, refs, info) = inTpl.clone();
    (partition, refs, cont) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::EVENT_CLOCK { condition: e, startInterval: _ } } => {
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let __pa0 = ::match_deref::match_deref! { match &(e.clone()) {
                Deref @ DAE::Exp::CREF { componentRef: __pa0, ty: _ } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cr = __pa0.clone();
            (partition.clone(), cons((cr.clone(), false), refs.clone()), false)
        },
        Deref @ DAE::Exp::CALL { expLst: exps, path, .. } => {
            detectEqPartitionCall(path.clone(), exps.clone(), refs.clone(), partition.clone(), info.clone())?
        },
        _ => {
            (partition.clone(), refs.clone(), true)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outTpl = (partition.clone(), refs.clone(), info.clone());
    Ok((outExp, cont, outTpl))
}

fn detectEqPartitionCall(mut inPath: Arc<Absyn::Path>, mut inExps: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inRefs: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, bool)>>, mut inPartition: Option<bool>, mut info: SourceInfo) -> Result<(Option<bool>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, bool)>>, bool)> {
    let mut outPartition: Option<bool> = None;
    let mut outRefs: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, bool)>> = metamodelica::nil();
    let mut cont: bool = false;
    (outPartition, outRefs, cont) = (::match_deref::match_deref! { match &((inPath.clone(), inExps.clone())) {
        (Deref @ Absyn::Path::IDENT { name: Deref @ "hold" }, Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }) => {
            detectEqPartitionCall1(false, true, inPartition.clone(), e.clone(), inRefs.clone(), info.clone())?
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }) => {
            detectEqPartitionCall1(true, false, inPartition.clone(), e.clone(), inRefs.clone(), info.clone())?
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "subSample" }, Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }) => {
            detectEqPartitionCall1(true, true, inPartition.clone(), e.clone(), inRefs.clone(), info.clone())?
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "superSample" }, Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }) => {
            detectEqPartitionCall1(true, true, inPartition.clone(), e.clone(), inRefs.clone(), info.clone())?
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "shiftSample" }, Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }) => {
            detectEqPartitionCall1(true, true, inPartition.clone(), e.clone(), inRefs.clone(), info.clone())?
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "backSample" }, Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }) => {
            detectEqPartitionCall1(true, true, inPartition.clone(), e.clone(), inRefs.clone(), info.clone())?
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "noClock" }, Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }) => {
            detectEqPartitionCall1(true, true, inPartition.clone(), e.clone(), inRefs.clone(), info.clone())?
        },
        _ => {
            (inPartition.clone(), inRefs.clone(), true)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outPartition, outRefs, cont))
}

fn detectEqPartitionCall1(mut expClocked: bool, mut refClocked: bool, mut inPartition: Option<bool>, mut inExp: Arc<DAE::Exp>, mut inRefs: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, bool)>>, mut info: SourceInfo) -> Result<(Option<bool>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, bool)>>, bool)> {
    let mut outPartition: Option<bool> = None;
    let mut outRefs: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, bool)>> = metamodelica::nil();
    let mut cont: bool = false;
    (outPartition, outRefs) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ } => {
            (setClockedPartition(Some(expClocked.clone()), inPartition.clone(), None, info.clone())?, cons((cr.clone(), refClocked.clone()), inRefs.clone()))
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SynchronousFeatures.detectEqPartitionCall1")); __mm_s.push_str(&*literal!(" failed.\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outPartition, outRefs, cont))
}

fn setSystPartition(mut inSyst: Arc<BackendDAE::EqSystem>, mut inPartitionKind: BackendDAE::BaseClockPartitionKind) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut outSyst: Arc<BackendDAE::EqSystem>;
    outSyst = (::match_deref::match_deref! { match &(inSyst.clone()) {
        syst @ Deref @ BackendDAE::EqSystem { .. } => {
            let mut syst = (*syst).clone();
            assign_field!(syst.partitionKind = inPartitionKind.clone());
            syst.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSyst)
}

fn getPartitionConflictError(mut inComp: Option<Arc<DAE::ComponentRef>>) -> Result<(ErrorTypes::Message, Arc<metamodelica::List<ArcStr>>)> {
    let mut msg: ErrorTypes::Message;
    let mut tokens: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (msg, tokens) = (::match_deref::match_deref! { match &(inComp.clone()) {
        Some(cr) => {
            (Error::CONT_CLOCKED_PARTITION_CONFLICT_VAR.clone(), list![(ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone()])
        },
        _ => {
            (Error::CONT_CLOCKED_PARTITION_CONFLICT_EQ.clone(), metamodelica::nil())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((msg, tokens))
}

fn setClockedPartition(mut inNewPartitionType: Option<bool>, mut inOldPartitionType: Option<bool>, mut inComp: Option<Arc<DAE::ComponentRef>>, mut info: SourceInfo) -> Result<Option<bool>> {
    let mut outPartitionType: Option<bool> = None;
    outPartitionType = (match (inOldPartitionType.clone(), inNewPartitionType.clone()) {
        (None, _) => {
            inNewPartitionType.clone()
        },
        (_, None) => {
            inOldPartitionType.clone()
        },
        (Some(mut oldVal), Some(mut newVal)) if (oldVal.clone() == newVal.clone()) => {
            inNewPartitionType.clone()
        },
        _ => {
            let mut msg: ErrorTypes::Message;
            let mut tokens: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            (msg, tokens) = getPartitionConflictError(inComp.clone())?;
            Error::addSourceMessage(msg.clone(), tokens.clone(), info.clone())?;
            bail!("fail")
        },
    });
    Ok(outPartitionType)
}

pub fn partitionIndependentBlocks0(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rm: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rmT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut eqPartMap: metamodelica::Array<i32>, mut varPartMap: metamodelica::Array<i32>, mut rixs: metamodelica::Array<i32>, mut vars: metamodelica::Array<bool>, mut rvars: metamodelica::Array<bool>) -> Result<i32> {
    let mut on: i32 = 0;
    let __range0 = (1..=(m.clone().borrow().len() as i32)).rev();
    for mut i in __range0 {
        on = if (partitionIndependentBlocksWork(i.clone(), false, on.clone() + 1, m.clone(), mT.clone(), rm.clone(), rmT.clone(), eqPartMap.clone(), varPartMap.clone(), rixs.clone(), vars.clone(), rvars.clone())?) {on.clone() + 1} else {on.clone()};
    }
    let __range1 = (1..=(rm.clone().borrow().len() as i32)).rev();
    for mut i in __range1 {
        on = if (partitionIndependentBlocksWork(i.clone(), true, on.clone() + 1, m.clone(), mT.clone(), rm.clone(), rmT.clone(), eqPartMap.clone(), varPartMap.clone(), rixs.clone(), vars.clone(), rvars.clone())?) {on.clone() + 1} else {on.clone()};
    }
    Ok(on)
}

fn partitionIndependentBlocks(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut eqPartMap: metamodelica::Array<i32>, mut varPartMap: metamodelica::Array<i32>) -> Result<i32> {
    let mut on: i32 = 0;
    let __range0 = (1..=(m.clone().borrow().len() as i32)).rev();
    for mut eq in __range0 {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("check eq ")); __mm_s.push_str(&*intString(eq.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        if !(intEq(eqPartMap.clone().borrow()[(eq.clone()-1) as usize].clone(), -2)) {
            on = if (partitionIndependentBlocks2(eq.clone(), on.clone() + 1, m.clone(), mT.clone(), eqPartMap.clone(), varPartMap.clone())?) {on.clone() + 1} else {on.clone()};
        }
    }
    Ok(on)
}

fn partitionIndependentBlocks2(mut eqIdx: i32, mut partIdx: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut eqPartMap: metamodelica::Array<i32>, mut varPartMap: metamodelica::Array<i32>) -> Result<bool> {
    let mut ochange: bool = false;
    ochange = eqPartMap.clone().borrow()[(eqIdx.clone()-1) as usize].clone() == -1;
    if ochange.clone() {
        {let _arr = eqPartMap.clone(); _arr.borrow_mut()[(eqIdx.clone()-1) as usize] = partIdx.clone(); _arr};
        let __range0 = &*m.clone().borrow()[(eqIdx.clone()-1) as usize].clone();
        for mut var in __range0 {
            let mut var = var.clone();
            if !(intGt(varPartMap.clone().borrow()[(intAbs(var.clone())-1) as usize].clone(), 0)) {
                {let _arr = varPartMap.clone(); _arr.borrow_mut()[(intAbs(var.clone())-1) as usize] = partIdx.clone(); _arr};
                let __range1 = &*mT.clone().borrow()[(intAbs(var.clone())-1) as usize].clone();
                for mut newEq in __range1 {
                    let mut newEq = newEq.clone();
                    partitionIndependentBlocks2(intAbs(newEq.clone()), partIdx.clone(), m.clone(), mT.clone(), eqPartMap.clone(), varPartMap.clone())?;
                }
            }
        }
    }
    Ok(ochange)
}

fn partitionIndependentBlocksMasked(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rm: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rmT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mask: metamodelica::Array<bool>, mut eqPartMap: metamodelica::Array<i32>, mut varPartMap: metamodelica::Array<i32>, mut remEqPartMap: metamodelica::Array<i32>, mut vars: metamodelica::Array<bool>, mut rvars: metamodelica::Array<bool>) -> Result<i32> {
    let mut on: i32 = 0;
    on = 0;
    let __range0 = (1..=(m.clone().borrow().len() as i32)).rev();
    for mut i in __range0 {
        if mask.borrow()[(i.clone()-1) as usize].clone() {
            if partitionIndependentBlocksWork(i.clone(), false, on.clone() + 1, m.clone(), mT.clone(), rm.clone(), rmT.clone(), eqPartMap.clone(), varPartMap.clone(), remEqPartMap.clone(), vars.clone(), rvars.clone())? {
                on = on.clone() + 1;
            }
        }
    }
    let __range1 = (1..=(rm.clone().borrow().len() as i32)).rev();
    for mut i in __range1 {
        if partitionIndependentBlocksWork(i.clone(), true, on.clone() + 1, m.clone(), mT.clone(), rm.clone(), rmT.clone(), eqPartMap.clone(), varPartMap.clone(), remEqPartMap.clone(), vars.clone(), rvars.clone())? {
            on = on.clone() + 1;
        }
    }
    Ok(on)
}

fn partitionIndependentBlocksWork(mut idx: i32, mut isRemovedIdx: bool, mut partIdx: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rm: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rmT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut eqPartMap: metamodelica::Array<i32>, mut varPartMap: metamodelica::Array<i32>, mut rixs: metamodelica::Array<i32>, mut vars: metamodelica::Array<bool>, mut rvars: metamodelica::Array<bool>) -> Result<bool> {
    let mut ochange: bool = false;
    let mut eqIdx: i32 = 0;
    let mut rmIdx: i32 = 0;
    let mut workListEq: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut workListRm: Arc<metamodelica::List<i32>> = metamodelica::nil();
    ochange = false;
    if isRemovedIdx.clone() {
        if rixs.clone().borrow()[(idx.clone()-1) as usize].clone() == 0 {
            {let _arr = rixs.clone(); _arr.borrow_mut()[(idx.clone()-1) as usize] = partIdx.clone(); _arr};
            workListRm = list![idx.clone()];
            ochange = true;
        }
    } else {
        if eqPartMap.clone().borrow()[(idx.clone()-1) as usize].clone() == 0 {
            {let _arr = eqPartMap.clone(); _arr.borrow_mut()[(idx.clone()-1) as usize] = partIdx.clone(); _arr};
            workListEq = list![idx.clone()];
            ochange = true;
        }
    }
    if !(ochange.clone()) {
        return Ok(ochange);
    }
    while !(workListEq.clone().is_empty() && workListRm.clone().is_empty()) {
        if !(workListEq.clone().is_empty()) {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(workListEq.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            eqIdx = __pa0.clone();
            workListEq = __pa1.clone();
            let __range2 = &*m.clone().borrow()[(eqIdx.clone()-1) as usize].clone();
            for mut varIdx in __range2 {
                let mut varIdx = varIdx.clone();
                if !(vars.clone().borrow()[(intAbs(varIdx.clone())-1) as usize].clone()) {
                    {let _arr = vars.clone(); _arr.borrow_mut()[(intAbs(varIdx.clone())-1) as usize] = true; _arr};
                    {let _arr = varPartMap.clone(); _arr.borrow_mut()[(intAbs(varIdx.clone())-1) as usize] = partIdx.clone(); _arr};
                    let __range3 = &*mT.clone().borrow()[(intAbs(varIdx.clone())-1) as usize].clone();
                    for mut nextEqIdx in __range3 {
                        let mut nextEqIdx = nextEqIdx.clone();
                        if eqPartMap.clone().borrow()[(intAbs(nextEqIdx.clone())-1) as usize].clone() == 0 {
                            workListEq = cons(intAbs(nextEqIdx.clone()), workListEq.clone());
                            {let _arr = eqPartMap.clone(); _arr.borrow_mut()[(intAbs(nextEqIdx.clone())-1) as usize] = partIdx.clone(); _arr};
                        }
                    }
                    let __range4 = &*rmT.clone().borrow()[(intAbs(varIdx.clone())-1) as usize].clone();
                    for mut nextEqIdx in __range4 {
                        let mut nextEqIdx = nextEqIdx.clone();
                        if rixs.clone().borrow()[(intAbs(nextEqIdx.clone())-1) as usize].clone() == 0 {
                            workListRm = cons(intAbs(nextEqIdx.clone()), workListRm.clone());
                            {let _arr = rixs.clone(); _arr.borrow_mut()[(intAbs(nextEqIdx.clone())-1) as usize] = partIdx.clone(); _arr};
                        }
                    }
                }
            }
        } else {
            let (__pa5, __pa6) = ::match_deref::match_deref! { match &(workListRm.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa5, tail: __pa6 } => (__pa5.clone(), __pa6.clone()),
                _ => bail!("pattern mismatch"),
            } };
            rmIdx = __pa5.clone();
            workListRm = __pa6.clone();
            let __range7 = &*rm.clone().borrow()[(rmIdx.clone()-1) as usize].clone();
            for mut varIdx in __range7 {
                let mut varIdx = varIdx.clone();
                if !(rvars.clone().borrow()[(intAbs(varIdx.clone())-1) as usize].clone()) {
                    {let _arr = rvars.clone(); _arr.borrow_mut()[(intAbs(varIdx.clone())-1) as usize] = true; _arr};
                    let __range8 = &*mT.clone().borrow()[(intAbs(varIdx.clone())-1) as usize].clone();
                    for mut nextEqIdx in __range8 {
                        let mut nextEqIdx = nextEqIdx.clone();
                        if eqPartMap.clone().borrow()[(intAbs(nextEqIdx.clone())-1) as usize].clone() == 0 {
                            workListEq = cons(intAbs(nextEqIdx.clone()), workListEq.clone());
                            {let _arr = eqPartMap.clone(); _arr.borrow_mut()[(intAbs(nextEqIdx.clone())-1) as usize] = partIdx.clone(); _arr};
                        }
                    }
                    let __range9 = &*rmT.clone().borrow()[(intAbs(varIdx.clone())-1) as usize].clone();
                    for mut nextEqIdx in __range9 {
                        let mut nextEqIdx = nextEqIdx.clone();
                        if rixs.clone().borrow()[(intAbs(nextEqIdx.clone())-1) as usize].clone() == 0 {
                            workListRm = cons(intAbs(nextEqIdx.clone()), workListRm.clone());
                            {let _arr = rixs.clone(); _arr.borrow_mut()[(intAbs(nextEqIdx.clone())-1) as usize] = partIdx.clone(); _arr};
                        }
                    }
                }
            }
        }
    }
    Ok(ochange)
}

fn setVarPartition(mut varsPartition: metamodelica::Array<i32>, mut i: i32, mut eqsIxs: Arc<metamodelica::List<i32>>, mut eqsPartitions: metamodelica::Array<i32>) -> Result<()> {
    let mut partitionIdx: i32 = 0;
    for mut eq in &*eqsIxs.clone() {
        let mut eq = eq.clone();
        partitionIdx = eqsPartitions.borrow()[(eq.clone()-1) as usize].clone();
        if partitionIdx.clone() != 0 {
            assert!(varsPartition.borrow()[(i.clone()-1) as usize].clone() == 0 || varsPartition.borrow()[(i.clone()-1) as usize].clone() == partitionIdx.clone(), "{}", &*(literal!("SynchronousFeatures.setVarPartition failed")).clone());
            {let _arr = varsPartition.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = partitionIdx.clone(); _arr};
        }
    }
    Ok(())
}

fn createEqSystem(mut el: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut vl: Arc<metamodelica::List<BackendDAE::Var>>, mut rel: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut iTpl: (bool, bool)) -> Result<(Arc<BackendDAE::EqSystem>, (bool, bool))> {
    let mut syst: Arc<BackendDAE::EqSystem>;
    let mut oTpl: (bool, bool);
    let mut arr: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut remArr: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut vars: BackendDAE::Variables;
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    let mut s1: ArcStr = arcstr::literal!("");
    let mut s2: ArcStr = arcstr::literal!("");
    let mut s3: ArcStr = arcstr::literal!("");
    let mut s4: ArcStr = arcstr::literal!("");
    let mut crs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut success: bool = false;
    let mut throwNoError: bool = false;
    (success, throwNoError) = iTpl.clone();
    vars = BackendVariable::listVar1(vl.clone());
    arr = BackendEquation::listEquation(el.clone())?;
    remArr = BackendEquation::listEquation(rel.clone())?;
    i1 = BackendEquation::equationArraySize(arr.clone())?;
    i2 = BackendVariable::varsSize(vars.clone())?;
    if i1.clone() != i2.clone() && !(throwNoError.clone()) {
        s1 = (intString(i1.clone())).clone();
        s2 = (intString(i2.clone())).clone();
        crs = List::mapMap(vl.clone(), Arc::new(BackendVariable::varCref), Arc::new(ComponentReferenceBasics::printComponentRefStr));
        s3 = stringDelimitList(crs.clone(), (literal!("\n")).clone());
        s4 = (BackendDump::dumpEqnsStr(el.clone())?).clone();
        Error::addSourceMessage(Error::IMBALANCED_EQUATIONS.clone(), list![(s1.clone()).clone(), (s2.clone()).clone(), (s3.clone()).clone(), (s4.clone()).clone()], Absyn::dummyInfo.clone())?;
        bail!("fail");
    }
    syst = BackendDAEUtil::createEqSystem(vars.clone(), arr.clone(), metamodelica::nil(), crate::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, remArr.clone());
    success = success.clone() && i1.clone() == i2.clone();
    oTpl = (success.clone(), throwNoError.clone());
    Ok((syst, oTpl))
}

fn partitionEquations(mut arr: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut ixs: metamodelica::Array<i32>, mut ea: metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut restEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut ix: i32 = 0;
    let mut lst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    for mut i in (1..=BackendEquation::getNumberOfEquations(arr.clone())).rev() {
        ix = ixs.borrow()[(i.clone()-1) as usize].clone();
        eq = BackendEquation::get(arr.clone(), i.clone());
        if ix.clone() == 0 {
            restEqs = cons(eq.clone(), restEqs.clone());
        } else {
            lst = ea.borrow()[(ix.clone()-1) as usize].clone();
            lst = cons(eq.clone(), lst.clone());
            {let _arr = ea.clone(); _arr.borrow_mut()[(ix.clone()-1) as usize] = lst.clone(); _arr};
        }
    }
    Ok(restEqs)
}

fn subClkEqual(mut sc1: BackendDAE::SubClock, mut sc2: BackendDAE::SubClock) -> Result<bool> {
    let mut isEqual: bool = false;
    isEqual = (match (sc1.clone(), sc2.clone()) {
        (BackendDAE::SubClock::INFERED_SUBCLOCK, BackendDAE::SubClock::INFERED_SUBCLOCK) => true,
        (BackendDAE::SubClock::SUBCLOCK { .. }, BackendDAE::SubClock::SUBCLOCK { .. }) => MMath::equals(var_field!(sc1.factor, BackendDAE::SubClock::SUBCLOCK).clone(), var_field!(sc2.factor, BackendDAE::SubClock::SUBCLOCK).clone())? && MMath::equals(var_field!(sc1.shift, BackendDAE::SubClock::SUBCLOCK).clone(), var_field!(sc2.shift, BackendDAE::SubClock::SUBCLOCK).clone())? && Util::optionEqual(var_field!(sc1.solver, BackendDAE::SubClock::SUBCLOCK).clone(), var_field!(sc2.solver, BackendDAE::SubClock::SUBCLOCK).clone(), Arc::new(fnptr!(stringEqual, ArcStr, ArcStr))),
        _ => false,
    });
    Ok(isEqual)
}

fn subClockTreeString(mut treeIn: metamodelica::Array<(BackendDAE::SubClock, i32)>) -> Result<ArcStr> {
    let mut sOut: ArcStr = literal!("");
    let mut tpl: (BackendDAE::SubClock, i32);
    let mut subClock: BackendDAE::SubClock = BackendDAE::SubClock::INFERED_SUBCLOCK;
    let mut i: i32 = 0;
    let mut idx: i32 = 1;
    let __range0 = treeIn.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut tpl in __range0 {
        (subClock, i) = tpl.clone();
        sOut = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(idx.clone())); __mm_s.push_str(&*literal!(": [")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!("]:  ")); __mm_s.push_str(&*BackendDump::subClockString(subClock.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*sOut.clone()); ArcStr::from(__mm_s) }).clone();
        idx = idx.clone() + 1;
    }
    Ok(sOut)
}

