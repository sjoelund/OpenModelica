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
use crate::HpcOmScheduler;
use crate::HpcOmSimCode;
use crate::HpcOmTaskGraph;
use crate::SimCode;
use crate::SimCodeUtil;
use crate::SimCodeVar;
use openmodelica_ast::Absyn;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::Expression;
use openmodelica_frontend::HashTableCrIListArray;
use openmodelica_frontend::HashTableCrILst;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_susan::GraphML;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

// -------------------------------------------
// STRUCTURES
// -------------------------------------------
pub const VARDATATYPE_FLOAT: i32 = 1;

pub const VARDATATYPE_INTEGER: i32 = 2;

pub const VARDATATYPE_BOOLEAN: i32 = 3;

pub const VARDATATYPE_STRING: i32 = 4;

pub const VARTYPE_STATE: i32 = 1;

pub const VARTYPE_STATEDER: i32 = 2;

pub const VARTYPE_PARAM: i32 = 3;

pub const VARTYPE_ALIAS: i32 = 4;

pub const VARTYPE_OTHER: i32 = 5;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CacheMap {
    CACHEMAP {
        cacheLineSize: i32,
        cacheVariables: Arc<metamodelica::List<SimCodeVar::SimVar>>,
        cacheLinesFloat: Arc<metamodelica::List<CacheLineMap>>,
        cacheLinesInt: Arc<metamodelica::List<CacheLineMap>>,
        cacheLinesBool: Arc<metamodelica::List<CacheLineMap>>,
    },
    UNIFORM_CACHEMAP {
        cacheLineSize: i32,
        cacheVariables: Arc<metamodelica::List<SimCodeVar::SimVar>>,
        cacheLines: Arc<metamodelica::List<CacheLineMap>>,
    },
}
pub use self::CacheMap::{CACHEMAP,UNIFORM_CACHEMAP};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CacheLineMap {
    pub idx: i32,
    pub numBytesFree: i32,
    pub entries: Arc<metamodelica::List<CacheLineEntry>>,
}

pub type CACHELINEMAP = CacheLineMap;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CacheLineEntry {
    pub start: i32,
    pub dataType: i32,
    pub size: i32,
    pub scVarIdx: i32,
    pub threadOwner: i32,
}

pub type CACHELINEENTRY = CacheLineEntry;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CacheMapMeta {
    pub allSCVarsMapping: metamodelica::Array<Option<SimCodeVar::SimVar>>,
    pub simCodeVarTypes: metamodelica::Array<(i32, i32, i32)>,
    pub scVarCLMapping: metamodelica::Array<(i32, i32)>,
}

pub type CACHEMAPMETA = CacheMapMeta;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PartlyFilledCacheLine {
    PARTLYFILLEDCACHELINE_LEVEL {
        cacheLineMap: CacheLineMap,
        prefetchLevel: Arc<metamodelica::List<i32>>,
        writeLevel: Arc<metamodelica::List<(i32, i32)>>,
    },
    PARTLYFILLEDCACHELINE_THREAD {
        cacheLineMap: CacheLineMap,
    },
}
pub use self::PartlyFilledCacheLine::{PARTLYFILLEDCACHELINE_LEVEL,PARTLYFILLEDCACHELINE_THREAD};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScVarInfo {
    pub ownerThread: i32,
    pub isShared: bool,
}

pub type SCVARINFO = ScVarInfo;


pub type PartlyFilledCacheLines = (Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>);

pub type CacheLines = (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>);

