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

use crate::System;
use crate::UnorderedSet;

/// Interval type for set based graphs.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SBInterval {
    pub lo: i32,
    pub step: i32,
    pub hi: i32,
}

impl Default for SBInterval {
    fn default() -> Self {
        Self {
            lo: Default::default(),
            step: Default::default(),
            hi: Default::default(),
        }
    }
}

pub type INTERVAL = SBInterval;

fn euclid(mut a: i32, mut b: i32) -> (i32, i32, i32, i32) {
    let mut d: i32 = 0;
    let mut m: i32 = 0;
    let mut ua: i32 = 0;
    let mut vb: i32 = 0;
    let mut q: i32 = 0;
    let mut r1: i32 = a.clone();
    let mut r2: i32 = b.clone();
    let mut s1: i32 = a.clone();
    let mut s2: i32 = 0;
    let mut tmp: i32 = 0;
    while r2.clone() != 0 {
        q = r1.clone() / r2.clone();
        tmp = r2.clone();
        r2 = r1.clone() - q.clone() * r2.clone();
        r1 = tmp.clone();
        tmp = s2.clone();
        s2 = s1.clone() - q.clone() * s2.clone();
        s1 = tmp.clone();
    }
    d = r1.clone();
    m = s2.clone().abs();
    ua = s1.clone();
    vb = r1.clone() - s1.clone();
    (d, m, ua, vb)
}

pub fn new(mut lo: i32, mut step: i32, mut hi: i32) -> Arc<SBInterval> {
    let mut int: Arc<SBInterval> = Arc::new(<SBInterval as ::std::default::Default>::default());
    if lo.clone() >= 0 && step.clone() > 0 && hi.clone() >= 0 {
        if lo.clone() <= hi.clone() && hi.clone() < System::intMaxLit() {
            int = Arc::new(SBInterval { lo: lo.clone(), step: step.clone(), hi: hi.clone() - intMod(hi.clone() - lo.clone(), step.clone()) });
        } else if lo.clone() <= hi.clone() && hi.clone() == System::intMaxLit() {
            int = Arc::new(SBInterval { lo: lo.clone(), step: step.clone(), hi: System::intMaxLit() });
        } else {
            int = Arc::new(SBInterval { lo: lo.clone(), step: 0, hi: hi.clone() });
        }
    } else if lo.clone() >= 0 && step.clone() == 0 && hi.clone() == lo.clone() {
        int = Arc::new(SBInterval { lo: lo.clone(), step: 1, hi: hi.clone() });
    } else {
        int = newEmpty();
    }
    int
}

pub fn newEmpty() -> Arc<SBInterval> {
    let mut int: Arc<SBInterval> = Arc::new(SBInterval { lo: -1, step: 0, hi: -1 });
    int
}

pub fn newUnit() -> Arc<SBInterval> {
    let mut int: Arc<SBInterval> = Arc::new(SBInterval { lo: 1, step: 1, hi: 1 });
    int
}

pub fn newFull() -> Arc<SBInterval> {
    let mut int: Arc<SBInterval> = Arc::new(SBInterval { lo: 1, step: 1, hi: System::intMaxLit() });
    int
}

pub fn lowerBound(mut int: Arc<SBInterval>) -> i32 {
    let mut lo: i32 = int.lo.clone();
    lo
}

pub fn stepValue(mut int: Arc<SBInterval>) -> i32 {
    let mut step: i32 = int.step.clone();
    step
}

pub fn upperBound(mut int: Arc<SBInterval>) -> i32 {
    let mut hi: i32 = int.hi.clone();
    hi
}

pub fn crop(mut int: Arc<SBInterval>) -> Arc<SBInterval> {
    let mut int: Arc<SBInterval> = int;
    if int.hi.clone() < System::intMaxLit() {
        assign_field!(int.hi = int.hi.clone() - intMod(int.hi.clone() - int.lo.clone(), int.step.clone()));
    }
    int
}

