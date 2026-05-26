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
use crate::BackendDump;
use crate::Matching;
use openmodelica_util_datatypes_basic::GCExt;

pub fn Tarjan(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut ass1: metamodelica::Array<i32>, mut N: i32) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut outComponents: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut index: i32 = 0;
    let mut stack: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut number: metamodelica::Array<i32>;
    let mut lowlink: metamodelica::Array<i32>;
    let mut onStack: metamodelica::Array<bool>;
    let mut eqn: i32 = 0;
    number = arrayCreate(N.clone(), -1);
    lowlink = arrayCreate(N.clone(), -1);
    onStack = arrayCreate(N.clone(), false);
    let __range0 = 1..=(ass1.clone().borrow().len() as i32);
    for mut var in __range0 {
        eqn = ass1.borrow()[(var.clone()-1) as usize].clone();
        if eqn.clone() > 0 && number.borrow()[(eqn.clone()-1) as usize].clone() == -1 {
            (stack, index, outComponents) = StrongConnect(m.clone(), ass1.clone(), eqn.clone(), stack.clone(), index.clone(), number.clone(), lowlink.clone(), onStack.clone(), outComponents.clone())?;
        }
    }
    GCExt::free(number.clone());
    GCExt::free(lowlink.clone());
    GCExt::free(onStack.clone());
    outComponents = outComponents.clone().reverse();
    Ok(outComponents)
}

