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

use crate::FUnit;
use openmodelica_util::BaseHashTable;

/* Below is the instance specific code. For each hashtable the user must define:

Key       - The key used to uniquely define elements in a hashtable
Value     - The data to associate with each key
hashFunc   - A function that maps a key to a positive integer.
keyEqual   - A comparison function between two keys, returns true if equal.
*/
/* HashTable instance specific code */
pub type Key = FUnit::Unit;

pub type Value = ArcStr;

pub type HashTableCrefFunctionsType = (FuncHashKey, FuncKeyEqual, FuncKeyStr, FuncValueStr);

pub type HashTable = (metamodelica::Array<Arc<metamodelica::List<(FUnit::Unit, i32)>>>, (i32, i32, metamodelica::Array<Option<(FUnit::Unit, ArcStr)>>), i32, (FuncHashKey, FuncKeyEqual, FuncKeyStr, FuncValueStr));

pub type FuncHashKey = fn(Key) -> Result<i32>;

pub type FuncKeyEqual = fn(Key, Key) -> Result<bool>;

pub type FuncKeyStr = fn(Key) -> Result<ArcStr>;

pub type FuncValueStr = fn(Value) -> Result<ArcStr>;

pub fn emptyHashTable() -> HashTable {
    let mut hashTable: HashTable;
    hashTable = emptyHashTableSized(BaseHashTable::defaultBucketSize.clone());
    hashTable
}

fn id(mut inStr: ArcStr) -> ArcStr {
    let mut outStr: ArcStr = arcstr::literal!("");
    outStr = (inStr.clone()).clone();
    outStr
}

pub fn emptyHashTableSized(mut size: i32) -> HashTable {
    let mut hashTable: HashTable;
    hashTable = BaseHashTable::emptyHashTableWork(size.clone(), (FUnit::hashUnit, FUnit::unitEqual, FUnit::unit2string, fnptr!(id, ArcStr)));
    hashTable
}

