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

use crate::List;

pub fn mapNoCopy<T: Clone + 'static>(mut inArray: metamodelica::Array<T>, mut inFunc: Arc<dyn ::std::ops::Fn(T) -> Result<T> + 'static>) -> metamodelica::Array<T> {
    pub type FuncType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<T> + 'static>;

    let mut outArray: metamodelica::Array<T> = inArray.clone();
    let __range0 = 1..=(inArray.clone().borrow().len() as i32);
    for mut i in __range0 {
        {let _arr = inArray.clone(); let _val = inFunc(inArray.clone().borrow()[(i.clone()-1) as usize].clone()).unwrap(); _arr.borrow_mut()[(i.clone()-1) as usize] = _val; _arr};
    }
    outArray
}

pub fn mapNoCopy_1<T: Clone + 'static, ArgT: Clone + 'static>(mut inArray: metamodelica::Array<T>, mut inFunc: Arc<dyn ::std::ops::Fn((T, ArgT)) -> Result<(T, ArgT)> + 'static>, mut inArg: ArgT) -> (metamodelica::Array<T>, ArgT) {
    pub type FuncType<T: Clone + 'static, ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn((T, ArgT)) -> Result<(T, ArgT)> + 'static>;

    let mut outArray: metamodelica::Array<T> = inArray.clone();
    let mut outArg: ArgT = inArg.clone();
    let mut e: T;
    let __range0 = 1..=(inArray.clone().borrow().len() as i32);
    for mut i in __range0 {
        (e, outArg) = inFunc((inArray.clone().borrow()[(i.clone()-1) as usize].clone(), outArg.clone())).unwrap();
        {let _arr = inArray.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = e.clone(); _arr};
    }
    (outArray, outArg)
}

fn downheap(mut inArray: metamodelica::Array<i32>, mut n: i32, mut vIn: i32) -> metamodelica::Array<i32> {
    let mut inArray: metamodelica::Array<i32> = inArray;
    let mut v: i32 = vIn.clone();
    let mut w: i32 = 2 * v.clone() + 1;
    let mut tmp: i32 = 0;
    while w.clone() < n.clone() {
        if w.clone() + 1 < n.clone() {
            if inArray.borrow()[(w.clone() + 2-1) as usize].clone() > inArray.borrow()[(w.clone() + 1-1) as usize].clone() {
                w = w.clone() + 1;
            }
        }
        if inArray.borrow()[(v.clone() + 1-1) as usize].clone() >= inArray.borrow()[(w.clone() + 1-1) as usize].clone() {
            return inArray;
        }
        tmp = inArray.borrow()[(v.clone() + 1-1) as usize].clone();
        {
            let __cell0 = inArray.borrow()[(w.clone() + 1-1) as usize].clone();
            inArray.clone().borrow_mut()[(v.clone() + 1-1) as usize] = __cell0;
        }
        {
            let __cell1 = tmp.clone();
            inArray.clone().borrow_mut()[(w.clone() + 1-1) as usize] = __cell1;
        }
        v = w.clone();
        w = 2 * v.clone() + 1;
    }
    inArray
}

pub fn heapSort(mut inArray: metamodelica::Array<i32>) -> metamodelica::Array<i32> {
    let mut inArray: metamodelica::Array<i32> = inArray;
    let mut n: i32 = (inArray.clone().borrow().len() as i32);
    let mut tmp: i32 = 0;
    for mut v in (0..=intDiv(n.clone(), 2) - 1).rev() {
        inArray = downheap(inArray.clone(), n.clone(), v.clone());
    }
    for mut v in (2..=n.clone()).rev() {
        tmp = inArray.borrow()[(1-1) as usize].clone();
        {
            let __cell0 = inArray.borrow()[(v.clone()-1) as usize].clone();
            inArray.clone().borrow_mut()[(1-1) as usize] = __cell0;
        }
        {
            let __cell1 = tmp.clone();
            inArray.clone().borrow_mut()[(v.clone()-1) as usize] = __cell1;
        }
        inArray = downheap(inArray.clone(), v.clone() - 1, 0);
    }
    inArray
}

pub fn findFirstOnTrue<T: Clone + 'static>(mut inArray: metamodelica::Array<T>, mut inPredicate: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> Option<T> {
    pub type FuncType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outElement: Option<T> = None;
    outElement = None;
    let __range0 = inArray.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        if inPredicate(e.clone()).unwrap() {
            outElement = Some(e.clone());
            break;
        }
    }
    outElement
}

pub fn findFirstOnTrueWithIdx<T: Clone + 'static>(mut inArray: metamodelica::Array<T>, mut inPredicate: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> (Option<T>, i32) {
    pub type FuncType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outElement: Option<T> = None;
    let mut idxOut: i32 = -1;
    let mut idx: i32 = 1;
    outElement = None;
    let __range0 = inArray.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        if inPredicate(e.clone()).unwrap() {
            idxOut = idx.clone();
            outElement = Some(e.clone());
            break;
        }
        idx = idx.clone() + 1;
    }
    (outElement, idxOut)
}

