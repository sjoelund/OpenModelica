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

use crate::AvlTreeCRToInt;
use crate::BackendDAE;
use crate::HashTableCrefSimVar;
use crate::HpcOmSimCode;
use crate::SimCodeFunction;
use crate::SimCodeVar;
use openmodelica_ast::Absyn;
use openmodelica_frontend::HashTable;
use openmodelica_frontend::HashTableCrIListArray;
use openmodelica_frontend::HashTableCrILst;
use openmodelica_frontend_types::DAE;

// public imports
pub type ExtConstructor = (Arc<DAE::ComponentRef>, ArcStr, Arc<metamodelica::List<Arc<DAE::Exp>>>);

pub type ExtDestructor = (ArcStr, Arc<DAE::ComponentRef>);

pub type ExtAlias = (Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>);

pub type SparsityPattern = Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>;

pub type NonlinearPattern = Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>;

// same structure but different name for the sake of maintenance
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct JacobianColumn {
    pub columnEqns: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub columnVars: Arc<metamodelica::List<SimCodeVar::SimVar>>,
    pub numberOfResultVars: i32,
    pub constantEqns: Arc<metamodelica::List<Arc<SimEqSystem>>>,
}

pub type JAC_COLUMN = JacobianColumn;


#[derive(Clone)]
pub struct JacobianMatrix {
    pub columns: Arc<metamodelica::List<Arc<JacobianColumn>>>,
    pub seedVars: Arc<metamodelica::List<SimCodeVar::SimVar>>,
    pub matrixName: ArcStr,
    pub sparsity: SparsityPattern,
    pub sparsityT: SparsityPattern,
    pub nonlinear: NonlinearPattern,
    pub nonlinearT: NonlinearPattern,
    pub coloredCols: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>,
    pub coloredRows: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>,
    pub maxColorCols: i32,
    pub jacobianIndex: i32,
    pub partitionIndex: i32,
    pub generic_loop_calls: Arc<metamodelica::List<SimGenericCall>>,
    pub crefsHT: Option<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, SimCodeVar::SimVar)>>), i32, (HashTableCrefSimVar::FuncHashCref, HashTableCrefSimVar::FuncCrefEqual, HashTableCrefSimVar::FuncCrefStr, HashTableCrefSimVar::FuncExpStr))>,
    pub isAdjoint: bool,
}

impl PartialEq for JacobianMatrix {
    fn eq(&self, other: &Self) -> bool {
        self.columns == other.columns && self.seedVars == other.seedVars && self.matrixName == other.matrixName && self.sparsity == other.sparsity && self.sparsityT == other.sparsityT && self.nonlinear == other.nonlinear && self.nonlinearT == other.nonlinearT && self.coloredCols == other.coloredCols && self.coloredRows == other.coloredRows && self.maxColorCols == other.maxColorCols && self.jacobianIndex == other.jacobianIndex && self.partitionIndex == other.partitionIndex && self.generic_loop_calls == other.generic_loop_calls && std::sync::Arc::ptr_eq(&self.crefsHT, &other.crefsHT) && self.isAdjoint == other.isAdjoint
    }
}
impl Eq for JacobianMatrix {}
impl PartialOrd for JacobianMatrix {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for JacobianMatrix {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.columns.cmp(&other.columns).then_with(|| self.seedVars.cmp(&other.seedVars).then_with(|| self.matrixName.cmp(&other.matrixName).then_with(|| self.sparsity.cmp(&other.sparsity).then_with(|| self.sparsityT.cmp(&other.sparsityT).then_with(|| self.nonlinear.cmp(&other.nonlinear).then_with(|| self.nonlinearT.cmp(&other.nonlinearT).then_with(|| self.coloredCols.cmp(&other.coloredCols).then_with(|| self.coloredRows.cmp(&other.coloredRows).then_with(|| self.maxColorCols.cmp(&other.maxColorCols).then_with(|| self.jacobianIndex.cmp(&other.jacobianIndex).then_with(|| self.partitionIndex.cmp(&other.partitionIndex).then_with(|| self.generic_loop_calls.cmp(&other.generic_loop_calls).then_with(|| (std::sync::Arc::as_ptr(&self.crefsHT) as *const ()).cmp(&(std::sync::Arc::as_ptr(&other.crefsHT) as *const ())).then_with(|| self.isAdjoint.cmp(&other.isAdjoint)))))))))))))))
    }
}
impl std::fmt::Debug for JacobianMatrix {
    fn fmt(&self, __f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut __ds = __f.debug_struct("JacobianMatrix");
        __ds.field("columns", &self.columns);
        __ds.field("seedVars", &self.seedVars);
        __ds.field("matrixName", &self.matrixName);
        __ds.field("sparsity", &self.sparsity);
        __ds.field("sparsityT", &self.sparsityT);
        __ds.field("nonlinear", &self.nonlinear);
        __ds.field("nonlinearT", &self.nonlinearT);
        __ds.field("coloredCols", &self.coloredCols);
        __ds.field("coloredRows", &self.coloredRows);
        __ds.field("maxColorCols", &self.maxColorCols);
        __ds.field("jacobianIndex", &self.jacobianIndex);
        __ds.field("partitionIndex", &self.partitionIndex);
        __ds.field("generic_loop_calls", &self.generic_loop_calls);
        __ds.field("crefsHT", &format_args!("<fn@{:p}>", std::sync::Arc::as_ptr(&self.crefsHT)));
        __ds.field("isAdjoint", &self.isAdjoint);
        __ds.finish()
    }
}

pub type JAC_MATRIX = JacobianMatrix;


