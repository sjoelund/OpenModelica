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
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_types::DAE;
use openmodelica_nf_frontend::NFBuiltinCall as BuiltinCall;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFConvertDAE as ConvertDAE;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFlatten::FunctionTree;
use openmodelica_nf_frontend::NFFlatten;
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
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

impl Default for SimCodeIndices {
    fn default() -> Self {
        Self {
            uniqueIndex: Default::default(),
            realVarIndex: Default::default(),
            integerVarIndex: Default::default(),
            booleanVarIndex: Default::default(),
            stringVarIndex: Default::default(),
            enumerationVarIndex: Default::default(),
            realParamIndex: Default::default(),
            integerParamIndex: Default::default(),
            booleanParamIndex: Default::default(),
            stringParamIndex: Default::default(),
            enumerationParamIndex: Default::default(),
            realAliasIndex: Default::default(),
            integerAliasIndex: Default::default(),
            booleanAliasIndex: Default::default(),
            stringAliasIndex: Default::default(),
            enumerationAliasIndex: Default::default(),
            equationIndex: Default::default(),
            linearSystemIndex: Default::default(),
            nonlinearSystemIndex: Default::default(),
            jacobianIndex: Default::default(),
            residualIndex: Default::default(),
            implicitIndex: Default::default(),
            extObjIndex: Default::default(),
            alias_map: Default::default(),
            generic_call_map: Default::default(),
        }
    }
}

pub type SIM_CODE_INDICES = SimCodeIndices;


pub mod Identifier {
    use super::*;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Identifier {
        pub eqn: Pointer::Pointer<Arc<Equation::Equation>>,
        pub var_cref: Arc<ComponentRef::NFComponentRef>,
        pub resizable: bool,
    }

