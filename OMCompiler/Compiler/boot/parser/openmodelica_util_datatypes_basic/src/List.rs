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

use crate::Array;
use crate::DoubleEnded;
use crate::GCExt;

pub fn create<T: Clone + 'static>(mut inElement: T) -> Arc<metamodelica::List<T>> {
    let mut outList: Arc<metamodelica::List<T>> = list![inElement.clone()];
    outList
}

pub fn fill<T: Clone + 'static>(mut inElement: T, mut inCount: i32) -> Arc<metamodelica::List<T>> {
    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut i: i32 = 0;
    while i.clone() < inCount.clone() {
        outList = cons(inElement.clone(), outList.clone());
        i = i.clone() + 1;
    }
    outList
}

pub fn repeat<T: Clone + 'static>(mut inElement: Arc<metamodelica::List<T>>, mut inCount: i32) -> Arc<metamodelica::List<T>> {
    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut i: i32 = 0;
    while i.clone() < inCount.clone() {
        outList = listAppend(inElement.clone(), outList.clone());
        i = i.clone() + 1;
    }
    outList
}

pub fn intRange(mut inStop: i32) -> Arc<metamodelica::List<i32>> {
    let mut outRange: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut i: i32 = inStop.clone();
    while i.clone() > 0 {
        outRange = cons(i.clone(), outRange.clone());
        i = i.clone() - 1;
    }
    outRange
}

pub fn intRange2(mut inStart: i32, mut inStop: i32) -> Arc<metamodelica::List<i32>> {
    let mut outRange: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut i: i32 = inStop.clone();
    if inStart.clone() < inStop.clone() {
        while i.clone() >= inStart.clone() {
            outRange = cons(i.clone(), outRange.clone());
            i = i.clone() - 1;
        }
    } else {
        while i.clone() <= inStart.clone() {
            outRange = cons(i.clone(), outRange.clone());
            i = i.clone() + 1;
        }
    }
    outRange
}

pub fn intRange3(mut inStart: i32, mut inStep: i32, mut inStop: i32) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outRange: Arc<metamodelica::List<i32>> = metamodelica::nil();
    if inStep.clone() == 0 {
        bail!("fail");
    }
    outRange = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (({let __s=inStart.clone(); let __e=inStop.clone(); let __step=inStep.clone(); if __step>0 {__s..=__e} else {__e..=__s}}).step_by((if inStep.clone()>0 {inStep.clone()} else {-(inStep.clone())}) as usize)).into_iter() {
            let __x = i.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(outRange)
}

pub fn fromOption<T: Clone + 'static>(mut inElement: Option<T>) -> Arc<metamodelica::List<T>> {
    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    outList = (match inElement.clone() {
        Some(mut e) => {
            list![e.clone()]
        },
        _ => {
            metamodelica::nil()
        },
    });
    outList
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isEqual<T: Clone + 'static + PartialEq>(mut inList1: Arc<metamodelica::List<T>>, mut inList2: Arc<metamodelica::List<T>>, mut inEqualLength: bool) -> bool {
    let mut outIsEqual: bool = false;
    outIsEqual = (::match_deref::match_deref! { match &((inList1.clone(), inList2.clone(), inEqualLength.clone())) {
        (Deref @ metamodelica::List::Cons { head: e1, tail: rest1 }, Deref @ metamodelica::List::Cons { head: e2, tail: rest2 }, _) if (e1.clone() == e2.clone()) => {
            isEqual(rest1.clone(), rest2.clone(), inEqualLength.clone())
        },
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _) => {
            true
        },
        (Deref @ metamodelica::List::Nil, _, false) => {
            true
        },
        (_, Deref @ metamodelica::List::Nil, false) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsEqual
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isEqualOnTrue<T1: Clone + 'static, T2: Clone + 'static>(mut inList1: Arc<metamodelica::List<T1>>, mut inList2: Arc<metamodelica::List<T2>>, mut inCompFunc: Arc<dyn ::std::ops::Fn(T1, T2) -> Result<bool> + 'static>) -> bool {
    pub type CompFunc<T1: Clone + 'static, T2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1, T2) -> Result<bool> + 'static>;

    let mut outIsEqual: bool = false;
    outIsEqual = (::match_deref::match_deref! { match &((inList1.clone(), inList2.clone())) {
        (Deref @ metamodelica::List::Cons { head: e1, tail: rest1 }, Deref @ metamodelica::List::Cons { head: e2, tail: rest2 }) if (inCompFunc(e1.clone(), e2.clone()).unwrap()) => {
            isEqualOnTrue(rest1.clone(), rest2.clone(), inCompFunc.clone())
        },
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsEqual
}

pub fn allEqual<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inCompFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>) -> Result<bool> {
    pub type CompFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outAllEqual: bool = true;
    let mut e1: T;
    let mut rest: Arc<metamodelica::List<T>> = metamodelica::nil();
    if !(inList.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inList.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e1 = __pa0.clone();
        rest = __pa1.clone();
        for mut e in &*rest.clone() {
            let mut e = e.clone();
            if !(inCompFunc(e1.clone(), e.clone())?) {
                outAllEqual = false;
                return Ok(outAllEqual);
            }
        }
    }
    Ok(outAllEqual)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn compareLength<T1: Clone + 'static, T2: Clone + 'static>(mut list1: Arc<metamodelica::List<T1>>, mut list2: Arc<metamodelica::List<T2>>) -> Result<i32> {
    let mut res: i32 = 0;
    res = (::match_deref::match_deref! { match &((list1.clone(), list2.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => 0,
        (Deref @ metamodelica::List::Nil, _) => -1,
        (_, Deref @ metamodelica::List::Nil) => 1,
        _ => compareLength(listRest(list1.clone())?, listRest(list2.clone())?)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub fn compare<T1: Clone + 'static, T2: Clone + 'static>(mut list1: Arc<metamodelica::List<T1>>, mut list2: Arc<metamodelica::List<T2>>, mut compareFn: Arc<dyn ::std::ops::Fn(T1, T2) -> Result<i32> + 'static>) -> Result<i32> {
    pub type CompFunc<T1: Clone + 'static, T2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1, T2) -> Result<i32> + 'static>;

    let mut res: i32 = 0;
    let mut e2: T2;
    let mut rest_e2: Arc<metamodelica::List<T2>> = metamodelica::nil();
    res = compareLength(list1.clone(), list2.clone())?;
    if res.clone() != 0 {
        return Ok(res);
    }
    rest_e2 = list2.clone();
    for mut e1 in &*list1.clone() {
        let mut e1 = e1.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_e2.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e2 = __pa0.clone();
        rest_e2 = __pa1.clone();
        res = compareFn(e1.clone(), e2.clone())?;
        if res.clone() != 0 {
            return Ok(res);
        }
    }
    Ok(res)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isPrefixOnTrue<T1: Clone + 'static, T2: Clone + 'static>(mut inList1: Arc<metamodelica::List<T1>>, mut inList2: Arc<metamodelica::List<T2>>, mut inCompFunc: Arc<dyn ::std::ops::Fn(T1, T2) -> Result<bool> + 'static>) -> bool {
    pub type CompFunc<T1: Clone + 'static, T2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1, T2) -> Result<bool> + 'static>;

    let mut outIsPrefix: bool = false;
    outIsPrefix = (::match_deref::match_deref! { match &((inList1.clone(), inList2.clone())) {
        (Deref @ metamodelica::List::Cons { head: e1, tail: rest1 }, Deref @ metamodelica::List::Cons { head: e2, tail: rest2 }) if (inCompFunc(e1.clone(), e2.clone()).unwrap()) => {
            isPrefixOnTrue(rest1.clone(), rest2.clone(), inCompFunc.clone())
        },
        (Deref @ metamodelica::List::Nil, _) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsPrefix
}

pub fn consr<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inElement: T) -> Arc<metamodelica::List<T>> {
    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    outList = cons(inElement.clone(), inList.clone());
    outList
}

pub fn consOnTrue<T: Clone + 'static>(mut inCondition: bool, mut inElement: T, mut inList: Arc<metamodelica::List<T>>) -> Arc<metamodelica::List<T>> {
    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    outList = if (inCondition.clone()) {cons(inElement.clone(), inList.clone())} else {inList.clone()};
    outList
}

pub fn consOption<T: Clone + 'static>(mut inElement: Option<T>, mut inList: Arc<metamodelica::List<T>>) -> Arc<metamodelica::List<T>> {
    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    outList = (match inElement.clone() {
        Some(mut e) => {
            cons(e.clone(), inList.clone())
        },
        _ => {
            inList.clone()
        },
    });
    outList
}

pub fn consN<T: Clone + 'static>(mut size: i32, mut inElement: T, mut inList: Arc<metamodelica::List<T>>) -> Arc<metamodelica::List<T>> {
    let mut inList: Arc<metamodelica::List<T>> = inList;
    for mut i in 1..=size.clone() {
        inList = cons(inElement.clone(), inList.clone());
    }
    inList
}

pub fn append_reverse<T: Clone + 'static>(mut inList1: Arc<metamodelica::List<T>>, mut inList2: Arc<metamodelica::List<T>>) -> Arc<metamodelica::List<T>> {
    let mut outList: Arc<metamodelica::List<T>> = inList2.clone();
    for mut e in &*inList1.clone() {
        let mut e = e.clone();
        outList = cons(e.clone(), outList.clone());
    }
    outList
}

pub fn appendElt<T: Clone + 'static>(mut inElement: T, mut inList: Arc<metamodelica::List<T>>) -> Arc<metamodelica::List<T>> {
    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    outList = listAppend(inList.clone(), list![inElement.clone()]);
    outList
}

pub fn appendLastList<T: Clone + 'static>(mut inListList: Arc<metamodelica::List<Arc<metamodelica::List<T>>>>, mut inList: Arc<metamodelica::List<T>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<T>>>>> {
    let mut outListList: Arc<metamodelica::List<Arc<metamodelica::List<T>>>> = metamodelica::nil();
    outListList = ({
        let mut ol: Arc<metamodelica::List<Arc<metamodelica::List<T>>>> = metamodelica::nil();
        (::match_deref::match_deref! { match &((inListList.clone(), inList.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            list![inList.clone()]
        },
        (Deref @ metamodelica::List::Cons { head: l, tail: Deref @ metamodelica::List::Nil }, _) => {
            list![listAppend(l.clone(), inList.clone())]
        },
        (Deref @ metamodelica::List::Cons { head: l, tail: ll }, _) => {
            let mut l = (*l).clone();
            let mut ll = (*ll).clone();
            while !(ll.clone().is_empty()) {
                ol = cons(l.clone(), ol.clone());
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ll.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                l = __pa0.clone();
                ll = __pa1.clone();
            }
            ol = cons(listAppend(l.clone(), inList.clone()), ol.clone());
            ol = ol.clone().reverse();
            ol.clone()
        },
        _ => bail!("match: no arm matched"),
    } })
    });
    Ok(outListList)
}

pub fn insert<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inN: i32, mut inElement: T) -> Result<Arc<metamodelica::List<T>>> {
    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut lst1: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut lst2: Arc<metamodelica::List<T>> = metamodelica::nil();
    let true = (inN.clone() > 0) else { bail!("pattern mismatch") };
    (lst1, lst2) = splitr(inList.clone(), inN.clone() - 1)?;
    outList = append_reverse(lst1.clone(), cons(inElement.clone(), lst2.clone()));
    Ok(outList)
}

pub fn insertListSorted<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inList2: Arc<metamodelica::List<T>>, mut inCompFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>) -> Result<Arc<metamodelica::List<T>>> {
    pub type CompareFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    outList = insertListSorted1(inList.clone(), inList2.clone(), inCompFunc.clone(), metamodelica::nil())?.reverse();
    Ok(outList)
}

fn insertListSorted1<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inList2: Arc<metamodelica::List<T>>, mut inCompFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>, mut inResultList: Arc<metamodelica::List<T>>) -> Result<Arc<metamodelica::List<T>>> {
    pub type CompareFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outResultList: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut listRest: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut listRest2: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut tmpResultList: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut listHead: T;
    let mut listHead2: T;
    let mut elem: T;
    outResultList = (::match_deref::match_deref! { match &((inList.clone(), inList2.clone(), inCompFunc.clone(), inResultList.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _, _) => inResultList.clone(),
        (Deref @ metamodelica::List::Nil, _, _, _) => append_reverse(inList2.clone(), inResultList.clone()),
        (_, Deref @ metamodelica::List::Nil, _, _) => append_reverse(inList.clone(), inResultList.clone()),
        (Deref @ metamodelica::List::Cons { head: listHead, tail: listRest }, Deref @ metamodelica::List::Cons { head: listHead2, tail: listRest2 }, _, _) => {
            if inCompFunc(listHead.clone(), listHead2.clone())? {
                tmpResultList = cons(listHead.clone(), inResultList.clone());
                tmpResultList = insertListSorted1(listRest.clone(), inList2.clone(), inCompFunc.clone(), tmpResultList.clone())?;
            } else {
                tmpResultList = cons(listHead2.clone(), inResultList.clone());
                tmpResultList = insertListSorted1(inList.clone(), listRest2.clone(), inCompFunc.clone(), tmpResultList.clone())?;
            }
            tmpResultList.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outResultList)
}

pub fn set<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inN: i32, mut inElement: T) -> Result<Arc<metamodelica::List<T>>> {
    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut lst1: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut lst2: Arc<metamodelica::List<T>> = metamodelica::nil();
    let true = (inN.clone() > 0) else { bail!("pattern mismatch") };
    (lst1, lst2) = splitr(inList.clone(), inN.clone() - 1)?;
    lst2 = restOrEmpty(lst2.clone())?;
    outList = append_reverse(lst1.clone(), cons(inElement.clone(), lst2.clone()));
    Ok(outList)
}

pub fn firstOrEmpty<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>) -> Arc<metamodelica::List<T>> {
    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    outList = (::match_deref::match_deref! { match &(inList.clone()) {
        Deref @ metamodelica::List::Cons { head: e, tail: _ } => {
            list![e.clone()]
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outList
}

pub fn second<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>) -> Result<T> {
    let mut outSecond: T;
    let __pa0 = ::match_deref::match_deref! { match &(inList.clone()) {
        Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outSecond = __pa0.clone();
    Ok(outSecond)
}

pub fn last<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>) -> Result<T> {
    let mut outLast: T;
    let mut rest: Arc<metamodelica::List<T>> = metamodelica::nil();
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inList.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outLast = __pa0.clone();
    rest = __pa1.clone();
    for mut e in &*rest.clone() {
        let mut e = e.clone();
        outLast = e.clone();
    }
    Ok(outLast)
}

pub fn lastListOrEmpty<T: Clone + 'static>(mut inListList: Arc<metamodelica::List<Arc<metamodelica::List<T>>>>) -> Arc<metamodelica::List<T>> {
    let mut outLastList: Arc<metamodelica::List<T>> = metamodelica::nil();
    for mut e in &*inListList.clone() {
        let mut e = e.clone();
        outLastList = e.clone();
    }
    outLastList
}

pub fn lastN<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inN: i32) -> Result<Arc<metamodelica::List<T>>> {
    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut len: i32 = 0;
    let true = (inN.clone() >= 0) else { bail!("pattern mismatch") };
    len = (inList.clone().len() as i32);
    outList = stripN(inList.clone(), len.clone() - inN.clone())?;
    Ok(outList)
}

pub fn trimToLength<T: Clone + 'static>(mut lst: Arc<metamodelica::List<T>>, mut n: i32) -> Result<Arc<metamodelica::List<T>>> {
    let mut lst: Arc<metamodelica::List<T>> = lst;
    let mut len: i32 = 0;
    len = (lst.clone().len() as i32);
    for mut i in 1..=len.clone() - n.clone() {
        lst = listRest(lst.clone())?;
    }
    Ok(lst)
}

pub fn restOrEmpty<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>) -> Result<Arc<metamodelica::List<T>>> {
    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    outList = if (inList.clone().is_empty()) {inList.clone()} else {listRest(inList.clone())?};
    Ok(outList)
}

pub fn getIndexFirst<T: Clone + 'static>(mut index: i32, mut inList: Arc<metamodelica::List<T>>) -> T {
    let mut element: T;
    element = (inList.clone()).get(index.clone()).unwrap();
    element
}

pub fn getAtIndexLst<T: Clone + 'static>(mut lst: Arc<metamodelica::List<T>>, mut positions: Arc<metamodelica::List<i32>>, mut zeroBased: bool) -> Arc<metamodelica::List<T>> {
    let mut olst: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut arr: metamodelica::Array<T> = metamodelica::arrayFromVec(lst.clone().into_iter().cloned().collect());
    let mut shift: i32 = if (zeroBased.clone()) {1} else {0};
    olst = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut pos in (positions.clone()).into_iter().cloned() {
            let __x = arr.borrow()[(pos.clone() + shift.clone()-1) as usize].clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    olst
}

pub fn firstN<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut N: i32) -> Result<Arc<metamodelica::List<T>>> {
    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    outList = firstN_reverse(inList.clone(), N.clone())?;
    outList = outList.clone().reverse();
    Ok(outList)
}

pub fn firstN_reverse<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut N: i32) -> Result<Arc<metamodelica::List<T>>> {
    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut e: T;
    let mut rest: Arc<metamodelica::List<T>> = metamodelica::nil();
    let true = (N.clone() >= 0) else { bail!("pattern mismatch") };
    rest = inList.clone();
    for mut i in 1..=N.clone() {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa0.clone();
        rest = __pa1.clone();
        outList = cons(e.clone(), outList.clone());
    }
    Ok(outList)
}

pub fn stripLast<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>) -> Result<Arc<metamodelica::List<T>>> {
    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    if inList.clone().is_empty() {
        outList = metamodelica::nil();
    } else {
        let __pa0 = ::match_deref::match_deref! { match &(inList.clone().reverse()) {
            Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        outList = __pa0.clone();
        outList = outList.clone().reverse();
    }
    Ok(outList)
}

pub fn stripN<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inN: i32) -> Result<Arc<metamodelica::List<T>>> {
    let mut outList: Arc<metamodelica::List<T>> = inList.clone();
    let true = (inN.clone() >= 0) else { bail!("pattern mismatch") };
    for mut i in 1..=inN.clone() {
        let __pa0 = ::match_deref::match_deref! { match &(outList.clone()) {
            Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        outList = __pa0.clone();
    }
    Ok(outList)
}

pub fn heapSortIntList(mut lst: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<i32>> {
    let mut lst: Arc<metamodelica::List<i32>> = lst;
    lst = (::match_deref::match_deref! { match &(lst.clone()) {
        Deref @ metamodelica::List::Nil => lst.clone(),
        Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } => lst.clone(),
        _ => Arc::new(Array::heapSort(metamodelica::arrayFromVec(lst.clone().into_iter().cloned().collect())).borrow().iter().cloned().collect::<metamodelica::List<_>>()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    lst
}

pub fn sort<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inCompFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>) -> Result<Arc<metamodelica::List<T>>> {
    pub type CompareFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut rest: Arc<metamodelica::List<T>> = inList.clone();
    let mut e1: T;
    let mut e2: T;
    let mut left: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut right: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut middle: i32 = 0;
    if !(rest.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e1 = __pa0.clone();
        rest = __pa1.clone();
        if rest.clone().is_empty() {
            outList = inList.clone();
        } else {
            let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e2 = __pa2.clone();
            rest = __pa3.clone();
            if rest.clone().is_empty() {
                outList = if (inCompFunc(e2.clone(), e1.clone())?) {inList.clone()} else {list![e2.clone(), e1.clone()]};
            } else {
                middle = intDiv((inList.clone().len() as i32), 2);
                (left, right) = split(inList.clone(), middle.clone())?;
                left = sort(left.clone(), inCompFunc.clone())?;
                right = sort(right.clone(), inCompFunc.clone())?;
                outList = merge(left.clone(), right.clone(), inCompFunc.clone(), metamodelica::nil())?;
            }
        }
    }
    Ok(outList)
}

pub fn sortedDuplicates<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inCompFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>) -> Result<Arc<metamodelica::List<T>>> {
    pub type CompareFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outDuplicates: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut e: T;
    let mut rest: Arc<metamodelica::List<T>> = inList.clone();
    while !(rest.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa0.clone();
        rest = __pa1.clone();
        if !(rest.clone().is_empty()) && inCompFunc(e.clone(), listHead(rest.clone())?)? {
            outDuplicates = cons(e.clone(), outDuplicates.clone());
        }
    }
    outDuplicates = outDuplicates.clone().reverse();
    Ok(outDuplicates)
}

pub fn sortedListAllUnique<T: Clone + 'static>(mut lst: Arc<metamodelica::List<T>>, mut compareFn: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>) -> Result<bool> {
    pub type CompareFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut allUnique: bool = false;
    let mut e: T;
    let mut rest: Arc<metamodelica::List<T>> = lst.clone();
    while !(rest.clone().is_empty()) {
        rest = (::match_deref::match_deref! { match &(rest.clone()) {
        Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: e1, tail: rest @ Deref @ metamodelica::List::Cons { head: e2, tail: _ } } => {
            if compareFn(e1.clone(), e2.clone())? {
                return Ok(allUnique);
            }
            rest.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    }
    allUnique = true;
    Ok(allUnique)
}

pub fn sortedUnique<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inCompFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>) -> Result<Arc<metamodelica::List<T>>> {
    pub type CompareFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outUniqueElements: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut e: T;
    let mut rest: Arc<metamodelica::List<T>> = inList.clone();
    while !(rest.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa0.clone();
        rest = __pa1.clone();
        if rest.clone().is_empty() || !(inCompFunc(e.clone(), listHead(rest.clone())?)?) {
            outUniqueElements = cons(e.clone(), outUniqueElements.clone());
        }
    }
    outUniqueElements = outUniqueElements.clone().reverse();
    Ok(outUniqueElements)
}

pub fn sortedUniqueAndDuplicates<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inCompFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>) -> Result<(Arc<metamodelica::List<T>>, Arc<metamodelica::List<T>>)> {
    pub type CompareFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outUniqueElements: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut outDuplicateElements: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut e: T;
    let mut rest: Arc<metamodelica::List<T>> = inList.clone();
    while !(rest.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa0.clone();
        rest = __pa1.clone();
        if !(rest.clone().is_empty()) && inCompFunc(e.clone(), listHead(rest.clone())?)? {
            outDuplicateElements = cons(e.clone(), outDuplicateElements.clone());
        } else {
            outUniqueElements = cons(e.clone(), outUniqueElements.clone());
        }
    }
    outUniqueElements = outUniqueElements.clone().reverse();
    outDuplicateElements = outDuplicateElements.clone().reverse();
    Ok((outUniqueElements, outDuplicateElements))
}

pub fn sortedUniqueOnlyDuplicates<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inCompFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>) -> Result<Arc<metamodelica::List<T>>> {
    pub type CompareFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outDuplicateElements: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut e: T;
    let mut rest: Arc<metamodelica::List<T>> = inList.clone();
    while !(rest.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa0.clone();
        rest = __pa1.clone();
        if !(rest.clone().is_empty()) && inCompFunc(e.clone(), listHead(rest.clone())?)? {
            outDuplicateElements = cons(e.clone(), outDuplicateElements.clone());
        }
    }
    outDuplicateElements = outDuplicateElements.clone().reverse();
    Ok(outDuplicateElements)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn merge<T: Clone + 'static>(mut inLeft: Arc<metamodelica::List<T>>, mut inRight: Arc<metamodelica::List<T>>, mut inCompFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>, mut acc: Arc<metamodelica::List<T>>) -> Result<Arc<metamodelica::List<T>>> {
    pub type CompareFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    outList = (::match_deref::match_deref! { match &((inLeft.clone(), inRight.clone())) {
        (Deref @ metamodelica::List::Cons { head: l, tail: l_rest }, Deref @ metamodelica::List::Cons { head: r, tail: r_rest }) => {
            let mut el: T;
            let mut l_rest = (*l_rest).clone();
            let mut r_rest = (*r_rest).clone();
            if inCompFunc(r.clone(), l.clone())? {
                r_rest = inRight.clone();
                el = l.clone();
            } else {
                l_rest = inLeft.clone();
                el = r.clone();
            }
            merge(l_rest.clone(), r_rest.clone(), inCompFunc.clone(), cons(el.clone(), acc.clone()))?
        },
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            acc.clone().reverse()
        },
        (Deref @ metamodelica::List::Nil, _) => {
            append_reverse(acc.clone(), inRight.clone())
        },
        (_, Deref @ metamodelica::List::Nil) => {
            append_reverse(acc.clone(), inLeft.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outList)
}

pub fn mergeSorted<T: Clone + 'static>(mut inList1: Arc<metamodelica::List<T>>, mut inList2: Arc<metamodelica::List<T>>, mut inCompFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>) -> Result<Arc<metamodelica::List<T>>> {
    pub type CompFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut l1: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut l2: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut e1: T;
    let mut e2: T;
    l1 = inList1.clone();
    l2 = inList2.clone();
    while !(l1.clone().is_empty()) && !(l2.clone().is_empty()) {
        let __pa0 = ::match_deref::match_deref! { match &(l1.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        e1 = __pa0.clone();
        let __pa1 = ::match_deref::match_deref! { match &(l2.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa1, tail: _ } => __pa1.clone(),
            _ => bail!("pattern mismatch"),
        } };
        e2 = __pa1.clone();
        if inCompFunc(e1.clone(), e2.clone())? {
            outList = cons(e1.clone(), outList.clone());
            let __pa2 = ::match_deref::match_deref! { match &(l1.clone()) {
                Deref @ metamodelica::List::Cons { head: _, tail: __pa2 } => __pa2.clone(),
                _ => bail!("pattern mismatch"),
            } };
            l1 = __pa2.clone();
        } else {
            outList = cons(e2.clone(), outList.clone());
            let __pa3 = ::match_deref::match_deref! { match &(l2.clone()) {
                Deref @ metamodelica::List::Cons { head: _, tail: __pa3 } => __pa3.clone(),
                _ => bail!("pattern mismatch"),
            } };
            l2 = __pa3.clone();
        }
    }
    l1 = if (l1.clone().is_empty()) {l2.clone()} else {l1.clone()};
    outList = append_reverse(outList.clone(), l1.clone());
    Ok(outList)
}

pub fn countingSort(mut inList: Arc<metamodelica::List<i32>>, mut N: i32) -> Arc<metamodelica::List<i32>> {
    let mut outSorted: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut a1: metamodelica::Array<i32>;
    if !(hasSeveralElements(inList.clone())) {
        outSorted = inList.clone();
        return outSorted;
    }
    a1 = arrayCreate(N.clone(), 0);
    for mut v in &*inList.clone() {
        let mut v = v.clone();
        {
            let __cell0 = intAdd(a1.borrow()[(v.clone()-1) as usize].clone(), 1);
            a1.clone().borrow_mut()[(v.clone()-1) as usize] = __cell0;
        }
    }
    for mut v in (1..=N.clone()).rev() {
        let __range1 = 1..=a1.borrow()[(v.clone()-1) as usize].clone();
        for mut c in __range1 {
            outSorted = cons(v.clone(), outSorted.clone());
        }
    }
    GCExt::free(a1.clone());
    outSorted
}

pub fn unique<T: Clone + 'static + PartialEq>(mut inList: Arc<metamodelica::List<T>>) -> Arc<metamodelica::List<T>> {
    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        if !(listMember(e.clone(), outList.clone())) {
            outList = cons(e.clone(), outList.clone());
        }
    }
    outList = outList.clone().reverse();
    outList
}

pub fn uniqueIntN(mut inList: Arc<metamodelica::List<i32>>, mut inN: i32) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut arr: metamodelica::Array<bool>;
    arr = arrayCreate(inN.clone(), true);
    for mut i in &*inList.clone() {
        let mut i = i.clone();
        if arr.clone().borrow()[(i.clone()-1) as usize].clone() {
            outList = cons(i.clone(), outList.clone());
        }
        {let _arr = arr.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = false; _arr};
    }
    GCExt::free(arr.clone());
    Ok(outList)
}

pub fn uniqueOnTrue<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inCompFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>) -> Arc<metamodelica::List<T>> {
    pub type CompFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        if !(isMemberOnTrue(e.clone(), outList.clone(), inCompFunc.clone())) {
            outList = cons(e.clone(), outList.clone());
        }
    }
    outList = outList.clone().reverse();
    outList
}

pub fn split<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inPosition: i32) -> Result<(Arc<metamodelica::List<T>>, Arc<metamodelica::List<T>>)> {
    let mut outList1: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut outList2: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut pos: i32 = 0;
    let mut l1: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut l2: Arc<metamodelica::List<T>> = inList.clone();
    let mut e: T;
    let true = (inPosition.clone() >= 0) else { bail!("pattern mismatch") };
    pos = inPosition.clone();
    for mut i in 1..=pos.clone() {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(l2.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa0.clone();
        l2 = __pa1.clone();
        l1 = cons(e.clone(), l1.clone());
    }
    outList1 = l1.clone().reverse();
    outList2 = l2.clone();
    Ok((outList1, outList2))
}

pub fn splitr<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inPosition: i32) -> Result<(Arc<metamodelica::List<T>>, Arc<metamodelica::List<T>>)> {
    let mut outList1: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut outList2: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut pos: i32 = 0;
    let mut l1: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut l2: Arc<metamodelica::List<T>> = inList.clone();
    let mut e: T;
    let true = (inPosition.clone() >= 0) else { bail!("pattern mismatch") };
    pos = inPosition.clone();
    for mut i in 1..=pos.clone() {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(l2.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa0.clone();
        l2 = __pa1.clone();
        l1 = cons(e.clone(), l1.clone());
    }
    outList1 = l1.clone();
    outList2 = l2.clone();
    Ok((outList1, outList2))
}

pub fn splitOnTrue<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFunc: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> (Arc<metamodelica::List<T>>, Arc<metamodelica::List<T>>) {
    pub type PredicateFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outTrueList: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut outFalseList: Arc<metamodelica::List<T>> = metamodelica::nil();
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        if inFunc(e.clone()).unwrap() {
            outTrueList = cons(e.clone(), outTrueList.clone());
        } else {
            outFalseList = cons(e.clone(), outFalseList.clone());
        }
    }
    outTrueList = outTrueList.clone().reverse();
    outFalseList = outFalseList.clone().reverse();
    (outTrueList, outFalseList)
}

pub fn split1OnTrue<T: Clone + 'static, ArgT1: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFunc: Arc<dyn ::std::ops::Fn(T, ArgT1) -> Result<bool> + 'static>, mut inArg1: ArgT1) -> (Arc<metamodelica::List<T>>, Arc<metamodelica::List<T>>) {
    pub type PredicateFunc<T: Clone + 'static, ArgT1: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1) -> Result<bool> + 'static>;

    let mut outTrueList: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut outFalseList: Arc<metamodelica::List<T>> = metamodelica::nil();
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        if inFunc(e.clone(), inArg1.clone()).unwrap() {
            outTrueList = cons(e.clone(), outTrueList.clone());
        } else {
            outFalseList = cons(e.clone(), outFalseList.clone());
        }
    }
    outTrueList = outTrueList.clone().reverse();
    outFalseList = outFalseList.clone().reverse();
    (outTrueList, outFalseList)
}

