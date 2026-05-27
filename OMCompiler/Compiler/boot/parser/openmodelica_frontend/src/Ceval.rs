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

use crate::BackendCevalInterface;
use crate::ComponentReference;
use crate::Expression;
use crate::ExpressionDump;
use crate::ExpressionSimplify;
use crate::FCore;
use crate::FGraph;
use crate::InstBinding;
use crate::InstUtil;
use crate::Lookup;
use crate::Static;
use crate::Types;
use crate::ValuesUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_dump::ValuesDump;
use openmodelica_frontend_dump::ValuesMake;
use openmodelica_frontend_inst::InstTypes;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::Print;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

// protected imports
pub fn ceval(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<DAE::Exp>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = cevalWork1(inCache.clone(), inEnv.clone(), inExp.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone(), numIter.clone() > Global::recursionDepthLimit.clone())?;
    Ok((outCache, outValue))
}

fn cevalWork1(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<DAE::Exp>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32, mut iterReached: bool) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExp.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone(), iterReached.clone())) {
        (_, _, _, _, _, _, false) => {
            (outCache, outValue) = cevalWork2(inCache.clone(), inEnv.clone(), inExp.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())?;
            (outCache.clone(), outValue.clone())
        },
        (_, _, _, _, Absyn::Msg::MSG { info }, _, true) => {
            let mut str1: ArcStr = arcstr::literal!("");
            let mut str2: ArcStr = arcstr::literal!("");
            str1 = (intString(Global::recursionDepthLimit.clone())).clone();
            str2 = (ExpressionBasics::printExpStr(inExp.clone())?).clone();
            Error::addSourceMessage(Error::RECURSION_DEPTH_WARNING.clone(), list![(str1.clone()).clone(), (str2.clone()).clone(), (FGraph::printGraphPathStr(inEnv.clone())?).clone()], info.clone())?;
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalWork2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<DAE::Exp>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    pub type ReductionOperator = std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>, Arc<Values::Value>) -> Result<Arc<Values::Value>> + 'static>;

    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inExp.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Exp::ICONST { integer: i }, _, _, _) => {
                    Ok((cache.clone(), Arc::new(Values::Value::INTEGER { integer: i.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Exp::RCONST { real: r }, _, _, _) => {
                    Ok((cache.clone(), Arc::new(Values::Value::REAL { real: r.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Exp::SCONST { string: s }, _, _, _) => {
                    Ok((cache.clone(), Arc::new(Values::Value::STRING { string: (s.clone()).clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Exp::BCONST { bool: b }, _, _, _) => {
                    Ok((cache.clone(), Arc::new(Values::Value::BOOL { boolean: b.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Exp::ENUM_LITERAL { index: i, name }, _, _, _) => {
                    Ok((cache.clone(), Arc::new(Values::Value::ENUM_LITERAL { name: name.clone(), index: i.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CODE { code: Deref @ Absyn::CodeNode::C_EXPRESSION { exp }, .. }, r#impl, msg, _) => {
                    let mut exp_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut cache = (*cache).clone();
                    (cache, exp_1) = cevalAstExp(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), Absyn::dummyInfo.clone())?;
                    Ok((cache.clone(), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_EXPRESSION { exp: exp_1.clone() }) })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CODE { code: Deref @ Absyn::CodeNode::C_ELEMENT { element: elt }, .. }, r#impl, msg, _) => {
                    let mut elt_1: Arc<Absyn::Element>;
                    let mut cache = (*cache).clone();
                    (cache, elt_1) = cevalAstElt(cache.clone(), env.clone(), elt.clone(), r#impl.clone(), msg.clone())?;
                    Ok((cache.clone(), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_ELEMENT { element: elt_1.clone() }) })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Exp::CODE { code: c, .. }, _, _, _) => {
                    Ok((cache.clone(), Arc::new(Values::Value::CODE { A: c.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { dims: arrayDims, .. }, array: es, .. }, r#impl, msg, _) => {
                    let mut es_1: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, es_1) = cevalList(cache.clone(), env.clone(), es.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
                    v = 'mc: {
        let __mc_input = ();
        if let Ok(__v) = (|| -> Result<_> {
                    let () = __mc_input.clone() else { bail!("nomatch") };
                    let mut dims: Arc<metamodelica::List<i32>>;
                    let mut v: Arc<Values::Value>;
                    dims = List::map(arrayDims.clone(), (std::sync::Arc::new(Expression::dimensionSize) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Dimension>) -> Result<i32> + 'static>));
                    v = Arc::new(Values::Value::ARRAY { valueLst: es_1.clone(), dimLst: dims.clone() });
                    Ok(v.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
                    let _ = __mc_input.clone() else { bail!("nomatch") };
                    let mut v: Arc<Values::Value>;
                    v = ValuesMake::makeArray(es_1.clone())?;
                    Ok(v.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
                    Ok((cache.clone(), v.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_UNKNOWN, array: es, .. }, r#impl, msg, _) => {
                    if !((Config::getGraphicsExpMode()? && Config::getEvaluateParametersInAnnotations()?)) { bail!("guard") }
                    let mut es_1: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, es_1) = cevalList(cache.clone(), env.clone(), es.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
                    v = 'mc: {
        let __mc_input = ();
        if let Ok(__v) = (|| -> Result<_> {
                    let () = __mc_input.clone() else { bail!("nomatch") };
                    let mut dims: Arc<metamodelica::List<i32>>;
                    let mut v: Arc<Values::Value>;
                    dims = list![1];
                    v = Arc::new(Values::Value::ARRAY { valueLst: es_1.clone(), dimLst: dims.clone() });
                    Ok(v.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
                    let _ = __mc_input.clone() else { bail!("nomatch") };
                    let mut v: Arc<Values::Value>;
                    v = ValuesMake::makeArray(es_1.clone())?;
                    Ok(v.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
                    Ok((cache.clone(), v.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::MATRIX { ty: Deref @ DAE::Type::T_ARRAY { dims: arrayDims, .. }, matrix: expll, .. }, r#impl, msg, _) => {
                    let mut elts: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    dims = List::map(arrayDims.clone(), (std::sync::Arc::new(Expression::dimensionSize) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Dimension>) -> Result<i32> + 'static>));
                    (cache, elts) = cevalMatrixElt(cache.clone(), env.clone(), expll.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
                    Ok((cache.clone(), Arc::new(Values::Value::ARRAY { valueLst: elts.clone(), dimLst: dims.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::LIST { valList: expl }, r#impl, msg, _) => {
                    let mut es_1: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, es_1) = cevalList(cache.clone(), env.clone(), expl.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
                    Ok((cache.clone(), Arc::new(Values::Value::LIST { valueLst: es_1.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BOX { exp: e1 }, r#impl, msg, _) => {
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    (cache, v) = ceval(cache.clone(), env.clone(), e1.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
                    Ok((cache.clone(), v.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::UNBOX { exp: e1, .. }, r#impl, msg, _) => {
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), e1.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa0, Deref @ Values::Value::META_BOX { value: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    v = __pa1.clone();
                    Ok((cache.clone(), v.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CONS { cdr: e2, car: e1 }, r#impl, msg, _) => {
                    let mut vallst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    (cache, v) = ceval(cache.clone(), env.clone(), e1.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), e2.clone(), r#impl.clone(), msg.clone(), numIter.clone())?) {
                        (__pa0, Deref @ Values::Value::LIST { valueLst: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    vallst = __pa1.clone();
                    Ok((cache.clone(), Arc::new(Values::Value::LIST { valueLst: cons(v.clone(), vallst.clone()) })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_VAR { .. }, .. }, _, _, _) => {
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::METARECORDCALL { index, fieldNames, args: expl, path: funcpath, .. }, r#impl, msg, _) => {
                    let mut vallst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, vallst) = cevalList(cache.clone(), env.clone(), expl.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
                    Ok((cache.clone(), Arc::new(Values::Value::RECORD { record_: funcpath.clone(), orderd: vallst.clone(), comp: fieldNames.clone(), index: index.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Exp::META_OPTION { exp: None }, _, _, _) => {
                    Ok((cache.clone(), Arc::new(Values::Value::OPTION { some: None })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::META_OPTION { exp: Some(expExp) }, r#impl, msg, _) => {
                    let mut value: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    (cache, value) = ceval(cache.clone(), env.clone(), expExp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
                    Ok((cache.clone(), Arc::new(Values::Value::OPTION { some: Some(value.clone()) })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::META_TUPLE { listExp: expl }, r#impl, msg, _) => {
                    let mut vallst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { bail!("pattern mismatch") };
                    (cache, vallst) = cevalList(cache.clone(), env.clone(), expl.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
                    Ok((cache.clone(), Arc::new(Values::Value::META_TUPLE { valueLst: vallst.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::TUPLE { PR: expl }, r#impl, msg, _) => {
                    let mut vallst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, vallst) = cevalList(cache.clone(), env.clone(), expl.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
                    Ok((cache.clone(), Arc::new(Values::Value::TUPLE { valueLst: vallst.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CREF { componentRef: cr, .. }, false, msg, _) => {
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    (cache, v) = cevalCref(cache.clone(), env.clone(), cr.clone(), false, msg.clone(), numIter.clone() + 1)?;
                    Ok((cache.clone(), v.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CREF { componentRef: cr, .. }, r#impl, msg, _) => {
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    (cache, v) = cevalCref(cache.clone(), env.clone(), cr.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
                    Ok((cache.clone(), v.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, expExp, r#impl, msg, _) => {
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    (cache, v) = cevalBuiltin(cache.clone(), env.clone(), expExp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
                    Ok((cache.clone(), v.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { isImpure: false, .. }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: 0 }, tail: Deref @ metamodelica::List::Cons { head: expExp, tail: Deref @ metamodelica::List::Nil } }, path: funcpath }, r#impl, msg, _) => {
                    let mut value: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    ::match_deref::match_deref! { match &(AbsynUtil::makeNotFullyQualified(funcpath.clone())) {
                        Deref @ Absyn::Path::IDENT { name: Deref @ "smooth" } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    (cache, value) = ceval(cache.clone(), env.clone(), expExp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
                    Ok((cache.clone(), value.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, e @ Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { isImpure: false, .. }, expLst: expl, path: funcpath }, r#impl, msg, _) => {
                    let mut vallst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut newval: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    let false = (AbsynUtil::pathEqual(Arc::new(Absyn::Path::QUALIFIED { name: (literal!("Connection")).clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("isRoot")).clone() }) }), funcpath.clone())) else { bail!("pattern mismatch") };
                    (cache, vallst) = cevalList(cache.clone(), env.clone(), expl.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
                    (cache, newval) = BackendCevalInterface::cevalCallFunction(cache.clone(), env.clone(), e.clone(), vallst.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1);
                    Ok((cache.clone(), newval.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CAST { exp: e, ty }, r#impl, msg, _) => {
                    let mut value: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    let true = (Types::isRecord(ty.clone())) else { bail!("pattern mismatch") };
                    (cache, value) = ceval(cache.clone(), env.clone(), e.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
                    Ok((cache.clone(), value.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, e @ Deref @ DAE::Exp::CALL { .. }, true, msg, _) => {
                    let mut value: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    (cache, value) = BackendCevalInterface::cevalInteractiveFunctions(cache.clone(), env.clone(), e.clone(), msg.clone(), numIter.clone() + 1);
                    Ok((cache.clone(), value.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, e @ Deref @ DAE::Exp::CALL { .. }, _, _, _) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- Ceval.ceval DAE.CALL failed: ")).clone())?;
                    r#str = (ExpressionBasics::printExpStr(e.clone())?).clone();
                    Debug::traceln((r#str.clone()).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::RECORD { comp: fieldNames, exps: expl, path: funcpath, .. }, r#impl, msg, _) => {
                    let mut vallst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, vallst) = cevalList(cache.clone(), env.clone(), expl.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
                    Ok((cache.clone(), Arc::new(Values::Value::RECORD { record_: funcpath.clone(), orderd: vallst.clone(), comp: fieldNames.clone(), index: -1 })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp2: rh, operator: DAE::Operator::ADD { ty: Deref @ DAE::Type::T_STRING { .. } }, exp1: lh }, r#impl, msg, _) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut lhvStr: ArcStr = arcstr::literal!("");
                    let mut rhvStr: ArcStr = arcstr::literal!("");
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?) {
                        (__pa0, Deref @ Values::Value::STRING { string: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    lhvStr = __pa1.clone();
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?) {
                        (__pa2, Deref @ Values::Value::STRING { string: __pa3 }) => (__pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa2.clone();
                    rhvStr = __pa3.clone();
                    r#str = (stringAppend((lhvStr.clone()).clone(), (rhvStr.clone()).clone())).clone();
                    Ok((cache.clone(), Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp2: rh, operator: DAE::Operator::ADD { ty: Deref @ DAE::Type::T_REAL { .. } }, exp1: lh }, r#impl, msg, _) => {
                    let mut lhvReal: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut rhvReal: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut sum: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?) {
                        (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    lhvReal = __pa1.clone();
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?) {
                        (__pa2, Deref @ Values::Value::REAL { real: __pa3 }) => (__pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa2.clone();
                    rhvReal = __pa3.clone();
                    sum = lhvReal.clone() + rhvReal.clone();
                    Ok((cache.clone(), Arc::new(Values::Value::REAL { real: sum.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp2: rh, operator: DAE::Operator::ADD_ARR { .. }, exp1: lh }, r#impl, msg, _) => {
                    let mut vlst1: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut vlst2: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut reslst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, dimLst: __pa2 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    vlst1 = __pa1.clone();
                    dims = __pa2.clone();
                    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?) {
                        (__pa3, Deref @ Values::Value::ARRAY { valueLst: __pa4, dimLst: _ }) => (__pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa3.clone();
                    vlst2 = __pa4.clone();
                    reslst = ValuesUtil::addElementwiseArrayelt(vlst1.clone(), vlst2.clone())?;
                    Ok((cache.clone(), Arc::new(Values::Value::ARRAY { valueLst: reslst.clone(), dimLst: dims.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp2: rh, operator: DAE::Operator::SUB_ARR { .. }, exp1: lh }, r#impl, msg, _) => {
                    let mut vlst1: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut vlst2: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut reslst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, dimLst: __pa2 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    vlst1 = __pa1.clone();
                    dims = __pa2.clone();
                    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?) {
                        (__pa3, Deref @ Values::Value::ARRAY { valueLst: __pa4, dimLst: _ }) => (__pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa3.clone();
                    vlst2 = __pa4.clone();
                    reslst = ValuesUtil::subElementwiseArrayelt(vlst1.clone(), vlst2.clone())?;
                    Ok((cache.clone(), Arc::new(Values::Value::ARRAY { valueLst: reslst.clone(), dimLst: dims.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp2: rh, operator: DAE::Operator::MUL_ARR { .. }, exp1: lh }, r#impl, msg, _) => {
                    let mut vlst1: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut vlst2: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut reslst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, dimLst: __pa2 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    vlst1 = __pa1.clone();
                    dims = __pa2.clone();
                    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?) {
                        (__pa3, Deref @ Values::Value::ARRAY { valueLst: __pa4, dimLst: _ }) => (__pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa3.clone();
                    vlst2 = __pa4.clone();
                    reslst = ValuesUtil::mulElementwiseArrayelt(vlst1.clone(), vlst2.clone())?;
                    Ok((cache.clone(), Arc::new(Values::Value::ARRAY { valueLst: reslst.clone(), dimLst: dims.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp2: rh, operator: DAE::Operator::DIV_ARR { .. }, exp1: lh }, r#impl, msg, _) => {
                    let mut vlst1: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut vlst2: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut reslst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, dimLst: __pa2 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    vlst1 = __pa1.clone();
                    dims = __pa2.clone();
                    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?) {
                        (__pa3, Deref @ Values::Value::ARRAY { valueLst: __pa4, dimLst: _ }) => (__pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa3.clone();
                    vlst2 = __pa4.clone();
                    reslst = ValuesUtil::divElementwiseArrayelt(vlst1.clone(), vlst2.clone())?;
                    Ok((cache.clone(), Arc::new(Values::Value::ARRAY { valueLst: reslst.clone(), dimLst: dims.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp2: rh, operator: DAE::Operator::POW_ARR2 { .. }, exp1: lh }, r#impl, msg, _) => {
                    let mut vlst1: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut vlst2: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut reslst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, dimLst: __pa2 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    vlst1 = __pa1.clone();
                    dims = __pa2.clone();
                    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?) {
                        (__pa3, Deref @ Values::Value::ARRAY { valueLst: __pa4, dimLst: _ }) => (__pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa3.clone();
                    vlst2 = __pa4.clone();
                    reslst = ValuesUtil::powElementwiseArrayelt(vlst1.clone(), vlst2.clone())?;
                    Ok((cache.clone(), Arc::new(Values::Value::ARRAY { valueLst: reslst.clone(), dimLst: dims.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp2: rh, operator: DAE::Operator::MUL_ARRAY_SCALAR { .. }, exp1: lh }, r#impl, msg, _) => {
                    let mut reslst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut aval: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut sval: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, sval) = ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, dimLst: __pa2 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    aval = __pa1.clone();
                    dims = __pa2.clone();
                    reslst = ValuesUtil::multScalarArrayelt(sval.clone(), aval.clone())?;
                    Ok((cache.clone(), Arc::new(Values::Value::ARRAY { valueLst: reslst.clone(), dimLst: dims.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp2: rh, operator: DAE::Operator::ADD_ARRAY_SCALAR { .. }, exp1: lh }, r#impl, msg, _) => {
                    let mut reslst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut aval: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut sval: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, sval) = ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, dimLst: __pa2 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    aval = __pa1.clone();
                    dims = __pa2.clone();
                    reslst = ValuesUtil::addScalarArrayelt(sval.clone(), aval.clone())?;
                    Ok((cache.clone(), Arc::new(Values::Value::ARRAY { valueLst: reslst.clone(), dimLst: dims.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp2: rh, operator: DAE::Operator::SUB_SCALAR_ARRAY { .. }, exp1: lh }, r#impl, msg, _) => {
                    let mut reslst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut aval: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut sval: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, sval) = ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, dimLst: __pa2 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    aval = __pa1.clone();
                    dims = __pa2.clone();
                    reslst = ValuesUtil::subScalarArrayelt(sval.clone(), aval.clone())?;
                    Ok((cache.clone(), Arc::new(Values::Value::ARRAY { valueLst: reslst.clone(), dimLst: dims.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp2: rh, operator: DAE::Operator::POW_SCALAR_ARRAY { .. }, exp1: lh }, r#impl, msg, _) => {
                    let mut reslst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut aval: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut sval: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, sval) = ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, dimLst: __pa2 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    aval = __pa1.clone();
                    dims = __pa2.clone();
                    reslst = ValuesUtil::powScalarArrayelt(sval.clone(), aval.clone())?;
                    Ok((cache.clone(), Arc::new(Values::Value::ARRAY { valueLst: reslst.clone(), dimLst: dims.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp2: rh, operator: DAE::Operator::POW_ARRAY_SCALAR { .. }, exp1: lh }, r#impl, msg, _) => {
                    let mut reslst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut aval: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut sval: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, sval) = ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, dimLst: __pa2 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    aval = __pa1.clone();
                    dims = __pa2.clone();
                    reslst = ValuesUtil::powArrayeltScalar(sval.clone(), aval.clone())?;
                    Ok((cache.clone(), Arc::new(Values::Value::ARRAY { valueLst: reslst.clone(), dimLst: dims.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp2: rh, operator: DAE::Operator::DIV_SCALAR_ARRAY { .. }, exp1: lh }, r#impl, msg, _) => {
                    let mut reslst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut aval: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut sval: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, sval) = ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, dimLst: __pa2 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    aval = __pa1.clone();
                    dims = __pa2.clone();
                    reslst = ValuesUtil::divScalarArrayelt(sval.clone(), aval.clone())?;
                    Ok((cache.clone(), Arc::new(Values::Value::ARRAY { valueLst: reslst.clone(), dimLst: dims.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp2: rh, operator: DAE::Operator::DIV_ARRAY_SCALAR { .. }, exp1: lh }, r#impl, msg, _) => {
                    let mut reslst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut aval: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut sval: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, sval) = ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, dimLst: __pa2 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    aval = __pa1.clone();
                    dims = __pa2.clone();
                    reslst = ValuesUtil::divArrayeltScalar(sval.clone(), aval.clone())?;
                    Ok((cache.clone(), Arc::new(Values::Value::ARRAY { valueLst: reslst.clone(), dimLst: dims.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp2: rh, operator: DAE::Operator::MUL_SCALAR_PRODUCT { .. }, exp1: lh }, r#impl, msg, _) => {
                    let mut rhvals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut lhvals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut resVal: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, .. }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    rhvals = __pa1.clone();
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?) {
                        (__pa2, Deref @ Values::Value::ARRAY { valueLst: __pa3, .. }) => (__pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa2.clone();
                    lhvals = __pa3.clone();
                    resVal = ValuesUtil::multScalarProduct(rhvals.clone(), lhvals.clone())?;
                    Ok((cache.clone(), resVal.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp2: rh, operator: DAE::Operator::MUL_MATRIX_PRODUCT { .. }, exp1: lh }, r#impl, msg, _) => {
                    let mut rhvals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut lhvals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut elt1: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut elt2: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut resVal: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa2, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa2 @ Deref @ metamodelica::List::Cons { head: __pa1, tail: _ }, .. }) => (__pa0.clone(), __pa2.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    elt1 = __pa1.clone();
                    lhvals = __pa2.clone();
                    let (__pa3, __pa5, __pa4) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?) {
                        (__pa3, Deref @ Values::Value::ARRAY { valueLst: __pa5 @ Deref @ metamodelica::List::Cons { head: __pa4, tail: _ }, .. }) => (__pa3.clone(), __pa5.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa3.clone();
                    elt2 = __pa4.clone();
                    rhvals = __pa5.clone();
                    let true = (ValuesUtil::isArray(elt1.clone())) else { bail!("pattern mismatch") };
                    let false = (ValuesUtil::isArray(elt2.clone())) else { bail!("pattern mismatch") };
                    resVal = ValuesUtil::multScalarProduct(lhvals.clone(), rhvals.clone())?;
                    Ok((cache.clone(), resVal.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp2: rh, operator: DAE::Operator::MUL_MATRIX_PRODUCT { .. }, exp1: lh }, r#impl, msg, _) => {
                    let mut rhvals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut lhvals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut elt1: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut elt2: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut resVal: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa2, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa2 @ Deref @ metamodelica::List::Cons { head: __pa1, tail: _ }, .. }) => (__pa0.clone(), __pa2.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    elt1 = __pa1.clone();
                    rhvals = __pa2.clone();
                    let (__pa3, __pa5, __pa4) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?) {
                        (__pa3, Deref @ Values::Value::ARRAY { valueLst: __pa5 @ Deref @ metamodelica::List::Cons { head: __pa4, tail: _ }, .. }) => (__pa3.clone(), __pa5.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa3.clone();
                    elt2 = __pa4.clone();
                    lhvals = __pa5.clone();
                    let true = (ValuesUtil::isArray(elt1.clone())) else { bail!("pattern mismatch") };
                    let false = (ValuesUtil::isArray(elt2.clone())) else { bail!("pattern mismatch") };
                    resVal = ValuesUtil::multScalarProduct(lhvals.clone(), rhvals.clone())?;
                    Ok((cache.clone(), resVal.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp2: rh, operator: DAE::Operator::MUL_MATRIX_PRODUCT { .. }, exp1: lh }, r#impl, msg, _) => {
                    let mut vallst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut rhvals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut lhvals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut elt1: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut elt2: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa2, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa2 @ Deref @ metamodelica::List::Cons { head: __pa1, tail: _ }, dimLst: _ }) => (__pa0.clone(), __pa2.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    elt1 = __pa1.clone();
                    rhvals = __pa2.clone();
                    let (__pa3, __pa5, __pa4) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa3, Deref @ Values::Value::ARRAY { valueLst: __pa5 @ Deref @ metamodelica::List::Cons { head: __pa4, tail: _ }, dimLst: _ }) => (__pa3.clone(), __pa5.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa3.clone();
                    elt2 = __pa4.clone();
                    lhvals = __pa5.clone();
                    let true = (ValuesUtil::isArray(elt1.clone())) else { bail!("pattern mismatch") };
                    let true = (ValuesUtil::isArray(elt2.clone())) else { bail!("pattern mismatch") };
                    vallst = ValuesUtil::multMatrix(lhvals.clone(), rhvals.clone())?;
                    Ok((cache.clone(), ValuesMake::makeArray(vallst.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp2: rh, operator: DAE::Operator::POW { .. }, exp1: lh }, r#impl, msg, _) => {
                    let mut resVal: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut lhvVal: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut rhvVal: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    (cache, lhvVal) = ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
                    (cache, rhvVal) = ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
                    resVal = ValuesUtil::safeIntRealOp(lhvVal.clone(), rhvVal.clone(), openmodelica_frontend_types::Values::IntRealOp::POWOP)?;
                    Ok((cache.clone(), resVal.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp2: rh, operator: DAE::Operator::MUL { .. }, exp1: lh }, r#impl, msg, _) => {
                    let mut resVal: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut lhvVal: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut rhvVal: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    (cache, lhvVal) = ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
                    (cache, rhvVal) = ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
                    resVal = ValuesUtil::safeIntRealOp(lhvVal.clone(), rhvVal.clone(), openmodelica_frontend_types::Values::IntRealOp::MULOP)?;
                    Ok((cache.clone(), resVal.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp2: rh, operator: DAE::Operator::DIV { .. }, exp1: lh }, r#impl, msg, _) => {
                    let mut resVal: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut lhvVal: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut rhvVal: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    (cache, lhvVal) = ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
                    (cache, rhvVal) = ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
                    resVal = ValuesUtil::safeIntRealOp(lhvVal.clone(), rhvVal.clone(), openmodelica_frontend_types::Values::IntRealOp::DIVOP)?;
                    Ok((cache.clone(), resVal.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp2: rh, operator: DAE::Operator::DIV { .. }, exp1: lh }, r#impl, msg @ Absyn::Msg::MSG { info }, _) => {
                    let mut lhvStr: ArcStr = arcstr::literal!("");
                    let mut rhvStr: ArcStr = arcstr::literal!("");
                    let mut lhvVal: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    (_, lhvVal) = ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
                    let true = (ValuesUtil::isZero(lhvVal.clone())) else { bail!("pattern mismatch") };
                    lhvStr = (ExpressionBasics::printExpStr(lh.clone())?).clone();
                    rhvStr = (ExpressionBasics::printExpStr(rh.clone())?).clone();
                    Error::addSourceMessage(Error::DIVISION_BY_ZERO.clone(), list![(lhvStr.clone()).clone(), (rhvStr.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp2: rh, operator: DAE::Operator::ADD { .. }, exp1: lh }, r#impl, msg, _) => {
                    let mut resVal: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut lhvVal: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut rhvVal: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    (cache, lhvVal) = ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
                    (cache, rhvVal) = ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
                    resVal = ValuesUtil::safeIntRealOp(lhvVal.clone(), rhvVal.clone(), openmodelica_frontend_types::Values::IntRealOp::ADDOP)?;
                    Ok((cache.clone(), resVal.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::BINARY { exp2: rh, operator: DAE::Operator::SUB { .. }, exp1: lh }, r#impl, msg, _) => {
                    let mut resVal: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut lhvVal: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut rhvVal: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    (cache, lhvVal) = ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
                    (cache, rhvVal) = ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
                    resVal = ValuesUtil::safeIntRealOp(lhvVal.clone(), rhvVal.clone(), openmodelica_frontend_types::Values::IntRealOp::SUBOP)?;
                    Ok((cache.clone(), resVal.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::UNARY { exp: daeExp, operator: DAE::Operator::UMINUS_ARR { .. } }, r#impl, msg, _) => {
                    let mut arr: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut arr_1: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), daeExp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, dimLst: __pa2 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    arr = __pa1.clone();
                    dims = __pa2.clone();
                    arr_1 = List::map(arr.clone(), (std::sync::Arc::new(ValuesUtil::valueNeg) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<Arc<Values::Value>> + 'static>));
                    Ok((cache.clone(), Arc::new(Values::Value::ARRAY { valueLst: arr_1.clone(), dimLst: dims.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::UNARY { exp: daeExp, operator: DAE::Operator::UMINUS { .. } }, r#impl, msg, _) => {
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut v_1: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    (cache, v) = ceval(cache.clone(), env.clone(), daeExp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
                    v_1 = ValuesUtil::valueNeg(v.clone())?;
                    Ok((cache.clone(), v_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::LBINARY { exp2: rh, operator: DAE::Operator::AND { ty: _ }, exp1: lh }, r#impl, msg, _) => {
                    let mut lhvBool: bool = false;
                    let mut rhvBool: bool = false;
                    let mut resBool: bool = false;
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?) {
                        (__pa0, Deref @ Values::Value::BOOL { boolean: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    lhvBool = __pa1.clone();
                    if !(lhvBool.clone()) {
                        v = Arc::new(Values::Value::BOOL { boolean: false });
                    } else {
                        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?) {
                            (__pa2, Deref @ Values::Value::BOOL { boolean: __pa3 }) => (__pa2.clone(), __pa3.clone()),
                            _ => bail!("pattern mismatch"),
                        } };
                        cache = __pa2.clone();
                        rhvBool = __pa3.clone();
                        resBool = boolAnd(lhvBool.clone(), rhvBool.clone());
                        v = Arc::new(Values::Value::BOOL { boolean: resBool.clone() });
                    }
                    Ok((cache.clone(), v.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::LBINARY { exp2: rh, operator: DAE::Operator::OR { ty: _ }, exp1: lh }, r#impl, msg, _) => {
                    let mut lhvBool: bool = false;
                    let mut rhvBool: bool = false;
                    let mut resBool: bool = false;
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?) {
                        (__pa0, Deref @ Values::Value::BOOL { boolean: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    lhvBool = __pa1.clone();
                    if lhvBool.clone() {
                        v = Arc::new(Values::Value::BOOL { boolean: true });
                    } else {
                        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?) {
                            (__pa2, Deref @ Values::Value::BOOL { boolean: __pa3 }) => (__pa2.clone(), __pa3.clone()),
                            _ => bail!("pattern mismatch"),
                        } };
                        cache = __pa2.clone();
                        rhvBool = __pa3.clone();
                        resBool = boolOr(lhvBool.clone(), rhvBool.clone());
                        v = Arc::new(Values::Value::BOOL { boolean: resBool.clone() });
                    }
                    Ok((cache.clone(), v.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::LBINARY { exp2: rh, operator: DAE::Operator::OR { ty: _ }, exp1: lh }, r#impl, msg, _) => {
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), lh.clone(), r#impl.clone(), msg.clone(), numIter.clone())?) {
                        (__pa0, __pa1 @ Deref @ Values::Value::BOOL { boolean: _ }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    v = __pa1.clone();
                    if '__try2: {
                        unwrap_break_err!(ceval(cache.clone(), env.clone(), rh.clone(), r#impl.clone(), msg.clone(), numIter.clone()), '__try2);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    Ok((cache.clone(), v.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::LUNARY { exp: e, operator: DAE::Operator::NOT { ty: _ } }, r#impl, msg, _) => {
                    let mut b: bool = false;
                    let mut b_1: bool = false;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), e.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa0, Deref @ Values::Value::BOOL { boolean: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    b = __pa1.clone();
                    b_1 = boolNot(b.clone());
                    Ok((cache.clone(), Arc::new(Values::Value::BOOL { boolean: b_1.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::RELATION { exp2: rhs, operator: relop, exp1: lhs, .. }, r#impl, msg, _) => {
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut lhs_1: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut rhs_1: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    (cache, lhs_1) = ceval(cache.clone(), env.clone(), lhs.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
                    (cache, rhs_1) = ceval(cache.clone(), env.clone(), rhs.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
                    v = cevalRelation(lhs_1.clone(), relop.clone(), rhs_1.clone())?;
                    Ok((cache.clone(), v.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ DAE::Exp::RANGE { .. }, _, _, _) => {
                    Ok(cevalRange(inCache.clone(), inEnv.clone(), inExp.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CAST { exp: e, ty: Deref @ DAE::Type::T_REAL { .. } }, r#impl, msg, _) => {
                    let mut i: i32 = 0;
                    let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), e.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    i = __pa1.clone();
                    r = intReal(i.clone());
                    Ok((cache.clone(), Arc::new(Values::Value::REAL { real: r.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CAST { exp: e, ty: Deref @ DAE::Type::T_INTEGER { .. } }, r#impl, msg, _) => {
                    let mut i: i32 = 0;
                    let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), e.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    r = __pa1.clone();
                    i = ((r.clone()).0 as i32);
                    Ok((cache.clone(), Arc::new(Values::Value::INTEGER { integer: i.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CAST { exp: e, ty: Deref @ DAE::Type::T_ENUMERATION { names: n, path, .. } }, r#impl, msg, _) => {
                    let mut i: i32 = 0;
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut cache = (*cache).clone();
                    let mut path = (*path).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), e.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    i = __pa1.clone();
                    r#str = ((n.clone()).get(i.clone())?).clone();
                    path = AbsynUtil::joinPaths(path.clone(), Arc::new(Absyn::Path::IDENT { name: (r#str.clone()).clone() }))?;
                    Ok((cache.clone(), Arc::new(Values::Value::ENUM_LITERAL { name: path.clone(), index: i.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CAST { exp: e, ty: Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_REAL { .. }, .. } }, r#impl, msg, _) => {
                    let mut ivals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut rvals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), e.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, dimLst: __pa2 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    ivals = __pa1.clone();
                    dims = __pa2.clone();
                    rvals = ValuesUtil::typeConvert(DAE::T_INTEGER_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone(), ivals.clone())?;
                    Ok((cache.clone(), Arc::new(Values::Value::ARRAY { valueLst: rvals.clone(), dimLst: dims.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::IFEXP { expElse: e2, expThen: e1, expCond: cond }, r#impl, msg, _) => {
                    let mut resBool: bool = false;
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    (cache, v) = ceval(cache.clone(), env.clone(), cond.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
                    let __pa0 = ::match_deref::match_deref! { match &(v.clone()) {
                        Deref @ Values::Value::BOOL { boolean: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    resBool = __pa0.clone();
                    (cache, v) = ceval(cache.clone(), env.clone(), if (resBool.clone()) {e1.clone()} else {e2.clone()}, r#impl.clone(), msg.clone(), numIter.clone())?;
                    Ok((cache.clone(), v.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::ASUB { sub: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: indx } }, tail: Deref @ metamodelica::List::Nil }, exp: e }, r#impl, msg, _) => {
                    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), e.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, dimLst: _ }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    vals = __pa1.clone();
                    v = (vals.clone()).get(indx.clone())?;
                    Ok((cache.clone(), v.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::ASUB { sub: subs, exp: e }, r#impl, msg, _) => {
                    let mut es_1: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    expl = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut sub in (subs.clone()).into_iter().cloned() {
                    let __x = Expression::getSubscriptExp(sub.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), e.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, dimLst: __pa2 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    vals = __pa1.clone();
                    dims = __pa2.clone();
                    (cache, es_1) = cevalList(cache.clone(), env.clone(), expl.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
                    v = listHead(es_1.clone())?;
                    v = ValuesUtil::nthnthArrayelt(es_1.clone(), Arc::new(Values::Value::ARRAY { valueLst: vals.clone(), dimLst: dims.clone() }), v.clone())?;
                    Ok((cache.clone(), v.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::TSUB { ix: indx, exp: e, .. }, r#impl, msg, _) => {
                    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), e.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa0, Deref @ Values::Value::TUPLE { valueLst: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    vals = __pa1.clone();
                    v = (vals.clone()).get(indx.clone())?;
                    Ok((cache.clone(), v.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::REDUCTION { iterators, expr: daeExp, reductionInfo: Deref @ DAE::ReductionInfo { exprType: ty, defaultValue: ov, foldExp, resultName, foldName, path, iterType } }, r#impl, msg, _) => {
                    let mut value: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut valMatrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Values::Value>>>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ov = (*ov).clone();
                    env = FGraph::openScope(env.clone(), openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, (arcstr::literal!(FCore::forScopeName)).clone(), None)?;
                    (cache, valMatrix, names, dims, tys) = cevalReductionIterators(cache.clone(), env.clone(), iterators.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
                    valMatrix = makeReductionAllCombinations(valMatrix.clone(), iterType.clone())?;
                    (cache, ov) = cevalReduction(cache.clone(), env.clone(), path.clone(), ov.clone(), daeExp.clone(), ty.clone(), (foldName.clone()).clone(), (resultName.clone()).clone(), foldExp.clone(), names.clone(), valMatrix.clone().reverse(), tys.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
                    value = Util::getOptionOrDefault(ov.clone(), Arc::new(openmodelica_frontend_types::Values::Value::META_FAIL));
                    value = backpatchArrayReduction(path.clone(), iterType.clone(), value.clone(), dims.clone())?;
                    Ok((cache.clone(), value.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ DAE::Exp::EMPTY { .. }, _, _, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    s = (ComponentReferenceBasics::printComponentRefStr(var_field!((*inExp).name, DAE::Exp::EMPTY).clone())?).clone();
                    v = Types::typeToValue(var_field!((*inExp).ty, DAE::Exp::EMPTY).clone())?;
                    Ok((inCache.clone(), Arc::new(Values::Value::EMPTY { scope: (var_field!((*inExp).scope, DAE::Exp::EMPTY).clone()).clone(), name: (s.clone()).clone(), ty: v.clone(), tyStr: (var_field!((*inExp).tyStr, DAE::Exp::EMPTY).clone()).clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _) => {
                    if !((Config::getGraphicsExpMode()?)) { bail!("guard") }
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    ty = Expression::r#typeof(inExp.clone())?;
                    v = Types::typeToValue(ty.clone())?;
                    Ok((inCache.clone(), Arc::new(Values::Value::EMPTY { scope: (literal!("#graphicsExp#")).clone(), name: (ExpressionBasics::printExpStr(inExp.clone())?).clone(), ty: v.clone(), tyStr: (TypesDump::unparseType(ty.clone())?).clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, env, e, _, _, _) => {
                    let true = (Flags::isSet(Flags::CEVAL.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Ceval.ceval failed: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  Scope: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(env.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outValue))
}

pub fn cevalIfConstant(mut cache: FCore::Cache, mut inEnv: FCore::Graph, mut exp: Arc<DAE::Exp>, mut prop: DAE::Properties, mut r#impl: bool, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>, DAE::Properties)> {
    let mut cache: FCore::Cache = cache;
    let mut exp: Arc<DAE::Exp> = exp;
    let mut prop: DAE::Properties = prop;
    if Expression::isEvaluatedConst(exp.clone()) {
        return Ok((cache, exp, prop));
    }
    (cache, exp, prop) = 'mc: {
        let __mc_input = prop.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Properties::PROP { type_: ref tp, constFlag: DAE::Const::C_PARAM } = __mc_input.clone() else { bail!("nomatch") };
            if !((!(Flags::getConfigBool(Flags::CEVAL_EQUATION.clone())?))) { bail!("guard") }
            Ok((cache.clone(), exp.clone(), DAE::Properties::PROP { type_: tp.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Properties::PROP { type_: ref tp, constFlag: DAE::Const::C_CONST } = __mc_input.clone() else { bail!("nomatch") };
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut exp: Arc<DAE::Exp> = exp.clone();
            let mut cache: FCore::Cache = cache.clone();
            (cache, v) = ceval(cache.clone(), inEnv.clone(), exp.clone(), r#impl.clone(), Absyn::Msg::MSG { info: inInfo.clone() }, 0)?;
            exp = ValuesUtil::valueExp(v.clone(), Some(exp.clone()))?;
            exp = ValuesUtil::fixZeroSizeArray(exp.clone(), tp.clone())?;
            Ok((cache.clone(), exp.clone(), prop.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Properties::PROP_TUPLE { .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut exp: Arc<DAE::Exp> = exp.clone();
            let mut cache: FCore::Cache = cache.clone();
            let DAE::C_CONST { .. } = (Types::propAllConst(prop.clone())?) else { bail!("pattern mismatch") };
            (cache, v) = ceval(cache.clone(), inEnv.clone(), exp.clone(), false, Absyn::Msg::MSG { info: inInfo.clone() }, 0)?;
            exp = ValuesUtil::valueExp(v.clone(), Some(exp.clone()))?;
            Ok((cache.clone(), exp.clone(), prop.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Properties::PROP_TUPLE { .. } = __mc_input.clone() else { bail!("nomatch") };
            if !((!(Flags::getConfigBool(Flags::CEVAL_EQUATION.clone())?))) { bail!("guard") }
            let DAE::C_PARAM { .. } = (Types::propAllConst(prop.clone())?) else { bail!("pattern mismatch") };
            println!("{}", (literal!(" tuple non constant evaluation not implemented yet\n")).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            if !((Expression::isConst(exp.clone())? && !(Config::acceptMetaModelicaGrammar()?))) { bail!("guard") }
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut exp: Arc<DAE::Exp> = exp.clone();
            (_, v) = ceval(cache.clone(), inEnv.clone(), exp.clone(), r#impl.clone(), Absyn::Msg::MSG { info: inInfo.clone() }, 0)?;
            exp = ValuesUtil::valueExp(v.clone(), Some(exp.clone()))?;
            exp = ValuesUtil::fixZeroSizeArray(exp.clone(), Types::getPropType(prop.clone())?)?;
            Ok((cache.clone(), exp.clone(), prop.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut exp: Arc<DAE::Exp> = exp.clone();
            (exp, _) = ExpressionSimplify::simplify1(exp.clone())?;
            Ok((cache.clone(), exp.clone(), prop.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((cache, exp, prop))
}

fn cevalWholedimRetCall(mut inExp: Arc<DAE::Exp>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inInfo: SourceInfo, mut numIter: i32) -> Result<(Arc<DAE::Exp>, DAE::Properties)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outProp: DAE::Properties;
    (outExp, outProp) = (::match_deref::match_deref! { match &((inExp.clone(), inCache.clone(), inEnv.clone(), inInfo.clone(), numIter.clone())) {
        (e @ Deref @ DAE::Exp::CALL { attr: attr @ Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_ARRAY { dims, .. }, .. }, expLst: el, path: p }, _, _, _, _) => {
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut cevalType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut attr = (*attr).clone();
            let true = (Expression::arrayContainWholeDimension(dims.clone())) else { bail!("pattern mismatch") };
            (_, v) = ceval(inCache.clone(), inEnv.clone(), e.clone(), true, Absyn::Msg::MSG { info: inInfo.clone() }, numIter.clone() + 1)?;
            ty = Types::typeOfValue(v.clone())?;
            cevalType = Types::simplifyType(ty.clone())?;
            assign_field!(attr.ty = cevalType.clone());
            (Arc::new(DAE::Exp::CALL { path: p.clone(), expLst: el.clone(), attr: attr.clone() }), DAE::Properties::PROP { type_: ty.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_PARAM })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outExp, outProp))
}

pub fn cevalRangeIfConstant(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<DAE::Exp>, mut inProp: DAE::Properties, mut r#impl: bool, mut inInfo: SourceInfo) -> Result<(FCore::Cache, Arc<DAE::Exp>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<DAE::Exp>;
    (outCache, outExp) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inExp.clone(), inProp.clone(), r#impl.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ DAE::Exp::RANGE { step: e3, stop: e2, start: e1, ty }, _, _, _) => {
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut e2 = (*e2).clone();
                    let mut e1 = (*e1).clone();
                    (cache, e1, _) = cevalIfConstant(inCache.clone(), inEnv.clone(), e1.clone(), inProp.clone(), r#impl.clone(), inInfo.clone())?;
                    (_, e2, _) = cevalIfConstant(cache.clone(), inEnv.clone(), e2.clone(), inProp.clone(), r#impl.clone(), inInfo.clone())?;
                    Ok((inCache.clone(), Arc::new(DAE::Exp::RANGE { ty: ty.clone(), start: e1.clone(), step: e3.clone(), stop: e2.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inCache.clone(), inExp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outExp))
}

fn cevalBuiltin(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<DAE::Exp>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    pub type HandlerFunc = std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>;

    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inExp.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::SIZE { sz: Some(dim), exp }, r#impl, msg, _) => {
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    (cache, v) = cevalBuiltinSize(cache.clone(), env.clone(), exp.clone(), dim.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
                    Ok((cache.clone(), v.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::SIZE { sz: None, exp }, r#impl, msg, _) => {
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    (cache, v) = cevalBuiltinSizeMatrix(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
                    Ok((cache.clone(), v.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { builtin: true, .. }, expLst: args, path }, r#impl, msg, _) => {
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut handler: Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>;
                    let mut id: ArcStr = arcstr::literal!("");
                    let mut cache = (*cache).clone();
                    id = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
                    handler = cevalBuiltinHandler((id.clone()).clone())?;
                    (cache, v) = handler(cache.clone(), env.clone(), args.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
                    Ok((cache.clone(), v.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, e @ Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { builtin: true, .. }, expLst: expl, .. }, r#impl, msg, _) => {
                    let mut newval: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut vallst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, vallst) = cevalList(cache.clone(), env.clone(), expl.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
                    (cache, newval) = BackendCevalInterface::cevalCallFunction(cache.clone(), env.clone(), e.clone(), vallst.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1);
                    Ok((cache.clone(), newval.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outValue))
}

fn cevalBuiltinHandler(mut inIdent: ArcStr) -> Result<fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)>> {
    pub type HandlerFunc = std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>;

    let mut handler: Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>;
    handler = (::match_deref::match_deref! { match &(inIdent.clone()) {
        Deref @ "floor" => {
            (std::sync::Arc::new(cevalBuiltinFloor) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "ceil" => {
            (std::sync::Arc::new(cevalBuiltinCeil) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "abs" => {
            (std::sync::Arc::new(cevalBuiltinAbs) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "sqrt" => {
            (std::sync::Arc::new(cevalBuiltinSqrt) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "div" => {
            (std::sync::Arc::new(cevalBuiltinDiv) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "sin" => {
            (std::sync::Arc::new(cevalBuiltinSin) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "cos" => {
            (std::sync::Arc::new(cevalBuiltinCos) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "tan" => {
            (std::sync::Arc::new(cevalBuiltinTan) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "sinh" => {
            (std::sync::Arc::new(cevalBuiltinSinh) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "cosh" => {
            (std::sync::Arc::new(cevalBuiltinCosh) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "tanh" => {
            (std::sync::Arc::new(cevalBuiltinTanh) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "asin" => {
            (std::sync::Arc::new(cevalBuiltinAsin) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "acos" => {
            (std::sync::Arc::new(cevalBuiltinAcos) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "atan" => {
            (std::sync::Arc::new(cevalBuiltinAtan) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "atan2" => {
            (std::sync::Arc::new(cevalBuiltinAtan2) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "log" => {
            (std::sync::Arc::new(cevalBuiltinLog) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "log10" => {
            (std::sync::Arc::new(cevalBuiltinLog10) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "integer" => {
            (std::sync::Arc::new(cevalBuiltinInteger) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "boolean" => {
            (std::sync::Arc::new(cevalBuiltinBoolean) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "mod" => {
            (std::sync::Arc::new(cevalBuiltinMod) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "max" => {
            (std::sync::Arc::new(cevalBuiltinMax) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "min" => {
            (std::sync::Arc::new(cevalBuiltinMin) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "rem" => {
            (std::sync::Arc::new(cevalBuiltinRem) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "sum" => {
            (std::sync::Arc::new(cevalBuiltinSum) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "diagonal" => {
            (std::sync::Arc::new(cevalBuiltinDiagonal) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "sign" => {
            (std::sync::Arc::new(cevalBuiltinSign) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "exp" => {
            (std::sync::Arc::new(cevalBuiltinExp) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "noEvent" => {
            (std::sync::Arc::new(cevalBuiltinNoevent) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "cat" => {
            (std::sync::Arc::new(cevalBuiltinCat) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "identity" => {
            (std::sync::Arc::new(cevalBuiltinIdentity) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "promote" => {
            (std::sync::Arc::new(cevalBuiltinPromote) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "String" => {
            (std::sync::Arc::new(cevalBuiltinString) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "Integer" => {
            (std::sync::Arc::new(cevalBuiltinIntegerEnumeration) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "rooted" => {
            (std::sync::Arc::new(cevalBuiltinRooted) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "cross" => {
            (std::sync::Arc::new(cevalBuiltinCross) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "fill" => {
            (std::sync::Arc::new(cevalBuiltinFill) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "Modelica.Utilities.Strings.substring" => {
            (std::sync::Arc::new(cevalBuiltinSubstring) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "print" => {
            (std::sync::Arc::new(cevalBuiltinPrint) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "fail" => {
            (std::sync::Arc::new(fnptr!(cevalBuiltinFail, FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "intString" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(cevalIntString) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "realString" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(cevalRealString) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "stringCharInt" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(cevalStringCharInt) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "intStringChar" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(cevalIntStringChar) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "stringLength" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(cevalStringLength) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "stringInt" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(cevalStringInt) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "stringListStringChar" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(cevalStringListStringChar) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "listStringCharString" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(cevalListStringCharString) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "stringAppendList" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(cevalStringAppendList) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "stringDelimitList" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(cevalStringDelimitList) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "listLength" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(cevalListLength) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "listAppend" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(cevalListAppend) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "listReverse" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(cevalListReverse) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "listHead" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(cevalListFirst) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "listRest" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(cevalListRest) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "listMember" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(cevalListMember) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "anyString" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(cevalAnyString) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "listArrayLiteral" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(cevalListArrayLiteral) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "intBitAnd" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(cevalIntBitAnd) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "intBitOr" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(cevalIntBitOr) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "intBitXor" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(cevalIntBitXor) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "intBitLShift" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(cevalIntBitLShift) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "intBitRShift" if (Config::acceptMetaModelicaGrammar()?) => {
            (std::sync::Arc::new(cevalIntBitRShift) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "numBits" => {
            (std::sync::Arc::new(cevalNumBits) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        Deref @ "integerMax" => {
            (std::sync::Arc::new(cevalIntegerMax) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, Arc<metamodelica::List<Arc<DAE::Exp>>>, bool, Absyn::Msg, i32) -> Result<(FCore::Cache, Arc<Values::Value>)> + 'static>)
        },
        id => {
            let true = (Flags::isSet(Flags::CEVAL.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("No cevalBuiltinHandler found for ")); __mm_s.push_str(&*id.clone()); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(handler)
}

pub fn cevalKnownExternalFuncs(mut inCache: FCore::Cache, mut env: FCore::Graph, mut funcpath: Arc<Absyn::Path>, mut vals: Arc<metamodelica::List<Arc<Values::Value>>>, mut msg: Absyn::Msg) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut cdef: Arc<SCode::Element>;
    let mut env_1: FCore::Graph;
    let mut fid: ArcStr = arcstr::literal!("");
    let mut id: ArcStr = arcstr::literal!("");
    let mut oid: Option<ArcStr> = None;
    let mut extdecl: Option<Arc<SCode::ExternalDecl>> = None;
    let mut lan: Option<ArcStr> = None;
    let mut out: Option<Arc<Absyn::ComponentRef>> = None;
    let mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
    let mut funcRest: SCode::FunctionRestriction = SCode::FunctionRestriction::FR_KERNEL_FUNCTION;
    (outCache, cdef, env_1) = Lookup::lookupClass(inCache.clone(), env.clone(), funcpath.clone(), None)?;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(cdef.clone()) {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { externalDecl: __pa0, .. }, restriction: SCode::Restriction::R_FUNCTION { functionRestriction: __pa1 }, name: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    extdecl = __pa0.clone();
    funcRest = __pa1.clone();
    fid = __pa2.clone();
    let SCode::FR_EXTERNAL_FUNCTION { purity: _ } = (funcRest.clone()) else { bail!("pattern mismatch") };
    let __pa4 = ::match_deref::match_deref! { match &(extdecl.clone()) {
        Some(Deref @ SCode::ExternalDecl { funcName: __pa4, lang: _, output_: _, args: _, annotation_: _ }) => __pa4.clone(),
        _ => bail!("pattern mismatch"),
    } };
    oid = __pa4.clone();
    id = (Util::getOptionOrDefault(oid.clone(), (fid.clone()).clone())).clone();
    isKnownExternalFunc((id.clone()).clone())?;
    res = cevalKnownExternalFuncs2((id.clone()).clone(), vals.clone(), msg.clone())?;
    Ok((outCache, res))
}

pub fn isKnownExternalFunc(mut id: ArcStr) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(id.clone()) {
        Deref @ "acos" => (),
        Deref @ "asin" => (),
        Deref @ "atan" => (),
        Deref @ "atan2" => (),
        Deref @ "cos" => (),
        Deref @ "cosh" => (),
        Deref @ "exp" => (),
        Deref @ "log" => (),
        Deref @ "log10" => (),
        Deref @ "sin" => (),
        Deref @ "sinh" => (),
        Deref @ "tan" => (),
        Deref @ "tanh" => (),
        Deref @ "print" => (),
        Deref @ "ModelicaStreams_closeFile" => (),
        Deref @ "ModelicaStrings_substring" => (),
        Deref @ "ModelicaStrings_length" => (),
        Deref @ "ModelicaInternal_print" => (),
        Deref @ "ModelicaInternal_countLines" => (),
        Deref @ "ModelicaInternal_readLine" => (),
        Deref @ "ModelicaInternal_stat" => (),
        Deref @ "ModelicaInternal_fullPathName" => (),
        Deref @ "ModelicaStrings_compare" => (),
        Deref @ "ModelicaStrings_scanReal" => (),
        Deref @ "ModelicaStrings_skipWhiteSpace" => (),
        Deref @ "ModelicaError" => (),
        Deref @ "OpenModelica_regex" => (),
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn cevalKnownExternalFuncs2(mut id: ArcStr, mut inValuesValueLst: Arc<metamodelica::List<Arc<Values::Value>>>, mut inMsg: Absyn::Msg) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outValue = (::match_deref::match_deref! { match &((id.clone(), inValuesValueLst.clone(), inMsg.clone())) {
        (Deref @ "acos", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: rv }, tail: Deref @ metamodelica::List::Nil }, _) => {
            let mut rv_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let true = (rv.clone() >= metamodelica::OrderedFloat(-1.0_f64) && rv.clone() <= metamodelica::OrderedFloat(1.0_f64)) else { bail!("pattern mismatch") };
            rv_1 = (rv.clone()).acos();
            Arc::new(Values::Value::REAL { real: rv_1.clone() })
        },
        (Deref @ "asin", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: rv }, tail: Deref @ metamodelica::List::Nil }, _) => {
            let mut rv_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let true = (rv.clone() >= metamodelica::OrderedFloat(-1.0_f64) && rv.clone() <= metamodelica::OrderedFloat(1.0_f64)) else { bail!("pattern mismatch") };
            rv_1 = (rv.clone()).asin();
            Arc::new(Values::Value::REAL { real: rv_1.clone() })
        },
        (Deref @ "atan", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: rv }, tail: Deref @ metamodelica::List::Nil }, _) => {
            let mut rv_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            rv_1 = (rv.clone()).atan();
            Arc::new(Values::Value::REAL { real: rv_1.clone() })
        },
        (Deref @ "atan2", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: rv1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: rv2 }, tail: Deref @ metamodelica::List::Nil } }, _) => {
            let mut rv_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            rv_1 = (rv1.clone()).atan2(rv2.clone());
            Arc::new(Values::Value::REAL { real: rv_1.clone() })
        },
        (Deref @ "cos", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: rv }, tail: Deref @ metamodelica::List::Nil }, _) => {
            let mut rv_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            rv_1 = (rv.clone()).cos();
            Arc::new(Values::Value::REAL { real: rv_1.clone() })
        },
        (Deref @ "cosh", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: rv }, tail: Deref @ metamodelica::List::Nil }, _) => {
            let mut rv_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            rv_1 = (rv.clone()).cosh();
            Arc::new(Values::Value::REAL { real: rv_1.clone() })
        },
        (Deref @ "exp", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: rv }, tail: Deref @ metamodelica::List::Nil }, _) => {
            let mut rv_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            rv_1 = (rv.clone()).exp();
            Arc::new(Values::Value::REAL { real: rv_1.clone() })
        },
        (Deref @ "log", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: rv }, tail: Deref @ metamodelica::List::Nil }, _) => {
            let mut rv_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let true = (rv.clone() > metamodelica::OrderedFloat((0) as f64)) else { bail!("pattern mismatch") };
            rv_1 = (rv.clone()).ln();
            Arc::new(Values::Value::REAL { real: rv_1.clone() })
        },
        (Deref @ "log10", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: rv }, tail: Deref @ metamodelica::List::Nil }, _) => {
            let mut rv_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let true = (rv.clone() > metamodelica::OrderedFloat((0) as f64)) else { bail!("pattern mismatch") };
            rv_1 = (rv.clone()).log10();
            Arc::new(Values::Value::REAL { real: rv_1.clone() })
        },
        (Deref @ "sin", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: rv }, tail: Deref @ metamodelica::List::Nil }, _) => {
            let mut rv_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            rv_1 = (rv.clone()).sin();
            Arc::new(Values::Value::REAL { real: rv_1.clone() })
        },
        (Deref @ "sinh", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: rv }, tail: Deref @ metamodelica::List::Nil }, _) => {
            let mut rv_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            rv_1 = (rv.clone()).sinh();
            Arc::new(Values::Value::REAL { real: rv_1.clone() })
        },
        (Deref @ "tan", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: rv }, tail: Deref @ metamodelica::List::Nil }, _) => {
            let mut rv_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            rv_1 = (rv.clone()).tan();
            Arc::new(Values::Value::REAL { real: rv_1.clone() })
        },
        (Deref @ "tanh", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: rv }, tail: Deref @ metamodelica::List::Nil }, _) => {
            let mut rv_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            rv_1 = (rv.clone()).tanh();
            Arc::new(Values::Value::REAL { real: rv_1.clone() })
        },
        (Deref @ "ModelicaStrings_substring", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: start }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: stop }, tail: Deref @ metamodelica::List::Nil } } }, _) => {
            let mut r#str = (*r#str).clone();
            r#str = substring((r#str.clone()).clone(), start.clone(), stop.clone())?;
            Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() })
        },
        (Deref @ "ModelicaStrings_length", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Nil }, _) => {
            let mut i: i32 = 0;
            i = ((r#str.clone()).clone().len() as i32);
            Arc::new(Values::Value::INTEGER { integer: i.clone() })
        },
        (Deref @ "print", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Nil }, _) => {
            println!("{}", (r#str.clone()).clone());
            Arc::new(openmodelica_frontend_types::Values::Value::NORETCALL)
        },
        (Deref @ "OpenModelica_regex", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: re }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: extended }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: insensitive }, tail: Deref @ metamodelica::List::Nil } } } } }, _) => {
            let mut n: i32 = 0;
            let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            (n, strs) = System::regex((r#str.clone()).clone(), (re.clone()).clone(), i.clone(), extended.clone(), insensitive.clone());
            vals = List::map(strs.clone(), (std::sync::Arc::new(fnptr!(ValuesMake::makeString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Values::Value>> + 'static>));
            v = Arc::new(Values::Value::ARRAY { valueLst: vals.clone(), dimLst: list![i.clone()] });
            Arc::new(Values::Value::TUPLE { valueLst: list![Arc::new(Values::Value::INTEGER { integer: n.clone() }), v.clone()] })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outValue)
}

pub static EnumCompareLess: std::sync::LazyLock<Arc<Absyn::Path>> = std::sync::LazyLock::new(|| { Arc::new(Absyn::Path::QUALIFIED { name: (literal!("Modelica")).clone(), path: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("Utilities")).clone(), path: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("Types")).clone(), path: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("Compare")).clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("Less")).clone() }) }) }) }) }) });

pub static EnumCompareEqual: std::sync::LazyLock<Arc<Absyn::Path>> = std::sync::LazyLock::new(|| { Arc::new(Absyn::Path::QUALIFIED { name: (literal!("Modelica")).clone(), path: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("Utilities")).clone(), path: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("Types")).clone(), path: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("Compare")).clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("Equal")).clone() }) }) }) }) }) });

pub static EnumCompareGreater: std::sync::LazyLock<Arc<Absyn::Path>> = std::sync::LazyLock::new(|| { Arc::new(Absyn::Path::QUALIFIED { name: (literal!("Modelica")).clone(), path: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("Utilities")).clone(), path: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("Types")).clone(), path: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("Compare")).clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("Greater")).clone() }) }) }) }) }) });

fn cevalMatrixElt(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inMatrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<Values::Value>>>)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outValues: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut vl: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    for mut expl in &*inMatrix.clone() {
        let mut expl = expl.clone();
        (outCache, vl) = cevalList(outCache.clone(), inEnv.clone(), expl.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())?;
        v = ValuesMake::makeArray(vl.clone())?;
        outValues = cons(v.clone(), outValues.clone());
    }
    outValues = outValues.clone().reverse();
    Ok((outCache, outValues))
}

fn cevalBuiltinSize(mut inCache: FCore::Cache, mut inEnv1: FCore::Graph, mut inExp2: Arc<DAE::Exp>, mut inDimExp: Arc<DAE::Exp>, mut inBoolean4: bool, mut inMsg6: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv1.clone(), inExp2.clone(), inDimExp.clone(), inBoolean4.clone(), inMsg6.clone(), numIter.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Exp::MATRIX { matrix: mat, .. }, Deref @ DAE::Exp::ICONST { integer: 1 }, _, _, _) => {
                    let mut i: i32 = 0;
                    i = (mat.clone().len() as i32);
                    Ok((cache.clone(), Arc::new(Values::Value::INTEGER { integer: i.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Exp::MATRIX { matrix: mat, .. }, Deref @ DAE::Exp::ICONST { integer: 2 }, _, _, _) => {
                    let mut i: i32 = 0;
                    i = (listHead(mat.clone())?.len() as i32);
                    Ok((cache.clone(), Arc::new(Values::Value::INTEGER { integer: i.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::MATRIX { matrix: mat, .. }, Deref @ DAE::Exp::ICONST { integer: dim }, r#impl, msg, _) => {
                    let mut dim_1: i32 = 0;
                    let mut i: i32 = 0;
                    let mut bl: bool = false;
                    let mut e: Arc<DAE::Exp>;
                    let mut cache = (*cache).clone();
                    bl = dim.clone() > 2;
                    let true = (bl.clone()) else { bail!("pattern mismatch") };
                    dim_1 = dim.clone() - 2;
                    e = listHead(listHead(mat.clone())?)?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(cevalBuiltinSize(cache.clone(), env.clone(), e.clone(), Arc::new(DAE::Exp::ICONST { integer: dim_1.clone() }), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    i = __pa1.clone();
                    Ok((cache.clone(), Arc::new(Values::Value::INTEGER { integer: i.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CREF { componentRef: cr, .. }, dimExp, r#impl, msg, _) => {
                    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut sizelst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut dim: i32 = 0;
                    let mut i: i32 = 0;
                    let mut cache = (*cache).clone();
                    (cache, _, tp, _, _, _, _, _, _) = Lookup::lookupVar(cache.clone(), env.clone(), cr.clone())?;
                    let true = (Types::dimensionsKnown(tp.clone())?) else { bail!("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(Types::getDimensionSizes(tp.clone())?) {
                        __pa0 @ Deref @ metamodelica::List::Cons { head: _, tail: _ } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    sizelst = __pa0.clone();
                    let (__pa1, __pa2) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), dimExp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa1, Deref @ Values::Value::INTEGER { integer: __pa2 }) => (__pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa1.clone();
                    dim = __pa2.clone();
                    i = (sizelst.clone()).get(dim.clone())?;
                    Ok((cache.clone(), Arc::new(Values::Value::INTEGER { integer: i.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CREF { componentRef: cr, .. }, dimExp, r#impl @ false, msg, _) => {
                    let mut dimv: i32 = 0;
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut v2: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut ddim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let mut cache = (*cache).clone();
                    (cache, dims) = InstUtil::elabComponentArraydimFromEnv(cache.clone(), env.clone(), cr.clone(), Absyn::dummyInfo.clone())?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), dimExp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    dimv = __pa1.clone();
                    ddim = (dims.clone()).get(dimv.clone())?;
                    (cache, v2) = cevalDimension(cache.clone(), env.clone(), ddim.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
                    Ok((cache.clone(), v2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CREF { componentRef: cr, .. }, dimExp, false, Absyn::Msg::MSG { info }, _) => {
                    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut cr_str: ArcStr = arcstr::literal!("");
                    let mut dim_str: ArcStr = arcstr::literal!("");
                    let mut size_str: ArcStr = arcstr::literal!("");
                    let mut expstr: ArcStr = arcstr::literal!("");
                    (_, _, tp, binding, _, _, _, _, _) = Lookup::lookupVar(cache.clone(), env.clone(), cr.clone())?;
                    if !(Types::dimensionsKnown(tp.clone())?) {
                        cr_str = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
                        dim_str = (ExpressionBasics::printExpStr(dimExp.clone())?).clone();
                        size_str = stringAppendList(list![(literal!("size(")).clone(), (cr_str.clone()).clone(), (literal!(", ")).clone(), (dim_str.clone()).clone(), (literal!(")")).clone()]);
                        Error::addSourceMessage(Error::DIMENSION_NOT_KNOWN.clone(), list![(size_str.clone()).clone()], info.clone())?;
                    } else {
                        let _ = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ DAE::Binding::UNBOUND => {
                    expstr = (ExpressionBasics::printExpStr(inExp2.clone())?).clone();
                    Error::addSourceMessage(Error::UNBOUND_VALUE.clone(), list![(expstr.clone()).clone()], info.clone())?;
                    bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
                    }
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CREF { componentRef: cr, .. }, dimExp, r#impl, msg, _) => {
                    let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut dimv: i32 = 0;
                    let mut v2: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    (cache, _, _, binding, _, _, _, _, _) = Lookup::lookupVar(cache.clone(), env.clone(), cr.clone())?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), dimExp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    dimv = __pa1.clone();
                    (cache, val) = cevalCrefBinding(cache.clone(), env.clone(), cr.clone(), binding.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
                    v2 = cevalBuiltinSize2(val.clone(), dimv.clone())?;
                    Ok((cache.clone(), v2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Cons { head: exp, tail: es }, .. }, dimExp, r#impl, msg, _) => {
                    let mut len: i32 = 0;
                    let mut cache = (*cache).clone();
                    let _ = Expression::r#typeof(exp.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), dimExp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa0, Deref @ Values::Value::INTEGER { integer: 1 }) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    len = (cons(exp.clone(), es.clone()).len() as i32);
                    Ok((cache.clone(), Arc::new(Values::Value::INTEGER { integer: len.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, exp, dimExp, r#impl, msg, _) => {
                    let mut adims: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut dimv: i32 = 0;
                    let mut v2: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    (cache, val) = ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), dimExp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    dimv = __pa1.clone();
                    v2 = (::match_deref::match_deref! { match &(val.clone()) {
        Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::List::Nil, dimLst: adims } => Arc::new(Values::Value::INTEGER { integer: (adims.clone()).get(dimv.clone())? }),
        _ => cevalBuiltinSize2(val.clone(), dimv.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                    Ok((cache.clone(), v2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, exp, _, _, Absyn::Msg::MSG { .. }, _) => {
                    let mut expstr: ArcStr = arcstr::literal!("");
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Print::printErrorBuf((literal!("#-- Ceval.cevalBuiltinSize failed: ")).clone())?;
                    expstr = (ExpressionBasics::printExpStr(exp.clone())?).clone();
                    Print::printErrorBuf((expstr.clone()).clone())?;
                    Print::printErrorBuf((literal!("\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outValue))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn cevalBuiltinSize2(mut inValue: Arc<Values::Value>, mut inInteger: i32) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outValue = 'mc: {
        let __mc_input = (inValue.clone(), inInteger.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::ARRAY { valueLst: lst, .. }, 1) => {
                    let mut dim: i32 = 0;
                    dim = (lst.clone().len() as i32);
                    Ok(Arc::new(Values::Value::INTEGER { integer: dim.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::List::Cons { head: l, tail: _ }, .. }, ind) => {
                    let mut ind_1: i32 = 0;
                    let mut dimVal: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    ind_1 = ind.clone() - 1;
                    dimVal = cevalBuiltinSize2(l.clone(), ind_1.clone())?;
                    Ok(dimVal.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- Ceval.cevalBuiltinSize2 failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outValue)
}

fn cevalBuiltinSize3(mut inDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut inIndex: i32) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut v: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &((inDims.clone()).get(inIndex.clone())?) {
        Deref @ DAE::Dimension::DIM_INTEGER { integer: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    v = __pa0.clone();
    outValue = Arc::new(Values::Value::INTEGER { integer: v.clone() });
    Ok(outValue)
}

fn cevalBuiltinAbs(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
                    let mut rv: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut rv_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    rv = __pa1.clone();
                    rv_1 = realAbs(rv.clone());
                    Ok((cache.clone(), Arc::new(Values::Value::REAL { real: rv_1.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
                    let mut iv: i32 = 0;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    iv = __pa1.clone();
                    iv = intAbs(iv.clone());
                    Ok((cache.clone(), Arc::new(Values::Value::INTEGER { integer: iv.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outValue))
}

fn cevalBuiltinSign(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut rv: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut b1: bool = false;
            let mut b2: bool = false;
            let mut b3: bool = false;
            let mut iv: i32 = 0;
            let mut iv_1: i32 = 0;
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut cache = (*cache).clone();
            (cache, v) = ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
            (b1, b2, b3) = (::match_deref::match_deref! { match &(v.clone()) {
        Deref @ Values::Value::REAL { real: rv } => (rv.clone() > metamodelica::OrderedFloat(0.0_f64), rv.clone() < metamodelica::OrderedFloat(0.0_f64), rv.clone() == metamodelica::OrderedFloat(0.0_f64)),
        Deref @ Values::Value::INTEGER { integer: iv } => (iv.clone() > 0, iv.clone() < 0, iv.clone() == 0),
        _ => bail!("match: no arm matched"),
    } });
            let metamodelica::List::Cons { head: (_, __pa0), tail: __t1 } = (List::select(list![(b1.clone(), 1), (b2.clone(), -1), (b3.clone(), 0)], std::sync::Arc::new(fnptr!(Util::tuple21, _)))) else { bail!("pattern mismatch") };
            let metamodelica::List::Nil = (__t1.clone()) else { bail!("pattern mismatch") };
            iv_1 = __pa0.clone();
            (cache.clone(), Arc::new(Values::Value::INTEGER { integer: iv_1.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinExp(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut rv: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rv_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            rv = __pa1.clone();
            rv_1 = (rv.clone()).exp();
            (cache.clone(), Arc::new(Values::Value::REAL { real: rv_1.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinNoevent(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut cache = (*cache).clone();
            (cache, v) = ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
            (cache.clone(), v.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinCat(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: dim, tail: matrices }, r#impl, msg, _) => {
            let mut dim_int: i32 = 0;
            let mut mat_lst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), dim.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            dim_int = __pa1.clone();
            (cache, mat_lst) = cevalList(cache.clone(), env.clone(), matrices.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
            v = cevalCat(mat_lst.clone(), dim_int.clone())?;
            (cache.clone(), v.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinIdentity(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: dim, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut dimension: i32 = 0;
            let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), dim.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            dimension = __pa1.clone();
            res = Arc::new(Values::Value::ARRAY { valueLst: {
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut j in (1..=dimension.clone()).into_iter() {
            let __x = Arc::new(Values::Value::ARRAY { valueLst: {
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut i in (1..=dimension.clone()).into_iter() {
            let __x = if (i.clone() == j.clone()) {Arc::new(Values::Value::INTEGER { integer: 1 })} else {Arc::new(Values::Value::INTEGER { integer: 0 })};
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, dimLst: list![dimension.clone()] });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, dimLst: list![dimension.clone(), dimension.clone()] });
            (cache.clone(), res.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinPromote(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: arr, tail: Deref @ metamodelica::List::Cons { head: dim, tail: Deref @ metamodelica::List::Nil } }, r#impl, msg, _) => {
            let mut arr_val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut dim_val: i32 = 0;
            let mut dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            let (__pa0, __pa2, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), arr.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, __pa2 @ Deref @ Values::Value::ARRAY { dimLst: __pa1, .. }) => (__pa0.clone(), __pa2.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            dims = __pa1.clone();
            arr_val = __pa2.clone();
            let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), dim.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa3, Deref @ Values::Value::INTEGER { integer: __pa4 }) => (__pa3.clone(), __pa4.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa3.clone();
            dim_val = __pa4.clone();
            res = cevalBuiltinPromote2(arr_val.clone(), dim_val.clone() - (dims.clone().len() as i32))?;
            (cache.clone(), res.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinPromote2(mut inValue: Arc<Values::Value>, mut inInteger: i32) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outValue = 'mc: {
        let __mc_input = (inValue.clone(), inInteger.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, 0) => {
                    Ok(Arc::new(Values::Value::ARRAY { valueLst: list![v.clone()], dimLst: list![1] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::ARRAY { dimLst: Deref @ metamodelica::List::Cons { head: i, tail: _ }, valueLst: vs }, n) => {
                    let mut n_1: i32 = 0;
                    let mut vs_1: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut il: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    n_1 = n.clone() - 1;
                    if vs.clone().is_empty() {
                        vs_1 = vs.clone();
                        il = listRest(var_field!((*inValue).dimLst, Values::Value::ARRAY).clone())?;
                        il = listAppend(List::fill(0, n.clone() - (il.clone().len() as i32)), il.clone());
                    } else {
                        let (__pa1, __pa0) = ::match_deref::match_deref! { match &(List::map1(vs.clone(), (std::sync::Arc::new(cevalBuiltinPromote2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>, i32) -> Result<Arc<Values::Value>> + 'static>), n_1.clone())) {
                            __pa1 @ Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { dimLst: __pa0, .. }, tail: _ } => (__pa1.clone(), __pa0.clone()),
                            _ => bail!("pattern mismatch"),
                        } };
                        il = __pa0.clone();
                        vs_1 = __pa1.clone();
                    }
                    Ok(Arc::new(Values::Value::ARRAY { valueLst: vs_1.clone(), dimLst: cons(i.clone(), il.clone()) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, n) => {
                    let mut n_1: i32 = 0;
                    let mut il: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut v = (*v).clone();
                    if '__try0: {
                        ::match_deref::match_deref! { match &(v.clone()) {
                            Deref @ Values::Value::ARRAY { .. } => (),
                            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                        } };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    n_1 = n.clone() - 1;
                    let (__pa2, __pa1) = ::match_deref::match_deref! { match &(cevalBuiltinPromote2(v.clone(), n_1.clone())?) {
                        __pa2 @ Deref @ Values::Value::ARRAY { dimLst: __pa1, .. } => (__pa2.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    il = __pa1.clone();
                    v = __pa2.clone();
                    Ok(Arc::new(Values::Value::ARRAY { valueLst: list![v.clone()], dimLst: cons(1, il.clone()) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- Ceval.cevalBuiltinPromote2 failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outValue)
}

fn cevalBuiltinSubstring(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: str_exp, tail: Deref @ metamodelica::List::Cons { head: start_exp, tail: Deref @ metamodelica::List::Cons { head: stop_exp, tail: Deref @ metamodelica::List::Nil } } }, r#impl, msg, _) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut start: i32 = 0;
            let mut stop: i32 = 0;
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), str_exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::STRING { string: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            r#str = __pa1.clone();
            let (__pa2, __pa3) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), start_exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa2, Deref @ Values::Value::INTEGER { integer: __pa3 }) => (__pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa2.clone();
            start = __pa3.clone();
            let (__pa4, __pa5) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), stop_exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa4, Deref @ Values::Value::INTEGER { integer: __pa5 }) => (__pa4.clone(), __pa5.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa4.clone();
            stop = __pa5.clone();
            r#str = substring((r#str.clone()).clone(), start.clone(), stop.clone())?;
            (cache.clone(), Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinString(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Cons { head: len_exp, tail: Deref @ metamodelica::List::Cons { head: justified_exp, tail: Deref @ metamodelica::List::Nil } } }, r#impl, msg, _) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut i: i32 = 0;
            let mut b: bool = false;
            let mut p: Arc<Absyn::Path>;
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut cache = (*cache).clone();
            (cache, v) = ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
            r#str = ((::match_deref::match_deref! { match &(v.clone()) {
        Deref @ Values::Value::INTEGER { integer: i } => intString(i.clone()),
        Deref @ Values::Value::BOOL { boolean: b } => boolString(b.clone()),
        Deref @ Values::Value::ENUM_LITERAL { name: p, .. } => AbsynUtil::pathLastIdent(p.clone())?,
        _ => bail!("match: no arm matched"),
    } })).clone();
            (cache, r#str) = cevalBuiltinStringFormat(cache.clone(), env.clone(), (r#str.clone()).clone(), len_exp.clone(), justified_exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
            (cache.clone(), Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }))
        },
        (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Cons { head: sig_dig, tail: Deref @ metamodelica::List::Cons { head: len_exp, tail: Deref @ metamodelica::List::Cons { head: justified_exp, tail: Deref @ metamodelica::List::Nil } } } }, r#impl, msg, _) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut format: ArcStr = arcstr::literal!("");
            let mut len: i32 = 0;
            let mut sig: i32 = 0;
            let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut left_just: bool = false;
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            r = __pa1.clone();
            let (__pa2, __pa3) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), len_exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa2, Deref @ Values::Value::INTEGER { integer: __pa3 }) => (__pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa2.clone();
            len = __pa3.clone();
            let (__pa4, __pa5) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), justified_exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa4, Deref @ Values::Value::BOOL { boolean: __pa5 }) => (__pa4.clone(), __pa5.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa4.clone();
            left_just = __pa5.clone();
            let (__pa6, __pa7) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), sig_dig.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa6, Deref @ Values::Value::INTEGER { integer: __pa7 }) => (__pa6.clone(), __pa7.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa6.clone();
            sig = __pa7.clone();
            format = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("%")); __mm_s.push_str(&*if (left_just.clone()) {literal!("-")} else {literal!("")}); __mm_s.push_str(&*intString(len.clone())); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*intString(sig.clone())); __mm_s.push_str(&*literal!("g")); ArcStr::from(__mm_s) }).clone();
            r#str = (System::snprintff((format.clone()).clone(), len.clone() + 20, r.clone())?).clone();
            (cache.clone(), Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinStringFormat(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inString: ArcStr, mut lengthExp: Arc<DAE::Exp>, mut justifiedExp: Arc<DAE::Exp>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, ArcStr)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outString: ArcStr = arcstr::literal!("");
    (outCache, outString) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inString.clone(), lengthExp.clone(), justifiedExp.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, _, _, _, _, _, _, _) => {
            let mut min_length: i32 = 0;
            let mut left_justified: bool = false;
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), inEnv.clone(), lengthExp.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            min_length = __pa1.clone();
            let (__pa2, __pa3) = ::match_deref::match_deref! { match &(ceval(cache.clone(), inEnv.clone(), justifiedExp.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone() + 1)?) {
                (__pa2, Deref @ Values::Value::BOOL { boolean: __pa3 }) => (__pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa2.clone();
            left_justified = __pa3.clone();
            r#str = (ExpressionSimplify::cevalBuiltinStringFormat((inString.clone()).clone(), ((inString.clone()).clone().len() as i32), min_length.clone(), left_justified.clone())).clone();
            (cache.clone(), r#str.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outString))
}

fn cevalBuiltinPrint(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::STRING { string: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            r#str = __pa1.clone();
            println!("{}", (r#str.clone()).clone());
            (cache.clone(), Arc::new(openmodelica_frontend_types::Values::Value::NORETCALL))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalIntString(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut i: i32 = 0;
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            i = __pa1.clone();
            r#str = (intString(i.clone())).clone();
            (cache.clone(), Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalRealString(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut cache = (*cache).clone();
            (cache, v) = ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
            let __pa0 = ::match_deref::match_deref! { match &(v.clone()) {
                Deref @ Values::Value::REAL { real: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            r = __pa0.clone();
            r#str = (realString(r.clone())).clone();
            (cache.clone(), Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalStringCharInt(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut i: i32 = 0;
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::STRING { string: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            r#str = __pa1.clone();
            i = stringCharInt((r#str.clone()).clone())?;
            (cache.clone(), Arc::new(Values::Value::INTEGER { integer: i.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalIntStringChar(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut i: i32 = 0;
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            i = __pa1.clone();
            r#str = intStringChar(i.clone());
            (cache.clone(), Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalStringInt(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut i: i32 = 0;
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::STRING { string: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            r#str = __pa1.clone();
            i = stringInt((r#str.clone()).clone())?;
            (cache.clone(), Arc::new(Values::Value::INTEGER { integer: i.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalStringLength(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut i: i32 = 0;
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::STRING { string: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            r#str = __pa1.clone();
            i = ((r#str.clone()).clone().len() as i32);
            (cache.clone(), Arc::new(Values::Value::INTEGER { integer: i.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalStringListStringChar(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut chList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut valList: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::STRING { string: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            r#str = __pa1.clone();
            chList = stringListStringChar((r#str.clone()).clone());
            valList = List::map(chList.clone(), (std::sync::Arc::new(fnptr!(generateValueString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Values::Value>> + 'static>));
            (cache.clone(), Arc::new(Values::Value::LIST { valueLst: valList.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn generateValueString(mut r#str: ArcStr) -> Arc<Values::Value> {
    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    val = Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() });
    val
}

fn cevalListStringCharString(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut chList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut valList: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::LIST { valueLst: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            valList = __pa1.clone();
            chList = List::map(valList.clone(), (std::sync::Arc::new(extractValueStringChar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<ArcStr> + 'static>));
            r#str = stringAppendList(chList.clone());
            (cache.clone(), Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalStringAppendList(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut chList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut valList: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::LIST { valueLst: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            valList = __pa1.clone();
            chList = List::map(valList.clone(), (std::sync::Arc::new(ValuesUtil::extractValueString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<ArcStr> + 'static>));
            r#str = stringAppendList(chList.clone());
            (cache.clone(), Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalStringDelimitList(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp1, tail: Deref @ metamodelica::List::Cons { head: exp2, tail: Deref @ metamodelica::List::Nil } }, r#impl, msg, _) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut chList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut valList: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp1.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::LIST { valueLst: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            valList = __pa1.clone();
            let (__pa2, __pa3) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp2.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa2, Deref @ Values::Value::STRING { string: __pa3 }) => (__pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa2.clone();
            r#str = __pa3.clone();
            chList = List::map(valList.clone(), (std::sync::Arc::new(ValuesUtil::extractValueString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<ArcStr> + 'static>));
            r#str = stringDelimitList(chList.clone(), (r#str.clone()).clone());
            (cache.clone(), Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalListLength(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut i: i32 = 0;
            let mut valList: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::LIST { valueLst: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            valList = __pa1.clone();
            i = (valList.clone().len() as i32);
            (cache.clone(), Arc::new(Values::Value::INTEGER { integer: i.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalListAppend(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp1, tail: Deref @ metamodelica::List::Cons { head: exp2, tail: Deref @ metamodelica::List::Nil } }, r#impl, msg, _) => {
            let mut valList: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut valList1: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut valList2: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp1.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::LIST { valueLst: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            valList1 = __pa1.clone();
            let (__pa2, __pa3) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp2.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa2, Deref @ Values::Value::LIST { valueLst: __pa3 }) => (__pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa2.clone();
            valList2 = __pa3.clone();
            valList = listAppend(valList1.clone(), valList2.clone());
            (cache.clone(), Arc::new(Values::Value::LIST { valueLst: valList.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalListReverse(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp1, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut valList: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut valList1: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp1.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::LIST { valueLst: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            valList1 = __pa1.clone();
            valList = valList1.clone().reverse();
            (cache.clone(), Arc::new(Values::Value::LIST { valueLst: valList.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalListRest(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp1, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut valList1: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp1.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::LIST { valueLst: Deref @ metamodelica::List::Cons { head: _, tail: __pa1 } }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            valList1 = __pa1.clone();
            (cache.clone(), Arc::new(Values::Value::LIST { valueLst: valList1.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalListMember(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp1, tail: Deref @ metamodelica::List::Cons { head: exp2, tail: Deref @ metamodelica::List::Nil } }, r#impl, msg, _) => {
            let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut b: bool = false;
            let mut cache = (*cache).clone();
            (cache, val) = ceval(cache.clone(), env.clone(), exp1.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp2.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::LIST { valueLst: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            vals = __pa1.clone();
            b = listMember(val.clone(), vals.clone());
            (cache.clone(), Arc::new(Values::Value::BOOL { boolean: b.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalListArrayLiteral(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::LIST { valueLst: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            vals = __pa1.clone();
            (cache.clone(), Arc::new(Values::Value::META_ARRAY { valueLst: vals.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalAnyString(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp1, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut s: ArcStr = arcstr::literal!("");
            let mut cache = (*cache).clone();
            (cache, v) = ceval(cache.clone(), env.clone(), exp1.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
            s = (ValuesDump::valString(v.clone())?).clone();
            (cache.clone(), Arc::new(Values::Value::STRING { string: (s.clone()).clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalNumBits(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (_, _, Deref @ metamodelica::List::Nil, _, _, _) => {
            let mut i: i32 = 0;
            i = System::numBits();
            (inCache.clone(), Arc::new(Values::Value::INTEGER { integer: i.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalIntegerMax(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (_, _, Deref @ metamodelica::List::Nil, _, _, _) => {
            let mut i: i32 = 0;
            i = System::intMaxLit();
            (inCache.clone(), Arc::new(Values::Value::INTEGER { integer: i.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalIntBitAnd(mut cache: FCore::Cache, mut env: FCore::Graph, mut args: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut r#impl: bool, mut msg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut cache: FCore::Cache = cache;
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut e1: Arc<DAE::Exp>;
    let mut e2: Arc<DAE::Exp>;
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: _ } } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    e1 = __pa0.clone();
    e2 = __pa1.clone();
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), e1.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
        (__pa3, Deref @ Values::Value::INTEGER { integer: __pa4 }) => (__pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    cache = __pa3.clone();
    i1 = __pa4.clone();
    let (__pa5, __pa6) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), e2.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
        (__pa5, Deref @ Values::Value::INTEGER { integer: __pa6 }) => (__pa5.clone(), __pa6.clone()),
        _ => bail!("pattern mismatch"),
    } };
    cache = __pa5.clone();
    i2 = __pa6.clone();
    result = Arc::new(Values::Value::INTEGER { integer: intBitAnd(i1.clone(), i2.clone()) });
    Ok((cache, result))
}

fn cevalIntBitOr(mut cache: FCore::Cache, mut env: FCore::Graph, mut args: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut r#impl: bool, mut msg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut cache: FCore::Cache = cache;
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut e1: Arc<DAE::Exp>;
    let mut e2: Arc<DAE::Exp>;
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: _ } } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    e1 = __pa0.clone();
    e2 = __pa1.clone();
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), e1.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
        (__pa3, Deref @ Values::Value::INTEGER { integer: __pa4 }) => (__pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    cache = __pa3.clone();
    i1 = __pa4.clone();
    let (__pa5, __pa6) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), e2.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
        (__pa5, Deref @ Values::Value::INTEGER { integer: __pa6 }) => (__pa5.clone(), __pa6.clone()),
        _ => bail!("pattern mismatch"),
    } };
    cache = __pa5.clone();
    i2 = __pa6.clone();
    result = Arc::new(Values::Value::INTEGER { integer: intBitOr(i1.clone(), i2.clone()) });
    Ok((cache, result))
}

fn cevalIntBitXor(mut cache: FCore::Cache, mut env: FCore::Graph, mut args: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut r#impl: bool, mut msg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut cache: FCore::Cache = cache;
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut e1: Arc<DAE::Exp>;
    let mut e2: Arc<DAE::Exp>;
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: _ } } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    e1 = __pa0.clone();
    e2 = __pa1.clone();
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), e1.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
        (__pa3, Deref @ Values::Value::INTEGER { integer: __pa4 }) => (__pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    cache = __pa3.clone();
    i1 = __pa4.clone();
    let (__pa5, __pa6) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), e2.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
        (__pa5, Deref @ Values::Value::INTEGER { integer: __pa6 }) => (__pa5.clone(), __pa6.clone()),
        _ => bail!("pattern mismatch"),
    } };
    cache = __pa5.clone();
    i2 = __pa6.clone();
    result = Arc::new(Values::Value::INTEGER { integer: intBitXor(i1.clone(), i2.clone()) });
    Ok((cache, result))
}

fn cevalIntBitLShift(mut cache: FCore::Cache, mut env: FCore::Graph, mut args: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut r#impl: bool, mut msg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut cache: FCore::Cache = cache;
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut e1: Arc<DAE::Exp>;
    let mut e2: Arc<DAE::Exp>;
    let mut i: i32 = 0;
    let mut s: i32 = 0;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: _ } } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    e1 = __pa0.clone();
    e2 = __pa1.clone();
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), e1.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
        (__pa3, Deref @ Values::Value::INTEGER { integer: __pa4 }) => (__pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    cache = __pa3.clone();
    i = __pa4.clone();
    let (__pa5, __pa6) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), e2.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
        (__pa5, Deref @ Values::Value::INTEGER { integer: __pa6 }) => (__pa5.clone(), __pa6.clone()),
        _ => bail!("pattern mismatch"),
    } };
    cache = __pa5.clone();
    s = __pa6.clone();
    result = Arc::new(Values::Value::INTEGER { integer: intBitLShift(i.clone(), s.clone()) });
    Ok((cache, result))
}

fn cevalIntBitRShift(mut cache: FCore::Cache, mut env: FCore::Graph, mut args: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut r#impl: bool, mut msg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut cache: FCore::Cache = cache;
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut e1: Arc<DAE::Exp>;
    let mut e2: Arc<DAE::Exp>;
    let mut i: i32 = 0;
    let mut s: i32 = 0;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: _ } } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    e1 = __pa0.clone();
    e2 = __pa1.clone();
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), e1.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
        (__pa3, Deref @ Values::Value::INTEGER { integer: __pa4 }) => (__pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    cache = __pa3.clone();
    i = __pa4.clone();
    let (__pa5, __pa6) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), e2.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
        (__pa5, Deref @ Values::Value::INTEGER { integer: __pa6 }) => (__pa5.clone(), __pa6.clone()),
        _ => bail!("pattern mismatch"),
    } };
    cache = __pa5.clone();
    s = __pa6.clone();
    result = Arc::new(Values::Value::INTEGER { integer: intBitRShift(i.clone(), s.clone()) });
    Ok((cache, result))
}

fn makeLoadLibrariesEntry(mut cl: Arc<SCode::Element>, mut acc: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<metamodelica::List<Arc<Values::Value>>>> {
    let mut out: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    out = (::match_deref::match_deref! { match &((cl.clone(), acc.clone())) {
        (Deref @ SCode::Element::CLASS { info: SourceInfo { fileName: Deref @ "<interactive>", .. }, .. }, _) => {
            acc.clone()
        },
        (Deref @ SCode::Element::CLASS { info: SourceInfo { fileName, .. }, name, .. }, _) => {
            let mut dir: ArcStr = arcstr::literal!("");
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut b: bool = false;
            let mut fileName = (*fileName).clone();
            dir = (System::dirname((fileName.clone()).clone())).clone();
            fileName = (System::basename((fileName.clone()).clone())).clone();
            v = ValuesMake::makeArray(list![Arc::new(Values::Value::STRING { string: (name.clone()).clone() }), Arc::new(Values::Value::STRING { string: (dir.clone()).clone() })])?;
            b = stringEq((fileName.clone()).clone(), (literal!("ModelicaBuiltin.mo")).clone()) || stringEq((fileName.clone()).clone(), (literal!("MetaModelicaBuiltin.mo")).clone()) || stringEq((dir.clone()).clone(), (literal!(".")).clone());
            List::consOnTrue(!(b.clone()), v.clone(), acc.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out)
}

fn cevalListFirst(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp1, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp1.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::LIST { valueLst: Deref @ metamodelica::List::Cons { head: __pa1, tail: _ } }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            v = __pa1.clone();
            (cache.clone(), ValuesUtil::boxIfUnboxedVal(v.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn extractValueStringChar(mut val: Arc<Values::Value>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(val.clone()) {
        Deref @ Values::Value::STRING { string: r#str } => {
            let 1 = (((r#str.clone()).clone().len() as i32)) else { bail!("pattern mismatch") };
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(r#str)
}

fn cevalCat(mut v_lst: Arc<metamodelica::List<Arc<Values::Value>>>, mut dim: i32) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut v_lst_1: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    v_lst_1 = catDimension(v_lst.clone(), dim.clone())?;
    outValue = ValuesMake::makeArray(v_lst_1.clone())?;
    Ok(outValue)
}

fn catDimension(mut inValuesValueLst: Arc<metamodelica::List<Arc<Values::Value>>>, mut inInteger: i32) -> Result<Arc<metamodelica::List<Arc<Values::Value>>>> {
    let mut outValuesValueLst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    outValuesValueLst = 'mc: {
        let __mc_input = (inValuesValueLst.clone(), inInteger.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vlst, 1) => {
                    let mut vlst_lst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Values::Value>>>>> = metamodelica::nil();
                    let mut v_lst_1: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    vlst_lst = List::map(vlst.clone(), (std::sync::Arc::new(ValuesUtil::arrayValues) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<Arc<metamodelica::List<Arc<Values::Value>>>> + 'static>));
                    v_lst_1 = List::flatten(vlst_lst.clone());
                    Ok(v_lst_1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vlst, dim) => {
                    let mut v_lst_lst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Values::Value>>>>> = metamodelica::nil();
                    let mut v_lst_lst_1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Values::Value>>>>> = metamodelica::nil();
                    let mut v_lst_1: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut dim_1: i32 = 0;
                    let mut i1: i32 = 0;
                    let mut i2: i32 = 0;
                    let mut il: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    v_lst_lst = List::map(vlst.clone(), (std::sync::Arc::new(ValuesUtil::arrayValues) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<Arc<metamodelica::List<Arc<Values::Value>>>> + 'static>));
                    dim_1 = dim.clone() - 1;
                    v_lst_lst_1 = catDimension2(v_lst_lst.clone(), dim_1.clone())?;
                    v_lst_1 = List::map(v_lst_lst_1.clone(), (std::sync::Arc::new(ValuesMake::makeArray) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<Values::Value>> + 'static>));
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(v_lst_1.clone()) {
                        Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { dimLst: Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 }, .. }, tail: _ } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    i2 = __pa0.clone();
                    il = __pa1.clone();
                    i1 = (v_lst_1.clone().len() as i32);
                    v_lst_1 = cevalBuiltinTranspose2(v_lst_1.clone(), 1, cons(i2.clone(), cons(i1.clone(), il.clone())))?;
                    Ok(v_lst_1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outValuesValueLst)
}

fn catDimension2(mut inValuesValueLstLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Values::Value>>>>>, mut inInteger: i32) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<Arc<Values::Value>>>>>> {
    let mut outValuesValueLstLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Values::Value>>>>> = metamodelica::nil();
    outValuesValueLstLst = 'mc: {
        let __mc_input = (inValuesValueLstLst.clone(), inInteger.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (lst, dim) => {
                    let mut l_lst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut first_lst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut first_lst_1: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut first_lst_2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Values::Value>>>>> = metamodelica::nil();
                    l_lst = listHead(lst.clone())?;
                    let 1 = ((l_lst.clone().len() as i32)) else { bail!("pattern mismatch") };
                    first_lst = List::map(lst.clone(), Arc::new(listHead.clone()));
                    first_lst_1 = catDimension(first_lst.clone(), dim.clone())?;
                    first_lst_2 = List::map(first_lst_1.clone(), std::sync::Arc::new(fnptr!(List::create, _)));
                    Ok(first_lst_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (lst, dim) => {
                    let mut first_lst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut first_lst_1: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut rest: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Values::Value>>>>> = metamodelica::nil();
                    let mut rest_1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Values::Value>>>>> = metamodelica::nil();
                    let mut res: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Values::Value>>>>> = metamodelica::nil();
                    first_lst = List::map(lst.clone(), Arc::new(listHead.clone()));
                    rest = List::map(lst.clone(), Arc::new(listRest.clone()));
                    first_lst_1 = catDimension(first_lst.clone(), dim.clone())?;
                    rest_1 = catDimension2(rest.clone(), dim.clone())?;
                    res = List::threadMap(rest_1.clone(), first_lst_1.clone(), std::sync::Arc::new(fnptr!(List::consr, _, _)));
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outValuesValueLstLst)
}

fn cevalBuiltinFloor(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut rv: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rv_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            rv = __pa1.clone();
            rv_1 = (rv.clone()).floor();
            (cache.clone(), Arc::new(Values::Value::REAL { real: rv_1.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinCeil(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut rv: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rv_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rvt: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut realRet: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut ri: i32 = 0;
            let mut ri_1: i32 = 0;
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            rv = __pa1.clone();
            rv_1 = (rv.clone()).floor();
            ri = ((rv_1.clone()).0 as i32);
            rvt = intReal(ri.clone());
            ri_1 = ri.clone() + 1;
            realRet = intReal(ri_1.clone());
            v = if (rvt.clone() == rv.clone()) {Arc::new(Values::Value::REAL { real: rvt.clone() })} else {Arc::new(Values::Value::REAL { real: realRet.clone() })};
            (cache.clone(), v.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinSqrt(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut rv: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rv_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            rv = __pa1.clone();
            if rv.clone() < metamodelica::OrderedFloat(0.0_f64) {
                let Absyn::MSG { info: __pa2 } = (msg.clone()) else { bail!("pattern mismatch") };
                info = __pa2.clone();
                Error::addSourceMessage(Error::NEGATIVE_SQRT.clone(), metamodelica::nil(), info.clone())?;
                bail!("fail");
            } else {
                rv_1 = (rv.clone()).sqrt();
            }
            (cache.clone(), Arc::new(Values::Value::REAL { real: rv_1.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinSin(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut rv: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rv_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            rv = __pa1.clone();
            rv_1 = (rv.clone()).sin();
            (cache.clone(), Arc::new(Values::Value::REAL { real: rv_1.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinSinh(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut rv: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rv_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            rv = __pa1.clone();
            rv_1 = (rv.clone()).sinh();
            (cache.clone(), Arc::new(Values::Value::REAL { real: rv_1.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinCos(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut rv: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rv_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            rv = __pa1.clone();
            rv_1 = (rv.clone()).cos();
            (cache.clone(), Arc::new(Values::Value::REAL { real: rv_1.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinCosh(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut rv: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rv_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            rv = __pa1.clone();
            rv_1 = (rv.clone()).cosh();
            (cache.clone(), Arc::new(Values::Value::REAL { real: rv_1.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinLog(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut rv: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rv_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            rv = __pa1.clone();
            let true = (rv.clone() > metamodelica::OrderedFloat((0) as f64)) else { bail!("pattern mismatch") };
            rv_1 = (rv.clone()).ln();
            (cache.clone(), Arc::new(Values::Value::REAL { real: rv_1.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinLog10(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut rv: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rv_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            rv = __pa1.clone();
            let true = (rv.clone() > metamodelica::OrderedFloat((0) as f64)) else { bail!("pattern mismatch") };
            rv_1 = (rv.clone()).log10();
            (cache.clone(), Arc::new(Values::Value::REAL { real: rv_1.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinTan(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut r#impl: bool, mut msg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }) => {
            let mut rv: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rv_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            rv = __pa1.clone();
            rv_1 = (rv.clone()).tan();
            (cache.clone(), Arc::new(Values::Value::REAL { real: rv_1.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinTanh(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut rv: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rv_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            rv = __pa1.clone();
            rv_1 = (rv.clone()).tanh();
            (cache.clone(), Arc::new(Values::Value::REAL { real: rv_1.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinAsin(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut rv: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rv_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            rv = __pa1.clone();
            let true = (rv.clone() >= metamodelica::OrderedFloat(-1.0_f64) && rv.clone() <= metamodelica::OrderedFloat(1.0_f64)) else { bail!("pattern mismatch") };
            rv_1 = (rv.clone()).asin();
            (cache.clone(), Arc::new(Values::Value::REAL { real: rv_1.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinAcos(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut rv: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rv_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            rv = __pa1.clone();
            let true = (rv.clone() >= metamodelica::OrderedFloat(-1.0_f64) && rv.clone() <= metamodelica::OrderedFloat(1.0_f64)) else { bail!("pattern mismatch") };
            rv_1 = (rv.clone()).acos();
            (cache.clone(), Arc::new(Values::Value::REAL { real: rv_1.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinAtan(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut rv: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rv_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            rv = __pa1.clone();
            rv_1 = (rv.clone()).atan();
            (cache.clone(), Arc::new(Values::Value::REAL { real: rv_1.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinAtan2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp1, tail: Deref @ metamodelica::List::Cons { head: exp2, tail: Deref @ metamodelica::List::Nil } }, r#impl, msg, _) => {
            let mut rv: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rv_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rv_2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp1.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            rv_1 = __pa1.clone();
            let (__pa2, __pa3) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp2.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa2, Deref @ Values::Value::REAL { real: __pa3 }) => (__pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa2.clone();
            rv_2 = __pa3.clone();
            rv = (rv_1.clone()).atan2(rv_2.clone());
            (cache.clone(), Arc::new(Values::Value::REAL { real: rv.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinDiv(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: exp1, tail: Deref @ metamodelica::List::Cons { head: exp2, tail: Deref @ metamodelica::List::Nil } }, r#impl, msg, _) => {
                    let mut rv1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut rv2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut rv_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut rv_2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut b: bool = false;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp1.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    rv1 = __pa1.clone();
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp2.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa2, Deref @ Values::Value::REAL { real: __pa3 }) => (__pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa2.clone();
                    rv2 = __pa3.clone();
                    rv_1 = rv1.clone() / rv2.clone();
                    b = rv_1.clone() < metamodelica::OrderedFloat(0.0_f64);
                    rv_2 = if (b.clone()) {(rv_1.clone()).ceil()} else {(rv_1.clone()).floor()};
                    Ok((cache.clone(), Arc::new(Values::Value::REAL { real: rv_2.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: exp1, tail: Deref @ metamodelica::List::Cons { head: exp2, tail: Deref @ metamodelica::List::Nil } }, r#impl, msg, _) => {
                    let mut rv1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut rv2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut rv_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut rv_2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut ri: i32 = 0;
                    let mut b: bool = false;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp1.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    ri = __pa1.clone();
                    rv1 = intReal(ri.clone());
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp2.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa2, Deref @ Values::Value::REAL { real: __pa3 }) => (__pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa2.clone();
                    rv2 = __pa3.clone();
                    Error::addInternalError((literal!("cevalBuiltinDiv got Integer and Real (type error)\n")).clone(), metamodelica::sourceInfo!())?;
                    rv_1 = rv1.clone() / rv2.clone();
                    b = rv_1.clone() < metamodelica::OrderedFloat(0.0_f64);
                    rv_2 = if (b.clone()) {(rv_1.clone()).ceil()} else {(rv_1.clone()).floor()};
                    Ok((cache.clone(), Arc::new(Values::Value::REAL { real: rv_2.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: exp1, tail: Deref @ metamodelica::List::Cons { head: exp2, tail: Deref @ metamodelica::List::Nil } }, r#impl, msg, _) => {
                    let mut rv1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut rv2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut rv_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut rv_2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut ri: i32 = 0;
                    let mut b: bool = false;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp1.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    rv1 = __pa1.clone();
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp2.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa2, Deref @ Values::Value::INTEGER { integer: __pa3 }) => (__pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa2.clone();
                    ri = __pa3.clone();
                    Error::addInternalError((literal!("cevalBuiltinDiv got Real and Integer (type error)\n")).clone(), metamodelica::sourceInfo!())?;
                    rv2 = intReal(ri.clone());
                    rv_1 = rv1.clone() / rv2.clone();
                    b = rv_1.clone() < metamodelica::OrderedFloat(0.0_f64);
                    rv_2 = if (b.clone()) {(rv_1.clone()).ceil()} else {(rv_1.clone()).floor()};
                    Ok((cache.clone(), Arc::new(Values::Value::REAL { real: rv_2.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: exp1, tail: Deref @ metamodelica::List::Cons { head: exp2, tail: Deref @ metamodelica::List::Nil } }, r#impl, msg, _) => {
                    let mut ri_1: i32 = 0;
                    let mut ri1: i32 = 0;
                    let mut ri2: i32 = 0;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp1.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    ri1 = __pa1.clone();
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp2.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa2, Deref @ Values::Value::INTEGER { integer: __pa3 }) => (__pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa2.clone();
                    ri2 = __pa3.clone();
                    ri_1 = intDiv(ri1.clone(), ri2.clone());
                    Ok((cache.clone(), Arc::new(Values::Value::INTEGER { integer: ri_1.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: exp1, tail: Deref @ metamodelica::List::Cons { head: exp2, tail: Deref @ metamodelica::List::Nil } }, r#impl, Absyn::Msg::MSG { info }, _) => {
                    let mut rv2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut exp1_str: ArcStr = arcstr::literal!("");
                    let mut exp2_str: ArcStr = arcstr::literal!("");
                    let __pa0 = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp2.clone(), r#impl.clone(), inMsg.clone(), numIter.clone() + 1)?) {
                        (_, Deref @ Values::Value::REAL { real: __pa0 }) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    rv2 = __pa0.clone();
                    let true = (rv2.clone() == metamodelica::OrderedFloat(0.0_f64)) else { bail!("pattern mismatch") };
                    exp1_str = (ExpressionBasics::printExpStr(exp1.clone())?).clone();
                    exp2_str = (ExpressionBasics::printExpStr(exp2.clone())?).clone();
                    Error::addSourceMessage(Error::DIVISION_BY_ZERO.clone(), list![(exp1_str.clone()).clone(), (exp2_str.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: exp2, tail: Deref @ metamodelica::List::Nil } }, r#impl, Absyn::Msg::NO_MSG, _) => {
                    let mut rv2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let __pa0 = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp2.clone(), r#impl.clone(), openmodelica_ast::Absyn::Msg::NO_MSG, numIter.clone() + 1)?) {
                        (_, Deref @ Values::Value::REAL { real: __pa0 }) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    rv2 = __pa0.clone();
                    let true = (rv2.clone() == metamodelica::OrderedFloat(0.0_f64)) else { bail!("pattern mismatch") };
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: exp1, tail: Deref @ metamodelica::List::Cons { head: exp2, tail: Deref @ metamodelica::List::Nil } }, r#impl, Absyn::Msg::MSG { info }, _) => {
                    let mut ri2: i32 = 0;
                    let mut lh_str: ArcStr = arcstr::literal!("");
                    let mut rh_str: ArcStr = arcstr::literal!("");
                    let __pa0 = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp2.clone(), r#impl.clone(), inMsg.clone(), numIter.clone() + 1)?) {
                        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    ri2 = __pa0.clone();
                    let true = (ri2.clone() == 0) else { bail!("pattern mismatch") };
                    lh_str = (ExpressionBasics::printExpStr(exp1.clone())?).clone();
                    rh_str = (ExpressionBasics::printExpStr(exp2.clone())?).clone();
                    Error::addSourceMessage(Error::DIVISION_BY_ZERO.clone(), list![(lh_str.clone()).clone(), (rh_str.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: exp2, tail: Deref @ metamodelica::List::Nil } }, r#impl, Absyn::Msg::NO_MSG, _) => {
                    let mut ri2: i32 = 0;
                    let __pa0 = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp2.clone(), r#impl.clone(), openmodelica_ast::Absyn::Msg::NO_MSG, numIter.clone() + 1)?) {
                        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    ri2 = __pa0.clone();
                    let true = (ri2.clone() == 0) else { bail!("pattern mismatch") };
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outValue))
}

fn cevalBuiltinMod(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut r#impl: bool, mut msg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut cache: FCore::Cache = inCache.clone();
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut v1: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut v2: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut exp1: Arc<DAE::Exp>;
    let mut exp2: Arc<DAE::Exp>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExpExpLst.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    exp1 = __pa0.clone();
    exp2 = __pa1.clone();
    (cache, v1) = ceval(cache.clone(), inEnv.clone(), exp1.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
    (cache, v2) = ceval(cache.clone(), inEnv.clone(), exp2.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
    outValue = (::match_deref::match_deref! { match &((v1.clone(), v2.clone(), msg.clone())) {
        (Deref @ Values::Value::REAL { real: rv1 }, Deref @ Values::Value::REAL { real: rv2 }, _) => {
            Arc::new(Values::Value::REAL { real: realMod(rv1.clone(), rv2.clone()) })
        },
        (Deref @ Values::Value::INTEGER { integer: ri }, Deref @ Values::Value::REAL { real: rv2 }, _) => {
            Arc::new(Values::Value::REAL { real: realMod(ri.clone(), rv2.clone()) })
        },
        (Deref @ Values::Value::REAL { real: rv1 }, Deref @ Values::Value::INTEGER { integer: ri }, _) => {
            Arc::new(Values::Value::REAL { real: realMod(rv1.clone(), ri.clone()) })
        },
        (Deref @ Values::Value::INTEGER { integer: ri1 }, Deref @ Values::Value::INTEGER { integer: ri2 }, _) => {
            Arc::new(Values::Value::INTEGER { integer: intMod(ri1.clone(), ri2.clone()) })
        },
        (_, Deref @ Values::Value::REAL { real: rv2 }, Absyn::Msg::MSG { info }) if (rv2.clone() == metamodelica::OrderedFloat(0.0_f64)) => {
            let mut lhs_str: ArcStr = arcstr::literal!("");
            let mut rhs_str: ArcStr = arcstr::literal!("");
            lhs_str = (ExpressionBasics::printExpStr(exp1.clone())?).clone();
            rhs_str = (ExpressionBasics::printExpStr(exp2.clone())?).clone();
            Error::addSourceMessage(Error::MODULO_BY_ZERO.clone(), list![(lhs_str.clone()).clone(), (rhs_str.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        (_, Deref @ Values::Value::INTEGER { integer: 0 }, Absyn::Msg::MSG { info }) => {
            let mut lhs_str: ArcStr = arcstr::literal!("");
            let mut rhs_str: ArcStr = arcstr::literal!("");
            lhs_str = (ExpressionBasics::printExpStr(exp1.clone())?).clone();
            rhs_str = (ExpressionBasics::printExpStr(exp2.clone())?).clone();
            Error::addSourceMessage(Error::MODULO_BY_ZERO.clone(), list![(lhs_str.clone()).clone(), (rhs_str.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((cache, outValue))
}

fn cevalBuiltinSum(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: arr, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), arr.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, .. }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            vals = __pa1.clone();
            if Types::isInteger(Expression::r#typeof(Expression::unboxExp(arr.clone()))?) {
                if vals.clone().is_empty() {
                    v = Arc::new(Values::Value::INTEGER { integer: 0 });
                } else {
                    let __pa2 = ::match_deref::match_deref! { match &(ValuesUtil::sumArrayelt(vals.clone())?) {
                        __pa2 @ Deref @ Values::Value::INTEGER { .. } => __pa2.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    v = __pa2.clone();
                }
            } else {
                if vals.clone().is_empty() {
                    v = Arc::new(Values::Value::REAL { real: metamodelica::OrderedFloat(0.0_f64) });
                } else {
                    let __pa3 = ::match_deref::match_deref! { match &(ValuesUtil::sumArrayelt(vals.clone())?) {
                        __pa3 @ Deref @ Values::Value::REAL { .. } => __pa3.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    v = __pa3.clone();
                }
            }
            (cache.clone(), v.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinMax(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: arr, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut v_1: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut cache = (*cache).clone();
            (cache, v) = ceval(cache.clone(), env.clone(), arr.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
            v_1 = cevalBuiltinMaxArr(v.clone())?;
            (cache.clone(), v_1.clone())
        },
        (cache, env, Deref @ metamodelica::List::Cons { head: s1, tail: Deref @ metamodelica::List::Cons { head: s2, tail: Deref @ metamodelica::List::Nil } }, r#impl, msg, _) => {
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut v1: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut v2: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut cache = (*cache).clone();
            (cache, v1) = ceval(cache.clone(), env.clone(), s1.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
            (cache, v2) = ceval(cache.clone(), env.clone(), s2.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
            v = cevalBuiltinMax2(v1.clone(), v2.clone())?;
            (cache.clone(), v.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinMax2(mut v1: Arc<Values::Value>, mut v2: Arc<Values::Value>) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outValue = (::match_deref::match_deref! { match &((v1.clone(), v2.clone())) {
        (Deref @ Values::Value::INTEGER { integer: i1 }, Deref @ Values::Value::INTEGER { integer: i2 }) => {
            Arc::new(Values::Value::INTEGER { integer: std::cmp::max(i1.clone(), i2.clone()) })
        },
        (Deref @ Values::Value::REAL { real: r1 }, Deref @ Values::Value::REAL { real: r2 }) => {
            Arc::new(Values::Value::REAL { real: std::cmp::max(r1.clone(), r2.clone()) })
        },
        (Deref @ Values::Value::BOOL { boolean: b1 }, Deref @ Values::Value::BOOL { boolean: b2 }) => {
            Arc::new(Values::Value::BOOL { boolean: b1.clone() || b2.clone() })
        },
        (Deref @ Values::Value::ENUM_LITERAL { .. }, Deref @ Values::Value::ENUM_LITERAL { .. }) => {
            if (var_field!((*v1).index, Values::Value::ENUM_LITERAL).clone() > var_field!((*v2).index, Values::Value::ENUM_LITERAL).clone()) {v1.clone()} else {v2.clone()}
        },
        _ => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            s1 = (ValuesDump::valString(v1.clone())?).clone();
            s2 = (ValuesDump::valString(v2.clone())?).clone();
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Ceval.cevalBuiltinMin2 failed: min(")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outValue)
}

fn cevalBuiltinMaxArr(mut inValue: Arc<Values::Value>) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(inValue.clone()) {
        Deref @ Values::Value::ARRAY { valueLst: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    vals = __pa0.clone();
    outValue = todo!("reduction cevalBuiltinMax2: cannot resolve default value");
    Ok(outValue)
}

fn cevalBuiltinMin(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: arr, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut v_1: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut cache = (*cache).clone();
            (cache, v) = ceval(cache.clone(), env.clone(), arr.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
            v_1 = cevalBuiltinMinArr(v.clone())?;
            (cache.clone(), v_1.clone())
        },
        (cache, env, Deref @ metamodelica::List::Cons { head: s1, tail: Deref @ metamodelica::List::Cons { head: s2, tail: Deref @ metamodelica::List::Nil } }, r#impl, msg, _) => {
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut v1: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut v2: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut cache = (*cache).clone();
            (cache, v1) = ceval(cache.clone(), env.clone(), s1.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
            (cache, v2) = ceval(cache.clone(), env.clone(), s2.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
            v = cevalBuiltinMin2(v1.clone(), v2.clone())?;
            (cache.clone(), v.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinMin2(mut v1: Arc<Values::Value>, mut v2: Arc<Values::Value>) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outValue = (::match_deref::match_deref! { match &((v1.clone(), v2.clone())) {
        (Deref @ Values::Value::INTEGER { integer: i1 }, Deref @ Values::Value::INTEGER { integer: i2 }) => {
            Arc::new(Values::Value::INTEGER { integer: std::cmp::min(i1.clone(), i2.clone()) })
        },
        (Deref @ Values::Value::REAL { real: r1 }, Deref @ Values::Value::REAL { real: r2 }) => {
            Arc::new(Values::Value::REAL { real: std::cmp::min(r1.clone(), r2.clone()) })
        },
        (Deref @ Values::Value::BOOL { boolean: b1 }, Deref @ Values::Value::BOOL { boolean: b2 }) => {
            Arc::new(Values::Value::BOOL { boolean: b1.clone() && b2.clone() })
        },
        (Deref @ Values::Value::ENUM_LITERAL { .. }, Deref @ Values::Value::ENUM_LITERAL { .. }) => {
            if (var_field!((*v1).index, Values::Value::ENUM_LITERAL).clone() < var_field!((*v2).index, Values::Value::ENUM_LITERAL).clone()) {v1.clone()} else {v2.clone()}
        },
        _ => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            s1 = (ValuesDump::valString(v1.clone())?).clone();
            s2 = (ValuesDump::valString(v2.clone())?).clone();
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Ceval.cevalBuiltinMin2 failed: min(")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outValue)
}

fn cevalBuiltinMinArr(mut inValue: Arc<Values::Value>) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(inValue.clone()) {
        Deref @ Values::Value::ARRAY { valueLst: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    vals = __pa0.clone();
    outValue = todo!("reduction cevalBuiltinMin2: cannot resolve default value");
    Ok(outValue)
}

fn cevalBuiltinRem(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: exp1, tail: Deref @ metamodelica::List::Cons { head: exp2, tail: Deref @ metamodelica::List::Nil } }, r#impl, msg, _) => {
                    let mut rv1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut rv2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut rvd: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut dr: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp1.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    rv1 = __pa1.clone();
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp2.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa2, Deref @ Values::Value::REAL { real: __pa3 }) => (__pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa2.clone();
                    rv2 = __pa3.clone();
                    let (__pa4, __pa5) = ::match_deref::match_deref! { match &(cevalBuiltinDiv(cache.clone(), env.clone(), list![exp1.clone(), exp2.clone()], r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa4, Deref @ Values::Value::REAL { real: __pa5 }) => (__pa4.clone(), __pa5.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa4.clone();
                    dr = __pa5.clone();
                    rvd = rv1.clone() - rv2.clone() * dr.clone();
                    Ok((cache.clone(), Arc::new(Values::Value::REAL { real: rvd.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: exp1, tail: Deref @ metamodelica::List::Cons { head: exp2, tail: Deref @ metamodelica::List::Nil } }, r#impl, msg, _) => {
                    let mut rv1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut rv2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut rvd: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut dr: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut ri: i32 = 0;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp1.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    ri = __pa1.clone();
                    rv1 = intReal(ri.clone());
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp2.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa2, Deref @ Values::Value::REAL { real: __pa3 }) => (__pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa2.clone();
                    rv2 = __pa3.clone();
                    let (__pa4, __pa5) = ::match_deref::match_deref! { match &(cevalBuiltinDiv(cache.clone(), env.clone(), list![exp1.clone(), exp2.clone()], r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa4, Deref @ Values::Value::REAL { real: __pa5 }) => (__pa4.clone(), __pa5.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa4.clone();
                    dr = __pa5.clone();
                    rvd = rv1.clone() - rv2.clone() * dr.clone();
                    Ok((cache.clone(), Arc::new(Values::Value::REAL { real: rvd.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: exp1, tail: Deref @ metamodelica::List::Cons { head: exp2, tail: Deref @ metamodelica::List::Nil } }, r#impl, msg, _) => {
                    let mut rv1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut rv2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut rvd: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut dr: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut ri: i32 = 0;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp1.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    rv1 = __pa1.clone();
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp2.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa2, Deref @ Values::Value::INTEGER { integer: __pa3 }) => (__pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa2.clone();
                    ri = __pa3.clone();
                    rv2 = intReal(ri.clone());
                    let (__pa4, __pa5) = ::match_deref::match_deref! { match &(cevalBuiltinDiv(cache.clone(), env.clone(), list![exp1.clone(), exp2.clone()], r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa4, Deref @ Values::Value::REAL { real: __pa5 }) => (__pa4.clone(), __pa5.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa4.clone();
                    dr = __pa5.clone();
                    rvd = rv1.clone() - rv2.clone() * dr.clone();
                    Ok((cache.clone(), Arc::new(Values::Value::REAL { real: rvd.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: exp1, tail: Deref @ metamodelica::List::Cons { head: exp2, tail: Deref @ metamodelica::List::Nil } }, r#impl, msg, _) => {
                    let mut ri1: i32 = 0;
                    let mut ri2: i32 = 0;
                    let mut ri_1: i32 = 0;
                    let mut di: i32 = 0;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp1.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    ri1 = __pa1.clone();
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp2.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa2, Deref @ Values::Value::INTEGER { integer: __pa3 }) => (__pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa2.clone();
                    ri2 = __pa3.clone();
                    let (__pa4, __pa5) = ::match_deref::match_deref! { match &(cevalBuiltinDiv(cache.clone(), env.clone(), list![exp1.clone(), exp2.clone()], r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa4, Deref @ Values::Value::INTEGER { integer: __pa5 }) => (__pa4.clone(), __pa5.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa4.clone();
                    di = __pa5.clone();
                    ri_1 = ri1.clone() - ri2.clone() * di.clone();
                    Ok((cache.clone(), Arc::new(Values::Value::INTEGER { integer: ri_1.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: exp1, tail: Deref @ metamodelica::List::Cons { head: exp2, tail: Deref @ metamodelica::List::Nil } }, r#impl, Absyn::Msg::MSG { info }, _) => {
                    let mut rv2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut exp1_str: ArcStr = arcstr::literal!("");
                    let mut exp2_str: ArcStr = arcstr::literal!("");
                    let __pa0 = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp2.clone(), r#impl.clone(), inMsg.clone(), numIter.clone() + 1)?) {
                        (_, Deref @ Values::Value::REAL { real: __pa0 }) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    rv2 = __pa0.clone();
                    let true = (rv2.clone() == metamodelica::OrderedFloat(0.0_f64)) else { bail!("pattern mismatch") };
                    exp1_str = (ExpressionBasics::printExpStr(exp1.clone())?).clone();
                    exp2_str = (ExpressionBasics::printExpStr(exp2.clone())?).clone();
                    Error::addSourceMessage(Error::REM_ARG_ZERO.clone(), list![(exp1_str.clone()).clone(), (exp2_str.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: exp1, tail: Deref @ metamodelica::List::Cons { head: exp2, tail: Deref @ metamodelica::List::Nil } }, r#impl, Absyn::Msg::MSG { info }, _) => {
                    let mut ri2: i32 = 0;
                    let mut exp1_str: ArcStr = arcstr::literal!("");
                    let mut exp2_str: ArcStr = arcstr::literal!("");
                    let __pa0 = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp2.clone(), r#impl.clone(), inMsg.clone(), numIter.clone() + 1)?) {
                        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    ri2 = __pa0.clone();
                    let true = (ri2.clone() == 0) else { bail!("pattern mismatch") };
                    exp1_str = (ExpressionBasics::printExpStr(exp1.clone())?).clone();
                    exp2_str = (ExpressionBasics::printExpStr(exp2.clone())?).clone();
                    Error::addSourceMessage(Error::REM_ARG_ZERO.clone(), list![(exp1_str.clone()).clone(), (exp2_str.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outValue))
}

fn cevalBuiltinInteger(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut rv: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut ri: i32 = 0;
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            rv = __pa1.clone();
            ri = ((rv.clone()).0 as i32);
            (cache.clone(), Arc::new(Values::Value::INTEGER { integer: ri.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinBoolean(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut rv: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut iv: i32 = 0;
            let mut bv: bool = false;
            let mut b: bool = false;
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut cache = (*cache).clone();
            (cache, v) = ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
            b = (::match_deref::match_deref! { match &(v.clone()) {
        Deref @ Values::Value::REAL { real: rv } => !(realEq(rv.clone(), metamodelica::OrderedFloat(0.0_f64))),
        Deref @ Values::Value::INTEGER { integer: iv } => !(intEq(iv.clone(), 0)),
        Deref @ Values::Value::BOOL { boolean: bv } => bv.clone(),
        _ => bail!("match: no arm matched"),
    } });
            (cache.clone(), Arc::new(Values::Value::BOOL { boolean: b.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinRooted(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut cache = (*cache).clone();
            (cache, _) = ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
            (cache.clone(), Arc::new(Values::Value::BOOL { boolean: true }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinIntegerEnumeration(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
            let mut ri: i32 = 0;
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::ENUM_LITERAL { index: __pa1, .. }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            ri = __pa1.clone();
            (cache.clone(), Arc::new(Values::Value::INTEGER { integer: ri.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinDiagonal(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, r#impl, msg, _) => {
                    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut dimension: i32 = 0;
                    let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut zero: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut cache = (*cache).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::r#typeof(exp.clone())?) {
                        Deref @ DAE::Type::T_ARRAY { ty: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    ty = __pa0.clone();
                    let (__pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa1, Deref @ Values::Value::ARRAY { valueLst: __pa2, dimLst: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Nil } }) => (__pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa1.clone();
                    vals = __pa2.clone();
                    dimension = __pa3.clone();
                    zero = ValuesMake::makeZero(ty.clone())?;
                    res = Arc::new(Values::Value::ARRAY { valueLst: {
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut j in (1..=dimension.clone()).into_iter() {
                    let __x = Arc::new(Values::Value::ARRAY { valueLst: {
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut i in (1..=dimension.clone()).into_iter() {
                    let __x = if (i.clone() == j.clone()) {(vals.clone()).get(i.clone())?} else {zero.clone()};
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, dimLst: list![dimension.clone()] });
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, dimLst: list![dimension.clone(), dimension.clone()] });
                    Ok((cache.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Absyn::Msg::MSG { info }, _) => {
                    Error::addSourceMessage(Error::COMPILER_ERROR.clone(), list![(literal!("Could not evaluate diagonal. Ceval.cevalBuiltinDiagonal failed.")).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outValue))
}

fn cevalBuiltinCross(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inExpExpLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: xe, tail: Deref @ metamodelica::List::Cons { head: ye, tail: Deref @ metamodelica::List::Nil } }, r#impl, msg, _) => {
                    let mut xv: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut yv: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), xe.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, dimLst: Deref @ metamodelica::List::Cons { head: 3, tail: Deref @ metamodelica::List::Nil } }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    xv = __pa1.clone();
                    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), ye.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa3, Deref @ Values::Value::ARRAY { valueLst: __pa4, dimLst: Deref @ metamodelica::List::Cons { head: 3, tail: Deref @ metamodelica::List::Nil } }) => (__pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa3.clone();
                    yv = __pa4.clone();
                    res = ValuesUtil::crossProduct(xv.clone(), yv.clone())?;
                    Ok((cache.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Absyn::Msg::MSG { info }, _) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("cross")); __mm_s.push_str(&*ExpressionBasics::printExpStr(Arc::new(DAE::Exp::TUPLE { PR: inExpExpLst.clone() }))?); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::FAILED_TO_EVALUATE_EXPRESSION.clone(), list![(r#str.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outValue))
}

fn cevalBuiltinTranspose2(mut inValuesValueLst1: Arc<metamodelica::List<Arc<Values::Value>>>, mut inInteger2: i32, mut inDims: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<Arc<Values::Value>>>> {
    let mut outValuesValueLst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    outValuesValueLst = 'mc: {
        let __mc_input = (inValuesValueLst1.clone(), inInteger2.clone(), inDims.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vlst, indx, Deref @ metamodelica::List::Cons { head: dim1, tail: _ }) => {
                    if !((indx.clone() <= dim1.clone())) { bail!("guard") }
                    let mut transposed_row: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut rest: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut indx_1: i32 = 0;
                    transposed_row = List::map1(vlst.clone(), (std::sync::Arc::new(ValuesUtil::nthArrayelt) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>, i32) -> Result<Arc<Values::Value>> + 'static>), indx.clone());
                    indx_1 = indx.clone() + 1;
                    rest = cevalBuiltinTranspose2(vlst.clone(), indx_1.clone(), inDims.clone())?;
                    Ok(cons(Arc::new(Values::Value::ARRAY { valueLst: transposed_row.clone(), dimLst: inDims.clone() }), rest.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outValuesValueLst)
}

fn cevalBuiltinSizeMatrix(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<DAE::Exp>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inExp.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CREF { componentRef: cr, .. }, _, _, _) => {
                    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut sizelst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    (cache, _, tp, _, _, _, _, _, _) = Lookup::lookupVar(cache.clone(), env.clone(), cr.clone())?;
                    sizelst = Types::getDimensionSizes(tp.clone())?;
                    v = ValuesUtil::intlistToValue(sizelst.clone())?;
                    Ok((cache.clone(), v.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Exp::MATRIX { ty: Deref @ DAE::Type::T_ARRAY { dims, .. }, .. }, _, _, _) => {
                    let mut sizelst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    sizelst = List::map(dims.clone(), (std::sync::Arc::new(Expression::dimensionSize) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Dimension>) -> Result<i32> + 'static>));
                    v = ValuesUtil::intlistToValue(sizelst.clone())?;
                    Ok((cache.clone(), v.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, exp, r#impl, msg, _) => {
                    let mut sizelst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                        (__pa0, Deref @ Values::Value::ARRAY { dimLst: __pa1, .. }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    sizelst = __pa1.clone();
                    v = ValuesUtil::intlistToValue(sizelst.clone())?;
                    Ok((cache.clone(), v.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outValue))
}

fn cevalBuiltinFail(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inImpl: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> (FCore::Cache, Arc<Values::Value>) {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outCache = inCache.clone();
    outValue = Arc::new(openmodelica_frontend_types::Values::Value::META_FAIL);
    (outCache, outValue)
}

fn cevalBuiltinFill(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inImpl: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpl.clone(), inImpl.clone(), inMsg.clone(), numIter.clone())) {
        (cache, _, Deref @ metamodelica::List::Cons { head: fill_exp, tail: dims }, _, _, _) => {
            let mut fill_val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut cache = (*cache).clone();
            (cache, fill_val) = ceval(cache.clone(), inEnv.clone(), fill_exp.clone(), inImpl.clone(), inMsg.clone(), numIter.clone() + 1)?;
            (cache, fill_val) = cevalBuiltinFill2(cache.clone(), inEnv.clone(), fill_val.clone(), dims.clone(), inImpl.clone(), inMsg.clone(), numIter.clone())?;
            (cache.clone(), fill_val.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalBuiltinFill2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inFillValue: Arc<Values::Value>, mut inDims: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inImpl: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inFillValue.clone(), inDims.clone(), inImpl.clone(), inMsg.clone(), numIter.clone())) {
        (cache, _, _, Deref @ metamodelica::List::Nil, _, _, _) => {
            (cache.clone(), inFillValue.clone())
        },
        (cache, _, _, Deref @ metamodelica::List::Cons { head: dim, tail: rest_dims }, _, _, _) => {
            let mut int_dim: i32 = 0;
            let mut array_dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut fill_value: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut fill_vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            (cache, fill_value) = cevalBuiltinFill2(cache.clone(), inEnv.clone(), inFillValue.clone(), rest_dims.clone(), inImpl.clone(), inMsg.clone(), numIter.clone())?;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), inEnv.clone(), dim.clone(), inImpl.clone(), inMsg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            int_dim = __pa1.clone();
            fill_vals = List::fill(fill_value.clone(), int_dim.clone());
            array_dims = ValuesUtil::valueDimensions(fill_value.clone());
            array_dims = cons(int_dim.clone(), array_dims.clone());
            (cache.clone(), Arc::new(Values::Value::ARRAY { valueLst: fill_vals.clone(), dimLst: array_dims.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalRelation(mut inValue1: Arc<Values::Value>, mut inOperator: DAE::Operator, mut inValue2: Arc<Values::Value>) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut result: bool = false;
    result = 'mc: {
        let __mc_input = inOperator.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Operator::GREATER { .. } = __mc_input.clone() else { bail!("nomatch") };
            Ok(cevalRelationLess(inValue2.clone(), inValue1.clone())?)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Operator::LESS { .. } = __mc_input.clone() else { bail!("nomatch") };
            Ok(cevalRelationLess(inValue1.clone(), inValue2.clone())?)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Operator::LESSEQ { .. } = __mc_input.clone() else { bail!("nomatch") };
            Ok(cevalRelationLessEq(inValue1.clone(), inValue2.clone())?)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Operator::GREATEREQ { .. } = __mc_input.clone() else { bail!("nomatch") };
            Ok(cevalRelationGreaterEq(inValue1.clone(), inValue2.clone())?)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Operator::EQUAL { .. } = __mc_input.clone() else { bail!("nomatch") };
            Ok(cevalRelationEqual(inValue1.clone(), inValue2.clone())?)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Operator::NEQUAL { .. } = __mc_input.clone() else { bail!("nomatch") };
            Ok(cevalRelationNotEqual(inValue1.clone(), inValue2.clone())?)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Ceval.cevalRelation failed on: ")); __mm_s.push_str(&*ValuesDump::printValStr(inValue1.clone())?); __mm_s.push_str(&*ExpressionDump::relopSymbol(inOperator.clone())?); __mm_s.push_str(&*ValuesDump::printValStr(inValue2.clone())?); ArcStr::from(__mm_s) }).clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    outValue = Arc::new(Values::Value::BOOL { boolean: result.clone() });
    Ok(outValue)
}

fn cevalRelationLess(mut inValue1: Arc<Values::Value>, mut inValue2: Arc<Values::Value>) -> Result<bool> {
    let mut result: bool = false;
    result = (::match_deref::match_deref! { match &((inValue1.clone(), inValue2.clone())) {
        (Deref @ Values::Value::STRING { .. }, Deref @ Values::Value::STRING { .. }) => stringCompare((var_field!((*inValue1).string, Values::Value::STRING).clone()).clone(), (var_field!((*inValue2).string, Values::Value::STRING).clone()).clone()) < 0,
        (Deref @ Values::Value::BOOL { .. }, Deref @ Values::Value::BOOL { .. }) => var_field!((*inValue1).boolean, Values::Value::BOOL).clone() < var_field!((*inValue2).boolean, Values::Value::BOOL).clone(),
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::INTEGER { .. }) => var_field!((*inValue1).integer, Values::Value::INTEGER).clone() < var_field!((*inValue2).integer, Values::Value::INTEGER).clone(),
        (Deref @ Values::Value::REAL { .. }, Deref @ Values::Value::REAL { .. }) => var_field!((*inValue1).real, Values::Value::REAL).clone() < var_field!((*inValue2).real, Values::Value::REAL).clone(),
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::REAL { .. }) => intReal(var_field!((*inValue1).integer, Values::Value::INTEGER).clone()) < var_field!((*inValue2).real, Values::Value::REAL).clone(),
        (Deref @ Values::Value::REAL { .. }, Deref @ Values::Value::INTEGER { .. }) => var_field!((*inValue1).real, Values::Value::REAL).clone() < intReal(var_field!((*inValue2).integer, Values::Value::INTEGER).clone()),
        (Deref @ Values::Value::ENUM_LITERAL { .. }, Deref @ Values::Value::ENUM_LITERAL { .. }) => var_field!((*inValue1).index, Values::Value::ENUM_LITERAL).clone() < var_field!((*inValue2).index, Values::Value::ENUM_LITERAL).clone(),
        (Deref @ Values::Value::ENUM_LITERAL { .. }, Deref @ Values::Value::INTEGER { .. }) => var_field!((*inValue1).index, Values::Value::ENUM_LITERAL).clone() < var_field!((*inValue2).integer, Values::Value::INTEGER).clone(),
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::ENUM_LITERAL { .. }) => var_field!((*inValue1).integer, Values::Value::INTEGER).clone() < var_field!((*inValue2).index, Values::Value::ENUM_LITERAL).clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(result)
}

fn cevalRelationLessEq(mut inValue1: Arc<Values::Value>, mut inValue2: Arc<Values::Value>) -> Result<bool> {
    let mut result: bool = false;
    result = (::match_deref::match_deref! { match &((inValue1.clone(), inValue2.clone())) {
        (Deref @ Values::Value::STRING { .. }, Deref @ Values::Value::STRING { .. }) => stringCompare((var_field!((*inValue1).string, Values::Value::STRING).clone()).clone(), (var_field!((*inValue2).string, Values::Value::STRING).clone()).clone()) <= 0,
        (Deref @ Values::Value::BOOL { .. }, Deref @ Values::Value::BOOL { .. }) => var_field!((*inValue1).boolean, Values::Value::BOOL).clone() <= var_field!((*inValue2).boolean, Values::Value::BOOL).clone(),
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::INTEGER { .. }) => var_field!((*inValue1).integer, Values::Value::INTEGER).clone() <= var_field!((*inValue2).integer, Values::Value::INTEGER).clone(),
        (Deref @ Values::Value::REAL { .. }, Deref @ Values::Value::REAL { .. }) => var_field!((*inValue1).real, Values::Value::REAL).clone() <= var_field!((*inValue2).real, Values::Value::REAL).clone(),
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::REAL { .. }) => intReal(var_field!((*inValue1).integer, Values::Value::INTEGER).clone()) <= var_field!((*inValue2).real, Values::Value::REAL).clone(),
        (Deref @ Values::Value::REAL { .. }, Deref @ Values::Value::INTEGER { .. }) => var_field!((*inValue1).real, Values::Value::REAL).clone() <= intReal(var_field!((*inValue2).integer, Values::Value::INTEGER).clone()),
        (Deref @ Values::Value::ENUM_LITERAL { .. }, Deref @ Values::Value::ENUM_LITERAL { .. }) => var_field!((*inValue1).index, Values::Value::ENUM_LITERAL).clone() <= var_field!((*inValue2).index, Values::Value::ENUM_LITERAL).clone(),
        (Deref @ Values::Value::ENUM_LITERAL { .. }, Deref @ Values::Value::INTEGER { .. }) => var_field!((*inValue1).index, Values::Value::ENUM_LITERAL).clone() <= var_field!((*inValue2).integer, Values::Value::INTEGER).clone(),
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::ENUM_LITERAL { .. }) => var_field!((*inValue1).integer, Values::Value::INTEGER).clone() <= var_field!((*inValue2).index, Values::Value::ENUM_LITERAL).clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(result)
}

fn cevalRelationGreaterEq(mut inValue1: Arc<Values::Value>, mut inValue2: Arc<Values::Value>) -> Result<bool> {
    let mut result: bool = false;
    result = (::match_deref::match_deref! { match &((inValue1.clone(), inValue2.clone())) {
        (Deref @ Values::Value::STRING { .. }, Deref @ Values::Value::STRING { .. }) => stringCompare((var_field!((*inValue1).string, Values::Value::STRING).clone()).clone(), (var_field!((*inValue2).string, Values::Value::STRING).clone()).clone()) >= 0,
        (Deref @ Values::Value::BOOL { .. }, Deref @ Values::Value::BOOL { .. }) => var_field!((*inValue1).boolean, Values::Value::BOOL).clone() >= var_field!((*inValue2).boolean, Values::Value::BOOL).clone(),
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::INTEGER { .. }) => var_field!((*inValue1).integer, Values::Value::INTEGER).clone() >= var_field!((*inValue2).integer, Values::Value::INTEGER).clone(),
        (Deref @ Values::Value::REAL { .. }, Deref @ Values::Value::REAL { .. }) => var_field!((*inValue1).real, Values::Value::REAL).clone() >= var_field!((*inValue2).real, Values::Value::REAL).clone(),
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::REAL { .. }) => intReal(var_field!((*inValue1).integer, Values::Value::INTEGER).clone()) >= var_field!((*inValue2).real, Values::Value::REAL).clone(),
        (Deref @ Values::Value::REAL { .. }, Deref @ Values::Value::INTEGER { .. }) => var_field!((*inValue1).real, Values::Value::REAL).clone() >= intReal(var_field!((*inValue2).integer, Values::Value::INTEGER).clone()),
        (Deref @ Values::Value::ENUM_LITERAL { .. }, Deref @ Values::Value::ENUM_LITERAL { .. }) => var_field!((*inValue1).index, Values::Value::ENUM_LITERAL).clone() >= var_field!((*inValue2).index, Values::Value::ENUM_LITERAL).clone(),
        (Deref @ Values::Value::ENUM_LITERAL { .. }, Deref @ Values::Value::INTEGER { .. }) => var_field!((*inValue1).index, Values::Value::ENUM_LITERAL).clone() >= var_field!((*inValue2).integer, Values::Value::INTEGER).clone(),
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::ENUM_LITERAL { .. }) => var_field!((*inValue1).integer, Values::Value::INTEGER).clone() >= var_field!((*inValue2).index, Values::Value::ENUM_LITERAL).clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(result)
}

fn cevalRelationEqual(mut inValue1: Arc<Values::Value>, mut inValue2: Arc<Values::Value>) -> Result<bool> {
    let mut result: bool = false;
    result = (::match_deref::match_deref! { match &((inValue1.clone(), inValue2.clone())) {
        (Deref @ Values::Value::STRING { .. }, Deref @ Values::Value::STRING { .. }) => stringCompare((var_field!((*inValue1).string, Values::Value::STRING).clone()).clone(), (var_field!((*inValue2).string, Values::Value::STRING).clone()).clone()) == 0,
        (Deref @ Values::Value::BOOL { .. }, Deref @ Values::Value::BOOL { .. }) => var_field!((*inValue1).boolean, Values::Value::BOOL).clone() == var_field!((*inValue2).boolean, Values::Value::BOOL).clone(),
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::INTEGER { .. }) => var_field!((*inValue1).integer, Values::Value::INTEGER).clone() == var_field!((*inValue2).integer, Values::Value::INTEGER).clone(),
        (Deref @ Values::Value::REAL { .. }, Deref @ Values::Value::REAL { .. }) => var_field!((*inValue1).real, Values::Value::REAL).clone() == var_field!((*inValue2).real, Values::Value::REAL).clone(),
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::REAL { .. }) => intReal(var_field!((*inValue1).integer, Values::Value::INTEGER).clone()) == var_field!((*inValue2).real, Values::Value::REAL).clone(),
        (Deref @ Values::Value::REAL { .. }, Deref @ Values::Value::INTEGER { .. }) => var_field!((*inValue1).real, Values::Value::REAL).clone() == intReal(var_field!((*inValue2).integer, Values::Value::INTEGER).clone()),
        (Deref @ Values::Value::ENUM_LITERAL { .. }, Deref @ Values::Value::ENUM_LITERAL { .. }) => var_field!((*inValue1).index, Values::Value::ENUM_LITERAL).clone() == var_field!((*inValue2).index, Values::Value::ENUM_LITERAL).clone(),
        (Deref @ Values::Value::ENUM_LITERAL { .. }, Deref @ Values::Value::INTEGER { .. }) => var_field!((*inValue1).index, Values::Value::ENUM_LITERAL).clone() == var_field!((*inValue2).integer, Values::Value::INTEGER).clone(),
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::ENUM_LITERAL { .. }) => var_field!((*inValue1).integer, Values::Value::INTEGER).clone() == var_field!((*inValue2).index, Values::Value::ENUM_LITERAL).clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(result)
}

fn cevalRelationNotEqual(mut inValue1: Arc<Values::Value>, mut inValue2: Arc<Values::Value>) -> Result<bool> {
    let mut result: bool = false;
    result = (::match_deref::match_deref! { match &((inValue1.clone(), inValue2.clone())) {
        (Deref @ Values::Value::STRING { .. }, Deref @ Values::Value::STRING { .. }) => stringCompare((var_field!((*inValue1).string, Values::Value::STRING).clone()).clone(), (var_field!((*inValue2).string, Values::Value::STRING).clone()).clone()) != 0,
        (Deref @ Values::Value::BOOL { .. }, Deref @ Values::Value::BOOL { .. }) => var_field!((*inValue1).boolean, Values::Value::BOOL).clone() != var_field!((*inValue2).boolean, Values::Value::BOOL).clone(),
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::INTEGER { .. }) => var_field!((*inValue1).integer, Values::Value::INTEGER).clone() != var_field!((*inValue2).integer, Values::Value::INTEGER).clone(),
        (Deref @ Values::Value::REAL { .. }, Deref @ Values::Value::REAL { .. }) => var_field!((*inValue1).real, Values::Value::REAL).clone() != var_field!((*inValue2).real, Values::Value::REAL).clone(),
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::REAL { .. }) => intReal(var_field!((*inValue1).integer, Values::Value::INTEGER).clone()) != var_field!((*inValue2).real, Values::Value::REAL).clone(),
        (Deref @ Values::Value::REAL { .. }, Deref @ Values::Value::INTEGER { .. }) => var_field!((*inValue1).real, Values::Value::REAL).clone() != intReal(var_field!((*inValue2).integer, Values::Value::INTEGER).clone()),
        (Deref @ Values::Value::ENUM_LITERAL { .. }, Deref @ Values::Value::ENUM_LITERAL { .. }) => var_field!((*inValue1).index, Values::Value::ENUM_LITERAL).clone() != var_field!((*inValue2).index, Values::Value::ENUM_LITERAL).clone(),
        (Deref @ Values::Value::ENUM_LITERAL { .. }, Deref @ Values::Value::INTEGER { .. }) => var_field!((*inValue1).index, Values::Value::ENUM_LITERAL).clone() != var_field!((*inValue2).integer, Values::Value::INTEGER).clone(),
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::ENUM_LITERAL { .. }) => var_field!((*inValue1).integer, Values::Value::INTEGER).clone() != var_field!((*inValue2).index, Values::Value::ENUM_LITERAL).clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(result)
}

fn cevalRange(mut cache: FCore::Cache, mut env: FCore::Graph, mut rangeExp: Arc<DAE::Exp>, mut r#impl: bool, mut msg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut start: Arc<DAE::Exp>;
    let mut step: Option<Arc<DAE::Exp>> = None;
    let mut stop: Arc<DAE::Exp>;
    let mut range_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut vstart: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut vstop: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut istep: i32 = 0;
    let mut rstep: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut arr: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(rangeExp.clone()) {
        Deref @ DAE::Exp::RANGE { stop: __pa0, step: __pa1, start: __pa2, ty: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    stop = __pa0.clone();
    step = __pa1.clone();
    start = __pa2.clone();
    range_ty = __pa3.clone();
    (outCache, vstart) = ceval(cache.clone(), env.clone(), start.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
    (outCache, vstop) = ceval(outCache.clone(), env.clone(), stop.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
    arr = (::match_deref::match_deref! { match &((vstart.clone(), vstop.clone())) {
        (Deref @ Values::Value::BOOL { .. }, Deref @ Values::Value::BOOL { .. }) => {
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut b in (ExpressionSimplify::simplifyRangeBool(var_field!((*vstart).boolean, Values::Value::BOOL).clone(), var_field!((*vstop).boolean, Values::Value::BOOL).clone())).into_iter().cloned() {
            let __x = ValuesMake::makeBoolean(b.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::INTEGER { .. }) => {
            if isSome(step.clone()) {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(outCache.clone(), env.clone(), Util::getOption(step.clone())?, r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                    (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                outCache = __pa0.clone();
                istep = __pa1.clone();
            } else {
                istep = 1;
            }
            {
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut i in (ExpressionSimplify::simplifyRange(var_field!((*vstart).integer, Values::Value::INTEGER).clone(), istep.clone(), var_field!((*vstop).integer, Values::Value::INTEGER).clone())?).into_iter().cloned() {
            let __x = ValuesMake::makeInteger(i.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
        },
        (Deref @ Values::Value::ENUM_LITERAL { .. }, Deref @ Values::Value::ENUM_LITERAL { .. }) => cevalRangeEnum(var_field!((*vstart).index, Values::Value::ENUM_LITERAL).clone(), var_field!((*vstop).index, Values::Value::ENUM_LITERAL).clone(), Types::arrayElementType(range_ty.clone()))?,
        (Deref @ Values::Value::REAL { .. }, Deref @ Values::Value::REAL { .. }) => {
            if isSome(step.clone()) {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(outCache.clone(), env.clone(), Util::getOption(step.clone())?, r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                    (__pa0, Deref @ Values::Value::REAL { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                outCache = __pa0.clone();
                rstep = __pa1.clone();
            } else {
                rstep = metamodelica::OrderedFloat(1.0_f64);
            }
            {
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut r in (ExpressionSimplify::simplifyRangeReal(var_field!((*vstart).real, Values::Value::REAL).clone(), rstep.clone(), var_field!((*vstop).real, Values::Value::REAL).clone())?).into_iter().cloned() {
            let __x = ValuesMake::makeReal(r.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
        },
        _ => bail!("match: no arm matched"),
    } });
    outValue = ValuesMake::makeArray(arr.clone())?;
    Ok((outCache, outValue))
}

pub fn cevalRangeEnum(mut startIndex: i32, mut stopIndex: i32, mut enumType: Arc<DAE::Type>) -> Result<Arc<metamodelica::List<Arc<Values::Value>>>> {
    let mut enumValList: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    enumValList = (::match_deref::match_deref! { match &((startIndex.clone(), stopIndex.clone(), enumType.clone())) {
        (_, _, Deref @ DAE::Type::T_ENUMERATION { names: enum_names, path: enum_type, .. }) if (startIndex.clone() <= stopIndex.clone()) => {
            let mut enum_paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
            let mut enum_values: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut enum_names = (*enum_names).clone();
            enum_names = List::sublist(enum_names.clone(), startIndex.clone(), stopIndex.clone() - startIndex.clone() + 1)?;
            enum_paths = List::map(enum_names.clone(), (std::sync::Arc::new(fnptr!(AbsynUtil::makeIdentPathFromString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Absyn::Path>> + 'static>));
            enum_paths = List::map1r(enum_paths.clone(), (std::sync::Arc::new(AbsynUtil::joinPaths) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> + 'static>), enum_type.clone());
            (enum_values, _) = List::mapFold(enum_paths.clone(), (std::sync::Arc::new(fnptr!(makeEnumValue, Arc<Absyn::Path>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, i32) -> Result<(Arc<Values::Value>, i32)> + 'static>), startIndex.clone());
            enum_values.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(enumValList)
}

fn makeEnumValue(mut name: Arc<Absyn::Path>, mut index: i32) -> (Arc<Values::Value>, i32) {
    let mut enumValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut newIndex: i32 = 0;
    enumValue = Arc::new(Values::Value::ENUM_LITERAL { name: name.clone(), index: index.clone() });
    newIndex = index.clone() + 1;
    (enumValue, newIndex)
}

pub fn cevalList(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<Values::Value>>>)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outValuesValueLst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    let mut expLstNew: Arc<metamodelica::List<Arc<DAE::Exp>>> = inExpExpLst.clone();
    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    for mut exp in &*expLstNew.clone() {
        let mut exp = exp.clone();
        (outCache, v) = ceval(outCache.clone(), inEnv.clone(), exp.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone() + 1)?;
        outValuesValueLst = cons(v.clone(), outValuesValueLst.clone());
    }
    outValuesValueLst = outValuesValueLst.clone().reverse();
    Ok((outCache, outValuesValueLst))
}

pub fn cevalCref(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inComponentRef: Arc<DAE::ComponentRef>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inComponentRef.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, c, r#impl, msg, _) => {
                    let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut classEnv: FCore::Graph;
                    let mut componentEnv: FCore::Graph;
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut const_for_range: Option<DAE::Const> = None;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut attr: Arc<DAE::Attributes>;
                    let mut splicedExpData: InstTypes::SplicedExpData;
                    let mut cache = (*cache).clone();
                    (cache, attr, ty, binding, const_for_range, splicedExpData, classEnv, componentEnv, name) = Lookup::lookupVar(cache.clone(), env.clone(), c.clone())?;
                    (cache, v) = cevalCref_dispatch(cache.clone(), env.clone(), c.clone(), attr.clone(), ty.clone(), binding.clone(), const_for_range.clone(), splicedExpData.clone(), classEnv.clone(), componentEnv.clone(), (name.clone()).clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
                    Ok((cache.clone(), v.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, c, false, Absyn::Msg::MSG { info }, _) => {
                    let mut scope_str: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    if '__try0: {
                        (_, _, _, _, _, _, _, _, _) = unwrap_break_err!(Lookup::lookupVar(cache.clone(), env.clone(), c.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    scope_str = (FGraph::printGraphPathStr(env.clone())?).clone();
                    r#str = (ComponentReferenceBasics::printComponentRefStr(c.clone())?).clone();
                    Error::addSourceMessage(Error::LOOKUP_VARIABLE_ERROR.clone(), list![(r#str.clone()).clone(), (scope_str.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outValue))
}

pub fn cevalCref_dispatch(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inCref: Arc<DAE::ComponentRef>, mut inAttr: Arc<DAE::Attributes>, mut inType: Arc<DAE::Type>, mut inBinding: Arc<DAE::Binding>, mut constForRange: Option<DAE::Const>, mut inSplicedExpData: InstTypes::SplicedExpData, mut inClassEnv: FCore::Graph, mut inComponentEnv: FCore::Graph, mut inFQName: ArcStr, mut inImpl: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inCref.clone(), inAttr.clone(), inType.clone(), inBinding.clone(), constForRange.clone(), inSplicedExpData.clone(), inClassEnv.clone(), inComponentEnv.clone(), inFQName.clone(), inImpl.clone(), inMsg.clone(), numIter.clone())) {
        (_, _, _, _, _, Deref @ DAE::Binding::UNBOUND, Some(_), _, _, _, _, _, _, _) => {
            bail!("fail")
        },
        (_, _, _, _, _, Deref @ DAE::Binding::UNBOUND, None, _, _, _, _, false, Absyn::Msg::MSG { .. }, _) => {
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut scope_str: ArcStr = arcstr::literal!("");
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut s3: ArcStr = arcstr::literal!("");
            r#str = (ComponentReferenceBasics::printComponentRefStr(inCref.clone())?).clone();
            scope_str = (FGraph::printGraphPathStr(inEnv.clone())?).clone();
            if Flags::isSet(Flags::CEVAL.clone())? {
                Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Ceval.cevalCref on: ")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" failed with no constant binding in scope: ")); __mm_s.push_str(&*scope_str.clone()); ArcStr::from(__mm_s) }).clone())?;
            }
            s1 = (FGraph::printGraphPathStr(inEnv.clone())?).clone();
            s2 = (ComponentReferenceBasics::printComponentRefStr(inCref.clone())?).clone();
            s3 = (TypesDump::printTypeStr(inType.clone())?).clone();
            v = Types::typeToValue(inType.clone())?;
            v = Arc::new(Values::Value::EMPTY { scope: (s1.clone()).clone(), name: (s2.clone()).clone(), ty: v.clone(), tyStr: (s3.clone()).clone() });
            (inCache.clone(), v.clone())
        },
        (_, _, _, Deref @ DAE::Attributes { variability, .. }, _, _, _, _, _, _, _, _, _, _) => {
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let true = (SCodeUtil::isParameterOrConst(variability.clone()) || inImpl.clone() || FGraph::inForLoopScope(inEnv.clone())?) else { bail!("pattern mismatch") };
            let false = (crefEqualValue(inCref.clone(), inBinding.clone())?) else { bail!("pattern mismatch") };
            (cache, v) = cevalCrefBinding(inCache.clone(), inEnv.clone(), inCref.clone(), inBinding.clone(), inImpl.clone(), inMsg.clone(), numIter.clone())?;
            cache = FCore::addEvaluatedCref(cache.clone(), variability.clone(), ComponentReferenceBasics::crefStripLastSubs(inCref.clone())?);
            (cache.clone(), v.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

pub fn cevalCrefBinding(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inComponentRef: Arc<DAE::ComponentRef>, mut inBinding: Arc<DAE::Binding>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inComponentRef.clone(), inBinding.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cr, Deref @ DAE::Binding::VALBOUND { valBound: v, .. }, r#impl, msg, _) => {
                    let mut subsc: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
                    let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    subsc = ComponentReference::crefLastSubs(cr.clone())?;
                    (cache, res) = cevalSubscriptValue(cache.clone(), env.clone(), subsc.clone(), v.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
                    Ok((cache.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, identType: ty, subscriptLst: Deref @ metamodelica::List::Nil }, Deref @ DAE::Binding::UNBOUND, _, Absyn::Msg::MSG { info }, _) => {
                    let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut vl: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut tpath: Arc<Absyn::Path>;
                    let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Types::arrayElementType(ty.clone())) {
                        Deref @ DAE::Type::T_COMPLEX { varLst: __pa0, complexClassType: ClassInf::State::RECORD { path: __pa1 }, .. } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    vl = __pa0.clone();
                    tpath = __pa1.clone();
                    let true = (Types::allHaveBindings(vl.clone())?) else { bail!("pattern mismatch") };
                    binding = InstBinding::makeRecordBinding(cache.clone(), env.clone(), tpath.clone(), ty.clone(), vl.clone(), metamodelica::nil(), info.clone())?;
                    (cache, res) = cevalCrefBinding(cache.clone(), env.clone(), inComponentRef.clone(), binding.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone() + 1)?;
                    Ok((cache.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Deref @ DAE::Binding::UNBOUND, false, Absyn::Msg::MSG { info: _ }, _) => {
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Deref @ DAE::Binding::UNBOUND, true, Absyn::Msg::MSG { info: _ }, _) => {
                    let true = (Flags::isSet(Flags::CEVAL.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("#- Ceval.cevalCrefBinding: Ignoring unbound when implicit\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cr, Deref @ DAE::Binding::EQBOUND { constant_: DAE::Const::C_CONST, exp, .. }, r#impl, msg, _) => {
                    let mut subsc: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
                    let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    ::match_deref::match_deref! { match &(exp.clone()) {
                        Deref @ DAE::Exp::REDUCTION { iterators: Deref @ metamodelica::List::Cons { head: Deref @ DAE::ReductionIterator { .. }, tail: Deref @ metamodelica::List::Nil }, reductionInfo: Deref @ DAE::ReductionInfo { path: Deref @ Absyn::Path::IDENT { .. }, .. }, .. } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    (cache, v) = ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
                    subsc = ComponentReference::crefLastSubs(cr.clone())?;
                    (cache, res) = cevalSubscriptValue(cache.clone(), env.clone(), subsc.clone(), v.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
                    Ok((cache.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cr, Deref @ DAE::Binding::EQBOUND { evaluatedExp: Some(e_val), .. }, r#impl, msg, _) => {
                    let mut subsc: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
                    let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    subsc = ComponentReference::crefLastSubs(cr.clone())?;
                    (cache, res) = cevalSubscriptValue(cache.clone(), env.clone(), subsc.clone(), e_val.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
                    Ok((cache.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cr, Deref @ DAE::Binding::EQBOUND { constant_: DAE::Const::C_CONST, exp, .. }, r#impl, msg, _) => {
                    let mut subsc: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
                    let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    (cache, v) = ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
                    subsc = ComponentReference::crefLastSubs(cr.clone())?;
                    (cache, res) = cevalSubscriptValue(cache.clone(), env.clone(), subsc.clone(), v.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
                    Ok((cache.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cr, Deref @ DAE::Binding::EQBOUND { constant_: DAE::Const::C_PARAM, exp, .. }, r#impl, msg, _) => {
                    let mut subsc: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
                    let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    let false = (isRecursiveBinding(cr.clone(), exp.clone())?) else { bail!("pattern mismatch") };
                    (cache, v) = ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
                    subsc = ComponentReference::crefLastSubs(cr.clone())?;
                    (cache, res) = cevalSubscriptValue(cache.clone(), env.clone(), subsc.clone(), v.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
                    Ok((cache.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Deref @ DAE::Binding::EQBOUND { constant_: DAE::Const::C_VAR, exp, .. }, _, Absyn::Msg::MSG { info: _ }, _) => {
                    let mut expstr: ArcStr = arcstr::literal!("");
                    let true = (Flags::isSet(Flags::CEVAL.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("#- Ceval.cevalCrefBinding failed (nonconstant EQBOUND(")).clone())?;
                    expstr = (ExpressionBasics::printExpStr(exp.clone())?).clone();
                    Debug::trace((expstr.clone()).clone())?;
                    Debug::traceln((literal!("))")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, env, e1, _, _, _, _) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let true = (Flags::isSet(Flags::CEVAL.clone())?) else { bail!("pattern mismatch") };
                    s1 = (ComponentReferenceBasics::printComponentRefStr(e1.clone())?).clone();
                    s2 = (TypesDump::printBindingStr(inBinding.clone())?).clone();
                    r#str = (FGraph::printGraphPathStr(env.clone())?).clone();
                    r#str = stringAppendList(list![(literal!("- Ceval.cevalCrefBinding: ")).clone(), (s1.clone()).clone(), (literal!(" = [")).clone(), (s2.clone()).clone(), (literal!("] in env:")).clone(), (r#str.clone()).clone(), (literal!(" failed")).clone()]);
                    Debug::traceln((r#str.clone()).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outValue))
}

fn isRecursiveBinding(mut cr: Arc<DAE::ComponentRef>, mut exp: Arc<DAE::Exp>) -> Result<bool> {
    let mut res: bool = false;
    res = 'mc: {
        let __mc_input = (cr.clone(), exp.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut res: bool = res.clone();
                    res = List::any(Expression::extractCrefsFromExp(exp.clone())?, Arc::new({ let __pe_b1 = cr.clone(); move |__pe_a0| ComponentReferenceBasics::crefEqual(__pe_a0, __pe_b1.clone()) }));
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(res)
}

pub fn cevalSubscriptValue(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpSubscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut inValue: Arc<Values::Value>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpSubscriptLst.clone(), inValue.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, env, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp }, tail: subs }, Deref @ Values::Value::ARRAY { valueLst: lst, .. }, r#impl, msg, _) => {
            let mut n: i32 = 0;
            let mut subval: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut cache = (*cache).clone();
            (cache, v) = ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
            n = (::match_deref::match_deref! { match &(v.clone()) {
        Deref @ Values::Value::INTEGER { integer: n } => n.clone(),
        Deref @ Values::Value::ENUM_LITERAL { index: n, .. } => n.clone(),
        _ => bail!("match: no arm matched"),
    } });
            subval = (lst.clone()).get(n.clone())?;
            (cache, res) = cevalSubscriptValue(cache.clone(), env.clone(), subs.clone(), subval.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
            (cache.clone(), res.clone())
        },
        (cache, env, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::SLICE { exp }, tail: subs }, Deref @ Values::Value::ARRAY { valueLst: lst, .. }, r#impl, msg, _) => {
            let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut sliceLst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut subvals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut slice: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            let mut lst = (*lst).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, .. }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            sliceLst = __pa1.clone();
            slice = List::map(sliceLst.clone(), (std::sync::Arc::new(ValuesUtil::valueInteger) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<i32> + 'static>));
            subvals = List::map1r(slice.clone(), (std::sync::Arc::new(listGet) as std::sync::Arc<dyn ::std::ops::Fn(_, i32) -> Result<_> + 'static>), lst.clone());
            (cache, lst) = cevalSubscriptValueList(cache.clone(), env.clone(), subs.clone(), subvals.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
            res = ValuesMake::makeArray(lst.clone())?;
            (cache.clone(), res.clone())
        },
        (cache, env, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::WHOLEDIM, tail: subs }, subval @ Deref @ Values::Value::ARRAY { .. }, r#impl, msg, _) => {
            let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut lst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            if subs.clone().is_empty() {
                res = subval.clone();
            } else {
                (cache, lst) = cevalSubscriptValueList(cache.clone(), env.clone(), subs.clone(), var_field!((**subval).valueLst, Values::Value::ARRAY).clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
                res = ValuesMake::makeArray(lst.clone())?;
            }
            (cache.clone(), res.clone())
        },
        (cache, _, Deref @ metamodelica::List::Nil, v, _, _, _) => {
            (cache.clone(), v.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn cevalSubscriptValueList(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpSubscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut inValue: Arc<metamodelica::List<Arc<Values::Value>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<Values::Value>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpSubscriptLst.clone(), inValue.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone())) {
        (cache, _, _, Deref @ metamodelica::List::Nil, _, _, _) => {
            (cache.clone(), metamodelica::nil())
        },
        (cache, env, subs, Deref @ metamodelica::List::Cons { head: subval, tail: subvals }, r#impl, msg, _) => {
            let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut lst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            (cache, res) = cevalSubscriptValue(cache.clone(), env.clone(), subs.clone(), subval.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
            (cache, lst) = cevalSubscriptValueList(cache.clone(), env.clone(), subs.clone(), subvals.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
            (cache.clone(), cons(res.clone(), lst.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

pub fn cevalSubscripts(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpSubscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut inIntegerLst: Arc<metamodelica::List<i32>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<DAE::Subscript>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExpSubscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
    (outCache, outExpSubscriptLst) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inExpSubscriptLst.clone(), inIntegerLst.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ metamodelica::List::Nil, _, _, _, _) => {
                    Ok((cache.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: sub, tail: subs }, Deref @ metamodelica::List::Cons { head: dim, tail: dims }, r#impl, msg, _) => {
                    let mut sub_1: Arc<DAE::Subscript> = Arc::new(DAE::Subscript::WHOLEDIM);
                    let mut subs_1: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, sub_1) = cevalSubscript(cache.clone(), env.clone(), sub.clone(), dim.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
                    (cache, subs_1) = cevalSubscripts(cache.clone(), env.clone(), subs.clone(), dims.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
                    Ok((cache.clone(), cons(sub_1.clone(), subs_1.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: sub, tail: subs }, Deref @ metamodelica::List::Cons { head: dim, tail: dims }, r#impl, msg, _) => {
                    let mut subs_1: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    if '__try0: {
                        (_, _) = unwrap_break_err!(cevalSubscript(cache.clone(), env.clone(), sub.clone(), dim.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    (cache, subs_1) = cevalSubscripts(cache.clone(), env.clone(), subs.clone(), dims.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
                    Ok((cache.clone(), cons(sub.clone(), subs_1.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outExpSubscriptLst))
}

pub fn cevalSubscript(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inSubscript: Arc<DAE::Subscript>, mut inInteger: i32, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<DAE::Subscript>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outSubscript: Arc<DAE::Subscript> = Arc::new(DAE::Subscript::WHOLEDIM);
    (outCache, outSubscript) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inSubscript.clone(), inInteger.clone(), inBoolean.clone(), inMsg.clone(), numIter.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Subscript::WHOLEDIM, _, _, _, _) => {
                    Ok((cache.clone(), Arc::new(openmodelica_frontend_types::DAE::Subscript::WHOLEDIM)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ENUM_LITERAL { .. } }, _, _, _, _) => {
                    Ok((cache.clone(), inSubscript.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Subscript::INDEX { exp: e1 }, _, r#impl, msg, _) => {
                    let mut v1: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut cache = (*cache).clone();
                    (cache, v1) = ceval(cache.clone(), env.clone(), e1.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
                    e1_1 = (::match_deref::match_deref! { match &(v1.clone()) {
        Deref @ Values::Value::INTEGER { integer: _ } => ValuesUtil::valueExp(v1.clone(), None)?,
        Deref @ Values::Value::ENUM_LITERAL { .. } => ValuesUtil::valueExp(v1.clone(), None)?,
        Deref @ Values::Value::BOOL { boolean: _ } => ValuesUtil::valueExp(v1.clone(), None)?,
        _ => bail!("match: no arm matched"),
    } });
                    Ok((cache.clone(), Arc::new(DAE::Subscript::INDEX { exp: e1_1.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Subscript::SLICE { exp: e1 }, _, r#impl, msg, _) => {
                    let mut v1: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut cache = (*cache).clone();
                    (cache, v1) = ceval(cache.clone(), env.clone(), e1.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
                    e1_1 = ValuesUtil::valueExp(v1.clone(), Some(e1.clone()))?;
                    Ok((cache.clone(), Arc::new(DAE::Subscript::SLICE { exp: e1_1.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outSubscript))
}

fn crefEqualValue(mut c: Arc<DAE::ComponentRef>, mut v: Arc<DAE::Binding>) -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &((c.clone(), v.clone())) {
        (_, Deref @ DAE::Binding::EQBOUND { exp: Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ }, evaluatedExp: None, constant_: _, source: _ }) => {
            ComponentReferenceBasics::crefEqual(c.clone(), cr.clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBoolean)
}

fn dimensionSliceInRange(mut arr: Arc<Values::Value>, mut dimSize: i32) -> Result<bool> {
    let mut inRange: bool = false;
    inRange = 'mc: {
        let __mc_input = (arr.clone(), dimSize.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::List::Nil, .. }, _) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::ARRAY { dimLst: Deref @ metamodelica::List::Cons { head: dim, tail: dims }, valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: indx }, tail: vlst } }, _) => {
                    let mut dim = (*dim).clone();
                    let mut dims = (*dims).clone();
                    dim = dim.clone() - 1;
                    dims = cons(dim.clone(), dims.clone());
                    let true = (indx.clone() <= dimSize.clone()) else { bail!("pattern mismatch") };
                    let true = (dimensionSliceInRange(Arc::new(Values::Value::ARRAY { valueLst: vlst.clone(), dimLst: dims.clone() }), dimSize.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(inRange)
}

fn cevalReduction(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut opPath: Arc<Absyn::Path>, mut inCurValue: Option<Arc<Values::Value>>, mut exp: Arc<DAE::Exp>, mut exprType: Arc<DAE::Type>, mut foldName: ArcStr, mut resultName: ArcStr, mut foldExp: Option<Arc<DAE::Exp>>, mut iteratorNames: Arc<metamodelica::List<ArcStr>>, mut inValueMatrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Values::Value>>>>>, mut iterTypes: Arc<metamodelica::List<Arc<DAE::Type>>>, mut r#impl: bool, mut msg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Option<Arc<Values::Value>>)> {
    let mut newCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut result: Option<Arc<Values::Value>> = None;
    (newCache, result) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), opPath.clone(), inCurValue.clone(), exp.clone(), exprType.clone(), foldName.clone(), resultName.clone(), foldExp.clone(), iteratorNames.clone(), inValueMatrix.clone(), iterTypes.clone(), r#impl.clone(), msg.clone(), numIter.clone())) {
        (cache, _, Deref @ Absyn::Path::IDENT { name: Deref @ "list" }, Some(Deref @ Values::Value::LIST { valueLst: vals }), _, _, _, _, _, _, Deref @ metamodelica::List::Nil, _, _, _, _) => {
            let mut vals = (*vals).clone();
            vals = vals.clone().reverse();
            (cache.clone(), Some(Arc::new(Values::Value::LIST { valueLst: vals.clone() })))
        },
        (cache, _, Deref @ Absyn::Path::IDENT { name: Deref @ "listReverse" }, Some(Deref @ Values::Value::LIST { valueLst: _ }), _, _, _, _, _, _, Deref @ metamodelica::List::Nil, _, _, _, _) => {
            (cache.clone(), inCurValue.clone())
        },
        (cache, _, Deref @ Absyn::Path::IDENT { name: Deref @ "array" }, Some(Deref @ Values::Value::ARRAY { valueLst: vals, dimLst: dims }), _, _, _, _, _, _, Deref @ metamodelica::List::Nil, _, _, _, _) => {
            let mut vals = (*vals).clone();
            vals = vals.clone().reverse();
            (cache.clone(), Some(Arc::new(Values::Value::ARRAY { valueLst: vals.clone(), dimLst: dims.clone() })))
        },
        (cache, _, _, curValue, _, _, _, _, _, _, Deref @ metamodelica::List::Nil, _, _, _, _) => {
            (cache.clone(), curValue.clone())
        },
        (cache, env, _, curValue, _, _, _, _, _, _, Deref @ metamodelica::List::Cons { head: vals, tail: valueMatrix }, _, _, _, _) => {
            let mut new_env: FCore::Graph;
            let mut cache = (*cache).clone();
            let mut curValue = (*curValue).clone();
            new_env = extendFrameForIterators(env.clone(), iteratorNames.clone(), vals.clone(), iterTypes.clone())?;
            (cache, curValue) = cevalReductionEvalAndFold(cache.clone(), new_env.clone(), opPath.clone(), curValue.clone(), exp.clone(), exprType.clone(), (foldName.clone()).clone(), (resultName.clone()).clone(), foldExp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
            (cache, curValue) = cevalReduction(cache.clone(), env.clone(), opPath.clone(), curValue.clone(), exp.clone(), exprType.clone(), (foldName.clone()).clone(), (resultName.clone()).clone(), foldExp.clone(), iteratorNames.clone(), valueMatrix.clone(), iterTypes.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
            (cache.clone(), curValue.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((newCache, result))
}

fn cevalReductionEvalAndFold(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut opPath: Arc<Absyn::Path>, mut inCurValue: Option<Arc<Values::Value>>, mut exp: Arc<DAE::Exp>, mut exprType: Arc<DAE::Type>, mut foldName: ArcStr, mut resultName: ArcStr, mut foldExp: Option<Arc<DAE::Exp>>, mut r#impl: bool, mut msg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Option<Arc<Values::Value>>)> {
    let mut newCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut result: Option<Arc<Values::Value>> = None;
    (newCache, result) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), opPath.clone(), inCurValue.clone(), exp.clone(), exprType.clone(), foldName.clone(), resultName.clone(), foldExp.clone(), r#impl.clone(), msg.clone(), numIter.clone())) {
        (cache, env, _, curValue, _, _, _, _, _, _, _, _) => {
            let mut value: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut cache = (*cache).clone();
            (cache, value) = ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
            (cache, result) = cevalReductionFold(cache.clone(), env.clone(), opPath.clone(), curValue.clone(), value.clone(), (foldName.clone()).clone(), (resultName.clone()).clone(), foldExp.clone(), exprType.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
            (cache.clone(), result.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((newCache, result))
}

fn cevalReductionFold(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut opPath: Arc<Absyn::Path>, mut inCurValue: Option<Arc<Values::Value>>, mut inValue: Arc<Values::Value>, mut foldName: ArcStr, mut resultName: ArcStr, mut foldExp: Option<Arc<DAE::Exp>>, mut exprType: Arc<DAE::Type>, mut r#impl: bool, mut msg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Option<Arc<Values::Value>>)> {
    let mut newCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut result: Option<Arc<Values::Value>> = None;
    (newCache, result) = (::match_deref::match_deref! { match &((inCache.clone(), opPath.clone(), inCurValue.clone(), foldExp.clone())) {
        (cache, Deref @ Absyn::Path::IDENT { name: Deref @ "array" }, Some(value), _) => {
            let mut value = (*value).clone();
            value = valueArrayCons(ValuesUtil::unboxIfBoxedVal(inValue.clone()), value.clone());
            (cache.clone(), Some(value.clone()))
        },
        (cache, Deref @ Absyn::Path::IDENT { name: Deref @ "list" }, Some(value), _) => {
            let mut value = (*value).clone();
            value = valueCons(ValuesUtil::unboxIfBoxedVal(inValue.clone()), value.clone())?;
            (cache.clone(), Some(value.clone()))
        },
        (cache, Deref @ Absyn::Path::IDENT { name: Deref @ "listReverse" }, Some(value), _) => {
            let mut value = (*value).clone();
            value = valueCons(ValuesUtil::unboxIfBoxedVal(inValue.clone()), value.clone())?;
            (cache.clone(), Some(value.clone()))
        },
        (cache, _, None, _) => {
            (cache.clone(), Some(inValue.clone()))
        },
        (cache, _, Some(value), Some(exp)) => {
            let mut env: FCore::Graph;
            let mut cache = (*cache).clone();
            let mut value = (*value).clone();
            env = FGraph::addForIterator(inEnv.clone(), (foldName.clone()).clone(), exprType.clone(), Arc::new(DAE::Binding::VALBOUND { valBound: inValue.clone(), source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE }), openmodelica_frontend_types::SCode::Variability::VAR, Some(openmodelica_frontend_types::DAE::Const::C_CONST))?;
            env = FGraph::addForIterator(env.clone(), (resultName.clone()).clone(), exprType.clone(), Arc::new(DAE::Binding::VALBOUND { valBound: value.clone(), source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE }), openmodelica_frontend_types::SCode::Variability::VAR, Some(openmodelica_frontend_types::DAE::Const::C_CONST))?;
            (cache, value) = ceval(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
            (cache.clone(), Some(value.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((newCache, result))
}

fn valueArrayCons(mut v1: Arc<Values::Value>, mut v2: Arc<Values::Value>) -> Arc<Values::Value> {
    let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    res = (::match_deref::match_deref! { match &((v1.clone(), v2.clone())) {
        (_, Deref @ Values::Value::ARRAY { dimLst: Deref @ metamodelica::List::Cons { head: dim_size, tail: rest_dims }, valueLst: vals }) => {
            let mut dim_size = (*dim_size).clone();
            dim_size = dim_size.clone() + 1;
            Arc::new(Values::Value::ARRAY { valueLst: cons(v1.clone(), vals.clone()), dimLst: cons(dim_size.clone(), rest_dims.clone()) })
        },
        _ => {
            Arc::new(Values::Value::ARRAY { valueLst: list![v1.clone(), v2.clone()], dimLst: list![2] })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

fn valueCons(mut inV1: Arc<Values::Value>, mut inV2: Arc<Values::Value>) -> Result<Arc<Values::Value>> {
    let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    res = (::match_deref::match_deref! { match &((inV1.clone(), inV2.clone())) {
        (Deref @ Values::Value::META_BOX { value: v1 }, Deref @ Values::Value::LIST { valueLst: vals }) => {
            Arc::new(Values::Value::LIST { valueLst: cons(v1.clone(), vals.clone()) })
        },
        (v1, Deref @ Values::Value::LIST { valueLst: vals }) => {
            Arc::new(Values::Value::LIST { valueLst: cons(v1.clone(), vals.clone()) })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(res)
}

fn cevalReductionIterators(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIterators: Arc<metamodelica::List<Arc<DAE::ReductionIterator>>>, mut r#impl: bool, mut msg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<metamodelica::List<Arc<Values::Value>>>>>, Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<DAE::Type>>>)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut vals: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Values::Value>>>>> = metamodelica::nil();
    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut iterVals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut id: ArcStr = arcstr::literal!("");
    let mut exp: Arc<DAE::Exp>;
    let mut guardExp: Option<Arc<DAE::Exp>> = None;
    for mut iter in &*inIterators.clone() {
        let mut iter = iter.clone();
        let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(iter.clone()) {
            Deref @ DAE::ReductionIterator { id: __pa0, exp: __pa1, guardExp: __pa2, ty: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        id = __pa0.clone();
        exp = __pa1.clone();
        guardExp = __pa2.clone();
        ty = __pa3.clone();
        (outCache, val) = ceval(outCache.clone(), inEnv.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
        iterVals = ValuesUtil::arrayOrListVals(val.clone(), true)?;
        (outCache, iterVals) = filterReductionIterator(outCache.clone(), inEnv.clone(), (id.clone()).clone(), ty.clone(), iterVals.clone(), guardExp.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
        vals = cons(iterVals.clone(), vals.clone());
        names = cons((id.clone()).clone(), names.clone());
        dims = cons((iterVals.clone().len() as i32), dims.clone());
        tys = cons(ty.clone(), tys.clone());
    }
    Ok((outCache, vals, names, dims, tys))
}

fn filterReductionIterator(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut id: ArcStr, mut ty: Arc<DAE::Type>, mut inVals: Arc<metamodelica::List<Arc<Values::Value>>>, mut guardExp: Option<Arc<DAE::Exp>>, mut r#impl: bool, mut msg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<Values::Value>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outVals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    (outCache, outVals) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), id.clone(), ty.clone(), inVals.clone(), guardExp.clone(), r#impl.clone(), msg.clone(), numIter.clone())) {
        (cache, _, _, _, Deref @ metamodelica::List::Nil, _, _, _, _) => {
            (cache.clone(), metamodelica::nil())
        },
        (cache, env, _, _, Deref @ metamodelica::List::Cons { head: val, tail: vals }, Some(exp), _, _, _) => {
            let mut b: bool = false;
            let mut new_env: FCore::Graph;
            let mut cache = (*cache).clone();
            let mut vals = (*vals).clone();
            new_env = FGraph::addForIterator(env.clone(), (id.clone()).clone(), ty.clone(), Arc::new(DAE::Binding::VALBOUND { valBound: val.clone(), source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE }), openmodelica_frontend_types::SCode::Variability::VAR, Some(openmodelica_frontend_types::DAE::Const::C_CONST))?;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), new_env.clone(), exp.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?) {
                (__pa0, Deref @ Values::Value::BOOL { boolean: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            b = __pa1.clone();
            (cache, vals) = filterReductionIterator(cache.clone(), env.clone(), (id.clone()).clone(), ty.clone(), vals.clone(), guardExp.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
            vals = if (b.clone()) {cons(val.clone(), vals.clone())} else {vals.clone()};
            (cache.clone(), vals.clone())
        },
        (cache, _, _, _, vals, None, _, _, _) => {
            (cache.clone(), vals.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outVals))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn extendFrameForIterators(mut inEnv: FCore::Graph, mut inNames: Arc<metamodelica::List<ArcStr>>, mut inVals: Arc<metamodelica::List<Arc<Values::Value>>>, mut inTys: Arc<metamodelica::List<Arc<DAE::Type>>>) -> Result<FCore::Graph> {
    let mut outEnv: FCore::Graph;
    outEnv = (::match_deref::match_deref! { match &((inEnv.clone(), inNames.clone(), inVals.clone(), inTys.clone())) {
        (env, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            env.clone()
        },
        (env, Deref @ metamodelica::List::Cons { head: name, tail: names }, Deref @ metamodelica::List::Cons { head: val, tail: vals }, Deref @ metamodelica::List::Cons { head: ty, tail: tys }) => {
            let mut env = (*env).clone();
            env = FGraph::addForIterator(env.clone(), (name.clone()).clone(), ty.clone(), Arc::new(DAE::Binding::VALBOUND { valBound: val.clone(), source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE }), openmodelica_frontend_types::SCode::Variability::VAR, Some(openmodelica_frontend_types::DAE::Const::C_CONST))?;
            env = extendFrameForIterators(env.clone(), names.clone(), vals.clone(), tys.clone())?;
            env.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outEnv)
}

fn backpatchArrayReduction(mut path: Arc<Absyn::Path>, mut iterType: Absyn::ReductionIterType, mut inValue: Arc<Values::Value>, mut dims: Arc<metamodelica::List<i32>>) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outValue = (::match_deref::match_deref! { match &((path.clone(), iterType.clone(), inValue.clone(), dims.clone())) {
        (_, _, value, Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }) => {
            value.clone()
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "array" }, Absyn::ReductionIterType::COMBINE, Deref @ Values::Value::ARRAY { valueLst: vals, .. }, _) => {
            let mut value: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            value = backpatchArrayReduction3(vals.clone(), dims.clone().reverse(), (std::sync::Arc::new(ValuesMake::makeArray) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<Values::Value>> + 'static>))?;
            value.clone()
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "list" }, Absyn::ReductionIterType::COMBINE, Deref @ Values::Value::LIST { valueLst: vals }, _) => {
            let mut value: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            value = backpatchArrayReduction3(vals.clone(), dims.clone().reverse(), (std::sync::Arc::new(fnptr!(ValuesMake::makeList, Arc<metamodelica::List<Arc<Values::Value>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<Values::Value>> + 'static>))?;
            value.clone()
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "listReverse" }, Absyn::ReductionIterType::COMBINE, Deref @ Values::Value::LIST { valueLst: vals }, _) => {
            let mut value: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            value = backpatchArrayReduction3(vals.clone(), dims.clone().reverse(), (std::sync::Arc::new(fnptr!(ValuesMake::makeList, Arc<metamodelica::List<Arc<Values::Value>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<Values::Value>> + 'static>))?;
            value.clone()
        },
        _ => {
            inValue.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outValue)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn backpatchArrayReduction3(mut inVals: Arc<metamodelica::List<Arc<Values::Value>>>, mut inDims: Arc<metamodelica::List<i32>>, mut makeSequence: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<Values::Value>> + 'static>) -> Result<Arc<Values::Value>> {
    pub type Func = std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<Values::Value>> + 'static>;

    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outValue = (::match_deref::match_deref! { match &((inVals.clone(), inDims.clone(), makeSequence.clone())) {
        (vals, Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, _) => {
            let mut value: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            value = makeSequence(vals.clone())?;
            value.clone()
        },
        (vals, Deref @ metamodelica::List::Cons { head: dim, tail: dims }, _) => {
            let mut valMatrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Values::Value>>>>> = metamodelica::nil();
            let mut value: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut vals = (*vals).clone();
            valMatrix = List::partition(vals.clone(), dim.clone())?;
            vals = List::map(valMatrix.clone(), makeSequence.clone());
            value = backpatchArrayReduction3(vals.clone(), dims.clone(), makeSequence.clone())?;
            value.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outValue)
}

pub fn cevalSimple(mut exp: Arc<DAE::Exp>) -> Result<Arc<Values::Value>> {
    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, val) = ceval(FCore::emptyCache(), FGraph::empty(), exp.clone(), false, Absyn::Msg::MSG { info: Absyn::dummyInfo.clone() }, 0)?;
    Ok(val)
}

pub fn cevalAstExp(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<Absyn::Exp>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<Absyn::Exp>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    (outCache, outExp) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inExp.clone(), inBoolean.clone(), inMsg.clone(), info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, e @ Deref @ Absyn::Exp::INTEGER { .. }, _, _, _) => {
                    Ok((cache.clone(), e.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, e @ Deref @ Absyn::Exp::REAL { .. }, _, _, _) => {
                    Ok((cache.clone(), e.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, e @ Deref @ Absyn::Exp::CREF { .. }, _, _, _) => {
                    Ok((cache.clone(), e.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, e @ Deref @ Absyn::Exp::STRING { .. }, _, _, _) => {
                    Ok((cache.clone(), e.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, e @ Deref @ Absyn::Exp::BOOL { .. }, _, _, _) => {
                    Ok((cache.clone(), e.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::BINARY { exp2: e2, op, exp1: e1 }, r#impl, msg, _) => {
                    let mut e1_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut e2_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut cache = (*cache).clone();
                    (cache, e1_1) = cevalAstExp(cache.clone(), env.clone(), e1.clone(), r#impl.clone(), msg.clone(), info.clone())?;
                    (cache, e2_1) = cevalAstExp(cache.clone(), env.clone(), e2.clone(), r#impl.clone(), msg.clone(), info.clone())?;
                    Ok((cache.clone(), Arc::new(Absyn::Exp::BINARY { exp1: e1_1.clone(), op: op.clone(), exp2: e2_1.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::UNARY { exp: e, op }, r#impl, msg, _) => {
                    let mut e_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut cache = (*cache).clone();
                    (cache, e_1) = cevalAstExp(cache.clone(), env.clone(), e.clone(), r#impl.clone(), msg.clone(), info.clone())?;
                    Ok((cache.clone(), Arc::new(Absyn::Exp::UNARY { op: op.clone(), exp: e_1.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::LBINARY { exp2: e2, op, exp1: e1 }, r#impl, msg, _) => {
                    let mut e1_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut e2_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut cache = (*cache).clone();
                    (cache, e1_1) = cevalAstExp(cache.clone(), env.clone(), e1.clone(), r#impl.clone(), msg.clone(), info.clone())?;
                    (cache, e2_1) = cevalAstExp(cache.clone(), env.clone(), e2.clone(), r#impl.clone(), msg.clone(), info.clone())?;
                    Ok((cache.clone(), Arc::new(Absyn::Exp::LBINARY { exp1: e1_1.clone(), op: op.clone(), exp2: e2_1.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::LUNARY { exp: e, op }, r#impl, msg, _) => {
                    let mut e_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut cache = (*cache).clone();
                    (cache, e_1) = cevalAstExp(cache.clone(), env.clone(), e.clone(), r#impl.clone(), msg.clone(), info.clone())?;
                    Ok((cache.clone(), Arc::new(Absyn::Exp::LUNARY { op: op.clone(), exp: e_1.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::RELATION { exp2: e2, op, exp1: e1 }, r#impl, msg, _) => {
                    let mut e1_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut e2_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut cache = (*cache).clone();
                    (cache, e1_1) = cevalAstExp(cache.clone(), env.clone(), e1.clone(), r#impl.clone(), msg.clone(), info.clone())?;
                    (cache, e2_1) = cevalAstExp(cache.clone(), env.clone(), e2.clone(), r#impl.clone(), msg.clone(), info.clone())?;
                    Ok((cache.clone(), Arc::new(Absyn::Exp::RELATION { exp1: e1_1.clone(), op: op.clone(), exp2: e2_1.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::IFEXP { elseIfBranch: nest, elseBranch: else_, trueBranch: then_, ifExp: cond }, r#impl, msg, _) => {
                    let mut cond_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut then_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut else_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut nest_1: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, cond_1) = cevalAstExp(cache.clone(), env.clone(), cond.clone(), r#impl.clone(), msg.clone(), info.clone())?;
                    (cache, then_1) = cevalAstExp(cache.clone(), env.clone(), then_.clone(), r#impl.clone(), msg.clone(), info.clone())?;
                    (cache, else_1) = cevalAstExp(cache.clone(), env.clone(), else_.clone(), r#impl.clone(), msg.clone(), info.clone())?;
                    (cache, nest_1) = cevalAstExpexpList(cache.clone(), env.clone(), nest.clone(), r#impl.clone(), msg.clone(), info.clone())?;
                    Ok((cache.clone(), Arc::new(Absyn::Exp::IFEXP { ifExp: cond_1.clone(), trueBranch: then_1.clone(), elseBranch: else_1.clone(), elseIfBranch: nest_1.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { argNames: Deref @ metamodelica::List::Nil, args: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil } }, function_: Deref @ Absyn::ComponentRef::CREF_IDENT { subscripts: Deref @ metamodelica::List::Nil, name: Deref @ "Eval" }, .. }, r#impl, msg, _) => {
                    let mut exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut daeExp: Arc<DAE::Exp>;
                    let mut cache = (*cache).clone();
                    (cache, daeExp, _) = Static::elabExp(cache.clone(), env.clone(), e.clone(), r#impl.clone(), true, openmodelica_frontend_types::DAE::Prefix::NOPRE, info.clone())?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ceval(cache.clone(), env.clone(), daeExp.clone(), r#impl.clone(), msg.clone(), 0)?) {
                        (__pa0, Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: __pa1 } }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    exp = __pa1.clone();
                    Ok((cache.clone(), exp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, e @ Deref @ Absyn::Exp::CALL { .. }, _, _, _) => {
                    Ok((cache.clone(), e.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::ARRAY { arrayExp: expl }, r#impl, msg, _) => {
                    let mut expl_1: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, expl_1) = cevalAstExpList(cache.clone(), env.clone(), expl.clone(), r#impl.clone(), msg.clone(), info.clone())?;
                    Ok((cache.clone(), Arc::new(Absyn::Exp::ARRAY { arrayExp: expl_1.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::MATRIX { matrix: lstExpl }, r#impl, msg, _) => {
                    let mut lstExpl_1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, lstExpl_1) = cevalAstExpListList(cache.clone(), env.clone(), lstExpl.clone(), r#impl.clone(), msg.clone(), info.clone())?;
                    Ok((cache.clone(), Arc::new(Absyn::Exp::MATRIX { matrix: lstExpl_1.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::RANGE { stop: e3, step: Some(e2), start: e1 }, r#impl, msg, _) => {
                    let mut e1_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut e2_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut e3_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut cache = (*cache).clone();
                    (cache, e1_1) = cevalAstExp(cache.clone(), env.clone(), e1.clone(), r#impl.clone(), msg.clone(), info.clone())?;
                    (cache, e2_1) = cevalAstExp(cache.clone(), env.clone(), e2.clone(), r#impl.clone(), msg.clone(), info.clone())?;
                    (cache, e3_1) = cevalAstExp(cache.clone(), env.clone(), e3.clone(), r#impl.clone(), msg.clone(), info.clone())?;
                    Ok((cache.clone(), Arc::new(Absyn::Exp::RANGE { start: e1_1.clone(), step: Some(e2_1.clone()), stop: e3_1.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::RANGE { stop: e3, step: None, start: e1 }, r#impl, msg, _) => {
                    let mut e1_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut e3_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut cache = (*cache).clone();
                    (cache, e1_1) = cevalAstExp(cache.clone(), env.clone(), e1.clone(), r#impl.clone(), msg.clone(), info.clone())?;
                    (cache, e3_1) = cevalAstExp(cache.clone(), env.clone(), e3.clone(), r#impl.clone(), msg.clone(), info.clone())?;
                    Ok((cache.clone(), Arc::new(Absyn::Exp::RANGE { start: e1_1.clone(), step: None, stop: e3_1.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::TUPLE { expressions: expl }, r#impl, msg, _) => {
                    let mut expl_1: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, expl_1) = cevalAstExpList(cache.clone(), env.clone(), expl.clone(), r#impl.clone(), msg.clone(), info.clone())?;
                    Ok((cache.clone(), Arc::new(Absyn::Exp::TUPLE { expressions: expl_1.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ Absyn::Exp::END, _, _, _) => {
                    Ok((cache.clone(), Arc::new(openmodelica_ast::Absyn::Exp::END)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, e @ Deref @ Absyn::Exp::CODE { .. }, _, _, _) => {
                    Ok((cache.clone(), e.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outExp))
}

pub fn cevalAstExpList(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inAbsynExpLst: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<Absyn::Exp>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outAbsynExpLst: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
    (outCache, outAbsynExpLst) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inAbsynExpLst.clone(), inBoolean.clone(), inMsg.clone(), info.clone())) {
        (cache, _, Deref @ metamodelica::List::Nil, _, _, _) => {
            (cache.clone(), metamodelica::nil())
        },
        (cache, env, Deref @ metamodelica::List::Cons { head: e, tail: es }, r#impl, msg, _) => {
            let mut res: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            (cache, _) = cevalAstExp(cache.clone(), env.clone(), e.clone(), r#impl.clone(), msg.clone(), info.clone())?;
            (cache, res) = cevalAstExpList(cache.clone(), env.clone(), es.clone(), r#impl.clone(), msg.clone(), info.clone())?;
            (cache.clone(), cons(e.clone(), res.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outAbsynExpLst))
}

fn cevalAstExpListList(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inAbsynExpLstLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outAbsynExpLstLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>> = metamodelica::nil();
    (outCache, outAbsynExpLstLst) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inAbsynExpLstLst.clone(), inBoolean.clone(), inMsg.clone(), info.clone())) {
        (cache, _, Deref @ metamodelica::List::Nil, _, _, _) => {
            (cache.clone(), metamodelica::nil())
        },
        (cache, env, Deref @ metamodelica::List::Cons { head: e, tail: es }, r#impl, msg, _) => {
            let mut res: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            (cache, _) = cevalAstExpList(cache.clone(), env.clone(), e.clone(), r#impl.clone(), msg.clone(), info.clone())?;
            (cache, res) = cevalAstExpListList(cache.clone(), env.clone(), es.clone(), r#impl.clone(), msg.clone(), info.clone())?;
            (cache.clone(), cons(e.clone(), res.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outAbsynExpLstLst))
}

pub fn cevalAstElt(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inElement: Arc<Absyn::Element>, mut inBoolean: bool, mut inMsg: Absyn::Msg) -> Result<(FCore::Cache, Arc<Absyn::Element>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outElement: Arc<Absyn::Element>;
    (outCache, outElement) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inElement.clone(), inBoolean.clone(), inMsg.clone())) {
        (cache, env, Deref @ Absyn::Element::ELEMENT { constrainClass: c, info: info @ SourceInfo { .. }, specification: Deref @ Absyn::ElementSpec::COMPONENTS { components: citems, typeSpec: tp, attributes: attr }, innerOuter: io, redeclareKeywords: r, finalPrefix: f }, r#impl, msg) => {
            let mut citems_1: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            (cache, citems_1) = cevalAstCitems(cache.clone(), env.clone(), citems.clone(), r#impl.clone(), msg.clone(), info.clone())?;
            (cache.clone(), Arc::new(Absyn::Element::ELEMENT { finalPrefix: f.clone(), redeclareKeywords: r.clone(), innerOuter: io.clone(), specification: Arc::new(Absyn::ElementSpec::COMPONENTS { attributes: attr.clone(), typeSpec: tp.clone(), components: citems_1.clone() }), info: info.clone(), constrainClass: c.clone() }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outElement))
}

fn cevalAstCitems(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inAbsynComponentItemLst: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outAbsynComponentItemLst: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
    (outCache, outAbsynComponentItemLst) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inAbsynComponentItemLst.clone(), inBoolean.clone(), inMsg.clone(), info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ metamodelica::List::Nil, _, _, _) => {
                    Ok((cache.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ComponentItem { comment: cmt, condition: cond, component: Absyn::Component { modification: modopt, arrayDim: ad, name: id } }, tail: xs }, r#impl, msg, _) => {
                    let mut res: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
                    let mut modopt_1: Option<Arc<Absyn::Modification>> = None;
                    let mut ad_1: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, res) = cevalAstCitems(cache.clone(), env.clone(), xs.clone(), r#impl.clone(), msg.clone(), info.clone())?;
                    (cache, modopt_1) = cevalAstModopt(cache.clone(), env.clone(), modopt.clone(), r#impl.clone(), msg.clone(), info.clone())?;
                    (cache, ad_1) = cevalAstArraydim(cache.clone(), env.clone(), ad.clone(), r#impl.clone(), msg.clone(), info.clone())?;
                    Ok((cache.clone(), cons(Arc::new(Absyn::ComponentItem { component: Absyn::Component { name: (id.clone()).clone(), arrayDim: ad_1.clone(), modification: modopt_1.clone() }, condition: cond.clone(), comment: cmt.clone() }), res.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: x, tail: xs }, r#impl, msg, _) => {
                    let mut res: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, res) = cevalAstCitems(cache.clone(), env.clone(), xs.clone(), r#impl.clone(), msg.clone(), info.clone())?;
                    Ok((cache.clone(), cons(x.clone(), res.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outAbsynComponentItemLst))
}

fn cevalAstModopt(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inAbsynModificationOption: Option<Arc<Absyn::Modification>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut info: SourceInfo) -> Result<(FCore::Cache, Option<Arc<Absyn::Modification>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outAbsynModificationOption: Option<Arc<Absyn::Modification>> = None;
    (outCache, outAbsynModificationOption) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inAbsynModificationOption.clone(), inBoolean.clone(), inMsg.clone(), info.clone())) {
        (cache, env, Some(r#mod), r#impl, msg, _) => {
            let mut res: Arc<Absyn::Modification>;
            let mut cache = (*cache).clone();
            (cache, res) = cevalAstModification(cache.clone(), env.clone(), r#mod.clone(), r#impl.clone(), msg.clone(), info.clone())?;
            (cache.clone(), Some(res.clone()))
        },
        (cache, _, None, _, _, _) => {
            (cache.clone(), None)
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outAbsynModificationOption))
}

fn cevalAstModification(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inModification: Arc<Absyn::Modification>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<Absyn::Modification>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outModification: Arc<Absyn::Modification>;
    (outCache, outModification) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inModification.clone(), inBoolean.clone(), inMsg.clone(), info.clone())) {
        (cache, env, Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: e, info: info2 }, elementArgLst: eltargs }, r#impl, msg, _) => {
            let mut e_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut eltargs_1: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            (cache, e_1) = cevalAstExp(cache.clone(), env.clone(), e.clone(), r#impl.clone(), msg.clone(), info.clone())?;
            (cache, eltargs_1) = cevalAstEltargs(cache.clone(), env.clone(), eltargs.clone(), r#impl.clone(), msg.clone(), info.clone())?;
            (cache.clone(), Arc::new(Absyn::Modification { elementArgLst: eltargs_1.clone(), eqMod: Arc::new(Absyn::EqMod::EQMOD { exp: e_1.clone(), info: info2.clone() }) }))
        },
        (cache, env, Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::NOMOD, elementArgLst: eltargs }, r#impl, msg, _) => {
            let mut eltargs_1: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            (cache, eltargs_1) = cevalAstEltargs(cache.clone(), env.clone(), eltargs.clone(), r#impl.clone(), msg.clone(), info.clone())?;
            (cache.clone(), Arc::new(Absyn::Modification { elementArgLst: eltargs_1.clone(), eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) }))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outModification))
}

fn cevalAstEltargs(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inAbsynElementArgLst: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<Absyn::ElementArg>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outAbsynElementArgLst: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    (outCache, outAbsynElementArgLst) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inAbsynElementArgLst.clone(), inBoolean.clone(), inMsg.clone(), info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ metamodelica::List::Nil, _, _, _) => {
                    Ok((cache.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { info: mod_info, comment: stropt, modification: Some(r#mod), path: p, eachPrefix: e, finalPrefix: b }, tail: args }, r#impl, msg, _) => {
                    let mut mod_1: Arc<Absyn::Modification>;
                    let mut res: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, mod_1) = cevalAstModification(cache.clone(), env.clone(), r#mod.clone(), r#impl.clone(), msg.clone(), info.clone())?;
                    (cache, res) = cevalAstEltargs(cache.clone(), env.clone(), args.clone(), r#impl.clone(), msg.clone(), info.clone())?;
                    Ok((cache.clone(), cons(Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: b.clone(), eachPrefix: e.clone(), path: p.clone(), modification: Some(mod_1.clone()), comment: stropt.clone(), info: mod_info.clone() }), res.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: m, tail: args }, r#impl, msg, _) => {
                    let mut res: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, res) = cevalAstEltargs(cache.clone(), env.clone(), args.clone(), r#impl.clone(), msg.clone(), info.clone())?;
                    Ok((cache.clone(), cons(m.clone(), res.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outAbsynElementArgLst))
}

fn cevalAstArraydim(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inArrayDim: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<Absyn::Subscript>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outArrayDim: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
    (outCache, outArrayDim) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inArrayDim.clone(), inBoolean.clone(), inMsg.clone(), info.clone())) {
        (cache, _, Deref @ metamodelica::List::Nil, _, _, _) => {
            (cache.clone(), metamodelica::nil())
        },
        (cache, env, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Subscript::NOSUB, tail: xs }, r#impl, msg, _) => {
            let mut res: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            (cache, res) = cevalAstArraydim(cache.clone(), env.clone(), xs.clone(), r#impl.clone(), msg.clone(), info.clone())?;
            (cache.clone(), cons(Arc::new(openmodelica_ast::Absyn::Subscript::NOSUB), res.clone()))
        },
        (cache, env, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Subscript::SUBSCRIPT { subscript: e }, tail: xs }, r#impl, msg, _) => {
            let mut res: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            (cache, res) = cevalAstArraydim(cache.clone(), env.clone(), xs.clone(), r#impl.clone(), msg.clone(), info.clone())?;
            (cache, _) = cevalAstExp(cache.clone(), env.clone(), e.clone(), r#impl.clone(), msg.clone(), info.clone())?;
            (cache.clone(), cons(Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: e.clone() }), res.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outArrayDim))
}

fn cevalAstExpexpList(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExpTpls: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut info: SourceInfo) -> Result<(FCore::Cache, Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExpTpls: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>> = metamodelica::nil();
    (outCache, outExpTpls) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inExpTpls.clone(), inBoolean.clone(), inMsg.clone(), info.clone())) {
        (cache, _, Deref @ metamodelica::List::Nil, _, _, _) => {
            (cache.clone(), metamodelica::nil())
        },
        (cache, env, Deref @ metamodelica::List::Cons { head: (e1, e2), tail: xs }, r#impl, msg, _) => {
            let mut e1_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut e2_1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut res: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            (cache, e1_1) = cevalAstExp(cache.clone(), env.clone(), e1.clone(), r#impl.clone(), msg.clone(), info.clone())?;
            (cache, e2_1) = cevalAstExp(cache.clone(), env.clone(), e2.clone(), r#impl.clone(), msg.clone(), info.clone())?;
            (cache, res) = cevalAstExpexpList(cache.clone(), env.clone(), xs.clone(), r#impl.clone(), msg.clone(), info.clone())?;
            (cache.clone(), cons((e1_1.clone(), e2_1.clone()), res.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outExpTpls))
}

pub fn cevalDimension(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inDimension: Arc<DAE::Dimension>, mut inImpl: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inDimension.clone(), inImpl.clone(), inMsg.clone(), numIter.clone())) {
        (_, _, Deref @ DAE::Dimension::DIM_INTEGER { integer: dim_int }, _, _, _) => {
            (inCache.clone(), Arc::new(Values::Value::INTEGER { integer: dim_int.clone() }))
        },
        (_, _, Deref @ DAE::Dimension::DIM_ENUM { size: dim_int, .. }, _, _, _) => {
            (inCache.clone(), Arc::new(Values::Value::INTEGER { integer: dim_int.clone() }))
        },
        (_, _, Deref @ DAE::Dimension::DIM_BOOLEAN, _, _, _) => {
            (inCache.clone(), Arc::new(Values::Value::INTEGER { integer: 2 }))
        },
        (_, _, Deref @ DAE::Dimension::DIM_EXP { exp }, _, _, _) => {
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            (cache, res) = ceval(inCache.clone(), inEnv.clone(), exp.clone(), inImpl.clone(), inMsg.clone(), numIter.clone() + 1)?;
            (cache.clone(), res.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outValue))
}

fn makeReductionAllCombinations(mut inValMatrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Values::Value>>>>>, mut rtype: Absyn::ReductionIterType) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<Arc<Values::Value>>>>>> {
    let mut valMatrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Values::Value>>>>> = metamodelica::nil();
    valMatrix = (::match_deref::match_deref! { match &((inValMatrix.clone(), rtype.clone())) {
        (_, Absyn::ReductionIterType::COMBINE) => List::allCombinations(inValMatrix.clone(), Some(100000), Absyn::dummyInfo.clone())?.reverse(),
        (_, Absyn::ReductionIterType::THREAD) => List::transposeList(inValMatrix.clone())?.reverse(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(valMatrix)
}