// -------------------------------------------
// FUNCTIONS
// -------------------------------------------
pub fn createMemoryMap(mut iModelInfo: SimCode::ModelInfo, mut iVarToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (HashTableCrIListArray::FuncHashCref, HashTableCrIListArray::FuncCrefEqual, HashTableCrIListArray::FuncCrefStr, HashTableCrIListArray::FuncExpStr)), mut iVarToIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr)), mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iEqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, mut iFileNamePrefix: ArcStr, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>, mut iSchedule: Arc<HpcOmSimCode::Schedule>, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iCriticalPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut iCriticalPathsWoC: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut iCriticalPathInfo: ArcStr, mut iNumberOfThreads: i32, mut iAllComponents: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut isInitial: bool) -> Result<(Option<HpcOmSimCode::MemoryMap>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (HashTableCrIListArray::FuncHashCref, HashTableCrIListArray::FuncCrefEqual, HashTableCrIListArray::FuncCrefStr, HashTableCrIListArray::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr)))> {
    let mut oMemoryMap: Option<HpcOmSimCode::MemoryMap> = None;
    let mut oVarToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (HashTableCrIListArray::FuncHashCref, HashTableCrIListArray::FuncCrefEqual, HashTableCrIListArray::FuncCrefStr, HashTableCrIListArray::FuncExpStr));
    let mut oVarToIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr));
    let mut simCodeVars: SimCodeVar::SimVars;
    let mut stateVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut derivativeVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut algVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut discreteAlgVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut intAlgVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut boolAlgVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut stringAlgVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut inputVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut outputVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut aliasVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut paramVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut intParamVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut boolParamVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut stringParamVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut intAliasVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut boolAliasVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut stringAliasVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut notOptimizedVarsFloatOpt: Arc<metamodelica::List<Option<SimCodeVar::SimVar>>> = metamodelica::nil();
    let mut notOptimizedVarsIntOpt: Arc<metamodelica::List<Option<SimCodeVar::SimVar>>> = metamodelica::nil();
    let mut notOptimizedVarsBoolOpt: Arc<metamodelica::List<Option<SimCodeVar::SimVar>>> = metamodelica::nil();
    let mut notOptimizedVarsStringOpt: Arc<metamodelica::List<Option<SimCodeVar::SimVar>>> = metamodelica::nil();
    let mut notOptimizedVarsFloat: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut notOptimizedVarsInt: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut notOptimizedVarsBool: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut notOptimizedVarsString: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut notOptimizedVars: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
    let mut allVarsMapping: metamodelica::Array<Option<SimCodeVar::SimVar>>;
    let mut simVarIdxMappingHashTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr));
    let mut numScVars: i32 = 0;
    let mut numCL: i32 = 0;
    let mut threadAttIdx: i32 = 0;
    let mut clTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut scVarSolvedTaskMapping: metamodelica::Array<i32>;
    let mut sccNodeMapping: metamodelica::Array<i32>;
    let mut scVarUnsolvedTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut annotInfo: metamodelica::Array<ArcStr>;
    let mut scVarCLMapping: metamodelica::Array<(i32, i32)>;
    let mut memoryPositionMapping: metamodelica::Array<(i32, i32)>;
    let mut cacheMap: CacheMap;
    let mut graphIdx: i32 = 0;
    let mut graphInfo: GraphML::GraphInfo;
    let mut fileName: ArcStr = arcstr::literal!("");
    let mut eqSimCodeVarMapping: metamodelica::Array<metamodelica::Array<Arc<metamodelica::List<i32>>>>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut adjacencyMatrix: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut tmpMemoryMapOpt: Option<HpcOmSimCode::MemoryMap> = None;
    let mut varCount: i32 = 0;
    let mut stateVarsCnt: i32 = 0;
    let mut derivativeVarsCnt: i32 = 0;
    let mut algVarsCnt: i32 = 0;
    let mut discreteAlgVarsCnt: i32 = 0;
    let mut intAlgVarsCnt: i32 = 0;
    let mut boolAlgVarsCnt: i32 = 0;
    let mut stringAlgVarsCnt: i32 = 0;
    let mut inputVarsCnt: i32 = 0;
    let mut outputVarsCnt: i32 = 0;
    let mut aliasVarsCnt: i32 = 0;
    let mut intAliasVarsCnt: i32 = 0;
    let mut boolAliasVarsCnt: i32 = 0;
    let mut stringAliasVarsCnt: i32 = 0;
    let mut paramVarsCnt: i32 = 0;
    let mut intParamVarsCnt: i32 = 0;
    let mut boolParamVarsCnt: i32 = 0;
    let mut stringParamVarsCnt: i32 = 0;
    let mut VARSIZE_FLOAT: i32 = 0;
    let mut VARSIZE_INTEGER: i32 = 0;
    let mut VARSIZE_BOOLEAN: i32 = 0;
    let mut VARSIZE_STRING: i32 = 0;
    let mut CACHELINE_SIZE: i32 = 0;
    let mut simCodeVarTypes: metamodelica::Array<(i32, i32, i32)>;
    let mut taskSolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut taskUnsolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut nodeSccMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut flatEqSimCodeVarMapping: metamodelica::Array<(i32, Arc<metamodelica::List<i32>>)>;
    let mut sccEqMapping: metamodelica::Array<Arc<metamodelica::List<(i32, i32, i32)>>>;
    let mut scVarInfos: metamodelica::Array<ScVarInfo>;
    let mut varToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (HashTableCrIListArray::FuncHashCref, HashTableCrIListArray::FuncCrefEqual, HashTableCrIListArray::FuncCrefStr, HashTableCrIListArray::FuncExpStr));
    let mut varToIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr));
    (oMemoryMap, oVarToArrayIndexMapping, oVarToIndexMapping) = 'mc: {
        let __mc_input = (iModelInfo.clone(), iVarToArrayIndexMapping.clone(), iVarToIndexMapping.clone(), iTaskGraph.clone(), iTaskGraphMeta.clone(), iEqSystems.clone(), iFileNamePrefix.clone(), iSchedulerInfo.clone(), iSchedule.clone(), iSccSimEqMapping.clone(), iCriticalPaths.clone(), iCriticalPathsWoC.clone(), iCriticalPathInfo.clone(), iNumberOfThreads.clone(), iAllComponents.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, varToArrayIndexMapping, varToIndexMapping, _, HpcOmTaskGraph::TaskGraphMeta { varCompMapping, eqCompMapping, .. }, _, _, _, _, _, _, _, _, _, _) => {
                    let mut varToArrayIndexMapping = (*varToArrayIndexMapping).clone();
                    let mut varToIndexMapping = (*varToIndexMapping).clone();
                    let mut VARSIZE_STRING: i32 = VARSIZE_STRING.clone();
                    let mut scVarInfos: metamodelica::Array<ScVarInfo>;
                    let mut intAliasVarsCnt: i32 = intAliasVarsCnt.clone();
                    let mut intParamVarsCnt: i32 = intParamVarsCnt.clone();
                    let mut stateVarsCnt: i32 = stateVarsCnt.clone();
                    let mut VARSIZE_INTEGER: i32 = VARSIZE_INTEGER.clone();
                    let mut simCodeVarTypes: metamodelica::Array<(i32, i32, i32)>;
                    let mut notOptimizedVarsIntOpt: Arc<metamodelica::List<Option<SimCodeVar::SimVar>>> = notOptimizedVarsIntOpt.clone();
                    let mut stringAlgVarsCnt: i32 = stringAlgVarsCnt.clone();
                    let mut notOptimizedVarsFloat: Arc<metamodelica::List<SimCodeVar::SimVar>> = notOptimizedVarsFloat.clone();
                    let mut aliasVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = aliasVars.clone();
                    let mut threadAttIdx: i32 = threadAttIdx.clone();
                    let mut inputVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = inputVars.clone();
                    let mut sccNodeMapping: metamodelica::Array<i32>;
                    let mut boolAliasVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = boolAliasVars.clone();
                    let mut eqSimCodeVarMapping: metamodelica::Array<metamodelica::Array<Arc<metamodelica::List<i32>>>>;
                    let mut derivativeVarsCnt: i32 = derivativeVarsCnt.clone();
                    let mut CACHELINE_SIZE: i32 = CACHELINE_SIZE.clone();
                    let mut VARSIZE_FLOAT: i32 = VARSIZE_FLOAT.clone();
                    let mut notOptimizedVarsInt: Arc<metamodelica::List<SimCodeVar::SimVar>> = notOptimizedVarsInt.clone();
                    let mut boolParamVarsCnt: i32 = boolParamVarsCnt.clone();
                    let mut VARSIZE_BOOLEAN: i32 = VARSIZE_BOOLEAN.clone();
                    let mut simVarIdxMappingHashTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr));
                    let mut flatEqSimCodeVarMapping: metamodelica::Array<(i32, Arc<metamodelica::List<i32>>)>;
                    let mut stringAliasVarsCnt: i32 = stringAliasVarsCnt.clone();
                    let mut notOptimizedVarsString: Arc<metamodelica::List<SimCodeVar::SimVar>> = notOptimizedVarsString.clone();
                    let mut stateVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = stateVars.clone();
                    let mut boolParamVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = boolParamVars.clone();
                    let mut algVarsCnt: i32 = algVarsCnt.clone();
                    let mut scVarUnsolvedTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut aliasVarsCnt: i32 = aliasVarsCnt.clone();
                    let mut fileName: ArcStr = fileName.clone();
                    let mut algVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = algVars.clone();
                    let mut graphIdx: i32 = graphIdx.clone();
                    let mut taskUnsolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut taskSolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut graphInfo: GraphML::GraphInfo;
                    let mut outputVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = outputVars.clone();
                    let mut notOptimizedVarsStringOpt: Arc<metamodelica::List<Option<SimCodeVar::SimVar>>> = notOptimizedVarsStringOpt.clone();
                    let mut scVarSolvedTaskMapping: metamodelica::Array<i32>;
                    let mut cacheMap: CacheMap;
                    let mut notOptimizedVars: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
                    let mut boolAliasVarsCnt: i32 = boolAliasVarsCnt.clone();
                    let mut intAlgVarsCnt: i32 = intAlgVarsCnt.clone();
                    let mut numCL: i32 = numCL.clone();
                    let mut derivativeVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = derivativeVars.clone();
                    let mut stringAlgVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = stringAlgVars.clone();
                    let mut intParamVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = intParamVars.clone();
                    let mut stringAliasVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = stringAliasVars.clone();
                    let mut paramVarsCnt: i32 = paramVarsCnt.clone();
                    let mut nodeSccMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut notOptimizedVarsBool: Arc<metamodelica::List<SimCodeVar::SimVar>> = notOptimizedVarsBool.clone();
                    let mut simCodeVars: SimCodeVar::SimVars;
                    let mut annotInfo: metamodelica::Array<ArcStr>;
                    let mut notOptimizedVarsFloatOpt: Arc<metamodelica::List<Option<SimCodeVar::SimVar>>> = notOptimizedVarsFloatOpt.clone();
                    let mut adjacencyMatrix: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut discreteAlgVarsCnt: i32 = discreteAlgVarsCnt.clone();
                    let mut stringParamVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = stringParamVars.clone();
                    let mut boolAlgVarsCnt: i32 = boolAlgVarsCnt.clone();
                    let mut outputVarsCnt: i32 = outputVarsCnt.clone();
                    let mut boolAlgVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = boolAlgVars.clone();
                    let mut intAliasVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = intAliasVars.clone();
                    let mut scVarCLMapping: metamodelica::Array<(i32, i32)>;
                    let mut sccEqMapping: metamodelica::Array<Arc<metamodelica::List<(i32, i32, i32)>>>;
                    let mut clTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut tmpMemoryMapOpt: Option<HpcOmSimCode::MemoryMap> = tmpMemoryMapOpt.clone();
                    let mut stringParamVarsCnt: i32 = stringParamVarsCnt.clone();
                    let mut paramVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = paramVars.clone();
                    let mut varCount: i32 = varCount.clone();
                    let mut intAlgVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = intAlgVars.clone();
                    let mut inputVarsCnt: i32 = inputVarsCnt.clone();
                    let mut discreteAlgVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = discreteAlgVars.clone();
                    let mut notOptimizedVarsBoolOpt: Arc<metamodelica::List<Option<SimCodeVar::SimVar>>> = notOptimizedVarsBoolOpt.clone();
                    let mut allVarsMapping: metamodelica::Array<Option<SimCodeVar::SimVar>>;
                    VARSIZE_FLOAT = 8;
                    VARSIZE_INTEGER = 4;
                    VARSIZE_BOOLEAN = 1;
                    VARSIZE_STRING = 4;
                    CACHELINE_SIZE = 64;
                    let SimCode::MODELINFO { vars: __pa0, .. } = (iModelInfo.clone()) else { bail!("pattern mismatch") };
                    simCodeVars = __pa0.clone();
                    let SimCodeVar::SIMVARS { stringParamVars: __pa1, boolParamVars: __pa2, intParamVars: __pa3, paramVars: __pa4, stringAliasVars: __pa5, boolAliasVars: __pa6, intAliasVars: __pa7, aliasVars: __pa8, outputVars: __pa9, inputVars: __pa10, stringAlgVars: __pa11, boolAlgVars: __pa12, intAlgVars: __pa13, discreteAlgVars: __pa14, algVars: __pa15, derivativeVars: __pa16, stateVars: __pa17, .. } = (simCodeVars.clone()) else { bail!("pattern mismatch") };
                    stringParamVars = __pa1.clone();
                    boolParamVars = __pa2.clone();
                    intParamVars = __pa3.clone();
                    paramVars = __pa4.clone();
                    stringAliasVars = __pa5.clone();
                    boolAliasVars = __pa6.clone();
                    intAliasVars = __pa7.clone();
                    aliasVars = __pa8.clone();
                    outputVars = __pa9.clone();
                    inputVars = __pa10.clone();
                    stringAlgVars = __pa11.clone();
                    boolAlgVars = __pa12.clone();
                    intAlgVars = __pa13.clone();
                    discreteAlgVars = __pa14.clone();
                    algVars = __pa15.clone();
                    derivativeVars = __pa16.clone();
                    stateVars = __pa17.clone();
                    allVarsMapping = SimCodeUtil::createIdxSCVarMapping(simCodeVars.clone())?;
                    simVarIdxMappingHashTable = HashTableCrILst::emptyHashTableSized(BaseHashTable::biggerBucketSize.clone());
                    varCount = 0;
                    stateVarsCnt = (stateVars.clone().len() as i32);
                    varCount = varCount.clone() + stateVarsCnt.clone();
                    derivativeVarsCnt = (derivativeVars.clone().len() as i32);
                    varCount = varCount.clone() + derivativeVarsCnt.clone();
                    simVarIdxMappingHashTable = fillSimVarHashTable(algVars.clone(), varCount.clone(), VARDATATYPE_FLOAT.clone(), simVarIdxMappingHashTable.clone())?;
                    algVarsCnt = (algVars.clone().len() as i32);
                    varCount = varCount.clone() + algVarsCnt.clone();
                    simVarIdxMappingHashTable = fillSimVarHashTable(discreteAlgVars.clone(), varCount.clone(), VARDATATYPE_FLOAT.clone(), simVarIdxMappingHashTable.clone())?;
                    discreteAlgVarsCnt = (discreteAlgVars.clone().len() as i32);
                    varCount = varCount.clone() + discreteAlgVarsCnt.clone();
                    simVarIdxMappingHashTable = fillSimVarHashTable(intAlgVars.clone(), varCount.clone(), VARDATATYPE_INTEGER.clone(), simVarIdxMappingHashTable.clone())?;
                    intAlgVarsCnt = (intAlgVars.clone().len() as i32);
                    varCount = varCount.clone() + intAlgVarsCnt.clone();
                    simVarIdxMappingHashTable = fillSimVarHashTable(boolAlgVars.clone(), varCount.clone(), VARDATATYPE_BOOLEAN.clone(), simVarIdxMappingHashTable.clone())?;
                    boolAlgVarsCnt = (boolAlgVars.clone().len() as i32);
                    varCount = varCount.clone() + boolAlgVarsCnt.clone();
                    simVarIdxMappingHashTable = fillSimVarHashTable(stringAlgVars.clone(), varCount.clone(), VARDATATYPE_STRING.clone(), simVarIdxMappingHashTable.clone())?;
                    stringAlgVarsCnt = (stringAlgVars.clone().len() as i32);
                    varCount = varCount.clone() + stringAlgVarsCnt.clone();
                    simVarIdxMappingHashTable = fillSimVarHashTable(inputVars.clone(), varCount.clone(), VARDATATYPE_FLOAT.clone(), simVarIdxMappingHashTable.clone())?;
                    inputVarsCnt = (inputVars.clone().len() as i32);
                    varCount = varCount.clone() + inputVarsCnt.clone();
                    simVarIdxMappingHashTable = fillSimVarHashTable(outputVars.clone(), varCount.clone(), VARDATATYPE_FLOAT.clone(), simVarIdxMappingHashTable.clone())?;
                    outputVarsCnt = (outputVars.clone().len() as i32);
                    varCount = varCount.clone() + outputVarsCnt.clone();
                    aliasVarsCnt = (aliasVars.clone().len() as i32);
                    varCount = varCount.clone() + aliasVarsCnt.clone();
                    intAliasVarsCnt = (intAliasVars.clone().len() as i32);
                    varCount = varCount.clone() + intAliasVarsCnt.clone();
                    boolAliasVarsCnt = (boolAliasVars.clone().len() as i32);
                    varCount = varCount.clone() + boolAliasVarsCnt.clone();
                    simVarIdxMappingHashTable = fillSimVarHashTable(stringAliasVars.clone(), varCount.clone(), VARDATATYPE_STRING.clone(), simVarIdxMappingHashTable.clone())?;
                    stringAliasVarsCnt = (stringAliasVars.clone().len() as i32);
                    varCount = varCount.clone() + stringAliasVarsCnt.clone();
                    simVarIdxMappingHashTable = fillSimVarHashTable(paramVars.clone(), varCount.clone(), VARDATATYPE_FLOAT.clone(), simVarIdxMappingHashTable.clone())?;
                    paramVarsCnt = (paramVars.clone().len() as i32);
                    varCount = varCount.clone() + paramVarsCnt.clone();
                    simVarIdxMappingHashTable = fillSimVarHashTable(intParamVars.clone(), varCount.clone(), VARDATATYPE_INTEGER.clone(), simVarIdxMappingHashTable.clone())?;
                    intParamVarsCnt = (intParamVars.clone().len() as i32);
                    varCount = varCount.clone() + intParamVarsCnt.clone();
                    simVarIdxMappingHashTable = fillSimVarHashTable(boolParamVars.clone(), varCount.clone(), VARDATATYPE_BOOLEAN.clone(), simVarIdxMappingHashTable.clone())?;
                    boolParamVarsCnt = (boolParamVars.clone().len() as i32);
                    varCount = varCount.clone() + boolParamVarsCnt.clone();
                    simVarIdxMappingHashTable = fillSimVarHashTable(stringParamVars.clone(), varCount.clone(), VARDATATYPE_STRING.clone(), simVarIdxMappingHashTable.clone())?;
                    stringParamVarsCnt = (stringParamVars.clone().len() as i32);
                    varCount = varCount.clone() + stringParamVarsCnt.clone();
                    simCodeVarTypes = arrayCreate(varCount.clone(), (-1, -1, -1));
                    varCount = 0;
                    varCount = varCount.clone() + stateVarsCnt.clone();
                    varCount = varCount.clone() + derivativeVarsCnt.clone();
                    if algVarsCnt.clone() > 0 {
                        List::map_0(List::intRange2(varCount.clone() + 1, varCount.clone() + algVarsCnt.clone()), Arc::new({ let __pe_b1 = (VARDATATYPE_FLOAT.clone(), VARSIZE_FLOAT.clone(), VARTYPE_OTHER.clone()); let __pe_b2 = simCodeVarTypes.clone(); move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
                    }
                    varCount = varCount.clone() + algVarsCnt.clone();
                    if discreteAlgVarsCnt.clone() > 0 {
                        List::map_0(List::intRange2(varCount.clone() + 1, varCount.clone() + discreteAlgVarsCnt.clone()), Arc::new({ let __pe_b1 = (VARDATATYPE_FLOAT.clone(), VARSIZE_FLOAT.clone(), VARTYPE_OTHER.clone()); let __pe_b2 = simCodeVarTypes.clone(); move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
                    }
                    varCount = varCount.clone() + discreteAlgVarsCnt.clone();
                    if intAlgVarsCnt.clone() > 0 {
                        List::map_0(List::intRange2(varCount.clone() + 1, varCount.clone() + intAlgVarsCnt.clone()), Arc::new({ let __pe_b1 = (VARDATATYPE_INTEGER.clone(), VARSIZE_INTEGER.clone(), VARTYPE_OTHER.clone()); let __pe_b2 = simCodeVarTypes.clone(); move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
                    }
                    varCount = varCount.clone() + intAlgVarsCnt.clone();
                    if boolAlgVarsCnt.clone() > 0 {
                        List::map_0(List::intRange2(varCount.clone() + 1, varCount.clone() + boolAlgVarsCnt.clone()), Arc::new({ let __pe_b1 = (VARDATATYPE_BOOLEAN.clone(), VARSIZE_BOOLEAN.clone(), VARTYPE_OTHER.clone()); let __pe_b2 = simCodeVarTypes.clone(); move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
                    }
                    varCount = varCount.clone() + boolAlgVarsCnt.clone();
                    if stringAlgVarsCnt.clone() > 0 {
                        List::map_0(List::intRange2(varCount.clone() + 1, varCount.clone() + stringAlgVarsCnt.clone()), Arc::new({ let __pe_b1 = (VARDATATYPE_STRING.clone(), VARSIZE_STRING.clone(), VARTYPE_OTHER.clone()); let __pe_b2 = simCodeVarTypes.clone(); move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
                    }
                    varCount = varCount.clone() + stringAlgVarsCnt.clone();
                    if inputVarsCnt.clone() > 0 {
                        List::map_0(List::intRange2(varCount.clone() + 1, varCount.clone() + inputVarsCnt.clone()), Arc::new({ let __pe_b1 = (VARDATATYPE_FLOAT.clone(), VARSIZE_FLOAT.clone(), VARTYPE_OTHER.clone()); let __pe_b2 = simCodeVarTypes.clone(); move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
                    }
                    varCount = varCount.clone() + inputVarsCnt.clone();
                    if outputVarsCnt.clone() > 0 {
                        List::map_0(List::intRange2(varCount.clone() + 1, varCount.clone() + outputVarsCnt.clone()), Arc::new({ let __pe_b1 = (VARDATATYPE_FLOAT.clone(), VARSIZE_FLOAT.clone(), VARTYPE_OTHER.clone()); let __pe_b2 = simCodeVarTypes.clone(); move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
                    }
                    varCount = varCount.clone() + outputVarsCnt.clone();
                    varCount = varCount.clone() + aliasVarsCnt.clone();
                    varCount = varCount.clone() + intAliasVarsCnt.clone();
                    varCount = varCount.clone() + boolAliasVarsCnt.clone();
                    varCount = varCount.clone() + stringAliasVarsCnt.clone();
                    if paramVarsCnt.clone() > 0 {
                        List::map_0(List::intRange2(varCount.clone() + 1, varCount.clone() + paramVarsCnt.clone()), Arc::new({ let __pe_b1 = (VARDATATYPE_FLOAT.clone(), VARSIZE_FLOAT.clone(), VARTYPE_PARAM.clone()); let __pe_b2 = simCodeVarTypes.clone(); move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
                    }
                    varCount = varCount.clone() + paramVarsCnt.clone();
                    if intParamVarsCnt.clone() > 0 {
                        List::map_0(List::intRange2(varCount.clone() + 1, varCount.clone() + intParamVarsCnt.clone()), Arc::new({ let __pe_b1 = (VARDATATYPE_INTEGER.clone(), VARSIZE_INTEGER.clone(), VARTYPE_PARAM.clone()); let __pe_b2 = simCodeVarTypes.clone(); move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
                    }
                    varCount = varCount.clone() + intParamVarsCnt.clone();
                    if boolParamVarsCnt.clone() > 0 {
                        List::map_0(List::intRange2(varCount.clone() + 1, varCount.clone() + boolParamVarsCnt.clone()), Arc::new({ let __pe_b1 = (VARDATATYPE_BOOLEAN.clone(), VARSIZE_BOOLEAN.clone(), VARTYPE_PARAM.clone()); let __pe_b2 = simCodeVarTypes.clone(); move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
                    }
                    varCount = varCount.clone() + boolParamVarsCnt.clone();
                    if stringParamVarsCnt.clone() > 0 {
                        List::map_0(List::intRange2(varCount.clone() + 1, varCount.clone() + stringParamVarsCnt.clone()), Arc::new({ let __pe_b1 = (VARDATATYPE_STRING.clone(), VARSIZE_STRING.clone(), VARTYPE_PARAM.clone()); let __pe_b2 = simCodeVarTypes.clone(); move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
                    }
                    varCount = varCount.clone() + stringParamVarsCnt.clone();
                    sccNodeMapping = HpcOmTaskGraph::getSccNodeMapping((iSccSimEqMapping.clone().borrow().len() as i32), iTaskGraphMeta.clone())?;
                    scVarSolvedTaskMapping = getSimCodeVarNodeMapping(iTaskGraphMeta.clone(), iEqSystems.clone(), varCount.clone(), sccNodeMapping.clone(), simVarIdxMappingHashTable.clone())?;
                    eqSimCodeVarMapping = getEqSCVarMapping(iEqSystems.clone(), simVarIdxMappingHashTable.clone());
                    sccEqMapping = invertEqCompMapping(eqCompMapping.clone(), (sccNodeMapping.clone().borrow().len() as i32))?;
                    nodeSccMapping = invertSccNodeMapping(sccNodeMapping.clone(), (iTaskGraph.clone().borrow().len() as i32))?;
                    flatEqSimCodeVarMapping = flattenEqSimCodeVarMapping(eqSimCodeVarMapping.clone())?;
                    (taskSolvedVarsMapping, taskUnsolvedVarsMapping) = getTaskSimVarMapping(sccEqMapping.clone(), nodeSccMapping.clone(), flatEqSimCodeVarMapping.clone(), scVarSolvedTaskMapping.clone(), simCodeVarTypes.clone())?;
                    scVarUnsolvedTaskMapping = transposeTasksScVarsMapping(taskUnsolvedVarsMapping.clone(), varCount.clone())?;
                    scVarInfos = createVarInfos(scVarSolvedTaskMapping.clone(), scVarUnsolvedTaskMapping.clone(), iSchedulerInfo.clone())?;
                    if Flags::isSet(Flags::HPCOM_MEMORY_OPT.clone())? {
                        (cacheMap, scVarCLMapping, numCL) = createCacheMapOptimized(iTaskGraph.clone(), iTaskGraphMeta.clone(), simCodeVars.clone(), allVarsMapping.clone(), simCodeVarTypes.clone(), scVarSolvedTaskMapping.clone(), scVarUnsolvedTaskMapping.clone(), CACHELINE_SIZE.clone(), iAllComponents.clone(), iSchedule.clone(), iSchedulerInfo.clone(), iNumberOfThreads.clone(), taskSolvedVarsMapping.clone(), taskUnsolvedVarsMapping.clone(), scVarInfos.clone())?;
                    } else {
                        (cacheMap, scVarCLMapping, numCL) = createCacheMapDefault(allVarsMapping.clone(), CACHELINE_SIZE.clone(), simCodeVars.clone(), scVarSolvedTaskMapping.clone(), iSchedulerInfo.clone(), simCodeVarTypes.clone())?;
                    }
                    (clTaskMapping, _) = getCacheLineTaskMapping(iTaskGraphMeta.clone(), iEqSystems.clone(), simVarIdxMappingHashTable.clone(), numCL.clone(), scVarCLMapping.clone())?;
                    notOptimizedVars = getNotOptimizedVarsByCacheLineMapping(scVarCLMapping.clone(), allVarsMapping.clone(), simCodeVarTypes.clone());
                    notOptimizedVarsFloatOpt = List::map(Util::tuple41(notOptimizedVars.clone()), Arc::new(todo!("PARTEVALFUNCTION of arrayGet: function signature not resolved")));
                    notOptimizedVarsIntOpt = List::map(Util::tuple42(notOptimizedVars.clone()), Arc::new(todo!("PARTEVALFUNCTION of arrayGet: function signature not resolved")));
                    notOptimizedVarsBoolOpt = List::map(Util::tuple43(notOptimizedVars.clone()), Arc::new(todo!("PARTEVALFUNCTION of arrayGet: function signature not resolved")));
                    notOptimizedVarsStringOpt = List::map(Util::tuple44(notOptimizedVars.clone()), Arc::new(todo!("PARTEVALFUNCTION of arrayGet: function signature not resolved")));
                    notOptimizedVarsFloat = List::map(notOptimizedVarsFloatOpt.clone(), (std::sync::Arc::new(Util::getOption) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<_> + 'static>));
                    notOptimizedVarsInt = List::map(notOptimizedVarsIntOpt.clone(), (std::sync::Arc::new(Util::getOption) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<_> + 'static>));
                    notOptimizedVarsBool = List::map(notOptimizedVarsBoolOpt.clone(), (std::sync::Arc::new(Util::getOption) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<_> + 'static>));
                    notOptimizedVarsString = List::map(notOptimizedVarsStringOpt.clone(), (std::sync::Arc::new(Util::getOption) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<_> + 'static>));
                    graphInfo = GraphML::createGraphInfo();
                    let (__pa18, (_, __pa19)) = GraphML::addGraph((literal!("TasksGroupGraph")).clone(), true, graphInfo.clone())?;
                    graphInfo = __pa18.clone();
                    graphIdx = __pa19.clone();
                    let (__pa20, (_, _), (_, __pa21)) = GraphML::addGroupNode((literal!("TasksGroup")).clone(), graphIdx.clone(), false, (literal!("TG")).clone(), graphInfo.clone())?;
                    graphInfo = __pa20.clone();
                    graphIdx = __pa21.clone();
                    annotInfo = arrayCreate((iTaskGraph.clone().borrow().len() as i32), (literal!("nothing")).clone());
                    graphInfo = HpcOmTaskGraph::convertToGraphMLSccLevelSubgraph(iTaskGraph.clone(), iTaskGraphMeta.clone(), (iCriticalPathInfo.clone()).clone(), HpcOmTaskGraph::convertNodeListToEdgeTuples(listHead(iCriticalPaths.clone())?)?, HpcOmTaskGraph::convertNodeListToEdgeTuples(listHead(iCriticalPathsWoC.clone())?)?, iSccSimEqMapping.clone(), iSchedulerInfo.clone(), annotInfo.clone(), graphIdx.clone(), HpcOmTaskGraph::GraphDumpOptions { visualizeCriticalPath: false, visualizeTaskStartAndFinishTime: false, visualizeTaskCalcTime: true, visualizeCommTime: true }, graphInfo.clone())?;
                    let Some((_, __pa22)) = (GraphML::getAttributeByNameAndTarget((literal!("ThreadId")).clone(), openmodelica_susan::GraphML::AttributeTarget::TARGET_NODE, graphInfo.clone())?) else { bail!("pattern mismatch") };
                    threadAttIdx = __pa22.clone();
                    (_, adjacencyMatrix, _) = BackendDAEUtil::getAdjacencyMatrix(listHead(iEqSystems.clone())?, crate::BackendDAE::IndexType::ABSOLUTE, None, isInitial.clone())?;
                    graphInfo = appendCacheLinesToGraph(cacheMap.clone(), (iTaskGraph.clone().borrow().len() as i32), eqSimCodeVarMapping.clone(), iEqSystems.clone(), simVarIdxMappingHashTable.clone(), eqCompMapping.clone(), scVarSolvedTaskMapping.clone(), iSchedulerInfo.clone(), threadAttIdx.clone(), sccNodeMapping.clone(), taskSolvedVarsMapping.clone(), taskUnsolvedVarsMapping.clone(), scVarCLMapping.clone(), scVarInfos.clone(), graphInfo.clone())?;
                    fileName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("taskGraph")); __mm_s.push_str(&*iFileNamePrefix.clone()); __mm_s.push_str(&*literal!("ODE_schedule_CL.graphml")); ArcStr::from(__mm_s) }).clone();
                    GraphML::dumpGraph(graphInfo.clone(), (fileName.clone()).clone())?;
                    if Flags::isSet(Flags::HPCOM_MEMORY_OPT.clone())? {
                        (varToArrayIndexMapping, varToIndexMapping, tmpMemoryMapOpt) = convertCacheToVarArrayMapping(cacheMap.clone(), CACHELINE_SIZE.clone(), stateVars.clone(), derivativeVars.clone(), aliasVars.clone(), intAliasVars.clone(), boolAliasVars.clone(), stringAliasVars.clone(), (VARSIZE_FLOAT.clone(), VARSIZE_INTEGER.clone(), VARSIZE_BOOLEAN.clone()), (notOptimizedVarsFloat.clone(), notOptimizedVarsInt.clone(), notOptimizedVarsBool.clone(), notOptimizedVarsString.clone()))?;
                    } else {
                        tmpMemoryMapOpt = None;
                    }
                    evaluateCacheBehaviour(varToIndexMapping.clone(), simVarIdxMappingHashTable.clone(), taskSolvedVarsMapping.clone(), taskUnsolvedVarsMapping.clone(), iTaskGraph.clone(), iTaskGraphT.clone(), iNumberOfThreads.clone(), CACHELINE_SIZE.clone(), simCodeVarTypes.clone(), iSchedulerInfo.clone());
                    graphInfo = GraphML::createGraphInfo();
                    let (__pa23, (_, __pa24)) = GraphML::addGraph((literal!("TasksGroupGraph")).clone(), true, graphInfo.clone())?;
                    graphInfo = __pa23.clone();
                    graphIdx = __pa24.clone();
                    annotInfo = arrayCreate((iTaskGraph.clone().borrow().len() as i32), (literal!("nothing")).clone());
                    graphInfo = HpcOmTaskGraph::convertToGraphMLSccLevelSubgraph(iTaskGraph.clone(), iTaskGraphMeta.clone(), (iCriticalPathInfo.clone()).clone(), HpcOmTaskGraph::convertNodeListToEdgeTuples(listHead(iCriticalPaths.clone())?)?, HpcOmTaskGraph::convertNodeListToEdgeTuples(listHead(iCriticalPathsWoC.clone())?)?, iSccSimEqMapping.clone(), iSchedulerInfo.clone(), annotInfo.clone(), graphIdx.clone(), HpcOmTaskGraph::GraphDumpOptions { visualizeCriticalPath: false, visualizeTaskStartAndFinishTime: false, visualizeTaskCalcTime: true, visualizeCommTime: true }, graphInfo.clone())?;
                    let Some((_, __pa25)) = (GraphML::getAttributeByNameAndTarget((literal!("ThreadId")).clone(), openmodelica_susan::GraphML::AttributeTarget::TARGET_NODE, graphInfo.clone())?) else { bail!("pattern mismatch") };
                    threadAttIdx = __pa25.clone();
                    graphInfo = appendVariablesToGraph(taskSolvedVarsMapping.clone(), taskUnsolvedVarsMapping.clone(), (scVarSolvedTaskMapping.clone().borrow().len() as i32), graphIdx.clone(), threadAttIdx.clone(), simVarIdxMappingHashTable.clone(), allVarsMapping.clone(), scVarInfos.clone(), graphInfo.clone())?;
                    fileName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("taskGraph")); __mm_s.push_str(&*iFileNamePrefix.clone()); __mm_s.push_str(&*literal!("ODE_schedule_vars.graphml")); ArcStr::from(__mm_s) }).clone();
                    GraphML::dumpGraph(graphInfo.clone(), (fileName.clone()).clone())?;
                    Ok((tmpMemoryMapOpt.clone(), varToArrayIndexMapping.clone(), varToIndexMapping.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError((literal!("CreateMemoryMap failed!")).clone(), metamodelica::sourceInfo!())?;
                    Ok((None, iVarToArrayIndexMapping.clone(), iVarToIndexMapping.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((oMemoryMap, oVarToArrayIndexMapping, oVarToIndexMapping))
}

fn createCacheMapOptimized(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iSimCodeVars: SimCodeVar::SimVars, mut iAllSCVarsMapping: metamodelica::Array<Option<SimCodeVar::SimVar>>, mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>, mut iScVarSolvedTaskMapping: metamodelica::Array<i32>, mut iScVarUnsolvedTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iCacheLineSize: i32, mut iAllComponents: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut iSchedule: Arc<HpcOmSimCode::Schedule>, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>, mut iNumberOfThreads: i32, mut iTaskSolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskUnsolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iScVarInfos: metamodelica::Array<ScVarInfo>) -> Result<(CacheMap, metamodelica::Array<(i32, i32)>, i32)> {
    let mut oCacheMap: CacheMap;
    let mut oScVarCLMapping: metamodelica::Array<(i32, i32)>;
    let mut oNumCL: i32 = 0;
    let mut cacheMap: CacheMap;
    let mut scVarCLMapping: metamodelica::Array<(i32, i32)>;
    let mut numCL: i32 = 0;
    let mut tasksOfLevels: Arc<metamodelica::List<HpcOmSimCode::TaskList>> = metamodelica::nil();
    let mut scheduleInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>;
    let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut allTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    (oCacheMap, oScVarCLMapping, oNumCL) = (::match_deref::match_deref! { match &((iTaskGraph.clone(), iTaskGraphMeta.clone(), iAllSCVarsMapping.clone(), iSimCodeVarTypes.clone(), iScVarSolvedTaskMapping.clone(), iScVarUnsolvedTaskMapping.clone(), iCacheLineSize.clone(), iAllComponents.clone(), iSchedule.clone(), iNumberOfThreads.clone(), iTaskSolvedVarsMapping.clone(), iTaskUnsolvedVarsMapping.clone())) {
        (_, _, _, _, _, _, _, _, Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { useFixedAssignments: true, tasksOfLevels }, _, _, _) => {
            println!("{}", (literal!("Creating optimized cache map for fixed level scheduler\n")).clone());
            scheduleInfo = HpcOmScheduler::convertScheduleStrucToInfo(iSchedule.clone(), (iTaskGraph.clone().borrow().len() as i32))?;
            (cacheMap, scVarCLMapping, numCL) = createCacheMapLevelFixedOptimized(iTaskGraph.clone(), iTaskGraphMeta.clone(), iAllSCVarsMapping.clone(), iSimCodeVarTypes.clone(), iScVarSolvedTaskMapping.clone(), iScVarUnsolvedTaskMapping.clone(), iCacheLineSize.clone(), iAllComponents.clone(), tasksOfLevels.clone(), iNumberOfThreads.clone(), scheduleInfo.clone(), iTaskSolvedVarsMapping.clone(), iTaskUnsolvedVarsMapping.clone(), iScVarInfos.clone())?;
            (cacheMap.clone(), scVarCLMapping.clone(), numCL.clone())
        },
        (_, _, _, _, _, _, _, _, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks, .. }, _, _, _) => {
            println!("{}", (literal!("Creating optimized cache map for thread scheduler\n")).clone());
            scheduleInfo = HpcOmScheduler::convertScheduleStrucToInfo(iSchedule.clone(), (iTaskGraph.clone().borrow().len() as i32))?;
            (cacheMap, scVarCLMapping, numCL) = createCacheMapThreadOptimized(iTaskGraph.clone(), iTaskGraphMeta.clone(), iAllSCVarsMapping.clone(), iSimCodeVarTypes.clone(), iScVarSolvedTaskMapping.clone(), iScVarUnsolvedTaskMapping.clone(), iCacheLineSize.clone(), iAllComponents.clone(), threadTasks.clone(), iNumberOfThreads.clone(), scheduleInfo.clone(), iTaskSolvedVarsMapping.clone(), iTaskUnsolvedVarsMapping.clone(), iScVarInfos.clone())?;
            (cacheMap.clone(), scVarCLMapping.clone(), numCL.clone())
        },
        (_, _, _, _, _, _, _, _, Deref @ HpcOmSimCode::Schedule::EMPTYSCHEDULE { tasks: HpcOmSimCode::TaskList::SERIALTASKLIST { tasks: allTasks, .. } }, _, _, _) => {
            println!("{}", (literal!("Creating optimized cache map for empty scheduler\n")).clone());
            threadTasks = arrayCreate(1, allTasks.clone());
            scheduleInfo = HpcOmScheduler::convertScheduleStrucToInfo(iSchedule.clone(), (iTaskGraph.clone().borrow().len() as i32))?;
            (cacheMap, scVarCLMapping, numCL) = createCacheMapThreadOptimized(iTaskGraph.clone(), iTaskGraphMeta.clone(), iAllSCVarsMapping.clone(), iSimCodeVarTypes.clone(), iScVarSolvedTaskMapping.clone(), iScVarUnsolvedTaskMapping.clone(), iCacheLineSize.clone(), iAllComponents.clone(), threadTasks.clone(), 1, scheduleInfo.clone(), iTaskSolvedVarsMapping.clone(), iTaskUnsolvedVarsMapping.clone(), iScVarInfos.clone())?;
            (cacheMap.clone(), scVarCLMapping.clone(), numCL.clone())
        },
        _ => {
            println!("{}", (literal!("No optimized cache map for the selected scheduler avaiable. Using default cacheMap!\n")).clone());
            (cacheMap, scVarCLMapping, numCL) = createCacheMapDefault(iAllSCVarsMapping.clone(), iCacheLineSize.clone(), iSimCodeVars.clone(), iScVarSolvedTaskMapping.clone(), iSchedulerInfo.clone(), iSimCodeVarTypes.clone())?;
            (cacheMap.clone(), scVarCLMapping.clone(), numCL.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oCacheMap, oScVarCLMapping, oNumCL))
}

fn createCacheMapLevelOptimized(mut iAllSCVarsMapping: metamodelica::Array<Option<SimCodeVar::SimVar>>, mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>, mut iScVarTaskMapping: metamodelica::Array<i32>, mut iCacheLineSize: i32, mut iAllComponents: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut iTasksOfLevels: Arc<metamodelica::List<HpcOmSimCode::TaskList>>, mut iNodeSimCodeVarMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<(CacheMap, metamodelica::Array<(i32, i32)>, i32)> {
    let mut oCacheMap: CacheMap;
    let mut oScVarCLMapping: metamodelica::Array<(i32, i32)>;
    let mut oNumCL: i32 = 0;
    let mut cacheMap: CacheMap;
    let mut cacheMapMeta: CacheMapMeta;
    let mut numCL: i32 = 0;
    let mut scVarCLMapping: metamodelica::Array<(i32, i32)>;
    let mut threadCacheLines: metamodelica::Array<Arc<metamodelica::List<CacheLineMap>>>;
    cacheMap = CacheMap::CACHEMAP { cacheLineSize: iCacheLineSize.clone(), cacheVariables: metamodelica::nil(), cacheLinesFloat: metamodelica::nil(), cacheLinesInt: metamodelica::nil(), cacheLinesBool: metamodelica::nil() };
    scVarCLMapping = arrayCreate((iAllSCVarsMapping.clone().borrow().len() as i32), (-1, -1));
    numCL = 0;
    cacheMapMeta = CacheMapMeta { allSCVarsMapping: iAllSCVarsMapping.clone(), simCodeVarTypes: iSimCodeVarTypes.clone(), scVarCLMapping: scVarCLMapping.clone() };
    (_, cacheMap, cacheMapMeta, numCL) = List::fold1(iTasksOfLevels.clone(), (std::sync::Arc::new(createCacheMapLevelOptimized0) as std::sync::Arc<dyn ::std::ops::Fn(HpcOmSimCode::TaskList, metamodelica::Array<Arc<metamodelica::List<i32>>>, (Arc<metamodelica::List<i32>>, CacheMap, CacheMapMeta, i32)) -> Result<(Arc<metamodelica::List<i32>>, CacheMap, CacheMapMeta, i32)> + 'static>), iNodeSimCodeVarMapping.clone(), (metamodelica::nil(), cacheMap.clone(), cacheMapMeta.clone(), numCL.clone()));
    oCacheMap = cacheMap.clone();
    let CacheMapMeta { scVarCLMapping: __pa0, .. } = (cacheMapMeta.clone()) else { bail!("pattern mismatch") };
    oScVarCLMapping = __pa0.clone();
    oNumCL = numCL.clone();
    Ok((oCacheMap, oScVarCLMapping, oNumCL))
}

fn createCacheMapLevelOptimized0(mut iLevelTasks: HpcOmSimCode::TaskList, mut iNodeSimCodeVarMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iInfo: (Arc<metamodelica::List<i32>>, CacheMap, CacheMapMeta, i32)) -> Result<(Arc<metamodelica::List<i32>>, CacheMap, CacheMapMeta, i32)> {
    let mut oInfo: (Arc<metamodelica::List<i32>>, CacheMap, CacheMapMeta, i32);
    let mut createdCL: i32 = 0;
    let mut numCL: i32 = 0;
    let mut cacheLineSize: i32 = 0;
    let mut allCL: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut availableCL: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut availableCLold: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut writtenCL: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut cacheLinesPrevLevel: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut detailedCacheLineInfo: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut cacheMap: CacheMap;
    let mut cacheMapMeta: CacheMapMeta;
    let mut cacheLinesFloat: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    (cacheLinesPrevLevel, cacheMap, cacheMapMeta, numCL) = iInfo.clone();
    allCL = List::intRange(numCL.clone());
    let CacheMap::CACHEMAP { cacheLineSize: __pa0, cacheLinesFloat: __pa1, .. } = (cacheMap.clone()) else { bail!("pattern mismatch") };
    cacheLineSize = __pa0.clone();
    cacheLinesFloat = __pa1.clone();
    availableCLold = List::setDifferenceIntN(allCL.clone(), cacheLinesPrevLevel.clone(), numCL.clone())?;
    detailedCacheLineInfo = createDetailedCacheMapInformation(availableCLold.clone(), cacheLinesFloat.clone(), cacheLineSize.clone());
    detailedCacheLineInfo = detailedCacheLineInfo.clone().reverse();
    (cacheMap, cacheMapMeta, createdCL, detailedCacheLineInfo) = List::fold1(getTaskListTasks(iLevelTasks.clone()), (std::sync::Arc::new(createCacheMapLevelOptimizedForTask) as std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, metamodelica::Array<Arc<metamodelica::List<i32>>>, (CacheMap, CacheMapMeta, i32, Arc<metamodelica::List<(i32, i32)>>)) -> Result<(CacheMap, CacheMapMeta, i32, Arc<metamodelica::List<(i32, i32)>>)> + 'static>), iNodeSimCodeVarMapping.clone(), (cacheMap.clone(), cacheMapMeta.clone(), 0, detailedCacheLineInfo.clone()));
    availableCL = List::map(detailedCacheLineInfo.clone(), std::sync::Arc::new(fnptr!(Util::tuple21, _)));
    writtenCL = List::setDifferenceIntN(availableCLold.clone(), availableCL.clone(), numCL.clone())?;
    writtenCL = listAppend(writtenCL.clone(), if (intLe(numCL.clone() + 1, numCL.clone() + createdCL.clone())) {List::intRange2(numCL.clone() + 1, numCL.clone() + createdCL.clone())} else {metamodelica::nil()});
    oInfo = (writtenCL.clone(), cacheMap.clone(), cacheMapMeta.clone(), numCL.clone() + createdCL.clone());
    Ok(oInfo)
}

fn createCacheMapLevelOptimizedForTask(mut iTask: Arc<HpcOmSimCode::Task>, mut iNodeSimCodeVarMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iInfo: (CacheMap, CacheMapMeta, i32, Arc<metamodelica::List<(i32, i32)>>)) -> Result<(CacheMap, CacheMapMeta, i32, Arc<metamodelica::List<(i32, i32)>>)> {
    let mut oInfo: (CacheMap, CacheMapMeta, i32, Arc<metamodelica::List<(i32, i32)>>);
    let mut nodeIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpInfo: (CacheMap, CacheMapMeta, i32, Arc<metamodelica::List<(i32, i32)>>);
    oInfo = (::match_deref::match_deref! { match &((iTask.clone(), iNodeSimCodeVarMapping.clone(), iInfo.clone())) {
        (Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { nodeIdc, .. }, _, _) => {
            tmpInfo = List::fold(nodeIdc.clone(), Arc::new({ let __pe_b1 = -1; let __pe_b2 = iNodeSimCodeVarMapping.clone(); move |__pe_a0, __pe_a3| appendNodeVarsToCacheMap(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }), iInfo.clone());
            tmpInfo.clone()
        },
        _ => {
            println!("{}", (literal!("createCacheMapLevelOptimized1: Unsupported task type\n")).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oInfo)
}

fn createCacheMapLevelFixedOptimized(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iAllSCVarsMapping: metamodelica::Array<Option<SimCodeVar::SimVar>>, mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>, mut iScVarSolvedTaskMapping: metamodelica::Array<i32>, mut iScVarUnsolvedTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iCacheLineSize: i32, mut iAllComponents: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut iTasksOfLevels: Arc<metamodelica::List<HpcOmSimCode::TaskList>>, mut iNumberOfThreads: i32, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>, mut iTaskSolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskUnsolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iScVarInfos: metamodelica::Array<ScVarInfo>) -> Result<(CacheMap, metamodelica::Array<(i32, i32)>, i32)> {
    let mut oCacheMap: CacheMap;
    let mut oScVarCLMapping: metamodelica::Array<(i32, i32)>;
    let mut oNumCL: i32 = 0;
    let mut cacheMap: CacheMap;
    let mut cacheMapMeta: CacheMapMeta;
    let mut handledVariables: metamodelica::Array<bool>;
    let mut scVarCLMapping: metamodelica::Array<(i32, i32)>;
    let mut threadCacheLines: metamodelica::Array<(Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>)>;
    let mut sharedCacheLines: metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>;
    cacheMap = CacheMap::CACHEMAP { cacheLineSize: iCacheLineSize.clone(), cacheVariables: metamodelica::nil(), cacheLinesFloat: metamodelica::nil(), cacheLinesInt: metamodelica::nil(), cacheLinesBool: metamodelica::nil() };
    scVarCLMapping = arrayCreate((iAllSCVarsMapping.clone().borrow().len() as i32), (-1, -1));
    handledVariables = arrayCreate((iSimCodeVarTypes.clone().borrow().len() as i32), false);
    oNumCL = 0;
    threadCacheLines = arrayCreate(iNumberOfThreads.clone(), (metamodelica::nil(), metamodelica::nil(), metamodelica::nil()));
    sharedCacheLines = arrayCreate(iNumberOfThreads.clone(), ((metamodelica::nil(), metamodelica::nil(), metamodelica::nil()), (metamodelica::nil(), metamodelica::nil(), metamodelica::nil())));
    cacheMapMeta = CacheMapMeta { allSCVarsMapping: iAllSCVarsMapping.clone(), simCodeVarTypes: iSimCodeVarTypes.clone(), scVarCLMapping: scVarCLMapping.clone() };
    (cacheMap, cacheMapMeta, oNumCL, _) = List::fold(iTasksOfLevels.clone(), Arc::new({ let __pe_b1 = iTaskGraph.clone(); let __pe_b2 = iTaskGraphMeta.clone(); let __pe_b3 = iNumberOfThreads.clone(); let __pe_b4 = iScVarInfos.clone(); let __pe_b5 = iTaskSolvedVarsMapping.clone(); let __pe_b6 = iTaskUnsolvedVarsMapping.clone(); let __pe_b7 = handledVariables.clone(); let __pe_b8 = iSchedulerInfo.clone(); let __pe_b9 = threadCacheLines.clone(); let __pe_b10 = sharedCacheLines.clone(); move |__pe_a0, __pe_a11| createCacheMapLevelFixedOptimizedForLevel(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone(), __pe_b7.clone(), __pe_b8.clone(), __pe_b9.clone(), __pe_b10.clone(), __pe_a11) }), (cacheMap.clone(), cacheMapMeta.clone(), oNumCL.clone(), 1));
    for mut threadIdx in 1..=iNumberOfThreads.clone() {
        cacheMap = createCacheMapFromThreadAndSharedCLs(threadCacheLines.clone().borrow()[(threadIdx.clone()-1) as usize].clone(), sharedCacheLines.clone().borrow()[(threadIdx.clone()-1) as usize].clone(), cacheMap.clone())?;
    }
    oCacheMap = cacheMap.clone();
    let CacheMapMeta { scVarCLMapping: __pa0, .. } = (cacheMapMeta.clone()) else { bail!("pattern mismatch") };
    oScVarCLMapping = __pa0.clone();
    Ok((oCacheMap, oScVarCLMapping, oNumCL))
}

fn createCacheMapLevelFixedOptimizedForLevel(mut iLevelTasks: HpcOmSimCode::TaskList, mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iNumberOfThreads: i32, mut iScVarInfos: metamodelica::Array<ScVarInfo>, mut iTaskSolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskUnsolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iHandledVariables: metamodelica::Array<bool>, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>, mut iThreadCacheLines: metamodelica::Array<(Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>)>, mut iSharedCacheLines: metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>, mut iInfo: (CacheMap, CacheMapMeta, i32, i32)) -> Result<(CacheMap, CacheMapMeta, i32, i32)> {
    let mut oInfo: (CacheMap, CacheMapMeta, i32, i32);
    let mut createdCL: i32 = 0;
    let mut numCL: i32 = 0;
    let mut cacheLineSize: i32 = 0;
    let mut level: i32 = 0;
    let mut allCL: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut availableCL: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut availableCLold: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut writtenCL: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut cacheLinesPrevLevel: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut cacheMap: CacheMap;
    let mut cacheMapMeta: CacheMapMeta;
    let mut cacheLinesFloat: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut sharedCacheLines: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheVariables: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut cacheLinesAvailableForLevel: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    (cacheMap, cacheMapMeta, numCL, level) = iInfo.clone();
    let CacheMap::CACHEMAP { cacheVariables: __pa0, .. } = (cacheMap.clone()) else { bail!("pattern mismatch") };
    cacheVariables = __pa0.clone();
    allCL = List::intRange(numCL.clone());
    let CacheMap::CACHEMAP { cacheLineSize: __pa1, cacheLinesFloat: __pa2, .. } = (cacheMap.clone()) else { bail!("pattern mismatch") };
    cacheLineSize = __pa1.clone();
    cacheLinesFloat = __pa2.clone();
    (cacheMap, cacheMapMeta, createdCL) = List::fold(getTaskListTasks(iLevelTasks.clone()), Arc::new({ let __pe_b1 = iTaskGraph.clone(); let __pe_b2 = iTaskGraphMeta.clone(); let __pe_b3 = iSchedulerInfo.clone(); let __pe_b4 = iNumberOfThreads.clone(); let __pe_b5 = level.clone(); let __pe_b6 = iScVarInfos.clone(); let __pe_b7 = iTaskSolvedVarsMapping.clone(); let __pe_b8 = iTaskUnsolvedVarsMapping.clone(); let __pe_b9 = iHandledVariables.clone(); let __pe_b10 = iThreadCacheLines.clone(); let __pe_b11 = iSharedCacheLines.clone(); move |__pe_a0, __pe_a12| createCacheMapLevelFixedOptimizedForTask(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone(), __pe_b7.clone(), __pe_b8.clone(), __pe_b9.clone(), __pe_b10.clone(), __pe_b11.clone(), __pe_a12) }), (cacheMap.clone(), cacheMapMeta.clone(), numCL.clone()));
    let CacheMap::CACHEMAP { cacheVariables: __pa3, .. } = (cacheMap.clone()) else { bail!("pattern mismatch") };
    cacheVariables = __pa3.clone();
    oInfo = (cacheMap.clone(), cacheMapMeta.clone(), createdCL.clone(), level.clone() + 1);
    Ok(oInfo)
}

fn createCacheMapLevelFixedOptimizedForTask(mut iTask: Arc<HpcOmSimCode::Task>, mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>, mut iNumberOfThreads: i32, mut iLevel: i32, mut iScVarInfos: metamodelica::Array<ScVarInfo>, mut iTaskSolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskUnsolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iHandledVariables: metamodelica::Array<bool>, mut iThreadCacheLines: metamodelica::Array<(Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>)>, mut iSharedCacheLines: metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>, mut iInfo: (CacheMap, CacheMapMeta, i32)) -> Result<(CacheMap, CacheMapMeta, i32)> {
    let mut oInfo: (CacheMap, CacheMapMeta, i32);
    let mut nodeIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut solvedVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut unsolvedVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut cacheMap: CacheMap;
    let mut cacheMapMeta: CacheMapMeta;
    let mut tmpInfo: (CacheMap, CacheMapMeta, i32);
    let mut threadIdx: i32 = 0;
    let mut numNewCL: i32 = 0;
    let mut allSCVarsMapping: metamodelica::Array<Option<SimCodeVar::SimVar>>;
    let mut cacheVariables: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    oInfo = (::match_deref::match_deref! { match &((iTask.clone(), iTaskGraph.clone(), iTaskGraphMeta.clone(), iSchedulerInfo.clone(), iNumberOfThreads.clone(), iLevel.clone(), iScVarInfos.clone(), iTaskSolvedVarsMapping.clone(), iTaskUnsolvedVarsMapping.clone(), iHandledVariables.clone(), iThreadCacheLines.clone(), iSharedCacheLines.clone(), iInfo.clone())) {
        (Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { threadIdx: Some(threadIdx), nodeIdc, .. }, _, _, _, _, _, _, _, _, _, _, _, (cacheMap, cacheMapMeta @ CacheMapMeta { allSCVarsMapping, .. }, numNewCL)) => {
            solvedVars = List::flatten(List::map(nodeIdc.clone(), Arc::new(todo!("PARTEVALFUNCTION of arrayGet: function signature not resolved"))));
            unsolvedVars = getUnsolvedVarsByNodeList(nodeIdc.clone(), (iScVarInfos.clone().borrow().len() as i32), iTaskUnsolvedVarsMapping.clone())?;
            tmpInfo = List::fold(listAppend(solvedVars.clone(), unsolvedVars.clone()), Arc::new({ let __pe_b1 = threadIdx.clone(); let __pe_b2 = iScVarInfos.clone(); let __pe_b3 = iHandledVariables.clone(); let __pe_b4: Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, _, metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>) -> Result<Option<(PartlyFilledCacheLine, i32)>> + 'static> = (std::sync::Arc::new(findMatchingSharedCLLevelfix) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, (i32, i32), metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>) -> Result<Option<(PartlyFilledCacheLine, i32)>> + 'static>); let __pe_b5 = (iLevel.clone(), threadIdx.clone()); let __pe_b6: Arc<dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, _) -> Result<PartlyFilledCacheLine> + 'static> = (std::sync::Arc::new(createSharedClLevelFix) as std::sync::Arc<dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, (i32, i32)) -> Result<PartlyFilledCacheLine> + 'static>); let __pe_b7 = iThreadCacheLines.clone(); let __pe_b8 = iSharedCacheLines.clone(); move |__pe_a0, __pe_a9| createCacheMapOptimizedForTask1(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone(), __pe_b7.clone(), __pe_b8.clone(), __pe_a9) }), (cacheMap.clone(), cacheMapMeta.clone(), numNewCL.clone()));
            let CacheMap::CACHEMAP { cacheVariables: __pa0, .. } = (Util::tuple31(tmpInfo.clone())) else { bail!("pattern mismatch") };
            cacheVariables = __pa0.clone();
            tmpInfo.clone()
        },
        (Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { threadIdx: None, nodeIdc, .. }, _, _, _, _, _, _, _, _, _, _, _, _) => {
            println!("{}", (literal!("createCacheMapLevelOptimized1: Calctask without threadIdx given\n")).clone());
            bail!("fail")
        },
        _ => {
            println!("{}", (literal!("createCacheMapLevelOptimized1: Unsupported task type\n")).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oInfo)
}

fn getUnsolvedVarsByNodeList(mut iNodeList: Arc<metamodelica::List<i32>>, mut iVarCount: i32, mut iTaskUnsolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut oUnsolvedVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut varMarks: metamodelica::Array<bool>;
    let mut nodeIdx: i32 = 0;
    let mut varIdx: i32 = 0;
    let mut nodeUnsolvedVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpUnsolvedVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    varMarks = arrayCreate(iVarCount.clone(), false);
    for mut nodeIdx in &*iNodeList.clone() {
        let mut nodeIdx = nodeIdx.clone();
        nodeUnsolvedVars = iTaskUnsolvedVarsMapping.clone().borrow()[(nodeIdx.clone()-1) as usize].clone();
        for mut varIdx in &*nodeUnsolvedVars.clone() {
            let mut varIdx = varIdx.clone();
            if boolNot(varMarks.clone().borrow()[(varIdx.clone()-1) as usize].clone()) {
                tmpUnsolvedVars = cons(varIdx.clone(), tmpUnsolvedVars.clone());
                varMarks = {let _arr = varMarks.clone(); _arr.borrow_mut()[(varIdx.clone()-1) as usize] = true; _arr};
            }
        }
    }
    oUnsolvedVars = tmpUnsolvedVars.clone();
    Ok(oUnsolvedVars)
}

fn createCacheMapThreadOptimized(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iAllSCVarsMapping: metamodelica::Array<Option<SimCodeVar::SimVar>>, mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>, mut iScVarSolvedTaskMapping: metamodelica::Array<i32>, mut iScVarUnsolvedTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iCacheLineSize: i32, mut iAllComponents: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut iThreadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, mut iNumberOfThreads: i32, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>, mut iTaskSolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskUnsolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iScVarInfos: metamodelica::Array<ScVarInfo>) -> Result<(CacheMap, metamodelica::Array<(i32, i32)>, i32)> {
    let mut oCacheMap: CacheMap;
    let mut oScVarCLMapping: metamodelica::Array<(i32, i32)>;
    let mut oNumCL: i32 = 0;
    let mut threadCacheLines: metamodelica::Array<(Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>)>;
    let mut sharedCacheLines: metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>;
    let mut tmpCacheInfo: (CacheMap, CacheMapMeta, i32);
    let mut cacheMap: CacheMap;
    let mut cacheMapMeta: CacheMapMeta;
    let mut scVarCLMapping: metamodelica::Array<(i32, i32)>;
    let mut handledVariables: metamodelica::Array<bool>;
    threadCacheLines = arrayCreate(iNumberOfThreads.clone(), (metamodelica::nil(), metamodelica::nil(), metamodelica::nil()));
    sharedCacheLines = arrayCreate(iNumberOfThreads.clone(), ((metamodelica::nil(), metamodelica::nil(), metamodelica::nil()), (metamodelica::nil(), metamodelica::nil(), metamodelica::nil())));
    handledVariables = arrayCreate((iSimCodeVarTypes.clone().borrow().len() as i32), false);
    cacheMap = CacheMap::CACHEMAP { cacheLineSize: iCacheLineSize.clone(), cacheVariables: metamodelica::nil(), cacheLinesFloat: metamodelica::nil(), cacheLinesInt: metamodelica::nil(), cacheLinesBool: metamodelica::nil() };
    scVarCLMapping = arrayCreate((iAllSCVarsMapping.clone().borrow().len() as i32), (-1, -1));
    oNumCL = 0;
    cacheMapMeta = CacheMapMeta { allSCVarsMapping: iAllSCVarsMapping.clone(), simCodeVarTypes: iSimCodeVarTypes.clone(), scVarCLMapping: scVarCLMapping.clone() };
    tmpCacheInfo = (cacheMap.clone(), cacheMapMeta.clone(), oNumCL.clone());
    for mut threadIdx in 1..=iNumberOfThreads.clone() {
        (cacheMap, cacheMapMeta, oNumCL) = List::fold(iThreadTasks.clone().borrow()[(threadIdx.clone()-1) as usize].clone(), Arc::new({ let __pe_b1 = iTaskGraph.clone(); let __pe_b2 = iTaskGraphMeta.clone(); let __pe_b3 = iSchedulerInfo.clone(); let __pe_b4 = iTaskSolvedVarsMapping.clone(); let __pe_b5 = iTaskUnsolvedVarsMapping.clone(); let __pe_b6 = handledVariables.clone(); let __pe_b7 = iNumberOfThreads.clone(); let __pe_b8: Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, _, metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>) -> Result<Option<(PartlyFilledCacheLine, i32)>> + 'static> = (std::sync::Arc::new(findMatchingSharedCLThread) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, i32, metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>) -> Result<Option<(PartlyFilledCacheLine, i32)>> + 'static>); let __pe_b9 = 0; let __pe_b10: Arc<dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, _) -> Result<PartlyFilledCacheLine> + 'static> = (std::sync::Arc::new(fnptr!(createSharedClThread, Option<PartlyFilledCacheLine>, CacheLineMap, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, i32) -> Result<PartlyFilledCacheLine> + 'static>); let __pe_b11 = threadCacheLines.clone(); let __pe_b12 = sharedCacheLines.clone(); let __pe_b13 = iScVarInfos.clone(); move |__pe_a0, __pe_a14| createCacheMapOptimizedForTask(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone(), __pe_b7.clone(), __pe_b8.clone(), __pe_b9.clone(), __pe_b10.clone(), __pe_b11.clone(), __pe_b12.clone(), __pe_b13.clone(), __pe_a14) }), tmpCacheInfo.clone());
        cacheMap = createCacheMapFromThreadAndSharedCLs(threadCacheLines.clone().borrow()[(threadIdx.clone()-1) as usize].clone(), sharedCacheLines.clone().borrow()[(threadIdx.clone()-1) as usize].clone(), cacheMap.clone())?;
        tmpCacheInfo = (cacheMap.clone(), cacheMapMeta.clone(), oNumCL.clone());
    }
    oCacheMap = Util::tuple31(tmpCacheInfo.clone());
    let CacheMapMeta { scVarCLMapping: __pa0, .. } = (cacheMapMeta.clone()) else { bail!("pattern mismatch") };
    oScVarCLMapping = __pa0.clone();
    Ok((oCacheMap, oScVarCLMapping, oNumCL))
}

fn createCacheMapOptimizedForTask<T: Clone + 'static>(mut iTask: Arc<HpcOmSimCode::Task>, mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>, mut iTaskSolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskUnsolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iHandledVariables: metamodelica::Array<bool>, mut iNumberOfThreads: i32, mut iSharedClSelectFunction: Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, T, metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>) -> Result<Option<(PartlyFilledCacheLine, i32)>> + 'static>, mut iCompareFuncArgument: T, mut iFactoryMethod: Arc<dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, T) -> Result<PartlyFilledCacheLine> + 'static>, mut iThreadCacheLines: metamodelica::Array<(Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>)>, mut iSharedCacheLines: metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>, mut iScVarInfos: metamodelica::Array<ScVarInfo>, mut iInfo: (CacheMap, CacheMapMeta, i32)) -> Result<(CacheMap, CacheMapMeta, i32)> {
    pub type HeuristicFunction<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, T, metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>) -> Result<Option<(PartlyFilledCacheLine, i32)>> + 'static>;

    pub type FactoryMethod<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, T) -> Result<PartlyFilledCacheLine> + 'static>;

    let mut oInfo: (CacheMap, CacheMapMeta, i32);
    let mut threadIdx: i32 = 0;
    let mut taskIdx: i32 = 0;
    let mut solvedVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut unsolvedVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut cacheMap: CacheMap;
    let mut cacheMapMeta: CacheMapMeta;
    let mut numOfCLs: i32 = 0;
    let mut tmpInfo: (CacheMap, CacheMapMeta, i32);
    let mut allSCVarsMapping: metamodelica::Array<Option<SimCodeVar::SimVar>>;
    let mut varInfo: ScVarInfo;
    oInfo = (::match_deref::match_deref! { match &((iTask.clone(), iTaskGraph.clone(), iTaskGraphMeta.clone(), iSchedulerInfo.clone(), iTaskSolvedVarsMapping.clone(), iTaskUnsolvedVarsMapping.clone(), iHandledVariables.clone(), iNumberOfThreads.clone(), iSharedClSelectFunction.clone(), iCompareFuncArgument.clone(), iFactoryMethod.clone(), iThreadCacheLines.clone(), iSharedCacheLines.clone(), iScVarInfos.clone(), iInfo.clone())) {
        (Deref @ HpcOmSimCode::Task::CALCTASK { threadIdx, index: taskIdx, .. }, _, _, _, _, _, _, _, _, _, _, _, _, _, (cacheMap, cacheMapMeta @ CacheMapMeta { allSCVarsMapping, .. }, numOfCLs)) => {
            solvedVars = iTaskSolvedVarsMapping.clone().borrow()[(taskIdx.clone()-1) as usize].clone();
            unsolvedVars = iTaskUnsolvedVarsMapping.clone().borrow()[(taskIdx.clone()-1) as usize].clone();
            vars = List::sort(listAppend(solvedVars.clone(), unsolvedVars.clone()), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            tmpInfo = List::fold(vars.clone(), Arc::new({ let __pe_b1 = threadIdx.clone(); let __pe_b2 = iScVarInfos.clone(); let __pe_b3 = iHandledVariables.clone(); let __pe_b4: Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, _, metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>) -> Result<Option<(PartlyFilledCacheLine, i32)>> + 'static> = iSharedClSelectFunction.clone(); let __pe_b5 = iCompareFuncArgument.clone(); let __pe_b6: Arc<dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, _) -> Result<PartlyFilledCacheLine> + 'static> = iFactoryMethod.clone(); let __pe_b7 = iThreadCacheLines.clone(); let __pe_b8 = iSharedCacheLines.clone(); move |__pe_a0, __pe_a9| createCacheMapOptimizedForTask1(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone(), __pe_b7.clone(), __pe_b8.clone(), __pe_a9) }), (cacheMap.clone(), cacheMapMeta.clone(), numOfCLs.clone()));
            tmpInfo.clone()
        },
        (Deref @ HpcOmSimCode::Task::DEPTASK { sourceTask: _, .. }, _, _, _, _, _, _, _, _, _, _, _, _, _, (cacheMap, cacheMapMeta @ CacheMapMeta { allSCVarsMapping, .. }, numOfCLs)) => iInfo.clone(),
        _ => {
            println!("{}", (literal!("createCacheMapThreadOptimizedForTask failed!\n")).clone());
            iInfo.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oInfo)
}

fn createCacheMapOptimizedForTask1<T: Clone + 'static>(mut iScVar: i32, mut iThreadIdx: i32, mut iScVarInfos: metamodelica::Array<ScVarInfo>, mut iHandledVariables: metamodelica::Array<bool>, mut iSharedClSelectFunction: Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, T, metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>) -> Result<Option<(PartlyFilledCacheLine, i32)>> + 'static>, mut iCompareFuncArgument: T, mut iFactoryMethod: Arc<dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, T) -> Result<PartlyFilledCacheLine> + 'static>, mut iThreadCacheLines: metamodelica::Array<(Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>)>, mut iSharedCacheLines: metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>, mut iInfo: (CacheMap, CacheMapMeta, i32)) -> Result<(CacheMap, CacheMapMeta, i32)> {
    pub type HeuristicFunction<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, T, metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>) -> Result<Option<(PartlyFilledCacheLine, i32)>> + 'static>;

    pub type FactoryMethod<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, T) -> Result<PartlyFilledCacheLine> + 'static>;

    let mut oInfo: (CacheMap, CacheMapMeta, i32);
    let mut isShared: bool = false;
    let mut cacheMap: CacheMap;
    let mut cacheMapMeta: CacheMapMeta;
    let mut numOfCLs: i32 = 0;
    let mut ownerThread: i32 = 0;
    (cacheMap, cacheMapMeta, numOfCLs) = iInfo.clone();
    let ScVarInfo { ownerThread: __pa0, isShared: __pa1 } = (iScVarInfos.clone().borrow()[(iScVar.clone()-1) as usize].clone()) else { bail!("pattern mismatch") };
    ownerThread = __pa0.clone();
    isShared = __pa1.clone();
    if boolAnd(boolNot(boolAnd(intEq(ownerThread.clone(), -1), isShared.clone())), boolNot(iHandledVariables.clone().borrow()[(iScVar.clone()-1) as usize].clone())) {
        if isShared.clone() {
            (cacheMap, cacheMapMeta, numOfCLs) = addVarsToSharedCL(list![iScVar.clone()], iSharedClSelectFunction.clone(), iFactoryMethod.clone(), iThreadIdx.clone(), iCompareFuncArgument.clone(), iSharedCacheLines.clone(), (cacheMap.clone(), cacheMapMeta.clone(), numOfCLs.clone()))?;
        } else {
            (cacheMap, cacheMapMeta, numOfCLs) = addVarsToThreadCL(list![iScVar.clone()], iThreadIdx.clone(), iThreadCacheLines.clone(), (cacheMap.clone(), cacheMapMeta.clone(), numOfCLs.clone()))?;
        }
    }
    let _ = {let _arr = iHandledVariables.clone(); _arr.borrow_mut()[(iScVar.clone()-1) as usize] = true; _arr};
    oInfo = (cacheMap.clone(), cacheMapMeta.clone(), numOfCLs.clone());
    Ok(oInfo)
}

fn createVarInfos(mut iScVarSolvedTaskMapping: metamodelica::Array<i32>, mut iScVarUnsolvedTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>) -> Result<metamodelica::Array<ScVarInfo>> {
    let mut oVarInfos: metamodelica::Array<ScVarInfo>;
    let mut tmpVarInfos: metamodelica::Array<ScVarInfo>;
    let mut scVarIdx: i32 = 0;
    let mut numberOfScVars: i32 = 0;
    numberOfScVars = (iScVarSolvedTaskMapping.clone().borrow().len() as i32);
    tmpVarInfos = arrayCreate(numberOfScVars.clone(), ScVarInfo { ownerThread: -1, isShared: false });
    for mut scVarIdx in 1..=numberOfScVars.clone() {
        tmpVarInfos = {let _arr = tmpVarInfos.clone(); _arr.borrow_mut()[(scVarIdx.clone()-1) as usize] = getVarInfoByScVarIdx(scVarIdx.clone(), iScVarSolvedTaskMapping.clone(), iScVarUnsolvedTaskMapping.clone(), iSchedulerInfo.clone())?; _arr};
    }
    oVarInfos = tmpVarInfos.clone();
    Ok(oVarInfos)
}

fn getVarInfoByScVarIdx(mut iScVarIdx: i32, mut iScVarSolvedTaskMapping: metamodelica::Array<i32>, mut iScVarUnsolvedTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>) -> Result<ScVarInfo> {
    let mut oVarInfo: ScVarInfo;
    let mut solvingThreadIdx: i32 = 0;
    let mut solvingTaskIdx: i32 = 0;
    let mut listLen: i32 = 0;
    let mut owner: i32 = -1;
    let mut isShared: bool = false;
    let mut threads: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut unsolvingThreadIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut unsolvingTaskIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    solvingTaskIdx = iScVarSolvedTaskMapping.clone().borrow()[(iScVarIdx.clone()-1) as usize].clone();
    unsolvingTaskIdc = iScVarUnsolvedTaskMapping.clone().borrow()[(iScVarIdx.clone()-1) as usize].clone();
    if intGt(solvingTaskIdx.clone(), 0) {
        solvingThreadIdx = Util::tuple31(iSchedulerInfo.clone().borrow()[(solvingTaskIdx.clone()-1) as usize].clone());
        owner = solvingThreadIdx.clone();
        threads = cons(owner.clone(), threads.clone());
    }
    listLen = (unsolvingTaskIdc.clone().len() as i32);
    unsolvingThreadIdc = List::map(List::map(unsolvingTaskIdc.clone(), Arc::new(todo!("PARTEVALFUNCTION of arrayGet: function signature not resolved"))), std::sync::Arc::new(fnptr!(Util::tuple31, _)));
    if intEq(listLen.clone(), 1) {
        if intLt(owner.clone(), 0) {
            owner = listHead(unsolvingThreadIdc.clone())?;
            threads = cons(owner.clone(), threads.clone());
        } else {
            isShared = true;
        }
    }
    if intGt(listLen.clone(), 1) {
        threads = List::unique(listAppend(unsolvingThreadIdc.clone(), threads.clone()));
        isShared = true;
    }
    oVarInfo = ScVarInfo { ownerThread: owner.clone(), isShared: isShared.clone() };
    Ok(oVarInfo)
}

fn addVarsToThreadCL(mut iNodeVars: Arc<metamodelica::List<i32>>, mut iThreadIdx: i32, mut iThreadCacheLines: metamodelica::Array<(Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>)>, mut iInfo: (CacheMap, CacheMapMeta, i32)) -> Result<(CacheMap, CacheMapMeta, i32)> {
    let mut oInfo: (CacheMap, CacheMapMeta, i32);
    let mut lastCL: CacheLineMap;
    let mut cacheVariable: SimCodeVar::SimVar;
    let mut allSCVarsMapping: metamodelica::Array<Option<SimCodeVar::SimVar>>;
    let mut varIdx: i32 = 0;
    let mut varDataType: i32 = 0;
    let mut varNumBytesRequired: i32 = 0;
    let mut numCLs: i32 = 0;
    let mut cacheLineSize: i32 = 0;
    let mut simCodeVarTypes: metamodelica::Array<(i32, i32, i32)>;
    let mut scVarCLMapping: metamodelica::Array<(i32, i32)>;
    let mut fullCLs: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut threadCacheLines: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheVariables: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut cacheLinesFloat: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheLinesInt: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheLinesBool: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut lastCLidx: i32 = 0;
    let mut lastCLnumBytesFree: i32 = 0;
    let mut lastCLentries: Arc<metamodelica::List<CacheLineEntry>> = metamodelica::nil();
    let mut varEntry: CacheLineEntry;
    let mut cacheVarName: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut threadCacheLinesFloat: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut threadCacheLinesInt: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut threadCacheLinesBool: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let (CacheMap::CACHEMAP { cacheLinesBool: __pa0, cacheLinesInt: __pa1, cacheLinesFloat: __pa2, cacheVariables: __pa3, cacheLineSize: __pa4 }, CacheMapMeta { scVarCLMapping: __pa5, simCodeVarTypes: __pa6, allSCVarsMapping: __pa7 }, __pa8) = (iInfo.clone()) else { bail!("pattern mismatch") };
    cacheLinesBool = __pa0.clone();
    cacheLinesInt = __pa1.clone();
    cacheLinesFloat = __pa2.clone();
    cacheVariables = __pa3.clone();
    cacheLineSize = __pa4.clone();
    scVarCLMapping = __pa5.clone();
    simCodeVarTypes = __pa6.clone();
    allSCVarsMapping = __pa7.clone();
    numCLs = __pa8.clone();
    for mut varIdx in &*iNodeVars.clone() {
        let mut varIdx = varIdx.clone();
        (varDataType, varNumBytesRequired, _) = simCodeVarTypes.clone().borrow()[(varIdx.clone()-1) as usize].clone();
        (threadCacheLinesFloat, threadCacheLinesInt, threadCacheLinesBool, threadCacheLines) = getCacheLineForVarType(varDataType.clone(), iThreadCacheLines.clone().borrow()[(iThreadIdx.clone()-1) as usize].clone());
        if !(threadCacheLines.clone().is_empty()) {
            let (__pa9, __pa10) = ::match_deref::match_deref! { match &(threadCacheLines.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa9, tail: __pa10 } => (__pa9.clone(), __pa10.clone()),
                _ => bail!("pattern mismatch"),
            } };
            lastCL = __pa9.clone();
            fullCLs = __pa10.clone();
        } else {
            lastCLidx = numCLs.clone() + 1;
            lastCLnumBytesFree = cacheLineSize.clone();
            lastCLentries = metamodelica::nil();
            lastCL = CacheLineMap { entries: lastCLentries.clone(), numBytesFree: lastCLnumBytesFree.clone(), idx: lastCLidx.clone() };
            numCLs = numCLs.clone() + 1;
            fullCLs = metamodelica::nil();
        }
        let CacheLineMap { entries: __pa11, numBytesFree: __pa12, idx: __pa13 } = (lastCL.clone()) else { bail!("pattern mismatch") };
        lastCLentries = __pa11.clone();
        lastCLnumBytesFree = __pa12.clone();
        lastCLidx = __pa13.clone();
        if intLt(lastCLnumBytesFree.clone(), varNumBytesRequired.clone()) {
            fullCLs = cons(lastCL.clone(), fullCLs.clone());
            lastCLidx = numCLs.clone() + 1;
            lastCLnumBytesFree = cacheLineSize.clone();
            lastCLentries = metamodelica::nil();
            lastCL = CacheLineMap { entries: lastCLentries.clone(), numBytesFree: lastCLnumBytesFree.clone(), idx: lastCLidx.clone() };
            numCLs = numCLs.clone() + 1;
        }
        let Some(ref __pa15 @ SimCodeVar::SIMVAR { name: ref __pa14, .. }) = (allSCVarsMapping.clone().borrow()[(varIdx.clone()-1) as usize].clone()) else { bail!("pattern mismatch") };
        cacheVarName = __pa14.clone();
        cacheVariable = __pa15.clone();
        cacheVariables = cons(cacheVariable.clone(), cacheVariables.clone());
        scVarCLMapping = {let _arr = scVarCLMapping.clone(); _arr.borrow_mut()[(varIdx.clone()-1) as usize] = (lastCLidx.clone(), varDataType.clone()); _arr};
        varEntry = CacheLineEntry { threadOwner: iThreadIdx.clone(), scVarIdx: (cacheVariables.clone().len() as i32), size: varNumBytesRequired.clone(), dataType: varDataType.clone(), start: cacheLineSize.clone() - lastCLnumBytesFree.clone() };
        lastCL = CacheLineMap { entries: cons(varEntry.clone(), lastCLentries.clone()), numBytesFree: lastCLnumBytesFree.clone() - varNumBytesRequired.clone(), idx: lastCLidx.clone() };
        let _ = {let _arr = iThreadCacheLines.clone(); _arr.borrow_mut()[(iThreadIdx.clone()-1) as usize] = contractCacheLineForVarType(varDataType.clone(), threadCacheLinesFloat.clone(), threadCacheLinesInt.clone(), threadCacheLinesBool.clone(), cons(lastCL.clone(), fullCLs.clone())); _arr};
    }
    oInfo = (CacheMap::CACHEMAP { cacheLineSize: cacheLineSize.clone(), cacheVariables: cacheVariables.clone(), cacheLinesFloat: cacheLinesFloat.clone(), cacheLinesInt: cacheLinesInt.clone(), cacheLinesBool: cacheLinesBool.clone() }, CacheMapMeta { allSCVarsMapping: allSCVarsMapping.clone(), simCodeVarTypes: simCodeVarTypes.clone(), scVarCLMapping: scVarCLMapping.clone() }, numCLs.clone());
    Ok(oInfo)
}

fn getCacheLineForVarType(mut iVarDataType: i32, mut iCacheLinesForTypes: CacheLines) -> (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>) {
    let mut oCacheLinesFloat: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut oCacheLinesInt: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut oCacheLinesBool: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut oVarCacheLines: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    (oCacheLinesFloat, oCacheLinesInt, oCacheLinesBool) = iCacheLinesForTypes.clone();
    if intEq(iVarDataType.clone(), VARDATATYPE_FLOAT.clone()) {
        (oVarCacheLines, _, _) = iCacheLinesForTypes.clone();
    } else {
        if intEq(iVarDataType.clone(), VARDATATYPE_INTEGER.clone()) {
            (_, oVarCacheLines, _) = iCacheLinesForTypes.clone();
        } else {
            if intEq(iVarDataType.clone(), VARDATATYPE_BOOLEAN.clone()) {
                (_, _, oVarCacheLines) = iCacheLinesForTypes.clone();
            } else {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getCacheLineForVarType: Found Variable with unknown type ( ")); __mm_s.push_str(&*intString(iVarDataType.clone())); __mm_s.push_str(&*literal!(")!\n")); ArcStr::from(__mm_s) }).clone());
            }
        }
    }
    (oCacheLinesFloat, oCacheLinesInt, oCacheLinesBool, oVarCacheLines)
}

fn contractCacheLineForVarType(mut iVarDataType: i32, mut iCacheLinesFloat: Arc<metamodelica::List<CacheLineMap>>, mut iCacheLinesInt: Arc<metamodelica::List<CacheLineMap>>, mut iCacheLinesBool: Arc<metamodelica::List<CacheLineMap>>, mut iVarCacheLines: Arc<metamodelica::List<CacheLineMap>>) -> CacheLines {
    let mut oContractedCacheLines: CacheLines;
    if intEq(iVarDataType.clone(), VARDATATYPE_FLOAT.clone()) {
        oContractedCacheLines = (iVarCacheLines.clone(), iCacheLinesInt.clone(), iCacheLinesBool.clone());
    } else {
        if intEq(iVarDataType.clone(), VARDATATYPE_INTEGER.clone()) {
            oContractedCacheLines = (iCacheLinesFloat.clone(), iVarCacheLines.clone(), iCacheLinesBool.clone());
        } else {
            if intEq(iVarDataType.clone(), VARDATATYPE_BOOLEAN.clone()) {
                oContractedCacheLines = (iCacheLinesFloat.clone(), iCacheLinesInt.clone(), iVarCacheLines.clone());
            }
        }
    }
    oContractedCacheLines
}

fn addVarsToSharedCL<T: Clone + 'static>(mut iNodeVars: Arc<metamodelica::List<i32>>, mut iSharedClSelectFunction: Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, T, metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>) -> Result<Option<(PartlyFilledCacheLine, i32)>> + 'static>, mut iFactoryMethod: Arc<dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, T) -> Result<PartlyFilledCacheLine> + 'static>, mut iThreadIdx: i32, mut iCompareFuncArgument: T, mut iSharedCacheLines: metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>, mut iInfo: (CacheMap, CacheMapMeta, i32)) -> Result<(CacheMap, CacheMapMeta, i32)> {
    pub type HeuristicFunction<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, T, metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>) -> Result<Option<(PartlyFilledCacheLine, i32)>> + 'static>;

    pub type FactoryMethod<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, T) -> Result<PartlyFilledCacheLine> + 'static>;

    let mut oInfo: (CacheMap, CacheMapMeta, i32);
    let mut lastCL: CacheLineMap;
    let mut cacheVariable: SimCodeVar::SimVar;
    let mut allSCVarsMapping: metamodelica::Array<Option<SimCodeVar::SimVar>>;
    let mut varIdx: i32 = 0;
    let mut varDataType: i32 = 0;
    let mut varNumBytesRequired: i32 = 0;
    let mut numOfCLs: i32 = 0;
    let mut cacheLineSize: i32 = 0;
    let mut varSize: i32 = 0;
    let mut simCodeVarTypes: metamodelica::Array<(i32, i32, i32)>;
    let mut scVarCLMapping: metamodelica::Array<(i32, i32)>;
    let mut fullCLs: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut threadCacheLines: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheVariables: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut cacheLinesFloat: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut matchedCacheLineIdx: i32 = 0;
    let mut cacheMap: CacheMap;
    let mut cacheMapMeta: CacheMapMeta;
    let mut matchedCacheLine: Option<(PartlyFilledCacheLine, i32)> = None;
    let (ref __pa3 @ CacheMap::CACHEMAP { cacheLinesFloat: ref __pa0, cacheVariables: ref __pa1, cacheLineSize: ref __pa2, .. }, ref __pa7 @ CacheMapMeta { scVarCLMapping: ref __pa4, simCodeVarTypes: ref __pa5, allSCVarsMapping: ref __pa6 }, __pa8) = (iInfo.clone()) else { bail!("pattern mismatch") };
    cacheLinesFloat = __pa0.clone();
    cacheVariables = __pa1.clone();
    cacheLineSize = __pa2.clone();
    cacheMap = __pa3.clone();
    scVarCLMapping = __pa4.clone();
    simCodeVarTypes = __pa5.clone();
    allSCVarsMapping = __pa6.clone();
    cacheMapMeta = __pa7.clone();
    numOfCLs = __pa8.clone();
    for mut varIdx in &*iNodeVars.clone() {
        let mut varIdx = varIdx.clone();
        (varDataType, varSize, _) = simCodeVarTypes.clone().borrow()[(varIdx.clone()-1) as usize].clone();
        matchedCacheLine = iSharedClSelectFunction(varIdx.clone(), varSize.clone(), varDataType.clone(), iThreadIdx.clone(), iCompareFuncArgument.clone(), iSharedCacheLines.clone())?;
        (cacheMap, cacheMapMeta, numOfCLs) = addVarsToSharedCL0(matchedCacheLine.clone(), varIdx.clone(), iFactoryMethod.clone(), iCompareFuncArgument.clone(), iThreadIdx.clone(), iSharedCacheLines.clone(), (cacheMap.clone(), cacheMapMeta.clone(), numOfCLs.clone()))?;
    }
    oInfo = (cacheMap.clone(), cacheMapMeta.clone(), numOfCLs.clone());
    Ok(oInfo)
}

fn addVarsToSharedCL0<T: Clone + 'static>(mut iMatchedCacheLine: Option<(PartlyFilledCacheLine, i32)>, mut iVarIdx: i32, mut iFactoryMethod: Arc<dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, T) -> Result<PartlyFilledCacheLine> + 'static>, mut iAdditionalArgument: T, mut iThreadIdx: i32, mut iSharedCacheLines: metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>, mut iInfo: (CacheMap, CacheMapMeta, i32)) -> Result<(CacheMap, CacheMapMeta, i32)> {
    pub type FactoryMethod<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, T) -> Result<PartlyFilledCacheLine> + 'static>;

    let mut oInfo: (CacheMap, CacheMapMeta, i32);
    let mut threadPartlyFilledCacheLines: PartlyFilledCacheLines;
    let mut partlyFilledClFloat: Arc<metamodelica::List<PartlyFilledCacheLine>> = metamodelica::nil();
    let mut partlyFilledClInt: Arc<metamodelica::List<PartlyFilledCacheLine>> = metamodelica::nil();
    let mut partlyFilledClBool: Arc<metamodelica::List<PartlyFilledCacheLine>> = metamodelica::nil();
    let mut threadFullyFilledCacheLines: CacheLines;
    let mut fullyFilledClFloat: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut fullyFilledClInt: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut fullyFilledClBool: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut allSCVarsMapping: metamodelica::Array<Option<SimCodeVar::SimVar>>;
    let mut simCodeVarTypes: metamodelica::Array<(i32, i32, i32)>;
    let mut scVarCLMapping: metamodelica::Array<(i32, i32)>;
    let mut partlyFilledCacheLine: PartlyFilledCacheLine;
    let mut partlyFilledCacheLineOption: Option<PartlyFilledCacheLine> = None;
    let mut matchedClIndex: i32 = 0;
    let mut numOfCLs: i32 = 0;
    let mut clMapIdx: i32 = 0;
    let mut clMapNumBytesFree: i32 = 0;
    let mut varDataType: i32 = 0;
    let mut varSize: i32 = 0;
    let mut cacheLineSize: i32 = 0;
    let mut cacheVariables: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut cacheLinesFloat: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheLinesInt: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheLinesBool: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut clMapEntries: Arc<metamodelica::List<CacheLineEntry>> = metamodelica::nil();
    let mut entry: CacheLineEntry;
    let mut cacheLineMap: CacheLineMap;
    let mut cacheVariable: SimCodeVar::SimVar;
    let (CacheMap::CACHEMAP { cacheLinesBool: __pa0, cacheLinesInt: __pa1, cacheLinesFloat: __pa2, cacheVariables: __pa3, cacheLineSize: __pa4 }, CacheMapMeta { scVarCLMapping: __pa5, simCodeVarTypes: __pa6, allSCVarsMapping: __pa7 }, __pa8) = (iInfo.clone()) else { bail!("pattern mismatch") };
    cacheLinesBool = __pa0.clone();
    cacheLinesInt = __pa1.clone();
    cacheLinesFloat = __pa2.clone();
    cacheVariables = __pa3.clone();
    cacheLineSize = __pa4.clone();
    scVarCLMapping = __pa5.clone();
    simCodeVarTypes = __pa6.clone();
    allSCVarsMapping = __pa7.clone();
    numOfCLs = __pa8.clone();
    (varDataType, varSize, _) = simCodeVarTypes.clone().borrow()[(iVarIdx.clone()-1) as usize].clone();
    (threadPartlyFilledCacheLines, threadFullyFilledCacheLines) = iSharedCacheLines.clone().borrow()[(iThreadIdx.clone()-1) as usize].clone();
    (partlyFilledClFloat, partlyFilledClInt, partlyFilledClBool) = threadPartlyFilledCacheLines.clone();
    (fullyFilledClFloat, fullyFilledClInt, fullyFilledClBool) = threadFullyFilledCacheLines.clone();
    if isSome(iMatchedCacheLine.clone()) {
        clMapIdx = numOfCLs.clone();
        let Some((__pa9, __pa10)) = (iMatchedCacheLine.clone()) else { bail!("pattern mismatch") };
        partlyFilledCacheLine = __pa9.clone();
        matchedClIndex = __pa10.clone();
        partlyFilledCacheLineOption = Some(partlyFilledCacheLine.clone());
        let CacheLineMap { idx: __pa11, numBytesFree: __pa12, entries: __pa13 } = (getCacheLineMapOfPartlyFilledCacheLine(partlyFilledCacheLine.clone())?) else { bail!("pattern mismatch") };
        clMapIdx = __pa11.clone();
        clMapNumBytesFree = __pa12.clone();
        clMapEntries = __pa13.clone();
    } else {
        numOfCLs = numOfCLs.clone() + 1;
        partlyFilledCacheLineOption = None;
        clMapIdx = numOfCLs.clone();
        clMapNumBytesFree = cacheLineSize.clone();
        clMapEntries = metamodelica::nil();
        matchedClIndex = -1;
    }
    clMapNumBytesFree = clMapNumBytesFree.clone() - varSize.clone();
    let Some(__pa14) = (allSCVarsMapping.clone().borrow()[(iVarIdx.clone()-1) as usize].clone()) else { bail!("pattern mismatch") };
    cacheVariable = __pa14.clone();
    cacheVariables = cons(cacheVariable.clone(), cacheVariables.clone());
    entry = CacheLineEntry { start: cacheLineSize.clone() - clMapNumBytesFree.clone() - varSize.clone(), dataType: varDataType.clone(), size: varSize.clone(), scVarIdx: (cacheVariables.clone().len() as i32), threadOwner: iThreadIdx.clone() };
    cacheLineMap = CacheLineMap { idx: clMapIdx.clone(), numBytesFree: clMapNumBytesFree.clone(), entries: cons(entry.clone(), clMapEntries.clone()) };
    partlyFilledCacheLine = iFactoryMethod(partlyFilledCacheLineOption.clone(), cacheLineMap.clone(), iAdditionalArgument.clone())?;
    scVarCLMapping = {let _arr = scVarCLMapping.clone(); _arr.borrow_mut()[(iVarIdx.clone()-1) as usize] = (clMapIdx.clone(), varDataType.clone()); _arr};
    if intEq(clMapNumBytesFree.clone(), 0) {
        if intEq(varDataType.clone(), VARDATATYPE_FLOAT.clone()) {
            partlyFilledClFloat = listDelete(partlyFilledClFloat.clone(), matchedClIndex.clone())?;
            fullyFilledClFloat = cons(cacheLineMap.clone(), fullyFilledClFloat.clone());
        } else {
            if intEq(varDataType.clone(), VARDATATYPE_INTEGER.clone()) {
                partlyFilledClInt = listDelete(partlyFilledClInt.clone(), matchedClIndex.clone())?;
                fullyFilledClInt = cons(cacheLineMap.clone(), fullyFilledClInt.clone());
            } else {
                partlyFilledClBool = listDelete(partlyFilledClBool.clone(), matchedClIndex.clone())?;
                fullyFilledClBool = cons(cacheLineMap.clone(), fullyFilledClBool.clone());
            }
        }
    } else {
        if intNe(matchedClIndex.clone(), -1) {
            if intEq(varDataType.clone(), VARDATATYPE_FLOAT.clone()) {
                partlyFilledClFloat = List::set(partlyFilledClFloat.clone(), matchedClIndex.clone(), partlyFilledCacheLine.clone())?;
            } else {
                if intEq(varDataType.clone(), VARDATATYPE_INTEGER.clone()) {
                    partlyFilledClInt = List::set(partlyFilledClInt.clone(), matchedClIndex.clone(), partlyFilledCacheLine.clone())?;
                } else {
                    partlyFilledClBool = List::set(partlyFilledClBool.clone(), matchedClIndex.clone(), partlyFilledCacheLine.clone())?;
                }
            }
        } else {
            if intEq(varDataType.clone(), VARDATATYPE_FLOAT.clone()) {
                partlyFilledClFloat = cons(partlyFilledCacheLine.clone(), partlyFilledClFloat.clone());
            } else {
                if intEq(varDataType.clone(), VARDATATYPE_INTEGER.clone()) {
                    partlyFilledClInt = cons(partlyFilledCacheLine.clone(), partlyFilledClInt.clone());
                } else {
                    partlyFilledClBool = cons(partlyFilledCacheLine.clone(), partlyFilledClBool.clone());
                }
            }
        }
    }
    let _ = {let _arr = iSharedCacheLines.clone(); _arr.borrow_mut()[(iThreadIdx.clone()-1) as usize] = ((partlyFilledClFloat.clone(), partlyFilledClInt.clone(), partlyFilledClBool.clone()), (fullyFilledClFloat.clone(), fullyFilledClInt.clone(), fullyFilledClBool.clone())); _arr};
    oInfo = (CacheMap::CACHEMAP { cacheLinesBool: cacheLinesBool.clone(), cacheLinesInt: cacheLinesInt.clone(), cacheLinesFloat: cacheLinesFloat.clone(), cacheVariables: cacheVariables.clone(), cacheLineSize: cacheLineSize.clone() }, CacheMapMeta { scVarCLMapping: scVarCLMapping.clone(), simCodeVarTypes: simCodeVarTypes.clone(), allSCVarsMapping: allSCVarsMapping.clone() }, numOfCLs.clone());
    Ok(oInfo)
}

fn getPartlyFilledCLByVarType(mut iVarType: i32, mut iSharedCacheLines: PartlyFilledCacheLines) -> Arc<metamodelica::List<PartlyFilledCacheLine>> {
    let mut oSharedCacheLinesForType: Arc<metamodelica::List<PartlyFilledCacheLine>> = metamodelica::nil();
    if intEq(iVarType.clone(), VARDATATYPE_FLOAT.clone()) {
        oSharedCacheLinesForType = Util::tuple31(iSharedCacheLines.clone());
    } else {
        if intEq(iVarType.clone(), VARDATATYPE_INTEGER.clone()) {
            oSharedCacheLinesForType = Util::tuple32(iSharedCacheLines.clone());
        } else {
            oSharedCacheLinesForType = Util::tuple33(iSharedCacheLines.clone());
        }
    }
    oSharedCacheLinesForType
}

fn findMatchingSharedCLLevelfix(mut iNodeVar: i32, mut iVarSize: i32, mut iVarType: i32, mut iThreadIdx: i32, mut iLevelThreadIdx: (i32, i32), mut iSharedCacheLines: metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>) -> Result<Option<(PartlyFilledCacheLine, i32)>> {
    let mut oMatchedCacheLine: Option<(PartlyFilledCacheLine, i32)> = None;
    let mut partlyFilledCacheLines: Arc<metamodelica::List<PartlyFilledCacheLine>> = metamodelica::nil();
    let mut sharedCacheLines: PartlyFilledCacheLines;
    let mut levelIdx: i32 = 0;
    (levelIdx, _) = iLevelThreadIdx.clone();
    sharedCacheLines = Util::tuple21(iSharedCacheLines.clone().borrow()[(iThreadIdx.clone()-1) as usize].clone());
    oMatchedCacheLine = None;
    partlyFilledCacheLines = getPartlyFilledCLByVarType(iVarType.clone(), sharedCacheLines.clone());
    oMatchedCacheLine = findMatchingSharedCLLevelfix0(iNodeVar.clone(), iVarSize.clone(), levelIdx.clone(), iThreadIdx.clone(), 1, partlyFilledCacheLines.clone());
    Ok(oMatchedCacheLine)
}

fn findMatchingSharedCLLevelfix0(mut iNodeVar: i32, mut iVarSize: i32, mut iLevelIdx: i32, mut iThreadIdx: i32, mut iCurrentListIdx: i32, mut iSharedCacheLines: Arc<metamodelica::List<PartlyFilledCacheLine>>) -> Option<(PartlyFilledCacheLine, i32)> {
    let mut oMatchedCacheLine: Option<(PartlyFilledCacheLine, i32)> = None;
    let mut head: PartlyFilledCacheLine;
    let mut rest: Arc<metamodelica::List<PartlyFilledCacheLine>> = metamodelica::nil();
    let mut tmpMatchedCacheLine: Option<(PartlyFilledCacheLine, i32)> = None;
    let mut cacheLineMap: CacheLineMap;
    let mut numBytesFree: i32 = 0;
    let mut prefetchLevel: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut writeLevel: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    oMatchedCacheLine = (::match_deref::match_deref! { match &((iNodeVar.clone(), iVarSize.clone(), iLevelIdx.clone(), iThreadIdx.clone(), iCurrentListIdx.clone(), iSharedCacheLines.clone())) {
        (_, _, _, _, _, Deref @ metamodelica::List::Cons { head: head @ PartlyFilledCacheLine::PARTLYFILLEDCACHELINE_LEVEL { writeLevel, prefetchLevel, cacheLineMap: cacheLineMap @ CacheLineMap { numBytesFree, .. } }, tail: rest }) => {
            if boolOr(intLt(numBytesFree.clone(), iVarSize.clone()), List::exist1(prefetchLevel.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), iLevelIdx.clone())) {
                tmpMatchedCacheLine = findMatchingSharedCLLevelfix0(iNodeVar.clone(), iVarSize.clone(), iLevelIdx.clone(), iThreadIdx.clone(), iCurrentListIdx.clone() + 1, rest.clone());
            } else {
                if List::any(writeLevel.clone(), Arc::new({ let __pe_b1 = iLevelIdx.clone(); let __pe_b2 = iThreadIdx.clone(); move |__pe_a0| Ok(isCLWrittenByOtherThread(__pe_a0, __pe_b1.clone(), __pe_b2.clone())) })) {
                    tmpMatchedCacheLine = findMatchingSharedCLLevelfix0(iNodeVar.clone(), iVarSize.clone(), iLevelIdx.clone(), iThreadIdx.clone(), iCurrentListIdx.clone() + 1, rest.clone());
                } else {
                    if List::any(writeLevel.clone(), Arc::new({ let __pe_b1 = iLevelIdx.clone() - 1; let __pe_b2 = iThreadIdx.clone(); move |__pe_a0| Ok(isCLWrittenByOtherThread(__pe_a0, __pe_b1.clone(), __pe_b2.clone())) })) {
                        tmpMatchedCacheLine = findMatchingSharedCLLevelfix0(iNodeVar.clone(), iVarSize.clone(), iLevelIdx.clone(), iThreadIdx.clone(), iCurrentListIdx.clone() + 1, rest.clone());
                    } else {
                        tmpMatchedCacheLine = Some((head.clone(), iCurrentListIdx.clone()));
                    }
                }
            }
            tmpMatchedCacheLine.clone()
        },
        (_, _, _, _, _, Deref @ metamodelica::List::Nil) => None,
        _ => {
            println!("{}", (literal!("findMatchingSharedCLLevelfix0: Unknown partly filled cache line type given.\n")).clone());
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oMatchedCacheLine
}

fn findMatchingSharedCLThread(mut iNodeVar: i32, mut iVarSize: i32, mut iVarType: i32, mut iThreadIdx: i32, mut iAdditionalArgument: i32, mut iSharedCacheLines: metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>) -> Result<Option<(PartlyFilledCacheLine, i32)>> {
    let mut oMatchedCacheLine: Option<(PartlyFilledCacheLine, i32)> = None;
    let mut partlyFilledCacheLines: Arc<metamodelica::List<PartlyFilledCacheLine>> = metamodelica::nil();
    let mut partlyFilledCL: PartlyFilledCacheLine;
    let mut numBytesFree: i32 = 0;
    let mut listIdx: i32 = 0;
    oMatchedCacheLine = None;
    partlyFilledCacheLines = getPartlyFilledCLByVarType(iVarType.clone(), Util::tuple21(iSharedCacheLines.clone().borrow()[(iThreadIdx.clone()-1) as usize].clone()));
    listIdx = 1;
    for mut partlyFilledCL in &*partlyFilledCacheLines.clone() {
        let mut partlyFilledCL = partlyFilledCL.clone();
        let CacheLineMap { numBytesFree: __pa0, .. } = (getCacheLineMapOfPartlyFilledCacheLine(partlyFilledCL.clone())?) else { bail!("pattern mismatch") };
        numBytesFree = __pa0.clone();
        if intGe(numBytesFree.clone(), iVarSize.clone()) {
            oMatchedCacheLine = Some((partlyFilledCL.clone(), listIdx.clone()));
            break;
        }
        listIdx = listIdx.clone() + 1;
    }
    Ok(oMatchedCacheLine)
}

fn createSharedClThread(mut iOldPartlyFilledCacheLine: Option<PartlyFilledCacheLine>, mut iCacheLineMap: CacheLineMap, mut iAdditionalArgument: i32) -> PartlyFilledCacheLine {
    let mut oCreatedCacheLine: PartlyFilledCacheLine;
    oCreatedCacheLine = PartlyFilledCacheLine::PARTLYFILLEDCACHELINE_THREAD { cacheLineMap: iCacheLineMap.clone() };
    oCreatedCacheLine
}

fn createSharedClLevelFix(mut iOldPartlyFilledCacheLine: Option<PartlyFilledCacheLine>, mut iCacheLineMap: CacheLineMap, mut iLevelThreadIdx: (i32, i32)) -> Result<PartlyFilledCacheLine> {
    let mut oCreatedCacheLine: PartlyFilledCacheLine;
    let mut prefetchLevel: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut writeLevel: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut levelIdx: i32 = 0;
    let mut threadIdx: i32 = 0;
    (levelIdx, threadIdx) = iLevelThreadIdx.clone();
    if isSome(iOldPartlyFilledCacheLine.clone()) {
        let Some(PartlyFilledCacheLine::PARTLYFILLEDCACHELINE_LEVEL { writeLevel: __pa0, prefetchLevel: __pa1, .. }) = (iOldPartlyFilledCacheLine.clone()) else { bail!("pattern mismatch") };
        writeLevel = __pa0.clone();
        prefetchLevel = __pa1.clone();
    } else {
        prefetchLevel = metamodelica::nil();
        writeLevel = metamodelica::nil();
    }
    if intGt(levelIdx.clone() - 1, 0) {
        prefetchLevel = cons(levelIdx.clone() - 1, prefetchLevel.clone());
    }
    writeLevel = cons((levelIdx.clone(), threadIdx.clone()), writeLevel.clone());
    oCreatedCacheLine = PartlyFilledCacheLine::PARTLYFILLEDCACHELINE_LEVEL { cacheLineMap: iCacheLineMap.clone(), prefetchLevel: prefetchLevel.clone(), writeLevel: writeLevel.clone() };
    Ok(oCreatedCacheLine)
}

fn isCLWrittenByOtherThread(mut iLevelInfo: (i32, i32), mut iLevelIdx: i32, mut iThreadIdx: i32) -> bool {
    let mut oWrittenByOtherThread: bool = false;
    let mut levelIdx: i32 = 0;
    let mut threadIdx: i32 = 0;
    let mut ret: bool = false;
    (levelIdx, threadIdx) = iLevelInfo.clone();
    ret = boolAnd(intEq(levelIdx.clone(), iLevelIdx.clone()), intNe(threadIdx.clone(), iThreadIdx.clone()));
    oWrittenByOtherThread = ret.clone();
    oWrittenByOtherThread
}

fn createCacheMapFromThreadAndSharedCLs(mut iThreadCacheLines: CacheLines, mut iSharedCacheLines: ((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>)), mut iCacheMap: CacheMap) -> Result<CacheMap> {
    let mut oCacheMap: CacheMap;
    let mut cacheLineSize: i32 = 0;
    let mut cacheLinesFloat: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheLinesInt: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheLinesBool: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut threadCacheLinesFloat: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut threadCacheLinesInt: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut threadCacheLinesBool: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut fullyFilledSharedCacheLines: CacheLines;
    let mut partlyFilledCacheLines: PartlyFilledCacheLines;
    let mut cacheVariables: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let CacheMap::CACHEMAP { cacheLineSize: __pa0, cacheVariables: __pa1, cacheLinesFloat: __pa2, cacheLinesInt: __pa3, cacheLinesBool: __pa4 } = (iCacheMap.clone()) else { bail!("pattern mismatch") };
    cacheLineSize = __pa0.clone();
    cacheVariables = __pa1.clone();
    cacheLinesFloat = __pa2.clone();
    cacheLinesInt = __pa3.clone();
    cacheLinesBool = __pa4.clone();
    (partlyFilledCacheLines, fullyFilledSharedCacheLines) = iSharedCacheLines.clone();
    cacheLinesFloat = listAppend(cacheLinesFloat.clone(), listAppend(Util::tuple31(iThreadCacheLines.clone()), Util::tuple31(fullyFilledSharedCacheLines.clone())));
    cacheLinesInt = listAppend(cacheLinesInt.clone(), listAppend(Util::tuple32(iThreadCacheLines.clone()), Util::tuple32(fullyFilledSharedCacheLines.clone())));
    cacheLinesBool = listAppend(cacheLinesBool.clone(), listAppend(Util::tuple33(iThreadCacheLines.clone()), Util::tuple33(fullyFilledSharedCacheLines.clone())));
    cacheLinesFloat = listAppend(cacheLinesFloat.clone(), List::map(Util::tuple31(partlyFilledCacheLines.clone()), (std::sync::Arc::new(getCacheLineMapOfPartlyFilledCacheLine) as std::sync::Arc<dyn ::std::ops::Fn(PartlyFilledCacheLine) -> Result<CacheLineMap> + 'static>)));
    cacheLinesInt = listAppend(cacheLinesInt.clone(), List::map(Util::tuple32(partlyFilledCacheLines.clone()), (std::sync::Arc::new(getCacheLineMapOfPartlyFilledCacheLine) as std::sync::Arc<dyn ::std::ops::Fn(PartlyFilledCacheLine) -> Result<CacheLineMap> + 'static>)));
    cacheLinesBool = listAppend(cacheLinesBool.clone(), List::map(Util::tuple33(partlyFilledCacheLines.clone()), (std::sync::Arc::new(getCacheLineMapOfPartlyFilledCacheLine) as std::sync::Arc<dyn ::std::ops::Fn(PartlyFilledCacheLine) -> Result<CacheLineMap> + 'static>)));
    oCacheMap = CacheMap::CACHEMAP { cacheLineSize: cacheLineSize.clone(), cacheVariables: cacheVariables.clone(), cacheLinesFloat: cacheLinesFloat.clone(), cacheLinesInt: cacheLinesInt.clone(), cacheLinesBool: cacheLinesBool.clone() };
    Ok(oCacheMap)
}

fn createCacheMapDefault(mut iAllSCVars: metamodelica::Array<Option<SimCodeVar::SimVar>>, mut iCacheLineSize: i32, mut iSimCodeVars: SimCodeVar::SimVars, mut iScVarTaskMapping: metamodelica::Array<i32>, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>, mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>) -> Result<(CacheMap, metamodelica::Array<(i32, i32)>, i32)> {
    let mut oCacheMap: CacheMap;
    let mut oScVarCLMapping: metamodelica::Array<(i32, i32)>;
    let mut oNumCL: i32 = 0;
    let mut iAllFloatVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut cacheLineFloatMaps: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut tmpScVarCLMapping: metamodelica::Array<(i32, i32)>;
    if stringEqual((Config::simCodeTarget()?).clone(), (literal!("Cpp")).clone()) {
        (oCacheMap, oScVarCLMapping, oNumCL) = createCacheMapDefaultCppRuntime(iAllSCVars.clone(), iCacheLineSize.clone(), iSimCodeVars.clone(), iScVarTaskMapping.clone(), iSchedulerInfo.clone(), iSimCodeVarTypes.clone())?;
    } else {
        oCacheMap = CacheMap::UNIFORM_CACHEMAP { cacheLineSize: iCacheLineSize.clone(), cacheVariables: metamodelica::nil(), cacheLines: metamodelica::nil() };
        oNumCL = 0;
        oScVarCLMapping = arrayCreate(0, (-1, -1));
    }
    Ok((oCacheMap, oScVarCLMapping, oNumCL))
}

fn createCacheMapDefaultCppRuntime(mut iAllSCVars: metamodelica::Array<Option<SimCodeVar::SimVar>>, mut iCacheLineSize: i32, mut iSimCodeVars: SimCodeVar::SimVars, mut iScVarTaskMapping: metamodelica::Array<i32>, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>, mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>) -> Result<(CacheMap, metamodelica::Array<(i32, i32)>, i32)> {
    let mut oCacheMap: CacheMap;
    let mut oScVarCLMapping: metamodelica::Array<(i32, i32)>;
    let mut oNumCL: i32 = 0;
    let mut stateVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut derivativeVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut algVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut discreteAlgVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut paramVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut aliasVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut intAlgVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut intParamVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut intAliasVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut boolAlgVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut boolParamVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut boolAliasVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut inputVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut outputVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut cacheMap: CacheMap;
    let mut lastCacheLine: CacheLineMap;
    let mut lastCacheLineNew: CacheLineMap;
    let mut scVarCLMapping: metamodelica::Array<(i32, i32)>;
    let mut tmpNumCL: i32 = 0;
    let mut currentScVarIdx: i32 = 0;
    let mut paramVarsStart: i32 = 0;
    let mut aliasVarsStart: i32 = 0;
    let mut stateDerVarsStart: i32 = 0;
    let mut algVarsStart: i32 = 0;
    let mut discreteAlgVarsStart: i32 = 0;
    let mut intAlgVarsStart: i32 = 0;
    let mut intParamVarsStart: i32 = 0;
    let mut filledCacheLines: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut allVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    (oCacheMap, oScVarCLMapping, oNumCL) = (match (iAllSCVars.clone(), iCacheLineSize.clone(), iSimCodeVars.clone(), iScVarTaskMapping.clone(), iSchedulerInfo.clone(), iSimCodeVarTypes.clone()) {
        (_, _, SimCodeVar::SimVars { outputVars: mut outputVars, inputVars: mut inputVars, boolAliasVars: mut boolAliasVars, boolParamVars: mut boolParamVars, boolAlgVars: mut boolAlgVars, intAliasVars: mut intAliasVars, intParamVars: mut intParamVars, intAlgVars: mut intAlgVars, aliasVars: mut aliasVars, paramVars: mut paramVars, discreteAlgVars: mut discreteAlgVars, algVars: mut algVars, derivativeVars: mut derivativeVars, stateVars: mut stateVars, .. }, _, _, _) => {
            currentScVarIdx = 1;
            stateDerVarsStart = (stateVars.clone().len() as i32) + 1;
            scVarCLMapping = arrayCreate((iAllSCVars.clone().borrow().len() as i32), (-1, -1));
            filledCacheLines = metamodelica::nil();
            lastCacheLine = CacheLineMap { idx: 1, numBytesFree: iCacheLineSize.clone(), entries: metamodelica::nil() };
            (filledCacheLines, lastCacheLine, currentScVarIdx) = createCacheMapDefaultCppRuntime0(derivativeVars.clone(), currentScVarIdx.clone(), stateDerVarsStart.clone(), scVarCLMapping.clone(), filledCacheLines.clone(), iScVarTaskMapping.clone(), iSchedulerInfo.clone(), lastCacheLine.clone(), iCacheLineSize.clone(), iSimCodeVarTypes.clone())?;
            filledCacheLines = cons(lastCacheLine.clone(), filledCacheLines.clone());
            lastCacheLine = CacheLineMap { idx: (filledCacheLines.clone().len() as i32) + 1, numBytesFree: iCacheLineSize.clone(), entries: metamodelica::nil() };
            allVars = derivativeVars.clone().reverse();
            algVarsStart = stateDerVarsStart.clone() + (derivativeVars.clone().len() as i32);
            discreteAlgVarsStart = algVarsStart.clone() + (algVars.clone().len() as i32);
            intAlgVarsStart = discreteAlgVarsStart.clone() + (discreteAlgVars.clone().len() as i32);
            aliasVarsStart = intAlgVarsStart.clone() + (boolAlgVars.clone().len() as i32) + (inputVars.clone().len() as i32) + (outputVars.clone().len() as i32);
            paramVarsStart = aliasVarsStart.clone() + (aliasVars.clone().len() as i32) + (intAliasVars.clone().len() as i32) + (boolAliasVars.clone().len() as i32);
            intParamVarsStart = paramVarsStart.clone() + (paramVars.clone().len() as i32);
            (filledCacheLines, lastCacheLine, currentScVarIdx) = createCacheMapDefaultCppRuntime0(algVars.clone(), currentScVarIdx.clone(), algVarsStart.clone(), scVarCLMapping.clone(), filledCacheLines.clone(), iScVarTaskMapping.clone(), iSchedulerInfo.clone(), lastCacheLine.clone(), iCacheLineSize.clone(), iSimCodeVarTypes.clone())?;
            allVars = List::append_reverse(algVars.clone(), allVars.clone());
            (filledCacheLines, lastCacheLine, currentScVarIdx) = createCacheMapDefaultCppRuntime0(discreteAlgVars.clone(), currentScVarIdx.clone(), discreteAlgVarsStart.clone(), scVarCLMapping.clone(), filledCacheLines.clone(), iScVarTaskMapping.clone(), iSchedulerInfo.clone(), lastCacheLine.clone(), iCacheLineSize.clone(), iSimCodeVarTypes.clone())?;
            allVars = List::append_reverse(discreteAlgVars.clone(), allVars.clone());
            (filledCacheLines, lastCacheLine, currentScVarIdx) = createCacheMapDefaultCppRuntime0(paramVars.clone(), currentScVarIdx.clone(), paramVarsStart.clone(), scVarCLMapping.clone(), filledCacheLines.clone(), iScVarTaskMapping.clone(), iSchedulerInfo.clone(), lastCacheLine.clone(), iCacheLineSize.clone(), iSimCodeVarTypes.clone())?;
            allVars = List::append_reverse(paramVars.clone(), allVars.clone());
            (filledCacheLines, lastCacheLine, currentScVarIdx) = createCacheMapDefaultCppRuntime0(aliasVars.clone(), currentScVarIdx.clone(), aliasVarsStart.clone(), scVarCLMapping.clone(), filledCacheLines.clone(), iScVarTaskMapping.clone(), iSchedulerInfo.clone(), lastCacheLine.clone(), iCacheLineSize.clone(), iSimCodeVarTypes.clone())?;
            allVars = List::append_reverse(aliasVars.clone(), allVars.clone());
            (filledCacheLines, lastCacheLine, currentScVarIdx) = createCacheMapDefaultCppRuntime0(intAlgVars.clone(), currentScVarIdx.clone(), intAlgVarsStart.clone(), scVarCLMapping.clone(), filledCacheLines.clone(), iScVarTaskMapping.clone(), iSchedulerInfo.clone(), lastCacheLine.clone(), iCacheLineSize.clone(), iSimCodeVarTypes.clone())?;
            allVars = List::append_reverse(intAlgVars.clone(), allVars.clone());
            (filledCacheLines, lastCacheLine, currentScVarIdx) = createCacheMapDefaultCppRuntime0(intParamVars.clone(), currentScVarIdx.clone(), intAlgVarsStart.clone(), scVarCLMapping.clone(), filledCacheLines.clone(), iScVarTaskMapping.clone(), iSchedulerInfo.clone(), lastCacheLine.clone(), iCacheLineSize.clone(), iSimCodeVarTypes.clone())?;
            allVars = List::append_reverse(intParamVars.clone(), allVars.clone());
            cacheMap = CacheMap::UNIFORM_CACHEMAP { cacheLineSize: iCacheLineSize.clone(), cacheVariables: allVars.clone(), cacheLines: cons(lastCacheLine.clone(), filledCacheLines.clone()) };
            (cacheMap.clone(), scVarCLMapping.clone(), (filledCacheLines.clone().len() as i32) + 1)
        },
        _ => bail!("match: no arm matched"),
    });
    Ok((oCacheMap, oScVarCLMapping, oNumCL))
}

fn createCacheMapDefaultCppRuntime0(mut iVariables: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut iScVarIdxStart: i32, mut iRealScVarIdxStart: i32, mut iScVarCLMapping: metamodelica::Array<(i32, i32)>, mut iFilledCacheLines: Arc<metamodelica::List<CacheLineMap>>, mut iScVarTaskMapping: metamodelica::Array<i32>, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>, mut iLastCacheLine: CacheLineMap, mut iCacheLineSize: i32, mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>) -> Result<(Arc<metamodelica::List<CacheLineMap>>, CacheLineMap, i32)> {
    let mut oFilledCacheLines: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut oLastCacheLine: CacheLineMap;
    let mut oScVarIdx: i32 = 0;
    let mut currentScVarIdx: i32 = 0;
    let mut varSize: i32 = 0;
    let mut varDataType: i32 = 0;
    let mut varTask: i32 = 0;
    let mut threadIdx: i32 = 0;
    let mut varCLIdx: i32 = 0;
    let mut var: SimCodeVar::SimVar;
    let mut entry: CacheLineEntry;
    let mut newCacheLineCreated: bool = false;
    let mut lastCacheLine: CacheLineMap;
    let mut lastCacheLineNew: CacheLineMap;
    let mut filledCacheLines: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cachelineEntries: Arc<metamodelica::List<CacheLineEntry>> = metamodelica::nil();
    let mut name: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut nameStr: ArcStr = arcstr::literal!("");
    currentScVarIdx = 0;
    lastCacheLine = iLastCacheLine.clone();
    filledCacheLines = iFilledCacheLines.clone();
    for mut var in &*iVariables.clone() {
        let mut var = var.clone();
        let SimCodeVar::SIMVAR { name: __pa0, .. } = (var.clone()) else { bail!("pattern mismatch") };
        name = __pa0.clone();
        nameStr = (ComponentReferenceBasics::printComponentRefStr(name.clone())?).clone();
        if boolAnd(intLt(currentScVarIdx.clone(), (iSimCodeVarTypes.clone().borrow().len() as i32)), intLt(currentScVarIdx.clone(), (iScVarCLMapping.clone().borrow().len() as i32))) {
            (varDataType, varSize, _) = iSimCodeVarTypes.clone().borrow()[(currentScVarIdx.clone() + iRealScVarIdxStart.clone()-1) as usize].clone();
            if intLe(currentScVarIdx.clone() + iRealScVarIdxStart.clone(), (iScVarTaskMapping.clone().borrow().len() as i32)) {
                varTask = iScVarTaskMapping.clone().borrow()[(currentScVarIdx.clone() + iRealScVarIdxStart.clone()-1) as usize].clone();
            } else {
                varTask = -1;
            }
            if boolAnd(intGe(varTask.clone(), 1), intGe((iSchedulerInfo.clone().borrow().len() as i32), varTask.clone())) {
                threadIdx = Util::tuple31(iSchedulerInfo.clone().borrow()[(varTask.clone()-1) as usize].clone());
            } else {
                threadIdx = -1;
            }
            entry = CacheLineEntry { start: -1, dataType: varDataType.clone(), size: varSize.clone(), scVarIdx: currentScVarIdx.clone() + iScVarIdxStart.clone(), threadOwner: threadIdx.clone() };
            (entry, lastCacheLineNew, newCacheLineCreated) = createCacheMapDefaultCppRuntime1(entry.clone(), iCacheLineSize.clone(), lastCacheLine.clone())?;
            let CacheLineMap { entries: __pa1, idx: __pa2, .. } = (lastCacheLineNew.clone()) else { bail!("pattern mismatch") };
            cachelineEntries = __pa1.clone();
            varCLIdx = __pa2.clone();
            let _ = {let _arr = iScVarCLMapping.clone(); _arr.borrow_mut()[(currentScVarIdx.clone() + iRealScVarIdxStart.clone()-1) as usize] = (varCLIdx.clone(), varDataType.clone()); _arr};
            if newCacheLineCreated.clone() {
                filledCacheLines = cons(lastCacheLine.clone(), filledCacheLines.clone());
            }
            lastCacheLine = lastCacheLineNew.clone();
        }
        currentScVarIdx = currentScVarIdx.clone() + 1;
    }
    oFilledCacheLines = filledCacheLines.clone();
    oLastCacheLine = lastCacheLine.clone();
    oScVarIdx = currentScVarIdx.clone() + iScVarIdxStart.clone();
    Ok((oFilledCacheLines, oLastCacheLine, oScVarIdx))
}

fn createCacheMapDefaultCppRuntime1(mut iCacheLineEntry: CacheLineEntry, mut iCacheLineSize: i32, mut iLastCacheLine: CacheLineMap) -> Result<(CacheLineEntry, CacheLineMap, bool)> {
    let mut oCacheLineEntry: CacheLineEntry;
    let mut oLastCacheLine: CacheLineMap;
    let mut oNewOneCreated: bool = false;
    let mut numberOfFreeBytesLastCacheLine: i32 = 0;
    let mut lastCacheLineEntries: Arc<metamodelica::List<CacheLineEntry>> = metamodelica::nil();
    let mut cacheLine: CacheLineMap;
    let mut cacheLineEntry: CacheLineEntry;
    let mut entrySize: i32 = 0;
    let mut entryStart: i32 = 0;
    let mut entryType: i32 = 0;
    let mut entryVarIdx: i32 = 0;
    let mut entryThreadOwner: i32 = 0;
    let mut lastCacheLineIdx: i32 = 0;
    let CacheLineEntry { start: __pa0, dataType: __pa1, size: __pa2, scVarIdx: __pa3, threadOwner: __pa4 } = (iCacheLineEntry.clone()) else { bail!("pattern mismatch") };
    entryStart = __pa0.clone();
    entryType = __pa1.clone();
    entrySize = __pa2.clone();
    entryVarIdx = __pa3.clone();
    entryThreadOwner = __pa4.clone();
    let CacheLineMap { idx: __pa5, numBytesFree: __pa6, entries: __pa7 } = (iLastCacheLine.clone()) else { bail!("pattern mismatch") };
    lastCacheLineIdx = __pa5.clone();
    numberOfFreeBytesLastCacheLine = __pa6.clone();
    lastCacheLineEntries = __pa7.clone();
    if intGt(entrySize.clone(), numberOfFreeBytesLastCacheLine.clone()) {
        cacheLineEntry = CacheLineEntry { start: 0, dataType: entryType.clone(), size: entrySize.clone(), scVarIdx: entryVarIdx.clone(), threadOwner: entryThreadOwner.clone() };
        cacheLine = CacheLineMap { idx: lastCacheLineIdx.clone() + 1, numBytesFree: iCacheLineSize.clone() - entrySize.clone(), entries: list![cacheLineEntry.clone()] };
        oNewOneCreated = true;
    } else {
        cacheLineEntry = CacheLineEntry { start: iCacheLineSize.clone() - numberOfFreeBytesLastCacheLine.clone(), dataType: entryType.clone(), size: entrySize.clone(), scVarIdx: entryVarIdx.clone(), threadOwner: entryThreadOwner.clone() };
        cacheLine = CacheLineMap { idx: lastCacheLineIdx.clone(), numBytesFree: numberOfFreeBytesLastCacheLine.clone() - entrySize.clone(), entries: cons(cacheLineEntry.clone(), lastCacheLineEntries.clone()) };
        oNewOneCreated = false;
    }
    oCacheLineEntry = cacheLineEntry.clone();
    oLastCacheLine = cacheLine.clone();
    Ok((oCacheLineEntry, oLastCacheLine, oNewOneCreated))
}

fn appendNodeVarsToCacheMap(mut iNodeIdx: i32, mut iOwnerThread: i32, mut iNodeSimCodeVarMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iInfo: (CacheMap, CacheMapMeta, i32, Arc<metamodelica::List<(i32, i32)>>)) -> Result<(CacheMap, CacheMapMeta, i32, Arc<metamodelica::List<(i32, i32)>>)> {
    let mut oInfo: (CacheMap, CacheMapMeta, i32, Arc<metamodelica::List<(i32, i32)>>);
    let mut simCodeVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut writtenCL: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut iCacheMap: CacheMap;
    let mut iCacheMapMeta: CacheMapMeta;
    let mut iNumNewCL: i32 = 0;
    let mut varsString: ArcStr = arcstr::literal!("");
    let mut clCandidates: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    simCodeVars = iNodeSimCodeVarMapping.clone().borrow()[(iNodeIdx.clone()-1) as usize].clone();
    (iCacheMap, iCacheMapMeta, iNumNewCL, clCandidates) = iInfo.clone();
    varsString = stringDelimitList(List::map(simCodeVars.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone());
    (iCacheMap, iCacheMapMeta, iNumNewCL, clCandidates, writtenCL, _) = List::fold(simCodeVars.clone(), Arc::new({ let __pe_b1 = iOwnerThread.clone(); move |__pe_a0, __pe_a2| appendSCVarToCacheMap(__pe_a0, __pe_b1.clone(), __pe_a2) }), (iCacheMap.clone(), iCacheMapMeta.clone(), iNumNewCL.clone(), clCandidates.clone(), metamodelica::nil(), 1));
    clCandidates = List::removeOnTrue(writtenCL.clone(), (std::sync::Arc::new(appendNodeVarsToCacheMap0) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, (i32, i32)) -> Result<bool> + 'static>), clCandidates.clone());
    oInfo = (iCacheMap.clone(), iCacheMapMeta.clone(), iNumNewCL.clone(), clCandidates.clone());
    Ok(oInfo)
}

fn appendNodeVarsToCacheMap0(mut iWrittenCLs: Arc<metamodelica::List<i32>>, mut iDetailedCLInfo: (i32, i32)) -> Result<bool> {
    let mut oRemove: bool = false;
    let mut clIdx: i32 = 0;
    let mut freeBytes: i32 = 0;
    let mut res: bool = false;
    oRemove = 'mc: {
        let __mc_input = (iWrittenCLs.clone(), iDetailedCLInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (clIdx, freeBytes)) => {
                    let true = (intEq(freeBytes.clone(), 0)) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (clIdx, freeBytes)) => {
                    let mut res: bool = res.clone();
                    res = List::isMemberOnTrue(clIdx.clone(), iWrittenCLs.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>));
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("appendNodeVarsToCacheMap0 failed!\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oRemove)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn appendSCVarToCacheMap(mut iSCVarIdx: i32, mut iOwnerThread: i32, mut iInfo: (CacheMap, CacheMapMeta, i32, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, i32)) -> Result<(CacheMap, CacheMapMeta, i32, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, i32)> {
    let mut oInfo: (CacheMap, CacheMapMeta, i32, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, i32);
    let mut iAllSCVarsMapping: metamodelica::Array<Option<SimCodeVar::SimVar>>;
    let mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>;
    let mut iScVarCLMapping: metamodelica::Array<(i32, i32)>;
    let mut currentCLCandidateIdx: i32 = 0;
    let mut currentCLCandidateCLIdx: i32 = 0;
    let mut clIdx: i32 = 0;
    let mut currentCLCandidateFreeBytes: i32 = 0;
    let mut cacheLineSize: i32 = 0;
    let mut numNewCL: i32 = 0;
    let mut varDataType: i32 = 0;
    let mut numBytesRequired: i32 = 0;
    let mut entryStart: i32 = 0;
    let mut currentCLCandidate: (i32, i32);
    let mut cacheLineCandidates: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut cacheLinesFloat: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheLinesInt: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheLinesBool: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheVariables: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut cacheLine: CacheLineMap;
    let mut CLentries: Arc<metamodelica::List<CacheLineEntry>> = metamodelica::nil();
    let mut scVar: SimCodeVar::SimVar;
    let mut numCacheVars: i32 = 0;
    let mut freeSpace: i32 = 0;
    let mut numBytesFree: i32 = 0;
    let mut cacheMap: CacheMap;
    let mut cacheMapMeta: CacheMapMeta;
    let mut writtenCL: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut varText: ArcStr = arcstr::literal!("");
    let mut tmpInfo: (CacheMap, CacheMapMeta, i32, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, i32);
    oInfo = 'mc: {
        let __mc_input = (iSCVarIdx.clone(), iOwnerThread.clone(), iInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, (cacheMap @ CacheMap::CACHEMAP { cacheLinesBool, cacheLinesInt, cacheLinesFloat, cacheVariables, cacheLineSize }, cacheMapMeta @ CacheMapMeta { allSCVarsMapping: iAllSCVarsMapping, simCodeVarTypes: iSimCodeVarTypes, scVarCLMapping: iScVarCLMapping }, numNewCL, cacheLineCandidates, writtenCL, currentCLCandidateIdx)) => {
                    let mut cacheMap = (*cacheMap).clone();
                    let mut cacheLinesFloat = (*cacheLinesFloat).clone();
                    let mut cacheVariables = (*cacheVariables).clone();
                    let mut cacheMapMeta = (*cacheMapMeta).clone();
                    let mut iScVarCLMapping = (*iScVarCLMapping).clone();
                    let mut cacheLineCandidates = (*cacheLineCandidates).clone();
                    let mut writtenCL = (*writtenCL).clone();
                    let mut numBytesRequired: i32 = numBytesRequired.clone();
                    let mut clIdx: i32 = clIdx.clone();
                    let mut entryStart: i32 = entryStart.clone();
                    let mut numBytesFree: i32 = numBytesFree.clone();
                    let mut cacheLine: CacheLineMap;
                    let mut scVar: SimCodeVar::SimVar;
                    let mut CLentries: Arc<metamodelica::List<CacheLineEntry>> = CLentries.clone();
                    let mut numCacheVars: i32 = numCacheVars.clone();
                    let mut varDataType: i32 = varDataType.clone();
                    let mut currentCLCandidateFreeBytes: i32 = currentCLCandidateFreeBytes.clone();
                    let mut currentCLCandidateCLIdx: i32 = currentCLCandidateCLIdx.clone();
                    let mut currentCLCandidate: (i32, i32);
                    let true = (intGe((cacheLineCandidates.clone().len() as i32), currentCLCandidateIdx.clone())) else { bail!("pattern mismatch") };
                    currentCLCandidate = (cacheLineCandidates.clone()).get(currentCLCandidateIdx.clone())?;
                    (varDataType, numBytesRequired, _) = iSimCodeVarTypes.clone().borrow()[(iSCVarIdx.clone()-1) as usize].clone();
                    let true = (doesSCVarFitIntoCL(currentCLCandidate.clone(), numBytesRequired.clone())) else { bail!("pattern mismatch") };
                    (currentCLCandidateCLIdx, currentCLCandidateFreeBytes) = currentCLCandidate.clone();
                    cacheLine = (cacheLinesFloat.clone()).get((cacheLinesFloat.clone().len() as i32) - currentCLCandidateCLIdx.clone() + 1)?;
                    let CacheLineMap { entries: __pa0, numBytesFree: __pa1, idx: __pa2 } = (cacheLine.clone()) else { bail!("pattern mismatch") };
                    CLentries = __pa0.clone();
                    numBytesFree = __pa1.clone();
                    clIdx = __pa2.clone();
                    entryStart = cacheLineSize.clone() - currentCLCandidateFreeBytes.clone();
                    numCacheVars = (cacheVariables.clone().len() as i32) + 1;
                    CLentries = cons(CacheLineEntry { start: entryStart.clone(), dataType: varDataType.clone(), size: numBytesRequired.clone(), scVarIdx: numCacheVars.clone(), threadOwner: iOwnerThread.clone() }, CLentries.clone());
                    cacheLine = CacheLineMap { idx: clIdx.clone(), numBytesFree: numBytesFree.clone() + numBytesRequired.clone(), entries: CLentries.clone() };
                    cacheLinesFloat = List::set(cacheLinesFloat.clone(), (cacheLinesFloat.clone().len() as i32) - currentCLCandidateCLIdx.clone() + 1, cacheLine.clone())?;
                    iScVarCLMapping = {let _arr = iScVarCLMapping.clone(); _arr.borrow_mut()[(iSCVarIdx.clone()-1) as usize] = (clIdx.clone(), varDataType.clone()); _arr};
                    let Some(__pa3) = (iAllSCVarsMapping.clone().borrow()[(iSCVarIdx.clone()-1) as usize].clone()) else { bail!("pattern mismatch") };
                    scVar = __pa3.clone();
                    cacheVariables = cons(scVar.clone(), cacheVariables.clone());
                    writtenCL = cons(clIdx.clone(), writtenCL.clone());
                    currentCLCandidate = (currentCLCandidateCLIdx.clone(), currentCLCandidateFreeBytes.clone() - numBytesRequired.clone());
                    cacheLineCandidates = List::set(cacheLineCandidates.clone(), currentCLCandidateIdx.clone(), currentCLCandidate.clone())?;
                    cacheMap = CacheMap::CACHEMAP { cacheLineSize: cacheLineSize.clone(), cacheVariables: cacheVariables.clone(), cacheLinesFloat: cacheLinesFloat.clone(), cacheLinesInt: cacheLinesInt.clone(), cacheLinesBool: cacheLinesBool.clone() };
                    cacheMapMeta = CacheMapMeta { allSCVarsMapping: iAllSCVarsMapping.clone(), simCodeVarTypes: iSimCodeVarTypes.clone(), scVarCLMapping: iScVarCLMapping.clone() };
                    Ok((cacheMap.clone(), cacheMapMeta.clone(), numNewCL.clone(), cacheLineCandidates.clone(), writtenCL.clone(), currentCLCandidateIdx.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, (cacheMap @ CacheMap::CACHEMAP { cacheLinesBool, cacheLinesInt, cacheLinesFloat, cacheVariables, cacheLineSize }, cacheMapMeta @ CacheMapMeta { allSCVarsMapping: iAllSCVarsMapping, simCodeVarTypes: iSimCodeVarTypes, scVarCLMapping: iScVarCLMapping }, numNewCL, cacheLineCandidates, writtenCL, currentCLCandidateIdx)) => {
                    let mut varDataType: i32 = varDataType.clone();
                    let mut numBytesRequired: i32 = numBytesRequired.clone();
                    let mut tmpInfo: (CacheMap, CacheMapMeta, i32, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, i32);
                    let true = (intGe((cacheLineCandidates.clone().len() as i32), currentCLCandidateIdx.clone())) else { bail!("pattern mismatch") };
                    (varDataType, numBytesRequired, _) = iSimCodeVarTypes.clone().borrow()[(iSCVarIdx.clone()-1) as usize].clone();
                    tmpInfo = appendSCVarToCacheMap(iSCVarIdx.clone(), iOwnerThread.clone(), (cacheMap.clone(), cacheMapMeta.clone(), numNewCL.clone(), cacheLineCandidates.clone(), writtenCL.clone(), currentCLCandidateIdx.clone() + 1))?;
                    Ok(tmpInfo.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, (cacheMap @ CacheMap::CACHEMAP { cacheLinesBool, cacheLinesInt, cacheLinesFloat, cacheVariables, cacheLineSize }, CacheMapMeta { allSCVarsMapping: iAllSCVarsMapping, simCodeVarTypes: iSimCodeVarTypes, scVarCLMapping: iScVarCLMapping }, numNewCL, cacheLineCandidates, writtenCL, currentCLCandidateIdx)) => {
                    let mut cacheMap = (*cacheMap).clone();
                    let mut cacheLinesFloat = (*cacheLinesFloat).clone();
                    let mut cacheVariables = (*cacheVariables).clone();
                    let mut iScVarCLMapping = (*iScVarCLMapping).clone();
                    let mut cacheLineCandidates = (*cacheLineCandidates).clone();
                    let mut writtenCL = (*writtenCL).clone();
                    let mut entryStart: i32 = entryStart.clone();
                    let mut numCacheVars: i32 = numCacheVars.clone();
                    let mut CLentries: Arc<metamodelica::List<CacheLineEntry>> = CLentries.clone();
                    let mut cacheMapMeta: CacheMapMeta;
                    let mut cacheLine: CacheLineMap;
                    let mut scVar: SimCodeVar::SimVar;
                    let mut clIdx: i32 = clIdx.clone();
                    let mut varDataType: i32 = varDataType.clone();
                    let mut numBytesRequired: i32 = numBytesRequired.clone();
                    let mut freeSpace: i32 = freeSpace.clone();
                    (varDataType, numBytesRequired, _) = iSimCodeVarTypes.clone().borrow()[(iSCVarIdx.clone()-1) as usize].clone();
                    entryStart = 0;
                    numCacheVars = (cacheVariables.clone().len() as i32) + 1;
                    CLentries = list![CacheLineEntry { start: entryStart.clone(), dataType: varDataType.clone(), size: numBytesRequired.clone(), scVarIdx: numCacheVars.clone(), threadOwner: iOwnerThread.clone() }];
                    clIdx = (cacheLinesFloat.clone().len() as i32) + 1;
                    cacheLine = CacheLineMap { idx: clIdx.clone(), numBytesFree: numBytesRequired.clone(), entries: CLentries.clone() };
                    cacheLinesFloat = cons(cacheLine.clone(), cacheLinesFloat.clone());
                    iScVarCLMapping = {let _arr = iScVarCLMapping.clone(); _arr.borrow_mut()[(iSCVarIdx.clone()-1) as usize] = (clIdx.clone(), varDataType.clone()); _arr};
                    let Some(__pa0) = (iAllSCVarsMapping.clone().borrow()[(iSCVarIdx.clone()-1) as usize].clone()) else { bail!("pattern mismatch") };
                    scVar = __pa0.clone();
                    cacheVariables = cons(scVar.clone(), cacheVariables.clone());
                    writtenCL = cons(clIdx.clone(), writtenCL.clone());
                    freeSpace = cacheLineSize.clone() - numBytesRequired.clone();
                    cacheLineCandidates = List::appendElt((clIdx.clone(), freeSpace.clone()), cacheLineCandidates.clone());
                    cacheMap = CacheMap::CACHEMAP { cacheLineSize: cacheLineSize.clone(), cacheVariables: cacheVariables.clone(), cacheLinesFloat: cacheLinesFloat.clone(), cacheLinesInt: cacheLinesInt.clone(), cacheLinesBool: cacheLinesBool.clone() };
                    cacheMapMeta = CacheMapMeta { allSCVarsMapping: iAllSCVarsMapping.clone(), simCodeVarTypes: iSimCodeVarTypes.clone(), scVarCLMapping: iScVarCLMapping.clone() };
                    Ok((cacheMap.clone(), cacheMapMeta.clone(), numNewCL.clone() + 1, cacheLineCandidates.clone(), writtenCL.clone(), currentCLCandidateIdx.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("appendSCVarToCacheMap failed! Variable skipped.\n")).clone());
                    Ok(iInfo.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oInfo)
}

fn doesSCVarFitIntoCL(mut iCacheLineCandidate: (i32, i32), mut iNumBytes: i32) -> bool {
    let mut oResult: bool = false;
    let mut freeSpace: i32 = 0;
    (_, freeSpace) = iCacheLineCandidate.clone();
    oResult = intGe(freeSpace.clone(), iNumBytes.clone());
    oResult
}

fn createDetailedCacheMapInformation(mut iCacheLinesIdc: Arc<metamodelica::List<i32>>, mut iCacheLines: Arc<metamodelica::List<CacheLineMap>>, mut iCacheLineSize: i32) -> Arc<metamodelica::List<(i32, i32)>> {
    let mut oCacheLines: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut iCacheLinesArray: metamodelica::Array<CacheLineMap>;
    iCacheLinesArray = metamodelica::arrayFromVec(iCacheLines.clone().into_iter().cloned().collect());
    oCacheLines = List::fold2(iCacheLinesIdc.clone(), (std::sync::Arc::new(createDetailedCacheMapInformation0) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<CacheLineMap>, i32, Arc<metamodelica::List<(i32, i32)>>) -> Result<Arc<metamodelica::List<(i32, i32)>>> + 'static>), iCacheLinesArray.clone(), iCacheLineSize.clone(), metamodelica::nil());
    oCacheLines
}

fn createDetailedCacheMapInformation0(mut iCacheLineIdx: i32, mut iCacheLinesArray: metamodelica::Array<CacheLineMap>, mut iCacheLineSize: i32, mut iCacheLines: Arc<metamodelica::List<(i32, i32)>>) -> Result<Arc<metamodelica::List<(i32, i32)>>> {
    let mut oCacheLines: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut cacheLineEntry: CacheLineMap;
    let mut numBytesFree: i32 = 0;
    let mut cacheLines: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    oCacheLines = 'mc: {
        let __mc_input = (iCacheLineIdx.clone(), iCacheLinesArray.clone(), iCacheLineSize.clone(), iCacheLines.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _) => {
                    let mut numBytesFree: i32 = numBytesFree.clone();
                    let mut cacheLines: Arc<metamodelica::List<(i32, i32)>> = cacheLines.clone();
                    let mut cacheLineEntry: CacheLineMap;
                    cacheLineEntry = iCacheLinesArray.clone().borrow()[((iCacheLinesArray.clone().borrow().len() as i32) - iCacheLineIdx.clone() + 1-1) as usize].clone();
                    numBytesFree = iCacheLineSize.clone() - getNumOfUsedBytesByCacheLine(cacheLineEntry.clone())?;
                    let true = (intGt(numBytesFree.clone(), 0)) else { bail!("pattern mismatch") };
                    cacheLines = cons((iCacheLineIdx.clone(), numBytesFree.clone()), iCacheLines.clone());
                    Ok(cacheLines.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iCacheLines.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oCacheLines)
}

fn getNumOfUsedBytesByCacheLine(mut iCacheLineMap: CacheLineMap) -> Result<i32> {
    let mut oNumBytes: i32 = 0;
    let mut entries: Arc<metamodelica::List<CacheLineEntry>> = metamodelica::nil();
    let mut firstEntryStart: i32 = 0;
    let mut firstEntrySize: i32 = 0;
    let CacheLineMap { entries: __pa0, .. } = (iCacheLineMap.clone()) else { bail!("pattern mismatch") };
    entries = __pa0.clone();
    entries = List::sort(entries.clone(), (std::sync::Arc::new(sortCacheLineEntriesByPos) as std::sync::Arc<dyn ::std::ops::Fn(CacheLineEntry, CacheLineEntry) -> Result<bool> + 'static>))?;
    let CacheLineEntry { size: __pa1, start: __pa2, .. } = (List::last(entries.clone())?) else { bail!("pattern mismatch") };
    firstEntrySize = __pa1.clone();
    firstEntryStart = __pa2.clone();
    oNumBytes = firstEntryStart.clone() + firstEntrySize.clone();
    Ok(oNumBytes)
}

fn sortCacheLineEntriesByPos(mut iCacheLineEntry1: CacheLineEntry, mut iCacheLineEntry2: CacheLineEntry) -> Result<bool> {
    let mut oIsGreater: bool = false;
    let mut start1: i32 = 0;
    let mut start2: i32 = 0;
    let CacheLineEntry { start: __pa0, .. } = (iCacheLineEntry1.clone()) else { bail!("pattern mismatch") };
    start1 = __pa0.clone();
    let CacheLineEntry { start: __pa1, .. } = (iCacheLineEntry2.clone()) else { bail!("pattern mismatch") };
    start2 = __pa1.clone();
    oIsGreater = intGt(start1.clone(), start2.clone());
    Ok(oIsGreater)
}

fn reverseCacheLineMapEntries(mut iCacheLineMap: CacheLineMap) -> Result<CacheLineMap> {
    let mut oCacheLineMap: CacheLineMap;
    let mut idx: i32 = 0;
    let mut numBytesFree: i32 = 0;
    let mut entries: Arc<metamodelica::List<CacheLineEntry>> = metamodelica::nil();
    let CacheLineMap { entries: __pa0, numBytesFree: __pa1, idx: __pa2 } = (iCacheLineMap.clone()) else { bail!("pattern mismatch") };
    entries = __pa0.clone();
    numBytesFree = __pa1.clone();
    idx = __pa2.clone();
    entries = entries.clone().reverse();
    oCacheLineMap = CacheLineMap { idx: idx.clone(), numBytesFree: numBytesFree.clone(), entries: entries.clone() };
    Ok(oCacheLineMap)
}

fn compareCacheLineMapByIdx(mut iCacheLineMap: CacheLineMap, mut iCacheLineMap2: CacheLineMap) -> Result<bool> {
    let mut oIsGreater: bool = false;
    let mut idx1: i32 = 0;
    let mut idx2: i32 = 0;
    let CacheLineMap { idx: __pa0, .. } = (iCacheLineMap.clone()) else { bail!("pattern mismatch") };
    idx1 = __pa0.clone();
    let CacheLineMap { idx: __pa1, .. } = (iCacheLineMap2.clone()) else { bail!("pattern mismatch") };
    idx2 = __pa1.clone();
    oIsGreater = intGt(idx1.clone(), idx2.clone());
    Ok(oIsGreater)
}

fn convertCacheToVarArrayMapping(mut iCacheMap: CacheMap, mut iCacheLineSize: i32, mut iStateVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut iDerivativeVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut iAliasVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut iIntAliasVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut iBoolAliasVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut iStringAliasVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut iVarSizes: (i32, i32, i32), mut iNotOptimizedVars: (Arc<metamodelica::List<SimCodeVar::SimVar>>, Arc<metamodelica::List<SimCodeVar::SimVar>>, Arc<metamodelica::List<SimCodeVar::SimVar>>, Arc<metamodelica::List<SimCodeVar::SimVar>>)) -> Result<((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (HashTableCrIListArray::FuncHashCref, HashTableCrIListArray::FuncCrefEqual, HashTableCrIListArray::FuncCrefStr, HashTableCrIListArray::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr)), Option<HpcOmSimCode::MemoryMap>)> {
    let mut oVarToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (HashTableCrIListArray::FuncHashCref, HashTableCrIListArray::FuncCrefEqual, HashTableCrIListArray::FuncCrefStr, HashTableCrIListArray::FuncExpStr));
    let mut oVarToIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr));
    let mut oMemoryMap: Option<HpcOmSimCode::MemoryMap> = None;
    let mut cacheLineSize: i32 = 0;
    let mut highestIdx: i32 = 0;
    let mut maxNumElemsFloat: i32 = 0;
    let mut maxNumElemsInt: i32 = 0;
    let mut maxNumElemsBool: i32 = 0;
    let mut stateAndStateDerSize: i32 = 0;
    let mut cacheVariables: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut unusedRealVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut cacheVariablesArray: metamodelica::Array<SimCodeVar::SimVar>;
    let mut cacheLinesFloat: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheLinesInt: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheLinesBool: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut allCacheLines: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut varArrayIndexMappingHashTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (HashTableCrIListArray::FuncHashCref, HashTableCrIListArray::FuncCrefEqual, HashTableCrIListArray::FuncCrefStr, HashTableCrIListArray::FuncExpStr));
    let mut varIndexMappingHashTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr));
    let mut positionMappingArray: metamodelica::Array<(i32, i32)>;
    let mut varSizeFloat: i32 = 0;
    let mut varSizeInt: i32 = 0;
    let mut varSizeBool: i32 = 0;
    let mut varSizeString: i32 = 0;
    let mut positionMappingList: Arc<metamodelica::List<(i32, i32, i32)>> = metamodelica::nil();
    let mut varIdxOffsets: metamodelica::Array<i32>;
    let mut notOptimizedVarsFloat: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut notOptimizedVarsInt: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut notOptimizedVarsBool: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut notOptimizedVarsString: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut currentVarIndices: metamodelica::Array<i32>;
    (oVarToArrayIndexMapping, oVarToIndexMapping, oMemoryMap) = (::match_deref::match_deref! { match &((iCacheMap.clone(), iCacheLineSize.clone(), iStateVars.clone(), iDerivativeVars.clone(), iAliasVars.clone(), iIntAliasVars.clone(), iBoolAliasVars.clone(), iStringAliasVars.clone(), iVarSizes.clone(), iNotOptimizedVars.clone())) {
        (CacheMap::CACHEMAP { cacheLinesBool, cacheLinesInt, cacheLinesFloat, cacheVariables, cacheLineSize }, _, _, _, _, _, _, _, (varSizeFloat, varSizeInt, varSizeBool), (notOptimizedVarsFloat, notOptimizedVarsInt, notOptimizedVarsBool, notOptimizedVarsString)) => {
            let mut varSizeFloat = (*varSizeFloat).clone();
            let mut varSizeInt = (*varSizeInt).clone();
            let mut varSizeBool = (*varSizeBool).clone();
            maxNumElemsFloat = intDiv(iCacheLineSize.clone(), varSizeFloat.clone());
            maxNumElemsInt = intDiv(iCacheLineSize.clone(), varSizeInt.clone());
            maxNumElemsBool = intDiv(iCacheLineSize.clone(), varSizeBool.clone());
            cacheVariablesArray = metamodelica::arrayFromVec(cacheVariables.clone().into_iter().cloned().collect());
            varArrayIndexMappingHashTable = HashTableCrIListArray::emptyHashTable();
            varIndexMappingHashTable = HashTableCrILst::emptyHashTable();
            currentVarIndices = arrayCreate(4, 1);
            (currentVarIndices, varArrayIndexMappingHashTable, varIndexMappingHashTable) = SimCodeUtil::addVarToArrayIndexMappings(iStateVars.clone(), VARDATATYPE_FLOAT.clone(), currentVarIndices.clone(), varArrayIndexMappingHashTable.clone(), varIndexMappingHashTable.clone())?;
            (currentVarIndices, varArrayIndexMappingHashTable, varIndexMappingHashTable) = SimCodeUtil::addVarToArrayIndexMappings(iDerivativeVars.clone(), VARDATATYPE_FLOAT.clone(), currentVarIndices.clone(), varArrayIndexMappingHashTable.clone(), varIndexMappingHashTable.clone())?;
            stateAndStateDerSize = intAdd((iStateVars.clone().len() as i32), (iDerivativeVars.clone().len() as i32));
            if intEq(intMod(stateAndStateDerSize.clone(), maxNumElemsFloat.clone()), 0) {
                {let _arr = currentVarIndices.clone(); _arr.borrow_mut()[(1-1) as usize] = stateAndStateDerSize.clone() + 1; _arr};
                {let _arr = currentVarIndices.clone(); _arr.borrow_mut()[(2-1) as usize] = 1; _arr};
                {let _arr = currentVarIndices.clone(); _arr.borrow_mut()[(3-1) as usize] = 1; _arr};
                {let _arr = currentVarIndices.clone(); _arr.borrow_mut()[(4-1) as usize] = 1; _arr};
            } else {
                {let _arr = currentVarIndices.clone(); _arr.borrow_mut()[(1-1) as usize] = stateAndStateDerSize.clone() + (maxNumElemsFloat.clone() - intMod(stateAndStateDerSize.clone(), maxNumElemsFloat.clone())) + 1; _arr};
                {let _arr = currentVarIndices.clone(); _arr.borrow_mut()[(2-1) as usize] = 1; _arr};
                {let _arr = currentVarIndices.clone(); _arr.borrow_mut()[(3-1) as usize] = 1; _arr};
                {let _arr = currentVarIndices.clone(); _arr.borrow_mut()[(4-1) as usize] = 1; _arr};
            }
            varSizeFloat = currentVarIndices.clone().borrow()[(1-1) as usize].clone();
            varIdxOffsets = arrayCreate(3, 1);
            varIdxOffsets = {let _arr = varIdxOffsets.clone(); let _val = currentVarIndices.clone().borrow()[(1-1) as usize].clone() + 1; _arr.borrow_mut()[(1-1) as usize] = _val; _arr};
            allCacheLines = List::sort(getAllCacheLinesOfCacheMap(iCacheMap.clone())?, (std::sync::Arc::new(compareCacheLineMapByIdx) as std::sync::Arc<dyn ::std::ops::Fn(CacheLineMap, CacheLineMap) -> Result<bool> + 'static>))?;
            (varArrayIndexMappingHashTable, varIndexMappingHashTable) = List::fold(allCacheLines.clone(), Arc::new({ let __pe_b1 = cacheLineSize.clone(); let __pe_b2 = varIdxOffsets.clone(); let __pe_b3 = cacheVariablesArray.clone(); move |__pe_a0, __pe_a4| addCacheLineMapToVarArrayMapping(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_a4) }), (varArrayIndexMappingHashTable.clone(), varIndexMappingHashTable.clone()));
            {let _arr = currentVarIndices.clone(); let _val = currentVarIndices.clone().borrow()[(1-1) as usize].clone() + intMul((cacheLinesFloat.clone().len() as i32), maxNumElemsFloat.clone()); _arr.borrow_mut()[(1-1) as usize] = _val; _arr};
            {let _arr = currentVarIndices.clone(); _arr.borrow_mut()[(2-1) as usize] = intMul((cacheLinesInt.clone().len() as i32), maxNumElemsInt.clone()) + 1; _arr};
            {let _arr = currentVarIndices.clone(); _arr.borrow_mut()[(3-1) as usize] = intMul((cacheLinesBool.clone().len() as i32), maxNumElemsBool.clone()) + 1; _arr};
            {let _arr = currentVarIndices.clone(); _arr.borrow_mut()[(4-1) as usize] = 1; _arr};
            (currentVarIndices, varArrayIndexMappingHashTable, varIndexMappingHashTable) = SimCodeUtil::addVarToArrayIndexMappings(notOptimizedVarsFloat.clone().reverse(), VARDATATYPE_FLOAT.clone(), currentVarIndices.clone(), varArrayIndexMappingHashTable.clone(), varIndexMappingHashTable.clone())?;
            (currentVarIndices, varArrayIndexMappingHashTable, varIndexMappingHashTable) = SimCodeUtil::addVarToArrayIndexMappings(notOptimizedVarsInt.clone().reverse(), VARDATATYPE_INTEGER.clone(), currentVarIndices.clone(), varArrayIndexMappingHashTable.clone(), varIndexMappingHashTable.clone())?;
            (currentVarIndices, varArrayIndexMappingHashTable, varIndexMappingHashTable) = SimCodeUtil::addVarToArrayIndexMappings(notOptimizedVarsBool.clone().reverse(), VARDATATYPE_BOOLEAN.clone(), currentVarIndices.clone(), varArrayIndexMappingHashTable.clone(), varIndexMappingHashTable.clone())?;
            (currentVarIndices, varArrayIndexMappingHashTable, varIndexMappingHashTable) = SimCodeUtil::addVarToArrayIndexMappings(notOptimizedVarsString.clone().reverse(), VARDATATYPE_STRING.clone(), currentVarIndices.clone(), varArrayIndexMappingHashTable.clone(), varIndexMappingHashTable.clone())?;
            (currentVarIndices, varArrayIndexMappingHashTable, varIndexMappingHashTable) = SimCodeUtil::addVarToArrayIndexMappings(iAliasVars.clone(), VARDATATYPE_FLOAT.clone(), currentVarIndices.clone(), varArrayIndexMappingHashTable.clone(), varIndexMappingHashTable.clone())?;
            (currentVarIndices, varArrayIndexMappingHashTable, varIndexMappingHashTable) = SimCodeUtil::addVarToArrayIndexMappings(iIntAliasVars.clone(), VARDATATYPE_INTEGER.clone(), currentVarIndices.clone(), varArrayIndexMappingHashTable.clone(), varIndexMappingHashTable.clone())?;
            (currentVarIndices, varArrayIndexMappingHashTable, varIndexMappingHashTable) = SimCodeUtil::addVarToArrayIndexMappings(iBoolAliasVars.clone(), VARDATATYPE_BOOLEAN.clone(), currentVarIndices.clone(), varArrayIndexMappingHashTable.clone(), varIndexMappingHashTable.clone())?;
            (currentVarIndices, varArrayIndexMappingHashTable, varIndexMappingHashTable) = SimCodeUtil::addVarToArrayIndexMappings(iStringAliasVars.clone(), VARDATATYPE_STRING.clone(), currentVarIndices.clone(), varArrayIndexMappingHashTable.clone(), varIndexMappingHashTable.clone())?;
            varSizeFloat = varSizeFloat.clone() + intMul((cacheLinesFloat.clone().len() as i32), maxNumElemsFloat.clone()) + (notOptimizedVarsFloat.clone().len() as i32);
            varSizeInt = intMul((cacheLinesInt.clone().len() as i32), maxNumElemsInt.clone()) + (notOptimizedVarsInt.clone().len() as i32);
            varSizeBool = intMul((cacheLinesBool.clone().len() as i32), maxNumElemsBool.clone()) + (notOptimizedVarsBool.clone().len() as i32);
            varSizeString = (notOptimizedVarsString.clone().len() as i32);
            (varArrayIndexMappingHashTable.clone(), varIndexMappingHashTable.clone(), Some(HpcOmSimCode::MemoryMap::MEMORYMAP_ARRAY { floatArraySize: varSizeFloat.clone(), intArraySize: varSizeInt.clone(), boolArraySize: varSizeBool.clone(), stringArraySize: varSizeString.clone() }))
        },
        (CacheMap::UNIFORM_CACHEMAP { .. }, _, _, _, _, _, _, _, _, _) => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("ConvertCacheToVarArrayMapping: Uniform-CacheMap not supported!")).clone()])?;
            bail!("fail")
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("ConvertCacheToVarArrayMapping: CacheMap-Type not supported!")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oVarToArrayIndexMapping, oVarToIndexMapping, oMemoryMap))
}

fn addCacheLineMapToVarArrayMapping(mut iCacheLineMap: CacheLineMap, mut iCacheLineSize: i32, mut iVarIdxOffsets: metamodelica::Array<i32>, mut iCacheVariables: metamodelica::Array<SimCodeVar::SimVar>, mut iPositionMapping: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (HashTableCrIListArray::FuncHashCref, HashTableCrIListArray::FuncCrefEqual, HashTableCrIListArray::FuncCrefStr, HashTableCrIListArray::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr)))) -> Result<((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (HashTableCrIListArray::FuncHashCref, HashTableCrIListArray::FuncCrefEqual, HashTableCrIListArray::FuncCrefStr, HashTableCrIListArray::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr)))> {
    let mut oPositionMapping: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (HashTableCrIListArray::FuncHashCref, HashTableCrIListArray::FuncCrefEqual, HashTableCrIListArray::FuncCrefStr, HashTableCrIListArray::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr)));
    let mut varArrayIndexMappingHashTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (HashTableCrIListArray::FuncHashCref, HashTableCrIListArray::FuncCrefEqual, HashTableCrIListArray::FuncCrefStr, HashTableCrIListArray::FuncExpStr));
    let mut varIndexMappingHashTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr));
    let mut idx: i32 = 0;
    let mut arrayIdx: i32 = 0;
    let mut entries: Arc<metamodelica::List<CacheLineEntry>> = metamodelica::nil();
    let mut head: CacheLineEntry;
    let mut dataType: i32 = 0;
    let mut size: i32 = 0;
    let mut iPositionMappingList: Arc<metamodelica::List<(i32, i32, i32)>> = metamodelica::nil();
    oPositionMapping = (match (iCacheLineMap.clone(), iCacheLineSize.clone(), iVarIdxOffsets.clone(), iCacheVariables.clone(), iPositionMapping.clone()) {
        (CacheLineMap { entries: mut entries, idx: mut idx, .. }, _, _, _, (mut varArrayIndexMappingHashTable, mut varIndexMappingHashTable)) => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(entries.clone()) {
                Deref @ metamodelica::List::Cons { head: CacheLineEntry { size: __pa0, dataType: __pa1, .. }, tail: _ } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            size = __pa0.clone();
            dataType = __pa1.clone();
            (varArrayIndexMappingHashTable, varIndexMappingHashTable) = List::fold(entries.clone(), Arc::new({ let __pe_b1 = dataType.clone(); let __pe_b2 = (idx.clone(), iCacheLineSize.clone()); let __pe_b3 = iVarIdxOffsets.clone(); let __pe_b4 = iCacheVariables.clone(); move |__pe_a0, __pe_a5| addCacheLineEntryToVarArrayMapping(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_a5) }), iPositionMapping.clone());
            let _ = {let _arr = iVarIdxOffsets.clone(); let _val = intAdd(iVarIdxOffsets.clone().borrow()[(dataType.clone()-1) as usize].clone(), intDiv(iCacheLineSize.clone(), size.clone())); _arr.borrow_mut()[(dataType.clone()-1) as usize] = _val; _arr};
            (varArrayIndexMappingHashTable.clone(), varIndexMappingHashTable.clone())
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("addCacheLineMapToVarArrayMapping failed! CacheLineMap-Type not supported!")).clone()])?;
            bail!("fail")
        },
    });
    Ok(oPositionMapping)
}

fn addCacheLineEntryToVarArrayMapping(mut iCacheLineEntry: CacheLineEntry, mut iArrayIdx: i32, mut iClIdxSize: (i32, i32), mut iVarIdxOffsets: metamodelica::Array<i32>, mut iCacheVariables: metamodelica::Array<SimCodeVar::SimVar>, mut iPositionMapping: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (HashTableCrIListArray::FuncHashCref, HashTableCrIListArray::FuncCrefEqual, HashTableCrIListArray::FuncCrefStr, HashTableCrIListArray::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr)))) -> Result<((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (HashTableCrIListArray::FuncHashCref, HashTableCrIListArray::FuncCrefEqual, HashTableCrIListArray::FuncCrefStr, HashTableCrIListArray::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr)))> {
    let mut oPositionMapping: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (HashTableCrIListArray::FuncHashCref, HashTableCrIListArray::FuncCrefEqual, HashTableCrIListArray::FuncCrefStr, HashTableCrIListArray::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr)));
    let mut varArrayIndexMappingHashTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (HashTableCrIListArray::FuncHashCref, HashTableCrIListArray::FuncCrefEqual, HashTableCrIListArray::FuncCrefStr, HashTableCrIListArray::FuncExpStr));
    let mut varIndexMappingHashTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr));
    let mut clIdx: i32 = 0;
    let mut clSize: i32 = 0;
    let mut iPositionMappingList: Arc<metamodelica::List<(i32, i32, i32)>> = metamodelica::nil();
    let mut scVarIdx: i32 = 0;
    let mut start: i32 = 0;
    let mut size: i32 = 0;
    let mut arrayPosition: i32 = 0;
    let mut highestIdx: i32 = 0;
    let mut offset: i32 = 0;
    let mut arridx: i32 = 0;
    let mut currentVarIndices: metamodelica::Array<i32>;
    oPositionMapping = (match (iCacheLineEntry.clone(), iArrayIdx.clone(), iClIdxSize.clone(), iVarIdxOffsets.clone(), iCacheVariables.clone(), iPositionMapping.clone()) {
        (CacheLineEntry { size: mut size, start: mut start, scVarIdx: mut scVarIdx, .. }, _, (mut clIdx, mut clSize), _, _, (mut varArrayIndexMappingHashTable, mut varIndexMappingHashTable)) => {
            offset = iVarIdxOffsets.clone().borrow()[(iArrayIdx.clone()-1) as usize].clone();
            arrayPosition = intDiv(start.clone(), size.clone()) + offset.clone();
            currentVarIndices = arrayCreate(4, arrayPosition.clone());
            (_, varArrayIndexMappingHashTable, varIndexMappingHashTable) = SimCodeUtil::addVarToArrayIndexMapping(iCacheVariables.clone().borrow()[((iCacheVariables.clone().borrow().len() as i32) - scVarIdx.clone() + 1-1) as usize].clone(), iArrayIdx.clone(), currentVarIndices.clone(), varArrayIndexMappingHashTable.clone(), varIndexMappingHashTable.clone())?;
            (varArrayIndexMappingHashTable.clone(), varIndexMappingHashTable.clone())
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("addCacheLineEntryToVarArrayMapping failed! Unsupported entry-type\n")).clone()])?;
            bail!("fail")
        },
    });
    Ok(oPositionMapping)
}

fn convertCacheToVarArrayMapping2Helper(mut iArray: metamodelica::Array<i32>, mut iOffset: i32, mut iIndex: i32) -> Result<metamodelica::Array<i32>> {
    let mut oArray: metamodelica::Array<i32>;
    let mut tmpArray: metamodelica::Array<i32>;
    let mut i: i32 = 0;
    tmpArray = iArray.clone();
    let __range0 = 1..=(tmpArray.clone().borrow().len() as i32);
    for mut i in __range0 {
        if intNe(i.clone(), iIndex.clone()) {
            tmpArray = {let _arr = tmpArray.clone(); let _val = tmpArray.clone().borrow()[(i.clone()-1) as usize].clone() + iOffset.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = _val; _arr};
        }
    }
    oArray = tmpArray.clone();
    Ok(oArray)
}

fn getNotOptimizedVarsByCacheLineMapping(mut iScVarCLMapping: metamodelica::Array<(i32, i32)>, mut iAllVarsMapping: metamodelica::Array<Option<SimCodeVar::SimVar>>, mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>) -> (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) {
    let mut oNotOptimizedVars: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
    (oNotOptimizedVars, _) = Array::fold(iScVarCLMapping.clone(), Arc::new({ let __pe_b1 = iAllVarsMapping.clone(); let __pe_b2 = iSimCodeVarTypes.clone(); move |__pe_a0, __pe_a3| getNotOptimizedVarsByCacheLineMapping0(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }), ((metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil()), 1));
    oNotOptimizedVars
}

fn getNotOptimizedVarsByCacheLineMapping0(mut iScVarCLMapping: (i32, i32), mut iAllVarsMapping: metamodelica::Array<Option<SimCodeVar::SimVar>>, mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>, mut iEntries: ((Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), i32)) -> Result<((Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), i32)> {
    let mut oEntries: ((Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), i32);
    let mut tmpSimVarsFloat: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpSimVarsInt: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpSimVarsBool: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpSimVarsString: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut scVarIdx: i32 = 0;
    let mut dataType: i32 = 0;
    oEntries = 'mc: {
        let __mc_input = (iScVarCLMapping.clone(), iAllVarsMapping.clone(), iSimCodeVarTypes.clone(), iEntries.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (((-1), _), _, _, ((tmpSimVarsFloat, tmpSimVarsInt, tmpSimVarsBool, tmpSimVarsString), scVarIdx)) => {
                    let mut tmpSimVarsFloat = (*tmpSimVarsFloat).clone();
                    let mut tmpSimVarsInt = (*tmpSimVarsInt).clone();
                    let mut tmpSimVarsBool = (*tmpSimVarsBool).clone();
                    let mut tmpSimVarsString = (*tmpSimVarsString).clone();
                    let mut dataType: i32 = dataType.clone();
                    dataType = Util::tuple31(iSimCodeVarTypes.clone().borrow()[(scVarIdx.clone()-1) as usize].clone());
                    if intEq(dataType.clone(), VARDATATYPE_FLOAT.clone()) {
                        tmpSimVarsFloat = cons(scVarIdx.clone(), tmpSimVarsFloat.clone());
                    } else {
                        if intEq(dataType.clone(), VARDATATYPE_INTEGER.clone()) {
                            tmpSimVarsInt = cons(scVarIdx.clone(), tmpSimVarsInt.clone());
                        } else {
                            if intEq(dataType.clone(), VARDATATYPE_BOOLEAN.clone()) {
                                        tmpSimVarsBool = cons(scVarIdx.clone(), tmpSimVarsBool.clone());
                            } else {
                                        if intEq(dataType.clone(), VARDATATYPE_STRING.clone()) {
                                            tmpSimVarsString = cons(scVarIdx.clone(), tmpSimVarsString.clone());
                                        }
                            }
                        }
                    }
                    Ok(((tmpSimVarsFloat.clone(), tmpSimVarsInt.clone(), tmpSimVarsBool.clone(), tmpSimVarsString.clone()), scVarIdx.clone() + 1))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, ((tmpSimVarsFloat, tmpSimVarsInt, tmpSimVarsBool, tmpSimVarsString), scVarIdx)) => {
                    Ok(((tmpSimVarsFloat.clone(), tmpSimVarsInt.clone(), tmpSimVarsBool.clone(), tmpSimVarsString.clone()), scVarIdx.clone() + 1))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oEntries)
}

// -------------------------------------------
// ANALYSIS
// -------------------------------------------
fn evaluateCacheBehaviour(mut iVarToIndexMappingHashTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr)), mut iSimVarIdxMappingHashTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr)), mut taskSolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskUnsolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iNumberOfThreads: i32, mut iCacheLineSize: i32, mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>) -> () {
    let mut varToCLMapping: metamodelica::Array<i32>;
    let mut varTypeCLOffset: metamodelica::Array<i32>;
    ()
}

fn createVarCLMappingFromVarArrayIndexHashTable(mut iVarToIndexMappingHashTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr)), mut iSimVarIdxMappingHashTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr)), mut iCacheLineSize: i32, mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut oNumberOfVars: metamodelica::Array<i32>;
    let mut oVarToCLMapping: metamodelica::Array<i32>;
    let mut hashTableElements: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut hashTableElement: (Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>);
    let mut varToCLMapping: metamodelica::Array<i32>;
    let mut numberOfVars: metamodelica::Array<i32>;
    let mut maxNumberOfVarsInCL: metamodelica::Array<i32>;
    let mut pos: i32 = 0;
    let mut id: i32 = 0;
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    varToCLMapping = arrayCreate((iSimCodeVarTypes.clone().borrow().len() as i32), -1);
    numberOfVars = arrayCreate(3, 0);
    hashTableElements = BaseHashTable::hashTableList(iVarToIndexMappingHashTable.clone());
    for mut hashTableElement in &*hashTableElements.clone() {
        let mut hashTableElement = hashTableElement.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(hashTableElement.clone()) {
            (__pa0, Deref @ metamodelica::List::Cons { head: __pa1, tail: _ }) => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        cref = __pa0.clone();
        pos = __pa1.clone();
    }
    oNumberOfVars = numberOfVars.clone();
    oVarToCLMapping = varToCLMapping.clone();
    Ok((oNumberOfVars, oVarToCLMapping))
}

fn createCacheLineThreadProperties(mut iCacheLine: CacheLineMap, mut iNumberOfThreads: i32, mut iCacheLineSize: i32, mut iCacheLineThreadProperties: metamodelica::Array<metamodelica::Array<metamodelica::Real>>) -> Result<()> {
    let mut bytesPerThread: metamodelica::Array<i32>;
    let mut threadProperties: metamodelica::Array<metamodelica::Real>;
    let mut cacheLineIdx: i32 = 0;
    let mut threadOwner: i32 = 0;
    let mut size: i32 = 0;
    let mut threadIdx: i32 = 0;
    let mut numBytesFree: i32 = 0;
    let mut numBytesUnassigned: i32 = 0;
    let mut entries: Arc<metamodelica::List<CacheLineEntry>> = metamodelica::nil();
    let mut entry: CacheLineEntry;
    let mut sizeReal: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let CacheLineMap { numBytesFree: __pa0, entries: __pa1, idx: __pa2 } = (iCacheLine.clone()) else { bail!("pattern mismatch") };
    numBytesFree = __pa0.clone();
    entries = __pa1.clone();
    cacheLineIdx = __pa2.clone();
    numBytesUnassigned = 0;
    threadProperties = arrayCreate(iNumberOfThreads.clone(), metamodelica::OrderedFloat(0.0_f64));
    bytesPerThread = arrayCreate(iNumberOfThreads.clone(), 0);
    for mut entry in &*entries.clone() {
        let mut entry = entry.clone();
        let CacheLineEntry { size: __pa3, threadOwner: __pa4, .. } = (entry.clone()) else { bail!("pattern mismatch") };
        size = __pa3.clone();
        threadOwner = __pa4.clone();
        if intLt(threadOwner.clone(), 0) {
            numBytesUnassigned = numBytesUnassigned.clone() + size.clone();
        } else {
            bytesPerThread = {let _arr = bytesPerThread.clone(); let _val = bytesPerThread.clone().borrow()[(threadOwner.clone()-1) as usize].clone() + size.clone(); _arr.borrow_mut()[(threadOwner.clone()-1) as usize] = _val; _arr};
        }
    }
    sizeReal = intReal(iCacheLineSize.clone() - numBytesFree.clone() - numBytesUnassigned.clone());
    if realGt(sizeReal.clone(), metamodelica::OrderedFloat((0) as f64)) {
        for mut threadIdx in 1..=iNumberOfThreads.clone() {
            {let _arr = threadProperties.clone(); let _val = realDiv(intReal(bytesPerThread.clone().borrow()[(threadIdx.clone()-1) as usize].clone()), sizeReal.clone()); _arr.borrow_mut()[(threadIdx.clone()-1) as usize] = _val; _arr};
        }
    }
    {let _arr = iCacheLineThreadProperties.clone(); _arr.borrow_mut()[(cacheLineIdx.clone()-1) as usize] = threadProperties.clone(); _arr};
    Ok(())
}

fn calculateLocCoRead(mut iTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iNodeSimCodeVarMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iScVarCLMapping: metamodelica::Array<(i32, i32)>, mut cacheLineThreadProperties: metamodelica::Array<metamodelica::Array<metamodelica::Real>>, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>) -> Result<metamodelica::Real> {
    let mut oLocCoRead: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut nodeIdx: i32 = 0;
    let mut numberOfNodes: i32 = 0;
    let mut threadIdx: i32 = 0;
    let mut sum: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut locCoRead: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    numberOfNodes = (iNodeSimCodeVarMapping.clone().borrow().len() as i32);
    sum = metamodelica::OrderedFloat(0.0_f64);
    for mut nodeIdx in 1..=numberOfNodes.clone() {
        threadIdx = Util::tuple31(iSchedulerInfo.clone().borrow()[(nodeIdx.clone()-1) as usize].clone());
        locCoRead = calculateLocCoReadForTask(nodeIdx.clone(), threadIdx.clone(), iTaskGraphT.clone(), iNodeSimCodeVarMapping.clone(), iScVarCLMapping.clone(), cacheLineThreadProperties.clone())?;
        sum = sum.clone() + locCoRead.clone();
    }
    if intGt(numberOfNodes.clone(), 0) {
        oLocCoRead = realDiv(sum.clone(), metamodelica::OrderedFloat((numberOfNodes.clone()) as f64));
    } else {
        oLocCoRead = metamodelica::OrderedFloat(1.0_f64);
    }
    Ok(oLocCoRead)
}

fn calculateLocCoReadForTask(mut iNodeIdx: i32, mut iThreadIdx: i32, mut iTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iNodeSimCodeVarMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iScVarCLMapping: metamodelica::Array<(i32, i32)>, mut iCacheLineThreadProperties: metamodelica::Array<metamodelica::Array<metamodelica::Real>>) -> Result<metamodelica::Real> {
    let mut oLocCoRead: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut predecessor: i32 = 0;
    let mut threadIdx: i32 = 0;
    let mut numberOfPredecessors: i32 = 0;
    let mut predecessors: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut sum: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    sum = metamodelica::OrderedFloat(0.0_f64);
    predecessors = iTaskGraphT.clone().borrow()[(iNodeIdx.clone()-1) as usize].clone();
    numberOfPredecessors = (predecessors.clone().len() as i32);
    for mut predecessor in &*predecessors.clone() {
        let mut predecessor = predecessor.clone();
        sum = sum.clone() + calculateLocCoForTask(predecessor.clone(), iThreadIdx.clone(), iNodeSimCodeVarMapping.clone().borrow()[(predecessor.clone()-1) as usize].clone(), iScVarCLMapping.clone(), iCacheLineThreadProperties.clone())?;
    }
    if intGt(numberOfPredecessors.clone(), 0) {
        oLocCoRead = realDiv(sum.clone(), metamodelica::OrderedFloat((numberOfPredecessors.clone()) as f64));
    } else {
        oLocCoRead = metamodelica::OrderedFloat(1.0_f64);
    }
    Ok(oLocCoRead)
}

fn calculateLocCoWrite(mut iNodeSimCodeVarMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iScVarCLMapping: metamodelica::Array<(i32, i32)>, mut cacheLineThreadProperties: metamodelica::Array<metamodelica::Array<metamodelica::Real>>, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>) -> Result<metamodelica::Real> {
    let mut oLocCoWrite: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut nodeIdx: i32 = 0;
    let mut numberOfNodes: i32 = 0;
    let mut threadIdx: i32 = 0;
    let mut sum: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut locCoWrite: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    numberOfNodes = (iNodeSimCodeVarMapping.clone().borrow().len() as i32);
    sum = metamodelica::OrderedFloat(0.0_f64);
    for mut nodeIdx in 1..=numberOfNodes.clone() {
        threadIdx = Util::tuple31(iSchedulerInfo.clone().borrow()[(nodeIdx.clone()-1) as usize].clone());
        locCoWrite = calculateLocCoForTask(nodeIdx.clone(), threadIdx.clone(), iNodeSimCodeVarMapping.clone().borrow()[(nodeIdx.clone()-1) as usize].clone(), iScVarCLMapping.clone(), cacheLineThreadProperties.clone())?;
        sum = sum.clone() + locCoWrite.clone();
    }
    if intGt(numberOfNodes.clone(), 0) {
        oLocCoWrite = realDiv(sum.clone(), metamodelica::OrderedFloat((numberOfNodes.clone()) as f64));
    } else {
        oLocCoWrite = metamodelica::OrderedFloat(1.0_f64);
    }
    Ok(oLocCoWrite)
}

fn calculateLocCoForTask(mut iTaskIdx: i32, mut iThreadIdx: i32, mut iNodeSimCodeVarMapping: Arc<metamodelica::List<i32>>, mut iScVarCLMapping: metamodelica::Array<(i32, i32)>, mut iCacheLineThreadProperties: metamodelica::Array<metamodelica::Array<metamodelica::Real>>) -> Result<metamodelica::Real> {
    let mut oLocCo: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut simCodeVar: i32 = 0;
    let mut index: i32 = 0;
    let mut clIdx: i32 = 0;
    let mut sum: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    sum = metamodelica::OrderedFloat(0.0_f64);
    for mut simCodeVar in &*iNodeSimCodeVarMapping.clone() {
        let mut simCodeVar = simCodeVar.clone();
        clIdx = Util::tuple21(iScVarCLMapping.clone().borrow()[(simCodeVar.clone()-1) as usize].clone());
        sum = sum.clone() + iCacheLineThreadProperties.clone().borrow()[(clIdx.clone()-1) as usize].clone().borrow()[(iThreadIdx.clone()-1) as usize].clone();
    }
    oLocCo = realDiv(sum.clone(), intReal((iNodeSimCodeVarMapping.clone().len() as i32)));
    Ok(oLocCo)
}

// -------------------------------------------
// MAPPINGS
// -------------------------------------------
fn fillSimVarHashTable(mut iSimVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut iOffset: i32, mut iType: i32, mut iHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr))> {
    let mut oHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr));
    let mut tmpHashTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr));
    let mut simVar: SimCodeVar::SimVar;
    let mut index: i32 = 0;
    let mut name: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    tmpHashTable = iHt.clone();
    for mut simVar in &*iSimVars.clone() {
        let mut simVar = simVar.clone();
        let SimCodeVar::SIMVAR { index: __pa0, name: __pa1, .. } = (simVar.clone()) else { bail!("pattern mismatch") };
        index = __pa0.clone();
        name = __pa1.clone();
        index = index.clone() + 1;
        tmpHashTable = BaseHashTable::add((name.clone(), list![index.clone(), iOffset.clone(), iType.clone()]), tmpHashTable.clone())?;
    }
    oHt = tmpHashTable.clone();
    Ok(oHt)
}

fn transposeScVarTaskMapping(mut iScVarTaskMapping: metamodelica::Array<i32>, mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut oNodeSimCodeVarMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut tmpNodeSimCodeVarMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut scVarIdx: i32 = 0;
    let mut taskIdx: i32 = 0;
    let mut oldList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    tmpNodeSimCodeVarMapping = arrayCreate((iTaskGraph.clone().borrow().len() as i32), metamodelica::nil());
    let __range0 = 1..=(iScVarTaskMapping.clone().borrow().len() as i32);
    for mut scVarIdx in __range0 {
        taskIdx = iScVarTaskMapping.clone().borrow()[(scVarIdx.clone()-1) as usize].clone();
        if intGt(taskIdx.clone(), 0) {
            oldList = tmpNodeSimCodeVarMapping.clone().borrow()[(taskIdx.clone()-1) as usize].clone();
            oldList = cons(scVarIdx.clone(), oldList.clone());
            let _ = {let _arr = tmpNodeSimCodeVarMapping.clone(); _arr.borrow_mut()[(taskIdx.clone()-1) as usize] = oldList.clone(); _arr};
        }
    }
    oNodeSimCodeVarMapping = tmpNodeSimCodeVarMapping.clone();
    Ok(oNodeSimCodeVarMapping)
}

fn transposeTasksScVarsMapping(mut iTasksScVarMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iNumberOfScVars: i32) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut oScVarTasksMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut tmpScVarTasksMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut scVarIdx: i32 = 0;
    let mut taskIdx: i32 = 0;
    let mut oldList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut scVarIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    tmpScVarTasksMapping = arrayCreate(iNumberOfScVars.clone(), metamodelica::nil());
    let __range0 = 1..=(iTasksScVarMapping.clone().borrow().len() as i32);
    for mut taskIdx in __range0 {
        scVarIdc = iTasksScVarMapping.clone().borrow()[(taskIdx.clone()-1) as usize].clone();
        for mut scVarIdx in &*scVarIdc.clone() {
            let mut scVarIdx = scVarIdx.clone();
            if intGt(scVarIdx.clone(), 0) {
                oldList = tmpScVarTasksMapping.clone().borrow()[(scVarIdx.clone()-1) as usize].clone();
                oldList = cons(taskIdx.clone(), oldList.clone());
                let _ = {let _arr = tmpScVarTasksMapping.clone(); _arr.borrow_mut()[(scVarIdx.clone()-1) as usize] = oldList.clone(); _arr};
            }
        }
    }
    oScVarTasksMapping = tmpScVarTasksMapping.clone();
    Ok(oScVarTasksMapping)
}

fn getEqSCVarMapping(mut iEqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, mut iHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr))) -> metamodelica::Array<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut oMapping: metamodelica::Array<metamodelica::Array<Arc<metamodelica::List<i32>>>>;
    let mut tmpMapping: Arc<metamodelica::List<metamodelica::Array<Arc<metamodelica::List<i32>>>>> = metamodelica::nil();
    tmpMapping = List::map1(iEqSystems.clone(), (std::sync::Arc::new(getEqSCVarMappingByEqSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr))) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> + 'static>), iHt.clone());
    oMapping = metamodelica::arrayFromVec(tmpMapping.clone().into_iter().cloned().collect());
    oMapping
}

fn getEqSCVarMappingByEqSystem(mut iEqSystem: Arc<BackendDAE::EqSystem>, mut iHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr))) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut oMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut orderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut equOptList: Arc<metamodelica::List<Option<Arc<BackendDAE::Equation>>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(iEqSystem.clone()) {
        Deref @ BackendDAE::EqSystem { orderedEqs: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    orderedEqs = __pa0.clone();
    equOptList = Arc::new(ExpandableArray::getData(orderedEqs.clone()).borrow().iter().cloned().collect::<metamodelica::List<_>>());
    oMapping = metamodelica::arrayFromVec(List::map1Option(equOptList.clone(), (std::sync::Arc::new(getEqSCVarMapping0) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr))) -> Result<Arc<metamodelica::List<i32>>> + 'static>), iHt.clone())?.into_iter().cloned().collect());
    Ok(oMapping)
}

fn getEqSCVarMapping0(mut iEquation: Arc<BackendDAE::Equation>, mut iHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr))) -> Result<Arc<metamodelica::List<i32>>> {
    let mut oMapping: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut varIdcList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let (_, (_, (_, __pa0))) = BackendEquation::traverseExpsOfEquation(iEquation.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(createMemoryMapTraverse0) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr)), Arc<metamodelica::List<i32>>)) -> Result<(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr)), Arc<metamodelica::List<i32>>))> + 'static>), (iHt.clone(), metamodelica::nil())))?;
    oMapping = __pa0.clone();
    Ok(oMapping)
}

fn createMemoryMapTraverse0(mut inExp: Arc<DAE::Exp>, mut inTpl: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr)), Arc<metamodelica::List<i32>>)) -> Result<(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr)), Arc<metamodelica::List<i32>>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut oTpl: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr)), Arc<metamodelica::List<i32>>);
    let mut iVarList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut oVarList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut varInfo: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut varIdx: i32 = 0;
    let mut varHead: i32 = 0;
    let mut iHashTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr));
    let mut iExp: Arc<DAE::Exp>;
    let mut componentRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    (outExp, oTpl) = 'mc: {
        let __mc_input = (inExp.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (iExp @ Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, (iHashTable, iVarList)) => {
                    let mut iVarList = (*iVarList).clone();
                    let mut varInfo: Arc<metamodelica::List<i32>> = varInfo.clone();
                    let mut varHead: i32 = varHead.clone();
                    let mut varIdx: i32 = varIdx.clone();
                    let mut oVarList: Arc<metamodelica::List<i32>> = oVarList.clone();
                    varInfo = BaseHashTable::get(componentRef.clone(), iHashTable.clone())?;
                    varIdx = listHead(varInfo.clone())? + List::second(varInfo.clone())?;
                    if boolNot(iVarList.clone().is_empty()) {
                        varHead = listHead(iVarList.clone())?;
                        if intEq(varHead.clone(), varIdx.clone()) {
                            iVarList = listRest(iVarList.clone())?;
                        }
                    }
                    varInfo = BaseHashTable::get(ComponentReference::crefPrefixDer(componentRef.clone()), iHashTable.clone())?;
                    varIdx = listHead(varInfo.clone())? + List::second(varInfo.clone())?;
                    oVarList = cons(varIdx.clone(), iVarList.clone());
                    Ok((iExp.clone(), (iHashTable.clone(), oVarList.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (iExp @ Deref @ DAE::Exp::CREF { componentRef, .. }, (iHashTable, iVarList)) => {
                    let mut varIdx: i32 = varIdx.clone();
                    let mut oVarList: Arc<metamodelica::List<i32>> = oVarList.clone();
                    let mut varInfo: Arc<metamodelica::List<i32>> = varInfo.clone();
                    varInfo = BaseHashTable::get(componentRef.clone(), iHashTable.clone())?;
                    varIdx = listHead(varInfo.clone())? + List::second(varInfo.clone())?;
                    oVarList = cons(varIdx.clone(), iVarList.clone());
                    Ok((iExp.clone(), (iHashTable.clone(), oVarList.clone())))
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
    Ok((outExp, oTpl))
}

fn getSimCodeVarNodeMapping(mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iEqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, mut iNumScVars: i32, mut iCompNodeMapping: metamodelica::Array<i32>, mut iVarNameSCVarIdxMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr))) -> Result<metamodelica::Array<i32>> {
    let mut oScVarTaskMapping: metamodelica::Array<i32>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut scVarTaskMapping: metamodelica::Array<i32>;
    scVarTaskMapping = arrayCreate(iNumScVars.clone(), -1);
    let HpcOmTaskGraph::TASKGRAPHMETA { varCompMapping: __pa0, .. } = (iTaskGraphMeta.clone()) else { bail!("pattern mismatch") };
    varCompMapping = __pa0.clone();
    (oScVarTaskMapping, _) = Array::fold(varCompMapping.clone(), Arc::new({ let __pe_b1 = iEqSystems.clone(); let __pe_b2 = iVarNameSCVarIdxMapping.clone(); let __pe_b3 = iCompNodeMapping.clone(); move |__pe_a0, __pe_a4| getSimCodeVarNodeMapping0(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_a4) }), (scVarTaskMapping.clone(), 1));
    Ok(oScVarTaskMapping)
}

fn getSimCodeVarNodeMapping0(mut iCompIdx: (i32, i32, i32), mut iEqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, mut iVarNameSCVarIdxMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr)), mut iCompNodeMapping: metamodelica::Array<i32>, mut iScVarTaskMappingVarIdx: (metamodelica::Array<i32>, i32)) -> Result<(metamodelica::Array<i32>, i32)> {
    let mut oScVarTaskMappingVarIdx: (metamodelica::Array<i32>, i32);
    let mut iScVarTaskMapping: metamodelica::Array<i32>;
    let mut varIdx: i32 = 0;
    let mut eqSysIdx: i32 = 0;
    let mut varOffset: i32 = 0;
    let mut scVarIdx: i32 = 0;
    let mut compIdx: i32 = 0;
    let mut nodeIdx: i32 = 0;
    let mut scVarOffset: i32 = 0;
    let mut eqSystem: Arc<BackendDAE::EqSystem>;
    let mut orderedVars: BackendDAE::Variables;
    let mut varOptArr: metamodelica::Array<Option<BackendDAE::Var>>;
    let mut var: BackendDAE::Var;
    let mut varName: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut scVarValues: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut varNameString: ArcStr = arcstr::literal!("");
    oScVarTaskMappingVarIdx = 'mc: {
        let __mc_input = (iCompIdx.clone(), iEqSystems.clone(), iVarNameSCVarIdxMapping.clone(), iCompNodeMapping.clone(), iScVarTaskMappingVarIdx.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((compIdx, eqSysIdx, varOffset), _, _, _, (iScVarTaskMapping, varIdx)) => {
                    let mut iScVarTaskMapping = (*iScVarTaskMapping).clone();
                    let mut varName: Arc<DAE::ComponentRef> = varName.clone();
                    let mut var: BackendDAE::Var;
                    let mut eqSystem: Arc<BackendDAE::EqSystem>;
                    let mut scVarIdx: i32 = scVarIdx.clone();
                    let mut orderedVars: BackendDAE::Variables;
                    let mut nodeIdx: i32 = nodeIdx.clone();
                    let mut scVarValues: Arc<metamodelica::List<i32>> = scVarValues.clone();
                    let mut varNameString: ArcStr = varNameString.clone();
                    let mut scVarOffset: i32 = scVarOffset.clone();
                    let true = (intGt(compIdx.clone(), 0)) else { bail!("pattern mismatch") };
                    eqSystem = (iEqSystems.clone()).get(eqSysIdx.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(eqSystem.clone()) {
                        Deref @ BackendDAE::EqSystem { orderedVars: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    orderedVars = __pa0.clone();
                    var = BackendVariable::getVarAt(orderedVars.clone(), varIdx.clone() - varOffset.clone())?;
                    let BackendDAE::VAR { varName: __pa1, .. } = (var.clone()) else { bail!("pattern mismatch") };
                    varName = __pa1.clone();
                    varName = getModifiedVarName(var.clone())?;
                    scVarValues = BaseHashTable::get(varName.clone(), iVarNameSCVarIdxMapping.clone())?;
                    varNameString = (ComponentReferenceBasics::printComponentRefStr(varName.clone())?).clone();
                    scVarIdx = listHead(scVarValues.clone())?;
                    scVarOffset = List::second(scVarValues.clone())?;
                    scVarIdx = scVarIdx.clone() + scVarOffset.clone();
                    nodeIdx = iCompNodeMapping.clone().borrow()[(compIdx.clone()-1) as usize].clone();
                    iScVarTaskMapping = {let _arr = iScVarTaskMapping.clone(); _arr.borrow_mut()[(scVarIdx.clone()-1) as usize] = nodeIdx.clone(); _arr};
                    Ok((iScVarTaskMapping.clone(), varIdx.clone() + 1))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, (iScVarTaskMapping, varIdx)) => {
                    Ok((iScVarTaskMapping.clone(), varIdx.clone() + 1))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oScVarTaskMappingVarIdx)
}

fn invertEqCompMapping(mut iEqCompMapping: metamodelica::Array<(i32, i32, i32)>, mut iNumOfComps: i32) -> Result<metamodelica::Array<Arc<metamodelica::List<(i32, i32, i32)>>>> {
    let mut oCompEqMapping: metamodelica::Array<Arc<metamodelica::List<(i32, i32, i32)>>>;
    let mut tmpCompEqMapping: metamodelica::Array<Arc<metamodelica::List<(i32, i32, i32)>>>;
    let mut eqIdx: i32 = 0;
    let mut compIdx: i32 = 0;
    let mut eqSystemIdx: i32 = 0;
    let mut offset: i32 = 0;
    let mut eqCompEntry: (i32, i32, i32);
    let mut compEqEntry: Arc<metamodelica::List<(i32, i32, i32)>> = metamodelica::nil();
    tmpCompEqMapping = arrayCreate(iNumOfComps.clone(), metamodelica::nil());
    let __range0 = 1..=(iEqCompMapping.clone().borrow().len() as i32);
    for mut eqIdx in __range0 {
        (compIdx, eqSystemIdx, offset) = iEqCompMapping.clone().borrow()[(eqIdx.clone()-1) as usize].clone();
        compEqEntry = tmpCompEqMapping.clone().borrow()[(compIdx.clone()-1) as usize].clone();
        tmpCompEqMapping = {let _arr = tmpCompEqMapping.clone(); _arr.borrow_mut()[(compIdx.clone()-1) as usize] = cons((eqIdx.clone(), eqSystemIdx.clone(), offset.clone()), compEqEntry.clone()); _arr};
    }
    oCompEqMapping = tmpCompEqMapping.clone();
    Ok(oCompEqMapping)
}

fn invertSccNodeMapping(mut iSccNodeMapping: metamodelica::Array<i32>, mut iNumberOfNodes: i32) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut oNodeSccMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut tmpNodeSccMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut sccIdx: i32 = 0;
    let mut nodeIdx: i32 = 0;
    let mut nodeSccEntry: Arc<metamodelica::List<i32>> = metamodelica::nil();
    tmpNodeSccMapping = arrayCreate(iNumberOfNodes.clone(), metamodelica::nil());
    let __range0 = 1..=(iSccNodeMapping.clone().borrow().len() as i32);
    for mut sccIdx in __range0 {
        nodeIdx = iSccNodeMapping.clone().borrow()[(sccIdx.clone()-1) as usize].clone();
        if intGt(nodeIdx.clone(), 0) {
            nodeSccEntry = tmpNodeSccMapping.clone().borrow()[(nodeIdx.clone()-1) as usize].clone();
            tmpNodeSccMapping = {let _arr = tmpNodeSccMapping.clone(); _arr.borrow_mut()[(nodeIdx.clone()-1) as usize] = cons(sccIdx.clone(), nodeSccEntry.clone()); _arr};
        }
    }
    oNodeSccMapping = tmpNodeSccMapping.clone();
    Ok(oNodeSccMapping)
}

fn flattenEqSimCodeVarMapping(mut iEqSimCodeVarMapping: metamodelica::Array<metamodelica::Array<Arc<metamodelica::List<i32>>>>) -> Result<metamodelica::Array<(i32, Arc<metamodelica::List<i32>>)>> {
    let mut oFlatEqSimCodeVarMapping: metamodelica::Array<(i32, Arc<metamodelica::List<i32>>)>;
    let mut simCodeVarList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpFlatEqSimCodeVarMapping: metamodelica::Array<(i32, Arc<metamodelica::List<i32>>)>;
    let mut eqCount: i32 = 0;
    let mut eqIdx: i32 = 0;
    let mut eqSysIdx: i32 = 0;
    let mut eqSimCodeVarIdx: i32 = 0;
    let mut eqSimCodeVarMappingEntry: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    eqCount = 0;
    let __range0 = 1..=(iEqSimCodeVarMapping.clone().borrow().len() as i32);
    for mut eqSysIdx in __range0 {
        eqSimCodeVarMappingEntry = iEqSimCodeVarMapping.clone().borrow()[(eqSysIdx.clone()-1) as usize].clone();
        eqCount = eqCount.clone() + (eqSimCodeVarMappingEntry.clone().borrow().len() as i32);
    }
    eqIdx = 1;
    tmpFlatEqSimCodeVarMapping = arrayCreate(eqCount.clone(), (-1, metamodelica::nil()));
    let __range1 = 1..=(iEqSimCodeVarMapping.clone().borrow().len() as i32);
    for mut eqSysIdx in __range1 {
        eqSimCodeVarMappingEntry = iEqSimCodeVarMapping.clone().borrow()[(eqSysIdx.clone()-1) as usize].clone();
        let __range2 = 1..=(eqSimCodeVarMappingEntry.clone().borrow().len() as i32);
        for mut eqSimCodeVarIdx in __range2 {
            simCodeVarList = eqSimCodeVarMappingEntry.clone().borrow()[(eqSimCodeVarIdx.clone()-1) as usize].clone();
            tmpFlatEqSimCodeVarMapping = {let _arr = tmpFlatEqSimCodeVarMapping.clone(); _arr.borrow_mut()[(eqIdx.clone()-1) as usize] = (eqSysIdx.clone(), simCodeVarList.clone()); _arr};
            eqIdx = eqIdx.clone() + 1;
        }
    }
    oFlatEqSimCodeVarMapping = tmpFlatEqSimCodeVarMapping.clone();
    Ok(oFlatEqSimCodeVarMapping)
}

fn getModifiedVarName(mut iVar: BackendDAE::Var) -> Result<Arc<DAE::ComponentRef>> {
    let mut oVarName: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut iVarName: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut tmpVarName: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut varKind: BackendDAE::VarKind = BackendDAE::VarKind::ALG_STATE;
    oVarName = (match iVar.clone() {
        BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { index: 1, .. }, varName: ref iVarName, .. } => {
            tmpVarName = Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (arcstr::literal!(DAE::derivativeNamePrefix)).clone(), identType: Arc::new(DAE::Type::T_REAL { varLst: metamodelica::nil() }), subscriptLst: metamodelica::nil(), componentRef: iVarName.clone() });
            tmpVarName.clone()
        },
        BackendDAE::Var { varKind: mut varKind, varName: ref iVarName, .. } => {
            tmpVarName = iVarName.clone();
            tmpVarName.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(oVarName)
}

fn getCacheLineTaskMapping(mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iEqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, mut iVarNameSCVarIdxMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr)), mut iNumCacheLines: i32, mut iSCVarCLMapping: metamodelica::Array<(i32, i32)>) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>)> {
    let mut oCLTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut oScVarTaskMapping: metamodelica::Array<i32>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut tmpCLTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut scVarTaskMapping: metamodelica::Array<i32>;
    tmpCLTaskMapping = arrayCreate(iNumCacheLines.clone(), metamodelica::nil());
    scVarTaskMapping = arrayCreate((iSCVarCLMapping.clone().borrow().len() as i32), -1);
    let HpcOmTaskGraph::TASKGRAPHMETA { varCompMapping: __pa0, .. } = (iTaskGraphMeta.clone()) else { bail!("pattern mismatch") };
    varCompMapping = __pa0.clone();
    (tmpCLTaskMapping, oScVarTaskMapping, _) = Array::fold(varCompMapping.clone(), Arc::new({ let __pe_b1 = iEqSystems.clone(); let __pe_b2 = iVarNameSCVarIdxMapping.clone(); let __pe_b3 = iSCVarCLMapping.clone(); move |__pe_a0, __pe_a4| getCacheLineTaskMapping0(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_a4) }), (tmpCLTaskMapping.clone(), scVarTaskMapping.clone(), 1));
    tmpCLTaskMapping = Array::map1(tmpCLTaskMapping.clone(), (std::sync::Arc::new(List::sort) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), (std::sync::Arc::new(fnptr!(intLt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    oCLTaskMapping = Array::map1(tmpCLTaskMapping.clone(), (std::sync::Arc::new(List::sortedUnique) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    Ok((oCLTaskMapping, oScVarTaskMapping))
}

fn getCacheLineTaskMapping0(mut iNodeIdx: (i32, i32, i32), mut iEqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, mut iVarNameSCVarIdxMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr)), mut iSCVarCLMapping: metamodelica::Array<(i32, i32)>, mut iCLTaskMappingVarIdx: (metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)> {
    let mut oCLTaskMappingVarIdx: (metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32);
    let mut iClTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut iScVarTaskMapping: metamodelica::Array<i32>;
    let mut varIdx: i32 = 0;
    let mut eqSysIdx: i32 = 0;
    let mut varOffset: i32 = 0;
    let mut scVarIdx: i32 = 0;
    let mut clIdx: i32 = 0;
    let mut nodeIdx: i32 = 0;
    let mut scVarOffset: i32 = 0;
    let mut eqSystem: Arc<BackendDAE::EqSystem>;
    let mut orderedVars: BackendDAE::Variables;
    let mut varOptArr: metamodelica::Array<Option<BackendDAE::Var>>;
    let mut var: BackendDAE::Var;
    let mut varName: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut oldVal: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut scVarValues: Arc<metamodelica::List<i32>> = metamodelica::nil();
    oCLTaskMappingVarIdx = 'mc: {
        let __mc_input = (iNodeIdx.clone(), iEqSystems.clone(), iVarNameSCVarIdxMapping.clone(), iSCVarCLMapping.clone(), iCLTaskMappingVarIdx.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((nodeIdx, eqSysIdx, varOffset), _, _, _, (iClTaskMapping, iScVarTaskMapping, varIdx)) => {
                    let mut iClTaskMapping = (*iClTaskMapping).clone();
                    let mut iScVarTaskMapping = (*iScVarTaskMapping).clone();
                    let mut scVarValues: Arc<metamodelica::List<i32>> = scVarValues.clone();
                    let mut scVarOffset: i32 = scVarOffset.clone();
                    let mut oldVal: Arc<metamodelica::List<i32>> = oldVal.clone();
                    let mut eqSystem: Arc<BackendDAE::EqSystem>;
                    let mut clIdx: i32 = clIdx.clone();
                    let mut orderedVars: BackendDAE::Variables;
                    let mut varName: Arc<DAE::ComponentRef> = varName.clone();
                    let mut scVarIdx: i32 = scVarIdx.clone();
                    let mut var: BackendDAE::Var;
                    let true = (intGt(nodeIdx.clone(), 0)) else { bail!("pattern mismatch") };
                    eqSystem = (iEqSystems.clone()).get(eqSysIdx.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(eqSystem.clone()) {
                        Deref @ BackendDAE::EqSystem { orderedVars: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    orderedVars = __pa0.clone();
                    var = BackendVariable::getVarAt(orderedVars.clone(), varIdx.clone() - varOffset.clone())?;
                    let BackendDAE::VAR { varName: __pa1, .. } = (var.clone()) else { bail!("pattern mismatch") };
                    varName = __pa1.clone();
                    varName = getModifiedVarName(var.clone())?;
                    scVarValues = BaseHashTable::get(varName.clone(), iVarNameSCVarIdxMapping.clone())?;
                    scVarIdx = listHead(scVarValues.clone())?;
                    scVarOffset = List::second(scVarValues.clone())?;
                    scVarIdx = scVarIdx.clone() + scVarOffset.clone();
                    (clIdx, _) = iSCVarCLMapping.clone().borrow()[(scVarIdx.clone()-1) as usize].clone();
                    oldVal = iClTaskMapping.clone().borrow()[(clIdx.clone()-1) as usize].clone();
                    iClTaskMapping = {let _arr = iClTaskMapping.clone(); _arr.borrow_mut()[(clIdx.clone()-1) as usize] = cons(nodeIdx.clone(), oldVal.clone()); _arr};
                    iScVarTaskMapping = {let _arr = iScVarTaskMapping.clone(); _arr.borrow_mut()[(scVarIdx.clone()-1) as usize] = nodeIdx.clone(); _arr};
                    Ok((iClTaskMapping.clone(), iScVarTaskMapping.clone(), varIdx.clone() + 1))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, (iClTaskMapping, iScVarTaskMapping, varIdx)) => {
                    Ok((iClTaskMapping.clone(), iScVarTaskMapping.clone(), varIdx.clone() + 1))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oCLTaskMappingVarIdx)
}

fn getTaskSimVarMapping(mut iSccEqMapping: metamodelica::Array<Arc<metamodelica::List<(i32, i32, i32)>>>, mut iNodeSccMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iEqSimCodeVarMapping: metamodelica::Array<(i32, Arc<metamodelica::List<i32>>)>, mut iScVarTaskMapping: metamodelica::Array<i32>, mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>)> {
    let mut oSolvedVars: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut oNotSolvedVars: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut tmpSolvedVars: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut tmpNotSolvedVars: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut scVarMarks: metamodelica::Array<i32>;
    let mut scSolvedVarMarks: metamodelica::Array<i32>;
    let mut nodeSccs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut nodeIdx: i32 = 0;
    let mut sccIdx: i32 = 0;
    let mut eqIdx: i32 = 0;
    let mut var: i32 = 0;
    let mut varTask: i32 = 0;
    let mut varMark: i32 = 0;
    let mut varType: i32 = 0;
    let mut nvar: i32 = 0;
    let mut var: i32 = 0;
    let mut sccEqs: Arc<metamodelica::List<(i32, i32, i32)>> = metamodelica::nil();
    let mut sccEq: (i32, i32, i32);
    match '__try0: {
        tmpSolvedVars = arrayCreate((iNodeSccMapping.clone().borrow().len() as i32), metamodelica::nil());
        tmpNotSolvedVars = arrayCreate((iNodeSccMapping.clone().borrow().len() as i32), metamodelica::nil());
        scVarMarks = arrayCreate((iScVarTaskMapping.clone().borrow().len() as i32), -1);
        scSolvedVarMarks = arrayCreate((iScVarTaskMapping.clone().borrow().len() as i32), -1);
        nvar = (iScVarTaskMapping.clone().borrow().len() as i32);
        let __range1 = 1..=(iNodeSccMapping.clone().borrow().len() as i32);
        for mut nodeIdx in __range1 {
            nodeSccs = iNodeSccMapping.clone().borrow()[(nodeIdx.clone()-1) as usize].clone();
            for mut sccIdx in &*nodeSccs.clone() {
                let mut sccIdx = sccIdx.clone();
                sccEqs = iSccEqMapping.clone().borrow()[(sccIdx.clone()-1) as usize].clone();
                for mut sccEq in &*sccEqs.clone() {
                    let mut sccEq = sccEq.clone();
                    (eqIdx, _, _) = sccEq.clone();
                    (_, eqVars) = iEqSimCodeVarMapping.clone().borrow()[(eqIdx.clone()-1) as usize].clone();
                    for mut v2 in &*eqVars.clone() {
                        let mut v2 = v2.clone();
                        var = if (v2.clone() > nvar.clone()) {v2.clone() - nvar.clone()} else {v2.clone()};
                        varTask = iScVarTaskMapping.clone().borrow()[(var.clone()-1) as usize].clone();
                        varType = Util::tuple31(iSimCodeVarTypes.clone().borrow()[(var.clone()-1) as usize].clone());
                        if intGt(varType.clone(), 0) {
                            if intEq(nodeIdx.clone(), varTask.clone()) {
                                varMark = scSolvedVarMarks.clone().borrow()[(var.clone()-1) as usize].clone();
                                if intNe(varMark.clone(), nodeIdx.clone()) {
                                    tmpSolvedVars = {let _arr = tmpSolvedVars.clone(); let _val = cons(var.clone(), tmpSolvedVars.clone().borrow()[(nodeIdx.clone()-1) as usize].clone()); _arr.borrow_mut()[(nodeIdx.clone()-1) as usize] = _val; _arr};
                                    scSolvedVarMarks = {let _arr = scSolvedVarMarks.clone(); _arr.borrow_mut()[(var.clone()-1) as usize] = nodeIdx.clone(); _arr};
                                }
                            } else {
                                varMark = scVarMarks.clone().borrow()[(var.clone()-1) as usize].clone();
                                if intNe(varMark.clone(), nodeIdx.clone()) {
                                    tmpNotSolvedVars = {let _arr = tmpNotSolvedVars.clone(); let _val = cons(var.clone(), tmpNotSolvedVars.clone().borrow()[(nodeIdx.clone()-1) as usize].clone()); _arr.borrow_mut()[(nodeIdx.clone()-1) as usize] = _val; _arr};
                                    scVarMarks = {let _arr = scVarMarks.clone(); _arr.borrow_mut()[(var.clone()-1) as usize] = nodeIdx.clone(); _arr};
                                }
                            }
                        }
                    }
                }
            }
        }
        oSolvedVars = tmpSolvedVars.clone();
        oNotSolvedVars = tmpNotSolvedVars.clone();
        Ok::<_, anyhow::Error>((nvar.clone(), oNotSolvedVars.clone(), oSolvedVars.clone(), scSolvedVarMarks.clone(), scVarMarks.clone(), tmpNotSolvedVars.clone(), tmpSolvedVars.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5, __try0_o6)) => {
            nvar = __try0_o0;
            oNotSolvedVars = __try0_o1;
            oSolvedVars = __try0_o2;
            scSolvedVarMarks = __try0_o3;
            scVarMarks = __try0_o4;
            tmpNotSolvedVars = __try0_o5;
            tmpSolvedVars = __try0_o6;
        }
        Err(_) => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("HpcOmMemory.getTaskSimVarMapping")); __mm_s.push_str(&*literal!(" failed")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail");
        }
    }
    Ok((oSolvedVars, oNotSolvedVars))
}

// -------------------------------------------
// GRAPH
// -------------------------------------------
fn appendCacheLinesToGraph(mut iCacheMap: CacheMap, mut iNumberOfNodes: i32, mut iEqSimCodeVarMapping: metamodelica::Array<metamodelica::Array<Arc<metamodelica::List<i32>>>>, mut iEqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, mut iVarNameSCVarIdxMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr)), mut ieqCompMapping: metamodelica::Array<(i32, i32, i32)>, mut iScVarTaskMapping: metamodelica::Array<i32>, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>, mut iThreadIdAttributeIdx: i32, mut iCompNodeMapping: metamodelica::Array<i32>, mut iTaskSolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskUnsolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iScVarCLMapping: metamodelica::Array<(i32, i32)>, mut iScVarInfos: metamodelica::Array<ScVarInfo>, mut iGraphInfo: GraphML::GraphInfo) -> Result<GraphML::GraphInfo> {
    let mut oGraphInfo: GraphML::GraphInfo;
    let mut clGroupNodeIdx: i32 = 0;
    let mut graphCount: i32 = 0;
    let mut tmpGraphInfo: GraphML::GraphInfo;
    let mut knownEdges: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut addedVariables: metamodelica::Array<bool>;
    let mut cacheVariables: metamodelica::Array<SimCodeVar::SimVar>;
    let mut cacheLines: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    oGraphInfo = 'mc: {
        let __mc_input = (iCacheMap.clone(), iNumberOfNodes.clone(), iEqSimCodeVarMapping.clone(), iEqSystems.clone(), iVarNameSCVarIdxMapping.clone(), ieqCompMapping.clone(), iScVarTaskMapping.clone(), iSchedulerInfo.clone(), iThreadIdAttributeIdx.clone(), iCompNodeMapping.clone(), iTaskSolvedVarsMapping.clone(), iTaskUnsolvedVarsMapping.clone(), iScVarCLMapping.clone(), iScVarInfos.clone(), iGraphInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, _, _, _, _, _, GraphML::GraphInfo::GRAPHINFO { graphCount, .. }) => {
                    let mut knownEdges: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut addedVariables: metamodelica::Array<bool>;
                    let mut clGroupNodeIdx: i32 = clGroupNodeIdx.clone();
                    let mut cacheVariables: metamodelica::Array<SimCodeVar::SimVar>;
                    let mut tmpGraphInfo: GraphML::GraphInfo;
                    let mut cacheLines: Arc<metamodelica::List<CacheLineMap>> = cacheLines.clone();
                    let true = (intLe(1, graphCount.clone())) else { bail!("pattern mismatch") };
                    knownEdges = arrayCreate(iNumberOfNodes.clone(), metamodelica::nil());
                    addedVariables = arrayCreate((iScVarTaskMapping.clone().borrow().len() as i32), false);
                    let (__pa0, (_, _), (_, __pa1)) = GraphML::addGroupNode((literal!("CL_GoupNode")).clone(), 1, false, (literal!("CL")).clone(), iGraphInfo.clone())?;
                    tmpGraphInfo = __pa0.clone();
                    clGroupNodeIdx = __pa1.clone();
                    cacheLines = getAllCacheLinesOfCacheMap(iCacheMap.clone())?;
                    cacheVariables = metamodelica::arrayFromVec(getCacheVariablesOfCacheMap(iCacheMap.clone())?.into_iter().cloned().collect());
                    tmpGraphInfo = List::fold(cacheLines.clone(), Arc::new({ let __pe_b1 = cacheVariables.clone(); let __pe_b2 = addedVariables.clone(); let __pe_b3 = iSchedulerInfo.clone(); let __pe_b4 = (clGroupNodeIdx.clone(), iThreadIdAttributeIdx.clone()); let __pe_b5 = iScVarTaskMapping.clone(); let __pe_b6 = iVarNameSCVarIdxMapping.clone(); let __pe_b7 = iScVarInfos.clone(); move |__pe_a0, __pe_a8| appendCacheLineMapToGraph(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone(), __pe_b7.clone(), __pe_a8) }), tmpGraphInfo.clone());
                    tmpGraphInfo = appendTaskVarEdgesToGraph(iTaskSolvedVarsMapping.clone(), iTaskUnsolvedVarsMapping.clone(), tmpGraphInfo.clone())?;
                    Ok(tmpGraphInfo.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, _, _, _, _, _, GraphML::GraphInfo::GRAPHINFO { graphCount, .. }) => {
                    let true = (intEq(graphCount.clone(), 0)) else { bail!("pattern mismatch") };
                    Ok(iGraphInfo.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("HpcOmSimCode.appendCacheLinesToGraph failed!\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oGraphInfo)
}

fn appendVariablesToGraph(mut iTaskSolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskUnsolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iNumberOfScVars: i32, mut iGraphIdx: i32, mut iThreadIdAttributeIdx: i32, mut iVarNameSCVarIdxMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr)), mut iAllVarsMapping: metamodelica::Array<Option<SimCodeVar::SimVar>>, mut iScVarInfos: metamodelica::Array<ScVarInfo>, mut iGraphInfo: GraphML::GraphInfo) -> Result<GraphML::GraphInfo> {
    let mut oGraphInfo: GraphML::GraphInfo;
    let mut tmpGraphInfo: GraphML::GraphInfo = iGraphInfo.clone();
    let mut description: ArcStr = arcstr::literal!("");
    let mut threadText: ArcStr = arcstr::literal!("");
    let mut simVarOpt: Option<SimCodeVar::SimVar> = None;
    let mut simVar: SimCodeVar::SimVar;
    let mut varCompRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut nodeLabel: GraphML::NodeLabel;
    let mut isValidVar: bool = false;
    let mut realScVarIdxOffset: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut realScVarIdx: i32 = 0;
    let mut realScVarOffset: i32 = 0;
    let mut threadOwner: i32 = 0;
    for mut varIdx in 1..=iNumberOfScVars.clone() {
        isValidVar = true;
        simVarOpt = iAllVarsMapping.clone().borrow()[(varIdx.clone()-1) as usize].clone();
        description = (literal!("unknown")).clone();
        threadText = (literal!("Th -1")).clone();
        if isSome(simVarOpt.clone()) {
            simVar = Util::getOption(simVarOpt.clone())?;
            varCompRef = simVar.name.clone();
            description = (ComponentReferenceBasics::printComponentRefStr(varCompRef.clone())?).clone();
            isValidVar = BaseHashTable::hasKey(varCompRef.clone(), iVarNameSCVarIdxMapping.clone());
            if BaseHashTable::hasKey(varCompRef.clone(), iVarNameSCVarIdxMapping.clone()) {
                realScVarIdxOffset = BaseHashTable::get(varCompRef.clone(), iVarNameSCVarIdxMapping.clone())?;
                realScVarIdx = (realScVarIdxOffset.clone()).get(1)?;
                realScVarOffset = (realScVarIdxOffset.clone()).get(2)?;
                realScVarIdx = realScVarIdx.clone() + realScVarOffset.clone();
                let ScVarInfo { ownerThread: __pa0, .. } = (iScVarInfos.clone().borrow()[(realScVarIdx.clone()-1) as usize].clone()) else { bail!("pattern mismatch") };
                threadOwner = __pa0.clone();
                threadText = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Th ")); __mm_s.push_str(&*intString(threadOwner.clone())); ArcStr::from(__mm_s) }).clone();
            }
        }
        if isValidVar.clone() {
            nodeLabel = GraphML::NodeLabel::NODELABEL_INTERNAL { text: (intString(varIdx.clone())).clone(), backgroundColor: None, fontStyle: openmodelica_susan::GraphML::FontStyle::FONTPLAIN };
            (tmpGraphInfo, _) = GraphML::addNode(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("var")); __mm_s.push_str(&*intString(varIdx.clone())); ArcStr::from(__mm_s) }).clone(), (arcstr::literal!(GraphML::COLOR_GREEN2)).clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![nodeLabel.clone()], openmodelica_susan::GraphML::ShapeType::ELLIPSE, Some((description.clone()).clone()), list![(iThreadIdAttributeIdx.clone(), threadText.clone())], iGraphIdx.clone(), tmpGraphInfo.clone())?;
        }
    }
    tmpGraphInfo = appendTaskVarEdgesToGraph(iTaskSolvedVarsMapping.clone(), iTaskUnsolvedVarsMapping.clone(), tmpGraphInfo.clone())?;
    oGraphInfo = tmpGraphInfo.clone();
    Ok(oGraphInfo)
}

fn appendTaskVarEdgesToGraph(mut iTaskSolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskUnsolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iGraphInfo: GraphML::GraphInfo) -> Result<GraphML::GraphInfo> {
    let mut oGraphInfo: GraphML::GraphInfo;
    let mut tmpGraphInfo: GraphML::GraphInfo = iGraphInfo.clone();
    let mut taskIdx: i32 = 0;
    let mut varIdx: i32 = 0;
    let mut taskVarList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let __range0 = 1..=(iTaskSolvedVarsMapping.clone().borrow().len() as i32);
    for mut taskIdx in __range0 {
        taskVarList = iTaskSolvedVarsMapping.clone().borrow()[(taskIdx.clone()-1) as usize].clone();
        for mut varIdx in &*taskVarList.clone() {
            let mut varIdx = varIdx.clone();
            (tmpGraphInfo, _) = GraphML::addEdge(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("varEdge_")); __mm_s.push_str(&*intString(taskIdx.clone())); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(varIdx.clone())); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("var")); __mm_s.push_str(&*intString(varIdx.clone())); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Node")); __mm_s.push_str(&*intString(taskIdx.clone())); ArcStr::from(__mm_s) }).clone(), (arcstr::literal!(GraphML::COLOR_BLACK)).clone(), openmodelica_susan::GraphML::LineType::LINE, GraphML::LINEWIDTH_STANDARD.clone(), false, metamodelica::nil(), (openmodelica_susan::GraphML::ArrowType::ARROWNONE, openmodelica_susan::GraphML::ArrowType::ARROWSTANDART), metamodelica::nil(), tmpGraphInfo.clone())?;
        }
    }
    let __range1 = 1..=(iTaskUnsolvedVarsMapping.clone().borrow().len() as i32);
    for mut taskIdx in __range1 {
        taskVarList = iTaskUnsolvedVarsMapping.clone().borrow()[(taskIdx.clone()-1) as usize].clone();
        for mut varIdx in &*taskVarList.clone() {
            let mut varIdx = varIdx.clone();
            (tmpGraphInfo, _) = GraphML::addEdge(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("varEdge_")); __mm_s.push_str(&*intString(taskIdx.clone())); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(varIdx.clone())); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Node")); __mm_s.push_str(&*intString(taskIdx.clone())); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("var")); __mm_s.push_str(&*intString(varIdx.clone())); ArcStr::from(__mm_s) }).clone(), (arcstr::literal!(GraphML::COLOR_BLACK)).clone(), openmodelica_susan::GraphML::LineType::LINE, GraphML::LINEWIDTH_STANDARD.clone(), false, metamodelica::nil(), (openmodelica_susan::GraphML::ArrowType::ARROWNONE, openmodelica_susan::GraphML::ArrowType::ARROWSTANDART), metamodelica::nil(), tmpGraphInfo.clone())?;
        }
    }
    oGraphInfo = tmpGraphInfo.clone();
    Ok(oGraphInfo)
}

fn appendUnmappedVariablesToGraph(mut iScVarCLMapping: metamodelica::Array<(i32, i32)>, mut iGraphInfo: GraphML::GraphInfo) -> Result<GraphML::GraphInfo> {
    let mut oGraphInfo: GraphML::GraphInfo;
    let mut tmpGraphInfo: GraphML::GraphInfo = iGraphInfo.clone();
    let mut scVarIdx: i32 = 0;
    let mut clIdx: i32 = 0;
    let __range0 = 1..=(iScVarCLMapping.clone().borrow().len() as i32);
    for mut scVarIdx in __range0 {
        (clIdx, _) = iScVarCLMapping.clone().borrow()[(scVarIdx.clone()-1) as usize].clone();
        if intLt(clIdx.clone(), 1) {
        }
    }
    oGraphInfo = tmpGraphInfo.clone();
    Ok(oGraphInfo)
}

fn appendCacheLineMapToGraph(mut iCacheLineMap: CacheLineMap, mut iCacheVariables: metamodelica::Array<SimCodeVar::SimVar>, mut iAddedVariables: metamodelica::Array<bool>, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>, mut iTopGraphAttThreadIdIdx: (i32, i32), mut iScVarTaskMapping: metamodelica::Array<i32>, mut iVarNameSCVarIdxMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr)), mut iScVarInfos: metamodelica::Array<ScVarInfo>, mut iGraphInfo: GraphML::GraphInfo) -> Result<GraphML::GraphInfo> {
    let mut oGraphInfo: GraphML::GraphInfo;
    let mut idx: i32 = 0;
    let mut graphIdx: i32 = 0;
    let mut iTopGraphIdx: i32 = 0;
    let mut iAttThreadIdIdx: i32 = 0;
    let mut entries: Arc<metamodelica::List<CacheLineEntry>> = metamodelica::nil();
    let mut entry: CacheLineEntry;
    let mut tmpGraphInfo: GraphML::GraphInfo;
    let mut entryThreadOwner: i32 = 0;
    let mut notOnlyParamters: bool = false;
    let CacheLineMap { entries: __pa0, idx: __pa1, .. } = (iCacheLineMap.clone()) else { bail!("pattern mismatch") };
    entries = __pa0.clone();
    idx = __pa1.clone();
    notOnlyParamters = false;
    for mut entry in &*entries.clone() {
        let mut entry = entry.clone();
        let CacheLineEntry { threadOwner: __pa2, .. } = (entry.clone()) else { bail!("pattern mismatch") };
        entryThreadOwner = __pa2.clone();
        notOnlyParamters = boolOr(notOnlyParamters.clone(), intNe(entryThreadOwner.clone(), -1));
    }
    if notOnlyParamters.clone() {
        (iTopGraphIdx, iAttThreadIdIdx) = iTopGraphAttThreadIdIdx.clone();
        let (__pa3, (_, _), (_, __pa4)) = GraphML::addGroupNode(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("CL_Meta_")); __mm_s.push_str(&*intString(idx.clone())); ArcStr::from(__mm_s) }).clone(), iTopGraphIdx.clone(), true, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("CL")); __mm_s.push_str(&*intString(idx.clone())); ArcStr::from(__mm_s) }).clone(), iGraphInfo.clone())?;
        tmpGraphInfo = __pa3.clone();
        graphIdx = __pa4.clone();
        oGraphInfo = List::fold(entries.clone(), Arc::new({ let __pe_b1 = iCacheVariables.clone(); let __pe_b2 = iAddedVariables.clone(); let __pe_b3 = iSchedulerInfo.clone(); let __pe_b4 = (graphIdx.clone(), iAttThreadIdIdx.clone()); let __pe_b5 = iScVarTaskMapping.clone(); let __pe_b6 = iVarNameSCVarIdxMapping.clone(); let __pe_b7 = iScVarInfos.clone(); move |__pe_a0, __pe_a8| appendCacheLineEntryToGraph(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone(), __pe_b7.clone(), __pe_a8) }), tmpGraphInfo.clone());
    } else {
        oGraphInfo = iGraphInfo.clone();
    }
    Ok(oGraphInfo)
}

