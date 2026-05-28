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

use crate::NBAdjacency::Dependency;
use crate::NBAdjacency::Mapping;
use crate::NBAdjacency::Mode;
use crate::NBBackendUtil as BackendUtil;
use crate::NBEquation::Equation;
use crate::NBEquation::Frame;
use crate::NBEquation::FrameLocation;
use crate::NBEquation::FrameOrderingStatus;
use crate::NBEquation::Iterator;
use crate::NBEquation::RecollectStatus;
use crate::NBReplacements as Replacements;
use crate::NBVariable as BVariable;
use crate::NBVariable::VariablePointers;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFComplexType as ComplexType;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFDimension as Dimension;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFSimplifyExp as SimplifyExp;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::Error;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

/// file:         NBSlice.mo
///  package:      NBSlice
///  description:  This file contains util functions for slicing operations.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NBSlice<T: Clone> {
    pub t: T,
    pub indices: IntLst,
}

pub type SLICE<T> = NBSlice<T>;

pub type IntLst = Arc<metamodelica::List<i32>>;

pub fn getT<T: Clone + 'static>(mut slice: Arc<NBSlice<T>>) -> T {
    let mut t: T = slice.t.clone();
    t
}

pub fn hash<T: Clone + 'static>(mut slice: Arc<NBSlice<T>>, mut func: Arc<dyn ::std::ops::Fn(T) -> Result<i32> + 'static>) -> i32 {
    let mut h: i32 = func(slice.t.clone()).unwrap();
    for mut i in &*List::firstOrEmpty(slice.indices.clone()) {
        let mut i = i.clone();
        h = stringHashDjb2Continue((intString(i.clone())).clone(), h.clone());
    }
    h
}

pub fn isEqual<T: Clone + 'static>(mut slice1: Arc<NBSlice<T>>, mut slice2: Arc<NBSlice<T>>, mut func: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>) -> bool {
    let mut b: bool = func(slice1.t.clone(), slice2.t.clone()).unwrap() && List::isEqualOnTrue(slice1.indices.clone(), slice2.indices.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>));
    b
}

pub fn toString<T: Clone + 'static>(mut slice: Arc<NBSlice<T>>, mut func: Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>, mut maxLength: i32) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = (func(slice.t.clone())?).clone();
    if maxLength.clone() > 0 && !(slice.indices.clone().is_empty()) {
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n\tslice: ")); __mm_s.push_str(&*List::toString(slice.indices.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, maxLength.clone())?); ArcStr::from(__mm_s) }).clone();
    }
    Ok(r#str)
}

pub fn lstToString<T: Clone + 'static>(mut lst: Arc<metamodelica::List<Arc<NBSlice<T>>>>, mut func: toStringT<T>, mut maxLength: i32) -> ArcStr {
    pub use toStringT as toStringT_;

    let mut r#str: ArcStr = List::toString(lst.clone(), Arc::new({ let __pe_b1 = (std::sync::Arc::new(func) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<_> + 'static>); let __pe_b2 = maxLength.clone(); move |__pe_a0| toString(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }), (literal!("")).clone(), (literal!("\t")).clone(), (literal!(";\n\t")).clone(), (literal!(";")).clone(), false, 0).unwrap();
    r#str
}

pub fn isFull<T: Clone + 'static>(mut slice: Arc<NBSlice<T>>) -> bool {
    let mut b: bool = slice.indices.clone().is_empty();
    b
}

pub fn size<T: Clone + 'static>(mut slice: Arc<NBSlice<T>>, mut func: Arc<dyn ::std::ops::Fn(T) -> Result<i32> + 'static>) -> i32 {
    let mut s: i32 = 0;
    if slice.indices.clone().is_empty() {
        s = func(slice.t.clone()).unwrap();
    } else {
        s = (slice.indices.clone().len() as i32);
    }
    s
}

pub fn simplify<T: Clone + 'static>(mut slice: Arc<NBSlice<T>>, mut func: Arc<dyn ::std::ops::Fn(T) -> Result<i32> + 'static>) -> Result<Arc<NBSlice<T>>> {
    let mut slice: Arc<NBSlice<T>> = slice;
    if (slice.indices.clone().len() as i32) == func(slice.t.clone())? {
        assign_field!(slice.indices = metamodelica::nil());
    } else {
        assign_field!(slice.indices = List::sort(slice.indices.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?);
    }
    Ok(slice)
}

pub fn addToSliceMap<T: Clone + 'static + Default>(mut t: T, mut i: i32, mut map: Arc<UnorderedMap::UnorderedMap<T, Arc<metamodelica::List<i32>>>>) -> Result<()> {
    UnorderedMap::add(t.clone(), cons(i.clone(), UnorderedMap::getOrDefault(t.clone(), map.clone(), metamodelica::nil())), map.clone())?;
    Ok(())
}

pub fn fromTpl<T: Clone + 'static>(mut tpl: (T, Arc<metamodelica::List<i32>>)) -> Arc<NBSlice<T>> {
    let mut slice: Arc<NBSlice<T>>;
    let mut t: T;
    let mut lst: IntLst = metamodelica::nil();
    (t, lst) = tpl.clone();
    slice = Arc::new(NBSlice { t: t.clone(), indices: lst.clone() });
    slice
}

pub fn fromMap<T: Clone + 'static>(mut map: Arc<UnorderedMap::UnorderedMap<T, Arc<metamodelica::List<i32>>>>) -> Arc<metamodelica::List<Arc<NBSlice<T>>>> {
    let mut slices: Arc<metamodelica::List<Arc<NBSlice<T>>>> = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut tpl in (UnorderedMap::toList(map.clone())).into_iter().cloned() {
            let __x = fromTpl(tpl.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    slices
}

pub fn apply<T: Clone + 'static>(mut slice: Arc<NBSlice<T>>, mut func: Arc<dyn ::std::ops::Fn(T) -> Result<T> + 'static>) -> Arc<NBSlice<T>> {
    let mut slice: Arc<NBSlice<T>> = slice;
    assign_field!(slice.t = func(slice.t.clone()).unwrap());
    slice
}

pub fn applyMutable<T: Clone + 'static>(mut slice: Arc<NBSlice<T>>, mut func: Arc<dyn ::std::ops::Fn(T) -> Result<()> + 'static>) -> () {
    func(slice.t.clone()).unwrap();
    ()
}

pub fn check<T: Clone + 'static, T2: Clone + 'static>(mut slice: Arc<NBSlice<T>>, mut func: Arc<dyn ::std::ops::Fn(T) -> Result<T2> + 'static>) -> T2 {
    let mut t2: T2 = func(slice.t.clone()).unwrap();
    t2
}

pub type toStringT<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>;

pub type sizeT<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<i32> + 'static>;

pub type hashT<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<i32> + 'static>;

pub type isEqualT<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

pub type applyT<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<T> + 'static>;

pub type applyMutableT<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<()> + 'static>;

pub type checkT<T2: Clone + 'static, T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<T2> + 'static>;

pub type filterCref = std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<ComponentRef::NFComponentRef>> + 'static>;

pub type getDependentCrefIndices = std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, Arc<Mapping::Mapping>, i32) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<metamodelica::Array<i32>>)> + 'static>;

