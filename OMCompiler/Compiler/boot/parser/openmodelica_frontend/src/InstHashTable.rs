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

use crate::ConnectionGraph;
use crate::FCore;
use crate::InstTypes;
use crate::OperatorOverloading;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE::Connect;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Flags;
use openmodelica_util::Global;

pub type Key = Arc<Absyn::Path>;

pub type Value = Arc<metamodelica::List<Option<CachedInstItem>>>;

pub type CachedInstItemInputs = (Arc<DAE::Mod>, DAE::Prefix, DAE::Connect::Sets, ClassInf::State, Arc<SCode::Element>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>, bool, Option<Arc<DAE::ComponentRef>>, InstTypes::CallingScope);

pub type CachedInstItemOutputs = (FCore::Graph, DAE::DAElist, DAE::Connect::Sets, ClassInf::State, Arc<metamodelica::List<Arc<DAE::Var>>>, Option<Arc<DAE::Type>>, Option<SCode::Attributes>, Option<(Arc<Absyn::Path>, i32, DAE::InlineType)>, ConnectionGraph::ConnectionGraph);

pub type CachedPartialInstItemInputs = (Arc<DAE::Mod>, DAE::Prefix, ClassInf::State, Arc<SCode::Element>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Dimension>>>>>);

pub type CachedPartialInstItemOutputs = (FCore::Graph, ClassInf::State, Arc<metamodelica::List<Arc<DAE::Var>>>);

pub type CachedInstItems = Arc<metamodelica::List<Option<CachedInstItem>>>;

pub fn init() -> Result<()> {
    let mut ht: HashTable;
    if '__try0: {
        ht = crate::Globals::instHashIndex.with(|__root| __root.borrow().clone());
        ht = unwrap_break_err!(BaseHashTable::clear(ht.clone()), '__try0);
        crate::Globals::instHashIndex.with(|__root| *__root.borrow_mut() = ht.clone());
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        crate::Globals::instHashIndex.with(|__root| *__root.borrow_mut() = emptyInstHashTable()?);
    }
    Ok(())
}

pub fn release() -> Result<()> {
    crate::Globals::instHashIndex.with(|__root| *__root.borrow_mut() = emptyInstHashTable()?);
    OperatorOverloading::initCache();
    Ok(())
}

pub fn get(mut k: Key) -> Result<Value> {
    let mut v: Value = metamodelica::nil();
    let mut ht: HashTable;
    ht = crate::Globals::instHashIndex.with(|__root| __root.borrow().clone());
    v = BaseHashTable::get(k.clone(), ht.clone())?;
    Ok(v)
}

#[derive(Clone, Debug, PartialEq)]
pub enum CachedInstItem {
    FUNC_instClassIn {
        inputs: CachedInstItemInputs,
        outputs: CachedInstItemOutputs,
    },
    FUNC_partialInstClassIn {
        inputs: CachedPartialInstItemInputs,
        outputs: CachedPartialInstItemOutputs,
    },
}
pub use self::CachedInstItem::{FUNC_instClassIn,FUNC_partialInstClassIn};

