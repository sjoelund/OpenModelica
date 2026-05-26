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
use crate::SBMultiInterval;
use crate::SBSet;
use crate::System;
use crate::UnorderedSet;
use crate::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, PartialEq)]
pub struct SBLinearMap {
    pub gain: metamodelica::Array<metamodelica::Real>,
    pub offset: metamodelica::Array<metamodelica::Real>,
}

impl Default for SBLinearMap {
    fn default() -> Self {
        Self {
            gain: Default::default(),
            offset: Default::default(),
        }
    }
}

pub type LINEAR_MAP = SBLinearMap;

pub fn new(mut gain: metamodelica::Array<metamodelica::Real>, mut offset: metamodelica::Array<metamodelica::Real>) -> Arc<SBLinearMap> {
    let mut map: Arc<SBLinearMap>;
    if Array::any(gain.clone(), Arc::new(fnptr!(Util::realNegative, metamodelica::Real))) {
        map = newEmpty();
    } else if (gain.clone().borrow().len() as i32) == (offset.clone().borrow().len() as i32) {
        map = Arc::new(SBLinearMap { gain: metamodelica::arrayFromVec(gain.clone().borrow().clone()), offset: metamodelica::arrayFromVec(offset.clone().borrow().clone()) });
    } else {
        map = newEmpty();
    }
    map
}

pub fn newEmpty() -> Arc<SBLinearMap> {
    let mut map: Arc<SBLinearMap> = Arc::new(SBLinearMap { gain: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), offset: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()) });
    map
}

pub fn newIdentity(mut dim: i32) -> Arc<SBLinearMap> {
    let mut map: Arc<SBLinearMap>;
    map = Arc::new(SBLinearMap { gain: arrayCreate(dim.clone(), metamodelica::OrderedFloat(1.0_f64)), offset: arrayCreate(dim.clone(), metamodelica::OrderedFloat(0.0_f64)) });
    map
}

pub fn copy(mut map: Arc<SBLinearMap>) -> Arc<SBLinearMap> {
    let mut outMap: Arc<SBLinearMap>;
    outMap = Arc::new(SBLinearMap { gain: metamodelica::arrayFromVec(map.gain.clone().borrow().clone()), offset: metamodelica::arrayFromVec(map.offset.clone().borrow().clone()) });
    outMap
}

pub fn ndim(mut map: Arc<SBLinearMap>) -> i32 {
    let mut ndim: i32 = (map.gain.clone().borrow().len() as i32);
    ndim
}

pub fn isDim(mut map: Arc<SBLinearMap>, mut dim: i32) -> bool {
    let mut res: bool = (map.gain.clone().borrow().len() as i32) == dim.clone();
    res
}

pub fn gain(mut map: Arc<SBLinearMap>) -> metamodelica::Array<metamodelica::Real> {
    let mut gain: metamodelica::Array<metamodelica::Real> = map.gain.clone();
    gain
}

pub fn offset(mut map: Arc<SBLinearMap>) -> metamodelica::Array<metamodelica::Real> {
    let mut offset: metamodelica::Array<metamodelica::Real> = map.offset.clone();
    offset
}

pub fn isEmpty(mut map: Arc<SBLinearMap>) -> bool {
    let mut empty: bool = map.gain.clone().borrow().is_empty();
    empty
}

pub fn isIdentity(mut map: Arc<SBLinearMap>) -> bool {
    let mut isIdentity: bool = false;
    isIdentity = Array::all(map.gain.clone(), Arc::new({ let __pe_b0 = metamodelica::OrderedFloat(1.0_f64); move |__pe_a1| Ok(realEq(__pe_b0.clone(), __pe_a1)) })) && Array::all(map.offset.clone(), Arc::new({ let __pe_b0 = metamodelica::OrderedFloat(0.0_f64); move |__pe_a1| Ok(realEq(__pe_b0.clone(), __pe_a1)) }));
    isIdentity
}

pub fn isEqual(mut map1: Arc<SBLinearMap>, mut map2: Arc<SBLinearMap>) -> bool {
    let mut equal: bool = false;
    equal = Array::isEqualOnTrue(map1.gain.clone(), map2.gain.clone(), Arc::new(fnptr!(realEq, metamodelica::Real, metamodelica::Real))) && Array::isEqualOnTrue(map1.offset.clone(), map2.offset.clone(), Arc::new(fnptr!(realEq, metamodelica::Real, metamodelica::Real)));
    equal
}

pub fn compose(mut map1: Arc<SBLinearMap>, mut map2: Arc<SBLinearMap>) -> Arc<SBLinearMap> {
    let mut map: Arc<SBLinearMap>;
    let mut gain: metamodelica::Array<metamodelica::Real>;
    let mut offset: metamodelica::Array<metamodelica::Real>;
    let mut len1: i32 = ndim(map1.clone());
    let mut len2: i32 = ndim(map2.clone());
    let mut g1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut g2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut o1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut o2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    if len1.clone() == len2.clone() {
        gain = metamodelica::arrayCreate(len1.clone(), metamodelica::OrderedFloat(0.0_f64));
        offset = metamodelica::arrayCreate(len1.clone(), metamodelica::OrderedFloat(0.0_f64));
        for mut i in 1..=len1.clone() {
            g1 = map1.gain.clone().borrow()[(i.clone()-1) as usize].clone();
            g2 = map2.gain.clone().borrow()[(i.clone()-1) as usize].clone();
            o1 = map1.offset.clone().borrow()[(i.clone()-1) as usize].clone();
            o2 = map2.offset.clone().borrow()[(i.clone()-1) as usize].clone();
            unsafe { metamodelica::Dangerous::arrayInitSlot(gain.clone(), i.clone(), g1.clone() * g2.clone()) };
            unsafe { metamodelica::Dangerous::arrayInitSlot(offset.clone(), i.clone(), o2.clone() * g1.clone() + o1.clone()) };
        }
        map = Arc::new(SBLinearMap { gain: gain.clone(), offset: offset.clone() });
    } else {
        map = newEmpty();
    }
    map
}

