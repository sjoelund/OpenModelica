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
use crate::SBPWLinearMap;
use crate::SBSet;
use crate::System;
use crate::UnorderedSet;
use crate::Util;
use crate::Vector;
use openmodelica_util_datatypes_basic::Array;

pub fn minAtomPW(mut dom: Arc<SBAtomicSet::SBAtomicSet>, mut lm1: Arc<SBLinearMap::SBLinearMap>, mut lm2: Arc<SBLinearMap::SBLinearMap>) -> Result<Arc<SBPWLinearMap::SBPWLinearMap>> {
    fn make_result(mut aset: Arc<SBAtomicSet::SBAtomicSet>, mut map: Arc<SBLinearMap::SBLinearMap>) -> Result<Arc<SBPWLinearMap::SBPWLinearMap>> {
        let mut outMap: Arc<SBPWLinearMap::SBPWLinearMap> = Arc::new(<SBPWLinearMap::SBPWLinearMap as ::std::default::Default>::default());
        let mut set: Arc<SBAtomicSet::SBAtomicSet> = Arc::new(<SBAtomicSet::SBAtomicSet as ::std::default::Default>::default());
        let mut dom: metamodelica::Array<Arc<SBSet::SBSet>>;
        let mut lm: metamodelica::Array<Arc<SBLinearMap::SBLinearMap>>;
        dom = arrayCreate(1, SBSet::addAtomicSet(aset.clone(), SBSet::newEmpty())?);
        lm = arrayCreate(1, map.clone());
        outMap = SBPWLinearMap::new(dom.clone(), lm.clone());
        Ok(outMap)
    }

    let mut outMap: Arc<SBPWLinearMap::SBPWLinearMap> = Arc::new(<SBPWLinearMap::SBPWLinearMap as ::std::default::Default>::default());
    let mut g1: metamodelica::Array<metamodelica::Real>;
    let mut g2: metamodelica::Array<metamodelica::Real>;
    let mut resg: metamodelica::Array<metamodelica::Real>;
    let mut o1: metamodelica::Array<metamodelica::Real>;
    let mut o2: metamodelica::Array<metamodelica::Real>;
    let mut reso: metamodelica::Array<metamodelica::Real>;
    let mut ints: metamodelica::Array<Arc<SBInterval::SBInterval>>;
    let mut as_aux: Arc<SBAtomicSet::SBAtomicSet> = Arc::new(<SBAtomicSet::SBAtomicSet as ::std::default::Default>::default());
    let mut lm_aux: Arc<SBLinearMap::SBLinearMap> = Arc::new(<SBLinearMap::SBLinearMap as ::std::default::Default>::default());
    let mut dom_res: metamodelica::Array<Arc<SBSet::SBSet>>;
    let mut lm_res: metamodelica::Array<Arc<SBLinearMap::SBLinearMap>>;
    let mut s_aux: Arc<SBSet::SBSet> = Arc::new(<SBSet::SBSet as ::std::default::Default>::default());
    let mut d1: Arc<SBSet::SBSet> = Arc::new(<SBSet::SBSet as ::std::default::Default>::default());
    let mut d2: Arc<SBSet::SBSet> = Arc::new(<SBSet::SBSet as ::std::default::Default>::default());
    let mut g1i: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut g2i: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut o1i: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut o2i: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut xinter: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut inti: Arc<SBInterval::SBInterval> = Arc::new(<SBInterval::SBInterval as ::std::default::Default>::default());
    let mut i1: Arc<SBInterval::SBInterval> = Arc::new(<SBInterval::SBInterval as ::std::default::Default>::default());
    let mut i2: Arc<SBInterval::SBInterval> = Arc::new(<SBInterval::SBInterval as ::std::default::Default>::default());
    g1 = SBLinearMap::gain(lm1.clone());
    o1 = SBLinearMap::offset(lm1.clone());
    g2 = SBLinearMap::gain(lm2.clone());
    o2 = SBLinearMap::offset(lm2.clone());
    ints = SBMultiInterval::intervals(SBAtomicSet::aset(dom.clone()));
    as_aux = SBAtomicSet::copy(dom.clone());
    lm_aux = SBLinearMap::copy(lm1.clone());
    resg = metamodelica::arrayFromVec(g1.clone().borrow().clone());
    reso = metamodelica::arrayFromVec(o1.clone().borrow().clone());
    let __range0 = 1..=(g1.clone().borrow().len() as i32);
    for mut i in __range0 {
        g1i = g1.borrow()[(i.clone()-1) as usize].clone();
        g2i = g2.borrow()[(i.clone()-1) as usize].clone();
        o1i = o1.borrow()[(i.clone()-1) as usize].clone();
        o2i = o2.borrow()[(i.clone()-1) as usize].clone();
        inti = ints.borrow()[(i.clone()-1) as usize].clone();
        if g1i.clone() != g2i.clone() {
            xinter = (o2i.clone() - o1i.clone()) / (g1i.clone() - g2i.clone());
            if xinter.clone() <= metamodelica::OrderedFloat((SBInterval::lowerBound(inti.clone())) as f64) {
                if g2i.clone() < g1i.clone() {
                    lm_aux = SBLinearMap::copy(lm2.clone());
                }
                outMap = make_result(as_aux.clone(), lm_aux.clone())?;
            } else if xinter.clone() >= metamodelica::OrderedFloat((SBInterval::upperBound(inti.clone())) as f64) {
                if g2i.clone() > g1i.clone() {
                    lm_aux = SBLinearMap::copy(lm2.clone());
                }
                outMap = make_result(as_aux.clone(), lm_aux.clone())?;
            } else {
                i1 = SBInterval::new(SBInterval::lowerBound(inti.clone()), SBInterval::stepValue(inti.clone()), (((xinter.clone()).floor()).0 as i32));
                i2 = SBInterval::new(SBInterval::upperBound(i1.clone()) + SBInterval::stepValue(i1.clone()), SBInterval::stepValue(inti.clone()), SBInterval::upperBound(inti.clone()));
                d1 = SBSet::addAtomicSet(SBAtomicSet::replace(i1.clone(), i.clone(), as_aux.clone()), SBSet::newEmpty())?;
                d2 = SBSet::addAtomicSet(SBAtomicSet::replace(i2.clone(), i.clone(), as_aux.clone()), SBSet::newEmpty())?;
                dom_res = metamodelica::arrayFromVec(list![d1.clone(), d2.clone()].into_iter().cloned().collect());
                if g1i.clone() > g2i.clone() {
                    lm_res = metamodelica::arrayFromVec(list![SBLinearMap::copy(lm1.clone()), SBLinearMap::copy(lm2.clone())].into_iter().cloned().collect());
                } else {
                    lm_res = metamodelica::arrayFromVec(list![SBLinearMap::copy(lm2.clone()), SBLinearMap::copy(lm1.clone())].into_iter().cloned().collect());
                }
                outMap = SBPWLinearMap::new(dom_res.clone(), lm_res.clone());
            }
            return Ok(outMap);
        } else if o1i.clone() != o2i.clone() {
            if o2i.clone() < o1i.clone() {
                lm_aux = SBLinearMap::copy(lm2.clone());
            }
            outMap = make_result(as_aux.clone(), lm_aux.clone())?;
            return Ok(outMap);
        }
    }
    outMap = make_result(as_aux.clone(), lm_aux.clone())?;
    Ok(outMap)
}