fn appendCacheLineEntryToGraph(mut iCacheLineEntry: CacheLineEntry, mut iCacheVariables: metamodelica::Array<SimCodeVar::SimVar>, mut iAddedVariables: metamodelica::Array<bool>, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>, mut iTopGraphAttThreadIdIdx: (i32, i32), mut iScVarTaskMapping: metamodelica::Array<i32>, mut iVarNameSCVarIdxMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr)), mut iScVarInfos: metamodelica::Array<ScVarInfo>, mut iGraphInfo: GraphML::GraphInfo) -> Result<GraphML::GraphInfo> {
    let mut oGraphInfo: GraphML::GraphInfo;
    let mut realScVarIdxOffset: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut scVarIdx: i32 = 0;
    let mut realScVarIdx: i32 = 0;
    let mut realScVarOffset: i32 = 0;
    let mut taskIdx: i32 = 0;
    let mut iTopGraphIdx: i32 = 0;
    let mut iAttThreadIdIdx: i32 = 0;
    let mut threadOwner: i32 = 0;
    let mut varString: ArcStr = arcstr::literal!("");
    let mut threadText: ArcStr = arcstr::literal!("");
    let mut nodeLabelText: ArcStr = arcstr::literal!("");
    let mut nodeId: ArcStr = arcstr::literal!("");
    let mut nodeLabel: GraphML::NodeLabel;
    let mut iVar: SimCodeVar::SimVar;
    let mut name: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let CacheLineEntry { threadOwner: __pa0, scVarIdx: __pa1, .. } = (iCacheLineEntry.clone()) else { bail!("pattern mismatch") };
    threadOwner = __pa0.clone();
    scVarIdx = __pa1.clone();
    (iTopGraphIdx, iAttThreadIdIdx) = iTopGraphAttThreadIdIdx.clone();
    if intGe((iCacheVariables.clone().borrow().len() as i32) - scVarIdx.clone() + 1, 1) {
        iVar = iCacheVariables.clone().borrow()[((iCacheVariables.clone().borrow().len() as i32) - scVarIdx.clone() + 1-1) as usize].clone();
        let SimCodeVar::SIMVAR { name: __pa2, .. } = (iVar.clone()) else { bail!("pattern mismatch") };
        name = __pa2.clone();
        if BaseHashTable::hasKey(name.clone(), iVarNameSCVarIdxMapping.clone()) {
            realScVarIdxOffset = BaseHashTable::get(name.clone(), iVarNameSCVarIdxMapping.clone())?;
            realScVarIdx = (realScVarIdxOffset.clone()).get(1)?;
            realScVarOffset = (realScVarIdxOffset.clone()).get(2)?;
            realScVarIdx = realScVarIdx.clone() + realScVarOffset.clone();
            varString = (ComponentReferenceBasics::printComponentRefStr(name.clone())?).clone();
            taskIdx = iScVarTaskMapping.clone().borrow()[(realScVarIdx.clone()-1) as usize].clone();
            let ScVarInfo { ownerThread: __pa3, .. } = (iScVarInfos.clone().borrow()[(realScVarIdx.clone()-1) as usize].clone()) else { bail!("pattern mismatch") };
            threadOwner = __pa3.clone();
            nodeId = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("var")); __mm_s.push_str(&*intString(realScVarIdx.clone())); ArcStr::from(__mm_s) }).clone();
            {let _arr = iAddedVariables.clone(); _arr.borrow_mut()[(realScVarIdx.clone()-1) as usize] = true; _arr};
            threadText = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Th ")); __mm_s.push_str(&*intString(threadOwner.clone())); ArcStr::from(__mm_s) }).clone();
            nodeLabelText = (intString(realScVarIdx.clone())).clone();
            nodeLabel = GraphML::NodeLabel::NODELABEL_INTERNAL { text: (nodeLabelText.clone()).clone(), backgroundColor: None, fontStyle: openmodelica_susan::GraphML::FontStyle::FONTPLAIN };
            (oGraphInfo, _) = GraphML::addNode((nodeId.clone()).clone(), (arcstr::literal!(GraphML::COLOR_GREEN2)).clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![nodeLabel.clone()], openmodelica_susan::GraphML::ShapeType::ELLIPSE, Some((varString.clone()).clone()), list![(iAttThreadIdIdx.clone(), threadText.clone())], iTopGraphIdx.clone(), iGraphInfo.clone())?;
        } else {
            oGraphInfo = iGraphInfo.clone();
        }
    } else {
        oGraphInfo = iGraphInfo.clone();
    }
    Ok(oGraphInfo)
}

