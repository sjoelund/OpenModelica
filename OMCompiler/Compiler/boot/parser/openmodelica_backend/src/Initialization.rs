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
use crate::BackendDAECreate;
use crate::BackendDAEEXT;
use crate::BackendDAEFunc;
use crate::BackendDAEOptimize;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVarTransform;
use crate::BackendVariable;
use crate::IndexReduction;
use crate::Matching;
use crate::Sorting;
use crate::SymbolicJacobian;
use crate::SynchronousFeatures;
use openmodelica_ast::Absyn;
use openmodelica_frontend::AvlSetCR;
use openmodelica_frontend::CheckModel;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::DAEUtil;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionSimplify;
use openmodelica_frontend::HashSet;
use openmodelica_frontend::Types;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashSet;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::GCExt;
use openmodelica_util_datatypes_basic::List;

// =============================================================================
// section for all public functions
//
// These are functions that can be used to access the initialization.
// =============================================================================
pub fn solveInitialSystem(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<(Arc<BackendDAE::BackendDAE>, Option<Arc<BackendDAE::BackendDAE>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, BackendDAE::Variables, Arc<BackendDAE::BackendDAE>)> {
    let mut outInitDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut outInitDAE_lambda0: Option<Arc<BackendDAE::BackendDAE>> = None;
    let mut outRemovedInitialEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut outGlobalKnownVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut outSimDAE: Arc<BackendDAE::BackendDAE> = inDAE.clone();
    let mut dae: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut initdae: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut initdae0: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut initsyst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut initsyst0: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut reeqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut eqnsLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut reeqnsLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut initVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut fixvars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut useHomotopy: bool = false;
    let mut datarecon: bool = false;
    let mut msg: ArcStr = arcstr::literal!("");
    let mut enabledModules: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut disabledModules: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    let mut removedEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut dumpVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut dumpVars2: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut outAllPrimaryParameters: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut allPrimaryParameters: Arc<AvlSetCR::Tree> = Arc::new(AvlSetCR::Tree::EMPTY);
    match '__try0: {
        dae = unwrap_break_err!(inlineWhenForInitialization(inDAE.clone()), '__try0);
        unwrap_break_err!(execStat((literal!("inlineWhenForInitialization (initialization)")).clone()), '__try0);
        (dae, initVars, outAllPrimaryParameters, outGlobalKnownVars) = unwrap_break_err!(selectInitializationVariablesDAE(dae.clone()), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::DUMP_INITIAL_SYSTEM.clone()), '__try0) {
            BackendDump::dumpVarList(outAllPrimaryParameters.clone(), (literal!("selected all primary parameters")).clone());
        }
        unwrap_break_err!(execStat((literal!("selectInitializationVariablesDAE (initialization)")).clone()), '__try0);
        hs = unwrap_break_err!(collectPreVariables(dae.clone()), '__try0);
        unwrap_break_err!(execStat((literal!("collectPreVariables (initialization)")).clone()), '__try0);
        vars = BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone());
        fixvars = BackendVariable::listVar(outAllPrimaryParameters.clone());
        eqns = BackendEquation::emptyEqnsSized(BackendVariable::varsSize(dae.shared.aliasVars.clone())? + BackendVariable::varsSize(dae.shared.globalKnownVars.clone())? + BackendVariable::varsSize(dae.shared.localKnownVars.clone())? + BackendEquation::getNumberOfEquations(dae.shared.initialEqs.clone()) + 2 * BackendDAEUtil::daeSize(dae.clone()));
        reeqns = BackendEquation::emptyEqnsSized(BackendEquation::getNumberOfEquations(dae.shared.removedEqs.clone()));
        allPrimaryParameters = Arc::new(openmodelica_frontend::AvlSetCR::Tree::EMPTY);
        for mut v in &*outAllPrimaryParameters.clone() {
            let mut v = v.clone();
            allPrimaryParameters = unwrap_break_err!(AvlSetCR::add(allPrimaryParameters.clone(), BackendVariable::varCref(v.clone())?), '__try0);
        }
        if isSome(inDAE.shared.dataReconciliationData.clone()) {
            datarecon = true;
        }
        (vars, fixvars, eqns, _) = unwrap_break_err!(BackendVariable::traverseBackendDAEVars(dae.shared.aliasVars.clone(), (std::sync::Arc::new(introducePreVarsForAliasVariables) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (BackendDAE::Variables, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)))) -> Result<(BackendDAE::Var, (BackendDAE::Variables, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))))> + 'static>), (vars.clone(), fixvars.clone(), eqns.clone(), hs.clone())), '__try0);
        (vars, fixvars, eqns, _, _, _, _) = unwrap_break_err!(BackendVariable::traverseBackendDAEVars(dae.shared.globalKnownVars.clone(), (std::sync::Arc::new(collectInitialVars) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (BackendDAE::Variables, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, metamodelica::Array<i32>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), Arc<AvlSetCR::Tree>, bool)) -> Result<(BackendDAE::Var, (BackendDAE::Variables, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, metamodelica::Array<i32>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), Arc<AvlSetCR::Tree>, bool))> + 'static>), (vars.clone(), fixvars.clone(), eqns.clone(), arrayCreate(0, 0), hs.clone(), allPrimaryParameters.clone(), datarecon.clone())), '__try0);
        (vars, fixvars, eqns, _, _, _, _) = unwrap_break_err!(BackendVariable::traverseBackendDAEVars(dae.shared.localKnownVars.clone(), (std::sync::Arc::new(collectInitialVars) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (BackendDAE::Variables, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, metamodelica::Array<i32>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), Arc<AvlSetCR::Tree>, bool)) -> Result<(BackendDAE::Var, (BackendDAE::Variables, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, metamodelica::Array<i32>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), Arc<AvlSetCR::Tree>, bool))> + 'static>), (vars.clone(), fixvars.clone(), eqns.clone(), arrayCreate(0, 0), hs.clone(), allPrimaryParameters.clone(), datarecon.clone())), '__try0);
        (eqns, reeqns) = unwrap_break_err!(BackendEquation::traverseEquationArray(dae.shared.initialEqs.clone(), (std::sync::Arc::new(collectInitialEqns) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)) -> Result<(Arc<BackendDAE::Equation>, (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>))> + 'static>), (eqns.clone(), reeqns.clone())), '__try0);
        (eqns, reeqns) = unwrap_break_err!(BackendEquation::traverseEquationArray(dae.shared.removedEqs.clone(), (std::sync::Arc::new(collectInitialEqns) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)) -> Result<(Arc<BackendDAE::Equation>, (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>))> + 'static>), (eqns.clone(), reeqns.clone())), '__try0);
        unwrap_break_err!(execStat((literal!("collectInitialEqns (initialization)")).clone()), '__try0);
        (vars, fixvars, eqns, reeqns) = unwrap_break_err!(collectInitialVarsEqnsSystem(dae.eqs.clone(), vars.clone(), fixvars.clone(), eqns.clone(), reeqns.clone(), hs.clone(), allPrimaryParameters.clone(), datarecon.clone()), '__try0);
        (eqns, reeqns) = unwrap_break_err!(BackendVariable::traverseBackendDAEVars(vars.clone(), (std::sync::Arc::new(collectInitialBindings) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)) -> Result<(BackendDAE::Var, (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>))> + 'static>), (eqns.clone(), reeqns.clone())), '__try0);
        unwrap_break_err!(execStat((literal!("collectInitialBindings (initialization)")).clone()), '__try0);
        eqnsLst = BackendEquation::equationList(eqns.clone());
        reeqnsLst = BackendEquation::equationList(reeqns.clone());
        (_, eqnsLst, reeqnsLst, _) = unwrap_break_err!(BackendDAECreate::patchRecordBindings(metamodelica::nil(), metamodelica::nil(), BackendVariable::varList(dae.shared.globalKnownVars.clone())?, eqnsLst.clone(), reeqnsLst.clone(), metamodelica::nil()), '__try0);
        eqns = unwrap_break_err!(BackendEquation::listEquation(eqnsLst.clone()), '__try0);
        reeqns = unwrap_break_err!(BackendEquation::listEquation(reeqnsLst.clone()), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::NF_SCALARIZE.clone()), '__try0) {
            vars = unwrap_break_err!(BackendVariable::scalarizeVariables(vars.clone()), '__try0);
            initVars = unwrap_break_err!(BackendVariable::scalarizeVariables(initVars.clone()), '__try0);
        }
        useHomotopy = unwrap_break_err!(BackendDAEUtil::traverseBackendDAEExpsEqns(eqns.clone(), (std::sync::Arc::new(simplifyInitialFunctions) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false), '__try0);
        unwrap_break_err!(execStat((literal!("simplifyInitialFunctions (initialization)")).clone()), '__try0);
        vars = unwrap_break_err!(BackendVariable::rehashVariables(vars.clone()), '__try0);
        fixvars = unwrap_break_err!(BackendVariable::rehashVariables(fixvars.clone()), '__try0);
        shared = unwrap_break_err!(BackendDAEUtil::createEmptyShared(crate::BackendDAE::BackendDAEType::INITIALSYSTEM, dae.shared.info.clone(), dae.shared.cache.clone(), dae.shared.graph.clone()), '__try0);
        shared = unwrap_break_err!(BackendDAEUtil::setSharedRemovedEqns(shared.clone(), BackendEquation::emptyEqns()), '__try0);
        shared = BackendDAEUtil::setSharedGlobalKnownVars(shared.clone(), fixvars.clone());
        shared = unwrap_break_err!(BackendDAEUtil::setSharedOptimica(shared.clone(), dae.shared.constraints.clone(), dae.shared.classAttrs.clone()), '__try0);
        shared = unwrap_break_err!(BackendDAEUtil::setSharedFunctionTree(shared.clone(), dae.shared.functionTree.clone()), '__try0);
        unwrap_break_err!(execStat((literal!("setup shared object (initialization)")).clone()), '__try0);
        initsyst = BackendDAEUtil::createEqSystem(vars.clone(), eqns.clone(), metamodelica::nil(), crate::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
        initsyst = BackendDAEUtil::setEqSystRemovedEqns(initsyst.clone(), reeqns.clone());
        if useHomotopy.clone() {
            initsyst0 = unwrap_break_err!(BackendDAEUtil::copyEqSystem(initsyst.clone()), '__try0);
            enabledModules = if (unwrap_break_err!(Config::adaptiveHomotopy(), '__try0)) {list![(literal!("inlineHomotopy")).clone(), (literal!("generateHomotopyComponents")).clone()]} else {metamodelica::nil()};
            disabledModules = metamodelica::nil();
        } else {
            enabledModules = metamodelica::nil();
            disabledModules = list![(literal!("inlineHomotopy")).clone(), (literal!("generateHomotopyComponents")).clone()];
        }
        (initdae, dumpVars, outRemovedInitialEquations) = unwrap_break_err!(createInitialDAEFromSystem(initsyst.clone(), shared.clone(), initVars.clone(), enabledModules.clone(), disabledModules.clone(), outGlobalKnownVars.clone(), false), '__try0);
        (outSimDAE, _) = unwrap_break_err!(BackendVariable::traverseBackendDAE(outSimDAE.clone(), (std::sync::Arc::new(updateFixedAttribute) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendDAE::Variables) -> Result<(BackendDAE::Var, BackendDAE::Variables)> + 'static>), BackendVariable::listVar(dumpVars.clone())), '__try0);
        if useHomotopy.clone() && unwrap_break_err!(Config::globalHomotopy(), '__try0) {
            initsyst0 = unwrap_break_err!(replaceHomotopyWithSimplifiedEqs(initsyst0.clone()), '__try0);
            initdae0 = unwrap_break_err!(BackendDAE::DAE(list![initsyst0.clone()], shared.clone()), '__try0);
            initdae0 = unwrap_break_err!(BackendDAEUtil::setFunctionTree(initdae0.clone(), BackendDAEUtil::getFunctions(initdae.shared.clone())?), '__try0);
            (initdae0, _, removedEqns) = unwrap_break_err!(createInitialDAEFromSystem(initsyst0.clone(), shared.clone(), initVars.clone(), metamodelica::nil(), list![(literal!("inlineHomotopy")).clone(), (literal!("generateHomotopyComponents")).clone()], outGlobalKnownVars.clone(), true), '__try0);
            outRemovedInitialEquations = listAppend(removedEqns.clone(), outRemovedInitialEquations.clone());
            assign_field!(initdae0.shared = BackendDAEUtil::setSharedGlobalKnownVars(initdae0.shared.clone(), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())));
            outInitDAE_lambda0 = Some(initdae0.clone());
            initdae = unwrap_break_err!(BackendDAEUtil::setFunctionTree(initdae.clone(), BackendDAEUtil::getFunctions(initdae0.shared.clone())?), '__try0);
        } else {
            outInitDAE_lambda0 = None;
        }
        if unwrap_break_err!(Flags::isSet(Flags::DUMP_EQNINORDER.clone()), '__try0) && unwrap_break_err!(Flags::isSet(Flags::DUMP_INITIAL_SYSTEM.clone()), '__try0) {
            unwrap_break_err!(BackendDump::dumpEqnsSolved(initdae.clone(), (literal!("initial system: eqns in order")).clone()), '__try0);
        }
        if unwrap_break_err!(Flags::isSet(Flags::ITERATION_VARS.clone()), '__try0) {
            unwrap_break_err!(BackendDAEOptimize::listAllIterationVariables(initdae.clone()), '__try0);
        }
        if unwrap_break_err!(Flags::isSet(Flags::DUMP_BACKENDDAE_INFO.clone()), '__try0) || unwrap_break_err!(Flags::isSet(Flags::DUMP_STATESELECTION_INFO.clone()), '__try0) || unwrap_break_err!(Flags::isSet(Flags::DUMP_DISCRETEVARS_INFO.clone()), '__try0) {
            unwrap_break_err!(BackendDump::dumpCompShort(initdae.clone()), '__try0);
        }
        outInitDAE = initdae.clone();
        Ok::<_, anyhow::Error>((allPrimaryParameters.clone(), dae.clone(), disabledModules.clone(), dumpVars.clone(), enabledModules.clone(), eqns.clone(), eqnsLst.clone(), fixvars.clone(), hs.clone(), initVars.clone(), initdae.clone(), initsyst.clone(), outAllPrimaryParameters.clone(), outGlobalKnownVars.clone(), outInitDAE.clone(), outInitDAE_lambda0.clone(), outRemovedInitialEquations.clone(), outSimDAE.clone(), reeqns.clone(), reeqnsLst.clone(), shared.clone(), useHomotopy.clone(), vars.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5, __try0_o6, __try0_o7, __try0_o8, __try0_o9, __try0_o10, __try0_o11, __try0_o12, __try0_o13, __try0_o14, __try0_o15, __try0_o16, __try0_o17, __try0_o18, __try0_o19, __try0_o20, __try0_o21, __try0_o22)) => {
            allPrimaryParameters = __try0_o0;
            dae = __try0_o1;
            disabledModules = __try0_o2;
            dumpVars = __try0_o3;
            enabledModules = __try0_o4;
            eqns = __try0_o5;
            eqnsLst = __try0_o6;
            fixvars = __try0_o7;
            hs = __try0_o8;
            initVars = __try0_o9;
            initdae = __try0_o10;
            initsyst = __try0_o11;
            outAllPrimaryParameters = __try0_o12;
            outGlobalKnownVars = __try0_o13;
            outInitDAE = __try0_o14;
            outInitDAE_lambda0 = __try0_o15;
            outRemovedInitialEquations = __try0_o16;
            outSimDAE = __try0_o17;
            reeqns = __try0_o18;
            reeqnsLst = __try0_o19;
            shared = __try0_o20;
            useHomotopy = __try0_o21;
            vars = __try0_o22;
        }
        Err(_) => {
            Error::addCompilerError((literal!("No system for the symbolic initialization was generated")).clone())?;
            bail!("fail");
        }
    }
    Ok((outInitDAE, outInitDAE_lambda0, outRemovedInitialEquations, outGlobalKnownVars, outSimDAE))
}

pub fn createInitialDAEFromSystem(mut inInitsyst: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut initVars: BackendDAE::Variables, mut enabledModules: Arc<metamodelica::List<ArcStr>>, mut disabledModules: Arc<metamodelica::List<ArcStr>>, mut globalKnownVars: BackendDAE::Variables, mut isLambda0: bool) -> Result<(Arc<BackendDAE::BackendDAE>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut initdae: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut dumpVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut removedEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut systemStr: ArcStr = if (isLambda0.clone()) {literal!("initialization_lambda0")} else {literal!("initialization")};
    let mut shared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut initsyst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut dumpVars2: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut initOptModules: Arc<metamodelica::List<(BackendDAEFunc::optimizationModule, ArcStr)>> = metamodelica::nil();
    let mut daeHandler: (BackendDAEFunc::StructurallySingularSystemHandlerFunc, ArcStr, BackendDAEFunc::stateDeselectionFunc, ArcStr);
    let mut matchingAlgorithm: (BackendDAEFunc::matchingAlgorithmFunc, ArcStr);
    let mut b1: bool = false;
    let mut b2: bool = false;
    let mut msg: ArcStr = arcstr::literal!("");
    (initsyst, dumpVars) = preBalanceInitialSystem(inInitsyst.clone(), initVars.clone(), isLambda0.clone())?;
    execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("preBalanceInitialSystem (")); __mm_s.push_str(&*systemStr.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
    initdae = BackendDAE::DAE(list![initsyst.clone()], shared.clone())?;
    if Flags::isSet(Flags::OPT_DAE_DUMP.clone())? {
        BackendDump::dumpBackendDAE(initdae.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("created ")); __mm_s.push_str(&*systemStr.clone()); __mm_s.push_str(&*literal!(" system")); ArcStr::from(__mm_s) }).clone())?;
    }
    if Flags::isSet(Flags::PARTITION_INITIALIZATION.clone())? {
        (systs, shared) = BackendDAEOptimize::partitionIndependentBlocksHelper(initsyst.clone(), shared.clone(), Error::getNumErrorMessages(), true)?;
        initdae = BackendDAE::DAE(systs.clone(), shared.clone())?;
        execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("partitionIndependentBlocks (")); __mm_s.push_str(&*systemStr.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
    }
    if Flags::isSet(Flags::OPT_DAE_DUMP.clone())? {
        BackendDump::dumpBackendDAE(initdae.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("partitioned ")); __mm_s.push_str(&*systemStr.clone()); __mm_s.push_str(&*literal!(" system")); ArcStr::from(__mm_s) }).clone())?;
    }
    (initdae, dumpVars2, removedEqns) = analyzeInitialSystem(initdae.clone(), initVars.clone(), (std::sync::Arc::new(balanceInitialSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32, BackendDAE::Variables, DoubleEnded::MutableList<BackendDAE::Var>, DoubleEnded::MutableList<Arc<BackendDAE::Equation>>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32)> + 'static>))?;
    dumpVars = listAppend(dumpVars.clone(), dumpVars2.clone());
    execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("analyzeInitialSystem (")); __mm_s.push_str(&*systemStr.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
    if Flags::isSet(Flags::DUMP_INITIAL_SYSTEM.clone())? {
        BackendDump::dumpBackendDAE(initdae.clone(), (systemStr.clone()).clone())?;
    }
    initdae = BackendDAEUtil::mapEqSystem(initdae.clone(), (std::sync::Arc::new(solveInitialSystemEqSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> + 'static>))?;
    execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("solveInitialSystemEqSystem (")); __mm_s.push_str(&*systemStr.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
    initdae = BackendDAEUtil::transformBackendDAE(initdae.clone(), Some((crate::BackendDAE::IndexReduction::NO_INDEX_REDUCTION, crate::BackendDAE::EquationConstraints::EXACT)), None, None)?;
    execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("matching and sorting (n=")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", BackendDAEUtil::daeSize(initdae.clone())))); __mm_s.push_str(&*literal!(") (")); __mm_s.push_str(&*systemStr.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
    initdae = BackendDAEOptimize::addInitialStmtsToAlgorithms(initdae.clone(), true)?;
    initdae = BackendDAEUtil::setDAEGlobalKnownVars(initdae.clone(), globalKnownVars.clone())?;
    initOptModules = BackendDAEUtil::getInitOptModules(None, enabledModules.clone(), disabledModules.clone())?;
    matchingAlgorithm = BackendDAEUtil::getMatchingAlgorithm(None)?;
    daeHandler = BackendDAEUtil::getIndexReductionMethod(Some((literal!("none")).clone()))?;
    initdae = BackendDAEUtil::postOptimizeDAE(initdae.clone(), initOptModules.clone(), matchingAlgorithm.clone(), daeHandler.clone())?;
    if Flags::isSet(Flags::DUMP_INITIAL_SYSTEM.clone())? {
        BackendDump::dumpBackendDAE(initdae.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("solved ")); __mm_s.push_str(&*systemStr.clone()); ArcStr::from(__mm_s) }).clone())?;
        if Flags::isSet(Flags::ADDITIONAL_GRAPHVIZ_DUMP.clone())? {
            BackendDump::graphvizBackendDAE(initdae.clone(), (literal!("dumpinitialsystem")).clone())?;
        }
    }
    assign_field!(initdae.shared = BackendDAEUtil::setSharedGlobalKnownVars(initdae.shared.clone(), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())));
    b1 = !(dumpVars.clone().is_empty());
    b2 = !(removedEqns.clone().is_empty());
    msg = (System::gettext((literal!("For more information set -d=initialization. In OMEdit Tools->Options->Simulation->Show additional information from the initialization process, in OMNotebook call setCommandLineOptions(\"-d=initialization\")")).clone())).clone();
    if Flags::isSet(Flags::INITIALIZATION.clone())? {
        if b1.clone() {
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Assuming fixed start value for the following ")); __mm_s.push_str(&*intString((dumpVars.clone().len() as i32))); __mm_s.push_str(&*literal!(" variables:\n")); __mm_s.push_str(&*warnAboutVars2(dumpVars.clone())?); ArcStr::from(__mm_s) }).clone())?;
        }
        if b2.clone() {
            Error::addMessage(Error::INITIALIZATION_OVER_SPECIFIED.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The following ")); __mm_s.push_str(&*intString((removedEqns.clone().len() as i32))); __mm_s.push_str(&*literal!(" initial equations are redundant, so they are removed from the ")); __mm_s.push_str(&*systemStr.clone()); __mm_s.push_str(&*literal!(" system:\n")); __mm_s.push_str(&*warnAboutEqns2(removedEqns.clone())?); ArcStr::from(__mm_s) }).clone()])?;
        }
    } else {
        if b1.clone() {
            Error::addMessage(Error::INITIALIZATION_NOT_FULLY_SPECIFIED.clone(), list![(msg.clone()).clone()])?;
        }
        if b2.clone() {
            Error::addMessage(Error::INITIALIZATION_OVER_SPECIFIED.clone(), list![(msg.clone()).clone()])?;
        }
    }
    Ok((initdae, dumpVars, removedEqns))
}

// =============================================================================
// section for helper functions of solveInitialSystem
//
// =============================================================================
fn solveInitialSystemEqSystem(mut isyst: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> {
    let mut osyst: Arc<BackendDAE::EqSystem> = isyst.clone();
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut nVars: i32 = 0;
    let mut nEqns: i32 = 0;
    nEqns = BackendDAEUtil::systemSize(isyst.clone());
    nVars = BackendVariable::varsSize(BackendVariable::daeVars(isyst.clone()))?;
    if intGt(nEqns.clone(), nVars.clone()) {
        if Flags::isSet(Flags::INITIALIZATION.clone())? {
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("It was not possible to solve the over-determined initial system (")); __mm_s.push_str(&*intString(nEqns.clone())); __mm_s.push_str(&*literal!(" equations and ")); __mm_s.push_str(&*intString(nVars.clone())); __mm_s.push_str(&*literal!(" variables)")); ArcStr::from(__mm_s) }).clone())?;
            BackendDump::dumpEqSystem(isyst.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("It was not possible to solve the over-determined initial system (")); __mm_s.push_str(&*intString(nEqns.clone())); __mm_s.push_str(&*literal!(" equations and ")); __mm_s.push_str(&*intString(nVars.clone())); __mm_s.push_str(&*literal!(" variables)")); ArcStr::from(__mm_s) }).clone())?;
        }
        bail!("fail");
    }
    if intLt(nEqns.clone(), nVars.clone()) {
        if Flags::isSet(Flags::INITIALIZATION.clone())? {
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("It was not possible to solve the under-determined initial system (")); __mm_s.push_str(&*intString(nEqns.clone())); __mm_s.push_str(&*literal!(" equations and ")); __mm_s.push_str(&*intString(nVars.clone())); __mm_s.push_str(&*literal!(" variables)")); ArcStr::from(__mm_s) }).clone())?;
            BackendDump::dumpEqSystem(isyst.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("It was not possible to solve the under-determined initial system (")); __mm_s.push_str(&*intString(nEqns.clone())); __mm_s.push_str(&*literal!(" equations and ")); __mm_s.push_str(&*intString(nVars.clone())); __mm_s.push_str(&*literal!(" variables)")); ArcStr::from(__mm_s) }).clone())?;
        }
        bail!("fail");
    }
    Ok((osyst, outShared))
}

// =============================================================================
// section for inlining when-clauses
//
// This section contains all the helper functions to replace all when-clauses
// from a given BackendDAE to get the initial equation system.
// =============================================================================
fn inlineWhenForInitialization(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = inDAE.clone();
    let mut eqnlst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut clockEqnsLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut leftCrs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)) = HashSet::emptyHashSet();
    assign_field!(outDAE.eqs = List::map(inDAE.eqs.clone(), (std::sync::Arc::new(inlineWhenForInitializationSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> + 'static>)));
    (eqnlst, _) = BackendEquation::traverseEquationArray(inDAE.shared.removedEqs.clone(), (std::sync::Arc::new(inlineWhenForInitializationEquation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)))) -> Result<(Arc<BackendDAE::Equation>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))))> + 'static>), (metamodelica::nil(), leftCrs.clone()))?;
    clockEqnsLst = BackendEquation::traverseEquationArray(inDAE.shared.removedEqs.clone(), (std::sync::Arc::new(fnptr!(SynchronousFeatures::getBoolClockWhenClauses, Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> + 'static>), metamodelica::nil())?;
    eqnlst = listAppend(clockEqnsLst.clone(), eqnlst.clone());
    assign_field!(outDAE.shared = BackendDAEUtil::setSharedRemovedEqns(outDAE.shared.clone(), BackendEquation::listEquation(eqnlst.clone())?)?);
    Ok(outDAE)
}

fn inlineWhenForInitializationSystem(mut inEqSystem: Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut outEqSystem: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut eqnlst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut leftCrs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)) = HashSet::emptyHashSet();
    let mut crefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    (eqnlst, leftCrs) = BackendEquation::traverseEquationArray(inEqSystem.orderedEqs.clone(), (std::sync::Arc::new(inlineWhenForInitializationEquation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)))) -> Result<(Arc<BackendDAE::Equation>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))))> + 'static>), (metamodelica::nil(), leftCrs.clone()))?;
    crefLst = BaseHashSet::hashSetList(leftCrs.clone())?;
    eqnlst = generateInactiveWhenEquationForInitialization(crefLst.clone(), DAE::emptyElementSource().clone(), eqnlst.clone())?;
    outEqSystem = BackendDAEUtil::setEqSystEqs(inEqSystem.clone(), BackendEquation::listEquation(eqnlst.clone())?);
    outEqSystem = BackendDAEUtil::clearEqSyst(outEqSystem.clone())?;
    Ok(outEqSystem)
}

fn inlineWhenForInitializationEquation(mut inEq: Arc<BackendDAE::Equation>, mut inTpl: (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)))) -> Result<(Arc<BackendDAE::Equation>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))))> {
    let mut outEq: Arc<BackendDAE::Equation> = inEq.clone();
    let mut outTpl: (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)));
    let mut eqAttr: BackendDAE::EquationAttributes = <BackendDAE::EquationAttributes as ::std::default::Default>::default();
    let mut weqn: Arc<BackendDAE::WhenEquation> = Arc::new(<BackendDAE::WhenEquation as ::std::default::Default>::default());
    let mut alg: Arc<DAE::Algorithm> = Arc::new(<DAE::Algorithm as ::std::default::Default>::default());
    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    let mut crefExpand: DAE::Expand = DAE::Expand::EXPAND;
    let mut leftCrs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    let mut size: i32 = 0;
    let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut accEq: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    (accEq, leftCrs) = inTpl.clone();
    outTpl = (::match_deref::match_deref! { match &(inEq.clone()) {
        Deref @ BackendDAE::Equation::WHEN_EQUATION { attr: eqAttr, source, whenEquation: weqn, .. } => {
            (leftCrs, eqns) = inlineWhenForInitializationWhenEquation(weqn.clone(), source.clone(), eqAttr.clone(), accEq.clone(), leftCrs.clone())?;
            (eqns.clone(), leftCrs.clone())
        },
        Deref @ BackendDAE::Equation::ALGORITHM { expand: crefExpand, source, alg, .. } => {
            let mut alg = (*alg).clone();
            let __pa0 = ::match_deref::match_deref! { match &(alg.clone()) {
                Deref @ DAE::Algorithm { statementLst: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            stmts = __pa0.clone();
            (stmts, leftCrs) = inlineWhenForInitializationWhenAlgorithm(stmts.clone(), metamodelica::nil(), leftCrs.clone())?;
            alg = Arc::new(DAE::Algorithm { statementLst: stmts.clone() });
            size = (CheckModel::checkAndGetAlgorithmOutputs(alg.clone(), source.clone(), crefExpand.clone())?.len() as i32);
            eqns = List::consOnTrue(!(stmts.clone().is_empty()), Arc::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: alg.clone(), source: source.clone(), expand: crefExpand.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() }), accEq.clone());
            (eqns.clone(), leftCrs.clone())
        },
        _ => (cons(inEq.clone(), accEq.clone()), leftCrs.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outEq, outTpl))
}

fn inlineWhenForInitializationWhenEquation(mut inWEqn: Arc<BackendDAE::WhenEquation>, mut inSource: Arc<DAE::ElementSource>, mut inEqAttr: BackendDAE::EquationAttributes, mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inLeftCrs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut outLeftCrs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)) = inLeftCrs.clone();
    let mut outEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = inEqns.clone();
    let mut lhs: Arc<DAE::Exp>;
    let mut condition: Arc<DAE::Exp>;
    let mut e: Arc<DAE::Exp>;
    let mut eLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut whenStmtLst: Arc<metamodelica::List<BackendDAE::WhenOperator>> = metamodelica::nil();
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut crefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut active: bool = false;
    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    outEqns = (::match_deref::match_deref! { match &(inWEqn.clone()) {
        Deref @ BackendDAE::WhenEquation { whenStmtLst, condition, .. } => {
            active = Expression::containsInitialCall(condition.clone());
            for mut stmt in &*whenStmtLst.clone() {
                let mut stmt = stmt.clone();
                let _ = (::match_deref::match_deref! { match &(stmt.clone()) {
        BackendDAE::WhenOperator::ASSIGN { right: e, left: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, .. } => {
            if active.clone() {
                lhs = Expression::crefExp(cr.clone())?;
                eqn = BackendEquation::generateEquation(lhs.clone(), e.clone(), inSource.clone(), inEqAttr.clone())?;
                outEqns = cons(eqn.clone(), outEqns.clone());
            } else {
                outLeftCrs = List::fold(ComponentReference::expandCref(cr.clone(), true)?, (std::sync::Arc::new(BaseHashSet::add) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), outLeftCrs.clone());
            }
            ()
        },
        BackendDAE::WhenOperator::ASSIGN { right: e, left: lhs @ Deref @ DAE::Exp::TUPLE { PR: eLst }, .. } => {
            if active.clone() {
                eqn = BackendEquation::generateEquation(lhs.clone(), e.clone(), inSource.clone(), inEqAttr.clone())?;
                outEqns = cons(eqn.clone(), outEqns.clone());
            } else {
                crefLst = List::flatten(List::map(eLst.clone(), (std::sync::Arc::new(Expression::getAllCrefs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> + 'static>)));
                for mut cr in &*crefLst.clone() {
                    let mut cr = cr.clone();
                    outLeftCrs = List::fold(ComponentReference::expandCref(cr.clone(), true)?, (std::sync::Arc::new(BaseHashSet::add) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), outLeftCrs.clone());
                }
            }
            ()
        },
        BackendDAE::WhenOperator::NORETCALL { source, exp: e } => {
            if active.clone() {
                eqn = Arc::new(BackendDAE::Equation::ALGORITHM { size: 0, alg: Arc::new(DAE::Algorithm { statementLst: list![Arc::new(DAE::Statement::STMT_NORETCALL { exp: e.clone(), source: source.clone() })] }), source: inSource.clone(), expand: openmodelica_frontend_types::DAE::Expand::EXPAND, attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                outEqns = cons(eqn.clone(), outEqns.clone());
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            outEqns.clone()
        },
        _ => outEqns.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outLeftCrs, outEqns))
}

fn inlineWhenForInitializationWhenAlgorithm(mut inStmts: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inAcc: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inLeftCrs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(Arc<metamodelica::List<Arc<DAE::Statement>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)))> {
    let mut outStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut outLeftCrs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    (outStmts, outLeftCrs) = (::match_deref::match_deref! { match &(inStmts.clone()) {
        Deref @ metamodelica::List::Nil => {
            (inAcc.clone().reverse(), inLeftCrs.clone())
        },
        Deref @ metamodelica::List::Cons { head: stmt @ Deref @ DAE::Statement::STMT_WHEN { .. }, tail: rest } => {
            let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut leftCrs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            (stmts, leftCrs) = inlineWhenForInitializationWhenStmt(stmt.clone(), inLeftCrs.clone(), inAcc.clone())?;
            (stmts, leftCrs) = inlineWhenForInitializationWhenAlgorithm(rest.clone(), stmts.clone(), leftCrs.clone())?;
            (stmts.clone(), leftCrs.clone())
        },
        Deref @ metamodelica::List::Cons { head: stmt, tail: rest } => {
            let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
            let mut leftCrs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            (stmts, leftCrs) = inlineWhenForInitializationWhenAlgorithm(rest.clone(), cons(stmt.clone(), inAcc.clone()), inLeftCrs.clone())?;
            (stmts.clone(), leftCrs.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outStmts, outLeftCrs))
}

fn inlineWhenForInitializationWhenStmt(mut inWhenStatement: Arc<DAE::Statement>, mut inLeftCrs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), mut inAcc: Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<(Arc<metamodelica::List<Arc<DAE::Statement>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)))> {
    let mut outStmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    let mut outLeftCrs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    (outStmts, outLeftCrs) = (::match_deref::match_deref! { match &(inWhenStatement.clone()) {
        Deref @ DAE::Statement::STMT_WHEN { statementLst: stmts, exp: condition, .. } if (Expression::containsInitialCall(condition.clone())) => {
            let mut stmts = (*stmts).clone();
            stmts = List::foldr(stmts.clone(), std::sync::Arc::new(fnptr!(List::consr, _, _)), inAcc.clone());
            (stmts.clone(), inLeftCrs.clone())
        },
        Deref @ DAE::Statement::STMT_WHEN { elseWhen: None, statementLst: stmts, .. } => {
            let mut crefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut leftCrs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            crefLst = CheckModel::algorithmStatementListOutputs(stmts.clone(), openmodelica_frontend_types::DAE::Expand::EXPAND)?;
            leftCrs = List::fold(crefLst.clone(), (std::sync::Arc::new(BaseHashSet::add) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), inLeftCrs.clone());
            (inAcc.clone(), leftCrs.clone())
        },
        Deref @ DAE::Statement::STMT_WHEN { elseWhen: Some(stmt), statementLst: stmts, .. } => {
            let mut crefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut leftCrs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut stmts = (*stmts).clone();
            crefLst = CheckModel::algorithmStatementListOutputs(stmts.clone(), openmodelica_frontend_types::DAE::Expand::EXPAND)?;
            leftCrs = List::fold(crefLst.clone(), (std::sync::Arc::new(BaseHashSet::add) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), inLeftCrs.clone());
            (stmts, leftCrs) = inlineWhenForInitializationWhenStmt(stmt.clone(), leftCrs.clone(), inAcc.clone())?;
            (stmts.clone(), leftCrs.clone())
        },
        _ => {
            Error::addInternalError((literal!("function inlineWhenForInitializationWhenStmt failed")).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outStmts, outLeftCrs))
}

fn generateInactiveWhenEquationForInitialization(mut inCrLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inSource: Arc<DAE::ElementSource>, mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut outEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = inEqns.clone();
    let mut identType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut crefExp: Arc<DAE::Exp>;
    let mut crefPreExp: Arc<DAE::Exp>;
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    for mut cr in &*inCrLst.clone() {
        let mut cr = cr.clone();
        identType = ComponentReference::crefTypeConsiderSubs(cr.clone())?;
        crefExp = Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: identType.clone() });
        crefPreExp = Expression::makePureBuiltinCall((literal!("pre")).clone(), list![crefExp.clone()], DAE::T_BOOL_DEFAULT().clone());
        eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: crefExp.clone(), scalar: crefPreExp.clone(), source: inSource.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
        outEqns = cons(eqn.clone(), outEqns.clone());
    }
    Ok(outEqns)
}

// =============================================================================
// section for collecting all variables, of which the left limit is also used.
//
// collect all pre variables in time equations
// =============================================================================
fn collectPreVariables(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))> {
    let mut outHS: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    outHS = List::fold(inDAE.eqs.clone(), (std::sync::Arc::new(collectPreVariablesEqSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))> + 'static>), HashSet::emptyHashSet());
    (_, outHS) = BackendDAEUtil::traverseBackendDAEExpsEqns(inDAE.shared.initialEqs.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(collectPreVariablesTraverseExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)))> + 'static>), outHS.clone()))?;
    (_, outHS) = BackendDAEUtil::traverseBackendDAEExpsEqns(inDAE.shared.removedEqs.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(collectPreVariablesTraverseExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)))> + 'static>), outHS.clone()))?;
    Ok(outHS)
}

pub fn collectPreVariablesEqSystem(mut inSyst: Arc<BackendDAE::EqSystem>, mut inHS: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))> {
    let mut outHS: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    (_, outHS) = BackendDAEUtil::traverseBackendDAEExpsEqns(inSyst.orderedEqs.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(collectPreVariablesTraverseExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)))> + 'static>), inHS.clone()))?;
    (_, outHS) = BackendDAEUtil::traverseBackendDAEExpsEqns(inSyst.removedEqs.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(collectPreVariablesTraverseExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)))> + 'static>), outHS.clone()))?;
    Ok(outHS)
}

pub fn collectPreVariablesTraverseExp(mut inExp: Arc<DAE::Exp>, mut inHS: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)))> {
    let mut outExp: Arc<DAE::Exp> = inExp.clone();
    let mut outHS: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    outHS = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. } => {
            (_, outHS) = Expression::traverseExpBottomUp(inExp.clone(), (std::sync::Arc::new(collectPreVariablesTraverseExp2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)))> + 'static>), inHS.clone())?;
            outHS.clone()
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "change" }, .. } => {
            (_, outHS) = Expression::traverseExpBottomUp(inExp.clone(), (std::sync::Arc::new(collectPreVariablesTraverseExp2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)))> + 'static>), inHS.clone())?;
            outHS.clone()
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "edge" }, .. } => {
            (_, outHS) = Expression::traverseExpBottomUp(inExp.clone(), (std::sync::Arc::new(collectPreVariablesTraverseExp2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)))> + 'static>), inHS.clone())?;
            outHS.clone()
        },
        _ => inHS.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outHS))
}

fn collectPreVariablesTraverseExp2(mut inExp: Arc<DAE::Exp>, mut inHS: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)))> {
    let mut outExp: Arc<DAE::Exp> = inExp.clone();
    let mut outHS: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    outHS = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            crefs = ComponentReference::expandCref(cr.clone(), true)?;
            outHS = List::fold(crefs.clone(), (std::sync::Arc::new(BaseHashSet::add) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), inHS.clone());
            outHS.clone()
        },
        _ => {
            inHS.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outHS))
}