pub fn split2OnTrue<T: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFunc: Arc<dyn ::std::ops::Fn(T, ArgT1, ArgT2) -> Result<bool> + 'static>, mut inArg1: ArgT1, mut inArg2: ArgT2) -> (Arc<metamodelica::List<T>>, Arc<metamodelica::List<T>>) {
    pub type PredicateFunc<T: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1, ArgT2) -> Result<bool> + 'static>;

    let mut outTrueList: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut outFalseList: Arc<metamodelica::List<T>> = metamodelica::nil();
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        if inFunc(e.clone(), inArg1.clone(), inArg2.clone()).unwrap() {
            outTrueList = cons(e.clone(), outTrueList.clone());
        } else {
            outFalseList = cons(e.clone(), outFalseList.clone());
        }
    }
    outTrueList = outTrueList.clone().reverse();
    outFalseList = outFalseList.clone().reverse();
    (outTrueList, outFalseList)
}

pub fn splitOnFirstMatch<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFunc: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> Result<(Arc<metamodelica::List<T>>, Arc<metamodelica::List<T>>)> {
    pub type CompFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outList1: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut outList2: Arc<metamodelica::List<T>> = inList.clone();
    let mut e: T;
    while !(outList2.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(outList2.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa0.clone();
        outList2 = __pa1.clone();
        if inFunc(e.clone())? {
            outList2 = cons(e.clone(), outList2.clone());
            break;
        }
        outList1 = cons(e.clone(), outList1.clone());
    }
    outList1 = outList1.clone().reverse();
    Ok((outList1, outList2))
}

pub fn splitLast<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>) -> Result<(T, Arc<metamodelica::List<T>>)> {
    let mut outLast: T;
    let mut outRest: Arc<metamodelica::List<T>> = metamodelica::nil();
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inList.clone().reverse()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outLast = __pa0.clone();
    outRest = __pa1.clone();
    outRest = outRest.clone().reverse();
    Ok((outLast, outRest))
}

pub fn splitEqualParts<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inParts: i32) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<T>>>>> {
    let mut outParts: Arc<metamodelica::List<Arc<metamodelica::List<T>>>> = metamodelica::nil();
    let mut length: i32 = 0;
    if inParts.clone() == 0 {
        outParts = metamodelica::nil();
    } else {
        length = (inList.clone().len() as i32);
        let 0 = (intMod(length.clone(), inParts.clone())) else { bail!("pattern mismatch") };
        outParts = partition(inList.clone(), intDiv(length.clone(), inParts.clone()))?;
    }
    Ok(outParts)
}

pub fn splitOnBoolList<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inBools: Arc<metamodelica::List<bool>>) -> Result<(Arc<metamodelica::List<T>>, Arc<metamodelica::List<T>>)> {
    let mut outTrueList: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut outFalseList: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut e: T;
    let mut rest_e: Arc<metamodelica::List<T>> = inList.clone();
    let mut b: bool = false;
    let mut rest_b: Arc<metamodelica::List<bool>> = inBools.clone();
    while !(rest_e.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_e.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa0.clone();
        rest_e = __pa1.clone();
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest_b.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        b = __pa2.clone();
        rest_b = __pa3.clone();
        if b.clone() {
            outTrueList = cons(e.clone(), outTrueList.clone());
        } else if true /* isPresent not implemented in Rust */ {
            outFalseList = cons(e.clone(), outFalseList.clone());
        }
    }
    outTrueList = outTrueList.clone().reverse();
    outFalseList = outFalseList.clone().reverse();
    Ok((outTrueList, outFalseList))
}

pub fn partition<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inPartitionLength: i32) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<T>>>>> {
    let mut outPartitions: Arc<metamodelica::List<Arc<metamodelica::List<T>>>> = metamodelica::nil();
    let mut lst: Arc<metamodelica::List<T>> = inList.clone();
    let mut part: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut length: i32 = 0;
    let true = (inPartitionLength.clone() > 0) else { bail!("pattern mismatch") };
    if inList.clone().is_empty() {
        return Ok(outPartitions);
    }
    length = (inList.clone().len() as i32);
    if inPartitionLength.clone() >= length.clone() {
        outPartitions = list![inList.clone()];
        return Ok(outPartitions);
    }
    for mut i in 1..=length.clone() / inPartitionLength.clone() {
        (part, lst) = split(lst.clone(), inPartitionLength.clone())?;
        outPartitions = cons(part.clone(), outPartitions.clone());
    }
    if !(lst.clone().is_empty()) {
        outPartitions = cons(lst.clone(), outPartitions.clone());
    }
    outPartitions = outPartitions.clone().reverse();
    Ok(outPartitions)
}

pub fn balancedPartition<T: Clone + 'static>(mut lst: Arc<metamodelica::List<T>>, mut maxLength: i32) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<T>>>>> {
    let mut outPartitions: Arc<metamodelica::List<Arc<metamodelica::List<T>>>> = metamodelica::nil();
    let mut length: i32 = 0;
    let mut n: i32 = 0;
    let true = (maxLength.clone() > 0) else { bail!("pattern mismatch") };
    if lst.clone().is_empty() {
        outPartitions = metamodelica::nil();
        return Ok(outPartitions);
    }
    length = (lst.clone().len() as i32);
    n = intDiv(length.clone() - 1, maxLength.clone()) + 1;
    outPartitions = partition(lst.clone(), intDiv(length.clone() - 1, n.clone()) + 1)?;
    Ok(outPartitions)
}

pub fn sublist<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inOffset: i32, mut inLength: i32) -> Result<Arc<metamodelica::List<T>>> {
    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut e: T;
    let mut rest: Arc<metamodelica::List<T>> = inList.clone();
    let mut res: Arc<metamodelica::List<T>> = metamodelica::nil();
    let true = (inOffset.clone() > 0) else { bail!("pattern mismatch") };
    let true = (inLength.clone() >= 0) else { bail!("pattern mismatch") };
    for mut i in 2..=inOffset.clone() {
        let __pa0 = ::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        rest = __pa0.clone();
    }
    for mut i in 1..=inLength.clone() {
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa1, tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa1.clone();
        rest = __pa2.clone();
        outList = cons(e.clone(), outList.clone());
    }
    outList = outList.clone().reverse();
    Ok(outList)
}

pub fn transposeList<T: Clone + 'static>(mut inList: Arc<metamodelica::List<Arc<metamodelica::List<T>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<T>>>>> {
    let mut outList: Arc<metamodelica::List<Arc<metamodelica::List<T>>>> = metamodelica::nil();
    let mut arr: metamodelica::Array<metamodelica::Array<T>>;
    let mut arr_row: metamodelica::Array<T>;
    let mut new_row: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut c_len: i32 = 0;
    let mut r_len: i32 = 0;
    if inList.clone().is_empty() {
        return Ok(outList);
    }
    arr = metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut lst in (inList.clone()).into_iter().cloned() {
            let __x = metamodelica::arrayFromVec(lst.clone().into_iter().cloned().collect());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect());
    c_len = (arr.clone().borrow().len() as i32);
    r_len = (arr.clone().borrow()[(1-1) as usize].clone().borrow().len() as i32);
    for mut i in (1..=r_len.clone()).rev() {
        new_row = metamodelica::nil();
        for mut j in (1..=c_len.clone()).rev() {
            new_row = cons(arr.clone().borrow()[(j.clone()-1) as usize].clone().borrow()[(i.clone()-1) as usize].clone(), new_row.clone());
        }
        outList = cons(new_row.clone(), outList.clone());
    }
    Ok(outList)
}

pub fn listArrayReverse<T: Clone + 'static>(mut inLst: Arc<metamodelica::List<T>>) -> Result<metamodelica::Array<T>> {
    let mut outArr: metamodelica::Array<T>;
    let mut len: i32 = 0;
    let mut defaultValue: T;
    if inLst.clone().is_empty() {
        outArr = metamodelica::arrayFromVec(inLst.clone().into_iter().cloned().collect());
        return Ok(outArr);
    }
    len = (inLst.clone().len() as i32);
    let __pa0 = ::match_deref::match_deref! { match &(inLst.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    defaultValue = __pa0.clone();
    outArr = metamodelica::arrayCreate(len.clone(), defaultValue.clone());
    for mut e in &*inLst.clone() {
        let mut e = e.clone();
        unsafe { metamodelica::Dangerous::arrayInitSlot(outArr.clone(), len.clone(), e.clone()) };
        len = len.clone() - 1;
    }
    Ok(outArr)
}

pub fn setEqualOnTrue<T: Clone + 'static>(mut inList1: Arc<metamodelica::List<T>>, mut inList2: Arc<metamodelica::List<T>>, mut inCompFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>) -> bool {
    pub type CompFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outIsEqual: bool = false;
    let mut lst: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut lst_size: i32 = 0;
    lst = intersectionOnTrue(inList1.clone(), inList2.clone(), inCompFunc.clone());
    lst_size = (lst.clone().len() as i32);
    outIsEqual = intEq(lst_size.clone(), (inList1.clone().len() as i32)) && intEq(lst_size.clone(), (inList2.clone().len() as i32));
    outIsEqual
}

fn addPos(mut inList: Arc<metamodelica::List<i32>>, mut inArray: metamodelica::Array<i32>, mut inIndex: i32) -> Result<metamodelica::Array<i32>> {
    let mut outArray: metamodelica::Array<i32>;
    for mut i in &*inList.clone() {
        let mut i = i.clone();
        let _ = {let _arr = inArray.clone(); let _val = intAdd(inArray.clone().borrow()[(i.clone()-1) as usize].clone(), inIndex.clone()); _arr.borrow_mut()[(i.clone()-1) as usize] = _val; _arr};
    }
    outArray = inArray.clone();
    Ok(outArray)
}

pub fn intersectionOnTrue<T: Clone + 'static>(mut inList1: Arc<metamodelica::List<T>>, mut inList2: Arc<metamodelica::List<T>>, mut inCompFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>) -> Arc<metamodelica::List<T>> {
    pub type CompFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outIntersection: Arc<metamodelica::List<T>> = metamodelica::nil();
    for mut e in &*inList1.clone() {
        let mut e = e.clone();
        if isMemberOnTrue(e.clone(), inList2.clone(), inCompFunc.clone()) {
            outIntersection = cons(e.clone(), outIntersection.clone());
        }
    }
    outIntersection = outIntersection.clone().reverse();
    outIntersection
}

