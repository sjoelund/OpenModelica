// Auto-generated from MetaModelica source
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

#[derive(Clone, Debug, PartialEq)]
pub struct MutableList<T: Clone + 'static> {
    pub length: Mutable::Mutable<i32>,
    pub front: Mutable::Mutable<Arc<metamodelica::List<T>>>,
    pub back: Mutable::Mutable<Arc<metamodelica::List<T>>>,
}

pub type LIST<T> = MutableList<T>;


pub fn clear<T: Clone + 'static>(delst: MutableList<T>) -> () {
    let mut lst: Arc<metamodelica::List<T>>;
    lst = Mutable::access(delst.front.clone());
    Mutable::update(delst.back.clone(), metamodelica::nil());
    Mutable::update(delst.front.clone(), metamodelica::nil());
    Mutable::update(delst.length.clone(), 0);
    for l in &*lst.clone() {
        GCExt::free(l.clone());
    }
    ()
}

pub fn currentBackCell<T: Clone + 'static>(delst: MutableList<T>) -> Arc<metamodelica::List<T>> {
    let mut last: Arc<metamodelica::List<T>>;
    last = Mutable::access(delst.back.clone());
    last
}

pub fn empty<T: Clone + 'static>(dummy: T) -> MutableList<T> {
    let mut delst: MutableList<T>;
    delst = MutableList { length: Mutable::create(0), front: Mutable::create(metamodelica::nil()), back: Mutable::create(metamodelica::nil()) };
    delst
}

pub fn fromList<T: Clone + 'static>(lst: Arc<metamodelica::List<T>>) -> Result<MutableList<T>> {
    let mut delst: MutableList<T>;
    let mut head: Arc<metamodelica::List<T>>;
    let mut tail: Arc<metamodelica::List<T>>;
    let mut tmp: Arc<metamodelica::List<T>>;
    let mut length: i32;
    let mut t: T;
    if lst.clone().is_empty() {
        delst = MutableList { length: Mutable::create(0), front: Mutable::create(metamodelica::nil()), back: Mutable::create(metamodelica::nil()) };
        return Ok(delst);
    }
    let metamodelica::List::Cons { head: __pa0, tail: __pa1 } = &(lst.clone()) else { bail!("pattern mismatch") };
    t = __pa0.clone();
    tmp = __pa1.clone();
    head = list![t.clone()];
    tail = head.clone();
    length = 1;
    for l in &*tmp.clone() {
        tmp = list![l.clone()];
        Dangerous::listSetRest(tail.clone(), tmp.clone())?;
        tail = tmp.clone();
        length = length.clone() + 1;
    }
    delst = MutableList { length: Mutable::create(length.clone()), front: Mutable::create(head.clone()), back: Mutable::create(tail.clone()) };
    Ok(delst)
}

pub fn length<T: Clone + 'static>(delst: MutableList<T>) -> i32 {
    let mut length: i32;
    length = Mutable::access(delst.length.clone());
    length
}

pub fn mapFoldNoCopy<T: Clone + 'static, ArgT1: Clone + 'static>(delst: MutableList<T>, inMapFunc: &impl Fn(T, ArgT1) -> Result<(T, ArgT1)>, arg: ArgT1) -> Result<ArgT1> {
    pub type MapFunc<T: Clone + 'static, ArgT1: Clone + 'static> = fn(T, ArgT1) -> Result<(T, ArgT1)>;

    let mut arg: ArgT1 = arg;
    let mut element: T;
    let mut lst: Arc<metamodelica::List<T>> = Mutable::access(delst.front.clone());
    while !(lst.clone().is_empty()) {
        (element, arg) = inMapFunc((lst.clone()).get(1)?, arg.clone())?;
        Dangerous::listSetFirst(lst.clone(), element.clone())?;
        let metamodelica::List::Cons { head: _, tail: __pa0 } = &(lst.clone()) else { bail!("pattern mismatch") };
        lst = __pa0.clone();
    }
    Ok(arg)
}

