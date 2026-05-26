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
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVariable;
use crate::SynchronousFeatures;
use crate::ZeroCrossings;
use openmodelica_ast::Absyn;
use openmodelica_frontend::CheckModel;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::DAEDump;
use openmodelica_frontend::DAEUtil;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionSimplify;
use openmodelica_frontend::HashTableExpToIndex;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::List;

pub type ZCArgType = ((BackendDAE::ZeroCrossingSet, DoubleEnded::MutableList<BackendDAE::ZeroCrossing>, BackendDAE::ZeroCrossingSet, i32), (i32, BackendDAE::Variables, BackendDAE::Variables), Option<Arc<metamodelica::List<BackendDAE::SimIterator>>>);

pub type ForArgType = (Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<DAE::Exp>, (BackendDAE::ZeroCrossingSet, DoubleEnded::MutableList<BackendDAE::ZeroCrossing>, BackendDAE::ZeroCrossingSet, i32), (i32, BackendDAE::Variables, BackendDAE::Variables));

// =============================================================================
// section for preOptModule >>encapsulateWhenConditions<<
//
// This module encapsulates each when-condition in a boolean-variable
// $whenConditionsN and generates to each of these variables an equation
// $whenConditions = whenConditions
// =============================================================================
pub fn encapsulateWhenConditions(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut syst: Arc<BackendDAE::EqSystem>;
    let mut shared: Arc<BackendDAE::Shared>;
    let mut index: i32 = 0;
    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
    let mut vars: DoubleEnded::MutableList<BackendDAE::Var>;
    let mut eqns: DoubleEnded::MutableList<Arc<BackendDAE::Equation>>;
    let mut vars_: BackendDAE::Variables;
    let mut eqns_: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut removedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ DAE { UNIQUEIO: __pa0, derivativeNamePrefix: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    systs = __pa0.clone();
    shared = __pa1.clone();
    ht = HashTableExpToIndex::emptyHashTable();
    (systs, index, ht) = List::mapFold2(systs.clone(), Arc::new(encapsulateWhenConditions_EqSystem), 1, ht.clone());
    (removedEqs, vars, eqns, index, _) = BackendEquation::traverseEquationArray(shared.removedEqs.clone(), Arc::new(encapsulateWhenConditions_Equation), (BackendEquation::emptyEqnsSized(BackendEquation::getNumberOfEquations(shared.removedEqs.clone())), DoubleEnded::fromList(metamodelica::nil())?, DoubleEnded::fromList(metamodelica::nil())?, index.clone(), ht.clone()))?;
    assign_field!(shared.removedEqs = removedEqs.clone());
    eqns_ = BackendEquation::listEquation(DoubleEnded::toListNoCopyNoClear(eqns.clone()))?;
    vars_ = BackendVariable::listVar(DoubleEnded::toListNoCopyNoClear(vars.clone()));
    syst = BackendDAEUtil::createEqSystem(vars_.clone(), eqns_.clone(), metamodelica::nil(), crate::BackendDAE::BaseClockPartitionKind::UNSPECIFIED_PARTITION, BackendEquation::emptyEqns());
    systs = List::appendElt(syst.clone(), systs.clone());
    outDAE = BackendDAE::DAE(systs.clone(), shared.clone())?;
    if index.clone() > 1 {
        outDAE = SynchronousFeatures::contPartitioning(outDAE.clone())?;
    }
    if Flags::isSet(Flags::DUMP_ENCAPSULATECONDITIONS.clone())? {
        BackendDump::dumpBackendDAE(outDAE.clone(), (literal!("DAE after PreOptModule >>encapsulateWhenConditions<<")).clone())?;
    }
    Ok(outDAE)
}

fn encapsulateWhenConditions_EqSystem(mut inEqSystem: Arc<BackendDAE::EqSystem>, mut inIndex: i32, mut inHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr))) -> Result<(Arc<BackendDAE::EqSystem>, i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)))> {
    let mut outEqSystem: Arc<BackendDAE::EqSystem>;
    let mut outIndex: i32 = 0;
    let mut outHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
    outEqSystem = (::match_deref::match_deref! { match &(inEqSystem.clone()) {
        syst @ Deref @ BackendDAE::EqSystem { orderedEqs, orderedVars, .. } => {
            let mut removedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
            let mut varLst: DoubleEnded::MutableList<BackendDAE::Var>;
            let mut eqnLst: DoubleEnded::MutableList<Arc<BackendDAE::Equation>>;
            let mut syst = (*syst).clone();
            let mut orderedEqs = (*orderedEqs).clone();
            (orderedEqs, varLst, eqnLst, outIndex, outHT) = BackendEquation::traverseEquationArray(orderedEqs.clone(), Arc::new(encapsulateWhenConditions_Equation), (BackendEquation::emptyEqnsSized(BackendEquation::getNumberOfEquations(orderedEqs.clone())), DoubleEnded::fromList(metamodelica::nil())?, DoubleEnded::fromList(metamodelica::nil())?, inIndex.clone(), inHT.clone()))?;
            (removedEqs, varLst, eqnLst, outIndex, outHT) = BackendEquation::traverseEquationArray(syst.removedEqs.clone(), Arc::new(encapsulateWhenConditions_Equation), (BackendEquation::emptyEqnsSized(BackendEquation::getNumberOfEquations(syst.removedEqs.clone())), varLst.clone(), eqnLst.clone(), outIndex.clone(), outHT.clone()))?;
            assign_field!(
                syst.removedEqs = removedEqs.clone(),
                syst.orderedVars = BackendVariable::addVars(DoubleEnded::toListNoCopyNoClear(varLst.clone()), orderedVars.clone()),
                syst.orderedEqs = BackendEquation::addList(DoubleEnded::toListNoCopyNoClear(eqnLst.clone()), orderedEqs.clone())?
            );
            BackendDAEUtil::clearEqSyst(syst.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outEqSystem, outIndex, outHT))
}

fn encapsulateWhenConditions_Equation(mut inEq: Arc<BackendDAE::Equation>, mut inTpl: (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, DoubleEnded::MutableList<BackendDAE::Var>, DoubleEnded::MutableList<Arc<BackendDAE::Equation>>, i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)))) -> Result<(Arc<BackendDAE::Equation>, (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, DoubleEnded::MutableList<BackendDAE::Var>, DoubleEnded::MutableList<Arc<BackendDAE::Equation>>, i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr))))> {
    let mut outEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut outTpl: (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, DoubleEnded::MutableList<BackendDAE::Var>, DoubleEnded::MutableList<Arc<BackendDAE::Equation>>, i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)));
    (outEq, outTpl) = (::match_deref::match_deref! { match &((inEq.clone(), inTpl.clone())) {
        (Deref @ BackendDAE::Equation::WHEN_EQUATION { attr, source, whenEquation, size }, (equationArray, vars, eqns, index, ht)) => {
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut vars1: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut eqns1: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut whenEquation = (*whenEquation).clone();
            let mut equationArray = (*equationArray).clone();
            let mut index = (*index).clone();
            let mut ht = (*ht).clone();
            (whenEquation, vars1, eqns1, index, ht) = encapsulateWhenConditions_Equations(whenEquation.clone(), source.clone(), index.clone(), ht.clone())?;
            DoubleEnded::push_list_back(vars.clone(), vars1.clone());
            DoubleEnded::push_list_back(eqns.clone(), eqns1.clone());
            eqn = Arc::new(BackendDAE::Equation::WHEN_EQUATION { size: size.clone(), whenEquation: whenEquation.clone(), source: source.clone(), attr: attr.clone() });
            equationArray = BackendEquation::add(eqn.clone(), equationArray.clone())?;
            (eqn.clone(), (equationArray.clone(), vars.clone(), eqns.clone(), index.clone(), ht.clone()))
        },
        (Deref @ BackendDAE::Equation::ALGORITHM { attr, expand: crefExpand, source, alg: alg_, size: 0 }, (equationArray, vars, eqns, index, ht)) => {
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut eqn2: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut size: i32 = 0;
            let mut sizePre: i32 = 0;
            let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut preStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut allPreStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut allStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut alg_ = (*alg_).clone();
            let mut equationArray = (*equationArray).clone();
            let mut index = (*index).clone();
            let __pa0 = ::match_deref::match_deref! { match &(alg_.clone()) {
                Deref @ DAE::Algorithm { statementLst: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            stmts = __pa0.clone();
            size = -(index.clone());
            allPreStmts = metamodelica::nil();
            allStmts = metamodelica::nil();
            for mut stmt in &*stmts.clone() {
                let mut stmt = stmt.clone();
                (stmts, preStmts, index) = encapsulateWhenConditions_Algorithms(list![stmt.clone()], vars.clone(), index.clone())?;
                allPreStmts = listAppend(preStmts.clone(), allPreStmts.clone());
                allStmts = listAppend(stmts.clone(), allStmts.clone());
            }
            stmts = allStmts.clone().reverse();
            sizePre = (allPreStmts.clone().len() as i32);
            size = size.clone() + index.clone() - sizePre.clone();
            alg_ = Arc::new(DAE::Algorithm { statementLst: stmts.clone() });
            eqn = Arc::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: alg_.clone(), source: source.clone(), expand: crefExpand.clone(), attr: attr.clone() });
            equationArray = BackendEquation::add(eqn.clone(), equationArray.clone())?;
            if sizePre.clone() > 0 {
                alg_ = Arc::new(DAE::Algorithm { statementLst: allPreStmts.clone() });
                eqn2 = Arc::new(BackendDAE::Equation::ALGORITHM { size: sizePre.clone(), alg: alg_.clone(), source: source.clone(), expand: crefExpand.clone(), attr: attr.clone() });
                DoubleEnded::push_front(eqns.clone(), eqn2.clone());
            }
            (eqn.clone(), (equationArray.clone(), vars.clone(), eqns.clone(), index.clone(), ht.clone()))
        },
        (Deref @ BackendDAE::Equation::ALGORITHM { attr, expand: crefExpand, source, alg: alg_, size }, (equationArray, vars, eqns, index, ht)) => {
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut preStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut alg_ = (*alg_).clone();
            let mut size = (*size).clone();
            let mut equationArray = (*equationArray).clone();
            let mut index = (*index).clone();
            let __pa0 = ::match_deref::match_deref! { match &(alg_.clone()) {
                Deref @ DAE::Algorithm { statementLst: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            stmts = __pa0.clone();
            size = size.clone() - index.clone();
            (stmts, preStmts, index) = encapsulateWhenConditions_Algorithms(stmts.clone(), vars.clone(), index.clone())?;
            size = size.clone() + index.clone();
            stmts = listAppend(preStmts.clone(), stmts.clone());
            alg_ = Arc::new(DAE::Algorithm { statementLst: stmts.clone() });
            eqn = Arc::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: alg_.clone(), source: source.clone(), expand: crefExpand.clone(), attr: attr.clone() });
            equationArray = BackendEquation::add(eqn.clone(), equationArray.clone())?;
            (eqn.clone(), (equationArray.clone(), vars.clone(), eqns.clone(), index.clone(), ht.clone()))
        },
        (_, (equationArray, vars, eqns, index, ht)) => {
            let mut equationArray = (*equationArray).clone();
            equationArray = BackendEquation::add(inEq.clone(), equationArray.clone())?;
            (inEq.clone(), (equationArray.clone(), vars.clone(), eqns.clone(), index.clone(), ht.clone()))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outEq, outTpl))
}

fn encapsulateWhenConditions_Equations(mut inWhenEquation: Arc<BackendDAE::WhenEquation>, mut inSource: Arc<DAE::ElementSource>, mut inIndex: i32, mut inHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr))) -> Result<(Arc<BackendDAE::WhenEquation>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)))> {
    let mut outWhenEquation: Arc<BackendDAE::WhenEquation>;
    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut outEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut outIndex: i32 = 0;
    let mut outHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
    (outWhenEquation, outVars, outEqns, outIndex, outHT) = (::match_deref::match_deref! { match &(inWhenEquation.clone()) {
        Deref @ BackendDAE::WhenEquation { elsewhenPart: None, whenStmtLst, condition } => {
            let mut index: i32 = 0;
            let mut whenEquation: Arc<BackendDAE::WhenEquation>;
            let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
            let mut condition = (*condition).clone();
            (condition, vars, eqns, index, ht) = encapsulateWhenConditions_Equations1(condition.clone(), inSource.clone(), inIndex.clone(), inHT.clone())?;
            whenEquation = Arc::new(BackendDAE::WhenEquation { condition: condition.clone(), whenStmtLst: whenStmtLst.clone(), elsewhenPart: None });
            (whenEquation.clone(), vars.clone(), eqns.clone(), index.clone(), ht.clone())
        },
        Deref @ BackendDAE::WhenEquation { elsewhenPart: Some(elsewhenPart), whenStmtLst, condition } => {
            let mut index: i32 = 0;
            let mut whenEquation: Arc<BackendDAE::WhenEquation>;
            let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut vars1: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut eqns1: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
            let mut elsewhenPart = (*elsewhenPart).clone();
            let mut condition = (*condition).clone();
            (elsewhenPart, vars1, eqns1, index, ht) = encapsulateWhenConditions_Equations(elsewhenPart.clone(), inSource.clone(), inIndex.clone(), inHT.clone())?;
            (condition, vars, eqns, index, ht) = encapsulateWhenConditions_Equations1(condition.clone(), inSource.clone(), index.clone(), ht.clone())?;
            whenEquation = Arc::new(BackendDAE::WhenEquation { condition: condition.clone(), whenStmtLst: whenStmtLst.clone(), elsewhenPart: Some(elsewhenPart.clone()) });
            vars1 = listAppend(vars.clone(), vars1.clone());
            eqns1 = listAppend(eqns.clone(), eqns1.clone());
            (whenEquation.clone(), vars1.clone(), eqns1.clone(), index.clone(), ht.clone())
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FindZeroCrossings.encapsulateWhenConditions_Equations")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outWhenEquation, outVars, outEqns, outIndex, outHT))
}

fn encapsulateWhenConditions_Equations1(mut inCondition: Arc<DAE::Exp>, mut inSource: Arc<DAE::ElementSource>, mut inIndex: i32, mut inHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr))) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)))> {
    let mut outCondition: Arc<DAE::Exp>;
    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut outEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut outIndex: i32 = 0;
    let mut outHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
    (outCondition, outVars, outEqns, outIndex, outHT) = (::match_deref::match_deref! { match &(inCondition.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initial" }, .. } => {
            (inCondition.clone(), metamodelica::nil(), metamodelica::nil(), inIndex.clone(), inHT.clone())
        },
        _ if (Expression::isConst(inCondition.clone())?) => {
            (inCondition.clone(), metamodelica::nil(), metamodelica::nil(), inIndex.clone(), inHT.clone())
        },
        Deref @ DAE::Exp::ARRAY { array, scalar, ty } => {
            let mut index: i32 = 0;
            let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
            let mut array = (*array).clone();
            (array, vars, eqns, index, ht) = encapsulateWhenConditions_EquationsWithArrayConditions(array.clone(), inSource.clone(), inIndex.clone(), inHT.clone())?;
            (Arc::new(DAE::Exp::ARRAY { ty: ty.clone(), scalar: scalar.clone(), array: array.clone() }), vars.clone(), eqns.clone(), index.clone(), ht.clone())
        },
        _ if (BaseHashTable::hasKey(inCondition.clone(), inHT.clone())) => {
            let mut localIndex: i32 = 0;
            let mut crStr: ArcStr = arcstr::literal!("");
            let mut condition: Arc<DAE::Exp>;
            localIndex = BaseHashTable::get(inCondition.clone(), inHT.clone())?;
            crStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$whenCondition")); __mm_s.push_str(&*intString(localIndex.clone())); ArcStr::from(__mm_s) }).clone();
            condition = Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (crStr.clone()).clone(), identType: DAE::T_BOOL_DEFAULT.clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_BOOL_DEFAULT.clone() });
            (condition.clone(), metamodelica::nil(), metamodelica::nil(), inIndex.clone(), inHT.clone())
        },
        _ => {
            let mut var: BackendDAE::Var;
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut crStr: ArcStr = arcstr::literal!("");
            let mut condition: Arc<DAE::Exp>;
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
            ht = BaseHashTable::add((inCondition.clone(), inIndex.clone()), inHT.clone())?;
            crStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$whenCondition")); __mm_s.push_str(&*intString(inIndex.clone())); ArcStr::from(__mm_s) }).clone();
            var = BackendDAE::Var { varName: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (crStr.clone()).clone(), identType: DAE::T_BOOL_DEFAULT.clone(), subscriptLst: metamodelica::nil() }), varKind: crate::BackendDAE::VarKind::DISCRETE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_BOOL_DEFAULT.clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: inSource.clone(), values: DAEUtil::setProtectedAttr(Some(DAE::emptyVarAttrBool.clone()), true)?, tearingSelectOption: None, hideResult: Some(Arc::new(DAE::Exp::BCONST { bool: true })), comment: Some(Arc::new(SCode::Comment { annotation_: None, comment: Some((ExpressionBasics::printExpStr(inCondition.clone())?).clone()) })), connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: true, initNonlinear: false, encrypted: false };
            var = BackendVariable::setVarFixed(var.clone(), true)?;
            eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (crStr.clone()).clone(), identType: DAE::T_BOOL_DEFAULT.clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_BOOL_DEFAULT.clone() }), scalar: inCondition.clone(), source: inSource.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
            condition = Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (crStr.clone()).clone(), identType: DAE::T_BOOL_DEFAULT.clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_BOOL_DEFAULT.clone() });
            (condition.clone(), list![var.clone()], list![eqn.clone()], inIndex.clone() + 1, ht.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCondition, outVars, outEqns, outIndex, outHT))
}

fn encapsulateWhenConditions_EquationsWithArrayConditions(mut inConditionList: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inSource: Arc<DAE::ElementSource>, mut inIndex: i32, mut inHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr))) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)))> {
    let mut outConditionList: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut outEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut outIndex: i32 = inIndex.clone();
    let mut outHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)) = inHT.clone();
    let mut vars1: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut eqns1: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    for mut condition in &*inConditionList.clone() {
        let mut condition = condition.clone();
        (condition, vars1, eqns1, outIndex, outHT) = encapsulateWhenConditions_Equations1(condition.clone(), inSource.clone(), outIndex.clone(), outHT.clone())?;
        outVars = List::append_reverse(vars1.clone(), outVars.clone());
        outEqns = List::append_reverse(eqns1.clone(), outEqns.clone());
        outConditionList = cons(condition.clone(), outConditionList.clone());
    }
    outVars = outVars.clone().reverse();
    outEqns = outEqns.clone().reverse();
    outConditionList = outConditionList.clone().reverse();
    Ok((outConditionList, outVars, outEqns, outIndex, outHT))
}

