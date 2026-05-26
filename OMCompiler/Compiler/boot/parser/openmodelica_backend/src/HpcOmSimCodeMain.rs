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
use crate::BackendDAE;
use crate::BackendDAEOptimize;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::HpcOmEqSystems;
use crate::HpcOmMemory;
use crate::HpcOmScheduler;
use crate::HpcOmSimCode;
use crate::HpcOmTaskGraph;
use crate::SimCode;
use crate::SimCodeFunction;
use crate::SimCodeUtil;
use crate::SimCodeVar;
use openmodelica_ast::Absyn;
use openmodelica_frontend::HashTableCrIListArray;
use openmodelica_frontend::HashTableCrILst;
use openmodelica_frontend::HashTableExpToIndex;
use openmodelica_frontend_types::DAE;
use openmodelica_util::ClockIndexes;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ExecStat;
use openmodelica_util::Flags;
use openmodelica_util::FlagsUtil;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

// public imports
// protected imports
pub fn createSimCode(mut inBackendDAE: Arc<BackendDAE::BackendDAE>, mut inInitDAE: Arc<BackendDAE::BackendDAE>, mut inInitDAE_lambda0: Option<Arc<BackendDAE::BackendDAE>>, mut inRemovedInitialEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inClassName: Arc<Absyn::Path>, mut filenamePrefix: ArcStr, mut inString11: ArcStr, mut functions: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>>, mut externalFunctionIncludes: Arc<metamodelica::List<ArcStr>>, mut includeDirs: Arc<metamodelica::List<ArcStr>>, mut libs: Arc<metamodelica::List<ArcStr>>, mut libPaths: Arc<metamodelica::List<ArcStr>>, mut program: Absyn::Program, mut simSettingsOpt: Option<SimCode::SimulationSettings>, mut recordDecls: Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>>, mut literals: (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<metamodelica::List<Arc<DAE::Exp>>>), mut args: Arc<Absyn::FunctionArgs>) -> Result<SimCode::SimCode> {
    let mut simCode: SimCode::SimCode;
    simCode = 'mc: {
        let __mc_input = (inBackendDAE.clone(), inClassName.clone(), filenamePrefix.clone(), inString11.clone(), functions.clone(), externalFunctionIncludes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), program.clone(), simSettingsOpt.clone(), recordDecls.clone(), literals.clone(), args.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE { .. }, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    let mut lastEqMappingIdx: i32 = 0;
                    let mut equationSccMapping: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
                    let mut sccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut daeSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut simeqCompMapping: metamodelica::Array<i32>;
                    let mut taskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut taskGraphDae: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut taskGraphOde: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut taskGraphData: HpcOmTaskGraph::TaskGraphMeta;
                    let mut taskGraphDataDae: HpcOmTaskGraph::TaskGraphMeta;
                    let mut taskGraphDataOde: HpcOmTaskGraph::TaskGraphMeta;
                    let mut fileName: ArcStr = arcstr::literal!("");
                    let mut schedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>;
                    let mut partData: SimCode::PartitionData;
                    let mut simCode: SimCode::SimCode;
                    let true = (Flags::isSet(Flags::MULTIRATE_PARTITION.clone())?) else { bail!("pattern mismatch") };
                    println!("{}", (literal!("DO MULTIRATE\n")).clone());
                    let (__pa0, (__pa1, __pa2)) = SimCodeUtil::createSimCode(inBackendDAE.clone(), inInitDAE.clone(), inInitDAE_lambda0.clone(), None, inRemovedInitialEquationLst.clone(), inClassName.clone(), (filenamePrefix.clone()).clone(), (inString11.clone()).clone(), functions.clone(), externalFunctionIncludes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), program.clone(), simSettingsOpt.clone(), recordDecls.clone(), literals.clone(), args.clone(), false, (literal!("")).clone(), (literal!("")).clone(), metamodelica::nil())?;
                    simCode = __pa0.clone();
                    lastEqMappingIdx = __pa1.clone();
                    equationSccMapping = __pa2.clone();
                    let _ = SimCodeUtil::getSimVarMappingOfBackendMapping(simCode.backendMapping.clone());
                    (simeqCompMapping, sccSimEqMapping, daeSccSimEqMapping) = HpcOmTaskGraph::setUpHpcOmMapping(inBackendDAE.clone(), simCode.clone(), lastEqMappingIdx.clone(), equationSccMapping.clone())?;
                    ExecStat::execStat((literal!("hpcom setup")).clone())?;
                    (taskGraph, taskGraphData) = HpcOmTaskGraph::createTaskGraph(inBackendDAE.clone(), false)?;
                    taskGraphDae = metamodelica::arrayFromVec(taskGraph.clone().borrow().clone());
                    taskGraphDataDae = HpcOmTaskGraph::copyTaskGraphMeta(taskGraphData.clone())?;
                    (taskGraphDae, taskGraphDataDae) = HpcOmTaskGraph::appendRemovedEquations(inBackendDAE.clone(), taskGraphDae.clone(), taskGraphDataDae.clone())?;
                    taskGraphDataDae = HpcOmTaskGraph::createCosts(inBackendDAE.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*filenamePrefix.clone()); __mm_s.push_str(&*literal!("_eqs_prof")); ArcStr::from(__mm_s) }).clone(), simeqCompMapping.clone(), taskGraphDataDae.clone())?;
                    taskGraphData = HpcOmTaskGraph::copyCosts(taskGraphDataDae.clone(), taskGraphData.clone())?;
                    taskGraphOde = metamodelica::arrayFromVec(taskGraph.clone().borrow().clone());
                    taskGraphDataOde = HpcOmTaskGraph::copyTaskGraphMeta(taskGraphData.clone())?;
                    (taskGraphOde, taskGraphDataOde) = HpcOmTaskGraph::getOdeSystem(taskGraphOde.clone(), taskGraphDataOde.clone(), inBackendDAE.clone())?;
                    fileName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("taskGraph")); __mm_s.push_str(&*filenamePrefix.clone()); __mm_s.push_str(&*literal!("_ODE.graphml")); ArcStr::from(__mm_s) }).clone();
                    schedulerInfo = arrayCreate((taskGraphOde.clone().borrow().len() as i32), (-1, -1, metamodelica::OrderedFloat(-1.0_f64)));
                    HpcOmTaskGraph::dumpAsGraphMLSccLevel(taskGraphOde.clone(), taskGraphDataOde.clone(), (fileName.clone()).clone(), (literal!("")).clone(), metamodelica::nil(), metamodelica::nil(), daeSccSimEqMapping.clone(), schedulerInfo.clone(), HpcOmTaskGraph::GraphDumpOptions { visualizeCriticalPath: false, visualizeTaskStartAndFinishTime: false, visualizeTaskCalcTime: true, visualizeCommTime: true })?;
                    partData = HpcOmTaskGraph::multirate_partitioning(taskGraphOde.clone(), taskGraphDataOde.clone(), inBackendDAE.clone(), simCode.clone(), sccSimEqMapping.clone())?;
                    simCode.partitionData = partData.clone();
                    Ok(simCode.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE { eqs, .. }, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    let mut lastEqMappingIdx: i32 = 0;
                    let mut equationSccMapping: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
                    let mut sccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut daeSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut simeqCompMapping: metamodelica::Array<i32>;
                    let mut taskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut taskGraphDae: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut taskGraphOde: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut taskGraphZeroFuncs: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut taskGraphOdeSimplified: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut taskGraphDaeSimplified: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut taskGraphZeroFuncSimplified: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut taskGraphOdeScheduled: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut taskGraphData: HpcOmTaskGraph::TaskGraphMeta;
                    let mut taskGraphDataDae: HpcOmTaskGraph::TaskGraphMeta;
                    let mut taskGraphDataOde: HpcOmTaskGraph::TaskGraphMeta;
                    let mut taskGraphDataZeroFuncs: HpcOmTaskGraph::TaskGraphMeta;
                    let mut taskGraphDataOdeSimplified: HpcOmTaskGraph::TaskGraphMeta;
                    let mut taskGraphDataDaeSimplified: HpcOmTaskGraph::TaskGraphMeta;
                    let mut taskGraphDataZeroFuncSimplified: HpcOmTaskGraph::TaskGraphMeta;
                    let mut taskGraphDataOdeScheduled: HpcOmTaskGraph::TaskGraphMeta;
                    let mut fileName: ArcStr = arcstr::literal!("");
                    let mut numProc: i32 = 0;
                    let mut criticalPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut criticalPathsWoC: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut cpCosts: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut cpCostsWoC: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut scheduledTasksOde: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
                    let mut scheduledTasksDae: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
                    let mut scheduledTasksZeroFunc: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
                    let mut zeroFuncsSimEqIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut taskGraphMetaValid: bool = false;
                    let mut criticalPathInfo: ArcStr = arcstr::literal!("");
                    let mut schedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>;
                    let mut scheduleOde: Arc<HpcOmSimCode::Schedule>;
                    let mut scheduleDae: Arc<HpcOmSimCode::Schedule>;
                    let mut scheduleZeroFunc: Arc<HpcOmSimCode::Schedule>;
                    let mut graphCosts: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut graphOps: i32 = 0;
                    let mut optTmpMemoryMap: Option<HpcOmSimCode::MemoryMap> = None;
                    let mut simVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>;
                    let mut varToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (HashTableCrIListArray::FuncHashCref, HashTableCrIListArray::FuncCrefEqual, HashTableCrIListArray::FuncCrefStr, HashTableCrIListArray::FuncExpStr));
                    let mut varToIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr));
                    let mut simCode: SimCode::SimCode;
                    let true = (Flags::isSet(Flags::HPCOM.clone())?) else { bail!("pattern mismatch") };
                    System::realtimeTick(ClockIndexes::RT_CLOCK_EXECSTAT_HPCOM_MODULES.clone())?;
                    let (__pa0, (__pa1, __pa2)) = SimCodeUtil::createSimCode(inBackendDAE.clone(), inInitDAE.clone(), inInitDAE_lambda0.clone(), None, inRemovedInitialEquationLst.clone(), inClassName.clone(), (filenamePrefix.clone()).clone(), (inString11.clone()).clone(), functions.clone(), externalFunctionIncludes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), program.clone(), simSettingsOpt.clone(), recordDecls.clone(), literals.clone(), args.clone(), false, (literal!("")).clone(), (literal!("")).clone(), metamodelica::nil())?;
                    simCode = __pa0.clone();
                    lastEqMappingIdx = __pa1.clone();
                    equationSccMapping = __pa2.clone();
                    simVarMapping = SimCodeUtil::getSimVarMappingOfBackendMapping(simCode.backendMapping.clone());
                    (simeqCompMapping, sccSimEqMapping, daeSccSimEqMapping) = HpcOmTaskGraph::setUpHpcOmMapping(inBackendDAE.clone(), simCode.clone(), lastEqMappingIdx.clone(), equationSccMapping.clone())?;
                    ExecStat::execStat((literal!("hpcom setup")).clone())?;
                    (taskGraph, taskGraphData) = HpcOmTaskGraph::createTaskGraph(inBackendDAE.clone(), false)?;
                    taskGraphDae = metamodelica::arrayFromVec(taskGraph.clone().borrow().clone());
                    taskGraphDataDae = HpcOmTaskGraph::copyTaskGraphMeta(taskGraphData.clone())?;
                    (taskGraphDae, taskGraphDataDae) = HpcOmTaskGraph::appendRemovedEquations(inBackendDAE.clone(), taskGraphDae.clone(), taskGraphDataDae.clone())?;
                    schedulerInfo = arrayCreate((taskGraphDae.clone().borrow().len() as i32), (-1, -1, metamodelica::OrderedFloat(-1.0_f64)));
                    ExecStat::execStat((literal!("hpcom create DAE TaskGraph")).clone())?;
                    let _ = checkTaskGraphMetaConsistency(taskGraphDae.clone(), taskGraphDataDae.clone(), (literal!("DAE system")).clone())?;
                    ExecStat::execStat((literal!("hpcom validate DAE TaskGraph")).clone())?;
                    taskGraphDataDae = HpcOmTaskGraph::createCosts(inBackendDAE.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*filenamePrefix.clone()); __mm_s.push_str(&*literal!("_eqs_prof")); ArcStr::from(__mm_s) }).clone(), simeqCompMapping.clone(), taskGraphDataDae.clone())?;
                    taskGraphData = HpcOmTaskGraph::copyCosts(taskGraphDataDae.clone(), taskGraphData.clone())?;
                    ExecStat::execStat((literal!("hpcom create costs")).clone())?;
                    taskGraphOde = metamodelica::arrayFromVec(taskGraph.clone().borrow().clone());
                    taskGraphDataOde = HpcOmTaskGraph::copyTaskGraphMeta(taskGraphData.clone())?;
                    (taskGraphOde, taskGraphDataOde) = HpcOmTaskGraph::getOdeSystem(taskGraphOde.clone(), taskGraphDataOde.clone(), inBackendDAE.clone())?;
                    ExecStat::execStat((literal!("hpcom create ODE TaskGraph")).clone())?;
                    taskGraphMetaValid = HpcOmTaskGraph::validateTaskGraphMeta(taskGraphDataOde.clone(), inBackendDAE.clone())?;
                    if boolNot(taskGraphMetaValid.clone()) {
                        println!("{}", (literal!("TaskgraphMeta ODE invalid\n")).clone());
                    }
                    ExecStat::execStat((literal!("hpcom validate ODE TaskGraph")).clone())?;
                    taskGraphDataDae = HpcOmTaskGraph::markSystemComponents(taskGraphOde.clone(), taskGraphDataOde.clone(), (false, true, false), taskGraphDataDae.clone())?;
                    taskGraphZeroFuncs = metamodelica::arrayFromVec(taskGraphDae.clone().borrow().clone());
                    taskGraphDataZeroFuncs = HpcOmTaskGraph::copyTaskGraphMeta(taskGraphDataDae.clone())?;
                    zeroFuncsSimEqIdc = List::map(simCode.equationsForZeroCrossings.clone(), Arc::new(SimCodeUtil::simEqSystemIndex));
                    (taskGraphZeroFuncs, taskGraphDataZeroFuncs) = HpcOmTaskGraph::getZeroFuncsSystem(taskGraphZeroFuncs.clone(), taskGraphDataZeroFuncs.clone(), inBackendDAE.clone(), (daeSccSimEqMapping.clone().borrow().len() as i32), zeroFuncsSimEqIdc.clone(), simeqCompMapping.clone())?;
                    fileName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("taskGraph")); __mm_s.push_str(&*filenamePrefix.clone()); __mm_s.push_str(&*literal!("_ZeroFuncs.graphml")); ArcStr::from(__mm_s) }).clone();
                    schedulerInfo = arrayCreate((taskGraphZeroFuncs.clone().borrow().len() as i32), (-1, -1, metamodelica::OrderedFloat(-1.0_f64)));
                    HpcOmTaskGraph::dumpAsGraphMLSccLevel(taskGraphZeroFuncs.clone(), taskGraphDataZeroFuncs.clone(), (fileName.clone()).clone(), (literal!("")).clone(), metamodelica::nil(), metamodelica::nil(), daeSccSimEqMapping.clone(), schedulerInfo.clone(), HpcOmTaskGraph::GraphDumpOptions { visualizeCriticalPath: false, visualizeTaskStartAndFinishTime: false, visualizeTaskCalcTime: true, visualizeCommTime: true })?;
                    ExecStat::execStat((literal!("hpcom create and dump zeroFuncs TaskGraph")).clone())?;
                    taskGraphDataDae = HpcOmTaskGraph::markSystemComponents(taskGraphZeroFuncs.clone(), taskGraphDataZeroFuncs.clone(), (true, false, false), taskGraphDataDae.clone())?;
                    let _ = checkTaskGraphMetaConsistency(taskGraphZeroFuncs.clone(), taskGraphDataZeroFuncs.clone(), (literal!("ZeroFunc system")).clone())?;
                    let _ = checkEquationCount(taskGraphDataZeroFuncs.clone(), (literal!("ZeroFunc system")).clone(), (zeroFuncsSimEqIdc.clone().len() as i32), sccSimEqMapping.clone())?;
                    fileName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("taskGraph")); __mm_s.push_str(&*filenamePrefix.clone()); __mm_s.push_str(&*literal!("DAE.graphml")); ArcStr::from(__mm_s) }).clone();
                    schedulerInfo = arrayCreate((taskGraphDae.clone().borrow().len() as i32), (-1, -1, metamodelica::OrderedFloat(-1.0_f64)));
                    HpcOmTaskGraph::dumpAsGraphMLSccLevel(taskGraphDae.clone(), taskGraphDataDae.clone(), (fileName.clone()).clone(), (literal!("")).clone(), metamodelica::nil(), metamodelica::nil(), daeSccSimEqMapping.clone(), schedulerInfo.clone(), HpcOmTaskGraph::GraphDumpOptions { visualizeCriticalPath: false, visualizeTaskStartAndFinishTime: false, visualizeTaskCalcTime: true, visualizeCommTime: true })?;
                    ExecStat::execStat((literal!("hpcom dump DAE TaskGraph")).clone())?;
                    let ((__pa3, __pa4), (__pa5, __pa6)) = HpcOmTaskGraph::getCriticalPaths(taskGraphOde.clone(), taskGraphDataOde.clone())?;
                    criticalPaths = __pa3.clone();
                    cpCosts = __pa4.clone();
                    criticalPathsWoC = __pa5.clone();
                    cpCostsWoC = __pa6.clone();
                    criticalPathInfo = (HpcOmTaskGraph::dumpCriticalPathInfo((criticalPaths.clone(), cpCosts.clone()), (criticalPathsWoC.clone(), cpCostsWoC.clone()))?).clone();
                    (graphOps, graphCosts) = HpcOmTaskGraph::sumUpExeCosts(taskGraphOde.clone(), taskGraphDataOde.clone());
                    graphCosts = HpcOmTaskGraph::roundReal(graphCosts.clone(), 2);
                    criticalPathInfo = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*criticalPathInfo.clone()); __mm_s.push_str(&*literal!(" sum: (")); __mm_s.push_str(&*realString(graphCosts.clone())); __mm_s.push_str(&*literal!(" ; ")); __mm_s.push_str(&*intString(graphOps.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
                    fileName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("taskGraph")); __mm_s.push_str(&*filenamePrefix.clone()); __mm_s.push_str(&*literal!("ODE.graphml")); ArcStr::from(__mm_s) }).clone();
                    schedulerInfo = arrayCreate((taskGraphOde.clone().borrow().len() as i32), (-1, -1, metamodelica::OrderedFloat(-1.0_f64)));
                    ExecStat::execStat((literal!("hpcom assign levels / get crit. path")).clone())?;
                    HpcOmTaskGraph::dumpAsGraphMLSccLevel(taskGraphOde.clone(), taskGraphDataOde.clone(), (fileName.clone()).clone(), (criticalPathInfo.clone()).clone(), HpcOmTaskGraph::convertNodeListToEdgeTuples(listHead(criticalPaths.clone())?)?, HpcOmTaskGraph::convertNodeListToEdgeTuples(listHead(criticalPathsWoC.clone())?)?, sccSimEqMapping.clone(), schedulerInfo.clone(), HpcOmTaskGraph::GraphDumpOptions { visualizeCriticalPath: true, visualizeTaskStartAndFinishTime: false, visualizeTaskCalcTime: true, visualizeCommTime: true })?;
                    ExecStat::execStat((literal!("hpcom dump ODE TaskGraph")).clone())?;
                    if Flags::isSet(Flags::HPCOM_DUMP.clone())? {
                        println!("{}", (literal!("Critical Path successfully calculated\n")).clone());
                    }
                    scheduledTasksDae = metamodelica::nil();
                    (scheduledTasksOde, _) = HpcOmEqSystems::parallelizeTornSystems(taskGraphOde.clone(), taskGraphDataOde.clone(), sccSimEqMapping.clone(), simVarMapping.clone(), inBackendDAE.clone())?;
                    scheduledTasksZeroFunc = metamodelica::nil();
                    (taskGraphDaeSimplified, taskGraphDataDaeSimplified) = applyGRS(taskGraphDae.clone(), taskGraphDataDae.clone())?;
                    (taskGraphOdeSimplified, taskGraphDataOdeSimplified) = applyGRS(taskGraphOde.clone(), taskGraphDataOde.clone())?;
                    (taskGraphZeroFuncSimplified, taskGraphDataZeroFuncSimplified) = applyGRS(taskGraphZeroFuncs.clone(), taskGraphDataZeroFuncs.clone())?;
                    ExecStat::execStat((literal!("hpcom GRS")).clone())?;
                    fileName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("taskGraph")); __mm_s.push_str(&*filenamePrefix.clone()); __mm_s.push_str(&*literal!("ODE_merged.graphml")); ArcStr::from(__mm_s) }).clone();
                    HpcOmTaskGraph::dumpAsGraphMLSccLevel(taskGraphOdeSimplified.clone(), taskGraphDataOdeSimplified.clone(), (fileName.clone()).clone(), (criticalPathInfo.clone()).clone(), HpcOmTaskGraph::convertNodeListToEdgeTuples(listHead(criticalPaths.clone())?)?, HpcOmTaskGraph::convertNodeListToEdgeTuples(listHead(criticalPathsWoC.clone())?)?, sccSimEqMapping.clone(), schedulerInfo.clone(), HpcOmTaskGraph::GraphDumpOptions { visualizeCriticalPath: true, visualizeTaskStartAndFinishTime: false, visualizeTaskCalcTime: true, visualizeCommTime: true })?;
                    ExecStat::execStat((literal!("hpcom dump simplified TaskGraph")).clone())?;
                    if Flags::isSet(Flags::HPCOM_DUMP.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Filter successfully applied. Merged ")); __mm_s.push_str(&*intString(intSub((taskGraphOde.clone().borrow().len() as i32), (taskGraphOdeSimplified.clone().borrow().len() as i32)))); __mm_s.push_str(&*literal!(" tasks.\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    numProc = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
                    (numProc, _) = setNumProc(numProc.clone(), cpCostsWoC.clone(), taskGraphDataOde.clone())?;
                    (scheduleDae, simCode, _, _, sccSimEqMapping) = createSchedule(taskGraphDaeSimplified.clone(), taskGraphDataDaeSimplified.clone(), daeSccSimEqMapping.clone(), simVarMapping.clone(), (filenamePrefix.clone()).clone(), numProc.clone(), numProc.clone(), simCode.clone(), scheduledTasksDae.clone(), (literal!("DAE system")).clone(), (Flags::getConfigString(Flags::HPCOM_SCHEDULER.clone())?).clone())?;
                    (scheduleOde, simCode, taskGraphOdeScheduled, taskGraphDataOdeScheduled, sccSimEqMapping) = createSchedule(taskGraphOdeSimplified.clone(), taskGraphDataOdeSimplified.clone(), sccSimEqMapping.clone(), simVarMapping.clone(), (filenamePrefix.clone()).clone(), numProc.clone(), numProc.clone(), simCode.clone(), scheduledTasksOde.clone(), (literal!("ODE system")).clone(), (Flags::getConfigString(Flags::HPCOM_SCHEDULER.clone())?).clone())?;
                    (scheduleZeroFunc, simCode, _, _, sccSimEqMapping) = createSchedule(taskGraphZeroFuncSimplified.clone(), taskGraphDataZeroFuncSimplified.clone(), daeSccSimEqMapping.clone(), simVarMapping.clone(), (filenamePrefix.clone()).clone(), numProc.clone(), numProc.clone(), simCode.clone(), scheduledTasksZeroFunc.clone(), (literal!("ZeroFunc system")).clone(), (Flags::getConfigString(Flags::HPCOM_SCHEDULER.clone())?).clone())?;
                    numProc = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
                    criticalPathInfo = (HpcOmScheduler::analyseScheduledTaskGraph(scheduleOde.clone(), numProc.clone(), taskGraphOdeScheduled.clone(), taskGraphDataOdeScheduled.clone(), (literal!("ODE system")).clone())?).clone();
                    schedulerInfo = HpcOmScheduler::convertScheduleStrucToInfo(scheduleOde.clone(), (taskGraphOdeScheduled.clone().borrow().len() as i32))?;
                    ExecStat::execStat((literal!("hpcom create schedule")).clone())?;
                    fileName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("taskGraph")); __mm_s.push_str(&*filenamePrefix.clone()); __mm_s.push_str(&*literal!("ODE_schedule.graphml")); ArcStr::from(__mm_s) }).clone();
                    HpcOmTaskGraph::dumpAsGraphMLSccLevel(taskGraphOdeScheduled.clone(), taskGraphDataOdeScheduled.clone(), (fileName.clone()).clone(), (criticalPathInfo.clone()).clone(), HpcOmTaskGraph::convertNodeListToEdgeTuples(listHead(criticalPaths.clone())?)?, HpcOmTaskGraph::convertNodeListToEdgeTuples(listHead(criticalPathsWoC.clone())?)?, sccSimEqMapping.clone(), schedulerInfo.clone(), HpcOmTaskGraph::GraphDumpOptions { visualizeCriticalPath: true, visualizeTaskStartAndFinishTime: false, visualizeTaskCalcTime: true, visualizeCommTime: true })?;
                    ExecStat::execStat((literal!("hpcom dump schedule TaskGraph")).clone())?;
                    if Flags::isSet(Flags::HPCOM_DUMP.clone())? {
                        println!("{}", (literal!("Schedule created\n")).clone());
                    }
                    System::realtimeTick(ClockIndexes::RT_CLOCK_EXECSTAT_HPCOM_MODULES.clone())?;
                    checkOdeSystemSize(taskGraphDataOdeScheduled.clone(), simCode.odeEquations.clone(), sccSimEqMapping.clone())?;
                    ExecStat::execStat((literal!("hpcom check ODE system size")).clone())?;
                    (optTmpMemoryMap, varToArrayIndexMapping, varToIndexMapping) = HpcOmMemory::createMemoryMap(simCode.modelInfo.clone(), simCode.varToArrayIndexMapping.clone(), simCode.varToIndexMapping.clone(), taskGraphOdeSimplified.clone(), AdjacencyMatrix::transposeAdjacencyMatrix(taskGraphOdeSimplified.clone(), (taskGraphOdeSimplified.clone().borrow().len() as i32))?, taskGraphDataOdeSimplified.clone(), eqs.clone(), (filenamePrefix.clone()).clone(), schedulerInfo.clone(), scheduleOde.clone(), sccSimEqMapping.clone(), criticalPaths.clone(), criticalPathsWoC.clone(), (criticalPathInfo.clone()).clone(), numProc.clone(), (HpcOmTaskGraph::getSystemComponents(inBackendDAE.clone())?).0, BackendDAEUtil::isInitializationDAE(inBackendDAE.shared.clone()))?;
                    ExecStat::execStat((literal!("hpcom create memory map")).clone())?;
                    simCode.varToArrayIndexMapping = varToArrayIndexMapping.clone();
                    simCode.varToIndexMapping = varToIndexMapping.clone();
                    simCode.hpcomData = HpcOmSimCode::HpcOmData { schedules: Some((scheduleOde.clone(), scheduleDae.clone(), scheduleZeroFunc.clone())), hpcOmMemory: optTmpMemoryMap.clone() };
                    ExecStat::execStat((literal!("hpcom other")).clone())?;
                    println!("{}", (literal!("HpcOm is still under construction.\n")).clone());
                    Ok(simCode.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("function createSimCode failed.")).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(simCode)
}

