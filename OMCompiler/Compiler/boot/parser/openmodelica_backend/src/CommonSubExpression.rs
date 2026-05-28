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

use crate::AvlSetInt;
use crate::BackendDAE;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVarTransform;
use crate::BackendVariable;
use crate::ExpressionSolve;
use crate::HpcOmTaskGraph;
use crate::ResolveLoops;
use openmodelica_ast::Absyn;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::DAEUtil;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionDump;
use openmodelica_frontend::ExpressionSimplify;
use openmodelica_frontend::HashSet;
use openmodelica_frontend::HashTableExpToExp;
use openmodelica_frontend::HashTableExpToIndex;
use openmodelica_frontend::Types;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashSet;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::GCExt;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CSE_Equation {
    /// lhs
    pub cse: Arc<DAE::Exp>,
    /// rhs
    pub call: Arc<DAE::Exp>,
    pub dependencies: Arc<metamodelica::List<i32>>,
}

impl Default for CSE_Equation {
    fn default() -> Self {
        Self {
            cse: Default::default(),
            call: Default::default(),
            dependencies: Default::default(),
        }
    }
}

pub type CSE_EQUATION = CSE_Equation;


thread_local! { static __dummy_equation_TLS: CSE_Equation = CSE_Equation { cse: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), call: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), dependencies: metamodelica::nil() }; }
pub fn dummy_equation() -> CSE_Equation { __dummy_equation_TLS.with(|__t| __t.clone()) }

pub const debug: bool = false;

pub const BORDER: &'static str = "###############################################################";

pub const UNDERLINE: &'static str = "========================================";

