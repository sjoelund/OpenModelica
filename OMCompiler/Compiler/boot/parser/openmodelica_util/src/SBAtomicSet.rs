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

use crate::SBInterval;
use crate::SBMultiInterval;
use crate::UnorderedSet;

#[derive(Clone, Debug, PartialEq)]
pub struct SBAtomicSet {
    pub aset: Arc<SBMultiInterval::SBMultiInterval>,
    pub ndim: i32,
}

impl Default for SBAtomicSet {
    fn default() -> Self {
        Self {
            aset: Default::default(),
            ndim: Default::default(),
        }
    }
}

pub type ATOMIC_SET = SBAtomicSet;

pub fn new(mut mi: Arc<SBMultiInterval::SBMultiInterval>) -> Arc<SBAtomicSet> {
    let mut set: Arc<SBAtomicSet>;
    set = Arc::new(SBAtomicSet { aset: SBMultiInterval::copy(mi.clone()), ndim: mi.ndim.clone() });
    set
}

pub fn newEmpty() -> Arc<SBAtomicSet> {
    let mut set: Arc<SBAtomicSet>;
    set = Arc::new(SBAtomicSet { aset: SBMultiInterval::newEmpty(), ndim: 0 });
    set
}

pub fn copy(mut set: Arc<SBAtomicSet>) -> Arc<SBAtomicSet> {
    let mut outSet: Arc<SBAtomicSet>;
    outSet = Arc::new(SBAtomicSet { aset: SBMultiInterval::copy(set.aset.clone()), ndim: set.ndim.clone() });
    outSet
}

pub fn ndim(mut set: Arc<SBAtomicSet>) -> i32 {
    let mut ndim: i32 = set.ndim.clone();
    ndim
}

pub fn isEmpty(mut set: Arc<SBAtomicSet>) -> bool {
    let mut empty: bool = SBMultiInterval::isEmpty(set.aset.clone());
    empty
}

pub fn contains(mut vals: metamodelica::Array<i32>, mut set: Arc<SBAtomicSet>) -> bool {
    let mut res: bool = SBMultiInterval::contains(vals.clone(), set.aset.clone());
    res
}

pub fn intersection(mut set1: Arc<SBAtomicSet>, mut set2: Arc<SBAtomicSet>) -> Result<Arc<SBAtomicSet>> {
    let mut res: Arc<SBAtomicSet>;
    res = new(SBMultiInterval::intersection(set1.aset.clone(), set2.aset.clone())?);
    Ok(res)
}

pub fn complement(mut set1: Arc<SBAtomicSet>, mut set2: Arc<SBAtomicSet>) -> Result<Arc<UnorderedSet::UnorderedSet<Arc<SBAtomicSet>>>> {
    let mut res: Arc<UnorderedSet::UnorderedSet<Arc<SBAtomicSet>>>;
    let mut diff: Arc<UnorderedSet::UnorderedSet<Arc<SBMultiInterval::SBMultiInterval>>>;
    diff = SBMultiInterval::complement(set1.aset.clone(), set2.aset.clone())?;
    res = UnorderedSet::new(fnptr!(hash, Arc<SBAtomicSet>), fnptr!(isEqual, Arc<SBAtomicSet>, Arc<SBAtomicSet>), UnorderedSet::bucketCount(diff.clone()));
    if !(UnorderedSet::isEmpty(diff.clone())) {
        let __range0 = UnorderedSet::toArray(diff.clone()).borrow().iter().cloned().collect::<Vec<_>>();
        for mut s in __range0 {
            UnorderedSet::add(new(s.clone()), res.clone())?;
        }
    }
    Ok(res)
}

pub fn crossProd(mut set1: Arc<SBAtomicSet>, mut set2: Arc<SBAtomicSet>) -> Result<Arc<SBAtomicSet>> {
    let mut res: Arc<SBAtomicSet>;
    res = new(SBMultiInterval::crossProd(set1.aset.clone(), set2.aset.clone())?);
    Ok(res)
}

pub fn cardinality(mut set: Arc<SBAtomicSet>, mut card: i32) -> i32 {
    let mut card: i32 = card;
    card = card.clone() + SBMultiInterval::cardinality(set.aset.clone());
    card
}

pub fn aset(mut set: Arc<SBAtomicSet>) -> Arc<SBMultiInterval::SBMultiInterval> {
    let mut res: Arc<SBMultiInterval::SBMultiInterval> = set.aset.clone();
    res
}

pub fn minElem(mut set: Arc<SBAtomicSet>) -> metamodelica::Array<i32> {
    let mut res: metamodelica::Array<i32> = SBMultiInterval::minElem(set.aset.clone());
    res
}

pub fn replace(mut i: Arc<SBInterval::SBInterval>, mut dim: i32, mut set: Arc<SBAtomicSet>) -> Arc<SBAtomicSet> {
    let mut res: Arc<SBAtomicSet>;
    res = new(SBMultiInterval::replace(i.clone(), dim.clone(), set.aset.clone()));
    res
}

pub fn isEqual(mut set1: Arc<SBAtomicSet>, mut set2: Arc<SBAtomicSet>) -> bool {
    let mut equal: bool = SBMultiInterval::isEqual(set1.aset.clone(), set2.aset.clone());
    equal
}

pub fn hash(mut set1: Arc<SBAtomicSet>) -> i32 {
    let mut hash: i32 = SBMultiInterval::hash(set1.aset.clone());
    hash
}

pub fn toString(mut set: Arc<SBAtomicSet>) -> ArcStr {
    let mut r#str: ArcStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*SBMultiInterval::toString(set.aset.clone())); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) };
    r#str
}