fn createAndExportInitialSystemTaskGraph(mut iInitDae: Option<Arc<BackendDAE::BackendDAE>>, mut iFileNamePrefix: ArcStr) -> Result<()> {
    let mut initDAE: Arc<BackendDAE::BackendDAE>;
    let mut tmpTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut tmpTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta;
    let mut fileName: ArcStr = arcstr::literal!("");
    let mut sccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut schedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>;
    let _ = (::match_deref::match_deref! { match &((iInitDae.clone(), iFileNamePrefix.clone())) {
        (Some(initDAE), _) => {
            (tmpTaskGraph, tmpTaskGraphMeta) = HpcOmTaskGraph::createTaskGraph(initDAE.clone(), false)?;
            fileName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("taskGraph")); __mm_s.push_str(&*iFileNamePrefix.clone()); __mm_s.push_str(&*literal!("_init.graphml")); ArcStr::from(__mm_s) }).clone();
            schedulerInfo = arrayCreate((tmpTaskGraph.clone().borrow().len() as i32), (-1, -1, metamodelica::OrderedFloat(-1.0_f64)));
            sccSimEqMapping = arrayCreate((tmpTaskGraph.clone().borrow().len() as i32), metamodelica::nil());
            HpcOmTaskGraph::dumpAsGraphMLSccLevel(tmpTaskGraph.clone(), tmpTaskGraphMeta.clone(), (fileName.clone()).clone(), (literal!("")).clone(), metamodelica::nil(), metamodelica::nil(), sccSimEqMapping.clone(), schedulerInfo.clone(), HpcOmTaskGraph::GraphDumpOptions { visualizeCriticalPath: false, visualizeTaskStartAndFinishTime: false, visualizeTaskCalcTime: true, visualizeCommTime: true })?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn setNumProc(mut numProcFlag: i32, mut cpCosts: metamodelica::Real, mut taskGraphMetaIn: HpcOmTaskGraph::TaskGraphMeta) -> Result<(i32, bool)> {
    let mut numProcOut: i32 = 0;
    let mut numFixed: bool = false;
    (numProcOut, numFixed) = (match (numProcFlag.clone(), cpCosts.clone(), taskGraphMetaIn.clone()) {
        (0, _, _) => {
            let mut numProcSys: i32 = 0;
            let mut numProc: i32 = 0;
            let mut numProcSched: i32 = 0;
            let mut serCosts: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut maxSpeedUp: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut string1: ArcStr = arcstr::literal!("");
            let mut string2: ArcStr = arcstr::literal!("");
            serCosts = HpcOmScheduler::getSerialExecutionTime(taskGraphMetaIn.clone())?;
            if realNe(serCosts.clone(), metamodelica::OrderedFloat(0.0_f64)) {
                maxSpeedUp = realDiv(serCosts.clone(), cpCosts.clone());
                numProcSched = (((maxSpeedUp.clone()) + (metamodelica::OrderedFloat(1.0_f64))).0 as i32);
                numProcSys = System::numProcessors();
                numProc = intMin(numProcSched.clone(), numProcSys.clone());
                string1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Your system provides only ")); __mm_s.push_str(&*intString(numProcSys.clone())); __mm_s.push_str(&*literal!(" processors!\n")); ArcStr::from(__mm_s) }).clone();
                string2 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(numProcSched.clone())); __mm_s.push_str(&*literal!(" processors might be a reasonable number of processors.\n")); ArcStr::from(__mm_s) }).clone();
                string1 = (if (intGt(numProcSched.clone(), numProcSys.clone())) {string1.clone()} else {string2.clone()}).clone();
                println!("{}", (literal!("Please set the number of processors you want to use!\n")).clone());
                println!("{}", (string1.clone()).clone());
            } else {
                numProc = 1;
                println!("{}", (literal!("You did not choose a number of cores. Since there is no ODE-System, the number of cores is set to 1!\n")).clone());
            }
            FlagsUtil::setConfigInt(Flags::NUM_PROC.clone(), numProc.clone())?;
            (numProc.clone(), true)
        },
        _ => {
            let mut numProcSys: i32 = 0;
            numProcSys = System::numProcessors();
            if intGt(numProcFlag.clone(), numProcSys.clone()) && Flags::isSet(Flags::HPCOM_DUMP.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Warning: Your system provides only ")); __mm_s.push_str(&*intString(numProcSys.clone())); __mm_s.push_str(&*literal!(" processors!\n")); ArcStr::from(__mm_s) }).clone());
            }
            (numProcFlag.clone(), true)
        },
    });
    Ok((numProcOut, numFixed))
}

