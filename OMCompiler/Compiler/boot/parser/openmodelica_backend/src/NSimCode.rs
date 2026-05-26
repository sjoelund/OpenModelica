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

use crate::BackendDAE as OldBackendDAE;
use crate::CevalScriptBackend;
use crate::HashTableCrefSimVar;
use crate::HpcOmSimCode;
use crate::InteractiveUtil;
use crate::NBEquation as BEquation;
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationPointer;
use crate::NBEquation::EquationPointers;
use crate::NBEvents::EventInfo;
use crate::NBPartition as Partition;
use crate::NBPartitioning::ClockedInfo;
use crate::NBStrongComponent::AliasInfo;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use crate::NSimCodeUtil as SimCodeUtil;
use crate::NSimGenericCall as SimGenericCall;
use crate::NSimJacobian::SimJacobian;
use crate::NSimPartition as SimPartition;
use crate::NSimStrongComponent as SimStrongComponent;
use crate::NSimVar::ExtObjInfo;
use crate::NSimVar::SimVar;
use crate::NSimVar::SimVars;
use crate::NSimVar::VarInfo;
use crate::SimCode as OldSimCode;
use crate::SimCodeFunction;
use crate::SimCodeFunctionUtil as OldSimCodeFunctionUtil;
use crate::SimCodeUtil as OldSimCodeUtil;
use crate::SimCodeVar;
use crate::SymbolTable;
use openmodelica_ast::Absyn;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::Expression as OldExpression;
use openmodelica_frontend::HashTable;
use openmodelica_frontend::HashTableCrIListArray;
use openmodelica_frontend::HashTableCrILst;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_nf_frontend::NFBuiltinCall as BuiltinCall;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFConvertDAE as ConvertDAE;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFlatten::FunctionTree;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFInstNode::InstNode;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

// OF imports
// NF imports
// Old Backend imports
// Backend imports
// SimCode imports
// Old SimCode imports
// Util imports
// Script imports
/// Unique simulation code indices
#[derive(Clone, Debug, PartialEq)]
pub struct SimCodeIndices {
    pub uniqueIndex: i32,
    pub realVarIndex: i32,
    pub integerVarIndex: i32,
    pub booleanVarIndex: i32,
    pub stringVarIndex: i32,
    pub enumerationVarIndex: i32,
    pub realParamIndex: i32,
    pub integerParamIndex: i32,
    pub booleanParamIndex: i32,
    pub stringParamIndex: i32,
    pub enumerationParamIndex: i32,
    pub realAliasIndex: i32,
    pub integerAliasIndex: i32,
    pub booleanAliasIndex: i32,
    pub stringAliasIndex: i32,
    pub enumerationAliasIndex: i32,
    pub equationIndex: i32,
    pub linearSystemIndex: i32,
    pub nonlinearSystemIndex: i32,
    pub jacobianIndex: i32,
    pub residualIndex: i32,
    pub implicitIndex: i32,
    pub extObjIndex: i32,
    pub alias_map: Arc<UnorderedMap::UnorderedMap<Arc<AliasInfo::AliasInfo>, i32>>,
    pub generic_call_map: Arc<UnorderedMap::UnorderedMap<Arc<Identifier::Identifier>, i32>>,
}

pub type SIM_CODE_INDICES = SimCodeIndices;


pub mod Identifier {
    use super::*;
#[derive(Clone, Debug, PartialEq)]
    pub struct Identifier {
        pub eqn: Pointer::Pointer<Arc<Equation::Equation>>,
        pub var_cref: Arc<ComponentRef::NFComponentRef>,
        pub resizable: bool,
    }

    pub type IDENTIFIER = Identifier;

    pub fn toString(mut ident: Arc<Identifier>) -> ArcStr {
        let mut r#str: ArcStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("cref: ")); __mm_s.push_str(&*ComponentRef::toString(ident.var_cref.clone()).unwrap()); __mm_s.push_str(&*literal!("\neqn: ")); __mm_s.push_str(&*BEquation::Equation::pointerToString(ident.eqn.clone(), (literal!("")).clone()).unwrap()); __mm_s.push_str(&*literal!("\n(resizable=")); __mm_s.push_str(&*boolString(ident.resizable.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) };
        r#str
    }

    pub fn hash(mut ident: Arc<Identifier>) -> i32 {
        let mut i: i32 = stringHashDjb2((toString(ident.clone())).clone());
        i
    }

