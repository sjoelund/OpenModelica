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
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Util;

/* Below is the instance specific code. For each hashtable the user must define:

Key       - The key used to uniquely define elements in a hashtable
Value     - The data to associate with each key
hashFunc   - A function that maps a key to a positive integer.
keyEqual   - A comparison function between two keys, returns true if equal.
*/
/* HashTable instance specific code */
pub type Key = (Arc<DAE::ComponentRef>, i32);

pub type Value = Arc<DAE::Exp>;

pub type HashTableCrefFunctionsType = (FuncHashCref, FuncCrefEqual, FuncCrefStr, FuncExpStr);

pub type HashTable = (metamodelica::Array<Arc<metamodelica::List<((Arc<DAE::ComponentRef>, i32), i32)>>>, (i32, i32, metamodelica::Array<Option<((Arc<DAE::ComponentRef>, i32), Arc<DAE::Exp>)>>), i32, (FuncHashCref, FuncCrefEqual, FuncCrefStr, FuncExpStr));

pub type FuncHashCref = std::sync::Arc<dyn ::std::ops::Fn(Key) -> Result<i32> + 'static>;

pub type FuncCrefEqual = std::sync::Arc<dyn ::std::ops::Fn(Key, Key) -> Result<bool> + 'static>;

pub type FuncCrefStr = std::sync::Arc<dyn ::std::ops::Fn(Key) -> Result<ArcStr> + 'static>;

pub type FuncExpStr = std::sync::Arc<dyn ::std::ops::Fn(Value) -> Result<ArcStr> + 'static>;

fn hashFunc(mut tpl: Key) -> Result<i32> {
    let mut res: i32 = 0;
    res = ComponentReference::hashComponentRef(Util::tuple21(tpl.clone()))? + Util::tuple22(tpl.clone());
    Ok(res)
}

fn keyEqual(mut tpl1: Key, mut tpl2: Key) -> Result<bool> {
    let mut res: bool = false;
    res = 'mc: {
        let __mc_input = (tpl1.clone(), tpl2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((cr1, i1), (cr2, i2)) => {
                    let true = (intEq(i1.clone(), i2.clone())) else { bail!("pattern mismatch") };
                    Ok(ComponentReferenceBasics::crefEqual(cr1.clone(), cr2.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(res)
}

fn printKey(mut tpl: Key) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    res = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(Util::tuple21(tpl.clone()))?); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*intString(Util::tuple22(tpl.clone()))); ArcStr::from(__mm_s) }).clone();
    Ok(res)
}

pub fn emptyHashTable() -> HashTable {
    let mut hashTable: HashTable;
    hashTable = emptyHashTableSized(BaseHashTable::defaultBucketSize.clone());
    hashTable
}

pub fn emptyHashTableSized(mut size: i32) -> HashTable {
    let mut hashTable: HashTable;
    hashTable = BaseHashTable::emptyHashTableWork(size.clone(), ((std::sync::Arc::new(hashFunc) as std::sync::Arc<dyn ::std::ops::Fn((Arc<DAE::ComponentRef>, i32)) -> Result<i32> + 'static>), (std::sync::Arc::new(keyEqual) as std::sync::Arc<dyn ::std::ops::Fn((Arc<DAE::ComponentRef>, i32), (Arc<DAE::ComponentRef>, i32)) -> Result<bool> + 'static>), (std::sync::Arc::new(printKey) as std::sync::Arc<dyn ::std::ops::Fn((Arc<DAE::ComponentRef>, i32)) -> Result<ArcStr> + 'static>), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>)));
    hashTable
}