pub fn applyGRS(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, HpcOmTaskGraph::TaskGraphMeta)> {
    let mut oTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut oTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta;
    let mut taskGraph1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut taskGraphMeta1: HpcOmTaskGraph::TaskGraphMeta;
    let mut contractedTasks: metamodelica::Array<i32>;
    let mut schedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>;
    let mut fileName: ArcStr = arcstr::literal!("");
    taskGraph1 = metamodelica::arrayFromVec(iTaskGraph.clone().borrow().clone());
    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(taskGraph1.clone(), (taskGraph1.clone().borrow().len() as i32))?;
    taskGraphMeta1 = HpcOmTaskGraph::copyTaskGraphMeta(iTaskGraphMeta.clone())?;
    contractedTasks = arrayCreate((taskGraph1.clone().borrow().len() as i32), 0);
    (taskGraph1, taskGraphT, taskGraphMeta1) = applyGRS1(taskGraph1.clone(), taskGraphT.clone(), taskGraphMeta1.clone(), contractedTasks.clone(), true)?;
    (oTaskGraph, oTaskGraphMeta) = GRS_newGraph(taskGraph1.clone(), taskGraphMeta1.clone(), contractedTasks.clone())?;
    Ok((oTaskGraph, oTaskGraphMeta))
}