pub fn inverse(mut map: Arc<SBLinearMap>) -> Arc<SBLinearMap> {
    let mut inv: Arc<SBLinearMap>;
    let mut gain: metamodelica::Array<metamodelica::Real>;
    let mut offset: metamodelica::Array<metamodelica::Real>;
    let mut len: i32 = ndim(map.clone());
    let mut g: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut o: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    gain = metamodelica::arrayCreate(len.clone(), metamodelica::OrderedFloat(0.0_f64));
    offset = metamodelica::arrayCreate(len.clone(), metamodelica::OrderedFloat(0.0_f64));
    for mut i in 1..=len.clone() {
        g = map.gain.clone().borrow()[(i.clone()-1) as usize].clone();
        o = map.offset.clone().borrow()[(i.clone()-1) as usize].clone();
        if g.clone() != metamodelica::OrderedFloat((0) as f64) {
            unsafe { metamodelica::Dangerous::arrayInitSlot(gain.clone(), i.clone(), metamodelica::OrderedFloat(1.0_f64) / g.clone()) };
            unsafe { metamodelica::Dangerous::arrayInitSlot(offset.clone(), i.clone(), -(o.clone() / g.clone())) };
        } else {
            unsafe { metamodelica::Dangerous::arrayInitSlot(gain.clone(), i.clone(), intReal(System::intMaxLit())) };
            unsafe { metamodelica::Dangerous::arrayInitSlot(offset.clone(), i.clone(), intReal(System::intMaxLit())) };
        }
    }
    inv = Arc::new(SBLinearMap { gain: gain.clone(), offset: offset.clone() });
    inv
}

pub fn apply(mut domain: Arc<SBSet::SBSet>, mut map: Arc<SBLinearMap>) -> Result<Arc<SBSet::SBSet>> {
    let mut target: Arc<SBSet::SBSet> = SBSet::copy(domain.clone());
    if !(isIdentity(map.clone())) {
        UnorderedSet::apply(target.asets.clone(), Arc::new({ let __pe_b1 = map.clone(); move |__pe_a0| Ok(applyAtomicSet(__pe_a0, __pe_b1.clone())) }))?;
    }
    Ok(target)
}

pub fn applyAtomicSet(mut atomic: Arc<SBAtomicSet::SBAtomicSet>, mut map: Arc<SBLinearMap>) -> Arc<SBAtomicSet::SBAtomicSet> {
    let mut atomic: Arc<SBAtomicSet::SBAtomicSet> = atomic;
    assign_field!(atomic.aset = applyMultiInterval(atomic.aset.clone(), map.clone()));
    atomic
}

pub fn applyMultiInterval(mut multiInt: Arc<SBMultiInterval::SBMultiInterval>, mut map: Arc<SBLinearMap>) -> Arc<SBMultiInterval::SBMultiInterval> {
    let mut multiInt: Arc<SBMultiInterval::SBMultiInterval> = multiInt;
    for mut i in 1..=multiInt.ndim.clone() {
        {
            let __cell0 = applyInterval(multiInt.intervals.borrow()[(i.clone()-1) as usize].clone(), map.gain.borrow()[(i.clone()-1) as usize].clone(), map.offset.borrow()[(i.clone()-1) as usize].clone());
            multiInt.intervals.clone().borrow_mut()[(i.clone()-1) as usize] = __cell0;
        }
    }
    multiInt
}

pub fn applyInterval(mut interval: Arc<SBInterval::SBInterval>, mut gain: metamodelica::Real, mut offset: metamodelica::Real) -> Arc<SBInterval::SBInterval> {
    let mut interval: Arc<SBInterval::SBInterval> = interval;
    assign_field!(
        interval.lo = ((intReal(interval.lo.clone()) * gain.clone() + offset.clone()).0 as i32),
        interval.step = ((intReal(interval.step.clone()) * gain.clone()).0 as i32),
        interval.hi = ((intReal(interval.hi.clone()) * gain.clone() + offset.clone()).0 as i32)
    );
    interval
}

pub fn toString(mut map: Arc<SBLinearMap>) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut strl: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let __range0 = (1..=(map.gain.clone().borrow().len() as i32)).rev();
    for mut i in __range0 {
        strl = cons({ let mut __mm_s = String::new(); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", map.gain.clone().borrow()[(i.clone()-1) as usize].clone()))); __mm_s.push_str(&*literal!(" * x + ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", map.offset.clone().borrow()[(i.clone()-1) as usize].clone()))); ArcStr::from(__mm_s) }, strl.clone());
    }
    r#str = stringDelimitList(strl.clone(), (literal!("\n")).clone());
    r#str
}


