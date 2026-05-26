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

use crate::NFDimension as Dimension;
use crate::NFExpression as Expression;
use crate::NFType as Type;
use openmodelica_util::Error;
use openmodelica_util::Util;

#[derive(Clone, Debug, PartialEq)]
pub enum NFRangeIterator {
    INT_RANGE {
        current: i32,
        last: i32,
    },
    INT_STEP_RANGE {
        current: i32,
        stepsize: i32,
        last: i32,
    },
    REAL_RANGE {
        start: metamodelica::Real,
        stepsize: metamodelica::Real,
        current: i32,
        steps: i32,
    },
    ARRAY_RANGE {
        values: metamodelica::Array<Arc<Expression::NFExpression>>,
        index: i32,
    },
    INVALID_RANGE {
        exp: Arc<Expression::NFExpression>,
    },
}
pub use self::NFRangeIterator::{INT_RANGE,INT_STEP_RANGE,REAL_RANGE,ARRAY_RANGE,INVALID_RANGE};
pub fn isValid(mut iterator: Arc<NFRangeIterator>) -> bool {
    let mut isValid: bool = false;
    isValid = (::match_deref::match_deref! { match &(iterator.clone()) {
        Deref @ INVALID_RANGE { .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isValid
}

pub fn fromExp(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<NFRangeIterator>> {
    let mut iterator: Arc<NFRangeIterator>;
    iterator = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::ARRAY { .. } => {
            Arc::new(NFRangeIterator::ARRAY_RANGE { values: var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone(), index: 1 })
        },
        Deref @ Expression::RANGE { stop: Deref @ Expression::INTEGER { value: istop }, step: Some(Deref @ Expression::INTEGER { value: istep }), start: Deref @ Expression::INTEGER { value: istart }, .. } => {
            Arc::new(NFRangeIterator::INT_STEP_RANGE { current: istart.clone(), stepsize: istep.clone(), last: istop.clone() })
        },
        Deref @ Expression::RANGE { stop: Deref @ Expression::INTEGER { value: istop }, step: None, start: Deref @ Expression::INTEGER { value: istart }, .. } => {
            Arc::new(NFRangeIterator::INT_RANGE { current: istart.clone(), last: istop.clone() })
        },
        Deref @ Expression::RANGE { stop: Deref @ Expression::REAL { value: rstop }, step: Some(Deref @ Expression::REAL { value: rstep }), start: Deref @ Expression::REAL { value: rstart }, .. } => {
            Arc::new(NFRangeIterator::REAL_RANGE { start: rstart.clone(), stepsize: rstep.clone(), current: 0, steps: Util::realRangeSize(rstart.clone(), rstep.clone(), rstop.clone()) })
        },
        Deref @ Expression::RANGE { stop: Deref @ Expression::REAL { value: rstop }, step: None, start: Deref @ Expression::REAL { value: rstart }, .. } => {
            Arc::new(NFRangeIterator::REAL_RANGE { start: rstart.clone(), stepsize: metamodelica::OrderedFloat(1.0_f64), current: 0, steps: Util::realRangeSize(rstart.clone(), metamodelica::OrderedFloat(1.0_f64), rstop.clone()) })
        },
        Deref @ Expression::RANGE { stop: Deref @ Expression::BOOLEAN { value: bstop }, start: Deref @ Expression::BOOLEAN { value: bstart }, .. } => {
            Arc::new(NFRangeIterator::ARRAY_RANGE { values: metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut b in (({let __bs = bstart.clone(); let __be = bstop.clone(); if !__bs && !__be { vec![false] } else if !__bs && __be { vec![false, true] } else if __bs && __be { vec![true] } else { Vec::<bool>::new() }})).into_iter() {
            let __x = Arc::new(Expression::NFExpression::BOOLEAN { value: b.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect()), index: 1 })
        },
        Deref @ Expression::RANGE { stop: Deref @ Expression::ENUM_LITERAL { index: istop, .. }, step: None, start: Deref @ Expression::ENUM_LITERAL { index: istart, ty, .. }, .. } => {
            let mut literals: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut values: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            let __pa0 = ::match_deref::match_deref! { match &(ty.clone()) {
                Deref @ Type::ENUMERATION { literals: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            literals = __pa0.clone();
            values = metamodelica::nil();
            if istart.clone() <= istop.clone() {
                for mut i in 2..=istart.clone() {
                    literals = listRest(literals.clone())?;
                }
                for mut i in istart.clone()..=istop.clone() {
                    values = cons(Arc::new(Expression::NFExpression::ENUM_LITERAL { ty: ty.clone(), name: (listHead(literals.clone())?).clone(), index: i.clone() }), values.clone());
                    literals = listRest(literals.clone())?;
                }
                values = values.clone().reverse();
            }
            Arc::new(NFRangeIterator::ARRAY_RANGE { values: metamodelica::arrayFromVec(values.clone().into_iter().cloned().collect()), index: 1 })
        },
        Deref @ Expression::TYPENAME { ty: Deref @ Type::ARRAY { elementType: ty @ Deref @ Type::ENUMERATION { literals, .. }, .. } } => {
            let mut istep: i32 = 0;
            let mut values: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            values = metamodelica::nil();
            istep = 0;
            for mut l in &*literals.clone() {
                let mut l = l.clone();
                istep = istep.clone() + 1;
                values = cons(Arc::new(Expression::NFExpression::ENUM_LITERAL { ty: ty.clone(), name: (l.clone()).clone(), index: istep.clone() }), values.clone());
            }
            Arc::new(NFRangeIterator::ARRAY_RANGE { values: metamodelica::arrayFromVec(values.clone().into_iter().cloned().collect()), index: 1 })
        },
        _ => {
            Arc::new(NFRangeIterator::INVALID_RANGE { exp: exp.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(iterator)
}

pub fn fromDim(mut dim: Arc<Dimension::NFDimension>, mut resizable: bool) -> Result<Arc<NFRangeIterator>> {
    let mut iterator: Arc<NFRangeIterator>;
    iterator = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ Dimension::INTEGER { .. } => {
            Arc::new(NFRangeIterator::INT_RANGE { current: 1, last: var_field!((*dim).size, Dimension::NFDimension::INTEGER).clone() })
        },
        Deref @ Dimension::BOOLEAN => {
            Arc::new(NFRangeIterator::ARRAY_RANGE { values: metamodelica::arrayFromVec(list![Arc::new(Expression::NFExpression::BOOLEAN { value: false }), Arc::new(Expression::NFExpression::BOOLEAN { value: true })].into_iter().cloned().collect()), index: 1 })
        },
        Deref @ Dimension::ENUM { enumType: ty @ Deref @ Type::ENUMERATION { .. } } => {
            Arc::new(NFRangeIterator::ARRAY_RANGE { values: metamodelica::arrayFromVec(Expression::makeEnumLiterals(ty.clone())?.into_iter().cloned().collect()), index: 1 })
        },
        Deref @ Dimension::EXP { .. } => {
            fromExp(var_field!((*dim).exp, Dimension::NFDimension::EXP).clone())?
        },
        Deref @ Dimension::RESIZABLE { .. } => {
            Arc::new(NFRangeIterator::INT_RANGE { current: 1, last: if (resizable.clone()) {Util::getOptionOrDefault(var_field!((*dim).opt_size, Dimension::NFDimension::RESIZABLE).clone(), var_field!((*dim).size, Dimension::NFDimension::RESIZABLE).clone())} else {var_field!((*dim).size, Dimension::NFDimension::RESIZABLE).clone()} })
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFRangeIterator.fromDim")); __mm_s.push_str(&*literal!(" got unknown dim")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(iterator)
}

pub fn next(mut iterator: Arc<NFRangeIterator>) -> Result<(Arc<NFRangeIterator>, Arc<Expression::NFExpression>)> {
    let mut iterator: Arc<NFRangeIterator> = iterator;
    let mut nextExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    nextExp = (::match_deref::match_deref! { match &(iterator.clone()) {
        Deref @ INT_RANGE { .. } => {
            nextExp = Arc::new(Expression::NFExpression::INTEGER { value: var_field!((*iterator).current, NFRangeIterator::INT_RANGE).clone() });
            assign_variant_field!(iterator => NFRangeIterator::INT_RANGE; current = var_field!((*iterator).current, NFRangeIterator::INT_RANGE).clone() + 1);
            nextExp.clone()
        },
        Deref @ INT_STEP_RANGE { .. } => {
            nextExp = Arc::new(Expression::NFExpression::INTEGER { value: var_field!((*iterator).current, NFRangeIterator::INT_STEP_RANGE).clone() });
            assign_variant_field!(iterator => NFRangeIterator::INT_STEP_RANGE; current = var_field!((*iterator).current, NFRangeIterator::INT_STEP_RANGE).clone() + var_field!((*iterator).stepsize, NFRangeIterator::INT_STEP_RANGE).clone());
            nextExp.clone()
        },
        Deref @ REAL_RANGE { .. } => {
            nextExp = Arc::new(Expression::NFExpression::REAL { value: var_field!((*iterator).start, NFRangeIterator::REAL_RANGE).clone() + var_field!((*iterator).stepsize, NFRangeIterator::REAL_RANGE).clone() * metamodelica::OrderedFloat((var_field!((*iterator).current, NFRangeIterator::REAL_RANGE).clone()) as f64) });
            assign_variant_field!(iterator => NFRangeIterator::REAL_RANGE; current = var_field!((*iterator).current, NFRangeIterator::REAL_RANGE).clone() + 1);
            nextExp.clone()
        },
        Deref @ ARRAY_RANGE { .. } => {
            nextExp = var_field!((*iterator).values, NFRangeIterator::ARRAY_RANGE).clone().borrow()[(var_field!((*iterator).index, NFRangeIterator::ARRAY_RANGE).clone()-1) as usize].clone();
            assign_variant_field!(iterator => NFRangeIterator::ARRAY_RANGE; index = var_field!((*iterator).index, NFRangeIterator::ARRAY_RANGE).clone() + 1);
            nextExp.clone()
        },
        Deref @ INVALID_RANGE { .. } => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFRangeIterator.next")); __mm_s.push_str(&*literal!(" got invalid range ")); __mm_s.push_str(&*Expression::toString(var_field!((*iterator).exp, NFRangeIterator::INVALID_RANGE).clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((iterator, nextExp))
}

pub fn hasNext(mut iterator: Arc<NFRangeIterator>) -> Result<bool> {
    let mut hasNext: bool = false;
    hasNext = (::match_deref::match_deref! { match &(iterator.clone()) {
        Deref @ INT_RANGE { .. } => var_field!((*iterator).current, NFRangeIterator::INT_RANGE).clone() <= var_field!((*iterator).last, NFRangeIterator::INT_RANGE).clone(),
        Deref @ INT_STEP_RANGE { .. } => if (var_field!((*iterator).stepsize, NFRangeIterator::INT_STEP_RANGE).clone() > 0) {var_field!((*iterator).current, NFRangeIterator::INT_STEP_RANGE).clone() <= var_field!((*iterator).last, NFRangeIterator::INT_STEP_RANGE).clone()} else {var_field!((*iterator).current, NFRangeIterator::INT_STEP_RANGE).clone() >= var_field!((*iterator).last, NFRangeIterator::INT_STEP_RANGE).clone()},
        Deref @ REAL_RANGE { .. } => var_field!((*iterator).current, NFRangeIterator::REAL_RANGE).clone() < var_field!((*iterator).steps, NFRangeIterator::REAL_RANGE).clone(),
        Deref @ ARRAY_RANGE { .. } => var_field!((*iterator).index, NFRangeIterator::ARRAY_RANGE).clone() <= (var_field!((*iterator).values, NFRangeIterator::ARRAY_RANGE).clone().borrow().len() as i32),
        Deref @ INVALID_RANGE { .. } => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFRangeIterator.hasNext")); __mm_s.push_str(&*literal!(" got invalid range ")); __mm_s.push_str(&*Expression::toString(var_field!((*iterator).exp, NFRangeIterator::INVALID_RANGE).clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(hasNext)
}

pub fn toList(mut iterator: Arc<NFRangeIterator>) -> Arc<metamodelica::List<Arc<Expression::NFExpression>>> {
    let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = toListReverse(iterator.clone()).unwrap().reverse();
    expl
}

pub fn toListReverse(mut iterator: Arc<NFRangeIterator>) -> Result<Arc<metamodelica::List<Arc<Expression::NFExpression>>>> {
    let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut iter: Arc<NFRangeIterator> = iterator.clone();
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    while hasNext(iter.clone())? {
        (iter, exp) = next(iter.clone())?;
        expl = cons(exp.clone(), expl.clone());
    }
    Ok(expl)
}

pub fn map<T: Clone + 'static>(mut iterator: Arc<NFRangeIterator>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<T> + 'static>) -> Result<Arc<metamodelica::List<T>>> {
    pub type FuncT<T: Clone> = fn(Arc<Expression::NFExpression>) -> Result<T>;

    let mut lst: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut iter: Arc<NFRangeIterator> = iterator.clone();
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    while hasNext(iter.clone())? {
        (iter, exp) = next(iter.clone())?;
        lst = cons(func(exp.clone())?, lst.clone());
    }
    lst = lst.clone().reverse();
    Ok(lst)
}

pub fn fold<ArgT: Clone + 'static>(mut iterator: Arc<NFRangeIterator>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>, mut arg: ArgT) -> Result<ArgT> {
    pub type FuncT<ArgT: Clone> = fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT>;

    let mut arg: ArgT = arg;
    let mut iter: Arc<NFRangeIterator> = iterator.clone();
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    while hasNext(iter.clone())? {
        (iter, exp) = next(iter.clone())?;
        arg = func(exp.clone(), arg.clone())?;
    }
    Ok(arg)
}