pub fn intersection(mut int1: Arc<SBInterval>, mut int2: Arc<SBInterval>) -> Arc<SBInterval> {
    let mut int: Arc<SBInterval> = Arc::new(<SBInterval as ::std::default::Default>::default());
    let mut new_lo: i32 = 0;
    let mut new_step: i32 = 0;
    let mut new_hi: i32 = 0;
    let mut gcd_: i32 = 0;
    let mut ua: i32 = 0;
    let mut vb: i32 = 0;
    let mut x: i32 = 0;
    if int1.hi.clone() < int2.lo.clone() || int2.hi.clone() < int1.lo.clone() {
        int = newEmpty();
    } else {
        (gcd_, new_step, ua, vb) = euclid(int1.step.clone(), int2.step.clone());
        if 0 != intMod(int1.lo.clone() - int2.lo.clone(), gcd_.clone()) {
            int = newEmpty();
        } else {
            x = int1.lo.clone() / gcd_.clone() * vb.clone() + int2.lo.clone() / gcd_.clone() * ua.clone() + intMod(int1.lo.clone(), gcd_.clone());
            new_lo = intMax(int1.lo.clone(), int2.lo.clone());
            new_hi = intMin(int1.hi.clone(), int2.hi.clone());
            new_lo = new_lo.clone() + intMod(x.clone() - new_lo.clone(), new_step.clone());
            if new_hi.clone() < System::intMaxLit() {
                new_hi = new_hi.clone() - intMod(new_hi.clone() - x.clone(), new_step.clone());
            }
            if new_hi.clone() < new_lo.clone() {
                int = newEmpty();
            } else {
                int = new(new_lo.clone(), new_step.clone(), new_hi.clone());
            }
        }
    }
    int
}