    pub fn isEqual(mut ident1: Arc<Identifier>, mut ident2: Arc<Identifier>) -> bool {
        let mut b: bool = BEquation::Equation::equalName(ident1.eqn.clone(), ident2.eqn.clone()) && ComponentRef::isEqual(ident1.var_cref.clone(), ident2.var_cref.clone()).unwrap();
        b
    }

}

pub fn EMPTY_SIM_CODE_INDICES() -> SimCodeIndices {
    let mut indices: SimCodeIndices = SimCodeIndices { uniqueIndex: 1, realVarIndex: 0, integerVarIndex: 0, booleanVarIndex: 0, stringVarIndex: 0, enumerationVarIndex: 0, realParamIndex: 0, integerParamIndex: 0, booleanParamIndex: 0, stringParamIndex: 0, enumerationParamIndex: 0, realAliasIndex: 0, integerAliasIndex: 0, booleanAliasIndex: 0, stringAliasIndex: 0, enumerationAliasIndex: 0, equationIndex: 1, linearSystemIndex: 0, nonlinearSystemIndex: 0, jacobianIndex: 0, residualIndex: 0, implicitIndex: 0, extObjIndex: 0, alias_map: UnorderedMap::new(fnptr!(AliasInfo::hash, Arc<AliasInfo::AliasInfo>), fnptr!(AliasInfo::isEqual, Arc<AliasInfo::AliasInfo>, Arc<AliasInfo::AliasInfo>), 1), generic_call_map: UnorderedMap::new(fnptr!(Identifier::hash, Arc<Identifier::Identifier>), fnptr!(Identifier::isEqual, Arc<Identifier::Identifier>, Arc<Identifier::Identifier>), 1) };
    indices
}

pub mod SimCode {
    use super::*;
#[derive(Clone, Debug, PartialEq)]
    pub struct SimCode {
        pub modelInfo: Arc<ModelInfo::ModelInfo>,
        /// shared literals
        pub literals: Arc<metamodelica::List<Arc<Expression::NFExpression>>>,
        pub recordDecls: Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>>,
        /// Names of all external functions that are called
        pub externalFunctionIncludes: Arc<metamodelica::List<ArcStr>>,
        /// Generic for-loop and array calls
        pub generic_loop_calls: Arc<metamodelica::List<Arc<SimGenericCall::NSimGenericCall>>>,
        /// state and strictly input dependent variables. they are not inserted into any partion
        pub independent: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>,
        /// All simulation system blocks
        pub allSim: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>,
        /// Only ode blocks for integrator
        pub ode: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>>>,
        /// Additional purely algebraic blocks
        pub algebraic: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>>>,
        /// Clocked Partitions
        pub clockedPartitions: Arc<metamodelica::List<Arc<SimPartition::NSimPartition>>>,
        /// Blocks for nominal value equations
        pub nominal: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>,
        /// Blocks for min value equations
        pub min: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>,
        /// Blocks for max value equations
        pub max: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>,
        /// Blocks for parameter equations
        pub param: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>,
        /// Blocks for equations without return value
        pub no_ret: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>,
        /// Blocks for algorithms and asserts
        pub algorithms: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>,
        /// Blocks for zero crossing functions
        pub event_blocks: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>,
        /// Blocks for jacobian equations
        pub jac_blocks: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>,
        /// Blocks for start value equations
        pub start: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>,
        /// Blocks for initial equations
        pub init: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>,
        /// Blocks for initial lambda 0 equations (homotopy)
        pub init_0: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>,
        /// Blocks for initial equations without return value
        pub init_no_ret: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>,
        /// List of discrete variables
        pub discreteVars: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>,
        pub extObjInfo: Arc<ExtObjInfo::ExtObjInfo>,
        pub makefileParams: SimCodeFunction::MakefileParams,
        /// List of symbolic jacobians
        pub jacobians: Arc<metamodelica::List<Arc<SimJacobian::SimJacobian>>>,
        pub simulationSettingsOpt: Option<OldSimCode::SimulationSettings>,
        pub fileNamePrefix: ArcStr,
        pub simcode_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>,
        pub equation_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimStrongComponent::Block::Block>>>,
        pub eventInfo: Arc<EventInfo::EventInfo>,
        /// Simulation system in case of DAEMode
        pub daeModeData: Option<Arc<DaeModeData::DaeModeData>>,
        pub inlineEquations: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>,
    }

    pub type SIM_CODE = SimCode;