fn applyGRS1(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iContractedTasks: metamodelica::Array<i32>, mut again: bool) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, HpcOmTaskGraph::TaskGraphMeta)> {
    let mut oTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut oTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut oTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta;
    (oTaskGraph, oTaskGraphT, oTaskGraphMeta) = (match (iTaskGraph.clone(), iTaskGraphT.clone(), iTaskGraphMeta.clone(), iContractedTasks.clone(), again.clone()) {
        (_, _, _, _, true) => {
            let mut changed: bool = false;
            let mut changed2: bool = false;
            let mut tmpTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut tmpTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut tmpTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta;
            let mut tmpContractedTasks: metamodelica::Array<i32>;
            (tmpTaskGraph, tmpTaskGraphT, tmpTaskGraphMeta, tmpContractedTasks, changed) = HpcOmTaskGraph::mergeSimpleNodes(iTaskGraph.clone(), iTaskGraphT.clone(), iTaskGraphMeta.clone(), iContractedTasks.clone())?;
            (tmpTaskGraph, tmpTaskGraphT, tmpTaskGraphMeta, tmpContractedTasks, changed2) = HpcOmTaskGraph::mergeParentNodes(tmpTaskGraph.clone(), tmpTaskGraphT.clone(), tmpTaskGraphMeta.clone(), tmpContractedTasks.clone())?;
            changed = changed.clone() || changed2.clone();
            applyGRS1(tmpTaskGraph.clone(), tmpTaskGraphT.clone(), tmpTaskGraphMeta.clone(), tmpContractedTasks.clone(), changed.clone())?
        },
        _ => {
            (iTaskGraph.clone(), iTaskGraphT.clone(), iTaskGraphMeta.clone())
        },
    });
    Ok((oTaskGraph, oTaskGraphT, oTaskGraphMeta))
}