pub fn select<T: Clone + 'static>(mut inArray: metamodelica::Array<T>, mut inIndices: Arc<metamodelica::List<i32>>) -> Result<metamodelica::Array<T>> {
    let mut outArray: metamodelica::Array<T>;
    let mut i: i32 = 1;
    outArray = metamodelica::arrayCreate((inIndices.clone().len() as i32), inArray.borrow()[(1-1) as usize].clone());
    for mut e in &*inIndices.clone() {
        let mut e = e.clone();
        unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), i.clone(), inArray.clone().borrow()[(e.clone()-1) as usize].clone()) };
        i = i.clone() + 1;
    }
    Ok(outArray)
}

pub fn map<TI: Clone + 'static, TO: Clone + 'static>(mut inArray: metamodelica::Array<TI>, mut inFunc: Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>) -> metamodelica::Array<TO> {
    pub type FuncType<TI: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>;

    let mut outArray: metamodelica::Array<TO>;
    let mut len: i32 = (inArray.clone().borrow().len() as i32);
    let mut res: TO;
    if len.clone() == 0 {
        outArray = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
    } else {
        res = inFunc(inArray.clone().borrow()[(1-1) as usize].clone()).unwrap();
        outArray = metamodelica::arrayCreate(len.clone(), res.clone());
        unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), 1, res.clone()) };
        for mut i in 2..=len.clone() {
            unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), i.clone(), inFunc(inArray.clone().borrow()[(i.clone()-1) as usize].clone()).unwrap()) };
        }
    }
    outArray
}

pub fn map1<TI: Clone + 'static, TO: Clone + 'static, ArgT: Clone + 'static>(mut inArray: metamodelica::Array<TI>, mut inFunc: Arc<dyn ::std::ops::Fn(TI, ArgT) -> Result<TO> + 'static>, mut inArg: ArgT) -> Result<metamodelica::Array<TO>> {
    pub type FuncType<TI: Clone + 'static, ArgT: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT) -> Result<TO> + 'static>;

    let mut outArray: metamodelica::Array<TO>;
    let mut len: i32 = (inArray.clone().borrow().len() as i32);
    let mut res: TO;
    if len.clone() == 0 {
        outArray = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
    } else {
        res = inFunc(inArray.clone().borrow()[(1-1) as usize].clone(), inArg.clone())?;
        outArray = metamodelica::arrayCreate(len.clone(), res.clone());
        unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), 1, res.clone()) };
        for mut i in 2..=len.clone() {
            unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), i.clone(), inFunc(inArray.clone().borrow()[(i.clone()-1) as usize].clone(), inArg.clone())?) };
        }
    }
    Ok(outArray)
}

pub fn map1Ind<TI: Clone + 'static, TO: Clone + 'static, ArgT: Clone + 'static>(mut inArray: metamodelica::Array<TI>, mut inFunc: Arc<dyn ::std::ops::Fn(TI, i32, ArgT) -> Result<TO> + 'static>, mut inArg: ArgT) -> Result<metamodelica::Array<TO>> {
    pub type FuncType<TI: Clone + 'static, ArgT: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, i32, ArgT) -> Result<TO> + 'static>;

    let mut outArray: metamodelica::Array<TO>;
    let mut len: i32 = (inArray.clone().borrow().len() as i32);
    let mut res: TO;
    if len.clone() == 0 {
        outArray = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
    } else {
        res = inFunc(inArray.clone().borrow()[(1-1) as usize].clone(), 1, inArg.clone())?;
        outArray = metamodelica::arrayCreate(len.clone(), res.clone());
        unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), 1, res.clone()) };
        for mut i in 2..=len.clone() {
            unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), i.clone(), inFunc(inArray.clone().borrow()[(i.clone()-1) as usize].clone(), i.clone(), inArg.clone())?) };
        }
    }
    Ok(outArray)
}

pub fn mapList<TI: Clone + 'static, TO: Clone + 'static>(mut inList: Arc<metamodelica::List<TI>>, mut inFunc: Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>) -> Result<metamodelica::Array<TO>> {
    pub type FuncType<TI: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI) -> Result<TO> + 'static>;

    let mut outArray: metamodelica::Array<TO>;
    let mut i: i32 = 2;
    let mut len: i32 = (inList.clone().len() as i32);
    let mut res: TO;
    if len.clone() == 0 {
        outArray = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
    } else {
        res = inFunc(listHead(inList.clone())?)?;
        outArray = metamodelica::arrayCreate(len.clone(), res.clone());
        unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), 1, res.clone()) };
        for mut e in &*listRest(inList.clone())? {
            let mut e = e.clone();
            unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), i.clone(), inFunc(e.clone())?) };
            i = i.clone() + 1;
        }
    }
    Ok(outArray)
}

pub fn fold<T: Clone + 'static, FoldT: Clone + 'static>(mut inArray: metamodelica::Array<T>, mut inFoldFunc: Arc<dyn ::std::ops::Fn(T, FoldT) -> Result<FoldT> + 'static>, mut inStartValue: FoldT) -> FoldT {
    pub type FoldFunc<T: Clone + 'static, FoldT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, FoldT) -> Result<FoldT> + 'static>;

    let mut outResult: FoldT = inStartValue.clone();
    let __range0 = inArray.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        outResult = inFoldFunc(e.clone(), outResult.clone()).unwrap();
    }
    outResult
}

