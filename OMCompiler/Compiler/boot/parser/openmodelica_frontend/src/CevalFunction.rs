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

use crate::Ceval;
use crate::ComponentReference;
use crate::DAEDump;
use crate::DAEUtil;
use crate::Expression;
use crate::FCore;
use crate::FGraph;
use crate::FNode;
use crate::Lookup;
use crate::Types;
use crate::ValuesUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_dump::ValuesMake;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Graph;
use openmodelica_util::Lapack;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

// Jump table for CevalFunction:
// [TYPE]  Types.
// [EVAL]  Constant evaluation functions.
// [EENV]  Environment extension functions (add variables).
// [MENV]  Environment manipulation functions (set and get variables).
// [DEPS]  Function variable dependency handling.
// [EOPT]  Expression optimization functions.
// public imports
// protected imports
// [TYPE]  Types
pub type FunctionVar = (Arc<DAE::Element>, Option<Arc<Values::Value>>);

// LoopControl is used to control the functions behaviour in different
// situations. All evaluation functions returns a LoopControl variable that
// tells the caller whether it should continue evaluating or not.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LoopControl {
    /// Continue to the next statement.
    NEXT,
    /// Exit the current loop.
    BREAK,
    /// Exit the function.
    RETURN,
}
pub use self::LoopControl::{NEXT,BREAK,RETURN};

