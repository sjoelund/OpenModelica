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

use crate::NBAdjacency as Adjacency;
use crate::NBEquation as BEquation;
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationPointers;
use crate::NBEvents::EventInfo;
use crate::NBInline as Inline;
use crate::NBJacobian::JacobianType;
use crate::NBPartition as Partition;
use crate::NBPartitioning::ClockedInfo;
use crate::NBStrongComponent as StrongComponent;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as Jacobian;
use openmodelica_ast::Absyn::Path;
use openmodelica_frontend_types::DAE;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util_datatypes_basic::Pointer;

// OF imports
// NF imports
// Backend imports
// Util imports
pub type wrapper = std::sync::Arc<dyn ::std::ops::Fn(Arc<Jacobian::NBackendDAE>) -> Result<Arc<Jacobian::NBackendDAE>> + 'static>;

pub fn moduleClockString(mut name_clock: (ArcStr, metamodelica::Real)) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut name: ArcStr = arcstr::literal!("");
    let mut clck: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    (name, clck) = name_clock.clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\t")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*StringUtil::repeat((literal!(".")).clone(), 50 - ((name.clone()).clone().len() as i32))); __mm_s.push_str(&*System::sprintff((literal!("%.4g")).clone(), clck.clone())?); ArcStr::from(__mm_s) }).clone();
    Ok(r#str)
}

// =========================================================================
//                                MAIN MODULES
// =========================================================================
//                               PARTITIONING
// *************************************************************************
pub type partitioningInterface = std::sync::Arc<dyn ::std::ops::Fn(Partition::Kind, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Arc<ClockedInfo::ClockedInfo>) -> Result<Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>> + 'static>;

//                               CAUSALIZE
// *************************************************************************
pub type causalizeInterface = std::sync::Arc<dyn ::std::ops::Fn(Arc<Partition::Partition::Partition>, Arc<VarData::VarData>, Arc<EqData::EqData>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>) -> Result<(Arc<Partition::Partition::Partition>, Arc<VarData::VarData>, Arc<EqData::EqData>)> + 'static>;

//                           RESOLVING SINGULARITIES
//                  Index Reduction + Balance Initialization
// *************************************************************************
//                               DAEMODE
// *************************************************************************
pub type daeModeInterface = std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>, Arc<VariablePointers::VariablePointers>, Pointer::Pointer<i32>) -> Result<Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>> + 'static>;

// =========================================================================
//                         MANDATORY PRE-OPT MODULES
// =========================================================================
//                            COLLECT EVENTS
// *************************************************************************
pub type eventsInterface = std::sync::Arc<dyn ::std::ops::Fn(Arc<VarData::VarData>, Arc<EqData::EqData>, Arc<EventInfo::EventInfo>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>) -> Result<(Arc<VarData::VarData>, Arc<EqData::EqData>, Arc<EventInfo::EventInfo>)> + 'static>;

//                               DETECT STATES
// *************************************************************************
pub type detectStatesInterface = std::sync::Arc<dyn ::std::ops::Fn(Arc<VarData::VarData>, Arc<EqData::EqData>, detectContinuousStatesInterface, detectDiscreteStatesInterface) -> Result<(Arc<VarData::VarData>, Arc<EqData::EqData>)> + 'static>;

pub type detectContinuousStatesInterface = std::sync::Arc<dyn ::std::ops::Fn(Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>) -> Result<(Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>)> + 'static>;

pub type detectDiscreteStatesInterface = std::sync::Arc<dyn ::std::ops::Fn(Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, ArcStr) -> Result<(Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>)> + 'static>;

// =========================================================================
//                         Optional PRE-OPT MODULES
// =========================================================================
//                                 ALIAS
// *************************************************************************
pub type functionAliasInterface = std::sync::Arc<dyn ::std::ops::Fn(Arc<VarData::VarData>, Arc<EqData::EqData>, Partition::Kind) -> Result<(Arc<VarData::VarData>, Arc<EqData::EqData>)> + 'static>;

pub type aliasInterface = std::sync::Arc<dyn ::std::ops::Fn(Arc<VarData::VarData>, Arc<EqData::EqData>, Partition::Kind) -> Result<(Arc<VarData::VarData>, Arc<EqData::EqData>)> + 'static>;

//                                 INLINE
// *************************************************************************
pub type inlineInterface = std::sync::Arc<dyn ::std::ops::Fn(Arc<EqData::EqData>, Arc<VarData::VarData>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, Arc<metamodelica::List<DAE::InlineType>>, bool) -> Result<(Arc<EqData::EqData>, Arc<VarData::VarData>)> + 'static>;

// =========================================================================
//                         MANDATORY POST-OPT MODULES
// =========================================================================
//                               JACOBIAN
// *************************************************************************
pub type jacobianInterface = std::sync::Arc<dyn ::std::ops::Fn(ArcStr, JacobianType, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Option<metamodelica::Array<Arc<StrongComponent::NBStrongComponent>>>, Option<Arc<Adjacency::Matrix::Matrix>>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, bool) -> Result<Option<Arc<Jacobian::NBackendDAE>>> + 'static>;

// =========================================================================
//                         Optional POST-OPT MODULES
// =========================================================================
//                                 TEARING
// *************************************************************************
pub type tearingInterface = std::sync::Arc<dyn ::std::ops::Fn(Arc<StrongComponent::NBStrongComponent>, Arc<Adjacency::Matrix::Matrix>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, i32, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Pointer::Pointer<i32>, Partition::Kind) -> Result<(Arc<StrongComponent::NBStrongComponent>, Arc<Adjacency::Matrix::Matrix>, i32)> + 'static>;