pub fn foldIndex<T: Clone + 'static, FoldT: Clone + 'static>(mut inArray: metamodelica::Array<T>, mut inFoldFunc: Arc<dyn ::std::ops::Fn(T, i32, FoldT) -> Result<FoldT> + 'static>, mut inStartValue: FoldT) -> Result<FoldT> {
    pub type FoldFunc<T: Clone + 'static, FoldT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, i32, FoldT) -> Result<FoldT> + 'static>;

    let mut outResult: FoldT = inStartValue.clone();
    let mut e: T;
    let __range0 = 1..=(inArray.clone().borrow().len() as i32);
    for mut i in __range0 {
        e = inArray.clone().borrow()[(i.clone()-1) as usize].clone();
        outResult = inFoldFunc(e.clone(), i.clone(), outResult.clone())?;
    }
    Ok(outResult)
}

pub fn reduce<T: Clone + 'static>(mut inArray: metamodelica::Array<T>, mut inReduceFunc: Arc<dyn ::std::ops::Fn(T, T) -> Result<T> + 'static>) -> Result<T> {
    pub type ReduceFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<T> + 'static>;

    let mut outResult: T;
    outResult = inArray.clone().borrow()[(1-1) as usize].clone();
    let __range0 = 2..=(inArray.clone().borrow().len() as i32);
    for mut i in __range0 {
        outResult = inReduceFunc(outResult.clone(), inArray.clone().borrow()[(i.clone()-1) as usize].clone())?;
    }
    Ok(outResult)
}

pub fn updateIndexFirst<T: Clone + 'static>(mut inIndex: i32, mut inValue: T, mut inArray: metamodelica::Array<T>) -> Result<()> {
    {let _arr = inArray.clone(); _arr.borrow_mut()[(inIndex.clone()-1) as usize] = inValue.clone(); _arr};
    Ok(())
}

pub fn getIndexFirst<T: Clone + 'static>(mut inIndex: i32, mut inArray: metamodelica::Array<T>) -> T {
    let mut outElement: T = inArray.clone().borrow()[(inIndex.clone()-1) as usize].clone();
    outElement
}

pub fn replaceAtWithFill<T: Clone + 'static>(mut inPos: i32, mut inTypeReplace: T, mut inTypeFill: T, mut inArray: metamodelica::Array<T>) -> Result<metamodelica::Array<T>> {
    let mut outArray: metamodelica::Array<T>;
    outArray = expandToSize(inPos.clone(), inArray.clone(), inTypeFill.clone())?;
    {let _arr = outArray.clone(); _arr.borrow_mut()[(inPos.clone()-1) as usize] = inTypeReplace.clone(); _arr};
    Ok(outArray)
}

pub fn expandToSize<T: Clone + 'static>(mut inNewSize: i32, mut inArray: metamodelica::Array<T>, mut inFill: T) -> Result<metamodelica::Array<T>> {
    let mut outArray: metamodelica::Array<T>;
    if inNewSize.clone() <= (inArray.clone().borrow().len() as i32) {
        outArray = inArray.clone();
    } else {
        outArray = arrayCreate(inNewSize.clone(), inFill.clone());
        copy(inArray.clone(), outArray.clone())?;
    }
    Ok(outArray)
}

pub fn expand<T: Clone + 'static>(mut inN: i32, mut inArray: metamodelica::Array<T>, mut inFill: T) -> Result<metamodelica::Array<T>> {
    let mut outArray: metamodelica::Array<T>;
    let mut len: i32 = 0;
    if inN.clone() < 1 {
        outArray = inArray.clone();
    } else {
        len = (inArray.clone().borrow().len() as i32);
        outArray = metamodelica::arrayCreate(len.clone() + inN.clone(), inFill.clone());
        copy(inArray.clone(), outArray.clone())?;
        setRange(len.clone() + 1, len.clone() + inN.clone(), outArray.clone(), inFill.clone())?;
    }
    Ok(outArray)
}

pub fn expandOnDemand<T: Clone + 'static>(mut inNewSize: i32, mut inArray: metamodelica::Array<T>, mut inExpansionFactor: metamodelica::Real, mut inFillValue: T) -> Result<metamodelica::Array<T>> {
    let mut outArray: metamodelica::Array<T>;
    let mut new_size: i32 = 0;
    let mut len: i32 = (inArray.clone().borrow().len() as i32);
    if inNewSize.clone() <= len.clone() {
        outArray = inArray.clone();
    } else {
        new_size = ((intReal(len.clone()) * inExpansionFactor.clone()).0 as i32);
        outArray = metamodelica::arrayCreate(new_size.clone(), inFillValue.clone());
        copy(inArray.clone(), outArray.clone())?;
        setRange(len.clone() + 1, new_size.clone(), outArray.clone(), inFillValue.clone())?;
    }
    Ok(outArray)
}