pub fn intersection1OnTrue<T: Clone + 'static>(mut inList1: Arc<metamodelica::List<T>>, mut inList2: Arc<metamodelica::List<T>>, mut inCompFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>) -> Result<(Arc<metamodelica::List<T>>, Arc<metamodelica::List<T>>, Arc<metamodelica::List<T>>)> {
    pub type CompFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outIntersection: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut outList1Rest: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut outList2Rest: Arc<metamodelica::List<T>> = inList2.clone();
    let mut oe: Option<T> = None;
    let mut lst1: Arc<metamodelica::List<T>> = inList1.clone();
    let mut lst2: Arc<metamodelica::List<T>> = inList2.clone();
    if inList1.clone().is_empty() {
        return Ok((outIntersection, outList1Rest, outList2Rest));
    }
    if inList2.clone().is_empty() {
        outList1Rest = inList1.clone();
        return Ok((outIntersection, outList1Rest, outList2Rest));
    }
    while !(lst1.clone().is_empty() || lst2.clone().is_empty()) {
        if !(inCompFunc(listHead(lst1.clone())?, listHead(lst2.clone())?)?) {
            break;
        }
        outIntersection = cons(listHead(lst1.clone())?, outIntersection.clone());
        lst1 = listRest(lst1.clone())?;
        lst2 = listRest(lst2.clone())?;
    }
    for mut e in &*lst1.clone() {
        let mut e = e.clone();
        if isMemberOnTrue(e.clone(), inList2.clone(), inCompFunc.clone()) {
            outIntersection = cons(e.clone(), outIntersection.clone());
        } else if true /* isPresent not implemented in Rust */ {
            outList1Rest = cons(e.clone(), outList1Rest.clone());
        }
    }
    outIntersection = outIntersection.clone().reverse();
    outList1Rest = if (true /* isPresent not implemented in Rust */) {outList1Rest.clone().reverse()} else {metamodelica::nil()};
    outList2Rest = if (true /* isPresent not implemented in Rust */) {setDifferenceOnTrue(inList2.clone(), outIntersection.clone(), inCompFunc.clone())?} else {metamodelica::nil()};
    Ok((outIntersection, outList1Rest, outList2Rest))
}

pub fn setDifferenceIntN(mut inList1: Arc<metamodelica::List<i32>>, mut inList2: Arc<metamodelica::List<i32>>, mut inN: i32) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outDifference: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut a: metamodelica::Array<i32>;
    if inN.clone() > 0 {
        a = arrayCreate(inN.clone(), 0);
        a = addPos(inList1.clone(), a.clone(), 1)?;
        a = addPos(inList2.clone(), a.clone(), 1)?;
        for mut i in (1..=inN.clone()).rev() {
            if a.clone().borrow()[(i.clone()-1) as usize].clone() == 1 {
                outDifference = cons(i.clone(), outDifference.clone());
            }
        }
        GCExt::free(a.clone());
    }
    Ok(outDifference)
}

pub fn setDifferenceOnTrue<T: Clone + 'static>(mut inList1: Arc<metamodelica::List<T>>, mut inList2: Arc<metamodelica::List<T>>, mut inCompFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>) -> Result<Arc<metamodelica::List<T>>> {
    pub type CompFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outDifference: Arc<metamodelica::List<T>> = inList1.clone();
    if inList1.clone().is_empty() {
        return Ok(outDifference);
    }
    for mut e in &*inList2.clone() {
        let mut e = e.clone();
        (outDifference, _) = deleteMemberOnTrue(e.clone(), outDifference.clone(), inCompFunc.clone())?;
    }
    Ok(outDifference)
}

pub fn setDifference<T: Clone + 'static + PartialEq>(mut inList1: Arc<metamodelica::List<T>>, mut inList2: Arc<metamodelica::List<T>>) -> Result<Arc<metamodelica::List<T>>> {
    let mut outDifference: Arc<metamodelica::List<T>> = inList1.clone();
    if inList1.clone().is_empty() {
        return Ok(outDifference);
    }
    for mut e in &*inList2.clone() {
        let mut e = e.clone();
        (outDifference, _) = deleteMemberOnTrue(e.clone(), outDifference.clone(), std::sync::Arc::new(fnptr!(valueEq, _, _)))?;
    }
    Ok(outDifference)
}

pub fn unionIntN(mut inList1: Arc<metamodelica::List<i32>>, mut inList2: Arc<metamodelica::List<i32>>, mut inN: i32) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outUnion: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut a: metamodelica::Array<i32>;
    if inN.clone() > 0 {
        a = arrayCreate(inN.clone(), 0);
        a = addPos(inList1.clone(), a.clone(), 1)?;
        a = addPos(inList2.clone(), a.clone(), 1)?;
        for mut i in (1..=inN.clone()).rev() {
            if a.clone().borrow()[(i.clone()-1) as usize].clone() > 0 {
                outUnion = cons(i.clone(), outUnion.clone());
            }
        }
        GCExt::free(a.clone());
    }
    Ok(outUnion)
}

pub fn unionElt<T: Clone + 'static + PartialEq>(mut inElement: T, mut inList: Arc<metamodelica::List<T>>) -> Arc<metamodelica::List<T>> {
    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    outList = consOnTrue(!(listMember(inElement.clone(), inList.clone())), inElement.clone(), inList.clone());
    outList
}

pub fn unionEltOnTrue<T: Clone + 'static>(mut inElement: T, mut inList: Arc<metamodelica::List<T>>, mut inCompFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>) -> Arc<metamodelica::List<T>> {
    pub type CompFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    outList = consOnTrue(!(isMemberOnTrue(inElement.clone(), inList.clone(), inCompFunc.clone())), inElement.clone(), inList.clone());
    outList
}

pub fn union<T: Clone + 'static + PartialEq>(mut inList1: Arc<metamodelica::List<T>>, mut inList2: Arc<metamodelica::List<T>>) -> Arc<metamodelica::List<T>> {
    let mut outUnion: Arc<metamodelica::List<T>> = metamodelica::nil();
    for mut e in &*inList1.clone() {
        let mut e = e.clone();
        outUnion = unionElt(e.clone(), outUnion.clone());
    }
    for mut e in &*inList2.clone() {
        let mut e = e.clone();
        outUnion = unionElt(e.clone(), outUnion.clone());
    }
    outUnion = outUnion.clone().reverse();
    outUnion
}

pub fn unionOnTrue<T: Clone + 'static>(mut inList1: Arc<metamodelica::List<T>>, mut inList2: Arc<metamodelica::List<T>>, mut inCompFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>) -> Arc<metamodelica::List<T>> {
    pub type CompFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outUnion: Arc<metamodelica::List<T>> = metamodelica::nil();
    for mut e in &*inList1.clone() {
        let mut e = e.clone();
        outUnion = unionEltOnTrue(e.clone(), outUnion.clone(), inCompFunc.clone());
    }
    for mut e in &*inList2.clone() {
        let mut e = e.clone();
        outUnion = unionEltOnTrue(e.clone(), outUnion.clone(), inCompFunc.clone());
    }
    outUnion = outUnion.clone().reverse();
    outUnion
}

pub fn unionAppendListOnTrue<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inUnion: Arc<metamodelica::List<T>>, mut inCompFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>) -> Arc<metamodelica::List<T>> {
    pub type CompFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outUnion: Arc<metamodelica::List<T>> = metamodelica::nil();
    outUnion = fold(inList.clone(), Arc::new({ let __pe_b2: Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static> = inCompFunc.clone(); move |__pe_a0, __pe_a1| Ok(unionEltOnTrue(__pe_a0, __pe_a1, __pe_b2.clone())) }), inUnion.clone());
    outUnion
}

pub fn unionList<T: Clone + 'static + PartialEq>(mut inList: Arc<metamodelica::List<Arc<metamodelica::List<T>>>>) -> Result<Arc<metamodelica::List<T>>> {
    let mut outUnion: Arc<metamodelica::List<T>> = metamodelica::nil();
    outUnion = if (inList.clone().is_empty()) {metamodelica::nil()} else {reduce(inList.clone(), std::sync::Arc::new(fnptr!(union, _, _)))?};
    Ok(outUnion)
}

pub fn unionOnTrueList<T: Clone + 'static>(mut inList: Arc<metamodelica::List<Arc<metamodelica::List<T>>>>, mut inCompFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>) -> Result<Arc<metamodelica::List<T>>> {
    pub type CompFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut outUnion: Arc<metamodelica::List<T>> = metamodelica::nil();
    outUnion = if (inList.clone().is_empty()) {metamodelica::nil()} else {reduce(inList.clone(), Arc::new({ let __pe_b2: Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static> = inCompFunc.clone(); move |__pe_a0, __pe_a1| Ok(unionOnTrue(__pe_a0, __pe_a1, __pe_b2.clone())) }))?};
    Ok(outUnion)
}

pub fn map<TI: Clone + 'static, TO: Clone + 'static>(mut inList: Arc<metamodelica::List<TI>>, mut inFunc: Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>) -> Arc<metamodelica::List<TO>> {
    pub type MapFunc<TI: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>;

    let mut outList: Arc<metamodelica::List<TO>> = metamodelica::nil();
    outList = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut e in (inList.clone()).into_iter().cloned() {
            let __x = inFunc(e.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outList
}

pub fn mapArray<TI: Clone + 'static, TO: Clone + 'static>(mut inArray: metamodelica::Array<TI>, mut inFunc: Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>) -> Arc<metamodelica::List<TO>> {
    pub type MapFunc<TI: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>;

    let mut outList: Arc<metamodelica::List<TO>> = metamodelica::nil();
    outList = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut e in (inArray.clone()).borrow().iter() {
            let __x = inFunc(e.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outList
}

pub fn mapCheckReferenceEq<TI: Clone + 'static + PartialEq>(mut inList: Arc<metamodelica::List<TI>>, mut inFunc: Arc<dyn ::std::ops::Fn(TI) -> Result<TI> + 'static>) -> Arc<metamodelica::List<TI>> {
    pub type MapFunc<TI: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<TI> + 'static>;

    let mut outList: Arc<metamodelica::List<TI>> = metamodelica::nil();
    let mut allEq: bool = true;
    let mut delst: DoubleEnded::MutableList<TI>;
    let mut n: i32 = 0;
    let mut e1: TI;
    let mut savedElt: TI;
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        e1 = inFunc(e.clone()).unwrap();
        if !(referenceEq(&e.clone(),&e1.clone())) {
            savedElt = e1.clone();
            delst = DoubleEnded::empty(e1.clone());
            for mut elt in &*inList.clone() {
                let mut elt = elt.clone();
                if n.clone() < 0 {
                    e1 = inFunc(elt.clone()).unwrap();
                } else {
                    e1 = if (n.clone() == 0) {savedElt.clone()} else {elt.clone()};
                }
                DoubleEnded::push_back(delst.clone(), e1.clone());
                n = n.clone() - 1;
            }
            outList = DoubleEnded::toListAndClear(delst.clone(), metamodelica::nil());
            return outList;
        }
        n = n.clone() + 1;
    }
    outList = inList.clone();
    outList
}

pub fn mapReverse<TI: Clone + 'static, TO: Clone + 'static>(mut inList: Arc<metamodelica::List<TI>>, mut inFunc: Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>) -> Arc<metamodelica::List<TO>> {
    pub type MapFunc<TI: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>;

    let mut outList: Arc<metamodelica::List<TO>> = metamodelica::nil();
    outList = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut e in (inList.clone()).into_iter().cloned() {
            let __x = inFunc(e.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc
    };
    outList
}

pub fn map_2<TI: Clone + 'static, TO1: Clone + 'static, TO2: Clone + 'static>(mut inList: Arc<metamodelica::List<TI>>, mut inFunc: Arc<dyn ::std::ops::Fn(TI) -> Result<(TO1, TO2)> + 'static>) -> (Arc<metamodelica::List<TO1>>, Arc<metamodelica::List<TO2>>) {
    pub type MapFunc<TI: Clone + 'static, TO1: Clone + 'static, TO2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<(TO1, TO2)> + 'static>;

    let mut outList1: Arc<metamodelica::List<TO1>> = metamodelica::nil();
    let mut outList2: Arc<metamodelica::List<TO2>> = metamodelica::nil();
    let mut e1: TO1;
    let mut e2: TO2;
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        (e1, e2) = inFunc(e.clone()).unwrap();
        outList1 = cons(e1.clone(), outList1.clone());
        if true /* isPresent not implemented in Rust */ {
            outList2 = cons(e2.clone(), outList2.clone());
        }
    }
    outList1 = outList1.clone().reverse();
    if true /* isPresent not implemented in Rust */ {
        outList2 = outList2.clone().reverse();
    }
    (outList1, outList2)
}

pub fn map_3<TI: Clone + 'static, TO1: Clone + 'static, TO2: Clone + 'static, TO3: Clone + 'static>(mut inList: Arc<metamodelica::List<TI>>, mut inFunc: Arc<dyn ::std::ops::Fn(TI) -> Result<(TO1, TO2, TO3)> + 'static>) -> (Arc<metamodelica::List<TO1>>, Arc<metamodelica::List<TO2>>, Arc<metamodelica::List<TO3>>) {
    pub type MapFunc<TI: Clone + 'static, TO1: Clone + 'static, TO2: Clone + 'static, TO3: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<(TO1, TO2, TO3)> + 'static>;

    let mut outList1: Arc<metamodelica::List<TO1>> = metamodelica::nil();
    let mut outList2: Arc<metamodelica::List<TO2>> = metamodelica::nil();
    let mut outList3: Arc<metamodelica::List<TO3>> = metamodelica::nil();
    let mut e1: TO1;
    let mut e2: TO2;
    let mut e3: TO3;
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        (e1, e2, e3) = inFunc(e.clone()).unwrap();
        outList1 = cons(e1.clone(), outList1.clone());
        if true /* isPresent not implemented in Rust */ {
            outList2 = cons(e2.clone(), outList2.clone());
        }
        if true /* isPresent not implemented in Rust */ {
            outList3 = cons(e3.clone(), outList3.clone());
        }
    }
    outList1 = outList1.clone().reverse();
    if true /* isPresent not implemented in Rust */ {
        outList2 = outList2.clone().reverse();
    }
    if true /* isPresent not implemented in Rust */ {
        outList3 = outList3.clone().reverse();
    }
    (outList1, outList2, outList3)
}

pub fn mapOption<TI: Clone + 'static, TO: Clone + 'static>(mut inList: Arc<metamodelica::List<Option<TI>>>, mut inFunc: Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>) -> Result<Arc<metamodelica::List<TO>>> {
    pub type MapFunc<TI: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>;

    let mut outList: Arc<metamodelica::List<TO>> = metamodelica::nil();
    let mut ei: TI;
    let mut eo: TO;
    for mut oe in &*inList.clone() {
        let mut oe = oe.clone();
        if isSome(oe.clone()) {
            let Some(__pa0) = (oe.clone()) else { bail!("pattern mismatch") };
            ei = __pa0.clone();
            eo = inFunc(ei.clone())?;
            outList = cons(eo.clone(), outList.clone());
        }
    }
    outList = outList.clone().reverse();
    Ok(outList)
}

pub fn map1Option<TI: Clone + 'static, ArgT: Clone + 'static, TO: Clone + 'static>(mut inList: Arc<metamodelica::List<Option<TI>>>, mut inFunc: Arc<dyn ::std::ops::Fn(TI, ArgT) -> Result<TO> + 'static>, mut inArg1: ArgT) -> Result<Arc<metamodelica::List<TO>>> {
    pub type MapFunc<TI: Clone + 'static, ArgT: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT) -> Result<TO> + 'static>;

    let mut outList: Arc<metamodelica::List<TO>> = metamodelica::nil();
    let mut ei: TI;
    let mut eo: TO;
    for mut oe in &*inList.clone() {
        let mut oe = oe.clone();
        if isSome(oe.clone()) {
            let Some(__pa0) = (oe.clone()) else { bail!("pattern mismatch") };
            ei = __pa0.clone();
            eo = inFunc(ei.clone(), inArg1.clone())?;
            outList = cons(eo.clone(), outList.clone());
        }
    }
    outList = outList.clone().reverse();
    Ok(outList)
}

pub fn map2Option<TI: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, TO: Clone + 'static>(mut inList: Arc<metamodelica::List<Option<TI>>>, mut inFunc: Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2) -> Result<TO> + 'static>, mut inArg1: ArgT1, mut inArg2: ArgT2) -> Result<Arc<metamodelica::List<TO>>> {
    pub type MapFunc<TI: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2) -> Result<TO> + 'static>;

    let mut outList: Arc<metamodelica::List<TO>> = metamodelica::nil();
    let mut ei: TI;
    let mut eo: TO;
    for mut oe in &*inList.clone() {
        let mut oe = oe.clone();
        if isSome(oe.clone()) {
            let Some(__pa0) = (oe.clone()) else { bail!("pattern mismatch") };
            ei = __pa0.clone();
            eo = inFunc(ei.clone(), inArg1.clone(), inArg2.clone())?;
            outList = cons(eo.clone(), outList.clone());
        }
    }
    outList = outList.clone().reverse();
    Ok(outList)
}

pub fn map_0<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFunc: Arc<dyn ::std::ops::Fn(T) -> Result<()> + 'static>) -> () {
    pub type MapFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<()> + 'static>;

    for mut e in &*inList.clone() {
        let mut e = e.clone();
        inFunc(e.clone()).unwrap();
    }
    ()
}

pub fn map1<TI: Clone + 'static, ArgT1: Clone + 'static, TO: Clone + 'static>(mut inList: Arc<metamodelica::List<TI>>, mut inMapFunc: Arc<dyn ::std::ops::Fn(TI, ArgT1) -> Result<TO> + 'static>, mut inArg1: ArgT1) -> Arc<metamodelica::List<TO>> {
    pub type MapFunc<TI: Clone + 'static, ArgT1: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1) -> Result<TO> + 'static>;

    let mut outList: Arc<metamodelica::List<TO>> = metamodelica::nil();
    outList = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut e in (inList.clone()).into_iter().cloned() {
            let __x = inMapFunc(e.clone(), inArg1.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outList
}

pub fn map1r<TI: Clone + 'static, ArgT1: Clone + 'static, TO: Clone + 'static>(mut inList: Arc<metamodelica::List<TI>>, mut inFunc: Arc<dyn ::std::ops::Fn(ArgT1, TI) -> Result<TO> + 'static>, mut inArg1: ArgT1) -> Arc<metamodelica::List<TO>> {
    pub type MapFunc<ArgT1: Clone + 'static, TI: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(ArgT1, TI) -> Result<TO> + 'static>;

    let mut outList: Arc<metamodelica::List<TO>> = metamodelica::nil();
    outList = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut e in (inList.clone()).into_iter().cloned() {
            let __x = inFunc(inArg1.clone(), e.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outList
}

pub fn map1_0<TI: Clone + 'static, ArgT1: Clone + 'static>(mut inList: Arc<metamodelica::List<TI>>, mut inFunc: Arc<dyn ::std::ops::Fn(TI, ArgT1) -> Result<()> + 'static>, mut inArg1: ArgT1) -> () {
    pub type MapFunc<TI: Clone + 'static, ArgT1: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1) -> Result<()> + 'static>;

    for mut e in &*inList.clone() {
        let mut e = e.clone();
        inFunc(e.clone(), inArg1.clone()).unwrap();
    }
    ()
}

pub fn map1_2<TI: Clone + 'static, ArgT1: Clone + 'static, TO1: Clone + 'static, TO2: Clone + 'static>(mut inList: Arc<metamodelica::List<TI>>, mut inFunc: Arc<dyn ::std::ops::Fn(TI, ArgT1) -> Result<(TO1, TO2)> + 'static>, mut inArg1: ArgT1) -> (Arc<metamodelica::List<TO1>>, Arc<metamodelica::List<TO2>>) {
    pub type MapFunc<TI: Clone + 'static, ArgT1: Clone + 'static, TO1: Clone + 'static, TO2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1) -> Result<(TO1, TO2)> + 'static>;

    let mut outList1: Arc<metamodelica::List<TO1>> = metamodelica::nil();
    let mut outList2: Arc<metamodelica::List<TO2>> = metamodelica::nil();
    let mut e1: TO1;
    let mut e2: TO2;
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        (e1, e2) = inFunc(e.clone(), inArg1.clone()).unwrap();
        outList1 = cons(e1.clone(), outList1.clone());
        outList2 = cons(e2.clone(), outList2.clone());
    }
    outList1 = outList1.clone().reverse();
    outList2 = outList2.clone().reverse();
    (outList1, outList2)
}

pub fn map2<TI: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, TO: Clone + 'static>(mut inList: Arc<metamodelica::List<TI>>, mut inFunc: Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2) -> Result<TO> + 'static>, mut inArg1: ArgT1, mut inArg2: ArgT2) -> Arc<metamodelica::List<TO>> {
    pub type MapFunc<TI: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2) -> Result<TO> + 'static>;

    let mut outList: Arc<metamodelica::List<TO>> = metamodelica::nil();
    outList = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut e in (inList.clone()).into_iter().cloned() {
            let __x = inFunc(e.clone(), inArg1.clone(), inArg2.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outList
}

pub fn map2Reverse<TI: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, TO: Clone + 'static>(mut inList: Arc<metamodelica::List<TI>>, mut inFunc: Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2) -> Result<TO> + 'static>, mut inArg1: ArgT1, mut inArg2: ArgT2) -> Arc<metamodelica::List<TO>> {
    pub type MapFunc<TI: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2) -> Result<TO> + 'static>;

    let mut outList: Arc<metamodelica::List<TO>> = metamodelica::nil();
    outList = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut e in (inList.clone()).into_iter().cloned() {
            let __x = inFunc(e.clone(), inArg1.clone(), inArg2.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc
    };
    outList
}

pub fn map2_0<TI: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static>(mut inList: Arc<metamodelica::List<TI>>, mut inFunc: Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2) -> Result<()> + 'static>, mut inArg1: ArgT1, mut inArg2: ArgT2) -> () {
    pub type MapFunc<TI: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2) -> Result<()> + 'static>;

    for mut e in &*inList.clone() {
        let mut e = e.clone();
        inFunc(e.clone(), inArg1.clone(), inArg2.clone()).unwrap();
    }
    ()
}

