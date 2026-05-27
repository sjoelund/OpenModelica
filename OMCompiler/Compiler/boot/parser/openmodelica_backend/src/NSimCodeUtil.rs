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

use crate::HashTableCrefSimVar;
use crate::NSimVar::ExtObjInfo;
use crate::NSimVar::SimVar;
use crate::NSimVar::SimVars;
use crate::SimCodeVar;
use openmodelica_frontend_types::DAE;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_util::UnorderedMap;
use openmodelica_util_datatypes_basic::List;

// Frontend imports
// SimCode imports
// Old SimCode imports
// Util imports
pub fn createSimCodeMap(mut simVars: Arc<SimVars::SimVars>, mut extObjInfo: Arc<ExtObjInfo::ExtObjInfo>) -> Result<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>> {
    let mut simcode_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>> = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
    addListSimCodeMap(simVars.stateVars.clone(), simcode_map.clone())?;
    addListSimCodeMap(simVars.derivativeVars.clone(), simcode_map.clone())?;
    addListSimCodeMap(simVars.algVars.clone(), simcode_map.clone())?;
    addListSimCodeMap(simVars.discreteAlgVars.clone(), simcode_map.clone())?;
    addListSimCodeMap(simVars.intAlgVars.clone(), simcode_map.clone())?;
    addListSimCodeMap(simVars.boolAlgVars.clone(), simcode_map.clone())?;
    addListSimCodeMap(simVars.inputVars.clone(), simcode_map.clone())?;
    addListSimCodeMap(simVars.outputVars.clone(), simcode_map.clone())?;
    addListSimCodeMap(simVars.aliasVars.clone(), simcode_map.clone())?;
    addListSimCodeMap(simVars.intAliasVars.clone(), simcode_map.clone())?;
    addListSimCodeMap(simVars.boolAliasVars.clone(), simcode_map.clone())?;
    addListSimCodeMap(simVars.paramVars.clone(), simcode_map.clone())?;
    addListSimCodeMap(simVars.intParamVars.clone(), simcode_map.clone())?;
    addListSimCodeMap(simVars.boolParamVars.clone(), simcode_map.clone())?;
    addListSimCodeMap(simVars.stringAlgVars.clone(), simcode_map.clone())?;
    addListSimCodeMap(simVars.stringParamVars.clone(), simcode_map.clone())?;
    addListSimCodeMap(simVars.stringAliasVars.clone(), simcode_map.clone())?;
    addListSimCodeMap(simVars.extObjVars.clone(), simcode_map.clone())?;
    addListSimCodeMap(simVars.constVars.clone(), simcode_map.clone())?;
    addListSimCodeMap(simVars.intConstVars.clone(), simcode_map.clone())?;
    addListSimCodeMap(simVars.boolConstVars.clone(), simcode_map.clone())?;
    addListSimCodeMap(simVars.stringConstVars.clone(), simcode_map.clone())?;
    addListSimCodeMap(simVars.residualVars.clone(), simcode_map.clone())?;
    addListSimCodeMap(simVars.jacobianVars.clone(), simcode_map.clone())?;
    addListSimCodeMap(simVars.seedVars.clone(), simcode_map.clone())?;
    addListSimCodeMap(simVars.realOptimizeConstraintsVars.clone(), simcode_map.clone())?;
    addListSimCodeMap(simVars.realOptimizeFinalConstraintsVars.clone(), simcode_map.clone())?;
    addListSimCodeMap(simVars.sensitivityVars.clone(), simcode_map.clone())?;
    addListSimCodeMap(simVars.dataReconSetcVars.clone(), simcode_map.clone())?;
    addListSimCodeMap(simVars.dataReconinputVars.clone(), simcode_map.clone())?;
    addListSimCodeMap(simVars.dataReconSetBVars.clone(), simcode_map.clone())?;
    addListSimCodeMap(extObjInfo.objects.clone(), simcode_map.clone())?;
    Ok(simcode_map)
}

pub fn addListSimCodeMap(mut simVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>, mut simcode_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>) -> Result<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>> {
    let mut simcode_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>> = simcode_map;
    for mut var in &*simVars.clone() {
        let mut var = var.clone();
        UnorderedMap::add(SimVar::getName(var.clone()), var.clone(), simcode_map.clone())?;
    }
    Ok(simcode_map)
}

pub fn convertSimCodeMap(mut simcode_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>) -> (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, SimCodeVar::SimVar)>>), i32, (HashTableCrefSimVar::FuncHashCref, HashTableCrefSimVar::FuncCrefEqual, HashTableCrefSimVar::FuncCrefStr, HashTableCrefSimVar::FuncExpStr)) {
    let mut old_ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, SimCodeVar::SimVar)>>), i32, (HashTableCrefSimVar::FuncHashCref, HashTableCrefSimVar::FuncCrefEqual, HashTableCrefSimVar::FuncCrefStr, HashTableCrefSimVar::FuncExpStr));
    let mut vars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = UnorderedMap::valueList(simcode_map.clone());
    old_ht = HashTableCrefSimVar::emptyHashTableSized(UnorderedMap::size(simcode_map.clone()));
    old_ht = List::fold(SimVar::convertList(vars.clone()), (std::sync::Arc::new(HashTableCrefSimVar::addSimVarToHashTable) as std::sync::Arc<dyn ::std::ops::Fn(SimCodeVar::SimVar, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, SimCodeVar::SimVar)>>), i32, (HashTableCrefSimVar::FuncHashCref, HashTableCrefSimVar::FuncCrefEqual, HashTableCrefSimVar::FuncCrefStr, HashTableCrefSimVar::FuncExpStr))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, SimCodeVar::SimVar)>>), i32, (HashTableCrefSimVar::FuncHashCref, HashTableCrefSimVar::FuncCrefEqual, HashTableCrefSimVar::FuncCrefStr, HashTableCrefSimVar::FuncExpStr))> + 'static>), old_ht.clone());
    old_ht
}