fn printCSEEquation(mut cseEquation: CSE_Equation) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut first: bool = true;
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ExpressionBasics::printExpStr(cseEquation.cse.clone())?); __mm_s.push_str(&*literal!(" - ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(cseEquation.call.clone())?); __mm_s.push_str(&*literal!(" - {")); ArcStr::from(__mm_s) }).clone();
    for mut i in &*cseEquation.dependencies.clone() {
        let mut i = i.clone();
        if first.clone() {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone();
            first = false;
        } else {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone();
        }
    }
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
    Ok(r#str)
}

pub fn wrapFunctionCalls(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut size: i32 = 0;
    let mut HT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
    let mut exarray: Arc<ExpandableArray::ExpandableArray<CSE_Equation>>;
    let mut cseIndex: i32 = System::tmpTickIndex(Global::backendDAE_cseIndex.clone());
    let mut index: i32 = 0;
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut functionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    let mut orderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut orderedEqs_new: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut orderedVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut globalKnownVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut eqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut daeTypeStr: ArcStr = BackendDump::printBackendDAEType2String(inDAE.shared.backendDAEType.clone())?;
    let mut isSimulationDAE: bool = stringEq((daeTypeStr.clone()).clone(), (literal!("simulation")).clone());
    let mut globalKnownVarHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    size = BackendDAEUtil::maxSizeOfEqSystems(inDAE.eqs.clone()) + 42;
    exarray = ExpandableArray::new(size.clone(), dummy_equation().clone());
    size = Util::nextPrime(((metamodelica::OrderedFloat(2.4_f64) * metamodelica::OrderedFloat((size.clone()) as f64)).0 as i32));
    HT = HashTableExpToIndex::emptyHashTableSized(size.clone());
    shared = inDAE.shared.clone();
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(shared.clone()) {
        Deref @ BackendDAE::Shared { functionTree: __pa0, globalKnownVars: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    functionTree = __pa0.clone();
    globalKnownVars = __pa1.clone();
    globalKnownVarHT = HashSet::emptyHashSetSized(Util::nextPrime(((metamodelica::OrderedFloat(2.4_f64) * (metamodelica::OrderedFloat((globalKnownVars.numberOfVars.clone() + 42) as f64))).0 as i32)));
    if isSimulationDAE.clone() {
        globalKnownVarHT = BackendVariable::traverseBackendDAEVars(globalKnownVars.clone(), (std::sync::Arc::new(VarToGlobalKnownVarHT) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(BackendDAE::Var, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)))> + 'static>), globalKnownVarHT.clone())?;
    }
    if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Start optimization module wrapFunctionCalls for ")); __mm_s.push_str(&*daeTypeStr.clone()); __mm_s.push_str(&*literal!(" DAE\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Phase 0: Set up data structure\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        BackendDump::dumpVariables(globalKnownVars.clone(), (literal!("globalKnownVars before WFC")).clone())?;
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("globalKnownVarHT before algorithm\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        BaseHashSet::dumpHashSet(globalKnownVarHT.clone())?;
    }
    for mut syst in &*inDAE.eqs.clone() {
        let mut syst = syst.clone();
        if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n\nHandle system (belongs to ")); __mm_s.push_str(&*daeTypeStr.clone()); __mm_s.push_str(&*literal!(" DAE):\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            BackendDump::dumpVariables(syst.orderedVars.clone(), (literal!("Variables")).clone())?;
            BackendDump::dumpEquationArray(syst.orderedEqs.clone(), (literal!("Equations")).clone())?;
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nPhase 1: Analysis\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        HT = BaseHashTable::clear(HT.clone())?;
        exarray = ExpandableArray::clear(exarray.clone());
        index = 0;
        orderedEqs = syst.orderedEqs.clone();
        orderedVars = syst.orderedVars.clone();
        (HT, exarray, cseIndex, index, _) = BackendEquation::traverseEquationArray(orderedEqs.clone(), (std::sync::Arc::new(wrapFunctionCalls_analysis) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, i32, i32, Arc<AvlTreePathFunction::Tree>)) -> Result<(Arc<BackendDAE::Equation>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, i32, i32, Arc<AvlTreePathFunction::Tree>))> + 'static>), (HT.clone(), exarray.clone(), cseIndex.clone(), index.clone(), functionTree.clone()))?;
        if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Hastable after analysis\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            BaseHashTable::dumpHashTable(HT.clone());
            println!("{}", (ExpandableArray::toString(exarray.clone(), (literal!("\nExpandable Array after analysis")).clone(), (std::sync::Arc::new(printCSEEquation) as std::sync::Arc<dyn ::std::ops::Fn(CSE_Equation) -> Result<ArcStr> + 'static>), true)?).clone());
        }
        if index.clone() > 0 {
            exarray = determineDependencies(exarray.clone(), HT.clone())?;
            if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n\nPhase 2: Dependencies\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Hashtable after dependencies\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                BaseHashTable::dumpHashTable(HT.clone());
                println!("{}", (ExpandableArray::toString(exarray.clone(), (literal!("\nExpandable Array after dependencies")).clone(), (std::sync::Arc::new(printCSEEquation) as std::sync::Arc<dyn ::std::ops::Fn(CSE_Equation) -> Result<ArcStr> + 'static>), true)?).clone());
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n\nPhase3: Substitution\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            orderedEqs_new = BackendEquation::emptyEqnsSized(ExpandableArray::getNumberOfElements(orderedEqs.clone()) + ExpandableArray::getNumberOfElements(exarray.clone()));
            (HT, exarray, orderedEqs_new) = BackendEquation::traverseEquationArray(orderedEqs.clone(), (std::sync::Arc::new(wrapFunctionCalls_substitution) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)) -> Result<(Arc<BackendDAE::Equation>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>))> + 'static>), (HT.clone(), exarray.clone(), orderedEqs_new.clone()))?;
            if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Hashtable after substitution\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                BaseHashTable::dumpHashTable(HT.clone());
                println!("{}", (ExpandableArray::toString(exarray.clone(), (literal!("\nExpandable Array after substitution")).clone(), (std::sync::Arc::new(printCSEEquation) as std::sync::Arc<dyn ::std::ops::Fn(CSE_Equation) -> Result<ArcStr> + 'static>), true)?).clone());
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n\nPhase 4: Create CSE-Equations\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
            }
            (orderedEqs_new, orderedVars, globalKnownVars, _) = createCseEquations(exarray.clone(), orderedEqs_new.clone(), orderedVars.clone(), globalKnownVars.clone(), globalKnownVarHT.clone())?;
            assign_field!(
                syst.orderedEqs = orderedEqs_new.clone(),
                syst.orderedVars = orderedVars.clone()
            );
            if !(intEq(BackendEquation::equationArraySize(orderedEqs_new.clone())?, orderedVars.numberOfVars.clone())) {
                Error::addCompilerWarning((literal!("After manipulating the system with postOptModule wrapFunctionCalls the system is unbalanced. This indicates that the original system is singular. You can use -d=dumpCSE and -d=dumpCSE_verbose for more information.")).clone())?;
            }
            assign_field!(
                syst.m = None,
                syst.mT = None,
                syst.matching = Arc::new(crate::BackendDAE::Matching::NO_MATCHING)
            );
            if Flags::isSet(Flags::DUMP_CSE.clone())? || Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n\n\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\nFinal Results\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                BackendDump::dumpVariables(syst.orderedVars.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("########### Updated Variable List (")); __mm_s.push_str(&*BackendDump::printBackendDAEType2String(shared.backendDAEType.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
                BackendDump::dumpEquationArray(syst.orderedEqs.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("########### Updated Equation List (")); __mm_s.push_str(&*BackendDump::printBackendDAEType2String(shared.backendDAEType.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
                BackendDump::dumpVariables(globalKnownVars.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("########### Updated globalKnownVars (")); __mm_s.push_str(&*BackendDump::printBackendDAEType2String(shared.backendDAEType.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
                println!("{}", (ExpandableArray::toString(exarray.clone(), (literal!("\n########### CSE Replacements")).clone(), (std::sync::Arc::new(printCSEEquation) as std::sync::Arc<dyn ::std::ops::Fn(CSE_Equation) -> Result<ArcStr> + 'static>), true)?).clone());
            }
            if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); ArcStr::from(__mm_s) }).clone());
                BackendDump::dumpEqSystem(syst.clone(), (literal!("Final EqSystem")).clone())?;
            }
        } else {
            if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\nNo function calls found. Exiting the algorithm...\n\n\n")); ArcStr::from(__mm_s) }).clone());
            }
        }
        eqSystems = cons(syst.clone(), eqSystems.clone());
    }
    assign_field!(shared.globalKnownVars = globalKnownVars.clone());
    System::tmpTickSetIndex(cseIndex.clone(), Global::backendDAE_cseIndex.clone());
    eqSystems = metamodelica::Dangerous::listReverseInPlace(eqSystems.clone());
    outDAE = BackendDAE::DAE(eqSystems.clone(), shared.clone())?;
    Ok(outDAE)
}

fn VarToGlobalKnownVarHT(mut inVar: BackendDAE::Var, mut inGlobalKnownVarHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<(BackendDAE::Var, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))> {
    let mut outVar: BackendDAE::Var = inVar.clone();
    let mut outGlobalKnownVarHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)) = inGlobalKnownVarHT.clone();
    if !(BackendVariable::isInput(inVar.clone())) && !(BackendVariable::isParam(inVar.clone()) && !(BackendVariable::varFixed(inVar.clone()))) && isSome(inVar.bindExp.clone()) {
        outGlobalKnownVarHT = BaseHashSet::add(BackendVariable::varCref(inVar.clone())?, inGlobalKnownVarHT.clone())?;
    }
    Ok((outVar, outGlobalKnownVarHT))
}

fn findCallsInGlobalKnownVars(mut inVar: BackendDAE::Var, mut inTuple: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, i32, i32, Arc<AvlTreePathFunction::Tree>)) -> Result<(BackendDAE::Var, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, i32, i32, Arc<AvlTreePathFunction::Tree>))> {
    let mut outVar: BackendDAE::Var = inVar.clone();
    let mut outTuple: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, i32, i32, Arc<AvlTreePathFunction::Tree>) = inTuple.clone();
    let mut exp: Arc<DAE::Exp>;
    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    if !(BackendVariable::isInput(inVar.clone())) && !(BackendVariable::isParam(inVar.clone()) && !(BackendVariable::varFixed(inVar.clone()))) && isSome(inVar.bindExp.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(inVar.bindExp.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        exp = __pa0.clone();
        if isCall(exp.clone()) {
            eq = BackendEquation::generateEquation(Arc::new(DAE::Exp::CREF { componentRef: inVar.varName.clone(), ty: inVar.varType.clone() }), exp.clone(), DAE::emptyElementSource().clone(), BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone())?;
            (_, outTuple) = wrapFunctionCalls_analysis(eq.clone(), inTuple.clone())?;
        }
    }
    Ok((outVar, outTuple))
}

fn wrapFunctionCalls_substitution(mut inEq: Arc<BackendDAE::Equation>, mut inTuple: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)) -> Result<(Arc<BackendDAE::Equation>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>))> {
    let mut outEq: Arc<BackendDAE::Equation> = inEq.clone();
    let mut outTuple: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>);
    let mut HT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
    let mut exarray: Arc<ExpandableArray::ExpandableArray<CSE_Equation>>;
    let mut orderedEqs_new: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    (HT, exarray, orderedEqs_new) = inTuple.clone();
    let () = (::match_deref::match_deref! { match &(inEq.clone()) {
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { .. } => {
            if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
                BackendDump::dumpEquationList(list![inEq.clone()], (literal!("wrapFunctionCalls_substitution (COMPLEX_EQUATION)")).clone());
            }
            let (__pa0, (__pa1, __pa2, __pa3)) = BackendEquation::traverseExpsOfEquation(inEq.clone(), (std::sync::Arc::new(wrapFunctionCalls_substitution2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)) -> Result<(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>))> + 'static>), (HT.clone(), exarray.clone(), orderedEqs_new.clone()))?;
            eq = __pa0.clone();
            HT = __pa1.clone();
            exarray = __pa2.clone();
            orderedEqs_new = __pa3.clone();
            if !(isEquationRedundant(eq.clone())?) {
                orderedEqs_new = BackendEquation::add(eq.clone(), orderedEqs_new.clone())?;
                if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
                    BackendDump::dumpEquationList(list![eq.clone()], (literal!("isEquationRedundant? no")).clone());
                }
            } else {
                if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
                    BackendDump::dumpEquationList(list![eq.clone()], (literal!("isEquationRedundant? yes")).clone());
                }
            }
            ()
        },
        Deref @ BackendDAE::Equation::EQUATION { .. } => {
            if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
                BackendDump::dumpEquationList(list![inEq.clone()], (literal!("wrapFunctionCalls_substitution (EQUATION)")).clone());
            }
            let (__pa0, (__pa1, __pa2, __pa3)) = BackendEquation::traverseExpsOfEquation(inEq.clone(), (std::sync::Arc::new(wrapFunctionCalls_substitution2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)) -> Result<(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>))> + 'static>), (HT.clone(), exarray.clone(), orderedEqs_new.clone()))?;
            eq = __pa0.clone();
            HT = __pa1.clone();
            exarray = __pa2.clone();
            orderedEqs_new = __pa3.clone();
            if !(isEquationRedundant(eq.clone())?) {
                orderedEqs_new = BackendEquation::add(eq.clone(), orderedEqs_new.clone())?;
            }
            ()
        },
        _ => {
            orderedEqs_new = BackendEquation::add(inEq.clone(), orderedEqs_new.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outTuple = (HT.clone(), exarray.clone(), orderedEqs_new.clone());
    Ok((outEq, outTuple))
}

fn wrapFunctionCalls_substitution2(mut inExp: Arc<DAE::Exp>, mut inTuple: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)) -> Result<(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTuple: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>);
    (outExp, outTuple) = Expression::traverseExpBottomUp(inExp.clone(), (std::sync::Arc::new(wrapFunctionCalls_substitution3) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)) -> Result<(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>))> + 'static>), inTuple.clone())?;
    Ok((outExp, outTuple))
}

fn wrapFunctionCalls_substitution3(mut inExp: Arc<DAE::Exp>, mut inTuple: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)) -> Result<(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTuple: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>);
    let mut HT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
    let mut exarray: Arc<ExpandableArray::ExpandableArray<CSE_Equation>>;
    let mut orderedEqs_new: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut id: i32 = 0;
    let mut ix: i32 = 0;
    let mut cse: Arc<DAE::Exp>;
    let mut call: Arc<DAE::Exp>;
    let mut tmp: Arc<DAE::Exp>;
    let mut PR: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut dependencies: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (HT, exarray, orderedEqs_new) = inTuple.clone();
    if Expression::isCall(inExp.clone()) && BaseHashTable::hasKey(inExp.clone(), HT.clone()) {
        id = BaseHashTable::get(inExp.clone(), HT.clone())?;
        let CSE_Equation { dependencies: __pa0, call: __pa1, cse: __pa2 } = (ExpandableArray::get(id.clone(), exarray.clone())?) else { bail!("pattern mismatch") };
        dependencies = __pa0.clone();
        call = __pa1.clone();
        cse = __pa2.clone();
        (HT, exarray) = substituteDependencies(dependencies.clone(), HT.clone(), exarray.clone(), call.clone(), cse.clone())?;
        ExpandableArray::update(id.clone(), CSE_Equation { cse: cse.clone(), call: call.clone(), dependencies: metamodelica::nil() }, exarray.clone())?;
        outExp = cse.clone();
    } else if Expression::isTSUB(inExp.clone()) {
        let (__pa3, __pa4) = ::match_deref::match_deref! { match &(inExp.clone()) {
            Deref @ DAE::Exp::TSUB { ix: __pa3, exp: __pa4, .. } => (__pa3.clone(), __pa4.clone()),
            _ => bail!("pattern mismatch"),
        } };
        ix = __pa3.clone();
        tmp = __pa4.clone();
        if Expression::isTuple(tmp.clone()) {
            let __pa5 = ::match_deref::match_deref! { match &(tmp.clone()) {
                Deref @ DAE::Exp::TUPLE { PR: __pa5 } => __pa5.clone(),
                _ => bail!("pattern mismatch"),
            } };
            PR = __pa5.clone();
            outExp = (PR.clone()).get(ix.clone())?;
        } else {
            outExp = inExp.clone();
        }
    } else {
        outExp = inExp.clone();
    }
    outTuple = (HT.clone(), exarray.clone(), orderedEqs_new.clone());
    Ok((outExp, outTuple))
}

fn substituteDependencies(mut inDependencies: Arc<metamodelica::List<i32>>, mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), mut exarray: Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, mut inCall: Arc<DAE::Exp>, mut inCSE: Arc<DAE::Exp>) -> Result<((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>)> {
    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)) = ht;
    let mut exarray: Arc<ExpandableArray::ExpandableArray<CSE_Equation>> = exarray;
    let mut cse: Arc<DAE::Exp>;
    let mut call: Arc<DAE::Exp>;
    let mut dependencies: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut cse2: Arc<DAE::Exp>;
    let mut call2: Arc<DAE::Exp>;
    let mut dependencies2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut id2: i32 = 0;
    for mut id in &*inDependencies.clone() {
        let mut id = id.clone();
        let CSE_Equation { dependencies: __pa0, call: __pa1, cse: __pa2 } = (ExpandableArray::get(id.clone(), exarray.clone())?) else { bail!("pattern mismatch") };
        dependencies = __pa0.clone();
        call = __pa1.clone();
        cse = __pa2.clone();
        call = substituteExp(call.clone(), inCall.clone(), inCSE.clone())?;
        if !(BaseHashTable::hasKey(call.clone(), ht.clone())) {
            ht = BaseHashTable::add((call.clone(), id.clone()), ht.clone())?;
            ExpandableArray::update(id.clone(), CSE_Equation { cse: cse.clone(), call: call.clone(), dependencies: dependencies.clone() }, exarray.clone())?;
        } else {
            id2 = BaseHashTable::get(call.clone(), ht.clone())?;
            let CSE_Equation { dependencies: __pa3, call: __pa4, cse: __pa5 } = (ExpandableArray::get(id2.clone(), exarray.clone())?) else { bail!("pattern mismatch") };
            dependencies2 = __pa3.clone();
            call2 = __pa4.clone();
            cse2 = __pa5.clone();
            cse2 = mergeCSETuples(cse.clone(), cse2.clone())?;
            ExpandableArray::update(id2.clone(), CSE_Equation { cse: cse2.clone(), call: call.clone(), dependencies: UnorderedSet::unique_list(listAppend(dependencies.clone(), dependencies2.clone()), std::sync::Arc::new(fnptr!(Util::id, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>)) }, exarray.clone())?;
            ExpandableArray::update(id.clone(), CSE_Equation { cse: cse.clone(), call: cse2.clone(), dependencies: metamodelica::nil() }, exarray.clone())?;
        }
    }
    Ok((ht, exarray))
}

fn substituteExp(mut inExp: Arc<DAE::Exp>, mut inKey: Arc<DAE::Exp>, mut inValue: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    (outExp, _) = Expression::traverseExpTopDown(inExp.clone(), (std::sync::Arc::new(substituteExp2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<DAE::Exp>, Arc<DAE::Exp>)) -> Result<(Arc<DAE::Exp>, bool, (Arc<DAE::Exp>, Arc<DAE::Exp>))> + 'static>), (inKey.clone(), inValue.clone()))?;
    Ok(outExp)
}

fn substituteExp2(mut inExp: Arc<DAE::Exp>, mut inTuple: (Arc<DAE::Exp>, Arc<DAE::Exp>)) -> Result<(Arc<DAE::Exp>, bool, (Arc<DAE::Exp>, Arc<DAE::Exp>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut outTuple: (Arc<DAE::Exp>, Arc<DAE::Exp>) = inTuple.clone();
    let mut key: Arc<DAE::Exp>;
    let mut value: Arc<DAE::Exp>;
    let mut tmp: Arc<DAE::Exp>;
    let mut expList: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut ix: i32 = 0;
    (key, value) = inTuple.clone();
    if ExpressionBasics::expEqual(inExp.clone(), key.clone())? {
        outExp = value.clone();
        cont = false;
    } else if Expression::isTSUB(inExp.clone()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp.clone()) {
            Deref @ DAE::Exp::TSUB { ix: __pa0, exp: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        ix = __pa0.clone();
        tmp = __pa1.clone();
        if ExpressionBasics::expEqual(tmp.clone(), key.clone())? {
            let __pa2 = ::match_deref::match_deref! { match &(value.clone()) {
                Deref @ DAE::Exp::TUPLE { PR: __pa2 } => __pa2.clone(),
                _ => bail!("pattern mismatch"),
            } };
            expList = __pa2.clone();
            outExp = (expList.clone()).get(ix.clone())?;
            cont = false;
        } else {
            outExp = inExp.clone();
            cont = true;
        }
    } else {
        outExp = inExp.clone();
        cont = true;
    }
    Ok((outExp, cont, outTuple))
}

fn createCseEquations(mut exarray: Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, mut orderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut orderedVars: BackendDAE::Variables, mut globalKnownVars: BackendDAE::Variables, mut globalKnownVarHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, BackendDAE::Variables, BackendDAE::Variables, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))> {
    let mut orderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = orderedEqs;
    let mut orderedVars: BackendDAE::Variables = orderedVars;
    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars;
    let mut globalKnownVarHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)) = globalKnownVarHT;
    let mut cse: Arc<DAE::Exp>;
    let mut call: Arc<DAE::Exp>;
    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut varList: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut delVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut isGlobalKnown: bool = false;
    let mut eqRedundant: bool = false;
    let mut add: bool = false;
    if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("globalKnownVars:\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        BaseHashSet::dumpHashSet(globalKnownVarHT.clone())?;
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nTraverse expandable array\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    for mut i in (1..=ExpandableArray::getNumberOfElements(exarray.clone())).rev() {
        add = true;
        let CSE_Equation { call: __pa0, cse: __pa1, .. } = (ExpandableArray::get(i.clone(), exarray.clone())?) else { bail!("pattern mismatch") };
        call = __pa0.clone();
        cse = __pa1.clone();
        if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n--> cse-equation: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(cse.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(call.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        eq = BackendEquation::generateEquation(cse.clone(), call.clone(), DAE::emptyElementSource().clone(), BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone())?;
        (globalKnownVarHT, globalKnownVars, orderedVars, eqRedundant, isGlobalKnown) = isEquationRedundant_flatten(eq.clone(), globalKnownVarHT.clone(), globalKnownVars.clone(), orderedVars.clone())?;
        if debug.clone() {
            println!("{}", (literal!("\ndebug 1 - eq redundant?\n")).clone());
        }
        if !(eqRedundant.clone()) {
            if debug.clone() {
                println!("{}", (literal!("\ndebug 2 - no, not redundant. let's loop\n")).clone());
            }
            varList = createVarsForExp(cse.clone(), metamodelica::nil())?;
            if varList.clone().is_empty() {
                orderedEqs = BackendEquation::add(eq.clone(), orderedEqs.clone())?;
            } else {
                for mut var in &*varList.clone() {
                    let mut var = var.clone();
                    if debug.clone() {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\ndebug 3 - handle var: ")); __mm_s.push_str(&*BackendDump::varString(var.clone())?); __mm_s.push_str(&*literal!(" Is it a globalKnownVar?\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    cr = BackendVariable::varCref(var.clone())?;
                    if !(isGlobalKnown.clone()) {
                        if debug.clone() {
                            println!("{}", (literal!("\ndebug 4 - The variable is not a globalKnownVar. Should an equation be added?\n")).clone());
                        }
                        if add.clone() {
                            if debug.clone() {
                                println!("{}", (literal!("\ndebug 5 - yes, definitely!\n")).clone());
                            }
                            orderedEqs = BackendEquation::add(eq.clone(), orderedEqs.clone())?;
                            add = false;
                        }
                        if debug.clone() {
                            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\ndebug 6 - Is this cref a CSE cref?: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(Expression::crefExp(cr.clone())?)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                        }
                        if isCSECref(cr.clone()) {
                            if debug.clone() {
                                println!("{}", (literal!("\ndebug 7 - yes it is a CSE cref. Add to orderedVars!\n")).clone());
                            }
                            orderedVars = BackendVariable::addVar(var.clone(), orderedVars.clone())?;
                        }
                        if debug.clone() {
                            println!("{}", (literal!("\ndebug 8\n")).clone());
                        }
                    } else {
                        if debug.clone() {
                            println!("{}", (literal!("\ndebug 9 - The variable is a globalKnownVar.\n")).clone());
                        }
                        if !(isCSECref(cr.clone())) {
                            if debug.clone() {
                                println!("{}", (literal!("\ndebug 10 - The globalKnownVar is no CSE cref, so copy attributes and delete it from orderedVars if it is in that list.\n")).clone());
                            }
                            (delVars, orderedVars) = BackendVariable::deleteVarIfExistsAndReturn(cr.clone(), orderedVars.clone());
                            if delVars.clone().is_empty() {
                                (delVars, _) = BackendVariable::getVar(cr.clone(), globalKnownVars.clone())?;
                            }
                            var = (delVars.clone()).get(1)?;
                        }
                        var = BackendVariable::setBindExp(var.clone(), Some(call.clone()));
                        var = BackendVariable::makeParam(var.clone());
                        var = BackendVariable::setVarFinal(var.clone(), true)?;
                        if intGt((varList.clone().len() as i32), 1) || Expression::isTuple(cse.clone()) {
                            if debug.clone() {
                                println!("{}", (literal!("\ndebug 11 - It is a tuple! Add it to tplExp\n")).clone());
                            }
                            var.tplExp = Some(cse.clone());
                        }
                        if debug.clone() {
                            println!("{}", (literal!("\ndebug 12 - Add the variable to globalKnownVars\n")).clone());
                        }
                        globalKnownVars = BackendVariable::addVar(var.clone(), globalKnownVars.clone())?;
                    }
                }
            }
        }
    }
    if debug.clone() {
        println!("{}", (literal!("\ndebug 13\n")).clone());
    }
    Ok((orderedEqs, orderedVars, globalKnownVars, globalKnownVarHT))
}

fn determineDependencies(mut exarray: Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, mut HT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))) -> Result<Arc<ExpandableArray::ExpandableArray<CSE_Equation>>> {
    let mut exarray: Arc<ExpandableArray::ExpandableArray<CSE_Equation>> = exarray;
    let mut callArguments: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    for mut i in 1..=ExpandableArray::getNumberOfElements(exarray.clone()) {
        let CSE_Equation { call: __t1, .. } = (ExpandableArray::get(i.clone(), exarray.clone())?) else { bail!("pattern mismatch") };
        let __pa0 = ::match_deref::match_deref! { match &(__t1.clone()) {
            Deref @ DAE::Exp::CALL { expLst: __pa0, .. } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        callArguments = __pa0.clone();
        let (_, (_, __pa2, _)) = Expression::traverseExpList(callArguments.clone(), (std::sync::Arc::new(determineDependencies2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, i32)) -> Result<(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, i32))> + 'static>), (HT.clone(), exarray.clone(), i.clone()))?;
        exarray = __pa2.clone();
    }
    Ok(exarray)
}

fn determineDependencies2(mut inExp: Arc<DAE::Exp>, mut inTuple: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, i32)) -> Result<(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, i32))> {
    let mut outExp: Arc<DAE::Exp> = inExp.clone();
    let mut outTuple: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, i32);
    let mut id: i32 = 0;
    let mut index: i32 = 0;
    let mut dependencies: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut HT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
    let mut exarray: Arc<ExpandableArray::ExpandableArray<CSE_Equation>>;
    let mut cse: Arc<DAE::Exp>;
    let mut call: Arc<DAE::Exp>;
    if Expression::isCall(inExp.clone()) {
        (HT, exarray, index) = inTuple.clone();
        if BaseHashTable::hasKey(inExp.clone(), HT.clone()) {
            id = BaseHashTable::get(inExp.clone(), HT.clone())?;
            let CSE_Equation { dependencies: __pa0, call: __pa1, cse: __pa2 } = (ExpandableArray::get(id.clone(), exarray.clone())?) else { bail!("pattern mismatch") };
            dependencies = __pa0.clone();
            call = __pa1.clone();
            cse = __pa2.clone();
            if !(listMember(index.clone(), dependencies.clone())) {
                dependencies = cons(index.clone(), dependencies.clone());
                ExpandableArray::update(id.clone(), CSE_Equation { cse: cse.clone(), call: call.clone(), dependencies: dependencies.clone() }, exarray.clone())?;
            }
        }
        outTuple = (HT.clone(), exarray.clone(), index.clone());
    } else {
        outTuple = inTuple.clone();
    }
    Ok((outExp, outTuple))
}

fn allArgsInGlobalKnownVars(mut callArgs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut globalKnownVarHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<bool> {
    let mut allCrefsAreGlobal: bool = true;
    let mut crefList: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    (_, crefList) = Expression::traverseExpList(callArgs.clone(), (std::sync::Arc::new(fnptr!(Expression::traversingComponentRefFinder, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), metamodelica::nil())?;
    for mut cr in &*crefList.clone() {
        let mut cr = cr.clone();
        if allCrefsAreGlobal.clone() {
            allCrefsAreGlobal = BaseHashSet::has(cr.clone(), globalKnownVarHT.clone())?;
        } else {
            return Ok(allCrefsAreGlobal.clone());
        }
    }
    Ok(allCrefsAreGlobal)
}

fn addConstantCseVarsToGlobalKnownVarHT(mut cse_crExp: Arc<DAE::Exp>, mut globalKnownVarHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))> {
    let mut globalKnownVarHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)) = globalKnownVarHT;
    let () = (::match_deref::match_deref! { match &(cse_crExp.clone()) {
        Deref @ DAE::Exp::TUPLE { PR: expLst } => {
            for mut exp in &*expLst.clone() {
                let mut exp = exp.clone();
                if Expression::isNotWild(exp.clone()) {
                    globalKnownVarHT = addConstantCseVarsToGlobalKnownVarHT(exp.clone(), globalKnownVarHT.clone())?;
                }
            }
            ()
        },
        Deref @ DAE::Exp::CALL { expLst, .. } => {
            for mut exp in &*expLst.clone() {
                let mut exp = exp.clone();
                if Expression::isNotWild(exp.clone()) {
                    globalKnownVarHT = addConstantCseVarsToGlobalKnownVarHT(exp.clone(), globalKnownVarHT.clone())?;
                }
            }
            ()
        },
        Deref @ DAE::Exp::RECORD { exps: expLst, .. } => {
            for mut exp in &*expLst.clone() {
                let mut exp = exp.clone();
                if Expression::isNotWild(exp.clone()) {
                    globalKnownVarHT = addConstantCseVarsToGlobalKnownVarHT(exp.clone(), globalKnownVarHT.clone())?;
                }
            }
            ()
        },
        Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, componentRef: cr } => {
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            globalKnownVarHT = BaseHashSet::add(cr.clone(), globalKnownVarHT.clone())?;
            crefs = ComponentReference::expandCref(cr.clone(), true)?;
            for mut cr_ in &*crefs.clone() {
                let mut cr_ = cr_.clone();
                globalKnownVarHT = BaseHashSet::add(cr_.clone(), globalKnownVarHT.clone())?;
            }
            ()
        },
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } if (Expression::isArrayType(Expression::r#typeof(cse_crExp.clone())?)) => {
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            globalKnownVarHT = BaseHashSet::add(cr.clone(), globalKnownVarHT.clone())?;
            crefs = ComponentReference::expandCref(cr.clone(), true)?;
            for mut cr_ in &*crefs.clone() {
                let mut cr_ = cr_.clone();
                globalKnownVarHT = BaseHashSet::add(cr_.clone(), globalKnownVarHT.clone())?;
            }
            ()
        },
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
            globalKnownVarHT = BaseHashSet::add(cr.clone(), globalKnownVarHT.clone())?;
            ()
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("addConstantCseVarsToGlobalKnownVarHT failed. Reached else case that should not be reachable while handling CSE expression:\n")); __mm_s.push_str(&*ExpressionDump::dumpExpStr(cse_crExp.clone(), 0)?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail");
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(globalKnownVarHT)
}

fn wrapFunctionCalls_analysis(mut inEq: Arc<BackendDAE::Equation>, mut inTuple: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, i32, i32, Arc<AvlTreePathFunction::Tree>)) -> Result<(Arc<BackendDAE::Equation>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, i32, i32, Arc<AvlTreePathFunction::Tree>))> {
    let mut outEq: Arc<BackendDAE::Equation> = inEq.clone();
    let mut outTuple: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, i32, i32, Arc<AvlTreePathFunction::Tree>);
    let mut functionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    let mut HT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
    let mut exarray: Arc<ExpandableArray::ExpandableArray<CSE_Equation>>;
    let mut cseIndex: i32 = 0;
    let mut exIndex: i32 = 0;
    let mut index: i32 = 0;
    let mut ix: i32 = 0;
    let mut lhs: Arc<DAE::Exp>;
    let mut rhs: Arc<DAE::Exp>;
    let mut cref: Arc<DAE::Exp>;
    let mut call: Arc<DAE::Exp>;
    let mut exp: Arc<DAE::Exp>;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut types: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    let mut cseEquation: CSE_Equation = <CSE_Equation as ::std::default::Default>::default();
    (HT, exarray, cseIndex, index, functionTree) = inTuple.clone();
    let () = (::match_deref::match_deref! { match &(inEq.clone()) {
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: rhs, left: lhs, .. } => {
            if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
                BackendDump::dumpEquationList(list![inEq.clone()], (literal!("wrapFunctionCalls_analysis (COMPLEX_EQUATION)")).clone());
            }
            if isCallAndTuple(lhs.clone(), rhs.clone()) {
                (cref, call) = getTheRightPattern(lhs.clone(), rhs.clone())?;
                if BaseHashTable::hasKey(call.clone(), HT.clone()) {
                    exIndex = BaseHashTable::get(call.clone(), HT.clone())?;
                    cseEquation = ExpandableArray::get(exIndex.clone(), exarray.clone())?;
                    cseEquation.cse = mergeCSETuples(cseEquation.cse.clone(), cref.clone())?;
                    exarray = ExpandableArray::update(exIndex.clone(), cseEquation.clone(), exarray.clone())?;
                } else if !(isSkipCase(call.clone(), functionTree.clone())?) {
                    index = index.clone() + 1;
                    HT = BaseHashTable::add((call.clone(), index.clone()), HT.clone())?;
                    exarray = ExpandableArray::set(index.clone(), CSE_Equation { cse: cref.clone(), call: call.clone(), dependencies: metamodelica::nil() }, exarray.clone())?;
                }
            } else if isCallAndRecord(lhs.clone(), rhs.clone()) {
                (cref, call) = getTheRightPattern(lhs.clone(), rhs.clone())?;
                if BaseHashTable::hasKey(call.clone(), HT.clone()) {
                    exIndex = BaseHashTable::get(call.clone(), HT.clone())?;
                    cseEquation = ExpandableArray::get(exIndex.clone(), exarray.clone())?;
                    cseEquation.cse = cref.clone();
                    exarray = ExpandableArray::update(exIndex.clone(), cseEquation.clone(), exarray.clone())?;
                } else if !(isSkipCase(call.clone(), functionTree.clone())?) {
                    index = index.clone() + 1;
                    HT = BaseHashTable::add((call.clone(), index.clone()), HT.clone())?;
                    exarray = ExpandableArray::set(index.clone(), CSE_Equation { cse: cref.clone(), call: call.clone(), dependencies: metamodelica::nil() }, exarray.clone())?;
                }
            }
            let (_, (__pa0, __pa1, __pa2, __pa3, __pa4)) = BackendEquation::traverseExpsOfEquation(inEq.clone(), (std::sync::Arc::new(wrapFunctionCalls_analysis2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, i32, i32, Arc<AvlTreePathFunction::Tree>)) -> Result<(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, i32, i32, Arc<AvlTreePathFunction::Tree>))> + 'static>), (HT.clone(), exarray.clone(), cseIndex.clone(), index.clone(), functionTree.clone()))?;
            HT = __pa0.clone();
            exarray = __pa1.clone();
            cseIndex = __pa2.clone();
            index = __pa3.clone();
            functionTree = __pa4.clone();
            ()
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: rhs, exp: lhs, .. } => {
            if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
                BackendDump::dumpEquationList(list![inEq.clone()], (literal!("wrapFunctionCalls_analysis (EQUATION)")).clone());
            }
            if isCallAndCref(lhs.clone(), rhs.clone()) || isConstAndCall(lhs.clone(), rhs.clone()) {
                (cref, call) = getTheRightPattern(lhs.clone(), rhs.clone())?;
                if BaseHashTable::hasKey(call.clone(), HT.clone()) {
                    exIndex = BaseHashTable::get(call.clone(), HT.clone())?;
                    cseEquation = ExpandableArray::get(exIndex.clone(), exarray.clone())?;
                    cseEquation.cse = cref.clone();
                    exarray = ExpandableArray::update(exIndex.clone(), cseEquation.clone(), exarray.clone())?;
                } else if !(isSkipCase(call.clone(), functionTree.clone())?) {
                    index = index.clone() + 1;
                    HT = BaseHashTable::add((call.clone(), index.clone()), HT.clone())?;
                    exarray = ExpandableArray::set(index.clone(), CSE_Equation { cse: cref.clone(), call: call.clone(), dependencies: metamodelica::nil() }, exarray.clone())?;
                }
            } else if isTsubAndCref(lhs.clone(), rhs.clone()) {
                let (__pa0, __pa2, __pa1, __pa3) = ::match_deref::match_deref! { match &(getTheRightPattern(lhs.clone(), rhs.clone())?) {
                    (__pa0, Deref @ DAE::Exp::TSUB { exp: __pa2 @ Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_TUPLE { types: __pa1, .. }, .. }, .. }, ix: __pa3, ty: _ }) => (__pa0.clone(), __pa2.clone(), __pa1.clone(), __pa3.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                cref = __pa0.clone();
                types = __pa1.clone();
                call = __pa2.clone();
                ix = __pa3.clone();
                if BaseHashTable::hasKey(call.clone(), HT.clone()) {
                    exIndex = BaseHashTable::get(call.clone(), HT.clone())?;
                    cseEquation = ExpandableArray::get(exIndex.clone(), exarray.clone())?;
                    cref = createCrefForTsub((types.clone().len() as i32), ix.clone(), cref.clone());
                    cseEquation.cse = mergeCSETuples(cseEquation.cse.clone(), cref.clone())?;
                    exarray = ExpandableArray::update(exIndex.clone(), cseEquation.clone(), exarray.clone())?;
                } else if !(isSkipCase(call.clone(), functionTree.clone())?) {
                    index = index.clone() + 1;
                    HT = BaseHashTable::add((call.clone(), index.clone()), HT.clone())?;
                    cref = createCrefForTsub((types.clone().len() as i32), ix.clone(), cref.clone());
                    exarray = ExpandableArray::set(index.clone(), CSE_Equation { cse: cref.clone(), call: call.clone(), dependencies: metamodelica::nil() }, exarray.clone())?;
                }
            }
            let (_, (__pa5, __pa6, __pa7, __pa8, __pa9)) = BackendEquation::traverseExpsOfEquation(inEq.clone(), (std::sync::Arc::new(wrapFunctionCalls_analysis2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, i32, i32, Arc<AvlTreePathFunction::Tree>)) -> Result<(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, i32, i32, Arc<AvlTreePathFunction::Tree>))> + 'static>), (HT.clone(), exarray.clone(), cseIndex.clone(), index.clone(), functionTree.clone()))?;
            HT = __pa5.clone();
            exarray = __pa6.clone();
            cseIndex = __pa7.clone();
            index = __pa8.clone();
            functionTree = __pa9.clone();
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outTuple = (HT.clone(), exarray.clone(), cseIndex.clone(), index.clone(), functionTree.clone());
    Ok((outEq, outTuple))
}

fn createCrefForTsub(mut length: i32, mut ix: i32, mut cref: Arc<DAE::Exp>) -> Arc<DAE::Exp> {
    let mut outCref: Arc<DAE::Exp>;
    let mut expList: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    for mut i in 1..=ix.clone() - 1 {
        expList = cons(Arc::new(DAE::Exp::CREF { componentRef: Arc::new(openmodelica_frontend_types::DAE::ComponentRef::WILD), ty: DAE::T_UNKNOWN_DEFAULT().clone() }), expList.clone());
    }
    expList = cons(cref.clone(), expList.clone());
    for mut i in ix.clone() + 1..=length.clone() {
        expList = cons(Arc::new(DAE::Exp::CREF { componentRef: Arc::new(openmodelica_frontend_types::DAE::ComponentRef::WILD), ty: DAE::T_UNKNOWN_DEFAULT().clone() }), expList.clone());
    }
    outCref = Arc::new(DAE::Exp::TUPLE { PR: expList.clone().reverse() });
    outCref
}

fn wrapFunctionCalls_analysis2(mut inExp: Arc<DAE::Exp>, mut inTuple: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, i32, i32, Arc<AvlTreePathFunction::Tree>)) -> Result<(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, i32, i32, Arc<AvlTreePathFunction::Tree>))> {
    let mut outExp: Arc<DAE::Exp> = inExp.clone();
    let mut outTuple: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, i32, i32, Arc<AvlTreePathFunction::Tree>);
    (_, outTuple) = Expression::traverseExpTopDown(inExp.clone(), (std::sync::Arc::new(wrapFunctionCalls_analysis3) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, i32, i32, Arc<AvlTreePathFunction::Tree>)) -> Result<(Arc<DAE::Exp>, bool, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, i32, i32, Arc<AvlTreePathFunction::Tree>))> + 'static>), inTuple.clone())?;
    Ok((outExp, outTuple))
}

fn wrapFunctionCalls_analysis3(mut inExp: Arc<DAE::Exp>, mut inTuple: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, i32, i32, Arc<AvlTreePathFunction::Tree>)) -> Result<(Arc<DAE::Exp>, bool, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, i32, i32, Arc<AvlTreePathFunction::Tree>))> {
    let mut outExp: Arc<DAE::Exp> = inExp.clone();
    let mut cont: bool = false;
    let mut outTuple: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, i32, i32, Arc<AvlTreePathFunction::Tree>);
    let mut functionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    let mut HT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
    let mut exarray: Arc<ExpandableArray::ExpandableArray<CSE_Equation>>;
    let mut cseIndex: i32 = 0;
    let mut index: i32 = 0;
    let mut tsub: Arc<DAE::Exp>;
    (HT, exarray, cseIndex, index, functionTree) = inTuple.clone();
    cont = ({
        let mut expList: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::IFEXP { .. } => {
            (_, outTuple) = Expression::traverseExpTopDown(var_field!((*inExp).expCond, DAE::Exp::IFEXP).clone(), (std::sync::Arc::new(wrapFunctionCalls_analysis3) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, i32, i32, Arc<AvlTreePathFunction::Tree>)) -> Result<(Arc<DAE::Exp>, bool, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, i32, i32, Arc<AvlTreePathFunction::Tree>))> + 'static>), inTuple.clone())?;
            cont = false;
            return Ok((outExp.clone(), cont.clone(), outTuple.clone()));
            bail!("fail")
        },
        _ if (isSkipCase(inExp.clone(), functionTree.clone())?) => {
            false
        },
        tsub @ Deref @ DAE::Exp::TSUB { ty, ix, exp: call @ Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_TUPLE { types, .. }, .. }, .. } } => {
            let mut cse_var: Arc<DAE::Exp>;
            let mut cse_var2: Arc<DAE::Exp>;
            let mut e: Arc<DAE::Exp>;
            let mut id: i32 = 0;
            let mut cseEquation: CSE_Equation = <CSE_Equation as ::std::default::Default>::default();
            if !(BaseHashTable::hasKey(call.clone(), HT.clone())) {
                index = index.clone() + 1;
                HT = BaseHashTable::add((call.clone(), index.clone()), HT.clone())?;
                (cse_var, cseIndex) = createReturnExp(ty.clone(), cseIndex.clone(), (literal!("$cse")).clone(), false)?;
                cse_var2 = createCrefForTsub((types.clone().len() as i32), ix.clone(), cse_var.clone());
                exarray = ExpandableArray::set(index.clone(), CSE_Equation { cse: cse_var2.clone(), call: call.clone(), dependencies: metamodelica::nil() }, exarray.clone())?;
            } else {
                id = BaseHashTable::get(call.clone(), HT.clone())?;
                cseEquation = ExpandableArray::get(id.clone(), exarray.clone())?;
                if Expression::isTuple(cseEquation.cse.clone()) {
                    let __pa0 = ::match_deref::match_deref! { match &(cseEquation.cse.clone()) {
                        Deref @ DAE::Exp::TUPLE { PR: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    expList = __pa0.clone();
                    e = (expList.clone()).get(ix.clone())?;
                    if isWildCref(e.clone()) {
                        (cse_var, cseIndex) = createReturnExp(ty.clone(), cseIndex.clone(), (literal!("$cse")).clone(), false)?;
                        expList = List::set(expList.clone(), ix.clone(), cse_var.clone())?;
                        cseEquation.cse = Arc::new(DAE::Exp::TUPLE { PR: expList.clone() });
                        exarray = ExpandableArray::update(id.clone(), cseEquation.clone(), exarray.clone())?;
                    }
                } else {
                    Error::addMessage(Error::GENERIC_ELAB_EXPRESSION.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*ExpressionDump::dumpExpStr(inExp.clone(), 0)?); __mm_s.push_str(&*literal!(" This should never happen, Error in wrapFunctionCalls_analysis3. Trying to recover.")); ArcStr::from(__mm_s) }).clone()])?;
                }
            }
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "noEvent" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::RELATION { exp1: e, operator: _, exp2: e2, index: _, optionExpisASUB: _ }, tail: Deref @ metamodelica::List::Nil }, .. } => {
            (_, outTuple) = Expression::traverseExpTopDown(e.clone(), (std::sync::Arc::new(wrapFunctionCalls_analysis3) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, i32, i32, Arc<AvlTreePathFunction::Tree>)) -> Result<(Arc<DAE::Exp>, bool, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, i32, i32, Arc<AvlTreePathFunction::Tree>))> + 'static>), inTuple.clone())?;
            (_, outTuple) = Expression::traverseExpTopDown(e2.clone(), (std::sync::Arc::new(wrapFunctionCalls_analysis3) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, i32, i32, Arc<AvlTreePathFunction::Tree>)) -> Result<(Arc<DAE::Exp>, bool, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<ExpandableArray::ExpandableArray<CSE_Equation>>, i32, i32, Arc<AvlTreePathFunction::Tree>))> + 'static>), outTuple.clone())?;
            cont = false;
            return Ok((outExp.clone(), cont.clone(), outTuple.clone()));
            true
        },
        Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty, .. }, .. } => {
            let mut cse_var: Arc<DAE::Exp>;
            if !(BaseHashTable::hasKey(inExp.clone(), HT.clone())) {
                index = index.clone() + 1;
                HT = BaseHashTable::add((inExp.clone(), index.clone()), HT.clone())?;
                (cse_var, cseIndex) = createReturnExp(ty.clone(), cseIndex.clone(), (literal!("$cse")).clone(), false)?;
                exarray = ExpandableArray::set(index.clone(), CSE_Equation { cse: cse_var.clone(), call: inExp.clone(), dependencies: metamodelica::nil() }, exarray.clone())?;
            }
            true
        },
        _ => {
            true
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
    outTuple = (HT.clone(), exarray.clone(), cseIndex.clone(), index.clone(), functionTree.clone());
    Ok((outExp, cont, outTuple))
}