pub fn consToElement<T: Clone + 'static>(mut inIndex: i32, mut inElement: T, mut inArray: metamodelica::Array<Arc<metamodelica::List<T>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<T>>>> {
    let mut outArray: metamodelica::Array<Arc<metamodelica::List<T>>>;
    outArray = {let _arr = inArray.clone(); let _val = cons(inElement.clone(), inArray.borrow()[(inIndex.clone()-1) as usize].clone()); _arr.borrow_mut()[(inIndex.clone()-1) as usize] = _val; _arr};
    Ok(outArray)
}

pub fn appendToElement<T: Clone + 'static>(mut inIndex: i32, mut inElements: Arc<metamodelica::List<T>>, mut inArray: metamodelica::Array<Arc<metamodelica::List<T>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<T>>>> {
    let mut outArray: metamodelica::Array<Arc<metamodelica::List<T>>>;
    outArray = {let _arr = inArray.clone(); let _val = listAppend(inArray.borrow()[(inIndex.clone()-1) as usize].clone(), inElements.clone()); _arr.borrow_mut()[(inIndex.clone()-1) as usize] = _val; _arr};
    Ok(outArray)
}

pub fn appendList<T: Clone + 'static>(mut arr: metamodelica::Array<T>, mut lst: Arc<metamodelica::List<T>>) -> Result<metamodelica::Array<T>> {
    let mut outArray: metamodelica::Array<T>;
    let mut arr_len: i32 = (arr.clone().borrow().len() as i32);
    let mut lst_len: i32 = 0;
    let mut e: T;
    let mut rest: Arc<metamodelica::List<T>> = metamodelica::nil();
    if lst.clone().is_empty() {
        outArray = arr.clone();
    } else if arr_len.clone() == 0 {
        outArray = metamodelica::arrayFromVec(lst.clone().into_iter().cloned().collect());
    } else {
        lst_len = (lst.clone().len() as i32);
        outArray = metamodelica::arrayCreate(arr_len.clone() + lst_len.clone(), arr.borrow()[(1-1) as usize].clone());
        copy(arr.clone(), outArray.clone())?;
        rest = lst.clone();
        for mut i in arr_len.clone() + 1..=arr_len.clone() + lst_len.clone() {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e = __pa0.clone();
            rest = __pa1.clone();
            unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), i.clone(), e.clone()) };
        }
    }
    Ok(outArray)
}

pub fn join<T: Clone + 'static>(mut arr1: metamodelica::Array<T>, mut arr2: metamodelica::Array<T>) -> Result<metamodelica::Array<T>> {
    let mut outArray: metamodelica::Array<T>;
    let mut len1: i32 = (arr1.clone().borrow().len() as i32);
    let mut len2: i32 = (arr2.clone().borrow().len() as i32);
    if len1.clone() == 0 {
        outArray = metamodelica::arrayFromVec(arr2.clone().borrow().clone());
    } else if len2.clone() == 0 {
        outArray = metamodelica::arrayFromVec(arr1.clone().borrow().clone());
    } else {
        outArray = metamodelica::arrayCreate(len1.clone() + len2.clone(), arr1.borrow()[(1-1) as usize].clone());
        copyRange(arr1.clone(), outArray.clone(), 1, len1.clone(), 1)?;
        copyRange(arr2.clone(), outArray.clone(), 1, len2.clone(), len1.clone() + 1)?;
    }
    Ok(outArray)
}

pub fn copy<T: Clone + 'static>(mut inArraySrc: metamodelica::Array<T>, mut inArrayDest: metamodelica::Array<T>) -> Result<metamodelica::Array<T>> {
    let mut outArray: metamodelica::Array<T> = inArrayDest.clone();
    if (inArraySrc.clone().borrow().len() as i32) > (inArrayDest.clone().borrow().len() as i32) {
        bail!("fail");
    }
    let __range0 = 1..=(inArraySrc.clone().borrow().len() as i32);
    for mut i in __range0 {
        {let _arr = outArray.clone(); let _val = inArraySrc.clone().borrow()[(i.clone()-1) as usize].clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = _val; _arr};
    }
    Ok(outArray)
}

pub fn copyN<T: Clone + 'static>(mut inArraySrc: metamodelica::Array<T>, mut inArrayDest: metamodelica::Array<T>, mut inN: i32, mut srcOffset: i32, mut dstOffset: i32) -> Result<metamodelica::Array<T>> {
    let mut outArray: metamodelica::Array<T> = inArrayDest.clone();
    if inN.clone() + dstOffset.clone() > (inArrayDest.clone().borrow().len() as i32) || inN.clone() + srcOffset.clone() > (inArraySrc.clone().borrow().len() as i32) {
        bail!("fail");
    }
    for mut i in 1..=inN.clone() {
        {let _arr = outArray.clone(); let _val = inArraySrc.clone().borrow()[(i.clone() + srcOffset.clone()-1) as usize].clone(); _arr.borrow_mut()[(i.clone() + dstOffset.clone()-1) as usize] = _val; _arr};
    }
    Ok(outArray)
}