pub fn minPW(mut dom: Arc<SBSet::SBSet>, mut lm1: Arc<SBLinearMap::SBLinearMap>, mut lm2: Arc<SBLinearMap::SBLinearMap>) -> Result<Arc<SBPWLinearMap::SBPWLinearMap>> {
    let mut outMap: Arc<SBPWLinearMap::SBPWLinearMap> = Arc::new(<SBPWLinearMap::SBPWLinearMap as ::std::default::Default>::default());
    let mut aux_dom: metamodelica::Array<Arc<SBSet::SBSet>>;
    let mut aux_lm: metamodelica::Array<Arc<SBLinearMap::SBLinearMap>>;
    let mut sres1: Arc<SBSet::SBSet> = Arc::new(<SBSet::SBSet as ::std::default::Default>::default());
    let mut sres2: Arc<SBSet::SBSet> = Arc::new(<SBSet::SBSet as ::std::default::Default>::default());
    let mut d: Arc<SBSet::SBSet> = Arc::new(<SBSet::SBSet as ::std::default::Default>::default());
    let mut lres1: Arc<SBLinearMap::SBLinearMap> = Arc::new(<SBLinearMap::SBLinearMap as ::std::default::Default>::default());
    let mut lres2: Arc<SBLinearMap::SBLinearMap> = Arc::new(<SBLinearMap::SBLinearMap as ::std::default::Default>::default());
    let mut l: Arc<SBLinearMap::SBLinearMap> = Arc::new(<SBLinearMap::SBLinearMap as ::std::default::Default>::default());
    let mut asets: metamodelica::Array<Arc<SBAtomicSet::SBAtomicSet>>;
    let mut aux: Arc<SBPWLinearMap::SBPWLinearMap> = Arc::new(<SBPWLinearMap::SBPWLinearMap as ::std::default::Default>::default());
    let mut as_aux: Arc<SBAtomicSet::SBAtomicSet> = Arc::new(<SBAtomicSet::SBAtomicSet as ::std::default::Default>::default());
    let mut sres: Arc<metamodelica::List<Arc<SBSet::SBSet>>> = metamodelica::nil();
    let mut lres: Arc<metamodelica::List<Arc<SBLinearMap::SBLinearMap>>> = metamodelica::nil();
    sres1 = SBSet::newEmpty();
    lres1 = SBLinearMap::newEmpty();
    sres2 = SBSet::newEmpty();
    lres2 = SBLinearMap::newEmpty();
    if !(SBSet::isEmpty(dom.clone())) {
        asets = UnorderedSet::toArray(SBSet::asets(dom.clone()));
        as_aux = asets.borrow()[(1-1) as usize].clone();
        aux = minAtomPW(as_aux.clone(), lm1.clone(), lm2.clone())?;
        if !(SBPWLinearMap::isEmpty(aux.clone())) {
            sres1 = SBPWLinearMap::dom(aux.clone()).borrow()[(1-1) as usize].clone();
            lres1 = SBPWLinearMap::lmap(aux.clone()).borrow()[(1-1) as usize].clone();
            let __range0 = 2..=(asets.clone().borrow().len() as i32);
            for mut i in __range0 {
                aux = minAtomPW(asets.borrow()[(i.clone()-1) as usize].clone(), lm1.clone(), lm2.clone())?;
                aux_dom = SBPWLinearMap::dom(aux.clone());
                aux_lm = SBPWLinearMap::lmap(aux.clone());
                let __range1 = 1..=(aux_dom.clone().borrow().len() as i32);
                for mut i in __range1 {
                    d = aux_dom.borrow()[(i.clone()-1) as usize].clone();
                    l = aux_lm.borrow()[(i.clone()-1) as usize].clone();
                    if SBLinearMap::isEqual(l.clone(), lres1.clone()) {
                        sres1 = SBSet::union(sres1.clone(), d.clone())?;
                    } else {
                        if SBSet::isEmpty(sres2.clone()) {
                            sres2 = SBSet::copy(d.clone());
                            lres2 = SBLinearMap::copy(l.clone());
                        } else {
                            sres2 = SBSet::union(sres2.clone(), d.clone())?;
                        }
                    }
                }
            }
        }
    }
    if !(SBSet::isEmpty(sres2.clone())) && !(SBLinearMap::isEmpty(lres2.clone())) {
        sres = cons(sres2.clone(), sres.clone());
        lres = cons(lres2.clone(), lres.clone());
    }
    if !(SBSet::isEmpty(sres1.clone())) && !(SBLinearMap::isEmpty(lres1.clone())) {
        sres = cons(sres1.clone(), sres.clone());
        lres = cons(lres1.clone(), lres.clone());
    }
    outMap = SBPWLinearMap::new(metamodelica::arrayFromVec(sres.clone().into_iter().cloned().collect()), metamodelica::arrayFromVec(lres.clone().into_iter().cloned().collect()));
    Ok(outMap)
}

pub fn minMap(mut pw1: Arc<SBPWLinearMap::SBPWLinearMap>, mut pw2: Arc<SBPWLinearMap::SBPWLinearMap>) -> Result<Arc<SBPWLinearMap::SBPWLinearMap>> {
    let mut outMap: Arc<SBPWLinearMap::SBPWLinearMap> = SBPWLinearMap::newEmpty();
    let mut d1: metamodelica::Array<Arc<SBSet::SBSet>>;
    let mut d2: metamodelica::Array<Arc<SBSet::SBSet>>;
    let mut lm1: metamodelica::Array<Arc<SBLinearMap::SBLinearMap>>;
    let mut lm2: metamodelica::Array<Arc<SBLinearMap::SBLinearMap>>;
    let mut d1i: Arc<SBSet::SBSet> = Arc::new(<SBSet::SBSet as ::std::default::Default>::default());
    let mut dom: Arc<SBSet::SBSet> = Arc::new(<SBSet::SBSet as ::std::default::Default>::default());
    let mut lm1i: Arc<SBLinearMap::SBLinearMap> = Arc::new(<SBLinearMap::SBLinearMap as ::std::default::Default>::default());
    let mut aux: Arc<SBPWLinearMap::SBPWLinearMap> = Arc::new(<SBPWLinearMap::SBPWLinearMap as ::std::default::Default>::default());
    if SBPWLinearMap::isEmpty(pw1.clone()) || SBPWLinearMap::isEmpty(pw2.clone()) {
        return Ok(outMap);
    }
    d1 = SBPWLinearMap::dom(pw1.clone());
    lm1 = SBPWLinearMap::lmap(pw1.clone());
    d2 = SBPWLinearMap::dom(pw2.clone());
    lm2 = SBPWLinearMap::lmap(pw2.clone());
    let __range0 = 1..=(d1.clone().borrow().len() as i32);
    for mut i in __range0 {
        d1i = d1.borrow()[(i.clone()-1) as usize].clone();
        lm1i = lm1.borrow()[(i.clone()-1) as usize].clone();
        let __range1 = 1..=(d2.clone().borrow().len() as i32);
        for mut j in __range1 {
            dom = SBSet::intersection(d1i.clone(), d2.borrow()[(j.clone()-1) as usize].clone())?;
            if !(SBSet::isEmpty(dom.clone())) {
                aux = minPW(dom.clone(), lm1i.clone(), lm2.borrow()[(j.clone()-1) as usize].clone())?;
                outMap = if (SBPWLinearMap::isEmpty(outMap.clone())) {aux.clone()} else {SBPWLinearMap::combine(aux.clone(), outMap.clone())?};
            }
        }
    }
    Ok(outMap)
}