thread_local! { static __emptyJacobian_TLS: Arc<JacobianMatrix> = Arc::new(JacobianMatrix { columns: metamodelica::nil(), seedVars: metamodelica::nil(), matrixName: (literal!("")).clone(), sparsity: metamodelica::nil(), sparsityT: metamodelica::nil(), nonlinear: metamodelica::nil(), nonlinearT: metamodelica::nil(), coloredCols: metamodelica::nil(), coloredRows: metamodelica::nil(), maxColorCols: 0, jacobianIndex: -1, partitionIndex: 0, generic_loop_calls: metamodelica::nil(), crefsHT: None, isAdjoint: false }); }
pub fn emptyJacobian() -> Arc<JacobianMatrix> { __emptyJacobian_TLS.with(|__t| __t.clone()) }

pub static emptyPartitionData: std::sync::LazyLock<PartitionData> = std::sync::LazyLock::new(|| { PartitionData { numPartitions: -1, partitions: metamodelica::nil(), activatorsForPartitions: metamodelica::nil(), stateToActivators: metamodelica::nil() } });

/// Root data structure containing information required for templates to
///  generate simulation code for a Modelica model.
#[derive(Clone)]
pub struct SimCode {
    pub modelInfo: ModelInfo,
    /// shared literals
    pub literals: Arc<metamodelica::List<Arc<DAE::Exp>>>,
    pub recordDecls: Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>>,
    pub externalFunctionIncludes: Arc<metamodelica::List<ArcStr>>,
    pub generic_loop_calls: Arc<metamodelica::List<SimGenericCall>>,
    /// state and input dependent variables, that are not inserted into any partion
    pub localKnownVars: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub allEquations: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub odeEquations: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimEqSystem>>>>>,
    pub algebraicEquations: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimEqSystem>>>>>,
    pub clockedPartitions: Arc<metamodelica::List<ClockedPartition>>,
    pub initialEquations: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub initialEquations_lambda0: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub removedInitialEquations: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub startValueEquations: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub nominalValueEquations: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub minValueEquations: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub maxValueEquations: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub parameterEquations: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub removedEquations: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub algorithmAndEquationAsserts: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub equationsForZeroCrossings: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub jacobianEquations: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub stateSets: Arc<metamodelica::List<StateSet>>,
    pub constraints: Arc<metamodelica::List<Arc<DAE::Constraint>>>,
    pub classAttributes: Arc<metamodelica::List<Arc<DAE::ClassAttributes>>>,
    pub zeroCrossings: Arc<metamodelica::List<BackendDAE::ZeroCrossing>>,
    /// only used by c runtime
    pub relations: Arc<metamodelica::List<BackendDAE::ZeroCrossing>>,
    /// only used by c runtime yet
    pub timeEvents: Arc<metamodelica::List<BackendDAE::TimeEvent>>,
    pub discreteModelVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>,
    pub extObjInfo: ExtObjInfo,
    pub makefileParams: SimCodeFunction::MakefileParams,
    pub delayedExps: DelayedExpression,
    pub spatialInfo: SpatialDistributionInfo,
    pub jacobianMatrices: Arc<metamodelica::List<Arc<JacobianMatrix>>>,
    pub simulationSettingsOpt: Option<SimulationSettings>,
    /// Prefix for all enerated C files. Usually the model name with dots replaced by underscores.
    pub fileNamePrefix: ArcStr,
    /// Used in FMI where files are generated in a special directory
    pub fullPathPrefix: ArcStr,
    /// Name of FMU file <fmuTargetName>.fmu
    pub fmuTargetName: ArcStr,
    pub hpcomData: HpcOmSimCode::HpcOmData,
    /// Used in FMI
    pub valueReferences: Arc<AvlTreeCRToInt::Tree>,
    pub varToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (HashTableCrIListArray::FuncHashCref, HashTableCrIListArray::FuncCrefEqual, HashTableCrIListArray::FuncCrefStr, HashTableCrIListArray::FuncExpStr)),
    pub varToIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr)),
    /// hidden from typeview - used by cref2simvar() for cref -> SIMVAR lookup available in templates.
    pub crefToSimVarHT: HashTableCrefToSimVar,
    /// map variables to clock indices
    pub crefToClockIndexHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr)),
    pub backendMapping: Option<BackendMapping>,
    pub modelStructure: Option<FmiModelStructure>,
    pub fmiSimulationFlags: Option<FmiSimulationFlags>,
    pub partitionData: PartitionData,
    pub daeModeData: Option<DaeModeData>,
    pub inlineEquations: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    /// used for OMSI to generate equations code
    pub omsiData: Option<OMSIData>,
    pub scalarized: bool,
}

