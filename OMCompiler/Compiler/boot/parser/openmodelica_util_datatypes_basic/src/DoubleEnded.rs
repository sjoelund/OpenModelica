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

use crate::GCExt;
use crate::Mutable;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MutableList<T: Clone> {
    pub length: Mutable::Mutable<i32>,
    pub front: Mutable::Mutable<Arc<metamodelica::List<T>>>,
    pub back: Mutable::Mutable<Arc<metamodelica::List<T>>>,
}

pub type LIST<T> = MutableList<T>;


pub fn new<T: Clone + 'static>(mut first: T) -> MutableList<T> {
    let mut delst: MutableList<T>;
    let mut lst: Arc<metamodelica::List<T>> = list![first.clone()];
    delst = MutableList { length: Mutable::create(1), front: Mutable::create(lst.clone()), back: Mutable::create(lst.clone()) };
    delst
}

pub fn fromList<T: Clone + 'static>(mut lst: Arc<metamodelica::List<T>>) -> Result<MutableList<T>> {
    let mut delst: MutableList<T>;
    let mut head: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut tail: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut tmp: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut length: i32 = 0;
    let mut t: T;
    if lst.clone().is_empty() {
        delst = MutableList { length: Mutable::create(0), front: Mutable::create(metamodelica::nil()), back: Mutable::create(metamodelica::nil()) };
        return Ok(delst);
    }
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lst.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    t = __pa0.clone();
    tmp = __pa1.clone();
    head = list![t.clone()];
    tail = head.clone();
    length = 1;
    for mut l in &*tmp.clone() {
        let mut l = l.clone();
        tmp = list![l.clone()];
        Dangerous::listSetRest(tail.clone(), tmp.clone())?;
        tail = tmp.clone();
        length = length.clone() + 1;
    }
    delst = MutableList { length: Mutable::create(length.clone()), front: Mutable::create(head.clone()), back: Mutable::create(tail.clone()) };
    Ok(delst)
}

pub fn empty<T: Clone + 'static>(mut dummy: T) -> MutableList<T> {
    let mut delst: MutableList<T>;
    delst = MutableList { length: Mutable::create(0), front: Mutable::create(metamodelica::nil()), back: Mutable::create(metamodelica::nil()) };
    delst
}

pub fn length<T: Clone + 'static>(mut delst: MutableList<T>) -> i32 {
    let mut length: i32 = 0;
    length = Mutable::access(delst.length.clone());
    length
}

pub fn pop_front<T: Clone + 'static>(mut delst: MutableList<T>) -> Result<T> {
    let mut elt: T;
    let mut length: i32 = Mutable::access(delst.length.clone());
    let mut lst: Arc<metamodelica::List<T>> = metamodelica::nil();
    let true = (length.clone() > 0) else { bail!("pattern mismatch") };
    Mutable::update(delst.length.clone(), length.clone() - 1);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Mutable::access(delst.front.clone())) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    elt = __pa0.clone();
    lst = __pa1.clone();
    if length.clone() == 1 {
        Mutable::update(delst.front.clone(), metamodelica::nil());
        Mutable::update(delst.back.clone(), metamodelica::nil());
        return Ok(elt);
    }
    Mutable::update(delst.front.clone(), lst.clone());
    Ok(elt)
}

pub fn currentBackCell<T: Clone + 'static>(mut delst: MutableList<T>) -> Arc<metamodelica::List<T>> {
    let mut last: Arc<metamodelica::List<T>> = metamodelica::nil();
    last = Mutable::access(delst.back.clone());
    last
}

pub fn push_front<T: Clone + 'static>(mut delst: MutableList<T>, mut elt: T) -> () {
    let mut length: i32 = Mutable::access(delst.length.clone());
    let mut lst: Arc<metamodelica::List<T>> = metamodelica::nil();
    Mutable::update(delst.length.clone(), length.clone() + 1);
    if length.clone() == 0 {
        lst = list![elt.clone()];
        Mutable::update(delst.front.clone(), lst.clone());
        Mutable::update(delst.back.clone(), lst.clone());
        return ();
    }
    lst = Mutable::access(delst.front.clone());
    Mutable::update(delst.front.clone(), cons(elt.clone(), lst.clone()));
    ()
}

pub fn push_list_front<T: Clone + 'static>(mut delst: MutableList<T>, mut lst: Arc<metamodelica::List<T>>) -> Result<()> {
    let mut length: i32 = Mutable::access(delst.length.clone());
    let mut lstLength: i32 = 0;
    let mut work: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut oldHead: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut tmp: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut head: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut t: T;
    lstLength = (lst.clone().len() as i32);
    if lstLength.clone() == 0 {
        return Ok(());
    }
    Mutable::update(delst.length.clone(), length.clone() + lstLength.clone());
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lst.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    t = __pa0.clone();
    tmp = __pa1.clone();
    head = list![t.clone()];
    oldHead = Mutable::access(delst.front.clone());
    Mutable::update(delst.front.clone(), head.clone());
    for mut l in &*tmp.clone() {
        let mut l = l.clone();
        work = list![l.clone()];
        Dangerous::listSetRest(head.clone(), work.clone())?;
        head = work.clone();
    }
    if length.clone() == 0 {
        Mutable::update(delst.back.clone(), head.clone());
    } else {
        Dangerous::listSetRest(head.clone(), oldHead.clone())?;
    }
    Ok(())
}