pub fn map2_2<TI: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, TO1: Clone + 'static, TO2: Clone + 'static>(mut inList: Arc<metamodelica::List<TI>>, mut inFunc: Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2) -> Result<(TO1, TO2)> + 'static>, mut inArg1: ArgT1, mut inArg2: ArgT2) -> (Arc<metamodelica::List<TO1>>, Arc<metamodelica::List<TO2>>) {
    pub type MapFunc<TI: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, TO1: Clone + 'static, TO2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2) -> Result<(TO1, TO2)> + 'static>;

    let mut outList1: Arc<metamodelica::List<TO1>> = metamodelica::nil();
    let mut outList2: Arc<metamodelica::List<TO2>> = metamodelica::nil();
    let mut e1: TO1;
    let mut e2: TO2;
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        (e1, e2) = inFunc(e.clone(), inArg1.clone(), inArg2.clone()).unwrap();
        outList1 = cons(e1.clone(), outList1.clone());
        outList2 = cons(e2.clone(), outList2.clone());
    }
    outList1 = outList1.clone().reverse();
    outList2 = outList2.clone().reverse();
    (outList1, outList2)
}

pub fn map3<TI: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, ArgT3: Clone + 'static, TO: Clone + 'static>(mut inList: Arc<metamodelica::List<TI>>, mut inFunc: Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2, ArgT3) -> Result<TO> + 'static>, mut inArg1: ArgT1, mut inArg2: ArgT2, mut inArg3: ArgT3) -> Arc<metamodelica::List<TO>> {
    pub type MapFunc<TI: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, ArgT3: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2, ArgT3) -> Result<TO> + 'static>;

    let mut outList: Arc<metamodelica::List<TO>> = metamodelica::nil();
    outList = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut e in (inList.clone()).into_iter().cloned() {
            let __x = inFunc(e.clone(), inArg1.clone(), inArg2.clone(), inArg3.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outList
}

pub fn map4<TI: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, ArgT3: Clone + 'static, ArgT4: Clone + 'static, TO: Clone + 'static>(mut inList: Arc<metamodelica::List<TI>>, mut inFunc: Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2, ArgT3, ArgT4) -> Result<TO> + 'static>, mut inArg1: ArgT1, mut inArg2: ArgT2, mut inArg3: ArgT3, mut inArg4: ArgT4) -> Arc<metamodelica::List<TO>> {
    pub type MapFunc<TI: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, ArgT3: Clone + 'static, ArgT4: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2, ArgT3, ArgT4) -> Result<TO> + 'static>;

    let mut outList: Arc<metamodelica::List<TO>> = metamodelica::nil();
    outList = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut e in (inList.clone()).into_iter().cloned() {
            let __x = inFunc(e.clone(), inArg1.clone(), inArg2.clone(), inArg3.clone(), inArg4.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outList
}

pub fn map4_0<TI: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, ArgT3: Clone + 'static, ArgT4: Clone + 'static>(mut inList: Arc<metamodelica::List<TI>>, mut inFunc: Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2, ArgT3, ArgT4) -> Result<()> + 'static>, mut inArg1: ArgT1, mut inArg2: ArgT2, mut inArg3: ArgT3, mut inArg4: ArgT4) -> () {
    pub type MapFunc<TI: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, ArgT3: Clone + 'static, ArgT4: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2, ArgT3, ArgT4) -> Result<()> + 'static>;

    for mut e in &*inList.clone() {
        let mut e = e.clone();
        inFunc(e.clone(), inArg1.clone(), inArg2.clone(), inArg3.clone(), inArg4.clone()).unwrap();
    }
    ()
}

pub fn map5<TI: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, ArgT3: Clone + 'static, ArgT4: Clone + 'static, ArgT5: Clone + 'static, TO: Clone + 'static>(mut inList: Arc<metamodelica::List<TI>>, mut inFunc: Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2, ArgT3, ArgT4, ArgT5) -> Result<TO> + 'static>, mut inArg1: ArgT1, mut inArg2: ArgT2, mut inArg3: ArgT3, mut inArg4: ArgT4, mut inArg5: ArgT5) -> Arc<metamodelica::List<TO>> {
    pub type MapFunc<TI: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, ArgT3: Clone + 'static, ArgT4: Clone + 'static, ArgT5: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2, ArgT3, ArgT4, ArgT5) -> Result<TO> + 'static>;

    let mut outList: Arc<metamodelica::List<TO>> = metamodelica::nil();
    outList = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut e in (inList.clone()).into_iter().cloned() {
            let __x = inFunc(e.clone(), inArg1.clone(), inArg2.clone(), inArg3.clone(), inArg4.clone(), inArg5.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outList
}

pub fn map6<TI: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, ArgT3: Clone + 'static, ArgT4: Clone + 'static, ArgT5: Clone + 'static, ArgT6: Clone + 'static, TO: Clone + 'static>(mut inList: Arc<metamodelica::List<TI>>, mut inFunc: Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2, ArgT3, ArgT4, ArgT5, ArgT6) -> Result<TO> + 'static>, mut inArg1: ArgT1, mut inArg2: ArgT2, mut inArg3: ArgT3, mut inArg4: ArgT4, mut inArg5: ArgT5, mut inArg6: ArgT6) -> Arc<metamodelica::List<TO>> {
    pub type MapFunc<TI: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, ArgT3: Clone + 'static, ArgT4: Clone + 'static, ArgT5: Clone + 'static, ArgT6: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2, ArgT3, ArgT4, ArgT5, ArgT6) -> Result<TO> + 'static>;

    let mut outList: Arc<metamodelica::List<TO>> = metamodelica::nil();
    outList = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut e in (inList.clone()).into_iter().cloned() {
            let __x = inFunc(e.clone(), inArg1.clone(), inArg2.clone(), inArg3.clone(), inArg4.clone(), inArg5.clone(), inArg6.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outList
}

pub fn mapFlat<TI: Clone + 'static, TO: Clone + 'static>(mut inList: Arc<metamodelica::List<TI>>, mut inMapFunc: Arc<dyn ::std::ops::Fn(TI) -> Result<Arc<metamodelica::List<TO>>> + 'static>) -> Arc<metamodelica::List<TO>> {
    pub type MapFunc<TI: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<Arc<metamodelica::List<TO>>> + 'static>;

    let mut outList: Arc<metamodelica::List<TO>> = metamodelica::nil();
    outList = mapFlatReverse(inList.clone(), inMapFunc.clone()).reverse();
    outList
}

pub fn mapFlatReverse<TI: Clone + 'static, TO: Clone + 'static>(mut inList: Arc<metamodelica::List<TI>>, mut inMapFunc: Arc<dyn ::std::ops::Fn(TI) -> Result<Arc<metamodelica::List<TO>>> + 'static>) -> Arc<metamodelica::List<TO>> {
    pub type MapFunc<TI: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<Arc<metamodelica::List<TO>>> + 'static>;

    let mut outList: Arc<metamodelica::List<TO>> = metamodelica::nil();
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        outList = listAppend(inMapFunc(e.clone()).unwrap(), outList.clone());
    }
    outList
}

pub fn mapMap<TI: Clone + 'static, TO1: Clone + 'static, TO2: Clone + 'static>(mut inList: Arc<metamodelica::List<TI>>, mut inMapFunc1: Arc<dyn ::std::ops::Fn(TI) -> Result<TO1> + 'static>, mut inMapFunc2: Arc<dyn ::std::ops::Fn(TO1) -> Result<TO2> + 'static>) -> Arc<metamodelica::List<TO2>> {
    pub type MapFunc1<TI: Clone + 'static, TO1: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<TO1> + 'static>;

    pub type MapFunc2<TO1: Clone + 'static, TO2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TO1) -> Result<TO2> + 'static>;

    let mut outList: Arc<metamodelica::List<TO2>> = metamodelica::nil();
    outList = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut e in (inList.clone()).into_iter().cloned() {
            let __x = inMapFunc2(inMapFunc1(e.clone()).unwrap()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outList
}

pub fn foldAllValue<TI: Clone + 'static, ArgT1: Clone + 'static, TO: Clone + 'static + PartialEq>(mut inList: Arc<metamodelica::List<TI>>, mut inMapFunc: Arc<dyn ::std::ops::Fn(TI, ArgT1) -> Result<(TO, ArgT1)> + 'static>, mut inValue: TO, mut inArg1: ArgT1) -> Result<()> {
    pub type MapFunc<TI: Clone + 'static, ArgT1: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1) -> Result<(TO, ArgT1)> + 'static>;

    let mut arg: ArgT1 = inArg1.clone();
    let mut eo: TO;
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        (eo, arg) = inMapFunc(e.clone(), arg.clone())?;
        let true = (eo.clone() == inValue.clone()) else { bail!("pattern mismatch") };
    }
    Ok(())
}

pub fn applyAndFold<TI: Clone + 'static, TO: Clone + 'static, FT: Clone + 'static>(mut inList: Arc<metamodelica::List<TI>>, mut inFoldFunc: Arc<dyn ::std::ops::Fn(TO, FT) -> Result<FT> + 'static>, mut inApplyFunc: Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>, mut inFoldArg: FT) -> FT {
    pub type ApplyFunc<TI: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>;

    pub type FoldFunc<TO: Clone + 'static, FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TO, FT) -> Result<FT> + 'static>;

    let mut outResult: FT = inFoldArg.clone();
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        outResult = inFoldFunc(inApplyFunc(e.clone()).unwrap(), outResult.clone()).unwrap();
    }
    outResult
}

pub fn applyAndFold1<TI: Clone + 'static, TO: Clone + 'static, FT: Clone + 'static, ArgT1: Clone + 'static>(mut inList: Arc<metamodelica::List<TI>>, mut inFoldFunc: Arc<dyn ::std::ops::Fn(TO, FT) -> Result<FT> + 'static>, mut inApplyFunc: Arc<dyn ::std::ops::Fn(TI, ArgT1) -> Result<TO> + 'static>, mut inExtraArg: ArgT1, mut inFoldArg: FT) -> FT {
    pub type ApplyFunc<TI: Clone + 'static, ArgT1: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1) -> Result<TO> + 'static>;

    pub type FoldFunc<TO: Clone + 'static, FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TO, FT) -> Result<FT> + 'static>;

    let mut outResult: FT = inFoldArg.clone();
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        outResult = inFoldFunc(inApplyFunc(e.clone(), inExtraArg.clone()).unwrap(), outResult.clone()).unwrap();
    }
    outResult
}

pub fn mapMapBoolAnd<TI: Clone + 'static, TI2: Clone + 'static>(mut inList: Arc<metamodelica::List<TI>>, mut inFunc: Arc<dyn ::std::ops::Fn(TI) -> Result<TI2> + 'static>, mut inBFunc: Arc<dyn ::std::ops::Fn(TI2) -> Result<bool> + 'static>) -> bool {
    pub type MapBFunc<TI2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI2) -> Result<bool> + 'static>;

    pub type MapFunc<TI: Clone + 'static, TI2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<TI2> + 'static>;

    let mut res: bool = false;
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        if !(inBFunc(inFunc(e.clone()).unwrap()).unwrap()) {
            return res;
        }
    }
    res = true;
    res
}

pub fn mapList<TI: Clone + 'static, TO: Clone + 'static>(mut inListList: Arc<metamodelica::List<Arc<metamodelica::List<TI>>>>, mut inFunc: Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>) -> Arc<metamodelica::List<Arc<metamodelica::List<TO>>>> {
    pub type MapFunc<TI: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>;

    let mut outListList: Arc<metamodelica::List<Arc<metamodelica::List<TO>>>> = metamodelica::nil();
    outListList = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut lst in (inListList.clone()).into_iter().cloned() {
            let __x = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut e in (lst.clone()).into_iter().cloned() {
            let __x = inFunc(e.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outListList
}

pub fn mapListReverse<TI: Clone + 'static, TO: Clone + 'static>(mut inListList: Arc<metamodelica::List<Arc<metamodelica::List<TI>>>>, mut inFunc: Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>) -> Arc<metamodelica::List<Arc<metamodelica::List<TO>>>> {
    pub type MapFunc<TI: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>;

    let mut outListList: Arc<metamodelica::List<Arc<metamodelica::List<TO>>>> = metamodelica::nil();
    outListList = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut lst in (inListList.clone()).into_iter().cloned() {
            let __x = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut e in (lst.clone()).into_iter().cloned() {
            let __x = inFunc(e.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc
    };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outListList
}

pub fn map1List<TI: Clone + 'static, ArgT1: Clone + 'static, TO: Clone + 'static>(mut inListList: Arc<metamodelica::List<Arc<metamodelica::List<TI>>>>, mut inFunc: Arc<dyn ::std::ops::Fn(TI, ArgT1) -> Result<TO> + 'static>, mut inArg1: ArgT1) -> Arc<metamodelica::List<Arc<metamodelica::List<TO>>>> {
    pub type MapFunc<TI: Clone + 'static, ArgT1: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1) -> Result<TO> + 'static>;

    let mut outListList: Arc<metamodelica::List<Arc<metamodelica::List<TO>>>> = metamodelica::nil();
    outListList = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut lst in (inListList.clone()).into_iter().cloned() {
            let __x = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut e in (lst.clone()).into_iter().cloned() {
            let __x = inFunc(e.clone(), inArg1.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outListList
}

pub fn map2List<TI: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, TO: Clone + 'static>(mut inListList: Arc<metamodelica::List<Arc<metamodelica::List<TI>>>>, mut inFunc: Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2) -> Result<TO> + 'static>, mut inArg1: ArgT1, mut inArg2: ArgT2) -> Arc<metamodelica::List<Arc<metamodelica::List<TO>>>> {
    pub type MapFunc<TI: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2) -> Result<TO> + 'static>;

    let mut outListList: Arc<metamodelica::List<Arc<metamodelica::List<TO>>>> = metamodelica::nil();
    outListList = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut lst in (inListList.clone()).into_iter().cloned() {
            let __x = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut e in (lst.clone()).into_iter().cloned() {
            let __x = inFunc(e.clone(), inArg1.clone(), inArg2.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outListList
}

pub fn fold<T: Clone + 'static, FT: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFoldFunc: Arc<dyn ::std::ops::Fn(T, FT) -> Result<FT> + 'static>, mut inStartValue: FT) -> FT {
    pub type FoldFunc<T: Clone + 'static, FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, FT) -> Result<FT> + 'static>;

    let mut outResult: FT = inStartValue.clone();
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        outResult = inFoldFunc(e.clone(), outResult.clone()).unwrap();
    }
    outResult
}

pub fn foldr<T: Clone + 'static, FT: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFoldFunc: Arc<dyn ::std::ops::Fn(FT, T) -> Result<FT> + 'static>, mut inStartValue: FT) -> FT {
    pub type FoldFunc<FT: Clone + 'static, T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(FT, T) -> Result<FT> + 'static>;

    let mut outResult: FT = inStartValue.clone();
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        outResult = inFoldFunc(outResult.clone(), e.clone()).unwrap();
    }
    outResult
}

pub fn fold1<T: Clone + 'static, ArgT1: Clone + 'static, FT: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFoldFunc: Arc<dyn ::std::ops::Fn(T, ArgT1, FT) -> Result<FT> + 'static>, mut inExtraArg: ArgT1, mut inStartValue: FT) -> FT {
    pub type FoldFunc<T: Clone + 'static, ArgT1: Clone + 'static, FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1, FT) -> Result<FT> + 'static>;

    let mut outResult: FT = inStartValue.clone();
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        outResult = inFoldFunc(e.clone(), inExtraArg.clone(), outResult.clone()).unwrap();
    }
    outResult
}

pub fn fold1r<T: Clone + 'static, FT: Clone + 'static, ArgT1: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFoldFunc: Arc<dyn ::std::ops::Fn(FT, T, ArgT1) -> Result<FT> + 'static>, mut inExtraArg: ArgT1, mut inStartValue: FT) -> FT {
    pub type FoldFunc<FT: Clone + 'static, T: Clone + 'static, ArgT1: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(FT, T, ArgT1) -> Result<FT> + 'static>;

    let mut outResult: FT = inStartValue.clone();
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        outResult = inFoldFunc(outResult.clone(), e.clone(), inExtraArg.clone()).unwrap();
    }
    outResult
}

pub fn fold2<T: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, FT: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFoldFunc: Arc<dyn ::std::ops::Fn(T, ArgT1, ArgT2, FT) -> Result<FT> + 'static>, mut inExtraArg1: ArgT1, mut inExtraArg2: ArgT2, mut inStartValue: FT) -> FT {
    pub type FoldFunc<T: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1, ArgT2, FT) -> Result<FT> + 'static>;

    let mut outResult: FT = inStartValue.clone();
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        outResult = inFoldFunc(e.clone(), inExtraArg1.clone(), inExtraArg2.clone(), outResult.clone()).unwrap();
    }
    outResult
}

pub fn fold22<T: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, FT1: Clone + 'static, FT2: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFoldFunc: Arc<dyn ::std::ops::Fn(T, ArgT1, ArgT2, FT1, FT2) -> Result<(FT1, FT2)> + 'static>, mut inExtraArg1: ArgT1, mut inExtraArg2: ArgT2, mut inStartValue1: FT1, mut inStartValue2: FT2) -> (FT1, FT2) {
    pub type FoldFunc<T: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, FT1: Clone + 'static, FT2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1, ArgT2, FT1, FT2) -> Result<(FT1, FT2)> + 'static>;

    let mut outResult1: FT1 = inStartValue1.clone();
    let mut outResult2: FT2 = inStartValue2.clone();
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        (outResult1, outResult2) = inFoldFunc(e.clone(), inExtraArg1.clone(), inExtraArg2.clone(), outResult1.clone(), outResult2.clone()).unwrap();
    }
    (outResult1, outResult2)
}

pub fn foldList<T: Clone + 'static, FT: Clone + 'static>(mut inList: Arc<metamodelica::List<Arc<metamodelica::List<T>>>>, mut inFoldFunc: Arc<dyn ::std::ops::Fn(T, FT) -> Result<FT> + 'static>, mut inStartValue: FT) -> FT {
    pub type FoldFunc<T: Clone + 'static, FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, FT) -> Result<FT> + 'static>;

    let mut outResult: FT = inStartValue.clone();
    for mut lst in &*inList.clone() {
        let mut lst = lst.clone();
        for mut e in &*lst.clone() {
            let mut e = e.clone();
            outResult = inFoldFunc(e.clone(), outResult.clone()).unwrap();
        }
    }
    outResult
}

pub fn fold2r<T: Clone + 'static, FT: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFoldFunc: Arc<dyn ::std::ops::Fn(FT, T, ArgT1, ArgT2) -> Result<FT> + 'static>, mut inExtraArg1: ArgT1, mut inExtraArg2: ArgT2, mut inStartValue: FT) -> FT {
    pub type FoldFunc<FT: Clone + 'static, T: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(FT, T, ArgT1, ArgT2) -> Result<FT> + 'static>;

    let mut outResult: FT = inStartValue.clone();
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        outResult = inFoldFunc(outResult.clone(), e.clone(), inExtraArg1.clone(), inExtraArg2.clone()).unwrap();
    }
    outResult
}

pub fn fold3<T: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, ArgT3: Clone + 'static, FT: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFoldFunc: Arc<dyn ::std::ops::Fn(T, ArgT1, ArgT2, ArgT3, FT) -> Result<FT> + 'static>, mut inExtraArg1: ArgT1, mut inExtraArg2: ArgT2, mut inExtraArg3: ArgT3, mut inStartValue: FT) -> FT {
    pub type FoldFunc<T: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, ArgT3: Clone + 'static, FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1, ArgT2, ArgT3, FT) -> Result<FT> + 'static>;

    let mut outResult: FT = inStartValue.clone();
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        outResult = inFoldFunc(e.clone(), inExtraArg1.clone(), inExtraArg2.clone(), inExtraArg3.clone(), outResult.clone()).unwrap();
    }
    outResult
}

pub fn fold4<T: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, ArgT3: Clone + 'static, ArgT4: Clone + 'static, FT: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFoldFunc: Arc<dyn ::std::ops::Fn(T, ArgT1, ArgT2, ArgT3, ArgT4, FT) -> Result<FT> + 'static>, mut inExtraArg1: ArgT1, mut inExtraArg2: ArgT2, mut inExtraArg3: ArgT3, mut inExtraArg4: ArgT4, mut inStartValue: FT) -> FT {
    pub type FoldFunc<T: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, ArgT3: Clone + 'static, ArgT4: Clone + 'static, FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1, ArgT2, ArgT3, ArgT4, FT) -> Result<FT> + 'static>;

    let mut outResult: FT = inStartValue.clone();
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        outResult = inFoldFunc(e.clone(), inExtraArg1.clone(), inExtraArg2.clone(), inExtraArg3.clone(), inExtraArg4.clone(), outResult.clone()).unwrap();
    }
    outResult
}

