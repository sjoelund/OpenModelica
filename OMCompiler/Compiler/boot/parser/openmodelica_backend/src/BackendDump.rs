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
use crate::BackendVariable;
use crate::DumpHTML;
use crate::GraphvizDump;
use crate::HpcOmTaskGraph;
use crate::Initialization;
use crate::Matching;
use crate::ZeroCrossings;
use openmodelica_ast::Absyn;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::DAEDump;
use openmodelica_frontend::DAEUtil;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionDump;
use openmodelica_frontend::HashSet;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::DAEDumpTypes;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::ExpressionDumpTpl;
use openmodelica_frontend_types::DAE;
use openmodelica_susan::GraphML;
use openmodelica_susan::Tpl;
use openmodelica_util::BaseHashSet;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::IOStream;
use openmodelica_util::MMath;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::List;

// =============================================================================
// section for all print* functions
//
// These are functions, that print directly to the standard-stream.
//   - printBackendDAE
//   - printEqSystem
//   - printEquation
//   - printEquationArray
//   - printEquationList
//   - printEquations
//   - printClassAttributes
//   - printShared
//   - printStateSets
//   - printVar
//   - printVariables
//   - printVarList
// =============================================================================
pub fn printBackendDAE(mut inBackendDAE: Arc<BackendDAE::BackendDAE>) -> Result<()> {
    let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inBackendDAE.clone()) {
        Deref @ DAE { UNIQUEIO: __pa0, derivativeNamePrefix: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    eqs = __pa0.clone();
    shared = __pa1.clone();
    List::map_0(eqs.clone(), (std::sync::Arc::new(printEqSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>) -> Result<()> + 'static>));
    println!("{}", (literal!("\n")).clone());
    printShared(shared.clone())?;
    Ok(())
}

pub fn printEqSystem(mut inSyst: Arc<BackendDAE::EqSystem>) -> Result<()> {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*partitionKindString(inSyst.partitionKind.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    dumpVariables(inSyst.orderedVars.clone(), (literal!("Variables")).clone())?;
    dumpEquationArray(inSyst.orderedEqs.clone(), (literal!("Equations")).clone())?;
    dumpEquationArray(inSyst.removedEqs.clone(), (literal!("Simple Equations")).clone())?;
    dumpStateSets(inSyst.stateSets.clone(), (literal!("State Sets")).clone());
    dumpOption(inSyst.m.clone(), (std::sync::Arc::new(fnptr!(dumpAdjacencyMatrix, metamodelica::Array<Arc<metamodelica::List<i32>>>)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> + 'static>));
    dumpOption(inSyst.mT.clone(), (std::sync::Arc::new(fnptr!(dumpAdjacencyMatrixT, metamodelica::Array<Arc<metamodelica::List<i32>>>)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> + 'static>));
    println!("{}", (literal!("\n")).clone());
    dumpFullMatching(inSyst.matching.clone(), Some(inSyst.clone()))?;
    println!("{}", (literal!("\n")).clone());
    Ok(())
}

pub fn printEquation(mut inEquation: Arc<BackendDAE::Equation>) -> Result<()> {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*equationString(inEquation.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub fn printEquationArray(mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> () {
    let _ = List::fold(BackendEquation::equationList(eqns.clone()), (std::sync::Arc::new(printEquationList2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (i32, i32)) -> Result<(i32, i32)> + 'static>), (1, 1));
    ()
}

pub fn printEquationList(mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> () {
    let _ = List::fold(eqns.clone(), (std::sync::Arc::new(printEquationList2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (i32, i32)) -> Result<(i32, i32)> + 'static>), (1, 1));
    ()
}

fn printEquationList2(mut inEquation: Arc<BackendDAE::Equation>, mut inInteger: (i32, i32)) -> Result<(i32, i32)> {
    let mut oInteger: (i32, i32);
    let mut iscalar: i32 = 0;
    let mut i: i32 = 0;
    let mut size: i32 = 0;
    let mut attr: BackendDAE::EquationAttributes;
    (i, iscalar) = inInteger.clone();
    size = BackendEquation::equationSize(inEquation.clone())?;
    attr = BackendEquation::getEquationAttributes(inEquation.clone())?;
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*intString(iscalar.clone())); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(size.clone())); __mm_s.push_str(&*literal!("): ")); __mm_s.push_str(&*equationString(inEquation.clone())?); __mm_s.push_str(&*literal!("   ")); __mm_s.push_str(&*equationAttrString(attr.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    oInteger = (i.clone() + 1, iscalar.clone() + size.clone());
    Ok(oInteger)
}

pub fn equationListString(mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut heading: ArcStr) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &((inEqns.clone(), heading.clone())) {
        (_, Deref @ "") => {
            let mut buffer: ArcStr = arcstr::literal!("");
            (_, _, buffer) = List::fold(inEqns.clone(), (std::sync::Arc::new(equationList2String) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (i32, i32, ArcStr)) -> Result<(i32, i32, ArcStr)> + 'static>), (1, 1, literal!("")));
            buffer.clone()
        },
        _ => {
            let mut buffer: ArcStr = arcstr::literal!("");
            (_, _, buffer) = List::fold(inEqns.clone(), (std::sync::Arc::new(equationList2String) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (i32, i32, ArcStr)) -> Result<(i32, i32, ArcStr)> + 'static>), (1, 1, literal!("")));
            buffer = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*heading.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*buffer.clone()); ArcStr::from(__mm_s) }).clone();
            buffer.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    outString
}

fn equationList2String(mut inEquation: Arc<BackendDAE::Equation>, mut inTuple: (i32, i32, ArcStr)) -> Result<(i32, i32, ArcStr)> {
    let mut outTuple: (i32, i32, ArcStr);
    let mut iscalar: i32 = 0;
    let mut i: i32 = 0;
    let mut size: i32 = 0;
    let mut buffer: ArcStr = arcstr::literal!("");
    (i, iscalar, buffer) = inTuple.clone();
    size = BackendEquation::equationSize(inEquation.clone())?;
    buffer = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*buffer.clone()); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*intString(iscalar.clone())); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(size.clone())); __mm_s.push_str(&*literal!("): ")); __mm_s.push_str(&*equationString(inEquation.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    outTuple = (i.clone() + 1, iscalar.clone() + size.clone(), buffer.clone());
    Ok(outTuple)
}

pub fn printEquations(mut inIntegerLst: Arc<metamodelica::List<i32>>, mut syst: Arc<BackendDAE::EqSystem>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inIntegerLst.clone(), syst.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: n, tail: rest }, _) => {
            printEquations(rest.clone(), syst.clone())?;
            printEquationNo(n.clone(), syst.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printEquationNo(mut inInteger: i32, mut syst: Arc<BackendDAE::EqSystem>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inInteger.clone(), syst.clone())) {
        (eqno, Deref @ BackendDAE::EqSystem { orderedEqs: eqns, .. }) => {
            let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            eq = BackendEquation::get(eqns.clone(), eqno.clone());
            printEquation(eq.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn printClassAttributes(mut optimicaFun: Arc<DAE::ClassAttributes>) -> Result<()> {
    let mut e1: Option<Arc<DAE::Exp>> = None;
    let mut e2: Option<Arc<DAE::Exp>> = None;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(optimicaFun.clone()) {
        Deref @ DAE::ClassAttributes { objectiveIntegrandE: __pa0, objetiveE: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    e2 = __pa0.clone();
    e1 = __pa1.clone();
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Mayer")); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    println!("{}", (ExpressionDump::printOptExpStr(e1.clone())?).clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Lagrange")); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    println!("{}", (ExpressionDump::printOptExpStr(e2.clone())?).clone());
    println!("{}", (literal!("\n")).clone());
    Ok(())
}

pub fn printShared(mut inShared: Arc<BackendDAE::Shared>) -> Result<()> {
    let mut eqS: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut inlineData: BackendDAE::InlineData;
    println!("{}", (literal!("\nBackendDAEType: ")).clone());
    printBackendDAEType(inShared.backendDAEType.clone())?;
    println!("{}", (literal!("\n\n")).clone());
    dumpVariables(inShared.globalKnownVars.clone(), (literal!("Known variables only depending on parameters and constants - globalKnownVars")).clone())?;
    dumpVariables(inShared.localKnownVars.clone(), (literal!("Known variables only depending on states and inputs - localKnownVars")).clone())?;
    dumpVariables(inShared.externalObjects.clone(), (literal!("External Objects")).clone())?;
    dumpExternalObjectClasses(inShared.extObjClasses.clone(), (literal!("Classes of External Objects")).clone())?;
    dumpVariables(inShared.aliasVars.clone(), (literal!("Alias Variables")).clone())?;
    dumpEquationArray(inShared.removedEqs.clone(), (literal!("Simple Shared Equations")).clone())?;
    dumpEquationArray(inShared.initialEqs.clone(), (literal!("Initial Equations")).clone())?;
    dumpZeroCrossingList(ZeroCrossings::toList(inShared.eventInfo.zeroCrossings.clone()), (literal!("Zero Crossings")).clone())?;
    dumpZeroCrossingList(DoubleEnded::toListNoCopyNoClear(inShared.eventInfo.relations.clone()), (literal!("Relations")).clone())?;
    if stringEqual((Config::simCodeTarget()?).clone(), (literal!("Cpp")).clone()) {
        dumpZeroCrossingList(ZeroCrossings::toList(inShared.eventInfo.samples.clone()), (literal!("Samples")).clone())?;
    } else {
        dumpTimeEvents(inShared.eventInfo.timeEvents.clone(), (literal!("Time Events")).clone())?;
    }
    dumpConstraintList(inShared.constraints.clone(), (literal!("Constraints")).clone())?;
    dumpBasePartitions(inShared.partitionsInfo.basePartitions.clone(), (literal!("Base partitions")).clone())?;
    dumpSubPartitions(inShared.partitionsInfo.subPartitions.clone(), (literal!("Sub partitions")).clone())?;
    if Flags::isSet(Flags::DUMP_FUNCTIONS.clone())? {
        DAEDump::dumpFunctionTree(inShared.functionTree.clone(), (literal!("Functions")).clone())?;
    }
    Ok(())
}

pub fn printBasePartitions(mut basePartitions: metamodelica::Array<BackendDAE::BasePartition>) -> Result<()> {
    let mut clkExpStr: ArcStr = arcstr::literal!("");
    let mut nSubClocksStr: ArcStr = arcstr::literal!("");
    let __range0 = 1..=(basePartitions.clone().borrow().len() as i32);
    for mut i in __range0 {
        clkExpStr = (Tpl::tplString2((std::sync::Arc::new(ExpressionDumpTpl::dumpClockKind) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, Arc<DAE::ClockKind>, ArcStr) -> Result<Tpl::Text> + 'static>), basePartitions[(i.clone()-1) as usize].clock.clone(), (literal!("")).clone())?).clone();
        nSubClocksStr = (intString(basePartitions[(i.clone()-1) as usize].nSubClocks.clone())).clone();
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*clkExpStr.clone()); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*nSubClocksStr.clone()); __mm_s.push_str(&*literal!("]")); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(())
}

pub fn printSubPartitions(mut subPartitions: metamodelica::Array<BackendDAE::SubPartition>) -> Result<()> {
    let mut subClockStr: ArcStr = arcstr::literal!("");
    let mut eventStr: ArcStr = arcstr::literal!("");
    let __range0 = 1..=(subPartitions.clone().borrow().len() as i32);
    for mut i in __range0 {
        subClockStr = (subClockString(subPartitions[(i.clone()-1) as usize].clock.clone())?).clone();
        eventStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("event(")); __mm_s.push_str(&*boolString(subPartitions[(i.clone()-1) as usize].holdEvents.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*subClockStr.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*eventStr.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(())
}

pub fn subClockString(mut subClock: BackendDAE::SubClock) -> Result<ArcStr> {
    let mut subClockString: ArcStr = arcstr::literal!("");
    subClockString = ((match subClock.clone() {
        BackendDAE::SubClock::INFERED_SUBCLOCK => {
            literal!("INFERED_SUBCLOCK")
        },
        BackendDAE::SubClock::SUBCLOCK { factor: _, .. } => {
            let mut factorStr: ArcStr = arcstr::literal!("");
            let mut shiftStr: ArcStr = arcstr::literal!("");
            let mut solverStr: ArcStr = arcstr::literal!("");
            factorStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("factor(")); __mm_s.push_str(&*MMath::rationalString(var_field!(subClock.factor, BackendDAE::SubClock::SUBCLOCK).clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            shiftStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("shift(")); __mm_s.push_str(&*MMath::rationalString(var_field!(subClock.shift, BackendDAE::SubClock::SUBCLOCK).clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            solverStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("solver(")); __mm_s.push_str(&*optionString(var_field!(subClock.solver, BackendDAE::SubClock::SUBCLOCK).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            if ((solverStr.clone()).clone().len() as i32) > 8 {
                subClockString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*factorStr.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*shiftStr.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*solverStr.clone()); ArcStr::from(__mm_s) }).clone();
            } else {
                subClockString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*factorStr.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*shiftStr.clone()); __mm_s.push_str(&*literal!(" ")); ArcStr::from(__mm_s) }).clone();
            }
            subClockString.clone()
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(subClockString)
}

pub fn optionString(mut option: Option<ArcStr>) -> ArcStr {
    let mut optionString: ArcStr = arcstr::literal!("");
    optionString = ((match option.clone() {
        Some(mut s) => {
            s.clone()
        },
        _ => {
            literal!("")
        },
    })).clone();
    optionString
}

pub fn printBackendDAEType(mut btp: BackendDAE::BackendDAEType) -> Result<()> {
    println!("{}", (printBackendDAEType2String(btp.clone())?).clone());
    Ok(())
}

pub fn printBackendDAEType2String(mut btp: BackendDAE::BackendDAEType) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match btp.clone() {
        BackendDAE::BackendDAEType::SIMULATION => literal!("simulation"),
        BackendDAE::BackendDAEType::JACOBIAN => literal!("jacobian"),
        BackendDAE::BackendDAEType::ALGEQSYSTEM => literal!("algebraic loop"),
        BackendDAE::BackendDAEType::ARRAYSYSTEM => literal!("multidim equation arrays"),
        BackendDAE::BackendDAEType::PARAMETERSYSTEM => literal!("parameter system"),
        BackendDAE::BackendDAEType::INITIALSYSTEM => literal!("initialization"),
        BackendDAE::BackendDAEType::INLINESYSTEM => literal!("inline system"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

pub fn printStateSets(mut stateSets: Arc<metamodelica::List<BackendDAE::StateSet>>) -> () {
    List::map_0(stateSets.clone(), (std::sync::Arc::new(printStateSet) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::StateSet) -> Result<()> + 'static>));
    ()
}

fn printStateSet(mut inStateSet: BackendDAE::StateSet) -> Result<()> {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("StateSet \"")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(ComponentReferenceBasics::crefFirstCref(inStateSet.crA.clone())?)?); __mm_s.push_str(&*literal!("\" (rang ")); __mm_s.push_str(&*intString(inStateSet.rang.clone())); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone());
    dumpVarList(inStateSet.statescandidates.clone(), (literal!("state candidates")).clone());
    dumpEquationList(inStateSet.eqns.clone(), (literal!("eqns")).clone());
    dumpVarList(inStateSet.ovars.clone(), (literal!("ovars")).clone());
    dumpEquationList(inStateSet.oeqns.clone(), (literal!("oeqns")).clone());
    dumpVarList(inStateSet.varA.clone(), (literal!("varA")).clone());
    dumpVarList(inStateSet.varJ.clone(), (literal!("varJ")).clone());
    Ok(())
}

pub fn printVar(mut inVar: BackendDAE::Var) -> Result<()> {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*varString(inVar.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub fn printVariables(mut vars: BackendDAE::Variables) -> Result<()> {
    let _ = List::fold(BackendVariable::varList(vars.clone())?, (std::sync::Arc::new(printVars1) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, i32) -> Result<i32> + 'static>), 1);
    Ok(())
}

pub fn printVarList(mut vars: Arc<metamodelica::List<BackendDAE::Var>>) -> () {
    let _ = List::fold(vars.clone(), (std::sync::Arc::new(printVars1) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, i32) -> Result<i32> + 'static>), 1);
    ()
}

fn printVars1(mut inVar: BackendDAE::Var, mut inVarNo: i32) -> Result<i32> {
    let mut outVarNo: i32 = 0;
    println!("{}", (intString(inVarNo.clone())).clone());
    println!("{}", (literal!(": ")).clone());
    printVar(inVar.clone())?;
    outVarNo = inVarNo.clone() + 1;
    Ok(outVarNo)
}

pub fn varListString(mut inVars: Arc<metamodelica::List<BackendDAE::Var>>, mut heading: ArcStr) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &((inVars.clone(), heading.clone())) {
        (_, Deref @ "") => {
            let mut buffer: ArcStr = arcstr::literal!("");
            (_, buffer) = List::fold(inVars.clone(), (std::sync::Arc::new(var1String) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (i32, ArcStr)) -> Result<(i32, ArcStr)> + 'static>), (1, literal!("")));
            buffer.clone()
        },
        _ => {
            let mut buffer: ArcStr = arcstr::literal!("");
            (_, buffer) = List::fold(inVars.clone(), (std::sync::Arc::new(var1String) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (i32, ArcStr)) -> Result<(i32, ArcStr)> + 'static>), (1, literal!("")));
            buffer = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*heading.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*buffer.clone()); ArcStr::from(__mm_s) }).clone();
            buffer.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    outString
}

fn var1String(mut inVar: BackendDAE::Var, mut inTpl: (i32, ArcStr)) -> Result<(i32, ArcStr)> {
    let mut outTpl: (i32, ArcStr);
    let mut varNo: i32 = 0;
    let mut buffer: ArcStr = arcstr::literal!("");
    (varNo, buffer) = inTpl.clone();
    buffer = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*buffer.clone()); __mm_s.push_str(&*intString(varNo.clone())); __mm_s.push_str(&*literal!(": ")); ArcStr::from(__mm_s) }).clone();
    buffer = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*buffer.clone()); __mm_s.push_str(&*varString(inVar.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    outTpl = (varNo.clone() + 1, buffer.clone());
    Ok(outTpl)
}

pub fn varListStringShort(mut inVars: Arc<metamodelica::List<BackendDAE::Var>>, mut heading: ArcStr) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &((inVars.clone(), heading.clone())) {
        (_, Deref @ "") => {
            let mut buffer: ArcStr = arcstr::literal!("");
            (_, buffer) = List::fold(inVars.clone(), (std::sync::Arc::new(varNameString) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (i32, ArcStr)) -> Result<(i32, ArcStr)> + 'static>), (1, literal!("")));
            buffer.clone()
        },
        _ => {
            let mut buffer: ArcStr = arcstr::literal!("");
            (_, buffer) = List::fold(inVars.clone(), (std::sync::Arc::new(varNameString) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (i32, ArcStr)) -> Result<(i32, ArcStr)> + 'static>), (1, literal!("")));
            buffer = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*heading.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*buffer.clone()); ArcStr::from(__mm_s) }).clone();
            buffer.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    outString
}

fn varNameString(mut inVar: BackendDAE::Var, mut inTpl: (i32, ArcStr)) -> Result<(i32, ArcStr)> {
    let mut outTpl: (i32, ArcStr);
    let mut varNo: i32 = 0;
    let mut buffer: ArcStr = arcstr::literal!("");
    (varNo, buffer) = inTpl.clone();
    buffer = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*buffer.clone()); __mm_s.push_str(&*intString(varNo.clone())); __mm_s.push_str(&*literal!(": ")); ArcStr::from(__mm_s) }).clone();
    buffer = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*buffer.clone()); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inVar.varName.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    outTpl = (varNo.clone() + 1, buffer.clone());
    Ok(outTpl)
}

pub fn varListStringIndented(mut inVars: Arc<metamodelica::List<BackendDAE::Var>>, mut heading: ArcStr) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &((inVars.clone(), heading.clone())) {
        (_, Deref @ "") => {
            let mut buffer: ArcStr = arcstr::literal!("");
            (_, buffer) = List::fold(inVars.clone(), (std::sync::Arc::new(var1StringIndented) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (i32, ArcStr)) -> Result<(i32, ArcStr)> + 'static>), (1, literal!("")));
            buffer.clone()
        },
        _ => {
            let mut buffer: ArcStr = arcstr::literal!("");
            (_, buffer) = List::fold(inVars.clone(), (std::sync::Arc::new(var1StringIndented) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (i32, ArcStr)) -> Result<(i32, ArcStr)> + 'static>), (1, literal!("")));
            buffer = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*heading.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*buffer.clone()); ArcStr::from(__mm_s) }).clone();
            buffer.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    outString
}

fn var1StringIndented(mut inVar: BackendDAE::Var, mut inTpl: (i32, ArcStr)) -> Result<(i32, ArcStr)> {
    let mut outTpl: (i32, ArcStr);
    let mut varNo: i32 = 0;
    let mut buffer: ArcStr = arcstr::literal!("");
    (varNo, buffer) = inTpl.clone();
    buffer = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*buffer.clone()); __mm_s.push_str(&*literal!("   ")); __mm_s.push_str(&*intString(varNo.clone())); __mm_s.push_str(&*literal!(": ")); ArcStr::from(__mm_s) }).clone();
    buffer = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*buffer.clone()); __mm_s.push_str(&*varString(inVar.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    outTpl = (varNo.clone() + 1, buffer.clone());
    Ok(outTpl)
}

fn printExternalObjectClasses(mut cls: Arc<metamodelica::List<BackendDAE::ExternalObjectClass>>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::ExternalObjectClass { path, source }, tail: _ } => {
            let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
            let mut paths_lst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut path_str: ArcStr = arcstr::literal!("");
            println!("{}", (literal!("class ")).clone());
            println!("{}", (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone());
            println!("{}", (literal!("\n  extends ExternalObject;")).clone());
            println!("{}", (literal!("\n origin: ")).clone());
            paths = ElementSource::getElementSourceTypes(source.clone());
            paths_lst = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut p in (paths.clone()).into_iter().cloned() {
            let __x = AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            path_str = stringDelimitList(paths_lst.clone(), (literal!(", ")).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*path_str.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            println!("{}", (literal!("end ")).clone());
            println!("{}", (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn printSparsityPatternCrefs(mut inPattern: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>) -> Result<()> {
    for mut e in &*inPattern.clone() {
        let mut e = e.clone();
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(Util::tuple21(e.clone()))?); __mm_s.push_str(&*literal!(" affects the following (")); __mm_s.push_str(&*intString((Util::tuple22(e.clone()).len() as i32))); __mm_s.push_str(&*literal!(") outputs\n  ")); ArcStr::from(__mm_s) }).clone());
        ComponentReference::printComponentRefList(Util::tuple22(e.clone()));
    }
    Ok(())
}

// =============================================================================
// section for all graphviz* functions
//
// =============================================================================
pub fn graphvizBackendDAE(mut inBackendDAE: Arc<BackendDAE::BackendDAE>, mut inFileNameSuffix: ArcStr) -> Result<()> {
    let mut dae: Arc<BackendDAE::BackendDAE>;
    let mut eqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut fileNamePrefix: ArcStr = arcstr::literal!("");
    let mut buffer: ArcStr = arcstr::literal!("");
    dae = setAdjacencyMatrix(inBackendDAE.clone())?;
    Tpl::tplNoret2((std::sync::Arc::new(GraphvizDump::dumpBackendDAE) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, Arc<BackendDAE::BackendDAE>, ArcStr) -> Result<Tpl::Text> + 'static>), dae.clone(), (inFileNameSuffix.clone()).clone())?;
    Ok(())
}

pub fn graphvizAdjacencyMatrix(mut inBackendDAE: Arc<BackendDAE::BackendDAE>, mut inFileNameSuffix: ArcStr) -> Result<()> {
    let mut dae: Arc<BackendDAE::BackendDAE>;
    let mut eqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut fileNamePrefix: ArcStr = arcstr::literal!("");
    let mut buffer: ArcStr = arcstr::literal!("");
    dae = setAdjacencyMatrix(inBackendDAE.clone())?;
    Tpl::tplNoret2((std::sync::Arc::new(GraphvizDump::dumpAdjacencyMatrix) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, Arc<BackendDAE::BackendDAE>, ArcStr) -> Result<Tpl::Text> + 'static>), dae.clone(), (inFileNameSuffix.clone()).clone())?;
    Ok(())
}

fn setAdjacencyMatrix(mut inBackendDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outBackendDAE: Arc<BackendDAE::BackendDAE>;
    let mut eqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inBackendDAE.clone()) {
        Deref @ DAE { UNIQUEIO: __pa0, derivativeNamePrefix: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    eqSystems = __pa0.clone();
    shared = __pa1.clone();
    eqSystems = List::map1(eqSystems.clone(), (std::sync::Arc::new(setAdjacencyMatrix1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, bool) -> Result<Arc<BackendDAE::EqSystem>> + 'static>), BackendDAEUtil::isInitializationDAE(shared.clone()));
    outBackendDAE = BackendDAE::DAE(eqSystems.clone(), shared.clone())?;
    Ok(outBackendDAE)
}

fn setAdjacencyMatrix1(mut inEqSystem: Arc<BackendDAE::EqSystem>, mut isInitial: bool) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut outEqSystem: Arc<BackendDAE::EqSystem>;
    (outEqSystem, _, _) = BackendDAEUtil::getAdjacencyMatrix(inEqSystem.clone(), crate::BackendDAE::IndexType::NORMAL, None, isInitial.clone())?;
    Ok(outEqSystem)
}

// =============================================================================
// section for all dump* functions
//
// These are functions, that print directly to the standard-stream and separates
// there output (e.g. with some kind of headings).
//   - dumpBackendDAE
//   - dumpBackendDAEEqnList
//   - dumpBackendDAEVarList
//   - dumpComponent
//   - dumpComponents
//   - dumpComponentsAdvanced
//   - dumpEqnsSolved
//   - dumpEqSystem
//   - dumpEqSystems
//   - dumpEquationArray
//   - dumpEquationList
//   - dumpHashSet
//   - dumpSparsityPattern
//   - dumpTearing
//   - dumpVariables
//   - dumpVarList
// =============================================================================
pub const BORDER: &'static str = "########################################";

pub const UNDERLINE: &'static str = "========================================";

pub fn dumpDAE(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = inDAE.clone();
    dumpBackendDAE(inDAE.clone(), (literal!("dumpDAE")).clone())?;
    Ok(outDAE)
}

pub fn dumpBackendDAE(mut inBackendDAE: Arc<BackendDAE::BackendDAE>, mut heading: ArcStr) -> Result<()> {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    printBackendDAE(inBackendDAE.clone())?;
    println!("{}", (literal!("\n")).clone());
    Ok(())
}

pub fn dumpEqSystem(mut inEqSystem: Arc<BackendDAE::EqSystem>, mut heading: ArcStr) -> Result<()> {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    printEqSystem(inEqSystem.clone())?;
    println!("{}", (literal!("\n")).clone());
    Ok(())
}

pub fn dumpEqSystemShort(mut inEqSystem: Arc<BackendDAE::EqSystem>, mut heading: ArcStr) -> Result<()> {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading.clone()); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*partitionKindString(inEqSystem.partitionKind.clone())?); __mm_s.push_str(&*literal!(")\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    dumpVariables(inEqSystem.orderedVars.clone(), (literal!("Variables")).clone())?;
    dumpEquationArray(inEqSystem.orderedEqs.clone(), (literal!("Equations")).clone())?;
    println!("{}", (literal!("\n")).clone());
    Ok(())
}

pub fn dumpEqSystems(mut inEqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, mut heading: ArcStr) -> () {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading.clone()); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString((inEqSystems.clone().len() as i32))); __mm_s.push_str(&*literal!(" partitions)\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    List::map_0(inEqSystems.clone(), (std::sync::Arc::new(printEqSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>) -> Result<()> + 'static>));
    println!("{}", (literal!("\n")).clone());
    ()
}

pub fn dumpBasePartitions(mut basePartitions: metamodelica::Array<BackendDAE::BasePartition>, mut heading: ArcStr) -> Result<()> {
    if (basePartitions.clone().borrow().len() as i32) > 0 {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading.clone()); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString((basePartitions.clone().borrow().len() as i32))); __mm_s.push_str(&*literal!(")\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        printBasePartitions(basePartitions.clone())?;
        println!("{}", (literal!("\n")).clone());
    }
    Ok(())
}

pub fn dumpSubPartitions(mut subPartitions: metamodelica::Array<BackendDAE::SubPartition>, mut heading: ArcStr) -> Result<()> {
    if (subPartitions.clone().borrow().len() as i32) > 0 {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading.clone()); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString((subPartitions.clone().borrow().len() as i32))); __mm_s.push_str(&*literal!(")\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        printSubPartitions(subPartitions.clone())?;
        println!("{}", (literal!("\n")).clone());
    }
    Ok(())
}

pub fn dumpVariables(mut inVars: BackendDAE::Variables, mut heading: ArcStr) -> Result<()> {
    if BackendVariable::varsSize(inVars.clone())? > 0 {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading.clone()); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(BackendVariable::varsSize(inVars.clone())?)); __mm_s.push_str(&*literal!(")\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        printVariables(inVars.clone())?;
        println!("{}", (literal!("\n")).clone());
    }
    Ok(())
}

pub fn dumpVarList(mut inVars: Arc<metamodelica::List<BackendDAE::Var>>, mut heading: ArcStr) -> () {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading.clone()); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString((inVars.clone().len() as i32))); __mm_s.push_str(&*literal!(")\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    printVarList(inVars.clone());
    println!("{}", (literal!("\n")).clone());
    ()
}

pub fn dumpEquationArray(mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut heading: ArcStr) -> Result<()> {
    if BackendEquation::getNumberOfEquations(inEqns.clone()) + BackendEquation::equationArraySize(inEqns.clone())? > 0 {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading.clone()); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(BackendEquation::getNumberOfEquations(inEqns.clone()))); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*intString(BackendEquation::equationArraySize(inEqns.clone())?)); __mm_s.push_str(&*literal!(")\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        printEquationArray(inEqns.clone());
        println!("{}", (literal!("\n")).clone());
    }
    Ok(())
}

pub fn dumpEquationList(mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut heading: ArcStr) -> () {
    if !(inEqns.clone().is_empty()) {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading.clone()); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString((inEqns.clone().len() as i32))); __mm_s.push_str(&*literal!(")\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        printEquationList(inEqns.clone());
        println!("{}", (literal!("\n")).clone());
    }
    ()
}

fn dumpExternalObjectClasses(mut inEOC: Arc<metamodelica::List<BackendDAE::ExternalObjectClass>>, mut heading: ArcStr) -> Result<()> {
    if !(inEOC.clone().is_empty()) {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading.clone()); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString((inEOC.clone().len() as i32))); __mm_s.push_str(&*literal!(")\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        printExternalObjectClasses(inEOC.clone())?;
        println!("{}", (literal!("\n")).clone());
    }
    Ok(())
}

pub fn dumpStateSets(mut stateSets: Arc<metamodelica::List<BackendDAE::StateSet>>, mut heading: ArcStr) -> () {
    if !(stateSets.clone().is_empty()) {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        printStateSets(stateSets.clone());
        println!("{}", (literal!("\n")).clone());
    }
    ()
}

pub fn dumpZeroCrossingList(mut inZeroCrossingList: Arc<metamodelica::List<BackendDAE::ZeroCrossing>>, mut heading: ArcStr) -> Result<()> {
    let mut zeroCrossing: BackendDAE::ZeroCrossing;
    if !(inZeroCrossingList.clone().is_empty()) {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading.clone()); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString((inZeroCrossingList.clone().len() as i32))); __mm_s.push_str(&*literal!(")\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        for mut zeroCrossing in &*inZeroCrossingList.clone() {
            let mut zeroCrossing = zeroCrossing.clone();
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*zeroCrossingString(zeroCrossing.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        println!("{}", (literal!("\n")).clone());
    }
    Ok(())
}

pub fn dumpTimeEvents(mut inTimeEvents: Arc<metamodelica::List<BackendDAE::TimeEvent>>, mut heading: ArcStr) -> Result<()> {
    let mut timeEvent: BackendDAE::TimeEvent = BackendDAE::TimeEvent::SIMPLE_TIME_EVENT;
    if !(inTimeEvents.clone().is_empty()) {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading.clone()); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString((inTimeEvents.clone().len() as i32))); __mm_s.push_str(&*literal!(")\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        for mut timeEvent in &*inTimeEvents.clone() {
            let mut timeEvent = timeEvent.clone();
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*timeEventString(timeEvent.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        println!("{}", (literal!("\n")).clone());
    }
    Ok(())
}

fn dumpConstraintList(mut inConstraintArray: Arc<metamodelica::List<Arc<DAE::Constraint>>>, mut heading: ArcStr) -> Result<()> {
    if !(inConstraintArray.clone().is_empty()) {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading.clone()); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString((inConstraintArray.clone().len() as i32))); __mm_s.push_str(&*literal!(")\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        dumpConstraints(inConstraintArray.clone(), 0)?;
        println!("{}", (literal!("\n")).clone());
    }
    Ok(())
}

pub fn dumpHashSet(mut hashSet: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), mut heading: ArcStr) -> Result<()> {
    let mut size: i32 = 0;
    size = BaseHashSet::currentSize(hashSet.clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading.clone()); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(size.clone())); __mm_s.push_str(&*literal!(")\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    BaseHashSet::printHashSet(hashSet.clone())?;
    println!("{}", (literal!("\n")).clone());
    Ok(())
}

pub fn dumpSparsityPattern(mut inPattern: (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32), mut heading: ArcStr) -> Result<()> {
    let mut pattern: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>> = metamodelica::nil();
    let mut patternT: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>> = metamodelica::nil();
    let mut diffVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut diffedVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut nnz: i32 = 0;
    let (__pa0, __pa1, (__pa2, __pa3), __pa4) = inPattern.clone();
    pattern = __pa0.clone();
    patternT = __pa1.clone();
    diffVars = __pa2.clone();
    diffedVars = __pa3.clone();
    nnz = __pa4.clone();
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Number of non zero elements: ")); __mm_s.push_str(&*intString(nnz.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Independents [or inputs] (")); __mm_s.push_str(&*intString((diffVars.clone().len() as i32))); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone());
    ComponentReference::printComponentRefList(diffVars.clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Dependents [or outputs] (")); __mm_s.push_str(&*intString((diffedVars.clone().len() as i32))); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone());
    ComponentReference::printComponentRefList(diffedVars.clone());
    printSparsityPatternCrefs(pattern.clone())?;
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("Transposed pattern")); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    printSparsityPatternCrefs(patternT.clone())?;
    Ok(())
}

pub fn dumpSparseColoring(mut inColoring: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>, mut heading: ArcStr) -> () {
    let mut i: i32 = 0;
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Number of colors: ")); __mm_s.push_str(&*intString((inColoring.clone().len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    for mut crList in &*inColoring.clone() {
        let mut crList = crList.clone();
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The following (")); __mm_s.push_str(&*intString((crList.clone().len() as i32))); __mm_s.push_str(&*literal!(") independents belong to one color\n")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(": ")); ArcStr::from(__mm_s) }).clone());
        ComponentReference::printComponentRefList(crList.clone());
        i = i.clone() + 1;
    }
    ()
}

pub fn dumpTearing(mut inResEqn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut inTearVar: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inResEqn.clone(), inTearVar.clone())) {
        (Deref @ metamodelica::List::Cons { head: residualeqns, tail: r }, Deref @ metamodelica::List::Cons { head: tearingvars, tail: t }) => {
            let mut str_r: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut str_t: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut str_r_f: ArcStr = arcstr::literal!("");
            let mut str_r_1: ArcStr = arcstr::literal!("");
            let mut str_t_f: ArcStr = arcstr::literal!("");
            let mut str_t_1: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut sr: ArcStr = arcstr::literal!("");
            let mut st: ArcStr = arcstr::literal!("");
            str_r = List::map(residualeqns.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
            str_r_f = stringDelimitList(str_r.clone(), (literal!(", ")).clone());
            str_r_1 = (stringAppend((str_r_f.clone()).clone(), (literal!("\n")).clone())).clone();
            sr = (stringAppend((literal!("ResidualEqns: ")).clone(), (str_r_1.clone()).clone())).clone();
            str_t = List::map(tearingvars.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
            str_t_f = stringDelimitList(str_t.clone(), (literal!(", ")).clone());
            str_t_1 = (stringAppend((str_t_f.clone()).clone(), (literal!("\n")).clone())).clone();
            st = (stringAppend((literal!("TearingVars: ")).clone(), (str_t_1.clone()).clone())).clone();
            r#str = (stringAppend((sr.clone()).clone(), (st.clone()).clone())).clone();
            println!("{}", (r#str.clone()).clone());
            println!("{}", (literal!("\n")).clone());
            dumpTearing(r.clone(), t.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn dumpBackendDAEEqnList(mut inBackendDAEEqnList: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut header: ArcStr, mut printExpTree: bool) -> Result<()> {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*header.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    dumpBackendDAEEqnList2(inBackendDAEEqnList.clone(), printExpTree.clone())?;
    println!("{}", (literal!("===================\n")).clone());
    Ok(())
}

fn dumpBackendDAEEqnList2(mut inBackendDAEEqnList: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut printExpTree: bool) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inBackendDAEEqnList.clone(), printExpTree.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::EQUATION { attr: BackendDAE::EquationAttributes { kind: eqKind, .. }, scalar: e2, exp: e1, .. }, tail: res }, _) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (literal!("EQUATION: ")).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*ExpressionBasics::printExpStr(e1.clone())?); ArcStr::from(__mm_s) }).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" = ")); ArcStr::from(__mm_s) }).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*ExpressionBasics::printExpStr(e2.clone())?); ArcStr::from(__mm_s) }).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*equationKindString(eqKind.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone();
                    println!("{}", (r#str.clone()).clone());
                    r#str = (literal!("LHS:\n")).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*ExpressionDump::dumpExpStr(e1.clone(), 0)?); ArcStr::from(__mm_s) }).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("RHS:\n")); ArcStr::from(__mm_s) }).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*ExpressionDump::dumpExpStr(e2.clone(), 0)?); ArcStr::from(__mm_s) }).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                    r#str = (if (printExpTree.clone()) {r#str.clone()} else {literal!("")}).clone();
                    println!("{}", (r#str.clone()).clone());
                    dumpBackendDAEEqnList2(res.clone(), printExpTree.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::COMPLEX_EQUATION { attr: BackendDAE::EquationAttributes { kind: eqKind, .. }, right: e2, left: e1, .. }, tail: res }, _) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (literal!("COMPLEX_EQUATION: ")).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*ExpressionBasics::printExpStr(e1.clone())?); ArcStr::from(__mm_s) }).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" = ")); ArcStr::from(__mm_s) }).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*ExpressionBasics::printExpStr(e2.clone())?); ArcStr::from(__mm_s) }).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*equationKindString(eqKind.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone();
                    println!("{}", (r#str.clone()).clone());
                    r#str = (literal!("LHS:\n")).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*ExpressionDump::dumpExpStr(e1.clone(), 0)?); ArcStr::from(__mm_s) }).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("RHS:\n")); ArcStr::from(__mm_s) }).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*ExpressionDump::dumpExpStr(e2.clone(), 0)?); ArcStr::from(__mm_s) }).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                    r#str = (if (printExpTree.clone()) {r#str.clone()} else {literal!("")}).clone();
                    println!("{}", (r#str.clone()).clone());
                    dumpBackendDAEEqnList2(res.clone(), printExpTree.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::SOLVED_EQUATION { attr: BackendDAE::EquationAttributes { kind: eqKind, .. }, exp: e, .. }, tail: res }, _) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    println!("{}", (literal!("SOLVED_EQUATION: ")).clone());
                    r#str = (ExpressionBasics::printExpStr(e.clone())?).clone();
                    println!("{}", (r#str.clone()).clone());
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*equationKindString(eqKind.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone());
                    r#str = (ExpressionDump::dumpExpStr(e.clone(), 0)?).clone();
                    r#str = (if (printExpTree.clone()) {r#str.clone()} else {literal!("")}).clone();
                    println!("{}", (r#str.clone()).clone());
                    println!("{}", (literal!("\n")).clone());
                    dumpBackendDAEEqnList2(res.clone(), printExpTree.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { attr: BackendDAE::EquationAttributes { kind: eqKind, .. }, exp: e, .. }, tail: res }, _) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (literal!("RESIDUAL_EQUATION: ")).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); ArcStr::from(__mm_s) }).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*equationKindString(eqKind.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone();
                    println!("{}", (r#str.clone()).clone());
                    r#str = (ExpressionDump::dumpExpStr(e.clone(), 0)?).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                    r#str = (if (printExpTree.clone()) {r#str.clone()} else {literal!("")}).clone();
                    println!("{}", (r#str.clone()).clone());
                    dumpBackendDAEEqnList2(res.clone(), printExpTree.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::ARRAY_EQUATION { attr: BackendDAE::EquationAttributes { kind: eqKind, .. }, left: e1, .. }, tail: res }, _) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    println!("{}", (literal!("ARRAY_EQUATION: ")).clone());
                    r#str = (ExpressionBasics::printExpStr(e1.clone())?).clone();
                    println!("{}", (r#str.clone()).clone());
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*equationKindString(eqKind.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone();
                    r#str = (ExpressionDump::dumpExpStr(e1.clone(), 0)?).clone();
                    r#str = (if (printExpTree.clone()) {r#str.clone()} else {literal!("")}).clone();
                    println!("{}", (r#str.clone()).clone());
                    println!("{}", (literal!("\n")).clone());
                    dumpBackendDAEEqnList2(res.clone(), printExpTree.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::ALGORITHM { attr: BackendDAE::EquationAttributes { kind: eqKind, .. }, alg, .. }, tail: res }, _) => {
                    println!("{}", (literal!("ALGORITHM: ")).clone());
                    dumpAlgorithms(list![alg.clone()], 0)?;
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*equationKindString(eqKind.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone());
                    dumpBackendDAEEqnList2(res.clone(), printExpTree.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::WHEN_EQUATION { attr: BackendDAE::EquationAttributes { kind: eqKind, .. }, whenEquation: weqn, .. }, tail: _ }, _) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut r#str: ArcStr = arcstr::literal!("");
                    println!("{}", (literal!("WHEN_EQUATION: ")).clone());
                    r#str = (whenEquationString(weqn.clone(), true)?).clone();
                    println!("{}", (r#str.clone()).clone());
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*equationKindString(eqKind.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone();
                    e = weqn.condition.clone();
                    r#str = (ExpressionDump::dumpExpStr(e.clone(), 0)?).clone();
                    r#str = (if (printExpTree.clone()) {r#str.clone()} else {literal!("")}).clone();
                    println!("{}", (r#str.clone()).clone());
                    println!("{}", (literal!("\n")).clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: res }, _) => {
                    println!("{}", (literal!("SKIPED EQUATION\n")).clone());
                    dumpBackendDAEEqnList2(res.clone(), printExpTree.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn dumpBackendDAEVarList(mut inBackendDAEVarList: Arc<metamodelica::List<BackendDAE::Var>>, mut header: ArcStr) -> () {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*header.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    printVarList(inBackendDAEVarList.clone());
    println!("{}", (literal!("===================\n")).clone());
    ()
}

pub fn dumpEqnsSolved(mut inBackendDAE: Arc<BackendDAE::BackendDAE>, mut heading: ArcStr) -> Result<()> {
    let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    let __pa0 = ::match_deref::match_deref! { match &(inBackendDAE.clone()) {
        Deref @ DAE { eqs: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    eqs = __pa0.clone();
    List::map_0(eqs.clone(), (std::sync::Arc::new(dumpEqnsSolved1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>) -> Result<()> + 'static>));
    println!("{}", (literal!("\n")).clone());
    Ok(())
}

fn dumpEqnsSolved1(mut inEqSystem: Arc<BackendDAE::EqSystem>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inEqSystem.clone()) {
        Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { comps, .. }, orderedEqs: eqns, orderedVars: vars, .. } => {
            dumpEqnsSolved2(comps.clone(), eqns.clone(), vars.clone())?;
            ()
        },
        _ => {
            println!("{}", (literal!("No Matching\n")).clone());
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpEqnsSolved2(mut inComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut vars: BackendDAE::Variables) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inComps.clone(), eqns.clone(), vars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { var: v, eqn: e }, tail: rest }, _, _) => {
                    let mut var: BackendDAE::Var;
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SingleEquation: ")); __mm_s.push_str(&*intString(e.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    var = BackendVariable::getVarAt(vars.clone(), v.clone())?;
                    printVarList(list![var.clone()]);
                    eqn = BackendEquation::get(eqns.clone(), e.clone());
                    printEquationList(list![eqn.clone()]);
                    println!("{}", (literal!("\n")).clone());
                    dumpEqnsSolved2(rest.clone(), eqns.clone(), vars.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jacType, jac: Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: jac }, vars: vlst, eqns: elst, .. }, tail: rest }, _, _) => {
                    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut eqnlst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Equationsystem ")); __mm_s.push_str(&*jacobianTypeStr(jacType.clone())?); __mm_s.push_str(&*literal!(":\n")); ArcStr::from(__mm_s) }).clone());
                    varlst = List::map1r(vlst.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone());
                    printVarList(varlst.clone());
                    eqnlst = BackendEquation::getList(elst.clone(), eqns.clone());
                    printEquationList(eqnlst.clone());
                    println!("{}", (literal!("\n")).clone());
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Jac:\n")); __mm_s.push_str(&*dumpJacobianStr(jac.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    println!("{}", (literal!("\n")).clone());
                    dumpEqnsSolved2(rest.clone(), eqns.clone(), vars.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEARRAY { vars: vlst, eqn: e }, tail: rest }, _, _) => {
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    println!("{}", (literal!("ArrayEquation:\n")).clone());
                    varlst = List::map1r(vlst.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone());
                    printVarList(varlst.clone());
                    eqn = BackendEquation::get(eqns.clone(), e.clone());
                    printEquationList(list![eqn.clone()]);
                    println!("{}", (literal!("\n")).clone());
                    dumpEqnsSolved2(rest.clone(), eqns.clone(), vars.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { vars: vlst, eqn: e }, tail: rest }, _, _) => {
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    println!("{}", (literal!("IfEquation:\n")).clone());
                    varlst = List::map1r(vlst.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone());
                    printVarList(varlst.clone());
                    eqn = BackendEquation::get(eqns.clone(), e.clone());
                    printEquationList(list![eqn.clone()]);
                    println!("{}", (literal!("\n")).clone());
                    dumpEqnsSolved2(rest.clone(), eqns.clone(), vars.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { vars: vlst, eqn: e }, tail: rest }, _, _) => {
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    println!("{}", (literal!("Algorithm:\n")).clone());
                    varlst = List::map1r(vlst.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone());
                    printVarList(varlst.clone());
                    eqn = BackendEquation::get(eqns.clone(), e.clone());
                    printEquationList(list![eqn.clone()]);
                    println!("{}", (literal!("\n")).clone());
                    dumpEqnsSolved2(rest.clone(), eqns.clone(), vars.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { vars: vlst, eqn: e }, tail: rest }, _, _) => {
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    println!("{}", (literal!("ComplexEquation:\n")).clone());
                    varlst = List::map1r(vlst.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone());
                    printVarList(varlst.clone());
                    eqn = BackendEquation::get(eqns.clone(), e.clone());
                    printEquationList(list![eqn.clone()]);
                    println!("{}", (literal!("\n")).clone());
                    dumpEqnsSolved2(rest.clone(), eqns.clone(), vars.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { vars: vlst, eqn: e }, tail: rest }, _, _) => {
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    println!("{}", (literal!("WhenEquation:\n")).clone());
                    varlst = List::map1r(vlst.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone());
                    printVarList(varlst.clone());
                    eqn = BackendEquation::get(eqns.clone(), e.clone());
                    printEquationList(list![eqn.clone()]);
                    println!("{}", (literal!("\n")).clone());
                    dumpEqnsSolved2(rest.clone(), eqns.clone(), vars.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::TORNSYSTEM { linear: b, casualTearingSet: None, strictTearingSet: BackendDAE::TearingSet { innerEquations, residualequations: elst, tearingvars: vlst, .. }, .. }, tail: rest }, _, _) => {
                    let mut vlst1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut elst1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut vlst1Lst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut eqnlst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut s: ArcStr = arcstr::literal!("");
                    s = (if (b.clone()) {literal!("linear")} else {literal!("nonlinear")}).clone();
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("torn ")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!(" Equationsystem:\n")); ArcStr::from(__mm_s) }).clone());
                    (elst1, vlst1Lst, _) = List::map_3(innerEquations.clone(), (std::sync::Arc::new(BackendDAEUtil::getEqnAndVarsFromInnerEquation) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::InnerEquation) -> Result<(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<DAE::Constraint>>>)> + 'static>));
                    vlst1 = List::flatten(vlst1Lst.clone());
                    varlst = List::map1r(vlst1.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone());
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\ninternal vars (")); __mm_s.push_str(&*intString((varlst.clone().len() as i32))); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone());
                    printVarList(varlst.clone());
                    varlst = List::map1r(vlst.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone());
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nresidual vars (")); __mm_s.push_str(&*intString((varlst.clone().len() as i32))); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone());
                    printVarList(varlst.clone());
                    eqnlst = BackendEquation::getList(elst1.clone(), eqns.clone());
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\ninternal equations (")); __mm_s.push_str(&*intString((eqnlst.clone().len() as i32))); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone());
                    printEquationList(eqnlst.clone());
                    eqnlst = BackendEquation::getList(elst.clone(), eqns.clone());
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nresidual equations (")); __mm_s.push_str(&*intString((eqnlst.clone().len() as i32))); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone());
                    printEquationList(eqnlst.clone());
                    println!("{}", (literal!("\n")).clone());
                    dumpEqnsSolved2(rest.clone(), eqns.clone(), vars.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::TORNSYSTEM { linear: b, casualTearingSet: Some(BackendDAE::TearingSet { innerEquations: innerEquations2, residualequations: elst2, tearingvars: vlst2, .. }), strictTearingSet: BackendDAE::TearingSet { innerEquations, residualequations: elst, tearingvars: vlst, .. }, .. }, tail: rest }, _, _) => {
                    let mut vlst1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut elst1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut vlst1Lst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut eqnlst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut s: ArcStr = arcstr::literal!("");
                    s = (if (b.clone()) {literal!("linear")} else {literal!("nonlinear")}).clone();
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Strict torn ")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!(" Equationsystem:\n")); ArcStr::from(__mm_s) }).clone());
                    (elst1, vlst1Lst, _) = List::map_3(innerEquations.clone(), (std::sync::Arc::new(BackendDAEUtil::getEqnAndVarsFromInnerEquation) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::InnerEquation) -> Result<(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<DAE::Constraint>>>)> + 'static>));
                    vlst1 = List::flatten(vlst1Lst.clone());
                    varlst = List::map1r(vlst1.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone());
                    printVarList(varlst.clone());
                    varlst = List::map1r(vlst.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone());
                    printVarList(varlst.clone());
                    println!("{}", (literal!("\n")).clone());
                    eqnlst = BackendEquation::getList(elst1.clone(), eqns.clone());
                    printEquationList(eqnlst.clone());
                    println!("{}", (literal!("\n")).clone());
                    eqnlst = BackendEquation::getList(elst.clone(), eqns.clone());
                    printEquationList(eqnlst.clone());
                    println!("{}", (literal!("\n")).clone());
                    dumpEqnsSolved2(rest.clone(), eqns.clone(), vars.clone())?;
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Casual torn ")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!(" Equationsystem:\n")); ArcStr::from(__mm_s) }).clone());
                    (elst1, vlst1Lst, _) = List::map_3(innerEquations2.clone(), (std::sync::Arc::new(BackendDAEUtil::getEqnAndVarsFromInnerEquation) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::InnerEquation) -> Result<(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<DAE::Constraint>>>)> + 'static>));
                    vlst1 = List::flatten(vlst1Lst.clone());
                    varlst = List::map1r(vlst1.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone());
                    printVarList(varlst.clone());
                    varlst = List::map1r(vlst2.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone());
                    printVarList(varlst.clone());
                    println!("{}", (literal!("\n")).clone());
                    eqnlst = BackendEquation::getList(elst1.clone(), eqns.clone());
                    printEquationList(eqnlst.clone());
                    println!("{}", (literal!("\n")).clone());
                    eqnlst = BackendEquation::getList(elst2.clone(), eqns.clone());
                    printEquationList(eqnlst.clone());
                    println!("{}", (literal!("\n")).clone());
                    dumpEqnsSolved2(rest.clone(), eqns.clone(), vars.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln((literal!("BackendDump.dumpEqnsSolved2 failed!")).clone())?;
                    dumpEqnsSolved2(rest.clone(), eqns.clone(), vars.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _, _) => {
                    dumpEqnsSolved2(rest.clone(), eqns.clone(), vars.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn dumpLoops(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = inDAE.clone();
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut vars: BackendDAE::Variables;
    let mut isyst: i32 = 1;
    let mut firstComp: bool = true;
    let _ = (::match_deref::match_deref! { match &(outDAE.shared.clone()) {
        Deref @ BackendDAE::Shared { backendDAEType: BackendDAE::BackendDAEType::SIMULATION, .. } => println!("{}", (literal!("SIMULATION\n")).clone()),
        Deref @ BackendDAE::Shared { backendDAEType: BackendDAE::BackendDAEType::INITIALSYSTEM, .. } => println!("{}", (literal!("INITIALSYSTEM\n")).clone()),
        _ => println!("{}", (literal!("UNKNOWN\n")).clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    for mut syst in &*inDAE.eqs.clone() {
        let mut syst = syst.clone();
        firstComp = true;
        let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(syst.clone()) {
            Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa0, .. }, orderedEqs: __pa1, orderedVars: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
            _ => bail!("pattern mismatch"),
        } };
        comps = __pa0.clone();
        eqns = __pa1.clone();
        vars = __pa2.clone();
        for mut comp in &*comps.clone() {
            let mut comp = comp.clone();
            if BackendEquation::isEquationsSystem(comp.clone()) || BackendEquation::isTornSystem(comp.clone()) {
                if firstComp.clone() {
                    firstComp = false;
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nsystem ")); __mm_s.push_str(&*intString(isyst.clone())); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                }
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n dumpLoops: SORTED COMPONENT \n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
                dumpEqnsSolved2(list![comp.clone()], eqns.clone(), vars.clone())?;
                if Flags::isSet(Flags::DUMP_LOOPS_VERBOSE.clone())? {
                    printComponentAdjacencyMatrixEnhanced(comp.clone(), eqns.clone(), vars.clone(), outDAE.shared.clone())?;
                }
            }
        }
        isyst = isyst.clone() + 1;
    }
    Ok(outDAE)
}

pub fn printComponentAdjacencyMatrixEnhanced(mut comp: Arc<BackendDAE::StrongComponent>, mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut vars: BackendDAE::Variables, mut shared: Arc<BackendDAE::Shared>) -> Result<()> {
    let mut compEqnLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut compVarLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut compEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut compVars: BackendDAE::Variables;
    let mut syst: Arc<BackendDAE::EqSystem>;
    let mut m: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
    let mut mT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
    (compVarLst, _, compEqnLst, _) = BackendDAEUtil::getStrongComponentVarsAndEquations(comp.clone(), vars.clone(), eqns.clone())?;
    compEqns = BackendEquation::listEquation(compEqnLst.clone())?;
    compVars = BackendVariable::listVar(compVarLst.clone());
    syst = BackendDAEUtil::createEqSystem(compVars.clone(), compEqns.clone(), metamodelica::nil(), crate::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
    (m, mT, _, _) = BackendDAEUtil::getAdjacencyMatrixEnhancedScalar(syst.clone(), shared.clone(), false)?;
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n dumpLoopsVerbose: UNSORTED COMPONENT WITH ENHANCED ADJACENCY MATRIX \n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    dumpVariables(compVars.clone(), (literal!("component variables")).clone())?;
    dumpEquationArray(compEqns.clone(), (literal!("component equations")).clone())?;
    dumpAdjacencyMatrixEnhanced(m.clone())?;
    println!("{}", (literal!("\n\n")).clone());
    dumpAdjacencyMatrixTEnhanced(mT.clone())?;
    Ok(())
}

pub fn dumpComponentsAdvanced(mut l: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut v2: metamodelica::Array<i32>, mut syst: Arc<BackendDAE::EqSystem>) -> Result<()> {
    let mut vars: BackendDAE::Variables;
    println!("{}", (literal!("Blocks\n")).clone());
    println!("{}", (literal!("=======\n")).clone());
    vars = BackendVariable::daeVars(syst.clone());
    dumpComponentsAdvanced2(l.clone(), 1, v2.clone(), vars.clone())?;
    Ok(())
}

fn dumpComponentsAdvanced2(mut inIntegerLstLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut inInteger: i32, mut v2: metamodelica::Array<i32>, mut vars: BackendDAE::Variables) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inIntegerLstLst.clone(), inInteger.clone(), v2.clone(), vars.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: l, tail: lst }, i, _, _) => {
            let mut i_1: i32 = 0;
            let mut ls: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut s: ArcStr = arcstr::literal!("");
            println!("{}", (literal!("{")).clone());
            ls = List::map(l.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
            s = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            println!("{}", (s.clone()).clone());
            println!("{}", (literal!("} ")).clone());
            dumpComponentsAdvanced3(l.clone(), v2.clone(), vars.clone())?;
            println!("{}", (literal!("\n")).clone());
            i_1 = i.clone() + 1;
            dumpComponentsAdvanced2(lst.clone(), i_1.clone(), v2.clone(), vars.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn dumpComponentsAdvanced3(mut inIntegerLst: Arc<metamodelica::List<i32>>, mut v2: metamodelica::Array<i32>, mut vars: BackendDAE::Variables) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inIntegerLst.clone(), v2.clone(), vars.clone())) {
        (Deref @ metamodelica::List::Nil, _, _) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: i, tail: Deref @ metamodelica::List::Nil }, _, _) => {
            let mut v: i32 = 0;
            let mut s: ArcStr = arcstr::literal!("");
            let mut c: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut var: BackendDAE::Var;
            let mut b: bool = false;
            v = v2.borrow()[(i.clone()-1) as usize].clone();
            var = BackendVariable::getVarAt(vars.clone(), v.clone())?;
            c = BackendVariable::varCref(var.clone())?;
            b = BackendVariable::isStateVar(var.clone());
            s = (if (b.clone()) {literal!("der(")} else {literal!("")}).clone();
            println!("{}", (s.clone()).clone());
            s = (ComponentReferenceBasics::printComponentRefStr(c.clone())?).clone();
            println!("{}", (s.clone()).clone());
            s = (if (b.clone()) {literal!(") ")} else {literal!(" ")}).clone();
            println!("{}", (s.clone()).clone());
            ()
        },
        (Deref @ metamodelica::List::Cons { head: i, tail: l }, _, _) => {
            let mut v: i32 = 0;
            let mut s: ArcStr = arcstr::literal!("");
            let mut c: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut var: BackendDAE::Var;
            let mut b: bool = false;
            v = v2.borrow()[(i.clone()-1) as usize].clone();
            var = BackendVariable::getVarAt(vars.clone(), v.clone())?;
            c = BackendVariable::varCref(var.clone())?;
            b = BackendVariable::isStateVar(var.clone());
            s = (if (b.clone()) {literal!("der(")} else {literal!("")}).clone();
            println!("{}", (s.clone()).clone());
            s = (ComponentReferenceBasics::printComponentRefStr(c.clone())?).clone();
            println!("{}", (s.clone()).clone());
            s = (if (b.clone()) {literal!(") ")} else {literal!(" ")}).clone();
            println!("{}", (s.clone()).clone());
            dumpComponentsAdvanced3(l.clone(), v2.clone(), vars.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn dumpComponents(mut inComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut inSyst: Option<Arc<BackendDAE::EqSystem>>) -> () {
    println!("{}", (literal!("StrongComponents\n")).clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    List::map1(inComps.clone(), (std::sync::Arc::new(dumpComponent) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::StrongComponent>, Option<Arc<BackendDAE::EqSystem>>) -> Result<()> + 'static>), inSyst.clone());
    ()
}

pub fn dumpComponent(mut inComp: Arc<BackendDAE::StrongComponent>, mut inSyst: Option<Arc<BackendDAE::EqSystem>>) -> Result<()> {
    println!("{}", (printComponent(inComp.clone(), inSyst.clone())?).clone());
    Ok(())
}

pub fn printComponent(mut inComp: Arc<BackendDAE::StrongComponent>, mut inSyst: Option<Arc<BackendDAE::EqSystem>>) -> Result<ArcStr> {
    let mut oString: ArcStr = arcstr::literal!("");
    let mut tmpStr: ArcStr = arcstr::literal!("");
    let mut tmpStr2: ArcStr = arcstr::literal!("");
    oString = ((::match_deref::match_deref! { match &(inComp.clone()) {
        Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { var: v, eqn: i } => {
            tmpStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*intString(v.clone())); __mm_s.push_str(&*literal!("}\n")); ArcStr::from(__mm_s) }).clone();
            tmpStr.clone()
        },
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jacType, vars: vlst, eqns: ilst, .. } => {
            let mut ls: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut s: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            ls = List::map(ilst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
            s = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            ls = List::map(vlst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
            s2 = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            tmpStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!("} Size: ")); __mm_s.push_str(&*intString((vlst.clone().len() as i32))); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*jacobianTypeStr(jacType.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            tmpStr.clone()
        },
        Deref @ BackendDAE::StrongComponent::SINGLEARRAY { vars: vlst, eqn: i } => {
            let mut ls: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut s: ArcStr = arcstr::literal!("");
            ls = List::map(vlst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
            s = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            tmpStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Array ")); __mm_s.push_str(&*literal!(" {{")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("}}\n")); ArcStr::from(__mm_s) }).clone();
            tmpStr.clone()
        },
        Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { vars: vlst, eqn: i } => {
            let mut ls: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut s: ArcStr = arcstr::literal!("");
            ls = List::map(vlst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
            s = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            tmpStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("IfEquation ")); __mm_s.push_str(&*literal!(" {{")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("}}\n")); ArcStr::from(__mm_s) }).clone();
            tmpStr.clone()
        },
        Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { vars: vlst, eqn: i } => {
            let mut ls: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut s: ArcStr = arcstr::literal!("");
            ls = List::map(vlst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
            s = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            tmpStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Algorithm ")); __mm_s.push_str(&*literal!(" {{")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("}}\n")); ArcStr::from(__mm_s) }).clone();
            tmpStr.clone()
        },
        Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { vars: vlst, eqn: i } => {
            let mut ls: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut s: ArcStr = arcstr::literal!("");
            ls = List::map(vlst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
            s = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            tmpStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ComplexEquation ")); __mm_s.push_str(&*literal!(" {")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("}\n")); ArcStr::from(__mm_s) }).clone();
            tmpStr.clone()
        },
        Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { vars: vlst, eqn: i } => {
            let mut ls: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut s: ArcStr = arcstr::literal!("");
            ls = List::map(vlst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
            s = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            tmpStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("WhenEquation ")); __mm_s.push_str(&*literal!(" {")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("}\n")); ArcStr::from(__mm_s) }).clone();
            tmpStr.clone()
        },
        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { linear: b, casualTearingSet: None, strictTearingSet: BackendDAE::TearingSet { innerEquations, tearingvars: vlst, residualequations: ilst, .. }, .. } => {
            let mut innerEqLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut innerVarLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut ls: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut s: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut s3: ArcStr = arcstr::literal!("");
            let mut s4: ArcStr = arcstr::literal!("");
            let mut eSys: Arc<BackendDAE::EqSystem>;
            ls = List::map(innerEquations.clone(), (std::sync::Arc::new(innerEquationString) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::InnerEquation) -> Result<ArcStr> + 'static>));
            s = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            ls = List::map(ilst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
            s2 = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            ls = List::map(vlst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
            s3 = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            s4 = (if (b.clone()) {literal!("linear")} else {literal!("nonlinear")}).clone();
            tmpStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{{")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("}\n,{")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*s3.clone()); __mm_s.push_str(&*literal!("}} Size: ")); __mm_s.push_str(&*intString((vlst.clone().len() as i32))); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*s4.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            if isSome(inSyst.clone()) {
                if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                    let __pa0 = ::match_deref::match_deref! { match &(inSyst.clone()) {
                        Some(__pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    eSys = __pa0.clone();
                    (innerEqLst, innerVarLst, _) = BackendDAEUtil::getEqnAndVarsFromInnerEquationLst(innerEquations.clone())?;
                    tmpStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmpStr.clone()); __mm_s.push_str(&*literal!("\nTearing Variables:\n-------------------------------------\n")); __mm_s.push_str(&*dumpMarkedVars(eSys.clone(), vlst.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("Residual Equations:\n-------------------------------------\n")); __mm_s.push_str(&*dumpMarkedEqns(eSys.clone(), ilst.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("Inner Variables:\n-------------------------------------\n")); __mm_s.push_str(&*dumpMarkedVarsLsts(eSys.clone(), innerVarLst.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("InnerEquations:\n-------------------------------------\n")); __mm_s.push_str(&*dumpMarkedEqns(eSys.clone(), innerEqLst.clone())?); ArcStr::from(__mm_s) }).clone();
                } else {
                    tmpStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmpStr.clone()); __mm_s.push_str(&*literal!("For more information please use \"-d=tearingdump\".\n")); ArcStr::from(__mm_s) }).clone();
                }
            }
            tmpStr.clone()
        },
        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { linear: b, casualTearingSet: Some(BackendDAE::TearingSet { innerEquations: innerEquations2, tearingvars: vlst2, residualequations: ilst2, .. }), strictTearingSet: BackendDAE::TearingSet { innerEquations, tearingvars: vlst, residualequations: ilst, .. }, .. } => {
            let mut innerEqLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut innerVarLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut ls: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut s: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut s3: ArcStr = arcstr::literal!("");
            let mut s4: ArcStr = arcstr::literal!("");
            let mut eSys: Arc<BackendDAE::EqSystem>;
            ls = List::map(innerEquations.clone(), (std::sync::Arc::new(innerEquationString) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::InnerEquation) -> Result<ArcStr> + 'static>));
            s = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            ls = List::map(ilst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
            s2 = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            ls = List::map(vlst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
            s3 = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            s4 = (if (b.clone()) {literal!("linear")} else {literal!("nonlinear")}).clone();
            tmpStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{{")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("}\n,{")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*s3.clone()); __mm_s.push_str(&*literal!("}} Size: ")); __mm_s.push_str(&*intString((vlst.clone().len() as i32))); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*s4.clone()); __mm_s.push_str(&*literal!(" (strict tearing set)\n")); ArcStr::from(__mm_s) }).clone();
            if isSome(inSyst.clone()) {
                if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                    let __pa0 = ::match_deref::match_deref! { match &(inSyst.clone()) {
                        Some(__pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    eSys = __pa0.clone();
                    (innerEqLst, innerVarLst, _) = BackendDAEUtil::getEqnAndVarsFromInnerEquationLst(innerEquations.clone())?;
                    tmpStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmpStr.clone()); __mm_s.push_str(&*literal!("\nTearing Variables:\n-------------------------------------\n")); __mm_s.push_str(&*dumpMarkedVars(eSys.clone(), vlst.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("Residual Equations:\n-------------------------------------\n")); __mm_s.push_str(&*dumpMarkedEqns(eSys.clone(), ilst.clone())?); __mm_s.push_str(&*literal!("Inner Variables:\n-------------------------------------\n")); __mm_s.push_str(&*dumpMarkedVarsLsts(eSys.clone(), innerVarLst.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("InnerEquations:\n-------------------------------------\n")); __mm_s.push_str(&*dumpMarkedEqns(eSys.clone(), innerEqLst.clone())?); ArcStr::from(__mm_s) }).clone();
                } else {
                    tmpStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmpStr.clone()); __mm_s.push_str(&*literal!("For more information please use \"-d=tearingdump\".\n")); ArcStr::from(__mm_s) }).clone();
                }
            }
            ls = List::map(innerEquations2.clone(), (std::sync::Arc::new(innerEquationString) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::InnerEquation) -> Result<ArcStr> + 'static>));
            s = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            ls = List::map(ilst2.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
            s2 = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            ls = List::map(vlst2.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
            s3 = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            s4 = (if (b.clone()) {literal!("linear")} else {literal!("nonlinear")}).clone();
            tmpStr2 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{{")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("}\n,{")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*s3.clone()); __mm_s.push_str(&*literal!("}} Size: ")); __mm_s.push_str(&*intString((vlst2.clone().len() as i32))); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*s4.clone()); __mm_s.push_str(&*literal!(" (casual tearing set)\n")); ArcStr::from(__mm_s) }).clone();
            if isSome(inSyst.clone()) {
                if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                    let __pa1 = ::match_deref::match_deref! { match &(inSyst.clone()) {
                        Some(__pa1) => __pa1.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    eSys = __pa1.clone();
                    (innerEqLst, innerVarLst, _) = BackendDAEUtil::getEqnAndVarsFromInnerEquationLst(innerEquations2.clone())?;
                    tmpStr2 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmpStr2.clone()); __mm_s.push_str(&*literal!("\nTearing Variables:\n-------------------------------------\n")); __mm_s.push_str(&*dumpMarkedVars(eSys.clone(), vlst2.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("Residual Equations:\n-------------------------------------\n")); __mm_s.push_str(&*dumpMarkedEqns(eSys.clone(), ilst2.clone())?); __mm_s.push_str(&*literal!("Inner Variables:\n-------------------------------------\n")); __mm_s.push_str(&*dumpMarkedVarsLsts(eSys.clone(), innerVarLst.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("InnerEquations:\n-------------------------------------\n")); __mm_s.push_str(&*dumpMarkedEqns(eSys.clone(), innerEqLst.clone())?); ArcStr::from(__mm_s) }).clone();
                } else {
                    tmpStr2 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmpStr2.clone()); __mm_s.push_str(&*literal!("For more information please use \"-d=tearingdump\".\n")); ArcStr::from(__mm_s) }).clone();
                }
            }
            { let mut __mm_s = String::new(); __mm_s.push_str(&*tmpStr.clone()); __mm_s.push_str(&*tmpStr2.clone()); ArcStr::from(__mm_s) }
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(oString)
}

pub fn dumpListList(mut lstLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut heading: ArcStr) -> () {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading.clone()); __mm_s.push_str(&*literal!(":\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*stringDelimitList(List::map(lstLst.clone(), (std::sync::Arc::new(fnptr!(intListStr, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    ()
}

// =============================================================================
// section for all *String functions
//
// These are functions, that return their output with a String.
//   - equationString
//   - strongComponentString
// =============================================================================
pub fn strongComponentString(mut inComp: Arc<BackendDAE::StrongComponent>) -> Result<ArcStr> {
    let mut outS: ArcStr = arcstr::literal!("");
    outS = ((::match_deref::match_deref! { match &(inComp.clone()) {
        Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { var: v, eqn: i } => {
            let mut s: ArcStr = arcstr::literal!("");
            let mut s1: ArcStr = arcstr::literal!("");
            s = (intString(i.clone())).clone();
            s1 = (intString(v.clone())).clone();
            s = stringAppendList(list![(literal!("{")).clone(), (s.clone()).clone(), (literal!(":")).clone(), (s1.clone()).clone(), (literal!("}")).clone()]);
            s.clone()
        },
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jacType, vars: vlst, eqns: ilst, .. } => {
            let mut ls: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut ls1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut s: ArcStr = arcstr::literal!("");
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut sl: ArcStr = arcstr::literal!("");
            let mut sj: ArcStr = arcstr::literal!("");
            ls = List::map(ilst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
            s = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            ls1 = List::map(vlst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
            s1 = stringDelimitList(ls1.clone(), (literal!(", ")).clone());
            sl = (intString((ilst.clone().len() as i32))).clone();
            sj = (jacobianTypeStr(jacType.clone())?).clone();
            s2 = stringAppendList(list![(literal!("{")).clone(), (s.clone()).clone(), (literal!(":")).clone(), (s1.clone()).clone(), (literal!("} Size: ")).clone(), (sl.clone()).clone(), (literal!(" ")).clone(), (sj.clone()).clone()]);
            s2.clone()
        },
        Deref @ BackendDAE::StrongComponent::SINGLEARRAY { vars: vlst, eqn: i } => {
            let mut ls: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut s: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut sl: ArcStr = arcstr::literal!("");
            ls = List::map(vlst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
            s = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            sl = (intString(i.clone())).clone();
            s2 = stringAppendList(list![(literal!("Array ")).clone(), (sl.clone()).clone(), (literal!(" {")).clone(), (s.clone()).clone(), (literal!("}")).clone()]);
            s2.clone()
        },
        Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { vars: vlst, eqn: i } => {
            let mut ls: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut s: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut sl: ArcStr = arcstr::literal!("");
            ls = List::map(vlst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
            s = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            sl = (intString(i.clone())).clone();
            s2 = stringAppendList(list![(literal!("Array ")).clone(), (sl.clone()).clone(), (literal!(" {")).clone(), (s.clone()).clone(), (literal!("}")).clone()]);
            s2.clone()
        },
        Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { vars: vlst, eqn: i } => {
            let mut ls: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut s: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut sl: ArcStr = arcstr::literal!("");
            ls = List::map(vlst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
            s = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            sl = (intString(i.clone())).clone();
            s2 = stringAppendList(list![(literal!("Algorithm ")).clone(), (sl.clone()).clone(), (literal!(" {")).clone(), (s.clone()).clone(), (literal!("}")).clone()]);
            s2.clone()
        },
        Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { vars: vlst, eqn: i } => {
            let mut ls: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut s: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut sl: ArcStr = arcstr::literal!("");
            ls = List::map(vlst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
            s = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            sl = (intString(i.clone())).clone();
            s2 = stringAppendList(list![(literal!("ComplexEquation ")).clone(), (sl.clone()).clone(), (literal!(" {")).clone(), (s.clone()).clone(), (literal!("}")).clone()]);
            s2.clone()
        },
        Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { vars: vlst, eqn: i } => {
            let mut ls: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut s: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut sl: ArcStr = arcstr::literal!("");
            ls = List::map(vlst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
            s = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            sl = (intString(i.clone())).clone();
            s2 = stringAppendList(list![(literal!("WhenEquation ")).clone(), (sl.clone()).clone(), (literal!(" {")).clone(), (s.clone()).clone(), (literal!("}")).clone()]);
            s2.clone()
        },
        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { linear: b, strictTearingSet: BackendDAE::TearingSet { innerEquations, tearingvars: vlst, residualequations: ilst, .. }, .. } => {
            let mut ls: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut s: ArcStr = arcstr::literal!("");
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut sl: ArcStr = arcstr::literal!("");
            let mut sj: ArcStr = arcstr::literal!("");
            ls = List::map(innerEquations.clone(), (std::sync::Arc::new(innerEquationString) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::InnerEquation) -> Result<ArcStr> + 'static>));
            s = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            ls = List::map(ilst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
            s1 = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            ls = List::map(vlst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
            s2 = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            sj = (intString((vlst.clone().len() as i32))).clone();
            sl = (if (b.clone()) {literal!("linear")} else {literal!("nonlinear")}).clone();
            s2 = stringAppendList(list![(literal!("torn ")).clone(), (sl.clone()).clone(), (literal!(" Equationsystem")).clone(), (literal!("{{")).clone(), (s.clone()).clone(), (literal!("},\n{")).clone(), (s1.clone()).clone(), (literal!(":")).clone(), (s2.clone()).clone(), (literal!("} Size: ")).clone(), (sj.clone()).clone()]);
            s2.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outS)
}

pub fn whenEquationString(mut inWhenEqn: Arc<BackendDAE::WhenEquation>, mut inStart: bool) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut conditionStr: ArcStr = arcstr::literal!("");
    let mut whenStmtStr: ArcStr = arcstr::literal!("");
    let mut elseWhenStr: ArcStr = arcstr::literal!("");
    let mut cond: Arc<DAE::Exp>;
    let mut weqn: Arc<BackendDAE::WhenEquation>;
    let mut oweqn: Option<Arc<BackendDAE::WhenEquation>> = None;
    let mut whenStmtLst: Arc<metamodelica::List<BackendDAE::WhenOperator>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inWhenEqn.clone()) {
        Deref @ BackendDAE::WhenEquation { elsewhenPart: __pa0, whenStmtLst: __pa1, condition: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    oweqn = __pa0.clone();
    whenStmtLst = __pa1.clone();
    cond = __pa2.clone();
    conditionStr = (ExpressionBasics::printExpStr(cond.clone())?).clone();
    whenStmtStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(List::map(whenStmtLst.clone(), (std::sync::Arc::new(dumpWhenOperatorStr) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::WhenOperator) -> Result<ArcStr> + 'static>)), (literal!(";\n  ")).clone())); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) }).clone();
    if isSome(oweqn.clone()) {
        let __pa3 = ::match_deref::match_deref! { match &(oweqn.clone()) {
            Some(__pa3) => __pa3.clone(),
            _ => bail!("pattern mismatch"),
        } };
        weqn = __pa3.clone();
        elseWhenStr = (whenEquationString(weqn.clone(), false)?).clone();
    } else {
        elseWhenStr = (literal!("")).clone();
    }
    if inStart.clone() {
        outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("when ")); __mm_s.push_str(&*conditionStr.clone()); __mm_s.push_str(&*literal!(" then\n  ")); __mm_s.push_str(&*whenStmtStr.clone()); __mm_s.push_str(&*elseWhenStr.clone()); __mm_s.push_str(&*literal!("end when;")); ArcStr::from(__mm_s) }).clone();
    } else {
        outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("elsewhen ")); __mm_s.push_str(&*conditionStr.clone()); __mm_s.push_str(&*literal!(" then\n  ")); __mm_s.push_str(&*whenStmtStr.clone()); __mm_s.push_str(&*elseWhenStr.clone()); ArcStr::from(__mm_s) }).clone();
    }
    Ok(outString)
}

pub fn equationString(mut inEquation: Arc<BackendDAE::Equation>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inEquation.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { scalar: e2, exp: e1, .. } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
            s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
            res = stringAppendList(list![(s1.clone()).clone(), (literal!(" = ")).clone(), (s2.clone()).clone()]);
            res.clone()
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: e2, left: e1, .. } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
            s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
            res = stringAppendList(list![(s1.clone()).clone(), (literal!(" = ")).clone(), (s2.clone()).clone()]);
            res.clone()
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { right: e2, left: e1, .. } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
            s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
            res = stringAppendList(list![(s1.clone()).clone(), (literal!(" = ")).clone(), (s2.clone()).clone()]);
            res.clone()
        },
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { exp: e2, componentRef: cr, .. } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
            s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
            res = stringAppendList(list![(s1.clone()).clone(), (literal!(" := ")).clone(), (s2.clone()).clone()]);
            res.clone()
        },
        Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: weqn, .. } => {
            let mut res: ArcStr = arcstr::literal!("");
            res = (whenEquationString(weqn.clone(), true)?).clone();
            res.clone()
        },
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (ExpressionBasics::printExpStr(e.clone())?).clone();
            res = stringAppendList(list![(s1.clone()).clone(), (literal!("= 0")).clone()]);
            res.clone()
        },
        Deref @ BackendDAE::Equation::ALGORITHM { source, alg, .. } => {
            let mut res: ArcStr = arcstr::literal!("");
            res = (DAEDump::dumpAlgorithmsStr(list![Arc::new(DAE::Element::ALGORITHM { algorithm_: alg.clone(), source: source.clone() })])?).clone();
            res.clone()
        },
        Deref @ BackendDAE::Equation::IF_EQUATION { eqnsfalse, eqnstrue: Deref @ metamodelica::List::Cons { head: eqns, tail: eqnstrue }, conditions: Deref @ metamodelica::List::Cons { head: e1, tail: expl }, .. } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut s3: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
            s2 = stringDelimitList(List::map(eqns.clone(), (std::sync::Arc::new(equationString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<ArcStr> + 'static>)), (literal!("\n  ")).clone());
            s3 = stringAppendList(list![(literal!("if ")).clone(), (s1.clone()).clone(), (literal!(" then\n  ")).clone(), (s2.clone()).clone()]);
            res = (ifequationString(expl.clone(), eqnstrue.clone(), eqnsfalse.clone(), (s3.clone()).clone())?).clone();
            res.clone()
        },
        Deref @ BackendDAE::Equation::FOR_EQUATION { body: eqn, stop, start, iter, .. } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ExpressionBasics::printExpStr(iter.clone())?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(start.clone())?); __mm_s.push_str(&*literal!(" : ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(stop.clone())?); ArcStr::from(__mm_s) }).clone();
            s2 = (equationString(eqn.clone())?).clone();
            res = stringAppendList(list![(literal!("for ")).clone(), (s1.clone()).clone(), (literal!(" loop\n    ")).clone(), (s2.clone()).clone(), (literal!("; end for; ")).clone()]);
            res.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

fn zeroCrossingString(mut inZeroCrossing: BackendDAE::ZeroCrossing) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inZeroCrossing.clone()) {
        BackendDAE::ZeroCrossing { occurEquLst: eq, relation_: e @ Deref @ DAE::Exp::RELATION { index: index_, .. }, .. } => {
            let mut eq_s_list: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut eq_s: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut str2: ArcStr = arcstr::literal!("");
            let mut str_index: ArcStr = arcstr::literal!("");
            eq_s_list = List::map(eq.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
            eq_s = stringDelimitList(eq_s_list.clone(), (literal!(",")).clone());
            r#str = (ExpressionBasics::printExpStr(e.clone())?).clone();
            str_index = (intString(index_.clone())).clone();
            str2 = stringAppendList(list![(r#str.clone()).clone(), (literal!(" with index = ")).clone(), (str_index.clone()).clone(), (literal!(" in equations [")).clone(), (eq_s.clone()).clone(), (literal!("]")).clone()]);
            str2.clone()
        },
        BackendDAE::ZeroCrossing { occurEquLst: eq, relation_: e @ Deref @ DAE::Exp::LBINARY { .. }, .. } => {
            let mut eq_s_list: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut eq_s: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut str2: ArcStr = arcstr::literal!("");
            eq_s_list = List::map(eq.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
            eq_s = stringDelimitList(eq_s_list.clone(), (literal!(",")).clone());
            r#str = (ExpressionBasics::printExpStr(e.clone())?).clone();
            str2 = stringAppendList(list![(r#str.clone()).clone(), (literal!(" in equations [")).clone(), (eq_s.clone()).clone(), (literal!("]")).clone()]);
            str2.clone()
        },
        BackendDAE::ZeroCrossing { occurEquLst: eq, relation_: e @ Deref @ DAE::Exp::LUNARY { .. }, .. } => {
            let mut eq_s_list: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut eq_s: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut str2: ArcStr = arcstr::literal!("");
            eq_s_list = List::map(eq.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
            eq_s = stringDelimitList(eq_s_list.clone(), (literal!(",")).clone());
            r#str = (ExpressionBasics::printExpStr(e.clone())?).clone();
            str2 = stringAppendList(list![(r#str.clone()).clone(), (literal!(" in equations [")).clone(), (eq_s.clone()).clone(), (literal!("]")).clone()]);
            str2.clone()
        },
        BackendDAE::ZeroCrossing { occurEquLst: eq, relation_: e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { .. }, .. }, .. } => {
            let mut eq_s_list: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut eq_s: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut str2: ArcStr = arcstr::literal!("");
            eq_s_list = List::map(eq.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
            eq_s = stringDelimitList(eq_s_list.clone(), (literal!(",")).clone());
            r#str = (ExpressionBasics::printExpStr(e.clone())?).clone();
            str2 = stringAppendList(list![(r#str.clone()).clone(), (literal!(" in equations [")).clone(), (eq_s.clone()).clone(), (literal!("]")).clone()]);
            str2.clone()
        },
        _ => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

fn timeEventString(mut inTimeEvent: BackendDAE::TimeEvent) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inTimeEvent.clone() {
        BackendDAE::TimeEvent::SIMPLE_TIME_EVENT => literal!("SIMPLE_TIME_EVENT"),
        BackendDAE::TimeEvent::SAMPLE_TIME_EVENT { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*intString(var_field!(inTimeEvent.index, BackendDAE::TimeEvent::SAMPLE_TIME_EVENT).clone())); __mm_s.push_str(&*literal!(": sample(")); __mm_s.push_str(&*ExpressionBasics::printExpStr(var_field!(inTimeEvent.startExp, BackendDAE::TimeEvent::SAMPLE_TIME_EVENT).clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(var_field!(inTimeEvent.intervalExp, BackendDAE::TimeEvent::SAMPLE_TIME_EVENT).clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
        _ => literal!("unknown time event"),
    })).clone();
    Ok(outString)
}

pub fn simIteratorString(mut iter: BackendDAE::SimIterator) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match iter.clone() {
        BackendDAE::SimIterator::SIM_ITERATOR_RANGE { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(var_field!(iter.name, BackendDAE::SimIterator::SIM_ITERATOR_RANGE).clone())?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(var_field!(iter.start, BackendDAE::SimIterator::SIM_ITERATOR_RANGE).clone())?); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*ExpressionBasics::printExpStr(var_field!(iter.step, BackendDAE::SimIterator::SIM_ITERATOR_RANGE).clone())?); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*ExpressionBasics::printExpStr(var_field!(iter.stop, BackendDAE::SimIterator::SIM_ITERATOR_RANGE).clone())?); ArcStr::from(__mm_s) },
        BackendDAE::SimIterator::SIM_ITERATOR_LIST { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(var_field!(iter.name, BackendDAE::SimIterator::SIM_ITERATOR_LIST).clone())?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*List::toString(var_field!(iter.lst, BackendDAE::SimIterator::SIM_ITERATOR_LIST).clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 10)?); ArcStr::from(__mm_s) },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

// =============================================================================
// section for all debug* functions
//
// description: ???
// =============================================================================
pub fn debugStrCrefLstStr(mut a: ArcStr, mut b: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut c: ArcStr, mut d: ArcStr) -> () {
    println!("{}", (a.clone()).clone());
    debuglst(b.clone(), (std::sync::Arc::new(ComponentReferenceBasics::printComponentRefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>), (c.clone()).clone(), (d.clone()).clone());
    ()
}

pub fn debugCrefStr(mut a: Arc<DAE::ComponentRef>, mut b: ArcStr) -> Result<()> {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(a.clone())?); __mm_s.push_str(&*b.clone()); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub fn debugStrIntStr(mut a: ArcStr, mut b: i32, mut c: ArcStr) -> () {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*a.clone()); __mm_s.push_str(&*intString(b.clone())); __mm_s.push_str(&*c.clone()); ArcStr::from(__mm_s) }).clone());
    ()
}

pub fn debugStrIntStrIntStr(mut a: ArcStr, mut b: i32, mut c: ArcStr, mut d: i32, mut e: ArcStr) -> () {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*a.clone()); __mm_s.push_str(&*intString(b.clone())); __mm_s.push_str(&*c.clone()); __mm_s.push_str(&*intString(d.clone())); __mm_s.push_str(&*e.clone()); ArcStr::from(__mm_s) }).clone());
    ()
}

pub fn debugCrefStrIntStr(mut a: Arc<DAE::ComponentRef>, mut b: ArcStr, mut c: i32, mut d: ArcStr) -> Result<()> {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(a.clone())?); __mm_s.push_str(&*b.clone()); __mm_s.push_str(&*intString(c.clone())); __mm_s.push_str(&*d.clone()); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub fn debugStrCrefStr(mut a: ArcStr, mut b: Arc<DAE::ComponentRef>, mut c: ArcStr) -> Result<()> {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*a.clone()); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(b.clone())?); __mm_s.push_str(&*c.clone()); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub fn debugStrCrefStrIntStr(mut a: ArcStr, mut b: Arc<DAE::ComponentRef>, mut c: ArcStr, mut d: i32, mut e: ArcStr) -> Result<()> {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*a.clone()); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(b.clone())?); __mm_s.push_str(&*c.clone()); __mm_s.push_str(&*intString(d.clone())); __mm_s.push_str(&*e.clone()); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub fn debugStrCrefStrRealStrRealStrRealStr(mut a: ArcStr, mut b: Arc<DAE::ComponentRef>, mut c: ArcStr, mut d: metamodelica::Real, mut e: ArcStr, mut f: metamodelica::Real, mut g: ArcStr, mut h: metamodelica::Real, mut i: ArcStr) -> Result<()> {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*a.clone()); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(b.clone())?); __mm_s.push_str(&*c.clone()); __mm_s.push_str(&*realString(d.clone())); __mm_s.push_str(&*e.clone()); __mm_s.push_str(&*realString(f.clone())); __mm_s.push_str(&*g.clone()); __mm_s.push_str(&*realString(h.clone())); __mm_s.push_str(&*i.clone()); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub fn debugStrRealStrRealStrRealStrRealStr(mut a: ArcStr, mut b: metamodelica::Real, mut c: ArcStr, mut d: metamodelica::Real, mut e: ArcStr, mut f: metamodelica::Real, mut g: ArcStr, mut h: metamodelica::Real, mut i: ArcStr) -> () {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*a.clone()); __mm_s.push_str(&*realString(b.clone())); __mm_s.push_str(&*c.clone()); __mm_s.push_str(&*realString(d.clone())); __mm_s.push_str(&*e.clone()); __mm_s.push_str(&*realString(f.clone())); __mm_s.push_str(&*g.clone()); __mm_s.push_str(&*realString(h.clone())); __mm_s.push_str(&*i.clone()); ArcStr::from(__mm_s) }).clone());
    ()
}

pub fn debugStrCrefStrExpStr(mut a: ArcStr, mut b: Arc<DAE::ComponentRef>, mut c: ArcStr, mut d: Arc<DAE::Exp>, mut e: ArcStr) -> Result<()> {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*a.clone()); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(b.clone())?); __mm_s.push_str(&*c.clone()); __mm_s.push_str(&*ExpressionBasics::printExpStr(d.clone())?); __mm_s.push_str(&*e.clone()); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub fn debugStrCrefStrCrefStr(mut a: ArcStr, mut b: Arc<DAE::ComponentRef>, mut c: ArcStr, mut d: Arc<DAE::ComponentRef>, mut e: ArcStr) -> Result<()> {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*a.clone()); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(b.clone())?); __mm_s.push_str(&*c.clone()); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(d.clone())?); __mm_s.push_str(&*e.clone()); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub fn debugExpStr(mut a: Arc<DAE::Exp>, mut b: ArcStr) -> Result<()> {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ExpressionBasics::printExpStr(a.clone())?); __mm_s.push_str(&*b.clone()); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub fn debugStrExpStr(mut a: ArcStr, mut b: Arc<DAE::Exp>, mut c: ArcStr) -> Result<()> {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*a.clone()); __mm_s.push_str(&*ExpressionBasics::printExpStr(b.clone())?); __mm_s.push_str(&*c.clone()); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub fn debugStrExpLstStr(mut a: ArcStr, mut b: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut c: ArcStr, mut d: ArcStr) -> () {
    println!("{}", (a.clone()).clone());
    debuglst(b.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>), (c.clone()).clone(), (d.clone()).clone());
    ()
}

pub fn debugStrExpStrCrefStr(mut a: ArcStr, mut b: Arc<DAE::Exp>, mut c: ArcStr, mut d: Arc<DAE::ComponentRef>, mut e: ArcStr) -> Result<()> {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*a.clone()); __mm_s.push_str(&*ExpressionBasics::printExpStr(b.clone())?); __mm_s.push_str(&*c.clone()); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(d.clone())?); __mm_s.push_str(&*e.clone()); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub fn debugStrExpStrExpStr(mut a: ArcStr, mut b: Arc<DAE::Exp>, mut c: ArcStr, mut d: Arc<DAE::Exp>, mut e: ArcStr) -> Result<()> {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*a.clone()); __mm_s.push_str(&*ExpressionBasics::printExpStr(b.clone())?); __mm_s.push_str(&*c.clone()); __mm_s.push_str(&*ExpressionBasics::printExpStr(d.clone())?); __mm_s.push_str(&*e.clone()); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub fn debugExpStrExpStrExpStr(mut a: Arc<DAE::Exp>, mut b: ArcStr, mut c: Arc<DAE::Exp>, mut d: ArcStr, mut e: Arc<DAE::Exp>, mut f: ArcStr) -> Result<()> {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ExpressionBasics::printExpStr(a.clone())?); __mm_s.push_str(&*b.clone()); __mm_s.push_str(&*ExpressionBasics::printExpStr(c.clone())?); __mm_s.push_str(&*d.clone()); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*f.clone()); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub fn debugStrExpStrExpStrExpStr(mut a: ArcStr, mut b: Arc<DAE::Exp>, mut c: ArcStr, mut d: Arc<DAE::Exp>, mut e: ArcStr, mut f: Arc<DAE::Exp>, mut g: ArcStr) -> Result<()> {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*a.clone()); __mm_s.push_str(&*ExpressionBasics::printExpStr(b.clone())?); __mm_s.push_str(&*c.clone()); __mm_s.push_str(&*ExpressionBasics::printExpStr(d.clone())?); __mm_s.push_str(&*e.clone()); __mm_s.push_str(&*ExpressionBasics::printExpStr(f.clone())?); __mm_s.push_str(&*g.clone()); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub fn debugStrEqnStr(mut a: ArcStr, mut b: Arc<BackendDAE::Equation>, mut c: ArcStr) -> Result<()> {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*a.clone()); __mm_s.push_str(&*equationString(b.clone())?); __mm_s.push_str(&*c.clone()); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub fn debugStrEqnStrEqnStr(mut a: ArcStr, mut b: Arc<BackendDAE::Equation>, mut c: ArcStr, mut d: Arc<BackendDAE::Equation>, mut e: ArcStr) -> Result<()> {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*a.clone()); __mm_s.push_str(&*equationString(b.clone())?); __mm_s.push_str(&*c.clone()); __mm_s.push_str(&*equationString(d.clone())?); __mm_s.push_str(&*e.clone()); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub fn debuglst<Type_a: Clone + 'static>(mut lst: Arc<metamodelica::List<Type_a>>, mut f: Arc<dyn ::std::ops::Fn(Type_a) -> Result<ArcStr> + 'static>, mut c: ArcStr, mut se: ArcStr) -> () {
    pub type FuncTypeType_aToStr<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Type_a) -> Result<ArcStr> + 'static>;

    let _ = (::match_deref::match_deref! { match &(lst.clone()) {
        Deref @ metamodelica::List::Nil => {
            println!("{}", (se.clone()).clone());
            ()
        },
        Deref @ metamodelica::List::Cons { head: a, tail: Deref @ metamodelica::List::Nil } => {
            println!("{}", (f(a.clone()).unwrap()).clone());
            println!("{}", (se.clone()).clone());
            ()
        },
        Deref @ metamodelica::List::Cons { head: a, tail: rest } => {
            println!("{}", (f(a.clone()).unwrap()).clone());
            println!("{}", (c.clone()).clone());
            debuglst(rest.clone(), f.clone(), (c.clone()).clone(), (se.clone()).clone());
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ()
}

// =============================================================================
// unsorted section
//
// These section should be empty. Feel free to sort these functions into one of
// the upper sections.
// =============================================================================
pub fn printCallFunction2StrDIVISION<Type_a: Clone + 'static>(mut inExp: Arc<DAE::Exp>, mut stringDelimiter: ArcStr, mut opcreffunc: Option<(Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Type_a) -> Result<ArcStr> + 'static>, Type_a)>) -> Result<ArcStr> {
    pub type strongComponentStringRefStrFunc<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Type_a) -> Result<ArcStr> + 'static>;

    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = (inExp.clone(), stringDelimiter.clone(), opcreffunc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty, .. }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::SCONST { string: _ }, tail: Deref @ metamodelica::List::Nil } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "DIVISION" } }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = (ExpressionDump::printExp2Str(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::DIV { ty: ty.clone() }, exp2: e2.clone() }), (stringDelimiter.clone()).clone(), opcreffunc.clone(), Some((std::sync::Arc::new(printCallFunction2StrDIVISION) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArcStr, _) -> Result<ArcStr> + 'static>)))?).clone();
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty, .. }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::SCONST { string: _ }, tail: Deref @ metamodelica::List::Nil } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "DIVISION_ARRAY_SCALAR" } }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = (ExpressionDump::printExp2Str(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::DIV_ARRAY_SCALAR { ty: ty.clone() }, exp2: e2.clone() }), (stringDelimiter.clone()).clone(), opcreffunc.clone(), Some((std::sync::Arc::new(printCallFunction2StrDIVISION) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArcStr, _) -> Result<ArcStr> + 'static>)))?).clone();
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty, .. }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::SCONST { string: _ }, tail: Deref @ metamodelica::List::Nil } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "DIVISION_SCALAR_ARRAY" } }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = (ExpressionDump::printExp2Str(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::DIV_SCALAR_ARRAY { ty: ty.clone() }, exp2: e2.clone() }), (stringDelimiter.clone()).clone(), opcreffunc.clone(), Some((std::sync::Arc::new(printCallFunction2StrDIVISION) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArcStr, _) -> Result<ArcStr> + 'static>)))?).clone();
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: args, path: fcn, .. }, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut s_1: ArcStr = arcstr::literal!("");
                    let mut s_2: ArcStr = arcstr::literal!("");
                    let mut fs: ArcStr = arcstr::literal!("");
                    let mut argstr: ArcStr = arcstr::literal!("");
                    fs = (AbsynUtil::pathString(fcn.clone(), (literal!(".")).clone(), true, false)?).clone();
                    argstr = stringDelimitList(List::map3(args.clone(), (std::sync::Arc::new(ExpressionDump::printExp2Str) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArcStr, _, _) -> Result<ArcStr> + 'static>), (stringDelimiter.clone()).clone(), opcreffunc.clone(), Some((std::sync::Arc::new(printCallFunction2StrDIVISION) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArcStr, _) -> Result<ArcStr> + 'static>))), (literal!(",")).clone());
                    s = (stringAppend((fs.clone()).clone(), (literal!("(")).clone())).clone();
                    s_1 = (stringAppend((s.clone()).clone(), (argstr.clone()).clone())).clone();
                    s_2 = (stringAppend((s_1.clone()).clone(), (literal!(")")).clone())).clone();
                    Ok(s_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

// protected function printVarsStatistics "author: PA
//
//   Prints statistics on variables, etc.
// "
//   input BackendDAE.Variables inVariables1;
//   input BackendDAE.Variables inVariables2;
// algorithm
//   _:=
//   matchcontinue (inVariables1,inVariables2)
//     local
//       String lenstr,bstr;
//       BackendDAE.VariableArray v1,v2;
//       Integer bsize1,n1,bsize2,n2;
//     case (BackendDAE.VARIABLES(varArr = v1,bucketSize = bsize1,numberOfVars = n1),BackendDAE.VARIABLES(varArr = v2,bucketSize = bsize2,numberOfVars = n2))
//       equation
//         print("Variable Statistics\n");
//         print("===================\n");
//         print("Number of variables: ");
//         lenstr = intString(n1);
//         print(lenstr);
//         print("\n");
//         print("Bucket size for variables: ");
//         bstr = intString(bsize1);
//         print(bstr);
//         print("\n");
//         print("Number of known variables: ");
//         lenstr = intString(n2);
//         print(lenstr);
//         print("\n");
//         print("Bucket size for known variables: ");
//         bstr = intString(bsize1);
//         print(bstr);
//         print("\n");
//       then
//         ();
//   end matchcontinue;
// end printVarsStatistics;
pub fn dumpWhenOperatorStr(mut inWhenOperator: BackendDAE::WhenOperator) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inWhenOperator.clone() {
        BackendDAE::WhenOperator::ASSIGN { right: ref e, left: ref e1, .. } => {
            let mut scr: ArcStr = arcstr::literal!("");
            let mut se: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            scr = (ExpressionBasics::printExpStr(e1.clone())?).clone();
            se = (ExpressionBasics::printExpStr(e.clone())?).clone();
            r#str = stringAppendList(list![(scr.clone()).clone(), (literal!(" := ")).clone(), (se.clone()).clone()]);
            r#str.clone()
        },
        BackendDAE::WhenOperator::REINIT { value: ref e, stateVar: ref cr, .. } => {
            let mut scr: ArcStr = arcstr::literal!("");
            let mut se: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            scr = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
            se = (ExpressionBasics::printExpStr(e.clone())?).clone();
            r#str = stringAppendList(list![(literal!("reinit(")).clone(), (scr.clone()).clone(), (literal!(",")).clone(), (se.clone()).clone(), (literal!(")")).clone()]);
            r#str.clone()
        },
        BackendDAE::WhenOperator::ASSERT { message: ref e1, condition: ref e, .. } => {
            let mut se: ArcStr = arcstr::literal!("");
            let mut se1: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            se = (ExpressionBasics::printExpStr(e.clone())?).clone();
            se1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
            r#str = stringAppendList(list![(literal!("assert(")).clone(), (se.clone()).clone(), (literal!(",")).clone(), (se1.clone()).clone(), (literal!(")")).clone()]);
            r#str.clone()
        },
        BackendDAE::WhenOperator::TERMINATE { message: ref e, .. } => {
            let mut se: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            se = (ExpressionBasics::printExpStr(e.clone())?).clone();
            r#str = stringAppendList(list![(literal!("terminate(")).clone(), (se.clone()).clone(), (literal!(")")).clone()]);
            r#str.clone()
        },
        BackendDAE::WhenOperator::NORETCALL { exp: ref e, .. } => {
            ExpressionBasics::printExpStr(e.clone())?
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn dumpOption<Type_A: Clone + 'static>(mut inType: Option<Type_A>, mut infunc: Arc<dyn ::std::ops::Fn(Type_A) -> Result<()> + 'static>) -> () {
    pub type printType_A<Type_A: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Type_A) -> Result<()> + 'static>;

    let _ = (match (inType.clone(), infunc.clone()) {
        (Some(mut a), _) => {
            infunc(a.clone()).unwrap();
            ()
        },
        _ => {
            ()
        },
    });
    ()
}

pub fn dumpAlgorithms(mut ialgs: Arc<metamodelica::List<Arc<DAE::Algorithm>>>, mut indx: i32) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((ialgs.clone(), indx.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Algorithm { statementLst: stmts }, tail: algs }, _) => {
            let mut myStream: IOStream::IOStream;
            let mut is: ArcStr = arcstr::literal!("");
            is = (intString(indx.clone())).clone();
            myStream = IOStream::create((literal!("")).clone(), openmodelica_util::IOStream::IOStreamType::LIST)?;
            myStream = IOStream::append(myStream.clone(), (stringAppend((is.clone()).clone(), (literal!(". ")).clone())).clone())?;
            myStream = DAEDump::dumpAlgorithmStream(Arc::new(DAE::Element::ALGORITHM { algorithm_: Arc::new(DAE::Algorithm { statementLst: stmts.clone() }), source: DAE::emptyElementSource().clone() }), myStream.clone())?;
            IOStream::print(myStream.clone(), IOStream::stdOutput.clone())?;
            dumpAlgorithms(algs.clone(), indx.clone() + 1)?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn dumpConstraints(mut ionstrs: Arc<metamodelica::List<Arc<DAE::Constraint>>>, mut indx: i32) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((ionstrs.clone(), indx.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Constraint::CONSTRAINT_EXPS { constraintLst: exps }, tail: constrs }, _) => {
            let mut myStream: IOStream::IOStream;
            let mut is: ArcStr = arcstr::literal!("");
            is = (intString(indx.clone())).clone();
            myStream = IOStream::create((literal!("")).clone(), openmodelica_util::IOStream::IOStreamType::LIST)?;
            myStream = IOStream::append(myStream.clone(), (stringAppend((is.clone()).clone(), (literal!(". ")).clone())).clone())?;
            myStream = DAEDump::dumpConstraintStream(list![Arc::new(DAE::Element::CONSTRAINT { constraints: Arc::new(DAE::Constraint::CONSTRAINT_EXPS { constraintLst: exps.clone() }), source: DAE::emptyElementSource().clone() })], myStream.clone())?;
            IOStream::print(myStream.clone(), IOStream::stdOutput.clone())?;
            dumpConstraints(constrs.clone(), indx.clone() + 1)?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn dumpSparsePatternArray(mut inSparsePatter: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Print sparse pattern: ")); __mm_s.push_str(&*intString((inSparsePatter.clone().borrow().len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    dumpSparsePattern2(Arc::new(inSparsePatter.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), 1)?;
    println!("{}", (literal!("\n")).clone());
    Ok(())
}

pub fn dumpSparsePattern(mut inSparsePatter: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<()> {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Print sparse pattern: ")); __mm_s.push_str(&*intString((inSparsePatter.clone().len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    dumpSparsePattern2(inSparsePatter.clone(), 1)?;
    println!("{}", (literal!("\n")).clone());
    Ok(())
}

pub fn dumpSparsePattern2(mut inSparsePatter: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut inInteger: i32) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inSparsePatter.clone(), inInteger.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: elem, tail: rest }, _) => {
            let mut sparsepatternStr: ArcStr = arcstr::literal!("");
            sparsepatternStr = (List::toString(elem.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Row[")); __mm_s.push_str(&*intString(inInteger.clone())); __mm_s.push_str(&*literal!("] = ")); ArcStr::from(__mm_s) }).clone(), (literal!("{")).clone(), (literal!(";")).clone(), (literal!("}")).clone(), true, 0)?).clone();
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*sparsepatternStr.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            dumpSparsePattern2(rest.clone(), inInteger.clone() + 1)?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn dumpJacobianStr(mut inTplIntegerIntegerEquationLstOption: Option<Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inTplIntegerIntegerEquationLstOption.clone()) {
        Some(eqns) => {
            let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut res_1: ArcStr = arcstr::literal!("");
            res = dumpJacobianStr2(eqns.clone())?;
            res_1 = stringDelimitList(res.clone(), (literal!(",\n")).clone());
            res_1.clone()
        },
        None => {
            literal!("No analytic jacobian available\n")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

fn dumpJacobianStr2(mut inTplIntegerIntegerEquationLst: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outStringLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outStringLst = (::match_deref::match_deref! { match &(inTplIntegerIntegerEquationLst.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: (row, col, Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. }), tail: eqns } => {
            let mut estr: ArcStr = arcstr::literal!("");
            let mut rowstr: ArcStr = arcstr::literal!("");
            let mut colstr: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            estr = (ExpressionBasics::printExpStr(e.clone())?).clone();
            rowstr = (intString(row.clone())).clone();
            colstr = (intString(col.clone())).clone();
            r#str = stringAppendList(list![(literal!("{")).clone(), (rowstr.clone()).clone(), (literal!(",")).clone(), (colstr.clone()).clone(), (literal!("}:")).clone(), (estr.clone()).clone()]);
            strs = dumpJacobianStr2(eqns.clone())?;
            cons((r#str.clone()).clone(), strs.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outStringLst)
}

pub fn jacobianTypeStr(mut inJacobianType: BackendDAE::JacobianType) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inJacobianType.clone() {
        BackendDAE::JacobianType::JAC_CONSTANT => literal!("Jacobian Constant"),
        BackendDAE::JacobianType::JAC_LINEAR => literal!("Jacobian Linear"),
        BackendDAE::JacobianType::JAC_NONLINEAR => literal!("Jacobian Nonlinear"),
        BackendDAE::JacobianType::JAC_GENERIC => literal!("Generic Jacobian via directional derivatives"),
        BackendDAE::JacobianType::JAC_NO_ANALYTIC => literal!("No analytic jacobian"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn dumpJacobianString(mut jacIn: Arc<BackendDAE::Jacobian>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(jacIn.clone()) {
        Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: fJac } => {
            let mut s: ArcStr = arcstr::literal!("");
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("###############\n")); __mm_s.push_str(&*literal!(" FULL_JACOBIAN \n")); __mm_s.push_str(&*literal!("###############\n\n")); __mm_s.push_str(&*dumpJacobianStr(fJac.clone())?); ArcStr::from(__mm_s) }).clone();
            println!("{}", (s.clone()).clone());
            literal!("")
        },
        Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { sparsePattern, jacobian: Some(sJac), .. } => {
            let mut dae: Arc<BackendDAE::BackendDAE>;
            (dae, _, _, _, _, _) = sJac.clone();
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("##################\n")); __mm_s.push_str(&*literal!(" GENERIC_JACOBIAN \n")); __mm_s.push_str(&*literal!("##################\n\n")); ArcStr::from(__mm_s) }).clone());
            dumpBackendDAE(dae.clone(), (literal!("Directional Derivatives System")).clone())?;
            dumpSparsityPattern(sparsePattern.clone(), (literal!("Sparse Pattern")).clone())?;
            literal!("")
        },
        Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { sparsePattern, jacobian: None, .. } => {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("##################\n")); __mm_s.push_str(&*literal!(" GENERIC_JACOBIAN \n")); __mm_s.push_str(&*literal!("##################\n\n")); ArcStr::from(__mm_s) }).clone());
            dumpSparsityPattern(sparsePattern.clone(), (literal!("Sparse Pattern")).clone())?;
            literal!("")
        },
        Deref @ BackendDAE::Jacobian::EMPTY_JACOBIAN => {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("################\n")); __mm_s.push_str(&*literal!(" EMPTY_JACOBIAN \n")); __mm_s.push_str(&*literal!("################\n\n")); ArcStr::from(__mm_s) }).clone());
            literal!("")
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn symJacString(mut jacIn: (Option<(Arc<BackendDAE::BackendDAE>, ArcStr, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32), Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>)) -> Result<ArcStr> {
    let mut sOut: ArcStr = arcstr::literal!("");
    sOut = ((::match_deref::match_deref! { match &(jacIn.clone()) {
        (Some(sJac), sparsePattern, _) => {
            let mut dae: Arc<BackendDAE::BackendDAE>;
            let mut s: ArcStr = arcstr::literal!("");
            (dae, _, _, _, _, _) = sJac.clone();
            s = (literal!("GENERIC JACOBIAN:\n")).clone();
            dumpBackendDAE(dae.clone(), (literal!("Directional Derivatives System")).clone())?;
            dumpSparsityPattern(sparsePattern.clone(), (literal!("Sparse Pattern")).clone())?;
            s.clone()
        },
        (None, sparsePattern, _) => {
            let mut s: ArcStr = arcstr::literal!("");
            s = (literal!("GENERIC JACOBIAN:\n")).clone();
            dumpSparsityPattern(sparsePattern.clone(), (literal!("Sparse Pattern")).clone())?;
            s.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(sOut)
}

pub fn dumpEqnsStr(mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = stringDelimitList(dumpEqnsStr2(eqns.clone(), 1, metamodelica::nil())?, (literal!("\n")).clone());
    Ok(r#str)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn dumpEqnsStr2(mut inEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inInteger: i32, mut inAcc: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    strs = (::match_deref::match_deref! { match &((inEquationLst.clone(), inInteger.clone(), inAcc.clone())) {
        (Deref @ metamodelica::List::Nil, _, acc) => {
            acc.clone().reverse()
        },
        (Deref @ metamodelica::List::Cons { head: eqn, tail: eqns }, index, acc) => {
            let mut es: ArcStr = arcstr::literal!("");
            let mut is: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut index_1: i32 = 0;
            let mut acc = (*acc).clone();
            es = (equationString(eqn.clone())?).clone();
            is = (intString(index.clone())).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*is.clone()); __mm_s.push_str(&*literal!(" : ")); __mm_s.push_str(&*es.clone()); ArcStr::from(__mm_s) }).clone();
            index_1 = index.clone() + 1;
            acc = cons((r#str.clone()).clone(), acc.clone());
            dumpEqnsStr2(eqns.clone(), index_1.clone(), acc.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(strs)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn ifequationString(mut conditions: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut eqnstrue: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, mut eqnsfalse: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut iString: ArcStr) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &((conditions.clone(), eqnstrue.clone(), eqnsfalse.clone(), iString.clone())) {
        (Deref @ metamodelica::List::Nil, _, Deref @ metamodelica::List::Nil, _) => {
            let mut s: ArcStr = arcstr::literal!("");
            s = stringAppendList(list![(iString.clone()).clone(), (literal!("\nend if")).clone()]);
            s.clone()
        },
        (Deref @ metamodelica::List::Nil, _, _, _) => {
            let mut seqns: ArcStr = arcstr::literal!("");
            let mut s: ArcStr = arcstr::literal!("");
            seqns = stringDelimitList(List::map(eqnsfalse.clone(), (std::sync::Arc::new(equationString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<ArcStr> + 'static>)), (literal!("\n  ")).clone());
            s = stringAppendList(list![(iString.clone()).clone(), (literal!("\nelse\n  ")).clone(), (seqns.clone()).clone(), (literal!("\nend if")).clone()]);
            s.clone()
        },
        (Deref @ metamodelica::List::Cons { head: e, tail: elst }, Deref @ metamodelica::List::Cons { head: eqns, tail: eqnslst }, _, _) => {
            let mut seqns: ArcStr = arcstr::literal!("");
            let mut s: ArcStr = arcstr::literal!("");
            let mut se: ArcStr = arcstr::literal!("");
            se = (ExpressionBasics::printExpStr(e.clone())?).clone();
            seqns = stringDelimitList(List::map(eqns.clone(), (std::sync::Arc::new(equationString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<ArcStr> + 'static>)), (literal!("\n  ")).clone());
            s = stringAppendList(list![(iString.clone()).clone(), (literal!("\nelseif ")).clone(), (se.clone()).clone(), (literal!(" then\n  ")).clone(), (seqns.clone()).clone()]);
            ifequationString(elst.clone(), eqnslst.clone(), eqnsfalse.clone(), (s.clone()).clone())?
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

pub fn varString(mut inVar: BackendDAE::Var) -> Result<ArcStr> {
    let mut outStr: ArcStr = arcstr::literal!("");
    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    let mut paths_lst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut unreplaceableStr: ArcStr = arcstr::literal!("");
    let mut dimensions: ArcStr = arcstr::literal!("");
    paths = ElementSource::getElementSourceTypes(inVar.source.clone());
    paths_lst = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut p in (paths.clone()).into_iter().cloned() {
            let __x = AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    unreplaceableStr = (if (inVar.unreplaceable.clone()) {literal!(" unreplaceable")} else {literal!("")}).clone();
    dimensions = (ExpressionBasics::dimensionsString(inVar.arryDim.clone())).clone();
    dimensions = (if (dimensions.clone() != literal!("")) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" [")); __mm_s.push_str(&*dimensions.clone()); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }} else {literal!("")}).clone();
    outStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*DAEDump::dumpDirectionStr(inVar.varDirection.clone())?); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inVar.varName.clone())?); __mm_s.push_str(&*if (isSome(inVar.tplExp.clone())) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(Util::getOption(inVar.tplExp.clone())?)?); ArcStr::from(__mm_s) }} else {literal!("")}); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*kindString(inVar.varKind.clone())?); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*connectorTypeString(inVar.connectorType.clone())); __mm_s.push_str(&*attributesString(inVar.values.clone())?); __mm_s.push_str(&*literal!(") ")); __mm_s.push_str(&*optExpressionString(inVar.bindExp.clone(), (literal!("")).clone())?); __mm_s.push_str(&*DAEDumpTypes::dumpCommentAnnotationStr(inVar.comment.clone())?); __mm_s.push_str(&*stringDelimitList(paths_lst.clone(), (literal!(", ")).clone())); __mm_s.push_str(&*literal!(" type: ")); __mm_s.push_str(&*DAEDump::daeTypeStr(inVar.varType.clone())?); __mm_s.push_str(&*dimensions.clone()); __mm_s.push_str(&*unreplaceableStr.clone()); ArcStr::from(__mm_s) }).clone();
    Ok(outStr)
}

pub fn varStringShort(mut inVar: BackendDAE::Var) -> Result<ArcStr> {
    let mut outStr: ArcStr = arcstr::literal!("");
    outStr = (ComponentReferenceBasics::printComponentRefStr(inVar.varName.clone())?).clone();
    Ok(outStr)
}

pub fn dumpKind(mut inVarKind: BackendDAE::VarKind) -> Result<()> {
    println!("{}", (kindString(inVarKind.clone())?).clone());
    Ok(())
}

pub fn kindString(mut inVarKind: BackendDAE::VarKind) -> Result<ArcStr> {
    let mut kindStr: ArcStr = arcstr::literal!("");
    kindStr = ((::match_deref::match_deref! { match &(inVarKind.clone()) {
        BackendDAE::VarKind::VARIABLE => {
            literal!("VARIABLE")
        },
        BackendDAE::VarKind::STATE { derName: None, index: i, .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("STATE(")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        BackendDAE::VarKind::STATE { derName: Some(dcr), index: i, .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("STATE(")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(dcr.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        BackendDAE::VarKind::STATE_DER => {
            literal!("STATE_DER")
        },
        BackendDAE::VarKind::DUMMY_DER => {
            literal!("DUMMY_DER")
        },
        BackendDAE::VarKind::DUMMY_STATE => {
            literal!("DUMMY_STATE")
        },
        BackendDAE::VarKind::CLOCKED_STATE { .. } => {
            literal!("CLOCKED_STATE")
        },
        BackendDAE::VarKind::DISCRETE => {
            literal!("DISCRETE")
        },
        BackendDAE::VarKind::PARAM => {
            literal!("PARAM")
        },
        BackendDAE::VarKind::CONST => {
            literal!("CONST")
        },
        BackendDAE::VarKind::EXTOBJ { fullClassName: path } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("EXTOBJ: ")); __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }
        },
        BackendDAE::VarKind::JAC_VAR => {
            literal!("JACOBIAN_VAR")
        },
        BackendDAE::VarKind::JAC_TMP_VAR => {
            literal!("JACOBIAN_TMP_VAR")
        },
        BackendDAE::VarKind::OPT_CONSTR => {
            literal!("OPT_CONSTR")
        },
        BackendDAE::VarKind::OPT_FCONSTR => {
            literal!("OPT_FCONSTR")
        },
        BackendDAE::VarKind::OPT_INPUT_WITH_DER => {
            literal!("OPT_INPUT_WITH_DER")
        },
        BackendDAE::VarKind::OPT_INPUT_DER => {
            literal!("OPT_INPUT_DER")
        },
        BackendDAE::VarKind::OPT_TGRID => {
            literal!("OPT_TGRID")
        },
        BackendDAE::VarKind::OPT_LOOP_INPUT { .. } => {
            literal!("OPT_LOOP_INPUT")
        },
        BackendDAE::VarKind::ALG_STATE => {
            literal!("ALG_STATE")
        },
        BackendDAE::VarKind::ALG_STATE_OLD => {
            literal!("ALG_STATE_OLD")
        },
        BackendDAE::VarKind::DAE_RESIDUAL_VAR => {
            literal!("DAE_RESIDUAL_VAR")
        },
        BackendDAE::VarKind::DAE_AUX_VAR => {
            literal!("DAE_AUX_VAR")
        },
        BackendDAE::VarKind::LOOP_ITERATION => {
            literal!("LOOP_ITERATION")
        },
        BackendDAE::VarKind::LOOP_SOLVED => {
            literal!("LOOP_SOLVED")
        },
        _ => {
            literal!("ERROR: BackendDump.kindString varKind not implemented")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(kindStr)
}

pub fn dumpConnectorType(mut inConnectorType: Arc<DAE::ConnectorType>) -> () {
    println!("{}", (connectorTypeString(inConnectorType.clone())).clone());
    ()
}

pub fn connectorTypeString(mut inConnectorType: Arc<DAE::ConnectorType>) -> ArcStr {
    let mut connectorTypeStr: ArcStr = arcstr::literal!("");
    connectorTypeStr = ((::match_deref::match_deref! { match &(inConnectorType.clone()) {
        Deref @ DAE::ConnectorType::FLOW => literal!("flow=true "),
        Deref @ DAE::ConnectorType::POTENTIAL => literal!("flow=false "),
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    connectorTypeStr
}

pub fn dumpAttributes(mut inAttr: Option<Arc<DAE::VariableAttributes>>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inAttr.clone()) {
        None => {
            ()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { distributionOption: None, finalPrefix: None, isProtected: None, stateSelectOption: None, nominal: None, fixed: None, start: None, max: None, min: None, .. }) => {
            ()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { distributionOption: dist, finalPrefix, isProtected, stateSelectOption, nominal, fixed, start, max, min, .. }) => {
            dumpOptExpression(min.clone(), (literal!("min")).clone())?;
            dumpOptExpression(max.clone(), (literal!("max")).clone())?;
            dumpOptExpression(start.clone(), (literal!("start")).clone())?;
            dumpOptExpression(fixed.clone(), (literal!("fixed")).clone())?;
            dumpOptExpression(nominal.clone(), (literal!("nominal")).clone())?;
            dumpOptStateSelection(stateSelectOption.clone());
            dumpOptBoolean(isProtected.clone(), (literal!("protected")).clone());
            dumpOptBoolean(finalPrefix.clone(), (literal!("final")).clone());
            dumpOptDistribution(dist.clone())?;
            ()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { distributionOption: None, finalPrefix: None, isProtected: None, fixed: None, start: None, max: None, min: None, .. }) => {
            ()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { distributionOption: dist, finalPrefix, isProtected, fixed, start, max, min, .. }) => {
            dumpOptExpression(min.clone(), (literal!("min")).clone())?;
            dumpOptExpression(max.clone(), (literal!("max")).clone())?;
            dumpOptExpression(start.clone(), (literal!("start")).clone())?;
            dumpOptExpression(fixed.clone(), (literal!("fixed")).clone())?;
            dumpOptBoolean(isProtected.clone(), (literal!("protected")).clone());
            dumpOptBoolean(finalPrefix.clone(), (literal!("final")).clone());
            dumpOptDistribution(dist.clone())?;
            ()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { finalPrefix: None, isProtected: None, fixed: None, start: None, .. }) => {
            ()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { finalPrefix, isProtected, fixed, start, .. }) => {
            dumpOptExpression(start.clone(), (literal!("start")).clone())?;
            dumpOptExpression(fixed.clone(), (literal!("fixed")).clone())?;
            dumpOptBoolean(isProtected.clone(), (literal!("protected")).clone());
            dumpOptBoolean(finalPrefix.clone(), (literal!("final")).clone());
            ()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { finalPrefix: None, isProtected: None, start: None, .. }) => {
            ()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { finalPrefix, isProtected, start, .. }) => {
            dumpOptExpression(start.clone(), (literal!("start")).clone())?;
            dumpOptBoolean(isProtected.clone(), (literal!("protected")).clone());
            dumpOptBoolean(finalPrefix.clone(), (literal!("final")).clone());
            ()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { finalPrefix: None, isProtected: None, fixed: None, start: None, max: None, min: None, .. }) => {
            ()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { finalPrefix, isProtected, fixed, start, max, min, .. }) => {
            dumpOptExpression(min.clone(), (literal!("min")).clone())?;
            dumpOptExpression(max.clone(), (literal!("max")).clone())?;
            dumpOptExpression(start.clone(), (literal!("start")).clone())?;
            dumpOptExpression(fixed.clone(), (literal!("fixed")).clone())?;
            dumpOptBoolean(isProtected.clone(), (literal!("protected")).clone());
            dumpOptBoolean(finalPrefix.clone(), (literal!("final")).clone());
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpOptDistribution(mut dist: Option<Arc<DAE::Distribution>>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(dist.clone()) {
        None => {
            ()
        },
        Some(Deref @ DAE::Distribution { name: e1, params: e2, paramNames: e3 }) => {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("distribution = Distribution(")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e1.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e2.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e3.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn dumpOptStateSelection(mut ss: Option<DAE::StateSelect>) -> () {
    let _ = (match ss.clone() {
        Some(DAE::StateSelect::NEVER) => {
            println!("{}", (literal!("stateSelect=StateSelect.never ")).clone());
            ()
        },
        Some(DAE::StateSelect::AVOID) => {
            println!("{}", (literal!("stateSelect=StateSelect.avoid ")).clone());
            ()
        },
        Some(DAE::StateSelect::DEFAULT) => (),
        Some(DAE::StateSelect::PREFER) => {
            println!("{}", (literal!("stateSelect=StateSelect.prefer ")).clone());
            ()
        },
        Some(DAE::StateSelect::ALWAYS) => {
            println!("{}", (literal!("stateSelect=StateSelect.alwas ")).clone());
            ()
        },
        _ => (),
    });
    ()
}

fn dumpOptExpression(mut inExp: Option<Arc<DAE::Exp>>, mut inString: ArcStr) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inExp.clone(), inString.clone())) {
        (Some(e), s) => {
            let mut se: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            se = (ExpressionBasics::printExpStr(e.clone())?).clone();
            r#str = stringAppendList(list![(s.clone()).clone(), (literal!(" = ")).clone(), (se.clone()).clone(), (literal!(" ")).clone()]);
            println!("{}", (r#str.clone()).clone());
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpOptBoolean(mut inExp: Option<bool>, mut inString: ArcStr) -> () {
    let _ = (match (inExp.clone(), inString.clone()) {
        (Some(true), mut s) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = stringAppendList(list![(s.clone()).clone(), (literal!(" = true ")).clone()]);
            println!("{}", (r#str.clone()).clone());
            ()
        },
        _ => {
            ()
        },
    });
    ()
}

pub fn attributesString(mut inAttr: Option<Arc<DAE::VariableAttributes>>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inAttr.clone()) {
        None => {
            literal!("")
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { uncertainOption: None, distributionOption: None, finalPrefix: None, isProtected: None, stateSelectOption: None, nominal: None, fixed: None, unit: None, start: None, max: None, min: None, .. }) => {
            literal!("")
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { uncertainOption: uncertainopt, distributionOption: dist, finalPrefix, isProtected, stateSelectOption, nominal, fixed, unit, start, max, min, .. }) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*optExpressionString(min.clone(), (literal!("min")).clone())?); __mm_s.push_str(&*optExpressionString(max.clone(), (literal!("max")).clone())?); __mm_s.push_str(&*optExpressionString(start.clone(), (literal!("start")).clone())?); __mm_s.push_str(&*optExpressionString(unit.clone(), (literal!("unit")).clone())?); __mm_s.push_str(&*optExpressionString(fixed.clone(), (literal!("fixed")).clone())?); __mm_s.push_str(&*optExpressionString(nominal.clone(), (literal!("nominal")).clone())?); __mm_s.push_str(&*optStateSelectionString(stateSelectOption.clone())); __mm_s.push_str(&*optBooleanString(isProtected.clone(), (literal!("protected")).clone())); __mm_s.push_str(&*optBooleanString(finalPrefix.clone(), (literal!("final")).clone())); __mm_s.push_str(&*optDistributionString(dist.clone())?); __mm_s.push_str(&*optUncertainty(uncertainopt.clone())?); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { uncertainOption: None, distributionOption: None, finalPrefix: None, isProtected: None, fixed: None, start: None, max: None, min: None, .. }) => {
            literal!("")
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { uncertainOption: uncertainopt, finalPrefix, isProtected, fixed, start, max, min, .. }) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*optExpressionString(min.clone(), (literal!("min")).clone())?); __mm_s.push_str(&*optExpressionString(max.clone(), (literal!("max")).clone())?); __mm_s.push_str(&*optExpressionString(start.clone(), (literal!("start")).clone())?); __mm_s.push_str(&*optExpressionString(fixed.clone(), (literal!("fixed")).clone())?); __mm_s.push_str(&*optBooleanString(isProtected.clone(), (literal!("protected")).clone())); __mm_s.push_str(&*optBooleanString(finalPrefix.clone(), (literal!("final")).clone())); __mm_s.push_str(&*optUncertainty(uncertainopt.clone())?); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { finalPrefix: None, isProtected: None, fixed: None, start: None, .. }) => {
            literal!("")
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { finalPrefix, isProtected, fixed, start, .. }) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*optExpressionString(start.clone(), (literal!("start")).clone())?); __mm_s.push_str(&*optExpressionString(fixed.clone(), (literal!("fixed")).clone())?); __mm_s.push_str(&*optBooleanString(isProtected.clone(), (literal!("protected")).clone())); __mm_s.push_str(&*optBooleanString(finalPrefix.clone(), (literal!("final")).clone())); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { finalPrefix: None, isProtected: None, start: None, .. }) => {
            literal!("")
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { finalPrefix, isProtected, start, .. }) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*optExpressionString(start.clone(), (literal!("start")).clone())?); __mm_s.push_str(&*optBooleanString(isProtected.clone(), (literal!("protected")).clone())); __mm_s.push_str(&*optBooleanString(finalPrefix.clone(), (literal!("final")).clone())); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { finalPrefix: None, isProtected: None, fixed: None, start: None, max: None, min: None, .. }) => {
            literal!("")
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { finalPrefix, isProtected, fixed, start, max, min, .. }) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*optExpressionString(min.clone(), (literal!("min")).clone())?); __mm_s.push_str(&*optExpressionString(max.clone(), (literal!("max")).clone())?); __mm_s.push_str(&*optExpressionString(start.clone(), (literal!("start")).clone())?); __mm_s.push_str(&*optExpressionString(fixed.clone(), (literal!("fixed")).clone())?); __mm_s.push_str(&*optBooleanString(isProtected.clone(), (literal!("protected")).clone())); __mm_s.push_str(&*optBooleanString(finalPrefix.clone(), (literal!("final")).clone())); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        _ => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

fn optDistributionString(mut dist: Option<Arc<DAE::Distribution>>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(dist.clone()) {
        None => {
            literal!("")
        },
        Some(Deref @ DAE::Distribution { name: e1, params: e2, paramNames: e3 }) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("distribution = Distribution(")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e1.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e2.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e3.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

fn optUncertainty(mut uncertainty: Option<DAE::Uncertainty>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match uncertainty.clone() {
        None => literal!(""),
        Some(DAE::Uncertainty::GIVEN) => literal!("uncertain=Uncertainty.given"),
        Some(DAE::Uncertainty::SOUGHT) => literal!("uncertain=Uncertainty.sought"),
        Some(DAE::Uncertainty::REFINE) => literal!("uncertain=Uncertainty.refine"),
        Some(DAE::Uncertainty::PROPAGATE) => literal!("uncertain=Uncertainty.propagate"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

fn optStateSelectionString(mut ss: Option<DAE::StateSelect>) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match ss.clone() {
        Some(DAE::StateSelect::NEVER) => literal!("stateSelect=StateSelect.never "),
        Some(DAE::StateSelect::AVOID) => literal!("stateSelect=StateSelect.avoid "),
        Some(DAE::StateSelect::DEFAULT) => literal!(""),
        Some(DAE::StateSelect::PREFER) => literal!("stateSelect=StateSelect.prefer "),
        Some(DAE::StateSelect::ALWAYS) => literal!("stateSelect=StateSelect.always "),
        _ => literal!(""),
    })).clone();
    outString
}

pub fn partitionKindString(mut inPartitionKind: BackendDAE::BaseClockPartitionKind) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inPartitionKind.clone() {
        BackendDAE::BaseClockPartitionKind::CLOCKED_PARTITION { subPartIdx: mut idx } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("clocked partition(")); __mm_s.push_str(&*intString(idx.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        BackendDAE::BaseClockPartitionKind::CONTINUOUS_TIME_PARTITION => {
            literal!("continuous time partition")
        },
        BackendDAE::BaseClockPartitionKind::UNSPECIFIED_PARTITION => {
            literal!("unspecified partition")
        },
        BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION => {
            literal!("unknown partition")
        },
        _ => {
            Error::addInternalError((literal!("function partitionKindString failed")).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
    })).clone();
    Ok(outString)
}

fn equationAttrString(mut inEqAttr: BackendDAE::EquationAttributes) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut kind: BackendDAE::EquationKind = BackendDAE::EquationKind::AUX_EQUATION;
    let mut evalStages: BackendDAE::EvaluationStages;
    let BackendDAE::EQUATION_ATTRIBUTES { evalStages: __pa0, kind: __pa1, .. } = (inEqAttr.clone()) else { bail!("pattern mismatch") };
    evalStages = __pa0.clone();
    kind = __pa1.clone();
    outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*equationKindString(kind.clone())?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*equationEvaluationStageString(evalStages.clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
    Ok(outString)
}

fn equationKindString(mut inEqKind: BackendDAE::EquationKind) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inEqKind.clone() {
        BackendDAE::EquationKind::BINDING_EQUATION => {
            literal!("binding")
        },
        BackendDAE::EquationKind::DYNAMIC_EQUATION => {
            literal!("dynamic")
        },
        BackendDAE::EquationKind::INITIAL_EQUATION => {
            literal!("initial")
        },
        BackendDAE::EquationKind::AUX_EQUATION => {
            literal!("auxiliary")
        },
        BackendDAE::EquationKind::DISCRETE_EQUATION => {
            literal!("discrete")
        },
        BackendDAE::EquationKind::UNKNOWN_EQUATION_KIND => {
            literal!("unknown")
        },
        BackendDAE::EquationKind::CLOCKED_EQUATION { clk: mut i } => {
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            cr = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(BackendDAE::WHENCLK_PRREFIX)); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone(), identType: DAE::T_CLOCK_DEFAULT().clone(), subscriptLst: metamodelica::nil() });
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("clocked(")); __mm_s.push_str(&*DAE::ComponentReferenceBasics::printComponentRefStr(cr.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        _ => {
            Error::addInternalError((literal!("function equationKindString failed")).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
    })).clone();
    Ok(outString)
}

fn equationEvaluationStageString(mut inEqEvalStage: BackendDAE::EvaluationStages) -> ArcStr {
    let mut outString: ArcStr = literal!("|");
    outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*outString.clone()); __mm_s.push_str(&*if (inEqEvalStage.dynamicEval.clone()) {literal!("1|")} else {literal!("0|")}); ArcStr::from(__mm_s) }).clone();
    outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*outString.clone()); __mm_s.push_str(&*if (inEqEvalStage.algebraicEval.clone()) {literal!("1|")} else {literal!("0|")}); ArcStr::from(__mm_s) }).clone();
    outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*outString.clone()); __mm_s.push_str(&*if (inEqEvalStage.zerocrossEval.clone()) {literal!("1|")} else {literal!("0|")}); ArcStr::from(__mm_s) }).clone();
    outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*outString.clone()); __mm_s.push_str(&*if (inEqEvalStage.discreteEval.clone()) {literal!("1|")} else {literal!("0|")}); ArcStr::from(__mm_s) }).clone();
    outString
}

fn optExpressionString(mut inExp: Option<Arc<DAE::Exp>>, mut inString: ArcStr) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &((inExp.clone(), inString.clone())) {
        (Some(e), _) => {
            let mut se: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            se = (ExpressionBasics::printExpStr(e.clone())?).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inString.clone()); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*se.clone()); __mm_s.push_str(&*literal!(" ")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        _ => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

fn optBooleanString(mut inExp: Option<bool>, mut inString: ArcStr) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match (inExp.clone(), inString.clone()) {
        (Some(true), _) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inString.clone()); __mm_s.push_str(&*literal!(" = true ")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        _ => {
            literal!("")
        },
    })).clone();
    outString
}

pub fn dumpAdjacencyMatrix(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> () {
    let mut rowIndex: i32 = 0;
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nAdjacency Matrix (row: equation)\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("number of rows: ")); __mm_s.push_str(&*intString((m.clone().borrow().len() as i32))); ArcStr::from(__mm_s) }).clone());
    let __range0 = m.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut row in __range0 {
        rowIndex = rowIndex.clone() + 1;
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*intString(rowIndex.clone())); __mm_s.push_str(&*literal!(":")); ArcStr::from(__mm_s) }).clone());
        for mut i in &*row.clone() {
            let mut i = i.clone();
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone());
        }
    }
    println!("{}", (literal!("\n")).clone());
    ()
}

pub fn dumpAdjacencyMatrixT(mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> () {
    let mut rowIndex: i32 = 0;
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nTransposed Adjacency Matrix (row: variable)\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("number of rows: ")); __mm_s.push_str(&*intString((mT.clone().borrow().len() as i32))); ArcStr::from(__mm_s) }).clone());
    let __range0 = mT.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut row in __range0 {
        rowIndex = rowIndex.clone() + 1;
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*intString(rowIndex.clone())); __mm_s.push_str(&*literal!(":")); ArcStr::from(__mm_s) }).clone());
        for mut i in &*row.clone() {
            let mut i = i.clone();
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone());
        }
    }
    println!("{}", (literal!("\n")).clone());
    ()
}

pub fn dumpAdjacencyRow(mut inIntegerLst: Arc<metamodelica::List<i32>>) -> () {
    let _ = (::match_deref::match_deref! { match &(inIntegerLst.clone()) {
        Deref @ metamodelica::List::Nil => {
            println!("{}", (literal!("\n")).clone());
            ()
        },
        Deref @ metamodelica::List::Cons { head: x, tail: xs } => {
            let mut s: ArcStr = arcstr::literal!("");
            s = (intString(x.clone())).clone();
            println!("{}", (s.clone()).clone());
            println!("{}", (literal!(" ")).clone());
            dumpAdjacencyRow(xs.clone());
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ()
}

pub fn dumpAdjacencyMatrixEnhanced(mut m: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>) -> Result<()> {
    let mut mlen: i32 = 0;
    let mut mlen_str: ArcStr = arcstr::literal!("");
    let mut m_1: Arc<metamodelica::List<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>> = metamodelica::nil();
    println!("{}", (literal!("Adjacency Matrix Enhanced (row == equation)\n")).clone());
    println!("{}", (literal!("====================================\n")).clone());
    mlen = (m.clone().borrow().len() as i32);
    mlen_str = (intString(mlen.clone())).clone();
    println!("{}", (literal!("number of rows: ")).clone());
    println!("{}", (mlen_str.clone()).clone());
    println!("{}", (literal!("\n")).clone());
    m_1 = Arc::new(m.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
    dumpAdjacencyMatrixEnhanced2(m_1.clone(), 1)?;
    Ok(())
}

pub fn dumpAdjacencyMatrixTEnhanced(mut m: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>) -> Result<()> {
    let mut mlen: i32 = 0;
    let mut mlen_str: ArcStr = arcstr::literal!("");
    let mut m_1: Arc<metamodelica::List<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>> = metamodelica::nil();
    println!("{}", (literal!("Transpose Adjacency Matrix Enhanced (row == var)\n")).clone());
    println!("{}", (literal!("=====================================\n")).clone());
    mlen = (m.clone().borrow().len() as i32);
    mlen_str = (intString(mlen.clone())).clone();
    println!("{}", (literal!("number of rows: ")).clone());
    println!("{}", (mlen_str.clone()).clone());
    println!("{}", (literal!("\n")).clone());
    m_1 = Arc::new(m.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
    dumpAdjacencyMatrixEnhanced2(m_1.clone(), 1)?;
    Ok(())
}

fn dumpAdjacencyMatrixEnhanced2(mut inRows: Arc<metamodelica::List<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>>, mut rowIndex: i32) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inRows.clone(), rowIndex.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: row, tail: rows }, _) => {
            println!("{}", (intString(rowIndex.clone())).clone());
            println!("{}", (literal!(":")).clone());
            dumpAdjacencyRowEnhanced(row.clone())?;
            dumpAdjacencyMatrixEnhanced2(rows.clone(), rowIndex.clone() + 1)?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn dumpAdjacencyRowEnhanced(mut inRow: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inRow.clone()) {
        Deref @ metamodelica::List::Nil => {
            println!("{}", (literal!("\n")).clone());
            ()
        },
        Deref @ metamodelica::List::Cons { head: (x, solva, Deref @ metamodelica::List::Nil), tail: xs } => {
            let mut s: ArcStr = arcstr::literal!("");
            let mut s1: ArcStr = arcstr::literal!("");
            s = (intString(x.clone())).clone();
            s1 = (dumpSolvability(solva.clone())?).clone();
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone());
            println!("{}", (literal!(" ")).clone());
            dumpAdjacencyRowEnhanced(xs.clone())?;
            ()
        },
        Deref @ metamodelica::List::Cons { head: (x, solva, cons), tail: xs } => {
            let mut s: ArcStr = arcstr::literal!("");
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            s = (intString(x.clone())).clone();
            s1 = (dumpSolvability(solva.clone())?).clone();
            s2 = (ExpressionDump::constraintDTlistToString(cons.clone(), (literal!(",")).clone())?).clone();
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone());
            println!("{}", (literal!(" ")).clone());
            dumpAdjacencyRowEnhanced(xs.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn dumpSolvability(mut solva: BackendDAE::Solvability) -> Result<ArcStr> {
    let mut s: ArcStr = arcstr::literal!("");
    s = ((match solva.clone() {
        BackendDAE::Solvability::SOLVABILITY_SOLVED => {
            literal!("solved")
        },
        BackendDAE::Solvability::SOLVABILITY_CONSTONE => {
            literal!("constone")
        },
        BackendDAE::Solvability::SOLVABILITY_CONST { b: mut b } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("const(")); __mm_s.push_str(&*boolString(b.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        BackendDAE::Solvability::SOLVABILITY_PARAMETER { b: mut b } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("param(")); __mm_s.push_str(&*boolString(b.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        BackendDAE::Solvability::SOLVABILITY_LINEAR { b: mut b } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("variable(")); __mm_s.push_str(&*boolString(b.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        BackendDAE::Solvability::SOLVABILITY_NONLINEAR => {
            literal!("nonlinear")
        },
        BackendDAE::Solvability::SOLVABILITY_UNSOLVABLE => {
            literal!("unsolvable")
        },
        BackendDAE::Solvability::SOLVABILITY_SOLVABLE => {
            literal!("solvable")
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(s)
}

pub fn dumpFullMatching(mut inMatch: Arc<BackendDAE::Matching>, mut inSyst: Option<Arc<BackendDAE::EqSystem>>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inMatch.clone()) {
        Deref @ BackendDAE::Matching::NO_MATCHING => {
            println!("{}", (literal!("no matching\n")).clone());
            ()
        },
        Deref @ BackendDAE::Matching::MATCHING { ass1, ass2: _, comps } => {
            dumpMatching(ass1.clone())?;
            println!("{}", (literal!("\n\n")).clone());
            dumpComponents(comps.clone(), inSyst.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn dumpMatching(mut v: metamodelica::Array<i32>) -> Result<()> {
    let mut len: i32 = 0;
    let mut len_str: ArcStr = arcstr::literal!("");
    println!("{}", (literal!("Matching\n")).clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    len = (v.clone().borrow().len() as i32);
    len_str = (intString(len.clone())).clone();
    println!("{}", (len_str.clone()).clone());
    println!("{}", (literal!(" variables and equations\n")).clone());
    dumpMatching2(v.clone(), 1, len.clone())?;
    Ok(())
}

fn dumpMatching2(mut v: metamodelica::Array<i32>, mut i: i32, mut len: i32) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (v.clone(), i.clone(), len.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut eqn: i32 = 0;
            let mut s: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let true = (intLe(i.clone(), len.clone())) else { bail!("pattern mismatch") };
            s = (intString(i.clone())).clone();
            eqn = v.borrow()[(i.clone()-1) as usize].clone();
            s2 = (intString(eqn.clone())).clone();
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("var ")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!(" is solved in eqn ")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            dumpMatching2(v.clone(), i.clone() + 1, len.clone())?;
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn dumpMatchingVars(mut ass1: metamodelica::Array<i32>) -> () {
    let mut varIndex: i32 = 0;
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nMatching\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString((ass1.clone().borrow().len() as i32))); __mm_s.push_str(&*literal!(" variables\n")); ArcStr::from(__mm_s) }).clone());
    let __range0 = ass1.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut i in __range0 {
        varIndex = varIndex.clone() + 1;
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("var ")); __mm_s.push_str(&*intString(varIndex.clone())); __mm_s.push_str(&*literal!(" is solved in eqn ")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    ()
}

pub fn dumpMatchingEqns(mut ass2: metamodelica::Array<i32>) -> () {
    let mut eqnIndex: i32 = 0;
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nMatching\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString((ass2.clone().borrow().len() as i32))); __mm_s.push_str(&*literal!(" equations\n")); ArcStr::from(__mm_s) }).clone());
    let __range0 = ass2.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut i in __range0 {
        eqnIndex = eqnIndex.clone() + 1;
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("eqn ")); __mm_s.push_str(&*intString(eqnIndex.clone())); __mm_s.push_str(&*literal!(" is solved for var ")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    ()
}

pub fn dumpMarkedEqns(mut syst: Arc<BackendDAE::EqSystem>, mut inIntegerLst: Arc<metamodelica::List<i32>>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut slst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(syst.clone()) {
        Deref @ BackendDAE::EqSystem { orderedEqs: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    eqns = __pa0.clone();
    slst = List::map1(inIntegerLst.clone(), (std::sync::Arc::new(dumpMarkedEqns1) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<ArcStr> + 'static>), eqns.clone());
    outString = stringDelimitList(slst.clone(), (literal!("\n")).clone());
    Ok(outString)
}

fn dumpMarkedEqns1(mut index: i32, mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<ArcStr> {
    let mut outS: ArcStr = arcstr::literal!("");
    let mut s1: ArcStr = arcstr::literal!("");
    let mut s2: ArcStr = arcstr::literal!("");
    let mut s3: ArcStr = arcstr::literal!("");
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    eqn = BackendEquation::get(eqns.clone(), index.clone());
    outS = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  ")); __mm_s.push_str(&*intString(index.clone())); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*equationString(eqn.clone())?); ArcStr::from(__mm_s) }).clone();
    Ok(outS)
}

pub fn dumpMarkedVarsLsts(mut syst: Arc<BackendDAE::EqSystem>, mut inIntegerLstLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<ArcStr> {
    let mut outString: ArcStr = literal!("");
    for mut inIntegerLst in &*inIntegerLstLst.clone() {
        let mut inIntegerLst = inIntegerLst.clone();
        outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*outString.clone()); __mm_s.push_str(&*dumpMarkedVars(syst.clone(), inIntegerLst.clone())?); __mm_s.push_str(&*literal!(",")); ArcStr::from(__mm_s) }).clone();
    }
    Ok(outString)
}

pub fn dumpMarkedVars(mut syst: Arc<BackendDAE::EqSystem>, mut inIntegerLst: Arc<metamodelica::List<i32>>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut vars: BackendDAE::Variables;
    let mut slst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(syst.clone()) {
        Deref @ BackendDAE::EqSystem { orderedVars: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    vars = __pa0.clone();
    slst = List::map1(inIntegerLst.clone(), (std::sync::Arc::new(dumpMarkedVars1) as std::sync::Arc<dyn ::std::ops::Fn(i32, BackendDAE::Variables) -> Result<ArcStr> + 'static>), vars.clone());
    outString = stringDelimitList(slst.clone(), (literal!("\n")).clone());
    Ok(outString)
}

fn dumpMarkedVars1(mut index: i32, mut vars: BackendDAE::Variables) -> Result<ArcStr> {
    let mut outS: ArcStr = arcstr::literal!("");
    let mut var: BackendDAE::Var;
    var = BackendVariable::getVarAt(vars.clone(), index.clone())?;
    outS = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  ")); __mm_s.push_str(&*intString(index.clone())); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*varString(var.clone())?); ArcStr::from(__mm_s) }).clone();
    Ok(outS)
}

pub fn dumpMarkedVarList(mut varList: Arc<metamodelica::List<BackendDAE::Var>>, mut selList: Arc<metamodelica::List<i32>>) -> Result<ArcStr> {
    let mut outString: ArcStr = literal!("");
    let mut var: BackendDAE::Var;
    for mut sel in &*selList.clone() {
        let mut sel = sel.clone();
        if let Ok(__iflet0) = (varList.clone()).get(sel.clone()) {
            var = __iflet0;
        } else {
            Error::addInternalError((literal!("function dumpMarkedVarList failed")).clone(), metamodelica::sourceInfo!())?;
            Error::addCompilerNotification(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Could not get variable ")); __mm_s.push_str(&*intString(sel.clone())); __mm_s.push_str(&*literal!(" from varList \n")); __mm_s.push_str(&*varListString(varList.clone(), (literal!("")).clone())); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail");
        }
        outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*outString.clone()); __mm_s.push_str(&*literal!("  ")); __mm_s.push_str(&*varString(var.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    }
    Ok(outString)
}

pub fn dumpComponentsGraphStr(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    let mut n: i32 = 0;
    let mut lst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut s: ArcStr = arcstr::literal!("");
    let mut syst: Arc<BackendDAE::EqSystem>;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let (__pa4, __pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ DAE { eqs: metamodelica::List::Cons { head: __pa4 @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { ass2: __pa0, ass1: __pa1, .. }, mT: Some(__pa2), m: Some(__pa3), .. }, tail: Deref @ metamodelica::List::Nil }, .. } => (__pa4.clone(), __pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ass2 = __pa0.clone();
    ass1 = __pa1.clone();
    mT = __pa2.clone();
    m = __pa3.clone();
    syst = __pa4.clone();
    n = BackendDAEUtil::systemSize(syst.clone());
    lst = dumpComponentsGraphStr2(1, n.clone(), m.clone(), mT.clone(), ass1.clone(), ass2.clone());
    s = stringDelimitList(lst.clone(), (literal!(",")).clone());
    s = stringAppendList(list![(literal!("{")).clone(), (s.clone()).clone(), (literal!("}")).clone()]);
    println!("{}", (s.clone()).clone());
    outDAE = inDAE.clone();
    Ok(outDAE)
}

fn dumpComponentsGraphStr2(mut i: i32, mut n: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Arc<metamodelica::List<ArcStr>> {
    let mut lst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut llst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut eqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut strLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut slst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut r#str: ArcStr = arcstr::literal!("");
    if i.clone() <= n.clone() {
        eqns = Matching::reachableEquations(i.clone(), mT.clone(), ass2.clone());
        llst = List::map(eqns.clone(), std::sync::Arc::new(fnptr!(List::create, _)));
        llst = List::map1(llst.clone(), std::sync::Arc::new(fnptr!(List::consr, _, _)), i.clone());
        slst = List::map(llst.clone(), (std::sync::Arc::new(fnptr!(intListStr, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>));
        r#str = stringDelimitList(slst.clone(), (literal!(",")).clone());
        r#str = stringAppendList(list![(literal!("{")).clone(), (r#str.clone()).clone(), (literal!("}")).clone()]);
        strLst = dumpComponentsGraphStr2(i.clone() + 1, n.clone(), m.clone(), mT.clone(), ass1.clone(), ass2.clone());
        lst = cons((r#str.clone()).clone(), strLst.clone());
    }
    lst
}

pub fn dumpList(mut l: Arc<metamodelica::List<i32>>, mut r#str: ArcStr) -> () {
    let mut s: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut sl: ArcStr = arcstr::literal!("");
    s = List::map(l.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
    sl = stringDelimitList(s.clone(), (literal!(", ")).clone());
    println!("{}", (r#str.clone()).clone());
    println!("{}", (sl.clone()).clone());
    println!("{}", (literal!("\n")).clone());
    ()
}

pub fn dumpComponentsOLD(mut l: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<()> {
    println!("{}", (literal!("Blocks\n")).clone());
    println!("{}", (literal!("=======\n")).clone());
    dumpComponents2(l.clone(), 1)?;
    Ok(())
}

fn dumpComponents2(mut inIntegerLstLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut inInteger: i32) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inIntegerLstLst.clone(), inInteger.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: l, tail: lst }, i) => {
            let mut i_1: i32 = 0;
            let mut ls: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut s: ArcStr = arcstr::literal!("");
            println!("{}", (literal!("{")).clone());
            ls = List::map(List::sort(l.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?, (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>));
            s = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            println!("{}", (s.clone()).clone());
            println!("{}", (literal!("}\n")).clone());
            i_1 = i.clone() + 1;
            dumpComponents2(lst.clone(), i_1.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn intListStr(mut lst: Arc<metamodelica::List<i32>>) -> ArcStr {
    let mut res: ArcStr = arcstr::literal!("");
    res = stringDelimitList(List::map(lst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone());
    res = stringAppendList(list![(literal!("{")).clone(), (res.clone()).clone(), (literal!("}")).clone()]);
    res
}

// protected function dumpAliasVariable
// "author: Frenkel TUD 2010-11"
//  input tuple<BackendDAE.Var,list<Integer>> inTpl;
//  output tuple<BackendDAE.Var,list<Integer>> outTpl;
// algorithm
//   outTpl:=
//   matchcontinue (inTpl)
//     local
//       BackendDAE.Var v;
//       DAE.ComponentRef cr;
//       DAE.Exp e;
//       String s,scr,se;
//     case ((v,_))
//       equation
//         cr = BackendVariable.varCref(v);
//         e = BackendVariable.varBindExp(v);
//         //print("### dump var : " +  ComponentReferenceBasics.printComponentRefStr(cr) + "\n");
//         scr = ComponentReferenceBasics.printComponentRefStr(cr);
//         se = ExpressionBasics.printExpStr(e);
//         s = stringAppendList({scr," = ",se,"\n"});
//         print(s);
//       then ((v,{}));
//     else inTpl;
//   end matchcontinue;
// end dumpAliasVariable;
pub fn dumpStateVariables(mut inVars: BackendDAE::Variables) -> Result<()> {
    println!("{}", (literal!("States Variables\n")).clone());
    println!("{}", (literal!("=================\n")).clone());
    let _ = BackendVariable::traverseBackendDAEVars(inVars.clone(), (std::sync::Arc::new(dumpStateVariable) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, i32) -> Result<(BackendDAE::Var, i32)> + 'static>), 1)?;
    println!("{}", (literal!("\n")).clone());
    Ok(())
}

fn dumpStateVariable(mut inVar: BackendDAE::Var, mut inPos: i32) -> Result<(BackendDAE::Var, i32)> {
    let mut v: BackendDAE::Var;
    let mut pos: i32 = 0;
    (v, pos) = 'mc: {
        let __mc_input = (inVar.clone(), inPos.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut v, mut pos) = __mc_input.clone() else { bail!("nomatch") };
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut scr: ArcStr = arcstr::literal!("");
            let true = (BackendVariable::isStateVar(v.clone())) else { bail!("pattern mismatch") };
            cr = BackendVariable::varCref(v.clone())?;
            scr = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
            println!("{}", (intString(pos.clone())).clone());
            println!("{}", (literal!(": ")).clone());
            println!("{}", (scr.clone()).clone());
            println!("{}", (literal!("\n")).clone());
            Ok((v.clone(), pos.clone() + 1))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok((inVar.clone(), inPos.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((v, pos))
}

pub fn bltdump(mut headerline: ArcStr, mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = inDAE.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut strlow: ArcStr = arcstr::literal!("");
                    let Flags::STRING_FLAG { data: __pa0 } = (Flags::getConfigValue(Flags::DUMP_TARGET.clone())?) else { bail!("pattern mismatch") };
                    r#str = __pa0.clone();
                    strlow = (System::tolower((r#str.clone()).clone())).clone();
                    let true = (intGt(System::stringFind((strlow.clone()).clone(), (literal!(".html")).clone())?, 0)) else { bail!("pattern mismatch") };
                    DumpHTML::dumpDAE(inDAE.clone(), (headerline.clone()).clone(), (r#str.clone()).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE { UNIQUEIO: eqs, derivativeNamePrefix: shared, .. } => {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*headerline.clone()); __mm_s.push_str(&*literal!(":\n")); ArcStr::from(__mm_s) }).clone());
                    List::map_0(eqs.clone(), (std::sync::Arc::new(printEqSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>) -> Result<()> + 'static>));
                    println!("{}", (literal!("\n")).clone());
                    printShared(shared.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn innerEquationString(mut innerEquation: BackendDAE::InnerEquation) -> Result<ArcStr> {
    let mut s: ArcStr = arcstr::literal!("");
    let mut e: i32 = 0;
    let mut v: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (e, v, _) = BackendDAEUtil::getEqnAndVarsFromInnerEquation(innerEquation.clone())?;
    s = stringDelimitList(List::map(v.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone());
    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*intString(e.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
    Ok(s)
}

pub type DumpCompShortSystemsTpl = (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);

pub type DumpCompShortMixedTpl = (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>);

pub type DumpCompShortTornTpl = (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>);

pub fn dumpCompShort(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<()> {
    let mut sys: i32 = 0;
    let mut inp: i32 = 0;
    let mut st: i32 = 0;
    let mut dvar: i32 = 0;
    let mut dst: i32 = 0;
    let mut seq: i32 = 0;
    let mut salg: i32 = 0;
    let mut sarr: i32 = 0;
    let mut sce: i32 = 0;
    let mut swe: i32 = 0;
    let mut sie: i32 = 0;
    let mut eqsys: i32 = 0;
    let mut meqsys: i32 = 0;
    let mut teqsys: i32 = 0;
    let mut teqsys2: i32 = 0;
    let mut strcomps: i32 = 0;
    let mut e_jc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut e_jn: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut e_nj: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut te_l: Arc<metamodelica::List<(i32, i32, i32)>> = metamodelica::nil();
    let mut te_l2: Arc<metamodelica::List<(i32, i32, i32)>> = metamodelica::nil();
    let mut te_nl: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut te_nl2: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut m_se: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut m_salg: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut m_sarr: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut m_sec: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut me_jc: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut e_jt: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut me_jt: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut me_jn: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut me_nj: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut me_lt: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut me_nt: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut states: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut discvars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut discstates: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut clockedstates: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut HS: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut removedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut sysStr: ArcStr = arcstr::literal!("");
    let mut stStr: ArcStr = arcstr::literal!("");
    let mut dvarStr: ArcStr = arcstr::literal!("");
    let mut dstStr: ArcStr = arcstr::literal!("");
    let mut clckStr: ArcStr = arcstr::literal!("");
    let mut statesStr: ArcStr = arcstr::literal!("");
    let mut discvarsStr: ArcStr = arcstr::literal!("");
    let mut discstatesStr: ArcStr = arcstr::literal!("");
    let mut clockedstatesStr: ArcStr = arcstr::literal!("");
    let mut inpStr: ArcStr = arcstr::literal!("");
    let mut strcompsStr: ArcStr = arcstr::literal!("");
    let mut seqStr: ArcStr = arcstr::literal!("");
    let mut sarrStr: ArcStr = arcstr::literal!("");
    let mut salgStr: ArcStr = arcstr::literal!("");
    let mut sceStr: ArcStr = arcstr::literal!("");
    let mut sweStr: ArcStr = arcstr::literal!("");
    let mut sieStr: ArcStr = arcstr::literal!("");
    let mut eqsysStr: ArcStr = arcstr::literal!("");
    let mut teqsysStr: ArcStr = arcstr::literal!("");
    let mut meqsysStr: ArcStr = arcstr::literal!("");
    let mut daeType: ArcStr = arcstr::literal!("");
    let mut msgs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut systemsTpl: DumpCompShortSystemsTpl;
    let mut mixedTpl: DumpCompShortMixedTpl;
    let mut tornTpl: DumpCompShortTornTpl;
    let mut tornTpl2: DumpCompShortTornTpl;
    let mut backendDAEType: BackendDAE::BackendDAEType = BackendDAE::BackendDAEType::ALGEQSYSTEM;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ DAE { UNIQUEIO: __pa0, derivativeNamePrefix: BackendDAE::Shared { backendDAEType: __pa1, .. }, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    systs = __pa0.clone();
    backendDAEType = __pa1.clone();
    removedEqs = BackendDAEUtil::collapseRemovedEqs(inDAE.clone())?;
    daeType = (printBackendDAEType2String(backendDAEType.clone())?).clone();
    HS = HashSet::emptyHashSet();
    HS = List::fold(systs.clone(), (std::sync::Arc::new(Initialization::collectPreVariablesEqSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))> + 'static>), HS.clone());
    (_, HS) = BackendDAEUtil::traverseBackendDAEExpsEqns(removedEqs.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(Initialization::collectPreVariablesTraverseExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)))> + 'static>), HS.clone()))?;
    discstates = BaseHashSet::hashSetList(HS.clone())?;
    dst = (discstates.clone().len() as i32);
    for mut syst in &*systs.clone() {
        let mut syst = syst.clone();
        clockedstates = BackendVariable::filterCrefs(syst.orderedVars.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::isVarClockedState, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>), clockedstates.clone())?;
    }
    (sys, inp, st, states, dvar, discvars, seq, salg, sarr, sce, swe, sie, systemsTpl, mixedTpl, tornTpl, tornTpl2) = BackendDAEUtil::foldEqSystem(inDAE.clone(), (std::sync::Arc::new(dumpCompShort1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (i32, i32, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, i32, i32, i32, i32, i32, i32, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>))) -> Result<(i32, i32, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, i32, i32, i32, i32, i32, i32, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>))> + 'static>), (0, 0, 0, metamodelica::nil(), 0, metamodelica::nil(), 0, 0, 0, 0, 0, 0, (metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil()), (metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil()), (metamodelica::nil(), metamodelica::nil()), (metamodelica::nil(), metamodelica::nil())))?;
    (e_jc, e_jt, e_jn, e_nj) = systemsTpl.clone();
    (m_se, m_salg, m_sarr, m_sec, me_jc, me_jt, me_jn, me_nj, me_lt, me_nt) = mixedTpl.clone();
    (te_l, te_nl) = tornTpl.clone();
    (te_l2, te_nl2) = tornTpl2.clone();
    eqsys = (e_jc.clone().len() as i32) + (e_jt.clone().len() as i32) + (e_jn.clone().len() as i32) + (e_nj.clone().len() as i32);
    meqsys = (m_se.clone().len() as i32) + (m_sarr.clone().len() as i32) + (m_salg.clone().len() as i32) + (m_sec.clone().len() as i32) + (me_jc.clone().len() as i32) + (me_jt.clone().len() as i32) + (me_jn.clone().len() as i32) + (me_nj.clone().len() as i32) + (me_lt.clone().len() as i32) + (me_nt.clone().len() as i32);
    teqsys = (te_l.clone().len() as i32) + (te_nl.clone().len() as i32);
    teqsys2 = (te_l2.clone().len() as i32) + (te_nl2.clone().len() as i32);
    strcomps = seq.clone() + eqsys.clone() + meqsys.clone() + sarr.clone() + salg.clone() + sce.clone() + swe.clone() + sie.clone() + teqsys.clone();
    sysStr = (intString(sys.clone())).clone();
    stStr = (intString(st.clone())).clone();
    dvarStr = (intString(dvar.clone())).clone();
    dstStr = (intString(dst.clone())).clone();
    clckStr = (intString((clockedstates.clone().len() as i32))).clone();
    statesStr = (if (Flags::isSet(Flags::DUMP_STATESELECTION_INFO.clone())?) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*stringDelimitList(List::map(states.clone(), (std::sync::Arc::new(ComponentReferenceBasics::printComponentRefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }} else {literal!(" ('-d=stateselection' for list of states)")}).clone();
    discvarsStr = (if (Flags::isSet(Flags::DUMP_DISCRETEVARS_INFO.clone())?) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*stringDelimitList(List::map(discvars.clone(), (std::sync::Arc::new(ComponentReferenceBasics::printComponentRefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }} else {literal!(" ('-d=discreteinfo' for list of discrete vars)")}).clone();
    discstatesStr = (if (Flags::isSet(Flags::DUMP_DISCRETEVARS_INFO.clone())?) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*stringDelimitList(List::map(discstates.clone(), (std::sync::Arc::new(ComponentReferenceBasics::printComponentRefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }} else {literal!(" ('-d=discreteinfo' for list of discrete states)")}).clone();
    clockedstatesStr = (if (Flags::isSet(Flags::DUMP_DISCRETEVARS_INFO.clone())?) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*stringDelimitList(List::map(clockedstates.clone(), (std::sync::Arc::new(ComponentReferenceBasics::printComponentRefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }} else {literal!(" ('-d=discreteinfo' for list of clocked states)")}).clone();
    stStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stStr.clone()); __mm_s.push_str(&*statesStr.clone()); ArcStr::from(__mm_s) }).clone();
    dvarStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*dvarStr.clone()); __mm_s.push_str(&*discvarsStr.clone()); ArcStr::from(__mm_s) }).clone();
    dstStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*dstStr.clone()); __mm_s.push_str(&*discstatesStr.clone()); ArcStr::from(__mm_s) }).clone();
    clckStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*clckStr.clone()); __mm_s.push_str(&*clockedstatesStr.clone()); ArcStr::from(__mm_s) }).clone();
    inpStr = (intString(inp.clone())).clone();
    msgs = list![(daeType.clone()).clone(), (sysStr.clone()).clone(), (stStr.clone()).clone(), (dvarStr.clone()).clone(), (dstStr.clone()).clone(), (clckStr.clone()).clone(), (inpStr.clone()).clone()];
    Error::addMessage(Error::BACKENDDAEINFO_STATISTICS.clone(), msgs.clone())?;
    strcompsStr = (intString(strcomps.clone())).clone();
    seqStr = (intString(seq.clone())).clone();
    sarrStr = (intString(sarr.clone())).clone();
    salgStr = (intString(salg.clone())).clone();
    sceStr = (intString(sce.clone())).clone();
    sweStr = (intString(swe.clone())).clone();
    sieStr = (intString(sie.clone())).clone();
    eqsysStr = (intString(eqsys.clone())).clone();
    teqsysStr = (intString(teqsys.clone())).clone();
    meqsysStr = (intString(meqsys.clone())).clone();
    msgs = list![(daeType.clone()).clone(), (strcompsStr.clone()).clone(), (seqStr.clone()).clone(), (sarrStr.clone()).clone(), (salgStr.clone()).clone(), (sceStr.clone()).clone(), (sweStr.clone()).clone(), (sieStr.clone()).clone(), (eqsysStr.clone()).clone(), (teqsysStr.clone()).clone(), (meqsysStr.clone()).clone()];
    Error::addMessage(Error::BACKENDDAEINFO_STRONGCOMPONENT_STATISTICS.clone(), msgs.clone())?;
    if intGt(eqsys.clone(), 0) {
        dumpCompSystems(systemsTpl.clone())?;
    }
    if intGt(meqsys.clone(), 0) {
        dumpCompMixed(mixedTpl.clone())?;
    }
    if intGt(teqsys.clone(), 0) {
        dumpCompTorn(tornTpl.clone(), (literal!("strict")).clone())?;
    }
    if intGt(teqsys2.clone(), 0) && !(stringEqual((Config::dynamicTearing()?).clone(), (literal!("false")).clone())) {
        dumpCompTorn(tornTpl2.clone(), (literal!("casual")).clone())?;
    }
    Ok(())
}

fn dumpCompSystems(mut systemsTpl: DumpCompShortSystemsTpl) -> Result<()> {
    let mut e_jc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut e_jn: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut e_nj: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut e_jt: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut s_jc: ArcStr = arcstr::literal!("");
    let mut s_jn: ArcStr = arcstr::literal!("");
    let mut s_nj: ArcStr = arcstr::literal!("");
    let mut s_jt: ArcStr = arcstr::literal!("");
    (e_jc, e_jt, e_jn, e_nj) = systemsTpl.clone();
    s_jc = (equationSizesStr(e_jc.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))).clone();
    s_jt = (equationSizesStr(e_jt.clone(), (std::sync::Arc::new(sizeNumNonZeroTplString) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32)) -> Result<ArcStr> + 'static>))).clone();
    s_jn = (equationSizesStr(e_jn.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))).clone();
    s_nj = (equationSizesStr(e_nj.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))).clone();
    Error::addMessage(Error::BACKENDDAEINFO_SYSTEMS.clone(), list![(s_jc.clone()).clone(), (s_jt.clone()).clone(), (s_jn.clone()).clone(), (s_nj.clone()).clone()])?;
    Ok(())
}

fn dumpCompTorn(mut systemsTpl: DumpCompShortTornTpl, mut whichset: ArcStr) -> Result<()> {
    let mut te_l: Arc<metamodelica::List<(i32, i32, i32)>> = metamodelica::nil();
    let mut te_nl: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut s_l: ArcStr = arcstr::literal!("");
    let mut s_nl: ArcStr = arcstr::literal!("");
    (te_l, te_nl) = systemsTpl.clone();
    s_l = (equationSizesStr(te_l.clone(), (std::sync::Arc::new(sizeNumNonZeroTornTplString) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32, i32)) -> Result<ArcStr> + 'static>))).clone();
    s_nl = (equationSizesStr(te_nl.clone(), (std::sync::Arc::new(fnptr!(intTplString, (i32, i32))) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32)) -> Result<ArcStr> + 'static>))).clone();
    Error::addMessage(Error::BACKENDDAEINFO_TORN.clone(), list![(whichset.clone()).clone(), (s_l.clone()).clone(), (s_nl.clone()).clone()])?;
    Ok(())
}

fn dumpCompMixed(mut mixedTpl: DumpCompShortMixedTpl) -> Result<()> {
    let mut m_se: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut m_salg: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut m_sarr: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut m_sec: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut me_jc: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut e_jt: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut me_jt: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut me_jn: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut me_nj: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut me_lt: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut me_nt: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut s_se: ArcStr = arcstr::literal!("");
    let mut s_salg: ArcStr = arcstr::literal!("");
    let mut s_sarr: ArcStr = arcstr::literal!("");
    let mut s_sec: ArcStr = arcstr::literal!("");
    let mut s_jc: ArcStr = arcstr::literal!("");
    let mut s_jt: ArcStr = arcstr::literal!("");
    let mut s_jn: ArcStr = arcstr::literal!("");
    let mut s_nj: ArcStr = arcstr::literal!("");
    let mut s_lt: ArcStr = arcstr::literal!("");
    let mut s_nt: ArcStr = arcstr::literal!("");
    (m_se, m_salg, m_sarr, m_sec, me_jc, me_jt, me_jn, me_nj, me_lt, me_nt) = mixedTpl.clone();
    s_se = (equationSizesStr(m_se.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))).clone();
    s_salg = (equationSizesStr(m_salg.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))).clone();
    s_sarr = (equationSizesStr(m_sarr.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))).clone();
    s_sec = (equationSizesStr(m_sec.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))).clone();
    s_jc = (equationSizesStr(me_jc.clone(), (std::sync::Arc::new(fnptr!(intTplString, (i32, i32))) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32)) -> Result<ArcStr> + 'static>))).clone();
    s_jt = (equationSizesStr(me_jt.clone(), (std::sync::Arc::new(fnptr!(intTplString, (i32, i32))) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32)) -> Result<ArcStr> + 'static>))).clone();
    s_jn = (equationSizesStr(me_jn.clone(), (std::sync::Arc::new(fnptr!(intTplString, (i32, i32))) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32)) -> Result<ArcStr> + 'static>))).clone();
    s_nj = (equationSizesStr(me_nj.clone(), (std::sync::Arc::new(fnptr!(intTplString, (i32, i32))) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32)) -> Result<ArcStr> + 'static>))).clone();
    s_lt = (equationSizesStr(me_lt.clone(), (std::sync::Arc::new(fnptr!(intTplString, (i32, i32))) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32)) -> Result<ArcStr> + 'static>))).clone();
    s_nt = (equationSizesStr(me_nt.clone(), (std::sync::Arc::new(fnptr!(intTplString, (i32, i32))) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32)) -> Result<ArcStr> + 'static>))).clone();
    Error::addMessage(Error::BACKENDDAEINFO_MIXED.clone(), list![(s_se.clone()).clone(), (s_salg.clone()).clone(), (s_sarr.clone()).clone(), (s_sec.clone()).clone(), (s_jc.clone()).clone(), (s_jt.clone()).clone(), (s_jn.clone()).clone(), (s_nj.clone()).clone(), (s_lt.clone()).clone(), (s_nt.clone()).clone()])?;
    Ok(())
}

fn equationSizesStr<A: Clone + 'static>(mut eqs: Arc<metamodelica::List<A>>, mut r#fn: Arc<dyn ::std::ops::Fn(A) -> Result<ArcStr> + 'static>) -> ArcStr {
    pub type AToStr<A: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(A) -> Result<ArcStr> + 'static>;

    let mut r#str: ArcStr = arcstr::literal!("");
    let mut len: i32 = 0;
    len = (eqs.clone().len() as i32);
    r#str = (if (len.clone() == 1) {literal!("1 system")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(len.clone())); __mm_s.push_str(&*literal!(" systems")); ArcStr::from(__mm_s) }}).clone();
    r#str = (if (len.clone() == 0) {r#str.clone()} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n   {")); __mm_s.push_str(&*stringDelimitList(List::map(eqs.clone(), r#fn.clone()), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }}).clone();
    r#str
}

fn sizeNumNonZeroTplString(mut inTpl: (i32, i32)) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut sz: i32 = 0;
    let mut nnz: i32 = 0;
    let mut density: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    (sz, nnz) = inTpl.clone();
    density = realDiv((metamodelica::OrderedFloat(100.0_f64)) * (intReal(nnz.clone())), (intReal(sz.clone())) * (intReal(sz.clone())));
    r#str = (System::snprintff((literal!("%.1f")).clone(), 20, density.clone())?).clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(sz.clone())); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("%)")); ArcStr::from(__mm_s) }).clone();
    Ok(r#str)
}

fn sizeNumNonZeroTornTplString(mut inTpl: (i32, i32, i32)) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut sz: i32 = 0;
    let mut nnz: i32 = 0;
    let mut others: i32 = 0;
    let mut density: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    (sz, others, nnz) = inTpl.clone();
    density = if (nnz.clone() == 0) {metamodelica::OrderedFloat(0.0_f64)} else {realDiv((metamodelica::OrderedFloat(100.0_f64)) * (intReal(nnz.clone())), (intReal(sz.clone())) * (intReal(sz.clone())))};
    r#str = (System::snprintff((literal!("%.1f")).clone(), 20, density.clone())?).clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(sz.clone())); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*intString(others.clone())); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("%)")); ArcStr::from(__mm_s) }).clone();
    Ok(r#str)
}

fn intTplString(mut inTpl: (i32, i32)) -> ArcStr {
    let mut outStr: ArcStr = arcstr::literal!("");
    let mut e: i32 = 0;
    let mut d: i32 = 0;
    (d, e) = inTpl.clone();
    outStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(d.clone())); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*intString(e.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
    outStr
}

fn dumpCompShort1(mut inSyst: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut inTpl: (i32, i32, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, i32, i32, i32, i32, i32, i32, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>))) -> Result<(i32, i32, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, i32, i32, i32, i32, i32, i32, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>))> {
    let mut outTpl: (i32, i32, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, i32, i32, i32, i32, i32, i32, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>));
    let mut vars: BackendDAE::Variables;
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut sys: i32 = 0;
    let mut inp: i32 = 0;
    let mut st: i32 = 0;
    let mut dvar: i32 = 0;
    let mut seq: i32 = 0;
    let mut salg: i32 = 0;
    let mut sarr: i32 = 0;
    let mut sce: i32 = 0;
    let mut swe: i32 = 0;
    let mut sie: i32 = 0;
    let mut inp1: i32 = 0;
    let mut st1: i32 = 0;
    let mut dvar1: i32 = 0;
    let mut seq1: i32 = 0;
    let mut salg1: i32 = 0;
    let mut sarr1: i32 = 0;
    let mut sce1: i32 = 0;
    let mut swe1: i32 = 0;
    let mut sie1: i32 = 0;
    let mut eqsys: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
    let mut eqsys1: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
    let mut meqsys: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>);
    let mut meqsys1: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>);
    let mut teqsys: (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>);
    let mut teqsys1: (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>);
    let mut teqsys_2: (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>);
    let mut teqsys1_2: (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>);
    let mut states: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut states1: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut discvars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut discvars1: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(inSyst.clone()) {
        Deref @ BackendDAE::EqSystem { orderedVars: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    vars = __pa0.clone();
    (sys, inp, st, states, dvar, discvars, seq, salg, sarr, sce, swe, sie, eqsys, meqsys, teqsys, teqsys_2) = inTpl.clone();
    (inp1, st1, states1, dvar1, discvars1) = BackendVariable::traverseBackendDAEVars(vars.clone(), (std::sync::Arc::new(traversingisStateTopInputVarFinder) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (i32, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) -> Result<(BackendDAE::Var, (i32, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>))> + 'static>), (inp.clone(), st.clone(), states.clone(), dvar.clone(), discvars.clone()))?;
    comps = BackendDAEUtil::getStrongComponents(inSyst.clone());
    (seq1, salg1, sarr1, sce1, swe1, sie1, eqsys1, meqsys1, teqsys1, teqsys1_2) = List::fold(comps.clone(), (std::sync::Arc::new(dumpCompShort2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::StrongComponent>, (i32, i32, i32, i32, i32, i32, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>))) -> Result<(i32, i32, i32, i32, i32, i32, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>))> + 'static>), (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone(), eqsys.clone(), meqsys.clone(), teqsys.clone(), teqsys_2.clone()));
    outTpl = (sys.clone() + 1, inp1.clone(), st1.clone(), states1.clone(), dvar1.clone(), discvars1.clone(), seq1.clone(), salg1.clone(), sarr1.clone(), sce1.clone(), swe1.clone(), sie1.clone(), eqsys1.clone(), meqsys1.clone(), teqsys1.clone(), teqsys1_2.clone());
    Ok(outTpl)
}

fn traversingisStateTopInputVarFinder(mut inVar: BackendDAE::Var, mut inTpl: (i32, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) -> Result<(BackendDAE::Var, (i32, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>))> {
    let mut outVar: BackendDAE::Var;
    let mut outTpl: (i32, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>);
    (outVar, outTpl) = 'mc: {
        let __mc_input = (inVar.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, (inp, st, states, dvar, discvars)) => {
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let true = (BackendVariable::isStateVar(v.clone())) else { bail!("pattern mismatch") };
                    cr = BackendVariable::varCref(v.clone())?;
                    Ok((v.clone(), (inp.clone(), st.clone() + 1, cons(cr.clone(), states.clone()), dvar.clone(), discvars.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, (inp, st, states, dvar, discvars)) => {
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let true = (BackendVariable::isVarDiscrete(v.clone())) else { bail!("pattern mismatch") };
                    cr = BackendVariable::varCref(v.clone())?;
                    Ok((v.clone(), (inp.clone(), st.clone(), states.clone(), dvar.clone() + 1, cons(cr.clone(), discvars.clone()))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, (inp, st, states, dvar, discvars)) => {
                    let true = (BackendVariable::isVarOnTopLevelAndInput(v.clone())) else { bail!("pattern mismatch") };
                    Ok((v.clone(), (inp.clone() + 1, st.clone(), states.clone(), dvar.clone(), discvars.clone())))
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

fn dumpCompShort2(mut inComp: Arc<BackendDAE::StrongComponent>, mut inTpl: (i32, i32, i32, i32, i32, i32, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>))) -> Result<(i32, i32, i32, i32, i32, i32, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>))> {
    let mut outTpl: (i32, i32, i32, i32, i32, i32, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>));
    outTpl = (::match_deref::match_deref! { match &((inComp.clone(), inTpl.clone())) {
        (Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { .. }, (seq, salg, sarr, sce, swe, sie, eqsys, meqsys, teqsys, teqsys2)) => {
            (seq.clone() + 1, salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone(), eqsys.clone(), meqsys.clone(), teqsys.clone(), teqsys2.clone())
        },
        (Deref @ BackendDAE::StrongComponent::SINGLEARRAY { .. }, (seq, salg, sarr, sce, swe, sie, eqsys, meqsys, teqsys, teqsys2)) => {
            (seq.clone(), salg.clone(), sarr.clone() + 1, sce.clone(), swe.clone(), sie.clone(), eqsys.clone(), meqsys.clone(), teqsys.clone(), teqsys2.clone())
        },
        (Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { .. }, (seq, salg, sarr, sce, swe, sie, eqsys, meqsys, teqsys, teqsys2)) => {
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone() + 1, eqsys.clone(), meqsys.clone(), teqsys.clone(), teqsys2.clone())
        },
        (Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { .. }, (seq, salg, sarr, sce, swe, sie, eqsys, meqsys, teqsys, teqsys2)) => {
            (seq.clone(), salg.clone() + 1, sarr.clone(), sce.clone(), swe.clone(), sie.clone(), eqsys.clone(), meqsys.clone(), teqsys.clone(), teqsys2.clone())
        },
        (Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { .. }, (seq, salg, sarr, sce, swe, sie, eqsys, meqsys, teqsys, teqsys2)) => {
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone() + 1, swe.clone(), sie.clone(), eqsys.clone(), meqsys.clone(), teqsys.clone(), teqsys2.clone())
        },
        (Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { .. }, (seq, salg, sarr, sce, swe, sie, eqsys, meqsys, teqsys, teqsys2)) => {
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone() + 1, sie.clone(), eqsys.clone(), meqsys.clone(), teqsys.clone(), teqsys2.clone())
        },
        (Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jacType: BackendDAE::JacobianType::JAC_CONSTANT, eqns: ilst, .. }, (seq, salg, sarr, sce, swe, sie, (e_jc, e_jt, e_jn, e_nj), meqsys, teqsys, teqsys2)) => {
            let mut e: i32 = 0;
            e = (ilst.clone().len() as i32);
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone(), (cons(e.clone(), e_jc.clone()), e_jt.clone(), e_jn.clone(), e_nj.clone()), meqsys.clone(), teqsys.clone(), teqsys2.clone())
        },
        (Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jacType: BackendDAE::JacobianType::JAC_LINEAR, jac: Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: Some(jac) }, eqns: ilst, .. }, (seq, salg, sarr, sce, swe, sie, (e_jc, e_jt, e_jn, e_nj), meqsys, teqsys, teqsys2)) => {
            let mut e: i32 = 0;
            let mut nnz: i32 = 0;
            e = (ilst.clone().len() as i32);
            nnz = (jac.clone().len() as i32);
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone(), (e_jc.clone(), cons((e.clone(), nnz.clone()), e_jt.clone()), e_jn.clone(), e_nj.clone()), meqsys.clone(), teqsys.clone(), teqsys2.clone())
        },
        (Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jacType: BackendDAE::JacobianType::JAC_NONLINEAR, eqns: ilst, .. }, (seq, salg, sarr, sce, swe, sie, (e_jc, e_jt, e_jn, e_nj), meqsys, teqsys, teqsys2)) => {
            let mut e: i32 = 0;
            e = (ilst.clone().len() as i32);
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone(), (e_jc.clone(), e_jt.clone(), cons(e.clone(), e_jn.clone()), e_nj.clone()), meqsys.clone(), teqsys.clone(), teqsys2.clone())
        },
        (Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jacType: BackendDAE::JacobianType::JAC_GENERIC, eqns: ilst, .. }, (seq, salg, sarr, sce, swe, sie, (e_jc, e_jt, e_jn, e_nj), meqsys, teqsys, teqsys2)) => {
            let mut e: i32 = 0;
            e = (ilst.clone().len() as i32);
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone(), (e_jc.clone(), e_jt.clone(), cons(e.clone(), e_jn.clone()), e_nj.clone()), meqsys.clone(), teqsys.clone(), teqsys2.clone())
        },
        (Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jacType: BackendDAE::JacobianType::JAC_NO_ANALYTIC, eqns: ilst, .. }, (seq, salg, sarr, sce, swe, sie, (e_jc, e_jt, e_jn, e_nj), meqsys, teqsys, teqsys2)) => {
            let mut e: i32 = 0;
            e = (ilst.clone().len() as i32);
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone(), (e_jc.clone(), e_jt.clone(), e_jn.clone(), cons(e.clone(), e_nj.clone())), meqsys.clone(), teqsys.clone(), teqsys2.clone())
        },
        (Deref @ BackendDAE::StrongComponent::TORNSYSTEM { linear: true, casualTearingSet: None, strictTearingSet: BackendDAE::TearingSet { jac: Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: _, sparsePattern: (_, _, _, nnz), coloring: _, .. }, innerEquations, tearingvars: ilst, .. }, .. }, (seq, salg, sarr, sce, swe, sie, eqsys, meqsys, (te_l, te_nl), (te_l2, te_nl2))) => {
            let mut e: i32 = 0;
            let mut d: i32 = 0;
            d = (ilst.clone().len() as i32);
            e = (innerEquations.clone().len() as i32);
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone(), eqsys.clone(), meqsys.clone(), (cons((d.clone(), e.clone(), nnz.clone()), te_l.clone()), te_nl.clone()), (cons((0, 0, 0), te_l2.clone()), te_nl2.clone()))
        },
        (Deref @ BackendDAE::StrongComponent::TORNSYSTEM { linear: false, casualTearingSet: None, strictTearingSet: BackendDAE::TearingSet { innerEquations, tearingvars: ilst, .. }, .. }, (seq, salg, sarr, sce, swe, sie, eqsys, meqsys, (te_l, te_nl), (te_l2, te_nl2))) => {
            let mut e: i32 = 0;
            let mut d: i32 = 0;
            d = (ilst.clone().len() as i32);
            e = (innerEquations.clone().len() as i32);
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone(), eqsys.clone(), meqsys.clone(), (te_l.clone(), cons((d.clone(), e.clone()), te_nl.clone())), (te_l2.clone(), cons((0, 0), te_nl2.clone())))
        },
        (Deref @ BackendDAE::StrongComponent::TORNSYSTEM { linear: true, casualTearingSet: None, strictTearingSet: BackendDAE::TearingSet { jac: Deref @ BackendDAE::Jacobian::EMPTY_JACOBIAN, innerEquations, tearingvars: ilst, .. }, .. }, (seq, salg, sarr, sce, swe, sie, eqsys, meqsys, (te_l, te_nl), (te_l2, te_nl2))) => {
            let mut e: i32 = 0;
            let mut d: i32 = 0;
            d = (ilst.clone().len() as i32);
            e = (innerEquations.clone().len() as i32);
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone(), eqsys.clone(), meqsys.clone(), (cons((d.clone(), e.clone(), 0), te_l.clone()), te_nl.clone()), (cons((0, 0, 0), te_l2.clone()), te_nl2.clone()))
        },
        (Deref @ BackendDAE::StrongComponent::TORNSYSTEM { linear: true, casualTearingSet: Some(BackendDAE::TearingSet { jac: Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: _, sparsePattern: (_, _, _, nnz2), coloring: _, .. }, innerEquations: innerEquations2, tearingvars: ilst2, .. }), strictTearingSet: BackendDAE::TearingSet { jac: Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: _, sparsePattern: (_, _, _, nnz), coloring: _, .. }, innerEquations, tearingvars: ilst, .. }, .. }, (seq, salg, sarr, sce, swe, sie, eqsys, meqsys, (te_l, te_nl), (te_l2, te_nl2))) => {
            let mut e: i32 = 0;
            let mut d: i32 = 0;
            let mut e2: i32 = 0;
            let mut d2: i32 = 0;
            d = (ilst.clone().len() as i32);
            e = (innerEquations.clone().len() as i32);
            d2 = (ilst2.clone().len() as i32);
            e2 = (innerEquations2.clone().len() as i32);
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone(), eqsys.clone(), meqsys.clone(), (cons((d.clone(), e.clone(), nnz.clone()), te_l.clone()), te_nl.clone()), (cons((d2.clone(), e2.clone(), nnz2.clone()), te_l2.clone()), te_nl2.clone()))
        },
        (Deref @ BackendDAE::StrongComponent::TORNSYSTEM { linear: false, casualTearingSet: Some(BackendDAE::TearingSet { innerEquations: innerEquations2, tearingvars: ilst2, .. }), strictTearingSet: BackendDAE::TearingSet { innerEquations, tearingvars: ilst, .. }, .. }, (seq, salg, sarr, sce, swe, sie, eqsys, meqsys, (te_l, te_nl), (te_l2, te_nl2))) => {
            let mut e: i32 = 0;
            let mut d: i32 = 0;
            let mut e2: i32 = 0;
            let mut d2: i32 = 0;
            d = (ilst.clone().len() as i32);
            e = (innerEquations.clone().len() as i32);
            d2 = (ilst2.clone().len() as i32);
            e2 = (innerEquations2.clone().len() as i32);
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone(), eqsys.clone(), meqsys.clone(), (te_l.clone(), cons((d.clone(), e.clone()), te_nl.clone())), (te_l2.clone(), cons((d2.clone(), e2.clone()), te_nl2.clone())))
        },
        _ => {
            println!("{}", (literal!("dumpCompShort2 failed with:\n")).clone());
            dumpComponent(inComp.clone(), None)?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outTpl)
}

pub fn dumpNrOfEquations(mut inDAE: Arc<BackendDAE::BackendDAE>, mut preStr: ArcStr) -> Result<()> {
    let mut nlst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut n: i32 = 0;
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ DAE { eqs: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    systs = __pa0.clone();
    nlst = List::map(systs.clone(), (std::sync::Arc::new(fnptr!(BackendDAEUtil::systemSize, Arc<BackendDAE::EqSystem>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>) -> Result<i32> + 'static>));
    n = List::fold(nlst.clone(), (std::sync::Arc::new(fnptr!(intAdd, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), 0);
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*preStr.clone()); __mm_s.push_str(&*literal!(" NrOfEquations: ")); __mm_s.push_str(&*intString(n.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub fn dumpCompInfo(mut compInfo: Arc<BackendDAE::CompInfo>) -> Result<()> {
    println!("{}", (printCompInfo(compInfo.clone())?).clone());
    Ok(())
}

fn printCompInfo(mut compInfo: Arc<BackendDAE::CompInfo>) -> Result<ArcStr> {
    let mut sOut: ArcStr = arcstr::literal!("");
    sOut = ('mc: {
        let __mc_input = compInfo.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::CompInfo::COUNTER { funcCalls: numFuncs, numOth, numLog, numRelations: numRel, numTrig, numDiv, numMul, numAdds, comp } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = (literal!("")).clone();
                    if BackendDAEUtil::isSingleEquationComp(comp.clone()) {
                        s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SE ")); __mm_s.push_str(&*printComponent(comp.clone(), None)?); ArcStr::from(__mm_s) }).clone();
                    } else if BackendDAEUtil::isWhenComp(comp.clone()) {
                        s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("WE ")); __mm_s.push_str(&*printComponent(comp.clone(), None)?); ArcStr::from(__mm_s) }).clone();
                    } else if BackendDAEUtil::isArrayComp(comp.clone()) {
                        s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("AE ")); __mm_s.push_str(&*printComponent(comp.clone(), None)?); ArcStr::from(__mm_s) }).clone();
                    }
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("\tadd|")); __mm_s.push_str(&*intString(numAdds.clone())); __mm_s.push_str(&*literal!("\tmul|")); __mm_s.push_str(&*intString(numMul.clone())); __mm_s.push_str(&*literal!("\tdiv|")); __mm_s.push_str(&*intString(numDiv.clone())); __mm_s.push_str(&*literal!("\ttrig|")); __mm_s.push_str(&*intString(numTrig.clone())); __mm_s.push_str(&*literal!("\trel|")); __mm_s.push_str(&*intString(numRel.clone())); __mm_s.push_str(&*literal!("\tlog|")); __mm_s.push_str(&*intString(numLog.clone())); __mm_s.push_str(&*literal!("\toth|")); __mm_s.push_str(&*intString(numOth.clone())); __mm_s.push_str(&*literal!("\tfuncs|")); __mm_s.push_str(&*intString(numFuncs.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::CompInfo::SYSTEM { density: dens, size, comp, allOperations: allOps } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = (literal!("")).clone();
                    if BackendDAEUtil::isLinearEqSystemComp(comp.clone()) {
                        s = (literal!("LSYS")).clone();
                    } else {
                        s = (literal!("NLSYS")).clone();
                    }
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*printComponent(comp.clone(), None)?); __mm_s.push_str(&*literal!("\tsize|")); __mm_s.push_str(&*intString(size.clone())); __mm_s.push_str(&*literal!("\tdens|")); __mm_s.push_str(&*intString(((dens.clone() * metamodelica::OrderedFloat(100.0_f64)).0 as i32))); __mm_s.push_str(&*printCompInfo(allOps.clone())?); ArcStr::from(__mm_s) }).clone();
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::CompInfo::TORN_ANALYSE { tornSize: size, comp, otherEqs, tornEqs } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TS ")); __mm_s.push_str(&*printComponent(comp.clone(), None)?); __mm_s.push_str(&*literal!("\tsize|")); __mm_s.push_str(&*intString(size.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("\tthe torn eqs:\t")); __mm_s.push_str(&*printCompInfo(tornEqs.clone())?); ArcStr::from(__mm_s) }).clone();
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("\tthe other eqs:\t")); __mm_s.push_str(&*printCompInfo(otherEqs.clone())?); ArcStr::from(__mm_s) }).clone();
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::CompInfo::NO_COMP { funcCalls: numFuncs, numOth, numLog, numRelations: numRel, numTrig, numDiv, numMul, numAdds } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = (literal!("NC")).clone();
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("\tadd|")); __mm_s.push_str(&*intString(numAdds.clone())); __mm_s.push_str(&*literal!("\tmul|")); __mm_s.push_str(&*intString(numMul.clone())); __mm_s.push_str(&*literal!("\tdiv|")); __mm_s.push_str(&*intString(numDiv.clone())); __mm_s.push_str(&*literal!("\ttrig|")); __mm_s.push_str(&*intString(numTrig.clone())); __mm_s.push_str(&*literal!("\trel|")); __mm_s.push_str(&*intString(numRel.clone())); __mm_s.push_str(&*literal!("\tlog|")); __mm_s.push_str(&*intString(numLog.clone())); __mm_s.push_str(&*literal!("\toth|")); __mm_s.push_str(&*intString(numOth.clone())); __mm_s.push_str(&*literal!("\tfuncs|")); __mm_s.push_str(&*intString(numFuncs.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!("Dont know this compInfo\n"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(sOut)
}

// =============================================================================
// section for all html-dumping functions
//
// =============================================================================
pub fn dumpEqSystemMatrixHTML(mut sys: Arc<BackendDAE::EqSystem>) -> Result<()> {
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    if isSome(sys.m.clone()) {
        m = Util::getOption(sys.m.clone())?;
    } else {
        (_, m, _) = BackendDAEUtil::getAdjacencyMatrix(sys.clone(), crate::BackendDAE::IndexType::NORMAL, None, false)?;
    }
    dumpEqSystem(sys.clone(), (literal!("SYS")).clone())?;
    dumpMatrixHTML(m.clone(), List::map(List::intRange(BackendDAEUtil::systemSize(sys.clone())), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), List::map(BackendVariable::varList(sys.orderedVars.clone())?, (std::sync::Arc::new(varStringShort) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<ArcStr> + 'static>)), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("MATRIX_")); __mm_s.push_str(&*intString(BackendDAEUtil::systemSize(sys.clone()))); ArcStr::from(__mm_s) }).clone())?;
    Ok(())
}

pub fn dumpEqSystemBLTmatrixHTML(mut sys: Arc<BackendDAE::EqSystem>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = sys.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqs, m: _, mT: _, mapping: _, matching: Deref @ BackendDAE::Matching::MATCHING { comps, .. }, stateSets: _, partitionKind: _, removedEqs: _ } => {
                    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut vIdxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut eIdxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut vars = (*vars).clone();
                    let mut eqs = (*eqs).clone();
                    (varLst, vIdxs, eqLst, eIdxs) = BackendDAEUtil::getStrongComponentsVarsAndEquations(comps.clone(), vars.clone(), eqs.clone())?;
                    eqs = BackendEquation::listEquation(eqLst.clone())?;
                    vars = BackendVariable::listVar1(varLst.clone());
                    (m, _) = BackendDAEUtil::adjacencyMatrixDispatch(vars.clone(), eqs.clone(), crate::BackendDAE::IndexType::NORMAL, None, false)?;
                    dumpMatrixHTML(m.clone(), List::map(eIdxs.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), List::map(vIdxs.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BLT_MATRIX_")); __mm_s.push_str(&*intString(BackendDAEUtil::systemSize(sys.clone()))); ArcStr::from(__mm_s) }).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("dumpEqSystemBLTmatrixHTML does not output anything since there is no BLT sorting.")).clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn dumpMatrixHTML(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowNames: Arc<metamodelica::List<ArcStr>>, mut columNames: Arc<metamodelica::List<ArcStr>>, mut fileName: ArcStr) -> Result<()> {
    let mut size: i32 = 0;
    size = (m.clone().borrow().len() as i32);
    if (rowNames.clone().len() as i32) == size.clone() && (columNames.clone().len() as i32) == size.clone() {
        DumpHTML::dumpMatrixHTML(m.clone(), rowNames.clone(), columNames.clone(), (fileName.clone()).clone())?;
    } else {
        DumpHTML::dumpMatrixHTML(m.clone(), List::fill((literal!("?")).clone(), size.clone()), List::fill((literal!("?")).clone(), size.clone()), (fileName.clone()).clone())?;
    }
    Ok(())
}

// =============================================================================
// section for all graphML dumping functions
//
// =============================================================================
pub fn dumpBipartiteGraphDAE(mut dae: Arc<BackendDAE::BackendDAE>, mut fileName: ArcStr) -> Result<()> {
    let mut vars: BackendDAE::Variables;
    let mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut eqSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut shared: Arc<BackendDAE::Shared>;
    let mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut varAtts: Arc<metamodelica::List<(bool, ArcStr)>> = metamodelica::nil();
    let mut eqAtts: Arc<metamodelica::List<(bool, ArcStr)>> = metamodelica::nil();
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(dae.clone()) {
        Deref @ DAE { shared: __pa0, eqs: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    shared = __pa0.clone();
    eqSysts = __pa1.clone();
    eqLst = List::flatten(List::mapMap(eqSysts.clone(), (std::sync::Arc::new(fnptr!(BackendEquation::getEqnsFromEqSystem, Arc<BackendDAE::EqSystem>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> + 'static>), (std::sync::Arc::new(fnptr!(BackendEquation::equationList, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> + 'static>)));
    varLst = List::flatten(List::mapMap(eqSysts.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::daeVars, Arc<BackendDAE::EqSystem>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>) -> Result<BackendDAE::Variables> + 'static>), (std::sync::Arc::new(BackendVariable::varList) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> + 'static>)));
    vars = BackendVariable::listVar1(varLst.clone());
    eqs = BackendEquation::listEquation(eqLst.clone())?;
    (_, m, _, _, _) = BackendDAEUtil::getAdjacencyMatrixScalar(Arc::new(BackendDAE::EqSystem { orderedVars: vars.clone(), orderedEqs: eqs.clone(), m: None, mT: None, mapping: None, matching: Arc::new(crate::BackendDAE::Matching::NO_MATCHING), stateSets: metamodelica::nil(), partitionKind: crate::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, removedEqs: BackendEquation::emptyEqns() }), crate::BackendDAE::IndexType::SOLVABLE, Some(BackendDAEUtil::getFunctions(shared.clone())?), BackendDAEUtil::isInitializationDAE(shared.clone()))?;
    varAtts = List::threadMap(List::fill(false, (varLst.clone().len() as i32)), List::fill((literal!("")).clone(), (varLst.clone().len() as i32)), std::sync::Arc::new(fnptr!(Util::makeTuple, _, _)));
    eqAtts = List::threadMap(List::fill(false, (eqLst.clone().len() as i32)), List::fill((literal!("")).clone(), (eqLst.clone().len() as i32)), std::sync::Arc::new(fnptr!(Util::makeTuple, _, _)));
    dumpBipartiteGraphStrongComponent2(vars.clone(), eqs.clone(), m.clone(), varAtts.clone(), eqAtts.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BipartiteGraph_")); __mm_s.push_str(&*fileName.clone()); ArcStr::from(__mm_s) }).clone())?;
    Ok(())
}

pub fn dumpBipartiteGraphEqSystem(mut syst: Arc<BackendDAE::EqSystem>, mut shared: Arc<BackendDAE::Shared>, mut fileName: ArcStr) -> Result<()> {
    let mut vars: BackendDAE::Variables;
    let mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut me: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
    let mut meT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mO: Option<metamodelica::Array<Arc<metamodelica::List<i32>>>> = None;
    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut varAtts: Arc<metamodelica::List<(bool, ArcStr)>> = metamodelica::nil();
    let mut eqAtts: Arc<metamodelica::List<(bool, ArcStr)>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(syst.clone()) {
        Deref @ BackendDAE::EqSystem { m: __pa0, orderedEqs: __pa1, orderedVars: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    mO = __pa0.clone();
    eqs = __pa1.clone();
    vars = __pa2.clone();
    varLst = BackendVariable::varList(vars.clone())?;
    varAtts = List::threadMap(List::fill(false, (varLst.clone().len() as i32)), List::fill((literal!("")).clone(), (varLst.clone().len() as i32)), std::sync::Arc::new(fnptr!(Util::makeTuple, _, _)));
    eqAtts = List::threadMap(List::fill(false, BackendEquation::equationArraySize(eqs.clone())?), List::fill((literal!("")).clone(), BackendEquation::equationArraySize(eqs.clone())?), std::sync::Arc::new(fnptr!(Util::makeTuple, _, _)));
    if isSome(mO.clone()) {
        dumpBipartiteGraphStrongComponent2(vars.clone(), eqs.clone(), Util::getOption(mO.clone())?, varAtts.clone(), eqAtts.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BipartiteGraph_")); __mm_s.push_str(&*fileName.clone()); ArcStr::from(__mm_s) }).clone())?;
    } else {
        (_, m, _, _, _) = BackendDAEUtil::getAdjacencyMatrixScalar(syst.clone(), crate::BackendDAE::IndexType::SOLVABLE, Some(BackendDAEUtil::getFunctions(shared.clone())?), BackendDAEUtil::isInitializationDAE(shared.clone()))?;
        dumpBipartiteGraphStrongComponent2(vars.clone(), eqs.clone(), m.clone(), varAtts.clone(), eqAtts.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BipartiteGraph2_")); __mm_s.push_str(&*fileName.clone()); ArcStr::from(__mm_s) }).clone())?;
    }
    Ok(())
}

pub fn dumpBipartiteGraphStrongComponent2(mut varsIn: BackendDAE::Variables, mut eqsIn: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut varAtts: Arc<metamodelica::List<(bool, ArcStr)>>, mut eqAtts: Arc<metamodelica::List<(bool, ArcStr)>>, mut name: ArcStr) -> Result<()> {
    let mut nameAttIdx: i32 = 0;
    let mut typeAttIdx: i32 = 0;
    let mut idxAttIdx: i32 = 0;
    let mut numVars: i32 = 0;
    let mut numEqs: i32 = 0;
    let mut varRange: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqRange: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut graphInfo: GraphML::GraphInfo;
    let mut graphIdx: i32 = 0;
    numEqs = BackendEquation::equationArraySize(eqsIn.clone())?;
    numVars = BackendVariable::varsSize(varsIn.clone())?;
    varRange = List::intRange(numVars.clone());
    eqRange = List::intRange(numEqs.clone());
    graphInfo = GraphML::createGraphInfo();
    let (__pa0, (_, __pa1)) = GraphML::addGraph((literal!("EqSystemGraph")).clone(), true, graphInfo.clone())?;
    graphInfo = __pa0.clone();
    graphIdx = __pa1.clone();
    let (__pa2, (_, __pa3)) = GraphML::addAttribute((literal!("")).clone(), (literal!("type")).clone(), openmodelica_susan::GraphML::AttributeType::TYPE_STRING, openmodelica_susan::GraphML::AttributeTarget::TARGET_NODE, graphInfo.clone())?;
    graphInfo = __pa2.clone();
    typeAttIdx = __pa3.clone();
    let (__pa4, (_, __pa5)) = GraphML::addAttribute((literal!("")).clone(), (literal!("name")).clone(), openmodelica_susan::GraphML::AttributeType::TYPE_STRING, openmodelica_susan::GraphML::AttributeTarget::TARGET_NODE, graphInfo.clone())?;
    graphInfo = __pa4.clone();
    nameAttIdx = __pa5.clone();
    let (__pa6, (_, __pa7)) = GraphML::addAttribute((literal!("")).clone(), (literal!("systIdx")).clone(), openmodelica_susan::GraphML::AttributeType::TYPE_STRING, openmodelica_susan::GraphML::AttributeTarget::TARGET_NODE, graphInfo.clone())?;
    graphInfo = __pa6.clone();
    idxAttIdx = __pa7.clone();
    (graphInfo, graphIdx) = addEqNodesToGraph(eqsIn.clone(), eqAtts.clone(), list![nameAttIdx.clone(), typeAttIdx.clone(), idxAttIdx.clone()], (graphInfo.clone(), graphIdx.clone()))?;
    (graphInfo, graphIdx) = List::fold3(varRange.clone(), (std::sync::Arc::new(addVarNodeToGraph) as std::sync::Arc<dyn ::std::ops::Fn(i32, BackendDAE::Variables, Arc<metamodelica::List<(bool, ArcStr)>>, Arc<metamodelica::List<i32>>, (GraphML::GraphInfo, i32)) -> Result<(GraphML::GraphInfo, i32)> + 'static>), varsIn.clone(), varAtts.clone(), list![nameAttIdx.clone(), typeAttIdx.clone(), idxAttIdx.clone()], (graphInfo.clone(), graphIdx.clone()));
    graphInfo = List::fold1(eqRange.clone(), (std::sync::Arc::new(addEdgeToGraph) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, GraphML::GraphInfo) -> Result<GraphML::GraphInfo> + 'static>), mIn.clone(), graphInfo.clone());
    GraphML::dumpGraph(graphInfo.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(".graphml")); ArcStr::from(__mm_s) }).clone())?;
    Ok(())
}

fn addEqNodesToGraph(mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut attsIn: Arc<metamodelica::List<(bool, ArcStr)>>, mut attributeIdcs: Arc<metamodelica::List<i32>>, mut graphInfoIn: (GraphML::GraphInfo, i32)) -> Result<(GraphML::GraphInfo, i32)> {
    let mut graphInfoOut: (GraphML::GraphInfo, i32);
    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut isResEq: bool = false;
    let mut nameAttrIdx: i32 = 0;
    let mut typeAttrIdx: i32 = 0;
    let mut idxAttrIdx: i32 = 0;
    let mut graphIdx: i32 = 0;
    let mut size: i32 = 0;
    let mut numEqs: i32 = 0;
    let mut e: i32 = 0;
    let mut eAbs: i32 = 0;
    let mut nextE: i32 = 0;
    let mut eqString: ArcStr = arcstr::literal!("");
    let mut eqNodeId: ArcStr = arcstr::literal!("");
    let mut idxString: ArcStr = arcstr::literal!("");
    let mut typeStr: ArcStr = arcstr::literal!("");
    let mut daeIdxStr: ArcStr = arcstr::literal!("");
    let mut eqChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut graphInfo: GraphML::GraphInfo;
    let mut nodeLabel: GraphML::NodeLabel;
    nameAttrIdx = (attributeIdcs.clone()).get(1)?;
    typeAttrIdx = (attributeIdcs.clone()).get(2)?;
    idxAttrIdx = (attributeIdcs.clone()).get(3)?;
    (graphInfo, graphIdx) = graphInfoIn.clone();
    numEqs = BackendEquation::getNumberOfEquations(eqs.clone());
    e = 1;
    eAbs = 1;
    size = 1;
    while e.clone() <= numEqs.clone() {
        eq = BackendEquation::get(eqs.clone(), e.clone());
        size = BackendEquation::equationSize(eq.clone())?;
        nextE = eAbs.clone() + size.clone();
        while nextE.clone() > eAbs.clone() {
            nameAttrIdx = (attributeIdcs.clone()).get(1)?;
            typeAttrIdx = (attributeIdcs.clone()).get(2)?;
            idxAttrIdx = (attributeIdcs.clone()).get(3)?;
            isResEq = Util::tuple21((attsIn.clone()).get(e.clone())?);
            daeIdxStr = (Util::tuple22((attsIn.clone()).get(e.clone())?)).clone();
            typeStr = (if (isResEq.clone()) {literal!("residualEq")} else {literal!("otherEq")}).clone();
            let __pa0 = ::match_deref::match_deref! { match &(BackendEquation::getList(list![e.clone()], eqs.clone())) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            eq = __pa0.clone();
            eqString = (equationString(eq.clone())?).clone();
            eqNodeId = (getEqNodeIdx(eAbs.clone())).clone();
            idxString = (intString(eAbs.clone())).clone();
            nodeLabel = GraphML::NodeLabel::NODELABEL_INTERNAL { text: (idxString.clone()).clone(), backgroundColor: None, fontStyle: openmodelica_susan::GraphML::FontStyle::FONTPLAIN };
            let (__pa2, (_, _)) = GraphML::addNode((eqNodeId.clone()).clone(), (arcstr::literal!(GraphML::COLOR_GREEN2)).clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![nodeLabel.clone()], openmodelica_susan::GraphML::ShapeType::RECTANGLE, Some((eqString.clone()).clone()), list![(nameAttrIdx.clone(), eqString.clone()), (typeAttrIdx.clone(), typeStr.clone()), (idxAttrIdx.clone(), daeIdxStr.clone())], graphIdx.clone(), graphInfo.clone())?;
            graphInfo = __pa2.clone();
            eAbs = eAbs.clone() + 1;
            size = size.clone() - 1;
        }
        e = e.clone() + 1;
    }
    graphInfoOut = (graphInfo.clone(), graphIdx.clone());
    Ok(graphInfoOut)
}

pub fn dumpDAGStrongComponent(mut graphIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut metaIn: HpcOmTaskGraph::TaskGraphMeta, mut fileName: ArcStr) -> Result<()> {
    let mut graphIdx: i32 = 0;
    let mut nameAttIdx: i32 = 0;
    let mut graphInfo: GraphML::GraphInfo;
    graphInfo = GraphML::createGraphInfo();
    let (__pa0, (_, __pa1)) = GraphML::addGraph((literal!("TornSystemGraph")).clone(), true, graphInfo.clone())?;
    graphInfo = __pa0.clone();
    graphIdx = __pa1.clone();
    let (__pa2, (_, __pa3)) = GraphML::addAttribute((literal!("")).clone(), (literal!("Name")).clone(), openmodelica_susan::GraphML::AttributeType::TYPE_STRING, openmodelica_susan::GraphML::AttributeTarget::TARGET_NODE, graphInfo.clone())?;
    graphInfo = __pa2.clone();
    nameAttIdx = __pa3.clone();
    graphInfo = buildGraphInfoDAG(graphIn.clone(), metaIn.clone(), graphInfo.clone(), graphIdx.clone(), list![nameAttIdx.clone()])?;
    GraphML::dumpGraph(graphInfo.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*fileName.clone()); __mm_s.push_str(&*literal!(".graphml")); ArcStr::from(__mm_s) }).clone())?;
    Ok(())
}

fn buildGraphInfoDAG(mut graphIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut metaIn: HpcOmTaskGraph::TaskGraphMeta, mut graphInfoIn: GraphML::GraphInfo, mut graphIdx: i32, mut attIdcs: Arc<metamodelica::List<i32>>) -> Result<GraphML::GraphInfo> {
    let mut graphInfoOut: GraphML::GraphInfo;
    let mut graphInfo: GraphML::GraphInfo;
    let mut nodeIdcs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut nodes: Arc<metamodelica::List<GraphML::Node>> = metamodelica::nil();
    let mut nameAttIdx: i32 = 0;
    nameAttIdx = listHead(attIdcs.clone())?;
    nodeIdcs = List::intRange((graphIn.clone().borrow().len() as i32));
    graphInfoOut = List::fold4(nodeIdcs.clone(), (std::sync::Arc::new(addNodeToDAG) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, HpcOmTaskGraph::TaskGraphMeta, i32, Arc<metamodelica::List<i32>>, GraphML::GraphInfo) -> Result<GraphML::GraphInfo> + 'static>), graphIn.clone(), metaIn.clone(), graphIdx.clone(), list![nameAttIdx.clone()], graphInfoIn.clone());
    let GraphML::GRAPHINFO { nodes: __pa0, .. } = (graphInfoOut.clone()) else { bail!("pattern mismatch") };
    nodes = __pa0.clone();
    Ok(graphInfoOut)
}

fn addNodeToDAG(mut nodeIdx: i32, mut graphIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut metaIn: HpcOmTaskGraph::TaskGraphMeta, mut graphIdx: i32, mut atts: Arc<metamodelica::List<i32>>, mut graphInfoIn: GraphML::GraphInfo) -> Result<GraphML::GraphInfo> {
    let mut graphInfoOut: GraphML::GraphInfo;
    let mut tmpGraph: GraphML::GraphInfo;
    let mut nameAttIdx: i32 = 0;
    let mut childNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut nodeLabel: GraphML::NodeLabel;
    let mut nodeString: ArcStr = arcstr::literal!("");
    let mut nodeDesc: ArcStr = arcstr::literal!("");
    let mut compName: ArcStr = arcstr::literal!("");
    let HpcOmTaskGraph::TASKGRAPHMETA { compDescs: __pa0, inComps: __pa1, .. } = (metaIn.clone()) else { bail!("pattern mismatch") };
    compDescs = __pa0.clone();
    inComps = __pa1.clone();
    nodeDesc = (compDescs.clone().borrow()[(nodeIdx.clone()-1) as usize].clone()).clone();
    nodeString = (intString(nodeIdx.clone())).clone();
    compName = stringDelimitList(List::map(inComps.clone().borrow()[(nodeIdx.clone()-1) as usize].clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone());
    nameAttIdx = (atts.clone()).get(1)?;
    nodeLabel = GraphML::NodeLabel::NODELABEL_INTERNAL { text: (nodeString.clone()).clone(), backgroundColor: None, fontStyle: openmodelica_susan::GraphML::FontStyle::FONTPLAIN };
    let (__pa2, (_, _)) = GraphML::addNode(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Node")); __mm_s.push_str(&*intString(nodeIdx.clone())); ArcStr::from(__mm_s) }).clone(), (arcstr::literal!(GraphML::COLOR_ORANGE)).clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![nodeLabel.clone()], openmodelica_susan::GraphML::ShapeType::RECTANGLE, Some((nodeDesc.clone()).clone()), list![(nameAttIdx.clone(), compName.clone())], graphIdx.clone(), graphInfoIn.clone())?;
    tmpGraph = __pa2.clone();
    childNodes = graphIn.clone().borrow()[(nodeIdx.clone()-1) as usize].clone();
    graphInfoOut = List::fold1(childNodes.clone(), (std::sync::Arc::new(addDirectedEdge) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, GraphML::GraphInfo) -> Result<GraphML::GraphInfo> + 'static>), nodeIdx.clone(), tmpGraph.clone());
    Ok(graphInfoOut)
}

fn addDirectedEdge(mut child: i32, mut parent: i32, mut graphInfoIn: GraphML::GraphInfo) -> Result<GraphML::GraphInfo> {
    let mut graphInfoOut: GraphML::GraphInfo;
    let (__pa0, (_, _)) = GraphML::addEdge(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Edge")); __mm_s.push_str(&*intString(parent.clone())); __mm_s.push_str(&*intString(child.clone())); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Node")); __mm_s.push_str(&*intString(child.clone())); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Node")); __mm_s.push_str(&*intString(parent.clone())); ArcStr::from(__mm_s) }).clone(), (arcstr::literal!(GraphML::COLOR_BLACK)).clone(), openmodelica_susan::GraphML::LineType::LINE, GraphML::LINEWIDTH_STANDARD.clone(), false, metamodelica::nil(), (openmodelica_susan::GraphML::ArrowType::ARROWNONE, openmodelica_susan::GraphML::ArrowType::ARROWSTANDART), metamodelica::nil(), graphInfoIn.clone())?;
    graphInfoOut = __pa0.clone();
    Ok(graphInfoOut)
}

fn addVarNodeToGraph(mut indx: i32, mut vars: BackendDAE::Variables, mut attsIn: Arc<metamodelica::List<(bool, ArcStr)>>, mut attributeIdcs: Arc<metamodelica::List<i32>>, mut graphInfoIn: (GraphML::GraphInfo, i32)) -> Result<(GraphML::GraphInfo, i32)> {
    let mut graphInfoOut: (GraphML::GraphInfo, i32);
    let mut var: BackendDAE::Var;
    let mut isTearVar: bool = false;
    let mut nameAttrIdx: i32 = 0;
    let mut typeAttIdx: i32 = 0;
    let mut idxAttrIdx: i32 = 0;
    let mut graphIdx: i32 = 0;
    let mut varString: ArcStr = arcstr::literal!("");
    let mut varNodeId: ArcStr = arcstr::literal!("");
    let mut idxString: ArcStr = arcstr::literal!("");
    let mut typeStr: ArcStr = arcstr::literal!("");
    let mut daeIdxStr: ArcStr = arcstr::literal!("");
    let mut varChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut graphInfo: GraphML::GraphInfo;
    let mut nodeLabel: GraphML::NodeLabel;
    (graphInfo, graphIdx) = graphInfoIn.clone();
    nameAttrIdx = (attributeIdcs.clone()).get(1)?;
    typeAttIdx = (attributeIdcs.clone()).get(2)?;
    idxAttrIdx = (attributeIdcs.clone()).get(3)?;
    isTearVar = Util::tuple21((attsIn.clone()).get(indx.clone())?);
    daeIdxStr = (Util::tuple22((attsIn.clone()).get(indx.clone())?)).clone();
    typeStr = (if (isTearVar.clone()) {literal!("tearingVar")} else {literal!("otherVar")}).clone();
    var = BackendVariable::getVarAt(vars.clone(), indx.clone())?;
    varString = (self::varString(var.clone())?).clone();
    varNodeId = (getVarNodeIdx(indx.clone())).clone();
    idxString = (intString(indx.clone())).clone();
    nodeLabel = GraphML::NodeLabel::NODELABEL_INTERNAL { text: (idxString.clone()).clone(), backgroundColor: None, fontStyle: openmodelica_susan::GraphML::FontStyle::FONTPLAIN };
    (graphInfo, _) = GraphML::addNode((varNodeId.clone()).clone(), (arcstr::literal!(GraphML::COLOR_ORANGE2)).clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![nodeLabel.clone()], openmodelica_susan::GraphML::ShapeType::ELLIPSE, Some((varString.clone()).clone()), list![(nameAttrIdx.clone(), varString.clone()), (typeAttIdx.clone(), typeStr.clone()), (idxAttrIdx.clone(), daeIdxStr.clone())], graphIdx.clone(), graphInfo.clone())?;
    graphInfoOut = (graphInfo.clone(), graphIdx.clone());
    Ok(graphInfoOut)
}

fn addEqNodeToGraph(mut indx: i32, mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut attsIn: Arc<metamodelica::List<(bool, ArcStr)>>, mut attributeIdcs: Arc<metamodelica::List<i32>>, mut graphInfoIn: (GraphML::GraphInfo, i32)) -> Result<(GraphML::GraphInfo, i32)> {
    let mut graphInfoOut: (GraphML::GraphInfo, i32);
    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut isResEq: bool = false;
    let mut nameAttrIdx: i32 = 0;
    let mut typeAttrIdx: i32 = 0;
    let mut idxAttrIdx: i32 = 0;
    let mut graphIdx: i32 = 0;
    let mut eqString: ArcStr = arcstr::literal!("");
    let mut eqNodeId: ArcStr = arcstr::literal!("");
    let mut idxString: ArcStr = arcstr::literal!("");
    let mut typeStr: ArcStr = arcstr::literal!("");
    let mut daeIdxStr: ArcStr = arcstr::literal!("");
    let mut eqChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut graphInfo: GraphML::GraphInfo;
    let mut nodeLabel: GraphML::NodeLabel;
    (graphInfo, graphIdx) = graphInfoIn.clone();
    nameAttrIdx = (attributeIdcs.clone()).get(1)?;
    typeAttrIdx = (attributeIdcs.clone()).get(2)?;
    idxAttrIdx = (attributeIdcs.clone()).get(3)?;
    isResEq = Util::tuple21((attsIn.clone()).get(indx.clone())?);
    daeIdxStr = (Util::tuple22((attsIn.clone()).get(indx.clone())?)).clone();
    typeStr = (if (isResEq.clone()) {literal!("residualEq")} else {literal!("otherEq")}).clone();
    let __pa0 = ::match_deref::match_deref! { match &(BackendEquation::getList(list![indx.clone()], eqs.clone())) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    eq = __pa0.clone();
    eqString = (equationString(eq.clone())?).clone();
    eqNodeId = (getEqNodeIdx(indx.clone())).clone();
    idxString = (intString(indx.clone())).clone();
    nodeLabel = GraphML::NodeLabel::NODELABEL_INTERNAL { text: (idxString.clone()).clone(), backgroundColor: None, fontStyle: openmodelica_susan::GraphML::FontStyle::FONTPLAIN };
    (graphInfo, _) = GraphML::addNode((eqNodeId.clone()).clone(), (arcstr::literal!(GraphML::COLOR_GREEN2)).clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![nodeLabel.clone()], openmodelica_susan::GraphML::ShapeType::RECTANGLE, Some((eqString.clone()).clone()), list![(nameAttrIdx.clone(), eqString.clone()), (typeAttrIdx.clone(), typeStr.clone()), (idxAttrIdx.clone(), daeIdxStr.clone())], graphIdx.clone(), graphInfo.clone())?;
    graphInfoOut = (graphInfo.clone(), graphIdx.clone());
    Ok(graphInfoOut)
}

fn addEdgeToGraph(mut eqIdx: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut graphInfoIn: GraphML::GraphInfo) -> Result<GraphML::GraphInfo> {
    let mut graphInfoOut: GraphML::GraphInfo;
    let mut varLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    varLst = m.clone().borrow()[(eqIdx.clone()-1) as usize].clone();
    graphInfoOut = List::fold1(varLst.clone(), (std::sync::Arc::new(addEdgeToGraph2) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, GraphML::GraphInfo) -> Result<GraphML::GraphInfo> + 'static>), eqIdx.clone(), graphInfoIn.clone());
    Ok(graphInfoOut)
}

fn addEdgeToGraph2(mut varIdxIn: i32, mut eqIdx: i32, mut graphInfoIn: GraphML::GraphInfo) -> Result<GraphML::GraphInfo> {
    let mut graphInfoOut: GraphML::GraphInfo;
    let mut varIdx: i32 = 0;
    let mut eqNodeId: ArcStr = arcstr::literal!("");
    let mut varNodeId: ArcStr = arcstr::literal!("");
    let mut lt: GraphML::LineType = GraphML::LineType::DASHED;
    if varIdxIn.clone() <= 0 {
        lt = openmodelica_susan::GraphML::LineType::DASHED;
    } else {
        lt = openmodelica_susan::GraphML::LineType::LINE;
    }
    varIdx = intAbs(varIdxIn.clone());
    eqNodeId = (getEqNodeIdx(eqIdx.clone())).clone();
    varNodeId = (getVarNodeIdx(varIdx.clone())).clone();
    (graphInfoOut, _) = GraphML::addEdge(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Edge_")); __mm_s.push_str(&*intString(varIdx.clone())); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(eqIdx.clone())); ArcStr::from(__mm_s) }).clone(), (varNodeId.clone()).clone(), (eqNodeId.clone()).clone(), (arcstr::literal!(GraphML::COLOR_BLACK)).clone(), lt.clone(), GraphML::LINEWIDTH_STANDARD.clone(), false, metamodelica::nil(), (openmodelica_susan::GraphML::ArrowType::ARROWNONE, openmodelica_susan::GraphML::ArrowType::ARROWNONE), metamodelica::nil(), graphInfoIn.clone())?;
    Ok(graphInfoOut)
}

fn getVarNodeIdx(mut idx: i32) -> ArcStr {
    let mut varString: ArcStr = arcstr::literal!("");
    varString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("varNode")); __mm_s.push_str(&*intString(intAbs(idx.clone()))); ArcStr::from(__mm_s) }).clone();
    varString
}

fn getEqNodeIdx(mut idx: i32) -> ArcStr {
    let mut eqString: ArcStr = arcstr::literal!("");
    eqString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("eqNode")); __mm_s.push_str(&*intString(intAbs(idx.clone()))); ArcStr::from(__mm_s) }).clone();
    eqString
}

pub fn dumpBackendDAEBipartiteGraph(mut dae: Arc<BackendDAE::BackendDAE>, mut filename: ArcStr) -> Result<()> {
    let mut graphIdx: i32 = 0;
    let mut sysIdx: i32 = 0;
    let mut varIdx: i32 = 0;
    let mut eqIdx: i32 = 0;
    let mut order: i32 = 0;
    let mut nameAttIdx: i32 = 0;
    let mut varAttIdx: i32 = 0;
    let mut eqAttIdx: i32 = 0;
    let mut sysAttIdx: i32 = 0;
    let mut tearAttIdx: i32 = 0;
    let mut compAttIdx: i32 = 0;
    let mut orderAttIdx: i32 = 0;
    let mut tearInfo: ArcStr = arcstr::literal!("");
    let mut nodeColor: ArcStr = arcstr::literal!("");
    let mut graphInfo: GraphML::GraphInfo;
    let mut shapeType: GraphML::ShapeType = GraphML::ShapeType::DIAMOND;
    let mut lineType: GraphML::LineType = GraphML::LineType::DASHED;
    let mut lineWidth: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut borderWidth: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared>;
    let mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut eqIdxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut varIdxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut adjVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut vars: BackendDAE::Variables;
    let mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut ass2: metamodelica::Array<i32>;
    graphInfo = GraphML::createGraphInfo();
    let (__pa0, (_, __pa1)) = GraphML::addGraph((literal!("TaskGraph")).clone(), true, graphInfo.clone())?;
    graphInfo = __pa0.clone();
    graphIdx = __pa1.clone();
    let (__pa2, (_, __pa3)) = GraphML::addAttribute((literal!("")).clone(), (literal!("Name")).clone(), openmodelica_susan::GraphML::AttributeType::TYPE_STRING, openmodelica_susan::GraphML::AttributeTarget::TARGET_NODE, graphInfo.clone())?;
    graphInfo = __pa2.clone();
    nameAttIdx = __pa3.clone();
    let (__pa4, (_, __pa5)) = GraphML::addAttribute((literal!("")).clone(), (literal!("VarIdx")).clone(), openmodelica_susan::GraphML::AttributeType::TYPE_STRING, openmodelica_susan::GraphML::AttributeTarget::TARGET_NODE, graphInfo.clone())?;
    graphInfo = __pa4.clone();
    varAttIdx = __pa5.clone();
    let (__pa6, (_, __pa7)) = GraphML::addAttribute((literal!("")).clone(), (literal!("EqIdx")).clone(), openmodelica_susan::GraphML::AttributeType::TYPE_STRING, openmodelica_susan::GraphML::AttributeTarget::TARGET_NODE, graphInfo.clone())?;
    graphInfo = __pa6.clone();
    eqAttIdx = __pa7.clone();
    let (__pa8, (_, __pa9)) = GraphML::addAttribute((literal!("")).clone(), (literal!("SysIdx")).clone(), openmodelica_susan::GraphML::AttributeType::TYPE_STRING, openmodelica_susan::GraphML::AttributeTarget::TARGET_NODE, graphInfo.clone())?;
    graphInfo = __pa8.clone();
    sysAttIdx = __pa9.clone();
    let (__pa10, (_, __pa11)) = GraphML::addAttribute((literal!("")).clone(), (literal!("Tearing")).clone(), openmodelica_susan::GraphML::AttributeType::TYPE_STRING, openmodelica_susan::GraphML::AttributeTarget::TARGET_NODE, graphInfo.clone())?;
    graphInfo = __pa10.clone();
    tearAttIdx = __pa11.clone();
    let (__pa12, (_, __pa13)) = GraphML::addAttribute((literal!("")).clone(), (literal!("SCC")).clone(), openmodelica_susan::GraphML::AttributeType::TYPE_STRING, openmodelica_susan::GraphML::AttributeTarget::TARGET_NODE, graphInfo.clone())?;
    graphInfo = __pa12.clone();
    compAttIdx = __pa13.clone();
    let (__pa14, (_, __pa15)) = GraphML::addAttribute((literal!("")).clone(), (literal!("executionOrder")).clone(), openmodelica_susan::GraphML::AttributeType::TYPE_STRING, openmodelica_susan::GraphML::AttributeTarget::TARGET_NODE, graphInfo.clone())?;
    graphInfo = __pa14.clone();
    orderAttIdx = __pa15.clone();
    let (__pa16, __pa17) = ::match_deref::match_deref! { match &(dae.clone()) {
        Deref @ DAE { UNIQUEIO: __pa16, derivativeNamePrefix: __pa17, .. } => (__pa16.clone(), __pa17.clone()),
        _ => bail!("pattern mismatch"),
    } };
    systs = __pa16.clone();
    shared = __pa17.clone();
    sysIdx = 1;
    for mut sys in &*systs.clone() {
        let mut sys = sys.clone();
        let (__pa18, __pa19, __pa20, __pa21) = ::match_deref::match_deref! { match &(sys.clone()) {
            Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { ass2: __pa18, comps: __pa19, .. }, orderedEqs: __pa20, orderedVars: __pa21, .. } => (__pa18.clone(), __pa19.clone(), __pa20.clone(), __pa21.clone()),
            _ => bail!("pattern mismatch"),
        } };
        ass2 = __pa18.clone();
        comps = __pa19.clone();
        eqs = __pa20.clone();
        vars = __pa21.clone();
        (m, mT) = BackendDAEUtil::adjacencyMatrix(sys.clone(), crate::BackendDAE::IndexType::NORMAL, Some(BackendDAEUtil::getFunctions(shared.clone())?), BackendDAEUtil::isInitializationDAE(shared.clone()))?;
        order = 1;
        for mut comp in &*comps.clone() {
            let mut comp = comp.clone();
            (varLst, varIdxs, eqLst, eqIdxs) = BackendDAEUtil::getStrongComponentsVarsAndEquations(list![comp.clone()], vars.clone(), eqs.clone())?;
            for mut varIdx in &*varIdxs.clone() {
                let mut varIdx = varIdx.clone();
                nodeColor = (if (isAlgLoop(comp.clone())) {arcstr::literal!(GraphML::COLOR_RED2)} else {arcstr::literal!(GraphML::COLOR_GREEN2)}).clone();
                borderWidth = if (BackendVariable::isStateVar(BackendVariable::getVarAt(vars.clone(), varIdx.clone())?)) {GraphML::BORDERWIDTH_BOLD.clone()} else {GraphML::BORDERWIDTH_STANDARD.clone()};
                if isTearingVar(varIdx.clone(), comp.clone()) {
                    shapeType = openmodelica_susan::GraphML::ShapeType::ELLIPSE;
                    tearInfo = (literal!("TearingVar")).clone();
                    nodeColor = (arcstr::literal!(GraphML::COLOR_RED)).clone();
                } else {
                    shapeType = openmodelica_susan::GraphML::ShapeType::ELLIPSE;
                    tearInfo = (literal!("AlgebraicVar")).clone();
                }
                let (__pa23, (_, _)) = GraphML::addNode(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("V_")); __mm_s.push_str(&*intString(sysIdx.clone())); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(varIdx.clone())); ArcStr::from(__mm_s) }).clone(), (nodeColor.clone()).clone(), borderWidth.clone(), list![GraphML::NodeLabel::NODELABEL_INTERNAL { text: (intString(varIdx.clone())).clone(), backgroundColor: None, fontStyle: openmodelica_susan::GraphML::FontStyle::FONTPLAIN }], shapeType.clone(), Some((varString(BackendVariable::getVarAt(vars.clone(), varIdx.clone())?)?).clone()), list![(nameAttIdx.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("V_")); __mm_s.push_str(&*intString(sysIdx.clone())); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(varIdx.clone())); ArcStr::from(__mm_s) }), (varAttIdx.clone(), intString(varIdx.clone())), (eqAttIdx.clone(), literal!("-")), (compAttIdx.clone(), printComponent(comp.clone(), None)?), (sysAttIdx.clone(), intString(sysIdx.clone())), (tearAttIdx.clone(), tearInfo.clone()), (orderAttIdx.clone(), intString(order.clone()))], graphIdx.clone(), graphInfo.clone())?;
                graphInfo = __pa23.clone();
            }
            for mut eqIdx in &*eqIdxs.clone() {
                let mut eqIdx = eqIdx.clone();
                nodeColor = (if (isAlgLoop(comp.clone())) {arcstr::literal!(GraphML::COLOR_RED2)} else {arcstr::literal!(GraphML::COLOR_GREEN2)}).clone();
                if isResidualEq(eqIdx.clone(), comp.clone()) {
                    shapeType = openmodelica_susan::GraphML::ShapeType::RECTANGLE;
                    tearInfo = (literal!("ResidualEq")).clone();
                    nodeColor = (arcstr::literal!(GraphML::COLOR_RED)).clone();
                } else {
                    shapeType = openmodelica_susan::GraphML::ShapeType::RECTANGLE;
                    tearInfo = (literal!("AlgebraicEq")).clone();
                }
                let (__pa24, (_, _)) = GraphML::addNode(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("E_")); __mm_s.push_str(&*intString(sysIdx.clone())); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(eqIdx.clone())); ArcStr::from(__mm_s) }).clone(), (nodeColor.clone()).clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![GraphML::NodeLabel::NODELABEL_INTERNAL { text: (intString(eqIdx.clone())).clone(), backgroundColor: None, fontStyle: openmodelica_susan::GraphML::FontStyle::FONTPLAIN }], shapeType.clone(), Some((equationString(BackendEquation::get(eqs.clone(), eqIdx.clone()))?).clone()), list![(nameAttIdx.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("E_")); __mm_s.push_str(&*intString(sysIdx.clone())); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(eqIdx.clone())); ArcStr::from(__mm_s) }), (varAttIdx.clone(), literal!("-")), (compAttIdx.clone(), printComponent(comp.clone(), None)?), (eqAttIdx.clone(), intString(eqIdx.clone())), (sysAttIdx.clone(), intString(sysIdx.clone())), (tearAttIdx.clone(), tearInfo.clone()), (orderAttIdx.clone(), intString(order.clone()))], graphIdx.clone(), graphInfo.clone())?;
                graphInfo = __pa24.clone();
            }
            order = order.clone() + 1;
        }
        let __range25 = 1..=(m.clone().borrow().len() as i32);
        for mut eqIdx in __range25 {
            let __range26 = &*m.clone().borrow()[(eqIdx.clone()-1) as usize].clone();
            for mut varIdx in __range26 {
                let mut varIdx = varIdx.clone();
                if intLe(varIdx.clone(), 0) {
                    lineType = openmodelica_susan::GraphML::LineType::DASHED;
                } else {
                    lineType = openmodelica_susan::GraphML::LineType::LINE;
                }
                varIdx = intAbs(varIdx.clone());
                lineWidth = if (intEq(varIdx.clone(), ass2.borrow()[(eqIdx.clone()-1) as usize].clone())) {GraphML::LINEWIDTH_BOLD.clone()} else {GraphML::LINEWIDTH_STANDARD.clone()};
                let (__pa27, (_, _)) = GraphML::addEdge(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Edge_")); __mm_s.push_str(&*intString(sysIdx.clone())); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(eqIdx.clone())); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(varIdx.clone())); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("V_")); __mm_s.push_str(&*intString(sysIdx.clone())); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(varIdx.clone())); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("E_")); __mm_s.push_str(&*intString(sysIdx.clone())); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(eqIdx.clone())); ArcStr::from(__mm_s) }).clone(), (arcstr::literal!(GraphML::COLOR_BLACK)).clone(), lineType.clone(), lineWidth.clone(), false, metamodelica::nil(), (openmodelica_susan::GraphML::ArrowType::ARROWNONE, openmodelica_susan::GraphML::ArrowType::ARROWNONE), metamodelica::nil(), graphInfo.clone())?;
                graphInfo = __pa27.clone();
            }
        }
        sysIdx = sysIdx.clone() + 1;
    }
    GraphML::dumpGraph(graphInfo.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*filename.clone()); __mm_s.push_str(&*literal!(".graphml")); ArcStr::from(__mm_s) }).clone())?;
    Ok(())
}

fn isTearingVar(mut varIdx: i32, mut comp: Arc<BackendDAE::StrongComponent>) -> bool {
    let mut isTear: bool = false;
    isTear = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: tVars, .. }, .. } => {
            List::exist1(tVars.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), varIdx.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isTear
}

fn isAlgLoop(mut comp: Arc<BackendDAE::StrongComponent>) -> bool {
    let mut isLoop: bool = false;
    isLoop = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: _, .. } => true,
        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: _, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isLoop
}

fn isResidualEq(mut eqIdx: i32, mut comp: Arc<BackendDAE::StrongComponent>) -> bool {
    let mut isRes: bool = false;
    isRes = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { residualequations: resEqs, .. }, .. } => {
            List::exist1(resEqs.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), eqIdx.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isRes
}

pub fn SSSHandlerArgString(mut arg: Option<(BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)>) -> Result<()> {
    let mut stateorder: BackendDAE::StateOrder = BackendDAE::StateOrder::NOSTATEORDER;
    let mut constraints: metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>;
    let mut eqs2EqIdxs: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut eqIdx2Eq: metamodelica::Array<i32>;
    let mut numEqs: i32 = 0;
    if isSome(arg.clone()) {
        let Some((__pa0, __pa1, __pa2, __pa3, __pa4)) = (arg.clone()) else { bail!("pattern mismatch") };
        stateorder = __pa0.clone();
        constraints = __pa1.clone();
        eqs2EqIdxs = __pa2.clone();
        eqIdx2Eq = __pa3.clone();
        numEqs = __pa4.clone();
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(numEqs.clone())); __mm_s.push_str(&*literal!("eqs before IR\n")); ArcStr::from(__mm_s) }).clone());
        dumpStateOrder(stateorder.clone())?;
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Constraints:\n")); __mm_s.push_str(&*constraintEquationString(constraints.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    } else {
        println!("{}", (literal!("Empty StructurallySingularSystemHandlerArg\n")).clone());
    }
    Ok(())
}

pub fn constraintEquationString(mut constraints: metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>) -> Result<ArcStr> {
    let mut s: ArcStr = literal!("");
    let mut i: i32 = 0;
    let mut s1: ArcStr = arcstr::literal!("");
    let __range0 = 1..=(constraints.clone().borrow().len() as i32);
    for mut i in __range0 {
        s1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(List::map(constraints.clone().borrow()[(i.clone()-1) as usize].clone(), (std::sync::Arc::new(equationString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<ArcStr> + 'static>)), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n------------------\n")); ArcStr::from(__mm_s) }).clone();
        if constraints.clone().borrow()[(i.clone()-1) as usize].clone().is_empty() {
            s1 = (literal!("empty Constraints\n")).clone();
        }
        s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("eq ")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
    }
    Ok(s)
}

pub fn dumpStateOrder(mut inStateOrder: BackendDAE::StateOrder) -> Result<()> {
    let _ = (match inStateOrder.clone() {
        BackendDAE::StateOrder::STATEORDER { hashTable: mut ht, invHashTable: _ } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut len_str: ArcStr = arcstr::literal!("");
            let mut len: i32 = 0;
            let mut tplLst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>> = metamodelica::nil();
            tplLst = BaseHashTable::hashTableList(ht.clone());
            if !(tplLst.clone().is_empty()) {
                println!("{}", (literal!("State Order: (")).clone());
                r#str = stringDelimitList(List::map(tplLst.clone(), (std::sync::Arc::new(printStateOrderStr) as std::sync::Arc<dyn ::std::ops::Fn((Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)) -> Result<ArcStr> + 'static>)), (literal!("\n")).clone());
                len = (tplLst.clone().len() as i32);
                len_str = (intString(len.clone())).clone();
                println!("{}", (len_str.clone()).clone());
                println!("{}", (literal!(")\n")).clone());
                println!("{}", (literal!("=============\n")).clone());
                println!("{}", (r#str.clone()).clone());
                println!("{}", (literal!("\n\n")).clone());
            }
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn printStateOrderStr(mut tpl: (Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(Util::tuple21(tpl.clone()))?); __mm_s.push_str(&*literal!(" ---d/dt---> ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(Util::tuple22(tpl.clone()))?); ArcStr::from(__mm_s) }).clone();
    Ok(r#str)
}

pub fn dumpBackendDAEModeData(mut inDAEmodeData: BackendDAE::BackendDAEModeData) -> Result<()> {
    let mut modelVars: BackendDAE::Variables;
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\nDAEMode\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    if isSome(inDAEmodeData.modelVars.clone()) {
        let Some(__pa0) = (inDAEmodeData.modelVars.clone()) else { bail!("pattern mismatch") };
        modelVars = __pa0.clone();
        dumpVariables(modelVars.clone(), (literal!("ModelVariables")).clone())?;
    } else {
        println!("{}", (literal!("No ModelVariables\n")).clone());
    }
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("DAEmode System:\n ")); __mm_s.push_str(&*intString(inDAEmodeData.numResVars.clone())); __mm_s.push_str(&*literal!(" residual variables\n ")); __mm_s.push_str(&*intString((inDAEmodeData.stateVars.clone().len() as i32))); __mm_s.push_str(&*literal!(" state variables\n ")); __mm_s.push_str(&*intString((inDAEmodeData.algStateVars.clone().len() as i32))); __mm_s.push_str(&*literal!(" algebraic state variables\n")); ArcStr::from(__mm_s) }).clone());
    dumpVarList(inDAEmodeData.stateVars.clone(), (literal!("State Variables")).clone());
    dumpVarList(inDAEmodeData.algStateVars.clone(), (literal!("Algebraic State Variables")).clone());
    Ok(())
}

