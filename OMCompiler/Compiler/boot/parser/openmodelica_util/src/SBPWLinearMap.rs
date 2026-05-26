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
use crate::SBLinearMap;
use crate::SBPWAtomicLinearMap;
use crate::SBSet;
use crate::System;
use crate::UnorderedSet;
use crate::Vector;
use openmodelica_util_datatypes_basic::Array;

#[derive(Clone, Debug, PartialEq)]
pub struct SBPWLinearMap {
    pub dom: metamodelica::Array<Arc<SBSet::SBSet>>,
    pub lmap: metamodelica::Array<Arc<SBLinearMap::SBLinearMap>>,
    pub ndim: i32,
}

impl Default for SBPWLinearMap {
    fn default() -> Self {
        Self {
            dom: Default::default(),
            lmap: Default::default(),
            ndim: Default::default(),
        }
    }
}

pub type PW_LINEAR_MAP = SBPWLinearMap;

pub fn new(mut dom: metamodelica::Array<Arc<SBSet::SBSet>>, mut lmap: metamodelica::Array<Arc<SBLinearMap::SBLinearMap>>) -> Arc<SBPWLinearMap> {
    let mut map: Arc<SBPWLinearMap>;
    let mut dim: i32 = 0;
    let mut same_dims: bool = false;
    if (dom.clone().borrow().len() as i32) != (lmap.clone().borrow().len() as i32) {
        map = newEmpty();
        return map;
    }
    if !(dom.clone().borrow().is_empty()) {
        dim = SBSet::ndim(dom.borrow()[(1-1) as usize].clone());
        same_dims = Array::all(dom.clone(), Arc::new({ let __pe_b1 = dim.clone(); move |__pe_a0| Ok(SBSet::isDim(__pe_a0, __pe_b1.clone())) })) && Array::all(lmap.clone(), Arc::new({ let __pe_b1 = dim.clone(); move |__pe_a0| Ok(SBLinearMap::isDim(__pe_a0, __pe_b1.clone())) }));
    }
    if !(same_dims.clone()) {
        map = newEmpty();
    } else {
        map = Arc::new(SBPWLinearMap { dom: metamodelica::arrayFromVec(dom.clone().borrow().clone()), lmap: metamodelica::arrayFromVec(lmap.clone().borrow().clone()), ndim: dim.clone() });
    }
    map
}

pub fn newScalar(mut dom: Arc<SBSet::SBSet>, mut lmap: Arc<SBLinearMap::SBLinearMap>) -> Arc<SBPWLinearMap> {
    let mut map: Arc<SBPWLinearMap>;
    if SBSet::ndim(dom.clone()) == SBLinearMap::ndim(lmap.clone()) {
        map = Arc::new(SBPWLinearMap { dom: arrayCreate(1, dom.clone()), lmap: arrayCreate(1, lmap.clone()), ndim: 1 });
    } else {
        map = newEmpty();
    }
    map
}

pub fn newEmpty() -> Arc<SBPWLinearMap> {
    let mut map: Arc<SBPWLinearMap>;
    map = Arc::new(SBPWLinearMap { dom: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), lmap: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), ndim: 0 });
    map
}

pub fn newIdentity(mut set: Arc<SBSet::SBSet>) -> Arc<SBPWLinearMap> {
    let mut map: Arc<SBPWLinearMap>;
    let mut lmap: Arc<SBLinearMap::SBLinearMap> = SBLinearMap::newIdentity(SBSet::ndim(set.clone()));
    map = Arc::new(SBPWLinearMap { dom: arrayCreate(1, set.clone()), lmap: arrayCreate(1, lmap.clone()), ndim: 1 });
    map
}

pub fn copy(mut map: Arc<SBPWLinearMap>) -> Arc<SBPWLinearMap> {
    let mut map: Arc<SBPWLinearMap> = map;
    assign_field!(
        map.dom = Array::map(map.dom.clone(), Arc::new(fnptr!(SBSet::copy, Arc<SBSet::SBSet>))),
        map.lmap = Array::map(map.lmap.clone(), Arc::new(fnptr!(SBLinearMap::copy, Arc<SBLinearMap::SBLinearMap>)))
    );
    map
}