fn encapsulateWhenConditions_Algorithms(mut inStmts: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut vars: DoubleEnded::MutableList<BackendDAE::Var>, mut inIndex: i32) -> Result<(Arc<metamodelica::List<Arc<DAE::Statement>>>, Arc<metamodelica::List<Arc<DAE::Statement>>>, i32)> {
    let mut outStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut outPreStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut outIndex: i32 = 0;
    (outStmts, outPreStmts, outIndex) = (::match_deref::match_deref! { match &(inStmts.clone()) {
        Deref @ metamodelica::List::Nil => {
            (metamodelica::nil(), metamodelica::nil(), inIndex.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_WHEN { source, elseWhen: None, statementLst: stmts1, exp: condition, .. }, tail: rest } => {
            let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut stmts_: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut preStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut preStmts2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut index: i32 = 0;
            let mut vars1: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut conditions: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut initialCall: bool = false;
            let mut condition = (*condition).clone();
            (condition, vars1, preStmts, index) = encapsulateWhenConditions_Algorithms1(condition.clone(), source.clone(), inIndex.clone())?;
            (conditions, initialCall) = BackendDAEUtil::getConditionList(condition.clone())?;
            DoubleEnded::push_list_front(vars.clone(), vars1.clone())?;
            if CheckModel::algorithmStatementListOutputs(stmts1.clone(), openmodelica_frontend_types::DAE::Expand::EXPAND)?.is_empty() {
                (stmts, preStmts2, index) = encapsulateWhenConditions_Algorithms(rest.clone(), vars.clone(), index.clone())?;
                preStmts = listAppend(preStmts.clone(), preStmts2.clone());
                stmts = cons(Arc::new(DAE::Statement::STMT_WHEN { exp: condition.clone(), conditions: conditions.clone(), initialCall: initialCall.clone(), statementLst: stmts1.clone(), elseWhen: None, source: source.clone() }), stmts.clone());
            } else {
                (stmts, stmts_, index) = encapsulateWhenConditions_Algorithms(rest.clone(), vars.clone(), index.clone())?;
                stmts_ = cons(Arc::new(DAE::Statement::STMT_WHEN { exp: condition.clone(), conditions: conditions.clone(), initialCall: initialCall.clone(), statementLst: stmts1.clone(), elseWhen: None, source: source.clone() }), stmts_.clone());
                stmts = listAppend(stmts_.clone(), stmts.clone());
            }
            (stmts.clone(), preStmts.clone(), index.clone())
        },
        Deref @ metamodelica::List::Cons { head: stmt @ Deref @ DAE::Statement::STMT_WHEN { source, elseWhen: Some(elseWhen), statementLst: stmts1, exp: condition, .. }, tail: rest } => {
            let mut stmt2: Arc<DAE::Statement>;
            let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut stmts_: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut preStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut preStmts2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut elseWhenList: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut index: i32 = 0;
            let mut vars1: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut conditions: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut initialCall: bool = false;
            let mut elseWhen = (*elseWhen).clone();
            let mut condition = (*condition).clone();
            (condition, vars1, preStmts, index) = encapsulateWhenConditions_Algorithms1(condition.clone(), source.clone(), inIndex.clone())?;
            (conditions, initialCall) = BackendDAEUtil::getConditionList(condition.clone())?;
            DoubleEnded::push_list_front(vars.clone(), vars1.clone())?;
            (elseWhenList, preStmts2, index) = encapsulateWhenConditions_Algorithms(list![elseWhen.clone()], vars.clone(), index.clone())?;
            if elseWhenList.clone().is_empty() {
                (stmts, preStmts, index) = encapsulateWhenConditions_Algorithms(rest.clone(), vars.clone(), inIndex.clone())?;
                stmts_ = cons(stmt.clone(), listAppend(preStmts.clone(), stmts.clone()));
            } else {
                elseWhen = List::last(elseWhenList.clone())?;
                stmt2 = Arc::new(DAE::Statement::STMT_WHEN { exp: condition.clone(), conditions: conditions.clone(), initialCall: initialCall.clone(), statementLst: stmts1.clone(), elseWhen: Some(elseWhen.clone()), source: source.clone() });
                if CheckModel::algorithmStatementListOutputs(list![stmt2.clone()], openmodelica_frontend_types::DAE::Expand::EXPAND)?.is_empty() {
                    preStmts2 = List::stripLast(elseWhenList.clone())?;
                    preStmts = listAppend(preStmts.clone(), preStmts2.clone());
                    (stmts, preStmts2, index) = encapsulateWhenConditions_Algorithms(rest.clone(), vars.clone(), index.clone())?;
                    preStmts = listAppend(preStmts.clone(), preStmts2.clone());
                    stmts_ = cons(stmt2.clone(), stmts.clone());
                } else if (elseWhenList.clone().len() as i32) == 1 {
                    preStmts = listAppend(preStmts.clone(), preStmts2.clone());
                    (stmts, stmts_, index) = encapsulateWhenConditions_Algorithms(rest.clone(), vars.clone(), index.clone())?;
                    stmts_ = cons(stmt2.clone(), listAppend(stmts_.clone(), stmts.clone()));
                } else {
                    (stmts, preStmts, index) = encapsulateWhenConditions_Algorithms(rest.clone(), vars.clone(), inIndex.clone())?;
                    stmts_ = listAppend(preStmts.clone(), stmts.clone());
                }
            }
            (stmts_.clone(), preStmts.clone(), index.clone())
        },
        Deref @ metamodelica::List::Cons { head: stmt, tail: rest } => {
            let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut preStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut index: i32 = 0;
            (stmts, preStmts, index) = encapsulateWhenConditions_Algorithms(rest.clone(), vars.clone(), inIndex.clone())?;
            stmts = listAppend(preStmts.clone(), stmts.clone());
            (cons(stmt.clone(), stmts.clone()), metamodelica::nil(), index.clone())
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FindZeroCrossings.encapsulateWhenConditions_Algorithms")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outStmts, outPreStmts, outIndex))
}

fn encapsulateWhenConditions_Algorithms1(mut inCondition: Arc<DAE::Exp>, mut inSource: Arc<DAE::ElementSource>, mut inIndex: i32) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<DAE::Statement>>>, i32)> {
    let mut outCondition: Arc<DAE::Exp>;
    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut outStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut outIndex: i32 = 0;
    (outCondition, outVars, outStmts, outIndex) = (::match_deref::match_deref! { match &(inCondition.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initial" }, .. } => {
            (inCondition.clone(), metamodelica::nil(), metamodelica::nil(), inIndex.clone())
        },
        _ if (Expression::isConst(inCondition.clone())?) => {
            (inCondition.clone(), metamodelica::nil(), metamodelica::nil(), inIndex.clone())
        },
        Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Cons { head: condition, tail: Deref @ metamodelica::List::Nil }, .. } => {
            let mut var: BackendDAE::Var;
            let mut stmt: Arc<DAE::Statement>;
            let mut crStr: ArcStr = arcstr::literal!("");
            let mut condition = (*condition).clone();
            crStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$whenCondition")); __mm_s.push_str(&*intString(inIndex.clone())); ArcStr::from(__mm_s) }).clone();
            var = BackendDAE::Var { varName: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (crStr.clone()).clone(), identType: DAE::T_BOOL_DEFAULT.clone(), subscriptLst: metamodelica::nil() }), varKind: crate::BackendDAE::VarKind::DISCRETE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_BOOL_DEFAULT.clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: inSource.clone(), values: DAEUtil::setProtectedAttr(Some(DAE::emptyVarAttrBool.clone()), true)?, tearingSelectOption: None, hideResult: Some(Arc::new(DAE::Exp::BCONST { bool: true })), comment: Some(Arc::new(SCode::Comment { annotation_: None, comment: Some((ExpressionBasics::printExpStr(inCondition.clone())?).clone()) })), connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: true, initNonlinear: false, encrypted: false };
            var = BackendVariable::setVarFixed(var.clone(), true)?;
            stmt = Arc::new(DAE::Statement::STMT_ASSIGN { type_: DAE::T_BOOL_DEFAULT.clone(), exp1: Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (crStr.clone()).clone(), identType: DAE::T_BOOL_DEFAULT.clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_BOOL_DEFAULT.clone() }), exp: condition.clone(), source: inSource.clone() });
            condition = Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (crStr.clone()).clone(), identType: DAE::T_BOOL_DEFAULT.clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_BOOL_DEFAULT.clone() });
            (condition.clone(), list![var.clone()], list![stmt.clone()], inIndex.clone() + 1)
        },
        Deref @ DAE::Exp::ARRAY { array, scalar, ty } => {
            let mut index: i32 = 0;
            let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut array = (*array).clone();
            (array, vars, stmts, index) = encapsulateWhenConditions_AlgorithmsWithArrayConditions(array.clone(), inSource.clone(), inIndex.clone())?;
            (Arc::new(DAE::Exp::ARRAY { ty: ty.clone(), scalar: scalar.clone(), array: array.clone() }), vars.clone(), stmts.clone(), index.clone())
        },
        _ => {
            let mut var: BackendDAE::Var;
            let mut stmt: Arc<DAE::Statement>;
            let mut crStr: ArcStr = arcstr::literal!("");
            let mut condition: Arc<DAE::Exp>;
            crStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$whenCondition")); __mm_s.push_str(&*intString(inIndex.clone())); ArcStr::from(__mm_s) }).clone();
            var = BackendDAE::Var { varName: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (crStr.clone()).clone(), identType: DAE::T_BOOL_DEFAULT.clone(), subscriptLst: metamodelica::nil() }), varKind: crate::BackendDAE::VarKind::DISCRETE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_BOOL_DEFAULT.clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: inSource.clone(), values: DAEUtil::setProtectedAttr(Some(DAE::emptyVarAttrBool.clone()), true)?, tearingSelectOption: None, hideResult: Some(Arc::new(DAE::Exp::BCONST { bool: true })), comment: Some(Arc::new(SCode::Comment { annotation_: None, comment: Some((ExpressionBasics::printExpStr(inCondition.clone())?).clone()) })), connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: true, initNonlinear: false, encrypted: false };
            var = BackendVariable::setVarFixed(var.clone(), true)?;
            stmt = Arc::new(DAE::Statement::STMT_ASSIGN { type_: DAE::T_BOOL_DEFAULT.clone(), exp1: Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (crStr.clone()).clone(), identType: DAE::T_BOOL_DEFAULT.clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_BOOL_DEFAULT.clone() }), exp: inCondition.clone(), source: inSource.clone() });
            condition = Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (crStr.clone()).clone(), identType: DAE::T_BOOL_DEFAULT.clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_BOOL_DEFAULT.clone() });
            (condition.clone(), list![var.clone()], list![stmt.clone()], inIndex.clone() + 1)
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FindZeroCrossings.encapsulateWhenConditions_Algorithms1")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FindZeroCrossings.encapsulateWhenConditions_Algorithms1")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCondition, outVars, outStmts, outIndex))
}

fn encapsulateWhenConditions_AlgorithmsWithArrayConditions(mut inConditionList: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inSource: Arc<DAE::ElementSource>, mut inIndex: i32) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<DAE::Statement>>>, i32)> {
    let mut outConditionList: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut outStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut outIndex: i32 = inIndex.clone();
    let mut vars1: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut stmt1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    for mut condition in &*inConditionList.clone() {
        let mut condition = condition.clone();
        (condition, vars1, stmt1, outIndex) = encapsulateWhenConditions_Algorithms1(condition.clone(), inSource.clone(), outIndex.clone())?;
        outVars = List::append_reverse(vars1.clone(), outVars.clone());
        outStmts = List::append_reverse(stmt1.clone(), outStmts.clone());
        outConditionList = cons(condition.clone(), outConditionList.clone());
    }
    outVars = outVars.clone().reverse();
    outStmts = outStmts.clone().reverse();
    outConditionList = outConditionList.clone().reverse();
    Ok((outConditionList, outVars, outStmts, outIndex))
}

// =============================================================================
// section for zero crossings
//
// This section contains all the functions to find zero crossings inside
// BackendDAE.
// =============================================================================
pub fn findZeroCrossings(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    outDAE = BackendDAEUtil::mapEqSystem(inDAE.clone(), Arc::new(findZeroCrossings1))?;
    Ok(outDAE)
}

