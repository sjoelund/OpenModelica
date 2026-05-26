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
use crate::BackendVarTransform;
use crate::HashTableCrefSimVar;
use crate::HpcOmSchedulerExt;
use crate::HpcOmSimCode;
use crate::HpcOmSimCodeMain;
use crate::HpcOmTaskGraph;
use crate::SimCode;
use crate::SimCodeUtil;
use crate::SimCodeVar;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::Expression;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::FlagsUtil;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

pub type TaskAssignment = metamodelica::Array<i32>;

//the information which node <idx> is assigned to which processor <value>
//--------------
// No Scheduling
//--------------
pub fn createEmptySchedule(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<HpcOmSimCode::Schedule>> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut sortedTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut allTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
    let mut taskIdx: i32 = 0;
    let mut weighting: i32 = 0;
    let mut index: i32 = 0;
    let mut threadIdx: i32 = 0;
    let mut calcTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut timeFinished: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut eqIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), (iTaskGraph.clone().borrow().len() as i32))?;
    allCalcTasks = convertTaskGraphToTasks(taskGraphT.clone(), iTaskGraphMeta.clone(), Arc::new(convertNodeToTask))?;
    let __range0 = &*List::intRange((allCalcTasks.clone().borrow().len() as i32)).reverse();
    for mut taskIdx in __range0 {
        let mut taskIdx = taskIdx.clone();
        let (__pa1, __pa2, __pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(allCalcTasks.clone().borrow()[(taskIdx.clone()-1) as usize].clone()) {
            (Deref @ HpcOmSimCode::Task::CALCTASK { weighting: __pa1, index: __pa2, calcTime: __pa3, timeFinished: __pa4, threadIdx: __pa5, eqIdc: __pa6 }, _) => (__pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
            _ => bail!("pattern mismatch"),
        } };
        weighting = __pa1.clone();
        index = __pa2.clone();
        calcTime = __pa3.clone();
        timeFinished = __pa4.clone();
        threadIdx = __pa5.clone();
        eqIdc = __pa6.clone();
        eqIdc = List::map(List::map1(eqIdc.clone(), Arc::new(getSimEqSysIdxForComp), iSccSimEqMapping.clone()), Arc::new(List::last));
        allTasks = cons(Arc::new(HpcOmSimCode::Task::CALCTASK { weighting: weighting.clone(), index: index.clone(), calcTime: calcTime.clone(), timeFinished: timeFinished.clone(), threadIdx: threadIdx.clone(), eqIdc: eqIdc.clone() }), allTasks.clone());
    }
    allTasks = List::sort(allTasks.clone(), Arc::new(compareTasksByEqIdc))?;
    oSchedule = Arc::new(HpcOmSimCode::Schedule::EMPTYSCHEDULE { tasks: HpcOmSimCode::TaskList::SERIALTASKLIST { tasks: allTasks.clone(), masterOnly: true } });
    Ok(oSchedule)
}

//----------------
// List Scheduling
//----------------
pub fn createListSchedule(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iNumberOfThreads: i32, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<Arc<HpcOmSimCode::Schedule>> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut nodeList_refCount: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = metamodelica::nil();
    let mut nodeList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut rootNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut threadReadyTimes: metamodelica::Array<metamodelica::Real>;
    let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
    let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>;
    let mut tmpSchedule: Arc<HpcOmSimCode::Schedule>;
    let HpcOmTaskGraph::TASKGRAPHMETA { inComps: __pa0, commCosts: __pa1, .. } = (iTaskGraphMeta.clone()) else { bail!("pattern mismatch") };
    inComps = __pa0.clone();
    commCosts = __pa1.clone();
    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), (iTaskGraph.clone().borrow().len() as i32))?;
    rootNodes = HpcOmTaskGraph::getRootNodes(iTaskGraph.clone())?;
    allCalcTasks = convertTaskGraphToTasks(taskGraphT.clone(), iTaskGraphMeta.clone(), Arc::new(convertNodeToTask))?;
    nodeList_refCount = List::map1(rootNodes.clone(), Arc::new(getTaskByIndex), allCalcTasks.clone());
    nodeList = List::map(nodeList_refCount.clone(), Arc::new(fnptr!(Util::tuple21, _)));
    nodeList = List::sort(nodeList.clone(), Arc::new(compareTasksByWeighting))?;
    threadReadyTimes = arrayCreate(iNumberOfThreads.clone(), metamodelica::OrderedFloat(0.0_f64));
    threadTasks = arrayCreate(iNumberOfThreads.clone(), metamodelica::nil());
    tmpSchedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTasks.clone(), outgoingDepTasks: metamodelica::nil(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
    (tmpSchedule, _) = createListSchedule1(nodeList.clone(), threadReadyTimes.clone(), iTaskGraph.clone(), taskGraphT.clone(), commCosts.clone(), inComps.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone(), Arc::new(fnptr!(getLocksByPredecessorList, Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, i32, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>)), tmpSchedule.clone())?;
    tmpSchedule = addSuccessorLocksToSchedule(iTaskGraph.clone(), Arc::new(addReleaseLocksToSchedule), commCosts.clone(), inComps.clone(), iSimVarMapping.clone(), tmpSchedule.clone())?;
    oSchedule = setScheduleLockIds(tmpSchedule.clone())?;
    Ok(oSchedule)
}

fn createListSchedule1(mut iNodeList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut iThreadReadyTimes: metamodelica::Array<metamodelica::Real>, mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut iLockWithPredecessorHandler: Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, i32, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)> + 'static>, mut iSchedule: Arc<HpcOmSimCode::Schedule>) -> Result<(Arc<HpcOmSimCode::Schedule>, metamodelica::Array<metamodelica::Real>)> {
    pub type FuncType = fn(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, i32, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)>;

    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut oThreadReadyTimes: metamodelica::Array<metamodelica::Real>;
    let mut head: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut newTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut newTaskRefCount: i32 = 0;
    let mut rest: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut lastChildFinishTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut lastChild: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut predecessors: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = metamodelica::nil();
    let mut successors: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = metamodelica::nil();
    let mut successorIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut outgoingDepTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut newOutgoingDepTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut threadFinishTimes: metamodelica::Array<metamodelica::Real>;
    let mut firstEq: i32 = 0;
    let mut allThreadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut threadTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut lockTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut threadId: i32 = 0;
    let mut threadFinishTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut tmpThreadReadyTimes: metamodelica::Array<metamodelica::Real>;
    let mut tmpNodeList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut weighting: i32 = 0;
    let mut index: i32 = 0;
    let mut calcTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut eqIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut simEqIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
    (oSchedule, oThreadReadyTimes) = (::match_deref::match_deref! { match &((iNodeList.clone(), iThreadReadyTimes.clone(), iTaskGraph.clone(), iTaskGraphT.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone(), iLockWithPredecessorHandler.clone(), iSchedule.clone())) {
        (Deref @ metamodelica::List::Cons { head: head @ Deref @ HpcOmSimCode::Task::CALCTASK { eqIdc: eqIdc @ Deref @ metamodelica::List::Cons { head: firstEq, tail: _ }, calcTime, index, weighting, .. }, tail: rest }, _, _, _, _, _, _, _, _, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { allCalcTasks, outgoingDepTasks, threadTasks: allThreadTasks, .. }) => {
            let mut allCalcTasks = (*allCalcTasks).clone();
            let mut outgoingDepTasks = (*outgoingDepTasks).clone();
            let mut allThreadTasks = (*allThreadTasks).clone();
            (predecessors, _) = getSuccessorsByTask(head.clone(), iTaskGraphT.clone(), allCalcTasks.clone())?;
            (successors, successorIdc) = getSuccessorsByTask(head.clone(), iTaskGraph.clone(), allCalcTasks.clone())?;
            if boolNot(predecessors.clone().is_empty()) {
                lastChild = getTaskWithHighestFinishTime(predecessors.clone(), None)?;
                let __pa0 = ::match_deref::match_deref! { match &(lastChild.clone()) {
                    Deref @ HpcOmSimCode::Task::CALCTASK { timeFinished: __pa0, .. } => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                lastChildFinishTime = __pa0.clone();
            } else {
                lastChildFinishTime = metamodelica::OrderedFloat(0.0_f64);
            }
            threadFinishTimes = calculateFinishTimes(lastChildFinishTime.clone(), head.clone(), predecessors.clone(), iCommCosts.clone(), iThreadReadyTimes.clone())?;
            (threadId, threadFinishTime) = getThreadFinishTimesMin(1, threadFinishTimes.clone(), -1, metamodelica::OrderedFloat(0.0_f64))?;
            tmpThreadReadyTimes = {let _arr = iThreadReadyTimes.clone(); _arr.borrow_mut()[(threadId.clone()-1) as usize] = threadFinishTime.clone(); _arr};
            threadTasks = allThreadTasks.clone().borrow()[(threadId.clone()-1) as usize].clone();
            if boolNot(predecessors.clone().is_empty()) {
                (lockTasks, newOutgoingDepTasks) = iLockWithPredecessorHandler(head.clone(), predecessors.clone(), threadId.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone())?;
                outgoingDepTasks = listAppend(outgoingDepTasks.clone(), newOutgoingDepTasks.clone());
                threadTasks = listAppend(lockTasks.clone(), threadTasks.clone());
                simEqIdc = List::map(List::map1(eqIdc.clone(), Arc::new(getSimEqSysIdxForComp), iSccSimEqMapping.clone()), Arc::new(List::last));
            } else {
                simEqIdc = List::flatten(List::map1(eqIdc.clone(), Arc::new(getSimEqSysIdxForComp), iSccSimEqMapping.clone()));
            }
            newTask = Arc::new(HpcOmSimCode::Task::CALCTASK { weighting: weighting.clone(), index: index.clone(), calcTime: calcTime.clone(), timeFinished: threadFinishTime.clone(), threadIdx: threadId.clone(), eqIdc: simEqIdc.clone() });
            threadTasks = cons(newTask.clone(), threadTasks.clone());
            allThreadTasks = {let _arr = allThreadTasks.clone(); _arr.borrow_mut()[(threadId.clone()-1) as usize] = threadTasks.clone(); _arr};
            (allCalcTasks, tmpNodeList) = updateRefCounterBySuccessorIdc(allCalcTasks.clone(), successorIdc.clone(), metamodelica::nil())?;
            tmpNodeList = listAppend(tmpNodeList.clone(), rest.clone());
            tmpNodeList = List::sort(tmpNodeList.clone(), Arc::new(compareTasksByWeighting))?;
            (_, newTaskRefCount) = allCalcTasks.clone().borrow()[(index.clone()-1) as usize].clone();
            {let _arr = allCalcTasks.clone(); _arr.borrow_mut()[(index.clone()-1) as usize] = (newTask.clone(), newTaskRefCount.clone()); _arr};
            (tmpSchedule, tmpThreadReadyTimes) = createListSchedule1(tmpNodeList.clone(), tmpThreadReadyTimes.clone(), iTaskGraph.clone(), iTaskGraphT.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone(), iLockWithPredecessorHandler.clone(), Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: allThreadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() }))?;
            (tmpSchedule.clone(), tmpThreadReadyTimes.clone())
        },
        (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, _, _) => (iSchedule.clone(), iThreadReadyTimes.clone()),
        _ => {
            println!("{}", (literal!("HpcOmScheduler.createListSchedule1 failed\n")).clone());
            (iSchedule.clone(), iThreadReadyTimes.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oSchedule, oThreadReadyTimes))
}

//----------------
// Random Scheduling
//----------------
pub fn createRandomSchedule(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iNumberOfThreads: i32, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<Arc<HpcOmSimCode::Schedule>> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut nodeList_refCount: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = metamodelica::nil();
    let mut nodeList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut rootNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut threadReadyTimes: metamodelica::Array<metamodelica::Real>;
    let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
    let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>;
    let mut tmpSchedule: Arc<HpcOmSimCode::Schedule>;
    let HpcOmTaskGraph::TASKGRAPHMETA { inComps: __pa0, commCosts: __pa1, .. } = (iTaskGraphMeta.clone()) else { bail!("pattern mismatch") };
    inComps = __pa0.clone();
    commCosts = __pa1.clone();
    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), (iTaskGraph.clone().borrow().len() as i32))?;
    rootNodes = HpcOmTaskGraph::getRootNodes(iTaskGraph.clone())?;
    allCalcTasks = convertTaskGraphToTasks(taskGraphT.clone(), iTaskGraphMeta.clone(), Arc::new(convertNodeToTask))?;
    nodeList_refCount = List::map1(rootNodes.clone(), Arc::new(getTaskByIndex), allCalcTasks.clone());
    nodeList = List::map(nodeList_refCount.clone(), Arc::new(fnptr!(Util::tuple21, _)));
    nodeList = List::sort(nodeList.clone(), Arc::new(compareTasksByWeighting))?;
    threadReadyTimes = arrayCreate(iNumberOfThreads.clone(), metamodelica::OrderedFloat(0.0_f64));
    threadTasks = arrayCreate(iNumberOfThreads.clone(), metamodelica::nil());
    tmpSchedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTasks.clone(), outgoingDepTasks: metamodelica::nil(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
    (tmpSchedule, _) = createRandomSchedule1(nodeList.clone(), threadReadyTimes.clone(), iTaskGraph.clone(), taskGraphT.clone(), commCosts.clone(), inComps.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone(), Arc::new(fnptr!(getLocksByPredecessorList, Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, i32, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>)), iNumberOfThreads.clone(), tmpSchedule.clone())?;
    tmpSchedule = addSuccessorLocksToSchedule(iTaskGraph.clone(), Arc::new(addReleaseLocksToSchedule), commCosts.clone(), inComps.clone(), iSimVarMapping.clone(), tmpSchedule.clone())?;
    oSchedule = setScheduleLockIds(tmpSchedule.clone())?;
    Ok(oSchedule)
}

fn createRandomSchedule1(mut iNodeList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut iThreadReadyTimes: metamodelica::Array<metamodelica::Real>, mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut iLockWithPredecessorHandler: Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, i32, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)> + 'static>, mut iNumberOfThreads: i32, mut iSchedule: Arc<HpcOmSimCode::Schedule>) -> Result<(Arc<HpcOmSimCode::Schedule>, metamodelica::Array<metamodelica::Real>)> {
    pub type FuncType = fn(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, i32, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)>;

    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut oThreadReadyTimes: metamodelica::Array<metamodelica::Real>;
    let mut head: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut newTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut newTaskRefCount: i32 = 0;
    let mut rest: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut lastChildFinishTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut lastChild: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut predecessors: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = metamodelica::nil();
    let mut successors: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = metamodelica::nil();
    let mut successorIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut outgoingDepTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut newOutgoingDepTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut threadFinishTimes: metamodelica::Array<metamodelica::Real>;
    let mut firstEq: i32 = 0;
    let mut allThreadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut threadTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut lockTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut threadId: i32 = 0;
    let mut threadFinishTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut tmpThreadReadyTimes: metamodelica::Array<metamodelica::Real>;
    let mut tmpNodeList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut weighting: i32 = 0;
    let mut index: i32 = 0;
    let mut calcTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut eqIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut simEqIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
    (oSchedule, oThreadReadyTimes) = 'mc: {
        let __mc_input = (iNodeList.clone(), iThreadReadyTimes.clone(), iTaskGraph.clone(), iTaskGraphT.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone(), iLockWithPredecessorHandler.clone(), iNumberOfThreads.clone(), iSchedule.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: head @ Deref @ HpcOmSimCode::Task::CALCTASK { eqIdc: eqIdc @ Deref @ metamodelica::List::Cons { head: firstEq, tail: _ }, calcTime, index, weighting, .. }, tail: rest }, _, _, _, _, _, _, _, _, _, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { allCalcTasks, outgoingDepTasks, threadTasks: allThreadTasks, .. }) => {
                    let mut allCalcTasks = (*allCalcTasks).clone();
                    let mut outgoingDepTasks = (*outgoingDepTasks).clone();
                    let mut allThreadTasks = (*allThreadTasks).clone();
                    let mut newOutgoingDepTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = newOutgoingDepTasks.clone();
                    let mut threadId: i32 = threadId.clone();
                    let mut threadFinishTimes: metamodelica::Array<metamodelica::Real>;
                    let mut newTask: Arc<HpcOmSimCode::Task> = newTask.clone();
                    let mut tmpSchedule: Arc<HpcOmSimCode::Schedule>;
                    let mut threadTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = threadTasks.clone();
                    let mut successorIdc: Arc<metamodelica::List<i32>> = successorIdc.clone();
                    let mut newTaskRefCount: i32 = newTaskRefCount.clone();
                    let mut successors: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = successors.clone();
                    let mut threadFinishTime: metamodelica::Real = threadFinishTime.clone();
                    let mut tmpNodeList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = tmpNodeList.clone();
                    let mut simEqIdc: Arc<metamodelica::List<i32>> = simEqIdc.clone();
                    let mut predecessors: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = predecessors.clone();
                    let mut tmpThreadReadyTimes: metamodelica::Array<metamodelica::Real>;
                    let mut lockTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = lockTasks.clone();
                    (predecessors, _) = getSuccessorsByTask(head.clone(), iTaskGraphT.clone(), allCalcTasks.clone())?;
                    (successors, successorIdc) = getSuccessorsByTask(head.clone(), iTaskGraph.clone(), allCalcTasks.clone())?;
                    let false = (predecessors.clone().is_empty()) else { bail!("pattern mismatch") };
                    threadId = System::intRandom(iNumberOfThreads.clone()) + 1;
                    threadFinishTimes = calculateFinishTimes(metamodelica::OrderedFloat(0.0_f64), head.clone(), metamodelica::nil(), iCommCosts.clone(), iThreadReadyTimes.clone())?;
                    threadFinishTime = threadFinishTimes.clone().borrow()[(threadId.clone()-1) as usize].clone();
                    tmpThreadReadyTimes = {let _arr = iThreadReadyTimes.clone(); _arr.borrow_mut()[(threadId.clone()-1) as usize] = threadFinishTime.clone(); _arr};
                    threadTasks = allThreadTasks.clone().borrow()[(threadId.clone()-1) as usize].clone();
                    (lockTasks, newOutgoingDepTasks) = iLockWithPredecessorHandler(head.clone(), predecessors.clone(), threadId.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone())?;
                    outgoingDepTasks = listAppend(outgoingDepTasks.clone(), newOutgoingDepTasks.clone());
                    threadTasks = listAppend(lockTasks.clone(), threadTasks.clone());
                    simEqIdc = List::map(List::map1(eqIdc.clone(), Arc::new(getSimEqSysIdxForComp), iSccSimEqMapping.clone()), Arc::new(List::last));
                    newTask = Arc::new(HpcOmSimCode::Task::CALCTASK { weighting: weighting.clone(), index: index.clone(), calcTime: calcTime.clone(), timeFinished: threadFinishTime.clone(), threadIdx: threadId.clone(), eqIdc: simEqIdc.clone() });
                    threadTasks = cons(newTask.clone(), threadTasks.clone());
                    allThreadTasks = {let _arr = allThreadTasks.clone(); _arr.borrow_mut()[(threadId.clone()-1) as usize] = threadTasks.clone(); _arr};
                    (allCalcTasks, tmpNodeList) = updateRefCounterBySuccessorIdc(allCalcTasks.clone(), successorIdc.clone(), metamodelica::nil())?;
                    tmpNodeList = listAppend(tmpNodeList.clone(), rest.clone());
                    tmpNodeList = List::sort(tmpNodeList.clone(), Arc::new(compareTasksByWeighting))?;
                    (_, newTaskRefCount) = allCalcTasks.clone().borrow()[(index.clone()-1) as usize].clone();
                    let _ = {let _arr = allCalcTasks.clone(); _arr.borrow_mut()[(index.clone()-1) as usize] = (newTask.clone(), newTaskRefCount.clone()); _arr};
                    (tmpSchedule, tmpThreadReadyTimes) = createRandomSchedule1(tmpNodeList.clone(), tmpThreadReadyTimes.clone(), iTaskGraph.clone(), iTaskGraphT.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone(), iLockWithPredecessorHandler.clone(), iNumberOfThreads.clone(), Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: allThreadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() }))?;
                    Ok((tmpSchedule.clone(), tmpThreadReadyTimes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: head @ Deref @ HpcOmSimCode::Task::CALCTASK { eqIdc: eqIdc @ Deref @ metamodelica::List::Cons { head: firstEq, tail: _ }, calcTime, index, weighting, .. }, tail: rest }, _, _, _, _, _, _, _, _, _, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { allCalcTasks, outgoingDepTasks, threadTasks: allThreadTasks, .. }) => {
                    let mut allCalcTasks = (*allCalcTasks).clone();
                    let mut allThreadTasks = (*allThreadTasks).clone();
                    let mut newTaskRefCount: i32 = newTaskRefCount.clone();
                    let mut simEqIdc: Arc<metamodelica::List<i32>> = simEqIdc.clone();
                    let mut successorIdc: Arc<metamodelica::List<i32>> = successorIdc.clone();
                    let mut successors: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = successors.clone();
                    let mut threadId: i32 = threadId.clone();
                    let mut tmpThreadReadyTimes: metamodelica::Array<metamodelica::Real>;
                    let mut threadFinishTimes: metamodelica::Array<metamodelica::Real>;
                    let mut threadTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = threadTasks.clone();
                    let mut newTask: Arc<HpcOmSimCode::Task> = newTask.clone();
                    let mut tmpNodeList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = tmpNodeList.clone();
                    let mut tmpSchedule: Arc<HpcOmSimCode::Schedule>;
                    let mut threadFinishTime: metamodelica::Real = threadFinishTime.clone();
                    (successors, successorIdc) = getSuccessorsByTask(head.clone(), iTaskGraph.clone(), allCalcTasks.clone())?;
                    threadId = System::intRandom(iNumberOfThreads.clone()) + 1;
                    threadFinishTimes = calculateFinishTimes(metamodelica::OrderedFloat(0.0_f64), head.clone(), metamodelica::nil(), iCommCosts.clone(), iThreadReadyTimes.clone())?;
                    threadFinishTime = threadFinishTimes.clone().borrow()[(threadId.clone()-1) as usize].clone();
                    tmpThreadReadyTimes = {let _arr = iThreadReadyTimes.clone(); _arr.borrow_mut()[(threadId.clone()-1) as usize] = threadFinishTime.clone(); _arr};
                    threadTasks = allThreadTasks.clone().borrow()[(threadId.clone()-1) as usize].clone();
                    simEqIdc = List::flatten(List::map1(eqIdc.clone(), Arc::new(getSimEqSysIdxForComp), iSccSimEqMapping.clone()));
                    newTask = Arc::new(HpcOmSimCode::Task::CALCTASK { weighting: weighting.clone(), index: index.clone(), calcTime: calcTime.clone(), timeFinished: threadFinishTime.clone(), threadIdx: threadId.clone(), eqIdc: simEqIdc.clone() });
                    allThreadTasks = {let _arr = allThreadTasks.clone(); _arr.borrow_mut()[(threadId.clone()-1) as usize] = cons(newTask.clone(), threadTasks.clone()); _arr};
                    (allCalcTasks, tmpNodeList) = updateRefCounterBySuccessorIdc(allCalcTasks.clone(), successorIdc.clone(), metamodelica::nil())?;
                    tmpNodeList = listAppend(tmpNodeList.clone(), rest.clone());
                    tmpNodeList = List::sort(tmpNodeList.clone(), Arc::new(compareTasksByWeighting))?;
                    (_, newTaskRefCount) = allCalcTasks.clone().borrow()[(index.clone()-1) as usize].clone();
                    let _ = {let _arr = allCalcTasks.clone(); _arr.borrow_mut()[(index.clone()-1) as usize] = (newTask.clone(), newTaskRefCount.clone()); _arr};
                    (tmpSchedule, tmpThreadReadyTimes) = createRandomSchedule1(tmpNodeList.clone(), tmpThreadReadyTimes.clone(), iTaskGraph.clone(), iTaskGraphT.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone(), iLockWithPredecessorHandler.clone(), iNumberOfThreads.clone(), Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: allThreadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() }))?;
                    Ok((tmpSchedule.clone(), tmpThreadReadyTimes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, _, _, _) => {
                    Ok((iSchedule.clone(), iThreadReadyTimes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("HpcOmScheduler.createRandomSchedule1 failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((oSchedule, oThreadReadyTimes))
}

//------------------------
// List Scheduling reverse
//------------------------
pub fn createListScheduleReverse(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iNumberOfThreads: i32, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<Arc<HpcOmSimCode::Schedule>> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut nodeList_refCount: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = metamodelica::nil();
    let mut nodeList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut leaveNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut threadReadyTimes: metamodelica::Array<metamodelica::Real>;
    let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
    let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>;
    let mut commCostsT: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>;
    let mut tmpSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut outgoingDepTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let HpcOmTaskGraph::TASKGRAPHMETA { inComps: __pa0, commCosts: __pa1, .. } = (iTaskGraphMeta.clone()) else { bail!("pattern mismatch") };
    inComps = __pa0.clone();
    commCosts = __pa1.clone();
    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), (iTaskGraph.clone().borrow().len() as i32))?;
    commCostsT = HpcOmTaskGraph::transposeCommCosts(commCosts.clone());
    leaveNodes = HpcOmTaskGraph::getLeafNodes(iTaskGraph.clone())?;
    allCalcTasks = convertTaskGraphToTasks(iTaskGraph.clone(), iTaskGraphMeta.clone(), Arc::new(convertNodeToTaskReverse))?;
    nodeList_refCount = List::map1(leaveNodes.clone(), Arc::new(getTaskByIndex), allCalcTasks.clone());
    nodeList = List::map(nodeList_refCount.clone(), Arc::new(fnptr!(Util::tuple21, _)));
    nodeList = List::sort(nodeList.clone(), Arc::new(compareTasksByWeighting))?;
    threadReadyTimes = arrayCreate(iNumberOfThreads.clone(), metamodelica::OrderedFloat(0.0_f64));
    threadTasks = arrayCreate(iNumberOfThreads.clone(), metamodelica::nil());
    tmpSchedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTasks.clone(), outgoingDepTasks: metamodelica::nil(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
    (tmpSchedule, _) = createListSchedule1(nodeList.clone(), threadReadyTimes.clone(), taskGraphT.clone(), iTaskGraph.clone(), commCostsT.clone(), inComps.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone(), Arc::new(fnptr!(getLockTasksByPredecessorListReverse, Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, i32, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>)), tmpSchedule.clone())?;
    tmpSchedule = addSuccessorLocksToSchedule(taskGraphT.clone(), Arc::new(addAssignLocksToSchedule), commCosts.clone(), inComps.clone(), iSimVarMapping.clone(), tmpSchedule.clone())?;
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(tmpSchedule.clone()) {
        Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { outgoingDepTasks: __pa2, threadTasks: __pa3, .. } => (__pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outgoingDepTasks = __pa2.clone();
    threadTasks = __pa3.clone();
    threadTasks = Array::map(threadTasks.clone(), Arc::new(listReverse.clone()));
    tmpSchedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
    oSchedule = setScheduleLockIds(tmpSchedule.clone())?;
    Ok(oSchedule)
}

fn addSuccessorLocksToSchedule(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iCreateLockFunction: Arc<dyn ::std::ops::Fn((Arc<HpcOmSimCode::Task>, i32), Arc<HpcOmSimCode::Task>, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> + 'static>, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut iSchedule: Arc<HpcOmSimCode::Schedule>) -> Result<Arc<HpcOmSimCode::Schedule>> {
    pub type FuncType = fn((Arc<HpcOmSimCode::Task>, i32), Arc<HpcOmSimCode::Task>, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;

    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut allThreadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut tmpSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut outgoingDepTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
    oSchedule = (::match_deref::match_deref! { match &((iTaskGraph.clone(), iCreateLockFunction.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone(), iSchedule.clone())) {
        (_, _, _, _, _, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { allCalcTasks, outgoingDepTasks, threadTasks: allThreadTasks, .. }) => {
            let mut allThreadTasks = (*allThreadTasks).clone();
            (allThreadTasks, _) = Array::fold(allThreadTasks.clone(), Arc::new({ let __pe_b1 = iTaskGraph.clone(); let __pe_b2 = allCalcTasks.clone(); let __pe_b3 = iSimVarMapping.clone(); let __pe_b4 = iCommCosts.clone(); let __pe_b5 = iCompTaskMapping.clone(); let __pe_b6: Arc<dyn ::std::ops::Fn((Arc<HpcOmSimCode::Task>, i32), Arc<HpcOmSimCode::Task>, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> + 'static> = iCreateLockFunction.clone(); move |__pe_a0, __pe_a7| addSuccessorLocksToSchedule0(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone(), __pe_a7) }), (allThreadTasks.clone(), 1));
            tmpSchedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: allThreadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
            tmpSchedule.clone()
        },
        _ => {
            println!("{}", (literal!("HpcOmScheduler.addReleaseLocksToSchedule failed\n")).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oSchedule)
}

fn addSuccessorLocksToSchedule0(mut iThreadTaskList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iAllCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iCreateLockFunction: Arc<dyn ::std::ops::Fn((Arc<HpcOmSimCode::Task>, i32), Arc<HpcOmSimCode::Task>, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> + 'static>, mut iThreadTasks: (metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, i32)) -> Result<(metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, i32)> {
    pub type FuncType = fn((Arc<HpcOmSimCode::Task>, i32), Arc<HpcOmSimCode::Task>, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;

    let mut oThreadTasks: (metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, i32);
    let mut threadId: i32 = 0;
    let mut allThreadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut threadTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    (allThreadTasks, threadId) = iThreadTasks.clone();
    threadTasks = List::fold(iThreadTaskList.clone(), Arc::new({ let __pe_b1 = iTaskGraph.clone(); let __pe_b2 = iAllCalcTasks.clone(); let __pe_b3 = iSimVarMapping.clone(); let __pe_b4 = iCommCosts.clone(); let __pe_b5 = iCompTaskMapping.clone(); let __pe_b6 = (threadId.clone(), iCreateLockFunction.clone()); move |__pe_a0, __pe_a7| addSuccessorLocksToSchedule1(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone(), __pe_a7) }), metamodelica::nil());
    allThreadTasks = {let _arr = allThreadTasks.clone(); _arr.borrow_mut()[(threadId.clone()-1) as usize] = threadTasks.clone(); _arr};
    oThreadTasks = (allThreadTasks.clone(), threadId.clone() + 1);
    Ok(oThreadTasks)
}

fn addSuccessorLocksToSchedule1(mut iTask: Arc<HpcOmSimCode::Task>, mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iAllCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iThreadIdLockFunction: (i32, Arc<dyn ::std::ops::Fn((Arc<HpcOmSimCode::Task>, i32), Arc<HpcOmSimCode::Task>, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> + 'static>), mut iThreadTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> {
    pub type FuncType = fn((Arc<HpcOmSimCode::Task>, i32), Arc<HpcOmSimCode::Task>, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;

    let mut oThreadTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut threadIdx: i32 = 0;
    let mut index: i32 = 0;
    let mut listIndex: i32 = 0;
    let mut successors: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = metamodelica::nil();
    let mut tmpThreadTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut releaseTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut iThreadId: i32 = 0;
    let mut iCreateLockFunction: Arc<dyn ::std::ops::Fn((Arc<HpcOmSimCode::Task>, i32), Arc<HpcOmSimCode::Task>, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> + 'static>;
    oThreadTasks = (::match_deref::match_deref! { match &((iTask.clone(), iTaskGraph.clone(), iAllCalcTasks.clone(), iSimVarMapping.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iThreadIdLockFunction.clone(), iThreadTasks.clone())) {
        (Deref @ HpcOmSimCode::Task::CALCTASK { index, threadIdx, .. }, _, _, _, _, _, (iThreadId, iCreateLockFunction), tmpThreadTasks) => {
            let mut tmpThreadTasks = (*tmpThreadTasks).clone();
            (successors, _) = getSuccessorsByTask(iTask.clone(), iTaskGraph.clone(), iAllCalcTasks.clone())?;
            successors = List::removeOnTrue(threadIdx.clone(), Arc::new(compareTaskWithThreadIdx), successors.clone());
            releaseTasks = List::fold4(successors.clone(), iCreateLockFunction.clone(), iTask.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone(), metamodelica::nil());
            tmpThreadTasks = listAppend(releaseTasks.clone(), tmpThreadTasks.clone());
            tmpThreadTasks = cons(iTask.clone(), tmpThreadTasks.clone());
            tmpThreadTasks.clone()
        },
        (_, _, _, _, _, _, _, tmpThreadTasks) => {
            let mut tmpThreadTasks = (*tmpThreadTasks).clone();
            tmpThreadTasks = cons(iTask.clone(), tmpThreadTasks.clone());
            tmpThreadTasks.clone()
        },
        _ => {
            println!("{}", (literal!("HpcOmScheduler.addReleaseLocksToSchedule0 failed\n")).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oThreadTasks)
}

fn addReleaseLocksToSchedule(mut iSuccessorTask: (Arc<HpcOmSimCode::Task>, i32), mut iTask: Arc<HpcOmSimCode::Task>, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut iReleaseTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> {
    let mut oReleaseTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut tmpTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut successorTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut lockString: ArcStr = arcstr::literal!("");
    let mut lockId: i32 = 0;
    let mut successorTaskId: i32 = 0;
    (successorTask, _) = iSuccessorTask.clone();
    tmpTask = createDepTaskAndCommunicationInfo(iTask.clone(), successorTask.clone(), true, iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone())?;
    oReleaseTasks = cons(tmpTask.clone(), iReleaseTasks.clone());
    Ok(oReleaseTasks)
}

fn addAssignLocksToSchedule(mut iSuccessorTask: (Arc<HpcOmSimCode::Task>, i32), mut iTask: Arc<HpcOmSimCode::Task>, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut iReleaseTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> {
    let mut oReleaseTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut tmpTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut successorTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    (successorTask, _) = iSuccessorTask.clone();
    tmpTask = createDepTaskAndCommunicationInfo(successorTask.clone(), iTask.clone(), false, iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone())?;
    oReleaseTasks = cons(tmpTask.clone(), iReleaseTasks.clone());
    Ok(oReleaseTasks)
}

fn getSimEqSysIdxForComp(mut compIdx: i32, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut simEqSysIdcs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    simEqSysIdcs = iSccSimEqMapping.clone().borrow()[(compIdx.clone()-1) as usize].clone();
    Ok(simEqSysIdcs)
}

fn getSimEqSysIdcsForCompLst(mut compIdcs: Arc<metamodelica::List<i32>>, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Arc<metamodelica::List<i32>> {
    let mut simEqSysIdcs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    simEqSysIdcs = List::flatten(List::map1(compIdcs.clone(), Arc::new(fnptr!(Array::getIndexFirst, i32, _)), iSccSimEqMapping.clone()));
    simEqSysIdcs
}

pub fn getSimEqSysIdcsForNodeLst(mut nodeIdcs: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> {
    let mut simEqSysIdcsLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    simEqSysIdcsLst = List::map1(nodeIdcs.clone(), Arc::new(fnptr!(getSimEqSysIdcsForCompLst, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>)), iSccSimEqMapping.clone());
    simEqSysIdcsLst
}

fn getLocksByPredecessorList(mut iTask: Arc<HpcOmSimCode::Task>, mut iPredecessorList: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, mut iThreadIdx: i32, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> (Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) {
    let mut oLockTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut oOutgoingDepTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut tmpTaskList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    oLockTasks = List::fold(iPredecessorList.clone(), Arc::new({ let __pe_b1 = iTask.clone(); let __pe_b2 = iThreadIdx.clone(); let __pe_b3 = iCommCosts.clone(); let __pe_b4 = iCompTaskMapping.clone(); let __pe_b5 = iSimVarMapping.clone(); move |__pe_a0, __pe_a6| getLockTasksByPredecessorList(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_a6) }), metamodelica::nil());
    oOutgoingDepTasks = oLockTasks.clone();
    (oLockTasks, oOutgoingDepTasks)
}

fn getLockTasksByPredecessorList(mut iPredecessorTask: (Arc<HpcOmSimCode::Task>, i32), mut iTask: Arc<HpcOmSimCode::Task>, mut iThreadIdx: i32, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut iLockTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> {
    let mut oLockTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut threadIdx: i32 = 0;
    let mut predIndex: i32 = 0;
    let mut taskIndex: i32 = 0;
    let mut tmpLockTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut tmpTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut predTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    oLockTasks = 'mc: {
        let __mc_input = (iPredecessorTask.clone(), iTask.clone(), iThreadIdx.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone(), iLockTasks.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((predTask @ Deref @ HpcOmSimCode::Task::CALCTASK { index: predIndex, threadIdx, .. }, _), Deref @ HpcOmSimCode::Task::CALCTASK { index: taskIndex, .. }, _, _, _, _, tmpLockTasks) => {
                    let mut tmpLockTasks = (*tmpLockTasks).clone();
                    let mut tmpTask: Arc<HpcOmSimCode::Task> = tmpTask.clone();
                    let true = (intNe(iThreadIdx.clone(), threadIdx.clone())) else { bail!("pattern mismatch") };
                    tmpTask = createDepTaskAndCommunicationInfo(predTask.clone(), iTask.clone(), false, iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone())?;
                    tmpLockTasks = cons(tmpTask.clone(), tmpLockTasks.clone());
                    Ok(tmpLockTasks.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iLockTasks.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oLockTasks)
}

fn getLockTasksByPredecessorListReverse(mut iTask: Arc<HpcOmSimCode::Task>, mut iPredecessorList: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, mut iThreadIdx: i32, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> (Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) {
    let mut oLockTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut oOutgoingDepTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    oLockTasks = List::fold(iPredecessorList.clone(), Arc::new({ let __pe_b1 = iTask.clone(); let __pe_b2 = iThreadIdx.clone(); let __pe_b3 = iCommCosts.clone(); let __pe_b4 = iCompTaskMapping.clone(); let __pe_b5 = iSimVarMapping.clone(); move |__pe_a0, __pe_a6| getLockTasksByPredecessorListReverse0(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_a6) }), metamodelica::nil());
    oOutgoingDepTasks = oLockTasks.clone();
    (oLockTasks, oOutgoingDepTasks)
}

fn getLockTasksByPredecessorListReverse0(mut iPredecessorTask: (Arc<HpcOmSimCode::Task>, i32), mut iTask: Arc<HpcOmSimCode::Task>, mut iThreadIdx: i32, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut iLockTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> {
    let mut oLockTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut index: i32 = 0;
    let mut threadIdx: i32 = 0;
    let mut predTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut tmpTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut tmpLockTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    oLockTasks = 'mc: {
        let __mc_input = (iPredecessorTask.clone(), iTask.clone(), iThreadIdx.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone(), iLockTasks.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((predTask @ Deref @ HpcOmSimCode::Task::CALCTASK { index, threadIdx, .. }, _), _, _, _, _, _, _) => {
                    let mut tmpTask: Arc<HpcOmSimCode::Task> = tmpTask.clone();
                    let mut tmpLockTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = tmpLockTasks.clone();
                    let true = (intNe(iThreadIdx.clone(), threadIdx.clone())) else { bail!("pattern mismatch") };
                    tmpTask = createDepTaskAndCommunicationInfo(iTask.clone(), predTask.clone(), true, iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone())?;
                    tmpLockTasks = cons(tmpTask.clone(), iLockTasks.clone());
                    Ok(tmpLockTasks.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iLockTasks.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oLockTasks)
}

fn getCommunicationObjBetweenMergedTasks(mut parentNode: i32, mut node: i32, mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>) -> Result<HpcOmTaskGraph::Communication> {
    let mut oCommunication: HpcOmTaskGraph::Communication;
    let mut nodeTasks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut parentTasks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut commFold: HpcOmTaskGraph::Communication;
    let mut edgesFromParents: Arc<metamodelica::List<HpcOmTaskGraph::Communication>> = metamodelica::nil();
    nodeTasks = inComps.clone().borrow()[(node.clone()-1) as usize].clone();
    parentTasks = inComps.clone().borrow()[(parentNode.clone()-1) as usize].clone();
    commFold = HpcOmTaskGraph::Communication { numberOfVars: 0, integerVars: metamodelica::nil(), floatVars: metamodelica::nil(), booleanVars: metamodelica::nil(), stringVars: metamodelica::nil(), childNode: node.clone(), requiredTime: metamodelica::OrderedFloat(-1.0_f64) };
    edgesFromParents = List::flatten(List::map1(parentTasks.clone(), Arc::new(fnptr!(Array::getIndexFirst, i32, _)), inCommCosts.clone()));
    oCommunication = List::fold(edgesFromParents.clone(), Arc::new({ let __pe_b1 = nodeTasks.clone(); move |__pe_a0, __pe_a2| getCommunicationObjBetweenMergedTasks1(__pe_a0, __pe_b1.clone(), __pe_a2) }), commFold.clone());
    Ok(oCommunication)
}

fn getCommunicationObjBetweenMergedTasks1(mut parentCommCost: HpcOmTaskGraph::Communication, mut tasks: Arc<metamodelica::List<i32>>, mut iCommunication: HpcOmTaskGraph::Communication) -> Result<HpcOmTaskGraph::Communication> {
    let mut oCommunication: HpcOmTaskGraph::Communication;
    oCommunication = 'mc: {
        let __mc_input = (parentCommCost.clone(), tasks.clone(), iCommunication.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (HpcOmTaskGraph::Communication { numberOfVars: nV1, integerVars: ints1, floatVars: fl1, booleanVars: b1, stringVars: s1, childNode, requiredTime: reqT1 }, _, HpcOmTaskGraph::Communication { numberOfVars: nV2, integerVars: ints2, floatVars: fl2, booleanVars: b2, stringVars: s2, childNode: _, requiredTime: reqT2 }) => {
                    let true = (listMember(childNode.clone(), tasks.clone())) else { bail!("pattern mismatch") };
                    Ok(HpcOmTaskGraph::Communication { numberOfVars: nV1.clone() + nV2.clone(), integerVars: listAppend(ints1.clone(), ints2.clone()), floatVars: listAppend(fl1.clone(), fl2.clone()), booleanVars: listAppend(b1.clone(), b2.clone()), stringVars: listAppend(s1.clone(), s2.clone()), childNode: childNode.clone(), requiredTime: reqT1.clone() + reqT2.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iCommunication.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oCommunication)
}

fn convertCommunicationToCommInfo(mut iCommunication: HpcOmTaskGraph::Communication, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<HpcOmSimCode::CommunicationInfo> {
    let mut oCommInfo: HpcOmSimCode::CommunicationInfo;
    let mut integerVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut floatVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut booleanVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut intSimVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut floatSimVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut boolSimVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    oCommInfo = (match (iCommunication.clone(), iSimVarMapping.clone()) {
        (HpcOmTaskGraph::Communication { booleanVars: mut booleanVars, floatVars: mut floatVars, integerVars: mut integerVars, .. }, _) => {
            intSimVars = List::fold1(integerVars.clone(), Arc::new(convertVarIdxToSimVar), iSimVarMapping.clone(), metamodelica::nil());
            floatSimVars = List::fold1(floatVars.clone(), Arc::new(convertVarIdxToSimVar), iSimVarMapping.clone(), metamodelica::nil());
            boolSimVars = List::fold1(booleanVars.clone(), Arc::new(convertVarIdxToSimVar), iSimVarMapping.clone(), metamodelica::nil());
            HpcOmSimCode::CommunicationInfo { floatVars: floatSimVars.clone(), intVars: intSimVars.clone(), boolVars: boolSimVars.clone() }
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(oCommInfo)
}

fn convertVarIdxToSimVar(mut iVarIdx: i32, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut iSimVar: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Arc<metamodelica::List<SimCodeVar::SimVar>>> {
    let mut oSimVar: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut tmpSimVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    tmpSimVars = iSimVarMapping.clone().borrow()[(iVarIdx.clone()-1) as usize].clone();
    oSimVar = listAppend(iSimVar.clone(), tmpSimVars.clone());
    Ok(oSimVar)
}

fn createDepTask(mut iSourceTask: Arc<HpcOmSimCode::Task>, mut iTargetTask: Arc<HpcOmSimCode::Task>, mut iOutgoing: bool, mut commInfo: HpcOmSimCode::CommunicationInfo) -> Arc<HpcOmSimCode::Task> {
    let mut oAssignTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    oAssignTask = Arc::new(HpcOmSimCode::Task::DEPTASK { sourceTask: iSourceTask.clone(), targetTask: iTargetTask.clone(), outgoing: iOutgoing.clone(), id: 0, communicationInfo: commInfo.clone() });
    oAssignTask
}

fn createDepTaskAndCommunicationInfo(mut iSourceTask: Arc<HpcOmSimCode::Task>, mut iTargetTask: Arc<HpcOmSimCode::Task>, mut iOutgoing: bool, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<Arc<HpcOmSimCode::Task>> {
    let mut oAssignTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut predIndex: i32 = 0;
    let mut taskIndex: i32 = 0;
    let mut tmpTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut commBetweenTasks: HpcOmTaskGraph::Communication;
    let mut commInfo: HpcOmSimCode::CommunicationInfo;
    oAssignTask = 'mc: {
        let __mc_input = (iSourceTask.clone(), iTargetTask.clone(), iOutgoing.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ HpcOmSimCode::Task::CALCTASK { index: predIndex, .. }, Deref @ HpcOmSimCode::Task::CALCTASK { index: taskIndex, .. }, _, _, _, _) => {
                    let mut commBetweenTasks: HpcOmTaskGraph::Communication;
                    let mut commInfo: HpcOmSimCode::CommunicationInfo;
                    let mut tmpTask: Arc<HpcOmSimCode::Task> = tmpTask.clone();
                    commBetweenTasks = getCommunicationObjBetweenMergedTasks(predIndex.clone(), taskIndex.clone(), iCompTaskMapping.clone(), iCommCosts.clone())?;
                    commInfo = convertCommunicationToCommInfo(commBetweenTasks.clone(), iSimVarMapping.clone())?;
                    tmpTask = createDepTask(iSourceTask.clone(), iTargetTask.clone(), iOutgoing.clone(), commInfo.clone());
                    Ok(tmpTask.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("CreateDepTaskAndCommunicationInfo failed!\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oAssignTask)
}

fn createDepTaskByTaskIdc(mut iSourceTaskIdx: i32, mut iTargetTaskIdx: i32, mut iAllCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>, mut iOutgoing: bool, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<Arc<HpcOmSimCode::Task>> {
    let mut oAssignTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut sourceTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut targetTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    sourceTask = Util::tuple21(iAllCalcTasks.clone().borrow()[(iSourceTaskIdx.clone()-1) as usize].clone());
    targetTask = Util::tuple21(iAllCalcTasks.clone().borrow()[(iTargetTaskIdx.clone()-1) as usize].clone());
    oAssignTask = createDepTaskAndCommunicationInfo(sourceTask.clone(), targetTask.clone(), iOutgoing.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone())?;
    Ok(oAssignTask)
}

fn createDepTaskByTaskIdcR(mut iSourceTaskIdx: i32, mut iTargetTaskIdx: i32, mut iAllCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>, mut iOutgoing: bool, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<Arc<HpcOmSimCode::Task>> {
    let mut oAssignTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    oAssignTask = createDepTaskByTaskIdc(iTargetTaskIdx.clone(), iSourceTaskIdx.clone(), iAllCalcTasks.clone(), iOutgoing.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone())?;
    Ok(oAssignTask)
}

fn updateRefCounterBySuccessorIdc(mut iAllCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>, mut iSuccessorIdc: Arc<metamodelica::List<i32>>, mut iRefZeroTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<(metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)> {
    let mut oAllCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
    let mut oRefZeroTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut head: i32 = 0;
    let mut currentRefCount: i32 = 0;
    let mut rest: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpRefZeroTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut currentTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut tmpAllCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
    (oAllCalcTasks, oRefZeroTasks) = 'mc: {
        let __mc_input = (iAllCalcTasks.clone(), iSuccessorIdc.clone(), iRefZeroTasks.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: head, tail: rest }, _) => {
                    let mut tmpAllCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
                    let mut currentTask: Arc<HpcOmSimCode::Task> = currentTask.clone();
                    let mut currentRefCount: i32 = currentRefCount.clone();
                    let mut tmpRefZeroTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = tmpRefZeroTasks.clone();
                    (currentTask, currentRefCount) = iAllCalcTasks.clone().borrow()[(head.clone()-1) as usize].clone();
                    let true = (intEq(currentRefCount.clone(), 1)) else { bail!("pattern mismatch") };
                    tmpAllCalcTasks = {let _arr = iAllCalcTasks.clone(); _arr.borrow_mut()[(head.clone()-1) as usize] = (currentTask.clone(), 0); _arr};
                    tmpRefZeroTasks = cons(currentTask.clone(), iRefZeroTasks.clone());
                    (tmpAllCalcTasks, tmpRefZeroTasks) = updateRefCounterBySuccessorIdc(tmpAllCalcTasks.clone(), rest.clone(), tmpRefZeroTasks.clone())?;
                    Ok((tmpAllCalcTasks.clone(), tmpRefZeroTasks.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: head, tail: rest }, _) => {
                    let mut currentRefCount: i32 = currentRefCount.clone();
                    let mut tmpRefZeroTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = tmpRefZeroTasks.clone();
                    let mut currentTask: Arc<HpcOmSimCode::Task> = currentTask.clone();
                    let mut tmpAllCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
                    (currentTask, currentRefCount) = iAllCalcTasks.clone().borrow()[(head.clone()-1) as usize].clone();
                    tmpAllCalcTasks = {let _arr = iAllCalcTasks.clone(); _arr.borrow_mut()[(head.clone()-1) as usize] = (currentTask.clone(), currentRefCount.clone() - 1); _arr};
                    (tmpAllCalcTasks, tmpRefZeroTasks) = updateRefCounterBySuccessorIdc(tmpAllCalcTasks.clone(), rest.clone(), iRefZeroTasks.clone())?;
                    Ok((tmpAllCalcTasks.clone(), tmpRefZeroTasks.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((iAllCalcTasks.clone(), iRefZeroTasks.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((oAllCalcTasks, oRefZeroTasks))
}

fn getThreadFinishTimesMin(mut iThreadIdx: i32, mut iThreadFinishTimes: metamodelica::Array<metamodelica::Real>, mut iCurrentMinThreadIdx: i32, mut iCurrentMinFinishTime: metamodelica::Real) -> Result<(i32, metamodelica::Real)> {
    let mut minThreadTime_Idx: (i32, metamodelica::Real);
    let mut threadFinishTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    minThreadTime_Idx = 'mc: {
        let __mc_input = (iThreadIdx.clone(), iThreadFinishTimes.clone(), iCurrentMinThreadIdx.clone(), iCurrentMinFinishTime.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let true = (intGt(iThreadIdx.clone(), (iThreadFinishTimes.clone().borrow().len() as i32))) else { bail!("pattern mismatch") };
            Ok((iCurrentMinThreadIdx.clone(), iCurrentMinFinishTime.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut threadFinishTime: metamodelica::Real = threadFinishTime.clone();
            threadFinishTime = iThreadFinishTimes.clone().borrow()[(iThreadIdx.clone()-1) as usize].clone();
            let true = (realLt(threadFinishTime.clone(), iCurrentMinFinishTime.clone()) || intEq(iCurrentMinThreadIdx.clone(), -1)) else { bail!("pattern mismatch") };
            Ok(getThreadFinishTimesMin(iThreadIdx.clone() + 1, iThreadFinishTimes.clone(), iThreadIdx.clone(), threadFinishTime.clone())?)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(getThreadFinishTimesMin(iThreadIdx.clone() + 1, iThreadFinishTimes.clone(), iCurrentMinThreadIdx.clone(), iCurrentMinFinishTime.clone())?)
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(minThreadTime_Idx)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getTaskWithHighestFinishTime(mut iTasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, mut iCurrentTask: Option<Arc<HpcOmSimCode::Task>>) -> Result<Arc<HpcOmSimCode::Task>> {
    let mut oTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut head: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut tmpTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut tail: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = metamodelica::nil();
    let mut timeFinishedHead: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut timeFinishedCurrent: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    oTask = 'mc: {
        let __mc_input = (iTasks.clone(), iCurrentTask.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (head, _), tail: tail }, None) => {
                    Ok(getTaskWithHighestFinishTime(tail.clone(), Some(head.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (head @ Deref @ HpcOmSimCode::Task::CALCTASK { timeFinished: timeFinishedHead, .. }, _), tail: tail }, Some(Deref @ HpcOmSimCode::Task::CALCTASK { timeFinished: timeFinishedCurrent, .. })) => {
                    let true = (realGt(timeFinishedHead.clone(), timeFinishedCurrent.clone())) else { bail!("pattern mismatch") };
                    Ok(getTaskWithHighestFinishTime(tail.clone(), Some(head.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (head, _), tail: tail }, Some(_)) => {
                    Ok(getTaskWithHighestFinishTime(tail.clone(), iCurrentTask.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Some(tmpTask)) => {
                    Ok(tmpTask.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("HpcOmScheduler.getTaskWithHighestFinishTime failed!\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oTask)
}

fn convertTaskGraphToTasks(mut iTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iConverterFunc: Arc<dyn ::std::ops::Fn(i32, HpcOmTaskGraph::TaskGraphMeta) -> Result<Arc<HpcOmSimCode::Task>> + 'static>) -> Result<metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>> {
    pub type FuncType = fn(i32, HpcOmTaskGraph::TaskGraphMeta) -> Result<Arc<HpcOmSimCode::Task>>;

    let mut oTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
    let mut tmpTaskArray: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    tmpTaskArray = arrayCreate((iTaskGraphT.clone().borrow().len() as i32), (Arc::new(crate::HpcOmSimCode::Task::TASKEMPTY), 0));
    oTasks = convertTaskGraphToTasks1(iTaskGraphMeta.clone(), iTaskGraphT.clone(), 1, iConverterFunc.clone(), tmpTaskArray.clone())?;
    Ok(oTasks)
}

fn convertTaskGraphToTasks1(mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iIndex: i32, mut iConverterFunc: Arc<dyn ::std::ops::Fn(i32, HpcOmTaskGraph::TaskGraphMeta) -> Result<Arc<HpcOmSimCode::Task>> + 'static>, mut iTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>) -> Result<metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>> {
    pub type FuncType = fn(i32, HpcOmTaskGraph::TaskGraphMeta) -> Result<Arc<HpcOmSimCode::Task>>;

    let mut oTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut nodeMarks: metamodelica::Array<i32>;
    let mut tmpTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
    let mut refCount: i32 = 0;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut newTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    oTasks = 'mc: {
        let __mc_input = (iTaskGraphMeta.clone(), iTaskGraphT.clone(), iIndex.clone(), iConverterFunc.clone(), iTasks.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut refCount: i32 = refCount.clone();
            let mut newTask: Arc<HpcOmSimCode::Task> = newTask.clone();
            let mut tmpTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
            let true = (intLe(iIndex.clone(), (iTaskGraphT.clone().borrow().len() as i32))) else { bail!("pattern mismatch") };
            refCount = (iTaskGraphT.clone().borrow()[(iIndex.clone()-1) as usize].clone().len() as i32);
            newTask = iConverterFunc(iIndex.clone(), iTaskGraphMeta.clone())?;
            tmpTasks = {let _arr = iTasks.clone(); _arr.borrow_mut()[(iIndex.clone()-1) as usize] = (newTask.clone(), refCount.clone()); _arr};
            tmpTasks = convertTaskGraphToTasks1(iTaskGraphMeta.clone(), iTaskGraphT.clone(), iIndex.clone() + 1, iConverterFunc.clone(), tmpTasks.clone())?;
            Ok(tmpTasks.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(iTasks.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oTasks)
}

fn convertNodeToTask(mut iNodeIdx: i32, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta) -> Result<Arc<HpcOmSimCode::Task>> {
    let mut oTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut nodeMark: i32 = 0;
    let mut primalComp: i32 = 0;
    let mut components: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut exeCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut nodeMarks: metamodelica::Array<i32>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    oTask = (match (iNodeIdx.clone(), iTaskGraphMeta.clone()) {
        (_, HpcOmTaskGraph::TaskGraphMeta { exeCosts: mut exeCosts, nodeMark: mut nodeMarks, inComps: mut inComps, .. }) => {
            components = inComps.clone().borrow()[(iNodeIdx.clone()-1) as usize].clone();
            primalComp = (components.clone()).get(1)?;
            nodeMark = nodeMarks.clone().borrow()[(primalComp.clone()-1) as usize].clone();
            (_, exeCost) = HpcOmTaskGraph::getExeCost(iNodeIdx.clone(), iTaskGraphMeta.clone())?;
            Arc::new(HpcOmSimCode::Task::CALCTASK { weighting: nodeMark.clone(), index: iNodeIdx.clone(), calcTime: exeCost.clone(), timeFinished: metamodelica::OrderedFloat(-1.0_f64), threadIdx: -1, eqIdc: components.clone() })
        },
        _ => {
            println!("{}", (literal!("HpcOmScheduler.convertNodeToTask failed!\n")).clone());
            bail!("fail")
        },
    });
    Ok(oTask)
}

fn convertNodeToTaskReverse(mut iNodeIdx: i32, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta) -> Result<Arc<HpcOmSimCode::Task>> {
    let mut oTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut nodeMark: i32 = 0;
    let mut primalComp: i32 = 0;
    let mut components: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut exeCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut nodeMarks: metamodelica::Array<i32>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    oTask = (match (iNodeIdx.clone(), iTaskGraphMeta.clone()) {
        (_, HpcOmTaskGraph::TaskGraphMeta { exeCosts: mut exeCosts, nodeMark: mut nodeMarks, inComps: mut inComps, .. }) => {
            components = inComps.clone().borrow()[(iNodeIdx.clone()-1) as usize].clone();
            primalComp = (components.clone()).get(1)?;
            nodeMark = nodeMarks.clone().borrow()[(primalComp.clone()-1) as usize].clone();
            (_, exeCost) = exeCosts.clone().borrow()[(iNodeIdx.clone()-1) as usize].clone();
            nodeMark = nodeMark.clone() * -1;
            Arc::new(HpcOmSimCode::Task::CALCTASK { weighting: nodeMark.clone(), index: iNodeIdx.clone(), calcTime: exeCost.clone(), timeFinished: metamodelica::OrderedFloat(-1.0_f64), threadIdx: -1, eqIdc: components.clone() })
        },
        _ => {
            println!("{}", (literal!("HpcOmScheduler.convertNodeToTask failed!\n")).clone());
            bail!("fail")
        },
    });
    Ok(oTask)
}

fn calculateFinishTimes(mut iPredecessorTaskLastFinished: metamodelica::Real, mut iTask: Arc<HpcOmSimCode::Task>, mut iPredecessorTasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iThreadReadyTimes: metamodelica::Array<metamodelica::Real>) -> Result<metamodelica::Array<metamodelica::Real>> {
    let mut oFinishTimes: metamodelica::Array<metamodelica::Real>;
    let mut tmpFinishTimes: metamodelica::Array<metamodelica::Real>;
    tmpFinishTimes = arrayCreate((iThreadReadyTimes.clone().borrow().len() as i32), metamodelica::OrderedFloat(0.0_f64));
    tmpFinishTimes = calculateFinishTimes1(iPredecessorTaskLastFinished.clone(), iTask.clone(), iPredecessorTasks.clone(), iCommCosts.clone(), iThreadReadyTimes.clone(), 1, tmpFinishTimes.clone())?;
    oFinishTimes = tmpFinishTimes.clone();
    Ok(oFinishTimes)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn calculateFinishTimes1(mut iPredecessorTaskLastFinished: metamodelica::Real, mut iTask: Arc<HpcOmSimCode::Task>, mut iPredecessorTasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iThreadReadyTimes: metamodelica::Array<metamodelica::Real>, mut iThreadIdx: i32, mut iFinishTimes: metamodelica::Array<metamodelica::Real>) -> Result<metamodelica::Array<metamodelica::Real>> {
    let mut oFinishTimes: metamodelica::Array<metamodelica::Real>;
    let mut thFinishTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut thReadyTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut tmpFinishTimes: metamodelica::Array<metamodelica::Real>;
    oFinishTimes = 'mc: {
        let __mc_input = (iPredecessorTaskLastFinished.clone(), iTask.clone(), iPredecessorTasks.clone(), iCommCosts.clone(), iThreadReadyTimes.clone(), iThreadIdx.clone(), iFinishTimes.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _) => {
                    let mut thFinishTime: metamodelica::Real = thFinishTime.clone();
                    let mut tmpFinishTimes: metamodelica::Array<metamodelica::Real>;
                    let mut thReadyTime: metamodelica::Real = thReadyTime.clone();
                    let true = (intLe(iThreadIdx.clone(), (iThreadReadyTimes.clone().borrow().len() as i32))) else { bail!("pattern mismatch") };
                    thReadyTime = iThreadReadyTimes.clone().borrow()[(iThreadIdx.clone()-1) as usize].clone();
                    thFinishTime = calculateFinishTimeByThreadId(thReadyTime.clone(), iPredecessorTaskLastFinished.clone(), iThreadIdx.clone(), iTask.clone(), iPredecessorTasks.clone(), iCommCosts.clone())?;
                    tmpFinishTimes = {let _arr = iFinishTimes.clone(); _arr.borrow_mut()[(iThreadIdx.clone()-1) as usize] = thFinishTime.clone(); _arr};
                    Ok(calculateFinishTimes1(iPredecessorTaskLastFinished.clone(), iTask.clone(), iPredecessorTasks.clone(), iCommCosts.clone(), iThreadReadyTimes.clone(), iThreadIdx.clone() + 1, tmpFinishTimes.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iFinishTimes.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oFinishTimes)
}

fn calculateFinishTimeByThreadId(mut iThreadReadyTime: metamodelica::Real, mut iPredecessorTaskLastFinished: metamodelica::Real, mut iThreadId: i32, mut iTask: Arc<HpcOmSimCode::Task>, mut iPredecessorTasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>) -> Result<metamodelica::Real> {
    let mut oFinishTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut predecessorTasksOtherTh: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = metamodelica::nil();
    let mut commCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut calcTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut startTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    oFinishTime = (::match_deref::match_deref! { match &((iThreadReadyTime.clone(), iPredecessorTaskLastFinished.clone(), iThreadId.clone(), iTask.clone(), iPredecessorTasks.clone(), iCommCosts.clone())) {
        (_, _, _, Deref @ HpcOmSimCode::Task::CALCTASK { calcTime, .. }, _, _) => {
            predecessorTasksOtherTh = List::removeOnTrue(iThreadId.clone(), Arc::new(compareTaskWithThreadIdx), iPredecessorTasks.clone());
            startTime = realMax(iThreadReadyTime.clone(), iPredecessorTaskLastFinished.clone());
            commCost = getMaxCommCostsByTaskList(iTask.clone(), predecessorTasksOtherTh.clone(), iCommCosts.clone());
            ((startTime.clone()) + (commCost.clone())) + (calcTime.clone())
        },
        _ => {
            println!("{}", (literal!("HpcOmScheduler.calculateFinishTimeByThreadId can only handle CALCTASKs\n")).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oFinishTime)
}

fn getMaxCommCostsByTaskList(mut iParentTask: Arc<HpcOmSimCode::Task>, mut iTaskList: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>) -> metamodelica::Real {
    let mut oCommCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    oCommCost = List::fold2(iTaskList.clone(), Arc::new(getMaxCommCostsByTaskList1), iParentTask.clone(), iCommCosts.clone(), metamodelica::OrderedFloat(0.0_f64));
    oCommCost
}

fn getMaxCommCostsByTaskList1(mut iTask: (Arc<HpcOmSimCode::Task>, i32), mut iParentTask: Arc<HpcOmSimCode::Task>, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCurrentMax: metamodelica::Real) -> Result<metamodelica::Real> {
    let mut oCommCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut taskIdx: i32 = 0;
    let mut reqCycles: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut eqIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut parentEqIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut childCommCosts: Arc<metamodelica::List<HpcOmTaskGraph::Communication>> = metamodelica::nil();
    oCommCost = 'mc: {
        let __mc_input = (iTask.clone(), iParentTask.clone(), iCommCosts.clone(), iCurrentMax.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ HpcOmSimCode::Task::CALCTASK { eqIdc, index: taskIdx, .. }, _), Deref @ HpcOmSimCode::Task::CALCTASK { eqIdc: parentEqIdc, .. }, _, _) => {
                    let mut childCommCosts: Arc<metamodelica::List<HpcOmTaskGraph::Communication>> = childCommCosts.clone();
                    let mut reqCycles: metamodelica::Real = reqCycles.clone();
                    childCommCosts = iCommCosts.clone().borrow()[(listHead(eqIdc.clone())?-1) as usize].clone();
                    let HpcOmTaskGraph::COMMUNICATION { requiredTime: __pa0, .. } = (getMaxCommCostsByTaskList2(childCommCosts.clone(), listHead(parentEqIdc.clone())?)?) else { bail!("pattern mismatch") };
                    reqCycles = __pa0.clone();
                    let true = (realGt(reqCycles.clone(), iCurrentMax.clone())) else { bail!("pattern mismatch") };
                    Ok(reqCycles.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iCurrentMax.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oCommCost)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getMaxCommCostsByTaskList2(mut iCommCosts: Arc<metamodelica::List<HpcOmTaskGraph::Communication>>, mut iIdx: i32) -> Result<HpcOmTaskGraph::Communication> {
    let mut oComm: HpcOmTaskGraph::Communication;
    let mut childIdxHead: i32 = 0;
    let mut tail: Arc<metamodelica::List<HpcOmTaskGraph::Communication>> = metamodelica::nil();
    let mut head: HpcOmTaskGraph::Communication;
    oComm = 'mc: {
        let __mc_input = (iCommCosts.clone(), iIdx.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: head @ HpcOmTaskGraph::Communication { childNode: childIdxHead, .. }, tail: tail }, _) => {
                    let true = (intEq(childIdxHead.clone(), iIdx.clone())) else { bail!("pattern mismatch") };
                    Ok(head.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: tail }, _) => {
                    Ok(getMaxCommCostsByTaskList2(tail.clone(), iIdx.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("HpcOmScheduler.getMaxCommCostsByTaskList2 failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oComm)
}

fn getTaskByIndex(mut iTaskIdx: i32, mut iAllCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>) -> Result<(Arc<HpcOmSimCode::Task>, i32)> {
    let mut oTask: (Arc<HpcOmSimCode::Task>, i32);
    oTask = iAllCalcTasks.clone().borrow()[(iTaskIdx.clone()-1) as usize].clone();
    Ok(oTask)
}

pub fn getSuccessorsByTask(mut iTask: Arc<HpcOmSimCode::Task>, mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iAllCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>) -> Result<(Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, Arc<metamodelica::List<i32>>)> {
    let mut oTasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = metamodelica::nil();
    let mut oTaskIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut taskIdx: i32 = 0;
    let mut successors: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpTasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = metamodelica::nil();
    (oTasks, oTaskIdc) = 'mc: {
        let __mc_input = (iTask.clone(), iTaskGraph.clone(), iAllCalcTasks.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ HpcOmSimCode::Task::CALCTASK { index: taskIdx, .. }, _, _) => {
                    let mut successors: Arc<metamodelica::List<i32>> = successors.clone();
                    let mut tmpTasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = tmpTasks.clone();
                    successors = iTaskGraph.clone().borrow()[(taskIdx.clone()-1) as usize].clone();
                    tmpTasks = List::map1(successors.clone(), Arc::new(getTaskByIndex), iAllCalcTasks.clone());
                    Ok((tmpTasks.clone(), successors.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("HpcOmScheduler.getSuccessorsByTask can only handle CALCTASKs.")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((oTasks, oTaskIdc))
}

fn compareTasksByWeighting(mut iTask1: Arc<HpcOmSimCode::Task>, mut iTask2: Arc<HpcOmSimCode::Task>) -> Result<bool> {
    let mut oResult: bool = false;
    let mut weightingTask1: i32 = 0;
    let mut weightingTask2: i32 = 0;
    oResult = (::match_deref::match_deref! { match &((iTask1.clone(), iTask2.clone())) {
        (Deref @ HpcOmSimCode::Task::CALCTASK { weighting: weightingTask1, .. }, Deref @ HpcOmSimCode::Task::CALCTASK { weighting: weightingTask2, .. }) => intGt(weightingTask1.clone(), weightingTask2.clone()),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("HpcOmScheduler.compareTasksByWeighting can only compare CALCTASKs! Task 1 has type ")); __mm_s.push_str(&*getTaskTypeString(iTask1.clone())); __mm_s.push_str(&*literal!(" and task 2 has type ")); __mm_s.push_str(&*getTaskTypeString(iTask2.clone())); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oResult)
}

fn compareTasksByEqIdc(mut iTask1: Arc<HpcOmSimCode::Task>, mut iTask2: Arc<HpcOmSimCode::Task>) -> Result<bool> {
    let mut oResult: bool = false;
    let mut eqIdcTask1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqIdcTask2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    oResult = (::match_deref::match_deref! { match &((iTask1.clone(), iTask2.clone())) {
        (Deref @ HpcOmSimCode::Task::CALCTASK { eqIdc: eqIdcTask1, .. }, Deref @ HpcOmSimCode::Task::CALCTASK { eqIdc: eqIdcTask2, .. }) => intGt(List::last(eqIdcTask1.clone())?, List::last(eqIdcTask2.clone())?),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("HpcOmScheduler.compareTasksByEqIdc can only compare CALCTASKs with at least one equation index! Task 1 has type ")); __mm_s.push_str(&*getTaskTypeString(iTask1.clone())); __mm_s.push_str(&*literal!(" and task 2 has type ")); __mm_s.push_str(&*getTaskTypeString(iTask2.clone())); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oResult)
}

fn compareTaskWithThreadIdx(mut iThreadIdx: i32, mut iTask1: (Arc<HpcOmSimCode::Task>, i32)) -> Result<bool> {
    let mut oMatch: bool = false;
    let mut threadIdx: i32 = 0;
    oMatch = (::match_deref::match_deref! { match &((iThreadIdx.clone(), iTask1.clone())) {
        (_, (Deref @ HpcOmSimCode::Task::CALCTASK { threadIdx, .. }, _)) => intEq(threadIdx.clone(), iThreadIdx.clone()),
        _ => {
            println!("{}", (literal!("HpcOmScheduler.compareTaskWithThreadIdx can only compare CALCTASKs!\n")).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oMatch)
}

fn dumpThreadSchedule(mut iTaskList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut iThreadIdx: i32) -> (ArcStr, i32) {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut oThreadIdx: i32 = 0;
    r#str = (literal!("--------------\n")).clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("Thread ")); __mm_s.push_str(&*intString(iThreadIdx.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("--------------\n")); ArcStr::from(__mm_s) }).clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*dumpTaskList(iTaskList.clone())); ArcStr::from(__mm_s) }).clone();
    oThreadIdx = iThreadIdx.clone() + 1;
    (r#str, oThreadIdx)
}

fn dumpTaskDepSchedule(mut iTaskInfo: (Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut s: ArcStr = arcstr::literal!("");
    let mut iTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut iDependencies: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (iTask, iDependencies) = iTaskInfo.clone();
    s = (literal!("Task: \n")).clone();
    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*dumpTask(iTask.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("-> Parents: ")); __mm_s.push_str(&*stringDelimitList(List::map(iDependencies.clone(), Arc::new(fnptr!(intString, i32))), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("---------------------\n")); ArcStr::from(__mm_s) }).clone();
    Ok(r#str)
}

fn printTaskList(mut iTaskList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> () {
    println!("{}", (dumpTaskList(iTaskList.clone())).clone());
    ()
}

fn dumpTaskList(mut iTaskList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = stringDelimitList(List::map(iTaskList.clone(), Arc::new(dumpTask)), (literal!("")).clone());
    r#str
}

fn dumpTask(mut iTask: Arc<HpcOmSimCode::Task>) -> Result<ArcStr> {
    let mut oString: ArcStr = arcstr::literal!("");
    let mut weighting: i32 = 0;
    let mut index: i32 = 0;
    let mut threadIdx: i32 = 0;
    let mut compIdx: i32 = 0;
    let mut numThreads: i32 = 0;
    let mut sourceIndex: i32 = 0;
    let mut targetIndex: i32 = 0;
    let mut eqIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut nodeIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut calcTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut timeFinished: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut lockId: ArcStr = arcstr::literal!("");
    let mut s: ArcStr = arcstr::literal!("");
    let mut taskSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut outgoing: bool = false;
    let mut threadIdx: i32 = 0;
    oString = ((::match_deref::match_deref! { match &(iTask.clone()) {
        Deref @ HpcOmSimCode::Task::SCHEDULED_TASK { taskSchedule, numThreads, compIdx } => {
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Scheduled Task (comp: ")); __mm_s.push_str(&*intString(compIdx.clone())); __mm_s.push_str(&*literal!(", numThreads: ")); __mm_s.push_str(&*intString(numThreads.clone())); __mm_s.push_str(&*literal!("):\n------------------------------------------------------\n")); ArcStr::from(__mm_s) }).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("\t")); __mm_s.push_str(&*System::stringReplace((dumpSchedule(taskSchedule.clone())?).clone(), (literal!("\n")).clone(), (literal!("\n\t")).clone())?); ArcStr::from(__mm_s) }).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("------------------------------------------------------\n")); ArcStr::from(__mm_s) }).clone();
            s.clone()
        },
        Deref @ HpcOmSimCode::Task::CALCTASK { eqIdc, index, timeFinished, weighting, .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Calculation task with index ")); __mm_s.push_str(&*intString(index.clone())); __mm_s.push_str(&*literal!(" including the equations: ")); __mm_s.push_str(&*stringDelimitList(List::map(eqIdc.clone(), Arc::new(fnptr!(intString, i32))), (literal!(", ")).clone())); __mm_s.push_str(&*literal!(" is finished at  ")); __mm_s.push_str(&*realString(timeFinished.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) },
        Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { threadIdx: None, nodeIdc, eqIdc } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Calculation task (")); __mm_s.push_str(&*stringDelimitList(List::map(nodeIdc.clone(), Arc::new(fnptr!(intString, i32))), (literal!(", ")).clone())); __mm_s.push_str(&*literal!(") including the equations: ")); __mm_s.push_str(&*stringDelimitList(List::map(eqIdc.clone(), Arc::new(fnptr!(intString, i32))), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) },
        Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { threadIdx: Some(threadIdx), nodeIdc, eqIdc } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Calculation task (")); __mm_s.push_str(&*stringDelimitList(List::map(nodeIdc.clone(), Arc::new(fnptr!(intString, i32))), (literal!(", ")).clone())); __mm_s.push_str(&*literal!(") including the equations: ")); __mm_s.push_str(&*stringDelimitList(List::map(eqIdc.clone(), Arc::new(fnptr!(intString, i32))), (literal!(", ")).clone())); __mm_s.push_str(&*literal!(" by thread ")); __mm_s.push_str(&*intString(threadIdx.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) },
        Deref @ HpcOmSimCode::Task::DEPTASK { outgoing, targetTask: Deref @ HpcOmSimCode::Task::CALCTASK { index: targetIndex, .. }, sourceTask: Deref @ HpcOmSimCode::Task::CALCTASK { index: sourceIndex, .. }, .. } => {
            s = (literal!("Dependency task ")).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*if (outgoing.clone()) {literal!("(outgoing)")} else {literal!("(incoming)")}); ArcStr::from(__mm_s) }).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!(" between ")); __mm_s.push_str(&*intString(sourceIndex.clone())); __mm_s.push_str(&*literal!(" and ")); __mm_s.push_str(&*intString(targetIndex.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            s.clone()
        },
        Deref @ HpcOmSimCode::Task::TASKEMPTY => literal!("empty task\n"),
        _ => {
            println!("{}", (literal!("HpcOmScheduler.dumpTask failed\n")).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(oString)
}

pub fn printTask(mut iTask: Arc<HpcOmSimCode::Task>) -> Result<()> {
    println!("{}", (dumpTask(iTask.clone())?).clone());
    Ok(())
}

pub fn convertScheduleStrucToInfo(mut iSchedule: Arc<HpcOmSimCode::Schedule>, mut iTaskCount: i32) -> Result<metamodelica::Array<(i32, i32, metamodelica::Real)>> {
    let mut oScheduleInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>;
    let mut tmpScheduleInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>;
    let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut tasksOfLevels: Arc<metamodelica::List<HpcOmSimCode::TaskList>> = metamodelica::nil();
    let mut allTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    oScheduleInfo = (::match_deref::match_deref! { match &((iSchedule.clone(), iTaskCount.clone())) {
        (Deref @ HpcOmSimCode::Schedule::EMPTYSCHEDULE { tasks: HpcOmSimCode::TaskList::SERIALTASKLIST { tasks: allTasks, .. } }, _) => {
            tmpScheduleInfo = arrayCreate(iTaskCount.clone(), (-1, -1, metamodelica::OrderedFloat(-1.0_f64)));
            threadTasks = arrayCreate(1, allTasks.clone());
            tmpScheduleInfo = Array::fold(threadTasks.clone(), Arc::new(fnptr!(convertScheduleStrucToInfo0, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, metamodelica::Array<(i32, i32, metamodelica::Real)>)), tmpScheduleInfo.clone());
            tmpScheduleInfo.clone()
        },
        (Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks, .. }, _) => {
            tmpScheduleInfo = arrayCreate(iTaskCount.clone(), (-1, -1, metamodelica::OrderedFloat(-1.0_f64)));
            tmpScheduleInfo = Array::fold(threadTasks.clone(), Arc::new(fnptr!(convertScheduleStrucToInfo0, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, metamodelica::Array<(i32, i32, metamodelica::Real)>)), tmpScheduleInfo.clone());
            tmpScheduleInfo.clone()
        },
        (Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { tasksOfLevels, .. }, _) => {
            tmpScheduleInfo = arrayCreate(iTaskCount.clone(), (-1, -1, metamodelica::OrderedFloat(-1.0_f64)));
            tmpScheduleInfo = convertScheduleStrucToInfoLevel(tasksOfLevels.clone(), 1, tmpScheduleInfo.clone())?;
            tmpScheduleInfo.clone()
        },
        (Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: _ }, _) => {
            tmpScheduleInfo = arrayCreate(iTaskCount.clone(), (-1, -1, metamodelica::OrderedFloat(-1.0_f64)));
            tmpScheduleInfo.clone()
        },
        _ => {
            println!("{}", (literal!("HpcOmScheduler.convertScheduleStrucToInfo unknown Schedule-Type.\n")).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oScheduleInfo)
}

fn convertScheduleStrucToInfo0(mut iTaskList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut iScheduleInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>) -> metamodelica::Array<(i32, i32, metamodelica::Real)> {
    let mut oScheduleInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>;
    (oScheduleInfo, _) = List::fold(iTaskList.clone(), Arc::new(convertScheduleStrucToInfo1), (iScheduleInfo.clone(), 1));
    oScheduleInfo
}

fn convertScheduleStrucToInfo1(mut iTask: Arc<HpcOmSimCode::Task>, mut iScheduleInfo: (metamodelica::Array<(i32, i32, metamodelica::Real)>, i32)) -> Result<(metamodelica::Array<(i32, i32, metamodelica::Real)>, i32)> {
    let mut oScheduleInfo: (metamodelica::Array<(i32, i32, metamodelica::Real)>, i32);
    let mut taskIdx: i32 = 0;
    let mut taskNumber: i32 = 0;
    let mut threadIdx: i32 = 0;
    let mut tmpScheduleInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>;
    let mut timeFinished: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    oScheduleInfo = (::match_deref::match_deref! { match &((iTask.clone(), iScheduleInfo.clone())) {
        (Deref @ HpcOmSimCode::Task::CALCTASK { timeFinished, threadIdx, index: taskIdx, .. }, (tmpScheduleInfo, taskNumber)) => {
            let mut tmpScheduleInfo = (*tmpScheduleInfo).clone();
            tmpScheduleInfo = {let _arr = tmpScheduleInfo.clone(); _arr.borrow_mut()[(taskIdx.clone()-1) as usize] = (threadIdx.clone(), taskNumber.clone(), timeFinished.clone()); _arr};
            (tmpScheduleInfo.clone(), taskNumber.clone() + 1)
        },
        (Deref @ HpcOmSimCode::Task::DEPTASK { .. }, _) => iScheduleInfo.clone(),
        _ => {
            println!("{}", (literal!("HpcOmScheduler.convertScheduleStrucToInfo1 failed. Unknown Task-Type.\n")).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oScheduleInfo)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn convertScheduleStrucToInfoLevel(mut taskLst: Arc<metamodelica::List<HpcOmSimCode::TaskList>>, mut sectionsNumber: i32, mut iScheduleInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>) -> Result<metamodelica::Array<(i32, i32, metamodelica::Real)>> {
    let mut oScheduleInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>;
    oScheduleInfo = 'mc: {
        let __mc_input = (taskLst.clone(), sectionsNumber.clone(), iScheduleInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(iScheduleInfo.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: HpcOmSimCode::TaskList::PARALLELTASKLIST { tasks }, tail: rest }, _, _) => {
                    let mut scheduleInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>;
                    scheduleInfo = convertScheduleStrucToInfoLevel1(tasks.clone(), sectionsNumber.clone(), 1, iScheduleInfo.clone())?;
                    Ok(convertScheduleStrucToInfoLevel(rest.clone(), sectionsNumber.clone() + 1, scheduleInfo.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: HpcOmSimCode::TaskList::SERIALTASKLIST { tasks, .. }, tail: rest }, _, _) => {
                    let mut scheduleInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>;
                    scheduleInfo = convertScheduleStrucToInfoLevel1(tasks.clone(), sectionsNumber.clone(), 1, iScheduleInfo.clone())?;
                    Ok(convertScheduleStrucToInfoLevel(rest.clone(), sectionsNumber.clone() + 1, scheduleInfo.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("convertScheduleStrucToInfoLevel failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oScheduleInfo)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn convertScheduleStrucToInfoLevel1(mut tasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut sectionsNumber: i32, mut sectionIdx: i32, mut iScheduleInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>) -> Result<metamodelica::Array<(i32, i32, metamodelica::Real)>> {
    let mut oScheduleInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>;
    oScheduleInfo = (::match_deref::match_deref! { match &((tasks.clone(), sectionsNumber.clone(), sectionIdx.clone(), iScheduleInfo.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _) => {
            iScheduleInfo.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { threadIdx: threadIdxOpt, nodeIdc, .. }, tail: rest }, _, _, _) => {
            let mut numNodes: i32 = 0;
            let mut threadIdx: i32 = 0;
            let mut tuplLst: Arc<metamodelica::List<(i32, i32, metamodelica::Real)>> = metamodelica::nil();
            numNodes = (nodeIdc.clone().len() as i32);
            threadIdx = Util::getOptionOrDefault(threadIdxOpt.clone(), -1);
            tuplLst = List::threadMap1(List::fill(threadIdx.clone(), numNodes.clone()), List::fill(-1, numNodes.clone()), Arc::new(fnptr!(Util::make3Tuple, _, _, _)), metamodelica::OrderedFloat(0.0_f64));
            List::threadMap1_0(nodeIdc.clone(), tuplLst.clone(), Arc::new(Array::updateIndexFirst), iScheduleInfo.clone())?;
            convertScheduleStrucToInfoLevel1(rest.clone(), sectionsNumber.clone(), sectionIdx.clone() + 1, iScheduleInfo.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(oScheduleInfo)
}

//-----------------
// Balanced Level Scheduling
//-----------------
pub fn createBalancedLevelScheduling(mut iGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iMeta: HpcOmTaskGraph::TaskGraphMeta, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<(Arc<HpcOmSimCode::Schedule>, HpcOmTaskGraph::TaskGraphMeta)> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut oMeta: HpcOmTaskGraph::TaskGraphMeta;
    let mut cpCostsWoC: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut targetCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut levelAss: metamodelica::Array<i32>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut startNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut critPathNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut critPathCosts: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut level: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut critPathSections: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut allSections: Arc<metamodelica::List<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>>> = metamodelica::nil();
    let mut levelComps: Arc<metamodelica::List<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>>> = metamodelica::nil();
    let mut SCCs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>>> = metamodelica::nil();
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut graphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut levelTasks: Arc<metamodelica::List<HpcOmSimCode::TaskList>> = metamodelica::nil();
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut rootNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>;
    let mut compParamMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut compInformations: metamodelica::Array<HpcOmTaskGraph::ComponentInfo>;
    targetCost = metamodelica::OrderedFloat(1000.0_f64);
    let HpcOmTaskGraph::TASKGRAPHMETA { inComps: __pa0, .. } = (iMeta.clone()) else { bail!("pattern mismatch") };
    inComps = __pa0.clone();
    graphT = AdjacencyMatrix::transposeAdjacencyMatrix(iGraph.clone(), (iGraph.clone().borrow().len() as i32))?;
    level = HpcOmTaskGraph::getLevelNodes(iGraph.clone());
    levelAss = arrayCreate((inComps.clone().borrow().len() as i32), -1);
    (_, levelAss) = List::fold(level.clone(), Arc::new(fnptr!(getLevelAssignment, Arc<metamodelica::List<i32>>, (i32, metamodelica::Array<i32>))), (1, levelAss.clone()));
    let __pa1 = ::match_deref::match_deref! { match &(HpcOmTaskGraph::getCriticalPaths(iGraph.clone(), iMeta.clone())?) {
        (_, (Deref @ metamodelica::List::Cons { head: __pa1, tail: _ }, _)) => __pa1.clone(),
        _ => bail!("pattern mismatch"),
    } };
    critPathNodes = __pa1.clone();
    critPathCosts = List::map1(critPathNodes.clone(), Arc::new(HpcOmTaskGraph::getExeCostReqCycles), iMeta.clone());
    allSections = BLS_fillParallelSections(level.clone(), levelAss.clone(), critPathNodes.clone(), 1, targetCost.clone(), iGraph.clone(), graphT.clone(), iMeta.clone(), metamodelica::nil(), metamodelica::nil())?;
    allSections = List::map2(allSections.clone(), Arc::new(BLS_mergeSmallSections), iMeta.clone(), targetCost.clone());
    levelTasks = List::map2(allSections.clone(), Arc::new(BLS_generateSchedule), iMeta.clone(), iSccSimEqMapping.clone());
    oSchedule = Arc::new(HpcOmSimCode::Schedule::LEVELSCHEDULE { tasksOfLevels: levelTasks.clone(), useFixedAssignments: false });
    let HpcOmTaskGraph::TASKGRAPHMETA { compInformations: __pa2, commCosts: __pa3, exeCosts: __pa4, compDescs: __pa5, compNames: __pa6, compParamMapping: __pa7, eqCompMapping: __pa8, varCompMapping: __pa9, inComps: __pa10, .. } = (iMeta.clone()) else { bail!("pattern mismatch") };
    compInformations = __pa2.clone();
    commCosts = __pa3.clone();
    exeCosts = __pa4.clone();
    compDescs = __pa5.clone();
    compNames = __pa6.clone();
    compParamMapping = __pa7.clone();
    eqCompMapping = __pa8.clone();
    varCompMapping = __pa9.clone();
    inComps = __pa10.clone();
    nodeMark = arrayCreate((inComps.clone().borrow().len() as i32), -1);
    level = List::map(allSections.clone(), Arc::new(fnptr!(List::flatten, _)));
    (_, nodeMark) = List::fold(level.clone(), Arc::new(fnptr!(getLevelAssignment, Arc<metamodelica::List<i32>>, (i32, metamodelica::Array<i32>))), (1, nodeMark.clone()));
    oMeta = HpcOmTaskGraph::TaskGraphMeta { inComps: inComps.clone(), varCompMapping: varCompMapping.clone(), eqCompMapping: eqCompMapping.clone(), compParamMapping: compParamMapping.clone(), compNames: compNames.clone(), compDescs: compDescs.clone(), exeCosts: exeCosts.clone(), commCosts: commCosts.clone(), nodeMark: nodeMark.clone(), compInformations: compInformations.clone() };
    Ok((oSchedule, oMeta))
}

fn BLS_mergeSmallSections(mut sectionsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut iMeta: HpcOmTaskGraph::TaskGraphMeta, mut targetCosts: metamodelica::Real) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut sectionsOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    sectionsOut = (::match_deref::match_deref! { match &((sectionsIn.clone(), iMeta.clone(), targetCosts.clone())) {
        (_, _, _) => {
            let mut costs: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
            let mut mergedSectionIdcs: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut sectionsNew: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut sectionsNewUnflattened: Arc<metamodelica::List<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>>> = metamodelica::nil();
            let mut sectionCosts: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            costs = List::map1List(sectionsIn.clone(), Arc::new(HpcOmTaskGraph::getExeCostReqCycles), iMeta.clone());
            sectionCosts = List::map(costs.clone(), Arc::new(fnptr!(realSum, Arc<metamodelica::List<metamodelica::Real>>)));
            (mergedSectionIdcs, _) = BLS_mergeToTargetSize(List::intRange((sectionsIn.clone().len() as i32)), sectionCosts.clone(), targetCosts.clone(), metamodelica::nil())?;
            sectionsNewUnflattened = List::map1List(mergedSectionIdcs.clone(), Arc::new(fnptr!(List::getIndexFirst, i32, _)), sectionsIn.clone());
            sectionsNew = List::map(sectionsNewUnflattened.clone(), Arc::new(fnptr!(List::flatten, _)));
            sectionsNew = List::map1(sectionsNew.clone(), Arc::new(List::sort), fnptr!(intGt, i32, i32));
            sectionsNew.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(sectionsOut)
}

fn BLS_generateSchedule(mut level: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut iMeta: HpcOmTaskGraph::TaskGraphMeta, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<HpcOmSimCode::TaskList> {
    let mut taskLstOut: HpcOmSimCode::TaskList;
    taskLstOut = 'mc: {
        let __mc_input = (level.clone(), iMeta.clone(), iSccSimEqMapping.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: section, tail: Deref @ metamodelica::List::Nil }, HpcOmTaskGraph::TaskGraphMeta { inComps, .. }, _) => {
                    let mut task: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
                    let mut taskLst: HpcOmSimCode::TaskList;
                    let _ = List::flatten(List::map1(section.clone(), Arc::new(fnptr!(Array::getIndexFirst, i32, _)), inComps.clone()));
                    task = makeCalcTaskLevel(section.clone(), inComps.clone(), iSccSimEqMapping.clone())?;
                    taskLst = HpcOmSimCode::TaskList::SERIALTASKLIST { tasks: list![task.clone()], masterOnly: true };
                    Ok(taskLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: _ }, HpcOmTaskGraph::TaskGraphMeta { inComps, .. }, _) => {
                    let mut taskLst: HpcOmSimCode::TaskList;
                    taskLst = makeCalcLevelParTaskLstForMergedNodes(level.clone(), iSccSimEqMapping.clone(), inComps.clone());
                    Ok(taskLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(taskLstOut)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn BLS_fillParallelSections(mut levelIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut levelAssIn: metamodelica::Array<i32>, mut critPathNodes: Arc<metamodelica::List<i32>>, mut levelIdx: i32, mut targetCosts: metamodelica::Real, mut iGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iMeta: HpcOmTaskGraph::TaskGraphMeta, mut unassNodesIn: Arc<metamodelica::List<i32>>, mut sectionsIn: Arc<metamodelica::List<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>>>> {
    let mut sectionsOut: Arc<metamodelica::List<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>>> = metamodelica::nil();
    sectionsOut = 'mc: {
        let __mc_input = (levelIn.clone(), levelAssIn.clone(), critPathNodes.clone(), levelIdx.clone(), targetCosts.clone(), iGraph.clone(), iGraphT.clone(), iMeta.clone(), unassNodesIn.clone(), sectionsIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _) => {
                    Ok(sectionsIn.clone().reverse())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Cons { head: critPathNode, tail: Deref @ metamodelica::List::Nil }, _, _, _, _, _, _, _) => {
                    let mut critNodeLevel: i32 = 0;
                    let mut levelNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut unassNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut levelNodeCluster: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut followingLevel: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut sectionLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>>> = metamodelica::nil();
                    let _ = HpcOmTaskGraph::getExeCostReqCycles(critPathNode.clone(), iMeta.clone())?;
                    critNodeLevel = levelAssIn.clone().borrow()[(critPathNode.clone()-1) as usize].clone();
                    critNodeLevel = intMin(levelIdx.clone(), critNodeLevel.clone());
                    (_, followingLevel) = List::split(levelIn.clone(), critNodeLevel.clone() - 1)?;
                    levelNodes = List::flatten(followingLevel.clone());
                    unassNodes = listAppend(levelNodes.clone(), unassNodesIn.clone());
                    levelNodeCluster = BLS_mergeDependentLevelTask(unassNodes.clone(), iGraph.clone(), iGraphT.clone(), metamodelica::nil())?;
                    sectionLst = cons(levelNodeCluster.clone(), sectionsIn.clone());
                    sectionLst = BLS_fillParallelSections(levelIn.clone(), levelAssIn.clone(), metamodelica::nil(), critNodeLevel.clone() + 1, targetCosts.clone(), iGraph.clone(), iGraphT.clone(), iMeta.clone(), unassNodes.clone(), sectionLst.clone())?;
                    Ok(sectionLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Cons { head: critPathNode, tail: restCritNodes }, _, _, _, _, _, _, _) => {
                    let mut critPathCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut critNodeLevel: i32 = 0;
                    let mut section: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut levelNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut unassNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut necessaryPredecessors: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut level: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut sectionLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>>> = metamodelica::nil();
                    critPathCost = HpcOmTaskGraph::getExeCostReqCycles(critPathNode.clone(), iMeta.clone())?;
                    critNodeLevel = levelAssIn.clone().borrow()[(critPathNode.clone()-1) as usize].clone();
                    let true = (critPathCost.clone() < targetCosts.clone()) else { bail!("pattern mismatch") };
                    levelNodes = List::flatten(List::map1(List::intRange2(levelIdx.clone(), critNodeLevel.clone()), Arc::new(fnptr!(List::getIndexFirst, i32, _)), levelIn.clone()));
                    (levelNodes, _) = List::deleteMemberOnTrue(critPathNode.clone(), levelNodes.clone(), Arc::new(fnptr!(intEq, i32, i32)))?;
                    necessaryPredecessors = iGraphT.clone().borrow()[(listHead(restCritNodes.clone())?-1) as usize].clone();
                    unassNodes = listAppend(levelNodes.clone(), unassNodesIn.clone());
                    necessaryPredecessors = List::flatten(List::map4(List::map(necessaryPredecessors.clone(), Arc::new(fnptr!(List::create, _))), Arc::new(BLS_getDependentGroups), iGraph.clone(), iGraphT.clone(), unassNodes.clone(), metamodelica::nil()));
                    necessaryPredecessors = List::unique(necessaryPredecessors.clone());
                    (necessaryPredecessors, _, unassNodes) = List::intersection1OnTrue(necessaryPredecessors.clone(), unassNodes.clone(), Arc::new(fnptr!(intEq, i32, i32)))?;
                    section = cons(critPathNode.clone(), necessaryPredecessors.clone());
                    section = List::unique(section.clone());
                    sectionLst = cons(list![section.clone()], sectionsIn.clone());
                    List::map2_0(section.clone(), Arc::new(Array::updateIndexFirst), critNodeLevel.clone(), levelAssIn.clone());
                    level = List::map1(levelIn.clone(), Arc::new(deleteIntListMembers), section.clone());
                    level = List::set(level.clone(), critNodeLevel.clone(), section.clone())?;
                    sectionLst = BLS_fillParallelSections(level.clone(), levelAssIn.clone(), restCritNodes.clone(), critNodeLevel.clone() + 1, targetCosts.clone(), iGraph.clone(), iGraphT.clone(), iMeta.clone(), unassNodes.clone(), sectionLst.clone())?;
                    Ok(sectionLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Cons { head: critPathNode, tail: restCritNodes }, _, _, _, _, _, _, _) => {
                    let mut critPathCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut critNodeLevel: i32 = 0;
                    let mut levelNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut unassNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut level: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut levelNodeCluster: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut sectionLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>>> = metamodelica::nil();
                    critPathCost = HpcOmTaskGraph::getExeCostReqCycles(critPathNode.clone(), iMeta.clone())?;
                    critNodeLevel = levelAssIn.clone().borrow()[(critPathNode.clone()-1) as usize].clone();
                    let true = (critPathCost.clone() >= targetCosts.clone()) else { bail!("pattern mismatch") };
                    let _ = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
                    levelNodes = List::flatten(List::map1(List::intRange2(levelIdx.clone(), critNodeLevel.clone()), Arc::new(fnptr!(List::getIndexFirst, i32, _)), levelIn.clone()));
                    (levelNodes, _) = List::deleteMemberOnTrue(critPathNode.clone(), levelNodes.clone(), Arc::new(fnptr!(intEq, i32, i32)))?;
                    let _ = iGraphT.clone().borrow()[(listHead(restCritNodes.clone())?-1) as usize].clone();
                    unassNodes = listAppend(unassNodesIn.clone(), levelNodes.clone());
                    unassNodes = cons(critPathNode.clone(), unassNodes.clone());
                    unassNodes = List::unique(unassNodes.clone());
                    levelNodeCluster = BLS_mergeDependentLevelTask(unassNodes.clone(), iGraph.clone(), iGraphT.clone(), metamodelica::nil())?;
                    (_, unassNodes, _) = List::intersection1OnTrue(unassNodes.clone(), List::flatten(levelNodeCluster.clone()), Arc::new(fnptr!(intEq, i32, i32)))?;
                    sectionLst = cons(levelNodeCluster.clone(), sectionsIn.clone());
                    List::map2_0(List::flatten(levelNodeCluster.clone()), Arc::new(Array::updateIndexFirst), critNodeLevel.clone(), levelAssIn.clone());
                    level = List::map1(levelIn.clone(), Arc::new(deleteIntListMembers), List::flatten(levelNodeCluster.clone()));
                    level = List::set(level.clone(), critNodeLevel.clone(), List::flatten(levelNodeCluster.clone()))?;
                    sectionLst = BLS_fillParallelSections(level.clone(), levelAssIn.clone(), restCritNodes.clone(), critNodeLevel.clone() + 1, targetCosts.clone(), iGraph.clone(), iGraphT.clone(), iMeta.clone(), metamodelica::nil(), sectionLst.clone())?;
                    Ok(sectionLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(sectionsOut)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn BLS_mergeDependentLevelTask(mut nodesIn: Arc<metamodelica::List<i32>>, mut iGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut sectionsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut sectionsOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    sectionsOut = (::match_deref::match_deref! { match &((nodesIn.clone(), iGraph.clone(), iGraphT.clone(), sectionsIn.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _) => {
            sectionsIn.clone().reverse()
        },
        (Deref @ metamodelica::List::Cons { head: node, tail: rest }, _, _, _) => {
            let mut dependentNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut section: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut sections: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut rest = (*rest).clone();
            dependentNodes = BLS_getDependentGroups(list![node.clone()], iGraph.clone(), iGraphT.clone(), nodesIn.clone(), metamodelica::nil())?;
            section = cons(node.clone(), dependentNodes.clone());
            section = List::unique(section.clone());
            (_, rest, _) = List::intersection1OnTrue(rest.clone(), dependentNodes.clone(), Arc::new(fnptr!(intEq, i32, i32)))?;
            section = section.clone().reverse();
            sections = BLS_mergeDependentLevelTask(rest.clone(), iGraph.clone(), iGraphT.clone(), cons(section.clone(), sectionsIn.clone()))?;
            sections.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(sectionsOut)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn BLS_getDependentGroups(mut nodes: Arc<metamodelica::List<i32>>, mut iGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut referenceNodesIn: Arc<metamodelica::List<i32>>, mut dependentsIn: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut dependentsOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    dependentsOut = 'mc: {
        let __mc_input = (nodes.clone(), iGraph.clone(), iGraphT.clone(), referenceNodesIn.clone(), dependentsIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _) => {
                    Ok(List::unique(dependentsIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: node, tail: rest }, _, _, _, _) => {
                    let mut successors: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut predecessors: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut dependentNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut referenceNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut allNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    successors = iGraph.clone().borrow()[(node.clone()-1) as usize].clone();
                    predecessors = iGraphT.clone().borrow()[(node.clone()-1) as usize].clone();
                    (successors, _, referenceNodes) = List::intersection1OnTrue(successors.clone(), referenceNodesIn.clone(), Arc::new(fnptr!(intEq, i32, i32)))?;
                    (predecessors, _, referenceNodes) = List::intersection1OnTrue(predecessors.clone(), referenceNodes.clone(), Arc::new(fnptr!(intEq, i32, i32)))?;
                    dependentNodes = listAppend(predecessors.clone(), successors.clone());
                    allNodes = cons(node.clone(), dependentNodes.clone());
                    dependentNodes = BLS_getDependentGroups(listAppend(rest.clone(), dependentNodes.clone()), iGraph.clone(), iGraphT.clone(), referenceNodes.clone(), listAppend(allNodes.clone(), dependentsIn.clone()))?;
                    Ok(dependentNodes.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("BLS_getDependentGroups failed!\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(dependentsOut)
}

fn BLS_mergeToTargetSize(mut nodesIn: Arc<metamodelica::List<i32>>, mut costsIn: Arc<metamodelica::List<metamodelica::Real>>, mut targetSize: metamodelica::Real, mut mergedNodesIn: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, metamodelica::Real)>>) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Arc<metamodelica::List<metamodelica::Real>>)> {
    let mut clustersOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut clusterCostsOut: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    (clustersOut, clusterCostsOut) = 'mc: {
        let __mc_input = (nodesIn.clone(), costsIn.clone(), targetSize.clone(), mergedNodesIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _, Deref @ metamodelica::List::Nil) => {
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _, _) => {
                    let mut cluster: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut clusterTmp: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut clusterCostsTmp: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
                    clusterCostsTmp = List::map(mergedNodesIn.clone(), Arc::new(fnptr!(Util::tuple22, _)));
                    clusterTmp = List::map(mergedNodesIn.clone(), Arc::new(fnptr!(Util::tuple21, _))).reverse();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(clusterTmp.clone()) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cluster = __pa0.clone();
                    clusterTmp = __pa1.clone();
                    cluster = if (clusterTmp.clone().is_empty()) {cluster.clone().reverse()} else {cluster.clone()};
                    clusterTmp = cons(cluster.clone(), clusterTmp.clone());
                    Ok((clusterTmp.clone(), clusterCostsTmp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: node, tail: nodeRest }, Deref @ metamodelica::List::Cons { head: cost, tail: costRest }, _, Deref @ metamodelica::List::Nil) => {
                    let mut clusterTmp: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut clusterCostsTmp: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
                    (clusterTmp, clusterCostsTmp) = BLS_mergeToTargetSize(nodeRest.clone(), costRest.clone(), targetSize.clone(), list![(list![node.clone()], cost.clone())])?;
                    Ok((clusterTmp.clone(), clusterCostsTmp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: node, tail: nodeRest }, Deref @ metamodelica::List::Cons { head: cost, tail: costRest }, _, Deref @ metamodelica::List::Cons { head: group, tail: restGroups }) => {
                    let mut clusterCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut cluster: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut clusterTmp: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut clusterCostsTmp: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
                    let mut group = (*group).clone();
                    (cluster, clusterCost) = group.clone();
                    let true = (clusterCost.clone() + cost.clone() < targetSize.clone()) else { bail!("pattern mismatch") };
                    (group, _) = (cons(node.clone(), cluster.clone()), cost.clone() + clusterCost.clone());
                    (clusterTmp, clusterCostsTmp) = BLS_mergeToTargetSize(nodeRest.clone(), costRest.clone(), targetSize.clone(), cons(group.clone(), restGroups.clone()))?;
                    Ok((clusterTmp.clone(), clusterCostsTmp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: node, tail: nodeRest }, Deref @ metamodelica::List::Cons { head: cost, tail: costRest }, _, Deref @ metamodelica::List::Cons { head: group, tail: restGroups }) => {
                    let mut clusterCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut cluster: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut clusterTmp: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut clusterCostsTmp: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
                    let mut group = (*group).clone();
                    let mut restGroups = (*restGroups).clone();
                    (cluster, clusterCost) = group.clone();
                    let true = (clusterCost.clone() + cost.clone() >= targetSize.clone()) else { bail!("pattern mismatch") };
                    cluster = cluster.clone().reverse();
                    restGroups = cons((cluster.clone(), clusterCost.clone()), restGroups.clone());
                    (group, _) = (list![node.clone()], cost.clone());
                    (clusterTmp, clusterCostsTmp) = BLS_mergeToTargetSize(nodeRest.clone(), costRest.clone(), targetSize.clone(), cons(group.clone(), restGroups.clone()))?;
                    Ok((clusterTmp.clone(), clusterCostsTmp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("BLS_mergeToTargetSize failed!")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((clustersOut, clusterCostsOut))
}

fn realSum(mut reals: Arc<metamodelica::List<metamodelica::Real>>) -> metamodelica::Real {
    let mut sum: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    sum = List::fold(reals.clone(), Arc::new(fnptr!(realAdd, metamodelica::Real, metamodelica::Real)), metamodelica::OrderedFloat(0.0_f64));
    sum
}

fn deleteIntListMembers(mut lst1: Arc<metamodelica::List<i32>>, mut lst2: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut lstOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (_, lstOut, _) = List::intersection1OnTrue(lst1.clone(), lst2.clone(), Arc::new(fnptr!(intEq, i32, i32)))?;
    Ok(lstOut)
}

//-----------------
// Level Scheduling
//-----------------
pub fn createLevelSchedule(mut iGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iMeta: HpcOmTaskGraph::TaskGraphMeta, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> (Arc<HpcOmSimCode::Schedule>, HpcOmTaskGraph::TaskGraphMeta) {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut oMeta: HpcOmTaskGraph::TaskGraphMeta;
    let mut levelTasks: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut tmpSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut levelTaskLists: Arc<metamodelica::List<HpcOmSimCode::TaskList>> = metamodelica::nil();
    levelTasks = HpcOmTaskGraph::getLevelNodes(iGraph.clone());
    levelTaskLists = List::fold(levelTasks.clone(), Arc::new({ let __pe_b1 = iGraph.clone(); let __pe_b2 = iMeta.clone(); let __pe_b3 = iSccSimEqMapping.clone(); move |__pe_a0, __pe_a4| createLevelScheduleForLevel(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_a4) }), metamodelica::nil());
    levelTaskLists = levelTaskLists.clone().reverse();
    oSchedule = Arc::new(HpcOmSimCode::Schedule::LEVELSCHEDULE { tasksOfLevels: levelTaskLists.clone(), useFixedAssignments: false });
    oMeta = iMeta.clone();
    (oSchedule, oMeta)
}

fn createLevelScheduleForLevel(mut iTasksOfLevel: Arc<metamodelica::List<i32>>, mut iGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iMeta: HpcOmTaskGraph::TaskGraphMeta, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iLevelTaskLists: Arc<metamodelica::List<HpcOmSimCode::TaskList>>) -> Result<Arc<metamodelica::List<HpcOmSimCode::TaskList>>> {
    let mut oLevelTaskLists: Arc<metamodelica::List<HpcOmSimCode::TaskList>> = metamodelica::nil();
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut taskList: HpcOmSimCode::TaskList;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut sortedTasksOfLevel: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tasksOfLevel: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let HpcOmTaskGraph::TASKGRAPHMETA { inComps: __pa0, exeCosts: __pa1, .. } = (iMeta.clone()) else { bail!("pattern mismatch") };
    inComps = __pa0.clone();
    exeCosts = __pa1.clone();
    sortedTasksOfLevel = iTasksOfLevel.clone();
    taskList = makeCalcLevelParTaskLst(sortedTasksOfLevel.clone(), iSccSimEqMapping.clone(), inComps.clone());
    oLevelTaskLists = cons(taskList.clone(), iLevelTaskLists.clone());
    Ok(oLevelTaskLists)
}

fn getLevelAssignment(mut level: Arc<metamodelica::List<i32>>, mut tplIn: (i32, metamodelica::Array<i32>)) -> (i32, metamodelica::Array<i32>) {
    let mut tplOut: (i32, metamodelica::Array<i32>);
    let mut idx: i32 = 0;
    let mut ass: metamodelica::Array<i32>;
    (idx, ass) = tplIn.clone();
    List::map2_0(level.clone(), Arc::new(Array::updateIndexFirst), idx.clone(), ass.clone());
    tplOut = (idx.clone() + 1, ass.clone());
    tplOut
}

fn makeCalcLevelParTaskLst(mut iNodeIdc: Arc<metamodelica::List<i32>>, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iNodeSccMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> HpcOmSimCode::TaskList {
    let mut oTasks: HpcOmSimCode::TaskList;
    let mut tmpList: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut nodeIdx: i32 = 0;
    for mut nodeIdx in &*iNodeIdc.clone().reverse() {
        let mut nodeIdx = nodeIdx.clone();
        tmpList = cons(list![nodeIdx.clone()], tmpList.clone());
    }
    oTasks = makeCalcLevelParTaskLstForMergedNodes(tmpList.clone(), iSccSimEqMapping.clone(), iNodeSccMapping.clone());
    oTasks
}

fn makeCalcLevelParTaskLstForMergedNodes(mut iNodeIdc: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iNodeSccMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> HpcOmSimCode::TaskList {
    let mut oTasks: HpcOmSimCode::TaskList;
    let mut tmpList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    tmpList = List::map(iNodeIdc.clone(), Arc::new({ let __pe_b1 = iNodeSccMapping.clone(); let __pe_b2 = iSccSimEqMapping.clone(); move |__pe_a0| makeCalcTaskLevel(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
    oTasks = HpcOmSimCode::TaskList::PARALLELTASKLIST { tasks: tmpList.clone() };
    oTasks
}

fn makeCalcTaskLevel(mut iNodeIdc: Arc<metamodelica::List<i32>>, mut iNodeSccMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<HpcOmSimCode::Task>> {
    let mut oTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut simEqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut sccs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut sccIdx: i32 = 0;
    for mut nodeIdx in &*iNodeIdc.clone() {
        let mut nodeIdx = nodeIdx.clone();
        sccs = iNodeSccMapping.clone().borrow()[(nodeIdx.clone()-1) as usize].clone();
        for mut sccIdx in &*sccs.clone() {
            let mut sccIdx = sccIdx.clone();
            simEqs = List::append_reverse(iSccSimEqMapping.clone().borrow()[(sccIdx.clone()-1) as usize].clone(), simEqs.clone());
        }
    }
    oTask = Arc::new(HpcOmSimCode::Task::CALCTASK_LEVEL { eqIdc: simEqs.clone().reverse(), nodeIdc: iNodeIdc.clone(), threadIdx: None });
    Ok(oTask)
}

pub fn makeCalcTask(mut simEqs: Arc<metamodelica::List<i32>>, mut node: i32, mut threadIdx: i32) -> Arc<HpcOmSimCode::Task> {
    let mut taskOut: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    taskOut = Arc::new(HpcOmSimCode::Task::CALCTASK { weighting: 0, index: node.clone(), calcTime: metamodelica::OrderedFloat(1.0_f64), timeFinished: metamodelica::OrderedFloat(1.0_f64), threadIdx: threadIdx.clone(), eqIdc: simEqs.clone() });
    taskOut
}

fn arrayIntIsNegative(mut node: i32, mut ass: metamodelica::Array<i32>) -> Result<bool> {
    let mut isAss: bool = false;
    isAss = intLt(ass.clone().borrow()[(node.clone()-1) as usize].clone(), 0);
    Ok(isAss)
}

fn dumpLevelSchedule(mut iLevelInfo: HpcOmSimCode::TaskList, mut iLevel: i32) -> Result<(ArcStr, i32)> {
    let mut levelStr: ArcStr = arcstr::literal!("");
    let mut oLevel: i32 = 0;
    let mut s: ArcStr = arcstr::literal!("");
    let mut tasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    (levelStr, oLevel) = (match (iLevelInfo.clone(), iLevel.clone()) {
        (HpcOmSimCode::TaskList::PARALLELTASKLIST { tasks: mut tasks }, _) => {
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Parallel Level ")); __mm_s.push_str(&*intString(iLevel.clone())); __mm_s.push_str(&*literal!(":\n")); ArcStr::from(__mm_s) }).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*dumpTaskList(tasks.clone())); ArcStr::from(__mm_s) }).clone();
            (s.clone(), iLevel.clone() + 1)
        },
        (HpcOmSimCode::TaskList::SERIALTASKLIST { tasks: mut tasks, .. }, _) => {
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Serial Level ")); __mm_s.push_str(&*intString(iLevel.clone())); __mm_s.push_str(&*literal!(":\n")); ArcStr::from(__mm_s) }).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*dumpTaskList(tasks.clone())); ArcStr::from(__mm_s) }).clone();
            (s.clone(), iLevel.clone() + 1)
        },
        _ => {
            println!("{}", (literal!("printLevelSchedule failed!\n")).clone());
            bail!("fail")
        },
    });
    Ok((levelStr, oLevel))
}

//-----------------------
// Fixed level Scheduling
//-----------------------
pub fn createFixedLevelSchedule(mut iGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iMeta: HpcOmTaskGraph::TaskGraphMeta, mut iNumberOfThreads: i32, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> (Arc<HpcOmSimCode::Schedule>, HpcOmTaskGraph::TaskGraphMeta) {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut oMeta: HpcOmTaskGraph::TaskGraphMeta;
    let mut levelTasks: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut adviceLists: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut tmpSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut levelTaskLists: Arc<metamodelica::List<HpcOmSimCode::TaskList>> = metamodelica::nil();
    levelTasks = HpcOmTaskGraph::getLevelNodes(iGraph.clone());
    adviceLists = arrayCreate((iGraph.clone().borrow().len() as i32), metamodelica::nil());
    levelTaskLists = List::fold(levelTasks.clone(), Arc::new({ let __pe_b1 = adviceLists.clone(); let __pe_b2 = iGraph.clone(); let __pe_b3 = iMeta.clone(); let __pe_b4 = iNumberOfThreads.clone(); let __pe_b5 = iSccSimEqMapping.clone(); move |__pe_a0, __pe_a6| createFixedLevelScheduleForLevel(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_a6) }), metamodelica::nil());
    levelTaskLists = levelTaskLists.clone().reverse();
    oSchedule = Arc::new(HpcOmSimCode::Schedule::LEVELSCHEDULE { tasksOfLevels: levelTaskLists.clone(), useFixedAssignments: true });
    oMeta = iMeta.clone();
    (oSchedule, oMeta)
}

fn createFixedLevelScheduleForLevel(mut iTasksOfLevel: Arc<metamodelica::List<i32>>, mut iAdviceList: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iMeta: HpcOmTaskGraph::TaskGraphMeta, mut iNumberOfThreads: i32, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iLevelTaskLists: Arc<metamodelica::List<HpcOmSimCode::TaskList>>) -> Result<Arc<metamodelica::List<HpcOmSimCode::TaskList>>> {
    let mut oLevelTaskLists: Arc<metamodelica::List<HpcOmSimCode::TaskList>> = metamodelica::nil();
    let mut levelExecCosts: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut threadReadyList: metamodelica::Array<metamodelica::Real>;
    let mut threadTaskList: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut taskList: HpcOmSimCode::TaskList;
    let mut tasksOfLevel: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut sortedTasksOfLevel: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let HpcOmTaskGraph::TASKGRAPHMETA { inComps: __pa0, exeCosts: __pa1, .. } = (iMeta.clone()) else { bail!("pattern mismatch") };
    inComps = __pa0.clone();
    exeCosts = __pa1.clone();
    levelExecCosts = HpcOmTaskGraph::getCostsForContractedNodes(iTasksOfLevel.clone(), exeCosts.clone());
    threadReadyList = arrayCreate(iNumberOfThreads.clone(), metamodelica::OrderedFloat(0.0_f64));
    threadTaskList = arrayCreate(iNumberOfThreads.clone(), metamodelica::nil());
    sortedTasksOfLevel = List::sort(iTasksOfLevel.clone(), Arc::new({ let __pe_b2 = inComps.clone(); let __pe_b3 = exeCosts.clone(); let __pe_b4 = true; move |__pe_a0, __pe_a1| HpcOmTaskGraph::compareTasksByExecTime(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }))?;
    let _ = List::fold(sortedTasksOfLevel.clone(), Arc::new({ let __pe_b1 = levelExecCosts.clone(); let __pe_b2 = iAdviceList.clone(); let __pe_b3 = threadReadyList.clone(); let __pe_b4 = iGraph.clone(); let __pe_b5 = iMeta.clone(); move |__pe_a0, __pe_a6| createFixedLevelScheduleForTask(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_a6) }), threadTaskList.clone());
    threadTaskList = Array::map(threadTaskList.clone(), Arc::new(listReverse.clone()));
    (_, tasksOfLevel) = Array::fold(threadTaskList.clone(), Arc::new({ let __pe_b1 = inComps.clone(); let __pe_b2 = iSccSimEqMapping.clone(); move |__pe_a0, __pe_a3| createFixedLevelScheduleForLevel0(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }), (1, metamodelica::nil()));
    taskList = HpcOmSimCode::TaskList::PARALLELTASKLIST { tasks: tasksOfLevel.clone() };
    oLevelTaskLists = cons(taskList.clone(), iLevelTaskLists.clone());
    Ok(oLevelTaskLists)
}

fn createFixedLevelScheduleForLevel0(mut iTaskList: Arc<metamodelica::List<i32>>, mut iComps: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iIdxTaskList: (i32, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)) -> Result<(i32, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)> {
    let mut oIdxTaskList: (i32, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>);
    let mut threadIdx: i32 = 0;
    let mut taskList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut newTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut components: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut simEqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut taskIdx: i32 = 0;
    (threadIdx, taskList) = iIdxTaskList.clone();
    for mut taskIdx in &*iTaskList.clone() {
        let mut taskIdx = taskIdx.clone();
        components = iComps.clone().borrow()[(taskIdx.clone()-1) as usize].clone();
        simEqs = List::flatten(List::map(List::map1(components.clone(), Arc::new(fnptr!(Array::getIndexFirst, i32, _)), iSccSimEqMapping.clone()), Arc::new(listReverse.clone())));
        if !(simEqs.clone().is_empty()) {
            simEqs = simEqs.clone();
            newTask = Arc::new(HpcOmSimCode::Task::CALCTASK_LEVEL { eqIdc: simEqs.clone(), nodeIdc: list![taskIdx.clone()], threadIdx: Some(threadIdx.clone()) });
            taskList = cons(newTask.clone(), taskList.clone());
        }
    }
    oIdxTaskList = (threadIdx.clone() + 1, taskList.clone());
    Ok(oIdxTaskList)
}

fn createFixedLevelScheduleForTask(mut iTaskIdx: i32, mut iLevelExecCosts: metamodelica::Real, mut iAdviceList: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iThreadReadyList: metamodelica::Array<metamodelica::Real>, mut iGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iMeta: HpcOmTaskGraph::TaskGraphMeta, mut iThreadTasks: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut oThreadTasks: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut adviceElem: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut threadTasks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut successorList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut threadIdx: i32 = 0;
    let mut threadReadyTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut exeCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    adviceElem = iAdviceList.clone().borrow()[(iTaskIdx.clone()-1) as usize].clone();
    adviceElem = flattenAdviceList(adviceElem.clone(), (iThreadReadyList.clone().borrow().len() as i32))?;
    threadIdx = getBestFittingThread(adviceElem.clone(), iLevelExecCosts.clone(), iThreadReadyList.clone())?;
    threadTasks = iThreadTasks.clone().borrow()[(threadIdx.clone()-1) as usize].clone();
    successorList = iGraph.clone().borrow()[(iTaskIdx.clone()-1) as usize].clone();
    let _ = List::fold1(successorList.clone(), Arc::new(createFixedLevelScheduleForTask0), threadIdx.clone(), iAdviceList.clone());
    threadReadyTime = iThreadReadyList.clone().borrow()[(threadIdx.clone()-1) as usize].clone();
    (_, exeCost) = HpcOmTaskGraph::getExeCost(iTaskIdx.clone(), iMeta.clone())?;
    threadReadyTime = (threadReadyTime.clone()) + (exeCost.clone());
    let _ = {let _arr = iThreadReadyList.clone(); _arr.borrow_mut()[(threadIdx.clone()-1) as usize] = threadReadyTime.clone(); _arr};
    threadTasks = cons(iTaskIdx.clone(), threadTasks.clone());
    oThreadTasks = {let _arr = iThreadTasks.clone(); _arr.borrow_mut()[(threadIdx.clone()-1) as usize] = threadTasks.clone(); _arr};
    Ok(oThreadTasks)
}

fn createFixedLevelScheduleForTask0(mut iSuccessor: i32, mut iThreadAdvice: i32, mut iAdviceList: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut oAdviceList: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut adviceElem: Arc<metamodelica::List<i32>> = metamodelica::nil();
    adviceElem = iAdviceList.clone().borrow()[(iSuccessor.clone()-1) as usize].clone();
    adviceElem = cons(iThreadAdvice.clone(), adviceElem.clone());
    oAdviceList = {let _arr = iAdviceList.clone(); _arr.borrow_mut()[(iSuccessor.clone()-1) as usize] = adviceElem.clone(); _arr};
    Ok(oAdviceList)
}

fn flattenAdviceList(mut iAdviceList: Arc<metamodelica::List<i32>>, mut iNumOfThreads: i32) -> Result<Arc<metamodelica::List<i32>>> {
    let mut oAdviceList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut counterArray: metamodelica::Array<i32>;
    let mut tupleList: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    counterArray = arrayCreate(iNumOfThreads.clone(), 0);
    counterArray = List::fold(iAdviceList.clone(), Arc::new(flattenAdviceListElem), counterArray.clone());
    tupleList = arrayToTupleListZeroRemoved(counterArray.clone(), 1, metamodelica::nil())?;
    oAdviceList = List::map(List::sort(tupleList.clone(), Arc::new(fnptr!(intTpl22Gt, (i32, i32), (i32, i32))))?, Arc::new(fnptr!(Util::tuple21, _)));
    Ok(oAdviceList)
}

fn flattenAdviceListElem(mut iAdviceElem: i32, mut iCounterArray: metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> {
    let mut oCounterArray: metamodelica::Array<i32>;
    let mut counter: i32 = 0;
    counter = iCounterArray.clone().borrow()[(iAdviceElem.clone()-1) as usize].clone();
    counter = counter.clone() + 1;
    oCounterArray = {let _arr = iCounterArray.clone(); _arr.borrow_mut()[(iAdviceElem.clone()-1) as usize] = counter.clone(); _arr};
    Ok(oCounterArray)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn arrayToTupleListZeroRemoved(mut iArray: metamodelica::Array<i32>, mut iCurrentIdx: i32, mut iTupleList: Arc<metamodelica::List<(i32, i32)>>) -> Result<Arc<metamodelica::List<(i32, i32)>>> {
    let mut oTupleList: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut tmpTupleList: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut currentValue: i32 = 0;
    oTupleList = 'mc: {
        let __mc_input = (iArray.clone(), iCurrentIdx.clone(), iTupleList.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut tmpTupleList: Arc<metamodelica::List<(i32, i32)>> = tmpTupleList.clone();
                    let mut currentValue: i32 = currentValue.clone();
                    let true = (intLe(iCurrentIdx.clone(), (iArray.clone().borrow().len() as i32))) else { bail!("pattern mismatch") };
                    currentValue = iArray.clone().borrow()[(iCurrentIdx.clone()-1) as usize].clone();
                    let true = (intNe(currentValue.clone(), 0)) else { bail!("pattern mismatch") };
                    tmpTupleList = cons((iCurrentIdx.clone(), currentValue.clone()), iTupleList.clone());
                    tmpTupleList = arrayToTupleListZeroRemoved(iArray.clone(), iCurrentIdx.clone() + 1, tmpTupleList.clone())?;
                    Ok(tmpTupleList.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut tmpTupleList: Arc<metamodelica::List<(i32, i32)>> = tmpTupleList.clone();
                    let true = (intLe(iCurrentIdx.clone(), (iArray.clone().borrow().len() as i32))) else { bail!("pattern mismatch") };
                    tmpTupleList = arrayToTupleListZeroRemoved(iArray.clone(), iCurrentIdx.clone() + 1, iTupleList.clone())?;
                    Ok(tmpTupleList.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iTupleList.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oTupleList)
}

fn intTpl22Gt(mut iTpl1: (i32, i32), mut iTpl2: (i32, i32)) -> bool {
    let mut oRes: bool = false;
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    (_, val1) = iTpl1.clone();
    (_, val2) = iTpl2.clone();
    oRes = intGt(val1.clone(), val2.clone());
    oRes
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getBestFittingThread(mut iAdviceList: Arc<metamodelica::List<i32>>, mut iLevelExecCosts: metamodelica::Real, mut iThreadReadyList: metamodelica::Array<metamodelica::Real>) -> Result<i32> {
    let mut oThreadIdx: i32 = 0;
    let mut averageThreadTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut readyTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut numOfThreads: i32 = 0;
    let mut threadIdx: i32 = 0;
    let mut head: i32 = 0;
    let mut tail: Arc<metamodelica::List<i32>> = metamodelica::nil();
    oThreadIdx = 'mc: {
        let __mc_input = (iAdviceList.clone(), iLevelExecCosts.clone(), iThreadReadyList.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    let mut threadIdx: i32 = threadIdx.clone();
                    threadIdx = getFirstReadyThread(iThreadReadyList.clone());
                    Ok(threadIdx.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: head, tail: tail }, _, _) => {
                    let mut numOfThreads: i32 = numOfThreads.clone();
                    let mut averageThreadTime: metamodelica::Real = averageThreadTime.clone();
                    let mut readyTime: metamodelica::Real = readyTime.clone();
                    readyTime = iThreadReadyList.clone().borrow()[(head.clone()-1) as usize].clone();
                    numOfThreads = (iThreadReadyList.clone().borrow().len() as i32);
                    averageThreadTime = realDiv(iLevelExecCosts.clone(), intReal(numOfThreads.clone()));
                    let true = (realLt(readyTime.clone(), averageThreadTime.clone())) else { bail!("pattern mismatch") };
                    Ok(head.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: head, tail: tail }, _, _) => {
                    Ok(getBestFittingThread(tail.clone(), iLevelExecCosts.clone(), iThreadReadyList.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oThreadIdx)
}

fn getFirstReadyThread(mut iThreadReadyList: metamodelica::Array<metamodelica::Real>) -> i32 {
    let mut oFirstReadyThreadIdx: i32 = 0;
    (oFirstReadyThreadIdx, _, _) = Array::fold(iThreadReadyList.clone(), Arc::new(fnptr!(getFirstReadyThread0, metamodelica::Real, (i32, metamodelica::Real, i32))), (-1, metamodelica::OrderedFloat(-1.0_f64), 1));
    oFirstReadyThreadIdx
}

fn getFirstReadyThread0(mut iThreadReadyTime: metamodelica::Real, mut iFirstReadyThread: (i32, metamodelica::Real, i32)) -> (i32, metamodelica::Real, i32) {
    let mut oFirstReadyThread: (i32, metamodelica::Real, i32);
    let mut firstThreadIdx: i32 = 0;
    let mut currentThreadIdx: i32 = 0;
    let mut readyTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut isLower: bool = false;
    oFirstReadyThread = (match (iThreadReadyTime.clone(), iFirstReadyThread.clone()) {
        (_, ((-1), _, mut currentThreadIdx)) => (currentThreadIdx.clone(), iThreadReadyTime.clone(), currentThreadIdx.clone() + 1),
        (_, (mut firstThreadIdx, mut readyTime, mut currentThreadIdx)) => {
            isLower = realLt(iThreadReadyTime.clone(), readyTime.clone());
            firstThreadIdx = if (isLower.clone()) {currentThreadIdx.clone()} else {firstThreadIdx.clone()};
            readyTime = if (isLower.clone()) {iThreadReadyTime.clone()} else {readyTime.clone()};
            (firstThreadIdx.clone(), readyTime.clone(), currentThreadIdx.clone() + 1)
        },
        _ => {
            println!("{}", (literal!("getFirstReadyThread0 failed\n")).clone());
            iFirstReadyThread.clone()
        },
    });
    oFirstReadyThread
}

//---------------------------
// Task Dependency Scheduling
//---------------------------
pub fn createTaskDepSchedule(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<HpcOmSimCode::Schedule>> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut tmpSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut nodeLevelMap: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut filteredNodeLevelMap: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    oSchedule = 'mc: {
        let __mc_input = (iTaskGraph.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, HpcOmTaskGraph::TaskGraphMeta { nodeMark: mut nodeMark, inComps: mut inComps, .. }, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut filteredNodeLevelMap: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>> = filteredNodeLevelMap.clone();
            let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut nodeLevelMap: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32, Arc<metamodelica::List<i32>>)>> = nodeLevelMap.clone();
            let mut tmpSchedule: Arc<HpcOmSimCode::Schedule>;
            taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), (iTaskGraph.clone().borrow().len() as i32))?;
            (_, nodeLevelMap) = Array::fold(taskGraphT.clone(), Arc::new({ let __pe_b1 = nodeMark.clone(); let __pe_b2 = inComps.clone(); let __pe_b3 = iSccSimEqMapping.clone(); move |__pe_a0, __pe_a4| createNodeLevelMapping(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_a4) }), (1, metamodelica::nil()));
            nodeLevelMap = List::sort(nodeLevelMap.clone(), Arc::new(sortNodeLevelMapping))?;
            filteredNodeLevelMap = List::map(nodeLevelMap.clone(), Arc::new(fnptr!(filterNodeLevelMapping, (Arc<HpcOmSimCode::Task>, i32, Arc<metamodelica::List<i32>>))));
            filteredNodeLevelMap = filteredNodeLevelMap.clone().reverse();
            tmpSchedule = Arc::new(HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: filteredNodeLevelMap.clone() });
            Ok(tmpSchedule.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            println!("{}", (literal!("HpcOmScheduler.createTaskDepSchedule failed.\n")).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oSchedule)
}

fn createNodeLevelMapping(mut iNodeDependenciesT: Arc<metamodelica::List<i32>>, mut nodeMarks: metamodelica::Array<i32>, mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iNodeInfo: (i32, Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32, Arc<metamodelica::List<i32>>)>>)) -> Result<(i32, Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32, Arc<metamodelica::List<i32>>)>>)> {
    let mut oNodeInfo: (i32, Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32, Arc<metamodelica::List<i32>>)>>);
    let mut task: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut nodeIdx: i32 = 0;
    let mut nodeMark: i32 = 0;
    let mut components: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut simEqIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut nodeLevelMap: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    (nodeIdx, nodeLevelMap) = iNodeInfo.clone();
    components = inComps.clone().borrow()[(nodeIdx.clone()-1) as usize].clone();
    nodeMark = nodeMarks.clone().borrow()[(List::last(components.clone())?-1) as usize].clone();
    simEqIdc = List::map(List::map1(components.clone(), Arc::new(getSimEqSysIdxForComp), iSccSimEqMapping.clone()), Arc::new(List::last));
    task = Arc::new(HpcOmSimCode::Task::CALCTASK { weighting: -1, index: nodeIdx.clone(), calcTime: metamodelica::OrderedFloat(-1.0_f64), timeFinished: metamodelica::OrderedFloat(-1.0_f64), threadIdx: -1, eqIdc: simEqIdc.clone() });
    nodeLevelMap = cons((task.clone(), nodeMark.clone(), iNodeDependenciesT.clone()), nodeLevelMap.clone());
    oNodeInfo = (nodeIdx.clone() + 1, nodeLevelMap.clone());
    Ok(oNodeInfo)
}

fn sortNodeLevelMapping(mut iElem1: (Arc<HpcOmSimCode::Task>, i32, Arc<metamodelica::List<i32>>), mut iElem2: (Arc<HpcOmSimCode::Task>, i32, Arc<metamodelica::List<i32>>)) -> Result<bool> {
    let mut oResult: bool = false;
    let mut elemLvl1: i32 = 0;
    let mut elemLvl2: i32 = 0;
    let mut task1Idx: i32 = 0;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(iElem1.clone()) {
        (Deref @ HpcOmSimCode::Task::CALCTASK { index: __pa0, .. }, __pa1, _) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    task1Idx = __pa0.clone();
    elemLvl1 = __pa1.clone();
    (_, elemLvl2, _) = iElem2.clone();
    oResult = intGe(elemLvl1.clone(), elemLvl2.clone());
    Ok(oResult)
}

fn filterNodeLevelMapping(mut iElem: (Arc<HpcOmSimCode::Task>, i32, Arc<metamodelica::List<i32>>)) -> (Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>) {
    let mut oElem: (Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>);
    let mut task: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut childTasks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (task, _, childTasks) = iElem.clone();
    oElem = (task.clone(), childTasks.clone());
    oElem
}

//-----------------
// Metis Scheduling
//-----------------
pub fn createMetisSchedule(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iNumberOfThreads: i32, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<Arc<HpcOmSimCode::Schedule>> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut extInfo: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut xadj: metamodelica::Array<i32>;
    let mut adjncy: metamodelica::Array<i32>;
    let mut vwgt: metamodelica::Array<i32>;
    let mut adjwgt: metamodelica::Array<i32>;
    let mut tmpSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut extInfoArr: metamodelica::Array<i32>;
    let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut rootNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
    let mut nodeList_refCount: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = metamodelica::nil();
    let mut nodeList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut priorityArr: metamodelica::Array<i32>;
    let mut levelNodes: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut procAss: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut priorityTasks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut otherTasks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut order: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut removeLocks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    oSchedule = 'mc: {
        let __mc_input = (iTaskGraph.clone(), iTaskGraphMeta.clone(), iNumberOfThreads.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, HpcOmTaskGraph::TaskGraphMeta { inComps: mut inComps, commCosts: mut commCosts, .. }, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut order: Arc<metamodelica::List<i32>> = order.clone();
            let mut vwgt: metamodelica::Array<i32>;
            let mut extInfoArr: metamodelica::Array<i32>;
            let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
            let mut xadj: metamodelica::Array<i32>;
            let mut adjwgt: metamodelica::Array<i32>;
            let mut priorityArr: metamodelica::Array<i32>;
            let mut priorityTasks: Arc<metamodelica::List<i32>> = priorityTasks.clone();
            let mut otherTasks: Arc<metamodelica::List<i32>> = otherTasks.clone();
            let mut extInfo: Arc<metamodelica::List<i32>> = extInfo.clone();
            let mut procAss: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut tmpSchedule: Arc<HpcOmSimCode::Schedule>;
            let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
            let mut adjncy: metamodelica::Array<i32>;
            let mut levelNodes: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = levelNodes.clone();
            let mut rootNodes: Arc<metamodelica::List<i32>> = rootNodes.clone();
            let mut removeLocks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = removeLocks.clone();
            (xadj, adjncy, vwgt, adjwgt) = prepareMetis(iTaskGraph.clone(), iTaskGraphMeta.clone())?;
            if intGt(iNumberOfThreads.clone(), 1) {
                extInfo = HpcOmSchedulerExt::scheduleMetis(xadj.clone(), adjncy.clone(), vwgt.clone(), adjwgt.clone(), iNumberOfThreads.clone())?;
                extInfoArr = metamodelica::arrayFromVec(extInfo.clone().into_iter().cloned().collect());
            } else {
                extInfoArr = arrayCreate((iTaskGraph.clone().borrow().len() as i32), 1);
                extInfo = Arc::new(extInfoArr.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            }
            let true = (intEq((iTaskGraph.clone().borrow().len() as i32), (extInfoArr.clone().borrow().len() as i32))) else { bail!("pattern mismatch") };
            taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), (iTaskGraph.clone().borrow().len() as i32))?;
            rootNodes = HpcOmTaskGraph::getRootNodes(iTaskGraph.clone())?;
            priorityArr = arrayCreate((iTaskGraph.clone().borrow().len() as i32), 0);
            createMetisSchedule1(List::intRange((iTaskGraph.clone().borrow().len() as i32)), extInfoArr.clone(), iTaskGraph.clone(), taskGraphT.clone(), priorityArr.clone())?;
            levelNodes = HpcOmTaskGraph::getLevelNodes(iTaskGraph.clone());
            allCalcTasks = convertTaskGraphToTasks(taskGraphT.clone(), iTaskGraphMeta.clone(), Arc::new(convertNodeToTask))?;
            (priorityTasks, otherTasks) = createMetisSchedule2(levelNodes.clone(), priorityArr.clone(), metamodelica::nil(), metamodelica::nil())?;
            order = listAppend(priorityTasks.clone(), otherTasks.clone());
            procAss = arrayCreate(iNumberOfThreads.clone(), metamodelica::nil());
            List::map2_0(List::intRange((iTaskGraph.clone().borrow().len() as i32)), Arc::new(getProcAss), extInfoArr.clone(), procAss.clone());
            threadTasks = arrayCreate(iNumberOfThreads.clone(), metamodelica::nil());
            removeLocks = metamodelica::nil();
            tmpSchedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTasks.clone(), outgoingDepTasks: metamodelica::nil(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
            (tmpSchedule, removeLocks) = createScheduleFromAssignments(extInfoArr.clone(), procAss.clone(), Some(order.clone()), iTaskGraph.clone(), taskGraphT.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone(), removeLocks.clone(), order.clone(), iSimVarMapping.clone(), tmpSchedule.clone())?;
            if Flags::isSet(Flags::HPCOM_DUMP.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("number of removed superfluous locks: ")); __mm_s.push_str(&*intString(intDiv((removeLocks.clone().len() as i32), 2))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            tmpSchedule = traverseAndUpdateThreadsInSchedule(tmpSchedule.clone(), Arc::new(removeLocksFromThread), removeLocks.clone())?;
            tmpSchedule = updateLockIdcsInThreadschedule(tmpSchedule.clone(), Arc::new(removeLocksFromLockList), removeLocks.clone());
            Ok(setScheduleLockIds(tmpSchedule.clone())?)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            println!("{}", (literal!("HpcOmScheduler.createMetisSchedule not every node has a scheduler-info.\n")).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oSchedule)
}

fn getProcAss(mut idx: i32, mut taskAss: metamodelica::Array<i32>, mut procAss: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    let mut thread: i32 = 0;
    thread = taskAss.clone().borrow()[(idx.clone()-1) as usize].clone();
    Array::appendToElement(thread.clone(), list![idx.clone()], procAss.clone())?;
    Ok(())
}

fn createMetisSchedule2(mut levelNodes: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut priorityArr: metamodelica::Array<i32>, mut prioLstIn: Arc<metamodelica::List<i32>>, mut otherLstIn: Arc<metamodelica::List<i32>>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut prioLstOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut otherLstOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (prioLstOut, otherLstOut) = (::match_deref::match_deref! { match &((levelNodes.clone(), priorityArr.clone(), prioLstIn.clone(), otherLstIn.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _) => {
            (prioLstIn.clone(), otherLstIn.clone())
        },
        (Deref @ metamodelica::List::Cons { head: level, tail: rest }, _, _, _) => {
            let mut prioLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut otherLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            (prioLst, otherLst) = List::split1OnTrue(level.clone(), Arc::new(isPrioNode), priorityArr.clone());
            prioLst = listAppend(prioLstIn.clone(), prioLst.clone());
            otherLst = listAppend(otherLstIn.clone(), otherLst.clone());
            (prioLst, otherLst) = createMetisSchedule2(rest.clone(), priorityArr.clone(), prioLst.clone(), otherLst.clone())?;
            (prioLst.clone(), otherLst.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((prioLstOut, otherLstOut))
}

fn isPrioNode(mut idx: i32, mut prioArr: metamodelica::Array<i32>) -> Result<bool> {
    let mut isPrio: bool = false;
    isPrio = intEq(1, prioArr.clone().borrow()[(idx.clone()-1) as usize].clone());
    Ok(isPrio)
}

fn createMetisSchedule1(mut taskIdcs: Arc<metamodelica::List<i32>>, mut threadIds: metamodelica::Array<i32>, mut taskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut priorityArr: metamodelica::Array<i32>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (taskIdcs.clone(), threadIds.clone(), taskGraph.clone(), taskGraphT.clone(), priorityArr.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: taskIdx, tail: rest }, _, _, _, _) => {
                    let mut preds: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut rest = (*rest).clone();
                    let true = (intEq(1, priorityArr.clone().borrow()[(taskIdx.clone()-1) as usize].clone())) else { bail!("pattern mismatch") };
                    preds = taskGraphT.clone().borrow()[(taskIdx.clone()-1) as usize].clone();
                    preds = List::filter1OnTrue(preds.clone(), Arc::new(arrayIntIsNotOne), priorityArr.clone());
                    List::map2_0(preds.clone(), Arc::new(Array::updateIndexFirst), 1, priorityArr.clone());
                    rest = listAppend(preds.clone(), rest.clone());
                    createMetisSchedule1(rest.clone(), threadIds.clone(), taskGraph.clone(), taskGraphT.clone(), priorityArr.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: taskIdx, tail: rest }, _, _, _, _) => {
                    let mut threadId: i32 = 0;
                    let mut preds: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut predThreads: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut rest = (*rest).clone();
                    threadId = threadIds.clone().borrow()[(taskIdx.clone()-1) as usize].clone();
                    preds = taskGraphT.clone().borrow()[(taskIdx.clone()-1) as usize].clone();
                    predThreads = List::map1(preds.clone(), Arc::new(fnptr!(Array::getIndexFirst, i32, _)), threadIds.clone());
                    (predThreads, preds) = List::filter1OnTrueSync(predThreads.clone(), Arc::new(fnptr!(intNe, i32, i32)), threadId.clone(), preds.clone())?;
                    if !(predThreads.clone().is_empty()) {
                        List::map2_0(preds.clone(), Arc::new(Array::updateIndexFirst), 1, priorityArr.clone());
                        rest = listAppend(preds.clone(), rest.clone());
                    } else {
                        {let _arr = priorityArr.clone(); _arr.borrow_mut()[(taskIdx.clone()-1) as usize] = 0; _arr};
                    }
                    createMetisSchedule1(rest.clone(), threadIds.clone(), taskGraph.clone(), taskGraphT.clone(), priorityArr.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn arrayIntIsNotOne(mut idx: i32, mut arr: metamodelica::Array<i32>) -> Result<bool> {
    let mut isOne: bool = false;
    isOne = intNe(1, arr.clone().borrow()[(idx.clone()-1) as usize].clone());
    Ok(isOne)
}

pub fn createHMetisSchedule(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iNumberOfThreads: i32, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<Arc<HpcOmSimCode::Schedule>> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut extInfo: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut xadj: metamodelica::Array<i32>;
    let mut adjncy: metamodelica::Array<i32>;
    let mut vwgt: metamodelica::Array<i32>;
    let mut adjwgt: metamodelica::Array<i32>;
    let mut tmpSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut extInfoArr: metamodelica::Array<i32>;
    let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut rootNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
    let mut nodeList_refCount: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = metamodelica::nil();
    let mut nodeList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    oSchedule = 'mc: {
        let __mc_input = (iTaskGraph.clone(), iTaskGraphMeta.clone(), iNumberOfThreads.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, HpcOmTaskGraph::TaskGraphMeta { inComps: mut inComps, commCosts: mut commCosts, .. }, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut adjncy: metamodelica::Array<i32>;
            let mut adjwgt: metamodelica::Array<i32>;
            let mut extInfoArr: metamodelica::Array<i32>;
            let mut rootNodes: Arc<metamodelica::List<i32>> = rootNodes.clone();
            let mut nodeList_refCount: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = nodeList_refCount.clone();
            let mut tmpSchedule: Arc<HpcOmSimCode::Schedule>;
            let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut vwgt: metamodelica::Array<i32>;
            let mut extInfo: Arc<metamodelica::List<i32>> = extInfo.clone();
            let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
            let mut xadj: metamodelica::Array<i32>;
            let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
            let mut nodeList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = nodeList.clone();
            println!("{}", (literal!("Funktionsaufruf!")).clone());
            (xadj, adjncy, vwgt, adjwgt) = preparehMetis(iTaskGraph.clone(), iTaskGraphMeta.clone());
            extInfo = HpcOmSchedulerExt::schedulehMetis(xadj.clone(), adjncy.clone(), vwgt.clone(), adjwgt.clone(), iNumberOfThreads.clone())?;
            extInfoArr = metamodelica::arrayFromVec(extInfo.clone().into_iter().cloned().collect());
            println!("{}", (literal!("Hier geht MetaModelica los!\n")).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("External scheduling info: ")); __mm_s.push_str(&*stringDelimitList(List::map(extInfo.clone(), Arc::new(fnptr!(intString, i32))), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            let true = (intEq((iTaskGraph.clone().borrow().len() as i32), (extInfoArr.clone().borrow().len() as i32))) else { bail!("pattern mismatch") };
            taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), (iTaskGraph.clone().borrow().len() as i32))?;
            rootNodes = HpcOmTaskGraph::getRootNodes(iTaskGraph.clone())?;
            allCalcTasks = convertTaskGraphToTasks(taskGraphT.clone(), iTaskGraphMeta.clone(), Arc::new(convertNodeToTask))?;
            nodeList_refCount = List::map1(rootNodes.clone(), Arc::new(getTaskByIndex), allCalcTasks.clone());
            nodeList = List::map(nodeList_refCount.clone(), Arc::new(fnptr!(Util::tuple21, _)));
            nodeList = List::sort(nodeList.clone(), Arc::new(compareTasksByWeighting))?;
            threadTasks = arrayCreate(iNumberOfThreads.clone(), metamodelica::nil());
            tmpSchedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTasks.clone(), outgoingDepTasks: metamodelica::nil(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
            tmpSchedule = createExtSchedule1(nodeList.clone(), extInfoArr.clone(), iTaskGraph.clone(), taskGraphT.clone(), commCosts.clone(), inComps.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone(), Arc::new(fnptr!(getLocksByPredecessorList, Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, i32, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>)), tmpSchedule.clone())?;
            tmpSchedule = addSuccessorLocksToSchedule(iTaskGraph.clone(), Arc::new(addReleaseLocksToSchedule), commCosts.clone(), inComps.clone(), iSimVarMapping.clone(), tmpSchedule.clone())?;
            Ok(setScheduleLockIds(tmpSchedule.clone())?)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            println!("{}", (literal!("HpcOmScheduler.createHMetisSchedule not every node has a scheduler-info.\n")).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oSchedule)
}

fn sumEdge(mut edges: Arc<metamodelica::List<i32>>, mut innumedge: i32) -> i32 {
    let mut outnumedge: i32 = 0;
    outnumedge = innumedge.clone() + (edges.clone().len() as i32);
    outnumedge
}

fn getSingleRelations(mut edge: i32, mut n: i32, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut irelations: Arc<metamodelica::List<(i32, i32, i32)>>) -> Result<Arc<metamodelica::List<(i32, i32, i32)>>> {
    let mut orelations: Arc<metamodelica::List<(i32, i32, i32)>> = metamodelica::nil();
    let mut costs: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut costsInt: i32 = 0;
    costs = HpcOmTaskGraph::getCommCostTimeBetweenNodes(n.clone(), edge.clone(), iTaskGraphMeta.clone())?;
    costsInt = ((costs.clone()).0 as i32);
    orelations = List::appendElt((edge.clone(), n.clone(), costsInt.clone()), irelations.clone());
    orelations = List::appendElt((n.clone(), edge.clone(), costsInt.clone()), orelations.clone());
    Ok(orelations)
}

fn getRelations(mut edges: Arc<metamodelica::List<i32>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut irelations: (Arc<metamodelica::List<(i32, i32, i32)>>, i32)) -> (Arc<metamodelica::List<(i32, i32, i32)>>, i32) {
    let mut orelations: (Arc<metamodelica::List<(i32, i32, i32)>>, i32);
    let mut n: i32 = 0;
    let mut relations: Arc<metamodelica::List<(i32, i32, i32)>> = metamodelica::nil();
    let mut orel: Arc<metamodelica::List<(i32, i32, i32)>> = metamodelica::nil();
    (relations, n) = irelations.clone();
    orel = List::fold2(edges.clone(), Arc::new(getSingleRelations), n.clone(), iTaskGraphMeta.clone(), relations.clone());
    orelations = (orel.clone(), n.clone() + 1);
    orelations
}

fn sortEdgeHelp(mut edge: (i32, i32, i32), mut actnode: i32, mut adjncy: metamodelica::Array<i32>, mut adjwgt: metamodelica::Array<i32>, mut imarker: i32) -> Result<i32> {
    let mut omarker: i32 = 0;
    omarker = 'mc: {
        let __mc_input = (edge.clone(), actnode.clone(), adjncy.clone(), adjwgt.clone(), imarker.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let ((mut fromnode, mut tonode, mut cost), _, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let true = (intEq(fromnode.clone(), actnode.clone())) else { bail!("pattern mismatch") };
            {let _arr = adjwgt.clone(); _arr.borrow_mut()[(imarker.clone()-1) as usize] = cost.clone(); _arr};
            {let _arr = adjncy.clone(); _arr.borrow_mut()[(imarker.clone()-1) as usize] = tonode.clone() - 1; _arr};
            Ok(imarker.clone() + 1)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            Ok(imarker.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(omarker)
}

fn sortEdge(mut actnode: i32, mut xadj: metamodelica::Array<i32>, mut adjncy: metamodelica::Array<i32>, mut adjwgt: metamodelica::Array<i32>, mut help: Arc<metamodelica::List<(i32, i32, i32)>>, mut imarker: i32) -> Result<i32> {
    let mut omarker: i32 = 0;
    let mut position: i32 = 0;
    omarker = List::fold3(help.clone(), Arc::new(sortEdgeHelp), actnode.clone(), adjncy.clone(), adjwgt.clone(), imarker.clone());
    let _ = {let _arr = xadj.clone(); _arr.borrow_mut()[(actnode.clone() + 1-1) as usize] = omarker.clone() - 1; _arr};
    Ok(omarker)
}

fn setVwgt(mut node: i32, mut vwgt: metamodelica::Array<i32>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta) -> Result<()> {
    let mut value: (i32, metamodelica::Real);
    let mut rv: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    value = HpcOmTaskGraph::getExeCost(node.clone(), iTaskGraphMeta.clone())?;
    (_, rv) = value.clone();
    let _ = {let _arr = vwgt.clone(); _arr.borrow_mut()[(node.clone()-1) as usize] = ((rv.clone()).0 as i32); _arr};
    Ok(())
}

fn prepareMetis(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>, metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut xadj: metamodelica::Array<i32>;
    let mut adjncy: metamodelica::Array<i32>;
    let mut vwgt: metamodelica::Array<i32>;
    let mut adjwgt: metamodelica::Array<i32>;
    let mut n: i32 = 0;
    let mut m: i32 = 0;
    let mut adjundirected: (Arc<metamodelica::List<(i32, i32, i32)>>, i32);
    let mut help: Arc<metamodelica::List<(i32, i32, i32)>> = metamodelica::nil();
    let mut allTheNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    help = metamodelica::nil();
    n = (iTaskGraph.clone().borrow().len() as i32);
    xadj = arrayCreate(n.clone() + 1, 0);
    m = Array::fold(iTaskGraph.clone(), Arc::new(fnptr!(sumEdge, Arc<metamodelica::List<i32>>, i32)), 0);
    adjwgt = arrayCreate(2 * m.clone(), 0);
    adjundirected = Array::fold(iTaskGraph.clone(), Arc::new({ let __pe_b1 = iTaskGraphMeta.clone(); move |__pe_a0, __pe_a2| Ok(getRelations(__pe_a0, __pe_b1.clone(), __pe_a2)) }), (metamodelica::nil(), 1));
    (help, _) = adjundirected.clone();
    allTheNodes = List::intRange(n.clone());
    adjncy = arrayCreate(2 * m.clone(), 0);
    xadj = {let _arr = xadj.clone(); _arr.borrow_mut()[(1-1) as usize] = 0; _arr};
    let _ = List::fold4(allTheNodes.clone(), Arc::new(sortEdge), xadj.clone(), adjncy.clone(), adjwgt.clone(), help.clone(), 1);
    vwgt = arrayCreate(n.clone(), 0);
    List::map2_0(allTheNodes.clone(), Arc::new(setVwgt), vwgt.clone(), iTaskGraphMeta.clone());
    Ok((xadj, adjncy, vwgt, adjwgt))
}

fn listNodes(mut node: i32, mut l_eint: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<i32>> {
    let mut l_eint_out: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut actnode: i32 = 0;
    actnode = node.clone() - 1;
    l_eint_out = listAppend(l_eint.clone(), list![actnode.clone()]);
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("l_eint length:")); __mm_s.push_str(&*intString((l_eint_out.clone().len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    l_eint_out
}

fn getHedge(mut childnodes: Arc<metamodelica::List<i32>>, mut actnode: (i32, i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)) -> (i32, i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) {
    let mut actnode_out: (i32, i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
    actnode_out = (::match_deref::match_deref! { match &((childnodes.clone(), actnode.clone())) {
        (Deref @ metamodelica::List::Nil, (node, position, l_eptr, l_eint, l_hewgts)) => {
            let mut help: (i32, i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
            help = (node.clone() + 1, position.clone(), l_eptr.clone(), l_eint.clone(), l_hewgts.clone());
            help.clone()
        },
        (_, (node, position, l_eptr, l_eint, l_hewgts)) => {
            let mut n: i32 = 0;
            let mut help: (i32, i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
            let mut l_eptr = (*l_eptr).clone();
            let mut l_eint = (*l_eint).clone();
            n = node.clone() - 1;
            l_eint = List::appendElt(n.clone(), l_eint.clone());
            l_eint = List::fold(childnodes.clone(), Arc::new(fnptr!(listNodes, i32, Arc<metamodelica::List<i32>>)), l_eint.clone());
            n = position.clone() + (childnodes.clone().len() as i32) + 1;
            l_eptr = List::appendElt(n.clone(), l_eptr.clone());
            help = (node.clone() + 1, n.clone(), l_eptr.clone(), l_eint.clone(), l_hewgts.clone());
            help.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    actnode_out
}

fn preparehMetis(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta) -> (metamodelica::Array<i32>, metamodelica::Array<i32>, metamodelica::Array<i32>, metamodelica::Array<i32>) {
    let mut vwgts: metamodelica::Array<i32>;
    let mut eptr: metamodelica::Array<i32>;
    let mut eint: metamodelica::Array<i32>;
    let mut hewgts: metamodelica::Array<i32>;
    let mut n: i32 = 0;
    let mut m: i32 = 0;
    let mut l_eptr: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut l_eint: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut l_hewgts: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut allTheNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut result: (i32, i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
    n = (iTaskGraph.clone().borrow().len() as i32);
    result = Array::fold(iTaskGraph.clone(), Arc::new(fnptr!(getHedge, Arc<metamodelica::List<i32>>, (i32, i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>))), (1, 0, list![0], metamodelica::nil(), metamodelica::nil()));
    (_, _, l_eptr, l_eint, l_hewgts) = result.clone();
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Diagnostic length: ")); __mm_s.push_str(&*intString((l_eptr.clone().len() as i32))); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*intString((l_eint.clone().len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    allTheNodes = List::intRange(n.clone());
    vwgts = arrayCreate(n.clone(), 0);
    List::map2_0(allTheNodes.clone(), Arc::new(setVwgt), vwgts.clone(), iTaskGraphMeta.clone());
    eptr = metamodelica::arrayFromVec(l_eptr.clone().into_iter().cloned().collect());
    eint = metamodelica::arrayFromVec(l_eint.clone().into_iter().cloned().collect());
    hewgts = metamodelica::arrayFromVec(l_hewgts.clone().into_iter().cloned().collect());
    (vwgts, eptr, eint, hewgts)
}

//--------------------
// External Scheduling //TODO: Rename to Yed Scheduling
//--------------------
pub fn createExtSchedule(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iNumberOfThreads: i32, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut iGraphMLFile: ArcStr) -> Result<Arc<HpcOmSimCode::Schedule>> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut extInfo: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut extInfoArr: metamodelica::Array<i32>;
    let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut tmpSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>;
    let mut rootNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
    let mut nodeList_refCount: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = metamodelica::nil();
    let mut nodeList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    oSchedule = 'mc: {
        let __mc_input = (iTaskGraph.clone(), iTaskGraphMeta.clone(), iNumberOfThreads.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone(), iGraphMLFile.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, HpcOmTaskGraph::TaskGraphMeta { inComps: mut inComps, commCosts: mut commCosts, .. }, _, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut rootNodes: Arc<metamodelica::List<i32>> = rootNodes.clone();
            let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
            let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
            let mut nodeList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = nodeList.clone();
            let mut tmpSchedule: Arc<HpcOmSimCode::Schedule>;
            let mut extInfo: Arc<metamodelica::List<i32>> = extInfo.clone();
            let mut extInfoArr: metamodelica::Array<i32>;
            let mut nodeList_refCount: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = nodeList_refCount.clone();
            let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            extInfo = HpcOmSchedulerExt::readScheduleFromGraphMl((iGraphMLFile.clone()).clone())?;
            extInfoArr = metamodelica::arrayFromVec(extInfo.clone().into_iter().cloned().collect());
            let true = (intEq((iTaskGraph.clone().borrow().len() as i32), (extInfoArr.clone().borrow().len() as i32))) else { bail!("pattern mismatch") };
            taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), (iTaskGraph.clone().borrow().len() as i32))?;
            rootNodes = HpcOmTaskGraph::getRootNodes(iTaskGraph.clone())?;
            allCalcTasks = convertTaskGraphToTasks(taskGraphT.clone(), iTaskGraphMeta.clone(), Arc::new(convertNodeToTask))?;
            nodeList_refCount = List::map1(rootNodes.clone(), Arc::new(getTaskByIndex), allCalcTasks.clone());
            nodeList = List::map(nodeList_refCount.clone(), Arc::new(fnptr!(Util::tuple21, _)));
            nodeList = List::sort(nodeList.clone(), Arc::new(compareTasksByWeighting))?;
            threadTasks = arrayCreate(iNumberOfThreads.clone(), metamodelica::nil());
            tmpSchedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTasks.clone(), outgoingDepTasks: metamodelica::nil(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
            tmpSchedule = createExtSchedule1(nodeList.clone(), extInfoArr.clone(), iTaskGraph.clone(), taskGraphT.clone(), commCosts.clone(), inComps.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone(), Arc::new(fnptr!(getLocksByPredecessorList, Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, i32, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>)), tmpSchedule.clone())?;
            tmpSchedule = addSuccessorLocksToSchedule(iTaskGraph.clone(), Arc::new(addReleaseLocksToSchedule), commCosts.clone(), inComps.clone(), iSimVarMapping.clone(), tmpSchedule.clone())?;
            Ok(tmpSchedule.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            println!("{}", (literal!("HpcOmScheduler.createExtSchedule not every node has a scheduler-info.\n")).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oSchedule)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn createExtSchedule1(mut iNodeList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut iThreadAssignments: metamodelica::Array<i32>, mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut iLockWithPredecessorHandler: Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, i32, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)> + 'static>, mut iSchedule: Arc<HpcOmSimCode::Schedule>) -> Result<Arc<HpcOmSimCode::Schedule>> {
    pub type FuncType = fn(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, i32, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)>;

    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut head: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut newTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut newTaskRefCount: i32 = 0;
    let mut rest: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut lastChildFinishTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut lastChild: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut predecessors: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = metamodelica::nil();
    let mut successors: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = metamodelica::nil();
    let mut successorIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut outgoingDepTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut newOutgoingDepTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut threadFinishTimes: metamodelica::Array<metamodelica::Real>;
    let mut firstEq: i32 = 0;
    let mut allThreadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut threadTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut lockTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut threadId: i32 = 0;
    let mut threadFinishTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut tmpThreadReadyTimes: metamodelica::Array<metamodelica::Real>;
    let mut tmpNodeList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut weighting: i32 = 0;
    let mut index: i32 = 0;
    let mut calcTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut eqIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut simEqIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
    let mut tmpSchedule: Arc<HpcOmSimCode::Schedule>;
    oSchedule = 'mc: {
        let __mc_input = (iNodeList.clone(), iThreadAssignments.clone(), iTaskGraph.clone(), iTaskGraphT.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone(), iLockWithPredecessorHandler.clone(), iSchedule.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: head @ Deref @ HpcOmSimCode::Task::CALCTASK { eqIdc: eqIdc @ Deref @ metamodelica::List::Cons { head: firstEq, tail: _ }, calcTime, index, weighting, .. }, tail: rest }, _, _, _, _, _, _, _, _, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { allCalcTasks, outgoingDepTasks, threadTasks: allThreadTasks, .. }) => {
                    let mut allCalcTasks = (*allCalcTasks).clone();
                    let mut outgoingDepTasks = (*outgoingDepTasks).clone();
                    let mut allThreadTasks = (*allThreadTasks).clone();
                    let mut tmpSchedule: Arc<HpcOmSimCode::Schedule>;
                    let mut successors: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = successors.clone();
                    let mut predecessors: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = predecessors.clone();
                    let mut newTask: Arc<HpcOmSimCode::Task> = newTask.clone();
                    let mut threadFinishTime: metamodelica::Real = threadFinishTime.clone();
                    let mut simEqIdc: Arc<metamodelica::List<i32>> = simEqIdc.clone();
                    let mut tmpNodeList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = tmpNodeList.clone();
                    let mut threadId: i32 = threadId.clone();
                    let mut newTaskRefCount: i32 = newTaskRefCount.clone();
                    let mut threadTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = threadTasks.clone();
                    let mut successorIdc: Arc<metamodelica::List<i32>> = successorIdc.clone();
                    let mut newOutgoingDepTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = newOutgoingDepTasks.clone();
                    let mut lockTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = lockTasks.clone();
                    (predecessors, _) = getSuccessorsByTask(head.clone(), iTaskGraphT.clone(), allCalcTasks.clone())?;
                    (successors, successorIdc) = getSuccessorsByTask(head.clone(), iTaskGraph.clone(), allCalcTasks.clone())?;
                    let false = (predecessors.clone().is_empty()) else { bail!("pattern mismatch") };
                    threadId = iThreadAssignments.clone().borrow()[(index.clone()-1) as usize].clone();
                    threadFinishTime = metamodelica::OrderedFloat(-1.0_f64);
                    threadTasks = allThreadTasks.clone().borrow()[(threadId.clone()-1) as usize].clone();
                    (lockTasks, newOutgoingDepTasks) = iLockWithPredecessorHandler(head.clone(), predecessors.clone(), threadId.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone())?;
                    outgoingDepTasks = listAppend(outgoingDepTasks.clone(), newOutgoingDepTasks.clone());
                    threadTasks = listAppend(lockTasks.clone(), threadTasks.clone());
                    simEqIdc = List::map(List::map1(eqIdc.clone(), Arc::new(getSimEqSysIdxForComp), iSccSimEqMapping.clone()), Arc::new(List::last));
                    newTask = Arc::new(HpcOmSimCode::Task::CALCTASK { weighting: weighting.clone(), index: index.clone(), calcTime: calcTime.clone(), timeFinished: threadFinishTime.clone(), threadIdx: threadId.clone(), eqIdc: simEqIdc.clone() });
                    threadTasks = cons(newTask.clone(), threadTasks.clone());
                    allThreadTasks = {let _arr = allThreadTasks.clone(); _arr.borrow_mut()[(threadId.clone()-1) as usize] = threadTasks.clone(); _arr};
                    (allCalcTasks, tmpNodeList) = updateRefCounterBySuccessorIdc(allCalcTasks.clone(), successorIdc.clone(), metamodelica::nil())?;
                    tmpNodeList = listAppend(tmpNodeList.clone(), rest.clone());
                    tmpNodeList = List::sort(tmpNodeList.clone(), Arc::new(compareTasksByWeighting))?;
                    (_, newTaskRefCount) = allCalcTasks.clone().borrow()[(index.clone()-1) as usize].clone();
                    {let _arr = allCalcTasks.clone(); _arr.borrow_mut()[(index.clone()-1) as usize] = (newTask.clone(), newTaskRefCount.clone()); _arr};
                    tmpSchedule = createExtSchedule1(tmpNodeList.clone(), iThreadAssignments.clone(), iTaskGraph.clone(), iTaskGraphT.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone(), iLockWithPredecessorHandler.clone(), Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: allThreadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() }))?;
                    Ok(tmpSchedule.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: head @ Deref @ HpcOmSimCode::Task::CALCTASK { eqIdc: eqIdc @ Deref @ metamodelica::List::Cons { head: firstEq, tail: _ }, calcTime, index, weighting, .. }, tail: rest }, _, _, _, _, _, _, _, _, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { allCalcTasks, outgoingDepTasks, threadTasks: allThreadTasks, .. }) => {
                    let mut allCalcTasks = (*allCalcTasks).clone();
                    let mut allThreadTasks = (*allThreadTasks).clone();
                    let mut simEqIdc: Arc<metamodelica::List<i32>> = simEqIdc.clone();
                    let mut successors: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = successors.clone();
                    let mut threadFinishTime: metamodelica::Real = threadFinishTime.clone();
                    let mut successorIdc: Arc<metamodelica::List<i32>> = successorIdc.clone();
                    let mut newTaskRefCount: i32 = newTaskRefCount.clone();
                    let mut tmpSchedule: Arc<HpcOmSimCode::Schedule>;
                    let mut tmpNodeList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = tmpNodeList.clone();
                    let mut threadId: i32 = threadId.clone();
                    let mut threadTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = threadTasks.clone();
                    let mut newTask: Arc<HpcOmSimCode::Task> = newTask.clone();
                    (successors, successorIdc) = getSuccessorsByTask(head.clone(), iTaskGraph.clone(), allCalcTasks.clone())?;
                    threadId = iThreadAssignments.clone().borrow()[(index.clone()-1) as usize].clone();
                    threadFinishTime = metamodelica::OrderedFloat(-1.0_f64);
                    threadTasks = allThreadTasks.clone().borrow()[(threadId.clone()-1) as usize].clone();
                    simEqIdc = List::flatten(List::map1(eqIdc.clone(), Arc::new(getSimEqSysIdxForComp), iSccSimEqMapping.clone()));
                    newTask = Arc::new(HpcOmSimCode::Task::CALCTASK { weighting: weighting.clone(), index: index.clone(), calcTime: calcTime.clone(), timeFinished: threadFinishTime.clone(), threadIdx: threadId.clone(), eqIdc: simEqIdc.clone() });
                    allThreadTasks = {let _arr = allThreadTasks.clone(); _arr.borrow_mut()[(threadId.clone()-1) as usize] = cons(newTask.clone(), threadTasks.clone()); _arr};
                    (allCalcTasks, tmpNodeList) = updateRefCounterBySuccessorIdc(allCalcTasks.clone(), successorIdc.clone(), metamodelica::nil())?;
                    tmpNodeList = listAppend(tmpNodeList.clone(), rest.clone());
                    tmpNodeList = List::sort(tmpNodeList.clone(), Arc::new(compareTasksByWeighting))?;
                    (_, newTaskRefCount) = allCalcTasks.clone().borrow()[(index.clone()-1) as usize].clone();
                    {let _arr = allCalcTasks.clone(); _arr.borrow_mut()[(index.clone()-1) as usize] = (newTask.clone(), newTaskRefCount.clone()); _arr};
                    tmpSchedule = createExtSchedule1(tmpNodeList.clone(), iThreadAssignments.clone(), iTaskGraph.clone(), iTaskGraphT.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone(), iLockWithPredecessorHandler.clone(), Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: allThreadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() }))?;
                    Ok(tmpSchedule.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, _, _) => {
                    Ok(iSchedule.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("HpcOmScheduler.createExtSchedule1 failed. Tasks in List:\n")).clone());
                    printTaskList(iNodeList.clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oSchedule)
}

//---------------------------------
// Task Duplication-based Scheduler
//---------------------------------
pub fn TDS_schedule(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut numProc: i32, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut iSimCode: SimCode::SimCode) -> Result<(Arc<HpcOmSimCode::Schedule>, SimCode::SimCode, metamodelica::Array<Arc<metamodelica::List<i32>>>, HpcOmTaskGraph::TaskGraphMeta, metamodelica::Array<Arc<metamodelica::List<i32>>>)> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut oSimCode: SimCode::SimCode;
    let mut oTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut oTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta;
    let mut oSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut size: i32 = 0;
    let mut queue: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut levels: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut ectArray: metamodelica::Array<metamodelica::Real>;
    let mut tdsLevelArray: metamodelica::Array<metamodelica::Real>;
    let mut lastArray: metamodelica::Array<metamodelica::Real>;
    let mut lactArray: metamodelica::Array<metamodelica::Real>;
    let mut fpredArray: metamodelica::Array<i32>;
    let mut initClusters: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let HpcOmTaskGraph::TASKGRAPHMETA { inComps: __pa0, commCosts: __pa1, .. } = (iTaskGraphMeta.clone()) else { bail!("pattern mismatch") };
    inComps = __pa0.clone();
    commCosts = __pa1.clone();
    size = (iTaskGraph.clone().borrow().len() as i32);
    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), size.clone())?;
    (_, _, ectArray) = computeGraphValuesBottomUp(iTaskGraph.clone(), iTaskGraphMeta.clone())?;
    (_, lastArray, lactArray, tdsLevelArray) = computeGraphValuesTopDown(iTaskGraph.clone(), iTaskGraphMeta.clone())?;
    fpredArray = computeFavouritePred(iTaskGraph.clone(), iTaskGraphMeta.clone(), ectArray.clone())?;
    (levels, queue) = quicksortWithOrder(Arc::new(tdsLevelArray.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()))?;
    initClusters = TDS_InitialCluster(iTaskGraph.clone(), taskGraphT.clone(), iTaskGraphMeta.clone(), lastArray.clone(), lactArray.clone(), fpredArray.clone(), queue.clone())?;
    (oSchedule, oSimCode, oTaskGraph, oTaskGraphMeta, oSccSimEqMapping) = TDS_schedule1(initClusters.clone(), iTaskGraph.clone(), taskGraphT.clone(), iTaskGraphMeta.clone(), tdsLevelArray.clone(), numProc.clone(), iSccSimEqMapping.clone(), iSimCode.clone(), commCosts.clone(), inComps.clone(), iSimVarMapping.clone())?;
    Ok((oSchedule, oSimCode, oTaskGraph, oTaskGraphMeta, oSccSimEqMapping))
}

fn insertLocksInSchedule(mut iSchedule: Arc<HpcOmSimCode::Schedule>, mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskAss: metamodelica::Array<i32>, mut procAss: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<Arc<HpcOmSimCode::Schedule>> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut threads: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
    let mut outgoingDepTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(iSchedule.clone()) {
        Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { allCalcTasks: __pa0, threadTasks: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    allCalcTasks = __pa0.clone();
    threadTasks = __pa1.clone();
    threads = Arc::new(threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
    (threads, outgoingDepTasks) = List::fold(threads.clone(), Arc::new({ let __pe_b1 = (iTaskGraph.clone(), iTaskGraphT.clone()); let __pe_b2 = (taskAss.clone(), procAss.clone()); let __pe_b3 = allCalcTasks.clone(); let __pe_b4 = iCommCosts.clone(); let __pe_b5 = iCompTaskMapping.clone(); let __pe_b6 = iSimVarMapping.clone(); move |__pe_a0, __pe_a7| insertLocksInSchedule1(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone(), __pe_a7) }), (metamodelica::nil(), metamodelica::nil()));
    threads = List::filterOnFalse(threads.clone(), Arc::new(listEmpty));
    threads = List::map(threads.clone(), Arc::new(listReverse.clone()));
    threads = threads.clone().reverse();
    threadTasks = metamodelica::arrayFromVec(threads.clone().into_iter().cloned().collect());
    outgoingDepTasks = List::unique(outgoingDepTasks.clone());
    oSchedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
    Ok(oSchedule)
}

fn insertLocksInSchedule1(mut threadsIn: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut iTaskGraphTransposed: (metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>), mut taskProcAss: (metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>), mut iAllCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut foldIn: (Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)> {
    let mut foldOut: (Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>);
    foldOut = (::match_deref::match_deref! { match &((threadsIn.clone(), iTaskGraphTransposed.clone(), taskProcAss.clone(), iAllCalcTasks.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone(), foldIn.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, (threads, outgoingDepTasks)) => {
            let mut threads = (*threads).clone();
            threads = cons(metamodelica::nil(), threads.clone());
            (threads.clone(), outgoingDepTasks.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ HpcOmSimCode::Task::CALCTASK { threadIdx: thr, index: idx, .. }, tail: rest }, (iTaskGraph, iTaskGraphT), (taskAss, _), _, _, _, _, (threads, outgoingDepTasks)) => {
            let mut preds: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut succs: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut predThr: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut succThr: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut thread: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
            let mut relLocks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
            let mut assLocks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
            let mut tasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
            let mut task: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
            let mut threads = (*threads).clone();
            let mut outgoingDepTasks = (*outgoingDepTasks).clone();
            task = listHead(threadsIn.clone())?;
            preds = iTaskGraphT.clone().borrow()[(idx.clone()-1) as usize].clone();
            succs = iTaskGraph.clone().borrow()[(idx.clone()-1) as usize].clone();
            predThr = List::map1(preds.clone(), Arc::new(fnptr!(Array::getIndexFirst, i32, _)), taskAss.clone());
            succThr = List::map1(succs.clone(), Arc::new(fnptr!(Array::getIndexFirst, i32, _)), taskAss.clone());
            (_, preds) = List::filter1OnTrueSync(predThr.clone(), Arc::new(fnptr!(intNe, i32, i32)), thr.clone(), preds.clone())?;
            (_, succs) = List::filter1OnTrueSync(succThr.clone(), Arc::new(fnptr!(intNe, i32, i32)), thr.clone(), succs.clone())?;
            assLocks = List::map6(preds.clone(), Arc::new(createDepTaskByTaskIdc), idx.clone(), iAllCalcTasks.clone(), false, iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone());
            relLocks = List::map6(succs.clone(), Arc::new(createDepTaskByTaskIdc), idx.clone(), iAllCalcTasks.clone(), true, iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone());
            tasks = listAppend(listAppend(relLocks.clone(), list![task.clone()]), assLocks.clone());
            thread = if (!(threads.clone().is_empty())) {listHead(threads.clone())?} else {metamodelica::nil()};
            thread = listAppend(tasks.clone(), thread.clone());
            threads = if (!(threads.clone().is_empty())) {List::replaceAt(thread.clone(), 1, threads.clone())?} else {list![thread.clone()]};
            outgoingDepTasks = listAppend(relLocks.clone(), outgoingDepTasks.clone());
            outgoingDepTasks = listAppend(assLocks.clone(), outgoingDepTasks.clone());
            (threads, outgoingDepTasks) = insertLocksInSchedule1(rest.clone(), iTaskGraphTransposed.clone(), taskProcAss.clone(), iAllCalcTasks.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone(), (threads.clone(), outgoingDepTasks.clone()))?;
            (threads.clone(), outgoingDepTasks.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(foldOut)
}

fn TDS_schedule1(mut clustersIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut TDSLevel: metamodelica::Array<metamodelica::Real>, mut numProc: i32, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimCode: SimCode::SimCode, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<(Arc<HpcOmSimCode::Schedule>, SimCode::SimCode, metamodelica::Array<Arc<metamodelica::List<i32>>>, HpcOmTaskGraph::TaskGraphMeta, metamodelica::Array<Arc<metamodelica::List<i32>>>)> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut oSimCode: SimCode::SimCode;
    let mut oTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut oTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta;
    let mut oSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    (oSchedule, oSimCode, oTaskGraph, oTaskGraphMeta, oSccSimEqMapping) = 'mc: {
        let __mc_input = (clustersIn.clone(), iTaskGraph.clone(), iTaskGraphT.clone(), iTaskGraphMeta.clone(), TDSLevel.clone(), numProc.clone(), iSccSimEqMapping.clone(), iSimCode.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, _, _) => {
                    let mut sccSimEqMap: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut clusters: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut schedule: Arc<HpcOmSimCode::Schedule>;
                    let mut taskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut meta: HpcOmTaskGraph::TaskGraphMeta;
                    let mut simCode: SimCode::SimCode;
                    let true = ((clustersIn.clone().len() as i32) < numProc.clone()) else { bail!("pattern mismatch") };
                    println!("{}", (literal!("There are less initial clusters than processors. we need duplication, but since this is a rare case, it is not done. Less processors are used.\n")).clone());
                    clusters = List::map(clustersIn.clone(), Arc::new(listReverse.clone()));
                    FlagsUtil::setConfigInt(Flags::NUM_PROC.clone(), (clustersIn.clone().len() as i32))?;
                    (schedule, simCode, taskGraph, meta, sccSimEqMap) = TDS_schedule1(clusters.clone(), iTaskGraph.clone(), iTaskGraphT.clone(), iTaskGraphMeta.clone(), TDSLevel.clone(), (clustersIn.clone().len() as i32), iSccSimEqMapping.clone(), iSimCode.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone())?;
                    Ok((schedule.clone(), simCode.clone(), taskGraph.clone(), meta.clone(), sccSimEqMap.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, _, _) => {
                    let mut sccSimEqMap: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut clusters: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut schedule: Arc<HpcOmSimCode::Schedule>;
                    let mut taskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut meta: HpcOmTaskGraph::TaskGraphMeta;
                    let mut simCode: SimCode::SimCode;
                    let true = ((clustersIn.clone().len() as i32) > numProc.clone()) else { bail!("pattern mismatch") };
                    clusters = TDS_CompactClusters(clustersIn.clone(), iTaskGraph.clone(), iTaskGraphMeta.clone(), TDSLevel.clone(), numProc.clone())?;
                    (schedule, simCode, taskGraph, meta, sccSimEqMap) = TDS_schedule1(clusters.clone(), iTaskGraph.clone(), iTaskGraphT.clone(), iTaskGraphMeta.clone(), TDSLevel.clone(), numProc.clone(), iSccSimEqMapping.clone(), iSimCode.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone())?;
                    Ok((schedule.clone(), simCode.clone(), taskGraph.clone(), meta.clone(), sccSimEqMap.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, _, _) => {
                    let mut sizeTasks: i32 = 0;
                    let mut numDupl: i32 = 0;
                    let mut threadIdx: i32 = 0;
                    let mut compIdx: i32 = 0;
                    let mut simVarIdx: i32 = 0;
                    let mut simEqSysIdx: i32 = 0;
                    let mut taskIdx: i32 = 0;
                    let mut lsIdx: i32 = 0;
                    let mut nlsIdx: i32 = 0;
                    let mut mIdx: i32 = 0;
                    let mut taskAss: metamodelica::Array<i32>;
                    let mut taskDuplAss: metamodelica::Array<i32>;
                    let mut nodeMark: metamodelica::Array<i32>;
                    let mut newIdxAss: metamodelica::Array<i32>;
                    let mut procAss: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut sccSimEqMap: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut comps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
                    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>;
                    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
                    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
                    let mut idcs: (i32, i32, i32, i32, i32, i32, i32, i32);
                    let mut compNames: metamodelica::Array<ArcStr>;
                    let mut compDescs: metamodelica::Array<ArcStr>;
                    let mut clusters: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut duplSccSimEqMap: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut duplComps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut schedule: Arc<HpcOmSimCode::Schedule>;
                    let mut taskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut meta: HpcOmTaskGraph::TaskGraphMeta;
                    let mut simCode: SimCode::SimCode;
                    let mut simVars: SimCodeVar::SimVars;
                    let mut algVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
                    let mut threadTask: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
                    let mut odes: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>>> = metamodelica::nil();
                    let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
                    let mut compParamMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut compInformations: metamodelica::Array<HpcOmTaskGraph::ComponentInfo>;
                    let true = ((clustersIn.clone().len() as i32) == numProc.clone()) else { bail!("pattern mismatch") };
                    clusters = List::map1(clustersIn.clone(), Arc::new(TDS_SortCompactClusters), TDSLevel.clone());
                    let SimCode::SIMCODE { odeEquations: __pa0, modelInfo: SimCode::MODELINFO { vars: __pa1, .. }, .. } = (iSimCode.clone()) else { bail!("pattern mismatch") };
                    odes = __pa0.clone();
                    simVars = __pa1.clone();
                    let SimCodeVar::SIMVARS { algVars: __pa2, .. } = (simVars.clone()) else { bail!("pattern mismatch") };
                    algVars = __pa2.clone();
                    let HpcOmTaskGraph::TASKGRAPHMETA { compInformations: __pa3, nodeMark: __pa4, commCosts: __pa5, exeCosts: __pa6, compDescs: __pa7, compNames: __pa8, compParamMapping: __pa9, eqCompMapping: __pa10, varCompMapping: __pa11, inComps: __pa12 } = (iTaskGraphMeta.clone()) else { bail!("pattern mismatch") };
                    compInformations = __pa3.clone();
                    nodeMark = __pa4.clone();
                    commCosts = __pa5.clone();
                    exeCosts = __pa6.clone();
                    compDescs = __pa7.clone();
                    compNames = __pa8.clone();
                    compParamMapping = __pa9.clone();
                    eqCompMapping = __pa10.clone();
                    varCompMapping = __pa11.clone();
                    inComps = __pa12.clone();
                    sizeTasks = List::fold(List::map(clusters.clone(), Arc::new(fnptr!(listLength, _))), Arc::new(fnptr!(intAdd, i32, i32)), 0);
                    taskAss = arrayCreate(sizeTasks.clone(), -1);
                    procAss = arrayCreate((clusters.clone().len() as i32), metamodelica::nil());
                    taskGraph = arrayCreate(sizeTasks.clone(), metamodelica::nil());
                    taskDuplAss = arrayCreate(sizeTasks.clone(), -1);
                    threadTask = arrayCreate(numProc.clone(), metamodelica::nil());
                    allCalcTasks = arrayCreate(sizeTasks.clone(), (Arc::new(crate::HpcOmSimCode::Task::TASKEMPTY), 0));
                    schedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTask.clone(), outgoingDepTasks: metamodelica::nil(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
                    duplSccSimEqMap = metamodelica::nil();
                    duplComps = metamodelica::nil();
                    threadIdx = 1;
                    compIdx = (iSccSimEqMapping.clone().borrow().len() as i32) + 1;
                    taskIdx = (iTaskGraph.clone().borrow().len() as i32) + 1;
                    simVarIdx = {
        let mut __acc: Option<i32> = None;
        for mut v in (algVars.clone()).into_iter().cloned() {
                    let __x = v.index.clone();
                    __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    } + 1;
                    simEqSysIdx = SimCodeUtil::getMaxSimEqSystemIndex(iSimCode.clone())? + 1;
                    lsIdx = List::fold(List::map(List::flatten(odes.clone()), Arc::new(fnptr!(SimCodeUtil::getLSindex, Arc<SimCode::SimEqSystem>))), Arc::new(fnptr!(intMax, i32, i32)), 0) + 1;
                    nlsIdx = List::fold(List::map(List::flatten(odes.clone()), Arc::new(fnptr!(SimCodeUtil::getNLSindex, Arc<SimCode::SimEqSystem>))), Arc::new(fnptr!(intMax, i32, i32)), 0) + 1;
                    mIdx = List::fold(List::map(List::flatten(odes.clone()), Arc::new(fnptr!(SimCodeUtil::getMixedindex, Arc<SimCode::SimEqSystem>))), Arc::new(fnptr!(intMax, i32, i32)), 0) + 1;
                    (taskAss, procAss, taskGraph, taskDuplAss, idcs, simCode, schedule, duplSccSimEqMap, duplComps) = TDS_duplicateTasks(clusters.clone(), taskAss.clone(), procAss.clone(), (threadIdx.clone(), taskIdx.clone(), compIdx.clone(), simVarIdx.clone(), simEqSysIdx.clone(), lsIdx.clone(), nlsIdx.clone(), mIdx.clone()), iTaskGraph.clone(), iTaskGraphT.clone(), taskGraph.clone(), taskDuplAss.clone(), iTaskGraphMeta.clone(), iSimCode.clone(), schedule.clone(), iSccSimEqMapping.clone(), duplSccSimEqMap.clone(), duplComps.clone())?;
                    simCode = TDS_updateModelInfo(simCode.clone(), idcs.clone());
                    numDupl = List::fold(List::map(duplComps.clone(), Arc::new(fnptr!(listLength, _))), Arc::new(fnptr!(intAdd, i32, i32)), 0);
                    procAss = Array::map(procAss.clone(), Arc::new(listReverse.clone()));
                    sccSimEqMap = arrayAppend(iSccSimEqMapping.clone(), metamodelica::arrayFromVec(duplSccSimEqMap.clone().reverse().into_iter().cloned().collect()))?;
                    comps = arrayAppend(inComps.clone(), metamodelica::arrayFromVec(duplComps.clone().reverse().into_iter().cloned().collect()))?;
                    varCompMapping = arrayAppend(varCompMapping.clone(), arrayCreate(numDupl.clone(), (0, 0, 0)))?;
                    eqCompMapping = arrayAppend(eqCompMapping.clone(), arrayCreate(numDupl.clone(), (0, 0, 0)))?;
                    compParamMapping = arrayAppend(compParamMapping.clone(), arrayCreate(numDupl.clone(), metamodelica::nil()))?;
                    compNames = arrayAppend(compNames.clone(), arrayCreate(numDupl.clone(), (literal!("duplicated")).clone()))?;
                    compDescs = arrayAppend(compDescs.clone(), arrayCreate(numDupl.clone(), (literal!("duplicated")).clone()))?;
                    exeCosts = arrayAppend(exeCosts.clone(), arrayCreate(numDupl.clone(), (1, metamodelica::OrderedFloat(1.0_f64))))?;
                    nodeMark = arrayAppend(nodeMark.clone(), arrayCreate(numDupl.clone(), -1))?;
                    meta = HpcOmTaskGraph::TaskGraphMeta { inComps: comps.clone(), varCompMapping: varCompMapping.clone(), eqCompMapping: eqCompMapping.clone(), compParamMapping: compParamMapping.clone(), compNames: compNames.clone(), compDescs: compDescs.clone(), exeCosts: exeCosts.clone(), commCosts: commCosts.clone(), nodeMark: nodeMark.clone(), compInformations: compInformations.clone() };
                    newIdxAss = arrayCreate(SimCodeUtil::getMaxSimEqSystemIndex(simCode.clone())?, -1);
                    (simCode, newIdxAss) = TDS_assignNewSimEqSysIdxs(simCode.clone(), newIdxAss.clone());
                    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(taskGraph.clone(), (taskGraph.clone().borrow().len() as i32))?;
                    schedule = insertLocksInSchedule(schedule.clone(), taskGraph.clone(), taskGraphT.clone(), taskAss.clone(), procAss.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone())?;
                    schedule = TDS_replaceSimEqSysIdxsInSchedule(schedule.clone(), newIdxAss.clone())?;
                    Ok((schedule.clone(), simCode.clone(), taskGraph.clone(), meta.clone(), sccSimEqMap.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("TDS_schedule1 failed!\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((oSchedule, oSimCode, oTaskGraph, oTaskGraphMeta, oSccSimEqMapping))
}

fn TDS_replaceSimEqSysIdxsInSchedule(mut scheduleIn: Arc<HpcOmSimCode::Schedule>, mut assIn: metamodelica::Array<i32>) -> Result<Arc<HpcOmSimCode::Schedule>> {
    let mut scheduleOut: Arc<HpcOmSimCode::Schedule>;
    scheduleOut = (::match_deref::match_deref! { match &((scheduleIn.clone(), assIn.clone())) {
        (Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { allCalcTasks, scheduledTasks, outgoingDepTasks, threadTasks }, _) => {
            let mut scheduledTasks = (*scheduledTasks).clone();
            let mut threadTasks = (*threadTasks).clone();
            scheduledTasks = List::map1(scheduledTasks.clone(), Arc::new(TDS_replaceSimEqSysIdxsInTask), assIn.clone());
            threadTasks = Array::map1(threadTasks.clone(), Arc::new(fnptr!(TDS_replaceSimEqSysIdxsInTaskLst, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, metamodelica::Array<i32>)), assIn.clone())?;
            Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: scheduledTasks.clone(), allCalcTasks: allCalcTasks.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(scheduleOut)
}

fn TDS_replaceSimEqSysIdxsInTask(mut taskIn: Arc<HpcOmSimCode::Task>, mut assIn: metamodelica::Array<i32>) -> Result<Arc<HpcOmSimCode::Task>> {
    let mut taskOut: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    taskOut = 'mc: {
        let __mc_input = (taskIn.clone(), assIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ HpcOmSimCode::Task::CALCTASK { eqIdc, threadIdx, timeFinished, calcTime, index, weighting }, _) => {
                    let mut eqIdc = (*eqIdc).clone();
                    eqIdc = List::map1(eqIdc.clone(), Arc::new(fnptr!(Array::getIndexFirst, i32, _)), assIn.clone());
                    Ok(Arc::new(HpcOmSimCode::Task::CALCTASK { weighting: weighting.clone(), index: index.clone(), calcTime: calcTime.clone(), timeFinished: timeFinished.clone(), threadIdx: threadIdx.clone(), eqIdc: eqIdc.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(taskIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(taskOut)
}

fn TDS_replaceSimEqSysIdxsInTaskLst(mut taskLstIn: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut assIn: metamodelica::Array<i32>) -> Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> {
    let mut taskLstOut: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    taskLstOut = List::map1(taskLstIn.clone(), Arc::new(TDS_replaceSimEqSysIdxsInTask), assIn.clone());
    taskLstOut
}

fn TDS_assignNewSimEqSysIdxs(mut simCodeIn: SimCode::SimCode, mut idxAssIn: metamodelica::Array<i32>) -> (SimCode::SimCode, metamodelica::Array<i32>) {
    let mut simCodeOut: SimCode::SimCode = simCodeIn.clone();
    let mut idxAssOut: metamodelica::Array<i32>;
    let mut modelInfo: SimCode::ModelInfo;
    let mut varInfo: SimCode::VarInfo;
    let mut jacObts: Arc<metamodelica::List<Option<Arc<SimCode::JacobianMatrix>>>> = metamodelica::nil();
    let mut eqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    let mut idx: i32 = 0;
    let mut ass: metamodelica::Array<i32>;
    modelInfo = simCodeOut.modelInfo.clone();
    varInfo = modelInfo.varInfo.clone();
    let (__pa0, (__pa1, __pa2)) = List::mapFold(simCodeOut.initialEquations.clone(), Arc::new(TDS_replaceSimEqSysIndexWithUpdate), (1, idxAssIn.clone()));
    eqs = __pa0.clone();
    idx = __pa1.clone();
    ass = __pa2.clone();
    simCodeOut.initialEquations = eqs.clone();
    let (__pa3, (__pa4, __pa5)) = List::mapFold(simCodeOut.allEquations.clone(), Arc::new(TDS_replaceSimEqSysIndexWithUpdate), (idx.clone(), ass.clone()));
    eqs = __pa3.clone();
    idx = __pa4.clone();
    ass = __pa5.clone();
    simCodeOut.allEquations = eqs.clone();
    let (__pa6, (__pa7, __pa8)) = List::mapFold(simCodeOut.startValueEquations.clone(), Arc::new(TDS_replaceSimEqSysIndexWithUpdate), (idx.clone(), ass.clone()));
    eqs = __pa6.clone();
    idx = __pa7.clone();
    ass = __pa8.clone();
    simCodeOut.startValueEquations = eqs.clone();
    let (__pa9, (__pa10, __pa11)) = List::mapFold(simCodeOut.nominalValueEquations.clone(), Arc::new(TDS_replaceSimEqSysIndexWithUpdate), (idx.clone(), ass.clone()));
    eqs = __pa9.clone();
    idx = __pa10.clone();
    ass = __pa11.clone();
    simCodeOut.nominalValueEquations = eqs.clone();
    let (__pa12, (__pa13, __pa14)) = List::mapFold(simCodeOut.minValueEquations.clone(), Arc::new(TDS_replaceSimEqSysIndexWithUpdate), (idx.clone(), ass.clone()));
    eqs = __pa12.clone();
    idx = __pa13.clone();
    ass = __pa14.clone();
    simCodeOut.minValueEquations = eqs.clone();
    let (__pa15, (__pa16, __pa17)) = List::mapFold(simCodeOut.maxValueEquations.clone(), Arc::new(TDS_replaceSimEqSysIndexWithUpdate), (idx.clone(), ass.clone()));
    eqs = __pa15.clone();
    idx = __pa16.clone();
    ass = __pa17.clone();
    simCodeOut.maxValueEquations = eqs.clone();
    let (__pa18, (__pa19, __pa20)) = List::mapFold(simCodeOut.parameterEquations.clone(), Arc::new(TDS_replaceSimEqSysIndexWithUpdate), (idx.clone(), ass.clone()));
    eqs = __pa18.clone();
    idx = __pa19.clone();
    ass = __pa20.clone();
    simCodeOut.parameterEquations = eqs.clone();
    let (__pa21, (__pa22, __pa23)) = List::mapFold(simCodeOut.algorithmAndEquationAsserts.clone(), Arc::new(TDS_replaceSimEqSysIndexWithUpdate), (idx.clone(), ass.clone()));
    eqs = __pa21.clone();
    idx = __pa22.clone();
    ass = __pa23.clone();
    simCodeOut.algorithmAndEquationAsserts = eqs.clone();
    simCodeOut.odeEquations = List::map1List(simCodeOut.odeEquations.clone(), Arc::new(TDS_replaceSimEqSysIndex), ass.clone());
    simCodeOut.algebraicEquations = List::map1List(simCodeOut.algebraicEquations.clone(), Arc::new(TDS_replaceSimEqSysIndex), ass.clone());
    simCodeOut.equationsForZeroCrossings = List::map1(simCodeOut.equationsForZeroCrossings.clone(), Arc::new(TDS_replaceSimEqSysIndex), ass.clone());
    jacObts = List::map(simCodeOut.jacobianMatrices.clone(), Arc::new(fnptr!(Util::makeOption, _)));
    jacObts = List::map1(jacObts.clone(), Arc::new(TDS_replaceSimEqSysIdxInJacobianMatrix), ass.clone());
    simCodeOut.jacobianMatrices = List::map(jacObts.clone(), Arc::new(Util::getOption));
    varInfo.numEquations = idx.clone();
    modelInfo.varInfo = varInfo.clone();
    simCodeOut.modelInfo = modelInfo.clone();
    idxAssOut = ass.clone();
    (simCodeOut, idxAssOut)
}

fn TDS_replaceSimEqSysIndex(mut simEqIn: Arc<SimCode::SimEqSystem>, mut assIn: metamodelica::Array<i32>) -> Result<Arc<SimCode::SimEqSystem>> {
    let mut simEqOut: Arc<SimCode::SimEqSystem>;
    simEqOut = 'mc: {
        let __mc_input = simEqIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem: nlSystem @ Deref @ SimCode::NonlinearSystem { jacobianMatrix, eqs, .. }, .. } => {
                    let mut newIdx: i32 = 0;
                    let mut oldIdx: i32 = 0;
                    let mut simEqSys = (*simEqSys).clone();
                    let mut nlSystem = (*nlSystem).clone();
                    let mut jacobianMatrix = (*jacobianMatrix).clone();
                    let mut eqs = (*eqs).clone();
                    eqs = List::map1(eqs.clone(), Arc::new(TDS_replaceSimEqSysIndex), assIn.clone());
                    oldIdx = SimCodeUtil::simEqSystemIndex(simEqIn.clone())?;
                    newIdx = assIn.clone().borrow()[(oldIdx.clone()-1) as usize].clone();
                    jacobianMatrix = TDS_replaceSimEqSysIdxInJacobianMatrix(jacobianMatrix.clone(), assIn.clone())?;
                    todo!("unhandled field-assign shape: nlSystem.jacobianMatrix");
                    todo!("unhandled field-assign shape: nlSystem.index");
                    todo!("unhandled field-assign shape: nlSystem.eqs");
                    let __owned_variant_nlSystem_0 = nlSystem.clone();
                    if let SimCode::SimEqSystem::SES_NONLINEAR { nlSystem, .. } = &mut simEqSys {
                        *nlSystem = __owned_variant_nlSystem_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than SimCode::SimEqSystem::SES_NONLINEAR"); }
                    Ok(simEqSys.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem: lSystem @ Deref @ SimCode::LinearSystem { jacobianMatrix, residual: eqs, .. }, .. } => {
                    let mut newIdx: i32 = 0;
                    let mut oldIdx: i32 = 0;
                    let mut simEqSys = (*simEqSys).clone();
                    let mut lSystem = (*lSystem).clone();
                    let mut jacobianMatrix = (*jacobianMatrix).clone();
                    let mut eqs = (*eqs).clone();
                    eqs = List::map1(eqs.clone(), Arc::new(TDS_replaceSimEqSysIndex), assIn.clone());
                    oldIdx = SimCodeUtil::simEqSystemIndex(simEqIn.clone())?;
                    newIdx = assIn.clone().borrow()[(oldIdx.clone()-1) as usize].clone();
                    jacobianMatrix = TDS_replaceSimEqSysIdxInJacobianMatrix(jacobianMatrix.clone(), assIn.clone())?;
                    todo!("unhandled field-assign shape: lSystem.jacobianMatrix");
                    todo!("unhandled field-assign shape: lSystem.index");
                    todo!("unhandled field-assign shape: lSystem.residual");
                    let __owned_variant_lSystem_0 = lSystem.clone();
                    if let SimCode::SimEqSystem::SES_LINEAR { lSystem, .. } = &mut simEqSys {
                        *lSystem = __owned_variant_lSystem_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than SimCode::SimEqSystem::SES_LINEAR"); }
                    Ok(simEqSys.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut newIdx: i32 = 0;
                    let mut oldIdx: i32 = 0;
                    let mut simEqSys: Arc<SimCode::SimEqSystem>;
                    oldIdx = SimCodeUtil::simEqSystemIndex(simEqIn.clone())?;
                    newIdx = assIn.clone().borrow()[(oldIdx.clone()-1) as usize].clone();
                    simEqSys = SimCodeUtil::replaceSimEqSysIndex(simEqIn.clone(), newIdx.clone())?;
                    Ok(simEqSys.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(simEqOut)
}

fn TDS_replaceSimEqSysIndexWithUpdate(mut simEqIn: Arc<SimCode::SimEqSystem>, mut tplIn: (i32, metamodelica::Array<i32>)) -> Result<(Arc<SimCode::SimEqSystem>, (i32, metamodelica::Array<i32>))> {
    let mut simEqOut: Arc<SimCode::SimEqSystem>;
    let mut tplOut: (i32, metamodelica::Array<i32>);
    (simEqOut, tplOut) = 'mc: {
        let __mc_input = (simEqIn.clone(), tplIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (simEqSys @ Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem: nlSystem @ Deref @ SimCode::NonlinearSystem { jacobianMatrix, eqs, index: oldIdx, .. }, .. }, (newIdx, ass)) => {
                    let mut simEqSys = (*simEqSys).clone();
                    let mut nlSystem = (*nlSystem).clone();
                    let mut jacobianMatrix = (*jacobianMatrix).clone();
                    let mut eqs = (*eqs).clone();
                    let mut newIdx = (*newIdx).clone();
                    let mut ass = (*ass).clone();
                    let (__pa0, (__pa1, __pa2)) = List::mapFold(eqs.clone(), Arc::new(TDS_replaceSimEqSysIndexWithUpdate), (newIdx.clone(), ass.clone()));
                    eqs = __pa0.clone();
                    newIdx = __pa1.clone();
                    ass = __pa2.clone();
                    let (__pa3, (__pa4, __pa5)) = TDS_replaceSimEqSysIdxInJacobianMatrixWithUpdate(jacobianMatrix.clone(), (newIdx.clone(), ass.clone()))?;
                    jacobianMatrix = __pa3.clone();
                    newIdx = __pa4.clone();
                    ass = __pa5.clone();
                    ass = {let _arr = ass.clone(); _arr.borrow_mut()[(oldIdx.clone()-1) as usize] = newIdx.clone(); _arr};
                    todo!("unhandled field-assign shape: nlSystem.jacobianMatrix");
                    todo!("unhandled field-assign shape: nlSystem.index");
                    todo!("unhandled field-assign shape: nlSystem.eqs");
                    let __owned_variant_nlSystem_0 = nlSystem.clone();
                    if let SimCode::SimEqSystem::SES_NONLINEAR { nlSystem, .. } = &mut simEqSys {
                        *nlSystem = __owned_variant_nlSystem_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than SimCode::SimEqSystem::SES_NONLINEAR"); }
                    Ok((simEqSys.clone(), (newIdx.clone() + 1, ass.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (simEqSys @ Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem: lSystem @ Deref @ SimCode::LinearSystem { jacobianMatrix, residual: eqs, index: oldIdx, .. }, .. }, (newIdx, ass)) => {
                    let mut simEqSys = (*simEqSys).clone();
                    let mut lSystem = (*lSystem).clone();
                    let mut jacobianMatrix = (*jacobianMatrix).clone();
                    let mut eqs = (*eqs).clone();
                    let mut newIdx = (*newIdx).clone();
                    let mut ass = (*ass).clone();
                    let (__pa0, (__pa1, __pa2)) = List::mapFold(eqs.clone(), Arc::new(TDS_replaceSimEqSysIndexWithUpdate), (newIdx.clone(), ass.clone()));
                    eqs = __pa0.clone();
                    newIdx = __pa1.clone();
                    ass = __pa2.clone();
                    let (__pa3, (__pa4, __pa5)) = TDS_replaceSimEqSysIdxInJacobianMatrixWithUpdate(jacobianMatrix.clone(), (newIdx.clone(), ass.clone()))?;
                    jacobianMatrix = __pa3.clone();
                    newIdx = __pa4.clone();
                    ass = __pa5.clone();
                    ass = {let _arr = ass.clone(); _arr.borrow_mut()[(oldIdx.clone()-1) as usize] = newIdx.clone(); _arr};
                    todo!("unhandled field-assign shape: lSystem.jacobianMatrix");
                    todo!("unhandled field-assign shape: lSystem.index");
                    todo!("unhandled field-assign shape: lSystem.residual");
                    let __owned_variant_lSystem_0 = lSystem.clone();
                    if let SimCode::SimEqSystem::SES_LINEAR { lSystem, .. } = &mut simEqSys {
                        *lSystem = __owned_variant_lSystem_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than SimCode::SimEqSystem::SES_LINEAR"); }
                    Ok((simEqSys.clone(), (newIdx.clone() + 1, ass.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (simEqSys @ Deref @ SimCode::SimEqSystem::SES_MIXED { discEqs: eqs, cont, index: oldIdx, .. }, (newIdx, ass)) => {
                    let mut simEqSys = (*simEqSys).clone();
                    let mut eqs = (*eqs).clone();
                    let mut cont = (*cont).clone();
                    let mut newIdx = (*newIdx).clone();
                    let mut ass = (*ass).clone();
                    let (__pa0, (__pa1, __pa2)) = TDS_replaceSimEqSysIndexWithUpdate(cont.clone(), (newIdx.clone(), ass.clone()))?;
                    cont = __pa0.clone();
                    newIdx = __pa1.clone();
                    ass = __pa2.clone();
                    let (__pa3, (__pa4, __pa5)) = List::mapFold(eqs.clone(), Arc::new(TDS_replaceSimEqSysIndexWithUpdate), (newIdx.clone(), ass.clone()));
                    eqs = __pa3.clone();
                    newIdx = __pa4.clone();
                    ass = __pa5.clone();
                    ass = {let _arr = ass.clone(); _arr.borrow_mut()[(oldIdx.clone()-1) as usize] = newIdx.clone(); _arr};
                    let __owned_variant_cont_0 = cont.clone();
                    let __owned_variant_discEqs_1 = eqs.clone();
                    if let SimCode::SimEqSystem::SES_MIXED { cont, discEqs, .. } = &mut simEqSys {
                        *cont = __owned_variant_cont_0;
                        *discEqs = __owned_variant_discEqs_1;
                    } else { panic!("owned-variant field-assign: value held a different variant than SimCode::SimEqSystem::SES_MIXED"); }
                    Ok((simEqSys.clone(), (newIdx.clone() + 1, ass.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (newIdx, ass)) => {
                    let mut oldIdx: i32 = 0;
                    let mut simEqSys: Arc<SimCode::SimEqSystem>;
                    let mut ass = (*ass).clone();
                    oldIdx = SimCodeUtil::simEqSystemIndex(simEqIn.clone())?;
                    ass = {let _arr = ass.clone(); _arr.borrow_mut()[(oldIdx.clone()-1) as usize] = newIdx.clone(); _arr};
                    simEqSys = SimCodeUtil::replaceSimEqSysIndex(simEqIn.clone(), newIdx.clone())?;
                    Ok((simEqSys.clone(), (newIdx.clone() + 1, ass.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((simEqOut, tplOut))
}

fn TDS_replaceSimEqSysIdxInJacobianMatrixWithUpdate(mut jacIn: Option<Arc<SimCode::JacobianMatrix>>, mut tplIn: (i32, metamodelica::Array<i32>)) -> Result<(Option<Arc<SimCode::JacobianMatrix>>, (i32, metamodelica::Array<i32>))> {
    let mut jacOut: Option<Arc<SimCode::JacobianMatrix>> = None;
    let mut tplOut: (i32, metamodelica::Array<i32>);
    (jacOut, tplOut) = 'mc: {
        let __mc_input = (jacIn.clone(), tplIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(Deref @ SimCode::JacobianMatrix { columns: jacCols, seedVars: vars, matrixName: name, sparsity, sparsityT, nonlinear: nonlinearPat, nonlinearT: nonlinearPatT, coloredCols: colCols, coloredRows: colRows, maxColorCols: maxCol, jacobianIndex: jacIdx, partitionIndex: partIdx, generic_loop_calls: Deref @ metamodelica::List::Nil, crefsHT: crefToSimVarHTJacobian, isAdjoint: isAdj }), (newIdx, ass)) => {
                    let mut jacCols = (*jacCols).clone();
                    let mut newIdx = (*newIdx).clone();
                    let mut ass = (*ass).clone();
                    let (__pa0, (__pa1, __pa2)) = List::mapFold(jacCols.clone(), Arc::new(TDS_replaceSimEqSysIdxInJacobianColumnWithUpdate), (newIdx.clone(), ass.clone()));
                    jacCols = __pa0.clone();
                    newIdx = __pa1.clone();
                    ass = __pa2.clone();
                    Ok((Some(Arc::new(SimCode::JacobianMatrix { columns: jacCols.clone(), seedVars: vars.clone(), matrixName: (name.clone()).clone(), sparsity: sparsity.clone(), sparsityT: sparsityT.clone(), nonlinear: nonlinearPat.clone(), nonlinearT: nonlinearPatT.clone(), coloredCols: colCols.clone(), coloredRows: colRows.clone(), maxColorCols: maxCol.clone(), jacobianIndex: jacIdx.clone(), partitionIndex: partIdx.clone(), generic_loop_calls: metamodelica::nil(), crefsHT: crefToSimVarHTJacobian.clone(), isAdjoint: isAdj.clone() })), (newIdx.clone(), ass.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((jacIn.clone(), tplIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((jacOut, tplOut))
}

fn TDS_replaceSimEqSysIdxInJacobianColumnWithUpdate(mut jacIn: Arc<SimCode::JacobianColumn>, mut tplIn: (i32, metamodelica::Array<i32>)) -> Result<(Arc<SimCode::JacobianColumn>, (i32, metamodelica::Array<i32>))> {
    let mut jacOut: Arc<SimCode::JacobianColumn>;
    let mut tplOut: (i32, metamodelica::Array<i32>);
    (jacOut, tplOut) = 'mc: {
        let __mc_input = (jacIn.clone(), tplIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SimCode::JacobianColumn { columnEqns: simEqs, columnVars: simVars, numberOfResultVars: rowLen, constantEqns: constEqns }, (newIdx, ass)) => {
                    let mut simEqs = (*simEqs).clone();
                    let mut newIdx = (*newIdx).clone();
                    let mut ass = (*ass).clone();
                    let (__pa0, (__pa1, __pa2)) = List::mapFold(simEqs.clone(), Arc::new(TDS_replaceSimEqSysIndexWithUpdate), (newIdx.clone(), ass.clone()));
                    simEqs = __pa0.clone();
                    newIdx = __pa1.clone();
                    ass = __pa2.clone();
                    Ok((Arc::new(SimCode::JacobianColumn { columnEqns: simEqs.clone(), columnVars: simVars.clone(), numberOfResultVars: rowLen.clone(), constantEqns: constEqns.clone() }), (newIdx.clone(), ass.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((jacIn.clone(), tplIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((jacOut, tplOut))
}

fn TDS_replaceSimEqSysIdxInJacobianMatrix(mut jacIn: Option<Arc<SimCode::JacobianMatrix>>, mut assIn: metamodelica::Array<i32>) -> Result<Option<Arc<SimCode::JacobianMatrix>>> {
    let mut jacOut: Option<Arc<SimCode::JacobianMatrix>> = jacIn.clone();
    jacOut = 'mc: {
        let __mc_input = jacIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(jacMatrix @ Deref @ SimCode::JacobianMatrix { .. }) => {
                    let mut jacMatrix = (*jacMatrix).clone();
                    todo!("unhandled field-assign shape: jacMatrix.columns");
                    Ok(Some(jacMatrix.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(jacIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(jacOut)
}

fn TDS_replaceSimEqSysIdxInJacobianColumn(mut jacIn: Arc<SimCode::JacobianColumn>, mut assIn: metamodelica::Array<i32>) -> Arc<SimCode::JacobianColumn> {
    let mut jacOut: Arc<SimCode::JacobianColumn> = jacIn.clone();
    assign_field!(jacOut.columnEqns = List::map1(jacOut.columnEqns.clone(), Arc::new(TDS_replaceSimEqSysIndex), assIn.clone()));
    jacOut
}

fn TDS_updateModelInfo(mut simCodeIn: SimCode::SimCode, mut idcs: (i32, i32, i32, i32, i32, i32, i32, i32)) -> SimCode::SimCode {
    let mut simCodeOut: SimCode::SimCode = simCodeIn.clone();
    let mut lsIdx: i32 = 0;
    let mut nlsIdx: i32 = 0;
    let mut mIdx: i32 = 0;
    let mut modelInfo: SimCode::ModelInfo;
    let mut varInfo: SimCode::VarInfo;
    (_, _, _, _, _, lsIdx, nlsIdx, mIdx) = idcs.clone();
    modelInfo = simCodeIn.modelInfo.clone();
    varInfo = modelInfo.varInfo.clone();
    varInfo.numStateVars = (modelInfo.vars.stateVars.clone().len() as i32);
    varInfo.numAlgVars = (modelInfo.vars.algVars.clone().len() as i32);
    varInfo.numLinearSystems = if (intEq(varInfo.numLinearSystems.clone(), 0)) {0} else {lsIdx.clone()};
    varInfo.numNonLinearSystems = if (intEq(varInfo.numNonLinearSystems.clone(), 0)) {0} else {nlsIdx.clone()};
    modelInfo.varInfo = varInfo.clone();
    simCodeOut.modelInfo = modelInfo.clone();
    simCodeOut
}

fn TDS_duplicateTasks(mut clustersIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut taskAssIn: metamodelica::Array<i32>, mut procAssIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut idcsIn: (i32, i32, i32, i32, i32, i32, i32, i32), mut taskGraphOrig: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphTOrig: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskDuplAssIn: metamodelica::Array<i32>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut simCodeIn: SimCode::SimCode, mut scheduleIn: Arc<HpcOmSimCode::Schedule>, mut sccSimEqMappingIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut duplSccSimEqMapIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut duplCompsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<(metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, (i32, i32, i32, i32, i32, i32, i32, i32), SimCode::SimCode, Arc<HpcOmSimCode::Schedule>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)> {
    let mut taskAssOut: metamodelica::Array<i32>;
    let mut procAssOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut taskGraphOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut taskDuplAssOut: metamodelica::Array<i32>;
    let mut idcsOut: (i32, i32, i32, i32, i32, i32, i32, i32);
    let mut simCodeOut: SimCode::SimCode;
    let mut scheduleOut: Arc<HpcOmSimCode::Schedule>;
    let mut duplSccSimEqMapOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut duplCompsOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    (taskAssOut, procAssOut, taskGraphOut, taskDuplAssOut, idcsOut, simCodeOut, scheduleOut, duplSccSimEqMapOut, duplCompsOut) = (::match_deref::match_deref! { match &((clustersIn.clone(), taskAssIn.clone(), procAssIn.clone(), idcsIn.clone(), taskGraphOrig.clone(), taskGraphTOrig.clone(), taskGraphIn.clone(), taskDuplAssIn.clone(), iTaskGraphMeta.clone(), simCodeIn.clone(), scheduleIn.clone(), sccSimEqMappingIn.clone(), duplSccSimEqMapIn.clone(), duplCompsIn.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            (taskAssIn.clone(), procAssIn.clone(), taskGraphIn.clone(), taskDuplAssIn.clone(), idcsIn.clone(), simCodeIn.clone(), scheduleIn.clone(), duplSccSimEqMapIn.clone(), duplCompsIn.clone())
        },
        (Deref @ metamodelica::List::Cons { head: cluster, tail: rest }, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            let mut threadIdx: i32 = 0;
            let mut compIdx: i32 = 0;
            let mut simVarIdx: i32 = 0;
            let mut simEqSysIdx: i32 = 0;
            let mut taskIdx: i32 = 0;
            let mut lsIdx: i32 = 0;
            let mut nlsIdx: i32 = 0;
            let mut mIdx: i32 = 0;
            let mut duplSccSimEqMap: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut duplComps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut taskAss: metamodelica::Array<i32>;
            let mut taskDuplAss: metamodelica::Array<i32>;
            let mut procAss: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut idcs: (i32, i32, i32, i32, i32, i32, i32, i32);
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut simCode: SimCode::SimCode;
            let mut schedule: Arc<HpcOmSimCode::Schedule>;
            let mut taskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut thread: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
            let mut outgoingDepTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
            let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
            let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
            repl = BackendVarTransform::emptyReplacements();
            let (__pa0, __pa1, __pa2, __pa3, __pa4, (__pa5, __pa6, __pa7, __pa8, __pa9, __pa10, __pa11, __pa12), __pa13, __pa14, __pa15) = TDS_duplicateTasks1(cluster.clone(), clustersIn.clone(), repl.clone(), taskAssIn.clone(), procAssIn.clone(), metamodelica::nil(), idcsIn.clone(), taskGraphOrig.clone(), taskGraphTOrig.clone(), taskGraphIn.clone(), taskDuplAssIn.clone(), iTaskGraphMeta.clone(), simCodeIn.clone(), sccSimEqMappingIn.clone(), duplSccSimEqMapIn.clone(), duplCompsIn.clone())?;
            taskAss = __pa0.clone();
            procAss = __pa1.clone();
            taskGraph = __pa2.clone();
            taskDuplAss = __pa3.clone();
            thread = __pa4.clone();
            threadIdx = __pa5.clone();
            taskIdx = __pa6.clone();
            compIdx = __pa7.clone();
            simVarIdx = __pa8.clone();
            simEqSysIdx = __pa9.clone();
            lsIdx = __pa10.clone();
            nlsIdx = __pa11.clone();
            mIdx = __pa12.clone();
            simCode = __pa13.clone();
            duplSccSimEqMap = __pa14.clone();
            duplComps = __pa15.clone();
            let SimCode::SIMCODE { .. } = (simCode.clone()) else { bail!("pattern mismatch") };
            let (__pa16, __pa17, __pa18) = ::match_deref::match_deref! { match &(scheduleIn.clone()) {
                Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { allCalcTasks: __pa16, outgoingDepTasks: __pa17, threadTasks: __pa18, .. } => (__pa16.clone(), __pa17.clone(), __pa18.clone()),
                _ => bail!("pattern mismatch"),
            } };
            allCalcTasks = __pa16.clone();
            outgoingDepTasks = __pa17.clone();
            threadTasks = __pa18.clone();
            threadTasks = {let _arr = threadTasks.clone(); _arr.borrow_mut()[(threadIdx.clone()-1) as usize] = thread.clone().reverse(); _arr};
            schedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
            threadIdx = threadIdx.clone() + 1;
            (taskAss, procAss, taskGraph, taskDuplAss, idcs, simCode, schedule, duplSccSimEqMap, duplComps) = TDS_duplicateTasks(rest.clone(), taskAss.clone(), procAss.clone(), (threadIdx.clone(), taskIdx.clone(), compIdx.clone(), simVarIdx.clone(), simEqSysIdx.clone(), lsIdx.clone(), nlsIdx.clone(), mIdx.clone()), taskGraphOrig.clone(), taskGraphTOrig.clone(), taskGraph.clone(), taskDuplAss.clone(), iTaskGraphMeta.clone(), simCode.clone(), schedule.clone(), sccSimEqMappingIn.clone(), duplSccSimEqMap.clone(), duplComps.clone())?;
            (taskAssIn.clone(), procAssIn.clone(), taskGraph.clone(), taskDuplAss.clone(), idcs.clone(), simCode.clone(), schedule.clone(), duplSccSimEqMap.clone(), duplComps.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((taskAssOut, procAssOut, taskGraphOut, taskDuplAssOut, idcsOut, simCodeOut, scheduleOut, duplSccSimEqMapOut, duplCompsOut))
}

fn TDS_duplicateTasks1(mut clusterIn: Arc<metamodelica::List<i32>>, mut allCluster: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut replIn: BackendVarTransform::VariableReplacements, mut taskAssIn: metamodelica::Array<i32>, mut procAssIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut threadIn: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut idcsIn: (i32, i32, i32, i32, i32, i32, i32, i32), mut taskGraphOrig: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphTOrig: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskDuplAssIn: metamodelica::Array<i32>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut simCodeIn: SimCode::SimCode, mut sccSimEqMappingIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut duplSccSimEqMapIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut duplCompsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<(metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, (i32, i32, i32, i32, i32, i32, i32, i32), SimCode::SimCode, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)> {
    let mut taskAssOut: metamodelica::Array<i32>;
    let mut procAssOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut taskGraphOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut taskDuplAssOut: metamodelica::Array<i32>;
    let mut threadOut: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut idcsOut: (i32, i32, i32, i32, i32, i32, i32, i32);
    let mut simCodeOut: SimCode::SimCode;
    let mut duplSccSimEqMapOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut duplCompsOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    (taskAssOut, procAssOut, taskGraphOut, taskDuplAssOut, threadOut, idcsOut, simCodeOut, duplSccSimEqMapOut, duplCompsOut) = 'mc: {
        let __mc_input = (clusterIn.clone(), allCluster.clone(), replIn.clone(), taskAssIn.clone(), procAssIn.clone(), threadIn.clone(), idcsIn.clone(), taskGraphOrig.clone(), taskGraphTOrig.clone(), taskGraphIn.clone(), taskDuplAssIn.clone(), iTaskGraphMeta.clone(), simCodeIn.clone(), sccSimEqMappingIn.clone(), duplSccSimEqMapIn.clone(), duplCompsIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    Ok((taskAssIn.clone(), procAssIn.clone(), taskGraphIn.clone(), taskDuplAssIn.clone(), threadIn.clone(), idcsIn.clone(), simCodeIn.clone(), duplSccSimEqMapIn.clone(), duplCompsIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: node, tail: rest }, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    let mut ass: i32 = 0;
                    let mut duplSccSimEqMap: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut duplComps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut taskAss: metamodelica::Array<i32>;
                    let mut taskDuplAss: metamodelica::Array<i32>;
                    let mut procAss: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut idcs: (i32, i32, i32, i32, i32, i32, i32, i32);
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut taskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut simCode: SimCode::SimCode;
                    let mut thread: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
                    ass = taskAssIn.clone().borrow()[(node.clone()-1) as usize].clone();
                    let true = (intNe(ass.clone(), -1)) else { bail!("pattern mismatch") };
                    (repl, taskAss, procAss, taskGraph, taskDuplAss, thread, idcs, simCode, duplSccSimEqMap, duplComps) = TDS_duplicateTasks2(node.clone(), allCluster.clone(), replIn.clone(), taskAssIn.clone(), procAssIn.clone(), threadIn.clone(), idcsIn.clone(), taskGraphOrig.clone(), taskGraphTOrig.clone(), taskGraphIn.clone(), taskDuplAssIn.clone(), iTaskGraphMeta.clone(), simCodeIn.clone(), sccSimEqMappingIn.clone(), duplSccSimEqMapIn.clone(), duplCompsIn.clone())?;
                    (taskAss, procAss, taskGraph, taskDuplAss, thread, idcs, simCode, duplSccSimEqMap, duplComps) = TDS_duplicateTasks1(rest.clone(), allCluster.clone(), repl.clone(), taskAss.clone(), procAss.clone(), thread.clone(), idcs.clone(), taskGraphOrig.clone(), taskGraphTOrig.clone(), taskGraph.clone(), taskDuplAss.clone(), iTaskGraphMeta.clone(), simCode.clone(), sccSimEqMappingIn.clone(), duplSccSimEqMap.clone(), duplComps.clone())?;
                    Ok((taskAss.clone(), procAss.clone(), taskGraph.clone(), taskDuplAss.clone(), thread.clone(), idcs.clone(), simCode.clone(), duplSccSimEqMap.clone(), duplComps.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: node, tail: rest }, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    let mut ass: i32 = 0;
                    let mut threadIdx: i32 = 0;
                    let mut comps: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut simEqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut taskLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut origPredTasks: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut clPredTasks: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut duplPredTasks: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut clTasks: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut pos: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut duplSccSimEqMap: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut duplComps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut simEqsLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut taskAss: metamodelica::Array<i32>;
                    let mut taskDuplAss: metamodelica::Array<i32>;
                    let mut procAss: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut idcs: (i32, i32, i32, i32, i32, i32, i32, i32);
                    let mut task: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
                    let mut taskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut simCode: SimCode::SimCode;
                    let mut thread: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
                    let mut odes: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>>> = metamodelica::nil();
                    let mut simEqSysts: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
                    let mut allEqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
                    let mut taskGraphOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    ass = taskAssIn.clone().borrow()[(node.clone()-1) as usize].clone();
                    let true = (intEq(ass.clone(), -1)) else { bail!("pattern mismatch") };
                    (threadIdx, _, _, _, _, _, _, _) = idcsIn.clone();
                    let HpcOmTaskGraph::TASKGRAPHMETA { inComps: __pa0, .. } = (iTaskGraphMeta.clone()) else { bail!("pattern mismatch") };
                    inComps = __pa0.clone();
                    taskAss = {let _arr = taskAssIn.clone(); _arr.borrow_mut()[(node.clone()-1) as usize] = threadIdx.clone(); _arr};
                    taskLst = procAssIn.clone().borrow()[(threadIdx.clone()-1) as usize].clone();
                    procAss = {let _arr = procAssIn.clone(); _arr.borrow_mut()[(threadIdx.clone()-1) as usize] = cons(node.clone(), taskLst.clone()); _arr};
                    comps = inComps.clone().borrow()[(node.clone()-1) as usize].clone();
                    simEqsLst = List::map1(comps.clone(), Arc::new(fnptr!(Array::getIndexFirst, i32, _)), sccSimEqMappingIn.clone());
                    simEqs = List::flatten(simEqsLst.clone());
                    simEqs = simEqs.clone().reverse();
                    let SimCode::SIMCODE { allEquations: __pa1, odeEquations: __pa2, .. } = (simCodeIn.clone()) else { bail!("pattern mismatch") };
                    allEqs = __pa1.clone();
                    odes = __pa2.clone();
                    simEqSysts = List::map1(simEqs.clone(), Arc::new(SimCodeUtil::getSimEqSysForIndex), List::flatten(odes.clone()));
                    (simEqSysts, _) = replaceInSimEqSystemLst(simEqSysts.clone(), replIn.clone());
                    allEqs = replaceSimEqSystemLstWithSameIndex(simEqSysts.clone(), allEqs.clone());
                    odes = List::map1r(odes.clone(), Arc::new(fnptr!(replaceSimEqSystemLstWithSameIndex, Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>)), simEqSysts.clone());
                    simCode = SimCodeUtil::replaceODEandALLequations(allEqs.clone(), odes.clone(), simCodeIn.clone());
                    clTasks = listHead(allCluster.clone())?;
                    origPredTasks = taskGraphTOrig.clone().borrow()[(node.clone()-1) as usize].clone();
                    (clPredTasks, origPredTasks, _) = List::intersection1OnTrue(origPredTasks.clone(), clTasks.clone(), Arc::new(fnptr!(intEq, i32, i32)))?;
                    pos = List::map1(clPredTasks.clone(), Arc::new(List::position), clTasks.clone());
                    clTasks = procAssIn.clone().borrow()[(threadIdx.clone()-1) as usize].clone();
                    clTasks = clTasks.clone().reverse();
                    clPredTasks = List::map1(pos.clone(), Arc::new(fnptr!(List::getIndexFirst, i32, _)), clTasks.clone());
                    (duplPredTasks, _, _) = List::intersection1OnTrue(clPredTasks.clone(), clTasks.clone(), Arc::new(fnptr!(intEq, i32, i32)))?;
                    taskGraph = List::fold1(duplPredTasks.clone(), Arc::new(Array::appendToElement), list![node.clone()], taskGraphIn.clone());
                    taskGraphOut = List::fold1(origPredTasks.clone(), Arc::new(Array::appendToElement), list![node.clone()], taskGraph.clone());
                    task = Arc::new(HpcOmSimCode::Task::CALCTASK { weighting: 1, index: node.clone(), calcTime: metamodelica::OrderedFloat(0.0_f64), timeFinished: metamodelica::OrderedFloat(-1.0_f64), threadIdx: threadIdx.clone(), eqIdc: simEqs.clone() });
                    thread = cons(task.clone(), threadIn.clone());
                    taskDuplAss = {let _arr = taskDuplAssIn.clone(); _arr.borrow_mut()[(node.clone()-1) as usize] = node.clone(); _arr};
                    (taskAss, procAss, taskGraph, taskDuplAss, thread, idcs, simCode, duplSccSimEqMap, duplComps) = TDS_duplicateTasks1(rest.clone(), allCluster.clone(), replIn.clone(), taskAss.clone(), procAss.clone(), thread.clone(), idcsIn.clone(), taskGraphOrig.clone(), taskGraphTOrig.clone(), taskGraph.clone(), taskDuplAss.clone(), iTaskGraphMeta.clone(), simCode.clone(), sccSimEqMappingIn.clone(), duplSccSimEqMapIn.clone(), duplCompsIn.clone())?;
                    Ok((taskAss.clone(), procAss.clone(), taskGraph.clone(), taskDuplAss.clone(), thread.clone(), idcs.clone(), simCode.clone(), duplSccSimEqMap.clone(), duplComps.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((taskAssOut, procAssOut, taskGraphOut, taskDuplAssOut, threadOut, idcsOut, simCodeOut, duplSccSimEqMapOut, duplCompsOut))
}

fn TDS_duplicateTasks2(mut node: i32, mut allCluster: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut replIn: BackendVarTransform::VariableReplacements, mut taskAssIn: metamodelica::Array<i32>, mut procAssIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut threadIn: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut idcsIn: (i32, i32, i32, i32, i32, i32, i32, i32), mut taskGraphOrig: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphTOrig: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskDuplAssIn: metamodelica::Array<i32>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut simCodeIn: SimCode::SimCode, mut sccSimEqMappingIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut duplSccSimEqMapIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut duplCompsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<(BackendVarTransform::VariableReplacements, metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, (i32, i32, i32, i32, i32, i32, i32, i32), SimCode::SimCode, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)> {
    let mut replOut: BackendVarTransform::VariableReplacements;
    let mut taskAssOut: metamodelica::Array<i32>;
    let mut procAssOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut taskGraphOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut taskDuplAssOut: metamodelica::Array<i32>;
    let mut threadOut: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut idcsOut: (i32, i32, i32, i32, i32, i32, i32, i32);
    let mut simCodeOut: SimCode::SimCode;
    let mut duplSccSimEqMapOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut duplCompsOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut crefAppend: ArcStr = arcstr::literal!("");
    let mut threadIdx: i32 = 0;
    let mut compIdx: i32 = 0;
    let mut simVarIdx: i32 = 0;
    let mut simVarIdx2: i32 = 0;
    let mut simEqSysIdx: i32 = 0;
    let mut simEqSysIdx2: i32 = 0;
    let mut simEqSysIdx3: i32 = 0;
    let mut numVars: i32 = 0;
    let mut numEqs: i32 = 0;
    let mut numInitEqs: i32 = 0;
    let mut taskIdx: i32 = 0;
    let mut lsIdx: i32 = 0;
    let mut nlsIdx: i32 = 0;
    let mut mIdx: i32 = 0;
    let mut comps: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut simVarSysIdcs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut simVarSysIdcs2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut simEqSysIdcs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut simEqSysIdcs2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut systSimEqSysIdcs2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut simEqSysIdcsInit: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut thread: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut clTasks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut origPredTasks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut clPredTasks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut duplPredTasks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut pos: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut simEqIdxLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut simVarIdxLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut repl: BackendVarTransform::VariableReplacements;
    let mut taskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, SimCodeVar::SimVar)>>), i32, (HashTableCrefSimVar::FuncHashCref, HashTableCrefSimVar::FuncCrefEqual, HashTableCrefSimVar::FuncCrefStr, HashTableCrefSimVar::FuncExpStr));
    let mut modelinfo: SimCode::ModelInfo;
    let mut simVars: SimCodeVar::SimVars;
    let mut simCode: SimCode::SimCode;
    let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut crefsDupl: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut crefLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>> = metamodelica::nil();
    let mut crefsDuplExp: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut simVarLst: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut simVarDupl: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut algVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut simEqSysts: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    let mut simEqSystsDupl: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    let mut systemSimEqSys: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    let mut systemSimEqSysDupl: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    let mut initEqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    let mut odes: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>>> = metamodelica::nil();
    let HpcOmTaskGraph::TASKGRAPHMETA { inComps: __pa0, .. } = (iTaskGraphMeta.clone()) else { bail!("pattern mismatch") };
    inComps = __pa0.clone();
    let SimCode::SIMCODE { crefToSimVarHT: __pa1, odeEquations: __pa2, modelInfo: SimCode::MODELINFO { vars: __pa3, .. }, .. } = (simCodeIn.clone()) else { bail!("pattern mismatch") };
    ht = __pa1.clone();
    odes = __pa2.clone();
    simVars = __pa3.clone();
    (threadIdx, taskIdx, compIdx, simVarIdx, simEqSysIdx, lsIdx, nlsIdx, mIdx) = idcsIn.clone();
    comps = inComps.clone().borrow()[(node.clone()-1) as usize].clone();
    comps = comps.clone().reverse();
    simEqIdxLst = List::map1(comps.clone(), Arc::new(fnptr!(Array::getIndexFirst, i32, _)), sccSimEqMappingIn.clone());
    simEqSysIdcs = List::flatten(simEqIdxLst.clone());
    crefLst = List::map1(simEqSysIdcs.clone(), Arc::new(SimCodeUtil::getAssignedCrefsOfSimEq), simCodeIn.clone());
    crefs = List::flatten(crefLst.clone());
    simVarLst = List::map1(crefs.clone(), Arc::new(BaseHashTable::get), ht.clone());
    numVars = (simVarLst.clone().len() as i32);
    simVarSysIdcs2 = List::intRange2(simVarIdx.clone(), simVarIdx.clone() + numVars.clone() - 1);
    crefAppend = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("_thr")); __mm_s.push_str(&*intString(threadIdx.clone())); ArcStr::from(__mm_s) }).clone();
    crefsDupl = List::map1r(crefs.clone(), Arc::new(ComponentReference::appendStringLastIdent), (crefAppend.clone()).clone());
    crefsDuplExp = List::map(crefsDupl.clone(), Arc::new(Expression::crefExp));
    simVarDupl = List::threadMap(crefsDupl.clone(), simVarLst.clone(), Arc::new(SimCodeUtil::replaceSimVarName));
    simVarDupl = List::threadMap(simVarSysIdcs2.clone(), simVarDupl.clone(), Arc::new(fnptr!(SimCodeUtil::replaceSimVarIndex, i32, SimCodeVar::SimVar)));
    simCode = List::fold(simVarDupl.clone(), Arc::new(fnptr!(SimCodeUtil::addSimVarToAlgVars, SimCodeVar::SimVar, SimCode::SimCode)), simCodeIn.clone());
    simVarIdx2 = simVarIdx.clone() + numVars.clone();
    ht = List::fold(simVarDupl.clone(), Arc::new(HashTableCrefSimVar::addSimVarToHashTable), ht.clone());
    repl = BackendVarTransform::addReplacements(replIn.clone(), crefs.clone(), crefsDuplExp.clone(), None)?;
    simEqSysts = List::map1(simEqSysIdcs.clone(), Arc::new(SimCodeUtil::getSimEqSysForIndex), List::flatten(odes.clone()));
    numEqs = (simEqSysts.clone().len() as i32);
    simEqSysIdcs2 = List::intRange2(simEqSysIdx.clone(), simEqSysIdx.clone() + numEqs.clone() - 1);
    (simEqSystsDupl, _) = List::map1_2(simEqSysts.clone(), Arc::new(replaceExpsInSimEqSystem), repl.clone());
    let (__pa4, (__pa5, __pa6, __pa7)) = List::mapFold(simEqSystsDupl.clone(), Arc::new(fnptr!(replaceSystemIndex, Arc<SimCode::SimEqSystem>, (i32, i32, i32))), (lsIdx.clone(), nlsIdx.clone(), mIdx.clone()));
    simEqSystsDupl = __pa4.clone();
    lsIdx = __pa5.clone();
    nlsIdx = __pa6.clone();
    mIdx = __pa7.clone();
    simEqSystsDupl = List::threadMap(simEqSystsDupl.clone(), simEqSysIdcs2.clone(), Arc::new(SimCodeUtil::replaceSimEqSysIndex));
    simEqSysIdx2 = simEqSysIdx.clone() + numEqs.clone();
    (simEqSystsDupl, simEqSysIdx2) = TDS_duplicateSystemOfEquations(simEqSystsDupl.clone(), simEqSysIdx2.clone(), repl.clone(), metamodelica::nil())?;
    duplSccSimEqMapOut = listAppend(List::map(simEqSysIdcs2.clone(), Arc::new(fnptr!(List::create, _))), duplSccSimEqMapIn.clone());
    simCode = List::fold1(simEqSystsDupl.clone(), Arc::new(SimCodeUtil::addSimEqSysToODEquations), 1, simCode.clone());
    threadOut = cons(Arc::new(HpcOmSimCode::Task::CALCTASK { weighting: 1, index: taskIdx.clone(), calcTime: metamodelica::OrderedFloat(0.0_f64), timeFinished: metamodelica::OrderedFloat(-1.0_f64), threadIdx: threadIdx.clone(), eqIdc: simEqSysIdcs2.clone() }), threadIn.clone());
    numInitEqs = (crefs.clone().len() as i32);
    simEqSysIdcsInit = List::intRange2(simEqSysIdx2.clone(), simEqSysIdx2.clone() + numInitEqs.clone() - 1);
    initEqs = List::thread3Map(crefsDupl.clone(), crefs.clone(), simEqSysIdcsInit.clone(), Arc::new(makeSEScrefAssignment));
    simCode = List::fold(initEqs.clone(), Arc::new(fnptr!(SimCodeUtil::addSimEqSysToInitialEquations, Arc<SimCode::SimEqSystem>, SimCode::SimCode)), simCode.clone());
    simEqSysIdx3 = simEqSysIdx2.clone() + numInitEqs.clone();
    let SimCode::SIMCODE { odeEquations: __pa8, .. } = (simCode.clone()) else { bail!("pattern mismatch") };
    odes = __pa8.clone();
    taskAssOut = {let _arr = taskAssIn.clone(); _arr.borrow_mut()[(taskIdx.clone()-1) as usize] = threadIdx.clone(); _arr};
    thread = procAssIn.clone().borrow()[(threadIdx.clone()-1) as usize].clone();
    thread = cons(taskIdx.clone(), thread.clone());
    procAssOut = {let _arr = procAssIn.clone(); _arr.borrow_mut()[(threadIdx.clone()-1) as usize] = thread.clone(); _arr};
    comps = List::intRange2(compIdx.clone(), compIdx.clone() + (comps.clone().len() as i32) - 1);
    compIdx = compIdx.clone() + (comps.clone().len() as i32);
    duplCompsOut = cons(comps.clone(), duplCompsIn.clone());
    taskDuplAssOut = {let _arr = taskDuplAssIn.clone(); _arr.borrow_mut()[(taskIdx.clone()-1) as usize] = node.clone(); _arr};
    clTasks = listHead(allCluster.clone())?;
    origPredTasks = taskGraphTOrig.clone().borrow()[(node.clone()-1) as usize].clone();
    (clPredTasks, origPredTasks, _) = List::intersection1OnTrue(origPredTasks.clone(), clTasks.clone(), Arc::new(fnptr!(intEq, i32, i32)))?;
    pos = List::map1(clPredTasks.clone(), Arc::new(List::position), clTasks.clone());
    clTasks = procAssOut.clone().borrow()[(threadIdx.clone()-1) as usize].clone();
    clTasks = clTasks.clone().reverse();
    clPredTasks = List::map1(pos.clone(), Arc::new(fnptr!(List::getIndexFirst, i32, _)), clTasks.clone());
    (duplPredTasks, _, _) = List::intersection1OnTrue(clPredTasks.clone(), clTasks.clone(), Arc::new(fnptr!(intEq, i32, i32)))?;
    taskGraph = List::fold1(duplPredTasks.clone(), Arc::new(Array::appendToElement), list![taskIdx.clone()], taskGraphIn.clone());
    taskGraphOut = List::fold1(origPredTasks.clone(), Arc::new(Array::appendToElement), list![taskIdx.clone()], taskGraph.clone());
    idcsOut = (threadIdx.clone(), taskIdx.clone() + 1, compIdx.clone(), simVarIdx2.clone(), simEqSysIdx3.clone(), lsIdx.clone(), nlsIdx.clone(), mIdx.clone());
    simCodeOut = simCode.clone();
    replOut = repl.clone();
    Ok((replOut, taskAssOut, procAssOut, taskGraphOut, taskDuplAssOut, threadOut, idcsOut, simCodeOut, duplSccSimEqMapOut, duplCompsOut))
}

fn TDS_duplicateSystemOfEquations(mut simEqsIn: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut simEqSysIdxIn: i32, mut repl: BackendVarTransform::VariableReplacements, mut simEqsFold: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, i32)> {
    let mut simEqsOut: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    let mut simEqSysIdxOut: i32 = 0;
    (simEqsOut, simEqSysIdxOut) = 'mc: {
        let __mc_input = (simEqsIn.clone(), simEqSysIdxIn.clone(), repl.clone(), simEqsFold.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _) => {
                    Ok((simEqsFold.clone().reverse(), simEqSysIdxIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: simEqSys @ Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem: lSystem @ Deref @ SimCode::LinearSystem { residual, .. }, .. }, tail: rest }, _, _, _) => {
                    let mut simEqSysIdx: i32 = 0;
                    let mut numEqs: i32 = 0;
                    let mut systSimEqSysIdcs2: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut duplicated: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
                    let mut simEqSys = (*simEqSys).clone();
                    let mut lSystem = (*lSystem).clone();
                    numEqs = (residual.clone().len() as i32);
                    systSimEqSysIdcs2 = if (intEq(numEqs.clone(), 0)) {metamodelica::nil()} else {List::intRange2(simEqSysIdxIn.clone(), simEqSysIdxIn.clone() + numEqs.clone() - 1)};
                    (duplicated, _) = List::map1_2(residual.clone(), Arc::new(replaceExpsInSimEqSystem), repl.clone());
                    duplicated = List::threadMap(duplicated.clone(), systSimEqSysIdcs2.clone(), Arc::new(SimCodeUtil::replaceSimEqSysIndex));
                    todo!("unhandled field-assign shape: lSystem.residual");
                    let __owned_variant_lSystem_0 = lSystem.clone();
                    if let SimCode::SimEqSystem::SES_LINEAR { lSystem, .. } = &mut simEqSys {
                        *lSystem = __owned_variant_lSystem_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than SimCode::SimEqSystem::SES_LINEAR"); }
                    simEqSysIdx = simEqSysIdxIn.clone() + numEqs.clone();
                    (duplicated, simEqSysIdx) = TDS_duplicateSystemOfEquations(rest.clone(), simEqSysIdx.clone(), repl.clone(), cons(simEqSys.clone(), simEqsFold.clone()))?;
                    Ok((duplicated.clone(), simEqSysIdx.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut simEqSysIdx: i32 = 0;
                    let mut simEqSys: Arc<SimCode::SimEqSystem>;
                    let mut rest: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
                    let mut duplicated: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(simEqsIn.clone()) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    simEqSys = __pa0.clone();
                    rest = __pa1.clone();
                    (duplicated, simEqSysIdx) = TDS_duplicateSystemOfEquations(rest.clone(), simEqSysIdxIn.clone(), repl.clone(), cons(simEqSys.clone(), simEqsFold.clone()))?;
                    Ok((duplicated.clone(), simEqSysIdx.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((simEqsOut, simEqSysIdxOut))
}

fn makeSEScrefAssignment(mut lhs: Arc<DAE::ComponentRef>, mut rhs: Arc<DAE::ComponentRef>, mut idx: i32) -> Result<Arc<SimCode::SimEqSystem>> {
    let mut sesOut: Arc<SimCode::SimEqSystem>;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    ty = ComponentReference::crefType(rhs.clone())?;
    sesOut = Arc::new(SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { index: idx.clone(), cref: lhs.clone(), exp: Arc::new(DAE::Exp::CREF { componentRef: rhs.clone(), ty: ty.clone() }), source: DAE::emptyElementSource.clone(), eqAttr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() });
    Ok(sesOut)
}

fn replaceSimEqSystemLstWithSameIndex(mut eqSystsIn: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut eqSysLstIn: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> {
    let mut eqSysLstOut: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    eqSysLstOut = List::fold(eqSystsIn.clone(), Arc::new(replaceSimEqSystemWithSameIndex), eqSysLstIn.clone());
    eqSysLstOut
}

fn replaceSimEqSystemWithSameIndex(mut eqSysIn: Arc<SimCode::SimEqSystem>, mut eqSysLstIn: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>> {
    let mut eqSysLstOut: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    eqSysLstOut = 'mc: {
        let __mc_input = (eqSysIn.clone(), eqSysLstIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut pos: i32 = 0;
                    let mut eqSysLst: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
                    let _ = SimCodeUtil::simEqSystemIndex(eqSysIn.clone())?;
                    pos = List::position1OnTrue(eqSysLstIn.clone(), Arc::new(SimCodeUtil::equationIndexEqual), eqSysIn.clone());
                    eqSysLst = List::replaceAt(eqSysIn.clone(), pos.clone(), eqSysLstIn.clone())?;
                    Ok(eqSysLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(eqSysLstIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(eqSysLstOut)
}

fn replaceSystemIndex(mut simEqSysIn: Arc<SimCode::SimEqSystem>, mut idcsIn: (i32, i32, i32)) -> (Arc<SimCode::SimEqSystem>, (i32, i32, i32)) {
    let mut simEqSysOut: Arc<SimCode::SimEqSystem>;
    let mut idcsOut: (i32, i32, i32);
    (simEqSysOut, idcsOut) = (::match_deref::match_deref! { match &(simEqSysIn.clone()) {
        simEqSys @ Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem, .. } => {
            let mut lsIdx: i32 = 0;
            let mut nlsIdx: i32 = 0;
            let mut mIdx: i32 = 0;
            let mut simEqSys = (*simEqSys).clone();
            let mut lSystem = (*lSystem).clone();
            (lsIdx, nlsIdx, mIdx) = idcsIn.clone();
            assign_field!(lSystem.indexLinearSystem = lsIdx.clone());
            assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_LINEAR; lSystem = lSystem.clone());
            (simEqSys.clone(), (lsIdx.clone() + 1, nlsIdx.clone(), mIdx.clone()))
        },
        simEqSys @ Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem, .. } => {
            let mut lsIdx: i32 = 0;
            let mut nlsIdx: i32 = 0;
            let mut mIdx: i32 = 0;
            let mut simEqSys = (*simEqSys).clone();
            let mut nlSystem = (*nlSystem).clone();
            (lsIdx, nlsIdx, mIdx) = idcsIn.clone();
            assign_field!(nlSystem.indexNonLinearSystem = nlsIdx.clone());
            assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_NONLINEAR; nlSystem = nlSystem.clone());
            (simEqSys.clone(), (lsIdx.clone(), nlsIdx.clone() + 1, mIdx.clone()))
        },
        simEqSys @ Deref @ SimCode::SimEqSystem::SES_MIXED { .. } => {
            let mut lsIdx: i32 = 0;
            let mut nlsIdx: i32 = 0;
            let mut mIdx: i32 = 0;
            let mut simEqSys = (*simEqSys).clone();
            (lsIdx, nlsIdx, mIdx) = idcsIn.clone();
            assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_MIXED; indexMixedSystem = mIdx.clone());
            (simEqSys.clone(), (lsIdx.clone(), nlsIdx.clone(), mIdx.clone() + 1))
        },
        _ => {
            (simEqSysIn.clone(), idcsIn.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (simEqSysOut, idcsOut)
}

fn replaceInSimEqSystemLst(mut simEqSysLstIn: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut replIn: BackendVarTransform::VariableReplacements) -> (Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, Arc<metamodelica::List<bool>>) {
    let mut simEqSysLstOut: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    let mut changedOut: Arc<metamodelica::List<bool>> = metamodelica::nil();
    (simEqSysLstOut, changedOut) = List::map1_2(simEqSysLstIn.clone(), Arc::new(replaceExpsInSimEqSystem), replIn.clone());
    (simEqSysLstOut, changedOut)
}

fn replaceExpsInSimEqSystem(mut simEqSysIn: Arc<SimCode::SimEqSystem>, mut replIn: BackendVarTransform::VariableReplacements) -> Result<(Arc<SimCode::SimEqSystem>, bool)> {
    let mut simEqSysOut: Arc<SimCode::SimEqSystem>;
    let mut changedOut: bool = false;
    (simEqSysOut, changedOut) = 'mc: {
        let __mc_input = simEqSysIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_RESIDUAL { .. } => {
                    let mut changed: bool = false;
                    let mut exp: Arc<DAE::Exp>;
                    let mut simEqSys = (*simEqSys).clone();
                    (exp, changed) = BackendVarTransform::replaceExp(var_field!((**simEqSys).exp, SimCode::SimEqSystem::SES_RESIDUAL).clone(), replIn.clone(), None)?;
                    let __owned_variant_exp_0 = exp.clone();
                    if let SimCode::SimEqSystem::SES_RESIDUAL { exp, .. } = &mut simEqSys {
                        *exp = __owned_variant_exp_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than SimCode::SimEqSystem::SES_RESIDUAL"); }
                    Ok((simEqSys.clone(), changed.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { exp, cref, .. } => {
                    let mut changed: bool = false;
                    let mut hasRepl: bool = false;
                    let mut simEqSys = (*simEqSys).clone();
                    let mut exp = (*exp).clone();
                    let mut cref = (*cref).clone();
                    hasRepl = BackendVarTransform::hasReplacement(replIn.clone(), cref.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(if (hasRepl.clone()) {BackendVarTransform::getReplacement(replIn.clone(), cref.clone())?} else {Arc::new(DAE::Exp::CREF { componentRef: cref.clone(), ty: DAE::T_UNKNOWN_DEFAULT.clone() })}) {
                        Deref @ DAE::Exp::CREF { componentRef: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cref = __pa0.clone();
                    (exp, changed) = BackendVarTransform::replaceExp(exp.clone(), replIn.clone(), None)?;
                    let __owned_variant_cref_0 = cref.clone();
                    let __owned_variant_exp_1 = exp.clone();
                    if let SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { cref, exp, .. } = &mut simEqSys {
                        *cref = __owned_variant_cref_0;
                        *exp = __owned_variant_exp_1;
                    } else { panic!("owned-variant field-assign: value held a different variant than SimCode::SimEqSystem::SES_SIMPLE_ASSIGN"); }
                    Ok((simEqSys.clone(), changed.clone() || hasRepl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS { exp, cref, .. } => {
                    let mut changed: bool = false;
                    let mut hasRepl: bool = false;
                    let mut simEqSys = (*simEqSys).clone();
                    let mut exp = (*exp).clone();
                    let mut cref = (*cref).clone();
                    hasRepl = BackendVarTransform::hasReplacement(replIn.clone(), cref.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(if (hasRepl.clone()) {BackendVarTransform::getReplacement(replIn.clone(), cref.clone())?} else {Arc::new(DAE::Exp::CREF { componentRef: cref.clone(), ty: DAE::T_UNKNOWN_DEFAULT.clone() })}) {
                        Deref @ DAE::Exp::CREF { componentRef: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cref = __pa0.clone();
                    (exp, changed) = BackendVarTransform::replaceExp(exp.clone(), replIn.clone(), None)?;
                    let __owned_variant_cref_0 = cref.clone();
                    let __owned_variant_exp_1 = exp.clone();
                    if let SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS { cref, exp, .. } = &mut simEqSys {
                        *cref = __owned_variant_cref_0;
                        *exp = __owned_variant_exp_1;
                    } else { panic!("owned-variant field-assign: value held a different variant than SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS"); }
                    Ok((simEqSys.clone(), changed.clone() || hasRepl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN { exp, lhs, .. } => {
                    let mut changed: bool = false;
                    let mut hasRepl: bool = false;
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut simEqSys = (*simEqSys).clone();
                    let mut exp = (*exp).clone();
                    let mut lhs = (*lhs).clone();
                    cref = Expression::expCref(lhs.clone())?;
                    hasRepl = BackendVarTransform::hasReplacement(replIn.clone(), cref.clone());
                    lhs = if (hasRepl.clone()) {BackendVarTransform::getReplacement(replIn.clone(), cref.clone())?} else {Arc::new(DAE::Exp::CREF { componentRef: cref.clone(), ty: DAE::T_UNKNOWN_DEFAULT.clone() })};
                    (exp, changed) = BackendVarTransform::replaceExp(exp.clone(), replIn.clone(), None)?;
                    let __owned_variant_lhs_0 = lhs.clone();
                    let __owned_variant_exp_1 = exp.clone();
                    if let SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN { lhs, exp, .. } = &mut simEqSys {
                        *lhs = __owned_variant_lhs_0;
                        *exp = __owned_variant_exp_1;
                    } else { panic!("owned-variant field-assign: value held a different variant than SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN"); }
                    Ok((simEqSys.clone(), changed.clone() || hasRepl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_IFEQUATION { elsebranch, ifbranches: ifs, .. } => {
                    let mut changed: bool = false;
                    let mut bLst: Arc<metamodelica::List<bool>> = metamodelica::nil();
                    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut simEqSysLstLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>>> = metamodelica::nil();
                    let mut simEqSys = (*simEqSys).clone();
                    let mut elsebranch = (*elsebranch).clone();
                    let mut ifs = (*ifs).clone();
                    expLst = List::map(ifs.clone(), Arc::new(fnptr!(Util::tuple21, _)));
                    (expLst, changed) = BackendVarTransform::replaceExpList(expLst.clone(), replIn.clone(), None)?;
                    simEqSysLstLst = List::map(ifs.clone(), Arc::new(fnptr!(Util::tuple22, _)));
                    (simEqSysLstLst, _) = List::map1_2(simEqSysLstLst.clone(), Arc::new(fnptr!(replaceInSimEqSystemLst, Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, BackendVarTransform::VariableReplacements)), replIn.clone());
                    ifs = List::threadMap(expLst.clone(), simEqSysLstLst.clone(), Arc::new(fnptr!(Util::makeTuple, _, _)));
                    (elsebranch, bLst) = List::map1_2(elsebranch.clone(), Arc::new(replaceExpsInSimEqSystem), replIn.clone());
                    changed = List::fold(bLst.clone(), Arc::new(fnptr!(boolOr, bool, bool)), changed.clone());
                    let __owned_variant_ifbranches_0 = ifs.clone();
                    let __owned_variant_elsebranch_1 = elsebranch.clone();
                    if let SimCode::SimEqSystem::SES_IFEQUATION { ifbranches, elsebranch, .. } = &mut simEqSys {
                        *ifbranches = __owned_variant_ifbranches_0;
                        *elsebranch = __owned_variant_elsebranch_1;
                    } else { panic!("owned-variant field-assign: value held a different variant than SimCode::SimEqSystem::SES_IFEQUATION"); }
                    Ok((simEqSys.clone(), changed.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_ALGORITHM { statements: stmts, .. } => {
                    let mut changed: bool = false;
                    let mut simEqSys = (*simEqSys).clone();
                    let mut stmts = (*stmts).clone();
                    (stmts, changed) = BackendVarTransform::replaceStatementLst(stmts.clone(), replIn.clone(), None, metamodelica::nil(), false)?;
                    let __owned_variant_statements_0 = stmts.clone();
                    if let SimCode::SimEqSystem::SES_ALGORITHM { statements, .. } = &mut simEqSys {
                        *statements = __owned_variant_statements_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than SimCode::SimEqSystem::SES_ALGORITHM"); }
                    Ok((simEqSys.clone(), changed.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem, .. } => {
                    let mut changed: bool = false;
                    let mut bLst: Arc<metamodelica::List<bool>> = metamodelica::nil();
                    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut simVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
                    let mut simJac: Arc<metamodelica::List<(i32, i32, Arc<SimCode::SimEqSystem>)>> = metamodelica::nil();
                    let mut simEqSys = (*simEqSys).clone();
                    let mut lSystem = (*lSystem).clone();
                    (simVars, bLst) = List::map1_2(lSystem.vars.clone(), Arc::new(fnptr!(replaceCrefInSimVar, SimCodeVar::SimVar, BackendVarTransform::VariableReplacements)), replIn.clone());
                    (expLst, changed) = BackendVarTransform::replaceExpList(lSystem.beqs.clone(), replIn.clone(), None)?;
                    changed = List::fold(bLst.clone(), Arc::new(fnptr!(boolOr, bool, bool)), changed.clone());
                    simJac = List::map1(lSystem.simJac.clone(), Arc::new(replaceInSimJac), replIn.clone());
                    todo!("unhandled field-assign shape: lSystem.vars");
                    todo!("unhandled field-assign shape: lSystem.beqs");
                    todo!("unhandled field-assign shape: lSystem.simJac");
                    let __owned_variant_lSystem_0 = lSystem.clone();
                    if let SimCode::SimEqSystem::SES_LINEAR { lSystem, .. } = &mut simEqSys {
                        *lSystem = __owned_variant_lSystem_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than SimCode::SimEqSystem::SES_LINEAR"); }
                    Ok((simEqSys.clone(), changed.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem, .. } => {
                    let mut changed: bool = false;
                    let mut bLst: Arc<metamodelica::List<bool>> = metamodelica::nil();
                    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut simEqSysLst: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
                    let mut simEqSys = (*simEqSys).clone();
                    let mut nlSystem = (*nlSystem).clone();
                    expLst = List::map(nlSystem.crefs.clone(), Arc::new(Expression::crefExp));
                    (expLst, changed) = BackendVarTransform::replaceExpList(expLst.clone(), replIn.clone(), None)?;
                    crefs = List::map(expLst.clone(), Arc::new(Expression::expCref));
                    (simEqSysLst, bLst) = List::map1_2(nlSystem.eqs.clone(), Arc::new(replaceExpsInSimEqSystem), replIn.clone());
                    changed = changed.clone() || List::fold(bLst.clone(), Arc::new(fnptr!(boolOr, bool, bool)), false);
                    println!("{}", (literal!("implement Jacobian replacement for SES_NONLINEAR in HpcOmScheduler.replaceExpsInSimEqSystems!\n")).clone());
                    todo!("unhandled field-assign shape: nlSystem.crefs");
                    todo!("unhandled field-assign shape: nlSystem.eqs");
                    let __owned_variant_nlSystem_0 = nlSystem.clone();
                    if let SimCode::SimEqSystem::SES_NONLINEAR { nlSystem, .. } = &mut simEqSys {
                        *nlSystem = __owned_variant_nlSystem_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than SimCode::SimEqSystem::SES_NONLINEAR"); }
                    Ok((simEqSys.clone(), changed.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_MIXED { discEqs: simEqSysLst, discVars: simVars, cont, .. } => {
                    let mut changed: bool = false;
                    let mut bLst: Arc<metamodelica::List<bool>> = metamodelica::nil();
                    let mut simEqSys = (*simEqSys).clone();
                    let mut simEqSysLst = (*simEqSysLst).clone();
                    let mut simVars = (*simVars).clone();
                    let mut cont = (*cont).clone();
                    (cont, changed) = replaceExpsInSimEqSystem(cont.clone(), replIn.clone())?;
                    (simVars, bLst) = List::map1_2(simVars.clone(), Arc::new(fnptr!(replaceCrefInSimVar, SimCodeVar::SimVar, BackendVarTransform::VariableReplacements)), replIn.clone());
                    changed = List::fold(bLst.clone(), Arc::new(fnptr!(boolOr, bool, bool)), changed.clone());
                    (simEqSysLst, bLst) = List::map1_2(simEqSysLst.clone(), Arc::new(replaceExpsInSimEqSystem), replIn.clone());
                    changed = List::fold(bLst.clone(), Arc::new(fnptr!(boolOr, bool, bool)), changed.clone());
                    let __owned_variant_discVars_0 = simVars.clone();
                    let __owned_variant_discEqs_1 = simEqSysLst.clone();
                    let __owned_variant_cont_2 = cont.clone();
                    if let SimCode::SimEqSystem::SES_MIXED { discVars, discEqs, cont, .. } = &mut simEqSys {
                        *discVars = __owned_variant_discVars_0;
                        *discEqs = __owned_variant_discEqs_1;
                        *cont = __owned_variant_cont_2;
                    } else { panic!("owned-variant field-assign: value held a different variant than SimCode::SimEqSystem::SES_MIXED"); }
                    Ok((simEqSys.clone(), changed.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_WHEN { elseWhen: None, whenStmtLst: Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { source, right: exp, left: lhs }, tail: Deref @ metamodelica::List::Nil }, conditions: crefs, .. } => {
                    let mut changed: bool = false;
                    let mut changed1: bool = false;
                    let mut bLst: Arc<metamodelica::List<bool>> = metamodelica::nil();
                    let mut crefExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut simEqSys = (*simEqSys).clone();
                    let mut exp = (*exp).clone();
                    let mut lhs = (*lhs).clone();
                    let mut crefs = (*crefs).clone();
                    (crefExps, bLst) = List::map1_2(crefs.clone(), Arc::new(BackendVarTransform::replaceCref), replIn.clone());
                    crefs = List::map(crefExps.clone(), Arc::new(Expression::expCref));
                    (lhs, changed) = BackendVarTransform::replaceExp(lhs.clone(), replIn.clone(), None)?;
                    changed = List::fold(bLst.clone(), Arc::new(fnptr!(boolOr, bool, bool)), changed.clone());
                    (exp, changed1) = BackendVarTransform::replaceExp(exp.clone(), replIn.clone(), None)?;
                    changed = boolOr(changed.clone(), changed1.clone());
                    let __owned_variant_conditions_0 = crefs.clone();
                    let __owned_variant_whenStmtLst_1 = list![BackendDAE::WhenOperator::ASSIGN { left: lhs.clone(), right: exp.clone(), source: source.clone() }];
                    if let SimCode::SimEqSystem::SES_WHEN { conditions, whenStmtLst, .. } = &mut simEqSys {
                        *conditions = __owned_variant_conditions_0;
                        *whenStmtLst = __owned_variant_whenStmtLst_1;
                    } else { panic!("owned-variant field-assign: value held a different variant than SimCode::SimEqSystem::SES_WHEN"); }
                    Ok((simEqSys.clone(), changed.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_WHEN { elseWhen: Some(elseWhen), whenStmtLst: Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { source, right: exp, left: lhs }, tail: Deref @ metamodelica::List::Nil }, conditions: crefs, .. } => {
                    let mut changed: bool = false;
                    let mut changed1: bool = false;
                    let mut bLst: Arc<metamodelica::List<bool>> = metamodelica::nil();
                    let mut crefExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut simEqSys = (*simEqSys).clone();
                    let mut exp = (*exp).clone();
                    let mut lhs = (*lhs).clone();
                    let mut crefs = (*crefs).clone();
                    (crefExps, bLst) = List::map1_2(crefs.clone(), Arc::new(BackendVarTransform::replaceCref), replIn.clone());
                    crefs = List::map(crefExps.clone(), Arc::new(Expression::expCref));
                    (lhs, changed) = BackendVarTransform::replaceExp(lhs.clone(), replIn.clone(), None)?;
                    changed = List::fold(bLst.clone(), Arc::new(fnptr!(boolOr, bool, bool)), changed.clone());
                    (exp, changed1) = BackendVarTransform::replaceExp(exp.clone(), replIn.clone(), None)?;
                    changed = boolOr(changed.clone(), changed1.clone());
                    (simEqSys, changed1) = replaceExpsInSimEqSystem(simEqSys.clone(), replIn.clone())?;
                    changed = boolOr(changed.clone(), changed1.clone());
                    let __owned_variant_conditions_0 = crefs.clone();
                    let __owned_variant_whenStmtLst_1 = list![BackendDAE::WhenOperator::ASSIGN { left: lhs.clone(), right: exp.clone(), source: source.clone() }];
                    let __owned_variant_elseWhen_2 = Some(elseWhen.clone());
                    if let SimCode::SimEqSystem::SES_WHEN { conditions, whenStmtLst, elseWhen, .. } = &mut simEqSys {
                        *conditions = __owned_variant_conditions_0;
                        *whenStmtLst = __owned_variant_whenStmtLst_1;
                        *elseWhen = __owned_variant_elseWhen_2;
                    } else { panic!("owned-variant field-assign: value held a different variant than SimCode::SimEqSystem::SES_WHEN"); }
                    Ok((simEqSys.clone(), changed.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("replaceExpsInSimEqSystem failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((simEqSysOut, changedOut))
}

fn replaceCrefInSimVar(mut simVarIn: SimCodeVar::SimVar, mut replIn: BackendVarTransform::VariableReplacements) -> (SimCodeVar::SimVar, bool) {
    let mut simVarOut: SimCodeVar::SimVar = simVarIn.clone();
    let mut changedOut: bool = false;
    let mut name: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    match '__try0: {
        if BackendVarTransform::hasReplacement(replIn.clone(), simVarIn.name.clone()) {
            let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(BackendVarTransform::getReplacement(replIn.clone(), simVarIn.name.clone()), '__try0)) {
                Deref @ DAE::Exp::CREF { componentRef: __pa1, .. } => __pa1.clone(),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            name = __pa1.clone();
            simVarOut.name = name.clone();
            changedOut = true;
        } else {
            changedOut = false;
        }
        Ok::<_, anyhow::Error>((changedOut.clone(),))
    } {
        Ok((__try0_o0,)) => {
            changedOut = __try0_o0;
        }
        Err(_) => {
            changedOut = false;
        }
    }
    (simVarOut, changedOut)
}

fn replaceInSimJac(mut simJacRowIn: (i32, i32, Arc<SimCode::SimEqSystem>), mut replIn: BackendVarTransform::VariableReplacements) -> Result<(i32, i32, Arc<SimCode::SimEqSystem>)> {
    let mut simJacRowOut: (i32, i32, Arc<SimCode::SimEqSystem>);
    let mut int1: i32 = 0;
    let mut int2: i32 = 0;
    let mut simEqSys: Arc<SimCode::SimEqSystem>;
    (int1, int2, simEqSys) = simJacRowIn.clone();
    (simEqSys, _) = replaceExpsInSimEqSystem(simEqSys.clone(), replIn.clone())?;
    simJacRowOut = (int1.clone(), int2.clone(), simEqSys.clone());
    Ok(simJacRowOut)
}

fn TDS_getTaskAssignment(mut procIdx: i32, mut clusterArrayIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskAssIn: metamodelica::Array<i32>) -> Result<()> {
    let mut taskAss: metamodelica::Array<i32>;
    let mut procTasks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    procTasks = clusterArrayIn.clone().borrow()[(procIdx.clone()-1) as usize].clone();
    List::map2_0(procTasks.clone(), Arc::new(Array::updateIndexFirst), procIdx.clone(), taskAssIn.clone());
    Ok(())
}

fn TDS_CompactClusters(mut clustersIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut TDSLevel: metamodelica::Array<metamodelica::Real>, mut numProc: i32) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut clustersOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut numMergeClusters: i32 = 0;
    let mut clusterExeCosts: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut clusterOrder: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut firstClusters: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut lastClusters: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut middleCluster: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut clusters: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut mergedClusters: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    clusterExeCosts = List::map1(clustersIn.clone(), Arc::new(fnptr!(TDS_computeClusterCosts, Arc<metamodelica::List<i32>>, HpcOmTaskGraph::TaskGraphMeta)), iTaskGraphMeta.clone());
    (_, clusterOrder) = quicksortWithOrder(clusterExeCosts.clone())?;
    clusterOrder = clusterOrder.clone().reverse();
    clusters = List::map1(clusterOrder.clone(), Arc::new(fnptr!(List::getIndexFirst, i32, _)), clustersIn.clone());
    numMergeClusters = intMin(intDiv((clustersIn.clone().len() as i32), 2), intSub((clustersIn.clone().len() as i32), numProc.clone()));
    (firstClusters, lastClusters) = List::split(clusters.clone(), numMergeClusters.clone())?;
    (middleCluster, lastClusters) = List::split(lastClusters.clone(), intSub((lastClusters.clone().len() as i32), numMergeClusters.clone()))?;
    lastClusters = lastClusters.clone().reverse();
    mergedClusters = List::threadMap(firstClusters.clone(), lastClusters.clone(), Arc::new(listAppend.clone()));
    clustersOut = listAppend(mergedClusters.clone(), middleCluster.clone());
    Ok(clustersOut)
}

fn TDS_SortCompactClusters(mut clusterIn: Arc<metamodelica::List<i32>>, mut tdsLevelIn: metamodelica::Array<metamodelica::Real>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut clusterOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut order: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut cluster: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tdsLevels: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    cluster = List::unique(clusterIn.clone());
    tdsLevels = List::map1(cluster.clone(), Arc::new(fnptr!(Array::getIndexFirst, i32, _)), tdsLevelIn.clone());
    (_, order) = quicksortWithOrder(tdsLevels.clone())?;
    order = order.clone().reverse();
    clusterOut = List::map1(order.clone(), Arc::new(fnptr!(List::getIndexFirst, i32, _)), cluster.clone());
    Ok(clusterOut)
}

fn TDS_computeClusterCosts(mut clusters: Arc<metamodelica::List<i32>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta) -> metamodelica::Real {
    let mut costs: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut nodeCosts: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    nodeCosts = List::map1(clusters.clone(), Arc::new(HpcOmTaskGraph::getExeCostReqCycles), iTaskGraphMeta.clone());
    costs = List::fold(nodeCosts.clone(), Arc::new(fnptr!(realAdd, metamodelica::Real, metamodelica::Real)), metamodelica::OrderedFloat(0.0_f64));
    costs
}

fn TDS_InitialCluster(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut lastArrayIn: metamodelica::Array<metamodelica::Real>, mut lactArrayIn: metamodelica::Array<metamodelica::Real>, mut fpredArrayIn: metamodelica::Array<i32>, mut queue: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut clustersOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut taskAssignments: metamodelica::Array<i32>;
    let mut rootNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    taskAssignments = arrayCreate((iTaskGraph.clone().borrow().len() as i32), -1);
    rootNodes = HpcOmTaskGraph::getRootNodes(iTaskGraph.clone())?;
    clustersOut = TDS_InitialCluster1(iTaskGraph.clone(), iTaskGraphT.clone(), iTaskGraphMeta.clone(), lastArrayIn.clone(), lactArrayIn.clone(), fpredArrayIn.clone(), rootNodes.clone(), taskAssignments.clone(), 1, queue.clone(), list![metamodelica::nil()])?;
    Ok(clustersOut)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn TDS_InitialCluster1(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut lastArrayIn: metamodelica::Array<metamodelica::Real>, mut lactArrayIn: metamodelica::Array<metamodelica::Real>, mut fpredArrayIn: metamodelica::Array<i32>, mut rootNodes: Arc<metamodelica::List<i32>>, mut taskAssIn: metamodelica::Array<i32>, mut currThread: i32, mut queue: Arc<metamodelica::List<i32>>, mut clustersIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut clustersOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    clustersOut = 'mc: {
        let __mc_input = (iTaskGraph.clone(), iTaskGraphT.clone(), iTaskGraphMeta.clone(), lastArrayIn.clone(), lactArrayIn.clone(), fpredArrayIn.clone(), rootNodes.clone(), taskAssIn.clone(), currThread.clone(), queue.clone(), clustersIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, Deref @ metamodelica::List::Nil, _) => {
                    let mut clusters: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    clusters = List::filterOnFalse(clustersIn.clone(), Arc::new(listEmpty));
                    clusters = List::map(clusters.clone(), Arc::new(listReverse.clone()));
                    Ok(clusters.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, Deref @ metamodelica::List::Cons { head: front, tail: rest }, _) => {
                    let mut thread: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut clusters: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let true = (List::isMemberOnTrue(front.clone(), rootNodes.clone(), Arc::new(fnptr!(intEq, i32, i32)))) else { bail!("pattern mismatch") };
                    thread = (clustersIn.clone()).get(currThread.clone())?;
                    thread = cons(front.clone(), thread.clone());
                    clusters = List::replaceAt(thread.clone(), currThread.clone(), clustersIn.clone())?;
                    clusters = List::appendElt(metamodelica::nil(), clusters.clone());
                    clusters = TDS_InitialCluster1(iTaskGraph.clone(), iTaskGraphT.clone(), iTaskGraphMeta.clone(), lastArrayIn.clone(), lactArrayIn.clone(), fpredArrayIn.clone(), rootNodes.clone(), taskAssIn.clone(), currThread.clone() + 1, rest.clone(), clusters.clone())?;
                    Ok(clusters.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, Deref @ metamodelica::List::Cons { head: front, tail: rest }, _) => {
                    let mut isCritical: bool = false;
                    let mut fpred: i32 = 0;
                    let mut thread: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut clusters: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut rest = (*rest).clone();
                    fpred = fpredArrayIn.clone().borrow()[(front.clone()-1) as usize].clone();
                    isCritical = TDSpredIsCritical(front.clone(), fpred.clone(), iTaskGraphMeta.clone(), lastArrayIn.clone(), lactArrayIn.clone())?;
                    let true = (isCritical.clone()) else { bail!("pattern mismatch") };
                    thread = (clustersIn.clone()).get(currThread.clone())?;
                    thread = cons(front.clone(), thread.clone());
                    clusters = List::replaceAt(thread.clone(), currThread.clone(), clustersIn.clone())?;
                    {let _arr = taskAssIn.clone(); _arr.borrow_mut()[(front.clone()-1) as usize] = currThread.clone(); _arr};
                    rest = List::removeOnTrue(fpred.clone(), Arc::new(fnptr!(intEq, i32, i32)), rest.clone());
                    rest = cons(fpred.clone(), rest.clone());
                    clusters = TDS_InitialCluster1(iTaskGraph.clone(), iTaskGraphT.clone(), iTaskGraphMeta.clone(), lastArrayIn.clone(), lactArrayIn.clone(), fpredArrayIn.clone(), rootNodes.clone(), taskAssIn.clone(), currThread.clone(), rest.clone(), clusters.clone())?;
                    Ok(clusters.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, Deref @ metamodelica::List::Cons { head: front, tail: rest }, _) => {
                    let mut isCritical: bool = false;
                    let mut fpred: i32 = 0;
                    let mut pos: i32 = 0;
                    let mut maxExeCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut parentExeCost: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
                    let mut parents: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut parentsNofpred: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut parentAssgmnts: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut unAssParents: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut thread: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut clusters: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut rest = (*rest).clone();
                    fpred = fpredArrayIn.clone().borrow()[(front.clone()-1) as usize].clone();
                    isCritical = TDSpredIsCritical(front.clone(), fpred.clone(), iTaskGraphMeta.clone(), lastArrayIn.clone(), lactArrayIn.clone())?;
                    let true = (!(isCritical.clone())) else { bail!("pattern mismatch") };
                    thread = (clustersIn.clone()).get(currThread.clone())?;
                    thread = cons(front.clone(), thread.clone());
                    clusters = List::replaceAt(thread.clone(), currThread.clone(), clustersIn.clone())?;
                    {let _arr = taskAssIn.clone(); _arr.borrow_mut()[(front.clone()-1) as usize] = currThread.clone(); _arr};
                    parents = iTaskGraphT.clone().borrow()[(front.clone()-1) as usize].clone();
                    parentsNofpred = List::removeOnTrue(fpred.clone(), Arc::new(fnptr!(intEq, i32, i32)), parents.clone());
                    parentAssgmnts = List::map1(parentsNofpred.clone(), Arc::new(fnptr!(Array::getIndexFirst, i32, _)), taskAssIn.clone());
                    (_, unAssParents) = List::filter1OnTrueSync(parentAssgmnts.clone(), Arc::new(fnptr!(intEq, i32, i32)), -1, parentsNofpred.clone())?;
                    parents = if (unAssParents.clone().is_empty()) {parents.clone()} else {unAssParents.clone()};
                    parentExeCost = List::map1(parents.clone(), Arc::new(HpcOmTaskGraph::getExeCostReqCycles), iTaskGraphMeta.clone());
                    maxExeCost = List::fold(parentExeCost.clone(), Arc::new(fnptr!(realMax, metamodelica::Real, metamodelica::Real)), metamodelica::OrderedFloat(0.0_f64));
                    pos = List::position(maxExeCost.clone(), parentExeCost.clone())?;
                    fpred = (parents.clone()).get(pos.clone())?;
                    rest = List::removeOnTrue(fpred.clone(), Arc::new(fnptr!(intEq, i32, i32)), rest.clone());
                    rest = cons(fpred.clone(), rest.clone());
                    clusters = TDS_InitialCluster1(iTaskGraph.clone(), iTaskGraphT.clone(), iTaskGraphMeta.clone(), lastArrayIn.clone(), lactArrayIn.clone(), fpredArrayIn.clone(), rootNodes.clone(), taskAssIn.clone(), currThread.clone(), rest.clone(), clusters.clone())?;
                    Ok(clusters.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("TDS_InitialCluster1 failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(clustersOut)
}

fn TDSpredIsCritical(mut node: i32, mut pred: i32, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut lastArrayIn: metamodelica::Array<metamodelica::Real>, mut lactArrayIn: metamodelica::Array<metamodelica::Real>) -> Result<bool> {
    let mut isCritical: bool = false;
    let mut lastNode: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut lactPred: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut commCosts: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    lastNode = lastArrayIn.clone().borrow()[(node.clone()-1) as usize].clone();
    lactPred = lactArrayIn.clone().borrow()[(pred.clone()-1) as usize].clone();
    commCosts = HpcOmTaskGraph::getCommCostTimeBetweenNodes(pred.clone(), node.clone(), iTaskGraphMeta.clone())?;
    isCritical = (lastNode.clone()) - (lactPred.clone()) <= commCosts.clone();
    Ok(isCritical)
}

fn computeFavouritePred(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut ect: metamodelica::Array<metamodelica::Real>) -> Result<metamodelica::Array<i32>> {
    let mut fpredOut: metamodelica::Array<i32>;
    let mut size: i32 = 0;
    let mut fpred: metamodelica::Array<i32>;
    let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    size = (iTaskGraph.clone().borrow().len() as i32);
    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), size.clone())?;
    fpred = arrayCreate(size.clone(), -1);
    fpredOut = List::fold3(List::intRange(size.clone()), Arc::new(computeFavouritePred1), taskGraphT.clone(), iTaskGraphMeta.clone(), ect.clone(), fpred.clone());
    Ok(fpredOut)
}

fn computeFavouritePred1(mut nodeIdx: i32, mut graphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut ect: metamodelica::Array<metamodelica::Real>, mut fpredIn: metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> {
    let mut fpredOut: metamodelica::Array<i32>;
    fpredOut = 'mc: {
        let __mc_input = (nodeIdx.clone(), graphT.clone(), iTaskGraphMeta.clone(), ect.clone(), fpredIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut fpredPos: i32 = 0;
            let mut fpred: i32 = 0;
            let mut maxCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut parents: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut parentECTs: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut commCosts: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut costs: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut fpredOut: metamodelica::Array<i32>;
            parents = graphT.clone().borrow()[(nodeIdx.clone()-1) as usize].clone();
            let false = (parents.clone().is_empty()) else { bail!("pattern mismatch") };
            parentECTs = List::map1(parents.clone(), Arc::new(fnptr!(Array::getIndexFirst, i32, _)), ect.clone());
            commCosts = List::map2(parents.clone(), Arc::new(HpcOmTaskGraph::getCommCostTimeBetweenNodes), nodeIdx.clone(), iTaskGraphMeta.clone());
            costs = List::threadMap(parentECTs.clone(), commCosts.clone(), Arc::new(fnptr!(realAdd, metamodelica::Real, metamodelica::Real)));
            maxCost = List::fold(costs.clone(), Arc::new(fnptr!(realMax, metamodelica::Real, metamodelica::Real)), metamodelica::OrderedFloat(0.0_f64));
            fpredPos = List::position(maxCost.clone(), costs.clone())?;
            fpred = (parents.clone()).get(fpredPos.clone())?;
            fpredOut = {let _arr = fpredIn.clone(); _arr.borrow_mut()[(nodeIdx.clone()-1) as usize] = fpred.clone(); _arr};
            Ok(fpredOut.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut parents: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut fpredOut: metamodelica::Array<i32>;
            parents = graphT.clone().borrow()[(nodeIdx.clone()-1) as usize].clone();
            let true = (parents.clone().is_empty()) else { bail!("pattern mismatch") };
            fpredOut = {let _arr = fpredIn.clone(); _arr.borrow_mut()[(nodeIdx.clone()-1) as usize] = 0; _arr};
            Ok(fpredOut.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(fpredOut)
}

//---------------------------------
// Partition Scheduler
//---------------------------------
pub fn createPartSchedule(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut numProc: i32, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<Arc<HpcOmSimCode::Schedule>> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    oSchedule = 'mc: {
        let __mc_input = (iTaskGraph.clone(), iTaskGraphMeta.clone(), numProc.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, HpcOmTaskGraph::TaskGraphMeta { .. }, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut nTasks: i32 = 0;
            let mut rootNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut taskMap: metamodelica::Array<i32>;
            let mut partitions: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut partMap: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut graphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut threadTask: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
            let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
            let mut schedule: Arc<HpcOmSimCode::Schedule>;
            let mut order: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
            let true = (intNe((iTaskGraph.clone().borrow().len() as i32), 0)) else { bail!("pattern mismatch") };
            nTasks = (iTaskGraph.clone().borrow().len() as i32);
            rootNodes = HpcOmTaskGraph::getRootNodes(iTaskGraph.clone())?;
            partitions = arrayCreate(numProc.clone(), metamodelica::nil());
            taskMap = arrayCreate(nTasks.clone(), -1);
            partMap = arrayCreate((rootNodes.clone().len() as i32), metamodelica::nil());
            let _ = arrayCreate(numProc.clone(), metamodelica::OrderedFloat(0.0_f64));
            graphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), (iTaskGraph.clone().borrow().len() as i32))?;
            (taskMap, partMap, _) = List::fold1(rootNodes.clone(), Arc::new(assignPartitions), iTaskGraph.clone(), (taskMap.clone(), partMap.clone(), 1));
            (taskMap, partitions) = distributePartitions(taskMap.clone(), partMap.clone(), iTaskGraphMeta.clone(), numProc.clone())?;
            threadTask = arrayCreate(numProc.clone(), metamodelica::nil());
            allCalcTasks = convertTaskGraphToTasks(graphT.clone(), iTaskGraphMeta.clone(), Arc::new(convertNodeToTask))?;
            schedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTask.clone(), outgoingDepTasks: metamodelica::nil(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
            order = List::flatten(HpcOmTaskGraph::getLevelNodes(iTaskGraph.clone()));
            if List::isEqual(partitions.clone().borrow()[(1-1) as usize].clone(), list![20, 7, 15, 16, 2], true) {
                order = order.clone().reverse();
            }
            (oSchedule, _) = createScheduleFromAssignments(taskMap.clone(), partitions.clone(), Some(order.clone()), iTaskGraph.clone(), graphT.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone(), metamodelica::nil(), order.clone(), iSimVarMapping.clone(), schedule.clone())?;
            Ok(oSchedule.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let true = (intEq((iTaskGraph.clone().borrow().len() as i32), 0)) else { bail!("pattern mismatch") };
            Ok(Arc::new(HpcOmSimCode::Schedule::EMPTYSCHEDULE { tasks: HpcOmSimCode::TaskList::PARALLELTASKLIST { tasks: metamodelica::nil() } }))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                println!("{}", (literal!("HpcOmScheduler.createPartSchedule failed\n")).clone());
            }
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oSchedule)
}

fn distributePartitions(mut taskMapIn: metamodelica::Array<i32>, mut partMap: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut metaIn: HpcOmTaskGraph::TaskGraphMeta, mut n: i32) -> Result<(metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>)> {
    let mut taskMapOut: metamodelica::Array<i32>;
    let mut partitions: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut partIdx: i32 = 0;
    let mut costs: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut part: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut clusters: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut partCosts: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let __range0 = partMap.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut part in __range0 {
        costs = List::fold(List::map1(part.clone(), Arc::new(HpcOmTaskGraph::getExeCostReqCycles), metaIn.clone()), Arc::new(fnptr!(realAdd, metamodelica::Real, metamodelica::Real)), metamodelica::OrderedFloat(0.0_f64));
        partCosts = cons(costs.clone(), partCosts.clone());
    }
    partCosts = partCosts.clone().reverse();
    (partitions, _) = HpcOmTaskGraph::distributeToClusters(List::intRange((partMap.clone().borrow().len() as i32)), partCosts.clone(), n.clone())?;
    for mut partIdx in 1..=n.clone() {
        part = partitions.clone().borrow()[(partIdx.clone()-1) as usize].clone();
        clusters = List::map1(part.clone(), Arc::new(fnptr!(Array::getIndexFirst, i32, _)), partMap.clone());
        part = List::fold(clusters.clone(), Arc::new(listAppend.clone()), metamodelica::nil());
        partitions = {let _arr = partitions.clone(); _arr.borrow_mut()[(partIdx.clone()-1) as usize] = part.clone(); _arr};
        List::map2_0(part.clone(), Arc::new(Array::updateIndexFirst), partIdx.clone(), taskMapIn.clone());
    }
    taskMapOut = taskMapIn.clone();
    Ok((taskMapOut, partitions))
}

fn assignPartitions(mut rootNode: i32, mut graph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut tplIn: (metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>, i32)) -> Result<(metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>, i32)> {
    let mut tplOut: (metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>, i32);
    let mut node: i32 = 0;
    let mut idx: i32 = 0;
    let mut taskAss: metamodelica::Array<i32>;
    let mut partAss: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut nodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut successors: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut assParts: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut unassTasks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut otherParts: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut otherPartsTasks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (taskAss, partAss, idx) = tplIn.clone();
    taskAss = {let _arr = taskAss.clone(); _arr.borrow_mut()[(rootNode.clone()-1) as usize] = idx.clone(); _arr};
    partAss = Array::appendToElement(idx.clone(), list![rootNode.clone()], partAss.clone())?;
    nodes = list![rootNode.clone()];
    while !(nodes.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(nodes.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        node = __pa0.clone();
        nodes = __pa1.clone();
        successors = graph.clone().borrow()[(node.clone()-1) as usize].clone();
        (unassTasks, otherPartsTasks) = List::split1OnTrue(successors.clone(), Arc::new(isUnAssigned), taskAss.clone());
        otherParts = List::map1(otherPartsTasks.clone(), Arc::new(fnptr!(Array::getIndexFirst, i32, _)), taskAss.clone());
        (otherParts, otherPartsTasks) = List::filter1OnTrueSync(otherParts.clone(), Arc::new(fnptr!(intNe, i32, i32)), idx.clone(), otherPartsTasks.clone())?;
        otherParts = List::unique(otherParts.clone());
        if !(otherParts.clone().is_empty()) {
            (taskAss, _) = Array::mapNoCopy_1(taskAss.clone(), Arc::new(fnptr!(reassignPartitions, (i32, (Arc<metamodelica::List<i32>>, i32)))), (otherParts.clone(), idx.clone()));
            otherPartsTasks = List::fold(List::map1(otherParts.clone(), Arc::new(fnptr!(Array::getIndexFirst, i32, _)), partAss.clone()), Arc::new(listAppend.clone()), metamodelica::nil());
            List::map2_0(otherParts.clone(), Arc::new(Array::updateIndexFirst), metamodelica::nil(), partAss.clone());
            partAss = Array::appendToElement(idx.clone(), otherPartsTasks.clone(), partAss.clone())?;
        }
        List::map2_0(unassTasks.clone(), Arc::new(Array::updateIndexFirst), idx.clone(), taskAss.clone());
        partAss = Array::appendToElement(idx.clone(), unassTasks.clone(), partAss.clone())?;
        nodes = listAppend(unassTasks.clone(), nodes.clone());
    }
    tplOut = (taskAss.clone(), partAss.clone(), idx.clone() + 1);
    Ok(tplOut)
}

fn isUnAssigned(mut task: i32, mut ass: metamodelica::Array<i32>) -> Result<bool> {
    let mut isUnass: bool = false;
    let mut idx: i32 = 0;
    idx = ass.clone().borrow()[(task.clone()-1) as usize].clone();
    isUnass = intEq(idx.clone(), -1);
    Ok(isUnass)
}

fn reassignPartitions(mut tplIn: (i32, (Arc<metamodelica::List<i32>>, i32))) -> (i32, (Arc<metamodelica::List<i32>>, i32)) {
    let mut tplOut: (i32, (Arc<metamodelica::List<i32>>, i32));
    let mut value: i32 = 0;
    let mut newAss: i32 = 0;
    let mut oldAss: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let (__pa0, (__pa1, __pa2)) = tplIn.clone();
    value = __pa0.clone();
    oldAss = __pa1.clone();
    newAss = __pa2.clone();
    if List::exist1(oldAss.clone(), Arc::new(fnptr!(intEq, i32, i32)), value.clone()) {
        value = newAss.clone();
    }
    tplOut = (value.clone(), (oldAss.clone(), newAss.clone()));
    tplOut
}

//---------------------------------
// SingleThread Schedule
//---------------------------------
pub fn createSingleThreadSchedule(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut numProc: i32) -> Result<Arc<HpcOmSimCode::Schedule>> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut nTasks: i32 = 0;
    let mut size: i32 = 0;
    let mut order: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut allTasksLst: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut thread2TaskAss: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
    nTasks = (iTaskGraph.clone().borrow().len() as i32);
    size = (iTaskGraph.clone().borrow().len() as i32);
    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), size.clone())?;
    allCalcTasks = convertTaskGraphToTasks(taskGraphT.clone(), iTaskGraphMeta.clone(), Arc::new(convertNodeToTask))?;
    order = List::flatten(HpcOmTaskGraph::getLevelNodes(iTaskGraph.clone()));
    for mut i in &*order.clone() {
        let mut i = i.clone();
        allTasksLst = cons(setSimEqIdcsInTask(Util::tuple21(allCalcTasks.clone().borrow()[(i.clone()-1) as usize].clone()), iSccSimEqMapping.clone())?, allTasksLst.clone());
    }
    allTasksLst = allTasksLst.clone().reverse();
    allTasksLst = List::map1(allTasksLst.clone(), Arc::new(setThreadIdxInTask), 1);
    thread2TaskAss = arrayCreate(numProc.clone(), metamodelica::nil());
    thread2TaskAss = {let _arr = thread2TaskAss.clone(); _arr.borrow_mut()[(1-1) as usize] = allTasksLst.clone(); _arr};
    oSchedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: thread2TaskAss.clone(), outgoingDepTasks: metamodelica::nil(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
    Ok(oSchedule)
}

//---------------------------------
// Modified Critical Path Scheduler
//---------------------------------
pub fn createMCPschedule(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut numProc: i32, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<Arc<HpcOmSimCode::Schedule>> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut size: i32 = 0;
    let mut numSfLocks: i32 = 0;
    let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut alapArray: metamodelica::Array<metamodelica::Real>;
    let mut alapSorted: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut priorityLst: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut order: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut taskAss: metamodelica::Array<i32>;
    let mut procAss: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut schedule: Arc<HpcOmSimCode::Schedule>;
    let mut removeLocks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>;
    let mut threads: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut threadTask: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let HpcOmTaskGraph::TASKGRAPHMETA { inComps: __pa0, commCosts: __pa1, .. } = (iTaskGraphMeta.clone()) else { bail!("pattern mismatch") };
    inComps = __pa0.clone();
    commCosts = __pa1.clone();
    size = (iTaskGraph.clone().borrow().len() as i32);
    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), size.clone())?;
    (alapArray, _, _, _) = computeGraphValuesTopDown(iTaskGraph.clone(), iTaskGraphMeta.clone())?;
    (priorityLst, order) = quicksortWithOrder(Arc::new(alapArray.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()))?;
    (taskAss, procAss) = MCP_getTaskAssignment(order.clone(), alapArray.clone(), numProc.clone(), iTaskGraph.clone(), iTaskGraphMeta.clone())?;
    threadTask = arrayCreate(numProc.clone(), metamodelica::nil());
    allCalcTasks = convertTaskGraphToTasks(taskGraphT.clone(), iTaskGraphMeta.clone(), Arc::new(convertNodeToTask))?;
    schedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTask.clone(), outgoingDepTasks: metamodelica::nil(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
    removeLocks = metamodelica::nil();
    (schedule, removeLocks) = createScheduleFromAssignments(taskAss.clone(), procAss.clone(), Some(order.clone()), iTaskGraph.clone(), taskGraphT.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone(), removeLocks.clone(), order.clone(), iSimVarMapping.clone(), schedule.clone())?;
    numSfLocks = intDiv((removeLocks.clone().len() as i32), 2);
    if Flags::isSet(Flags::HPCOM_DUMP.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("number of removed superfluous locks: ")); __mm_s.push_str(&*intString(numSfLocks.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    schedule = traverseAndUpdateThreadsInSchedule(schedule.clone(), Arc::new(removeLocksFromThread), removeLocks.clone())?;
    schedule = updateLockIdcsInThreadschedule(schedule.clone(), Arc::new(removeLocksFromLockList), removeLocks.clone());
    oSchedule = setScheduleLockIds(schedule.clone())?;
    Ok(oSchedule)
}

fn MCP_getTaskAssignment(mut orderIn: Arc<metamodelica::List<i32>>, mut alapIn: metamodelica::Array<metamodelica::Real>, mut numProc: i32, mut taskGraphIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphMetaIn: HpcOmTaskGraph::TaskGraphMeta) -> Result<(metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>)> {
    let mut taskAssOut: metamodelica::Array<i32>;
    let mut procAssOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut processorTime: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut taskAss: metamodelica::Array<i32>;
    let mut procAss: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    processorTime = List::fill(metamodelica::OrderedFloat(0.0_f64), numProc.clone());
    taskAss = arrayCreate((orderIn.clone().len() as i32), 0);
    procAss = arrayCreate(numProc.clone(), metamodelica::nil());
    (taskAssOut, procAssOut) = MCP_getTaskAssignment1(orderIn.clone(), taskAss.clone(), procAss.clone(), processorTime.clone(), taskGraphIn.clone(), taskGraphMetaIn.clone())?;
    Ok((taskAssOut, procAssOut))
}

fn MCP_getTaskAssignment1(mut orderIn: Arc<metamodelica::List<i32>>, mut taskAssIn: metamodelica::Array<i32>, mut procAssIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut processorTimeIn: Arc<metamodelica::List<metamodelica::Real>>, mut taskGraphIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphMetaIn: HpcOmTaskGraph::TaskGraphMeta) -> Result<(metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>)> {
    let mut taskAssOut: metamodelica::Array<i32>;
    let mut procAssOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    (taskAssOut, procAssOut) = 'mc: {
        let __mc_input = (orderIn.clone(), taskAssIn.clone(), procAssIn.clone(), processorTimeIn.clone(), taskGraphIn.clone(), taskGraphMetaIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _) => {
                    Ok((taskAssIn.clone(), procAssIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: node, tail: rest }, _, _, _, _, _) => {
                    let mut processor: i32 = 0;
                    let mut eft: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut exeCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut newTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut taskLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut processorTime: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
                    let mut taskAss: metamodelica::Array<i32>;
                    let mut procAss: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    eft = List::fold(processorTimeIn.clone(), Arc::new(fnptr!(realMin, metamodelica::Real, metamodelica::Real)), (processorTimeIn.clone()).get(1)?);
                    processor = List::position(eft.clone(), processorTimeIn.clone())?;
                    taskAss = {let _arr = taskAssIn.clone(); _arr.borrow_mut()[(node.clone()-1) as usize] = processor.clone(); _arr};
                    taskLst = procAssIn.clone().borrow()[(processor.clone()-1) as usize].clone();
                    taskLst = cons(node.clone(), taskLst.clone());
                    procAss = {let _arr = procAssIn.clone(); _arr.borrow_mut()[(processor.clone()-1) as usize] = taskLst.clone(); _arr};
                    (_, exeCost) = HpcOmTaskGraph::getExeCost(node.clone(), taskGraphMetaIn.clone())?;
                    newTime = eft.clone() + exeCost.clone();
                    processorTime = List::replaceAt(newTime.clone(), processor.clone(), processorTimeIn.clone())?;
                    (taskAss, procAss) = MCP_getTaskAssignment1(rest.clone(), taskAss.clone(), procAss.clone(), processorTime.clone(), taskGraphIn.clone(), taskGraphMetaIn.clone())?;
                    Ok((taskAss.clone(), procAss.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("MCP_getTaskAssignment1 failed!\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((taskAssOut, procAssOut))
}

fn updateLockIdcsInThreadschedule<ArgType: Clone + 'static>(mut scheduleIn: Arc<HpcOmSimCode::Schedule>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, ArgType) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> + 'static>, mut extraArg: ArgType) -> Arc<HpcOmSimCode::Schedule> {
    pub type FuncType<ArgType: Clone> = fn(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, ArgType) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;

    let mut scheduleOut: Arc<HpcOmSimCode::Schedule>;
    scheduleOut = (::match_deref::match_deref! { match &((scheduleIn.clone(), inFunc.clone(), extraArg.clone())) {
        (Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { allCalcTasks, outgoingDepTasks, threadTasks, .. }, _, _) => {
            let mut schedule: Arc<HpcOmSimCode::Schedule>;
            let mut outgoingDepTasks = (*outgoingDepTasks).clone();
            outgoingDepTasks = inFunc(outgoingDepTasks.clone(), extraArg.clone()).unwrap();
            schedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
            schedule.clone()
        },
        _ => {
            println!("{}", (literal!("this is not a thread schedule!\n")).clone());
            scheduleIn.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    scheduleOut
}

fn traverseAndUpdateThreadsInSchedule<ArgType: Clone + 'static>(mut scheduleIn: Arc<HpcOmSimCode::Schedule>, mut funcIn: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, ArgType) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> + 'static>, mut extraArg: ArgType) -> Result<Arc<HpcOmSimCode::Schedule>> {
    pub type FuncType<ArgType: Clone> = fn(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, ArgType) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;

    let mut scheduleOut: Arc<HpcOmSimCode::Schedule>;
    scheduleOut = (::match_deref::match_deref! { match &((scheduleIn.clone(), funcIn.clone(), extraArg.clone())) {
        (Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { .. }, _, _) => {
            scheduleIn.clone()
        },
        (Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { allCalcTasks, outgoingDepTasks, threadTasks, .. }, _, _) => {
            let mut schedule: Arc<HpcOmSimCode::Schedule>;
            let mut threadTasks = (*threadTasks).clone();
            threadTasks = Array::map1(threadTasks.clone(), funcIn.clone(), extraArg.clone())?;
            schedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
            schedule.clone()
        },
        (Deref @ HpcOmSimCode::Schedule::EMPTYSCHEDULE { .. }, _, _) => {
            scheduleIn.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(scheduleOut)
}

fn createScheduleFromAssignments(mut taskAss: metamodelica::Array<i32>, mut procAss: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut orderOpt: Option<Arc<metamodelica::List<i32>>>, mut taskGraphIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphTIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphMetaIn: HpcOmTaskGraph::TaskGraphMeta, mut SccSimEqMappingIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut removeLocksIn: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut orderIn: Arc<metamodelica::List<i32>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut scheduleIn: Arc<HpcOmSimCode::Schedule>) -> Result<(Arc<HpcOmSimCode::Schedule>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)> {
    let mut scheduleOut: Arc<HpcOmSimCode::Schedule>;
    let mut removeLocksOut: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    (scheduleOut, removeLocksOut) = (::match_deref::match_deref! { match &((taskAss.clone(), procAss.clone(), orderOpt.clone(), taskGraphIn.clone(), taskGraphTIn.clone(), taskGraphMetaIn.clone(), SccSimEqMappingIn.clone(), removeLocksIn.clone(), orderIn.clone(), iSimVarMapping.clone(), scheduleIn.clone())) {
        (_, _, Some(Deref @ metamodelica::List::Nil), _, _, _, _, _, _, _, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { .. }) => {
            (scheduleIn.clone(), removeLocksIn.clone())
        },
        (_, _, Some(order), _, _, HpcOmTaskGraph::TaskGraphMeta { nodeMark, inComps, commCosts: inCommCosts, .. }, _, _, _, _, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { allCalcTasks, outgoingDepTasks, threadTasks, .. }) => {
            let mut node: i32 = 0;
            let mut proc: i32 = 0;
            let mut mark: i32 = 0;
            let mut numProc: i32 = 0;
            let mut exeCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rest: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut components: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut simEqIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut parentNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut childNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut sameProcTasks: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut otherParents: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut otherChildren: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut taskLst1: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
            let mut taskLst: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
            let mut taskLstAss: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
            let mut taskLstRel: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
            let mut removeLocks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
            let mut schedule: Arc<HpcOmSimCode::Schedule>;
            let mut task: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
            let mut outgoingDepTasks = (*outgoingDepTasks).clone();
            let mut threadTasks = (*threadTasks).clone();
            numProc = (procAss.clone().borrow().len() as i32);
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(order.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            node = __pa0.clone();
            rest = __pa1.clone();
            proc = taskAss.clone().borrow()[(node.clone()-1) as usize].clone();
            taskLst = threadTasks.clone().borrow()[(proc.clone()-1) as usize].clone();
            parentNodes = taskGraphTIn.clone().borrow()[(node.clone()-1) as usize].clone();
            childNodes = taskGraphIn.clone().borrow()[(node.clone()-1) as usize].clone();
            sameProcTasks = procAss.clone().borrow()[(proc.clone()-1) as usize].clone();
            (_, otherParents, _) = List::intersection1OnTrue(parentNodes.clone(), sameProcTasks.clone(), Arc::new(fnptr!(intEq, i32, i32)))?;
            (_, otherChildren, _) = List::intersection1OnTrue(childNodes.clone(), sameProcTasks.clone(), Arc::new(fnptr!(intEq, i32, i32)))?;
            removeLocks = getSuperfluousLocks(otherParents.clone(), node.clone(), taskAss.clone(), orderIn.clone(), numProc.clone(), allCalcTasks.clone(), inCommCosts.clone(), inComps.clone(), iSimVarMapping.clone(), removeLocksIn.clone())?;
            taskLstAss = List::map6(otherParents.clone(), Arc::new(createDepTaskByTaskIdc), node.clone(), allCalcTasks.clone(), false, inCommCosts.clone(), inComps.clone(), iSimVarMapping.clone());
            taskLstRel = List::map6(otherChildren.clone(), Arc::new(createDepTaskByTaskIdcR), node.clone(), allCalcTasks.clone(), true, inCommCosts.clone(), inComps.clone(), iSimVarMapping.clone());
            components = inComps.clone().borrow()[(node.clone()-1) as usize].clone();
            mark = nodeMark.clone().borrow()[(node.clone()-1) as usize].clone();
            (_, exeCost) = HpcOmTaskGraph::getExeCost(node.clone(), taskGraphMetaIn.clone())?;
            simEqIdc = List::map(List::map1(components.clone(), Arc::new(getSimEqSysIdxForComp), SccSimEqMappingIn.clone()), Arc::new(List::last));
            task = Arc::new(HpcOmSimCode::Task::CALCTASK { weighting: mark.clone(), index: node.clone(), calcTime: exeCost.clone(), timeFinished: metamodelica::OrderedFloat(-1.0_f64), threadIdx: proc.clone(), eqIdc: simEqIdc.clone() });
            taskLst1 = cons(task.clone(), taskLstRel.clone());
            taskLst1 = listAppend(taskLstAss.clone(), taskLst1.clone());
            taskLst1 = listAppend(taskLst.clone(), taskLst1.clone());
            threadTasks = {let _arr = threadTasks.clone(); _arr.borrow_mut()[(proc.clone()-1) as usize] = taskLst1.clone(); _arr};
            outgoingDepTasks = listAppend(outgoingDepTasks.clone(), taskLstAss.clone());
            schedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
            (schedule, removeLocks) = createScheduleFromAssignments(taskAss.clone(), procAss.clone(), Some(rest.clone()), taskGraphIn.clone(), taskGraphTIn.clone(), taskGraphMetaIn.clone(), SccSimEqMappingIn.clone(), removeLocks.clone(), orderIn.clone(), iSimVarMapping.clone(), schedule.clone())?;
            (schedule.clone(), removeLocks.clone())
        },
        (_, _, None, _, _, _, _, _, _, _, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { .. }) => {
            println!("{}", (literal!("createSchedulerFromAssignments failed.implement this!\n")).clone());
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((scheduleOut, removeLocksOut))
}

fn setSimEqIdcsInTask(mut taskIn: Arc<HpcOmSimCode::Task>, mut SccSimEqMappingIn: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<HpcOmSimCode::Task>> {
    let mut taskOut: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    taskOut = 'mc: {
        let __mc_input = taskIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ HpcOmSimCode::Task::CALCTASK { eqIdc, threadIdx, timeFinished, calcTime, index, weighting } => {
                    let mut eqIdc = (*eqIdc).clone();
                    eqIdc = List::flatten(List::map1(eqIdc.clone(), Arc::new(getSimEqSysIdxForComp), SccSimEqMappingIn.clone()));
                    Ok(Arc::new(HpcOmSimCode::Task::CALCTASK { weighting: weighting.clone(), index: index.clone(), calcTime: calcTime.clone(), timeFinished: timeFinished.clone(), threadIdx: threadIdx.clone(), eqIdc: eqIdc.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(taskIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(taskOut)
}

fn setThreadIdxInTask(mut taskIn: Arc<HpcOmSimCode::Task>, mut threadIdx: i32) -> Result<Arc<HpcOmSimCode::Task>> {
    let mut taskOut: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    taskOut = 'mc: {
        let __mc_input = taskIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ HpcOmSimCode::Task::CALCTASK { eqIdc, timeFinished, calcTime, index, weighting, .. } => {
                    Ok(Arc::new(HpcOmSimCode::Task::CALCTASK { weighting: weighting.clone(), index: index.clone(), calcTime: calcTime.clone(), timeFinished: timeFinished.clone(), threadIdx: threadIdx.clone(), eqIdc: eqIdc.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(taskIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(taskOut)
}

fn tasksEqual(mut task1: Arc<HpcOmSimCode::Task>, mut task2: Arc<HpcOmSimCode::Task>) -> bool {
    let mut isEqOut: bool = false;
    isEqOut = (::match_deref::match_deref! { match &((task1.clone(), task2.clone())) {
        (Deref @ HpcOmSimCode::Task::CALCTASK { index: id1, .. }, Deref @ HpcOmSimCode::Task::CALCTASK { index: id2, .. }) => {
            let mut isEq: bool = false;
            isEq = intEq(id1.clone(), id2.clone());
            isEq.clone()
        },
        (Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { nodeIdc: nodeIdc1, .. }, Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { nodeIdc: nodeIdc2, .. }) => {
            let mut isEq: bool = false;
            isEq = List::isEqual(nodeIdc1.clone(), nodeIdc2.clone(), true);
            isEq.clone()
        },
        (Deref @ HpcOmSimCode::Task::DEPTASK { targetTask: targetTask1, sourceTask: sourceTask1, .. }, Deref @ HpcOmSimCode::Task::DEPTASK { targetTask: targetTask2, sourceTask: sourceTask2, .. }) => {
            let mut isEq: bool = false;
            isEq = tasksEqual(sourceTask1.clone(), sourceTask2.clone());
            isEq = boolAnd(isEq.clone(), tasksEqual(targetTask1.clone(), targetTask2.clone()));
            isEq.clone()
        },
        (Deref @ HpcOmSimCode::Task::TASKEMPTY, Deref @ HpcOmSimCode::Task::TASKEMPTY) => {
            false
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isEqOut
}

fn removeLocksFromLockList(mut lockIdsIn: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut lockTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> {
    let mut lockIdsOut: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    (_, lockIdsOut, _) = List::intersection1OnTrue(lockIdsIn.clone(), lockTasks.clone(), Arc::new(fnptr!(tasksEqual, Arc<HpcOmSimCode::Task>, Arc<HpcOmSimCode::Task>)))?;
    Ok(lockIdsOut)
}

fn removeLocksFromThread(mut threadIn: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut lockLst: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> {
    let mut threadOut: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    (_, threadOut, _) = List::intersection1OnTrue(threadIn.clone(), lockLst.clone(), Arc::new(fnptr!(tasksEqual, Arc<HpcOmSimCode::Task>, Arc<HpcOmSimCode::Task>)))?;
    Ok(threadOut)
}

fn getSuperfluousLocks(mut otherParentsIn: Arc<metamodelica::List<i32>>, mut nodeIn: i32, mut taskAssIn: metamodelica::Array<i32>, mut orderIn: Arc<metamodelica::List<i32>>, mut numProc: i32, mut iAllCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut removeLocksIn: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> {
    let mut removeLocksOut: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut parentsOnThreads: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut otherParentsProcs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut lockCandidatesFlat: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut lockCandidates: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut removeLocks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut taskLstAss: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut taskLstRel: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    otherParentsProcs = List::map1(otherParentsIn.clone(), Arc::new(fnptr!(Array::getIndexFirst, i32, _)), taskAssIn.clone());
    parentsOnThreads = arrayCreate(numProc.clone(), metamodelica::nil());
    parentsOnThreads = List::fold1(List::intRange((otherParentsProcs.clone().len() as i32)), Arc::new(listIndecesForValues), otherParentsProcs.clone(), parentsOnThreads.clone());
    parentsOnThreads = Array::map1(parentsOnThreads.clone(), Arc::new(fnptr!(mapListGet, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)), otherParentsIn.clone())?;
    lockCandidates = List::filterOnTrue(Arc::new(parentsOnThreads.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), Arc::new(fnptr!(lengthNotOne, Arc<metamodelica::List<i32>>)));
    lockCandidates = List::map1(lockCandidates.clone(), Arc::new(fnptr!(removeLatestTaskFromList, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)), orderIn.clone());
    lockCandidatesFlat = List::flatten(lockCandidates.clone());
    taskLstAss = List::map6(lockCandidatesFlat.clone(), Arc::new(createDepTaskByTaskIdc), nodeIn.clone(), iAllCalcTasks.clone(), false, iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone());
    taskLstRel = List::map6(lockCandidatesFlat.clone(), Arc::new(createDepTaskByTaskIdc), nodeIn.clone(), iAllCalcTasks.clone(), true, iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone());
    removeLocks = listAppend(removeLocksIn.clone(), taskLstAss.clone());
    removeLocksOut = listAppend(removeLocks.clone(), taskLstRel.clone());
    Ok(removeLocksOut)
}

fn removeLatestTaskFromList(mut taskLstIn: Arc<metamodelica::List<i32>>, mut taskOrderIn: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<i32>> {
    let mut taskLstOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    taskLstOut = (::match_deref::match_deref! { match &((taskLstIn.clone(), taskOrderIn.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            taskLstIn.clone()
        },
        (_, _) => {
            let mut posInOrder: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut taskLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut latestTask: i32 = 0;
            posInOrder = List::map1(taskLstIn.clone(), Arc::new(List::position), taskOrderIn.clone());
            posInOrder = List::map1(posInOrder.clone(), Arc::new(fnptr!(intSub, i32, i32)), 1);
            latestTask = List::fold(posInOrder.clone(), Arc::new(fnptr!(intMax, i32, i32)), -1);
            latestTask = (taskOrderIn.clone()).get(latestTask.clone() + 1).unwrap();
            taskLst = List::removeOnTrue(latestTask.clone(), Arc::new(fnptr!(intEq, i32, i32)), taskLstIn.clone());
            taskLst.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    taskLstOut
}

fn lengthNotOne(mut lstIn: Arc<metamodelica::List<i32>>) -> bool {
    let mut b: bool = false;
    b = intNe((lstIn.clone().len() as i32), 1);
    b
}

fn mapListGet(mut mapLstIn: Arc<metamodelica::List<i32>>, mut argLst: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<i32>> {
    let mut mapLstOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    mapLstOut = List::map1(mapLstIn.clone(), Arc::new(fnptr!(List::getIndexFirst, i32, _)), argLst.clone());
    mapLstOut
}

fn listIndecesForValues(mut idx: i32, mut lstIn: Arc<metamodelica::List<i32>>, mut arrayIn: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut arrayOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut value: i32 = 0;
    let mut valueLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    value = (lstIn.clone()).get(idx.clone())?;
    valueLst = arrayIn.clone().borrow()[(value.clone()-1) as usize].clone();
    valueLst = cons(idx.clone(), valueLst.clone());
    arrayOut = {let _arr = arrayIn.clone(); _arr.borrow_mut()[(value.clone()-1) as usize] = valueLst.clone(); _arr};
    Ok(arrayOut)
}

//---------------------------
// quicksort with order
//---------------------------
pub fn quicksortWithOrder(mut lstIn: Arc<metamodelica::List<metamodelica::Real>>) -> Result<(Arc<metamodelica::List<metamodelica::Real>>, Arc<metamodelica::List<i32>>)> {
    let mut lstOut: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut orderOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (lstOut, orderOut) = 'mc: {
        let __mc_input = lstIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut length: i32 = 0;
                    let mut pivotIdx: i32 = 0;
                    let mut r1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut r2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut r3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut pivotValue: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut orderTmp: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut lstTmp: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
                    length = (lstIn.clone().len() as i32);
                    orderTmp = List::intRange(length.clone());
                    r1 = listHead(lstIn.clone())?;
                    r2 = List::last(lstIn.clone())?;
                    r3 = (lstIn.clone()).get(intDiv(length.clone(), 2))?;
                    (pivotValue, _) = getMedian3(r1.clone(), r2.clone(), r3.clone())?;
                    pivotIdx = List::position(pivotValue.clone(), lstIn.clone())?;
                    (lstTmp, orderTmp) = quicksortWithOrder1(lstIn.clone(), orderTmp.clone(), pivotIdx.clone(), lstIn.clone(), length.clone())?;
                    Ok((lstTmp.clone(), orderTmp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r1, tail: Deref @ metamodelica::List::Nil } => {
                    Ok((list![r1.clone()], list![1]))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((lstOut, orderOut))
}

fn quicksortWithOrder1(mut lstIn: Arc<metamodelica::List<metamodelica::Real>>, mut orderIn: Arc<metamodelica::List<i32>>, mut pivotIdx: i32, mut markedIn: Arc<metamodelica::List<metamodelica::Real>>, mut size: i32) -> Result<(Arc<metamodelica::List<metamodelica::Real>>, Arc<metamodelica::List<i32>>)> {
    let mut lstOut: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut orderOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (lstOut, orderOut) = (::match_deref::match_deref! { match &((lstIn.clone(), orderIn.clone(), pivotIdx.clone(), markedIn.clone(), size.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _) => {
            (metamodelica::nil(), metamodelica::nil())
        },
        (Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, _, _, _, _) => {
            (list![e.clone()], list![1])
        },
        (_, _, _, Deref @ metamodelica::List::Nil, _) => {
            (lstIn.clone(), orderIn.clone())
        },
        _ => {
            let mut b1: bool = false;
            let mut b2: bool = false;
            let mut lIdx: i32 = 0;
            let mut rIdx: i32 = 0;
            let mut pivot: i32 = 0;
            let mut p: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut orderTmp: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut marked: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut lstTmp: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut leftLst: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut rightLst: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            p = (lstIn.clone()).get(pivotIdx.clone())?;
            (leftLst, rightLst) = List::split(lstIn.clone(), pivotIdx.clone())?;
            rightLst = rightLst.clone().reverse();
            (_, lIdx, b1) = getMemberOnTrueWithIdx(p.clone(), leftLst.clone(), Arc::new(fnptr!(realLt, metamodelica::Real, metamodelica::Real)))?;
            (_, rIdx, b2) = getMemberOnTrueWithIdx(p.clone(), rightLst.clone(), Arc::new(fnptr!(realGt, metamodelica::Real, metamodelica::Real)))?;
            rIdx = size.clone() + 1 - rIdx.clone();
            lstTmp = if (b1.clone()) {swapEntriesInList(pivotIdx.clone(), lIdx.clone(), lstIn.clone())?} else {lstIn.clone()};
            lstTmp = if (b2.clone()) {swapEntriesInList(pivotIdx.clone(), rIdx.clone(), lstTmp.clone())?} else {lstTmp.clone()};
            orderTmp = if (b1.clone()) {swapEntriesInList(pivotIdx.clone(), lIdx.clone(), orderIn.clone())?} else {orderIn.clone()};
            orderTmp = if (b2.clone()) {swapEntriesInList(pivotIdx.clone(), rIdx.clone(), orderTmp.clone())?} else {orderTmp.clone()};
            if !(b1.clone()) && !(b2.clone()) {
                (marked, pivot) = getNextPivot(lstTmp.clone(), markedIn.clone(), pivotIdx.clone())?;
            } else {
                marked = markedIn.clone();
                pivot = pivotIdx.clone();
            }
            (lstTmp, orderTmp) = quicksortWithOrder1(lstTmp.clone(), orderTmp.clone(), pivot.clone(), marked.clone(), size.clone())?;
            (lstTmp.clone(), orderTmp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((lstOut, orderOut))
}

fn getNextPivot(mut lstIn: Arc<metamodelica::List<metamodelica::Real>>, mut markedLstIn: Arc<metamodelica::List<metamodelica::Real>>, mut pivotIdx: i32) -> Result<(Arc<metamodelica::List<metamodelica::Real>>, i32)> {
    let mut marked: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut newIdx: i32 = 0;
    (marked, newIdx) = (::match_deref::match_deref! { match &((lstIn.clone(), markedLstIn.clone(), pivotIdx.clone())) {
        (_, Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, _) => {
            (metamodelica::nil(), 0)
        },
        (_, Deref @ metamodelica::List::Cons { head: _, tail: _ }, _) => {
            let mut midIdx: i32 = 0;
            let mut pivotElement: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut r1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut r2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut r3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            pivotElement = (lstIn.clone()).get(pivotIdx.clone())?;
            (marked, _) = List::deleteMemberOnTrue(pivotElement.clone(), markedLstIn.clone(), Arc::new(fnptr!(realEq, metamodelica::Real, metamodelica::Real)))?;
            r1 = listHead(marked.clone())?;
            r2 = List::last(marked.clone())?;
            midIdx = intDiv((marked.clone().len() as i32), 2);
            midIdx = if (intEq(midIdx.clone(), 0)) {1} else {midIdx.clone()};
            r3 = (marked.clone()).get(midIdx.clone())?;
            (pivotElement, _) = getMedian3(r1.clone(), r2.clone(), r3.clone())?;
            newIdx = List::position(pivotElement.clone(), lstIn.clone())?;
            (marked.clone(), newIdx.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((marked, newIdx))
}

fn getMemberOnTrueWithIdx(mut inValue: metamodelica::Real, mut inList: Arc<metamodelica::List<metamodelica::Real>>, mut inCompFunc: Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<bool> + 'static>) -> Result<(metamodelica::Real, i32, bool)> {
    pub type CompFunc = fn(metamodelica::Real, metamodelica::Real) -> Result<bool>;

    let mut outElement: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut outIdx: i32 = 0;
    let mut found: bool = false;
    (outElement, outIdx, found) = getMemberOnTrueWithIdx1(1, inValue.clone(), inList.clone(), inCompFunc.clone())?;
    Ok((outElement, outIdx, found))
}

fn getMemberOnTrueWithIdx1(mut inIdx: i32, mut inValue: metamodelica::Real, mut inList: Arc<metamodelica::List<metamodelica::Real>>, mut inCompFunc: Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<bool> + 'static>) -> Result<(metamodelica::Real, i32, bool)> {
    pub type CompFunc = fn(metamodelica::Real, metamodelica::Real) -> Result<bool>;

    let mut outElement: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut outIdx: i32 = 0;
    let mut found: bool = false;
    (outElement, outIdx, found) = 'mc: {
        let __mc_input = (inIdx.clone(), inValue.clone(), inList.clone(), inCompFunc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Nil, _) => {
                    Ok((metamodelica::OrderedFloat(0.0_f64), 0, false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Cons { head: e, tail: _ }, _) => {
                    let mut b: bool = false;
                    b = inCompFunc(inValue.clone(), e.clone())?;
                    let true = (b.clone()) else { bail!("pattern mismatch") };
                    Ok((e.clone(), inIdx.clone(), b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Cons { head: _, tail: rest }, _) => {
                    let mut value: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut idx: i32 = 0;
                    let mut b: bool = false;
                    (value, idx, b) = getMemberOnTrueWithIdx1(inIdx.clone() + 1, inValue.clone(), rest.clone(), inCompFunc.clone())?;
                    Ok((value.clone(), idx.clone(), b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outElement, outIdx, found))
}

fn swapEntriesInList<ElementType: Clone + 'static>(mut idx1: i32, mut idx2: i32, mut lstIn: Arc<metamodelica::List<ElementType>>) -> Result<Arc<metamodelica::List<ElementType>>> {
    let mut lstOut: Arc<metamodelica::List<ElementType>> = metamodelica::nil();
    let mut r1: ElementType;
    let mut r2: ElementType;
    let mut lstTmp: Arc<metamodelica::List<ElementType>> = metamodelica::nil();
    r1 = (lstIn.clone()).get(idx1.clone())?;
    r2 = (lstIn.clone()).get(idx2.clone())?;
    lstTmp = List::replaceAt(r1.clone(), idx2.clone(), lstIn.clone())?;
    lstOut = List::replaceAt(r2.clone(), idx1.clone(), lstTmp.clone())?;
    Ok(lstOut)
}

fn getMedian3(mut r1: metamodelica::Real, mut r2: metamodelica::Real, mut r3: metamodelica::Real) -> Result<(metamodelica::Real, i32)> {
    let mut rOut: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut which: i32 = 0;
    let mut r: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    r = List::sort(list![r1.clone(), r2.clone(), r3.clone()], Arc::new(fnptr!(realGt, metamodelica::Real, metamodelica::Real)))?;
    rOut = (r.clone()).get(2)?;
    which = List::position(rOut.clone(), list![r1.clone(), r2.clone(), r3.clone()])?;
    Ok((rOut, which))
}

//----------------------------
// traverse the task graph bottoms up (beginning at the root nodes)
//----------------------------
fn computeGraphValuesBottomUp(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta) -> Result<(metamodelica::Array<metamodelica::Real>, metamodelica::Array<metamodelica::Real>, metamodelica::Array<metamodelica::Real>)> {
    let mut asapOut: metamodelica::Array<metamodelica::Real>;
    let mut estOut: metamodelica::Array<metamodelica::Real>;
    let mut ectOut: metamodelica::Array<metamodelica::Real>;
    let mut size: i32 = 0;
    let mut rootNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut asap: metamodelica::Array<metamodelica::Real>;
    let mut ect: metamodelica::Array<metamodelica::Real>;
    let mut est: metamodelica::Array<metamodelica::Real>;
    let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    size = (iTaskGraph.clone().borrow().len() as i32);
    rootNodes = HpcOmTaskGraph::getRootNodes(iTaskGraph.clone())?;
    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), size.clone())?;
    asap = arrayCreate(size.clone(), metamodelica::OrderedFloat(-1.0_f64));
    est = arrayCreate(size.clone(), metamodelica::OrderedFloat(-1.0_f64));
    ect = arrayCreate(size.clone(), metamodelica::OrderedFloat(-1.0_f64));
    (asapOut, estOut, ectOut) = computeGraphValuesBottomUp1(rootNodes.clone(), iTaskGraph.clone(), taskGraphT.clone(), iTaskGraphMeta.clone(), asap.clone(), est.clone(), ect.clone())?;
    Ok((asapOut, estOut, ectOut))
}

fn computeGraphValuesBottomUp1(mut parentsIn: Arc<metamodelica::List<i32>>, mut graph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut graphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut asapIn: metamodelica::Array<metamodelica::Real>, mut estIn: metamodelica::Array<metamodelica::Real>, mut ectIn: metamodelica::Array<metamodelica::Real>) -> Result<(metamodelica::Array<metamodelica::Real>, metamodelica::Array<metamodelica::Real>, metamodelica::Array<metamodelica::Real>)> {
    let mut asapOut: metamodelica::Array<metamodelica::Real>;
    let mut estOut: metamodelica::Array<metamodelica::Real>;
    let mut ectOut: metamodelica::Array<metamodelica::Real>;
    (asapOut, estOut, ectOut) = (::match_deref::match_deref! { match &((parentsIn.clone(), graph.clone(), graphT.clone(), iTaskGraphMeta.clone(), asapIn.clone(), estIn.clone(), ectIn.clone())) {
        (Deref @ metamodelica::List::Cons { head: node, tail: rest }, _, _, _, asap, est, ect) => {
            let mut children: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut asap = (*asap).clone();
            let mut est = (*est).clone();
            let mut ect = (*ect).clone();
            (asap, est, ect, children) = computeGraphValuesBottomUp2(node.clone(), graph.clone(), graphT.clone(), iTaskGraphMeta.clone(), asap.clone(), est.clone(), ect.clone())?;
            (asap, est, ect) = computeGraphValuesBottomUp1(listAppend(rest.clone(), children.clone()), graph.clone(), graphT.clone(), iTaskGraphMeta.clone(), asap.clone(), est.clone(), ect.clone())?;
            (asap.clone(), est.clone(), ect.clone())
        },
        (Deref @ metamodelica::List::Nil, _, _, _, _, _, _) => {
            (asapIn.clone(), estIn.clone(), ectIn.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((asapOut, estOut, ectOut))
}

fn computeGraphValuesBottomUp2(mut node: i32, mut graph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut graphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut asapIn: metamodelica::Array<metamodelica::Real>, mut estIn: metamodelica::Array<metamodelica::Real>, mut ectIn: metamodelica::Array<metamodelica::Real>) -> Result<(metamodelica::Array<metamodelica::Real>, metamodelica::Array<metamodelica::Real>, metamodelica::Array<metamodelica::Real>, Arc<metamodelica::List<i32>>)> {
    let mut asapOut: metamodelica::Array<metamodelica::Real>;
    let mut estOut: metamodelica::Array<metamodelica::Real>;
    let mut ectOut: metamodelica::Array<metamodelica::Real>;
    let mut children: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (asapOut, estOut, ectOut, children) = 'mc: {
        let __mc_input = (node.clone(), graph.clone(), graphT.clone(), iTaskGraphMeta.clone(), asapIn.clone(), estIn.clone(), ectIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut maxASAP: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut maxEct: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut exeCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut asap: metamodelica::Array<metamodelica::Real>;
            let mut ect: metamodelica::Array<metamodelica::Real>;
            let mut est: metamodelica::Array<metamodelica::Real>;
            let mut parents: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut parentEcts: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut parentAsaps: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut parentAsaps2: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut parentsExeCosts: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut commCosts: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut children: Arc<metamodelica::List<i32>> = children.clone();
            parents = graphT.clone().borrow()[(node.clone()-1) as usize].clone();
            parentAsaps = List::map1(parents.clone(), Arc::new(fnptr!(Array::getIndexFirst, i32, _)), asapIn.clone());
            let false = (List::isMemberOnTrue(metamodelica::OrderedFloat(-1.0_f64), parentAsaps.clone(), Arc::new(fnptr!(realEq, metamodelica::Real, metamodelica::Real)))) else { bail!("pattern mismatch") };
            exeCost = HpcOmTaskGraph::getExeCostReqCycles(node.clone(), iTaskGraphMeta.clone())?;
            parentsExeCosts = List::map1(parents.clone(), Arc::new(HpcOmTaskGraph::getExeCostReqCycles), iTaskGraphMeta.clone());
            commCosts = List::map2(parents.clone(), Arc::new(HpcOmTaskGraph::getCommCostTimeBetweenNodes), node.clone(), iTaskGraphMeta.clone());
            parentAsaps2 = List::threadMap(parentAsaps.clone(), parentsExeCosts.clone(), Arc::new(fnptr!(realAdd, metamodelica::Real, metamodelica::Real)));
            parentAsaps2 = List::threadMap(parentAsaps2.clone(), commCosts.clone(), Arc::new(fnptr!(realAdd, metamodelica::Real, metamodelica::Real)));
            maxASAP = List::fold(parentAsaps2.clone(), Arc::new(fnptr!(realMax, metamodelica::Real, metamodelica::Real)), metamodelica::OrderedFloat(0.0_f64));
            asap = {let _arr = asapIn.clone(); _arr.borrow_mut()[(node.clone()-1) as usize] = maxASAP.clone(); _arr};
            parentEcts = List::map1(parents.clone(), Arc::new(fnptr!(Array::getIndexFirst, i32, _)), ectIn.clone());
            maxEct = List::fold(parentEcts.clone(), Arc::new(fnptr!(realMax, metamodelica::Real, metamodelica::Real)), metamodelica::OrderedFloat(0.0_f64));
            est = {let _arr = estIn.clone(); _arr.borrow_mut()[(node.clone()-1) as usize] = maxEct.clone(); _arr};
            ect = {let _arr = ectIn.clone(); _arr.borrow_mut()[(node.clone()-1) as usize] = (maxEct.clone()) + (exeCost.clone()); _arr};
            children = graph.clone().borrow()[(node.clone()-1) as usize].clone();
            Ok((asap.clone(), est.clone(), ect.clone(), children.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut parents: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut parentAsaps: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            parents = graphT.clone().borrow()[(node.clone()-1) as usize].clone();
            parentAsaps = List::map1(parents.clone(), Arc::new(fnptr!(Array::getIndexFirst, i32, _)), asapIn.clone());
            let true = (List::isMemberOnTrue(metamodelica::OrderedFloat(-1.0_f64), parentAsaps.clone(), Arc::new(fnptr!(realEq, metamodelica::Real, metamodelica::Real)))) else { bail!("pattern mismatch") };
            Ok((asapIn.clone(), estIn.clone(), ectIn.clone(), list![node.clone()]))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            println!("{}", (literal!("computeGraphValuesBottomUp2 failed!\n")).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((asapOut, estOut, ectOut, children))
}

//----------------------------
// traverse the task graph top down (beginning at the leaf nodes)
//----------------------------
fn computeGraphValuesTopDown(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta) -> Result<(metamodelica::Array<metamodelica::Real>, metamodelica::Array<metamodelica::Real>, metamodelica::Array<metamodelica::Real>, metamodelica::Array<metamodelica::Real>)> {
    let mut alapOut: metamodelica::Array<metamodelica::Real>;
    let mut lastOut: metamodelica::Array<metamodelica::Real>;
    let mut lactOut: metamodelica::Array<metamodelica::Real>;
    let mut tdsLevelOut: metamodelica::Array<metamodelica::Real>;
    let mut size: i32 = 0;
    let mut lastNodeInCP: i32 = 0;
    let mut cp: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut cpWithComm: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut endNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut alap: metamodelica::Array<metamodelica::Real>;
    let mut lact: metamodelica::Array<metamodelica::Real>;
    let mut last: metamodelica::Array<metamodelica::Real>;
    let mut tdsLevel: metamodelica::Array<metamodelica::Real>;
    let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut visitedNodes: metamodelica::Array<bool>;
    size = (iTaskGraph.clone().borrow().len() as i32);
    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), size.clone())?;
    endNodes = HpcOmTaskGraph::getLeafNodes(iTaskGraph.clone())?;
    alap = arrayCreate(size.clone(), metamodelica::OrderedFloat(-1.0_f64));
    last = arrayCreate(size.clone(), metamodelica::OrderedFloat(-1.0_f64));
    lact = arrayCreate(size.clone(), metamodelica::OrderedFloat(-1.0_f64));
    tdsLevel = arrayCreate(size.clone(), metamodelica::OrderedFloat(-1.0_f64));
    visitedNodes = arrayCreate(size.clone(), false);
    computeGraphValuesTopDown1(endNodes.clone(), iTaskGraph.clone(), taskGraphT.clone(), iTaskGraphMeta.clone(), alap.clone(), last.clone(), lact.clone(), tdsLevel.clone(), visitedNodes.clone())?;
    cpWithComm = Array::fold(alap.clone(), Arc::new(fnptr!(realMax, metamodelica::Real, metamodelica::Real)), metamodelica::OrderedFloat(0.0_f64));
    lastNodeInCP = Array::position(alap.clone(), cpWithComm.clone(), size.clone());
    cp = Array::fold(last.clone(), Arc::new(fnptr!(realMax, metamodelica::Real, metamodelica::Real)), metamodelica::OrderedFloat(0.0_f64));
    alapOut = Array::map1(alap.clone(), Arc::new(fnptr!(realSubr, metamodelica::Real, metamodelica::Real)), cpWithComm.clone())?;
    lactOut = Array::map1(lact.clone(), Arc::new(fnptr!(realSubr, metamodelica::Real, metamodelica::Real)), cp.clone())?;
    lastOut = Array::map1(last.clone(), Arc::new(fnptr!(realSubr, metamodelica::Real, metamodelica::Real)), cp.clone())?;
    tdsLevelOut = tdsLevel.clone();
    Ok((alapOut, lastOut, lactOut, tdsLevelOut))
}

fn computeGraphValuesTopDown1(mut nodesIn: Arc<metamodelica::List<i32>>, mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut alapIn: metamodelica::Array<metamodelica::Real>, mut lastIn: metamodelica::Array<metamodelica::Real>, mut lactIn: metamodelica::Array<metamodelica::Real>, mut tdsLevelIn: metamodelica::Array<metamodelica::Real>, mut visitedNodes: metamodelica::Array<bool>) -> Result<()> {
    let mut nodes: Arc<metamodelica::List<i32>> = nodesIn.clone();
    let mut alap: metamodelica::Array<metamodelica::Real> = alapIn.clone();
    let mut last: metamodelica::Array<metamodelica::Real> = lastIn.clone();
    let mut lact: metamodelica::Array<metamodelica::Real> = lactIn.clone();
    let mut tdsLevel: metamodelica::Array<metamodelica::Real> = tdsLevelIn.clone();
    while !(nodes.clone().is_empty()) {
        if visitedNodes.clone().borrow()[(listHead(nodes.clone())?-1) as usize].clone() {
            nodes = listRest(nodes.clone())?;
        } else {
            nodes = computeGraphValuesTopDown2(nodes.clone(), iTaskGraph.clone(), iTaskGraphT.clone(), iTaskGraphMeta.clone(), alap.clone(), last.clone(), lact.clone(), tdsLevel.clone(), visitedNodes.clone())?;
        }
    }
    Ok(())
}

fn computeGraphValuesTopDown2(mut nodesIn: Arc<metamodelica::List<i32>>, mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut alapIn: metamodelica::Array<metamodelica::Real>, mut lastIn: metamodelica::Array<metamodelica::Real>, mut lactIn: metamodelica::Array<metamodelica::Real>, mut tdsLevelIn: metamodelica::Array<metamodelica::Real>, mut visitedNodes: metamodelica::Array<bool>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut nodesOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut computeValues: bool = false;
    let mut nodeIdx: i32 = 0;
    let mut pos: i32 = 0;
    let mut nodeExeCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut maxLevel: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut maxAlap: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut maxLast: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut maxLact: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut rest: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut parentNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut childNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut childTDSLevels: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut childAlaps: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut childLasts: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut childLacts: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut commCostsToChilds: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut alap: metamodelica::Array<metamodelica::Real>;
    let mut last: metamodelica::Array<metamodelica::Real>;
    let mut lact: metamodelica::Array<metamodelica::Real>;
    let mut tdsLevel: metamodelica::Array<metamodelica::Real>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(nodesIn.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    nodeIdx = __pa0.clone();
    rest = __pa1.clone();
    childNodes = iTaskGraph.clone().borrow()[(nodeIdx.clone()-1) as usize].clone();
    nodeExeCost = HpcOmTaskGraph::getExeCostReqCycles(nodeIdx.clone(), iTaskGraphMeta.clone())?;
    {let _arr = visitedNodes.clone(); _arr.borrow_mut()[(nodeIdx.clone()-1) as usize] = true; _arr};
    if childNodes.clone().is_empty() {
        alap = {let _arr = alapIn.clone(); _arr.borrow_mut()[(nodeIdx.clone()-1) as usize] = nodeExeCost.clone(); _arr};
        last = {let _arr = lastIn.clone(); _arr.borrow_mut()[(nodeIdx.clone()-1) as usize] = nodeExeCost.clone(); _arr};
        lact = {let _arr = lactIn.clone(); _arr.borrow_mut()[(nodeIdx.clone()-1) as usize] = metamodelica::OrderedFloat(0.0_f64); _arr};
        tdsLevel = {let _arr = tdsLevelIn.clone(); _arr.borrow_mut()[(nodeIdx.clone()-1) as usize] = nodeExeCost.clone(); _arr};
        parentNodes = iTaskGraphT.clone().borrow()[(nodeIdx.clone()-1) as usize].clone();
        nodesOut = listAppend(rest.clone(), parentNodes.clone());
    } else {
        childTDSLevels = List::map1(childNodes.clone(), Arc::new(fnptr!(Array::getIndexFirst, i32, _)), tdsLevelIn.clone());
        if List::isMemberOnTrue(metamodelica::OrderedFloat(-1.0_f64), childTDSLevels.clone(), Arc::new(fnptr!(realEq, metamodelica::Real, metamodelica::Real))) {
            nodesOut = listAppend(rest.clone(), list![nodeIdx.clone()]);
            {let _arr = visitedNodes.clone(); _arr.borrow_mut()[(nodeIdx.clone()-1) as usize] = false; _arr};
        } else {
            commCostsToChilds = {
        let mut __acc: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
        for mut n in (childNodes.clone()).into_iter().cloned() {
            let __x = HpcOmTaskGraph::getCommCostTimeBetweenNodes(nodeIdx.clone(), n.clone(), iTaskGraphMeta.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            childAlaps = List::map1(childNodes.clone(), Arc::new(fnptr!(Array::getIndexFirst, i32, _)), alapIn.clone());
            childAlaps = List::threadMap(childAlaps.clone(), commCostsToChilds.clone(), Arc::new(fnptr!(realAdd, metamodelica::Real, metamodelica::Real)));
            childLasts = List::map1(childNodes.clone(), Arc::new(fnptr!(Array::getIndexFirst, i32, _)), lastIn.clone());
            childLacts = List::map1(childNodes.clone(), Arc::new(fnptr!(Array::getIndexFirst, i32, _)), lactIn.clone());
            maxLevel = List::fold(childTDSLevels.clone(), Arc::new(fnptr!(realMax, metamodelica::Real, metamodelica::Real)), metamodelica::OrderedFloat(0.0_f64));
            maxAlap = List::fold(childAlaps.clone(), Arc::new(fnptr!(realMax, metamodelica::Real, metamodelica::Real)), metamodelica::OrderedFloat(0.0_f64));
            maxLast = List::fold(childLasts.clone(), Arc::new(fnptr!(realMax, metamodelica::Real, metamodelica::Real)), metamodelica::OrderedFloat(0.0_f64));
            let _ = List::fold(childLacts.clone(), Arc::new(fnptr!(realMax, metamodelica::Real, metamodelica::Real)), metamodelica::OrderedFloat(0.0_f64));
            tdsLevel = {let _arr = tdsLevelIn.clone(); _arr.borrow_mut()[(nodeIdx.clone()-1) as usize] = nodeExeCost.clone() + maxLevel.clone(); _arr};
            alap = {let _arr = alapIn.clone(); _arr.borrow_mut()[(nodeIdx.clone()-1) as usize] = nodeExeCost.clone() + maxAlap.clone(); _arr};
            last = {let _arr = lastIn.clone(); _arr.borrow_mut()[(nodeIdx.clone()-1) as usize] = nodeExeCost.clone() + maxLast.clone(); _arr};
            lact = {let _arr = lactIn.clone(); _arr.borrow_mut()[(nodeIdx.clone()-1) as usize] = maxLast.clone(); _arr};
            parentNodes = iTaskGraphT.clone().borrow()[(nodeIdx.clone()-1) as usize].clone();
            nodesOut = listAppend(rest.clone(), parentNodes.clone());
        }
    }
    Ok(nodesOut)
}

fn realSubr(mut r1: metamodelica::Real, mut r2: metamodelica::Real) -> metamodelica::Real {
    let mut r3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    r3 = (r2.clone()) - (r1.clone());
    r3
}

//-----
// Util
//-----
pub fn printSchedule(mut iSchedule: Arc<HpcOmSimCode::Schedule>) -> Result<()> {
    println!("{}", (dumpSchedule(iSchedule.clone())?).clone());
    Ok(())
}

fn dumpSchedule(mut iSchedule: Arc<HpcOmSimCode::Schedule>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut s: ArcStr = arcstr::literal!("");
    let mut sLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outgoingDepTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut allTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut tasksOfLevels: Arc<metamodelica::List<HpcOmSimCode::TaskList>> = metamodelica::nil();
    let mut taskDepTasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    r#str = ((::match_deref::match_deref! { match &(iSchedule.clone()) {
        Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { outgoingDepTasks, threadTasks, .. } => {
            (sLst, _) = List::mapFold(Arc::new(threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), Arc::new(fnptr!(dumpThreadSchedule, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, i32)), 1);
            s = stringDelimitList(sLst.clone(), (literal!("\n")).clone());
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("\nDependency tasks: {\n")); __mm_s.push_str(&*stringDelimitList(List::map(outgoingDepTasks.clone(), Arc::new(dumpTask)), (literal!("")).clone())); __mm_s.push_str(&*literal!("}\n")); ArcStr::from(__mm_s) }).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("THREADSCHEDULE\n")); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
            s.clone()
        },
        Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { tasksOfLevels, .. } => {
            (sLst, _) = List::mapFold(tasksOfLevels.clone(), Arc::new(dumpLevelSchedule), 1);
            s = stringDelimitList(sLst.clone(), (literal!("\n")).clone());
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("LEVELSCHEDULE\n")); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
            s.clone()
        },
        Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: taskDepTasks } => {
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(List::map(taskDepTasks.clone(), Arc::new(dumpTaskDepSchedule)), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TASKDEPSCHEDULE\n")); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
            s.clone()
        },
        Deref @ HpcOmSimCode::Schedule::EMPTYSCHEDULE { tasks: HpcOmSimCode::TaskList::SERIALTASKLIST { tasks: allTasks, .. } } => {
            (s, _) = dumpThreadSchedule(allTasks.clone(), 1);
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("EMPTYSCHEDULE\n")); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
            s.clone()
        },
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(r#str)
}

pub fn analyseScheduledTaskGraph(mut scheduleIn: Arc<HpcOmSimCode::Schedule>, mut numProcIn: i32, mut taskGraphIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphMetaIn: HpcOmTaskGraph::TaskGraphMeta, mut inSystemName: ArcStr) -> Result<ArcStr> {
    let mut criticalPathInfoOut: ArcStr = arcstr::literal!("");
    criticalPathInfoOut = ('mc: {
        let __mc_input = (scheduleIn.clone(), numProcIn.clone(), taskGraphIn.clone(), taskGraphMetaIn.clone(), inSystemName.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ HpcOmSimCode::Schedule::EMPTYSCHEDULE { tasks: _ }, _, _, _, _) => {
                    let mut criticalPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut criticalPathsWoC: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut cpCosts: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut cpCostsWoC: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut criticalPathInfo: ArcStr = arcstr::literal!("");
                    let ((__pa0, __pa1), (__pa2, __pa3)) = HpcOmTaskGraph::getCriticalPaths(taskGraphIn.clone(), taskGraphMetaIn.clone())?;
                    criticalPaths = __pa0.clone();
                    cpCosts = __pa1.clone();
                    criticalPathsWoC = __pa2.clone();
                    cpCostsWoC = __pa3.clone();
                    criticalPathInfo = (HpcOmTaskGraph::dumpCriticalPathInfo((criticalPaths.clone(), cpCosts.clone()), (criticalPathsWoC.clone(), cpCostsWoC.clone()))?).clone();
                    Ok(criticalPathInfo.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { useFixedAssignments: false, tasksOfLevels }, _, _, _, _) => {
                    let mut criticalPathInfo: ArcStr = arcstr::literal!("");
                    criticalPathInfo = (analyseScheduledTaskGraphLevel(tasksOfLevels.clone(), numProcIn.clone(), taskGraphIn.clone(), taskGraphMetaIn.clone(), Arc::new(fnptr!(getLevelParallelTime, HpcOmSimCode::TaskList, metamodelica::Array<Arc<metamodelica::List<i32>>>, HpcOmTaskGraph::TaskGraphMeta, i32)))?).clone();
                    Ok(criticalPathInfo.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { useFixedAssignments: true, tasksOfLevels }, _, _, _, _) => {
                    let mut criticalPathInfo: ArcStr = arcstr::literal!("");
                    criticalPathInfo = (analyseScheduledTaskGraphLevel(tasksOfLevels.clone(), numProcIn.clone(), taskGraphIn.clone(), taskGraphMetaIn.clone(), Arc::new(fnptr!(getLevelParallelTime, HpcOmSimCode::TaskList, metamodelica::Array<Arc<metamodelica::List<i32>>>, HpcOmTaskGraph::TaskGraphMeta, i32)))?).clone();
                    Ok(criticalPathInfo.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { outgoingDepTasks, .. }, _, _, _, _) => {
                    let mut criticalPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut criticalPathsWoC: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut cpCosts: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut cpCostsWoC: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut serTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut parTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut speedUp: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut speedUpMax: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut criticalPathInfo: ArcStr = arcstr::literal!("");
                    if Flags::isSet(Flags::HPCOM_DUMP.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("the number of locks: ")); __mm_s.push_str(&*intString((outgoingDepTasks.clone().len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    let ((__pa0, __pa1), (__pa2, __pa3)) = HpcOmTaskGraph::getCriticalPaths(taskGraphIn.clone(), taskGraphMetaIn.clone())?;
                    criticalPaths = __pa0.clone();
                    cpCosts = __pa1.clone();
                    criticalPathsWoC = __pa2.clone();
                    cpCostsWoC = __pa3.clone();
                    criticalPathInfo = (HpcOmTaskGraph::dumpCriticalPathInfo((criticalPaths.clone(), cpCosts.clone()), (criticalPathsWoC.clone(), cpCostsWoC.clone()))?).clone();
                    (serTime, parTime, speedUp, speedUpMax) = predictExecutionTime(scheduleIn.clone(), Some(cpCostsWoC.clone()), numProcIn.clone(), taskGraphIn.clone(), taskGraphMetaIn.clone())?;
                    serTime = HpcOmTaskGraph::roundReal(serTime.clone(), 2);
                    parTime = HpcOmTaskGraph::roundReal(parTime.clone(), 2);
                    cpCostsWoC = HpcOmTaskGraph::roundReal(cpCostsWoC.clone(), 2);
                    if Flags::isSet(Flags::HPCOM_DUMP.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("the serialCosts: ")); __mm_s.push_str(&*realString(serTime.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("the parallelCosts: ")); __mm_s.push_str(&*realString(parTime.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("the cpCosts: ")); __mm_s.push_str(&*realString(cpCostsWoC.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    if realLe(speedUpMax.clone(), metamodelica::OrderedFloat(2.0_f64)) {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("There is no parallel potential in the ")); __mm_s.push_str(&*inSystemName.clone()); __mm_s.push_str(&*literal!(" model!\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    if realLe(serTime.clone(), metamodelica::OrderedFloat(20000.0_f64)) {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The ")); __mm_s.push_str(&*inSystemName.clone()); __mm_s.push_str(&*literal!(" model is not big enough to perform an effective parallel simulation!\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    printPredictedExeTimeInfo(serTime.clone(), parTime.clone(), speedUp.clone(), speedUpMax.clone(), numProcIn.clone())?;
                    Ok(criticalPathInfo.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { .. }, _, _, _, _) => {
                    let mut criticalPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut criticalPathsWoC: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut cpCosts: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut cpCostsWoC: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut criticalPathInfo: ArcStr = arcstr::literal!("");
                    let ((__pa0, __pa1), (__pa2, __pa3)) = HpcOmTaskGraph::getCriticalPaths(taskGraphIn.clone(), taskGraphMetaIn.clone())?;
                    criticalPaths = __pa0.clone();
                    cpCosts = __pa1.clone();
                    criticalPathsWoC = __pa2.clone();
                    cpCostsWoC = __pa3.clone();
                    criticalPathInfo = (HpcOmTaskGraph::dumpCriticalPathInfo((criticalPaths.clone(), cpCosts.clone()), (criticalPathsWoC.clone(), cpCostsWoC.clone()))?).clone();
                    Ok(criticalPathInfo.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("HpcOmScheduler.analyseScheduledTaskGraph failed\n")).clone());
                    Ok(literal!("HpcOmScheduler.analyseScheduledTaskGraph failed\n"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(criticalPathInfoOut)
}

fn analyseScheduledTaskGraphLevel(mut iLevelTasks: Arc<metamodelica::List<HpcOmSimCode::TaskList>>, mut iNumProc: i32, mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iParallelSectionCalculator: Arc<dyn ::std::ops::Fn(HpcOmSimCode::TaskList, metamodelica::Array<Arc<metamodelica::List<i32>>>, HpcOmTaskGraph::TaskGraphMeta, i32) -> Result<metamodelica::Real> + 'static>) -> Result<ArcStr> {
    pub type LevelParallelSectionFunc = fn(HpcOmSimCode::TaskList, metamodelica::Array<Arc<metamodelica::List<i32>>>, HpcOmTaskGraph::TaskGraphMeta, i32) -> Result<metamodelica::Real>;

    let mut oCriticalPathInfo: ArcStr = arcstr::literal!("");
    let mut i: i32 = 0;
    let mut costShare: i32 = 0;
    let mut levelCosts: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut criticalPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut criticalPathsWoC: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut levelSectionCosts: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut cpCosts: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut cpCostsWoC: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut serTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut parTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut speedUp: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut speedUpMax: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut levelCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let ((__pa0, __pa1), (__pa2, __pa3)) = HpcOmTaskGraph::getCriticalPaths(iTaskGraph.clone(), iTaskGraphMeta.clone())?;
    criticalPaths = __pa0.clone();
    cpCosts = __pa1.clone();
    criticalPathsWoC = __pa2.clone();
    cpCostsWoC = __pa3.clone();
    levelSectionCosts = List::map1(iLevelTasks.clone(), Arc::new(fnptr!(getLevelListTaskCosts, HpcOmSimCode::TaskList, HpcOmTaskGraph::TaskGraphMeta)), iTaskGraphMeta.clone());
    serTime = realSum(List::map(levelSectionCosts.clone(), Arc::new(fnptr!(realSum, Arc<metamodelica::List<metamodelica::Real>>))));
    serTime = HpcOmTaskGraph::roundReal(serTime.clone(), 2);
    levelCosts = List::map(iLevelTasks.clone(), Arc::new({ let __pe_b1 = iTaskGraph.clone(); let __pe_b2 = iTaskGraphMeta.clone(); let __pe_b3 = iNumProc.clone(); move |__pe_a0| iParallelSectionCalculator(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }));
    parTime = realSum(levelCosts.clone());
    parTime = HpcOmTaskGraph::roundReal(parTime.clone(), 2);
    oCriticalPathInfo = (HpcOmTaskGraph::dumpCriticalPathInfo((criticalPaths.clone(), cpCosts.clone()), (criticalPathsWoC.clone(), cpCostsWoC.clone()))?).clone();
    cpCostsWoC = HpcOmTaskGraph::roundReal(cpCostsWoC.clone(), 2);
    if Flags::isSet(Flags::HPCOM_DUMP.clone())? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("the serialCosts: ")); __mm_s.push_str(&*realString(serTime.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("the parallelCosts: ")); __mm_s.push_str(&*realString(parTime.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("the cpCosts: ")); __mm_s.push_str(&*realString(cpCostsWoC.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        i = 1;
        for mut levelCost in &*levelCosts.clone() {
            let mut levelCost = levelCost.clone();
            costShare = intDiv(((levelCost.clone()).0 as i32) * 100, ((parTime.clone()).0 as i32));
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\tcosts for level ")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*realString(levelCost.clone())); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*System::snprintff((literal!("%.0f")).clone(), 5, metamodelica::OrderedFloat((costShare.clone()) as f64))?); __mm_s.push_str(&*literal!("%)\n")); ArcStr::from(__mm_s) }).clone());
            i = i.clone() + 1;
        }
    }
    speedUp = metamodelica::OrderedFloat(0.0_f64);
    speedUpMax = metamodelica::OrderedFloat(0.0_f64);
    if realNe(parTime.clone(), metamodelica::OrderedFloat(0.0_f64)) {
        speedUp = realDiv(serTime.clone(), parTime.clone());
    }
    if realNe(cpCostsWoC.clone(), metamodelica::OrderedFloat(0.0_f64)) {
        speedUpMax = realDiv(serTime.clone(), cpCostsWoC.clone());
    }
    printPredictedExeTimeInfo(serTime.clone(), parTime.clone(), speedUp.clone(), speedUpMax.clone(), iNumProc.clone())?;
    Ok(oCriticalPathInfo)
}

fn getLevelParallelTime(mut iLevelTaskList: HpcOmSimCode::TaskList, mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iNumProc: i32) -> metamodelica::Real {
    let mut oLevelCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut workload: metamodelica::Array<metamodelica::Real>;
    let mut levelTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    levelTasks = getTasksOfTaskList(iLevelTaskList.clone());
    workload = arrayCreate(iNumProc.clone(), metamodelica::OrderedFloat(0.0_f64));
    workload = List::fold(levelTasks.clone(), Arc::new({ let __pe_b1 = iTaskGraphMeta.clone(); move |__pe_a0, __pe_a2| getLevelParallelTime1(__pe_a0, __pe_b1.clone(), __pe_a2) }), workload.clone());
    oLevelCost = Array::fold(workload.clone(), Arc::new(fnptr!(realMax, metamodelica::Real, metamodelica::Real)), metamodelica::OrderedFloat(0.0_f64));
    oLevelCost
}

fn getLevelParallelTime1(mut iTask: Arc<HpcOmSimCode::Task>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iThreadWorkLoad: metamodelica::Array<metamodelica::Real>) -> Result<metamodelica::Array<metamodelica::Real>> {
    let mut oThreadWorkLoad: metamodelica::Array<metamodelica::Real>;
    let mut minWorkLoad: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut taskCosts: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut threadIdx: i32 = 0;
    let mut tmpThreadWorkLoad: metamodelica::Array<metamodelica::Real>;
    oThreadWorkLoad = (::match_deref::match_deref! { match &((iTask.clone(), iTaskGraphMeta.clone(), iThreadWorkLoad.clone())) {
        (Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { threadIdx: None, .. }, _, _) => {
            taskCosts = getLevelTaskCosts(iTask.clone(), iTaskGraphMeta.clone())?;
            minWorkLoad = Array::fold(iThreadWorkLoad.clone(), Arc::new(fnptr!(realMin, metamodelica::Real, metamodelica::Real)), iThreadWorkLoad.clone().borrow()[(1-1) as usize].clone());
            threadIdx = List::position(minWorkLoad.clone(), Arc::new(iThreadWorkLoad.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()))?;
            tmpThreadWorkLoad = {let _arr = iThreadWorkLoad.clone(); _arr.borrow_mut()[(threadIdx.clone()-1) as usize] = minWorkLoad.clone() + taskCosts.clone(); _arr};
            tmpThreadWorkLoad.clone()
        },
        (Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { threadIdx: Some(threadIdx), .. }, _, _) => {
            taskCosts = getLevelTaskCosts(iTask.clone(), iTaskGraphMeta.clone())?;
            tmpThreadWorkLoad = {let _arr = iThreadWorkLoad.clone(); let _val = iThreadWorkLoad.clone().borrow()[(threadIdx.clone()-1) as usize].clone() + taskCosts.clone(); _arr.borrow_mut()[(threadIdx.clone()-1) as usize] = _val; _arr};
            tmpThreadWorkLoad.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(oThreadWorkLoad)
}

fn getTasksOfTaskList(mut iTaskList: HpcOmSimCode::TaskList) -> Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> {
    let mut oTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut tasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    oTasks = (match iTaskList.clone() {
        HpcOmSimCode::TaskList::PARALLELTASKLIST { tasks: mut tasks } => tasks.clone(),
        HpcOmSimCode::TaskList::SERIALTASKLIST { tasks: mut tasks, .. } => tasks.clone(),
        _ => {
            println!("{}", (literal!("getTasksOfTaskList failed! Unsupported task list.\n")).clone());
            metamodelica::nil()
        },
    });
    oTasks
}

fn getLevelListTaskCosts(mut iTaskList: HpcOmSimCode::TaskList, mut iMeta: HpcOmTaskGraph::TaskGraphMeta) -> Arc<metamodelica::List<metamodelica::Real>> {
    let mut costsOut: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut tasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut costs: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    tasks = getTasksOfTaskList(iTaskList.clone());
    costsOut = List::map1(tasks.clone(), Arc::new(getLevelTaskCosts), iMeta.clone());
    costsOut
}

fn getLevelTaskCosts(mut levelTask: Arc<HpcOmSimCode::Task>, mut iMeta: HpcOmTaskGraph::TaskGraphMeta) -> Result<metamodelica::Real> {
    let mut costsOut: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    costsOut = (::match_deref::match_deref! { match &((levelTask.clone(), iMeta.clone())) {
        (Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { nodeIdc, .. }, _) => {
            let mut nodeCosts: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut costs: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            nodeCosts = List::map1(nodeIdc.clone(), Arc::new(HpcOmTaskGraph::getExeCostReqCycles), iMeta.clone());
            costs = List::fold(nodeCosts.clone(), Arc::new(fnptr!(realAdd, metamodelica::Real, metamodelica::Real)), metamodelica::OrderedFloat(0.0_f64));
            costs.clone()
        },
        _ => {
            println!("{}", (literal!("getLevelTaskCosts failed!\n")).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(costsOut)
}

pub fn predictExecutionTime(mut scheduleIn: Arc<HpcOmSimCode::Schedule>, mut cpCostsOption: Option<metamodelica::Real>, mut numProc: i32, mut taskGraphIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphMetaIn: HpcOmTaskGraph::TaskGraphMeta) -> Result<(metamodelica::Real, metamodelica::Real, metamodelica::Real, metamodelica::Real)> {
    let mut serialTimeOut: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut parallelTimeOut: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut speedUpOut: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut speedUpMaxOut: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut parTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut serTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut speedUp: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut speedUpMax: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut cpCosts: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut helper: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut schedule: Arc<HpcOmSimCode::Schedule>;
    if intNe((taskGraphIn.clone().borrow().len() as i32), 0) {
        serTime = getSerialExecutionTime(taskGraphMetaIn.clone())?;
        (_, parTime) = getFinishingTimesForSchedule(scheduleIn.clone(), numProc.clone(), taskGraphIn.clone(), taskGraphMetaIn.clone())?;
        speedUp = serTime.clone() / parTime.clone();
        helper = Util::getOptionOrDefault(cpCostsOption.clone(), (metamodelica::OrderedFloat(-1.0_f64)) * (serTime.clone()));
        speedUpMax = realDiv(serTime.clone(), helper.clone());
    }
    serialTimeOut = serTime.clone();
    parallelTimeOut = parTime.clone();
    speedUpOut = speedUp.clone();
    speedUpMaxOut = speedUpMax.clone();
    Ok((serialTimeOut, parallelTimeOut, speedUpOut, speedUpMaxOut))
}

fn printPredictedExeTimeInfo(mut serTime: metamodelica::Real, mut parTime: metamodelica::Real, mut speedUp: metamodelica::Real, mut speedUpMax: metamodelica::Real, mut numProc: i32) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (serTime.clone(), parTime.clone(), speedUp.clone(), speedUpMax.clone(), numProc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, __rlit_0, _) = __mc_input.clone() else { bail!("nomatch") };
            if !(__rlit_0.eq(&metamodelica::OrderedFloat((0.0) as f64))) { bail!("guard") }
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let true = (speedUpMax.clone() == metamodelica::OrderedFloat(-1.0_f64)) else { bail!("pattern mismatch") };
            if Flags::isSet(Flags::HPCOM_DUMP.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The predicted SpeedUp with ")); __mm_s.push_str(&*intString(numProc.clone())); __mm_s.push_str(&*literal!(" processors is ")); __mm_s.push_str(&*System::snprintff((literal!("%.2f")).clone(), 25, speedUp.clone())?); __mm_s.push_str(&*literal!(".\n")); ArcStr::from(__mm_s) }).clone());
            }
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            if Flags::isSet(Flags::HPCOM_DUMP.clone())? {
                if speedUp.clone() > speedUpMax.clone() {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Something is weird. The predicted SpeedUp is ")); __mm_s.push_str(&*System::snprintff((literal!("%.2f")).clone(), 25, speedUp.clone())?); __mm_s.push_str(&*literal!(" and the theoretical maximum speedUp is ")); __mm_s.push_str(&*System::snprintff((literal!("%.2f")).clone(), 25, speedUpMax.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                } else if speedUp.clone() <= speedUpMax.clone() {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The predicted SpeedUp with ")); __mm_s.push_str(&*intString(numProc.clone())); __mm_s.push_str(&*literal!(" processors is: ")); __mm_s.push_str(&*System::snprintff((literal!("%.2f")).clone(), 25, speedUp.clone())?); __mm_s.push_str(&*literal!(" With a theoretical maximmum speedUp of: ")); __mm_s.push_str(&*System::snprintff((literal!("%.2f")).clone(), 25, speedUpMax.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                }
            }
            Ok(())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn getSerialExecutionTime(mut taskGraphMetaIn: HpcOmTaskGraph::TaskGraphMeta) -> Result<metamodelica::Real> {
    let mut serialTimeOut: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut odeComps: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut exeCostsReal: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut exeCosts1: metamodelica::Array<metamodelica::Real>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let HpcOmTaskGraph::TASKGRAPHMETA { inComps: __pa0, exeCosts: __pa1, .. } = (taskGraphMetaIn.clone()) else { bail!("pattern mismatch") };
    inComps = __pa0.clone();
    exeCosts = __pa1.clone();
    odeComps = Array::fold(inComps.clone(), Arc::new(listAppend.clone()), metamodelica::nil());
    exeCosts1 = Array::map(exeCosts.clone(), Arc::new(fnptr!(Util::tuple22, _)));
    exeCostsReal = List::map1(odeComps.clone(), Arc::new(fnptr!(Array::getIndexFirst, i32, _)), exeCosts1.clone());
    serialTimeOut = List::fold(exeCostsReal.clone(), Arc::new(fnptr!(realAdd, metamodelica::Real, metamodelica::Real)), metamodelica::OrderedFloat(0.0_f64));
    Ok(serialTimeOut)
}

fn getFinishingTimesForSchedule(mut scheduleIn: Arc<HpcOmSimCode::Schedule>, mut numProc: i32, mut taskGraphIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphMetaIn: HpcOmTaskGraph::TaskGraphMeta) -> Result<(Arc<HpcOmSimCode::Schedule>, metamodelica::Real)> {
    let mut scheduleOut: Arc<HpcOmSimCode::Schedule>;
    let mut finishingTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    (scheduleOut, finishingTime) = 'mc: {
        let __mc_input = (scheduleIn.clone(), numProc.clone(), taskGraphIn.clone(), taskGraphMetaIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { allCalcTasks, outgoingDepTasks, threadTasks, .. }, _, _, _) => {
                    let mut finTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut taskIdcs: metamodelica::Array<i32>;
                    let mut finTimes: metamodelica::Array<metamodelica::Real>;
                    let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut checkedTasks: metamodelica::Array<Arc<HpcOmSimCode::Task>>;
                    let mut schedule: Arc<HpcOmSimCode::Schedule>;
                    taskIdcs = arrayCreate((threadTasks.clone().borrow().len() as i32), 1);
                    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(taskGraphIn.clone(), (taskGraphIn.clone().borrow().len() as i32))?;
                    checkedTasks = arrayCreate((taskGraphIn.clone().borrow().len() as i32), Arc::new(crate::HpcOmSimCode::Task::TASKEMPTY));
                    computeTimeFinished(threadTasks.clone(), taskIdcs.clone(), 1, checkedTasks.clone(), taskGraphIn.clone(), taskGraphT.clone(), taskGraphMetaIn.clone(), numProc.clone(), metamodelica::nil())?;
                    finTimes = Array::map(threadTasks.clone(), Arc::new(getTimeFinishedOfLastTask));
                    finTime = Array::fold(finTimes.clone(), Arc::new(fnptr!(realMax, metamodelica::Real, metamodelica::Real)), metamodelica::OrderedFloat(0.0_f64));
                    schedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
                    Ok((schedule.clone(), finTime.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { tasksOfLevels: _, useFixedAssignments: _ }, _, _, _) => {
                    let mut finTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut schedule: Arc<HpcOmSimCode::Schedule>;
                    schedule = scheduleIn.clone();
                    finTime = metamodelica::OrderedFloat(0.0_f64);
                    Ok((schedule.clone(), finTime.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ HpcOmSimCode::Schedule::EMPTYSCHEDULE { .. }, _, _, _) => {
                    let mut finTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut schedule: Arc<HpcOmSimCode::Schedule>;
                    schedule = scheduleIn.clone();
                    finTime = metamodelica::OrderedFloat(-1.0_f64);
                    Ok((schedule.clone(), finTime.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("getFinishingTimesForSchedule failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((scheduleOut, finishingTime))
}

fn getTimeFinishedOfLastTask(mut threadTasksIn: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<metamodelica::Real> {
    let mut finTimeOut: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    finTimeOut = 'mc: {
        let __mc_input = threadTasksIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut lastTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
                    let mut finTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    lastTask = List::last(threadTasksIn.clone())?;
                    finTime = getTimeFinished(lastTask.clone());
                    Ok(finTime.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(metamodelica::OrderedFloat(-1.0_f64))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(finTimeOut)
}

fn computeTimeFinished(mut threadTasksIn: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, mut taskIdcsIn: metamodelica::Array<i32>, mut threadIdxIn: i32, mut checkedTasksIn: metamodelica::Array<Arc<HpcOmSimCode::Task>>, mut taskGraphIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphTIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphMetaIn: HpcOmTaskGraph::TaskGraphMeta, mut numProc: i32, mut closedThreadsIn: Arc<metamodelica::List<i32>>) -> Result<()> {
    let mut isCalc: bool = false;
    let mut isComputable: bool = false;
    let mut taskIdx: i32 = 0;
    let mut nextTaskIdx: i32 = 0;
    let mut threadIdx: i32 = threadIdxIn.clone();
    let mut taskIdcs: metamodelica::Array<i32>;
    let mut closedThreads: Arc<metamodelica::List<i32>> = closedThreadsIn.clone();
    let mut task: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> = threadTasksIn.clone();
    let mut checkedTasks: metamodelica::Array<Arc<HpcOmSimCode::Task>>;
    let mut thread: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    while !((closedThreads.clone().len() as i32) == numProc.clone()) {
        (threadIdx, closedThreads) = computeTimeFinished1(threadTasks.clone(), taskIdcsIn.clone(), threadIdx.clone(), checkedTasksIn.clone(), taskGraphIn.clone(), taskGraphTIn.clone(), taskGraphMetaIn.clone(), numProc.clone(), closedThreads.clone())?;
    }
    Ok(())
}

fn computeTimeFinished1(mut threadTasksIn: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, mut taskIdcsIn: metamodelica::Array<i32>, mut threadIdxIn: i32, mut checkedTasksIn: metamodelica::Array<Arc<HpcOmSimCode::Task>>, mut taskGraphIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphTIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphMetaIn: HpcOmTaskGraph::TaskGraphMeta, mut numProc: i32, mut closedThreadsIn: Arc<metamodelica::List<i32>>) -> Result<(i32, Arc<metamodelica::List<i32>>)> {
    let mut threadIdxOut: i32 = 0;
    let mut closedThreadsOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (threadIdxOut, closedThreadsOut) = 'mc: {
        let __mc_input = (threadTasksIn.clone(), taskIdcsIn.clone(), threadIdxIn.clone(), checkedTasksIn.clone(), taskGraphIn.clone(), taskGraphTIn.clone(), taskGraphMetaIn.clone(), numProc.clone(), closedThreadsIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _) => {
                    let mut taskIdx: i32 = 0;
                    let mut nextThreadIdx: i32 = 0;
                    let mut nextTaskIdx: i32 = 0;
                    let mut task: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
                    let mut thread: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
                    let true = (threadIdxIn.clone() <= (taskIdcsIn.clone().borrow().len() as i32)) else { bail!("pattern mismatch") };
                    taskIdx = taskIdcsIn.clone().borrow()[(threadIdxIn.clone()-1) as usize].clone();
                    thread = threadTasksIn.clone().borrow()[(threadIdxIn.clone()-1) as usize].clone();
                    let true = (taskIdx.clone() <= (thread.clone().len() as i32)) else { bail!("pattern mismatch") };
                    task = (thread.clone()).get(taskIdx.clone())?;
                    (_, _, nextTaskIdx) = updateFinishingTime(task.clone(), taskIdx.clone(), threadIdxIn.clone(), threadTasksIn.clone(), checkedTasksIn.clone(), taskGraphTIn.clone(), taskGraphMetaIn.clone())?;
                    let _ = {let _arr = taskIdcsIn.clone(); _arr.borrow_mut()[(threadIdxIn.clone()-1) as usize] = nextTaskIdx.clone(); _arr};
                    nextThreadIdx = getNextThreadIdx(threadIdxIn.clone(), closedThreadsIn.clone(), numProc.clone());
                    Ok((nextThreadIdx.clone(), closedThreadsIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _) => {
                    let mut nextThreadIdx: i32 = 0;
                    let true = (threadIdxIn.clone() > (taskIdcsIn.clone().borrow().len() as i32)) else { bail!("pattern mismatch") };
                    nextThreadIdx = if (intGe(threadIdxIn.clone(), numProc.clone())) {1} else {threadIdxIn.clone() + 1};
                    Ok((nextThreadIdx.clone(), closedThreadsIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _) => {
                    let mut taskIdx: i32 = 0;
                    let mut nextThreadIdx: i32 = 0;
                    let mut closedThreads1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut thread: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
                    let true = (threadIdxIn.clone() <= (taskIdcsIn.clone().borrow().len() as i32)) else { bail!("pattern mismatch") };
                    taskIdx = taskIdcsIn.clone().borrow()[(threadIdxIn.clone()-1) as usize].clone();
                    thread = threadTasksIn.clone().borrow()[(threadIdxIn.clone()-1) as usize].clone();
                    let true = (taskIdx.clone() > (thread.clone().len() as i32)) else { bail!("pattern mismatch") };
                    nextThreadIdx = if (intGe(threadIdxIn.clone(), numProc.clone())) {1} else {threadIdxIn.clone() + 1};
                    closedThreads1 = cons(threadIdxIn.clone(), closedThreadsIn.clone());
                    closedThreads1 = List::unique(closedThreads1.clone());
                    Ok((nextThreadIdx.clone(), closedThreads1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("computeTimeFinished failed!\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((threadIdxOut, closedThreadsOut))
}

#[tailcall::tailcall]
fn getNextThreadIdx(mut threadId: i32, mut closedThreads: Arc<metamodelica::List<i32>>, mut numThreads: i32) -> i32 {
    let mut isLastThread: bool = false;
    let mut isClosed: bool = false;
    let mut nextThread: i32 = 0;
    isLastThread = intEq(threadId.clone(), numThreads.clone());
    nextThread = if (isLastThread.clone()) {1} else {threadId.clone() + 1};
    isClosed = List::isMemberOnTrue(nextThread.clone(), closedThreads.clone(), Arc::new(fnptr!(intEq, i32, i32)));
    if (isClosed.clone()) {tailcall::call!{ getNextThreadIdx(nextThread.clone(), closedThreads.clone(), numThreads.clone()) }} else {nextThread.clone()}
}

fn updateFinishingTime(mut taskIn: Arc<HpcOmSimCode::Task>, mut taskIdxIn: i32, mut threadIdxIn: i32, mut threadTasksIn: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, mut checkedTasksIn: metamodelica::Array<Arc<HpcOmSimCode::Task>>, mut taskGraphTIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphMetaIn: HpcOmTaskGraph::TaskGraphMeta) -> Result<(metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, metamodelica::Array<Arc<HpcOmSimCode::Task>>, i32)> {
    let mut threadTasksOut: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut checkedTasksOut: metamodelica::Array<Arc<HpcOmSimCode::Task>>;
    let mut taskIdxOut: i32 = 0;
    (threadTasksOut, checkedTasksOut, taskIdxOut) = (::match_deref::match_deref! { match &((taskIn.clone(), taskIdxIn.clone(), threadIdxIn.clone(), threadTasksIn.clone(), checkedTasksIn.clone(), taskGraphTIn.clone(), taskGraphMetaIn.clone())) {
        (Deref @ HpcOmSimCode::Task::CALCTASK { index: taskID, .. }, _, _, _, _, _, _) => {
            let mut isComputable: bool = false;
            let mut taskIdxNew: i32 = 0;
            let mut parentLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut latestTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
            let mut checkedTasks: metamodelica::Array<Arc<HpcOmSimCode::Task>>;
            let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
            parentLst = taskGraphTIn.clone().borrow()[(taskID.clone()-1) as usize].clone();
            (parentLst, latestTask) = List::fold1(parentLst.clone(), Arc::new(updateFinishingTime1), checkedTasksIn.clone(), (metamodelica::nil(), Arc::new(crate::HpcOmSimCode::Task::TASKEMPTY)));
            isComputable = parentLst.clone().is_empty();
            taskIdxNew = if (isComputable.clone()) {taskIdxIn.clone() + 1} else {taskIdxIn.clone()};
            (threadTasks, checkedTasks) = if (isComputable.clone()) {computeFinishingTimeForOneTask((threadTasksIn.clone(), checkedTasksIn.clone(), taskIdxIn.clone(), threadIdxIn.clone(), latestTask.clone(), taskGraphMetaIn.clone()))?} else {(threadTasksIn.clone(), checkedTasksIn.clone())};
            (threadTasks.clone(), checkedTasks.clone(), taskIdxNew.clone())
        },
        (Deref @ HpcOmSimCode::Task::DEPTASK { .. }, _, _, _, _, _, _) => {
            let mut taskIdxNew: i32 = 0;
            taskIdxNew = taskIdxIn.clone() + 1;
            (threadTasksIn.clone(), checkedTasksIn.clone(), taskIdxNew.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((threadTasksOut, checkedTasksOut, taskIdxOut))
}

fn updateFinishingTime1(mut parentIdx: i32, mut checkedTaskIn: metamodelica::Array<Arc<HpcOmSimCode::Task>>, mut tplIn: (Arc<metamodelica::List<i32>>, Arc<HpcOmSimCode::Task>)) -> Result<(Arc<metamodelica::List<i32>>, Arc<HpcOmSimCode::Task>)> {
    let mut tplOut: (Arc<metamodelica::List<i32>>, Arc<HpcOmSimCode::Task>);
    let mut isCalc: bool = false;
    let mut finishingTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut finishingTime1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut finishingTimeIn: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut parentLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut parentLstIn: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut task: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut taskIn: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    (parentLstIn, taskIn) = tplIn.clone();
    finishingTimeIn = getTimeFinished(taskIn.clone());
    task = checkedTaskIn.clone().borrow()[(parentIdx.clone()-1) as usize].clone();
    isCalc = isCalcTask(task.clone());
    finishingTime = if (isCalc.clone()) {getTimeFinished(task.clone())} else {metamodelica::OrderedFloat(-1.0_f64)};
    task = if (realGt(finishingTime.clone(), finishingTimeIn.clone())) {task.clone()} else {taskIn.clone()};
    parentLst = if (isCalc.clone()) {parentLstIn.clone()} else {cons(parentIdx.clone(), parentLstIn.clone())};
    tplOut = (parentLst.clone(), task.clone());
    Ok(tplOut)
}

fn computeFinishingTimeForOneTask(mut tplIn: (metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, metamodelica::Array<Arc<HpcOmSimCode::Task>>, i32, i32, Arc<HpcOmSimCode::Task>, HpcOmTaskGraph::TaskGraphMeta)) -> Result<(metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, metamodelica::Array<Arc<HpcOmSimCode::Task>>)> {
    let mut tplOut: (metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, metamodelica::Array<Arc<HpcOmSimCode::Task>>);
    tplOut = 'mc: {
        let __mc_input = tplIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (threadTasksIn, checkedTasksIn, taskNum, threadIdx, latestTask, taskGraphMeta) => {
                    let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
                    let mut checkedTasks: metamodelica::Array<Arc<HpcOmSimCode::Task>>;
                    let mut taskIdx: i32 = 0;
                    let mut finishingTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut exeCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut task: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
                    let mut preTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
                    let mut thread: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
                    let mut threadIdx = (*threadIdx).clone();
                    let true = (isEmptyTask(latestTask.clone())) else { bail!("pattern mismatch") };
                    thread = threadTasksIn.clone().borrow()[(threadIdx.clone()-1) as usize].clone();
                    task = (thread.clone()).get(taskNum.clone())?;
                    threadIdx = getThreadId(task.clone());
                    preTask = getPredecessorCalcTask(thread.clone(), taskNum.clone())?;
                    finishingTime = getTimeFinished(preTask.clone());
                    taskIdx = getTaskIdx(task.clone());
                    (_, exeCost) = HpcOmTaskGraph::getExeCost(taskIdx.clone(), taskGraphMeta.clone())?;
                    finishingTime = finishingTime.clone() + exeCost.clone();
                    task = updateTimeFinished(task.clone(), finishingTime.clone())?;
                    thread = List::replaceAt(task.clone(), taskNum.clone(), thread.clone())?;
                    threadTasks = {let _arr = threadTasksIn.clone(); _arr.borrow_mut()[(threadIdx.clone()-1) as usize] = thread.clone(); _arr};
                    checkedTasks = {let _arr = checkedTasksIn.clone(); _arr.borrow_mut()[(taskIdx.clone()-1) as usize] = task.clone(); _arr};
                    Ok((threadTasks.clone(), checkedTasks.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (threadTasksIn, checkedTasksIn, taskNum, threadIdx, latestTask, taskGraphMeta) => {
                    let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
                    let mut checkedTasks: metamodelica::Array<Arc<HpcOmSimCode::Task>>;
                    let mut taskIdx: i32 = 0;
                    let mut taskIdxLatest: i32 = 0;
                    let mut threadIdxLatest: i32 = 0;
                    let mut commCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut finishingTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut finishingTime1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut finishingTimeComm: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut exeCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut task: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
                    let mut preTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
                    let mut thread: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
                    let false = (isEmptyTask(latestTask.clone())) else { bail!("pattern mismatch") };
                    finishingTime = getTimeFinished(latestTask.clone());
                    threadIdxLatest = getThreadId(latestTask.clone());
                    taskIdxLatest = getTaskIdx(latestTask.clone());
                    thread = threadTasksIn.clone().borrow()[(threadIdx.clone()-1) as usize].clone();
                    task = (thread.clone()).get(taskNum.clone())?;
                    taskIdx = getTaskIdx(task.clone());
                    commCost = HpcOmTaskGraph::getCommCostTimeBetweenNodes(taskIdxLatest.clone(), taskIdx.clone(), taskGraphMeta.clone())?;
                    (_, exeCost) = HpcOmTaskGraph::getExeCost(taskIdx.clone(), taskGraphMeta.clone())?;
                    finishingTime = finishingTime.clone() + exeCost.clone();
                    finishingTimeComm = finishingTime.clone() + commCost.clone();
                    finishingTime = if (intEq(threadIdxLatest.clone(), threadIdx.clone())) {finishingTime.clone()} else {finishingTimeComm.clone()};
                    preTask = getPredecessorCalcTask(thread.clone(), taskNum.clone())?;
                    finishingTime1 = getTimeFinished(preTask.clone());
                    finishingTime1 = finishingTime1.clone() + exeCost.clone();
                    finishingTime = realMax(finishingTime.clone(), finishingTime1.clone());
                    task = updateTimeFinished(task.clone(), finishingTime.clone())?;
                    thread = List::replaceAt(task.clone(), taskNum.clone(), thread.clone())?;
                    threadTasks = {let _arr = threadTasksIn.clone(); _arr.borrow_mut()[(threadIdx.clone()-1) as usize] = thread.clone(); _arr};
                    checkedTasks = {let _arr = checkedTasksIn.clone(); _arr.borrow_mut()[(taskIdx.clone()-1) as usize] = task.clone(); _arr};
                    Ok((threadTasks.clone(), checkedTasks.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(tplOut)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getPredecessorCalcTask(mut threadIn: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut indexIn: i32) -> Result<Arc<HpcOmSimCode::Task>> {
    let mut taskOut: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    taskOut = 'mc: {
        let __mc_input = (threadIn.clone(), indexIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let true = (indexIn.clone() == 1) else { bail!("pattern mismatch") };
                    Ok(Arc::new(crate::HpcOmSimCode::Task::TASKEMPTY))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut isCalc: bool = false;
                    let mut index: i32 = 0;
                    let mut preTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
                    let true = (indexIn.clone() >= 2) else { bail!("pattern mismatch") };
                    index = indexIn.clone() - 1;
                    preTask = (threadIn.clone()).get(index.clone())?;
                    isCalc = isCalcTask(preTask.clone());
                    preTask = if (boolNot(isCalc.clone())) {getPredecessorCalcTask(threadIn.clone(), index.clone())?} else {preTask.clone()};
                    Ok(preTask.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(taskOut)
}

fn updateTimeFinished(mut taskIn: Arc<HpcOmSimCode::Task>, mut timeFinishedIn: metamodelica::Real) -> Result<Arc<HpcOmSimCode::Task>> {
    let mut taskOut: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut weighting: i32 = 0;
    let mut index: i32 = 0;
    let mut calcTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut timeFinished: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut threadIdx: i32 = 0;
    let mut eqIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(taskIn.clone()) {
        Deref @ HpcOmSimCode::Task::CALCTASK { eqIdc: __pa0, threadIdx: __pa1, timeFinished: __pa2, calcTime: __pa3, index: __pa4, weighting: __pa5 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
        _ => bail!("pattern mismatch"),
    } };
    eqIdc = __pa0.clone();
    threadIdx = __pa1.clone();
    timeFinished = __pa2.clone();
    calcTime = __pa3.clone();
    index = __pa4.clone();
    weighting = __pa5.clone();
    taskOut = Arc::new(HpcOmSimCode::Task::CALCTASK { weighting: weighting.clone(), index: index.clone(), calcTime: calcTime.clone(), timeFinished: timeFinishedIn.clone(), threadIdx: threadIdx.clone(), eqIdc: eqIdc.clone() });
    Ok(taskOut)
}

fn getTimeFinished(mut taskIn: Arc<HpcOmSimCode::Task>) -> metamodelica::Real {
    let mut finishingTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    finishingTime = (::match_deref::match_deref! { match &(taskIn.clone()) {
        Deref @ HpcOmSimCode::Task::CALCTASK { timeFinished: fTime, .. } => {
            fTime.clone()
        },
        Deref @ HpcOmSimCode::Task::TASKEMPTY => {
            metamodelica::OrderedFloat(0.0_f64)
        },
        _ => {
            metamodelica::OrderedFloat(-1.0_f64)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    finishingTime
}

fn getThreadId(mut taskIn: Arc<HpcOmSimCode::Task>) -> i32 {
    let mut threadId: i32 = 0;
    threadId = (::match_deref::match_deref! { match &(taskIn.clone()) {
        Deref @ HpcOmSimCode::Task::CALCTASK { threadIdx, .. } => {
            threadIdx.clone()
        },
        _ => {
            -1
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    threadId
}

fn getTaskIdx(mut taskIn: Arc<HpcOmSimCode::Task>) -> i32 {
    let mut idx: i32 = 0;
    idx = (::match_deref::match_deref! { match &(taskIn.clone()) {
        Deref @ HpcOmSimCode::Task::CALCTASK { index: taskIdx, .. } => {
            taskIdx.clone()
        },
        _ => {
            -1
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    idx
}

fn getTaskTypeString(mut iTask: Arc<HpcOmSimCode::Task>) -> ArcStr {
    let mut oTypeString: ArcStr = arcstr::literal!("");
    oTypeString = ((::match_deref::match_deref! { match &(iTask.clone()) {
        Deref @ HpcOmSimCode::Task::SCHEDULED_TASK { .. } => literal!("Scheduled task"),
        Deref @ HpcOmSimCode::Task::CALCTASK { .. } => literal!("Calctask"),
        Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { .. } => literal!("Calctask level"),
        Deref @ HpcOmSimCode::Task::DEPTASK { .. } => literal!("Deptask"),
        Deref @ HpcOmSimCode::Task::PREFETCHTASK { .. } => literal!("Prefetch task"),
        Deref @ HpcOmSimCode::Task::TASKEMPTY => literal!("Empty task"),
        _ => literal!("Unknown"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    oTypeString
}

fn isCalcTask(mut taskIn: Arc<HpcOmSimCode::Task>) -> bool {
    let mut isCalc: bool = false;
    isCalc = (::match_deref::match_deref! { match &(taskIn.clone()) {
        Deref @ HpcOmSimCode::Task::CALCTASK { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isCalc
}

fn isEmptyTask(mut taskIn: Arc<HpcOmSimCode::Task>) -> bool {
    let mut isEmpty: bool = false;
    isEmpty = (::match_deref::match_deref! { match &(taskIn.clone()) {
        Deref @ HpcOmSimCode::Task::TASKEMPTY => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isEmpty
}

pub fn convertFixedLevelScheduleToLevelThreadLists(mut iSchedule: Arc<HpcOmSimCode::Schedule>, mut iNumOfThreads: i32) -> Arc<metamodelica::List<metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>> {
    let mut oLevelThreadLists: Arc<metamodelica::List<metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>> = metamodelica::nil();
    let mut tasksOfLevels: Arc<metamodelica::List<HpcOmSimCode::TaskList>> = metamodelica::nil();
    let mut tmpLevelThreadLists: Arc<metamodelica::List<metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>> = metamodelica::nil();
    oLevelThreadLists = (::match_deref::match_deref! { match &((iSchedule.clone(), iNumOfThreads.clone())) {
        (Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { useFixedAssignments: true, tasksOfLevels }, _) => {
            tmpLevelThreadLists = List::map(tasksOfLevels.clone(), Arc::new({ let __pe_b1 = iNumOfThreads.clone(); move |__pe_a0| convertFixedLevelScheduleToLevelThreadLists0(__pe_a0, __pe_b1.clone()) }));
            tmpLevelThreadLists.clone()
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oLevelThreadLists
}

fn convertFixedLevelScheduleToLevelThreadLists0(mut iTasksOfLevel: HpcOmSimCode::TaskList, mut iNumOfThreads: i32) -> Result<metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> {
    let mut oLevelThreadLists: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut tasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut task: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut threadIdx: i32 = 0;
    let mut tmpLevelThreadLists: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    tasks = getTasksOfTaskList(iTasksOfLevel.clone());
    tmpLevelThreadLists = arrayCreate(iNumOfThreads.clone(), metamodelica::nil());
    for mut task in &*tasks.clone().reverse() {
        let mut task = task.clone();
        let __pa0 = ::match_deref::match_deref! { match &(task.clone()) {
            Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { threadIdx: Some(__pa0), .. } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        threadIdx = __pa0.clone();
        tmpLevelThreadLists = {let _arr = tmpLevelThreadLists.clone(); let _val = cons(task.clone(), tmpLevelThreadLists.clone().borrow()[(threadIdx.clone()-1) as usize].clone()); _arr.borrow_mut()[(threadIdx.clone()-1) as usize] = _val; _arr};
    }
    oLevelThreadLists = tmpLevelThreadLists.clone();
    Ok(oLevelThreadLists)
}

pub fn convertFixedLevelScheduleToTaskLists(mut iOdeSchedule: Arc<HpcOmSimCode::Schedule>, mut iDaeSchedule: Arc<HpcOmSimCode::Schedule>, mut iZeroFuncSchedule: Arc<HpcOmSimCode::Schedule>, mut iNumOfThreads: i32) -> Result<metamodelica::Array<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>> {
    let mut oThreadLevelTasks: metamodelica::Array<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>;
    let mut tasksOfLevelsOde: Arc<metamodelica::List<HpcOmSimCode::TaskList>> = metamodelica::nil();
    let mut tasksOfLevelsDae: Arc<metamodelica::List<HpcOmSimCode::TaskList>> = metamodelica::nil();
    let mut tasksOfLevelsZeroFunc: Arc<metamodelica::List<HpcOmSimCode::TaskList>> = metamodelica::nil();
    let mut tmpThreadLevelTasksDae: Arc<metamodelica::List<metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>> = metamodelica::nil();
    let mut tmpThreadLevelTasksOde: Arc<metamodelica::List<metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>> = metamodelica::nil();
    let mut tmpThreadLevelTasksZeroFunc: Arc<metamodelica::List<metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>> = metamodelica::nil();
    let mut tmpResultLists: metamodelica::Array<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>;
    oThreadLevelTasks = (::match_deref::match_deref! { match &((iOdeSchedule.clone(), iDaeSchedule.clone(), iZeroFuncSchedule.clone(), iNumOfThreads.clone())) {
        (Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { useFixedAssignments: true, tasksOfLevels: tasksOfLevelsOde }, Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { useFixedAssignments: true, tasksOfLevels: tasksOfLevelsDae }, Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { useFixedAssignments: true, tasksOfLevels: tasksOfLevelsZeroFunc }, _) => {
            tmpResultLists = arrayCreate(iNumOfThreads.clone(), (metamodelica::nil(), metamodelica::nil(), metamodelica::nil()));
            tmpThreadLevelTasksOde = List::map1(tasksOfLevelsOde.clone(), Arc::new(convertFixedLevelScheduleToTaskListsForLevel), iNumOfThreads.clone());
            tmpThreadLevelTasksDae = List::map1(tasksOfLevelsDae.clone(), Arc::new(convertFixedLevelScheduleToTaskListsForLevel), iNumOfThreads.clone());
            tmpThreadLevelTasksZeroFunc = List::map1(tasksOfLevelsZeroFunc.clone(), Arc::new(convertFixedLevelScheduleToTaskListsForLevel), iNumOfThreads.clone());
            tmpResultLists = List::fold(tmpThreadLevelTasksOde.clone(), Arc::new({ let __pe_b1 = 1; let __pe_b2 = 0; move |__pe_a0, __pe_a3| convertFixedLevelScheduleToTaskLists1(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }), tmpResultLists.clone());
            tmpResultLists = List::fold(tmpThreadLevelTasksDae.clone(), Arc::new({ let __pe_b1 = 1; let __pe_b2 = 1; move |__pe_a0, __pe_a3| convertFixedLevelScheduleToTaskLists1(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }), tmpResultLists.clone());
            tmpResultLists = List::fold(tmpThreadLevelTasksZeroFunc.clone(), Arc::new({ let __pe_b1 = 1; let __pe_b2 = 2; move |__pe_a0, __pe_a3| convertFixedLevelScheduleToTaskLists1(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }), tmpResultLists.clone());
            tmpResultLists = revertTaskLists(1, tmpResultLists.clone())?;
            tmpResultLists.clone()
        },
        _ => {
            tmpResultLists = arrayCreate(iNumOfThreads.clone(), (metamodelica::nil(), metamodelica::nil(), metamodelica::nil()));
            tmpResultLists.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oThreadLevelTasks)
}

fn convertFixedLevelScheduleToTaskLists1(mut iLevelTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, mut iCurrentThreadIdx: i32, mut iModifiedSystemIdx: i32, mut iResultList: metamodelica::Array<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>) -> Result<metamodelica::Array<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>> {
    let mut oResultList: metamodelica::Array<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>;
    let mut tmpResultList: metamodelica::Array<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>;
    let mut entryOde: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
    let mut entryDae: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
    let mut entryZeroFunc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
    oResultList = 'mc: {
        let __mc_input = (iLevelTasks.clone(), iCurrentThreadIdx.clone(), iModifiedSystemIdx.clone(), iResultList.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut entryDae: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = entryDae.clone();
            let mut entryOde: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = entryOde.clone();
            let mut entryZeroFunc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = entryZeroFunc.clone();
            let mut tmpResultList: metamodelica::Array<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>;
            let true = (intLe(iCurrentThreadIdx.clone(), (iLevelTasks.clone().borrow().len() as i32))) else { bail!("pattern mismatch") };
            (entryOde, entryDae, entryZeroFunc) = iResultList.clone().borrow()[(iCurrentThreadIdx.clone()-1) as usize].clone();
            if intEq(iModifiedSystemIdx.clone(), 0) {
                entryOde = cons(iLevelTasks.clone().borrow()[(iCurrentThreadIdx.clone()-1) as usize].clone(), entryOde.clone());
            } else {
                if intEq(iModifiedSystemIdx.clone(), 1) {
                    entryDae = cons(iLevelTasks.clone().borrow()[(iCurrentThreadIdx.clone()-1) as usize].clone(), entryDae.clone());
                } else {
                    entryZeroFunc = cons(iLevelTasks.clone().borrow()[(iCurrentThreadIdx.clone()-1) as usize].clone(), entryZeroFunc.clone());
                }
            }
            tmpResultList = {let _arr = iResultList.clone(); _arr.borrow_mut()[(iCurrentThreadIdx.clone()-1) as usize] = (entryOde.clone(), entryDae.clone(), entryZeroFunc.clone()); _arr};
            tmpResultList = convertFixedLevelScheduleToTaskLists1(iLevelTasks.clone(), iCurrentThreadIdx.clone() + 1, iModifiedSystemIdx.clone(), tmpResultList.clone())?;
            Ok(tmpResultList.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(iResultList.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oResultList)
}

fn revertTaskLists(mut iCurrentArrayIdx: i32, mut iResultList: metamodelica::Array<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>) -> Result<metamodelica::Array<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>> {
    let mut oResultList: metamodelica::Array<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>;
    let mut entryOde: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
    let mut entryDae: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
    let mut entryZeroFunc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
    let mut tmpResultList: metamodelica::Array<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>;
    oResultList = 'mc: {
        let __mc_input = (iCurrentArrayIdx.clone(), iResultList.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut entryZeroFunc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = entryZeroFunc.clone();
            let mut entryDae: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = entryDae.clone();
            let mut entryOde: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = entryOde.clone();
            let mut tmpResultList: metamodelica::Array<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>;
            let true = (intLe(iCurrentArrayIdx.clone(), (iResultList.clone().borrow().len() as i32))) else { bail!("pattern mismatch") };
            (entryOde, entryDae, entryZeroFunc) = iResultList.clone().borrow()[(iCurrentArrayIdx.clone()-1) as usize].clone();
            entryOde = entryOde.clone().reverse();
            entryDae = entryDae.clone().reverse();
            entryZeroFunc = entryZeroFunc.clone().reverse();
            tmpResultList = {let _arr = iResultList.clone(); _arr.borrow_mut()[(iCurrentArrayIdx.clone()-1) as usize] = (entryOde.clone(), entryDae.clone(), entryZeroFunc.clone()); _arr};
            tmpResultList = revertTaskLists(iCurrentArrayIdx.clone() + 1, tmpResultList.clone())?;
            Ok(tmpResultList.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(iResultList.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oResultList)
}

fn revertTaskList(mut iCurrentArrayIdx: i32, mut iResultList: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> {
    let mut oResultList: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut entry: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut tmpResultList: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    oResultList = 'mc: {
        let __mc_input = (iCurrentArrayIdx.clone(), iResultList.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut entry: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = entry.clone();
            let mut tmpResultList: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
            let true = (intLe(iCurrentArrayIdx.clone(), (iResultList.clone().borrow().len() as i32))) else { bail!("pattern mismatch") };
            entry = iResultList.clone().borrow()[(iCurrentArrayIdx.clone()-1) as usize].clone();
            entry = entry.clone().reverse();
            tmpResultList = {let _arr = iResultList.clone(); _arr.borrow_mut()[(iCurrentArrayIdx.clone()-1) as usize] = entry.clone(); _arr};
            Ok(tmpResultList.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(iResultList.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oResultList)
}

//----------------
//  LockIdSetter
//----------------
fn setScheduleLockIds(mut iSchedule: Arc<HpcOmSimCode::Schedule>) -> Result<Arc<HpcOmSimCode::Schedule>> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut allThreadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut tmpFoldArray: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut newAllThreadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut scheduledTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut lockIds: metamodelica::Array<Arc<metamodelica::List<(i32, i32)>>>;
    let mut outgoingDepTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut newOutgoingDepTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
    let mut newTuple: (i32, i32);
    let mut sourceTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut targetTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut iterTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut counter: i32 = 0;
    let mut id: i32 = 0;
    let mut sourceTaskId: i32 = 0;
    let mut targetTaskId: i32 = 0;
    let mut outgoing: bool = false;
    let mut communicationInfo: HpcOmSimCode::CommunicationInfo;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(iSchedule.clone()) {
        Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: __pa0, outgoingDepTasks: __pa1, scheduledTasks: __pa2, allCalcTasks: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    allThreadTasks = __pa0.clone();
    outgoingDepTasks = __pa1.clone();
    scheduledTasks = __pa2.clone();
    allCalcTasks = __pa3.clone();
    lockIds = arrayCreate((allCalcTasks.clone().borrow().len() as i32), metamodelica::nil());
    newAllThreadTasks = arrayCreate((allThreadTasks.clone().borrow().len() as i32), metamodelica::nil());
    counter = 0;
    for mut iterTask in &*outgoingDepTasks.clone() {
        let mut iterTask = iterTask.clone();
        let (__pa4, __pa5, __pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &(iterTask.clone()) {
            Deref @ HpcOmSimCode::Task::DEPTASK { communicationInfo: __pa4, id: __pa5, outgoing: __pa6, targetTask: __pa7, sourceTask: __pa8 } => (__pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone()),
            _ => bail!("pattern mismatch"),
        } };
        communicationInfo = __pa4.clone();
        id = __pa5.clone();
        outgoing = __pa6.clone();
        targetTask = __pa7.clone();
        sourceTask = __pa8.clone();
        let __pa9 = ::match_deref::match_deref! { match &(sourceTask.clone()) {
            Deref @ HpcOmSimCode::Task::CALCTASK { index: __pa9, .. } => __pa9.clone(),
            _ => bail!("pattern mismatch"),
        } };
        sourceTaskId = __pa9.clone();
        let __pa10 = ::match_deref::match_deref! { match &(targetTask.clone()) {
            Deref @ HpcOmSimCode::Task::CALCTASK { index: __pa10, .. } => __pa10.clone(),
            _ => bail!("pattern mismatch"),
        } };
        targetTaskId = __pa10.clone();
        newTuple = (targetTaskId.clone(), counter.clone());
        {let _arr = lockIds.clone(); let _val = listAppend(lockIds.clone().borrow()[(sourceTaskId.clone()-1) as usize].clone(), list![newTuple.clone()]); _arr.borrow_mut()[(sourceTaskId.clone()-1) as usize] = _val; _arr};
        newOutgoingDepTasks = cons(Arc::new(HpcOmSimCode::Task::DEPTASK { sourceTask: sourceTask.clone(), targetTask: targetTask.clone(), outgoing: outgoing.clone(), id: counter.clone(), communicationInfo: communicationInfo.clone() }), newOutgoingDepTasks.clone());
        counter = counter.clone() + 1;
    }
    tmpFoldArray = arrayCreate((allThreadTasks.clone().borrow().len() as i32), metamodelica::nil());
    (newAllThreadTasks, _) = Array::fold(allThreadTasks.clone(), Arc::new({ let __pe_b1 = lockIds.clone(); move |__pe_a0, __pe_a2| replaceDepTaskIdsByLockIds(__pe_a0, __pe_b1.clone(), __pe_a2) }), (tmpFoldArray.clone(), 1));
    oSchedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: newAllThreadTasks.clone(), outgoingDepTasks: newOutgoingDepTasks.clone(), scheduledTasks: scheduledTasks.clone(), allCalcTasks: allCalcTasks.clone() });
    Ok(oSchedule)
}

fn replaceDepTaskIdsByLockIds(mut inTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut lockIds: metamodelica::Array<Arc<metamodelica::List<(i32, i32)>>>, mut iAllThreadTasks: (metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, i32)) -> Result<(metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, i32)> {
    let mut oTasks: (metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, i32);
    let mut allThreadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut tmpList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut threadId: i32 = 0;
    (allThreadTasks, threadId) = iAllThreadTasks.clone();
    tmpList = List::fold(inTasks.clone(), Arc::new({ let __pe_b1 = lockIds.clone(); move |__pe_a0, __pe_a2| replaceDepTasksInListByLockIds(__pe_a0, __pe_b1.clone(), __pe_a2) }), metamodelica::nil()).reverse();
    {let _arr = allThreadTasks.clone(); _arr.borrow_mut()[(threadId.clone()-1) as usize] = tmpList.clone(); _arr};
    oTasks = (allThreadTasks.clone(), threadId.clone() + 1);
    Ok(oTasks)
}

fn replaceDepTasksInListByLockIds(mut inTask: Arc<HpcOmSimCode::Task>, mut lockIds: metamodelica::Array<Arc<metamodelica::List<(i32, i32)>>>, mut tmpTaskList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> {
    let mut oList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut tmpTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    tmpTask = findTaskWithLockId(lockIds.clone(), inTask.clone())?;
    oList = cons(tmpTask.clone(), tmpTaskList.clone());
    Ok(oList)
}

fn findTaskWithLockId(mut lockIds: metamodelica::Array<Arc<metamodelica::List<(i32, i32)>>>, mut iTask: Arc<HpcOmSimCode::Task>) -> Result<Arc<HpcOmSimCode::Task>> {
    let mut oTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut tmpTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut sourceTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut targetTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut outgoing: bool = false;
    let mut lockId: i32 = 0;
    let mut sourceTaskId: i32 = 0;
    let mut targetTaskId: i32 = 0;
    let mut communicationInfo: HpcOmSimCode::CommunicationInfo;
    oTask = (::match_deref::match_deref! { match &(iTask.clone()) {
        Deref @ HpcOmSimCode::Task::DEPTASK { communicationInfo, outgoing, targetTask, sourceTask, .. } => {
            let __pa0 = ::match_deref::match_deref! { match &(sourceTask.clone()) {
                Deref @ HpcOmSimCode::Task::CALCTASK { index: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            sourceTaskId = __pa0.clone();
            let __pa1 = ::match_deref::match_deref! { match &(targetTask.clone()) {
                Deref @ HpcOmSimCode::Task::CALCTASK { index: __pa1, .. } => __pa1.clone(),
                _ => bail!("pattern mismatch"),
            } };
            targetTaskId = __pa1.clone();
            lockId = findInIntTuple1(lockIds.clone().borrow()[(sourceTaskId.clone()-1) as usize].clone(), targetTaskId.clone());
            tmpTask = Arc::new(HpcOmSimCode::Task::DEPTASK { sourceTask: sourceTask.clone(), targetTask: targetTask.clone(), outgoing: outgoing.clone(), id: lockId.clone(), communicationInfo: communicationInfo.clone() });
            tmpTask.clone()
        },
        _ => iTask.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oTask)
}

fn findInIntTuple1(mut liste: Arc<metamodelica::List<(i32, i32)>>, mut toFind: i32) -> i32 {
    let mut secondElement: i32 = 0;
    let mut first: i32 = 0;
    let mut second: i32 = 0;
    let mut iter: (i32, i32);
    for mut iter in &*liste.clone() {
        let mut iter = iter.clone();
        (first, second) = iter.clone();
        if intEq(first.clone(), toFind.clone()) {
            secondElement = second.clone();
            return secondElement;
        }
    }
    secondElement
}

fn convertFixedLevelScheduleToTaskListsForLevel(mut iTasksOfLevel: HpcOmSimCode::TaskList, mut iThreadCount: i32) -> Result<metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> {
    let mut oThreadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut tmpTaskLists: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut tasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    oThreadTasks = (match (iTasksOfLevel.clone(), iThreadCount.clone()) {
        (HpcOmSimCode::TaskList::PARALLELTASKLIST { tasks: mut tasks }, _) => {
            tmpTaskLists = arrayCreate(iThreadCount.clone(), metamodelica::nil());
            tmpTaskLists = List::fold(tasks.clone(), Arc::new(convertFixedLevelScheduleToTaskListsForTask), tmpTaskLists.clone());
            tmpTaskLists = revertTaskList(1, tmpTaskLists.clone())?;
            tmpTaskLists.clone()
        },
        (HpcOmSimCode::TaskList::SERIALTASKLIST { tasks: mut tasks, .. }, _) => {
            tmpTaskLists = arrayCreate(iThreadCount.clone(), metamodelica::nil());
            tmpTaskLists = {let _arr = tmpTaskLists.clone(); _arr.borrow_mut()[(1-1) as usize] = tasks.clone(); _arr};
            tmpTaskLists.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(oThreadTasks)
}

fn convertFixedLevelScheduleToTaskListsForTask(mut iTask: Arc<HpcOmSimCode::Task>, mut iThreadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> {
    let mut oThreadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut tmpTaskLists: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut threadIdx: i32 = 0;
    let mut oldTaskList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    oThreadTasks = (::match_deref::match_deref! { match &((iTask.clone(), iThreadTasks.clone())) {
        (Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { threadIdx: Some(threadIdx), .. }, _) => {
            oldTaskList = iThreadTasks.clone().borrow()[(threadIdx.clone()-1) as usize].clone();
            tmpTaskLists = {let _arr = iThreadTasks.clone(); _arr.borrow_mut()[(threadIdx.clone()-1) as usize] = cons(iTask.clone(), oldTaskList.clone()); _arr};
            tmpTaskLists.clone()
        },
        (_, _) => {
            println!("{}", (literal!("ConvertFixedLevelScheduleToTaskListsForTask can just handle CALCTASK_LEVEL with defined thread idx!\n")).clone());
            iThreadTasks.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oThreadTasks)
}

fn printRealArray(mut inArray: metamodelica::Array<metamodelica::Real>, mut header: ArcStr) -> () {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The ")); __mm_s.push_str(&*header.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    println!("{}", (literal!("-----------------------------------------\n")).clone());
    let _ = Array::fold(inArray.clone(), Arc::new({ let __pe_b1 = (header.clone()).clone(); move |__pe_a0, __pe_a2| Ok(printRealArray1(__pe_a0, __pe_b1.clone(), __pe_a2)) }), 1);
    println!("{}", (literal!("\n")).clone());
    ()
}

fn printRealArray1(mut inValue: metamodelica::Real, mut header: ArcStr, mut idxIn: i32) -> i32 {
    let mut idxOut: i32 = 0;
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("node: ")); __mm_s.push_str(&*intString(idxIn.clone())); __mm_s.push_str(&*literal!(" has the ")); __mm_s.push_str(&*header.clone()); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*realString(inValue.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    idxOut = idxIn.clone() + 1;
    idxOut
}

fn intListString(mut lstIn: Arc<metamodelica::List<i32>>) -> ArcStr {
    let mut s: ArcStr = arcstr::literal!("");
    s = stringDelimitList(List::map(lstIn.clone(), Arc::new(fnptr!(intString, i32))), (literal!(" , ")).clone());
    s = (if (lstIn.clone().is_empty()) {literal!("{}")} else {s.clone()}).clone();
    s
}

fn intListListString(mut lstIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> ArcStr {
    let mut s: ArcStr = arcstr::literal!("");
    s = stringDelimitList(List::map(lstIn.clone(), Arc::new(fnptr!(intListString, Arc<metamodelica::List<i32>>))), (literal!(" | ")).clone());
    s
}

pub fn expandSchedule(mut iNumProc: i32, mut iNumUsedProc: i32, mut iSchedule: Arc<HpcOmSimCode::Schedule>) -> Result<Arc<HpcOmSimCode::Schedule>> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut outgoingDepTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut scheduledTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
    oSchedule = (::match_deref::match_deref! { match &((iNumProc.clone(), iNumUsedProc.clone(), iSchedule.clone())) {
        (_, _, Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { .. }) => iSchedule.clone(),
        (_, _, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { allCalcTasks, scheduledTasks, outgoingDepTasks, threadTasks }) => {
            let mut threadTasks = (*threadTasks).clone();
            threadTasks = Array::expandToSize(iNumProc.clone(), threadTasks.clone(), metamodelica::nil())?;
            Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: scheduledTasks.clone(), allCalcTasks: allCalcTasks.clone() })
        },
        (_, _, Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { .. }) => iSchedule.clone(),
        (_, _, Deref @ HpcOmSimCode::Schedule::EMPTYSCHEDULE { .. }) => iSchedule.clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(oSchedule)
}

