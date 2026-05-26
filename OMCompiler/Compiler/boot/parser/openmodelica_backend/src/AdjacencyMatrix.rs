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

use crate::BackendDAE;
use openmodelica_util::Debug;
use openmodelica_util::Flags;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

pub fn copyAdjacencyMatrix(mut inAdjacencyMatrix: Option<metamodelica::Array<Arc<metamodelica::List<i32>>>>) -> Option<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut outAdjacencyMatrix: Option<metamodelica::Array<Arc<metamodelica::List<i32>>>> = None;
    outAdjacencyMatrix = (match inAdjacencyMatrix.clone() {
        Some(mut m) => {
            m = metamodelica::arrayFromVec(m.clone().borrow().clone());
            Some(m.clone())
        },
        _ => {
            None
        },
    });
    outAdjacencyMatrix
}

pub use copyAdjacencyMatrix as copyAdjacencyMatrixT;

pub fn traverseAdjacencyMatrix<T: Clone + 'static>(mut inM: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32, T) -> Result<(Arc<metamodelica::List<i32>>, T)> + 'static>, mut inTypeA: T) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, T)> {
    pub type FuncType<T: Clone> = fn(Arc<metamodelica::List<i32>>, i32, T) -> Result<(Arc<metamodelica::List<i32>>, T)>;

    let mut outM: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut outTypeA: T;
    (outM, outTypeA) = traverseAdjacencyMatrix1(inM.clone(), func.clone(), 1, (inM.clone().borrow().len() as i32), inTypeA.clone())?;
    Ok((outM, outTypeA))
}

fn traverseAdjacencyMatrix1<T: Clone + 'static>(mut inM: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32, T) -> Result<(Arc<metamodelica::List<i32>>, T)> + 'static>, mut pos: i32, mut len: i32, mut inTypeA: T) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, T)> {
    pub type FuncType<T: Clone> = fn(Arc<metamodelica::List<i32>>, i32, T) -> Result<(Arc<metamodelica::List<i32>>, T)>;

    let mut outM: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut outTypeA: T;
    (outM, outTypeA) = traverseAdjacencyMatrix2(inM.clone(), func.clone(), pos.clone(), len.clone(), intGt(pos.clone(), len.clone()), inTypeA.clone())?;
    Ok((outM, outTypeA))
}

fn traverseAdjacencyMatrix2<T: Clone + 'static>(mut inM: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32, T) -> Result<(Arc<metamodelica::List<i32>>, T)> + 'static>, mut pos: i32, mut len: i32, mut stop: bool, mut inTypeA: T) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, T)> {
    pub type FuncType<T: Clone> = fn(Arc<metamodelica::List<i32>>, i32, T) -> Result<(Arc<metamodelica::List<i32>>, T)>;

    let mut outM: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut outTypeA: T;
    (outM, outTypeA) = (match stop.clone() {
        true => {
            (inM.clone(), inTypeA.clone())
        },
        false => {
            let mut m1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut m2: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut extArg: T;
            let mut extArg1: T;
            let mut extArg2: T;
            let mut eqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut eqns1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            (eqns, extArg) = func(inM.borrow()[(pos.clone()-1) as usize].clone(), pos.clone(), inTypeA.clone())?;
            eqns1 = List::removeOnTrue(pos.clone(), Arc::new(fnptr!(intLt, i32, i32)), eqns.clone());
            (m1, extArg1) = traverseAdjacencyMatrixList(eqns1.clone(), inM.clone(), func.clone(), (inM.clone().borrow().len() as i32), pos.clone(), extArg.clone())?;
            (m2, extArg2) = traverseAdjacencyMatrix2(m1.clone(), func.clone(), pos.clone() + 1, len.clone(), intGt(pos.clone() + 1, len.clone()), extArg1.clone())?;
            (m2.clone(), extArg2.clone())
        },
    });
    Ok((outM, outTypeA))
}

