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

use crate::SBAtomicSet;
use crate::UnorderedSet;
use crate::Vector;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SBSet {
    pub asets: Arc<UnorderedSet::UnorderedSet<Arc<SBAtomicSet::SBAtomicSet>>>,
    pub ndim: i32,
}

impl Default for SBSet {
    fn default() -> Self {
        Self {
            asets: Default::default(),
            ndim: Default::default(),
        }
    }
}

pub type SET = SBSet;

pub fn new(mut ss: Arc<UnorderedSet::UnorderedSet<Arc<SBAtomicSet::SBAtomicSet>>>) -> Result<Arc<SBSet>> {
    fn is_equal_dim(mut set1: Arc<SBAtomicSet::SBAtomicSet>, mut dim: i32) -> bool {
        let mut equal: bool = SBAtomicSet::ndim(set1.clone()) == dim.clone();
        equal
    }

    let mut set: Arc<SBSet> = Arc::new(<SBSet as ::std::default::Default>::default());
    let mut dim: i32 = 0;
    if !(UnorderedSet::isEmpty(ss.clone())) {
        dim = SBAtomicSet::ndim(UnorderedSet::first(ss.clone())?);
        if dim.clone() != 0 && UnorderedSet::all(ss.clone(), Arc::new({ let __pe_b1 = dim.clone(); move |__pe_a0| Ok(is_equal_dim(__pe_a0, __pe_b1.clone())) })) {
            set = Arc::new(SBSet { asets: UnorderedSet::copy(ss.clone()), ndim: dim.clone() });
        } else {
            set = newEmpty();
        }
    } else {
        set = Arc::new(SBSet { asets: UnorderedSet::copy(ss.clone()), ndim: 0 });
    }
    Ok(set)
}