pub fn copyRange<T: Clone + 'static>(mut srcArray: metamodelica::Array<T>, mut dstArray: metamodelica::Array<T>, mut srcFirst: i32, mut srcLast: i32, mut dstPos: i32) -> Result<()> {
    let mut offset: i32 = dstPos.clone() - srcFirst.clone();
    if srcFirst.clone() > srcLast.clone() || srcLast.clone() > (srcArray.clone().borrow().len() as i32) || offset.clone() + srcLast.clone() > (dstArray.clone().borrow().len() as i32) {
        bail!("fail");
    }
    for mut i in srcFirst.clone()..=srcLast.clone() {
        {let _arr = dstArray.clone(); let _val = srcArray.clone().borrow()[(i.clone()-1) as usize].clone(); _arr.borrow_mut()[(offset.clone() + i.clone()-1) as usize] = _val; _arr};
    }
    Ok(())
}

pub fn createIntRange(mut inLen: i32) -> metamodelica::Array<i32> {
    let mut outArray: metamodelica::Array<i32>;
    outArray = metamodelica::arrayCreate(inLen.clone(), 0);
    for mut i in 1..=inLen.clone() {
        unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), i.clone(), i.clone()) };
    }
    outArray
}

pub fn setRange<T: Clone + 'static>(mut inStart: i32, mut inEnd: i32, mut inArray: metamodelica::Array<T>, mut inValue: T) -> Result<metamodelica::Array<T>> {
    let mut outArray: metamodelica::Array<T> = inArray.clone();
    if inStart.clone() > (inArray.clone().borrow().len() as i32) {
        bail!("fail");
    }
    for mut i in inStart.clone()..=inEnd.clone() {
        {let _arr = inArray.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = inValue.clone(); _arr};
    }
    Ok(outArray)
}

pub fn getRange<T: Clone + 'static>(mut inStart: i32, mut inEnd: i32, mut inArray: metamodelica::Array<T>) -> Result<Arc<metamodelica::List<T>>> {
    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut value: T;
    if inStart.clone() > (inArray.clone().borrow().len() as i32) {
        bail!("fail");
    }
    for mut i in inStart.clone()..=inEnd.clone() {
        value = inArray.clone().borrow()[(i.clone()-1) as usize].clone();
        outList = cons(value.clone(), outList.clone());
    }
    Ok(outList)
}

pub fn position<T: Clone + 'static + PartialEq>(mut inArray: metamodelica::Array<T>, mut inElement: T, mut inFilledSize: i32) -> i32 {
    let mut outIndex: i32 = 0;
    let mut e: T;
    for mut i in 1..=inFilledSize.clone() {
        if inElement.clone() == inArray.borrow()[(i.clone()-1) as usize].clone() {
            outIndex = i.clone();
            return outIndex;
        }
    }
    outIndex = 0;
    outIndex
}

pub fn getMemberOnTrue<VT: Clone + 'static, ET: Clone + 'static>(mut inValue: VT, mut inArray: metamodelica::Array<ET>, mut inCompFunc: Arc<dyn ::std::ops::Fn(VT, ET) -> Result<bool> + 'static>) -> Result<(ET, i32)> {
    pub type CompFunc<VT: Clone + 'static, ET: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(VT, ET) -> Result<bool> + 'static>;

    let mut outElement: ET;
    let mut outIndex: i32 = 0;
    let __range0 = 1..=(inArray.clone().borrow().len() as i32);
    for mut i in __range0 {
        if inCompFunc(inValue.clone(), inArray.clone().borrow()[(i.clone()-1) as usize].clone())? {
            outElement = inArray.clone().borrow()[(i.clone()-1) as usize].clone();
            outIndex = i.clone();
            return Ok((outElement, outIndex));
        }
    }
    bail!("fail");
    Ok((outElement, outIndex))
}

pub fn reverse<T: Clone + 'static>(mut inArray: metamodelica::Array<T>) -> Result<metamodelica::Array<T>> {
    let mut outArray: metamodelica::Array<T>;
    let mut size: i32 = 0;
    let mut i: i32 = 0;
    let mut elem1: T;
    let mut elem2: T;
    outArray = inArray.clone();
    size = (inArray.clone().borrow().len() as i32);
    for mut i in 1..=size.clone() / 2 {
        elem1 = inArray.clone().borrow()[(i.clone()-1) as usize].clone();
        elem2 = inArray.clone().borrow()[(size.clone() - i.clone() + 1-1) as usize].clone();
        outArray = {let _arr = outArray.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = elem2.clone(); _arr};
        outArray = {let _arr = outArray.clone(); _arr.borrow_mut()[(size.clone() - i.clone() + 1-1) as usize] = elem1.clone(); _arr};
    }
    Ok(outArray)
}

