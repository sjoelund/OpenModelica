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
use crate::SBInterval;
use crate::SBLinearMap;
use crate::SBMultiInterval;
use crate::System;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, PartialEq)]
pub struct SBPWAtomicLinearMap {
    pub dom: Arc<SBAtomicSet::SBAtomicSet>,
    pub lmap: Arc<SBLinearMap::SBLinearMap>,
}

impl Default for SBPWAtomicLinearMap {
    fn default() -> Self {
        Self {
            dom: Default::default(),
            lmap: Default::default(),
        }
    }
}

pub type PW_ATOMIC_LINEAR_MAP = SBPWAtomicLinearMap;

pub fn new(mut dom: Arc<SBAtomicSet::SBAtomicSet>, mut lmap: Arc<SBLinearMap::SBLinearMap>) -> Arc<SBPWAtomicLinearMap> {
    let mut map: Arc<SBPWAtomicLinearMap>;
    let mut compatible: bool = true;
    let mut ints: metamodelica::Array<Arc<SBInterval::SBInterval>>;
    let mut g: metamodelica::Array<metamodelica::Real>;
    let mut o: metamodelica::Array<metamodelica::Real>;
    let mut i: Arc<SBInterval::SBInterval>;
    let mut gain: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut offset: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut lo: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut step: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut hi: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    if SBAtomicSet::ndim(dom.clone()) != SBLinearMap::ndim(lmap.clone()) {
        map = newEmpty();
        return map;
    }
    ints = SBMultiInterval::intervals(SBAtomicSet::aset(dom.clone()));
    g = SBLinearMap::gain(lmap.clone());
    o = SBLinearMap::offset(lmap.clone());
    let __range0 = 1..=(ints.clone().borrow().len() as i32);
    for mut j in __range0 {
        i = ints.clone().borrow()[(j.clone()-1) as usize].clone();
        gain = g.clone().borrow()[(j.clone()-1) as usize].clone();
        offset = g.clone().borrow()[(j.clone()-1) as usize].clone();
        if gain.clone() < intReal(System::intMaxLit()) {
            lo = metamodelica::OrderedFloat((SBInterval::lowerBound(i.clone())) as f64) * gain.clone() + offset.clone();
            step = metamodelica::OrderedFloat((SBInterval::stepValue(i.clone())) as f64) * gain.clone();
            hi = metamodelica::OrderedFloat((SBInterval::upperBound(i.clone())) as f64) * gain.clone() + offset.clone();
            if lo.clone() != metamodelica::OrderedFloat((((lo.clone()).0 as i32)) as f64) && SBInterval::lowerBound(i.clone()) > 0 {
                compatible = false;
                break;
            }
            if step.clone() != metamodelica::OrderedFloat((((step.clone()).0 as i32)) as f64) && SBInterval::stepValue(i.clone()) > 0 {
                compatible = false;
                break;
            }
            if hi.clone() != metamodelica::OrderedFloat((((hi.clone()).0 as i32)) as f64) && SBInterval::upperBound(i.clone()) > 0 {
                compatible = false;
                break;
            }
        }
    }
    if compatible.clone() {
        map = Arc::new(SBPWAtomicLinearMap { dom: SBAtomicSet::copy(dom.clone()), lmap: SBLinearMap::copy(lmap.clone()) });
    } else {
        map = newEmpty();
    }
    map
}

pub fn newEmpty() -> Arc<SBPWAtomicLinearMap> {
    let mut map: Arc<SBPWAtomicLinearMap>;
    map = Arc::new(SBPWAtomicLinearMap { dom: SBAtomicSet::newEmpty(), lmap: SBLinearMap::newEmpty() });
    map
}

pub fn dom(mut map: Arc<SBPWAtomicLinearMap>) -> Arc<SBAtomicSet::SBAtomicSet> {
    let mut dom: Arc<SBAtomicSet::SBAtomicSet> = map.dom.clone();
    dom
}

pub fn lmap(mut map: Arc<SBPWAtomicLinearMap>) -> Arc<SBLinearMap::SBLinearMap> {
    let mut lmap: Arc<SBLinearMap::SBLinearMap> = map.lmap.clone();
    lmap
}

pub fn isEmpty(mut map: Arc<SBPWAtomicLinearMap>) -> bool {
    let mut empty: bool = false;
    empty = SBAtomicSet::isEmpty(map.dom.clone()) && SBLinearMap::isEmpty(map.lmap.clone());
    empty
}

