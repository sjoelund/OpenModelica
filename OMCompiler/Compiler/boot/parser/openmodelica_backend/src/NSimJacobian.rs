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

use crate::NBEquation as BEquation;
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationPointer;
use crate::NBEquation::EquationPointers;
use crate::NBJacobian as Jacobian;
use crate::NBPartition as Partition;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use crate::NSimCode as SimCode;
use crate::NSimCode::Identifier;
use crate::NSimCodeUtil as SimCodeUtil;
use crate::NSimGenericCall as SimGenericCall;
use crate::NSimStrongComponent as SimStrongComponent;
use crate::NSimVar::SimVar;
use crate::NSimVar::SimVars;
use crate::NSimVar::VarType;
use crate::SimCode as OldSimCode;
use crate::SimCodeVar;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFInstNode::InstNode;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

// NF imports
// Backend imports
// SimCode imports
// Old SimCode imports
// Util imports
pub type SparsityPattern = Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>;

pub type SparsityColoring = Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;

pub mod SimJacobian {
    use super::*;
#[derive(Clone, Debug, PartialEq)]
    pub struct SimJacobian {
        /// unique matrix name
        pub name: ArcStr,
        /// unique jacobian index
        pub jacobianIndex: i32,
        /// index of partition it belongs to
        pub partitionIndex: i32,
        /// corresponds to the number of rows
        pub numberOfResultVars: i32,
        /// column equations equals in size to column vars
        pub columnEqns: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>,
        /// List of constant equations independent of seed variables
        pub constantEqns: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>,
        /// all column vars, none results vars index -1, the other corresponding to rows index
        pub columnVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        /// corresponds to the number of columns
        pub seedVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        /// sparsity pattern in index form
        pub sparsity: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>,
        /// transposed sparsity pattern
        pub sparsityT: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>,
        /// coloring columns in index form (column coloring)
        pub coloring: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>,
        /// coloring rows in index form (row coloring)
        pub rowColoring: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>,
        /// number of colors
        pub numColors: i32,
        /// Generic for-loop and array calls
        pub generic_loop_calls: Arc<metamodelica::List<Arc<SimGenericCall::NSimGenericCall>>>,
        /// hash table for cref -> simVar
        pub jac_map: Option<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>>,
        /// indicates if this is an adjoint jacobian
        pub isAdjoint: bool,
    }

    impl Default for SimJacobian {
        fn default() -> Self {
            Self {
                name: Default::default(),
                jacobianIndex: Default::default(),
                partitionIndex: Default::default(),
                numberOfResultVars: Default::default(),
                columnEqns: Default::default(),
                constantEqns: Default::default(),
                columnVars: Default::default(),
                seedVars: Default::default(),
                sparsity: Default::default(),
                sparsityT: Default::default(),
                coloring: Default::default(),
                rowColoring: Default::default(),
                numColors: Default::default(),
                generic_loop_calls: Default::default(),
                jac_map: Default::default(),
                isAdjoint: Default::default(),
            }
        }
    }

    pub type SIM_JAC = SimJacobian;

