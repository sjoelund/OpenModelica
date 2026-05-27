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

use crate::ComponentReference;
use crate::HashSet;
use crate::InstStateMachineUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashSet;
use openmodelica_util::BaseHashTable;
use openmodelica_util_datatypes_basic::List;

pub type Key = Arc<DAE::ComponentRef>;

pub type Value = InstStateMachineUtil::SMNode;

pub type HashTableCrefFunctionsType = (FuncHashCref, FuncCrefEqual, FuncCrefStr, FuncExpStr);

pub type HashTable = (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, InstStateMachineUtil::SMNode)>>), i32, (FuncHashCref, FuncCrefEqual, FuncCrefStr, FuncExpStr));

pub type FuncHashCref = std::sync::Arc<dyn ::std::ops::Fn(Key) -> Result<i32> + 'static>;

pub type FuncCrefEqual = std::sync::Arc<dyn ::std::ops::Fn(Key, Key) -> Result<bool> + 'static>;

pub type FuncCrefStr = std::sync::Arc<dyn ::std::ops::Fn(Key) -> Result<ArcStr> + 'static>;

pub type FuncExpStr = std::sync::Arc<dyn ::std::ops::Fn(Value) -> Result<ArcStr> + 'static>;

pub fn emptyHashTable() -> HashTable {
    let mut hashTable: HashTable;
    hashTable = emptyHashTableSized(BaseHashTable::defaultBucketSize.clone());
    hashTable
}

pub fn emptyHashTableSized(mut size: i32) -> HashTable {
    let mut hashTable: HashTable;
    hashTable = BaseHashTable::emptyHashTableWork(size.clone(), ((std::sync::Arc::new(ComponentReference::hashComponentRef) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), (std::sync::Arc::new(ComponentReferenceBasics::printComponentRefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>), (std::sync::Arc::new(modeStr) as std::sync::Arc<dyn ::std::ops::Fn(InstStateMachineUtil::SMNode) -> Result<ArcStr> + 'static>)));
    hashTable
}

pub fn modeStr(mut mode: InstStateMachineUtil::SMNode) -> Result<ArcStr> {
    let mut s: ArcStr = arcstr::literal!("");
    let mut componentRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut isInitial: bool = false;
    let mut edges: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut paths: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let InstStateMachineUtil::SMNODE { edges: __pa0, isInitial: __pa1, componentRef: __pa2 } = (mode.clone()) else { bail!("pattern mismatch") };
    edges = __pa0.clone();
    isInitial = __pa1.clone();
    componentRef = __pa2.clone();
    crefs = BaseHashSet::hashSetList(edges.clone())?;
    paths = List::map(crefs.clone(), (std::sync::Arc::new(ComponentReferenceBasics::printComponentRefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>));
    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SMNODE(")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(componentRef.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*boolString(isInitial.clone())); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*literal!("EDGES(")); __mm_s.push_str(&*stringDelimitList(paths.clone(), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("))\n")); ArcStr::from(__mm_s) }).clone();
    Ok(s)
}