pub fn toString<T: Clone + 'static>(mut inArray: metamodelica::Array<T>, mut inPrintFunc: Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>, mut inNameStr: ArcStr, mut inBeginStr: ArcStr, mut inDelimitStr: ArcStr, mut inEndStr: ArcStr, mut inPrintEmpty: bool, mut maxLength: i32) -> Result<ArcStr> {
    pub type FuncType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>;

    let mut outString: ArcStr = arcstr::literal!("");
    let mut lst: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut endStr: ArcStr = inEndStr.clone();
    if maxLength.clone() > 0 && (inArray.clone().borrow().len() as i32) > maxLength.clone() {
        lst = List::firstN(Arc::new(inArray.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), maxLength.clone())?;
        endStr = stringAppendList(list![(inDelimitStr.clone()).clone(), (literal!("...")).clone(), (inEndStr.clone()).clone()]);
    } else {
        lst = Arc::new(inArray.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
    }
    outString = ((::match_deref::match_deref! { match &((lst.clone(), inPrintEmpty.clone())) {
        (Deref @ metamodelica::List::Nil, true) => {
            stringAppendList(list![(inNameStr.clone()).clone(), (inBeginStr.clone()).clone(), (inEndStr.clone()).clone()])
        },
        (Deref @ metamodelica::List::Nil, false) => {
            inNameStr.clone()
        },
        _ => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = stringDelimitList(List::map(lst.clone(), inPrintFunc.clone()), (inDelimitStr.clone()).clone());
            r#str = stringAppendList(list![(inNameStr.clone()).clone(), (inBeginStr.clone()).clone(), (r#str.clone()).clone(), (endStr.clone()).clone()]);
            r#str.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

pub fn isEqual<T: Clone + 'static + PartialEq>(mut inArr1: metamodelica::Array<T>, mut inArr2: metamodelica::Array<T>) -> Result<bool> {
    let mut outIsEqual: bool = true;
    let mut arrLength: i32 = 0;
    arrLength = (inArr1.clone().borrow().len() as i32);
    if !(intEq(arrLength.clone(), (inArr2.clone().borrow().len() as i32))) {
        bail!("fail");
    }
    for mut i in 1..=arrLength.clone() {
        if !(inArr1.borrow()[(i.clone()-1) as usize].clone() == inArr2.borrow()[(i.clone()-1) as usize].clone()) {
            outIsEqual = false;
            break;
        }
    }
    Ok(outIsEqual)
}

pub fn isEqualOnTrue<T1: Clone + 'static, T2: Clone + 'static>(mut arr1: metamodelica::Array<T1>, mut arr2: metamodelica::Array<T2>, mut pred: Arc<dyn ::std::ops::Fn(T1, T2) -> Result<bool> + 'static>) -> bool {
    pub type PredFunc<T1: Clone + 'static, T2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1, T2) -> Result<bool> + 'static>;

    let mut equal: bool = false;
    equal = (arr1.clone().borrow().len() as i32) == (arr2.clone().borrow().len() as i32);
    if !(equal.clone()) {
        return equal;
    }
    let __range0 = 1..=(arr1.clone().borrow().len() as i32);
    for mut i in __range0 {
        if !(pred(arr1.clone().borrow()[(i.clone()-1) as usize].clone(), arr2.clone().borrow()[(i.clone()-1) as usize].clone()).unwrap()) {
            equal = false;
            return equal;
        }
    }
    equal
}

pub fn allEqual<T: Clone + 'static>(mut arr: metamodelica::Array<T>, mut pred: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>) -> bool {
    pub type PredFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut equal: bool = true;
    if arr.clone().borrow().is_empty() {
        return equal;
    }
    let __range0 = 2..=(arr.clone().borrow().len() as i32);
    for mut i in __range0 {
        if !(pred(arr.clone().borrow()[(1-1) as usize].clone(), arr.clone().borrow()[(i.clone()-1) as usize].clone()).unwrap()) {
            equal = false;
            return equal;
        }
    }
    equal
}

pub fn isLess<T: Clone + 'static>(mut arr1: metamodelica::Array<T>, mut arr2: metamodelica::Array<T>, mut lessFn: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>) -> bool {
    pub type LessFn<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut res: bool = false;
    let mut len1: i32 = 0;
    let mut len2: i32 = 0;
    let mut e1: T;
    let mut e2: T;
    len1 = (arr1.clone().borrow().len() as i32);
    len2 = (arr2.clone().borrow().len() as i32);
    for mut i in 1..=std::cmp::min(len1.clone(), len2.clone()) {
        e1 = arr1.clone().borrow()[(i.clone()-1) as usize].clone();
        e2 = arr2.clone().borrow()[(i.clone()-1) as usize].clone();
        if lessFn(e1.clone(), e2.clone()).unwrap() {
            res = true;
            return res;
        } else if lessFn(e2.clone(), e1.clone()).unwrap() {
            res = false;
            return res;
        }
    }
    res = len1.clone() < len2.clone();
    res
}

pub fn insertList<T: Clone + 'static>(mut arr: metamodelica::Array<T>, mut lst: Arc<metamodelica::List<T>>, mut startPos: i32) -> metamodelica::Array<T> {
    let mut arr: metamodelica::Array<T> = arr;
    let mut i: i32 = startPos.clone();
    for mut e in &*lst.clone() {
        let mut e = e.clone();
        {
            let __cell0 = e.clone();
            arr.clone().borrow_mut()[(i.clone()-1) as usize] = __cell0;
        }
        i = i.clone() + 1;
    }
    arr
}

pub fn remove<T: Clone + 'static>(mut arr: metamodelica::Array<T>, mut index: i32) -> Result<metamodelica::Array<T>> {
    let mut outArr: metamodelica::Array<T>;
    let mut len: i32 = (arr.clone().borrow().len() as i32);
    let true = (index.clone() <= len.clone() && index.clone() >= 1) else { bail!("pattern mismatch") };
    if len.clone() <= 1 {
        outArr = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
    } else {
        outArr = metamodelica::arrayCreate(len.clone() - 1, arr.borrow()[(1-1) as usize].clone());
        for mut i in 1..=index.clone() - 1 {
            unsafe { metamodelica::Dangerous::arrayInitSlot(outArr.clone(), i.clone(), arr.clone().borrow()[(i.clone()-1) as usize].clone()) };
        }
        for mut i in index.clone() + 1..=len.clone() {
            unsafe { metamodelica::Dangerous::arrayInitSlot(outArr.clone(), i.clone() - 1, arr.clone().borrow()[(i.clone()-1) as usize].clone()) };
        }
    }
    Ok(outArr)
}