pub fn dom(mut map: Arc<SBPWLinearMap>) -> metamodelica::Array<Arc<SBSet::SBSet>> {
    let mut dom: metamodelica::Array<Arc<SBSet::SBSet>> = map.dom.clone();
    dom
}

pub fn lmap(mut map: Arc<SBPWLinearMap>) -> metamodelica::Array<Arc<SBLinearMap::SBLinearMap>> {
    let mut lmap: metamodelica::Array<Arc<SBLinearMap::SBLinearMap>> = map.lmap.clone();
    lmap
}

pub fn ndim(mut map: Arc<SBPWLinearMap>) -> i32 {
    let mut ndim: i32 = map.ndim.clone();
    ndim
}

pub fn isEmpty(mut map: Arc<SBPWLinearMap>) -> bool {
    let mut empty: bool = map.dom.clone().borrow().is_empty();
    empty
}

pub fn image(mut map: Arc<SBPWLinearMap>, mut set: Arc<SBSet::SBSet>) -> Result<Arc<SBSet::SBSet>> {
    fn add_set(mut aset: Arc<SBAtomicSet::SBAtomicSet>, mut map: Arc<SBLinearMap::SBLinearMap>, mut set: Arc<SBSet::SBSet>) -> Result<Arc<SBSet::SBSet>> {
        let mut set: Arc<SBSet::SBSet> = set;
        let mut aux_map: Arc<SBPWAtomicLinearMap::SBPWAtomicLinearMap>;
        aux_map = SBPWAtomicLinearMap::new(aset.clone(), map.clone());
        set = SBSet::addAtomicSet(SBPWAtomicLinearMap::image(aux_map.clone(), aset.clone())?, set.clone())?;
        Ok(set)
    }

    let mut outSet: Arc<SBSet::SBSet> = SBSet::newEmpty();
    let mut dom: metamodelica::Array<Arc<SBSet::SBSet>> = map.dom.clone();
    let mut lmap: metamodelica::Array<Arc<SBLinearMap::SBLinearMap>> = map.lmap.clone();
    let mut ss: Arc<SBSet::SBSet>;
    let mut partial_res: Arc<SBSet::SBSet>;
    let __range0 = 1..=(dom.clone().borrow().len() as i32);
    for mut i in __range0 {
        ss = dom.borrow()[(i.clone()-1) as usize].clone();
        ss = SBSet::intersection(ss.clone(), set.clone())?;
        partial_res = UnorderedSet::fold(SBSet::asets(ss.clone()), Arc::new({ let __pe_b1 = lmap.borrow()[(i.clone()-1) as usize].clone(); move |__pe_a0, __pe_a2| add_set(__pe_a0, __pe_b1.clone(), __pe_a2) }), SBSet::newEmpty());
        outSet = SBSet::union(outSet.clone(), partial_res.clone())?;
    }
    Ok(outSet)
}