    pub fn toString(mut simCode: Arc<SimCode>, mut r#str: ArcStr) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        let mut idx: i32 = 1;
        r#str = (StringUtil::headline_1(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SimCode ")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())).clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*ModelInfo::toString(simCode.modelInfo.clone())?); ArcStr::from(__mm_s) }).clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*ExtObjInfo::toString(simCode.extObjInfo.clone())); ArcStr::from(__mm_s) }).clone();
        if !(simCode.init_0.clone().is_empty()) {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*SimStrongComponent::Block::listToString(simCode.init_0.clone(), (literal!("  ")).clone(), (literal!("Initial Partition (Lambda = 0)")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        }
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*SimStrongComponent::Block::listToString(simCode.init.clone(), (literal!("  ")).clone(), (literal!("Initial Partition")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        for mut blck_lst in &*simCode.ode.clone() {
            let mut blck_lst = blck_lst.clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*SimStrongComponent::Block::listToString(blck_lst.clone(), (literal!("  ")).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ODE Partition ")); __mm_s.push_str(&*intString(idx.clone())); ArcStr::from(__mm_s) }).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            idx = idx.clone() + 1;
        }
        idx = 1;
        for mut blck_lst in &*simCode.algebraic.clone() {
            let mut blck_lst = blck_lst.clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*SimStrongComponent::Block::listToString(blck_lst.clone(), (literal!("  ")).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Algebraic Partition ")); __mm_s.push_str(&*intString(idx.clone())); ArcStr::from(__mm_s) }).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            idx = idx.clone() + 1;
        }
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*SimStrongComponent::Block::listToString(simCode.event_blocks.clone(), (literal!("  ")).clone(), (literal!("Event Partition")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        if !(simCode.clockedPartitions.clone().is_empty()) {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*SimPartition::listToString(simCode.clockedPartitions.clone(), (literal!("  ")).clone(), (literal!("Clocked Partitions")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        }
        if !(simCode.literals.clone().is_empty()) {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*StringUtil::headline_3((literal!("Shared Literals")).clone())); ArcStr::from(__mm_s) }).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*List::toString(simCode.literals.clone(), Arc::new(OldExpression::toString), (literal!("")).clone(), (literal!("  ")).clone(), (literal!("\n  ")).clone(), (literal!("\n\n")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone();
        }
        if !(simCode.generic_loop_calls.clone().is_empty()) {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*StringUtil::headline_3((literal!("Generic Calls")).clone())); ArcStr::from(__mm_s) }).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*List::toString(simCode.generic_loop_calls.clone(), Arc::new(SimGenericCall::toString), (literal!("")).clone(), (literal!("  ")).clone(), (literal!("\n  ")).clone(), (literal!("\n\n")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone();
        }
        if isSome(simCode.daeModeData.clone()) {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*DaeModeData::toString(Util::getOption(simCode.daeModeData.clone())?)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        }
        for mut jac in &*simCode.jacobians.clone() {
            let mut jac = jac.clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*SimJacobian::toString(jac.clone())?); ArcStr::from(__mm_s) }).clone();
        }
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*EventInfo::toString(simCode.eventInfo.clone())?); ArcStr::from(__mm_s) }).clone();
        Ok(r#str)
    }

    pub fn convert(mut simCode: Arc<SimCode>) -> Result<OldSimCode::SimCode> {
        let mut oldSimCode: OldSimCode::SimCode;
        let mut modelInfo: OldSimCode::ModelInfo;
        let mut discreteModelVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        let mut zeroCrossings: Arc<metamodelica::List<OldBackendDAE::ZeroCrossing>> = metamodelica::nil();
        let mut relations: Arc<metamodelica::List<OldBackendDAE::ZeroCrossing>> = metamodelica::nil();
        let mut timeEvents: Arc<metamodelica::List<OldBackendDAE::TimeEvent>> = metamodelica::nil();
        let mut varToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (HashTableCrIListArray::FuncHashCref, HashTableCrIListArray::FuncCrefEqual, HashTableCrIListArray::FuncCrefStr, HashTableCrIListArray::FuncExpStr));
        let mut varToIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr));
        let mut crefToSimVarHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, SimCodeVar::SimVar)>>), i32, (HashTableCrefSimVar::FuncHashCref, HashTableCrefSimVar::FuncCrefEqual, HashTableCrefSimVar::FuncCrefStr, HashTableCrefSimVar::FuncExpStr));
        let mut crefToClockIndexHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
        let mut residualVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        modelInfo = ModelInfo::convert(simCode.modelInfo.clone());
        (zeroCrossings, relations, timeEvents) = EventInfo::convert(simCode.eventInfo.clone(), simCode.equation_map.clone())?;
        (varToArrayIndexMapping, varToIndexMapping) = OldSimCodeUtil::createVarToArrayIndexMapping(modelInfo.clone())?;
        crefToSimVarHT = SimCodeUtil::convertSimCodeMap(simCode.simcode_map.clone());
        if isSome(simCode.daeModeData.clone()) {
            let __pa0 = ::match_deref::match_deref! { match &(simCode.daeModeData.clone()) {
                Some(Deref @ DaeModeData::DAE_MODE_DATA { residualVars: __pa0, .. }) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            residualVars = __pa0.clone();
            crefToSimVarHT = List::fold(SimVar::convertList(residualVars.clone()), Arc::new(HashTableCrefSimVar::addSimVarToHashTable), crefToSimVarHT.clone());
        }
        crefToClockIndexHT = HashTable::emptyHashTable();
        for mut cref in &*simCode.discreteVars.clone() {
            let mut cref = cref.clone();
            discreteModelVars = cons(ComponentRef::toDAE(cref.clone())?, discreteModelVars.clone());
        }
        oldSimCode = OldSimCode::SimCode { scalarized: Flags::getConfigBool(Flags::SIM_CODE_SCALARIZE.clone())?, omsiData: None, inlineEquations: metamodelica::nil(), daeModeData: if (isSome(simCode.daeModeData.clone())) {Some(DaeModeData::convert(Util::getOption(simCode.daeModeData.clone())?)?)} else {None}, partitionData: OldSimCode::PartitionData { numPartitions: -1, partitions: metamodelica::nil(), activatorsForPartitions: metamodelica::nil(), stateToActivators: metamodelica::nil() }, fmiSimulationFlags: None, modelStructure: None, backendMapping: None, crefToClockIndexHT: crefToClockIndexHT.clone(), crefToSimVarHT: crefToSimVarHT.clone(), varToIndexMapping: varToIndexMapping.clone(), varToArrayIndexMapping: varToArrayIndexMapping.clone(), valueReferences: Arc::new(crate::AvlTreeCRToInt::Tree::EMPTY), hpcomData: HpcOmSimCode::emptyHpcomData().clone(), fmuTargetName: (literal!("")).clone(), fullPathPrefix: (literal!("")).clone(), fileNamePrefix: (simCode.fileNamePrefix.clone()).clone(), simulationSettingsOpt: simCode.simulationSettingsOpt.clone(), jacobianMatrices: {
        let mut __acc: Arc<metamodelica::List<Arc<OldSimCode::JacobianMatrix>>> = metamodelica::nil();
        for mut jac in (simCode.jacobians.clone()).into_iter().cloned() {
            let __x = SimJacobian::convert(jac.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, spatialInfo: OldSimCode::SpatialDistributionInfo { spatialDistributions: metamodelica::nil(), maxIndex: 0 }, delayedExps: OldSimCode::DelayedExpression { delayedExps: metamodelica::nil(), maxDelayedIndex: 0 }, makefileParams: simCode.makefileParams.clone(), extObjInfo: ExtObjInfo::convert(simCode.extObjInfo.clone()), discreteModelVars: discreteModelVars.clone(), timeEvents: timeEvents.clone(), relations: relations.clone(), zeroCrossings: zeroCrossings.clone(), classAttributes: metamodelica::nil(), constraints: metamodelica::nil(), stateSets: metamodelica::nil(), jacobianEquations: SimStrongComponent::Block::convertList(simCode.jac_blocks.clone()), equationsForZeroCrossings: SimStrongComponent::Block::convertList(simCode.event_blocks.clone()), algorithmAndEquationAsserts: SimStrongComponent::Block::convertList(simCode.algorithms.clone()), removedEquations: SimStrongComponent::Block::convertList(simCode.no_ret.clone()), parameterEquations: SimStrongComponent::Block::convertList(simCode.param.clone()), maxValueEquations: SimStrongComponent::Block::convertList(simCode.max.clone()), minValueEquations: SimStrongComponent::Block::convertList(simCode.min.clone()), nominalValueEquations: SimStrongComponent::Block::convertList(simCode.nominal.clone()), startValueEquations: SimStrongComponent::Block::convertList(simCode.start.clone()), removedInitialEquations: SimStrongComponent::Block::convertList(simCode.init_no_ret.clone()), initialEquations_lambda0: SimStrongComponent::Block::convertList(simCode.init_0.clone()), initialEquations: SimStrongComponent::Block::convertList(simCode.init.clone()), clockedPartitions: {
        let mut __acc: Arc<metamodelica::List<OldSimCode::ClockedPartition>> = metamodelica::nil();
        for mut part in (simCode.clockedPartitions.clone()).into_iter().cloned() {
            let __x = SimPartition::convertBase(part.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, algebraicEquations: SimStrongComponent::Block::convertListList(simCode.algebraic.clone()), odeEquations: SimStrongComponent::Block::convertListList(simCode.ode.clone()), allEquations: SimStrongComponent::Block::convertList(simCode.allSim.clone()), localKnownVars: SimStrongComponent::Block::convertList(simCode.independent.clone()), generic_loop_calls: {
        let mut __acc: Arc<metamodelica::List<OldSimCode::SimGenericCall>> = metamodelica::nil();
        for mut gc in (simCode.generic_loop_calls.clone()).into_iter().cloned() {
            let __x = SimGenericCall::convert(gc.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, externalFunctionIncludes: simCode.externalFunctionIncludes.clone(), recordDecls: simCode.recordDecls.clone(), literals: {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut lit in (simCode.literals.clone()).into_iter().cloned() {
            let __x = Expression::toDAE(lit.clone(), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, modelInfo: modelInfo.clone() };
        Ok(oldSimCode)
    }

    pub fn getDirectoryAndLibs(mut simCode: Arc<SimCode>) -> Result<(ArcStr, Arc<metamodelica::List<ArcStr>>)> {
        let mut directory: ArcStr = arcstr::literal!("");
        let mut libs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        (directory, libs) = (::match_deref::match_deref! { match &(simCode.clone()) {
        Deref @ SimCode { makefileParams: SimCodeFunction::MakefileParams { libs, .. }, modelInfo: Deref @ ModelInfo::MODEL_INFO { directory, .. }, .. } => (directory.clone(), libs.clone()),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimCode.SimCode.getDirectoryAndLibs")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok((directory, libs))
    }

    fn collectAlgebraicLoops(mut init: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>, mut init_0: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>, mut ode: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>>>, mut algebraic: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>>>, mut daeModeData: Option<Arc<DaeModeData::DaeModeData>>, mut simCodeIndices: SimCodeIndices, mut simcode_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>) -> Result<(Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>, Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>, Arc<metamodelica::List<Arc<SimJacobian::SimJacobian>>>, SimCodeIndices)> {
        let mut linearLoops: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>> = metamodelica::nil();
        let mut nonlinearLoops: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>> = metamodelica::nil();
        let mut jacobians: Arc<metamodelica::List<Arc<SimJacobian::SimJacobian>>> = metamodelica::nil();
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut dae_mode_blcks: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>>> = metamodelica::nil();
        (linearLoops, nonlinearLoops, jacobians, simCodeIndices) = SimStrongComponent::Block::collectAlgebraicLoops(list![init.clone(), init_0.clone()], linearLoops.clone(), nonlinearLoops.clone(), jacobians.clone(), simCodeIndices.clone(), simcode_map.clone())?;
        (linearLoops, nonlinearLoops, jacobians, simCodeIndices) = SimStrongComponent::Block::collectAlgebraicLoops(ode.clone(), linearLoops.clone(), nonlinearLoops.clone(), jacobians.clone(), simCodeIndices.clone(), simcode_map.clone())?;
        (linearLoops, nonlinearLoops, jacobians, simCodeIndices) = SimStrongComponent::Block::collectAlgebraicLoops(algebraic.clone(), linearLoops.clone(), nonlinearLoops.clone(), jacobians.clone(), simCodeIndices.clone(), simcode_map.clone())?;
        if isSome(daeModeData.clone()) {
            let __pa0 = ::match_deref::match_deref! { match &(daeModeData.clone()) {
                Some(Deref @ DaeModeData::DAE_MODE_DATA { blcks: __pa0, .. }) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            dae_mode_blcks = __pa0.clone();
            (linearLoops, nonlinearLoops, jacobians, simCodeIndices) = SimStrongComponent::Block::collectAlgebraicLoops(dae_mode_blcks.clone(), linearLoops.clone(), nonlinearLoops.clone(), jacobians.clone(), simCodeIndices.clone(), simcode_map.clone())?;
        }
        Ok((linearLoops, nonlinearLoops, jacobians, simCodeIndices))
    }

}

pub mod ModelInfo {
    use super::*;
#[derive(Clone, Debug, PartialEq)]
    pub struct ModelInfo {
        pub name: Arc<Absyn::Path>,
        pub description: ArcStr,
        pub version: ArcStr,
        pub author: ArcStr,
        pub license: ArcStr,
        pub copyright: ArcStr,
        pub directory: ArcStr,
        pub fileName: ArcStr,
        pub vars: Arc<SimVars::SimVars>,
        pub varInfo: Arc<VarInfo::VarInfo>,
        pub functions: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>>,
        pub labels: Arc<metamodelica::List<ArcStr>>,
        /// Paths of all resources used by the model. Used in FMI2 to package resources in the FMU.
        pub resourcePaths: Arc<metamodelica::List<ArcStr>>,
        pub sortedClasses: Arc<metamodelica::List<Arc<Absyn::Class>>>,
        pub nClocks: i32,
        pub nSubClocks: i32,
        pub nSpatialDistributions: i32,
        pub hasLargeLinearEquationSystems: bool,
        pub linearLoops: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>,
        pub nonlinearLoops: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>,
    }

    pub type MODEL_INFO = ModelInfo;

    pub fn toString(mut modelInfo: Arc<ModelInfo>) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = (SimVars::toString(modelInfo.vars.clone(), (literal!("")).clone())?).clone();
        Ok(r#str)
    }

    pub fn create(mut vars: Arc<SimVars::SimVars>, mut name: Arc<Absyn::Path>, mut fileName: ArcStr, mut directory: ArcStr, mut functions: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>>, mut linearLoops: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>, mut nonlinearLoops: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>, mut eventInfo: Arc<EventInfo::EventInfo>, mut clockedInfo: Arc<ClockedInfo::ClockedInfo>, mut simCodeIndices: SimCodeIndices) -> (Arc<ModelInfo>, SimCodeIndices) {
        let mut modelInfo: Arc<ModelInfo>;
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut info: Arc<VarInfo::VarInfo>;
        info = VarInfo::create(vars.clone(), eventInfo.clone(), simCodeIndices.clone());
        modelInfo = Arc::new(ModelInfo { nonlinearLoops: nonlinearLoops.clone(), linearLoops: linearLoops.clone(), hasLargeLinearEquationSystems: true, nSpatialDistributions: 0, nSubClocks: ClockedInfo::subClockCount(clockedInfo.clone()), nClocks: ClockedInfo::baseClockCount(clockedInfo.clone(), false), sortedClasses: metamodelica::nil(), resourcePaths: metamodelica::nil(), labels: metamodelica::nil(), functions: functions.clone(), varInfo: info.clone(), vars: vars.clone(), fileName: (fileName.clone()).clone(), directory: (directory.clone()).clone(), copyright: (literal!("")).clone(), license: (literal!("")).clone(), author: (literal!("")).clone(), version: (literal!("")).clone(), description: (literal!("")).clone(), name: name.clone() });
        (modelInfo, simCodeIndices)
    }

    pub fn setSeedVars(mut modelInfo: Arc<ModelInfo>, mut seedVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>) -> Result<Arc<ModelInfo>> {
        let mut modelInfo: Arc<ModelInfo> = modelInfo;
        modelInfo = (::match_deref::match_deref! { match &(modelInfo.clone()) {
        Deref @ ModelInfo { vars, .. } => {
            let mut vars = (*vars).clone();
            assign_field!(vars.seedVars = seedVars.clone());
            assign_field!(modelInfo.vars = vars.clone());
            modelInfo.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimCode.ModelInfo.setSeedVars")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(modelInfo)
    }

    pub fn convert(mut modelInfo: Arc<ModelInfo>) -> OldSimCode::ModelInfo {
        let mut oldModelInfo: OldSimCode::ModelInfo;
        let mut varInfo: OldSimCode::VarInfo;
        varInfo = VarInfo::convert(modelInfo.varInfo.clone());
        oldModelInfo = OldSimCode::ModelInfo { unitDefinitions: metamodelica::nil(), nonLinearSystems: SimStrongComponent::Block::convertList(modelInfo.nonlinearLoops.clone()), linearSystems: SimStrongComponent::Block::convertList(modelInfo.linearLoops.clone()), hasLargeLinearEquationSystems: modelInfo.hasLargeLinearEquationSystems.clone(), nSpatialDistributions: modelInfo.nSpatialDistributions.clone(), nSubClocks: modelInfo.nSubClocks.clone(), nClocks: modelInfo.nClocks.clone(), sortedClasses: modelInfo.sortedClasses.clone(), resourcePaths: modelInfo.resourcePaths.clone(), labels: modelInfo.labels.clone(), functions: modelInfo.functions.clone(), vars: SimVars::convert(modelInfo.vars.clone()), varInfo: VarInfo::convert(modelInfo.varInfo.clone()), fileName: (modelInfo.fileName.clone()).clone(), directory: (modelInfo.directory.clone()).clone(), copyright: (modelInfo.copyright.clone()).clone(), license: (modelInfo.license.clone()).clone(), author: (modelInfo.author.clone()).clone(), version: (modelInfo.version.clone()).clone(), description: (modelInfo.description.clone()).clone(), name: modelInfo.name.clone() };
        oldModelInfo
    }

}

pub mod DaeModeData {
    use super::*;
    /// contains data that belongs to the dae mode
#[derive(Clone, Debug, PartialEq)]
    pub struct DaeModeData {
        /// daeMode blocks
        pub blcks: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>>>,
        /// contains the sparsity pattern for the daeMode
        pub sparsityPattern: Option<Arc<SimJacobian::SimJacobian>>,
        /// variable used to calculate residuals of a DAE form, they are of type real
        pub residualVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub algebraicVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub auxiliaryVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub modeCreated: DaeModeConfig,
    }

    pub type DAE_MODE_DATA = DaeModeData;

    pub fn toString(mut data: Arc<DaeModeData>) -> Result<ArcStr> {
        let mut r#str: ArcStr = literal!("");
        let mut idx: i32 = 1;
        for mut blck_lst in &*data.blcks.clone() {
            let mut blck_lst = blck_lst.clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*SimStrongComponent::Block::listToString(blck_lst.clone(), (literal!("  ")).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("DAE Partition ")); __mm_s.push_str(&*intString(idx.clone())); ArcStr::from(__mm_s) }).clone())?); ArcStr::from(__mm_s) }).clone();
            idx = idx.clone() + 1;
        }
        if isSome(data.sparsityPattern.clone()) {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*SimJacobian::toString(Util::getOption(data.sparsityPattern.clone())?)?); ArcStr::from(__mm_s) }).clone();
        }
        Ok(r#str)
    }

    pub fn create(mut systems: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>, mut simCodeIndices: SimCodeIndices, mut simcode_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>, mut equation_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimStrongComponent::Block::Block>>>) -> Result<(Option<Arc<DaeModeData>>, SimCodeIndices)> {
        let mut data: Option<Arc<DaeModeData>> = None;
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut blcks: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>>> = metamodelica::nil();
        let mut residualVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut algebraicVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        (blcks, residualVars, simCodeIndices) = SimStrongComponent::Block::createDAEModeBlocks(systems.clone(), simCodeIndices.clone(), simcode_map.clone(), equation_map.clone())?;
        data = Some(Arc::new(DaeModeData { blcks: blcks.clone(), sparsityPattern: None, residualVars: residualVars.clone(), algebraicVars: metamodelica::nil(), auxiliaryVars: metamodelica::nil(), modeCreated: DaeModeConfig::ALL.clone() }));
        Ok((data, simCodeIndices))
    }

    pub fn addJacobian(mut data: Option<Arc<DaeModeData>>, mut daeModeJac: Arc<SimJacobian::SimJacobian>) -> Option<Arc<DaeModeData>> {
        let mut data: Option<Arc<DaeModeData>> = data;
        data = (::match_deref::match_deref! { match &(data.clone()) {
        Some(dmd) => {
            let mut dmd = (*dmd).clone();
            assign_field!(dmd.sparsityPattern = Some(daeModeJac.clone()));
            Some(dmd.clone())
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        data
    }

    pub fn convert(mut data: Arc<DaeModeData>) -> Result<OldSimCode::DaeModeData> {
        let mut oldData: OldSimCode::DaeModeData;
        let mut simEqSystems: Arc<metamodelica::List<Arc<metamodelica::List<Arc<OldSimCode::SimEqSystem>>>>> = metamodelica::nil();
        simEqSystems = SimStrongComponent::Block::convertListList(data.blcks.clone());
        oldData = OldSimCode::DaeModeData { modeCreated: convertMode(data.modeCreated.clone())?, auxiliaryVars: SimVar::convertList(data.auxiliaryVars.clone()), algebraicVars: SimVar::convertList(data.algebraicVars.clone()), residualVars: SimVar::convertList(data.residualVars.clone()), sparsityPattern: Util::applyOption(data.sparsityPattern.clone(), Arc::new(SimJacobian::convert)), daeEquations: simEqSystems.clone() };
        Ok(oldData)
    }

    fn convertMode(mut mode: DaeModeConfig) -> Result<OldSimCode::DaeModeConfig> {
        let mut oldMode: OldSimCode::DaeModeConfig = OldSimCode::DaeModeConfig::ALL_EQUATIONS;
        oldMode = (match mode.clone() {
        DaeModeConfig::ALL => crate::SimCode::DaeModeConfig::ALL_EQUATIONS,
        DaeModeConfig::DYNAMIC => crate::SimCode::DaeModeConfig::DYNAMIC_EQUATIONS,
        _ => bail!("match: no arm matched"),
    });
        Ok(oldMode)
    }

    fn createSparsityJacobian(mut daeModeDataOpt: Option<Arc<DaeModeData>>, mut modelInfo: Arc<ModelInfo::ModelInfo>, mut systems: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>, mut simcode_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>, mut simCodeIndices: SimCodeIndices) -> Result<(Option<Arc<DaeModeData>>, Arc<ModelInfo::ModelInfo>, Arc<SimJacobian::SimJacobian>, Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>, SimCodeIndices)> {
        let mut daeModeDataOpt: Option<Arc<DaeModeData>> = daeModeDataOpt;
        let mut modelInfo: Arc<ModelInfo::ModelInfo> = modelInfo;
        let mut jacobian: Arc<SimJacobian::SimJacobian>;
        let mut simcode_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>> = simcode_map;
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        daeModeDataOpt = (::match_deref::match_deref! { match &(daeModeDataOpt.clone()) {
        Some(daeModeData) => {
            (jacobian, simCodeIndices) = SimJacobian::empty((literal!("A")).clone(), simCodeIndices.clone())?;
            Some(daeModeData.clone())
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimCode.DaeModeData.createSparsityJacobian")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok((daeModeDataOpt, modelInfo, jacobian, simcode_map, simCodeIndices))
    }

    fn rewriteAlgebraicVarsIdx(mut simulationAlgVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>, mut simcode_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>) -> Result<Arc<metamodelica::List<Arc<SimVar::SimVar>>>> {
        let mut daeModeAlgVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut seedCref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        seedCref = ComponentRef::fromNode(Arc::new(InstNode::InstNode::VAR_NODE { name: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(BVariable::SEED_STR)); __mm_s.push_str(&*literal!("_A")); ArcStr::from(__mm_s) }).clone(), varPointer: Pointer::create(BVariable::DUMMY_VARIABLE().clone()) }), Arc::new(openmodelica_nf_frontend::NFType::UNKNOWN), metamodelica::nil(), ComponentRef::Origin::CREF.clone());
        for mut var in &*simulationAlgVars.clone().reverse() {
            let mut var = var.clone();
            cref = ComponentRef::append(var.name.clone(), seedCref.clone())?;
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Searching for: ")); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            assign_field!(var.index = SimVar::getIndex(cref.clone(), simcode_map.clone())?);
            daeModeAlgVars = cons(var.clone(), daeModeAlgVars.clone());
        }
        Ok(daeModeAlgVars)
    }

    fn replaceDerCrefSES(mut sys: Arc<OldSimCode::SimEqSystem>) -> Result<Arc<OldSimCode::SimEqSystem>> {
        let mut sys: Arc<OldSimCode::SimEqSystem> = sys;
        sys = (::match_deref::match_deref! { match &(sys.clone()) {
        qual @ Deref @ OldSimCode::SimEqSystem::SES_RESIDUAL { .. } => {
            let mut qual = (*qual).clone();
            let (qual.exp, _) = OldExpression::traverseExpTopDown(var_field!((*qual).exp, OldSimCode::SimEqSystem::SES_RESIDUAL).clone(), Arc::new(replaceDerCref), 0)?;
            qual.clone()
        },
        qual @ Deref @ OldSimCode::SimEqSystem::SES_SIMPLE_ASSIGN { .. } => {
            let mut qual = (*qual).clone();
            let (qual.exp, _) = OldExpression::traverseExpTopDown(var_field!((*qual).exp, OldSimCode::SimEqSystem::SES_SIMPLE_ASSIGN).clone(), Arc::new(replaceDerCref), 0)?;
            qual.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(sys)
    }

    fn replaceDerCref(mut exp: Arc<DAE::Exp>, mut i: i32) -> Result<(Arc<DAE::Exp>, bool, i32)> {
        let mut exp: Arc<DAE::Exp> = exp;
        let mut b: bool = false;
        let mut i: i32 = i;
        (exp, b) = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cref, ident: Deref @ "$DER", .. }, .. } => {
            (Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("der")).clone() }), expLst: list![Arc::new(DAE::Exp::CREF { componentRef: cref.clone(), ty: ComponentReference::crefTypeFull(cref.clone())? })], attr: DAE::callAttrBuiltinReal.clone() }), false)
        },
        _ => {
            (exp.clone(), true)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok((exp, b, i))
    }

}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum DaeModeConfig {
    ALL = 1,
    DYNAMIC = 2,
}
impl PartialOrd for DaeModeConfig {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for DaeModeConfig {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