pub fn all<T: Clone + 'static>(mut arr: metamodelica::Array<T>, mut inFunc: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> bool {
    pub type PredFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outResult: bool = false;
    let __range0 = arr.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        if !(inFunc(e.clone()).unwrap()) {
            outResult = false;
            return outResult;
        }
    }
    outResult = true;
    outResult
}

pub fn any<T: Clone + 'static>(mut arr: metamodelica::Array<T>, mut inFunc: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> bool {
    pub type PredFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut outResult: bool = false;
    let __range0 = arr.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        if inFunc(e.clone()).unwrap() {
            outResult = true;
            return outResult;
        }
    }
    outResult = false;
    outResult
}

pub fn minElement<T: Clone + 'static>(mut arr: metamodelica::Array<T>, mut lessFn: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>) -> T {
    pub type LessFn<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut res: T;
    let mut e: T;
    res = arr.borrow()[(1-1) as usize].clone();
    let __range0 = 2..=(arr.clone().borrow().len() as i32);
    for mut i in __range0 {
        e = arr.clone().borrow()[(i.clone()-1) as usize].clone();
        if lessFn(e.clone(), res.clone()).unwrap() {
            res = e.clone();
        }
    }
    res
}

pub fn maxElement<T: Clone + 'static>(mut arr: metamodelica::Array<T>, mut lessFn: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>) -> T {
    pub type LessFn<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut res: T;
    let mut e: T;
    res = arr.borrow()[(1-1) as usize].clone();
    let __range0 = 2..=(arr.clone().borrow().len() as i32);
    for mut i in __range0 {
        e = arr.clone().borrow()[(i.clone()-1) as usize].clone();
        if lessFn(res.clone(), e.clone()).unwrap() {
            res = e.clone();
        }
    }
    res
}

pub fn compare<T1: Clone + 'static, T2: Clone + 'static>(mut arr1: metamodelica::Array<T1>, mut arr2: metamodelica::Array<T2>, mut compFn: Arc<dyn ::std::ops::Fn(T1, T2) -> Result<i32> + 'static>) -> i32 {
    pub type CompFunc<T1: Clone + 'static, T2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1, T2) -> Result<i32> + 'static>;

    let mut res: i32 = 0;
    let mut l1: i32 = 0;
    let mut l2: i32 = 0;
    l1 = (arr1.clone().borrow().len() as i32);
    l2 = (arr2.clone().borrow().len() as i32);
    res = if (l1.clone() == l2.clone()) {0} else if (l1.clone() > l2.clone()) {1} else {-1};
    if res.clone() != 0 {
        return res;
    }
    for mut i in 1..=l1.clone() {
        res = compFn(arr1.clone().borrow()[(i.clone()-1) as usize].clone(), arr2.clone().borrow()[(i.clone()-1) as usize].clone()).unwrap();
        if res.clone() != 0 {
            return res;
        }
    }
    res
}

pub fn mapFold<TI: Clone + 'static, TO: Clone + 'static, ArgT: Clone + 'static>(mut arr: metamodelica::Array<TI>, mut func: Arc<dyn ::std::ops::Fn(TI, ArgT) -> Result<(TO, ArgT)> + 'static>, mut arg: ArgT) -> (metamodelica::Array<TO>, ArgT) {
    pub type FuncType<TI: Clone + 'static, ArgT: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(TI, ArgT) -> Result<(TO, ArgT)> + 'static>;

    let mut outArray: metamodelica::Array<TO>;
    let mut outArg: ArgT = arg.clone();
    let mut len: i32 = (arr.clone().borrow().len() as i32);
    let mut res: TO;
    if len.clone() == 0 {
        outArray = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
    } else {
        (res, outArg) = func(arr.clone().borrow()[(1-1) as usize].clone(), outArg.clone()).unwrap();
        outArray = metamodelica::arrayCreate(len.clone(), res.clone());
        unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), 1, res.clone()) };
        for mut i in 2..=len.clone() {
            (res, outArg) = func(arr.clone().borrow()[(i.clone()-1) as usize].clone(), outArg.clone()).unwrap();
            unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), i.clone(), res.clone()) };
        }
    }
    (outArray, outArg)
}

