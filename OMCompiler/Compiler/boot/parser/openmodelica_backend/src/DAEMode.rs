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
use crate::BackendDAEFunc;
use crate::BackendDAEOptimize;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVariable;
use crate::ExpressionSolve;
use crate::Initialization;
use crate::Matching;
use openmodelica_frontend::CheckModel;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionDump;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::FlagsUtil;
use openmodelica_util::Global;
use openmodelica_util::StackOverflow;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

pub fn getEqSystemDAEmode(mut inDAE: Arc<BackendDAE::BackendDAE>, mut fileNamePrefix: ArcStr, mut strPreOptModules: Option<Arc<metamodelica::List<ArcStr>>>, mut strmatchingAlgorithm: Option<ArcStr>, mut strdaeHandler: Option<ArcStr>, mut strPostOptModules: Option<Arc<metamodelica::List<ArcStr>>>) -> Result<(Arc<BackendDAE::BackendDAE>, Arc<BackendDAE::BackendDAE>, Option<Arc<BackendDAE::BackendDAE>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut outDAEmode: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut outInitDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut outInitDAE_lambda0_option: Option<Arc<BackendDAE::BackendDAE>> = None;
    let mut outRemovedInitialEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut dae: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut simDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut preOptModules: Arc<metamodelica::List<(BackendDAEFunc::optimizationModule, ArcStr)>> = metamodelica::nil();
    let mut postOptModules: Arc<metamodelica::List<(BackendDAEFunc::optimizationModule, ArcStr)>> = metamodelica::nil();
    let mut daeHandler: (BackendDAEFunc::StructurallySingularSystemHandlerFunc, ArcStr, BackendDAEFunc::stateDeselectionFunc, ArcStr);
    let mut matchingAlgorithm: (BackendDAEFunc::matchingAlgorithmFunc, ArcStr);
    let mut globalKnownVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut numCheckpoints: i32 = 0;
    let mut oldSize: i32 = 0;
    let mut eqSyst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    numCheckpoints = ErrorExt::getNumCheckpoints();
    if '__try0: {
        StackOverflow::clearStacktraceMessages();
        preOptModules = unwrap_break_err!(BackendDAEUtil::getPreOptModules(strPreOptModules.clone()), '__try0);
        postOptModules = unwrap_break_err!(BackendDAEUtil::getPostOptModules((::match_deref::match_deref! { match &(strPostOptModules.clone()) {
        None => Some(getPostOptModulesDAEString()?),
        _ => strPostOptModules.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })), '__try0);
        matchingAlgorithm = unwrap_break_err!(BackendDAEUtil::getMatchingAlgorithm(strmatchingAlgorithm.clone()), '__try0);
        unwrap_break_err!(FlagsUtil::setConfigString(Flags::INDEX_REDUCTION_METHOD.clone(), (literal!("dummyDerivatives")).clone()), '__try0);
        daeHandler = unwrap_break_err!(BackendDAEUtil::getIndexReductionMethod(strdaeHandler.clone()), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::DUMP_DAE_LOW.clone()), '__try0) {
            unwrap_break_err!(BackendDump::dumpBackendDAE(inDAE.clone(), (literal!("dumpdaelow")).clone()), '__try0);
            if unwrap_break_err!(Flags::isSet(Flags::ADDITIONAL_GRAPHVIZ_DUMP.clone()), '__try0) {
                unwrap_break_err!(BackendDump::graphvizAdjacencyMatrix(inDAE.clone(), (literal!("dumpdaelow")).clone()), '__try0);
            }
        }
        dae = unwrap_break_err!(BackendDAEUtil::preOptimizeDAE(inDAE.clone(), preOptModules.clone()), '__try0);
        unwrap_break_err!(execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("pre-optimization done (n=")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", BackendDAEUtil::daeSize(dae.clone())))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone()), '__try0);
        dae = unwrap_break_err!(BackendDAEUtil::causalizeDAE(dae.clone(), None, matchingAlgorithm.clone(), daeHandler.clone(), true), '__try0);
        unwrap_break_err!(execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("matching and sorting (n=")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", BackendDAEUtil::daeSize(dae.clone())))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone()), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::GRAPHML.clone()), '__try0) {
            unwrap_break_err!(BackendDump::dumpBipartiteGraphDAE(dae.clone(), (fileNamePrefix.clone()).clone()), '__try0);
        }
        if unwrap_break_err!(Flags::isSet(Flags::EVAL_OUTPUT_ONLY.clone()), '__try0) {
            oldSize = BackendDAEUtil::daeSize(dae.clone());
            dae = unwrap_break_err!(BackendDAEOptimize::evaluateOutputsOnly(dae.clone()), '__try0);
            unwrap_break_err!(execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("evaluateOutputsOnly (n=")); __mm_s.push_str(&*intString(oldSize.clone())); __mm_s.push_str(&*literal!(" -> n=")); __mm_s.push_str(&*intString(BackendDAEUtil::daeSize(dae.clone()))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone()), '__try0);
        }
        if unwrap_break_err!(Flags::isSet(Flags::BLT_DUMP.clone()), '__try0) {
            unwrap_break_err!(BackendDump::bltdump((literal!("bltdump")).clone(), dae.clone()), '__try0);
        }
        (outInitDAE, outInitDAE_lambda0_option, outRemovedInitialEquationLst, globalKnownVars, dae) = unwrap_break_err!(Initialization::solveInitialSystem(dae.clone()), '__try0);
        simDAE = unwrap_break_err!(BackendDAEUtil::setFunctionTree(dae.clone(), BackendDAEUtil::getFunctions(outInitDAE.shared.clone())?), '__try0);
        simDAE = unwrap_break_err!(BackendDAEUtil::setDAEGlobalKnownVars(simDAE.clone(), globalKnownVars.clone()), '__try0);
        simDAE = unwrap_break_err!(BackendDAEOptimize::addInitialStmtsToAlgorithms(simDAE.clone(), false), '__try0);
        simDAE = unwrap_break_err!(Initialization::removeInitializationStuff(simDAE.clone()), '__try0);
        simDAE = unwrap_break_err!(BackendDAEUtil::postOptimizeDAE(simDAE.clone(), postOptModules.clone(), matchingAlgorithm.clone(), daeHandler.clone()), '__try0);
        simDAE = unwrap_break_err!(BackendDAEUtil::sortGlobalKnownVarsInDAE(simDAE.clone()), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::DUMP_INDX_DAE.clone()), '__try0) {
            unwrap_break_err!(BackendDump::dumpBackendDAE(simDAE.clone(), (literal!("dumpindxdae")).clone()), '__try0);
        }
        outDAEmode = simDAE.clone();
        return Ok((outDAEmode, outInitDAE, outInitDAE_lambda0_option, outRemovedInitialEquationLst));
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        openmodelica_util::Globals::stackoverFlowIndex.with(|__root| *__root.borrow_mut() = None);
        ErrorExt::rollbackNumCheckpoints(ErrorExt::getNumCheckpoints() - numCheckpoints.clone());
        Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Stack overflow in ")); __mm_s.push_str(&*literal!("DAEMode.getEqSystemDAEmode")); __mm_s.push_str(&*literal!("...\n")); __mm_s.push_str(&*stringDelimitList(StackOverflow::readableStacktraceMessages()?, (literal!("\n")).clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
        StackOverflow::clearStacktraceMessages();
    }
    bail!("fail");
    Ok((outDAEmode, outInitDAE, outInitDAE_lambda0_option, outRemovedInitialEquationLst))
}

/*
get config function
*/
fn getPostOptModulesDAEString() -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut strpostOptModules: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    strpostOptModules = Config::getPostOptModulesDAE()?;
    Ok(strpostOptModules)
}

// =============================================================================
// public section for createDAEmodeBDAE
//
// =============================================================================
pub fn createDAEmodeBDAE(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    outDAE = BackendDAEUtil::mapEqSystem(inDAE.clone(), (std::sync::Arc::new(createDAEmodeEqSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> + 'static>))?;
    Ok(outDAE)
}

// =============================================================================
// protected section for createDAEmodeBDAE
//
// =============================================================================
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TraverseEqnAryFold {
    pub globalDAEData: BackendDAE::BackendDAEModeData,
    pub newDAEVars: BackendDAE::Variables,
    pub newDAEEquations: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>,
    pub systemVars: BackendDAE::Variables,
    pub functionTree: Arc<AvlTreePathFunction::Tree>,
    pub recursiveStrongComponentRun: bool,
    pub shared: Arc<BackendDAE::Shared>,
}

impl Default for TraverseEqnAryFold {
    fn default() -> Self {
        Self {
            globalDAEData: Default::default(),
            newDAEVars: Default::default(),
            newDAEEquations: Default::default(),
            systemVars: Default::default(),
            functionTree: Default::default(),
            recursiveStrongComponentRun: Default::default(),
            shared: Default::default(),
        }
    }
}

pub type TRAVERSER_CREATE_DAE = TraverseEqnAryFold;


fn createDAEmodeEqSystem(mut syst: Arc<BackendDAE::EqSystem>, mut shared: Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> {
    let mut syst: Arc<BackendDAE::EqSystem> = syst;
    let mut shared: Arc<BackendDAE::Shared> = shared;
    let mut travArgs: TraverseEqnAryFold = <TraverseEqnAryFold as ::std::default::Default>::default();
    let mut globalDAEData: BackendDAE::BackendDAEModeData = <BackendDAE::BackendDAEModeData as ::std::default::Default>::default();
    let mut tmp: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut retSystem: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut newDAEVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut newDAEEquations: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut resEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut systemSize: i32 = 0;
    let mut debug: bool = Flags::isSet(Flags::DEBUG_DAEMODE.clone())?;
    let exec: bool = false;
    globalDAEData = shared.daeModeData.clone();
    systemSize = BackendDAEUtil::systemSize(syst.clone());
    newDAEVars = BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone());
    newDAEEquations = BackendEquation::emptyEqnsSized(systemSize.clone());
    travArgs = TraverseEqnAryFold { globalDAEData: globalDAEData.clone(), newDAEVars: newDAEVars.clone(), newDAEEquations: newDAEEquations.clone(), systemVars: syst.orderedVars.clone(), functionTree: shared.functionTree.clone(), recursiveStrongComponentRun: false, shared: shared.clone() };
    if debug.clone() {
        BackendDump::printEqSystem(syst.clone())?;
    }
    travArgs = BackendDAEUtil::traverseEqSystemStrongComponents(syst.clone(), (std::sync::Arc::new(traverserStrongComponents) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, TraverseEqnAryFold) -> Result<TraverseEqnAryFold> + 'static>), travArgs.clone())?;
    if exec.clone() {
        execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("DAEmode: created residual equations for system size :  ")); __mm_s.push_str(&*intString(BackendDAEUtil::systemSize(syst.clone()))); __mm_s.push_str(&*literal!(": ")); ArcStr::from(__mm_s) }).clone())?;
    }
    globalDAEData = travArgs.globalDAEData.clone();
    if isSome(globalDAEData.modelVars.clone()) {
        globalDAEData.modelVars = Some(BackendVariable::addVariables(travArgs.systemVars.clone(), Util::getOption(globalDAEData.modelVars.clone())?)?);
    } else {
        globalDAEData.modelVars = Some(travArgs.systemVars.clone());
    }
    if exec.clone() {
        execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("DAEmode: adding residual variables:  ")); __mm_s.push_str(&*intString(BackendVariable::varsSize(Util::getOption(globalDAEData.modelVars.clone())?)?)); __mm_s.push_str(&*literal!(": ")); ArcStr::from(__mm_s) }).clone())?;
    }
    retSystem = BackendDAEUtil::createEqSystem(travArgs.newDAEVars.clone(), BackendEquation::emptyEqns(), metamodelica::nil(), crate::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
    retSystem = BackendDAEUtil::setEqSystEqs(retSystem.clone(), travArgs.newDAEEquations.clone());
    retSystem = BackendDAEUtil::setEqSystRemovedEqns(retSystem.clone(), syst.removedEqs.clone());
    retSystem = BackendEquation::requationsAddDAE(ExpandableArray::toList(shared.removedEqs.clone())?, retSystem.clone())?;
    if exec.clone() {
        execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("DAEmode: created system:  ")); __mm_s.push_str(&*intString(BackendDAEUtil::systemSize(retSystem.clone()))); __mm_s.push_str(&*literal!(": ")); ArcStr::from(__mm_s) }).clone())?;
    }
    syst = retSystem.clone();
    assign_field!(shared.daeModeData = globalDAEData.clone());
    if debug.clone() {
        BackendDump::printEqSystem(syst.clone())?;
    }
    if debug.clone() {
        BackendDump::dumpBackendDAEModeData(globalDAEData.clone())?;
    }
    Ok((syst, shared))
}

fn traverserStrongComponents(mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inVars: Arc<metamodelica::List<BackendDAE::Var>>, mut varIdxs: Arc<metamodelica::List<i32>>, mut eqnIdxs: Arc<metamodelica::List<i32>>, mut traverserArgs: TraverseEqnAryFold) -> Result<TraverseEqnAryFold> {
    let mut traverserArgs: TraverseEqnAryFold = traverserArgs;
    let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = inVars.clone();
    let mut varCrefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut recursiveStrongComponentRun: bool = false;
    let mut isStateVarInvolved: bool = false;
    let mut isDiscrete: bool = false;
    varCrefLst = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        for mut v in (inVars.clone()).into_iter().cloned() {
            let __x = v.varName.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    isStateVarInvolved = !(Flags::getConfigBool(Flags::CAUSALIZE_DAE_MODE.clone())?) || List::any(inVars.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::isStateVar, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>));
    isDiscrete = List::any(inVars.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::isVarDiscrete, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>));
    traverserArgs = ({
        let mut debug: bool = false;
        'mc: {
        let __mc_input = (inEqns.clone(), traverserArgs.recursiveStrongComponentRun.clone(), isStateVarInvolved.clone(), isDiscrete.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: eq, tail: Deref @ metamodelica::List::Nil }, false, false, _) => {
                    if !((List::all(vars.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::isCSEVar, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>)))) { bail!("guard") }
                    let mut newResVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut new_eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut traverserArgs: TraverseEqnAryFold = traverserArgs.clone();
                    newResVars = {
        let mut __acc: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
        for mut v in (vars.clone()).into_iter().cloned() {
                    let __x = BackendVariable::setVarKind(v.clone(), crate::BackendDAE::VarKind::DAE_AUX_VAR)?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    new_eq = BackendEquation::setEquationAttributes(eq.clone(), BackendDAE::EQ_ATTR_DEFAULT_AUX.clone())?;
                    traverserArgs.newDAEVars = BackendVariable::addNewVars(newResVars.clone(), traverserArgs.newDAEVars.clone());
                    traverserArgs.newDAEEquations = BackendEquation::addList(list![new_eq.clone()], traverserArgs.newDAEEquations.clone())?;
                    traverserArgs.systemVars = BackendVariable::removeCrefs(varCrefLst.clone(), traverserArgs.systemVars.clone());
                    if debug.clone() {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[DAEmode] Added solved aux vars. vars:\n")); __mm_s.push_str(&*BackendDump::varListString(vars.clone(), (literal!("")).clone())); __mm_s.push_str(&*literal!("eq:\n")); __mm_s.push_str(&*BackendDump::equationListString(list![eq.clone()], (literal!("")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    Ok(traverserArgs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: eq, tail: Deref @ metamodelica::List::Nil }, false, _, true) => {
                    let mut new_eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut traverserArgs: TraverseEqnAryFold = traverserArgs.clone();
                    new_eq = BackendEquation::setEquationAttributes(eq.clone(), BackendDAE::EQ_ATTR_DEFAULT_DISCRETE.clone())?;
                    traverserArgs.newDAEVars = BackendVariable::addNewVars(vars.clone(), traverserArgs.newDAEVars.clone());
                    traverserArgs.newDAEEquations = BackendEquation::addList(list![new_eq.clone()], traverserArgs.newDAEEquations.clone())?;
                    if debug.clone() {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[DAEmode] Create solved discrete equation. vars:\n")); __mm_s.push_str(&*BackendDump::varListString(vars.clone(), (literal!("")).clone())); __mm_s.push_str(&*literal!("eq:\n")); __mm_s.push_str(&*BackendDump::equationListString(list![new_eq.clone()], (literal!("")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    Ok(traverserArgs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: eq @ Deref @ BackendDAE::Equation::WHEN_EQUATION { .. }, tail: Deref @ metamodelica::List::Nil }, false, _, _) => {
                    let mut new_eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut traverserArgs: TraverseEqnAryFold = traverserArgs.clone();
                    new_eq = BackendEquation::setEquationAttributes(eq.clone(), BackendDAE::EQ_ATTR_DEFAULT_DISCRETE.clone())?;
                    traverserArgs.newDAEVars = BackendVariable::addNewVars(vars.clone(), traverserArgs.newDAEVars.clone());
                    traverserArgs.newDAEEquations = BackendEquation::addList(list![new_eq.clone()], traverserArgs.newDAEEquations.clone())?;
                    if debug.clone() {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[DAEmode] Create solved when equation. vars:\n")); __mm_s.push_str(&*BackendDump::varListString(vars.clone(), (literal!("")).clone())); __mm_s.push_str(&*literal!("eq:\n")); __mm_s.push_str(&*BackendDump::equationListString(list![new_eq.clone()], (literal!("")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    Ok(traverserArgs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: eq @ Deref @ BackendDAE::Equation::EQUATION { .. }, tail: Deref @ metamodelica::List::Nil }, false, false, _) => {
                    let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut new_eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut eq = (*eq).clone();
                    let mut traverserArgs: TraverseEqnAryFold = traverserArgs.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(vars.clone()) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    var = __pa0.clone();
                    assign_variant_field!(eq => BackendDAE::Equation::EQUATION; exp = ExpressionSolve::solve(var_field!((*eq).exp, BackendDAE::Equation::EQUATION).clone(), var_field!((*eq).scalar, BackendDAE::Equation::EQUATION).clone(), Expression::crefExp(var.varName.clone())?, None)?.0);
                    new_eq = Arc::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: var.varName.clone(), exp: var_field!((*eq).exp, BackendDAE::Equation::EQUATION).clone(), source: var_field!((*eq).source, BackendDAE::Equation::EQUATION).clone(), attr: var_field!((*eq).attr, BackendDAE::Equation::EQUATION).clone() });
                    new_eq = BackendEquation::setEquationAttributes(new_eq.clone(), BackendDAE::EQ_ATTR_DEFAULT_AUX.clone())?;
                    traverserArgs.newDAEVars = BackendVariable::addNewVars(vars.clone(), traverserArgs.newDAEVars.clone());
                    traverserArgs.newDAEEquations = BackendEquation::addList(list![new_eq.clone()], traverserArgs.newDAEEquations.clone())?;
                    if debug.clone() {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[DAEmode] Create solved equation. vars:\n")); __mm_s.push_str(&*BackendDump::varListString(vars.clone(), (literal!("")).clone())); __mm_s.push_str(&*literal!("eq:\n")); __mm_s.push_str(&*BackendDump::equationListString(list![new_eq.clone()], (literal!("")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    Ok(traverserArgs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: eq @ Deref @ BackendDAE::Equation::COMPLEX_EQUATION { .. }, tail: Deref @ metamodelica::List::Nil }, false, false, _) => {
                    let mut new_eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut traverserArgs: TraverseEqnAryFold = traverserArgs.clone();
                    new_eq = BackendEquation::setEquationAttributes(eq.clone(), BackendDAE::EQ_ATTR_DEFAULT_AUX.clone())?;
                    traverserArgs.newDAEVars = BackendVariable::addNewVars(vars.clone(), traverserArgs.newDAEVars.clone());
                    traverserArgs.newDAEEquations = BackendEquation::addList(list![new_eq.clone()], traverserArgs.newDAEEquations.clone())?;
                    if debug.clone() {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[DAEmode] Create solved complex equation. vars:\n")); __mm_s.push_str(&*BackendDump::varListString(vars.clone(), (literal!("")).clone())); __mm_s.push_str(&*literal!("eq:\n")); __mm_s.push_str(&*BackendDump::equationListString(list![new_eq.clone()], (literal!("")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    Ok(traverserArgs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: eq @ Deref @ BackendDAE::Equation::ARRAY_EQUATION { .. }, tail: Deref @ metamodelica::List::Nil }, false, false, _) => {
                    let mut new_eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut traverserArgs: TraverseEqnAryFold = traverserArgs.clone();
                    new_eq = BackendEquation::setEquationAttributes(eq.clone(), BackendDAE::EQ_ATTR_DEFAULT_AUX.clone())?;
                    traverserArgs.newDAEVars = BackendVariable::addNewVars(vars.clone(), traverserArgs.newDAEVars.clone());
                    traverserArgs.newDAEEquations = BackendEquation::addList(list![new_eq.clone()], traverserArgs.newDAEEquations.clone())?;
                    if debug.clone() {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[DAEmode] Create solved array equations. vars:\n")); __mm_s.push_str(&*BackendDump::varListString(vars.clone(), (literal!("")).clone())); __mm_s.push_str(&*literal!("eq:\n")); __mm_s.push_str(&*BackendDump::equationListString(list![new_eq.clone()], (literal!("")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    Ok(traverserArgs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: eq @ Deref @ BackendDAE::Equation::ALGORITHM { expand: crefExpand, source, alg, .. }, tail: Deref @ metamodelica::List::Nil }, false, false, _) => {
                    let mut new_eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut traverserArgs: TraverseEqnAryFold = traverserArgs.clone();
                    let true = (CheckModel::isCrefListAlgorithmOutput(varCrefLst.clone(), alg.clone(), source.clone(), crefExpand.clone())?) else { bail!("pattern mismatch") };
                    new_eq = BackendEquation::setEquationAttributes(eq.clone(), BackendDAE::EQ_ATTR_DEFAULT_AUX.clone())?;
                    traverserArgs.newDAEVars = BackendVariable::addNewVars(vars.clone(), traverserArgs.newDAEVars.clone());
                    traverserArgs.newDAEEquations = BackendEquation::addList(list![new_eq.clone()], traverserArgs.newDAEEquations.clone())?;
                    if debug.clone() {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[DAEmode] Create solved algorithms. vars:\n")); __mm_s.push_str(&*BackendDump::varListString(vars.clone(), (literal!("")).clone())); __mm_s.push_str(&*literal!("eq:\n")); __mm_s.push_str(&*BackendDump::equationListString(list![new_eq.clone()], (literal!("")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    Ok(traverserArgs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: eq @ Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: exp, .. }, tail: Deref @ metamodelica::List::Nil }, b1, b2, _) => {
                    if !((Expression::isCref(exp.clone()) && (b1.clone() || b2.clone()))) { bail!("guard") }
                    let mut newResEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut newResVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut newAuxVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut aux_eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut newnumResVars: i32 = 0;
                    let mut globalDAEData: BackendDAE::BackendDAEModeData = <BackendDAE::BackendDAEModeData as ::std::default::Default>::default();
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut newCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut eq = (*eq).clone();
                    let mut traverserArgs: TraverseEqnAryFold = traverserArgs.clone();
                    globalDAEData = traverserArgs.globalDAEData.clone();
                    cref = Expression::expCref(exp.clone())?;
                    (newAuxVars, _) = BackendVariable::getVar(cref.clone(), traverserArgs.systemVars.clone())?;
                    crlst = ComponentReference::expandCref(cref.clone(), true)?;
                    newAuxVars = {
        let mut __acc: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
        for (cr, v) in (&(crlst.clone())).into_iter().zip((&(newAuxVars.clone())).into_iter()) {
                    let __x = BackendVariable::copyVarNewName(ComponentReference::crefPrefixAux(cr.clone()), v.clone());
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    newAuxVars = {
        let mut __acc: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
        for mut v in (newAuxVars.clone()).into_iter().cloned() {
                    let __x = BackendVariable::setVarKind(v.clone(), crate::BackendDAE::VarKind::DAE_AUX_VAR)?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    traverserArgs.newDAEVars = BackendVariable::addNewVars(newAuxVars.clone(), traverserArgs.newDAEVars.clone());
                    newCref = ComponentReference::crefPrefixAux(cref.clone());
                    assign_variant_field!(eq => BackendDAE::Equation::ARRAY_EQUATION; left = Expression::crefExp(newCref.clone())?);
                    aux_eq = eq.clone();
                    aux_eq = BackendEquation::setEquationAttributes(aux_eq.clone(), BackendDAE::EQ_ATTR_DEFAULT_AUX.clone())?;
                    traverserArgs.newDAEEquations = BackendEquation::addList(list![aux_eq.clone()], traverserArgs.newDAEEquations.clone())?;
                    globalDAEData = traverserArgs.globalDAEData.clone();
                    assign_variant_field!(eq => BackendDAE::Equation::ARRAY_EQUATION; right = Expression::crefToExp(cref.clone())?);
                    newResEqns = BackendEquation::equationToScalarResidualForm(eq.clone(), traverserArgs.functionTree.clone())?;
                    (newResEqns, newResVars, newnumResVars) = BackendEquation::convertResidualsIntoSolvedEquations(newResEqns.clone(), (literal!("$DAEres")).clone(), globalDAEData.numResVars.clone(), true)?;
                    globalDAEData.numResVars = newnumResVars.clone();
                    newResEqns = {
        let mut __acc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
        for mut e in (newResEqns.clone()).into_iter().cloned() {
                    let __x = BackendEquation::setEquationAttributes(e.clone(), BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    traverserArgs.newDAEVars = BackendVariable::addNewVars(newResVars.clone(), traverserArgs.newDAEVars.clone());
                    traverserArgs.newDAEEquations = BackendEquation::addList(newResEqns.clone(), traverserArgs.newDAEEquations.clone())?;
                    globalDAEData = addVarsGlobalData(globalDAEData.clone(), vars.clone())?;
                    traverserArgs.globalDAEData = globalDAEData.clone();
                    if debug.clone() {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[DAEmode] Added residual array equation\n")); __mm_s.push_str(&*BackendDump::varListString(newResVars.clone(), (literal!("")).clone())); __mm_s.push_str(&*literal!("states:\n")); __mm_s.push_str(&*BackendDump::varListString(vars.clone(), (literal!("")).clone())); __mm_s.push_str(&*literal!("eqs:\n")); __mm_s.push_str(&*BackendDump::equationListString(newResEqns.clone(), (literal!("")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    Ok(traverserArgs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: eq, tail: Deref @ metamodelica::List::Nil }, b1, b2, _) => {
                    if !((b1.clone() || b2.clone())) { bail!("guard") }
                    let mut newResEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut newResVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut newnumResVars: i32 = 0;
                    let mut globalDAEData: BackendDAE::BackendDAEModeData = <BackendDAE::BackendDAEModeData as ::std::default::Default>::default();
                    let mut traverserArgs: TraverseEqnAryFold = traverserArgs.clone();
                    globalDAEData = traverserArgs.globalDAEData.clone();
                    newResEqns = BackendEquation::equationToScalarResidualForm(eq.clone(), traverserArgs.functionTree.clone())?;
                    (newResEqns, newResVars, newnumResVars) = BackendEquation::convertResidualsIntoSolvedEquations(newResEqns.clone(), (literal!("$DAEres")).clone(), globalDAEData.numResVars.clone(), true)?;
                    globalDAEData.numResVars = newnumResVars.clone();
                    newResEqns = {
        let mut __acc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
        for mut e in (newResEqns.clone()).into_iter().cloned() {
                    let __x = BackendEquation::setEquationAttributes(e.clone(), BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    traverserArgs.newDAEVars = BackendVariable::addNewVars(newResVars.clone(), traverserArgs.newDAEVars.clone());
                    traverserArgs.newDAEEquations = BackendEquation::addList(newResEqns.clone(), traverserArgs.newDAEEquations.clone())?;
                    globalDAEData = addVarsGlobalData(globalDAEData.clone(), vars.clone())?;
                    traverserArgs.globalDAEData = globalDAEData.clone();
                    if debug.clone() {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[DAEmode] Added strong component or state eqns\n")); __mm_s.push_str(&*BackendDump::varListString(newResVars.clone(), (literal!("")).clone())); __mm_s.push_str(&*literal!("states:\n")); __mm_s.push_str(&*BackendDump::varListString(vars.clone(), (literal!("")).clone())); __mm_s.push_str(&*literal!("eqs:\n")); __mm_s.push_str(&*BackendDump::equationListString(newResEqns.clone(), (literal!("")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    Ok(traverserArgs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: eq @ Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: exp, .. }, tail: Deref @ metamodelica::List::Nil }, _, _, _) => {
                    if !((Expression::isCref(exp.clone()))) { bail!("guard") }
                    let mut newResEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut newResVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut newAuxVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut aux_eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut newnumResVars: i32 = 0;
                    let mut globalDAEData: BackendDAE::BackendDAEModeData = <BackendDAE::BackendDAEModeData as ::std::default::Default>::default();
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut newCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut eq = (*eq).clone();
                    let mut traverserArgs: TraverseEqnAryFold = traverserArgs.clone();
                    if debug.clone() {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("case: Complex: ")); __mm_s.push_str(&*BackendDump::equationListString(inEqns.clone(), (literal!("")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    cref = Expression::expCref(exp.clone())?;
                    newAuxVars = {
        let mut __acc: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
        for (cr, v) in (&(varCrefLst.clone())).into_iter().zip((&(vars.clone())).into_iter()) {
                    let __x = BackendVariable::copyVarNewName(ComponentReference::crefPrefixAux(cr.clone()), v.clone());
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    newAuxVars = {
        let mut __acc: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
        for mut v in (newAuxVars.clone()).into_iter().cloned() {
                    let __x = BackendVariable::setVarKind(v.clone(), crate::BackendDAE::VarKind::DAE_AUX_VAR)?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    traverserArgs.newDAEVars = BackendVariable::addNewVars(newAuxVars.clone(), traverserArgs.newDAEVars.clone());
                    newCref = ComponentReference::crefPrefixAux(cref.clone());
                    assign_variant_field!(eq => BackendDAE::Equation::COMPLEX_EQUATION; left = Expression::crefToExp(newCref.clone())?);
                    aux_eq = eq.clone();
                    aux_eq = BackendEquation::setEquationAttributes(aux_eq.clone(), BackendDAE::EQ_ATTR_DEFAULT_AUX.clone())?;
                    traverserArgs.newDAEEquations = BackendEquation::addList(list![aux_eq.clone()], traverserArgs.newDAEEquations.clone())?;
                    globalDAEData = traverserArgs.globalDAEData.clone();
                    assign_variant_field!(eq => BackendDAE::Equation::COMPLEX_EQUATION; right = Expression::crefToExp(cref.clone())?);
                    newResEqns = BackendEquation::equationToScalarResidualForm(eq.clone(), traverserArgs.functionTree.clone())?;
                    (newResEqns, newResVars, newnumResVars) = BackendEquation::convertResidualsIntoSolvedEquations(newResEqns.clone(), (literal!("$DAEres")).clone(), globalDAEData.numResVars.clone(), true)?;
                    globalDAEData.numResVars = newnumResVars.clone();
                    newResEqns = {
        let mut __acc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
        for mut e in (newResEqns.clone()).into_iter().cloned() {
                    let __x = BackendEquation::setEquationAttributes(e.clone(), BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    traverserArgs.newDAEVars = BackendVariable::addNewVars(newResVars.clone(), traverserArgs.newDAEVars.clone());
                    traverserArgs.newDAEEquations = BackendEquation::addList(newResEqns.clone(), traverserArgs.newDAEEquations.clone())?;
                    globalDAEData = addVarsGlobalData(globalDAEData.clone(), vars.clone())?;
                    traverserArgs.globalDAEData = globalDAEData.clone();
                    if debug.clone() {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[DAEmode] Added complex residual equation with aux variables. Res-vars:\n")); __mm_s.push_str(&*BackendDump::varListString(newResVars.clone(), (literal!("")).clone())); __mm_s.push_str(&*literal!("eqs:\n")); __mm_s.push_str(&*BackendDump::equationListString(newResEqns.clone(), (literal!("")).clone())); __mm_s.push_str(&*literal!("aux vars:\n")); __mm_s.push_str(&*BackendDump::varListString(newAuxVars.clone(), (literal!("")).clone())); __mm_s.push_str(&*literal!("aux eq:\n")); __mm_s.push_str(&*BackendDump::equationListString(list![aux_eq.clone()], (literal!("")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    Ok(traverserArgs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, false, _, _) => {
                    let mut newAuxVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut discVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut contVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut size: i32 = 0;
                    let mut discEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut contEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut traverserArgs: TraverseEqnAryFold = traverserArgs.clone();
                    (discVars, contVars) = List::splitOnTrue(inVars.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::isVarDiscrete, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>));
                    (discEqns, contEqns) = getDiscAndContEqns(inVars.clone(), inEqns.clone(), discVars.clone(), contVars.clone(), traverserArgs.shared.functionTree.clone(), BackendDAEUtil::isInitializationDAE(traverserArgs.shared.clone()))?;
                    for mut e in &*discEqns.clone() {
                        let mut e = e.clone();
                        size = BackendEquation::equationSize(e.clone())?;
                        newAuxVars = List::firstN(discVars.clone(), size.clone())?;
                        traverserArgs = traverserStrongComponents(list![e.clone()], newAuxVars.clone(), metamodelica::nil(), metamodelica::nil(), traverserArgs.clone())?;
                        discVars = List::stripN(discVars.clone(), size.clone())?;
                    }
                    for mut e in &*contEqns.clone() {
                        let mut e = e.clone();
                        size = BackendEquation::equationSize(e.clone())?;
                        newAuxVars = List::firstN(contVars.clone(), size.clone())?;
                        traverserArgs.recursiveStrongComponentRun = true;
                        traverserArgs = traverserStrongComponents(list![e.clone()], newAuxVars.clone(), metamodelica::nil(), metamodelica::nil(), traverserArgs.clone())?;
                        traverserArgs.recursiveStrongComponentRun = false;
                        contVars = List::stripN(contVars.clone(), size.clone())?;
                    }
                    Ok(traverserArgs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("DAEMode.traverserStrongComponents failed on equation:\n")); __mm_s.push_str(&*BackendDump::equationListString(inEqns.clone(), (literal!("")).clone())); __mm_s.push_str(&*literal!("\nVariables:\n")); __mm_s.push_str(&*BackendDump::varListString(inVars.clone(), (literal!("")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }
    });
    Ok(traverserArgs)
}

fn getDiscAndContEqns(mut inAllVars: Arc<metamodelica::List<BackendDAE::Var>>, mut inAllEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inDiscVars: Arc<metamodelica::List<BackendDAE::Var>>, mut inContVars: Arc<metamodelica::List<BackendDAE::Var>>, mut functionTree: Arc<AvlTreePathFunction::Tree>, mut isInitial: bool) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut discEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut contEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut adjMatrix: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut varsIndex: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqnIndex: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut assignVarEqn: metamodelica::Array<i32>;
    let mut assignEqnVar: metamodelica::Array<i32>;
    let mut mapEqnScalarArray: metamodelica::Array<i32>;
    let debug: bool = false;
    match '__try0: {
        syst = BackendDAEUtil::createEqSystem(BackendVariable::listVar1(inAllVars.clone()), BackendEquation::listEquation(inAllEqns.clone())?, metamodelica::nil(), crate::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
        if debug.clone() {
            unwrap_break_err!(BackendDump::printEqSystem(syst.clone()), '__try0);
        }
        (adjMatrix, _, _, mapEqnScalarArray) = unwrap_break_err!(BackendDAEUtil::adjacencyMatrixScalar(syst.clone(), crate::BackendDAE::IndexType::NORMAL, Some(functionTree.clone()), isInitial.clone()), '__try0);
        if debug.clone() {
            BackendDump::dumpAdjacencyMatrix(adjMatrix.clone());
        }
        let (__pa1, __pa2, true, _, _) = (unwrap_break_err!(Matching::RegularMatching(adjMatrix.clone(), BackendDAEUtil::systemSize(syst.clone()), BackendDAEUtil::systemSize(syst.clone())), '__try0)) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        assignVarEqn = __pa1.clone();
        assignEqnVar = __pa2.clone();
        if debug.clone() {
            unwrap_break_err!(BackendDump::dumpMatching(assignVarEqn.clone()), '__try0);
        }
        varsIndex = BackendVariable::getVarIndexFromVars(inDiscVars.clone(), syst.orderedVars.clone());
        if debug.clone() {
            println!("{}", (literal!("discVarsIndex: ")).clone());
            BackendDump::dumpAdjacencyRow(varsIndex.clone());
        }
        eqnIndex = List::map1(varsIndex.clone(), std::sync::Arc::new(fnptr!(Array::getIndexFirst, i32, _)), assignVarEqn.clone());
        if debug.clone() {
            println!("{}", (literal!("discEqnIndex: ")).clone());
            BackendDump::dumpAdjacencyRow(eqnIndex.clone());
        }
        eqnIndex = List::unique({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (eqnIndex.clone()).into_iter().cloned() {
            let __x = mapEqnScalarArray.borrow()[(i.clone()-1) as usize].clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        discEqns = BackendEquation::getList(eqnIndex.clone(), syst.orderedEqs.clone());
        if debug.clone() {
            BackendDump::equationListString(discEqns.clone(), (literal!("Discrete Equations")).clone());
        }
        varsIndex = BackendVariable::getVarIndexFromVars(inContVars.clone(), syst.orderedVars.clone());
        eqnIndex = List::map1(varsIndex.clone(), std::sync::Arc::new(fnptr!(Array::getIndexFirst, i32, _)), assignVarEqn.clone());
        eqnIndex = List::unique({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (eqnIndex.clone()).into_iter().cloned() {
            let __x = mapEqnScalarArray.borrow()[(i.clone()-1) as usize].clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        if debug.clone() {
            println!("{}", (literal!("contEqnIndex: ")).clone());
            BackendDump::dumpAdjacencyRow(eqnIndex.clone());
        }
        contEqns = BackendEquation::getList(eqnIndex.clone(), syst.orderedEqs.clone());
        if debug.clone() {
            BackendDump::equationListString(contEqns.clone(), (literal!("Continuous Equations")).clone());
        }
        Ok::<_, anyhow::Error>((adjMatrix.clone(), assignEqnVar.clone(), assignVarEqn.clone(), contEqns.clone(), discEqns.clone(), eqnIndex.clone(), mapEqnScalarArray.clone(), syst.clone(), varsIndex.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5, __try0_o6, __try0_o7, __try0_o8)) => {
            adjMatrix = __try0_o0;
            assignEqnVar = __try0_o1;
            assignVarEqn = __try0_o2;
            contEqns = __try0_o3;
            discEqns = __try0_o4;
            eqnIndex = __try0_o5;
            mapEqnScalarArray = __try0_o6;
            syst = __try0_o7;
            varsIndex = __try0_o8;
        }
        Err(_) => {
            bail!("fail");
        }
    }
    Ok((discEqns, contEqns))
}

fn addVarsGlobalData(mut globalDAEData: BackendDAE::BackendDAEModeData, mut inVars: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<BackendDAE::BackendDAEModeData> {
    let mut globalDAEData: BackendDAE::BackendDAEModeData = globalDAEData;
    let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    vars = List::filterOnTrue(inVars.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::isNonStateVar, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>));
    vars = {
        let mut __acc: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
        for mut v in (vars.clone()).into_iter().cloned() {
            let __x = BackendVariable::setVarKind(v.clone(), crate::BackendDAE::VarKind::ALG_STATE)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    globalDAEData.algStateVars = listAppend(vars.clone(), globalDAEData.algStateVars.clone());
    globalDAEData.stateVars = listAppend(List::filterOnTrue(inVars.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::isStateVar, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>)), globalDAEData.stateVars.clone());
    Ok(globalDAEData)
}

fn setNonStateVarAlgState(mut varList: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut varList: Arc<metamodelica::List<BackendDAE::Var>> = varList;
    let mut tmpVarList: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    for mut v in &*varList.clone() {
        let mut v = v.clone();
        v = (match v.clone() {
        BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, .. } => v.clone(),
        BackendDAE::Var { varKind: BackendDAE::VarKind::VARIABLE, .. } => {
            v = BackendVariable::setVarKind(v.clone(), crate::BackendDAE::VarKind::ALG_STATE)?;
            v.clone()
        },
        _ => bail!("fail"),
    });
    }
    varList = varList.clone().reverse();
    Ok(varList)
}