fn warnAboutVars2(mut vars: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut len: i32 = 0;
    let mut size: i32 = 0;
    if vars.clone().is_empty() {
        outString = (literal!("")).clone();
        return Ok(outString);
    }
    strs = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut v in (vars.clone()).into_iter().cloned() {
            let __x = BackendDump::varString(v.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    len = (strs.clone().len() as i32);
    size = {
        let mut __acc: i32 = 0;
        for mut s in (strs.clone()).into_iter().cloned() {
            let __x = ((s.clone()).clone().len() as i32);
            __acc += __x;
        }
        __acc
    } + len.clone() * 10;
    outString = (warnAboutVars2Work(strs.clone(), (literal!("         ")).clone(), (literal!("\n")).clone(), size.clone())).clone();
    Ok(outString)
}

fn warnAboutVars2Work(mut strs: Arc<metamodelica::List<ArcStr>>, mut prefix: ArcStr, mut suffix: ArcStr, mut size: i32) -> ArcStr {
    let mut s: ArcStr = literal!("");
    let mut sb: System::StringAllocator = System::StringAllocator(size.clone()).unwrap();
    let mut i: i32 = 0;
    for mut r#str in &*strs.clone() {
        let mut r#str = r#str.clone();
        System::stringAllocatorStringCopy(sb.clone(), (prefix.clone()).clone(), i.clone());
        i = i.clone() + ((prefix.clone()).clone().len() as i32);
        System::stringAllocatorStringCopy(sb.clone(), (r#str.clone()).clone(), i.clone());
        i = i.clone() + ((r#str.clone()).clone().len() as i32);
        System::stringAllocatorStringCopy(sb.clone(), (suffix.clone()).clone(), i.clone());
        i = i.clone() + ((suffix.clone()).clone().len() as i32);
    }
    s = (System::stringAllocatorResult(sb.clone(), (s.clone()).clone())).clone();
    s
}

fn warnAboutEqns2(mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inEqns.clone()) {
        Deref @ metamodelica::List::Nil => {
            literal!("")
        },
        Deref @ metamodelica::List::Cons { head: eq, tail: Deref @ metamodelica::List::Nil } => {
            let mut crStr: ArcStr = arcstr::literal!("");
            crStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("         ")); __mm_s.push_str(&*BackendDump::equationString(eq.clone())?); ArcStr::from(__mm_s) }).clone();
            crStr.clone()
        },
        Deref @ metamodelica::List::Cons { head: eq, tail: eqns } => {
            let mut crStr: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            crStr = (BackendDump::equationString(eq.clone())?).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("         ")); __mm_s.push_str(&*crStr.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*warnAboutEqns2(eqns.clone())?); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

// =============================================================================
// section for selecting initialization variables
//
//   - unfixed state
//   - secondary parameter
//   - unfixed discrete -> pre(vd)
// =============================================================================
fn selectInitializationVariablesDAE(mut dae: Arc<BackendDAE::BackendDAE>) -> Result<(Arc<BackendDAE::BackendDAE>, BackendDAE::Variables, Arc<metamodelica::List<BackendDAE::Var>>, BackendDAE::Variables)> {
    let mut dae: Arc<BackendDAE::BackendDAE> = dae;
    let mut outInitVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut outAllPrimaryParameters: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut outGlobalKnownVars: BackendDAE::Variables = dae.shared.globalKnownVars.clone();
    let mut otherVariables: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut globalKnownVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut globalKnownVarsEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut globalKnownVarsSystem: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut comps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut flatComps: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut nGlobalKnownVars: i32 = 0;
    let mut secondary: metamodelica::Array<i32>;
    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut bindExp: Arc<DAE::Exp>;
    let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut globalKnownVarList: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    for mut var in &*BackendVariable::varList(dae.shared.globalKnownVars.clone())? {
        let mut var = var.clone();
        if BackendVariable::isInput(var.clone()) && !(Expression::isConstValue(BackendVariable::varStartValue(var.clone())?)?) && !(Types::isArray(BackendVariable::varType(var.clone())?)) {
            bindExp = BackendVariable::varStartValue(var.clone())?;
            (v, _) = BackendVariable::getVarSingle(Expression::expCref(bindExp.clone())?, dae.shared.globalKnownVars.clone())?;
            var = BackendVariable::setVarStartValueOption(var.clone(), v.bindExp.clone())?;
        }
        globalKnownVarList = cons(var.clone(), globalKnownVarList.clone());
    }
    dae = BackendDAEUtil::setDAEGlobalKnownVars(dae.clone(), BackendVariable::listVar(globalKnownVarList.clone()))?;
    globalKnownVars = BackendVariable::listVar(BackendVariable::varList(dae.shared.globalKnownVars.clone())?);
    outInitVars = selectInitializationVariables(dae.eqs.clone());
    outInitVars = BackendVariable::traverseBackendDAEVars(dae.shared.globalKnownVars.clone(), (std::sync::Arc::new(selectInitializationVariables2) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendDAE::Variables) -> Result<(BackendDAE::Var, BackendDAE::Variables)> + 'static>), outInitVars.clone())?;
    outInitVars = BackendVariable::traverseBackendDAEVars(dae.shared.aliasVars.clone(), (std::sync::Arc::new(selectInitializationVariables2) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendDAE::Variables) -> Result<(BackendDAE::Var, BackendDAE::Variables)> + 'static>), outInitVars.clone())?;
    globalKnownVars = BackendVariable::traverseBackendDAEVars(dae.shared.externalObjects.clone(), (std::sync::Arc::new(addExtObjToGlobalKnownVars) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendDAE::Variables) -> Result<(BackendDAE::Var, BackendDAE::Variables)> + 'static>), globalKnownVars.clone())?;
    nGlobalKnownVars = BackendVariable::varsSize(globalKnownVars.clone())?;
    otherVariables = BackendVariable::emptyVarsSized(nGlobalKnownVars.clone());
    globalKnownVarsEqns = BackendEquation::emptyEqnsSized(nGlobalKnownVars.clone());
    globalKnownVarsEqns = BackendVariable::traverseBackendDAEVars(globalKnownVars.clone(), (std::sync::Arc::new(createGlobalKnownVarsEquations) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<(BackendDAE::Var, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)> + 'static>), globalKnownVarsEqns.clone())?;
    if nGlobalKnownVars.clone() > 0 {
        globalKnownVarsSystem = BackendDAEUtil::createEqSystem(globalKnownVars.clone(), globalKnownVarsEqns.clone(), metamodelica::nil(), crate::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
        (m, mT) = BackendDAEUtil::adjacencyMatrix(globalKnownVarsSystem.clone(), crate::BackendDAE::IndexType::NORMAL, None, BackendDAEUtil::isInitializationDAE(dae.shared.clone()))?;
        (ass1, ass2) = Matching::PerfectMatching(m.clone())?;
        comps = Sorting::Tarjan(m.clone(), ass1.clone(), (ass1.clone().borrow().len() as i32))?;
        comps = mapListIndices(comps.clone(), ass2.clone());
        flatComps = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut comp in (comps.clone()).into_iter().cloned() {
            let __x = flattenParamComp(comp.clone(), globalKnownVars.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        secondary = arrayCreate(nGlobalKnownVars.clone(), 0);
        secondary = selectSecondaryParameters(flatComps.clone(), globalKnownVars.clone(), mT.clone(), secondary.clone())?;
        hs = HashSet::emptyHashSetSized(2 * nGlobalKnownVars.clone() + 1);
        for mut i in &*flatComps.clone() {
            let mut i = i.clone();
            v = BackendVariable::getVarAt(globalKnownVars.clone(), i.clone())?;
            bindExp = BackendVariable::varBindExpStartValueNoFail(v.clone())?;
            crefs = Expression::getAllCrefsExpanded(bindExp.clone())?;
            let _ = (::match_deref::match_deref! { match &(v.clone()) {
        BackendDAE::Var { varKind: BackendDAE::VarKind::PARAM, .. } if (0 == secondary.borrow()[(i.clone()-1) as usize].clone() && BaseHashSet::hasAll(crefs.clone(), hs.clone())?) => {
            outAllPrimaryParameters = cons(v.clone(), outAllPrimaryParameters.clone());
            hs = BaseHashSet::add(BackendVariable::varCref(v.clone())?, hs.clone())?;
            ()
        },
        BackendDAE::Var { bindExp: Some(bindExp), varKind: BackendDAE::VarKind::EXTOBJ { .. }, .. } if (0 == secondary.borrow()[(i.clone()-1) as usize].clone() && BaseHashSet::hasAll(crefs.clone(), hs.clone())?) => {
            outAllPrimaryParameters = cons(v.clone(), outAllPrimaryParameters.clone());
            v = BackendVariable::setVarFixed(v.clone(), true)?;
            outGlobalKnownVars = BackendVariable::addVar(v.clone(), outGlobalKnownVars.clone())?;
            hs = BaseHashSet::add(BackendVariable::varCref(v.clone())?, hs.clone())?;
            ()
        },
        BackendDAE::Var { varKind: BackendDAE::VarKind::PARAM, .. } => {
            otherVariables = BackendVariable::addVar(v.clone(), otherVariables.clone())?;
            v = BackendVariable::setVarFixed(v.clone(), false)?;
            outInitVars = BackendVariable::addVar(v.clone(), outInitVars.clone())?;
            outGlobalKnownVars = BackendVariable::addVar(v.clone(), outGlobalKnownVars.clone())?;
            ()
        },
        _ if (BackendVariable::isVarAlg(v.clone()) && 0 == secondary.borrow()[(i.clone()-1) as usize].clone() && BaseHashSet::hasAll(crefs.clone(), hs.clone())?) => {
            otherVariables = BackendVariable::addVar(v.clone(), otherVariables.clone())?;
            v = BackendVariable::setVarFixed(v.clone(), true)?;
            v = BackendVariable::setVarFinal(v.clone(), true)?;
            outGlobalKnownVars = BackendVariable::addVar(v.clone(), outGlobalKnownVars.clone())?;
            hs = BaseHashSet::add(BackendVariable::varCref(v.clone())?, hs.clone())?;
            ()
        },
        _ if (BackendVariable::isVarAlg(v.clone())) => {
            otherVariables = BackendVariable::addVar(v.clone(), otherVariables.clone())?;
            v = BackendVariable::setVarFixed(v.clone(), false)?;
            outGlobalKnownVars = BackendVariable::addVar(v.clone(), outGlobalKnownVars.clone())?;
            ()
        },
        _ => {
            otherVariables = BackendVariable::addVar(v.clone(), otherVariables.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
        GCExt::free(secondary.clone());
        outAllPrimaryParameters = outAllPrimaryParameters.clone().reverse();
        dae = BackendDAEUtil::setDAEGlobalKnownVars(dae.clone(), otherVariables.clone())?;
    }
    Ok((dae, outInitVars, outAllPrimaryParameters, outGlobalKnownVars))
}

fn addExtObjToGlobalKnownVars(mut extObj: BackendDAE::Var, mut globalKnownVars: BackendDAE::Variables) -> Result<(BackendDAE::Var, BackendDAE::Variables)> {
    let mut extObj: BackendDAE::Var = extObj;
    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars;
    globalKnownVars = (::match_deref::match_deref! { match &(extObj.clone()) {
        BackendDAE::Var { bindExp: Some(_), varKind: BackendDAE::VarKind::EXTOBJ { .. }, .. } => {
            let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
            var = BackendVariable::setVarFixed(extObj.clone(), true)?;
            globalKnownVars = BackendVariable::addVar(var.clone(), globalKnownVars.clone())?;
            globalKnownVars.clone()
        },
        _ => {
            globalKnownVars.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((extObj, globalKnownVars))
}

fn createGlobalKnownVarsEquations(mut var: BackendDAE::Var, mut parameterEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<(BackendDAE::Var, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)> {
    let mut var: BackendDAE::Var = var;
    let mut parameterEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = parameterEqns;
    let mut lhs: Arc<DAE::Exp>;
    let mut rhs: Arc<DAE::Exp>;
    let mut startValue: Arc<DAE::Exp>;
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut s: ArcStr = arcstr::literal!("");
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    lhs = BackendVariable::varExp(var.clone())?;
    if BackendVariable::isParam(var.clone()) && !(BackendVariable::varHasBindExp(var.clone())) && BackendVariable::varFixed(var.clone()) {
        s = (ExpressionBasics::printExpStr(lhs.clone())?).clone();
        startValue = BackendVariable::varStartValue(var.clone())?;
        r#str = (ExpressionBasics::printExpStr(startValue.clone())?).clone();
        v = BackendVariable::setVarKind(var.clone(), crate::BackendDAE::VarKind::VARIABLE)?;
        v = BackendVariable::setBindExp(v.clone(), Some(startValue.clone()));
        v = BackendVariable::setVarFixed(v.clone(), true)?;
        info = ElementSource::getElementSourceFileInfo(BackendVariable::getVarSource(v.clone()));
        Error::addSourceMessage(Error::UNBOUND_PARAMETER_WITH_START_VALUE_WARNING.clone(), list![(s.clone()).clone(), (r#str.clone()).clone()], info.clone())?;
    }
    rhs = BackendVariable::varBindExpStartValueNoFail(var.clone())?;
    eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: lhs.clone(), scalar: rhs.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_BINDING.clone() });
    parameterEqns = BackendEquation::add(eqn.clone(), parameterEqns.clone())?;
    Ok((var, parameterEqns))
}

fn markIndex(mut inIndex: i32, mut inArray: metamodelica::Array<i32>) -> metamodelica::Array<i32> {
    let mut outArray: metamodelica::Array<i32> = inArray.clone();
    {
        let __cell0 = 1;
        outArray.clone().borrow_mut()[(inIndex.clone()-1) as usize] = __cell0;
    }
    outArray
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn selectSecondaryParameters(mut inOrdering: Arc<metamodelica::List<i32>>, mut inParameters: BackendDAE::Variables, mut inM: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inSecondaryParams: metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> {
    let mut outSecondaryParams: metamodelica::Array<i32>;
    outSecondaryParams = (::match_deref::match_deref! { match &(inOrdering.clone()) {
        Deref @ metamodelica::List::Nil => {
            inSecondaryParams.clone()
        },
        Deref @ metamodelica::List::Cons { head: i, tail: rest } => {
            let mut secondaryParams: metamodelica::Array<i32>;
            let mut param: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
            param = BackendVariable::getVarAt(inParameters.clone(), i.clone())?;
            secondaryParams = if (if (BackendVariable::isVarAlg(param.clone())) {false} else {!(BackendVariable::varFixed(param.clone()))} || 1 == inSecondaryParams.borrow()[(i.clone()-1) as usize].clone()) {List::fold(inM.borrow()[(i.clone()-1) as usize].clone(), (std::sync::Arc::new(fnptr!(markIndex, i32, metamodelica::Array<i32>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> + 'static>), inSecondaryParams.clone())} else {inSecondaryParams.clone()};
            secondaryParams = selectSecondaryParameters(rest.clone(), inParameters.clone(), inM.clone(), secondaryParams.clone())?;
            secondaryParams.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outSecondaryParams)
}

pub fn flattenParamComp(mut paramIndices: Arc<metamodelica::List<i32>>, mut inAllParameters: BackendDAE::Variables) -> Result<i32> {
    let mut outFlatComp: i32 = 0;
    outFlatComp = (::match_deref::match_deref! { match &(paramIndices.clone()) {
        Deref @ metamodelica::List::Cons { head: i, tail: Deref @ metamodelica::List::Nil } => {
            i.clone()
        },
        _ => {
            let mut i: i32 = 0;
            let mut paramLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut param: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
            paramLst = metamodelica::nil();
            for mut i in &*paramIndices.clone() {
                let mut i = i.clone();
                param = BackendVariable::getVarAt(inAllParameters.clone(), i.clone())?;
                paramLst = cons(param.clone(), paramLst.clone());
            }
            Error::addCompilerError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Cyclically dependent parameters found:\n")); __mm_s.push_str(&*warnAboutVars2(paramLst.clone())?); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outFlatComp)
}

fn selectInitializationVariables(mut inEqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>) -> BackendDAE::Variables {
    let mut outVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    outVars = BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone());
    outVars = List::fold(inEqSystems.clone(), (std::sync::Arc::new(selectInitializationVariables1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, BackendDAE::Variables) -> Result<BackendDAE::Variables> + 'static>), outVars.clone());
    outVars
}

fn selectInitializationVariables1(mut inEqSystem: Arc<BackendDAE::EqSystem>, mut inVars: BackendDAE::Variables) -> Result<BackendDAE::Variables> {
    let mut outVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    outVars = BackendVariable::traverseBackendDAEVars(inEqSystem.orderedVars.clone(), (std::sync::Arc::new(selectInitializationVariables2) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendDAE::Variables) -> Result<(BackendDAE::Var, BackendDAE::Variables)> + 'static>), inVars.clone())?;
    Ok(outVars)
}

fn selectInitializationVariables2(mut inVar: BackendDAE::Var, mut inVars: BackendDAE::Variables) -> Result<(BackendDAE::Var, BackendDAE::Variables)> {
    let mut outVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut outVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    (outVar, outVars) = 'mc: {
        let __mc_input = (inVar.clone(), inVars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, .. }, mut vars) = __mc_input.clone() else { bail!("nomatch") };
            let false = (BackendVariable::varFixed(inVar.clone())) else { bail!("pattern mismatch") };
            vars = BackendVariable::addVar(inVar.clone(), vars.clone())?;
            Ok((inVar.clone(), vars.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (BackendDAE::Var { arryDim: mut arryDim, varType: ref ty, varKind: BackendDAE::VarKind::DISCRETE, varName: ref cr, .. }, mut vars) = __mc_input.clone() else { bail!("nomatch") };
            let mut preVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
            let mut preCR: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let false = (BackendVariable::varFixed(inVar.clone())) else { bail!("pattern mismatch") };
            preCR = ComponentReference::crefPrefixPre(cr.clone());
            preVar = BackendDAE::Var { varName: preCR.clone(), varKind: crate::BackendDAE::VarKind::VARIABLE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: ty.clone(), bindExp: None, tplExp: None, arryDim: arryDim.clone(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
            vars = BackendVariable::addVar(preVar.clone(), vars.clone())?;
            Ok((inVar.clone(), vars.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok((inVar.clone(), inVars.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVar, outVars))
}

// =============================================================================
// section for simplifying initial functions
//
// =============================================================================
fn simplifyInitialFunctions(mut inExp: Arc<DAE::Exp>, mut inUseHomotopy: bool) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outUseHomotopy: bool = false;
    (outExp, outUseHomotopy) = Expression::traverseExpBottomUp(inExp.clone(), (std::sync::Arc::new(fnptr!(simplifyInitialFunctionsExp, Arc<DAE::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), inUseHomotopy.clone())?;
    Ok((outExp, outUseHomotopy))
}

fn simplifyInitialFunctionsExp(mut inExp: Arc<DAE::Exp>, mut inUseHomotopy: bool) -> (Arc<DAE::Exp>, bool) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outUseHomotopy: bool = false;
    (outExp, outUseHomotopy) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initial" }, .. } => {
            (Arc::new(DAE::Exp::BCONST { bool: true }), inUseHomotopy.clone())
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, .. } => {
            (Arc::new(DAE::Exp::BCONST { bool: false }), inUseHomotopy.clone())
        },
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: expr, tail: _ } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "delay" }, .. } => {
            (expr.clone(), inUseHomotopy.clone())
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "homotopy" }, .. } => {
            (inExp.clone(), true)
        },
        _ => {
            (inExp.clone(), inUseHomotopy.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outUseHomotopy)
}

// =============================================================================
// section for pre-balancing the initial system
//
// This section removes unused pre variables and auto-fixes non-pre variables,
// which occur in no equation.
// =============================================================================
fn preBalanceInitialSystem(mut inEqSystem: Arc<BackendDAE::EqSystem>, mut initVars: BackendDAE::Variables, mut isLambda0: bool) -> Result<(Arc<BackendDAE::EqSystem>, Arc<metamodelica::List<BackendDAE::Var>>)> {
    let mut outEqSystem: Arc<BackendDAE::EqSystem> = inEqSystem.clone();
    let mut outDumpVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut orderedVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut orderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut b: bool = false;
    let mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    (_, mt) = BackendDAEUtil::adjacencyMatrix(inEqSystem.clone(), crate::BackendDAE::IndexType::NORMAL, None, true)?;
    (orderedVars, orderedEqs, b, outDumpVars) = preBalanceInitialSystem1((mt.clone().borrow().len() as i32), mt.clone(), inEqSystem.orderedVars.clone(), inEqSystem.orderedEqs.clone(), initVars.clone(), isLambda0.clone(), false, metamodelica::nil())?;
    if b.clone() {
        assign_field!(
            outEqSystem.orderedEqs = orderedEqs.clone(),
            outEqSystem.orderedVars = orderedVars.clone()
        );
        outEqSystem = BackendDAEUtil::clearEqSyst(outEqSystem.clone())?;
    }
    Ok((outEqSystem, outDumpVars))
}

fn preBalanceInitialSystem1(mut n: i32, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inVars: BackendDAE::Variables, mut inEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut initVars: BackendDAE::Variables, mut isLambda0: bool, mut inB: bool, mut inDumpVars: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<(BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, bool, Arc<metamodelica::List<BackendDAE::Var>>)> {
    let mut outVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut outEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut outB: bool = false;
    let mut outDumpVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    (outVars, outEqs, outB, outDumpVars) = (match (n.clone(), inB.clone()) {
        (0, false) => {
            (inVars.clone(), inEqs.clone(), false, inDumpVars.clone())
        },
        (0, true) => {
            let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
            vars = BackendVariable::listVar1(BackendVariable::varList(inVars.clone())?);
            (vars.clone(), inEqs.clone(), true, inDumpVars.clone())
        },
        _ => {
            let mut b: bool = false;
            let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
            let mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
            let mut dumpVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let true = (n.clone() > 0) else { bail!("pattern mismatch") };
            (vars, eqs, b, dumpVars) = preBalanceInitialSystem2(n.clone(), mt.clone(), inVars.clone(), inEqs.clone(), initVars.clone(), isLambda0.clone(), inB.clone(), inDumpVars.clone())?;
            (vars, eqs, b, dumpVars) = preBalanceInitialSystem1(n.clone() - 1, mt.clone(), vars.clone(), eqs.clone(), initVars.clone(), isLambda0.clone(), b.clone(), dumpVars.clone())?;
            (vars.clone(), eqs.clone(), b.clone(), dumpVars.clone())
        },
    });
    Ok((outVars, outEqs, outB, outDumpVars))
}

fn preBalanceInitialSystem2(mut n: i32, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inVars: BackendDAE::Variables, mut inEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut initVars: BackendDAE::Variables, mut isLambda0: bool, mut inB: bool, mut inDumpVars: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<(BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, bool, Arc<metamodelica::List<BackendDAE::Var>>)> {
    let mut outVars: BackendDAE::Variables = inVars.clone();
    let mut outEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = inEqs.clone();
    let mut outB: bool = inB.clone();
    let mut outDumpVars: Arc<metamodelica::List<BackendDAE::Var>> = inDumpVars.clone();
    let mut row: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut err_str: ArcStr = literal!(" with unknown reason.");
    match '__try0: {
        row = mt.borrow()[(n.clone()-1) as usize].clone();
        if row.clone().is_empty() {
            outB = true;
            var = unwrap_break_err!(BackendVariable::getVarAt(inVars.clone(), n.clone()), '__try0);
            cref = unwrap_break_err!(BackendVariable::varCref(var.clone()), '__try0);
            if ComponentReference::isPreCref(cref.clone()) {
                (outVars, _) = unwrap_break_err!(BackendVariable::removeVars(list![n.clone()], inVars.clone(), metamodelica::nil()), '__try0);
            } else if BackendVariable::containsVar(var.clone(), initVars.clone()) {
                (outEqs, outDumpVars) = unwrap_break_err!(addStartValueEquations(list![var.clone()], inEqs.clone(), inDumpVars.clone()), '__try0);
            } else {
                r#str = (if (isLambda0.clone()) {literal!("lambda 0 ")} else {literal!("")}).clone();
                err_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" because variable ")); __mm_s.push_str(&*unwrap_break_err!(BackendDump::varString(var.clone()), '__try0)); __mm_s.push_str(&*literal!(" does not appear in any equation in the ")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("initial system and is not fixable.")); ArcStr::from(__mm_s) }).clone();
                break '__try0 Err::<_, _>(anyhow::anyhow!("fail"));
            }
        }
        Ok::<_, anyhow::Error>((row.clone(),))
    } {
        Ok((__try0_o0,)) => {
            row = __try0_o0;
        }
        Err(_) => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Initialization.preBalanceInitialSystem2")); __mm_s.push_str(&*literal!(" failed")); __mm_s.push_str(&*err_str.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail");
        }
    }
    Ok((outVars, outEqs, outB, outDumpVars))
}

fn analyzeInitialSystem(mut inInitDAE: Arc<BackendDAE::BackendDAE>, mut inInitVars: BackendDAE::Variables, mut func: Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32, BackendDAE::Variables, DoubleEnded::MutableList<BackendDAE::Var>, DoubleEnded::MutableList<Arc<BackendDAE::Equation>>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32)> + 'static>) -> Result<(Arc<BackendDAE::BackendDAE>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut outDumpVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut outRemovedEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut dae: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut dumpVars: DoubleEnded::MutableList<BackendDAE::Var>;
    let mut removedEqns: DoubleEnded::MutableList<Arc<BackendDAE::Equation>>;
    let mut filtered_initial_eqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    eqs = metamodelica::nil();
    dumpVars = DoubleEnded::fromList(metamodelica::nil())?;
    removedEqns = DoubleEnded::fromList(metamodelica::nil())?;
    for mut syst in &*inInitDAE.eqs.clone() {
        let mut syst = syst.clone();
        if BackendDAEUtil::nonEmptySystem(syst.clone())? {
            eqs = cons(syst.clone(), eqs.clone());
        } else {
            filtered_initial_eqs = {
        let mut __acc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
        for mut eqn in (BackendEquation::equationList(syst.orderedEqs.clone())).into_iter().cloned() {
            if !(BackendEquation::hasAnyUnknown(eqn.clone(), inInitVars.clone())?) { continue; }
            let __x = eqn.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            DoubleEnded::push_list_back(removedEqns.clone(), filtered_initial_eqs.clone());
            DoubleEnded::push_list_back(removedEqns.clone(), BackendEquation::equationList(syst.removedEqs.clone()));
        }
    }
    dae = BackendDAE::DAE(eqs.clone(), inInitDAE.shared.clone())?;
    (outDAE, _) = BackendDAEUtil::mapEqSystemAndFold(dae.clone(), Arc::new({ let __pe_b3 = inInitVars.clone(); let __pe_b4 = dumpVars.clone(); let __pe_b5 = removedEqns.clone(); move |__pe_a0, __pe_a1, __pe_a2| func(__pe_a0, __pe_a1, __pe_a2, __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone()) }), 0)?;
    outRemovedEqns = DoubleEnded::toListAndClear(removedEqns.clone(), metamodelica::nil());
    outDumpVars = DoubleEnded::toListAndClear(dumpVars.clone(), metamodelica::nil());
    Ok((outDAE, outDumpVars, outRemovedEqns))
}

fn getInitEqIndices(mut equations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut indices: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut i: i32 = 1;
    for mut eq in &*equations.clone() {
        let mut eq = eq.clone();
        if BackendEquation::isInitialEquation(eq.clone())? {
            indices = cons(i.clone(), indices.clone());
        }
        i = i.clone() + 1;
    }
    indices = indices.clone().reverse();
    Ok(indices)
}

type constraintHandlerFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32, BackendDAE::Variables, DoubleEnded::MutableList<BackendDAE::Var>, DoubleEnded::MutableList<Arc<BackendDAE::Equation>>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32)> + 'static>;

fn balanceInitialSystem(mut inEqSystem: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut dummy: i32, mut initVars: BackendDAE::Variables, mut dumpVars: DoubleEnded::MutableList<BackendDAE::Var>, mut removedEqns: DoubleEnded::MutableList<Arc<BackendDAE::Equation>>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32)> {
    let mut outEqSystem: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut dummy: i32 = dummy;
    let mut debug: bool = false;
    let mut eqn_lst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut init_eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut sim_eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut me: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
    let mut nVars: i32 = 0;
    let mut nEqns: i32 = 0;
    let mut scal_to_arr: metamodelica::Array<i32>;
    let mut var_to_eqn: metamodelica::Array<i32>;
    let mut eqn_to_var: metamodelica::Array<i32>;
    let mut changed: bool = false;
    let mut comps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut redundantEqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut unfixedVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut initASSC: bool = Flags::getConfigBool(Flags::INIT_ASSC.clone())?;
    if BackendVariable::varsSize(inEqSystem.orderedVars.clone())? > 0 {
        (init_eqns, sim_eqns) = List::splitOnTrue(BackendEquation::equationList(inEqSystem.orderedEqs.clone()), (std::sync::Arc::new(BackendEquation::isInitialEquation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<bool> + 'static>));
        outEqSystem = BackendDAEUtil::createEqSystem(BackendVariable::sortInitialVars(inEqSystem.orderedVars.clone(), initVars.clone())?, BackendEquation::listEquation(sim_eqns.clone())?, metamodelica::nil(), crate::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
        assign_field!(outEqSystem.removedEqs = inEqSystem.removedEqs.clone());
        funcs = BackendDAEUtil::getFunctions(inShared.clone())?;
        (outEqSystem, m, mT, _, scal_to_arr) = BackendDAEUtil::getAdjacencyMatrixScalar(outEqSystem.clone(), crate::BackendDAE::IndexType::SOLVABLE, Some(funcs.clone()), true)?;
        nVars = BackendVariable::varsSize(outEqSystem.orderedVars.clone())?;
        nEqns = BackendEquation::equationArraySize(inEqSystem.orderedEqs.clone())?;
        (eqn_to_var, var_to_eqn, _, _, _) = Matching::RegularMatching(mT.clone(), nEqns.clone(), nVars.clone())?;
        assign_field!(outEqSystem.orderedEqs = BackendEquation::addList(init_eqns.clone(), outEqSystem.orderedEqs.clone())?);
        (outEqSystem, m, mT, _, scal_to_arr) = BackendDAEUtil::getAdjacencyMatrixScalar(outEqSystem.clone(), crate::BackendDAE::IndexType::SOLVABLE, Some(funcs.clone()), true)?;
        (eqn_to_var, var_to_eqn, _, _, _) = Matching::ContinueMatching(mT.clone(), nEqns.clone(), nVars.clone(), eqn_to_var.clone(), var_to_eqn.clone(), false)?;
        unfixedVars = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (1..=(var_to_eqn.clone().borrow().len() as i32)).into_iter() {
            if !(var_to_eqn.borrow()[(i.clone()-1) as usize].clone() < 0) { continue; }
            let __x = i.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        redundantEqns = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (1..=(eqn_to_var.clone().borrow().len() as i32)).into_iter() {
            if !(eqn_to_var.borrow()[(i.clone()-1) as usize].clone() < 0) { continue; }
            let __x = i.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        if !(redundantEqns.clone().is_empty() && unfixedVars.clone().is_empty()) {
            (me, _, _, _) = BackendDAEUtil::getAdjacencyMatrixEnhancedScalar(outEqSystem.clone(), inShared.clone(), false)?;
            (_, _, _) = consistencyCheck(redundantEqns.clone(), outEqSystem.orderedEqs.clone(), outEqSystem.orderedVars.clone(), inShared.clone(), 0, m.clone(), me.clone(), var_to_eqn.clone(), eqn_to_var.clone(), scal_to_arr.clone())?;
            redundantEqns = List::unique({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (redundantEqns.clone()).into_iter().cloned() {
            let __x = scal_to_arr.borrow()[(i.clone()-1) as usize].clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            outEqSystem = resolveOverAndUnderconstraints(outEqSystem.clone(), initVars.clone(), unfixedVars.clone(), redundantEqns.clone(), dumpVars.clone(), removedEqns.clone())?;
            (outEqSystem, m, mT, _, scal_to_arr) = BackendDAEUtil::getAdjacencyMatrixScalar(outEqSystem.clone(), crate::BackendDAE::IndexType::SOLVABLE, Some(funcs.clone()), true)?;
            nVars = BackendVariable::varsSize(outEqSystem.orderedVars.clone())?;
            nEqns = BackendEquation::equationArraySize(outEqSystem.orderedEqs.clone())?;
            (eqn_to_var, var_to_eqn, _, _, _) = Matching::RegularMatching(mT.clone(), nEqns.clone(), nVars.clone())?;
        } else if !(initASSC.clone()) {
            outEqSystem = inEqSystem.clone();
        }
        if debug.clone() {
            BackendDump::dumpEqSystem(outEqSystem.clone(), (literal!("fixInitialSystem")).clone())?;
            BackendDump::dumpAdjacencyMatrixT(mT.clone());
            BackendDump::dumpMatchingVars(var_to_eqn.clone());
            BackendDump::dumpMatchingEqns(eqn_to_var.clone());
        }
        if initASSC.clone() {
            comps = Sorting::Tarjan(m.clone(), var_to_eqn.clone(), nEqns.clone())?;
            for mut comp in &*comps.clone() {
                let mut comp = comp.clone();
                (eqn_to_var, var_to_eqn, outEqSystem, changed) = BackendDAEUtil::analyticalToStructuralSingularity(comp.clone(), eqn_to_var.clone(), var_to_eqn.clone(), outEqSystem.clone(), changed.clone(), true)?;
            }
            if changed.clone() {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(outEqSystem.clone()) {
                    Deref @ BackendDAE::EqSystem { mT: Some(__pa0), m: Some(__pa1), .. } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                mT = __pa0.clone();
                m = __pa1.clone();
                (outEqSystem, m, mT, _, scal_to_arr) = BackendDAEUtil::getAdjacencyMatrixScalar(outEqSystem.clone(), crate::BackendDAE::IndexType::NORMAL, Some(funcs.clone()), true)?;
                (eqn_to_var, var_to_eqn, _, _, _) = Matching::ContinueMatching(mT.clone(), nEqns.clone(), nVars.clone(), eqn_to_var.clone(), var_to_eqn.clone(), false)?;
                unfixedVars = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (1..=(var_to_eqn.clone().borrow().len() as i32)).into_iter() {
            if !(var_to_eqn.borrow()[(i.clone()-1) as usize].clone() < 0) { continue; }
            let __x = i.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                redundantEqns = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (1..=(eqn_to_var.clone().borrow().len() as i32)).into_iter() {
            if !(eqn_to_var.borrow()[(i.clone()-1) as usize].clone() < 0) { continue; }
            let __x = i.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                redundantEqns = List::unique({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (redundantEqns.clone()).into_iter().cloned() {
            let __x = scal_to_arr.borrow()[(i.clone()-1) as usize].clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                if !(redundantEqns.clone().is_empty() && unfixedVars.clone().is_empty()) {
                    (me, _, _, _) = BackendDAEUtil::getAdjacencyMatrixEnhancedScalar(outEqSystem.clone(), inShared.clone(), false)?;
                    (_, _, _) = consistencyCheck(redundantEqns.clone(), outEqSystem.orderedEqs.clone(), outEqSystem.orderedVars.clone(), inShared.clone(), 0, m.clone(), me.clone(), var_to_eqn.clone(), eqn_to_var.clone(), scal_to_arr.clone())?;
                    outEqSystem = resolveOverAndUnderconstraints(outEqSystem.clone(), initVars.clone(), unfixedVars.clone(), redundantEqns.clone(), dumpVars.clone(), removedEqns.clone())?;
                    (outEqSystem, m, mT, _, scal_to_arr) = BackendDAEUtil::getAdjacencyMatrixScalar(outEqSystem.clone(), crate::BackendDAE::IndexType::SOLVABLE, Some(funcs.clone()), true)?;
                }
            }
        }
    } else {
        outEqSystem = inEqSystem.clone();
    }
    Ok((outEqSystem, outShared, dummy))
}

fn resolveOverAndUnderconstraints(mut syst: Arc<BackendDAE::EqSystem>, mut initVars: BackendDAE::Variables, mut unfixedVars: Arc<metamodelica::List<i32>>, mut redundantEqns: Arc<metamodelica::List<i32>>, mut dumpVars: DoubleEnded::MutableList<BackendDAE::Var>, mut removedEqns: DoubleEnded::MutableList<Arc<BackendDAE::Equation>>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut syst: Arc<BackendDAE::EqSystem> = syst;
    let mut debug: bool = false;
    let mut redundant_lst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut failed_var_lst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut var_lst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut new_eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    redundant_lst = BackendEquation::getList(redundantEqns.clone(), syst.orderedEqs.clone());
    DoubleEnded::push_list_back(removedEqns.clone(), redundant_lst.clone());
    new_eqns = BackendEquation::deleteList(syst.orderedEqs.clone(), redundantEqns.clone())?;
    if debug.clone() {
        BackendDump::dumpEquationList(redundant_lst.clone(), (literal!("removed eqns")).clone());
    }
    var_lst = {
        let mut __acc: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
        for mut i in (unfixedVars.clone()).into_iter().cloned() {
            let __x = BackendVariable::getVarAt(syst.orderedVars.clone(), i.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    (new_eqns, var_lst) = addStartValueEquations(var_lst.clone(), new_eqns.clone(), metamodelica::nil())?;
    DoubleEnded::push_list_back(dumpVars.clone(), var_lst.clone());
    if debug.clone() {
        failed_var_lst = {
        let mut __acc: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
        for mut var in (var_lst.clone()).into_iter().cloned() {
            if !(!(BackendVariable::containsVar(var.clone(), initVars.clone()))) { continue; }
            let __x = var.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        BackendDump::dumpVarList(var_lst.clone(), (literal!("fixed vars")).clone());
        BackendDump::dumpVarList(failed_var_lst.clone(), (literal!("failed vars")).clone());
    }
    syst = BackendDAEUtil::setEqSystEqs(syst.clone(), BackendEquation::sortInitialEqns(new_eqns.clone())?);
    Ok(syst)
}

fn fixInitialSystem(mut inEqSystem: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut dummy: i32, mut initVars: BackendDAE::Variables, mut dumpVars: DoubleEnded::MutableList<BackendDAE::Var>, mut removedEqns: DoubleEnded::MutableList<Arc<BackendDAE::Equation>>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32)> {
    let mut outEqSystem: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut dummy: i32 = dummy;
    let mut eqns2: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut dumpVars2: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut removedEqns2: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut nVars: i32 = 0;
    let mut nEqns: i32 = 0;
    let mut nInitEqs: i32 = 0;
    let mut nAddEqs: i32 = 0;
    let mut nAddVars: i32 = 0;
    let mut stateIndices: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut range: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut initEqsIndices: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut redundantEqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut initVarList: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut m_: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    let mut me: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
    let mut mapIncRowEqn: metamodelica::Array<i32>;
    let mut perfectMatching: bool = false;
    let mut maxMixedDeterminedIndex: i32 = intMax(0, Flags::getConfigInt(Flags::MAX_MIXED_DETERMINED_INDEX.clone())?);
    let mut eMarks: metamodelica::Array<bool>;
    let mut vMarks: metamodelica::Array<bool>;
    let mut singular_eqns_idx: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut singular_vars_idx: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut overDetIndex: i32 = 0;
    let mut underDetIndex: i32 = 0;
    let mut scalarEqnSize: i32 = 0;
    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let debug: bool = false;
    for mut index in 0..=maxMixedDeterminedIndex.clone() {
        nVars = BackendVariable::varsSize(inEqSystem.orderedVars.clone())?;
        nEqns = BackendEquation::equationArraySize(inEqSystem.orderedEqs.clone())?;
        syst = BackendDAEUtil::createEqSystem(inEqSystem.orderedVars.clone(), inEqSystem.orderedEqs.clone(), metamodelica::nil(), crate::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
        funcs = BackendDAEUtil::getFunctions(inShared.clone())?;
        (m_, _, _, mapIncRowEqn) = BackendDAEUtil::adjacencyMatrixScalar(syst.clone(), crate::BackendDAE::IndexType::SOLVABLE, Some(funcs.clone()), BackendDAEUtil::isInitializationDAE(inShared.clone()))?;
        if debug.clone() {
            BackendDump::dumpEqSystem(syst.clone(), (literal!("fixInitialSystem")).clone())?;
            BackendDump::dumpVariables(initVars.clone(), (literal!("selected initialization variables")).clone())?;
            BackendDump::dumpVariables(inEqSystem.orderedVars.clone(), (literal!("vars in the system")).clone())?;
            BackendDump::dumpAdjacencyMatrix(m_.clone());
        }
        stateIndices = BackendVariable::getVarIndexFromVariablesIndexInFirstSet(inEqSystem.orderedVars.clone(), initVars.clone())?;
        nAddEqs = intMax(nVars.clone() - nEqns.clone() + index.clone(), index.clone());
        if debug.clone() {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("nAddEqs: ")); __mm_s.push_str(&*intString(nAddEqs.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        m = fixUnderDeterminedSystem(m_.clone(), stateIndices.clone(), nEqns.clone(), nAddEqs.clone())?;
        nAddVars = intMax(nEqns.clone() - nVars.clone() + index.clone(), index.clone());
        if debug.clone() {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("nAddVars: ")); __mm_s.push_str(&*intString(nAddVars.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        m = fixOverDeterminedSystem(m.clone(), inEqSystem.orderedEqs.clone(), nVars.clone(), nAddVars.clone())?;
        (ass1, ass2, perfectMatching, eMarks, vMarks) = Matching::RegularMatching(m.clone(), nVars.clone() + nAddVars.clone(), nEqns.clone() + nAddEqs.clone())?;
        if debug.clone() {
            BackendDump::dumpMatchingVars(ass1.clone());
            BackendDump::dumpMatchingEqns(ass2.clone());
        }
        if perfectMatching.clone() {
            if index.clone() > 0 {
                Error::addCompilerNotification(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The given system is mixed-determined.   [index = ")); __mm_s.push_str(&*intString(index.clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone())?;
            }
            if nAddVars.clone() > 0 {
                range = List::intRange2(nVars.clone() + 1, nVars.clone() + nAddVars.clone());
                redundantEqns = mapIndices(range.clone(), ass1.clone());
                (me, _, _, _) = BackendDAEUtil::getAdjacencyMatrixEnhancedScalar(syst.clone(), inShared.clone(), false)?;
                (_, _, _) = consistencyCheck(redundantEqns.clone(), inEqSystem.orderedEqs.clone(), inEqSystem.orderedVars.clone(), inShared.clone(), nAddVars.clone(), m_.clone(), me.clone(), ass1.clone(), ass2.clone(), mapIncRowEqn.clone())?;
                removedEqns2 = BackendEquation::getList(redundantEqns.clone(), inEqSystem.orderedEqs.clone());
                eqns2 = BackendEquation::deleteList(inEqSystem.orderedEqs.clone(), redundantEqns.clone())?;
                DoubleEnded::push_list_back(removedEqns.clone(), removedEqns2.clone());
            } else {
                eqns2 = inEqSystem.orderedEqs.clone();
            }
            if nAddEqs.clone() > 0 {
                range = List::intRange2(nEqns.clone() + 1, nEqns.clone() + nAddEqs.clone());
                range = mapIndices(range.clone(), ass2.clone());
                initVarList = List::map1r(range.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), inEqSystem.orderedVars.clone());
                (eqns2, dumpVars2) = addStartValueEquations(initVarList.clone(), eqns2.clone(), metamodelica::nil())?;
                DoubleEnded::push_list_back(dumpVars.clone(), dumpVars2.clone());
            }
            outEqSystem = BackendDAEUtil::setEqSystEqs(inEqSystem.clone(), eqns2.clone());
            return Ok((outEqSystem, outShared, dummy));
        }
        if debug.clone() {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("index-")); __mm_s.push_str(&*intString(index.clone())); __mm_s.push_str(&*literal!(" ende\n")); ArcStr::from(__mm_s) }).clone());
        }
    }
    if Flags::isSet(Flags::INITIALIZATION.clone())? {
        overDetIndex = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (1..=(ass1.clone().borrow().len() as i32)).into_iter() {
            if !(ass1.borrow()[(i.clone()-1) as usize].clone() < 0) { continue; }
            let __x = i.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.len() as i32);
        underDetIndex = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (1..=(ass2.clone().borrow().len() as i32)).into_iter() {
            if !(ass2.borrow()[(i.clone()-1) as usize].clone() < 0) { continue; }
            let __x = i.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.len() as i32);
        singular_eqns_idx = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (1..=(mapIncRowEqn.clone().borrow().len() as i32)).into_iter() {
            if !(eMarks.borrow()[(i.clone()-1) as usize].clone()) { continue; }
            let __x = i.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        singular_vars_idx = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (1..=BackendVariable::varsSize(syst.orderedVars.clone())?).into_iter() {
            if !(vMarks.borrow()[(i.clone()-1) as usize].clone()) { continue; }
            let __x = i.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        scalarEqnSize = (singular_eqns_idx.clone().len() as i32);
        singular_eqns_idx = List::uniqueOnTrue({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (singular_eqns_idx.clone()).into_iter().cloned() {
            let __x = mapIncRowEqn.borrow()[(i.clone()-1) as usize].clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>));
        println!("{}", (literal!("\n------------ UNBALANCED INITIAL SYSTEM ------------\n")).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The initial system is over- as well as underdetermined and it could not be resolved after ")); __mm_s.push_str(&*intString(maxMixedDeterminedIndex.clone())); __mm_s.push_str(&*literal!(" iterations.\n\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("==== OVERDETERMINATION BY ")); __mm_s.push_str(&*intString(overDetIndex.clone())); __mm_s.push_str(&*literal!(" EQUATION(S)\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("==== UNDERDETERMINATION OF ")); __mm_s.push_str(&*intString(underDetIndex.clone())); __mm_s.push_str(&*literal!(" VARIABLE(S)\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n---- involved set eqns (")); __mm_s.push_str(&*intString(scalarEqnSize.clone())); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*intString((singular_eqns_idx.clone().len() as i32))); __mm_s.push_str(&*literal!("):\n")); ArcStr::from(__mm_s) }).clone());
        for mut eqn in &*singular_eqns_idx.clone() {
            let mut eqn = eqn.clone();
            eq = BackendEquation::get(syst.orderedEqs.clone(), mapIncRowEqn.borrow()[(eqn.clone()-1) as usize].clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  ")); __mm_s.push_str(&*intString(eqn.clone())); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(BackendEquation::equationSize(eq.clone())?)); __mm_s.push_str(&*literal!("):\t")); __mm_s.push_str(&*BackendDump::equationString(eq.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n---- involved set vars (")); __mm_s.push_str(&*intString((singular_vars_idx.clone().len() as i32))); __mm_s.push_str(&*literal!("):\n")); ArcStr::from(__mm_s) }).clone());
        for mut var in &*singular_vars_idx.clone() {
            let mut var = var.clone();
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  ")); __mm_s.push_str(&*intString(var.clone())); __mm_s.push_str(&*literal!(":\t")); __mm_s.push_str(&*BackendDump::varString(BackendVariable::getVarAt(syst.orderedVars.clone(), var.clone())?)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        println!("{}", (literal!("--------------------------------------------------\n")).clone());
    }
    Error::addMessage(Error::MIXED_DETERMINED.clone(), list![(intString(maxMixedDeterminedIndex.clone())).clone()])?;
    bail!("fail");
    Ok((outEqSystem, outShared, dummy))
}

fn updateFixedAttribute(mut var: BackendDAE::Var, mut vars: BackendDAE::Variables) -> Result<(BackendDAE::Var, BackendDAE::Variables)> {
    let mut var: BackendDAE::Var = var;
    let mut vars: BackendDAE::Variables = vars;
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    cr = BackendVariable::varCref(var.clone())?;
    if BackendVariable::containsCref(cr.clone(), vars.clone()) {
        var = BackendVariable::setVarFixed(var.clone(), true)?;
    }
    Ok((var, vars))
}

fn fixUnderDeterminedSystem(mut inM: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inInitVarIndices: Arc<metamodelica::List<i32>>, mut inNEqns: i32, mut inNAddEqns: i32) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut outM: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut newEqIndices: Arc<metamodelica::List<i32>> = metamodelica::nil();
    if inNAddEqns.clone() < 0 {
        Error::addInternalError((literal!("function fixUnderDeterminedSystem failed due to invalid input")).clone(), metamodelica::sourceInfo!())?;
        bail!("fail");
    }
    if inNAddEqns.clone() > 0 {
        outM = arrayCreate(inNEqns.clone() + inNAddEqns.clone(), metamodelica::nil());
        outM = Array::copy(inM.clone(), outM.clone())?;
        newEqIndices = List::intRange2(inNEqns.clone() + 1, inNEqns.clone() + inNAddEqns.clone());
        outM = List::fold1(newEqIndices.clone(), (std::sync::Arc::new(fnptr!(squareAdjacencyMatrix1, i32, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> + 'static>), inInitVarIndices.clone(), outM.clone());
    } else {
        outM = metamodelica::arrayFromVec(inM.clone().borrow().clone());
    }
    Ok(outM)
}

fn squareAdjacencyMatrix1(mut inPos: i32, mut inDependency: Arc<metamodelica::List<i32>>, mut inM: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> metamodelica::Array<Arc<metamodelica::List<i32>>> {
    let mut outM: metamodelica::Array<Arc<metamodelica::List<i32>>> = inM.clone();
    {
        let __cell0 = inDependency.clone();
        outM.clone().borrow_mut()[(inPos.clone()-1) as usize] = __cell0;
    }
    outM
}

fn fixOverDeterminedSystem(mut inM: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut orderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inNVars: i32, mut inNAddVars: i32) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut outM: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut newVarIndices: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut initEqsIndices: Arc<metamodelica::List<i32>> = metamodelica::nil();
    if inNAddVars.clone() < 0 {
        Error::addInternalError((literal!("function fixOverDeterminedSystem failed due to invalid input")).clone(), metamodelica::sourceInfo!())?;
        bail!("fail");
    }
    if inNAddVars.clone() > 0 {
        initEqsIndices = getInitEqIndices(BackendEquation::equationList(orderedEqs.clone()))?;
        newVarIndices = List::intRange2(inNVars.clone() + 1, inNVars.clone() + inNAddVars.clone());
        outM = List::fold1(initEqsIndices.clone(), (std::sync::Arc::new(fnptr!(squareAdjacencyMatrix2, i32, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> + 'static>), newVarIndices.clone(), inM.clone());
    } else {
        outM = inM.clone();
    }
    Ok(outM)
}

fn squareAdjacencyMatrix2(mut inPos: i32, mut inRange: Arc<metamodelica::List<i32>>, mut inM: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> metamodelica::Array<Arc<metamodelica::List<i32>>> {
    let mut outM: metamodelica::Array<Arc<metamodelica::List<i32>>> = inM.clone();
    {
        let __cell0 = listAppend(inM.borrow()[(inPos.clone()-1) as usize].clone(), inRange.clone());
        outM.clone().borrow_mut()[(inPos.clone()-1) as usize] = __cell0;
    }
    outM
}

fn addStartValueEquations(mut inVarLst: Arc<metamodelica::List<BackendDAE::Var>>, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inDumpVars: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)> {
    let mut outEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = inEqns.clone();
    let mut outDumpVars: Arc<metamodelica::List<BackendDAE::Var>> = inDumpVars.clone();
    let mut dumpVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut e: Arc<DAE::Exp>;
    let mut crefExp: Arc<DAE::Exp>;
    let mut startExp: Arc<DAE::Exp>;
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut isPreCref: bool = false;
    for mut var in &*inVarLst.clone() {
        let mut var = var.clone();
        cref = BackendVariable::varCref(var.clone())?;
        tp = BackendVariable::varType(var.clone())?;
        crefExp = Arc::new(DAE::Exp::CREF { componentRef: cref.clone(), ty: tp.clone() });
        isPreCref = ComponentReference::isPreCref(cref.clone());
        if isPreCref.clone() {
            cref = ComponentReference::popPreCref(cref.clone());
        }
        e = Expression::crefExp(cref.clone())?;
        tp = Expression::r#typeof(e.clone())?;
        startExp = Expression::crefExp(ComponentReference::crefPrefixStart(cref.clone()))?;
        eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: crefExp.clone(), scalar: startExp.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
        outEqns = BackendEquation::add(eqn.clone(), outEqns.clone())?;
        if isPreCref.clone() {
            dumpVar = BackendVariable::copyVarNewName(cref.clone(), var.clone());
            dumpVar = BackendVariable::setVarFixed(dumpVar.clone(), true)?;
            outDumpVars = cons(dumpVar.clone(), outDumpVars.clone());
        } else {
            dumpVar = BackendVariable::setVarFixed(var.clone(), true)?;
            outDumpVars = cons(dumpVar.clone(), outDumpVars.clone());
        }
    }
    Ok((outEqns, outDumpVars))
}

// =============================================================================
// section for symbolic consistency check
//
// =============================================================================
fn consistencyCheck(mut inRedundantEqns: Arc<metamodelica::List<i32>>, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inVars: BackendDAE::Variables, mut inShared: Arc<BackendDAE::Shared>, mut nAddVars: i32, mut inM: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut me: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut vecVarToEqs: metamodelica::Array<i32>, mut vecEqsToVar: metamodelica::Array<i32>, mut mapIncRowEqn: metamodelica::Array<i32>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut outConsistentEquations: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut outInconsistentEquations: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut outUncheckedEquations: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (outConsistentEquations, outInconsistentEquations, outUncheckedEquations) = 'mc: {
        let __mc_input = inRedundantEqns.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((metamodelica::nil(), metamodelica::nil(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: currRedundantEqn, tail: restRedundantEqns } => {
                    let mut outRange: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut flatComps: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut markedComps: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut outLoopListComps: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut consistentEquations: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut consistentEquations2: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut inconsistentEquations: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut uncheckedEquations: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut uncheckedEquations2: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut nEqns: i32 = 0;
                    let mut redundantEqn: i32 = 0;
                    let mut comps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
                    let mut substEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let _ = BackendVariable::varsSize(inVars.clone())?;
                    nEqns = BackendEquation::equationArraySize(inEqns.clone())?;
                    comps = Sorting::Tarjan(inM.clone(), vecVarToEqs.clone(), nEqns.clone())?;
                    flatComps = List::flatten(comps.clone());
                    (_, outLoopListComps) = splitStrongComponents(comps.clone());
                    redundantEqn = mapIndex(currRedundantEqn.clone(), mapIncRowEqn.clone());
                    flatComps = mapIndices(flatComps.clone(), mapIncRowEqn.clone());
                    outLoopListComps = mapIndices(outLoopListComps.clone(), mapIncRowEqn.clone());
                    markedComps = compsMarker(currRedundantEqn.clone(), vecVarToEqs.clone(), inM.clone(), flatComps.clone(), outLoopListComps.clone())?;
                    repl = BackendVarTransform::emptyReplacements();
                    repl = setupVarReplacements(markedComps.clone(), inEqns.clone(), inVars.clone(), vecEqsToVar.clone(), repl.clone(), mapIncRowEqn.clone(), me.clone(), inShared.clone())?;
                    substEqns = applyVarReplacements(redundantEqn.clone(), inEqns.clone(), repl.clone())?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(getConsistentEquation(redundantEqn.clone(), substEqns.clone(), inEqns.clone(), inM.clone(), vecVarToEqs.clone(), inVars.clone(), inShared.clone(), 1)?) {
                        (__pa0, true, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    outRange = __pa0.clone();
                    uncheckedEquations = __pa1.clone();
                    (consistentEquations, inconsistentEquations, uncheckedEquations2) = consistencyCheck(restRedundantEqns.clone(), inEqns.clone(), inVars.clone(), inShared.clone(), nAddVars.clone(), inM.clone(), me.clone(), vecVarToEqs.clone(), vecEqsToVar.clone(), mapIncRowEqn.clone())?;
                    consistentEquations2 = listAppend(consistentEquations.clone(), outRange.clone());
                    uncheckedEquations2 = listAppend(uncheckedEquations.clone(), uncheckedEquations2.clone());
                    Ok((consistentEquations2.clone(), inconsistentEquations.clone(), uncheckedEquations2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: currRedundantEqn, tail: restRedundantEqns } => {
                    let mut consistentEquations: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut inconsistentEquations: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut uncheckedEquations: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    (consistentEquations, inconsistentEquations, uncheckedEquations) = consistencyCheck(restRedundantEqns.clone(), inEqns.clone(), inVars.clone(), inShared.clone(), nAddVars.clone(), inM.clone(), me.clone(), vecVarToEqs.clone(), vecEqsToVar.clone(), mapIncRowEqn.clone())?;
                    Ok((consistentEquations.clone(), cons(currRedundantEqn.clone(), inconsistentEquations.clone()), uncheckedEquations.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outConsistentEquations, outInconsistentEquations, outUncheckedEquations))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn isVarExplicitSolvable(mut inElem: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>, mut inVarID: i32) -> bool {
    let mut outSolvable: bool = false;
    outSolvable = (::match_deref::match_deref! { match &(inElem.clone()) {
        Deref @ metamodelica::List::Nil => {
            true
        },
        Deref @ metamodelica::List::Cons { head: (id, BackendDAE::Solvability::SOLVABILITY_UNSOLVABLE, _), tail: _ } if (intEq(id.clone(), inVarID.clone())) => {
            false
        },
        Deref @ metamodelica::List::Cons { head: (id, BackendDAE::Solvability::SOLVABILITY_NONLINEAR, _), tail: _ } if (intEq(id.clone(), inVarID.clone())) => {
            false
        },
        Deref @ metamodelica::List::Cons { head: (_, _, _), tail: elem } => {
            let mut b: bool = false;
            b = isVarExplicitSolvable(elem.clone(), inVarID.clone());
            b.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outSolvable
}

fn splitStrongComponents(mut inComps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) {
    let mut outListComps: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut outLoopListComps: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (outListComps, outLoopListComps) = (::match_deref::match_deref! { match &(inComps.clone()) {
        Deref @ metamodelica::List::Nil => {
            (metamodelica::nil(), metamodelica::nil())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Cons { head: currIndex, tail: Deref @ metamodelica::List::Nil }, tail: restComps } => {
            let mut listComps: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut loopListComps: Arc<metamodelica::List<i32>> = metamodelica::nil();
            (listComps, loopListComps) = splitStrongComponents(restComps.clone());
            (cons(currIndex.clone(), listComps.clone()), loopListComps.clone())
        },
        Deref @ metamodelica::List::Cons { head: currComp, tail: restComps } => {
            let mut listComps: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut loopListComps: Arc<metamodelica::List<i32>> = metamodelica::nil();
            (listComps, loopListComps) = splitStrongComponents(restComps.clone());
            loopListComps = listAppend(currComp.clone(), loopListComps.clone());
            (listComps.clone(), loopListComps.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outListComps, outLoopListComps)
}

fn mapIndex(mut inIndex: i32, mut inMapping: metamodelica::Array<i32>) -> i32 {
    let mut outIndex: i32 = 0;
    outIndex = inMapping.borrow()[(inIndex.clone()-1) as usize].clone();
    outIndex
}

fn mapIndices(mut inIndices: Arc<metamodelica::List<i32>>, mut inMapping: metamodelica::Array<i32>) -> Arc<metamodelica::List<i32>> {
    let mut outIndices: Arc<metamodelica::List<i32>> = metamodelica::nil();
    outIndices = List::map1(inIndices.clone(), (std::sync::Arc::new(fnptr!(mapIndex, i32, metamodelica::Array<i32>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<i32> + 'static>), inMapping.clone());
    outIndices
}

fn mapListIndices(mut inListIndices: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut inMapping: metamodelica::Array<i32>) -> Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> {
    let mut outListIndices: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    outListIndices = List::map1(inListIndices.clone(), (std::sync::Arc::new(fnptr!(mapIndices, Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), inMapping.clone());
    outListIndices
}

fn compsMarker(mut inUnassignedEqn: i32, mut inVecVarToEq: metamodelica::Array<i32>, mut inM: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inFlatComps: Arc<metamodelica::List<i32>>, mut inLoopListComps: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outMarkedEqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut varList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut markedEqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    match '__try0: {
        let false = (listMember(inUnassignedEqn.clone(), inLoopListComps.clone())) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        varList = inM.borrow()[(inUnassignedEqn.clone()-1) as usize].clone();
        markedEqns = unwrap_break_err!(compsMarker2(varList.clone(), inVecVarToEq.clone(), inM.clone(), inFlatComps.clone(), metamodelica::nil(), inLoopListComps.clone()), '__try0);
        outMarkedEqns = unwrap_break_err!(downCompsMarker(inFlatComps.clone().reverse(), inVecVarToEq.clone(), inM.clone(), inFlatComps.clone(), markedEqns.clone(), inLoopListComps.clone()), '__try0);
        Ok::<_, anyhow::Error>((markedEqns.clone(), outMarkedEqns.clone(), varList.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            markedEqns = __try0_o0;
            outMarkedEqns = __try0_o1;
            varList = __try0_o2;
        }
        Err(_) => {
            Error::addCompilerNotification((literal!("It was not possible to check the given initialization system for consistency symbolically, because the relevant equations are part of an algebraic loop. This is not supported yet.")).clone())?;
            bail!("fail");
        }
    }
    Ok(outMarkedEqns)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn compsMarker2(mut inVarList: Arc<metamodelica::List<i32>>, mut inVecVarToEq: metamodelica::Array<i32>, mut inM: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inFlatComps: Arc<metamodelica::List<i32>>, mut inMarkedEqns: Arc<metamodelica::List<i32>>, mut inLoopListComps: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outMarkedEqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    outMarkedEqns = 'mc: {
        let __mc_input = inVarList.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(inMarkedEqns.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: indexVar, tail: var_list2 } => {
                    let mut indexEq: i32 = 0;
                    let mut markedEqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    indexEq = inVecVarToEq.borrow()[(indexVar.clone()-1) as usize].clone();
                    let false = (listMember(indexEq.clone(), inLoopListComps.clone())) else { bail!("pattern mismatch") };
                    let false = (listMember(indexEq.clone(), inMarkedEqns.clone())) else { bail!("pattern mismatch") };
                    markedEqns = compsMarker2(var_list2.clone(), inVecVarToEq.clone(), inM.clone(), inFlatComps.clone(), inMarkedEqns.clone(), inLoopListComps.clone())?;
                    Ok(cons(indexEq.clone(), markedEqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: indexVar, tail: var_list2 } => {
                    let mut indexEq: i32 = 0;
                    let mut markedEqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    indexEq = inVecVarToEq.borrow()[(indexVar.clone()-1) as usize].clone();
                    let false = (listMember(indexEq.clone(), inLoopListComps.clone())) else { bail!("pattern mismatch") };
                    let true = (listMember(indexEq.clone(), inMarkedEqns.clone())) else { bail!("pattern mismatch") };
                    markedEqns = compsMarker2(var_list2.clone(), inVecVarToEq.clone(), inM.clone(), inFlatComps.clone(), inMarkedEqns.clone(), inLoopListComps.clone())?;
                    Ok(markedEqns.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addCompilerNotification((literal!("It was not possible to check the given initialization system for consistency symbolically, because the relevant equations are part of an algebraic loop. This is not supported yet.")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outMarkedEqns)
}

fn downCompsMarker(mut unassignedEqns: Arc<metamodelica::List<i32>>, mut vecVarToEq: metamodelica::Array<i32>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut flatComps: Arc<metamodelica::List<i32>>, mut inMarkedEqns: Arc<metamodelica::List<i32>>, mut inLoopListComps: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut inMarkedEqns: Arc<metamodelica::List<i32>> = inMarkedEqns;
    for mut indexUnassigned in &*unassignedEqns.clone() {
        let mut indexUnassigned = indexUnassigned.clone();
        if listMember(indexUnassigned.clone(), inMarkedEqns.clone()) {
            inMarkedEqns = compsMarker2(m.borrow()[(indexUnassigned.clone()-1) as usize].clone(), vecVarToEq.clone(), m.clone(), flatComps.clone(), inMarkedEqns.clone(), inLoopListComps.clone())?;
        }
    }
    Ok(inMarkedEqns)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn setupVarReplacements(mut inMarkedEqns: Arc<metamodelica::List<i32>>, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inVars: BackendDAE::Variables, mut inVecEqToVar: metamodelica::Array<i32>, mut inRepls: BackendVarTransform::VariableReplacements, mut inMapIncRowEqn: metamodelica::Array<i32>, mut inME: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut inShared: Arc<BackendDAE::Shared>) -> Result<BackendVarTransform::VariableReplacements> {
    let mut outRepls: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    outRepls = 'mc: {
        let __mc_input = inMarkedEqns.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(inRepls.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: markedEqn, tail: markedEqns } => {
                    let mut indexVar: i32 = 0;
                    let mut indexEq: i32 = 0;
                    let mut repls: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
                    let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut varName: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut type_: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut exp: Arc<DAE::Exp>;
                    let mut exp1: Arc<DAE::Exp>;
                    let mut x: Arc<DAE::Exp>;
                    indexVar = inVecEqToVar.borrow()[(markedEqn.clone()-1) as usize].clone();
                    let true = (isVarExplicitSolvable(inME.borrow()[(markedEqn.clone()-1) as usize].clone(), indexVar.clone())) else { bail!("pattern mismatch") };
                    var = BackendVariable::getVarAt(inVars.clone(), indexVar.clone())?;
                    indexEq = inMapIncRowEqn.borrow()[(markedEqn.clone()-1) as usize].clone();
                    eqn = BackendEquation::get(inEqns.clone(), indexEq.clone());
                    cref = BackendVariable::varCref(var.clone())?;
                    type_ = BackendVariable::varType(var.clone())?;
                    x = Arc::new(DAE::Exp::CREF { componentRef: cref.clone(), ty: type_.clone() });
                    let (__pa1, __pa0) = ::match_deref::match_deref! { match &(BackendEquation::solveEquation(eqn.clone(), x.clone(), Some(inShared.functionTree.clone()))?) {
                        __pa1 @ Deref @ BackendDAE::Equation::EQUATION { scalar: __pa0, .. } => (__pa1.clone(), __pa0.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    exp = __pa0.clone();
                    eqn = __pa1.clone();
                    varName = BackendVariable::varCref(var.clone())?;
                    (exp1, _) = Expression::traverseExpBottomUp(exp.clone(), (std::sync::Arc::new(BackendDAEUtil::replaceCrefsWithValues) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<DAE::ComponentRef>)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<DAE::ComponentRef>))> + 'static>), (inVars.clone(), varName.clone()))?;
                    repls = BackendVarTransform::addReplacement(inRepls.clone(), varName.clone(), exp1.clone(), None)?;
                    repls = setupVarReplacements(markedEqns.clone(), inEqns.clone(), inVars.clone(), inVecEqToVar.clone(), repls.clone(), inMapIncRowEqn.clone(), inME.clone(), inShared.clone())?;
                    Ok(repls.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: markedEqns } => {
                    let mut repls: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
                    repls = setupVarReplacements(markedEqns.clone(), inEqns.clone(), inVars.clone(), inVecEqToVar.clone(), inRepls.clone(), inMapIncRowEqn.clone(), inME.clone(), inShared.clone())?;
                    Ok(repls.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outRepls)
}

fn applyVarReplacements(mut inEqnIndex: i32, mut inEqnList: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inVarRepls: BackendVarTransform::VariableReplacements) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> {
    let mut outEqnList: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    outEqnList = BackendEquation::copyEquationArray(inEqnList.clone());
    eqn = BackendEquation::get(outEqnList.clone(), inEqnIndex.clone());
    let __pa0 = ::match_deref::match_deref! { match &(BackendVarTransform::replaceEquations(list![eqn.clone()], inVarRepls.clone(), None)?) {
        (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, _) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    eqn = __pa0.clone();
    outEqnList = BackendEquation::setAtIndex(outEqnList.clone(), inEqnIndex.clone(), eqn.clone())?;
    Ok(outEqnList)
}

fn getConsistentEquation(mut inUnassignedEqn: i32, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inEqnsOrig: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inM: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut vecVarToEqs: metamodelica::Array<i32>, mut vars: BackendDAE::Variables, mut shared: Arc<BackendDAE::Shared>, mut counter: i32) -> Result<(Arc<metamodelica::List<i32>>, bool, Arc<metamodelica::List<i32>>)> {
    let mut outUnassignedEqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut outConsistent: bool = false;
    let mut outRemovedEqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (outUnassignedEqns, outConsistent, outRemovedEqns) = 'mc: {
        let __mc_input = inUnassignedEqn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut nVars: i32 = 0;
            let mut nEqns: i32 = 0;
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut lhs: Arc<DAE::Exp>;
            let mut rhs: Arc<DAE::Exp>;
            let mut exp: Arc<DAE::Exp>;
            nVars = BackendVariable::varsSize(vars.clone())?;
            nEqns = BackendEquation::equationArraySize(inEqnsOrig.clone())?;
            let true = (intLe(counter.clone(), nEqns.clone() - nVars.clone())) else { bail!("pattern mismatch") };
            eqn = BackendEquation::get(inEqns.clone(), inUnassignedEqn.clone());
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(eqn.clone()) {
                Deref @ BackendDAE::Equation::EQUATION { scalar: __pa0, exp: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            rhs = __pa0.clone();
            lhs = __pa1.clone();
            exp = Arc::new(DAE::Exp::BINARY { exp1: lhs.clone(), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: rhs.clone() });
            (exp, _) = ExpressionSimplify::simplify(exp.clone())?;
            let true = (Expression::isZero(exp.clone())) else { bail!("pattern mismatch") };
            let _ = BackendEquation::get(inEqnsOrig.clone(), inUnassignedEqn.clone());
            Ok((list![inUnassignedEqn.clone()], true, metamodelica::nil()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut nVars: i32 = 0;
            let mut nEqns: i32 = 0;
            nVars = BackendVariable::varsSize(vars.clone())?;
            nEqns = BackendEquation::equationArraySize(inEqnsOrig.clone())?;
            let true = (intGt(counter.clone(), nEqns.clone() - nVars.clone())) else { bail!("pattern mismatch") };
            Error::addCompilerError((literal!("Initialization problem is structural singular. Please, check the initial conditions.")).clone())?;
            Ok((metamodelica::nil(), true, metamodelica::nil()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut nVars: i32 = 0;
            let mut nEqns: i32 = 0;
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut eqn2: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut lhs: Arc<DAE::Exp>;
            let mut rhs: Arc<DAE::Exp>;
            let mut exp: Arc<DAE::Exp>;
            let mut listParameter: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            nVars = BackendVariable::varsSize(vars.clone())?;
            nEqns = BackendEquation::equationArraySize(inEqnsOrig.clone())?;
            let true = (intLe(counter.clone(), nEqns.clone() - nVars.clone())) else { bail!("pattern mismatch") };
            eqn = BackendEquation::get(inEqns.clone(), inUnassignedEqn.clone());
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(eqn.clone()) {
                Deref @ BackendDAE::Equation::EQUATION { scalar: __pa0, exp: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            rhs = __pa0.clone();
            lhs = __pa1.clone();
            exp = Arc::new(DAE::Exp::BINARY { exp1: lhs.clone(), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: rhs.clone() });
            (exp, _) = ExpressionSimplify::simplify(exp.clone())?;
            let false = (Expression::isZero(exp.clone())) else { bail!("pattern mismatch") };
            let __pa2 = ::match_deref::match_deref! { match &(parameterCheck(exp.clone())?) {
                (__pa2, false) => __pa2.clone(),
                _ => bail!("pattern mismatch"),
            } };
            listParameter = __pa2.clone();
            let true = (listParameter.clone().is_empty()) else { bail!("pattern mismatch") };
            eqn2 = BackendEquation::get(inEqnsOrig.clone(), inUnassignedEqn.clone());
            Error::addCompilerError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The initialization problem is inconsistent due to the following equation: ")); __mm_s.push_str(&*BackendDump::equationString(eqn2.clone())?); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*BackendDump::equationString(eqn.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
            Ok((metamodelica::nil(), false, metamodelica::nil()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut nVars: i32 = 0;
            let mut nEqns: i32 = 0;
            let mut listVar: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut lhs: Arc<DAE::Exp>;
            let mut rhs: Arc<DAE::Exp>;
            let mut exp: Arc<DAE::Exp>;
            let mut listParameter: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut system: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut list_inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            nVars = BackendVariable::varsSize(vars.clone())?;
            nEqns = BackendEquation::equationArraySize(inEqnsOrig.clone())?;
            let true = (intLe(counter.clone(), nEqns.clone() - nVars.clone())) else { bail!("pattern mismatch") };
            eqn = BackendEquation::get(inEqns.clone(), inUnassignedEqn.clone());
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(eqn.clone()) {
                Deref @ BackendDAE::Equation::EQUATION { scalar: __pa0, exp: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            rhs = __pa0.clone();
            lhs = __pa1.clone();
            exp = Arc::new(DAE::Exp::BINARY { exp1: lhs.clone(), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: rhs.clone() });
            (exp, _) = ExpressionSimplify::simplify(exp.clone())?;
            let false = (Expression::isZero(exp.clone())) else { bail!("pattern mismatch") };
            (listParameter, _) = parameterCheck(exp.clone())?;
            let false = (listParameter.clone().is_empty()) else { bail!("pattern mismatch") };
            list_inEqns = BackendEquation::equationList(inEqns.clone());
            list_inEqns = List::set(list_inEqns.clone(), inUnassignedEqn.clone(), eqn.clone())?;
            eqns = BackendEquation::listEquation(list_inEqns.clone())?;
            funcs = BackendDAEUtil::getFunctions(shared.clone())?;
            system = BackendDAEUtil::createEqSystem(vars.clone(), eqns.clone(), metamodelica::nil(), crate::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
            (m, _) = BackendDAEUtil::adjacencyMatrix(system.clone(), crate::BackendDAE::IndexType::NORMAL, Some(funcs.clone()), BackendDAEUtil::isInitializationDAE(shared.clone()))?;
            listVar = m.borrow()[(inUnassignedEqn.clone()-1) as usize].clone();
            let false = (listVar.clone().is_empty()) else { bail!("pattern mismatch") };
            let _ = BackendEquation::get(inEqnsOrig.clone(), inUnassignedEqn.clone());
            Error::addCompilerNotification((literal!("It was not possible to check the given initialization system for consistency symbolically, because the relevant equations are part of an algebraic loop. This is not supported yet.")).clone())?;
            Ok((metamodelica::nil(), false, metamodelica::nil()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut nVars: i32 = 0;
            let mut nEqns: i32 = 0;
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut eqn2: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut lhs: Arc<DAE::Exp>;
            let mut rhs: Arc<DAE::Exp>;
            let mut exp: Arc<DAE::Exp>;
            let mut listParameter: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut anyStartValue: bool = false;
            nVars = BackendVariable::varsSize(vars.clone())?;
            nEqns = BackendEquation::equationArraySize(inEqnsOrig.clone())?;
            let true = (intLe(counter.clone(), nEqns.clone() - nVars.clone())) else { bail!("pattern mismatch") };
            eqn = BackendEquation::get(inEqns.clone(), inUnassignedEqn.clone());
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(eqn.clone()) {
                Deref @ BackendDAE::Equation::EQUATION { scalar: __pa0, exp: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            rhs = __pa0.clone();
            lhs = __pa1.clone();
            exp = Arc::new(DAE::Exp::BINARY { exp1: lhs.clone(), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: rhs.clone() });
            (exp, _) = ExpressionSimplify::simplify(exp.clone())?;
            let false = (Expression::isZero(exp.clone())) else { bail!("pattern mismatch") };
            (listParameter, anyStartValue) = parameterCheck(exp.clone())?;
            let true = (!(listParameter.clone().is_empty()) || anyStartValue.clone()) else { bail!("pattern mismatch") };
            eqn2 = BackendEquation::get(inEqnsOrig.clone(), inUnassignedEqn.clone());
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("It was not possible to determine if the initialization problem is consistent, because of not evaluable parameters/start values during compile time: ")); __mm_s.push_str(&*BackendDump::equationString(eqn2.clone())?); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*BackendDump::equationString(eqn.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
            Ok((metamodelica::nil(), true, list![inUnassignedEqn.clone()]))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outUnassignedEqns, outConsistent, outRemovedEqns))
}

fn parameterCheck(mut inExp: Arc<DAE::Exp>) -> Result<(Arc<metamodelica::List<ArcStr>>, bool)> {
    let mut outParameters: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outAnyStartValue: bool = false;
    let (_, (__pa0, __pa1)) = Expression::traverseExpTopDown(inExp.clone(), (std::sync::Arc::new(parameterCheck2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<ArcStr>>, bool)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<ArcStr>>, bool))> + 'static>), (metamodelica::nil(), false))?;
    outParameters = __pa0.clone();
    outAnyStartValue = __pa1.clone();
    Ok((outParameters, outAnyStartValue))
}

fn parameterCheck2(mut inExp: Arc<DAE::Exp>, mut inParams: (Arc<metamodelica::List<ArcStr>>, bool)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<ArcStr>>, bool))> {
    let mut outExp: Arc<DAE::Exp> = inExp.clone();
    let mut outContinue: bool = false;
    let mut outParams: (Arc<metamodelica::List<ArcStr>>, bool);
    let mut componentRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut parameters: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut anyStartValue: bool = false;
    (parameters, anyStartValue) = inParams.clone();
    (outParams, outContinue) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef, .. } => {
            if ComponentReference::isStartCref(componentRef.clone()) {
                anyStartValue = true;
            } else {
                parameters = cons((ComponentReference::crefStr(componentRef.clone())?).clone(), parameters.clone());
            }
            ((parameters.clone(), anyStartValue.clone()), !(anyStartValue.clone()))
        },
        _ => (inParams.clone(), true),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outContinue, outParams))
}

// =============================================================================
// section for introducing pre-variables for alias variables
//
// =============================================================================
fn introducePreVarsForAliasVariables(mut inVar: BackendDAE::Var, mut inTpl: (BackendDAE::Variables, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)))) -> Result<(BackendDAE::Var, (BackendDAE::Variables, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))))> {
    let mut outVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut outTpl: (BackendDAE::Variables, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)));
    (outVar, outTpl) = 'mc: {
        let __mc_input = (inVar.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ BackendDAE::Var { arryDim, varType: ty, varKind: BackendDAE::VarKind::DISCRETE, varName: cr, .. }, (vars, fixvars, eqns, hs)) => {
                    let mut preUsed: bool = false;
                    let mut isFixed: bool = false;
                    let mut startValue: Arc<DAE::Exp>;
                    let mut preCR: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut preVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut vars = (*vars).clone();
                    let mut eqns = (*eqns).clone();
                    preUsed = BaseHashSet::has(cr.clone(), hs.clone())?;
                    isFixed = BackendVariable::varFixed(var.clone());
                    startValue = BackendVariable::varStartValue(var.clone())?;
                    preCR = ComponentReference::crefPrefixPre(cr.clone());
                    preVar = BackendDAE::Var { varName: preCR.clone(), varKind: crate::BackendDAE::VarKind::DISCRETE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: ty.clone(), bindExp: None, tplExp: None, arryDim: arryDim.clone(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
                    preVar = BackendVariable::setVarFixed(preVar.clone(), false)?;
                    preVar = BackendVariable::setVarStartValueOption(preVar.clone(), Some(startValue.clone()))?;
                    eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: preCR.clone(), ty: ty.clone() }), scalar: startValue.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                    vars = if (preUsed.clone()) {BackendVariable::addVar(preVar.clone(), vars.clone())?} else {vars.clone()};
                    eqns = if (preUsed.clone() && isFixed.clone()) {BackendEquation::add(eqn.clone(), eqns.clone())?} else {eqns.clone()};
                    Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), hs.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ BackendDAE::Var { arryDim, varType: ty, varName: cr, .. }, (vars, fixvars, eqns, hs)) => {
                    let mut preUsed: bool = false;
                    let mut preCR: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut preVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut vars = (*vars).clone();
                    let mut eqns = (*eqns).clone();
                    preUsed = BaseHashSet::has(cr.clone(), hs.clone())?;
                    preCR = ComponentReference::crefPrefixPre(cr.clone());
                    preVar = BackendDAE::Var { varName: preCR.clone(), varKind: crate::BackendDAE::VarKind::VARIABLE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: ty.clone(), bindExp: None, tplExp: None, arryDim: arryDim.clone(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
                    preVar = BackendVariable::setVarFixed(preVar.clone(), false)?;
                    preVar = BackendVariable::setVarStartValueOption(preVar.clone(), Some(Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() })))?;
                    eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: preCR.clone(), ty: ty.clone() }), scalar: Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() }), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                    vars = if (preUsed.clone()) {BackendVariable::addVar(preVar.clone(), vars.clone())?} else {vars.clone()};
                    eqns = if (preUsed.clone()) {BackendEquation::add(eqn.clone(), eqns.clone())?} else {eqns.clone()};
                    Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), hs.clone())))
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

// =============================================================================
// section for collecting initial vars/eqns
//
// =============================================================================
fn collectInitialVarsEqnsSystem(mut eqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, mut vars: BackendDAE::Variables, mut fixVars: BackendDAE::Variables, mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut reEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), mut allPrimaryParams: Arc<AvlSetCR::Tree>, mut datareconFlag: bool) -> Result<(BackendDAE::Variables, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)> {
    let mut vars: BackendDAE::Variables = vars;
    let mut fixVars: BackendDAE::Variables = fixVars;
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = eqns;
    let mut reEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = reEqns;
    let mut stateSetFixCounts: metamodelica::Array<i32>;
    for mut eq in &*eqSystems.clone() {
        let mut eq = eq.clone();
        let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ BackendDAE::EqSystem { partitionKind: BackendDAE::BaseClockPartitionKind::CLOCKED_PARTITION { .. }, .. } => {
            (vars, eqns) = BackendVariable::traverseBackendDAEVars(eq.orderedVars.clone(), (std::sync::Arc::new(collectInitialClockedVarsEqns) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)) -> Result<(BackendDAE::Var, (BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>))> + 'static>), (vars.clone(), eqns.clone()))?;
            ()
        },
        _ => {
            stateSetFixCounts = arrayCreate((eq.stateSets.clone().len() as i32), 0);
            (vars, fixVars, eqns, stateSetFixCounts, _, _, _) = BackendVariable::traverseBackendDAEVars(eq.orderedVars.clone(), (std::sync::Arc::new(collectInitialVars) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (BackendDAE::Variables, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, metamodelica::Array<i32>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), Arc<AvlSetCR::Tree>, bool)) -> Result<(BackendDAE::Var, (BackendDAE::Variables, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, metamodelica::Array<i32>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), Arc<AvlSetCR::Tree>, bool))> + 'static>), (vars.clone(), fixVars.clone(), eqns.clone(), stateSetFixCounts.clone(), hs.clone(), allPrimaryParams.clone(), datareconFlag.clone()))?;
            (eqns, reEqns) = BackendEquation::traverseEquationArray(eq.orderedEqs.clone(), (std::sync::Arc::new(collectInitialEqns) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)) -> Result<(Arc<BackendDAE::Equation>, (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>))> + 'static>), (eqns.clone(), reEqns.clone()))?;
            if Flags::getConfigBool(Flags::INITIAL_STATE_SELECTION.clone())? {
                (vars, eqns) = collectInitialStateSets(eq.stateSets.clone(), stateSetFixCounts.clone(), vars.clone(), eqns.clone())?;
            }
            GCExt::free(stateSetFixCounts.clone());
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((vars, fixVars, eqns, reEqns))
}

fn collectInitialStateSets(mut stateSets: Arc<metamodelica::List<BackendDAE::StateSet>>, mut stateSetFixCounts: metamodelica::Array<i32>, mut iVars: BackendDAE::Variables, mut iEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<(BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)> {
    let mut oVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut oEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut stateSet: BackendDAE::StateSet = <BackendDAE::StateSet as ::std::default::Default>::default();
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut Feqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut initEqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut lhs: Arc<DAE::Exp>;
    let mut rhs: Arc<DAE::Exp>;
    let mut exp: Arc<DAE::Exp>;
    let mut expcrF: Arc<DAE::Exp>;
    let mut expInitset: Arc<DAE::Exp>;
    let mut mulFstates: Arc<DAE::Exp>;
    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut expcrstates: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut expcrInitset: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut crLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut crInitSet: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut set: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut crF: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut crInitStates: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut fixState: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut statesToFix: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut unfixedStates: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut VarsF: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut oInitSetVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut tyExpCrStates: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut toFix: i32 = 0;
    let mut setsize: i32 = 0;
    let mut nCandidates: i32 = 0;
    let mut recordSize: Option<i32> = None;
    let mut b: bool = false;
    let mut op: DAE::Operator;
    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    (oVars, oEqns) = (iVars.clone(), iEqns.clone());
    for mut stateSet in &*stateSets.clone() {
        let mut stateSet = stateSet.clone();
        oVars = BackendVariable::addVars(stateSet.varA.clone(), oVars.clone());
        lhs = Expression::crefToExp(stateSet.crA.clone())?;
        tp = ComponentReference::crefTypeFull(stateSet.crA.clone())?;
        tp = DAEUtil::expTypeElementType(tp.clone());
        if DAEUtil::expTypeComplex(tp.clone()) {
            recordSize = Some(Expression::sizeOf(tp.clone())?);
        } else {
            recordSize = None;
        }
        expLst = metamodelica::nil();
        crLst = SymbolicJacobian::getJacobianDependencies(stateSet.jacobian.clone())?;
        expLst = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut cr in (crLst.clone()).into_iter().cloned() {
            let __x = Expression::crefToExp(cr.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        expLst = cons(Arc::new(DAE::Exp::ICONST { integer: stateSet.index.clone() - 1 }), expLst.clone());
        rhs = Arc::new(DAE::Exp::CALL { attr: DAE::callAttrBuiltinOther().clone(), expLst: expLst.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("$stateSelectionSet")).clone() }) });
        eqn = Arc::new(BackendDAE::Equation::ARRAY_EQUATION { recordSize: recordSize.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone(), source: DAE::emptyElementSource().clone(), right: rhs.clone(), left: lhs.clone(), dimSize: list![(stateSet.varA.clone().len() as i32)] });
        (oEqns, _) = ExpandableArray::add(eqn.clone(), oEqns.clone())?;
        if Flags::isSet(Flags::BLT_DUMP.clone())? || Flags::isSet(Flags::INITIALIZATION.clone())? {
            BackendDump::dumpEquationList(list![eqn.clone()], (literal!("initial state selection equation generated:")).clone());
        }
        if (stateSetFixCounts.clone().borrow().len() as i32) >= stateSet.index.clone() && stateSetFixCounts.clone().borrow()[(stateSet.index.clone()-1) as usize].clone() > 0 {
            unfixedStates = metamodelica::nil();
            for mut state in &*stateSet.statescandidates.clone() {
                let mut state = state.clone();
                if !(BackendVariable::varFixed(state.clone())) {
                    unfixedStates = cons(state.clone(), unfixedStates.clone());
                }
            }
            toFix = stateSetFixCounts.clone().borrow()[(stateSet.index.clone()-1) as usize].clone();
            statesToFix = metamodelica::nil();
            statesToFix = SymbolicJacobian::getFixedStatesForSelfdependentSets(stateSet.clone(), unfixedStates.clone(), toFix.clone())?;
            for mut state in &*statesToFix.clone() {
                let mut state = state.clone();
                lhs = Expression::crefToExp(state.varName.clone())?;
                rhs = IndexReduction::makeStartExp(state.varName.clone())?;
                initEqn = Arc::new(BackendDAE::Equation::EQUATION { attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone(), source: DAE::emptyElementSource().clone(), scalar: rhs.clone(), exp: lhs.clone() });
                (oEqns, _) = ExpandableArray::add(initEqn.clone(), oEqns.clone())?;
            }
            if Flags::isSet(Flags::BLT_DUMP.clone())? || Flags::isSet(Flags::INITIALIZATION.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("StateSet ")); __mm_s.push_str(&*intString(stateSet.index.clone())); __mm_s.push_str(&*literal!(" is underconstraint for the initial system.\n")); ArcStr::from(__mm_s) }).clone());
                println!("{}", (literal!("======================================\n")).clone());
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("# States left to fix: ")); __mm_s.push_str(&*intString(toFix.clone())); __mm_s.push_str(&*literal!(".\n")); ArcStr::from(__mm_s) }).clone());
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("# Unfixed candidates: ")); __mm_s.push_str(&*intString((stateSet.statescandidates.clone().len() as i32) - toFix.clone())); __mm_s.push_str(&*literal!(".\n")); ArcStr::from(__mm_s) }).clone());
                BackendDump::dumpVarList(statesToFix.clone(), (literal!("Chosen states to fix:")).clone());
            }
        }
    }
    Ok((oVars, oEqns))
}

fn collectInitialVars(mut inVar: BackendDAE::Var, mut inTpl: (BackendDAE::Variables, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, metamodelica::Array<i32>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), Arc<AvlSetCR::Tree>, bool)) -> Result<(BackendDAE::Var, (BackendDAE::Variables, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, metamodelica::Array<i32>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), Arc<AvlSetCR::Tree>, bool))> {
    let mut outVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut outTpl: (BackendDAE::Variables, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, metamodelica::Array<i32>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), Arc<AvlSetCR::Tree>, bool);
    (outVar, outTpl) = 'mc: {
        let __mc_input = (inVar.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ BackendDAE::Var { varType: ty, varKind: BackendDAE::VarKind::STATE { .. }, varName: cr, .. }, (vars, fixvars, eqns, stateSetFixCounts, hs, allPrimaryParameters, datarecon)) => {
                    let mut preVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut derVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut startVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut preCR: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut derCR: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut startCR: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut isFixed: bool = false;
                    let mut preUsed: bool = false;
                    let mut startExp: Arc<DAE::Exp>;
                    let mut crefExp: Arc<DAE::Exp>;
                    let mut stateSetIdxString: ArcStr = arcstr::literal!("");
                    let mut stateSetSplit: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut stateSetIdx: i32 = 0;
                    let mut parameters: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut var = (*var).clone();
                    let mut vars = (*vars).clone();
                    let mut eqns = (*eqns).clone();
                    isFixed = BackendVariable::varFixed(var.clone());
                    preUsed = BaseHashSet::has(cr.clone(), hs.clone())?;
                    crefExp = Expression::crefExp(cr.clone())?;
                    startCR = ComponentReference::crefPrefixStart(cr.clone());
                    startVar = BackendVariable::copyVarNewName(startCR.clone(), var.clone());
                    startVar = BackendVariable::setBindExp(startVar.clone(), None);
                    startVar = BackendVariable::setVarDirection(startVar.clone(), openmodelica_frontend_types::DAE::VarDirection::BIDIR);
                    startVar = BackendVariable::setVarFixed(startVar.clone(), false)?;
                    startVar = BackendVariable::setVarKind(startVar.clone(), crate::BackendDAE::VarKind::VARIABLE)?;
                    startVar = BackendVariable::setVarStartValueOption(startVar.clone(), None)?;
                    startExp = BackendVariable::varStartValue(var.clone())?;
                    parameters = Expression::getAllCrefs(startExp.clone())?;
                    if !({
        let mut __acc: Option<bool> = None;
        for mut p in (parameters.clone()).into_iter().cloned() {
                    let __x = AvlSetCR::hasKey(allPrimaryParameters.clone(), p.clone())?;
                    __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty min reduction"))?
    }) {
                        eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: Expression::crefExp(startCR.clone())?, scalar: startExp.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                        eqns = BackendEquation::add(eqn.clone(), eqns.clone())?;
                        vars = BackendVariable::addVar(startVar.clone(), vars.clone())?;
                    }
                    if isFixed.clone() {
                        if StringUtil::startsWith((ComponentReferenceBasics::crefFirstIdent(cr.clone())?).clone(), (literal!("$STATESET")).clone()) && Flags::getConfigBool(Flags::INITIAL_STATE_SELECTION.clone())? {
                            stateSetSplit = Util::stringSplitAtChar((ComponentReferenceBasics::crefFirstIdent(cr.clone())?).clone(), (literal!(".")).clone())?;
                            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(stateSetSplit.clone()) {
                                        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                                        _ => bail!("pattern mismatch"),
                            } };
                            stateSetIdxString = __pa0.clone();
                            stateSetSplit = __pa1.clone();
                            stateSetIdxString = substring((stateSetIdxString.clone()).clone(), 10, ((stateSetIdxString.clone()).clone().len() as i32))?;
                            stateSetIdx = stringInt((stateSetIdxString.clone()).clone())?;
                            {let _arr = stateSetFixCounts.clone(); let _val = stateSetFixCounts.clone().borrow()[(stateSetIdx.clone()-1) as usize].clone() + 1; _arr.borrow_mut()[(stateSetIdx.clone()-1) as usize] = _val; _arr};
                        } else {
                            if Expression::isConstValue(startExp.clone())? {
                                        eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: crefExp.clone(), scalar: Expression::crefExp(startCR.clone())?, source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                            } else {
                                        eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: crefExp.clone(), scalar: startExp.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                            }
                            eqns = BackendEquation::add(eqn.clone(), eqns.clone())?;
                        }
                    }
                    var = BackendVariable::setVarKind(var.clone(), crate::BackendDAE::VarKind::VARIABLE)?;
                    derCR = ComponentReference::crefPrefixDer(cr.clone());
                    derVar = BackendVariable::copyVarNewName(derCR.clone(), var.clone());
                    derVar = BackendVariable::setVarDirection(derVar.clone(), openmodelica_frontend_types::DAE::VarDirection::BIDIR);
                    derVar = BackendVariable::setBindExp(derVar.clone(), None);
                    preCR = ComponentReference::crefPrefixPre(cr.clone());
                    preVar = BackendVariable::copyVarNewName(preCR.clone(), var.clone());
                    preVar = BackendVariable::setVarDirection(preVar.clone(), openmodelica_frontend_types::DAE::VarDirection::BIDIR);
                    preVar = BackendVariable::setBindExp(preVar.clone(), None);
                    preVar = BackendVariable::setVarFixed(preVar.clone(), true)?;
                    preVar = BackendVariable::setVarStartValueOption(preVar.clone(), Some(Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() })))?;
                    eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() }), scalar: Arc::new(DAE::Exp::CREF { componentRef: preCR.clone(), ty: ty.clone() }), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                    vars = BackendVariable::addVar(derVar.clone(), vars.clone())?;
                    vars = BackendVariable::addVar(var.clone(), vars.clone())?;
                    vars = if (preUsed.clone()) {BackendVariable::addVar(preVar.clone(), vars.clone())?} else {vars.clone()};
                    eqns = if (preUsed.clone()) {BackendEquation::add(eqn.clone(), eqns.clone())?} else {eqns.clone()};
                    Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), stateSetFixCounts.clone(), hs.clone(), allPrimaryParameters.clone(), datarecon.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ BackendDAE::Var { varType: ty, varKind: BackendDAE::VarKind::DISCRETE, varName: cr, .. }, (vars, fixvars, eqns, stateSetFixCounts, hs, allPrimaryParameters, datarecon)) => {
                    let mut preVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut preCR: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut startValue_: Arc<DAE::Exp>;
                    let mut var = (*var).clone();
                    let mut vars = (*vars).clone();
                    let mut eqns = (*eqns).clone();
                    let true = (BaseHashSet::has(cr.clone(), hs.clone())?) else { bail!("pattern mismatch") };
                    let true = (BackendVariable::varFixed(var.clone())) else { bail!("pattern mismatch") };
                    startValue_ = BackendVariable::varStartValue(var.clone())?;
                    var = BackendVariable::setVarFixed(var.clone(), false)?;
                    preCR = ComponentReference::crefPrefixPre(cr.clone());
                    preVar = BackendVariable::copyVarNewName(preCR.clone(), var.clone());
                    preVar = BackendVariable::setVarDirection(preVar.clone(), openmodelica_frontend_types::DAE::VarDirection::BIDIR);
                    preVar = BackendVariable::setBindExp(preVar.clone(), None);
                    preVar = BackendVariable::setVarFixed(preVar.clone(), false)?;
                    preVar = BackendVariable::setVarStartValueOption(preVar.clone(), Some(startValue_.clone()))?;
                    eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: preCR.clone(), ty: ty.clone() }), scalar: startValue_.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                    vars = BackendVariable::addVar(var.clone(), vars.clone())?;
                    vars = BackendVariable::addVar(preVar.clone(), vars.clone())?;
                    eqns = BackendEquation::add(eqn.clone(), eqns.clone())?;
                    Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), stateSetFixCounts.clone(), hs.clone(), allPrimaryParameters.clone(), datarecon.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ BackendDAE::Var { varKind: BackendDAE::VarKind::DISCRETE, varName: cr, .. }, (vars, fixvars, eqns, stateSetFixCounts, hs, allPrimaryParameters, datarecon)) => {
                    let mut preVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut preCR: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut preUsed: bool = false;
                    let mut startValue: Option<Arc<DAE::Exp>> = None;
                    let mut var = (*var).clone();
                    let mut vars = (*vars).clone();
                    preUsed = BaseHashSet::has(cr.clone(), hs.clone())?;
                    startValue = BackendVariable::varStartValueOption(var.clone())?;
                    var = BackendVariable::setVarFixed(var.clone(), false)?;
                    preCR = ComponentReference::crefPrefixPre(cr.clone());
                    preVar = BackendVariable::copyVarNewName(preCR.clone(), var.clone());
                    preVar = BackendVariable::setVarDirection(preVar.clone(), openmodelica_frontend_types::DAE::VarDirection::BIDIR);
                    preVar = BackendVariable::setBindExp(preVar.clone(), None);
                    preVar = BackendVariable::setVarFixed(preVar.clone(), false)?;
                    preVar = BackendVariable::setVarStartValueOption(preVar.clone(), startValue.clone())?;
                    vars = BackendVariable::addVar(var.clone(), vars.clone())?;
                    vars = if (preUsed.clone()) {BackendVariable::addVar(preVar.clone(), vars.clone())?} else {vars.clone()};
                    Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), stateSetFixCounts.clone(), hs.clone(), allPrimaryParameters.clone(), datarecon.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ BackendDAE::Var { bindExp: None, varKind: BackendDAE::VarKind::PARAM, varName: cr, .. }, (vars, fixvars, eqns, stateSetFixCounts, hs, allPrimaryParameters, datarecon)) => {
                    let mut startExp: Arc<DAE::Exp>;
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    let mut var = (*var).clone();
                    let true = (BackendVariable::varFixed(var.clone())) else { bail!("pattern mismatch") };
                    startExp = BackendVariable::varStartValueType(var.clone())?;
                    s = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
                    r#str = (ExpressionBasics::printExpStr(startExp.clone())?).clone();
                    var = BackendVariable::setVarKind(var.clone(), crate::BackendDAE::VarKind::VARIABLE)?;
                    var = BackendVariable::setBindExp(var.clone(), Some(startExp.clone()));
                    var = BackendVariable::setVarFixed(var.clone(), true)?;
                    info = ElementSource::getElementSourceFileInfo(BackendVariable::getVarSource(var.clone()));
                    Error::addSourceMessage(Error::UNBOUND_PARAMETER_WITH_START_VALUE_WARNING.clone(), list![(s.clone()).clone(), (r#str.clone()).clone()], info.clone())?;
                    Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), stateSetFixCounts.clone(), hs.clone(), allPrimaryParameters.clone(), datarecon.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ BackendDAE::Var { varType: ty, bindExp: Some(bindExp), varKind: BackendDAE::VarKind::PARAM, varName: cr, .. }, (vars, fixvars, eqns, stateSetFixCounts, hs, allPrimaryParameters, datarecon)) => {
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    let mut var = (*var).clone();
                    let mut vars = (*vars).clone();
                    let mut eqns = (*eqns).clone();
                    let true = (intGt(Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, 31)) else { bail!("pattern mismatch") };
                    let false = (BackendVariable::varFixed(var.clone())) else { bail!("pattern mismatch") };
                    var = BackendVariable::setVarKind(var.clone(), crate::BackendDAE::VarKind::VARIABLE)?;
                    var = BackendVariable::setBindExp(var.clone(), None);
                    s = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
                    r#str = (ExpressionBasics::printExpStr(bindExp.clone())?).clone();
                    info = ElementSource::getElementSourceFileInfo(BackendVariable::getVarSource(var.clone()));
                    Error::addSourceMessage(Error::UNFIXED_PARAMETER_WITH_BINDING.clone(), list![(s.clone()).clone(), (s.clone()).clone(), (r#str.clone()).clone()], info.clone())?;
                    eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() }), scalar: bindExp.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                    eqns = BackendEquation::add(eqn.clone(), eqns.clone())?;
                    vars = BackendVariable::addVar(var.clone(), vars.clone())?;
                    Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), stateSetFixCounts.clone(), hs.clone(), allPrimaryParameters.clone(), datarecon.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ BackendDAE::Var { bindExp: Some(bindExp), varKind: BackendDAE::VarKind::PARAM, varName: cr, .. }, (vars, fixvars, eqns, stateSetFixCounts, hs, allPrimaryParameters, datarecon)) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    let mut var = (*var).clone();
                    let mut vars = (*vars).clone();
                    let true = (intLe(Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, 31)) else { bail!("pattern mismatch") };
                    let false = (BackendVariable::varFixed(var.clone())) else { bail!("pattern mismatch") };
                    var = BackendVariable::setVarKind(var.clone(), crate::BackendDAE::VarKind::VARIABLE)?;
                    var = BackendVariable::setBindExp(var.clone(), None);
                    ::match_deref::match_deref! { match &(BackendVariable::varStartValueOption(var.clone())?) {
                        None => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    var = BackendVariable::setVarStartValue(var.clone(), bindExp.clone())?;
                    s = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
                    r#str = (ExpressionBasics::printExpStr(bindExp.clone())?).clone();
                    info = ElementSource::getElementSourceFileInfo(BackendVariable::getVarSource(var.clone()));
                    Error::addSourceMessage(Error::UNFIXED_PARAMETER_WITH_BINDING_31.clone(), list![(s.clone()).clone(), (s.clone()).clone(), (r#str.clone()).clone()], info.clone())?;
                    vars = BackendVariable::addVar(var.clone(), vars.clone())?;
                    Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), stateSetFixCounts.clone(), hs.clone(), allPrimaryParameters.clone(), datarecon.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ BackendDAE::Var { bindExp: Some(bindExp), varKind: BackendDAE::VarKind::PARAM, varName: cr, .. }, (vars, fixvars, eqns, stateSetFixCounts, hs, allPrimaryParameters, datarecon)) => {
                    let mut startExp: Arc<DAE::Exp>;
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut sv: ArcStr = arcstr::literal!("");
                    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    let mut var = (*var).clone();
                    let mut vars = (*vars).clone();
                    let true = (intLe(Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, 31)) else { bail!("pattern mismatch") };
                    let false = (BackendVariable::varFixed(var.clone())) else { bail!("pattern mismatch") };
                    var = BackendVariable::setVarKind(var.clone(), crate::BackendDAE::VarKind::VARIABLE)?;
                    var = BackendVariable::setBindExp(var.clone(), None);
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::varStartValueOption(var.clone())?) {
                        Some(__pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    startExp = __pa0.clone();
                    s = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
                    r#str = (ExpressionBasics::printExpStr(bindExp.clone())?).clone();
                    sv = (ExpressionBasics::printExpStr(startExp.clone())?).clone();
                    info = ElementSource::getElementSourceFileInfo(BackendVariable::getVarSource(var.clone()));
                    Error::addSourceMessage(Error::UNFIXED_PARAMETER_WITH_BINDING_AND_START_VALUE_31.clone(), list![(s.clone()).clone(), (sv.clone()).clone(), (s.clone()).clone(), (r#str.clone()).clone()], info.clone())?;
                    vars = BackendVariable::addVar(var.clone(), vars.clone())?;
                    Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), stateSetFixCounts.clone(), hs.clone(), allPrimaryParameters.clone(), datarecon.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ BackendDAE::Var { varKind: BackendDAE::VarKind::PARAM, .. }, (vars, fixvars, eqns, stateSetFixCounts, hs, allPrimaryParameters, datarecon)) => {
                    let mut var = (*var).clone();
                    let mut vars = (*vars).clone();
                    var = BackendVariable::setVarKind(var.clone(), crate::BackendDAE::VarKind::VARIABLE)?;
                    vars = BackendVariable::addVar(var.clone(), vars.clone())?;
                    Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), stateSetFixCounts.clone(), hs.clone(), allPrimaryParameters.clone(), datarecon.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ BackendDAE::Var { varKind: BackendDAE::VarKind::EXTOBJ { .. }, .. }, (vars, fixvars, eqns, stateSetFixCounts, hs, allPrimaryParameters, datarecon)) => {
                    let mut vars = (*vars).clone();
                    vars = BackendVariable::addVar(var.clone(), vars.clone())?;
                    Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), stateSetFixCounts.clone(), hs.clone(), allPrimaryParameters.clone(), datarecon.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ BackendDAE::Var { varKind: BackendDAE::VarKind::CONST, .. }, _) => {
                    Ok((var.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ BackendDAE::Var { varType: ty, varName: cr, .. }, (vars, fixvars, eqns, stateSetFixCounts, hs, allPrimaryParameters, datarecon)) => {
                    let mut preVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut startVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut preCR: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut startCR: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut isInput: bool = false;
                    let mut preUsed: bool = false;
                    let mut startExp: Arc<DAE::Exp>;
                    let mut parameters: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut var = (*var).clone();
                    let mut vars = (*vars).clone();
                    let mut fixvars = (*fixvars).clone();
                    let mut eqns = (*eqns).clone();
                    let true = (BackendVariable::varFixed(var.clone())) else { bail!("pattern mismatch") };
                    if datarecon.clone() {
                        isInput = checkComponentNames(var.varDirection.clone(), cr.clone());
                    } else {
                        isInput = BackendVariable::isVarOnTopLevelAndInput(var.clone());
                    }
                    preUsed = BaseHashSet::has(cr.clone(), hs.clone())?;
                    let _ = Expression::crefExp(cr.clone())?;
                    startCR = ComponentReference::crefPrefixStart(cr.clone());
                    startVar = BackendVariable::copyVarNewName(startCR.clone(), var.clone());
                    startVar = BackendVariable::setBindExp(startVar.clone(), None);
                    startVar = BackendVariable::setVarDirection(startVar.clone(), openmodelica_frontend_types::DAE::VarDirection::BIDIR);
                    startVar = BackendVariable::setVarFixed(startVar.clone(), false)?;
                    startVar = BackendVariable::setVarKind(startVar.clone(), crate::BackendDAE::VarKind::VARIABLE)?;
                    startVar = BackendVariable::setVarStartValueOption(startVar.clone(), None)?;
                    startExp = BackendVariable::varStartValue(var.clone())?;
                    parameters = Expression::getAllCrefs(startExp.clone())?;
                    if !({
        let mut __acc: Option<bool> = None;
        for mut p in (parameters.clone()).into_iter().cloned() {
                    let __x = AvlSetCR::hasKey(allPrimaryParameters.clone(), p.clone())?;
                    __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty min reduction"))?
    }) {
                        eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: Expression::crefExp(startCR.clone())?, scalar: startExp.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                        eqns = BackendEquation::add(eqn.clone(), eqns.clone())?;
                        vars = BackendVariable::addVar(startVar.clone(), vars.clone())?;
                    }
                    var = BackendVariable::setVarFixed(var.clone(), false)?;
                    preCR = ComponentReference::crefPrefixPre(cr.clone());
                    preVar = BackendVariable::copyVarNewName(preCR.clone(), var.clone());
                    preVar = BackendVariable::setVarDirection(preVar.clone(), openmodelica_frontend_types::DAE::VarDirection::BIDIR);
                    preVar = BackendVariable::setBindExp(preVar.clone(), None);
                    preVar = BackendVariable::setVarFixed(preVar.clone(), true)?;
                    preVar = BackendVariable::setVarStartValueOption(preVar.clone(), Some(Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() })))?;
                    if Expression::isConstValue(startExp.clone())? {
                        eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() }), scalar: Expression::crefExp(startCR.clone())?, source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                    } else {
                        eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() }), scalar: startExp.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                    }
                    vars = if (!(isInput.clone())) {BackendVariable::addVar(var.clone(), vars.clone())?} else {vars.clone()};
                    fixvars = if (isInput.clone()) {BackendVariable::addVar(var.clone(), fixvars.clone())?} else {fixvars.clone()};
                    vars = if (preUsed.clone()) {BackendVariable::addVar(preVar.clone(), vars.clone())?} else {vars.clone()};
                    eqns = BackendEquation::add(eqn.clone(), eqns.clone())?;
                    Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), stateSetFixCounts.clone(), hs.clone(), allPrimaryParameters.clone(), datarecon.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ BackendDAE::Var { varType: ty, varName: cr, .. }, (vars, fixvars, eqns, stateSetFixCounts, hs, allPrimaryParameters, datarecon)) => {
                    let mut preVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut startVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut preCR: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut startCR: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut isInput: bool = false;
                    let mut preUsed: bool = false;
                    let mut startExp: Arc<DAE::Exp>;
                    let mut parameters: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut vars = (*vars).clone();
                    let mut fixvars = (*fixvars).clone();
                    let mut eqns = (*eqns).clone();
                    let false = (BackendVariable::varFixed(var.clone())) else { bail!("pattern mismatch") };
                    if datarecon.clone() {
                        isInput = checkComponentNames(var.varDirection.clone(), cr.clone());
                    } else {
                        isInput = BackendVariable::isVarOnTopLevelAndInput(var.clone());
                    }
                    preUsed = BaseHashSet::has(cr.clone(), hs.clone())?;
                    let _ = Expression::crefExp(cr.clone())?;
                    startCR = ComponentReference::crefPrefixStart(cr.clone());
                    startVar = BackendVariable::copyVarNewName(startCR.clone(), var.clone());
                    startVar = BackendVariable::setBindExp(startVar.clone(), None);
                    startVar = BackendVariable::setVarDirection(startVar.clone(), openmodelica_frontend_types::DAE::VarDirection::BIDIR);
                    startVar = BackendVariable::setVarFixed(startVar.clone(), false)?;
                    startVar = BackendVariable::setVarKind(startVar.clone(), crate::BackendDAE::VarKind::VARIABLE)?;
                    startVar = BackendVariable::setVarStartValueOption(startVar.clone(), None)?;
                    startExp = BackendVariable::varStartValue(var.clone())?;
                    parameters = Expression::getAllCrefs(startExp.clone())?;
                    if !({
        let mut __acc: Option<bool> = None;
        for mut p in (parameters.clone()).into_iter().cloned() {
                    let __x = AvlSetCR::hasKey(allPrimaryParameters.clone(), p.clone())?;
                    __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty min reduction"))?
    }) {
                        eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: Expression::crefExp(startCR.clone())?, scalar: startExp.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                        eqns = BackendEquation::add(eqn.clone(), eqns.clone())?;
                        vars = BackendVariable::addVar(startVar.clone(), vars.clone())?;
                    }
                    preCR = ComponentReference::crefPrefixPre(cr.clone());
                    preVar = BackendVariable::copyVarNewName(preCR.clone(), var.clone());
                    preVar = BackendVariable::setVarDirection(preVar.clone(), openmodelica_frontend_types::DAE::VarDirection::BIDIR);
                    preVar = BackendVariable::setBindExp(preVar.clone(), None);
                    preVar = BackendVariable::setVarFixed(preVar.clone(), true)?;
                    preVar = BackendVariable::setVarStartValueOption(preVar.clone(), Some(Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() })))?;
                    if Expression::isConstValue(startExp.clone())? {
                        eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: preCR.clone(), ty: ty.clone() }), scalar: Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() }), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                    } else {
                        eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: preCR.clone(), ty: ty.clone() }), scalar: startExp.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                    }
                    vars = if (!(isInput.clone())) {BackendVariable::addVar(var.clone(), vars.clone())?} else {vars.clone()};
                    fixvars = if (isInput.clone()) {BackendVariable::addVar(var.clone(), fixvars.clone())?} else {fixvars.clone()};
                    vars = if (preUsed.clone()) {BackendVariable::addVar(preVar.clone(), vars.clone())?} else {vars.clone()};
                    eqns = if (preUsed.clone()) {BackendEquation::add(eqn.clone(), eqns.clone())?} else {eqns.clone()};
                    Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), stateSetFixCounts.clone(), hs.clone(), allPrimaryParameters.clone(), datarecon.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function collectInitialVars failed for: ")); __mm_s.push_str(&*BackendDump::varString(inVar.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVar, outTpl))
}

fn checkComponentNames(mut inVarDirection: DAE::VarDirection, mut inComponentRef: Arc<DAE::ComponentRef>) -> bool {
    let mut isTopLevel: bool = false;
    isTopLevel = (::match_deref::match_deref! { match &((inVarDirection.clone(), inComponentRef.clone())) {
        (DAE::VarDirection::INPUT, Deref @ DAE::ComponentRef::CREF_IDENT { .. }) => true,
        (DAE::VarDirection::INPUT, Deref @ DAE::ComponentRef::CREF_QUAL { .. }) => true,
        (_, _) => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isTopLevel
}

fn collectInitialClockedVarsEqns(mut inVar: BackendDAE::Var, mut inTpl: (BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)) -> Result<(BackendDAE::Var, (BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>))> {
    let mut outVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut outTpl: (BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>);
    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    (vars, eqns) = inTpl.clone();
    (outVar, outTpl) = (match inVar.clone() {
        ref var @ BackendDAE::Var { varKind: ref kind, varType: ref ty, varName: ref cr, .. } => {
            let mut crExp: Arc<DAE::Exp>;
            let mut startExp: Arc<DAE::Exp>;
            crExp = Expression::crefExp(cr.clone())?;
            (vars, eqns) = (match kind.clone() {
        BackendDAE::VarKind::CLOCKED_STATE { previousName: ref previousCR, .. } => {
            let mut previousVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
            let mut previousExp: Arc<DAE::Exp>;
            previousVar = BackendVariable::copyVarNewName(previousCR.clone(), var.clone());
            previousVar = BackendVariable::setVarKind(previousVar.clone(), crate::BackendDAE::VarKind::VARIABLE)?;
            previousVar = BackendVariable::setVarDirection(previousVar.clone(), openmodelica_frontend_types::DAE::VarDirection::BIDIR);
            previousVar = BackendVariable::setBindExp(previousVar.clone(), None);
            previousVar = BackendVariable::setVarFixed(previousVar.clone(), true)?;
            previousVar = BackendVariable::setVarStartValueOption(previousVar.clone(), Some(Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() })))?;
            previousExp = Expression::crefExp(previousCR.clone())?;
            vars = BackendVariable::addVar(previousVar.clone(), vars.clone())?;
            eqns = BackendEquation::add(Arc::new(BackendDAE::Equation::EQUATION { exp: previousExp.clone(), scalar: crExp.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() }), eqns.clone())?;
            startExp = BackendVariable::varStartValue(var.clone())?;
            vars = BackendVariable::addVar(var.clone(), vars.clone())?;
            eqns = BackendEquation::add(Arc::new(BackendDAE::Equation::EQUATION { exp: crExp.clone(), scalar: startExp.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() }), eqns.clone())?;
            (vars.clone(), eqns.clone())
        },
        _ => {
            (vars.clone(), eqns.clone())
        },
    });
            (var.clone(), (vars.clone(), eqns.clone()))
        },
        _ => bail!("match: no arm matched"),
    });
    Ok((outVar, outTpl))
}

fn collectInitialEqns(mut inEq: Arc<BackendDAE::Equation>, mut inTpl: (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)) -> Result<(Arc<BackendDAE::Equation>, (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>))> {
    let mut outEq: Arc<BackendDAE::Equation> = inEq.clone();
    let mut outTpl: (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>);
    let mut previousHS: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    let mut eqn1: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut reeqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut size: i32 = 0;
    let mut b: bool = false;
    (eqns, reeqns) = inTpl.clone();
    (eqn1, _) = BackendEquation::traverseExpsOfEquation(inEq.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsDummyHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>>) -> Result<(Arc<DAE::Exp>, fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>>)> + 'static>), (std::sync::Arc::new(fnptr!(replaceDerPreCref, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?;
    size = BackendEquation::equationSize(eqn1.clone())?;
    b = intGt(size.clone(), 0);
    eqns = if (b.clone()) {BackendEquation::add(eqn1.clone(), eqns.clone())?} else {eqns.clone()};
    reeqns = if (!(b.clone())) {BackendEquation::add(eqn1.clone(), reeqns.clone())?} else {reeqns.clone()};
    outTpl = (eqns.clone(), reeqns.clone());
    Ok((outEq, outTpl))
}

fn replaceDerPreCref(mut inExp: Arc<DAE::Exp>) -> Arc<DAE::Exp> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty, .. }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" } } => {
            let mut dummyder: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            dummyder = ComponentReference::crefPrefixDer(cr.clone());
            Arc::new(DAE::Exp::CREF { componentRef: dummyder.clone(), ty: ty.clone() })
        },
        Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty, .. }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" } } => {
            let mut dummyder: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            dummyder = ComponentReference::crefPrefixPre(cr.clone());
            Arc::new(DAE::Exp::CREF { componentRef: dummyder.clone(), ty: ty.clone() })
        },
        Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty, .. }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" } } => {
            let mut dummyder: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            dummyder = ComponentReference::crefPrefixPrevious(cr.clone());
            Arc::new(DAE::Exp::CREF { componentRef: dummyder.clone(), ty: ty.clone() })
        },
        _ => {
            inExp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp
}

// =============================================================================
// section for bindings
//
// =============================================================================
fn collectInitialBindings(mut inVar: BackendDAE::Var, mut inTpl: (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)) -> Result<(BackendDAE::Var, (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>))> {
    let mut outVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut outTpl: (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>);
    (outVar, outTpl) = (::match_deref::match_deref! { match &((inVar.clone(), inTpl.clone())) {
        (var @ BackendDAE::Var { bindExp: None, .. }, _) => {
            (var.clone(), inTpl.clone())
        },
        (var @ BackendDAE::Var { source, varKind: BackendDAE::VarKind::EXTOBJ { .. }, bindExp: Some(bindExp), varName: cr, .. }, (eqns, reeqns)) => {
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut eqns = (*eqns).clone();
            eqn = Arc::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr.clone(), exp: bindExp.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
            eqns = BackendEquation::add(eqn.clone(), eqns.clone())?;
            (var.clone(), (eqns.clone(), reeqns.clone()))
        },
        (var @ BackendDAE::Var { source, varType: ty, bindExp: Some(bindExp), varName: cr, .. }, (eqns, reeqns)) => {
            let mut basic_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut crefExp: Arc<DAE::Exp>;
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut record_size: Option<i32> = None;
            let mut eqns = (*eqns).clone();
            crefExp = Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() });
            if Types::isArray(ty.clone()) {
                basic_ty = Types::getBasicType(ty.clone());
                record_size = if (Types::isRecord(basic_ty.clone())) {Some(Types::getDimensionProduct(basic_ty.clone())?)} else {None};
                eqn = Arc::new(BackendDAE::Equation::ARRAY_EQUATION { dimSize: Types::getDimensionSizes(ty.clone())?, left: crefExp.clone(), right: bindExp.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone(), recordSize: record_size.clone() });
            } else {
                eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: crefExp.clone(), scalar: bindExp.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
            }
            eqns = BackendEquation::add(eqn.clone(), eqns.clone())?;
            (var.clone(), (eqns.clone(), reeqns.clone()))
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function collectInitialBindings failed for: ")); __mm_s.push_str(&*BackendDump::varString(inVar.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outVar, outTpl))
}

// =============================================================================
// section for post-optimization module "removeInitializationStuff"
//
// =============================================================================
pub fn removeInitializationStuff(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = inDAE.clone();
    let mut removedEqsList: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared> = inDAE.shared.clone();
    for mut eqs in &*outDAE.eqs.clone() {
        let mut eqs = eqs.clone();
        let _ = BackendDAEUtil::traverseBackendDAEExpsEqns(eqs.orderedEqs.clone(), (std::sync::Arc::new(removeInitializationStuff1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false)?;
        let _ = BackendDAEUtil::traverseBackendDAEExpsEqns(eqs.removedEqs.clone(), (std::sync::Arc::new(removeInitializationStuff1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false)?;
    }
    let _ = BackendDAEUtil::traverseBackendDAEExpsEqns(shared.removedEqs.clone(), (std::sync::Arc::new(removeInitializationStuff1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false)?;
    for mut eq in &*BackendEquation::equationList(shared.removedEqs.clone()) {
        let mut eq = eq.clone();
        removedEqsList = (match BackendEquation::equationKind(eq.clone())? {
        BackendDAE::EquationKind::INITIAL_EQUATION => removedEqsList.clone(),
        _ => filterWhenEquation(eq.clone(), removedEqsList.clone())?,
    });
    }
    assign_field!(
        shared.removedEqs = BackendEquation::listEquation(removedEqsList.clone().reverse())?,
        shared.initialEqs = BackendEquation::emptyEqns()
    );
    assign_field!(outDAE.shared = shared.clone());
    Ok(outDAE)
}

fn filterWhenEquation(mut inEqn: Arc<BackendDAE::Equation>, mut inEqnLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut outEqnLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut condition: Arc<DAE::Exp>;
    outEqnLst = (::match_deref::match_deref! { match &(inEqn.clone()) {
        Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: Deref @ BackendDAE::WhenEquation { elsewhenPart: None, condition, .. }, .. } if ((BackendDAEUtil::getConditionList(condition.clone())?).0.is_empty()) => inEqnLst.clone(),
        _ => cons(inEqn.clone(), inEqnLst.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEqnLst)
}

fn removeInitializationStuff1(mut inExp: Arc<DAE::Exp>, mut inUseHomotopy: bool) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outUseHomotopy: bool = false;
    (outExp, outUseHomotopy) = Expression::traverseExpBottomUp(inExp.clone(), (std::sync::Arc::new(fnptr!(removeInitializationStuff2, Arc<DAE::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), inUseHomotopy.clone())?;
    Ok((outExp, outUseHomotopy))
}

fn removeInitializationStuff2(mut inExp: Arc<DAE::Exp>, mut inUseHomotopy: bool) -> (Arc<DAE::Exp>, bool) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outUseHomotopy: bool = false;
    (outExp, outUseHomotopy) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initial" }, .. } => {
            (Arc::new(DAE::Exp::BCONST { bool: false }), inUseHomotopy.clone())
        },
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: actual, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "homotopy" }, .. } => {
            (actual.clone(), true)
        },
        _ => {
            (inExp.clone(), inUseHomotopy.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outUseHomotopy)
}

// =============================================================================
// section for post-optimization module "replaceHomotopyWithSimplified"
//
// =============================================================================
pub fn replaceHomotopyWithSimplified(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = inDAE.clone();
    assign_field!(outDAE.eqs = {
        let mut __acc: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
        for mut eqs in (outDAE.eqs.clone()).into_iter().cloned() {
            let __x = replaceHomotopyWithSimplifiedEqs(eqs.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outDAE)
}

pub fn replaceHomotopyWithSimplifiedEqs(mut eqs: Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut eqs: Arc<BackendDAE::EqSystem> = eqs;
    let _ = BackendDAEUtil::traverseBackendDAEExpsEqns(eqs.orderedEqs.clone(), (std::sync::Arc::new(replaceHomotopyWithSimplified1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false)?;
    let _ = BackendDAEUtil::traverseBackendDAEExpsEqns(eqs.removedEqs.clone(), (std::sync::Arc::new(replaceHomotopyWithSimplified1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false)?;
    eqs = BackendDAEUtil::clearEqSyst(eqs.clone())?;
    Ok(eqs)
}

fn replaceHomotopyWithSimplified1(mut inExp: Arc<DAE::Exp>, mut inUseHomotopy: bool) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outUseHomotopy: bool = false;
    (outExp, outUseHomotopy) = Expression::traverseExpBottomUp(inExp.clone(), (std::sync::Arc::new(fnptr!(replaceHomotopyWithSimplified2, Arc<DAE::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), inUseHomotopy.clone())?;
    Ok((outExp, outUseHomotopy))
}

fn replaceHomotopyWithSimplified2(mut inExp: Arc<DAE::Exp>, mut inUseHomotopy: bool) -> (Arc<DAE::Exp>, bool) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outUseHomotopy: bool = false;
    (outExp, outUseHomotopy) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: simplified, tail: _ } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "homotopy" }, .. } => {
            (simplified.clone(), true)
        },
        _ => {
            (inExp.clone(), inUseHomotopy.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outUseHomotopy)
}

