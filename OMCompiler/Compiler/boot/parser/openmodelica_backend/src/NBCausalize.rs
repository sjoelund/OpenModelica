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

use crate::NBASSC as ASSC;
use crate::NBAdjacency as Adjacency;
use crate::NBBackendUtil as BackendUtil;
use crate::NBDifferentiate as Differentiate;
use crate::NBEquation as BEquation;
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationAttributes;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::Iterator;
use crate::NBMatching as Matching;
use crate::NBModule as Module;
use crate::NBPartition as BPartition;
use crate::NBPartition::Partition;
use crate::NBSorting as Sorting;
use crate::NBStrongComponent as StrongComponent;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use openmodelica_ast::Absyn::Path;
use openmodelica_nf_frontend::NFArrayConnections::NameVertexTable;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFDimension as Dimension;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFInstNode::InstNode;
use openmodelica_nf_frontend::NFPrefixes as Prefixes;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFTypeCheck as TypeCheck;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

// OF imports
// NF imports
// Backend imports
// util imports
// ############################################################
//                      Main Functions
// ############################################################
pub fn applyModule(mut partitions: Arc<metamodelica::List<Arc<Partition::Partition>>>, mut kind: BPartition::Kind, mut varData: Arc<VarData::VarData>, mut eqData: Arc<EqData::EqData>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<Partition::Partition>, Arc<VarData::VarData>, Arc<EqData::EqData>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>) -> Result<(Arc<Partition::Partition>, Arc<VarData::VarData>, Arc<EqData::EqData>)> + 'static>) -> Result<(Arc<metamodelica::List<Arc<Partition::Partition>>>, Arc<VarData::VarData>, Arc<EqData::EqData>)> {
    let mut new_partitions: Arc<metamodelica::List<Arc<Partition::Partition>>> = metamodelica::nil();
    let mut varData: Arc<VarData::VarData> = varData;
    let mut eqData: Arc<EqData::EqData> = eqData;
    let mut new_partition: Arc<Partition::Partition> = Arc::new(<Partition::Partition as ::std::default::Default>::default());
    let mut violated: bool = false;
    for mut partition in &*partitions.clone() {
        let mut partition = partition.clone();
        (new_partition, varData, eqData) = func(partition.clone(), varData.clone(), eqData.clone(), funcMap.clone())?;
        new_partitions = if (BPartition::Partition::isEmpty(new_partition.clone())) {new_partitions.clone()} else {cons(new_partition.clone(), new_partitions.clone())};
    }
    new_partitions = new_partitions.clone().reverse();
    if !(BPartition::kindIsInitial(kind.clone())) {
        for mut partition in &*new_partitions.clone() {
            let mut partition = partition.clone();
            violated = checkSystemVariabilities(partition.clone())? || violated.clone();
        }
        if violated.clone() {
            bail!("fail");
        }
    }
    Ok((new_partitions, varData, eqData))
}

pub fn checkSystemVariabilities(mut partition: Arc<Partition::Partition>) -> Result<bool> {
    let mut violated: bool = false;
    let mut err: ArcStr = arcstr::literal!("");
    if isSome(partition.strongComponents.clone()) {
        let __range0 = Util::getOption(partition.strongComponents.clone())?.borrow().iter().cloned().collect::<Vec<_>>();
        for mut scc in __range0 {
            let () = (::match_deref::match_deref! { match &(scc.clone()) {
        Deref @ StrongComponent::SINGLE_COMPONENT { .. } => {
            let mut ty1: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut ty2: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut kind: TypeCheck::MatchKind = TypeCheck::MatchKind::EXACT;
            ty1 = Type::removeSizeOneArraysAndRecords(Variable::typeOf(Pointer::access(var_field!((*scc).var, StrongComponent::NBStrongComponent::SINGLE_COMPONENT).clone())))?;
            ty2 = Type::removeSizeOneArraysAndRecords(BEquation::Equation::getType(Pointer::access(var_field!((*scc).eqn, StrongComponent::NBStrongComponent::SINGLE_COMPONENT).clone()), false)?)?;
            (_, _, kind) = TypeCheck::matchTypes(ty1.clone(), ty2.clone(), Expression::fromCref(BVariable::getVarName(var_field!((*scc).var, StrongComponent::NBStrongComponent::SINGLE_COMPONENT).clone()), false)?, TypeCheck::DEFAULT_OPTIONS.clone())?;
            if kind.clone() != TypeCheck::MatchKind::EXACT.clone() {
                err = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBCausalize.checkSystemVariabilities")); __mm_s.push_str(&*literal!(" failed. The following strong component has conflicting types: ")); __mm_s.push_str(&*Type::toString(ty1.clone())?); __mm_s.push_str(&*literal!(" != ")); __mm_s.push_str(&*Type::toString(ty2.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StrongComponent::toString(scc.clone(), -1)?); ArcStr::from(__mm_s) }).clone();
                if Flags::isSet(Flags::BLT_DUMP.clone())? {
                    err = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*err.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*BPartition::Partition::toString(partition.clone(), 0)?); ArcStr::from(__mm_s) }).clone();
                }
                Error::addMessage(Error::COMPILER_ERROR.clone(), list![(err.clone()).clone()])?;
                violated = true;
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
    }
    Ok(violated)
}

pub fn simple(mut vars: Arc<VariablePointers::VariablePointers>, mut eqns: Arc<EquationPointers::EquationPointers>, mut kind: BPartition::Kind, mut st: Adjacency::MatrixStrictness, mut iter: Arc<Iterator::Iterator>) -> Result<(Arc<Matching::NBMatching>, Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>>)> {
    let mut matching: Arc<Matching::NBMatching> = Arc::new(<Matching::NBMatching as ::std::default::Default>::default());
    let mut comps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = metamodelica::nil();
    let mut full: Arc<Adjacency::Matrix::Matrix>;
    let mut adj: Arc<Adjacency::Matrix::Matrix>;
    full = Adjacency::Matrix::createFull(vars.clone(), eqns.clone(), kind.clone())?;
    adj = Adjacency::Matrix::fullToFinal(full.clone(), vars.map.clone(), eqns.map.clone(), eqns.clone(), st.clone(), iter.clone());
    matching = Matching::regular(Matching::EMPTY_MATCHING().clone(), adj.clone(), false, false, true)?;
    adj = Adjacency::Matrix::upgrade(adj.clone(), full.clone(), vars.map.clone(), eqns.map.clone(), eqns.clone(), Adjacency::MatrixStrictness::SORTING.clone(), Arc::new(crate::NBEquation::Iterator::EMPTY))?;
    comps = Sorting::tarjan(adj.clone(), matching.clone(), vars.clone(), eqns.clone())?;
    Ok((matching, comps))
}

pub fn getModule() -> Result<Arc<dyn ::std::ops::Fn(Arc<Partition::Partition>, Arc<VarData::VarData>, Arc<EqData::EqData>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>) -> Result<(Arc<Partition::Partition>, Arc<VarData::VarData>, Arc<EqData::EqData>)> + 'static>> {
    let mut func: Module::causalizeInterface;
    let mut flag: ArcStr = Flags::getConfigString(Flags::MATCHING_ALGORITHM.clone())?;
    func = (::match_deref::match_deref! { match &(flag.clone()) {
        Deref @ "PFPlusExt" => causalizePseudoArray.clone(),
        Deref @ "pseudo" => causalizePseudoArray.clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBCausalize.getModule")); __mm_s.push_str(&*literal!(" failed for unknown option: ")); __mm_s.push_str(&*flag.clone()); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(func)
}

// ############################################################
//                Protected Functions and Types
// ############################################################