// [EVAL]  Constant evaluation functions.
pub fn evaluate(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inFunction: DAE::Function, mut inFunctionArguments: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outResult: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outResult) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inFunction.clone(), inFunctionArguments.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, DAE::Function::FUNCTION { source: src, partialPrefix: false, type_: ty, functions: Deref @ metamodelica::List::Cons { head: func, tail: _ }, path: p, .. }, _) => {
                    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut func_name: ArcStr = arcstr::literal!("");
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    func_name = (AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone();
                    (cache, result) = evaluateFunctionDefinition(inCache.clone(), inEnv.clone(), (func_name.clone()).clone(), func.clone(), ty.clone(), inFunctionArguments.clone(), src.clone())?;
                    Ok((cache.clone(), result.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, DAE::Function::FUNCTION { partialPrefix, functions: Deref @ metamodelica::List::Cons { head: _, tail: _ }, path: p, .. }, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- CevalFunction.evaluate failed for function: ")); __mm_s.push_str(&*if (partialPrefix.clone()) {literal!("partial ")} else {literal!("")}); __mm_s.push_str(&*AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outResult))
}

fn evaluateFunctionDefinition(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inFuncName: ArcStr, mut inFunc: DAE::FunctionDefinition, mut inFuncType: Arc<DAE::Type>, mut inFuncArgs: Arc<metamodelica::List<Arc<Values::Value>>>, mut inSource: Arc<DAE::ElementSource>) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outResult: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outResult) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inFuncName.clone(), inFunc.clone(), inFuncType.clone(), inFuncArgs.clone(), inSource.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, DAE::FunctionDefinition::FUNCTION_DEF { body }, _, _, _) => {
                    let mut vars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut output_vars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut func_params: Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>> = metamodelica::nil();
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut env: FCore::Graph;
                    let mut return_values: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut return_value: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut body = (*body).clone();
                    (vars, body) = List::splitOnFirstMatch(body.clone(), Arc::new(fnptr!(DAEUtil::isNotVar, Arc<DAE::Element>)))?;
                    vars = List::map(vars.clone(), Arc::new(removeSelfReferentialDims));
                    output_vars = List::filterOnTrue(vars.clone(), Arc::new(fnptr!(DAEUtil::isOutputVar, Arc<DAE::Element>)));
                    func_params = pairFuncParamsWithArgs(vars.clone(), inFuncArgs.clone())?;
                    func_params = sortFunctionVarsByDependency(func_params.clone(), inSource.clone())?;
                    (cache, env) = setupFunctionEnvironment(inCache.clone(), inEnv.clone(), (inFuncName.clone()).clone(), func_params.clone())?;
                    (cache, env, _) = evaluateElements(body.clone(), cache.clone(), env.clone(), crate::CevalFunction::LoopControl::NEXT)?;
                    return_values = List::map1(output_vars.clone(), Arc::new(getFunctionReturnValue), env.clone());
                    return_value = boxReturnValue(return_values.clone());
                    Ok((cache.clone(), return_value.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, DAE::FunctionDefinition::FUNCTION_EXT { externalDecl: DAE::ExternalDecl { args: ext_fun_args, name: ext_fun_name, .. }, body }, _, _, _) => {
                    let mut vars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut output_vars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
                    let mut func_params: Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>> = metamodelica::nil();
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut env: FCore::Graph;
                    let mut return_values: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut return_value: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    (vars, _) = List::splitOnFirstMatch(body.clone(), Arc::new(fnptr!(DAEUtil::isNotVar, Arc<DAE::Element>)))?;
                    vars = List::map(vars.clone(), Arc::new(removeSelfReferentialDims));
                    output_vars = List::filterOnTrue(vars.clone(), Arc::new(fnptr!(DAEUtil::isOutputVar, Arc<DAE::Element>)));
                    func_params = pairFuncParamsWithArgs(vars.clone(), inFuncArgs.clone())?;
                    func_params = sortFunctionVarsByDependency(func_params.clone(), inSource.clone())?;
                    (cache, env) = setupFunctionEnvironment(inCache.clone(), inEnv.clone(), (inFuncName.clone()).clone(), func_params.clone())?;
                    (cache, env) = evaluateExternalFunc((ext_fun_name.clone()).clone(), ext_fun_args.clone(), cache.clone(), env.clone())?;
                    return_values = List::map1(output_vars.clone(), Arc::new(getFunctionReturnValue), env.clone());
                    return_value = boxReturnValue(return_values.clone());
                    Ok((cache.clone(), return_value.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- CevalFunction.evaluateFunction failed.\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outResult))
}

fn pairFuncParamsWithArgs(mut inElements: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inValues: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>> {
    let mut outFunctionVars: Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>> = metamodelica::nil();
    outFunctionVars = (::match_deref::match_deref! { match &((inElements.clone(), inValues.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::VAR { direction: DAE::VarDirection::INPUT, .. }, tail: _ }, Deref @ metamodelica::List::Nil) => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("- CevalFunction.pairFuncParamsWithArgs failed because of too few input arguments.\n")).clone())?;
            bail!("fail")
        },
        (Deref @ metamodelica::List::Cons { head: var @ Deref @ DAE::Element::VAR { direction: DAE::VarDirection::INPUT, .. }, tail: rest_vars }, Deref @ metamodelica::List::Cons { head: val, tail: rest_vals }) => {
            let mut params: Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>> = metamodelica::nil();
            params = pairFuncParamsWithArgs(rest_vars.clone(), rest_vals.clone())?;
            cons((var.clone(), Some(val.clone())), params.clone())
        },
        (Deref @ metamodelica::List::Cons { head: var, tail: rest_vars }, _) => {
            let mut params: Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>> = metamodelica::nil();
            params = pairFuncParamsWithArgs(rest_vars.clone(), inValues.clone())?;
            cons((var.clone(), None), params.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outFunctionVars)
}

fn removeSelfReferentialDims(mut inElement: Arc<DAE::Element>) -> Result<Arc<DAE::Element>> {
    let mut outElement: Arc<DAE::Element>;
    outElement = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::VAR { componentRef: cref @ Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, .. }, kind: vk, direction: vd, parallelism: vp, protection: vv, ty, binding: bind, dims, connectorType: ct, source: es, variableAttributesOption: va, comment: cmt, innerOuter: io, encrypted: e } => {
            let mut dims = (*dims).clone();
            dims = List::map1(dims.clone(), Arc::new(removeSelfReferentialDim), (name.clone()).clone());
            Arc::new(DAE::Element::VAR { componentRef: cref.clone(), kind: vk.clone(), direction: vd.clone(), parallelism: vp.clone(), protection: vv.clone(), ty: ty.clone(), binding: bind.clone(), dims: dims.clone(), connectorType: ct.clone(), source: es.clone(), variableAttributesOption: va.clone(), comment: cmt.clone(), innerOuter: io.clone(), encrypted: e.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outElement)
}

fn removeSelfReferentialDim(mut inDim: Arc<DAE::Dimension>, mut inName: ArcStr) -> Result<Arc<DAE::Dimension>> {
    let mut outDim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    outDim = 'mc: {
        let __mc_input = (inDim.clone(), inName.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Dimension::DIM_EXP { exp }, _) => {
                    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    crefs = Expression::extractCrefsFromExp(exp.clone())?;
                    let true = (List::isMemberOnTrue((inName.clone()).clone(), crefs.clone(), Arc::new(fnptr!(isCrefNamed, ArcStr, Arc<DAE::ComponentRef>)))) else { bail!("pattern mismatch") };
                    Ok(Arc::new(openmodelica_frontend_types::DAE::Dimension::DIM_UNKNOWN))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inDim.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outDim)
}

fn isCrefNamed(mut inName: ArcStr, mut inCref: Arc<DAE::ComponentRef>) -> bool {
    let mut outIsNamed: bool = false;
    outIsNamed = (::match_deref::match_deref! { match &((inName.clone(), inCref.clone())) {
        (_, Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, .. }) => {
            stringEq((inName.clone()).clone(), (name.clone()).clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsNamed
}

fn evaluateExtInputArg(mut inArgument: DAE::ExtArg, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(Arc<Values::Value>, FCore::Cache)> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    (outValue, outCache) = 'mc: {
        let __mc_input = (inArgument.clone(), inCache.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (DAE::ExtArg::EXTARG { type_: ref ty, componentRef: ref cref, .. }, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            val = getVariableValue(cref.clone(), ty.clone(), inEnv.clone())?;
            Ok((val.clone(), inCache.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (DAE::ExtArg::EXTARGEXP { exp: mut exp, .. }, mut cache, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            (cache, val) = cevalExp(exp.clone(), cache.clone(), inEnv.clone())?;
            Ok((val.clone(), cache.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (DAE::ExtArg::EXTARGSIZE { exp: mut exp, componentRef: ref cref, .. }, mut cache, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut exp = exp.clone();
            exp = Arc::new(DAE::Exp::SIZE { exp: Arc::new(DAE::Exp::CREF { componentRef: cref.clone(), ty: DAE::T_UNKNOWN_DEFAULT.clone() }), sz: Some(exp.clone()) });
            (cache, val) = cevalExp(exp.clone(), cache.clone(), inEnv.clone())?;
            Ok((val.clone(), cache.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut err_str: ArcStr = arcstr::literal!("");
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            err_str = (DAEDump::dumpExtArgStr(inArgument.clone())?).clone();
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- CevalFunction.evaluateExtInputArg failed on ")); __mm_s.push_str(&*err_str.clone()); ArcStr::from(__mm_s) }).clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outValue, outCache))
}

fn evaluateExtIntArg(mut inArg: DAE::ExtArg, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(i32, FCore::Cache)> {
    let mut outValue: i32 = 0;
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(evaluateExtInputArg(inArg.clone(), inCache.clone(), inEnv.clone())?) {
        (Deref @ Values::Value::INTEGER { integer: __pa0 }, __pa1) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outValue = __pa0.clone();
    outCache = __pa1.clone();
    Ok((outValue, outCache))
}

fn evaluateExtRealArg(mut inArg: DAE::ExtArg, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(metamodelica::Real, FCore::Cache)> {
    let mut outValue: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(evaluateExtInputArg(inArg.clone(), inCache.clone(), inEnv.clone())?) {
        (Deref @ Values::Value::REAL { real: __pa0 }, __pa1) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outValue = __pa0.clone();
    outCache = __pa1.clone();
    Ok((outValue, outCache))
}

fn evaluateExtStringArg(mut inArg: DAE::ExtArg, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(ArcStr, FCore::Cache)> {
    let mut outValue: ArcStr = arcstr::literal!("");
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(evaluateExtInputArg(inArg.clone(), inCache.clone(), inEnv.clone())?) {
        (Deref @ Values::Value::STRING { string: __pa0 }, __pa1) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outValue = __pa0.clone();
    outCache = __pa1.clone();
    Ok((outValue, outCache))
}

fn evaluateExtIntArrayArg(mut inArg: DAE::ExtArg, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(Arc<metamodelica::List<i32>>, FCore::Cache)> {
    let mut outValue: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (val, outCache) = evaluateExtInputArg(inArg.clone(), inCache.clone(), inEnv.clone())?;
    outValue = ValuesUtil::arrayValueInts(val.clone())?;
    Ok((outValue, outCache))
}

fn evaluateExtRealArrayArg(mut inArg: DAE::ExtArg, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(Arc<metamodelica::List<metamodelica::Real>>, FCore::Cache)> {
    let mut outValue: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (val, outCache) = evaluateExtInputArg(inArg.clone(), inCache.clone(), inEnv.clone())?;
    outValue = ValuesUtil::arrayValueReals(val.clone())?;
    Ok((outValue, outCache))
}

fn evaluateExtRealMatrixArg(mut inArg: DAE::ExtArg, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, FCore::Cache)> {
    let mut outValue: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (val, outCache) = evaluateExtInputArg(inArg.clone(), inCache.clone(), inEnv.clone())?;
    outValue = ValuesUtil::matrixValueReals(val.clone())?;
    Ok((outValue, outCache))
}

fn evaluateExtOutputArg(mut inArg: DAE::ExtArg) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let DAE::EXTARG { componentRef: __pa0, .. } = (inArg.clone()) else { bail!("pattern mismatch") };
    outCref = __pa0.clone();
    Ok(outCref)
}

fn assignExtOutputs(mut inArgs: Arc<metamodelica::List<DAE::ExtArg>>, mut inValues: Arc<metamodelica::List<Arc<Values::Value>>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    (outCache, outEnv) = (::match_deref::match_deref! { match &((inArgs.clone(), inValues.clone(), inCache.clone(), inEnv.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _, _) => {
            (inCache.clone(), inEnv.clone())
        },
        (Deref @ metamodelica::List::Cons { head: arg, tail: rest_args }, Deref @ metamodelica::List::Cons { head: val, tail: rest_vals }, cache, env) => {
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut val = (*val).clone();
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            cr = evaluateExtOutputArg(arg.clone())?;
            val = unliftExtOutputValue(cr.clone(), val.clone(), env.clone())?;
            (cache, env) = assignVariable(cr.clone(), val.clone(), cache.clone(), env.clone())?;
            (cache, env) = assignExtOutputs(rest_args.clone(), rest_vals.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outEnv))
}

fn unliftExtOutputValue(mut inCref: Arc<DAE::ComponentRef>, mut inValue: Arc<Values::Value>, mut inEnv: FCore::Graph) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outValue = 'mc: {
        let __mc_input = (inCref.clone(), inValue.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ Values::Value::ARRAY { dimLst: Deref @ metamodelica::List::Cons { head: dim, tail: _ }, valueLst: vals @ Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { .. }, tail: _ } }, _) => {
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let mut vals = (*vals).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(getVariableTypeAndBinding(inCref.clone(), inEnv.clone())?) {
                        (Deref @ DAE::Type::T_ARRAY { dims: __pa0, ty: __pa1 }, _) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    dims = __pa0.clone();
                    ty = __pa1.clone();
                    let false = (Types::isNonscalarArray(ty.clone(), dims.clone())?) else { bail!("pattern mismatch") };
                    vals = List::map(vals.clone(), Arc::new(ValuesUtil::arrayScalar));
                    Ok(Arc::new(Values::Value::ARRAY { valueLst: vals.clone(), dimLst: list![dim.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inValue.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outValue)
}

fn evaluateExternalFunc(mut inFuncName: ArcStr, mut inFuncArgs: Arc<metamodelica::List<DAE::ExtArg>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    (outCache, outEnv) = (::match_deref::match_deref! { match &((inFuncName.clone(), inFuncArgs.clone(), inCache.clone(), inEnv.clone())) {
        (Deref @ "dgeev", Deref @ metamodelica::List::Cons { head: arg_JOBVL, tail: Deref @ metamodelica::List::Cons { head: arg_JOBVR, tail: Deref @ metamodelica::List::Cons { head: arg_N, tail: Deref @ metamodelica::List::Cons { head: arg_A, tail: Deref @ metamodelica::List::Cons { head: arg_LDA, tail: Deref @ metamodelica::List::Cons { head: arg_WR, tail: Deref @ metamodelica::List::Cons { head: arg_WI, tail: Deref @ metamodelica::List::Cons { head: arg_VL, tail: Deref @ metamodelica::List::Cons { head: arg_LDVL, tail: Deref @ metamodelica::List::Cons { head: arg_VR, tail: Deref @ metamodelica::List::Cons { head: arg_LDVR, tail: Deref @ metamodelica::List::Cons { head: arg_WORK, tail: Deref @ metamodelica::List::Cons { head: arg_LWORK, tail: Deref @ metamodelica::List::Cons { head: arg_INFO, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } } }, cache, env) => {
            let mut val_INFO: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_WI: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_WORK: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_WR: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_A: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_VL: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_VR: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut INFO: i32 = 0;
            let mut LDA: i32 = 0;
            let mut LDVL: i32 = 0;
            let mut LDVR: i32 = 0;
            let mut LWORK: i32 = 0;
            let mut N: i32 = 0;
            let mut JOBVL: ArcStr = arcstr::literal!("");
            let mut JOBVR: ArcStr = arcstr::literal!("");
            let mut WI: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut WORK: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut WR: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
            let mut VL: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
            let mut VR: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
            let mut arg_out: Arc<metamodelica::List<DAE::ExtArg>> = metamodelica::nil();
            let mut val_out: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (JOBVL, cache) = evaluateExtStringArg(arg_JOBVL.clone(), cache.clone(), env.clone())?;
            (JOBVR, cache) = evaluateExtStringArg(arg_JOBVR.clone(), cache.clone(), env.clone())?;
            (N, cache) = evaluateExtIntArg(arg_N.clone(), cache.clone(), env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(arg_A.clone(), cache.clone(), env.clone())?;
            (LDA, cache) = evaluateExtIntArg(arg_LDA.clone(), cache.clone(), env.clone())?;
            (LDVL, cache) = evaluateExtIntArg(arg_LDVL.clone(), cache.clone(), env.clone())?;
            (LDVR, cache) = evaluateExtIntArg(arg_LDVR.clone(), cache.clone(), env.clone())?;
            (WORK, cache) = evaluateExtRealArrayArg(arg_WORK.clone(), cache.clone(), env.clone())?;
            (LWORK, cache) = evaluateExtIntArg(arg_LWORK.clone(), cache.clone(), env.clone())?;
            (A, WR, WI, VL, VR, WORK, INFO) = Lapack::dgeev((JOBVL.clone()).clone(), (JOBVR.clone()).clone(), N.clone(), A.clone(), LDA.clone(), LDVL.clone(), LDVR.clone(), WORK.clone(), LWORK.clone());
            val_A = ValuesMake::makeRealMatrix(A.clone())?;
            val_WR = ValuesMake::makeRealArray(WR.clone())?;
            val_WI = ValuesMake::makeRealArray(WI.clone())?;
            val_VL = ValuesMake::makeRealMatrix(VL.clone())?;
            val_VR = ValuesMake::makeRealMatrix(VR.clone())?;
            val_WORK = ValuesMake::makeRealArray(WORK.clone())?;
            val_INFO = ValuesMake::makeInteger(INFO.clone());
            arg_out = list![arg_A.clone(), arg_WR.clone(), arg_WI.clone(), arg_VL.clone(), arg_VR.clone(), arg_WORK.clone(), arg_INFO.clone()];
            val_out = list![val_A.clone(), val_WR.clone(), val_WI.clone(), val_VL.clone(), val_VR.clone(), val_WORK.clone(), val_INFO.clone()];
            (cache, env) = assignExtOutputs(arg_out.clone(), val_out.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        (Deref @ "dgegv", Deref @ metamodelica::List::Cons { head: arg_JOBVL, tail: Deref @ metamodelica::List::Cons { head: arg_JOBVR, tail: Deref @ metamodelica::List::Cons { head: arg_N, tail: Deref @ metamodelica::List::Cons { head: arg_A, tail: Deref @ metamodelica::List::Cons { head: arg_LDA, tail: Deref @ metamodelica::List::Cons { head: arg_B, tail: Deref @ metamodelica::List::Cons { head: arg_LDB, tail: Deref @ metamodelica::List::Cons { head: arg_ALPHAR, tail: Deref @ metamodelica::List::Cons { head: arg_ALPHAI, tail: Deref @ metamodelica::List::Cons { head: arg_BETA, tail: Deref @ metamodelica::List::Cons { head: arg_VL, tail: Deref @ metamodelica::List::Cons { head: arg_LDVL, tail: Deref @ metamodelica::List::Cons { head: arg_VR, tail: Deref @ metamodelica::List::Cons { head: arg_LDVR, tail: Deref @ metamodelica::List::Cons { head: arg_WORK, tail: Deref @ metamodelica::List::Cons { head: arg_LWORK, tail: Deref @ metamodelica::List::Cons { head: arg_INFO, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } } } } } }, cache, env) => {
            let mut val_INFO: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_ALPHAI: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_ALPHAR: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_BETA: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_WORK: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_VL: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_VR: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut INFO: i32 = 0;
            let mut LDA: i32 = 0;
            let mut LDB: i32 = 0;
            let mut LDVL: i32 = 0;
            let mut LDVR: i32 = 0;
            let mut LWORK: i32 = 0;
            let mut N: i32 = 0;
            let mut JOBVL: ArcStr = arcstr::literal!("");
            let mut JOBVR: ArcStr = arcstr::literal!("");
            let mut ALPHAI: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut ALPHAR: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut BETA: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut WORK: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
            let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
            let mut VL: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
            let mut VR: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
            let mut arg_out: Arc<metamodelica::List<DAE::ExtArg>> = metamodelica::nil();
            let mut val_out: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (JOBVL, cache) = evaluateExtStringArg(arg_JOBVL.clone(), cache.clone(), env.clone())?;
            (JOBVR, cache) = evaluateExtStringArg(arg_JOBVR.clone(), cache.clone(), env.clone())?;
            (N, cache) = evaluateExtIntArg(arg_N.clone(), cache.clone(), env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(arg_A.clone(), cache.clone(), env.clone())?;
            (LDA, cache) = evaluateExtIntArg(arg_LDA.clone(), cache.clone(), env.clone())?;
            (B, cache) = evaluateExtRealMatrixArg(arg_B.clone(), cache.clone(), env.clone())?;
            (LDB, cache) = evaluateExtIntArg(arg_LDB.clone(), cache.clone(), env.clone())?;
            (LDVL, cache) = evaluateExtIntArg(arg_LDVL.clone(), cache.clone(), env.clone())?;
            (LDVR, cache) = evaluateExtIntArg(arg_LDVR.clone(), cache.clone(), env.clone())?;
            (WORK, cache) = evaluateExtRealArrayArg(arg_WORK.clone(), cache.clone(), env.clone())?;
            (LWORK, cache) = evaluateExtIntArg(arg_LWORK.clone(), cache.clone(), env.clone())?;
            (ALPHAR, ALPHAI, BETA, VL, VR, WORK, INFO) = Lapack::dgegv((JOBVL.clone()).clone(), (JOBVR.clone()).clone(), N.clone(), A.clone(), LDA.clone(), B.clone(), LDB.clone(), LDVL.clone(), LDVR.clone(), WORK.clone(), LWORK.clone());
            val_ALPHAR = ValuesMake::makeRealArray(ALPHAR.clone())?;
            val_ALPHAI = ValuesMake::makeRealArray(ALPHAI.clone())?;
            val_BETA = ValuesMake::makeRealArray(BETA.clone())?;
            val_VL = ValuesMake::makeRealMatrix(VL.clone())?;
            val_VR = ValuesMake::makeRealMatrix(VR.clone())?;
            val_WORK = ValuesMake::makeRealArray(WORK.clone())?;
            val_INFO = ValuesMake::makeInteger(INFO.clone());
            arg_out = list![arg_ALPHAR.clone(), arg_ALPHAI.clone(), arg_BETA.clone(), arg_VL.clone(), arg_VR.clone(), arg_WORK.clone(), arg_INFO.clone()];
            val_out = list![val_ALPHAR.clone(), val_ALPHAI.clone(), val_BETA.clone(), val_VL.clone(), val_VR.clone(), val_WORK.clone(), val_INFO.clone()];
            (cache, env) = assignExtOutputs(arg_out.clone(), val_out.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        (Deref @ "dgels", Deref @ metamodelica::List::Cons { head: arg_TRANS, tail: Deref @ metamodelica::List::Cons { head: arg_M, tail: Deref @ metamodelica::List::Cons { head: arg_N, tail: Deref @ metamodelica::List::Cons { head: arg_NRHS, tail: Deref @ metamodelica::List::Cons { head: arg_A, tail: Deref @ metamodelica::List::Cons { head: arg_LDA, tail: Deref @ metamodelica::List::Cons { head: arg_B, tail: Deref @ metamodelica::List::Cons { head: arg_LDB, tail: Deref @ metamodelica::List::Cons { head: arg_WORK, tail: Deref @ metamodelica::List::Cons { head: arg_LWORK, tail: Deref @ metamodelica::List::Cons { head: arg_INFO, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } }, cache, env) => {
            let mut val_INFO: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_WORK: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_A: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_B: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut INFO: i32 = 0;
            let mut LDA: i32 = 0;
            let mut LDB: i32 = 0;
            let mut LWORK: i32 = 0;
            let mut M: i32 = 0;
            let mut N: i32 = 0;
            let mut NRHS: i32 = 0;
            let mut TRANS: ArcStr = arcstr::literal!("");
            let mut WORK: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
            let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
            let mut arg_out: Arc<metamodelica::List<DAE::ExtArg>> = metamodelica::nil();
            let mut val_out: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (TRANS, cache) = evaluateExtStringArg(arg_TRANS.clone(), cache.clone(), env.clone())?;
            (M, cache) = evaluateExtIntArg(arg_M.clone(), cache.clone(), env.clone())?;
            (N, cache) = evaluateExtIntArg(arg_N.clone(), cache.clone(), env.clone())?;
            (NRHS, cache) = evaluateExtIntArg(arg_NRHS.clone(), cache.clone(), env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(arg_A.clone(), cache.clone(), env.clone())?;
            (LDA, cache) = evaluateExtIntArg(arg_LDA.clone(), cache.clone(), env.clone())?;
            (B, cache) = evaluateExtRealMatrixArg(arg_B.clone(), cache.clone(), env.clone())?;
            (LDB, cache) = evaluateExtIntArg(arg_LDB.clone(), cache.clone(), env.clone())?;
            (WORK, cache) = evaluateExtRealArrayArg(arg_WORK.clone(), cache.clone(), env.clone())?;
            (LWORK, cache) = evaluateExtIntArg(arg_LWORK.clone(), cache.clone(), env.clone())?;
            (A, B, WORK, INFO) = Lapack::dgels((TRANS.clone()).clone(), M.clone(), N.clone(), NRHS.clone(), A.clone(), LDA.clone(), B.clone(), LDB.clone(), WORK.clone(), LWORK.clone());
            val_A = ValuesMake::makeRealMatrix(A.clone())?;
            val_B = ValuesMake::makeRealMatrix(B.clone())?;
            val_WORK = ValuesMake::makeRealArray(WORK.clone())?;
            val_INFO = ValuesMake::makeInteger(INFO.clone());
            arg_out = list![arg_A.clone(), arg_B.clone(), arg_WORK.clone(), arg_INFO.clone()];
            val_out = list![val_A.clone(), val_B.clone(), val_WORK.clone(), val_INFO.clone()];
            (cache, env) = assignExtOutputs(arg_out.clone(), val_out.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        (Deref @ "dgelsx", Deref @ metamodelica::List::Cons { head: arg_M, tail: Deref @ metamodelica::List::Cons { head: arg_N, tail: Deref @ metamodelica::List::Cons { head: arg_NRHS, tail: Deref @ metamodelica::List::Cons { head: arg_A, tail: Deref @ metamodelica::List::Cons { head: arg_LDA, tail: Deref @ metamodelica::List::Cons { head: arg_B, tail: Deref @ metamodelica::List::Cons { head: arg_LDB, tail: Deref @ metamodelica::List::Cons { head: arg_JPVT, tail: Deref @ metamodelica::List::Cons { head: arg_RCOND, tail: Deref @ metamodelica::List::Cons { head: arg_RANK, tail: Deref @ metamodelica::List::Cons { head: arg_WORK, tail: Deref @ metamodelica::List::Cons { head: arg_INFO, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } }, cache, env) => {
            let mut val_INFO: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_RANK: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_JPVT: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_A: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_B: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut INFO: i32 = 0;
            let mut LDA: i32 = 0;
            let mut LDB: i32 = 0;
            let mut M: i32 = 0;
            let mut N: i32 = 0;
            let mut NRHS: i32 = 0;
            let mut RANK: i32 = 0;
            let mut RCOND: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut JPVT: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut WORK: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
            let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
            let mut arg_out: Arc<metamodelica::List<DAE::ExtArg>> = metamodelica::nil();
            let mut val_out: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (M, cache) = evaluateExtIntArg(arg_M.clone(), cache.clone(), env.clone())?;
            (N, cache) = evaluateExtIntArg(arg_N.clone(), cache.clone(), env.clone())?;
            (NRHS, cache) = evaluateExtIntArg(arg_NRHS.clone(), cache.clone(), env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(arg_A.clone(), cache.clone(), env.clone())?;
            (LDA, cache) = evaluateExtIntArg(arg_LDA.clone(), cache.clone(), env.clone())?;
            (B, cache) = evaluateExtRealMatrixArg(arg_B.clone(), cache.clone(), env.clone())?;
            (LDB, cache) = evaluateExtIntArg(arg_LDB.clone(), cache.clone(), env.clone())?;
            (JPVT, cache) = evaluateExtIntArrayArg(arg_JPVT.clone(), cache.clone(), env.clone())?;
            (RCOND, cache) = evaluateExtRealArg(arg_RCOND.clone(), cache.clone(), env.clone())?;
            (WORK, cache) = evaluateExtRealArrayArg(arg_WORK.clone(), cache.clone(), env.clone())?;
            (A, B, JPVT, RANK, INFO) = Lapack::dgelsx(M.clone(), N.clone(), NRHS.clone(), A.clone(), LDA.clone(), B.clone(), LDB.clone(), JPVT.clone(), RCOND.clone(), WORK.clone());
            val_A = ValuesMake::makeRealMatrix(A.clone())?;
            val_B = ValuesMake::makeRealMatrix(B.clone())?;
            val_JPVT = ValuesMake::makeIntArray(JPVT.clone())?;
            val_RANK = ValuesMake::makeInteger(RANK.clone());
            val_INFO = ValuesMake::makeInteger(INFO.clone());
            arg_out = list![arg_A.clone(), arg_B.clone(), arg_JPVT.clone(), arg_RANK.clone(), arg_INFO.clone()];
            val_out = list![val_A.clone(), val_B.clone(), val_JPVT.clone(), val_RANK.clone(), val_INFO.clone()];
            (cache, env) = assignExtOutputs(arg_out.clone(), val_out.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        (Deref @ "dgelsx", Deref @ metamodelica::List::Cons { head: arg_M, tail: Deref @ metamodelica::List::Cons { head: arg_N, tail: Deref @ metamodelica::List::Cons { head: arg_NRHS, tail: Deref @ metamodelica::List::Cons { head: arg_A, tail: Deref @ metamodelica::List::Cons { head: arg_LDA, tail: Deref @ metamodelica::List::Cons { head: arg_B, tail: Deref @ metamodelica::List::Cons { head: arg_LDB, tail: Deref @ metamodelica::List::Cons { head: arg_JPVT, tail: Deref @ metamodelica::List::Cons { head: arg_RCOND, tail: Deref @ metamodelica::List::Cons { head: arg_RANK, tail: Deref @ metamodelica::List::Cons { head: arg_WORK, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: arg_INFO, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } }, cache, env) => {
            let mut val_INFO: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_RANK: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_JPVT: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_A: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_B: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut INFO: i32 = 0;
            let mut LDA: i32 = 0;
            let mut LDB: i32 = 0;
            let mut M: i32 = 0;
            let mut N: i32 = 0;
            let mut NRHS: i32 = 0;
            let mut RANK: i32 = 0;
            let mut RCOND: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut JPVT: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut WORK: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
            let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
            let mut arg_out: Arc<metamodelica::List<DAE::ExtArg>> = metamodelica::nil();
            let mut val_out: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (M, cache) = evaluateExtIntArg(arg_M.clone(), cache.clone(), env.clone())?;
            (N, cache) = evaluateExtIntArg(arg_N.clone(), cache.clone(), env.clone())?;
            (NRHS, cache) = evaluateExtIntArg(arg_NRHS.clone(), cache.clone(), env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(arg_A.clone(), cache.clone(), env.clone())?;
            (LDA, cache) = evaluateExtIntArg(arg_LDA.clone(), cache.clone(), env.clone())?;
            (B, cache) = evaluateExtRealMatrixArg(arg_B.clone(), cache.clone(), env.clone())?;
            (LDB, cache) = evaluateExtIntArg(arg_LDB.clone(), cache.clone(), env.clone())?;
            (JPVT, cache) = evaluateExtIntArrayArg(arg_JPVT.clone(), cache.clone(), env.clone())?;
            (RCOND, cache) = evaluateExtRealArg(arg_RCOND.clone(), cache.clone(), env.clone())?;
            (WORK, cache) = evaluateExtRealArrayArg(arg_WORK.clone(), cache.clone(), env.clone())?;
            (A, B, JPVT, RANK, INFO) = Lapack::dgelsx(M.clone(), N.clone(), NRHS.clone(), A.clone(), LDA.clone(), B.clone(), LDB.clone(), JPVT.clone(), RCOND.clone(), WORK.clone());
            val_A = ValuesMake::makeRealMatrix(A.clone())?;
            val_B = ValuesMake::makeRealMatrix(B.clone())?;
            val_JPVT = ValuesMake::makeIntArray(JPVT.clone())?;
            val_RANK = ValuesMake::makeInteger(RANK.clone());
            val_INFO = ValuesMake::makeInteger(INFO.clone());
            arg_out = list![arg_A.clone(), arg_B.clone(), arg_JPVT.clone(), arg_RANK.clone(), arg_INFO.clone()];
            val_out = list![val_A.clone(), val_B.clone(), val_JPVT.clone(), val_RANK.clone(), val_INFO.clone()];
            (cache, env) = assignExtOutputs(arg_out.clone(), val_out.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        (Deref @ "dgelsy", Deref @ metamodelica::List::Cons { head: arg_M, tail: Deref @ metamodelica::List::Cons { head: arg_N, tail: Deref @ metamodelica::List::Cons { head: arg_NRHS, tail: Deref @ metamodelica::List::Cons { head: arg_A, tail: Deref @ metamodelica::List::Cons { head: arg_LDA, tail: Deref @ metamodelica::List::Cons { head: arg_B, tail: Deref @ metamodelica::List::Cons { head: arg_LDB, tail: Deref @ metamodelica::List::Cons { head: arg_JPVT, tail: Deref @ metamodelica::List::Cons { head: arg_RCOND, tail: Deref @ metamodelica::List::Cons { head: arg_RANK, tail: Deref @ metamodelica::List::Cons { head: arg_WORK, tail: Deref @ metamodelica::List::Cons { head: arg_LWORK, tail: Deref @ metamodelica::List::Cons { head: arg_INFO, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } }, cache, env) => {
            let mut val_INFO: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_RANK: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_JPVT: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_WORK: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_A: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_B: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut INFO: i32 = 0;
            let mut LDA: i32 = 0;
            let mut LDB: i32 = 0;
            let mut LWORK: i32 = 0;
            let mut M: i32 = 0;
            let mut N: i32 = 0;
            let mut NRHS: i32 = 0;
            let mut RANK: i32 = 0;
            let mut RCOND: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut JPVT: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut WORK: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
            let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
            let mut arg_out: Arc<metamodelica::List<DAE::ExtArg>> = metamodelica::nil();
            let mut val_out: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (M, cache) = evaluateExtIntArg(arg_M.clone(), cache.clone(), env.clone())?;
            (N, cache) = evaluateExtIntArg(arg_N.clone(), cache.clone(), env.clone())?;
            (NRHS, cache) = evaluateExtIntArg(arg_NRHS.clone(), cache.clone(), env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(arg_A.clone(), cache.clone(), env.clone())?;
            (LDA, cache) = evaluateExtIntArg(arg_LDA.clone(), cache.clone(), env.clone())?;
            (B, cache) = evaluateExtRealMatrixArg(arg_B.clone(), cache.clone(), env.clone())?;
            (LDB, cache) = evaluateExtIntArg(arg_LDB.clone(), cache.clone(), env.clone())?;
            (JPVT, cache) = evaluateExtIntArrayArg(arg_JPVT.clone(), cache.clone(), env.clone())?;
            (RCOND, cache) = evaluateExtRealArg(arg_RCOND.clone(), cache.clone(), env.clone())?;
            (WORK, cache) = evaluateExtRealArrayArg(arg_WORK.clone(), cache.clone(), env.clone())?;
            (LWORK, cache) = evaluateExtIntArg(arg_LWORK.clone(), cache.clone(), env.clone())?;
            (A, B, JPVT, RANK, WORK, INFO) = Lapack::dgelsy(M.clone(), N.clone(), NRHS.clone(), A.clone(), LDA.clone(), B.clone(), LDB.clone(), JPVT.clone(), RCOND.clone(), WORK.clone(), LWORK.clone());
            val_A = ValuesMake::makeRealMatrix(A.clone())?;
            val_B = ValuesMake::makeRealMatrix(B.clone())?;
            val_JPVT = ValuesMake::makeIntArray(JPVT.clone())?;
            val_RANK = ValuesMake::makeInteger(RANK.clone());
            val_WORK = ValuesMake::makeRealArray(WORK.clone())?;
            val_INFO = ValuesMake::makeInteger(INFO.clone());
            arg_out = list![arg_A.clone(), arg_B.clone(), arg_JPVT.clone(), arg_RANK.clone(), arg_WORK.clone(), arg_INFO.clone()];
            val_out = list![val_A.clone(), val_B.clone(), val_JPVT.clone(), val_RANK.clone(), val_WORK.clone(), val_INFO.clone()];
            (cache, env) = assignExtOutputs(arg_out.clone(), val_out.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        (Deref @ "dgesv", Deref @ metamodelica::List::Cons { head: arg_N, tail: Deref @ metamodelica::List::Cons { head: arg_NRHS, tail: Deref @ metamodelica::List::Cons { head: arg_A, tail: Deref @ metamodelica::List::Cons { head: arg_LDA, tail: Deref @ metamodelica::List::Cons { head: arg_IPIV, tail: Deref @ metamodelica::List::Cons { head: arg_B, tail: Deref @ metamodelica::List::Cons { head: arg_LDB, tail: Deref @ metamodelica::List::Cons { head: arg_INFO, tail: Deref @ metamodelica::List::Nil } } } } } } } }, cache, env) => {
            let mut val_INFO: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_IPIV: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_A: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_B: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut INFO: i32 = 0;
            let mut LDA: i32 = 0;
            let mut LDB: i32 = 0;
            let mut N: i32 = 0;
            let mut NRHS: i32 = 0;
            let mut IPIV: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
            let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
            let mut arg_out: Arc<metamodelica::List<DAE::ExtArg>> = metamodelica::nil();
            let mut val_out: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (N, cache) = evaluateExtIntArg(arg_N.clone(), cache.clone(), env.clone())?;
            (NRHS, cache) = evaluateExtIntArg(arg_NRHS.clone(), cache.clone(), env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(arg_A.clone(), cache.clone(), env.clone())?;
            (LDA, cache) = evaluateExtIntArg(arg_LDA.clone(), cache.clone(), env.clone())?;
            (B, cache) = evaluateExtRealMatrixArg(arg_B.clone(), cache.clone(), env.clone())?;
            (LDB, cache) = evaluateExtIntArg(arg_LDB.clone(), cache.clone(), env.clone())?;
            (A, IPIV, B, INFO) = Lapack::dgesv(N.clone(), NRHS.clone(), A.clone(), LDA.clone(), B.clone(), LDB.clone());
            val_A = ValuesMake::makeRealMatrix(A.clone())?;
            val_IPIV = ValuesMake::makeIntArray(IPIV.clone())?;
            val_B = ValuesMake::makeRealMatrix(B.clone())?;
            val_INFO = ValuesMake::makeInteger(INFO.clone());
            arg_out = list![arg_A.clone(), arg_IPIV.clone(), arg_B.clone(), arg_INFO.clone()];
            val_out = list![val_A.clone(), val_IPIV.clone(), val_B.clone(), val_INFO.clone()];
            (cache, env) = assignExtOutputs(arg_out.clone(), val_out.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        (Deref @ "dgglse", Deref @ metamodelica::List::Cons { head: arg_M, tail: Deref @ metamodelica::List::Cons { head: arg_N, tail: Deref @ metamodelica::List::Cons { head: arg_P, tail: Deref @ metamodelica::List::Cons { head: arg_A, tail: Deref @ metamodelica::List::Cons { head: arg_LDA, tail: Deref @ metamodelica::List::Cons { head: arg_B, tail: Deref @ metamodelica::List::Cons { head: arg_LDB, tail: Deref @ metamodelica::List::Cons { head: arg_C, tail: Deref @ metamodelica::List::Cons { head: arg_D, tail: Deref @ metamodelica::List::Cons { head: arg_X, tail: Deref @ metamodelica::List::Cons { head: arg_WORK, tail: Deref @ metamodelica::List::Cons { head: arg_LWORK, tail: Deref @ metamodelica::List::Cons { head: arg_INFO, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } }, cache, env) => {
            let mut val_INFO: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_C: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_D: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_WORK: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_X: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_A: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_B: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut INFO: i32 = 0;
            let mut LDA: i32 = 0;
            let mut LDB: i32 = 0;
            let mut LWORK: i32 = 0;
            let mut M: i32 = 0;
            let mut N: i32 = 0;
            let mut P: i32 = 0;
            let mut C: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut D: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut WORK: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut X: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
            let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
            let mut arg_out: Arc<metamodelica::List<DAE::ExtArg>> = metamodelica::nil();
            let mut val_out: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (M, cache) = evaluateExtIntArg(arg_M.clone(), cache.clone(), env.clone())?;
            (N, cache) = evaluateExtIntArg(arg_N.clone(), cache.clone(), env.clone())?;
            (P, cache) = evaluateExtIntArg(arg_P.clone(), cache.clone(), env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(arg_A.clone(), cache.clone(), env.clone())?;
            (LDA, cache) = evaluateExtIntArg(arg_LDA.clone(), cache.clone(), env.clone())?;
            (B, cache) = evaluateExtRealMatrixArg(arg_B.clone(), cache.clone(), env.clone())?;
            (LDB, cache) = evaluateExtIntArg(arg_LDB.clone(), cache.clone(), env.clone())?;
            (C, cache) = evaluateExtRealArrayArg(arg_C.clone(), cache.clone(), env.clone())?;
            (D, cache) = evaluateExtRealArrayArg(arg_D.clone(), cache.clone(), env.clone())?;
            (WORK, cache) = evaluateExtRealArrayArg(arg_WORK.clone(), cache.clone(), env.clone())?;
            (LWORK, cache) = evaluateExtIntArg(arg_LWORK.clone(), cache.clone(), env.clone())?;
            (A, B, C, D, X, WORK, INFO) = Lapack::dgglse(M.clone(), N.clone(), P.clone(), A.clone(), LDA.clone(), B.clone(), LDB.clone(), C.clone(), D.clone(), WORK.clone(), LWORK.clone());
            val_A = ValuesMake::makeRealMatrix(A.clone())?;
            val_B = ValuesMake::makeRealMatrix(B.clone())?;
            val_C = ValuesMake::makeRealArray(C.clone())?;
            val_D = ValuesMake::makeRealArray(D.clone())?;
            val_X = ValuesMake::makeRealArray(X.clone())?;
            val_WORK = ValuesMake::makeRealArray(WORK.clone())?;
            val_INFO = ValuesMake::makeInteger(INFO.clone());
            arg_out = list![arg_A.clone(), arg_B.clone(), arg_C.clone(), arg_D.clone(), arg_X.clone(), arg_WORK.clone(), arg_INFO.clone()];
            val_out = list![val_A.clone(), val_B.clone(), val_C.clone(), val_D.clone(), val_X.clone(), val_WORK.clone(), val_INFO.clone()];
            (cache, env) = assignExtOutputs(arg_out.clone(), val_out.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        (Deref @ "dgtsv", Deref @ metamodelica::List::Cons { head: arg_N, tail: Deref @ metamodelica::List::Cons { head: arg_NRHS, tail: Deref @ metamodelica::List::Cons { head: arg_DL, tail: Deref @ metamodelica::List::Cons { head: arg_D, tail: Deref @ metamodelica::List::Cons { head: arg_DU, tail: Deref @ metamodelica::List::Cons { head: arg_B, tail: Deref @ metamodelica::List::Cons { head: arg_LDB, tail: Deref @ metamodelica::List::Cons { head: arg_INFO, tail: Deref @ metamodelica::List::Nil } } } } } } } }, cache, env) => {
            let mut val_INFO: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_D: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_DL: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_DU: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_B: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut INFO: i32 = 0;
            let mut LDB: i32 = 0;
            let mut N: i32 = 0;
            let mut NRHS: i32 = 0;
            let mut D: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut DL: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut DU: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
            let mut arg_out: Arc<metamodelica::List<DAE::ExtArg>> = metamodelica::nil();
            let mut val_out: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (N, cache) = evaluateExtIntArg(arg_N.clone(), cache.clone(), env.clone())?;
            (NRHS, cache) = evaluateExtIntArg(arg_NRHS.clone(), cache.clone(), env.clone())?;
            (DL, cache) = evaluateExtRealArrayArg(arg_DL.clone(), cache.clone(), env.clone())?;
            (D, cache) = evaluateExtRealArrayArg(arg_D.clone(), cache.clone(), env.clone())?;
            (DU, cache) = evaluateExtRealArrayArg(arg_DU.clone(), cache.clone(), env.clone())?;
            (B, cache) = evaluateExtRealMatrixArg(arg_B.clone(), cache.clone(), env.clone())?;
            (LDB, cache) = evaluateExtIntArg(arg_LDB.clone(), cache.clone(), env.clone())?;
            (DL, D, DU, B, INFO) = Lapack::dgtsv(N.clone(), NRHS.clone(), DL.clone(), D.clone(), DU.clone(), B.clone(), LDB.clone());
            val_DL = ValuesMake::makeRealArray(DL.clone())?;
            val_D = ValuesMake::makeRealArray(D.clone())?;
            val_DU = ValuesMake::makeRealArray(DU.clone())?;
            val_B = ValuesMake::makeRealMatrix(B.clone())?;
            val_INFO = ValuesMake::makeInteger(INFO.clone());
            arg_out = list![arg_DL.clone(), arg_D.clone(), arg_DU.clone(), arg_B.clone(), arg_INFO.clone()];
            val_out = list![val_DL.clone(), val_D.clone(), val_DU.clone(), val_B.clone(), val_INFO.clone()];
            (cache, env) = assignExtOutputs(arg_out.clone(), val_out.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        (Deref @ "dgbsv", Deref @ metamodelica::List::Cons { head: arg_N, tail: Deref @ metamodelica::List::Cons { head: arg_KL, tail: Deref @ metamodelica::List::Cons { head: arg_KU, tail: Deref @ metamodelica::List::Cons { head: arg_NRHS, tail: Deref @ metamodelica::List::Cons { head: arg_AB, tail: Deref @ metamodelica::List::Cons { head: arg_LDAB, tail: Deref @ metamodelica::List::Cons { head: arg_IPIV, tail: Deref @ metamodelica::List::Cons { head: arg_B, tail: Deref @ metamodelica::List::Cons { head: arg_LDB, tail: Deref @ metamodelica::List::Cons { head: arg_INFO, tail: Deref @ metamodelica::List::Nil } } } } } } } } } }, cache, env) => {
            let mut val_INFO: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_IPIV: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_AB: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_B: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut INFO: i32 = 0;
            let mut KL: i32 = 0;
            let mut KU: i32 = 0;
            let mut LDAB: i32 = 0;
            let mut LDB: i32 = 0;
            let mut N: i32 = 0;
            let mut NRHS: i32 = 0;
            let mut IPIV: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut AB: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
            let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
            let mut arg_out: Arc<metamodelica::List<DAE::ExtArg>> = metamodelica::nil();
            let mut val_out: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (N, cache) = evaluateExtIntArg(arg_N.clone(), cache.clone(), env.clone())?;
            (KL, cache) = evaluateExtIntArg(arg_KL.clone(), cache.clone(), env.clone())?;
            (KU, cache) = evaluateExtIntArg(arg_KU.clone(), cache.clone(), env.clone())?;
            (NRHS, cache) = evaluateExtIntArg(arg_NRHS.clone(), cache.clone(), env.clone())?;
            (AB, cache) = evaluateExtRealMatrixArg(arg_AB.clone(), cache.clone(), env.clone())?;
            (LDAB, cache) = evaluateExtIntArg(arg_LDAB.clone(), cache.clone(), env.clone())?;
            (B, cache) = evaluateExtRealMatrixArg(arg_B.clone(), cache.clone(), env.clone())?;
            (LDB, cache) = evaluateExtIntArg(arg_LDB.clone(), cache.clone(), env.clone())?;
            (AB, IPIV, B, INFO) = Lapack::dgbsv(N.clone(), KL.clone(), KU.clone(), NRHS.clone(), AB.clone(), LDAB.clone(), B.clone(), LDB.clone());
            val_AB = ValuesMake::makeRealMatrix(AB.clone())?;
            val_IPIV = ValuesMake::makeIntArray(IPIV.clone())?;
            val_B = ValuesMake::makeRealMatrix(B.clone())?;
            val_INFO = ValuesMake::makeInteger(INFO.clone());
            arg_out = list![arg_AB.clone(), arg_IPIV.clone(), arg_B.clone(), arg_INFO.clone()];
            val_out = list![val_AB.clone(), val_IPIV.clone(), val_B.clone(), val_INFO.clone()];
            (cache, env) = assignExtOutputs(arg_out.clone(), val_out.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        (Deref @ "dgesvd", Deref @ metamodelica::List::Cons { head: arg_JOBU, tail: Deref @ metamodelica::List::Cons { head: arg_JOBVT, tail: Deref @ metamodelica::List::Cons { head: arg_M, tail: Deref @ metamodelica::List::Cons { head: arg_N, tail: Deref @ metamodelica::List::Cons { head: arg_A, tail: Deref @ metamodelica::List::Cons { head: arg_LDA, tail: Deref @ metamodelica::List::Cons { head: arg_S, tail: Deref @ metamodelica::List::Cons { head: arg_U, tail: Deref @ metamodelica::List::Cons { head: arg_LDU, tail: Deref @ metamodelica::List::Cons { head: arg_VT, tail: Deref @ metamodelica::List::Cons { head: arg_LDVT, tail: Deref @ metamodelica::List::Cons { head: arg_WORK, tail: Deref @ metamodelica::List::Cons { head: arg_LWORK, tail: Deref @ metamodelica::List::Cons { head: arg_INFO, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } } }, cache, env) => {
            let mut val_INFO: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_WORK: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_A: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_S: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_U: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_VT: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut INFO: i32 = 0;
            let mut LDA: i32 = 0;
            let mut LDU: i32 = 0;
            let mut LDVT: i32 = 0;
            let mut LWORK: i32 = 0;
            let mut M: i32 = 0;
            let mut N: i32 = 0;
            let mut JOBU: ArcStr = arcstr::literal!("");
            let mut JOBVT: ArcStr = arcstr::literal!("");
            let mut WORK: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut S: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
            let mut U: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
            let mut VT: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
            let mut arg_out: Arc<metamodelica::List<DAE::ExtArg>> = metamodelica::nil();
            let mut val_out: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (JOBU, cache) = evaluateExtStringArg(arg_JOBU.clone(), cache.clone(), env.clone())?;
            (JOBVT, cache) = evaluateExtStringArg(arg_JOBVT.clone(), cache.clone(), env.clone())?;
            (M, cache) = evaluateExtIntArg(arg_M.clone(), cache.clone(), env.clone())?;
            (N, cache) = evaluateExtIntArg(arg_N.clone(), cache.clone(), env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(arg_A.clone(), cache.clone(), env.clone())?;
            (LDA, cache) = evaluateExtIntArg(arg_LDA.clone(), cache.clone(), env.clone())?;
            (LDU, cache) = evaluateExtIntArg(arg_LDU.clone(), cache.clone(), env.clone())?;
            (LDVT, cache) = evaluateExtIntArg(arg_LDVT.clone(), cache.clone(), env.clone())?;
            (WORK, cache) = evaluateExtRealArrayArg(arg_WORK.clone(), cache.clone(), env.clone())?;
            (LWORK, cache) = evaluateExtIntArg(arg_LWORK.clone(), cache.clone(), env.clone())?;
            (A, S, U, VT, WORK, INFO) = Lapack::dgesvd((JOBU.clone()).clone(), (JOBVT.clone()).clone(), M.clone(), N.clone(), A.clone(), LDA.clone(), LDU.clone(), LDVT.clone(), WORK.clone(), LWORK.clone());
            val_A = ValuesMake::makeRealMatrix(A.clone())?;
            val_S = ValuesMake::makeRealArray(S.clone())?;
            val_U = ValuesMake::makeRealMatrix(U.clone())?;
            val_VT = ValuesMake::makeRealMatrix(VT.clone())?;
            val_WORK = ValuesMake::makeRealArray(WORK.clone())?;
            val_INFO = ValuesMake::makeInteger(INFO.clone());
            arg_out = list![arg_A.clone(), arg_S.clone(), arg_U.clone(), arg_VT.clone(), arg_WORK.clone(), arg_INFO.clone()];
            val_out = list![val_A.clone(), val_S.clone(), val_U.clone(), val_VT.clone(), val_WORK.clone(), val_INFO.clone()];
            (cache, env) = assignExtOutputs(arg_out.clone(), val_out.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        (Deref @ "dgetrf", Deref @ metamodelica::List::Cons { head: arg_M, tail: Deref @ metamodelica::List::Cons { head: arg_N, tail: Deref @ metamodelica::List::Cons { head: arg_A, tail: Deref @ metamodelica::List::Cons { head: arg_LDA, tail: Deref @ metamodelica::List::Cons { head: arg_IPIV, tail: Deref @ metamodelica::List::Cons { head: arg_INFO, tail: Deref @ metamodelica::List::Nil } } } } } }, cache, env) => {
            let mut val_INFO: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_IPIV: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_A: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut INFO: i32 = 0;
            let mut LDA: i32 = 0;
            let mut M: i32 = 0;
            let mut N: i32 = 0;
            let mut IPIV: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
            let mut arg_out: Arc<metamodelica::List<DAE::ExtArg>> = metamodelica::nil();
            let mut val_out: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (M, cache) = evaluateExtIntArg(arg_M.clone(), cache.clone(), env.clone())?;
            (N, cache) = evaluateExtIntArg(arg_N.clone(), cache.clone(), env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(arg_A.clone(), cache.clone(), env.clone())?;
            (LDA, cache) = evaluateExtIntArg(arg_LDA.clone(), cache.clone(), env.clone())?;
            (A, IPIV, INFO) = Lapack::dgetrf(M.clone(), N.clone(), A.clone(), LDA.clone());
            val_A = ValuesMake::makeRealMatrix(A.clone())?;
            val_IPIV = ValuesMake::makeIntArray(IPIV.clone())?;
            val_INFO = ValuesMake::makeInteger(INFO.clone());
            arg_out = list![arg_A.clone(), arg_IPIV.clone(), arg_INFO.clone()];
            val_out = list![val_A.clone(), val_IPIV.clone(), val_INFO.clone()];
            (cache, env) = assignExtOutputs(arg_out.clone(), val_out.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        (Deref @ "dgetrs", Deref @ metamodelica::List::Cons { head: arg_TRANS, tail: Deref @ metamodelica::List::Cons { head: arg_N, tail: Deref @ metamodelica::List::Cons { head: arg_NRHS, tail: Deref @ metamodelica::List::Cons { head: arg_A, tail: Deref @ metamodelica::List::Cons { head: arg_LDA, tail: Deref @ metamodelica::List::Cons { head: arg_IPIV, tail: Deref @ metamodelica::List::Cons { head: arg_B, tail: Deref @ metamodelica::List::Cons { head: arg_LDB, tail: Deref @ metamodelica::List::Cons { head: arg_INFO, tail: Deref @ metamodelica::List::Nil } } } } } } } } }, cache, env) => {
            let mut val_INFO: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_B: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut INFO: i32 = 0;
            let mut LDA: i32 = 0;
            let mut LDB: i32 = 0;
            let mut N: i32 = 0;
            let mut NRHS: i32 = 0;
            let mut TRANS: ArcStr = arcstr::literal!("");
            let mut IPIV: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
            let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
            let mut arg_out: Arc<metamodelica::List<DAE::ExtArg>> = metamodelica::nil();
            let mut val_out: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (TRANS, cache) = evaluateExtStringArg(arg_TRANS.clone(), cache.clone(), env.clone())?;
            (N, cache) = evaluateExtIntArg(arg_N.clone(), cache.clone(), env.clone())?;
            (NRHS, cache) = evaluateExtIntArg(arg_NRHS.clone(), cache.clone(), env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(arg_A.clone(), cache.clone(), env.clone())?;
            (LDA, cache) = evaluateExtIntArg(arg_LDA.clone(), cache.clone(), env.clone())?;
            (IPIV, cache) = evaluateExtIntArrayArg(arg_IPIV.clone(), cache.clone(), env.clone())?;
            (B, cache) = evaluateExtRealMatrixArg(arg_B.clone(), cache.clone(), env.clone())?;
            (LDB, cache) = evaluateExtIntArg(arg_LDB.clone(), cache.clone(), env.clone())?;
            (B, INFO) = Lapack::dgetrs((TRANS.clone()).clone(), N.clone(), NRHS.clone(), A.clone(), LDA.clone(), IPIV.clone(), B.clone(), LDB.clone());
            val_B = ValuesMake::makeRealMatrix(B.clone())?;
            val_INFO = ValuesMake::makeInteger(INFO.clone());
            arg_out = list![arg_B.clone(), arg_INFO.clone()];
            val_out = list![val_B.clone(), val_INFO.clone()];
            (cache, env) = assignExtOutputs(arg_out.clone(), val_out.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        (Deref @ "dgetri", Deref @ metamodelica::List::Cons { head: arg_N, tail: Deref @ metamodelica::List::Cons { head: arg_A, tail: Deref @ metamodelica::List::Cons { head: arg_LDA, tail: Deref @ metamodelica::List::Cons { head: arg_IPIV, tail: Deref @ metamodelica::List::Cons { head: arg_WORK, tail: Deref @ metamodelica::List::Cons { head: arg_LWORK, tail: Deref @ metamodelica::List::Cons { head: arg_INFO, tail: Deref @ metamodelica::List::Nil } } } } } } }, cache, env) => {
            let mut val_INFO: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_WORK: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_A: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut INFO: i32 = 0;
            let mut LDA: i32 = 0;
            let mut LWORK: i32 = 0;
            let mut N: i32 = 0;
            let mut IPIV: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut WORK: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
            let mut arg_out: Arc<metamodelica::List<DAE::ExtArg>> = metamodelica::nil();
            let mut val_out: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (N, cache) = evaluateExtIntArg(arg_N.clone(), cache.clone(), env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(arg_A.clone(), cache.clone(), env.clone())?;
            (LDA, cache) = evaluateExtIntArg(arg_LDA.clone(), cache.clone(), env.clone())?;
            (IPIV, cache) = evaluateExtIntArrayArg(arg_IPIV.clone(), cache.clone(), env.clone())?;
            (WORK, cache) = evaluateExtRealArrayArg(arg_WORK.clone(), cache.clone(), env.clone())?;
            (LWORK, cache) = evaluateExtIntArg(arg_LWORK.clone(), cache.clone(), env.clone())?;
            (A, WORK, INFO) = Lapack::dgetri(N.clone(), A.clone(), LDA.clone(), IPIV.clone(), WORK.clone(), LWORK.clone());
            val_A = ValuesMake::makeRealMatrix(A.clone())?;
            val_WORK = ValuesMake::makeRealArray(WORK.clone())?;
            val_INFO = ValuesMake::makeInteger(INFO.clone());
            arg_out = list![arg_A.clone(), arg_WORK.clone(), arg_INFO.clone()];
            val_out = list![val_A.clone(), val_WORK.clone(), val_INFO.clone()];
            (cache, env) = assignExtOutputs(arg_out.clone(), val_out.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        (Deref @ "dgeqpf", Deref @ metamodelica::List::Cons { head: arg_M, tail: Deref @ metamodelica::List::Cons { head: arg_N, tail: Deref @ metamodelica::List::Cons { head: arg_A, tail: Deref @ metamodelica::List::Cons { head: arg_LDA, tail: Deref @ metamodelica::List::Cons { head: arg_JPVT, tail: Deref @ metamodelica::List::Cons { head: arg_TAU, tail: Deref @ metamodelica::List::Cons { head: arg_WORK, tail: Deref @ metamodelica::List::Cons { head: arg_INFO, tail: Deref @ metamodelica::List::Nil } } } } } } } }, cache, env) => {
            let mut val_INFO: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_JPVT: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_TAU: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_A: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut INFO: i32 = 0;
            let mut LDA: i32 = 0;
            let mut M: i32 = 0;
            let mut N: i32 = 0;
            let mut JPVT: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut TAU: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut WORK: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
            let mut arg_out: Arc<metamodelica::List<DAE::ExtArg>> = metamodelica::nil();
            let mut val_out: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (M, cache) = evaluateExtIntArg(arg_M.clone(), cache.clone(), env.clone())?;
            (N, cache) = evaluateExtIntArg(arg_N.clone(), cache.clone(), env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(arg_A.clone(), cache.clone(), env.clone())?;
            (LDA, cache) = evaluateExtIntArg(arg_LDA.clone(), cache.clone(), env.clone())?;
            (JPVT, cache) = evaluateExtIntArrayArg(arg_JPVT.clone(), cache.clone(), env.clone())?;
            (WORK, cache) = evaluateExtRealArrayArg(arg_WORK.clone(), cache.clone(), env.clone())?;
            (A, JPVT, TAU, INFO) = Lapack::dgeqpf(M.clone(), N.clone(), A.clone(), LDA.clone(), JPVT.clone(), WORK.clone());
            val_A = ValuesMake::makeRealMatrix(A.clone())?;
            val_JPVT = ValuesMake::makeIntArray(JPVT.clone())?;
            val_TAU = ValuesMake::makeRealArray(TAU.clone())?;
            val_INFO = ValuesMake::makeInteger(INFO.clone());
            arg_out = list![arg_A.clone(), arg_JPVT.clone(), arg_TAU.clone(), arg_INFO.clone()];
            val_out = list![val_A.clone(), val_JPVT.clone(), val_TAU.clone(), val_INFO.clone()];
            (cache, env) = assignExtOutputs(arg_out.clone(), val_out.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        (Deref @ "dorgqr", Deref @ metamodelica::List::Cons { head: arg_M, tail: Deref @ metamodelica::List::Cons { head: arg_N, tail: Deref @ metamodelica::List::Cons { head: arg_K, tail: Deref @ metamodelica::List::Cons { head: arg_A, tail: Deref @ metamodelica::List::Cons { head: arg_LDA, tail: Deref @ metamodelica::List::Cons { head: arg_TAU, tail: Deref @ metamodelica::List::Cons { head: arg_WORK, tail: Deref @ metamodelica::List::Cons { head: arg_LWORK, tail: Deref @ metamodelica::List::Cons { head: arg_INFO, tail: Deref @ metamodelica::List::Nil } } } } } } } } }, cache, env) => {
            let mut val_INFO: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_WORK: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut val_A: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut INFO: i32 = 0;
            let mut K: i32 = 0;
            let mut LDA: i32 = 0;
            let mut LWORK: i32 = 0;
            let mut M: i32 = 0;
            let mut N: i32 = 0;
            let mut TAU: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut WORK: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
            let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>> = metamodelica::nil();
            let mut arg_out: Arc<metamodelica::List<DAE::ExtArg>> = metamodelica::nil();
            let mut val_out: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (M, cache) = evaluateExtIntArg(arg_M.clone(), cache.clone(), env.clone())?;
            (N, cache) = evaluateExtIntArg(arg_N.clone(), cache.clone(), env.clone())?;
            (K, cache) = evaluateExtIntArg(arg_K.clone(), cache.clone(), env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(arg_A.clone(), cache.clone(), env.clone())?;
            (LDA, cache) = evaluateExtIntArg(arg_LDA.clone(), cache.clone(), env.clone())?;
            (TAU, cache) = evaluateExtRealArrayArg(arg_TAU.clone(), cache.clone(), env.clone())?;
            (WORK, cache) = evaluateExtRealArrayArg(arg_WORK.clone(), cache.clone(), env.clone())?;
            (LWORK, cache) = evaluateExtIntArg(arg_LWORK.clone(), cache.clone(), env.clone())?;
            (A, WORK, INFO) = Lapack::dorgqr(M.clone(), N.clone(), K.clone(), A.clone(), LDA.clone(), TAU.clone(), WORK.clone(), LWORK.clone());
            val_A = ValuesMake::makeRealMatrix(A.clone())?;
            val_WORK = ValuesMake::makeRealArray(WORK.clone())?;
            val_INFO = ValuesMake::makeInteger(INFO.clone());
            arg_out = list![arg_A.clone(), arg_WORK.clone(), arg_INFO.clone()];
            val_out = list![val_A.clone(), val_WORK.clone(), val_INFO.clone()];
            (cache, env) = assignExtOutputs(arg_out.clone(), val_out.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outEnv))
}

fn evaluateElements(mut inElements: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inLoopControl: LoopControl) -> Result<(FCore::Cache, FCore::Graph, LoopControl)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outLoopControl: LoopControl = LoopControl::BREAK;
    (outCache, outEnv, outLoopControl) = (::match_deref::match_deref! { match &((inElements.clone(), inCache.clone(), inEnv.clone(), inLoopControl.clone())) {
        (_, _, _, LoopControl::RETURN { .. }) => {
            (inCache.clone(), inEnv.clone(), inLoopControl.clone())
        },
        (Deref @ metamodelica::List::Nil, _, _, _) => {
            (inCache.clone(), inEnv.clone(), crate::CevalFunction::LoopControl::NEXT)
        },
        (Deref @ metamodelica::List::Cons { head: elem, tail: rest_elems }, _, _, _) => {
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            let mut env: FCore::Graph;
            let mut loop_ctrl: LoopControl = LoopControl::BREAK;
            (cache, env, loop_ctrl) = evaluateElement(elem.clone(), inCache.clone(), inEnv.clone())?;
            (cache, env, loop_ctrl) = evaluateElements(rest_elems.clone(), cache.clone(), env.clone(), loop_ctrl.clone())?;
            (cache.clone(), env.clone(), loop_ctrl.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outEnv, outLoopControl))
}

fn evaluateElement(mut inElement: Arc<DAE::Element>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, FCore::Graph, LoopControl)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outLoopControl: LoopControl = LoopControl::BREAK;
    (outCache, outEnv, outLoopControl) = (::match_deref::match_deref! { match &((inElement.clone(), inCache.clone(), inEnv.clone())) {
        (Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: sl }, .. }, _, _) => {
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            let mut env: FCore::Graph;
            let mut loop_ctrl: LoopControl = LoopControl::BREAK;
            let mut sl = (*sl).clone();
            let (__pa0, (_, __pa1)) = DAEUtil::traverseDAEEquationsStmts(sl.clone(), Arc::new(Expression::traverseSubexpressionsHelper), (optimizeExpTraverser, inEnv.clone()));
            sl = __pa0.clone();
            env = __pa1.clone();
            (cache, env, loop_ctrl) = evaluateStatements(sl.clone(), inCache.clone(), env.clone())?;
            (cache.clone(), env.clone(), loop_ctrl.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outEnv, outLoopControl))
}

fn evaluateStatement(mut inStatement: Arc<DAE::Statement>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, FCore::Graph, LoopControl)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outLoopControl: LoopControl = LoopControl::BREAK;
    (outCache, outEnv, outLoopControl) = (::match_deref::match_deref! { match &((inStatement.clone(), inCache.clone(), inEnv.clone())) {
        (Deref @ DAE::Statement::STMT_ASSIGN { exp: rhs, exp1: lhs, .. }, cache, env) => {
            let mut lhs_cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut rhs_val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (cache, rhs_val) = cevalExp(rhs.clone(), cache.clone(), env.clone())?;
            lhs_cref = extractLhsComponentRef(lhs.clone())?;
            (cache, env) = assignVariable(lhs_cref.clone(), rhs_val.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone(), crate::CevalFunction::LoopControl::NEXT)
        },
        (Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { .. }, _, _) => {
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            let mut env: FCore::Graph;
            (cache, env) = evaluateTupleAssignStatement(inStatement.clone(), inCache.clone(), inEnv.clone())?;
            (cache.clone(), env.clone(), crate::CevalFunction::LoopControl::NEXT)
        },
        (Deref @ DAE::Statement::STMT_ASSIGN_ARR { exp: rhs, lhs, .. }, _, env) => {
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            let mut lhs_cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut rhs_val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut env = (*env).clone();
            (cache, rhs_val) = cevalExp(rhs.clone(), inCache.clone(), env.clone())?;
            lhs_cref = extractLhsComponentRef(lhs.clone())?;
            (cache, env) = assignVariable(lhs_cref.clone(), rhs_val.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone(), crate::CevalFunction::LoopControl::NEXT)
        },
        (Deref @ DAE::Statement::STMT_IF { .. }, _, _) => {
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            let mut env: FCore::Graph;
            let mut loop_ctrl: LoopControl = LoopControl::BREAK;
            (cache, env, loop_ctrl) = evaluateIfStatement(inStatement.clone(), inCache.clone(), inEnv.clone())?;
            (cache.clone(), env.clone(), loop_ctrl.clone())
        },
        (Deref @ DAE::Statement::STMT_FOR { .. }, _, _) => {
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            let mut env: FCore::Graph;
            let mut loop_ctrl: LoopControl = LoopControl::BREAK;
            (cache, env, loop_ctrl) = evaluateForStatement(inStatement.clone(), inCache.clone(), inEnv.clone())?;
            (cache.clone(), env.clone(), loop_ctrl.clone())
        },
        (Deref @ DAE::Statement::STMT_WHILE { statementLst: statements, exp: condition, .. }, _, _) => {
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            let mut env: FCore::Graph;
            let mut loop_ctrl: LoopControl = LoopControl::BREAK;
            (cache, env, loop_ctrl) = evaluateWhileStatement(condition.clone(), statements.clone(), inCache.clone(), inEnv.clone(), crate::CevalFunction::LoopControl::NEXT)?;
            (cache.clone(), env.clone(), loop_ctrl.clone())
        },
        (Deref @ DAE::Statement::STMT_ASSERT { cond: condition, .. }, _, _) => {
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            let __pa0 = ::match_deref::match_deref! { match &(cevalExp(condition.clone(), inCache.clone(), inEnv.clone())?) {
                (__pa0, Deref @ Values::Value::BOOL { boolean: true }) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            (cache.clone(), inEnv.clone(), crate::CevalFunction::LoopControl::NEXT)
        },
        (Deref @ DAE::Statement::STMT_ASSERT { cond: condition, .. }, _, _) => {
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            let __pa0 = ::match_deref::match_deref! { match &(cevalExp(condition.clone(), inCache.clone(), inEnv.clone())?) {
                (__pa0, Deref @ Values::Value::BOOL { boolean: true }) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            (cache.clone(), inEnv.clone(), crate::CevalFunction::LoopControl::NEXT)
        },
        (Deref @ DAE::Statement::STMT_NORETCALL { exp: rhs @ Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { tailCall, .. }, expLst: exps, .. }, .. }, _, _) => {
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            let mut env: FCore::Graph;
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut var: ArcStr = arcstr::literal!("");
            let mut vars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            (cache, vals) = cevalExpList(exps.clone(), inCache.clone(), inEnv.clone())?;
            (cache, v) = cevalExp(rhs.clone(), cache.clone(), inEnv.clone())?;
            (cache, env, outLoopControl) = (::match_deref::match_deref! { match &(tailCall.clone()) {
        DAE::TailCall::NO_TAIL => (cache.clone(), inEnv.clone(), crate::CevalFunction::LoopControl::NEXT),
        DAE::TailCall::TAIL { outVars: Deref @ metamodelica::List::Nil, .. } => (cache.clone(), inEnv.clone(), crate::CevalFunction::LoopControl::RETURN),
        DAE::TailCall::TAIL { outVars: Deref @ metamodelica::List::Cons { head: var, tail: Deref @ metamodelica::List::Nil }, .. } => {
            (cache, env) = assignVariable(ComponentReference::makeUntypedCrefIdent((var.clone()).clone()), v.clone(), cache.clone(), inEnv.clone())?;
            (cache.clone(), env.clone(), crate::CevalFunction::LoopControl::RETURN)
        },
        DAE::TailCall::TAIL { outVars: vars, .. } => {
            let mut vars = (*vars).clone();
            let __pa0 = ::match_deref::match_deref! { match &(v.clone()) {
                Deref @ Values::Value::TUPLE { valueLst: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            vals = __pa0.clone();
            for mut val in &*vals.clone() {
                let mut val = val.clone();
                let (__pa1, __pa2) = ::match_deref::match_deref! { match &(vars.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa1, tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                var = __pa1.clone();
                vars = __pa2.clone();
                (cache, env) = assignVariable(ComponentReference::makeUntypedCrefIdent((var.clone()).clone()), val.clone(), cache.clone(), inEnv.clone())?;
            }
            (cache.clone(), env.clone(), crate::CevalFunction::LoopControl::RETURN)
        },
        _ => bail!("match: no arm matched"),
    } });
            (cache.clone(), env.clone(), crate::CevalFunction::LoopControl::NEXT)
        },
        (Deref @ DAE::Statement::STMT_RETURN { .. }, _, _) => {
            (inCache.clone(), inEnv.clone(), crate::CevalFunction::LoopControl::RETURN)
        },
        (Deref @ DAE::Statement::STMT_BREAK { .. }, _, _) => {
            (inCache.clone(), inEnv.clone(), crate::CevalFunction::LoopControl::BREAK)
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln((literal!("- CevalFunction.evaluateStatement failed for:")).clone())?;
            Debug::traceln((DAEDump::ppStatementStr(inStatement.clone())?).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outEnv, outLoopControl))
}

fn evaluateStatements(mut inStatement: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, FCore::Graph, LoopControl)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outLoopControl: LoopControl = LoopControl::BREAK;
    (outCache, outEnv, outLoopControl) = evaluateStatements2(inStatement.clone(), inCache.clone(), inEnv.clone(), crate::CevalFunction::LoopControl::NEXT)?;
    Ok((outCache, outEnv, outLoopControl))
}

fn evaluateStatements2(mut inStatement: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inLoopControl: LoopControl) -> Result<(FCore::Cache, FCore::Graph, LoopControl)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outLoopControl: LoopControl = LoopControl::BREAK;
    (outCache, outEnv, outLoopControl) = (::match_deref::match_deref! { match &((inStatement.clone(), inCache.clone(), inEnv.clone(), inLoopControl.clone())) {
        (_, _, _, LoopControl::BREAK { .. }) => {
            (inCache.clone(), inEnv.clone(), inLoopControl.clone())
        },
        (_, _, _, LoopControl::RETURN { .. }) => {
            (inCache.clone(), inEnv.clone(), inLoopControl.clone())
        },
        (Deref @ metamodelica::List::Nil, _, _, _) => {
            (inCache.clone(), inEnv.clone(), inLoopControl.clone())
        },
        (Deref @ metamodelica::List::Cons { head: stmt, tail: rest_stmts }, _, _, LoopControl::NEXT { .. }) => {
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            let mut env: FCore::Graph;
            let mut loop_ctrl: LoopControl = LoopControl::BREAK;
            (cache, env, loop_ctrl) = evaluateStatement(stmt.clone(), inCache.clone(), inEnv.clone())?;
            (cache, env, loop_ctrl) = evaluateStatements2(rest_stmts.clone(), cache.clone(), env.clone(), loop_ctrl.clone())?;
            (cache.clone(), env.clone(), loop_ctrl.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outEnv, outLoopControl))
}

fn evaluateTupleAssignStatement(mut inStatement: Arc<DAE::Statement>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    (outCache, outEnv) = (::match_deref::match_deref! { match &((inStatement.clone(), inCache.clone(), inEnv.clone())) {
        (Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { exp: rhs, expExpLst: lhs_expl, .. }, _, env) => {
            let mut rhs_vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut lhs_crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            let mut env = (*env).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(cevalExp(rhs.clone(), inCache.clone(), env.clone())?) {
                (__pa0, Deref @ Values::Value::TUPLE { valueLst: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            rhs_vals = __pa1.clone();
            lhs_crefs = List::map(lhs_expl.clone(), Arc::new(extractLhsComponentRef));
            (cache, env) = assignTuple(lhs_crefs.clone(), rhs_vals.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outEnv))
}

fn evaluateIfStatement(mut inStatement: Arc<DAE::Statement>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, FCore::Graph, LoopControl)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outLoopControl: LoopControl = LoopControl::BREAK;
    (outCache, outEnv, outLoopControl) = (::match_deref::match_deref! { match &((inStatement.clone(), inCache.clone(), inEnv.clone())) {
        (Deref @ DAE::Statement::STMT_IF { else_: else_branch, statementLst: stmts, exp: cond, .. }, _, _) => {
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            let mut env: FCore::Graph;
            let mut bool_cond: bool = false;
            let mut loop_ctrl: LoopControl = LoopControl::BREAK;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(cevalExp(cond.clone(), inCache.clone(), inEnv.clone())?) {
                (__pa0, Deref @ Values::Value::BOOL { boolean: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            bool_cond = __pa1.clone();
            (cache, env, loop_ctrl) = evaluateIfStatement2(bool_cond.clone(), stmts.clone(), else_branch.clone(), cache.clone(), inEnv.clone())?;
            (cache.clone(), env.clone(), loop_ctrl.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outEnv, outLoopControl))
}

fn evaluateIfStatement2(mut inCondition: bool, mut inStatements: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inElse: Arc<DAE::Else>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, FCore::Graph, LoopControl)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outLoopControl: LoopControl = LoopControl::BREAK;
    (outCache, outEnv, outLoopControl) = (::match_deref::match_deref! { match &((inCondition.clone(), inStatements.clone(), inElse.clone(), inCache.clone(), inEnv.clone())) {
        (true, statements, _, _, env) => {
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            let mut loop_ctrl: LoopControl = LoopControl::BREAK;
            let mut env = (*env).clone();
            (cache, env, loop_ctrl) = evaluateStatements(statements.clone(), inCache.clone(), env.clone())?;
            (cache.clone(), env.clone(), loop_ctrl.clone())
        },
        (false, _, Deref @ DAE::Else::ELSE { statementLst: statements }, _, env) => {
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            let mut loop_ctrl: LoopControl = LoopControl::BREAK;
            let mut env = (*env).clone();
            (cache, env, loop_ctrl) = evaluateStatements(statements.clone(), inCache.clone(), env.clone())?;
            (cache.clone(), env.clone(), loop_ctrl.clone())
        },
        (false, _, Deref @ DAE::Else::ELSEIF { else_: else_branch, statementLst: statements, exp: condition }, _, env) => {
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            let mut bool_condition: bool = false;
            let mut loop_ctrl: LoopControl = LoopControl::BREAK;
            let mut env = (*env).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(cevalExp(condition.clone(), inCache.clone(), env.clone())?) {
                (__pa0, Deref @ Values::Value::BOOL { boolean: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            bool_condition = __pa1.clone();
            (cache, env, loop_ctrl) = evaluateIfStatement2(bool_condition.clone(), statements.clone(), else_branch.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone(), loop_ctrl.clone())
        },
        (false, _, Deref @ DAE::Else::NOELSE, _, _) => {
            (inCache.clone(), inEnv.clone(), crate::CevalFunction::LoopControl::NEXT)
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outEnv, outLoopControl))
}

fn evaluateForStatement(mut inStatement: Arc<DAE::Statement>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, FCore::Graph, LoopControl)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outLoopControl: LoopControl = LoopControl::BREAK;
    (outCache, outEnv, outLoopControl) = 'mc: {
        let __mc_input = (inStatement.clone(), inCache.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_FOR { statementLst: statements, range, iter: iter_name, type_: ety, .. }, _, env) => {
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut range_vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut iter_cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut loop_ctrl: LoopControl = LoopControl::BREAK;
                    let mut env = (*env).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(cevalExp(range.clone(), inCache.clone(), env.clone())?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, .. }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    range_vals = __pa1.clone();
                    (env, ty, iter_cr) = extendEnvWithForScope((iter_name.clone()).clone(), ety.clone(), env.clone())?;
                    (cache, env, loop_ctrl) = evaluateForLoopArray(cache.clone(), env.clone(), iter_cr.clone(), ty.clone(), range_vals.clone(), statements.clone(), crate::CevalFunction::LoopControl::NEXT)?;
                    Ok((cache.clone(), env.clone(), loop_ctrl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_FOR { range, .. }, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln((literal!("- evaluateForStatement not implemented for:")).clone())?;
                    Debug::traceln((ExpressionBasics::printExpStr(range.clone())?).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outLoopControl))
}

fn evaluateForLoopArray(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIter: Arc<DAE::ComponentRef>, mut inIterType: Arc<DAE::Type>, mut inValues: Arc<metamodelica::List<Arc<Values::Value>>>, mut inStatements: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inLoopControl: LoopControl) -> Result<(FCore::Cache, FCore::Graph, LoopControl)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outLoopControl: LoopControl = LoopControl::BREAK;
    (outCache, outEnv, outLoopControl) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inIter.clone(), inIterType.clone(), inValues.clone(), inStatements.clone(), inLoopControl.clone())) {
        (_, _, _, _, _, _, LoopControl::BREAK { .. }) => {
            (inCache.clone(), inEnv.clone(), crate::CevalFunction::LoopControl::NEXT)
        },
        (_, _, _, _, _, _, LoopControl::RETURN { .. }) => {
            (inCache.clone(), inEnv.clone(), inLoopControl.clone())
        },
        (_, _, _, _, Deref @ metamodelica::List::Nil, _, _) => {
            (inCache.clone(), inEnv.clone(), inLoopControl.clone())
        },
        (_, env, _, _, Deref @ metamodelica::List::Cons { head: value, tail: rest_vals }, _, LoopControl::NEXT { .. }) => {
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            let mut loop_ctrl: LoopControl = LoopControl::BREAK;
            let mut env = (*env).clone();
            env = updateVariableBinding(inIter.clone(), env.clone(), inIterType.clone(), value.clone())?;
            (cache, env, loop_ctrl) = evaluateStatements(inStatements.clone(), inCache.clone(), env.clone())?;
            (cache, env, loop_ctrl) = evaluateForLoopArray(cache.clone(), env.clone(), inIter.clone(), inIterType.clone(), rest_vals.clone(), inStatements.clone(), loop_ctrl.clone())?;
            (cache.clone(), env.clone(), loop_ctrl.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outEnv, outLoopControl))
}

fn evaluateWhileStatement(mut inCondition: Arc<DAE::Exp>, mut inStatements: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inLoopControl: LoopControl) -> Result<(FCore::Cache, FCore::Graph, LoopControl)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outLoopControl: LoopControl = LoopControl::BREAK;
    (outCache, outEnv, outLoopControl) = (::match_deref::match_deref! { match &((inCondition.clone(), inStatements.clone(), inCache.clone(), inEnv.clone(), inLoopControl.clone())) {
        (_, _, _, _, LoopControl::BREAK { .. }) => {
            (inCache.clone(), inEnv.clone(), crate::CevalFunction::LoopControl::NEXT)
        },
        (_, _, _, _, LoopControl::RETURN { .. }) => {
            (inCache.clone(), inEnv.clone(), inLoopControl.clone())
        },
        (_, _, _, _, _) => {
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            let mut env: FCore::Graph;
            let mut loop_ctrl: LoopControl = LoopControl::BREAK;
            let mut b: bool = false;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(cevalExp(inCondition.clone(), inCache.clone(), inEnv.clone())?) {
                (__pa0, Deref @ Values::Value::BOOL { boolean: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            b = __pa1.clone();
            if b.clone() {
                (cache, env, loop_ctrl) = evaluateStatements(inStatements.clone(), cache.clone(), inEnv.clone())?;
                (cache, env, loop_ctrl) = evaluateWhileStatement(inCondition.clone(), inStatements.clone(), cache.clone(), env.clone(), loop_ctrl.clone())?;
            } else {
                loop_ctrl = crate::CevalFunction::LoopControl::NEXT;
                env = inEnv.clone();
            }
            (cache.clone(), env.clone(), loop_ctrl.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outEnv, outLoopControl))
}

fn extractLhsComponentRef(mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    outCref = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: cref, .. } => {
            cref.clone()
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- CevalFunction.extractLhsComponentRef failed on ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outCref)
}

fn cevalExp(mut inExp: Arc<DAE::Exp>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = Ceval::ceval(inCache.clone(), inEnv.clone(), inExp.clone(), true, Absyn::Msg::MSG { info: Absyn::dummyInfo.clone() }, 0)?;
    let false = (Arc::new(openmodelica_frontend_types::Values::Value::META_FAIL) == outValue.clone()) else { bail!("pattern mismatch") };
    Ok((outCache, outValue))
}

fn cevalExpList(mut inExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<Values::Value>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    (outCache, outValue) = Ceval::cevalList(inCache.clone(), inEnv.clone(), inExpLst.clone(), true, Absyn::Msg::MSG { info: Absyn::dummyInfo.clone() }, 0)?;
    Ok((outCache, outValue))
}

// [EENV]  Environment extension functions (add variables).
fn setupFunctionEnvironment(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inFuncName: ArcStr, mut inFuncParams: Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    outEnv = FGraph::openScope(inEnv.clone(), openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, (inFuncName.clone()).clone(), Some(crate::FCore::ScopeType::FUNCTION_SCOPE))?;
    (outCache, outEnv) = extendEnvWithFunctionVars(inCache.clone(), outEnv.clone(), inFuncParams.clone())?;
    Ok((outCache, outEnv))
}

fn extendEnvWithFunctionVars(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inFuncParams: Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    (outCache, outEnv) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inFuncParams.clone())) {
        (_, _, Deref @ metamodelica::List::Nil) => {
            (inCache.clone(), inEnv.clone())
        },
        (cache, env, Deref @ metamodelica::List::Cons { head: param, tail: rest_params }) => {
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (cache, env) = extendEnvWithFunctionVar(cache.clone(), env.clone(), param.clone())?;
            (cache, env) = extendEnvWithFunctionVars(cache.clone(), env.clone(), rest_params.clone())?;
            (cache.clone(), env.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outEnv))
}

fn extendEnvWithFunctionVar(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inFuncParam: FunctionVar) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    (outCache, outEnv) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inFuncParam.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, env, (e, val @ Some(_))) => {
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut env = (*env).clone();
                    (cache, env) = extendEnvWithElement(e.clone(), val.clone(), inCache.clone(), env.clone())?;
                    Ok((cache.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, env, (e @ Deref @ DAE::Element::VAR { binding: binding_exp, .. }, None)) => {
                    let mut val: Option<Arc<Values::Value>> = None;
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut env = (*env).clone();
                    (val, cache) = evaluateBinding(binding_exp.clone(), inCache.clone(), inEnv.clone())?;
                    (cache, env) = extendEnvWithElement(e.clone(), val.clone(), cache.clone(), env.clone())?;
                    Ok((cache.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, (e, _)) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln((literal!("- CevalFunction.extendEnvWithFunctionVars failed for:")).clone())?;
                    Debug::traceln((DAEDump::dumpElementsStr(list![e.clone()])?).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv))
}

fn evaluateBinding(mut inBinding: Option<Arc<DAE::Exp>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(Option<Arc<Values::Value>>, FCore::Cache)> {
    let mut outValue: Option<Arc<Values::Value>> = None;
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    (outValue, outCache) = (::match_deref::match_deref! { match &((inBinding.clone(), inCache.clone(), inEnv.clone())) {
        (Some(binding_exp), _, _) => {
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            (cache, val) = cevalExp(binding_exp.clone(), inCache.clone(), inEnv.clone())?;
            (Some(val.clone()), cache.clone())
        },
        (None, _, _) => {
            (None, inCache.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outValue, outCache))
}

fn extendEnvWithElement(mut inElement: Arc<DAE::Element>, mut inBindingValue: Option<Arc<Values::Value>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    (outCache, outEnv) = (::match_deref::match_deref! { match &((inElement.clone(), inBindingValue.clone(), inCache.clone(), inEnv.clone())) {
        (Deref @ DAE::Element::VAR { dims, ty, componentRef: cr, .. }, _, _, _) => {
            let mut name: ArcStr = arcstr::literal!("");
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            let mut env: FCore::Graph;
            name = (ComponentReference::crefStr(cr.clone())?).clone();
            (cache, env) = extendEnvWithVar((name.clone()).clone(), ty.clone(), inBindingValue.clone(), dims.clone(), inCache.clone(), inEnv.clone())?;
            (cache.clone(), env.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outEnv))
}

fn extendEnvWithVar(mut inName: ArcStr, mut inType: Arc<DAE::Type>, mut inOptValue: Option<Arc<Values::Value>>, mut inDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    (outCache, outEnv) = 'mc: {
        let __mc_input = (inName.clone(), inType.clone(), inOptValue.clone(), inDims.clone(), inCache.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _) => {
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut var: Arc<DAE::Var>;
                    let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut env: FCore::Graph;
                    let mut record_env: FCore::Graph;
                    let true = (Types::isRecord(inType.clone())) else { bail!("pattern mismatch") };
                    binding = makeBinding(inOptValue.clone());
                    (cache, ty) = appendDimensions(inType.clone(), inOptValue.clone(), inDims.clone(), inCache.clone(), inEnv.clone())?;
                    var = makeFunctionVariable((inName.clone()).clone(), ty.clone(), binding.clone());
                    (cache, record_env) = makeRecordEnvironment(inType.clone(), inOptValue.clone(), cache.clone(), inEnv.clone())?;
                    env = FGraph::mkComponentNode(inEnv.clone(), var.clone(), Arc::new(SCode::Element::COMPONENT { name: (inName.clone()).clone(), prefixes: SCode::defaultPrefixes.clone(), attributes: SCode::Attributes { arrayDims: metamodelica::nil(), connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL, parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: openmodelica_frontend_types::SCode::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD }, typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }), arrayDim: None }), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), crate::FCore::Status::VAR_TYPED, record_env.clone())?;
                    Ok((cache.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut var: Arc<DAE::Var>;
                    let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut env: FCore::Graph;
                    binding = makeBinding(inOptValue.clone());
                    (cache, ty) = appendDimensions(inType.clone(), inOptValue.clone(), inDims.clone(), inCache.clone(), inEnv.clone())?;
                    var = makeFunctionVariable((inName.clone()).clone(), ty.clone(), binding.clone());
                    env = FGraph::mkComponentNode(inEnv.clone(), var.clone(), Arc::new(SCode::Element::COMPONENT { name: (inName.clone()).clone(), prefixes: SCode::defaultPrefixes.clone(), attributes: SCode::Attributes { arrayDims: metamodelica::nil(), connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL, parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: openmodelica_frontend_types::SCode::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD }, typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }), arrayDim: None }), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), crate::FCore::Status::VAR_TYPED, FGraph::empty())?;
                    Ok((cache.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv))
}

fn makeFunctionVariable(mut inName: ArcStr, mut inType: Arc<DAE::Type>, mut inBinding: Arc<DAE::Binding>) -> Arc<DAE::Var> {
    let mut outVar: Arc<DAE::Var>;
    outVar = Arc::new(DAE::Var { name: (inName.clone()).clone(), attributes: DAE::dummyAttrVar.clone(), ty: inType.clone(), binding: inBinding.clone(), bind_from_outside: false, constOfForIteratorRange: None });
    outVar
}

fn makeBinding(mut inBindingValue: Option<Arc<Values::Value>>) -> Arc<DAE::Binding> {
    let mut outBinding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
    outBinding = (::match_deref::match_deref! { match &(inBindingValue.clone()) {
        Some(val) => {
            Arc::new(DAE::Binding::VALBOUND { valBound: val.clone(), source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE })
        },
        None => {
            Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBinding
}

fn makeRecordEnvironment(mut inRecordType: Arc<DAE::Type>, mut inOptValue: Option<Arc<Values::Value>>, mut inCache: FCore::Cache, mut inGraph: FCore::Graph) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outRecordEnv: FCore::Graph;
    (outCache, outRecordEnv) = (::match_deref::match_deref! { match &((inRecordType.clone(), inOptValue.clone(), inCache.clone(), inGraph.clone())) {
        (Deref @ DAE::Type::T_COMPLEX { varLst: var_lst, complexClassType: ClassInf::State::RECORD { .. }, .. }, _, _, _) => {
            let mut vals: Arc<metamodelica::List<Option<Arc<Values::Value>>>> = metamodelica::nil();
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            let mut graph: FCore::Graph;
            let mut parent: metamodelica::Array<FCore::Node>;
            let mut child: metamodelica::Array<FCore::Node>;
            let mut node: FCore::Node;
            parent = FGraph::lastScopeRef(inGraph.clone())?;
            (graph, node) = FGraph::node(inGraph.clone(), (arcstr::literal!(FNode::feNodeName)).clone(), list![parent.clone()], FCore::Data::ND { scopeType: None });
            child = FNode::toRef(node.clone());
            FNode::addChildRef(parent.clone(), (arcstr::literal!(FNode::feNodeName)).clone(), child.clone(), false)?;
            graph = FGraph::pushScopeRef(graph.clone(), child.clone())?;
            vals = getRecordValues(inOptValue.clone(), inRecordType.clone())?;
            (cache, graph) = List::threadFold(var_lst.clone(), vals.clone(), Arc::new(extendEnvWithRecordVar), (inCache.clone(), graph.clone()))?;
            (cache.clone(), graph.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outRecordEnv))
}

fn getRecordValues(mut inOptValue: Option<Arc<Values::Value>>, mut inRecordType: Arc<DAE::Type>) -> Result<Arc<metamodelica::List<Option<Arc<Values::Value>>>>> {
    let mut outValues: Arc<metamodelica::List<Option<Arc<Values::Value>>>> = metamodelica::nil();
    outValues = (::match_deref::match_deref! { match &((inOptValue.clone(), inRecordType.clone())) {
        (Some(Deref @ Values::Value::RECORD { orderd: vals, .. }), _) => {
            let mut opt_vals: Arc<metamodelica::List<Option<Arc<Values::Value>>>> = metamodelica::nil();
            opt_vals = List::map(vals.clone(), Arc::new(fnptr!(Util::makeOption, _)));
            opt_vals.clone()
        },
        (None, Deref @ DAE::Type::T_COMPLEX { varLst: vars, .. }) => {
            let mut opt_vals: Arc<metamodelica::List<Option<Arc<Values::Value>>>> = metamodelica::nil();
            let mut n: i32 = 0;
            n = (vars.clone().len() as i32);
            opt_vals = List::fill(None, n.clone());
            opt_vals.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outValues)
}

fn extendEnvWithRecordVar(mut inVar: Arc<DAE::Var>, mut inOptValue: Option<Arc<Values::Value>>, mut inEnv: (FCore::Cache, FCore::Graph)) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outEnv: (FCore::Cache, FCore::Graph);
    outEnv = (::match_deref::match_deref! { match &((inVar.clone(), inOptValue.clone(), inEnv.clone())) {
        (Deref @ DAE::Var { ty, name, .. }, _, (cache, env)) => {
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (cache, env) = extendEnvWithVar((name.clone()).clone(), ty.clone(), inOptValue.clone(), metamodelica::nil(), cache.clone(), env.clone())?;
            outEnv = (cache.clone(), env.clone());
            outEnv.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outEnv)
}

fn extendEnvWithForScope(mut inIterName: ArcStr, mut inIterType: Arc<DAE::Type>, mut inEnv: FCore::Graph) -> Result<(FCore::Graph, Arc<DAE::Type>, Arc<DAE::ComponentRef>)> {
    let mut outEnv: FCore::Graph;
    let mut outIterType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut outIterCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut iter_cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    outIterType = Types::expTypetoTypesType(inIterType.clone())?;
    outEnv = FGraph::addForIterator(inEnv.clone(), (inIterName.clone()).clone(), outIterType.clone(), Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), openmodelica_frontend_types::SCode::Variability::CONST, Some(openmodelica_frontend_types::DAE::Const::C_CONST))?;
    outIterCref = ComponentReferenceBasics::makeCrefIdent((inIterName.clone()).clone(), inIterType.clone(), metamodelica::nil());
    Ok((outEnv, outIterType, outIterCref))
}

fn appendDimensions(mut inType: Arc<DAE::Type>, mut inOptBinding: Option<Arc<Values::Value>>, mut inDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, Arc<DAE::Type>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut binding_dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
    binding_dims = ValuesUtil::valueDimensions(Util::getOptionOrDefault(inOptBinding.clone(), Arc::new(Values::Value::INTEGER { integer: 0 })));
    (outCache, outType) = appendDimensions2(inType.clone(), inDims.clone(), binding_dims.clone(), inCache.clone(), inEnv.clone())?;
    Ok((outCache, outType))
}

fn appendDimensions2(mut inType: Arc<DAE::Type>, mut inDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut inBindingDims: Arc<metamodelica::List<i32>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, Arc<DAE::Type>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    (outCache, outType) = 'mc: {
        let __mc_input = (inType.clone(), inDims.clone(), inBindingDims.clone(), inCache.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty, Deref @ metamodelica::List::Nil, _, _, _) => {
                    Ok((inCache.clone(), ty.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN, tail: rest_dims }, Deref @ metamodelica::List::Cons { head: dim_int, tail: bind_dims }, _, _) => {
                    let mut dim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut ty = (*ty).clone();
                    dim = Expression::intDimension(dim_int.clone());
                    (cache, ty) = appendDimensions2(ty.clone(), rest_dims.clone(), bind_dims.clone(), inCache.clone(), inEnv.clone())?;
                    Ok((cache.clone(), Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![dim.clone()] })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN, tail: rest_dims }, bind_dims, _, _) => {
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut ty = (*ty).clone();
                    (cache, ty) = appendDimensions2(ty.clone(), rest_dims.clone(), bind_dims.clone(), inCache.clone(), inEnv.clone())?;
                    Ok((cache.clone(), Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: 0 })] })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_INTEGER { integer: dim_int }, tail: rest_dims }, bind_dims, _, _) => {
                    let mut dim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut ty = (*ty).clone();
                    let mut bind_dims = (*bind_dims).clone();
                    dim = Arc::new(DAE::Dimension::DIM_INTEGER { integer: dim_int.clone() });
                    bind_dims = List::restOrEmpty(bind_dims.clone())?;
                    (cache, ty) = appendDimensions2(ty.clone(), rest_dims.clone(), bind_dims.clone(), inCache.clone(), inEnv.clone())?;
                    Ok((cache.clone(), Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![dim.clone()] })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_BOOLEAN, tail: rest_dims }, bind_dims, _, _) => {
                    let mut dim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut ty = (*ty).clone();
                    let mut bind_dims = (*bind_dims).clone();
                    dim = Arc::new(DAE::Dimension::DIM_INTEGER { integer: 2 });
                    bind_dims = List::restOrEmpty(bind_dims.clone())?;
                    (cache, ty) = appendDimensions2(ty.clone(), rest_dims.clone(), bind_dims.clone(), inCache.clone(), inEnv.clone())?;
                    Ok((cache.clone(), Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![dim.clone()] })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_ENUM { size: dim_int, .. }, tail: rest_dims }, bind_dims, _, _) => {
                    let mut dim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut ty = (*ty).clone();
                    let mut bind_dims = (*bind_dims).clone();
                    dim = Arc::new(DAE::Dimension::DIM_INTEGER { integer: dim_int.clone() });
                    bind_dims = List::restOrEmpty(bind_dims.clone())?;
                    (cache, ty) = appendDimensions2(ty.clone(), rest_dims.clone(), bind_dims.clone(), inCache.clone(), inEnv.clone())?;
                    Ok((cache.clone(), Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![dim.clone()] })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_EXP { exp: dim_exp }, tail: rest_dims }, bind_dims, _, _) => {
                    let mut dim_val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut dim_int: i32 = 0;
                    let mut dim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut ty = (*ty).clone();
                    let mut bind_dims = (*bind_dims).clone();
                    (cache, dim_val) = cevalExp(dim_exp.clone(), inCache.clone(), inEnv.clone())?;
                    dim_int = ValuesUtil::valueInteger(dim_val.clone())?;
                    dim = Arc::new(DAE::Dimension::DIM_INTEGER { integer: dim_int.clone() });
                    bind_dims = List::restOrEmpty(bind_dims.clone())?;
                    (cache, ty) = appendDimensions2(ty.clone(), rest_dims.clone(), bind_dims.clone(), inCache.clone(), inEnv.clone())?;
                    Ok((cache.clone(), Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![dim.clone()] })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: _, tail: _ }, _, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- CevalFunction.appendDimensions2 failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outType))
}

// [MENV]  Environment manipulation functions (set and get variables).
fn assignVariable(mut inCref: Arc<DAE::ComponentRef>, mut inNewValue: Arc<Values::Value>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    (outCache, outEnv) = 'mc: {
        let __mc_input = (inCref.clone(), inNewValue.clone(), inCache.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::WILD, _, _, _) => {
                    Ok((inCache.clone(), inEnv.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { identType: ety @ Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { .. }, .. }, subscriptLst: Deref @ metamodelica::List::Nil, ident: id }, _, _, _) => {
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut env: FCore::Graph;
                    let mut var: Arc<DAE::Var>;
                    let mut inst_status: FCore::Status = FCore::Status::CLS_FULL;
                    (_, var, _, _, inst_status, env) = Lookup::lookupIdentLocal(inCache.clone(), inEnv.clone(), (id.clone()).clone())?;
                    (cache, env) = assignRecord(ety.clone(), inNewValue.clone(), inCache.clone(), env.clone())?;
                    var = updateRecordBinding(var.clone(), inNewValue.clone());
                    env = FGraph::updateComp(inEnv.clone(), var.clone(), inst_status.clone(), env.clone())?;
                    Ok((cache.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cr @ Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::List::Nil, .. }, _, _, _) => {
                    let mut env: FCore::Graph;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    ty = Types::unflattenArrayType(Expression::r#typeof(ValuesUtil::valueExp(inNewValue.clone(), None)?)?)?;
                    env = updateVariableBinding(cr.clone(), inEnv.clone(), ty.clone(), inNewValue.clone())?;
                    Ok((inCache.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: subs, .. }, _, _, _) => {
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut env: FCore::Graph;
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    cr = ComponentReference::crefStripSubs(inCref.clone())?;
                    (ty, val) = getVariableTypeAndValue(cr.clone(), inEnv.clone())?;
                    (cache, val) = assignVector(inNewValue.clone(), val.clone(), subs.clone(), inCache.clone(), inEnv.clone())?;
                    env = updateVariableBinding(cr.clone(), inEnv.clone(), ty.clone(), val.clone())?;
                    Ok((cache.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cr_rest, subscriptLst: Deref @ metamodelica::List::Nil, ident: id, .. }, _, _, _) => {
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut env: FCore::Graph;
                    let mut var: Arc<DAE::Var>;
                    let mut inst_status: FCore::Status = FCore::Status::CLS_FULL;
                    let mut comp_id: ArcStr = arcstr::literal!("");
                    (_, var, _, _, inst_status, env) = Lookup::lookupIdentLocal(inCache.clone(), inEnv.clone(), (id.clone()).clone())?;
                    (cache, env) = assignVariable(cr_rest.clone(), inNewValue.clone(), inCache.clone(), env.clone())?;
                    comp_id = (ComponentReferenceBasics::crefFirstIdent(cr_rest.clone())?).clone();
                    var = updateRecordComponentBinding(var.clone(), (comp_id.clone()).clone(), inNewValue.clone())?;
                    env = FGraph::updateComp(inEnv.clone(), var.clone(), inst_status.clone(), env.clone())?;
                    Ok((cache.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv))
}

fn assignTuple(mut inLhsCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inRhsValues: Arc<metamodelica::List<Arc<Values::Value>>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    (outCache, outEnv) = (::match_deref::match_deref! { match &((inLhsCrefs.clone(), inRhsValues.clone(), inCache.clone(), inEnv.clone())) {
        (Deref @ metamodelica::List::Nil, _, cache, env) => {
            (cache.clone(), env.clone())
        },
        (Deref @ metamodelica::List::Cons { head: cr, tail: rest_crefs }, Deref @ metamodelica::List::Cons { head: value, tail: rest_vals }, cache, env) => {
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (cache, env) = assignVariable(cr.clone(), value.clone(), cache.clone(), env.clone())?;
            (cache, env) = assignTuple(rest_crefs.clone(), rest_vals.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outEnv))
}

fn assignRecord(mut inType: Arc<DAE::Type>, mut inValue: Arc<Values::Value>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    (outCache, outEnv) = (::match_deref::match_deref! { match &((inType.clone(), inValue.clone(), inCache.clone(), inEnv.clone())) {
        (Deref @ DAE::Type::T_COMPLEX { varLst: vars, .. }, Deref @ Values::Value::RECORD { orderd: values, .. }, _, _) => {
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            let mut env: FCore::Graph;
            (cache, env) = assignRecordComponents(vars.clone(), values.clone(), inCache.clone(), inEnv.clone())?;
            (cache.clone(), env.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outEnv))
}

fn assignRecordComponents(mut inVars: Arc<metamodelica::List<Arc<DAE::Var>>>, mut inValues: Arc<metamodelica::List<Arc<Values::Value>>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    (outCache, outEnv) = (::match_deref::match_deref! { match &((inVars.clone(), inValues.clone(), inCache.clone(), inEnv.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _, _) => {
            (inCache.clone(), inEnv.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { ty, name, .. }, tail: rest_vars }, Deref @ metamodelica::List::Cons { head: val, tail: rest_vals }, _, _) => {
            let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            let mut env: FCore::Graph;
            cr = ComponentReferenceBasics::makeCrefIdent((name.clone()).clone(), ty.clone(), metamodelica::nil());
            (cache, env) = assignVariable(cr.clone(), val.clone(), inCache.clone(), inEnv.clone())?;
            (cache, env) = assignRecordComponents(rest_vars.clone(), rest_vals.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outEnv))
}

pub fn assignVector(mut inNewValue: Arc<Values::Value>, mut inOldValue: Arc<Values::Value>, mut inSubscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outResult: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outResult) = 'mc: {
        let __mc_input = (inNewValue.clone(), inOldValue.clone(), inSubscripts.clone(), inCache.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Nil, _, _) => {
                    Ok((inCache.clone(), inNewValue.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ Values::Value::ARRAY { dimLst: dims, valueLst: values }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: e }, tail: rest_subs }, _, _) => {
                    let mut index: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut i: i32 = 0;
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut values = (*values).clone();
                    (cache, index) = cevalExp(e.clone(), inCache.clone(), inEnv.clone())?;
                    i = ValuesUtil::valueInteger(index.clone())?;
                    val = (values.clone()).get(i.clone())?;
                    (cache, val) = assignVector(inNewValue.clone(), val.clone(), rest_subs.clone(), cache.clone(), inEnv.clone())?;
                    values = List::replaceAt(val.clone(), i.clone(), values.clone())?;
                    Ok((cache.clone(), Arc::new(Values::Value::ARRAY { valueLst: values.clone(), dimLst: dims.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::ARRAY { valueLst: values, .. }, Deref @ Values::Value::ARRAY { dimLst: dims, valueLst: old_values }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::SLICE { exp: e }, tail: rest_subs }, _, _) => {
                    let mut values2: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut old_values2: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut indices: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut i: i32 = 0;
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut values = (*values).clone();
                    let mut old_values = (*old_values).clone();
                    let (__pa0, __pa2, __pa1) = ::match_deref::match_deref! { match &(cevalExp(e.clone(), inCache.clone(), inEnv.clone())?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa2 @ Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: _ }, .. }) => (__pa0.clone(), __pa2.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    i = __pa1.clone();
                    indices = __pa2.clone();
                    (old_values, old_values2) = List::splitr(old_values.clone(), i.clone() - 1)?;
                    (cache, values2) = assignSlice(values.clone(), old_values2.clone(), indices.clone(), rest_subs.clone(), i.clone(), cache.clone(), inEnv.clone())?;
                    values = List::append_reverse(old_values.clone(), values2.clone());
                    Ok((cache.clone(), Arc::new(Values::Value::ARRAY { valueLst: values.clone(), dimLst: dims.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::ARRAY { valueLst: values, .. }, Deref @ Values::Value::ARRAY { dimLst: dims, valueLst: values2 }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::WHOLEDIM, tail: rest_subs }, _, _) => {
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut values = (*values).clone();
                    (cache, values) = assignWholeDim(values.clone(), values2.clone(), rest_subs.clone(), inCache.clone(), inEnv.clone())?;
                    Ok((cache.clone(), Arc::new(Values::Value::ARRAY { valueLst: values.clone(), dimLst: dims.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Cons { head: sub, tail: _ }, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    println!("{}", (literal!("- CevalFunction.assignVector failed on: ")).clone());
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ExpressionBasics::printSubscriptStr(sub.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outResult))
}

fn assignSlice(mut inNewValues: Arc<metamodelica::List<Arc<Values::Value>>>, mut inOldValues: Arc<metamodelica::List<Arc<Values::Value>>>, mut inIndices: Arc<metamodelica::List<Arc<Values::Value>>>, mut inSubscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut inIndex: i32, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<Values::Value>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outResult: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    (outCache, outResult) = 'mc: {
        let __mc_input = (inNewValues.clone(), inOldValues.clone(), inIndices.clone(), inSubscripts.clone(), inIndex.clone(), inCache.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Nil, _, _, _, _) => {
                    Ok((inCache.clone(), inOldValues.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vl1, Deref @ metamodelica::List::Cons { head: v2, tail: vl2 }, Deref @ metamodelica::List::Cons { head: index, tail: _ }, _, _, _, _) => {
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut vl1 = (*vl1).clone();
                    let true = (inIndex.clone() < ValuesUtil::valueInteger(index.clone())?) else { bail!("pattern mismatch") };
                    (cache, vl1) = assignSlice(vl1.clone(), vl2.clone(), inIndices.clone(), inSubscripts.clone(), inIndex.clone() + 1, inCache.clone(), inEnv.clone())?;
                    Ok((cache.clone(), cons(v2.clone(), vl1.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: v1, tail: vl1 }, Deref @ metamodelica::List::Cons { head: v2, tail: vl2 }, Deref @ metamodelica::List::Cons { head: _, tail: rest_indices }, _, _, _, _) => {
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut v1 = (*v1).clone();
                    let mut vl1 = (*vl1).clone();
                    (cache, v1) = assignVector(v1.clone(), v2.clone(), inSubscripts.clone(), inCache.clone(), inEnv.clone())?;
                    (cache, vl1) = assignSlice(vl1.clone(), vl2.clone(), rest_indices.clone(), inSubscripts.clone(), inIndex.clone() + 1, inCache.clone(), inEnv.clone())?;
                    Ok((cache.clone(), cons(v1.clone(), vl1.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outResult))
}

fn assignWholeDim(mut inNewValues: Arc<metamodelica::List<Arc<Values::Value>>>, mut inOldValues: Arc<metamodelica::List<Arc<Values::Value>>>, mut inSubscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<Values::Value>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outResult: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    (outCache, outResult) = (::match_deref::match_deref! { match &((inNewValues.clone(), inOldValues.clone(), inSubscripts.clone(), inCache.clone(), inEnv.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _) => {
            (inCache.clone(), metamodelica::nil())
        },
        (Deref @ metamodelica::List::Cons { head: v1, tail: vl1 }, Deref @ metamodelica::List::Cons { head: v2, tail: vl2 }, _, _, _) => {
            let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
            let mut v1 = (*v1).clone();
            let mut vl1 = (*vl1).clone();
            (cache, v1) = assignVector(v1.clone(), v2.clone(), inSubscripts.clone(), inCache.clone(), inEnv.clone())?;
            (cache, vl1) = assignWholeDim(vl1.clone(), vl2.clone(), inSubscripts.clone(), inCache.clone(), inEnv.clone())?;
            (cache.clone(), cons(v1.clone(), vl1.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outResult))
}

fn updateVariableBinding(mut inVariableCref: Arc<DAE::ComponentRef>, mut inEnv: FCore::Graph, mut inType: Arc<DAE::Type>, mut inNewValue: Arc<Values::Value>) -> Result<FCore::Graph> {
    let mut outEnv: FCore::Graph;
    let mut var_name: ArcStr = arcstr::literal!("");
    let mut var: Arc<DAE::Var>;
    var_name = (ComponentReference::crefStr(inVariableCref.clone())?).clone();
    var = makeFunctionVariable((var_name.clone()).clone(), inType.clone(), Arc::new(DAE::Binding::VALBOUND { valBound: inNewValue.clone(), source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE }));
    outEnv = FGraph::updateComp(inEnv.clone(), var.clone(), crate::FCore::Status::VAR_TYPED, FGraph::empty())?;
    Ok(outEnv)
}

fn updateRecordBinding(mut inVar: Arc<DAE::Var>, mut inValue: Arc<Values::Value>) -> Arc<DAE::Var> {
    let mut outVar: Arc<DAE::Var>;
    let mut name: ArcStr = arcstr::literal!("");
    let mut attr: Arc<DAE::Attributes>;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut c: Option<DAE::Const> = None;
    outVar = inVar.clone();
    assign_field!(outVar.binding = Arc::new(DAE::Binding::VALBOUND { valBound: inValue.clone(), source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE }));
    outVar
}

fn updateRecordComponentBinding(mut inVar: Arc<DAE::Var>, mut inComponentId: ArcStr, mut inValue: Arc<Values::Value>) -> Result<Arc<DAE::Var>> {
    let mut outVar: Arc<DAE::Var>;
    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outVar = inVar.clone();
    val = getBindingOrDefault(outVar.binding.clone(), outVar.ty.clone())?;
    val = updateRecordComponentValue((inComponentId.clone()).clone(), inValue.clone(), val.clone())?;
    assign_field!(outVar.binding = Arc::new(DAE::Binding::VALBOUND { valBound: val.clone(), source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE }));
    Ok(outVar)
}

fn updateRecordComponentValue(mut inComponentId: ArcStr, mut inComponentValue: Arc<Values::Value>, mut inRecordValue: Arc<Values::Value>) -> Result<Arc<Values::Value>> {
    let mut outRecordValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut name: Arc<Absyn::Path>;
    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    let mut comps: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut pos: i32 = 0;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inRecordValue.clone()) {
        Deref @ Values::Value::RECORD { record_: __pa0, orderd: __pa1, comp: __pa2, index: (-1) } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa0.clone();
    vals = __pa1.clone();
    comps = __pa2.clone();
    pos = List::position((inComponentId.clone()).clone(), comps.clone())?;
    vals = List::replaceAt(inComponentValue.clone(), pos.clone(), vals.clone())?;
    outRecordValue = Arc::new(Values::Value::RECORD { record_: name.clone(), orderd: vals.clone(), comp: comps.clone(), index: -1 });
    Ok(outRecordValue)
}

fn getVariableTypeAndBinding(mut inCref: Arc<DAE::ComponentRef>, mut inEnv: FCore::Graph) -> Result<(Arc<DAE::Type>, Arc<DAE::Binding>)> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut outBinding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
    (_, _, outType, outBinding, _, _, _, _, _) = Lookup::lookupVar(FCore::emptyCache(), inEnv.clone(), inCref.clone())?;
    Ok((outType, outBinding))
}

fn getVariableTypeAndValue(mut inCref: Arc<DAE::ComponentRef>, mut inEnv: FCore::Graph) -> Result<(Arc<DAE::Type>, Arc<Values::Value>)> {
    let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
    (outType, binding) = getVariableTypeAndBinding(inCref.clone(), inEnv.clone())?;
    outValue = getBindingOrDefault(binding.clone(), outType.clone())?;
    Ok((outType, outValue))
}

fn getBindingValueOpt(mut inBinding: Arc<DAE::Binding>) -> Option<Arc<Values::Value>> {
    let mut outValue: Option<Arc<Values::Value>> = None;
    outValue = (::match_deref::match_deref! { match &(inBinding.clone()) {
        Deref @ DAE::Binding::VALBOUND { valBound: val, .. } => {
            Some(val.clone())
        },
        Deref @ DAE::Binding::EQBOUND { evaluatedExp: Some(val), .. } => {
            Some(val.clone())
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outValue
}

fn getBindingOrDefault(mut inBinding: Arc<DAE::Binding>, mut inType: Arc<DAE::Type>) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outValue = (::match_deref::match_deref! { match &((inBinding.clone(), inType.clone())) {
        (Deref @ DAE::Binding::VALBOUND { valBound: val, .. }, _) => {
            val.clone()
        },
        (Deref @ DAE::Binding::EQBOUND { evaluatedExp: Some(val), .. }, _) => {
            val.clone()
        },
        _ => {
            generateDefaultBinding(inType.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outValue)
}

fn generateDefaultBinding(mut inType: Arc<DAE::Type>) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outValue = 'mc: {
        let __mc_input = inType.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_INTEGER { .. } => {
                    Ok(Arc::new(Values::Value::INTEGER { integer: 0 }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_REAL { .. } => {
                    Ok(Arc::new(Values::Value::REAL { real: metamodelica::OrderedFloat(0.0_f64) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_STRING { .. } => {
                    Ok(Arc::new(Values::Value::STRING { string: (literal!("")).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_BOOL { .. } => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ENUMERATION { .. } => {
                    Ok(Arc::new(Values::Value::ENUM_LITERAL { name: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }), index: 0 }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { ty, dims: Deref @ metamodelica::List::Cons { head: dim, tail: Deref @ metamodelica::List::Nil } } => {
                    let mut int_dim: i32 = 0;
                    let mut dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut values: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut value: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    int_dim = Expression::dimensionSize(dim.clone())?;
                    value = generateDefaultBinding(ty.clone())?;
                    values = List::fill(value.clone(), int_dim.clone());
                    dims = ValuesUtil::valueDimensions(value.clone());
                    Ok(Arc::new(Values::Value::ARRAY { valueLst: values.clone(), dimLst: cons(int_dim.clone(), dims.clone()) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_COMPLEX { varLst: vars, complexClassType: ClassInf::State::RECORD { path }, .. } => {
                    let mut values: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut var_names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    (values, var_names) = List::map_2(vars.clone(), Arc::new(getRecordVarBindingAndName));
                    Ok(Arc::new(Values::Value::RECORD { record_: path.clone(), orderd: values.clone(), comp: var_names.clone(), index: -1 }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- CevalFunction.generateDefaultBinding failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outValue)
}

fn getRecordVarBindingAndName(mut inVar: Arc<DAE::Var>) -> Result<(Arc<Values::Value>, ArcStr)> {
    let mut outBinding: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut outName: ArcStr = arcstr::literal!("");
    (outBinding, outName) = 'mc: {
        let __mc_input = inVar.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Var { binding, ty, name, .. } => {
                    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    val = getBindingOrDefault(binding.clone(), ty.clone())?;
                    Ok((val.clone(), name.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Var { name, .. } => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- CevalFunction.getRecordVarBindingAndName failed on variable ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outBinding, outName))
}

fn getFunctionReturnValue(mut inOutputVar: Arc<DAE::Element>, mut inEnv: FCore::Graph) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outValue = (::match_deref::match_deref! { match &((inOutputVar.clone(), inEnv.clone())) {
        (Deref @ DAE::Element::VAR { ty, componentRef: cr, .. }, _) => {
            let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            val = getVariableValue(cr.clone(), ty.clone(), inEnv.clone())?;
            val.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outValue)
}

fn getVariableValue(mut inCref: Arc<DAE::ComponentRef>, mut inType: Arc<DAE::Type>, mut inEnv: FCore::Graph) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outValue = 'mc: {
        let __mc_input = (inCref.clone(), inType.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { .. }, .. }, _) => {
                    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut p: Arc<Absyn::Path>;
                    p = ComponentReference::crefToPath(inCref.clone())?;
                    val = getRecordValue(p.clone(), inType.clone(), inEnv.clone())?;
                    Ok(val.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    (_, val) = getVariableTypeAndValue(inCref.clone(), inEnv.clone())?;
                    Ok(val.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outValue)
}

fn getRecordValue(mut inRecordName: Arc<Absyn::Path>, mut inType: Arc<DAE::Type>, mut inEnv: FCore::Graph) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outValue = (::match_deref::match_deref! { match &((inRecordName.clone(), inType.clone(), inEnv.clone())) {
        (Deref @ Absyn::Path::IDENT { name: id }, Deref @ DAE::Type::T_COMPLEX { varLst: vars, complexClassType: ClassInf::State::RECORD { path: p }, .. }, _) => {
            let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut var_names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut env: FCore::Graph;
            (_, _, _, _, _, env) = Lookup::lookupIdentLocal(FCore::emptyCache(), inEnv.clone(), (id.clone()).clone())?;
            vals = List::map1(vars.clone(), Arc::new(getRecordComponentValue), env.clone());
            var_names = List::map(vars.clone(), Arc::new(TypesDump::getVarName));
            Arc::new(Values::Value::RECORD { record_: p.clone(), orderd: vals.clone(), comp: var_names.clone(), index: -1 })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outValue)
}

fn getRecordComponentValue(mut inVars: Arc<DAE::Var>, mut inEnv: FCore::Graph) -> Result<Arc<Values::Value>> {
    let mut outValues: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outValues = (::match_deref::match_deref! { match &((inVars.clone(), inEnv.clone())) {
        (Deref @ DAE::Var { ty: ty @ Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { .. }, .. }, name: id, .. }, _) => {
            let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            val = getRecordValue(Arc::new(Absyn::Path::IDENT { name: (id.clone()).clone() }), ty.clone(), inEnv.clone())?;
            val.clone()
        },
        (Deref @ DAE::Var { binding: tvbinding, ty, name: id, .. }, _) => {
            let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut oval: Option<Arc<Values::Value>> = None;
            let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
            let __pa0 = ::match_deref::match_deref! { match &(Lookup::lookupIdentLocal(FCore::emptyCache(), inEnv.clone(), (id.clone()).clone())?) {
                (_, Deref @ DAE::Var { binding: __pa0, .. }, _, _, _, _) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            binding = __pa0.clone();
            oval = getBindingValueOpt(binding.clone());
            if isNone(oval.clone()) {
                oval = getBindingValueOpt(tvbinding.clone());
            }
            if isSome(oval.clone()) {
                let __pa1 = ::match_deref::match_deref! { match &(oval.clone()) {
                    Some(__pa1) => __pa1.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                val = __pa1.clone();
            } else {
                val = generateDefaultBinding(ty.clone())?;
            }
            val.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outValues)
}

fn boxReturnValue(mut inReturnValues: Arc<metamodelica::List<Arc<Values::Value>>>) -> Arc<Values::Value> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outValue = (::match_deref::match_deref! { match &(inReturnValues.clone()) {
        Deref @ metamodelica::List::Nil => {
            Arc::new(openmodelica_frontend_types::Values::Value::NORETCALL)
        },
        Deref @ metamodelica::List::Cons { head: val, tail: Deref @ metamodelica::List::Nil } => {
            val.clone()
        },
        Deref @ metamodelica::List::Cons { head: _, tail: _ } => {
            Arc::new(Values::Value::TUPLE { valueLst: inReturnValues.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outValue
}

// [DEPS]  Function variable dependency handling.
fn sortFunctionVarsByDependency(mut inFuncVars: Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, mut inSource: Arc<DAE::ElementSource>) -> Result<Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>> {
    let mut outFuncVars: Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>> = metamodelica::nil();
    let mut cycles: Arc<metamodelica::List<((Arc<DAE::Element>, Option<Arc<Values::Value>>), Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>)>> = metamodelica::nil();
    (outFuncVars, cycles) = Graph::topologicalSort(Graph::buildGraph(inFuncVars.clone(), Arc::new(getElementDependencies), inFuncVars.clone()), Arc::new(isElementEqual))?;
    checkCyclicalComponents(cycles.clone(), inSource.clone())?;
    Ok(outFuncVars)
}

fn getElementDependencies(mut inElement: FunctionVar, mut inAllElements: Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>) -> Result<Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>> {
    let mut outDependencies: Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>> = metamodelica::nil();
    outDependencies = 'mc: {
        let __mc_input = (inElement.clone(), inAllElements.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ DAE::Element::VAR { dims, binding: Some(bind_exp), .. }, _), _) => {
                    let mut deps: Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>> = metamodelica::nil();
                    let mut arg: (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>);
                    let (_, ref __pa1 @ (_, ref __pa0, _)) = Expression::traverseExpBidir(bind_exp.clone(), Arc::new(getElementDependenciesTraverserEnter), Arc::new(getElementDependenciesTraverserExit), (inAllElements.clone(), metamodelica::nil(), metamodelica::nil()))?;
                    deps = __pa0.clone();
                    arg = __pa1.clone();
                    let (_, (_, __pa2, _)) = List::mapFold(dims.clone(), Arc::new(getElementDependenciesFromDims), arg.clone());
                    deps = __pa2.clone();
                    Ok(deps.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ DAE::Element::VAR { dims, .. }, _), _) => {
                    let mut deps: Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>> = metamodelica::nil();
                    let (_, (_, __pa0, _)) = List::mapFold(dims.clone(), Arc::new(getElementDependenciesFromDims), (inAllElements.clone(), metamodelica::nil(), metamodelica::nil()));
                    deps = __pa0.clone();
                    Ok(deps.clone())
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
    Ok(outDependencies)
}

fn getElementDependenciesFromDims(mut inDimension: Arc<DAE::Dimension>, mut inArg: (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>)) -> Result<(Arc<DAE::Dimension>, (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>))> {
    let mut outDimension: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    let mut outArg: (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>);
    (outDimension, outArg) = 'mc: {
        let __mc_input = (inDimension.clone(), inArg.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut arg: (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>);
                    let mut dim_exp: Arc<DAE::Exp>;
                    dim_exp = Expression::dimensionSizeExp(inDimension.clone())?;
                    (_, arg) = Expression::traverseExpBidir(dim_exp.clone(), Arc::new(getElementDependenciesTraverserEnter), Arc::new(getElementDependenciesTraverserExit), inArg.clone())?;
                    Ok((inDimension.clone(), arg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inDimension.clone(), inArg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outDimension, outArg))
}

fn getElementDependenciesTraverserEnter(mut inExp: Arc<DAE::Exp>, mut inArg: (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outArg: (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>);
    (outExp, outArg) = 'mc: {
        let __mc_input = (inExp.clone(), inArg.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: iter, .. }, .. }, (all_el, accum_el, iters @ Deref @ metamodelica::List::Cons { head: _, tail: _ })) => {
                    let true = (List::isMemberOnTrue((iter.clone()).clone(), iters.clone(), Arc::new(fnptr!(stringEqual, ArcStr, ArcStr)))) else { bail!("pattern mismatch") };
                    Ok((exp.clone(), (all_el.clone(), accum_el.clone(), iters.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ DAE::Exp::CREF { componentRef: cref, .. }, (all_el, accum_el, iters)) => {
                    let mut e: FunctionVar;
                    let mut all_el = (*all_el).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(List::deleteMemberOnTrue(cref.clone(), all_el.clone(), Arc::new(isElementNamed))?) {
                        (__pa0, Some(__pa1)) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    all_el = __pa0.clone();
                    e = __pa1.clone();
                    Ok((exp.clone(), (all_el.clone(), cons(e.clone(), accum_el.clone()), iters.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ DAE::Exp::REDUCTION { iterators: riters, .. }, (all_el, accum_el, iters)) => {
                    let mut iters = (*iters).clone();
                    iters = listAppend(List::map(riters.clone(), Arc::new(Expression::reductionIterName)), iters.clone());
                    Ok((exp.clone(), (all_el.clone(), accum_el.clone(), iters.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inArg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outArg))
}

fn getElementDependenciesTraverserExit(mut inExp: Arc<DAE::Exp>, mut inArg: (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outArg: (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>);
    (outExp, outArg) = (::match_deref::match_deref! { match &((inExp.clone(), inArg.clone())) {
        (exp @ Deref @ DAE::Exp::REDUCTION { iterators: riters, .. }, (all_el, accum_el, iters)) => {
            let mut iters = (*iters).clone();
            iters = compareIterators(riters.clone().reverse(), iters.clone())?;
            (exp.clone(), (all_el.clone(), accum_el.clone(), iters.clone()))
        },
        _ => {
            (inExp.clone(), inArg.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outArg))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn compareIterators(mut inRiters: Arc<metamodelica::List<Arc<DAE::ReductionIterator>>>, mut inIters: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outIters: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outIters = 'mc: {
        let __mc_input = (inRiters.clone(), inIters.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::ReductionIterator { id: id1, .. }, tail: riters }, Deref @ metamodelica::List::Cons { head: id2, tail: iters }) => {
                    let true = (stringEqual((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(compareIterators(riters.clone(), iters.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(inIters.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Different iterators in CevalFunction.compareIterators.")).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outIters)
}

fn isElementNamed(mut inName: Arc<DAE::ComponentRef>, mut inElement: FunctionVar) -> Result<bool> {
    let mut isNamed: bool = false;
    let mut name: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let __pa0 = ::match_deref::match_deref! { match &(inElement.clone()) {
        (Deref @ DAE::Element::VAR { componentRef: __pa0, .. }, _) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa0.clone();
    isNamed = ComponentReferenceBasics::crefEqualWithoutSubs(name.clone(), inName.clone());
    Ok(isNamed)
}

fn isElementEqual(mut inElement1: FunctionVar, mut inElement2: FunctionVar) -> Result<bool> {
    let mut isEqual: bool = false;
    let mut cr1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut cr2: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let __pa0 = ::match_deref::match_deref! { match &(inElement1.clone()) {
        (Deref @ DAE::Element::VAR { componentRef: __pa0, .. }, _) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    cr1 = __pa0.clone();
    let __pa1 = ::match_deref::match_deref! { match &(inElement2.clone()) {
        (Deref @ DAE::Element::VAR { componentRef: __pa1, .. }, _) => __pa1.clone(),
        _ => bail!("pattern mismatch"),
    } };
    cr2 = __pa1.clone();
    isEqual = ComponentReferenceBasics::crefEqualWithoutSubs(cr1.clone(), cr2.clone());
    Ok(isEqual)
}

fn checkCyclicalComponents(mut inCycles: Arc<metamodelica::List<((Arc<DAE::Element>, Option<Arc<Values::Value>>), Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>)>>, mut inSource: Arc<DAE::ElementSource>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inCycles.clone(), inSource.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            ()
        },
        _ => {
            let mut cycles: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>>> = metamodelica::nil();
            let mut elements: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>> = metamodelica::nil();
            let mut crefs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>> = metamodelica::nil();
            let mut names: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
            let mut cycles_strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut cycles_str: ArcStr = arcstr::literal!("");
            let mut scope_str: ArcStr = arcstr::literal!("");
            let mut info: SourceInfo;
            cycles = Graph::findCycles(inCycles.clone(), Arc::new(isElementEqual))?;
            elements = List::mapList(cycles.clone(), Arc::new(fnptr!(Util::tuple21, _)));
            crefs = List::mapList(elements.clone(), Arc::new(DAEUtil::varCref));
            names = List::mapList(crefs.clone(), Arc::new(ComponentReferenceBasics::printComponentRefStr));
            cycles_strs = List::map1(names.clone(), Arc::new(fnptr!(stringDelimitList, Arc<metamodelica::List<ArcStr>>, ArcStr)), (literal!(",")).clone());
            cycles_str = stringDelimitList(cycles_strs.clone(), (literal!("}, {")).clone());
            cycles_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*cycles_str.clone()); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
            scope_str = (literal!("")).clone();
            info = ElementSource::getElementSourceFileInfo(inSource.clone());
            Error::addSourceMessage(Error::CIRCULAR_COMPONENTS.clone(), list![(scope_str.clone()).clone(), (cycles_str.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

// [EOPT]  Expression optimization functions.
fn optimizeExpTraverser(mut inExp: Arc<DAE::Exp>, mut inEnv: FCore::Graph) -> Result<(Arc<DAE::Exp>, FCore::Graph)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outEnv: FCore::Graph;
    (outExp, outEnv) = (::match_deref::match_deref! { match &((inExp.clone(), inEnv.clone())) {
        (Deref @ DAE::Exp::ASUB { sub: subs, exp: Deref @ DAE::Exp::CREF { ty: ety, componentRef: cref } }, env) => {
            let mut exp: Arc<DAE::Exp>;
            let mut cref = (*cref).clone();
            cref = ComponentReference::subscriptCref(cref.clone(), subs.clone())?;
            exp = Expression::makeCrefExp(cref.clone(), ety.clone())?;
            (exp.clone(), env.clone())
        },
        (Deref @ DAE::Exp::TSUB { ix: 1, exp: Deref @ DAE::Exp::TUPLE { PR: Deref @ metamodelica::List::Cons { head: exp, tail: _ } }, .. }, env) => {
            (exp.clone(), env.clone())
        },
        _ => {
            (inExp.clone(), inEnv.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outEnv))
}