impl PartialEq for SimCode {
    fn eq(&self, other: &Self) -> bool {
        self.modelInfo == other.modelInfo && self.literals == other.literals && self.recordDecls == other.recordDecls && self.externalFunctionIncludes == other.externalFunctionIncludes && self.generic_loop_calls == other.generic_loop_calls && self.localKnownVars == other.localKnownVars && self.allEquations == other.allEquations && self.odeEquations == other.odeEquations && self.algebraicEquations == other.algebraicEquations && self.clockedPartitions == other.clockedPartitions && self.initialEquations == other.initialEquations && self.initialEquations_lambda0 == other.initialEquations_lambda0 && self.removedInitialEquations == other.removedInitialEquations && self.startValueEquations == other.startValueEquations && self.nominalValueEquations == other.nominalValueEquations && self.minValueEquations == other.minValueEquations && self.maxValueEquations == other.maxValueEquations && self.parameterEquations == other.parameterEquations && self.removedEquations == other.removedEquations && self.algorithmAndEquationAsserts == other.algorithmAndEquationAsserts && self.equationsForZeroCrossings == other.equationsForZeroCrossings && self.jacobianEquations == other.jacobianEquations && self.stateSets == other.stateSets && self.constraints == other.constraints && self.classAttributes == other.classAttributes && self.zeroCrossings == other.zeroCrossings && self.relations == other.relations && self.timeEvents == other.timeEvents && self.discreteModelVars == other.discreteModelVars && self.extObjInfo == other.extObjInfo && self.makefileParams == other.makefileParams && self.delayedExps == other.delayedExps && self.spatialInfo == other.spatialInfo && self.jacobianMatrices == other.jacobianMatrices && self.simulationSettingsOpt == other.simulationSettingsOpt && self.fileNamePrefix == other.fileNamePrefix && self.fullPathPrefix == other.fullPathPrefix && self.fmuTargetName == other.fmuTargetName && self.hpcomData == other.hpcomData && self.valueReferences == other.valueReferences && std::sync::Arc::ptr_eq(&self.varToArrayIndexMapping, &other.varToArrayIndexMapping) && std::sync::Arc::ptr_eq(&self.varToIndexMapping, &other.varToIndexMapping) && std::sync::Arc::ptr_eq(&self.crefToSimVarHT, &other.crefToSimVarHT) && std::sync::Arc::ptr_eq(&self.crefToClockIndexHT, &other.crefToClockIndexHT) && self.backendMapping == other.backendMapping && self.modelStructure == other.modelStructure && self.fmiSimulationFlags == other.fmiSimulationFlags && self.partitionData == other.partitionData && self.daeModeData == other.daeModeData && self.inlineEquations == other.inlineEquations && self.omsiData == other.omsiData && self.scalarized == other.scalarized
    }
}
impl Eq for SimCode {}
impl PartialOrd for SimCode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for SimCode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.modelInfo.cmp(&other.modelInfo).then_with(|| self.literals.cmp(&other.literals).then_with(|| self.recordDecls.cmp(&other.recordDecls).then_with(|| self.externalFunctionIncludes.cmp(&other.externalFunctionIncludes).then_with(|| self.generic_loop_calls.cmp(&other.generic_loop_calls).then_with(|| self.localKnownVars.cmp(&other.localKnownVars).then_with(|| self.allEquations.cmp(&other.allEquations).then_with(|| self.odeEquations.cmp(&other.odeEquations).then_with(|| self.algebraicEquations.cmp(&other.algebraicEquations).then_with(|| self.clockedPartitions.cmp(&other.clockedPartitions).then_with(|| self.initialEquations.cmp(&other.initialEquations).then_with(|| self.initialEquations_lambda0.cmp(&other.initialEquations_lambda0).then_with(|| self.removedInitialEquations.cmp(&other.removedInitialEquations).then_with(|| self.startValueEquations.cmp(&other.startValueEquations).then_with(|| self.nominalValueEquations.cmp(&other.nominalValueEquations).then_with(|| self.minValueEquations.cmp(&other.minValueEquations).then_with(|| self.maxValueEquations.cmp(&other.maxValueEquations).then_with(|| self.parameterEquations.cmp(&other.parameterEquations).then_with(|| self.removedEquations.cmp(&other.removedEquations).then_with(|| self.algorithmAndEquationAsserts.cmp(&other.algorithmAndEquationAsserts).then_with(|| self.equationsForZeroCrossings.cmp(&other.equationsForZeroCrossings).then_with(|| self.jacobianEquations.cmp(&other.jacobianEquations).then_with(|| self.stateSets.cmp(&other.stateSets).then_with(|| self.constraints.cmp(&other.constraints).then_with(|| self.classAttributes.cmp(&other.classAttributes).then_with(|| self.zeroCrossings.cmp(&other.zeroCrossings).then_with(|| self.relations.cmp(&other.relations).then_with(|| self.timeEvents.cmp(&other.timeEvents).then_with(|| self.discreteModelVars.cmp(&other.discreteModelVars).then_with(|| self.extObjInfo.cmp(&other.extObjInfo).then_with(|| self.makefileParams.cmp(&other.makefileParams).then_with(|| self.delayedExps.cmp(&other.delayedExps).then_with(|| self.spatialInfo.cmp(&other.spatialInfo).then_with(|| self.jacobianMatrices.cmp(&other.jacobianMatrices).then_with(|| self.simulationSettingsOpt.cmp(&other.simulationSettingsOpt).then_with(|| self.fileNamePrefix.cmp(&other.fileNamePrefix).then_with(|| self.fullPathPrefix.cmp(&other.fullPathPrefix).then_with(|| self.fmuTargetName.cmp(&other.fmuTargetName).then_with(|| self.hpcomData.cmp(&other.hpcomData).then_with(|| self.valueReferences.cmp(&other.valueReferences).then_with(|| (std::sync::Arc::as_ptr(&self.varToArrayIndexMapping) as *const ()).cmp(&(std::sync::Arc::as_ptr(&other.varToArrayIndexMapping) as *const ())).then_with(|| (std::sync::Arc::as_ptr(&self.varToIndexMapping) as *const ()).cmp(&(std::sync::Arc::as_ptr(&other.varToIndexMapping) as *const ())).then_with(|| (std::sync::Arc::as_ptr(&self.crefToSimVarHT) as *const ()).cmp(&(std::sync::Arc::as_ptr(&other.crefToSimVarHT) as *const ())).then_with(|| (std::sync::Arc::as_ptr(&self.crefToClockIndexHT) as *const ()).cmp(&(std::sync::Arc::as_ptr(&other.crefToClockIndexHT) as *const ())).then_with(|| self.backendMapping.cmp(&other.backendMapping).then_with(|| self.modelStructure.cmp(&other.modelStructure).then_with(|| self.fmiSimulationFlags.cmp(&other.fmiSimulationFlags).then_with(|| self.partitionData.cmp(&other.partitionData).then_with(|| self.daeModeData.cmp(&other.daeModeData).then_with(|| self.inlineEquations.cmp(&other.inlineEquations).then_with(|| self.omsiData.cmp(&other.omsiData).then_with(|| self.scalarized.cmp(&other.scalarized))))))))))))))))))))))))))))))))))))))))))))))))))))
    }
}
impl std::fmt::Debug for SimCode {
    fn fmt(&self, __f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut __ds = __f.debug_struct("SimCode");
        __ds.field("modelInfo", &self.modelInfo);
        __ds.field("literals", &self.literals);
        __ds.field("recordDecls", &self.recordDecls);
        __ds.field("externalFunctionIncludes", &self.externalFunctionIncludes);
        __ds.field("generic_loop_calls", &self.generic_loop_calls);
        __ds.field("localKnownVars", &self.localKnownVars);
        __ds.field("allEquations", &self.allEquations);
        __ds.field("odeEquations", &self.odeEquations);
        __ds.field("algebraicEquations", &self.algebraicEquations);
        __ds.field("clockedPartitions", &self.clockedPartitions);
        __ds.field("initialEquations", &self.initialEquations);
        __ds.field("initialEquations_lambda0", &self.initialEquations_lambda0);
        __ds.field("removedInitialEquations", &self.removedInitialEquations);
        __ds.field("startValueEquations", &self.startValueEquations);
        __ds.field("nominalValueEquations", &self.nominalValueEquations);
        __ds.field("minValueEquations", &self.minValueEquations);
        __ds.field("maxValueEquations", &self.maxValueEquations);
        __ds.field("parameterEquations", &self.parameterEquations);
        __ds.field("removedEquations", &self.removedEquations);
        __ds.field("algorithmAndEquationAsserts", &self.algorithmAndEquationAsserts);
        __ds.field("equationsForZeroCrossings", &self.equationsForZeroCrossings);
        __ds.field("jacobianEquations", &self.jacobianEquations);
        __ds.field("stateSets", &self.stateSets);
        __ds.field("constraints", &self.constraints);
        __ds.field("classAttributes", &self.classAttributes);
        __ds.field("zeroCrossings", &self.zeroCrossings);
        __ds.field("relations", &self.relations);
        __ds.field("timeEvents", &self.timeEvents);
        __ds.field("discreteModelVars", &self.discreteModelVars);
        __ds.field("extObjInfo", &self.extObjInfo);
        __ds.field("makefileParams", &self.makefileParams);
        __ds.field("delayedExps", &self.delayedExps);
        __ds.field("spatialInfo", &self.spatialInfo);
        __ds.field("jacobianMatrices", &self.jacobianMatrices);
        __ds.field("simulationSettingsOpt", &self.simulationSettingsOpt);
        __ds.field("fileNamePrefix", &self.fileNamePrefix);
        __ds.field("fullPathPrefix", &self.fullPathPrefix);
        __ds.field("fmuTargetName", &self.fmuTargetName);
        __ds.field("hpcomData", &self.hpcomData);
        __ds.field("valueReferences", &self.valueReferences);
        __ds.field("varToArrayIndexMapping", &format_args!("<fn@{:p}>", std::sync::Arc::as_ptr(&self.varToArrayIndexMapping)));
        __ds.field("varToIndexMapping", &format_args!("<fn@{:p}>", std::sync::Arc::as_ptr(&self.varToIndexMapping)));
        __ds.field("crefToSimVarHT", &format_args!("<fn@{:p}>", std::sync::Arc::as_ptr(&self.crefToSimVarHT)));
        __ds.field("crefToClockIndexHT", &format_args!("<fn@{:p}>", std::sync::Arc::as_ptr(&self.crefToClockIndexHT)));
        __ds.field("backendMapping", &self.backendMapping);
        __ds.field("modelStructure", &self.modelStructure);
        __ds.field("fmiSimulationFlags", &self.fmiSimulationFlags);
        __ds.field("partitionData", &self.partitionData);
        __ds.field("daeModeData", &self.daeModeData);
        __ds.field("inlineEquations", &self.inlineEquations);
        __ds.field("omsiData", &self.omsiData);
        __ds.field("scalarized", &self.scalarized);
        __ds.finish()
    }
}