fn StrongConnect(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut ass1: metamodelica::Array<i32>, mut eqn: i32, mut stack: Arc<metamodelica::List<i32>>, mut index: i32, mut number: metamodelica::Array<i32>, mut lowlink: metamodelica::Array<i32>, mut onStack: metamodelica::Array<bool>, mut inComponents: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)> {
    let mut outStack: Arc<metamodelica::List<i32>> = stack.clone();
    let mut outIndex: i32 = index.clone();
    let mut outComponents: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = inComponents.clone();
    let mut SCC: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqn2: i32 = 0;
    {let _arr = number.clone(); _arr.borrow_mut()[(eqn.clone()-1) as usize] = outIndex.clone(); _arr};
    {let _arr = lowlink.clone(); _arr.borrow_mut()[(eqn.clone()-1) as usize] = outIndex.clone(); _arr};
    {let _arr = onStack.clone(); _arr.borrow_mut()[(eqn.clone()-1) as usize] = true; _arr};
    outIndex = outIndex.clone() + 1;
    outStack = cons(eqn.clone(), outStack.clone());
    for mut eqn2 in &*Matching::incomingEquations(eqn.clone(), m.clone(), ass1.clone()) {
        let mut eqn2 = eqn2.clone();
        if number.borrow()[(eqn2.clone()-1) as usize].clone() == -1 {
            (outStack, outIndex, outComponents) = StrongConnect(m.clone(), ass1.clone(), eqn2.clone(), outStack.clone(), outIndex.clone(), number.clone(), lowlink.clone(), onStack.clone(), outComponents.clone())?;
            {let _arr = lowlink.clone(); let _val = intMin(lowlink.borrow()[(eqn.clone()-1) as usize].clone(), lowlink.borrow()[(eqn2.clone()-1) as usize].clone()); _arr.borrow_mut()[(eqn.clone()-1) as usize] = _val; _arr};
        } else if onStack.borrow()[(eqn2.clone()-1) as usize].clone() {
            {let _arr = lowlink.clone(); let _val = intMin(lowlink.borrow()[(eqn.clone()-1) as usize].clone(), number.borrow()[(eqn2.clone()-1) as usize].clone()); _arr.borrow_mut()[(eqn.clone()-1) as usize] = _val; _arr};
        }
    }
    if lowlink.borrow()[(eqn.clone()-1) as usize].clone() == number.borrow()[(eqn.clone()-1) as usize].clone() {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(outStack.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        eqn2 = __pa0.clone();
        outStack = __pa1.clone();
        {let _arr = onStack.clone(); _arr.borrow_mut()[(eqn2.clone()-1) as usize] = false; _arr};
        SCC = list![eqn2.clone()];
        while eqn.clone() != eqn2.clone() {
            let (__pa2, __pa3) = ::match_deref::match_deref! { match &(outStack.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            eqn2 = __pa2.clone();
            outStack = __pa3.clone();
            {let _arr = onStack.clone(); _arr.borrow_mut()[(eqn2.clone()-1) as usize] = false; _arr};
            SCC = cons(eqn2.clone(), SCC.clone());
        }
        outComponents = cons(metamodelica::Dangerous::listReverseInPlace(SCC.clone()), outComponents.clone());
    }
    Ok((outStack, outIndex, outComponents))
}

pub fn TarjanTransposed(mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut ass2: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut outComponents: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut index: i32 = 0;
    let mut stack: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut number: metamodelica::Array<i32>;
    let mut lowlink: metamodelica::Array<i32>;
    let mut onStack: metamodelica::Array<bool>;
    let mut N: i32 = (ass2.clone().borrow().len() as i32);
    number = arrayCreate(N.clone(), -1);
    lowlink = arrayCreate(N.clone(), -1);
    onStack = arrayCreate(N.clone(), false);
    for mut eqn in 1..=N.clone() {
        if number.borrow()[(eqn.clone()-1) as usize].clone() == -1 && ass2.borrow()[(eqn.clone()-1) as usize].clone() > 0 {
            (stack, index, outComponents) = StrongConnectTransposed(mT.clone(), ass2.clone(), eqn.clone(), stack.clone(), index.clone(), number.clone(), lowlink.clone(), onStack.clone(), outComponents.clone())?;
        }
    }
    Ok(outComponents)
}

fn StrongConnectTransposed(mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut ass2: metamodelica::Array<i32>, mut eqn: i32, mut stack: Arc<metamodelica::List<i32>>, mut index: i32, mut number: metamodelica::Array<i32>, mut lowlink: metamodelica::Array<i32>, mut onStack: metamodelica::Array<bool>, mut inComponents: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<(Arc<metamodelica::List<i32>>, i32, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)> {
    let mut outStack: Arc<metamodelica::List<i32>> = stack.clone();
    let mut outIndex: i32 = index.clone();
    let mut outComponents: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = inComponents.clone();
    let mut SCC: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut var: i32 = 0;
    let mut eqn2: i32 = 0;
    {let _arr = number.clone(); _arr.borrow_mut()[(eqn.clone()-1) as usize] = outIndex.clone(); _arr};
    {let _arr = lowlink.clone(); _arr.borrow_mut()[(eqn.clone()-1) as usize] = outIndex.clone(); _arr};
    {let _arr = onStack.clone(); _arr.borrow_mut()[(eqn.clone()-1) as usize] = true; _arr};
    outIndex = outIndex.clone() + 1;
    outStack = cons(eqn.clone(), outStack.clone());
    var = ass2.borrow()[(eqn.clone()-1) as usize].clone();
    if var.clone() > 0 {
        let __range0 = &*mT.borrow()[(var.clone()-1) as usize].clone();
        for mut eqn2 in __range0 {
            let mut eqn2 = eqn2.clone();
            if eqn2.clone() > 0 && eqn2.clone() != eqn.clone() {
                if number.borrow()[(eqn2.clone()-1) as usize].clone() == -1 {
                    (outStack, outIndex, outComponents) = StrongConnectTransposed(mT.clone(), ass2.clone(), eqn2.clone(), outStack.clone(), outIndex.clone(), number.clone(), lowlink.clone(), onStack.clone(), outComponents.clone())?;
                    {let _arr = lowlink.clone(); let _val = intMin(lowlink.borrow()[(eqn.clone()-1) as usize].clone(), lowlink.borrow()[(eqn2.clone()-1) as usize].clone()); _arr.borrow_mut()[(eqn.clone()-1) as usize] = _val; _arr};
                } else if onStack.borrow()[(eqn2.clone()-1) as usize].clone() {
                    {let _arr = lowlink.clone(); let _val = intMin(lowlink.borrow()[(eqn.clone()-1) as usize].clone(), number.borrow()[(eqn2.clone()-1) as usize].clone()); _arr.borrow_mut()[(eqn.clone()-1) as usize] = _val; _arr};
                }
            }
        }
    }
    if lowlink.borrow()[(eqn.clone()-1) as usize].clone() == number.borrow()[(eqn.clone()-1) as usize].clone() {
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(outStack.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa1, tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
            _ => bail!("pattern mismatch"),
        } };
        eqn2 = __pa1.clone();
        outStack = __pa2.clone();
        {let _arr = onStack.clone(); _arr.borrow_mut()[(eqn2.clone()-1) as usize] = false; _arr};
        SCC = list![eqn2.clone()];
        while eqn.clone() != eqn2.clone() {
            let (__pa3, __pa4) = ::match_deref::match_deref! { match &(outStack.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
                _ => bail!("pattern mismatch"),
            } };
            eqn2 = __pa3.clone();
            outStack = __pa4.clone();
            {let _arr = onStack.clone(); _arr.borrow_mut()[(eqn2.clone()-1) as usize] = false; _arr};
            SCC = cons(eqn2.clone(), SCC.clone());
        }
        outComponents = cons(metamodelica::Dangerous::listReverseInPlace(SCC.clone()), outComponents.clone());
    }
    Ok((outStack, outIndex, outComponents))
}