fn getTheRightPattern(mut inExp1: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>) -> Result<(Arc<DAE::Exp>, Arc<DAE::Exp>)> {
    let mut outExp1: Arc<DAE::Exp>;
    let mut outExp2: Arc<DAE::Exp>;
    (outExp1, outExp2) = (::match_deref::match_deref! { match &((inExp1.clone(), inExp2.clone())) {
        (Deref @ DAE::Exp::RCONST { .. }, Deref @ DAE::Exp::CALL { .. }) => (inExp1.clone(), inExp2.clone()),
        (Deref @ DAE::Exp::CALL { .. }, Deref @ DAE::Exp::RCONST { .. }) => (inExp2.clone(), inExp1.clone()),
        (Deref @ DAE::Exp::TUPLE { .. }, Deref @ DAE::Exp::CALL { .. }) => (inExp1.clone(), inExp2.clone()),
        (Deref @ DAE::Exp::CALL { .. }, Deref @ DAE::Exp::TUPLE { .. }) => (inExp2.clone(), inExp1.clone()),
        (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::CALL { .. }) => (inExp1.clone(), inExp2.clone()),
        (Deref @ DAE::Exp::CALL { .. }, Deref @ DAE::Exp::CREF { .. }) => (inExp2.clone(), inExp1.clone()),
        (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::TSUB { .. }) => (inExp1.clone(), inExp2.clone()),
        (Deref @ DAE::Exp::TSUB { .. }, Deref @ DAE::Exp::CREF { .. }) => (inExp2.clone(), inExp1.clone()),
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp1, outExp2))
}

fn isEquationRedundant(mut inEq: Arc<BackendDAE::Equation>) -> Result<bool> {
    let mut outB: bool = false;
    outB = (::match_deref::match_deref! { match &(inEq.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { scalar: exp2, exp: exp1, .. } => {
            ExpressionBasics::expEqual(exp1.clone(), exp2.clone())?
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::TUPLE { PR: rhs }, exp: Deref @ DAE::Exp::TUPLE { PR: lhs }, .. } if ((lhs.clone().len() as i32) == (rhs.clone().len() as i32)) => {
            println!("{}", (literal!("This should never appear\n")).clone());
            isEquationRedundant2(lhs.clone(), rhs.clone())?
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: Deref @ DAE::Exp::TUPLE { PR: rhs }, left: Deref @ DAE::Exp::TUPLE { PR: lhs }, .. } if ((lhs.clone().len() as i32) == (rhs.clone().len() as i32)) => {
            isEquationRedundant2(lhs.clone(), rhs.clone())?
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: exp2 @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, .. }, left: exp1 @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, .. }, .. } => {
            ExpressionBasics::expEqual(exp1.clone(), exp2.clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outB)
}

fn isEquationRedundant2(mut lhs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut rhs: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<bool> {
    let mut result: bool = true;
    let mut l: Arc<DAE::Exp>;
    let mut r: Arc<DAE::Exp>;
    let mut ll: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut rr: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    if lhs.clone().is_empty() {
        return Ok(result.clone());
    }
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lhs.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    l = __pa0.clone();
    ll = __pa1.clone();
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rhs.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    r = __pa2.clone();
    rr = __pa3.clone();
    if !(isWildCref(l.clone())) && !(isWildCref(r.clone())) {
        if !(ExpressionBasics::expEqual(l.clone(), r.clone())?) {
            result = false;
            return Ok(result.clone());
        }
    }
    result = isEquationRedundant2(ll.clone(), rr.clone())?;
    Ok(result)
}

fn isEquationRedundant_flatten(mut inEq: Arc<BackendDAE::Equation>, mut globalKnownVarHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), mut globalKnownVars: BackendDAE::Variables, mut orderedVars: BackendDAE::Variables) -> Result<((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), BackendDAE::Variables, BackendDAE::Variables, bool, bool)> {
    let mut globalKnownVarHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)) = globalKnownVarHT;
    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars;
    let mut orderedVars: BackendDAE::Variables = orderedVars;
    let mut outB: bool = false;
    let mut isGlobalKnown: bool = false;
    outB = (::match_deref::match_deref! { match &(inEq.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { scalar: exp2, exp: exp1, .. } => {
            let mut isRedundant: bool = false;
            isRedundant = ExpressionBasics::expEqual(exp1.clone(), exp2.clone())?;
            if !(isRedundant.clone()) {
                isGlobalKnown = allArgsInGlobalKnownVars(list![exp2.clone()], globalKnownVarHT.clone())?;
                if isGlobalKnown.clone() {
                    globalKnownVarHT = addConstantCseVarsToGlobalKnownVarHT(exp1.clone(), globalKnownVarHT.clone())?;
                }
            }
            isRedundant.clone()
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: Deref @ DAE::Exp::TUPLE { PR: rhs }, left: Deref @ DAE::Exp::TUPLE { PR: lhs }, .. } if ((lhs.clone().len() as i32) == (rhs.clone().len() as i32)) => {
            let mut isRedundant: bool = false;
            (globalKnownVarHT, globalKnownVars, orderedVars, isRedundant) = isEquationRedundant_flatten2(lhs.clone(), rhs.clone(), globalKnownVarHT.clone(), globalKnownVars.clone(), orderedVars.clone())?;
            isRedundant.clone()
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { size: _, left: exp1 @ Deref @ DAE::Exp::TUPLE { PR: lhs }, right: exp2, source: _, attr: _ } => {
            let mut varList: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut isRedundant: bool = false;
            isRedundant = ExpressionBasics::expEqual(exp1.clone(), exp2.clone())?;
            if !(isRedundant.clone()) {
                isGlobalKnown = allArgsInGlobalKnownVars(list![exp2.clone()], globalKnownVarHT.clone())?;
                if isGlobalKnown.clone() {
                    for mut expMem in &*lhs.clone() {
                        let mut expMem = expMem.clone();
                        varList = createVarsForExp(expMem.clone(), metamodelica::nil())?;
                        for mut var in &*varList.clone() {
                            let mut var = var.clone();
                            var = BackendVariable::setBindExp(var.clone(), Some(exp2.clone()));
                            globalKnownVars = BackendVariable::addVar(var.clone(), globalKnownVars.clone())?;
                            globalKnownVarHT = addConstantCseVarsToGlobalKnownVarHT(expMem.clone(), globalKnownVarHT.clone())?;
                        }
                    }
                }
            }
            isRedundant.clone()
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: exp2, left: exp1, .. } => {
            let mut isRedundant: bool = false;
            isRedundant = ExpressionBasics::expEqual(exp1.clone(), exp2.clone())?;
            if !(isRedundant.clone()) {
                isGlobalKnown = allArgsInGlobalKnownVars(list![exp2.clone()], globalKnownVarHT.clone())?;
                if isGlobalKnown.clone() {
                    globalKnownVarHT = addConstantCseVarsToGlobalKnownVarHT(exp1.clone(), globalKnownVarHT.clone())?;
                }
            }
            isRedundant.clone()
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((globalKnownVarHT, globalKnownVars, orderedVars, outB, isGlobalKnown))
}

fn isEquationRedundant_flatten2(mut lhs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut rhs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut globalKnownVarHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), mut globalKnownVars: BackendDAE::Variables, mut orderedVars: BackendDAE::Variables) -> Result<((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), BackendDAE::Variables, BackendDAE::Variables, bool)> {
    let mut globalKnownVarHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)) = globalKnownVarHT;
    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars;
    let mut orderedVars: BackendDAE::Variables = orderedVars;
    let mut result: bool = true;
    let mut l: Arc<DAE::Exp>;
    let mut r: Arc<DAE::Exp>;
    let mut ll: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut rr: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    if lhs.clone().is_empty() {
        return Ok((globalKnownVarHT.clone(), globalKnownVars.clone(), orderedVars.clone(), result.clone()));
    }
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lhs.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    l = __pa0.clone();
    ll = __pa1.clone();
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rhs.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    r = __pa2.clone();
    rr = __pa3.clone();
    if !(isWildCref(l.clone())) && !(isWildCref(r.clone())) {
        if !(ExpressionBasics::expEqual(l.clone(), r.clone())?) {
            if BaseHashSet::has(Expression::expCref(r.clone())?, globalKnownVarHT.clone())? {
                let __pa4 = ::match_deref::match_deref! { match &(createVarsForExp(l.clone(), metamodelica::nil())?) {
                    Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Nil } => __pa4.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                var = __pa4.clone();
                var = BackendVariable::setBindExp(var.clone(), Some(r.clone()));
                globalKnownVars = BackendVariable::addVar(var.clone(), globalKnownVars.clone())?;
                globalKnownVarHT = addConstantCseVarsToGlobalKnownVarHT(l.clone(), globalKnownVarHT.clone())?;
                if !(isCSECref(var.varName.clone())) {
                    (_, orderedVars) = BackendVariable::deleteVarIfExistsAndReturn(var.varName.clone(), orderedVars.clone());
                }
            } else {
                result = false;
                return Ok((globalKnownVarHT.clone(), globalKnownVars.clone(), orderedVars.clone(), result.clone()));
            }
        }
    }
    (globalKnownVarHT, globalKnownVars, orderedVars, result) = isEquationRedundant_flatten2(ll.clone(), rr.clone(), globalKnownVarHT.clone(), globalKnownVars.clone(), orderedVars.clone())?;
    Ok((globalKnownVarHT, globalKnownVars, orderedVars, result))
}

fn isCall(mut inExp: Arc<DAE::Exp>) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

fn isCallAndCref(mut inExp: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &((inExp.clone(), inExp2.clone())) {
        (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::CALL { .. }) => true,
        (Deref @ DAE::Exp::CALL { .. }, Deref @ DAE::Exp::CREF { .. }) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

fn isTsubAndCref(mut inExp: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &((inExp.clone(), inExp2.clone())) {
        (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::TSUB { .. }) => true,
        (Deref @ DAE::Exp::TSUB { .. }, Deref @ DAE::Exp::CREF { .. }) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

fn isConstAndCall(mut inExp: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &((inExp.clone(), inExp2.clone())) {
        (Deref @ DAE::Exp::RCONST { .. }, Deref @ DAE::Exp::CALL { .. }) => true,
        (Deref @ DAE::Exp::CALL { .. }, Deref @ DAE::Exp::RCONST { .. }) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

fn isCallAndTuple(mut inExp: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &((inExp.clone(), inExp2.clone())) {
        (Deref @ DAE::Exp::TUPLE { .. }, Deref @ DAE::Exp::CALL { .. }) => true,
        (Deref @ DAE::Exp::CALL { .. }, Deref @ DAE::Exp::TUPLE { .. }) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

fn isCallAndRecord(mut inExp: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &((inExp.clone(), inExp2.clone())) {
        (Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, .. }, Deref @ DAE::Exp::CALL { .. }) => true,
        (Deref @ DAE::Exp::CALL { .. }, Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, .. }) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

fn mergeCSETuples(mut inCref1: Arc<DAE::Exp>, mut inCref2: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outCref: Arc<DAE::Exp>;
    let mut expLst1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut expLst2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut expLst3: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut e: Arc<DAE::Exp>;
    if Expression::isTuple(inCref1.clone()) && Expression::isTuple(inCref2.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(inCref1.clone()) {
            Deref @ DAE::Exp::TUPLE { PR: __pa0 } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        expLst1 = __pa0.clone();
        let __pa1 = ::match_deref::match_deref! { match &(inCref2.clone()) {
            Deref @ DAE::Exp::TUPLE { PR: __pa1 } => __pa1.clone(),
            _ => bail!("pattern mismatch"),
        } };
        expLst2 = __pa1.clone();
        expLst1 = mergeCSETuples2(expLst1.clone(), expLst2.clone())?;
        outCref = Arc::new(DAE::Exp::TUPLE { PR: expLst1.clone() });
    } else if !(Expression::isTuple(inCref1.clone())) && Expression::isTuple(inCref2.clone()) {
        println!("{}", (literal!("mergeCSETuples: This should never appear! (1)\n")).clone());
        let __pa2 = ::match_deref::match_deref! { match &(inCref2.clone()) {
            Deref @ DAE::Exp::TUPLE { PR: __pa2 } => __pa2.clone(),
            _ => bail!("pattern mismatch"),
        } };
        expLst2 = __pa2.clone();
        let (__pa3, __pa4) = ::match_deref::match_deref! { match &(expLst2.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa3.clone();
        expLst3 = __pa4.clone();
        if isWildCref(e.clone()) {
            expLst2 = cons(inCref1.clone(), expLst3.clone());
        }
        outCref = Arc::new(DAE::Exp::TUPLE { PR: expLst2.clone() });
    } else if Expression::isTuple(inCref1.clone()) && !(Expression::isTuple(inCref2.clone())) {
        println!("{}", (literal!("mergeCSETuples: This should never appear! (2)\n")).clone());
        let __pa5 = ::match_deref::match_deref! { match &(inCref1.clone()) {
            Deref @ DAE::Exp::TUPLE { PR: __pa5 } => __pa5.clone(),
            _ => bail!("pattern mismatch"),
        } };
        expLst1 = __pa5.clone();
        let (__pa6, __pa7) = ::match_deref::match_deref! { match &(expLst1.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa6, tail: __pa7 } => (__pa6.clone(), __pa7.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa6.clone();
        expLst3 = __pa7.clone();
        if isWildCref(e.clone()) {
            expLst1 = cons(inCref2.clone(), expLst3.clone());
        }
        outCref = Arc::new(DAE::Exp::TUPLE { PR: expLst1.clone() });
    } else {
        outCref = inCref1.clone();
    }
    Ok(outCref)
}

fn mergeCSETuples2(mut inExpLst1: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inExpLst2: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    outExpLst = (::match_deref::match_deref! { match &((inExpLst1.clone(), inExpLst2.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            outExpLst.clone()
        },
        (Deref @ metamodelica::List::Cons { head: e1, tail: expLst1 }, Deref @ metamodelica::List::Cons { head: e2, tail: expLst2 }) => {
            outExpLst = mergeCSETuples2(expLst1.clone(), expLst2.clone())?;
            if !(isWildCref(e1.clone())) && !(isWildCref(e2.clone())) {
                if isCSEExp(e1.clone()) && !(isCSEExp(e2.clone())) {
                    outExpLst = cons(e2.clone(), outExpLst.clone());
                } else {
                    outExpLst = cons(e1.clone(), outExpLst.clone());
                }
            } else if isWildCref(e1.clone()) && !(isWildCref(e2.clone())) {
                outExpLst = cons(e2.clone(), outExpLst.clone());
            } else if !(isWildCref(e1.clone())) && isWildCref(e2.clone()) {
                outExpLst = cons(e1.clone(), outExpLst.clone());
            } else if isWildCref(e1.clone()) && isWildCref(e2.clone()) {
                outExpLst = cons(e1.clone(), outExpLst.clone());
            }
            outExpLst.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExpLst)
}

fn isWildCref(mut inExp: Arc<DAE::Exp>) -> bool {
    let mut outB: bool = false;
    outB = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outB
}

fn isSkipCase(mut inCall: Arc<DAE::Exp>, mut functionTree: Arc<AvlTreePathFunction::Tree>) -> Result<bool> {
    let mut outB: bool = false;
    outB = (::match_deref::match_deref! { match &(inCall.clone()) {
        Deref @ DAE::Exp::ASUB { .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "$_round" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "$getPart" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "abs" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "actualStream" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "backSample" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cardinality" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "ceil" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "change" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "Clock" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "delay" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "div" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "edge" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "firstTick" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "floor" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "getInstanceName" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "hold" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "homotopy" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initial" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "inStream" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "integer" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "Integer" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "interval" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "mod" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "noClock" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "reinit" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "rem" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "shiftSample" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sign" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "smooth" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "spatialDistribution" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "String" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "subSample" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sum" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "superSample" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "terminal" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { .. } if (Expression::isImpureCall(inCall.clone())? || isCallRecordConstructor(inCall.clone(), functionTree.clone())?) => {
            true
        },
        Deref @ DAE::Exp::CALL { .. } if (Flags::getConfigBool(Flags::WFC_ADVANCED.clone())?) => {
            isSkipCase_advanced(inCall.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outB)
}

fn isSkipCase_advanced(mut inCall: Arc<DAE::Exp>) -> bool {
    let mut outB: bool = false;
    outB = (::match_deref::match_deref! { match &(inCall.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "acos" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "asin" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "atan" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "atan2" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cos" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cosh" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "log" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "log10" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sin" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sinh" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "tan" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "tanh" }, .. } => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outB
}

fn isCallRecordConstructor(mut inExp: Arc<DAE::Exp>, mut funcsIn: Arc<AvlTreePathFunction::Tree>) -> Result<bool> {
    let mut outIsCall: bool = false;
    outIsCall = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path, .. } => {
                    let mut func: DAE::Function;
                    let __pa0 = ::match_deref::match_deref! { match &(AvlTreePathFunction::get(funcsIn.clone(), path.clone())?) {
                        Some(__pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    func = __pa0.clone();
                    Ok(DAEUtil::getFunctionElements(func.clone())?.is_empty())
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
    Ok(outIsCall)
}

fn createReturnExp(mut inType: Arc<DAE::Type>, mut inIndex: i32, mut inPrefix: ArcStr, mut inComplex: bool) -> Result<(Arc<DAE::Exp>, i32)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outIndex: i32 = 0;
    (outExp, outIndex) = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_REAL { .. } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut value: Arc<DAE::Exp>;
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inPrefix.clone()); __mm_s.push_str(&*intString(inIndex.clone())); ArcStr::from(__mm_s) }).clone();
            cr = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (r#str.clone()).clone(), identType: DAE::T_REAL_DEFAULT().clone(), subscriptLst: metamodelica::nil() });
            value = Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: DAE::T_REAL_DEFAULT().clone() });
            (value.clone(), inIndex.clone() + 1)
        },
        Deref @ DAE::Type::T_INTEGER { .. } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut value: Arc<DAE::Exp>;
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inPrefix.clone()); __mm_s.push_str(&*intString(inIndex.clone())); ArcStr::from(__mm_s) }).clone();
            cr = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (r#str.clone()).clone(), identType: DAE::T_INTEGER_DEFAULT().clone(), subscriptLst: metamodelica::nil() });
            value = Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: DAE::T_INTEGER_DEFAULT().clone() });
            (value.clone(), inIndex.clone() + 1)
        },
        Deref @ DAE::Type::T_STRING { .. } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut value: Arc<DAE::Exp>;
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inPrefix.clone()); __mm_s.push_str(&*intString(inIndex.clone())); ArcStr::from(__mm_s) }).clone();
            cr = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (r#str.clone()).clone(), identType: DAE::T_STRING_DEFAULT().clone(), subscriptLst: metamodelica::nil() });
            value = Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: DAE::T_STRING_DEFAULT().clone() });
            (value.clone(), inIndex.clone() + 1)
        },
        Deref @ DAE::Type::T_BOOL { .. } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut value: Arc<DAE::Exp>;
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inPrefix.clone()); __mm_s.push_str(&*intString(inIndex.clone())); ArcStr::from(__mm_s) }).clone();
            cr = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (r#str.clone()).clone(), identType: DAE::T_BOOL_DEFAULT().clone(), subscriptLst: metamodelica::nil() });
            value = Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: DAE::T_BOOL_DEFAULT().clone() });
            (value.clone(), inIndex.clone() + 1)
        },
        Deref @ DAE::Type::T_ENUMERATION { .. } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut value: Arc<DAE::Exp>;
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inPrefix.clone()); __mm_s.push_str(&*intString(inIndex.clone())); ArcStr::from(__mm_s) }).clone();
            cr = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (r#str.clone()).clone(), identType: inType.clone(), subscriptLst: metamodelica::nil() });
            value = Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: inType.clone() });
            (value.clone(), inIndex.clone() + 1)
        },
        Deref @ DAE::Type::T_CLOCK { .. } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut value: Arc<DAE::Exp>;
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inPrefix.clone()); __mm_s.push_str(&*intString(inIndex.clone())); ArcStr::from(__mm_s) }).clone();
            cr = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (r#str.clone()).clone(), identType: DAE::T_CLOCK_DEFAULT().clone(), subscriptLst: metamodelica::nil() });
            value = Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: DAE::T_CLOCK_DEFAULT().clone() });
            (value.clone(), inIndex.clone() + 1)
        },
        Deref @ DAE::Type::T_TUPLE { types: typeLst, .. } => {
            let mut i: i32 = 0;
            let mut value: Arc<DAE::Exp>;
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            if inComplex.clone() {
                (expLst, i) = List::mapFold(typeLst.clone(), Arc::new({ let __pe_b2 = (inPrefix.clone()).clone(); let __pe_b3 = inComplex.clone(); move |__pe_a0, __pe_a1| createReturnExp(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone()) }), inIndex.clone());
                value = Arc::new(DAE::Exp::TUPLE { PR: expLst.clone() });
            } else {
                let __pa0 = ::match_deref::match_deref! { match &(typeLst.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                ty = __pa0.clone();
                (value, i) = createReturnExp(ty.clone(), inIndex.clone(), (inPrefix.clone()).clone(), false)?;
            }
            (value.clone(), i.clone())
        },
        Deref @ DAE::Type::T_ARRAY { .. } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut value: Arc<DAE::Exp>;
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inPrefix.clone()); __mm_s.push_str(&*intString(inIndex.clone())); ArcStr::from(__mm_s) }).clone();
            cr = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (r#str.clone()).clone(), identType: inType.clone(), subscriptLst: metamodelica::nil() });
            value = Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: inType.clone() });
            (value.clone(), inIndex.clone() + 1)
        },
        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut value: Arc<DAE::Exp>;
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inPrefix.clone()); __mm_s.push_str(&*intString(inIndex.clone())); ArcStr::from(__mm_s) }).clone();
            cr = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (r#str.clone()).clone(), identType: inType.clone(), subscriptLst: metamodelica::nil() });
            value = Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: inType.clone() });
            (value.clone(), inIndex.clone() + 1)
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  - createReturnExp failed for ")); __mm_s.push_str(&*TypesDump::printTypeStr(inType.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outIndex))
}

