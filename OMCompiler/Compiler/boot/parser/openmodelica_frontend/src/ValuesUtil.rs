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

use crate::Expression;
use crate::ExpressionSimplify;
use crate::ExpressionSimplifyTypes;
use crate::Types;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ValuesDump;
use openmodelica_frontend_dump::ValuesMake;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::Values;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Print;
use openmodelica_util::System;
use openmodelica_util_datatypes_basic::List;

pub fn typeConvert(mut inType1: Arc<DAE::Type>, mut inType2: Arc<DAE::Type>, mut inValueLst3: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<metamodelica::List<Arc<Values::Value>>>> {
    let mut outValueLst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    outValueLst = (::match_deref::match_deref! { match &((inType1.clone(), inType2.clone(), inValueLst3.clone())) {
        (_, _, Deref @ metamodelica::List::Nil) => {
            metamodelica::nil()
        },
        (from @ Deref @ DAE::Type::T_INTEGER { .. }, to @ Deref @ DAE::Type::T_REAL { .. }, Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: vrest }) => {
            let mut vallst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut rval: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            vallst = typeConvert(from.clone(), to.clone(), vrest.clone())?;
            rval = intReal(i.clone());
            cons(Arc::new(Values::Value::REAL { real: rval.clone() }), vallst.clone())
        },
        (from @ Deref @ DAE::Type::T_REAL { .. }, to @ Deref @ DAE::Type::T_INTEGER { .. }, Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: r }, tail: vrest }) => {
            let mut vallst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut ival: i32 = 0;
            vallst = typeConvert(from.clone(), to.clone(), vrest.clone())?;
            ival = ((r.clone()).0 as i32);
            cons(Arc::new(Values::Value::INTEGER { integer: ival.clone() }), vallst.clone())
        },
        (from, to, Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { dimLst: dims, valueLst: vals }, tail: vrest }) => {
            let mut vallst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut vallst2: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            vallst = typeConvert(from.clone(), to.clone(), vals.clone())?;
            vallst2 = typeConvert(from.clone(), to.clone(), vrest.clone())?;
            cons(Arc::new(Values::Value::ARRAY { valueLst: vallst.clone(), dimLst: dims.clone() }), vallst2.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outValueLst)
}

pub fn valueExpType(mut inValue: Arc<Values::Value>) -> Result<Arc<DAE::Type>> {
    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    tp = 'mc: {
        let __mc_input = inValue.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::INTEGER { integer: _ } => {
                    Ok(DAE::T_INTEGER_DEFAULT().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::REAL { real: _ } => {
                    Ok(DAE::T_REAL_DEFAULT().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::BOOL { boolean: _ } => {
                    Ok(DAE::T_BOOL_DEFAULT().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::STRING { string: _ } => {
                    Ok(DAE::T_STRING_DEFAULT().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::ENUM_LITERAL { name: path, .. } => {
                    let mut path = (*path).clone();
                    path = AbsynUtil::pathPrefix(path.clone())?;
                    Ok(Arc::new(DAE::Type::T_ENUMERATION { index: None, path: path.clone(), names: metamodelica::nil(), literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::ARRAY { valueLst: valLst, dimLst: int_dims } => {
                    let mut eltTp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    eltTp = valueExpType(listHead(valLst.clone())?)?;
                    dims = List::map(int_dims.clone(), (std::sync::Arc::new(fnptr!(Expression::intDimension, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<Arc<DAE::Dimension>> + 'static>));
                    Ok(Arc::new(DAE::Type::T_ARRAY { ty: eltTp.clone(), dims: dims.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::RECORD { record_: path, orderd: valLst, comp: nameLst, index: _ } => {
                    let mut eltTps: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut varLst: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    eltTps = List::map(valLst.clone(), (std::sync::Arc::new(valueExpType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<Arc<DAE::Type>> + 'static>));
                    varLst = List::threadMap(eltTps.clone(), nameLst.clone(), (std::sync::Arc::new(fnptr!(valueExpTypeExpVar, Arc<DAE::Type>, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, ArcStr) -> Result<Arc<DAE::Var>> + 'static>));
                    Ok(Arc::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: path.clone() }, varLst: varLst.clone(), equalityConstraint: None, usedExternally: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("valueExpType on ")); __mm_s.push_str(&*ValuesDump::valString(inValue.clone())?); __mm_s.push_str(&*literal!(" not implemented yet\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(tp)
}

fn valueExpTypeExpVar(mut etp: Arc<DAE::Type>, mut name: ArcStr) -> Arc<DAE::Var> {
    let mut expVar: Arc<DAE::Var>;
    expVar = Arc::new(DAE::Var { name: (name.clone()).clone(), attributes: DAE::dummyAttrVar().clone(), ty: etp.clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None });
    expVar
}

pub fn isZero(mut inValue: Arc<Values::Value>) -> bool {
    let mut isZero: bool = false;
    isZero = (::match_deref::match_deref! { match &(inValue.clone()) {
        Deref @ Values::Value::REAL { real: rval } => {
            realEq(rval.clone(), metamodelica::OrderedFloat(0.0_f64))
        },
        Deref @ Values::Value::INTEGER { integer: ival } => {
            intEq(ival.clone(), 0)
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isZero
}

pub fn isArray(mut inValue: Arc<Values::Value>) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inValue.clone()) {
        Deref @ Values::Value::ARRAY { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isRecord(mut inValue: Arc<Values::Value>) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inValue.clone()) {
        Deref @ Values::Value::RECORD { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn nthArrayelt(mut inValue: Arc<Values::Value>, mut inInteger: i32) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut vlst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(inValue.clone()) {
        Deref @ Values::Value::ARRAY { valueLst: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    vlst = __pa0.clone();
    outValue = (vlst.clone()).get(inInteger.clone())?;
    Ok(outValue)
}

pub fn safeIntRealOp(mut val1: Arc<Values::Value>, mut val2: Arc<Values::Value>, mut op: Values::IntRealOp) -> Result<Arc<Values::Value>> {
    let mut outv: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outv = 'mc: {
        let __mc_input = (val1.clone(), val2.clone(), op.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::INTEGER { integer: iv1 }, Deref @ Values::Value::INTEGER { integer: iv2 }, Values::IntRealOp::MULOP) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut outv: Arc<Values::Value> = outv.clone();
                    e = ExpressionSimplify::safeIntOp(iv1.clone(), iv2.clone(), crate::ExpressionSimplifyTypes::IntOp::MULOP)?;
                    outv = expValue(e.clone())?;
                    Ok(outv.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::REAL { real: rv1 }, Deref @ Values::Value::INTEGER { integer: iv2 }, Values::IntRealOp::MULOP) => {
                    let mut rv2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut rv3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    rv2 = intReal(iv2.clone());
                    rv3 = rv1.clone() * rv2.clone();
                    Ok(Arc::new(Values::Value::REAL { real: rv3.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::INTEGER { integer: iv1 }, Deref @ Values::Value::REAL { real: rv2 }, Values::IntRealOp::MULOP) => {
                    let mut rv1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut rv3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    rv1 = intReal(iv1.clone());
                    rv3 = rv1.clone() * rv2.clone();
                    Ok(Arc::new(Values::Value::REAL { real: rv3.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::REAL { real: rv1 }, Deref @ Values::Value::REAL { real: rv2 }, Values::IntRealOp::MULOP) => {
                    let mut rv3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    rv3 = rv1.clone() * rv2.clone();
                    Ok(Arc::new(Values::Value::REAL { real: rv3.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::INTEGER { integer: iv1 }, Deref @ Values::Value::INTEGER { integer: iv2 }, Values::IntRealOp::DIVOP) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut outv: Arc<Values::Value> = outv.clone();
                    e = ExpressionSimplify::safeIntOp(iv1.clone(), iv2.clone(), crate::ExpressionSimplifyTypes::IntOp::DIVOP)?;
                    outv = expValue(e.clone())?;
                    Ok(outv.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::REAL { real: rv1 }, Deref @ Values::Value::INTEGER { integer: iv2 }, Values::IntRealOp::DIVOP) => {
                    let mut rv2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut rv3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    rv2 = intReal(iv2.clone());
                    rv3 = rv1.clone() / rv2.clone();
                    Ok(Arc::new(Values::Value::REAL { real: rv3.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::INTEGER { integer: iv1 }, Deref @ Values::Value::REAL { real: rv2 }, Values::IntRealOp::DIVOP) => {
                    let mut rv1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut rv3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    rv1 = intReal(iv1.clone());
                    rv3 = rv1.clone() / rv2.clone();
                    Ok(Arc::new(Values::Value::REAL { real: rv3.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::REAL { real: rv1 }, Deref @ Values::Value::REAL { real: rv2 }, Values::IntRealOp::DIVOP) => {
                    let mut rv3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    rv3 = rv1.clone() / rv2.clone();
                    Ok(Arc::new(Values::Value::REAL { real: rv3.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::INTEGER { integer: iv1 }, Deref @ Values::Value::INTEGER { integer: iv2 }, Values::IntRealOp::POWOP) => {
                    let mut rv1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut rv2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut rv3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let true = (iv2.clone() < 0) else { bail!("pattern mismatch") };
                    rv1 = intReal(iv1.clone());
                    rv2 = intReal(iv2.clone());
                    rv3 = realPow(rv1.clone(), rv2.clone());
                    Ok(Arc::new(Values::Value::REAL { real: rv3.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::INTEGER { integer: iv1 }, Deref @ Values::Value::INTEGER { integer: iv2 }, Values::IntRealOp::POWOP) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut outv: Arc<Values::Value> = outv.clone();
                    e = ExpressionSimplify::safeIntOp(iv1.clone(), iv2.clone(), crate::ExpressionSimplifyTypes::IntOp::POWOP)?;
                    outv = expValue(e.clone())?;
                    Ok(outv.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::REAL { real: rv1 }, Deref @ Values::Value::INTEGER { integer: iv2 }, Values::IntRealOp::POWOP) => {
                    let mut rv2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut rv3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    rv2 = intReal(iv2.clone());
                    rv3 = realPow(rv1.clone(), rv2.clone());
                    Ok(Arc::new(Values::Value::REAL { real: rv3.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::INTEGER { integer: iv1 }, Deref @ Values::Value::REAL { real: rv2 }, Values::IntRealOp::POWOP) => {
                    let mut iv2: i32 = 0;
                    let mut e: Arc<DAE::Exp>;
                    let mut outv: Arc<Values::Value> = outv.clone();
                    iv2 = ((rv2.clone()).0 as i32);
                    e = ExpressionSimplify::safeIntOp(iv1.clone(), iv2.clone(), crate::ExpressionSimplifyTypes::IntOp::POWOP)?;
                    outv = expValue(e.clone())?;
                    Ok(outv.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::INTEGER { integer: iv1 }, Deref @ Values::Value::REAL { real: rv2 }, Values::IntRealOp::POWOP) => {
                    let mut rv1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut rv3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    rv1 = intReal(iv1.clone());
                    rv3 = realPow(rv1.clone(), rv2.clone());
                    Ok(Arc::new(Values::Value::REAL { real: rv3.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::REAL { real: rv1 }, Deref @ Values::Value::REAL { real: rv2 }, Values::IntRealOp::POWOP) => {
                    let mut rv3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    rv3 = realPow(rv1.clone(), rv2.clone());
                    Ok(Arc::new(Values::Value::REAL { real: rv3.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::INTEGER { integer: iv1 }, Deref @ Values::Value::INTEGER { integer: iv2 }, Values::IntRealOp::ADDOP) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut outv: Arc<Values::Value> = outv.clone();
                    e = ExpressionSimplify::safeIntOp(iv1.clone(), iv2.clone(), crate::ExpressionSimplifyTypes::IntOp::ADDOP)?;
                    outv = expValue(e.clone())?;
                    Ok(outv.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::REAL { real: rv1 }, Deref @ Values::Value::INTEGER { integer: iv2 }, Values::IntRealOp::ADDOP) => {
                    let mut rv2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut rv3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    rv2 = intReal(iv2.clone());
                    rv3 = rv1.clone() + rv2.clone();
                    Ok(Arc::new(Values::Value::REAL { real: rv3.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::INTEGER { integer: iv1 }, Deref @ Values::Value::REAL { real: rv2 }, Values::IntRealOp::ADDOP) => {
                    let mut rv1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut rv3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    rv1 = intReal(iv1.clone());
                    rv3 = rv1.clone() + rv2.clone();
                    Ok(Arc::new(Values::Value::REAL { real: rv3.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::REAL { real: rv1 }, Deref @ Values::Value::REAL { real: rv2 }, Values::IntRealOp::ADDOP) => {
                    let mut rv3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    rv3 = rv1.clone() + rv2.clone();
                    Ok(Arc::new(Values::Value::REAL { real: rv3.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::INTEGER { integer: iv1 }, Deref @ Values::Value::INTEGER { integer: iv2 }, Values::IntRealOp::SUBOP) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut outv: Arc<Values::Value> = outv.clone();
                    e = ExpressionSimplify::safeIntOp(iv1.clone(), iv2.clone(), crate::ExpressionSimplifyTypes::IntOp::SUBOP)?;
                    outv = expValue(e.clone())?;
                    Ok(outv.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::REAL { real: rv1 }, Deref @ Values::Value::INTEGER { integer: iv2 }, Values::IntRealOp::SUBOP) => {
                    let mut rv2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut rv3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    rv2 = intReal(iv2.clone());
                    rv3 = rv1.clone() - rv2.clone();
                    Ok(Arc::new(Values::Value::REAL { real: rv3.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::INTEGER { integer: iv1 }, Deref @ Values::Value::REAL { real: rv2 }, Values::IntRealOp::SUBOP) => {
                    let mut rv1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut rv3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    rv1 = intReal(iv1.clone());
                    rv3 = rv1.clone() - rv2.clone();
                    Ok(Arc::new(Values::Value::REAL { real: rv3.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::REAL { real: rv1 }, Deref @ Values::Value::REAL { real: rv2 }, Values::IntRealOp::SUBOP) => {
                    let mut rv3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    rv3 = rv1.clone() - rv2.clone();
                    Ok(Arc::new(Values::Value::REAL { real: rv3.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outv)
}

pub fn safeLessEq(mut val1: Arc<Values::Value>, mut val2: Arc<Values::Value>) -> Result<bool> {
    let mut outv: bool = false;
    outv = (::match_deref::match_deref! { match &((val1.clone(), val2.clone())) {
        (Deref @ Values::Value::REAL { real: r1 }, Deref @ Values::Value::REAL { real: r2 }) => {
            r1.clone() <= r2.clone()
        },
        (Deref @ Values::Value::REAL { real: r1 }, _) => {
            let mut r2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            r2 = intReal(valueInteger(val2.clone())?);
            r1.clone() <= r2.clone()
        },
        (_, Deref @ Values::Value::REAL { real: r2 }) => {
            let mut r1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            r1 = intReal(valueInteger(val1.clone())?);
            r1.clone() <= r2.clone()
        },
        (_, _) => {
            let mut i1: i32 = 0;
            let mut i2: i32 = 0;
            i1 = valueInteger(val1.clone())?;
            i2 = valueInteger(val2.clone())?;
            i1.clone() <= i2.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outv)
}

pub fn writeToFileAsArgs(mut vallst: Arc<metamodelica::List<Arc<Values::Value>>>, mut filename: ArcStr) -> Result<()> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = (ValuesDump::unparseValues(vallst.clone())?).clone();
    System::writeFile((filename.clone()).clone(), (r#str.clone()).clone())?;
    Ok(())
}

pub fn addElementwiseArrayelt(mut inValueLst1: Arc<metamodelica::List<Arc<Values::Value>>>, mut inValueLst2: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<metamodelica::List<Arc<Values::Value>>>> {
    let mut outValueLst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    outValueLst = (::match_deref::match_deref! { match &((inValueLst1.clone(), inValueLst2.clone())) {
        (Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { dimLst: dims, valueLst: v1lst }, tail: rest1 }, Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: v2lst, .. }, tail: rest2 }) => {
            let mut reslst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut res2: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            reslst = addElementwiseArrayelt(v1lst.clone(), v2lst.clone())?;
            res2 = addElementwiseArrayelt(rest1.clone(), rest2.clone())?;
            cons(Arc::new(Values::Value::ARRAY { valueLst: reslst.clone(), dimLst: dims.clone() }), res2.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: v1 }, tail: rest1 }, Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: v2 }, tail: rest2 }) => {
            let mut res2: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut res: i32 = 0;
            res = v1.clone() + v2.clone();
            res2 = addElementwiseArrayelt(rest1.clone(), rest2.clone())?;
            cons(Arc::new(Values::Value::INTEGER { integer: res.clone() }), res2.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: r1 }, tail: rest1 }, Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: r2 }, tail: rest2 }) => {
            let mut res2: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut rres: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            rres = r1.clone() + r2.clone();
            res2 = addElementwiseArrayelt(rest1.clone(), rest2.clone())?;
            cons(Arc::new(Values::Value::REAL { real: rres.clone() }), res2.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: s1 }, tail: rest1 }, Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: s2 }, tail: rest2 }) => {
            let mut res2: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut sres: ArcStr = arcstr::literal!("");
            sres = (stringAppend((s1.clone()).clone(), (s2.clone()).clone())).clone();
            res2 = addElementwiseArrayelt(rest1.clone(), rest2.clone())?;
            cons(Arc::new(Values::Value::STRING { string: (sres.clone()).clone() }), res2.clone())
        },
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            metamodelica::nil()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outValueLst)
}

pub fn subElementwiseArrayelt(mut inValueLst1: Arc<metamodelica::List<Arc<Values::Value>>>, mut inValueLst2: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<metamodelica::List<Arc<Values::Value>>>> {
    let mut outValueLst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    outValueLst = (::match_deref::match_deref! { match &((inValueLst1.clone(), inValueLst2.clone())) {
        (Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { dimLst: dims, valueLst: v1lst }, tail: rest1 }, Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: v2lst, .. }, tail: rest2 }) => {
            let mut reslst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut res2: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            reslst = subElementwiseArrayelt(v1lst.clone(), v2lst.clone())?;
            res2 = subElementwiseArrayelt(rest1.clone(), rest2.clone())?;
            cons(Arc::new(Values::Value::ARRAY { valueLst: reslst.clone(), dimLst: dims.clone() }), res2.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: v1 }, tail: rest1 }, Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: v2 }, tail: rest2 }) => {
            let mut res2: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut res: i32 = 0;
            res = v1.clone() - v2.clone();
            res2 = subElementwiseArrayelt(rest1.clone(), rest2.clone())?;
            cons(Arc::new(Values::Value::INTEGER { integer: res.clone() }), res2.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: r1 }, tail: rest1 }, Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: r2 }, tail: rest2 }) => {
            let mut res2: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut rres: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            rres = r1.clone() - r2.clone();
            res2 = subElementwiseArrayelt(rest1.clone(), rest2.clone())?;
            cons(Arc::new(Values::Value::REAL { real: rres.clone() }), res2.clone())
        },
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            metamodelica::nil()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outValueLst)
}

pub fn mulElementwiseArrayelt(mut inValueLst1: Arc<metamodelica::List<Arc<Values::Value>>>, mut inValueLst2: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<metamodelica::List<Arc<Values::Value>>>> {
    let mut outValueLst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    outValueLst = (::match_deref::match_deref! { match &((inValueLst1.clone(), inValueLst2.clone())) {
        (Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { dimLst: dims, valueLst: v1lst }, tail: rest1 }, Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: v2lst, .. }, tail: rest2 }) => {
            let mut reslst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut res2: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            reslst = mulElementwiseArrayelt(v1lst.clone(), v2lst.clone())?;
            res2 = mulElementwiseArrayelt(rest1.clone(), rest2.clone())?;
            cons(Arc::new(Values::Value::ARRAY { valueLst: reslst.clone(), dimLst: dims.clone() }), res2.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: v1 }, tail: rest1 }, Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: v2 }, tail: rest2 }) => {
            let mut res2: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut res: i32 = 0;
            res = v1.clone() * v2.clone();
            res2 = mulElementwiseArrayelt(rest1.clone(), rest2.clone())?;
            cons(Arc::new(Values::Value::INTEGER { integer: res.clone() }), res2.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: r1 }, tail: rest1 }, Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: r2 }, tail: rest2 }) => {
            let mut res2: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut rres: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            rres = r1.clone() * r2.clone();
            res2 = mulElementwiseArrayelt(rest1.clone(), rest2.clone())?;
            cons(Arc::new(Values::Value::REAL { real: rres.clone() }), res2.clone())
        },
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            metamodelica::nil()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outValueLst)
}

pub fn divElementwiseArrayelt(mut inValueLst1: Arc<metamodelica::List<Arc<Values::Value>>>, mut inValueLst2: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<metamodelica::List<Arc<Values::Value>>>> {
    let mut outValueLst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    outValueLst = (::match_deref::match_deref! { match &((inValueLst1.clone(), inValueLst2.clone())) {
        (Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { dimLst: dims, valueLst: v1lst }, tail: rest1 }, Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: v2lst, .. }, tail: rest2 }) => {
            let mut reslst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut res2: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            reslst = divElementwiseArrayelt(v1lst.clone(), v2lst.clone())?;
            res2 = divElementwiseArrayelt(rest1.clone(), rest2.clone())?;
            cons(Arc::new(Values::Value::ARRAY { valueLst: reslst.clone(), dimLst: dims.clone() }), res2.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: i1 }, tail: rest1 }, Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: i2 }, tail: rest2 }) => {
            let mut res2: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut res: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut r1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut r2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            r1 = intReal(i1.clone());
            r2 = intReal(i2.clone());
            res = r1.clone() / r2.clone();
            res2 = divElementwiseArrayelt(rest1.clone(), rest2.clone())?;
            cons(Arc::new(Values::Value::REAL { real: res.clone() }), res2.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: r1 }, tail: rest1 }, Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: r2 }, tail: rest2 }) => {
            let mut res2: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut res: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            res = r1.clone() / r2.clone();
            res2 = divElementwiseArrayelt(rest1.clone(), rest2.clone())?;
            cons(Arc::new(Values::Value::REAL { real: res.clone() }), res2.clone())
        },
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            metamodelica::nil()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outValueLst)
}

pub fn powElementwiseArrayelt(mut inValueLst1: Arc<metamodelica::List<Arc<Values::Value>>>, mut inValueLst2: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<metamodelica::List<Arc<Values::Value>>>> {
    let mut outValueLst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    outValueLst = (::match_deref::match_deref! { match &((inValueLst1.clone(), inValueLst2.clone())) {
        (Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { dimLst: dims, valueLst: v1lst }, tail: rest1 }, Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: v2lst, .. }, tail: rest2 }) => {
            let mut reslst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut res2: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            reslst = powElementwiseArrayelt(v1lst.clone(), v2lst.clone())?;
            res2 = powElementwiseArrayelt(rest1.clone(), rest2.clone())?;
            cons(Arc::new(Values::Value::ARRAY { valueLst: reslst.clone(), dimLst: dims.clone() }), res2.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: i1 }, tail: rest1 }, Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: i2 }, tail: rest2 }) => {
            let mut res2: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut res: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut r1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut r2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            r1 = intReal(i1.clone());
            r2 = intReal(i2.clone());
            res = (r1.clone()).powf(r2.clone());
            res2 = powElementwiseArrayelt(rest1.clone(), rest2.clone())?;
            cons(Arc::new(Values::Value::REAL { real: res.clone() }), res2.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: r1 }, tail: rest1 }, Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: r2 }, tail: rest2 }) => {
            let mut res2: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut res: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            res = (r1.clone()).powf(r2.clone());
            res2 = powElementwiseArrayelt(rest1.clone(), rest2.clone())?;
            cons(Arc::new(Values::Value::REAL { real: res.clone() }), res2.clone())
        },
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            metamodelica::nil()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outValueLst)
}

pub fn absynExpValue(mut exp: Arc<Absyn::Exp>) -> Result<Arc<Values::Value>> {
    let mut value: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    value = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::Exp::INTEGER { .. } => Arc::new(Values::Value::INTEGER { integer: var_field!((*exp).value, Absyn::Exp::INTEGER).clone() }),
        Deref @ Absyn::Exp::REAL { .. } => Arc::new(Values::Value::REAL { real: stringReal((var_field!((*exp).value, Absyn::Exp::REAL).clone()).clone())? }),
        Deref @ Absyn::Exp::CREF { .. } => Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_VARIABLENAME { componentRef: var_field!((*exp).componentRef, Absyn::Exp::CREF).clone() }) }),
        Deref @ Absyn::Exp::STRING { .. } => Arc::new(Values::Value::STRING { string: (var_field!((*exp).value, Absyn::Exp::STRING).clone()).clone() }),
        Deref @ Absyn::Exp::BOOL { .. } => Arc::new(Values::Value::BOOL { boolean: var_field!((*exp).value, Absyn::Exp::BOOL).clone() }),
        Deref @ Absyn::Exp::ARRAY { .. } => ValuesMake::makeArray({
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).arrayExp, Absyn::Exp::ARRAY).clone()).into_iter().cloned() {
            let __x = absynExpValue(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })?,
        Deref @ Absyn::Exp::TUPLE { .. } => ValuesMake::makeTuple({
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).expressions, Absyn::Exp::TUPLE).clone()).into_iter().cloned() {
            let __x = absynExpValue(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
        Deref @ Absyn::Exp::CODE { .. } => Arc::new(Values::Value::CODE { A: var_field!((*exp).code, Absyn::Exp::CODE).clone() }),
        _ => Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_EXPRESSION { exp: exp.clone() }) }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(value)
}

pub fn expValue(mut inExp: Arc<DAE::Exp>) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outValue = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::ICONST { integer: i } => {
            Arc::new(Values::Value::INTEGER { integer: i.clone() })
        },
        Deref @ DAE::Exp::RCONST { real: r } => {
            Arc::new(Values::Value::REAL { real: r.clone() })
        },
        Deref @ DAE::Exp::SCONST { string: s } => {
            Arc::new(Values::Value::STRING { string: (s.clone()).clone() })
        },
        Deref @ DAE::Exp::BCONST { bool: b } => {
            Arc::new(Values::Value::BOOL { boolean: b.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outValue)
}

pub fn valueExp(mut inValue: Arc<Values::Value>, mut originalExp: Option<Arc<DAE::Exp>>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(inValue.clone()) {
        Deref @ Values::Value::INTEGER { integer: i } => {
            Arc::new(DAE::Exp::ICONST { integer: i.clone() })
        },
        Deref @ Values::Value::REAL { real: r } => {
            Arc::new(DAE::Exp::RCONST { real: r.clone() })
        },
        Deref @ Values::Value::STRING { string: s } => {
            Arc::new(DAE::Exp::SCONST { string: (s.clone()).clone() })
        },
        Deref @ Values::Value::BOOL { boolean: b } => {
            Arc::new(DAE::Exp::BCONST { bool: b.clone() })
        },
        Deref @ Values::Value::ENUM_LITERAL { index: i, name: path } => {
            Arc::new(DAE::Exp::ENUM_LITERAL { name: path.clone(), index: i.clone() })
        },
        Deref @ Values::Value::ARRAY { dimLst: int_dims, valueLst: vallist } => {
            valueExpArray(vallist.clone(), int_dims.clone(), originalExp.clone())?
        },
        Deref @ Values::Value::TUPLE { valueLst: vallist } => {
            let mut explist: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            explist = List::map(vallist.clone(), Arc::new({ let __pe_b1 = None; move |__pe_a0| valueExp(__pe_a0, __pe_b1.clone()) }));
            Arc::new(DAE::Exp::TUPLE { PR: explist.clone() })
        },
        Deref @ Values::Value::RECORD { record_: path, orderd: vallist, comp: namelst, index: (-1) } => {
            let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut tpl: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            let mut varlst: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
            expl = List::map(vallist.clone(), Arc::new({ let __pe_b1 = None; move |__pe_a0| valueExp(__pe_a0, __pe_b1.clone()) }));
            tpl = List::map(expl.clone(), (std::sync::Arc::new(Expression::r#typeof) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Type>> + 'static>));
            varlst = List::threadMap(namelst.clone(), tpl.clone(), (std::sync::Arc::new(fnptr!(Expression::makeVar, ArcStr, Arc<DAE::Type>)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<DAE::Type>) -> Result<Arc<DAE::Var>> + 'static>));
            t = Arc::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: path.clone() }, varLst: varlst.clone(), equalityConstraint: None, usedExternally: false });
            Arc::new(DAE::Exp::RECORD { path: path.clone(), exps: expl.clone(), comp: namelst.clone(), ty: t.clone() })
        },
        Deref @ Values::Value::ENUM_LITERAL { index: ix, name: path } => {
            Arc::new(DAE::Exp::ENUM_LITERAL { name: path.clone(), index: ix.clone() })
        },
        Deref @ Values::Value::TUPLE { valueLst: vallist } => {
            let mut explist: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            explist = List::map(vallist.clone(), Arc::new({ let __pe_b1 = None; move |__pe_a0| valueExp(__pe_a0, __pe_b1.clone()) }));
            Arc::new(DAE::Exp::TUPLE { PR: explist.clone() })
        },
        Deref @ Values::Value::OPTION { some: Some(v) } => {
            let mut e: Arc<DAE::Exp>;
            e = valueExp(v.clone(), None)?;
            (e, _) = Types::matchType(e.clone(), Types::typeOfValue(v.clone())?, DAE::T_METABOXED_DEFAULT().clone(), true)?;
            Arc::new(DAE::Exp::META_OPTION { exp: Some(e.clone()) })
        },
        Deref @ Values::Value::OPTION { some: None } => {
            Arc::new(DAE::Exp::META_OPTION { exp: None })
        },
        Deref @ Values::Value::META_TUPLE { valueLst: vallist } => {
            let mut explist: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut typelist: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            explist = List::map(vallist.clone(), Arc::new({ let __pe_b1 = None; move |__pe_a0| valueExp(__pe_a0, __pe_b1.clone()) }));
            typelist = List::map(vallist.clone(), (std::sync::Arc::new(Types::typeOfValue) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<Arc<DAE::Type>> + 'static>));
            (explist, _) = Types::matchTypeTuple(explist.clone(), typelist.clone(), List::map(typelist.clone(), (std::sync::Arc::new(Types::boxIfUnboxedType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>)), true)?;
            Arc::new(DAE::Exp::META_TUPLE { listExp: explist.clone() })
        },
        Deref @ Values::Value::LIST { valueLst: Deref @ metamodelica::List::Nil } => {
            Arc::new(DAE::Exp::LIST { valList: metamodelica::nil() })
        },
        Deref @ Values::Value::LIST { valueLst: vallist } => {
            let mut explist: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut vt: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut typelist: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            explist = List::map(vallist.clone(), Arc::new({ let __pe_b1 = None; move |__pe_a0| valueExp(__pe_a0, __pe_b1.clone()) }));
            typelist = List::map(vallist.clone(), (std::sync::Arc::new(Types::typeOfValue) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<Arc<DAE::Type>> + 'static>));
            vt = Types::boxIfUnboxedType(List::reduce(typelist.clone(), (std::sync::Arc::new(Types::superType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>))?)?;
            (explist, _) = Types::matchTypes(explist.clone(), typelist.clone(), vt.clone(), true)?;
            Arc::new(DAE::Exp::LIST { valList: explist.clone() })
        },
        Deref @ Values::Value::META_ARRAY { valueLst: vallist } => {
            let mut explist: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut vt: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut typelist: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            explist = List::map(vallist.clone(), Arc::new({ let __pe_b1 = None; move |__pe_a0| valueExp(__pe_a0, __pe_b1.clone()) }));
            typelist = List::map(vallist.clone(), (std::sync::Arc::new(Types::typeOfValue) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<Arc<DAE::Type>> + 'static>));
            vt = Types::boxIfUnboxedType(List::reduce(typelist.clone(), (std::sync::Arc::new(Types::superType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>))?)?;
            (explist, _) = Types::matchTypes(explist.clone(), typelist.clone(), vt.clone(), true)?;
            Expression::makeBuiltinCall((literal!("listArrayLiteral")).clone(), list![Arc::new(DAE::Exp::LIST { valList: explist.clone() })], Arc::new(DAE::Type::T_METAARRAY { ty: vt.clone() }), false)
        },
        Deref @ Values::Value::RECORD { record_: path, orderd: vallist, comp: namelst, index: ix } => {
            let mut explist: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut typelist: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            let true = (ix.clone() >= 0) else { bail!("pattern mismatch") };
            explist = List::map(vallist.clone(), Arc::new({ let __pe_b1 = None; move |__pe_a0| valueExp(__pe_a0, __pe_b1.clone()) }));
            typelist = List::map(vallist.clone(), (std::sync::Arc::new(Types::typeOfValue) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<Arc<DAE::Type>> + 'static>));
            (explist, _) = Types::matchTypeTuple(explist.clone(), typelist.clone(), List::map(typelist.clone(), (std::sync::Arc::new(Types::boxIfUnboxedType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Type>> + 'static>)), true)?;
            Arc::new(DAE::Exp::METARECORDCALL { path: path.clone(), args: explist.clone(), fieldNames: namelst.clone(), index: ix.clone(), typeVars: metamodelica::nil() })
        },
        Deref @ Values::Value::META_FAIL => {
            Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("fail")).clone() }), expLst: metamodelica::nil(), attr: DAE::callAttrBuiltinOther().clone() })
        },
        Deref @ Values::Value::META_BOX { value: v } => {
            let mut e: Arc<DAE::Exp>;
            e = valueExp(v.clone(), None)?;
            Arc::new(DAE::Exp::BOX { exp: e.clone() })
        },
        Deref @ Values::Value::CODE { A: code } => {
            Arc::new(DAE::Exp::CODE { code: code.clone(), ty: DAE::T_UNKNOWN_DEFAULT().clone() })
        },
        Deref @ Values::Value::EMPTY { ty: valType, tyStr, name, scope } => {
            let mut e: Arc<DAE::Exp>;
            let mut ety: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            if isSome(originalExp.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(originalExp.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                e = __pa0.clone();
            } else {
                ety = Types::simplifyType(Types::typeOfValue(valType.clone())?)?;
                e = Arc::new(DAE::Exp::EMPTY { scope: (scope.clone()).clone(), name: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (name.clone()).clone(), identType: ety.clone(), subscriptLst: metamodelica::nil() }), ty: ety.clone(), tyStr: (tyStr.clone()).clone() });
            }
            e.clone()
        },
        Deref @ Values::Value::NORETCALL => {
            Arc::new(DAE::Exp::TUPLE { PR: metamodelica::nil() })
        },
        v => {
            let mut s: ArcStr = arcstr::literal!("");
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ValuesUtil.valueExp failed for ")); __mm_s.push_str(&*ValuesDump::valString(v.clone())?); ArcStr::from(__mm_s) }).clone();
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(s.clone()).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

fn valueExpArray(mut values: Arc<metamodelica::List<Arc<Values::Value>>>, mut inDims: Arc<metamodelica::List<i32>>, mut originalExp: Option<Arc<DAE::Exp>>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (values.clone(), inDims.clone(), originalExp.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _) => {
                    Ok(Arc::new(DAE::Exp::ARRAY { ty: DAE::T_UNKNOWN_DEFAULT().clone(), scalar: false, array: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    dims = List::map(inDims.clone(), (std::sync::Arc::new(fnptr!(Expression::intDimension, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<Arc<DAE::Dimension>> + 'static>));
                    Ok(Arc::new(DAE::Exp::ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_UNKNOWN_DEFAULT().clone(), dims: dims.clone() }), scalar: false, array: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::List::Cons { head: v, tail: xs }, .. }, tail: xs2 }, Deref @ metamodelica::List::Cons { head: dim, tail: int_dims }, _) => {
                    let mut explist: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut mexpl: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
                    if '__try0: {
                        ::match_deref::match_deref! { match &(v.clone()) {
                            Deref @ Values::Value::ARRAY { .. } => (),
                            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                        } };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    explist = List::map(cons(v.clone(), xs.clone()), Arc::new({ let __pe_b1 = None; move |__pe_a0| valueExp(__pe_a0, __pe_b1.clone()) }));
                    let (__pa1, __pa2) = ::match_deref::match_deref! { match &(valueExp(Arc::new(Values::Value::ARRAY { valueLst: xs2.clone(), dimLst: int_dims.clone() }), None)?) {
                        Deref @ DAE::Exp::MATRIX { ty: __pa1, integer: _, matrix: __pa2 } => (__pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    t = __pa1.clone();
                    mexpl = __pa2.clone();
                    t = Expression::arrayDimensionSetFirst(t.clone(), Arc::new(DAE::Dimension::DIM_INTEGER { integer: dim.clone() }))?;
                    Ok(Arc::new(DAE::Exp::MATRIX { ty: t.clone(), integer: dim.clone(), matrix: cons(explist.clone(), mexpl.clone()) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::List::Cons { head: v, tail: xs }, .. }, tail: Deref @ metamodelica::List::Nil }, _, _) => {
                    let mut explist: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut vt: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dim: i32 = 0;
                    if '__try0: {
                        ::match_deref::match_deref! { match &(v.clone()) {
                            Deref @ Values::Value::ARRAY { .. } => (),
                            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                        } };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    dim = (cons(v.clone(), xs.clone()).len() as i32);
                    explist = List::map(cons(v.clone(), xs.clone()), Arc::new({ let __pe_b1 = None; move |__pe_a0| valueExp(__pe_a0, __pe_b1.clone()) }));
                    vt = Types::typeOfValue(v.clone())?;
                    t = Types::simplifyType(vt.clone())?;
                    dim = (cons(v.clone(), xs.clone()).len() as i32);
                    t = Expression::liftArrayR(t.clone(), Arc::new(DAE::Dimension::DIM_INTEGER { integer: dim.clone() }));
                    t = Expression::liftArrayR(t.clone(), Arc::new(DAE::Dimension::DIM_INTEGER { integer: 1 }));
                    Ok(Arc::new(DAE::Exp::MATRIX { ty: t.clone(), integer: dim.clone(), matrix: list![explist.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: v, tail: xs }, _, Some(Deref @ DAE::Exp::ARRAY { array: exps1, .. })) => {
                    let mut explist: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut vt: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dim: i32 = 0;
                    let mut b: bool = false;
                    explist = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for (e1, e2) in (&(values.clone())).into_iter().zip((&(exps1.clone())).into_iter()) {
                    let __x = valueExp(e1.clone(), Some(e2.clone()))?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    vt = Types::typeOfValue(v.clone())?;
                    t = Types::simplifyType(vt.clone())?;
                    dim = (cons(v.clone(), xs.clone()).len() as i32);
                    t = Expression::liftArrayR(t.clone(), Arc::new(DAE::Dimension::DIM_INTEGER { integer: dim.clone() }));
                    b = Types::isArray(vt.clone());
                    b = boolNot(b.clone());
                    Ok(Arc::new(DAE::Exp::ARRAY { ty: t.clone(), scalar: b.clone(), array: explist.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: v, tail: xs }, _, _) => {
                    let mut explist: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut vt: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dim: i32 = 0;
                    let mut b: bool = false;
                    explist = List::map(cons(v.clone(), xs.clone()), Arc::new({ let __pe_b1 = None; move |__pe_a0| valueExp(__pe_a0, __pe_b1.clone()) }));
                    vt = Types::typeOfValue(v.clone())?;
                    t = Types::simplifyType(vt.clone())?;
                    dim = (cons(v.clone(), xs.clone()).len() as i32);
                    t = Expression::liftArrayR(t.clone(), Arc::new(DAE::Dimension::DIM_INTEGER { integer: dim.clone() }));
                    b = Types::isArray(vt.clone());
                    b = boolNot(b.clone());
                    Ok(Arc::new(DAE::Exp::ARRAY { ty: t.clone(), scalar: b.clone(), array: explist.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

pub fn valueReal(mut inValue: Arc<Values::Value>) -> Result<metamodelica::Real> {
    let mut outReal: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    outReal = (::match_deref::match_deref! { match &(inValue.clone()) {
        Deref @ Values::Value::REAL { .. } => var_field!((*inValue).real, Values::Value::REAL).clone(),
        Deref @ Values::Value::INTEGER { .. } => intReal(var_field!((*inValue).integer, Values::Value::INTEGER).clone()),
        _ => bail!("match: no arm matched"),
    } });
    Ok(outReal)
}

pub fn valueBool(mut inValue: Arc<Values::Value>) -> Result<bool> {
    let mut outBool: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(inValue.clone()) {
        Deref @ Values::Value::BOOL { boolean: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outBool = __pa0.clone();
    Ok(outBool)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn valueReals(mut inValue: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<metamodelica::List<metamodelica::Real>>> {
    let mut outReal: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    outReal = 'mc: {
        let __mc_input = inValue.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: r }, tail: rest } => {
                    let mut res: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
                    res = valueReals(rest.clone())?;
                    Ok(cons(r.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: rest } => {
                    let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut res: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
                    r = intReal(i.clone());
                    res = valueReals(rest.clone())?;
                    Ok(cons(r.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    let mut res: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
                    res = valueReals(rest.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outReal)
}

pub fn valueString(mut value: Arc<Values::Value>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(value.clone()) {
        Deref @ Values::Value::STRING { string: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    r#str = __pa0.clone();
    Ok(r#str)
}

pub fn arrayValueInts(mut inValue: Arc<Values::Value>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outReal: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(inValue.clone()) {
        Deref @ Values::Value::ARRAY { valueLst: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    vals = __pa0.clone();
    outReal = List::map(vals.clone(), (std::sync::Arc::new(valueInteger) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<i32> + 'static>));
    Ok(outReal)
}

pub fn arrayValueReals(mut inValue: Arc<Values::Value>) -> Result<Arc<metamodelica::List<metamodelica::Real>>> {
    let mut outReal: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(inValue.clone()) {
        Deref @ Values::Value::ARRAY { valueLst: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    vals = __pa0.clone();
    outReal = valueReals(vals.clone())?;
    Ok(outReal)
}

pub fn matrixValueReals(mut inValue: Arc<Values::Value>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>> {
    let mut outReals: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    outReals = 'mc: {
        let __mc_input = inValue.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::ARRAY { valueLst: vals, .. } => {
                    Ok(List::map(vals.clone(), (std::sync::Arc::new(arrayValueReals) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<Arc<metamodelica::List<metamodelica::Real>>> + 'static>)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::ARRAY { valueLst: vals, .. } => {
                    let mut reals: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
                    reals = valueReals(vals.clone())?;
                    Ok(List::map(reals.clone(), std::sync::Arc::new(fnptr!(List::create, _))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outReals)
}

pub fn arrayValueStrings(mut value: Arc<Values::Value>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut strings: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    strings = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut v in (arrayValues(value.clone())?).into_iter().cloned() {
            let __x = valueString(v.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(strings)
}

pub fn valueNeg(mut inValue: Arc<Values::Value>) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outValue = (::match_deref::match_deref! { match &(inValue.clone()) {
        Deref @ Values::Value::REAL { real: r } => {
            let mut r_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            r_1 = -(r.clone());
            Arc::new(Values::Value::REAL { real: r_1.clone() })
        },
        Deref @ Values::Value::INTEGER { integer: i } => {
            let mut i_1: i32 = 0;
            i_1 = -(i.clone());
            Arc::new(Values::Value::INTEGER { integer: i_1.clone() })
        },
        Deref @ Values::Value::ARRAY { dimLst: dims, valueLst: vlst } => {
            let mut vlst_1: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            vlst_1 = List::map(vlst.clone(), (std::sync::Arc::new(valueNeg) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<Arc<Values::Value>> + 'static>));
            Arc::new(Values::Value::ARRAY { valueLst: vlst_1.clone(), dimLst: dims.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outValue)
}

pub fn valueSum(mut value1: Arc<Values::Value>, mut value2: Arc<Values::Value>) -> Result<Arc<Values::Value>> {
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    result = (::match_deref::match_deref! { match &((value1.clone(), value2.clone())) {
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::INTEGER { .. }) => Arc::new(Values::Value::INTEGER { integer: var_field!((*value1).integer, Values::Value::INTEGER).clone() + var_field!((*value2).integer, Values::Value::INTEGER).clone() }),
        (Deref @ Values::Value::STRING { .. }, Deref @ Values::Value::STRING { .. }) => Arc::new(Values::Value::STRING { string: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*var_field!((*value1).string, Values::Value::STRING).clone()); __mm_s.push_str(&*var_field!((*value2).string, Values::Value::STRING).clone()); ArcStr::from(__mm_s) }).clone() }),
        _ => Arc::new(Values::Value::REAL { real: valueReal(value1.clone())? + valueReal(value2.clone())? }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub fn valueSubtract(mut value1: Arc<Values::Value>, mut value2: Arc<Values::Value>) -> Result<Arc<Values::Value>> {
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    result = (::match_deref::match_deref! { match &((value1.clone(), value2.clone())) {
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::INTEGER { .. }) => Arc::new(Values::Value::INTEGER { integer: var_field!((*value1).integer, Values::Value::INTEGER).clone() - var_field!((*value2).integer, Values::Value::INTEGER).clone() }),
        _ => Arc::new(Values::Value::REAL { real: valueReal(value1.clone())? - valueReal(value2.clone())? }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub fn valueMultiply(mut value1: Arc<Values::Value>, mut value2: Arc<Values::Value>) -> Result<Arc<Values::Value>> {
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    result = (::match_deref::match_deref! { match &((value1.clone(), value2.clone())) {
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::INTEGER { .. }) => Arc::new(Values::Value::INTEGER { integer: var_field!((*value1).integer, Values::Value::INTEGER).clone() * var_field!((*value2).integer, Values::Value::INTEGER).clone() }),
        _ => Arc::new(Values::Value::REAL { real: valueReal(value1.clone())? * valueReal(value2.clone())? }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub fn valueDivide(mut value1: Arc<Values::Value>, mut value2: Arc<Values::Value>) -> Result<Arc<Values::Value>> {
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    result = (::match_deref::match_deref! { match &((value1.clone(), value2.clone())) {
        (_, Deref @ Values::Value::INTEGER { integer: 0 }) => {
            Error::addMessage(Error::DIVISION_BY_ZERO.clone(), list![(literal!("0")).clone(), (intString(var_field!((*value2).integer, Values::Value::INTEGER).clone())).clone()])?;
            bail!("fail")
        },
        (_, Deref @ Values::Value::REAL { real: __rlit_0 }) if __rlit_0.eq(&metamodelica::OrderedFloat((0.0) as f64)) => {
            Error::addMessage(Error::DIVISION_BY_ZERO.clone(), list![(literal!("0")).clone(), (realString(var_field!((*value2).real, Values::Value::REAL).clone())).clone()])?;
            bail!("fail")
        },
        _ => Arc::new(Values::Value::REAL { real: valueReal(value1.clone())? / valueReal(value2.clone())? }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub fn valuePow(mut value1: Arc<Values::Value>, mut value2: Arc<Values::Value>) -> Result<Arc<Values::Value>> {
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    result = Arc::new(Values::Value::REAL { real: (valueReal(value1.clone())?).powf(valueReal(value2.clone())?) });
    Ok(result)
}

pub fn sumArray(mut value: Arc<Values::Value>) -> Result<Arc<Values::Value>> {
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    result = (::match_deref::match_deref! { match &(value.clone()) {
        Deref @ Values::Value::ARRAY { .. } => sumArrayelt(var_field!((*value).valueLst, Values::Value::ARRAY).clone())?,
        _ => value.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub fn sumArrayelt(mut values: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<Values::Value>> {
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    result = sumArray(listHead(values.clone())?)?;
    for mut v in &*listRest(values.clone())? {
        let mut v = v.clone();
        result = valueSum(sumArray(v.clone())?, result.clone())?;
    }
    Ok(result)
}

pub fn multScalarArrayelt(mut scalarValue: Arc<Values::Value>, mut arrayValues: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<metamodelica::List<Arc<Values::Value>>>> {
    let mut result: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    result = {
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut v in (arrayValues.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(v.clone()) {
        Deref @ Values::Value::ARRAY { .. } => Arc::new(Values::Value::ARRAY { valueLst: multScalarArrayelt(scalarValue.clone(), var_field!((*v).valueLst, Values::Value::ARRAY).clone())?, dimLst: var_field!((*v).dimLst, Values::Value::ARRAY).clone() }),
        _ => valueMultiply(scalarValue.clone(), v.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(result)
}

pub fn addScalarArrayelt(mut scalarValue: Arc<Values::Value>, mut arrayValues: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<metamodelica::List<Arc<Values::Value>>>> {
    let mut result: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    result = {
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut v in (arrayValues.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(v.clone()) {
        Deref @ Values::Value::ARRAY { .. } => Arc::new(Values::Value::ARRAY { valueLst: addScalarArrayelt(scalarValue.clone(), var_field!((*v).valueLst, Values::Value::ARRAY).clone())?, dimLst: var_field!((*v).dimLst, Values::Value::ARRAY).clone() }),
        _ => valueSum(scalarValue.clone(), v.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(result)
}

pub fn subScalarArrayelt(mut scalarValue: Arc<Values::Value>, mut arrayValues: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<metamodelica::List<Arc<Values::Value>>>> {
    let mut result: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    result = {
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut v in (arrayValues.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(v.clone()) {
        Deref @ Values::Value::ARRAY { .. } => Arc::new(Values::Value::ARRAY { valueLst: subScalarArrayelt(scalarValue.clone(), var_field!((*v).valueLst, Values::Value::ARRAY).clone())?, dimLst: var_field!((*v).dimLst, Values::Value::ARRAY).clone() }),
        _ => valueSubtract(scalarValue.clone(), v.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(result)
}

pub fn subArrayeltScalar(mut scalarValue: Arc<Values::Value>, mut arrayValues: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<metamodelica::List<Arc<Values::Value>>>> {
    let mut result: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    result = {
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut v in (arrayValues.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(v.clone()) {
        Deref @ Values::Value::ARRAY { .. } => Arc::new(Values::Value::ARRAY { valueLst: subArrayeltScalar(scalarValue.clone(), var_field!((*v).valueLst, Values::Value::ARRAY).clone())?, dimLst: var_field!((*v).dimLst, Values::Value::ARRAY).clone() }),
        _ => valueSubtract(v.clone(), scalarValue.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(result)
}

pub fn divScalarArrayelt(mut scalarValue: Arc<Values::Value>, mut arrayValues: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<metamodelica::List<Arc<Values::Value>>>> {
    let mut result: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    result = {
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut v in (arrayValues.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(v.clone()) {
        Deref @ Values::Value::ARRAY { .. } => Arc::new(Values::Value::ARRAY { valueLst: divScalarArrayelt(scalarValue.clone(), var_field!((*v).valueLst, Values::Value::ARRAY).clone())?, dimLst: var_field!((*v).dimLst, Values::Value::ARRAY).clone() }),
        _ => valueDivide(scalarValue.clone(), v.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(result)
}

pub fn divArrayeltScalar(mut scalarValue: Arc<Values::Value>, mut arrayValues: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<metamodelica::List<Arc<Values::Value>>>> {
    let mut result: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    result = {
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut v in (arrayValues.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(v.clone()) {
        Deref @ Values::Value::ARRAY { .. } => Arc::new(Values::Value::ARRAY { valueLst: divArrayeltScalar(scalarValue.clone(), var_field!((*v).valueLst, Values::Value::ARRAY).clone())?, dimLst: var_field!((*v).dimLst, Values::Value::ARRAY).clone() }),
        _ => valueDivide(v.clone(), scalarValue.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(result)
}

pub fn powScalarArrayelt(mut scalarValue: Arc<Values::Value>, mut arrayValues: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<metamodelica::List<Arc<Values::Value>>>> {
    let mut result: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    result = {
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut v in (arrayValues.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(v.clone()) {
        Deref @ Values::Value::ARRAY { .. } => Arc::new(Values::Value::ARRAY { valueLst: powScalarArrayelt(scalarValue.clone(), var_field!((*v).valueLst, Values::Value::ARRAY).clone())?, dimLst: var_field!((*v).dimLst, Values::Value::ARRAY).clone() }),
        _ => valuePow(scalarValue.clone(), v.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(result)
}

pub fn powArrayeltScalar(mut scalarValue: Arc<Values::Value>, mut arrayValues: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<metamodelica::List<Arc<Values::Value>>>> {
    let mut result: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    result = {
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut v in (arrayValues.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(v.clone()) {
        Deref @ Values::Value::ARRAY { .. } => Arc::new(Values::Value::ARRAY { valueLst: powArrayeltScalar(scalarValue.clone(), var_field!((*v).valueLst, Values::Value::ARRAY).clone())?, dimLst: var_field!((*v).dimLst, Values::Value::ARRAY).clone() }),
        _ => valuePow(scalarValue.clone(), v.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(result)
}

pub fn multScalarProduct(mut inValueLst1: Arc<metamodelica::List<Arc<Values::Value>>>, mut inValueLst2: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outValue = 'mc: {
        let __mc_input = (inValueLst1.clone(), inValueLst2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: i1 }, tail: v1lst @ Deref @ metamodelica::List::Cons { head: _, tail: _ } }, Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: i2 }, tail: v2lst @ Deref @ metamodelica::List::Cons { head: _, tail: _ } }) => {
                    let mut res: i32 = 0;
                    let mut i1 = (*i1).clone();
                    let mut i2 = (*i2).clone();
                    i1 = i1.clone() * i2.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(multScalarProduct(v1lst.clone(), v2lst.clone())?) {
                        Deref @ Values::Value::INTEGER { integer: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    i2 = __pa0.clone();
                    res = i1.clone() + i2.clone();
                    Ok(Arc::new(Values::Value::INTEGER { integer: res.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: v1 }, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: v2 }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut res: i32 = 0;
                    res = v1.clone() * v2.clone();
                    Ok(Arc::new(Values::Value::INTEGER { integer: res.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: r1 }, tail: v1lst @ Deref @ metamodelica::List::Cons { head: _, tail: _ } }, Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: r2 }, tail: v2lst @ Deref @ metamodelica::List::Cons { head: _, tail: _ } }) => {
                    let mut rres: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut r1 = (*r1).clone();
                    let mut r2 = (*r2).clone();
                    r1 = r1.clone() * r2.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(multScalarProduct(v1lst.clone(), v2lst.clone())?) {
                        Deref @ Values::Value::REAL { real: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    r2 = __pa0.clone();
                    rres = r1.clone() + r2.clone();
                    Ok(Arc::new(Values::Value::REAL { real: rres.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: r1 }, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: r2 }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut rres: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    rres = r1.clone() * r2.clone();
                    Ok(Arc::new(Values::Value::REAL { real: rres.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: v2lst, .. }, tail: rest }, vlst @ Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { .. }, tail: _ }) => {
                    let mut dim: i32 = 0;
                    let mut vres: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut sres: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    sres = multScalarProduct(v2lst.clone(), vlst.clone())?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(multScalarProduct(rest.clone(), vlst.clone())?) {
                        Deref @ Values::Value::ARRAY { valueLst: __pa0, dimLst: Deref @ metamodelica::List::Cons { head: __pa1, tail: __pa2 } } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    vres = __pa0.clone();
                    dim = __pa1.clone();
                    dims = __pa2.clone();
                    dim = dim.clone() + 1;
                    Ok(Arc::new(Values::Value::ARRAY { valueLst: cons(sres.clone(), vres.clone()), dimLst: cons(dim.clone(), dims.clone()) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { .. }, tail: _ }) => {
                    Ok(ValuesMake::makeArray(metamodelica::nil())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: v2lst, .. }, tail: rest }, vlst @ Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { .. }, tail: _ }) => {
                    let mut dim: i32 = 0;
                    let mut vres: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut sres: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    sres = multScalarProduct(v2lst.clone(), vlst.clone())?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(multScalarProduct(rest.clone(), vlst.clone())?) {
                        Deref @ Values::Value::ARRAY { valueLst: __pa0, dimLst: Deref @ metamodelica::List::Cons { head: __pa1, tail: __pa2 } } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    vres = __pa0.clone();
                    dim = __pa1.clone();
                    dims = __pa2.clone();
                    dim = dim.clone() + 1;
                    Ok(Arc::new(Values::Value::ARRAY { valueLst: cons(sres.clone(), vres.clone()), dimLst: cons(dim.clone(), dims.clone()) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { .. }, tail: _ }) => {
                    Ok(ValuesMake::makeArray(metamodelica::nil())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vlst @ Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { .. }, tail: _ }, mat @ Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } }, .. }, tail: _ }) => {
                    let mut dim: i32 = 0;
                    let mut col: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut mat_1: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(matrixStripFirstColumn(mat.clone())?) {
                        (Deref @ Values::Value::ARRAY { valueLst: __pa0, .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    col = __pa0.clone();
                    mat_1 = __pa1.clone();
                    v = multScalarProduct(vlst.clone(), col.clone())?;
                    let (__pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(multScalarProduct(vlst.clone(), mat_1.clone())?) {
                        Deref @ Values::Value::ARRAY { valueLst: __pa2, dimLst: Deref @ metamodelica::List::Cons { head: __pa3, tail: __pa4 } } => (__pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    vals = __pa2.clone();
                    dim = __pa3.clone();
                    dims = __pa4.clone();
                    Ok(Arc::new(Values::Value::ARRAY { valueLst: cons(v.clone(), vals.clone()), dimLst: cons(dim.clone(), dims.clone()) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vlst @ Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { .. }, tail: _ }, mat @ Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, .. }, tail: _ }) => {
                    let mut i1: i32 = 0;
                    let mut col: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(matrixStripFirstColumn(mat.clone())?) {
                        (Deref @ Values::Value::ARRAY { valueLst: __pa0, .. }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    col = __pa0.clone();
                    let __pa1 = ::match_deref::match_deref! { match &(multScalarProduct(vlst.clone(), col.clone())?) {
                        Deref @ Values::Value::INTEGER { integer: __pa1 } => __pa1.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    i1 = __pa1.clone();
                    Ok(ValuesMake::makeArray(list![Arc::new(Values::Value::INTEGER { integer: i1.clone() })])?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vlst @ Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { .. }, tail: _ }, mat @ Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } }, .. }, tail: _ }) => {
                    let mut dim: i32 = 0;
                    let mut col: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut mat_1: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(matrixStripFirstColumn(mat.clone())?) {
                        (Deref @ Values::Value::ARRAY { valueLst: __pa0, .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    col = __pa0.clone();
                    mat_1 = __pa1.clone();
                    v = multScalarProduct(vlst.clone(), col.clone())?;
                    let (__pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(multScalarProduct(vlst.clone(), mat_1.clone())?) {
                        Deref @ Values::Value::ARRAY { dimLst: Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 }, valueLst: __pa4 } => (__pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    dim = __pa2.clone();
                    dims = __pa3.clone();
                    vals = __pa4.clone();
                    dim = dim.clone() + 1;
                    Ok(Arc::new(Values::Value::ARRAY { valueLst: cons(v.clone(), vals.clone()), dimLst: cons(dim.clone(), dims.clone()) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vlst @ Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { .. }, tail: _ }, mat @ Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, .. }, tail: _ }) => {
                    let mut col: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut r1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let __pa0 = ::match_deref::match_deref! { match &(matrixStripFirstColumn(mat.clone())?) {
                        (Deref @ Values::Value::ARRAY { valueLst: __pa0, .. }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    col = __pa0.clone();
                    let __pa1 = ::match_deref::match_deref! { match &(multScalarProduct(vlst.clone(), col.clone())?) {
                        Deref @ Values::Value::REAL { real: __pa1 } => __pa1.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    r1 = __pa1.clone();
                    Ok(ValuesMake::makeArray(list![Arc::new(Values::Value::REAL { real: r1.clone() })])?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("Values.multScalarProduct failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outValue)
}

pub fn crossProduct(mut inValueLst1: Arc<metamodelica::List<Arc<Values::Value>>>, mut inValueLst2: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outValue = (::match_deref::match_deref! { match &((inValueLst1.clone(), inValueLst2.clone())) {
        (Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: x1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: x2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: x3 }, tail: Deref @ metamodelica::List::Nil } } }, Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: y1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: y2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: y3 }, tail: Deref @ metamodelica::List::Nil } } }) => {
            let mut z1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut z2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut z3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            z1 = ((x2.clone()) * (y3.clone())) - ((x3.clone()) * (y2.clone()));
            z2 = ((x3.clone()) * (y1.clone())) - ((x1.clone()) * (y3.clone()));
            z3 = ((x1.clone()) * (y2.clone())) - ((x2.clone()) * (y1.clone()));
            ValuesMake::makeArray(list![Arc::new(Values::Value::REAL { real: z1.clone() }), Arc::new(Values::Value::REAL { real: z2.clone() }), Arc::new(Values::Value::REAL { real: z3.clone() })])?
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: ix1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: ix2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: ix3 }, tail: Deref @ metamodelica::List::Nil } } }, Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: iy1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: iy2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: iy3 }, tail: Deref @ metamodelica::List::Nil } } }) => {
            let mut iz1: i32 = 0;
            let mut iz2: i32 = 0;
            let mut iz3: i32 = 0;
            iz1 = intSub(intMul(ix2.clone(), iy3.clone()), intMul(ix3.clone(), iy2.clone()));
            iz2 = intSub(intMul(ix3.clone(), iy1.clone()), intMul(ix1.clone(), iy3.clone()));
            iz3 = intSub(intMul(ix1.clone(), iy2.clone()), intMul(ix2.clone(), iy1.clone()));
            ValuesMake::makeArray(list![Arc::new(Values::Value::INTEGER { integer: iz1.clone() }), Arc::new(Values::Value::INTEGER { integer: iz2.clone() }), Arc::new(Values::Value::INTEGER { integer: iz3.clone() })])?
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("ValuesUtil.crossProduct failed")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outValue)
}

pub fn multMatrix(mut inValueLst1: Arc<metamodelica::List<Arc<Values::Value>>>, mut inValueLst2: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<metamodelica::List<Arc<Values::Value>>>> {
    let mut outValueLst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    outValueLst = (::match_deref::match_deref! { match &((inValueLst1.clone(), inValueLst2.clone())) {
        (Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: v1lst, .. }, tail: rest1 }, m2 @ Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { .. }, tail: _ }) => {
            let mut res1: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut res2: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            res1 = multScalarProduct(v1lst.clone(), m2.clone())?;
            res2 = multMatrix(rest1.clone(), m2.clone())?;
            cons(res1.clone(), res2.clone())
        },
        (Deref @ metamodelica::List::Nil, _) => {
            metamodelica::nil()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outValueLst)
}

fn matrixStripFirstColumn(mut inValueLst: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<(Arc<Values::Value>, Arc<metamodelica::List<Arc<Values::Value>>>)> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut outValueLst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    (outValue, outValueLst) = (::match_deref::match_deref! { match &(inValueLst.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { dimLst: Deref @ metamodelica::List::Cons { head: dim, tail: Deref @ metamodelica::List::Nil }, valueLst: Deref @ metamodelica::List::Cons { head: v1, tail: vrest } }, tail: rest } => {
            let mut resl: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut resl2: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut i: i32 = 0;
            let mut dim = (*dim).clone();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(matrixStripFirstColumn(rest.clone())?) {
                (Deref @ Values::Value::ARRAY { valueLst: __pa0, dimLst: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            resl = __pa0.clone();
            i = __pa1.clone();
            resl2 = __pa2.clone();
            i = i.clone() + 1;
            dim = dim.clone() - 1;
            (Arc::new(Values::Value::ARRAY { valueLst: cons(v1.clone(), resl.clone()), dimLst: list![i.clone()] }), cons(Arc::new(Values::Value::ARRAY { valueLst: vrest.clone(), dimLst: list![dim.clone()] }), resl2.clone()))
        },
        Deref @ metamodelica::List::Nil => {
            (Arc::new(Values::Value::ARRAY { valueLst: metamodelica::nil(), dimLst: list![0] }), metamodelica::nil())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outValue, outValueLst))
}

pub fn intlistToValue(mut inIntegerLst: Arc<metamodelica::List<i32>>) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outValue = (::match_deref::match_deref! { match &(inIntegerLst.clone()) {
        Deref @ metamodelica::List::Nil => {
            Arc::new(Values::Value::ARRAY { valueLst: metamodelica::nil(), dimLst: list![0] })
        },
        Deref @ metamodelica::List::Cons { head: i, tail: lst } => {
            let mut res: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut len: i32 = 0;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(intlistToValue(lst.clone())?) {
                Deref @ Values::Value::ARRAY { valueLst: __pa0, dimLst: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            res = __pa0.clone();
            len = __pa1.clone();
            len = len.clone() + 1;
            Arc::new(Values::Value::ARRAY { valueLst: cons(Arc::new(Values::Value::INTEGER { integer: i.clone() }), res.clone()), dimLst: list![len.clone()] })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outValue)
}

pub fn arrayValues(mut inValue: Arc<Values::Value>) -> Result<Arc<metamodelica::List<Arc<Values::Value>>>> {
    let mut outValueLst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    outValueLst = (::match_deref::match_deref! { match &(inValue.clone()) {
        Deref @ Values::Value::ARRAY { valueLst: v_lst, .. } => {
            v_lst.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outValueLst)
}

pub fn arrayScalar(mut inValue: Arc<Values::Value>) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let __pa0 = ::match_deref::match_deref! { match &(inValue.clone()) {
        Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outValue = __pa0.clone();
    Ok(outValue)
}

pub fn writePtolemyplotDataset(mut inString1: ArcStr, mut inValue2: Arc<Values::Value>, mut inStringLst3: Arc<metamodelica::List<ArcStr>>, mut inString4: ArcStr) -> Result<i32> {
    let mut outInteger: i32 = 0;
    outInteger = (::match_deref::match_deref! { match &((inString1.clone(), inValue2.clone(), inStringLst3.clone(), inString4.clone())) {
        (filename, Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::List::Cons { head: t, tail: rest }, .. }, Deref @ metamodelica::List::Cons { head: _, tail: varnames }, message) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut handle: i32 = 0;
            handle = Print::saveAndClearBuf()?;
            Print::printBuf((literal!("#Ptolemy Plot generated by OpenModelica\nTitleText: ")).clone())?;
            Print::printBuf((message.clone()).clone())?;
            Print::printBuf((literal!("\n")).clone())?;
            unparsePtolemyValues(t.clone(), rest.clone(), varnames.clone())?;
            r#str = (Print::getString()?).clone();
            Print::restoreBuf(handle.clone())?;
            System::writeFile((filename.clone()).clone(), (r#str.clone()).clone())?;
            0
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outInteger)
}

fn unparsePtolemyValues(mut inValue: Arc<Values::Value>, mut inValueLst: Arc<metamodelica::List<Arc<Values::Value>>>, mut inStringLst: Arc<metamodelica::List<ArcStr>>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inValue.clone(), inValueLst.clone(), inStringLst.clone())) {
        (_, Deref @ metamodelica::List::Nil, _) => {
            ()
        },
        (t, Deref @ metamodelica::List::Cons { head: s1, tail: xs }, Deref @ metamodelica::List::Cons { head: v1, tail: vs }) => {
            unparsePtolemySet(t.clone(), s1.clone(), (v1.clone()).clone())?;
            unparsePtolemyValues(t.clone(), xs.clone(), vs.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn unparsePtolemySet(mut v1: Arc<Values::Value>, mut v2: Arc<Values::Value>, mut varname: ArcStr) -> Result<()> {
    Print::printBuf(stringAppendList(list![(literal!("DataSet: ")).clone(), (varname.clone()).clone(), (literal!("\n")).clone()]))?;
    unparsePtolemySet2(v1.clone(), v2.clone())?;
    Ok(())
}

fn unparsePtolemySet2(mut inValue1: Arc<Values::Value>, mut inValue2: Arc<Values::Value>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inValue1.clone(), inValue2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::List::Nil, .. }, Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::List::Nil, .. }) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::List::Cons { head: v1, tail: v1s }, .. }, Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::List::Cons { head: v2, tail: v2s }, .. }) => {
                    ValuesDump::valString2(v1.clone())?;
                    Print::printBuf((literal!(",")).clone())?;
                    ValuesDump::valString2(v2.clone())?;
                    Print::printBuf((literal!("\n")).clone())?;
                    unparsePtolemySet2(Arc::new(Values::Value::ARRAY { valueLst: v1s.clone(), dimLst: metamodelica::nil() }), Arc::new(Values::Value::ARRAY { valueLst: v2s.clone(), dimLst: metamodelica::nil() }))?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v1, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ValuesUtil.unparsePtolemySet2 failed on v1: ")); __mm_s.push_str(&*ValuesDump::printValStr(v1.clone())?); __mm_s.push_str(&*literal!(" and v2: ")); __mm_s.push_str(&*ValuesDump::printValStr(v1.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn reverseMatrix(mut inValue: Arc<Values::Value>) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outValue = 'mc: {
        let __mc_input = inValue.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::ARRAY { dimLst: dims, valueLst: lst } => {
                    let mut lst_1: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut lst_2: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    lst_1 = List::map(lst.clone(), (std::sync::Arc::new(reverseMatrix) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<Arc<Values::Value>> + 'static>));
                    lst_2 = lst_1.clone().reverse();
                    Ok(Arc::new(Values::Value::ARRAY { valueLst: lst_2.clone(), dimLst: dims.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                value => {
                    Ok(value.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outValue)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn nthnthArrayelt(mut inLst: Arc<metamodelica::List<Arc<Values::Value>>>, mut inValue: Arc<Values::Value>, mut lastValue: Arc<Values::Value>) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outValue = (::match_deref::match_deref! { match &((inLst.clone(), inValue.clone(), lastValue.clone())) {
        (Deref @ metamodelica::List::Nil, _, preRes) => {
            preRes.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: vlst2 }, Deref @ Values::Value::ARRAY { valueLst: vlst, .. }, _) => {
            let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            res = (vlst.clone()).get(n.clone())?;
            res = nthnthArrayelt(vlst2.clone(), res.clone(), res.clone())?;
            res.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ENUM_LITERAL { index: n, .. }, tail: vlst2 }, Deref @ Values::Value::ARRAY { valueLst: vlst, .. }, _) => {
            let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            res = (vlst.clone()).get(n.clone())?;
            res = nthnthArrayelt(vlst2.clone(), res.clone(), res.clone())?;
            res.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: vlst2 }, Deref @ Values::Value::ARRAY { valueLst: vlst, .. }, _) => {
            let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            res = (vlst.clone()).get(if (b.clone()) {2} else {1})?;
            res = nthnthArrayelt(vlst2.clone(), res.clone(), res.clone())?;
            res.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outValue)
}

pub fn valueInteger(mut inValue: Arc<Values::Value>) -> Result<i32> {
    let mut outInteger: i32 = 0;
    outInteger = (::match_deref::match_deref! { match &(inValue.clone()) {
        Deref @ Values::Value::INTEGER { integer: i } => {
            i.clone()
        },
        Deref @ Values::Value::ENUM_LITERAL { index: i, .. } => {
            i.clone()
        },
        Deref @ Values::Value::BOOL { boolean: true } => {
            1
        },
        Deref @ Values::Value::BOOL { boolean: false } => {
            0
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outInteger)
}

pub fn valueDimensions(mut inValue: Arc<Values::Value>) -> Arc<metamodelica::List<i32>> {
    let mut outDimensions: Arc<metamodelica::List<i32>> = metamodelica::nil();
    outDimensions = (::match_deref::match_deref! { match &(inValue.clone()) {
        Deref @ Values::Value::ARRAY { dimLst: dims, .. } => {
            dims.clone()
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outDimensions
}

pub fn extractValueString(mut val: Arc<Values::Value>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(val.clone()) {
        Deref @ Values::Value::STRING { string: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    r#str = __pa0.clone();
    Ok(r#str)
}

pub fn getCode(mut val: Arc<Values::Value>) -> Result<Arc<Absyn::CodeNode>> {
    let mut code: Arc<Absyn::CodeNode>;
    let __pa0 = ::match_deref::match_deref! { match &(val.clone()) {
        Deref @ Values::Value::CODE { A: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    code = __pa0.clone();
    Ok(code)
}

pub fn getPath(mut val: Arc<Values::Value>) -> Result<Arc<Absyn::Path>> {
    let mut path: Arc<Absyn::Path>;
    let mut code: Arc<Absyn::CodeNode>;
    let __pa0 = ::match_deref::match_deref! { match &(val.clone()) {
        Deref @ Values::Value::CODE { A: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    code = __pa0.clone();
    let __pa1 = ::match_deref::match_deref! { match &(code.clone()) {
        Deref @ Absyn::CodeNode::C_TYPENAME { path: __pa1 } => __pa1.clone(),
        _ => bail!("pattern mismatch"),
    } };
    path = __pa1.clone();
    Ok(path)
}

pub fn printCodeVariableName(mut val: Arc<Values::Value>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(val.clone()) {
        Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp } } => {
            Dump::printExpStr(exp.clone())?
        },
        Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_VARIABLENAME { componentRef: cr } } => {
            Dump::printComponentRefStr(cr.clone())?
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(r#str)
}

pub fn boxIfUnboxedVal(mut v: Arc<Values::Value>) -> Arc<Values::Value> {
    let mut ov: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    ov = (::match_deref::match_deref! { match &(v.clone()) {
        Deref @ Values::Value::INTEGER { integer: _ } => Arc::new(Values::Value::META_BOX { value: v.clone() }),
        Deref @ Values::Value::REAL { real: _ } => Arc::new(Values::Value::META_BOX { value: v.clone() }),
        Deref @ Values::Value::BOOL { boolean: _ } => Arc::new(Values::Value::META_BOX { value: v.clone() }),
        _ => v.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ov
}

pub fn unboxIfBoxedVal(mut iv: Arc<Values::Value>) -> Arc<Values::Value> {
    let mut ov: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    ov = (::match_deref::match_deref! { match &(iv.clone()) {
        Deref @ Values::Value::META_BOX { value: v } => {
            v.clone()
        },
        _ => {
            iv.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ov
}

pub fn arrayOrListVals(mut v: Arc<Values::Value>, mut boxIfUnboxed: bool) -> Result<Arc<metamodelica::List<Arc<Values::Value>>>> {
    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    vals = (::match_deref::match_deref! { match &((v.clone(), boxIfUnboxed.clone())) {
        (Deref @ Values::Value::ARRAY { valueLst: vals, .. }, _) => vals.clone(),
        (Deref @ Values::Value::LIST { valueLst: vals }, true) => List::map(vals.clone(), (std::sync::Arc::new(fnptr!(boxIfUnboxedVal, Arc<Values::Value>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<Arc<Values::Value>> + 'static>)),
        (Deref @ Values::Value::LIST { valueLst: vals }, _) => vals.clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(vals)
}

pub fn containsEmpty(mut inValue: Arc<Values::Value>) -> Option<Arc<Values::Value>> {
    let mut outEmptyVal: Option<Arc<Values::Value>> = None;
    outEmptyVal = (::match_deref::match_deref! { match &(inValue.clone()) {
        Deref @ Values::Value::EMPTY { .. } => Some(inValue.clone()),
        Deref @ Values::Value::ARRAY { .. } => arrayContainsEmpty(var_field!((*inValue).valueLst, Values::Value::ARRAY).clone()),
        Deref @ Values::Value::RECORD { .. } => arrayContainsEmpty(var_field!((*inValue).orderd, Values::Value::RECORD).clone()),
        Deref @ Values::Value::TUPLE { .. } => arrayContainsEmpty(var_field!((*inValue).valueLst, Values::Value::TUPLE).clone()),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outEmptyVal
}

pub fn arrayContainsEmpty(mut inValues: Arc<metamodelica::List<Arc<Values::Value>>>) -> Option<Arc<Values::Value>> {
    let mut outOptValue: Option<Arc<Values::Value>> = None;
    for mut val in &*inValues.clone() {
        let mut val = val.clone();
        outOptValue = containsEmpty(val.clone());
        if isSome(outOptValue.clone()) {
            break;
        }
    }
    outOptValue
}

pub fn liftValueList(mut inValue: Arc<Values::Value>, mut inDimensions: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value> = inValue.clone();
    for mut dim in &*inDimensions.clone().reverse() {
        let mut dim = dim.clone();
        outValue = ValuesMake::makeArray(List::fill(outValue.clone(), Expression::dimensionSize(dim.clone())?))?;
    }
    Ok(outValue)
}

pub fn isEmpty(mut inValue: Arc<Values::Value>) -> bool {
    let mut outIsEmpty: bool = false;
    outIsEmpty = (::match_deref::match_deref! { match &(inValue.clone()) {
        Deref @ Values::Value::EMPTY { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsEmpty
}

pub fn typeConvertRecord(mut inValue: Arc<Values::Value>, mut inType: Arc<DAE::Type>) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value> = inValue.clone();
    outValue = (::match_deref::match_deref! { match &((outValue.clone(), inType.clone())) {
        (Deref @ Values::Value::RECORD { .. }, Deref @ DAE::Type::T_COMPLEX { .. }) => {
            assign_variant_field!(outValue => Values::Value::RECORD; orderd = {
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for (val, var) in (&(var_field!((*outValue).orderd, Values::Value::RECORD).clone())).into_iter().zip((&(var_field!((*inType).varLst, DAE::Type::T_COMPLEX).clone())).into_iter()) {
            let __x = typeConvertRecord(val.clone(), Types::getVarType(var.clone())?)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            outValue.clone()
        },
        (Deref @ Values::Value::INTEGER { .. }, Deref @ DAE::Type::T_REAL { .. }) => {
            Arc::new(Values::Value::REAL { real: intReal(var_field!((*outValue).integer, Values::Value::INTEGER).clone()) })
        },
        (Deref @ Values::Value::ARRAY { .. }, Deref @ DAE::Type::T_ARRAY { .. }) => {
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            ty = Expression::unliftArray(inType.clone())?;
            assign_variant_field!(outValue => Values::Value::ARRAY; valueLst = {
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut v in (var_field!((*outValue).valueLst, Values::Value::ARRAY).clone()).into_iter().cloned() {
            let __x = typeConvertRecord(v.clone(), ty.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            outValue.clone()
        },
        _ => {
            outValue.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outValue)
}

pub fn fixZeroSizeArray(mut e: Arc<DAE::Exp>, mut ty: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut e: Arc<DAE::Exp> = e;
    e = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Nil, scalar: false, ty: Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_UNKNOWN, .. } } => Arc::new(DAE::Exp::ARRAY { ty: ty.clone(), scalar: !(Types::isArray(Types::unliftArray(ty.clone())?)), array: metamodelica::nil() }),
        _ => e.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(e)
}

pub fn arraySize(mut value: Arc<Values::Value>) -> Result<i32> {
    let mut size: i32 = 0;
    size = (::match_deref::match_deref! { match &(value.clone()) {
        Deref @ Values::Value::ARRAY { .. } => listHead(var_field!((*value).dimLst, Values::Value::ARRAY).clone())?,
        Deref @ Values::Value::META_ARRAY { .. } => (var_field!((*value).valueLst, Values::Value::META_ARRAY).clone().len() as i32),
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(size)
}