// -------------------------------------------
// PRINT
// -------------------------------------------
fn printCacheMap(mut iCacheMap: CacheMap) -> () {
    let mut cacheLineSize: i32 = 0;
    let mut cacheLinesFloat: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheLinesInt: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheLinesBool: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheLines: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheVariables: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let _ = (match iCacheMap.clone() {
        CacheMap::CACHEMAP { cacheLinesBool: mut cacheLinesBool, cacheLinesInt: mut cacheLinesInt, cacheLinesFloat: mut cacheLinesFloat, cacheVariables: mut cacheVariables, cacheLineSize: mut cacheLineSize } => {
            println!("{}", (literal!("\n\nCacheMap\n---------------\n")).clone());
            println!("{}", (literal!("  Variables\n")).clone());
            let _ = List::fold(cacheVariables.clone(), (std::sync::Arc::new(printCacheVariable) as std::sync::Arc<dyn ::std::ops::Fn(SimCodeVar::SimVar, i32) -> Result<i32> + 'static>), (cacheVariables.clone().len() as i32));
            println!("{}", (literal!("  Float Cache Lines\n")).clone());
            List::map1_0(cacheLinesFloat.clone(), (std::sync::Arc::new(printCacheLineMap) as std::sync::Arc<dyn ::std::ops::Fn(CacheLineMap, Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<()> + 'static>), cacheVariables.clone());
            println!("{}", (literal!("  Int Cache Lines\n")).clone());
            List::map1_0(cacheLinesInt.clone(), (std::sync::Arc::new(printCacheLineMap) as std::sync::Arc<dyn ::std::ops::Fn(CacheLineMap, Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<()> + 'static>), cacheVariables.clone());
            println!("{}", (literal!("  Bool Cache Lines\n")).clone());
            List::map1_0(cacheLinesBool.clone(), (std::sync::Arc::new(printCacheLineMap) as std::sync::Arc<dyn ::std::ops::Fn(CacheLineMap, Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<()> + 'static>), cacheVariables.clone());
            ()
        },
        CacheMap::UNIFORM_CACHEMAP { cacheLines: mut cacheLines, cacheVariables: mut cacheVariables, cacheLineSize: mut cacheLineSize } => {
            println!("{}", (literal!("\n\nUniform CacheMap\n---------------\n")).clone());
            println!("{}", (literal!("  Variables.\n")).clone());
            List::map1_0(cacheLines.clone(), (std::sync::Arc::new(printCacheLineMap) as std::sync::Arc<dyn ::std::ops::Fn(CacheLineMap, Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<()> + 'static>), cacheVariables.clone());
            ()
        },
        _ => {
            println!("{}", (literal!("printCacheMap: Unsupported cache map type!\n")).clone());
            ()
        },
    });
    ()
}

fn printCacheVariable(mut iCacheVariable: SimCodeVar::SimVar, mut iIdx: i32) -> Result<i32> {
    let mut oIdx: i32 = 0;
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("    ")); __mm_s.push_str(&*intString(iIdx.clone())); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*dumpSimCodeVar(iCacheVariable.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    oIdx = iIdx.clone() - 1;
    Ok(oIdx)
}

fn printCacheLineMap(mut iCacheLineMap: CacheLineMap, mut iCacheVariables: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<()> {
    let mut idx: i32 = 0;
    let mut entries: Arc<metamodelica::List<CacheLineEntry>> = metamodelica::nil();
    let mut iVarsString: ArcStr = arcstr::literal!("");
    let mut iBytesString: ArcStr = arcstr::literal!("");
    let CacheLineMap { entries: __pa0, idx: __pa1, .. } = (iCacheLineMap.clone()) else { bail!("pattern mismatch") };
    entries = __pa0.clone();
    idx = __pa1.clone();
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  CacheLineMap ")); __mm_s.push_str(&*intString(idx.clone())); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString((entries.clone().len() as i32))); __mm_s.push_str(&*literal!(" entries)\n")); ArcStr::from(__mm_s) }).clone());
    (iVarsString, iBytesString) = List::fold1(entries.clone(), (std::sync::Arc::new(cacheLineEntryToString) as std::sync::Arc<dyn ::std::ops::Fn(CacheLineEntry, Arc<metamodelica::List<SimCodeVar::SimVar>>, (ArcStr, ArcStr)) -> Result<(ArcStr, ArcStr)> + 'static>), iCacheVariables.clone(), (literal!(""), literal!("")));
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("    ")); __mm_s.push_str(&*iVarsString.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("    ")); __mm_s.push_str(&*iBytesString.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    println!("{}", (literal!("\n")).clone());
    Ok(())
}

fn printCacheLineMapClean(mut iCacheLineMap: CacheLineMap) -> Result<()> {
    let mut idx: i32 = 0;
    let mut entries: Arc<metamodelica::List<CacheLineEntry>> = metamodelica::nil();
    let mut iVarsString: ArcStr = arcstr::literal!("");
    let mut iBytesString: ArcStr = arcstr::literal!("");
    let CacheLineMap { entries: __pa0, idx: __pa1, .. } = (iCacheLineMap.clone()) else { bail!("pattern mismatch") };
    entries = __pa0.clone();
    idx = __pa1.clone();
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  CacheLineMap ")); __mm_s.push_str(&*intString(idx.clone())); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString((entries.clone().len() as i32))); __mm_s.push_str(&*literal!(" entries)\n")); ArcStr::from(__mm_s) }).clone());
    (iVarsString, iBytesString) = List::fold(entries.clone(), (std::sync::Arc::new(cacheLineEntryToStringClean) as std::sync::Arc<dyn ::std::ops::Fn(CacheLineEntry, (ArcStr, ArcStr)) -> Result<(ArcStr, ArcStr)> + 'static>), (literal!(""), literal!("")));
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("    ")); __mm_s.push_str(&*iVarsString.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("    ")); __mm_s.push_str(&*iBytesString.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    println!("{}", (literal!("\n")).clone());
    Ok(())
}

fn cacheLineEntryToString(mut iCacheLineEntry: CacheLineEntry, mut iCacheVariables: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut iString: (ArcStr, ArcStr)) -> Result<(ArcStr, ArcStr)> {
    let mut oString: (ArcStr, ArcStr);
    let mut start: i32 = 0;
    let mut dataType: i32 = 0;
    let mut size: i32 = 0;
    let mut scVarIdx: i32 = 0;
    let mut scVarStr: ArcStr = arcstr::literal!("");
    let mut iVar: SimCodeVar::SimVar;
    let mut iVarsString: ArcStr = arcstr::literal!("");
    let mut iBytesString: ArcStr = arcstr::literal!("");
    let mut iBytesStringNew: ArcStr = arcstr::literal!("");
    let mut byteStartString: ArcStr = arcstr::literal!("");
    (iVarsString, iBytesString) = iString.clone();
    let CacheLineEntry { scVarIdx: __pa0, size: __pa1, dataType: __pa2, start: __pa3, .. } = (iCacheLineEntry.clone()) else { bail!("pattern mismatch") };
    scVarIdx = __pa0.clone();
    size = __pa1.clone();
    dataType = __pa2.clone();
    start = __pa3.clone();
    iVar = (iCacheVariables.clone()).get((iCacheVariables.clone().len() as i32) - scVarIdx.clone() + 1)?;
    scVarStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*dumpSimCodeVar(iVar.clone())?); __mm_s.push_str(&*literal!(" [")); __mm_s.push_str(&*intString(scVarIdx.clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
    iVarsString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*iVarsString.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*scVarStr.clone()); ArcStr::from(__mm_s) }).clone();
    if intGt(start.clone(), 0) {
        iVarsString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*iVarsString.clone()); __mm_s.push_str(&*literal!(" | ")); ArcStr::from(__mm_s) }).clone();
        iBytesStringNew = (intString(start.clone())).clone();
    } else {
        iBytesStringNew = (literal!("")).clone();
    }
    iBytesStringNew = (Util::stringPadLeft((iBytesStringNew.clone()).clone(), 2 + ((scVarStr.clone()).clone().len() as i32) + ((iBytesStringNew.clone()).clone().len() as i32), (literal!(" ")).clone())).clone();
    iBytesString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*iBytesString.clone()); __mm_s.push_str(&*iBytesStringNew.clone()); ArcStr::from(__mm_s) }).clone();
    oString = (iVarsString.clone(), iBytesString.clone());
    Ok(oString)
}

fn cacheLineEntryToStringClean(mut iCacheLineEntry: CacheLineEntry, mut iString: (ArcStr, ArcStr)) -> Result<(ArcStr, ArcStr)> {
    let mut oString: (ArcStr, ArcStr);
    let mut start: i32 = 0;
    let mut dataType: i32 = 0;
    let mut size: i32 = 0;
    let mut scVarIdx: i32 = 0;
    let mut scVarStr: ArcStr = arcstr::literal!("");
    let mut iVarsString: ArcStr = arcstr::literal!("");
    let mut iBytesString: ArcStr = arcstr::literal!("");
    let mut iBytesStringNew: ArcStr = arcstr::literal!("");
    let mut byteStartString: ArcStr = arcstr::literal!("");
    (iVarsString, iBytesString) = iString.clone();
    let CacheLineEntry { scVarIdx: __pa0, size: __pa1, dataType: __pa2, start: __pa3, .. } = (iCacheLineEntry.clone()) else { bail!("pattern mismatch") };
    scVarIdx = __pa0.clone();
    size = __pa1.clone();
    dataType = __pa2.clone();
    start = __pa3.clone();
    scVarStr = (intString(scVarIdx.clone())).clone();
    iVarsString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*iVarsString.clone()); __mm_s.push_str(&*literal!("| ")); __mm_s.push_str(&*scVarStr.clone()); __mm_s.push_str(&*literal!(" ")); ArcStr::from(__mm_s) }).clone();
    iBytesStringNew = (intString(start.clone())).clone();
    iBytesStringNew = (Util::stringPadRight((iBytesStringNew.clone()).clone(), 3 + ((scVarStr.clone()).clone().len() as i32), (literal!(" ")).clone())).clone();
    iBytesString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*iBytesString.clone()); __mm_s.push_str(&*iBytesStringNew.clone()); ArcStr::from(__mm_s) }).clone();
    oString = (iVarsString.clone(), iBytesString.clone());
    Ok(oString)
}

fn dumpSimCodeVar(mut iVar: SimCodeVar::SimVar) -> Result<ArcStr> {
    let mut oString: ArcStr = arcstr::literal!("");
    let mut name: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let SimCodeVar::SIMVAR { name: __pa0, .. } = (iVar.clone()) else { bail!("pattern mismatch") };
    name = __pa0.clone();
    oString = (ComponentReferenceBasics::printComponentRefStr(name.clone())?).clone();
    Ok(oString)
}

fn printNodeSimCodeVarMapping(mut iMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> () {
    println!("{}", (literal!("Node - SimCodeVar - Mapping\n------------------\n")).clone());
    let _ = Array::fold(iMapping.clone(), (std::sync::Arc::new(fnptr!(printNodeSimCodeVarMapping0, Arc<metamodelica::List<i32>>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<i32> + 'static>), 1);
    println!("{}", (literal!("\n")).clone());
    ()
}

fn printNodeSimCodeVarMapping0(mut iMappingEntry: Arc<metamodelica::List<i32>>, mut iNodeIdx: i32) -> i32 {
    let mut oNodeIdx: i32 = 0;
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Node ")); __mm_s.push_str(&*intString(iNodeIdx.clone())); __mm_s.push_str(&*literal!(" uses sc-vars: ")); __mm_s.push_str(&*stringDelimitList(List::map(iMappingEntry.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    oNodeIdx = iNodeIdx.clone() + 1;
    oNodeIdx
}

fn printScVarTaskMapping(mut iMapping: metamodelica::Array<i32>) -> () {
    println!("{}", (literal!("----------------------\nSCVar - Task - Mapping\n----------------------\n")).clone());
    let _ = Array::fold(iMapping.clone(), (std::sync::Arc::new(fnptr!(printScVarTaskMapping0, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), 1);
    println!("{}", (literal!("\n")).clone());
    ()
}

fn printScVarTaskMapping0(mut iMappingEntry: i32, mut iScVarIdx: i32) -> i32 {
    let mut oScVarIdx: i32 = 0;
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SCVar ")); __mm_s.push_str(&*intString(iScVarIdx.clone())); __mm_s.push_str(&*literal!(" is solved in task: ")); __mm_s.push_str(&*intString(iMappingEntry.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    oScVarIdx = iScVarIdx.clone() + 1;
    oScVarIdx
}

fn printCacheLineTaskMapping(mut iCacheLineTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> () {
    let _ = Array::fold(iCacheLineTaskMapping.clone(), (std::sync::Arc::new(fnptr!(printCacheLineTaskMapping0, Arc<metamodelica::List<i32>>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<i32> + 'static>), 1);
    ()
}

fn printCacheLineTaskMapping0(mut iTasks: Arc<metamodelica::List<i32>>, mut iCacheLineIdx: i32) -> i32 {
    let mut oCacheLineIdx: i32 = 0;
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Tasks that are writing to cacheline ")); __mm_s.push_str(&*intString(iCacheLineIdx.clone())); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*stringDelimitList(List::map(iTasks.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    oCacheLineIdx = iCacheLineIdx.clone() + 1;
    oCacheLineIdx
}

fn printEqSimCodeVarMapping(mut iMapping: metamodelica::Array<metamodelica::Array<Arc<metamodelica::List<i32>>>>) -> Result<()> {
    let mut sysInformation: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut sysIdx: i32 = 0;
    let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let __range0 = 1..=(iMapping.clone().borrow().len() as i32);
    for mut sysIdx in __range0 {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("System ")); __mm_s.push_str(&*intString(sysIdx.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        sysInformation = iMapping.clone().borrow()[(sysIdx.clone()-1) as usize].clone();
        let __range1 = 1..=(sysInformation.clone().borrow().len() as i32);
        for mut eqIdx in __range1 {
            vars = sysInformation.clone().borrow()[(eqIdx.clone()-1) as usize].clone();
        }
    }
    Ok(())
}

fn printSccNodeMapping(mut iMapping: metamodelica::Array<i32>) -> () {
    println!("{}", (literal!("--------------------\nScc - Node - Mapping\n--------------------\n")).clone());
    let _ = Array::fold(iMapping.clone(), (std::sync::Arc::new(fnptr!(printSccNodeMapping0, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), 1);
    ()
}

fn printSccNodeMapping0(mut iMappingEntry: i32, mut iIdx: i32) -> i32 {
    let mut oIdx: i32 = 0;
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Scc ")); __mm_s.push_str(&*intString(iIdx.clone())); __mm_s.push_str(&*literal!(" is solved by node ")); __mm_s.push_str(&*intString(iMappingEntry.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    oIdx = iIdx.clone() + 1;
    oIdx
}

fn printScVarInfos(mut iScVarInfos: metamodelica::Array<ScVarInfo>) -> Result<()> {
    let mut scVarIdx: i32 = 0;
    let mut ownerThread: i32 = 0;
    let mut isShared: bool = false;
    println!("{}", (literal!("--------------------\nScVar - Infos\n--------------------\n")).clone());
    let __range0 = 1..=(iScVarInfos.clone().borrow().len() as i32);
    for mut scVarIdx in __range0 {
        let ScVarInfo { isShared: __pa1, ownerThread: __pa2 } = (iScVarInfos.clone().borrow()[(scVarIdx.clone()-1) as usize].clone()) else { bail!("pattern mismatch") };
        isShared = __pa1.clone();
        ownerThread = __pa2.clone();
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ScVar ")); __mm_s.push_str(&*intString(scVarIdx.clone())); __mm_s.push_str(&*literal!(" has thread owner ")); __mm_s.push_str(&*intString(ownerThread.clone())); __mm_s.push_str(&*literal!(" and shared state ")); __mm_s.push_str(&*boolString(isShared.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(())
}

fn dumpScVarsByIdx(mut iSimCodeVarIdx: i32, mut iAllSCVarsMapping: metamodelica::Array<Option<SimCodeVar::SimVar>>) -> Result<ArcStr> {
    let mut oString: ArcStr = arcstr::literal!("");
    let mut tmpString: ArcStr = arcstr::literal!("");
    let mut simVar: SimCodeVar::SimVar;
    oString = ('mc: {
        let __mc_input = (iSimCodeVarIdx.clone(), iAllSCVarsMapping.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut tmpString: ArcStr = tmpString.clone();
            let mut simVar: SimCodeVar::SimVar;
            let Some(__pa0) = (iAllSCVarsMapping.clone().borrow()[(iSimCodeVarIdx.clone()-1) as usize].clone()) else { bail!("pattern mismatch") };
            simVar = __pa0.clone();
            tmpString = (dumpSimCodeVar(simVar.clone())?).clone();
            Ok(tmpString.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("dumpScVarsByIdx: Failed to find simcode-variable with index ")); __mm_s.push_str(&*intString(iSimCodeVarIdx.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            Ok(literal!("NONE"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(oString)
}

fn printSimCodeVarTypes(mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>) -> Result<()> {
    let mut varIdx: i32 = 0;
    let mut varDataType: i32 = 0;
    let mut varSize: i32 = 0;
    let mut varType: i32 = 0;
    let __range0 = 1..=(iSimCodeVarTypes.clone().borrow().len() as i32);
    for mut varIdx in __range0 {
        (varDataType, varSize, varType) = iSimCodeVarTypes.clone().borrow()[(varIdx.clone()-1) as usize].clone();
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable ")); __mm_s.push_str(&*intString(varIdx.clone())); __mm_s.push_str(&*literal!(" has data type ")); __mm_s.push_str(&*intString(varDataType.clone())); __mm_s.push_str(&*literal!(" and size ")); __mm_s.push_str(&*intString(varSize.clone())); __mm_s.push_str(&*literal!(" and type ")); __mm_s.push_str(&*intString(varType.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(())
}

// -------------------------------------------
// SUSAN
// -------------------------------------------
pub fn getSubscriptListOfArrayCref(mut iCref: Arc<DAE::ComponentRef>, mut iNumArrayElems: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>>> {
    let mut oSubscriptList: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>> = metamodelica::nil();
    let mut tmpCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    tmpCrefs = expandCref(iCref.clone(), iNumArrayElems.clone())?;
    oSubscriptList = List::map(tmpCrefs.clone(), (std::sync::Arc::new(ComponentReference::crefLastSubs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<metamodelica::List<Arc<DAE::Subscript>>>> + 'static>));
    Ok(oSubscriptList)
}

pub fn expandCref(mut iCref: Arc<DAE::ComponentRef>, mut iNumArrayElems: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut oCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut elems: i32 = 0;
    let mut dims: i32 = 0;
    let mut dimElemCount: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    cref = removeSubscripts(iCref.clone());
    dims = getCrefDims(iCref.clone());
    dimElemCount = getDimElemCount(iNumArrayElems.clone().reverse(), dims.clone());
    elems = List::reduce(dimElemCount.clone(), (std::sync::Arc::new(fnptr!(intMul, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>))?;
    dims = (iNumArrayElems.clone().len() as i32);
    oCrefs = expandCref1(cref.clone(), elems.clone(), dimElemCount.clone())?;
    Ok(oCrefs)
}

pub fn expandCrefWithDims(mut iCref: Arc<DAE::ComponentRef>, mut iDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut oCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut elems: i32 = 0;
    let mut dims: i32 = 0;
    let mut dimElemCount: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut dim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    let mut numArrayElems: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    numArrayElems = metamodelica::nil();
    for mut dim in &*iDims.clone() {
        let mut dim = dim.clone();
        numArrayElems = cons((getDimStringOfDimElement(dim.clone())).clone(), numArrayElems.clone());
    }
    oCrefs = expandCref(iCref.clone(), numArrayElems.clone())?;
    Ok(oCrefs)
}

fn getDimStringOfDimElement(mut iDim: Arc<DAE::Dimension>) -> ArcStr {
    let mut oDimString: ArcStr = arcstr::literal!("");
    let mut integer: i32 = 0;
    oDimString = ((::match_deref::match_deref! { match &(iDim.clone()) {
        Deref @ DAE::Dimension::DIM_INTEGER { integer } => intString(integer.clone()),
        _ => {
            println!("{}", (literal!("getDimStringOfDimElement: unsupported Dimension-type given!\n")).clone());
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    oDimString
}

fn removeSubscripts(mut iCref: Arc<DAE::ComponentRef>) -> Arc<DAE::ComponentRef> {
    let mut oCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut ident: ArcStr = arcstr::literal!("");
    let mut identType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut subscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
    let mut componentRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut index: i32 = 0;
    oCref = (::match_deref::match_deref! { match &(iCref.clone()) {
        Deref @ DAE::ComponentRef::CREF_QUAL { ident, identType, subscriptLst, componentRef } => {
            let mut componentRef = (*componentRef).clone();
            componentRef = removeSubscripts(componentRef.clone());
            Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (ident.clone()).clone(), identType: identType.clone(), subscriptLst: subscriptLst.clone(), componentRef: componentRef.clone() })
        },
        Deref @ DAE::ComponentRef::CREF_IDENT { ident, identType, subscriptLst } => Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (ident.clone()).clone(), identType: identType.clone(), subscriptLst: metamodelica::nil() }),
        _ => iCref.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oCref
}

fn getDimElemCount(mut iNumArrayElems: Arc<metamodelica::List<ArcStr>>, mut iDims: i32) -> Arc<metamodelica::List<i32>> {
    let mut oNumArrayElems: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut dimList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut intNumArrayElems: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut dims: i32 = 0;
    dims = if (intLe(iDims.clone(), 0)) {(iNumArrayElems.clone().len() as i32)} else {iDims.clone()};
    dimList = List::intRange(dims.clone());
    intNumArrayElems = List::map(iNumArrayElems.clone(), (std::sync::Arc::new(stringInt) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>));
    oNumArrayElems = List::map1(dimList.clone(), std::sync::Arc::new(fnptr!(List::getIndexFirst, i32, _)), intNumArrayElems.clone());
    oNumArrayElems
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getCrefDims(mut iCref: Arc<DAE::ComponentRef>) -> i32 {
    let mut oDims: i32 = 0;
    let mut componentRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut subscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
    let mut tmpDims: i32 = 0;
    oDims = (::match_deref::match_deref! { match &(iCref.clone()) {
        Deref @ DAE::ComponentRef::CREF_QUAL { componentRef, .. } => getCrefDims(componentRef.clone()),
        Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst, .. } => {
            tmpDims = (subscriptLst.clone().len() as i32);
            tmpDims.clone()
        },
        _ => {
            println!("{}", (literal!("HpcOmMemory.getCrefDims failed!\n")).clone());
            0
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oDims
}

fn expandCref1(mut iCref: Arc<DAE::ComponentRef>, mut iElems: i32, mut iDimElemCount: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut oCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut tmpCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut idxList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    oCrefs = 'mc: {
        let __mc_input = (iCref.clone(), iElems.clone(), iDimElemCount.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut tmpCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = tmpCrefs.clone();
                    tmpCrefs = ComponentReference::expandCref(iCref.clone(), false)?;
                    let true = (intEq((tmpCrefs.clone().len() as i32), iElems.clone())) else { bail!("pattern mismatch") };
                    Ok(tmpCrefs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut idxList: Arc<metamodelica::List<i32>> = idxList.clone();
                    let mut tmpCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = tmpCrefs.clone();
                    idxList = List::intRange(List::reduce(iDimElemCount.clone(), (std::sync::Arc::new(fnptr!(intMul, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>))?);
                    tmpCrefs = List::map2(idxList.clone(), (std::sync::Arc::new(createArrayIndexCref) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<i32>>, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> + 'static>), iDimElemCount.clone(), iCref.clone());
                    Ok(tmpCrefs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oCrefs)
}

fn createArrayIndexCref(mut iIdx: i32, mut iDimElemCount: Arc<metamodelica::List<i32>>, mut iCref: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut oCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    (oCref, _) = createArrayIndexCref_impl(iIdx.clone(), iDimElemCount.clone(), (iCref.clone(), 1))?;
    Ok(oCref)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn createArrayIndexCref_impl(mut iIdx: i32, mut iDimElemCount: Arc<metamodelica::List<i32>>, mut iRefCurrentDim: (Arc<DAE::ComponentRef>, i32)) -> Result<(Arc<DAE::ComponentRef>, i32)> {
    let mut oRefCurrentDim: (Arc<DAE::ComponentRef>, i32);
    let mut ident: ArcStr = arcstr::literal!("");
    let mut identType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut subscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
    let mut componentRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut currentDim: i32 = 0;
    let mut idxValue: i32 = 0;
    let mut dimElemsPre: i32 = 0;
    let mut dimElems: i32 = 0;
    oRefCurrentDim = 'mc: {
        let __mc_input = (iIdx.clone(), iDimElemCount.clone(), iRefCurrentDim.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, (Deref @ DAE::ComponentRef::CREF_QUAL { ident, identType, subscriptLst, componentRef }, 1)) => {
                    let mut componentRef = (*componentRef).clone();
                    let true = (intLe(1, (iDimElemCount.clone().len() as i32))) else { bail!("pattern mismatch") };
                    (componentRef, _) = createArrayIndexCref_impl(iIdx.clone(), iDimElemCount.clone(), (componentRef.clone(), 1))?;
                    Ok((Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (ident.clone()).clone(), identType: identType.clone(), subscriptLst: subscriptLst.clone(), componentRef: componentRef.clone() }), 2))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, (Deref @ DAE::ComponentRef::CREF_QUAL { ident, identType, subscriptLst, componentRef }, currentDim)) => {
                    let mut componentRef = (*componentRef).clone();
                    let true = (intLe(currentDim.clone(), (iDimElemCount.clone().len() as i32))) else { bail!("pattern mismatch") };
                    (componentRef, _) = createArrayIndexCref_impl(iIdx.clone(), iDimElemCount.clone(), (componentRef.clone(), currentDim.clone()))?;
                    Ok((Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (ident.clone()).clone(), identType: identType.clone(), subscriptLst: subscriptLst.clone(), componentRef: componentRef.clone() }), currentDim.clone() + 1))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, (Deref @ DAE::ComponentRef::CREF_IDENT { ident, identType, subscriptLst }, 1)) => {
                    let mut subscriptLst = (*subscriptLst).clone();
                    let mut idxValue: i32 = idxValue.clone();
                    let true = (intLe(1, (iDimElemCount.clone().len() as i32))) else { bail!("pattern mismatch") };
                    idxValue = intMod(iIdx.clone() - 1, listHead(iDimElemCount.clone())?) + 1;
                    subscriptLst = cons(Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: idxValue.clone() }) }), subscriptLst.clone());
                    Ok(createArrayIndexCref_impl(iIdx.clone(), iDimElemCount.clone(), (Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (ident.clone()).clone(), identType: identType.clone(), subscriptLst: subscriptLst.clone() }), 2))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, (Deref @ DAE::ComponentRef::CREF_IDENT { ident, identType, subscriptLst }, currentDim)) => {
                    let mut subscriptLst = (*subscriptLst).clone();
                    let mut dimElems: i32 = dimElems.clone();
                    let mut dimElemsPre: i32 = dimElemsPre.clone();
                    let mut idxValue: i32 = idxValue.clone();
                    let true = (intLe(currentDim.clone(), (iDimElemCount.clone().len() as i32))) else { bail!("pattern mismatch") };
                    dimElemsPre = List::reduce(List::sublist(iDimElemCount.clone(), 1, (iDimElemCount.clone().len() as i32) - currentDim.clone() + 1)?, (std::sync::Arc::new(fnptr!(intMul, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>))?;
                    dimElems = (iDimElemCount.clone()).get(currentDim.clone())?;
                    idxValue = intMod(intDiv(iIdx.clone() - 1, dimElemsPre.clone()), dimElems.clone()) + 1;
                    subscriptLst = cons(Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: idxValue.clone() }) }), subscriptLst.clone());
                    Ok(createArrayIndexCref_impl(iIdx.clone(), iDimElemCount.clone(), (Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (ident.clone()).clone(), identType: identType.clone(), subscriptLst: subscriptLst.clone() }), currentDim.clone() + 1))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, (Deref @ DAE::ComponentRef::CREF_IDENT { ident, identType, subscriptLst }, currentDim)) => {
                    let false = (intLe(currentDim.clone(), (iDimElemCount.clone().len() as i32))) else { bail!("pattern mismatch") };
                    Ok(iRefCurrentDim.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("createArrayIndexCref_impl failed!\n")).clone());
                    Ok(iRefCurrentDim.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oRefCurrentDim)
}

// -------------------------------------------
// UTIL
// -------------------------------------------
fn getTaskListTasks(mut iTaskList: HpcOmSimCode::TaskList) -> Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> {
    let mut oTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut tasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    oTasks = (match iTaskList.clone() {
        HpcOmSimCode::TaskList::PARALLELTASKLIST { tasks: mut tasks } => tasks.clone(),
        HpcOmSimCode::TaskList::PARALLELTASKLIST { tasks: mut tasks } => tasks.clone(),
        _ => {
            println!("{}", (literal!("getTaskListTasks failed!\n")).clone());
            metamodelica::nil()
        },
    });
    oTasks
}

fn getCacheLineMapOfPartlyFilledCacheLine(mut iPartlyFilledCacheLine: PartlyFilledCacheLine) -> Result<CacheLineMap> {
    let mut oCacheLineMap: CacheLineMap;
    let mut cacheLineMap: CacheLineMap;
    oCacheLineMap = (match iPartlyFilledCacheLine.clone() {
        PartlyFilledCacheLine::PARTLYFILLEDCACHELINE_LEVEL { cacheLineMap: mut cacheLineMap, .. } => cacheLineMap.clone(),
        PartlyFilledCacheLine::PARTLYFILLEDCACHELINE_THREAD { cacheLineMap: mut cacheLineMap } => cacheLineMap.clone(),
        _ => bail!("match: no arm matched"),
    });
    Ok(oCacheLineMap)
}

fn getAllCacheLinesOfCacheMap(mut iCacheMap: CacheMap) -> Result<Arc<metamodelica::List<CacheLineMap>>> {
    let mut oCacheLines: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheLinesFloat: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheLinesInt: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheLinesBool: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut allCacheLines: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    oCacheLines = (match iCacheMap.clone() {
        CacheMap::CACHEMAP { cacheLinesBool: mut cacheLinesBool, cacheLinesInt: mut cacheLinesInt, cacheLinesFloat: mut cacheLinesFloat, .. } => {
            allCacheLines = listAppend(cacheLinesFloat.clone(), listAppend(cacheLinesInt.clone(), cacheLinesBool.clone()));
            allCacheLines.clone()
        },
        CacheMap::UNIFORM_CACHEMAP { cacheLines: ref allCacheLines, .. } => allCacheLines.clone(),
        _ => bail!("match: no arm matched"),
    });
    Ok(oCacheLines)
}

fn getCacheVariablesOfCacheMap(mut iCacheMap: CacheMap) -> Result<Arc<metamodelica::List<SimCodeVar::SimVar>>> {
    let mut oCacheVariables: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut cacheVariables: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    oCacheVariables = (match iCacheMap.clone() {
        CacheMap::CACHEMAP { cacheVariables: mut cacheVariables, .. } => cacheVariables.clone(),
        CacheMap::UNIFORM_CACHEMAP { cacheVariables: mut cacheVariables, .. } => cacheVariables.clone(),
        _ => bail!("match: no arm matched"),
    });
    Ok(oCacheVariables)
}

fn getCacheLineSizeOfCacheMap(mut iCacheMap: CacheMap) -> Result<i32> {
    let mut oCacheLineSize: i32 = 0;
    let mut cacheLineSize: i32 = 0;
    oCacheLineSize = (match iCacheMap.clone() {
        CacheMap::CACHEMAP { cacheLineSize: mut cacheLineSize, .. } => cacheLineSize.clone(),
        CacheMap::UNIFORM_CACHEMAP { cacheLineSize: mut cacheLineSize, .. } => cacheLineSize.clone(),
        _ => bail!("match: no arm matched"),
    });
    Ok(oCacheLineSize)
}