pub fn transpose<T: Clone + 'static>(mut arr: metamodelica::Array<metamodelica::Array<T>>) -> metamodelica::Array<metamodelica::Array<T>> {
    let mut outArray: metamodelica::Array<metamodelica::Array<T>>;
    let mut c_len: i32 = 0;
    let mut r_len: i32 = 0;
    let mut val: T;
    let mut row: metamodelica::Array<T>;
    if arr.clone().borrow().is_empty() {
        outArray = arr.clone();
        return outArray;
    }
    row = arr.clone().borrow()[(1-1) as usize].clone();
    if row.clone().borrow().is_empty() {
        outArray = arr.clone();
        return outArray;
    }
    val = row.clone().borrow()[(1-1) as usize].clone();
    c_len = (arr.clone().borrow().len() as i32);
    r_len = (row.clone().borrow().len() as i32);
    outArray = metamodelica::arrayCreate(r_len.clone(), row.clone());
    for mut i in 1..=r_len.clone() {
        unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), i.clone(), metamodelica::arrayCreate(c_len.clone(), val.clone())) };
    }
    for mut r in 1..=r_len.clone() {
        for mut c in 1..=c_len.clone() {
            val = arr.clone().borrow()[(c.clone()-1) as usize].clone().borrow()[(r.clone()-1) as usize].clone();
            {let _arr = outArray.clone().borrow()[(r.clone()-1) as usize].clone(); _arr.borrow_mut()[(c.clone()-1) as usize] = val.clone(); _arr};
        }
    }
    outArray
}

pub fn threadMap<T1: Clone + 'static, T2: Clone + 'static, TO: Clone + 'static>(mut arr1: metamodelica::Array<T1>, mut arr2: metamodelica::Array<T2>, mut func: Arc<dyn ::std::ops::Fn(T1, T2) -> Result<TO> + 'static>) -> Result<metamodelica::Array<TO>> {
    pub type MapFunc<T1: Clone + 'static, T2: Clone + 'static, TO: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1, T2) -> Result<TO> + 'static>;

    let mut outArray: metamodelica::Array<TO>;
    let mut res: TO;
    let mut len1: i32 = 0;
    let mut len2: i32 = 0;
    if arr1.clone().borrow().is_empty() {
        outArray = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
        return Ok(outArray);
    }
    len1 = (arr1.clone().borrow().len() as i32);
    len2 = (arr2.clone().borrow().len() as i32);
    if len1.clone() != len2.clone() {
        bail!("fail");
    }
    res = func(arr1.clone().borrow()[(1-1) as usize].clone(), arr2.clone().borrow()[(1-1) as usize].clone())?;
    outArray = metamodelica::arrayCreate(len1.clone(), res.clone());
    unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), 1, res.clone()) };
    for mut i in 2..=len1.clone() {
        unsafe { metamodelica::Dangerous::arrayInitSlot(outArray.clone(), i.clone(), func(arr1.clone().borrow()[(i.clone()-1) as usize].clone(), arr2.clone().borrow()[(i.clone()-1) as usize].clone())?) };
    }
    Ok(outArray)
}

pub fn generate<T: Clone + 'static>(mut n: i32, mut generator: Arc<dyn ::std::ops::Fn() -> Result<T> + 'static>) -> metamodelica::Array<T> {
    pub type Generator<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn() -> Result<T> + 'static>;

    let mut arr: metamodelica::Array<T>;
    let mut e: T;
    if n.clone() <= 0 {
        arr = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
    } else {
        e = generator().unwrap();
        arr = metamodelica::arrayCreate(n.clone(), e.clone());
        unsafe { metamodelica::Dangerous::arrayInitSlot(arr.clone(), 1, e.clone()) };
        for mut i in 2..=n.clone() {
            unsafe { metamodelica::Dangerous::arrayInitSlot(arr.clone(), i.clone(), generator().unwrap()) };
        }
    }
    arr
}

pub fn filter<T: Clone + 'static + Default>(mut arr: metamodelica::Array<T>, mut fun: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> metamodelica::Array<T> {
    pub type filterFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut new_arr: metamodelica::Array<T>;
    let mut new_size: i32 = 0;
    let mut dummy: T;
    let mut index: i32 = 1;
    new_size = (arr.clone().borrow().len() as i32) - {
        let mut __acc: i32 = 0;
        for mut e in (arr.clone()).borrow().iter() {
            if !(fun(e.clone()).unwrap()) { continue; }
            let __x = 1;
            __acc += __x;
        }
        __acc
    };
    new_arr = metamodelica::arrayCreateDefault(new_size.clone());
    let __range0 = arr.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        if !(fun(e.clone()).unwrap()) {
            unsafe { metamodelica::Dangerous::arrayInitSlot(new_arr.clone(), index.clone(), e.clone()) };
            index = index.clone() + 1;
        }
    }
    new_arr
}