fn createVarsForExp_onlyCSECrefs(mut inExp: Arc<DAE::Exp>, mut inAccumVarLst: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut outVarLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    outVarLst = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD, .. } => {
            inAccumVarLst.clone()
        },
        Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, componentRef: cr } if (isCSECref(cr.clone())) => {
            let mut cr_: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut arrayDim: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            crefs = ComponentReference::expandCref(cr.clone(), true)?;
            outVarLst = inAccumVarLst.clone();
            for mut cr_ in &*crefs.clone() {
                let mut cr_ = cr_.clone();
                arrayDim = ComponentReferenceBasics::crefDims(cr_.clone())?;
                outVarLst = cons(BackendVariable::createCSEArrayVar(cr_.clone(), ComponentReference::crefTypeFull(cr_.clone())?, arrayDim.clone())?, outVarLst.clone());
            }
            outVarLst.clone()
        },
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } if (isCSECref(cr.clone()) && Expression::isArrayType(Expression::r#typeof(inExp.clone())?)) => {
            let mut cr_: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut arrayDim: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            crefs = ComponentReference::expandCref(cr.clone(), true)?;
            outVarLst = inAccumVarLst.clone();
            ty = DAEUtil::expTypeElementType(Expression::r#typeof(inExp.clone())?);
            for mut cr_ in &*crefs.clone() {
                let mut cr_ = cr_.clone();
                arrayDim = ComponentReferenceBasics::crefDims(cr_.clone())?;
                outVarLst = cons(BackendVariable::createCSEArrayVar(cr_.clone(), ty.clone(), arrayDim.clone())?, outVarLst.clone());
            }
            outVarLst.clone()
        },
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } if (isCSECref(cr.clone())) => {
            let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
            var = BackendVariable::createCSEVar(cr.clone(), Expression::r#typeof(inExp.clone())?)?;
            cons(var.clone(), inAccumVarLst.clone())
        },
        Deref @ DAE::Exp::TUPLE { PR: expLst } => {
            outVarLst = List::fold(expLst.clone(), (std::sync::Arc::new(createVarsForExp_onlyCSECrefs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<BackendDAE::Var>>) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> + 'static>), inAccumVarLst.clone());
            outVarLst.clone()
        },
        Deref @ DAE::Exp::ARRAY { array: expLst, .. } => {
            outVarLst = List::fold(expLst.clone(), (std::sync::Arc::new(createVarsForExp_onlyCSECrefs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<BackendDAE::Var>>) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> + 'static>), inAccumVarLst.clone());
            outVarLst.clone()
        },
        Deref @ DAE::Exp::RECORD { exps: expLst, .. } => {
            println!("{}", (literal!("This should never appear\n")).clone());
            outVarLst = List::fold(expLst.clone(), (std::sync::Arc::new(createVarsForExp_onlyCSECrefs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<BackendDAE::Var>>) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> + 'static>), inAccumVarLst.clone());
            outVarLst.clone()
        },
        _ => {
            inAccumVarLst.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outVarLst)
}

fn createVarsForExp(mut inExp: Arc<DAE::Exp>, mut inAccumVarLst: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut outVarLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    outVarLst = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD, .. } => {
            inAccumVarLst.clone()
        },
        Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, componentRef: cr } => {
            let mut cr_: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut arrayDim: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            crefs = ComponentReference::expandCref(cr.clone(), true)?;
            outVarLst = inAccumVarLst.clone();
            for mut cr_ in &*crefs.clone() {
                let mut cr_ = cr_.clone();
                arrayDim = ComponentReferenceBasics::crefDims(cr_.clone())?;
                outVarLst = cons(BackendVariable::createCSEArrayVar(cr_.clone(), ComponentReference::crefTypeFull(cr_.clone())?, arrayDim.clone())?, outVarLst.clone());
            }
            outVarLst.clone()
        },
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } if (Expression::isArrayType(Expression::r#typeof(inExp.clone())?)) => {
            let mut cr_: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut arrayDim: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
            crefs = ComponentReference::expandCref(cr.clone(), true)?;
            outVarLst = inAccumVarLst.clone();
            ty = DAEUtil::expTypeElementType(Expression::r#typeof(inExp.clone())?);
            for mut cr_ in &*crefs.clone() {
                let mut cr_ = cr_.clone();
                arrayDim = ComponentReferenceBasics::crefDims(cr_.clone())?;
                outVarLst = cons(BackendVariable::createCSEArrayVar(cr_.clone(), ty.clone(), arrayDim.clone())?, outVarLst.clone());
            }
            outVarLst.clone()
        },
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
            let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
            var = BackendVariable::createCSEVar(cr.clone(), Expression::r#typeof(inExp.clone())?)?;
            cons(var.clone(), inAccumVarLst.clone())
        },
        Deref @ DAE::Exp::TUPLE { PR: expLst } => {
            outVarLst = List::fold(expLst.clone(), (std::sync::Arc::new(createVarsForExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<BackendDAE::Var>>) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> + 'static>), inAccumVarLst.clone());
            outVarLst.clone()
        },
        Deref @ DAE::Exp::ARRAY { array: expLst, .. } => {
            outVarLst = List::fold(expLst.clone(), (std::sync::Arc::new(createVarsForExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<BackendDAE::Var>>) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> + 'static>), inAccumVarLst.clone());
            outVarLst.clone()
        },
        Deref @ DAE::Exp::RECORD { exps: expLst, .. } => {
            outVarLst = List::fold(expLst.clone(), (std::sync::Arc::new(createVarsForExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<BackendDAE::Var>>) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> + 'static>), inAccumVarLst.clone());
            outVarLst.clone()
        },
        Deref @ DAE::Exp::CALL { expLst, .. } => {
            outVarLst = List::fold(expLst.clone(), (std::sync::Arc::new(createVarsForExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<BackendDAE::Var>>) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> + 'static>), inAccumVarLst.clone());
            outVarLst.clone()
        },
        _ => {
            inAccumVarLst.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outVarLst)
}

pub fn isCSECref(mut cr: Arc<DAE::ComponentRef>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(cr.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: s, .. } => {
            StringUtil::startsWith((s.clone()).clone(), (literal!("$cse")).clone())
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: s, .. } => {
            StringUtil::startsWith((s.clone()).clone(), (literal!("$cse")).clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isCSEExp(mut inExp: Arc<DAE::Exp>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CREF { .. } => isCSECref(var_field!((*inExp).componentRef, DAE::Exp::CREF).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn cseBinary(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    (outDAE, _) = BackendDAEUtil::mapEqSystemAndFold(inDAE.clone(), (std::sync::Arc::new(CSE1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32)> + 'static>), 1)?;
    Ok(outDAE)
}

fn CSE1(mut inSystem: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut inIndex: i32) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32)> {
    let mut outSystem: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut outIndex: i32 = 0;
    (outSystem, outIndex) = ({
        let mut index: i32 = inIndex.clone();
        'mc: {
        let __mc_input = inSystem.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                syst @ Deref @ BackendDAE::EqSystem { orderedEqs, orderedVars, .. } => {
                    let mut varList: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut eqList: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut HT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (HashTableExpToExp::FuncHashCref, HashTableExpToExp::FuncCrefEqual, HashTableExpToExp::FuncCrefStr, HashTableExpToExp::FuncExpStr));
                    let mut HT2: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
                    let mut HT3: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
                    let mut syst = (*syst).clone();
                    let mut orderedEqs = (*orderedEqs).clone();
                    HT = HashTableExpToExp::emptyHashTableSized(49999);
                    HT2 = HashTableExpToIndex::emptyHashTableSized(49999);
                    HT3 = HashTableExpToIndex::emptyHashTableSized(49999);
                    if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
                        println!("{}", (literal!("collect statistics\n========================================\n")).clone());
                    }
                    (HT, HT2, index) = BackendEquation::traverseEquationArray(orderedEqs.clone(), (std::sync::Arc::new(createStatistics) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (HashTableExpToExp::FuncHashCref, HashTableExpToExp::FuncCrefEqual, HashTableExpToExp::FuncCrefStr, HashTableExpToExp::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), i32)) -> Result<(Arc<BackendDAE::Equation>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (HashTableExpToExp::FuncHashCref, HashTableExpToExp::FuncCrefEqual, HashTableExpToExp::FuncCrefStr, HashTableExpToExp::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), i32))> + 'static>), (HT.clone(), HT2.clone(), index.clone()))?;
                    if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
                        println!("{}", (literal!("\nstart substitution\n========================================\n")).clone());
                    }
                    let (__pa0, (__pa1, __pa2, _, __pa3, __pa4)) = BackendEquation::traverseEquationArray_WithUpdate(orderedEqs.clone(), (std::sync::Arc::new(substituteCSE) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (HashTableExpToExp::FuncHashCref, HashTableExpToExp::FuncCrefEqual, HashTableExpToExp::FuncCrefStr, HashTableExpToExp::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)) -> Result<(Arc<BackendDAE::Equation>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (HashTableExpToExp::FuncHashCref, HashTableExpToExp::FuncCrefEqual, HashTableExpToExp::FuncCrefStr, HashTableExpToExp::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>))> + 'static>), (HT.clone(), HT2.clone(), HT3.clone(), metamodelica::nil(), metamodelica::nil()))?;
                    orderedEqs = __pa0.clone();
                    HT = __pa1.clone();
                    HT2 = __pa2.clone();
                    eqList = __pa3.clone();
                    varList = __pa4.clone();
                    if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
                        println!("{}", (literal!("\n")).clone());
                    }
                    assign_field!(
                        syst.orderedEqs = BackendEquation::addList(eqList.clone(), orderedEqs.clone())?,
                        syst.orderedVars = BackendVariable::addVars(varList.clone(), orderedVars.clone())
                    );
                    if Flags::isSet(Flags::DUMP_CSE.clone())? {
                        BackendDump::dumpVariables(syst.orderedVars.clone(), (literal!("########### Updated Variable List ###########")).clone())?;
                        BackendDump::dumpEquationArray(syst.orderedEqs.clone(), (literal!("########### Updated Equation List ###########")).clone())?;
                    }
                    Ok((BackendDAEUtil::clearEqSyst(syst.clone())?, index.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inSystem.clone(), inIndex.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }
    });
    Ok((outSystem, outShared, outIndex))
}

fn substituteCSE(mut inEq: Arc<BackendDAE::Equation>, mut inTuple: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)) -> Result<(Arc<BackendDAE::Equation>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>))> {
    let mut outEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut outTuple: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (HashTableExpToExp::FuncHashCref, HashTableExpToExp::FuncCrefEqual, HashTableExpToExp::FuncCrefStr, HashTableExpToExp::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>);
    (outEq, outTuple) = (::match_deref::match_deref! { match &(inEq.clone()) {
        Deref @ BackendDAE::Equation::ALGORITHM { .. } => {
            (inEq.clone(), inTuple.clone())
        },
        Deref @ BackendDAE::Equation::WHEN_EQUATION { .. } => {
            (inEq.clone(), inTuple.clone())
        },
        Deref @ BackendDAE::Equation::IF_EQUATION { .. } => {
            (inEq.clone(), inTuple.clone())
        },
        _ => {
            let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut tpl: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (HashTableExpToExp::FuncHashCref, HashTableExpToExp::FuncCrefEqual, HashTableExpToExp::FuncCrefStr, HashTableExpToExp::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>);
            if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("traverse ")); __mm_s.push_str(&*BackendDump::equationString(inEq.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            let (__pa0, (__pa1, _)) = BackendEquation::traverseExpsOfEquation(inEq.clone(), (std::sync::Arc::new(substituteCSE1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (HashTableExpToExp::FuncHashCref, HashTableExpToExp::FuncCrefEqual, HashTableExpToExp::FuncCrefStr, HashTableExpToExp::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>), Arc<DAE::ElementSource>)) -> Result<(Arc<DAE::Exp>, (((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (HashTableExpToExp::FuncHashCref, HashTableExpToExp::FuncCrefEqual, HashTableExpToExp::FuncCrefStr, HashTableExpToExp::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>), Arc<DAE::ElementSource>))> + 'static>), (inTuple.clone(), BackendEquation::equationSource(inEq.clone())?))?;
            eq = __pa0.clone();
            tpl = __pa1.clone();
            (eq.clone(), tpl.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outEq, outTuple))
}

fn substituteCSE1(mut inExp: Arc<DAE::Exp>, mut inTuple: (((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>), Arc<DAE::ElementSource>)) -> Result<(Arc<DAE::Exp>, (((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>), Arc<DAE::ElementSource>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTuple: (((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (HashTableExpToExp::FuncHashCref, HashTableExpToExp::FuncCrefEqual, HashTableExpToExp::FuncCrefStr, HashTableExpToExp::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>), Arc<DAE::ElementSource>);
    (outExp, outTuple) = Expression::traverseExpTopDown(inExp.clone(), (std::sync::Arc::new(substituteCSE_main) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (HashTableExpToExp::FuncHashCref, HashTableExpToExp::FuncCrefEqual, HashTableExpToExp::FuncCrefStr, HashTableExpToExp::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>), Arc<DAE::ElementSource>)) -> Result<(Arc<DAE::Exp>, bool, (((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (HashTableExpToExp::FuncHashCref, HashTableExpToExp::FuncCrefEqual, HashTableExpToExp::FuncCrefStr, HashTableExpToExp::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>), Arc<DAE::ElementSource>))> + 'static>), inTuple.clone())?;
    Ok((outExp, outTuple))
}

fn substituteCSE_main(mut inExp: Arc<DAE::Exp>, mut inTuple: (((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>), Arc<DAE::ElementSource>)) -> Result<(Arc<DAE::Exp>, bool, (((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>), Arc<DAE::ElementSource>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut outTuple: (((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (HashTableExpToExp::FuncHashCref, HashTableExpToExp::FuncCrefEqual, HashTableExpToExp::FuncCrefStr, HashTableExpToExp::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>), Arc<DAE::ElementSource>);
    (outExp, cont, outTuple) = 'mc: {
        let __mc_input = (inExp.clone(), inTuple.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { .. }, ((HT, HT2, HT3, eqLst, varLst), source)) => {
                    let mut value: Arc<DAE::Exp>;
                    let mut counter: i32 = 0;
                    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut HT3 = (*HT3).clone();
                    let mut eqLst = (*eqLst).clone();
                    let mut varLst = (*varLst).clone();
                    value = BaseHashTable::get(inExp.clone(), HT.clone())?;
                    counter = BaseHashTable::get(value.clone(), HT2.clone())?;
                    let true = (intGt(counter.clone(), 1)) else { bail!("pattern mismatch") };
                    if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  - substitute cse binary: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" (counter: ")); __mm_s.push_str(&*intString(counter.clone())); __mm_s.push_str(&*literal!(", id: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(value.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    if !(BaseHashTable::hasKey(value.clone(), HT3.clone())) {
                        HT3 = BaseHashTable::add((value.clone(), 1), HT3.clone())?;
                        varLst = createVarsForExp_onlyCSECrefs(value.clone(), varLst.clone())?;
                        eq = BackendEquation::generateEquation(value.clone(), inExp.clone(), source.clone(), BackendDAE::EQ_ATTR_DEFAULT_BINDING.clone())?;
                        eqLst = cons(eq.clone(), eqLst.clone());
                    }
                    Ok((value.clone(), true, ((HT.clone(), HT2.clone(), HT3.clone(), eqLst.clone(), varLst.clone()), source.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), true, inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, cont, outTuple))
}

fn createStatistics(mut inEq: Arc<BackendDAE::Equation>, mut inTuple: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), i32)) -> Result<(Arc<BackendDAE::Equation>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), i32))> {
    let mut outEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut outTuple: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (HashTableExpToExp::FuncHashCref, HashTableExpToExp::FuncCrefEqual, HashTableExpToExp::FuncCrefStr, HashTableExpToExp::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), i32);
    (outEq, outTuple) = (::match_deref::match_deref! { match &(inEq.clone()) {
        Deref @ BackendDAE::Equation::ALGORITHM { .. } => {
            (inEq.clone(), inTuple.clone())
        },
        Deref @ BackendDAE::Equation::WHEN_EQUATION { .. } => {
            (inEq.clone(), inTuple.clone())
        },
        Deref @ BackendDAE::Equation::IF_EQUATION { .. } => {
            (inEq.clone(), inTuple.clone())
        },
        _ => {
            let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut tpl: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (HashTableExpToExp::FuncHashCref, HashTableExpToExp::FuncCrefEqual, HashTableExpToExp::FuncCrefStr, HashTableExpToExp::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), i32);
            if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("traverse ")); __mm_s.push_str(&*BackendDump::equationString(inEq.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            (eq, tpl) = BackendEquation::traverseExpsOfEquation(inEq.clone(), (std::sync::Arc::new(createStatistics1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (HashTableExpToExp::FuncHashCref, HashTableExpToExp::FuncCrefEqual, HashTableExpToExp::FuncCrefStr, HashTableExpToExp::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), i32)) -> Result<(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (HashTableExpToExp::FuncHashCref, HashTableExpToExp::FuncCrefEqual, HashTableExpToExp::FuncCrefStr, HashTableExpToExp::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), i32))> + 'static>), inTuple.clone())?;
            (eq.clone(), tpl.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outEq, outTuple))
}

fn createStatistics1(mut inExp: Arc<DAE::Exp>, mut inTuple: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), i32)) -> Result<(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), i32))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTuple: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (HashTableExpToExp::FuncHashCref, HashTableExpToExp::FuncCrefEqual, HashTableExpToExp::FuncCrefStr, HashTableExpToExp::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), i32);
    (outExp, outTuple) = Expression::traverseExpTopDown(inExp.clone(), (std::sync::Arc::new(createStatistics_main) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (HashTableExpToExp::FuncHashCref, HashTableExpToExp::FuncCrefEqual, HashTableExpToExp::FuncCrefStr, HashTableExpToExp::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), i32)) -> Result<(Arc<DAE::Exp>, bool, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (HashTableExpToExp::FuncHashCref, HashTableExpToExp::FuncCrefEqual, HashTableExpToExp::FuncCrefStr, HashTableExpToExp::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), i32))> + 'static>), inTuple.clone())?;
    Ok((outExp, outTuple))
}

fn createStatistics_main(mut inExp: Arc<DAE::Exp>, mut inTuple: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), i32)) -> Result<(Arc<DAE::Exp>, bool, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), i32))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut outTuple: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>), i32, (HashTableExpToExp::FuncHashCref, HashTableExpToExp::FuncCrefEqual, HashTableExpToExp::FuncCrefStr, HashTableExpToExp::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), i32);
    (outExp, cont, outTuple) = 'mc: {
        let __mc_input = (inExp.clone(), inTuple.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: op, exp2 }, (HT, HT2, i)) => {
                    let mut value: Arc<DAE::Exp>;
                    let mut counter: i32 = 0;
                    let mut HT = (*HT).clone();
                    let mut HT2 = (*HT2).clone();
                    let mut i = (*i).clone();
                    if checkOp(op.clone()) {
                        if BaseHashTable::hasKey(inExp.clone(), HT.clone()) {
                            value = BaseHashTable::get(inExp.clone(), HT.clone())?;
                            counter = BaseHashTable::get(value.clone(), HT2.clone())? + 1;
                            BaseHashTable::update((value.clone(), counter.clone()), HT2.clone())?;
                            if isCommutative(op.clone()) {
                                        value = BaseHashTable::get(Arc::new(DAE::Exp::BINARY { exp1: exp2.clone(), operator: op.clone(), exp2: exp1.clone() }), HT.clone())?;
                                        BaseHashTable::update((value.clone(), counter.clone()), HT2.clone())?;
                            }
                        } else {
                            (value, i) = createReturnExp(Expression::r#typeof(inExp.clone())?, i.clone(), (literal!("$cseb")).clone(), true)?;
                            counter = 1;
                            HT = BaseHashTable::add((inExp.clone(), value.clone()), HT.clone())?;
                            HT2 = BaseHashTable::add((value.clone(), counter.clone()), HT2.clone())?;
                            if isCommutative(op.clone()) {
                                        HT = BaseHashTable::add((Arc::new(DAE::Exp::BINARY { exp1: exp2.clone(), operator: op.clone(), exp2: exp1.clone() }), value.clone()), HT.clone())?;
                            }
                        }
                        if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
                            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  - cse binary expression: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" (counter: ")); __mm_s.push_str(&*intString(counter.clone())); __mm_s.push_str(&*literal!(", id: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(value.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone());
                        }
                    }
                    Ok((inExp.clone(), true, (HT.clone(), HT2.clone(), i.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { .. }, _) => {
                    Ok((inExp.clone(), false, inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, _) => {
                    Ok((inExp.clone(), false, inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "smooth" }, .. }, _) => {
                    Ok((inExp.clone(), false, inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "noEvent" }, .. }, _) => {
                    Ok((inExp.clone(), false, inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, .. }, _) => {
                    Ok((inExp.clone(), false, inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "homotopy" }, .. }, _) => {
                    Ok((inExp.clone(), false, inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), true, inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, cont, outTuple))
}

fn isCommutative(mut inOp: DAE::Operator) -> bool {
    let mut outCommutative: bool = false;
    outCommutative = (match inOp.clone() {
        DAE::Operator::MUL { .. } => true,
        DAE::Operator::ADD { .. } => true,
        _ => false,
    });
    outCommutative
}

fn checkOp(mut inOp: DAE::Operator) -> bool {
    let mut outB: bool = false;
    outB = (match inOp.clone() {
        DAE::Operator::ADD { .. } => true,
        DAE::Operator::SUB { .. } => true,
        DAE::Operator::MUL { .. } => true,
        DAE::Operator::DIV { .. } => true,
        DAE::Operator::POW { .. } => true,
        DAE::Operator::UMINUS { .. } => true,
        _ => false,
    });
    outB
}

// =============================================================================
// Common Sub Expressions
//
// =============================================================================
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CommonSubExp {
    ASSIGNMENT_CSE {
        eqIdcs: Arc<metamodelica::List<i32>>,
        sharedVars: Arc<metamodelica::List<i32>>,
        aliasVars: Arc<metamodelica::List<i32>>,
    },
    SHORTCUT_CSE {
        eqIdcs: Arc<metamodelica::List<i32>>,
        sharedVar: i32,
    },
}
pub use self::CommonSubExp::{ASSIGNMENT_CSE,SHORTCUT_CSE};

pub fn commonSubExpressionReplacement(mut daeIn: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut daeOut: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    daeOut = BackendDAEUtil::mapEqSystem(daeIn.clone(), (std::sync::Arc::new(commonSubExpression) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> + 'static>))?;
    Ok(daeOut)
}

fn commonSubExpression(mut sysIn: Arc<BackendDAE::EqSystem>, mut sharedIn: Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> {
    let mut sysOut: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut sharedOut: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    (sysOut, sharedOut) = 'mc: {
        let __mc_input = (sysIn.clone(), sharedIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::EqSystem { orderedEqs: eqs, orderedVars: vars, .. }, Deref @ BackendDAE::Shared { functionTree, .. }) => {
                    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
                    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut cseLst: Arc<metamodelica::List<CommonSubExp>> = metamodelica::nil();
                    let mut isInitial: bool = false;
                    isInitial = BackendDAEUtil::isInitializationDAE(sharedIn.clone());
                    (_, m, mT) = BackendDAEUtil::getAdjacencyMatrix(sysIn.clone(), crate::BackendDAE::IndexType::ABSOLUTE, Some(functionTree.clone()), isInitial.clone())?;
                    cseLst = commonSubExpressionFind(m.clone(), mT.clone(), vars.clone(), eqs.clone(), isInitial.clone());
                    syst = commonSubExpressionUpdate(cseLst.clone(), m.clone(), mT.clone(), sysIn.clone())?;
                    GCExt::free(m.clone());
                    GCExt::free(mT.clone());
                    assign_field!(syst.orderedEqs = eqs.clone());
                    Ok((syst.clone(), sharedIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((sysIn.clone(), sharedIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((sysOut, sharedOut))
}

fn commonSubExpressionFind(mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mTIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut varsIn: BackendDAE::Variables, mut eqsIn: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut isInitial: bool) -> Arc<metamodelica::List<CommonSubExp>> {
    let mut cseOut: Arc<metamodelica::List<CommonSubExp>> = metamodelica::nil();
    let mut eqIdcs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut varIdcs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut lengthLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut range: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut partitions: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut eqSys: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut cseLst2: Arc<metamodelica::List<CommonSubExp>> = metamodelica::nil();
    let mut cseLst3: Arc<metamodelica::List<CommonSubExp>> = metamodelica::nil();
    let mut shortenPathsCSE: Arc<metamodelica::List<CommonSubExp>> = metamodelica::nil();
    let mut varIdcsSet: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
    match '__try0: {
        range = List::intRange((mIn.clone().borrow().len() as i32));
        lengthLst = List::mapArray(mIn.clone(), std::sync::Arc::new(fnptr!(listLength, _)));
        (_, eqIdcs) = unwrap_break_err!(List::filter1OnTrueSync(lengthLst.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 2, range.clone()), '__try0);
        (eqLst, eqIdcs) = unwrap_break_err!(List::filterOnTrueSync(BackendEquation::getList(eqIdcs.clone(), eqsIn.clone()), (std::sync::Arc::new(fnptr!(BackendEquation::isNotAlgorithm, Arc<BackendDAE::Equation>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<bool> + 'static>), eqIdcs.clone()), '__try0);
        eqs = unwrap_break_err!(BackendEquation::listEquation(eqLst.clone()), '__try0);
        varIdcs = UnorderedSet::unique_list(List::flatten(List::map1(eqIdcs.clone(), std::sync::Arc::new(fnptr!(Array::getIndexFirst, i32, _)), mIn.clone())), std::sync::Arc::new(fnptr!(Util::id, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>));
        varLst = List::map1(varIdcs.clone(), (std::sync::Arc::new(BackendVariable::getVarAtIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, BackendDAE::Variables) -> Result<BackendDAE::Var> + 'static>), varsIn.clone());
        vars = BackendVariable::listVar1(varLst.clone());
        eqSys = BackendDAEUtil::createEqSystem(vars.clone(), eqs.clone(), metamodelica::nil(), crate::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
        (_, m, mT) = unwrap_break_err!(BackendDAEUtil::getAdjacencyMatrix(eqSys.clone(), crate::BackendDAE::IndexType::ABSOLUTE, None, isInitial.clone()), '__try0);
        partitions = unwrap_break_err!(ResolveLoops::partitionBipartiteGraph(m.clone(), mT.clone()), '__try0);
        partitions = List::filterOnFalse(partitions.clone(), std::sync::Arc::new(fnptr!(listEmpty, _)));
        cseLst2 = List::fold(partitions.clone(), Arc::new({ let __pe_b1 = m.clone(); let __pe_b2 = mT.clone(); let __pe_b3 = vars.clone(); let __pe_b4 = eqs.clone(); let __pe_b5 = eqIdcs.clone(); let __pe_b6 = varIdcs.clone(); move |__pe_a0, __pe_a7| getCSE2(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone(), __pe_a7) }), metamodelica::nil());
        shortenPathsCSE = shortenPaths(partitions.clone(), m.clone(), mT.clone(), vars.clone(), eqs.clone(), metamodelica::arrayFromVec(eqIdcs.clone().into_iter().cloned().collect()), metamodelica::arrayFromVec(varIdcs.clone().into_iter().cloned().collect()), metamodelica::nil(), isInitial.clone());
        (_, eqIdcs) = unwrap_break_err!(List::filter1OnTrueSync(lengthLst.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 3, range.clone()), '__try0);
        (eqLst, eqIdcs) = unwrap_break_err!(List::filterOnTrueSync(BackendEquation::getList(eqIdcs.clone(), eqsIn.clone()), (std::sync::Arc::new(fnptr!(BackendEquation::isNotAlgorithm, Arc<BackendDAE::Equation>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<bool> + 'static>), eqIdcs.clone()), '__try0);
        eqs = unwrap_break_err!(BackendEquation::listEquation(eqLst.clone()), '__try0);
        varIdcsSet = Arc::new(crate::AvlSetInt::Tree::EMPTY);
        for mut eq in &*eqIdcs.clone() {
            let mut eq = eq.clone();
            varIdcsSet = unwrap_break_err!(AvlSetInt::addList(varIdcsSet.clone(), mIn.clone().borrow()[(eq.clone()-1) as usize].clone()), '__try0);
        }
        varIdcs = AvlSetInt::listKeysReverse(varIdcsSet.clone(), metamodelica::nil());
        varLst = List::map1(varIdcs.clone(), (std::sync::Arc::new(BackendVariable::getVarAtIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, BackendDAE::Variables) -> Result<BackendDAE::Var> + 'static>), varsIn.clone());
        vars = BackendVariable::listVar1(varLst.clone());
        eqSys = BackendDAEUtil::createEqSystem(vars.clone(), eqs.clone(), metamodelica::nil(), crate::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
        (_, m, mT) = unwrap_break_err!(BackendDAEUtil::getAdjacencyMatrix(eqSys.clone(), crate::BackendDAE::IndexType::ABSOLUTE, None, isInitial.clone()), '__try0);
        partitions = unwrap_break_err!(ResolveLoops::partitionBipartiteGraph(m.clone(), mT.clone()), '__try0);
        cseLst3 = List::fold(partitions.clone(), Arc::new({ let __pe_b1 = m.clone(); let __pe_b2 = mT.clone(); let __pe_b3 = vars.clone(); let __pe_b4 = eqs.clone(); let __pe_b5 = eqIdcs.clone(); let __pe_b6 = varIdcs.clone(); move |__pe_a0, __pe_a7| getCSE3(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone(), __pe_a7) }), metamodelica::nil());
        cseOut = listAppend(cseLst2.clone(), listAppend(cseLst3.clone(), shortenPathsCSE.clone()));
        Ok::<_, anyhow::Error>((cseOut.clone(),))
    } {
        Ok((__try0_o0,)) => {
            cseOut = __try0_o0;
        }
        Err(_) => {
            cseOut = metamodelica::nil();
        }
    }
    cseOut
}

fn shortenPaths(mut allPartitions: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mTIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut allVars: BackendDAE::Variables, mut allEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut eqMap: metamodelica::Array<i32>, mut varMap: metamodelica::Array<i32>, mut cseIn: Arc<metamodelica::List<CommonSubExp>>, mut isInitial: bool) -> Arc<metamodelica::List<CommonSubExp>> {
    let mut cseOut: Arc<metamodelica::List<CommonSubExp>> = metamodelica::nil();
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut eqSys: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut pathVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut numVars: i32 = 0;
    let mut varIdx: i32 = 0;
    let mut pathVarIdxMap: metamodelica::Array<i32>;
    let mut partition: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut adjEqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut pathVarIdcs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut cses: Arc<metamodelica::List<CommonSubExp>> = metamodelica::nil();
    match '__try0: {
        numVars = unwrap_break_err!(BackendVariable::varsSize(allVars.clone()), '__try0);
        (_, pathVarIdcs) = unwrap_break_err!(List::filter1OnTrueSync(List::mapArray(mTIn.clone(), std::sync::Arc::new(fnptr!(listLength, _))), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 2, List::intRange(numVars.clone())), '__try0);
        pathVars = BackendVariable::listVar1(List::map1(pathVarIdcs.clone(), (std::sync::Arc::new(BackendVariable::getVarAtIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, BackendDAE::Variables) -> Result<BackendDAE::Var> + 'static>), allVars.clone()));
        pathVarIdxMap = metamodelica::arrayFromVec(List::map1(pathVarIdcs.clone(), std::sync::Arc::new(fnptr!(Array::getIndexFirst, i32, _)), varMap.clone()).into_iter().cloned().collect());
        cses = cseIn.clone();
        if unwrap_break_err!(BackendVariable::varsSize(pathVars.clone()), '__try0) > 0 {
            for mut partition in &*allPartitions.clone() {
                let mut partition = partition.clone();
                eqLst = ({
        let mut __acc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
        for mut i in (partition.clone()).into_iter().cloned() {
            let __x = BackendEquation::get(allEqs.clone(), i.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                eqs = unwrap_break_err!(BackendEquation::listEquation(eqLst.clone()), '__try0);
                eqSys = BackendDAEUtil::createEqSystem(pathVars.clone(), eqs.clone(), metamodelica::nil(), crate::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
                (_, m, mT) = unwrap_break_err!(BackendDAEUtil::getAdjacencyMatrix(eqSys.clone(), crate::BackendDAE::IndexType::SOLVABLE, None, isInitial.clone()), '__try0);
                let __range1 = 1..=(mT.clone().borrow().len() as i32);
                for mut idx in __range1 {
                    adjEqs = unwrap_break_err!(metamodelica::Dangerous::arrayGet(mT.clone(), idx.clone()), '__try0);
                    if (adjEqs.clone().len() as i32) == 2 {
                        adjEqs = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut eq in (adjEqs.clone()).into_iter().cloned() {
            let __x = eqMap.clone().borrow()[(unwrap_break_err!((partition.clone()).get(eq.clone()), '__try0)-1) as usize].clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                        varIdx = pathVarIdxMap.clone().borrow()[(idx.clone()-1) as usize].clone();
                        cses = cons(CommonSubExp::SHORTCUT_CSE { eqIdcs: adjEqs.clone(), sharedVar: varIdx.clone() }, cses.clone());
                    }
                }
                GCExt::free(m.clone());
                GCExt::free(mT.clone());
            }
        }
        cseOut = cses.clone();
        Ok::<_, anyhow::Error>((cseOut.clone(),))
    } {
        Ok((__try0_o0,)) => {
            cseOut = __try0_o0;
        }
        Err(_) => {
            cseOut = cseIn.clone();
        }
    }
    cseOut
}

fn getCSE2(mut partition: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut vars: BackendDAE::Variables, mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut eqMap: Arc<metamodelica::List<i32>>, mut varMap: Arc<metamodelica::List<i32>>, mut cseIn: Arc<metamodelica::List<CommonSubExp>>) -> Result<Arc<metamodelica::List<CommonSubExp>>> {
    let mut cseOut: Arc<metamodelica::List<CommonSubExp>> = metamodelica::nil();
    cseOut = 'mc: {
        let __mc_input = partition.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: eqIdx1, tail: Deref @ metamodelica::List::Cons { head: eqIdx2, tail: Deref @ metamodelica::List::Nil } } => {
                    let mut sharedVarIdx: i32 = 0;
                    let mut varIdx1: i32 = 0;
                    let mut varIdx2: i32 = 0;
                    let mut varIdcs1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut varIdcs2: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut sharedVarIdcs: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut eqIdcs: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut eq1: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut eq2: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut var1: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut var2: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut varExp1: Arc<DAE::Exp>;
                    let mut varExp2: Arc<DAE::Exp>;
                    let mut lhs: Arc<DAE::Exp>;
                    let mut rhs1: Arc<DAE::Exp>;
                    let mut rhs2: Arc<DAE::Exp>;
                    varIdcs1 = m.clone().borrow()[(eqIdx1.clone()-1) as usize].clone();
                    varIdcs2 = m.clone().borrow()[(eqIdx2.clone()-1) as usize].clone();
                    (sharedVarIdcs, varIdcs1, varIdcs2) = List::intersection1OnTrue(varIdcs1.clone(), varIdcs2.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    let __pa0 = ::match_deref::match_deref! { match &(varIdcs1.clone()) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    varIdx1 = __pa0.clone();
                    let __pa2 = ::match_deref::match_deref! { match &(varIdcs2.clone()) {
                        Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil } => __pa2.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    varIdx2 = __pa2.clone();
                    let __pa4 = ::match_deref::match_deref! { match &(sharedVarIdcs.clone()) {
                        Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Nil } => __pa4.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    sharedVarIdx = __pa4.clone();
                    let (__pa6, __pa7) = ::match_deref::match_deref! { match &(BackendEquation::getList(partition.clone(), eqs.clone())) {
                        Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Nil } } => (__pa6.clone(), __pa7.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    eq1 = __pa6.clone();
                    eq2 = __pa7.clone();
                    BackendVariable::getVarAt(vars.clone(), sharedVarIdx.clone())?;
                    var1 = BackendVariable::getVarAt(vars.clone(), varIdx1.clone())?;
                    var2 = BackendVariable::getVarAt(vars.clone(), varIdx2.clone())?;
                    varExp1 = BackendVariable::varExp(var1.clone())?;
                    varExp2 = BackendVariable::varExp(var2.clone())?;
                    let (__pa9, __pa10) = ::match_deref::match_deref! { match &(eq1.clone()) {
                        Deref @ BackendDAE::Equation::EQUATION { scalar: __pa9, exp: __pa10, .. } => (__pa9.clone(), __pa10.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    rhs1 = __pa9.clone();
                    lhs = __pa10.clone();
                    (rhs1, _) = ExpressionSolve::solve(lhs.clone(), rhs1.clone(), varExp1.clone(), None)?;
                    let (__pa11, __pa12) = ::match_deref::match_deref! { match &(eq2.clone()) {
                        Deref @ BackendDAE::Equation::EQUATION { scalar: __pa11, exp: __pa12, .. } => (__pa11.clone(), __pa12.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    rhs2 = __pa11.clone();
                    lhs = __pa12.clone();
                    (rhs2, _) = ExpressionSolve::solve(lhs.clone(), rhs2.clone(), varExp2.clone(), None)?;
                    let true = (ExpressionBasics::expEqual(rhs1.clone(), rhs2.clone())?) else { bail!("pattern mismatch") };
                    sharedVarIdcs = List::map1(sharedVarIdcs.clone(), std::sync::Arc::new(fnptr!(List::getIndexFirst, i32, _)), varMap.clone());
                    varIdcs2 = listAppend(varIdcs1.clone(), varIdcs2.clone());
                    varIdcs2 = List::map1(varIdcs2.clone(), std::sync::Arc::new(fnptr!(List::getIndexFirst, i32, _)), varMap.clone());
                    eqIdcs = List::map1(partition.clone(), std::sync::Arc::new(fnptr!(List::getIndexFirst, i32, _)), eqMap.clone());
                    Ok(cons(CommonSubExp::ASSIGNMENT_CSE { eqIdcs: eqIdcs.clone(), sharedVars: sharedVarIdcs.clone(), aliasVars: varIdcs2.clone() }, cseIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(cseIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(cseOut)
}

fn getCSE3(mut partition: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut vars: BackendDAE::Variables, mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut eqMap: Arc<metamodelica::List<i32>>, mut varMap: Arc<metamodelica::List<i32>>, mut cseIn: Arc<metamodelica::List<CommonSubExp>>) -> Result<Arc<metamodelica::List<CommonSubExp>>> {
    let mut cseOut: Arc<metamodelica::List<CommonSubExp>> = metamodelica::nil();
    cseOut = 'mc: {
        let __mc_input = cseIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut eqIdx1: i32 = 0;
                    let mut eqIdx2: i32 = 0;
                    let mut varIdx1: i32 = 0;
                    let mut varIdx2: i32 = 0;
                    let mut varIdcs1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut varIdcs2: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut sharedVarIdcs: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut eqIdcs: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut loop1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut loops: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut eq1: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut eq2: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut var1: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut var2: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut varExp1: Arc<DAE::Exp>;
                    let mut varExp2: Arc<DAE::Exp>;
                    let mut lhs: Arc<DAE::Exp>;
                    let mut rhs1: Arc<DAE::Exp>;
                    let mut rhs2: Arc<DAE::Exp>;
                    let mut varMapArr: metamodelica::Array<i32>;
                    let mut eqMapArr: metamodelica::Array<i32>;
                    let mut cseLst: Arc<metamodelica::List<CommonSubExp>> = metamodelica::nil();
                    (loops, _, _, _) = ResolveLoops::resolveLoops_findLoops(list![partition.clone()], m.clone(), mT.clone(), false);
                    cseLst = cseIn.clone();
                    for mut loop1 in &*loops.clone() {
                        let mut loop1 = loop1.clone();
                        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(loop1.clone()) {
                            Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } } => (__pa0.clone(), __pa1.clone()),
                            _ => bail!("pattern mismatch"),
                        } };
                        eqIdx1 = __pa0.clone();
                        eqIdx2 = __pa1.clone();
                        varIdcs1 = m.clone().borrow()[(eqIdx1.clone()-1) as usize].clone();
                        varIdcs2 = m.clone().borrow()[(eqIdx2.clone()-1) as usize].clone();
                        (sharedVarIdcs, varIdcs1, varIdcs2) = List::intersection1OnTrue(varIdcs1.clone(), varIdcs2.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                        let __pa3 = ::match_deref::match_deref! { match &(varIdcs1.clone()) {
                            Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Nil } => __pa3.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        varIdx1 = __pa3.clone();
                        let __pa5 = ::match_deref::match_deref! { match &(varIdcs2.clone()) {
                            Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Nil } => __pa5.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        varIdx2 = __pa5.clone();
                        let (__pa7, __pa8) = ::match_deref::match_deref! { match &(BackendEquation::getList(loop1.clone(), eqs.clone())) {
                            Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Cons { head: __pa8, tail: Deref @ metamodelica::List::Nil } } => (__pa7.clone(), __pa8.clone()),
                            _ => bail!("pattern mismatch"),
                        } };
                        eq1 = __pa7.clone();
                        eq2 = __pa8.clone();
                        var1 = BackendVariable::getVarAt(vars.clone(), varIdx1.clone())?;
                        var2 = BackendVariable::getVarAt(vars.clone(), varIdx2.clone())?;
                        varExp1 = BackendVariable::varExp(var1.clone())?;
                        varExp2 = BackendVariable::varExp(var2.clone())?;
                        let (__pa10, __pa11) = ::match_deref::match_deref! { match &(eq1.clone()) {
                            Deref @ BackendDAE::Equation::EQUATION { scalar: __pa10, exp: __pa11, .. } => (__pa10.clone(), __pa11.clone()),
                            _ => bail!("pattern mismatch"),
                        } };
                        rhs1 = __pa10.clone();
                        lhs = __pa11.clone();
                        (rhs1, _) = ExpressionSolve::solve(lhs.clone(), rhs1.clone(), varExp1.clone(), None)?;
                        let (__pa12, __pa13) = ::match_deref::match_deref! { match &(eq2.clone()) {
                            Deref @ BackendDAE::Equation::EQUATION { scalar: __pa12, exp: __pa13, .. } => (__pa12.clone(), __pa13.clone()),
                            _ => bail!("pattern mismatch"),
                        } };
                        rhs2 = __pa12.clone();
                        lhs = __pa13.clone();
                        (rhs2, _) = ExpressionSolve::solve(lhs.clone(), rhs2.clone(), varExp2.clone(), None)?;
                        if ExpressionBasics::expEqual(rhs1.clone(), rhs2.clone())? {
                            eqMapArr = metamodelica::arrayFromVec(eqMap.clone().into_iter().cloned().collect());
                            varMapArr = metamodelica::arrayFromVec(varMap.clone().into_iter().cloned().collect());
                            sharedVarIdcs = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (sharedVarIdcs.clone()).into_iter().cloned() {
                    let __x = varMapArr.clone().borrow()[(i.clone()-1) as usize].clone();
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                            varIdcs2 = listAppend(varIdcs1.clone(), varIdcs2.clone());
                            varIdcs2 = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (varIdcs2.clone()).into_iter().cloned() {
                    let __x = varMapArr.clone().borrow()[(i.clone()-1) as usize].clone();
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                            eqIdcs = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (loop1.clone()).into_iter().cloned() {
                    let __x = eqMapArr.clone().borrow()[(i.clone()-1) as usize].clone();
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                            GCExt::free(eqMapArr.clone());
                            GCExt::free(varMapArr.clone());
                            cseLst = cons(CommonSubExp::ASSIGNMENT_CSE { eqIdcs: eqIdcs.clone(), sharedVars: sharedVarIdcs.clone(), aliasVars: varIdcs2.clone() }, cseLst.clone());
                        }
                    }
                    Ok(cseLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(cseIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(cseOut)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn commonSubExpressionUpdate(mut tplsIn: Arc<metamodelica::List<CommonSubExp>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut sysIn: Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut sysOut: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    sysOut = (::match_deref::match_deref! { match &((tplsIn.clone(), sysIn.clone())) {
        (Deref @ metamodelica::List::Nil, syst @ Deref @ BackendDAE::EqSystem { .. }) => {
            BackendDAEUtil::clearEqSyst(syst.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: CommonSubExp::ASSIGNMENT_CSE { aliasVars: Deref @ metamodelica::List::Cons { head: varIdx1, tail: Deref @ metamodelica::List::Cons { head: varIdx2, tail: Deref @ metamodelica::List::Nil } }, eqIdcs: Deref @ metamodelica::List::Cons { head: eqIdx1, tail: Deref @ metamodelica::List::Cons { head: eqIdx2, tail: Deref @ metamodelica::List::Nil } }, .. }, tail: rest }, syst @ Deref @ BackendDAE::EqSystem { orderedEqs: eqs, orderedVars: vars, .. }) => {
            let mut varIdx_remain: i32 = 0;
            let mut varIdxAlias: i32 = 0;
            let mut eqIdxDel: i32 = 0;
            let mut eqIdcs: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut eqs1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut eqs2: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut var1: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
            let mut var2: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
            let mut var_remain: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
            let mut var_alias: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
            let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
            let mut varExp_remain: Arc<DAE::Exp>;
            let mut varExp_alias: Arc<DAE::Exp>;
            let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut eqs = (*eqs).clone();
            repl = BackendVarTransform::emptyReplacements();
            eqs1 = mT.clone().borrow()[(varIdx1.clone()-1) as usize].clone();
            eqs2 = mT.clone().borrow()[(varIdx2.clone()-1) as usize].clone();
            var1 = BackendVariable::getVarAt(vars.clone(), varIdx1.clone())?;
            var2 = BackendVariable::getVarAt(vars.clone(), varIdx2.clone())?;
            if BackendVariable::isStateVar(var1.clone()) {
                varIdxAlias = varIdx2.clone();
                varIdx_remain = varIdx1.clone();
            } else if BackendVariable::isStateVar(var2.clone()) {
                varIdx_remain = varIdx2.clone();
                varIdxAlias = varIdx1.clone();
            } else {
                if intLe((eqs2.clone().len() as i32), (eqs1.clone().len() as i32)) {
                    varIdxAlias = varIdx2.clone();
                    varIdx_remain = varIdx1.clone();
                } else {
                    varIdxAlias = varIdx1.clone();
                    varIdx_remain = varIdx2.clone();
                }
            }
            if intLe((eqs2.clone().len() as i32), (eqs1.clone().len() as i32)) {
                eqIdxDel = eqIdx2.clone();
            } else {
                eqIdxDel = eqIdx1.clone();
            }
            var_remain = BackendVariable::getVarAt(vars.clone(), varIdx_remain.clone())?;
            var_alias = BackendVariable::getVarAt(vars.clone(), varIdxAlias.clone())?;
            cref = BackendVariable::varCref(var_alias.clone())?;
            varExp_remain = BackendVariable::varExp(var_remain.clone())?;
            varExp_alias = BackendVariable::varExp(var_alias.clone())?;
            repl = BackendVarTransform::addReplacement(repl.clone(), cref.clone(), varExp_remain.clone(), None)?;
            eqIdcs = mT.clone().borrow()[(varIdxAlias.clone()-1) as usize].clone();
            eqLst = BackendEquation::getList(eqIdcs.clone(), eqs.clone());
            eqs = List::threadFold(eqIdcs.clone(), eqLst.clone(), (std::sync::Arc::new(fnptr!(BackendEquation::setAtIndexFirst, i32, Arc<BackendDAE::Equation>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<BackendDAE::Equation>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> + 'static>), eqs.clone())?;
            BackendEquation::setAtIndex(eqs.clone(), eqIdxDel.clone(), Arc::new(BackendDAE::Equation::EQUATION { exp: varExp_remain.clone(), scalar: varExp_alias.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() }))?;
            commonSubExpressionUpdate(rest.clone(), m.clone(), mT.clone(), syst.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: CommonSubExp::SHORTCUT_CSE { sharedVar, eqIdcs: Deref @ metamodelica::List::Cons { head: eqIdx1, tail: Deref @ metamodelica::List::Cons { head: eqIdx2, tail: Deref @ metamodelica::List::Nil } } }, tail: rest }, syst @ Deref @ BackendDAE::EqSystem { orderedEqs: eqs, orderedVars: vars, .. }) => {
            let mut n: i32 = 0;
            let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
            let mut eq1: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut eq2: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut eqNew: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut lhs1: Arc<DAE::Exp>;
            let mut rhs1: Arc<DAE::Exp>;
            let mut lhs2: Arc<DAE::Exp>;
            let mut rhs2: Arc<DAE::Exp>;
            let mut varExp: Arc<DAE::Exp>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(BackendEquation::getList(list![eqIdx1.clone(), eqIdx2.clone()], eqs.clone())) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            eq1 = __pa0.clone();
            eq2 = __pa1.clone();
            var = BackendVariable::getVarAt(vars.clone(), sharedVar.clone())?;
            varExp = BackendVariable::varExp(var.clone())?;
            Expression::r#typeof(varExp.clone())?;
            let (__pa3, __pa4) = ::match_deref::match_deref! { match &(eq1.clone()) {
                Deref @ BackendDAE::Equation::EQUATION { scalar: __pa3, exp: __pa4, .. } => (__pa3.clone(), __pa4.clone()),
                _ => bail!("pattern mismatch"),
            } };
            rhs1 = __pa3.clone();
            lhs1 = __pa4.clone();
            let (__pa5, __pa6) = ::match_deref::match_deref! { match &(eq2.clone()) {
                Deref @ BackendDAE::Equation::EQUATION { scalar: __pa5, exp: __pa6, .. } => (__pa5.clone(), __pa6.clone()),
                _ => bail!("pattern mismatch"),
            } };
            rhs2 = __pa5.clone();
            lhs2 = __pa6.clone();
            let true = (hasAlgebraicOperationsOnly(lhs1.clone())) else { bail!("pattern mismatch") };
            let true = (hasAlgebraicOperationsOnly(rhs1.clone())) else { bail!("pattern mismatch") };
            let true = (hasAlgebraicOperationsOnly(lhs2.clone())) else { bail!("pattern mismatch") };
            let true = (hasAlgebraicOperationsOnly(rhs2.clone())) else { bail!("pattern mismatch") };
            (rhs1, _) = ExpressionSolve::solve(lhs1.clone(), rhs1.clone(), varExp.clone(), None)?;
            (lhs1, _) = ExpressionSolve::solve(lhs2.clone(), rhs2.clone(), varExp.clone(), None)?;
            (_, lhs1, rhs1) = cancelExpressions(lhs1.clone(), rhs1.clone())?;
            n = (Expression::getAllCrefs(Expression::expSub(lhs1.clone(), rhs1.clone())?)?.len() as i32);
            if n.clone() <= 2 {
                eqNew = Arc::new(BackendDAE::Equation::EQUATION { exp: lhs1.clone(), scalar: rhs1.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                BackendEquation::setAtIndex(eqs.clone(), eqIdx1.clone(), eqNew.clone())?;
            }
            commonSubExpressionUpdate(rest.clone(), m.clone(), mT.clone(), syst.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _) => {
            commonSubExpressionUpdate(rest.clone(), m.clone(), mT.clone(), sysIn.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(sysOut)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn hasAlgebraicOperationsOnly(mut exp: Arc<DAE::Exp>) -> bool {
    let mut isAlgOut: bool = false;
    isAlgOut = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::RCONST { .. } => {
            true
        },
        Deref @ DAE::Exp::CREF { .. } => {
            true
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: _, exp2: e2 } => {
            let mut b: bool = false;
            b = hasAlgebraicOperationsOnly(e1.clone());
            b = b.clone() && hasAlgebraicOperationsOnly(e2.clone());
            b.clone()
        },
        Deref @ DAE::Exp::UNARY { operator: _, exp: e1 } => {
            let mut b: bool = false;
            b = hasAlgebraicOperationsOnly(e1.clone());
            b.clone()
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isAlgOut
}

fn cancelExpressions(mut e1In: Arc<DAE::Exp>, mut e2In: Arc<DAE::Exp>) -> Result<(bool, Arc<DAE::Exp>, Arc<DAE::Exp>)> {
    let mut canceled: bool = false;
    let mut e1Out: Arc<DAE::Exp> = e1In.clone();
    let mut e2Out: Arc<DAE::Exp> = e2In.clone();
    let mut topLevelFactors1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut topLevelFactors2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    topLevelFactors1 = getTopLevelFactors(e1In.clone(), metamodelica::nil())?;
    topLevelFactors2 = getTopLevelFactors(e2In.clone(), metamodelica::nil())?;
    if !(topLevelFactors1.clone().is_empty()) && !(topLevelFactors1.clone().is_empty()) {
        topLevelFactors1 = List::intersectionOnTrue(topLevelFactors1.clone(), topLevelFactors2.clone(), (std::sync::Arc::new(ExpressionBasics::expEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>));
        if (topLevelFactors1.clone().len() as i32) == 1 {
            e1Out = Expression::expDiv(e1In.clone(), listHead(topLevelFactors1.clone())?)?;
            (e1Out, _) = ExpressionSimplify::simplify(e1Out.clone())?;
            e2Out = Expression::expDiv(e2In.clone(), listHead(topLevelFactors2.clone())?)?;
            (e2Out, _) = ExpressionSimplify::simplify(e2Out.clone())?;
            canceled = true;
        }
    }
    Ok((canceled, e1Out, e2Out))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getTopLevelFactors(mut exp: Arc<DAE::Exp>, mut lstIn: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut lstOut: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    lstOut = 'mc: {
        let __mc_input = exp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { ty: _ }, exp2: e2 } => {
                    let mut eLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    eLst = getTopLevelFactors(e1.clone(), lstIn.clone())?;
                    eLst = getTopLevelFactors(e2.clone(), eLst.clone())?;
                    Ok(eLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::UNARY { operator: _, exp: e1 @ Deref @ DAE::Exp::CREF { .. } } => {
                    Ok(cons(e1.clone(), lstIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e1 @ Deref @ DAE::Exp::CREF { .. } => {
                    Ok(cons(e1.clone(), lstIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(lstIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(lstOut)
}

fn printCSE(mut cse: CommonSubExp) -> Result<ArcStr> {
    let mut s: ArcStr = arcstr::literal!("");
    s = ((match cse.clone() {
        CommonSubExp::ASSIGNMENT_CSE { aliasVars: mut aliasVars, sharedVars: mut sharedVars, eqIdcs: mut eqIdcs } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ASSIGN_CSE: eqs{")); __mm_s.push_str(&*stringDelimitList(List::map(eqIdcs.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("}")); __mm_s.push_str(&*literal!("   sharedVars{")); __mm_s.push_str(&*stringDelimitList(List::map(sharedVars.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("}")); __mm_s.push_str(&*literal!("   aliasVars{")); __mm_s.push_str(&*stringDelimitList(List::map(aliasVars.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }
        },
        CommonSubExp::SHORTCUT_CSE { eqIdcs: mut eqIdcs, sharedVar: mut sharedVar } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SHORTCUT_CSE: eqs{")); __mm_s.push_str(&*stringDelimitList(List::map(eqIdcs.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("}")); __mm_s.push_str(&*literal!("   sharedVar{")); __mm_s.push_str(&*intString(sharedVar.clone())); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(s)
}