pub fn reduceMapN(mut pw: Arc<SBPWLinearMap::SBPWLinearMap>, mut dim: i32) -> Result<Arc<SBPWLinearMap::SBPWLinearMap>> {
    let mut outMap: Arc<SBPWLinearMap::SBPWLinearMap> = Arc::new(<SBPWLinearMap::SBPWLinearMap as ::std::default::Default>::default());
    let mut dom: metamodelica::Array<Arc<SBSet::SBSet>>;
    let mut new_s: metamodelica::Array<Arc<SBSet::SBSet>>;
    let mut sres: Arc<Vector::Vector<Arc<SBSet::SBSet>>>;
    let mut lmap: metamodelica::Array<Arc<SBLinearMap::SBLinearMap>>;
    let mut new_l: metamodelica::Array<Arc<SBLinearMap::SBLinearMap>>;
    let mut lres: Arc<Vector::Vector<Arc<SBLinearMap::SBLinearMap>>>;
    let mut pw_copy: Arc<SBPWLinearMap::SBPWLinearMap> = Arc::new(<SBPWLinearMap::SBPWLinearMap as ::std::default::Default>::default());
    let mut new_map: Arc<SBPWLinearMap::SBPWLinearMap> = Arc::new(<SBPWLinearMap::SBPWLinearMap as ::std::default::Default>::default());
    let mut di: Arc<SBSet::SBSet> = Arc::new(<SBSet::SBSet as ::std::default::Default>::default());
    let mut new_domi: Arc<SBSet::SBSet> = Arc::new(<SBSet::SBSet as ::std::default::Default>::default());
    let mut li: Arc<SBLinearMap::SBLinearMap> = Arc::new(<SBLinearMap::SBLinearMap as ::std::default::Default>::default());
    let mut new_lm: Arc<SBLinearMap::SBLinearMap> = Arc::new(<SBLinearMap::SBLinearMap as ::std::default::Default>::default());
    let mut gdim: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut odim: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut off: i32 = 0;
    let mut mi: Arc<SBMultiInterval::SBMultiInterval> = Arc::new(<SBMultiInterval::SBMultiInterval as ::std::default::Default>::default());
    let mut idim: Arc<SBInterval::SBInterval> = Arc::new(<SBInterval::SBInterval as ::std::default::Default>::default());
    let mut new_inter: Arc<SBInterval::SBInterval> = Arc::new(<SBInterval::SBInterval as ::std::default::Default>::default());
    let mut loint: i32 = 0;
    let mut hiint: i32 = 0;
    let mut resg: metamodelica::Array<metamodelica::Real>;
    let mut reso: metamodelica::Array<metamodelica::Real>;
    let mut aux_as: Arc<SBAtomicSet::SBAtomicSet> = Arc::new(<SBAtomicSet::SBAtomicSet as ::std::default::Default>::default());
    let mut aux_newd: Arc<UnorderedSet::UnorderedSet<Arc<SBAtomicSet::SBAtomicSet>>>;
    let mut asets: metamodelica::Array<Arc<SBAtomicSet::SBAtomicSet>>;
    dom = SBPWLinearMap::dom(pw.clone());
    lmap = SBPWLinearMap::lmap(pw.clone());
    pw_copy = SBPWLinearMap::copy(pw.clone());
    sres = Vector::fromArray(SBPWLinearMap::dom(pw_copy.clone()));
    lres = Vector::fromArray(SBPWLinearMap::lmap(pw_copy.clone()));
    let __range0 = 1..=(dom.clone().borrow().len() as i32);
    for mut i in __range0 {
        di = dom.borrow()[(i.clone()-1) as usize].clone();
        li = lmap.borrow()[(i.clone()-1) as usize].clone();
        gdim = SBLinearMap::gain(li.clone()).borrow()[(dim.clone()-1) as usize].clone();
        odim = SBLinearMap::offset(li.clone()).borrow()[(dim.clone()-1) as usize].clone();
        if gdim.clone() == metamodelica::OrderedFloat((1) as f64) && odim.clone() < metamodelica::OrderedFloat((0) as f64) {
            off = ((-(odim.clone())).0 as i32);
            asets = UnorderedSet::toArray(SBSet::asets(di.clone()));
            let __range1 = asets.clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut adom in __range1 {
                mi = SBAtomicSet::aset(adom.clone());
                idim = SBMultiInterval::intervals(mi.clone()).borrow()[(dim.clone()-1) as usize].clone();
                loint = SBInterval::lowerBound(idim.clone());
                hiint = SBInterval::upperBound(idim.clone());
                if hiint.clone() - loint.clone() > off.clone() * off.clone() {
                    new_s = metamodelica::arrayCreate(off.clone(), di.clone());
                    new_l = metamodelica::arrayCreate(off.clone(), li.clone());
                    for mut k in 1..=off.clone() {
                        resg = metamodelica::arrayFromVec(SBLinearMap::gain(li.clone()).borrow().clone());
                        reso = metamodelica::arrayFromVec(SBLinearMap::offset(li.clone()).borrow().clone());
                        {
                            let __cell2 = metamodelica::OrderedFloat((0) as f64);
                            resg.clone().borrow_mut()[(dim.clone()-1) as usize] = __cell2;
                        }
                        {
                            let __cell3 = metamodelica::OrderedFloat((loint.clone() + k.clone() - off.clone() - 1) as f64);
                            reso.clone().borrow_mut()[(dim.clone()-1) as usize] = __cell3;
                        }
                        {
                            let __cell4 = SBLinearMap::new(resg.clone(), reso.clone());
                            unsafe { metamodelica::Dangerous::arrayInitSlot(new_l.clone().clone(), k.clone(), __cell4); }
                        }
                        new_inter = SBInterval::new(loint.clone() + k.clone() - 1, off.clone(), hiint.clone());
                        aux_as = SBAtomicSet::replace(new_inter.clone(), dim.clone(), adom.clone());
                        {
                            let __cell5 = SBSet::addAtomicSet(aux_as.clone(), SBSet::newEmpty())?;
                            unsafe { metamodelica::Dangerous::arrayInitSlot(new_s.clone().clone(), k.clone(), __cell5); }
                        }
                    }
                    new_map = SBPWLinearMap::new(new_s.clone(), new_l.clone());
                    aux_newd = UnorderedSet::new((std::sync::Arc::new(fnptr!(SBAtomicSet::hash, Arc<SBAtomicSet::SBAtomicSet>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SBAtomicSet::SBAtomicSet>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(SBAtomicSet::isEqual, Arc<SBAtomicSet::SBAtomicSet>, Arc<SBAtomicSet::SBAtomicSet>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SBAtomicSet::SBAtomicSet>, Arc<SBAtomicSet::SBAtomicSet>) -> Result<bool> + 'static>), 13);
                    let __range6 = asets.clone().borrow().iter().cloned().collect::<Vec<_>>();
                    for mut aux_asi in __range6 {
                        if !(SBAtomicSet::isEqual(aux_asi.clone(), adom.clone())) {
                            UnorderedSet::add(aux_asi.clone(), aux_newd.clone())?;
                        }
                    }
                    new_domi = SBSet::new(aux_newd.clone())?;
                    if SBSet::isEmpty(new_domi.clone()) {
                        if i.clone() < Vector::size(sres.clone()) {
                            Vector::remove(sres.clone(), i.clone())?;
                            Vector::remove(lres.clone(), i.clone())?;
                        } else {
                            Vector::shrink(sres.clone(), i.clone() + 1);
                            Vector::shrink(lres.clone(), i.clone() + 1);
                        }
                    } else {
                        Vector::update(sres.clone(), i.clone(), new_domi.clone())?;
                    }
                    Vector::appendArray(sres.clone(), SBPWLinearMap::dom(new_map.clone()));
                    Vector::appendArray(lres.clone(), SBPWLinearMap::lmap(new_map.clone()));
                }
            }
        }
    }
    outMap = SBPWLinearMap::new(Vector::toArray(sres.clone()), Vector::toArray(lres.clone()));
    Ok(outMap)
}

pub fn mapInf(mut pw: Arc<SBPWLinearMap::SBPWLinearMap>) -> Result<Arc<SBPWLinearMap::SBPWLinearMap>> {
    fn max_inter(mut aset: Arc<SBAtomicSet::SBAtomicSet>, mut offset: metamodelica::Real, mut dim: i32, mut its: metamodelica::Real) -> metamodelica::Real {
        let mut its: metamodelica::Real = its;
        let mut is: metamodelica::Array<Arc<SBInterval::SBInterval>>;
        let mut i: Arc<SBInterval::SBInterval> = Arc::new(<SBInterval::SBInterval as ::std::default::Default>::default());
        let mut hi: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
        let mut lo: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
        is = SBMultiInterval::intervals(SBAtomicSet::aset(aset.clone()));
        i = is.borrow()[(dim.clone()-1) as usize].clone();
        hi = metamodelica::OrderedFloat((SBInterval::upperBound(i.clone())) as f64);
        lo = metamodelica::OrderedFloat((SBInterval::lowerBound(i.clone())) as f64);
        its = std::cmp::max(its.clone(), ((hi.clone() - lo.clone()) / offset.clone().abs()).ceil());
        its
    }

    let mut outMap: Arc<SBPWLinearMap::SBPWLinearMap> = Arc::new(<SBPWLinearMap::SBPWLinearMap as ::std::default::Default>::default());
    let mut max_it: i32 = 0;
    let mut dom: metamodelica::Array<Arc<SBSet::SBSet>>;
    let mut lmap: metamodelica::Array<Arc<SBLinearMap::SBLinearMap>>;
    let mut d: Arc<SBSet::SBSet> = Arc::new(<SBSet::SBSet as ::std::default::Default>::default());
    let mut lm: Arc<SBLinearMap::SBLinearMap> = Arc::new(<SBLinearMap::SBLinearMap as ::std::default::Default>::default());
    let mut gain: metamodelica::Array<metamodelica::Real>;
    let mut off: metamodelica::Array<metamodelica::Real>;
    let mut a: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut b: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut its: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    if SBPWLinearMap::isEmpty(pw.clone()) {
        outMap = SBPWLinearMap::newEmpty();
        return Ok(outMap);
    }
    outMap = reduceMapN(pw.clone(), 1)?;
    for mut i in 2..=SBPWLinearMap::ndim(outMap.clone()) {
        outMap = reduceMapN(pw.clone(), i.clone())?;
    }
    max_it = 0;
    dom = SBPWLinearMap::dom(outMap.clone());
    lmap = SBPWLinearMap::lmap(outMap.clone());
    let __range0 = 1..=(dom.clone().borrow().len() as i32);
    for mut i in __range0 {
        d = dom.borrow()[(i.clone()-1) as usize].clone();
        lm = lmap.borrow()[(i.clone()-1) as usize].clone();
        gain = SBLinearMap::gain(lm.clone());
        off = SBLinearMap::offset(lm.clone());
        a = metamodelica::OrderedFloat((0) as f64);
        b = gain.borrow()[(1-1) as usize].clone();
        let __range1 = 1..=(gain.clone().borrow().len() as i32);
        for mut j in __range1 {
            a = realMax(a.clone(), gain.borrow()[(j.clone()-1) as usize].clone() * off.borrow()[(j.clone()-1) as usize].clone().abs());
            b = realMin(b.clone(), gain.borrow()[(j.clone()-1) as usize].clone());
        }
        if a.clone() > metamodelica::OrderedFloat((0) as f64) {
            its = metamodelica::OrderedFloat((0) as f64);
            for mut dim in 1..=SBPWLinearMap::ndim(outMap.clone()) {
                if gain.borrow()[(dim.clone()-1) as usize].clone() == metamodelica::OrderedFloat((1) as f64) && off.borrow()[(dim.clone()-1) as usize].clone() < metamodelica::OrderedFloat((0) as f64) {
                    its = UnorderedSet::fold(SBSet::asets(d.clone()), Arc::new({ let __pe_b1 = off.borrow()[(dim.clone()-1) as usize].clone(); let __pe_b2 = dim.clone(); move |__pe_a0, __pe_a3| Ok(max_inter(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3)) }), its.clone());
                }
            }
            max_it = max_it.clone() + ((its.clone()).0 as i32);
        } else if b.clone() == metamodelica::OrderedFloat((0) as f64) {
            max_it = max_it.clone() + 1;
        }
    }
    for mut i in 1..=Util::msb(max_it.clone()) {
        outMap = SBPWLinearMap::compPW(outMap.clone(), outMap.clone())?;
    }
    Ok(outMap)
}

pub fn minAdjCompMap(mut pw2: Arc<SBPWLinearMap::SBPWLinearMap>, mut pw1: Arc<SBPWLinearMap::SBPWLinearMap>) -> Result<Arc<SBPWLinearMap::SBPWLinearMap>> {
    let mut outMap: Arc<SBPWLinearMap::SBPWLinearMap> = Arc::new(<SBPWLinearMap::SBPWLinearMap as ::std::default::Default>::default());
    let mut dom: metamodelica::Array<Arc<SBSet::SBSet>>;
    let mut lmap: metamodelica::Array<Arc<SBLinearMap::SBLinearMap>>;
    let mut d: Arc<SBSet::SBSet> = Arc::new(<SBSet::SBSet as ::std::default::Default>::default());
    let mut dom_inv: Arc<SBSet::SBSet> = Arc::new(<SBSet::SBSet as ::std::default::Default>::default());
    let mut aux: Arc<SBSet::SBSet> = Arc::new(<SBSet::SBSet as ::std::default::Default>::default());
    let mut lm_inv: Arc<SBLinearMap::SBLinearMap> = Arc::new(<SBLinearMap::SBLinearMap as ::std::default::Default>::default());
    let mut aux_lm1: Arc<SBLinearMap::SBLinearMap> = Arc::new(<SBLinearMap::SBLinearMap as ::std::default::Default>::default());
    let mut aux_lm2: Arc<SBLinearMap::SBLinearMap> = Arc::new(<SBLinearMap::SBLinearMap as ::std::default::Default>::default());
    let mut lm_res: Arc<SBLinearMap::SBLinearMap> = Arc::new(<SBLinearMap::SBLinearMap as ::std::default::Default>::default());
    let mut inv_pw: Arc<SBPWLinearMap::SBPWLinearMap> = Arc::new(<SBPWLinearMap::SBPWLinearMap as ::std::default::Default>::default());
    let mut aux_inv: Arc<SBPWLinearMap::SBPWLinearMap> = Arc::new(<SBPWLinearMap::SBPWLinearMap as ::std::default::Default>::default());
    let mut aux_res: Arc<SBPWLinearMap::SBPWLinearMap> = Arc::new(<SBPWLinearMap::SBPWLinearMap as ::std::default::Default>::default());
    let mut min_g: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut max_g: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut inf: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut g: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut min_aux: metamodelica::Array<i32>;
    let mut resg: metamodelica::Array<metamodelica::Real>;
    let mut reso: metamodelica::Array<metamodelica::Real>;
    let mut gain: metamodelica::Array<metamodelica::Real>;
    let mut off: metamodelica::Array<metamodelica::Real>;
    let mut gres: metamodelica::Array<metamodelica::Real>;
    let mut oi: metamodelica::Array<metamodelica::Real>;
    let mut ginv: metamodelica::Array<metamodelica::Real>;
    dom = SBPWLinearMap::dom(pw2.clone());
    lmap = SBPWLinearMap::lmap(pw2.clone());
    if (dom.clone().borrow().len() as i32) != 1 {
        outMap = SBPWLinearMap::newEmpty();
        return Ok(outMap);
    }
    d = dom.borrow()[(1-1) as usize].clone();
    dom_inv = SBPWLinearMap::image(pw2.clone(), d.clone())?;
    lm_inv = SBLinearMap::inverse(lmap.borrow()[(1-1) as usize].clone());
    inv_pw = SBPWLinearMap::newScalar(dom_inv.clone(), lm_inv.clone());
    inf = intReal(System::intMaxLit());
    if Array::maxElement(SBLinearMap::gain(lm_inv.clone()), (std::sync::Arc::new(fnptr!(realLt, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<bool> + 'static>)) < inf.clone() {
        outMap = SBPWLinearMap::compPW(pw1.clone(), inv_pw.clone())?;
    } else if Array::minElement(SBLinearMap::gain(lm_inv.clone()), (std::sync::Arc::new(fnptr!(realLt, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<bool> + 'static>)) == inf.clone() {
        if !(SBPWLinearMap::isEmpty(pw2.clone())) {
            aux = SBPWLinearMap::image(pw1.clone(), d.clone())?;
            min_aux = SBSet::minElem(aux.clone())?;
            resg = arrayCreate((min_aux.clone().borrow().len() as i32), metamodelica::OrderedFloat(0.0_f64));
            reso = Array::map(min_aux.clone(), (std::sync::Arc::new(fnptr!(intReal, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<metamodelica::Real> + 'static>));
            lm_res = SBLinearMap::new(resg.clone(), reso.clone());
            outMap = SBPWLinearMap::newScalar(dom_inv.clone(), lm_res.clone());
        } else {
            outMap = SBPWLinearMap::newEmpty();
        }
    } else {
        min_aux = SBSet::minElem(d.clone())?;
        gain = SBLinearMap::gain(lm_inv.clone());
        off = SBLinearMap::offset(lm_inv.clone());
        resg = metamodelica::arrayCreate((gain.clone().borrow().len() as i32), metamodelica::OrderedFloat(0.0_f64));
        reso = metamodelica::arrayCreate((gain.clone().borrow().len() as i32), metamodelica::OrderedFloat(0.0_f64));
        let __range0 = 1..=(gain.clone().borrow().len() as i32);
        for mut i in __range0 {
            g = gain.clone().borrow()[(i.clone()-1) as usize].clone();
            if g.clone() == inf.clone() {
                {
                    let __cell1 = metamodelica::OrderedFloat(0.0_f64);
                    unsafe { metamodelica::Dangerous::arrayInitSlot(resg.clone().clone(), i.clone(), __cell1); }
                }
                {
                    let __cell2 = intReal(min_aux.borrow()[(i.clone()-1) as usize].clone());
                    unsafe { metamodelica::Dangerous::arrayInitSlot(reso.clone().clone(), i.clone(), __cell2); }
                }
            } else {
                {
                    let __cell3 = g.clone();
                    unsafe { metamodelica::Dangerous::arrayInitSlot(resg.clone().clone(), i.clone(), __cell3); }
                }
                {
                    let __cell4 = off.borrow()[(i.clone()-1) as usize].clone();
                    unsafe { metamodelica::Dangerous::arrayInitSlot(reso.clone().clone(), i.clone(), __cell4); }
                }
            }
        }
        aux_lm1 = SBLinearMap::new(resg.clone(), reso.clone());
        aux_inv = SBPWLinearMap::newScalar(dom_inv.clone(), aux_lm1.clone());
        aux_res = SBPWLinearMap::compPW(pw1.clone(), aux_inv.clone())?;
        if SBPWLinearMap::isEmpty(aux_res.clone()) {
            outMap = SBPWLinearMap::newEmpty();
        } else {
            aux = SBPWLinearMap::image(pw1.clone(), d.clone())?;
            min_aux = SBSet::minElem(aux.clone())?;
            lm_res = SBPWLinearMap::lmap(aux_res.clone()).borrow()[(1-1) as usize].clone();
            gres = SBLinearMap::gain(lm_res.clone());
            oi = SBLinearMap::offset(lm_res.clone());
            ginv = SBLinearMap::gain(lm_inv.clone());
            let __range5 = 1..=(gain.clone().borrow().len() as i32);
            for mut i in __range5 {
                g = gain.clone().borrow()[(i.clone()-1) as usize].clone();
                if g.clone() == inf.clone() {
                    {
                        let __cell6 = metamodelica::OrderedFloat(0.0_f64);
                        unsafe { metamodelica::Dangerous::arrayInitSlot(resg.clone().clone(), i.clone(), __cell6); }
                    }
                    {
                        let __cell7 = intReal(min_aux.borrow()[(i.clone()-1) as usize].clone());
                        unsafe { metamodelica::Dangerous::arrayInitSlot(reso.clone().clone(), i.clone(), __cell7); }
                    }
                } else {
                    {
                        let __cell8 = gres.borrow()[(i.clone()-1) as usize].clone();
                        unsafe { metamodelica::Dangerous::arrayInitSlot(resg.clone().clone(), i.clone(), __cell8); }
                    }
                    {
                        let __cell9 = oi.borrow()[(i.clone()-1) as usize].clone();
                        unsafe { metamodelica::Dangerous::arrayInitSlot(reso.clone().clone(), i.clone(), __cell9); }
                    }
                }
            }
            aux_lm2 = SBLinearMap::new(resg.clone(), reso.clone());
            outMap = SBPWLinearMap::newScalar(SBPWLinearMap::dom(aux_res.clone()).borrow()[(1-1) as usize].clone(), aux_lm2.clone());
        }
    }
    Ok(outMap)
}

pub fn minAdjMap(mut pw2: Arc<SBPWLinearMap::SBPWLinearMap>, mut pw1: Arc<SBPWLinearMap::SBPWLinearMap>) -> Result<Arc<SBPWLinearMap::SBPWLinearMap>> {
    let mut outMap: Arc<SBPWLinearMap::SBPWLinearMap> = Arc::new(<SBPWLinearMap::SBPWLinearMap as ::std::default::Default>::default());
    let mut dom2: metamodelica::Array<Arc<SBSet::SBSet>>;
    let mut lm2: metamodelica::Array<Arc<SBLinearMap::SBLinearMap>>;
    let mut map1: Arc<SBPWLinearMap::SBPWLinearMap> = Arc::new(<SBPWLinearMap::SBPWLinearMap as ::std::default::Default>::default());
    let mut mapi: Arc<SBPWLinearMap::SBPWLinearMap> = Arc::new(<SBPWLinearMap::SBPWLinearMap as ::std::default::Default>::default());
    let mut min_adj: Arc<SBPWLinearMap::SBPWLinearMap> = Arc::new(<SBPWLinearMap::SBPWLinearMap as ::std::default::Default>::default());
    let mut min_m: Arc<SBPWLinearMap::SBPWLinearMap> = Arc::new(<SBPWLinearMap::SBPWLinearMap as ::std::default::Default>::default());
    if SBPWLinearMap::isEmpty(pw2.clone()) {
        outMap = SBPWLinearMap::newEmpty();
        return Ok(outMap);
    }
    dom2 = SBPWLinearMap::dom(pw2.clone());
    lm2 = SBPWLinearMap::lmap(pw2.clone());
    map1 = SBPWLinearMap::newScalar(dom2.borrow()[(1-1) as usize].clone(), lm2.borrow()[(1-1) as usize].clone());
    outMap = minAdjCompMap(map1.clone(), pw1.clone())?;
    let __range0 = 1..=(dom2.clone().borrow().len() as i32);
    for mut i in __range0 {
        mapi = SBPWLinearMap::newScalar(dom2.borrow()[(i.clone()-1) as usize].clone(), lm2.borrow()[(i.clone()-1) as usize].clone());
        min_adj = minAdjCompMap(mapi.clone(), pw1.clone())?;
        min_m = minMap(outMap.clone(), min_adj.clone())?;
        outMap = SBPWLinearMap::combine(min_adj.clone(), outMap.clone())?;
        if !(SBPWLinearMap::isEmpty(min_m.clone())) {
            outMap = SBPWLinearMap::combine(min_m.clone(), outMap.clone())?;
        }
    }
    Ok(outMap)
}

pub fn connectedComponents(mut vss: Arc<SBSet::SBSet>, mut emap1: Arc<SBPWLinearMap::SBPWLinearMap>, mut emap2: Arc<SBPWLinearMap::SBPWLinearMap>) -> Result<Arc<SBPWLinearMap::SBPWLinearMap>> {
    let mut outMap: Arc<SBPWLinearMap::SBPWLinearMap> = Arc::new(<SBPWLinearMap::SBPWLinearMap as ::std::default::Default>::default());
    let mut ermap1: Arc<SBPWLinearMap::SBPWLinearMap> = Arc::new(<SBPWLinearMap::SBPWLinearMap as ::std::default::Default>::default());
    let mut ermap2: Arc<SBPWLinearMap::SBPWLinearMap> = Arc::new(<SBPWLinearMap::SBPWLinearMap as ::std::default::Default>::default());
    let mut rmap1: Arc<SBPWLinearMap::SBPWLinearMap> = Arc::new(<SBPWLinearMap::SBPWLinearMap as ::std::default::Default>::default());
    let mut rmap2: Arc<SBPWLinearMap::SBPWLinearMap> = Arc::new(<SBPWLinearMap::SBPWLinearMap as ::std::default::Default>::default());
    let mut new_res: Arc<SBPWLinearMap::SBPWLinearMap> = Arc::new(<SBPWLinearMap::SBPWLinearMap as ::std::default::Default>::default());
    let mut last_im: Arc<SBSet::SBSet> = Arc::new(<SBSet::SBSet as ::std::default::Default>::default());
    let mut new_im: Arc<SBSet::SBSet> = Arc::new(<SBSet::SBSet as ::std::default::Default>::default());
    let mut diff_im: Arc<SBSet::SBSet> = Arc::new(<SBSet::SBSet as ::std::default::Default>::default());
    outMap = SBPWLinearMap::newIdentity(vss.clone());
    new_im = vss.clone();
    diff_im = vss.clone();
    while !(SBSet::isEmpty(diff_im.clone())) {
        ermap1 = SBPWLinearMap::compPW(outMap.clone(), emap1.clone())?;
        ermap2 = SBPWLinearMap::compPW(outMap.clone(), emap2.clone())?;
        rmap1 = minAdjMap(ermap1.clone(), ermap2.clone())?;
        rmap2 = minAdjMap(ermap2.clone(), ermap1.clone())?;
        rmap1 = SBPWLinearMap::combine(rmap1.clone(), outMap.clone())?;
        rmap2 = SBPWLinearMap::combine(rmap2.clone(), outMap.clone())?;
        new_res = minMap(rmap1.clone(), rmap2.clone())?;
        last_im = new_im.clone();
        new_im = SBPWLinearMap::image(new_res.clone(), vss.clone())?;
        diff_im = SBSet::complement(last_im.clone(), new_im.clone())?;
        if !(SBSet::isEmpty(diff_im.clone())) {
            outMap = mapInf(new_res.clone())?;
            new_im = SBPWLinearMap::image(outMap.clone(), vss.clone())?;
        }
    }
    Ok(outMap)
}

pub fn test() -> Result<()> {
    test1()?;
    test2()?;
    test3()?;
    Ok(())
}

pub fn make_set(mut i: Arc<metamodelica::List<Arc<SBInterval::SBInterval>>>) -> Result<Arc<SBSet::SBSet>> {
    let mut s: Arc<SBSet::SBSet> = Arc::new(<SBSet::SBSet as ::std::default::Default>::default());
    let mut ss: Arc<UnorderedSet::UnorderedSet<Arc<SBAtomicSet::SBAtomicSet>>>;
    ss = UnorderedSet::new((std::sync::Arc::new(fnptr!(SBAtomicSet::hash, Arc<SBAtomicSet::SBAtomicSet>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SBAtomicSet::SBAtomicSet>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(SBAtomicSet::isEqual, Arc<SBAtomicSet::SBAtomicSet>, Arc<SBAtomicSet::SBAtomicSet>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SBAtomicSet::SBAtomicSet>, Arc<SBAtomicSet::SBAtomicSet>) -> Result<bool> + 'static>), 13);
    UnorderedSet::add(SBAtomicSet::new(SBMultiInterval::fromList(i.clone())), ss.clone())?;
    s = SBSet::new(ss.clone())?;
    Ok(s)
}

pub fn make_pw(mut i: Arc<metamodelica::List<Arc<SBInterval::SBInterval>>>, mut gain: Arc<metamodelica::List<metamodelica::Real>>, mut offset: Arc<metamodelica::List<metamodelica::Real>>) -> Result<Arc<SBPWLinearMap::SBPWLinearMap>> {
    let mut pw: Arc<SBPWLinearMap::SBPWLinearMap> = Arc::new(<SBPWLinearMap::SBPWLinearMap as ::std::default::Default>::default());
    let mut dom: Arc<SBSet::SBSet> = Arc::new(<SBSet::SBSet as ::std::default::Default>::default());
    let mut lmap: Arc<SBLinearMap::SBLinearMap> = Arc::new(<SBLinearMap::SBLinearMap as ::std::default::Default>::default());
    dom = make_set(i.clone())?;
    lmap = SBLinearMap::new(metamodelica::arrayFromVec(gain.clone().into_iter().cloned().collect()), metamodelica::arrayFromVec(offset.clone().into_iter().cloned().collect()));
    pw = SBPWLinearMap::newScalar(dom.clone(), lmap.clone());
    Ok(pw)
}

pub fn test1() -> Result<()> {
    let mut vss: Arc<SBSet::SBSet> = Arc::new(<SBSet::SBSet as ::std::default::Default>::default());
    let mut emap1: Arc<SBPWLinearMap::SBPWLinearMap> = Arc::new(<SBPWLinearMap::SBPWLinearMap as ::std::default::Default>::default());
    let mut emap2: Arc<SBPWLinearMap::SBPWLinearMap> = Arc::new(<SBPWLinearMap::SBPWLinearMap as ::std::default::Default>::default());
    let mut sets: Arc<metamodelica::List<Arc<SBSet::SBSet>>> = metamodelica::nil();
    let mut pws1: Arc<metamodelica::List<Arc<SBPWLinearMap::SBPWLinearMap>>> = metamodelica::nil();
    let mut pws2: Arc<metamodelica::List<Arc<SBPWLinearMap::SBPWLinearMap>>> = metamodelica::nil();
    let mut res: Arc<SBPWLinearMap::SBPWLinearMap> = Arc::new(<SBPWLinearMap::SBPWLinearMap as ::std::default::Default>::default());
    sets = list![make_set(list![SBInterval::new(1, 1, 1)])?, make_set(list![SBInterval::new(2, 1, 1001)])?, make_set(list![SBInterval::new(1002, 1, 1002)])?, make_set(list![SBInterval::new(1003, 1, 1003)])?, make_set(list![SBInterval::new(1004, 1, 2003)])?, make_set(list![SBInterval::new(2004, 1, 3003)])?, make_set(list![SBInterval::new(3004, 1, 4003)])?];
    vss = SBSet::newEmpty();
    for mut s in &*sets.clone() {
        let mut s = s.clone();
        vss = SBSet::union(vss.clone(), s.clone())?;
    }
    pws1 = list![make_pw(list![SBInterval::new(1, 1, 1)], list![metamodelica::OrderedFloat(0.0_f64)], list![metamodelica::OrderedFloat(1.0_f64)])?, make_pw(list![SBInterval::new(2, 1, 2)], list![metamodelica::OrderedFloat(0.0_f64)], list![metamodelica::OrderedFloat(1002.0_f64)])?, make_pw(list![SBInterval::new(3, 1, 1001)], list![metamodelica::OrderedFloat(1.0_f64)], list![metamodelica::OrderedFloat(1001.0_f64)])?, make_pw(list![SBInterval::new(1002, 1, 2001)], list![metamodelica::OrderedFloat(1.0_f64)], list![metamodelica::OrderedFloat(1002.0_f64)])?, make_pw(list![SBInterval::new(2002, 1, 3001)], list![metamodelica::OrderedFloat(1.0_f64)], list![metamodelica::OrderedFloat(1002.0_f64)])?];
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(pws1.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    emap1 = __pa0.clone();
    pws1 = __pa1.clone();
    for mut pw in &*pws1.clone() {
        let mut pw = pw.clone();
        emap1 = SBPWLinearMap::combine(pw.clone(), emap1.clone())?;
    }
    pws2 = list![make_pw(list![SBInterval::new(1, 1, 1)], list![metamodelica::OrderedFloat(0.0_f64)], list![metamodelica::OrderedFloat(2.0_f64)])?, make_pw(list![SBInterval::new(2, 1, 2)], list![metamodelica::OrderedFloat(0.0_f64)], list![metamodelica::OrderedFloat(1003.0_f64)])?, make_pw(list![SBInterval::new(3, 1, 1001)], list![metamodelica::OrderedFloat(1.0_f64)], list![metamodelica::OrderedFloat(0.0_f64)])?, make_pw(list![SBInterval::new(1002, 1, 2001)], list![metamodelica::OrderedFloat(1.0_f64)], list![metamodelica::OrderedFloat(2.0_f64)])?, make_pw(list![SBInterval::new(2002, 1, 3001)], list![metamodelica::OrderedFloat(0.0_f64)], list![metamodelica::OrderedFloat(1003.0_f64)])?];
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(pws2.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    emap2 = __pa2.clone();
    pws2 = __pa3.clone();
    for mut pw in &*pws2.clone() {
        let mut pw = pw.clone();
        emap2 = SBPWLinearMap::combine(pw.clone(), emap2.clone())?;
    }
    res = connectedComponents(vss.clone(), emap1.clone(), emap2.clone())?;
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*SBPWLinearMap::toString(res.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub fn test2() -> Result<()> {
    let mut vss: Arc<SBSet::SBSet> = Arc::new(<SBSet::SBSet as ::std::default::Default>::default());
    let mut emap1: Arc<SBPWLinearMap::SBPWLinearMap> = Arc::new(<SBPWLinearMap::SBPWLinearMap as ::std::default::Default>::default());
    let mut emap2: Arc<SBPWLinearMap::SBPWLinearMap> = Arc::new(<SBPWLinearMap::SBPWLinearMap as ::std::default::Default>::default());
    let mut sets: Arc<metamodelica::List<Arc<SBSet::SBSet>>> = metamodelica::nil();
    let mut pws1: Arc<metamodelica::List<Arc<SBPWLinearMap::SBPWLinearMap>>> = metamodelica::nil();
    let mut pws2: Arc<metamodelica::List<Arc<SBPWLinearMap::SBPWLinearMap>>> = metamodelica::nil();
    let mut res: Arc<SBPWLinearMap::SBPWLinearMap> = Arc::new(<SBPWLinearMap::SBPWLinearMap as ::std::default::Default>::default());
    sets = list![make_set(list![SBInterval::new(1, 1, 1)])?, make_set(list![SBInterval::new(2, 1, 1001)])?, make_set(list![SBInterval::new(1002, 1, 1002)])?, make_set(list![SBInterval::new(1003, 1, 1003)])?, make_set(list![SBInterval::new(1004, 1, 2003)])?, make_set(list![SBInterval::new(2004, 1, 3003)])?, make_set(list![SBInterval::new(3004, 1, 4003)])?];
    vss = SBSet::newEmpty();
    for mut s in &*sets.clone() {
        let mut s = s.clone();
        vss = SBSet::union(vss.clone(), s.clone())?;
    }
    pws1 = list![make_pw(list![SBInterval::new(1, 1, 1)], list![metamodelica::OrderedFloat(0.0_f64)], list![metamodelica::OrderedFloat(1.0_f64)])?, make_pw(list![SBInterval::new(2, 1, 2)], list![metamodelica::OrderedFloat(0.0_f64)], list![metamodelica::OrderedFloat(1002.0_f64)])?, make_pw(list![SBInterval::new(3, 1, 3)], list![metamodelica::OrderedFloat(0.0_f64)], list![metamodelica::OrderedFloat(1004.0_f64)])?, make_pw(list![SBInterval::new(4, 1, 1002)], list![metamodelica::OrderedFloat(1.0_f64)], list![metamodelica::OrderedFloat(2000.0_f64)])?, make_pw(list![SBInterval::new(1003, 1, 2001)], list![metamodelica::OrderedFloat(1.0_f64)], list![metamodelica::OrderedFloat(2.0_f64)])?, make_pw(list![SBInterval::new(2002, 1, 3001)], list![metamodelica::OrderedFloat(1.0_f64)], list![metamodelica::OrderedFloat(1002.0_f64)])?];
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(pws1.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    emap1 = __pa0.clone();
    pws1 = __pa1.clone();
    for mut pw in &*pws1.clone() {
        let mut pw = pw.clone();
        emap1 = SBPWLinearMap::combine(pw.clone(), emap1.clone())?;
    }
    pws2 = list![make_pw(list![SBInterval::new(1, 1, 1)], list![metamodelica::OrderedFloat(0.0_f64)], list![metamodelica::OrderedFloat(2.0_f64)])?, make_pw(list![SBInterval::new(2, 1, 2)], list![metamodelica::OrderedFloat(0.0_f64)], list![metamodelica::OrderedFloat(1003.0_f64)])?, make_pw(list![SBInterval::new(3, 1, 3)], list![metamodelica::OrderedFloat(0.0_f64)], list![metamodelica::OrderedFloat(1003.0_f64)])?, make_pw(list![SBInterval::new(4, 1, 1002)], list![metamodelica::OrderedFloat(1.0_f64)], list![metamodelica::OrderedFloat(-1.0_f64)])?, make_pw(list![SBInterval::new(1003, 1, 2001)], list![metamodelica::OrderedFloat(1.0_f64)], list![metamodelica::OrderedFloat(1.0_f64)])?, make_pw(list![SBInterval::new(2002, 1, 3001)], list![metamodelica::OrderedFloat(1.0_f64)], list![metamodelica::OrderedFloat(2.0_f64)])?];
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(pws2.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    emap2 = __pa2.clone();
    pws2 = __pa3.clone();
    for mut pw in &*pws2.clone() {
        let mut pw = pw.clone();
        emap2 = SBPWLinearMap::combine(pw.clone(), emap2.clone())?;
    }
    res = connectedComponents(vss.clone(), emap1.clone(), emap2.clone())?;
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*SBPWLinearMap::toString(res.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub fn test3() -> Result<()> {
    let mut vss: Arc<SBSet::SBSet> = Arc::new(<SBSet::SBSet as ::std::default::Default>::default());
    let mut emap1: Arc<SBPWLinearMap::SBPWLinearMap> = Arc::new(<SBPWLinearMap::SBPWLinearMap as ::std::default::Default>::default());
    let mut emap2: Arc<SBPWLinearMap::SBPWLinearMap> = Arc::new(<SBPWLinearMap::SBPWLinearMap as ::std::default::Default>::default());
    let mut res: Arc<SBPWLinearMap::SBPWLinearMap> = Arc::new(<SBPWLinearMap::SBPWLinearMap as ::std::default::Default>::default());
    let mut sets: Arc<metamodelica::List<Arc<SBSet::SBSet>>> = metamodelica::nil();
    let mut pws1: Arc<metamodelica::List<Arc<SBPWLinearMap::SBPWLinearMap>>> = metamodelica::nil();
    let mut pws2: Arc<metamodelica::List<Arc<SBPWLinearMap::SBPWLinearMap>>> = metamodelica::nil();
    sets = list![make_set(list![SBInterval::new(1, 1, 1000), SBInterval::new(1, 1, 100)])?, make_set(list![SBInterval::new(1001, 1, 2000), SBInterval::new(101, 1, 200)])?, make_set(list![SBInterval::new(2001, 1, 3000), SBInterval::new(201, 1, 300)])?, make_set(list![SBInterval::new(3001, 1, 4000), SBInterval::new(301, 1, 400)])?, make_set(list![SBInterval::new(4001, 1, 4001)])?, make_set(list![SBInterval::new(4002, 1, 4002)])?];
    vss = SBSet::newEmpty();
    for mut s in &*sets.clone() {
        let mut s = s.clone();
        vss = SBSet::union(vss.clone(), s.clone())?;
    }
    pws1 = list![make_pw(list![SBInterval::new(1, 1, 999), SBInterval::new(1, 1, 99)], list![metamodelica::OrderedFloat(1.0_f64), metamodelica::OrderedFloat(1.0_f64)], list![metamodelica::OrderedFloat(0.0_f64), metamodelica::OrderedFloat(0.0_f64)])?, make_pw(list![SBInterval::new(1000, 1, 1998), SBInterval::new(100, 1, 198)], list![metamodelica::OrderedFloat(1.0_f64), metamodelica::OrderedFloat(1.0_f64)], list![metamodelica::OrderedFloat(1001.0_f64), metamodelica::OrderedFloat(101.0_f64)])?, make_pw(list![SBInterval::new(1999, 1, 2998), SBInterval::new(199, 1, 199)], list![metamodelica::OrderedFloat(1.0_f64), metamodelica::OrderedFloat(0.0_f64)], list![metamodelica::OrderedFloat(-1998.0_f64), metamodelica::OrderedFloat(100.0_f64)])?, make_pw(list![SBInterval::new(2999, 1, 2999), SBInterval::new(200, 1, 299)], list![metamodelica::OrderedFloat(0.0_f64), metamodelica::OrderedFloat(1.0_f64)], list![metamodelica::OrderedFloat(3001.0_f64), metamodelica::OrderedFloat(101.0_f64)])?, make_pw(list![SBInterval::new(3000, 1, 3000), SBInterval::new(300, 1, 399)], list![metamodelica::OrderedFloat(0.0_f64), metamodelica::OrderedFloat(1.0_f64)], list![metamodelica::OrderedFloat(3000.0_f64), metamodelica::OrderedFloat(-99.0_f64)])?];
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(pws1.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    emap1 = __pa0.clone();
    pws1 = __pa1.clone();
    for mut pw in &*pws1.clone() {
        let mut pw = pw.clone();
        emap1 = SBPWLinearMap::combine(pw.clone(), emap1.clone())?;
    }
    pws2 = list![make_pw(list![SBInterval::new(1, 1, 999), SBInterval::new(1, 1, 99)], list![metamodelica::OrderedFloat(1.0_f64), metamodelica::OrderedFloat(1.0_f64)], list![metamodelica::OrderedFloat(1000.0_f64), metamodelica::OrderedFloat(101.0_f64)])?, make_pw(list![SBInterval::new(1000, 1, 1998), SBInterval::new(100, 1, 198)], list![metamodelica::OrderedFloat(1.0_f64), metamodelica::OrderedFloat(1.0_f64)], list![metamodelica::OrderedFloat(2002.0_f64), metamodelica::OrderedFloat(201.0_f64)])?, make_pw(list![SBInterval::new(1999, 1, 2998), SBInterval::new(199, 1, 199)], list![metamodelica::OrderedFloat(1.0_f64), metamodelica::OrderedFloat(0.0_f64)], list![metamodelica::OrderedFloat(-998.0_f64), metamodelica::OrderedFloat(101.0_f64)])?, make_pw(list![SBInterval::new(2999, 1, 2999), SBInterval::new(200, 1, 299)], list![metamodelica::OrderedFloat(0.0_f64), metamodelica::OrderedFloat(0.0_f64)], list![metamodelica::OrderedFloat(4001.0_f64), metamodelica::OrderedFloat(4001.0_f64)])?, make_pw(list![SBInterval::new(3000, 1, 3000), SBInterval::new(300, 1, 399)], list![metamodelica::OrderedFloat(0.0_f64), metamodelica::OrderedFloat(0.0_f64)], list![metamodelica::OrderedFloat(4002.0_f64), metamodelica::OrderedFloat(4002.0_f64)])?];
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(pws2.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    emap2 = __pa2.clone();
    pws2 = __pa3.clone();
    for mut pw in &*pws2.clone() {
        let mut pw = pw.clone();
        emap2 = SBPWLinearMap::combine(pw.clone(), emap2.clone())?;
    }
    res = connectedComponents(vss.clone(), emap1.clone(), emap2.clone())?;
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*SBPWLinearMap::toString(res.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