pub fn newEmpty() -> Arc<SBSet> {
    let mut set: Arc<SBSet> = Arc::new(<SBSet as ::std::default::Default>::default());
    set = Arc::new(SBSet { asets: UnorderedSet::new((std::sync::Arc::new(fnptr!(SBAtomicSet::hash, Arc<SBAtomicSet::SBAtomicSet>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SBAtomicSet::SBAtomicSet>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(SBAtomicSet::isEqual, Arc<SBAtomicSet::SBAtomicSet>, Arc<SBAtomicSet::SBAtomicSet>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SBAtomicSet::SBAtomicSet>, Arc<SBAtomicSet::SBAtomicSet>) -> Result<bool> + 'static>), 13), ndim: 0 });
    set
}

pub fn copy(mut set: Arc<SBSet>) -> Arc<SBSet> {
    let mut set: Arc<SBSet> = set;
    assign_field!(set.asets = UnorderedSet::copy(set.asets.clone()));
    set
}

pub fn ndim(mut set: Arc<SBSet>) -> i32 {
    let mut ndim: i32 = set.ndim.clone();
    ndim
}

pub fn isEmpty(mut set: Arc<SBSet>) -> bool {
    let mut empty: bool = UnorderedSet::isEmpty(set.asets.clone());
    empty
}

pub fn isDim(mut set: Arc<SBSet>, mut dim: i32) -> bool {
    let mut res: bool = set.ndim.clone() == dim.clone();
    res
}

pub fn asets(mut set: Arc<SBSet>) -> Arc<UnorderedSet::UnorderedSet<Arc<SBAtomicSet::SBAtomicSet>>> {
    let mut asets: Arc<UnorderedSet::UnorderedSet<Arc<SBAtomicSet::SBAtomicSet>>> = set.asets.clone();
    asets
}

pub fn contains(mut vals: metamodelica::Array<i32>, mut set: Arc<SBSet>) -> bool {
    let mut res: bool = false;
    res = UnorderedSet::all(set.asets.clone(), Arc::new({ let __pe_b0 = vals.clone(); move |__pe_a1| Ok(SBAtomicSet::contains(__pe_b0.clone(), __pe_a1)) }));
    res
}

pub fn addAtomicSet(mut aset: Arc<SBAtomicSet::SBAtomicSet>, mut set: Arc<SBSet>) -> Result<Arc<SBSet>> {
    let mut set: Arc<SBSet> = set;
    if SBAtomicSet::isEmpty(aset.clone()) {
        return Ok(set);
    }
    if UnorderedSet::isEmpty(set.asets.clone()) {
        UnorderedSet::add(aset.clone(), set.asets.clone())?;
        assign_field!(set.ndim = SBAtomicSet::ndim(aset.clone()));
    } else if SBAtomicSet::ndim(aset.clone()) == set.ndim.clone() {
        UnorderedSet::add(aset.clone(), set.asets.clone())?;
    }
    Ok(set)
}

pub fn addAtomicSets(mut asets: Arc<UnorderedSet::UnorderedSet<Arc<SBAtomicSet::SBAtomicSet>>>, mut set: Arc<SBSet>) -> Arc<SBSet> {
    let mut set: Arc<SBSet> = set;
    set = UnorderedSet::fold(asets.clone(), (std::sync::Arc::new(addAtomicSet) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SBAtomicSet::SBAtomicSet>, Arc<SBSet>) -> Result<Arc<SBSet>> + 'static>), set.clone());
    set
}

pub fn intersection(mut set1: Arc<SBSet>, mut set2: Arc<SBSet>) -> Result<Arc<SBSet>> {
    let mut outSet: Arc<SBSet> = Arc::new(<SBSet as ::std::default::Default>::default());
    let mut int_set: Arc<SBAtomicSet::SBAtomicSet> = Arc::new(<SBAtomicSet::SBAtomicSet as ::std::default::Default>::default());
    let mut res: Arc<UnorderedSet::UnorderedSet<Arc<SBAtomicSet::SBAtomicSet>>>;
    if UnorderedSet::isEmpty(set1.asets.clone()) || UnorderedSet::isEmpty(set2.asets.clone()) {
        outSet = newEmpty();
        return Ok(outSet);
    }
    res = UnorderedSet::new((std::sync::Arc::new(fnptr!(SBAtomicSet::hash, Arc<SBAtomicSet::SBAtomicSet>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SBAtomicSet::SBAtomicSet>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(SBAtomicSet::isEqual, Arc<SBAtomicSet::SBAtomicSet>, Arc<SBAtomicSet::SBAtomicSet>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SBAtomicSet::SBAtomicSet>, Arc<SBAtomicSet::SBAtomicSet>) -> Result<bool> + 'static>), 13);
    let __range0 = UnorderedSet::toArray(set1.asets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
    for mut as1 in __range0 {
        let __range1 = UnorderedSet::toArray(set2.asets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
        for mut as2 in __range1 {
            int_set = SBAtomicSet::intersection(as1.clone(), as2.clone())?;
            if !(SBAtomicSet::isEmpty(int_set.clone())) {
                UnorderedSet::add(int_set.clone(), res.clone())?;
            }
        }
    }
    outSet = new(res.clone())?;
    Ok(outSet)
}

pub fn complement(mut set1: Arc<SBSet>, mut set2: Arc<SBSet>) -> Result<Arc<SBSet>> {
    let mut outSet: Arc<SBSet> = Arc::new(<SBSet as ::std::default::Default>::default());
    let mut int_res: Arc<UnorderedSet::UnorderedSet<Arc<SBAtomicSet::SBAtomicSet>>>;
    let mut aux: Arc<UnorderedSet::UnorderedSet<Arc<SBAtomicSet::SBAtomicSet>>>;
    let mut comp_res: Arc<UnorderedSet::UnorderedSet<Arc<SBAtomicSet::SBAtomicSet>>>;
    let mut new_sets: Arc<SBSet> = Arc::new(<SBSet as ::std::default::Default>::default());
    outSet = newEmpty();
    let __pa0 = ::match_deref::match_deref! { match &(intersection(set1.clone(), set2.clone())?) {
        Deref @ SBSet { asets: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    int_res = __pa0.clone();
    if !(UnorderedSet::isEmpty(int_res.clone())) {
        let __range1 = UnorderedSet::toArray(set1.asets.clone()).borrow().iter().cloned().collect::<Vec<_>>();
        for mut as1 in __range1 {
            aux = UnorderedSet::new((std::sync::Arc::new(fnptr!(SBAtomicSet::hash, Arc<SBAtomicSet::SBAtomicSet>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SBAtomicSet::SBAtomicSet>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(SBAtomicSet::isEqual, Arc<SBAtomicSet::SBAtomicSet>, Arc<SBAtomicSet::SBAtomicSet>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SBAtomicSet::SBAtomicSet>, Arc<SBAtomicSet::SBAtomicSet>) -> Result<bool> + 'static>), 13);
            UnorderedSet::add(as1.clone(), aux.clone())?;
            let __range2 = UnorderedSet::toArray(int_res.clone()).borrow().iter().cloned().collect::<Vec<_>>();
            for mut as2 in __range2 {
                new_sets = newEmpty();
                let __range3 = UnorderedSet::toArray(aux.clone()).borrow().iter().cloned().collect::<Vec<_>>();
                for mut as3 in __range3 {
                    comp_res = SBAtomicSet::complement(as3.clone(), as2.clone())?;
                    new_sets = addAtomicSets(comp_res.clone(), new_sets.clone());
                }
                aux = new_sets.asets.clone();
            }
            outSet = addAtomicSets(aux.clone(), outSet.clone());
        }
    } else {
        outSet = addAtomicSets(set1.asets.clone(), outSet.clone());
    }
    Ok(outSet)
}

pub fn union(mut set1: Arc<SBSet>, mut set2: Arc<SBSet>) -> Result<Arc<SBSet>> {
    let mut outSet: Arc<SBSet> = Arc::new(<SBSet as ::std::default::Default>::default());
    let mut aux: Arc<SBSet> = Arc::new(<SBSet as ::std::default::Default>::default());
    outSet = Arc::new(SBSet { asets: UnorderedSet::copy(set1.asets.clone()), ndim: set1.ndim.clone() });
    aux = complement(set2.clone(), outSet.clone())?;
    if !(isEmpty(aux.clone())) {
        outSet = addAtomicSets(aux.asets.clone(), outSet.clone());
    }
    Ok(outSet)
}

pub fn card(mut set: Arc<SBSet>) -> i32 {
    let mut cardinality: i32 = UnorderedSet::fold(set.asets.clone(), (std::sync::Arc::new(fnptr!(SBAtomicSet::cardinality, Arc<SBAtomicSet::SBAtomicSet>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SBAtomicSet::SBAtomicSet>, i32) -> Result<i32> + 'static>), 0);
    cardinality
}

pub fn maxCardinality(mut sets: Arc<Vector::Vector<Arc<SBSet>>>) -> Result<(Arc<SBSet>, i32)> {
    pub fn maxCardinality_traverse(mut set: Arc<SBSet>, mut maxCard: i32) -> (bool, i32) {
        let mut res: bool = false;
        let mut maxCard: i32 = maxCard;
        let mut cardinality: i32 = card(set.clone());
        if cardinality.clone() > maxCard.clone() {
            res = true;
            maxCard = cardinality.clone();
        }
        (res, maxCard)
    }

    let mut maxSet: Arc<SBSet> = Arc::new(<SBSet as ::std::default::Default>::default());
    let mut index: i32 = 0;
    match '__try0: {
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(Vector::findFold(sets.clone(), (std::sync::Arc::new(fnptr!(maxCardinality_traverse, Arc<SBSet>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SBSet>, i32) -> Result<(bool, i32)> + 'static>), 0)) {
            (Some(__pa1), __pa2, _) => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        maxSet = __pa1.clone();
        index = __pa2.clone();
        Ok::<_, anyhow::Error>((index.clone(), maxSet.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            index = __try0_o0;
            maxSet = __try0_o1;
        }
        Err(_) => {
            bail!("fail");
        }
    }
    Ok((maxSet, index))
}

pub fn minElem(mut set: Arc<SBSet>) -> Result<metamodelica::Array<i32>> {
    fn lessFn(mut set1: metamodelica::Array<i32>, mut set2: metamodelica::Array<i32>) -> bool {
        let mut res: bool = false;
        res = Array::isLess(set1.clone(), set2.clone(), (std::sync::Arc::new(fnptr!(intLt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>));
        res
    }

    let mut res: metamodelica::Array<i32>;
    let mut min_elems: Arc<metamodelica::List<metamodelica::Array<i32>>> = metamodelica::nil();
    if isEmpty(set.clone()) {
        res = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
    } else {
        min_elems = {
        let mut __acc: Arc<metamodelica::List<metamodelica::Array<i32>>> = metamodelica::nil();
        for mut e in (UnorderedSet::toArray(set.asets.clone())).borrow().iter() {
            let __x = SBAtomicSet::minElem(e.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        res = List::minElement(min_elems.clone(), (std::sync::Arc::new(fnptr!(lessFn, metamodelica::Array<i32>, metamodelica::Array<i32>)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<i32>, metamodelica::Array<i32>) -> Result<bool> + 'static>))?;
    }
    Ok(res)
}

pub fn isEqual(mut set1: Arc<SBSet>, mut set2: Arc<SBSet>) -> bool {
    let mut equal: bool = UnorderedSet::isEqual(set1.asets.clone(), set2.asets.clone()).unwrap();
    equal
}

pub fn hash(mut set: Arc<SBSet>) -> i32 {
    let mut hash: i32 = UnorderedSet::size(set.asets.clone());
    hash
}

pub fn toString(mut set: Arc<SBSet>) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*UnorderedSet::toString(set.asets.clone(), (std::sync::Arc::new(fnptr!(SBAtomicSet::toString, Arc<SBAtomicSet::SBAtomicSet>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SBAtomicSet::SBAtomicSet>) -> Result<ArcStr> + 'static>), (literal!("U")).clone())); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
    r#str
}