    impl Default for Identifier {
        fn default() -> Self {
            Self {
                eqn: Default::default(),
                var_cref: Default::default(),
                resizable: Default::default(),
            }
        }
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
    let mut indices: SimCodeIndices = SimCodeIndices { uniqueIndex: 1, realVarIndex: 0, integerVarIndex: 0, booleanVarIndex: 0, stringVarIndex: 0, enumerationVarIndex: 0, realParamIndex: 0, integerParamIndex: 0, booleanParamIndex: 0, stringParamIndex: 0, enumerationParamIndex: 0, realAliasIndex: 0, integerAliasIndex: 0, booleanAliasIndex: 0, stringAliasIndex: 0, enumerationAliasIndex: 0, equationIndex: 1, linearSystemIndex: 0, nonlinearSystemIndex: 0, jacobianIndex: 0, residualIndex: 0, implicitIndex: 0, extObjIndex: 0, alias_map: UnorderedMap::new((std::sync::Arc::new(fnptr!(AliasInfo::hash, Arc<AliasInfo::AliasInfo>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<AliasInfo::AliasInfo>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(AliasInfo::isEqual, Arc<AliasInfo::AliasInfo>, Arc<AliasInfo::AliasInfo>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<AliasInfo::AliasInfo>, Arc<AliasInfo::AliasInfo>) -> Result<bool> + 'static>), 1), generic_call_map: UnorderedMap::new((std::sync::Arc::new(fnptr!(Identifier::hash, Arc<Identifier::Identifier>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Identifier::Identifier>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(Identifier::isEqual, Arc<Identifier::Identifier>, Arc<Identifier::Identifier>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Identifier::Identifier>, Arc<Identifier::Identifier>) -> Result<bool> + 'static>), 1) };
    indices
}

pub mod SimCode {
    use super::*;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

    impl Default for SimCode {
        fn default() -> Self {
            Self {
                modelInfo: Default::default(),
                literals: Default::default(),
                recordDecls: Default::default(),
                externalFunctionIncludes: Default::default(),
                generic_loop_calls: Default::default(),
                independent: Default::default(),
                allSim: Default::default(),
                ode: Default::default(),
                algebraic: Default::default(),
                clockedPartitions: Default::default(),
                nominal: Default::default(),
                min: Default::default(),
                max: Default::default(),
                param: Default::default(),
                no_ret: Default::default(),
                algorithms: Default::default(),
                event_blocks: Default::default(),
                jac_blocks: Default::default(),
                start: Default::default(),
                init: Default::default(),
                init_0: Default::default(),
                init_no_ret: Default::default(),
                discreteVars: Default::default(),
                extObjInfo: Default::default(),
                makefileParams: Default::default(),
                jacobians: Default::default(),
                simulationSettingsOpt: Default::default(),
                fileNamePrefix: Default::default(),
                simcode_map: Default::default(),
                equation_map: Default::default(),
                eventInfo: Default::default(),
                daeModeData: Default::default(),
                inlineEquations: Default::default(),
            }
        }
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
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*List::toString(simCode.literals.clone(), (std::sync::Arc::new(OldExpression::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("  ")).clone(), (literal!("\n  ")).clone(), (literal!("\n\n")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone();
        }
        if !(simCode.generic_loop_calls.clone().is_empty()) {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*StringUtil::headline_3((literal!("Generic Calls")).clone())); ArcStr::from(__mm_s) }).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*List::toString(simCode.generic_loop_calls.clone(), (std::sync::Arc::new(SimGenericCall::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimGenericCall::NSimGenericCall>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("  ")).clone(), (literal!("\n  ")).clone(), (literal!("\n\n")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone();
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

    pub fn create(mut bdae: Arc<BackendDAE::NBackendDAE>, mut name: Arc<Absyn::Path>, mut fileNamePrefix: ArcStr, mut simSettingsOpt: Option<OldSimCode::SimulationSettings>) -> Result<(Arc<SimCode>, Arc<AvlTreePathFunction::Tree>)> {
        type mapExp = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>;

        let mut simCode: Arc<SimCode> = Arc::new(<SimCode as ::std::default::Default>::default());
        let mut oldFunctionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
        simCode = ({
        let mut literals_map: Arc<UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, i32>> = UnorderedMap::new((std::sync::Arc::new(fnptr!(OldExpression::hash, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<i32> + 'static>), (std::sync::Arc::new(OldExpression::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<bool> + 'static>), 1);
        let mut literals_idx: Pointer::Pointer<i32> = Pointer::create(0);
        let mut allSim: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>> = metamodelica::nil();
        let mut event_blocks: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>> = metamodelica::nil();
        (::match_deref::match_deref! { match &(bdae.clone()) {
        Deref @ BackendDAE::MAIN { eqData: eqData @ Deref @ BEquation::EqData::EQ_DATA_SIM { .. }, varData: varData @ Deref @ BVariable::VarData::VAR_DATA_SIM { .. }, .. } => {
            let mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Absyn::Path>, Arc<Function::Function>>>;
            let mut residual_vars: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
            let mut vars: Arc<SimVars::SimVars> = Arc::new(<SimVars::SimVars as ::std::default::Default>::default());
            let mut program: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
            let mut libs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut includeDirs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut libPaths: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut directory: ArcStr = arcstr::literal!("");
            let mut fileName: ArcStr = arcstr::literal!("");
            let mut makefileParams: SimCodeFunction::MakefileParams = <SimCodeFunction::MakefileParams as ::std::default::Default>::default();
            let mut functions: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>> = metamodelica::nil();
            let mut recordDecls: Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>> = metamodelica::nil();
            let mut modelInfo: Arc<ModelInfo::ModelInfo> = Arc::new(<ModelInfo::ModelInfo as ::std::default::Default>::default());
            let mut simCodeIndices: SimCodeIndices = <SimCodeIndices as ::std::default::Default>::default();
            let mut clockedPartitions: Arc<metamodelica::List<Arc<SimPartition::NSimPartition>>> = metamodelica::nil();
            let mut literals: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            let mut externalFunctionIncludes: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut generic_loop_calls: Arc<metamodelica::List<Arc<SimGenericCall::NSimGenericCall>>> = metamodelica::nil();
            let mut independent: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>> = metamodelica::nil();
            let mut nominal: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>> = metamodelica::nil();
            let mut min: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>> = metamodelica::nil();
            let mut max: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>> = metamodelica::nil();
            let mut param: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>> = metamodelica::nil();
            let mut no_ret: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>> = metamodelica::nil();
            let mut event_clocks: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>> = metamodelica::nil();
            let mut algorithms: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>> = metamodelica::nil();
            let mut jac_blocks: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>> = metamodelica::nil();
            let mut init: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>> = metamodelica::nil();
            let mut init_0: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>> = metamodelica::nil();
            let mut init_no_ret: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>> = metamodelica::nil();
            let mut start: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>> = metamodelica::nil();
            let mut ode: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>>> = metamodelica::nil();
            let mut algebraic: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>>> = metamodelica::nil();
            let mut linearLoops: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>> = metamodelica::nil();
            let mut nonlinearLoops: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>> = metamodelica::nil();
            let mut discreteVars: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut extObjInfo: Arc<ExtObjInfo::ExtObjInfo> = Arc::new(<ExtObjInfo::ExtObjInfo as ::std::default::Default>::default());
            let mut jacobians: Arc<metamodelica::List<Arc<SimJacobian::SimJacobian>>> = metamodelica::nil();
            let mut simcode_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>;
            let mut equation_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimStrongComponent::Block::Block>>>;
            let mut daeModeData: Option<Arc<DaeModeData::DaeModeData>> = None;
            let mut jacA: Arc<SimJacobian::SimJacobian> = Arc::new(<SimJacobian::SimJacobian as ::std::default::Default>::default());
            let mut jacB: Arc<SimJacobian::SimJacobian> = Arc::new(<SimJacobian::SimJacobian as ::std::default::Default>::default());
            let mut jacC: Arc<SimJacobian::SimJacobian> = Arc::new(<SimJacobian::SimJacobian as ::std::default::Default>::default());
            let mut jacD: Arc<SimJacobian::SimJacobian> = Arc::new(<SimJacobian::SimJacobian as ::std::default::Default>::default());
            let mut jacF: Arc<SimJacobian::SimJacobian> = Arc::new(<SimJacobian::SimJacobian as ::std::default::Default>::default());
            let mut jacH: Arc<SimJacobian::SimJacobian> = Arc::new(<SimJacobian::SimJacobian as ::std::default::Default>::default());
            let mut jacAdjoint: Arc<SimJacobian::SimJacobian> = Arc::new(<SimJacobian::SimJacobian as ::std::default::Default>::default());
            let mut inlineEquations: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>> = metamodelica::nil();
            let mut collect_literals: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>;
            simCodeIndices = EMPTY_SIM_CODE_INDICES();
            funcMap = BackendDAE::getFunctionMap(bdae.clone())?;
            collect_literals = Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = Arc::new({ let __pe_b1 = literals_map.clone(); let __pe_b2 = literals_idx.clone(); move |__pe_a0| OldExpression::replaceLiteral(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }); move |__pe_a0| Ok(OldExpression::fakeMap(__pe_a0, __pe_b1.clone())) });
            UnorderedMap::apply(funcMap.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = collect_literals.clone(); let __pe_b2: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = collect_literals.clone(); let __pe_b3 = true; let __pe_b4 = true; move |__pe_a0| Function::mapExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }));
            residual_vars = BackendDAE::getLoopResiduals(bdae.clone());
            (vars, simCodeIndices) = SimVars::create(varData.clone(), residual_vars.clone(), simCodeIndices.clone())?;
            (extObjInfo, vars, simCodeIndices) = ExtObjInfo::create(var_field!((**varData).external_objects, VarData::VarData::VAR_DATA_SIM).clone(), vars.clone(), simCodeIndices.clone())?;
            simcode_map = SimCodeUtil::createSimCodeMap(vars.clone(), extObjInfo.clone())?;
            equation_map = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
            independent = metamodelica::nil();
            nominal = metamodelica::nil();
            min = metamodelica::nil();
            max = metamodelica::nil();
            param = metamodelica::nil();
            algorithms = metamodelica::nil();
            (init, simCodeIndices) = SimStrongComponent::Block::createInitialBlocks(var_field!((*bdae).init, BackendDAE::NBackendDAE::MAIN).clone(), simCodeIndices.clone(), simcode_map.clone(), equation_map.clone())?;
            if isSome(var_field!((*bdae).init_0, BackendDAE::NBackendDAE::MAIN).clone()) {
                (init_0, simCodeIndices) = SimStrongComponent::Block::createInitialBlocks(Util::getOption(var_field!((*bdae).init_0, BackendDAE::NBackendDAE::MAIN).clone())?, simCodeIndices.clone(), simcode_map.clone(), equation_map.clone())?;
            } else {
                init_0 = metamodelica::nil();
            }
            (no_ret, simCodeIndices) = SimStrongComponent::Block::createNoReturnBlocks(var_field!((**eqData).removed, EqData::EqData::EQ_DATA_SIM).clone(), simCodeIndices.clone(), Partition::Kind::ODE.clone(), simcode_map.clone(), equation_map.clone())?;
            init_no_ret = metamodelica::nil();
            start = metamodelica::nil();
            discreteVars = metamodelica::nil();
            jacobians = metamodelica::nil();
            if isSome(var_field!((*bdae).dae, BackendDAE::NBackendDAE::MAIN).clone()) {
                ode = metamodelica::nil();
                algebraic = metamodelica::nil();
                (daeModeData, simCodeIndices) = DaeModeData::create(Util::getOption(var_field!((*bdae).dae, BackendDAE::NBackendDAE::MAIN).clone())?, simCodeIndices.clone(), simcode_map.clone(), equation_map.clone())?;
            } else {
                daeModeData = None;
                (ode, allSim, simCodeIndices) = SimStrongComponent::Block::createBlocks(var_field!((*bdae).ode, BackendDAE::NBackendDAE::MAIN).clone(), allSim.clone(), simCodeIndices.clone(), simcode_map.clone(), equation_map.clone())?;
                (algebraic, allSim, simCodeIndices) = SimStrongComponent::Block::createBlocks(var_field!((*bdae).algebraic, BackendDAE::NBackendDAE::MAIN).clone(), allSim.clone(), simCodeIndices.clone(), simcode_map.clone(), equation_map.clone())?;
                (ode, allSim, event_blocks, simCodeIndices) = SimStrongComponent::Block::createDiscreteBlocks(var_field!((*bdae).ode_event, BackendDAE::NBackendDAE::MAIN).clone(), ode.clone(), allSim.clone(), event_blocks.clone(), simCodeIndices.clone(), simcode_map.clone(), equation_map.clone())?;
                (algebraic, allSim, event_blocks, simCodeIndices) = SimStrongComponent::Block::createDiscreteBlocks(var_field!((*bdae).alg_event, BackendDAE::NBackendDAE::MAIN).clone(), algebraic.clone(), allSim.clone(), event_blocks.clone(), simCodeIndices.clone(), simcode_map.clone(), equation_map.clone())?;
            }
            (clockedPartitions, event_clocks, simCodeIndices) = SimStrongComponent::Block::createClockedBlocks(var_field!((*bdae).clocked, BackendDAE::NBackendDAE::MAIN).clone(), simCodeIndices.clone(), simcode_map.clone(), equation_map.clone(), var_field!((*bdae).clockedInfo, BackendDAE::NBackendDAE::MAIN).clone())?;
            if !(no_ret.clone().is_empty()) {
                algebraic = cons(no_ret.clone(), algebraic.clone().reverse()).reverse();
            }
            no_ret = listAppend(event_clocks.clone(), no_ret.clone());
            if !(no_ret.clone().is_empty()) {
                allSim = listAppend(no_ret.clone(), allSim.clone().reverse()).reverse();
            }
            allSim = listAppend(List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>>> = metamodelica::nil();
        for mut blck in (allSim.clone()).into_iter().cloned() {
            let __x = SimStrongComponent::Block::collectEntwinedEquations(blck.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), allSim.clone());
            inlineEquations = metamodelica::nil();
            program = SymbolTable::getAbsyn();
            directory = (CevalScriptBackend::getFileDir(AbsynUtil::pathToCref(name.clone())?, program.clone())?).clone();
            oldFunctionTree = ConvertDAE::convertFunctionTree(NFFlatten::FunctionTreeImpl::fromList(UnorderedMap::toList(funcMap.clone()), (std::sync::Arc::new(fnptr!(NFFlatten::FunctionTreeImpl::addConflictDefault, _, _, _)) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>))?)?;
            (libs, libPaths, externalFunctionIncludes, includeDirs, recordDecls, functions, _) = OldSimCodeUtil::createFunctions(program.clone(), oldFunctionTree.clone())?;
            makefileParams = OldSimCodeFunctionUtil::createMakefileParams(includeDirs.clone(), libs.clone(), libPaths.clone(), false, false)?;
            fileName = (System::basename((AbsynUtil::classFilename(InteractiveUtil::getPathedClassInProgram(name.clone(), program.clone(), false, false)?)?).clone())).clone();
            (linearLoops, nonlinearLoops, jacobians, simCodeIndices) = collectAlgebraicLoops(init.clone(), init_0.clone(), ode.clone(), algebraic.clone(), daeModeData.clone(), simCodeIndices.clone(), simcode_map.clone())?;
            if isSome(daeModeData.clone()) {
                (jacA, jacAdjoint, simCodeIndices) = SimJacobian::createSimulationJacobian(Util::getOption(var_field!((*bdae).dae, BackendDAE::NBackendDAE::MAIN).clone())?, simCodeIndices.clone(), simcode_map.clone())?;
                daeModeData = DaeModeData::addJacobian(daeModeData.clone(), jacA.clone());
            } else {
                (jacA, jacAdjoint, simCodeIndices) = SimJacobian::createSimulationJacobian(listAppend(var_field!((*bdae).ode, BackendDAE::NBackendDAE::MAIN).clone(), var_field!((*bdae).ode_event, BackendDAE::NBackendDAE::MAIN).clone()), simCodeIndices.clone(), simcode_map.clone())?;
            }
            (jacB, simCodeIndices) = SimJacobian::empty((literal!("B")).clone(), simCodeIndices.clone())?;
            (jacC, simCodeIndices) = SimJacobian::empty((literal!("C")).clone(), simCodeIndices.clone())?;
            (jacD, simCodeIndices) = SimJacobian::empty((literal!("D")).clone(), simCodeIndices.clone())?;
            (jacF, simCodeIndices) = SimJacobian::empty((literal!("F")).clone(), simCodeIndices.clone())?;
            (jacH, simCodeIndices) = SimJacobian::empty((literal!("H")).clone(), simCodeIndices.clone())?;
            jacobians = cons(jacAdjoint.clone(), cons(jacH.clone(), cons(jacF.clone(), cons(jacD.clone(), cons(jacC.clone(), cons(jacB.clone(), cons(jacA.clone(), jacobians.clone()))))))).reverse();
            for mut jac in &*jacobians.clone() {
                let mut jac = jac.clone();
                if isSome(jac.jac_map.clone()) {
                    vars = SimVars::addSeedAndJacobianVars(vars.clone(), UnorderedMap::toList(Util::getOption(jac.jac_map.clone())?));
                }
            }
            jac_blocks = SimJacobian::getJacobiansBlocks(list![jacA.clone(), jacB.clone(), jacC.clone(), jacD.clone(), jacF.clone(), jacH.clone(), jacAdjoint.clone()])?;
            (jac_blocks, simCodeIndices) = SimStrongComponent::Block::fixIndices(jac_blocks.clone(), metamodelica::nil(), simCodeIndices.clone())?;
            generic_loop_calls = {
        let mut __acc: Arc<metamodelica::List<Arc<SimGenericCall::NSimGenericCall>>> = metamodelica::nil();
        for mut tpl in (UnorderedMap::toList(simCodeIndices.generic_call_map.clone())).into_iter().cloned() {
            let __x = SimGenericCall::fromIdentifier(tpl.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            generic_loop_calls = {
        let mut __acc: Arc<metamodelica::List<Arc<SimGenericCall::NSimGenericCall>>> = metamodelica::nil();
        for mut call in (generic_loop_calls.clone()).into_iter().cloned() {
            let __x = SimGenericCall::mapShallow(call.clone(), collect_literals.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            literals = UnorderedMap::keyList(literals_map.clone());
            (modelInfo, simCodeIndices) = ModelInfo::create(vars.clone(), name.clone(), (fileName.clone()).clone(), (directory.clone()).clone(), functions.clone(), linearLoops.clone(), nonlinearLoops.clone(), var_field!((*bdae).eventInfo, BackendDAE::NBackendDAE::MAIN).clone(), var_field!((*bdae).clockedInfo, BackendDAE::NBackendDAE::MAIN).clone(), simCodeIndices.clone());
            simCode = Arc::new(SimCode { inlineEquations: inlineEquations.clone(), daeModeData: daeModeData.clone(), eventInfo: var_field!((*bdae).eventInfo, BackendDAE::NBackendDAE::MAIN).clone(), equation_map: equation_map.clone(), simcode_map: simcode_map.clone(), fileNamePrefix: (fileNamePrefix.clone()).clone(), simulationSettingsOpt: simSettingsOpt.clone(), jacobians: jacobians.clone(), makefileParams: makefileParams.clone(), extObjInfo: extObjInfo.clone(), discreteVars: discreteVars.clone(), init_no_ret: init_no_ret.clone(), init_0: init_0.clone(), init: init.clone(), start: start.clone(), jac_blocks: jac_blocks.clone(), event_blocks: event_blocks.clone(), algorithms: algorithms.clone(), no_ret: no_ret.clone(), param: param.clone(), max: max.clone(), min: min.clone(), nominal: nominal.clone(), clockedPartitions: clockedPartitions.clone(), algebraic: algebraic.clone(), ode: ode.clone(), allSim: allSim.clone(), independent: independent.clone(), generic_loop_calls: generic_loop_calls.clone(), externalFunctionIncludes: externalFunctionIncludes.clone(), recordDecls: recordDecls.clone(), literals: literals.clone(), modelInfo: modelInfo.clone() });
            simCode.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimCode.SimCode.create")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
        Ok((simCode, oldFunctionTree))
    }

    pub fn convert(mut simCode: Arc<SimCode>) -> Result<OldSimCode::SimCode> {
        let mut oldSimCode: OldSimCode::SimCode = <OldSimCode::SimCode as ::std::default::Default>::default();
        let mut modelInfo: OldSimCode::ModelInfo = <OldSimCode::ModelInfo as ::std::default::Default>::default();
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
            crefToSimVarHT = List::fold(SimVar::convertList(residualVars.clone()), (std::sync::Arc::new(HashTableCrefSimVar::addSimVarToHashTable) as std::sync::Arc<dyn ::std::ops::Fn(SimCodeVar::SimVar, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, SimCodeVar::SimVar)>>), i32, (HashTableCrefSimVar::FuncHashCref, HashTableCrefSimVar::FuncCrefEqual, HashTableCrefSimVar::FuncCrefStr, HashTableCrefSimVar::FuncExpStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, SimCodeVar::SimVar)>>), i32, (HashTableCrefSimVar::FuncHashCref, HashTableCrefSimVar::FuncCrefEqual, HashTableCrefSimVar::FuncCrefStr, HashTableCrefSimVar::FuncExpStr))> + 'static>), crefToSimVarHT.clone());
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

    impl Default for ModelInfo {
        fn default() -> Self {
            Self {
                name: Default::default(),
                description: Default::default(),
                version: Default::default(),
                author: Default::default(),
                license: Default::default(),
                copyright: Default::default(),
                directory: Default::default(),
                fileName: Default::default(),
                vars: Default::default(),
                varInfo: Default::default(),
                functions: Default::default(),
                labels: Default::default(),
                resourcePaths: Default::default(),
                sortedClasses: Default::default(),
                nClocks: Default::default(),
                nSubClocks: Default::default(),
                nSpatialDistributions: Default::default(),
                hasLargeLinearEquationSystems: Default::default(),
                linearLoops: Default::default(),
                nonlinearLoops: Default::default(),
            }
        }
    }

    pub type MODEL_INFO = ModelInfo;

    pub fn toString(mut modelInfo: Arc<ModelInfo>) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = (SimVars::toString(modelInfo.vars.clone(), (literal!("")).clone())?).clone();
        Ok(r#str)
    }

    pub fn create(mut vars: Arc<SimVars::SimVars>, mut name: Arc<Absyn::Path>, mut fileName: ArcStr, mut directory: ArcStr, mut functions: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>>, mut linearLoops: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>, mut nonlinearLoops: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>, mut eventInfo: Arc<EventInfo::EventInfo>, mut clockedInfo: Arc<ClockedInfo::ClockedInfo>, mut simCodeIndices: SimCodeIndices) -> (Arc<ModelInfo>, SimCodeIndices) {
        let mut modelInfo: Arc<ModelInfo> = Arc::new(<ModelInfo as ::std::default::Default>::default());
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut info: Arc<VarInfo::VarInfo> = Arc::new(<VarInfo::VarInfo as ::std::default::Default>::default());
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
        let mut oldModelInfo: OldSimCode::ModelInfo = <OldSimCode::ModelInfo as ::std::default::Default>::default();
        let mut varInfo: OldSimCode::VarInfo = <OldSimCode::VarInfo as ::std::default::Default>::default();
        varInfo = VarInfo::convert(modelInfo.varInfo.clone());
        oldModelInfo = OldSimCode::ModelInfo { unitDefinitions: metamodelica::nil(), nonLinearSystems: SimStrongComponent::Block::convertList(modelInfo.nonlinearLoops.clone()), linearSystems: SimStrongComponent::Block::convertList(modelInfo.linearLoops.clone()), hasLargeLinearEquationSystems: modelInfo.hasLargeLinearEquationSystems.clone(), nSpatialDistributions: modelInfo.nSpatialDistributions.clone(), nSubClocks: modelInfo.nSubClocks.clone(), nClocks: modelInfo.nClocks.clone(), sortedClasses: modelInfo.sortedClasses.clone(), resourcePaths: modelInfo.resourcePaths.clone(), labels: modelInfo.labels.clone(), functions: modelInfo.functions.clone(), vars: SimVars::convert(modelInfo.vars.clone()), varInfo: VarInfo::convert(modelInfo.varInfo.clone()), fileName: (modelInfo.fileName.clone()).clone(), directory: (modelInfo.directory.clone()).clone(), copyright: (modelInfo.copyright.clone()).clone(), license: (modelInfo.license.clone()).clone(), author: (modelInfo.author.clone()).clone(), version: (modelInfo.version.clone()).clone(), description: (modelInfo.description.clone()).clone(), name: modelInfo.name.clone() };
        oldModelInfo
    }

}

pub mod DaeModeData {
    use super::*;
    /// contains data that belongs to the dae mode
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

    impl Default for DaeModeData {
        fn default() -> Self {
            Self {
                blcks: Default::default(),
                sparsityPattern: Default::default(),
                residualVars: Default::default(),
                algebraicVars: Default::default(),
                auxiliaryVars: Default::default(),
                modeCreated: Default::default(),
            }
        }
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
        let mut oldData: OldSimCode::DaeModeData = <OldSimCode::DaeModeData as ::std::default::Default>::default();
        let mut simEqSystems: Arc<metamodelica::List<Arc<metamodelica::List<Arc<OldSimCode::SimEqSystem>>>>> = metamodelica::nil();
        simEqSystems = SimStrongComponent::Block::convertListList(data.blcks.clone());
        oldData = OldSimCode::DaeModeData { modeCreated: convertMode(data.modeCreated.clone())?, auxiliaryVars: SimVar::convertList(data.auxiliaryVars.clone()), algebraicVars: SimVar::convertList(data.algebraicVars.clone()), residualVars: SimVar::convertList(data.residualVars.clone()), sparsityPattern: Util::applyOption(data.sparsityPattern.clone(), (std::sync::Arc::new(SimJacobian::convert) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimJacobian::SimJacobian>) -> Result<Arc<OldSimCode::JacobianMatrix>> + 'static>)), daeEquations: simEqSystems.clone() };
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
        let mut jacobian: Arc<SimJacobian::SimJacobian> = Arc::new(<SimJacobian::SimJacobian as ::std::default::Default>::default());
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
            let (qual.exp, _) = OldExpression::traverseExpTopDown(var_field!((*qual).exp, OldSimCode::SimEqSystem::SES_RESIDUAL).clone(), (std::sync::Arc::new(replaceDerCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, i32) -> Result<(Arc<DAE::Exp>, bool, i32)> + 'static>), 0)?;
            qual.clone()
        },
        qual @ Deref @ OldSimCode::SimEqSystem::SES_SIMPLE_ASSIGN { .. } => {
            let mut qual = (*qual).clone();
            let (qual.exp, _) = OldExpression::traverseExpTopDown(var_field!((*qual).exp, OldSimCode::SimEqSystem::SES_SIMPLE_ASSIGN).clone(), (std::sync::Arc::new(replaceDerCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, i32) -> Result<(Arc<DAE::Exp>, bool, i32)> + 'static>), 0)?;
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
            (Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("der")).clone() }), expLst: list![Arc::new(DAE::Exp::CREF { componentRef: cref.clone(), ty: ComponentReference::crefTypeFull(cref.clone())? })], attr: DAE::callAttrBuiltinReal().clone() }), false)
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
impl Default for DaeModeConfig {
    fn default() -> Self { Self::ALL }
}