fn applyGRSForScheduler(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iContractedTasks: metamodelica::Array<i32>) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, HpcOmTaskGraph::TaskGraphMeta)> {
    let mut oTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut oTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut oTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta;
    let mut flagValue: ArcStr = arcstr::literal!("");
    let mut levelNodes: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut contractedNodes: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut tmpTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut tmpTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut tmpTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta;
    (oTaskGraph, oTaskGraphT, oTaskGraphMeta) = 'mc: {
        let __mc_input = (iTaskGraph.clone(), iTaskGraphT.clone(), iTaskGraphMeta.clone(), iContractedTasks.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut levelNodes: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = levelNodes.clone();
            let mut tmpTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut tmpTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut contractedNodes: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = contractedNodes.clone();
            let mut tmpTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta;
            let mut flagValue: ArcStr = flagValue.clone();
            flagValue = (Flags::getConfigString(Flags::HPCOM_SCHEDULER.clone())?).clone();
            let true = (stringEq((flagValue.clone()).clone(), (literal!("levelfix")).clone())) else { bail!("pattern mismatch") };
            levelNodes = HpcOmTaskGraph::getLevelNodes(iTaskGraph.clone());
            contractedNodes = applyGRSForLevelFixScheduler(iTaskGraphMeta.clone(), iContractedTasks.clone(), levelNodes.clone(), metamodelica::nil())?;
            (tmpTaskGraph, tmpTaskGraphT, tmpTaskGraphMeta, _) = HpcOmTaskGraph::contractNodesInGraph(contractedNodes.clone(), iTaskGraph.clone(), iTaskGraphT.clone(), iTaskGraphMeta.clone(), iContractedTasks.clone())?;
            Ok((tmpTaskGraph.clone(), tmpTaskGraphT.clone(), tmpTaskGraphMeta.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok((iTaskGraph.clone(), iTaskGraphT.clone(), iTaskGraphMeta.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((oTaskGraph, oTaskGraphT, oTaskGraphMeta))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn applyGRSForLevelFixScheduler(mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iContractedTasks: metamodelica::Array<i32>, mut iLevelNodes: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut iContractedLevelfixTasks: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut oContractedLevelfixTasks: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut rest: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut head: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut sortedHead: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut sortedHeadArray: metamodelica::Array<i32>;
    let mut tmpContractedLevelfixTasks: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut bigTaskExecTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut tmpTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut tmpTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut tmpTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta;
    oContractedLevelfixTasks = (::match_deref::match_deref! { match &((iTaskGraphMeta.clone(), iContractedTasks.clone(), iLevelNodes.clone(), iContractedLevelfixTasks.clone())) {
        (HpcOmTaskGraph::TaskGraphMeta { inComps, exeCosts, .. }, _, Deref @ metamodelica::List::Cons { head: head, tail: rest }, _) => {
            sortedHead = List::sort(head.clone(), Arc::new({ let __pe_b2 = inComps.clone(); let __pe_b3 = exeCosts.clone(); let __pe_b4 = false; move |__pe_a0, __pe_a1| HpcOmTaskGraph::compareTasksByExecTime(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }))?;
            sortedHeadArray = metamodelica::arrayFromVec(sortedHead.clone().into_iter().cloned().collect());
            if intGt((sortedHeadArray.clone().borrow().len() as i32), 0) {
                bigTaskExecTime = HpcOmTaskGraph::getExeCostReqCycles(sortedHeadArray.clone().borrow()[((sortedHeadArray.clone().borrow().len() as i32)-1) as usize].clone(), iTaskGraphMeta.clone())?;
            } else {
                bigTaskExecTime = metamodelica::OrderedFloat(0.0_f64);
            }
            tmpContractedLevelfixTasks = applyGRSForLevelFixSchedulerLevel(iTaskGraphMeta.clone(), iContractedTasks.clone(), 500, sortedHeadArray.clone(), 1, ((sortedHeadArray.clone().borrow().len() as i32), metamodelica::nil(), bigTaskExecTime.clone()), iContractedLevelfixTasks.clone())?;
            tmpContractedLevelfixTasks = applyGRSForLevelFixScheduler(iTaskGraphMeta.clone(), iContractedTasks.clone(), rest.clone(), tmpContractedLevelfixTasks.clone())?;
            tmpContractedLevelfixTasks.clone()
        },
        _ => iContractedLevelfixTasks.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oContractedLevelfixTasks)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn applyGRSForLevelFixSchedulerLevel(mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iContractedTasks: metamodelica::Array<i32>, mut iCriticalSize: i32, mut iSortedLevelTasks: metamodelica::Array<i32>, mut iCurrentSmallTask: i32, mut iCurrentBigTask: (i32, Arc<metamodelica::List<i32>>, metamodelica::Real), mut iContractedLevelfixTasks: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut oContractedLevelfixTasks: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut tmpContractedTasks: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut head: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut bigTaskChilds: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut mergedGroupExecTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut bigTaskIdx: i32 = 0;
    let mut tmpTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut tmpTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut tmpTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta;
    oContractedLevelfixTasks = 'mc: {
        let __mc_input = (iTaskGraphMeta.clone(), iContractedTasks.clone(), iCriticalSize.clone(), iSortedLevelTasks.clone(), iCurrentSmallTask.clone(), iCurrentBigTask.clone(), iContractedLevelfixTasks.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, (bigTaskIdx, bigTaskChilds, mergedGroupExecTime), tmpContractedTasks) => {
                    let mut tmpContractedTasks = (*tmpContractedTasks).clone();
                    let true = (intLe(bigTaskIdx.clone(), iCurrentSmallTask.clone())) else { bail!("pattern mismatch") };
                    if !(bigTaskChilds.clone().is_empty()) {
                        tmpContractedTasks = cons(cons(iSortedLevelTasks.clone().borrow()[(bigTaskIdx.clone()-1) as usize].clone(), bigTaskChilds.clone()), tmpContractedTasks.clone());
                    }
                    Ok(tmpContractedTasks.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, (bigTaskIdx, bigTaskChilds, mergedGroupExecTime), _) => {
                    let mut mergedGroupExecTime = (*mergedGroupExecTime).clone();
                    let mut tmpContractedTasks: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = tmpContractedTasks.clone();
                    let true = (HpcOmTaskGraph::isNodeContracted(bigTaskIdx.clone(), iContractedTasks.clone())?) else { bail!("pattern mismatch") };
                    if intGt(bigTaskIdx.clone(), 1) {
                        mergedGroupExecTime = HpcOmTaskGraph::getExeCostReqCycles(iSortedLevelTasks.clone().borrow()[(bigTaskIdx.clone() - 1-1) as usize].clone(), iTaskGraphMeta.clone())?;
                    } else {
                        mergedGroupExecTime = metamodelica::OrderedFloat(0.0_f64);
                    }
                    tmpContractedTasks = applyGRSForLevelFixSchedulerLevel(iTaskGraphMeta.clone(), iContractedTasks.clone(), iCriticalSize.clone(), iSortedLevelTasks.clone(), iCurrentSmallTask.clone(), (bigTaskIdx.clone() - 1, metamodelica::nil(), mergedGroupExecTime.clone()), iContractedLevelfixTasks.clone())?;
                    Ok(tmpContractedTasks.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, (bigTaskIdx, bigTaskChilds, mergedGroupExecTime), _) => {
                    let mut tmpContractedTasks: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = tmpContractedTasks.clone();
                    let true = (HpcOmTaskGraph::isNodeContracted(iCurrentSmallTask.clone(), iContractedTasks.clone())?) else { bail!("pattern mismatch") };
                    tmpContractedTasks = applyGRSForLevelFixSchedulerLevel(iTaskGraphMeta.clone(), iContractedTasks.clone(), iCriticalSize.clone(), iSortedLevelTasks.clone(), iCurrentSmallTask.clone() + 1, (bigTaskIdx.clone(), bigTaskChilds.clone(), mergedGroupExecTime.clone()), iContractedLevelfixTasks.clone())?;
                    Ok(tmpContractedTasks.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, (bigTaskIdx, bigTaskChilds, mergedGroupExecTime), tmpContractedTasks) => {
                    let mut mergedGroupExecTime = (*mergedGroupExecTime).clone();
                    let mut tmpContractedTasks = (*tmpContractedTasks).clone();
                    mergedGroupExecTime = mergedGroupExecTime.clone() + HpcOmTaskGraph::getExeCostReqCycles(iSortedLevelTasks.clone().borrow()[(iCurrentSmallTask.clone()-1) as usize].clone(), iTaskGraphMeta.clone())?;
                    if realGe(mergedGroupExecTime.clone(), metamodelica::OrderedFloat((iCriticalSize.clone()) as f64)) {
                        if !(bigTaskChilds.clone().is_empty()) {
                            tmpContractedTasks = cons(cons(iSortedLevelTasks.clone().borrow()[(bigTaskIdx.clone()-1) as usize].clone(), bigTaskChilds.clone()), tmpContractedTasks.clone());
                        }
                        if intGt(bigTaskIdx.clone(), 1) {
                            mergedGroupExecTime = HpcOmTaskGraph::getExeCostReqCycles(iSortedLevelTasks.clone().borrow()[(bigTaskIdx.clone() - 1-1) as usize].clone(), iTaskGraphMeta.clone())?;
                        } else {
                            mergedGroupExecTime = metamodelica::OrderedFloat(0.0_f64);
                        }
                        tmpContractedTasks = applyGRSForLevelFixSchedulerLevel(iTaskGraphMeta.clone(), iContractedTasks.clone(), iCriticalSize.clone(), iSortedLevelTasks.clone(), iCurrentSmallTask.clone(), (bigTaskIdx.clone() - 1, metamodelica::nil(), mergedGroupExecTime.clone()), tmpContractedTasks.clone())?;
                    } else {
                        tmpContractedTasks = applyGRSForLevelFixSchedulerLevel(iTaskGraphMeta.clone(), iContractedTasks.clone(), iCriticalSize.clone(), iSortedLevelTasks.clone(), iCurrentSmallTask.clone() + 1, (bigTaskIdx.clone(), cons(iSortedLevelTasks.clone().borrow()[(iCurrentSmallTask.clone()-1) as usize].clone(), bigTaskChilds.clone()), mergedGroupExecTime.clone()), tmpContractedTasks.clone())?;
                    }
                    Ok(tmpContractedTasks.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iContractedLevelfixTasks.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oContractedLevelfixTasks)
}

fn GRS_newGraph(mut graphIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut metaIn: HpcOmTaskGraph::TaskGraphMeta, mut contrTasks: metamodelica::Array<i32>) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, HpcOmTaskGraph::TaskGraphMeta)> {
    let mut graphOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut metaOut: HpcOmTaskGraph::TaskGraphMeta;
    let mut newSize: i32 = 0;
    let mut notRemovedNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut removedNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut inCompsNew: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let HpcOmTaskGraph::TASKGRAPHMETA { inComps: __pa0, .. } = (metaIn.clone()) else { bail!("pattern mismatch") };
    inComps = __pa0.clone();
    notRemovedNodes = HpcOmTaskGraph::filterContractedNodes(List::intRange((graphIn.clone().borrow().len() as i32)), contrTasks.clone());
    removedNodes = HpcOmTaskGraph::filterNonContractedNodes(List::intRange((graphIn.clone().borrow().len() as i32)), contrTasks.clone());
    newSize = (notRemovedNodes.clone().len() as i32);
    graphOut = arrayCreate(newSize.clone(), metamodelica::nil());
    inCompsNew = arrayCreate(newSize.clone(), metamodelica::nil());
    (graphOut, inCompsNew) = GRS_newGraph2(notRemovedNodes.clone(), removedNodes.clone(), contrTasks.clone(), graphIn.clone(), inComps.clone(), graphOut.clone(), inCompsNew.clone(), 1)?;
    metaOut = HpcOmTaskGraph::setInCompsInMeta(inCompsNew.clone(), metaIn.clone())?;
    Ok((graphOut, metaOut))
}

fn GRS_newGraph2(mut origNodes: Arc<metamodelica::List<i32>>, mut removedNodes: Arc<metamodelica::List<i32>>, mut contrTasks: metamodelica::Array<i32>, mut origGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut origInComps: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut newGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut newInComps: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut newNode: i32) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>)> {
    let mut graphOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut inCompsOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    (graphOut, inCompsOut) = (::match_deref::match_deref! { match &((origNodes.clone(), removedNodes.clone(), contrTasks.clone(), origGraph.clone(), origInComps.clone(), newGraph.clone(), newInComps.clone(), newNode.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _) => {
            (newGraph.clone(), newInComps.clone())
        },
        (Deref @ metamodelica::List::Cons { head: node, tail: rest }, _, _, _, _, _, _, _) => {
            let mut row: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut comps: Arc<metamodelica::List<i32>> = metamodelica::nil();
            row = origGraph.clone().borrow()[(node.clone()-1) as usize].clone();
            row = HpcOmTaskGraph::filterContractedNodes(row.clone(), contrTasks.clone());
            row = HpcOmTaskGraph::updateContinuousEntriesInList(row.clone(), removedNodes.clone())?;
            comps = origInComps.clone().borrow()[(node.clone()-1) as usize].clone();
            {let _arr = newGraph.clone(); _arr.borrow_mut()[(newNode.clone()-1) as usize] = row.clone(); _arr};
            {let _arr = newInComps.clone(); _arr.borrow_mut()[(newNode.clone()-1) as usize] = comps.clone(); _arr};
            GRS_newGraph2(rest.clone(), removedNodes.clone(), contrTasks.clone(), origGraph.clone(), origInComps.clone(), newGraph.clone(), newInComps.clone(), newNode.clone() + 1)?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((graphOut, inCompsOut))
}

fn createSchedule(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut iFilenamePrefix: ArcStr, mut iNumProc: i32, mut iNumProcToUse: i32, mut iSimCode: SimCode::SimCode, mut iScheduledTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut iSystemName: ArcStr, mut iSchedulerName: ArcStr) -> Result<(Arc<HpcOmSimCode::Schedule>, SimCode::SimCode, metamodelica::Array<Arc<metamodelica::List<i32>>>, HpcOmTaskGraph::TaskGraphMeta, metamodelica::Array<Arc<metamodelica::List<i32>>>)> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut oSimCode: SimCode::SimCode;
    let mut oTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut oTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta;
    let mut oSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut knownScheduler: Arc<metamodelica::List<ArcStr>> = list![(literal!("none")).clone(), (literal!("level")).clone(), (literal!("levelfix")).clone(), (literal!("ext")).clone(), (literal!("metis")).clone(), (literal!("hmet")).clone(), (literal!("listr")).clone(), (literal!("rand")).clone(), (literal!("list")).clone(), (literal!("mcp")).clone(), (literal!("part")).clone(), (literal!("taskdep")).clone(), (literal!("tds")).clone(), (literal!("bls")).clone(), (literal!("sbs")).clone(), (literal!("sts")).clone()];
    let mut schedulerName: ArcStr = iSchedulerName.clone();
    let mut tmpSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut numProcToUse: i32 = iNumProcToUse.clone();
    if boolNot(List::exist1(knownScheduler.clone(), Arc::new(fnptr!(stringEq, ArcStr, ArcStr)), (schedulerName.clone()).clone())) {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("HpcOmScheduler.createSchedule warning: The scheduler '")); __mm_s.push_str(&*iSchedulerName.clone()); __mm_s.push_str(&*literal!("' is unknown. The list-scheduling algorithm is used instead for the ")); __mm_s.push_str(&*iSystemName.clone()); __mm_s.push_str(&*literal!(".\n")); ArcStr::from(__mm_s) }).clone());
        schedulerName = (literal!("list")).clone();
    }
    if intGt(iNumProcToUse.clone(), iNumProc.clone()) {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("HpcOmScheduler.createSchedule warning: Cannot schedule the the task graph to ")); __mm_s.push_str(&*intString(iNumProcToUse.clone())); __mm_s.push_str(&*literal!(" processors, because the number is larger than the available processors (")); __mm_s.push_str(&*intString(iNumProc.clone())); __mm_s.push_str(&*literal!(").\n")); ArcStr::from(__mm_s) }).clone());
        numProcToUse = iNumProc.clone();
    }
    (tmpSchedule, oSimCode, oTaskGraph, oTaskGraphMeta, oSccSimEqMapping) = createSchedule1(iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone(), (iFilenamePrefix.clone()).clone(), numProcToUse.clone(), iSimCode.clone(), iScheduledTasks.clone(), (iSystemName.clone()).clone(), (schedulerName.clone()).clone())?;
    oSchedule = HpcOmScheduler::expandSchedule(iNumProc.clone(), numProcToUse.clone(), tmpSchedule.clone())?;
    Ok((oSchedule, oSimCode, oTaskGraph, oTaskGraphMeta, oSccSimEqMapping))
}

fn createSchedule1(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut iFilenamePrefix: ArcStr, mut iNumProc: i32, mut iSimCode: SimCode::SimCode, mut iScheduledTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut iSystemName: ArcStr, mut iSchedulerName: ArcStr) -> Result<(Arc<HpcOmSimCode::Schedule>, SimCode::SimCode, metamodelica::Array<Arc<metamodelica::List<i32>>>, HpcOmTaskGraph::TaskGraphMeta, metamodelica::Array<Arc<metamodelica::List<i32>>>)> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut oSimCode: SimCode::SimCode;
    let mut oTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut oTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta;
    let mut oSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut sccSimEqMap: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut schedule: Arc<HpcOmSimCode::Schedule>;
    let mut taskGraph1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut taskGraphMeta1: HpcOmTaskGraph::TaskGraphMeta;
    let mut simCode: SimCode::SimCode;
    (oSchedule, oSimCode, oTaskGraph, oTaskGraphMeta, oSccSimEqMapping) = 'mc: {
        let __mc_input = (iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone(), iFilenamePrefix.clone(), iNumProc.clone(), iSimCode.clone(), iScheduledTasks.clone(), iSystemName.clone(), iSchedulerName.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, Deref @ "none") => {
                    let mut schedule: Arc<HpcOmSimCode::Schedule>;
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Using serial code for the ")); __mm_s.push_str(&*iSystemName.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    schedule = HpcOmScheduler::createEmptySchedule(iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone())?;
                    Ok((schedule.clone(), iSimCode.clone(), iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, Deref @ "level") => {
                    let mut schedule: Arc<HpcOmSimCode::Schedule>;
                    let mut taskGraphMeta1: HpcOmTaskGraph::TaskGraphMeta;
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Using level Scheduler for the ")); __mm_s.push_str(&*iSystemName.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    (schedule, taskGraphMeta1) = HpcOmScheduler::createLevelSchedule(iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone());
                    Ok((schedule.clone(), iSimCode.clone(), iTaskGraph.clone(), taskGraphMeta1.clone(), iSccSimEqMapping.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, Deref @ "levelfix") => {
                    let mut taskGraphMeta1: HpcOmTaskGraph::TaskGraphMeta;
                    let mut schedule: Arc<HpcOmSimCode::Schedule>;
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Using fixed level Scheduler (experimental) for the ")); __mm_s.push_str(&*iSystemName.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    (schedule, taskGraphMeta1) = HpcOmScheduler::createFixedLevelSchedule(iTaskGraph.clone(), iTaskGraphMeta.clone(), iNumProc.clone(), iSccSimEqMapping.clone());
                    Ok((schedule.clone(), iSimCode.clone(), iTaskGraph.clone(), taskGraphMeta1.clone(), iSccSimEqMapping.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, Deref @ "ext") => {
                    let mut schedule: Arc<HpcOmSimCode::Schedule>;
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Using external Scheduler for the ")); __mm_s.push_str(&*iSystemName.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    schedule = HpcOmScheduler::createExtSchedule(iTaskGraph.clone(), iTaskGraphMeta.clone(), iNumProc.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("taskGraph")); __mm_s.push_str(&*iFilenamePrefix.clone()); __mm_s.push_str(&*literal!("_ext.graphml")); ArcStr::from(__mm_s) }).clone())?;
                    Ok((schedule.clone(), iSimCode.clone(), iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, Deref @ "metis") => {
                    let mut schedule: Arc<HpcOmSimCode::Schedule>;
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Using METIS Scheduler for the ")); __mm_s.push_str(&*iSystemName.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    schedule = HpcOmScheduler::createMetisSchedule(iTaskGraph.clone(), iTaskGraphMeta.clone(), iNumProc.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone())?;
                    Ok((schedule.clone(), iSimCode.clone(), iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, Deref @ "hmet") => {
                    let mut schedule: Arc<HpcOmSimCode::Schedule>;
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Using hMETIS Scheduler for the ")); __mm_s.push_str(&*iSystemName.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    schedule = HpcOmScheduler::createHMetisSchedule(iTaskGraph.clone(), iTaskGraphMeta.clone(), iNumProc.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone())?;
                    Ok((schedule.clone(), iSimCode.clone(), iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, Deref @ "listr") => {
                    let mut schedule: Arc<HpcOmSimCode::Schedule>;
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Using list reverse Scheduler for the ")); __mm_s.push_str(&*iSystemName.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    schedule = HpcOmScheduler::createListScheduleReverse(iTaskGraph.clone(), iTaskGraphMeta.clone(), iNumProc.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone())?;
                    Ok((schedule.clone(), iSimCode.clone(), iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, Deref @ "rand") => {
                    let mut schedule: Arc<HpcOmSimCode::Schedule>;
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Using Random Scheduler for the ")); __mm_s.push_str(&*iSystemName.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    schedule = HpcOmScheduler::createRandomSchedule(iTaskGraph.clone(), iTaskGraphMeta.clone(), iNumProc.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone())?;
                    Ok((schedule.clone(), iSimCode.clone(), iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, Deref @ "list") => {
                    let mut schedule: Arc<HpcOmSimCode::Schedule>;
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Using list Scheduler for the ")); __mm_s.push_str(&*iSystemName.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    schedule = HpcOmScheduler::createListSchedule(iTaskGraph.clone(), iTaskGraphMeta.clone(), iNumProc.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone())?;
                    Ok((schedule.clone(), iSimCode.clone(), iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, Deref @ "mcp") => {
                    let mut schedule: Arc<HpcOmSimCode::Schedule>;
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Using Modified Critical Path Scheduler for the ")); __mm_s.push_str(&*iSystemName.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    schedule = HpcOmScheduler::createMCPschedule(iTaskGraph.clone(), iTaskGraphMeta.clone(), iNumProc.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone())?;
                    Ok((schedule.clone(), iSimCode.clone(), iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, Deref @ "part") => {
                    let mut schedule: Arc<HpcOmSimCode::Schedule>;
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Using partition Scheduler for the ")); __mm_s.push_str(&*iSystemName.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    schedule = HpcOmScheduler::createPartSchedule(iTaskGraph.clone(), iTaskGraphMeta.clone(), iNumProc.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone())?;
                    Ok((schedule.clone(), iSimCode.clone(), iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, Deref @ "taskdep") => {
                    let mut schedule: Arc<HpcOmSimCode::Schedule>;
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Using dynamic task dependencies for the ")); __mm_s.push_str(&*iSystemName.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    schedule = HpcOmScheduler::createTaskDepSchedule(iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone())?;
                    Ok((schedule.clone(), iSimCode.clone(), iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, Deref @ "tds") => {
                    let mut schedule: Arc<HpcOmSimCode::Schedule>;
                    let mut taskGraph1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut taskGraphMeta1: HpcOmTaskGraph::TaskGraphMeta;
                    let mut sccSimEqMap: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut simCode: SimCode::SimCode;
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Using Task Duplication-based Scheduling for the ")); __mm_s.push_str(&*iSystemName.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    (schedule, simCode, taskGraph1, taskGraphMeta1, sccSimEqMap) = HpcOmScheduler::TDS_schedule(iTaskGraph.clone(), iTaskGraphMeta.clone(), iNumProc.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone(), iSimCode.clone())?;
                    Ok((schedule.clone(), simCode.clone(), taskGraph1.clone(), taskGraphMeta1.clone(), sccSimEqMap.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, Deref @ "bls") => {
                    let mut schedule: Arc<HpcOmSimCode::Schedule>;
                    let mut taskGraphMeta1: HpcOmTaskGraph::TaskGraphMeta;
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Using Balanced Level Scheduling for the ")); __mm_s.push_str(&*iSystemName.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    (schedule, taskGraphMeta1) = HpcOmScheduler::createBalancedLevelScheduling(iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone())?;
                    Ok((schedule.clone(), iSimCode.clone(), iTaskGraph.clone(), taskGraphMeta1.clone(), iSccSimEqMapping.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, Deref @ "sbs") => {
                    let mut schedule: Arc<HpcOmSimCode::Schedule>;
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Using Single Block Scheduling for the ")); __mm_s.push_str(&*iSystemName.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    schedule = HpcOmEqSystems::createSingleBlockSchedule(iTaskGraph.clone(), iTaskGraphMeta.clone(), iScheduledTasks.clone(), iSccSimEqMapping.clone())?;
                    Ok((schedule.clone(), iSimCode.clone(), iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, Deref @ "sts") => {
                    let mut schedule: Arc<HpcOmSimCode::Schedule>;
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Using Single Thread Scheduling for the ")); __mm_s.push_str(&*iSystemName.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    schedule = HpcOmScheduler::createSingleThreadSchedule(iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone(), iNumProc.clone())?;
                    Ok((schedule.clone(), iSimCode.clone(), iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut schedule: Arc<HpcOmSimCode::Schedule>;
                    println!("{}", (literal!("HpcOmSimCode.createSchedule failed!\n")).clone());
                    schedule = HpcOmScheduler::createEmptySchedule(iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone())?;
                    Ok((schedule.clone(), iSimCode.clone(), iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((oSchedule, oSimCode, oTaskGraph, oTaskGraphMeta, oSccSimEqMapping))
}

// test functions
//------------------------------------------
//------------------------------------------
fn checkOdeSystemSize(mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iOdeEqs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>>>, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<bool> {
    let mut oIsCorrect: bool = false;
    let mut scc: i32 = 0;
    let mut sccs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut actualSizePre: i32 = 0;
    let mut actualSize: i32 = 0;
    let mut targetSize: i32 = 0;
    sccs = List::sort(HpcOmTaskGraph::getAllSCCsOfGraph(iTaskGraphMeta.clone())?, Arc::new(fnptr!(intGt, i32, i32)))?;
    actualSizePre = (sccs.clone().len() as i32);
    actualSize = (List::sortedUnique(sccs.clone(), Arc::new(fnptr!(intEq, i32, i32)))?.len() as i32);
    if intNe(actualSizePre.clone(), actualSize.clone()) {
        println!("{}", (literal!("There are simCode-equations multiple times in the graph structure.\n")).clone());
    }
    actualSize = 0;
    for mut scc in &*sccs.clone() {
        let mut scc = scc.clone();
        actualSize = actualSize.clone() + (iSccSimEqMapping.clone().borrow()[(scc.clone()-1) as usize].clone().len() as i32);
    }
    targetSize = (List::flatten(iOdeEqs.clone()).len() as i32);
    oIsCorrect = intEq(targetSize.clone(), actualSize.clone());
    if oIsCorrect.clone() {
    } else {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("the size of the ODE-system should be ")); __mm_s.push_str(&*intString(targetSize.clone())); __mm_s.push_str(&*literal!(" but it is ")); __mm_s.push_str(&*intString(actualSize.clone())); __mm_s.push_str(&*literal!("!\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("expected the following sim code equations: ")); __mm_s.push_str(&*stringDelimitList(List::map(List::map(List::flatten(iOdeEqs.clone()), Arc::new(SimCodeUtil::simEqSystemIndex)), Arc::new(fnptr!(intString, i32))), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", (literal!("the ODE-system is NOT correct\n")).clone());
    }
    Ok(oIsCorrect)
}

fn checkTaskGraphMetaConsistency(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iSystemName: ArcStr) -> Result<bool> {
    let mut oIsCorrect: bool = false;
    let mut numberOfNodes: i32 = 0;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    numberOfNodes = (iTaskGraph.clone().borrow().len() as i32);
    let HpcOmTaskGraph::TASKGRAPHMETA { inComps: __pa0, .. } = (iTaskGraphMeta.clone()) else { bail!("pattern mismatch") };
    inComps = __pa0.clone();
    if boolNot(intEq(numberOfNodes.clone(), (inComps.clone().borrow().len() as i32))) {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("the number of nodes in the ")); __mm_s.push_str(&*iSystemName.clone()); __mm_s.push_str(&*literal!(" task graph (")); __mm_s.push_str(&*intString(numberOfNodes.clone())); __mm_s.push_str(&*literal!(") is distinguished from the number of nodes in task graph meta (")); __mm_s.push_str(&*intString((inComps.clone().borrow().len() as i32))); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone());
        oIsCorrect = false;
    } else {
        oIsCorrect = true;
    }
    Ok(oIsCorrect)
}

fn checkEquationCount(mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iSystemName: ArcStr, mut iExpectedNumberOfEqs: i32, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<bool> {
    let mut oIsCorrect: bool = false;
    let mut inCompsIdx: i32 = 0;
    let mut eqCount: i32 = 0;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut comps: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut compEqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let HpcOmTaskGraph::TASKGRAPHMETA { inComps: __pa0, .. } = (iTaskGraphMeta.clone()) else { bail!("pattern mismatch") };
    inComps = __pa0.clone();
    inCompsIdx = (inComps.clone().borrow().len() as i32);
    eqCount = 0;
    while intGt(inCompsIdx.clone(), 0) {
        comps = inComps.clone().borrow()[(inCompsIdx.clone()-1) as usize].clone();
        for mut comp in &*comps.clone() {
            let mut comp = comp.clone();
            compEqs = iSccSimEqMapping.clone().borrow()[(comp.clone()-1) as usize].clone();
            eqCount = eqCount.clone() + (compEqs.clone().len() as i32);
        }
        inCompsIdx = inCompsIdx.clone() - 1;
    }
    oIsCorrect = intEq(iExpectedNumberOfEqs.clone(), eqCount.clone());
    if boolNot(oIsCorrect.clone()) {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("the number of equations in the ")); __mm_s.push_str(&*iSystemName.clone()); __mm_s.push_str(&*literal!(" task graph (")); __mm_s.push_str(&*intString(eqCount.clone())); __mm_s.push_str(&*literal!(") is distinguished from the expected number of equations (")); __mm_s.push_str(&*intString(iExpectedNumberOfEqs.clone())); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(oIsCorrect)
}

/*
protected function repeatScheduleWithOtherNumProc "author:Waurich TUD 2013-011
  checks if the scheduling with the given numProc is fine.
 if n=auto, more cores are available and more speedup could be achieved repeat schedule with increased num of procs."
  input HpcOmTaskGraph.TaskGraph taskGraphIn;
  input HpcOmTaskGraph.TaskGraphMeta taskGraphMetaIn;
  input array<list<Integer>> sccSimEqMappingIn;
  input String fileNamePrefix;
  input Real cpCostsWoC;
  input HpcOmSimCode.Schedule scheduleIn;
  input Integer numProcIn;
  input Boolean numFixed;
  output HpcOmSimCode.Schedule scheduleOut;
  output Integer numProcOut;
protected
  Integer maxNumProc, maxIter;
  Real maxDiff;
algorithm
  maxNumProc := System.numProcessors();
  maxIter := 3;
  maxDiff := 0.5;
  (scheduleOut,numProcOut,_) := repeatScheduleWithOtherNumProc1(taskGraphIn,taskGraphMetaIn,sccSimEqMappingIn,fileNamePrefix,cpCostsWoC,scheduleIn,numProcIn,numFixed,maxNumProc,maxDiff,maxIter);
end repeatScheduleWithOtherNumProc;


protected function repeatScheduleWithOtherNumProc1 "author:Waurich TUD 2013-011
  checks if the scheduling with the given numProc is fine.
 if n=auto, more cores are available and more speedup could be achieved repeat schedule with increased num of procs."
  input HpcOmTaskGraph.TaskGraph taskGraphIn;
  input HpcOmTaskGraph.TaskGraphMeta taskGraphMetaIn;
  input BackendDAE.BackendDAE inDAE;
  input array<list<Integer>> sccSimEqMappingIn;
  input String fileNamePrefix;
  input Real cpCostsWoC;
  input HpcOmSimCode.Schedule scheduleIn;
  input Integer numProcIn;
  input Boolean numFixed;
  input Integer maxNumProc;
  input Real maxDiff;
  input Integer numIterIn;
  output HpcOmSimCode.Schedule scheduleOut;
  output Integer numProcOut;
  output Integer numIterOut;
algorithm
  (scheduleOut,numProcOut,numIterOut) := matchcontinue(taskGraphIn,taskGraphMetaIn,inDAE,sccSimEqMappingIn,fileNamePrefix,cpCostsWoC,scheduleIn,numProcIn,numFixed,maxNumProc,maxDiff,numIterIn)
    local
      Boolean scheduleAgain;
      Integer numProc, numIt;
      Real serTime,parTime,speedup,speedUp,speedUpMax,diff;
      HpcOmSimCode.Schedule schedule;
    case(_,_,_,_,_,_,_,_,true,_,_,_)
      equation // do not schedule again because the number of procs was given
        then
          (scheduleIn,numProcIn,0);
    case(_,_,_,_,_,_,_,_,false,_,_,_)
      algorithm
        true = numIterIn == 0; // the max number of schedules with increased num of procs
        then
          (scheduleIn,numProcIn,0);
    case(_,_,_,_,_,_,_,_,false,_,_,_)
      algorithm
        (_,_,speedUp,speedUpMax) = HpcOmScheduler.predictExecutionTime(scheduleIn,SOME(cpCostsWoC),numProcIn,taskGraphIn,taskGraphMetaIn);
        diff = speedUpMax -. speedUp;
        //print("the new speedUp with "+intString(numProcIn)+" processors: "+realString(speedUp)+"\n");
        true = diff <. maxDiff;
        //print("the schedule is fine\n");
      then
        (scheduleIn,numProcIn,numIterIn);
    else
      algorithm
        numProc = numProcIn+1; // increase the number of procs
        numIt = numIterIn-1; // lower the counter of scheduling runs
        scheduleAgain = intLe(numProc,maxNumProc);
        //print("schedule again\n");
        numProc = if_(scheduleAgain,numProc,numProcIn);
        numIt = if_(scheduleAgain,numIt,0);
        schedule= Debug.bcallret6(scheduleAgain,createSchedule,taskGraphIn,taskGraphMetaIn,sccSimEqMappingIn,fileNamePrefix,numProc,scheduleIn);
        (schedule,numProc,numIt) = repeatScheduleWithOtherNumProc1(taskGraphIn,taskGraphMetaIn,sccSimEqMappingIn,fileNamePrefix,cpCostsWoC,schedule,numProc,numFixed,maxNumProc,maxDiff,numIt);
      then
        (schedule,numProc,numIt);
  end matchcontinue;
end repeatScheduleWithOtherNumProc1;
*/
//----------------------------
// output data about operations in equations and composition of systems of equations
//----------------------------
pub fn outputTimeBenchmark(mut graphData: HpcOmTaskGraph::TaskGraphMeta, mut dae: Arc<BackendDAE::BackendDAE>) -> Result<()> {
    let mut eqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut numCycles: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(dae.clone()) {
        Deref @ DAE { shared: __pa0, eqs: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    shared = __pa0.clone();
    eqSystems = __pa1.clone();
    let HpcOmTaskGraph::TASKGRAPHMETA { exeCosts: __pa2, .. } = (graphData.clone()) else { bail!("pattern mismatch") };
    exeCosts = __pa2.clone();
    numCycles = List::mapArray(exeCosts.clone(), Arc::new(fnptr!(Util::tuple22, _)));
    println!("{}", (literal!("start cost benchmark\n")).clone());
    outputTimeBenchmark2(BackendDAEUtil::getStrongComponents(listHead(eqSystems.clone())?), numCycles.clone(), eqSystems.clone(), shared.clone(), 1)?;
    println!("{}", (literal!("finish cost benchmark\n")).clone());
    Ok(())
}

fn outputTimeBenchmark2(mut compsIn: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut numCycles: Arc<metamodelica::List<metamodelica::Real>>, mut eqSystemsIn: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, mut shared: Arc<BackendDAE::Shared>, mut compIdx: i32) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (compsIn.clone(), numCycles.clone(), eqSystemsIn.clone(), shared.clone(), compIdx.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, Deref @ metamodelica::List::Cons { head: _, tail: eqSysRest }, _, _) => {
                    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
                    comps = BackendDAEUtil::getStrongComponents(listHead(eqSysRest.clone())?);
                    outputTimeBenchmark2(comps.clone(), numCycles.clone(), eqSysRest.clone(), shared.clone(), compIdx.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: comp, tail: comps }, Deref @ metamodelica::List::Cons { head: exeCost, tail: restCosts }, Deref @ metamodelica::List::Cons { head: eqSys, tail: _ }, _, _) => {
                    let mut estimate: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut compInfo: Arc<BackendDAE::CompInfo>;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendDAEOptimize::countOperationstraverseComps(list![comp.clone()], eqSys.clone(), shared.clone(), metamodelica::nil())?) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    compInfo = __pa0.clone();
                    (_, estimate) = HpcOmTaskGraph::calculateCosts(compInfo.clone())?;
                    BackendDump::dumpCompInfo(compInfo.clone())?;
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("task")); __mm_s.push_str(&*intString(compIdx.clone())); __mm_s.push_str(&*literal!("-> measured: ")); __mm_s.push_str(&*intString(((exeCost.clone()).0 as i32))); __mm_s.push_str(&*literal!(" and estimated: ")); __mm_s.push_str(&*intString(((estimate.clone()).0 as i32))); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
                    outputTimeBenchmark2(comps.clone(), restCosts.clone(), eqSystemsIn.clone(), shared.clone(), compIdx.clone() + 1)?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

