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

use crate::AbsynUtil;
use crate::ComponentReferenceBasics;
use crate::ExpressionDumpTpl;
use openmodelica_ast::Absyn;
use openmodelica_frontend_types::DAE;
use openmodelica_susan::Tpl;
use openmodelica_util::Error;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

// public imports
pub fn printExpStr(mut e: Arc<DAE::Exp>) -> Result<ArcStr> {
    let mut s: ArcStr = arcstr::literal!("");
    s = (Tpl::tplString2((std::sync::Arc::new(ExpressionDumpTpl::dumpExp) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, Arc<DAE::Exp>, ArcStr) -> Result<Tpl::Text> + 'static>), e.clone(), (literal!("\"")).clone())?).clone();
    Ok(s)
}

pub fn dimensionString(mut dim: Arc<DAE::Dimension>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ DAE::Dimension::DIM_UNKNOWN => {
            literal!(":")
        },
        Deref @ DAE::Dimension::DIM_ENUM { enumTypeName: p, .. } => {
            let mut s: ArcStr = arcstr::literal!("");
            s = (AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone();
            s.clone()
        },
        Deref @ DAE::Dimension::DIM_BOOLEAN => {
            literal!("Boolean")
        },
        Deref @ DAE::Dimension::DIM_INTEGER { integer: x } => {
            let mut s: ArcStr = arcstr::literal!("");
            s = (intString(x.clone())).clone();
            s.clone()
        },
        Deref @ DAE::Dimension::DIM_EXP { exp: e } => {
            let mut s: ArcStr = arcstr::literal!("");
            s = (printExpStr(e.clone())?).clone();
            s.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(r#str)
}

pub fn dimensionsString(mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = stringDelimitList(List::map(dims.clone(), (std::sync::Arc::new(dimensionString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Dimension>) -> Result<ArcStr> + 'static>)), (literal!(",")).clone());
    r#str
}

pub fn shouldParenthesize(mut inOperand: Arc<DAE::Exp>, mut inOperator: Arc<DAE::Exp>, mut inLhs: bool) -> Result<bool> {
    let mut outShouldParenthesize: bool = false;
    outShouldParenthesize = (::match_deref::match_deref! { match &(inOperand.clone()) {
        Deref @ DAE::Exp::UNARY { .. } => {
            true
        },
        _ => {
            let mut diff: i32 = 0;
            diff = Util::intCompare(priority(inOperand.clone(), inLhs.clone())?, priority(inOperator.clone(), inLhs.clone())?);
            shouldParenthesize2(diff.clone(), inOperand.clone(), inLhs.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outShouldParenthesize)
}

fn shouldParenthesize2(mut inPrioDiff: i32, mut inOperand: Arc<DAE::Exp>, mut inLhs: bool) -> bool {
    let mut outShouldParenthesize: bool = false;
    outShouldParenthesize = (match inPrioDiff.clone() {
        1 => true,
        0 => if (inLhs.clone()) {isNonAssociativeExp(inOperand.clone())} else {!(isAssociativeExp(inOperand.clone()))},
        _ => false,
    });
    outShouldParenthesize
}

fn isAssociativeExp(mut inExp: Arc<DAE::Exp>) -> bool {
    let mut outIsAssociative: bool = false;
    outIsAssociative = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::BINARY { operator: op, .. } => {
            isAssociativeOp(op.clone())
        },
        Deref @ DAE::Exp::LBINARY { .. } => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsAssociative
}

fn isAssociativeOp(mut inOperator: DAE::Operator) -> bool {
    let mut outIsAssociative: bool = false;
    outIsAssociative = (match inOperator.clone() {
        DAE::Operator::ADD { .. } => true,
        DAE::Operator::MUL { .. } => true,
        DAE::Operator::ADD_ARR { .. } => true,
        DAE::Operator::MUL_ARRAY_SCALAR { .. } => true,
        DAE::Operator::ADD_ARRAY_SCALAR { .. } => true,
        _ => false,
    });
    outIsAssociative
}

fn isNonAssociativeExp(mut exp: Arc<DAE::Exp>) -> bool {
    let mut isNonAssociative: bool = false;
    isNonAssociative = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::BINARY { .. } => isNonAssociativeOp(var_field!((*exp).operator, DAE::Exp::BINARY).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isNonAssociative
}

fn isNonAssociativeOp(mut inOperator: DAE::Operator) -> bool {
    let mut isNonAssociative: bool = false;
    isNonAssociative = (match inOperator.clone() {
        DAE::Operator::POW { .. } => true,
        DAE::Operator::POW_ARRAY_SCALAR { .. } => true,
        DAE::Operator::POW_SCALAR_ARRAY { .. } => true,
        DAE::Operator::POW_ARR { .. } => true,
        DAE::Operator::POW_ARR2 { .. } => true,
        _ => false,
    });
    isNonAssociative
}

pub fn priority(mut inExp: Arc<DAE::Exp>, mut inLhs: bool) -> Result<i32> {
    let mut outPriority: i32 = 0;
    outPriority = (::match_deref::match_deref! { match &((inExp.clone(), inLhs.clone())) {
        (Deref @ DAE::Exp::BINARY { operator: op, .. }, false) => {
            priorityBinopRhs(op.clone())?
        },
        (Deref @ DAE::Exp::BINARY { operator: op, .. }, true) => {
            priorityBinopLhs(op.clone())?
        },
        (Deref @ DAE::Exp::RCONST { .. }, _) if (var_field!((*inExp).real, DAE::Exp::RCONST).clone() < metamodelica::OrderedFloat(0.0_f64)) => {
            4
        },
        (Deref @ DAE::Exp::UNARY { .. }, _) => {
            4
        },
        (Deref @ DAE::Exp::LBINARY { operator: op, .. }, _) => {
            priorityLBinop(op.clone())?
        },
        (Deref @ DAE::Exp::LUNARY { .. }, _) => {
            7
        },
        (Deref @ DAE::Exp::RELATION { .. }, _) => {
            6
        },
        (Deref @ DAE::Exp::RANGE { .. }, _) => {
            10
        },
        (Deref @ DAE::Exp::IFEXP { .. }, _) => {
            11
        },
        _ => {
            0
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outPriority)
}

fn priorityBinopLhs(mut inOp: DAE::Operator) -> Result<i32> {
    let mut outPriority: i32 = 0;
    outPriority = (match inOp.clone() {
        DAE::Operator::ADD { .. } => 5,
        DAE::Operator::SUB { .. } => 5,
        DAE::Operator::MUL { .. } => 2,
        DAE::Operator::DIV { .. } => 2,
        DAE::Operator::POW { .. } => 1,
        DAE::Operator::ADD_ARR { .. } => 5,
        DAE::Operator::SUB_ARR { .. } => 5,
        DAE::Operator::MUL_ARR { .. } => 2,
        DAE::Operator::DIV_ARR { .. } => 2,
        DAE::Operator::MUL_ARRAY_SCALAR { .. } => 2,
        DAE::Operator::ADD_ARRAY_SCALAR { .. } => 5,
        DAE::Operator::SUB_SCALAR_ARRAY { .. } => 5,
        DAE::Operator::MUL_SCALAR_PRODUCT { .. } => 2,
        DAE::Operator::MUL_MATRIX_PRODUCT { .. } => 2,
        DAE::Operator::DIV_ARRAY_SCALAR { .. } => 2,
        DAE::Operator::DIV_SCALAR_ARRAY { .. } => 2,
        DAE::Operator::POW_ARRAY_SCALAR { .. } => 1,
        DAE::Operator::POW_SCALAR_ARRAY { .. } => 1,
        DAE::Operator::POW_ARR { .. } => 1,
        DAE::Operator::POW_ARR2 { .. } => 1,
        _ => bail!("match: no arm matched"),
    });
    Ok(outPriority)
}

fn priorityBinopRhs(mut inOp: DAE::Operator) -> Result<i32> {
    let mut outPriority: i32 = 0;
    outPriority = (match inOp.clone() {
        DAE::Operator::ADD { .. } => 6,
        DAE::Operator::SUB { .. } => 5,
        DAE::Operator::MUL { .. } => 3,
        DAE::Operator::DIV { .. } => 2,
        DAE::Operator::POW { .. } => 1,
        DAE::Operator::ADD_ARR { .. } => 6,
        DAE::Operator::SUB_ARR { .. } => 5,
        DAE::Operator::MUL_ARR { .. } => 3,
        DAE::Operator::DIV_ARR { .. } => 2,
        DAE::Operator::MUL_ARRAY_SCALAR { .. } => 3,
        DAE::Operator::ADD_ARRAY_SCALAR { .. } => 6,
        DAE::Operator::SUB_SCALAR_ARRAY { .. } => 5,
        DAE::Operator::MUL_SCALAR_PRODUCT { .. } => 3,
        DAE::Operator::MUL_MATRIX_PRODUCT { .. } => 3,
        DAE::Operator::DIV_ARRAY_SCALAR { .. } => 2,
        DAE::Operator::DIV_SCALAR_ARRAY { .. } => 2,
        DAE::Operator::POW_ARRAY_SCALAR { .. } => 1,
        DAE::Operator::POW_SCALAR_ARRAY { .. } => 1,
        DAE::Operator::POW_ARR { .. } => 1,
        DAE::Operator::POW_ARR2 { .. } => 1,
        _ => bail!("match: no arm matched"),
    });
    Ok(outPriority)
}

fn priorityLBinop(mut inOp: DAE::Operator) -> Result<i32> {
    let mut outPriority: i32 = 0;
    outPriority = (match inOp.clone() {
        DAE::Operator::AND { .. } => 8,
        DAE::Operator::OR { .. } => 9,
        _ => bail!("match: no arm matched"),
    });
    Ok(outPriority)
}

pub fn evalCat<Exp: Clone + 'static>(mut dim: i32, mut exps: Arc<metamodelica::List<Exp>>, mut getArrayContents: Arc<dyn ::std::ops::Fn(Exp) -> Result<Arc<metamodelica::List<Exp>>> + 'static>, mut toString: Arc<dyn ::std::ops::Fn(Exp) -> Result<ArcStr> + 'static>) -> Result<(Arc<metamodelica::List<Exp>>, Arc<metamodelica::List<i32>>)> {
    pub type GetArrayContents<Exp: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Exp) -> Result<Arc<metamodelica::List<Exp>>> + 'static>;

    pub type MakeArrayFromList<Exp: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Exp>>) -> Result<Exp> + 'static>;

    pub type ToString<Exp: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Exp) -> Result<ArcStr> + 'static>;

    let mut outExps: Arc<metamodelica::List<Exp>> = metamodelica::nil();
    let mut outDims: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut arr: Arc<metamodelica::List<Exp>> = metamodelica::nil();
    let mut arrs: Arc<metamodelica::List<Arc<metamodelica::List<Exp>>>> = metamodelica::nil();
    let mut dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut firstDims: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut lastDims: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut reverseDims: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut dimsLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut l: i32 = 0;
    let mut thisDim: i32 = 0;
    let mut lastDim: i32 = 0;
    let mut expArr: metamodelica::Array<Exp>;
    let true = (dim.clone() >= 1) else { bail!("pattern mismatch") };
    let false = (exps.clone().is_empty()) else { bail!("pattern mismatch") };
    if 1 == dim.clone() {
        outExps = ({
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut e in (exps.clone().reverse()).into_iter().cloned() {
            let __x = getArrayContents(e.clone())?;
            __acc = __x.append(&__acc);
        }
        __acc
    });
        outDims = list![(outExps.clone().len() as i32)];
        return Ok((outExps.clone(), outDims.clone()));
    }
    for mut e in &*exps.clone().reverse() {
        let mut e = e.clone();
        (arr, dims) = evalCatGetFlatArray(e.clone(), dim.clone(), getArrayContents.clone(), toString.clone())?;
        arrs = cons(arr.clone(), arrs.clone());
        dimsLst = cons(dims.clone(), dimsLst.clone());
    }
    for mut i in 1..=dim.clone() - 1 {
        j = ({
        let mut __acc: Option<i32> = None;
        for mut d in (dimsLst.clone()).into_iter().cloned() {
            let __x = listHead(d.clone())?;
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty min reduction"))?
    });
        if j.clone() != ({
        let mut __acc: Option<i32> = None;
        for mut d in (dimsLst.clone()).into_iter().cloned() {
            let __x = listHead(d.clone())?;
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    }) {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ExpressionBasics.evalCat")); __mm_s.push_str(&*literal!(": cat got uneven dimensions for dim=")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", i.clone()))); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (exps.clone()).into_iter().cloned() {
            let __x = toString(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!(", ")).clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
        }
        firstDims = cons(j.clone(), firstDims.clone());
        dimsLst = ({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
        for mut d in (dimsLst.clone()).into_iter().cloned() {
            let __x = listRest(d.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    }
    reverseDims = firstDims.clone();
    firstDims = firstDims.clone().reverse();
    lastDims = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut d in (dimsLst.clone()).into_iter().cloned() {
            let __x = listHead(d.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    lastDim = ({
        let mut __acc: i32 = 0;
        for mut d in (lastDims.clone()).into_iter().cloned() {
            let __x = d.clone();
            __acc += __x;
        }
        __acc
    });
    reverseDims = cons(lastDim.clone(), reverseDims.clone());
    expArr = metamodelica::arrayCreate(lastDim.clone() * ({
        let mut __acc: i32 = 1;
        for mut d in (firstDims.clone()).into_iter().cloned() {
            let __x = d.clone();
            __acc *= __x;
        }
        __acc
    }), listHead(exps.clone())?);
    k = 1;
    for mut exps in &*arrs.clone() {
        let mut exps = exps.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lastDims.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        thisDim = __pa0.clone();
        lastDims = __pa1.clone();
        l = 0;
        for mut e in &*exps.clone() {
            let mut e = e.clone();
            unsafe { metamodelica::Dangerous::arrayInitSlot(expArr.clone(), k.clone() + intMod(l.clone(), thisDim.clone()) + lastDim.clone() * l.clone() / thisDim.clone(), e.clone()) };
            l = l.clone() + 1;
        }
        k = k.clone() + thisDim.clone();
    }
    outExps = Arc::new(expArr.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
    outDims = reverseDims.clone().reverse();
    Ok((outExps, outDims))
}

fn evalCatGetFlatArray<Exp: Clone + 'static>(mut e: Exp, mut dim: i32, mut getArrayContents: Arc<dyn ::std::ops::Fn(Exp) -> Result<Arc<metamodelica::List<Exp>>> + 'static>, mut toString: Arc<dyn ::std::ops::Fn(Exp) -> Result<ArcStr> + 'static>) -> Result<(Arc<metamodelica::List<Exp>>, Arc<metamodelica::List<i32>>)> {
    pub type GetArrayContents<Exp: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Exp) -> Result<Arc<metamodelica::List<Exp>>> + 'static>;

    pub type ToString<Exp: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Exp) -> Result<ArcStr> + 'static>;

    let mut outExps: Arc<metamodelica::List<Exp>> = metamodelica::nil();
    let mut outDims: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut arr: Arc<metamodelica::List<Exp>> = metamodelica::nil();
    let mut dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut i: i32 = 0;
    if dim.clone() == 1 {
        outExps = getArrayContents(e.clone())?;
        outDims = list![(outExps.clone().len() as i32)];
        return Ok((outExps.clone(), outDims.clone()));
    }
    i = 0;
    for mut exp in &*getArrayContents(e.clone())?.reverse() {
        let mut exp = exp.clone();
        (arr, dims) = evalCatGetFlatArray(exp.clone(), dim.clone() - 1, getArrayContents.clone(), toString.clone())?;
        if outDims.clone().is_empty() {
            outDims = dims.clone();
        } else if !(dims.clone() == outDims.clone()) {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ExpressionBasics.evalCatGetFlatArray")); __mm_s.push_str(&*literal!(": Got unbalanced array from ")); __mm_s.push_str(&*toString(e.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
        }
        outExps = listAppend(arr.clone(), outExps.clone());
        i = i.clone() + 1;
    }
    outDims = cons(i.clone(), outDims.clone());
    Ok((outExps, outDims))
}

pub fn expEqual(mut inExp1: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>) -> Result<bool> {
    let mut outEqual: bool = false;
    outEqual = 0 == compare(inExp1.clone(), inExp2.clone())?;
    Ok(outEqual)
}

pub fn compare(mut inExp1: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>) -> Result<i32> {
    let mut comp: i32 = 0;
    if referenceEq(&inExp1.clone(),&inExp2.clone()) {
        comp = 0;
        return Ok(comp.clone());
    }
    comp = Util::intCompare(metamodelica::valueConstructor((&*inExp1.clone()))?, metamodelica::valueConstructor((&*inExp2.clone()))?);
    if comp.clone() != 0 {
        return Ok(comp.clone());
    }
    comp = (::match_deref::match_deref! { match &(inExp1.clone()) {
        Deref @ DAE::Exp::ICONST { .. } => {
            let mut i: i32 = 0;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::ICONST { integer: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            i = __pa0.clone();
            Util::intCompare(var_field!((*inExp1).integer, DAE::Exp::ICONST).clone(), i.clone())
        },
        Deref @ DAE::Exp::RCONST { .. } => {
            let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let __pa0 = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::RCONST { real: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            r = __pa0.clone();
            Util::realCompare(var_field!((*inExp1).real, DAE::Exp::RCONST).clone(), r.clone())
        },
        Deref @ DAE::Exp::SCONST { .. } => {
            let mut s: ArcStr = arcstr::literal!("");
            let __pa0 = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::SCONST { string: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            s = __pa0.clone();
            stringCompare((var_field!((*inExp1).string, DAE::Exp::SCONST).clone()).clone(), (s.clone()).clone())
        },
        Deref @ DAE::Exp::BCONST { .. } => {
            let mut b: bool = false;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::BCONST { bool: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            b = __pa0.clone();
            Util::boolCompare(var_field!((*inExp1).bool, DAE::Exp::BCONST).clone(), b.clone())
        },
        Deref @ DAE::Exp::ENUM_LITERAL { .. } => {
            let mut p: Arc<Absyn::Path>;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::ENUM_LITERAL { name: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            p = __pa0.clone();
            AbsynUtil::pathCompare(var_field!((*inExp1).name, DAE::Exp::ENUM_LITERAL).clone(), p.clone())?
        },
        Deref @ DAE::Exp::CREF { .. } => {
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let __pa0 = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::CREF { componentRef: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cr = __pa0.clone();
            ComponentReferenceBasics::crefCompareGeneric(var_field!((*inExp1).componentRef, DAE::Exp::CREF).clone(), cr.clone())?
        },
        Deref @ DAE::Exp::ARRAY { .. } => {
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::ARRAY { array: __pa0, ty: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            expl = __pa0.clone();
            ty = __pa1.clone();
            comp = valueCompare(var_field!((*inExp1).ty, DAE::Exp::ARRAY).clone(), ty.clone());
            if (0 == comp.clone()) {compareList(var_field!((*inExp1).array, DAE::Exp::ARRAY).clone(), expl.clone())?} else {comp.clone()}
        },
        Deref @ DAE::Exp::MATRIX { .. } => {
            let mut mexpl: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::MATRIX { matrix: __pa0, ty: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            mexpl = __pa0.clone();
            ty = __pa1.clone();
            comp = valueCompare(var_field!((*inExp1).ty, DAE::Exp::MATRIX).clone(), ty.clone());
            if (0 == comp.clone()) {compareListList(var_field!((*inExp1).matrix, DAE::Exp::MATRIX).clone(), mexpl.clone())?} else {comp.clone()}
        },
        Deref @ DAE::Exp::BINARY { .. } => {
            let mut e1: Arc<DAE::Exp>;
            let mut e2: Arc<DAE::Exp>;
            let mut op: DAE::Operator;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::BINARY { exp2: __pa0, operator: __pa1, exp1: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e2 = __pa0.clone();
            op = __pa1.clone();
            e1 = __pa2.clone();
            comp = operatorCompare(var_field!((*inExp1).operator, DAE::Exp::BINARY).clone(), op.clone())?;
            comp = if (0 == comp.clone()) {compare(var_field!((*inExp1).exp1, DAE::Exp::BINARY).clone(), e1.clone())?} else {comp.clone()};
            if (0 == comp.clone()) {compare(var_field!((*inExp1).exp2, DAE::Exp::BINARY).clone(), e2.clone())?} else {comp.clone()}
        },
        Deref @ DAE::Exp::LBINARY { .. } => {
            let mut e1: Arc<DAE::Exp>;
            let mut e2: Arc<DAE::Exp>;
            let mut op: DAE::Operator;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::LBINARY { exp2: __pa0, operator: __pa1, exp1: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e2 = __pa0.clone();
            op = __pa1.clone();
            e1 = __pa2.clone();
            comp = operatorCompare(var_field!((*inExp1).operator, DAE::Exp::LBINARY).clone(), op.clone())?;
            comp = if (0 == comp.clone()) {compare(var_field!((*inExp1).exp1, DAE::Exp::LBINARY).clone(), e1.clone())?} else {comp.clone()};
            if (0 == comp.clone()) {compare(var_field!((*inExp1).exp2, DAE::Exp::LBINARY).clone(), e2.clone())?} else {comp.clone()}
        },
        Deref @ DAE::Exp::UNARY { .. } => {
            let mut e: Arc<DAE::Exp>;
            let mut op: DAE::Operator;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::UNARY { operator: __pa0, exp: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            op = __pa0.clone();
            e = __pa1.clone();
            comp = operatorCompare(var_field!((*inExp1).operator, DAE::Exp::UNARY).clone(), op.clone())?;
            if (0 == comp.clone()) {compare(var_field!((*inExp1).exp, DAE::Exp::UNARY).clone(), e.clone())?} else {comp.clone()}
        },
        Deref @ DAE::Exp::LUNARY { .. } => {
            let mut e: Arc<DAE::Exp>;
            let mut op: DAE::Operator;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::LUNARY { operator: __pa0, exp: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            op = __pa0.clone();
            e = __pa1.clone();
            comp = operatorCompare(var_field!((*inExp1).operator, DAE::Exp::LUNARY).clone(), op.clone())?;
            if (0 == comp.clone()) {compare(var_field!((*inExp1).exp, DAE::Exp::LUNARY).clone(), e.clone())?} else {comp.clone()}
        },
        Deref @ DAE::Exp::RELATION { .. } => {
            let mut e1: Arc<DAE::Exp>;
            let mut e2: Arc<DAE::Exp>;
            let mut op: DAE::Operator;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::RELATION { exp2: __pa0, operator: __pa1, exp1: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e2 = __pa0.clone();
            op = __pa1.clone();
            e1 = __pa2.clone();
            comp = operatorCompare(var_field!((*inExp1).operator, DAE::Exp::RELATION).clone(), op.clone())?;
            comp = if (0 == comp.clone()) {compare(var_field!((*inExp1).exp1, DAE::Exp::RELATION).clone(), e1.clone())?} else {comp.clone()};
            if (0 == comp.clone()) {compare(var_field!((*inExp1).exp2, DAE::Exp::RELATION).clone(), e2.clone())?} else {comp.clone()}
        },
        Deref @ DAE::Exp::IFEXP { .. } => {
            let mut e: Arc<DAE::Exp>;
            let mut e1: Arc<DAE::Exp>;
            let mut e2: Arc<DAE::Exp>;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::IFEXP { expElse: __pa0, expThen: __pa1, expCond: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e2 = __pa0.clone();
            e1 = __pa1.clone();
            e = __pa2.clone();
            comp = compare(var_field!((*inExp1).expCond, DAE::Exp::IFEXP).clone(), e.clone())?;
            comp = if (0 == comp.clone()) {compare(var_field!((*inExp1).expThen, DAE::Exp::IFEXP).clone(), e1.clone())?} else {comp.clone()};
            if (0 == comp.clone()) {compare(var_field!((*inExp1).expElse, DAE::Exp::IFEXP).clone(), e2.clone())?} else {comp.clone()}
        },
        Deref @ DAE::Exp::CALL { .. } => {
            let mut p: Arc<Absyn::Path>;
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::CALL { expLst: __pa0, path: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            expl = __pa0.clone();
            p = __pa1.clone();
            comp = AbsynUtil::pathCompare(var_field!((*inExp1).path, DAE::Exp::CALL).clone(), p.clone())?;
            if (0 == comp.clone()) {compareList(var_field!((*inExp1).expLst, DAE::Exp::CALL).clone(), expl.clone())?} else {comp.clone()}
        },
        Deref @ DAE::Exp::RECORD { .. } => {
            let mut p: Arc<Absyn::Path>;
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::RECORD { exps: __pa0, path: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            expl = __pa0.clone();
            p = __pa1.clone();
            comp = AbsynUtil::pathCompare(var_field!((*inExp1).path, DAE::Exp::RECORD).clone(), p.clone())?;
            if (0 == comp.clone()) {compareList(var_field!((*inExp1).exps, DAE::Exp::RECORD).clone(), expl.clone())?} else {comp.clone()}
        },
        Deref @ DAE::Exp::PARTEVALFUNCTION { .. } => {
            let mut p: Arc<Absyn::Path>;
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::PARTEVALFUNCTION { expList: __pa0, path: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            expl = __pa0.clone();
            p = __pa1.clone();
            comp = AbsynUtil::pathCompare(var_field!((*inExp1).path, DAE::Exp::PARTEVALFUNCTION).clone(), p.clone())?;
            if (0 == comp.clone()) {compareList(var_field!((*inExp1).expList, DAE::Exp::PARTEVALFUNCTION).clone(), expl.clone())?} else {comp.clone()}
        },
        Deref @ DAE::Exp::RANGE { .. } => {
            let mut e1: Arc<DAE::Exp>;
            let mut e2: Arc<DAE::Exp>;
            let mut oe: Option<Arc<DAE::Exp>> = None;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::RANGE { stop: __pa0, step: __pa1, start: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e2 = __pa0.clone();
            oe = __pa1.clone();
            e1 = __pa2.clone();
            comp = compare(var_field!((*inExp1).start, DAE::Exp::RANGE).clone(), e1.clone())?;
            comp = if (0 == comp.clone()) {compare(var_field!((*inExp1).stop, DAE::Exp::RANGE).clone(), e2.clone())?} else {comp.clone()};
            if (0 == comp.clone()) {compareOpt(var_field!((*inExp1).step, DAE::Exp::RANGE).clone(), oe.clone())?} else {comp.clone()}
        },
        Deref @ DAE::Exp::TUPLE { .. } => {
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let __pa0 = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::TUPLE { PR: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            expl = __pa0.clone();
            compareList(var_field!((*inExp1).PR, DAE::Exp::TUPLE).clone(), expl.clone())?
        },
        Deref @ DAE::Exp::CAST { .. } => {
            let mut e: Arc<DAE::Exp>;
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::CAST { exp: __pa0, ty: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e = __pa0.clone();
            ty = __pa1.clone();
            comp = valueCompare(var_field!((*inExp1).ty, DAE::Exp::CAST).clone(), ty.clone());
            if (0 == comp.clone()) {compare(var_field!((*inExp1).exp, DAE::Exp::CAST).clone(), e.clone())?} else {comp.clone()}
        },
        Deref @ DAE::Exp::ASUB { .. } => {
            let mut e: Arc<DAE::Exp>;
            let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::ASUB { sub: __pa0, exp: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            subs = __pa0.clone();
            e = __pa1.clone();
            comp = compare(var_field!((*inExp1).exp, DAE::Exp::ASUB).clone(), e.clone())?;
            if (comp.clone() == 0) {compareSubscriptList(var_field!((*inExp1).sub, DAE::Exp::ASUB).clone(), subs.clone())?} else {comp.clone()}
        },
        Deref @ DAE::Exp::RSUB { .. } => {
            let mut i: i32 = 0;
            let mut s: ArcStr = arcstr::literal!("");
            let mut e: Arc<DAE::Exp>;
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::RSUB { ty: __pa0, fieldName: __pa1, ix: __pa2, exp: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            ty = __pa0.clone();
            s = __pa1.clone();
            i = __pa2.clone();
            e = __pa3.clone();
            comp = Util::intCompare(var_field!((*inExp1).ix, DAE::Exp::RSUB).clone(), i.clone());
            comp = if (comp.clone() == 0) {valueCompare(var_field!((*inExp1).ty, DAE::Exp::RSUB).clone(), ty.clone())} else {comp.clone()};
            comp = if (comp.clone() == 0) {stringCompare((var_field!((*inExp1).fieldName, DAE::Exp::RSUB).clone()).clone(), (s.clone()).clone())} else {comp.clone()};
            if (comp.clone() == 0) {compare(var_field!((*inExp1).exp, DAE::Exp::RSUB).clone(), e.clone())?} else {comp.clone()}
        },
        Deref @ DAE::Exp::TSUB { .. } => {
            let mut i: i32 = 0;
            let mut e: Arc<DAE::Exp>;
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::TSUB { ty: __pa0, ix: __pa1, exp: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            ty = __pa0.clone();
            i = __pa1.clone();
            e = __pa2.clone();
            comp = Util::intCompare(var_field!((*inExp1).ix, DAE::Exp::TSUB).clone(), i.clone());
            comp = if (0 == comp.clone()) {valueCompare(var_field!((*inExp1).ty, DAE::Exp::TSUB).clone(), ty.clone())} else {comp.clone()};
            if (0 == comp.clone()) {compare(var_field!((*inExp1).exp, DAE::Exp::TSUB).clone(), e.clone())?} else {comp.clone()}
        },
        Deref @ DAE::Exp::SIZE { .. } => {
            let mut e: Arc<DAE::Exp>;
            let mut oe: Option<Arc<DAE::Exp>> = None;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::SIZE { sz: __pa0, exp: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            oe = __pa0.clone();
            e = __pa1.clone();
            comp = compare(var_field!((*inExp1).exp, DAE::Exp::SIZE).clone(), e.clone())?;
            if (comp.clone() == 0) {compareOpt(var_field!((*inExp1).sz, DAE::Exp::SIZE).clone(), oe.clone())?} else {comp.clone()}
        },
        Deref @ DAE::Exp::REDUCTION { .. } => {
            valueCompare(inExp1.clone(), inExp2.clone())
        },
        Deref @ DAE::Exp::LIST { .. } => {
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let __pa0 = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::LIST { valList: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            expl = __pa0.clone();
            compareList(var_field!((*inExp1).valList, DAE::Exp::LIST).clone(), expl.clone())?
        },
        Deref @ DAE::Exp::CONS { .. } => {
            let mut e1: Arc<DAE::Exp>;
            let mut e2: Arc<DAE::Exp>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::CONS { cdr: __pa0, car: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e2 = __pa0.clone();
            e1 = __pa1.clone();
            comp = compare(var_field!((*inExp1).car, DAE::Exp::CONS).clone(), e1.clone())?;
            if (0 == comp.clone()) {compare(var_field!((*inExp1).cdr, DAE::Exp::CONS).clone(), e2.clone())?} else {comp.clone()}
        },
        Deref @ DAE::Exp::META_TUPLE { .. } => {
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let __pa0 = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::META_TUPLE { listExp: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            expl = __pa0.clone();
            compareList(var_field!((*inExp1).listExp, DAE::Exp::META_TUPLE).clone(), expl.clone())?
        },
        Deref @ DAE::Exp::META_OPTION { .. } => {
            let mut oe: Option<Arc<DAE::Exp>> = None;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::META_OPTION { exp: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            oe = __pa0.clone();
            compareOpt(var_field!((*inExp1).exp, DAE::Exp::META_OPTION).clone(), oe.clone())?
        },
        Deref @ DAE::Exp::METARECORDCALL { .. } => {
            let mut p: Arc<Absyn::Path>;
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::METARECORDCALL { args: __pa0, path: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            expl = __pa0.clone();
            p = __pa1.clone();
            comp = AbsynUtil::pathCompare(var_field!((*inExp1).path, DAE::Exp::METARECORDCALL).clone(), p.clone())?;
            if (comp.clone() == 0) {compareList(var_field!((*inExp1).args, DAE::Exp::METARECORDCALL).clone(), expl.clone())?} else {comp.clone()}
        },
        Deref @ DAE::Exp::MATCHEXPRESSION { .. } => {
            valueCompare(inExp1.clone(), inExp2.clone())
        },
        Deref @ DAE::Exp::BOX { .. } => {
            let mut e: Arc<DAE::Exp>;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::BOX { exp: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            e = __pa0.clone();
            compare(var_field!((*inExp1).exp, DAE::Exp::BOX).clone(), e.clone())?
        },
        Deref @ DAE::Exp::UNBOX { .. } => {
            let mut e: Arc<DAE::Exp>;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::UNBOX { exp: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            e = __pa0.clone();
            compare(var_field!((*inExp1).exp, DAE::Exp::UNBOX).clone(), e.clone())?
        },
        Deref @ DAE::Exp::SHARED_LITERAL { .. } => {
            let mut i: i32 = 0;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::SHARED_LITERAL { index: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            i = __pa0.clone();
            Util::intCompare(var_field!((*inExp1).index, DAE::Exp::SHARED_LITERAL).clone(), i.clone())
        },
        Deref @ DAE::Exp::EMPTY { .. } => {
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let __pa0 = ::match_deref::match_deref! { match &(inExp2.clone()) {
                Deref @ DAE::Exp::EMPTY { name: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cr = __pa0.clone();
            ComponentReferenceBasics::crefCompareGeneric(var_field!((*inExp1).name, DAE::Exp::EMPTY).clone(), cr.clone())?
        },
        Deref @ DAE::Exp::CODE { .. } => {
            valueCompare(inExp1.clone(), inExp2.clone())
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ExpressionBasics.compare failed: ctor:")); __mm_s.push_str(&*ArcStr::from(::std::format!("{:?}", metamodelica::valueConstructor((&*inExp1.clone()))?))); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*printExpStr(inExp1.clone())?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*printExpStr(inExp2.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(comp)
}

fn compareList(mut inExpl1: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inExpl2: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<i32> {
    let mut comp: i32 = 0;
    let mut len1: i32 = 0;
    let mut len2: i32 = 0;
    let mut e2: Arc<DAE::Exp>;
    let mut rest_expl2: Arc<metamodelica::List<Arc<DAE::Exp>>> = inExpl2.clone();
    len1 = (inExpl1.clone().len() as i32);
    len2 = (inExpl2.clone().len() as i32);
    comp = Util::intCompare(len1.clone(), len2.clone());
    if comp.clone() != 0 {
        return Ok(comp.clone());
    }
    for mut e1 in &*inExpl1.clone() {
        let mut e1 = e1.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_expl2.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e2 = __pa0.clone();
        rest_expl2 = __pa1.clone();
        comp = compare(e1.clone(), e2.clone())?;
        if 0 != comp.clone() {
            return Ok(comp.clone());
        }
    }
    comp = 0;
    Ok(comp)
}

fn compareListList(mut inExpl1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, mut inExpl2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>) -> Result<i32> {
    let mut comp: i32 = 0;
    let mut expl2: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut rest_expl2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = inExpl2.clone();
    let mut len1: i32 = 0;
    let mut len2: i32 = 0;
    len1 = (inExpl1.clone().len() as i32);
    len2 = (inExpl2.clone().len() as i32);
    comp = Util::intCompare(len1.clone(), len2.clone());
    if comp.clone() != 0 {
        return Ok(comp.clone());
    }
    for mut expl1 in &*inExpl1.clone() {
        let mut expl1 = expl1.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_expl2.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        expl2 = __pa0.clone();
        rest_expl2 = __pa1.clone();
        comp = compareList(expl1.clone(), expl2.clone())?;
        if 0 != comp.clone() {
            return Ok(comp.clone());
        }
    }
    comp = 0;
    Ok(comp)
}

fn compareOpt(mut inExp1: Option<Arc<DAE::Exp>>, mut inExp2: Option<Arc<DAE::Exp>>) -> Result<i32> {
    let mut comp: i32 = 0;
    let mut e1: Arc<DAE::Exp>;
    let mut e2: Arc<DAE::Exp>;
    comp = (::match_deref::match_deref! { match &((inExp1.clone(), inExp2.clone())) {
        (None, None) => 0,
        (None, _) => -1,
        (_, None) => 1,
        (Some(e1), Some(e2)) => compare(e1.clone(), e2.clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(comp)
}

pub fn operatorCompare(mut inOperator1: DAE::Operator, mut inOperator2: DAE::Operator) -> Result<i32> {
    let mut comp: i32 = 0;
    comp = (match (inOperator1.clone(), inOperator2.clone()) {
        (DAE::Operator::USERDEFINED { fqName: ref p1 }, DAE::Operator::USERDEFINED { fqName: ref p2 }) => {
            AbsynUtil::pathCompare(p1.clone(), p2.clone())?
        },
        _ => {
            Util::intCompare(metamodelica::valueConstructor((&inOperator1.clone()))?, metamodelica::valueConstructor((&inOperator2.clone()))?)
        },
    });
    Ok(comp)
}

pub fn compareSubscripts(mut sub1: Arc<DAE::Subscript>, mut sub2: Arc<DAE::Subscript>) -> Result<i32> {
    let mut res: i32 = 0;
    if referenceEq(&sub1.clone(),&sub2.clone()) {
        res = 0;
    } else {
        res = (::match_deref::match_deref! { match &((sub1.clone(), sub2.clone())) {
        (Deref @ DAE::Subscript::WHOLEDIM { .. }, Deref @ DAE::Subscript::WHOLEDIM { .. }) => 0,
        (Deref @ DAE::Subscript::SLICE { .. }, Deref @ DAE::Subscript::SLICE { .. }) => compare(var_field!((*sub1).exp, DAE::Subscript::SLICE).clone(), var_field!((*sub2).exp, DAE::Subscript::SLICE).clone())?,
        (Deref @ DAE::Subscript::INDEX { .. }, Deref @ DAE::Subscript::INDEX { .. }) => compare(var_field!((*sub1).exp, DAE::Subscript::INDEX).clone(), var_field!((*sub2).exp, DAE::Subscript::INDEX).clone())?,
        (Deref @ DAE::Subscript::WHOLE_NONEXP { .. }, Deref @ DAE::Subscript::WHOLE_NONEXP { .. }) => compare(var_field!((*sub1).exp, DAE::Subscript::WHOLE_NONEXP).clone(), var_field!((*sub2).exp, DAE::Subscript::WHOLE_NONEXP).clone())?,
        _ => Util::intCompare(metamodelica::valueConstructor((&*sub1.clone()))?, metamodelica::valueConstructor((&*sub2.clone()))?),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(res)
}

fn compareSubscriptList(mut subs1: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut subs2: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<i32> {
    let mut comp: i32 = 0;
    let mut len1: i32 = 0;
    let mut len2: i32 = 0;
    let mut s2: Arc<DAE::Subscript> = Arc::new(DAE::Subscript::WHOLEDIM);
    let mut rest_subs2: Arc<metamodelica::List<Arc<DAE::Subscript>>> = subs2.clone();
    len1 = (subs1.clone().len() as i32);
    len2 = (subs2.clone().len() as i32);
    comp = Util::intCompare(len1.clone(), len2.clone());
    if comp.clone() != 0 {
        return Ok(comp.clone());
    }
    for mut s1 in &*subs1.clone() {
        let mut s1 = s1.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_subs2.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        s2 = __pa0.clone();
        rest_subs2 = __pa1.clone();
        comp = compareSubscripts(s1.clone(), s2.clone())?;
        if 0 != comp.clone() {
            return Ok(comp.clone());
        }
    }
    comp = 0;
    Ok(comp)
}

pub fn subscriptInt(mut inSubscript: Arc<DAE::Subscript>) -> i32 {
    let mut outInteger: i32 = expArrayIndex(subscriptIndexExp(inSubscript.clone()).unwrap()).unwrap();
    outInteger
}

pub fn subscriptsInt(mut inSubscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Arc<metamodelica::List<i32>> {
    let mut outIntegers: Arc<metamodelica::List<i32>> = metamodelica::nil();
    outIntegers = List::map(inSubscripts.clone(), (std::sync::Arc::new(fnptr!(subscriptInt, Arc<DAE::Subscript>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Subscript>) -> Result<i32> + 'static>));
    outIntegers
}

pub fn expArrayIndex(mut inExp: Arc<DAE::Exp>) -> Result<i32> {
    let mut outIndex: i32 = 0;
    outIndex = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::ICONST { .. } => var_field!((*inExp).integer, DAE::Exp::ICONST).clone(),
        Deref @ DAE::Exp::ENUM_LITERAL { .. } => var_field!((*inExp).index, DAE::Exp::ENUM_LITERAL).clone(),
        Deref @ DAE::Exp::BCONST { .. } => if (var_field!((*inExp).bool, DAE::Exp::BCONST).clone()) {2} else {1},
        _ => bail!("match: no arm matched"),
    } });
    Ok(outIndex)
}

pub fn subscriptIndexExp(mut inSubscript: Arc<DAE::Subscript>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let __pa0 = ::match_deref::match_deref! { match &(inSubscript.clone()) {
        Deref @ DAE::Subscript::INDEX { exp: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outExp = __pa0.clone();
    Ok(outExp)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn subscriptEqual(mut inSubscriptLst1: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut inSubscriptLst2: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &((inSubscriptLst1.clone(), inSubscriptLst2.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            true
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::WHOLEDIM, tail: xs1 }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::WHOLEDIM, tail: xs2 }) => {
            subscriptEqual(xs1.clone(), xs2.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::SLICE { exp: e1 }, tail: xs1 }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::SLICE { exp: e2 }, tail: xs2 }) => {
            if (expEqual(e1.clone(), e2.clone())?) {subscriptEqual(xs1.clone(), xs2.clone())?} else {false}
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: e1 }, tail: xs1 }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: e2 }, tail: xs2 }) => {
            if (expEqual(e1.clone(), e2.clone())?) {subscriptEqual(xs1.clone(), xs2.clone())?} else {false}
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::WHOLE_NONEXP { exp: e1 }, tail: xs1 }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::WHOLE_NONEXP { exp: e2 }, tail: xs2 }) => {
            if (expEqual(e1.clone(), e2.clone())?) {subscriptEqual(xs1.clone(), xs2.clone())?} else {false}
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBoolean)
}

pub fn printListStr<Type_a: Clone + 'static>(mut inTypeALst: Arc<metamodelica::List<Type_a>>, mut inFuncTypeTypeAToString: Arc<dyn ::std::ops::Fn(Type_a) -> Result<ArcStr> + 'static>, mut inString: ArcStr) -> ArcStr {
    pub type FuncTypeType_aToString<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Type_a) -> Result<ArcStr> + 'static>;

    let mut outString: ArcStr = arcstr::literal!("");
    outString = stringDelimitList(List::map(inTypeALst.clone(), inFuncTypeTypeAToString.clone()), (inString.clone()).clone());
    outString
}

pub fn printSubscriptStr(mut sub: Arc<DAE::Subscript>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ DAE::Subscript::WHOLEDIM => literal!(":"),
        Deref @ DAE::Subscript::INDEX { .. } => printExpStr(var_field!((*sub).exp, DAE::Subscript::INDEX).clone())?,
        Deref @ DAE::Subscript::SLICE { .. } => printExpStr(var_field!((*sub).exp, DAE::Subscript::SLICE).clone())?,
        Deref @ DAE::Subscript::WHOLE_NONEXP { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("1:")); __mm_s.push_str(&*printExpStr(var_field!((*sub).exp, DAE::Subscript::WHOLE_NONEXP).clone())?); ArcStr::from(__mm_s) },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