pub type SIMCODE = SimCode;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClockedPartition {
    pub baseClock: Arc<DAE::ClockKind>,
    pub subPartitions: Arc<metamodelica::List<SubPartition>>,
}

pub type CLOCKED_PARTITION = ClockedPartition;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SubPartition {
    pub vars: Arc<metamodelica::List<(SimCodeVar::SimVar, bool)>>,
    pub equations: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub removedEquations: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub subClock: BackendDAE::SubClock,
    pub holdEvents: bool,
}

pub type SUBPARTITION = SubPartition;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BackendMapping {
    BACKENDMAPPING {
        m: metamodelica::Array<Arc<metamodelica::List<i32>>>,
        mT: metamodelica::Array<Arc<metamodelica::List<i32>>>,
        eqMapping: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>,
        varMapping: Arc<metamodelica::List<(i32, i32)>>,
        eqMatch: metamodelica::Array<i32>,
        varMatch: metamodelica::Array<i32>,
        eqTree: metamodelica::Array<Arc<metamodelica::List<i32>>>,
        simVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>,
    },
    NO_MAPPING,
}
pub use self::BackendMapping::{BACKENDMAPPING,NO_MAPPING};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PartitionData {
    pub numPartitions: i32,
    pub partitions: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>,
    pub activatorsForPartitions: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>,
    pub stateToActivators: Arc<metamodelica::List<i32>>,
}