pub fn fold20<T: Clone + 'static, FT1: Clone + 'static, FT2: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFoldFunc: Arc<dyn ::std::ops::Fn(T, FT1, FT2) -> Result<(FT1, FT2)> + 'static>, mut inStartValue1: FT1, mut inStartValue2: FT2) -> (FT1, FT2) {
    pub type FoldFunc<T: Clone + 'static, FT1: Clone + 'static, FT2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, FT1, FT2) -> Result<(FT1, FT2)> + 'static>;

    let mut outResult1: FT1 = inStartValue1.clone();
    let mut outResult2: FT2 = inStartValue2.clone();
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        (outResult1, outResult2) = inFoldFunc(e.clone(), outResult1.clone(), outResult2.clone()).unwrap();
    }
    (outResult1, outResult2)
}

pub fn fold21<T: Clone + 'static, ArgT1: Clone + 'static, FT1: Clone + 'static, FT2: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFoldFunc: Arc<dyn ::std::ops::Fn(T, ArgT1, FT1, FT2) -> Result<(FT1, FT2)> + 'static>, mut inExtraArg1: ArgT1, mut inStartValue1: FT1, mut inStartValue2: FT2) -> (FT1, FT2) {
    pub type FoldFunc<T: Clone + 'static, ArgT1: Clone + 'static, FT1: Clone + 'static, FT2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1, FT1, FT2) -> Result<(FT1, FT2)> + 'static>;

    let mut outResult1: FT1 = inStartValue1.clone();
    let mut outResult2: FT2 = inStartValue2.clone();
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        (outResult1, outResult2) = inFoldFunc(e.clone(), inExtraArg1.clone(), outResult1.clone(), outResult2.clone()).unwrap();
    }
    (outResult1, outResult2)
}

pub fn fold31<T: Clone + 'static, ArgT1: Clone + 'static, FT1: Clone + 'static, FT2: Clone + 'static, FT3: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFoldFunc: Arc<dyn ::std::ops::Fn(T, ArgT1, FT1, FT2, FT3) -> Result<(FT1, FT2, FT3)> + 'static>, mut inExtraArg1: ArgT1, mut inStartValue1: FT1, mut inStartValue2: FT2, mut inStartValue3: FT3) -> (FT1, FT2, FT3) {
    pub type FoldFunc<T: Clone + 'static, ArgT1: Clone + 'static, FT1: Clone + 'static, FT2: Clone + 'static, FT3: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1, FT1, FT2, FT3) -> Result<(FT1, FT2, FT3)> + 'static>;

    let mut outResult1: FT1 = inStartValue1.clone();
    let mut outResult2: FT2 = inStartValue2.clone();
    let mut outResult3: FT3 = inStartValue3.clone();
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        (outResult1, outResult2, outResult3) = inFoldFunc(e.clone(), inExtraArg1.clone(), outResult1.clone(), outResult2.clone(), outResult3.clone()).unwrap();
    }
    (outResult1, outResult2, outResult3)
}

pub fn mapFold<TI: Clone + 'static, FT: Clone + 'static, TO: Clone + 'static>(mut inList: Arc<metamodelica::List<TI>>, mut inFunc: Arc<dyn ::std::ops::Fn(TI, FT) -> Result<(TO, FT)> + 'static>, mut inArg: FT) -> (Arc<metamodelica::List<TO>>, FT) {
    pub type FuncType<TI: Clone + 'static, FT: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, FT) -> Result<(TO, FT)> + 'static>;

    let mut outList: Arc<metamodelica::List<TO>> = metamodelica::nil();
    let mut outArg: FT = inArg.clone();
    let mut res: TO;
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        (res, outArg) = inFunc(e.clone(), outArg.clone()).unwrap();
        outList = cons(res.clone(), outList.clone());
    }
    outList = outList.clone().reverse();
    (outList, outArg)
}

pub fn mapFold2<TI: Clone + 'static, FT1: Clone + 'static, FT2: Clone + 'static, TO: Clone + 'static>(mut inList: Arc<metamodelica::List<TI>>, mut inFunc: Arc<dyn ::std::ops::Fn(TI, FT1, FT2) -> Result<(TO, FT1, FT2)> + 'static>, mut inArg1: FT1, mut inArg2: FT2) -> (Arc<metamodelica::List<TO>>, FT1, FT2) {
    pub type FuncType<TI: Clone + 'static, FT1: Clone + 'static, FT2: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, FT1, FT2) -> Result<(TO, FT1, FT2)> + 'static>;

    let mut outList: Arc<metamodelica::List<TO>> = metamodelica::nil();
    let mut outArg1: FT1 = inArg1.clone();
    let mut outArg2: FT2 = inArg2.clone();
    let mut res: TO;
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        (res, outArg1, outArg2) = inFunc(e.clone(), outArg1.clone(), outArg2.clone()).unwrap();
        outList = cons(res.clone(), outList.clone());
    }
    outList = outList.clone().reverse();
    (outList, outArg1, outArg2)
}

pub fn mapFold3<TI: Clone + 'static, FT1: Clone + 'static, FT2: Clone + 'static, FT3: Clone + 'static, TO: Clone + 'static>(mut inList: Arc<metamodelica::List<TI>>, mut inFunc: Arc<dyn ::std::ops::Fn(TI, FT1, FT2, FT3) -> Result<(TO, FT1, FT2, FT3)> + 'static>, mut inArg1: FT1, mut inArg2: FT2, mut inArg3: FT3) -> (Arc<metamodelica::List<TO>>, FT1, FT2, FT3) {
    pub type FuncType<TI: Clone + 'static, FT1: Clone + 'static, FT2: Clone + 'static, FT3: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, FT1, FT2, FT3) -> Result<(TO, FT1, FT2, FT3)> + 'static>;

    let mut outList: Arc<metamodelica::List<TO>> = metamodelica::nil();
    let mut inArg1: FT1 = inArg1;
    let mut inArg2: FT2 = inArg2;
    let mut inArg3: FT3 = inArg3;
    let mut res: TO;
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        (res, inArg1, inArg2, inArg3) = inFunc(e.clone(), inArg1.clone(), inArg2.clone(), inArg3.clone()).unwrap();
        outList = cons(res.clone(), outList.clone());
    }
    outList = outList.clone().reverse();
    (outList, inArg1, inArg2, inArg3)
}

pub fn mapFold5<TI: Clone + 'static, FT1: Clone + 'static, FT2: Clone + 'static, FT3: Clone + 'static, FT4: Clone + 'static, FT5: Clone + 'static, TO: Clone + 'static>(mut inList: Arc<metamodelica::List<TI>>, mut inFunc: Arc<dyn ::std::ops::Fn(TI, FT1, FT2, FT3, FT4, FT5) -> Result<(TO, FT1, FT2, FT3, FT4, FT5)> + 'static>, mut inArg1: FT1, mut inArg2: FT2, mut inArg3: FT3, mut inArg4: FT4, mut inArg5: FT5) -> (Arc<metamodelica::List<TO>>, FT1, FT2, FT3, FT4, FT5) {
    pub type FuncType<TI: Clone + 'static, FT1: Clone + 'static, FT2: Clone + 'static, FT3: Clone + 'static, FT4: Clone + 'static, FT5: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, FT1, FT2, FT3, FT4, FT5) -> Result<(TO, FT1, FT2, FT3, FT4, FT5)> + 'static>;

    let mut outList: Arc<metamodelica::List<TO>> = metamodelica::nil();
    let mut inArg1: FT1 = inArg1;
    let mut inArg2: FT2 = inArg2;
    let mut inArg3: FT3 = inArg3;
    let mut inArg4: FT4 = inArg4;
    let mut inArg5: FT5 = inArg5;
    let mut res: TO;
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        (res, inArg1, inArg2, inArg3, inArg4, inArg5) = inFunc(e.clone(), inArg1.clone(), inArg2.clone(), inArg3.clone(), inArg4.clone(), inArg5.clone()).unwrap();
        outList = cons(res.clone(), outList.clone());
    }
    outList = outList.clone().reverse();
    (outList, inArg1, inArg2, inArg3, inArg4, inArg5)
}

pub fn map1Fold<TI: Clone + 'static, ArgT1: Clone + 'static, FT: Clone + 'static, TO: Clone + 'static>(mut inList: Arc<metamodelica::List<TI>>, mut inFunc: Arc<dyn ::std::ops::Fn(TI, ArgT1, FT) -> Result<(TO, FT)> + 'static>, mut inConstArg: ArgT1, mut inArg: FT) -> (Arc<metamodelica::List<TO>>, FT) {
    pub type FuncType<TI: Clone + 'static, ArgT1: Clone + 'static, FT: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1, FT) -> Result<(TO, FT)> + 'static>;

    let mut outList: Arc<metamodelica::List<TO>> = metamodelica::nil();
    let mut outArg: FT = inArg.clone();
    let mut res: TO;
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        (res, outArg) = inFunc(e.clone(), inConstArg.clone(), outArg.clone()).unwrap();
        outList = cons(res.clone(), outList.clone());
    }
    outList = outList.clone().reverse();
    (outList, outArg)
}

pub fn map2Fold<TI: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, FT: Clone + 'static, TO: Clone + 'static>(mut inList: Arc<metamodelica::List<TI>>, mut inFunc: Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2, FT) -> Result<(TO, FT)> + 'static>, mut inConstArg: ArgT1, mut inConstArg2: ArgT2, mut inArg: FT, mut inAccum: Arc<metamodelica::List<TO>>) -> (Arc<metamodelica::List<TO>>, FT) {
    pub type FuncType<TI: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, FT: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2, FT) -> Result<(TO, FT)> + 'static>;

    let mut outList: Arc<metamodelica::List<TO>> = inAccum.clone();
    let mut outArg: FT = inArg.clone();
    let mut res: TO;
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        (res, outArg) = inFunc(e.clone(), inConstArg.clone(), inConstArg2.clone(), outArg.clone()).unwrap();
        outList = cons(res.clone(), outList.clone());
    }
    outList = outList.clone().reverse();
    (outList, outArg)
}

pub fn map2FoldCheckReferenceEq<TIO: Clone + 'static + PartialEq, ArgT1: Clone + 'static, ArgT2: Clone + 'static, FT: Clone + 'static>(mut inList: Arc<metamodelica::List<TIO>>, mut inFunc: Arc<dyn ::std::ops::Fn(TIO, ArgT1, ArgT2, FT) -> Result<(TIO, FT)> + 'static>, mut inConstArg: ArgT1, mut inConstArg2: ArgT2, mut inArg: FT) -> (Arc<metamodelica::List<TIO>>, FT) {
    pub type FuncType<TIO: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TIO, ArgT1, ArgT2, FT) -> Result<(TIO, FT)> + 'static>;

    let mut outList: Arc<metamodelica::List<TIO>> = metamodelica::nil();
    let mut outArg: FT = inArg.clone();
    let mut res: TIO;
    let mut savedElt: TIO;
    let mut delst: DoubleEnded::MutableList<TIO>;
    let mut n: i32 = 0;
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        (res, outArg) = inFunc(e.clone(), inConstArg.clone(), inConstArg2.clone(), outArg.clone()).unwrap();
        if !(referenceEq(&e.clone(),&res.clone())) {
            savedElt = res.clone();
            delst = DoubleEnded::empty(res.clone());
            for mut elt in &*inList.clone() {
                let mut elt = elt.clone();
                if n.clone() < 0 {
                    (res, outArg) = inFunc(elt.clone(), inConstArg.clone(), inConstArg2.clone(), outArg.clone()).unwrap();
                } else {
                    res = if (n.clone() == 0) {savedElt.clone()} else {elt.clone()};
                }
                DoubleEnded::push_back(delst.clone(), res.clone());
                n = n.clone() - 1;
            }
            outList = DoubleEnded::toListAndClear(delst.clone(), metamodelica::nil());
            return (outList, outArg);
        }
        n = n.clone() + 1;
    }
    outList = inList.clone();
    (outList, outArg)
}

pub fn map3Fold<TI: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, ArgT3: Clone + 'static, FT: Clone + 'static, TO: Clone + 'static>(mut inList: Arc<metamodelica::List<TI>>, mut inFunc: Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2, ArgT3, FT) -> Result<(TO, FT)> + 'static>, mut inConstArg: ArgT1, mut inConstArg2: ArgT2, mut inConstArg3: ArgT3, mut inArg: FT) -> (Arc<metamodelica::List<TO>>, FT) {
    pub type FuncType<TI: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, ArgT3: Clone + 'static, FT: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1, ArgT2, ArgT3, FT) -> Result<(TO, FT)> + 'static>;

    let mut outList: Arc<metamodelica::List<TO>> = metamodelica::nil();
    let mut outArg: FT = inArg.clone();
    let mut res: TO;
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        (res, outArg) = inFunc(e.clone(), inConstArg.clone(), inConstArg2.clone(), inConstArg3.clone(), outArg.clone()).unwrap();
        outList = cons(res.clone(), outList.clone());
    }
    outList = outList.clone().reverse();
    (outList, outArg)
}

pub fn mapFoldList<TI: Clone + 'static, FT: Clone + 'static, TO: Clone + 'static>(mut inListList: Arc<metamodelica::List<Arc<metamodelica::List<TI>>>>, mut inFunc: Arc<dyn ::std::ops::Fn(TI, FT) -> Result<(TO, FT)> + 'static>, mut inArg: FT) -> (Arc<metamodelica::List<Arc<metamodelica::List<TO>>>>, FT) {
    pub type FuncType<TI: Clone + 'static, FT: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, FT) -> Result<(TO, FT)> + 'static>;

    let mut outListList: Arc<metamodelica::List<Arc<metamodelica::List<TO>>>> = metamodelica::nil();
    let mut outArg: FT = inArg.clone();
    let mut res: Arc<metamodelica::List<TO>> = metamodelica::nil();
    for mut lst in &*inListList.clone() {
        let mut lst = lst.clone();
        (res, outArg) = mapFold(lst.clone(), inFunc.clone(), outArg.clone());
        outListList = cons(res.clone(), outListList.clone());
    }
    outListList = outListList.clone().reverse();
    (outListList, outArg)
}

pub fn reduce<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inReduceFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<T> + 'static>) -> Result<T> {
    pub type ReduceFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<T> + 'static>;

    let mut outResult: T;
    let mut rest: Arc<metamodelica::List<T>> = metamodelica::nil();
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inList.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outResult = __pa0.clone();
    rest = __pa1.clone();
    for mut e in &*rest.clone() {
        let mut e = e.clone();
        outResult = inReduceFunc(outResult.clone(), e.clone())?;
    }
    Ok(outResult)
}

pub fn flatten<T: Clone + 'static>(mut inList: Arc<metamodelica::List<Arc<metamodelica::List<T>>>>) -> Arc<metamodelica::List<T>> {
    let mut outList: Arc<metamodelica::List<T>> = if (inList.clone().is_empty()) {metamodelica::nil()} else if (hasOneElement(inList.clone())) {listHead(inList.clone()).unwrap()} else {{
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut lst in (inList.clone().reverse()).into_iter().cloned() {
            let __x = lst.clone();
            __acc = __x.append(&__acc);
        }
        __acc
    }};
    outList
}

pub fn flattenReverse<T: Clone + 'static>(mut inList: Arc<metamodelica::List<Arc<metamodelica::List<T>>>>) -> Arc<metamodelica::List<T>> {
    let mut outList: Arc<metamodelica::List<T>> = if (inList.clone().is_empty()) {metamodelica::nil()} else if (hasOneElement(inList.clone())) {listHead(inList.clone()).unwrap()} else {{
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut lst in (inList.clone()).into_iter().cloned() {
            let __x = lst.clone();
            __acc = __x.append(&__acc);
        }
        __acc
    }};
    outList
}

pub fn thread<T: Clone + 'static>(mut inList1: Arc<metamodelica::List<T>>, mut inList2: Arc<metamodelica::List<T>>, mut inAccum: Arc<metamodelica::List<T>>) -> Result<Arc<metamodelica::List<T>>> {
    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut e2: T;
    let mut rest_e2: Arc<metamodelica::List<T>> = inList2.clone();
    for mut e1 in &*inList1.clone() {
        let mut e1 = e1.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_e2.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e2 = __pa0.clone();
        rest_e2 = __pa1.clone();
        outList = cons(e1.clone(), cons(e2.clone(), outList.clone()));
    }
    let true = (rest_e2.clone().is_empty()) else { bail!("pattern mismatch") };
    outList = outList.clone().reverse();
    Ok(outList)
}