fn traverseAdjacencyMatrixList<T: Clone + 'static>(mut inLst: Arc<metamodelica::List<i32>>, mut inM: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32, T) -> Result<(Arc<metamodelica::List<i32>>, T)> + 'static>, mut len: i32, mut maxpos: i32, mut inTypeA: T) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, T)> {
    pub type FuncType<T: Clone> = fn(Arc<metamodelica::List<i32>>, i32, T) -> Result<(Arc<metamodelica::List<i32>>, T)>;

    let mut outM: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut outTypeA: T;
    (outM, outTypeA) = 'mc: {
        let __mc_input = inLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((inM.clone(), inTypeA.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: pos, tail: rest } => {
                    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut extArg: T;
                    let mut extArg1: T;
                    let mut eqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut eqns1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut alleqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let true = (intLt(pos.clone(), len.clone() + 1)) else { bail!("pattern mismatch") };
                    let true = (intLt(pos.clone(), maxpos.clone())) else { bail!("pattern mismatch") };
                    (eqns, extArg) = func(inM.borrow()[(pos.clone()-1) as usize].clone(), pos.clone(), inTypeA.clone())?;
                    eqns1 = List::removeOnTrue(maxpos.clone(), Arc::new(fnptr!(intLt, i32, i32)), eqns.clone());
                    alleqns = List::unionOnTrueList(list![rest.clone(), eqns1.clone()], Arc::new(fnptr!(intEq, i32, i32)))?;
                    (m, extArg1) = traverseAdjacencyMatrixList(alleqns.clone(), inM.clone(), func.clone(), len.clone(), maxpos.clone(), extArg.clone())?;
                    Ok((m.clone(), extArg1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: pos, tail: rest } => {
                    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut extArg: T;
                    let true = (intLt(pos.clone(), len.clone() + 1)) else { bail!("pattern mismatch") };
                    (m, extArg) = traverseAdjacencyMatrixList(rest.clone(), inM.clone(), func.clone(), len.clone(), maxpos.clone(), inTypeA.clone())?;
                    Ok((m.clone(), extArg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- BackendDAEOptimize.traverseAdjacencyMatrixList failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outM, outTypeA))
}

#[tailcall::tailcall]
pub fn getOtherEqSysAdjacencyMatrix(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut size: i32, mut index: i32, mut skip: metamodelica::Array<i32>, mut rowskip: metamodelica::Array<i32>, mut mnew: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    match m.clone() {
        _ if (intGt(index.clone(), size.clone())) => {
            Ok(mnew.clone())
        },
        _ if (intGt(skip.borrow()[(index.clone()-1) as usize].clone(), 0)) => {
            let mut row: Arc<metamodelica::List<i32>> = metamodelica::nil();
            row = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut r in (m.borrow()[(index.clone()-1) as usize].clone()).into_iter().cloned() {
            if !(intGt(r.clone(), 0) && intGt(rowskip.borrow()[(r.clone()-1) as usize].clone(), 0)) { continue; }
            let __x = r.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            {let _arr = mnew.clone(); _arr.borrow_mut()[(index.clone()-1) as usize] = row.clone(); _arr};
            tailcall::call!{ getOtherEqSysAdjacencyMatrix(m.clone(), size.clone(), index.clone() + 1, skip.clone(), rowskip.clone(), mnew.clone()) }
        },
        _ => {
            {let _arr = mnew.clone(); _arr.borrow_mut()[(index.clone()-1) as usize] = metamodelica::nil(); _arr};
            tailcall::call!{ getOtherEqSysAdjacencyMatrix(m.clone(), size.clone(), index.clone() + 1, skip.clone(), rowskip.clone(), mnew.clone()) }
        },
    }
}

fn isAssigned(mut ass: metamodelica::Array<i32>, mut i: i32) -> bool {
    let mut b: bool = false;
    b = intGt(ass.borrow()[(i.clone()-1) as usize].clone(), 0);
    b
}

pub fn transposeAdjacencyMatrix(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut nRowsMt: i32) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut i: i32 = 1;
    mt = arrayCreate(nRowsMt.clone(), metamodelica::nil());
    let __range0 = m.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        (mt, i) = transposeRow(e.clone(), mt.clone(), i.clone())?;
    }
    Ok(mt)
}

fn transposeRow(mut row: Arc<metamodelica::List<i32>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut indx: i32) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, i32)> {
    let mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>> = mt;
    let mut indx: i32 = indx;
    (mt, indx) = (::match_deref::match_deref! { match &(row.clone()) {
        Deref @ metamodelica::List::Nil => {
            (mt.clone(), indx.clone() + 1)
        },
        Deref @ metamodelica::List::Cons { head: i, tail: res } => {
            let mut indx1: i32 = 0;
            let mut iabs: i32 = 0;
            let mut col: Arc<metamodelica::List<i32>> = metamodelica::nil();
            iabs = intAbs(i.clone());
            mt = Array::expand(iabs.clone() - (mt.clone().borrow().len() as i32), mt.clone(), metamodelica::nil())?;
            col = mt.borrow()[(iabs.clone()-1) as usize].clone();
            indx1 = if (intLt(i.clone(), 0)) {-(indx.clone())} else {indx.clone()};
            {let _arr = mt.clone(); _arr.borrow_mut()[(iabs.clone()-1) as usize] = cons(indx1.clone(), col.clone()); _arr};
            transposeRow(res.clone(), mt.clone(), indx.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((mt, indx))
}

pub fn absAdjacencyMatrix(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> metamodelica::Array<Arc<metamodelica::List<i32>>> {
    let mut res: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut lst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut lst_1: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut i: i32 = 1;
    let mut minn: i32 = 0;
    res = metamodelica::arrayCreate((m.clone().borrow().len() as i32), metamodelica::nil());
    let __range0 = m.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut v in __range0 {
        minn = List::fold(v.clone(), Arc::new(fnptr!(intMin, i32, i32)), 0);
        if minn.clone() < 0 {
            Dangerous::arrayUpdate(res.clone(), i.clone(), List::map(v.clone(), Arc::new(intAbs.clone()))).unwrap();
        } else {
            Dangerous::arrayUpdate(res.clone(), i.clone(), v.clone()).unwrap();
        }
        i = i.clone() + 1;
    }
    res
}

pub fn isEmpty(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> bool {
    let mut b: bool = true;
    let __range0 = m.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut element in __range0 {
        if !(element.clone().is_empty()) {
            b = false;
            return b;
        }
    }
    b
}