    pub fn toString(mut simJac: Arc<SimJacobian>) -> Result<ArcStr> {
        let mut r#str: ArcStr = literal!("");
        let mut idx: i32 = 0;
        let mut dependencies: Arc<metamodelica::List<i32>> = metamodelica::nil();
        r#str = ((::match_deref::match_deref! { match &(simJac.clone()) {
        Deref @ SimJacobian { .. } => {
            if isEmpty(simJac.clone()) {
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_2(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[EMPTY] SimCode Jacobian ")); __mm_s.push_str(&*simJac.name.clone()); __mm_s.push_str(&*literal!("(idx = ")); __mm_s.push_str(&*intString(simJac.jacobianIndex.clone())); __mm_s.push_str(&*literal!(", partition = ")); __mm_s.push_str(&*intString(simJac.partitionIndex.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            } else {
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_2(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SimCode Jacobian ")); __mm_s.push_str(&*simJac.name.clone()); __mm_s.push_str(&*literal!("(idx = ")); __mm_s.push_str(&*intString(simJac.jacobianIndex.clone())); __mm_s.push_str(&*literal!(", partition = ")); __mm_s.push_str(&*intString(simJac.jacobianIndex.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SeedVars (size = ")); __mm_s.push_str(&*intString((simJac.seedVars.clone().len() as i32))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())); ArcStr::from(__mm_s) }).clone();
                for mut var in &*simJac.seedVars.clone() {
                    let mut var = var.clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*SimVar::toString(var.clone(), (literal!("  ")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                }
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TmpVars (size = ")); __mm_s.push_str(&*intString((simJac.columnVars.clone().len() as i32))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())); ArcStr::from(__mm_s) }).clone();
                for mut var in &*simJac.columnVars.clone() {
                    let mut var = var.clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*SimVar::toString(var.clone(), (literal!("  ")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                }
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ResultVars (size = ")); __mm_s.push_str(&*intString(simJac.numberOfResultVars.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())); ArcStr::from(__mm_s) }).clone();
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_3(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Column Equations (size = ")); __mm_s.push_str(&*intString((simJac.columnEqns.clone().len() as i32))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())); ArcStr::from(__mm_s) }).clone();
                for mut eq in &*simJac.columnEqns.clone() {
                    let mut eq = eq.clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*SimStrongComponent::Block::toString(eq.clone(), (literal!("  ")).clone())?); ArcStr::from(__mm_s) }).clone();
                }
                if !(simJac.constantEqns.clone().is_empty()) {
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*StringUtil::headline_3((literal!("Constant Equations")).clone())); ArcStr::from(__mm_s) }).clone();
                    for mut eq in &*simJac.constantEqns.clone() {
                        let mut eq = eq.clone();
                        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*SimStrongComponent::Block::toString(eq.clone(), (literal!("  ")).clone())?); ArcStr::from(__mm_s) }).clone();
                    }
                }
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_4((literal!("Sparsity Pattern Cols")).clone())); ArcStr::from(__mm_s) }).clone();
                if !(simJac.sparsityT.clone().is_empty()) {
                    for mut tpl in &*simJac.sparsityT.clone() {
                        let mut tpl = tpl.clone();
                        (idx, dependencies) = tpl.clone();
                        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("  ")); __mm_s.push_str(&*intString(idx.clone())); __mm_s.push_str(&*literal!(":\t")); __mm_s.push_str(&*List::toString(dependencies.clone(), Arc::new(fnptr!(intString, i32)), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                    }
                }
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_4((literal!("Sparsity Pattern Rows")).clone())); ArcStr::from(__mm_s) }).clone();
                if !(simJac.sparsity.clone().is_empty()) {
                    for mut tpl in &*simJac.sparsity.clone() {
                        let mut tpl = tpl.clone();
                        (idx, dependencies) = tpl.clone();
                        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("  ")); __mm_s.push_str(&*intString(idx.clone())); __mm_s.push_str(&*literal!(":\t")); __mm_s.push_str(&*List::toString(dependencies.clone(), Arc::new(fnptr!(intString, i32)), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                    }
                }
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_4((literal!("Sparsity Coloring Columns")).clone())); ArcStr::from(__mm_s) }).clone();
                if !(simJac.coloring.clone().is_empty()) {
                    for mut lst in &*simJac.coloring.clone() {
                        let mut lst = lst.clone();
                        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("  ")); __mm_s.push_str(&*List::toString(lst.clone(), Arc::new(fnptr!(intString, i32)), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                    }
                }
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_4((literal!("Sparsity Coloring Rows")).clone())); ArcStr::from(__mm_s) }).clone();
                if !(simJac.rowColoring.clone().is_empty()) {
                    for mut lst in &*simJac.rowColoring.clone() {
                        let mut lst = lst.clone();
                        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("  ")); __mm_s.push_str(&*List::toString(lst.clone(), Arc::new(fnptr!(intString, i32)), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                    }
                }
                if !(simJac.generic_loop_calls.clone().is_empty()) {
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*StringUtil::headline_3((literal!("Generic Calls")).clone())); ArcStr::from(__mm_s) }).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*List::toString(simJac.generic_loop_calls.clone(), Arc::new(SimGenericCall::toString), (literal!("")).clone(), (literal!("  ")).clone(), (literal!("\n  ")).clone(), (literal!("\n")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone();
                }
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            }
            r#str.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimJacobian.SimJacobian.toString")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
        Ok(r#str)
    }

    pub fn isEmpty(mut simJac: Arc<SimJacobian>) -> bool {
        let mut b: bool = false;
        b = (::match_deref::match_deref! { match &(simJac.clone()) {
        Deref @ SimJacobian { .. } => simJac.numberOfResultVars.clone() == 0,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        b
    }

    pub fn create(mut jacobian: Arc<BackendDAE::NBackendDAE>, mut indices: SimCode::SimCodeIndices, mut simcode_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>) -> Result<(Option<Arc<SimJacobian>>, SimCode::SimCodeIndices)> {
        let mut simJacobian: Option<Arc<SimJacobian>> = None;
        let mut indices: SimCode::SimCodeIndices = indices;
        simJacobian = (::match_deref::match_deref! { match &(jacobian.clone()) {
        Deref @ BackendDAE::JACOBIAN { varData: varData @ Deref @ BVariable::VarData::VAR_DATA_JAC { .. }, .. } => {
            let mut dummy_sim_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>> = UnorderedMap::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>), ComponentRef::isEqual, 1);
            let mut dummy_eqn_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimStrongComponent::Block::Block>>> = UnorderedMap::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>), ComponentRef::isEqual, 1);
            let mut columnEqn: Arc<SimStrongComponent::Block::Block>;
            let mut columnEqns: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>> = metamodelica::nil();
            let mut seed_vec: Arc<VariablePointers::VariablePointers>;
            let mut res_vec: Arc<VariablePointers::VariablePointers>;
            let mut tmp_vec: Arc<VariablePointers::VariablePointers>;
            let mut seedVars_ptr: Pointer::Pointer<Arc<metamodelica::List<Arc<SimVar::SimVar>>>> = Pointer::create(metamodelica::nil());
            let mut resVars_ptr: Pointer::Pointer<Arc<metamodelica::List<Arc<SimVar::SimVar>>>> = Pointer::create(metamodelica::nil());
            let mut tmpVars_ptr: Pointer::Pointer<Arc<metamodelica::List<Arc<SimVar::SimVar>>>> = Pointer::create(metamodelica::nil());
            let mut seedVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
            let mut resVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
            let mut tmpVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
            let mut loopVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
            let mut jac_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>;
            let mut idx_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>;
            let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut sparsity: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
            let mut sparsityT: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
            let mut coloring: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut rowColoring: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut jac: Arc<SimJacobian>;
            let mut sim_map: Arc<UnorderedMap::UnorderedMap<Arc<Identifier::Identifier>, i32>>;
            let mut generic_loop_calls: Arc<metamodelica::List<Arc<SimGenericCall::NSimGenericCall>>> = metamodelica::nil();
            sim_map = indices.generic_call_map.clone();
            indices.generic_call_map = UnorderedMap::new(fnptr!(Identifier::hash, Arc<Identifier::Identifier>), fnptr!(Identifier::isEqual, Arc<Identifier::Identifier>, Arc<Identifier::Identifier>), 1);
            let __range0 = (1..=(var_field!((*jacobian).comps, BackendDAE::NBackendDAE::JACOBIAN).clone().borrow().len() as i32)).rev();
            for mut i in __range0 {
                (columnEqn, indices, _) = SimStrongComponent::Block::fromStrongComponent(var_field!((*jacobian).comps, BackendDAE::NBackendDAE::JACOBIAN).borrow()[(i.clone()-1) as usize].clone(), indices.clone(), Partition::Kind::JAC.clone(), dummy_sim_map.clone(), dummy_eqn_map.clone())?;
                columnEqns = cons(columnEqn.clone(), columnEqns.clone());
            }
            generic_loop_calls = {
        let mut __acc: Arc<metamodelica::List<Arc<SimGenericCall::NSimGenericCall>>> = metamodelica::nil();
        for mut tpl in (UnorderedMap::toList(indices.generic_call_map.clone())).into_iter().cloned() {
            let __x = SimGenericCall::fromIdentifier(tpl.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            indices.generic_call_map = sim_map.clone();
            if Flags::getConfigBool(Flags::SIM_CODE_SCALARIZE.clone())? {
                seed_vec = BVariable::VariablePointers::scalarize(var_field!((**varData).seedVars, VarData::VarData::VAR_DATA_JAC).clone())?;
                res_vec = BVariable::VariablePointers::scalarize(var_field!((**varData).resultVars, VarData::VarData::VAR_DATA_JAC).clone())?;
                tmp_vec = BVariable::VariablePointers::scalarize(var_field!((**varData).tmpVars, VarData::VarData::VAR_DATA_JAC).clone())?;
            } else {
                seed_vec = var_field!((**varData).seedVars, VarData::VarData::VAR_DATA_JAC).clone();
                res_vec = var_field!((**varData).resultVars, VarData::VarData::VAR_DATA_JAC).clone();
                tmp_vec = var_field!((**varData).tmpVars, VarData::VarData::VAR_DATA_JAC).clone();
            }
            BVariable::VariablePointers::map(seed_vec.clone(), Arc::new({ let __pe_b1 = seedVars_ptr.clone(); let __pe_b2 = Pointer::create(SimCode::EMPTY_SIM_CODE_INDICES()); let __pe_b3 = VarType::SIMULATION.clone(); move |__pe_a0| SimVar::traverseCreate(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }))?;
            BVariable::VariablePointers::map(res_vec.clone(), Arc::new({ let __pe_b1 = resVars_ptr.clone(); let __pe_b2 = Pointer::create(SimCode::EMPTY_SIM_CODE_INDICES()); let __pe_b3 = VarType::SIMULATION.clone(); move |__pe_a0| SimVar::traverseCreate(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }))?;
            BVariable::VariablePointers::map(tmp_vec.clone(), Arc::new({ let __pe_b1 = tmpVars_ptr.clone(); let __pe_b2 = Pointer::create(SimCode::EMPTY_SIM_CODE_INDICES()); let __pe_b3 = VarType::SIMULATION.clone(); move |__pe_a0| SimVar::traverseCreate(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }))?;
            seedVars = Pointer::access(seedVars_ptr.clone()).reverse();
            resVars = Pointer::access(resVars_ptr.clone()).reverse();
            tmpVars = Pointer::access(tmpVars_ptr.clone()).reverse();
            jac_map = UnorderedMap::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>), ComponentRef::isEqual, (seedVars.clone().len() as i32) + (resVars.clone().len() as i32) + (tmpVars.clone().len() as i32));
            SimCodeUtil::addListSimCodeMap(seedVars.clone(), jac_map.clone())?;
            SimCodeUtil::addListSimCodeMap(resVars.clone(), jac_map.clone())?;
            SimCodeUtil::addListSimCodeMap(tmpVars.clone(), jac_map.clone())?;
            match '__try1: {
                idx_map = UnorderedMap::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>), ComponentRef::isEqual, (seedVars.clone().len() as i32) + (resVars.clone().len() as i32));
                if Jacobian::isDynamic(var_field!((*jacobian).jacType, BackendDAE::NBackendDAE::JACOBIAN).clone()) {
                    if var_field!((*jacobian).isAdjoint, BackendDAE::NBackendDAE::JACOBIAN).clone() {
                        loopVars = resVars.clone();
                    } else {
                        loopVars = seedVars.clone();
                    }
                    for mut var in &*loopVars.clone() {
                        let mut var = var.clone();
                        cref = SimVar::getName(var.clone());
                        if BVariable::checkCref(cref.clone(), fnptr!(BVariable::isSeed, Pointer::Pointer<Arc<NFVariable::NFVariable>>), metamodelica::sourceInfo!()) {
                            cref = unwrap_break_err!(BVariable::getPartnerCref(cref.clone(), fnptr!(BVariable::getVarSeed, Pointer::Pointer<Arc<NFVariable::NFVariable>>), false), '__try1);
                        } else if BVariable::checkCref(cref.clone(), fnptr!(BVariable::isPDer, Pointer::Pointer<Arc<NFVariable::NFVariable>>), metamodelica::sourceInfo!()) {
                            cref = unwrap_break_err!(BVariable::getPartnerCref(cref.clone(), fnptr!(BVariable::getVarPDer, Pointer::Pointer<Arc<NFVariable::NFVariable>>), false), '__try1);
                        }
                        unwrap_break_err!(UnorderedMap::add(cref.clone(), var.index.clone(), idx_map.clone()), '__try1);
                        if BVariable::checkCref(cref.clone(), fnptr!(BVariable::isState, Pointer::Pointer<Arc<NFVariable::NFVariable>>), metamodelica::sourceInfo!()) {
                            cref = unwrap_break_err!(BVariable::getPartnerCref(cref.clone(), fnptr!(BVariable::getVarDer, Pointer::Pointer<Arc<NFVariable::NFVariable>>), false), '__try1);
                            unwrap_break_err!(UnorderedMap::add(cref.clone(), var.index.clone(), idx_map.clone()), '__try1);
                        }
                    }
                    if var_field!((*jacobian).jacType, BackendDAE::NBackendDAE::JACOBIAN).clone() == Jacobian::JacobianType::DAE.clone() {
                        for mut var in &*resVars.clone() {
                            let mut var = var.clone();
                            cref = SimVar::getName(var.clone());
                            unwrap_break_err!(UnorderedMap::add(cref.clone(), var.index.clone(), idx_map.clone()), '__try1);
                        }
                    }
                } else {
                    for mut var in &*seedVars.clone() {
                        let mut var = var.clone();
                        cref = SimVar::getName(var.clone());
                        unwrap_break_err!(UnorderedMap::add(cref.clone(), var.index.clone(), idx_map.clone()), '__try1);
                        cref = unwrap_break_err!(BVariable::getPartnerCref(cref.clone(), fnptr!(BVariable::getVarSeed, Pointer::Pointer<Arc<NFVariable::NFVariable>>), false), '__try1);
                        unwrap_break_err!(UnorderedMap::add(cref.clone(), var.index.clone(), idx_map.clone()), '__try1);
                    }
                    for mut var in &*resVars.clone() {
                        let mut var = var.clone();
                        cref = SimVar::getName(var.clone());
                        unwrap_break_err!(UnorderedMap::add(cref.clone(), var.index.clone(), idx_map.clone()), '__try1);
                        cref = unwrap_break_err!(BVariable::getPartnerCref(cref.clone(), fnptr!(BVariable::getVarPDer, Pointer::Pointer<Arc<NFVariable::NFVariable>>), false), '__try1);
                        unwrap_break_err!(UnorderedMap::add(cref.clone(), var.index.clone(), idx_map.clone()), '__try1);
                    }
                }
                (sparsity, sparsityT, coloring, rowColoring) = unwrap_break_err!(createSparsity(jacobian.clone(), idx_map.clone()), '__try1);
                jac = Arc::new(SimJacobian { isAdjoint: var_field!((*jacobian).isAdjoint, BackendDAE::NBackendDAE::JACOBIAN).clone(), jac_map: Some(jac_map.clone()), generic_loop_calls: generic_loop_calls.clone(), numColors: (coloring.clone().len() as i32), rowColoring: rowColoring.clone(), coloring: coloring.clone(), sparsityT: sparsityT.clone(), sparsity: sparsity.clone(), seedVars: seedVars.clone(), columnVars: tmpVars.clone(), constantEqns: metamodelica::nil(), columnEqns: columnEqns.clone(), numberOfResultVars: (resVars.clone().len() as i32), partitionIndex: 0, jacobianIndex: indices.jacobianIndex.clone(), name: (var_field!((*jacobian).name, BackendDAE::NBackendDAE::JACOBIAN).clone()).clone() });
                indices.jacobianIndex = indices.jacobianIndex.clone() + 1;
                simJacobian = Some(jac.clone());
                Ok::<_, anyhow::Error>((coloring.clone(), idx_map.clone(), indices.clone(), jac.clone(), rowColoring.clone(), simJacobian.clone(), sparsity.clone(), sparsityT.clone()))
            } {
                Ok((__try1_o0, __try1_o1, __try1_o2, __try1_o3, __try1_o4, __try1_o5, __try1_o6, __try1_o7)) => {
                    coloring = __try1_o0;
                    idx_map = __try1_o1;
                    indices = __try1_o2;
                    jac = __try1_o3;
                    rowColoring = __try1_o4;
                    simJacobian = __try1_o5;
                    sparsity = __try1_o6;
                    sparsityT = __try1_o7;
                }
                Err(_) => {
                    simJacobian = None;
                    Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimJacobian.SimJacobian.create")); __mm_s.push_str(&*literal!(" could not generate sparsity pattern.")); ArcStr::from(__mm_s) }).clone())?;
                    bail!("try/else: outputs not set in else branch");
                }
            }
            simJacobian.clone()
        },
        _ => {
            let mut dummy_sim_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>> = UnorderedMap::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>), ComponentRef::isEqual, 1);
            let mut dummy_eqn_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimStrongComponent::Block::Block>>> = UnorderedMap::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>), ComponentRef::isEqual, 1);
            let mut columnEqns: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>> = metamodelica::nil();
            let mut seedVars_ptr: Pointer::Pointer<Arc<metamodelica::List<Arc<SimVar::SimVar>>>> = Pointer::create(metamodelica::nil());
            let mut resVars_ptr: Pointer::Pointer<Arc<metamodelica::List<Arc<SimVar::SimVar>>>> = Pointer::create(metamodelica::nil());
            let mut tmpVars_ptr: Pointer::Pointer<Arc<metamodelica::List<Arc<SimVar::SimVar>>>> = Pointer::create(metamodelica::nil());
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimJacobian.SimJacobian.create")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok((simJacobian, indices))
    }

    pub fn createSimulationJacobian(mut partitions: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>, mut simCodeIndices: SimCode::SimCodeIndices, mut simcode_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>) -> Result<(Arc<SimJacobian>, Arc<SimJacobian>, SimCode::SimCodeIndices)> {
        let mut simJac: Arc<SimJacobian>;
        let mut simJacAdjoint: Arc<SimJacobian>;
        let mut simCodeIndices: SimCode::SimCodeIndices = simCodeIndices;
        let mut jacobians: Arc<metamodelica::List<Arc<BackendDAE::NBackendDAE>>> = metamodelica::nil();
        let mut jacobiansAdjoint: Arc<metamodelica::List<Arc<BackendDAE::NBackendDAE>>> = metamodelica::nil();
        let mut simJacobian: Arc<BackendDAE::NBackendDAE>;
        let mut simJacobianAdjoint: Arc<BackendDAE::NBackendDAE>;
        let mut simJac_opt: Option<Arc<SimJacobian>> = None;
        let mut simJacAdj_opt: Option<Arc<SimJacobian>> = None;
        let mut jacobian: Option<Arc<BackendDAE::NBackendDAE>> = None;
        let mut jacobianAdjoint: Option<Arc<BackendDAE::NBackendDAE>> = None;
        for mut partition in &*partitions.clone() {
            let mut partition = partition.clone();
            jacobian = Partition::Partition::getJacobian(partition.clone());
            if isSome(jacobian.clone()) {
                jacobians = cons(Util::getOption(jacobian.clone())?, jacobians.clone());
            }
            jacobianAdjoint = Partition::Partition::getJacobianAdjoint(partition.clone());
            if isSome(jacobianAdjoint.clone()) {
                jacobiansAdjoint = cons(Util::getOption(jacobianAdjoint.clone())?, jacobiansAdjoint.clone());
            }
        }
        if jacobians.clone().is_empty() {
            (simJac, simCodeIndices) = empty((literal!("A")).clone(), simCodeIndices.clone())?;
        } else {
            simJacobian = Jacobian::combine(jacobians.clone(), (literal!("A")).clone())?;
            (simJac_opt, simCodeIndices) = create(simJacobian.clone(), simCodeIndices.clone(), simcode_map.clone())?;
            if isSome(simJac_opt.clone()) {
                simJac = Util::getOption(simJac_opt.clone())?;
            } else {
                (simJac, simCodeIndices) = empty((literal!("A")).clone(), simCodeIndices.clone())?;
            }
        }
        if jacobiansAdjoint.clone().is_empty() {
            (simJacAdjoint, simCodeIndices) = empty((literal!("ADJ")).clone(), simCodeIndices.clone())?;
        } else {
            simJacobianAdjoint = Jacobian::combine(jacobiansAdjoint.clone(), (literal!("ADJ")).clone())?;
            (simJacAdj_opt, simCodeIndices) = create(simJacobianAdjoint.clone(), simCodeIndices.clone(), simcode_map.clone())?;
            if isSome(simJacAdj_opt.clone()) {
                simJacAdjoint = Util::getOption(simJacAdj_opt.clone())?;
            } else {
                (simJacAdjoint, simCodeIndices) = empty((literal!("ADJ")).clone(), simCodeIndices.clone())?;
            }
        }
        Ok((simJac, simJacAdjoint, simCodeIndices))
    }

    pub fn createSparsity(mut jacobian: Arc<BackendDAE::NBackendDAE>, mut idx_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>) -> Result<(Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>, Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)> {
        let mut sparsity: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
        let mut sparsityT: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
        let mut coloring: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
        let mut rowColoring: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
        (sparsity, sparsityT, coloring, rowColoring) = (::match_deref::match_deref! { match &(jacobian.clone()) {
        Deref @ BackendDAE::JACOBIAN { sparsityPattern: Bpattern, .. } => {
            sparsity = createSparsityPattern(Bpattern.col_wise_pattern.clone(), idx_map.clone())?;
            sparsityT = createSparsityPattern(Bpattern.row_wise_pattern.clone(), idx_map.clone())?;
            (coloring, rowColoring) = createSparsityColoring(var_field!((*jacobian).sparsityColoring, BackendDAE::NBackendDAE::JACOBIAN).clone(), idx_map.clone());
            (sparsity.clone(), sparsityT.clone(), coloring.clone(), rowColoring.clone())
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimJacobian.SimJacobian.createSparsity")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok((sparsity, sparsityT, coloring, rowColoring))
    }

    pub fn createSparsityPattern(mut cols: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)>>, mut idx_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>) -> Result<Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>> {
        let mut simPattern: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
        let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut dependencies: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut dep_indices: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut col in &*cols.clone() {
            let mut col = col.clone();
            (cref, dependencies) = col.clone();
            dep_indices = List::map(dependencies.clone(), Arc::new({ let __pe_b1 = idx_map.clone(); move |__pe_a0| Ok(UnorderedMap::getOrFail(__pe_a0, __pe_b1.clone())) }));
            simPattern = cons((UnorderedMap::getOrFail(cref.clone(), idx_map.clone()), List::sort(dep_indices.clone(), Arc::new(fnptr!(intGt, i32, i32)))?), simPattern.clone());
        }
        simPattern = List::sort(simPattern.clone(), Arc::new(fnptr!(Util::compareTupleIntGt, _, _)))?;
        Ok(simPattern)
    }

    pub fn createSparsityColoring(mut coloring: Arc<Jacobian::SparsityColoring::SparsityColoring>, mut idx_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>) -> (Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) {
        let mut simColoringCols: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
        let mut simColoringRows: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
        simColoringCols = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut group in (coloring.cols.clone()).borrow().iter() {
            let __x = List::map(group.clone(), Arc::new({ let __pe_b1 = idx_map.clone(); move |__pe_a0| Ok(UnorderedMap::getOrFail(__pe_a0, __pe_b1.clone())) }));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        simColoringRows = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut group in (coloring.rows.clone()).borrow().iter() {
            let __x = List::map(group.clone(), Arc::new({ let __pe_b1 = idx_map.clone(); move |__pe_a0| Ok(UnorderedMap::getOrFail(__pe_a0, __pe_b1.clone())) }));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        (simColoringCols, simColoringRows)
    }

    pub fn empty(mut name: ArcStr, mut indices: SimCode::SimCodeIndices) -> Result<(Arc<SimJacobian>, SimCode::SimCodeIndices)> {
        let mut emptyJac: Arc<SimJacobian> = EMPTY_SIM_JAC().clone();
        let mut indices: SimCode::SimCodeIndices = indices;
        emptyJac = (::match_deref::match_deref! { match &(emptyJac.clone()) {
        Deref @ SimJacobian { .. } => {
            assign_field!(
                emptyJac.name = name.clone(),
                emptyJac.jacobianIndex = indices.jacobianIndex.clone()
            );
            indices.jacobianIndex = indices.jacobianIndex.clone() + 1;
            emptyJac.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimJacobian.SimJacobian.empty")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok((emptyJac, indices))
    }

    pub fn getJacobianBlocks(mut jacobian: Arc<SimJacobian>) -> Result<Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>> {
        let mut blcks: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>> = metamodelica::nil();
        blcks = (::match_deref::match_deref! { match &(jacobian.clone()) {
        Deref @ SimJacobian { .. } => listAppend(jacobian.constantEqns.clone(), jacobian.columnEqns.clone()),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimJacobian.SimJacobian.getJacobianBlocks")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(blcks)
    }

    pub fn getJacobiansBlocks(mut jacobians: Arc<metamodelica::List<Arc<SimJacobian>>>) -> Result<Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>> {
        let mut blcks: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>> = metamodelica::nil();
        for mut jacobian in &*jacobians.clone() {
            let mut jacobian = jacobian.clone();
            blcks = listAppend(getJacobianBlocks(jacobian.clone())?, blcks.clone());
        }
        Ok(blcks)
    }

    pub fn getJacobianHT(mut jacobian: Arc<SimJacobian>) -> Result<Option<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>>> {
        let mut jac_map: Option<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>> = None;
        jac_map = (::match_deref::match_deref! { match &(jacobian.clone()) {
        Deref @ SimJacobian { .. } => jacobian.jac_map.clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimJacobian.SimJacobian.getJacobianHT")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(jac_map)
    }

    pub fn convert(mut simJac: Arc<SimJacobian>) -> Result<Arc<OldSimCode::JacobianMatrix>> {
        let mut oldJac: Arc<OldSimCode::JacobianMatrix>;
        let mut oldJacCol: Arc<OldSimCode::JacobianColumn>;
        oldJac = (::match_deref::match_deref! { match &(simJac.clone()) {
        Deref @ SimJacobian { .. } => {
            oldJacCol = Arc::new(OldSimCode::JacobianColumn { constantEqns: {
        let mut __acc: Arc<metamodelica::List<Arc<OldSimCode::SimEqSystem>>> = metamodelica::nil();
        for mut blck in (simJac.constantEqns.clone()).into_iter().cloned() {
            let __x = SimStrongComponent::Block::convert(blck.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, numberOfResultVars: simJac.numberOfResultVars.clone(), columnVars: {
        let mut __acc: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
        for mut var in (simJac.columnVars.clone()).into_iter().cloned() {
            let __x = SimVar::convert(var.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, columnEqns: {
        let mut __acc: Arc<metamodelica::List<Arc<OldSimCode::SimEqSystem>>> = metamodelica::nil();
        for mut blck in (simJac.columnEqns.clone()).into_iter().cloned() {
            let __x = SimStrongComponent::Block::convert(blck.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } });
            oldJac = Arc::new(OldSimCode::JacobianMatrix { isAdjoint: simJac.isAdjoint.clone(), crefsHT: Util::applyOption(simJac.jac_map.clone(), Arc::new(fnptr!(SimCodeUtil::convertSimCodeMap, Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>))), generic_loop_calls: {
        let mut __acc: Arc<metamodelica::List<OldSimCode::SimGenericCall>> = metamodelica::nil();
        for mut gc in (simJac.generic_loop_calls.clone()).into_iter().cloned() {
            let __x = SimGenericCall::convert(gc.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, partitionIndex: simJac.partitionIndex.clone(), jacobianIndex: simJac.jacobianIndex.clone(), maxColorCols: simJac.numColors.clone(), coloredRows: simJac.rowColoring.clone(), coloredCols: simJac.coloring.clone(), nonlinearT: metamodelica::nil(), nonlinear: metamodelica::nil(), sparsityT: simJac.sparsityT.clone(), sparsity: simJac.sparsity.clone(), matrixName: (simJac.name.clone()).clone(), seedVars: SimVar::convertList(simJac.seedVars.clone()), columns: list![oldJacCol.clone()] });
            oldJac.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimJacobian.SimJacobian.convert")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(oldJac)
    }

}

// TODO: non-Sync, non-const-emittable constant — needs new emission path.
// Type: Arc<SimJacobian::SimJacobian>
// Expr: Constructor { name: 'NSimJacobian.SimJacobian.SIM_JAC', args: [Lit(Str('')), Lit(Int(0)), Lit(Int(0)), Lit(Int(0)), Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Lit(Int(0)), Array { elems: [], ty: List(Unknown) }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Lit(Bool(false))], named_args: [], ty: RustStruct('NSimJacobian.SimJacobian'), field_names: ['name', 'jacobianIndex', 'partitionIndex', 'numberOfResultVars', 'columnEqns', 'constantEqns', 'columnVars', 'seedVars', 'sparsity', 'sparsityT', 'coloring', 'rowColoring', 'numColors', 'generic_loop_calls', 'jac_map', 'isAdjoint'] }
pub fn EMPTY_SIM_JAC() -> Arc<SimJacobian::SimJacobian> { todo!("non-Sync, non-const-emittable constant EMPTY_SIM_JAC — extend codegen") }