pub fn zip<T1: Clone + 'static, T2: Clone + 'static>(mut inList1: Arc<metamodelica::List<T1>>, mut inList2: Arc<metamodelica::List<T2>>) -> Arc<metamodelica::List<(T1, T2)>> {
    let mut outTuples: Arc<metamodelica::List<(T1, T2)>> = metamodelica::nil();
    outTuples = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for (e1, e2) in (&(inList1.clone())).into_iter().zip((&(inList2.clone())).into_iter()) {
            let __x = (e1.clone(), e2.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outTuples
}

pub fn zip3<T1: Clone + 'static, T2: Clone + 'static, T3: Clone + 'static>(mut l1: Arc<metamodelica::List<T1>>, mut l2: Arc<metamodelica::List<T2>>, mut l3: Arc<metamodelica::List<T3>>) -> Arc<metamodelica::List<(T1, T2, T3)>> {
    let mut res: Arc<metamodelica::List<(T1, T2, T3)>> = metamodelica::nil();
    res = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for ((e1, e2), e3) in (&(l1.clone())).into_iter().zip((&(l2.clone())).into_iter()).zip((&(l3.clone())).into_iter()) {
            let __x = (e1.clone(), e2.clone(), e3.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    res
}

pub fn unzip<T1: Clone + 'static, T2: Clone + 'static>(mut inTuples: Arc<metamodelica::List<(T1, T2)>>) -> (Arc<metamodelica::List<T1>>, Arc<metamodelica::List<T2>>) {
    let mut outList1: Arc<metamodelica::List<T1>> = metamodelica::nil();
    let mut outList2: Arc<metamodelica::List<T2>> = metamodelica::nil();
    let mut e1: T1;
    let mut e2: T2;
    for mut tpl in &*inTuples.clone() {
        let mut tpl = tpl.clone();
        (e1, e2) = tpl.clone();
        outList1 = cons(e1.clone(), outList1.clone());
        if true /* isPresent not implemented in Rust */ {
            outList2 = cons(e2.clone(), outList2.clone());
        }
    }
    outList1 = outList1.clone().reverse();
    outList2 = outList2.clone().reverse();
    (outList1, outList2)
}

pub fn unzip3<T1: Clone + 'static, T2: Clone + 'static, T3: Clone + 'static>(mut tuples: Arc<metamodelica::List<(T1, T2, T3)>>) -> (Arc<metamodelica::List<T1>>, Arc<metamodelica::List<T2>>, Arc<metamodelica::List<T3>>) {
    let mut l1: Arc<metamodelica::List<T1>> = metamodelica::nil();
    let mut l2: Arc<metamodelica::List<T2>> = metamodelica::nil();
    let mut l3: Arc<metamodelica::List<T3>> = metamodelica::nil();
    let mut e1: T1;
    let mut e2: T2;
    let mut e3: T3;
    for mut t in &*tuples.clone().reverse() {
        let mut t = t.clone();
        (e1, e2, e3) = t.clone();
        l1 = cons(e1.clone(), l1.clone());
        l2 = cons(e2.clone(), l2.clone());
        l3 = cons(e3.clone(), l3.clone());
    }
    (l1, l2, l3)
}

pub fn unzipSecond<T1: Clone + 'static, T2: Clone + 'static>(mut inTuples: Arc<metamodelica::List<(T1, T2)>>) -> Arc<metamodelica::List<T2>> {
    let mut outList: Arc<metamodelica::List<T2>> = metamodelica::nil();
    let mut e: T2;
    for mut tpl in &*inTuples.clone() {
        let mut tpl = tpl.clone();
        (_, e) = tpl.clone();
        outList = cons(e.clone(), outList.clone());
    }
    outList = outList.clone().reverse();
    outList
}

pub fn threadMap<T1: Clone + 'static, T2: Clone + 'static, TO: Clone + 'static>(mut inList1: Arc<metamodelica::List<T1>>, mut inList2: Arc<metamodelica::List<T2>>, mut inMapFunc: Arc<dyn ::std::ops::Fn(T1, T2) -> Result<TO> + 'static>) -> Arc<metamodelica::List<TO>> {
    pub type MapFunc<T1: Clone + 'static, T2: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1, T2) -> Result<TO> + 'static>;

    let mut outList: Arc<metamodelica::List<TO>> = metamodelica::nil();
    outList = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for (e1, e2) in (&(inList1.clone())).into_iter().zip((&(inList2.clone())).into_iter()) {
            let __x = inMapFunc(e1.clone(), e2.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outList
}

pub fn threadMap_2<T1: Clone + 'static, T2: Clone + 'static, TO1: Clone + 'static, TO2: Clone + 'static>(mut inList1: Arc<metamodelica::List<T1>>, mut inList2: Arc<metamodelica::List<T2>>, mut inMapFunc: Arc<dyn ::std::ops::Fn(T1, T2) -> Result<(TO1, TO2)> + 'static>) -> Result<(Arc<metamodelica::List<TO1>>, Arc<metamodelica::List<TO2>>)> {
    pub type MapFunc<T1: Clone + 'static, T2: Clone + 'static, TO1: Clone + 'static, TO2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1, T2) -> Result<(TO1, TO2)> + 'static>;

    let mut outList1: Arc<metamodelica::List<TO1>> = metamodelica::nil();
    let mut outList2: Arc<metamodelica::List<TO2>> = metamodelica::nil();
    let mut e2: T2;
    let mut rest_e2: Arc<metamodelica::List<T2>> = inList2.clone();
    let mut ret1: TO1;
    let mut ret2: TO2;
    for mut e1 in &*inList1.clone() {
        let mut e1 = e1.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_e2.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e2 = __pa0.clone();
        rest_e2 = __pa1.clone();
        (ret1, ret2) = inMapFunc(e1.clone(), e2.clone())?;
        outList1 = cons(ret1.clone(), outList1.clone());
        outList2 = cons(ret2.clone(), outList2.clone());
    }
    outList1 = outList1.clone().reverse();
    outList2 = outList2.clone().reverse();
    Ok((outList1, outList2))
}

pub fn threadMapList<T1: Clone + 'static, T2: Clone + 'static, TO: Clone + 'static>(mut inList1: Arc<metamodelica::List<Arc<metamodelica::List<T1>>>>, mut inList2: Arc<metamodelica::List<Arc<metamodelica::List<T2>>>>, mut inMapFunc: Arc<dyn ::std::ops::Fn(T1, T2) -> Result<TO> + 'static>) -> Arc<metamodelica::List<Arc<metamodelica::List<TO>>>> {
    pub type MapFunc<T1: Clone + 'static, T2: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1, T2) -> Result<TO> + 'static>;

    let mut outList: Arc<metamodelica::List<Arc<metamodelica::List<TO>>>> = metamodelica::nil();
    outList = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for (lst1, lst2) in (&(inList1.clone())).into_iter().zip((&(inList2.clone())).into_iter()) {
            let __x = threadMap(lst1.clone(), lst2.clone(), inMapFunc.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outList
}

pub fn threadMapList_2<T1: Clone + 'static, T2: Clone + 'static, TO1: Clone + 'static, TO2: Clone + 'static>(mut inList1: Arc<metamodelica::List<Arc<metamodelica::List<T1>>>>, mut inList2: Arc<metamodelica::List<Arc<metamodelica::List<T2>>>>, mut inMapFunc: Arc<dyn ::std::ops::Fn(T1, T2) -> Result<(TO1, TO2)> + 'static>) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<TO1>>>>, Arc<metamodelica::List<Arc<metamodelica::List<TO2>>>>)> {
    pub type MapFunc<T1: Clone + 'static, T2: Clone + 'static, TO1: Clone + 'static, TO2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1, T2) -> Result<(TO1, TO2)> + 'static>;

    let mut outList1: Arc<metamodelica::List<Arc<metamodelica::List<TO1>>>> = metamodelica::nil();
    let mut outList2: Arc<metamodelica::List<Arc<metamodelica::List<TO2>>>> = metamodelica::nil();
    let mut l2: Arc<metamodelica::List<T2>> = metamodelica::nil();
    let mut rest_l2: Arc<metamodelica::List<Arc<metamodelica::List<T2>>>> = inList2.clone();
    let mut ret1: Arc<metamodelica::List<TO1>> = metamodelica::nil();
    let mut ret2: Arc<metamodelica::List<TO2>> = metamodelica::nil();
    for mut l1 in &*inList1.clone() {
        let mut l1 = l1.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_l2.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        l2 = __pa0.clone();
        rest_l2 = __pa1.clone();
        (ret1, ret2) = threadMap_2(l1.clone(), l2.clone(), inMapFunc.clone())?;
        outList1 = cons(ret1.clone(), outList1.clone());
        outList2 = cons(ret2.clone(), outList2.clone());
    }
    outList1 = outList1.clone().reverse();
    outList2 = outList2.clone().reverse();
    Ok((outList1, outList2))
}

pub fn threadMap1<T1: Clone + 'static, T2: Clone + 'static, ArgT1: Clone + 'static, TO: Clone + 'static>(mut inList1: Arc<metamodelica::List<T1>>, mut inList2: Arc<metamodelica::List<T2>>, mut inMapFunc: Arc<dyn ::std::ops::Fn(T1, T2, ArgT1) -> Result<TO> + 'static>, mut inArg1: ArgT1) -> Arc<metamodelica::List<TO>> {
    pub type MapFunc<T1: Clone + 'static, T2: Clone + 'static, ArgT1: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1, T2, ArgT1) -> Result<TO> + 'static>;

    let mut outList: Arc<metamodelica::List<TO>> = metamodelica::nil();
    outList = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for (e1, e2) in (&(inList1.clone())).into_iter().zip((&(inList2.clone())).into_iter()) {
            let __x = inMapFunc(e1.clone(), e2.clone(), inArg1.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outList
}

pub fn threadMap1_0<T1: Clone + 'static, T2: Clone + 'static, ArgT1: Clone + 'static>(mut inList1: Arc<metamodelica::List<T1>>, mut inList2: Arc<metamodelica::List<T2>>, mut inMapFunc: Arc<dyn ::std::ops::Fn(T1, T2, ArgT1) -> Result<()> + 'static>, mut inArg1: ArgT1) -> Result<()> {
    pub type MapFunc<T1: Clone + 'static, T2: Clone + 'static, ArgT1: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1, T2, ArgT1) -> Result<()> + 'static>;

    let _ = (::match_deref::match_deref! { match &((inList1.clone(), inList2.clone(), inMapFunc.clone(), inArg1.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _, _) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: e1, tail: rest1 }, Deref @ metamodelica::List::Cons { head: e2, tail: rest2 }, _, _) => {
            inMapFunc(e1.clone(), e2.clone(), inArg1.clone())?;
            threadMap1_0(rest1.clone(), rest2.clone(), inMapFunc.clone(), inArg1.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn threadMap2<T1: Clone + 'static, T2: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, TO: Clone + 'static>(mut inList1: Arc<metamodelica::List<T1>>, mut inList2: Arc<metamodelica::List<T2>>, mut inMapFunc: Arc<dyn ::std::ops::Fn(T1, T2, ArgT1, ArgT2) -> Result<TO> + 'static>, mut inArg1: ArgT1, mut inArg2: ArgT2) -> Arc<metamodelica::List<TO>> {
    pub type MapFunc<T1: Clone + 'static, T2: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1, T2, ArgT1, ArgT2) -> Result<TO> + 'static>;

    let mut outList: Arc<metamodelica::List<TO>> = metamodelica::nil();
    outList = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for (e1, e2) in (&(inList1.clone())).into_iter().zip((&(inList2.clone())).into_iter()) {
            let __x = inMapFunc(e1.clone(), e2.clone(), inArg1.clone(), inArg2.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outList
}

pub fn thread3Map<T1: Clone + 'static, T2: Clone + 'static, T3: Clone + 'static, TO: Clone + 'static>(mut inList1: Arc<metamodelica::List<T1>>, mut inList2: Arc<metamodelica::List<T2>>, mut inList3: Arc<metamodelica::List<T3>>, mut inFunc: Arc<dyn ::std::ops::Fn(T1, T2, T3) -> Result<TO> + 'static>) -> Arc<metamodelica::List<TO>> {
    pub type MapFunc<T1: Clone + 'static, T2: Clone + 'static, T3: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1, T2, T3) -> Result<TO> + 'static>;

    let mut outList: Arc<metamodelica::List<TO>> = metamodelica::nil();
    outList = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for ((e1, e2), e3) in (&(inList1.clone())).into_iter().zip((&(inList2.clone())).into_iter()).zip((&(inList3.clone())).into_iter()) {
            let __x = inFunc(e1.clone(), e2.clone(), e3.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outList
}

pub fn thread3MapFold<T1: Clone + 'static, T2: Clone + 'static, T3: Clone + 'static, ArgT1: Clone + 'static, TO: Clone + 'static>(mut inList1: Arc<metamodelica::List<T1>>, mut inList2: Arc<metamodelica::List<T2>>, mut inList3: Arc<metamodelica::List<T3>>, mut inFunc: Arc<dyn ::std::ops::Fn(T1, T2, T3, ArgT1) -> Result<(TO, ArgT1)> + 'static>, mut inArg: ArgT1) -> Result<(Arc<metamodelica::List<TO>>, ArgT1)> {
    pub type MapFunc<T1: Clone + 'static, T2: Clone + 'static, T3: Clone + 'static, ArgT1: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1, T2, T3, ArgT1) -> Result<(TO, ArgT1)> + 'static>;

    let mut outList: Arc<metamodelica::List<TO>> = metamodelica::nil();
    let mut outArg: ArgT1 = inArg.clone();
    let mut e2: T2;
    let mut rest_e2: Arc<metamodelica::List<T2>> = inList2.clone();
    let mut e3: T3;
    let mut rest_e3: Arc<metamodelica::List<T3>> = inList3.clone();
    let mut res: TO;
    for mut e1 in &*inList1.clone() {
        let mut e1 = e1.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_e2.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e2 = __pa0.clone();
        rest_e2 = __pa1.clone();
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest_e3.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e3 = __pa2.clone();
        rest_e3 = __pa3.clone();
        (res, outArg) = inFunc(e1.clone(), e2.clone(), e3.clone(), outArg.clone())?;
        outList = cons(res.clone(), outList.clone());
    }
    let true = (rest_e2.clone().is_empty()) else { bail!("pattern mismatch") };
    let true = (rest_e3.clone().is_empty()) else { bail!("pattern mismatch") };
    outList = outList.clone().reverse();
    Ok((outList, outArg))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn threadFold1<T1: Clone + 'static, T2: Clone + 'static, ArgT1: Clone + 'static, FT: Clone + 'static>(mut inList1: Arc<metamodelica::List<T1>>, mut inList2: Arc<metamodelica::List<T2>>, mut inFoldFunc: Arc<dyn ::std::ops::Fn(T1, T2, ArgT1, FT) -> Result<FT> + 'static>, mut inArg1: ArgT1, mut inFoldArg: FT) -> Result<FT> {
    pub type FoldFunc<T1: Clone + 'static, T2: Clone + 'static, ArgT1: Clone + 'static, FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1, T2, ArgT1, FT) -> Result<FT> + 'static>;

    let mut outFoldArg: FT;
    outFoldArg = (::match_deref::match_deref! { match &((inList1.clone(), inList2.clone())) {
        (Deref @ metamodelica::List::Cons { head: e1, tail: rest1 }, Deref @ metamodelica::List::Cons { head: e2, tail: rest2 }) => {
            let mut res: FT;
            res = inFoldFunc(e1.clone(), e2.clone(), inArg1.clone(), inFoldArg.clone())?;
            threadFold1(rest1.clone(), rest2.clone(), inFoldFunc.clone(), inArg1.clone(), res.clone())?
        },
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            inFoldArg.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outFoldArg)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn threadFold2<T1: Clone + 'static, T2: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, FT: Clone + 'static>(mut inList1: Arc<metamodelica::List<T1>>, mut inList2: Arc<metamodelica::List<T2>>, mut inFoldFunc: Arc<dyn ::std::ops::Fn(T1, T2, ArgT1, ArgT2, FT) -> Result<FT> + 'static>, mut inArg1: ArgT1, mut inArg2: ArgT2, mut inFoldArg: FT) -> Result<FT> {
    pub type FoldFunc<T1: Clone + 'static, T2: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1, T2, ArgT1, ArgT2, FT) -> Result<FT> + 'static>;

    let mut outFoldArg: FT;
    outFoldArg = (::match_deref::match_deref! { match &((inList1.clone(), inList2.clone())) {
        (Deref @ metamodelica::List::Cons { head: e1, tail: rest1 }, Deref @ metamodelica::List::Cons { head: e2, tail: rest2 }) => {
            let mut res: FT;
            res = inFoldFunc(e1.clone(), e2.clone(), inArg1.clone(), inArg2.clone(), inFoldArg.clone())?;
            threadFold2(rest1.clone(), rest2.clone(), inFoldFunc.clone(), inArg1.clone(), inArg2.clone(), res.clone())?
        },
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            inFoldArg.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outFoldArg)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn threadFold3<T1: Clone + 'static, T2: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, ArgT3: Clone + 'static, FT: Clone + 'static>(mut inList1: Arc<metamodelica::List<T1>>, mut inList2: Arc<metamodelica::List<T2>>, mut inFoldFunc: Arc<dyn ::std::ops::Fn(T1, T2, ArgT1, ArgT2, ArgT3, FT) -> Result<FT> + 'static>, mut inArg1: ArgT1, mut inArg2: ArgT2, mut inArg3: ArgT3, mut inFoldArg: FT) -> Result<FT> {
    pub type FoldFunc<T1: Clone + 'static, T2: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static, ArgT3: Clone + 'static, FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1, T2, ArgT1, ArgT2, ArgT3, FT) -> Result<FT> + 'static>;

    let mut outFoldArg: FT;
    outFoldArg = (::match_deref::match_deref! { match &((inList1.clone(), inList2.clone())) {
        (Deref @ metamodelica::List::Cons { head: e1, tail: rest1 }, Deref @ metamodelica::List::Cons { head: e2, tail: rest2 }) => {
            let mut res: FT;
            res = inFoldFunc(e1.clone(), e2.clone(), inArg1.clone(), inArg2.clone(), inArg3.clone(), inFoldArg.clone())?;
            threadFold3(rest1.clone(), rest2.clone(), inFoldFunc.clone(), inArg1.clone(), inArg2.clone(), inArg3.clone(), res.clone())?
        },
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            inFoldArg.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outFoldArg)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn threadFold<T1: Clone + 'static, T2: Clone + 'static, FT: Clone + 'static>(mut inList1: Arc<metamodelica::List<T1>>, mut inList2: Arc<metamodelica::List<T2>>, mut inFoldFunc: Arc<dyn ::std::ops::Fn(T1, T2, FT) -> Result<FT> + 'static>, mut inFoldArg: FT) -> Result<FT> {
    pub type FoldFunc<T1: Clone + 'static, T2: Clone + 'static, FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1, T2, FT) -> Result<FT> + 'static>;

    let mut outFoldArg: FT;
    outFoldArg = (::match_deref::match_deref! { match &((inList1.clone(), inList2.clone())) {
        (Deref @ metamodelica::List::Cons { head: e1, tail: rest1 }, Deref @ metamodelica::List::Cons { head: e2, tail: rest2 }) => {
            let mut res: FT;
            res = inFoldFunc(e1.clone(), e2.clone(), inFoldArg.clone())?;
            threadFold(rest1.clone(), rest2.clone(), inFoldFunc.clone(), res.clone())?
        },
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            inFoldArg.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outFoldArg)
}

pub fn threadMapFold<T1: Clone + 'static, T2: Clone + 'static, FT: Clone + 'static, TO: Clone + 'static>(mut inList1: Arc<metamodelica::List<T1>>, mut inList2: Arc<metamodelica::List<T2>>, mut inFunc: Arc<dyn ::std::ops::Fn(T1, T2, FT) -> Result<(TO, FT)> + 'static>, mut inArg: FT) -> Result<(Arc<metamodelica::List<TO>>, FT)> {
    pub type FuncType<T1: Clone + 'static, T2: Clone + 'static, FT: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1, T2, FT) -> Result<(TO, FT)> + 'static>;

    let mut outList: Arc<metamodelica::List<TO>> = metamodelica::nil();
    let mut outArg: FT = inArg.clone();
    let mut e2: T2;
    let mut rest_e2: Arc<metamodelica::List<T2>> = inList2.clone();
    let mut res: TO;
    for mut e1 in &*inList1.clone() {
        let mut e1 = e1.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_e2.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e2 = __pa0.clone();
        rest_e2 = __pa1.clone();
        (res, outArg) = inFunc(e1.clone(), e2.clone(), outArg.clone())?;
        outList = cons(res.clone(), outList.clone());
    }
    let true = (rest_e2.clone().is_empty()) else { bail!("pattern mismatch") };
    outList = outList.clone().reverse();
    Ok((outList, outArg))
}

pub fn position<T: Clone + 'static + PartialEq>(mut inElement: T, mut inList: Arc<metamodelica::List<T>>) -> Result<i32> {
    let mut outPosition: i32 = 1;
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        if e.clone() == inElement.clone() {
            return Ok(outPosition);
        }
        outPosition = outPosition.clone() + 1;
    }
    bail!("fail");
    Ok(outPosition)
}

pub fn positionOnTrue<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inPredFunc: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> i32 {
    pub type PredFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outPosition: i32 = 1;
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        if inPredFunc(e.clone()).unwrap() {
            return outPosition;
        }
        outPosition = outPosition.clone() + 1;
    }
    outPosition = -1;
    outPosition
}

pub fn position1OnTrue<T: Clone + 'static, ArgT: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inPredFunc: Arc<dyn ::std::ops::Fn(T, ArgT) -> Result<bool> + 'static>, mut inArg: ArgT) -> i32 {
    pub type PredFunc<T: Clone + 'static, ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, ArgT) -> Result<bool> + 'static>;

    let mut outPosition: i32 = 1;
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        if inPredFunc(e.clone(), inArg.clone()).unwrap() {
            return outPosition;
        }
        outPosition = outPosition.clone() + 1;
    }
    outPosition = -1;
    outPosition
}

pub fn getMember<T: Clone + 'static + PartialEq>(mut inElement: T, mut inList: Arc<metamodelica::List<T>>) -> Result<T> {
    let mut outElement: T;
    let mut e: T;
    let mut res: T;
    let mut rest: Arc<metamodelica::List<T>> = metamodelica::nil();
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        if inElement.clone() == e.clone() {
            outElement = e.clone();
            return Ok(outElement);
        }
    }
    bail!("fail");
    Ok(outElement)
}

pub fn getMemberOnTrue<VT: Clone + 'static, T: Clone + 'static>(mut inValue: VT, mut inList: Arc<metamodelica::List<T>>, mut inCompFunc: Arc<dyn ::std::ops::Fn(VT, T) -> Result<bool> + 'static>) -> Result<T> {
    pub type CompFunc<VT: Clone + 'static, T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(VT, T) -> Result<bool> + 'static>;

    let mut outElement: T;
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        if inCompFunc(inValue.clone(), e.clone())? {
            outElement = e.clone();
            return Ok(outElement);
        }
    }
    bail!("fail");
    Ok(outElement)
}

pub fn notMember<T: Clone + 'static + PartialEq>(mut inElement: T, mut inList: Arc<metamodelica::List<T>>) -> bool {
    let mut outIsNotMember: bool = false;
    outIsNotMember = !(listMember(inElement.clone(), inList.clone()));
    outIsNotMember
}

pub fn isMemberOnTrue<VT: Clone + 'static, T: Clone + 'static>(mut inValue: VT, mut inList: Arc<metamodelica::List<T>>, mut inCompFunc: Arc<dyn ::std::ops::Fn(VT, T) -> Result<bool> + 'static>) -> bool {
    pub type CompFunc<VT: Clone + 'static, T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(VT, T) -> Result<bool> + 'static>;

    let mut outIsMember: bool = false;
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        if inCompFunc(inValue.clone(), e.clone()).unwrap() {
            outIsMember = true;
            return outIsMember;
        }
    }
    outIsMember = false;
    outIsMember
}

pub fn exist1<T: Clone + 'static, ArgT1: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFindFunc: Arc<dyn ::std::ops::Fn(T, ArgT1) -> Result<bool> + 'static>, mut inExtraArg: ArgT1) -> bool {
    pub type FindFunc<T: Clone + 'static, ArgT1: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1) -> Result<bool> + 'static>;

    let mut outExists: bool = false;
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        if inFindFunc(e.clone(), inExtraArg.clone()).unwrap() {
            outExists = true;
            return outExists;
        }
    }
    outExists = false;
    outExists
}

pub fn extractOnTrue<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFilterFunc: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> (Arc<metamodelica::List<T>>, Arc<metamodelica::List<T>>) {
    pub type FilterFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outExtractedList: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut outRemainingList: Arc<metamodelica::List<T>> = metamodelica::nil();
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        if inFilterFunc(e.clone()).unwrap() {
            outExtractedList = cons(e.clone(), outExtractedList.clone());
        } else {
            outRemainingList = cons(e.clone(), outRemainingList.clone());
        }
    }
    outExtractedList = outExtractedList.clone().reverse();
    outRemainingList = outRemainingList.clone().reverse();
    (outExtractedList, outRemainingList)
}

pub fn extract1OnTrue<T: Clone + 'static, ArgT1: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFilterFunc: Arc<dyn ::std::ops::Fn(T, ArgT1) -> Result<bool> + 'static>, mut inArg: ArgT1) -> (Arc<metamodelica::List<T>>, Arc<metamodelica::List<T>>) {
    pub type FilterFunc<T: Clone + 'static, ArgT1: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1) -> Result<bool> + 'static>;

    let mut outExtractedList: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut outRemainingList: Arc<metamodelica::List<T>> = metamodelica::nil();
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        if inFilterFunc(e.clone(), inArg.clone()).unwrap() {
            outExtractedList = cons(e.clone(), outExtractedList.clone());
        } else {
            outRemainingList = cons(e.clone(), outRemainingList.clone());
        }
    }
    outExtractedList = outExtractedList.clone().reverse();
    outRemainingList = outRemainingList.clone().reverse();
    (outExtractedList, outRemainingList)
}

pub fn filter<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFilterFunc: Arc<dyn ::std::ops::Fn(T) -> Result<()> + 'static>) -> Arc<metamodelica::List<T>> {
    pub type FilterFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<()> + 'static>;

    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        if '__try0: {
            unwrap_break_err!(inFilterFunc(e.clone()), '__try0);
            outList = cons(e.clone(), outList.clone());
            Ok::<(), anyhow::Error>(())
        }.is_err() {
        }
    }
    outList = outList.clone().reverse();
    outList
}