pub fn image(mut map: Arc<SBPWAtomicLinearMap>, mut set: Arc<SBAtomicSet::SBAtomicSet>) -> Result<Arc<SBAtomicSet::SBAtomicSet>> {
    fn crop_inf(mut v: metamodelica::Real) -> i32 {
        let mut i: i32 = 0;
        i = if (v.clone() >= intReal(System::intMaxLit())) {System::intMaxLit()} else {((v.clone()).0 as i32)};
        i
    }

    let mut outSet: Arc<SBAtomicSet::SBAtomicSet>;
    let mut inters: metamodelica::Array<Arc<SBInterval::SBInterval>>;
    let mut res: metamodelica::Array<Arc<SBInterval::SBInterval>>;
    let mut gains: metamodelica::Array<metamodelica::Real>;
    let mut offsets: metamodelica::Array<metamodelica::Real>;
    let mut set_int: Arc<SBAtomicSet::SBAtomicSet>;
    let mut int: Arc<SBInterval::SBInterval>;
    let mut gain: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut offset: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut new_lo: i32 = 0;
    let mut new_step: i32 = 0;
    let mut new_hi: i32 = 0;
    let mut tmp_lo: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut tmp_step: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut tmp_hi: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    if SBAtomicSet::isEmpty(map.dom.clone()) {
        outSet = SBAtomicSet::newEmpty();
        return Ok(outSet);
    }
    set_int = SBAtomicSet::intersection(set.clone(), map.dom.clone())?;
    inters = SBMultiInterval::intervals(SBAtomicSet::aset(set_int.clone()));
    if inters.clone().borrow().is_empty() {
        outSet = SBAtomicSet::newEmpty();
        return Ok(outSet);
    }
    gains = SBLinearMap::gain(map.lmap.clone());
    offsets = SBLinearMap::offset(map.lmap.clone());
    res = metamodelica::arrayCreate((inters.clone().borrow().len() as i32), inters.borrow()[(1-1) as usize].clone());
    let __range0 = 1..=(inters.clone().borrow().len() as i32);
    for mut i in __range0 {
        int = inters.clone().borrow()[(i.clone()-1) as usize].clone();
        gain = gains.borrow()[(i.clone()-1) as usize].clone();
        offset = offsets.borrow()[(i.clone()-1) as usize].clone();
        tmp_lo = metamodelica::OrderedFloat((SBInterval::lowerBound(int.clone())) as f64) * gain.clone() + offset.clone();
        tmp_step = metamodelica::OrderedFloat((SBInterval::stepValue(int.clone())) as f64) * gain.clone();
        tmp_hi = metamodelica::OrderedFloat((SBInterval::upperBound(int.clone())) as f64) * gain.clone() + offset.clone();
        if gain.clone() < intReal(System::intMaxLit()) {
            new_lo = crop_inf(tmp_lo.clone());
            new_step = crop_inf(tmp_step.clone());
            new_hi = crop_inf(tmp_hi.clone());
        } else {
            new_lo = 1;
            new_step = 1;
            new_hi = System::intMaxLit();
        }
        unsafe { metamodelica::Dangerous::arrayInitSlot(res.clone(), i.clone(), SBInterval::new(new_lo.clone(), new_step.clone(), new_hi.clone())) };
    }
    outSet = SBAtomicSet::new(SBMultiInterval::fromArray(res.clone()));
    Ok(outSet)
}

pub fn preImage(mut map: Arc<SBPWAtomicLinearMap>, mut set: Arc<SBAtomicSet::SBAtomicSet>) -> Result<Arc<SBAtomicSet::SBAtomicSet>> {
    let mut outSet: Arc<SBAtomicSet::SBAtomicSet>;
    let mut full_im: Arc<SBAtomicSet::SBAtomicSet>;
    let mut actual_im: Arc<SBAtomicSet::SBAtomicSet>;
    let mut aux: Arc<SBAtomicSet::SBAtomicSet>;
    let mut inv: Arc<SBPWAtomicLinearMap>;
    full_im = image(map.clone(), map.dom.clone())?;
    actual_im = SBAtomicSet::intersection(full_im.clone(), set.clone())?;
    inv = new(actual_im.clone(), SBLinearMap::inverse(map.lmap.clone()));
    aux = image(inv.clone(), actual_im.clone())?;
    outSet = SBAtomicSet::intersection(map.dom.clone(), aux.clone())?;
    Ok(outSet)
}

pub fn isEqual(mut map1: Arc<SBPWAtomicLinearMap>, mut map2: Arc<SBPWAtomicLinearMap>) -> bool {
    let mut equal: bool = false;
    equal = SBAtomicSet::isEqual(map1.dom.clone(), map2.dom.clone()) && SBLinearMap::isEqual(map1.lmap.clone(), map2.lmap.clone());
    equal
}

pub fn toString(mut map: Arc<SBPWAtomicLinearMap>) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut strl: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut g: metamodelica::Array<metamodelica::Real>;
    let mut o: metamodelica::Array<metamodelica::Real>;
    let mut ints: metamodelica::Array<Arc<SBInterval::SBInterval>>;
    g = SBLinearMap::gain(map.lmap.clone());
    o = SBLinearMap::offset(map.lmap.clone());
    ints = SBMultiInterval::intervals(SBAtomicSet::aset(map.dom.clone()));
    let __range0 = (1..=(ints.clone().borrow().len() as i32)).rev();
    for mut i in __range0 {
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*SBInterval::toString(ints.borrow()[(i.clone()-1) as usize].clone())); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", g.borrow()[(i.clone()-1) as usize].clone()))); __mm_s.push_str(&*literal!(" * x + ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", o.borrow()[(i.clone()-1) as usize].clone()))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
        strl = cons(r#str.clone(), strl.clone());
    }
    r#str = stringDelimitList(strl.clone(), (literal!("x")).clone());
    r#str
}


