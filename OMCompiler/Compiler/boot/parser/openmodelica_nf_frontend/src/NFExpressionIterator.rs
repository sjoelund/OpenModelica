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

use crate::NFBinding as Binding;
use crate::NFComponentRef as ComponentRef;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFInstNode::InstNode;
use crate::NFSimplifyExp as SimplifyExp;
use openmodelica_util::Error;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NFExpressionIterator {
    ARRAY_ITERATOR {
        arr: metamodelica::Array<Arc<Expression::NFExpression>>,
        index: i32,
        arrays: Arc<metamodelica::List<metamodelica::Array<Arc<Expression::NFExpression>>>>,
    },
    SCALAR_ITERATOR {
        exp: Arc<Expression::NFExpression>,
    },
    EACH_ITERATOR {
        exp: Arc<Expression::NFExpression>,
    },
    NONE_ITERATOR,
    REPEAT_ITERATOR {
        current: Arc<metamodelica::List<Arc<Expression::NFExpression>>>,
        all: Arc<metamodelica::List<Arc<Expression::NFExpression>>>,
    },
}
impl Default for NFExpressionIterator {
    fn default() -> Self { Self::NONE_ITERATOR }
}
pub use self::NFExpressionIterator::{ARRAY_ITERATOR,SCALAR_ITERATOR,EACH_ITERATOR,NONE_ITERATOR,REPEAT_ITERATOR};
pub fn toString(mut iter: Arc<NFExpressionIterator>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(iter.clone()) {
        Deref @ ARRAY_ITERATOR { .. } => List::toString(var_field!((*iter).arrays, NFExpressionIterator::ARRAY_ITERATOR).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(_) -> Result<ArcStr> + 'static> = (std::sync::Arc::new(Expression::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<ArcStr> + 'static>); let __pe_b2 = (literal!("")).clone(); let __pe_b3 = (literal!("{")).clone(); let __pe_b4 = (literal!(", ")).clone(); let __pe_b5 = (literal!("}")).clone(); let __pe_b6 = false; let __pe_b7 = 0; move |__pe_a0| Array::toString(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone(), __pe_b7.clone()) }), (literal!("[ARRY] array iterator:\n")).clone(), (literal!("")).clone(), (literal!("\n")).clone(), (literal!("")).clone(), true, 0)?,
        Deref @ REPEAT_ITERATOR { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[REAP] repeat iterator:\n")); __mm_s.push_str(&*List::toString(var_field!((*iter).all, NFExpressionIterator::REPEAT_ITERATOR).clone(), (std::sync::Arc::new(Expression::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); ArcStr::from(__mm_s) },
        Deref @ SCALAR_ITERATOR { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[SCAL] scalar iterator: ")); __mm_s.push_str(&*Expression::toString(var_field!((*iter).exp, NFExpressionIterator::SCALAR_ITERATOR).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) },
        Deref @ EACH_ITERATOR { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[EACH] each iterator: ")); __mm_s.push_str(&*Expression::toString(var_field!((*iter).exp, NFExpressionIterator::EACH_ITERATOR).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) },
        Deref @ NONE_ITERATOR { .. } => literal!("[NONE] no iterator.\n"),
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(r#str)
}

pub fn fromExp(mut exp: Arc<Expression::NFExpression>, mut backend: bool, mut resize: bool) -> Result<Arc<NFExpressionIterator>> {
    let mut iterator: Arc<NFExpressionIterator> = Arc::new(NFExpressionIterator::NONE_ITERATOR);
    iterator = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::ARRAY { .. } => {
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut expanded: bool = false;
            (e, expanded) = ExpandExp::expand(exp.clone(), backend.clone(), resize.clone())?;
            if !(expanded.clone()) {
                Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpressionIterator.fromExp")); __mm_s.push_str(&*literal!(" got unexpandable expression `")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!("`")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            }
            makeArrayIterator(e.clone())?
        },
        Deref @ Expression::CREF { .. } => {
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            (e, _) = ExpandExp::expandCref(exp.clone(), backend.clone(), false)?;
            iterator = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ Expression::ARRAY { .. } => fromExp(e.clone(), backend.clone(), resize.clone())?,
        _ => Arc::new(NFExpressionIterator::SCALAR_ITERATOR { exp: e.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            iterator.clone()
        },
        _ => {
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut expanded: bool = false;
            (e, expanded) = ExpandExp::expand(exp.clone(), backend.clone(), resize.clone())?;
            if (expanded.clone()) {if (Expression::isEqual(e.clone(), exp.clone())?) {Arc::new(NFExpressionIterator::SCALAR_ITERATOR { exp: exp.clone() })} else {fromExp(e.clone(), backend.clone(), resize.clone())?}} else {Arc::new(crate::NFExpressionIterator::NONE_ITERATOR)}
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(iterator)
}

pub fn fromExpOpt(mut optExp: Option<Arc<Expression::NFExpression>>) -> Result<Arc<NFExpressionIterator>> {
    let mut iterator: Arc<NFExpressionIterator> = Arc::new(NFExpressionIterator::NONE_ITERATOR);
    iterator = (::match_deref::match_deref! { match &(optExp.clone()) {
        Some(exp) => {
            fromExp(exp.clone(), false, false)?
        },
        _ => {
            Arc::new(crate::NFExpressionIterator::NONE_ITERATOR)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(iterator)
}

pub fn fromBinding(mut binding: Arc<Binding::NFBinding>) -> Result<Arc<NFExpressionIterator>> {
    let mut iterator: Arc<NFExpressionIterator> = Arc::new(NFExpressionIterator::NONE_ITERATOR);
    iterator = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ Binding::TYPED_BINDING { eachType: Binding::EachType::EACH, .. } => Arc::new(NFExpressionIterator::EACH_ITERATOR { exp: var_field!((*binding).bindingExp, Binding::NFBinding::TYPED_BINDING).clone() }),
        Deref @ Binding::TYPED_BINDING { .. } => fromExp(var_field!((*binding).bindingExp, Binding::NFBinding::TYPED_BINDING).clone(), false, false)?,
        Deref @ Binding::FLAT_BINDING { .. } => Arc::new(NFExpressionIterator::EACH_ITERATOR { exp: var_field!((*binding).bindingExp, Binding::NFBinding::FLAT_BINDING).clone() }),
        _ => bail!("match: no arm matched"),
    } });
    Ok(iterator)
}

pub fn hasNext(mut iterator: Arc<NFExpressionIterator>) -> Result<bool> {
    let mut hasNext: bool = false;
    hasNext = (::match_deref::match_deref! { match &(iterator.clone()) {
        Deref @ ARRAY_ITERATOR { .. } => var_field!((*iterator).index, NFExpressionIterator::ARRAY_ITERATOR).clone() <= (var_field!((*iterator).arr, NFExpressionIterator::ARRAY_ITERATOR).clone().borrow().len() as i32),
        Deref @ SCALAR_ITERATOR { .. } => true,
        Deref @ EACH_ITERATOR { .. } => true,
        Deref @ NONE_ITERATOR { .. } => false,
        Deref @ REPEAT_ITERATOR { .. } => true,
        _ => bail!("match: no arm matched"),
    } });
    Ok(hasNext)
}

pub fn next(mut iterator: Arc<NFExpressionIterator>) -> Result<(Arc<NFExpressionIterator>, Arc<Expression::NFExpression>)> {
    let mut iterator: Arc<NFExpressionIterator> = iterator;
    let mut nextExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    (iterator, nextExp) = (::match_deref::match_deref! { match &(iterator.clone()) {
        Deref @ ARRAY_ITERATOR { .. } => {
            let mut next: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut arrs: Arc<metamodelica::List<metamodelica::Array<Arc<Expression::NFExpression>>>> = metamodelica::nil();
            next = var_field!((*iterator).arr, NFExpressionIterator::ARRAY_ITERATOR).clone().borrow()[(var_field!((*iterator).index, NFExpressionIterator::ARRAY_ITERATOR).clone()-1) as usize].clone();
            if var_field!((*iterator).index, NFExpressionIterator::ARRAY_ITERATOR).clone() >= (var_field!((*iterator).arr, NFExpressionIterator::ARRAY_ITERATOR).clone().borrow().len() as i32) {
                arrs = var_field!((*iterator).arrays, NFExpressionIterator::ARRAY_ITERATOR).clone();
                while !(arrs.clone().is_empty()) && listHead(arrs.clone())?.borrow().is_empty() {
                    arrs = listRest(arrs.clone())?;
                }
                if arrs.clone().is_empty() {
                    iterator = Arc::new(NFExpressionIterator::ARRAY_ITERATOR { arr: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), index: 1, arrays: metamodelica::nil() });
                } else {
                    iterator = Arc::new(NFExpressionIterator::ARRAY_ITERATOR { arr: listHead(arrs.clone())?, index: 1, arrays: listRest(arrs.clone())? });
                }
            } else {
                todo!("unhandled field-assign shape: iterator.index");
            }
            (iterator.clone(), next.clone())
        },
        Deref @ SCALAR_ITERATOR { .. } => {
            (Arc::new(crate::NFExpressionIterator::NONE_ITERATOR), var_field!((*iterator).exp, NFExpressionIterator::SCALAR_ITERATOR).clone())
        },
        Deref @ EACH_ITERATOR { .. } => {
            (iterator.clone(), var_field!((*iterator).exp, NFExpressionIterator::EACH_ITERATOR).clone())
        },
        Deref @ REPEAT_ITERATOR { current: rest, all: arr } => {
            let mut next: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut rest = (*rest).clone();
            if !(rest.clone().is_empty()) {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                next = __pa0.clone();
                rest = __pa1.clone();
            } else {
                let (__pa2, __pa3) = ::match_deref::match_deref! { match &(arr.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                next = __pa2.clone();
                rest = __pa3.clone();
            }
            (Arc::new(NFExpressionIterator::REPEAT_ITERATOR { current: rest.clone(), all: arr.clone() }), next.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((iterator, nextExp))
}

pub fn nextOpt(mut iterator: Arc<NFExpressionIterator>) -> Result<(Arc<NFExpressionIterator>, Option<Arc<Expression::NFExpression>>)> {
    let mut iterator: Arc<NFExpressionIterator> = iterator;
    let mut nextExp: Option<Arc<Expression::NFExpression>> = None;
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    if hasNext(iterator.clone())? {
        (iterator, exp) = next(iterator.clone())?;
        nextExp = Some(exp.clone());
    } else {
        nextExp = None;
    }
    Ok((iterator, nextExp))
}

pub fn toList(mut iterator: Arc<NFExpressionIterator>) -> Result<Arc<metamodelica::List<Arc<Expression::NFExpression>>>> {
    let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut iter: Arc<NFExpressionIterator> = Arc::new(NFExpressionIterator::NONE_ITERATOR);
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    iter = iterator.clone();
    while hasNext(iter.clone())? {
        (iter, exp) = next(iter.clone())?;
        expl = cons(exp.clone(), expl.clone());
    }
    expl = expl.clone().reverse();
    Ok(expl)
}

pub fn isSubscriptedArrayCall(mut iterator: Arc<NFExpressionIterator>, mut trySimplify: bool) -> Result<bool> {
    fn is_sub_call(mut exp: Arc<Expression::NFExpression>, mut trySimplify: bool) -> Result<bool> {
        let mut res: bool = false;
        res = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::SUBSCRIPTED_EXP { exp: Deref @ Expression::CALL { .. }, .. } => !(trySimplify.clone()) || Expression::isCall(SimplifyExp::simplify(var_field!((*exp).exp, Expression::NFExpression::SUBSCRIPTED_EXP).clone(), false)?),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(res)
    }

    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(iterator.clone()) {
        Deref @ ARRAY_ITERATOR { .. } => is_sub_call(var_field!((*iterator).arr, NFExpressionIterator::ARRAY_ITERATOR).clone().borrow()[(1-1) as usize].clone(), trySimplify.clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

fn makeArrayIterator(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<NFExpressionIterator>> {
    let mut iterator: Arc<NFExpressionIterator> = Arc::new(NFExpressionIterator::NONE_ITERATOR);
    let mut arrays: Arc<metamodelica::List<metamodelica::Array<Arc<Expression::NFExpression>>>> = metamodelica::nil();
    arrays = flattenArray(exp.clone(), metamodelica::nil())?;
    if arrays.clone().is_empty() {
        iterator = Arc::new(NFExpressionIterator::ARRAY_ITERATOR { arr: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), index: 1, arrays: arrays.clone() });
    } else {
        iterator = Arc::new(NFExpressionIterator::ARRAY_ITERATOR { arr: listHead(arrays.clone())?, index: 1, arrays: listRest(arrays.clone())? });
    }
    Ok(iterator)
}

fn flattenArray(mut exp: Arc<Expression::NFExpression>, mut arrays: Arc<metamodelica::List<metamodelica::Array<Arc<Expression::NFExpression>>>>) -> Result<Arc<metamodelica::List<metamodelica::Array<Arc<Expression::NFExpression>>>>> {
    let mut arrays: Arc<metamodelica::List<metamodelica::Array<Arc<Expression::NFExpression>>>> = arrays;
    arrays = flattenArray_impl(exp.clone(), metamodelica::nil())?;
    arrays = arrays.clone().reverse();
    while !(arrays.clone().is_empty()) && listHead(arrays.clone())?.borrow().is_empty() {
        arrays = listRest(arrays.clone())?;
    }
    Ok(arrays)
}

fn flattenArray_impl(mut exp: Arc<Expression::NFExpression>, mut arrays: Arc<metamodelica::List<metamodelica::Array<Arc<Expression::NFExpression>>>>) -> Result<Arc<metamodelica::List<metamodelica::Array<Arc<Expression::NFExpression>>>>> {
    let mut arrays: Arc<metamodelica::List<metamodelica::Array<Arc<Expression::NFExpression>>>> = arrays;
    if Expression::isVector(exp.clone()) {
        arrays = cons(Expression::arrayElements(exp.clone())?, arrays.clone());
    } else {
        let __range0 = Expression::arrayElements(exp.clone())?.borrow().iter().cloned().collect::<Vec<_>>();
        for mut e in __range0 {
            arrays = flattenArray_impl(e.clone(), arrays.clone())?;
        }
    }
    Ok(arrays)
}