pub fn complement(mut int1: Arc<SBInterval>, mut int2: Arc<SBInterval>) -> Result<Arc<UnorderedSet::UnorderedSet<Arc<SBInterval>>>> {
    let mut ints: Arc<UnorderedSet::UnorderedSet<Arc<SBInterval>>>;
    let mut i2: Arc<SBInterval> = Arc::new(<SBInterval as ::std::default::Default>::default());
    let mut count_r: i32 = 0;
    let mut count_s: i32 = 0;
    ints = UnorderedSet::new((std::sync::Arc::new(fnptr!(hash, Arc<SBInterval>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SBInterval>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(isEqual, Arc<SBInterval>, Arc<SBInterval>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SBInterval>, Arc<SBInterval>) -> Result<bool> + 'static>), 13);
    i2 = intersection(int1.clone(), int2.clone());
    if isEmpty(i2.clone()) {
        UnorderedSet::add(int1.clone(), ints.clone())?;
    } else if !(isEqual(int1.clone(), i2.clone())) {
        if i2.hi.clone() < int1.hi.clone() {
            UnorderedSet::add(new(i2.hi.clone() + int1.step.clone(), int1.step.clone(), int1.hi.clone()), ints.clone())?;
        }
        count_r = i2.step.clone() / int1.step.clone() - 1;
        count_s = if (i2.hi.clone() < System::intMaxLit()) {i2.hi.clone() - i2.lo.clone() / i2.step.clone()} else {System::intMaxLit()};
        if count_r.clone() < count_s.clone() {
            if count_s.clone() < System::intMaxLit() {
                for mut i in (1..=count_r.clone()).rev() {
                    UnorderedSet::add(new(i2.lo.clone() + i.clone() * int1.step.clone(), i2.step.clone(), i2.hi.clone() - i2.step.clone() + i.clone() * int1.step.clone()), ints.clone())?;
                }
            } else {
                for mut i in (1..=count_r.clone()).rev() {
                    UnorderedSet::add(new(i2.lo.clone() + i.clone() * int1.step.clone(), i2.step.clone(), System::intMaxLit()), ints.clone())?;
                }
            }
        } else {
            for mut i in (1..=count_s.clone()).rev() {
                UnorderedSet::add(new(i2.lo.clone() + int1.step.clone() + (i.clone() - 1) * i2.step.clone(), int1.step.clone(), i2.lo.clone() - int1.step.clone() + i.clone() * i2.step.clone()), ints.clone())?;
            }
        }
        if i2.lo.clone() > int1.lo.clone() {
            UnorderedSet::add(new(int1.lo.clone(), int1.step.clone(), i2.lo.clone() - int1.step.clone()), ints.clone())?;
        }
    }
    Ok(ints)
}

pub fn affine(mut int: Arc<SBInterval>, mut gain: metamodelica::Real, mut offset: i32) -> Result<Arc<SBInterval>> {
    let mut res: Arc<SBInterval> = Arc::new(<SBInterval as ::std::default::Default>::default());
    let mut lo: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut step: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut hi: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut ilo: i32 = 0;
    let mut istep: i32 = 0;
    let mut ihi: i32 = 0;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(int.clone()) {
        Deref @ SBInterval { lo: __pa0, step: __pa1, hi: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    lo = metamodelica::OrderedFloat((__pa0.clone()) as f64);
    step = metamodelica::OrderedFloat((__pa1.clone()) as f64);
    hi = metamodelica::OrderedFloat((__pa2.clone()) as f64);
    if gain.clone() > metamodelica::OrderedFloat((0) as f64) {
        lo = lo.clone() * gain.clone() + metamodelica::OrderedFloat((offset.clone()) as f64);
        hi = hi.clone() * gain.clone() + metamodelica::OrderedFloat((offset.clone()) as f64);
        step = step.clone() * gain.clone();
        if step.clone() < metamodelica::OrderedFloat((1) as f64) {
            step = metamodelica::OrderedFloat(1.0_f64);
            lo = (lo.clone()).ceil();
            hi = (hi.clone()).floor();
        }
        if lo.clone() < metamodelica::OrderedFloat((0) as f64) {
            lo = lo.clone() + step.clone() * (metamodelica::OrderedFloat((1) as f64) + (lo.clone().abs() / step.clone()).floor());
        }
        if hi.clone() < lo.clone() {
            res = newEmpty();
        } else {
            ilo = ((lo.clone()).0.floor() as i32);
            ihi = ((hi.clone()).0.floor() as i32);
            istep = if (ilo.clone() == ihi.clone()) {1} else {((step.clone()).0.floor() as i32)};
            res = new(ilo.clone(), istep.clone(), ihi.clone());
        }
    } else {
        if offset.clone() > 0 {
            res = new(offset.clone(), 1, offset.clone());
        } else {
            res = newEmpty();
        }
    }
    Ok(res)
}

pub fn cardinality(mut int: Arc<SBInterval>) -> i32 {
    let mut card: i32 = ((intReal(int.hi.clone() - int.lo.clone()) / intReal(int.step.clone())).0 as i32);
    card
}

pub fn contains(mut c: i32, mut int: Arc<SBInterval>) -> bool {
    let mut res: bool = false;
    res = !(isEmpty(int.clone())) && c.clone() >= int.lo.clone() && c.clone() <= int.hi.clone() && intMod(c.clone() - int.lo.clone(), int.step.clone()) == 0;
    res
}

pub fn isEmpty(mut int: Arc<SBInterval>) -> bool {
    let mut res: bool = int.step.clone() == 0;
    res
}

pub fn size(mut int: Arc<SBInterval>) -> i32 {
    let mut res: i32 = intDiv(int.hi.clone() - int.lo.clone(), int.step.clone()) + 1;
    res
}

pub fn isEqual(mut int1: Arc<SBInterval>, mut int2: Arc<SBInterval>) -> bool {
    let mut equal: bool = false;
    equal = int1.lo.clone() == int2.lo.clone() && int1.step.clone() == int2.step.clone() && int1.hi.clone() == int2.hi.clone();
    equal
}

pub fn hash(mut int: Arc<SBInterval>) -> i32 {
    let mut hash: i32 = int.lo.clone();
    hash
}

pub fn toString(mut interval: Arc<SBInterval>) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", interval.lo.clone()))); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", interval.step.clone()))); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", interval.hi.clone()))); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
    r#str
}


