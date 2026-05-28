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

use crate::NBAlias as Alias;
use crate::NBBindings as Bindings;
use crate::NBCausalize as Causalize;
use crate::NBDAEMode as DAEMode;
use crate::NBDetectStates as DetectStates;
use crate::NBDifferentiate as Differentiate;
use crate::NBEquation as BEquation;
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationAttributes;
use crate::NBEquation::EquationKind;
use crate::NBEquation::EquationPointer;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::IfEquationBody;
use crate::NBEquation::Iterator;
use crate::NBEvaluation as Evaluation;
use crate::NBEvents as Events;
use crate::NBFunctionAlias as FunctionAlias;
use crate::NBInitialization as Initialization;
use crate::NBInline as Inline;
use crate::NBJacobian as Jacobian;
use crate::NBJacobian::JacobianType;
use crate::NBJacobian::SparsityColoring;
use crate::NBJacobian::SparsityPattern;
use crate::NBModule as Module;
use crate::NBPartition::Partition;
use crate::NBPartition;
use crate::NBPartitioning as Partitioning;
use crate::NBResizable as Resizable;
use crate::NBSolve as Solve;
use crate::NBStrongComponent as StrongComponent;
use crate::NBStrongComponent::CountCollector;
use crate::NBTearing as Tearing;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointer;
use crate::NBVariable::VariablePointers;
use openmodelica_ast::Absyn::Path;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_nf_frontend::NFAlgorithm as Algorithm;
use openmodelica_nf_frontend::NFBackendExtension::Annotations;
use openmodelica_nf_frontend::NFBackendExtension::BackendInfo;
use openmodelica_nf_frontend::NFBackendExtension::VariableAttributes;
use openmodelica_nf_frontend::NFBackendExtension::VariableKind;
use openmodelica_nf_frontend::NFBackendExtension;
use openmodelica_nf_frontend::NFBinding as Binding;
use openmodelica_nf_frontend::NFBuiltinFuncs;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFClass as Class;
use openmodelica_nf_frontend::NFComplexType as ComplexType;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFConvertDAE as ConvertDAE;
use openmodelica_nf_frontend::NFDimension as Dimension;
use openmodelica_nf_frontend::NFEquation as FEquation;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFlatModel as FlatModel;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFInstNode::InstNode;
use openmodelica_nf_frontend::NFPrefixes as Prefixes;
use openmodelica_nf_frontend::NFSimplifyExp as SimplifyExp;
use openmodelica_nf_frontend::NFStatement as Statement;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::BaseHashTable;
use openmodelica_util::ClockIndexes;
use openmodelica_util::Error;
use openmodelica_util::ExecStat;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

/// file:        NBackendDAE.mo
/// package:     NBackendDAE
/// description: This file contains the main data type for the backend containing
///              all data. It further contains the lower and solve main function.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NBackendDAE {
    MAIN {
        /// Partitions for differential-algebraic equations
        ode: Arc<metamodelica::List<Arc<Partition::Partition>>>,
        /// Partitions for algebraic equations
        algebraic: Arc<metamodelica::List<Arc<Partition::Partition>>>,
        /// Partitions for differential-algebraic event iteration
        ode_event: Arc<metamodelica::List<Arc<Partition::Partition>>>,
        /// Partitions for algebraic event iteration
        alg_event: Arc<metamodelica::List<Arc<Partition::Partition>>>,
        /// Clocked Partitions
        clocked: Arc<metamodelica::List<Arc<Partition::Partition>>>,
        /// Partitions for initialization
        init: Arc<metamodelica::List<Arc<Partition::Partition>>>,
        /// Partitions for initialization with lambda = 0 (homotopy)
        init_0: Option<Arc<metamodelica::List<Arc<Partition::Partition>>>>,
        /// Partitions for dae mode
        dae: Option<Arc<metamodelica::List<Arc<Partition::Partition>>>>,
        /// Variable data
        varData: Arc<VarData::VarData>,
        /// Equation data
        eqData: Arc<EqData::EqData>,
        /// contains time and state events
        eventInfo: Arc<Events::EventInfo::EventInfo>,
        /// contains information about clocked partitions
        clockedInfo: Arc<Partitioning::ClockedInfo::ClockedInfo>,
        /// Function bodies
        funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>,
    },
    JACOBIAN {
        /// unique matrix name
        name: ArcStr,
        /// type of jacobian
        jacType: JacobianType,
        /// Variable data
        varData: Arc<VarData::VarData>,
        /// the sorted equations
        comps: metamodelica::Array<Arc<StrongComponent::NBStrongComponent>>,
        /// Sparsity pattern for the jacobian
        sparsityPattern: Arc<SparsityPattern::SparsityPattern>,
        /// Coloring information
        sparsityColoring: Arc<SparsityColoring::SparsityColoring>,
        /// is this an adjoint jacobian?
        isAdjoint: bool,
    },
    HESSIAN {
        /// Variable data
        varData: Arc<VarData::VarData>,
        /// Equation data
        eqData: Arc<EqData::EqData>,
    },
}
impl Default for NBackendDAE {
    fn default() -> Self {
        Self::HESSIAN {
            varData: Default::default(),
            eqData: Default::default(),
        }
    }
}
pub use self::NBackendDAE::{MAIN,JACOBIAN,HESSIAN};
pub fn toString(mut bdae: Arc<NBackendDAE>, mut r#str: ArcStr) -> Result<ArcStr> {
    let mut r#str: ArcStr = r#str;
    r#str = (({
        let mut tmp: ArcStr = literal!("");
        (::match_deref::match_deref! { match &(bdae.clone()) {
        Deref @ MAIN { .. } => {
            if !(Flags::isSet(Flags::BLT_DUMP.clone())?) || var_field!((*bdae).ode, NBackendDAE::MAIN).clone().is_empty() && var_field!((*bdae).algebraic, NBackendDAE::MAIN).clone().is_empty() && var_field!((*bdae).ode_event, NBackendDAE::MAIN).clone().is_empty() && var_field!((*bdae).alg_event, NBackendDAE::MAIN).clone().is_empty() && var_field!((*bdae).clocked, NBackendDAE::MAIN).clone().is_empty() && var_field!((*bdae).init, NBackendDAE::MAIN).clone().is_empty() && isNone(var_field!((*bdae).dae, NBackendDAE::MAIN).clone()) {
                tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_1(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAE: ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*BVariable::VarData::toString(var_field!((*bdae).varData, NBackendDAE::MAIN).clone(), 2)?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*BEquation::EqData::toString(var_field!((*bdae).eqData, NBackendDAE::MAIN).clone(), 1, None)?); ArcStr::from(__mm_s) }).clone();
            } else {
                tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*NBPartition::Partition::toStringList(var_field!((*bdae).ode, NBackendDAE::MAIN).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[ODE] Differential-Algebraic: ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone())?); ArcStr::from(__mm_s) }).clone();
                tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*NBPartition::Partition::toStringList(var_field!((*bdae).algebraic, NBackendDAE::MAIN).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[ALG] Algebraic: ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone())?); ArcStr::from(__mm_s) }).clone();
                tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*NBPartition::Partition::toStringList(var_field!((*bdae).ode_event, NBackendDAE::MAIN).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[ODE_EVENT] Event Handling: ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone())?); ArcStr::from(__mm_s) }).clone();
                tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*NBPartition::Partition::toStringList(var_field!((*bdae).alg_event, NBackendDAE::MAIN).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[ALG_EVENT] Event Handling: ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone())?); ArcStr::from(__mm_s) }).clone();
                tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*NBPartition::Partition::toStringList(var_field!((*bdae).clocked, NBackendDAE::MAIN).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[CLOCKED] Event Handling: ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone())?); ArcStr::from(__mm_s) }).clone();
                tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*NBPartition::Partition::toStringList(var_field!((*bdae).init, NBackendDAE::MAIN).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[INI] Initialization: ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone())?); ArcStr::from(__mm_s) }).clone();
                if isSome(var_field!((*bdae).init_0, NBackendDAE::MAIN).clone()) {
                    tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*NBPartition::Partition::toStringList(Util::getOption(var_field!((*bdae).init_0, NBackendDAE::MAIN).clone())?, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[INI_0] Initialization Lambda=0: ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone())?); ArcStr::from(__mm_s) }).clone();
                }
                if isSome(var_field!((*bdae).dae, NBackendDAE::MAIN).clone()) {
                    tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*NBPartition::Partition::toStringList(Util::getOption(var_field!((*bdae).dae, NBackendDAE::MAIN).clone())?, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[DAE] DAEMode: ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone())?); ArcStr::from(__mm_s) }).clone();
                }
            }
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*Events::EventInfo::toString(var_field!((*bdae).eventInfo, NBackendDAE::MAIN).clone())?); ArcStr::from(__mm_s) }).clone();
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*Partitioning::ClockedInfo::toString(var_field!((*bdae).clockedInfo, NBackendDAE::MAIN).clone())?); ArcStr::from(__mm_s) }).clone();
            tmp.clone()
        },
        Deref @ JACOBIAN { .. } => {
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_1(({ let mut __mm_s = String::new(); __mm_s.push_str(&*Jacobian::jacobianTypeString(var_field!((*bdae).jacType, NBackendDAE::JACOBIAN).clone())); __mm_s.push_str(&*literal!(" Jacobian ")); __mm_s.push_str(&*var_field!((*bdae).name, NBackendDAE::JACOBIAN).clone()); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*BVariable::VarData::toString(var_field!((*bdae).varData, NBackendDAE::JACOBIAN).clone(), 1)?); ArcStr::from(__mm_s) }).clone();
            let __range0 = 1..=(var_field!((*bdae).comps, NBackendDAE::JACOBIAN).clone().borrow().len() as i32);
            for mut i in __range0 {
                tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*StrongComponent::toString(var_field!((*bdae).comps, NBackendDAE::JACOBIAN).borrow()[(i.clone()-1) as usize].clone(), i.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            }
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*Jacobian::SparsityPattern::toString(var_field!((*bdae).sparsityPattern, NBackendDAE::JACOBIAN).clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*Jacobian::SparsityColoring::toString(var_field!((*bdae).sparsityColoring, NBackendDAE::JACOBIAN).clone())); ArcStr::from(__mm_s) }).clone();
            tmp.clone()
        },
        Deref @ HESSIAN { .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_1(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Hessian: ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone())); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*BVariable::VarData::toString(var_field!((*bdae).varData, NBackendDAE::HESSIAN).clone(), 1)?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*BEquation::EqData::toString(var_field!((*bdae).eqData, NBackendDAE::HESSIAN).clone(), 1, None)?); ArcStr::from(__mm_s) }
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.toString")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    })).clone();
    Ok(r#str)
}