pub fn filterMap<TI: Clone + 'static, TO: Clone + 'static>(mut inList: Arc<metamodelica::List<TI>>, mut inFilterMapFunc: Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>) -> Arc<metamodelica::List<TO>> {
    pub type FilterMapFunc<TI: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>;

    let mut outList: Arc<metamodelica::List<TO>> = metamodelica::nil();
    let mut oe: TO;
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        if '__try0: {
            oe = unwrap_break_err!(inFilterMapFunc(e.clone()), '__try0);
            outList = cons(oe.clone(), outList.clone());
            Ok::<(), anyhow::Error>(())
        }.is_err() {
        }
    }
    outList = outList.clone().reverse();
    outList
}

pub fn filterMap1<TI: Clone + 'static, ArgT1: Clone + 'static, TO: Clone + 'static>(mut inList: Arc<metamodelica::List<TI>>, mut inFilterMapFunc: Arc<dyn ::std::ops::Fn(TI, ArgT1) -> Result<TO> + 'static>, mut inExtraArg: ArgT1) -> Arc<metamodelica::List<TO>> {
    pub type FilterMapFunc<TI: Clone + 'static, ArgT1: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT1) -> Result<TO> + 'static>;

    let mut outList: Arc<metamodelica::List<TO>> = metamodelica::nil();
    let mut oe: TO;
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        if '__try0: {
            oe = unwrap_break_err!(inFilterMapFunc(e.clone(), inExtraArg.clone()), '__try0);
            outList = cons(oe.clone(), outList.clone());
            Ok::<(), anyhow::Error>(())
        }.is_err() {
        }
    }
    outList = outList.clone().reverse();
    outList
}

pub fn filterOnTrue<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFilterFunc: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> Arc<metamodelica::List<T>> {
    pub type FilterFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    outList = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut e in (inList.clone()).into_iter().cloned() {
            if !(inFilterFunc(e.clone()).unwrap()) { continue; }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outList
}

pub fn filterOnFalse<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFilterFunc: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> Arc<metamodelica::List<T>> {
    pub type FilterFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    outList = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut e in (inList.clone()).into_iter().cloned() {
            if !(boolNot(inFilterFunc(e.clone()).unwrap())) { continue; }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outList
}

pub fn filter1OnTrueSync<T1: Clone + 'static, ArgT1: Clone + 'static, T2: Clone + 'static>(mut inList: Arc<metamodelica::List<T1>>, mut inFilterFunc: Arc<dyn ::std::ops::Fn(T1, ArgT1) -> Result<bool> + 'static>, mut inArg1: ArgT1, mut inSyncList: Arc<metamodelica::List<T2>>) -> Result<(Arc<metamodelica::List<T1>>, Arc<metamodelica::List<T2>>)> {
    pub type FilterFunc<T1: Clone + 'static, ArgT1: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1, ArgT1) -> Result<bool> + 'static>;

    let mut outList_a: Arc<metamodelica::List<T1>> = metamodelica::nil();
    let mut outList_b: Arc<metamodelica::List<T2>> = metamodelica::nil();
    let mut e2: T2;
    let mut rest2: Arc<metamodelica::List<T2>> = inSyncList.clone();
    for mut e1 in &*inList.clone() {
        let mut e1 = e1.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest2.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e2 = __pa0.clone();
        rest2 = __pa1.clone();
        if inFilterFunc(e1.clone(), inArg1.clone())? {
            outList_a = cons(e1.clone(), outList_a.clone());
            outList_b = cons(e2.clone(), outList_b.clone());
        }
    }
    outList_a = outList_a.clone().reverse();
    outList_b = outList_b.clone().reverse();
    Ok((outList_a, outList_b))
}

pub fn filterOnTrueSync<T1: Clone + 'static, T2: Clone + 'static>(mut inList: Arc<metamodelica::List<T1>>, mut inFilterFunc: Arc<dyn ::std::ops::Fn(T1) -> Result<bool> + 'static>, mut inSyncList: Arc<metamodelica::List<T2>>) -> Result<(Arc<metamodelica::List<T1>>, Arc<metamodelica::List<T2>>)> {
    pub type FilterFunc<T1: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1) -> Result<bool> + 'static>;

    let mut outList_a: Arc<metamodelica::List<T1>> = metamodelica::nil();
    let mut outList_b: Arc<metamodelica::List<T2>> = metamodelica::nil();
    let mut e2: T2;
    let mut rest2: Arc<metamodelica::List<T2>> = inSyncList.clone();
    let true = ((inList.clone().len() as i32) == (inSyncList.clone().len() as i32)) else { bail!("pattern mismatch") };
    for mut e1 in &*inList.clone() {
        let mut e1 = e1.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest2.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e2 = __pa0.clone();
        rest2 = __pa1.clone();
        if inFilterFunc(e1.clone())? {
            outList_a = cons(e1.clone(), outList_a.clone());
            outList_b = cons(e2.clone(), outList_b.clone());
        }
    }
    outList_a = outList_a.clone().reverse();
    outList_b = outList_b.clone().reverse();
    Ok((outList_a, outList_b))
}

pub fn filter1<T: Clone + 'static, ArgT1: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFilterFunc: Arc<dyn ::std::ops::Fn(T, ArgT1) -> Result<()> + 'static>, mut inArg1: ArgT1) -> Arc<metamodelica::List<T>> {
    pub type FilterFunc<T: Clone + 'static, ArgT1: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1) -> Result<()> + 'static>;

    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        if '__try0: {
            unwrap_break_err!(inFilterFunc(e.clone(), inArg1.clone()), '__try0);
            outList = cons(e.clone(), outList.clone());
            Ok::<(), anyhow::Error>(())
        }.is_err() {
        }
    }
    outList = outList.clone().reverse();
    outList
}

pub fn filter1OnTrue<T: Clone + 'static, ArgT1: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFilterFunc: Arc<dyn ::std::ops::Fn(T, ArgT1) -> Result<bool> + 'static>, mut inArg1: ArgT1) -> Arc<metamodelica::List<T>> {
    pub type FilterFunc<T: Clone + 'static, ArgT1: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1) -> Result<bool> + 'static>;

    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    outList = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut e in (inList.clone()).into_iter().cloned() {
            if !(inFilterFunc(e.clone(), inArg1.clone()).unwrap()) { continue; }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outList
}

pub fn filter1OnTrueAndUpdate<T: Clone + 'static, ArgT1: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFilterFunc: Arc<dyn ::std::ops::Fn(T, ArgT1) -> Result<bool> + 'static>, mut inUpdateFunc: Arc<dyn ::std::ops::Fn(T, ArgT1) -> Result<T> + 'static>, mut inArg1: ArgT1) -> Arc<metamodelica::List<T>> {
    pub type FilterFunc<T: Clone + 'static, ArgT1: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1) -> Result<bool> + 'static>;

    pub type UpdateFunc<T: Clone + 'static, ArgT1: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1) -> Result<T> + 'static>;

    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    outList = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut e in (inList.clone()).into_iter().cloned() {
            if !(inFilterFunc(e.clone(), inArg1.clone()).unwrap()) { continue; }
            let __x = inUpdateFunc(e.clone(), inArg1.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outList
}

pub fn filter1rOnTrue<T: Clone + 'static, ArgT1: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFilterFunc: Arc<dyn ::std::ops::Fn(ArgT1, T) -> Result<bool> + 'static>, mut inArg1: ArgT1) -> Arc<metamodelica::List<T>> {
    pub type FilterFunc<ArgT1: Clone + 'static, T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(ArgT1, T) -> Result<bool> + 'static>;

    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    outList = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut e in (inList.clone()).into_iter().cloned() {
            if !(inFilterFunc(inArg1.clone(), e.clone()).unwrap()) { continue; }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outList
}

pub fn filter2OnTrue<T: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFilterFunc: Arc<dyn ::std::ops::Fn(T, ArgT1, ArgT2) -> Result<bool> + 'static>, mut inArg1: ArgT1, mut inArg2: ArgT2) -> Arc<metamodelica::List<T>> {
    pub type FilterFunc<T: Clone + 'static, ArgT1: Clone + 'static, ArgT2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1, ArgT2) -> Result<bool> + 'static>;

    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    outList = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut e in (inList.clone()).into_iter().cloned() {
            if !(inFilterFunc(e.clone(), inArg1.clone(), inArg2.clone()).unwrap()) { continue; }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outList
}

pub fn removeOnTrue<VT: Clone + 'static, T: Clone + 'static>(mut inValue: VT, mut inCompFunc: Arc<dyn ::std::ops::Fn(VT, T) -> Result<bool> + 'static>, mut inList: Arc<metamodelica::List<T>>) -> Arc<metamodelica::List<T>> {
    pub type CompFunc<VT: Clone + 'static, T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(VT, T) -> Result<bool> + 'static>;

    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    outList = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut e in (inList.clone()).into_iter().cloned() {
            if !(!(inCompFunc(inValue.clone(), e.clone()).unwrap())) { continue; }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outList
}

pub fn filterCons<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut r#fn: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>, mut accumList: Arc<metamodelica::List<T>>) -> Arc<metamodelica::List<T>> {
    pub type FilterFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut accumList: Arc<metamodelica::List<T>> = accumList;
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        if r#fn(e.clone()).unwrap() {
            accumList = cons(e.clone(), accumList.clone());
        }
    }
    accumList
}

pub use filterOnTrue as select;

pub use filter1OnTrue as select1;

pub use filter1rOnTrue as select1r;

pub use filter2OnTrue as select2;

pub fn find<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFunc: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> Result<T> {
    pub type SelectFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outElement: T;
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        if inFunc(e.clone())? {
            outElement = e.clone();
            return Ok(outElement);
        }
    }
    bail!("fail");
    Ok(outElement)
}

pub fn findOption<T: Clone + 'static>(mut lst: Arc<metamodelica::List<T>>, mut r#fn: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> Option<T> {
    pub type Predicate<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut result: Option<T> = None;
    for mut e in &*lst.clone() {
        let mut e = e.clone();
        if r#fn(e.clone()).unwrap() {
            result = Some(e.clone());
            return result;
        }
    }
    result = None;
    result
}

pub fn find1<T: Clone + 'static, ArgT1: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFunc: Arc<dyn ::std::ops::Fn(T, ArgT1) -> Result<bool> + 'static>, mut arg1: ArgT1) -> Result<T> {
    pub type SelectFunc<T: Clone + 'static, ArgT1: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1) -> Result<bool> + 'static>;

    let mut outElement: T;
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        if inFunc(e.clone(), arg1.clone())? {
            outElement = e.clone();
            return Ok(outElement);
        }
    }
    bail!("fail");
    Ok(outElement)
}

pub fn findAndRemove<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFunc: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> Result<(T, Arc<metamodelica::List<T>>)> {
    pub type SelectFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outElement: T;
    let mut rest: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut i: i32 = 0;
    let mut delst: DoubleEnded::MutableList<T>;
    let mut t: T;
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        if inFunc(e.clone())? {
            outElement = e.clone();
            delst = DoubleEnded::fromList(metamodelica::nil())?;
            rest = inList.clone();
            for mut i in 1..=i.clone() {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                t = __pa0.clone();
                rest = __pa1.clone();
                DoubleEnded::push_back(delst.clone(), t.clone());
            }
            let __pa2 = ::match_deref::match_deref! { match &(rest.clone()) {
                Deref @ metamodelica::List::Cons { head: _, tail: __pa2 } => __pa2.clone(),
                _ => bail!("pattern mismatch"),
            } };
            rest = __pa2.clone();
            rest = DoubleEnded::toListAndClear(delst.clone(), rest.clone());
            return Ok((outElement, rest));
        }
        i = i.clone() + 1;
    }
    bail!("fail");
    Ok((outElement, rest))
}

pub fn findAndRemove1<T: Clone + 'static, ArgT1: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFunc: Arc<dyn ::std::ops::Fn(T, ArgT1) -> Result<bool> + 'static>, mut arg1: ArgT1) -> Result<(T, Arc<metamodelica::List<T>>)> {
    pub type SelectFunc<T: Clone + 'static, ArgT1: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1) -> Result<bool> + 'static>;

    let mut outElement: T;
    let mut rest: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut i: i32 = 0;
    let mut delst: DoubleEnded::MutableList<T>;
    let mut t: T;
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        if inFunc(e.clone(), arg1.clone())? {
            outElement = e.clone();
            delst = DoubleEnded::fromList(metamodelica::nil())?;
            rest = inList.clone();
            for mut i in 1..=i.clone() {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                t = __pa0.clone();
                rest = __pa1.clone();
                DoubleEnded::push_back(delst.clone(), t.clone());
            }
            let __pa2 = ::match_deref::match_deref! { match &(rest.clone()) {
                Deref @ metamodelica::List::Cons { head: _, tail: __pa2 } => __pa2.clone(),
                _ => bail!("pattern mismatch"),
            } };
            rest = __pa2.clone();
            rest = DoubleEnded::toListAndClear(delst.clone(), rest.clone());
            return Ok((outElement, rest));
        }
        i = i.clone() + 1;
    }
    bail!("fail");
    Ok((outElement, rest))
}

pub fn findBoolList<T: Clone + 'static>(mut inBooleans: Arc<metamodelica::List<bool>>, mut inList: Arc<metamodelica::List<T>>, mut inFalseValue: T) -> Result<T> {
    let mut outElement: T;
    let mut e: T;
    let mut rest: Arc<metamodelica::List<T>> = inList.clone();
    for mut b in &*inBooleans.clone() {
        let mut b = b.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa0.clone();
        rest = __pa1.clone();
        if b.clone() {
            outElement = e.clone();
            return Ok(outElement);
        }
    }
    outElement = inFalseValue.clone();
    Ok(outElement)
}

pub fn deleteMemberOnTrue<VT: Clone + 'static, T: Clone + 'static>(mut inValue: VT, mut inList: Arc<metamodelica::List<T>>, mut inCompareFunc: Arc<dyn ::std::ops::Fn(VT, T) -> Result<bool> + 'static>) -> Result<(Arc<metamodelica::List<T>>, Option<T>)> {
    pub type CompareFunc<VT: Clone + 'static, T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(VT, T) -> Result<bool> + 'static>;

    let mut outList: Arc<metamodelica::List<T>> = inList.clone();
    let mut outDeletedElement: Option<T> = None;
    let mut e: T;
    let mut rest: Arc<metamodelica::List<T>> = inList.clone();
    let mut acc: Arc<metamodelica::List<T>> = metamodelica::nil();
    while !(rest.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa0.clone();
        rest = __pa1.clone();
        if inCompareFunc(inValue.clone(), e.clone())? {
            outList = listAppend(acc.clone().reverse(), rest.clone());
            if true /* isPresent not implemented in Rust */ {
                outDeletedElement = Some(e.clone());
            }
            return Ok((outList, outDeletedElement));
        }
        acc = cons(e.clone(), acc.clone());
    }
    Ok((outList, outDeletedElement))
}

pub fn deletePositions<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inPositions: Arc<metamodelica::List<i32>>, mut zeroBased: bool) -> Result<Arc<metamodelica::List<T>>> {
    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut sorted_pos: Arc<metamodelica::List<i32>> = metamodelica::nil();
    sorted_pos = sortedUnique(sort(inPositions.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    outList = deletePositionsSorted(inList.clone(), sorted_pos.clone(), zeroBased.clone())?;
    Ok(outList)
}

pub fn deletePositionsSorted<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inPositions: Arc<metamodelica::List<i32>>, mut zeroBased: bool) -> Result<Arc<metamodelica::List<T>>> {
    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut i: i32 = if (zeroBased.clone()) {0} else {1};
    let mut e: T;
    let mut rest: Arc<metamodelica::List<T>> = inList.clone();
    for mut pos in &*inPositions.clone() {
        let mut pos = pos.clone();
        while i.clone() != pos.clone() {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e = __pa0.clone();
            rest = __pa1.clone();
            outList = cons(e.clone(), outList.clone());
            i = i.clone() + 1;
        }
        let __pa2 = ::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ metamodelica::List::Cons { head: _, tail: __pa2 } => __pa2.clone(),
            _ => bail!("pattern mismatch"),
        } };
        rest = __pa2.clone();
        i = i.clone() + 1;
    }
    outList = append_reverse(outList.clone(), rest.clone());
    Ok(outList)
}

pub fn keepPositions<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inPositions: Arc<metamodelica::List<i32>>, mut zeroBased: bool) -> Result<Arc<metamodelica::List<T>>> {
    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut sorted_pos: Arc<metamodelica::List<i32>> = metamodelica::nil();
    sorted_pos = sortedUnique(sort(inPositions.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    outList = keepPositionsSorted(inList.clone(), sorted_pos.clone(), zeroBased.clone())?;
    Ok(outList)
}

pub fn keepPositionsSorted<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inPositions: Arc<metamodelica::List<i32>>, mut zeroBased: bool) -> Result<Arc<metamodelica::List<T>>> {
    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut i: i32 = if (zeroBased.clone()) {0} else {1};
    let mut e: T;
    let mut rest: Arc<metamodelica::List<T>> = inList.clone();
    for mut pos in &*inPositions.clone() {
        let mut pos = pos.clone();
        while i.clone() != pos.clone() {
            let __pa0 = ::match_deref::match_deref! { match &(rest.clone()) {
                Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            rest = __pa0.clone();
            i = i.clone() + 1;
        }
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa1, tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa1.clone();
        rest = __pa2.clone();
        outList = cons(e.clone(), outList.clone());
        i = i.clone() + 1;
    }
    outList = outList.clone().reverse();
    Ok(outList)
}

pub fn replaceAt<T: Clone + 'static>(mut inElement: T, mut inPosition: i32, mut inList: Arc<metamodelica::List<T>>) -> Result<Arc<metamodelica::List<T>>> {
    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut e: T;
    let mut rest: Arc<metamodelica::List<T>> = inList.clone();
    let mut delst: DoubleEnded::MutableList<T>;
    let true = (inPosition.clone() >= 1) else { bail!("pattern mismatch") };
    delst = DoubleEnded::fromList(metamodelica::nil())?;
    for mut i in 1..=inPosition.clone() - 1 {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa0.clone();
        rest = __pa1.clone();
        DoubleEnded::push_back(delst.clone(), e.clone());
    }
    let __pa2 = ::match_deref::match_deref! { match &(rest.clone()) {
        Deref @ metamodelica::List::Cons { head: _, tail: __pa2 } => __pa2.clone(),
        _ => bail!("pattern mismatch"),
    } };
    rest = __pa2.clone();
    outList = DoubleEnded::toListAndClear(delst.clone(), cons(inElement.clone(), rest.clone()));
    Ok(outList)
}

pub fn replaceOnTrue<T: Clone + 'static>(mut inReplacement: T, mut inList: Arc<metamodelica::List<T>>, mut inFunc: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> Result<(Arc<metamodelica::List<T>>, bool)> {
    pub type FuncType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut outReplaced: bool = false;
    let mut e: T;
    let mut rest: Arc<metamodelica::List<T>> = inList.clone();
    while !(rest.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa0.clone();
        rest = __pa1.clone();
        if inFunc(e.clone())? {
            outReplaced = true;
            outList = append_reverse(outList.clone(), cons(inReplacement.clone(), rest.clone()));
            return Ok((outList, outReplaced));
        }
        outList = cons(e.clone(), outList.clone());
    }
    outList = inList.clone();
    Ok((outList, outReplaced))
}

pub fn replaceAtIndexFirst<T: Clone + 'static>(mut inPosition: i32, mut inElement: T, mut inList: Arc<metamodelica::List<T>>) -> Result<Arc<metamodelica::List<T>>> {
    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    outList = replaceAt(inElement.clone(), inPosition.clone(), inList.clone())?;
    Ok(outList)
}