pub fn mapNoCopy_1<T: Clone + 'static, ArgT1: Clone + 'static>(delst: MutableList<T>, inMapFunc: &impl Fn(T, ArgT1) -> Result<T>, inArg1: ArgT1) -> Result<()> {
    pub type MapFunc<T: Clone + 'static, ArgT1: Clone + 'static> = fn(T, ArgT1) -> Result<T>;

    let mut lst: Arc<metamodelica::List<T>> = Mutable::access(delst.front.clone());
    while !(lst.clone().is_empty()) {
        Dangerous::listSetFirst(lst.clone(), inMapFunc((lst.clone()).get(1)?, inArg1.clone())?)?;
        let metamodelica::List::Cons { head: _, tail: __pa0 } = &(lst.clone()) else { bail!("pattern mismatch") };
        lst = __pa0.clone();
    }
    Ok(())
}

pub fn new<T: Clone + 'static>(first: T) -> MutableList<T> {
    let mut delst: MutableList<T>;
    let mut lst: Arc<metamodelica::List<T>> = list![first.clone()];
    delst = MutableList { length: Mutable::create(1), front: Mutable::create(lst.clone()), back: Mutable::create(lst.clone()) };
    delst
}

pub fn pop_front<T: Clone + 'static>(delst: MutableList<T>) -> Result<T> {
    let mut elt: T;
    let mut length: i32 = Mutable::access(delst.length.clone());
    let mut lst: Arc<metamodelica::List<T>>;
    let true = (length.clone() > 0) else { bail!("pattern mismatch") };
    Mutable::update(delst.length.clone(), length.clone() - 1);
    let metamodelica::List::Cons { head: __pa0, tail: __pa1 } = &(Mutable::access(delst.front.clone())) else { bail!("pattern mismatch") };
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

pub fn push_back<T: Clone + 'static>(delst: MutableList<T>, elt: T) -> () {
    let mut length: i32 = Mutable::access(delst.length.clone());
    let mut lst: Arc<metamodelica::List<T>>;
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

pub fn push_front<T: Clone + 'static>(delst: MutableList<T>, elt: T) -> () {
    let mut length: i32 = Mutable::access(delst.length.clone());
    let mut lst: Arc<metamodelica::List<T>>;
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

pub fn push_list_back<T: Clone + 'static>(delst: MutableList<T>, lst: Arc<metamodelica::List<T>>) -> () {
    let mut length: i32 = Mutable::access(delst.length.clone());
    let mut lstLength: i32;
    let mut tail: Arc<metamodelica::List<T>>;
    let mut tmp: Arc<metamodelica::List<T>>;
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
    for l in &*listRest(lst.clone()).unwrap() {
        tmp = list![l.clone()];
        Dangerous::listSetRest(tail.clone(), tmp.clone()).unwrap();
        tail = tmp.clone();
    }
    Mutable::update(delst.back.clone(), tail.clone());
    ()
}

pub fn push_list_front<T: Clone + 'static>(delst: MutableList<T>, lst: Arc<metamodelica::List<T>>) -> Result<()> {
    let mut length: i32 = Mutable::access(delst.length.clone());
    let mut lstLength: i32;
    let mut work: Arc<metamodelica::List<T>>;
    let mut oldHead: Arc<metamodelica::List<T>>;
    let mut tmp: Arc<metamodelica::List<T>>;
    let mut head: Arc<metamodelica::List<T>>;
    let mut t: T;
    lstLength = (lst.clone().len() as i32);
    if lstLength.clone() == 0 {
        return Ok(());
    }
    Mutable::update(delst.length.clone(), length.clone() + lstLength.clone());
    let metamodelica::List::Cons { head: __pa0, tail: __pa1 } = &(lst.clone()) else { bail!("pattern mismatch") };
    t = __pa0.clone();
    tmp = __pa1.clone();
    head = list![t.clone()];
    oldHead = Mutable::access(delst.front.clone());
    Mutable::update(delst.front.clone(), head.clone());
    for l in &*tmp.clone() {
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

pub fn toListAndClear<T: Clone + 'static>(delst: MutableList<T>, prependToList: Arc<metamodelica::List<T>>) -> Arc<metamodelica::List<T>> {
    let mut res: Arc<metamodelica::List<T>>;
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

pub fn toListNoCopyNoClear<T: Clone + 'static>(delst: MutableList<T>) -> Arc<metamodelica::List<T>> {
    let mut res: Arc<metamodelica::List<T>>;
    res = Mutable::access(delst.front.clone());
    res
}

