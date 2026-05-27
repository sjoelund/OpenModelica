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
use crate::BackendEquation;
use crate::BackendVariable;
use crate::HpcOmBenchmark;
use crate::HpcOmScheduler;
use crate::HpcOmSimCode;
use crate::SimCode;
use crate::SimCodeUtil;
use crate::SimCodeVar;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::Expression;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::DAEDumpTypes;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_susan::GraphML;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

//----------------------------
//  Graph Structure
//----------------------------
pub type TaskGraph = metamodelica::Array<Arc<metamodelica::List<i32>>>;

pub type Communications = Arc<metamodelica::List<Communication>>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Communication {
    pub numberOfVars: i32,
    pub integerVars: Arc<metamodelica::List<i32>>,
    pub floatVars: Arc<metamodelica::List<i32>>,
    pub booleanVars: Arc<metamodelica::List<i32>>,
    pub stringVars: Arc<metamodelica::List<i32>>,
    pub childNode: i32,
    pub requiredTime: metamodelica::Real,
}

pub type COMMUNICATION = Communication;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ComponentInfo {
    pub isPartOfODESystem: bool,
    pub isPartOfZeroFuncSystem: bool,
    pub isRemovedComponent: bool,
}

pub type COMPONENTINFO = ComponentInfo;


// TODO: Store compParamMapping, compNames and compDescs in ComponentInfo
// TODO: Change nodeMark to compMarks
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskGraphMeta {
    pub inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>,
    pub varCompMapping: metamodelica::Array<(i32, i32, i32)>,
    pub eqCompMapping: metamodelica::Array<(i32, i32, i32)>,
    pub compParamMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>,
    pub compNames: metamodelica::Array<ArcStr>,
    pub compDescs: metamodelica::Array<ArcStr>,
    pub exeCosts: metamodelica::Array<(i32, metamodelica::Real)>,
    pub commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>,
    pub nodeMark: metamodelica::Array<i32>,
    pub compInformations: metamodelica::Array<ComponentInfo>,
}

pub type TASKGRAPHMETA = TaskGraphMeta;


#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum VariableType {
    INTEGER = 1,
    REAL = 2,
    BOOLEAN = 3,
    STRING = 4,
}
impl PartialOrd for VariableType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for VariableType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

pub type VariableList = (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);

//variables <int, float, bool, string>
//----------------------------------------------------------
//  Functions to build the task graph from the BLT structure
//----------------------------------------------------------
pub fn createTaskGraph(mut iDAE: Arc<BackendDAE::BackendDAE>, mut iAnalyzeParameters: bool) -> Result<(TaskGraph, TaskGraphMeta)> {
    let mut oGraph: TaskGraph;
    let mut oGraphData: TaskGraphMeta;
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared>;
    let mut graph: TaskGraph;
    let mut graphData: TaskGraphMeta;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(iDAE.clone()) {
        Deref @ DAE { UNIQUEIO: __pa0, derivativeNamePrefix: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    systs = __pa0.clone();
    shared = __pa1.clone();
    (graph, graphData) = getEmptyTaskGraph(0, 0, 0);
    (oGraph, oGraphData, _) = List::fold(systs.clone(), Arc::new(todo!("PARTEVALFUNCTION of createTaskGraph0: function signature not resolved")), (graph.clone(), graphData.clone(), 1));
    Ok((oGraph, oGraphData))
}

pub fn getSystemComponents(mut iDae: Arc<BackendDAE::BackendDAE>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, metamodelica::Array<(Arc<BackendDAE::EqSystem>, i32)>)> {
    let mut oComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut oMapping: metamodelica::Array<(Arc<BackendDAE::EqSystem>, i32)>;
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut tmpSystems: Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>> = metamodelica::nil();
    let mut tmpComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    (oComps, oMapping) = (::match_deref::match_deref! { match &(iDae.clone()) {
        Deref @ DAE { eqs: systs, .. } => {
            (tmpComps, tmpSystems, _) = List::fold(systs.clone(), (std::sync::Arc::new(getSystemComponents0) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, (Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>, i32)) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>, i32)> + 'static>), (metamodelica::nil(), metamodelica::nil(), 1));
            (tmpComps.clone(), metamodelica::arrayFromVec(tmpSystems.clone().into_iter().cloned().collect()))
        },
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oComps, oMapping))
}

fn getSystemComponents0(mut iSyst: Arc<BackendDAE::EqSystem>, mut iSystMapping: (Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>, i32)) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>, i32)> {
    let mut oSystMapping: (Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>, i32);
    let mut tmpComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut matching: Arc<BackendDAE::Matching> = Arc::new(BackendDAE::Matching::NO_MATCHING);
    let mut tmpSystMapping: Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>> = metamodelica::nil();
    let mut currentIdx: i32 = 0;
    oSystMapping = (::match_deref::match_deref! { match &((iSyst.clone(), iSystMapping.clone())) {
        (Deref @ BackendDAE::EqSystem { matching, .. }, (tmpComps, tmpSystMapping, currentIdx)) => {
            let mut tmpSystMapping = (*tmpSystMapping).clone();
            comps = BackendDAEUtil::getCompsOfMatching(matching.clone());
            tmpSystMapping = List::fold2(comps.clone(), (std::sync::Arc::new(fnptr!(getSystemComponents1, Arc<BackendDAE::StrongComponent>, Arc<BackendDAE::EqSystem>, i32, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::StrongComponent>, Arc<BackendDAE::EqSystem>, i32, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>) -> Result<Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>> + 'static>), iSyst.clone(), currentIdx.clone(), tmpSystMapping.clone());
            comps = listAppend(tmpComps.clone(), comps.clone());
            (comps.clone(), tmpSystMapping.clone(), currentIdx.clone() + 1)
        },
        _ => {
            println!("{}", (literal!("getSystemComponents0 failed\n")).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oSystMapping)
}

fn getSystemComponents1(mut icomp: Arc<BackendDAE::StrongComponent>, mut isyst: Arc<BackendDAE::EqSystem>, mut isystIdx: i32, mut iMapping: Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>) -> Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>> {
    let mut oMapping: Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>> = metamodelica::nil();
    oMapping = listAppend(iMapping.clone(), list![(isyst.clone(), isystIdx.clone())]);
    oMapping
}

fn getNumberOfSystemComponents(mut iDae: Arc<BackendDAE::BackendDAE>) -> Result<i32> {
    let mut oNumOfComps: i32 = 0;
    let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(iDae.clone()) {
        Deref @ DAE { eqs: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    eqs = __pa0.clone();
    oNumOfComps = List::fold(eqs.clone(), (std::sync::Arc::new(getNumberOfEqSystemComponents) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, i32) -> Result<i32> + 'static>), 0);
    Ok(oNumOfComps)
}

fn getNumberOfEqSystemComponents(mut iEqSystem: Arc<BackendDAE::EqSystem>, mut iNumOfComps: i32) -> Result<i32> {
    let mut oNumOfComps: i32 = 0;
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut matching: Arc<BackendDAE::Matching> = Arc::new(BackendDAE::Matching::NO_MATCHING);
    let __pa0 = ::match_deref::match_deref! { match &(iEqSystem.clone()) {
        Deref @ BackendDAE::EqSystem { matching: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    matching = __pa0.clone();
    comps = BackendDAEUtil::getCompsOfMatching(matching.clone());
    oNumOfComps = iNumOfComps.clone() + (comps.clone().len() as i32);
    Ok(oNumOfComps)
}

pub fn getEmptyTaskGraph(mut numComps: i32, mut numVars: i32, mut numEqs: i32) -> (TaskGraph, TaskGraphMeta) {
    let mut graph: TaskGraph;
    let mut graphData: TaskGraphMeta;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut compParamMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut compInformations: metamodelica::Array<ComponentInfo>;
    graph = arrayCreate(numComps.clone(), metamodelica::nil());
    inComps = arrayCreate(numComps.clone(), metamodelica::nil());
    compParamMapping = arrayCreate(numComps.clone(), metamodelica::nil());
    varCompMapping = arrayCreate(numVars.clone(), (0, 0, 0));
    eqCompMapping = arrayCreate(numEqs.clone(), (0, 0, 0));
    compNames = arrayCreate(numComps.clone(), (literal!("")).clone());
    compDescs = arrayCreate(numComps.clone(), (literal!("")).clone());
    exeCosts = arrayCreate(numComps.clone(), (-1, metamodelica::OrderedFloat(-1.0_f64)));
    commCosts = arrayCreate(numComps.clone(), metamodelica::nil());
    nodeMark = arrayCreate(numComps.clone(), 0);
    compInformations = arrayCreate(numComps.clone(), ComponentInfo { isPartOfODESystem: false, isPartOfZeroFuncSystem: false, isRemovedComponent: false });
    graphData = TaskGraphMeta { inComps: inComps.clone(), varCompMapping: varCompMapping.clone(), eqCompMapping: eqCompMapping.clone(), compParamMapping: compParamMapping.clone(), compNames: compNames.clone(), compDescs: compDescs.clone(), exeCosts: exeCosts.clone(), commCosts: commCosts.clone(), nodeMark: nodeMark.clone(), compInformations: compInformations.clone() };
    (graph, graphData)
}

pub fn copyTaskGraphMeta(mut graphDataIn: TaskGraphMeta) -> Result<TaskGraphMeta> {
    let mut graphDataOut: TaskGraphMeta;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut inComps1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut varCompMapping1: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping1: metamodelica::Array<(i32, i32, i32)>;
    let mut compParamMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut compParamMapping1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compNames1: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut compDescs1: metamodelica::Array<ArcStr>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut exeCosts1: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut commCosts1: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut nodeMark1: metamodelica::Array<i32>;
    let mut compInformations: metamodelica::Array<ComponentInfo>;
    let mut compInformations1: metamodelica::Array<ComponentInfo>;
    let TaskGraphMeta { compInformations: __pa0, nodeMark: __pa1, commCosts: __pa2, exeCosts: __pa3, compDescs: __pa4, compNames: __pa5, compParamMapping: __pa6, eqCompMapping: __pa7, varCompMapping: __pa8, inComps: __pa9 } = (graphDataIn.clone()) else { bail!("pattern mismatch") };
    compInformations = __pa0.clone();
    nodeMark = __pa1.clone();
    commCosts = __pa2.clone();
    exeCosts = __pa3.clone();
    compDescs = __pa4.clone();
    compNames = __pa5.clone();
    compParamMapping = __pa6.clone();
    eqCompMapping = __pa7.clone();
    varCompMapping = __pa8.clone();
    inComps = __pa9.clone();
    inComps1 = metamodelica::arrayFromVec(inComps.clone().borrow().clone());
    varCompMapping1 = metamodelica::arrayFromVec(varCompMapping.clone().borrow().clone());
    eqCompMapping1 = metamodelica::arrayFromVec(eqCompMapping.clone().borrow().clone());
    compParamMapping1 = metamodelica::arrayFromVec(compParamMapping.clone().borrow().clone());
    compNames1 = metamodelica::arrayFromVec(compNames.clone().borrow().clone());
    compDescs1 = metamodelica::arrayFromVec(compDescs.clone().borrow().clone());
    exeCosts1 = metamodelica::arrayFromVec(exeCosts.clone().borrow().clone());
    commCosts1 = metamodelica::arrayFromVec(commCosts.clone().borrow().clone());
    nodeMark1 = metamodelica::arrayFromVec(nodeMark.clone().borrow().clone());
    compInformations1 = metamodelica::arrayFromVec(compInformations.clone().borrow().clone());
    graphDataOut = TaskGraphMeta { inComps: inComps1.clone(), varCompMapping: varCompMapping1.clone(), eqCompMapping: eqCompMapping1.clone(), compParamMapping: compParamMapping1.clone(), compNames: compNames1.clone(), compDescs: compDescs1.clone(), exeCosts: exeCosts1.clone(), commCosts: commCosts1.clone(), nodeMark: nodeMark1.clone(), compInformations: compInformations1.clone() };
    Ok(graphDataOut)
}

fn taskGraphAppend(mut graph1In: TaskGraph, mut graphData1In: TaskGraphMeta, mut graph2In: TaskGraph, mut graphData2In: TaskGraphMeta) -> Result<(TaskGraph, TaskGraphMeta)> {
    let mut graphOut: TaskGraph;
    let mut graphDataOut: TaskGraphMeta;
    let mut eqOffset: i32 = 0;
    let mut idxOffset: i32 = 0;
    let mut varOffset: i32 = 0;
    let mut commCosts1: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut commCosts2: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut inComps1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut inComps2: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut eqCompMapping1: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping2: metamodelica::Array<(i32, i32, i32)>;
    let mut exeCosts1: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut exeCosts2: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut nodeMark1: metamodelica::Array<i32>;
    let mut nodeMark2: metamodelica::Array<i32>;
    let mut compParamMapping1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut compParamMapping2: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut varCompMapping1: metamodelica::Array<(i32, i32, i32)>;
    let mut varCompMapping2: metamodelica::Array<(i32, i32, i32)>;
    let mut compNames1: metamodelica::Array<ArcStr>;
    let mut compNames2: metamodelica::Array<ArcStr>;
    let mut compDescs1: metamodelica::Array<ArcStr>;
    let mut compDescs2: metamodelica::Array<ArcStr>;
    let mut compInformations1: metamodelica::Array<ComponentInfo>;
    let mut compInformations2: metamodelica::Array<ComponentInfo>;
    let mut graph2: TaskGraph;
    let TaskGraphMeta { compInformations: __pa0, nodeMark: __pa1, commCosts: __pa2, exeCosts: __pa3, compDescs: __pa4, compNames: __pa5, compParamMapping: __pa6, eqCompMapping: __pa7, varCompMapping: __pa8, inComps: __pa9 } = (graphData1In.clone()) else { bail!("pattern mismatch") };
    compInformations1 = __pa0.clone();
    nodeMark1 = __pa1.clone();
    commCosts1 = __pa2.clone();
    exeCosts1 = __pa3.clone();
    compDescs1 = __pa4.clone();
    compNames1 = __pa5.clone();
    compParamMapping1 = __pa6.clone();
    eqCompMapping1 = __pa7.clone();
    varCompMapping1 = __pa8.clone();
    inComps1 = __pa9.clone();
    let TaskGraphMeta { compInformations: __pa10, nodeMark: __pa11, commCosts: __pa12, exeCosts: __pa13, compDescs: __pa14, compNames: __pa15, compParamMapping: __pa16, eqCompMapping: __pa17, varCompMapping: __pa18, inComps: __pa19 } = (graphData2In.clone()) else { bail!("pattern mismatch") };
    compInformations2 = __pa10.clone();
    nodeMark2 = __pa11.clone();
    commCosts2 = __pa12.clone();
    exeCosts2 = __pa13.clone();
    compDescs2 = __pa14.clone();
    compNames2 = __pa15.clone();
    compParamMapping2 = __pa16.clone();
    eqCompMapping2 = __pa17.clone();
    varCompMapping2 = __pa18.clone();
    inComps2 = __pa19.clone();
    eqOffset = (eqCompMapping1.clone().borrow().len() as i32);
    idxOffset = (graph1In.clone().borrow().len() as i32);
    varOffset = (varCompMapping1.clone().borrow().len() as i32);
    eqOffset = (eqCompMapping1.clone().borrow().len() as i32);
    graph2 = Array::map1(graph2In.clone(), (std::sync::Arc::new(fnptr!(updateTaskGraphSystem, Arc<metamodelica::List<i32>>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<Arc<metamodelica::List<i32>>> + 'static>), idxOffset.clone())?;
    graphOut = arrayAppend(graph1In.clone(), graph2.clone())?;
    inComps2 = Array::map1(inComps2.clone(), (std::sync::Arc::new(fnptr!(updateTaskGraphSystem, Arc<metamodelica::List<i32>>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<Arc<metamodelica::List<i32>>> + 'static>), idxOffset.clone())?;
    inComps2 = arrayAppend(inComps1.clone(), inComps2.clone())?;
    varCompMapping2 = Array::map1(varCompMapping2.clone(), (std::sync::Arc::new(fnptr!(modifyMapping, (i32, i32, i32), i32)) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32, i32), i32) -> Result<(i32, i32, i32)> + 'static>), idxOffset.clone())?;
    varCompMapping2 = arrayAppend(varCompMapping1.clone(), varCompMapping2.clone())?;
    eqCompMapping2 = Array::map1(eqCompMapping2.clone(), (std::sync::Arc::new(fnptr!(modifyMapping, (i32, i32, i32), i32)) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32, i32), i32) -> Result<(i32, i32, i32)> + 'static>), idxOffset.clone())?;
    eqCompMapping2 = arrayAppend(eqCompMapping1.clone(), eqCompMapping2.clone())?;
    compParamMapping2 = arrayAppend(compParamMapping1.clone(), compParamMapping2.clone())?;
    compNames2 = Array::map1(compNames2.clone(), (std::sync::Arc::new(fnptr!(stringAppend, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<ArcStr> + 'static>), (literal!(" subsys")).clone())?;
    compNames2 = arrayAppend(compNames1.clone(), compNames2.clone())?;
    compDescs2 = arrayAppend(compDescs1.clone(), compDescs2.clone())?;
    exeCosts2 = arrayAppend(exeCosts1.clone(), exeCosts2.clone())?;
    commCosts2 = Array::map1(commCosts2.clone(), (std::sync::Arc::new(fnptr!(updateCommCosts, Arc<metamodelica::List<Communication>>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Communication>>, i32) -> Result<Arc<metamodelica::List<Communication>>> + 'static>), idxOffset.clone())?;
    commCosts2 = arrayAppend(commCosts1.clone(), commCosts2.clone())?;
    nodeMark2 = arrayAppend(nodeMark1.clone(), nodeMark2.clone())?;
    compInformations2 = arrayAppend(compInformations1.clone(), compInformations2.clone())?;
    graphDataOut = TaskGraphMeta { inComps: inComps2.clone(), varCompMapping: varCompMapping2.clone(), eqCompMapping: eqCompMapping2.clone(), compParamMapping: compParamMapping2.clone(), compNames: compNames2.clone(), compDescs: compDescs2.clone(), exeCosts: exeCosts2.clone(), commCosts: commCosts2.clone(), nodeMark: nodeMark2.clone(), compInformations: compInformations2.clone() };
    Ok((graphOut, graphDataOut))
}

fn modifyMapping(mut iMappingTuple: (i32, i32, i32), mut iOffset: i32) -> (i32, i32, i32) {
    let mut oMappingTuple: (i32, i32, i32);
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    let mut i3: i32 = 0;
    (i1, i2, i3) = iMappingTuple.clone();
    oMappingTuple = (i1.clone() + iOffset.clone(), i2.clone(), iOffset.clone());
    oMappingTuple
}

fn updateCommCosts(mut commCostsIn: Communications, mut idxOffset: i32) -> Communications {
    let mut commCostsOut: Communications = metamodelica::nil();
    commCostsOut = List::map1(commCostsIn.clone(), (std::sync::Arc::new(updateCommCosts1) as std::sync::Arc<dyn ::std::ops::Fn(Communication, i32) -> Result<Communication> + 'static>), idxOffset.clone());
    commCostsOut
}

fn updateCommCosts1(mut commCostsIn: Communication, mut idxOffset: i32) -> Result<Communication> {
    let mut commCostsOut: Communication;
    let mut numberOfVars: i32 = 0;
    let mut childNode: i32 = 0;
    let mut integerVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut floatVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut booleanVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut stringVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut requiredTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let Communication { requiredTime: __pa0, childNode: __pa1, stringVars: __pa2, booleanVars: __pa3, floatVars: __pa4, integerVars: __pa5, numberOfVars: __pa6 } = (commCostsIn.clone()) else { bail!("pattern mismatch") };
    requiredTime = __pa0.clone();
    childNode = __pa1.clone();
    stringVars = __pa2.clone();
    booleanVars = __pa3.clone();
    floatVars = __pa4.clone();
    integerVars = __pa5.clone();
    numberOfVars = __pa6.clone();
    childNode = childNode.clone() + idxOffset.clone();
    commCostsOut = Communication { numberOfVars: numberOfVars.clone(), integerVars: integerVars.clone(), floatVars: floatVars.clone(), booleanVars: booleanVars.clone(), stringVars: stringVars.clone(), childNode: childNode.clone(), requiredTime: requiredTime.clone() };
    Ok(commCostsOut)
}

fn updateTaskGraphSystem(mut graphRowIn: Arc<metamodelica::List<i32>>, mut idxOffset: i32) -> Arc<metamodelica::List<i32>> {
    let mut graphRowOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    graphRowOut = List::map1(graphRowIn.clone(), (std::sync::Arc::new(fnptr!(intAdd, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), idxOffset.clone());
    graphRowOut
}

fn createTaskGraph1(mut iComponent: Arc<BackendDAE::StrongComponent>, mut iSystInfo: (metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32), mut iVarInfo: (metamodelica::Array<(i32, i32, i32)>, metamodelica::Array<(i32, i32, i32)>, Arc<metamodelica::List<i32>>), mut iAnalyzeParameters: bool, mut graphInfoIn: (metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<Communication>>>, metamodelica::Array<ArcStr>, metamodelica::Array<i32>, i32)) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<Communication>>>, metamodelica::Array<ArcStr>, metamodelica::Array<i32>, i32)> {
    let mut graphInfoOut: (metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<Communication>>>, metamodelica::Array<ArcStr>, metamodelica::Array<i32>, i32);
    let mut adjacencyMatrix: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut isyst: Arc<BackendDAE::EqSystem>;
    let mut ishared: Arc<BackendDAE::Shared>;
    let mut orderedVars: BackendDAE::Variables;
    let mut globalKnownVars: BackendDAE::Variables;
    let mut localKnownVars: BackendDAE::Variables;
    let mut knownVars: BackendDAE::Variables;
    let mut orderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut graphIn: TaskGraph;
    let mut graphTmp: TaskGraph;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut commCostsOfNode: Communications = metamodelica::nil();
    let mut nodeMark: metamodelica::Array<i32>;
    let mut unsolvedVars: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
    let mut eventVarLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut componentIndex: i32 = 0;
    let mut numberOfComps: i32 = 0;
    let mut requiredSccs_RefCount: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut compName: ArcStr = arcstr::literal!("");
    let mut paramVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut compParamMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut requiredSccs: Arc<UnorderedMap::UnorderedMap<i32, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>>;
    (adjacencyMatrix, isyst, ishared, numberOfComps) = iSystInfo.clone();
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ishared.clone()) {
        Deref @ BackendDAE::Shared { localKnownVars: __pa0, globalKnownVars: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    localKnownVars = __pa0.clone();
    globalKnownVars = __pa1.clone();
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(isyst.clone()) {
        Deref @ BackendDAE::EqSystem { orderedEqs: __pa2, orderedVars: __pa3, .. } => (__pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    orderedEqs = __pa2.clone();
    orderedVars = __pa3.clone();
    (varCompMapping, eqCompMapping, eventVarLst) = iVarInfo.clone();
    (graphIn, inComps, compParamMapping, commCosts, compNames, nodeMark, componentIndex) = graphInfoIn.clone();
    inComps = {let _arr = inComps.clone(); _arr.borrow_mut()[(componentIndex.clone()-1) as usize] = list![componentIndex.clone()]; _arr};
    compName = (BackendDump::strongComponentString(iComponent.clone())?).clone();
    compNames = {let _arr = compNames.clone(); _arr.borrow_mut()[(componentIndex.clone()-1) as usize] = (compName.clone()).clone(); _arr};
    let _ = HpcOmBenchmark::benchSystem()?;
    if iAnalyzeParameters.clone() {
        knownVars = BackendVariable::addVariables(globalKnownVars.clone(), localKnownVars.clone())?;
    } else {
        knownVars = globalKnownVars.clone();
    }
    (unsolvedVars, paramVars) = getUnsolvedVarsBySCC(iComponent.clone(), adjacencyMatrix.clone(), orderedVars.clone(), knownVars.clone(), orderedEqs.clone(), eventVarLst.clone(), iAnalyzeParameters.clone())?;
    compParamMapping = {let _arr = compParamMapping.clone(); _arr.borrow_mut()[(componentIndex.clone()-1) as usize] = paramVars.clone(); _arr};
    requiredSccs = UnorderedMap::new(std::sync::Arc::new(fnptr!(Util::id, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 1);
    for mut intVar in &*Util::tuple41(unsolvedVars.clone()) {
        let mut intVar = intVar.clone();
        fillRequiredSccs((intVar.clone(), 1), VariableType::INTEGER.clone(), varCompMapping.clone(), requiredSccs.clone())?;
    }
    for mut floatVar in &*Util::tuple42(unsolvedVars.clone()) {
        let mut floatVar = floatVar.clone();
        fillRequiredSccs(floatVar.clone(), VariableType::REAL.clone(), varCompMapping.clone(), requiredSccs.clone())?;
    }
    for mut boolVar in &*Util::tuple43(unsolvedVars.clone()) {
        let mut boolVar = boolVar.clone();
        fillRequiredSccs((boolVar.clone(), 1), VariableType::BOOLEAN.clone(), varCompMapping.clone(), requiredSccs.clone())?;
    }
    for mut stringVar in &*Util::tuple44(unsolvedVars.clone()) {
        let mut stringVar = stringVar.clone();
        fillRequiredSccs((stringVar.clone(), 1), VariableType::STRING.clone(), varCompMapping.clone(), requiredSccs.clone())?;
    }
    requiredSccs_RefCount = createRequiredSccsRefCount(requiredSccs.clone());
    (commCosts, commCostsOfNode) = updateCommCostBySccRef(requiredSccs_RefCount.clone(), componentIndex.clone(), commCosts.clone());
    graphTmp = fillAdjacencyList(graphIn.clone(), componentIndex.clone(), commCostsOfNode.clone(), 1)?;
    graphInfoOut = (graphTmp.clone(), inComps.clone(), compParamMapping.clone(), commCosts.clone(), compNames.clone(), nodeMark.clone(), componentIndex.clone() + 1);
    Ok(graphInfoOut)
}

fn createRequiredSccsRefCount(mut requiredSccs: Arc<UnorderedMap::UnorderedMap<i32, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>>) -> Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>> {
    let mut requiredSccsRefCount: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut scc_idx: i32 = 0;
    let mut int_vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut float_vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut bool_vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut string_vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut e in &*UnorderedMap::toList(requiredSccs.clone()) {
        let mut e = e.clone();
        let (__pa0, (__pa1, __pa2, __pa3, __pa4)) = e.clone();
        scc_idx = __pa0.clone();
        int_vars = __pa1.clone();
        float_vars = __pa2.clone();
        bool_vars = __pa3.clone();
        string_vars = __pa4.clone();
        requiredSccsRefCount = cons((scc_idx.clone(), int_vars.clone(), float_vars.clone(), bool_vars.clone(), string_vars.clone()), requiredSccsRefCount.clone());
    }
    requiredSccsRefCount
}

fn updateCommCostBySccRef(mut requiredSccs_RefCount: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>>, mut nodeIdx: i32, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>) -> (metamodelica::Array<Arc<metamodelica::List<Communication>>>, Communications) {
    let mut oCommCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut oNodeComms: Communications = metamodelica::nil();
    let mut tmpComms: Communications = metamodelica::nil();
    tmpComms = List::map1(requiredSccs_RefCount.clone(), (std::sync::Arc::new(fnptr!(createCommunicationObject, (i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn((i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), metamodelica::Real) -> Result<Communication> + 'static>), metamodelica::OrderedFloat(-1.0_f64));
    oCommCosts = List::fold1(tmpComms.clone(), (std::sync::Arc::new(updateCommCostBySccRef1) as std::sync::Arc<dyn ::std::ops::Fn(Communication, i32, metamodelica::Array<Arc<metamodelica::List<Communication>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<Communication>>>> + 'static>), nodeIdx.clone(), iCommCosts.clone());
    oNodeComms = tmpComms.clone();
    (oCommCosts, oNodeComms)
}

fn createCommunicationObject(mut iTuple: (i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), mut requiredTime: metamodelica::Real) -> Communication {
    let mut oComm: Communication;
    let mut integerVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut floatVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut booleanVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut stringVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut sccIdx: i32 = 0;
    let mut refCountSum: i32 = 0;
    (sccIdx, integerVars, floatVars, booleanVars, stringVars) = iTuple.clone();
    refCountSum = (integerVars.clone().len() as i32) + (floatVars.clone().len() as i32) + (booleanVars.clone().len() as i32) + (stringVars.clone().len() as i32);
    oComm = Communication { numberOfVars: refCountSum.clone(), integerVars: integerVars.clone(), floatVars: floatVars.clone(), booleanVars: booleanVars.clone(), stringVars: stringVars.clone(), childNode: sccIdx.clone(), requiredTime: requiredTime.clone() };
    oComm
}

fn updateCommCostBySccRef1(mut iEdgeSource: Communication, mut iEdgeTarget: i32, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<Communication>>>> {
    let mut oCommCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut oldComms: Communications = metamodelica::nil();
    let mut sourceSccIdx: i32 = 0;
    let mut integerVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut floatVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut booleanVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut stringVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut numberOfVars: i32 = 0;
    let mut requiredTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut tmpComm: Communication;
    let Communication { requiredTime: __pa0, childNode: __pa1, stringVars: __pa2, booleanVars: __pa3, floatVars: __pa4, integerVars: __pa5, numberOfVars: __pa6 } = (iEdgeSource.clone()) else { bail!("pattern mismatch") };
    requiredTime = __pa0.clone();
    sourceSccIdx = __pa1.clone();
    stringVars = __pa2.clone();
    booleanVars = __pa3.clone();
    floatVars = __pa4.clone();
    integerVars = __pa5.clone();
    numberOfVars = __pa6.clone();
    oldComms = iCommCosts.clone().borrow()[(sourceSccIdx.clone()-1) as usize].clone();
    tmpComm = Communication { numberOfVars: numberOfVars.clone(), integerVars: integerVars.clone(), floatVars: floatVars.clone(), booleanVars: booleanVars.clone(), stringVars: stringVars.clone(), childNode: iEdgeTarget.clone(), requiredTime: requiredTime.clone() };
    oCommCosts = {let _arr = iCommCosts.clone(); _arr.borrow_mut()[(sourceSccIdx.clone()-1) as usize] = cons(tmpComm.clone(), oldComms.clone()); _arr};
    Ok(oCommCosts)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn fillAdjacencyList(mut adjLstIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut childNode: i32, mut parentLst: Communications, mut Idx: i32) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut adjLstOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    adjLstOut = 'mc: {
        let __mc_input = (adjLstIn.clone(), childNode.clone(), parentLst.clone(), Idx.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _) => {
                    let mut parentNode: Communication;
                    let mut parentRow: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut adjLst: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut parentNodeIdx: i32 = 0;
                    let true = ((parentLst.clone().len() as i32) >= Idx.clone()) else { bail!("pattern mismatch") };
                    parentNode = (parentLst.clone()).get(Idx.clone())?;
                    let Communication { childNode: __pa0, .. } = (parentNode.clone()) else { bail!("pattern mismatch") };
                    parentNodeIdx = __pa0.clone();
                    parentRow = adjLstIn.clone().borrow()[(parentNodeIdx.clone()-1) as usize].clone();
                    parentRow = cons(childNode.clone(), parentRow.clone());
                    parentRow = List::removeOnTrue(parentNodeIdx.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), parentRow.clone());
                    adjLst = {let _arr = adjLstIn.clone(); _arr.borrow_mut()[(parentNodeIdx.clone()-1) as usize] = parentRow.clone(); _arr};
                    adjLst = fillAdjacencyList(adjLst.clone(), childNode.clone(), parentLst.clone(), Idx.clone() + 1)?;
                    Ok(adjLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(adjLstIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(adjLstOut)
}

fn getEquationStrings(mut iComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut iEqSystem: Arc<BackendDAE::EqSystem>) -> metamodelica::Array<ArcStr> {
    let mut eqDescsOut: metamodelica::Array<ArcStr>;
    let mut eqDescs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    eqDescs = List::fold1(iComps.clone(), (std::sync::Arc::new(getEquationStrings2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::StrongComponent>, Arc<BackendDAE::EqSystem>, Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> + 'static>), iEqSystem.clone(), metamodelica::nil());
    eqDescs = eqDescs.clone().reverse();
    eqDescsOut = metamodelica::arrayFromVec(eqDescs.clone().into_iter().cloned().collect());
    eqDescsOut
}

fn getEquationStrings2(mut comp: Arc<BackendDAE::StrongComponent>, mut iEqSystem: Arc<BackendDAE::EqSystem>, mut iEqDesc: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut oEqDesc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    oEqDesc = 'mc: {
        let __mc_input = (comp.clone(), iEqSystem.clone(), iEqDesc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { var: v, eqn: i }, Deref @ BackendDAE::EqSystem { orderedVars, orderedEqs, .. }, _) => {
                    let mut descLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut eqString: ArcStr = arcstr::literal!("");
                    let mut varString: ArcStr = arcstr::literal!("");
                    let mut desc: ArcStr = arcstr::literal!("");
                    eqString = (BackendDump::equationString(BackendEquation::get(orderedEqs.clone(), i.clone()))?).clone();
                    varString = (getVarString(BackendVariable::getVarAt(orderedVars.clone(), v.clone())?)?).clone();
                    desc = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*eqString.clone()); __mm_s.push_str(&*literal!(" FOR ")); __mm_s.push_str(&*varString.clone()); ArcStr::from(__mm_s) }).clone();
                    descLst = cons((desc.clone()).clone(), iEqDesc.clone());
                    Ok(descLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jac: Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: _ }, .. }, Deref @ BackendDAE::EqSystem { .. }, _) => {
                    let mut descLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut desc: ArcStr = arcstr::literal!("");
                    desc = (literal!("Equation System")).clone();
                    descLst = cons((desc.clone()).clone(), iEqDesc.clone());
                    Ok(descLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEARRAY { vars: vs, eqn: i }, Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { .. }, orderedVars, orderedEqs, .. }, _) => {
                    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut descLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut eqString: ArcStr = arcstr::literal!("");
                    let mut desc: ArcStr = arcstr::literal!("");
                    eqString = (BackendDump::equationString(BackendEquation::get(orderedEqs.clone(), i.clone()))?).clone();
                    varLst = BackendVariable::varList(orderedVars.clone())?;
                    desc = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ARRAY:")); __mm_s.push_str(&*eqString.clone()); __mm_s.push_str(&*literal!(" FOR THE VARS: ")); __mm_s.push_str(&*stringDelimitList(List::map1(vs.clone(), std::sync::Arc::new(fnptr!(List::getIndexFirst, i32, _)), List::map(varLst.clone(), (std::sync::Arc::new(getVarString) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<ArcStr> + 'static>))), (literal!(" AND ")).clone())); ArcStr::from(__mm_s) }).clone();
                    descLst = cons((desc.clone()).clone(), iEqDesc.clone());
                    Ok(descLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { vars: vs, eqn: i }, Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { .. }, orderedVars, orderedEqs, .. }, _) => {
                    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut descLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut eqString: ArcStr = arcstr::literal!("");
                    let mut desc: ArcStr = arcstr::literal!("");
                    eqString = (BackendDump::equationString(BackendEquation::get(orderedEqs.clone(), i.clone()))?).clone();
                    varLst = BackendVariable::varList(orderedVars.clone())?;
                    desc = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ALGO: ")); __mm_s.push_str(&*eqString.clone()); __mm_s.push_str(&*literal!(" FOR THE VARS: ")); __mm_s.push_str(&*stringDelimitList(List::map1(vs.clone(), std::sync::Arc::new(fnptr!(List::getIndexFirst, i32, _)), List::map(varLst.clone(), (std::sync::Arc::new(getVarString) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<ArcStr> + 'static>))), (literal!(" AND ")).clone())); ArcStr::from(__mm_s) }).clone();
                    descLst = cons((desc.clone()).clone(), iEqDesc.clone());
                    Ok(descLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { vars: vs, eqn: i }, Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { .. }, orderedVars, orderedEqs, .. }, _) => {
                    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut descLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut eqString: ArcStr = arcstr::literal!("");
                    let mut desc: ArcStr = arcstr::literal!("");
                    eqString = (BackendDump::equationString(BackendEquation::get(orderedEqs.clone(), i.clone()))?).clone();
                    varLst = BackendVariable::varList(orderedVars.clone())?;
                    desc = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("COMPLEX: ")); __mm_s.push_str(&*eqString.clone()); __mm_s.push_str(&*literal!(" FOR THE VARS: ")); __mm_s.push_str(&*stringDelimitList(List::map1(vs.clone(), std::sync::Arc::new(fnptr!(List::getIndexFirst, i32, _)), List::map(varLst.clone(), (std::sync::Arc::new(getVarString) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<ArcStr> + 'static>))), (literal!(" AND ")).clone())); ArcStr::from(__mm_s) }).clone();
                    descLst = cons((desc.clone()).clone(), iEqDesc.clone());
                    Ok(descLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { vars: vs, eqn: i }, Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { .. }, orderedVars, orderedEqs, .. }, _) => {
                    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut descLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut eqString: ArcStr = arcstr::literal!("");
                    let mut desc: ArcStr = arcstr::literal!("");
                    eqString = (BackendDump::equationString(BackendEquation::get(orderedEqs.clone(), i.clone()))?).clone();
                    varLst = BackendVariable::varList(orderedVars.clone())?;
                    desc = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("WHEN:")); __mm_s.push_str(&*eqString.clone()); __mm_s.push_str(&*literal!(" FOR THE VARS: ")); __mm_s.push_str(&*stringDelimitList(List::map1(vs.clone(), std::sync::Arc::new(fnptr!(List::getIndexFirst, i32, _)), List::map(varLst.clone(), (std::sync::Arc::new(getVarString) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<ArcStr> + 'static>))), (literal!(" AND ")).clone())); ArcStr::from(__mm_s) }).clone();
                    descLst = cons((desc.clone()).clone(), iEqDesc.clone());
                    Ok(descLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { vars: vs, eqn: i }, Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { .. }, orderedVars, orderedEqs, .. }, _) => {
                    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut descLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut eqString: ArcStr = arcstr::literal!("");
                    let mut desc: ArcStr = arcstr::literal!("");
                    eqString = (BackendDump::equationString(BackendEquation::get(orderedEqs.clone(), i.clone()))?).clone();
                    varLst = BackendVariable::varList(orderedVars.clone())?;
                    desc = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("IFEQ:")); __mm_s.push_str(&*eqString.clone()); __mm_s.push_str(&*literal!(" FOR THE VARS: ")); __mm_s.push_str(&*stringDelimitList(List::map1(vs.clone(), std::sync::Arc::new(fnptr!(List::getIndexFirst, i32, _)), List::map(varLst.clone(), (std::sync::Arc::new(getVarString) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<ArcStr> + 'static>))), (literal!(" AND ")).clone())); ArcStr::from(__mm_s) }).clone();
                    descLst = cons((desc.clone()).clone(), iEqDesc.clone());
                    Ok(descLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::TORNSYSTEM { linear: true, .. }, Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { .. }, .. }, _) => {
                    let mut descLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut desc: ArcStr = arcstr::literal!("");
                    desc = (literal!("Torn linear System")).clone();
                    descLst = cons((desc.clone()).clone(), iEqDesc.clone());
                    Ok(descLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::TORNSYSTEM { linear: false, .. }, Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { .. }, .. }, _) => {
                    let mut descLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut desc: ArcStr = arcstr::literal!("");
                    desc = (literal!("Torn nonlinear System")).clone();
                    descLst = cons((desc.clone()).clone(), iEqDesc.clone());
                    Ok(descLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut descLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut desc: ArcStr = arcstr::literal!("");
                    desc = (literal!("no singleEquation")).clone();
                    descLst = cons((desc.clone()).clone(), iEqDesc.clone());
                    Ok(descLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oEqDesc)
}

pub fn getVarString(mut inVar: BackendDAE::Var) -> Result<ArcStr> {
    let mut varString: ArcStr = arcstr::literal!("");
    varString = ('mc: {
        let __mc_input = inVar.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut varDescLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut varString: ArcStr = varString.clone();
            let true = (BackendVariable::isNonStateVar(inVar.clone())) else { bail!("pattern mismatch") };
            varString = (BackendDump::varString(inVar.clone())?).clone();
            varDescLst = stringListStringChar((varString.clone()).clone());
            varDescLst = shortenVarString(varDescLst.clone())?;
            varString = (stringCharListString(varDescLst.clone())).clone();
            Ok(varString.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut varDescLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut varString: ArcStr = varString.clone();
            let false = (BackendVariable::isNonStateVar(inVar.clone())) else { bail!("pattern mismatch") };
            varString = (BackendDump::varString(inVar.clone())?).clone();
            varDescLst = stringListStringChar((varString.clone()).clone());
            varDescLst = shortenVarString(varDescLst.clone())?;
            varString = (stringCharListString(varDescLst.clone())).clone();
            varString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" der(")); __mm_s.push_str(&*varString.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            Ok(varString.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(varString)
}

fn shortenVarString(mut iString: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut oString: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut pos: i32 = 0;
    pos = List::position((literal!(":")).clone(), iString.clone())? - 1;
    (oString, _) = List::split(iString.clone(), pos.clone())?;
    Ok(oString)
}

fn getEventNodes(mut systIn: Arc<BackendDAE::BackendDAE>, mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut eventNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tplLst: Arc<metamodelica::List<(i32, i32, i32)>> = metamodelica::nil();
    let mut systemsIn: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(systIn.clone()) {
        Deref @ DAE { eqs: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    systemsIn = __pa0.clone();
    (eqLst, _) = List::fold(systemsIn.clone(), (std::sync::Arc::new(getEventNodeEqs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, (Arc<metamodelica::List<i32>>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32)> + 'static>), (metamodelica::nil(), 0));
    eventNodes = getArrayTuple31(eqLst.clone(), eqCompMapping.clone());
    Ok(eventNodes)
}

fn getEventNodeEqs(mut systIn: Arc<BackendDAE::EqSystem>, mut eventInfoIn: (Arc<metamodelica::List<i32>>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32)> {
    let mut eventInfoOut: (Arc<metamodelica::List<i32>>, i32);
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut matching: Arc<BackendDAE::Matching> = Arc::new(BackendDAE::Matching::NO_MATCHING);
    let mut orderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut eventEqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eventEqsIn: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut offset: i32 = 0;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(systIn.clone()) {
        Deref @ BackendDAE::EqSystem { matching: __pa0, orderedEqs: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    matching = __pa0.clone();
    orderedEqs = __pa1.clone();
    comps = BackendDAEUtil::getCompsOfMatching(matching.clone());
    (eventEqsIn, offset) = eventInfoIn.clone();
    eventEqs = getEventNodeEqs1(comps.clone(), offset.clone(), metamodelica::nil())?;
    offset = offset.clone() + ExpandableArray::getNumberOfElements(orderedEqs.clone());
    eventInfoOut = (listAppend(eventEqs.clone(), eventEqsIn.clone()), offset.clone());
    Ok(eventInfoOut)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getEventNodeEqs1(mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut offset: i32, mut eventEqsIn: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut eventEqsOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    eventEqsOut = 'mc: {
        let __mc_input = (comps.clone(), offset.clone(), eventEqsIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: head, tail: rest }, _, _) => {
                    let mut eqn: i32 = 0;
                    let mut eventEqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let true = (isWhenEquation(head.clone())?) else { bail!("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(head.clone()) {
                        Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { eqn: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    eqn = __pa0.clone();
                    eqn = eqn.clone() + offset.clone();
                    eventEqs = getEventNodeEqs1(rest.clone(), offset.clone(), cons(eqn.clone(), eventEqsIn.clone()))?;
                    Ok(eventEqs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: head, tail: rest }, _, _) => {
                    let mut eventEqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let false = (isWhenEquation(head.clone())?) else { bail!("pattern mismatch") };
                    eventEqs = getEventNodeEqs1(rest.clone(), offset.clone(), eventEqsIn.clone())?;
                    Ok(eventEqs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(eventEqsIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(eventEqsOut)
}

fn getArrayTuple31(mut list1: Arc<metamodelica::List<i32>>, mut assign: metamodelica::Array<(i32, i32, i32)>) -> Arc<metamodelica::List<i32>> {
    let mut list2Out: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tplLst: Arc<metamodelica::List<(i32, i32, i32)>> = metamodelica::nil();
    tplLst = List::map1(list1.clone(), std::sync::Arc::new(fnptr!(Array::getIndexFirst, i32, _)), assign.clone());
    list2Out = List::map(tplLst.clone(), std::sync::Arc::new(fnptr!(Util::tuple31, _)));
    list2Out
}

fn isWhenEquation(mut inComp: Arc<BackendDAE::StrongComponent>) -> Result<bool> {
    let mut isWhenEq: bool = false;
    isWhenEq = 'mc: {
        let __mc_input = inComp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { .. } => {
                    Ok(true)
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
    Ok(isWhenEq)
}

fn fillRequiredSccs(mut var: (i32, i32), mut varType: VariableType, mut varMapping: metamodelica::Array<(i32, i32, i32)>, mut requiredSccs: Arc<UnorderedMap::UnorderedMap<i32, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>>) -> Result<()> {
    let mut var_idx: i32 = 0;
    let mut scc_idx: i32 = 0;
    let mut not_derived: i32 = 0;
    let mut integerVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut floatVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut booleanVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut stringVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (var_idx, not_derived) = var.clone();
    if not_derived.clone() == 1 {
        (scc_idx, _, _) = varMapping.borrow()[(var_idx.clone()-1) as usize].clone();
        (integerVars, floatVars, booleanVars, stringVars) = UnorderedMap::getOrDefault(scc_idx.clone(), requiredSccs.clone(), (metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil()));
        let () = (match varType.clone() {
        VariableType::INTEGER { .. } => {
            integerVars = cons(var_idx.clone(), integerVars.clone());
            ()
        },
        VariableType::REAL { .. } => {
            floatVars = cons(var_idx.clone(), floatVars.clone());
            ()
        },
        VariableType::BOOLEAN => {
            booleanVars = cons(var_idx.clone(), booleanVars.clone());
            ()
        },
        VariableType::STRING { .. } => {
            stringVars = cons(var_idx.clone(), stringVars.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    });
        UnorderedMap::add(scc_idx.clone(), (integerVars.clone(), floatVars.clone(), booleanVars.clone(), stringVars.clone()), requiredSccs.clone())?;
    }
    Ok(())
}

fn getUnsolvedVarsBySCC(mut iComponent: Arc<BackendDAE::StrongComponent>, mut iAdjacencyMatrix: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iOrderedVars: BackendDAE::Variables, mut iKnownVars: BackendDAE::Variables, mut iOrderedEquations: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut iEventVarLst: Arc<metamodelica::List<i32>>, mut iAnalyzeParameters: bool) -> Result<((Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), Arc<metamodelica::List<i32>>)> {
    let mut oUnsolvedVars: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
    let mut oParamVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (oUnsolvedVars, oParamVars) = 'mc: {
        let __mc_input = (iComponent.clone(), iAdjacencyMatrix.clone(), iOrderedVars.clone(), iKnownVars.clone(), iOrderedEquations.clone(), iEventVarLst.clone(), iAnalyzeParameters.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { var: varIdx, .. }, _, _, _, _, _, _) => {
                    let mut tmpVars: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
                    let mut paramVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    (tmpVars, paramVars) = getUnsolvedVarsBySCC0(iComponent.clone(), iAdjacencyMatrix.clone(), iOrderedVars.clone(), iKnownVars.clone(), iOrderedEquations.clone(), list![varIdx.clone()], iEventVarLst.clone(), iAnalyzeParameters.clone())?;
                    Ok((tmpVars.clone(), paramVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { vars: varIdc, .. }, _, _, _, _, _, _) => {
                    let mut tmpVars: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
                    let mut paramVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    (tmpVars, paramVars) = getUnsolvedVarsBySCC0(iComponent.clone(), iAdjacencyMatrix.clone(), iOrderedVars.clone(), iKnownVars.clone(), iOrderedEquations.clone(), varIdc.clone(), iEventVarLst.clone(), iAnalyzeParameters.clone())?;
                    Ok((tmpVars.clone(), paramVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEARRAY { vars: varIdc, .. }, _, _, _, _, _, _) => {
                    let mut tmpVars: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
                    let mut paramVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    (tmpVars, paramVars) = getUnsolvedVarsBySCC0(iComponent.clone(), iAdjacencyMatrix.clone(), iOrderedVars.clone(), iKnownVars.clone(), iOrderedEquations.clone(), varIdc.clone(), iEventVarLst.clone(), iAnalyzeParameters.clone())?;
                    Ok((tmpVars.clone(), paramVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { vars: varIdc, .. }, _, _, _, _, _, _) => {
                    let mut tmpVars: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
                    let mut paramVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    (tmpVars, paramVars) = getUnsolvedVarsBySCC0(iComponent.clone(), iAdjacencyMatrix.clone(), iOrderedVars.clone(), iKnownVars.clone(), iOrderedEquations.clone(), varIdc.clone(), iEventVarLst.clone(), iAnalyzeParameters.clone())?;
                    Ok((tmpVars.clone(), paramVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { vars: varIdc, .. }, _, _, _, _, _, _) => {
                    let mut tmpVars: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
                    let mut paramVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    (tmpVars, paramVars) = getUnsolvedVarsBySCC0(iComponent.clone(), iAdjacencyMatrix.clone(), iOrderedVars.clone(), iKnownVars.clone(), iOrderedEquations.clone(), varIdc.clone(), iEventVarLst.clone(), iAnalyzeParameters.clone())?;
                    Ok((tmpVars.clone(), paramVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { vars: varIdc, .. }, _, _, _, _, _, _) => {
                    let mut tmpVars: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
                    let mut paramVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    (tmpVars, paramVars) = getUnsolvedVarsBySCC0(iComponent.clone(), iAdjacencyMatrix.clone(), iOrderedVars.clone(), iKnownVars.clone(), iOrderedEquations.clone(), varIdc.clone(), iEventVarLst.clone(), iAnalyzeParameters.clone())?;
                    Ok((tmpVars.clone(), paramVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { vars: varIdc, .. }, _, _, _, _, _, _) => {
                    let mut tmpVars: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
                    let mut paramVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    (tmpVars, paramVars) = getUnsolvedVarsBySCC0(iComponent.clone(), iAdjacencyMatrix.clone(), iOrderedVars.clone(), iKnownVars.clone(), iOrderedEquations.clone(), varIdc.clone(), iEventVarLst.clone(), iAnalyzeParameters.clone())?;
                    Ok((tmpVars.clone(), paramVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: varIdc, .. }, .. }, _, _, _, _, _, _) => {
                    let mut tmpVars: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
                    let mut paramVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    (tmpVars, paramVars) = getUnsolvedVarsBySCC0(iComponent.clone(), iAdjacencyMatrix.clone(), iOrderedVars.clone(), iKnownVars.clone(), iOrderedEquations.clone(), varIdc.clone(), iEventVarLst.clone(), iAnalyzeParameters.clone())?;
                    Ok((tmpVars.clone(), paramVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("getUnsolvedVarsBySCC failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((oUnsolvedVars, oParamVars))
}

fn getUnsolvedVarsBySCC0(mut iComponent: Arc<BackendDAE::StrongComponent>, mut iAdjacencyMatrix: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iOrderedVars: BackendDAE::Variables, mut iKnownVars: BackendDAE::Variables, mut iOrderedEquations: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut iVarIdc: Arc<metamodelica::List<i32>>, mut iEventVarLst: Arc<metamodelica::List<i32>>, mut iAnalyzeParameters: bool) -> Result<((Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), Arc<metamodelica::List<i32>>)> {
    let mut oUnsolvedVars: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
    let mut oParamVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpVars: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    (tmpVars, oParamVars) = getVarsBySCC(iComponent.clone(), iAdjacencyMatrix.clone(), iOrderedVars.clone(), iKnownVars.clone(), iOrderedEquations.clone(), iAnalyzeParameters.clone())?;
    tmpVars = List::filter1OnTrue(tmpVars.clone(), (std::sync::Arc::new(isTupleMember) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32), Arc<metamodelica::List<i32>>) -> Result<bool> + 'static>), iVarIdc.clone());
    tmpVars = removeEventVars(iEventVarLst.clone(), tmpVars.clone(), 1)?;
    oUnsolvedVars = List::fold1(tmpVars.clone(), (std::sync::Arc::new(getUnsolvedVarsBySCC1) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32), BackendDAE::Variables, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> + 'static>), iOrderedVars.clone(), (metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil()));
    Ok((oUnsolvedVars, oParamVars))
}

fn getUnsolvedVarsBySCC1(mut iVarIdx: (i32, i32), mut orderedVars: BackendDAE::Variables, mut iUnsolvedVars: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut oUnsolvedVars: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
    let mut var: BackendDAE::Var;
    let mut varType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    var = BackendVariable::getVarAt(orderedVars.clone(), Util::tuple21(iVarIdx.clone()))?;
    varType = BackendVariable::getVarType(var.clone());
    oUnsolvedVars = getUnsolvedVarsBySCC2(varType.clone(), iVarIdx.clone(), iUnsolvedVars.clone());
    Ok(oUnsolvedVars)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getUnsolvedVarsBySCC2(mut iVarType: Arc<DAE::Type>, mut iVarIdx: (i32, i32), mut iUnsolvedVars: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)) -> (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) {
    let mut oUnsolvedVars: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
    let mut intVarIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut boolVarIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut stringVarIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut realVarIdc: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut varIdx: i32 = 0;
    let mut derived: i32 = 0;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    oUnsolvedVars = (::match_deref::match_deref! { match &((iVarType.clone(), iVarIdx.clone(), iUnsolvedVars.clone())) {
        (Deref @ DAE::Type::T_INTEGER { .. }, (varIdx, derived), (intVarIdc, realVarIdc, boolVarIdc, stringVarIdc)) => {
            let mut intVarIdc = (*intVarIdc).clone();
            intVarIdc = cons(varIdx.clone(), intVarIdc.clone());
            (intVarIdc.clone(), realVarIdc.clone(), boolVarIdc.clone(), stringVarIdc.clone())
        },
        (Deref @ DAE::Type::T_REAL { .. }, (varIdx, derived), (intVarIdc, realVarIdc, boolVarIdc, stringVarIdc)) => {
            let mut realVarIdc = (*realVarIdc).clone();
            realVarIdc = cons((varIdx.clone(), derived.clone()), realVarIdc.clone());
            (intVarIdc.clone(), realVarIdc.clone(), boolVarIdc.clone(), stringVarIdc.clone())
        },
        (Deref @ DAE::Type::T_BOOL { .. }, (varIdx, derived), (intVarIdc, realVarIdc, boolVarIdc, stringVarIdc)) => {
            let mut boolVarIdc = (*boolVarIdc).clone();
            boolVarIdc = cons(varIdx.clone(), boolVarIdc.clone());
            (intVarIdc.clone(), realVarIdc.clone(), boolVarIdc.clone(), stringVarIdc.clone())
        },
        (Deref @ DAE::Type::T_ARRAY { ty, .. }, (varIdx, derived), (intVarIdc, realVarIdc, boolVarIdc, stringVarIdc)) => getUnsolvedVarsBySCC2(ty.clone(), iVarIdx.clone(), iUnsolvedVars.clone()),
        (Deref @ DAE::Type::T_ENUMERATION { .. }, (varIdx, derived), (intVarIdc, realVarIdc, boolVarIdc, stringVarIdc)) => {
            let mut stringVarIdc = (*stringVarIdc).clone();
            stringVarIdc = cons(varIdx.clone(), stringVarIdc.clone());
            (intVarIdc.clone(), realVarIdc.clone(), boolVarIdc.clone(), stringVarIdc.clone())
        },
        (Deref @ DAE::Type::T_STRING { .. }, (varIdx, derived), (intVarIdc, realVarIdc, boolVarIdc, stringVarIdc)) => {
            let mut stringVarIdc = (*stringVarIdc).clone();
            stringVarIdc = cons(varIdx.clone(), stringVarIdc.clone());
            (intVarIdc.clone(), realVarIdc.clone(), boolVarIdc.clone(), stringVarIdc.clone())
        },
        _ => {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getUnsolvedVarsBySCC2: Warning, unknown varType for variable ")); __mm_s.push_str(&*intString(Util::tuple21(iVarIdx.clone()))); __mm_s.push_str(&*literal!(" !\n")); ArcStr::from(__mm_s) }).clone());
            iUnsolvedVars.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oUnsolvedVars
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn removeEventVars(mut eventVarLst: Arc<metamodelica::List<i32>>, mut varLstIn: Arc<metamodelica::List<(i32, i32)>>, mut varIdx: i32) -> Result<Arc<metamodelica::List<(i32, i32)>>> {
    let mut varLstOut: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    varLstOut = 'mc: {
        let __mc_input = (eventVarLst.clone(), varLstIn.clone(), varIdx.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut varTpl: (i32, i32);
                    let mut varLst: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
                    let mut var: i32 = 0;
                    let true = (intLe(varIdx.clone(), (varLstIn.clone().len() as i32))) else { bail!("pattern mismatch") };
                    varTpl = (varLstIn.clone()).get(varIdx.clone())?;
                    (var, _) = varTpl.clone();
                    let true = (List::isMemberOnTrue(var.clone(), eventVarLst.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
                    varLst = listDelete(varLstIn.clone(), varIdx.clone())?;
                    varLst = removeEventVars(eventVarLst.clone(), varLst.clone(), varIdx.clone())?;
                    Ok(varLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut varTpl: (i32, i32);
                    let mut varLst: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
                    let mut var: i32 = 0;
                    let true = (intLe(varIdx.clone(), (varLstIn.clone().len() as i32))) else { bail!("pattern mismatch") };
                    varTpl = (varLstIn.clone()).get(varIdx.clone())?;
                    (var, _) = varTpl.clone();
                    let false = (List::isMemberOnTrue(var.clone(), eventVarLst.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
                    varLst = removeEventVars(eventVarLst.clone(), varLstIn.clone(), varIdx.clone() + 1)?;
                    Ok(varLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(varLstIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(varLstOut)
}

fn isTupleMember(mut inTuple: (i32, i32), mut varIdc: Arc<metamodelica::List<i32>>) -> Result<bool> {
    let mut isNotMember: bool = false;
    let mut varIdx: i32 = 0;
    let mut varState: i32 = 0;
    let mut returnValue: bool = false;
    isNotMember = 'mc: {
        let __mc_input = (inTuple.clone(), varIdc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((varIdx, varState), _) => {
                    let mut returnValue: bool = returnValue.clone();
                    let true = (intGt(varIdx.clone(), 0)) else { bail!("pattern mismatch") };
                    let true = (intEq(varState.clone(), 1)) else { bail!("pattern mismatch") };
                    returnValue = List::isMemberOnTrue(varIdx.clone(), varIdc.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>));
                    Ok(!(returnValue.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(isNotMember)
}

fn compareTupleByVarIdx(mut varIdx: i32, mut var2Idx: (i32, i32)) -> bool {
    let mut equal: bool = false;
    equal = intEq(Util::tuple21(var2Idx.clone()), varIdx.clone());
    equal
}

pub fn compareTasksByExecTime(mut iTask1: i32, mut iTask2: i32, mut iTaskComps: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iExeCosts: metamodelica::Array<(i32, metamodelica::Real)>, mut iDescending: bool) -> Result<bool> {
    let mut oResult: bool = false;
    let mut exeCosts1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut exeCosts2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut taskComps1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut taskComps2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    taskComps1 = iTaskComps.clone().borrow()[(iTask1.clone()-1) as usize].clone();
    taskComps2 = iTaskComps.clone().borrow()[(iTask2.clone()-1) as usize].clone();
    exeCosts1 = addUpExeCostsForNode(taskComps1.clone(), iExeCosts.clone(), metamodelica::OrderedFloat(0.0_f64))?;
    exeCosts2 = addUpExeCostsForNode(taskComps2.clone(), iExeCosts.clone(), metamodelica::OrderedFloat(0.0_f64))?;
    if iDescending.clone() {
        oResult = realLt(exeCosts1.clone(), exeCosts2.clone());
    } else {
        oResult = realGt(exeCosts1.clone(), exeCosts2.clone());
    }
    Ok(oResult)
}

fn getVarsBySCC(mut iComponent: Arc<BackendDAE::StrongComponent>, mut iAdjacencyMatrix: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iOrderedVars: BackendDAE::Variables, mut iKnownVars: BackendDAE::Variables, mut iOrderedEquations: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut iAnalyzeParameters: bool) -> Result<(Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>)> {
    let mut oVars: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut oParamVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (oVars, oParamVars) = (::match_deref::match_deref! { match &((iComponent.clone(), iAdjacencyMatrix.clone(), iOrderedVars.clone(), iKnownVars.clone(), iOrderedEquations.clone(), iAnalyzeParameters.clone())) {
        (Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { eqn: eqnIdx, .. }, _, _, _, _, _) => {
            let mut eqnVars: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
            let mut paramVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            (eqnVars, paramVars) = getVarsByEqns(list![eqnIdx.clone()], iAdjacencyMatrix.clone(), iOrderedVars.clone(), iKnownVars.clone(), iOrderedEquations.clone(), iAnalyzeParameters.clone())?;
            let _ = List::toString(eqnVars.clone(), (std::sync::Arc::new(fnptr!(tupleToString, (i32, i32))) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32)) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(";")).clone(), (literal!("}")).clone(), true, 0)?;
            (eqnVars.clone(), paramVars.clone())
        },
        (Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns, .. }, _, _, _, _, _) => {
            let mut eqnVars: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
            let mut paramVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            (eqnVars, paramVars) = getVarsByEqns(eqns.clone(), iAdjacencyMatrix.clone(), iOrderedVars.clone(), iKnownVars.clone(), iOrderedEquations.clone(), iAnalyzeParameters.clone())?;
            (eqnVars.clone(), paramVars.clone())
        },
        (Deref @ BackendDAE::StrongComponent::SINGLEARRAY { eqn: eqnIdx, .. }, _, _, _, _, _) => {
            let mut eqnVars: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
            let mut paramVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            (eqnVars, paramVars) = getVarsByEqns(list![eqnIdx.clone()], iAdjacencyMatrix.clone(), iOrderedVars.clone(), iKnownVars.clone(), iOrderedEquations.clone(), iAnalyzeParameters.clone())?;
            let _ = List::toString(eqnVars.clone(), (std::sync::Arc::new(fnptr!(tupleToString, (i32, i32))) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32)) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(";")).clone(), (literal!("}")).clone(), true, 0)?;
            (eqnVars.clone(), paramVars.clone())
        },
        (Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { eqn: eqnIdx, .. }, _, _, _, _, _) => {
            let mut eqnVars: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
            let mut paramVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            (eqnVars, paramVars) = getVarsByEqns(list![eqnIdx.clone()], iAdjacencyMatrix.clone(), iOrderedVars.clone(), iKnownVars.clone(), iOrderedEquations.clone(), iAnalyzeParameters.clone())?;
            let _ = List::toString(eqnVars.clone(), (std::sync::Arc::new(fnptr!(tupleToString, (i32, i32))) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32)) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(";")).clone(), (literal!("}")).clone(), true, 0)?;
            (eqnVars.clone(), paramVars.clone())
        },
        (Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { eqn: eqnIdx, .. }, _, _, _, _, _) => {
            let mut eqnVars: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
            let mut paramVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            (eqnVars, paramVars) = getVarsByEqns(list![eqnIdx.clone()], iAdjacencyMatrix.clone(), iOrderedVars.clone(), iKnownVars.clone(), iOrderedEquations.clone(), iAnalyzeParameters.clone())?;
            let _ = List::toString(eqnVars.clone(), (std::sync::Arc::new(fnptr!(tupleToString, (i32, i32))) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32)) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(";")).clone(), (literal!("}")).clone(), true, 0)?;
            (eqnVars.clone(), paramVars.clone())
        },
        (Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { eqn: eqnIdx, .. }, _, _, _, _, _) => {
            let mut eqnVars: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
            let mut paramVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            (eqnVars, paramVars) = getVarsByEqns(list![eqnIdx.clone()], iAdjacencyMatrix.clone(), iOrderedVars.clone(), iKnownVars.clone(), iOrderedEquations.clone(), iAnalyzeParameters.clone())?;
            let _ = List::toString(eqnVars.clone(), (std::sync::Arc::new(fnptr!(tupleToString, (i32, i32))) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32)) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(";")).clone(), (literal!("}")).clone(), true, 0)?;
            (eqnVars.clone(), paramVars.clone())
        },
        (Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { eqn: eqnIdx, .. }, _, _, _, _, _) => {
            let mut eqnVars: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
            let mut paramVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            (eqnVars, paramVars) = getVarsByEqns(list![eqnIdx.clone()], iAdjacencyMatrix.clone(), iOrderedVars.clone(), iKnownVars.clone(), iOrderedEquations.clone(), iAnalyzeParameters.clone())?;
            let _ = List::toString(eqnVars.clone(), (std::sync::Arc::new(fnptr!(tupleToString, (i32, i32))) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32)) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(";")).clone(), (literal!("}")).clone(), true, 0)?;
            (eqnVars.clone(), paramVars.clone())
        },
        (Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { innerEquations, residualequations: resEqns, .. }, .. }, _, _, _, _, _) => {
            let mut eqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut eqnVars: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
            let mut paramVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            (eqns, _, _) = List::map_3(innerEquations.clone(), (std::sync::Arc::new(BackendDAEUtil::getEqnAndVarsFromInnerEquation) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::InnerEquation) -> Result<(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<DAE::Constraint>>>)> + 'static>));
            (eqnVars, paramVars) = getVarsByEqns(listAppend(resEqns.clone(), eqns.clone()), iAdjacencyMatrix.clone(), iOrderedVars.clone(), iKnownVars.clone(), iOrderedEquations.clone(), iAnalyzeParameters.clone())?;
            (eqnVars.clone(), paramVars.clone())
        },
        _ => {
            println!("{}", (literal!("Error in getVarsBySCC! Unsupported component-type \n")).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oVars, oParamVars))
}

fn tupleToString(mut inTuple: (i32, i32)) -> ArcStr {
    let mut result: ArcStr = arcstr::literal!("");
    result = ((match inTuple.clone() {
        (mut int1, mut int2) => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(int1.clone())); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*intString(int2.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
    })).clone();
    result
}

fn tuple3ToString(mut inTuple: (i32, i32, i32)) -> ArcStr {
    let mut result: ArcStr = arcstr::literal!("");
    result = ((match inTuple.clone() {
        (mut int1, mut int2, mut int3) => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(int1.clone())); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*intString(int2.clone())); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*intString(int3.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
    })).clone();
    result
}

fn getVarsByEqns(mut iEqnIdc: Arc<metamodelica::List<i32>>, mut iAdjacencyMatrix: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iOrderedVars: BackendDAE::Variables, mut iKnownVars: BackendDAE::Variables, mut iOrderedEquations: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut iAnalyzeParameters: bool) -> Result<(Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>)> {
    let mut oAdjacencyVars: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut oParamVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut adjacencyVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut paramVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    for mut eqIdx in &*iEqnIdc.clone() {
        let mut eqIdx = eqIdx.clone();
        adjacencyVars = listAppend(iAdjacencyMatrix.clone().borrow()[(eqIdx.clone()-1) as usize].clone(), adjacencyVars.clone());
        eqs = cons(BackendEquation::get(iOrderedEquations.clone(), eqIdx.clone()), eqs.clone());
    }
    oAdjacencyVars = List::map(adjacencyVars.clone(), (std::sync::Arc::new(fnptr!(getVarTuple, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<(i32, i32)> + 'static>));
    if iAnalyzeParameters.clone() {
        (paramVars, oParamVars) = BackendEquation::equationsParams(eqs.clone(), iKnownVars.clone())?;
    } else {
        oParamVars = metamodelica::nil();
    }
    Ok((oAdjacencyVars, oParamVars))
}

fn getVarTuple(mut varIdx: i32) -> (i32, i32) {
    let mut outIdx: (i32, i32);
    outIdx = if (intLe(0, varIdx.clone())) {(varIdx.clone(), 1)} else {(-(varIdx.clone()), 0)};
    outIdx
}

fn compareIntTuple2(mut tuple1: (i32, i32), mut tuple2: (i32, i32)) -> bool {
    let mut equals: bool = false;
    equals = (match (tuple1.clone(), tuple2.clone()) {
        ((mut int1, mut int2), (mut int3, mut int4)) if (int1.clone() == int3.clone() && int2.clone() == int4.clone()) => {
            true
        },
        _ => {
            false
        },
    });
    equals
}

fn getVarEqCompMapping(mut components: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut iEqSysIdx: i32, mut iVarIdxOffset: i32, mut iEqIdxOffset: i32, mut ivarCompMapping: metamodelica::Array<(i32, i32, i32)>, mut ieqCompMapping: metamodelica::Array<(i32, i32, i32)>) -> (metamodelica::Array<(i32, i32, i32)>, metamodelica::Array<(i32, i32, i32)>) {
    let mut ovarCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut oeqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let _ = List::fold4(components.clone(), (std::sync::Arc::new(getVarEqCompMapping0) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::StrongComponent>, metamodelica::Array<(i32, i32, i32)>, metamodelica::Array<(i32, i32, i32)>, i32, (i32, i32), i32) -> Result<i32> + 'static>), ivarCompMapping.clone(), ieqCompMapping.clone(), iEqSysIdx.clone(), (iVarIdxOffset.clone(), iEqIdxOffset.clone()), 1);
    ovarCompMapping = ivarCompMapping.clone();
    oeqCompMapping = ieqCompMapping.clone();
    (ovarCompMapping, oeqCompMapping)
}

fn getVarEqCompMapping0(mut component: Arc<BackendDAE::StrongComponent>, mut varCompMapping: metamodelica::Array<(i32, i32, i32)>, mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>, mut iEqSysIdx: i32, mut iVarEqOffset: (i32, i32), mut iSccIdx: i32) -> Result<i32> {
    let mut oSccIdx: i32 = 0;
    oSccIdx = 'mc: {
        let __mc_input = (component.clone(), varCompMapping.clone(), eqCompMapping.clone(), iEqSysIdx.clone(), iVarEqOffset.clone(), iSccIdx.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { eqn: eq, var: compVarIdx }, _, _, _, (iVarOffset, iEqOffset), _) => {
                    {let _arr = varCompMapping.clone(); _arr.borrow_mut()[(compVarIdx.clone() + iVarOffset.clone()-1) as usize] = (iSccIdx.clone(), iEqSysIdx.clone(), iVarOffset.clone()); _arr};
                    {let _arr = eqCompMapping.clone(); _arr.borrow_mut()[(eq.clone() + iEqOffset.clone()-1) as usize] = (iSccIdx.clone(), iEqSysIdx.clone(), iEqOffset.clone()); _arr};
                    Ok(iSccIdx.clone() + 1)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns, vars: compVarIdc, .. }, _, _, _, (iVarOffset, iEqOffset), _) => {
                    let _ = List::fold3(compVarIdc.clone(), (std::sync::Arc::new(updateMappingTuple) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, metamodelica::Array<(i32, i32, i32)>) -> Result<metamodelica::Array<(i32, i32, i32)>> + 'static>), iSccIdx.clone(), iEqSysIdx.clone(), iVarOffset.clone(), varCompMapping.clone());
                    let _ = List::fold3(eqns.clone(), (std::sync::Arc::new(updateMappingTuple) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, metamodelica::Array<(i32, i32, i32)>) -> Result<metamodelica::Array<(i32, i32, i32)>> + 'static>), iSccIdx.clone(), iEqSysIdx.clone(), iEqOffset.clone(), eqCompMapping.clone());
                    Ok(iSccIdx.clone() + 1)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { eqn: eq, vars: compVarIdc }, _, _, _, (iVarOffset, iEqOffset), _) => {
                    let _ = List::fold3(compVarIdc.clone(), (std::sync::Arc::new(updateMappingTuple) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, metamodelica::Array<(i32, i32, i32)>) -> Result<metamodelica::Array<(i32, i32, i32)>> + 'static>), iSccIdx.clone(), iEqSysIdx.clone(), iVarOffset.clone(), varCompMapping.clone());
                    {let _arr = eqCompMapping.clone(); _arr.borrow_mut()[(eq.clone() + iEqOffset.clone()-1) as usize] = (iSccIdx.clone(), iEqSysIdx.clone(), iEqOffset.clone()); _arr};
                    Ok(iSccIdx.clone() + 1)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEARRAY { eqn: eq, vars: compVarIdc }, _, _, _, (iVarOffset, iEqOffset), _) => {
                    let _ = List::fold3(compVarIdc.clone(), (std::sync::Arc::new(updateMappingTuple) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, metamodelica::Array<(i32, i32, i32)>) -> Result<metamodelica::Array<(i32, i32, i32)>> + 'static>), iSccIdx.clone(), iEqSysIdx.clone(), iVarOffset.clone(), varCompMapping.clone());
                    {let _arr = eqCompMapping.clone(); _arr.borrow_mut()[(eq.clone() + iEqOffset.clone()-1) as usize] = (iSccIdx.clone(), iEqSysIdx.clone(), iEqOffset.clone()); _arr};
                    Ok(iSccIdx.clone() + 1)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { eqn: eq, vars: compVarIdc }, _, _, _, (iVarOffset, iEqOffset), _) => {
                    let _ = List::fold3(compVarIdc.clone(), (std::sync::Arc::new(updateMappingTuple) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, metamodelica::Array<(i32, i32, i32)>) -> Result<metamodelica::Array<(i32, i32, i32)>> + 'static>), iSccIdx.clone(), iEqSysIdx.clone(), iVarOffset.clone(), varCompMapping.clone());
                    {let _arr = eqCompMapping.clone(); _arr.borrow_mut()[(eq.clone() + iEqOffset.clone()-1) as usize] = (iSccIdx.clone(), iEqSysIdx.clone(), iEqOffset.clone()); _arr};
                    Ok(iSccIdx.clone() + 1)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { eqn: eq, vars: compVarIdc }, _, _, _, (iVarOffset, iEqOffset), _) => {
                    let _ = List::fold3(compVarIdc.clone(), (std::sync::Arc::new(updateMappingTuple) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, metamodelica::Array<(i32, i32, i32)>) -> Result<metamodelica::Array<(i32, i32, i32)>> + 'static>), iSccIdx.clone(), iEqSysIdx.clone(), iVarOffset.clone(), varCompMapping.clone());
                    {let _arr = eqCompMapping.clone(); _arr.borrow_mut()[(eq.clone() + iEqOffset.clone()-1) as usize] = (iSccIdx.clone(), iEqSysIdx.clone(), iEqOffset.clone()); _arr};
                    Ok(iSccIdx.clone() + 1)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { innerEquations, residualequations: residuals, tearingvars: compVarIdc, .. }, .. }, _, _, _, (iVarOffset, iEqOffset), _) => {
                    let mut eqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut othereqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut othervars: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut othervarsLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut compVarIdc = (*compVarIdc).clone();
                    (othereqs, othervarsLst, _) = List::map_3(innerEquations.clone(), (std::sync::Arc::new(BackendDAEUtil::getEqnAndVarsFromInnerEquation) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::InnerEquation) -> Result<(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<DAE::Constraint>>>)> + 'static>));
                    othervars = List::flatten(othervarsLst.clone());
                    compVarIdc = listAppend(othervars.clone(), compVarIdc.clone());
                    eqns = listAppend(othereqs.clone(), residuals.clone());
                    let _ = List::fold3(compVarIdc.clone(), (std::sync::Arc::new(updateMappingTuple) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, metamodelica::Array<(i32, i32, i32)>) -> Result<metamodelica::Array<(i32, i32, i32)>> + 'static>), iSccIdx.clone(), iEqSysIdx.clone(), iVarOffset.clone(), varCompMapping.clone());
                    let _ = List::fold3(eqns.clone(), (std::sync::Arc::new(updateMappingTuple) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, metamodelica::Array<(i32, i32, i32)>) -> Result<metamodelica::Array<(i32, i32, i32)>> + 'static>), iSccIdx.clone(), iEqSysIdx.clone(), iEqOffset.clone(), eqCompMapping.clone());
                    Ok(iSccIdx.clone() + 1)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { eqn: eq, vars: compVarIdc }, _, _, _, (iVarOffset, iEqOffset), _) => {
                    let _ = List::fold3(compVarIdc.clone(), (std::sync::Arc::new(updateMappingTuple) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, metamodelica::Array<(i32, i32, i32)>) -> Result<metamodelica::Array<(i32, i32, i32)>> + 'static>), iSccIdx.clone(), iEqSysIdx.clone(), iVarOffset.clone(), varCompMapping.clone());
                    {let _arr = eqCompMapping.clone(); _arr.borrow_mut()[(eq.clone() + iEqOffset.clone()-1) as usize] = (iSccIdx.clone(), iEqSysIdx.clone(), iEqOffset.clone()); _arr};
                    Ok(iSccIdx.clone() + 1)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut helperStr: ArcStr = arcstr::literal!("");
                    helperStr = (BackendDump::strongComponentString(component.clone())?).clone();
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getVarEqCompMapping0 - Unsupported component-type:\n")); __mm_s.push_str(&*helperStr.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oSccIdx)
}

pub fn getSccNodeMapping(mut iNumberOfSccs: i32, mut iTaskGraphMeta: TaskGraphMeta) -> Result<metamodelica::Array<i32>> {
    let mut oMapping: metamodelica::Array<i32>;
    let mut tmpMappingArray: metamodelica::Array<i32>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut nodeMark: metamodelica::Array<i32>;
    tmpMappingArray = arrayCreate(iNumberOfSccs.clone(), -1);
    let TaskGraphMeta { nodeMark: __pa0, inComps: __pa1, .. } = (iTaskGraphMeta.clone()) else { bail!("pattern mismatch") };
    nodeMark = __pa0.clone();
    inComps = __pa1.clone();
    (oMapping, _) = Array::fold(inComps.clone(), Arc::new({ let __pe_b1 = nodeMark.clone(); move |__pe_a0, __pe_a2| Ok(getSccNodeMapping0(__pe_a0, __pe_b1.clone(), __pe_a2)) }), (tmpMappingArray.clone(), 1));
    Ok(oMapping)
}

fn getSccNodeMapping0(mut iCompsOfNode: Arc<metamodelica::List<i32>>, mut iNodeMarks: metamodelica::Array<i32>, mut iArrayNodeIdx: (metamodelica::Array<i32>, i32)) -> (metamodelica::Array<i32>, i32) {
    let mut oArrayNodeIdx: (metamodelica::Array<i32>, i32);
    let mut tmpMappingArray: metamodelica::Array<i32>;
    let mut nodeIdx: i32 = 0;
    (tmpMappingArray, nodeIdx) = List::fold1(iCompsOfNode.clone(), (std::sync::Arc::new(getSccNodeMapping1) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, (metamodelica::Array<i32>, i32)) -> Result<(metamodelica::Array<i32>, i32)> + 'static>), iNodeMarks.clone(), iArrayNodeIdx.clone());
    oArrayNodeIdx = (tmpMappingArray.clone(), nodeIdx.clone() + 1);
    oArrayNodeIdx
}

fn getSccNodeMapping1(mut iCompIdx: i32, mut iNodeMark: metamodelica::Array<i32>, mut iArrayNodeIdx: (metamodelica::Array<i32>, i32)) -> Result<(metamodelica::Array<i32>, i32)> {
    let mut oArrayNodeIdx: (metamodelica::Array<i32>, i32);
    let mut iNodeIdx: i32 = 0;
    let mut offset: i32 = 0;
    let mut nodeMark: i32 = 0;
    let mut iMappingArray: metamodelica::Array<i32>;
    oArrayNodeIdx = 'mc: {
        let __mc_input = (iCompIdx.clone(), iNodeMark.clone(), iArrayNodeIdx.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, (mut iMappingArray, mut iNodeIdx)) = __mc_input.clone() else { bail!("nomatch") };
            let mut nodeMark: i32 = nodeMark.clone();
            nodeMark = iNodeMark.clone().borrow()[(iCompIdx.clone()-1) as usize].clone();
            let true = (intNe(-1, nodeMark.clone())) else { bail!("pattern mismatch") };
            iMappingArray = {let _arr = iMappingArray.clone(); _arr.borrow_mut()[(iCompIdx.clone()-1) as usize] = iNodeIdx.clone(); _arr};
            Ok((iMappingArray.clone(), iNodeIdx.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, (mut iMappingArray, mut iNodeIdx)) = __mc_input.clone() else { bail!("nomatch") };
            Ok((iMappingArray.clone(), iNodeIdx.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oArrayNodeIdx)
}

fn othersInTearComp(mut otherEqnVarTpl: (i32, Arc<metamodelica::List<i32>>), mut othersIn: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut othersOut: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
    othersOut = 'mc: {
        let __mc_input = (otherEqnVarTpl.clone(), othersIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut eq: i32 = 0;
                    let mut eqLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut varTplLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut varLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    (eq, varTplLst) = otherEqnVarTpl.clone();
                    let _ = (varTplLst.clone()).get(1)?;
                    (eqLst, varLst) = othersIn.clone();
                    varLst = listAppend(varTplLst.clone(), varLst.clone());
                    eqLst = cons(eq.clone(), eqLst.clone());
                    Ok((eqLst.clone(), varLst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("check number of vars in relation to number of eqs in otherEqnVarTpl in the torn system\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(othersOut)
}

fn updateMapping(mut varIdx: i32, mut sccIdx: i32, mut iMapping: metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> {
    let mut oMapping: metamodelica::Array<i32>;
    oMapping = {let _arr = iMapping.clone(); _arr.borrow_mut()[(varIdx.clone()-1) as usize] = sccIdx.clone(); _arr};
    Ok(oMapping)
}

fn updateMappingTuple(mut varIdx: i32, mut sccIdx: i32, mut iEqSysIdx: i32, mut iVarOffset: i32, mut iMapping: metamodelica::Array<(i32, i32, i32)>) -> Result<metamodelica::Array<(i32, i32, i32)>> {
    let mut oMapping: metamodelica::Array<(i32, i32, i32)>;
    oMapping = {let _arr = iMapping.clone(); _arr.borrow_mut()[(varIdx.clone() + iVarOffset.clone()-1) as usize] = (sccIdx.clone(), iEqSysIdx.clone(), iVarOffset.clone()); _arr};
    Ok(oMapping)
}

//--------------------------------------------------------
//  Functions to get the ODEsystem graph and adjacencyList
//--------------------------------------------------------
pub fn getOdeSystem(mut graphIn: TaskGraph, mut graphDataIn: TaskGraphMeta, mut systIn: Arc<BackendDAE::BackendDAE>) -> Result<(TaskGraph, TaskGraphMeta)> {
    let mut graphOdeOut: TaskGraph;
    let mut graphDataOdeOut: TaskGraphMeta;
    let mut stateNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut whenNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut cutNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut cutNodeChildren: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut graphTmp: TaskGraph;
    let mut graphDataTmp: TaskGraphMeta;
    let TaskGraphMeta { inComps: __pa0, eqCompMapping: __pa1, varCompMapping: __pa2, .. } = (graphDataIn.clone()) else { bail!("pattern mismatch") };
    inComps = __pa0.clone();
    eqCompMapping = __pa1.clone();
    varCompMapping = __pa2.clone();
    let __pa3 = ::match_deref::match_deref! { match &(systIn.clone()) {
        Deref @ DAE { UNIQUEIO: __pa3, derivativeNamePrefix: _, .. } => __pa3.clone(),
        _ => bail!("pattern mismatch"),
    } };
    systs = __pa3.clone();
    (stateNodes, _) = List::fold2(systs.clone(), (std::sync::Arc::new(getAllStateNodes) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, metamodelica::Array<(i32, i32, i32)>, metamodelica::Array<Arc<metamodelica::List<i32>>>, (Arc<metamodelica::List<i32>>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32)> + 'static>), varCompMapping.clone(), inComps.clone(), (metamodelica::nil(), 0));
    whenNodes = getEventNodes(systIn.clone(), eqCompMapping.clone())?;
    graphTmp = metamodelica::arrayFromVec(graphIn.clone().borrow().clone());
    (graphOdeOut, cutNodes) = cutTaskGraph(graphTmp.clone(), stateNodes.clone(), whenNodes.clone())?;
    cutNodeChildren = List::flatten(List::map1(listAppend(cutNodes.clone(), whenNodes.clone()), std::sync::Arc::new(fnptr!(Array::getIndexFirst, i32, _)), graphIn.clone()));
    (_, cutNodeChildren, _) = List::intersection1OnTrue(cutNodeChildren.clone(), cutNodes.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    graphDataOdeOut = cutSystemData(graphDataIn.clone(), listAppend(cutNodes.clone(), metamodelica::nil()), cutNodeChildren.clone())?;
    Ok((graphOdeOut, graphDataOdeOut))
}

fn getAllStateNodes(mut systIn: Arc<BackendDAE::EqSystem>, mut varCompMapping: metamodelica::Array<(i32, i32, i32)>, mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut stateInfoIn: (Arc<metamodelica::List<i32>>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32)> {
    let mut stateInfoOut: (Arc<metamodelica::List<i32>>, i32);
    stateInfoOut = 'mc: {
        let __mc_input = (systIn.clone(), varCompMapping.clone(), inComps.clone(), stateInfoIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, (stateNodesIn, varOffset)) => {
                    let mut stateNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut stateVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut varOffsetNew: i32 = 0;
                    let mut orderedVars: BackendDAE::Variables;
                    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(systIn.clone()) {
                        Deref @ BackendDAE::EqSystem { orderedVars: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    orderedVars = __pa0.clone();
                    varLst = BackendVariable::varList(orderedVars.clone())?;
                    stateVars = getStates(varLst.clone(), metamodelica::nil(), 1)?;
                    let false = (stateVars.clone().is_empty()) else { bail!("pattern mismatch") };
                    stateVars = List::map1(stateVars.clone(), (std::sync::Arc::new(fnptr!(intAdd, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), varOffset.clone());
                    stateNodes = getArrayTuple31(stateVars.clone(), varCompMapping.clone());
                    stateNodes = List::map3(stateNodes.clone(), (std::sync::Arc::new(getCompInComps) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>) -> Result<i32> + 'static>), 1, inComps.clone(), arrayCreate((inComps.clone().borrow().len() as i32), 0));
                    stateNodes = listAppend(stateNodesIn.clone(), stateNodes.clone());
                    varOffsetNew = (varLst.clone().len() as i32) + varOffset.clone();
                    Ok((stateNodes.clone(), varOffsetNew.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, (stateNodesIn, varOffset)) => {
                    let mut stateVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut varOffsetNew: i32 = 0;
                    let mut orderedVars: BackendDAE::Variables;
                    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(systIn.clone()) {
                        Deref @ BackendDAE::EqSystem { orderedVars: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    orderedVars = __pa0.clone();
                    varLst = BackendVariable::varList(orderedVars.clone())?;
                    stateVars = getStates(varLst.clone(), metamodelica::nil(), 1)?;
                    let true = (stateVars.clone().is_empty()) else { bail!("pattern mismatch") };
                    varOffsetNew = (varLst.clone().len() as i32) + varOffset.clone();
                    Ok((stateNodesIn.clone(), varOffsetNew.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, (_, _)) => {
                    let mut stateVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut orderedVars: BackendDAE::Variables;
                    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(systIn.clone()) {
                        Deref @ BackendDAE::EqSystem { orderedVars: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    orderedVars = __pa0.clone();
                    varLst = BackendVariable::varList(orderedVars.clone())?;
                    stateVars = getStates(varLst.clone(), metamodelica::nil(), 1)?;
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getAllStateNodes failed! StateVars-Count: ")); __mm_s.push_str(&*intString((stateVars.clone().len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(stateInfoOut)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getStates(mut inVarLst: Arc<metamodelica::List<BackendDAE::Var>>, mut stateVarsIn: Arc<metamodelica::List<i32>>, mut Idx: i32) -> Result<Arc<metamodelica::List<i32>>> {
    let mut stateVarsOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    stateVarsOut = 'mc: {
        let __mc_input = (inVarLst.clone(), stateVarsIn.clone(), Idx.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: head, tail: rest }, _, _) => {
                    let mut stateVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let false = (BackendVariable::isStateVar(head.clone())) else { bail!("pattern mismatch") };
                    stateVars = getStates(rest.clone(), stateVarsIn.clone(), Idx.clone() + 1)?;
                    Ok(stateVars.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: head, tail: rest }, _, _) => {
                    let mut stateVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let true = (BackendVariable::isStateVar(head.clone())) else { bail!("pattern mismatch") };
                    stateVars = getStates(rest.clone(), cons(Idx.clone(), stateVarsIn.clone()), Idx.clone() + 1)?;
                    Ok(stateVars.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(stateVarsIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(stateVarsOut)
}

fn cutTaskGraph(mut graphIn: TaskGraph, mut exceptNodes: Arc<metamodelica::List<i32>>, mut whenNodes: Arc<metamodelica::List<i32>>) -> Result<(TaskGraph, Arc<metamodelica::List<i32>>)> {
    let mut graphOut: TaskGraph;
    let mut cutNodesOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (graphOut, cutNodesOut) = 'mc: {
        let __mc_input = (graphIn.clone(), exceptNodes.clone(), whenNodes.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: (-1), tail: Deref @ metamodelica::List::Nil }, _) => {
                    Ok((graphIn.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut sizeDAE: i32 = 0;
                    let mut sizeODE: i32 = 0;
                    let mut graphT: TaskGraph;
                    let mut graphODE: TaskGraph;
                    let mut cutNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut odeNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut odeMap: metamodelica::Array<i32>;
                    sizeDAE = (graphIn.clone().borrow().len() as i32);
                    graphT = AdjacencyMatrix::transposeAdjacencyMatrix(graphIn.clone(), sizeDAE.clone())?;
                    odeNodes = listAppend(exceptNodes.clone(), getAllSuccessors(exceptNodes.clone(), graphT.clone())?);
                    (_, odeNodes, _) = List::intersection1OnTrue(odeNodes.clone(), whenNodes.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    (odeNodes, _, _) = List::intersection1OnTrue(List::intRange(sizeDAE.clone()), odeNodes.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    odeNodes = List::sort(odeNodes.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    sizeODE = (odeNodes.clone().len() as i32);
                    odeMap = arrayCreate(sizeDAE.clone(), -1);
                    List::threadMap1_0(odeNodes.clone(), List::intRange(sizeODE.clone()), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), odeMap.clone())?;
                    graphODE = arrayCreate(sizeODE.clone(), metamodelica::nil());
                    (graphODE, cutNodes) = cutTaskGraph2(List::intRange(sizeDAE.clone()), graphODE.clone(), metamodelica::nil(), graphIn.clone(), odeMap.clone())?;
                    Ok((graphODE.clone(), cutNodes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("cutTaskGraph failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((graphOut, cutNodesOut))
}

fn cutTaskGraph2(mut daeNodes: Arc<metamodelica::List<i32>>, mut graphODE: TaskGraph, mut cutNodesIn: Arc<metamodelica::List<i32>>, mut graphDAE: TaskGraph, mut odeMap: metamodelica::Array<i32>) -> Result<(TaskGraph, Arc<metamodelica::List<i32>>)> {
    let mut graphOut: TaskGraph;
    let mut cutNodesOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (graphOut, cutNodesOut) = 'mc: {
        let __mc_input = (daeNodes.clone(), graphODE.clone(), cutNodesIn.clone(), graphDAE.clone(), odeMap.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: daeIdx, tail: rest }, _, _, _, _) => {
                    let mut odeIdx: i32 = 0;
                    let mut row: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut cutNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    odeIdx = odeMap.clone().borrow()[(daeIdx.clone()-1) as usize].clone();
                    let true = (intGt(odeIdx.clone(), 0)) else { bail!("pattern mismatch") };
                    row = graphDAE.clone().borrow()[(daeIdx.clone()-1) as usize].clone();
                    row = List::map1(row.clone(), std::sync::Arc::new(fnptr!(Array::getIndexFirst, i32, _)), odeMap.clone());
                    row = List::filter1OnTrue(row.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0);
                    {let _arr = graphODE.clone(); _arr.borrow_mut()[(odeIdx.clone()-1) as usize] = row.clone(); _arr};
                    (_, cutNodes) = cutTaskGraph2(rest.clone(), graphODE.clone(), cutNodesIn.clone(), graphDAE.clone(), odeMap.clone())?;
                    Ok((graphODE.clone(), cutNodes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: daeIdx, tail: rest }, _, _, _, _) => {
                    let mut odeIdx: i32 = 0;
                    let mut cutNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    odeIdx = odeMap.clone().borrow()[(daeIdx.clone()-1) as usize].clone();
                    let true = (intEq(odeIdx.clone(), -1)) else { bail!("pattern mismatch") };
                    (_, cutNodes) = cutTaskGraph2(rest.clone(), graphODE.clone(), cons(daeIdx.clone(), cutNodesIn.clone()), graphDAE.clone(), odeMap.clone())?;
                    Ok((graphODE.clone(), cutNodes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _) => {
                    Ok((graphODE.clone(), cutNodesIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((graphOut, cutNodesOut))
}

fn cutSystemData(mut graphDataIn: TaskGraphMeta, mut cutNodes: Arc<metamodelica::List<i32>>, mut cutNodeChildren: Arc<metamodelica::List<i32>>) -> Result<TaskGraphMeta> {
    let mut graphDataOut: TaskGraphMeta;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut rangeLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut compParamMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut compInformations: metamodelica::Array<ComponentInfo>;
    let TaskGraphMeta { compInformations: __pa0, nodeMark: __pa1, commCosts: __pa2, exeCosts: __pa3, compDescs: __pa4, compNames: __pa5, compParamMapping: __pa6, eqCompMapping: __pa7, varCompMapping: __pa8, inComps: __pa9 } = (graphDataIn.clone()) else { bail!("pattern mismatch") };
    compInformations = __pa0.clone();
    nodeMark = __pa1.clone();
    commCosts = __pa2.clone();
    exeCosts = __pa3.clone();
    compDescs = __pa4.clone();
    compNames = __pa5.clone();
    compParamMapping = __pa6.clone();
    eqCompMapping = __pa7.clone();
    varCompMapping = __pa8.clone();
    inComps = __pa9.clone();
    inComps = metamodelica::arrayFromVec(List::deletePositions(Arc::new(inComps.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), cutNodes.clone(), false)?.into_iter().cloned().collect());
    rangeLst = List::intRange((nodeMark.clone().borrow().len() as i32));
    nodeMark = List::fold1(rangeLst.clone(), (std::sync::Arc::new(markRemovedNodes) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<i32>>, metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> + 'static>), cutNodes.clone(), nodeMark.clone());
    graphDataOut = TaskGraphMeta { inComps: inComps.clone(), varCompMapping: varCompMapping.clone(), eqCompMapping: eqCompMapping.clone(), compParamMapping: compParamMapping.clone(), compNames: compNames.clone(), compDescs: compDescs.clone(), exeCosts: exeCosts.clone(), commCosts: commCosts.clone(), nodeMark: nodeMark.clone(), compInformations: compInformations.clone() };
    Ok(graphDataOut)
}

fn markRemovedNodes(mut nodeMarkIdx: i32, mut removedNodes: Arc<metamodelica::List<i32>>, mut nodeMarkIn: metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> {
    let mut nodeMarkOut: metamodelica::Array<i32>;
    nodeMarkOut = 'mc: {
        let __mc_input = (nodeMarkIdx.clone(), removedNodes.clone(), nodeMarkIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let true = (intEq(-2, nodeMarkIn.clone().borrow()[(nodeMarkIdx.clone()-1) as usize].clone())) else { bail!("pattern mismatch") };
                    Ok(nodeMarkIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let false = (List::isMemberOnTrue(nodeMarkIdx.clone(), removedNodes.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
                    Ok(nodeMarkIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut nodeMarkTmp: metamodelica::Array<i32>;
                    let true = (List::isMemberOnTrue(nodeMarkIdx.clone(), removedNodes.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
                    nodeMarkTmp = Array::replaceAtWithFill(nodeMarkIdx.clone(), -1, 999, nodeMarkIn.clone())?;
                    Ok(nodeMarkTmp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(nodeMarkOut)
}

pub fn getCompInComps(mut compIn: i32, mut compIdx: i32, mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut nodeMark: metamodelica::Array<i32>) -> Result<i32> {
    let mut compOut: i32 = 0;
    compOut = 'mc: {
        let __mc_input = (compIn.clone(), compIdx.clone(), inComps.clone(), nodeMark.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut mergedComp: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut compTmp: i32 = 0;
            let true = ((inComps.clone().borrow().len() as i32) >= compIdx.clone()) else { bail!("pattern mismatch") };
            mergedComp = inComps.clone().borrow()[(compIdx.clone()-1) as usize].clone();
            let false = (List::isMemberOnTrue(compIn.clone(), mergedComp.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
            compTmp = getCompInComps(compIn.clone(), compIdx.clone() + 1, inComps.clone(), nodeMark.clone())?;
            Ok(compTmp.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut mergedComp: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let true = ((inComps.clone().borrow().len() as i32) >= compIdx.clone()) else { bail!("pattern mismatch") };
            mergedComp = inComps.clone().borrow()[(compIdx.clone()-1) as usize].clone();
            let true = (List::isMemberOnTrue(compIn.clone(), mergedComp.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
            Ok(compIdx.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut nodeMarkEntry: i32 = 0;
            nodeMarkEntry = nodeMark.clone().borrow()[(compIn.clone()-1) as usize].clone();
            let true = (intLt(nodeMarkEntry.clone(), 0)) else { bail!("pattern mismatch") };
            Ok(-1)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getCompInComps failed! CompIn idx: ")); __mm_s.push_str(&*intString(compIn.clone())); __mm_s.push_str(&*literal!(" | Component array-size: ")); __mm_s.push_str(&*intString((inComps.clone().borrow().len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(compOut)
}

pub fn getAllSuccessors(mut nodes: Arc<metamodelica::List<i32>>, mut graph: TaskGraph) -> Result<Arc<metamodelica::List<i32>>> {
    let mut successors: Arc<metamodelica::List<i32>> = metamodelica::nil();
    successors = 'mc: {
        let __mc_input = (nodes.clone(), graph.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut alreadyVisited: metamodelica::Array<bool>;
                    let mut check: Arc<metamodelica::List<bool>> = metamodelica::nil();
                    let mut successors1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    alreadyVisited = arrayCreate((graph.clone().borrow().len() as i32), false);
                    List::map2_0(nodes.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), true, alreadyVisited.clone());
                    successors1 = List::flatten(List::map1(nodes.clone(), std::sync::Arc::new(fnptr!(Array::getIndexFirst, i32, _)), graph.clone()));
                    check = List::map1(successors1.clone(), std::sync::Arc::new(fnptr!(Array::getIndexFirst, i32, _)), alreadyVisited.clone());
                    (_, successors1) = List::filterOnTrueSync(check.clone(), (std::sync::Arc::new(fnptr!(boolNot, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool) -> Result<bool> + 'static>), successors1.clone())?;
                    successors1 = List::unique(successors1.clone());
                    Ok(getAllSuccessors2(successors1.clone(), graph.clone(), alreadyVisited.clone(), successors1.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("getAllSuccessors failed!\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(successors)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getAllSuccessors2(mut nodes: Arc<metamodelica::List<i32>>, mut graph: TaskGraph, mut alreadyVisited: metamodelica::Array<bool>, mut successorsIn: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut successorsOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    successorsOut = (::match_deref::match_deref! { match &((nodes.clone(), graph.clone(), alreadyVisited.clone(), successorsIn.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _) => {
            List::unique(successorsIn.clone())
        },
        (_, _, _, _) => {
            let mut check: Arc<metamodelica::List<bool>> = metamodelica::nil();
            let mut successors1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            successors1 = List::flatten(List::map1(nodes.clone(), std::sync::Arc::new(fnptr!(Array::getIndexFirst, i32, _)), graph.clone()));
            check = List::map1(successors1.clone(), std::sync::Arc::new(fnptr!(Array::getIndexFirst, i32, _)), alreadyVisited.clone());
            (_, successors1) = List::filterOnTrueSync(check.clone(), (std::sync::Arc::new(fnptr!(boolNot, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool) -> Result<bool> + 'static>), successors1.clone())?;
            successors1 = List::unique(successors1.clone());
            List::map2_0(successors1.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), true, alreadyVisited.clone());
            getAllSuccessors2(successors1.clone(), graph.clone(), alreadyVisited.clone(), listAppend(successors1.clone(), successorsIn.clone()))?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(successorsOut)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getChildNodes(mut adjacencyLstIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut parents: Arc<metamodelica::List<i32>>, mut childLstTmp: Arc<metamodelica::List<i32>>, mut Idx: i32) -> Result<Arc<metamodelica::List<i32>>> {
    let mut childLsts: Arc<metamodelica::List<i32>> = metamodelica::nil();
    childLsts = 'mc: {
        let __mc_input = (adjacencyLstIn.clone(), parents.clone(), childLstTmp.clone(), Idx.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _) => {
                    let mut parent: i32 = 0;
                    let mut row: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut childLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let true = ((parents.clone().len() as i32) >= Idx.clone()) else { bail!("pattern mismatch") };
                    parent = (parents.clone()).get(Idx.clone())?;
                    row = adjacencyLstIn.clone().borrow()[(parent.clone()-1) as usize].clone();
                    childLst = listAppend(childLstTmp.clone(), row.clone());
                    childLst = getChildNodes(adjacencyLstIn.clone(), parents.clone(), childLst.clone(), Idx.clone() + 1)?;
                    Ok(childLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(childLstTmp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(childLsts)
}

pub fn updateContinuousEntriesInList(mut lstIn: Arc<metamodelica::List<i32>>, mut deleteEntriesIn: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut lstOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    lstOut = (::match_deref::match_deref! { match &((lstIn.clone(), deleteEntriesIn.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            metamodelica::nil()
        },
        (_, Deref @ metamodelica::List::Nil) => {
            lstIn.clone()
        },
        (Deref @ metamodelica::List::Cons { head: start, tail: rest }, _) => {
            let mut lstTmp: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut deleteArr: metamodelica::Array<i32>;
            deleteArr = arrayCreate(List::fold(listAppend(rest.clone(), deleteEntriesIn.clone()), (std::sync::Arc::new(fnptr!(intMax, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), start.clone()), 0);
            List::map2_0(deleteEntriesIn.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), 1, deleteArr.clone());
            (deleteArr, _) = Array::mapFold(deleteArr.clone(), (std::sync::Arc::new(setDeleteArr) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<(i32, i32)> + 'static>), 0);
            lstTmp = List::map1(lstIn.clone(), (std::sync::Arc::new(removeContinuousEntries1) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<i32> + 'static>), deleteArr.clone());
            lstTmp.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(lstOut)
}

fn setDeleteArr(mut entryIn: i32, mut offsetIn: i32) -> Result<(i32, i32)> {
    let mut entryOut: i32 = 0;
    let mut offsetOut: i32 = 0;
    (entryOut, offsetOut) = (match (entryIn.clone(), offsetIn.clone()) {
        (0, _) => (offsetIn.clone(), offsetIn.clone()),
        (1, _) => (offsetIn.clone() + 1, offsetIn.clone() + 1),
        _ => bail!("match: no arm matched"),
    });
    Ok((entryOut, offsetOut))
}

fn removeContinuousEntries1(mut entryIn: i32, mut deleteEntriesIn: metamodelica::Array<i32>) -> Result<i32> {
    let mut entryOut: i32 = 0;
    let mut eqSysIdx: i32 = 0;
    let mut entryOffset: i32 = 0;
    entryOut = 'mc: {
        let __mc_input = (entryIn.clone(), deleteEntriesIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut offset: i32 = 0;
            offset = deleteEntriesIn.clone().borrow()[(entryIn.clone()-1) as usize].clone();
            Ok(entryIn.clone() - offset.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            println!("{}", (literal!("removeContinuousEntries1 failed!\n")).clone());
            Ok(entryIn.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(entryOut)
}

fn deleteRowInAdjLst(mut adjacencyLstIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowsDel: Arc<metamodelica::List<i32>>) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>)> {
    let mut adjacencyLstOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut odeMapping: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut adjLst: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut copiedRows: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut size: i32 = 0;
    size = (adjacencyLstIn.clone().borrow().len() as i32) - (rowsDel.clone().len() as i32);
    adjLst = arrayCreate(size.clone(), metamodelica::nil());
    copiedRows = List::intRange((adjacencyLstIn.clone().borrow().len() as i32));
    copiedRows = List::deletePositions(copiedRows.clone(), rowsDel.clone(), false)?;
    adjacencyLstOut = arrayCopyRows(adjacencyLstIn.clone(), adjLst.clone(), copiedRows.clone(), 1)?;
    odeMapping = copiedRows.clone();
    Ok((adjacencyLstOut, odeMapping))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn arrayCopyRows(mut inArray: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut newArray: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut copiedRows: Arc<metamodelica::List<i32>>, mut Idx: i32) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut outArray: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    outArray = 'mc: {
        let __mc_input = (inArray.clone(), newArray.clone(), copiedRows.clone(), Idx.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _) => {
                    let mut copyRow: i32 = 0;
                    let mut row: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut arrayTmp: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let true = ((copiedRows.clone().len() as i32) >= Idx.clone()) else { bail!("pattern mismatch") };
                    copyRow = (copiedRows.clone()).get(Idx.clone())?;
                    row = inArray.clone().borrow()[(copyRow.clone()-1) as usize].clone();
                    arrayTmp = Array::replaceAtWithFill(Idx.clone(), row.clone(), list![111, 222], newArray.clone())?;
                    arrayTmp = arrayCopyRows(inArray.clone(), arrayTmp.clone(), copiedRows.clone(), Idx.clone() + 1)?;
                    Ok(arrayTmp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(newArray.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outArray)
}

pub fn getRootNodes(mut iTaskGraph: TaskGraph) -> Result<Arc<metamodelica::List<i32>>> {
    let mut rootsOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut size: i32 = 0;
    let mut taskGraphT: TaskGraph;
    size = (iTaskGraph.clone().borrow().len() as i32);
    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), size.clone())?;
    rootsOut = getLeafNodes(taskGraphT.clone())?;
    Ok(rootsOut)
}

pub fn getLeafNodes(mut iTaskGraph: TaskGraph) -> Result<Arc<metamodelica::List<i32>>> {
    let mut oLeafNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpLeafNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut nodeSuccessors: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut nodeIdx: i32 = 0;
    tmpLeafNodes = metamodelica::nil();
    let __range0 = 1..=(iTaskGraph.clone().borrow().len() as i32);
    for mut nodeIdx in __range0 {
        nodeSuccessors = iTaskGraph.clone().borrow()[(nodeIdx.clone()-1) as usize].clone();
        if nodeSuccessors.clone().is_empty() {
            tmpLeafNodes = cons(nodeIdx.clone(), tmpLeafNodes.clone());
        }
    }
    oLeafNodes = tmpLeafNodes.clone();
    Ok(oLeafNodes)
}

pub fn getLevelNodes(mut iTaskGraph: TaskGraph) -> Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> {
    let mut oLevelNodes: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut refCounter: metamodelica::Array<i32>;
    let mut roots: Arc<metamodelica::List<i32>> = metamodelica::nil();
    refCounter = createRefCounter(iTaskGraph.clone());
    roots = getNodesWithRefCountZero(refCounter.clone());
    oLevelNodes = getLevelNodes0(iTaskGraph.clone(), refCounter.clone(), roots.clone(), metamodelica::nil());
    oLevelNodes
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getLevelNodes0(mut iTaskGraph: TaskGraph, mut iRefCounter: metamodelica::Array<i32>, mut iNodesWithRefZero: Arc<metamodelica::List<i32>>, mut iLevelNodes: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> {
    let mut oLevelNodes: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut tmpLevelNodes: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut zeroRefNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    oLevelNodes = (::match_deref::match_deref! { match &((iTaskGraph.clone(), iRefCounter.clone(), iNodesWithRefZero.clone(), iLevelNodes.clone())) {
        (_, _, Deref @ metamodelica::List::Nil, _) => {
            tmpLevelNodes = iLevelNodes.clone().reverse();
            tmpLevelNodes.clone()
        },
        (_, _, zeroRefNodes, _) => {
            let mut zeroRefNodes = (*zeroRefNodes).clone();
            tmpLevelNodes = cons(zeroRefNodes.clone(), iLevelNodes.clone());
            zeroRefNodes = List::fold2(zeroRefNodes.clone(), (std::sync::Arc::new(getLevelNodes1) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), iTaskGraph.clone(), iRefCounter.clone(), metamodelica::nil());
            tmpLevelNodes = getLevelNodes0(iTaskGraph.clone(), iRefCounter.clone(), zeroRefNodes.clone(), tmpLevelNodes.clone());
            tmpLevelNodes.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oLevelNodes
}

fn getLevelNodes1(mut iNodeIdx: i32, mut iTaskGraph: TaskGraph, mut iRefCounter: metamodelica::Array<i32>, mut iNodesWithRefZero: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut oNodesWithRefZero: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut childNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpNodesWithRefZero: Arc<metamodelica::List<i32>> = metamodelica::nil();
    childNodes = iTaskGraph.clone().borrow()[(iNodeIdx.clone()-1) as usize].clone();
    tmpNodesWithRefZero = List::fold1(childNodes.clone(), (std::sync::Arc::new(getLevelNodes2) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), iRefCounter.clone(), metamodelica::nil());
    oNodesWithRefZero = listAppend(tmpNodesWithRefZero.clone(), iNodesWithRefZero.clone());
    Ok(oNodesWithRefZero)
}

fn getLevelNodes2(mut iNodeIdx: i32, mut iRefCounter: metamodelica::Array<i32>, mut iNodesWithRefZero: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut oNodesWithRefZero: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpNodesWithRefZero: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut refCounter: i32 = 0;
    oNodesWithRefZero = 'mc: {
        let __mc_input = (iNodeIdx.clone(), iRefCounter.clone(), iNodesWithRefZero.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, tmpNodesWithRefZero) => {
                    let mut tmpNodesWithRefZero = (*tmpNodesWithRefZero).clone();
                    let mut refCounter: i32 = refCounter.clone();
                    refCounter = iRefCounter.clone().borrow()[(iNodeIdx.clone()-1) as usize].clone() - 1;
                    {let _arr = iRefCounter.clone(); _arr.borrow_mut()[(iNodeIdx.clone()-1) as usize] = refCounter.clone(); _arr};
                    let true = (intEq(refCounter.clone(), 0)) else { bail!("pattern mismatch") };
                    tmpNodesWithRefZero = cons(iNodeIdx.clone(), tmpNodesWithRefZero.clone());
                    Ok(tmpNodesWithRefZero.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iNodesWithRefZero.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oNodesWithRefZero)
}

fn createRefCounter(mut iTaskGraph: TaskGraph) -> metamodelica::Array<i32> {
    let mut oRefCounter: metamodelica::Array<i32>;
    let mut tmpRefCounter: metamodelica::Array<i32>;
    tmpRefCounter = arrayCreate((iTaskGraph.clone().borrow().len() as i32), 0);
    tmpRefCounter = Array::fold(iTaskGraph.clone(), (std::sync::Arc::new(createRefCounter0) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> + 'static>), tmpRefCounter.clone());
    oRefCounter = tmpRefCounter.clone();
    oRefCounter
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn createRefCounter0(mut iChildNodes: Arc<metamodelica::List<i32>>, mut iRefCounter: metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> {
    let mut oRefCounter: metamodelica::Array<i32>;
    let mut tmpRefCounter: metamodelica::Array<i32>;
    let mut counter: i32 = 0;
    let mut head: i32 = 0;
    let mut tail: Arc<metamodelica::List<i32>> = metamodelica::nil();
    oRefCounter = (::match_deref::match_deref! { match &((iChildNodes.clone(), iRefCounter.clone())) {
        (Deref @ metamodelica::List::Nil, _) => iRefCounter.clone(),
        (Deref @ metamodelica::List::Cons { head: head, tail: tail }, _) => {
            counter = iRefCounter.clone().borrow()[(head.clone()-1) as usize].clone() + 1;
            tmpRefCounter = {let _arr = iRefCounter.clone(); _arr.borrow_mut()[(head.clone()-1) as usize] = counter.clone(); _arr};
            tmpRefCounter = createRefCounter0(tail.clone(), tmpRefCounter.clone())?;
            tmpRefCounter.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(oRefCounter)
}

fn getNodesWithRefCountZero(mut iRefCounter: metamodelica::Array<i32>) -> Arc<metamodelica::List<i32>> {
    let mut oZeroIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (oZeroIdc, _) = Array::fold(iRefCounter.clone(), (std::sync::Arc::new(fnptr!(getNodesWithRefCountZero0, i32, (Arc<metamodelica::List<i32>>, i32))) as std::sync::Arc<dyn ::std::ops::Fn(i32, (Arc<metamodelica::List<i32>>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32)> + 'static>), (metamodelica::nil(), 1));
    oZeroIdc
}

fn getNodesWithRefCountZero0(mut iRefCount: i32, mut iZeroIdc: (Arc<metamodelica::List<i32>>, i32)) -> (Arc<metamodelica::List<i32>>, i32) {
    let mut oZeroIdc: (Arc<metamodelica::List<i32>>, i32);
    let mut resultList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut currentNodeIdx: i32 = 0;
    oZeroIdc = (::match_deref::match_deref! { match &((iRefCount.clone(), iZeroIdc.clone())) {
        (0, (resultList, currentNodeIdx)) => {
            let mut resultList = (*resultList).clone();
            resultList = cons(currentNodeIdx.clone(), resultList.clone());
            (resultList.clone(), currentNodeIdx.clone() + 1)
        },
        (_, (resultList, currentNodeIdx)) => (resultList.clone(), currentNodeIdx.clone() + 1),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oZeroIdc
}

//----------------------------------
//  Functions to get the event-graph
//----------------------------------
pub fn getZeroFuncsSystem(mut iTaskGraph: TaskGraph, mut iTaskGraphMeta: TaskGraphMeta, mut iBackendDAE: Arc<BackendDAE::BackendDAE>, mut iNumberOfSccs: i32, mut iZeroCrossingEquationIdc: Arc<metamodelica::List<i32>>, mut iSimCodeEqCompMapping: metamodelica::Array<i32>) -> Result<(TaskGraph, TaskGraphMeta)> {
    let mut oTaskGraph: TaskGraph;
    let mut oTaskGraphMeta: TaskGraphMeta;
    let mut zeroCrossingNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut nodeList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut newNodeList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut predecessors: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut successors: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut successorsTmp: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut predecessorsTmp: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut zeroFuncNodeMarks: metamodelica::Array<i32>;
    let mut sccNodeMapping: metamodelica::Array<i32>;
    let mut handledNodes: metamodelica::Array<bool>;
    let mut whenNodeMarks: metamodelica::Array<bool>;
    let mut iTaskGraphTCopy: TaskGraph;
    let mut iTaskGraphCopy: TaskGraph;
    let mut zeroFuncTaskGraph: TaskGraph;
    let mut zeroFuncTaskGraphMeta: TaskGraphMeta;
    let mut whenNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut inCompsEntry: i32 = 0;
    let mut zeroFuncInComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqIdx: i32 = 0;
    let mut compIdx: i32 = 0;
    let mut nodeIdx: i32 = 0;
    let mut successor: i32 = 0;
    let mut predecessor: i32 = 0;
    let mut zeroFuncNodeMark: i32 = 0;
    let mut successorMark: i32 = 0;
    let mut zeroFuncNodeCount: i32 = 0;
    let mut zeroFuncNodeIdx: i32 = 0;
    let mut nodeToZeroFuncNodeMapping: metamodelica::Array<i32>;
    let mut stop: bool = false;
    let TaskGraphMeta { eqCompMapping: __pa0, inComps: __pa1, .. } = (iTaskGraphMeta.clone()) else { bail!("pattern mismatch") };
    eqCompMapping = __pa0.clone();
    inComps = __pa1.clone();
    zeroFuncNodeMarks = arrayCreate((iTaskGraph.clone().borrow().len() as i32), 0);
    handledNodes = arrayCreate((iTaskGraph.clone().borrow().len() as i32), false);
    nodeToZeroFuncNodeMapping = arrayCreate((iTaskGraph.clone().borrow().len() as i32), -1);
    whenNodes = getEventNodes(iBackendDAE.clone(), eqCompMapping.clone())?;
    whenNodeMarks = arrayCreate((iTaskGraph.clone().borrow().len() as i32), false);
    sccNodeMapping = getSccNodeMapping(iNumberOfSccs.clone(), iTaskGraphMeta.clone())?;
    iTaskGraphCopy = metamodelica::arrayFromVec(iTaskGraph.clone().borrow().clone());
    iTaskGraphTCopy = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), (iTaskGraph.clone().borrow().len() as i32))?;
    for mut eqIdx in &*iZeroCrossingEquationIdc.clone() {
        let mut eqIdx = eqIdx.clone();
        compIdx = iSimCodeEqCompMapping.clone().borrow()[(eqIdx.clone()-1) as usize].clone();
        nodeIdx = sccNodeMapping.clone().borrow()[(compIdx.clone()-1) as usize].clone();
        zeroFuncNodeMarks = {let _arr = zeroFuncNodeMarks.clone(); _arr.borrow_mut()[(nodeIdx.clone()-1) as usize] = 1; _arr};
    }
    for mut nodeIdx in &*whenNodes.clone() {
        let mut nodeIdx = nodeIdx.clone();
        whenNodeMarks = {let _arr = whenNodeMarks.clone(); _arr.borrow_mut()[(nodeIdx.clone()-1) as usize] = true; _arr};
    }
    nodeList = getRootNodes(iTaskGraphTCopy.clone())?;
    zeroFuncNodeCount = 0;
    zeroFuncNodeIdx = 1;
    while boolNot(nodeList.clone().is_empty()) {
        newNodeList = metamodelica::nil();
        for mut nodeIdx in &*nodeList.clone() {
            let mut nodeIdx = nodeIdx.clone();
            if boolNot(handledNodes.clone().borrow()[(nodeIdx.clone()-1) as usize].clone()) {
                handledNodes = {let _arr = handledNodes.clone(); _arr.borrow_mut()[(nodeIdx.clone()-1) as usize] = true; _arr};
                predecessors = iTaskGraphTCopy.clone().borrow()[(nodeIdx.clone()-1) as usize].clone();
                successors = iTaskGraphCopy.clone().borrow()[(nodeIdx.clone()-1) as usize].clone();
                zeroFuncNodeMark = -1;
                if whenNodeMarks.clone().borrow()[(nodeIdx.clone()-1) as usize].clone() {
                    for mut predecessor in &*predecessors.clone() {
                        let mut predecessor = predecessor.clone();
                        successorsTmp = iTaskGraphCopy.clone().borrow()[(predecessor.clone()-1) as usize].clone();
                        {let _arr = iTaskGraphCopy.clone(); _arr.borrow_mut()[(predecessor.clone()-1) as usize] = listAppend(successorsTmp.clone(), successors.clone()); _arr};
                    }
                    for mut successor in &*successors.clone() {
                        let mut successor = successor.clone();
                        predecessorsTmp = iTaskGraphTCopy.clone().borrow()[(successor.clone()-1) as usize].clone();
                        {let _arr = iTaskGraphTCopy.clone(); _arr.borrow_mut()[(successor.clone()-1) as usize] = listAppend(predecessorsTmp.clone(), predecessors.clone()); _arr};
                    }
                } else {
                    if intGt(zeroFuncNodeMarks.clone().borrow()[(nodeIdx.clone()-1) as usize].clone(), 0) {
                        zeroFuncNodeMark = zeroFuncNodeIdx.clone();
                    } else {
                        stop = false;
                        while boolAnd(boolNot(stop.clone()), boolNot(successors.clone().is_empty())) {
                            let (__pa2, __pa3) = ::match_deref::match_deref! { match &(successors.clone()) {
                                Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                                _ => bail!("pattern mismatch"),
                            } };
                            successor = __pa2.clone();
                            successors = __pa3.clone();
                            successorMark = zeroFuncNodeMarks.clone().borrow()[(successor.clone()-1) as usize].clone();
                            if intGt(successorMark.clone(), 0) {
                                zeroFuncNodeMark = zeroFuncNodeIdx.clone();
                                stop = true;
                            }
                        }
                    }
                    if intGt(zeroFuncNodeMark.clone(), 0) {
                        zeroFuncNodeCount = zeroFuncNodeCount.clone() + 1;
                        nodeToZeroFuncNodeMapping = {let _arr = nodeToZeroFuncNodeMapping.clone(); _arr.borrow_mut()[(nodeIdx.clone()-1) as usize] = zeroFuncNodeCount.clone(); _arr};
                        zeroFuncNodeIdx = zeroFuncNodeIdx.clone() + 1;
                    }
                }
                zeroFuncNodeMarks = {let _arr = zeroFuncNodeMarks.clone(); _arr.borrow_mut()[(nodeIdx.clone()-1) as usize] = zeroFuncNodeMark.clone(); _arr};
                newNodeList = List::append_reverse(predecessors.clone(), newNodeList.clone());
            }
        }
        nodeList = newNodeList.clone().reverse();
    }
    zeroFuncTaskGraph = arrayCreate(zeroFuncNodeCount.clone(), metamodelica::nil());
    zeroFuncInComps = arrayCreate(zeroFuncNodeCount.clone(), metamodelica::nil());
    nodeIdx = (zeroFuncNodeMarks.clone().borrow().len() as i32);
    while intGt(nodeIdx.clone(), 0) {
        zeroFuncNodeIdx = zeroFuncNodeMarks.clone().borrow()[(nodeIdx.clone()-1) as usize].clone();
        if intGt(zeroFuncNodeIdx.clone(), 0) {
            successors = iTaskGraphCopy.clone().borrow()[(nodeIdx.clone()-1) as usize].clone();
            zeroFuncInComps = {let _arr = zeroFuncInComps.clone(); let _val = inComps.clone().borrow()[(nodeIdx.clone()-1) as usize].clone(); _arr.borrow_mut()[(zeroFuncNodeIdx.clone()-1) as usize] = _val; _arr};
            newNodeList = metamodelica::nil();
            while boolNot(successors.clone().is_empty()) {
                let (__pa4, __pa5) = ::match_deref::match_deref! { match &(successors.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                successor = __pa4.clone();
                successors = __pa5.clone();
                successor = zeroFuncNodeMarks.clone().borrow()[(successor.clone()-1) as usize].clone();
                if intGt(successor.clone(), 0) {
                    newNodeList = cons(successor.clone(), newNodeList.clone());
                }
            }
            newNodeList = List::sort(newNodeList.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            newNodeList = List::sortedUnique(newNodeList.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            zeroFuncTaskGraph = {let _arr = zeroFuncTaskGraph.clone(); _arr.borrow_mut()[(zeroFuncNodeIdx.clone()-1) as usize] = newNodeList.clone(); _arr};
        }
        nodeIdx = nodeIdx.clone() - 1;
    }
    zeroFuncTaskGraphMeta = copyTaskGraphMeta(iTaskGraphMeta.clone())?;
    zeroFuncTaskGraphMeta = setInCompsInMeta(zeroFuncInComps.clone(), zeroFuncTaskGraphMeta.clone())?;
    (oTaskGraph, oTaskGraphMeta) = reverseTaskGraphIndices(zeroFuncTaskGraph.clone(), zeroFuncTaskGraphMeta.clone())?;
    Ok((oTaskGraph, oTaskGraphMeta))
}

fn reverseTaskGraphIndices(mut iTaskGraph: TaskGraph, mut iTaskGraphMeta: TaskGraphMeta) -> Result<(TaskGraph, TaskGraphMeta)> {
    let mut oTaskGraph: TaskGraph;
    let mut oTaskGraphMeta: TaskGraphMeta;
    let mut nTasks: i32 = 0;
    let mut idxMap: metamodelica::Array<i32>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut compParamMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut compInformations: metamodelica::Array<ComponentInfo>;
    nTasks = (iTaskGraph.clone().borrow().len() as i32);
    idxMap = arrayCreate(nTasks.clone(), -1);
    let TaskGraphMeta { compInformations: __pa0, nodeMark: __pa1, commCosts: __pa2, exeCosts: __pa3, compDescs: __pa4, compNames: __pa5, compParamMapping: __pa6, eqCompMapping: __pa7, varCompMapping: __pa8, inComps: __pa9 } = (iTaskGraphMeta.clone()) else { bail!("pattern mismatch") };
    compInformations = __pa0.clone();
    nodeMark = __pa1.clone();
    commCosts = __pa2.clone();
    exeCosts = __pa3.clone();
    compDescs = __pa4.clone();
    compNames = __pa5.clone();
    compParamMapping = __pa6.clone();
    eqCompMapping = __pa7.clone();
    varCompMapping = __pa8.clone();
    inComps = __pa9.clone();
    for mut i in 1..=nTasks.clone() {
        idxMap = {let _arr = idxMap.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = nTasks.clone() - i.clone() + 1; _arr};
    }
    (oTaskGraph, _) = Array::mapNoCopy_1(iTaskGraph.clone(), (std::sync::Arc::new(mapIntegers) as std::sync::Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<(Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)> + 'static>), idxMap.clone());
    oTaskGraph = Array::reverse(oTaskGraph.clone())?;
    inComps = Array::reverse(inComps.clone())?;
    oTaskGraphMeta = TaskGraphMeta { inComps: inComps.clone(), varCompMapping: varCompMapping.clone(), eqCompMapping: eqCompMapping.clone(), compParamMapping: compParamMapping.clone(), compNames: compNames.clone(), compDescs: compDescs.clone(), exeCosts: exeCosts.clone(), commCosts: commCosts.clone(), nodeMark: nodeMark.clone(), compInformations: compInformations.clone() };
    Ok((oTaskGraph, oTaskGraphMeta))
}

fn mapIntegers(mut iTpl: (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<(Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)> {
    let mut oTpl: (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>);
    let mut map: metamodelica::Array<i32>;
    let mut iLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut oLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (iLst, map) = iTpl.clone();
    for mut i in &*iLst.clone() {
        let mut i = i.clone();
        oLst = cons(map.clone().borrow()[(i.clone()-1) as usize].clone(), oLst.clone());
    }
    oLst = oLst.clone().reverse();
    oTpl = (oLst.clone(), map.clone());
    Ok(oTpl)
}

fn getEventSystem(mut iTaskGraph: TaskGraph, mut iTaskGraphMeta: TaskGraphMeta, mut iSyst: Arc<BackendDAE::BackendDAE>, mut iZeroCrossings: Arc<metamodelica::List<BackendDAE::ZeroCrossing>>, mut iSimCodeEqCompMapping: metamodelica::Array<i32>) -> Result<(TaskGraph, TaskGraphMeta)> {
    let mut oTaskGraph: TaskGraph;
    let mut oTaskGraphMeta: TaskGraphMeta;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut discreteNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut cutNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut cutNodeChildren: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut zeroCrossingNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut sccsContainingTime: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared>;
    let mut graphTmp: TaskGraph;
    let mut graphDataTmp: TaskGraphMeta;
    let TaskGraphMeta { inComps: __pa0, eqCompMapping: __pa1, varCompMapping: __pa2, .. } = (iTaskGraphMeta.clone()) else { bail!("pattern mismatch") };
    inComps = __pa0.clone();
    eqCompMapping = __pa1.clone();
    varCompMapping = __pa2.clone();
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(iSyst.clone()) {
        Deref @ DAE { UNIQUEIO: __pa3, derivativeNamePrefix: __pa4, .. } => (__pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    systs = __pa3.clone();
    shared = __pa4.clone();
    discreteNodes = getDiscreteNodes(iSyst.clone(), eqCompMapping.clone())?;
    zeroCrossingNodes = List::flatten(List::map1(iZeroCrossings.clone(), (std::sync::Arc::new(getComponentsOfZeroCrossing) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::ZeroCrossing, metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), iSimCodeEqCompMapping.clone()));
    sccsContainingTime = metamodelica::nil();
    discreteNodes = List::flatten(list![discreteNodes.clone(), sccsContainingTime.clone(), zeroCrossingNodes.clone()]);
    graphTmp = iTaskGraph.clone();
    (graphTmp, cutNodes) = cutTaskGraph(graphTmp.clone(), discreteNodes.clone(), metamodelica::nil())?;
    cutNodeChildren = List::flatten(List::map1(cutNodes.clone(), std::sync::Arc::new(fnptr!(Array::getIndexFirst, i32, _)), iTaskGraph.clone()));
    (_, cutNodeChildren, _) = List::intersection1OnTrue(cutNodeChildren.clone(), cutNodes.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    oTaskGraphMeta = cutSystemData(iTaskGraphMeta.clone(), cutNodes.clone(), cutNodeChildren.clone())?;
    oTaskGraph = graphTmp.clone();
    Ok((oTaskGraph, oTaskGraphMeta))
}

fn getComponentsOfZeroCrossing(mut iZeroCrossing: BackendDAE::ZeroCrossing, mut iSimCodeEqCompMapping: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut oCompIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut occurEquLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpCompIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    oCompIdc = 'mc: {
        let __mc_input = (iZeroCrossing.clone(), iSimCodeEqCompMapping.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (BackendDAE::ZeroCrossing { occurEquLst: mut occurEquLst, .. }, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut occurEquLst = occurEquLst.clone();
            let mut tmpCompIdc: Arc<metamodelica::List<i32>> = tmpCompIdc.clone();
            occurEquLst = List::filter1OnTrue(occurEquLst.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0);
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getComponentsOfZeroCrossing: simEqs: ")); __mm_s.push_str(&*stringDelimitList(List::map(occurEquLst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            tmpCompIdc = List::map1(occurEquLst.clone(), std::sync::Arc::new(fnptr!(Array::getIndexFirst, i32, _)), iSimCodeEqCompMapping.clone());
            tmpCompIdc = List::filter1OnTrue(tmpCompIdc.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0);
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getComponentsOfZeroCrossing: components: ")); __mm_s.push_str(&*stringDelimitList(List::map(tmpCompIdc.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            Ok(tmpCompIdc.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(metamodelica::nil())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oCompIdc)
}

fn getComponentsIncludingTime(mut iSystem: Arc<BackendDAE::EqSystem>, mut iEqCompMapping: metamodelica::Array<(i32, i32, i32)>, mut iOffsetResList: (i32, Arc<metamodelica::List<i32>>)) -> Result<(i32, Arc<metamodelica::List<i32>>)> {
    let mut oOffsetResList: (i32, Arc<metamodelica::List<i32>>);
    let mut orderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut offset: i32 = 0;
    let mut resultList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(iSystem.clone()) {
        Deref @ BackendDAE::EqSystem { orderedEqs: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    orderedEqs = __pa0.clone();
    (offset, resultList) = iOffsetResList.clone();
    (offset, resultList, _, _) = BackendEquation::traverseEquationArray(orderedEqs.clone(), (std::sync::Arc::new(getComponentsIncludingTime0) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (i32, Arc<metamodelica::List<i32>>, metamodelica::Array<(i32, i32, i32)>, i32)) -> Result<(Arc<BackendDAE::Equation>, (i32, Arc<metamodelica::List<i32>>, metamodelica::Array<(i32, i32, i32)>, i32))> + 'static>), (offset.clone(), resultList.clone(), iEqCompMapping.clone(), 1))?;
    oOffsetResList = (offset.clone(), resultList.clone());
    Ok(oOffsetResList)
}

fn getComponentsIncludingTime0(mut inEq: Arc<BackendDAE::Equation>, mut iOffsetResList: (i32, Arc<metamodelica::List<i32>>, metamodelica::Array<(i32, i32, i32)>, i32)) -> Result<(Arc<BackendDAE::Equation>, (i32, Arc<metamodelica::List<i32>>, metamodelica::Array<(i32, i32, i32)>, i32))> {
    let mut outEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut oOffsetResList: (i32, Arc<metamodelica::List<i32>>, metamodelica::Array<(i32, i32, i32)>, i32);
    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut offset: i32 = 0;
    let mut eqIdx: i32 = 0;
    let mut sccIdx: i32 = 0;
    let mut resultList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut timeIsPartOfEquation: bool = false;
    (outEq, oOffsetResList) = 'mc: {
        let __mc_input = (inEq.clone(), iOffsetResList.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eq, (offset, resultList, eqCompMapping, eqIdx)) => {
                    let mut resultList = (*resultList).clone();
                    let mut sccIdx: i32 = sccIdx.clone();
                    (sccIdx, _, _) = eqCompMapping.clone().borrow()[(eqIdx.clone() + offset.clone()-1) as usize].clone();
                    let true = (BackendDAEUtil::traverseBackendDAEExpsOptEqn(Some(eq.clone()), (std::sync::Arc::new(getComponentsIncludingTime1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false)?) else { bail!("pattern mismatch") };
                    resultList = cons(sccIdx.clone(), resultList.clone());
                    Ok((eq.clone(), (offset.clone(), resultList.clone(), eqCompMapping.clone(), eqIdx.clone() + 1)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eq, (offset, resultList, eqCompMapping, eqIdx)) => {
                    Ok((eq.clone(), (offset.clone(), resultList.clone(), eqCompMapping.clone(), eqIdx.clone() + 1)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outEq, oOffsetResList))
}

fn getComponentsIncludingTime1(mut inExp: Arc<DAE::Exp>, mut inB: bool) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut e: Arc<DAE::Exp>;
    let mut res: bool = false;
    (e, res) = (::match_deref::match_deref! { match &((inExp.clone(), inB.clone())) {
        (e, false) => {
            res = Expression::traverseCrefsFromExp(e.clone(), (std::sync::Arc::new(fnptr!(getComponentsIncludingTime2, Arc<DAE::ComponentRef>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, bool) -> Result<bool> + 'static>), false)?;
            (e.clone(), res.clone())
        },
        _ => (inExp.clone(), inB.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((e, res))
}

fn getComponentsIncludingTime2(mut iRef: Arc<DAE::ComponentRef>, mut iIncludingTime: bool) -> bool {
    let mut oIncludingTime: bool = false;
    oIncludingTime = (::match_deref::match_deref! { match &((iRef.clone(), iIncludingTime.clone())) {
        (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. }, _) => true,
        _ => false || iIncludingTime.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oIncludingTime
}

fn getDiscreteNodes(mut systIn: Arc<BackendDAE::BackendDAE>, mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut eventNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tplLst: Arc<metamodelica::List<(i32, i32, i32)>> = metamodelica::nil();
    let mut systemsIn: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(systIn.clone()) {
        Deref @ DAE { eqs: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    systemsIn = __pa0.clone();
    (eqLst, _) = List::fold(systemsIn.clone(), (std::sync::Arc::new(getDiscreteNodesEqs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, (Arc<metamodelica::List<i32>>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32)> + 'static>), (metamodelica::nil(), 0));
    eventNodes = getArrayTuple31(eqLst.clone(), eqCompMapping.clone());
    Ok(eventNodes)
}

fn getDiscreteNodesEqs(mut systIn: Arc<BackendDAE::EqSystem>, mut eventInfoIn: (Arc<metamodelica::List<i32>>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32)> {
    let mut eventInfoOut: (Arc<metamodelica::List<i32>>, i32);
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut orderedVars: BackendDAE::Variables;
    let mut matching: Arc<BackendDAE::Matching> = Arc::new(BackendDAE::Matching::NO_MATCHING);
    let mut orderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut eventEqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eventEqsIn: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut numOfEqs: i32 = 0;
    let mut offset: i32 = 0;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(systIn.clone()) {
        Deref @ BackendDAE::EqSystem { matching: __pa0, orderedVars: __pa1, orderedEqs: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    matching = __pa0.clone();
    orderedVars = __pa1.clone();
    orderedEqs = __pa2.clone();
    comps = BackendDAEUtil::getCompsOfMatching(matching.clone());
    (eventEqsIn, offset) = eventInfoIn.clone();
    eventEqs = getDiscreteNodesEqs1(comps.clone(), offset.clone(), orderedVars.clone(), metamodelica::nil())?;
    offset = offset.clone() + ExpandableArray::getNumberOfElements(orderedEqs.clone());
    eventInfoOut = (listAppend(eventEqs.clone(), eventEqsIn.clone()), offset.clone());
    Ok(eventInfoOut)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getDiscreteNodesEqs1(mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut offset: i32, mut iOrderedVars: BackendDAE::Variables, mut discreteEqsIn: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut discreteEqsOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    discreteEqsOut = 'mc: {
        let __mc_input = (comps.clone(), offset.clone(), iOrderedVars.clone(), discreteEqsIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: head, tail: rest }, _, _, _) => {
                    let mut eqn: i32 = 0;
                    let mut eventEqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let (true, __pa0) = (solvesDiscreteValue(head.clone(), iOrderedVars.clone())?) else { bail!("pattern mismatch") };
                    eqn = __pa0.clone();
                    eqn = eqn.clone() + offset.clone();
                    eventEqs = getDiscreteNodesEqs1(rest.clone(), offset.clone(), iOrderedVars.clone(), cons(eqn.clone(), discreteEqsIn.clone()))?;
                    Ok(eventEqs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _, _, _) => {
                    let mut eventEqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    eventEqs = getDiscreteNodesEqs1(rest.clone(), offset.clone(), iOrderedVars.clone(), discreteEqsIn.clone())?;
                    Ok(eventEqs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _) => {
                    Ok(discreteEqsIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(discreteEqsOut)
}

fn solvesDiscreteValue(mut inComp: Arc<BackendDAE::StrongComponent>, mut iOrderedVars: BackendDAE::Variables) -> Result<(bool, i32)> {
    let mut oSolvesDiscreteValue: bool = false;
    let mut oFirstEqIdx: i32 = 0;
    (oSolvesDiscreteValue, oFirstEqIdx) = 'mc: {
        let __mc_input = (inComp.clone(), iOrderedVars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { eqn, var }, _) => {
                    let mut backendVar: BackendDAE::Var;
                    let mut solvesDiscreteValue: bool = false;
                    backendVar = BackendVariable::getVarAt(iOrderedVars.clone(), var.clone())?;
                    solvesDiscreteValue = BackendVariable::isVarDiscrete(backendVar.clone());
                    Ok((solvesDiscreteValue.clone(), eqn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns, vars, .. }, _) => {
                    let mut eqn: i32 = 0;
                    let mut backendVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut solvesDiscreteValue: bool = false;
                    backendVars = List::map1r(vars.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), iOrderedVars.clone());
                    solvesDiscreteValue = BackendVariable::hasDiscreteVar(backendVars.clone());
                    eqn = listHead(eqns.clone())?;
                    Ok((solvesDiscreteValue.clone(), eqn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEARRAY { eqn, vars }, _) => {
                    let mut backendVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut solvesDiscreteValue: bool = false;
                    backendVars = List::map1r(vars.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), iOrderedVars.clone());
                    solvesDiscreteValue = BackendVariable::hasDiscreteVar(backendVars.clone());
                    Ok((solvesDiscreteValue.clone(), eqn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { eqn, vars }, _) => {
                    let mut backendVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut solvesDiscreteValue: bool = false;
                    backendVars = List::map1r(vars.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), iOrderedVars.clone());
                    solvesDiscreteValue = BackendVariable::hasDiscreteVar(backendVars.clone());
                    Ok((solvesDiscreteValue.clone(), eqn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { eqn, vars }, _) => {
                    let mut backendVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut solvesDiscreteValue: bool = false;
                    backendVars = List::map1r(vars.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), iOrderedVars.clone());
                    solvesDiscreteValue = BackendVariable::hasDiscreteVar(backendVars.clone());
                    Ok((solvesDiscreteValue.clone(), eqn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { eqn, vars }, _) => {
                    let mut backendVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut solvesDiscreteValue: bool = false;
                    backendVars = List::map1r(vars.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), iOrderedVars.clone());
                    solvesDiscreteValue = BackendVariable::hasDiscreteVar(backendVars.clone());
                    Ok((solvesDiscreteValue.clone(), eqn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { eqn, vars }, _) => {
                    let mut backendVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut solvesDiscreteValue: bool = false;
                    backendVars = List::map1r(vars.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), iOrderedVars.clone());
                    solvesDiscreteValue = BackendVariable::hasDiscreteVar(backendVars.clone());
                    Ok((solvesDiscreteValue.clone(), eqn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((false, -1))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((oSolvesDiscreteValue, oFirstEqIdx))
}

//------------------------------------------
//Methods to write blt-structure as xml-file
//------------------------------------------
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GraphDumpOptions {
    pub visualizeCriticalPath: bool,
    pub visualizeTaskStartAndFinishTime: bool,
    pub visualizeTaskCalcTime: bool,
    pub visualizeCommTime: bool,
}

pub type GRAPHDUMPOPTIONS = GraphDumpOptions;


pub fn dumpTaskGraph(mut dae: Arc<BackendDAE::BackendDAE>, mut fileName: ArcStr) -> Result<()> {
    let mut name: ArcStr = arcstr::literal!("");
    let mut taskGraph: TaskGraph;
    let mut taskGraphData: TaskGraphMeta;
    let mut schedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>;
    let mut sccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    (taskGraph, taskGraphData) = createTaskGraph(dae.clone(), false)?;
    name = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TaskGraph_")); __mm_s.push_str(&*fileName.clone()); __mm_s.push_str(&*literal!(".graphml")); ArcStr::from(__mm_s) }).clone();
    schedulerInfo = arrayCreate((taskGraph.clone().borrow().len() as i32), (-1, -1, metamodelica::OrderedFloat(-1.0_f64)));
    sccSimEqMapping = arrayCreate((taskGraph.clone().borrow().len() as i32), list![-1]);
    dumpAsGraphMLSccLevel(taskGraph.clone(), taskGraphData.clone(), (name.clone()).clone(), (literal!("")).clone(), metamodelica::nil(), metamodelica::nil(), sccSimEqMapping.clone(), schedulerInfo.clone(), GraphDumpOptions { visualizeCriticalPath: false, visualizeTaskStartAndFinishTime: false, visualizeTaskCalcTime: true, visualizeCommTime: true })?;
    Ok(())
}

pub fn dumpAsGraphMLSccLevel(mut iGraph: TaskGraph, mut iGraphData: TaskGraphMeta, mut iFileName: ArcStr, mut iCriticalPathInfo: ArcStr, mut iCriticalPath: Arc<metamodelica::List<(i32, i32)>>, mut iCriticalPathWoC: Arc<metamodelica::List<(i32, i32)>>, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>, mut iGraphDumpOptions: GraphDumpOptions) -> Result<()> {
    let mut graphInfo: GraphML::GraphInfo;
    graphInfo = convertToGraphMLSccLevel(iGraph.clone(), iGraphData.clone(), (iCriticalPathInfo.clone()).clone(), iCriticalPath.clone(), iCriticalPathWoC.clone(), iSccSimEqMapping.clone(), iSchedulerInfo.clone(), iGraphDumpOptions.clone())?;
    GraphML::dumpGraph(graphInfo.clone(), (iFileName.clone()).clone())?;
    Ok(())
}

pub fn convertToGraphMLSccLevel(mut iGraph: TaskGraph, mut iGraphData: TaskGraphMeta, mut iCriticalPathInfo: ArcStr, mut iCriticalPath: Arc<metamodelica::List<(i32, i32)>>, mut iCriticalPathWoC: Arc<metamodelica::List<(i32, i32)>>, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>, mut iGraphDumpOptions: GraphDumpOptions) -> Result<GraphML::GraphInfo> {
    let mut oGraphInfo: GraphML::GraphInfo;
    let mut graphIdx: i32 = 0;
    let mut annotationInfo: metamodelica::Array<ArcStr>;
    let mut graphInfo: GraphML::GraphInfo;
    graphInfo = GraphML::createGraphInfo();
    let (__pa0, (_, __pa1)) = GraphML::addGraph((literal!("TaskGraph")).clone(), true, graphInfo.clone())?;
    graphInfo = __pa0.clone();
    graphIdx = __pa1.clone();
    annotationInfo = arrayCreate((iGraph.clone().borrow().len() as i32), (literal!("uncomment in HpcOmTaskGraph and +showAnnotations")).clone());
    oGraphInfo = convertToGraphMLSccLevelSubgraph(iGraph.clone(), iGraphData.clone(), (iCriticalPathInfo.clone()).clone(), iCriticalPath.clone(), iCriticalPathWoC.clone(), iSccSimEqMapping.clone(), iSchedulerInfo.clone(), annotationInfo.clone(), graphIdx.clone(), iGraphDumpOptions.clone(), graphInfo.clone())?;
    Ok(oGraphInfo)
}

pub fn convertToGraphMLSccLevelSubgraph(mut iGraph: TaskGraph, mut iGraphData: TaskGraphMeta, mut iCriticalPathInfo: ArcStr, mut iCriticalPath: Arc<metamodelica::List<(i32, i32)>>, mut iCriticalPathWoC: Arc<metamodelica::List<(i32, i32)>>, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>, mut iAnnotationInfo: metamodelica::Array<ArcStr>, mut iGraphIdx: i32, mut iGraphDumpOptions: GraphDumpOptions, mut iGraphInfo: GraphML::GraphInfo) -> Result<GraphML::GraphInfo> {
    let mut oGraphInfo: GraphML::GraphInfo;
    let mut graphInfo: GraphML::GraphInfo;
    let mut nameAttIdx: i32 = 0;
    let mut calcTimeAttIdx: i32 = 0;
    let mut opCountAttIdx: i32 = 0;
    let mut yCoordAttIdx: i32 = 0;
    let mut taskIdAttIdx: i32 = 0;
    let mut commCostAttIdx: i32 = 0;
    let mut commVarsAttIdx: i32 = 0;
    let mut commVarsIntAttIdx: i32 = 0;
    let mut commVarsFloatAttIdx: i32 = 0;
    let mut commVarsBoolAttIdx: i32 = 0;
    let mut critPathAttIdx: i32 = 0;
    let mut simCodeEqAttIdx: i32 = 0;
    let mut threadIdAttIdx: i32 = 0;
    let mut taskNumberAttIdx: i32 = 0;
    let mut annotAttIdx: i32 = 0;
    let mut compsIdAttIdx: i32 = 0;
    let mut partOfEventAttIdx: i32 = 0;
    let mut partOfOdeAttIdx: i32 = 0;
    let mut removedCompAttIdx: i32 = 0;
    let mut nodeIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    oGraphInfo = (::match_deref::match_deref! { match &((iGraph.clone(), iGraphData.clone(), iCriticalPathInfo.clone(), iCriticalPath.clone(), iCriticalPathWoC.clone(), iSccSimEqMapping.clone(), iSchedulerInfo.clone(), iAnnotationInfo.clone(), iGraphIdx.clone(), iGraphDumpOptions.clone(), iGraphInfo.clone())) {
        (_, _, _, _, _, _, _, _, _, _, _) => {
            let (__pa0, (_, __pa1)) = GraphML::addAttribute((literal!("")).clone(), (literal!("Name")).clone(), openmodelica_susan::GraphML::AttributeType::TYPE_STRING, openmodelica_susan::GraphML::AttributeTarget::TARGET_NODE, iGraphInfo.clone())?;
            graphInfo = __pa0.clone();
            nameAttIdx = __pa1.clone();
            let (__pa2, (_, __pa3)) = GraphML::addAttribute((literal!("-1")).clone(), (literal!("Operations")).clone(), openmodelica_susan::GraphML::AttributeType::TYPE_INTEGER, openmodelica_susan::GraphML::AttributeTarget::TARGET_NODE, graphInfo.clone())?;
            graphInfo = __pa2.clone();
            opCountAttIdx = __pa3.clone();
            let (__pa4, (_, __pa5)) = GraphML::addAttribute((literal!("-1")).clone(), (literal!("CalcTime")).clone(), openmodelica_susan::GraphML::AttributeType::TYPE_DOUBLE, openmodelica_susan::GraphML::AttributeTarget::TARGET_NODE, graphInfo.clone())?;
            graphInfo = __pa4.clone();
            calcTimeAttIdx = __pa5.clone();
            let (__pa6, (_, __pa7)) = GraphML::addAttribute((literal!("")).clone(), (literal!("TaskID")).clone(), openmodelica_susan::GraphML::AttributeType::TYPE_STRING, openmodelica_susan::GraphML::AttributeTarget::TARGET_NODE, graphInfo.clone())?;
            graphInfo = __pa6.clone();
            taskIdAttIdx = __pa7.clone();
            let (__pa8, (_, __pa9)) = GraphML::addAttribute((literal!("")).clone(), (literal!("Components")).clone(), openmodelica_susan::GraphML::AttributeType::TYPE_STRING, openmodelica_susan::GraphML::AttributeTarget::TARGET_NODE, graphInfo.clone())?;
            graphInfo = __pa8.clone();
            compsIdAttIdx = __pa9.clone();
            let (__pa10, (_, __pa11)) = GraphML::addAttribute((literal!("17")).clone(), (literal!("yCoord")).clone(), openmodelica_susan::GraphML::AttributeType::TYPE_INTEGER, openmodelica_susan::GraphML::AttributeTarget::TARGET_NODE, graphInfo.clone())?;
            graphInfo = __pa10.clone();
            yCoordAttIdx = __pa11.clone();
            let (__pa12, (_, __pa13)) = GraphML::addAttribute((literal!("")).clone(), (literal!("SimCodeEqs")).clone(), openmodelica_susan::GraphML::AttributeType::TYPE_STRING, openmodelica_susan::GraphML::AttributeTarget::TARGET_NODE, graphInfo.clone())?;
            graphInfo = __pa12.clone();
            simCodeEqAttIdx = __pa13.clone();
            let (__pa14, (_, __pa15)) = GraphML::addAttribute((literal!("")).clone(), (literal!("ThreadId")).clone(), openmodelica_susan::GraphML::AttributeType::TYPE_STRING, openmodelica_susan::GraphML::AttributeTarget::TARGET_NODE, graphInfo.clone())?;
            graphInfo = __pa14.clone();
            threadIdAttIdx = __pa15.clone();
            let (__pa16, (_, __pa17)) = GraphML::addAttribute((literal!("-1")).clone(), (literal!("TaskNumber")).clone(), openmodelica_susan::GraphML::AttributeType::TYPE_INTEGER, openmodelica_susan::GraphML::AttributeTarget::TARGET_NODE, graphInfo.clone())?;
            graphInfo = __pa16.clone();
            taskNumberAttIdx = __pa17.clone();
            let (__pa18, (_, __pa19)) = GraphML::addAttribute((literal!("-1")).clone(), (literal!("CommCost")).clone(), openmodelica_susan::GraphML::AttributeType::TYPE_DOUBLE, openmodelica_susan::GraphML::AttributeTarget::TARGET_EDGE, graphInfo.clone())?;
            graphInfo = __pa18.clone();
            commCostAttIdx = __pa19.clone();
            let (__pa20, (_, __pa21)) = GraphML::addAttribute((literal!("-1")).clone(), (literal!("CommVars")).clone(), openmodelica_susan::GraphML::AttributeType::TYPE_INTEGER, openmodelica_susan::GraphML::AttributeTarget::TARGET_EDGE, graphInfo.clone())?;
            graphInfo = __pa20.clone();
            commVarsAttIdx = __pa21.clone();
            let (__pa22, (_, __pa23)) = GraphML::addAttribute((literal!("-1")).clone(), (literal!("CommVarsInt")).clone(), openmodelica_susan::GraphML::AttributeType::TYPE_INTEGER, openmodelica_susan::GraphML::AttributeTarget::TARGET_EDGE, graphInfo.clone())?;
            graphInfo = __pa22.clone();
            commVarsIntAttIdx = __pa23.clone();
            let (__pa24, (_, __pa25)) = GraphML::addAttribute((literal!("-1")).clone(), (literal!("CommVarsFloat")).clone(), openmodelica_susan::GraphML::AttributeType::TYPE_INTEGER, openmodelica_susan::GraphML::AttributeTarget::TARGET_EDGE, graphInfo.clone())?;
            graphInfo = __pa24.clone();
            commVarsFloatAttIdx = __pa25.clone();
            let (__pa26, (_, __pa27)) = GraphML::addAttribute((literal!("-1")).clone(), (literal!("CommVarsBool")).clone(), openmodelica_susan::GraphML::AttributeType::TYPE_INTEGER, openmodelica_susan::GraphML::AttributeTarget::TARGET_EDGE, graphInfo.clone())?;
            graphInfo = __pa26.clone();
            commVarsBoolAttIdx = __pa27.clone();
            let (__pa28, (_, __pa29)) = GraphML::addAttribute((literal!("annotation")).clone(), (literal!("Annotations")).clone(), openmodelica_susan::GraphML::AttributeType::TYPE_STRING, openmodelica_susan::GraphML::AttributeTarget::TARGET_NODE, graphInfo.clone())?;
            graphInfo = __pa28.clone();
            annotAttIdx = __pa29.clone();
            let (__pa30, (_, __pa31)) = GraphML::addAttribute((literal!("")).clone(), (literal!("CriticalPath")).clone(), openmodelica_susan::GraphML::AttributeType::TYPE_STRING, openmodelica_susan::GraphML::AttributeTarget::TARGET_GRAPH, graphInfo.clone())?;
            graphInfo = __pa30.clone();
            critPathAttIdx = __pa31.clone();
            let (__pa32, (_, __pa33)) = GraphML::addAttribute((literal!("false")).clone(), (literal!("isPartOfZeroFuncSystem")).clone(), openmodelica_susan::GraphML::AttributeType::TYPE_BOOLEAN, openmodelica_susan::GraphML::AttributeTarget::TARGET_NODE, graphInfo.clone())?;
            graphInfo = __pa32.clone();
            partOfEventAttIdx = __pa33.clone();
            let (__pa34, (_, __pa35)) = GraphML::addAttribute((literal!("false")).clone(), (literal!("IsPartOfOdeSystem")).clone(), openmodelica_susan::GraphML::AttributeType::TYPE_BOOLEAN, openmodelica_susan::GraphML::AttributeTarget::TARGET_NODE, graphInfo.clone())?;
            graphInfo = __pa34.clone();
            partOfOdeAttIdx = __pa35.clone();
            let (__pa36, (_, __pa37)) = GraphML::addAttribute((literal!("false")).clone(), (literal!("IsRemovedComponent")).clone(), openmodelica_susan::GraphML::AttributeType::TYPE_BOOLEAN, openmodelica_susan::GraphML::AttributeTarget::TARGET_NODE, graphInfo.clone())?;
            graphInfo = __pa36.clone();
            removedCompAttIdx = __pa37.clone();
            graphInfo = GraphML::addGraphAttributeValue((critPathAttIdx.clone(), iCriticalPathInfo.clone()), iGraphIdx.clone(), graphInfo.clone())?;
            nodeIdc = List::intRange((iGraph.clone().borrow().len() as i32));
            (graphInfo, _) = List::fold(nodeIdc.clone(), Arc::new({ let __pe_b1 = (iGraph.clone(), iGraphData.clone()); let __pe_b2 = (nameAttIdx.clone(), opCountAttIdx.clone(), calcTimeAttIdx.clone(), taskIdAttIdx.clone(), compsIdAttIdx.clone(), yCoordAttIdx.clone(), commCostAttIdx.clone(), commVarsAttIdx.clone(), commVarsIntAttIdx.clone(), commVarsFloatAttIdx.clone(), commVarsBoolAttIdx.clone(), simCodeEqAttIdx.clone(), threadIdAttIdx.clone(), taskNumberAttIdx.clone(), annotAttIdx.clone(), partOfEventAttIdx.clone(), partOfOdeAttIdx.clone(), removedCompAttIdx.clone()); let __pe_b3 = iSccSimEqMapping.clone(); let __pe_b4 = (iCriticalPath.clone(), iCriticalPathWoC.clone(), iSchedulerInfo.clone(), iAnnotationInfo.clone()); let __pe_b5 = iGraphDumpOptions.clone(); move |__pe_a0, __pe_a6| addNodeToGraphML(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_a6) }), (graphInfo.clone(), iGraphIdx.clone()));
            graphInfo.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oGraphInfo)
}

fn addNodeToGraphML(mut nodeIdx: i32, mut tGraphDataTuple: (metamodelica::Array<Arc<metamodelica::List<i32>>>, TaskGraphMeta), mut attIdc: (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32), mut sccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSchedulerInfoCritPath: (Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, metamodelica::Array<(i32, i32, metamodelica::Real)>, metamodelica::Array<ArcStr>), mut iGraphDumpOptions: GraphDumpOptions, mut iGraph: (GraphML::GraphInfo, i32)) -> Result<(GraphML::GraphInfo, i32)> {
    let mut oGraph: (GraphML::GraphInfo, i32);
    let mut tGraphIn: TaskGraph;
    let mut tGraphDataIn: TaskGraphMeta;
    let mut tmpGraph: GraphML::GraphInfo;
    let mut graphIdx: i32 = 0;
    let mut opCount: i32 = 0;
    let mut nameAttIdx: i32 = 0;
    let mut calcTimeAttIdx: i32 = 0;
    let mut opCountAttIdx: i32 = 0;
    let mut taskIdAttIdx: i32 = 0;
    let mut compsIdAttIdx: i32 = 0;
    let mut yCoordAttIdx: i32 = 0;
    let mut commCostAttIdx: i32 = 0;
    let mut commVarsAttIdx: i32 = 0;
    let mut commVarsAttIntIdx: i32 = 0;
    let mut commVarsAttFloatIdx: i32 = 0;
    let mut commVarsAttBoolIdx: i32 = 0;
    let mut yCoord: i32 = 0;
    let mut simCodeEqAttIdx: i32 = 0;
    let mut threadIdAttIdx: i32 = 0;
    let mut taskNumberAttIdx: i32 = 0;
    let mut annotationAttIdx: i32 = 0;
    let mut partOfEventAttIdx: i32 = 0;
    let mut partOfOdeAttIdx: i32 = 0;
    let mut removedCompAttIdx: i32 = 0;
    let mut calcTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut taskFinishTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut taskStartTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut primalComp: i32 = 0;
    let mut childNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut components: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut simCodeEqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut annotationInfo: metamodelica::Array<ArcStr>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut calcTimeString: ArcStr = arcstr::literal!("");
    let mut opCountString: ArcStr = arcstr::literal!("");
    let mut yCoordString: ArcStr = arcstr::literal!("");
    let mut taskFinishTimeString: ArcStr = arcstr::literal!("");
    let mut taskStartTimeString: ArcStr = arcstr::literal!("");
    let mut compText: ArcStr = arcstr::literal!("");
    let mut compsText: ArcStr = arcstr::literal!("");
    let mut description: ArcStr = arcstr::literal!("");
    let mut nodeDesc: ArcStr = arcstr::literal!("");
    let mut componentsString: ArcStr = arcstr::literal!("");
    let mut simCodeEqString: ArcStr = arcstr::literal!("");
    let mut threadIdxString: ArcStr = arcstr::literal!("");
    let mut taskNumberString: ArcStr = arcstr::literal!("");
    let mut annotationString: ArcStr = arcstr::literal!("");
    let mut schedulerThreadId: i32 = 0;
    let mut schedulerTaskNumber: i32 = 0;
    let mut nodeLabels: Arc<metamodelica::List<GraphML::NodeLabel>> = metamodelica::nil();
    let mut schedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>;
    let mut criticalPath: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut criticalPathWoC: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut visualizeTaskStartAndFinishTime: bool = false;
    let mut visualizeTaskCalcTime: bool = false;
    let mut isPartOfODESystem: bool = false;
    let mut isPartOfZeroFuncSystem: bool = false;
    let mut isRemovedComponent: bool = false;
    let mut compInformations: metamodelica::Array<ComponentInfo>;
    (tmpGraph, graphIdx) = iGraph.clone();
    if intGt(nodeIdx.clone(), 0) {
        (tGraphIn, tGraphDataIn) = tGraphDataTuple.clone();
        let TaskGraphMeta { compInformations: __pa0, nodeMark: __pa1, exeCosts: __pa2, compDescs: __pa3, compNames: __pa4, inComps: __pa5, .. } = (tGraphDataIn.clone()) else { bail!("pattern mismatch") };
        compInformations = __pa0.clone();
        nodeMark = __pa1.clone();
        exeCosts = __pa2.clone();
        compDescs = __pa3.clone();
        compNames = __pa4.clone();
        inComps = __pa5.clone();
        (nameAttIdx, opCountAttIdx, calcTimeAttIdx, taskIdAttIdx, compsIdAttIdx, yCoordAttIdx, commCostAttIdx, commVarsAttIdx, commVarsAttIntIdx, commVarsAttFloatIdx, commVarsAttBoolIdx, simCodeEqAttIdx, threadIdAttIdx, taskNumberAttIdx, annotationAttIdx, partOfEventAttIdx, partOfOdeAttIdx, removedCompAttIdx) = attIdc.clone();
        (criticalPath, criticalPathWoC, schedulerInfo, annotationInfo) = iSchedulerInfoCritPath.clone();
        let GraphDumpOptions { visualizeTaskCalcTime: __pa6, visualizeTaskStartAndFinishTime: __pa7, .. } = (iGraphDumpOptions.clone()) else { bail!("pattern mismatch") };
        visualizeTaskCalcTime = __pa6.clone();
        visualizeTaskStartAndFinishTime = __pa7.clone();
        components = inComps.clone().borrow()[(nodeIdx.clone()-1) as usize].clone();
        (isPartOfODESystem, isPartOfZeroFuncSystem, isRemovedComponent) = getNodeMembershipByComponents(components.clone(), compInformations.clone())?;
        if intNe((components.clone().len() as i32), 1) {
            primalComp = List::last(components.clone())?;
            simCodeEqs = List::flatten(List::map1(components.clone(), std::sync::Arc::new(fnptr!(Array::getIndexFirst, i32, _)), sccSimEqMapping.clone()));
            nodeDesc = stringDelimitList(List::map1(components.clone(), std::sync::Arc::new(fnptr!(Array::getIndexFirst, i32, _)), compDescs.clone()), (literal!("\n")).clone());
            (opCount, calcTime) = List::fold1(components.clone(), (std::sync::Arc::new(addNodeToGraphML1) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<(i32, metamodelica::Real)>, (i32, metamodelica::Real)) -> Result<(i32, metamodelica::Real)> + 'static>), exeCosts.clone(), (0, metamodelica::OrderedFloat(0.0_f64)));
        } else {
            primalComp = (components.clone()).get(1)?;
            simCodeEqs = sccSimEqMapping.clone().borrow()[(primalComp.clone()-1) as usize].clone();
            nodeDesc = (compDescs.clone().borrow()[(primalComp.clone()-1) as usize].clone()).clone();
            (_, calcTime) = exeCosts.clone().borrow()[(primalComp.clone()-1) as usize].clone();
            (opCount, calcTime) = exeCosts.clone().borrow()[(primalComp.clone()-1) as usize].clone();
        }
        compText = (compNames.clone().borrow()[(primalComp.clone()-1) as usize].clone()).clone();
        compsText = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*stringDelimitList(List::map(components.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
        annotationString = (annotationInfo.clone().borrow()[(nodeIdx.clone()-1) as usize].clone()).clone();
        calcTimeString = (realString(calcTime.clone())).clone();
        yCoord = nodeMark.clone().borrow()[(nodeIdx.clone()-1) as usize].clone() * 100;
        opCountString = (intString(opCount.clone())).clone();
        yCoordString = (intString(yCoord.clone())).clone();
        childNodes = tGraphIn.clone().borrow()[(nodeIdx.clone()-1) as usize].clone();
        simCodeEqString = stringDelimitList(List::map(simCodeEqs.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(", ")).clone());
        componentsString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*intString(nodeIdx.clone())); __mm_s.push_str(&*literal!(" ")); ArcStr::from(__mm_s) }).clone();
        (schedulerThreadId, schedulerTaskNumber, taskFinishTime) = schedulerInfo.clone().borrow()[(nodeIdx.clone()-1) as usize].clone();
        taskStartTime = (taskFinishTime.clone()) - (calcTime.clone());
        threadIdxString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Th ")); __mm_s.push_str(&*intString(schedulerThreadId.clone())); ArcStr::from(__mm_s) }).clone();
        taskNumberString = (intString(schedulerTaskNumber.clone())).clone();
        calcTimeString = (System::snprintff((literal!("%.0f")).clone(), 25, calcTime.clone())?).clone();
        taskFinishTimeString = (System::snprintff((literal!("%.0f")).clone(), 25, taskFinishTime.clone())?).clone();
        taskStartTimeString = (System::snprintff((literal!("%.0f")).clone(), 25, taskStartTime.clone())?).clone();
        nodeLabels = list![GraphML::NodeLabel::NODELABEL_INTERNAL { text: (componentsString.clone()).clone(), backgroundColor: None, fontStyle: openmodelica_susan::GraphML::FontStyle::FONTPLAIN }];
        nodeLabels = if (visualizeTaskCalcTime.clone()) {cons(GraphML::NodeLabel::NODELABEL_CORNER { text: (calcTimeString.clone()).clone(), backgroundColor: Some((arcstr::literal!(GraphML::COLOR_YELLOW)).clone()), fontStyle: openmodelica_susan::GraphML::FontStyle::FONTBOLD, position: (literal!("se")).clone() }, nodeLabels.clone())} else {nodeLabels.clone()};
        nodeLabels = if (visualizeTaskStartAndFinishTime.clone()) {listAppend(nodeLabels.clone(), list![GraphML::NodeLabel::NODELABEL_CORNER { text: (taskStartTimeString.clone()).clone(), backgroundColor: Some((arcstr::literal!(GraphML::COLOR_CYAN)).clone()), fontStyle: openmodelica_susan::GraphML::FontStyle::FONTBOLD, position: (literal!("nw")).clone() }, GraphML::NodeLabel::NODELABEL_CORNER { text: (taskFinishTimeString.clone()).clone(), backgroundColor: Some((arcstr::literal!(GraphML::COLOR_PINK)).clone()), fontStyle: openmodelica_susan::GraphML::FontStyle::FONTBOLD, position: (literal!("sw")).clone() }])} else {nodeLabels.clone()};
        let (__pa8, (_, _)) = GraphML::addNode(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Node")); __mm_s.push_str(&*intString(nodeIdx.clone())); ArcStr::from(__mm_s) }).clone(), (arcstr::literal!(GraphML::COLOR_ORANGE)).clone(), GraphML::BORDERWIDTH_STANDARD.clone(), nodeLabels.clone(), openmodelica_susan::GraphML::ShapeType::RECTANGLE, Some((nodeDesc.clone()).clone()), list![(nameAttIdx.clone(), compText.clone()), (calcTimeAttIdx.clone(), calcTimeString.clone()), (opCountAttIdx.clone(), opCountString.clone()), (taskIdAttIdx.clone(), componentsString.clone()), (compsIdAttIdx.clone(), compsText.clone()), (yCoordAttIdx.clone(), yCoordString.clone()), (simCodeEqAttIdx.clone(), simCodeEqString.clone()), (threadIdAttIdx.clone(), threadIdxString.clone()), (taskNumberAttIdx.clone(), taskNumberString.clone()), (annotationAttIdx.clone(), annotationString.clone()), (partOfEventAttIdx.clone(), boolString(isPartOfODESystem.clone())), (partOfOdeAttIdx.clone(), boolString(isPartOfZeroFuncSystem.clone())), (removedCompAttIdx.clone(), boolString(isRemovedComponent.clone()))], graphIdx.clone(), tmpGraph.clone())?;
        tmpGraph = __pa8.clone();
        tmpGraph = List::fold(childNodes.clone(), Arc::new({ let __pe_b1 = nodeIdx.clone(); let __pe_b2 = tGraphDataIn.clone(); let __pe_b3 = (commCostAttIdx.clone(), commVarsAttIdx.clone(), commVarsAttIntIdx.clone(), commVarsAttFloatIdx.clone(), commVarsAttBoolIdx.clone()); let __pe_b4 = (criticalPath.clone(), criticalPathWoC.clone()); let __pe_b5 = iGraphDumpOptions.clone(); move |__pe_a0, __pe_a6| addDepToGraph(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_a6) }), tmpGraph.clone());
    } else {
        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("function addNodeToGraphML failed.")).clone()])?;
    }
    oGraph = (tmpGraph.clone(), graphIdx.clone());
    Ok(oGraph)
}

fn addNodeToGraphML1(mut compIdx: i32, mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>, mut exeCostsIn: (i32, metamodelica::Real)) -> Result<(i32, metamodelica::Real)> {
    let mut exeCostsOut: (i32, metamodelica::Real);
    let mut opCount: i32 = 0;
    let mut opCountIn: i32 = 0;
    let mut exeTimeIn: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut exeTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    (opCountIn, exeTimeIn) = exeCostsIn.clone();
    (opCount, exeTime) = exeCosts.clone().borrow()[(compIdx.clone()-1) as usize].clone();
    exeCostsOut = (opCountIn.clone() + opCount.clone(), (exeTimeIn.clone()) + (exeTime.clone()));
    Ok(exeCostsOut)
}

fn addDepToGraph(mut childIdx: i32, mut parentIdx: i32, mut tGraphDataIn: TaskGraphMeta, mut iCommAttIdc: (i32, i32, i32, i32, i32), mut iCriticalPathEdges: (Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), mut iGraphDumpOptions: GraphDumpOptions, mut iGraph: GraphML::GraphInfo) -> Result<GraphML::GraphInfo> {
    let mut oGraph: GraphML::GraphInfo;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut integerVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut floatVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut booleanVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut commCostAttIdx: i32 = 0;
    let mut commVarsAttIdx: i32 = 0;
    let mut commVarsAttIntIdx: i32 = 0;
    let mut commVarsAttFloatIdx: i32 = 0;
    let mut commVarsAttBoolIdx: i32 = 0;
    let mut numOfCommVars: i32 = 0;
    let mut numOfCommVarsInt: i32 = 0;
    let mut numOfCommVarsFloat: i32 = 0;
    let mut numOfCommVarsBool: i32 = 0;
    let mut primalCompParent: i32 = 0;
    let mut primalCompChild: i32 = 0;
    let mut commCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut refSccCountStr: ArcStr = arcstr::literal!("");
    let mut commCostString: ArcStr = arcstr::literal!("");
    let mut numOfCommVarsString: ArcStr = arcstr::literal!("");
    let mut numOfCommVarsIntString: ArcStr = arcstr::literal!("");
    let mut numOfCommVarsFloatString: ArcStr = arcstr::literal!("");
    let mut numOfCommVarsBoolString: ArcStr = arcstr::literal!("");
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut components: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpGraph: GraphML::GraphInfo;
    let mut criticalPathEdges: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut criticalPathEdgesWoC: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut edgeColor: ArcStr = arcstr::literal!(GraphML::COLOR_BLACK);
    let mut visualizeCriticalPath: bool = false;
    let mut visualizeCommTime: bool = false;
    let mut edgeLabels: Arc<metamodelica::List<GraphML::EdgeLabel>> = metamodelica::nil();
    let mut lineWidth: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let TaskGraphMeta { inComps: __pa0, nodeMark: __pa1, commCosts: __pa2, .. } = (tGraphDataIn.clone()) else { bail!("pattern mismatch") };
    inComps = __pa0.clone();
    nodeMark = __pa1.clone();
    commCosts = __pa2.clone();
    (commCostAttIdx, commVarsAttIdx, commVarsAttIntIdx, commVarsAttFloatIdx, commVarsAttBoolIdx) = iCommAttIdc.clone();
    (criticalPathEdges, criticalPathEdgesWoC) = iCriticalPathEdges.clone();
    let GraphDumpOptions { visualizeCommTime: __pa3, visualizeCriticalPath: __pa4, .. } = (iGraphDumpOptions.clone()) else { bail!("pattern mismatch") };
    visualizeCommTime = __pa3.clone();
    visualizeCriticalPath = __pa4.clone();
    if List::exist1(criticalPathEdges.clone(), (std::sync::Arc::new(fnptr!(compareIntTuple2, (i32, i32), (i32, i32))) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32), (i32, i32)) -> Result<bool> + 'static>), (parentIdx.clone(), childIdx.clone())) {
        lineWidth = GraphML::LINEWIDTH_BOLD.clone();
        edgeColor = (if (visualizeCriticalPath.clone()) {arcstr::literal!(GraphML::COLOR_GRAY)} else {edgeColor.clone()}).clone();
    } else {
        lineWidth = GraphML::LINEWIDTH_STANDARD.clone();
    }
    let Communication { requiredTime: __pa5, booleanVars: __pa6, floatVars: __pa7, integerVars: __pa8, numberOfVars: __pa9, .. } = (getCommCostBetweenNodes(parentIdx.clone(), childIdx.clone(), tGraphDataIn.clone())?) else { bail!("pattern mismatch") };
    commCost = __pa5.clone();
    booleanVars = __pa6.clone();
    floatVars = __pa7.clone();
    integerVars = __pa8.clone();
    numOfCommVars = __pa9.clone();
    numOfCommVarsString = (intString(numOfCommVars.clone())).clone();
    numOfCommVarsIntString = (intString((integerVars.clone().len() as i32))).clone();
    numOfCommVarsFloatString = (intString((floatVars.clone().len() as i32))).clone();
    numOfCommVarsBoolString = (intString((booleanVars.clone().len() as i32))).clone();
    commCostString = (System::snprintff((literal!("%.0f")).clone(), 25, commCost.clone())?).clone();
    edgeLabels = if (visualizeCommTime.clone()) {list![GraphML::EdgeLabel { text: (commCostString.clone()).clone(), backgroundColor: Some((edgeColor.clone()).clone()), fontSize: GraphML::FONTSIZE_STANDARD.clone() }]} else {metamodelica::nil()};
    let (__pa10, (_, _)) = GraphML::addEdge(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Edge")); __mm_s.push_str(&*intString(parentIdx.clone())); __mm_s.push_str(&*intString(childIdx.clone())); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Node")); __mm_s.push_str(&*intString(childIdx.clone())); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Node")); __mm_s.push_str(&*intString(parentIdx.clone())); ArcStr::from(__mm_s) }).clone(), (edgeColor.clone()).clone(), openmodelica_susan::GraphML::LineType::LINE, lineWidth.clone(), false, edgeLabels.clone(), (openmodelica_susan::GraphML::ArrowType::ARROWNONE, openmodelica_susan::GraphML::ArrowType::ARROWSTANDART), list![(commCostAttIdx.clone(), commCostString.clone()), (commVarsAttIdx.clone(), numOfCommVarsString.clone()), (commVarsAttIntIdx.clone(), numOfCommVarsIntString.clone()), (commVarsAttFloatIdx.clone(), numOfCommVarsFloatString.clone()), (commVarsAttBoolIdx.clone(), numOfCommVarsBoolString.clone())], iGraph.clone())?;
    tmpGraph = __pa10.clone();
    oGraph = tmpGraph.clone();
    Ok(oGraph)
}

fn getNodeMembershipByComponents(mut iNodeComponents: Arc<metamodelica::List<i32>>, mut iCompInformations: metamodelica::Array<ComponentInfo>) -> Result<(bool, bool, bool)> {
    let mut oMembership: (bool, bool, bool);
    let mut isPartOfODESystem: bool = false;
    let mut isPartOfZeroFuncSystem: bool = false;
    let mut isRemovedComponent: bool = false;
    let mut compIdx: i32 = 0;
    let mut tmpComponentInformation: ComponentInfo;
    tmpComponentInformation = ComponentInfo { isPartOfODESystem: false, isPartOfZeroFuncSystem: false, isRemovedComponent: false };
    for mut compIdx in &*iNodeComponents.clone() {
        let mut compIdx = compIdx.clone();
        tmpComponentInformation = combineComponentInformations(iCompInformations.clone().borrow()[(compIdx.clone()-1) as usize].clone(), tmpComponentInformation.clone())?;
    }
    let ComponentInfo { isPartOfODESystem: __pa0, isPartOfZeroFuncSystem: __pa1, isRemovedComponent: __pa2 } = (tmpComponentInformation.clone()) else { bail!("pattern mismatch") };
    isPartOfODESystem = __pa0.clone();
    isPartOfZeroFuncSystem = __pa1.clone();
    isRemovedComponent = __pa2.clone();
    oMembership = (isPartOfODESystem.clone(), isPartOfZeroFuncSystem.clone(), isRemovedComponent.clone());
    Ok(oMembership)
}

//-----------------
//  Print functions
//-----------------
pub fn printTaskGraph(mut graphIn: TaskGraph) -> Result<()> {
    let mut graphLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    println!("{}", (literal!("\n")).clone());
    println!("{}", (literal!("--------------------------------\n")).clone());
    println!("{}", (literal!("TASKGRAPH\n")).clone());
    println!("{}", (literal!("--------------------------------\n")).clone());
    graphLst = Arc::new(graphIn.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
    dumpAdjacencyLst(graphLst.clone(), 1)?;
    println!("{}", (literal!("\n")).clone());
    Ok(())
}

fn dumpAdjacencyLst(mut inIntegerLstLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut rowIndex: i32) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inIntegerLstLst.clone(), rowIndex.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: row, tail: rows }, _) => {
            println!("{}", (intString(rowIndex.clone())).clone());
            println!("{}", (literal!(":")).clone());
            dumpAdjacencyRow(row.clone());
            dumpAdjacencyLst(rows.clone(), rowIndex.clone() + 1)?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn dumpAdjacencyRow(mut inIntegerLst: Arc<metamodelica::List<i32>>) -> () {
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

pub fn printTaskGraphMeta(mut metaDataIn: TaskGraphMeta) -> Result<()> {
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut compParamMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut compInformations: metamodelica::Array<ComponentInfo>;
    let TaskGraphMeta { compInformations: __pa0, nodeMark: __pa1, commCosts: __pa2, exeCosts: __pa3, compDescs: __pa4, compNames: __pa5, compParamMapping: __pa6, eqCompMapping: __pa7, varCompMapping: __pa8, inComps: __pa9 } = (metaDataIn.clone()) else { bail!("pattern mismatch") };
    compInformations = __pa0.clone();
    nodeMark = __pa1.clone();
    commCosts = __pa2.clone();
    exeCosts = __pa3.clone();
    compDescs = __pa4.clone();
    compNames = __pa5.clone();
    compParamMapping = __pa6.clone();
    eqCompMapping = __pa7.clone();
    varCompMapping = __pa8.clone();
    inComps = __pa9.clone();
    println!("{}", (literal!("\n")).clone());
    println!("{}", (literal!("--------------------------------\n")).clone());
    println!("{}", (literal!("TASKGRAPH METADATA\n")).clone());
    println!("{}", (literal!("--------------------------------\n")).clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString((inComps.clone().borrow().len() as i32))); __mm_s.push_str(&*literal!(" nodes include components:\n")); ArcStr::from(__mm_s) }).clone());
    printInComps(inComps.clone())?;
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString((varCompMapping.clone().borrow().len() as i32))); __mm_s.push_str(&*literal!(" vars are solved in the nodes \n")); ArcStr::from(__mm_s) }).clone());
    printVarCompMapping(varCompMapping.clone())?;
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString((eqCompMapping.clone().borrow().len() as i32))); __mm_s.push_str(&*literal!(" equations are computed in the nodes \n")); ArcStr::from(__mm_s) }).clone());
    printEqCompMapping(eqCompMapping.clone())?;
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString((compParamMapping.clone().borrow().len() as i32))); __mm_s.push_str(&*literal!(" parameters are part of the components \n")); ArcStr::from(__mm_s) }).clone());
    printCompParamMapping(compParamMapping.clone())?;
    println!("{}", (literal!("the names of the components \n")).clone());
    printComponentNames(compNames.clone())?;
    println!("{}", (literal!("the description of the node\n")).clone());
    printCompDescs(compDescs.clone())?;
    println!("{}", (literal!("the execution costs of the nodes\n")).clone());
    printExeCosts(exeCosts.clone())?;
    println!("{}", (literal!("the communication costs of the nodes\n")).clone());
    printCommCosts(commCosts.clone())?;
    println!("{}", (literal!("the nodeMark of the nodes\n")).clone());
    printNodeMarks(nodeMark.clone())?;
    println!("{}", (literal!("the component informations are\n")).clone());
    printComponentInformations(compInformations.clone())?;
    println!("{}", (literal!("\n")).clone());
    Ok(())
}

fn printInComps(mut iInComps: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    let mut nodeIdx: i32 = 0;
    let mut compRow: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let __range0 = 1..=(iInComps.clone().borrow().len() as i32);
    for mut nodeIdx in __range0 {
        compRow = iInComps.clone().borrow()[(nodeIdx.clone()-1) as usize].clone();
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("node ")); __mm_s.push_str(&*intString(nodeIdx.clone())); __mm_s.push_str(&*literal!(" solves components: ")); __mm_s.push_str(&*stringDelimitList(List::map(compRow.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    println!("{}", (literal!("--------------------------------\n")).clone());
    Ok(())
}

fn printVarCompMapping(mut iVarCompMapping: metamodelica::Array<(i32, i32, i32)>) -> Result<()> {
    let mut varIdx: i32 = 0;
    let mut comp: i32 = 0;
    let mut eqSysIdx: i32 = 0;
    let mut varOffset: i32 = 0;
    let __range0 = 1..=(iVarCompMapping.clone().borrow().len() as i32);
    for mut varIdx in __range0 {
        (comp, eqSysIdx, varOffset) = iVarCompMapping.clone().borrow()[(varIdx.clone()-1) as usize].clone();
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("variable ")); __mm_s.push_str(&*intString(varIdx.clone() - varOffset.clone())); __mm_s.push_str(&*literal!(" (offset: ")); __mm_s.push_str(&*intString(varOffset.clone())); __mm_s.push_str(&*literal!(") of equation system ")); __mm_s.push_str(&*intString(eqSysIdx.clone())); __mm_s.push_str(&*literal!(" is solved in component: ")); __mm_s.push_str(&*intString(comp.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    println!("{}", (literal!("--------------------------------\n")).clone());
    Ok(())
}

fn printEqCompMapping(mut iEqCompMapping: metamodelica::Array<(i32, i32, i32)>) -> Result<()> {
    let mut eqIdx: i32 = 0;
    let mut comp: i32 = 0;
    let mut eqSysIdx: i32 = 0;
    let mut eqOffset: i32 = 0;
    let __range0 = 1..=(iEqCompMapping.clone().borrow().len() as i32);
    for mut eqIdx in __range0 {
        (comp, eqSysIdx, eqOffset) = iEqCompMapping.clone().borrow()[(eqIdx.clone()-1) as usize].clone();
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("equation ")); __mm_s.push_str(&*intString(eqIdx.clone())); __mm_s.push_str(&*literal!(" (offset: ")); __mm_s.push_str(&*intString(eqOffset.clone())); __mm_s.push_str(&*literal!(") of equation system ")); __mm_s.push_str(&*intString(eqSysIdx.clone())); __mm_s.push_str(&*literal!(" is computed in component: ")); __mm_s.push_str(&*intString(comp.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    println!("{}", (literal!("--------------------------------\n")).clone());
    Ok(())
}

fn printCompParamMapping(mut iCompParamMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    let mut compIdx: i32 = 0;
    let mut params: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let __range0 = 1..=(iCompParamMapping.clone().borrow().len() as i32);
    for mut compIdx in __range0 {
        params = iCompParamMapping.clone().borrow()[(compIdx.clone()-1) as usize].clone();
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("component ")); __mm_s.push_str(&*intString(compIdx.clone())); __mm_s.push_str(&*literal!(" needs the parameters: ")); __mm_s.push_str(&*stringDelimitList(List::map(params.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    println!("{}", (literal!("--------------------------------\n")).clone());
    Ok(())
}

fn printComponentNames(mut iCompNames: metamodelica::Array<ArcStr>) -> Result<()> {
    let mut compIdx: i32 = 0;
    let mut compName: ArcStr = arcstr::literal!("");
    let __range0 = 1..=(iCompNames.clone().borrow().len() as i32);
    for mut compIdx in __range0 {
        compName = (iCompNames.clone().borrow()[(compIdx.clone()-1) as usize].clone()).clone();
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("component ")); __mm_s.push_str(&*intString(compIdx.clone())); __mm_s.push_str(&*literal!(" is named ")); __mm_s.push_str(&*compName.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    println!("{}", (literal!("--------------------------------\n")).clone());
    Ok(())
}

fn printCompDescs(mut iCompDescs: metamodelica::Array<ArcStr>) -> Result<()> {
    let mut compIdx: i32 = 0;
    let mut compDesc: ArcStr = arcstr::literal!("");
    let __range0 = 1..=(iCompDescs.clone().borrow().len() as i32);
    for mut compIdx in __range0 {
        compDesc = (iCompDescs.clone().borrow()[(compIdx.clone()-1) as usize].clone()).clone();
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("component ")); __mm_s.push_str(&*intString(compIdx.clone())); __mm_s.push_str(&*literal!(" is described with: ")); __mm_s.push_str(&*compDesc.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    println!("{}", (literal!("--------------------------------\n")).clone());
    Ok(())
}

fn printExeCosts(mut iExeCosts: metamodelica::Array<(i32, metamodelica::Real)>) -> Result<()> {
    let mut compIdx: i32 = 0;
    let mut opCount: i32 = 0;
    let mut execTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let __range0 = 1..=(iExeCosts.clone().borrow().len() as i32);
    for mut compIdx in __range0 {
        (opCount, execTime) = iExeCosts.clone().borrow()[(compIdx.clone()-1) as usize].clone();
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("component ")); __mm_s.push_str(&*intString(compIdx.clone())); __mm_s.push_str(&*literal!(" has execution cost of: (")); __mm_s.push_str(&*intString(opCount.clone())); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*realString(execTime.clone())); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone());
    }
    println!("{}", (literal!("--------------------------------\n")).clone());
    Ok(())
}

fn printCommCosts(mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>) -> Result<()> {
    let mut nodeIdx: i32 = 0;
    let mut nodeComms: Communications = metamodelica::nil();
    let __range0 = 1..=(iCommCosts.clone().borrow().len() as i32);
    for mut nodeIdx in __range0 {
        nodeComms = iCommCosts.clone().borrow()[(nodeIdx.clone()-1) as usize].clone();
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("edges from node ")); __mm_s.push_str(&*intString(nodeIdx.clone())); __mm_s.push_str(&*literal!(": with the communication costs ")); __mm_s.push_str(&*stringDelimitList(List::map(nodeComms.clone(), (std::sync::Arc::new(printCommCost) as std::sync::Arc<dyn ::std::ops::Fn(Communication) -> Result<ArcStr> + 'static>)), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    println!("{}", (literal!("--------------------------------\n")).clone());
    Ok(())
}

fn printCommCost(mut iComm: Communication) -> Result<ArcStr> {
    let mut oCommString: ArcStr = arcstr::literal!("");
    let mut numberOfVars: i32 = 0;
    let mut numberOfIntegers: i32 = 0;
    let mut numberOfFloats: i32 = 0;
    let mut numberOfBooleans: i32 = 0;
    let mut childNode: i32 = 0;
    let mut integerVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut floatVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut booleanVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut requiredTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let Communication { requiredTime: __pa0, childNode: __pa1, booleanVars: __pa2, floatVars: __pa3, integerVars: __pa4, numberOfVars: __pa5, .. } = (iComm.clone()) else { bail!("pattern mismatch") };
    requiredTime = __pa0.clone();
    childNode = __pa1.clone();
    booleanVars = __pa2.clone();
    floatVars = __pa3.clone();
    integerVars = __pa4.clone();
    numberOfVars = __pa5.clone();
    numberOfIntegers = (integerVars.clone().len() as i32);
    numberOfFloats = (floatVars.clone().len() as i32);
    numberOfBooleans = (booleanVars.clone().len() as i32);
    oCommString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(target node: ")); __mm_s.push_str(&*intString(childNode.clone())); __mm_s.push_str(&*literal!(" ints: ")); __mm_s.push_str(&*intString(numberOfIntegers.clone())); __mm_s.push_str(&*literal!(" floats: ")); __mm_s.push_str(&*intString(numberOfFloats.clone())); __mm_s.push_str(&*literal!(" booleans: ")); __mm_s.push_str(&*intString(numberOfBooleans.clone())); __mm_s.push_str(&*literal!(" [requiredTime: ")); __mm_s.push_str(&*realString(requiredTime.clone())); __mm_s.push_str(&*literal!(" for ")); __mm_s.push_str(&*intString(numberOfVars.clone())); __mm_s.push_str(&*literal!(" variables)")); ArcStr::from(__mm_s) }).clone();
    Ok(oCommString)
}

fn printNodeMarks(mut iNodeMarks: metamodelica::Array<i32>) -> Result<()> {
    let mut compIdx: i32 = 0;
    let mut mark: i32 = 0;
    let __range0 = 1..=(iNodeMarks.clone().borrow().len() as i32);
    for mut compIdx in __range0 {
        mark = iNodeMarks.clone().borrow()[(compIdx.clone()-1) as usize].clone();
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("component ")); __mm_s.push_str(&*intString(compIdx.clone())); __mm_s.push_str(&*literal!(" has the nodeMark : ")); __mm_s.push_str(&*intString(mark.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    println!("{}", (literal!("--------------------------------\n")).clone());
    Ok(())
}

fn printComponentInformations(mut iComponentInformations: metamodelica::Array<ComponentInfo>) -> Result<()> {
    let mut compIdx: i32 = 0;
    let mut isPartOfODESystem: bool = false;
    let mut isPartOfZeroFuncSystem: bool = false;
    let mut isRemovedComponent: bool = false;
    let __range0 = 1..=(iComponentInformations.clone().borrow().len() as i32);
    for mut compIdx in __range0 {
        let ComponentInfo { isRemovedComponent: __pa1, isPartOfZeroFuncSystem: __pa2, isPartOfODESystem: __pa3 } = (iComponentInformations.clone().borrow()[(compIdx.clone()-1) as usize].clone()) else { bail!("pattern mismatch") };
        isRemovedComponent = __pa1.clone();
        isPartOfZeroFuncSystem = __pa2.clone();
        isPartOfODESystem = __pa3.clone();
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("component ")); __mm_s.push_str(&*intString(compIdx.clone())); __mm_s.push_str(&*literal!(" has component information:\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("   Is part of ODE-System:   ")); __mm_s.push_str(&*boolString(isPartOfODESystem.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("   Is part of Event-System: ")); __mm_s.push_str(&*boolString(isPartOfZeroFuncSystem.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("   Is removed component:    ")); __mm_s.push_str(&*boolString(isRemovedComponent.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    println!("{}", (literal!("--------------------------------\n")).clone());
    Ok(())
}

pub fn intLstString(mut lstIn: Arc<metamodelica::List<i32>>) -> ArcStr {
    let mut strOut: ArcStr = arcstr::literal!("");
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = stringDelimitList(List::map(lstIn.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone());
    strOut = (if (lstIn.clone().is_empty()) {literal!("---")} else {r#str.clone()}).clone();
    strOut
}

pub fn dumpCriticalPathInfo(mut iCriticalPaths: (Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, metamodelica::Real), mut iCriticalPathsWoC: (Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, metamodelica::Real)) -> Result<ArcStr> {
    let mut oString: ArcStr = arcstr::literal!("");
    let mut tmpString: ArcStr = arcstr::literal!("");
    let mut critPath: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut critPathWoC: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut costPath: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut costPathWoC: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    oString = ('mc: {
        let __mc_input = (iCriticalPaths.clone(), iCriticalPathsWoC.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ metamodelica::List::Nil, _), _) => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((critPath, costPath), (critPathWoC, costPathWoC)) => {
                    let mut tmpString: ArcStr = tmpString.clone();
                    tmpString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("critical path with costs of ")); __mm_s.push_str(&*realString(costPath.clone())); __mm_s.push_str(&*literal!(" cycles -- ")); ArcStr::from(__mm_s) }).clone();
                    tmpString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmpString.clone()); __mm_s.push_str(&*dumpCriticalPathInfo1(critPath.clone(), 1)); ArcStr::from(__mm_s) }).clone();
                    tmpString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" ;; ")); __mm_s.push_str(&*tmpString.clone()); __mm_s.push_str(&*literal!("critical path' with costs of ")); __mm_s.push_str(&*realString(costPathWoC.clone())); __mm_s.push_str(&*literal!(" cycles -- ")); ArcStr::from(__mm_s) }).clone();
                    tmpString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmpString.clone()); __mm_s.push_str(&*dumpCriticalPathInfo1(critPathWoC.clone(), 1)); ArcStr::from(__mm_s) }).clone();
                    Ok(tmpString.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(oString)
}

fn dumpCriticalPathInfo1(mut criticalPathsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut cpIdx: i32) -> ArcStr {
    let mut oString: ArcStr = arcstr::literal!("");
    oString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intLstString((criticalPathsIn.clone()).get(cpIdx.clone()).unwrap())); __mm_s.push_str(&*literal!("")); ArcStr::from(__mm_s) }).clone();
    oString
}

fn printCriticalPathInfo(mut criticalPathsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut cpCosts: metamodelica::Real) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (criticalPathsIn.clone(), cpCosts.clone());
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
                _ => {
                    println!("{}", (literal!("--------------------------------\n")).clone());
                    println!("{}", (literal!(" CRITICAL PATH INFO\n")).clone());
                    println!("{}", (literal!("--------------------------------\n")).clone());
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("found ")); __mm_s.push_str(&*intString((criticalPathsIn.clone().len() as i32))); __mm_s.push_str(&*literal!(" critical paths with costs of ")); __mm_s.push_str(&*realString(cpCosts.clone())); __mm_s.push_str(&*literal!(" sec\n")); ArcStr::from(__mm_s) }).clone());
                    printCriticalPathInfo1(criticalPathsIn.clone(), 1);
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn printCriticalPathInfo1(mut criticalPathsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut cpIdx: i32) -> () {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(cpIdx.clone())); __mm_s.push_str(&*literal!(". path: ")); __mm_s.push_str(&*intLstString((criticalPathsIn.clone()).get(cpIdx.clone()).unwrap())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    ()
}

//--------------------------
//  Functions to merge nodes
//--------------------------
fn mergeSingleNodes(mut iTaskGraph: TaskGraph, mut iTaskGraphMeta: TaskGraphMeta, mut doNotMergeIn: Arc<metamodelica::List<i32>>) -> Result<(TaskGraph, TaskGraphMeta, bool)> {
    let mut oTaskGraph: TaskGraph;
    let mut oTaskGraphMeta: TaskGraphMeta;
    let mut changed: bool = false;
    (oTaskGraph, oTaskGraphMeta, changed) = 'mc: {
        let __mc_input = (iTaskGraph.clone(), iTaskGraphMeta.clone(), doNotMergeIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut numProc: i32 = 0;
                    let mut singleNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut singleNodes1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut pos: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut exeCosts: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
                    let mut taskGraphT: TaskGraph;
                    let mut changed: bool = changed.clone();
                    numProc = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
                    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), (iTaskGraph.clone().borrow().len() as i32))?;
                    (_, singleNodes) = List::filterOnTrueSync(Arc::new(iTaskGraph.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), std::sync::Arc::new(fnptr!(listEmpty, _)), List::intRange((iTaskGraph.clone().borrow().len() as i32)))?;
                    (_, singleNodes1) = List::filterOnTrueSync(Arc::new(taskGraphT.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), std::sync::Arc::new(fnptr!(listEmpty, _)), List::intRange((taskGraphT.clone().borrow().len() as i32)))?;
                    (singleNodes, _, _) = List::intersection1OnTrue(singleNodes.clone(), singleNodes1.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    (_, singleNodes, _) = List::intersection1OnTrue(singleNodes.clone(), doNotMergeIn.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    exeCosts = List::map1(singleNodes.clone(), (std::sync::Arc::new(getExeCostReqCycles) as std::sync::Arc<dyn ::std::ops::Fn(i32, TaskGraphMeta) -> Result<metamodelica::Real> + 'static>), iTaskGraphMeta.clone());
                    (exeCosts, pos) = HpcOmScheduler::quicksortWithOrder(exeCosts.clone())?;
                    singleNodes = List::map1(pos.clone(), std::sync::Arc::new(fnptr!(List::getIndexFirst, i32, _)), singleNodes.clone());
                    singleNodes = singleNodes.clone().reverse();
                    exeCosts = exeCosts.clone().reverse();
                    (_, _) = distributeToClusters(singleNodes.clone(), exeCosts.clone(), numProc.clone())?;
                    changed = intGt((singleNodes.clone().len() as i32), numProc.clone());
                    Ok((iTaskGraph.clone(), iTaskGraphMeta.clone(), changed.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((iTaskGraph.clone(), iTaskGraphMeta.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((oTaskGraph, oTaskGraphMeta, changed))
}

pub fn distributeToClusters(mut items: Arc<metamodelica::List<i32>>, mut values: Arc<metamodelica::List<metamodelica::Real>>, mut numClusters: i32) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<metamodelica::Real>)> {
    let mut clustersOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut clusterValuesOut: metamodelica::Array<metamodelica::Real>;
    let mut b: bool = false;
    let mut itemArr: metamodelica::Array<i32>;
    let mut itemsCopy: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut clusters: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut clusterValues: metamodelica::Array<metamodelica::Real>;
    b = intGt((items.clone().len() as i32), numClusters.clone());
    clusters = metamodelica::arrayFromVec(List::map(List::intRange((items.clone().len() as i32)), std::sync::Arc::new(fnptr!(List::create, _))).into_iter().cloned().collect());
    clusterValues = metamodelica::arrayFromVec(values.clone().into_iter().cloned().collect());
    itemArr = metamodelica::arrayFromVec(items.clone().into_iter().cloned().collect());
    itemsCopy = Array::map(itemArr.clone(), std::sync::Arc::new(fnptr!(List::create, _)));
    clusters = if (true) {Array::copy(itemsCopy.clone(), clusters.clone())?} else {clusters.clone()};
    clusterValues = if (!(b.clone())) {Array::copy(metamodelica::arrayFromVec(values.clone().into_iter().cloned().collect()), clusterValues.clone())?} else {clusterValues.clone()};
    if b.clone() {
        (clustersOut, clusterValuesOut) = distributeToClusters1((items.clone(), values.clone()), (clusters.clone(), clusterValues.clone()), numClusters.clone())?;
    } else {
        (clustersOut, clusterValuesOut) = (clusters.clone(), clusterValues.clone());
    }
    Ok((clustersOut, clusterValuesOut))
}

fn distributeToClusters1(mut tplIn: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<metamodelica::Real>>), mut tplFold: (metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<metamodelica::Real>), mut numClusters: i32) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<metamodelica::Real>)> {
    let mut clustersOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut clusterValuesOut: metamodelica::Array<metamodelica::Real>;
    (clustersOut, clusterValuesOut) = 'mc: {
        let __mc_input = (tplIn.clone(), tplFold.clone(), numClusters.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((itemsIn, _), (clusters, clusterValues), _) => {
                    let mut idcsLst1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut clustersFinal: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut clusterValuesFinal: metamodelica::Array<metamodelica::Real>;
                    let true = ((itemsIn.clone().len() as i32) <= numClusters.clone()) else { bail!("pattern mismatch") };
                    idcsLst1 = List::intRange(numClusters.clone());
                    clustersFinal = Array::select(clusters.clone(), idcsLst1.clone())?;
                    clusterValuesFinal = Array::select(clusterValues.clone(), idcsLst1.clone())?;
                    Ok((clustersFinal.clone(), clusterValuesFinal.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((itemsIn, valuesIn), (clusters, clusterValues), _) => {
                    let mut diff: i32 = 0;
                    let mut lst1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut idcsLst2: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut idcsLst1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut entries: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut entries2: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut values: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
                    let mut addValues: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
                    let mut clusters = (*clusters).clone();
                    let mut clusterValues = (*clusterValues).clone();
                    let true = ((itemsIn.clone().len() as i32) > numClusters.clone()) else { bail!("pattern mismatch") };
                    let true = ((itemsIn.clone().len() as i32) / 2 < numClusters.clone()) else { bail!("pattern mismatch") };
                    (lst1, _) = List::split(itemsIn.clone(), numClusters.clone())?;
                    diff = (itemsIn.clone().len() as i32) - numClusters.clone();
                    idcsLst1 = List::intRange2(numClusters.clone() - diff.clone() + 1, numClusters.clone());
                    idcsLst2 = List::intRange2(numClusters.clone() + 1, (itemsIn.clone().len() as i32));
                    entries = List::map1(idcsLst2.clone(), std::sync::Arc::new(fnptr!(Array::getIndexFirst, i32, _)), clusters.clone());
                    entries = entries.clone().reverse();
                    entries2 = List::map1(idcsLst1.clone(), std::sync::Arc::new(fnptr!(Array::getIndexFirst, i32, _)), clusters.clone());
                    entries = List::threadMap(entries.clone(), entries2.clone(), Arc::new(listAppend.clone()));
                    List::threadMap1_0(idcsLst1.clone(), entries.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), clusters.clone())?;
                    values = List::map1(idcsLst1.clone(), std::sync::Arc::new(fnptr!(Array::getIndexFirst, i32, _)), clusterValues.clone());
                    addValues = List::map1(idcsLst2.clone(), std::sync::Arc::new(fnptr!(Array::getIndexFirst, i32, _)), clusterValues.clone());
                    values = List::threadMap(values.clone(), addValues.clone(), (std::sync::Arc::new(fnptr!(realAdd, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<metamodelica::Real> + 'static>));
                    List::threadMap1_0(idcsLst1.clone(), values.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), clusterValues.clone())?;
                    (clusters, clusterValues) = distributeToClusters1((lst1.clone(), valuesIn.clone()), (clusters.clone(), clusterValues.clone()), numClusters.clone())?;
                    Ok((clusters.clone(), clusterValues.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((itemsIn, valuesIn), (clusters, clusterValues), _) => {
                    let mut numCl: i32 = 0;
                    let mut lst1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut idcsLst1_2: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut idcsLst2: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut entries: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut entries2: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut values: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
                    let mut addValues: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
                    let mut clusters = (*clusters).clone();
                    let mut clusterValues = (*clusterValues).clone();
                    let true = ((itemsIn.clone().len() as i32) > numClusters.clone()) else { bail!("pattern mismatch") };
                    let true = ((itemsIn.clone().len() as i32) / 2 >= numClusters.clone()) else { bail!("pattern mismatch") };
                    numCl = nextGreaterPowerOf2(intReal((itemsIn.clone().len() as i32)))?;
                    (lst1, _) = List::split(itemsIn.clone(), intDiv(numCl.clone(), 2))?;
                    idcsLst2 = List::intRange2(intDiv(numCl.clone(), 2) + 1, (itemsIn.clone().len() as i32));
                    idcsLst1_2 = List::intRange2(intDiv(numCl.clone(), 2) - (idcsLst2.clone().len() as i32) + 1, intDiv(numCl.clone(), 2));
                    entries = List::map1(idcsLst2.clone(), std::sync::Arc::new(fnptr!(Array::getIndexFirst, i32, _)), clusters.clone());
                    entries = entries.clone().reverse();
                    entries2 = List::map1(idcsLst1_2.clone(), std::sync::Arc::new(fnptr!(Array::getIndexFirst, i32, _)), clusters.clone());
                    entries = List::threadMap(entries.clone(), entries2.clone(), Arc::new(listAppend.clone()));
                    List::threadMap1_0(idcsLst1_2.clone(), entries.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), clusters.clone())?;
                    values = List::map1(idcsLst1_2.clone(), std::sync::Arc::new(fnptr!(Array::getIndexFirst, i32, _)), clusterValues.clone());
                    addValues = List::map1(idcsLst2.clone(), std::sync::Arc::new(fnptr!(Array::getIndexFirst, i32, _)), clusterValues.clone());
                    values = List::threadMap(values.clone(), addValues.clone(), (std::sync::Arc::new(fnptr!(realAdd, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<metamodelica::Real> + 'static>));
                    List::threadMap1_0(idcsLst1_2.clone(), values.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), clusterValues.clone())?;
                    (clusters, clusterValues) = distributeToClusters1((lst1.clone(), valuesIn.clone()), (clusters.clone(), clusterValues.clone()), numClusters.clone())?;
                    Ok((clusters.clone(), clusterValues.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("distributeToClusters failed!\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((clustersOut, clusterValuesOut))
}

fn nextGreaterPowerOf2(mut n: metamodelica::Real) -> Result<i32> {
    let mut powOf2: i32 = 0;
    powOf2 = nextGreaterPowerOf2_impl(n.clone(), 1)?;
    Ok(powOf2)
}

fn nextGreaterPowerOf2_impl(mut n: metamodelica::Real, mut pow: i32) -> Result<i32> {
    let mut powOf2: i32 = 0;
    powOf2 = 'mc: {
        let __mc_input = (n.clone(), pow.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _) = __mc_input.clone() else { bail!("nomatch") };
            let true = (n.clone() <= realPow(metamodelica::OrderedFloat(2.0_f64), intReal(pow.clone()))) else { bail!("pattern mismatch") };
            Ok(((realPow(metamodelica::OrderedFloat(2.0_f64), intReal(pow.clone()))).0 as i32))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut n2: i32 = 0;
            let true = (n.clone() > realPow(metamodelica::OrderedFloat(2.0_f64), intReal(pow.clone()))) else { bail!("pattern mismatch") };
            n2 = nextGreaterPowerOf2_impl(n.clone(), pow.clone() + 1)?;
            Ok(n2.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(powOf2)
}

pub fn mergeSimpleNodes(mut graphIn: TaskGraph, mut graphTIn: TaskGraph, mut graphDataIn: TaskGraphMeta, mut contractedTasksIn: metamodelica::Array<i32>) -> Result<(TaskGraph, TaskGraph, TaskGraphMeta, metamodelica::Array<i32>, bool)> {
    let mut graphOut: TaskGraph;
    let mut graphTOut: TaskGraph;
    let mut graphDataOut: TaskGraphMeta;
    let mut contractedTasksOut: metamodelica::Array<i32>;
    let mut changed: bool = false;
    let mut allNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut child: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut oneChildren: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    allNodes = List::intRange((graphIn.clone().borrow().len() as i32));
    oneChildren = findOneChildParents(allNodes.clone(), graphIn.clone(), metamodelica::nil(), list![metamodelica::nil()], 0, contractedTasksIn.clone())?;
    oneChildren = listDelete(oneChildren.clone(), (oneChildren.clone().len() as i32))?;
    oneChildren = List::removeOnTrue(1, (std::sync::Arc::new(compareListLengthOnTrue) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<i32>>) -> Result<bool> + 'static>), oneChildren.clone());
    (graphOut, graphTOut, graphDataOut, contractedTasksOut) = contractNodesInGraph(oneChildren.clone(), graphIn.clone(), graphTIn.clone(), graphDataIn.clone(), contractedTasksIn.clone())?;
    changed = !(oneChildren.clone().is_empty());
    Ok((graphOut, graphTOut, graphDataOut, contractedTasksOut, changed))
}

pub fn mergeParentNodes(mut graphIn: TaskGraph, mut graphTIn: TaskGraph, mut graphDataIn: TaskGraphMeta, mut contractedTasksIn: metamodelica::Array<i32>) -> Result<(TaskGraph, TaskGraph, TaskGraphMeta, metamodelica::Array<i32>, bool)> {
    let mut graphOut: TaskGraph;
    let mut graphTOut: TaskGraph;
    let mut graphDataOut: TaskGraphMeta;
    let mut contractedTasksOut: metamodelica::Array<i32>;
    let mut changed: bool = false;
    let mut alreadyMerged: metamodelica::Array<i32>;
    let mut mergedNodes: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    alreadyMerged = arrayCreate((graphIn.clone().borrow().len() as i32), 0);
    mergedNodes = mergeParentNodes0(graphIn.clone(), graphTIn.clone(), graphDataIn.clone(), contractedTasksIn.clone(), alreadyMerged.clone(), 1, metamodelica::nil())?;
    (graphOut, graphTOut, graphDataOut, contractedTasksOut) = contractNodesInGraph(mergedNodes.clone(), graphIn.clone(), graphTIn.clone(), graphDataIn.clone(), contractedTasksIn.clone())?;
    changed = !(mergedNodes.clone().is_empty());
    Ok((graphOut, graphTOut, graphDataOut, contractedTasksOut, changed))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn mergeParentNodes0(mut iGraph: TaskGraph, mut iGraphT: TaskGraph, mut iGraphData: TaskGraphMeta, mut contractedTasksIn: metamodelica::Array<i32>, mut alreadyMerged: metamodelica::Array<i32>, mut iNodeIdx: i32, mut iMergedNodes: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut oMergedNodes: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut tmpGraph: TaskGraph;
    let mut tmpGraphData: TaskGraphMeta;
    let mut tmpChanged: bool = false;
    let mut highestParentExeCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut sumParentExeCosts: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut parentNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut mergeNodeList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut highestCommCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut parentExeCosts: Arc<metamodelica::List<(i32, metamodelica::Real)>> = metamodelica::nil();
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut parentCommCosts: Communications = metamodelica::nil();
    let mut parentChilds: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut tmpMergedNodes: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    oMergedNodes = 'mc: {
        let __mc_input = (iGraph.clone(), iGraphT.clone(), iGraphData.clone(), contractedTasksIn.clone(), alreadyMerged.clone(), iNodeIdx.clone(), iMergedNodes.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, TaskGraphMeta { commCosts, exeCosts, .. }, _, _, _, _) => {
                    let mut mergeNodeList: Arc<metamodelica::List<i32>> = mergeNodeList.clone();
                    let mut parentExeCosts: Arc<metamodelica::List<(i32, metamodelica::Real)>> = parentExeCosts.clone();
                    let mut highestParentExeCost: metamodelica::Real = highestParentExeCost.clone();
                    let mut parentCommCosts: Arc<metamodelica::List<Communication>> = parentCommCosts.clone();
                    let mut highestCommCost: metamodelica::Real = highestCommCost.clone();
                    let mut parentChilds: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = parentChilds.clone();
                    let mut parentNodes: Arc<metamodelica::List<i32>> = parentNodes.clone();
                    let mut sumParentExeCosts: metamodelica::Real = sumParentExeCosts.clone();
                    let mut tmpMergedNodes: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = tmpMergedNodes.clone();
                    let true = (intLe(iNodeIdx.clone(), (iGraphT.clone().borrow().len() as i32))) else { bail!("pattern mismatch") };
                    let true = (intNe(contractedTasksIn.clone().borrow()[(iNodeIdx.clone()-1) as usize].clone(), -1)) else { bail!("pattern mismatch") };
                    let true = (intNe(alreadyMerged.clone().borrow()[(iNodeIdx.clone()-1) as usize].clone(), -1)) else { bail!("pattern mismatch") };
                    parentNodes = iGraphT.clone().borrow()[(iNodeIdx.clone()-1) as usize].clone();
                    parentNodes = filterContractedNodes(parentNodes.clone(), contractedTasksIn.clone());
                    let false = (List::exist1(parentNodes.clone(), (std::sync::Arc::new(isNodeContracted) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>), alreadyMerged.clone())) else { bail!("pattern mismatch") };
                    parentCommCosts = List::map2(parentNodes.clone(), (std::sync::Arc::new(getCommCostBetweenNodes) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, TaskGraphMeta) -> Result<Communication> + 'static>), iNodeIdx.clone(), iGraphData.clone());
                    let Communication { requiredTime: __pa0, .. } = (getHighestCommCost(parentCommCosts.clone(), Communication { numberOfVars: 0, integerVars: metamodelica::nil(), floatVars: metamodelica::nil(), booleanVars: metamodelica::nil(), stringVars: metamodelica::nil(), childNode: -1, requiredTime: metamodelica::OrderedFloat(-1.0_f64) })?) else { bail!("pattern mismatch") };
                    highestCommCost = __pa0.clone();
                    parentExeCosts = List::map1(parentNodes.clone(), (std::sync::Arc::new(getExeCost) as std::sync::Arc<dyn ::std::ops::Fn(i32, TaskGraphMeta) -> Result<(i32, metamodelica::Real)> + 'static>), iGraphData.clone());
                    (_, sumParentExeCosts) = List::fold(parentExeCosts.clone(), (std::sync::Arc::new(fnptr!(addUpExeCosts, (i32, metamodelica::Real), (i32, metamodelica::Real))) as std::sync::Arc<dyn ::std::ops::Fn((i32, metamodelica::Real), (i32, metamodelica::Real)) -> Result<(i32, metamodelica::Real)> + 'static>), (0, metamodelica::OrderedFloat(0.0_f64)));
                    (_, highestParentExeCost) = getHighestExecCost(parentExeCosts.clone(), (0, metamodelica::OrderedFloat(0.0_f64)))?;
                    let true = (realGt((highestCommCost.clone()) + (highestParentExeCost.clone()), sumParentExeCosts.clone())) else { bail!("pattern mismatch") };
                    parentChilds = List::map1(parentNodes.clone(), std::sync::Arc::new(fnptr!(Array::getIndexFirst, i32, _)), iGraph.clone());
                    let true = (List::removeOnTrue(1, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), List::map(parentChilds.clone(), std::sync::Arc::new(fnptr!(listLength, _)))).is_empty()) else { bail!("pattern mismatch") };
                    mergeNodeList = cons(iNodeIdx.clone(), parentNodes.clone());
                    tmpMergedNodes = cons(mergeNodeList.clone(), iMergedNodes.clone());
                    List::map_0(mergeNodeList.clone(), Arc::new({ let __pe_b1 = -1; let __pe_b2 = alreadyMerged.clone(); move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
                    tmpMergedNodes = mergeParentNodes0(iGraph.clone(), iGraphT.clone(), iGraphData.clone(), contractedTasksIn.clone(), alreadyMerged.clone(), iNodeIdx.clone() + 1, tmpMergedNodes.clone())?;
                    Ok(tmpMergedNodes.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _) => {
                    let mut tmpMergedNodes: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = tmpMergedNodes.clone();
                    let true = (intLe(iNodeIdx.clone(), (iGraphT.clone().borrow().len() as i32))) else { bail!("pattern mismatch") };
                    tmpMergedNodes = mergeParentNodes0(iGraph.clone(), iGraphT.clone(), iGraphData.clone(), contractedTasksIn.clone(), alreadyMerged.clone(), iNodeIdx.clone() + 1, iMergedNodes.clone())?;
                    Ok(tmpMergedNodes.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iMergedNodes.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oMergedNodes)
}

fn mergeSinkNodes(mut graphIn: TaskGraph, mut graphTIn: TaskGraph, mut graphDataIn: TaskGraphMeta, mut contractedTasksIn: metamodelica::Array<i32>) -> Result<(TaskGraph, TaskGraph, TaskGraphMeta, metamodelica::Array<i32>, bool)> {
    let mut graphOut: TaskGraph;
    let mut graphTOut: TaskGraph;
    let mut graphDataOut: TaskGraphMeta;
    let mut contractedTasksOut: metamodelica::Array<i32>;
    let mut changed: bool = false;
    let mut alreadyMerged: metamodelica::Array<i32>;
    let mut mergedNodes: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    alreadyMerged = arrayCreate((graphIn.clone().borrow().len() as i32), 0);
    mergedNodes = mergeParentNodes0(graphIn.clone(), graphTIn.clone(), graphDataIn.clone(), contractedTasksIn.clone(), alreadyMerged.clone(), 1, metamodelica::nil())?;
    (graphOut, graphTOut, graphDataOut, contractedTasksOut) = contractNodesInGraph(mergedNodes.clone(), graphIn.clone(), graphTIn.clone(), graphDataIn.clone(), contractedTasksIn.clone())?;
    changed = !(mergedNodes.clone().is_empty());
    Ok((graphOut, graphTOut, graphDataOut, contractedTasksOut, changed))
}

pub fn markSystemComponents(mut iTaskGraph: TaskGraph, mut iTaskGraphMeta: TaskGraphMeta, mut iComponentMarks: (bool, bool, bool), mut iTargetTaskGraphMeta: TaskGraphMeta) -> Result<TaskGraphMeta> {
    let mut oTargetTaskGraphMeta: TaskGraphMeta;
    let mut odeInComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut nodeComps: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut nodeIdx: i32 = 0;
    let mut compIdx: i32 = 0;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut compParamMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut compInformations: metamodelica::Array<ComponentInfo>;
    let mut componentInformation: ComponentInfo;
    let mut iComponentInformation: ComponentInfo;
    iComponentInformation = ComponentInfo { isPartOfODESystem: Util::tuple31(iComponentMarks.clone()), isPartOfZeroFuncSystem: Util::tuple32(iComponentMarks.clone()), isRemovedComponent: Util::tuple33(iComponentMarks.clone()) };
    let TaskGraphMeta { inComps: __pa0, .. } = (iTaskGraphMeta.clone()) else { bail!("pattern mismatch") };
    odeInComps = __pa0.clone();
    let TaskGraphMeta { inComps: __pa1, varCompMapping: __pa2, eqCompMapping: __pa3, compParamMapping: __pa4, compNames: __pa5, compDescs: __pa6, exeCosts: __pa7, commCosts: __pa8, nodeMark: __pa9, compInformations: __pa10 } = (iTargetTaskGraphMeta.clone()) else { bail!("pattern mismatch") };
    inComps = __pa1.clone();
    varCompMapping = __pa2.clone();
    eqCompMapping = __pa3.clone();
    compParamMapping = __pa4.clone();
    compNames = __pa5.clone();
    compDescs = __pa6.clone();
    exeCosts = __pa7.clone();
    commCosts = __pa8.clone();
    nodeMark = __pa9.clone();
    compInformations = __pa10.clone();
    let __range11 = 1..=(iTaskGraph.clone().borrow().len() as i32);
    for mut nodeIdx in __range11 {
        nodeComps = odeInComps.clone().borrow()[(nodeIdx.clone()-1) as usize].clone();
        for mut compIdx in &*nodeComps.clone() {
            let mut compIdx = compIdx.clone();
            componentInformation = combineComponentInformations(compInformations.clone().borrow()[(compIdx.clone()-1) as usize].clone(), iComponentInformation.clone())?;
            compInformations = {let _arr = compInformations.clone(); _arr.borrow_mut()[(compIdx.clone()-1) as usize] = componentInformation.clone(); _arr};
        }
    }
    oTargetTaskGraphMeta = TaskGraphMeta { inComps: inComps.clone(), varCompMapping: varCompMapping.clone(), eqCompMapping: eqCompMapping.clone(), compParamMapping: compParamMapping.clone(), compNames: compNames.clone(), compDescs: compDescs.clone(), exeCosts: exeCosts.clone(), commCosts: commCosts.clone(), nodeMark: nodeMark.clone(), compInformations: compInformations.clone() };
    Ok(oTargetTaskGraphMeta)
}

fn combineComponentInformations(mut iComponentInfo: ComponentInfo, mut iComponentInfo2: ComponentInfo) -> Result<ComponentInfo> {
    let mut oComponentInfo: ComponentInfo;
    let mut isPartOfODESystem: bool = false;
    let mut iIsPartOfODESystem: bool = false;
    let mut isPartOfZeroFuncSystem: bool = false;
    let mut iisPartOfZeroFuncSystem: bool = false;
    let mut isRemovedComponent: bool = false;
    let mut iIsRemovedComponent: bool = false;
    let ComponentInfo { isPartOfODESystem: __pa0, isPartOfZeroFuncSystem: __pa1, isRemovedComponent: __pa2 } = (iComponentInfo.clone()) else { bail!("pattern mismatch") };
    isPartOfODESystem = __pa0.clone();
    isPartOfZeroFuncSystem = __pa1.clone();
    isRemovedComponent = __pa2.clone();
    let ComponentInfo { isPartOfODESystem: __pa3, isPartOfZeroFuncSystem: __pa4, isRemovedComponent: __pa5 } = (iComponentInfo2.clone()) else { bail!("pattern mismatch") };
    iIsPartOfODESystem = __pa3.clone();
    iisPartOfZeroFuncSystem = __pa4.clone();
    iIsRemovedComponent = __pa5.clone();
    oComponentInfo = ComponentInfo { isPartOfODESystem: boolOr(isPartOfODESystem.clone(), iIsPartOfODESystem.clone()), isPartOfZeroFuncSystem: boolOr(isPartOfZeroFuncSystem.clone(), iisPartOfZeroFuncSystem.clone()), isRemovedComponent: boolOr(isRemovedComponent.clone(), iIsRemovedComponent.clone()) };
    Ok(oComponentInfo)
}

fn addUpExeCosts(mut iExeCost1: (i32, metamodelica::Real), mut iExeCost2: (i32, metamodelica::Real)) -> (i32, metamodelica::Real) {
    let mut oExeCost: (i32, metamodelica::Real);
    let mut ex1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut ex2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut op1: i32 = 0;
    let mut op2: i32 = 0;
    (op1, ex1) = iExeCost1.clone();
    (op2, ex2) = iExeCost2.clone();
    oExeCost = (op1.clone() + op2.clone(), (ex1.clone()) + (ex2.clone()));
    oExeCost
}

pub fn getExeCostReqCycles(mut iNodeIdx: i32, mut iGraphData: TaskGraphMeta) -> Result<metamodelica::Real> {
    let mut oExeCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    oExeCost = Util::tuple22(getExeCost(iNodeIdx.clone(), iGraphData.clone())?);
    Ok(oExeCost)
}

pub fn getExeCost(mut iNodeIdx: i32, mut iGraphData: TaskGraphMeta) -> Result<(i32, metamodelica::Real)> {
    let mut oExeCost: (i32, metamodelica::Real);
    let mut comp: i32 = 0;
    let mut opCount: i32 = 0;
    let mut opCount1: i32 = 0;
    let mut exeCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut exeCost1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut comps: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let TaskGraphMeta { exeCosts: __pa0, inComps: __pa1, .. } = (iGraphData.clone()) else { bail!("pattern mismatch") };
    exeCosts = __pa0.clone();
    inComps = __pa1.clone();
    exeCost = metamodelica::OrderedFloat(0.0_f64);
    opCount = 0;
    comps = inComps.clone().borrow()[(iNodeIdx.clone()-1) as usize].clone();
    for mut comp in &*comps.clone() {
        let mut comp = comp.clone();
        (opCount1, exeCost1) = exeCosts.clone().borrow()[(comp.clone()-1) as usize].clone();
        opCount = intAdd(opCount.clone(), opCount1.clone());
        exeCost = (exeCost.clone()) + (exeCost1.clone());
    }
    oExeCost = (opCount.clone(), exeCost.clone());
    Ok(oExeCost)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getHighestExecCost(mut iExecCosts: Arc<metamodelica::List<(i32, metamodelica::Real)>>, mut iHighestTuple: (i32, metamodelica::Real)) -> Result<(i32, metamodelica::Real)> {
    let mut oHighestTuple: (i32, metamodelica::Real);
    let mut highestCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut currentCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut head: (i32, metamodelica::Real);
    let mut rest: Arc<metamodelica::List<(i32, metamodelica::Real)>> = metamodelica::nil();
    oHighestTuple = 'mc: {
        let __mc_input = (iExecCosts.clone(), iHighestTuple.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: head @ (_, currentCost), tail: rest }, (_, highestCost)) => {
                    let true = (realGt(currentCost.clone(), highestCost.clone())) else { bail!("pattern mismatch") };
                    Ok(getHighestExecCost(rest.clone(), head.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: head @ (_, currentCost), tail: rest }, (_, highestCost)) => {
                    let true = (realGt(currentCost.clone(), highestCost.clone())) else { bail!("pattern mismatch") };
                    Ok(getHighestExecCost(rest.clone(), iHighestTuple.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iHighestTuple.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oHighestTuple)
}

pub fn contractNodesInGraph(mut iContractNodes: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut iTaskGraph: TaskGraph, mut iTaskGraphT: TaskGraph, mut iTaskGraphMeta: TaskGraphMeta, mut iContractedTasks: metamodelica::Array<i32>) -> Result<(TaskGraph, TaskGraph, TaskGraphMeta, metamodelica::Array<i32>)> {
    let mut oTaskGraph: TaskGraph;
    let mut oTaskGraphT: TaskGraph;
    let mut oTaskGraphMeta: TaskGraphMeta;
    let mut oContractedTasks: metamodelica::Array<i32>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut tmpTaskGraph: TaskGraph = iTaskGraph.clone();
    let mut tmpTaskGraphT: TaskGraph = iTaskGraphT.clone();
    let mut tmpContractedTasks: metamodelica::Array<i32> = iContractedTasks.clone();
    let mut nodeListHeadIdx: i32 = 0;
    let mut negNodeListHeadIdx: i32 = 0;
    let mut nodeIdx: i32 = 0;
    let mut nodeContractionValue: i32 = 0;
    let mut parentChild: i32 = 0;
    let mut parentChildContractionValue: i32 = 0;
    let mut nodeListRestIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut nodeCompIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut headCompIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut parentNodeChildList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut parentNodeChildListNew: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut outgoingEdges: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut incomingEdges: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut nodeMarks: metamodelica::Array<i32>;
    let mut nodeMarksT: metamodelica::Array<i32>;
    let mut iNodeList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut nodeList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut childNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut parentNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let TaskGraphMeta { inComps: __pa0, .. } = (iTaskGraphMeta.clone()) else { bail!("pattern mismatch") };
    inComps = __pa0.clone();
    nodeMarks = arrayCreate((iTaskGraph.clone().borrow().len() as i32), 0);
    nodeMarksT = arrayCreate((iTaskGraph.clone().borrow().len() as i32), 0);
    for mut iNodeList in &*iContractNodes.clone() {
        let mut iNodeList = iNodeList.clone();
        nodeList = metamodelica::nil();
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(iNodeList.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa1, tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
            _ => bail!("pattern mismatch"),
        } };
        nodeListHeadIdx = __pa1.clone();
        nodeListRestIdc = __pa2.clone();
        for mut nodeIdx in &*iNodeList.clone() {
            let mut nodeIdx = nodeIdx.clone();
            nodeIdx = getRealTaskIdxOfTask(nodeIdx.clone(), tmpContractedTasks.clone())?;
            if intNe(nodeMarks.clone().borrow()[(nodeIdx.clone()-1) as usize].clone(), nodeListHeadIdx.clone()) {
                nodeMarks = {let _arr = nodeMarks.clone(); _arr.borrow_mut()[(nodeIdx.clone()-1) as usize] = nodeListHeadIdx.clone(); _arr};
                nodeList = cons(nodeIdx.clone(), nodeList.clone());
            }
        }
        let (__pa3, __pa4) = ::match_deref::match_deref! { match &(nodeList.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
            _ => bail!("pattern mismatch"),
        } };
        nodeListHeadIdx = __pa3.clone();
        nodeListRestIdc = __pa4.clone();
        nodeListHeadIdx = getRealTaskIdxOfTask(nodeListHeadIdx.clone(), tmpContractedTasks.clone())?;
        negNodeListHeadIdx = intMul(-1, nodeListHeadIdx.clone());
        for mut nodeIdx in &*nodeListRestIdc.clone() {
            let mut nodeIdx = nodeIdx.clone();
            nodeMarks = {let _arr = nodeMarks.clone(); _arr.borrow_mut()[(nodeIdx.clone()-1) as usize] = nodeListHeadIdx.clone(); _arr};
            nodeMarksT = {let _arr = nodeMarksT.clone(); _arr.borrow_mut()[(nodeIdx.clone()-1) as usize] = nodeListHeadIdx.clone(); _arr};
            tmpContractedTasks = {let _arr = tmpContractedTasks.clone(); _arr.borrow_mut()[(nodeIdx.clone()-1) as usize] = negNodeListHeadIdx.clone(); _arr};
        }
        nodeMarks = {let _arr = nodeMarks.clone(); _arr.borrow_mut()[(nodeListHeadIdx.clone()-1) as usize] = nodeListHeadIdx.clone(); _arr};
        nodeMarksT = {let _arr = nodeMarksT.clone(); _arr.borrow_mut()[(nodeListHeadIdx.clone()-1) as usize] = nodeListHeadIdx.clone(); _arr};
        outgoingEdges = tmpTaskGraph.clone().borrow()[(nodeListHeadIdx.clone()-1) as usize].clone();
        (outgoingEdges, _) = List::deleteMemberOnTrue(negNodeListHeadIdx.clone(), outgoingEdges.clone(), Arc::new({ let __pe_b2 = tmpContractedTasks.clone(); move |__pe_a0, __pe_a1| checkIfNodeBelongsToCluster(__pe_a0, __pe_a1, __pe_b2.clone()) }))?;
        incomingEdges = tmpTaskGraphT.clone().borrow()[(nodeListHeadIdx.clone()-1) as usize].clone();
        List::map_0(outgoingEdges.clone(), Arc::new({ let __pe_b1 = nodeListHeadIdx.clone(); let __pe_b2 = nodeMarks.clone(); move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
        List::map_0(incomingEdges.clone(), Arc::new({ let __pe_b1 = nodeListHeadIdx.clone(); let __pe_b2 = nodeMarksT.clone(); move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
        childNodes = List::flatten(List::map(nodeListRestIdc.clone(), Arc::new({ let __pe_b1 = nodeListHeadIdx.clone(); let __pe_b2 = tmpTaskGraph.clone(); let __pe_b3 = tmpContractedTasks.clone(); let __pe_b4 = nodeMarks.clone(); move |__pe_a0| getContractedNodeChildren(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) })));
        parentNodes = List::flatten(List::map(nodeList.clone(), Arc::new({ let __pe_b1 = nodeListHeadIdx.clone(); let __pe_b2 = iTaskGraphT.clone(); let __pe_b3 = tmpContractedTasks.clone(); let __pe_b4 = nodeMarks.clone(); move |__pe_a0| getContractedNodeChildren(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) })));
        headCompIdc = inComps.clone().borrow()[(nodeListHeadIdx.clone()-1) as usize].clone();
        for mut nodeIdx in &*nodeListRestIdc.clone() {
            let mut nodeIdx = nodeIdx.clone();
            tmpTaskGraph = {let _arr = tmpTaskGraph.clone(); _arr.borrow_mut()[(nodeIdx.clone()-1) as usize] = metamodelica::nil(); _arr};
            tmpTaskGraphT = {let _arr = tmpTaskGraphT.clone(); _arr.borrow_mut()[(nodeIdx.clone()-1) as usize] = metamodelica::nil(); _arr};
            nodeCompIdc = inComps.clone().borrow()[(nodeIdx.clone()-1) as usize].clone();
            inComps = {let _arr = inComps.clone(); _arr.borrow_mut()[(nodeIdx.clone()-1) as usize] = metamodelica::nil(); _arr};
            headCompIdc = List::insertListSorted(headCompIdc.clone(), nodeCompIdc.clone(), (std::sync::Arc::new(fnptr!(intLt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
        }
        {let _arr = inComps.clone(); _arr.borrow_mut()[(nodeListHeadIdx.clone()-1) as usize] = headCompIdc.clone(); _arr};
        for mut nodeIdx in &*parentNodes.clone() {
            let mut nodeIdx = nodeIdx.clone();
            if intNe(nodeMarksT.clone().borrow()[(nodeIdx.clone()-1) as usize].clone(), nodeListHeadIdx.clone()) {
                incomingEdges = cons(nodeIdx.clone(), incomingEdges.clone());
            }
        }
        tmpTaskGraphT = {let _arr = tmpTaskGraphT.clone(); _arr.borrow_mut()[(nodeListHeadIdx.clone()-1) as usize] = incomingEdges.clone(); _arr};
        for mut nodeIdx in &*childNodes.clone() {
            let mut nodeIdx = nodeIdx.clone();
            parentNodeChildList = tmpTaskGraphT.clone().borrow()[(nodeIdx.clone()-1) as usize].clone();
            parentNodeChildListNew = metamodelica::nil();
            for mut parentChild in &*parentNodeChildList.clone() {
                let mut parentChild = parentChild.clone();
                parentChildContractionValue = tmpContractedTasks.clone().borrow()[(parentChild.clone()-1) as usize].clone();
                parentChild = getRealTaskIdxOfTask(parentChild.clone(), tmpContractedTasks.clone())?;
                if intEq(parentChild.clone(), nodeListHeadIdx.clone()) || intEq(parentChildContractionValue.clone(), negNodeListHeadIdx.clone()) {
                    if intNe(nodeMarksT.clone().borrow()[(parentChild.clone()-1) as usize].clone(), nodeIdx.clone()) {
                        parentNodeChildListNew = cons(nodeListHeadIdx.clone(), parentNodeChildListNew.clone());
                        let _ = {let _arr = nodeMarksT.clone(); _arr.borrow_mut()[(parentChild.clone()-1) as usize] = nodeIdx.clone(); _arr};
                    }
                } else {
                    parentNodeChildListNew = cons(parentChild.clone(), parentNodeChildListNew.clone());
                }
            }
            tmpTaskGraphT = {let _arr = tmpTaskGraphT.clone(); _arr.borrow_mut()[(nodeIdx.clone()-1) as usize] = parentNodeChildListNew.clone(); _arr};
        }
        outgoingEdges = listAppend(outgoingEdges.clone(), childNodes.clone());
        nodeMarks = {let _arr = nodeMarks.clone(); _arr.borrow_mut()[(nodeListHeadIdx.clone()-1) as usize] = 0; _arr};
        for mut nodeIdx in &*parentNodes.clone() {
            let mut nodeIdx = nodeIdx.clone();
            parentNodeChildList = tmpTaskGraph.clone().borrow()[(nodeIdx.clone()-1) as usize].clone();
            parentNodeChildListNew = metamodelica::nil();
            for mut parentChild in &*parentNodeChildList.clone() {
                let mut parentChild = parentChild.clone();
                parentChildContractionValue = tmpContractedTasks.clone().borrow()[(parentChild.clone()-1) as usize].clone();
                parentChild = getRealTaskIdxOfTask(parentChild.clone(), tmpContractedTasks.clone())?;
                if intEq(parentChild.clone(), nodeListHeadIdx.clone()) || intEq(parentChildContractionValue.clone(), negNodeListHeadIdx.clone()) {
                    if intNe(nodeMarks.clone().borrow()[(parentChild.clone()-1) as usize].clone(), nodeIdx.clone()) {
                        parentNodeChildListNew = cons(nodeListHeadIdx.clone(), parentNodeChildListNew.clone());
                        let _ = {let _arr = nodeMarks.clone(); _arr.borrow_mut()[(parentChild.clone()-1) as usize] = nodeIdx.clone(); _arr};
                    }
                } else {
                    parentNodeChildListNew = cons(parentChild.clone(), parentNodeChildListNew.clone());
                }
            }
            tmpTaskGraph = {let _arr = tmpTaskGraph.clone(); _arr.borrow_mut()[(nodeIdx.clone()-1) as usize] = parentNodeChildListNew.clone(); _arr};
        }
        tmpTaskGraph = {let _arr = tmpTaskGraph.clone(); _arr.borrow_mut()[(nodeListHeadIdx.clone()-1) as usize] = outgoingEdges.clone(); _arr};
    }
    oTaskGraph = tmpTaskGraph.clone();
    oTaskGraphT = tmpTaskGraphT.clone();
    oTaskGraphMeta = iTaskGraphMeta.clone();
    oContractedTasks = iContractedTasks.clone();
    Ok((oTaskGraph, oTaskGraphT, oTaskGraphMeta, oContractedTasks))
}

fn checkIfNodeBelongsToCluster(mut iNegativeRefValue: i32, mut iNodeIdx: i32, mut iContractedTasks: metamodelica::Array<i32>) -> Result<bool> {
    let mut oIsNodePartOfCluster: bool = false;
    oIsNodePartOfCluster = intEq(iNegativeRefValue.clone(), iContractedTasks.clone().borrow()[(iNodeIdx.clone()-1) as usize].clone());
    Ok(oIsNodePartOfCluster)
}

fn getContractedNodeChildren(mut iParentTask: i32, mut iRefValue: i32, mut iTaskGraph: TaskGraph, mut iContractedTasks: metamodelica::Array<i32>, mut iNodeMarks: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut oChildTasks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut task: i32 = 0;
    let mut taskMark: i32 = 0;
    let mut contractedValue: i32 = 0;
    let mut childTasks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut resultTasks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    childTasks = iTaskGraph.clone().borrow()[(iParentTask.clone()-1) as usize].clone();
    for mut task in &*childTasks.clone() {
        let mut task = task.clone();
        task = getRealTaskIdxOfTask(task.clone(), iContractedTasks.clone())?;
        taskMark = iNodeMarks.clone().borrow()[(task.clone()-1) as usize].clone();
        if boolAnd(intNe(taskMark.clone(), iRefValue.clone()), intNe(task.clone(), iRefValue.clone())) {
            resultTasks = cons(task.clone(), resultTasks.clone());
            let _ = {let _arr = iNodeMarks.clone(); _arr.borrow_mut()[(task.clone()-1) as usize] = iRefValue.clone(); _arr};
        }
    }
    oChildTasks = resultTasks.clone();
    Ok(oChildTasks)
}

#[tailcall::tailcall]
fn getRealTaskIdxOfTask(mut iTaskIdx: i32, mut iContractedTasks: metamodelica::Array<i32>) -> Result<i32> {
    let mut contractionMark: i32 = 0;
    contractionMark = iContractedTasks.clone().borrow()[(iTaskIdx.clone()-1) as usize].clone();
    if intLt(contractionMark.clone(), 0) {
        tailcall::call!{ getRealTaskIdxOfTask(intMul(contractionMark.clone(), -1), iContractedTasks.clone()) }
    } else {
        Ok(iTaskIdx.clone())
    }
}

pub fn setInCompsInMeta(mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut metaIn: TaskGraphMeta) -> Result<TaskGraphMeta> {
    let mut metaOut: TaskGraphMeta;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut compParamMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut compInformations: metamodelica::Array<ComponentInfo>;
    let TaskGraphMeta { compInformations: __pa0, nodeMark: __pa1, commCosts: __pa2, exeCosts: __pa3, compDescs: __pa4, compNames: __pa5, compParamMapping: __pa6, eqCompMapping: __pa7, varCompMapping: __pa8, .. } = (metaIn.clone()) else { bail!("pattern mismatch") };
    compInformations = __pa0.clone();
    nodeMark = __pa1.clone();
    commCosts = __pa2.clone();
    exeCosts = __pa3.clone();
    compDescs = __pa4.clone();
    compNames = __pa5.clone();
    compParamMapping = __pa6.clone();
    eqCompMapping = __pa7.clone();
    varCompMapping = __pa8.clone();
    metaOut = TaskGraphMeta { inComps: inComps.clone(), varCompMapping: varCompMapping.clone(), eqCompMapping: eqCompMapping.clone(), compParamMapping: compParamMapping.clone(), compNames: compNames.clone(), compDescs: compDescs.clone(), exeCosts: exeCosts.clone(), commCosts: commCosts.clone(), nodeMark: nodeMark.clone(), compInformations: compInformations.clone() };
    Ok(metaOut)
}

fn updateInCompsInfo(mut contrNode: i32, mut removedNodes: Arc<metamodelica::List<i32>>, mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    let mut comps: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut contrComps: Arc<metamodelica::List<i32>> = metamodelica::nil();
    comps = inComps.clone().borrow()[(contrNode.clone()-1) as usize].clone();
    contrComps = List::flatten(List::map(removedNodes.clone(), Arc::new({ let __pe_b1 = inComps.clone(); move |__pe_a0| Ok(Array::getIndexFirst(__pe_a0, __pe_b1.clone())) })));
    comps = List::unique(listAppend(contrComps.clone(), comps.clone()));
    {let _arr = inComps.clone(); _arr.borrow_mut()[(contrNode.clone()-1) as usize] = comps.clone(); _arr};
    Ok(())
}

pub fn filterContractedNodes(mut nodesIn: Arc<metamodelica::List<i32>>, mut contrNodes: metamodelica::Array<i32>) -> Arc<metamodelica::List<i32>> {
    let mut nodesOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    nodesOut = List::filterOnFalse(nodesIn.clone(), Arc::new({ let __pe_b1 = contrNodes.clone(); move |__pe_a0| isNodeContracted(__pe_a0, __pe_b1.clone()) }));
    nodesOut
}

pub fn filterNonContractedNodes(mut nodesIn: Arc<metamodelica::List<i32>>, mut contrNodes: metamodelica::Array<i32>) -> Arc<metamodelica::List<i32>> {
    let mut nodesOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    nodesOut = List::filterOnTrue(nodesIn.clone(), Arc::new({ let __pe_b1 = contrNodes.clone(); move |__pe_a0| isNodeContracted(__pe_a0, __pe_b1.clone()) }));
    nodesOut
}

pub fn isNodeContracted(mut iNode: i32, mut iContrNodes: metamodelica::Array<i32>) -> Result<bool> {
    let mut oIsContracted: bool = false;
    if intLe(iNode.clone(), (iContrNodes.clone().borrow().len() as i32)) {
        oIsContracted = intLt(iContrNodes.clone().borrow()[(iNode.clone()-1) as usize].clone(), 0);
    } else {
        oIsContracted = false;
    }
    Ok(oIsContracted)
}

fn contractNodesInGraph1(mut contractNodes: Arc<metamodelica::List<i32>>, mut graphIn: TaskGraph) -> Result<TaskGraph> {
    let mut graphOut: TaskGraph;
    let mut graphInT: TaskGraph;
    let mut endNode: i32 = 0;
    let mut startNode: i32 = 0;
    let mut deleteEntries: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut startNodeChildren: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut endChildren: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut deleteNodesParents: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut graphTmp: TaskGraph;
    graphInT = AdjacencyMatrix::transposeAdjacencyMatrix(graphIn.clone(), (graphIn.clone().borrow().len() as i32))?;
    startNode = List::last(contractNodes.clone())?;
    (deleteEntries, _) = List::deleteMemberOnTrue(startNode.clone(), contractNodes.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    deleteNodesParents = List::flatten(List::map1(deleteEntries.clone(), std::sync::Arc::new(fnptr!(Array::getIndexFirst, i32, _)), graphInT.clone()));
    deleteNodesParents = List::sortedUnique(List::sort(deleteNodesParents.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    deleteNodesParents = List::setDifferenceOnTrue(deleteNodesParents.clone(), contractNodes.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    endNode = listHead(contractNodes.clone())?;
    endChildren = graphIn.clone().borrow()[(endNode.clone()-1) as usize].clone();
    startNodeChildren = graphIn.clone().borrow()[(startNode.clone()-1) as usize].clone();
    startNodeChildren = List::setDifferenceOnTrue(startNodeChildren.clone(), deleteEntries.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    graphTmp = {let _arr = graphIn.clone(); _arr.borrow_mut()[(startNode.clone()-1) as usize] = startNodeChildren.clone(); _arr};
    graphTmp = List::fold2(deleteNodesParents.clone(), (std::sync::Arc::new(contractNodesInGraph2) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<i32>>, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> + 'static>), deleteEntries.clone(), startNode.clone(), graphTmp.clone());
    graphTmp = {let _arr = graphIn.clone(); _arr.borrow_mut()[(startNode.clone()-1) as usize] = endChildren.clone(); _arr};
    graphOut = graphTmp.clone();
    Ok(graphOut)
}

fn contractNodesInGraph2(mut iParentNode: i32, mut iDeletedNodes: Arc<metamodelica::List<i32>>, mut iNewNodeIdx: i32, mut iGraph: TaskGraph) -> Result<TaskGraph> {
    let mut oGraph: TaskGraph;
    let mut adjLstEntry: Arc<metamodelica::List<i32>> = metamodelica::nil();
    adjLstEntry = iGraph.clone().borrow()[(iParentNode.clone()-1) as usize].clone();
    adjLstEntry = List::setDifferenceOnTrue(adjLstEntry.clone(), iDeletedNodes.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    adjLstEntry = cons(iNewNodeIdx.clone(), adjLstEntry.clone());
    adjLstEntry = List::sortedUnique(List::sort(adjLstEntry.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    oGraph = {let _arr = iGraph.clone(); _arr.borrow_mut()[(iParentNode.clone()-1) as usize] = adjLstEntry.clone(); _arr};
    Ok(oGraph)
}

fn compareListLengthOnTrue(mut inValue: i32, mut inLst: Arc<metamodelica::List<i32>>) -> Result<bool> {
    let mut equalLength: bool = false;
    equalLength = 'mc: {
        let __mc_input = (inValue.clone(), inLst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let true = (intEq(inValue.clone(), (inLst.clone().len() as i32))) else { bail!("pattern mismatch") };
                    Ok(true)
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
    Ok(equalLength)
}

fn getMergedSystemData(mut graphDataIn: TaskGraphMeta, mut contractNodes: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<TaskGraphMeta> {
    let mut graphDataOut: TaskGraphMeta;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut compParamMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut inCompsLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut compInformations: metamodelica::Array<ComponentInfo>;
    let TaskGraphMeta { compInformations: __pa0, nodeMark: __pa1, commCosts: __pa2, exeCosts: __pa3, compDescs: __pa4, compNames: __pa5, compParamMapping: __pa6, eqCompMapping: __pa7, varCompMapping: __pa8, inComps: __pa9 } = (graphDataIn.clone()) else { bail!("pattern mismatch") };
    compInformations = __pa0.clone();
    nodeMark = __pa1.clone();
    commCosts = __pa2.clone();
    exeCosts = __pa3.clone();
    compDescs = __pa4.clone();
    compNames = __pa5.clone();
    compParamMapping = __pa6.clone();
    eqCompMapping = __pa7.clone();
    varCompMapping = __pa8.clone();
    inComps = __pa9.clone();
    inComps = updateInCompsForMerging(inComps.clone(), contractNodes.clone())?;
    compNames = List::fold2(List::intRange((compNames.clone().borrow().len() as i32)), (std::sync::Arc::new(updateCompNamesForMerging) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, metamodelica::Array<ArcStr>) -> Result<metamodelica::Array<ArcStr>> + 'static>), inComps.clone(), nodeMark.clone(), compNames.clone());
    graphDataOut = TaskGraphMeta { inComps: inComps.clone(), varCompMapping: varCompMapping.clone(), eqCompMapping: eqCompMapping.clone(), compParamMapping: compParamMapping.clone(), compNames: compNames.clone(), compDescs: compDescs.clone(), exeCosts: exeCosts.clone(), commCosts: commCosts.clone(), nodeMark: nodeMark.clone(), compInformations: compInformations.clone() };
    Ok(graphDataOut)
}

fn updateCompNamesForMerging(mut compIdx: i32, mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut nodeMark: metamodelica::Array<i32>, mut compNamesIn: metamodelica::Array<ArcStr>) -> Result<metamodelica::Array<ArcStr>> {
    let mut compNamesOut: metamodelica::Array<ArcStr>;
    compNamesOut = 'mc: {
        let __mc_input = (compIdx.clone(), inComps.clone(), nodeMark.clone(), compNamesIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut unionNode: i32 = 0;
            let mut mergedComps: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let true = (compIdx.clone() <= (compNamesIn.clone().borrow().len() as i32)) else { bail!("pattern mismatch") };
            unionNode = getCompInComps(compIdx.clone(), 1, inComps.clone(), nodeMark.clone())?;
            let true = (unionNode.clone() != -1) else { bail!("pattern mismatch") };
            mergedComps = inComps.clone().borrow()[(unionNode.clone()-1) as usize].clone();
            let true = ((mergedComps.clone().len() as i32) == 1) else { bail!("pattern mismatch") };
            Ok(compNamesIn.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut unionNode: i32 = 0;
            let mut mergedComps: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut compNamesTmp: metamodelica::Array<ArcStr>;
            let mut compName: ArcStr = arcstr::literal!("");
            let true = (compIdx.clone() <= (compNamesIn.clone().borrow().len() as i32)) else { bail!("pattern mismatch") };
            unionNode = getCompInComps(compIdx.clone(), 1, inComps.clone(), nodeMark.clone())?;
            let true = (unionNode.clone() != -1) else { bail!("pattern mismatch") };
            mergedComps = inComps.clone().borrow()[(unionNode.clone()-1) as usize].clone();
            let false = ((mergedComps.clone().len() as i32) == 1) else { bail!("pattern mismatch") };
            compName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("contracted comps ")); __mm_s.push_str(&*stringDelimitList(List::map(mergedComps.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); ArcStr::from(__mm_s) }).clone();
            compNamesTmp = {let _arr = compNamesIn.clone(); _arr.borrow_mut()[(compIdx.clone()-1) as usize] = (compName.clone()).clone(); _arr};
            Ok(compNamesTmp.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut unionNode: i32 = 0;
            let true = (compIdx.clone() <= (compNamesIn.clone().borrow().len() as i32)) else { bail!("pattern mismatch") };
            unionNode = getCompInComps(compIdx.clone(), 1, inComps.clone(), nodeMark.clone())?;
            let true = (unionNode.clone() == -1) else { bail!("pattern mismatch") };
            Ok(compNamesIn.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            println!("{}", (literal!("updateCompNamesForMerging failed!\n")).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(compNamesOut)
}

fn updateInCompsForMerging(mut inCompsIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mergedPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut inCompsOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut inCompsTmp: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut inCompsLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut deleteNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut startNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    startNodes = List::map(mergedPaths.clone(), (std::sync::Arc::new(List::last) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<_> + 'static>));
    (_, deleteNodes, _) = List::intersection1OnTrue(List::flatten(mergedPaths.clone()), startNodes.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    inCompsLst = Arc::new(inCompsIn.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
    inCompsLst = List::fold2(List::intRange((inCompsIn.clone().borrow().len() as i32)), (std::sync::Arc::new(updateInComps1) as std::sync::Arc<dyn ::std::ops::Fn(i32, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>), metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> + 'static>), (startNodes.clone(), deleteNodes.clone(), mergedPaths.clone()), inCompsIn.clone(), inCompsLst.clone());
    inCompsLst = List::removeOnTrue(metamodelica::nil(), (std::sync::Arc::new(fnptr!(equalLists, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) -> Result<bool> + 'static>), inCompsLst.clone());
    inCompsOut = metamodelica::arrayFromVec(inCompsLst.clone().into_iter().cloned().collect());
    Ok(inCompsOut)
}

fn updateInComps1(mut nodeIdx: i32, mut mergeInfo: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>), mut primInComps: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inCompLstIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut inCompLstOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    inCompLstOut = 'mc: {
        let __mc_input = (nodeIdx.clone(), mergeInfo.clone(), primInComps.clone(), inCompLstIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _) => {
                    let mut mergeGroupIdx: i32 = 0;
                    let mut inComps: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut mergedSet: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut mergedNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut startNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut mergedPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut inCompLstTmp: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    (startNodes, _, mergedPaths) = mergeInfo.clone();
                    inComps = (inCompLstIn.clone()).get(nodeIdx.clone())?;
                    let _ = (inComps.clone()).get(1)?;
                    let true = (List::isMemberOnTrue(nodeIdx.clone(), startNodes.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
                    mergeGroupIdx = List::position(nodeIdx.clone(), startNodes.clone())?;
                    mergedNodes = (mergedPaths.clone()).get(mergeGroupIdx.clone())?;
                    mergedSet = List::flatten(List::map1(mergedNodes.clone(), std::sync::Arc::new(fnptr!(Array::getIndexFirst, i32, _)), primInComps.clone()));
                    inCompLstTmp = List::fold(mergedNodes.clone(), (std::sync::Arc::new(updateInComps2) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> + 'static>), inCompLstIn.clone());
                    inCompLstTmp = List::replaceAt(mergedSet.clone(), nodeIdx.clone(), inCompLstTmp.clone())?;
                    Ok(inCompLstTmp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inCompLstIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(inCompLstOut)
}

fn updateInComps2(mut iNodeIdx: i32, mut inCompLstIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut inCompLstOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    inCompLstOut = List::replaceAt(metamodelica::nil(), iNodeIdx.clone(), inCompLstIn.clone())?;
    Ok(inCompLstOut)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn equalLists(mut inList1: Arc<metamodelica::List<i32>>, mut inList2: Arc<metamodelica::List<i32>>) -> bool {
    let mut outIsEqual: bool = false;
    outIsEqual = (::match_deref::match_deref! { match &((inList1.clone(), inList2.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            true
        },
        (Deref @ metamodelica::List::Nil, _) => {
            false
        },
        (_, Deref @ metamodelica::List::Nil) => {
            false
        },
        (Deref @ metamodelica::List::Cons { head: e1, tail: rest1 }, Deref @ metamodelica::List::Cons { head: e2, tail: rest2 }) if (intEq(e1.clone(), e2.clone())) => {
            equalLists(rest1.clone(), rest2.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsEqual
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn findOneChildParents(mut allNodes: Arc<metamodelica::List<i32>>, mut graphIn: TaskGraph, mut doNotMerge: Arc<metamodelica::List<i32>>, mut lstIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut inPath: i32, mut contrNodes: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut lstOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    lstOut = 'mc: {
        let __mc_input = (allNodes.clone(), graphIn.clone(), doNotMerge.clone(), lstIn.clone(), inPath.clone(), contrNodes.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _) => {
                    Ok(lstIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: head, tail: rest }, _, _, _, _, _) => {
                    let mut nodeChildren: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut lstTmp: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let true = (intEq(inPath.clone(), 0)) else { bail!("pattern mismatch") };
                    nodeChildren = graphIn.clone().borrow()[(head.clone()-1) as usize].clone();
                    nodeChildren = filterContractedNodes(nodeChildren.clone(), contrNodes.clone());
                    let false = ((nodeChildren.clone().len() as i32) == 1) else { bail!("pattern mismatch") };
                    lstTmp = findOneChildParents(rest.clone(), graphIn.clone(), doNotMerge.clone(), lstIn.clone(), 0, contrNodes.clone())?;
                    Ok(lstTmp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: head, tail: rest }, _, _, _, _, _) => {
                    let mut lstTmp: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let true = (intEq(inPath.clone(), 0)) else { bail!("pattern mismatch") };
                    let true = (listMember(head.clone(), doNotMerge.clone())) else { bail!("pattern mismatch") };
                    lstTmp = findOneChildParents(rest.clone(), graphIn.clone(), doNotMerge.clone(), lstIn.clone(), 0, contrNodes.clone())?;
                    Ok(lstTmp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: head, tail: rest }, _, _, _, _, _) => {
                    let mut child: i32 = 0;
                    let mut nodeChildren: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut lstTmp: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let true = (intEq(inPath.clone(), 0)) else { bail!("pattern mismatch") };
                    nodeChildren = graphIn.clone().borrow()[(head.clone()-1) as usize].clone();
                    nodeChildren = filterContractedNodes(nodeChildren.clone(), contrNodes.clone());
                    let true = ((nodeChildren.clone().len() as i32) == 1) else { bail!("pattern mismatch") };
                    child = (nodeChildren.clone()).get(1)?;
                    let true = (listMember(child.clone(), doNotMerge.clone())) else { bail!("pattern mismatch") };
                    lstTmp = findOneChildParents(rest.clone(), graphIn.clone(), doNotMerge.clone(), lstIn.clone(), child.clone(), contrNodes.clone())?;
                    Ok(lstTmp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: head, tail: rest }, _, _, _, _, _) => {
                    let mut child: i32 = 0;
                    let mut nodeChildren: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut lstTmp: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let true = (intEq(inPath.clone(), 0)) else { bail!("pattern mismatch") };
                    nodeChildren = graphIn.clone().borrow()[(head.clone()-1) as usize].clone();
                    nodeChildren = filterContractedNodes(nodeChildren.clone(), contrNodes.clone());
                    let true = ((nodeChildren.clone().len() as i32) == 1) else { bail!("pattern mismatch") };
                    child = (nodeChildren.clone()).get(1)?;
                    lstTmp = cons(list![head.clone()], lstIn.clone());
                    lstTmp = findOneChildParents(rest.clone(), graphIn.clone(), doNotMerge.clone(), lstTmp.clone(), child.clone(), contrNodes.clone())?;
                    Ok(lstTmp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: _ }, _, _, _, _, _) => {
                    let mut lstTmp: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let false = (intEq(inPath.clone(), 0)) else { bail!("pattern mismatch") };
                    let true = (listMember(inPath.clone(), doNotMerge.clone())) else { bail!("pattern mismatch") };
                    lstTmp = findOneChildParents(allNodes.clone(), graphIn.clone(), doNotMerge.clone(), lstIn.clone(), 0, contrNodes.clone())?;
                    Ok(lstTmp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _, _, _, _, _) => {
                    let mut child: i32 = 0;
                    let mut nodeChildren: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut parents: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut pathLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut lstTmp: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut rest = (*rest).clone();
                    let false = (intEq(inPath.clone(), 0)) else { bail!("pattern mismatch") };
                    nodeChildren = graphIn.clone().borrow()[(inPath.clone()-1) as usize].clone();
                    nodeChildren = filterContractedNodes(nodeChildren.clone(), contrNodes.clone());
                    parents = getParentNodes(inPath.clone(), graphIn.clone())?;
                    parents = filterContractedNodes(parents.clone(), contrNodes.clone());
                    let true = ((nodeChildren.clone().len() as i32) == 1 && !(nodeChildren.clone().is_empty()) && (parents.clone().len() as i32) == 1) else { bail!("pattern mismatch") };
                    child = (nodeChildren.clone()).get(1)?;
                    pathLst = listHead(lstIn.clone())?;
                    pathLst = cons(inPath.clone(), pathLst.clone());
                    lstTmp = List::replaceAt(pathLst.clone(), 1, lstIn.clone())?;
                    (rest, _) = List::deleteMemberOnTrue(inPath.clone(), allNodes.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    lstTmp = findOneChildParents(rest.clone(), graphIn.clone(), doNotMerge.clone(), lstTmp.clone(), child.clone(), contrNodes.clone())?;
                    Ok(lstTmp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _, _, _, _, _) => {
                    let mut nodeChildren: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut parents: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut pathLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut lstTmp: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut rest = (*rest).clone();
                    let false = (intEq(inPath.clone(), 0)) else { bail!("pattern mismatch") };
                    nodeChildren = graphIn.clone().borrow()[(inPath.clone()-1) as usize].clone();
                    nodeChildren = filterContractedNodes(nodeChildren.clone(), contrNodes.clone());
                    parents = getParentNodes(inPath.clone(), graphIn.clone())?;
                    parents = filterContractedNodes(parents.clone(), contrNodes.clone());
                    pathLst = listHead(lstIn.clone())?;
                    pathLst = cons(inPath.clone(), pathLst.clone());
                    lstTmp = List::replaceAt(pathLst.clone(), 1, lstIn.clone())?;
                    (rest, _) = List::deleteMemberOnTrue(inPath.clone(), allNodes.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    lstTmp = findOneChildParents(rest.clone(), graphIn.clone(), doNotMerge.clone(), lstTmp.clone(), 0, contrNodes.clone())?;
                    Ok(lstTmp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("findOneChildParents failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(lstOut)
}

fn getParentNodes(mut nodeIdx: i32, mut graphIn: TaskGraph) -> Result<Arc<metamodelica::List<i32>>> {
    let mut parentNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut graphInT: TaskGraph;
    graphInT = AdjacencyMatrix::transposeAdjacencyMatrix(graphIn.clone(), (graphIn.clone().borrow().len() as i32))?;
    parentNodes = graphInT.clone().borrow()[(nodeIdx.clone()-1) as usize].clone();
    Ok(parentNodes)
}

fn checkParentNode(mut lstIdx: i32, mut graphIn: TaskGraph, mut lstIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut lstOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    lstOut = 'mc: {
        let __mc_input = (lstIdx.clone(), graphIn.clone(), lstIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut childLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut child: i32 = 0;
                    let mut parent: i32 = 0;
                    let mut parents: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut lstTmp: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    childLst = (lstIn.clone()).get(lstIdx.clone())?;
                    child = List::last(childLst.clone())?;
                    parents = getParentNodes(child.clone(), graphIn.clone())?;
                    let true = (intEq((parents.clone().len() as i32), 1)) else { bail!("pattern mismatch") };
                    parent = (parents.clone()).get(1)?;
                    childLst = childLst.clone().reverse();
                    childLst = cons(parent.clone(), childLst.clone());
                    childLst = childLst.clone().reverse();
                    lstTmp = List::replaceAt(childLst.clone(), lstIdx.clone(), lstIn.clone())?;
                    Ok(lstTmp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut childLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut child: i32 = 0;
                    let mut parents: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    childLst = (lstIn.clone()).get(lstIdx.clone())?;
                    child = List::last(childLst.clone())?;
                    parents = getParentNodes(child.clone(), graphIn.clone())?;
                    let false = (intEq((parents.clone().len() as i32), 1)) else { bail!("pattern mismatch") };
                    Ok(lstIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(lstOut)
}

//-----------------------------
//  Functions to generate costs
//-----------------------------
pub fn createCosts(mut iDae: Arc<BackendDAE::BackendDAE>, mut iBenchFilePrefix: ArcStr, mut iSimEqCompMapping: metamodelica::Array<i32>, mut iTaskGraphMeta: TaskGraphMeta) -> Result<TaskGraphMeta> {
    let mut oTaskGraphMeta: TaskGraphMeta;
    let mut compMapping: metamodelica::Array<Arc<BackendDAE::EqSystem>>;
    let mut compMapping_withIdx: metamodelica::Array<(Arc<BackendDAE::EqSystem>, i32)>;
    let mut shared: Arc<BackendDAE::Shared>;
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut reqTimeCom: (i32, i32);
    let mut reqTimeOpLstSimCode: Arc<metamodelica::List<(i32, i32, metamodelica::Real)>> = metamodelica::nil();
    let mut reqTimeOpSimCode: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut tmpTaskGraphMeta: TaskGraphMeta;
    let mut reqTimeOp: metamodelica::Array<metamodelica::Real>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    oTaskGraphMeta = 'mc: {
        let __mc_input = (iDae.clone(), iBenchFilePrefix.clone(), iSimEqCompMapping.clone(), iTaskGraphMeta.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE { shared, .. }, _, _, TaskGraphMeta { commCosts, inComps, .. }) => {
                    let mut commCosts = (*commCosts).clone();
                    let mut compMapping: metamodelica::Array<Arc<BackendDAE::EqSystem>>;
                    let mut reqTimeOp: metamodelica::Array<metamodelica::Real>;
                    let mut tmpTaskGraphMeta: TaskGraphMeta;
                    let mut reqTimeOpSimCode: metamodelica::Array<(i32, metamodelica::Real)>;
                    let mut reqTimeCom: (i32, i32);
                    let mut reqTimeOpLstSimCode: Arc<metamodelica::List<(i32, i32, metamodelica::Real)>> = reqTimeOpLstSimCode.clone();
                    let mut compMapping_withIdx: metamodelica::Array<(Arc<BackendDAE::EqSystem>, i32)>;
                    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = comps.clone();
                    (comps, compMapping_withIdx) = getSystemComponents(iDae.clone())?;
                    compMapping = Array::map(compMapping_withIdx.clone(), std::sync::Arc::new(fnptr!(Util::tuple21, _)));
                    (_, reqTimeCom) = HpcOmBenchmark::benchSystem()?;
                    reqTimeOpLstSimCode = HpcOmBenchmark::readCalcTimesFromFile((iBenchFilePrefix.clone()).clone())?;
                    reqTimeOpSimCode = arrayCreate((reqTimeOpLstSimCode.clone().len() as i32), (-1, metamodelica::OrderedFloat(-1.0_f64)));
                    reqTimeOpSimCode = List::fold(reqTimeOpLstSimCode.clone(), (std::sync::Arc::new(createCosts1) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32, metamodelica::Real), metamodelica::Array<(i32, metamodelica::Real)>) -> Result<metamodelica::Array<(i32, metamodelica::Real)>> + 'static>), reqTimeOpSimCode.clone());
                    reqTimeOp = arrayCreate((comps.clone().len() as i32), metamodelica::OrderedFloat(-1.0_f64));
                    reqTimeOp = convertSimEqToSccCosts(reqTimeOpSimCode.clone(), iSimEqCompMapping.clone(), reqTimeOp.clone());
                    commCosts = createCommCosts(commCosts.clone(), 1, reqTimeCom.clone())?;
                    (_, tmpTaskGraphMeta) = Array::fold(inComps.clone(), Arc::new({ let __pe_b1 = (comps.clone(), shared.clone()); let __pe_b2 = compMapping.clone(); let __pe_b3 = reqTimeOp.clone(); let __pe_b4 = reqTimeCom.clone(); move |__pe_a0, __pe_a5| createCosts0(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_a5) }), (1, iTaskGraphMeta.clone()));
                    Ok(tmpTaskGraphMeta.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut tmpTaskGraphMeta: TaskGraphMeta;
                    tmpTaskGraphMeta = estimateCosts(iDae.clone(), iTaskGraphMeta.clone())?;
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Warning: The costs have been estimated. Maybe ")); __mm_s.push_str(&*iBenchFilePrefix.clone()); __mm_s.push_str(&*literal!("-file is missing.\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(tmpTaskGraphMeta.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oTaskGraphMeta)
}

fn estimateCosts(mut daeIn: Arc<BackendDAE::BackendDAE>, mut taskGraphMetaIn: TaskGraphMeta) -> Result<TaskGraphMeta> {
    let mut taskGraphMetaOut: TaskGraphMeta;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut comNumLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut exeCostsLst: Arc<metamodelica::List<(i32, metamodelica::Real)>> = metamodelica::nil();
    let mut eqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared>;
    let mut compsLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>>> = metamodelica::nil();
    let mut compParamMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut compInformations: metamodelica::Array<ComponentInfo>;
    let mut compIdx: i32 = 0;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(daeIn.clone()) {
        Deref @ DAE { shared: __pa0, eqs: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    shared = __pa0.clone();
    eqSystems = __pa1.clone();
    compsLst = List::map(eqSystems.clone(), (std::sync::Arc::new(fnptr!(BackendDAEUtil::getStrongComponents, Arc<BackendDAE::EqSystem>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>> + 'static>));
    comNumLst = List::map(compsLst.clone(), std::sync::Arc::new(fnptr!(listLength, _)));
    let TaskGraphMeta { compInformations: __pa2, nodeMark: __pa3, commCosts: __pa4, exeCosts: __pa5, compDescs: __pa6, compNames: __pa7, compParamMapping: __pa8, eqCompMapping: __pa9, varCompMapping: __pa10, inComps: __pa11 } = (taskGraphMetaIn.clone()) else { bail!("pattern mismatch") };
    compInformations = __pa2.clone();
    nodeMark = __pa3.clone();
    commCosts = __pa4.clone();
    exeCosts = __pa5.clone();
    compDescs = __pa6.clone();
    compNames = __pa7.clone();
    compParamMapping = __pa8.clone();
    eqCompMapping = __pa9.clone();
    varCompMapping = __pa10.clone();
    inComps = __pa11.clone();
    commCosts = getCommCostsOnly(commCosts.clone())?;
    exeCostsLst = List::flatten(List::map3(List::intRange((compsLst.clone().len() as i32)), (std::sync::Arc::new(estimateCosts0) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>>>, Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, Arc<BackendDAE::Shared>) -> Result<Arc<metamodelica::List<(i32, metamodelica::Real)>>> + 'static>), compsLst.clone(), eqSystems.clone(), shared.clone()));
    compIdx = 1;
    for mut exeCost in &*exeCostsLst.clone() {
        let mut exeCost = exeCost.clone();
        {let _arr = exeCosts.clone(); _arr.borrow_mut()[(compIdx.clone()-1) as usize] = exeCost.clone(); _arr};
        compIdx = compIdx.clone() + 1;
    }
    taskGraphMetaOut = TaskGraphMeta { inComps: inComps.clone(), varCompMapping: varCompMapping.clone(), eqCompMapping: eqCompMapping.clone(), compParamMapping: compParamMapping.clone(), compNames: compNames.clone(), compDescs: compDescs.clone(), exeCosts: exeCosts.clone(), commCosts: commCosts.clone(), nodeMark: nodeMark.clone(), compInformations: compInformations.clone() };
    Ok(taskGraphMetaOut)
}

fn estimateCosts0(mut systIdx: i32, mut compsLstIn: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>>>, mut eqSystemsIn: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, mut sharedIn: Arc<BackendDAE::Shared>) -> Result<Arc<metamodelica::List<(i32, metamodelica::Real)>>> {
    let mut exeCostsOut: Arc<metamodelica::List<(i32, metamodelica::Real)>> = metamodelica::nil();
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut eqSys: Arc<BackendDAE::EqSystem>;
    let mut compsInfos: Arc<metamodelica::List<Arc<BackendDAE::CompInfo>>> = metamodelica::nil();
    comps = (compsLstIn.clone()).get(systIdx.clone())?;
    eqSys = (eqSystemsIn.clone()).get(systIdx.clone())?;
    compsInfos = BackendDAEOptimize::countOperationstraverseComps(comps.clone(), eqSys.clone(), sharedIn.clone(), metamodelica::nil())?.reverse();
    exeCostsOut = List::map(compsInfos.clone(), (std::sync::Arc::new(calculateCosts) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::CompInfo>) -> Result<(i32, metamodelica::Real)> + 'static>));
    Ok(exeCostsOut)
}

pub fn calculateCosts(mut compInfo: Arc<BackendDAE::CompInfo>) -> Result<(i32, metamodelica::Real)> {
    let mut exeCost: (i32, metamodelica::Real);
    exeCost = 'mc: {
        let __mc_input = compInfo.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::CompInfo::COUNTER { funcCalls: numFuncs, numOth, numLog, numRelations: numRel, numTrig, numDiv, numMul, numAdds, comp } => {
                    let mut costs: i32 = 0;
                    let mut ops: i32 = 0;
                    let mut offset: i32 = 0;
                    ops = numAdds.clone() + numMul.clone() + numOth.clone() + numTrig.clone() + numRel.clone() + numLog.clone();
                    if BackendDAEUtil::isSingleEquationComp(comp.clone()) {
                        offset = 35;
                    } else if BackendDAEUtil::isWhenComp(comp.clone()) {
                        offset = 113;
                    } else if BackendDAEUtil::isArrayComp(comp.clone()) {
                        offset = 100;
                    } else {
                        offset = 0;
                    }
                    costs = offset.clone() + 12 * numAdds.clone() + 32 * numMul.clone() + 37 * numDiv.clone() + 236 * numTrig.clone() + 2 * numRel.clone() + 4 * numLog.clone() + 110 * numOth.clone() + 375 * numFuncs.clone();
                    Ok((ops.clone(), intReal(costs.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::CompInfo::SYSTEM { density: dens, size, .. } => {
                    let mut allOpCosts: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    allOpCosts = (metamodelica::OrderedFloat(0.049_f64)) * (realPow((intReal(size.clone())) * ((metamodelica::OrderedFloat(1.0_f64)) + ((dens.clone()) * (metamodelica::OrderedFloat(19.0_f64)))), metamodelica::OrderedFloat(3.0_f64)));
                    Ok((1, allOpCosts.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::CompInfo::TORN_ANALYSE { tornSize: size, otherEqs: other, tornEqs: torn, .. } => {
                    let mut ops: i32 = 0;
                    let mut ops1: i32 = 0;
                    let mut allOpCosts: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut tornCosts: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut otherCosts: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    (ops, tornCosts) = calculateCosts(torn.clone())?;
                    (ops1, otherCosts) = calculateCosts(other.clone())?;
                    allOpCosts = ((metamodelica::OrderedFloat(3000.0_f64)) + ((metamodelica::OrderedFloat(7.62_f64)) * (realPow(intReal(size.clone()), metamodelica::OrderedFloat(3.0_f64))))) + (((metamodelica::OrderedFloat(2.0_f64)) * (tornCosts.clone())) + ((metamodelica::OrderedFloat(1.4_f64)) * (otherCosts.clone())));
                    Ok((ops.clone() + ops1.clone(), allOpCosts.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::CompInfo::NO_COMP { funcCalls: numFuncs, numOth, numLog, numRelations: numRel, numTrig, numDiv, numMul, numAdds } => {
                    let mut costs: i32 = 0;
                    let mut ops: i32 = 0;
                    let mut offset: i32 = 0;
                    ops = numAdds.clone() + numMul.clone() + numOth.clone() + numTrig.clone() + numRel.clone() + numLog.clone();
                    offset = 50;
                    costs = offset.clone() + 12 * numAdds.clone() + 32 * numMul.clone() + 37 * numDiv.clone() + 236 * numTrig.clone() + 2 * numRel.clone() + 4 * numLog.clone() + 110 * numOth.clone() + 375 * numFuncs.clone();
                    Ok((ops.clone(), intReal(costs.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("calculate costs failed!\n")).clone());
                    Ok((-1, metamodelica::OrderedFloat(-1.0_f64)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(exeCost)
}

pub fn copyCosts(mut iSourceTaskGraphData: TaskGraphMeta, mut iTargetTaskGraphData: TaskGraphMeta) -> Result<TaskGraphMeta> {
    let mut oTaskGraphData: TaskGraphMeta;
    let mut inCompsSource: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut inCompsTarget: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut exeCostsSource: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut exeCostsTarget: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut compIdx: i32 = 0;
    let mut childIdx: i32 = 0;
    let mut commCostsTarget: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut reqTimeCom: (i32, i32);
    let TaskGraphMeta { exeCosts: __pa0, inComps: __pa1, .. } = (iSourceTaskGraphData.clone()) else { bail!("pattern mismatch") };
    exeCostsSource = __pa0.clone();
    inCompsSource = __pa1.clone();
    let TaskGraphMeta { commCosts: __pa2, exeCosts: __pa3, inComps: __pa4, .. } = (iTargetTaskGraphData.clone()) else { bail!("pattern mismatch") };
    commCostsTarget = __pa2.clone();
    exeCostsTarget = __pa3.clone();
    inCompsTarget = __pa4.clone();
    compIdx = intMin((exeCostsSource.clone().borrow().len() as i32), (exeCostsTarget.clone().borrow().len() as i32));
    while intGt(compIdx.clone(), 0) {
        exeCostsTarget = {let _arr = exeCostsTarget.clone(); let _val = exeCostsSource.clone().borrow()[(compIdx.clone()-1) as usize].clone(); _arr.borrow_mut()[(compIdx.clone()-1) as usize] = _val; _arr};
        compIdx = compIdx.clone() - 1;
    }
    (_, reqTimeCom) = HpcOmBenchmark::benchSystem()?;
    commCostsTarget = createCommCosts(commCostsTarget.clone(), 1, reqTimeCom.clone())?;
    oTaskGraphData = iTargetTaskGraphData.clone();
    Ok(oTaskGraphData)
}

fn getCommCostsOnly(mut commCostsIn: metamodelica::Array<Arc<metamodelica::List<Communication>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<Communication>>>> {
    let mut commCostsOut: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut reqTimeCom: (i32, i32);
    (_, reqTimeCom) = HpcOmBenchmark::benchSystem()?;
    commCostsOut = createCommCosts(commCostsIn.clone(), 1, reqTimeCom.clone())?;
    Ok(commCostsOut)
}

fn checkForExecutionCosts(mut dataIn: TaskGraphMeta) -> Result<bool> {
    let mut isFine: bool = false;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let TaskGraphMeta { exeCosts: __pa0, inComps: __pa1, .. } = (dataIn.clone()) else { bail!("pattern mismatch") };
    exeCosts = __pa0.clone();
    inComps = __pa1.clone();
    isFine = checkForExecutionCosts1(exeCosts.clone(), inComps.clone(), 1)?;
    if !(isFine.clone()) {
        println!("{}", (literal!("There are execution costs with value 0.0!\n")).clone());
    }
    Ok(isFine)
}

fn checkForExecutionCosts1(mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>, mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut nodeIdx: i32) -> Result<bool> {
    let mut bOut: bool = false;
    bOut = 'mc: {
        let __mc_input = (exeCosts.clone(), inComps.clone(), nodeIdx.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut b: bool = false;
            let mut isZero: bool = false;
            let mut comps: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let true = ((inComps.clone().borrow().len() as i32) >= nodeIdx.clone()) else { bail!("pattern mismatch") };
            comps = inComps.clone().borrow()[(nodeIdx.clone()-1) as usize].clone();
            isZero = List::fold1(comps.clone(), (std::sync::Arc::new(checkTpl2ForZero) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<(i32, metamodelica::Real)>, bool) -> Result<bool> + 'static>), exeCosts.clone(), false);
            let false = (isZero.clone()) else { bail!("pattern mismatch") };
            b = checkForExecutionCosts1(exeCosts.clone(), inComps.clone(), nodeIdx.clone() + 1)?;
            Ok(b.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let true = ((inComps.clone().borrow().len() as i32) < nodeIdx.clone()) else { bail!("pattern mismatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(false)
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(bOut)
}

fn checkTpl2ForZero(mut comp: i32, mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>, mut bIn: bool) -> Result<bool> {
    let mut bOut: bool = false;
    let mut b: bool = false;
    let mut value: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut tpl: (i32, metamodelica::Real);
    tpl = exeCosts.clone().borrow()[(comp.clone()-1) as usize].clone();
    (_, value) = tpl.clone();
    b = realEq(value.clone(), metamodelica::OrderedFloat(0.0_f64));
    bOut = b.clone() || bIn.clone();
    Ok(bOut)
}

pub fn convertNodeListToEdgeTuples(mut iNodeList: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<(i32, i32)>>> {
    let mut oEdgeList: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    oEdgeList = convertNodeListToEdgeTuples0(iNodeList.clone(), (iNodeList.clone().len() as i32), metamodelica::nil())?;
    Ok(oEdgeList)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn convertNodeListToEdgeTuples0(mut iNodeList: Arc<metamodelica::List<i32>>, mut iNodeIdx: i32, mut iEdgeList: Arc<metamodelica::List<(i32, i32)>>) -> Result<Arc<metamodelica::List<(i32, i32)>>> {
    let mut oEdgeList: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut tmpEdgeList: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut elem: i32 = 0;
    let mut preElem: i32 = 0;
    oEdgeList = 'mc: {
        let __mc_input = (iNodeList.clone(), iNodeIdx.clone(), iEdgeList.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, tmpEdgeList) => {
                    let mut tmpEdgeList = (*tmpEdgeList).clone();
                    let mut elem: i32 = elem.clone();
                    let mut preElem: i32 = preElem.clone();
                    let true = (intGt(iNodeIdx.clone(), 1)) else { bail!("pattern mismatch") };
                    elem = (iNodeList.clone()).get(iNodeIdx.clone())?;
                    preElem = (iNodeList.clone()).get(iNodeIdx.clone() - 1)?;
                    tmpEdgeList = cons((preElem.clone(), elem.clone()), tmpEdgeList.clone());
                    tmpEdgeList = convertNodeListToEdgeTuples0(iNodeList.clone(), iNodeIdx.clone() - 1, tmpEdgeList.clone())?;
                    Ok(tmpEdgeList.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iEdgeList.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oEdgeList)
}

fn convertSimEqToSccCosts(mut iReqTimeOpSimCode: metamodelica::Array<(i32, metamodelica::Real)>, mut iSimeqCompMapping: metamodelica::Array<i32>, mut iReqTimeOp: metamodelica::Array<metamodelica::Real>) -> metamodelica::Array<metamodelica::Real> {
    let mut oReqTimeOp: metamodelica::Array<metamodelica::Real>;
    (_, oReqTimeOp) = Array::fold(iReqTimeOpSimCode.clone(), Arc::new({ let __pe_b1 = iSimeqCompMapping.clone(); move |__pe_a0, __pe_a2| convertSimEqToSccCosts1(__pe_a0, __pe_b1.clone(), __pe_a2) }), (1, iReqTimeOp.clone()));
    oReqTimeOp
}

fn convertSimEqToSccCosts1(mut iReqTimeOpSimCode: (i32, metamodelica::Real), mut iSimeqCompMapping: metamodelica::Array<i32>, mut iReqTimeOp: (i32, metamodelica::Array<metamodelica::Real>)) -> Result<(i32, metamodelica::Array<metamodelica::Real>)> {
    let mut oReqTimeOp: (i32, metamodelica::Array<metamodelica::Real>);
    let mut simEqCalcCount: i32 = 0;
    let mut simEqIdx: i32 = 0;
    let mut simEqCalcTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut realSimEqCalcCount: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut reqTime: metamodelica::Array<metamodelica::Real>;
    oReqTimeOp = 'mc: {
        let __mc_input = (iReqTimeOpSimCode.clone(), iSimeqCompMapping.clone(), iReqTimeOp.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let ((mut simEqCalcCount, mut simEqCalcTime), _, (mut simEqIdx, mut reqTime)) = __mc_input.clone() else { bail!("nomatch") };
            let mut realSimEqCalcCount: metamodelica::Real = realSimEqCalcCount.clone();
            realSimEqCalcCount = intReal(simEqCalcCount.clone());
            let true = (realNe(realSimEqCalcCount.clone(), metamodelica::OrderedFloat(0.0_f64))) else { bail!("pattern mismatch") };
            reqTime = convertSimEqToSccCosts2(reqTime.clone(), realDiv(simEqCalcTime.clone(), realSimEqCalcCount.clone()), simEqIdx.clone(), iSimeqCompMapping.clone())?;
            Ok((simEqIdx.clone() + 1, reqTime.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let ((mut simEqCalcCount, mut simEqCalcTime), _, (mut simEqIdx, mut reqTime)) = __mc_input.clone() else { bail!("nomatch") };
            let mut realSimEqCalcCount: metamodelica::Real = realSimEqCalcCount.clone();
            realSimEqCalcCount = intReal(simEqCalcCount.clone());
            reqTime = convertSimEqToSccCosts2(reqTime.clone(), metamodelica::OrderedFloat(0.0_f64), simEqIdx.clone(), iSimeqCompMapping.clone())?;
            Ok((simEqIdx.clone() + 1, reqTime.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            println!("{}", (literal!("convertSimEqToSccCosts1 failed!\n")).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oReqTimeOp)
}

fn convertSimEqToSccCosts2(mut iReqTime: metamodelica::Array<metamodelica::Real>, mut iSimEqCalcTime: metamodelica::Real, mut iSimEqIdx: i32, mut iSimeqCompMapping: metamodelica::Array<i32>) -> Result<metamodelica::Array<metamodelica::Real>> {
    let mut oReqTime: metamodelica::Array<metamodelica::Real>;
    let mut reqTime: metamodelica::Array<metamodelica::Real>;
    let mut sccIdx: i32 = 0;
    oReqTime = 'mc: {
        let __mc_input = (iReqTime.clone(), iSimEqCalcTime.clone(), iSimEqIdx.clone(), iSimeqCompMapping.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut reqTime, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut sccIdx: i32 = sccIdx.clone();
            let true = (intGe((iSimeqCompMapping.clone().borrow().len() as i32), iSimEqIdx.clone())) else { bail!("pattern mismatch") };
            sccIdx = iSimeqCompMapping.clone().borrow()[(iSimEqIdx.clone()-1) as usize].clone();
            let true = (intGt(sccIdx.clone(), 0)) else { bail!("pattern mismatch") };
            reqTime = {let _arr = reqTime.clone(); _arr.borrow_mut()[(sccIdx.clone()-1) as usize] = iSimEqCalcTime.clone(); _arr};
            Ok(reqTime.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(iReqTime.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oReqTime)
}

fn createCosts0(mut iNode: Arc<metamodelica::List<i32>>, mut iComps_shared: (Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<BackendDAE::Shared>), mut iCompMapping: metamodelica::Array<Arc<BackendDAE::EqSystem>>, mut reqTimeOp: metamodelica::Array<metamodelica::Real>, mut reqTimeCom: (i32, i32), mut iTaskGraphMeta: (i32, TaskGraphMeta)) -> Result<(i32, TaskGraphMeta)> {
    let mut oTaskGraphMeta: (i32, TaskGraphMeta);
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut compParamMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut nodeRefCount: metamodelica::Array<i32>;
    let mut execCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut comp: Arc<BackendDAE::EqSystem>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut nodeNumber: i32 = 0;
    let mut taskGraphMeta: TaskGraphMeta;
    let mut compInformations: metamodelica::Array<ComponentInfo>;
    (nodeNumber, taskGraphMeta) = iTaskGraphMeta.clone();
    let TaskGraphMeta { compInformations: __pa0, nodeMark: __pa1, commCosts: __pa2, exeCosts: __pa3, compDescs: __pa4, compNames: __pa5, compParamMapping: __pa6, eqCompMapping: __pa7, varCompMapping: __pa8, inComps: __pa9 } = (taskGraphMeta.clone()) else { bail!("pattern mismatch") };
    compInformations = __pa0.clone();
    nodeRefCount = __pa1.clone();
    commCosts = __pa2.clone();
    execCosts = __pa3.clone();
    compDescs = __pa4.clone();
    compNames = __pa5.clone();
    compParamMapping = __pa6.clone();
    eqCompMapping = __pa7.clone();
    varCompMapping = __pa8.clone();
    inComps = __pa9.clone();
    createExecCost(iNode.clone(), iComps_shared.clone(), reqTimeOp.clone(), execCosts.clone(), iCompMapping.clone(), nodeNumber.clone())?;
    oTaskGraphMeta = (nodeNumber.clone() + 1, TaskGraphMeta { inComps: inComps.clone(), varCompMapping: varCompMapping.clone(), eqCompMapping: eqCompMapping.clone(), compParamMapping: compParamMapping.clone(), compNames: compNames.clone(), compDescs: compDescs.clone(), exeCosts: execCosts.clone(), commCosts: commCosts.clone(), nodeMark: nodeRefCount.clone(), compInformations: compInformations.clone() });
    Ok(oTaskGraphMeta)
}

fn createCosts1(mut iTuple: (i32, i32, metamodelica::Real), mut iReqTime: metamodelica::Array<(i32, metamodelica::Real)>) -> Result<metamodelica::Array<(i32, metamodelica::Real)>> {
    let mut oReqTime: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut tmpArray: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut simEqIdx: i32 = 0;
    let mut calcTimeCount: i32 = 0;
    let mut calcTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    oReqTime = (match (iTuple.clone(), iReqTime.clone()) {
        ((0, mut calcTimeCount, mut calcTime), _) => iReqTime.clone(),
        ((mut simEqIdx, mut calcTimeCount, mut calcTime), mut tmpArray) => {
            tmpArray = {let _arr = iReqTime.clone(); _arr.borrow_mut()[(simEqIdx.clone()-1) as usize] = (calcTimeCount.clone(), calcTime.clone()); _arr};
            tmpArray.clone()
        },
    });
    Ok(oReqTime)
}

fn createExecCost(mut iNodeSccs: Arc<metamodelica::List<i32>>, mut icomps_shared: (Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<BackendDAE::Shared>), mut iRequiredTime: metamodelica::Array<metamodelica::Real>, mut iExecCosts: metamodelica::Array<(i32, metamodelica::Real)>, mut compMapping: metamodelica::Array<Arc<BackendDAE::EqSystem>>, mut iNodeIdx: i32) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (iNodeSccs.clone(), icomps_shared.clone(), iRequiredTime.clone(), iExecCosts.clone(), compMapping.clone(), iNodeIdx.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _) => {
                    let mut execCost: (i32, metamodelica::Real);
                    execCost = List::fold3(iNodeSccs.clone(), (std::sync::Arc::new(createExecCost0) as std::sync::Arc<dyn ::std::ops::Fn(i32, (Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<BackendDAE::Shared>), metamodelica::Array<Arc<BackendDAE::EqSystem>>, metamodelica::Array<metamodelica::Real>, (i32, metamodelica::Real)) -> Result<(i32, metamodelica::Real)> + 'static>), icomps_shared.clone(), compMapping.clone(), iRequiredTime.clone(), (0, metamodelica::OrderedFloat(0.0_f64)));
                    {let _arr = iExecCosts.clone(); _arr.borrow_mut()[(iNodeIdx.clone()-1) as usize] = execCost.clone(); _arr};
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

fn createExecCost0(mut sccIndex: i32, mut icomps_shared: (Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<BackendDAE::Shared>), mut compMapping: metamodelica::Array<Arc<BackendDAE::EqSystem>>, mut iRequiredTime: metamodelica::Array<metamodelica::Real>, mut iCosts: (i32, metamodelica::Real)) -> Result<(i32, metamodelica::Real)> {
    let mut oCosts: (i32, metamodelica::Real);
    let mut costAdd: i32 = 0;
    let mut costMul: i32 = 0;
    let mut costTrig: i32 = 0;
    let mut iCosts_op: i32 = 0;
    let mut iCosts_cyc: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut comp: Arc<BackendDAE::StrongComponent>;
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut syst: Arc<BackendDAE::EqSystem>;
    let mut shared: Arc<BackendDAE::Shared>;
    let mut reqTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    (comps, shared) = icomps_shared.clone();
    (iCosts_op, iCosts_cyc) = iCosts.clone();
    comp = (comps.clone()).get(sccIndex.clone())?;
    syst = compMapping.clone().borrow()[(sccIndex.clone()-1) as usize].clone();
    reqTime = iRequiredTime.clone().borrow()[(sccIndex.clone()-1) as usize].clone();
    oCosts = (-100 + iCosts_op.clone(), (iCosts_cyc.clone()) + (reqTime.clone()));
    Ok(oCosts)
}

fn createCommCosts(mut iCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>, mut iCurrentIndex: i32, mut iReqTimeCom: (i32, i32)) -> Result<metamodelica::Array<Arc<metamodelica::List<Communication>>>> {
    let mut oCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut tmpCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut currentCom: Communications = metamodelica::nil();
    oCosts = 'mc: {
        let __mc_input = (iCosts.clone(), iCurrentIndex.clone(), iReqTimeCom.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut tmpCosts, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut currentCom: Arc<metamodelica::List<Communication>> = currentCom.clone();
            let true = (intLe(iCurrentIndex.clone(), (iCosts.clone().borrow().len() as i32))) else { bail!("pattern mismatch") };
            currentCom = tmpCosts.clone().borrow()[(iCurrentIndex.clone()-1) as usize].clone();
            currentCom = List::map1(currentCom.clone(), (std::sync::Arc::new(createCommCosts0) as std::sync::Arc<dyn ::std::ops::Fn(Communication, (i32, i32)) -> Result<Communication> + 'static>), iReqTimeCom.clone());
            tmpCosts = {let _arr = tmpCosts.clone(); _arr.borrow_mut()[(iCurrentIndex.clone()-1) as usize] = currentCom.clone(); _arr};
            tmpCosts = createCommCosts(tmpCosts.clone(), iCurrentIndex.clone() + 1, iReqTimeCom.clone())?;
            Ok(tmpCosts.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(iCosts.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oCosts)
}

fn createCommCosts0(mut iComm: Communication, mut iReqTimeCom: (i32, i32)) -> Result<Communication> {
    let mut oComm: Communication;
    let mut childNode: i32 = 0;
    let mut reqTimeM: i32 = 0;
    let mut reqTimeN: i32 = 0;
    let mut numberOfVars: i32 = 0;
    let mut requiredTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut integerVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut floatVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut booleanVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut stringVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let Communication { requiredTime: __pa0, childNode: __pa1, stringVars: __pa2, booleanVars: __pa3, floatVars: __pa4, integerVars: __pa5, numberOfVars: __pa6 } = (iComm.clone()) else { bail!("pattern mismatch") };
    requiredTime = __pa0.clone();
    childNode = __pa1.clone();
    stringVars = __pa2.clone();
    booleanVars = __pa3.clone();
    floatVars = __pa4.clone();
    integerVars = __pa5.clone();
    numberOfVars = __pa6.clone();
    (reqTimeM, reqTimeN) = iReqTimeCom.clone();
    requiredTime = intReal(reqTimeN.clone() + numberOfVars.clone() * reqTimeM.clone());
    oComm = Communication { numberOfVars: numberOfVars.clone(), integerVars: integerVars.clone(), floatVars: floatVars.clone(), booleanVars: booleanVars.clone(), stringVars: stringVars.clone(), childNode: childNode.clone(), requiredTime: requiredTime.clone() };
    Ok(oComm)
}

//---------------------------------
//  Functions to validate the graph
//---------------------------------
pub fn validateTaskGraphMeta(mut iMeta: TaskGraphMeta, mut iDae: Arc<BackendDAE::BackendDAE>) -> Result<bool> {
    let mut valid: bool = false;
    valid = 'mc: {
        let __mc_input = (iMeta.clone(), iDae.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut systComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
                    let mut graphComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
                    let mut systCompsArray: metamodelica::Array<Arc<BackendDAE::StrongComponent>>;
                    let mut systCompEqSysMapping: metamodelica::Array<(Arc<BackendDAE::EqSystem>, i32)>;
                    let mut graphCompEqSysMapping: metamodelica::Array<(Arc<BackendDAE::EqSystem>, i32)>;
                    let mut systCompEqSysMappingIdx: Arc<metamodelica::List<(Arc<BackendDAE::StrongComponent>, i32)>> = metamodelica::nil();
                    let mut graphCompEqSysMappingIdx: Arc<metamodelica::List<(Arc<BackendDAE::StrongComponent>, i32)>> = metamodelica::nil();
                    (systComps, systCompEqSysMapping) = getSystemComponents(iDae.clone())?;
                    systCompsArray = metamodelica::arrayFromVec(systComps.clone().into_iter().cloned().collect());
                    (graphComps, graphCompEqSysMapping) = getGraphComponents(iMeta.clone(), systCompsArray.clone(), systCompEqSysMapping.clone())?;
                    (_, _, systCompEqSysMappingIdx) = validateTaskGraphMeta0(systCompEqSysMapping.clone(), (1, systComps.clone(), metamodelica::nil()))?;
                    (_, _, graphCompEqSysMappingIdx) = validateTaskGraphMeta0(graphCompEqSysMapping.clone(), (1, graphComps.clone(), metamodelica::nil()))?;
                    let true = (validateComponents(graphCompEqSysMappingIdx.clone(), systCompEqSysMappingIdx.clone())?) else { bail!("pattern mismatch") };
                    let true = (checkForDuplicates(graphCompEqSysMappingIdx.clone())?) else { bail!("pattern mismatch") };
                    let true = (checkForExecutionCosts(iMeta.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
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
    Ok(valid)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn validateTaskGraphMeta0(mut iEqSysMapping: metamodelica::Array<(Arc<BackendDAE::EqSystem>, i32)>, mut iCompsTpl: (i32, Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::StrongComponent>, i32)>>)) -> Result<(i32, Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::StrongComponent>, i32)>>)> {
    let mut oCompsTpl: (i32, Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::StrongComponent>, i32)>>);
    let mut currentIdx: i32 = 0;
    let mut eqSysIdx: i32 = 0;
    let mut iComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut rest: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut head: Arc<BackendDAE::StrongComponent>;
    let mut iCompEqSysMapping: Arc<metamodelica::List<(Arc<BackendDAE::StrongComponent>, i32)>> = metamodelica::nil();
    let mut oCompEqSysMapping: Arc<metamodelica::List<(Arc<BackendDAE::StrongComponent>, i32)>> = metamodelica::nil();
    let mut tmpCompsTpl: (i32, Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::StrongComponent>, i32)>>);
    oCompsTpl = (::match_deref::match_deref! { match &((iEqSysMapping.clone(), iCompsTpl.clone())) {
        (_, (currentIdx, Deref @ metamodelica::List::Cons { head: head, tail: rest }, iCompEqSysMapping)) => {
            (_, eqSysIdx) = iEqSysMapping.clone().borrow()[(currentIdx.clone()-1) as usize].clone();
            oCompEqSysMapping = cons((head.clone(), eqSysIdx.clone()), iCompEqSysMapping.clone());
            tmpCompsTpl = validateTaskGraphMeta0(iEqSysMapping.clone(), (currentIdx.clone() + 1, rest.clone(), oCompEqSysMapping.clone()))?;
            tmpCompsTpl.clone()
        },
        _ => iCompsTpl.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oCompsTpl)
}

fn validateComponents(mut graphComps: Arc<metamodelica::List<(Arc<BackendDAE::StrongComponent>, i32)>>, mut systComps: Arc<metamodelica::List<(Arc<BackendDAE::StrongComponent>, i32)>>) -> Result<bool> {
    let mut res: bool = false;
    let mut isEqual: bool = false;
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    let mut comp1: Arc<BackendDAE::StrongComponent>;
    let mut comp2: Arc<BackendDAE::StrongComponent>;
    let mut tpl1: (Arc<BackendDAE::StrongComponent>, i32);
    let mut tpl2: (Arc<BackendDAE::StrongComponent>, i32);
    let mut sortedGraphComps: Arc<metamodelica::List<(Arc<BackendDAE::StrongComponent>, i32)>> = metamodelica::nil();
    let mut sortedSystComps: Arc<metamodelica::List<(Arc<BackendDAE::StrongComponent>, i32)>> = metamodelica::nil();
    res = 'mc: {
        let __mc_input = (graphComps.clone(), systComps.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut comp1: Arc<BackendDAE::StrongComponent>;
                    let mut tpl2: (Arc<BackendDAE::StrongComponent>, i32);
                    let mut i1: i32 = i1.clone();
                    let mut comp2: Arc<BackendDAE::StrongComponent>;
                    let mut sortedGraphComps: Arc<metamodelica::List<(Arc<BackendDAE::StrongComponent>, i32)>> = sortedGraphComps.clone();
                    let mut i2: i32 = i2.clone();
                    let mut isEqual: bool = isEqual.clone();
                    let mut sortedSystComps: Arc<metamodelica::List<(Arc<BackendDAE::StrongComponent>, i32)>> = sortedSystComps.clone();
                    let mut tpl1: (Arc<BackendDAE::StrongComponent>, i32);
                    sortedGraphComps = List::sort(graphComps.clone(), (std::sync::Arc::new(compareComponents) as std::sync::Arc<dyn ::std::ops::Fn((Arc<BackendDAE::StrongComponent>, i32), (Arc<BackendDAE::StrongComponent>, i32)) -> Result<bool> + 'static>))?;
                    sortedSystComps = List::sort(systComps.clone(), (std::sync::Arc::new(compareComponents) as std::sync::Arc<dyn ::std::ops::Fn((Arc<BackendDAE::StrongComponent>, i32), (Arc<BackendDAE::StrongComponent>, i32)) -> Result<bool> + 'static>))?;
                    if intNe((sortedSystComps.clone().len() as i32), (sortedGraphComps.clone().len() as i32)) {
                        println!("{}", (literal!("the graph and the system have a difference number of components.\n")).clone());
                    }
                    isEqual = true;
                    while isEqual.clone() && !(sortedGraphComps.clone().is_empty()) {
                        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(sortedGraphComps.clone()) {
                            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                            _ => bail!("pattern mismatch"),
                        } };
                        tpl1 = __pa0.clone();
                        sortedGraphComps = __pa1.clone();
                        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(sortedSystComps.clone()) {
                            Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                            _ => bail!("pattern mismatch"),
                        } };
                        tpl2 = __pa2.clone();
                        sortedSystComps = __pa3.clone();
                        (comp1, i1) = tpl1.clone();
                        (comp2, i2) = tpl2.clone();
                        if componentsEqual(tpl1.clone(), tpl2.clone())? {
                            isEqual = true;
                        } else {
                            isEqual = false;
                            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("comp ")); __mm_s.push_str(&*intString(i1.clone())); __mm_s.push_str(&*BackendDump::printComponent(comp1.clone(), None)?); __mm_s.push_str(&*literal!(" is not equal to ")); __mm_s.push_str(&*literal!("comp")); __mm_s.push_str(&*intString(i2.clone())); __mm_s.push_str(&*BackendDump::printComponent(comp2.clone(), None)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                        }
                    }
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("Different components in graph and system\n")).clone());
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(res)
}

fn checkForDuplicates(mut iComps: Arc<metamodelica::List<(Arc<BackendDAE::StrongComponent>, i32)>>) -> Result<bool> {
    let mut res: bool = false;
    let mut sortedComps: Arc<metamodelica::List<(Arc<BackendDAE::StrongComponent>, i32)>> = metamodelica::nil();
    sortedComps = List::sort(iComps.clone(), (std::sync::Arc::new(compareComponents) as std::sync::Arc<dyn ::std::ops::Fn((Arc<BackendDAE::StrongComponent>, i32), (Arc<BackendDAE::StrongComponent>, i32)) -> Result<bool> + 'static>))?;
    (res, _) = List::fold(sortedComps.clone(), (std::sync::Arc::new(checkForDuplicates0) as std::sync::Arc<dyn ::std::ops::Fn((Arc<BackendDAE::StrongComponent>, i32), (bool, Option<(Arc<BackendDAE::StrongComponent>, i32)>)) -> Result<(bool, Option<(Arc<BackendDAE::StrongComponent>, i32)>)> + 'static>), (true, None));
    Ok(res)
}

fn checkForDuplicates0(mut currentComp_idx: (Arc<BackendDAE::StrongComponent>, i32), mut iLastComp: (bool, Option<(Arc<BackendDAE::StrongComponent>, i32)>)) -> Result<(bool, Option<(Arc<BackendDAE::StrongComponent>, i32)>)> {
    let mut oLastComp: (bool, Option<(Arc<BackendDAE::StrongComponent>, i32)>);
    let mut lastComp: Arc<BackendDAE::StrongComponent>;
    let mut currentComp: Arc<BackendDAE::StrongComponent>;
    let mut lastComp_idx: (Arc<BackendDAE::StrongComponent>, i32);
    let mut idxLast: i32 = 0;
    let mut idxCurrent: i32 = 0;
    oLastComp = 'mc: {
        let __mc_input = (currentComp_idx.clone(), iLastComp.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (false, _)) => {
                    Ok((false, Some(currentComp_idx.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (_, None)) => {
                    Ok((true, Some(currentComp_idx.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((currentComp, idxCurrent), (_, Some(lastComp_idx @ (lastComp, idxLast)))) => {
                    let true = (componentsEqual(currentComp_idx.clone(), lastComp_idx.clone())?) else { bail!("pattern mismatch") };
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Component duplicate detected: current: ")); __mm_s.push_str(&*BackendDump::printComponent(currentComp.clone(), None)?); __mm_s.push_str(&*literal!(" (eqSystem ")); __mm_s.push_str(&*intString(idxCurrent.clone())); __mm_s.push_str(&*literal!(") last ")); __mm_s.push_str(&*BackendDump::printComponent(lastComp.clone(), None)?); __mm_s.push_str(&*literal!(" (eqSystem ")); __mm_s.push_str(&*intString(idxLast.clone())); __mm_s.push_str(&*literal!(").\n")); ArcStr::from(__mm_s) }).clone());
                    Ok((false, Some(currentComp_idx.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((true, Some(currentComp_idx.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oLastComp)
}

fn getGraphComponents(mut iTaskGraphMeta: TaskGraphMeta, mut iSystComps: metamodelica::Array<Arc<BackendDAE::StrongComponent>>, mut iCompEqSysMapping: metamodelica::Array<(Arc<BackendDAE::EqSystem>, i32)>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, metamodelica::Array<(Arc<BackendDAE::EqSystem>, i32)>)> {
    let mut oComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut oCompEqGraphMapping: metamodelica::Array<(Arc<BackendDAE::EqSystem>, i32)>;
    let mut tmpComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut tmpMapping: Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>> = metamodelica::nil();
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut nodeMarks: metamodelica::Array<i32>;
    tmpComps = metamodelica::nil();
    tmpMapping = metamodelica::nil();
    let TaskGraphMeta { nodeMark: __pa0, inComps: __pa1, .. } = (iTaskGraphMeta.clone()) else { bail!("pattern mismatch") };
    nodeMarks = __pa0.clone();
    inComps = __pa1.clone();
    (tmpComps, tmpMapping) = Array::fold(inComps.clone(), Arc::new({ let __pe_b1 = iSystComps.clone(); let __pe_b2 = iCompEqSysMapping.clone(); move |__pe_a0, __pe_a3| Ok(getGraphComponents0(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3)) }), (tmpComps.clone(), tmpMapping.clone()));
    let (_, (__pa2, __pa3)) = Array::fold(nodeMarks.clone(), Arc::new({ let __pe_b1 = iSystComps.clone(); let __pe_b2 = iCompEqSysMapping.clone(); move |__pe_a0, __pe_a3| getGraphComponents2(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }), (1, (tmpComps.clone(), tmpMapping.clone())));
    tmpComps = __pa2.clone();
    tmpMapping = __pa3.clone();
    oComps = tmpComps.clone();
    oCompEqGraphMapping = metamodelica::arrayFromVec(tmpMapping.clone().into_iter().cloned().collect());
    Ok((oComps, oCompEqGraphMapping))
}

fn getGraphComponents0(mut inComp: Arc<metamodelica::List<i32>>, mut systComps: metamodelica::Array<Arc<BackendDAE::StrongComponent>>, mut iCompEqSysMapping: metamodelica::Array<(Arc<BackendDAE::EqSystem>, i32)>, mut iNodeComps_Mapping: (Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>)) -> (Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>) {
    let mut oNodeComps_Mapping: (Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>);
    let mut iNodeComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut tmpNodeComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut iCompsMapping: Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>> = metamodelica::nil();
    let mut tmpCompsMapping: Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>> = metamodelica::nil();
    (iNodeComps, iCompsMapping) = iNodeComps_Mapping.clone();
    (tmpNodeComps, tmpCompsMapping) = List::fold2(inComp.clone(), (std::sync::Arc::new(getGraphComponents1) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<BackendDAE::StrongComponent>>, metamodelica::Array<(Arc<BackendDAE::EqSystem>, i32)>, (Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>)) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>)> + 'static>), systComps.clone(), iCompEqSysMapping.clone(), (metamodelica::nil(), metamodelica::nil()));
    tmpNodeComps = listAppend(iNodeComps.clone(), tmpNodeComps.clone());
    tmpCompsMapping = listAppend(iCompsMapping.clone(), tmpCompsMapping.clone());
    oNodeComps_Mapping = (tmpNodeComps.clone(), tmpCompsMapping.clone());
    oNodeComps_Mapping
}

fn getGraphComponents1(mut compIdx: i32, mut systComps: metamodelica::Array<Arc<BackendDAE::StrongComponent>>, mut iCompEqSysMapping: metamodelica::Array<(Arc<BackendDAE::EqSystem>, i32)>, mut iNodeComps_Mapping: (Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>)) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>)> {
    let mut oNodeComps_Mapping: (Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>);
    let mut comp: Arc<BackendDAE::StrongComponent>;
    let mut eqSyst: (Arc<BackendDAE::EqSystem>, i32);
    let mut tmpComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut tmpSysts: Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>> = metamodelica::nil();
    (tmpComps, tmpSysts) = iNodeComps_Mapping.clone();
    comp = systComps.clone().borrow()[(compIdx.clone()-1) as usize].clone();
    eqSyst = iCompEqSysMapping.clone().borrow()[(compIdx.clone()-1) as usize].clone();
    tmpComps = cons(comp.clone(), tmpComps.clone());
    tmpSysts = cons(eqSyst.clone(), tmpSysts.clone());
    oNodeComps_Mapping = (tmpComps.clone(), tmpSysts.clone());
    Ok(oNodeComps_Mapping)
}

fn getGraphComponents2(mut nodeMark: i32, mut systComps: metamodelica::Array<Arc<BackendDAE::StrongComponent>>, mut iCompEqSysMapping: metamodelica::Array<(Arc<BackendDAE::EqSystem>, i32)>, mut iNodeComps_Mapping: (i32, (Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>))) -> Result<(i32, (Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>))> {
    let mut oNodeComps_Mapping: (i32, (Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>));
    let mut nodeIdx: i32 = 0;
    let mut comp: Arc<BackendDAE::StrongComponent>;
    let mut eqSyst: (Arc<BackendDAE::EqSystem>, i32);
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut eqSysts: Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>> = metamodelica::nil();
    oNodeComps_Mapping = 'mc: {
        let __mc_input = (nodeMark.clone(), systComps.clone(), iCompEqSysMapping.clone(), iNodeComps_Mapping.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, (nodeIdx, (comps, eqSysts))) => {
                    let true = (intGe(nodeMark.clone(), 0)) else { bail!("pattern mismatch") };
                    Ok((nodeIdx.clone() + 1, (comps.clone(), eqSysts.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, (nodeIdx, (comps, eqSysts))) => {
                    let true = (intEq(nodeMark.clone(), -2)) else { bail!("pattern mismatch") };
                    Ok((nodeIdx.clone() + 1, (comps.clone(), eqSysts.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, (nodeIdx, (comps, eqSysts))) => {
                    let mut comps = (*comps).clone();
                    let mut eqSysts = (*eqSysts).clone();
                    let mut eqSyst: (Arc<BackendDAE::EqSystem>, i32);
                    let mut comp: Arc<BackendDAE::StrongComponent>;
                    comp = systComps.clone().borrow()[(nodeIdx.clone()-1) as usize].clone();
                    eqSyst = iCompEqSysMapping.clone().borrow()[(nodeIdx.clone()-1) as usize].clone();
                    comps = cons(comp.clone(), comps.clone());
                    eqSysts = cons(eqSyst.clone(), eqSysts.clone());
                    Ok((nodeIdx.clone() + 1, (comps.clone(), eqSysts.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oNodeComps_Mapping)
}

fn componentsEqual(mut iComp1: (Arc<BackendDAE::StrongComponent>, i32), mut iComp2: (Arc<BackendDAE::StrongComponent>, i32)) -> Result<bool> {
    let mut res: bool = false;
    let mut comp1Str: ArcStr = arcstr::literal!("");
    let mut comp2Str: ArcStr = arcstr::literal!("");
    let mut comp1Idx: i32 = 0;
    let mut comp2Idx: i32 = 0;
    let mut comp1: Arc<BackendDAE::StrongComponent>;
    let mut comp2: Arc<BackendDAE::StrongComponent>;
    (comp1, comp1Idx) = iComp1.clone();
    (comp2, comp2Idx) = iComp2.clone();
    comp1Str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*BackendDump::printComponent(comp1.clone(), None)?); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(comp1Idx.clone())); ArcStr::from(__mm_s) }).clone();
    comp2Str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*BackendDump::printComponent(comp2.clone(), None)?); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(comp2Idx.clone())); ArcStr::from(__mm_s) }).clone();
    if intNe(((comp1Str.clone()).clone().len() as i32), ((comp2Str.clone()).clone().len() as i32)) {
        res = false;
    } else {
        res = intEq(System::strncmp((comp1Str.clone()).clone(), (comp2Str.clone()).clone(), ((comp1Str.clone()).clone().len() as i32)), 0);
    }
    Ok(res)
}

fn compareComponents(mut iComp1: (Arc<BackendDAE::StrongComponent>, i32), mut iComp2: (Arc<BackendDAE::StrongComponent>, i32)) -> Result<bool> {
    let mut res: bool = false;
    let mut comp1Str: ArcStr = arcstr::literal!("");
    let mut comp2Str: ArcStr = arcstr::literal!("");
    let mut minLength: i32 = 0;
    let mut compRes: i32 = 0;
    let mut comp1Idx: i32 = 0;
    let mut comp2Idx: i32 = 0;
    let mut comp1: Arc<BackendDAE::StrongComponent>;
    let mut comp2: Arc<BackendDAE::StrongComponent>;
    if componentsEqual(iComp1.clone(), iComp2.clone())? {
        res = false;
    } else {
        (comp1, comp1Idx) = iComp1.clone();
        (comp2, comp2Idx) = iComp2.clone();
        comp1Str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*BackendDump::printComponent(comp1.clone(), None)?); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(comp1Idx.clone())); ArcStr::from(__mm_s) }).clone();
        comp2Str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*BackendDump::printComponent(comp2.clone(), None)?); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(comp2Idx.clone())); ArcStr::from(__mm_s) }).clone();
        minLength = intMin(((comp1Str.clone()).clone().len() as i32), ((comp2Str.clone()).clone().len() as i32));
        compRes = System::strncmp((comp1Str.clone()).clone(), (comp2Str.clone()).clone(), minLength.clone());
        if intEq(compRes.clone(), 0) {
            res = intLt(((comp1Str.clone()).clone().len() as i32), ((comp2Str.clone()).clone().len() as i32));
        } else {
            res = intLt(compRes.clone(), 0);
        }
    }
    Ok(res)
}

//------------------------------------
//  Evaluation and analysing functions
//------------------------------------
pub fn getCriticalPaths(mut graphIn: TaskGraph, mut graphDataIn: TaskGraphMeta) -> Result<((Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, metamodelica::Real), (Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, metamodelica::Real))> {
    let mut criticalPathOut: (Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, metamodelica::Real);
    let mut criticalPathOutWoC: (Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, metamodelica::Real);
    (criticalPathOut, criticalPathOutWoC) = 'mc: {
        let __mc_input = (graphIn.clone(), graphDataIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, TaskGraphMeta { .. }) = __mc_input.clone() else { bail!("nomatch") };
            let mut rootNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut cpWCpaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut CpWoCpaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut cpWCcosts: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut cpWoCcosts: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let true = ((graphIn.clone().borrow().len() as i32) != 0) else { bail!("pattern mismatch") };
            rootNodes = getRootNodes(graphIn.clone())?;
            (cpWCpaths, cpWCcosts) = getCriticalPath(graphIn.clone(), graphDataIn.clone(), rootNodes.clone(), true)?;
            (CpWoCpaths, cpWoCcosts) = getCriticalPath(graphIn.clone(), graphDataIn.clone(), rootNodes.clone(), false)?;
            cpWCcosts = roundReal(cpWCcosts.clone(), 2);
            cpWoCcosts = roundReal(cpWoCcosts.clone(), 2);
            Ok(((cpWCpaths.clone(), cpWCcosts.clone()), (CpWoCpaths.clone(), cpWoCcosts.clone())))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _) = __mc_input.clone() else { bail!("nomatch") };
            let true = ((graphIn.clone().borrow().len() as i32) == 0) else { bail!("pattern mismatch") };
            Ok(((list![metamodelica::nil()], metamodelica::OrderedFloat(0.0_f64)), (list![metamodelica::nil()], metamodelica::OrderedFloat(0.0_f64))))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            println!("{}", (literal!("getCriticalPaths failed!\n")).clone());
            Ok(((list![metamodelica::nil()], metamodelica::OrderedFloat(0.0_f64)), (list![metamodelica::nil()], metamodelica::OrderedFloat(0.0_f64))))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((criticalPathOut, criticalPathOutWoC))
}

fn getCriticalPath(mut iGraph: TaskGraph, mut iGraphData: TaskGraphMeta, mut iRootNodes: Arc<metamodelica::List<i32>>, mut iHandleCommCosts: bool) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, metamodelica::Real)> {
    let mut oCriticalPathsOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut oCpCosts: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut nodeCriticalPaths: metamodelica::Array<(metamodelica::Real, Arc<metamodelica::List<i32>>)>;
    let mut criticalPaths: Arc<metamodelica::List<(metamodelica::Real, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut criticalPathIdx: i32 = 0;
    let mut criticalPath: Arc<metamodelica::List<i32>> = metamodelica::nil();
    nodeCriticalPaths = arrayCreate((iGraph.clone().borrow().len() as i32), (metamodelica::OrderedFloat(-1.0_f64), metamodelica::nil()));
    criticalPaths = List::map4(iRootNodes.clone(), (std::sync::Arc::new(getCriticalPath1) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, TaskGraphMeta, bool, metamodelica::Array<(metamodelica::Real, Arc<metamodelica::List<i32>>)>) -> Result<(metamodelica::Real, Arc<metamodelica::List<i32>>)> + 'static>), iGraph.clone(), iGraphData.clone(), iHandleCommCosts.clone(), nodeCriticalPaths.clone());
    criticalPathIdx = getCriticalPath2(criticalPaths.clone(), 1, metamodelica::OrderedFloat(-1.0_f64), -1)?;
    (oCpCosts, criticalPath) = (criticalPaths.clone()).get(criticalPathIdx.clone())?;
    oCriticalPathsOut = list![criticalPath.clone()];
    Ok((oCriticalPathsOut, oCpCosts))
}

fn getCriticalPath1(mut iNode: i32, mut iGraph: TaskGraph, mut iGraphData: TaskGraphMeta, mut iHandleCommCosts: bool, mut iNodeCriticalPaths: metamodelica::Array<(metamodelica::Real, Arc<metamodelica::List<i32>>)>) -> Result<(metamodelica::Real, Arc<metamodelica::List<i32>>)> {
    let mut criticalPathOut: (metamodelica::Real, Arc<metamodelica::List<i32>>);
    let mut cpCalcTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut calcTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut commTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut criticalPathIdx: i32 = 0;
    let mut commCost: Communication;
    let mut childNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut criticalPathChild: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut criticalPath: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut nodeComps: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut criticalPaths: Arc<metamodelica::List<(metamodelica::Real, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    criticalPathOut = 'mc: {
        let __mc_input = (iNode.clone(), iGraph.clone(), iGraphData.clone(), iHandleCommCosts.clone(), iNodeCriticalPaths.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, TaskGraphMeta { exeCosts: mut exeCosts, inComps: mut inComps, .. }, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut cpCalcTime: metamodelica::Real = cpCalcTime.clone();
            let mut criticalPath: Arc<metamodelica::List<i32>> = criticalPath.clone();
            (cpCalcTime, criticalPath) = iNodeCriticalPaths.clone().borrow()[(iNode.clone()-1) as usize].clone();
            let true = (realGe(cpCalcTime.clone(), metamodelica::OrderedFloat(0.0_f64))) else { bail!("pattern mismatch") };
            Ok((cpCalcTime.clone(), criticalPath.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, TaskGraphMeta { exeCosts: mut exeCosts, inComps: mut inComps, .. }, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut commTime: metamodelica::Real = commTime.clone();
            let mut nodeComps: Arc<metamodelica::List<i32>> = nodeComps.clone();
            let mut criticalPaths: Arc<metamodelica::List<(metamodelica::Real, Arc<metamodelica::List<i32>>)>> = criticalPaths.clone();
            let mut criticalPathChild: Arc<metamodelica::List<i32>> = criticalPathChild.clone();
            let mut cpCalcTime: metamodelica::Real = cpCalcTime.clone();
            let mut criticalPathIdx: i32 = criticalPathIdx.clone();
            let mut criticalPath: Arc<metamodelica::List<i32>> = criticalPath.clone();
            let mut childNodes: Arc<metamodelica::List<i32>> = childNodes.clone();
            let mut calcTime: metamodelica::Real = calcTime.clone();
            let mut commCost: Communication;
            childNodes = iGraph.clone().borrow()[(iNode.clone()-1) as usize].clone();
            let false = (childNodes.clone().is_empty()) else { bail!("pattern mismatch") };
            criticalPaths = List::map4(childNodes.clone(), (std::sync::Arc::new(getCriticalPath1) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, TaskGraphMeta, bool, metamodelica::Array<(metamodelica::Real, Arc<metamodelica::List<i32>>)>) -> Result<(metamodelica::Real, Arc<metamodelica::List<i32>>)> + 'static>), iGraph.clone(), iGraphData.clone(), iHandleCommCosts.clone(), iNodeCriticalPaths.clone());
            criticalPathIdx = getCriticalPath2(criticalPaths.clone(), 1, metamodelica::OrderedFloat(-1.0_f64), -1)?;
            (cpCalcTime, criticalPathChild) = (criticalPaths.clone()).get(criticalPathIdx.clone())?;
            criticalPath = cons(iNode.clone(), criticalPathChild.clone());
            commCost = if (iHandleCommCosts.clone()) {getCommCostBetweenNodes(iNode.clone(), listHead(criticalPathChild.clone())?, iGraphData.clone())?} else {Communication { numberOfVars: 0, integerVars: metamodelica::nil(), floatVars: metamodelica::nil(), booleanVars: metamodelica::nil(), stringVars: metamodelica::nil(), childNode: -1, requiredTime: metamodelica::OrderedFloat(0.0_f64) }};
            nodeComps = inComps.clone().borrow()[(iNode.clone()-1) as usize].clone();
            calcTime = addUpExeCostsForNode(nodeComps.clone(), exeCosts.clone(), metamodelica::OrderedFloat(0.0_f64))?;
            calcTime = (cpCalcTime.clone()) + (calcTime.clone());
            let Communication { requiredTime: __pa0, .. } = (commCost.clone()) else { bail!("pattern mismatch") };
            commTime = __pa0.clone();
            calcTime = (calcTime.clone()) + (commTime.clone());
            {let _arr = iNodeCriticalPaths.clone(); _arr.borrow_mut()[(iNode.clone()-1) as usize] = (calcTime.clone(), criticalPath.clone()); _arr};
            Ok((calcTime.clone(), criticalPath.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, TaskGraphMeta { exeCosts: mut exeCosts, inComps: mut inComps, .. }, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut criticalPath: Arc<metamodelica::List<i32>> = criticalPath.clone();
            let mut nodeComps: Arc<metamodelica::List<i32>> = nodeComps.clone();
            let mut calcTime: metamodelica::Real = calcTime.clone();
            let mut childNodes: Arc<metamodelica::List<i32>> = childNodes.clone();
            childNodes = iGraph.clone().borrow()[(iNode.clone()-1) as usize].clone();
            let true = (childNodes.clone().is_empty()) else { bail!("pattern mismatch") };
            criticalPath = cons(iNode.clone(), metamodelica::nil());
            nodeComps = inComps.clone().borrow()[(iNode.clone()-1) as usize].clone();
            calcTime = addUpExeCostsForNode(nodeComps.clone(), exeCosts.clone(), metamodelica::OrderedFloat(0.0_f64))?;
            {let _arr = iNodeCriticalPaths.clone(); _arr.borrow_mut()[(iNode.clone()-1) as usize] = (calcTime.clone(), criticalPath.clone()); _arr};
            Ok((calcTime.clone(), criticalPath.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            println!("{}", (literal!("HpcOmTaskGraph.getCriticalPath_1 failed\n")).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(criticalPathOut)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getCriticalPath2(mut iCriticalPaths: Arc<metamodelica::List<(metamodelica::Real, Arc<metamodelica::List<i32>>)>>, mut iListIdx: i32, mut iLongestPath: metamodelica::Real, mut iLongestPathIndex: i32) -> Result<i32> {
    let mut oLongestPathIndex: i32 = 0;
    let mut cpCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut criticalPath: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut rest: Arc<metamodelica::List<(metamodelica::Real, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    oLongestPathIndex = 'mc: {
        let __mc_input = (iCriticalPaths.clone(), iListIdx.clone(), iLongestPath.clone(), iLongestPathIndex.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (cpCost, criticalPath), tail: rest }, _, _, _) => {
                    let true = (realGt(cpCost.clone(), iLongestPath.clone())) else { bail!("pattern mismatch") };
                    Ok(getCriticalPath2(rest.clone(), iListIdx.clone() + 1, cpCost.clone(), iListIdx.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (cpCost, criticalPath), tail: rest }, _, _, _) => {
                    Ok(getCriticalPath2(rest.clone(), iListIdx.clone() + 1, iLongestPath.clone(), iLongestPathIndex.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iLongestPathIndex.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oLongestPathIndex)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn addUpExeCostsForNode(mut iNodeComps: Arc<metamodelica::List<i32>>, mut iExeCosts: metamodelica::Array<(i32, metamodelica::Real)>, mut iExeCost: metamodelica::Real) -> Result<metamodelica::Real> {
    let mut oExeCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut head: i32 = 0;
    let mut rest: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut cost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    oExeCost = (::match_deref::match_deref! { match &((iNodeComps.clone(), iExeCosts.clone(), iExeCost.clone())) {
        (Deref @ metamodelica::List::Cons { head: head, tail: rest }, _, _) => {
            (_, cost) = iExeCosts.clone().borrow()[(head.clone()-1) as usize].clone();
            cost = (cost.clone()) + (iExeCost.clone());
            cost = addUpExeCostsForNode(rest.clone(), iExeCosts.clone(), cost.clone())?;
            cost.clone()
        },
        _ => iExeCost.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oExeCost)
}

fn gatherParallelSets(mut nodeInfo: metamodelica::Array<(i32, metamodelica::Real, i32)>) -> Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> {
    let mut parallelSetsOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut numLevels: i32 = 0;
    numLevels = Array::fold(nodeInfo.clone(), (std::sync::Arc::new(fnptr!(numberOfLevels, (i32, metamodelica::Real, i32), i32)) as std::sync::Arc<dyn ::std::ops::Fn((i32, metamodelica::Real, i32), i32) -> Result<i32> + 'static>), 0);
    parallelSetsOut = List::fold1(List::intRange((nodeInfo.clone().borrow().len() as i32)), (std::sync::Arc::new(gatherParallelSets1) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<(i32, metamodelica::Real, i32)>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> + 'static>), nodeInfo.clone(), List::fill(metamodelica::nil(), numLevels.clone()));
    parallelSetsOut
}

fn numberOfLevels(mut nodeInfoEntry: (i32, metamodelica::Real, i32), mut numLevelsIn: i32) -> i32 {
    let mut numLevelsOut: i32 = 0;
    let mut levelIn: i32 = 0;
    (levelIn, _, _) = nodeInfoEntry.clone();
    numLevelsOut = intMax(levelIn.clone(), numLevelsIn.clone());
    numLevelsOut
}

fn gatherParallelSets1(mut idx: i32, mut nodeInfo: metamodelica::Array<(i32, metamodelica::Real, i32)>, mut parallelSetIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut parallelSetOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut level: i32 = 0;
    let mut pSet: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (level, _, _) = nodeInfo.clone().borrow()[(idx.clone()-1) as usize].clone();
    pSet = (parallelSetIn.clone()).get(level.clone())?;
    pSet = cons(idx.clone(), pSet.clone());
    parallelSetOut = List::replaceAt(pSet.clone(), level.clone(), parallelSetIn.clone())?;
    Ok(parallelSetOut)
}

fn getCostsForNode(mut parentNode: i32, mut childNode: i32, mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>, mut commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>) -> Result<metamodelica::Real> {
    let mut costsOut: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    costsOut = 'mc: {
        let __mc_input = (parentNode.clone(), childNode.clone(), inComps.clone(), exeCosts.clone(), commCosts.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (0, _, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut costs: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut primalChild: i32 = 0;
            let mut primalChildLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            primalChildLst = inComps.clone().borrow()[(childNode.clone()-1) as usize].clone();
            let true = ((primalChildLst.clone().len() as i32) == 1) else { bail!("pattern mismatch") };
            primalChild = (primalChildLst.clone()).get(1)?;
            (_, costs) = exeCosts.clone().borrow()[(primalChild.clone()-1) as usize].clone();
            Ok(costs.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (0, _, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut costs: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut primalChildLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            primalChildLst = inComps.clone().borrow()[(childNode.clone()-1) as usize].clone();
            let true = ((primalChildLst.clone().len() as i32) > 1) else { bail!("pattern mismatch") };
            let _ = (primalChildLst.clone()).get(1)?;
            costs = getCostsForContractedNodes(primalChildLst.clone(), exeCosts.clone());
            Ok(costs.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut costs: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut commCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut primalChild: i32 = 0;
            let mut primalParent: i32 = 0;
            let mut primalChildLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut primalParentLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            primalChildLst = inComps.clone().borrow()[(childNode.clone()-1) as usize].clone();
            primalParentLst = inComps.clone().borrow()[(parentNode.clone()-1) as usize].clone();
            let true = ((primalChildLst.clone().len() as i32) == 1) else { bail!("pattern mismatch") };
            primalChild = (primalChildLst.clone()).get(1)?;
            primalParent = (primalParentLst.clone()).get(1)?;
            (_, costs) = exeCosts.clone().borrow()[(primalChild.clone()-1) as usize].clone();
            let Communication { requiredTime: __pa0, .. } = (getCommunicationCost(primalChild.clone(), primalParent.clone(), commCosts.clone())?) else { bail!("pattern mismatch") };
            commCost = __pa0.clone();
            costs = costs.clone() + commCost.clone();
            Ok(costs.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut costs: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut primalChildLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            primalChildLst = inComps.clone().borrow()[(childNode.clone()-1) as usize].clone();
            let _ = inComps.clone().borrow()[(parentNode.clone()-1) as usize].clone();
            let true = ((primalChildLst.clone().len() as i32) > 1) else { bail!("pattern mismatch") };
            costs = getCostsForContractedNodes(primalChildLst.clone(), exeCosts.clone());
            Ok(costs.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            println!("{}", (literal!("getCostsForNode failed! \n")).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(costsOut)
}

pub fn getCostsForContractedNodes(mut nodeList: Arc<metamodelica::List<i32>>, mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>) -> metamodelica::Real {
    let mut costsOut: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    costsOut = List::fold1(nodeList.clone(), (std::sync::Arc::new(getCostsForContractedNodes1) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<(i32, metamodelica::Real)>, metamodelica::Real) -> Result<metamodelica::Real> + 'static>), exeCosts.clone(), metamodelica::OrderedFloat(0.0_f64));
    costsOut
}

fn getCostsForContractedNodes1(mut node: i32, mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>, mut costsIn: metamodelica::Real) -> Result<metamodelica::Real> {
    let mut costsOut: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut exeCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    (_, exeCost) = exeCosts.clone().borrow()[(node.clone()-1) as usize].clone();
    costsOut = (costsIn.clone()) + (exeCost.clone());
    Ok(costsOut)
}

fn getNodeCoords(mut parallelSets: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut graphIn: TaskGraph) -> metamodelica::Array<(i32, i32)> {
    let mut nodeCoordsOut: metamodelica::Array<(i32, i32)>;
    let mut nodeCoords: metamodelica::Array<(i32, i32)>;
    let mut size: i32 = 0;
    size = (graphIn.clone().borrow().len() as i32);
    nodeCoords = arrayCreate(size.clone(), (0, 0));
    nodeCoords = List::fold1(List::intRange(size.clone()), (std::sync::Arc::new(getYCoordForNode) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, metamodelica::Array<(i32, i32)>) -> Result<metamodelica::Array<(i32, i32)>> + 'static>), parallelSets.clone(), nodeCoords.clone());
    nodeCoordsOut = nodeCoords.clone();
    nodeCoordsOut
}

fn getYCoordForNode(mut compIdx: i32, mut parallelSets: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut nodeCoordsIn: metamodelica::Array<(i32, i32)>) -> Result<metamodelica::Array<(i32, i32)>> {
    let mut nodeCoordsOut: metamodelica::Array<(i32, i32)>;
    let mut parallelSetIdx: i32 = 0;
    let mut xCoord: i32 = 0;
    let mut yCoord: i32 = 0;
    let mut levelInterval: i32 = 0;
    let mut coords: (i32, i32);
    parallelSetIdx = getParallelSetForComp(compIdx.clone(), 1, parallelSets.clone())?;
    (xCoord, yCoord) = nodeCoordsIn.clone().borrow()[(compIdx.clone()-1) as usize].clone();
    coords = (xCoord.clone(), parallelSetIdx.clone());
    nodeCoordsOut = {let _arr = nodeCoordsIn.clone(); _arr.borrow_mut()[(compIdx.clone()-1) as usize] = coords.clone(); _arr};
    Ok(nodeCoordsOut)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getParallelSetForComp(mut compIn: i32, mut setIdx: i32, mut parallelSets: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<i32> {
    let mut parallelSetOut: i32 = 0;
    parallelSetOut = 'mc: {
        let __mc_input = (compIn.clone(), setIdx.clone(), parallelSets.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut parallelSet: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let true = (setIdx.clone() <= (parallelSets.clone().len() as i32)) else { bail!("pattern mismatch") };
                    parallelSet = (parallelSets.clone()).get(setIdx.clone())?;
                    let true = (List::isMemberOnTrue(compIn.clone(), parallelSet.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
                    Ok(setIdx.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut parallelSet: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut parallelSetTmp: i32 = 0;
                    let true = (setIdx.clone() <= (parallelSets.clone().len() as i32)) else { bail!("pattern mismatch") };
                    parallelSet = (parallelSets.clone()).get(setIdx.clone())?;
                    let false = (List::isMemberOnTrue(compIn.clone(), parallelSet.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
                    parallelSetTmp = getParallelSetForComp(compIn.clone(), setIdx.clone() + 1, parallelSets.clone())?;
                    Ok(parallelSetTmp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("getParallelSetForComp failed!\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(parallelSetOut)
}

fn setLevelInNodeMark(mut nodeIdx: i32, mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut nodeCoords: metamodelica::Array<(i32, i32)>, mut nodeMarkIn: metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> {
    let mut nodeMarkOut: metamodelica::Array<i32>;
    nodeMarkOut = 'mc: {
        let __mc_input = (nodeIdx.clone(), inComps.clone(), nodeCoords.clone(), nodeMarkIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut components: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut primalComp: i32 = 0;
            let mut nodeMarkEntry: i32 = 0;
            nodeMarkEntry = nodeMarkIn.clone().borrow()[(nodeIdx.clone()-1) as usize].clone();
            components = inComps.clone().borrow()[(nodeIdx.clone()-1) as usize].clone();
            primalComp = List::last(components.clone())?;
            nodeMarkEntry = nodeMarkIn.clone().borrow()[(primalComp.clone()-1) as usize].clone();
            let true = (intEq(-1, nodeMarkEntry.clone())) else { bail!("pattern mismatch") };
            Ok(nodeMarkIn.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut nodeMarkTmp: metamodelica::Array<i32>;
            let mut components: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut primalComp: i32 = 0;
            let mut nodeMarkEntry: i32 = 0;
            let mut yCoord: i32 = 0;
            nodeMarkEntry = nodeMarkIn.clone().borrow()[(nodeIdx.clone()-1) as usize].clone();
            components = inComps.clone().borrow()[(nodeIdx.clone()-1) as usize].clone();
            primalComp = List::last(components.clone())?;
            nodeMarkEntry = nodeMarkIn.clone().borrow()[(primalComp.clone()-1) as usize].clone();
            let false = (intEq(-1, nodeMarkEntry.clone())) else { bail!("pattern mismatch") };
            (_, yCoord) = nodeCoords.clone().borrow()[(nodeIdx.clone()-1) as usize].clone();
            nodeMarkTmp = {let _arr = nodeMarkIn.clone(); _arr.borrow_mut()[(primalComp.clone()-1) as usize] = yCoord.clone(); _arr};
            Ok(nodeMarkTmp.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(nodeMarkOut)
}

fn tupleToStringIntRealInt(mut inTuple: (i32, metamodelica::Real, i32)) -> ArcStr {
    let mut result: ArcStr = arcstr::literal!("");
    result = ((match inTuple.clone() {
        (mut int1, mut real1, mut int2) => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(int1.clone())); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*realString(real1.clone())); __mm_s.push_str(&*literal!(" , ")); __mm_s.push_str(&*intString(int2.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
    })).clone();
    result
}

pub fn transposeCommCosts(mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>) -> metamodelica::Array<Arc<metamodelica::List<Communication>>> {
    let mut oCommCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut tmpCommCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    tmpCommCosts = arrayCreate((iCommCosts.clone().borrow().len() as i32), metamodelica::nil());
    (_, tmpCommCosts) = Array::fold(iCommCosts.clone(), (std::sync::Arc::new(fnptr!(transposeCommCosts0, Arc<metamodelica::List<Communication>>, (i32, metamodelica::Array<Arc<metamodelica::List<Communication>>>))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Communication>>, (i32, metamodelica::Array<Arc<metamodelica::List<Communication>>>)) -> Result<(i32, metamodelica::Array<Arc<metamodelica::List<Communication>>>)> + 'static>), (1, tmpCommCosts.clone()));
    oCommCosts = tmpCommCosts.clone();
    oCommCosts
}

fn transposeCommCosts0(mut iCosts: Communications, mut iCommCosts: (i32, metamodelica::Array<Arc<metamodelica::List<Communication>>>)) -> (i32, metamodelica::Array<Arc<metamodelica::List<Communication>>>) {
    let mut oCommCosts: (i32, metamodelica::Array<Arc<metamodelica::List<Communication>>>);
    let mut iParentCompIdx: i32 = 0;
    let mut tmpCommCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    (iParentCompIdx, tmpCommCosts) = iCommCosts.clone();
    tmpCommCosts = List::fold1(iCosts.clone(), (std::sync::Arc::new(transposeCommCosts1) as std::sync::Arc<dyn ::std::ops::Fn(Communication, i32, metamodelica::Array<Arc<metamodelica::List<Communication>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<Communication>>>> + 'static>), iParentCompIdx.clone(), tmpCommCosts.clone());
    oCommCosts = (iParentCompIdx.clone() + 1, tmpCommCosts.clone());
    oCommCosts
}

fn transposeCommCosts1(mut iCost: Communication, mut iParentCompIdx: i32, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<Communication>>>> {
    let mut oCommCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut tmpCommCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut costs: Communications = metamodelica::nil();
    let mut numberOfVars: i32 = 0;
    let mut nodeIdx: i32 = 0;
    let mut integerVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut floatVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut booleanVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut stringVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut requiredTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    oCommCosts = 'mc: {
        let __mc_input = (iCost.clone(), iParentCompIdx.clone(), iCommCosts.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (Communication { requiredTime: mut requiredTime, childNode: mut nodeIdx, stringVars: mut stringVars, booleanVars: mut booleanVars, floatVars: mut floatVars, integerVars: mut integerVars, numberOfVars: mut numberOfVars }, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut costs: Arc<metamodelica::List<Communication>> = costs.clone();
            let mut tmpCommCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
            let true = (intLe(nodeIdx.clone(), (iCommCosts.clone().borrow().len() as i32))) else { bail!("pattern mismatch") };
            costs = iCommCosts.clone().borrow()[(nodeIdx.clone()-1) as usize].clone();
            costs = cons(Communication { numberOfVars: numberOfVars.clone(), integerVars: integerVars.clone(), floatVars: floatVars.clone(), booleanVars: booleanVars.clone(), stringVars: stringVars.clone(), childNode: iParentCompIdx.clone(), requiredTime: requiredTime.clone() }, costs.clone());
            tmpCommCosts = {let _arr = iCommCosts.clone(); _arr.borrow_mut()[(nodeIdx.clone()-1) as usize] = costs.clone(); _arr};
            Ok(tmpCommCosts.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(iCommCosts.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oCommCosts)
}

//TODO: Can this be merged with getCommCostBetweenNodes?
fn getCommunicationCost(mut childIdx: i32, mut parentIdx: i32, mut commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>) -> Result<Communication> {
    let mut oComm: Communication;
    let mut commRow: Communications = metamodelica::nil();
    let mut commEntry: Communication;
    commRow = commCosts.clone().borrow()[(parentIdx.clone()-1) as usize].clone();
    commEntry = getCommunicationByChildIdx(commRow.clone(), childIdx.clone())?;
    oComm = commEntry.clone();
    Ok(oComm)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getCommunicationByChildIdx(mut iComms: Communications, mut iChildIdx: i32) -> Result<Communication> {
    let mut oComm: Communication;
    oComm = 'mc: {
        let __mc_input = (iComms.clone(), iChildIdx.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Communication { childNode: currentCommChild, .. }, tail: rest }, _) => {
                    let mut tmpComm: Communication;
                    let false = (intEq(currentCommChild.clone(), iChildIdx.clone())) else { bail!("pattern mismatch") };
                    tmpComm = getCommunicationByChildIdx(rest.clone(), iChildIdx.clone())?;
                    Ok(tmpComm.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: head @ Communication { childNode: currentCommChild, .. }, tail: _ }, _) => {
                    let true = (intEq(currentCommChild.clone(), iChildIdx.clone())) else { bail!("pattern mismatch") };
                    Ok(head.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getCommunicationByChildIdx failed! - the child idx ")); __mm_s.push_str(&*intString(iChildIdx.clone())); __mm_s.push_str(&*literal!(" can not be found in the list of edges\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oComm)
}

pub fn getCommCostTimeBetweenNodes(mut iParentNodeIdx: i32, mut iChildNodeIdx: i32, mut iTaskGraphMeta: TaskGraphMeta) -> Result<metamodelica::Real> {
    let mut oCommCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut requiredTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let Communication { requiredTime: __pa0, .. } = (getCommCostBetweenNodes(iParentNodeIdx.clone(), iChildNodeIdx.clone(), iTaskGraphMeta.clone())?) else { bail!("pattern mismatch") };
    requiredTime = __pa0.clone();
    oCommCost = requiredTime.clone();
    Ok(oCommCost)
}

fn getCommCostBetweenNodes(mut iParentNodeIdx: i32, mut iChildNodeIdx: i32, mut iTaskGraphMeta: TaskGraphMeta) -> Result<Communication> {
    let mut oCommCost: Communication;
    let mut childComps: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut parentComps: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut concreteCommCostsOpt: Arc<metamodelica::List<Option<Communication>>> = metamodelica::nil();
    let mut concreteCommCosts: Communications = metamodelica::nil();
    let TaskGraphMeta { commCosts: __pa0, inComps: __pa1, .. } = (iTaskGraphMeta.clone()) else { bail!("pattern mismatch") };
    commCosts = __pa0.clone();
    inComps = __pa1.clone();
    parentComps = inComps.clone().borrow()[(iParentNodeIdx.clone()-1) as usize].clone();
    childComps = inComps.clone().borrow()[(iChildNodeIdx.clone()-1) as usize].clone();
    concreteCommCostsOpt = List::map2(parentComps.clone(), (std::sync::Arc::new(getCommCostBetweenNodes0) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<Communication>>>) -> Result<Option<Communication>> + 'static>), childComps.clone(), commCosts.clone());
    concreteCommCosts = {
        let mut __acc: Arc<metamodelica::List<Communication>> = metamodelica::nil();
        for mut c in (concreteCommCostsOpt.clone()).into_iter().cloned() {
            if !(isSome(c.clone())) { continue; }
            let __x = Util::getOption(c.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    oCommCost = getHighestCommCost(concreteCommCosts.clone(), Communication { numberOfVars: 0, integerVars: metamodelica::nil(), floatVars: metamodelica::nil(), booleanVars: metamodelica::nil(), stringVars: metamodelica::nil(), childNode: -1, requiredTime: metamodelica::OrderedFloat(-1.0_f64) })?;
    Ok(oCommCost)
}

fn getCommCostBetweenNodes0(mut iParentComp: i32, mut iChildComps: Arc<metamodelica::List<i32>>, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>) -> Result<Option<Communication>> {
    let mut oHighestComm: Option<Communication> = None;
    let mut commCosts: Communications = metamodelica::nil();
    let mut filteredCommCosts: Communications = metamodelica::nil();
    let mut highestCommCost: Communication;
    oHighestComm = 'mc: {
        let __mc_input = (iParentComp.clone(), iChildComps.clone(), iCommCosts.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut commCosts: Arc<metamodelica::List<Communication>> = commCosts.clone();
                    let mut highestCommCost: Communication;
                    let mut filteredCommCosts: Arc<metamodelica::List<Communication>> = filteredCommCosts.clone();
                    commCosts = iCommCosts.clone().borrow()[(iParentComp.clone()-1) as usize].clone();
                    filteredCommCosts = List::filter1OnTrue(commCosts.clone(), (std::sync::Arc::new(getCommCostBetweenNodes1) as std::sync::Arc<dyn ::std::ops::Fn(Communication, Arc<metamodelica::List<i32>>) -> Result<bool> + 'static>), iChildComps.clone());
                    let false = (filteredCommCosts.clone().is_empty()) else { bail!("pattern mismatch") };
                    highestCommCost = getHighestCommCost(filteredCommCosts.clone(), Communication { numberOfVars: 0, integerVars: metamodelica::nil(), floatVars: metamodelica::nil(), booleanVars: metamodelica::nil(), stringVars: metamodelica::nil(), childNode: -1, requiredTime: metamodelica::OrderedFloat(-1.0_f64) })?;
                    Ok(Some(highestCommCost.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(None)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oHighestComm)
}

fn getCommCostBetweenNodes1(mut iCommCost: Communication, mut iChildComps: Arc<metamodelica::List<i32>>) -> Result<bool> {
    let mut oResult: bool = false;
    let mut compIdx: i32 = 0;
    let Communication { childNode: __pa0, .. } = (iCommCost.clone()) else { bail!("pattern mismatch") };
    compIdx = __pa0.clone();
    oResult = List::exist1(iChildComps.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), compIdx.clone());
    Ok(oResult)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getHighestCommCost(mut iCommCosts: Communications, mut iHighestTuple: Communication) -> Result<Communication> {
    let mut oHighestTuple: Communication;
    let mut highestCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut currentCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut head: Communication;
    let mut rest: Communications = metamodelica::nil();
    oHighestTuple = 'mc: {
        let __mc_input = (iCommCosts.clone(), iHighestTuple.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: head @ Communication { requiredTime: currentCost, .. }, tail: rest }, Communication { requiredTime: highestCost, .. }) => {
                    let true = (realGt(currentCost.clone(), highestCost.clone())) else { bail!("pattern mismatch") };
                    Ok(getHighestCommCost(rest.clone(), head.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: head, tail: rest }, _) => {
                    Ok(getHighestCommCost(rest.clone(), iHighestTuple.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iHighestTuple.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oHighestTuple)
}

pub fn sumUpExeCosts(mut iGraph: TaskGraph, mut iMeta: TaskGraphMeta) -> (i32, metamodelica::Real) {
    let mut execCosts: (i32, metamodelica::Real);
    let mut cost1: i32 = 0;
    let mut cost2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut comps: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut costs: (i32, metamodelica::Real);
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut exeCostLst: Arc<metamodelica::List<(i32, metamodelica::Real)>> = metamodelica::nil();
    execCosts = (match (iGraph.clone(), iMeta.clone()) {
        (_, TaskGraphMeta { exeCosts: mut exeCosts, inComps: mut inComps, .. }) => {
            comps = List::flatten(List::map1(List::intRange((iGraph.clone().borrow().len() as i32)), std::sync::Arc::new(fnptr!(Array::getIndexFirst, i32, _)), inComps.clone()));
            exeCostLst = List::map1(comps.clone(), std::sync::Arc::new(fnptr!(Array::getIndexFirst, i32, _)), exeCosts.clone());
            cost1 = List::fold(List::map(exeCostLst.clone(), std::sync::Arc::new(fnptr!(Util::tuple21, _))), (std::sync::Arc::new(fnptr!(intAdd, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), 0);
            cost2 = List::fold(List::map(exeCostLst.clone(), std::sync::Arc::new(fnptr!(Util::tuple22, _))), (std::sync::Arc::new(fnptr!(realAdd, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<metamodelica::Real> + 'static>), metamodelica::OrderedFloat(0.0_f64));
            (cost1.clone(), cost2.clone())
        },
        _ => (0, metamodelica::OrderedFloat(0.0_f64)),
    });
    execCosts
}

pub fn getAllSCCsOfGraph(mut iTaskGraphMeta: TaskGraphMeta) -> Result<Arc<metamodelica::List<i32>>> {
    let mut oSccs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut taskIdx: i32 = 0;
    let mut mark: i32 = 0;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut comps: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut nodeMark: metamodelica::Array<i32>;
    let mut tmpSccs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    tmpSccs = metamodelica::nil();
    let TaskGraphMeta { nodeMark: __pa0, inComps: __pa1, .. } = (iTaskGraphMeta.clone()) else { bail!("pattern mismatch") };
    nodeMark = __pa0.clone();
    inComps = __pa1.clone();
    let __range2 = 1..=(inComps.clone().borrow().len() as i32);
    for mut taskIdx in __range2 {
        comps = inComps.clone().borrow()[(taskIdx.clone()-1) as usize].clone();
        tmpSccs = List::append_reverse(comps.clone(), tmpSccs.clone());
    }
    oSccs = tmpSccs.clone().reverse();
    Ok(oSccs)
}

//TODO: Remove
pub fn roundReal(mut inReal: metamodelica::Real, mut nIn: i32) -> metamodelica::Real {
    let mut outReal: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut int: i32 = 0;
    let mut real: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    real = inReal.clone() * (metamodelica::OrderedFloat(10.0_f64)).powf(metamodelica::OrderedFloat((nIn.clone()) as f64));
    real = (real.clone()).floor();
    outReal = real.clone() / (metamodelica::OrderedFloat(10.0_f64)).powf(metamodelica::OrderedFloat((nIn.clone()) as f64));
    outReal
}

//--------------------------------------------------------
//  Get annotations from backendDAE and display in graphML
//--------------------------------------------------------
fn setAnnotationsForTasks(mut taskGraphInfo: TaskGraphMeta, mut backendDAE: Arc<BackendDAE::BackendDAE>, mut annotInfoIn: metamodelica::Array<ArcStr>) -> Result<metamodelica::Array<ArcStr>> {
    let mut annotInfoOut: metamodelica::Array<ArcStr>;
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(backendDAE.clone()) {
        Deref @ DAE { eqs: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    systs = __pa0.clone();
    (_, annotInfoOut) = List::fold1(systs.clone(), (std::sync::Arc::new(setAnnotationsForTasks1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, TaskGraphMeta, (i32, metamodelica::Array<ArcStr>)) -> Result<(i32, metamodelica::Array<ArcStr>)> + 'static>), taskGraphInfo.clone(), (0, annotInfoIn.clone()));
    Ok(annotInfoOut)
}

fn setAnnotationsForTasks1(mut syst: Arc<BackendDAE::EqSystem>, mut taskGraphInfo: TaskGraphMeta, mut infoIn: (i32, metamodelica::Array<ArcStr>)) -> Result<(i32, metamodelica::Array<ArcStr>)> {
    let mut infoOut: (i32, metamodelica::Array<ArcStr>);
    let mut idx: i32 = 0;
    let mut annots: metamodelica::Array<ArcStr>;
    let mut vars: BackendDAE::Variables;
    let mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    (idx, annots) = infoIn.clone();
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(syst.clone()) {
        Deref @ BackendDAE::EqSystem { orderedEqs: __pa0, orderedVars: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    eqs = __pa0.clone();
    vars = __pa1.clone();
    annots = List::fold3(List::intRange(BackendVariable::varsSize(vars.clone())?), (std::sync::Arc::new(setAnnotationsForVar) as std::sync::Arc<dyn ::std::ops::Fn(i32, BackendDAE::Variables, TaskGraphMeta, i32, metamodelica::Array<ArcStr>) -> Result<metamodelica::Array<ArcStr>> + 'static>), vars.clone(), taskGraphInfo.clone(), idx.clone(), annots.clone());
    infoOut = (BackendVariable::varsSize(vars.clone())? + idx.clone(), annots.clone());
    Ok(infoOut)
}

fn setAnnotationsForVar(mut backendVarIdx: i32, mut vars: BackendDAE::Variables, mut taskGraphInfo: TaskGraphMeta, mut eqSysOffset: i32, mut annotInfoIn: metamodelica::Array<ArcStr>) -> Result<metamodelica::Array<ArcStr>> {
    let mut annotInfoOut: metamodelica::Array<ArcStr>;
    annotInfoOut = 'mc: {
        let __mc_input = (backendVarIdx.clone(), vars.clone(), taskGraphInfo.clone(), eqSysOffset.clone(), annotInfoIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, TaskGraphMeta { nodeMark: mut nodeMark, varCompMapping: mut varCompMapping, inComps: mut inComps, .. }, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut compIdx: i32 = 0;
            let mut taskIdx: i32 = 0;
            let mut annotString: ArcStr = arcstr::literal!("");
            let mut var: BackendDAE::Var;
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut annot: Option<Arc<SCode::Comment>> = None;
            var = BackendVariable::getVarAt(vars.clone(), backendVarIdx.clone())?;
            BackendDump::printVar(var.clone())?;
            let true = (BackendVariable::hasAnnotation(var.clone())) else { bail!("pattern mismatch") };
            (compIdx, _, _) = varCompMapping.clone().borrow()[(backendVarIdx.clone() + eqSysOffset.clone()-1) as usize].clone();
            taskIdx = getCompInComps(compIdx.clone(), 1, inComps.clone(), nodeMark.clone())?;
            annot = BackendVariable::getAnnotationComment(var.clone())?;
            annotString = (annotInfoIn.clone().borrow()[(taskIdx.clone()-1) as usize].clone()).clone();
            cr = BackendVariable::varCref(var.clone())?;
            annotString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*annotString.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr.clone())?); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*DAEDumpTypes::dumpCommentAnnotationStr(annot.clone())?); __mm_s.push_str(&*literal!(") ")); ArcStr::from(__mm_s) }).clone();
            {let _arr = annotInfoIn.clone(); _arr.borrow_mut()[(taskIdx.clone()-1) as usize] = (annotString.clone()).clone(); _arr};
            Ok(annotInfoIn.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(annotInfoIn.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(annotInfoOut)
}

//--------------------------------------------------------
//  Append removed equations like asserts to the DAE graph
//--------------------------------------------------------
pub fn appendRemovedEquations(mut dae: Arc<BackendDAE::BackendDAE>, mut graphIn: TaskGraph, mut graphDataIn: TaskGraphMeta) -> Result<(TaskGraph, TaskGraphMeta)> {
    let mut graphOut: TaskGraph;
    let mut graphDataOut: TaskGraphMeta;
    (graphOut, graphDataOut) = 'mc: {
        let __mc_input = (dae.clone(), graphIn.clone(), graphDataIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut numNewComps: i32 = 0;
                    let mut newComps: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut nodeVarLst: Arc<metamodelica::List<Arc<metamodelica::List<(i32, i32)>>>> = metamodelica::nil();
                    let mut varCompMap: metamodelica::Array<(i32, i32, i32)>;
                    let mut graph: TaskGraph;
                    let mut graphData: TaskGraphMeta;
                    let mut remEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut shared: Arc<BackendDAE::Shared>;
                    let mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut crefsLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>> = metamodelica::nil();
                    let mut inComps1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut inComps2: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut varCompMapping1: metamodelica::Array<(i32, i32, i32)>;
                    let mut eqCompMapping1: metamodelica::Array<(i32, i32, i32)>;
                    let mut compParamMapping1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut compNames1: metamodelica::Array<ArcStr>;
                    let mut compNames2: metamodelica::Array<ArcStr>;
                    let mut compDescs1: metamodelica::Array<ArcStr>;
                    let mut compDescs2: metamodelica::Array<ArcStr>;
                    let mut exeCosts1: metamodelica::Array<(i32, metamodelica::Real)>;
                    let mut exeCosts2: metamodelica::Array<(i32, metamodelica::Real)>;
                    let mut commCosts1: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
                    let mut nodeMark1: metamodelica::Array<i32>;
                    let mut nodeMark2: metamodelica::Array<i32>;
                    let mut compInformations1: metamodelica::Array<ComponentInfo>;
                    let mut compInformations2: metamodelica::Array<ComponentInfo>;
                    let __pa0 = ::match_deref::match_deref! { match &(dae.clone()) {
                        Deref @ DAE { shared: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    shared = __pa0.clone();
                    remEqs = BackendDAEUtil::collapseRemovedEqs(dae.clone())?;
                    let TaskGraphMeta { varCompMapping: __pa1, .. } = (graphDataIn.clone()) else { bail!("pattern mismatch") };
                    varCompMap = __pa1.clone();
                    eqLst = BackendEquation::equationList(remEqs.clone());
                    numNewComps = (eqLst.clone().len() as i32);
                    let true = (intNe(numNewComps.clone(), 0)) else { bail!("pattern mismatch") };
                    crefsLst = List::map(eqLst.clone(), (std::sync::Arc::new(BackendEquation::equationCrefs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> + 'static>));
                    nodeVarLst = List::map2(crefsLst.clone(), (std::sync::Arc::new(fnptr!(getNodeForCrefLst, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<BackendDAE::BackendDAE>, metamodelica::Array<(i32, i32, i32)>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<BackendDAE::BackendDAE>, metamodelica::Array<(i32, i32, i32)>) -> Result<Arc<metamodelica::List<(i32, i32)>>> + 'static>), dae.clone(), varCompMap.clone());
                    let TaskGraphMeta { compInformations: __pa2, nodeMark: __pa3, commCosts: __pa4, exeCosts: __pa5, compDescs: __pa6, compNames: __pa7, compParamMapping: __pa8, eqCompMapping: __pa9, varCompMapping: __pa10, inComps: __pa11 } = (graphDataIn.clone()) else { bail!("pattern mismatch") };
                    compInformations1 = __pa2.clone();
                    nodeMark1 = __pa3.clone();
                    commCosts1 = __pa4.clone();
                    exeCosts1 = __pa5.clone();
                    compDescs1 = __pa6.clone();
                    compNames1 = __pa7.clone();
                    compParamMapping1 = __pa8.clone();
                    eqCompMapping1 = __pa9.clone();
                    varCompMapping1 = __pa10.clone();
                    inComps1 = __pa11.clone();
                    graph = arrayAppend(graphIn.clone(), arrayCreate(numNewComps.clone(), metamodelica::nil()))?;
                    newComps = List::intRange2((graphIn.clone().borrow().len() as i32) + 1, (graphIn.clone().borrow().len() as i32) + numNewComps.clone());
                    graph = List::threadFold(nodeVarLst.clone(), newComps.clone(), (std::sync::Arc::new(fnptr!(addEdgesToGraph, Arc<metamodelica::List<(i32, i32)>>, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<(i32, i32)>>, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> + 'static>), graph.clone())?;
                    inComps2 = metamodelica::arrayFromVec(List::map(newComps.clone(), std::sync::Arc::new(fnptr!(List::create, _))).into_iter().cloned().collect());
                    compNames2 = arrayCreate(numNewComps.clone(), (literal!("assert")).clone());
                    compDescs2 = metamodelica::arrayFromVec(List::map(eqLst.clone(), (std::sync::Arc::new(BackendDump::equationString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<ArcStr> + 'static>)).into_iter().cloned().collect());
                    nodeMark2 = arrayCreate(numNewComps.clone(), -2);
                    exeCosts2 = metamodelica::arrayFromVec(List::map1(eqLst.clone(), (std::sync::Arc::new(estimateEquationCosts) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, Arc<BackendDAE::Shared>) -> Result<(i32, metamodelica::Real)> + 'static>), shared.clone()).into_iter().cloned().collect());
                    compInformations2 = arrayCreate(numNewComps.clone(), ComponentInfo { isPartOfODESystem: false, isPartOfZeroFuncSystem: false, isRemovedComponent: true });
                    inComps1 = arrayAppend(inComps1.clone(), inComps2.clone())?;
                    compNames1 = arrayAppend(compNames1.clone(), compNames2.clone())?;
                    compDescs1 = arrayAppend(compDescs1.clone(), compDescs2.clone())?;
                    nodeMark1 = arrayAppend(nodeMark1.clone(), nodeMark2.clone())?;
                    exeCosts1 = arrayAppend(exeCosts1.clone(), exeCosts2.clone())?;
                    compInformations1 = arrayAppend(compInformations1.clone(), compInformations2.clone())?;
                    commCosts1 = List::threadFold1(nodeVarLst.clone(), newComps.clone(), (std::sync::Arc::new(fnptr!(setCommCostsToParent, Arc<metamodelica::List<(i32, i32)>>, i32, metamodelica::Real, metamodelica::Array<Arc<metamodelica::List<Communication>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<(i32, i32)>>, i32, metamodelica::Real, metamodelica::Array<Arc<metamodelica::List<Communication>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<Communication>>>> + 'static>), metamodelica::OrderedFloat(74.0_f64), commCosts1.clone())?;
                    graphData = TaskGraphMeta { inComps: inComps1.clone(), varCompMapping: varCompMapping1.clone(), eqCompMapping: eqCompMapping1.clone(), compParamMapping: compParamMapping1.clone(), compNames: compNames1.clone(), compDescs: compDescs1.clone(), exeCosts: exeCosts1.clone(), commCosts: commCosts1.clone(), nodeMark: nodeMark1.clone(), compInformations: compInformations1.clone() };
                    Ok((graph.clone(), graphData.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((graphIn.clone(), graphDataIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((graphOut, graphDataOut))
}

fn estimateEquationCosts(mut eqIn: Arc<BackendDAE::Equation>, mut sharedIn: Arc<BackendDAE::Shared>) -> Result<(i32, metamodelica::Real)> {
    let mut tplOut: (i32, metamodelica::Real);
    let mut numAdd: i32 = 0;
    let mut numMul: i32 = 0;
    let mut numDiv: i32 = 0;
    let mut numTrig: i32 = 0;
    let mut numRel: i32 = 0;
    let mut numOth: i32 = 0;
    let mut numFuncs: i32 = 0;
    let mut numLog: i32 = 0;
    let mut compInfo: Arc<BackendDAE::CompInfo>;
    let (_, (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7)) = BackendEquation::traverseExpsOfEquation(eqIn.clone(), Arc::new({ let __pe_b1 = sharedIn.clone(); move |__pe_a0, __pe_a2| BackendDAEOptimize::countOperationsExp(__pe_a0, __pe_b1.clone(), __pe_a2) }), (0, 0, 0, 0, 0, 0, 0, 0))?;
    numAdd = __pa0.clone();
    numMul = __pa1.clone();
    numDiv = __pa2.clone();
    numTrig = __pa3.clone();
    numRel = __pa4.clone();
    numLog = __pa5.clone();
    numOth = __pa6.clone();
    numFuncs = __pa7.clone();
    compInfo = Arc::new(BackendDAE::CompInfo::NO_COMP { numAdds: numAdd.clone(), numMul: numMul.clone(), numDiv: numDiv.clone(), numTrig: numTrig.clone(), numRelations: numRel.clone(), numLog: numLog.clone(), numOth: numOth.clone(), funcCalls: numFuncs.clone() });
    tplOut = calculateCosts(compInfo.clone())?;
    Ok(tplOut)
}

fn printNodeVars(mut nodes: Arc<metamodelica::List<(i32, i32)>>) -> ArcStr {
    let mut s: ArcStr = arcstr::literal!("");
    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*stringDelimitList(List::map(nodes.clone(), (std::sync::Arc::new(fnptr!(printNodeVars1, (i32, i32))) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32)) -> Result<ArcStr> + 'static>)), (literal!(" | ")).clone())); ArcStr::from(__mm_s) }).clone();
    s
}

fn printNodeVars1(mut node: (i32, i32)) -> ArcStr {
    let mut s: ArcStr = arcstr::literal!("");
    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(Util::tuple21(node.clone()))); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*intString(Util::tuple22(node.clone()))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
    s
}

fn setCommCostsToParent(mut parents: Arc<metamodelica::List<(i32, i32)>>, mut child: i32, mut reqCycles: metamodelica::Real, mut commCostsIn: metamodelica::Array<Arc<metamodelica::List<Communication>>>) -> metamodelica::Array<Arc<metamodelica::List<Communication>>> {
    let mut commCostsOut: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    commCostsOut = List::fold2(parents.clone(), (std::sync::Arc::new(setCommCosts) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32), i32, metamodelica::Real, metamodelica::Array<Arc<metamodelica::List<Communication>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<Communication>>>> + 'static>), child.clone(), reqCycles.clone(), commCostsIn.clone());
    commCostsOut
}

fn setCommCosts(mut parent: (i32, i32), mut child: i32, mut reqCycles: metamodelica::Real, mut commCostsIn: metamodelica::Array<Arc<metamodelica::List<Communication>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<Communication>>>> {
    let mut commCostsOut: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut row: Communications = metamodelica::nil();
    let mut parentNodeIdx: i32 = 0;
    let mut varIdx: i32 = 0;
    (parentNodeIdx, varIdx) = parent.clone();
    row = commCostsIn.clone().borrow()[(parentNodeIdx.clone()-1) as usize].clone();
    row = List::filter1OnTrue(row.clone(), (std::sync::Arc::new(isCommunicationChildEqualToIdx) as std::sync::Arc<dyn ::std::ops::Fn(Communication, i32) -> Result<bool> + 'static>), child.clone());
    row = cons(Communication { numberOfVars: 1, integerVars: metamodelica::nil(), floatVars: list![varIdx.clone()], booleanVars: metamodelica::nil(), stringVars: metamodelica::nil(), childNode: child.clone(), requiredTime: reqCycles.clone() }, row.clone());
    commCostsOut = {let _arr = commCostsIn.clone(); _arr.borrow_mut()[(parentNodeIdx.clone()-1) as usize] = row.clone(); _arr};
    Ok(commCostsOut)
}

fn isCommunicationChildEqualToIdx(mut iComm: Communication, mut iIdx: i32) -> Result<bool> {
    let mut isEq: bool = false;
    let mut childNode: i32 = 0;
    let Communication { childNode: __pa0, .. } = (iComm.clone()) else { bail!("pattern mismatch") };
    childNode = __pa0.clone();
    isEq = intNe(childNode.clone(), iIdx.clone());
    Ok(isEq)
}

fn addEdgesToGraph(mut parents: Arc<metamodelica::List<(i32, i32)>>, mut child: i32, mut graphIn: TaskGraph) -> TaskGraph {
    let mut graphOut: TaskGraph;
    graphOut = List::fold1(List::map(parents.clone(), std::sync::Arc::new(fnptr!(Util::tuple21, _))), (std::sync::Arc::new(addEdgeToGraph) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> + 'static>), child.clone(), graphIn.clone());
    graphOut
}

fn addEdgeToGraph(mut parent: i32, mut child: i32, mut graphIn: TaskGraph) -> Result<TaskGraph> {
    let mut graphOut: TaskGraph;
    let mut row: Arc<metamodelica::List<i32>> = metamodelica::nil();
    row = graphIn.clone().borrow()[(parent.clone()-1) as usize].clone();
    row = List::unique(cons(child.clone(), row.clone()));
    graphOut = {let _arr = graphIn.clone(); _arr.borrow_mut()[(parent.clone()-1) as usize] = row.clone(); _arr};
    Ok(graphOut)
}

fn getNodeForCrefLst(mut iCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut iDae: Arc<BackendDAE::BackendDAE>, mut iVarCompMap: metamodelica::Array<(i32, i32, i32)>) -> Arc<metamodelica::List<(i32, i32)>> {
    let mut oNodeVarLst: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut tmpNodeVarLst: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    tmpNodeVarLst = List::map2(iCrefs.clone(), (std::sync::Arc::new(getNodeForCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<BackendDAE::BackendDAE>, metamodelica::Array<(i32, i32, i32)>) -> Result<(i32, i32)> + 'static>), iDae.clone(), iVarCompMap.clone());
    oNodeVarLst = List::filterOnTrue(tmpNodeVarLst.clone(), (std::sync::Arc::new(fnptr!(nodeIsDependent, (i32, i32))) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32)) -> Result<bool> + 'static>));
    oNodeVarLst
}

fn nodeIsDependent(mut node: (i32, i32)) -> bool {
    let mut dep: bool = false;
    let mut tpl1: i32 = 0;
    (tpl1, _) = node.clone();
    dep = intNe(tpl1.clone(), -1);
    dep
}

fn getNodeForCref(mut iCref: Arc<DAE::ComponentRef>, mut iDae: Arc<BackendDAE::BackendDAE>, mut iVarCompMapping: metamodelica::Array<(i32, i32, i32)>) -> Result<(i32, i32)> {
    let mut oNodeVarIdx: (i32, i32);
    let mut eqSysIdx: i32 = 0;
    let mut varIdx: i32 = 0;
    let mut nodeIdx: i32 = 0;
    let mut eqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(iDae.clone()) {
        Deref @ DAE { eqs: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    eqSystems = __pa0.clone();
    (eqSysIdx, varIdx, _) = getNodeForCref1(eqSystems.clone(), iCref.clone(), 1)?;
    nodeIdx = getNodeForVarIdx(varIdx.clone(), eqSysIdx.clone(), iVarCompMapping.clone(), varIdx.clone())?;
    oNodeVarIdx = (nodeIdx.clone(), varIdx.clone());
    Ok(oNodeVarIdx)
}

fn getNodeForCref1(mut eqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, mut cref: Arc<DAE::ComponentRef>, mut eqSysIdxIn: i32) -> Result<(i32, i32, bool)> {
    let mut eqSysIdx: i32 = 0;
    let mut varIdx: i32 = 0;
    let mut found: bool = false;
    (eqSysIdx, varIdx, found) = 'mc: {
        let __mc_input = (eqSystems.clone(), cref.clone(), eqSysIdxIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::EqSystem { orderedVars: vars, .. }, tail: _ }, _, _) => {
                    let mut b: bool = false;
                    let mut esIdx: i32 = 0;
                    let mut vIdx: i32 = 0;
                    let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    (varLst, lst) = BackendVariable::getVar(cref.clone(), vars.clone())?;
                    if intNe((lst.clone().len() as i32), 1) {
                        println!("{}", (literal!("Check if there is a assert or something that is dependent of arrayEquations")).clone());
                    }
                    if BackendVariable::isStateVar(listHead(varLst.clone())?) {
                        (esIdx, vIdx, b) = (-1, -1, false);
                    } else {
                        (esIdx, vIdx, b) = (eqSysIdxIn.clone(), listHead(lst.clone())?, true);
                    }
                    Ok((esIdx.clone(), vIdx.clone(), b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::EqSystem { .. }, tail: rest }, _, _) => {
                    let mut b: bool = false;
                    let mut esIdx: i32 = 0;
                    let mut vIdx: i32 = 0;
                    (esIdx, vIdx, b) = getNodeForCref1(rest.clone(), cref.clone(), eqSysIdxIn.clone() + 1)?;
                    Ok((esIdx.clone(), vIdx.clone(), b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok((-1, -1, false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((eqSysIdx, varIdx, found))
}

fn getNodeForVarIdx(mut varIdx: i32, mut eqSysIdx: i32, mut varCompMapping: metamodelica::Array<(i32, i32, i32)>, mut inTryThisIndex: i32) -> Result<i32> {
    let mut node: i32 = 0;
    let mut offset: i32 = 0;
    let mut eqSys: i32 = 0;
    let mut tryThisIndex: i32 = inTryThisIndex.clone();
    let mut n: i32 = 0;
    let mut arrayLengthVarCompMapping: i32 = 0;
    let mut eqSysNeq: bool = false;
    arrayLengthVarCompMapping = (varCompMapping.clone().borrow().len() as i32);
    loop {
        if tryThisIndex.clone() >= 1 && tryThisIndex.clone() <= arrayLengthVarCompMapping.clone() {
            (node, eqSys, offset) = varCompMapping.clone().borrow()[(tryThisIndex.clone()-1) as usize].clone();
            if eqSys.clone() == eqSysIdx.clone() {
                node = node.clone() + varIdx.clone() - 1;
                return Ok(node);
            } else {
                tryThisIndex = offset.clone() + 2;
            }
        } else if varIdx.clone() == -1 && eqSysIdx.clone() == -1 {
            node = -1;
            return Ok(node);
        } else {
            println!("{}", (literal!("HpcOmTaskGraph.getNodeForVarIdx failed\n")).clone());
        }
        n = n.clone() + 1;
        if n.clone() > arrayLengthVarCompMapping.clone() {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("HpcOmTaskGraph.getNodeForVarIdx")); __mm_s.push_str(&*literal!(" failed (there is a loop somewhere)")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail");
        }
    }
    Ok(node)
}

//----------------------------
//  MULTIRATE PARTITIONING
//----------------------------
pub fn multirate_partitioning(mut odeGraph: TaskGraph, mut odeGraphData: TaskGraphMeta, mut backendDAE: Arc<BackendDAE::BackendDAE>, mut simCode: SimCode::SimCode, mut sccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<SimCode::PartitionData> {
    let mut partitionDataOut: SimCode::PartitionData;
    let mut stateTasksArray: metamodelica::Array<i32>;
    let mut stateTaskAssign: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut stateTasks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tasksPerLevel: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut partitions: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut odeGraphT: TaskGraph;
    let mut numPartitions: i32 = 0;
    let mut activatorsForPartitions: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut stateToActivators: Arc<metamodelica::List<i32>> = metamodelica::nil();
    tasksPerLevel = getLevelNodes(odeGraph.clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("tasksPerLevel ")); __mm_s.push_str(&*stringDelimitList(List::map(tasksPerLevel.clone(), (std::sync::Arc::new(fnptr!(intLstString, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    stateTasks = getLeafNodes(odeGraph.clone())?;
    stateTasks = multirate_orderStateTasksInSimVarStateOrder(stateTasks.clone(), odeGraphData.clone(), backendDAE.clone(), simCode.clone())?;
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("stateTasks ")); __mm_s.push_str(&*intLstString(stateTasks.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    odeGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(odeGraph.clone(), (odeGraph.clone().borrow().len() as i32))?;
    stateTaskAssign = multirate_assignTasksToStates(tasksPerLevel.clone(), stateTasks.clone(), odeGraphT.clone())?;
    dumpStateAssign(stateTaskAssign.clone());
    partitions = multirate_getPartitions(stateTaskAssign.clone(), stateTasks.clone(), odeGraphT.clone())?;
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("PARTITIONS :\n")); __mm_s.push_str(&*stringDelimitList(List::map(partitions.clone(), (std::sync::Arc::new(fnptr!(intLstString, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    activatorsForPartitions = List::mapMap(partitions.clone(), Arc::new(listHead.clone()), Arc::new({ let __pe_b1 = stateTaskAssign.clone(); move |__pe_a0| Ok(Array::getIndexFirst(__pe_a0, __pe_b1.clone())) }));
    partitions = List::map1(partitions.clone(), (std::sync::Arc::new(fnptr!(getSimEqsIdxLstForSCCIdxLst, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), sccSimEqMapping.clone());
    numPartitions = (partitions.clone().len() as i32);
    stateToActivators = List::intRange((stateTasks.clone().len() as i32));
    partitionDataOut = SimCode::PartitionData { numPartitions: numPartitions.clone(), partitions: partitions.clone(), activatorsForPartitions: activatorsForPartitions.clone(), stateToActivators: stateToActivators.clone() };
    dumpPartitionData(partitionDataOut.clone())?;
    Ok(partitionDataOut)
}

fn multirate_orderStateTasksInSimVarStateOrder(mut stateTasks: Arc<metamodelica::List<i32>>, mut taskGraphData: TaskGraphMeta, mut dae: Arc<BackendDAE::BackendDAE>, mut simCode: SimCode::SimCode) -> Result<Arc<metamodelica::List<i32>>> {
    let mut orderedTasks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut state: i32 = 0;
    let mut compIdx: i32 = 0;
    let mut eqSysIdx: i32 = 0;
    let mut offset: i32 = 0;
    let mut varIdx: i32 = 0;
    let mut simVarIdx: i32 = 0;
    let mut simVarIdxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut order: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut varMapTpl: (i32, i32, i32);
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut var: BackendDAE::Var;
    let mut eqSys: Arc<BackendDAE::EqSystem>;
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut simVar: SimCodeVar::SimVar;
    let mut eqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(dae.clone()) {
        Deref @ DAE { eqs: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    eqSystems = __pa0.clone();
    simVarIdxs = metamodelica::nil();
    for mut state in &*stateTasks.clone() {
        let mut state = state.clone();
        compIdx = listHead(taskGraphData.inComps.clone().borrow()[(state.clone()-1) as usize].clone())?;
        let (Some((__pa1, __pa2, __pa3)), __pa4) = (Array::findFirstOnTrueWithIdx(taskGraphData.varCompMapping.clone(), Arc::new({ let __pe_b1 = compIdx.clone(); move |__pe_a0| Ok(varMappingTupleCompEqual(__pe_a0, __pe_b1.clone())) }))) else { bail!("pattern mismatch") };
        compIdx = __pa1.clone();
        eqSysIdx = __pa2.clone();
        offset = __pa3.clone();
        varIdx = __pa4.clone();
        eqSys = (eqSystems.clone()).get(eqSysIdx.clone())?;
        varIdx = varIdx.clone() - offset.clone();
        var = BackendVariable::getVarAt(eqSys.orderedVars.clone(), varIdx.clone())?;
        cref = var.varName.clone();
        let __pa5 = ::match_deref::match_deref! { match &(SimCodeUtil::getSimVars2Crefs(list![cref.clone()], simCode.crefToSimVarHT.clone())) {
            Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Nil } => __pa5.clone(),
            _ => bail!("pattern mismatch"),
        } };
        simVar = __pa5.clone();
        simVarIdx = simVar.index.clone();
        simVarIdxs = cons(simVarIdx.clone(), simVarIdxs.clone());
    }
    (_, order) = HpcOmScheduler::quicksortWithOrder(List::map(simVarIdxs.clone().reverse(), (std::sync::Arc::new(fnptr!(intReal, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<metamodelica::Real> + 'static>)))?;
    orderedTasks = List::map1(order.clone(), std::sync::Arc::new(fnptr!(List::getIndexFirst, i32, _)), stateTasks.clone());
    Ok(orderedTasks)
}

fn varMappingTupleCompEqual(mut tpl: (i32, i32, i32), mut compIdx: i32) -> bool {
    let mut compEqual: bool = false;
    compEqual = intEq(compIdx.clone(), Util::tuple31(tpl.clone()));
    compEqual
}

fn getSimEqIdxForSCCIdx(mut sccIdx: i32, mut sccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<i32> {
    let mut simEqIdx: i32 = 0;
    simEqIdx = listHead(sccSimEqMapping.clone().borrow()[(sccIdx.clone()-1) as usize].clone())?;
    Ok(simEqIdx)
}

fn getSimEqsIdxLstForSCCIdxLst(mut sccIdxs: Arc<metamodelica::List<i32>>, mut sccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Arc<metamodelica::List<i32>> {
    let mut simEqIdxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    simEqIdxs = List::map1(sccIdxs.clone(), (std::sync::Arc::new(getSimEqIdxForSCCIdx) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<i32> + 'static>), sccSimEqMapping.clone());
    simEqIdxs
}

fn multirate_getPartitions(mut stateTaskAssign: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut stateTasks: Arc<metamodelica::List<i32>>, mut odeGraphT: TaskGraph) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut partitions: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut task: i32 = 0;
    let mut numStates: i32 = 0;
    let mut numAssigns: i32 = 0;
    let mut leaveNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut predecessors: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut samePartTasks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut partition: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut otherPartTasks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut stateAss: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut visitedTasks: metamodelica::Array<i32>;
    let mut leaveNodesWithNassigns: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    visitedTasks = arrayCreate((odeGraphT.clone().borrow().len() as i32), -1);
    numStates = (stateTasks.clone().len() as i32);
    leaveNodesWithNassigns = arrayCreate(numStates.clone(), metamodelica::nil());
    {let _arr = leaveNodesWithNassigns.clone(); _arr.borrow_mut()[(1-1) as usize] = stateTasks.clone(); _arr};
    for mut numAssigns in &*List::intRange(numStates.clone()) {
        let mut numAssigns = numAssigns.clone();
        leaveNodes = leaveNodesWithNassigns.clone().borrow()[(numAssigns.clone()-1) as usize].clone();
        leaveNodes = List::unique(leaveNodes.clone());
        while !(leaveNodes.clone().is_empty()) {
            stateAss = stateTaskAssign.clone().borrow()[(listHead(leaveNodes.clone())?-1) as usize].clone();
            (samePartTasks, leaveNodes) = List::separateOnTrue(leaveNodes.clone(), Arc::new({ let __pe_b1 = stateTaskAssign.clone(); let __pe_b2 = stateAss.clone(); move |__pe_a0| hasSameStateAssign(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
            (partition, otherPartTasks) = multirate_getPartitionPredecessors(samePartTasks.clone(), odeGraphT.clone(), stateTaskAssign.clone(), stateAss.clone(), visitedTasks.clone())?;
            partition = List::sort(partition.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            multirate_dispatchLeaveNodes(otherPartTasks.clone(), stateTaskAssign.clone(), leaveNodesWithNassigns.clone())?;
            partitions = cons(partition.clone(), partitions.clone());
        }
    }
    Ok(partitions)
}

fn multirate_dispatchLeaveNodes(mut tasksIn: Arc<metamodelica::List<i32>>, mut stateTaskAssign: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut leaveNodesWithNassigns: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    let mut numAss: i32 = 0;
    let mut stateAss: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut leaveNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut task in &*tasksIn.clone() {
        let mut task = task.clone();
        stateAss = stateTaskAssign.clone().borrow()[(task.clone()-1) as usize].clone();
        numAss = (stateAss.clone().len() as i32);
        leaveNodes = leaveNodesWithNassigns.clone().borrow()[(numAss.clone()-1) as usize].clone();
        leaveNodes = cons(task.clone(), leaveNodes.clone());
        let _ = {let _arr = leaveNodesWithNassigns.clone(); _arr.borrow_mut()[(numAss.clone()-1) as usize] = leaveNodes.clone(); _arr};
    }
    Ok(())
}

fn multirate_getPartitionPredecessors(mut leavesIn: Arc<metamodelica::List<i32>>, mut odeGraphT: TaskGraph, mut stateTaskAssign: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut refStateAssign: Arc<metamodelica::List<i32>>, mut visitedTasks: metamodelica::Array<i32>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut partitionTasks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut otherLeaveNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut cont: bool = false;
    let mut task: i32 = 0;
    let mut tasks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut predecessors: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut samePartTasks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut otherLeaves: Arc<metamodelica::List<i32>> = metamodelica::nil();
    cont = true;
    tasks = leavesIn.clone();
    while cont.clone() {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(tasks.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        task = __pa0.clone();
        tasks = __pa1.clone();
        predecessors = odeGraphT.clone().borrow()[(task.clone()-1) as usize].clone();
        predecessors = List::filter1OnTrue(predecessors.clone(), (std::sync::Arc::new(taskIsNotVisited) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>), visitedTasks.clone());
        (samePartTasks, otherLeaves) = List::separateOnTrue(predecessors.clone(), Arc::new({ let __pe_b1 = stateTaskAssign.clone(); let __pe_b2 = refStateAssign.clone(); move |__pe_a0| hasSameStateAssign(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
        partitionTasks = cons(task.clone(), partitionTasks.clone());
        partitionTasks = listAppend(samePartTasks.clone(), partitionTasks.clone());
        tasks = listAppend(samePartTasks.clone(), tasks.clone());
        otherLeaveNodes = listAppend(otherLeaves.clone(), otherLeaveNodes.clone());
        let _ = {let _arr = visitedTasks.clone(); _arr.borrow_mut()[(task.clone()-1) as usize] = 0; _arr};
        List::map2_0(samePartTasks.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), 0, visitedTasks.clone());
        List::map2_0(otherLeaves.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), 0, visitedTasks.clone());
        if tasks.clone().is_empty() {
            cont = false;
        }
    }
    partitionTasks = List::unique(partitionTasks.clone());
    otherLeaveNodes = List::unique(otherLeaveNodes.clone());
    Ok((partitionTasks, otherLeaveNodes))
}

fn taskIsNotVisited(mut task: i32, mut visitedTasks: metamodelica::Array<i32>) -> Result<bool> {
    let mut isNotVisited: bool = false;
    isNotVisited = intEq(-1, visitedTasks.clone().borrow()[(task.clone()-1) as usize].clone());
    Ok(isNotVisited)
}

fn hasSameStateAssign(mut task: i32, mut stateTaskAssign: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut refStateAssign: Arc<metamodelica::List<i32>>) -> Result<bool> {
    let mut sameStateAssign: bool = false;
    sameStateAssign = List::isEqual(stateTaskAssign.clone().borrow()[(task.clone()-1) as usize].clone(), refStateAssign.clone(), true);
    Ok(sameStateAssign)
}

fn multirate_assignTasksToStates(mut tasksPerLevel: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut stateTasks: Arc<metamodelica::List<i32>>, mut odeGraphT: TaskGraph) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut stateTaskAssignOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut taskIdx: i32 = 0;
    let mut assignments: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut predecessors: Arc<metamodelica::List<i32>> = metamodelica::nil();
    stateTaskAssignOut = arrayCreate((odeGraphT.clone().borrow().len() as i32), metamodelica::nil());
    taskIdx = 1;
    for mut task in &*stateTasks.clone() {
        let mut task = task.clone();
        stateTaskAssignOut = {let _arr = stateTaskAssignOut.clone(); _arr.borrow_mut()[(task.clone()-1) as usize] = list![taskIdx.clone()]; _arr};
        taskIdx = taskIdx.clone() + 1;
    }
    for mut levelTasks in &*tasksPerLevel.clone().reverse() {
        let mut levelTasks = levelTasks.clone();
        for mut task in &*levelTasks.clone() {
            let mut task = task.clone();
            assignments = stateTaskAssignOut.clone().borrow()[(task.clone()-1) as usize].clone();
            predecessors = odeGraphT.clone().borrow()[(task.clone()-1) as usize].clone();
            stateTaskAssignOut = List::fold1(predecessors.clone(), (std::sync::Arc::new(appendToElementUnique) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<_> + 'static>), assignments.clone(), stateTaskAssignOut.clone());
        }
    }
    stateTaskAssignOut = Array::map1(stateTaskAssignOut.clone(), (std::sync::Arc::new(List::sort) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    Ok(stateTaskAssignOut)
}

fn appendToElementUnique<T: Clone + 'static + PartialEq>(mut inIndex: i32, mut inElements: Arc<metamodelica::List<T>>, mut inArray: metamodelica::Array<Arc<metamodelica::List<T>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<T>>>> {
    let mut outArray: metamodelica::Array<Arc<metamodelica::List<T>>>;
    outArray = {let _arr = inArray.clone(); let _val = List::unique(listAppend(inArray.borrow()[(inIndex.clone()-1) as usize].clone(), inElements.clone())); _arr.borrow_mut()[(inIndex.clone()-1) as usize] = _val; _arr};
    Ok(outArray)
}

fn dumpStateAssign(mut stateAssign: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> () {
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("stateAssign ")); __mm_s.push_str(&*stringDelimitList(List::mapArray(stateAssign.clone(), (std::sync::Arc::new(fnptr!(intLstString, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    ()
}

fn dumpPartitionData(mut partData: SimCode::PartitionData) -> Result<()> {
    let mut numPartitions: i32 = 0;
    let mut act: i32 = 0;
    let mut part: i32 = 0;
    let mut state: i32 = 0;
    let mut activatorsForPartitions: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut partitions: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut stateToActivators: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let SimCode::PARTITIONDATA { stateToActivators: __pa0, activatorsForPartitions: __pa1, partitions: __pa2, numPartitions: __pa3 } = (partData.clone()) else { bail!("pattern mismatch") };
    stateToActivators = __pa0.clone();
    activatorsForPartitions = __pa1.clone();
    partitions = __pa2.clone();
    numPartitions = __pa3.clone();
    println!("{}", (literal!("Multirate Partition Data\n")).clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(numPartitions.clone())); __mm_s.push_str(&*literal!(" partitions:\n")); ArcStr::from(__mm_s) }).clone());
    act = 1;
    for mut state in &*stateToActivators.clone() {
        let mut state = state.clone();
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("activator ")); __mm_s.push_str(&*intString(act.clone())); __mm_s.push_str(&*literal!(" is state ")); __mm_s.push_str(&*intString(state.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        act = act.clone() + 1;
    }
    println!("{}", (literal!("\n")).clone());
    for mut part in 1..=numPartitions.clone() {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("activators: ")); __mm_s.push_str(&*intLstString((activatorsForPartitions.clone()).get(part.clone())?)); __mm_s.push_str(&*literal!("\t\t\t\tderStateTasks: ")); __mm_s.push_str(&*intLstString(List::map1((activatorsForPartitions.clone()).get(part.clone())?, std::sync::Arc::new(fnptr!(List::getIndexFirst, i32, _)), stateToActivators.clone()))); __mm_s.push_str(&*literal!("\t\t\t\tnodes: \t")); __mm_s.push_str(&*intLstString((partitions.clone()).get(part.clone())?)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(())
}

//----------------------------
//  MAPPING FUNCTIONS
//----------------------------
pub fn setUpHpcOmMapping(mut daeIn: Arc<BackendDAE::BackendDAE>, mut simCodeIn: SimCode::SimCode, mut lastEqMappingIdx: i32, mut equationSccMappingIn: Arc<metamodelica::List<(i32, i32)>>) -> Result<(metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>)> {
    let mut simeqCompMapping: metamodelica::Array<i32>;
    let mut sccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut daeSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut highestSccIdx: i32 = 0;
    let mut compCountPlusDummy: i32 = 0;
    let mut equationSccMapping: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut equationSccMapping1: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut allComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    (allComps, _) = getSystemComponents(daeIn.clone())?;
    highestSccIdx = findHighestSccIdxInMapping(equationSccMappingIn.clone(), -1)?;
    compCountPlusDummy = (allComps.clone().len() as i32) + 1;
    equationSccMapping1 = removeDummyStateFromMapping(equationSccMappingIn.clone());
    equationSccMapping = if (intEq(highestSccIdx.clone(), compCountPlusDummy.clone())) {equationSccMapping1.clone()} else {equationSccMappingIn.clone()};
    sccSimEqMapping = convertToSccSimEqMapping(equationSccMapping.clone(), (allComps.clone().len() as i32));
    simeqCompMapping = convertToSimeqCompMapping(equationSccMapping.clone(), lastEqMappingIdx.clone());
    daeSccSimEqMapping = metamodelica::arrayFromVec(List::map(SimCodeUtil::getRemovedEquationSimEqSysIdxes(simCodeIn.clone())?, std::sync::Arc::new(fnptr!(List::create, _))).into_iter().cloned().collect());
    daeSccSimEqMapping = arrayAppend(sccSimEqMapping.clone(), daeSccSimEqMapping.clone())?;
    Ok((simeqCompMapping, sccSimEqMapping, daeSccSimEqMapping))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn findHighestSccIdxInMapping(mut iEquationSccMapping: Arc<metamodelica::List<(i32, i32)>>, mut iHighestIndex: i32) -> Result<i32> {
    let mut oIndex: i32 = 0;
    let mut eqIdx: i32 = 0;
    let mut sccIdx: i32 = 0;
    let mut rest: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    oIndex = 'mc: {
        let __mc_input = (iEquationSccMapping.clone(), iHighestIndex.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (eqIdx, sccIdx), tail: rest }, _) => {
                    let true = (intGt(sccIdx.clone(), iHighestIndex.clone())) else { bail!("pattern mismatch") };
                    Ok(findHighestSccIdxInMapping(rest.clone(), sccIdx.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (eqIdx, sccIdx), tail: rest }, _) => {
                    Ok(findHighestSccIdxInMapping(rest.clone(), iHighestIndex.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iHighestIndex.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oIndex)
}

fn removeDummyStateFromMapping(mut iEquationSccMapping: Arc<metamodelica::List<(i32, i32)>>) -> Arc<metamodelica::List<(i32, i32)>> {
    let mut oEquationSccMapping: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    oEquationSccMapping = List::fold(iEquationSccMapping.clone(), (std::sync::Arc::new(removeDummyStateFromMapping1) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32), Arc<metamodelica::List<(i32, i32)>>) -> Result<Arc<metamodelica::List<(i32, i32)>>> + 'static>), metamodelica::nil());
    oEquationSccMapping
}

fn removeDummyStateFromMapping1(mut iTuple: (i32, i32), mut iNewList: Arc<metamodelica::List<(i32, i32)>>) -> Result<Arc<metamodelica::List<(i32, i32)>>> {
    let mut oNewList: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut eqIdx: i32 = 0;
    let mut sccIdx: i32 = 0;
    let mut newElem: (i32, i32);
    oNewList = 'mc: {
        let __mc_input = (iTuple.clone(), iNewList.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((eqIdx, sccIdx), _) => {
                    let true = (intEq(sccIdx.clone(), 1)) else { bail!("pattern mismatch") };
                    Ok(iNewList.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((eqIdx, sccIdx), _) => {
                    let mut newElem: (i32, i32);
                    newElem = (eqIdx.clone(), sccIdx.clone() - 1);
                    Ok(cons(newElem.clone(), iNewList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("removeDummyStateFromMapping1 failed\n")).clone());
                    Ok(iNewList.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oNewList)
}

fn convertToSccSimEqMapping(mut iMapping: Arc<metamodelica::List<(i32, i32)>>, mut numOfSccs: i32) -> metamodelica::Array<Arc<metamodelica::List<i32>>> {
    let mut oMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut tmpMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    tmpMapping = arrayCreate(numOfSccs.clone(), metamodelica::nil());
    let _ = List::fold(iMapping.clone(), (std::sync::Arc::new(convertToSccSimEqMapping1) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32), metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> + 'static>), tmpMapping.clone());
    oMapping = tmpMapping.clone();
    oMapping
}

fn convertToSccSimEqMapping1(mut iMapping: (i32, i32), mut iSccMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut oSccMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    let mut tmpList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (i1, i2) = iMapping.clone();
    tmpList = iSccMapping.clone().borrow()[(i2.clone()-1) as usize].clone();
    tmpList = cons(i1.clone(), tmpList.clone());
    oSccMapping = {let _arr = iSccMapping.clone(); _arr.borrow_mut()[(i2.clone()-1) as usize] = tmpList.clone(); _arr};
    Ok(oSccMapping)
}

fn convertToSimeqCompMapping(mut iMapping: Arc<metamodelica::List<(i32, i32)>>, mut numOfSimEqs: i32) -> metamodelica::Array<i32> {
    let mut oMapping: metamodelica::Array<i32>;
    let mut tmpMapping: metamodelica::Array<i32>;
    tmpMapping = arrayCreate(numOfSimEqs.clone(), -1);
    oMapping = List::fold(iMapping.clone(), (std::sync::Arc::new(convertToSimeqCompMapping1) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32), metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> + 'static>), tmpMapping.clone());
    oMapping
}

fn convertToSimeqCompMapping1(mut iSimEqTuple: (i32, i32), mut iMapping: metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> {
    let mut oMapping: metamodelica::Array<i32>;
    let mut simEqIdx: i32 = 0;
    let mut sccIdx: i32 = 0;
    (simEqIdx, sccIdx) = iSimEqTuple.clone();
    oMapping = {let _arr = iMapping.clone(); _arr.borrow_mut()[(simEqIdx.clone()-1) as usize] = sccIdx.clone(); _arr};
    Ok(oMapping)
}

fn getSimEqIdxSimEqMapping(mut iAllEquations: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut iSimEqSystemHighestIdx: i32) -> metamodelica::Array<Option<Arc<SimCode::SimEqSystem>>> {
    let mut oMapping: metamodelica::Array<Option<Arc<SimCode::SimEqSystem>>>;
    let mut tmpMapping: metamodelica::Array<Option<Arc<SimCode::SimEqSystem>>>;
    tmpMapping = arrayCreate(iSimEqSystemHighestIdx.clone(), None);
    oMapping = List::fold(iAllEquations.clone(), (std::sync::Arc::new(getSimEqIdxSimEqMapping1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::SimEqSystem>, metamodelica::Array<Option<Arc<SimCode::SimEqSystem>>>) -> Result<metamodelica::Array<Option<Arc<SimCode::SimEqSystem>>>> + 'static>), tmpMapping.clone());
    oMapping
}

fn getSimEqIdxSimEqMapping1(mut iEquation: Arc<SimCode::SimEqSystem>, mut iMapping: metamodelica::Array<Option<Arc<SimCode::SimEqSystem>>>) -> Result<metamodelica::Array<Option<Arc<SimCode::SimEqSystem>>>> {
    let mut oMapping: metamodelica::Array<Option<Arc<SimCode::SimEqSystem>>>;
    let mut simEqIdx: i32 = 0;
    let mut tmpMapping: metamodelica::Array<Option<Arc<SimCode::SimEqSystem>>>;
    oMapping = 'mc: {
        let __mc_input = (iEquation.clone(), iMapping.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut tmpMapping: metamodelica::Array<Option<Arc<SimCode::SimEqSystem>>>;
                    let mut simEqIdx: i32 = simEqIdx.clone();
                    (simEqIdx, _) = getIndexBySimCodeEq(iEquation.clone())?;
                    tmpMapping = {let _arr = iMapping.clone(); _arr.borrow_mut()[(simEqIdx.clone()-1) as usize] = Some(iEquation.clone()); _arr};
                    Ok(tmpMapping.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut simEqIdx: i32 = simEqIdx.clone();
                    (simEqIdx, _) = getIndexBySimCodeEq(iEquation.clone())?;
                    Ok(iMapping.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oMapping)
}

fn getSimCodeEqByIndexAndMapping(mut iSimEqIdxSimEqMapping: metamodelica::Array<Option<Arc<SimCode::SimEqSystem>>>, mut iIdx: i32) -> Result<Arc<SimCode::SimEqSystem>> {
    let mut oSimEqSystem: Arc<SimCode::SimEqSystem>;
    let mut tmpSimEqSystem: Option<Arc<SimCode::SimEqSystem>> = None;
    tmpSimEqSystem = iSimEqIdxSimEqMapping.clone().borrow()[(iIdx.clone()-1) as usize].clone();
    oSimEqSystem = getSimCodeEqByIndexAndMapping1(tmpSimEqSystem.clone(), iIdx.clone())?;
    Ok(oSimEqSystem)
}

fn getSimCodeEqByIndexAndMapping1(mut iSimEqSystem: Option<Arc<SimCode::SimEqSystem>>, mut iIdx: i32) -> Result<Arc<SimCode::SimEqSystem>> {
    let mut oSimEqSystem: Arc<SimCode::SimEqSystem>;
    let mut tmpSys: Arc<SimCode::SimEqSystem>;
    oSimEqSystem = (::match_deref::match_deref! { match &((iSimEqSystem.clone(), iIdx.clone())) {
        (Some(tmpSys), _) => tmpSys.clone(),
        _ => {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getSimCodeEqByIndexAndMapping1 failed. Looking for Index ")); __mm_s.push_str(&*intString(iIdx.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oSimEqSystem)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getSimCodeEqByIndex(mut iEqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut iIdx: i32) -> Result<Arc<SimCode::SimEqSystem>> {
    let mut oEq: Arc<SimCode::SimEqSystem>;
    let mut rest: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    let mut head: Arc<SimCode::SimEqSystem>;
    let mut headIdx: i32 = 0;
    let mut headIdx2: i32 = 0;
    oEq = 'mc: {
        let __mc_input = (iEqs.clone(), iIdx.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: head, tail: rest }, _) => {
                    let mut headIdx2: i32 = headIdx2.clone();
                    let mut headIdx: i32 = headIdx.clone();
                    (headIdx, headIdx2) = getIndexBySimCodeEq(head.clone())?;
                    let true = (intEq(headIdx.clone(), iIdx.clone()) || intEq(headIdx2.clone(), iIdx.clone())) else { bail!("pattern mismatch") };
                    Ok(head.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: head, tail: rest }, _) => {
                    Ok(getSimCodeEqByIndex(rest.clone(), iIdx.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getSimCodeEqByIndex failed. Looking for Index ")); __mm_s.push_str(&*intString(iIdx.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oEq)
}

fn getIndexBySimCodeEq(mut iEq: Arc<SimCode::SimEqSystem>) -> Result<(i32, i32)> {
    let mut oIdx: i32 = 0;
    let mut oIdx2: i32 = 0;
    let mut index: i32 = 0;
    let mut index2: i32 = 0;
    (oIdx, oIdx2) = (::match_deref::match_deref! { match &(iEq.clone()) {
        Deref @ SimCode::SimEqSystem::SES_RESIDUAL { index, .. } => (index.clone(), 0),
        Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { index, .. } => (index.clone(), 0),
        Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS { index, .. } => (index.clone(), 0),
        Deref @ SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN { index, .. } => (index.clone(), 0),
        Deref @ SimCode::SimEqSystem::SES_IFEQUATION { index, .. } => (index.clone(), 0),
        Deref @ SimCode::SimEqSystem::SES_ALGORITHM { index, .. } => (index.clone(), 0),
        Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem: Deref @ SimCode::LinearSystem { index, .. }, alternativeTearing: None, .. } => (index.clone(), 0),
        Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem: Deref @ SimCode::NonlinearSystem { index, .. }, alternativeTearing: None, .. } => (index.clone(), 0),
        Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem: Deref @ SimCode::LinearSystem { index, .. }, alternativeTearing: Some(Deref @ SimCode::LinearSystem { index: index2, .. }), .. } => (index.clone(), index2.clone()),
        Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem: Deref @ SimCode::NonlinearSystem { index, .. }, alternativeTearing: Some(Deref @ SimCode::NonlinearSystem { index: index2, .. }), .. } => (index.clone(), index2.clone()),
        Deref @ SimCode::SimEqSystem::SES_MIXED { index, .. } => (index.clone(), 0),
        Deref @ SimCode::SimEqSystem::SES_WHEN { index, .. } => (index.clone(), 0),
        Deref @ SimCode::SimEqSystem::SES_ALIAS { aliasOf: index, .. } => (index.clone(), 0),
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("HpcOmTaskGraph.getIndexBySimCodeEq")); __mm_s.push_str(&*literal!(" failed")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oIdx, oIdx2))
}

fn getSimCodeEqsByTaskList(mut iTaskList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut iSimEqIdxSimEqMapping: metamodelica::Array<Option<Arc<SimCode::SimEqSystem>>>) -> Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> {
    let mut oSimEqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    let mut tmpSimEqs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>>> = metamodelica::nil();
    tmpSimEqs = List::map1(iTaskList.clone(), (std::sync::Arc::new(fnptr!(getSimCodeEqsByTaskList0, Arc<HpcOmSimCode::Task>, metamodelica::Array<Option<Arc<SimCode::SimEqSystem>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, metamodelica::Array<Option<Arc<SimCode::SimEqSystem>>>) -> Result<Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>> + 'static>), iSimEqIdxSimEqMapping.clone());
    oSimEqs = List::flatten(tmpSimEqs.clone());
    oSimEqs
}

fn getSimCodeEqsByTaskList0(mut iTask: Arc<HpcOmSimCode::Task>, mut iSimEqIdxSimEqMapping: metamodelica::Array<Option<Arc<SimCode::SimEqSystem>>>) -> Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> {
    let mut oSimEqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    let mut eqIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpSimEqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    oSimEqs = (::match_deref::match_deref! { match &((iTask.clone(), iSimEqIdxSimEqMapping.clone())) {
        (Deref @ HpcOmSimCode::Task::CALCTASK { eqIdc, .. }, _) => {
            tmpSimEqs = List::map1r(eqIdc.clone(), (std::sync::Arc::new(getSimCodeEqByIndexAndMapping) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<Option<Arc<SimCode::SimEqSystem>>>, i32) -> Result<Arc<SimCode::SimEqSystem>> + 'static>), iSimEqIdxSimEqMapping.clone());
            tmpSimEqs.clone()
        },
        (Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { eqIdc, .. }, _) => {
            tmpSimEqs = List::map1r(eqIdc.clone(), (std::sync::Arc::new(getSimCodeEqByIndexAndMapping) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<Option<Arc<SimCode::SimEqSystem>>>, i32) -> Result<Arc<SimCode::SimEqSystem>> + 'static>), iSimEqIdxSimEqMapping.clone());
            tmpSimEqs.clone()
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oSimEqs
}

fn dumpSimEqSCCMapping(mut iSccMapping: metamodelica::Array<i32>) -> () {
    let mut text: ArcStr = arcstr::literal!("");
    text = (literal!("SimEqToSCCMapping")).clone();
    (_, text) = Array::fold(iSccMapping.clone(), (std::sync::Arc::new(fnptr!(dumpSimEqSCCMapping1, i32, (i32, ArcStr))) as std::sync::Arc<dyn ::std::ops::Fn(i32, (i32, ArcStr)) -> Result<(i32, ArcStr)> + 'static>), (1, text.clone()));
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*text.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    ()
}

fn dumpSimEqSCCMapping1(mut iMapping: i32, mut iIndexText: (i32, ArcStr)) -> (i32, ArcStr) {
    let mut oIndexText: (i32, ArcStr);
    let mut iIndex: i32 = 0;
    let mut text: ArcStr = arcstr::literal!("");
    let mut iText: ArcStr = arcstr::literal!("");
    (iIndex, iText) = iIndexText.clone();
    text = (intString(iMapping.clone())).clone();
    text = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*iText.clone()); __mm_s.push_str(&*literal!("\nSimEq ")); __mm_s.push_str(&*intString(iIndex.clone())); __mm_s.push_str(&*literal!(": {")); __mm_s.push_str(&*text.clone()); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
    oIndexText = (iIndex.clone() + 1, text.clone());
    oIndexText
}

fn dumpSccSimEqMapping(mut iSccMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> () {
    let mut text: ArcStr = arcstr::literal!("");
    text = (literal!("SccToSimEqMapping")).clone();
    (_, text) = Array::fold(iSccMapping.clone(), (std::sync::Arc::new(fnptr!(dumpSccSimEqMapping1, Arc<metamodelica::List<i32>>, (i32, ArcStr))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, (i32, ArcStr)) -> Result<(i32, ArcStr)> + 'static>), (1, text.clone()));
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*text.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    ()
}

fn dumpSccSimEqMapping1(mut iMapping: Arc<metamodelica::List<i32>>, mut iIndexText: (i32, ArcStr)) -> (i32, ArcStr) {
    let mut oIndexText: (i32, ArcStr);
    let mut iIndex: i32 = 0;
    let mut text: ArcStr = arcstr::literal!("");
    let mut iText: ArcStr = arcstr::literal!("");
    (iIndex, iText) = iIndexText.clone();
    text = (List::fold(iMapping.clone(), (std::sync::Arc::new(fnptr!(dumpSccSimEqMapping2, i32, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(i32, ArcStr) -> Result<ArcStr> + 'static>), (literal!(" ")).clone())).clone();
    text = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*iText.clone()); __mm_s.push_str(&*literal!("\nSCC ")); __mm_s.push_str(&*intString(iIndex.clone())); __mm_s.push_str(&*literal!(": {")); __mm_s.push_str(&*text.clone()); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
    oIndexText = (iIndex.clone() + 1, text.clone());
    oIndexText
}

fn dumpSccSimEqMapping2(mut iIndex: i32, mut iText: ArcStr) -> ArcStr {
    let mut oText: ArcStr = arcstr::literal!("");
    oText = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*iText.clone()); __mm_s.push_str(&*intString(iIndex.clone())); __mm_s.push_str(&*literal!(" ")); ArcStr::from(__mm_s) }).clone();
    oText
}