pub fn addToInstCache(mut fullEnvPathPlusClass: Arc<Absyn::Path>, mut fullInstOpt: Option<CachedInstItem>, mut partialInstOpt: Option<CachedInstItem>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (fullEnvPathPlusClass.clone(), fullInstOpt.clone(), partialInstOpt.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let false = (Flags::isSet(Flags::CACHE.clone())?) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Some(_), Some(_)) => {
                    let mut instHash: HashTable;
                    instHash = crate::Globals::instHashIndex.with(|__root| __root.borrow().clone());
                    instHash = BaseHashTable::add((fullEnvPathPlusClass.clone(), list![fullInstOpt.clone(), partialInstOpt.clone()]), instHash.clone())?;
                    crate::Globals::instHashIndex.with(|__root| *__root.borrow_mut() = instHash.clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, None, Some(_)) => {
                    let mut instHash: HashTable;
                    let mut opt: Option<CachedInstItem> = None;
                    instHash = crate::Globals::instHashIndex.with(|__root| __root.borrow().clone());
                    let __pa0 = ::match_deref::match_deref! { match &(BaseHashTable::get(fullEnvPathPlusClass.clone(), instHash.clone())?) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    opt = __pa0.clone();
                    instHash = BaseHashTable::add((fullEnvPathPlusClass.clone(), list![opt.clone(), partialInstOpt.clone()]), instHash.clone())?;
                    crate::Globals::instHashIndex.with(|__root| *__root.borrow_mut() = instHash.clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, None, Some(_)) => {
                    let mut instHash: HashTable;
                    instHash = crate::Globals::instHashIndex.with(|__root| __root.borrow().clone());
                    instHash = BaseHashTable::add((fullEnvPathPlusClass.clone(), list![None, partialInstOpt.clone()]), instHash.clone())?;
                    crate::Globals::instHashIndex.with(|__root| *__root.borrow_mut() = instHash.clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Some(_), None) => {
                    let mut instHash: HashTable;
                    let mut lst: Arc<metamodelica::List<Option<CachedInstItem>>> = metamodelica::nil();
                    instHash = crate::Globals::instHashIndex.with(|__root| __root.borrow().clone());
                    let __pa0 = ::match_deref::match_deref! { match &(BaseHashTable::get(fullEnvPathPlusClass.clone(), instHash.clone())?) {
                        Deref @ metamodelica::List::Cons { head: _, tail: __pa0 @ Deref @ metamodelica::List::Cons { head: Some(_), tail: Deref @ metamodelica::List::Nil } } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    lst = __pa0.clone();
                    instHash = BaseHashTable::add((fullEnvPathPlusClass.clone(), cons(fullInstOpt.clone(), lst.clone())), instHash.clone())?;
                    crate::Globals::instHashIndex.with(|__root| *__root.borrow_mut() = instHash.clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Some(_), None) => {
                    let mut instHash: HashTable;
                    instHash = crate::Globals::instHashIndex.with(|__root| __root.borrow().clone());
                    instHash = BaseHashTable::add((fullEnvPathPlusClass.clone(), list![fullInstOpt.clone(), None]), instHash.clone())?;
                    crate::Globals::instHashIndex.with(|__root| *__root.borrow_mut() = instHash.clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub type HashTableKeyFunctionsType = (FuncHashKey, FuncKeyEqual, FuncKeyStr, FuncValueStr);

pub type HashTable = (metamodelica::Array<Arc<metamodelica::List<(Arc<Absyn::Path>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<Absyn::Path>, Arc<metamodelica::List<Option<CachedInstItem>>>)>>), i32, (FuncHashKey, FuncKeyEqual, FuncKeyStr, FuncValueStr));

type FuncHashKey = fn(Key) -> Result<i32>;

type FuncKeyEqual = fn(Key, Key) -> Result<bool>;

type FuncKeyStr = fn(Key) -> Result<ArcStr>;

type FuncValueStr = fn(Value) -> Result<ArcStr>;

fn opaqVal(mut v: Value) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = (literal!("OPAQUE_VALUE")).clone();
    r#str
}

fn emptyInstHashTable() -> Result<HashTable> {
    let mut hashTable: HashTable;
    hashTable = emptyInstHashTableSized(Flags::getConfigInt(Flags::INST_CACHE_SIZE.clone())?);
    OperatorOverloading::initCache();
    Ok(hashTable)
}

fn emptyInstHashTableSized(mut size: i32) -> HashTable {
    let mut hashTable: HashTable;
    hashTable = BaseHashTable::emptyHashTableWork(size.clone(), (AbsynUtil::pathHash, fnptr!(AbsynUtil::pathEqual, Arc<Absyn::Path>, Arc<Absyn::Path>), fnptr!(AbsynUtil::pathStringDefault, Arc<Absyn::Path>), fnptr!(opaqVal, Arc<metamodelica::List<Option<CachedInstItem>>>)));
    hashTable
}