fn findZeroCrossings1(mut inSyst: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> {
    let mut outSyst: Arc<BackendDAE::EqSystem> = inSyst.clone();
    let mut outShared: Arc<BackendDAE::Shared>;
    let mut vars: BackendDAE::Variables;
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut matching: Arc<BackendDAE::Matching> = Arc::new(BackendDAE::Matching::NO_MATCHING);
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inSyst.clone()) {
        Deref @ BackendDAE::EqSystem { matching: __pa0, orderedEqs: __pa1, orderedVars: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    matching = __pa0.clone();
    eqns = __pa1.clone();
    vars = __pa2.clone();
    (outSyst, outShared) = (match BackendDAEUtil::getSubClock(inSyst.clone(), inShared.clone()) {
        Some(BackendDAE::SubClock::SUBCLOCK { solver: mut solver, .. }) if (BackendDump::optionString(solver.clone()) != literal!("External")) => {
            (inSyst.clone(), inShared.clone())
        },
        _ => {
            let mut globalKnownVars: BackendDAE::Variables;
            let mut eqns1: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
            let mut einfo: BackendDAE::EventInfo;
            let mut eqs_lst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut eqs_lst1: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut timeEvents: Arc<metamodelica::List<BackendDAE::TimeEvent>> = metamodelica::nil();
            let mut zero_crossings: BackendDAE::ZeroCrossingSet;
            let mut sampleLst: BackendDAE::ZeroCrossingSet;
            let mut relations: DoubleEnded::MutableList<BackendDAE::ZeroCrossing>;
            let mut countMathFunctions: i32 = 0;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inShared.clone()) {
                Deref @ BackendDAE::Shared { eventInfo: __pa0, globalKnownVars: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            einfo = __pa0.clone();
            globalKnownVars = __pa1.clone();
            let BackendDAE::EVENT_INFO { numberMathEvents: __pa2, relations: __pa3, samples: __pa4, zeroCrossings: __pa5, timeEvents: __pa6 } = (einfo.clone()) else { bail!("pattern mismatch") };
            countMathFunctions = __pa2.clone();
            relations = __pa3.clone();
            sampleLst = __pa4.clone();
            zero_crossings = __pa5.clone();
            timeEvents = __pa6.clone();
            eqs_lst = BackendEquation::equationList(eqns.clone());
            (zero_crossings, eqs_lst1, countMathFunctions, relations, sampleLst) = findZeroCrossings2(vars.clone(), globalKnownVars.clone(), eqs_lst.clone(), 0, countMathFunctions.clone(), zero_crossings.clone(), relations.clone(), sampleLst.clone(), metamodelica::nil())?;
            eqs_lst1 = eqs_lst1.clone().reverse();
            eqns1 = BackendEquation::listEquation(eqs_lst1.clone())?;
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("findZeroCrossings1 number of relations: ")); __mm_s.push_str(&*intString(DoubleEnded::length(relations.clone()))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("findZeroCrossings1 sample index: ")); __mm_s.push_str(&*intString(ZeroCrossings::length(sampleLst.clone())?)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            if '__try7: {
                let (__pa8, __pa9, __pa10) = ::match_deref::match_deref! { match &(matching.clone()) {
                    Deref @ BackendDAE::Matching::MATCHING { ass2: __pa8, ass1: __pa9, comps: __pa10 } => (__pa8.clone(), __pa9.clone(), __pa10.clone()),
                    _ => break '__try7 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                } };
                ass2 = __pa8.clone();
                ass1 = __pa9.clone();
                comps = __pa10.clone();
                comps = unwrap_break_err!(findZeroCrossingsinJacobians(comps.clone(), zero_crossings.clone(), relations.clone(), sampleLst.clone(), vars.clone(), globalKnownVars.clone()), '__try7);
                assign_field!(
                    outSyst.orderedEqs = eqns1.clone(),
                    outSyst.matching = Arc::new(BackendDAE::Matching::MATCHING { ass1: ass1.clone(), ass2: ass2.clone(), comps: comps.clone() })
                );
                Ok::<(), anyhow::Error>(())
            }.is_err() {
            }
            einfo = BackendDAE::EventInfo { timeEvents: timeEvents.clone(), zeroCrossings: zero_crossings.clone(), relations: relations.clone(), samples: sampleLst.clone(), numberMathEvents: countMathFunctions.clone() };
            (outSyst.clone(), BackendDAEUtil::setSharedEventInfo(inShared.clone(), einfo.clone()))
        },
    });
    Ok((outSyst, outShared))
}

fn findZeroCrossings2(mut inVariables1: BackendDAE::Variables, mut globalKnownVars: BackendDAE::Variables, mut inEquationLst2: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inEqnCount: i32, mut inNumberOfMathFunctions: i32, mut inZeroCrossingLst: BackendDAE::ZeroCrossingSet, mut inRelationsLst: DoubleEnded::MutableList<BackendDAE::ZeroCrossing>, mut inSamplesLst: BackendDAE::ZeroCrossingSet, mut inEquationLstAccum: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(BackendDAE::ZeroCrossingSet, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, i32, DoubleEnded::MutableList<BackendDAE::ZeroCrossing>, BackendDAE::ZeroCrossingSet)> {
    let mut outZeroCrossingLst: BackendDAE::ZeroCrossingSet;
    let mut outEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut outNumberOfMathFunctions: i32 = 0;
    let mut outRelationsLst: DoubleEnded::MutableList<BackendDAE::ZeroCrossing>;
    let mut outSamplesLst: BackendDAE::ZeroCrossingSet;
    (outZeroCrossingLst, outEquationLst, outNumberOfMathFunctions, outRelationsLst, outSamplesLst) = (::match_deref::match_deref! { match &(inEquationLst2.clone()) {
        Deref @ metamodelica::List::Nil => {
            (inZeroCrossingLst.clone(), inEquationLstAccum.clone(), inNumberOfMathFunctions.clone(), inRelationsLst.clone(), inSamplesLst.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::ALGORITHM { attr: eqAttr, expand, source: source_, alg: Deref @ DAE::Algorithm { statementLst: stmts }, size }, tail: xs } => {
            let mut res: BackendDAE::ZeroCrossingSet;
            let mut res1: BackendDAE::ZeroCrossingSet;
            let mut sampleLst: BackendDAE::ZeroCrossingSet;
            let mut relationsLst: DoubleEnded::MutableList<BackendDAE::ZeroCrossing>;
            let mut eq_count: i32 = 0;
            let mut countMathFunctions: i32 = 0;
            let mut eq_reslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut eqnsAccum: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut stmts_1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            eq_count = inEqnCount.clone() + 1;
            let (__pa0, (_, _, _, (__pa1, __pa2, __pa3, __pa4), _)) = traverseStmtsExps(stmts.clone(), (Arc::new(DAE::Exp::SCONST { string: (literal!("$$$")).clone() }), metamodelica::nil(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), (inZeroCrossingLst.clone(), inRelationsLst.clone(), inSamplesLst.clone(), inNumberOfMathFunctions.clone()), (eq_count.clone(), inVariables1.clone(), globalKnownVars.clone())), globalKnownVars.clone())?;
            stmts_1 = __pa0.clone();
            res = __pa1.clone();
            relationsLst = __pa2.clone();
            sampleLst = __pa3.clone();
            countMathFunctions = __pa4.clone();
            eqnsAccum = cons(Arc::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: Arc::new(DAE::Algorithm { statementLst: stmts_1.clone() }), source: source_.clone(), expand: expand.clone(), attr: eqAttr.clone() }), inEquationLstAccum.clone());
            (res1, eq_reslst, countMathFunctions, relationsLst, sampleLst) = findZeroCrossings2(inVariables1.clone(), globalKnownVars.clone(), xs.clone(), eq_count.clone(), countMathFunctions.clone(), res.clone(), relationsLst.clone(), sampleLst.clone(), eqnsAccum.clone())?;
            (res1.clone(), eq_reslst.clone(), countMathFunctions.clone(), relationsLst.clone(), sampleLst.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::WHEN_EQUATION { attr: eqAttr, source: source_, whenEquation: weqn, size }, tail: xs } => {
            let mut res: BackendDAE::ZeroCrossingSet;
            let mut res1: BackendDAE::ZeroCrossingSet;
            let mut sampleLst: BackendDAE::ZeroCrossingSet;
            let mut relationsLst: DoubleEnded::MutableList<BackendDAE::ZeroCrossing>;
            let mut eq_count: i32 = 0;
            let mut countMathFunctions: i32 = 0;
            let mut eq_reslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut eqnsAccum: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut weqn = (*weqn).clone();
            eq_count = inEqnCount.clone() + 1;
            (weqn, countMathFunctions, res, relationsLst, sampleLst) = findZeroCrossingsWhenEqns(weqn.clone(), inZeroCrossingLst.clone(), inRelationsLst.clone(), inSamplesLst.clone(), inNumberOfMathFunctions.clone(), eq_count.clone(), -1, inVariables1.clone(), globalKnownVars.clone())?;
            eqnsAccum = cons(Arc::new(BackendDAE::Equation::WHEN_EQUATION { size: size.clone(), whenEquation: weqn.clone(), source: source_.clone(), attr: eqAttr.clone() }), inEquationLstAccum.clone());
            (res1, eq_reslst, countMathFunctions, relationsLst, sampleLst) = findZeroCrossings2(inVariables1.clone(), globalKnownVars.clone(), xs.clone(), eq_count.clone(), countMathFunctions.clone(), res.clone(), relationsLst.clone(), sampleLst.clone(), eqnsAccum.clone())?;
            (res1.clone(), eq_reslst.clone(), countMathFunctions.clone(), relationsLst.clone(), sampleLst.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::EQUATION { attr: eqAttr, source: source_, scalar: e2, exp: e1 }, tail: xs } => {
            let mut zcs1: BackendDAE::ZeroCrossingSet;
            let mut res: BackendDAE::ZeroCrossingSet;
            let mut res1: BackendDAE::ZeroCrossingSet;
            let mut sampleLst: BackendDAE::ZeroCrossingSet;
            let mut relationsLst: DoubleEnded::MutableList<BackendDAE::ZeroCrossing>;
            let mut eq_count: i32 = 0;
            let mut countMathFunctions: i32 = 0;
            let mut eq_reslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut eqnsAccum: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut eres1: Arc<DAE::Exp>;
            let mut eres2: Arc<DAE::Exp>;
            eq_count = inEqnCount.clone() + 1;
            (eres1, countMathFunctions, zcs1, relationsLst, sampleLst) = findZeroCrossings3(e1.clone(), inZeroCrossingLst.clone(), inRelationsLst.clone(), inSamplesLst.clone(), inNumberOfMathFunctions.clone(), eq_count.clone(), -1, inVariables1.clone(), globalKnownVars.clone())?;
            (eres2, countMathFunctions, res, relationsLst, sampleLst) = findZeroCrossings3(e2.clone(), zcs1.clone(), relationsLst.clone(), sampleLst.clone(), countMathFunctions.clone(), eq_count.clone(), -1, inVariables1.clone(), globalKnownVars.clone())?;
            eqnsAccum = cons(Arc::new(BackendDAE::Equation::EQUATION { exp: eres1.clone(), scalar: eres2.clone(), source: source_.clone(), attr: eqAttr.clone() }), inEquationLstAccum.clone());
            (res1, eq_reslst, countMathFunctions, relationsLst, sampleLst) = findZeroCrossings2(inVariables1.clone(), globalKnownVars.clone(), xs.clone(), eq_count.clone(), countMathFunctions.clone(), res.clone(), relationsLst.clone(), sampleLst.clone(), eqnsAccum.clone())?;
            (res1.clone(), eq_reslst.clone(), countMathFunctions.clone(), relationsLst.clone(), sampleLst.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::COMPLEX_EQUATION { attr: eqAttr, source, right: e2, left: e1, size }, tail: xs } => {
            let mut zcs1: BackendDAE::ZeroCrossingSet;
            let mut res: BackendDAE::ZeroCrossingSet;
            let mut res1: BackendDAE::ZeroCrossingSet;
            let mut sampleLst: BackendDAE::ZeroCrossingSet;
            let mut relationsLst: DoubleEnded::MutableList<BackendDAE::ZeroCrossing>;
            let mut eq_count: i32 = 0;
            let mut countMathFunctions: i32 = 0;
            let mut eq_reslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut eqnsAccum: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut eres1: Arc<DAE::Exp>;
            let mut eres2: Arc<DAE::Exp>;
            eq_count = inEqnCount.clone() + 1;
            (eres1, countMathFunctions, zcs1, relationsLst, sampleLst) = findZeroCrossings3(e1.clone(), inZeroCrossingLst.clone(), inRelationsLst.clone(), inSamplesLst.clone(), inNumberOfMathFunctions.clone(), eq_count.clone(), -1, inVariables1.clone(), globalKnownVars.clone())?;
            (eres2, countMathFunctions, res, relationsLst, sampleLst) = findZeroCrossings3(e2.clone(), zcs1.clone(), relationsLst.clone(), sampleLst.clone(), countMathFunctions.clone(), eq_count.clone(), -1, inVariables1.clone(), globalKnownVars.clone())?;
            eqnsAccum = cons(Arc::new(BackendDAE::Equation::COMPLEX_EQUATION { size: size.clone(), left: eres1.clone(), right: eres2.clone(), source: source.clone(), attr: eqAttr.clone() }), inEquationLstAccum.clone());
            (res1, eq_reslst, countMathFunctions, relationsLst, sampleLst) = findZeroCrossings2(inVariables1.clone(), globalKnownVars.clone(), xs.clone(), eq_count.clone(), countMathFunctions.clone(), res.clone(), relationsLst.clone(), sampleLst.clone(), eqnsAccum.clone())?;
            (res1.clone(), eq_reslst.clone(), countMathFunctions.clone(), relationsLst.clone(), sampleLst.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::ARRAY_EQUATION { recordSize, attr: eqAttr, source, right: e2, left: e1, dimSize: dimsize }, tail: xs } => {
            let mut zcs1: BackendDAE::ZeroCrossingSet;
            let mut res: BackendDAE::ZeroCrossingSet;
            let mut res1: BackendDAE::ZeroCrossingSet;
            let mut sampleLst: BackendDAE::ZeroCrossingSet;
            let mut relationsLst: DoubleEnded::MutableList<BackendDAE::ZeroCrossing>;
            let mut eq_count: i32 = 0;
            let mut countMathFunctions: i32 = 0;
            let mut eq_reslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut eqnsAccum: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut eres1: Arc<DAE::Exp>;
            let mut eres2: Arc<DAE::Exp>;
            eq_count = inEqnCount.clone() + 1;
            (eres1, countMathFunctions, zcs1, relationsLst, sampleLst) = findZeroCrossings3(e1.clone(), inZeroCrossingLst.clone(), inRelationsLst.clone(), inSamplesLst.clone(), inNumberOfMathFunctions.clone(), eq_count.clone(), -1, inVariables1.clone(), globalKnownVars.clone())?;
            (eres2, countMathFunctions, res, relationsLst, sampleLst) = findZeroCrossings3(e2.clone(), zcs1.clone(), relationsLst.clone(), sampleLst.clone(), countMathFunctions.clone(), eq_count.clone(), -1, inVariables1.clone(), globalKnownVars.clone())?;
            eqnsAccum = cons(Arc::new(BackendDAE::Equation::ARRAY_EQUATION { dimSize: dimsize.clone(), left: eres1.clone(), right: eres2.clone(), source: source.clone(), attr: eqAttr.clone(), recordSize: recordSize.clone() }), inEquationLstAccum.clone());
            (res1, eq_reslst, countMathFunctions, relationsLst, sampleLst) = findZeroCrossings2(inVariables1.clone(), globalKnownVars.clone(), xs.clone(), eq_count.clone(), countMathFunctions.clone(), res.clone(), relationsLst.clone(), sampleLst.clone(), eqnsAccum.clone())?;
            (res1.clone(), eq_reslst.clone(), countMathFunctions.clone(), relationsLst.clone(), sampleLst.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::SOLVED_EQUATION { attr: eqAttr, source: source_, exp: e1, componentRef: cref }, tail: xs } => {
            let mut res: BackendDAE::ZeroCrossingSet;
            let mut res1: BackendDAE::ZeroCrossingSet;
            let mut sampleLst: BackendDAE::ZeroCrossingSet;
            let mut relationsLst: DoubleEnded::MutableList<BackendDAE::ZeroCrossing>;
            let mut countMathFunctions: i32 = 0;
            let mut eq_reslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut eqnsAccum: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut eres1: Arc<DAE::Exp>;
            (eres1, countMathFunctions, res, relationsLst, sampleLst) = findZeroCrossings3(e1.clone(), inZeroCrossingLst.clone(), inRelationsLst.clone(), inSamplesLst.clone(), inNumberOfMathFunctions.clone(), inEqnCount.clone(), -1, inVariables1.clone(), globalKnownVars.clone())?;
            eqnsAccum = cons(Arc::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: cref.clone(), exp: eres1.clone(), source: source_.clone(), attr: eqAttr.clone() }), inEquationLstAccum.clone());
            (res1, eq_reslst, countMathFunctions, relationsLst, sampleLst) = findZeroCrossings2(inVariables1.clone(), globalKnownVars.clone(), xs.clone(), inEqnCount.clone(), countMathFunctions.clone(), res.clone(), relationsLst.clone(), sampleLst.clone(), eqnsAccum.clone())?;
            (res1.clone(), eq_reslst.clone(), countMathFunctions.clone(), relationsLst.clone(), sampleLst.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { attr: eqAttr, source: source_, exp: e1 }, tail: xs } => {
            let mut res: BackendDAE::ZeroCrossingSet;
            let mut res1: BackendDAE::ZeroCrossingSet;
            let mut sampleLst: BackendDAE::ZeroCrossingSet;
            let mut relationsLst: DoubleEnded::MutableList<BackendDAE::ZeroCrossing>;
            let mut eq_count: i32 = 0;
            let mut countMathFunctions: i32 = 0;
            let mut eq_reslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut eqnsAccum: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut eres1: Arc<DAE::Exp>;
            eq_count = inEqnCount.clone() + 1;
            (eres1, countMathFunctions, res, relationsLst, sampleLst) = findZeroCrossings3(e1.clone(), inZeroCrossingLst.clone(), inRelationsLst.clone(), inSamplesLst.clone(), inNumberOfMathFunctions.clone(), eq_count.clone(), -1, inVariables1.clone(), globalKnownVars.clone())?;
            eqnsAccum = cons(Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: eres1.clone(), source: source_.clone(), attr: eqAttr.clone() }), inEquationLstAccum.clone());
            (res1, eq_reslst, countMathFunctions, relationsLst, sampleLst) = findZeroCrossings2(inVariables1.clone(), globalKnownVars.clone(), xs.clone(), eq_count.clone(), countMathFunctions.clone(), res.clone(), relationsLst.clone(), sampleLst.clone(), eqnsAccum.clone())?;
            (res1.clone(), eq_reslst.clone(), countMathFunctions.clone(), relationsLst.clone(), sampleLst.clone())
        },
        Deref @ metamodelica::List::Cons { head: e @ Deref @ BackendDAE::Equation::IF_EQUATION { .. }, tail: xs } => {
            let mut res: BackendDAE::ZeroCrossingSet;
            let mut res1: BackendDAE::ZeroCrossingSet;
            let mut sampleLst: BackendDAE::ZeroCrossingSet;
            let mut relationsLst: DoubleEnded::MutableList<BackendDAE::ZeroCrossing>;
            let mut eq_count: i32 = 0;
            let mut countMathFunctions: i32 = 0;
            let mut eq_reslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut eqnsAccum: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut e = (*e).clone();
            eq_count = inEqnCount.clone() + 1;
            (e, countMathFunctions, res, relationsLst, sampleLst) = findZeroCrossingsIfEqns(e.clone(), inZeroCrossingLst.clone(), inRelationsLst.clone(), inSamplesLst.clone(), inNumberOfMathFunctions.clone(), eq_count.clone(), -1, inVariables1.clone(), globalKnownVars.clone())?;
            eqnsAccum = cons(e.clone(), inEquationLstAccum.clone());
            (res1, eq_reslst, countMathFunctions, relationsLst, sampleLst) = findZeroCrossings2(inVariables1.clone(), globalKnownVars.clone(), xs.clone(), eq_count.clone(), countMathFunctions.clone(), res.clone(), relationsLst.clone(), sampleLst.clone(), eqnsAccum.clone())?;
            (res1.clone(), eq_reslst.clone(), countMathFunctions.clone(), relationsLst.clone(), sampleLst.clone())
        },
        Deref @ metamodelica::List::Cons { head: e, tail: xs } => {
            let mut res1: BackendDAE::ZeroCrossingSet;
            let mut sampleLst: BackendDAE::ZeroCrossingSet;
            let mut relationsLst: DoubleEnded::MutableList<BackendDAE::ZeroCrossing>;
            let mut eq_count: i32 = 0;
            let mut countMathFunctions: i32 = 0;
            let mut eq_reslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut eqnsAccum: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            eq_count = inEqnCount.clone() + 1;
            eqnsAccum = cons(e.clone(), inEquationLstAccum.clone());
            (res1, eq_reslst, countMathFunctions, relationsLst, sampleLst) = findZeroCrossings2(inVariables1.clone(), globalKnownVars.clone(), xs.clone(), eq_count.clone(), inNumberOfMathFunctions.clone(), inZeroCrossingLst.clone(), inRelationsLst.clone(), inSamplesLst.clone(), eqnsAccum.clone())?;
            (res1.clone(), eq_reslst.clone(), countMathFunctions.clone(), relationsLst.clone(), sampleLst.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outZeroCrossingLst, outEquationLst, outNumberOfMathFunctions, outRelationsLst, outSamplesLst))
}

fn findZeroCrossingsWhenEqns(mut inWhenEqn: Arc<BackendDAE::WhenEquation>, mut inZeroCrossings: BackendDAE::ZeroCrossingSet, mut inrelationsinZC: DoubleEnded::MutableList<BackendDAE::ZeroCrossing>, mut inSamplesLst: BackendDAE::ZeroCrossingSet, mut incountMathFunctions: i32, mut counteq: i32, mut countwc: i32, mut vars: BackendDAE::Variables, mut globalKnownVars: BackendDAE::Variables) -> Result<(Arc<BackendDAE::WhenEquation>, i32, BackendDAE::ZeroCrossingSet, DoubleEnded::MutableList<BackendDAE::ZeroCrossing>, BackendDAE::ZeroCrossingSet)> {
    let mut oWhenEqn: Arc<BackendDAE::WhenEquation>;
    let mut outCountMathFunctions: i32 = 0;
    let mut outZeroCrossings: BackendDAE::ZeroCrossingSet;
    let mut outrelationsinZC: DoubleEnded::MutableList<BackendDAE::ZeroCrossing>;
    let mut outSamplesLst: BackendDAE::ZeroCrossingSet;
    (oWhenEqn, outCountMathFunctions, outZeroCrossings, outrelationsinZC, outSamplesLst) = (::match_deref::match_deref! { match &(inWhenEqn.clone()) {
        Deref @ BackendDAE::WhenEquation { elsewhenPart: oweelse, whenStmtLst, condition: cond } => {
            let mut we: Arc<BackendDAE::WhenEquation>;
            let mut zc: BackendDAE::ZeroCrossingSet;
            let mut samples: BackendDAE::ZeroCrossingSet;
            let mut relations: DoubleEnded::MutableList<BackendDAE::ZeroCrossing>;
            let mut countMathFunctions: i32 = 0;
            let mut oweelse = (*oweelse).clone();
            let mut cond = (*cond).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                BackendDump::debugStrExpStr((literal!("processed when condition: ")).clone(), cond.clone(), (literal!("\n")).clone())?;
            }
            (cond, countMathFunctions, zc, relations, samples) = findZeroCrossings3(cond.clone(), inZeroCrossings.clone(), inrelationsinZC.clone(), inSamplesLst.clone(), incountMathFunctions.clone(), counteq.clone(), countwc.clone(), vars.clone(), globalKnownVars.clone())?;
            if isSome(oweelse.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(oweelse.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                we = __pa0.clone();
                (we, countMathFunctions, zc, relations, samples) = findZeroCrossingsWhenEqns(we.clone(), zc.clone(), relations.clone(), samples.clone(), countMathFunctions.clone(), counteq.clone(), countwc.clone(), vars.clone(), globalKnownVars.clone())?;
                oweelse = Some(we.clone());
            } else {
                oweelse = None;
            }
            (Arc::new(BackendDAE::WhenEquation { condition: cond.clone(), whenStmtLst: whenStmtLst.clone(), elsewhenPart: oweelse.clone() }), countMathFunctions.clone(), zc.clone(), relations.clone(), samples.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((oWhenEqn, outCountMathFunctions, outZeroCrossings, outrelationsinZC, outSamplesLst))
}

fn findZeroCrossingsIfEqns(mut inIfEqn: Arc<BackendDAE::Equation>, mut inZeroCrossings: BackendDAE::ZeroCrossingSet, mut inrelationsinZC: DoubleEnded::MutableList<BackendDAE::ZeroCrossing>, mut inSamplesLst: BackendDAE::ZeroCrossingSet, mut incountMathFunctions: i32, mut counteq: i32, mut countwc: i32, mut vars: BackendDAE::Variables, mut globalKnownVars: BackendDAE::Variables) -> Result<(Arc<BackendDAE::Equation>, i32, BackendDAE::ZeroCrossingSet, DoubleEnded::MutableList<BackendDAE::ZeroCrossing>, BackendDAE::ZeroCrossingSet)> {
    let mut outIfEqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut outCountMathFunctions: i32 = 0;
    let mut outZeroCrossings: BackendDAE::ZeroCrossingSet;
    let mut outrelationsinZC: DoubleEnded::MutableList<BackendDAE::ZeroCrossing>;
    let mut outSamplesLst: BackendDAE::ZeroCrossingSet;
    (outIfEqn, outCountMathFunctions, outZeroCrossings, outrelationsinZC, outSamplesLst) = (::match_deref::match_deref! { match &(inIfEqn.clone()) {
        Deref @ BackendDAE::Equation::IF_EQUATION { attr: eqAttr, source: source_, eqnsfalse: elseeqns, eqnstrue: Deref @ metamodelica::List::Nil, conditions: Deref @ metamodelica::List::Nil } => {
            let mut zc: BackendDAE::ZeroCrossingSet;
            let mut samples: BackendDAE::ZeroCrossingSet;
            let mut relations: DoubleEnded::MutableList<BackendDAE::ZeroCrossing>;
            let mut countMathFunctions: i32 = 0;
            let mut elseeqns = (*elseeqns).clone();
            (zc, elseeqns, countMathFunctions, relations, samples) = findZeroCrossings2(vars.clone(), globalKnownVars.clone(), elseeqns.clone(), counteq.clone(), incountMathFunctions.clone(), inZeroCrossings.clone(), inrelationsinZC.clone(), inSamplesLst.clone(), metamodelica::nil())?;
            elseeqns = elseeqns.clone().reverse();
            (Arc::new(BackendDAE::Equation::IF_EQUATION { conditions: metamodelica::nil(), eqnstrue: metamodelica::nil(), eqnsfalse: elseeqns.clone(), source: source_.clone(), attr: eqAttr.clone() }), countMathFunctions.clone(), zc.clone(), relations.clone(), samples.clone())
        },
        Deref @ BackendDAE::Equation::IF_EQUATION { attr: eqAttr, source: source_, eqnsfalse: elseeqns, eqnstrue: Deref @ metamodelica::List::Cons { head: eqnstrue, tail: resteqns }, conditions: Deref @ metamodelica::List::Cons { head: condition, tail: restconditions } } => {
            let mut conditions: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut ifeqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut eqnsTrueLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>> = metamodelica::nil();
            let mut zc: BackendDAE::ZeroCrossingSet;
            let mut samples: BackendDAE::ZeroCrossingSet;
            let mut relations: DoubleEnded::MutableList<BackendDAE::ZeroCrossing>;
            let mut countMathFunctions: i32 = 0;
            let mut source_ = (*source_).clone();
            let mut elseeqns = (*elseeqns).clone();
            let mut eqnstrue = (*eqnstrue).clone();
            let mut condition = (*condition).clone();
            (condition, countMathFunctions, zc, relations, samples) = findZeroCrossings3(condition.clone(), inZeroCrossings.clone(), inrelationsinZC.clone(), inSamplesLst.clone(), incountMathFunctions.clone(), counteq.clone(), countwc.clone(), vars.clone(), globalKnownVars.clone())?;
            (zc, eqnstrue, countMathFunctions, relations, samples) = findZeroCrossings2(vars.clone(), globalKnownVars.clone(), eqnstrue.clone(), counteq.clone(), countMathFunctions.clone(), zc.clone(), relations.clone(), samples.clone(), metamodelica::nil())?;
            eqnstrue = eqnstrue.clone().reverse();
            ifeqn = Arc::new(BackendDAE::Equation::IF_EQUATION { conditions: restconditions.clone(), eqnstrue: resteqns.clone(), eqnsfalse: elseeqns.clone(), source: source_.clone(), attr: eqAttr.clone() });
            let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(findZeroCrossingsIfEqns(ifeqn.clone(), zc.clone(), relations.clone(), samples.clone(), countMathFunctions.clone(), counteq.clone(), countwc.clone(), vars.clone(), globalKnownVars.clone())?) {
                (Deref @ BackendDAE::Equation::IF_EQUATION { source: __pa0, eqnsfalse: __pa1, eqnstrue: __pa2, conditions: __pa3, .. }, __pa4, __pa5, __pa6, __pa7) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
                _ => bail!("pattern mismatch"),
            } };
            source_ = __pa0.clone();
            elseeqns = __pa1.clone();
            eqnsTrueLst = __pa2.clone();
            conditions = __pa3.clone();
            countMathFunctions = __pa4.clone();
            zc = __pa5.clone();
            relations = __pa6.clone();
            samples = __pa7.clone();
            conditions = cons(condition.clone(), conditions.clone());
            eqnsTrueLst = cons(eqnstrue.clone(), eqnsTrueLst.clone());
            (Arc::new(BackendDAE::Equation::IF_EQUATION { conditions: conditions.clone(), eqnstrue: eqnsTrueLst.clone(), eqnsfalse: elseeqns.clone(), source: source_.clone(), attr: eqAttr.clone() }), countMathFunctions.clone(), zc.clone(), relations.clone(), samples.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outIfEqn, outCountMathFunctions, outZeroCrossings, outrelationsinZC, outSamplesLst))
}

fn findZeroCrossingsinJacobians(mut inStrongComponents: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut zeroCrossingLst: BackendDAE::ZeroCrossingSet, mut relationsLst: DoubleEnded::MutableList<BackendDAE::ZeroCrossing>, mut samplesLst: BackendDAE::ZeroCrossingSet, mut allVariables: BackendDAE::Variables, mut globalKnownVars: BackendDAE::Variables) -> Result<Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>> {
    let mut strongComponents: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut outComponent: Arc<BackendDAE::StrongComponent>;
    for mut component in &*inStrongComponents.clone() {
        let mut component = component.clone();
        outComponent = 'mc: {
        let __mc_input = component.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                comp @ Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jac: Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: fullJacobian }, .. } => {
                    let mut comp = (*comp).clone();
                    let mut fullJacobian = (*fullJacobian).clone();
                    fullJacobian = replaceZCExpinFullJacobian(fullJacobian.clone(), zeroCrossingLst.clone(), relationsLst.clone(), samplesLst.clone(), allVariables.clone(), globalKnownVars.clone())?;
                    let __owned_variant_jac_0 = Arc::new(BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: fullJacobian.clone() });
                    if let BackendDAE::StrongComponent::EQUATIONSYSTEM { jac, .. } = &mut comp {
                        *jac = __owned_variant_jac_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than BackendDAE::StrongComponent::EQUATIONSYSTEM"); }
                    Ok(comp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                comp @ Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jac: jacobian @ Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { nonlinearPattern, coloring, sparsePattern, jacobian: Some(symJacobian) }, .. } => {
                    let mut comp = (*comp).clone();
                    let mut symJacobian = (*symJacobian).clone();
                    (symJacobian, _, _, _, _, _) = replaceZCExpinSymJacobian(symJacobian.clone(), zeroCrossingLst.clone(), relationsLst.clone(), samplesLst.clone(), allVariables.clone(), globalKnownVars.clone())?;
                    let __owned_variant_jac_0 = Arc::new(BackendDAE::Jacobian::GENERIC_JACOBIAN { nonlinearPattern: nonlinearPattern.clone(), coloring: coloring.clone(), sparsePattern: sparsePattern.clone(), jacobian: Some(symJacobian.clone()) });
                    if let BackendDAE::StrongComponent::EQUATIONSYSTEM { jac, .. } = &mut comp {
                        *jac = __owned_variant_jac_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than BackendDAE::StrongComponent::EQUATIONSYSTEM"); }
                    Ok(comp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                comp @ Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: tearingSet @ BackendDAE::TearingSet { jac: jacobian @ Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { nonlinearPattern, coloring, sparsePattern, jacobian: Some(symJacobian) }, .. }, .. } => {
                    let mut comp = (*comp).clone();
                    let mut tearingSet = (*tearingSet).clone();
                    let mut symJacobian = (*symJacobian).clone();
                    (symJacobian, _, _, _, _, _) = replaceZCExpinSymJacobian(symJacobian.clone(), zeroCrossingLst.clone(), relationsLst.clone(), samplesLst.clone(), allVariables.clone(), globalKnownVars.clone())?;
                    todo!("unhandled field-assign shape: tearingSet.jac");
                    let __owned_variant_strictTearingSet_0 = tearingSet.clone();
                    if let BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet, .. } = &mut comp {
                        *strictTearingSet = __owned_variant_strictTearingSet_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than BackendDAE::StrongComponent::TORNSYSTEM"); }
                    Ok(comp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(component.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
        strongComponents = cons(outComponent.clone(), strongComponents.clone());
    }
    strongComponents = strongComponents.clone().reverse();
    Ok(strongComponents)
}

fn replaceZCExpinFullJacobian(mut fullJac: Option<Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>>, mut zeroCrossingLst: BackendDAE::ZeroCrossingSet, mut relationsLst: DoubleEnded::MutableList<BackendDAE::ZeroCrossing>, mut samplesLst: BackendDAE::ZeroCrossingSet, mut allVariables: BackendDAE::Variables, mut globalKnownVars: BackendDAE::Variables) -> Result<Option<Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>>> {
    let mut outFullJac: Option<Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>> = None;
    let mut jac: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>> = metamodelica::nil();
    let mut outJac: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>> = metamodelica::nil();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut element: (i32, i32, Arc<BackendDAE::Equation>);
    jac = Util::getOption(fullJac.clone())?;
    for mut element in &*jac.clone() {
        let mut element = element.clone();
        (i, j, eqn) = element.clone();
        let __pa0 = ::match_deref::match_deref! { match &(findZeroCrossings2(allVariables.clone(), globalKnownVars.clone(), list![eqn.clone()], 0, 0, zeroCrossingLst.clone(), relationsLst.clone(), samplesLst.clone(), metamodelica::nil())?) {
            (_, Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, _, _, _) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        eqn = __pa0.clone();
        outJac = cons((i.clone(), j.clone(), eqn.clone()), outJac.clone());
    }
    outJac = outJac.clone().reverse();
    outFullJac = Some(outJac.clone());
    Ok(outFullJac)
}

fn replaceZCExpinSymJacobian(mut symJac: (Arc<BackendDAE::BackendDAE>, ArcStr, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), mut zeroCrossingLst: BackendDAE::ZeroCrossingSet, mut relationsLst: DoubleEnded::MutableList<BackendDAE::ZeroCrossing>, mut samplesLst: BackendDAE::ZeroCrossingSet, mut allVariables: BackendDAE::Variables, mut globalKnownVars: BackendDAE::Variables) -> Result<(Arc<BackendDAE::BackendDAE>, ArcStr, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut outSymJac: (Arc<BackendDAE::BackendDAE>, ArcStr, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>);
    let mut jacBDAE: Arc<BackendDAE::BackendDAE>;
    let mut name: ArcStr = arcstr::literal!("");
    let mut seedVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut tmpVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut resultVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut depCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    (jacBDAE, name, seedVars, tmpVars, resultVars, depCrefs) = symJac.clone();
    jacBDAE = replaceZeroCrossingsJacBackend(jacBDAE.clone(), zeroCrossingLst.clone(), relationsLst.clone(), samplesLst.clone(), allVariables.clone(), globalKnownVars.clone())?;
    outSymJac = (jacBDAE.clone(), name.clone(), seedVars.clone(), tmpVars.clone(), resultVars.clone(), depCrefs.clone());
    Ok(outSymJac)
}

fn replaceZeroCrossingsJacBackend(mut inBackendDAE: Arc<BackendDAE::BackendDAE>, mut zeroCrossingLst: BackendDAE::ZeroCrossingSet, mut relationsLst: DoubleEnded::MutableList<BackendDAE::ZeroCrossing>, mut samplesLst: BackendDAE::ZeroCrossingSet, mut allVariables: BackendDAE::Variables, mut globalKnownVars: BackendDAE::Variables) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outBackendDAE: Arc<BackendDAE::BackendDAE>;
    let mut vars: BackendDAE::Variables;
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut outEqs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut eqs_lst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared>;
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut matching: Arc<BackendDAE::Matching> = Arc::new(BackendDAE::Matching::NO_MATCHING);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inBackendDAE.clone()) {
        Deref @ DAE { UNIQUEIO: __pa0, derivativeNamePrefix: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    eqs = __pa0.clone();
    shared = __pa1.clone();
    for mut system in &*eqs.clone() {
        let mut system = system.clone();
        eqs_lst = BackendEquation::equationList(system.orderedEqs.clone());
        (_, eqs_lst, _, _, _) = findZeroCrossings2(allVariables.clone(), globalKnownVars.clone(), eqs_lst.clone(), 0, 0, zeroCrossingLst.clone(), relationsLst.clone(), samplesLst.clone(), metamodelica::nil())?;
        eqns = BackendEquation::listEquation(eqs_lst.clone().reverse())?;
        assign_field!(system.orderedEqs = eqns.clone());
        let (__pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(system.matching.clone()) {
            Deref @ BackendDAE::Matching::MATCHING { ass2: __pa2, ass1: __pa3, comps: __pa4 } => (__pa2.clone(), __pa3.clone(), __pa4.clone()),
            _ => bail!("pattern mismatch"),
        } };
        ass2 = __pa2.clone();
        ass1 = __pa3.clone();
        comps = __pa4.clone();
        comps = findZeroCrossingsinJacobians(comps.clone(), zeroCrossingLst.clone(), relationsLst.clone(), samplesLst.clone(), allVariables.clone(), globalKnownVars.clone())?;
        matching = Arc::new(BackendDAE::Matching::MATCHING { ass2: ass2.clone(), ass1: ass1.clone(), comps: comps.clone() });
        assign_field!(system.matching = matching.clone());
        outEqs = cons(system.clone(), outEqs.clone());
    }
    outEqs = outEqs.clone().reverse();
    outBackendDAE = BackendDAE::DAE(outEqs.clone(), shared.clone())?;
    Ok(outBackendDAE)
}

fn findZeroCrossings3(mut e: Arc<DAE::Exp>, mut inZeroCrossings: BackendDAE::ZeroCrossingSet, mut inrelationsinZC: DoubleEnded::MutableList<BackendDAE::ZeroCrossing>, mut inSamplesLst: BackendDAE::ZeroCrossingSet, mut incountMathFunctions: i32, mut counteq: i32, mut countwc: i32, mut vars: BackendDAE::Variables, mut globalKnownVars: BackendDAE::Variables) -> Result<(Arc<DAE::Exp>, i32, BackendDAE::ZeroCrossingSet, DoubleEnded::MutableList<BackendDAE::ZeroCrossing>, BackendDAE::ZeroCrossingSet)> {
    let mut eres: Arc<DAE::Exp>;
    let mut outCountMathFunctions: i32 = 0;
    let mut outZeroCrossings: BackendDAE::ZeroCrossingSet;
    let mut outrelationsinZC: DoubleEnded::MutableList<BackendDAE::ZeroCrossing>;
    let mut outSamplesLst: BackendDAE::ZeroCrossingSet;
    if Flags::isSet(Flags::RELIDX.clone())? {
        BackendDump::debugStrExpStr((literal!("start: ")).clone(), e.clone(), (literal!("\n")).clone())?;
    }
    let (__pa0, ((__pa1, __pa2, __pa3, __pa4), _, _)) = Expression::traverseExpTopDown(e.clone(), Arc::new(collectZC), ((inZeroCrossings.clone(), inrelationsinZC.clone(), inSamplesLst.clone(), incountMathFunctions.clone()), (counteq.clone(), vars.clone(), globalKnownVars.clone()), None))?;
    eres = __pa0.clone();
    outZeroCrossings = __pa1.clone();
    outrelationsinZC = __pa2.clone();
    outSamplesLst = __pa3.clone();
    outCountMathFunctions = __pa4.clone();
    Ok((eres, outCountMathFunctions, outZeroCrossings, outrelationsinZC, outSamplesLst))
}

fn collectZC(mut inExp: Arc<DAE::Exp>, mut inTpl: ZCArgType) -> Result<(Arc<DAE::Exp>, bool, ZCArgType)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut outTpl: ZCArgType;
    (outExp, cont, outTpl) = (::match_deref::match_deref! { match &((inExp.clone(), inTpl.clone(), Config::simCodeTarget()?)) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "noEvent" }, .. }, _, _) => {
            (inExp.clone(), false, inTpl.clone())
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "smooth" }, .. }, _, _) => {
            (inExp.clone(), false, inTpl.clone())
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, .. }, ((_, _, samples, _), (eq_count, _, _), iters), _) => {
            let mut zc: BackendDAE::ZeroCrossing;
            zc = createZeroCrossing(inExp.clone(), list![eq_count.clone()], iters.clone());
            mergeZeroCrossings(zc.clone(), samples.clone())?;
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("sample index: ")); __mm_s.push_str(&*intString(ZeroCrossings::length(samples.clone())?)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            (inExp.clone(), true, inTpl.clone())
        },
        (outExp @ Deref @ DAE::Exp::REDUCTION { .. }, ((zeroCrossings, relations, samples, numMathFunctions), tp1, _), _) => {
            let mut e: Arc<DAE::Exp>;
            let mut iters: Option<Arc<metamodelica::List<BackendDAE::SimIterator>>> = None;
            let mut outExp = (*outExp).clone();
            let mut zeroCrossings = (*zeroCrossings).clone();
            let mut relations = (*relations).clone();
            let mut samples = (*samples).clone();
            let mut numMathFunctions = (*numMathFunctions).clone();
            let mut tp1 = (*tp1).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("collectZC searching in: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            iters = Some({
        let mut __acc: Arc<metamodelica::List<BackendDAE::SimIterator>> = metamodelica::nil();
        for mut iter in (var_field!((*outExp).iterators, DAE::Exp::REDUCTION).clone()).into_iter().cloned() {
            let __x = createIterator(iter.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            let (__pa0, ((__pa1, __pa2, __pa3, __pa4), __pa5, _)) = Expression::traverseExpTopDown(var_field!((*outExp).expr, DAE::Exp::REDUCTION).clone(), Arc::new(collectZC), ((zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone()))?;
            e = __pa0.clone();
            zeroCrossings = __pa1.clone();
            relations = __pa2.clone();
            samples = __pa3.clone();
            numMathFunctions = __pa4.clone();
            tp1 = __pa5.clone();
            assign_variant_field!(outExp => DAE::Exp::REDUCTION; expr = e.clone());
            (outExp.clone(), false, ((zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), None))
        },
        (Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: index, tail: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Cons { head: delay, tail: Deref @ metamodelica::List::Cons { head: delayMax, tail: Deref @ metamodelica::List::Nil } } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "delay" } }, ((zeroCrossings, relations, samples, numMathFunctions), tp1 @ (eq_count, _, _), iters), Deref @ "C") => {
            let mut e_1: Arc<DAE::Exp>;
            let mut eres: Arc<DAE::Exp>;
            let mut eres1: Arc<DAE::Exp>;
            let mut itmp: i32 = 0;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut e = (*e).clone();
            let mut delay = (*delay).clone();
            let mut zeroCrossings = (*zeroCrossings).clone();
            let mut relations = (*relations).clone();
            let mut samples = (*samples).clone();
            let mut numMathFunctions = (*numMathFunctions).clone();
            let mut tp1 = (*tp1).clone();
            let mut iters = (*iters).clone();
            let (__pa0, ((_, __pa1, __pa2, __pa3), __pa4, __pa5)) = Expression::traverseExpTopDown(e.clone(), Arc::new(collectZC), ((ZeroCrossings::new()?, relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone()))?;
            e = __pa0.clone();
            relations = __pa1.clone();
            samples = __pa2.clone();
            numMathFunctions = __pa3.clone();
            tp1 = __pa4.clone();
            iters = __pa5.clone();
            let (__pa6, ((_, __pa7, __pa8, __pa9), __pa10, __pa11)) = Expression::traverseExpTopDown(delay.clone(), Arc::new(collectZC), ((ZeroCrossings::new()?, relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone()))?;
            delay = __pa6.clone();
            relations = __pa7.clone();
            samples = __pa8.clone();
            numMathFunctions = __pa9.clone();
            tp1 = __pa10.clone();
            iters = __pa11.clone();
            eres1 = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("delayZeroCrossing")).clone() }), expLst: list![index.clone(), Arc::new(DAE::Exp::ICONST { integer: DoubleEnded::length(relations.clone()) }), delay.clone()], attr: attr.clone() });
            e_1 = Arc::new(DAE::Exp::RELATION { exp1: eres1.clone(), operator: DAE::Operator::GREATER { ty: DAE::T_REAL_DEFAULT.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), index: DoubleEnded::length(relations.clone()), optionExpisASUB: None });
            zc = createZeroCrossing(eres1.clone(), list![eq_count.clone()], iters.clone());
            (eres, relations, _) = zcIndexRelation(e_1.clone(), relations.clone(), DoubleEnded::length(relations.clone()), zc.clone())?;
            zc = createZeroCrossing(eres.clone(), list![eq_count.clone()], iters.clone());
            let (__pa12, __pa13) = ::match_deref::match_deref! { match &(zcIndex(eres.clone(), zeroCrossings.clone(), DoubleEnded::length(relations.clone()), zc.clone())?) {
                (Deref @ DAE::Exp::RELATION { index: __pa12, .. }, __pa13, _) => (__pa12.clone(), __pa13.clone()),
                _ => bail!("pattern mismatch"),
            } };
            itmp = __pa12.clone();
            zeroCrossings = __pa13.clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("collectZC result zc: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eres.clone())?); __mm_s.push_str(&*literal!(" index: ")); __mm_s.push_str(&*intString(itmp.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            (Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("delay")).clone() }), expLst: list![index.clone(), e.clone(), delay.clone(), delayMax.clone()], attr: attr.clone() }), true, ((zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone()))
        },
        (Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: index, tail: Deref @ metamodelica::List::Cons { head: in0, tail: Deref @ metamodelica::List::Cons { head: in1, tail: Deref @ metamodelica::List::Cons { head: x, tail: Deref @ metamodelica::List::Cons { head: dir, tail: Deref @ metamodelica::List::Cons { head: initPnts, tail: Deref @ metamodelica::List::Cons { head: initVals, tail: Deref @ metamodelica::List::Nil } } } } } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "spatialDistribution" } }, ((zeroCrossings, relations, samples, numMathFunctions), tp1 @ (eq_count, _, _), iters), Deref @ "C") => {
            let mut e_1: Arc<DAE::Exp>;
            let mut eres: Arc<DAE::Exp>;
            let mut eres1: Arc<DAE::Exp>;
            let mut itmp: i32 = 0;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut in0 = (*in0).clone();
            let mut in1 = (*in1).clone();
            let mut x = (*x).clone();
            let mut dir = (*dir).clone();
            let mut zeroCrossings = (*zeroCrossings).clone();
            let mut relations = (*relations).clone();
            let mut samples = (*samples).clone();
            let mut numMathFunctions = (*numMathFunctions).clone();
            let mut tp1 = (*tp1).clone();
            let mut eq_count = (*eq_count).clone();
            let mut iters = (*iters).clone();
            let (__pa0, ((_, __pa1, __pa2, __pa3), __pa4, __pa5)) = Expression::traverseExpTopDown(in0.clone(), Arc::new(collectZC), ((ZeroCrossings::new()?, relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone()))?;
            in0 = __pa0.clone();
            relations = __pa1.clone();
            samples = __pa2.clone();
            numMathFunctions = __pa3.clone();
            tp1 = __pa4.clone();
            iters = __pa5.clone();
            let (__pa6, ((_, __pa7, __pa8, __pa9), __pa10, __pa11)) = Expression::traverseExpTopDown(in1.clone(), Arc::new(collectZC), ((ZeroCrossings::new()?, relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone()))?;
            in1 = __pa6.clone();
            relations = __pa7.clone();
            samples = __pa8.clone();
            numMathFunctions = __pa9.clone();
            tp1 = __pa10.clone();
            iters = __pa11.clone();
            let (__pa12, ((_, __pa13, __pa14, __pa15), __pa16, __pa17)) = Expression::traverseExpTopDown(x.clone(), Arc::new(collectZC), ((ZeroCrossings::new()?, relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone()))?;
            x = __pa12.clone();
            relations = __pa13.clone();
            samples = __pa14.clone();
            numMathFunctions = __pa15.clone();
            tp1 = __pa16.clone();
            iters = __pa17.clone();
            let (__pa18, ((_, __pa19, __pa20, __pa21), ref __pa23 @ (ref __pa22, _, _), __pa24)) = Expression::traverseExpTopDown(dir.clone(), Arc::new(collectZC), ((ZeroCrossings::new()?, relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone()))?;
            dir = __pa18.clone();
            relations = __pa19.clone();
            samples = __pa20.clone();
            numMathFunctions = __pa21.clone();
            eq_count = __pa22.clone();
            tp1 = __pa23.clone();
            iters = __pa24.clone();
            eres1 = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("spatialDistributionZeroCrossing")).clone() }), expLst: list![index.clone(), Arc::new(DAE::Exp::ICONST { integer: DoubleEnded::length(relations.clone()) }), x.clone(), dir.clone()], attr: attr.clone() });
            e_1 = Arc::new(DAE::Exp::RELATION { exp1: eres1.clone(), operator: DAE::Operator::GREATER { ty: DAE::T_REAL_DEFAULT.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), index: DoubleEnded::length(relations.clone()), optionExpisASUB: None });
            zc = createZeroCrossing(eres1.clone(), list![eq_count.clone()], iters.clone());
            (eres, relations, _) = zcIndexRelation(e_1.clone(), relations.clone(), DoubleEnded::length(relations.clone()), zc.clone())?;
            zc = createZeroCrossing(eres.clone(), list![eq_count.clone()], iters.clone());
            let (__pa25, __pa26) = ::match_deref::match_deref! { match &(zcIndex(eres.clone(), zeroCrossings.clone(), DoubleEnded::length(relations.clone()), zc.clone())?) {
                (Deref @ DAE::Exp::RELATION { index: __pa25, .. }, __pa26, _) => (__pa25.clone(), __pa26.clone()),
                _ => bail!("pattern mismatch"),
            } };
            itmp = __pa25.clone();
            zeroCrossings = __pa26.clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("collectZC result zc: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eres.clone())?); __mm_s.push_str(&*literal!(" index: ")); __mm_s.push_str(&*intString(itmp.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            (Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("spatialDistribution")).clone() }), expLst: list![index.clone(), in0.clone(), in1.clone(), x.clone(), dir.clone(), initPnts.clone(), initVals.clone()], attr: attr.clone() }), true, ((zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone()))
        },
        (Deref @ DAE::Exp::LUNARY { exp: e1, .. }, ((_, relations, _, _), (_, vars, globalKnownVars), _), _) if (!(BackendDAEUtil::hasExpContinuousParts(e1.clone(), vars.clone(), globalKnownVars.clone())?)) => {
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("discrete LUNARY: ")); __mm_s.push_str(&*intString(DoubleEnded::length(relations.clone()))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            (inExp.clone(), true, inTpl.clone())
        },
        (Deref @ DAE::Exp::LBINARY { exp2: e2, exp1: e1, .. }, ((_, relations, _, _), (_, vars, globalKnownVars), _), _) if (!(BackendDAEUtil::hasExpContinuousParts(e1.clone(), vars.clone(), globalKnownVars.clone())? || BackendDAEUtil::hasExpContinuousParts(e2.clone(), vars.clone(), globalKnownVars.clone())?)) => {
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("discrete LBINARY: ")); __mm_s.push_str(&*intString(DoubleEnded::length(relations.clone()))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            (inExp.clone(), true, inTpl.clone())
        },
        (Deref @ DAE::Exp::LUNARY { operator: op, exp: e1 }, ((zeroCrossings, relations, _, _), _, iters), _) => {
            let mut e_1: Arc<DAE::Exp>;
            let mut eq_count: i32 = 0;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut tpl: ZCArgType;
            let mut empty: bool = false;
            let mut e1 = (*e1).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("continues LUNARY: ")); __mm_s.push_str(&*intString(DoubleEnded::length(relations.clone()))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            let (__pa0, ref __pa2 @ ((_, _, _, _), (ref __pa1, _, _), _)) = Expression::traverseExpTopDown(e1.clone(), Arc::new(collectZC), inTpl.clone())?;
            e1 = __pa0.clone();
            eq_count = __pa1.clone();
            tpl = __pa2.clone();
            e_1 = Arc::new(DAE::Exp::LUNARY { operator: op.clone(), exp: e1.clone() });
            zc = createZeroCrossing(e_1.clone(), list![eq_count.clone()], iters.clone());
            empty = !(ZeroCrossings::contains(zeroCrossings.clone(), zc.clone())?);
            if empty.clone() {
                ZeroCrossings::add(zeroCrossings.clone(), zc.clone())?;
            }
            if Flags::isSet(Flags::RELIDX.clone())? {
                BackendDump::debugExpStr(e_1.clone(), (literal!("\n")).clone())?;
            }
            (e_1.clone(), false, if (empty.clone()) {tpl.clone()} else {inTpl.clone()})
        },
        (Deref @ DAE::Exp::LBINARY { exp2: e2, operator: op, exp1: e1 }, ((zeroCrossings, relations, samples, numMathFunctions), tp1, iters), _) => {
            let mut e_1: Arc<DAE::Exp>;
            let mut e_2: Arc<DAE::Exp>;
            let mut eq_count: i32 = 0;
            let mut oldNumRelations: i32 = 0;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut empty: bool = false;
            let mut relations = (*relations).clone();
            let mut samples = (*samples).clone();
            let mut numMathFunctions = (*numMathFunctions).clone();
            let mut tp1 = (*tp1).clone();
            let mut iters = (*iters).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("continues LBINARY: ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", DoubleEnded::length(relations.clone())))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                BackendDump::debugExpStr(inExp.clone(), (literal!("\n")).clone())?;
            }
            oldNumRelations = DoubleEnded::length(relations.clone());
            let (__pa0, ((_, __pa1, __pa2, __pa3), __pa4, __pa5)) = Expression::traverseExpTopDown(e1.clone(), Arc::new(collectZC), ((ZeroCrossings::new()?, relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone()))?;
            e_1 = __pa0.clone();
            relations = __pa1.clone();
            samples = __pa2.clone();
            numMathFunctions = __pa3.clone();
            tp1 = __pa4.clone();
            iters = __pa5.clone();
            let (__pa6, ((_, __pa7, __pa8, __pa9), ref __pa11 @ (ref __pa10, _, _), __pa12)) = Expression::traverseExpTopDown(e2.clone(), Arc::new(collectZC), ((ZeroCrossings::new()?, relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone()))?;
            e_2 = __pa6.clone();
            relations = __pa7.clone();
            samples = __pa8.clone();
            numMathFunctions = __pa9.clone();
            eq_count = __pa10.clone();
            tp1 = __pa11.clone();
            iters = __pa12.clone();
            if intGt(DoubleEnded::length(relations.clone()), oldNumRelations.clone()) {
                e_1 = Arc::new(DAE::Exp::LBINARY { exp1: e_1.clone(), operator: op.clone(), exp2: e_2.clone() });
                zc = createZeroCrossing(e_1.clone(), list![eq_count.clone()], iters.clone());
                empty = !(ZeroCrossings::contains(zeroCrossings.clone(), zc.clone())?);
                cont = false;
                if empty.clone() {
                    ZeroCrossings::add(zeroCrossings.clone(), zc.clone())?;
                }
                if Flags::isSet(Flags::RELIDX.clone())? {
                    BackendDump::dumpZeroCrossingList(ZeroCrossings::toList(zeroCrossings.clone()), (literal!("LBINARY")).clone())?;
                }
            } else {
                empty = true;
                cont = true;
            }
            (if (cont.clone()) {inExp.clone()} else {e_1.clone()}, cont.clone(), if (!(cont.clone()) && empty.clone()) {((zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone())} else {inTpl.clone()})
        },
        (Deref @ DAE::Exp::RELATION { exp2: e2, exp1: e1, .. }, ((_, relations, _, _), (_, vars, globalKnownVars), _), _) if (!(BackendDAEUtil::hasExpContinuousParts(e1.clone(), vars.clone(), globalKnownVars.clone())? || BackendDAEUtil::hasExpContinuousParts(e2.clone(), vars.clone(), globalKnownVars.clone())?)) => {
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("discrete RELATION: ")); __mm_s.push_str(&*intString(DoubleEnded::length(relations.clone()))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            (inExp.clone(), true, inTpl.clone())
        },
        (Deref @ DAE::Exp::RELATION { exp2: e2, operator: op, exp1: e1, .. }, ((zeroCrossings, relations, samples, numMathFunctions), tp1 @ (eq_count, _, _), iters), _) if (Flags::isSet(Flags::EVENTS.clone())?) => {
            let mut e_1: Arc<DAE::Exp>;
            let mut eres: Arc<DAE::Exp>;
            let mut itmp: i32 = 0;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut zeroCrossings = (*zeroCrossings).clone();
            let mut relations = (*relations).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("start collectZC (2): ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" numRelations: ")); __mm_s.push_str(&*intString(DoubleEnded::length(relations.clone()))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            e_1 = Arc::new(DAE::Exp::RELATION { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone(), index: DoubleEnded::length(relations.clone()), optionExpisASUB: None });
            zc = createZeroCrossing(e_1.clone(), list![eq_count.clone()], iters.clone());
            (eres, relations, _) = zcIndexRelation(e_1.clone(), relations.clone(), DoubleEnded::length(relations.clone()), zc.clone())?;
            zc = createZeroCrossing(eres.clone(), list![eq_count.clone()], iters.clone());
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(zcIndex(eres.clone(), zeroCrossings.clone(), DoubleEnded::length(relations.clone()), zc.clone())?) {
                (Deref @ DAE::Exp::RELATION { index: __pa0, .. }, __pa1, _) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            itmp = __pa0.clone();
            zeroCrossings = __pa1.clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("collectZC result zc: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eres.clone())?); __mm_s.push_str(&*literal!(" index: ")); __mm_s.push_str(&*intString(itmp.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            (eres.clone(), true, ((zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone()))
        },
        (Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "integer" } }, ((zeroCrossings, relations, samples, numMathFunctions), tp1 @ (eq_count, _, _), iters), _) if (Flags::isSet(Flags::EVENTS.clone())?) => {
            let mut e_1: Arc<DAE::Exp>;
            let mut eres: Arc<DAE::Exp>;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut zeroCrossings = (*zeroCrossings).clone();
            let mut numMathFunctions = (*numMathFunctions).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("start collectZC: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" numMathFunctions: ")); __mm_s.push_str(&*intString(numMathFunctions.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            e_1 = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("integer")).clone() }), expLst: list![e1.clone(), Arc::new(DAE::Exp::ICONST { integer: numMathFunctions.clone() })], attr: attr.clone() });
            zc = createZeroCrossing(e_1.clone(), list![eq_count.clone()], iters.clone());
            (eres, zeroCrossings, numMathFunctions) = zcIndex(e_1.clone(), zeroCrossings.clone(), numMathFunctions.clone(), zc.clone())?;
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("collectZC result zc: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eres.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            (eres.clone(), true, ((zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone()))
        },
        (Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "floor" } }, ((zeroCrossings, relations, samples, numMathFunctions), tp1 @ (eq_count, _, _), iters), _) if (Flags::isSet(Flags::EVENTS.clone())?) => {
            let mut e_1: Arc<DAE::Exp>;
            let mut eres: Arc<DAE::Exp>;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut zeroCrossings = (*zeroCrossings).clone();
            let mut numMathFunctions = (*numMathFunctions).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("start collectZC: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" numMathFunctions: ")); __mm_s.push_str(&*intString(numMathFunctions.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            e_1 = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("floor")).clone() }), expLst: list![e1.clone(), Arc::new(DAE::Exp::ICONST { integer: numMathFunctions.clone() })], attr: attr.clone() });
            zc = createZeroCrossing(e_1.clone(), list![eq_count.clone()], iters.clone());
            (eres, zeroCrossings, numMathFunctions) = zcIndex(e_1.clone(), zeroCrossings.clone(), numMathFunctions.clone(), zc.clone())?;
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("collectZC result zc: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eres.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            (eres.clone(), true, ((zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone()))
        },
        (Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "ceil" } }, ((zeroCrossings, relations, samples, numMathFunctions), tp1 @ (eq_count, _, _), iters), _) if (Flags::isSet(Flags::EVENTS.clone())?) => {
            let mut e_1: Arc<DAE::Exp>;
            let mut eres: Arc<DAE::Exp>;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut zeroCrossings = (*zeroCrossings).clone();
            let mut numMathFunctions = (*numMathFunctions).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("start collectZC: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" numMathFunctions: ")); __mm_s.push_str(&*intString(numMathFunctions.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            e_1 = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("ceil")).clone() }), expLst: list![e1.clone(), Arc::new(DAE::Exp::ICONST { integer: numMathFunctions.clone() })], attr: attr.clone() });
            zc = createZeroCrossing(e_1.clone(), list![eq_count.clone()], iters.clone());
            (eres, zeroCrossings, numMathFunctions) = zcIndex(e_1.clone(), zeroCrossings.clone(), numMathFunctions.clone(), zc.clone())?;
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("collectZC result zc: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eres.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            (eres.clone(), true, ((zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone()))
        },
        (Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "div" } }, ((zeroCrossings, relations, samples, numMathFunctions), tp1 @ (eq_count, _, _), iters), _) if (Flags::isSet(Flags::EVENTS.clone())?) => {
            let mut e_1: Arc<DAE::Exp>;
            let mut eres: Arc<DAE::Exp>;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut zeroCrossings = (*zeroCrossings).clone();
            let mut numMathFunctions = (*numMathFunctions).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("start collectZC: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" numMathFunctions: ")); __mm_s.push_str(&*intString(numMathFunctions.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            e_1 = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("div")).clone() }), expLst: list![e1.clone(), e2.clone(), Arc::new(DAE::Exp::ICONST { integer: numMathFunctions.clone() })], attr: attr.clone() });
            zc = createZeroCrossing(e_1.clone(), list![eq_count.clone()], iters.clone());
            (eres, zeroCrossings, numMathFunctions) = zcIndex(e_1.clone(), zeroCrossings.clone(), numMathFunctions.clone(), zc.clone())?;
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("collectZC result zc: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eres.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            (eres.clone(), true, ((zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone()))
        },
        (Deref @ DAE::Exp::CALL { attr: attr @ Deref @ DAE::CallAttributes { .. }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "mod" } }, ((zeroCrossings, relations, samples, numMathFunctions), tp1 @ (eq_count, _, _), iters), _) if (Flags::isSet(Flags::EVENTS.clone())?) => {
            let mut e_1: Arc<DAE::Exp>;
            let mut eres: Arc<DAE::Exp>;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut zeroCrossings = (*zeroCrossings).clone();
            let mut numMathFunctions = (*numMathFunctions).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("start collectZC: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" numMathFunctions: ")); __mm_s.push_str(&*intString(numMathFunctions.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            e_1 = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("mod")).clone() }), expLst: list![e1.clone(), e2.clone(), Arc::new(DAE::Exp::ICONST { integer: numMathFunctions.clone() })], attr: attr.clone() });
            zc = createZeroCrossing(e_1.clone(), list![eq_count.clone()], iters.clone());
            (eres, zeroCrossings, numMathFunctions) = zcIndex(e_1.clone(), zeroCrossings.clone(), numMathFunctions.clone(), zc.clone())?;
            numMathFunctions = numMathFunctions.clone() + 1;
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("collectZC result zc: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eres.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            (eres.clone(), true, ((zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone()))
        },
        (Deref @ DAE::Exp::CALL { attr: attr @ Deref @ DAE::CallAttributes { ty, .. }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "rem" } }, ((zeroCrossings, relations, samples, numMathFunctions), tp1 @ (eq_count, _, _), iters), _) if (Flags::isSet(Flags::EVENTS.clone())?) => {
            let mut e_1: Arc<DAE::Exp>;
            let mut e_2: Arc<DAE::Exp>;
            let mut eres: Arc<DAE::Exp>;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut zeroCrossings = (*zeroCrossings).clone();
            let mut numMathFunctions = (*numMathFunctions).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("start collectZC: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" numMathFunctions: ")); __mm_s.push_str(&*intString(numMathFunctions.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            e_1 = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("div")).clone() }), expLst: list![e1.clone(), e2.clone(), Arc::new(DAE::Exp::ICONST { integer: numMathFunctions.clone() })], attr: attr.clone() });
            zc = createZeroCrossing(e_1.clone(), list![eq_count.clone()], iters.clone());
            (eres, zeroCrossings, numMathFunctions) = zcIndex(e_1.clone(), zeroCrossings.clone(), numMathFunctions.clone(), zc.clone())?;
            e_2 = Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::SUB { ty: ty.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: eres.clone(), operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: e2.clone() }) });
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("collectZC result zc: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eres.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            (e_2.clone(), true, ((zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone(), iters.clone()))
        },
        _ => {
            (inExp.clone(), true, inTpl.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outTpl))
}

fn collectZCAlgsFor(mut inExp: Arc<DAE::Exp>, mut inTpl: ForArgType) -> Result<(Arc<DAE::Exp>, bool, ForArgType)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut outTpl: ForArgType;
    (outExp, cont, outTpl) = (::match_deref::match_deref! { match &((inExp.clone(), inTpl.clone())) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "noEvent" }, .. }, _) => {
            (inExp.clone(), false, inTpl.clone())
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "smooth" }, .. }, _) => {
            (inExp.clone(), false, inTpl.clone())
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, .. }, (_, _, _, (_, _, samples, _), (alg_indx, _, _))) => {
            let mut eqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut zc: BackendDAE::ZeroCrossing;
            eqs = list![alg_indx.clone()];
            zc = createZeroCrossing(inExp.clone(), eqs.clone(), None);
            ZeroCrossings::add(samples.clone(), zc.clone())?;
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("sample index algotihm: ")); __mm_s.push_str(&*intString(alg_indx.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            (inExp.clone(), true, inTpl.clone())
        },
        (Deref @ DAE::Exp::LUNARY { exp: e1, .. }, (_, _, _, _, (_, vars, globalKnownVars))) if (!(BackendDAEUtil::hasExpContinuousParts(e1.clone(), vars.clone(), globalKnownVars.clone())?)) => {
            (inExp.clone(), true, inTpl.clone())
        },
        (Deref @ DAE::Exp::LUNARY { operator: op, exp: e1 }, (iterator, _, Deref @ DAE::Exp::RANGE { .. }, (zeroCrossings, relations, _, _), _)) if (Expression::expContains(inExp.clone(), iterator.clone())?) => {
            let mut e_1: Arc<DAE::Exp>;
            let mut inExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut explst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut zc_lst: Arc<metamodelica::List<BackendDAE::ZeroCrossing>> = metamodelica::nil();
            let mut alg_indx: i32 = 0;
            let mut tpl: ForArgType;
            let mut e1 = (*e1).clone();
            let mut iterator = (*iterator).clone();
            let mut relations = (*relations).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("continues LUNARY with Iterator: ")); __mm_s.push_str(&*intString(DoubleEnded::length(relations.clone()))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            let (__pa0, ref __pa5 @ (ref __pa1, ref __pa2, _, (_, ref __pa3, _, _), (ref __pa4, _, _))) = Expression::traverseExpTopDown(e1.clone(), Arc::new(collectZCAlgsFor), inTpl.clone())?;
            e1 = __pa0.clone();
            iterator = __pa1.clone();
            inExpLst = __pa2.clone();
            relations = __pa3.clone();
            alg_indx = __pa4.clone();
            tpl = __pa5.clone();
            e_1 = Arc::new(DAE::Exp::LUNARY { operator: op.clone(), exp: e1.clone() });
            (explst, _) = replaceIteratorWithStaticValues(e_1.clone(), iterator.clone(), inExpLst.clone(), DoubleEnded::length(relations.clone()))?;
            zc_lst = createZeroCrossings(explst.clone(), list![alg_indx.clone()]);
            ZeroCrossings::add_list(zeroCrossings.clone(), zc_lst.clone())?;
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", (literal!("collectZCAlgsFor LUNARY with Iterator result zc: ")).clone());
                BackendDump::debugExpStr(e_1.clone(), (literal!("\n")).clone())?;
            }
            (e_1.clone(), false, tpl.clone())
        },
        (Deref @ DAE::Exp::LUNARY { operator: op, exp: e1 }, (_, _, _, (zeroCrossings, relations, _, _), _)) => {
            let mut e_1: Arc<DAE::Exp>;
            let mut alg_indx: i32 = 0;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut tpl: ForArgType;
            let mut e1 = (*e1).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("continues LUNARY: ")); __mm_s.push_str(&*intString(DoubleEnded::length(relations.clone()))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            let (__pa0, ref __pa2 @ (_, _, _, (_, _, _, _), (ref __pa1, _, _))) = Expression::traverseExpTopDown(e1.clone(), Arc::new(collectZCAlgsFor), inTpl.clone())?;
            e1 = __pa0.clone();
            alg_indx = __pa1.clone();
            tpl = __pa2.clone();
            e_1 = Arc::new(DAE::Exp::LUNARY { operator: op.clone(), exp: e1.clone() });
            zc = createZeroCrossing(e_1.clone(), list![alg_indx.clone()], None);
            ZeroCrossings::add(zeroCrossings.clone(), zc.clone())?;
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", (literal!("collectZCAlgsFor LUNARY result zc: ")).clone());
                BackendDump::debugExpStr(e_1.clone(), (literal!("\n")).clone())?;
            }
            (e_1.clone(), false, tpl.clone())
        },
        (Deref @ DAE::Exp::LBINARY { exp2: e2, exp1: e1, .. }, (_, _, _, _, (_, vars, globalKnownVars))) if (!(BackendDAEUtil::hasExpContinuousParts(e1.clone(), vars.clone(), globalKnownVars.clone())? || BackendDAEUtil::hasExpContinuousParts(e2.clone(), vars.clone(), globalKnownVars.clone())?)) => {
            (inExp.clone(), true, inTpl.clone())
        },
        (Deref @ DAE::Exp::LBINARY { exp2: e2, operator: op, exp1: e1 }, (iterator, inExpLst, range, (zeroCrossings, relations, samples, numMathFunctions), tp1)) => {
            let mut e_1: Arc<DAE::Exp>;
            let mut e_2: Arc<DAE::Exp>;
            let mut explst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut zc_lst: Arc<metamodelica::List<BackendDAE::ZeroCrossing>> = metamodelica::nil();
            let mut alg_indx: i32 = 0;
            let mut oldNumRelations: i32 = 0;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut tpl: ForArgType;
            let mut tp2: (BackendDAE::ZeroCrossingSet, DoubleEnded::MutableList<BackendDAE::ZeroCrossing>, BackendDAE::ZeroCrossingSet, i32);
            let mut inExpLst = (*inExpLst).clone();
            let mut range = (*range).clone();
            let mut relations = (*relations).clone();
            let mut samples = (*samples).clone();
            let mut numMathFunctions = (*numMathFunctions).clone();
            let mut tp1 = (*tp1).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("continues LBINARY: ")); __mm_s.push_str(&*intString(DoubleEnded::length(relations.clone()))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                BackendDump::debugExpStr(inExp.clone(), (literal!("\n")).clone())?;
            }
            oldNumRelations = DoubleEnded::length(relations.clone());
            let (__pa0, (_, __pa1, __pa2, __pa3, __pa4)) = Expression::traverseExpTopDown(e1.clone(), Arc::new(collectZCAlgsFor), (iterator.clone(), inExpLst.clone(), range.clone(), (ZeroCrossings::new()?, relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone()))?;
            e_1 = __pa0.clone();
            inExpLst = __pa1.clone();
            range = __pa2.clone();
            tp2 = __pa3.clone();
            tp1 = __pa4.clone();
            let (__pa5, (_, __pa6, __pa7, (_, __pa8, __pa9, __pa10), ref __pa12 @ (ref __pa11, _, _))) = Expression::traverseExpTopDown(e2.clone(), Arc::new(collectZCAlgsFor), (iterator.clone(), inExpLst.clone(), range.clone(), tp2.clone(), tp1.clone()))?;
            e_2 = __pa5.clone();
            inExpLst = __pa6.clone();
            range = __pa7.clone();
            relations = __pa8.clone();
            samples = __pa9.clone();
            numMathFunctions = __pa10.clone();
            alg_indx = __pa11.clone();
            tp1 = __pa12.clone();
            if intGt(DoubleEnded::length(relations.clone()), oldNumRelations.clone()) {
                e_1 = Arc::new(DAE::Exp::LBINARY { exp1: e_1.clone(), operator: op.clone(), exp2: e_2.clone() });
                if Expression::expContains(e1.clone(), iterator.clone())? || Expression::expContains(e2.clone(), iterator.clone())? {
                    (explst, _) = replaceIteratorWithStaticValues(e_1.clone(), iterator.clone(), inExpLst.clone(), DoubleEnded::length(relations.clone()))?;
                    zc_lst = createZeroCrossings(explst.clone(), list![alg_indx.clone()]);
                    ZeroCrossings::add_list(zeroCrossings.clone(), zc_lst.clone())?;
                    if Flags::isSet(Flags::RELIDX.clone())? {
                        BackendDump::dumpZeroCrossingList(ZeroCrossings::toList(zeroCrossings.clone()), (literal!("collectZCAlgsFor LBINARY1 result zc")).clone())?;
                    }
                } else {
                    zc = createZeroCrossing(e_1.clone(), list![alg_indx.clone()], None);
                    if !(ZeroCrossings::contains(zeroCrossings.clone(), zc.clone())?) {
                        ZeroCrossings::add(zeroCrossings.clone(), zc.clone())?;
                    }
                    if Flags::isSet(Flags::RELIDX.clone())? {
                        BackendDump::dumpZeroCrossingList(ZeroCrossings::toList(zeroCrossings.clone()), (literal!("collectZCAlgsFor LBINARY2 result zc")).clone())?;
                    }
                }
                cont = false;
                tpl = (iterator.clone(), inExpLst.clone(), range.clone(), (zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone());
            } else {
                e_1 = inExp.clone();
                cont = true;
                tpl = inTpl.clone();
            }
            (e_1.clone(), cont.clone(), tpl.clone())
        },
        (Deref @ DAE::Exp::RELATION { exp2: e2, exp1: e1, .. }, (_, _, _, _, (_, vars, globalKnownVars))) if (!(BackendDAEUtil::hasExpContinuousParts(e1.clone(), vars.clone(), globalKnownVars.clone())? || BackendDAEUtil::hasExpContinuousParts(e2.clone(), vars.clone(), globalKnownVars.clone())?)) => {
            (inExp.clone(), true, inTpl.clone())
        },
        (Deref @ DAE::Exp::RELATION { exp2: e2, operator: op, exp1: e1, .. }, (iterator, inExpLst, range @ Deref @ DAE::Exp::RANGE { step: stepvalueopt, start: startvalue, .. }, (zeroCrossings, relations, samples, numMathFunctions), tp1 @ (alg_indx, _, globalKnownVars))) if (if (Flags::isSet(Flags::EVENTS.clone())?) {if (Expression::expContains(e1.clone(), iterator.clone())?) {true} else {Expression::expContains(e2.clone(), iterator.clone())?}} else {false}) => {
            let mut eres: Arc<DAE::Exp>;
            let mut explst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut zcLstNew: Arc<metamodelica::List<BackendDAE::ZeroCrossing>> = metamodelica::nil();
            let mut itmp: i32 = 0;
            let mut stepvalue: Arc<DAE::Exp>;
            let mut istart: i32 = 0;
            let mut istep: i32 = 0;
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" number of relations: ")); __mm_s.push_str(&*intString(DoubleEnded::length(relations.clone()))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            stepvalue = Util::getOptionOrDefault(stepvalueopt.clone(), Arc::new(DAE::Exp::ICONST { integer: 1 }));
            istart = BackendDAEUtil::expInt(startvalue.clone(), globalKnownVars.clone())?;
            istep = BackendDAEUtil::expInt(stepvalue.clone(), globalKnownVars.clone())?;
            eres = Arc::new(DAE::Exp::RELATION { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone(), index: DoubleEnded::length(relations.clone()), optionExpisASUB: Some((iterator.clone(), istart.clone(), istep.clone())) });
            (explst, itmp) = replaceIteratorWithStaticValues(inExp.clone(), iterator.clone(), inExpLst.clone(), DoubleEnded::length(relations.clone()))?;
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" number of new zc (1): ")); __mm_s.push_str(&*intString((explst.clone().len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            zcLstNew = createZeroCrossings(explst.clone(), list![alg_indx.clone()]);
            DoubleEnded::push_list_back(relations.clone(), zcLstNew.clone());
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" number of new zc (2): ")); __mm_s.push_str(&*intString(DoubleEnded::length(relations.clone()))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            itmp = (zcLstNew.clone().len() as i32);
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" itmp: ")); __mm_s.push_str(&*intString(itmp.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            ZeroCrossings::add_list(zeroCrossings.clone(), zcLstNew.clone())?;
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("collectZCAlgsFor result zc: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eres.clone())?); __mm_s.push_str(&*literal!(" index:")); __mm_s.push_str(&*intString(DoubleEnded::length(relations.clone()))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            (eres.clone(), true, (iterator.clone(), inExpLst.clone(), range.clone(), (zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone()))
        },
        (Deref @ DAE::Exp::RELATION { exp2: e2, operator: op, exp1: e1, .. }, (iterator, inExpLst, range, (zeroCrossings, relations, samples, numMathFunctions), tp1 @ (alg_indx, _, _))) if (Flags::isSet(Flags::EVENTS.clone())?) => {
            let mut eres: Arc<DAE::Exp>;
            let mut zc: BackendDAE::ZeroCrossing;
            eres = Arc::new(DAE::Exp::RELATION { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone(), index: DoubleEnded::length(relations.clone()), optionExpisASUB: None });
            zc = createZeroCrossing(eres.clone(), list![alg_indx.clone()], None);
            DoubleEnded::push_back(relations.clone(), zc.clone());
            ZeroCrossings::add(zeroCrossings.clone(), zc.clone())?;
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("collectZCAlgsFor result zc: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eres.clone())?); __mm_s.push_str(&*literal!(" index:")); __mm_s.push_str(&*intString(DoubleEnded::length(relations.clone()))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            (eres.clone(), true, (iterator.clone(), inExpLst.clone(), range.clone(), (zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone()))
        },
        (Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "integer" } }, (iterator, le, range, (zeroCrossings, relations, samples, numMathFunctions), tp1 @ (alg_indx, _, _))) if (Flags::isSet(Flags::EVENTS.clone())?) => {
            let mut e_1: Arc<DAE::Exp>;
            let mut eres: Arc<DAE::Exp>;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut zeroCrossings = (*zeroCrossings).clone();
            let mut numMathFunctions = (*numMathFunctions).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("start collectZC: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" numMathFunctions: ")); __mm_s.push_str(&*intString(numMathFunctions.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            e_1 = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("integer")).clone() }), expLst: list![e1.clone(), Arc::new(DAE::Exp::ICONST { integer: numMathFunctions.clone() })], attr: attr.clone() });
            zc = createZeroCrossing(e_1.clone(), list![alg_indx.clone()], None);
            (eres, zeroCrossings, numMathFunctions) = zcIndex(e_1.clone(), zeroCrossings.clone(), numMathFunctions.clone(), zc.clone())?;
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("collectZC result zc: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eres.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            (eres.clone(), true, (iterator.clone(), le.clone(), range.clone(), (zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone()))
        },
        (Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "floor" } }, (iterator, le, range, (zeroCrossings, relations, samples, numMathFunctions), tp1 @ (alg_indx, _, _))) if (Flags::isSet(Flags::EVENTS.clone())?) => {
            let mut e_1: Arc<DAE::Exp>;
            let mut eres: Arc<DAE::Exp>;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut zeroCrossings = (*zeroCrossings).clone();
            let mut numMathFunctions = (*numMathFunctions).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("start collectZC: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" numMathFunctions: ")); __mm_s.push_str(&*intString(numMathFunctions.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            e_1 = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("floor")).clone() }), expLst: list![e1.clone(), Arc::new(DAE::Exp::ICONST { integer: numMathFunctions.clone() })], attr: attr.clone() });
            zc = createZeroCrossing(e_1.clone(), list![alg_indx.clone()], None);
            (eres, zeroCrossings, numMathFunctions) = zcIndex(e_1.clone(), zeroCrossings.clone(), numMathFunctions.clone(), zc.clone())?;
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("collectZC result zc: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eres.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            (eres.clone(), true, (iterator.clone(), le.clone(), range.clone(), (zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone()))
        },
        (Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "ceil" } }, (iterator, le, range, (zeroCrossings, relations, samples, numMathFunctions), tp1 @ (alg_indx, _, _))) if (Flags::isSet(Flags::EVENTS.clone())?) => {
            let mut e_1: Arc<DAE::Exp>;
            let mut eres: Arc<DAE::Exp>;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut zeroCrossings = (*zeroCrossings).clone();
            let mut numMathFunctions = (*numMathFunctions).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("start collectZC: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" numMathFunctions: ")); __mm_s.push_str(&*intString(numMathFunctions.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            e_1 = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("ceil")).clone() }), expLst: list![e1.clone(), Arc::new(DAE::Exp::ICONST { integer: numMathFunctions.clone() })], attr: attr.clone() });
            zc = createZeroCrossing(e_1.clone(), list![alg_indx.clone()], None);
            (eres, zeroCrossings, numMathFunctions) = zcIndex(e_1.clone(), zeroCrossings.clone(), numMathFunctions.clone(), zc.clone())?;
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("collectZC result zc: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eres.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            (eres.clone(), true, (iterator.clone(), le.clone(), range.clone(), (zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone()))
        },
        (Deref @ DAE::Exp::CALL { attr, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "div" } }, (iterator, le, range, (zeroCrossings, relations, samples, numMathFunctions), tp1 @ (alg_indx, _, _))) if (Flags::isSet(Flags::EVENTS.clone())?) => {
            let mut e_1: Arc<DAE::Exp>;
            let mut eres: Arc<DAE::Exp>;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut zeroCrossings = (*zeroCrossings).clone();
            let mut numMathFunctions = (*numMathFunctions).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("start collectZC: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" numMathFunctions: ")); __mm_s.push_str(&*intString(numMathFunctions.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            e_1 = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("div")).clone() }), expLst: list![e1.clone(), e2.clone(), Arc::new(DAE::Exp::ICONST { integer: numMathFunctions.clone() })], attr: attr.clone() });
            zc = createZeroCrossing(e_1.clone(), list![alg_indx.clone()], None);
            (eres, zeroCrossings, numMathFunctions) = zcIndex(e_1.clone(), zeroCrossings.clone(), numMathFunctions.clone(), zc.clone())?;
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("collectZC result zc: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eres.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            (eres.clone(), true, (iterator.clone(), le.clone(), range.clone(), (zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone()))
        },
        (Deref @ DAE::Exp::CALL { attr: attr @ Deref @ DAE::CallAttributes { .. }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "mod" } }, (iterator, le, range, (zeroCrossings, relations, samples, numMathFunctions), tp1 @ (alg_indx, _, _))) if (Flags::isSet(Flags::EVENTS.clone())?) => {
            let mut e_1: Arc<DAE::Exp>;
            let mut eres: Arc<DAE::Exp>;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut zeroCrossings = (*zeroCrossings).clone();
            let mut numMathFunctions = (*numMathFunctions).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("start collectZC: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" numMathFunctions: ")); __mm_s.push_str(&*intString(numMathFunctions.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            e_1 = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("mod")).clone() }), expLst: list![e1.clone(), e2.clone(), Arc::new(DAE::Exp::ICONST { integer: numMathFunctions.clone() })], attr: attr.clone() });
            zc = createZeroCrossing(e_1.clone(), list![alg_indx.clone()], None);
            (eres, zeroCrossings, numMathFunctions) = zcIndex(e_1.clone(), zeroCrossings.clone(), numMathFunctions.clone(), zc.clone())?;
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("collectZC result zc: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eres.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            (eres.clone(), true, (iterator.clone(), le.clone(), range.clone(), (zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone()))
        },
        (Deref @ DAE::Exp::CALL { attr: attr @ Deref @ DAE::CallAttributes { ty, .. }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "rem" } }, (iterator, le, range, (zeroCrossings, relations, samples, numMathFunctions), tp1 @ (alg_indx, _, _))) if (Flags::isSet(Flags::EVENTS.clone())?) => {
            let mut e_1: Arc<DAE::Exp>;
            let mut e_2: Arc<DAE::Exp>;
            let mut eres: Arc<DAE::Exp>;
            let mut zc: BackendDAE::ZeroCrossing;
            let mut zeroCrossings = (*zeroCrossings).clone();
            let mut numMathFunctions = (*numMathFunctions).clone();
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("start collectZC: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" numMathFunctions: ")); __mm_s.push_str(&*intString(numMathFunctions.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            e_1 = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("div")).clone() }), expLst: list![e1.clone(), e2.clone(), Arc::new(DAE::Exp::ICONST { integer: numMathFunctions.clone() })], attr: attr.clone() });
            zc = createZeroCrossing(e_1.clone(), list![alg_indx.clone()], None);
            (eres, zeroCrossings, numMathFunctions) = zcIndex(e_1.clone(), zeroCrossings.clone(), numMathFunctions.clone(), zc.clone())?;
            e_2 = Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::SUB { ty: ty.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: eres.clone(), operator: DAE::Operator::MUL { ty: ty.clone() }, exp2: e2.clone() }) });
            if Flags::isSet(Flags::RELIDX.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("collectZC result zc: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(eres.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            (e_2.clone(), true, (iterator.clone(), le.clone(), range.clone(), (zeroCrossings.clone(), relations.clone(), samples.clone(), numMathFunctions.clone()), tp1.clone()))
        },
        _ => {
            (inExp.clone(), true, inTpl.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outTpl))
}

fn replaceIteratorWithStaticValues(mut inExp: Arc<DAE::Exp>, mut inIterator: Arc<DAE::Exp>, mut inExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inIndex: i32) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, i32)> {
    let mut outZeroCrossings: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut outIndex: i32 = 0;
    (outZeroCrossings, outIndex) = (::match_deref::match_deref! { match &((inExp.clone(), inExpLst.clone())) {
        (_, Deref @ metamodelica::List::Nil) => {
            (metamodelica::nil(), inIndex.clone())
        },
        (Deref @ DAE::Exp::RELATION { exp2: e2, operator: op, exp1: e1, .. }, Deref @ metamodelica::List::Cons { head: e, tail: rest }) => {
            let mut res1: Arc<DAE::Exp>;
            let mut e_1: Arc<DAE::Exp>;
            let mut res2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut index: i32 = 0;
            e_1 = Arc::new(DAE::Exp::RELATION { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone(), index: inIndex.clone(), optionExpisASUB: None });
            (res1, _) = Expression::replaceExpTpl(e_1.clone(), (inIterator.clone(), e.clone()))?;
            (res2, index) = replaceIteratorWithStaticValues(inExp.clone(), inIterator.clone(), rest.clone(), inIndex.clone() + 1)?;
            res2 = cons(res1.clone(), res2.clone());
            (res2.clone(), index.clone())
        },
        (Deref @ DAE::Exp::LUNARY { operator: op, exp: e1 }, Deref @ metamodelica::List::Cons { head: e, tail: rest }) => {
            let mut res1: Arc<DAE::Exp>;
            let mut e_1: Arc<DAE::Exp>;
            let mut res2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut index: i32 = 0;
            e_1 = Arc::new(DAE::Exp::LUNARY { operator: op.clone(), exp: e1.clone() });
            (res1, _) = Expression::replaceExpTpl(e_1.clone(), (inIterator.clone(), e.clone()))?;
            (res2, index) = replaceIteratorWithStaticValues(inExp.clone(), inIterator.clone(), rest.clone(), inIndex.clone() + 1)?;
            res2 = cons(res1.clone(), res2.clone());
            (res2.clone(), index.clone())
        },
        (Deref @ DAE::Exp::LBINARY { exp2: e2, operator: op, exp1: e1 }, Deref @ metamodelica::List::Cons { head: e, tail: rest }) => {
            let mut res1: Arc<DAE::Exp>;
            let mut e_1: Arc<DAE::Exp>;
            let mut res2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut index: i32 = 0;
            e_1 = Arc::new(DAE::Exp::LBINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone() });
            (res1, _) = Expression::replaceExpTpl(e_1.clone(), (inIterator.clone(), e.clone()))?;
            (res2, index) = replaceIteratorWithStaticValues(inExp.clone(), inIterator.clone(), rest.clone(), inIndex.clone() + 1)?;
            res2 = cons(res1.clone(), res2.clone());
            (res2.clone(), index.clone())
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FindZeroCrossings.replaceIteratorWithStaticValues")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outZeroCrossings, outIndex))
}

fn zcIndex(mut relation: Arc<DAE::Exp>, mut zeroCrossings: BackendDAE::ZeroCrossingSet, mut index: i32, mut zc: BackendDAE::ZeroCrossing) -> Result<(Arc<DAE::Exp>, BackendDAE::ZeroCrossingSet, i32)> {
    let mut relation: Arc<DAE::Exp> = relation;
    let mut zeroCrossings: BackendDAE::ZeroCrossingSet = zeroCrossings;
    let mut index: i32 = index;
    let mut duplicate: Arc<metamodelica::List<BackendDAE::ZeroCrossing>> = metamodelica::nil();
    if ZeroCrossings::contains(zeroCrossings.clone(), zc.clone())? {
        let BackendDAE::ZERO_CROSSING { relation_: __pa0, .. } = (ZeroCrossings::get(zeroCrossings.clone(), zc.clone())?) else { bail!("pattern mismatch") };
        relation = __pa0.clone();
        return Ok((relation, zeroCrossings, index));
    }
    (relation, index) = (::match_deref::match_deref! { match &(relation.clone()) {
        Deref @ DAE::Exp::RELATION { .. } => {
            ZeroCrossings::add(zeroCrossings.clone(), zc.clone())?;
            (relation.clone(), index.clone() + 1)
        },
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, .. } => {
            ZeroCrossings::add(zeroCrossings.clone(), zc.clone())?;
            (relation.clone(), index.clone() + 1)
        },
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }, .. } => {
            ZeroCrossings::add(zeroCrossings.clone(), zc.clone())?;
            (relation.clone(), index.clone() + 2)
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FindZeroCrossings.zcIndex")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(relation.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((relation, zeroCrossings, index))
}

fn zcIndexRelation(mut relation: Arc<DAE::Exp>, mut zeroCrossings: DoubleEnded::MutableList<BackendDAE::ZeroCrossing>, mut index: i32, mut zc: BackendDAE::ZeroCrossing) -> Result<(Arc<DAE::Exp>, DoubleEnded::MutableList<BackendDAE::ZeroCrossing>, i32)> {
    let mut relation: Arc<DAE::Exp> = relation;
    let mut zeroCrossings: DoubleEnded::MutableList<BackendDAE::ZeroCrossing> = zeroCrossings;
    let mut index: i32 = index;
    let mut duplicate: Arc<metamodelica::List<BackendDAE::ZeroCrossing>> = metamodelica::nil();
    duplicate = List::select1(DoubleEnded::toListNoCopyNoClear(zeroCrossings.clone()), Arc::new(ZeroCrossings::equals), zc.clone());
    (relation, index) = (::match_deref::match_deref! { match &((relation.clone(), duplicate.clone())) {
        (Deref @ DAE::Exp::RELATION { .. }, Deref @ metamodelica::List::Nil) => {
            DoubleEnded::push_back(zeroCrossings.clone(), zc.clone());
            (relation.clone(), index.clone() + 1)
        },
        (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, .. }, Deref @ metamodelica::List::Nil) => {
            DoubleEnded::push_back(zeroCrossings.clone(), zc.clone());
            (relation.clone(), index.clone() + 1)
        },
        (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }, .. }, Deref @ metamodelica::List::Nil) => {
            DoubleEnded::push_back(zeroCrossings.clone(), zc.clone());
            (relation.clone(), index.clone() + 2)
        },
        (_, Deref @ metamodelica::List::Cons { head: BackendDAE::ZeroCrossing { relation_: rel, .. }, tail: _ }) => {
            (rel.clone(), index.clone())
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FindZeroCrossings.zcIndexRelation")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(relation.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((relation, zeroCrossings, index))
}

fn mergeZeroCrossings(mut newZc: BackendDAE::ZeroCrossing, mut zcs: BackendDAE::ZeroCrossingSet) -> Result<()> {
    let mut matches: i32 = 0;
    let mut samezc: Arc<metamodelica::List<BackendDAE::ZeroCrossing>> = metamodelica::nil();
    let mut diff: Arc<metamodelica::List<BackendDAE::ZeroCrossing>> = metamodelica::nil();
    let mut zc1: BackendDAE::ZeroCrossing;
    let mut same_1: BackendDAE::ZeroCrossing;
    if !(ZeroCrossings::contains(zcs.clone(), newZc.clone())?) {
        ZeroCrossings::add(zcs.clone(), newZc.clone())?;
    } else {
        DoubleEnded::mapNoCopy_1(zcs.zc.clone(), Arc::new(mergeZeroCrossingIfEqual), newZc.clone())?;
    }
    Ok(())
}

fn mergeZeroCrossingIfEqual(mut zc1: BackendDAE::ZeroCrossing, mut zc2: BackendDAE::ZeroCrossing) -> Result<BackendDAE::ZeroCrossing> {
    let mut zc: BackendDAE::ZeroCrossing;
    zc = if (ZeroCrossings::equals(zc1.clone(), zc2.clone())?) {mergeZeroCrossing(zc1.clone(), zc2.clone())?} else {zc1.clone()};
    Ok(zc)
}

fn mergeZeroCrossing(mut inZeroCrossing1: BackendDAE::ZeroCrossing, mut inZeroCrossing2: BackendDAE::ZeroCrossing) -> Result<BackendDAE::ZeroCrossing> {
    let mut outZeroCrossing: BackendDAE::ZeroCrossing;
    let mut eq: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eq1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eq2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut e1: Arc<DAE::Exp>;
    let mut e2: Arc<DAE::Exp>;
    let mut res: Arc<DAE::Exp>;
    let BackendDAE::ZERO_CROSSING { occurEquLst: __pa0, relation_: __pa1, .. } = (inZeroCrossing1.clone()) else { bail!("pattern mismatch") };
    eq1 = __pa0.clone();
    e1 = __pa1.clone();
    let BackendDAE::ZERO_CROSSING { occurEquLst: __pa2, relation_: __pa3, .. } = (inZeroCrossing2.clone()) else { bail!("pattern mismatch") };
    eq2 = __pa2.clone();
    e2 = __pa3.clone();
    res = getMinZeroCrossings(e1.clone(), e2.clone())?;
    eq = List::union(eq1.clone(), eq2.clone());
    outZeroCrossing = BackendDAE::ZeroCrossing { index: 0, relation_: res.clone(), occurEquLst: eq.clone(), iter: None };
    Ok(outZeroCrossing)
}

fn getMinZeroCrossings(mut inZCexp1: Arc<DAE::Exp>, mut inZCexp2: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outMinZC: Arc<DAE::Exp>;
    outMinZC = (::match_deref::match_deref! { match &((inZCexp1.clone(), inZCexp2.clone())) {
        (Deref @ DAE::Exp::RELATION { index: index1, .. }, Deref @ DAE::Exp::RELATION { index: index2, .. }) => {
            let mut res: Arc<DAE::Exp>;
            res = if (index1.clone() < index2.clone()) {inZCexp1.clone()} else {inZCexp2.clone()};
            res.clone()
        },
        (Deref @ DAE::Exp::LUNARY { exp: e1, operator: op }, Deref @ DAE::Exp::LUNARY { exp: e2, .. }) => {
            let mut res: Arc<DAE::Exp>;
            res = getMinZeroCrossings(e1.clone(), e2.clone())?;
            Arc::new(DAE::Exp::LUNARY { operator: op.clone(), exp: res.clone() })
        },
        (Deref @ DAE::Exp::LBINARY { exp2: e2, operator: op, exp1: e1 }, Deref @ DAE::Exp::LBINARY { exp2: e4, exp1: e3, .. }) => {
            let mut res: Arc<DAE::Exp>;
            let mut res2: Arc<DAE::Exp>;
            res = getMinZeroCrossings(e1.clone(), e2.clone())?;
            res2 = getMinZeroCrossings(e3.clone(), e4.clone())?;
            Arc::new(DAE::Exp::LBINARY { exp1: res.clone(), operator: op.clone(), exp2: res2.clone() })
        },
        (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, .. }, _) => {
            inZCexp1.clone()
        },
        (_, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, .. }) => {
            inZCexp2.clone()
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FindZeroCrossings.getMinZeroCrossings")); __mm_s.push_str(&*literal!(" failed for {")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inZCexp1.clone())?); __mm_s.push_str(&*literal!("} and {")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inZCexp2.clone())?); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outMinZC)
}

fn traverseStmtsExps(mut inStmts: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inExtraArg: ForArgType, mut inKnvars: BackendDAE::Variables) -> Result<(Arc<metamodelica::List<Arc<DAE::Statement>>>, ForArgType)> {
    let mut slist: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut extraArg: ForArgType = inExtraArg.clone();
    let mut e_1: Arc<DAE::Exp>;
    let mut e_2: Arc<DAE::Exp>;
    let mut e: Arc<DAE::Exp>;
    let mut e2: Arc<DAE::Exp>;
    let mut iteratorExp: Arc<DAE::Exp>;
    let mut expl1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut expl2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut iteratorexps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut cr_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut stmts2: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut x: Arc<DAE::Statement>;
    let mut ew: Arc<DAE::Statement>;
    let mut ew_1: Arc<DAE::Statement>;
    let mut b1: bool = false;
    let mut id1: ArcStr = arcstr::literal!("");
    let mut source: Arc<DAE::ElementSource>;
    let mut algElse: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
    let mut loopPrlVars: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, SourceInfo)>> = metamodelica::nil();
    let mut conditions: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut initialCall: bool = false;
    for mut stmt in &*inStmts.clone() {
        let mut stmt = stmt.clone();
        (stmt, extraArg) = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ DAE::Statement::STMT_ASSIGN { source, exp: e, exp1: e2, type_: tp } => {
            (e_1, extraArg) = Expression::traverseExpTopDown(e.clone(), Arc::new(collectZCAlgsFor), extraArg.clone())?;
            (e_2, extraArg) = Expression::traverseExpTopDown(e2.clone(), Arc::new(collectZCAlgsFor), extraArg.clone())?;
            (Arc::new(DAE::Statement::STMT_ASSIGN { type_: tp.clone(), exp1: e_2.clone(), exp: e_1.clone(), source: source.clone() }), extraArg.clone())
        },
        Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { source, exp: e, expExpLst: expl1, type_: tp } => {
            (e_1, extraArg) = Expression::traverseExpTopDown(e.clone(), Arc::new(collectZCAlgsFor), extraArg.clone())?;
            (expl2, extraArg) = Expression::traverseExpListTopDown(expl1.clone(), Arc::new(collectZCAlgsFor), extraArg.clone())?;
            (Arc::new(DAE::Statement::STMT_TUPLE_ASSIGN { type_: tp.clone(), expExpLst: expl2.clone(), exp: e_1.clone(), source: source.clone() }), extraArg.clone())
        },
        Deref @ DAE::Statement::STMT_ASSIGN_ARR { source, exp: e, lhs: e2, type_: tp } => {
            (e_1, extraArg) = Expression::traverseExpTopDown(e.clone(), Arc::new(collectZCAlgsFor), extraArg.clone())?;
            (e_2, _, extraArg) = collectZCAlgsFor(e2.clone(), extraArg.clone())?;
            (Arc::new(DAE::Statement::STMT_ASSIGN_ARR { type_: tp.clone(), lhs: e_2.clone(), exp: e_1.clone(), source: source.clone() }), extraArg.clone())
        },
        x @ Deref @ DAE::Statement::STMT_ASSIGN_ARR { source, exp: e, lhs: e2, type_: tp } => {
            (e_1, extraArg) = Expression::traverseExpTopDown(e.clone(), Arc::new(collectZCAlgsFor), extraArg.clone())?;
            if '__try0: {
                (e_2, _, _) = unwrap_break_err!(collectZCAlgsFor(e2.clone(), extraArg.clone()), '__try0);
                Ok::<(), anyhow::Error>(())
            }.is_ok() { bail!("failure(): body succeeded") }
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            println!("{}", (DAEDump::ppStatementStr(x.clone())?).clone());
            println!("{}", (literal!("Warning, not allowed to set the componentRef to a expression in FindZeroCrossings.traverseStmtsExps for ZeroCrosssing\n")).clone());
            (Arc::new(DAE::Statement::STMT_ASSIGN_ARR { type_: tp.clone(), lhs: e_2.clone(), exp: e_1.clone(), source: source.clone() }), extraArg.clone())
        },
        Deref @ DAE::Statement::STMT_IF { source, else_: algElse, statementLst: stmts, exp: e } => {
            let mut algElse = (*algElse).clone();
            (algElse, extraArg) = traverseStmtsElseExps(algElse.clone(), extraArg.clone(), inKnvars.clone())?;
            (stmts2, extraArg) = traverseStmtsExps(stmts.clone(), extraArg.clone(), inKnvars.clone())?;
            (e_1, extraArg) = Expression::traverseExpTopDown(e.clone(), Arc::new(collectZCAlgsFor), extraArg.clone())?;
            (Arc::new(DAE::Statement::STMT_IF { exp: e_1.clone(), statementLst: stmts2.clone(), else_: algElse.clone(), source: source.clone() }), extraArg.clone())
        },
        Deref @ DAE::Statement::STMT_FOR { source, statementLst: stmts, range: e, iter: id1, iterIsArray: b1, type_: tp } => {
            cr = ComponentReferenceBasics::makeCrefIdent((id1.clone()).clone(), tp.clone(), metamodelica::nil());
            iteratorExp = Expression::crefExp(cr.clone())?;
            iteratorexps = BackendDAEUtil::extendRange(e.clone(), inKnvars.clone())?;
            (stmts2, extraArg) = traverseStmtsForExps(iteratorExp.clone(), iteratorexps.clone(), e.clone(), stmts.clone(), inKnvars.clone(), extraArg.clone())?;
            (Arc::new(DAE::Statement::STMT_FOR { type_: tp.clone(), iterIsArray: b1.clone(), iter: (id1.clone()).clone(), range: e.clone(), statementLst: stmts2.clone(), source: source.clone() }), extraArg.clone())
        },
        Deref @ DAE::Statement::STMT_PARFOR { source, loopPrlVars, statementLst: stmts, range: e, iter: id1, iterIsArray: b1, type_: tp } => {
            cr = ComponentReferenceBasics::makeCrefIdent((id1.clone()).clone(), tp.clone(), metamodelica::nil());
            iteratorExp = Expression::crefExp(cr.clone())?;
            iteratorexps = BackendDAEUtil::extendRange(e.clone(), inKnvars.clone())?;
            (stmts2, extraArg) = traverseStmtsForExps(iteratorExp.clone(), iteratorexps.clone(), e.clone(), stmts.clone(), inKnvars.clone(), extraArg.clone())?;
            (Arc::new(DAE::Statement::STMT_PARFOR { type_: tp.clone(), iterIsArray: b1.clone(), iter: (id1.clone()).clone(), range: e.clone(), statementLst: stmts2.clone(), loopPrlVars: loopPrlVars.clone(), source: source.clone() }), extraArg.clone())
        },
        Deref @ DAE::Statement::STMT_WHILE { source, statementLst: stmts, exp: e } => {
            (stmts2, extraArg) = traverseStmtsExps(stmts.clone(), extraArg.clone(), inKnvars.clone())?;
            (e_1, extraArg) = Expression::traverseExpTopDown(e.clone(), Arc::new(collectZCAlgsFor), extraArg.clone())?;
            (Arc::new(DAE::Statement::STMT_WHILE { exp: e_1.clone(), statementLst: stmts2.clone(), source: source.clone() }), extraArg.clone())
        },
        Deref @ DAE::Statement::STMT_WHEN { source, elseWhen: None, statementLst: stmts, initialCall, conditions, exp: e } => {
            (e_1, extraArg) = Expression::traverseExpTopDown(e.clone(), Arc::new(collectZCAlgsFor), extraArg.clone())?;
            (Arc::new(DAE::Statement::STMT_WHEN { exp: e_1.clone(), conditions: conditions.clone(), initialCall: initialCall.clone(), statementLst: stmts.clone(), elseWhen: None, source: source.clone() }), extraArg.clone())
        },
        Deref @ DAE::Statement::STMT_WHEN { source, elseWhen: Some(ew), statementLst: stmts, initialCall, conditions, exp: e } => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(traverseStmtsExps(list![ew.clone()], extraArg.clone(), inKnvars.clone())?) {
                (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, __pa1) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            ew_1 = __pa0.clone();
            extraArg = __pa1.clone();
            (e_1, extraArg) = Expression::traverseExpTopDown(e.clone(), Arc::new(collectZCAlgsFor), extraArg.clone())?;
            (Arc::new(DAE::Statement::STMT_WHEN { exp: e_1.clone(), conditions: conditions.clone(), initialCall: initialCall.clone(), statementLst: stmts.clone(), elseWhen: Some(ew_1.clone()), source: source.clone() }), extraArg.clone())
        },
        Deref @ DAE::Statement::STMT_ASSERT { .. } => (stmt.clone(), extraArg.clone()),
        Deref @ DAE::Statement::STMT_TERMINATE { .. } => (stmt.clone(), extraArg.clone()),
        Deref @ DAE::Statement::STMT_REINIT { .. } => (stmt.clone(), extraArg.clone()),
        Deref @ DAE::Statement::STMT_NORETCALL { source, exp: e } => {
            (e_1, extraArg) = Expression::traverseExpTopDown(e.clone(), Arc::new(collectZCAlgsFor), extraArg.clone())?;
            (Arc::new(DAE::Statement::STMT_NORETCALL { exp: e_1.clone(), source: source.clone() }), extraArg.clone())
        },
        Deref @ DAE::Statement::STMT_RETURN { .. } => (stmt.clone(), extraArg.clone()),
        Deref @ DAE::Statement::STMT_BREAK { .. } => (stmt.clone(), extraArg.clone()),
        Deref @ DAE::Statement::STMT_FAILURE { source, body: stmts } => {
            (stmts2, extraArg) = traverseStmtsExps(stmts.clone(), extraArg.clone(), inKnvars.clone())?;
            (Arc::new(DAE::Statement::STMT_FAILURE { body: stmts2.clone(), source: source.clone() }), extraArg.clone())
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FindZeroCrossings.traverseStmtsExps")); __mm_s.push_str(&*literal!(" failed: ")); __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        slist = cons(stmt.clone(), slist.clone());
    }
    slist = slist.clone().reverse();
    Ok((slist, extraArg))
}

fn traverseStmtsElseExps(mut inElse: Arc<DAE::Else>, mut inExtraArg: ForArgType, mut inKnvars: BackendDAE::Variables) -> Result<(Arc<DAE::Else>, ForArgType)> {
    let mut outElse: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
    let mut outTplStmtTypeA: ForArgType;
    (outElse, outTplStmtTypeA) = (::match_deref::match_deref! { match &(inElse.clone()) {
        Deref @ DAE::Else::NOELSE => {
            (Arc::new(openmodelica_frontend_types::DAE::Else::NOELSE), inExtraArg.clone())
        },
        Deref @ DAE::Else::ELSEIF { exp: e, statementLst: st, else_: el } => {
            let mut e_1: Arc<DAE::Exp>;
            let mut st_1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut el_1: Arc<DAE::Else> = Arc::new(DAE::Else::NOELSE);
            let mut extraArg: ForArgType;
            (el_1, extraArg) = traverseStmtsElseExps(el.clone(), inExtraArg.clone(), inKnvars.clone())?;
            (st_1, extraArg) = traverseStmtsExps(st.clone(), extraArg.clone(), inKnvars.clone())?;
            (e_1, extraArg) = Expression::traverseExpTopDown(e.clone(), Arc::new(collectZCAlgsFor), extraArg.clone())?;
            (Arc::new(DAE::Else::ELSEIF { exp: e_1.clone(), statementLst: st_1.clone(), else_: el_1.clone() }), extraArg.clone())
        },
        Deref @ DAE::Else::ELSE { statementLst: st } => {
            let mut st_1: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut extraArg: ForArgType;
            (st_1, extraArg) = traverseStmtsExps(st.clone(), inExtraArg.clone(), inKnvars.clone())?;
            (Arc::new(DAE::Else::ELSE { statementLst: st_1.clone() }), extraArg.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outElse, outTplStmtTypeA))
}

fn traverseStmtsForExps(mut inIteratorExp: Arc<DAE::Exp>, mut inExplst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inRange: Arc<DAE::Exp>, mut inStmts: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inKnvars: BackendDAE::Variables, mut inExtraArg: ForArgType) -> Result<(Arc<metamodelica::List<Arc<DAE::Statement>>>, ForArgType)> {
    let mut outStatements: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut outTpl: ForArgType;
    (outStatements, outTpl) = (::match_deref::match_deref! { match &((inExplst.clone(), inExtraArg.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            (inStmts.clone(), inExtraArg.clone())
        },
        (_, (_, _, _, tpl2, tpl3)) => {
            let mut statementLst: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut extraArg: ForArgType;
            (statementLst, extraArg) = traverseStmtsExps(inStmts.clone(), (inIteratorExp.clone(), inExplst.clone(), inRange.clone(), tpl2.clone(), tpl3.clone()), inKnvars.clone())?;
            (statementLst.clone(), extraArg.clone())
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FindZeroCrossings.traverseStmtsForExps")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outStatements, outTpl))
}

fn createZeroCrossings(mut inExpExpLst1: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inOccurEquLst: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<BackendDAE::ZeroCrossing>> {
    let mut outZeroCrossingLst: Arc<metamodelica::List<BackendDAE::ZeroCrossing>> = metamodelica::nil();
    outZeroCrossingLst = List::map1(inExpExpLst1.clone(), Arc::new({ let __pe_b2 = None; move |__pe_a0, __pe_a1| Ok(createZeroCrossing(__pe_a0, __pe_a1, __pe_b2.clone())) }), inOccurEquLst.clone());
    outZeroCrossingLst
}

fn createZeroCrossing(mut inRelation: Arc<DAE::Exp>, mut inOccurEquLst: Arc<metamodelica::List<i32>>, mut iters: Option<Arc<metamodelica::List<BackendDAE::SimIterator>>>) -> BackendDAE::ZeroCrossing {
    let mut outZeroCrossing: BackendDAE::ZeroCrossing;
    outZeroCrossing = (::match_deref::match_deref! { match &(inOccurEquLst.clone()) {
        Deref @ metamodelica::List::Cons { head: (-1), tail: Deref @ metamodelica::List::Nil } => BackendDAE::ZeroCrossing { index: 0, relation_: inRelation.clone(), occurEquLst: metamodelica::nil(), iter: iters.clone() },
        _ => BackendDAE::ZeroCrossing { index: 0, relation_: inRelation.clone(), occurEquLst: inOccurEquLst.clone(), iter: iters.clone() },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outZeroCrossing
}

fn createIterator(mut red_iter: Arc<DAE::ReductionIterator>) -> Result<BackendDAE::SimIterator> {
    let mut iter: BackendDAE::SimIterator;
    iter = (::match_deref::match_deref! { match &(red_iter.exp.clone()) {
        exp @ Deref @ DAE::Exp::RANGE { .. } => {
            let mut step: Arc<DAE::Exp>;
            let mut size: Arc<DAE::Exp>;
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut non_resizable_size: i32 = 0;
            ty = Expression::r#typeof(var_field!((**exp).start, DAE::Exp::RANGE).clone())?;
            step = Util::getOptionOrDefault(var_field!((**exp).step, DAE::Exp::RANGE).clone(), Arc::new(DAE::Exp::ICONST { integer: 1 }));
            size = Arc::new(DAE::Exp::BINARY { exp1: var_field!((**exp).stop, DAE::Exp::RANGE).clone(), operator: DAE::Operator::SUB { ty: ty.clone() }, exp2: var_field!((**exp).start, DAE::Exp::RANGE).clone() });
            size = Arc::new(DAE::Exp::BINARY { exp1: size.clone(), operator: DAE::Operator::DIV { ty: ty.clone() }, exp2: step.clone() });
            size = Arc::new(DAE::Exp::BINARY { exp1: size.clone(), operator: DAE::Operator::ADD { ty: ty.clone() }, exp2: Arc::new(DAE::Exp::ICONST { integer: 1 }) });
            (size, _) = ExpressionSimplify::simplify(size.clone())?;
            match '__try0: {
                non_resizable_size = unwrap_break_err!(Expression::getEvaluatedConstInteger(size.clone()), '__try0);
                Ok::<_, anyhow::Error>((non_resizable_size.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    non_resizable_size = __try0_o0;
                }
                Err(_) => {
                    non_resizable_size = 0;
                }
            }
            BackendDAE::SimIterator::SIM_ITERATOR_RANGE { name: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (red_iter.id.clone()).clone(), identType: DAE::T_INTEGER_DEFAULT.clone(), subscriptLst: metamodelica::nil() }), start: var_field!((**exp).start, DAE::Exp::RANGE).clone(), step: step.clone(), stop: var_field!((**exp).stop, DAE::Exp::RANGE).clone(), size: size.clone(), non_resizable_size: non_resizable_size.clone(), sub_iter: metamodelica::nil() }
        },
        exp @ Deref @ DAE::Exp::ARRAY { .. } => {
            BackendDAE::SimIterator::SIM_ITERATOR_LIST { name: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (red_iter.id.clone()).clone(), identType: DAE::T_INTEGER_DEFAULT.clone(), subscriptLst: metamodelica::nil() }), lst: {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut e in (var_field!((**exp).array, DAE::Exp::ARRAY).clone()).into_iter().cloned() {
            let __x = DAEUtil::getInteger(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, size: (var_field!((**exp).array, DAE::Exp::ARRAY).clone().len() as i32), sub_iter: metamodelica::nil() }
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FindZeroCrossings.createIterator")); __mm_s.push_str(&*literal!(" failed for expression: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(red_iter.exp.clone())?); __mm_s.push_str(&*literal!(".\n")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(iter)
}