pub fn replaceAtWithList<T: Clone + 'static>(mut inReplacementList: Arc<metamodelica::List<T>>, mut inPosition: i32, mut inList: Arc<metamodelica::List<T>>) -> Result<Arc<metamodelica::List<T>>> {
    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut e: T;
    let mut rest: Arc<metamodelica::List<T>> = inList.clone();
    let true = (inPosition.clone() >= 0) else { bail!("pattern mismatch") };
    for mut i in 0..=inPosition.clone() - 1 {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa0.clone();
        rest = __pa1.clone();
        outList = cons(e.clone(), outList.clone());
    }
    let __pa2 = ::match_deref::match_deref! { match &(rest.clone()) {
        Deref @ metamodelica::List::Cons { head: _, tail: __pa2 } => __pa2.clone(),
        _ => bail!("pattern mismatch"),
    } };
    rest = __pa2.clone();
    rest = listAppend(inReplacementList.clone(), rest.clone());
    outList = append_reverse(outList.clone(), rest.clone());
    Ok(outList)
}

pub fn toString<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inPrintFunc: Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>, mut inNameStr: ArcStr, mut inBeginStr: ArcStr, mut inDelimitStr: ArcStr, mut inEndStr: ArcStr, mut inPrintEmpty: bool, mut maxLength: i32) -> Result<ArcStr> {
    pub type FuncType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>;

    let mut outString: ArcStr = arcstr::literal!("");
    let mut lst: Arc<metamodelica::List<T>> = inList.clone();
    let mut endStr: ArcStr = inEndStr.clone();
    if maxLength.clone() > 0 && (lst.clone().len() as i32) > maxLength.clone() {
        lst = firstN(lst.clone(), maxLength.clone())?;
        endStr = stringAppendList(list![(inDelimitStr.clone()).clone(), (literal!("...")).clone(), (endStr.clone()).clone()]);
    }
    outString = ((::match_deref::match_deref! { match &((lst.clone(), inPrintEmpty.clone())) {
        (Deref @ metamodelica::List::Nil, true) => {
            stringAppendList(list![(inNameStr.clone()).clone(), (inBeginStr.clone()).clone(), (endStr.clone()).clone()])
        },
        (Deref @ metamodelica::List::Nil, false) => {
            inNameStr.clone()
        },
        _ => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = stringDelimitList(map(lst.clone(), inPrintFunc.clone()), (inDelimitStr.clone()).clone());
            r#str = stringAppendList(list![(inNameStr.clone()).clone(), (inBeginStr.clone()).clone(), (r#str.clone()).clone(), (endStr.clone()).clone()]);
            r#str.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

pub fn hasOneElement<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(inList.clone()) {
        Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn hasSeveralElements<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(inList.clone()) {
        Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } => false,
        Deref @ metamodelica::List::Nil => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn lengthListElements<T: Clone + 'static>(mut inListList: Arc<metamodelica::List<Arc<metamodelica::List<T>>>>) -> i32 {
    let mut outLength: i32 = 0;
    outLength = {
        let mut __acc: i32 = 0;
        for mut lst in (inListList.clone()).into_iter().cloned() {
            let __x = (lst.clone().len() as i32);
            __acc += __x;
        }
        __acc
    };
    outLength
}

pub fn accumulateMapAccum<TI: Clone + 'static, TO: Clone + 'static>(mut inList: Arc<metamodelica::List<TI>>, mut inMapFunc: Arc<dyn ::std::ops::Fn(TI, Arc<metamodelica::List<TO>>) -> Result<Arc<metamodelica::List<TO>>> + 'static>) -> Arc<metamodelica::List<TO>> {
    pub type MapFunc<TI: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, Arc<metamodelica::List<TO>>) -> Result<Arc<metamodelica::List<TO>>> + 'static>;

    let mut outList: Arc<metamodelica::List<TO>> = metamodelica::nil();
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        outList = inMapFunc(e.clone(), outList.clone()).unwrap();
    }
    outList = outList.clone().reverse();
    outList
}

pub fn findMap<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFunc: Arc<dyn ::std::ops::Fn(T) -> Result<(T, bool)> + 'static>) -> Result<(Arc<metamodelica::List<T>>, bool)> {
    pub type FuncType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<(T, bool)> + 'static>;

    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut outFound: bool = false;
    let mut e: T;
    let mut rest: Arc<metamodelica::List<T>> = inList.clone();
    while !(rest.clone().is_empty()) && !(outFound.clone()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa0.clone();
        rest = __pa1.clone();
        (e, outFound) = inFunc(e.clone())?;
        outList = cons(e.clone(), outList.clone());
    }
    outList = append_reverse(outList.clone(), rest.clone());
    Ok((outList, outFound))
}

pub fn findAndMap<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut pred: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>, mut func: Arc<dyn ::std::ops::Fn(T) -> Result<T> + 'static>) -> Result<(Arc<metamodelica::List<T>>, bool)> {
    pub type PredFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    pub type Func<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<T> + 'static>;

    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut found: bool = false;
    let mut e: T;
    let mut rest: Arc<metamodelica::List<T>> = inList.clone();
    while !(rest.clone().is_empty()) && !(found.clone()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa0.clone();
        rest = __pa1.clone();
        if pred(e.clone())? {
            e = func(e.clone())?;
            found = true;
        }
        outList = cons(e.clone(), outList.clone());
    }
    if found.clone() {
        outList = append_reverse(outList.clone(), rest.clone());
    } else {
        outList = inList.clone();
    }
    Ok((outList, found))
}

pub fn findSome<T1: Clone + 'static, T2: Clone + 'static>(mut inList: Arc<metamodelica::List<T1>>, mut inFunc: Arc<dyn ::std::ops::Fn(T1) -> Result<Option<T2>> + 'static>) -> Option<T2> {
    pub type FuncType<T1: Clone + 'static, T2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1) -> Result<Option<T2>> + 'static>;

    let mut outVal: Option<T2> = None;
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        outVal = inFunc(e.clone()).unwrap();
        if isSome(outVal.clone()) {
            return outVal;
        }
    }
    outVal
}

pub fn splitEqualPrefix<T1: Clone + 'static, T2: Clone + 'static>(mut inFullList: Arc<metamodelica::List<T1>>, mut inPrefixList: Arc<metamodelica::List<T2>>, mut inEqFunc: Arc<dyn ::std::ops::Fn(T1, T2) -> Result<bool> + 'static>, mut inAccum: Arc<metamodelica::List<T1>>) -> Result<(Arc<metamodelica::List<T1>>, Arc<metamodelica::List<T1>>)> {
    pub type EqFunc<T1: Clone + 'static, T2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1, T2) -> Result<bool> + 'static>;

    let mut outPrefix: Arc<metamodelica::List<T1>> = metamodelica::nil();
    let mut outRest: Arc<metamodelica::List<T1>> = metamodelica::nil();
    let mut e1: T1;
    let mut e2: T2;
    let mut rest_e1: Arc<metamodelica::List<T1>> = inFullList.clone();
    let mut rest_e2: Arc<metamodelica::List<T2>> = inPrefixList.clone();
    loop {
        if rest_e1.clone().is_empty() || rest_e2.clone().is_empty() {
            break;
        }
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_e1.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e1 = __pa0.clone();
        rest_e1 = __pa1.clone();
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest_e2.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e2 = __pa2.clone();
        rest_e2 = __pa3.clone();
        if !(inEqFunc(e1.clone(), e2.clone())?) {
            break;
        }
        outPrefix = cons(e1.clone(), outPrefix.clone());
    }
    outPrefix = outPrefix.clone().reverse();
    outRest = rest_e1.clone();
    Ok((outPrefix, outRest))
}

pub fn combination<TI: Clone + 'static>(mut inElements: Arc<metamodelica::List<Arc<metamodelica::List<TI>>>>) -> Arc<metamodelica::List<Arc<metamodelica::List<TI>>>> {
    let mut outElements: Arc<metamodelica::List<Arc<metamodelica::List<TI>>>> = metamodelica::nil();
    let mut elems: Arc<metamodelica::List<Arc<metamodelica::List<TI>>>> = metamodelica::nil();
    if inElements.clone().is_empty() {
        outElements = metamodelica::nil();
    } else {
        elems = combination_tail(inElements.clone(), metamodelica::nil(), metamodelica::nil());
        outElements = elems.clone().reverse();
    }
    outElements
}

fn combination_tail<TI: Clone + 'static>(mut inElements: Arc<metamodelica::List<Arc<metamodelica::List<TI>>>>, mut inCombination: Arc<metamodelica::List<TI>>, mut inAccumElems: Arc<metamodelica::List<Arc<metamodelica::List<TI>>>>) -> Arc<metamodelica::List<Arc<metamodelica::List<TI>>>> {
    let mut outElements: Arc<metamodelica::List<Arc<metamodelica::List<TI>>>> = metamodelica::nil();
    outElements = (::match_deref::match_deref! { match &(inElements.clone()) {
        Deref @ metamodelica::List::Cons { head: head, tail: rest } => {
            let mut acc: Arc<metamodelica::List<Arc<metamodelica::List<TI>>>> = metamodelica::nil();
            acc = inAccumElems.clone();
            for mut e in &*head.clone() {
                let mut e = e.clone();
                acc = combination_tail(rest.clone(), cons(e.clone(), inCombination.clone()), acc.clone());
            }
            acc.clone()
        },
        _ => {
            cons(inCombination.clone().reverse(), inAccumElems.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outElements
}

pub fn combinationMap<TI: Clone + 'static, TO: Clone + 'static>(mut inElements: Arc<metamodelica::List<Arc<metamodelica::List<TI>>>>, mut inMapFunc: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<TI>>) -> Result<TO> + 'static>) -> Arc<metamodelica::List<TO>> {
    pub type MapFunc<TI: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<TI>>) -> Result<TO> + 'static>;

    let mut outElements: Arc<metamodelica::List<TO>> = metamodelica::nil();
    let mut elems: Arc<metamodelica::List<TO>> = metamodelica::nil();
    elems = combinationMap_tail(inElements.clone(), inMapFunc.clone(), metamodelica::nil(), metamodelica::nil());
    outElements = elems.clone().reverse();
    outElements
}

fn combinationMap_tail<TI: Clone + 'static, TO: Clone + 'static>(mut inElements: Arc<metamodelica::List<Arc<metamodelica::List<TI>>>>, mut inMapFunc: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<TI>>) -> Result<TO> + 'static>, mut inCombination: Arc<metamodelica::List<TI>>, mut inAccumElems: Arc<metamodelica::List<TO>>) -> Arc<metamodelica::List<TO>> {
    pub type MapFunc<TI: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<TI>>) -> Result<TO> + 'static>;

    let mut outElements: Arc<metamodelica::List<TO>> = metamodelica::nil();
    outElements = (::match_deref::match_deref! { match &(inElements.clone()) {
        Deref @ metamodelica::List::Cons { head: head, tail: rest } => {
            let mut acc: Arc<metamodelica::List<TO>> = metamodelica::nil();
            acc = inAccumElems.clone();
            for mut e in &*head.clone() {
                let mut e = e.clone();
                acc = combinationMap_tail(rest.clone(), inMapFunc.clone(), cons(e.clone(), inCombination.clone()), acc.clone());
            }
            acc.clone()
        },
        _ => {
            cons(inMapFunc(inCombination.clone().reverse()).unwrap(), inAccumElems.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outElements
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn allReferenceEq<T: Clone + 'static + PartialEq>(mut inList1: Arc<metamodelica::List<T>>, mut inList2: Arc<metamodelica::List<T>>) -> bool {
    let mut outEqual: bool = false;
    outEqual = (::match_deref::match_deref! { match &((inList1.clone(), inList2.clone())) {
        (Deref @ metamodelica::List::Cons { head: el1, tail: rest1 }, Deref @ metamodelica::List::Cons { head: el2, tail: rest2 }) => {
            if (referenceEq(&el1.clone(),&el2.clone())) {allReferenceEq(rest1.clone(), rest2.clone())} else {false}
        },
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outEqual
}

pub fn listIsLonger<T: Clone + 'static>(mut inList1: Arc<metamodelica::List<T>>, mut inList2: Arc<metamodelica::List<T>>) -> bool {
    let mut isLonger: bool = compareLength(inList1.clone(), inList2.clone()).unwrap() > 0;
    isLonger
}

pub fn toListWithPositions<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>) -> Arc<metamodelica::List<(T, i32)>> {
    let mut outList: Arc<metamodelica::List<(T, i32)>> = metamodelica::nil();
    let mut pos: i32 = 1;
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        outList = cons((e.clone(), pos.clone()), outList.clone());
        pos = pos.clone() + 1;
    }
    outList = outList.clone().reverse();
    outList
}

pub fn mkOption<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>) -> Option<Arc<metamodelica::List<T>>> {
    let mut outOption: Option<Arc<metamodelica::List<T>>> = None;
    outOption = if (inList.clone().is_empty()) {None} else {Some(inList.clone())};
    outOption
}

pub fn all<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFunc: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> bool {
    pub type PredFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outResult: bool = false;
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        if !(inFunc(e.clone()).unwrap()) {
            outResult = false;
            return outResult;
        }
    }
    outResult = true;
    outResult
}

pub fn none<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFunc: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> bool {
    pub type PredFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outResult: bool = false;
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        if inFunc(e.clone()).unwrap() {
            outResult = false;
            return outResult;
        }
    }
    outResult = true;
    outResult
}

pub fn any<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFunc: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> bool {
    pub type PredFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outResult: bool = false;
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        if inFunc(e.clone()).unwrap() {
            outResult = true;
            return outResult;
        }
    }
    outResult = false;
    outResult
}

pub fn count<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFunc: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> i32 {
    pub type PredFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outResult: i32 = 0;
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        if inFunc(e.clone()).unwrap() {
            outResult = outResult.clone() + 1;
        }
    }
    outResult
}

pub fn separateOnTrue<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFilterFunc: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> (Arc<metamodelica::List<T>>, Arc<metamodelica::List<T>>) {
    pub type FilterFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outListTrue: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut outListFalse: Arc<metamodelica::List<T>> = metamodelica::nil();
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        if inFilterFunc(e.clone()).unwrap() {
            outListTrue = cons(e.clone(), outListTrue.clone());
        } else {
            outListFalse = cons(e.clone(), outListFalse.clone());
        }
    }
    (outListTrue, outListFalse)
}

pub fn separate1OnTrue<T: Clone + 'static, ArgT1: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFilterFunc: Arc<dyn ::std::ops::Fn(T, ArgT1) -> Result<bool> + 'static>, mut inArg1: ArgT1) -> (Arc<metamodelica::List<T>>, Arc<metamodelica::List<T>>) {
    pub type FilterFunc<T: Clone + 'static, ArgT1: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, ArgT1) -> Result<bool> + 'static>;

    let mut outListTrue: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut outListFalse: Arc<metamodelica::List<T>> = metamodelica::nil();
    for mut e in &*inList.clone() {
        let mut e = e.clone();
        if inFilterFunc(e.clone(), inArg1.clone()).unwrap() {
            outListTrue = cons(e.clone(), outListTrue.clone());
        } else {
            outListFalse = cons(e.clone(), outListFalse.clone());
        }
    }
    (outListTrue, outListFalse)
}

pub fn mapIndices<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut indices: Arc<metamodelica::List<i32>>, mut func: Arc<dyn ::std::ops::Fn(T) -> Result<T> + 'static>) -> Result<Arc<metamodelica::List<T>>> {
    pub type MapFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<T> + 'static>;

    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut i: i32 = 1;
    let mut idx: i32 = 0;
    let mut rest_idx: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut e: T;
    let mut rest_lst: Arc<metamodelica::List<T>> = metamodelica::nil();
    if indices.clone().is_empty() {
        outList = inList.clone();
        return Ok(outList);
    }
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(indices.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    idx = __pa0.clone();
    rest_idx = __pa1.clone();
    rest_lst = inList.clone();
    outList = metamodelica::nil();
    while !(rest_lst.clone().is_empty()) {
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest_lst.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa2.clone();
        rest_lst = __pa3.clone();
        if i.clone() == idx.clone() {
            outList = cons(func(e.clone())?, outList.clone());
            if rest_idx.clone().is_empty() {
                outList = append_reverse(rest_lst.clone(), outList.clone());
                break;
            } else {
                let (__pa4, __pa5) = ::match_deref::match_deref! { match &(rest_idx.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                idx = __pa4.clone();
                rest_idx = __pa5.clone();
            }
        } else {
            outList = cons(e.clone(), outList.clone());
        }
        i = i.clone() + 1;
    }
    outList = outList.clone().reverse();
    Ok(outList)
}

pub fn allCombinations<T: Clone + 'static>(mut lst: Arc<metamodelica::List<Arc<metamodelica::List<T>>>>, mut maxTotalSize: Option<i32>, mut info: SourceInfo) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<T>>>>> {
    let mut out: Arc<metamodelica::List<Arc<metamodelica::List<T>>>> = metamodelica::nil();
    out = (::match_deref::match_deref! { match &((lst.clone(), maxTotalSize.clone(), info.clone())) {
        (_, Some(maxSz), _) => {
            let mut sz: i32 = 0;
            sz = intMul((lst.clone().len() as i32), applyAndFold(lst.clone(), (std::sync::Arc::new(fnptr!(intMul, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), std::sync::Arc::new(fnptr!(listLength, _)), 1));
            let true = (sz.clone() <= maxSz.clone()) else { bail!("pattern mismatch") };
            allCombinations2(lst.clone())
        },
        (_, None, _) => {
            allCombinations2(lst.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out)
}

fn allCombinations2<T: Clone + 'static>(mut ilst: Arc<metamodelica::List<Arc<metamodelica::List<T>>>>) -> Arc<metamodelica::List<Arc<metamodelica::List<T>>>> {
    let mut out: Arc<metamodelica::List<Arc<metamodelica::List<T>>>> = metamodelica::nil();
    out = (::match_deref::match_deref! { match &(ilst.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: x, tail: lst } => {
            let mut lst = (*lst).clone();
            lst = allCombinations2(lst.clone());
            allCombinations3(x.clone(), lst.clone(), metamodelica::nil())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn allCombinations3<T: Clone + 'static>(mut ilst1: Arc<metamodelica::List<T>>, mut ilst2: Arc<metamodelica::List<Arc<metamodelica::List<T>>>>, mut iacc: Arc<metamodelica::List<Arc<metamodelica::List<T>>>>) -> Arc<metamodelica::List<Arc<metamodelica::List<T>>>> {
    let mut out: Arc<metamodelica::List<Arc<metamodelica::List<T>>>> = metamodelica::nil();
    out = (::match_deref::match_deref! { match &(ilst1.clone()) {
        Deref @ metamodelica::List::Nil => {
            iacc.clone().reverse()
        },
        Deref @ metamodelica::List::Cons { head: x, tail: lst1 } => {
            let mut acc: Arc<metamodelica::List<Arc<metamodelica::List<T>>>> = metamodelica::nil();
            acc = allCombinations4(x.clone(), ilst2.clone(), iacc.clone());
            allCombinations3(lst1.clone(), ilst2.clone(), acc.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out
}

fn allCombinations4<T: Clone + 'static>(mut x: T, mut ilst: Arc<metamodelica::List<Arc<metamodelica::List<T>>>>, mut iacc: Arc<metamodelica::List<Arc<metamodelica::List<T>>>>) -> Arc<metamodelica::List<Arc<metamodelica::List<T>>>> {
    let mut out: Arc<metamodelica::List<Arc<metamodelica::List<T>>>> = metamodelica::nil();
    let mut acc: Arc<metamodelica::List<Arc<metamodelica::List<T>>>> = iacc.clone();
    if ilst.clone().is_empty() {
        out = cons(list![x.clone()], acc.clone());
        return out;
    }
    for mut l in &*ilst.clone() {
        let mut l = l.clone();
        acc = cons(cons(x.clone(), l.clone()), acc.clone());
    }
    out = acc.clone();
    out
}

pub fn contains<T: Clone + 'static>(mut lst: Arc<metamodelica::List<T>>, mut elem: T, mut eqFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>) -> bool {
    pub type equalityFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut res: bool = false;
    for mut i in &*lst.clone() {
        let mut i = i.clone();
        if eqFunc(i.clone(), elem.clone()).unwrap() {
            res = true;
            return res;
        }
    }
    res
}

pub fn minElement<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut lessFn: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>) -> Result<T> {
    pub type LessFn<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut res: T;
    res = listHead(inList.clone())?;
    for mut e in &*listRest(inList.clone())? {
        let mut e = e.clone();
        if lessFn(e.clone(), res.clone())? {
            res = e.clone();
        }
    }
    Ok(res)
}

pub fn maxElement<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut lessFn: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>) -> Result<T> {
    pub type LessFn<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut res: T;
    res = listHead(inList.clone())?;
    for mut e in &*listRest(inList.clone())? {
        let mut e = e.clone();
        if lessFn(res.clone(), e.clone())? {
            res = e.clone();
        }
    }
    Ok(res)
}

pub fn trim<T: Clone + 'static>(mut l: Arc<metamodelica::List<T>>, mut r#fn: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> Result<Arc<metamodelica::List<T>>> {
    pub type PredFn<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut l: Arc<metamodelica::List<T>> = l;
    while !(l.clone().is_empty()) && r#fn(listHead(l.clone())?)? {
        l = listRest(l.clone())?;
    }
    Ok(l)
}

