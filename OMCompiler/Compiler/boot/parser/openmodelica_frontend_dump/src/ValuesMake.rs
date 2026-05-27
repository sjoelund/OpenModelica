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

use openmodelica_ast::Absyn;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::Values;
use openmodelica_util_datatypes_basic::List;

pub fn makeZero(mut ty: Arc<DAE::Type>) -> Result<Arc<Values::Value>> {
    let mut zero: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    zero = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_REAL { .. } => Arc::new(Values::Value::REAL { real: metamodelica::OrderedFloat(0.0_f64) }),
        Deref @ DAE::Type::T_INTEGER { .. } => Arc::new(Values::Value::INTEGER { integer: 0 }),
        _ => bail!("match: no arm matched"),
    } });
    Ok(zero)
}

pub fn makeBoolean(mut b: bool) -> Arc<Values::Value> {
    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    v = Arc::new(Values::Value::BOOL { boolean: b.clone() });
    v
}

pub fn makeReal(mut r: metamodelica::Real) -> Arc<Values::Value> {
    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    v = Arc::new(Values::Value::REAL { real: r.clone() });
    v
}

pub fn makeInteger(mut i: i32) -> Arc<Values::Value> {
    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    v = Arc::new(Values::Value::INTEGER { integer: i.clone() });
    v
}

pub fn makeString(mut s: ArcStr) -> Arc<Values::Value> {
    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    v = Arc::new(Values::Value::STRING { string: (s.clone()).clone() });
    v
}

pub fn makeTuple(mut inValueLst: Arc<metamodelica::List<Arc<Values::Value>>>) -> Arc<Values::Value> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outValue = Arc::new(Values::Value::TUPLE { valueLst: inValueLst.clone() });
    outValue
}

pub fn makeList(mut inValueLst: Arc<metamodelica::List<Arc<Values::Value>>>) -> Arc<Values::Value> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outValue = Arc::new(Values::Value::LIST { valueLst: inValueLst.clone() });
    outValue
}

pub fn makeArray(mut inValueLst: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outValue = 'mc: {
        let __mc_input = inValueLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                vlst @ Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { dimLst: il, .. }, tail: _ } => {
                    let mut i1: i32 = 0;
                    i1 = (vlst.clone().len() as i32);
                    Ok(Arc::new(Values::Value::ARRAY { valueLst: vlst.clone(), dimLst: cons(i1.clone(), il.clone()) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                vlst => {
                    let mut i1: i32 = 0;
                    i1 = (vlst.clone().len() as i32);
                    Ok(Arc::new(Values::Value::ARRAY { valueLst: vlst.clone(), dimLst: list![i1.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outValue)
}

pub fn makeEmptyArray() -> Arc<Values::Value> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::ARRAY { valueLst: metamodelica::nil(), dimLst: list![0] });
    outValue
}

pub fn makeStringArray(mut inReals: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<Values::Value>> {
    let mut outArray: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outArray = makeArray(List::map(inReals.clone(), (std::sync::Arc::new(fnptr!(makeString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Values::Value>> + 'static>)))?;
    Ok(outArray)
}

pub fn makeIntArray(mut inInts: Arc<metamodelica::List<i32>>) -> Result<Arc<Values::Value>> {
    let mut outArray: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outArray = makeArray(List::map(inInts.clone(), (std::sync::Arc::new(fnptr!(makeInteger, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<Arc<Values::Value>> + 'static>)))?;
    Ok(outArray)
}

pub fn makeRealArray(mut inReals: Arc<metamodelica::List<metamodelica::Real>>) -> Result<Arc<Values::Value>> {
    let mut outArray: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outArray = makeArray(List::map(inReals.clone(), (std::sync::Arc::new(fnptr!(makeReal, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real) -> Result<Arc<Values::Value>> + 'static>)))?;
    Ok(outArray)
}

pub fn makeRealMatrix(mut inReals: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>) -> Result<Arc<Values::Value>> {
    let mut outArray: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outArray = makeArray(List::map(inReals.clone(), (std::sync::Arc::new(makeRealArray) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<metamodelica::Real>>) -> Result<Arc<Values::Value>> + 'static>)))?;
    Ok(outArray)
}

pub fn makeCodeTypeName(mut path: Arc<Absyn::Path>) -> Arc<Values::Value> {
    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    val = Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: path.clone() }) });
    val
}

pub fn makeCodeTypeNameStr(mut r#str: ArcStr) -> Arc<Values::Value> {
    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    val = Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Arc::new(Absyn::Path::IDENT { name: (r#str.clone()).clone() }) }) });
    val
}

pub fn makeCodeTypeNameArray(mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Result<Arc<Values::Value>> {
    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    val = makeArray({
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut p in (paths.clone()).into_iter().cloned() {
            let __x = makeCodeTypeName(p.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })?;
    Ok(val)
}