pub fn filterExp(mut exp: Arc<Expression::NFExpression>, mut filter: Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<ComponentRef::NFComponentRef>> + 'static>, mut acc: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => {
            filter(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), acc.clone())?;
            ()
        },
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_REDUCTION { exp: call_exp, .. } } => {
            let mut call_exp = (*call_exp).clone();
            for mut iter in &*var_field!((**call).iters, Call::NFCall::TYPED_REDUCTION).clone() {
                let mut iter = iter.clone();
                call_exp = Expression::replaceIterator(call_exp.clone(), Util::tuple21(iter.clone()), Util::tuple22(iter.clone()))?;
            }
            let _ = Expression::mapShallow(call_exp.clone(), Arc::new({ let __pe_b1 = filter.clone(); let __pe_b2 = acc.clone(); move |__pe_a0| filterExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
            ()
        },
        _ => {
            let _ = Expression::mapShallow(exp.clone(), Arc::new({ let __pe_b1 = filter.clone(); let __pe_b2 = acc.clone(); move |__pe_a0| filterExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn getContinuous(mut cref: Arc<ComponentRef::NFComponentRef>, mut acc: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut init: bool) -> Result<Arc<ComponentRef::NFComponentRef>> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = cref;
    if BVariable::checkCref(cref.clone(), Arc::new({ let __pe_b1 = init.clone(); move |__pe_a0| BVariable::isContinuous(__pe_a0, __pe_b1.clone()) }), metamodelica::sourceInfo!()) {
        UnorderedSet::add(cref.clone(), acc.clone())?;
    }
    Ok(cref)
}

pub fn getSliceCandidates(mut cref: Arc<ComponentRef::NFComponentRef>, mut acc: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut name: Arc<ComponentRef::NFComponentRef>) -> Result<Arc<ComponentRef::NFComponentRef>> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = cref;
    let mut checkCref: Arc<ComponentRef::NFComponentRef> = ComponentRef::stripSubscriptsAll(cref.clone());
    if ComponentRef::isEqual(name.clone(), checkCref.clone())? || ComponentRef::isEqualRecordChild(name.clone(), checkCref.clone())? {
        UnorderedSet::add(cref.clone(), acc.clone())?;
    }
    Ok(cref)
}

pub fn getDependentCref(mut cref: Arc<ComponentRef::NFComponentRef>, mut acc: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut pseudo: bool) -> Result<Arc<ComponentRef::NFComponentRef>> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = cref;
    let mut checkCref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut childCref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut record_children: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    checkCref = if (pseudo.clone()) {ComponentRef::stripSubscriptsAll(cref.clone())} else {cref.clone()};
    record_children = BVariable::getRecordChildren(BVariable::getVarPointer(checkCref.clone(), metamodelica::sourceInfo!())?);
    if record_children.clone().is_empty() {
        if UnorderedMap::contains(checkCref.clone(), map.clone()) {
            UnorderedSet::add(cref.clone(), acc.clone())?;
        }
    } else {
        for mut child in &*record_children.clone() {
            let mut child = child.clone();
            childCref = BVariable::getVarName(child.clone());
            if UnorderedMap::contains(childCref.clone(), map.clone()) {
                UnorderedSet::add(childCref.clone(), acc.clone())?;
            }
        }
    }
    Ok(cref)
}

pub fn getDependentCrefCausalized(mut cref: Arc<ComponentRef::NFComponentRef>, mut acc: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<ComponentRef::NFComponentRef>> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = cref;
    let mut checkCref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut childCref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut record_children: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    checkCref = ComponentRef::stripSubscriptsAll(cref.clone());
    record_children = BVariable::getRecordChildren(BVariable::getVarPointer(checkCref.clone(), metamodelica::sourceInfo!())?);
    if record_children.clone().is_empty() {
        if UnorderedSet::contains(checkCref.clone(), set.clone())? {
            UnorderedSet::add(cref.clone(), acc.clone())?;
        }
    } else {
        for mut child in &*record_children.clone() {
            let mut child = child.clone();
            childCref = BVariable::getVarName(child.clone());
            if UnorderedSet::contains(childCref.clone(), set.clone())? {
                UnorderedSet::add(childCref.clone(), acc.clone())?;
            }
        }
    }
    Ok(cref)
}

pub fn getUnsolvableExpCrefs(mut exp: Arc<Expression::NFExpression>, mut acc: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut pseudo: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::RANGE { .. } => Expression::mapShallow(exp.clone(), Arc::new({ let __pe_b1 = Arc::new({ let __pe_b2 = map.clone(); let __pe_b3 = pseudo.clone(); move |__pe_a0, __pe_a1| getDependentCref(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone()) }); let __pe_b2 = acc.clone(); move |__pe_a0| filterExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?,
        Deref @ Expression::LBINARY { .. } => Expression::mapShallow(exp.clone(), Arc::new({ let __pe_b1 = Arc::new({ let __pe_b2 = map.clone(); let __pe_b3 = pseudo.clone(); move |__pe_a0, __pe_a1| getDependentCref(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone()) }); let __pe_b2 = acc.clone(); move |__pe_a0| filterExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?,
        Deref @ Expression::RELATION { .. } => Expression::mapShallow(exp.clone(), Arc::new({ let __pe_b1 = Arc::new({ let __pe_b2 = map.clone(); let __pe_b3 = pseudo.clone(); move |__pe_a0, __pe_a1| getDependentCref(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone()) }); let __pe_b2 = acc.clone(); move |__pe_a0| filterExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?,
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn getDependentCrefIndicesPseudoScalar(mut dependencies: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut mapping: Arc<Mapping::Mapping>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut indices: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut scalarized_dependencies: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
        for mut dep in (dependencies.clone()).into_iter().cloned() {
            let __x = ComponentRef::scalarizeAll(dep.clone(), true)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    let mut stripped: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut var_arr_idx: i32 = 0;
    let mut var_start: i32 = 0;
    let mut var_scal_idx: i32 = 0;
    let mut sizes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut int_subs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut cref in &*scalarized_dependencies.clone() {
        let mut cref = cref.clone();
        stripped = ComponentRef::stripSubscriptsAll(cref.clone());
        var_arr_idx = UnorderedMap::getSafe(stripped.clone(), map.clone(), metamodelica::sourceInfo!())?;
        (var_start, _) = mapping.var_AtS.borrow()[(var_arr_idx.clone()-1) as usize].clone();
        sizes = ComponentRef::sizes(stripped.clone(), false, false, metamodelica::nil())?;
        int_subs = ComponentRef::subscriptsToInteger(cref.clone())?;
        var_scal_idx = locationToIndex(sizes.clone(), int_subs.clone(), var_start.clone())?;
        indices = cons(var_scal_idx.clone(), indices.clone());
    }
    if !(indices.clone().is_empty()) {
        indices = List::sort(List::uniqueIntN(indices.clone(), {
        let mut __acc: Option<i32> = None;
        for mut i in (indices.clone()).into_iter().cloned() {
            let __x = i.clone();
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    })?, (std::sync::Arc::new(fnptr!(intLt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    }
    Ok(indices)
}

pub fn getDependentCrefIndicesPseudoFull(mut dependencies: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut mapping: Arc<Mapping::Mapping>, mut eqn_arr_idx: i32) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<metamodelica::Array<i32>>)> {
    let mut indices: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mode_to_var: metamodelica::Array<metamodelica::Array<i32>>;
    let mut scalarized_dependencies: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
        for mut dep in (dependencies.clone()).into_iter().cloned() {
            let __x = ComponentRef::scalarizeAll(dep.clone(), true)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    let mut stripped: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut eqn_start: i32 = 0;
    let mut eqn_size: i32 = 0;
    let mut var_arr_idx: i32 = 0;
    let mut var_scal_idx: i32 = 0;
    let mut mode: i32 = 1;
    let mut scal_lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut idx: i32 = 0;
    let mut mode_to_var_row: metamodelica::Array<i32>;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    (eqn_start, eqn_size) = mapping.eqn_AtS.borrow()[(eqn_arr_idx.clone()-1) as usize].clone();
    indices = arrayCreate(eqn_size.clone(), metamodelica::nil());
    mode_to_var = arrayCreate(eqn_size.clone(), arrayCreate(0, 0));
    for mut i in 1..=eqn_size.clone() {
        {
            let __cell0 = arrayCreate((scalarized_dependencies.clone().len() as i32), -1);
            mode_to_var.clone().borrow_mut()[(i.clone()-1) as usize] = __cell0;
        }
    }
    for mut cref in &*scalarized_dependencies.clone() {
        let mut cref = cref.clone();
        stripped = ComponentRef::stripSubscriptsAll(cref.clone());
        var_arr_idx = UnorderedMap::getSafe(stripped.clone(), map.clone(), metamodelica::sourceInfo!())?;
        subs = ComponentRef::subscriptsAllWithWholeFlat(cref.clone());
        ty = ComponentRef::getSubscriptedType(stripped.clone(), true)?;
        dims = Type::arrayDims(ty.clone());
        scal_lst = Mapping::getVarScalIndices(var_arr_idx.clone(), mapping.clone(), subs.clone(), dims.clone(), true)?;
        if intMod(eqn_size.clone(), (scal_lst.clone().len() as i32)) != 0 {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSlice.getDependentCrefIndicesPseudoFull")); __mm_s.push_str(&*literal!(" failed because flattened indices ")); __mm_s.push_str(&*intString((scal_lst.clone().len() as i32))); __mm_s.push_str(&*literal!(" could not be repeated to fit equation size ")); __mm_s.push_str(&*intString(eqn_size.clone())); __mm_s.push_str(&*literal!(". lst: ")); __mm_s.push_str(&*List::toString(scal_lst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail");
        } else {
            scal_lst = List::repeat(scal_lst.clone(), intDiv(eqn_size.clone(), (scal_lst.clone().len() as i32)));
        }
        idx = 1;
        for mut var_scal_idx in &*scal_lst.clone().reverse() {
            let mut var_scal_idx = var_scal_idx.clone();
            mode_to_var_row = mode_to_var.borrow()[(idx.clone()-1) as usize].clone();
            {let _arr = mode_to_var_row.clone(); _arr.borrow_mut()[(mode.clone()-1) as usize] = var_scal_idx.clone(); _arr};
            {let _arr = mode_to_var.clone(); _arr.borrow_mut()[(idx.clone()-1) as usize] = mode_to_var_row.clone(); _arr};
            {
                let __cell1 = cons(var_scal_idx.clone(), indices.borrow()[(idx.clone()-1) as usize].clone());
                indices.clone().borrow_mut()[(idx.clone()-1) as usize] = __cell1;
            }
            idx = idx.clone() + 1;
        }
        mode = mode.clone() + 1;
    }
    let __range2 = 1..=(indices.clone().borrow().len() as i32);
    for mut i in __range2 {
        {
            let __cell3 = List::sort(UnorderedSet::unique_list(indices.borrow()[(i.clone()-1) as usize].clone(), std::sync::Arc::new(fnptr!(Util::id, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>)), (std::sync::Arc::new(fnptr!(intLt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            indices.clone().borrow_mut()[(i.clone()-1) as usize] = __cell3;
        }
    }
    Ok((indices, mode_to_var))
}

pub fn getDependentCrefIndicesPseudoFor(mut dependencies: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut mapping: Arc<Mapping::Mapping>, mut eqn_arr_idx: i32, mut iter: Arc<Iterator::Iterator>) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<metamodelica::Array<i32>>)> {
    let mut indices: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mode_to_var: metamodelica::Array<metamodelica::Array<i32>>;
    let mut names: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut maps: Arc<metamodelica::List<Option<Arc<Iterator::Iterator>>>> = metamodelica::nil();
    let mut frames: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)>> = metamodelica::nil();
    let mut eqn_size: i32 = 0;
    let mut iter_size: i32 = 0;
    let mut body_size: i32 = 0;
    let mut mode: i32 = 1;
    let mut func: updateDependencies;
    iter_size = Iterator::size(iter.clone(), false);
    (names, ranges, maps) = Iterator::getFrames(iter.clone())?;
    frames = List::zip3(names.clone(), ranges.clone(), maps.clone());
    (_, eqn_size) = mapping.eqn_AtS.borrow()[(eqn_arr_idx.clone()-1) as usize].clone();
    indices = arrayCreate(eqn_size.clone(), metamodelica::nil());
    mode_to_var = arrayCreate(eqn_size.clone(), arrayCreate(0, 0));
    if intMod(eqn_size.clone(), iter_size.clone()) == 0 {
        body_size = intDiv(eqn_size.clone(), iter_size.clone());
    } else {
        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSlice.getDependentCrefIndicesPseudoFor")); __mm_s.push_str(&*literal!(" failed because the equation size ")); __mm_s.push_str(&*intString(eqn_size.clone())); __mm_s.push_str(&*literal!(" could not be divided by the iterator size ")); __mm_s.push_str(&*intString(iter_size.clone())); __mm_s.push_str(&*literal!(" without rest.")); ArcStr::from(__mm_s) }).clone()])?;
    }
    for mut i in 1..=eqn_size.clone() {
        {
            let __cell0 = arrayCreate((dependencies.clone().len() as i32), -1);
            mode_to_var.clone().borrow_mut()[(i.clone()-1) as usize] = __cell0;
        }
    }
    for mut dep in &*dependencies.clone() {
        let mut dep = dep.clone();
        func = Arc::new({ let __pe_b3 = mode.clone(); let __pe_b4 = mode_to_var.clone(); let __pe_b5 = indices.clone(); move |__pe_a0, __pe_a1, __pe_a2| updateDependenciesInteger(__pe_a0, __pe_a1, __pe_a2, __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone()) });
        fillDependencyArray(dep.clone(), body_size.clone(), frames.clone(), mapping.clone(), map.clone(), func.clone(), 0, true)?;
        mode = mode.clone() + 1;
    }
    let __range1 = 1..=(indices.clone().borrow().len() as i32);
    for mut i in __range1 {
        {
            let __cell2 = List::sort(UnorderedSet::unique_list(indices.borrow()[(i.clone()-1) as usize].clone(), std::sync::Arc::new(fnptr!(Util::id, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>)), (std::sync::Arc::new(fnptr!(intLt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            indices.clone().borrow_mut()[(i.clone()-1) as usize] = __cell2;
        }
    }
    Ok((indices, mode_to_var))
}

pub fn getDependentCrefsPseudoForCausalized(mut row_cref: Arc<ComponentRef::NFComponentRef>, mut dependencies: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut var_rep: Arc<VariablePointers::VariablePointers>, mut eqn_rep: Arc<VariablePointers::VariablePointers>, mut var_rep_mapping: Arc<Mapping::Mapping>, mut eqn_rep_mapping: Arc<Mapping::Mapping>, mut iter: Arc<Iterator::Iterator>, mut eqn_size: i32, mut slice: Arc<metamodelica::List<i32>>, mut implicit: bool) -> Result<Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)>>> {
    let mut tpl_lst: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)>> = metamodelica::nil();
    let mut names: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut maps: Arc<metamodelica::List<Option<Arc<Iterator::Iterator>>>> = metamodelica::nil();
    let mut frames: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)>> = metamodelica::nil();
    let mut iter_size: i32 = 0;
    let mut body_size: i32 = 0;
    let mut var_arr_idx: i32 = 0;
    let mut row_crefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut row_scal_lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut accum_row_lst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut accum_dep_arr: metamodelica::Array<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>;
    let mut accum_dep_lst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
    let mut func_var: updateDependencies;
    let mut func_eqn: updateDependencies;
    let mut final_dep: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    accum_dep_arr = arrayCreate(eqn_size.clone(), metamodelica::nil());
    iter_size = Iterator::size(iter.clone(), false);
    (names, ranges, maps) = Iterator::getFrames(iter.clone())?;
    frames = List::zip3(names.clone(), ranges.clone(), maps.clone());
    if intMod(eqn_size.clone(), iter_size.clone()) == 0 {
        body_size = intDiv(eqn_size.clone(), iter_size.clone());
    } else {
        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSlice.getDependentCrefsPseudoForCausalized")); __mm_s.push_str(&*literal!(" failed because the equation size ")); __mm_s.push_str(&*intString(eqn_size.clone())); __mm_s.push_str(&*literal!(" could not be divided by the iterator size ")); __mm_s.push_str(&*intString(iter_size.clone())); __mm_s.push_str(&*literal!(" without rest.")); ArcStr::from(__mm_s) }).clone()])?;
    }
    if implicit.clone() {
        row_crefs = ComponentRef::scalarizeSlice(row_cref.clone(), slice.clone(), false)?;
    } else {
        for mut cref in &*ComponentRef::scalarizeAll(row_cref.clone(), false)? {
            let mut cref = cref.clone();
            row_scal_lst = getCrefInFrameIndices(cref.clone(), frames.clone(), eqn_rep_mapping.clone(), eqn_rep.map.clone(), false)?;
            accum_row_lst = cons(row_scal_lst.clone(), accum_row_lst.clone());
        }
        row_scal_lst = List::flatten(accum_row_lst.clone());
        row_scal_lst = if (slice.clone().is_empty() || (slice.clone().len() as i32) > (row_scal_lst.clone().len() as i32)) {row_scal_lst.clone()} else {List::getAtIndexLst(row_scal_lst.clone(), slice.clone(), true)};
        row_crefs = {
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut i in (row_scal_lst.clone()).into_iter().cloned() {
            let __x = BVariable::VariablePointers::varSlice(eqn_rep.clone(), i.clone(), eqn_rep_mapping.var_StA.borrow()[(i.clone()-1) as usize].clone(), eqn_rep_mapping.clone(), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    }
    func_var = Arc::new({ let __pe_b3 = accum_dep_arr.clone(); let __pe_b4 = var_rep.clone(); let __pe_b5 = var_rep_mapping.clone(); let __pe_b6 = false; move |__pe_a0, __pe_a1, __pe_a2| updateDependenciesCref(__pe_a0, __pe_a1, __pe_a2, __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone()) });
    func_eqn = Arc::new({ let __pe_b3 = accum_dep_arr.clone(); let __pe_b4 = eqn_rep.clone(); let __pe_b5 = eqn_rep_mapping.clone(); let __pe_b6 = false; move |__pe_a0, __pe_a1, __pe_a2| updateDependenciesCref(__pe_a0, __pe_a1, __pe_a2, __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone()) });
    for mut dep in &*dependencies.clone() {
        let mut dep = dep.clone();
        if UnorderedMap::contains(dep.clone(), var_rep.map.clone()) {
            (final_dep, var_arr_idx) = getVarArrIdx(dep.clone(), var_rep_mapping.clone(), var_rep.map.clone())?;
            fillDependencyArray(final_dep.clone(), body_size.clone(), frames.clone(), var_rep_mapping.clone(), var_rep.map.clone(), func_var.clone(), var_arr_idx.clone(), false)?;
        } else if UnorderedMap::contains(dep.clone(), eqn_rep.map.clone()) {
            (final_dep, var_arr_idx) = getVarArrIdx(dep.clone(), eqn_rep_mapping.clone(), eqn_rep.map.clone())?;
            fillDependencyArray(final_dep.clone(), body_size.clone(), frames.clone(), eqn_rep_mapping.clone(), eqn_rep.map.clone(), func_eqn.clone(), var_arr_idx.clone(), false)?;
        }
    }
    accum_dep_lst = Arc::new(accum_dep_arr.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()).reverse();
    accum_dep_lst = if (slice.clone().is_empty() || (slice.clone().len() as i32) > (accum_dep_lst.clone().len() as i32)) {accum_dep_lst.clone()} else {List::getAtIndexLst(accum_dep_lst.clone(), slice.clone(), true)};
    tpl_lst = List::zip(row_crefs.clone(), accum_dep_lst.clone());
    Ok(tpl_lst)
}

pub fn fillDependencyArray(mut dep: Arc<ComponentRef::NFComponentRef>, mut body_size: i32, mut frames: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)>>, mut mapping: Arc<Mapping::Mapping>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut func: Arc<dyn ::std::ops::Fn(i32, i32, i32) -> Result<i32> + 'static>, mut var_arr_idx: i32, mut resize: bool) -> Result<()> {
    let mut scal_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut scal_length: i32 = 0;
    let mut body_repeat: i32 = 0;
    let mut element_repeat: i32 = 0;
    let mut eqn_idx: i32 = 0;
    let mut scal_lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut scal_tpl_lst: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    for mut scal_cref in &*ComponentRef::scalarizeAll(dep.clone(), true)? {
        let mut scal_cref = scal_cref.clone();
        scal_lst = getCrefInFrameIndices(scal_cref.clone(), frames.clone(), mapping.clone(), map.clone(), resize.clone())?;
        scal_tpl_lst = cons((scal_cref.clone(), scal_lst.clone()), scal_tpl_lst.clone());
    }
    scal_length = (scal_tpl_lst.clone().len() as i32);
    if intMod(scal_length.clone(), body_size.clone()) == 0 {
        body_repeat = intDiv(scal_length.clone(), body_size.clone());
        element_repeat = 1;
    } else if intMod(body_size.clone(), scal_length.clone()) == 0 {
        body_repeat = 1;
        element_repeat = intDiv(body_size.clone(), scal_length.clone());
    } else {
        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSlice.fillDependencyArray")); __mm_s.push_str(&*literal!(" failed because number of flattened indices ")); __mm_s.push_str(&*intString(scal_length.clone())); __mm_s.push_str(&*literal!(" for dependency ")); __mm_s.push_str(&*ComponentRef::toString(dep.clone())?); __mm_s.push_str(&*literal!(" could not be divided by or repeated to fit the body size ")); __mm_s.push_str(&*intString(body_size.clone())); __mm_s.push_str(&*literal!(" without rest.")); ArcStr::from(__mm_s) }).clone()])?;
        bail!("fail");
    }
    eqn_idx = 1;
    for mut i in 1..=element_repeat.clone() {
        for mut tpl in &*scal_tpl_lst.clone().reverse() {
            let mut tpl = tpl.clone();
            (scal_cref, scal_lst) = tpl.clone();
            scal_lst = scal_lst.clone().reverse();
            if body_repeat.clone() > 1 {
                eqn_idx = 1;
            }
            for mut var_idx in &*scal_lst.clone() {
                let mut var_idx = var_idx.clone();
                if var_idx.clone() > 0 {
                    eqn_idx = func(eqn_idx.clone(), var_idx.clone(), var_arr_idx.clone())?;
                }
            }
        }
    }
    Ok(())
}

pub type updateDependencies = std::sync::Arc<dyn ::std::ops::Fn(i32, i32, i32) -> Result<i32> + 'static>;

pub fn updateDependenciesCref(mut eqn_idx: i32, mut var_idx: i32, mut var_arr_idx: i32, mut accum_dep_arr: metamodelica::Array<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>, mut vars: Arc<VariablePointers::VariablePointers>, mut mapping: Arc<Mapping::Mapping>, mut resize: bool) -> Result<i32> {
    let mut eqn_idx: i32 = eqn_idx;
    {let _arr = accum_dep_arr.clone(); let _val = cons(BVariable::VariablePointers::varSlice(vars.clone(), var_idx.clone(), var_arr_idx.clone(), mapping.clone(), resize.clone())?, accum_dep_arr.borrow()[(eqn_idx.clone()-1) as usize].clone()); _arr.borrow_mut()[(eqn_idx.clone()-1) as usize] = _val; _arr};
    eqn_idx = eqn_idx.clone() + 1;
    Ok(eqn_idx)
}

pub fn updateDependenciesInteger(mut eqn_idx: i32, mut var_idx: i32, mut var_arr_idx: i32, mut mode: i32, mut mode_to_var: metamodelica::Array<metamodelica::Array<i32>>, mut indices: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<i32> {
    let mut eqn_idx: i32 = eqn_idx;
    let mut mode_to_var_row: metamodelica::Array<i32>;
    mode_to_var_row = mode_to_var.borrow()[(eqn_idx.clone()-1) as usize].clone();
    {let _arr = mode_to_var_row.clone(); _arr.borrow_mut()[(mode.clone()-1) as usize] = var_idx.clone(); _arr};
    {let _arr = indices.clone(); let _val = cons(var_idx.clone(), indices.borrow()[(eqn_idx.clone()-1) as usize].clone()); _arr.borrow_mut()[(eqn_idx.clone()-1) as usize] = _val; _arr};
    eqn_idx = eqn_idx.clone() + 1;
    Ok(eqn_idx)
}

pub fn getDependentCrefsPseudoArrayCausalized(mut row_cref: Arc<ComponentRef::NFComponentRef>, mut dependencies: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut slice: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)>>> {
    fn fixSingleDep(mut row_size: i32, mut single_dep: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut full_deps: Pointer::Pointer<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>) -> Result<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>> {
        let mut single_dep: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = single_dep;
        let mut dep_size: i32 = (single_dep.clone().len() as i32);
        if row_size.clone() > dep_size.clone() {
            if intMod(row_size.clone(), dep_size.clone()) == 0 {
                single_dep = List::repeat(single_dep.clone(), intDiv(row_size.clone(), dep_size.clone()));
            } else {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSlice.getDependentCrefsPseudoArrayCausalized.fixSingleDep")); __mm_s.push_str(&*literal!(" failed because dependencies of size ")); __mm_s.push_str(&*intString(dep_size.clone())); __mm_s.push_str(&*literal!(" could not be repeated to fit row size ")); __mm_s.push_str(&*intString(row_size.clone())); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
                bail!("fail");
            }
        } else if row_size.clone() < dep_size.clone() {
            Pointer::update(full_deps.clone(), listAppend(single_dep.clone(), Pointer::access(full_deps.clone())));
            single_dep = metamodelica::nil();
        }
        Ok(single_dep)
    }

    let mut tpl_lst: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)>> = metamodelica::nil();
    let mut row_cref_scal: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut dependencies_resizable: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut row_size: i32 = 0;
    let mut dependencies_scal: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
    let mut full_deps: Pointer::Pointer<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>> = Pointer::create(metamodelica::nil());
    row_cref_scal = ComponentRef::scalarizeSlice(row_cref.clone(), slice.clone(), false)?;
    row_size = (row_cref_scal.clone().len() as i32);
    dependencies_resizable = {
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut dep in (dependencies.clone()).into_iter().cloned() {
            let __x = ComponentRef::simplifySubscripts(ComponentRef::mapExp(dep.clone(), (std::sync::Arc::new(Expression::replaceResizableParameterWithOriginal) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?, false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    dependencies_scal = {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
        for mut dep in (dependencies_resizable.clone()).into_iter().cloned() {
            let __x = ComponentRef::scalarizeSlice(dep.clone(), slice.clone(), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    if !(dependencies_scal.clone().is_empty()) {
        dependencies_scal = {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
        for mut d in (dependencies_scal.clone()).into_iter().cloned() {
            let __x = fixSingleDep(row_size.clone(), d.clone(), full_deps.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        dependencies_scal = {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
        for mut d in (dependencies_scal.clone()).into_iter().cloned() {
            if !(!(d.clone().is_empty())) { continue; }
            let __x = d.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        if dependencies_scal.clone().is_empty() {
            dependencies_scal = List::fill(Pointer::access(full_deps.clone()), row_size.clone());
        } else {
            dependencies_scal = {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
        for mut d in (List::transposeList(dependencies_scal.clone())?).into_iter().cloned() {
            let __x = listAppend(Pointer::access(full_deps.clone()), d.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        }
        tpl_lst = List::zip(row_cref_scal.clone(), dependencies_scal.clone());
    } else {
        tpl_lst = {
        let mut __acc: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<_>>)>> = metamodelica::nil();
        for mut cref in (row_cref_scal.clone()).into_iter().cloned() {
            let __x = (cref.clone(), metamodelica::nil());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    }
    Ok(tpl_lst)
}

pub fn locationToIndex(mut sizes: Arc<metamodelica::List<i32>>, mut values: Arc<metamodelica::List<i32>>, mut index: i32) -> Result<i32> {
    let mut index: i32 = index;
    let mut factor: i32 = 1;
    let mut val: i32 = 0;
    let mut siz: i32 = 0;
    let mut val_trav: Arc<metamodelica::List<i32>> = values.clone();
    let mut siz_trav: Arc<metamodelica::List<i32>> = sizes.clone();
    while !(val_trav.clone().is_empty() || siz_trav.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(val_trav.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        val = __pa0.clone();
        val_trav = __pa1.clone();
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(siz_trav.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        siz = __pa2.clone();
        siz_trav = __pa3.clone();
        if val.clone() > siz.clone() {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSlice.locationToIndex")); __mm_s.push_str(&*literal!(" failed because value of ")); __mm_s.push_str(&*intString(val.clone())); __mm_s.push_str(&*literal!(" is too large for size ")); __mm_s.push_str(&*intString(siz.clone())); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail");
        }
        index = index.clone() + (val.clone() - 1) * factor.clone();
        factor = factor.clone() * siz.clone();
    }
    Ok(index)
}

pub fn indexToLocation(mut index: i32, mut sizes: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<i32>> {
    let mut vals: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut iterator: i32 = index.clone();
    let mut divisor: i32 = {
        let mut __acc: i32 = 1;
        for mut s in (sizes.clone()).into_iter().cloned() {
            let __x = s.clone();
            __acc *= __x;
        }
        __acc
    };
    for mut size in &*sizes.clone() {
        let mut size = size.clone();
        divisor = intDiv(divisor.clone(), size.clone());
        vals = cons(intDiv(iterator.clone(), divisor.clone()), vals.clone());
        iterator = intMod(iterator.clone(), divisor.clone());
    }
    vals
}

pub fn transposeLocations(mut locations: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut out_size: i32) -> Arc<metamodelica::List<metamodelica::Array<i32>>> {
    let mut locations_transposed: Arc<metamodelica::List<metamodelica::Array<i32>>> = metamodelica::nil();
    let mut lT_tmp: metamodelica::Array<Arc<metamodelica::List<i32>>> = arrayCreate(out_size.clone(), metamodelica::nil());
    let mut lT_tmp2: metamodelica::Array<metamodelica::Array<i32>> = arrayCreate(out_size.clone(), arrayCreate(0, 0));
    let mut idx: i32 = 0;
    for mut location in &*locations.clone() {
        let mut location = location.clone();
        idx = 1;
        for mut i in &*location.clone() {
            let mut i = i.clone();
            {
                let __cell0 = cons(i.clone(), lT_tmp.borrow()[(idx.clone()-1) as usize].clone());
                lT_tmp.clone().borrow_mut()[(idx.clone()-1) as usize] = __cell0;
            }
            idx = idx.clone() + 1;
        }
    }
    let __range1 = 1..=(lT_tmp.clone().borrow().len() as i32);
    for mut j in __range1 {
        {
            let __cell2 = metamodelica::arrayFromVec(lT_tmp.borrow()[(j.clone()-1) as usize].clone().reverse().into_iter().cloned().collect());
            lT_tmp2.clone().borrow_mut()[(j.clone()-1) as usize] = __cell2;
        }
    }
    locations_transposed = Arc::new(lT_tmp2.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()).reverse();
    locations_transposed
}

pub fn orderTransposedFrameLocations(mut frame_locations_transposed: Arc<metamodelica::List<(metamodelica::Array<i32>, (Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>))>>) -> Result<(Arc<metamodelica::List<(metamodelica::Array<i32>, (Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>))>>, Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>, FrameOrderingStatus)> {
    let mut frame_locations_transposed: Arc<metamodelica::List<(metamodelica::Array<i32>, (Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>))>> = frame_locations_transposed;
    let mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>> = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
    let mut status: FrameOrderingStatus = FrameOrderingStatus::UNCHANGED;
    let mut frame_inertia_lst: Arc<metamodelica::List<(i32, (metamodelica::Array<i32>, (Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)))>> = metamodelica::nil();
    frame_inertia_lst = {
        let mut __acc: Arc<metamodelica::List<(i32, (metamodelica::Array<i32>, (Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)))>> = metamodelica::nil();
        for mut frame in (frame_locations_transposed.clone()).into_iter().cloned() {
            let __x = (frameLocationInertia(frame.clone()), frame.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    frame_inertia_lst = List::sort(frame_inertia_lst.clone(), std::sync::Arc::new(fnptr!(Util::compareTupleIntGt, _, _)))?;
    (frame_inertia_lst, status) = resolveEqualInertia(frame_inertia_lst.clone(), replacements.clone())?;
    frame_locations_transposed = {
        let mut __acc: Arc<metamodelica::List<(metamodelica::Array<i32>, (Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>))>> = metamodelica::nil();
        for mut frame_inertia in (frame_inertia_lst.clone()).into_iter().cloned() {
            let __x = Util::tuple22(frame_inertia.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok((frame_locations_transposed, replacements, status))
}

fn frameLocationInertia(mut frameLocation: (metamodelica::Array<i32>, (Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>))) -> i32 {
    let mut inertia: i32 = 1;
    let mut dim: metamodelica::Array<i32>;
    dim = Util::tuple21(frameLocation.clone());
    while inertia.clone() < (dim.clone().borrow().len() as i32) && dim.borrow()[(inertia.clone()-1) as usize].clone() == dim.borrow()[(inertia.clone() + 1-1) as usize].clone() {
        inertia = inertia.clone() + 1;
    }
    inertia
}

fn resolveEqualInertia(mut frame_inertia_lst: Arc<metamodelica::List<(i32, (metamodelica::Array<i32>, (Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)))>>, mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>) -> Result<(Arc<metamodelica::List<(i32, (metamodelica::Array<i32>, (Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)))>>, FrameOrderingStatus)> {
    let mut resolved: Arc<metamodelica::List<(i32, (metamodelica::Array<i32>, (Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)))>> = metamodelica::nil();
    let mut status: FrameOrderingStatus = FrameOrderingStatus::UNCHANGED.clone();
    let mut tpl1: (i32, (metamodelica::Array<i32>, (Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)));
    let mut tpl2: (i32, (metamodelica::Array<i32>, (Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)));
    let mut rest: Arc<metamodelica::List<(i32, (metamodelica::Array<i32>, (Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)))>> = metamodelica::nil();
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(frame_inertia_lst.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    tpl1 = __pa0.clone();
    rest = __pa1.clone();
    while !(rest.clone().is_empty()) {
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        tpl2 = __pa2.clone();
        rest = __pa3.clone();
        tpl1 = (::match_deref::match_deref! { match &((tpl1.clone(), tpl2.clone())) {
        ((inertia1, (loc1, (name1, _, _))), (inertia2, (loc2, (name2, _, _)))) if (inertia1.clone() == inertia2.clone()) => {
            let mut m: i32 = 0;
            let mut b: i32 = 0;
            let mut addOp: Arc<Operator::NFOperator> = Arc::new(<Operator::NFOperator as ::std::default::Default>::default());
            let mut mulOp: Arc<Operator::NFOperator> = Arc::new(<Operator::NFOperator as ::std::default::Default>::default());
            let mut linMap: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            addOp = Operator::fromClassification((Operator::MathClassification::ADDITION.clone(), Operator::SizeClassification::SCALAR.clone()), Arc::new(openmodelica_nf_frontend::NFType::INTEGER))?;
            mulOp = Operator::fromClassification((Operator::MathClassification::MULTIPLICATION.clone(), Operator::SizeClassification::SCALAR.clone()), Arc::new(openmodelica_nf_frontend::NFType::INTEGER))?;
            if (loc1.clone().borrow().len() as i32) != (loc2.clone().borrow().len() as i32) {
                status = FrameOrderingStatus::FAILURE.clone();
                return Ok((resolved, status));
            } else if (loc1.clone().borrow().len() as i32) == 1 {
                b = loc2.borrow()[(1-1) as usize].clone() - loc1.borrow()[(1-1) as usize].clone();
                linMap = Expression::fromCref(name1.clone(), false)?;
                if b.clone() != 0 {
                    linMap = Arc::new(Expression::NFExpression::MULTARY { arguments: list![Arc::new(Expression::NFExpression::INTEGER { value: b.clone() }), linMap.clone()], inv_arguments: metamodelica::nil(), operator: addOp.clone() });
                }
                UnorderedMap::add(name2.clone(), linMap.clone(), replacements.clone())?;
                status = FrameOrderingStatus::CHANGED.clone();
            } else {
                m = ((metamodelica::OrderedFloat(((loc2.borrow()[(1-1) as usize].clone() - loc2.borrow()[(1 + inertia2.clone()-1) as usize].clone()) / (loc1.borrow()[(1-1) as usize].clone() - loc1.borrow()[(1 + inertia1.clone()-1) as usize].clone())) as f64)).0 as i32);
                b = loc2.borrow()[(1-1) as usize].clone() - m.clone() * loc1.borrow()[(1-1) as usize].clone();
                let __range0 = 2..=(loc1.clone().borrow().len() as i32);
                for mut i in __range0 {
                    if loc2.borrow()[(i.clone()-1) as usize].clone() != m.clone() * loc1.borrow()[(i.clone()-1) as usize].clone() + b.clone() {
                        status = FrameOrderingStatus::FAILURE.clone();
                        return Ok((resolved, status));
                    }
                }
                linMap = Expression::fromCref(name1.clone(), false)?;
                if m.clone() != 1 {
                    linMap = Arc::new(Expression::NFExpression::MULTARY { arguments: list![Arc::new(Expression::NFExpression::INTEGER { value: m.clone() }), linMap.clone()], inv_arguments: metamodelica::nil(), operator: mulOp.clone() });
                }
                if b.clone() != 0 {
                    linMap = Arc::new(Expression::NFExpression::MULTARY { arguments: list![Arc::new(Expression::NFExpression::INTEGER { value: b.clone() }), linMap.clone()], inv_arguments: metamodelica::nil(), operator: addOp.clone() });
                }
                UnorderedMap::add(name2.clone(), linMap.clone(), replacements.clone())?;
                status = FrameOrderingStatus::CHANGED.clone();
            }
            tpl1.clone()
        },
        _ => {
            resolved = cons(tpl1.clone(), resolved.clone());
            tpl2.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    resolved = cons(tpl1.clone(), resolved.clone()).reverse();
    Ok((resolved, status))
}

pub fn recollectRangesHeuristic(mut frame_locations_transposed: Arc<metamodelica::List<(metamodelica::Array<i32>, (Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>))>>) -> Result<(Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)>>, Option<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>>, RecollectStatus)> {
    let mut frames: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)>> = metamodelica::nil();
    let mut removed_diagonal: Option<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>> = None;
    let mut status: RecollectStatus = RecollectStatus::SUCCESS;
    let mut dim: metamodelica::Array<i32>;
    let mut frame: (Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>);
    let mut check_shift: i32 = 0;
    let mut pre_shift: i32 = 0;
    let mut shift: i32 = 1;
    let mut start: i32 = 0;
    let mut step: i32 = 0;
    let mut stop: i32 = 0;
    let mut max_size: i32 = 0;
    let mut new_step: i32 = 0;
    let mut new_stop: i32 = 0;
    let mut check_stop: i32 = 0;
    let mut fail_: bool = false;
    let mut rest: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut starts: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut stops: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut steps: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut shifts: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut failed: Arc<metamodelica::List<bool>> = metamodelica::nil();
    let mut min_dim: i32 = 0;
    let mut max_dim: i32 = 0;
    let mut diagonal: Arc<metamodelica::List<(metamodelica::Array<i32>, (Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>))>> = metamodelica::nil();
    let mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>;
    let mut fos: FrameOrderingStatus = FrameOrderingStatus::UNCHANGED;
    for mut tpl in &*frame_locations_transposed.clone() {
        let mut tpl = tpl.clone();
        fail_ = false;
        (dim, frame) = tpl.clone();
        pre_shift = shift.clone();
        max_size = (dim.clone().borrow().len() as i32);
        if max_size.clone() == 1 {
            frames = cons(applyNewFrameRange(frame.clone(), (dim.borrow()[(1-1) as usize].clone(), 1, dim.borrow()[(1-1) as usize].clone()))?, frames.clone());
            starts = cons(dim.borrow()[(1-1) as usize].clone(), starts.clone());
            steps = cons(0, steps.clone());
            stops = cons(dim.borrow()[(1-1) as usize].clone(), stops.clone());
            shifts = cons(shift.clone(), shifts.clone());
        } else {
            start = dim.borrow()[(1-1) as usize].clone();
            stop = dim.borrow()[(1 + shift.clone()-1) as usize].clone();
            step = stop.clone() - start.clone();
            if step.clone() == 0 {
                frames = cons(applyNewFrameRange(frame.clone(), (start.clone(), 1, stop.clone()))?, frames.clone());
                starts = cons(start.clone(), starts.clone());
                steps = cons(step.clone(), steps.clone());
                stops = cons(stop.clone(), stops.clone());
                shifts = cons(shift.clone(), shifts.clone());
            } else {
                new_step = step.clone();
                new_stop = stop.clone();
                while new_step.clone() == step.clone() && shift.clone() + pre_shift.clone() < max_size.clone() {
                    stop = new_stop.clone();
                    shift = shift.clone() + pre_shift.clone();
                    new_stop = dim.borrow()[(1 + shift.clone()-1) as usize].clone();
                    new_step = new_stop.clone() - stop.clone();
                }
                if new_step.clone() == step.clone() {
                    stop = new_stop.clone();
                    shift = shift.clone() + pre_shift.clone();
                } else {
                    check_shift = shift.clone();
                    while check_shift.clone() + pre_shift.clone() < max_size.clone() {
                        new_step = step.clone();
                        while new_step.clone() == step.clone() && check_shift.clone() + pre_shift.clone() < max_size.clone() {
                            check_stop = new_stop.clone();
                            check_shift = check_shift.clone() + pre_shift.clone();
                            new_stop = dim.borrow()[(1 + check_shift.clone()-1) as usize].clone();
                            new_step = new_stop.clone() - check_stop.clone();
                        }
                        if check_shift.clone() + pre_shift.clone() == max_size.clone() {
                            check_shift = check_shift.clone() + pre_shift.clone();
                        }
                        if !(intMod(check_shift.clone(), shift.clone()) == 0) {
                            fail_ = true;
                            break;
                        }
                    }
                }
                min_dim = {
        let mut __acc: Option<i32> = None;
        for mut d in (dim.clone()).borrow().iter() {
            let __x = d.clone();
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty min reduction"))?
    };
                max_dim = {
        let mut __acc: Option<i32> = None;
        for mut d in (dim.clone()).borrow().iter() {
            let __x = d.clone();
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    };
                if fail_.clone() {
                    if step.clone() > 0 {
                        frames = cons(applyNewFrameRange(frame.clone(), (min_dim.clone(), step.clone(), max_dim.clone()))?, frames.clone());
                    } else {
                        frames = cons(applyNewFrameRange(frame.clone(), (max_dim.clone(), step.clone(), min_dim.clone()))?, frames.clone());
                    }
                } else {
                    frames = cons(applyNewFrameRange(frame.clone(), (start.clone(), step.clone(), stop.clone()))?, frames.clone());
                }
                steps = cons(step.clone(), steps.clone());
                starts = if (step.clone() > 0) {cons(min_dim.clone(), starts.clone())} else {cons(max_dim.clone(), starts.clone())};
                stops = if (step.clone() > 0) {cons(max_dim.clone(), stops.clone())} else {cons(min_dim.clone(), stops.clone())};
                shifts = cons(shift.clone(), shifts.clone());
                failed = cons(fail_.clone(), failed.clone());
            }
        }
    }
    if List::fold(failed.clone(), (std::sync::Arc::new(fnptr!(boolOr, bool, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool, bool) -> Result<bool> + 'static>), false) {
        diagonal = reconstructDiagonal(frame_locations_transposed.clone(), starts.clone().reverse(), steps.clone().reverse(), stops.clone().reverse(), shifts.clone().reverse(), failed.clone().reverse())?;
        (diagonal, replacements, fos) = orderTransposedFrameLocations(diagonal.clone())?;
        if fos.clone() == FrameOrderingStatus::CHANGED.clone() {
            removed_diagonal = Some(replacements.clone());
            status = RecollectStatus::SUCCESS.clone();
        } else {
            status = RecollectStatus::FAILURE.clone();
        }
    } else {
        status = RecollectStatus::SUCCESS.clone();
    }
    Ok((frames, removed_diagonal, status))
}

pub fn reconstructDiagonal(mut frame_locations_transposed: Arc<metamodelica::List<(metamodelica::Array<i32>, (Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>))>>, mut starts: Arc<metamodelica::List<i32>>, mut steps: Arc<metamodelica::List<i32>>, mut stops: Arc<metamodelica::List<i32>>, mut shifts: Arc<metamodelica::List<i32>>, mut failed: Arc<metamodelica::List<bool>>) -> Result<Arc<metamodelica::List<(metamodelica::Array<i32>, (Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>))>>> {
    let mut diagonal: Arc<metamodelica::List<(metamodelica::Array<i32>, (Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>))>> = metamodelica::nil();
    let mut start: i32 = 0;
    let mut step: i32 = 0;
    let mut stop: i32 = 0;
    let mut pos: i32 = 0;
    let mut shift: i32 = 1;
    let mut fail_: bool = false;
    let mut start_rest: Arc<metamodelica::List<i32>> = starts.clone();
    let mut step_rest: Arc<metamodelica::List<i32>> = steps.clone();
    let mut stop_rest: Arc<metamodelica::List<i32>> = stops.clone();
    let mut shift_rest: Arc<metamodelica::List<i32>> = shifts.clone();
    let mut fail_rest: Arc<metamodelica::List<bool>> = failed.clone();
    let mut dim: metamodelica::Array<i32>;
    let mut missing_dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut frame: (Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>);
    for mut tpl in &*frame_locations_transposed.clone() {
        let mut tpl = tpl.clone();
        (dim, frame) = tpl.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(start_rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        start = __pa0.clone();
        start_rest = __pa1.clone();
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(step_rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        step = __pa2.clone();
        step_rest = __pa3.clone();
        let (__pa4, __pa5) = ::match_deref::match_deref! { match &(stop_rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
            _ => bail!("pattern mismatch"),
        } };
        stop = __pa4.clone();
        stop_rest = __pa5.clone();
        let (__pa6, __pa7) = ::match_deref::match_deref! { match &(fail_rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa6, tail: __pa7 } => (__pa6.clone(), __pa7.clone()),
            _ => bail!("pattern mismatch"),
        } };
        fail_ = __pa6.clone();
        fail_rest = __pa7.clone();
        missing_dims = metamodelica::nil();
        pos = start.clone();
        if fail_.clone() {
            let __range8 = ({let __s=1; let __e=(dim.clone().borrow().len() as i32); let __step=shift.clone(); if __step>0 {__s..=__e} else {__e..=__s}}).step_by((if shift.clone()>0 {shift.clone()} else {-(shift.clone())}) as usize);
            for mut i in __range8 {
                while dim.borrow()[(i.clone()-1) as usize].clone() != pos.clone() {
                    missing_dims = cons(pos.clone(), missing_dims.clone());
                    pos = pos.clone() + step.clone();
                    if sign(metamodelica::OrderedFloat((step.clone()) as f64)) * pos.clone() > sign(metamodelica::OrderedFloat((step.clone()) as f64)) * stop.clone() {
                        break;
                    }
                }
                if sign(metamodelica::OrderedFloat((step.clone()) as f64)) * (pos.clone() + step.clone()) > sign(metamodelica::OrderedFloat((step.clone()) as f64)) * stop.clone() {
                    pos = start.clone();
                } else {
                    pos = pos.clone() + step.clone();
                }
            }
            while sign(metamodelica::OrderedFloat((step.clone()) as f64)) * pos.clone() <= sign(metamodelica::OrderedFloat((step.clone()) as f64)) * stop.clone() {
                missing_dims = cons(pos.clone(), missing_dims.clone());
                pos = pos.clone() + step.clone();
            }
        } else {
            let __range9 = ({let __s=1; let __e=(dim.clone().borrow().len() as i32); let __step=shift.clone(); if __step>0 {__s..=__e} else {__e..=__s}}).step_by((if shift.clone()>0 {shift.clone()} else {-(shift.clone())}) as usize);
            for mut i in __range9 {
                missing_dims = cons(dim.borrow()[(i.clone()-1) as usize].clone(), missing_dims.clone());
            }
        }
        diagonal = cons((metamodelica::arrayFromVec(missing_dims.clone().reverse().into_iter().cloned().collect()), frame.clone()), diagonal.clone());
        let (__pa10, __pa11) = ::match_deref::match_deref! { match &(shift_rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa10, tail: __pa11 } => (__pa10.clone(), __pa11.clone()),
            _ => bail!("pattern mismatch"),
        } };
        shift = __pa10.clone();
        shift_rest = __pa11.clone();
    }
    diagonal = diagonal.clone().reverse();
    Ok(diagonal)
}

pub fn naiveSeparation(mut indices: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut index_clusters: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut i: i32 = 0;
    let mut rest: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut current: Arc<metamodelica::List<i32>> = metamodelica::nil();
    if !(indices.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(indices.clone().reverse()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        i = __pa0.clone();
        rest = __pa1.clone();
        current = list![i.clone()];
        for mut i2 in &*rest.clone() {
            let mut i2 = i2.clone();
            if i.clone() - i2.clone() == 1 {
                current = cons(i2.clone(), current.clone());
            } else {
                index_clusters = cons(current.clone(), index_clusters.clone());
                current = list![i2.clone()];
            }
            i = i2.clone();
        }
        index_clusters = cons(current.clone(), index_clusters.clone());
    }
    Ok(index_clusters)
}

pub fn upgradeRowFull(mut dependencies: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut mapping: Arc<Mapping::Mapping>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut indices: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut scalarized_dependencies: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
        for mut dep in (dependencies.clone()).into_iter().cloned() {
            let __x = ComponentRef::scalarizeAll(dep.clone(), true)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    let mut replaced: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut stripped: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut var_arr_idx: i32 = 0;
    let mut var_start: i32 = 0;
    let mut var_scal_idx: i32 = 0;
    let mut sizes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut int_subs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut cref in &*scalarized_dependencies.clone() {
        let mut cref = cref.clone();
        replaced = ComponentRef::mapExp(cref.clone(), (std::sync::Arc::new(Expression::replaceResizableParameter) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
        replaced = ComponentRef::simplifySubscripts(replaced.clone(), false)?;
        stripped = ComponentRef::stripSubscriptsAll(replaced.clone());
        var_arr_idx = UnorderedMap::getSafe(stripped.clone(), map.clone(), metamodelica::sourceInfo!())?;
        (var_start, _) = mapping.var_AtS.borrow()[(var_arr_idx.clone()-1) as usize].clone();
        sizes = ComponentRef::sizes(stripped.clone(), false, false, metamodelica::nil())?;
        int_subs = ComponentRef::subscriptsToInteger(replaced.clone())?;
        var_scal_idx = locationToIndex(sizes.clone(), int_subs.clone(), var_start.clone())?;
        indices = cons(var_scal_idx.clone(), indices.clone());
    }
    Ok(indices)
}

pub fn upgradeRow(mut eqn_name: Arc<ComponentRef::NFComponentRef>, mut eqn_arr_idx: i32, mut iter: Arc<Iterator::Iterator>, mut ty: Arc<Type::NFType>, mut dependencies: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut dep: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Dependency::Dependency>>>, mut rep: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut fullmap: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapping: Arc<Mapping::Mapping>, mut modes: Arc<UnorderedMap::UnorderedMap<(i32, i32), Arc<Mode::Mode>>>) -> Result<()> {
    for mut cref in &*dependencies.clone() {
        let mut cref = cref.clone();
        resolveDependency(cref.clone(), eqn_name.clone(), eqn_arr_idx.clone(), iter.clone(), ty.clone(), dep.clone(), rep.clone(), map.clone(), fullmap.clone(), m.clone(), mapping.clone(), modes.clone())?;
    }
    Ok(())
}

fn resolveSkipsLst(mut index: i32, mut ty: Arc<Type::NFType>, mut skips: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut cref: Arc<ComponentRef::NFComponentRef>, mut fullmap: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>) -> Result<Arc<metamodelica::List<(i32, Arc<Type::NFType>)>>> {
    let mut skip_lst: Arc<metamodelica::List<(i32, Arc<Type::NFType>)>> = metamodelica::nil();
    let mut combinations: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = List::combination(skips.clone());
    let mut sub_idx: i32 = 0;
    let mut sub_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    if combinations.clone().is_empty() {
        skip_lst = list![(index.clone(), ty.clone())];
    } else {
        for mut com in &*combinations.clone() {
            let mut com = com.clone();
            (sub_idx, sub_ty) = resolveSkips(index.clone(), ty.clone(), com.clone(), cref.clone(), fullmap.clone())?;
            skip_lst = cons((sub_idx.clone(), sub_ty.clone()), skip_lst.clone());
        }
    }
    Ok(skip_lst)
}

fn resolveSkips(mut index: i32, mut ty: Arc<Type::NFType>, mut skips: Arc<metamodelica::List<i32>>, mut cref: Arc<ComponentRef::NFComponentRef>, mut fullmap: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>) -> Result<(i32, Arc<Type::NFType>)> {
    let mut index: i32 = index;
    let mut ty: Arc<Type::NFType> = ty;
    (index, ty) = (::match_deref::match_deref! { match &((ty.clone(), skips.clone())) {
        (Deref @ Type::TUPLE { .. }, Deref @ metamodelica::List::Cons { head: 0, tail: _ }) => {
            (index.clone(), ty.clone())
        },
        (Deref @ Type::TUPLE { types: rest_ty, .. }, Deref @ metamodelica::List::Cons { head: skip, tail: rest }) if (skip.clone() <= (rest_ty.clone().len() as i32)) => {
            let mut sub_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut rest_ty = (*rest_ty).clone();
            for mut i in 1..=skip.clone() - 1 {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_ty.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                sub_ty = __pa0.clone();
                rest_ty = __pa1.clone();
                index = index.clone() + Type::sizeOf(sub_ty.clone(), false)?;
            }
            let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest_ty.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            sub_ty = __pa2.clone();
            rest_ty = __pa3.clone();
            resolveSkips(index.clone(), sub_ty.clone(), rest.clone(), cref.clone(), fullmap.clone())?
        },
        (Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::RECORD { .. }, .. }, Deref @ metamodelica::List::Cons { head: skip, tail: rest }) => {
            let mut parent: Pointer::Pointer<Arc<Variable::NFVariable>>;
            let mut crefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut field: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut subs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>> = metamodelica::nil();
            field = (match BVariable::getParent(BVariable::getVarPointer(cref.clone(), metamodelica::sourceInfo!())?) {
        Some(mut parent) => {
            subs = ComponentRef::subscriptsAll(cref.clone());
            crefs = {
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut child in (BVariable::getRecordChildren(parent.clone())).into_iter().cloned() {
            let __x = BVariable::getVarName(child.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            crefs = {
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut c in (crefs.clone()).into_iter().cloned() {
            if !(UnorderedMap::contains(c.clone(), fullmap.clone())) { continue; }
            let __x = c.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            if skip.clone() <= (crefs.clone().len() as i32) {
                for mut i in 1..=skip.clone() - 1 {
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(crefs.clone()) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    field = __pa0.clone();
                    crefs = __pa1.clone();
                    field = ComponentRef::setSubscriptsList(subs.clone(), field.clone())?;
                    index = index.clone() + Type::sizeOf(ComponentRef::getSubscriptedType(field.clone(), false)?, false)?;
                }
                let (__pa2, __pa3) = ::match_deref::match_deref! { match &(crefs.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                field = __pa2.clone();
                crefs = __pa3.clone();
                field = ComponentRef::setSubscriptsList(subs.clone(), field.clone())?;
            } else {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSlice.resolveSkips")); __mm_s.push_str(&*literal!(" failed because skip of ")); __mm_s.push_str(&*intString(skip.clone())); __mm_s.push_str(&*literal!(" is too large for record elements ")); __mm_s.push_str(&*List::toString(crefs.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
                bail!("fail");
            }
            field.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSlice.resolveSkips")); __mm_s.push_str(&*literal!(" failed because skip of ")); __mm_s.push_str(&*intString(skip.clone())); __mm_s.push_str(&*literal!(" for type ")); __mm_s.push_str(&*Type::toString(ty.clone())?); __mm_s.push_str(&*literal!(" is requested, but the cref is not part of a record: ")); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
    });
            resolveSkips(index.clone(), ComponentRef::getSubscriptedType(field.clone(), false)?, rest.clone(), cref.clone(), fullmap.clone())?
        },
        (Deref @ Type::ARRAY { .. }, rest) if (Dimension::sizesProduct(var_field!((*ty).dimensions, Type::NFType::ARRAY).clone(), true) == 1) => {
            resolveSkips(index.clone(), var_field!((*ty).elementType, Type::NFType::ARRAY).clone(), rest.clone(), cref.clone(), fullmap.clone())?
        },
        (Deref @ Type::ARRAY { .. }, rest) if (List::compareLength(rest.clone(), var_field!((*ty).dimensions, Type::NFType::ARRAY).clone())? >= 0) => {
            let mut tail: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut rest = (*rest).clone();
            (rest, tail) = List::split(rest.clone(), (var_field!((*ty).dimensions, Type::NFType::ARRAY).clone().len() as i32))?;
            index = locationToIndex({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut dim in (var_field!((*ty).dimensions, Type::NFType::ARRAY).clone()).into_iter().cloned() {
            let __x = Dimension::size(dim.clone(), true)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, rest.clone(), index.clone())?;
            resolveSkips(index.clone(), var_field!((*ty).elementType, Type::NFType::ARRAY).clone(), tail.clone(), cref.clone(), fullmap.clone())?
        },
        (Deref @ Type::ARRAY { .. }, rest) => {
            let mut rest_dim: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
            let mut tail_dim: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
            (rest_dim, tail_dim) = List::split(var_field!((*ty).dimensions, Type::NFType::ARRAY).clone(), (rest.clone().len() as i32))?;
            index = locationToIndex({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut dim in (rest_dim.clone()).into_iter().cloned() {
            let __x = Dimension::size(dim.clone(), true)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, rest.clone(), index.clone())?;
            assign_variant_field!(ty => Type::NFType::ARRAY; dimensions = tail_dim.clone());
            (index.clone(), ty.clone())
        },
        (_, Deref @ metamodelica::List::Cons { head: skip, tail: _ }) if (Type::isTuple(ty.clone()) || Type::isArray(ty.clone())) => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSlice.resolveSkips")); __mm_s.push_str(&*literal!(" failed because skip of ")); __mm_s.push_str(&*intString(skip.clone())); __mm_s.push_str(&*literal!(" for type ")); __mm_s.push_str(&*Type::toString(ty.clone())?); __mm_s.push_str(&*literal!(" is too large.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        (Deref @ Type::TUPLE { .. }, Deref @ metamodelica::List::Nil) => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSlice.resolveSkips")); __mm_s.push_str(&*literal!(" failed because there is no skip for type ")); __mm_s.push_str(&*Type::toString(ty.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        (_, Deref @ metamodelica::List::Cons { head: skip, tail: _ }) if (skip.clone() != 1) => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSlice.resolveSkips")); __mm_s.push_str(&*literal!(" failed because skip of ")); __mm_s.push_str(&*intString(skip.clone())); __mm_s.push_str(&*literal!(" for type ")); __mm_s.push_str(&*Type::toString(ty.clone())?); __mm_s.push_str(&*literal!(" is invalid.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => {
            (index.clone(), ty.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((index, ty))
}

pub type Key = Arc<metamodelica::List<i32>>;

pub type Val1 = Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;

pub type Val2 = Arc<metamodelica::List<i32>>;

fn keyString(mut key: Key) -> ArcStr {
    let mut r#str: ArcStr = List::toString(key.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0).unwrap();
    r#str
}

fn keyHash(mut key: Key) -> i32 {
    let mut hash: i32 = Util::HASH_SEED.clone();
    for mut k in &*key.clone() {
        let mut k = k.clone();
        hash = stringHashDjb2Continue((intString(k.clone())).clone(), hash.clone());
    }
    hash
}

fn keyEqual(mut key1: Key, mut key2: Key) -> bool {
    let mut b: bool = List::isEqualOnTrue(key1.clone(), key2.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>));
    b
}

fn val1String(mut val: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>) -> ArcStr {
    let mut r#str: ArcStr = List::toString(val.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0).unwrap();
    r#str
}

fn resolveDependency(mut original_cref: Arc<ComponentRef::NFComponentRef>, mut eqn_name: Arc<ComponentRef::NFComponentRef>, mut eqn_arr_idx: i32, mut iter: Arc<Iterator::Iterator>, mut ty: Arc<Type::NFType>, mut dep: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Dependency::Dependency>>>, mut rep: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut fullmap: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapping: Arc<Mapping::Mapping>, mut modes: Arc<UnorderedMap::UnorderedMap<(i32, i32), Arc<Mode::Mode>>>) -> Result<()> {
    let mut d: Arc<Dependency::Dependency> = Arc::new(<Dependency::Dependency as ::std::default::Default>::default());
    let mut skip_lst: Arc<metamodelica::List<(i32, Arc<Type::NFType>)>> = metamodelica::nil();
    let mut skip_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut skip_idx: i32 = 0;
    let mut start: i32 = 0;
    let mut size: i32 = 0;
    let mut body_size: i32 = 0;
    let mut iter_size: i32 = 0;
    let mut names: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut maps: Arc<metamodelica::List<Option<Arc<Iterator::Iterator>>>> = metamodelica::nil();
    let mut frames: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)>> = metamodelica::nil();
    let mut regulars: Arc<metamodelica::List<bool>> = metamodelica::nil();
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    match '__try0: {
        cref = unwrap_break_err!(ComponentRef::mapExp(original_cref.clone(), (std::sync::Arc::new(Expression::replaceResizableParameter) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>)), '__try0);
        cref = unwrap_break_err!(ComponentRef::simplifySubscripts(cref.clone(), false), '__try0);
        d = unwrap_break_err!(UnorderedMap::getSafe(original_cref.clone(), dep.clone(), metamodelica::sourceInfo!()), '__try0);
        (start, _) = mapping.eqn_AtS.borrow()[(eqn_arr_idx.clone()-1) as usize].clone();
        if !(unwrap_break_err!(UnorderedSet::contains(cref.clone(), rep.clone()), '__try0)) {
            skip_lst = unwrap_break_err!(resolveSkipsLst(start.clone(), ty.clone(), Arc::new(d.skips.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), cref.clone(), fullmap.clone()), '__try0);
        } else {
            skip_lst = list![(start.clone(), ty.clone())];
        }
        for mut tpl in &*skip_lst.clone() {
            let mut tpl = tpl.clone();
            (skip_idx, skip_ty) = tpl.clone();
            body_size = unwrap_break_err!(Type::sizeOf(skip_ty.clone(), true), '__try0);
            iter_size = Iterator::size(iter.clone(), true);
            size = body_size.clone() * iter_size.clone();
            (names, ranges, maps) = unwrap_break_err!(Iterator::getFrames(iter.clone()), '__try0);
            frames = List::zip3(names.clone(), ranges.clone(), maps.clone());
            regulars = Dependency::toBoolean(d.clone());
            if List::all(regulars.clone(), std::sync::Arc::new(fnptr!(Util::id, _))) {
                unwrap_break_err!(resolveAllRegular(cref.clone(), original_cref.clone(), eqn_name.clone(), skip_idx.clone(), size.clone(), iter_size.clone(), frames.clone(), rep.clone(), map.clone(), m.clone(), mapping.clone(), modes.clone()), '__try0);
            } else if List::any(regulars.clone(), std::sync::Arc::new(fnptr!(Util::id, _))) {
                unwrap_break_err!(resolveMixed(cref.clone(), original_cref.clone(), eqn_name.clone(), skip_idx.clone(), ty.clone(), frames.clone(), regulars.clone(), map.clone(), m.clone(), mapping.clone(), modes.clone()), '__try0);
            } else {
                unwrap_break_err!(resolveAllReduced(cref.clone(), original_cref.clone(), eqn_name.clone(), skip_idx.clone(), size.clone(), iter_size.clone(), frames.clone(), rep.clone(), map.clone(), m.clone(), mapping.clone(), modes.clone()), '__try0);
            }
        }
        Ok::<_, anyhow::Error>((cref.clone(), d.clone(), skip_lst.clone(), start.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            cref = __try0_o0;
            d = __try0_o1;
            skip_lst = __try0_o2;
            start = __try0_o3;
        }
        Err(_) => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSlice.resolveDependency")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*ComponentRef::toString(original_cref.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail");
        }
    }
    Ok(())
}

fn resolveAllRegular(mut cref: Arc<ComponentRef::NFComponentRef>, mut original_cref: Arc<ComponentRef::NFComponentRef>, mut eqn_name: Arc<ComponentRef::NFComponentRef>, mut skip_idx: i32, mut size: i32, mut iter_size: i32, mut frames: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)>>, mut rep: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapping: Arc<Mapping::Mapping>, mut modes: Arc<UnorderedMap::UnorderedMap<(i32, i32), Arc<Mode::Mode>>>) -> Result<()> {
    let mut mode: Arc<Mode::Mode> = Arc::new(<Mode::Mode as ::std::default::Default>::default());
    let mut scalarized: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut map3: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<i32>>>>;
    let mut scal_size: i32 = 0;
    let mut shift: i32 = 0;
    mode = Mode::create(eqn_name.clone(), list![original_cref.clone()], false);
    scalarized = ComponentRef::scalarizeAll(cref.clone(), true)?.reverse();
    map3 = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
    for mut scal in &*scalarized.clone() {
        let mut scal = scal.clone();
        UnorderedMap::add(scal.clone(), getCrefInFrameIndices(scal.clone(), frames.clone(), mapping.clone(), map.clone(), true)?, map3.clone())?;
    }
    scal_size = (List::flatten(UnorderedMap::valueList(map3.clone())).len() as i32);
    if size.clone() == scal_size.clone() || UnorderedSet::contains(cref.clone(), rep.clone())? && intMod(size.clone(), scal_size.clone()) == 0 {
        shift = 0;
        for mut i in 1..=size.clone() / scal_size.clone() {
            for mut scal in &*scalarized.clone() {
                let mut scal = scal.clone();
                for mut scal_idx in &*UnorderedMap::getSafe(scal.clone(), map3.clone(), metamodelica::sourceInfo!())? {
                    let mut scal_idx = scal_idx.clone();
                    addMatrixEntry(m.clone(), modes.clone(), skip_idx.clone() + shift.clone(), scal_idx.clone(), mode.clone())?;
                    shift = shift.clone() + 1;
                }
            }
        }
    } else {
        resolveAllReduced(cref.clone(), original_cref.clone(), eqn_name.clone(), skip_idx.clone(), size.clone(), iter_size.clone(), frames.clone(), rep.clone(), map.clone(), m.clone(), mapping.clone(), modes.clone())?;
    }
    Ok(())
}

fn resolveMixed(mut cref: Arc<ComponentRef::NFComponentRef>, mut original_cref: Arc<ComponentRef::NFComponentRef>, mut eqn_name: Arc<ComponentRef::NFComponentRef>, mut skip_idx: i32, mut ty: Arc<Type::NFType>, mut frames: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)>>, mut regulars: Arc<metamodelica::List<bool>>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapping: Arc<Mapping::Mapping>, mut modes: Arc<UnorderedMap::UnorderedMap<(i32, i32), Arc<Mode::Mode>>>) -> Result<()> {
    let mut stripped: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut eq_dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut key: metamodelica::Array<i32>;
    let mut map1: Arc<UnorderedMap::UnorderedMap<Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>;
    let mut map2: Arc<UnorderedMap::UnorderedMap<Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>>>;
    let mut scalarized: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut scal_lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut size_comp: i32 = 0;
    let mut eq_reg: Arc<metamodelica::List<bool>> = metamodelica::nil();
    subs = ComponentRef::subscriptsAllWithWholeFlat(cref.clone());
    dims = Type::arrayDims(ComponentRef::getSubscriptedType(cref.clone(), false)?);
    eq_dims = Type::arrayDims(ty.clone());
    if List::compareLength(subs.clone(), dims.clone())? == 0 && List::compareLength(subs.clone(), regulars.clone())? == 0 {
        stripped = ComponentRef::stripSubscriptsAll(cref.clone());
        key = arrayCreate((subs.clone().len() as i32), 0);
        map1 = UnorderedMap::new((std::sync::Arc::new(fnptr!(keyHash, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(keyEqual, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) -> Result<bool> + 'static>), 1);
        resolveReductions(List::zip3(subs.clone(), dims.clone(), regulars.clone()), map1.clone(), key.clone(), stripped.clone(), metamodelica::nil(), 1)?;
        map2 = UnorderedMap::new((std::sync::Arc::new(fnptr!(keyHash, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(keyEqual, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) -> Result<bool> + 'static>), 1);
        for mut k in &*UnorderedMap::keyList(map1.clone()) {
            let mut k = k.clone();
            scalarized = UnorderedMap::getSafe(k.clone(), map1.clone(), metamodelica::sourceInfo!())?;
            scal_lst = List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
        for mut scal in (scalarized.clone()).into_iter().cloned() {
            let __x = getCrefInFrameIndices(scal.clone(), frames.clone(), mapping.clone(), map.clone(), true)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            UnorderedMap::add(k.clone(), scal_lst.clone(), map2.clone())?;
        }
        size_comp = List::compareLength(eq_dims.clone(), regulars.clone())?;
        if size_comp.clone() > 0 {
            eq_reg = listAppend(regulars.clone(), List::fill(false, (eq_dims.clone().len() as i32) - (regulars.clone().len() as i32)));
        } else if size_comp.clone() < 0 {
            eq_reg = List::filterOnTrue(regulars.clone(), std::sync::Arc::new(fnptr!(Util::id, _)));
            size_comp = List::compareLength(eq_dims.clone(), eq_reg.clone())?;
            if size_comp.clone() > 0 {
                eq_reg = listAppend(eq_reg.clone(), List::fill(false, (eq_dims.clone().len() as i32) - (eq_reg.clone().len() as i32)));
            } else if size_comp.clone() < 0 {
                eq_reg = List::firstN(eq_reg.clone(), (eq_dims.clone().len() as i32))?;
            }
        } else {
            eq_reg = regulars.clone();
        }
        key = arrayCreate((subs.clone().len() as i32), 0);
        resolveEquationDimensions(List::zip(eq_dims.clone(), eq_reg.clone()), map2.clone(), key.clone(), m.clone(), modes.clone(), Mode::create(eqn_name.clone(), list![original_cref.clone()], false), Pointer::create(skip_idx.clone()), 1)?;
    } else {
        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSlice.resolveMixed")); __mm_s.push_str(&*literal!(" failed because subscripts, dimensions and dependencies were not of equal length.\n")); __mm_s.push_str(&*literal!("variable subscripts(")); __mm_s.push_str(&*intString((subs.clone().len() as i32))); __mm_s.push_str(&*literal!("): ")); __mm_s.push_str(&*List::toString(subs.clone(), (std::sync::Arc::new(Subscript::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Subscript::NFSubscript>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("variable dimensions(")); __mm_s.push_str(&*intString((dims.clone().len() as i32))); __mm_s.push_str(&*literal!("): ")); __mm_s.push_str(&*List::toString(dims.clone(), (std::sync::Arc::new(Dimension::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Dimension::NFDimension>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("equation dimensions(")); __mm_s.push_str(&*intString((eq_dims.clone().len() as i32))); __mm_s.push_str(&*literal!("): ")); __mm_s.push_str(&*List::toString(eq_dims.clone(), (std::sync::Arc::new(Dimension::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Dimension::NFDimension>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("variable dependencies(")); __mm_s.push_str(&*intString((regulars.clone().len() as i32))); __mm_s.push_str(&*literal!("): ")); __mm_s.push_str(&*List::toString(regulars.clone(), (std::sync::Arc::new(fnptr!(boolString, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone()])?;
        bail!("fail");
    }
    Ok(())
}

fn resolveAllReduced(mut cref: Arc<ComponentRef::NFComponentRef>, mut original_cref: Arc<ComponentRef::NFComponentRef>, mut eqn_name: Arc<ComponentRef::NFComponentRef>, mut skip_idx: i32, mut size: i32, mut iter_size: i32, mut frames: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)>>, mut rep: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapping: Arc<Mapping::Mapping>, mut modes: Arc<UnorderedMap::UnorderedMap<(i32, i32), Arc<Mode::Mode>>>) -> Result<()> {
    let mut repeated: bool = false;
    let mut scalarized: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut map3: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<i32>>>>;
    let mut shift: i32 = 0;
    let mut mode: Arc<Mode::Mode> = Arc::new(<Mode::Mode as ::std::default::Default>::default());
    repeated = UnorderedSet::contains(cref.clone(), rep.clone())?;
    scalarized = ComponentRef::scalarizeAll(cref.clone(), true)?.reverse();
    map3 = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
    for mut scal in &*scalarized.clone() {
        let mut scal = scal.clone();
        UnorderedMap::add(scal.clone(), getCrefInFrameIndices(scal.clone(), frames.clone(), mapping.clone(), map.clone(), true)?, map3.clone())?;
    }
    if repeated.clone() {
        mode = Mode::create(eqn_name.clone(), list![original_cref.clone()], false);
    }
    for mut i in ({let __s=skip_idx.clone(); let __e=skip_idx.clone() + size.clone() - iter_size.clone(); let __step=iter_size.clone(); if __step>0 {__s..=__e} else {__e..=__s}}).step_by((if iter_size.clone()>0 {iter_size.clone()} else {-(iter_size.clone())}) as usize) {
        shift = 0;
        for mut scal in &*scalarized.clone() {
            let mut scal = scal.clone();
            if !(repeated.clone()) {
                mode = Mode::create(eqn_name.clone(), list![original_cref.clone()], true);
            }
            for mut scal_idx in &*UnorderedMap::getSafe(scal.clone(), map3.clone(), metamodelica::sourceInfo!())? {
                let mut scal_idx = scal_idx.clone();
                if intMod(shift.clone(), iter_size.clone()) == 0 {
                    shift = 0;
                }
                addMatrixEntry(m.clone(), modes.clone(), i.clone() + shift.clone(), scal_idx.clone(), mode.clone())?;
                shift = shift.clone() + 1;
            }
        }
    }
    Ok(())
}

fn resolveEquationDimensions(mut lst: Arc<metamodelica::List<(Arc<Dimension::NFDimension>, bool)>>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>>>, mut key: metamodelica::Array<i32>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut modes: Arc<UnorderedMap::UnorderedMap<(i32, i32), Arc<Mode::Mode>>>, mut mode: Arc<Mode::Mode>, mut eqn_idx_ptr: Pointer::Pointer<i32>, mut index: i32) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(lst.clone()) {
        Deref @ metamodelica::List::Nil => {
            let mut eqn_idx: i32 = 0;
            let mut scal_lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            eqn_idx = Pointer::access(eqn_idx_ptr.clone());
            scal_lst = UnorderedMap::getSafe(Arc::new(key.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), map.clone(), metamodelica::sourceInfo!())?;
            for mut scal_idx in &*scal_lst.clone() {
                let mut scal_idx = scal_idx.clone();
                addMatrixEntry(m.clone(), modes.clone(), eqn_idx.clone(), scal_idx.clone(), mode.clone())?;
            }
            Pointer::update(eqn_idx_ptr.clone(), eqn_idx.clone() + 1);
            ()
        },
        Deref @ metamodelica::List::Cons { head: (dim, false), tail: rest } => {
            for mut i in 1..=Dimension::size(dim.clone(), true)? {
                resolveEquationDimensions(rest.clone(), map.clone(), key.clone(), m.clone(), modes.clone(), mode.clone(), eqn_idx_ptr.clone(), index.clone() + 1)?;
            }
            ()
        },
        Deref @ metamodelica::List::Cons { head: (dim, true), tail: rest } => {
            for mut i in 1..=Dimension::size(dim.clone(), true)? {
                {let _arr = key.clone(); _arr.borrow_mut()[(index.clone()-1) as usize] = i.clone(); _arr};
                resolveEquationDimensions(rest.clone(), map.clone(), key.clone(), m.clone(), modes.clone(), mode.clone(), eqn_idx_ptr.clone(), index.clone() + 1)?;
            }
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn addMatrixEntry(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut modes: Arc<UnorderedMap::UnorderedMap<(i32, i32), Arc<Mode::Mode>>>, mut eqn_idx: i32, mut var_idx: i32, mut mode: Arc<Mode::Mode>) -> Result<()> {
    if '__try0: {
        if var_idx.clone() > 0 {
            {let _arr = m.clone(); let _val = cons(var_idx.clone(), m.borrow()[(eqn_idx.clone()-1) as usize].clone()); _arr.borrow_mut()[(eqn_idx.clone()-1) as usize] = _val; _arr};
            unwrap_break_err!(UnorderedMap::addUpdate((eqn_idx.clone(), var_idx.clone()), Arc::new({ let __pe_b1 = mode.clone(); move |__pe_a0| Ok(Mode::mergeCreate(__pe_a0, __pe_b1.clone())) }), modes.clone()), '__try0);
        }
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSlice.addMatrixEntry")); __mm_s.push_str(&*literal!(" failed because index ")); __mm_s.push_str(&*intString(eqn_idx.clone())); __mm_s.push_str(&*literal!(" could not be added. Matrix size: ")); __mm_s.push_str(&*intString((m.clone().borrow().len() as i32))); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
        bail!("fail");
    }
    Ok(())
}

fn resolveReductions(mut lst: Arc<metamodelica::List<(Arc<Subscript::NFSubscript>, Arc<Dimension::NFDimension>, bool)>>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>, mut key: metamodelica::Array<i32>, mut stripped: Arc<ComponentRef::NFComponentRef>, mut acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut index: i32) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(lst.clone()) {
        Deref @ metamodelica::List::Nil => {
            let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut val: Val1 = metamodelica::nil();
            cref = ComponentRef::mergeSubscripts(acc.clone().reverse(), stripped.clone(), true, false, false)?;
            val = ComponentRef::scalarizeAll(cref.clone(), true)?;
            UnorderedMap::add(Arc::new(key.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), val.clone(), map.clone())?;
            ()
        },
        Deref @ metamodelica::List::Cons { head: (sub, _, false), tail: rest } => {
            resolveReductions(rest.clone(), map.clone(), key.clone(), stripped.clone(), cons(sub.clone(), acc.clone()), index.clone() + 1)?;
            ()
        },
        Deref @ metamodelica::List::Cons { head: (sub, dim, true), tail: rest } => {
            let mut sub_idx: i32 = 0;
            sub_idx = 1;
            for mut s in &*Subscript::scalarize(sub.clone(), dim.clone(), true)? {
                let mut s = s.clone();
                {let _arr = key.clone(); _arr.borrow_mut()[(index.clone()-1) as usize] = sub_idx.clone(); _arr};
                resolveReductions(rest.clone(), map.clone(), key.clone(), stripped.clone(), cons(s.clone(), acc.clone()), index.clone() + 1)?;
                sub_idx = sub_idx.clone() + 1;
            }
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn combineFrames2Indices(mut first: i32, mut sizes: Arc<metamodelica::List<i32>>, mut subs: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut frames: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)>>, mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>, mut resize: bool, mut indices: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut indices: Arc<metamodelica::List<i32>> = indices;
    indices = (::match_deref::match_deref! { match &(frames.clone()) {
        Deref @ metamodelica::List::Nil => {
            let mut values: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            values = resolveDimensionsSubscripts(sizes.clone(), subs.clone(), replacements.clone(), resize.clone())?;
            {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut v in (values.clone()).into_iter().cloned() {
            let __x = locationToIndex(sizes.clone(), v.clone(), first.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
        },
        Deref @ metamodelica::List::Cons { head: (iterator, range, map), tail: rest } => {
            let mut start: i32 = 0;
            let mut step: i32 = 0;
            let mut stop: i32 = 0;
            let mut values: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut iterator_exps: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            let mut iterator_lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut sub_idx: i32 = 0;
            iterator_lst = (::match_deref::match_deref! { match &(range.clone()) {
        Deref @ Expression::RANGE { .. } => {
            (start, step, stop) = Expression::getIntegerRange(range.clone(), resize.clone())?;
            List::intRange3(start.clone(), step.clone(), stop.clone())?
        },
        Deref @ Expression::ARRAY { .. } => {
            iterator_exps = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut e in (var_field!((*range).elements, Expression::NFExpression::ARRAY).clone()).borrow().iter() {
            let __x = Expression::map(e.clone(), Arc::new({ let __pe_b1 = replacements.clone(); move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone()) }))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            iterator_lst = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut e in (iterator_exps.clone()).into_iter().cloned() {
            let __x = Expression::integerValue(SimplifyExp::simplifyDump(e.clone(), true, (literal!("NBSlice.combineFrames2Indices")).clone(), (literal!("")).clone())?)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            iterator_lst.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSlice.combineFrames2Indices")); __mm_s.push_str(&*literal!(" failed because iterator binding could not be parsed: ")); __mm_s.push_str(&*ComponentRef::toString(iterator.clone())?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*Expression::toString(range.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            sub_idx = 1;
            for mut index in &*iterator_lst.clone() {
                let mut index = index.clone();
                UnorderedMap::add(iterator.clone(), Arc::new(Expression::NFExpression::INTEGER { value: index.clone() }), replacements.clone())?;
                Iterator::createMappedLocationReplacement(map.clone(), sub_idx.clone(), replacements.clone())?;
                if rest.clone().is_empty() {
                    values = resolveDimensionsSubscripts(sizes.clone(), subs.clone(), replacements.clone(), resize.clone())?;
                    for mut v in &*values.clone().reverse() {
                        let mut v = v.clone();
                        indices = cons(locationToIndex(sizes.clone(), v.clone(), first.clone())?, indices.clone());
                    }
                } else {
                    indices = combineFrames2Indices(first.clone(), sizes.clone(), subs.clone(), rest.clone(), replacements.clone(), resize.clone(), indices.clone())?;
                }
                sub_idx = sub_idx.clone() + 1;
            }
            indices.clone()
        },
        Deref @ metamodelica::List::Cons { head: (iterator, range, _), tail: _ } => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSlice.combineFrames2Indices")); __mm_s.push_str(&*literal!(" failed because uniontype records are wrong: ")); __mm_s.push_str(&*ComponentRef::toString(iterator.clone())?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*Expression::toString(range.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSlice.combineFrames2Indices")); __mm_s.push_str(&*literal!(" failed for an unknown reason.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(indices)
}

fn getCrefInFrameIndices(mut cref: Arc<ComponentRef::NFComponentRef>, mut frames: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)>>, mut mapping: Arc<Mapping::Mapping>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut resize: bool) -> Result<Arc<metamodelica::List<i32>>> {
    let mut scal_lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut final_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut var_arr_idx: i32 = 0;
    let mut var_start: i32 = 0;
    (final_cref, var_arr_idx) = getVarArrIdx(cref.clone(), mapping.clone(), map.clone())?;
    (var_start, _) = mapping.var_AtS.borrow()[(var_arr_idx.clone()-1) as usize].clone();
    scal_lst = getCrefInFrameIndicesLocal(cref.clone(), final_cref.clone(), frames.clone(), var_start.clone(), resize.clone())?;
    Ok(scal_lst)
}

fn getVarArrIdx(mut cref: Arc<ComponentRef::NFComponentRef>, mut mapping: Arc<Mapping::Mapping>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>) -> Result<(Arc<ComponentRef::NFComponentRef>, i32)> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = cref;
    let mut var_arr_idx: i32 = 0;
    (var_arr_idx, cref) = (match UnorderedMap::get(cref.clone(), map.clone()) {
        Some(mut var_arr_idx) => (var_arr_idx.clone(), cref.clone()),
        _ => {
            cref = ComponentRef::stripSubscriptsAll(cref.clone());
            (UnorderedMap::getSafe(cref.clone(), map.clone(), metamodelica::sourceInfo!())?, cref.clone())
        },
    });
    Ok((cref, var_arr_idx))
}

pub fn getCrefInFrameIndicesLocal(mut subscripted_cref: Arc<ComponentRef::NFComponentRef>, mut stripped_cref: Arc<ComponentRef::NFComponentRef>, mut frames: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)>>, mut var_start: i32, mut resize: bool) -> Result<Arc<metamodelica::List<i32>>> {
    let mut scal_lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut sizes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut subs: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut complex_size: i32 = 0;
    sizes = ComponentRef::sizes(stripped_cref.clone(), false, resize.clone(), metamodelica::nil())?;
    subs = ComponentRef::subscriptsToExpression(subscripted_cref.clone(), true)?;
    ty = Type::arrayElementType(ComponentRef::getComponentType(subscripted_cref.clone()));
    scal_lst = (match Type::complexSize(ty.clone(), false)? {
        Some(mut complex_size) => {
            scal_lst = metamodelica::nil();
            for mut i in (1..=complex_size.clone()).rev() {
                scal_lst = listAppend(combineFrames2Indices(var_start.clone(), cons(complex_size.clone(), sizes.clone()), cons(Arc::new(Expression::NFExpression::INTEGER { value: i.clone() }), subs.clone()), frames.clone(), UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1), resize.clone(), metamodelica::nil())?.reverse(), scal_lst.clone());
            }
            scal_lst.clone()
        },
        _ => combineFrames2Indices(var_start.clone(), sizes.clone(), subs.clone(), frames.clone(), UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1), resize.clone(), metamodelica::nil())?.reverse(),
    });
    Ok(scal_lst)
}

fn resolveDimensionsSubscripts(mut sizes: Arc<metamodelica::List<i32>>, mut subs: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>, mut resize: bool) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut values: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut replaced: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    replaced = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut sub in (subs.clone()).into_iter().cloned() {
            let __x = Expression::map(sub.clone(), Arc::new({ let __pe_b1 = replacements.clone(); move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone()) }))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    values = {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
        for (exp, size) in (&(replaced.clone())).into_iter().zip((&(sizes.clone())).into_iter()) {
            let __x = resolveDimensionsSubscript(exp.clone(), size.clone(), resize.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    values = List::combination(values.clone());
    Ok(values)
}

fn resolveDimensionsSubscript(mut replaced: Arc<Expression::NFExpression>, mut size: i32, mut resize: bool) -> Result<Arc<metamodelica::List<i32>>> {
    let mut res: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut rep: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    rep = SimplifyExp::simplifyDump(replaced.clone(), true, (literal!("NBSlice.resolveDimensionsSubscript")).clone(), (literal!("")).clone())?;
    res = (::match_deref::match_deref! { match &(rep.clone()) {
        Deref @ Expression::INTEGER { .. } => {
            list![var_field!((*rep).value, Expression::NFExpression::INTEGER).clone()]
        },
        Deref @ Expression::ENUM_LITERAL { .. } => {
            list![var_field!((*rep).index, Expression::NFExpression::ENUM_LITERAL).clone()]
        },
        Deref @ Expression::RANGE { .. } => {
            let mut start: i32 = 0;
            let mut step: i32 = 0;
            let mut stop: i32 = 0;
            (start, step, stop) = Expression::getIntegerRange(rep.clone(), resize.clone())?;
            List::intRange3(start.clone(), step.clone(), stop.clone())?
        },
        Deref @ Expression::ARRAY { .. } => {
            List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
        for mut e in (var_field!((*rep).elements, Expression::NFExpression::ARRAY).clone()).borrow().iter() {
            let __x = resolveDimensionsSubscript(e.clone(), size.clone(), resize.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
        },
        _ => {
            List::intRange(size.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

fn applyNewFrameRange(mut frame: (Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>), mut range: (i32, i32, i32)) -> Result<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)> {
    let mut frame: (Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>) = frame;
    frame = (::match_deref::match_deref! { match &(frame.clone()) {
        (name, exp @ Deref @ Expression::RANGE { .. }, map) => {
            (name.clone(), Expression::sliceRange(exp.clone(), range.clone())?, map.clone())
        },
        (_, exp, _) => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSlice.applyNewFrameRange")); __mm_s.push_str(&*literal!(" failed because frame expression was not Expression.RANGE(): ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(frame)
}