pub type PARTITIONDATA = PartitionData;


/// Delayed expressions type
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DelayedExpression {
    pub delayedExps: Arc<metamodelica::List<(i32, (Arc<DAE::Exp>, Arc<DAE::Exp>, Arc<DAE::Exp>))>>,
    pub maxDelayedIndex: i32,
}

pub type DELAYED_EXPRESSIONS = DelayedExpression;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpatialDistributionInfo {
    pub spatialDistributions: Arc<metamodelica::List<SpatialDistribution>>,
    pub maxIndex: i32,
}

pub type SPATIAL_DISTRIBUTION_INFO = SpatialDistributionInfo;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpatialDistribution {
    /// uniqueIndex
    pub index: i32,
    /// input 0
    pub in0: Arc<DAE::Exp>,
    /// input 1
    pub in1: Arc<DAE::Exp>,
    /// current pos
    pub pos: Arc<DAE::Exp>,
    /// flow direction
    pub dir: Arc<DAE::Exp>,
    /// initial grid points
    pub initPnts: Arc<DAE::Exp>,
    /// initial grid values
    pub initVals: Arc<DAE::Exp>,
    /// number of initial points
    pub initSize: i32,
}

pub type SPATIAL_DISTRIBUTION = SpatialDistribution;


/// unitDefinitions for fmi modelDescription.xml
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnitDefinition {
    pub name: ArcStr,
    pub baseUnit: BaseUnit,
}

pub type UNITDEFINITION = UnitDefinition;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BaseUnit {
    BASEUNIT {
        /// exponent
        s: i32,
        /// exponent
        m: i32,
        /// exponent
        kg: i32,
        /// exponent
        A: i32,
        /// exponent
        K: i32,
        /// exponent
        mol: i32,
        /// exponent
        cd: i32,
        /// prefix
        factor: metamodelica::Real,
        /// offset
        offset: metamodelica::Real,
    },
    /// no baseunit definition available
    NOBASEUNIT,
}
pub use self::BaseUnit::{BASEUNIT,NOBASEUNIT};

/// Container for metadata about a Modelica model.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ModelInfo {
    pub name: Arc<Absyn::Path>,
    pub description: ArcStr,
    pub version: ArcStr,
    pub author: ArcStr,
    pub license: ArcStr,
    pub copyright: ArcStr,
    pub directory: ArcStr,
    pub fileName: ArcStr,
    pub varInfo: VarInfo,
    pub vars: SimCodeVar::SimVars,
    pub functions: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>>,
    pub labels: Arc<metamodelica::List<ArcStr>>,
    /// Paths of all resources used by the model. Used in FMI2 to package resources in the FMU.
    pub resourcePaths: Arc<metamodelica::List<ArcStr>>,
    pub sortedClasses: Arc<metamodelica::List<Arc<Absyn::Class>>>,
    pub nClocks: i32,
    pub nSubClocks: i32,
    pub nSpatialDistributions: i32,
    pub hasLargeLinearEquationSystems: bool,
    pub linearSystems: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub nonLinearSystems: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    /// export unitDefintion in modelDescription.xml
    pub unitDefinitions: Arc<metamodelica::List<UnitDefinition>>,
}

pub type MODELINFO = ModelInfo;


pub type Files = Arc<metamodelica::List<FileInfo>>;

/// contains all the .mo files present in all SourceInfo and DAE.ElementSource.info
///   of all the variables, functions, etc from SimCode that have origin info.
///   it is used to generate the file information in one place and use an index
///   whenever we need to refer to one file from a var or function.
///   this is done so that we don't repeat long filenames everywhere.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileInfo {
    /// fileName where the class/component is defined in
    pub fileName: ArcStr,
    /// isReadOnly : (true|false). Should be true for libraries
    pub isReadOnly: bool,
}

pub type FILEINFO = FileInfo;


/// Number of variables of various types in a Modelica model.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VarInfo {
    pub numZeroCrossings: i32,
    pub numTimeEvents: i32,
    pub numRelations: i32,
    pub numMathEventFunctions: i32,
    pub numStateVars: i32,
    pub numAlgVars: i32,
    pub numDiscreteReal: i32,
    pub numIntAlgVars: i32,
    pub numBoolAlgVars: i32,
    pub numAlgAliasVars: i32,
    pub numIntAliasVars: i32,
    pub numBoolAliasVars: i32,
    pub numParams: i32,
    pub numIntParams: i32,
    pub numBoolParams: i32,
    pub numOutVars: i32,
    pub numInVars: i32,
    pub numExternalObjects: i32,
    pub numStringAlgVars: i32,
    pub numStringParamVars: i32,
    pub numStringAliasVars: i32,
    pub numEquations: i32,
    pub numLinearSystems: i32,
    pub numNonLinearSystems: i32,
    pub numMixedSystems: i32,
    pub numStateSets: i32,
    pub numJacobians: i32,
    pub numOptimizeConstraints: i32,
    pub numOptimizeFinalConstraints: i32,
    pub numSensitivityParameters: i32,
    pub numSetcVars: i32,
    pub numDataReconVars: i32,
    /// for fmi cs to interpolate inputs
    pub numRealInputVars: i32,
    /// for data reconciliation setB vars
    pub numSetbVars: i32,
    /// for data reconciliation count number of boundary conditions which failed the extraction algorithm
    pub numRelatedBoundaryConditions: i32,
}

