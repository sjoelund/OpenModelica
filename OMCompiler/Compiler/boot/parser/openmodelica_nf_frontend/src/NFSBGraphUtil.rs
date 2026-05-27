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

use crate::NFCeval as Ceval;
use crate::NFDimension as Dimension;
use crate::NFExpression as Expression;
use crate::NFInstContext;
use crate::NFOperator as Operator;
use crate::NFOperator::Op;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFSubscript as Subscript;
use openmodelica_ast::Absyn;
use openmodelica_util::Error;
use openmodelica_util::SBAtomicSet;
use openmodelica_util::SBGraph::IncidenceList;
use openmodelica_util::SBGraph::VertexDescriptor;
use openmodelica_util::SBInterval;
use openmodelica_util::SBLinearMap;
use openmodelica_util::SBMultiInterval;
use openmodelica_util::SBPWLinearMap;
use openmodelica_util::SBSet;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util::Vector;
use openmodelica_util_datatypes_basic::Array;

pub fn multiIntervalFromDimensions(mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>, mut vCount: Arc<Vector::Vector<i32>>) -> Result<Arc<SBMultiInterval::SBMultiInterval>> {
    let mut multiInt: Arc<SBMultiInterval::SBMultiInterval> = Arc::new(<SBMultiInterval::SBMultiInterval as ::std::default::Default>::default());
    let mut new_vCount: Arc<Vector::Vector<i32>>;
    let mut vc: i32 = 0;
    let mut dim_size: i32 = 0;
    let mut index: i32 = 0;
    let mut ints: metamodelica::Array<Arc<SBInterval::SBInterval>>;
    let mut int: Arc<SBInterval::SBInterval> = Arc::new(<SBInterval::SBInterval as ::std::default::Default>::default());
    if dims.clone().is_empty() {
        vc = Vector::get(vCount.clone(), 1)?;
        Vector::update(vCount.clone(), 1, vc.clone() + 1)?;
        multiInt = SBMultiInterval::fromArray(arrayCreate(Vector::size(vCount.clone()), SBInterval::new(vc.clone(), 1, vc.clone())));
    } else {
        ints = arrayCreate(Vector::size(vCount.clone()), SBInterval::newEmpty());
        new_vCount = Vector::copy(vCount.clone());
        index = 1;
        for mut dim in &*dims.clone() {
            let mut dim = dim.clone();
            if !(Dimension::isKnown(dim.clone(), false)) {
                Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFSBGraphUtil.multiIntervalFromDimensions")); __mm_s.push_str(&*literal!(": unknown dimension ")); __mm_s.push_str(&*Dimension::toString(dim.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            }
            dim_size = Dimension::size(dim.clone(), false)?;
            vc = Vector::get(vCount.clone(), index.clone())?;
            int = SBInterval::new(vc.clone(), 1, vc.clone() + dim_size.clone() - 1);
            if SBInterval::isEmpty(int.clone()) {
                ints = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                break;
            } else {
                {
                    let __cell0 = int.clone();
                    ints.clone().borrow_mut()[(index.clone()-1) as usize] = __cell0;
                }
                Vector::update(new_vCount.clone(), index.clone(), vc.clone() + dim_size.clone())?;
            }
            index = index.clone() + 1;
        }
        for mut i in (dims.clone().len() as i32) + 1..=Vector::size(vCount.clone()) {
            vc = Vector::get(vCount.clone(), 1)?;
            {
                let __cell1 = SBInterval::new(vc.clone(), 1, vc.clone());
                ints.clone().borrow_mut()[(i.clone()-1) as usize] = __cell1;
            }
        }
        multiInt = SBMultiInterval::fromArray(ints.clone());
        if !(SBMultiInterval::isEmpty(multiInt.clone())) {
            Vector::swap(new_vCount.clone(), vCount.clone());
        }
    }
    Ok(multiInt)
}

pub fn multiIntervalFromSubscripts(mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut vCount: Arc<Vector::Vector<i32>>, mut multiInt: Arc<SBMultiInterval::SBMultiInterval>) -> Result<Arc<SBMultiInterval::SBMultiInterval>> {
    let mut multiInt: Arc<SBMultiInterval::SBMultiInterval> = multiInt;
    let mut mi: metamodelica::Array<Arc<SBInterval::SBInterval>>;
    let mut miv: metamodelica::Array<Arc<SBInterval::SBInterval>>;
    let mut int: Arc<SBInterval::SBInterval> = Arc::new(<SBInterval::SBInterval as ::std::default::Default>::default());
    let mut index: i32 = 0;
    let mut aux_lo: i32 = 0;
    let mut sub_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    miv = SBMultiInterval::intervals(multiInt.clone());
    if subs.clone().is_empty() {
        mi = Array::map(miv.clone(), (std::sync::Arc::new(fnptr!(make_lo_interval, Arc<SBInterval::SBInterval>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SBInterval::SBInterval>) -> Result<Arc<SBInterval::SBInterval>> + 'static>));
    } else {
        index = 1;
        mi = metamodelica::arrayFromVec(miv.clone().borrow().clone());
        for mut s in &*subs.clone() {
            let mut s = s.clone();
            sub_exp = evalCrefs(Subscript::toExp(s.clone())?)?;
            int = intervalFromExp(sub_exp.clone())?;
            aux_lo = SBInterval::lowerBound(miv.borrow()[(index.clone()-1) as usize].clone()) - 1;
            int = SBInterval::new(aux_lo.clone() + SBInterval::lowerBound(int.clone()), SBInterval::stepValue(int.clone()), aux_lo.clone() + SBInterval::upperBound(int.clone()));
            if !(SBInterval::isEmpty(int.clone())) {
                {
                    let __cell0 = int.clone();
                    mi.clone().borrow_mut()[(index.clone()-1) as usize] = __cell0;
                }
            } else {
                mi = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                break;
            }
            index = index.clone() + 1;
        }
        let __range1 = (subs.clone().len() as i32) + 1..=(mi.clone().borrow().len() as i32);
        for mut i in __range1 {
            aux_lo = SBInterval::lowerBound(miv.borrow()[(i.clone()-1) as usize].clone());
            {
                let __cell2 = SBInterval::new(aux_lo.clone(), 1, aux_lo.clone());
                mi.clone().borrow_mut()[(index.clone()-1) as usize] = __cell2;
            }
        }
    }
    multiInt = SBMultiInterval::fromArray(mi.clone());
    Ok(multiInt)
}

pub fn make_lo_interval(mut i: Arc<SBInterval::SBInterval>) -> Arc<SBInterval::SBInterval> {
    let mut res: Arc<SBInterval::SBInterval> = Arc::new(<SBInterval::SBInterval as ::std::default::Default>::default());
    let mut lo: i32 = SBInterval::lowerBound(i.clone());
    res = SBInterval::new(lo.clone(), 1, lo.clone());
    res
}

pub fn evalCrefs(mut e: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    fn evalCref(mut e: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
        let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        if Expression::isCref(e.clone()) {
            outExp = Ceval::evalExp(e.clone(), Ceval::EvalTarget::new(Absyn::dummyInfo.clone(), NFInstContext::ITERATION_RANGE.clone(), None))?;
        } else {
            outExp = e.clone();
        }
        Ok(outExp)
    }

    let mut e: Arc<Expression::NFExpression> = e;
    e = Expression::map(e.clone(), (std::sync::Arc::new(evalCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    Ok(e)
}

pub fn intervalFromExp(mut e: Arc<Expression::NFExpression>) -> Result<Arc<SBInterval::SBInterval>> {
    let mut i: Arc<SBInterval::SBInterval> = Arc::new(<SBInterval::SBInterval as ::std::default::Default>::default());
    i = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ Expression::INTEGER { .. } => SBInterval::new(var_field!((*e).value, Expression::NFExpression::INTEGER).clone(), 1, var_field!((*e).value, Expression::NFExpression::INTEGER).clone()),
        Deref @ Expression::BOOLEAN { .. } => SBInterval::new(Util::boolInt(var_field!((*e).value, Expression::NFExpression::BOOLEAN).clone()), 1, Util::boolInt(var_field!((*e).value, Expression::NFExpression::BOOLEAN).clone())),
        Deref @ Expression::REAL { .. } => SBInterval::new(((var_field!((*e).value, Expression::NFExpression::REAL).clone()).0 as i32), 1, ((var_field!((*e).value, Expression::NFExpression::REAL).clone()).0 as i32)),
        Deref @ Expression::BINARY { .. } => intervalFromBinaryExp(var_field!((*e).exp1, Expression::NFExpression::BINARY).clone(), var_field!((*e).operator, Expression::NFExpression::BINARY).clone(), var_field!((*e).exp2, Expression::NFExpression::BINARY).clone())?,
        Deref @ Expression::UNARY { .. } => intervalFromUnaryExp(var_field!((*e).exp, Expression::NFExpression::UNARY).clone())?,
        Deref @ Expression::RANGE { .. } => intervalFromRange(e.clone())?,
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFSBGraphUtil.intervalFromExp")); __mm_s.push_str(&*literal!(" got unknown expression ")); __mm_s.push_str(&*Expression::toString(e.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(i)
}

pub fn intervalFromBinaryExp(mut lhs: Arc<Expression::NFExpression>, mut op: Arc<Operator::NFOperator>, mut rhs: Arc<Expression::NFExpression>) -> Result<Arc<SBInterval::SBInterval>> {
    let mut i: Arc<SBInterval::SBInterval> = Arc::new(<SBInterval::SBInterval as ::std::default::Default>::default());
    let mut lhs_i: Arc<SBInterval::SBInterval> = Arc::new(<SBInterval::SBInterval as ::std::default::Default>::default());
    let mut rhs_i: Arc<SBInterval::SBInterval> = Arc::new(<SBInterval::SBInterval as ::std::default::Default>::default());
    let mut lhs_sz: i32 = 0;
    let mut rhs_sz: i32 = 0;
    let mut res: i32 = 0;
    let mut llo: i32 = 0;
    let mut rlo: i32 = 0;
    let mut lhi: i32 = 0;
    let mut rhi: i32 = 0;
    let mut step: i32 = 0;
    lhs_i = intervalFromExp(lhs.clone())?;
    rhs_i = intervalFromExp(rhs.clone())?;
    lhs_sz = SBInterval::size(lhs_i.clone());
    rhs_sz = SBInterval::size(rhs_i.clone());
    llo = SBInterval::lowerBound(lhs_i.clone());
    rlo = SBInterval::lowerBound(rhs_i.clone());
    if lhs_sz.clone() == 1 && rhs_sz.clone() == 1 {
        let __pa0 = ::match_deref::match_deref! { match &(Ceval::evalBinaryOp_dispatch(Arc::new(Expression::NFExpression::INTEGER { value: llo.clone() }), op.clone(), Arc::new(Expression::NFExpression::INTEGER { value: rlo.clone() }), Ceval::noTarget().clone())?) {
            Deref @ Expression::INTEGER { value: __pa0 } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        res = __pa0.clone();
        i = SBInterval::new(res.clone(), 1, res.clone());
    } else if lhs_sz.clone() == 1 || rhs_sz.clone() == 1 {
        lhi = SBInterval::upperBound(lhs_i.clone());
        rhi = SBInterval::upperBound(rhs_i.clone());
        step = SBInterval::stepValue(if (lhs_sz.clone() == 1) {rhs_i.clone()} else {lhs_i.clone()});
        i = (match op.op.clone() {
        Operator::Op::ADD => SBInterval::new(llo.clone() + rlo.clone(), step.clone(), lhi.clone() + rhi.clone()),
        Operator::Op::SUB => SBInterval::new(llo.clone() - rlo.clone(), step.clone(), lhi.clone() - rhi.clone()),
        Operator::Op::MUL => SBInterval::new(llo.clone() * rlo.clone(), llo.clone() * step.clone(), lhi.clone() * rhi.clone()),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFSBGraphUtil.intervalFromBinaryExp")); __mm_s.push_str(&*literal!(" got unknown operator ")); __mm_s.push_str(&*Operator::symbol(op.clone(), (literal!(" ")).clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
    });
    } else {
        Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFSBGraphUtil.intervalFromBinaryExp")); __mm_s.push_str(&*literal!(" got unknown expression ")); __mm_s.push_str(&*Expression::toString(Arc::new(Expression::NFExpression::BINARY { exp1: lhs.clone(), operator: op.clone(), exp2: rhs.clone() }))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
    }
    Ok(i)
}

pub fn intervalFromUnaryExp(mut e: Arc<Expression::NFExpression>) -> Result<Arc<SBInterval::SBInterval>> {
    let mut i: Arc<SBInterval::SBInterval> = Arc::new(<SBInterval::SBInterval as ::std::default::Default>::default());
    i = intervalFromExp(e.clone())?;
    i = SBInterval::new(-(SBInterval::lowerBound(i.clone())), 1, -(SBInterval::upperBound(i.clone())));
    Ok(i)
}

pub fn intervalFromRange(mut e: Arc<Expression::NFExpression>) -> Result<Arc<SBInterval::SBInterval>> {
    let mut i: Arc<SBInterval::SBInterval> = Arc::new(<SBInterval::SBInterval as ::std::default::Default>::default());
    let mut start: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut stop: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ostep: Option<Arc<Expression::NFExpression>> = None;
    let mut lo: i32 = 0;
    let mut step: i32 = 0;
    let mut hi: i32 = 0;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(SimplifyExp::simplify(e.clone(), false)?) {
        Deref @ Expression::RANGE { stop: __pa0, step: __pa1, start: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    stop = __pa0.clone();
    ostep = __pa1.clone();
    start = __pa2.clone();
    lo = Expression::toInteger(start.clone())?;
    hi = Expression::toInteger(stop.clone())?;
    if isSome(ostep.clone()) {
        step = Expression::toInteger(Util::getOption(ostep.clone())?)?;
    } else {
        step = 1;
    }
    i = SBInterval::new(lo.clone(), step.clone(), hi.clone());
    Ok(i)
}

pub fn linearMapFromIntervals(mut d1: i32, mut d2: i32, mut mi1: Arc<SBMultiInterval::SBMultiInterval>, mut mi2: Arc<SBMultiInterval::SBMultiInterval>, mut eCount: Arc<Vector::Vector<i32>>) -> Result<(ArcStr, Arc<SBPWLinearMap::SBPWLinearMap>, Arc<SBPWLinearMap::SBPWLinearMap>)> {
    let mut name: ArcStr = arcstr::literal!("");
    let mut pw1: Arc<SBPWLinearMap::SBPWLinearMap> = Arc::new(<SBPWLinearMap::SBPWLinearMap as ::std::default::Default>::default());
    let mut pw2: Arc<SBPWLinearMap::SBPWLinearMap> = Arc::new(<SBPWLinearMap::SBPWLinearMap as ::std::default::Default>::default());
    let mut ints1: metamodelica::Array<Arc<SBInterval::SBInterval>>;
    let mut ints2: metamodelica::Array<Arc<SBInterval::SBInterval>>;
    let mut mi: metamodelica::Array<Arc<SBInterval::SBInterval>>;
    let mut mi1_sz: i32 = 0;
    let mut mi2_sz: i32 = 0;
    let mut sz: i32 = 0;
    let mut sz1: i32 = 0;
    let mut sz2: i32 = 0;
    let mut count: i32 = 0;
    let mut aux_ec: i32 = 0;
    let mut g1: metamodelica::Array<metamodelica::Real>;
    let mut g2: metamodelica::Array<metamodelica::Real>;
    let mut o1: metamodelica::Array<metamodelica::Real>;
    let mut o2: metamodelica::Array<metamodelica::Real>;
    let mut g1i: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut g2i: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut o1i: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut o2i: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut i1: Arc<SBInterval::SBInterval> = Arc::new(<SBInterval::SBInterval as ::std::default::Default>::default());
    let mut i2: Arc<SBInterval::SBInterval> = Arc::new(<SBInterval::SBInterval as ::std::default::Default>::default());
    let mut new_ec: Arc<Vector::Vector<i32>>;
    let mut s: Arc<SBSet::SBSet> = Arc::new(<SBSet::SBSet as ::std::default::Default>::default());
    let mut lm1: Arc<SBLinearMap::SBLinearMap> = Arc::new(<SBLinearMap::SBLinearMap as ::std::default::Default>::default());
    let mut lm2: Arc<SBLinearMap::SBLinearMap> = Arc::new(<SBLinearMap::SBLinearMap as ::std::default::Default>::default());
    ints1 = SBMultiInterval::intervals(mi1.clone());
    mi1_sz = SBMultiInterval::size(mi1.clone());
    ints2 = SBMultiInterval::intervals(mi2.clone());
    mi2_sz = SBMultiInterval::size(mi2.clone());
    if SBMultiInterval::ndim(mi1.clone()) != SBMultiInterval::ndim(mi2.clone()) && mi1_sz.clone() != 1 && mi2_sz.clone() != 1 {
        Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFSBGraphUtil.linearMapFromIntervals")); __mm_s.push_str(&*literal!(" got incompatible connect")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
    }
    sz = (ints1.clone().borrow().len() as i32);
    g1 = metamodelica::arrayCreate(sz.clone(), metamodelica::OrderedFloat(0.0_f64));
    g2 = metamodelica::arrayCreate(sz.clone(), metamodelica::OrderedFloat(0.0_f64));
    o1 = metamodelica::arrayCreate(sz.clone(), metamodelica::OrderedFloat(0.0_f64));
    o2 = metamodelica::arrayCreate(sz.clone(), metamodelica::OrderedFloat(0.0_f64));
    mi = metamodelica::arrayCreate(sz.clone(), ints1.borrow()[(1-1) as usize].clone());
    new_ec = Vector::new(0);
    for mut i in 1..=sz.clone() {
        sz1 = SBInterval::size(ints1.borrow()[(i.clone()-1) as usize].clone());
        sz2 = SBInterval::size(ints2.borrow()[(i.clone()-1) as usize].clone());
        if sz1.clone() != sz2.clone() && sz1.clone() != 1 && sz2.clone() != 1 {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFSBGraphUtil.linearMapFromIntervals")); __mm_s.push_str(&*literal!(" got incompatible connect")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
        }
        count = std::cmp::max(sz1.clone(), sz2.clone());
        aux_ec = Vector::get(eCount.clone(), i.clone())?;
        {
            let __cell0 = SBInterval::new(aux_ec.clone(), 1, aux_ec.clone() + count.clone() - 1);
            unsafe { metamodelica::Dangerous::arrayInitSlot(mi.clone().clone(), i.clone(), __cell0); }
        }
        i1 = ints1.borrow()[(i.clone()-1) as usize].clone();
        i2 = ints2.borrow()[(i.clone()-1) as usize].clone();
        if sz1.clone() == 1 {
            {
                let __cell1 = metamodelica::OrderedFloat(0.0_f64);
                unsafe { metamodelica::Dangerous::arrayInitSlot(g1.clone().clone(), i.clone(), __cell1); }
            }
            {
                let __cell2 = metamodelica::OrderedFloat((SBInterval::lowerBound(i1.clone())) as f64);
                unsafe { metamodelica::Dangerous::arrayInitSlot(o1.clone().clone(), i.clone(), __cell2); }
            }
        } else {
            g1i = metamodelica::OrderedFloat((SBInterval::stepValue(i1.clone())) as f64);
            o1i = -(g1i.clone() * metamodelica::OrderedFloat((aux_ec.clone()) as f64)) + metamodelica::OrderedFloat((SBInterval::lowerBound(i1.clone())) as f64);
            {
                let __cell3 = g1i.clone();
                unsafe { metamodelica::Dangerous::arrayInitSlot(g1.clone().clone(), i.clone(), __cell3); }
            }
            {
                let __cell4 = o1i.clone();
                unsafe { metamodelica::Dangerous::arrayInitSlot(o1.clone().clone(), i.clone(), __cell4); }
            }
        }
        if sz2.clone() == 1 {
            {
                let __cell5 = metamodelica::OrderedFloat(0.0_f64);
                unsafe { metamodelica::Dangerous::arrayInitSlot(g2.clone().clone(), i.clone(), __cell5); }
            }
            {
                let __cell6 = metamodelica::OrderedFloat((SBInterval::lowerBound(i2.clone())) as f64);
                unsafe { metamodelica::Dangerous::arrayInitSlot(o2.clone().clone(), i.clone(), __cell6); }
            }
        } else {
            g2i = metamodelica::OrderedFloat((SBInterval::stepValue(i2.clone())) as f64);
            o2i = -(g2i.clone() * metamodelica::OrderedFloat((aux_ec.clone()) as f64)) + metamodelica::OrderedFloat((SBInterval::lowerBound(i2.clone())) as f64);
            {
                let __cell7 = g2i.clone();
                unsafe { metamodelica::Dangerous::arrayInitSlot(g2.clone().clone(), i.clone(), __cell7); }
            }
            {
                let __cell8 = o2i.clone();
                unsafe { metamodelica::Dangerous::arrayInitSlot(o2.clone().clone(), i.clone(), __cell8); }
            }
        }
        Vector::push(new_ec.clone(), aux_ec.clone() + count.clone());
    }
    Vector::swap(eCount.clone(), new_ec.clone());
    s = SBSet::newEmpty();
    s = SBSet::addAtomicSet(SBAtomicSet::new(SBMultiInterval::fromArray(mi.clone())), s.clone())?;
    lm1 = SBLinearMap::new(g1.clone(), o1.clone());
    lm2 = SBLinearMap::new(g2.clone(), o2.clone());
    pw1 = SBPWLinearMap::newScalar(s.clone(), lm1.clone());
    pw2 = SBPWLinearMap::newScalar(s.clone(), lm2.clone());
    name = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("E")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", System::tmpTick()))); ArcStr::from(__mm_s) }).clone();
    Ok((name, pw1, pw2))
}