pub fn push_back<T: Clone + 'static>(mut delst: MutableList<T>, mut elt: T) -> () {
    let mut length: i32 = Mutable::access(delst.length.clone());
    let mut lst: Arc<metamodelica::List<T>> = metamodelica::nil();
    Mutable::update(delst.length.clone(), length.clone() + 1);
    if length.clone() == 0 {
        lst = list![elt.clone()];
        Mutable::update(delst.front.clone(), lst.clone());
        Mutable::update(delst.back.clone(), lst.clone());
        return ();
    }
    lst = list![elt.clone()];
    Dangerous::listSetRest(Mutable::access(delst.back.clone()), lst.clone()).unwrap();
    Mutable::update(delst.back.clone(), lst.clone());
    ()
}

pub fn push_list_back<T: Clone + 'static>(mut delst: MutableList<T>, mut lst: Arc<metamodelica::List<T>>) -> () {
    let mut length: i32 = Mutable::access(delst.length.clone());
    let mut lstLength: i32 = 0;
    let mut tail: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut tmp: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut t: T;
    lstLength = (lst.clone().len() as i32);
    if lstLength.clone() == 0 {
        return ();
    }
    Mutable::update(delst.length.clone(), length.clone() + lstLength.clone());
    t = (lst.clone()).get(1).unwrap();
    tmp = list![t.clone()];
    if length.clone() == 0 {
        Mutable::update(delst.front.clone(), tmp.clone());
    } else {
        Dangerous::listSetRest(Mutable::access(delst.back.clone()), tmp.clone()).unwrap();
    }
    tail = tmp.clone();
    for mut l in &*listRest(lst.clone()).unwrap() {
        let mut l = l.clone();
        tmp = list![l.clone()];
        Dangerous::listSetRest(tail.clone(), tmp.clone()).unwrap();
        tail = tmp.clone();
    }
    Mutable::update(delst.back.clone(), tail.clone());
    ()
}

pub fn toListAndClear<T: Clone + 'static>(mut delst: MutableList<T>, mut prependToList: Arc<metamodelica::List<T>>) -> Arc<metamodelica::List<T>> {
    let mut res: Arc<metamodelica::List<T>> = metamodelica::nil();
    if Mutable::access(delst.length.clone()) == 0 {
        res = prependToList.clone();
        return res;
    }
    res = Mutable::access(delst.front.clone());
    if !(prependToList.clone().is_empty()) {
        Dangerous::listSetRest(Mutable::access(delst.back.clone()), prependToList.clone()).unwrap();
    }
    Mutable::update(delst.back.clone(), metamodelica::nil());
    Mutable::update(delst.front.clone(), metamodelica::nil());
    Mutable::update(delst.length.clone(), 0);
    res
}

pub fn toListNoCopyNoClear<T: Clone + 'static>(mut delst: MutableList<T>) -> Arc<metamodelica::List<T>> {
    let mut res: Arc<metamodelica::List<T>> = metamodelica::nil();
    res = Mutable::access(delst.front.clone());
    res
}

pub fn clear<T: Clone + 'static>(mut delst: MutableList<T>) -> () {
    let mut lst: Arc<metamodelica::List<T>> = metamodelica::nil();
    lst = Mutable::access(delst.front.clone());
    Mutable::update(delst.back.clone(), metamodelica::nil());
    Mutable::update(delst.front.clone(), metamodelica::nil());
    Mutable::update(delst.length.clone(), 0);
    for mut l in &*lst.clone() {
        let mut l = l.clone();
        GCExt::free(l.clone());
    }
    ()
}

pub fn mapNoCopy_1<T: Clone + 'static, ArgT1: Clone + 'static>(mut delst: MutableList<T>, mut inMapFunc: Arc<dyn ::std::ops::Fn(T, ArgT1) -> Result<T> + 'static>, mut inArg1: ArgT1) -> Result<()> {
    pub type MapFunc<T: Clone + 'static, ArgT1: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1) -> Result<T> + 'static>;

    let mut lst: Arc<metamodelica::List<T>> = Mutable::access(delst.front.clone());
    while !(lst.clone().is_empty()) {
        Dangerous::listSetFirst(lst.clone(), inMapFunc((lst.clone()).get(1)?, inArg1.clone())?)?;
        let __pa0 = ::match_deref::match_deref! { match &(lst.clone()) {
            Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        lst = __pa0.clone();
    }
    Ok(())
}

pub fn mapFoldNoCopy<T: Clone + 'static, ArgT1: Clone + 'static>(mut delst: MutableList<T>, mut inMapFunc: Arc<dyn ::std::ops::Fn(T, ArgT1) -> Result<(T, ArgT1)> + 'static>, mut arg: ArgT1) -> Result<ArgT1> {
    pub type MapFunc<T: Clone + 'static, ArgT1: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1) -> Result<(T, ArgT1)> + 'static>;

    let mut arg: ArgT1 = arg;
    let mut element: T;
    let mut lst: Arc<metamodelica::List<T>> = Mutable::access(delst.front.clone());
    while !(lst.clone().is_empty()) {
        (element, arg) = inMapFunc((lst.clone()).get(1)?, arg.clone())?;
        Dangerous::listSetFirst(lst.clone(), element.clone())?;
        let __pa0 = ::match_deref::match_deref! { match &(lst.clone()) {
            Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        lst = __pa0.clone();
    }
    Ok(arg)
}