pub type VARINFO = VarInfo;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DaeModeConfig {
    ALL_EQUATIONS,
    DYNAMIC_EQUATIONS,
}
pub use self::DaeModeConfig::{ALL_EQUATIONS,DYNAMIC_EQUATIONS};

/// contains data that belongs to the dae mode
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DaeModeData {
    /// daeModel residuals equations
    pub daeEquations: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimEqSystem>>>>>,
    /// contains the sparsity pattern for the daeMode
    pub sparsityPattern: Option<Arc<JacobianMatrix>>,
    /// variable used to calculate residuals of a DAE form, they are real
    pub residualVars: Arc<metamodelica::List<SimCodeVar::SimVar>>,
    pub algebraicVars: Arc<metamodelica::List<SimCodeVar::SimVar>>,
    pub auxiliaryVars: Arc<metamodelica::List<SimCodeVar::SimVar>>,
    pub modeCreated: DaeModeConfig,
}

pub type DAEMODEDATA = DaeModeData;


/// contains data for code generation for OMSI
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct OMSIData {
    /// contains equations and variables for initialization problem
    pub initialization: Arc<OMSIFunction>,
    /// contains equations and variables for simulation problem
    pub simulation: Arc<OMSIFunction>,
}

pub type OMSI_DATA = OMSIData;


/// contains equations and variables for initialization or simulation problem
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct OMSIFunction {
    /// causalized list of single equations and systems of equations
    pub equations: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    /// list of simcode variables determining input variables for equation(s)
    pub inputVars: Arc<metamodelica::List<SimCodeVar::SimVar>>,
    /// list of simcode variables determining output variables for equation(s)
    pub outputVars: Arc<metamodelica::List<SimCodeVar::SimVar>>,
    /// list of simcode variables determining inner variables for equation(s), e.g $DER(x)
    pub innerVars: Arc<metamodelica::List<SimCodeVar::SimVar>>,
    /// number of input, inner and output vars
    pub nAllVars: i32,
    /// contains crefToSimVar hash table for lookup function in templates
    pub context: SimCodeFunction::Context,
    /// number of linear and non-linear algebraic systems in OMSI_FUNCTION.equations
    pub nAlgebraicSystems: i32,
}

pub type OMSI_FUNCTION = OMSIFunction;


thread_local! { static __emptyOMSIFunction_TLS: Arc<OMSIFunction> = Arc::new(OMSIFunction { nAlgebraicSystems: 0, context: SimCodeFunction::contextOMSI().clone(), nAllVars: 0, innerVars: metamodelica::nil(), outputVars: metamodelica::nil(), inputVars: metamodelica::nil(), equations: metamodelica::nil() }); }
pub fn emptyOMSIFunction() -> Arc<OMSIFunction> { __emptyOMSIFunction_TLS.with(|__t| __t.clone()) }