pub fn getVarData(mut bdae: Arc<NBackendDAE>) -> Result<Arc<VarData::VarData>> {
    let mut varData: Arc<VarData::VarData> = Arc::new(VarData::VAR_DATA_EMPTY);
    varData = (::match_deref::match_deref! { match &(bdae.clone()) {
        Deref @ MAIN { .. } => var_field!((*bdae).varData, NBackendDAE::MAIN).clone(),
        Deref @ JACOBIAN { .. } => var_field!((*bdae).varData, NBackendDAE::JACOBIAN).clone(),
        Deref @ HESSIAN { .. } => var_field!((*bdae).varData, NBackendDAE::HESSIAN).clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.getVarData")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(varData)
}

pub fn setVarData(mut bdae: Arc<NBackendDAE>, mut varData: Arc<VarData::VarData>) -> Result<Arc<NBackendDAE>> {
    let mut bdae: Arc<NBackendDAE> = bdae;
    bdae = (::match_deref::match_deref! { match &(bdae.clone()) {
        Deref @ MAIN { .. } => {
            assign_variant_field!(bdae => NBackendDAE::MAIN; varData = varData.clone());
            bdae.clone()
        },
        Deref @ JACOBIAN { .. } => {
            assign_variant_field!(bdae => NBackendDAE::JACOBIAN; varData = varData.clone());
            bdae.clone()
        },
        Deref @ HESSIAN { .. } => {
            assign_variant_field!(bdae => NBackendDAE::HESSIAN; varData = varData.clone());
            bdae.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.setVarData")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(bdae)
}

pub fn getIsAdjoint(mut bdae: Arc<NBackendDAE>) -> Result<bool> {
    let mut isAdjoint: bool = false;
    isAdjoint = (::match_deref::match_deref! { match &(bdae.clone()) {
        Deref @ JACOBIAN { isAdjoint, .. } => isAdjoint.clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.getIsAdjoint")); __mm_s.push_str(&*literal!(" failed! Only the record type JACOBIAN() has a jacobian.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isAdjoint)
}

pub fn getFunctionMap(mut bdae: Arc<NBackendDAE>) -> Result<Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>> {
    let mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>;
    funcMap = (::match_deref::match_deref! { match &(bdae.clone()) {
        Deref @ MAIN { .. } => var_field!((*bdae).funcMap, NBackendDAE::MAIN).clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.getFunctionMap")); __mm_s.push_str(&*literal!(" failed! Only the record type MAIN() has a function map.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(funcMap)
}

pub fn sizes(mut bdae: Arc<NBackendDAE>) -> Result<((i32, i32), (i32, i32))> {
    let mut varSizes: (i32, i32);
    let mut eqnSizes: (i32, i32);
    (varSizes, eqnSizes) = (::match_deref::match_deref! { match &(bdae.clone()) {
        Deref @ MAIN { .. } => ((BVariable::VarData::scalarSize(var_field!((*bdae).varData, NBackendDAE::MAIN).clone(), true)?, BVariable::VarData::size(var_field!((*bdae).varData, NBackendDAE::MAIN).clone())?), (BEquation::EqData::scalarSize(var_field!((*bdae).eqData, NBackendDAE::MAIN).clone(), true)?, BEquation::EqData::size(var_field!((*bdae).eqData, NBackendDAE::MAIN).clone())?)),
        _ => ((0, 0), (0, 0)),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((varSizes, eqnSizes))
}

pub fn lower(mut flatModel: Arc<FlatModel::NFFlatModel>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>) -> Result<Arc<NBackendDAE>> {
    let mut bdae: Arc<NBackendDAE>;
    let mut variableData: Arc<VarData::VarData> = Arc::new(VarData::VAR_DATA_EMPTY);
    let mut equationData: Arc<EqData::EqData> = Arc::new(EqData::EQ_DATA_EMPTY);
    let mut eventInfo: Arc<Events::EventInfo::EventInfo> = Events::EventInfo::empty();
    let mut clockedInfo: Arc<Partitioning::ClockedInfo::ClockedInfo> = Partitioning::ClockedInfo::new();
    variableData = lowerVariableData(flatModel.variables.clone())?;
    (equationData, variableData) = lowerEquationData(flatModel.equations.clone(), flatModel.algorithms.clone(), flatModel.initialEquations.clone(), flatModel.initialAlgorithms.clone(), variableData.clone())?;
    bdae = Arc::new(NBackendDAE::MAIN { ode: metamodelica::nil(), algebraic: metamodelica::nil(), ode_event: metamodelica::nil(), alg_event: metamodelica::nil(), clocked: metamodelica::nil(), init: metamodelica::nil(), init_0: None, dae: None, varData: variableData.clone(), eqData: equationData.clone(), eventInfo: eventInfo.clone(), clockedInfo: clockedInfo.clone(), funcMap: lowerFunctions(funcMap.clone()) });
    Ok(bdae)
}

pub fn main(mut bdae: Arc<NBackendDAE>) -> Result<Arc<NBackendDAE>> {
    let mut bdae: Arc<NBackendDAE> = bdae;
    let mut preOptModules: Arc<metamodelica::List<(Module::wrapper, ArcStr)>> = metamodelica::nil();
    let mut mainModules: Arc<metamodelica::List<(Module::wrapper, ArcStr)>> = metamodelica::nil();
    let mut postOptModules: Arc<metamodelica::List<(Module::wrapper, ArcStr)>> = metamodelica::nil();
    let mut preOptClocks: Arc<metamodelica::List<(ArcStr, metamodelica::Real)>> = metamodelica::nil();
    let mut mainClocks: Arc<metamodelica::List<(ArcStr, metamodelica::Real)>> = metamodelica::nil();
    let mut postOptClocks: Arc<metamodelica::List<(ArcStr, metamodelica::Real)>> = metamodelica::nil();
    let mut followEquations: Arc<metamodelica::List<ArcStr>> = Flags::getConfigStringList(Flags::DEBUG_FOLLOW_EQUATIONS.clone())?;
    let mut eq_filter_opt: Option<Arc<UnorderedSet::UnorderedSet<ArcStr>>> = None;
    let mut inline_types: Arc<metamodelica::List<DAE::InlineType>> = list![openmodelica_frontend_types::DAE::InlineType::NORM_INLINE, openmodelica_frontend_types::DAE::InlineType::BUILTIN_EARLY_INLINE, openmodelica_frontend_types::DAE::InlineType::EARLY_INLINE, openmodelica_frontend_types::DAE::InlineType::DEFAULT_INLINE];
    let mut kind: NBPartition::Kind = NBPartition::Kind::ODE;
    if followEquations.clone().is_empty() {
        eq_filter_opt = None;
    } else {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*List::toString(followEquations.clone(), std::sync::Arc::new(fnptr!(Util::id, _)), (literal!("[debugFilterEquations] filtering for equations: ")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
        eq_filter_opt = Some(UnorderedSet::fromList(followEquations.clone(), (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(stringEqual, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?);
    }
    if Flags::getConfigBool(Flags::DAE_MODE.clone())? {
        mainModules = list![(DAEMode::main.clone(), literal!("DAE-Mode"))];
        kind = NBPartition::Kind::DAE.clone();
    } else {
        mainModules = metamodelica::nil();
        kind = NBPartition::Kind::ODE.clone();
    }
    preOptModules = list![(Bindings::main.clone(), literal!("Bindings")), (todo!("PARTEVALFUNCTION of FunctionAlias.main: function signature not resolved"), literal!("FunctionAlias")), (todo!("PARTEVALFUNCTION of Inline.main: function signature not resolved"), literal!("Early Inline")), ({ let __pe_b1 = false; move |__pe_a0| simplify(__pe_a0, __pe_b1.clone()) }, literal!("Simplify 1")), (todo!("PARTEVALFUNCTION of Alias.main: function signature not resolved"), literal!("Alias")), ({ let __pe_b1 = false; move |__pe_a0| simplify(__pe_a0, __pe_b1.clone()) }, literal!("Simplify 2")), ((std::sync::Arc::new(removeStream) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NBackendDAE>) -> Result<Arc<NBackendDAE>> + 'static>), literal!("Remove Stream")), (DetectStates::main.clone(), literal!("Detect States")), (Events::main.clone(), literal!("Events"))];
    mainModules = listAppend(list![(todo!("PARTEVALFUNCTION of Partitioning.main: function signature not resolved"), literal!("Partitioning")), (todo!("PARTEVALFUNCTION of Causalize.main: function signature not resolved"), literal!("Causalize")), (todo!("PARTEVALFUNCTION of Inline.main: function signature not resolved"), literal!("After Index Reduction Inline")), (Initialization::main.clone(), literal!("Initialization"))], mainModules.clone());
    postOptModules = list![((std::sync::Arc::new(fnptr!(Evaluation::removeDummies, Arc<NBackendDAE>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NBackendDAE>) -> Result<Arc<NBackendDAE>> + 'static>), literal!("Remove Dummies")), (todo!("PARTEVALFUNCTION of Tearing.main: function signature not resolved"), literal!("Tearing")), (Partitioning::categorize.clone(), literal!("Categorize")), (Solve::main.clone(), literal!("Solve")), (todo!("PARTEVALFUNCTION of Jacobian.main: function signature not resolved"), literal!("Jacobian")), (Initialization::minimizeHomotopySystem.clone(), literal!("Minimize Homotopy System"))];
    (bdae, preOptClocks) = applyModules(bdae.clone(), preOptModules.clone(), eq_filter_opt.clone(), ClockIndexes::RT_CLOCK_NEW_BACKEND_MODULE.clone())?;
    (bdae, mainClocks) = applyModules(bdae.clone(), mainModules.clone(), eq_filter_opt.clone(), ClockIndexes::RT_CLOCK_NEW_BACKEND_MODULE.clone())?;
    (bdae, postOptClocks) = applyModules(bdae.clone(), postOptModules.clone(), eq_filter_opt.clone(), ClockIndexes::RT_CLOCK_NEW_BACKEND_MODULE.clone())?;
    if Flags::isSet(Flags::DUMP_BACKEND_CLOCKS.clone())? {
        if !(preOptClocks.clone().is_empty()) {
            println!("{}", (StringUtil::headline_4((literal!("Pre-Opt Backend Clocks:")).clone())).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut clck in (preOptClocks.clone()).into_iter().cloned() {
            let __x = Module::moduleClockString(clck.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        if !(mainClocks.clone().is_empty()) {
            println!("{}", (StringUtil::headline_4((literal!("Main Backend Clocks:")).clone())).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut clck in (mainClocks.clone()).into_iter().cloned() {
            let __x = Module::moduleClockString(clck.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        if !(postOptClocks.clone().is_empty()) {
            println!("{}", (StringUtil::headline_4((literal!("Post-Opt Backend Clocks:")).clone())).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut clck in (postOptClocks.clone()).into_iter().cloned() {
            let __x = Module::moduleClockString(clck.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
        }
    }
    backenddaeinfo(bdae.clone())?;
    Ok(bdae)
}

pub fn applyModules(mut bdae: Arc<NBackendDAE>, mut modules: Arc<metamodelica::List<(Module::wrapper, ArcStr)>>, mut eq_filter_opt: Option<Arc<UnorderedSet::UnorderedSet<ArcStr>>>, mut clock_idx: i32) -> Result<(Arc<NBackendDAE>, Arc<metamodelica::List<(ArcStr, metamodelica::Real)>>)> {
    let mut bdae: Arc<NBackendDAE> = bdae;
    let mut module_clocks: Arc<metamodelica::List<(ArcStr, metamodelica::Real)>> = metamodelica::nil();
    let mut func: Module::wrapper;
    let mut name: ArcStr = arcstr::literal!("");
    let mut debugStr: ArcStr = arcstr::literal!("");
    let mut clock_time: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut length: i32 = 0;
    let mut varSizes: (i32, i32);
    let mut eqnSizes: (i32, i32);
    for mut module in &*modules.clone() {
        let mut module = module.clone();
        (func, name) = module.clone();
        if Flags::isSet(Flags::FAILTRACE.clone())? {
            debugStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[failtrace] ........ [")); __mm_s.push_str(&*ClockIndexes::toString(clock_idx.clone())); __mm_s.push_str(&*literal!("] ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone();
            debugStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*debugStr.clone()); __mm_s.push_str(&*StringUtil::repeat((literal!(".")).clone(), intMax(60 - ((debugStr.clone()).clone().len() as i32), 0))); ArcStr::from(__mm_s) }).clone();
        }
        if clock_idx.clone() != -1 {
            System::realtimeClear(clock_idx.clone())?;
            System::realtimeTick(clock_idx.clone())?;
            if let Ok(__iflet0) = func(bdae.clone()) {
                bdae = __iflet0;
            } else {
                if Flags::isSet(Flags::FAILTRACE.clone())? {
                    debugStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*debugStr.clone()); __mm_s.push_str(&*literal!(" failed\n")); ArcStr::from(__mm_s) }).clone();
                    println!("{}", (debugStr.clone()).clone());
                }
                bail!("fail");
            }
            clock_time = System::realtimeTock(clock_idx.clone())?;
            ExecStat::execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*ClockIndexes::toString(clock_idx.clone())); __mm_s.push_str(&*literal!("] ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone())?;
            module_clocks = cons((name.clone(), clock_time.clone()), module_clocks.clone());
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                (varSizes, eqnSizes) = sizes(bdae.clone())?;
                debugStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*debugStr.clone()); __mm_s.push_str(&*literal!(" V(")); __mm_s.push_str(&*intString(Util::tuple21(varSizes.clone()))); __mm_s.push_str(&*literal!("|")); __mm_s.push_str(&*intString(Util::tuple22(varSizes.clone()))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
                debugStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*debugStr.clone()); __mm_s.push_str(&*literal!(" E(")); __mm_s.push_str(&*intString(Util::tuple21(eqnSizes.clone()))); __mm_s.push_str(&*literal!("|")); __mm_s.push_str(&*intString(Util::tuple22(eqnSizes.clone()))); __mm_s.push_str(&*literal!(") ")); ArcStr::from(__mm_s) }).clone();
                if Util::tuple21(varSizes.clone()) != Util::tuple21(eqnSizes.clone()) {
                    debugStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*debugStr.clone()); __mm_s.push_str(&*literal!("XX ")); ArcStr::from(__mm_s) }).clone();
                }
                debugStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*debugStr.clone()); __mm_s.push_str(&*StringUtil::repeat((literal!(".")).clone(), intMax(100 - ((debugStr.clone()).clone().len() as i32), 0))); ArcStr::from(__mm_s) }).clone();
                debugStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*debugStr.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*realString(clock_time.clone())); __mm_s.push_str(&*literal!("s\n")); ArcStr::from(__mm_s) }).clone();
                println!("{}", (debugStr.clone()).clone());
                debugLowering(bdae.clone())?;
            }
        } else {
            bdae = func(bdae.clone())?;
        }
        if Flags::isSet(Flags::OPT_DAE_DUMP.clone())? || Flags::isSet(Flags::BLT_DUMP.clone())? && (name.clone() == literal!("Causalize") || name.clone() == literal!("Solve")) {
            println!("{}", (toString(bdae.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?).clone());
        }
        if Util::isSome(eq_filter_opt.clone()) {
            debugFollowEquations(bdae.clone(), eq_filter_opt.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
        }
    }
    module_clocks = module_clocks.clone().reverse();
    Ok((bdae, module_clocks))
}

pub fn simplify(mut bdae: Arc<NBackendDAE>, mut init: bool) -> Result<Arc<NBackendDAE>> {
    let mut bdae: Arc<NBackendDAE> = bdae;
    let mut acc_discrete_states: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>> = Pointer::create(metamodelica::nil());
    let mut acc_previous: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>> = Pointer::create(metamodelica::nil());
    let mut func: BEquation::MapFuncEqn = { let __pe_b1 = (literal!("NBackendDAE.simplify")).clone(); let __pe_b2 = (literal!("")).clone(); let __pe_b3 = acc_discrete_states.clone(); let __pe_b4 = acc_previous.clone(); let __pe_b5: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = Arc::new({ let __pe_b1 = true; let __pe_b2 = (literal!("NBackendDAE.simplify")).clone(); let __pe_b3 = (literal!("")).clone(); move |__pe_a0| SimplifyExp::simplifyDump(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }); move |__pe_a0| Equation::simplify(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone()) };
    bdae = (::match_deref::match_deref! { match &(bdae.clone()) {
        Deref @ MAIN { eqData: eqData @ Deref @ BEquation::EqData::EQ_DATA_SIM { .. }, .. } => {
            let mut varData: Arc<VarData::VarData> = Arc::new(VarData::VAR_DATA_EMPTY);
            let mut acc_discrete_states_accessed: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
            let mut eqData = (*eqData).clone();
            if init.clone() {
                assign_variant_field!(eqData => EqData::EqData::EQ_DATA_SIM; initials = BEquation::EquationPointers::map(var_field!((*eqData).initials, EqData::EqData::EQ_DATA_SIM).clone(), func.clone())?);
            } else {
                assign_variant_field!(eqData => EqData::EqData::EQ_DATA_SIM; equations = BEquation::EquationPointers::map(var_field!((*eqData).equations, EqData::EqData::EQ_DATA_SIM).clone(), func.clone())?);
            }
            assign_variant_field!(bdae => NBackendDAE::MAIN;
                eqData = BEquation::EqData::compress(eqData.clone())?,
                varData = (::match_deref::match_deref! { match &(var_field!((*bdae).varData, NBackendDAE::MAIN).clone()) {
        varData @ Deref @ BVariable::VarData::VAR_DATA_SIM { .. } => {
            acc_discrete_states_accessed = Pointer::access(acc_discrete_states.clone());
            BVariable::VariablePointers::removeList(acc_discrete_states_accessed.clone(), var_field!((**varData).unknowns, VarData::VarData::VAR_DATA_SIM).clone())?;
            BVariable::VariablePointers::removeList(acc_discrete_states_accessed.clone(), var_field!((**varData).discretes, VarData::VarData::VAR_DATA_SIM).clone())?;
            BVariable::VariablePointers::removeList(acc_discrete_states_accessed.clone(), var_field!((**varData).discrete_states, VarData::VarData::VAR_DATA_SIM).clone())?;
            BVariable::VariablePointers::removeList(Pointer::access(acc_previous.clone()), var_field!((**varData).previous, VarData::VarData::VAR_DATA_SIM).clone())?;
            BVariable::VariablePointers::removeList(Pointer::access(acc_previous.clone()), var_field!((**varData).variables, VarData::VarData::VAR_DATA_SIM).clone())?;
            BVariable::VariablePointers::addList(acc_discrete_states_accessed.clone(), var_field!((**varData).parameters, VarData::VarData::VAR_DATA_SIM).clone());
            BVariable::VariablePointers::addList(acc_discrete_states_accessed.clone(), var_field!((**varData).knowns, VarData::VarData::VAR_DATA_SIM).clone());
            for mut v in &*acc_discrete_states_accessed.clone() {
                let mut v = v.clone();
                BVariable::setVarKind(v.clone(), Arc::new(VariableKind::VariableKind::PARAMETER { resize_value: None }));
                BVariable::removePartner(v.clone(), (std::sync::Arc::new(fnptr!(BackendInfo::setVarPre, Arc<BackendInfo::BackendInfo>, Option<Pointer::Pointer<Arc<Variable::NFVariable>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendInfo::BackendInfo>, Option<Pointer::Pointer<Arc<Variable::NFVariable>>>) -> Result<Arc<BackendInfo::BackendInfo>> + 'static>));
            }
            varData.clone()
        },
        _ => var_field!((*bdae).varData, NBackendDAE::MAIN).clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
            );
            bdae.clone()
        },
        _ => {
            bdae.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(bdae)
}

pub fn removeStream(mut bdae: Arc<NBackendDAE>) -> Result<Arc<NBackendDAE>> {
    let mut bdae: Arc<NBackendDAE> = bdae;
    bdae = ({
        let mut acc_discrete_states: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>> = Pointer::create(metamodelica::nil());
        let mut acc_previous: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>> = Pointer::create(metamodelica::nil());
        (::match_deref::match_deref! { match &(bdae.clone()) {
        Deref @ MAIN { eqData: eqData @ Deref @ BEquation::EqData::EQ_DATA_SIM { .. }, .. } => {
            let mut eqData = (*eqData).clone();
            assign_variant_field!(eqData => EqData::EqData::EQ_DATA_SIM; equations = BEquation::EquationPointers::map(var_field!((*eqData).equations, EqData::EqData::EQ_DATA_SIM).clone(), Arc::new({ let __pe_b1 = (literal!("NBackendDAE.removeStream")).clone(); let __pe_b2 = (literal!("")).clone(); let __pe_b3 = acc_discrete_states.clone(); let __pe_b4 = acc_previous.clone(); let __pe_b5: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = (std::sync::Arc::new(SimplifyExp::removeStream) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>); move |__pe_a0| Equation::simplify(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone()) }))?);
            assign_variant_field!(bdae => NBackendDAE::MAIN; eqData = BEquation::EqData::compress(eqData.clone())?);
            bdae.clone()
        },
        _ => {
            bdae.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
    Ok(bdae)
}

pub fn getLoopResiduals(mut bdae: Arc<NBackendDAE>) -> Arc<VariablePointers::VariablePointers> {
    let mut residuals: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    residuals = ({
        let mut var_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        (::match_deref::match_deref! { match &(bdae.clone()) {
        Deref @ MAIN { .. } => {
            for mut syst in &*var_field!((*bdae).ode, NBackendDAE::MAIN).clone() {
                let mut syst = syst.clone();
                var_lst = listAppend(NBPartition::Partition::getLoopResiduals(syst.clone()), var_lst.clone());
            }
            for mut syst in &*var_field!((*bdae).algebraic, NBackendDAE::MAIN).clone() {
                let mut syst = syst.clone();
                var_lst = listAppend(NBPartition::Partition::getLoopResiduals(syst.clone()), var_lst.clone());
            }
            for mut syst in &*var_field!((*bdae).ode_event, NBackendDAE::MAIN).clone() {
                let mut syst = syst.clone();
                var_lst = listAppend(NBPartition::Partition::getLoopResiduals(syst.clone()), var_lst.clone());
            }
            for mut syst in &*var_field!((*bdae).alg_event, NBackendDAE::MAIN).clone() {
                let mut syst = syst.clone();
                var_lst = listAppend(NBPartition::Partition::getLoopResiduals(syst.clone()), var_lst.clone());
            }
            for mut syst in &*var_field!((*bdae).init, NBackendDAE::MAIN).clone() {
                let mut syst = syst.clone();
                var_lst = listAppend(NBPartition::Partition::getLoopResiduals(syst.clone()), var_lst.clone());
            }
            residuals = BVariable::VariablePointers::fromList(var_lst.clone(), false);
            residuals.clone()
        },
        _ => {
            BVariable::VariablePointers::empty(BaseHashTable::bigBucketSize.clone(), false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
    residuals
}

fn lowerVariableData(mut varList: Arc<metamodelica::List<Arc<Variable::NFVariable>>>) -> Result<Arc<VarData::VarData>> {
    let mut variableData: Arc<VarData::VarData> = Arc::new(VarData::VAR_DATA_EMPTY);
    let mut lowVar: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
    let mut vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut lowVar_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut time_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut dummy_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut unknowns_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut knowns_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut initials_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut auxiliaries_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut aliasVars_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut nonTrivialAlias_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut states_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut derivatives_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut algebraics_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut discretes_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut discrete_states_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut clocked_states_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut previous_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut clocks_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut inputs_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut resizables_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut parameters_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut constants_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut records_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut external_objects_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut artificials_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut variables: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut unknowns: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut knowns: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut initials: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut auxiliaries: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut aliasVars: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut nonTrivialAlias: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut states: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut derivatives: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut algebraics: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut discretes: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut discrete_states: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut clocked_states: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut previous: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut clocks: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut inputs: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut resizables: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut parameters: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut constants: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut records: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut external_objects: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut artificials: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut binding_iter_set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>> = UnorderedSet::new((std::sync::Arc::new(fnptr!(BVariable::hash, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(BVariable::equalName, Pointer::Pointer<Arc<Variable::NFVariable>>, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>, Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), 13);
    let mut binding_iter_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut scalarized: bool = Flags::isSet(Flags::NF_SCALARIZE.clone())?;
    let mut forced_states: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    vars = List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Variable::NFVariable>>>>> = metamodelica::nil();
        for mut v in (varList.clone()).into_iter().cloned() {
            let __x = Variable::expandChildren(v.clone(), metamodelica::nil(), true);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    variables = BVariable::VariablePointers::empty((vars.clone().len() as i32) + 1, scalarized.clone());
    dummy_ptr = Pointer::create(BVariable::DUMMY_VARIABLE().clone());
    time_ptr = BVariable::createTimeVar()?;
    variables = BVariable::VariablePointers::add(dummy_ptr.clone(), variables.clone())?;
    variables = BVariable::VariablePointers::add(time_ptr.clone(), variables.clone())?;
    artificials_lst = list![dummy_ptr.clone(), time_ptr.clone()];
    for mut var in &*vars.clone().reverse() {
        let mut var = var.clone();
        lowVar_ptr = lowerVariable(var.clone())?;
        lowVar = Pointer::access(lowVar_ptr.clone());
        variables = BVariable::VariablePointers::add(lowVar_ptr.clone(), variables.clone())?;
        let () = (::match_deref::match_deref! { match &(lowVar.backendinfo.varKind.clone()) {
        _ if (Variable::size(lowVar.clone(), false) == 0) => {
            ()
        },
        _ if (Variable::isTopLevelInput(lowVar.clone())) => {
            inputs_lst = cons(lowVar_ptr.clone(), inputs_lst.clone());
            knowns_lst = cons(lowVar_ptr.clone(), knowns_lst.clone());
            ()
        },
        Deref @ VariableKind::ALGEBRAIC => {
            algebraics_lst = cons(lowVar_ptr.clone(), algebraics_lst.clone());
            unknowns_lst = cons(lowVar_ptr.clone(), unknowns_lst.clone());
            initials_lst = cons(lowVar_ptr.clone(), initials_lst.clone());
            ()
        },
        Deref @ VariableKind::STATE { natural, .. } => {
            let mut der_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
            if !(natural.clone()) {
                (_, der_ptr) = BVariable::makeDerVar(BVariable::getVarName(lowVar_ptr.clone()), false)?;
                BVariable::setStateDerivativeVar(lowVar_ptr.clone(), der_ptr.clone());
                derivatives_lst = cons(der_ptr.clone(), derivatives_lst.clone());
                unknowns_lst = cons(der_ptr.clone(), unknowns_lst.clone());
                initials_lst = cons(der_ptr.clone(), initials_lst.clone());
                forced_states = cons(lowVar_ptr.clone(), forced_states.clone());
            }
            states_lst = cons(lowVar_ptr.clone(), states_lst.clone());
            knowns_lst = cons(lowVar_ptr.clone(), knowns_lst.clone());
            initials_lst = cons(lowVar_ptr.clone(), initials_lst.clone());
            ()
        },
        Deref @ VariableKind::STATE_DER { .. } => {
            derivatives_lst = cons(lowVar_ptr.clone(), derivatives_lst.clone());
            unknowns_lst = cons(lowVar_ptr.clone(), unknowns_lst.clone());
            initials_lst = cons(lowVar_ptr.clone(), initials_lst.clone());
            ()
        },
        Deref @ VariableKind::DISCRETE => {
            discretes_lst = cons(lowVar_ptr.clone(), discretes_lst.clone());
            unknowns_lst = cons(lowVar_ptr.clone(), unknowns_lst.clone());
            initials_lst = cons(lowVar_ptr.clone(), initials_lst.clone());
            ()
        },
        Deref @ VariableKind::PREVIOUS => {
            previous_lst = cons(lowVar_ptr.clone(), previous_lst.clone());
            knowns_lst = cons(lowVar_ptr.clone(), knowns_lst.clone());
            initials_lst = cons(lowVar_ptr.clone(), initials_lst.clone());
            ()
        },
        Deref @ VariableKind::PARAMETER { .. } => {
            if BVariable::isResizableParameter(lowVar_ptr.clone()) {
                resizables_lst = cons(lowVar_ptr.clone(), resizables_lst.clone());
            } else {
                parameters_lst = cons(lowVar_ptr.clone(), parameters_lst.clone());
            }
            knowns_lst = cons(lowVar_ptr.clone(), knowns_lst.clone());
            ()
        },
        Deref @ VariableKind::CONSTANT => {
            constants_lst = cons(lowVar_ptr.clone(), constants_lst.clone());
            knowns_lst = cons(lowVar_ptr.clone(), knowns_lst.clone());
            ()
        },
        Deref @ VariableKind::RECORD { .. } => {
            records_lst = cons(lowVar_ptr.clone(), records_lst.clone());
            knowns_lst = cons(lowVar_ptr.clone(), knowns_lst.clone());
            ()
        },
        Deref @ VariableKind::CLOCK => {
            clocks_lst = cons(lowVar_ptr.clone(), clocks_lst.clone());
            ()
        },
        Deref @ VariableKind::CLOCKED => {
            algebraics_lst = cons(lowVar_ptr.clone(), algebraics_lst.clone());
            unknowns_lst = cons(lowVar_ptr.clone(), unknowns_lst.clone());
            initials_lst = cons(lowVar_ptr.clone(), initials_lst.clone());
            ()
        },
        Deref @ VariableKind::EXTOBJ { .. } => {
            lowVar_ptr = BVariable::setFixed(lowVar_ptr.clone(), true, false)?;
            external_objects_lst = cons(lowVar_ptr.clone(), external_objects_lst.clone());
            knowns_lst = cons(lowVar_ptr.clone(), knowns_lst.clone());
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.lowerVariableData")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*BVariable::toString(var.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    unknowns = BVariable::VariablePointers::fromList(unknowns_lst.clone(), scalarized.clone());
    knowns = BVariable::VariablePointers::fromList(knowns_lst.clone(), scalarized.clone());
    initials = BVariable::VariablePointers::fromList(initials_lst.clone(), scalarized.clone());
    auxiliaries = BVariable::VariablePointers::fromList(auxiliaries_lst.clone(), scalarized.clone());
    aliasVars = BVariable::VariablePointers::fromList(aliasVars_lst.clone(), scalarized.clone());
    nonTrivialAlias = BVariable::VariablePointers::fromList(nonTrivialAlias_lst.clone(), scalarized.clone());
    states = BVariable::VariablePointers::fromList(states_lst.clone(), scalarized.clone());
    derivatives = BVariable::VariablePointers::fromList(derivatives_lst.clone(), scalarized.clone());
    algebraics = BVariable::VariablePointers::fromList(algebraics_lst.clone(), scalarized.clone());
    discretes = BVariable::VariablePointers::fromList(discretes_lst.clone(), scalarized.clone());
    discrete_states = BVariable::VariablePointers::fromList(discrete_states_lst.clone(), scalarized.clone());
    clocked_states = BVariable::VariablePointers::fromList(clocked_states_lst.clone(), scalarized.clone());
    previous = BVariable::VariablePointers::fromList(previous_lst.clone(), scalarized.clone());
    clocks = BVariable::VariablePointers::fromList(clocks_lst.clone(), scalarized.clone());
    inputs = BVariable::VariablePointers::fromList(inputs_lst.clone(), scalarized.clone());
    resizables = BVariable::VariablePointers::fromList(resizables_lst.clone(), scalarized.clone());
    parameters = BVariable::VariablePointers::fromList(parameters_lst.clone(), scalarized.clone());
    constants = BVariable::VariablePointers::fromList(constants_lst.clone(), scalarized.clone());
    records = BVariable::VariablePointers::fromList(records_lst.clone(), scalarized.clone());
    external_objects = BVariable::VariablePointers::fromList(external_objects_lst.clone(), scalarized.clone());
    artificials = BVariable::VariablePointers::fromList(artificials_lst.clone(), scalarized.clone());
    variables = BVariable::VariablePointers::map(variables.clone(), Arc::new({ let __pe_b1 = variables.clone(); let __pe_b2 = binding_iter_set.clone(); move |__pe_a0| collectVariableBindingIterators(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
    binding_iter_lst = UnorderedSet::toList(binding_iter_set.clone());
    variables = BVariable::VariablePointers::addList(binding_iter_lst.clone(), variables.clone());
    knowns = BVariable::VariablePointers::addList(binding_iter_lst.clone(), knowns.clone());
    artificials = BVariable::VariablePointers::addList(binding_iter_lst.clone(), artificials.clone());
    variables = BVariable::VariablePointers::map(variables.clone(), Arc::new({ let __pe_b1 = Arc::new({ let __pe_b1 = variables.clone(); let __pe_b2 = true; move |__pe_a0| lowerComponentReferenceExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }); move |__pe_a0| Variable::mapExp(__pe_a0, __pe_b1.clone()) }))?;
    variables = BVariable::VariablePointers::map(variables.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Type::NFType>) -> Result<Arc<Type::NFType>> + 'static> = Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Dimension::NFDimension>) -> Result<Arc<Dimension::NFDimension>> + 'static> = Arc::new({ let __pe_b1 = variables.clone(); let __pe_b2 = true; move |__pe_a0| lowerDimension(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }); move |__pe_a0| Type::applyToDims(__pe_a0, __pe_b1.clone()) }); move |__pe_a0| Ok(Variable::applyToType(__pe_a0, __pe_b1.clone())) }))?;
    records = BVariable::VariablePointers::mapPtr(records.clone(), Arc::new({ let __pe_b1 = variables.clone(); move |__pe_a0| lowerRecordChildren(__pe_a0, __pe_b1.clone()) }))?;
    variableData = Arc::new(VarData::VarData::VAR_DATA_SIM { uniqueIndex: Pointer::create(0), variables: variables.clone(), unknowns: unknowns.clone(), knowns: knowns.clone(), initials: initials.clone(), auxiliaries: auxiliaries.clone(), aliasVars: aliasVars.clone(), nonTrivialAlias: nonTrivialAlias.clone(), derivatives: derivatives.clone(), algebraics: algebraics.clone(), discretes: discretes.clone(), discrete_states: discrete_states.clone(), clocked_states: clocked_states.clone(), previous: previous.clone(), clocks: clocks.clone(), states: states.clone(), top_level_inputs: inputs.clone(), resizables: resizables.clone(), parameters: parameters.clone(), constants: constants.clone(), records: records.clone(), external_objects: external_objects.clone(), artificials: artificials.clone(), state_order: UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1) });
    if Flags::isSet(Flags::DUMP_STATESELECTION_INFO.clone())? {
        println!("{}", (StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[stateselection] (")); __mm_s.push_str(&*intString((forced_states.clone().len() as i32))); __mm_s.push_str(&*literal!(") Forced states by StateSelect.ALWAYS:")); ArcStr::from(__mm_s) }).clone())).clone());
        if forced_states.clone().is_empty() {
            println!("{}", (literal!("\t<no states>\n\n")).clone());
        } else {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*List::toString(forced_states.clone(), (std::sync::Arc::new(fnptr!(BVariable::pointerToString, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("\t")).clone(), (literal!("\n\t")).clone(), (literal!("\n")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
    }
    Ok(variableData)
}

fn lowerVariable(mut var: Arc<Variable::NFVariable>) -> Result<Pointer::Pointer<Arc<Variable::NFVariable>>> {
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut varKind: Arc<VariableKind::VariableKind> = Arc::new(VariableKind::ALGEBRAIC);
    let mut attributes: Arc<VariableAttributes::VariableAttributes>;
    let mut annotations: Arc<Annotations::Annotations> = Arc::new(<Annotations::Annotations as ::std::default::Default>::default());
    match '__try0: {
        attributes = unwrap_break_err!(VariableAttributes::create(var.typeAttributes.clone(), var.ty.clone(), var.attributes.clone(), var.children.clone(), var.comment.clone()), '__try0);
        annotations = Annotations::create(var.comment.clone(), var.attributes.clone());
        assign_field!(
            var.backendinfo = (::match_deref::match_deref! { match &(var.backendinfo.clone()) {
        Deref @ BackendInfo::BACKEND_INFO { varKind: Deref @ VariableKind::FRONTEND_DUMMY, .. } => {
            (varKind, attributes) = unwrap_break_err!(lowerVariableKind(var.clone(), attributes.clone(), var.ty.clone()), '__try0);
            Arc::new(BackendInfo::BackendInfo { varKind: varKind.clone(), attributes: attributes.clone(), annotations: annotations.clone(), var_pre: None, var_seed: None, var_pder: None, var_start: None, parent: None })
        },
        _ => BackendInfo::setAttributes(var.backendinfo.clone(), attributes.clone(), annotations.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }),
            var.typeAttributes = metamodelica::nil()
        );
        (var_ptr, _) = unwrap_break_err!(BVariable::makeVarPtrCyclic(var.clone(), var.name.clone()), '__try0);
        Ok::<_, anyhow::Error>((annotations.clone(), attributes.clone(), var.clone(), var_ptr.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            annotations = __try0_o0;
            attributes = __try0_o1;
            var = __try0_o2;
            var_ptr = __try0_o3;
        }
        Err(_) => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.lowerVariable")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*Variable::toString(var.clone(), (literal!("")).clone(), false)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail");
        }
    }
    Ok(var_ptr)
}

fn lowerVariableKind(mut var: Arc<Variable::NFVariable>, mut attributes: Arc<VariableAttributes::VariableAttributes>, mut ty: Arc<Type::NFType>) -> Result<(Arc<VariableKind::VariableKind>, Arc<VariableAttributes::VariableAttributes>)> {
    fn lowerRecordKind(mut children: Arc<metamodelica::List<Arc<Variable::NFVariable>>>) -> (Prefixes::Variability, Prefixes::Variability) {
        let mut min_var: Prefixes::Variability = Prefixes::Variability::CONTINUOUS.clone();
        let mut max_var: Prefixes::Variability = Prefixes::Variability::CONSTANT.clone();
        let mut tmp_min_var: Prefixes::Variability = Prefixes::Variability::CONSTANT;
        let mut tmp_max_var: Prefixes::Variability = Prefixes::Variability::CONSTANT;
        for mut child in &*children.clone() {
            let mut child = child.clone();
            (tmp_min_var, tmp_max_var) = (::match_deref::match_deref! { match &(child.ty.clone()) {
        Deref @ Type::COMPLEX { .. } => lowerRecordKind(child.children.clone()),
        Deref @ Type::ARRAY { elementType: Deref @ Type::COMPLEX { .. }, .. } => lowerRecordKind(child.children.clone()),
        _ => (Variable::variability(child.clone()), Variable::variability(child.clone())),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            min_var = if (tmp_min_var.clone() < min_var.clone()) {tmp_min_var.clone()} else {min_var.clone()};
            max_var = if (tmp_max_var.clone() > max_var.clone()) {tmp_max_var.clone()} else {max_var.clone()};
        }
        (min_var, max_var)
    }

    let mut varKind: Arc<VariableKind::VariableKind> = Arc::new(VariableKind::ALGEBRAIC);
    let mut attributes: Arc<VariableAttributes::VariableAttributes> = attributes;
    let mut min_var: Prefixes::Variability = Prefixes::Variability::CONSTANT;
    let mut max_var: Prefixes::Variability = Prefixes::Variability::CONSTANT;
    let mut variability: Prefixes::Variability = Variable::variability(var.clone());
    varKind = ({
        let mut children: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        (::match_deref::match_deref! { match &((variability.clone(), attributes.clone(), ty.clone())) {
        (_, _, Deref @ Type::CLOCK) => {
            Arc::new(openmodelica_nf_frontend::NFBackendExtension::VariableKind::CLOCK)
        },
        (_, _, _) if (Binding::isClockOrSampleFunction(var.binding.clone())?) => {
            Arc::new(openmodelica_nf_frontend::NFBackendExtension::VariableKind::CLOCKED)
        },
        (Prefixes::Variability::CONTINUOUS, Deref @ VariableAttributes::VAR_ATTR_REAL { stateSelect: Some(NFBackendExtension::StateSelect::ALWAYS), .. }, _) if (variability.clone() == Prefixes::Variability::CONTINUOUS.clone()) => {
            Arc::new(VariableKind::VariableKind::STATE { index: 1, derivative: None, natural: false })
        },
        (_, _, Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::EXTERNAL_OBJECT { .. }, .. }) => {
            Arc::new(VariableKind::VariableKind::EXTOBJ { fullClassName: Class::constrainingClassPath(var_field!((*ty).cls, Type::NFType::COMPLEX).clone())? })
        },
        (_, _, Deref @ Type::ARRAY { elementType: elemTy @ Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::EXTERNAL_OBJECT { .. }, .. }, .. }) => {
            Arc::new(VariableKind::VariableKind::EXTOBJ { fullClassName: Class::constrainingClassPath(var_field!((**elemTy).cls, Type::NFType::COMPLEX).clone())? })
        },
        (_, _, Deref @ Type::COMPLEX { .. }) => {
            (min_var, max_var) = lowerRecordKind(var.children.clone());
            Arc::new(VariableKind::VariableKind::RECORD { children: metamodelica::nil(), min_var: min_var.clone(), max_var: max_var.clone() })
        },
        (_, _, Deref @ Type::ARRAY { elementType: Deref @ Type::COMPLEX { .. }, .. }) => {
            (min_var, max_var) = lowerRecordKind(var.children.clone());
            Arc::new(VariableKind::VariableKind::RECORD { children: metamodelica::nil(), min_var: min_var.clone(), max_var: max_var.clone() })
        },
        (Prefixes::Variability::CONTINUOUS, _, Deref @ Type::BOOLEAN) => {
            Arc::new(openmodelica_nf_frontend::NFBackendExtension::VariableKind::DISCRETE)
        },
        (Prefixes::Variability::CONTINUOUS, _, Deref @ Type::INTEGER) => {
            Arc::new(openmodelica_nf_frontend::NFBackendExtension::VariableKind::DISCRETE)
        },
        (Prefixes::Variability::CONTINUOUS, _, Deref @ Type::ENUMERATION { .. }) => {
            Arc::new(openmodelica_nf_frontend::NFBackendExtension::VariableKind::DISCRETE)
        },
        (Prefixes::Variability::CONTINUOUS, _, _) => {
            Arc::new(openmodelica_nf_frontend::NFBackendExtension::VariableKind::ALGEBRAIC)
        },
        (Prefixes::Variability::DISCRETE, _, _) => {
            Arc::new(openmodelica_nf_frontend::NFBackendExtension::VariableKind::DISCRETE)
        },
        (Prefixes::Variability::IMPLICITLY_DISCRETE, _, _) => {
            Arc::new(openmodelica_nf_frontend::NFBackendExtension::VariableKind::DISCRETE)
        },
        (Prefixes::Variability::PARAMETER, _, _) => {
            Arc::new(VariableKind::VariableKind::PARAMETER { resize_value: None })
        },
        (Prefixes::Variability::STRUCTURAL_PARAMETER, _, _) => {
            Arc::new(VariableKind::VariableKind::PARAMETER { resize_value: None })
        },
        (Prefixes::Variability::NON_STRUCTURAL_PARAMETER, _, _) => {
            Arc::new(VariableKind::VariableKind::PARAMETER { resize_value: None })
        },
        (Prefixes::Variability::CONSTANT, _, _) => {
            Arc::new(openmodelica_nf_frontend::NFBackendExtension::VariableKind::CONSTANT)
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.lowerVariableKind")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
    attributes = (::match_deref::match_deref! { match &(varKind.clone()) {
        Deref @ VariableKind::PARAMETER { .. } => VariableAttributes::setFixed(attributes.clone(), ty.clone(), true, false)?,
        _ => attributes.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((varKind, attributes))
}

fn collectVariableBindingIterators(mut var: Arc<Variable::NFVariable>, mut variables: Arc<VariablePointers::VariablePointers>, mut set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>) -> Result<Arc<Variable::NFVariable>> {
    let mut var: Arc<Variable::NFVariable> = var;
    let mut exp_opt: Option<Arc<Expression::NFExpression>> = None;
    BackendInfo::map(var.backendinfo.clone(), Arc::new({ let __pe_b1 = variables.clone(); let __pe_b2 = set.clone(); move |__pe_a0| collectIterators(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
    exp_opt = Binding::typedExp(var.binding.clone());
    if isSome(exp_opt.clone()) {
        Expression::map(Util::getOption(exp_opt.clone())?, Arc::new({ let __pe_b1 = variables.clone(); let __pe_b2 = set.clone(); move |__pe_a0| collectIterators(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
    }
    Ok(var)
}

pub fn lowerRecordChildren(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut variables: Arc<VariablePointers::VariablePointers>) -> Result<()> {
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    var = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { backendinfo: binfo @ Deref @ BackendInfo::BACKEND_INFO { varKind: varKind @ Deref @ VariableKind::RECORD { .. }, .. }, .. } => {
            let mut binfo = (*binfo).clone();
            let mut varKind = (*varKind).clone();
            assign_variant_field!(varKind => VariableKind::VariableKind::RECORD;
                children = {
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut child in (var.children.clone()).into_iter().cloned() {
            let __x = BVariable::VariablePointers::getVarSafe(variables.clone(), ComponentRef::stripSubscriptsAll(child.name.clone()), Some(metamodelica::sourceInfo!()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                children = {
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut child in (var_field!((*varKind).children, VariableKind::VariableKind::RECORD).clone()).into_iter().cloned() {
            let __x = BVariable::setParent(child.clone(), var_ptr.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
            );
            assign_field!(binfo.varKind = varKind.clone());
            assign_field!(var.backendinfo = binfo.clone());
            var.clone()
        },
        _ => {
            var.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Pointer::update(var_ptr.clone(), var.clone());
    Ok(())
}

fn lowerEquationData(mut eq_lst: Arc<metamodelica::List<Arc<FEquation::NFEquation>>>, mut al_lst: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>>, mut init_eq_lst: Arc<metamodelica::List<Arc<FEquation::NFEquation>>>, mut init_al_lst: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>>, mut varData: Arc<VarData::VarData>) -> Result<(Arc<EqData::EqData>, Arc<VarData::VarData>)> {
    let mut eqData: Arc<EqData::EqData> = Arc::new(EqData::EQ_DATA_EMPTY);
    let mut varData: Arc<VarData::VarData> = varData;
    let mut set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>> = UnorderedSet::new((std::sync::Arc::new(fnptr!(BVariable::hash, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(BVariable::equalName, Pointer::Pointer<Arc<Variable::NFVariable>>, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>, Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), 13);
    let mut equation_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    let mut continuous_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    let mut clocked_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    let mut discretes_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    let mut initials_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    let mut auxiliaries_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    let mut simulation_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    let mut removed_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    let mut equations: Arc<EquationPointers::EquationPointers> = Arc::new(<EquationPointers::EquationPointers as ::std::default::Default>::default());
    let mut eq: Pointer::Pointer<Arc<Equation::Equation>>;
    let mut idx: Pointer::Pointer<i32> = Pointer::create(0);
    equation_lst = lowerEquationsAndAlgorithms(eq_lst.clone(), al_lst.clone(), init_eq_lst.clone(), init_al_lst.clone())?;
    for mut eqn_ptr in &*equation_lst.clone() {
        let mut eqn_ptr = eqn_ptr.clone();
        BEquation::Equation::createName(eqn_ptr.clone(), idx.clone(), (arcstr::literal!(BEquation::SIMULATION_STR)).clone())?;
        BEquation::Equation::renameIterators(eqn_ptr.clone(), (literal!("$i")).clone())?;
        lowerEquationIterators(Pointer::access(eqn_ptr.clone()), BVariable::VarData::getVariables(varData.clone())?, set.clone())?;
    }
    varData = BVariable::VarData::addTypedList(varData.clone(), UnorderedSet::toList(set.clone()), BVariable::VarData::VarType::ITERATOR.clone())?;
    equations = BEquation::EquationPointers::fromList(equation_lst.clone());
    equations = lowerComponentReferences(equations.clone(), BVariable::VarData::getVariables(varData.clone())?)?;
    (simulation_lst, continuous_lst, clocked_lst, discretes_lst, initials_lst, auxiliaries_lst, removed_lst) = BEquation::typeList(BEquation::EquationPointers::toList(equations.clone())?)?;
    equations = BEquation::EquationPointers::removeList(clocked_lst.clone(), equations.clone())?;
    (equations, _) = Resizable::resize(equations.clone(), varData.clone())?;
    eqData = Arc::new(EqData::EqData::EQ_DATA_SIM { removed: BEquation::EquationPointers::fromList(removed_lst.clone()), auxiliaries: BEquation::EquationPointers::fromList(auxiliaries_lst.clone()), initials: BEquation::EquationPointers::fromList(initials_lst.clone()), discretes: BEquation::EquationPointers::fromList(discretes_lst.clone()), clocked: BEquation::EquationPointers::fromList(clocked_lst.clone()), continuous: BEquation::EquationPointers::fromList(continuous_lst.clone()), simulation: BEquation::EquationPointers::fromList(simulation_lst.clone()), equations: equations.clone(), uniqueIndex: idx.clone() });
    Ok((eqData, varData))
}

fn lowerEquationsAndAlgorithms(mut eq_lst: Arc<metamodelica::List<Arc<FEquation::NFEquation>>>, mut al_lst: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>>, mut init_eq_lst: Arc<metamodelica::List<Arc<FEquation::NFEquation>>>, mut init_al_lst: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>>) -> Result<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>> {
    let mut equations: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    for mut eq in &*eq_lst.clone() {
        let mut eq = eq.clone();
        equations = listAppend(lowerEquation(eq.clone(), false, false)?, equations.clone());
    }
    for mut alg in &*al_lst.clone() {
        let mut alg = alg.clone();
        equations = cons(lowerAlgorithm(alg.clone(), false), equations.clone());
    }
    for mut eq in &*init_eq_lst.clone() {
        let mut eq = eq.clone();
        equations = listAppend(lowerEquation(eq.clone(), true, false)?, equations.clone());
    }
    for mut alg in &*init_al_lst.clone() {
        let mut alg = alg.clone();
        equations = cons(lowerAlgorithm(alg.clone(), true), equations.clone());
    }
    Ok(equations)
}

fn lowerEquation(mut frontend_equation: Arc<FEquation::NFEquation>, mut init: bool, mut in_for: bool) -> Result<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>> {
    let mut backend_equations: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    backend_equations = (::match_deref::match_deref! { match &(frontend_equation.clone()) {
        Deref @ FEquation::EQUALITY { source, ty, rhs, lhs, .. } => {
            let mut attr: Arc<EquationAttributes::EquationAttributes> = Arc::new(<EquationAttributes::EquationAttributes as ::std::default::Default>::default());
            attr = lowerEquationAttributes(ty.clone(), init.clone());
            backend_equations = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::ARRAY { .. } => list![Pointer::create(Arc::new(Equation::Equation::ARRAY_EQUATION { ty: ty.clone(), lhs: lhs.clone(), rhs: rhs.clone(), source: source.clone(), attr: attr.clone(), recordSize: Type::complexSize(ty.clone(), false)? }))],
        Deref @ Type::COMPLEX { .. } => list![Pointer::create(Arc::new(Equation::Equation::RECORD_EQUATION { ty: ty.clone(), lhs: lhs.clone(), rhs: rhs.clone(), source: source.clone(), attr: attr.clone(), recordSize: Type::recordFieldCount(ty.clone()) }))],
        Deref @ Type::TUPLE { .. } => list![Pointer::create(Arc::new(Equation::Equation::RECORD_EQUATION { ty: ty.clone(), lhs: lhs.clone(), rhs: rhs.clone(), source: source.clone(), attr: attr.clone(), recordSize: Type::tupleFieldCount(ty.clone()) }))],
        _ => list![Pointer::create(Arc::new(Equation::Equation::SCALAR_EQUATION { ty: ty.clone(), lhs: lhs.clone(), rhs: rhs.clone(), source: source.clone(), attr: attr.clone() }))],
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            backend_equations.clone()
        },
        Deref @ FEquation::FOR { .. } => {
            lowerForEquation(frontend_equation.clone(), init.clone())?
        },
        Deref @ FEquation::IF { .. } => {
            lowerIfEquation(frontend_equation.clone(), init.clone(), in_for.clone())?
        },
        Deref @ FEquation::WHEN { .. } => {
            lowerWhenEquation(frontend_equation.clone(), init.clone())?
        },
        Deref @ FEquation::ASSERT { .. } => {
            lowerAssert(frontend_equation.clone(), init.clone())?
        },
        Deref @ FEquation::NORETCALL { .. } => {
            let mut stmt: Arc<Statement::NFStatement>;
            let mut alg: Arc<Algorithm::NFAlgorithm> = Arc::new(<Algorithm::NFAlgorithm as ::std::default::Default>::default());
            stmt = Arc::new(Statement::NFStatement::NORETCALL { exp: var_field!((*frontend_equation).exp, FEquation::NFEquation::NORETCALL).clone(), source: var_field!((*frontend_equation).source, FEquation::NFEquation::NORETCALL).clone() });
            alg = Arc::new(Algorithm::NFAlgorithm { statements: list![stmt.clone()], inputs: metamodelica::nil(), outputs: metamodelica::nil(), stmtDiffInfo: None, scope: Arc::new(openmodelica_nf_frontend::NFInstNode::InstNode::EMPTY_NODE), source: var_field!((*frontend_equation).source, FEquation::NFEquation::NORETCALL).clone() });
            alg = Algorithm::setInputsOutputs(alg.clone())?;
            list![lowerAlgorithm(alg.clone(), init.clone())]
        },
        Deref @ FEquation::TERMINATE { .. } => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.lowerEquation")); __mm_s.push_str(&*literal!(" failed for TERMINATE expression without condition:\n")); __mm_s.push_str(&*FEquation::toString(frontend_equation.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        Deref @ FEquation::REINIT { .. } => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.lowerEquation")); __mm_s.push_str(&*literal!(" failed for REINIT expression without condition:\n")); __mm_s.push_str(&*FEquation::toString(frontend_equation.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.lowerEquation")); __mm_s.push_str(&*literal!(" failed for\n")); __mm_s.push_str(&*FEquation::toString(frontend_equation.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(backend_equations)
}

fn lowerForEquation(mut frontend_equation: Arc<FEquation::NFEquation>, mut init: bool) -> Result<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>> {
    let mut backend_equations: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    let mut range: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut new_body: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    let mut body_elem: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
    let mut body: Arc<metamodelica::List<Arc<FEquation::NFEquation>>> = metamodelica::nil();
    let mut bodies: Arc<metamodelica::List<Arc<IfEquationBody::IfEquationBody>>> = metamodelica::nil();
    let mut iterator: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut isAlgorithm: bool = false;
    let mut alg: Arc<Algorithm::NFAlgorithm> = Arc::new(<Algorithm::NFAlgorithm as ::std::default::Default>::default());
    let mut size: i32 = 0;
    backend_equations = (::match_deref::match_deref! { match &(frontend_equation.clone()) {
        Deref @ FEquation::FOR { range: Some(range), .. } => {
            if Expression::rangeSize(range.clone(), false) > 0 {
                iterator = ComponentRef::fromNode(var_field!((*frontend_equation).iterator, FEquation::NFEquation::FOR).clone(), Arc::new(openmodelica_nf_frontend::NFType::INTEGER), metamodelica::nil(), ComponentRef::Origin::ITERATOR.clone());
                for mut eq in &*var_field!((*frontend_equation).body, FEquation::NFEquation::FOR).clone() {
                    let mut eq = eq.clone();
                    for mut body_elem_ptr in &*lowerEquation(eq.clone(), init.clone(), true)? {
                        let mut body_elem_ptr = body_elem_ptr.clone();
                        body_elem = Pointer::access(body_elem_ptr.clone());
                        new_body = (::match_deref::match_deref! { match &(body_elem.clone()) {
        Deref @ BEquation::Equation::IF_EQUATION { .. } => {
            bodies = BEquation::IfEquationBody::split(var_field!((*body_elem).body, Equation::Equation::IF_EQUATION).clone())?;
            for mut body in &*bodies.clone() {
                let mut body = body.clone();
                new_body = cons(Pointer::create(Arc::new(Equation::Equation::IF_EQUATION { size: BEquation::IfEquationBody::size(body.clone(), false), body: body.clone(), source: var_field!((*body_elem).source, Equation::Equation::IF_EQUATION).clone(), attr: var_field!((*body_elem).attr, Equation::Equation::IF_EQUATION).clone() })), new_body.clone());
            }
            new_body.clone()
        },
        _ => cons(body_elem_ptr.clone(), new_body.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                    }
                }
                for mut body_elem_ptr in &*new_body.clone() {
                    let mut body_elem_ptr = body_elem_ptr.clone();
                    body_elem = Pointer::access(body_elem_ptr.clone());
                    isAlgorithm = BEquation::Equation::isAlgorithm(body_elem_ptr.clone());
                    body_elem = Arc::new(Equation::Equation::FOR_EQUATION { attr: BEquation::Equation::getAttributes(body_elem.clone()), source: var_field!((*frontend_equation).source, FEquation::NFEquation::FOR).clone(), body: list![body_elem.clone()], iter: Arc::new(Iterator::Iterator::SINGLE { name: iterator.clone(), range: range.clone(), map: None }), size: Expression::rangeSize(range.clone(), false) * BEquation::Equation::size(body_elem_ptr.clone(), false)? });
                    (body_elem, _) = BEquation::Equation::mergeIterators(body_elem.clone(), true)?;
                    body_elem = BEquation::Equation::simplify(body_elem.clone(), (literal!("")).clone(), (literal!("")).clone(), Pointer::create(metamodelica::nil()), Pointer::create(metamodelica::nil()), Arc::new({ let __pe_b1 = true; let __pe_b2 = (literal!("")).clone(); let __pe_b3 = (literal!("")).clone(); move |__pe_a0| SimplifyExp::simplifyDump(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }))?;
                    if isAlgorithm.clone() {
                        alg = Arc::new(Algorithm::NFAlgorithm { statements: BEquation::Equation::toStatement(body_elem.clone())?, inputs: metamodelica::nil(), outputs: metamodelica::nil(), stmtDiffInfo: None, scope: Arc::new(openmodelica_nf_frontend::NFInstNode::InstNode::EMPTY_NODE), source: var_field!((*frontend_equation).source, FEquation::NFEquation::FOR).clone() });
                        alg = Algorithm::setInputsOutputs(alg.clone())?;
                        size = {
        let mut __acc: i32 = 0;
        for mut out in (alg.outputs.clone()).into_iter().cloned() {
            let __x = ComponentRef::size(out.clone(), false, false);
            __acc += __x;
        }
        __acc
    };
                        body_elem = Arc::new(Equation::Equation::ALGORITHM { size: size.clone(), alg: alg.clone(), source: alg.source.clone(), expand: openmodelica_frontend_types::DAE::Expand::EXPAND, attr: BEquation::Equation::getAttributes(body_elem.clone()) });
                    }
                    Pointer::update(body_elem_ptr.clone(), body_elem.clone());
                    backend_equations = cons(body_elem_ptr.clone(), backend_equations.clone());
                }
            } else {
                if Flags::isSet(Flags::FAILTRACE.clone())? {
                    Error::addMessage(Error::COMPILER_WARNING.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.lowerForEquation")); __mm_s.push_str(&*literal!(": Empty for-equation got removed:\n")); __mm_s.push_str(&*FEquation::toString(frontend_equation.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
                }
            }
            backend_equations.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.lowerForEquation")); __mm_s.push_str(&*literal!(" failed for\n")); __mm_s.push_str(&*FEquation::toString(frontend_equation.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(backend_equations)
}

fn lowerIfEquation(mut frontend_equation: Arc<FEquation::NFEquation>, mut init: bool, mut in_for: bool) -> Result<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>> {
    let mut backend_equations: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    backend_equations = (::match_deref::match_deref! { match &(frontend_equation.clone()) {
        Deref @ FEquation::IF { source, branches, .. } => {
            let mut ifEqBody: Arc<IfEquationBody::IfEquationBody> = Arc::new(<IfEquationBody::IfEquationBody as ::std::default::Default>::default());
            let mut bodies: Arc<metamodelica::List<Arc<IfEquationBody::IfEquationBody>>> = metamodelica::nil();
            if let Ok(__iflet0) = lowerIfEquationBody(branches.clone(), init.clone(), in_for.clone() || FEquation::sizeOf(frontend_equation.clone())? == 0) {
                ifEqBody = __iflet0;
            } else {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.lowerIfEquation")); __mm_s.push_str(&*literal!(" failed for:\n")); __mm_s.push_str(&*FEquation::toString(frontend_equation.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
                bail!("fail");
            }
            if Expression::isEnd(ifEqBody.condition.clone()) {
                backend_equations = ifEqBody.then_eqns.clone();
            } else {
                bodies = BEquation::IfEquationBody::split(ifEqBody.clone())?;
                backend_equations = {
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
        for mut body in (bodies.clone()).into_iter().cloned() {
            let __x = BEquation::IfEquationBody::toEquation(body.clone(), source.clone(), init.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            }
            backend_equations.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.lowerIfEquation")); __mm_s.push_str(&*literal!(" failed for\n")); __mm_s.push_str(&*FEquation::toString(frontend_equation.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(backend_equations)
}

fn lowerIfEquationBody(mut branches: Arc<metamodelica::List<Arc<FEquation::Branch::Branch>>>, mut init: bool, mut allow_imbalance: bool) -> Result<Arc<IfEquationBody::IfEquationBody>> {
    let mut ifEq: Arc<IfEquationBody::IfEquationBody> = Arc::new(<IfEquationBody::IfEquationBody as ::std::default::Default>::default());
    ifEq = (::match_deref::match_deref! { match &(branches.clone()) {
        Deref @ metamodelica::List::Cons { head: branch, tail: rest } => {
            let mut eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
            let mut condition: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut result: Arc<IfEquationBody::IfEquationBody> = Arc::new(<IfEquationBody::IfEquationBody as ::std::default::Default>::default());
            (eqns, condition) = lowerIfBranch(branch.clone(), init.clone())?;
            if Expression::isTrue(condition.clone()) {
                result = Arc::new(IfEquationBody::IfEquationBody { condition: Arc::new(openmodelica_nf_frontend::NFExpression::END), then_eqns: eqns.clone(), else_if: None });
            } else if Expression::isFalse(condition.clone()) {
                result = lowerIfEquationBody(rest.clone(), init.clone(), allow_imbalance.clone())?;
            } else {
                if rest.clone().is_empty() && (init.clone() || allow_imbalance.clone()) {
                    result = Arc::new(IfEquationBody::IfEquationBody { condition: condition.clone(), then_eqns: eqns.clone(), else_if: None });
                } else {
                    result = Arc::new(IfEquationBody::IfEquationBody { condition: condition.clone(), then_eqns: eqns.clone(), else_if: Some(lowerIfEquationBody(rest.clone(), init.clone(), allow_imbalance.clone())?) });
                }
            }
            result.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.lowerIfEquationBody")); __mm_s.push_str(&*literal!(" failed due to invalid missing else case.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ifEq)
}

fn lowerIfBranch(mut branch: Arc<FEquation::Branch::Branch>, mut init: bool) -> Result<(Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>, Arc<Expression::NFExpression>)> {
    let mut eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    (eqns, cond) = (::match_deref::match_deref! { match &(branch.clone()) {
        Deref @ FEquation::Branch::BRANCH { .. } => {
            if Expression::isFalse(var_field!((*branch).condition, FEquation::Branch::Branch::BRANCH).clone()) {
                eqns = metamodelica::nil();
            } else {
                eqns = lowerIfBranchBody(var_field!((*branch).body, FEquation::Branch::Branch::BRANCH).clone(), init.clone(), metamodelica::nil())?;
            }
            (eqns.clone(), var_field!((*branch).condition, FEquation::Branch::Branch::BRANCH).clone())
        },
        Deref @ FEquation::Branch::INVALID_BRANCH { .. } => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.lowerIfBranch")); __mm_s.push_str(&*literal!(" failed for invalid branch that should not exist outside of frontend.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.lowerIfBranch")); __mm_s.push_str(&*literal!(" failed without proper error message.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((eqns, cond))
}

fn lowerIfBranchBody(mut body: Arc<metamodelica::List<Arc<FEquation::NFEquation>>>, mut init: bool, mut eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>) -> Result<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>> {
    let mut eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = eqns;
    eqns = (::match_deref::match_deref! { match &(body.clone()) {
        Deref @ metamodelica::List::Nil => {
            eqns.clone()
        },
        Deref @ metamodelica::List::Cons { head: elem, tail: rest } => {
            lowerIfBranchBody(rest.clone(), init.clone(), listAppend(lowerEquation(elem.clone(), init.clone(), false)?, eqns.clone()))?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eqns)
}

fn lowerAssert(mut frontend_eq: Arc<FEquation::NFEquation>, mut init: bool) -> Result<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>> {
    let mut backend_equations: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    backend_equations = (::match_deref::match_deref! { match &(frontend_eq.clone()) {
        Deref @ FEquation::ASSERT { .. } => {
            let mut alg: Arc<Algorithm::NFAlgorithm> = Arc::new(<Algorithm::NFAlgorithm as ::std::default::Default>::default());
            let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let _ = BEquation::default(EquationKind::EMPTY.clone(), init.clone(), None, None);
            cond = if (Expression::isCall(var_field!((*frontend_eq).condition, FEquation::NFEquation::ASSERT).clone())) {var_field!((*frontend_eq).condition, FEquation::NFEquation::ASSERT).clone()} else {Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::NO_EVENT().clone(), list![var_field!((*frontend_eq).condition, FEquation::NFEquation::ASSERT).clone()], Expression::variability(var_field!((*frontend_eq).condition, FEquation::NFEquation::ASSERT).clone())?, Prefixes::Purity::PURE.clone(), NFBuiltinFuncs::NO_EVENT().returnType.clone()) })};
            alg = Arc::new(Algorithm::NFAlgorithm { statements: list![Arc::new(Statement::NFStatement::ASSERT { condition: cond.clone(), message: var_field!((*frontend_eq).message, FEquation::NFEquation::ASSERT).clone(), level: var_field!((*frontend_eq).level, FEquation::NFEquation::ASSERT).clone(), source: var_field!((*frontend_eq).source, FEquation::NFEquation::ASSERT).clone() })], inputs: metamodelica::nil(), outputs: metamodelica::nil(), stmtDiffInfo: None, scope: var_field!((*frontend_eq).scope, FEquation::NFEquation::ASSERT).clone(), source: var_field!((*frontend_eq).source, FEquation::NFEquation::ASSERT).clone() });
            list![lowerAlgorithm(alg.clone(), init.clone())]
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.lowerAssert")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*FEquation::toString(frontend_eq.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(backend_equations)
}

fn lowerWhenEquation(mut frontend_eq: Arc<FEquation::NFEquation>, mut init: bool) -> Result<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>> {
    let mut backend_equations: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    backend_equations = (::match_deref::match_deref! { match &(frontend_eq.clone()) {
        Deref @ FEquation::WHEN { .. } => {
            let mut whenEqBody: Arc<BEquation::WhenEquationBody::WhenEquationBody> = Arc::new(<BEquation::WhenEquationBody::WhenEquationBody as ::std::default::Default>::default());
            let mut bodies: Arc<metamodelica::List<Arc<BEquation::WhenEquationBody::WhenEquationBody>>> = metamodelica::nil();
            let __pa0 = ::match_deref::match_deref! { match &(lowerWhenEquationBody(var_field!((*frontend_eq).branches, FEquation::NFEquation::WHEN).clone())?) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            whenEqBody = __pa0.clone();
            bodies = BEquation::WhenEquationBody::split(whenEqBody.clone())?;
            {
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<BEquation::Equation::WHEN_EQUATION>>> = metamodelica::nil();
        for mut b in (bodies.clone()).into_iter().cloned() {
            let __x = Pointer::create(Arc::new(Equation::Equation::WHEN_EQUATION { attr: BEquation::default(if (BEquation::WhenEquationBody::size(b.clone(), false) > 0) {EquationKind::DISCRETE.clone()} else {EquationKind::EMPTY.clone()}, init.clone(), None, None), source: var_field!((*frontend_eq).source, FEquation::NFEquation::WHEN).clone(), body: b.clone(), size: BEquation::WhenEquationBody::size(b.clone(), false) }));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.lowerWhenEquation")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*FEquation::toString(frontend_eq.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(backend_equations)
}

fn lowerWhenEquationBody(mut branches: Arc<metamodelica::List<Arc<FEquation::Branch::Branch>>>) -> Result<Option<Arc<BEquation::WhenEquationBody::WhenEquationBody>>> {
    let mut whenEq: Option<Arc<BEquation::WhenEquationBody::WhenEquationBody>> = None;
    whenEq = (::match_deref::match_deref! { match &(branches.clone()) {
        Deref @ metamodelica::List::Nil => {
            None
        },
        Deref @ metamodelica::List::Cons { head: branch, tail: rest } => {
            let mut stmts: Arc<metamodelica::List<Arc<BEquation::WhenStatement::WhenStatement>>> = metamodelica::nil();
            let mut condition: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            (stmts, condition) = lowerWhenBranch(branch.clone())?;
            Some(Arc::new(BEquation::WhenEquationBody::WhenEquationBody { condition: condition.clone(), when_stmts: stmts.clone(), else_when: lowerWhenEquationBody(rest.clone())? }))
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.lowerWhenEquationBody")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(whenEq)
}

fn lowerWhenBranch(mut branch: Arc<FEquation::Branch::Branch>) -> Result<(Arc<metamodelica::List<Arc<BEquation::WhenStatement::WhenStatement>>>, Arc<Expression::NFExpression>)> {
    let mut stmts: Arc<metamodelica::List<Arc<BEquation::WhenStatement::WhenStatement>>> = metamodelica::nil();
    let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    (stmts, cond) = (::match_deref::match_deref! { match &(branch.clone()) {
        Deref @ FEquation::Branch::BRANCH { body, condition, .. } => {
            (lowerWhenBranchBody(condition.clone(), body.clone(), metamodelica::nil())?, condition.clone())
        },
        Deref @ FEquation::Branch::INVALID_BRANCH { .. } => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.lowerWhenBranch")); __mm_s.push_str(&*literal!(" failed for invalid branch that should not exist outside of frontend.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.lowerWhenBranch")); __mm_s.push_str(&*literal!(" failed without proper error message.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((stmts, cond))
}

fn lowerWhenBranchBody(mut condition: Arc<Expression::NFExpression>, mut body: Arc<metamodelica::List<Arc<FEquation::NFEquation>>>, mut stmts: Arc<metamodelica::List<Arc<BEquation::WhenStatement::WhenStatement>>>) -> Result<Arc<metamodelica::List<Arc<BEquation::WhenStatement::WhenStatement>>>> {
    let mut stmts: Arc<metamodelica::List<Arc<BEquation::WhenStatement::WhenStatement>>> = stmts;
    stmts = (::match_deref::match_deref! { match &(body.clone()) {
        Deref @ metamodelica::List::Cons { head: elem, tail: rest } => {
            lowerWhenBranchBody(condition.clone(), rest.clone(), lowerWhenBranchStatement(elem.clone(), condition.clone(), stmts.clone())?)?
        },
        _ => {
            stmts.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(stmts)
}

fn lowerWhenBranchStatement(mut eq: Arc<FEquation::NFEquation>, mut condition: Arc<Expression::NFExpression>, mut stmts: Arc<metamodelica::List<Arc<BEquation::WhenStatement::WhenStatement>>>) -> Result<Arc<metamodelica::List<Arc<BEquation::WhenStatement::WhenStatement>>>> {
    let mut stmts: Arc<metamodelica::List<Arc<BEquation::WhenStatement::WhenStatement>>> = stmts;
    stmts = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ FEquation::TERMINATE { .. } => {
            cons(Arc::new(BEquation::WhenStatement::WhenStatement::TERMINATE { message: var_field!((*eq).message, FEquation::NFEquation::TERMINATE).clone(), source: var_field!((*eq).source, FEquation::NFEquation::TERMINATE).clone() }), stmts.clone())
        },
        Deref @ FEquation::REINIT { cref: Deref @ Expression::CREF { cref, .. }, .. } => {
            cons(Arc::new(BEquation::WhenStatement::WhenStatement::REINIT { stateVar: cref.clone(), value: var_field!((*eq).reinitExp, FEquation::NFEquation::REINIT).clone(), source: var_field!((*eq).source, FEquation::NFEquation::REINIT).clone() }), stmts.clone())
        },
        Deref @ FEquation::NORETCALL { .. } => {
            cons(Arc::new(BEquation::WhenStatement::WhenStatement::NORETCALL { exp: var_field!((*eq).exp, FEquation::NFEquation::NORETCALL).clone(), source: var_field!((*eq).source, FEquation::NFEquation::NORETCALL).clone() }), stmts.clone())
        },
        Deref @ FEquation::ASSERT { .. } => {
            cons(Arc::new(BEquation::WhenStatement::WhenStatement::ASSERT { condition: var_field!((*eq).condition, FEquation::NFEquation::ASSERT).clone(), message: var_field!((*eq).message, FEquation::NFEquation::ASSERT).clone(), level: var_field!((*eq).level, FEquation::NFEquation::ASSERT).clone(), source: var_field!((*eq).source, FEquation::NFEquation::ASSERT).clone() }), stmts.clone())
        },
        Deref @ FEquation::EQUALITY { .. } => {
            cons(Arc::new(BEquation::WhenStatement::WhenStatement::ASSIGN { lhs: var_field!((*eq).lhs, FEquation::NFEquation::EQUALITY).clone(), rhs: var_field!((*eq).rhs, FEquation::NFEquation::EQUALITY).clone(), source: var_field!((*eq).source, FEquation::NFEquation::EQUALITY).clone() }), stmts.clone())
        },
        Deref @ FEquation::IF { .. } => {
            let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut head: Arc<FEquation::Branch::Branch>;
            let mut tail: Arc<metamodelica::List<Arc<FEquation::Branch::Branch>>> = metamodelica::nil();
            let mut if_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>;
            if_map = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(var_field!((*eq).branches, FEquation::NFEquation::IF).clone().reverse()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            head = __pa0.clone();
            tail = __pa1.clone();
            lowerWhenBranchIf(head.clone(), if_map.clone(), true)?;
            for mut branch in &*tail.clone() {
                let mut branch = branch.clone();
                lowerWhenBranchIf(branch.clone(), if_map.clone(), false)?;
            }
            for mut tpl in &*UnorderedMap::toList(if_map.clone()) {
                let mut tpl = tpl.clone();
                (cref, rhs) = tpl.clone();
                stmts = cons(Arc::new(BEquation::WhenStatement::WhenStatement::ASSIGN { lhs: Expression::fromCref(cref.clone(), false)?, rhs: rhs.clone(), source: var_field!((*eq).source, FEquation::NFEquation::IF).clone() }), stmts.clone());
            }
            stmts.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.lowerWhenBranchStatement")); __mm_s.push_str(&*literal!(" failed for:\n")); __mm_s.push_str(&*FEquation::toString(eq.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(stmts)
}

fn lowerWhenBranchIf(mut branch: Arc<FEquation::Branch::Branch>, mut if_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>, mut first: bool) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(branch.clone()) {
        Deref @ FEquation::Branch::BRANCH { .. } => {
            let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            for mut eq in &*var_field!((*branch).body, FEquation::Branch::Branch::BRANCH).clone() {
                let mut eq = eq.clone();
                let _ = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ FEquation::EQUALITY { lhs: Deref @ Expression::CREF { cref, .. }, .. } => {
            exp = (::match_deref::match_deref! { match &(UnorderedMap::get(cref.clone(), if_map.clone())) {
        Some(exp) if (!(first.clone())) => Arc::new(Expression::NFExpression::IF { ty: Expression::typeOf(exp.clone()), condition: var_field!((*branch).condition, FEquation::Branch::Branch::BRANCH).clone(), trueBranch: var_field!((*eq).rhs, FEquation::NFEquation::EQUALITY).clone(), falseBranch: exp.clone() }),
        None if (first.clone()) => var_field!((*eq).rhs, FEquation::NFEquation::EQUALITY).clone(),
        Some(_) => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.lowerWhenBranchIf")); __mm_s.push_str(&*literal!(" failed because branch has multiple assignments for the same cref:\n")); __mm_s.push_str(&*FEquation::Branch::toString(branch.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.lowerWhenBranchIf")); __mm_s.push_str(&*literal!(" failed because branch equation has an assignment that is missing in other branches:\n")); __mm_s.push_str(&*FEquation::toString(eq.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            UnorderedMap::add(cref.clone(), exp.clone(), if_map.clone())?;
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.lowerWhenBranchIf")); __mm_s.push_str(&*literal!(" failed for branch equation:\n")); __mm_s.push_str(&*FEquation::toString(eq.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.lowerWhenBranchIf")); __mm_s.push_str(&*literal!(" failed for:\n")); __mm_s.push_str(&*FEquation::Branch::toString(branch.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn lowerAlgorithm(mut alg: Arc<Algorithm::NFAlgorithm>, mut init: bool) -> Pointer::Pointer<Arc<Equation::Equation>> {
    let mut eq: Pointer::Pointer<Arc<Equation::Equation>>;
    let mut size: i32 = 0;
    let mut inputs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut outputs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut attr: Arc<EquationAttributes::EquationAttributes> = Arc::new(<EquationAttributes::EquationAttributes as ::std::default::Default>::default());
    size = {
        let mut __acc: i32 = 0;
        for mut out in (alg.outputs.clone()).into_iter().cloned() {
            let __x = ComponentRef::size(out.clone(), false, false);
            __acc += __x;
        }
        __acc
    };
    if alg.outputs.clone().is_empty() {
        attr = BEquation::default(EquationKind::EMPTY.clone(), init.clone(), None, None);
    } else if Algorithm::isDiscrete(alg.clone()) {
        attr = BEquation::default(EquationKind::DISCRETE.clone(), init.clone(), None, None);
    } else {
        attr = BEquation::default(EquationKind::CONTINUOUS.clone(), init.clone(), None, None);
    }
    eq = Pointer::create(Arc::new(Equation::Equation::ALGORITHM { size: size.clone(), alg: alg.clone(), source: alg.source.clone(), expand: openmodelica_frontend_types::DAE::Expand::EXPAND, attr: attr.clone() }));
    eq
}

pub fn lowerEquationAttributes(mut ty: Arc<Type::NFType>, mut init: bool) -> Arc<EquationAttributes::EquationAttributes> {
    let mut attr: Arc<EquationAttributes::EquationAttributes> = Arc::new(<EquationAttributes::EquationAttributes as ::std::default::Default>::default());
    if Type::isClock(ty.clone()) {
        attr = BEquation::default(EquationKind::CLOCKED.clone(), init.clone(), Some(-1), None);
    } else if Type::isDiscrete(ty.clone()) {
        attr = BEquation::default(EquationKind::DISCRETE.clone(), init.clone(), None, None);
    } else {
        attr = BEquation::default(EquationKind::CONTINUOUS.clone(), init.clone(), None, None);
    }
    attr
}

fn lowerComponentReferences(mut equations: Arc<EquationPointers::EquationPointers>, mut variables: Arc<VariablePointers::VariablePointers>) -> Result<Arc<EquationPointers::EquationPointers>> {
    let mut equations: Arc<EquationPointers::EquationPointers> = equations;
    equations = BEquation::EquationPointers::mapExp(equations.clone(), Arc::new({ let __pe_b1 = variables.clone(); let __pe_b2 = true; move |__pe_a0| lowerComponentReferenceExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }), Some({ let __pe_b1 = variables.clone(); let __pe_b2 = true; move |__pe_a0| lowerComponentReference(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }), (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    Ok(equations)
}

pub fn lowerComponentReferenceExp(mut exp: Arc<Expression::NFExpression>, mut variables: Arc<VariablePointers::VariablePointers>, mut complete: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } if (!(ComponentRef::isNameNode(var_field!((*exp).cref, Expression::NFExpression::CREF).clone()))) => {
            Arc::new(Expression::NFExpression::CREF { ty: var_field!((*exp).ty, Expression::NFExpression::CREF).clone(), cref: lowerComponentReference(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), variables.clone(), complete.clone())? })
        },
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } } => {
            let mut call = (*call).clone();
            assign_variant_field!(call => Call::NFCall::TYPED_ARRAY_CONSTRUCTOR; iters = {
        let mut __acc: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
        for mut tpl in (var_field!((*call).iters, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone()).into_iter().cloned() {
            let __x = Util::applyTuple21(tpl.clone(), Arc::new({ let __pe_b1 = variables.clone(); move |__pe_a0| lowerInstNode(__pe_a0, __pe_b1.clone()) }));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            assign_variant_field!(exp => Expression::NFExpression::CALL; call = call.clone());
            exp.clone()
        },
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_REDUCTION { .. } } => {
            let mut call = (*call).clone();
            assign_variant_field!(call => Call::NFCall::TYPED_REDUCTION; iters = {
        let mut __acc: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
        for mut tpl in (var_field!((*call).iters, Call::NFCall::TYPED_REDUCTION).clone()).into_iter().cloned() {
            let __x = Util::applyTuple21(tpl.clone(), Arc::new({ let __pe_b1 = variables.clone(); move |__pe_a0| lowerInstNode(__pe_a0, __pe_b1.clone()) }));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            assign_variant_field!(exp => Expression::NFExpression::CALL; call = call.clone());
            exp.clone()
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exp = Expression::applyToType(exp.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Dimension::NFDimension>) -> Result<Arc<Dimension::NFDimension>> + 'static> = Arc::new({ let __pe_b1 = variables.clone(); let __pe_b2 = complete.clone(); move |__pe_a0| lowerDimension(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }); move |__pe_a0| Type::applyToDims(__pe_a0, __pe_b1.clone()) }))?;
    Ok(exp)
}

pub fn lowerComponentReference(mut cref: Arc<ComponentRef::NFComponentRef>, mut variables: Arc<VariablePointers::VariablePointers>, mut complete: bool) -> Result<Arc<ComponentRef::NFComponentRef>> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = cref;
    let mut var: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut subs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>> = metamodelica::nil();
    if '__try0: {
        if !(ComponentRef::isWild(cref.clone())) {
            var = unwrap_break_err!(BVariable::VariablePointers::getVarSafe(variables.clone(), ComponentRef::stripSubscriptsAll(cref.clone()), if (complete.clone()) {Some(metamodelica::sourceInfo!())} else {None}), '__try0);
            cref = unwrap_break_err!(lowerComponentReferenceInstNode(cref.clone(), var.clone()), '__try0);
            cref = ComponentRef::mapSubscripts(cref.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = Arc::new({ let __pe_b1 = variables.clone(); let __pe_b2 = complete.clone(); move |__pe_a0| lowerComponentReferenceExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }); move |__pe_a0| Subscript::mapExp(__pe_a0, __pe_b1.clone()) }), false);
        }
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        if Flags::isSet(Flags::FAILTRACE.clone())? && complete.clone() {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.lowerComponentReference")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); ArcStr::from(__mm_s) }).clone()])?;
        }
    }
    Ok(cref)
}

fn lowerDimension(mut dim: Arc<Dimension::NFDimension>, mut variables: Arc<VariablePointers::VariablePointers>, mut complete: bool) -> Result<Arc<Dimension::NFDimension>> {
    let mut dim: Arc<Dimension::NFDimension> = dim;
    dim = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ Dimension::RESIZABLE { .. } => {
            assign_variant_field!(dim => Dimension::NFDimension::RESIZABLE; exp = Expression::map(var_field!((*dim).exp, Dimension::NFDimension::RESIZABLE).clone(), Arc::new({ let __pe_b1 = variables.clone(); let __pe_b2 = complete.clone(); move |__pe_a0| lowerComponentReferenceExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?);
            dim.clone()
        },
        _ => dim.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dim)
}

fn collectIterators(mut exp: Arc<Expression::NFExpression>, mut variables: Arc<VariablePointers::VariablePointers>, mut set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    if '__try0: {
        let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } if (!(BVariable::VariablePointers::containsCref(ComponentRef::stripSubscriptsAll(var_field!((*exp).cref, Expression::NFExpression::CREF).clone()), variables.clone()) || ComponentRef::isNameNode(var_field!((*exp).cref, Expression::NFExpression::CREF).clone()) || ComponentRef::isWild(var_field!((*exp).cref, Expression::NFExpression::CREF).clone()))) => {
            unwrap_break_err!(UnorderedSet::add(lowerIterator(var_field!((*exp).cref, Expression::NFExpression::CREF).clone()), set.clone()), '__try0);
            ()
        },
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } } => {
            for mut tpl in &*var_field!((**call).iters, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone() {
                let mut tpl = tpl.clone();
                unwrap_break_err!(collectIterator(Util::tuple21(tpl.clone()), variables.clone(), set.clone()), '__try0);
            }
            ()
        },
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_REDUCTION { .. } } => {
            for mut tpl in &*var_field!((**call).iters, Call::NFCall::TYPED_REDUCTION).clone() {
                let mut tpl = tpl.clone();
                unwrap_break_err!(collectIterator(Util::tuple21(tpl.clone()), variables.clone(), set.clone()), '__try0);
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.collectIterators")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); ArcStr::from(__mm_s) }).clone()])?;
        bail!("fail");
    }
    Ok(exp)
}

fn collectIterator(mut iterator: Arc<InstNode::InstNode>, mut variables: Arc<VariablePointers::VariablePointers>, mut set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>) -> Result<()> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    cref = ComponentRef::fromNode(iterator.clone(), InstNode::getType(iterator.clone())?, metamodelica::nil(), ComponentRef::Origin::ITERATOR.clone());
    cref = ComponentRef::stripSubscriptsAll(cref.clone());
    if !(BVariable::VariablePointers::containsCref(cref.clone(), variables.clone())) {
        UnorderedSet::add(lowerIterator(cref.clone()), set.clone())?;
    }
    Ok(())
}

fn lowerInstNode(mut node: Arc<InstNode::InstNode>, mut variables: Arc<VariablePointers::VariablePointers>) -> Result<Arc<InstNode::InstNode>> {
    let mut node: Arc<InstNode::InstNode> = node;
    let mut cref: Arc<ComponentRef::NFComponentRef> = ComponentRef::fromNode(node.clone(), Arc::new(openmodelica_nf_frontend::NFType::INTEGER), metamodelica::nil(), ComponentRef::Origin::ITERATOR.clone());
    let mut var: Pointer::Pointer<Arc<Variable::NFVariable>>;
    var = BVariable::VariablePointers::getVarSafe(variables.clone(), ComponentRef::stripSubscriptsAll(cref.clone()), Some(metamodelica::sourceInfo!()))?;
    node = Arc::new(InstNode::InstNode::VAR_NODE { name: (InstNode::name(node.clone())?).clone(), varPointer: var.clone() });
    Ok(node)
}

pub fn lowerComponentReferenceInstNode(mut cref: Arc<ComponentRef::NFComponentRef>, mut var: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<Arc<ComponentRef::NFComponentRef>> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = cref;
    cref = (::match_deref::match_deref! { match &(cref.clone()) {
        qual @ Deref @ ComponentRef::CREF { .. } => {
            let mut qual = (*qual).clone();
            assign_variant_field!(qual => ComponentRef::NFComponentRef::CREF; node = Arc::new(InstNode::InstNode::VAR_NODE { name: (InstNode::name(var_field!((*qual).node, ComponentRef::NFComponentRef::CREF).clone())?).clone(), varPointer: var.clone() }));
            qual.clone()
        },
        _ => {
            cref.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cref)
}

pub fn lowerEquationIterators(mut eqn: Arc<Equation::Equation>, mut variables: Arc<VariablePointers::VariablePointers>, mut set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>) -> Result<Arc<Equation::Equation>> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    let mut iter: Arc<Iterator::Iterator> = BEquation::Equation::getForIterator(eqn.clone());
    let mut iterators: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    (iterators, _, _) = BEquation::Iterator::getFrames(iter.clone())?;
    for mut iter in &*iterators.clone() {
        let mut iter = iter.clone();
        UnorderedSet::add(lowerIterator(iter.clone()), set.clone())?;
    }
    BEquation::Equation::map(eqn.clone(), Arc::new({ let __pe_b1 = variables.clone(); let __pe_b2 = set.clone(); move |__pe_a0| collectIterators(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    Ok(eqn)
}

pub fn lowerIterator(mut iterator: Arc<ComponentRef::NFComponentRef>) -> Pointer::Pointer<Arc<Variable::NFVariable>> {
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>> = lowerVariable(Variable::fromCref(iterator.clone()).unwrap()).unwrap();
    var_ptr
}

pub fn lowerIteratorCref(mut iterator: Arc<ComponentRef::NFComponentRef>) -> Arc<ComponentRef::NFComponentRef> {
    let mut iterator: Arc<ComponentRef::NFComponentRef> = iterator;
    iterator = BVariable::getVarName(lowerIterator(iterator.clone()));
    iterator
}

pub fn lowerIteratorExp(mut exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => {
            assign_variant_field!(exp => Expression::NFExpression::CREF; cref = lowerIteratorCref(var_field!((*exp).cref, Expression::NFExpression::CREF).clone()));
            exp.clone()
        },
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exp
}

pub fn lowerFunctions(mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>) -> Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>> {
    let mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>> = funcMap;
    UnorderedMap::apply(funcMap.clone(), Arc::new({ let __pe_b1 = funcMap.clone(); move |__pe_a0| Differentiate::resolvePartialDerivatives(__pe_a0, __pe_b1.clone()) }));
    funcMap
}

pub fn backenddaeinfo(mut bdae: Arc<NBackendDAE>) -> Result<()> {
    if Flags::isSet(Flags::DUMP_BACKENDDAE_INFO.clone())? {
        let _ = (::match_deref::match_deref! { match &(bdae.clone()) {
        Deref @ MAIN { eqData: Deref @ BEquation::EqData::EQ_DATA_SIM { .. }, varData: varData @ Deref @ BVariable::VarData::VAR_DATA_SIM { .. }, .. } => {
            let mut p_ode: ArcStr = arcstr::literal!("");
            let mut p_alg: ArcStr = arcstr::literal!("");
            let mut p_ode_e: ArcStr = arcstr::literal!("");
            let mut p_alg_e: ArcStr = arcstr::literal!("");
            let mut p_clk: ArcStr = arcstr::literal!("");
            let mut p_ini: ArcStr = arcstr::literal!("");
            let mut p_ini_0: ArcStr = arcstr::literal!("");
            let mut states: ArcStr = arcstr::literal!("");
            let mut discretes: ArcStr = arcstr::literal!("");
            let mut discrete_states: ArcStr = arcstr::literal!("");
            let mut clocked_states: ArcStr = arcstr::literal!("");
            let mut clocks: ArcStr = arcstr::literal!("");
            let mut inputs: ArcStr = arcstr::literal!("");
            p_ode = (intString((var_field!((*bdae).ode, NBackendDAE::MAIN).clone().len() as i32))).clone();
            p_alg = (intString((var_field!((*bdae).algebraic, NBackendDAE::MAIN).clone().len() as i32))).clone();
            p_ode_e = (intString((var_field!((*bdae).ode_event, NBackendDAE::MAIN).clone().len() as i32))).clone();
            p_alg_e = (intString((var_field!((*bdae).alg_event, NBackendDAE::MAIN).clone().len() as i32))).clone();
            p_clk = (literal!("0")).clone();
            p_ini = (intString((var_field!((*bdae).init, NBackendDAE::MAIN).clone().len() as i32))).clone();
            p_ini_0 = (if (isSome(var_field!((*bdae).init_0, NBackendDAE::MAIN).clone())) {intString((Util::getOption(var_field!((*bdae).init_0, NBackendDAE::MAIN).clone())?.len() as i32))} else {literal!("0")}).clone();
            states = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(BVariable::VariablePointers::scalarSize(var_field!((**varData).states, VarData::VarData::VAR_DATA_SIM).clone(), false))); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(BVariable::VariablePointers::size(var_field!((**varData).states, VarData::VarData::VAR_DATA_SIM).clone()))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            discretes = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(BVariable::VariablePointers::scalarSize(var_field!((**varData).discretes, VarData::VarData::VAR_DATA_SIM).clone(), false))); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(BVariable::VariablePointers::size(var_field!((**varData).discretes, VarData::VarData::VAR_DATA_SIM).clone()))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            discrete_states = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(BVariable::VariablePointers::scalarSize(var_field!((**varData).discrete_states, VarData::VarData::VAR_DATA_SIM).clone(), false))); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(BVariable::VariablePointers::size(var_field!((**varData).discrete_states, VarData::VarData::VAR_DATA_SIM).clone()))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            clocked_states = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(BVariable::VariablePointers::scalarSize(var_field!((**varData).clocked_states, VarData::VarData::VAR_DATA_SIM).clone(), false))); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(BVariable::VariablePointers::size(var_field!((**varData).clocked_states, VarData::VarData::VAR_DATA_SIM).clone()))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            clocks = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(BVariable::VariablePointers::scalarSize(var_field!((**varData).clocks, VarData::VarData::VAR_DATA_SIM).clone(), false))); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(BVariable::VariablePointers::size(var_field!((**varData).clocks, VarData::VarData::VAR_DATA_SIM).clone()))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            inputs = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(BVariable::VariablePointers::scalarSize(var_field!((**varData).top_level_inputs, VarData::VarData::VAR_DATA_SIM).clone(), false))); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(BVariable::VariablePointers::size(var_field!((**varData).top_level_inputs, VarData::VarData::VAR_DATA_SIM).clone()))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            if Flags::isSet(Flags::DUMP_STATESELECTION_INFO.clone())? {
                states = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*states.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*List::toString(BVariable::VariablePointers::toList(var_field!((**varData).states, VarData::VarData::VAR_DATA_SIM).clone())?, (std::sync::Arc::new(fnptr!(BVariable::nameString, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone();
            } else {
                states = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*states.clone()); __mm_s.push_str(&*literal!(" ('-d=stateselection' for the list of states)")); ArcStr::from(__mm_s) }).clone();
            }
            if Flags::isSet(Flags::DUMP_DISCRETEVARS_INFO.clone())? {
                discretes = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*discretes.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*List::toString(BVariable::VariablePointers::toList(var_field!((**varData).discretes, VarData::VarData::VAR_DATA_SIM).clone())?, (std::sync::Arc::new(fnptr!(BVariable::nameString, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone();
                clocks = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*clocks.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*List::toString(BVariable::VariablePointers::toList(var_field!((**varData).clocks, VarData::VarData::VAR_DATA_SIM).clone())?, (std::sync::Arc::new(fnptr!(BVariable::nameString, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone();
                inputs = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inputs.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*List::toString(BVariable::VariablePointers::toList(var_field!((**varData).top_level_inputs, VarData::VarData::VAR_DATA_SIM).clone())?, (std::sync::Arc::new(fnptr!(BVariable::nameString, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone();
            } else {
                discretes = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*discretes.clone()); __mm_s.push_str(&*literal!(" ('-d=discreteinfo' for the list of discrete variables)")); ArcStr::from(__mm_s) }).clone();
                clocks = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*clocks.clone()); __mm_s.push_str(&*literal!(" ('-d=discreteinfo' for the list of clocks variables)")); ArcStr::from(__mm_s) }).clone();
                inputs = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inputs.clone()); __mm_s.push_str(&*literal!(" ('-d=discreteinfo' for the list of top level inputs)")); ArcStr::from(__mm_s) }).clone();
            }
            if Flags::isSet(Flags::DUMP_STATESELECTION_INFO.clone())? || Flags::isSet(Flags::DUMP_DISCRETEVARS_INFO.clone())? {
                discrete_states = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*discrete_states.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*List::toString(BVariable::VariablePointers::toList(var_field!((**varData).discrete_states, VarData::VarData::VAR_DATA_SIM).clone())?, (std::sync::Arc::new(fnptr!(BVariable::nameString, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone();
                clocked_states = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*clocked_states.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*List::toString(BVariable::VariablePointers::toList(var_field!((**varData).clocked_states, VarData::VarData::VAR_DATA_SIM).clone())?, (std::sync::Arc::new(fnptr!(BVariable::nameString, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone();
            } else {
                discrete_states = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*discrete_states.clone()); __mm_s.push_str(&*literal!(" ('-d=discreteinfo' or '-d=stateselection' for the list of discrete states)")); ArcStr::from(__mm_s) }).clone();
                clocked_states = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*clocked_states.clone()); __mm_s.push_str(&*literal!(" ('-d=discreteinfo' or '-d=stateselection' for the list of clocked states)")); ArcStr::from(__mm_s) }).clone();
            }
            Error::addCompilerNotification(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Partition statistics after passing the back-end:\n")); __mm_s.push_str(&*literal!(" * Number of ODE partitions: ..................... ")); __mm_s.push_str(&*p_ode.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!(" * Number of algebraic partitions: ............... ")); __mm_s.push_str(&*p_alg.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!(" * Number of ODE event partitions: ............... ")); __mm_s.push_str(&*p_ode_e.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!(" * Number of algebraic event partitions: ......... ")); __mm_s.push_str(&*p_alg_e.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!(" * Number of clocked partitions: ................. ")); __mm_s.push_str(&*p_clk.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!(" * Number of initial partitions: ................. ")); __mm_s.push_str(&*p_ini.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!(" * Number of initial(lambda=0) partitions: ....... ")); __mm_s.push_str(&*p_ini_0.clone()); ArcStr::from(__mm_s) }).clone())?;
            Error::addCompilerNotification(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable statistics after passing the back-end:\n")); __mm_s.push_str(&*literal!(" * Number of states: ............................. ")); __mm_s.push_str(&*states.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!(" * Number of discrete states: .................... ")); __mm_s.push_str(&*discrete_states.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!(" * Number of clocked states: ..................... ")); __mm_s.push_str(&*clocked_states.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!(" * Number of discrete variables: ................. ")); __mm_s.push_str(&*discretes.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!(" * Number of clocks: ............................. ")); __mm_s.push_str(&*clocks.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!(" * Number of top-level inputs: ................... ")); __mm_s.push_str(&*inputs.clone()); ArcStr::from(__mm_s) }).clone())?;
            strongcomponentinfo((literal!("Simulation")).clone(), list![var_field!((*bdae).ode, NBackendDAE::MAIN).clone(), var_field!((*bdae).algebraic, NBackendDAE::MAIN).clone(), var_field!((*bdae).ode_event, NBackendDAE::MAIN).clone(), var_field!((*bdae).alg_event, NBackendDAE::MAIN).clone()])?;
            strongcomponentinfo((literal!("Initialization")).clone(), list![var_field!((*bdae).init, NBackendDAE::MAIN).clone()])?;
            if Util::isSome(var_field!((*bdae).init_0, NBackendDAE::MAIN).clone()) {
                strongcomponentinfo((literal!("Initialization (lambda=0)")).clone(), list![Util::getOption(var_field!((*bdae).init_0, NBackendDAE::MAIN).clone())?])?;
            }
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    }
    Ok(())
}

pub fn strongcomponentinfo(mut phase: ArcStr, mut systems: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Partition::Partition>>>>>) -> Result<()> {
    let mut c: CountCollector = Arc::new(CountCollector { single_scalar: 0, single_array: 0, single_record: 0, multi_algorithm: 0, multi_when: 0, multi_if: 0, multi_tpl: 0, resizable_for: 0, generic_for: 0, entwined_for: 0, loop_lin: 0, loop_nlin: 0 });
    let mut collector_ptr: Pointer::Pointer<CountCollector> = Pointer::create(c.clone());
    let mut single_sc: ArcStr = arcstr::literal!("");
    let mut multi_sc: ArcStr = arcstr::literal!("");
    let mut for_sc: ArcStr = arcstr::literal!("");
    let mut alg_sc: ArcStr = arcstr::literal!("");
    for mut lst in &*systems.clone() {
        let mut lst = lst.clone();
        for mut system in &*lst.clone() {
            let mut system = system.clone();
            NBPartition::Partition::mapStrongComponents(system.clone(), Arc::new({ let __pe_b1 = collector_ptr.clone(); move |__pe_a0| StrongComponent::strongComponentInfo(__pe_a0, __pe_b1.clone()) }))?;
        }
    }
    c = Pointer::access(collector_ptr.clone());
    single_sc = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(c.single_scalar.clone() + c.single_array.clone() + c.single_record.clone())); __mm_s.push_str(&*literal!(" (scalar:")); __mm_s.push_str(&*intString(c.single_scalar.clone())); __mm_s.push_str(&*literal!(", array:")); __mm_s.push_str(&*intString(c.single_array.clone())); __mm_s.push_str(&*literal!(", record:")); __mm_s.push_str(&*intString(c.single_record.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
    multi_sc = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(c.multi_algorithm.clone() + c.multi_when.clone() + c.multi_if.clone())); __mm_s.push_str(&*literal!(" (algorithm:")); __mm_s.push_str(&*intString(c.multi_algorithm.clone())); __mm_s.push_str(&*literal!(", when:")); __mm_s.push_str(&*intString(c.multi_when.clone())); __mm_s.push_str(&*literal!(", if:")); __mm_s.push_str(&*intString(c.multi_if.clone())); __mm_s.push_str(&*literal!(", tuple:")); __mm_s.push_str(&*intString(c.multi_tpl.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
    for_sc = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(c.resizable_for.clone() + c.generic_for.clone() + c.entwined_for.clone())); __mm_s.push_str(&*literal!(" (resizable: ")); __mm_s.push_str(&*intString(c.resizable_for.clone())); __mm_s.push_str(&*literal!(", generic: ")); __mm_s.push_str(&*intString(c.generic_for.clone())); __mm_s.push_str(&*literal!(", entwined:")); __mm_s.push_str(&*intString(c.entwined_for.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
    alg_sc = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(c.loop_lin.clone() + c.loop_nlin.clone())); __mm_s.push_str(&*literal!(" (linear: ")); __mm_s.push_str(&*intString(c.loop_lin.clone())); __mm_s.push_str(&*literal!(", nonlinear:")); __mm_s.push_str(&*intString(c.loop_nlin.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
    Error::addCompilerNotification(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*phase.clone()); __mm_s.push_str(&*literal!("] Strong Component statistics after passing the back-end:\n")); __mm_s.push_str(&*literal!(" * Number of single strong components: ........... ")); __mm_s.push_str(&*single_sc.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!(" * Number of multi strong components: ............ ")); __mm_s.push_str(&*multi_sc.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!(" * Number of for-loop strong components: ......... ")); __mm_s.push_str(&*for_sc.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!(" * Number of algebraic-loop strong components: ... ")); __mm_s.push_str(&*alg_sc.clone()); ArcStr::from(__mm_s) }).clone())?;
    Ok(())
}

pub fn debugFollowEquations(mut bdae: Arc<NBackendDAE>, mut eq_filter_opt: Option<Arc<UnorderedSet::UnorderedSet<ArcStr>>>, mut r#str: ArcStr) -> Result<()> {
    let _ = ({
        let mut tmp: ArcStr = literal!("");
        (::match_deref::match_deref! { match &(bdae.clone()) {
        Deref @ MAIN { .. } => {
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_1(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[debugFollowEquations]: ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*BEquation::EqData::toString(var_field!((*bdae).eqData, NBackendDAE::MAIN).clone(), 1, eq_filter_opt.clone())?); ArcStr::from(__mm_s) }).clone();
            println!("{}", (tmp.clone()).clone());
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
    Ok(())
}

pub fn debugLowering(mut bdae: Arc<NBackendDAE>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(bdae.clone()) {
        Deref @ MAIN { .. } => {
            BEquation::EqData::map(var_field!((*bdae).eqData, NBackendDAE::MAIN).clone(), (std::sync::Arc::new(checkLoweredCrefEqn) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::Equation>) -> Result<Arc<Equation::Equation>> + 'static>))?;
            BVariable::VariablePointers::mapPtr(BVariable::VarData::getVariables(var_field!((*bdae).varData, NBackendDAE::MAIN).clone())?, (std::sync::Arc::new(checkLoweredCrefVar) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<()> + 'static>))?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn checkLoweredCrefVar(mut var: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<()> {
    let mut set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
    BVariable::mapExp(var.clone(), Arc::new({ let __pe_b1 = set.clone(); move |__pe_a0| checkLoweredCrefExp(__pe_a0, __pe_b1.clone()) }), (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    if !(UnorderedSet::isEmpty(set.clone())) {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[failtrace] the variable:\n")); __mm_s.push_str(&*BVariable::pointerToString(var.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[failtrace] has following non-lowered component references: ")); __mm_s.push_str(&*List::toString(UnorderedSet::toList(set.clone()), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(())
}

pub fn checkLoweredCrefEqn(mut eqn: Arc<Equation::Equation>) -> Result<Arc<Equation::Equation>> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    let mut set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
    BEquation::Equation::map(eqn.clone(), Arc::new({ let __pe_b1 = set.clone(); move |__pe_a0| checkLoweredCrefExp(__pe_a0, __pe_b1.clone()) }), Some({ let __pe_b1 = set.clone(); move |__pe_a0| checkLoweredCref(__pe_a0, __pe_b1.clone()) }), (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    if !(UnorderedSet::isEmpty(set.clone())) {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[failtrace] the equation:\n")); __mm_s.push_str(&*BEquation::Equation::toString(eqn.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[failtrace] has following non-lowered component references: ")); __mm_s.push_str(&*List::toString(UnorderedSet::toList(set.clone()), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(eqn)
}

pub fn checkLoweredCrefExp(mut exp: Arc<Expression::NFExpression>, mut set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let _ = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => {
            checkLoweredCref(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), set.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn checkLoweredCref(mut cref: Arc<ComponentRef::NFComponentRef>, mut set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<ComponentRef::NFComponentRef>> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = cref;
    let _ = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ ComponentRef::CREF { node: Deref @ InstNode::VAR_NODE { .. }, .. } => (),
        Deref @ ComponentRef::CREF { node: Deref @ InstNode::NAME_NODE { .. }, .. } => (),
        Deref @ ComponentRef::CREF { .. } => {
            UnorderedSet::add(cref.clone(), set.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cref)
}