pub fn preImage(mut map: Arc<SBPWLinearMap>, mut set: Arc<SBSet::SBSet>) -> Result<Arc<SBSet::SBSet>> {
    fn add_set(mut aset: Arc<SBAtomicSet::SBAtomicSet>, mut map: Arc<SBLinearMap::SBLinearMap>, mut sets: metamodelica::Array<Arc<SBAtomicSet::SBAtomicSet>>, mut set: Arc<SBSet::SBSet>) -> Result<Arc<SBSet::SBSet>> {
        let mut set: Arc<SBSet::SBSet> = set;
        let mut aux_map: Arc<SBPWAtomicLinearMap::SBPWAtomicLinearMap>;
        aux_map = SBPWAtomicLinearMap::new(aset.clone(), map.clone());
        let __range0 = sets.clone().borrow().iter().cloned().collect::<Vec<_>>();
        for mut as2 in __range0 {
            set = SBSet::addAtomicSet(SBPWAtomicLinearMap::preImage(aux_map.clone(), as2.clone())?, set.clone())?;
        }
        Ok(set)
    }

    let mut outSet: Arc<SBSet::SBSet> = SBSet::newEmpty();
    let mut dom: metamodelica::Array<Arc<SBSet::SBSet>> = map.dom.clone();
    let mut lmap: metamodelica::Array<Arc<SBLinearMap::SBLinearMap>> = map.lmap.clone();
    let mut ss: Arc<SBSet::SBSet>;
    let mut partial_res: Arc<SBSet::SBSet>;
    let mut sets: metamodelica::Array<Arc<SBAtomicSet::SBAtomicSet>>;
    sets = UnorderedSet::toArray(SBSet::asets(set.clone()));
    let __range0 = 1..=(dom.clone().borrow().len() as i32);
    for mut i in __range0 {
        ss = dom.borrow()[(i.clone()-1) as usize].clone();
        partial_res = SBSet::newEmpty();
        partial_res = UnorderedSet::fold(SBSet::asets(ss.clone()), Arc::new({ let __pe_b1 = lmap.borrow()[(i.clone()-1) as usize].clone(); let __pe_b2 = sets.clone(); move |__pe_a0, __pe_a3| add_set(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }), SBSet::newEmpty());
        outSet = SBSet::union(outSet.clone(), partial_res.clone())?;
    }
    Ok(outSet)
}

pub fn compPW(mut map1: Arc<SBPWLinearMap>, mut map2: Arc<SBPWLinearMap>) -> Result<Arc<SBPWLinearMap>> {
    let mut outMap: Arc<SBPWLinearMap>;
    let mut dom1: metamodelica::Array<Arc<SBSet::SBSet>> = map1.dom.clone();
    let mut dom2: metamodelica::Array<Arc<SBSet::SBSet>> = map2.dom.clone();
    let mut ress: Arc<Vector::Vector<Arc<SBSet::SBSet>>>;
    let mut lmap1: metamodelica::Array<Arc<SBLinearMap::SBLinearMap>> = map1.lmap.clone();
    let mut lmap2: metamodelica::Array<Arc<SBLinearMap::SBLinearMap>> = map2.lmap.clone();
    let mut reslm: Arc<Vector::Vector<Arc<SBLinearMap::SBLinearMap>>>;
    let mut aux_dom: Arc<SBSet::SBSet>;
    let mut new_dom: Arc<SBSet::SBSet>;
    let mut d1: Arc<SBSet::SBSet>;
    let mut d2: Arc<SBSet::SBSet>;
    let mut l1: Arc<SBLinearMap::SBLinearMap>;
    let mut l2: Arc<SBLinearMap::SBLinearMap>;
    let mut new_lm: Arc<SBLinearMap::SBLinearMap>;
    if isEmpty(map1.clone()) || isEmpty(map2.clone()) {
        outMap = newEmpty();
        return Ok(outMap);
    }
    ress = Vector::new(0);
    reslm = Vector::new(0);
    let __range0 = 1..=(dom1.clone().borrow().len() as i32);
    for mut i in __range0 {
        d1 = dom1.clone().borrow()[(i.clone()-1) as usize].clone();
        let __range1 = 1..=(dom2.clone().borrow().len() as i32);
        for mut j in __range1 {
            d2 = dom2.clone().borrow()[(j.clone()-1) as usize].clone();
            aux_dom = image(map2.clone(), d2.clone())?;
            aux_dom = SBSet::intersection(aux_dom.clone(), d1.clone())?;
            aux_dom = preImage(map2.clone(), aux_dom.clone())?;
            new_dom = SBSet::intersection(aux_dom.clone(), d2.clone())?;
            if !(SBSet::isEmpty(new_dom.clone())) {
                l1 = lmap1.borrow()[(i.clone()-1) as usize].clone();
                l2 = lmap2.borrow()[(j.clone()-1) as usize].clone();
                new_lm = SBLinearMap::compose(l1.clone(), l2.clone());
                Vector::push(ress.clone(), new_dom.clone());
                Vector::push(reslm.clone(), new_lm.clone());
            }
        }
    }
    outMap = new(Vector::toArray(ress.clone()), Vector::toArray(reslm.clone()));
    Ok(outMap)
}

pub fn minInvCompact(mut map: Arc<SBPWLinearMap>) -> Result<Arc<SBPWLinearMap>> {
    let mut outMap: Arc<SBPWLinearMap>;
    let mut aux_dom: Arc<SBSet::SBSet>;
    let mut dom_inv: Arc<SBSet::SBSet>;
    let mut aux_map: Arc<SBLinearMap::SBLinearMap>;
    let mut map_inv: Arc<SBLinearMap::SBLinearMap>;
    let mut min: metamodelica::Array<i32>;
    let mut resg: metamodelica::Array<metamodelica::Real>;
    let mut reso: metamodelica::Array<metamodelica::Real>;
    let mut g: metamodelica::Array<metamodelica::Real>;
    let mut o: metamodelica::Array<metamodelica::Real>;
    if (map.dom.clone().borrow().len() as i32) != 1 {
        outMap = newEmpty();
        return Ok(outMap);
    }
    aux_dom = map.dom.clone().borrow()[(1-1) as usize].clone();
    dom_inv = image(map.clone(), aux_dom.clone())?;
    aux_map = map.lmap.clone().borrow()[(1-1) as usize].clone();
    map_inv = SBLinearMap::inverse(aux_map.clone());
    min = SBSet::minElem(aux_dom.clone())?;
    g = SBLinearMap::gain(map_inv.clone());
    o = SBLinearMap::offset(map_inv.clone());
    resg = metamodelica::arrayCreate((g.clone().borrow().len() as i32), metamodelica::OrderedFloat(0.0_f64));
    reso = metamodelica::arrayCreate((o.clone().borrow().len() as i32), metamodelica::OrderedFloat(0.0_f64));
    let __range0 = 1..=(g.clone().borrow().len() as i32);
    for mut i in __range0 {
        if g.borrow()[(i.clone()-1) as usize].clone() == intReal(System::intMaxLit()) {
            {
                let __cell1 = metamodelica::OrderedFloat((0) as f64);
                unsafe { metamodelica::Dangerous::arrayInitSlot(resg.clone().clone(), i.clone(), __cell1); }
            }
            {
                let __cell2 = intReal(min.borrow()[(i.clone()-1) as usize].clone());
                unsafe { metamodelica::Dangerous::arrayInitSlot(reso.clone().clone(), i.clone(), __cell2); }
            }
        } else {
            {
                let __cell3 = g.borrow()[(i.clone()-1) as usize].clone();
                unsafe { metamodelica::Dangerous::arrayInitSlot(resg.clone().clone(), i.clone(), __cell3); }
            }
            {
                let __cell4 = o.borrow()[(i.clone()-1) as usize].clone();
                unsafe { metamodelica::Dangerous::arrayInitSlot(reso.clone().clone(), i.clone(), __cell4); }
            }
        }
    }
    outMap = new(arrayCreate(1, dom_inv.clone()), arrayCreate(1, SBLinearMap::new(resg.clone(), reso.clone())));
    Ok(outMap)
}

pub fn wholeDom(mut map: Arc<SBPWLinearMap>) -> Result<Arc<SBSet::SBSet>> {
    let mut set: Arc<SBSet::SBSet>;
    set = SBSet::newEmpty();
    let __range0 = map.dom.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut s in __range0 {
        set = SBSet::union(set.clone(), s.clone())?;
    }
    Ok(set)
}

pub fn combine(mut map1: Arc<SBPWLinearMap>, mut map2: Arc<SBPWLinearMap>) -> Result<Arc<SBPWLinearMap>> {
    let mut outMap: Arc<SBPWLinearMap>;
    let mut sres: Arc<Vector::Vector<Arc<SBSet::SBSet>>>;
    let mut lres: Arc<Vector::Vector<Arc<SBLinearMap::SBLinearMap>>>;
    let mut dom2: metamodelica::Array<Arc<SBSet::SBSet>>;
    let mut lm2: metamodelica::Array<Arc<SBLinearMap::SBLinearMap>>;
    let mut aux1: Arc<SBSet::SBSet>;
    let mut s2: Arc<SBSet::SBSet>;
    let mut new_dom: Arc<SBSet::SBSet>;
    if isEmpty(map1.clone()) {
        outMap = copy(map2.clone());
        return Ok(outMap);
    }
    if isEmpty(map2.clone()) {
        outMap = copy(map1.clone());
        return Ok(outMap);
    }
    sres = Vector::fromArray(map1.dom.clone());
    lres = Vector::fromArray(map1.lmap.clone());
    dom2 = map2.dom.clone();
    lm2 = map2.lmap.clone();
    aux1 = wholeDom(map1.clone())?;
    let __range0 = 1..=(dom2.clone().borrow().len() as i32);
    for mut i in __range0 {
        s2 = dom2.borrow()[(i.clone()-1) as usize].clone();
        new_dom = SBSet::complement(s2.clone(), aux1.clone())?;
        if !(SBSet::isEmpty(new_dom.clone())) {
            Vector::push(sres.clone(), new_dom.clone());
            Vector::push(lres.clone(), lm2.borrow()[(i.clone()-1) as usize].clone());
        }
    }
    outMap = new(Vector::toArray(sres.clone()), Vector::toArray(lres.clone()));
    Ok(outMap)
}

pub fn atomize(mut map: Arc<SBPWLinearMap>) -> Result<Arc<SBPWLinearMap>> {
    let mut outMap: Arc<SBPWLinearMap>;
    let mut dres: Arc<metamodelica::List<Arc<SBSet::SBSet>>> = metamodelica::nil();
    let mut dom: metamodelica::Array<Arc<SBSet::SBSet>> = map.dom.clone();
    let mut lres: Arc<metamodelica::List<Arc<SBLinearMap::SBLinearMap>>> = metamodelica::nil();
    let mut lm: metamodelica::Array<Arc<SBLinearMap::SBLinearMap>> = map.lmap.clone();
    let mut d: Arc<SBSet::SBSet>;
    let mut aux: Arc<SBSet::SBSet>;
    let mut l: Arc<SBLinearMap::SBLinearMap>;
    let mut asets: metamodelica::Array<Arc<SBAtomicSet::SBAtomicSet>>;
    let __range0 = 1..=(dom.clone().borrow().len() as i32);
    for mut i in __range0 {
        d = dom.borrow()[(i.clone()-1) as usize].clone();
        l = lm.borrow()[(i.clone()-1) as usize].clone();
        asets = UnorderedSet::toArray(SBSet::asets(d.clone()));
        let __range1 = asets.clone().borrow().iter().cloned().collect::<Vec<_>>();
        for mut s in __range1 {
            aux = SBSet::newEmpty();
            aux = SBSet::addAtomicSet(s.clone(), aux.clone())?;
            dres = cons(aux.clone(), dres.clone());
            lres = cons(l.clone(), lres.clone());
        }
    }
    outMap = new(metamodelica::arrayFromVec(dres.clone().reverse().into_iter().cloned().collect()), metamodelica::arrayFromVec(lres.clone().reverse().into_iter().cloned().collect()));
    Ok(outMap)
}

pub fn isEqual(mut map1: Arc<SBPWLinearMap>, mut map2: Arc<SBPWLinearMap>) -> bool {
    let mut equal: bool = false;
    equal = Array::isEqualOnTrue(map1.dom.clone(), map2.dom.clone(), Arc::new(fnptr!(SBSet::isEqual, Arc<SBSet::SBSet>, Arc<SBSet::SBSet>))) && Array::isEqualOnTrue(map1.lmap.clone(), map2.lmap.clone(), Arc::new(fnptr!(SBLinearMap::isEqual, Arc<SBLinearMap::SBLinearMap>, Arc<SBLinearMap::SBLinearMap>)));
    equal
}

pub fn toString(mut map: Arc<SBPWLinearMap>) -> ArcStr {
    fn helper(mut set: Arc<SBAtomicSet::SBAtomicSet>, mut lm: Arc<SBLinearMap::SBLinearMap>) -> ArcStr {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*SBPWAtomicLinearMap::toString(Arc::new(SBPWAtomicLinearMap::SBPWAtomicLinearMap { dom: set.clone(), lmap: lm.clone() }))); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
        r#str
    }

    let mut r#str: ArcStr = arcstr::literal!("");
    let mut dom: metamodelica::Array<Arc<SBSet::SBSet>> = map.dom.clone();
    let mut lmap: metamodelica::Array<Arc<SBLinearMap::SBLinearMap>> = map.lmap.clone();
    let mut strl: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let __range0 = (1..=(dom.clone().borrow().len() as i32)).rev();
    for mut i in __range0 {
        strl = cons(UnorderedSet::toString(SBSet::asets(dom.borrow()[(i.clone()-1) as usize].clone()), Arc::new({ let __pe_b1 = lmap.borrow()[(i.clone()-1) as usize].clone(); move |__pe_a0| Ok(helper(__pe_a0, __pe_b1.clone())) }), (literal!("U")).clone()), strl.clone());
    }
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*stringDelimitList(strl.clone(), (literal!(",")).clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
    r#str
}