/// Represents a single equation or a system of equations that must be solved together.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SimEqSystem {
    SES_RESIDUAL {
        index: i32,
        res_index: i32,
        exp: Arc<DAE::Exp>,
        source: Arc<DAE::ElementSource>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_FOR_RESIDUAL {
        index: i32,
        res_index: i32,
        iterators: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>,
        exp: Arc<DAE::Exp>,
        source: Arc<DAE::ElementSource>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    /// a generic residual calling a for loop body function with an index list.
    SES_GENERIC_RESIDUAL {
        index: i32,
        res_index: i32,
        scal_indices: Arc<metamodelica::List<i32>>,
        iterators: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>,
        exp: Arc<DAE::Exp>,
        source: Arc<DAE::ElementSource>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_SIMPLE_ASSIGN {
        index: i32,
        /// left hand side of equation
        cref: Arc<DAE::ComponentRef>,
        exp: Arc<DAE::Exp>,
        source: Arc<DAE::ElementSource>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    /// Solved inner equation of (casual) tearing set (Dynamic Tearing) with constraints on the solvability
    SES_SIMPLE_ASSIGN_CONSTRAINTS {
        index: i32,
        cref: Arc<DAE::ComponentRef>,
        exp: Arc<DAE::Exp>,
        source: Arc<DAE::ElementSource>,
        cons: Arc<metamodelica::List<Arc<DAE::Constraint>>>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_ARRAY_CALL_ASSIGN {
        index: i32,
        lhs: Arc<DAE::Exp>,
        exp: Arc<DAE::Exp>,
        source: Arc<DAE::ElementSource>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    /// a resizable assignment calling a for loop body function.
    SES_RESIZABLE_ASSIGN {
        index: i32,
        call_index: i32,
        iters: Arc<metamodelica::List<BackendDAE::SimIterator>>,
        source: Arc<DAE::ElementSource>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    /// a generic assignment calling a for loop body function with an index list.
    SES_GENERIC_ASSIGN {
        index: i32,
        call_index: i32,
        scal_indices: Arc<metamodelica::List<i32>>,
        source: Arc<DAE::ElementSource>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    /// entwined generic assignments calling for loop body functions with an index list and a call order.
    SES_ENTWINED_ASSIGN {
        index: i32,
        call_order: Arc<metamodelica::List<i32>>,
        single_calls: Arc<metamodelica::List<Arc<SimEqSystem>>>,
        source: Arc<DAE::ElementSource>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_IFEQUATION {
        index: i32,
        ifbranches: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<SimEqSystem>>>)>>,
        elsebranch: Arc<metamodelica::List<Arc<SimEqSystem>>>,
        source: Arc<DAE::ElementSource>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_ALGORITHM {
        index: i32,
        statements: Arc<metamodelica::List<Arc<DAE::Statement>>>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_INVERSE_ALGORITHM {
        index: i32,
        statements: Arc<metamodelica::List<Arc<DAE::Statement>>>,
        /// this is a subset of output crefs of the original algorithm, which are already known
        knownOutputCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>,
        insideNonLinearSystem: bool,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_LINEAR {
        lSystem: Arc<LinearSystem>,
        alternativeTearing: Option<Arc<LinearSystem>>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_NONLINEAR {
        nlSystem: Arc<NonlinearSystem>,
        alternativeTearing: Option<Arc<NonlinearSystem>>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_MIXED {
        index: i32,
        cont: Arc<SimEqSystem>,
        discVars: Arc<metamodelica::List<SimCodeVar::SimVar>>,
        discEqs: Arc<metamodelica::List<Arc<SimEqSystem>>>,
        indexMixedSystem: i32,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_WHEN {
        index: i32,
        /// list of boolean variables as conditions
        conditions: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>,
        /// true, if top-level branch with initial()
        initialCall: bool,
        whenStmtLst: Arc<metamodelica::List<BackendDAE::WhenOperator>>,
        elseWhen: Option<Arc<SimEqSystem>>,
        source: Arc<DAE::ElementSource>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_FOR_LOOP {
        index: i32,
        iter: Arc<DAE::Exp>,
        startIt: Arc<DAE::Exp>,
        endIt: Arc<DAE::Exp>,
        cref: Arc<DAE::ComponentRef>,
        exp: Arc<DAE::Exp>,
        source: Arc<DAE::ElementSource>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_FOR_EQUATION {
        index: i32,
        iter: Arc<DAE::Exp>,
        startIt: Arc<DAE::Exp>,
        endIt: Arc<DAE::Exp>,
        body: Arc<metamodelica::List<Arc<SimEqSystem>>>,
        source: Arc<DAE::ElementSource>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_ALIAS {
        index: i32,
        aliasOf: i32,
    },
    SES_ALGEBRAIC_SYSTEM {
        /// equation index
        index: i32,
        /// index of algebraic system
        algSysIndex: i32,
        /// dimension of algebraic loop (after tearing)
        dim_n: i32,
        partOfMixed: bool,
        tornSystem: bool,
        linearSystem: bool,
        residual: Arc<OMSIFunction>,
        matrix: Option<Arc<DerivativeMatrix>>,
        zeroCrossingConditions: Arc<metamodelica::List<i32>>,
        sources: Arc<metamodelica::List<Arc<DAE::ElementSource>>>,
        eqAttr: BackendDAE::EquationAttributes,
    },
}
pub use self::SimEqSystem::{SES_RESIDUAL,SES_FOR_RESIDUAL,SES_GENERIC_RESIDUAL,SES_SIMPLE_ASSIGN,SES_SIMPLE_ASSIGN_CONSTRAINTS,SES_ARRAY_CALL_ASSIGN,SES_RESIZABLE_ASSIGN,SES_GENERIC_ASSIGN,SES_ENTWINED_ASSIGN,SES_IFEQUATION,SES_ALGORITHM,SES_INVERSE_ALGORITHM,SES_LINEAR,SES_NONLINEAR,SES_MIXED,SES_WHEN,SES_FOR_LOOP,SES_FOR_EQUATION,SES_ALIAS,SES_ALGEBRAIC_SYSTEM};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SimGenericCall {
    SINGLE_GENERIC_CALL {
        index: i32,
        iters: Arc<metamodelica::List<BackendDAE::SimIterator>>,
        lhs: Arc<DAE::Exp>,
        rhs: Arc<DAE::Exp>,
        resizable: bool,
    },
    IF_GENERIC_CALL {
        index: i32,
        iters: Arc<metamodelica::List<BackendDAE::SimIterator>>,
        branches: Arc<metamodelica::List<SimBranch>>,
        resizable: bool,
    },
    WHEN_GENERIC_CALL {
        index: i32,
        iters: Arc<metamodelica::List<BackendDAE::SimIterator>>,
        branches: Arc<metamodelica::List<SimBranch>>,
        resizable: bool,
    },
}
pub use self::SimGenericCall::{SINGLE_GENERIC_CALL,IF_GENERIC_CALL,WHEN_GENERIC_CALL};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SimBranch {
    SIM_BRANCH {
        condition: Option<Arc<DAE::Exp>>,
        body: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>,
    },
    SIM_BRANCH_STMT {
        condition: Option<Arc<DAE::Exp>>,
        body: Arc<metamodelica::List<Arc<DAE::Statement>>>,
    },
}
pub use self::SimBranch::{SIM_BRANCH,SIM_BRANCH_STMT};

/// represents directional derivatives with sparsity and coloring
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DerivativeMatrix {
    pub columns: Arc<metamodelica::List<Arc<OMSIFunction>>>,
    /// unique matrix name
    pub matrixName: ArcStr,
    pub sparsity: SparsityPattern,
    pub sparsityT: SparsityPattern,
    pub coloredCols: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>,
    pub maxColorCols: i32,
}

pub type DERIVATIVE_MATRIX = DerivativeMatrix;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LinearSystem {
    pub index: i32,
    pub partOfMixed: bool,
    pub tornSystem: bool,
    pub vars: Arc<metamodelica::List<SimCodeVar::SimVar>>,
    pub beqs: Arc<metamodelica::List<Arc<DAE::Exp>>>,
    pub simJac: Arc<metamodelica::List<(i32, i32, Arc<SimEqSystem>)>>,
    pub residual: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub jacobianMatrix: Option<Arc<JacobianMatrix>>,
    pub sources: Arc<metamodelica::List<Arc<DAE::ElementSource>>>,
    pub indexLinearSystem: i32,
    /// Number of variables that are solved in this system. Needed because 'crefs' only contains the iteration variables.
    pub nUnknowns: i32,
    /// if TRUE then this system is part of a jacobian matrix
    pub partOfJac: bool,
}

pub type LINEARSYSTEM = LinearSystem;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NonlinearSystem {
    pub index: i32,
    pub eqs: Arc<metamodelica::List<Arc<SimEqSystem>>>,
    pub crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>,
    pub indexNonLinearSystem: i32,
    /// Number of variables that are solved in this system. Needed because 'crefs' only contains the iteration variables.
    pub nUnknowns: i32,
    pub jacobianMatrix: Option<Arc<JacobianMatrix>>,
    pub homotopySupport: bool,
    pub mixedSystem: bool,
    pub tornSystem: bool,
    pub clockIndex: Option<i32>,
}

pub type NONLINEARSYSTEM = NonlinearSystem;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StateSet {
    pub index: i32,
    pub nCandidates: i32,
    pub nStates: i32,
    pub states: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>,
    pub statescandidates: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>,
    pub crA: Arc<DAE::ComponentRef>,
    pub jacobianMatrix: Arc<JacobianMatrix>,
}

pub type SES_STATESET = StateSet;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExtObjInfo {
    pub vars: Arc<metamodelica::List<SimCodeVar::SimVar>>,
    pub aliases: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>,
}

pub type EXTOBJINFO = ExtObjInfo;


/// Settings for simulation init file header.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SimulationSettings {
    pub startTime: metamodelica::Real,
    pub stopTime: metamodelica::Real,
    pub numberOfIntervals: i32,
    pub stepSize: metamodelica::Real,
    pub tolerance: metamodelica::Real,
    pub method: ArcStr,
    pub options: ArcStr,
    pub outputFormat: ArcStr,
    pub variableFilter: ArcStr,
    pub cflags: ArcStr,
    pub simflags: ArcStr,
}

pub type SIMULATION_SETTINGS = SimulationSettings;


/* ***** HashTable ComponentRef -> SimCodeVar.SimVar ******/
pub type Key = Arc<DAE::ComponentRef>;

pub type Value = SimCodeVar::SimVar;

pub type HashTableCrefToSimVar = (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, SimCodeVar::SimVar)>>), i32, (HashTableCrefSimVar::FuncHashCref, HashTableCrefSimVar::FuncCrefEqual, HashTableCrefSimVar::FuncCrefStr, HashTableCrefSimVar::FuncExpStr));

/* FMI 2.0 Export */
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FmiUnknown {
    pub index: i32,
    pub dependencies: Arc<metamodelica::List<i32>>,
    pub dependenciesKind: Arc<metamodelica::List<ArcStr>>,
}

pub type FMIUNKNOWN = FmiUnknown;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FmiOutputs {
    pub fmiUnknownsList: Arc<metamodelica::List<FmiUnknown>>,
}

pub type FMIOUTPUTS = FmiOutputs;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FmiDerivatives {
    pub fmiUnknownsList: Arc<metamodelica::List<FmiUnknown>>,
}

pub type FMIDERIVATIVES = FmiDerivatives;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FmiDiscreteStates {
    pub fmiUnknownsList: Arc<metamodelica::List<FmiUnknown>>,
}

pub type FMIDISCRETESTATES = FmiDiscreteStates;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FmiInitialUnknowns {
    pub fmiUnknownsList: Arc<metamodelica::List<FmiUnknown>>,
    /// use the sorted crefs to get the ValueReference of unknowns
    pub sortedUnknownCrefs: Arc<metamodelica::List<(i32, Arc<DAE::ComponentRef>)>>,
    /// use the sorted crefs to get the ValueReference of knowns
    pub sortedknownCrefs: Arc<metamodelica::List<(i32, Arc<DAE::ComponentRef>)>>,
}

pub type FMIINITIALUNKNOWNS = FmiInitialUnknowns;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FmiModelStructure {
    pub fmiOutputs: FmiOutputs,
    pub fmiDerivatives: FmiDerivatives,
    pub continuousPartialDerivatives: Option<Arc<JacobianMatrix>>,
    pub initialPartialDerivatives: Option<Arc<JacobianMatrix>>,
    pub fmiDiscreteStates: FmiDiscreteStates,
    pub fmiInitialUnknowns: FmiInitialUnknowns,
}

pub type FMIMODELSTRUCTURE = FmiModelStructure;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FmiSimulationFlags {
    FMI_SIMULATION_FLAGS {
        nameValueTuples: Arc<metamodelica::List<(ArcStr, ArcStr)>>,
    },
    FMI_SIMULATION_FLAGS_FILE {
        path: ArcStr,
    },
}
pub use self::FmiSimulationFlags::{FMI_SIMULATION_FLAGS,FMI_SIMULATION_FLAGS_FILE};

pub static defaultFmiSimulationFlags: std::sync::LazyLock<FmiSimulationFlags> = std::sync::LazyLock::new(|| { FmiSimulationFlags::FMI_SIMULATION_FLAGS { nameValueTuples: list![(literal!("s"), literal!("euler"))] } });

