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

use crate::NFBuiltinFuncs;
use crate::NFCall as Call;
use crate::NFCeval as Ceval;
use crate::NFCeval::EvalTarget;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFOperator::Op;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFTypeCheck as TypeCheck;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::Flags;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

pub fn simplifyDump(mut exp: Arc<Expression::NFExpression>, mut includeScope: bool, mut name: ArcStr, mut indent: ArcStr) -> Result<Arc<Expression::NFExpression>> {
    let mut res: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    res = simplify(exp.clone(), includeScope.clone())?;
    if Flags::isSet(Flags::DUMP_SIMPLIFY.clone())? && !(Expression::isEqual(exp.clone(), res.clone())?) {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("### dumpSimplify | ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(" ###\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("[BEFORE] ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("[AFTER ] ")); __mm_s.push_str(&*Expression::toString(res.clone())?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(res)
}

pub fn simplify(mut exp: Arc<Expression::NFExpression>, mut includeScope: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut old: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut new: Arc<Type::NFType> = Arc::new(Type::ANY);
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => {
            assign_variant_field!(exp => Expression::NFExpression::CREF;
                cref = ComponentRef::simplifySubscripts(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), false)?,
                ty = ComponentRef::getSubscriptedType(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), includeScope.clone())?
            );
            exp.clone()
        },
        Deref @ Expression::ARRAY { .. } if (!(var_field!((*exp).literal, Expression::NFExpression::ARRAY).clone())) => {
            assign_variant_field!(exp => Expression::NFExpression::ARRAY; elements = Array::map(var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone(), Arc::new({ let __pe_b1 = false; move |__pe_a0| simplify(__pe_a0, __pe_b1.clone()) })));
            exp.clone()
        },
        Deref @ Expression::RANGE { .. } => simplifyRange(exp.clone())?,
        Deref @ Expression::RECORD { .. } => {
            assign_variant_field!(exp => Expression::NFExpression::RECORD; elements = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, Expression::NFExpression::RECORD).clone()).into_iter().cloned() {
            let __x = simplify(e.clone(), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            exp.clone()
        },
        Deref @ Expression::CALL { .. } => simplifyCall(exp.clone())?,
        Deref @ Expression::SIZE { .. } => simplifySize(exp.clone())?,
        Deref @ Expression::MULTARY { .. } => simplifyMultary(exp.clone())?,
        Deref @ Expression::BINARY { .. } => simplifyBinary(exp.clone())?,
        Deref @ Expression::UNARY { .. } => simplifyUnary(exp.clone())?,
        Deref @ Expression::LBINARY { .. } => simplifyLogicBinary(exp.clone())?,
        Deref @ Expression::LUNARY { .. } => simplifyLogicUnary(exp.clone())?,
        Deref @ Expression::RELATION { .. } => simplifyRelation(exp.clone())?,
        Deref @ Expression::IF { .. } => simplifyIf(exp.clone())?,
        Deref @ Expression::CAST { .. } => simplifyCast(simplify(var_field!((*exp).exp, Expression::NFExpression::CAST).clone(), false)?, var_field!((*exp).ty, Expression::NFExpression::CAST).clone())?,
        Deref @ Expression::UNBOX { .. } => Arc::new(Expression::NFExpression::UNBOX { exp: simplify(var_field!((*exp).exp, Expression::NFExpression::UNBOX).clone(), false)?, ty: var_field!((*exp).ty, Expression::NFExpression::UNBOX).clone() }),
        Deref @ Expression::SUBSCRIPTED_EXP { .. } => simplifySubscriptedExp(exp.clone())?,
        Deref @ Expression::TUPLE_ELEMENT { .. } => simplifyTupleElement(exp.clone())?,
        Deref @ Expression::RECORD_ELEMENT { .. } => simplifyRecordElement(exp.clone())?,
        Deref @ Expression::BOX { .. } => Arc::new(Expression::NFExpression::BOX { exp: simplify(var_field!((*exp).exp, Expression::NFExpression::BOX).clone(), false)? }),
        Deref @ Expression::MUTABLE { .. } => simplify(Mutable::access(var_field!((*exp).exp, Expression::NFExpression::MUTABLE).clone()), false)?,
        Deref @ Expression::INSTANCE_NAME { .. } => Ceval::evalGetInstanceName(var_field!((*exp).scope, Expression::NFExpression::INSTANCE_NAME).clone())?,
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    old = Expression::typeOf(exp.clone());
    new = Type::simplify(old.clone())?;
    if !(referenceEq(&old.clone(),&new.clone())) {
        exp = Expression::setType(new.clone(), exp.clone())?;
    }
    Ok(exp)
}

pub fn simplifyRange(mut range: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut start_exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut stop_exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut start_exp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut stop_exp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut step_exp1: Option<Arc<Expression::NFExpression>> = None;
    let mut step_exp2: Option<Arc<Expression::NFExpression>> = None;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ty2: Arc<Type::NFType> = Arc::new(Type::ANY);
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(range.clone()) {
        Deref @ Expression::RANGE { stop: __pa0, step: __pa1, start: __pa2, ty: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    stop_exp1 = __pa0.clone();
    step_exp1 = __pa1.clone();
    start_exp1 = __pa2.clone();
    ty = __pa3.clone();
    start_exp2 = simplify(start_exp1.clone(), false)?;
    step_exp2 = Util::applyOption(step_exp1.clone(), Arc::new({ let __pe_b1 = false; move |__pe_a0| simplify(__pe_a0, __pe_b1.clone()) }));
    stop_exp2 = simplify(stop_exp1.clone(), false)?;
    ty2 = Type::simplify(ty.clone())?;
    if referenceEq(&start_exp1.clone(),&start_exp2.clone()) && referenceEq(&step_exp1.clone(),&step_exp2.clone()) && referenceEq(&stop_exp1.clone(),&stop_exp2.clone()) && referenceEq(&ty.clone(),&ty2.clone()) {
        exp = range.clone();
    } else {
        if !(Type::isResizable(ty.clone())) {
            ty = TypeCheck::getRangeType(start_exp2.clone(), step_exp2.clone(), stop_exp2.clone(), Type::arrayElementType(ty.clone()), Absyn::dummyInfo.clone())?;
        } else {
            ty = ty2.clone();
        }
        exp = Arc::new(Expression::NFExpression::RANGE { ty: ty.clone(), start: start_exp2.clone(), step: step_exp2.clone(), stop: stop_exp2.clone() });
    }
    Ok(exp)
}

pub fn simplifyCall(mut callExp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut callExp: Arc<Expression::NFExpression> = callExp;
    let mut call: Arc<Call::NFCall>;
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut builtin: bool = false;
    let mut is_pure: bool = false;
    let mut scalarize: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(callExp.clone()) {
        Deref @ Expression::CALL { call: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    call = __pa0.clone();
    callExp = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::TYPED_CALL { arguments: args, .. } if (!(Call::isExternal(call.clone())?)) => {
            let mut args = (*args).clone();
            if Flags::isSet(Flags::NF_EXPAND_FUNC_ARGS.clone())? {
                args = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut arg in (args.clone()).into_iter().cloned() {
            let __x = if (Expression::hasArrayCall(arg.clone())?) {arg.clone()} else {(ExpandExp::expand(arg.clone(), false, false)?).0};
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            }
            args = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut arg in (args.clone()).into_iter().cloned() {
            let __x = simplify(arg.clone(), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            assign_variant_field!(call => Call::NFCall::TYPED_CALL; arguments = args.clone());
            builtin = Function::isBuiltin(var_field!((*call).r#fn, Call::NFCall::TYPED_CALL).clone());
            is_pure = !(Function::isImpure(var_field!((*call).r#fn, Call::NFCall::TYPED_CALL).clone()));
            if builtin.clone() {
                scalarize = Flags::isSet(Flags::NF_SCALARIZE.clone())?;
                if is_pure.clone() && List::all(args.clone(), (std::sync::Arc::new(fnptr!(Expression::isLiteral, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>)) && (scalarize.clone() || Type::isScalar(var_field!((*call).ty, Call::NFCall::TYPED_CALL).clone())) {
                    match '__try0: {
                        callExp = unwrap_break_err!(Ceval::evalCall(call.clone(), Ceval::noTarget().clone()), '__try0);
                        Ok::<_, anyhow::Error>((callExp.clone(),))
                    } {
                        Ok((__try0_o0,)) => {
                            callExp = __try0_o0;
                        }
                        Err(_) => {
                            callExp = Arc::new(Expression::NFExpression::CALL { call: call.clone() });
                        }
                    }
                } else {
                    callExp = simplifyBuiltinCall(Function::nameConsiderBuiltin(var_field!((*call).r#fn, Call::NFCall::TYPED_CALL).clone())?, args.clone(), call.clone(), scalarize.clone())?;
                }
            } else if Flags::isSet(Flags::NF_EVAL_CONST_ARG_FUNCS.clone())? && is_pure.clone() && List::all(args.clone(), (std::sync::Arc::new(fnptr!(Expression::isLiteral, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>)) {
                callExp = simplifyCall2(call.clone())?;
            } else {
                callExp = Arc::new(Expression::NFExpression::CALL { call: call.clone() });
            }
            callExp.clone()
        },
        Deref @ Call::TYPED_CALL { arguments: args, .. } => {
            let mut args = (*args).clone();
            args = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut arg in (args.clone()).into_iter().cloned() {
            let __x = simplify(arg.clone(), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            assign_variant_field!(call => Call::NFCall::TYPED_CALL; arguments = args.clone());
            Arc::new(Expression::NFExpression::CALL { call: call.clone() })
        },
        Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } => simplifyArrayConstructor(call.clone())?,
        Deref @ Call::TYPED_REDUCTION { .. } => simplifyReduction(call.clone())?,
        _ => callExp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(callExp)
}

pub fn simplifyCall2(mut call: Arc<Call::NFCall>) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    ErrorExt::setCheckpoint((literal!("NFSimplifyExp.simplifyCall2")).clone());
    match '__try0: {
        outExp = unwrap_break_err!(Ceval::evalCall(call.clone(), Ceval::noTarget().clone()), '__try0);
        ErrorExt::delCheckpoint((literal!("NFSimplifyExp.simplifyCall2")).clone());
        Ok::<_, anyhow::Error>((outExp.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outExp = __try0_o0;
        }
        Err(_) => {
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                ErrorExt::delCheckpoint((literal!("NFSimplifyExp.simplifyCall2")).clone());
                Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ")); __mm_s.push_str(&*literal!("NFSimplifyExp.simplifyCall2")); __mm_s.push_str(&*literal!(" failed to evaluate ")); __mm_s.push_str(&*Call::toString(call.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
            } else {
                ErrorExt::rollBack((literal!("NFSimplifyExp.simplifyCall2")).clone());
            }
            outExp = Arc::new(Expression::NFExpression::CALL { call: call.clone() });
        }
    }
    Ok(outExp)
}

pub fn simplifyBuiltinCall(mut name: Arc<Absyn::Path>, mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut call: Arc<Call::NFCall>, mut expand: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (::match_deref::match_deref! { match &(AbsynUtil::pathFirstIdent(name.clone())?) {
        Deref @ "cat" => {
            if !(Flags::getConfigBool(Flags::NEW_BACKEND.clone())?) || List::all(args.clone(), (std::sync::Arc::new(fnptr!(Expression::isLiteral, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>)) {
                (exp, _) = ExpandExp::expandBuiltinCat(args.clone(), call.clone(), false)?;
            } else {
                exp = simplifyCat(args.clone(), call.clone())?;
            }
            exp.clone()
        },
        Deref @ "pre" => (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: exp @ Deref @ Expression::BOOLEAN { .. }, tail: Deref @ metamodelica::List::Nil } => exp.clone(),
        _ => Arc::new(Expression::NFExpression::CALL { call: call.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }),
        Deref @ "delay" => simplifyDelay(args.clone(), call.clone())?,
        Deref @ "der" => simplifyDer(listHead(args.clone())?, call.clone())?,
        Deref @ "fill" => simplifyFill(listHead(args.clone())?, listRest(args.clone())?, call.clone(), expand.clone())?,
        Deref @ "homotopy" => simplifyHomotopy(args.clone(), call.clone())?,
        Deref @ "max" => simplifyMinMax(args.clone(), call.clone(), false)?,
        Deref @ "min" => simplifyMinMax(args.clone(), call.clone(), true)?,
        Deref @ "ones" => simplifyFill(Arc::new(Expression::NFExpression::INTEGER { value: 1 }), args.clone(), call.clone(), expand.clone())?,
        Deref @ "product" => simplifySumProduct(listHead(args.clone())?, call.clone(), expand.clone(), false)?,
        Deref @ "sum" => simplifySumProduct(listHead(args.clone())?, call.clone(), expand.clone(), true)?,
        Deref @ "transpose" => simplifyTranspose(listHead(args.clone())?, call.clone(), expand.clone())?,
        Deref @ "vector" => simplifyVector(listHead(args.clone())?, call.clone())?,
        Deref @ "zeros" => simplifyFill(Arc::new(Expression::NFExpression::INTEGER { value: 0 }), args.clone(), call.clone(), expand.clone())?,
        Deref @ "semiLinear" => simplifySemiLinear(args.clone(), call.clone())?,
        Deref @ "$OMC$PositiveMax" => simplifyPositiveMax(args.clone(), call.clone())?,
        Deref @ "$OMC$inStreamDiv" => simplifyInStreamDiv(args.clone(), call.clone(), false)?,
        Deref @ "OpenModelica_uriToFilename" => simplifyURIToFilename(listHead(args.clone())?, call.clone())?,
        _ => Arc::new(Expression::NFExpression::CALL { call: call.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn simplifyCat(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut call: Arc<Call::NFCall>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut nonempty_args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut arg in (args.clone()).into_iter().cloned() {
            if !(!(Expression::sizeZero(arg.clone()))) { continue; }
            let __x = arg.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    if (nonempty_args.clone().len() as i32) == 2 {
        let __pa0 = ::match_deref::match_deref! { match &(nonempty_args.clone()) {
            Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        exp = __pa0.clone();
    } else if (nonempty_args.clone().len() as i32) == 1 {
        let __pa2 = ::match_deref::match_deref! { match &(args.clone()) {
            Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: _ } } => __pa2.clone(),
            _ => bail!("pattern mismatch"),
        } };
        exp = __pa2.clone();
    } else {
        exp = Arc::new(Expression::NFExpression::CALL { call: Call::setArguments(call.clone(), nonempty_args.clone())? });
    }
    Ok(exp)
}

pub fn simplifySemiLinear(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut call: Arc<Call::NFCall>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut x: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut m1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut m2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    x = __pa0.clone();
    m1 = __pa1.clone();
    m2 = __pa2.clone();
    ty = Expression::typeOf(x.clone());
    if Expression::isZero(x.clone()) || Expression::isZero(m1.clone()) && Expression::isZero(m2.clone()) {
        exp = Expression::makeZero(ty.clone())?;
    } else if Expression::isEqual(m1.clone(), m2.clone())? {
        exp = Arc::new(Expression::NFExpression::BINARY { exp1: x.clone(), operator: Operator::makeMul(ty.clone()), exp2: m1.clone() });
    } else {
        exp = Arc::new(Expression::NFExpression::CALL { call: call.clone() });
    }
    Ok(exp)
}

pub fn simplifyMinMax(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut call: Arc<Call::NFCall>, mut isMin: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    if (args.clone().len() as i32) == 1 {
        arg = listHead(args.clone())?;
        ty = Expression::typeOf(arg.clone());
        if Type::isEmptyArray(ty.clone()) {
            ty = Type::arrayElementType(ty.clone());
            exp = if (isMin.clone()) {Expression::makeMaxValue(ty.clone())?} else {Expression::makeMinValue(ty.clone())?};
        } else {
            exp = simplifyReducedArrayConstructor(arg.clone(), call.clone())?;
        }
    } else {
        exp = Arc::new(Expression::NFExpression::CALL { call: call.clone() });
    }
    Ok(exp)
}

pub fn simplifyPositiveMax(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut call: Arc<Call::NFCall>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut flow_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut eps: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    flow_exp = __pa0.clone();
    eps = __pa1.clone();
    if Expression::isNonPositive(flow_exp.clone()) {
        exp = Expression::makeZero(Expression::typeOf(flow_exp.clone()))?;
    } else if Expression::isGreaterOrEqual(flow_exp.clone(), eps.clone()) {
        exp = flow_exp.clone();
    } else {
        exp = Arc::new(Expression::NFExpression::CALL { call: call.clone() });
    }
    Ok(exp)
}

pub fn simplifyInStreamDiv(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut call: Arc<Call::NFCall>, mut removeStream: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut stream_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut fallback: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    stream_exp = __pa0.clone();
    fallback = __pa1.clone();
    if Expression::isNaN(stream_exp.clone())? {
        exp = fallback.clone();
    } else if removeStream.clone() {
        exp = stream_exp.clone();
    } else {
        exp = Arc::new(Expression::NFExpression::CALL { call: call.clone() });
    }
    Ok(exp)
}

pub fn removeStream(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = Expression::mapReverse(exp.clone(), (std::sync::Arc::new(removeInStreamDiv) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    Ok(exp)
}

pub fn removeInStreamDiv(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } if (literal!("$OMC$inStreamDiv") == AbsynUtil::pathFirstIdent(Function::nameConsiderBuiltin(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL).clone())?)?) => {
            let mut res: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            res = simplify(Expression::map(listHead(var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone())?, (std::sync::Arc::new(removePositiveMax) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?, true)?;
            simplifyInStreamDiv(cons(res.clone(), listRest(var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone())?), call.clone(), true)?
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn removePositiveMax(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } if (literal!("$OMC$PositiveMax") == AbsynUtil::pathFirstIdent(Function::nameConsiderBuiltin(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL).clone())?)?) => {
            let mut res: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            res = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::MAX_REAL().clone(), var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone(), Expression::variability(listHead(var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone())?)?, Purity::PURE.clone(), NFBuiltinFuncs::MAX_REAL().returnType.clone()) });
            res.clone()
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn simplifySumProduct(mut arg: Arc<Expression::NFExpression>, mut call: Arc<Call::NFCall>, mut expand: bool, mut isSum: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expanded: bool = false;
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut op: Arc<Operator::NFOperator>;
    if expand.clone() {
        (exp, expanded) = ExpandExp::expand(arg.clone(), false, false)?;
        if expanded.clone() {
            args = Expression::arrayScalarElements(exp.clone());
            ty = Type::arrayElementType(Expression::typeOf(arg.clone()));
            if args.clone().is_empty() {
                exp = if (isSum.clone()) {Expression::makeZero(ty.clone())?} else {Expression::makeOne(ty.clone())?};
            } else {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(args.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                exp = __pa0.clone();
                args = __pa1.clone();
                op = if (isSum.clone()) {Operator::makeAdd(ty.clone())} else {Operator::makeMul(ty.clone())};
                for mut e in &*args.clone() {
                    let mut e = e.clone();
                    exp = Arc::new(Expression::NFExpression::BINARY { exp1: exp.clone(), operator: op.clone(), exp2: e.clone() });
                }
            }
            return Ok(exp);
        }
    }
    exp = simplifyReducedArrayConstructor(arg.clone(), call.clone())?;
    Ok(exp)
}

pub fn simplifyReducedArrayConstructor(mut arg: Arc<Expression::NFExpression>, mut call: Arc<Call::NFCall>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::CALL { call: arr_call @ Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } } if (Type::dimensionCount(var_field!((**arr_call).ty, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone()) == 1) => {
            let mut r#fn: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut var: Variability = Variability::CONSTANT;
            let mut purity: Purity = Purity::PURE;
            let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(call.clone()) {
                Deref @ Call::TYPED_CALL { purity: __pa0, var: __pa1, ty: __pa2, r#fn: __pa3, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            purity = __pa0.clone();
            var = __pa1.clone();
            ty = __pa2.clone();
            r#fn = __pa3.clone();
            Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedReduction(r#fn.clone(), ty.clone(), var.clone(), purity.clone(), var_field!((**arr_call).exp, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), var_field!((**arr_call).iters, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), Absyn::dummyInfo.clone())? })
        },
        _ => {
            Arc::new(Expression::NFExpression::CALL { call: call.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn simplifyTranspose(mut arg: Arc<Expression::NFExpression>, mut call: Arc<Call::NFCall>, mut expand: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    e = if (!(expand.clone()) || Expression::hasArrayCall(arg.clone())?) {arg.clone()} else {(ExpandExp::expand(arg.clone(), false, false)?).0};
    exp = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ Expression::ARRAY { .. } if (Array::all(var_field!((*e).elements, Expression::NFExpression::ARRAY).clone(), (std::sync::Arc::new(fnptr!(Expression::isArray, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>))) => Expression::transposeArray(e.clone())?,
        _ => Arc::new(Expression::NFExpression::CALL { call: call.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn simplifyVector(mut arg: Arc<Expression::NFExpression>, mut call: Arc<Call::NFCall>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut is_literal: bool = false;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    expl = Expression::arrayScalarElements(arg.clone());
    is_literal = Expression::isLiteral(arg.clone());
    if is_literal.clone() {
        (expl, _) = ExpandExp::expandList(expl.clone(), true)?;
    }
    if is_literal.clone() || List::all(expl.clone(), (std::sync::Arc::new(fnptr!(Expression::isScalar, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>)) {
        ty = Type::arrayElementType(Expression::typeOf(arg.clone()));
        exp = Expression::makeExpArray(metamodelica::arrayFromVec(expl.clone().into_iter().cloned().collect()), ty.clone(), false);
    } else {
        exp = Arc::new(Expression::NFExpression::CALL { call: call.clone() });
    }
    Ok(exp)
}

pub fn simplifyFill(mut fillArg: Arc<Expression::NFExpression>, mut dimArgs: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut call: Arc<Call::NFCall>, mut expand: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    if List::all(dimArgs.clone(), (std::sync::Arc::new(fnptr!(Expression::isLiteral, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>)) && expand.clone() {
        exp = Expression::fillArgs(fillArg.clone(), dimArgs.clone())?;
    } else {
        exp = Arc::new(Expression::NFExpression::CALL { call: call.clone() });
    }
    Ok(exp)
}

pub fn simplifyHomotopy(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut call: Arc<Call::NFCall>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (::match_deref::match_deref! { match &(Flags::getConfigString(Flags::REPLACE_HOMOTOPY.clone())?) {
        Deref @ "actual" => listHead(args.clone())?,
        Deref @ "simplified" => listHead(listRest(args.clone())?)?,
        _ => Arc::new(Expression::NFExpression::CALL { call: call.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn simplifyDelay(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut call: Arc<Call::NFCall>) -> Result<Arc<Expression::NFExpression>> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut delayTime: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: _ } } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    exp = __pa0.clone();
    delayTime = __pa1.clone();
    if Expression::variability(delayTime.clone())? <= Variability::PARAMETER.clone() {
        delayTime = Ceval::tryEvalExp(delayTime.clone(), Ceval::noTarget().clone());
        if Expression::isZero(delayTime.clone()) {
            callExp = exp.clone();
            return Ok(callExp);
        }
    }
    callExp = Arc::new(Expression::NFExpression::CALL { call: call.clone() });
    Ok(callExp)
}

pub fn simplifyDer(mut arg: Arc<Expression::NFExpression>, mut call: Arc<Call::NFCall>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    if Call::variability(call.clone())? < Variability::DISCRETE.clone() {
        exp = Expression::makeZero(Expression::typeOf(arg.clone()))?;
    } else {
        exp = Arc::new(Expression::NFExpression::CALL { call: call.clone() });
    }
    Ok(exp)
}

pub fn simplifyArrayConstructor(mut call: Arc<Call::NFCall>) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::CONSTANT;
    let mut pur: Purity = Purity::PURE;
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut iter: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut dim_size: i32 = 0;
    let mut expanded: bool = false;
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { ty: __pa0, var: __pa1, purity: __pa2, exp: __pa3, iters: __pa4 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ty = __pa0.clone();
    var = __pa1.clone();
    pur = __pa2.clone();
    exp = __pa3.clone();
    iters = __pa4.clone();
    iters = {
        let mut __acc: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
        for mut i in (iters.clone()).into_iter().cloned() {
            let __x = (Util::tuple21(i.clone()), simplify(Util::tuple22(i.clone()), false)?);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outExp = 'mc: {
        let __mc_input = iters.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (iter, e), tail: Deref @ metamodelica::List::Nil } => {
                    let mut e = (*e).clone();
                    let mut expanded: bool = expanded.clone();
                    let mut dim_size: i32 = dim_size.clone();
                    let mut dim: Arc<Dimension::NFDimension> = dim.clone();
                    let mut outExp: Arc<Expression::NFExpression> = outExp.clone();
                    let mut exp: Arc<Expression::NFExpression> = exp.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::typeOf(e.clone())) {
                        Deref @ Type::ARRAY { dimensions: Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    dim = __pa0.clone();
                    dim_size = Dimension::size(dim.clone(), false)?;
                    if dim_size.clone() == 0 {
                        outExp = Expression::makeEmptyArray(ty.clone());
                    } else if dim_size.clone() == 1 {
                        (e, _) = ExpandExp::expand(e.clone(), false, false)?;
                        e = Expression::arrayScalarElement(e.clone())?;
                        exp = Expression::replaceIterator(exp.clone(), iter.clone(), e.clone())?;
                        exp = Expression::makeArray(ty.clone(), metamodelica::arrayFromVec(list![exp.clone()].into_iter().cloned().collect()), false);
                        outExp = simplify(exp.clone(), false)?;
                    } else if Expression::isLiteral(e.clone()) && isIteratorSubscriptedArray(exp.clone(), iter.clone()) {
                        (outExp, expanded) = ExpandExp::expandArrayConstructor(exp.clone(), ty.clone(), iters.clone())?;
                        if expanded.clone() {
                            outExp = simplify(outExp.clone(), false)?;
                        }
                    } else {
                        bail!("fail");
                    }
                    Ok(outExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut ty: Arc<Type::NFType> = ty.clone();
                    let mut exp: Arc<Expression::NFExpression> = exp.clone();
                    exp = simplify(exp.clone(), false)?;
                    ty = Type::simplify(ty.clone())?;
                    Ok(Arc::new(Expression::NFExpression::CALL { call: Arc::new(Call::NFCall::TYPED_ARRAY_CONSTRUCTOR { ty: ty.clone(), var: var.clone(), purity: pur.clone(), exp: exp.clone(), iters: iters.clone() }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

pub fn isIteratorSubscriptedArray(mut exp: Arc<Expression::NFExpression>, mut iterator: Arc<InstNode::InstNode>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::SUBSCRIPTED_EXP { .. } => Expression::isArray(var_field!((*exp).exp, Expression::NFExpression::SUBSCRIPTED_EXP).clone()) && List::all(var_field!((*exp).subscripts, Expression::NFExpression::SUBSCRIPTED_EXP).clone(), Arc::new({ let __pe_b1 = iterator.clone(); move |__pe_a0| Subscript::equalsIterator(__pe_a0, __pe_b1.clone()) })),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn simplifyReduction(mut call: Arc<Call::NFCall>) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    outExp = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::TYPED_REDUCTION { .. } => {
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
            let mut iter: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
            let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
            let mut dim_size: i32 = 0;
            iters = {
        let mut __acc: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
        for mut i in (var_field!((*call).iters, Call::NFCall::TYPED_REDUCTION).clone()).into_iter().cloned() {
            let __x = (Util::tuple21(i.clone()), simplify(Util::tuple22(i.clone()), false)?);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            'mc: {
        let __mc_input = iters.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (iter, e), tail: Deref @ metamodelica::List::Nil } => {
                    let mut e = (*e).clone();
                    let mut dim: Arc<Dimension::NFDimension>;
                    let mut outExp: Arc<Expression::NFExpression> = outExp.clone();
                    let mut dim_size: i32;
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::typeOf(e.clone())) {
                        Deref @ Type::ARRAY { dimensions: Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    dim = __pa0.clone();
                    dim_size = Dimension::size(dim.clone(), false)?;
                    if dim_size.clone() == 0 {
                        let __pa2 = ::match_deref::match_deref! { match &(var_field!((*call).defaultExp, Call::NFCall::TYPED_REDUCTION).clone()) {
                            Some(__pa2) => __pa2.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        outExp = __pa2.clone();
                    } else if dim_size.clone() == 1 {
                        (e, _) = ExpandExp::expand(e.clone(), false, false)?;
                        e = Expression::arrayScalarElement(e.clone())?;
                        outExp = Expression::replaceIterator(var_field!((*call).exp, Call::NFCall::TYPED_REDUCTION).clone(), iter.clone(), e.clone())?;
                        outExp = simplify(outExp.clone(), false)?;
                    } else {
                        bail!("fail");
                    }
                    Ok(outExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if !((var_field!((*call).var, Call::NFCall::TYPED_REDUCTION).clone() <= Variability::STRUCTURAL_PARAMETER.clone())) { bail!("guard") }
                    Ok(Ceval::tryEvalExp(Arc::new(Expression::NFExpression::CALL { call: call.clone() }), Ceval::noTarget().clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if !((Flags::isSet(Flags::NF_SCALARIZE.clone())?)) { bail!("guard") }
                    Ok(simplifyReduction2((AbsynUtil::pathString(Function::name(var_field!((*call).r#fn, Call::NFCall::TYPED_REDUCTION).clone()), (literal!(".")).clone(), true, false)?).clone(), var_field!((*call).exp, Call::NFCall::TYPED_REDUCTION).clone(), iters.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut call: Arc<Call::NFCall> = call.clone();
                    assign_variant_field!(call => Call::NFCall::TYPED_REDUCTION;
                        exp = simplify(var_field!((*call).exp, Call::NFCall::TYPED_REDUCTION).clone(), false)?,
                        iters = iters.clone()
                    );
                    Ok(Arc::new(Expression::NFExpression::CALL { call: call.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

pub fn simplifyReduction2(mut name: ArcStr, mut exp: Arc<Expression::NFExpression>, mut iterators: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>>) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut iter: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut range: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut default_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expanded: bool = true;
    let mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut op: Arc<Operator::NFOperator>;
    ty = Expression::typeOf(exp.clone());
    let false = (Type::isRecord(Type::arrayElementType(ty.clone()))) else { bail!("pattern mismatch") };
    (default_exp, op) = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "sum" => (Expression::makeZero(ty.clone())?, Operator::makeAdd(ty.clone())),
        Deref @ "product" => (Expression::makeOne(ty.clone())?, Operator::makeMul(ty.clone())),
        _ => bail!("match: no arm matched"),
    } });
    for mut i in &*iterators.clone() {
        let mut i = i.clone();
        (iter, range) = i.clone();
        let __pa0 = ::match_deref::match_deref! { match &(ExpandExp::expand(range.clone(), false, false)?) {
            (__pa0, true) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        range = __pa0.clone();
        iters = cons((iter.clone(), range.clone()), iters.clone());
    }
    outExp = Expression::foldReduction(simplify(exp.clone(), false)?, iters.clone().reverse(), default_exp.clone(), Arc::new({ let __pe_b1 = false; move |__pe_a0| simplify(__pe_a0, __pe_b1.clone()) }), Arc::new({ let __pe_b1 = op.clone(); move |__pe_a0, __pe_a2| simplifyBinaryOp(__pe_a0, __pe_b1.clone(), __pe_a2) }))?;
    Ok(outExp)
}

pub fn simplifySize(mut sizeExp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut sizeExp: Arc<Expression::NFExpression> = sizeExp;
    sizeExp = (::match_deref::match_deref! { match &(sizeExp.clone()) {
        Deref @ Expression::SIZE { exp, dimIndex: Some(index) } => {
            let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
            let mut exp = (*exp).clone();
            let mut index = (*index).clone();
            index = simplify(index.clone(), false)?;
            if Expression::isLiteral(index.clone()) {
                dim = (Type::arrayDims(Expression::typeOf(exp.clone()))).get(Expression::toInteger(index.clone())?)?;
                if Dimension::isKnown(dim.clone(), false) {
                    exp = Arc::new(Expression::NFExpression::INTEGER { value: Dimension::size(dim.clone(), false)? });
                } else {
                    exp = Arc::new(Expression::NFExpression::SIZE { exp: exp.clone(), dimIndex: Some(index.clone()) });
                }
            } else {
                exp = Arc::new(Expression::NFExpression::SIZE { exp: exp.clone(), dimIndex: Some(index.clone()) });
            }
            exp.clone()
        },
        Deref @ Expression::SIZE { .. } => {
            let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
            dims = Type::arrayDims(Expression::typeOf(var_field!((*sizeExp).exp, Expression::NFExpression::SIZE).clone()));
            if List::all(dims.clone(), Arc::new({ let __pe_b1 = true; move |__pe_a0| Ok(Dimension::isKnown(__pe_a0, __pe_b1.clone())) })) {
                exp = Expression::makeArray(Arc::new(Type::NFType::ARRAY { elementType: Arc::new(crate::NFType::INTEGER), dimensions: list![Dimension::fromInteger((dims.clone().len() as i32), Variability::CONSTANT.clone())] }), metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut d in (dims.clone()).into_iter().cloned() {
            let __x = Dimension::sizeExp(d.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect()), false);
            } else {
                exp = sizeExp.clone();
            }
            exp.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(sizeExp)
}

pub fn simplifyMultary(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::MULTARY { operator, inv_arguments: Deref @ metamodelica::List::Nil, arguments: Deref @ metamodelica::List::Nil } if (Operator::isDashClassification(Operator::getMathClassification(operator.clone())?)) => {
            Expression::makeZero(operator.ty.clone())?
        },
        Deref @ Expression::MULTARY { operator, inv_arguments: Deref @ metamodelica::List::Nil, arguments: Deref @ metamodelica::List::Nil } => {
            Expression::makeOne(operator.ty.clone())?
        },
        Deref @ Expression::MULTARY { inv_arguments: Deref @ metamodelica::List::Nil, arguments: Deref @ metamodelica::List::Cons { head: tmp, tail: Deref @ metamodelica::List::Nil }, .. } => {
            simplify(tmp.clone(), false)?
        },
        Deref @ Expression::MULTARY { operator, inv_arguments, arguments } => {
            let mut const_args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            let mut inv_const_args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            let mut new_const: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut tmp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut mcl: Operator::MathClassification = Operator::MathClassification::ADDITION;
            let mut neutralConst: bool = false;
            let mut isNegative: bool = false;
            let mut inv_arguments = (*inv_arguments).clone();
            let mut arguments = (*arguments).clone();
            mcl = Operator::getMathClassification(operator.clone())?;
            arguments = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut arg in (arguments.clone()).into_iter().cloned() {
            let __x = simplify(arg.clone(), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            inv_arguments = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut arg in (inv_arguments.clone()).into_iter().cloned() {
            let __x = simplify(arg.clone(), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            (arguments, inv_arguments, isNegative) = simplifyMultarySigns(arguments.clone(), inv_arguments.clone(), mcl.clone())?;
            (const_args, arguments) = List::splitOnTrue(arguments.clone(), (std::sync::Arc::new(fnptr!(Expression::isLiteral, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>));
            (inv_const_args, inv_arguments) = List::splitOnTrue(inv_arguments.clone(), (std::sync::Arc::new(fnptr!(Expression::isLiteral, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>));
            if mcl.clone() == Operator::MathClassification::ADDITION.clone() {
                (new_const, neutralConst) = Ceval::evalMultaryAddSub(const_args.clone(), inv_const_args.clone(), Operator::typeOf(operator.clone()))?;
            } else if mcl.clone() == Operator::MathClassification::MULTIPLICATION.clone() {
                (new_const, neutralConst) = Ceval::evalMultaryMulDiv(const_args.clone(), inv_const_args.clone(), Operator::typeOf(operator.clone()))?;
            } else {
                Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFSimplifyExp.simplifyMultary")); __mm_s.push_str(&*literal!(" detected non-commutative operator in MULTARY(): [")); __mm_s.push_str(&*Operator::mathSymbol(mcl.clone())?); __mm_s.push_str(&*literal!("]\n with following arguments: ")); __mm_s.push_str(&*stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (const_args.clone()).into_iter().cloned() {
            let __x = Expression::toString(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n and following inverse arguments: ")); __mm_s.push_str(&*stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (inv_const_args.clone()).into_iter().cloned() {
            let __x = Expression::toString(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(", ")).clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            }
            (arguments, inv_arguments) = cancelTermsInMultary(arguments.clone(), inv_arguments.clone())?;
            result = (::match_deref::match_deref! { match &((mcl.clone(), arguments.clone(), inv_arguments.clone())) {
        (Operator::MathClassification::ADDITION, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => if (Expression::isEmpty(new_const.clone())) {Expression::makeZero(Expression::typeOf(new_const.clone()))?} else {new_const.clone()},
        (Operator::MathClassification::MULTIPLICATION, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => if (Expression::isEmpty(new_const.clone())) {Expression::makeOne(Expression::typeOf(new_const.clone()))?} else {new_const.clone()},
        (_, Deref @ metamodelica::List::Cons { head: tmp, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Nil) if (neutralConst.clone()) => tmp.clone(),
        (Operator::MathClassification::ADDITION, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Cons { head: tmp, tail: Deref @ metamodelica::List::Nil }) if (neutralConst.clone()) => Expression::negate(tmp.clone()),
        (Operator::MathClassification::MULTIPLICATION, _, _) if (Expression::isZero(new_const.clone())) => new_const.clone(),
        _ => Arc::new(Expression::NFExpression::MULTARY { operator: operator.clone(), inv_arguments: inv_arguments.clone(), arguments: if (neutralConst.clone()) {arguments.clone()} else {cons(new_const.clone(), arguments.clone())} }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if (isNegative.clone()) {Expression::negate(result.clone())} else {result.clone()}
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFSimplifyExp.simplifyMultary")); __mm_s.push_str(&*literal!(" failed for expression: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn simplifyMultarySigns(mut arguments: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut inv_arguments: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut mcl: Operator::MathClassification) -> Result<(Arc<metamodelica::List<Arc<Expression::NFExpression>>>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>, bool)> {
    let mut new_arguments: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut new_inv_arguments: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut isNegative: bool = false;
    let _ = (match mcl.clone() {
        Operator::MathClassification::ADDITION => {
            for mut arg in &*arguments.clone().reverse() {
                let mut arg = arg.clone();
                if Expression::isNegated(arg.clone()) {
                    new_inv_arguments = cons(Expression::negate(arg.clone()), new_inv_arguments.clone());
                } else {
                    new_arguments = cons(arg.clone(), new_arguments.clone());
                }
            }
            for mut arg in &*inv_arguments.clone().reverse() {
                let mut arg = arg.clone();
                if Expression::isNegated(arg.clone()) {
                    new_arguments = cons(Expression::negate(arg.clone()), new_arguments.clone());
                } else {
                    new_inv_arguments = cons(arg.clone(), new_inv_arguments.clone());
                }
            }
            ()
        },
        Operator::MathClassification::MULTIPLICATION => {
            for mut arg in &*arguments.clone().reverse() {
                let mut arg = arg.clone();
                if Expression::isNegated(arg.clone()) {
                    new_arguments = cons(Expression::negate(arg.clone()), new_arguments.clone());
                    isNegative = !(isNegative.clone());
                } else {
                    new_arguments = cons(arg.clone(), new_arguments.clone());
                }
            }
            for mut arg in &*inv_arguments.clone().reverse() {
                let mut arg = arg.clone();
                if Expression::isNegated(arg.clone()) {
                    new_inv_arguments = cons(Expression::negate(arg.clone()), new_inv_arguments.clone());
                    isNegative = !(isNegative.clone());
                } else {
                    new_inv_arguments = cons(arg.clone(), new_inv_arguments.clone());
                }
            }
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFSimplifyExp.simplifyMultarySigns")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
    });
    Ok((new_arguments, new_inv_arguments, isNegative))
}

pub fn simplifyBinary(mut binaryExp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut binaryExp: Arc<Expression::NFExpression> = binaryExp;
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut se1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut se2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut op: Arc<Operator::NFOperator>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(binaryExp.clone()) {
        Deref @ Expression::BINARY { exp1: __pa0, operator: __pa1, exp2: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    e1 = __pa0.clone();
    op = __pa1.clone();
    e2 = __pa2.clone();
    se1 = simplify(e1.clone(), false)?;
    se2 = simplify(e2.clone(), false)?;
    binaryExp = simplifyBinaryOp(se1.clone(), op.clone(), se2.clone())?;
    if Flags::isSet(Flags::NF_EXPAND_OPERATIONS.clone())? && !(Expression::hasArrayCall(binaryExp.clone())?) {
        (binaryExp, _) = ExpandExp::expand(binaryExp.clone(), false, false)?;
    }
    Ok(binaryExp)
}

pub fn simplifyBinaryOp(mut exp1: Arc<Expression::NFExpression>, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    if Expression::isLiteral(exp1.clone()) && Expression::isLiteral(exp2.clone()) {
        outExp = Ceval::evalBinaryOp((ExpandExp::expand(exp1.clone(), false, false)?).0, op.clone(), (ExpandExp::expand(exp2.clone(), false, false)?).0, Ceval::noTarget().clone())?;
    } else if Expression::isArray(exp1.clone()) && Expression::isArray(exp2.clone()) {
        outExp = (match op.op.clone() {
        Operator::Op::ADD => simplifyBinaryEW(exp1.clone(), op.clone(), exp2.clone())?,
        Operator::Op::SUB => simplifyBinaryEW(exp1.clone(), op.clone(), exp2.clone())?,
        Operator::Op::ADD_EW => simplifyBinaryEW(exp1.clone(), op.clone(), exp2.clone())?,
        Operator::Op::SUB_EW => simplifyBinaryEW(exp1.clone(), op.clone(), exp2.clone())?,
        Operator::Op::MUL_EW => simplifyBinaryEW(exp1.clone(), op.clone(), exp2.clone())?,
        Operator::Op::DIV_EW => simplifyBinaryEW(exp1.clone(), op.clone(), exp2.clone())?,
        Operator::Op::POW_EW => simplifyBinaryEW(exp1.clone(), op.clone(), exp2.clone())?,
        _ => Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() }),
    });
    } else {
        outExp = (match op.op.clone() {
        Operator::Op::ADD => simplifyBinaryAdd(exp1.clone(), op.clone(), exp2.clone())?,
        Operator::Op::SUB => simplifyBinarySub(exp1.clone(), op.clone(), exp2.clone())?,
        Operator::Op::MUL => simplifyBinaryMul(exp1.clone(), op.clone(), exp2.clone(), false),
        Operator::Op::DIV => simplifyBinaryDiv(exp1.clone(), op.clone(), exp2.clone())?,
        Operator::Op::POW => simplifyBinaryPow(exp1.clone(), op.clone(), exp2.clone())?,
        Operator::Op::SCALAR_PRODUCT if (Expression::isZero(exp1.clone()) || Expression::isZero(exp2.clone())) => Expression::makeZero(op.ty.clone())?,
        _ => Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() }),
    });
    }
    Ok(outExp)
}

pub fn simplifyBinaryAdd(mut exp1: Arc<Expression::NFExpression>, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    if Expression::isZero(exp1.clone()) {
        outExp = exp2.clone();
    } else if Expression::isZero(exp2.clone()) {
        outExp = exp1.clone();
    } else if Expression::isNegated(exp1.clone()) {
        if Expression::isNegated(exp2.clone()) {
            outExp = Expression::negate(Arc::new(Expression::NFExpression::BINARY { exp1: Expression::negate(exp1.clone()), operator: op.clone(), exp2: Expression::negate(exp2.clone()) }));
        } else {
            outExp = simplifyBinarySub(exp2.clone(), Operator::invert(op.clone())?, Expression::negate(exp1.clone()))?;
        }
    } else if Expression::isNegated(exp2.clone()) {
        outExp = simplifyBinarySub(exp1.clone(), Operator::invert(op.clone())?, Expression::negate(exp2.clone()))?;
    } else {
        outExp = Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() });
    }
    Ok(outExp)
}

pub fn simplifyBinarySub(mut exp1: Arc<Expression::NFExpression>, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    if Expression::isZero(exp1.clone()) {
        outExp = Expression::negate(exp2.clone());
    } else if Expression::isZero(exp2.clone()) {
        outExp = exp1.clone();
    } else if Expression::isEqual(exp1.clone(), exp2.clone())? {
        outExp = Expression::makeZero(Operator::typeOf(op.clone()))?;
    } else if Expression::isNegated(exp1.clone()) {
        if Expression::isNegated(exp2.clone()) {
            outExp = Arc::new(Expression::NFExpression::BINARY { exp1: Expression::negate(exp2.clone()), operator: op.clone(), exp2: Expression::negate(exp1.clone()) });
        } else {
            outExp = Expression::negate(simplifyBinaryAdd(Expression::negate(exp1.clone()), Operator::invert(op.clone())?, exp2.clone())?);
        }
    } else if Expression::isNegated(exp2.clone()) {
        outExp = simplifyBinaryAdd(exp1.clone(), Operator::invert(op.clone())?, Expression::negate(exp2.clone()))?;
    } else {
        outExp = Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() });
    }
    Ok(outExp)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn simplifyBinaryMul(mut exp1: Arc<Expression::NFExpression>, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut switched: bool) -> Arc<Expression::NFExpression> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    outExp = (::match_deref::match_deref! { match &(exp1.clone()) {
        Deref @ Expression::INTEGER { value: 0 } => exp1.clone(),
        Deref @ Expression::REAL { value: __rlit_0 } if __rlit_0.eq(&metamodelica::OrderedFloat((0.0) as f64)) => exp1.clone(),
        Deref @ Expression::INTEGER { value: 1 } => exp2.clone(),
        Deref @ Expression::REAL { value: __rlit_1 } if __rlit_1.eq(&metamodelica::OrderedFloat((1.0) as f64)) => exp2.clone(),
        _ => if (switched.clone()) {Arc::new(Expression::NFExpression::BINARY { exp1: exp2.clone(), operator: op.clone(), exp2: exp1.clone() })} else {simplifyBinaryMul(exp2.clone(), op.clone(), exp1.clone(), true)},
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp
}

pub fn simplifyBinaryDiv(mut exp1: Arc<Expression::NFExpression>, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    outExp = if (Expression::isOne(exp2.clone())) {exp1.clone()} else if (Expression::isMinusOne(exp2.clone())) {Expression::negate(exp1.clone())} else if (Expression::isZero(exp1.clone()) && Expression::isNonZero(exp2.clone())) {exp1.clone()} else {(match (Expression::isNegated(exp1.clone()), Expression::isNegated(exp1.clone())) {
        (true, true) => Arc::new(Expression::NFExpression::BINARY { exp1: Expression::negate(exp1.clone()), operator: op.clone(), exp2: Expression::negate(exp2.clone()) }),
        (false, true) => Expression::negate(Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: Expression::negate(exp2.clone()) })),
        (true, false) => Expression::negate(Arc::new(Expression::NFExpression::BINARY { exp1: Expression::negate(exp1.clone()), operator: op.clone(), exp2: exp2.clone() })),
        (false, false) => Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() }),
        _ => bail!("match: no arm matched"),
    })};
    Ok(outExp)
}

pub fn simplifyBinaryPow(mut exp1: Arc<Expression::NFExpression>, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    if Expression::isZero(exp2.clone()) {
        outExp = Expression::makeOne(Operator::typeOf(op.clone()))?;
    } else if Expression::isOne(exp2.clone()) {
        outExp = exp1.clone();
    } else {
        outExp = Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() });
    }
    Ok(outExp)
}

pub fn simplifyBinaryEW(mut exp1: Arc<Expression::NFExpression>, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    outExp = Expression::makeArray(Operator::typeOf(op.clone()), Array::threadMap(Expression::arrayElements(exp1.clone())?, Expression::arrayElements(exp2.clone())?, Arc::new({ let __pe_b1 = Operator::stripEW(Operator::unlift(op.clone())?); move |__pe_a0, __pe_a2| simplifyBinaryOp(__pe_a0, __pe_b1.clone(), __pe_a2) }))?, false);
    Ok(outExp)
}

pub fn simplifyUnary(mut unaryExp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut unaryExp: Arc<Expression::NFExpression> = unaryExp;
    let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut se: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut op: Arc<Operator::NFOperator>;
    unaryExp = (::match_deref::match_deref! { match &(unaryExp.clone()) {
        Deref @ Expression::UNARY { operator: _, exp: Deref @ Expression::UNARY { operator: _, exp: e } } => simplify(e.clone(), false)?,
        Deref @ Expression::UNARY { operator: op, exp: e } => {
            se = simplify(e.clone(), false)?;
            simplifyUnaryOp(se.clone(), op.clone())?
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFSimplifyExp.simplifyUnary")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if Flags::isSet(Flags::NF_EXPAND_OPERATIONS.clone())? && !(Expression::hasArrayCall(unaryExp.clone())?) {
        (unaryExp, _) = ExpandExp::expand(unaryExp.clone(), false, false)?;
    }
    Ok(unaryExp)
}

pub fn simplifyUnaryOp(mut exp: Arc<Expression::NFExpression>, mut op: Arc<Operator::NFOperator>) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    if Expression::isLiteral(exp.clone()) {
        outExp = Ceval::evalUnaryOp(exp.clone(), op.clone())?;
    } else {
        outExp = simplifyUnarySign(exp.clone(), true);
    }
    Ok(outExp)
}

pub fn simplifyUnarySign(mut unaryExp: Arc<Expression::NFExpression>, mut isNegative: bool) -> Arc<Expression::NFExpression> {
    let mut unaryExp: Arc<Expression::NFExpression> = unaryExp;
    unaryExp = (::match_deref::match_deref! { match &(unaryExp.clone()) {
        Deref @ Expression::UNARY { .. } => simplifyUnarySign(var_field!((*unaryExp).exp, Expression::NFExpression::UNARY).clone(), !(isNegative.clone())),
        _ => if (isNegative.clone()) {Expression::negate(unaryExp.clone())} else {unaryExp.clone()},
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    unaryExp
}

pub fn simplifyLogicBinary(mut binaryExp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut binaryExp: Arc<Expression::NFExpression> = binaryExp;
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut se1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut se2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut op: Arc<Operator::NFOperator>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(binaryExp.clone()) {
        Deref @ Expression::LBINARY { exp1: __pa0, operator: __pa1, exp2: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    e1 = __pa0.clone();
    op = __pa1.clone();
    e2 = __pa2.clone();
    se1 = simplify(e1.clone(), false)?;
    se2 = simplify(e2.clone(), false)?;
    binaryExp = (match op.op.clone() {
        Operator::Op::AND => simplifyLogicBinaryAnd(se1.clone(), op.clone(), se2.clone())?,
        Operator::Op::OR => simplifyLogicBinaryOr(se1.clone(), op.clone(), se2.clone())?,
        _ => bail!("match: no arm matched"),
    });
    Ok(binaryExp)
}

pub fn simplifyLogicBinaryAnd(mut exp1: Arc<Expression::NFExpression>, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::BOOLEAN { value: false }, _) => {
            exp1.clone()
        },
        (_, Deref @ Expression::BOOLEAN { value: false }) => {
            exp2.clone()
        },
        (Deref @ Expression::BOOLEAN { value: true }, _) => {
            exp2.clone()
        },
        (_, Deref @ Expression::BOOLEAN { value: true }) => {
            exp1.clone()
        },
        (Deref @ Expression::ARRAY { .. }, Deref @ Expression::ARRAY { .. }) => {
            let mut o: Arc<Operator::NFOperator>;
            let mut arr: metamodelica::Array<Arc<Expression::NFExpression>>;
            o = Operator::unlift(op.clone())?;
            arr = Array::threadMap(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone(), Arc::new({ let __pe_b1 = o.clone(); move |__pe_a0, __pe_a2| simplifyLogicBinaryAnd(__pe_a0, __pe_b1.clone(), __pe_a2) }))?;
            Expression::makeArray(Operator::typeOf(op.clone()), arr.clone(), false)
        },
        _ => {
            Arc::new(Expression::NFExpression::LBINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn simplifyLogicBinaryOr(mut exp1: Arc<Expression::NFExpression>, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::BOOLEAN { value: true }, _) => {
            exp1.clone()
        },
        (_, Deref @ Expression::BOOLEAN { value: true }) => {
            exp2.clone()
        },
        (Deref @ Expression::BOOLEAN { value: false }, _) => {
            exp2.clone()
        },
        (_, Deref @ Expression::BOOLEAN { value: false }) => {
            exp1.clone()
        },
        (Deref @ Expression::ARRAY { .. }, Deref @ Expression::ARRAY { .. }) => {
            let mut o: Arc<Operator::NFOperator>;
            let mut arr: metamodelica::Array<Arc<Expression::NFExpression>>;
            o = Operator::unlift(op.clone())?;
            arr = Array::threadMap(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone(), Arc::new({ let __pe_b1 = o.clone(); move |__pe_a0, __pe_a2| simplifyLogicBinaryOr(__pe_a0, __pe_b1.clone(), __pe_a2) }))?;
            Expression::makeArray(Operator::typeOf(op.clone()), arr.clone(), false)
        },
        _ => {
            Arc::new(Expression::NFExpression::LBINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn simplifyLogicUnary(mut unaryExp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut unaryExp: Arc<Expression::NFExpression> = unaryExp;
    let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut se: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut newExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut op: Arc<Operator::NFOperator>;
    unaryExp = (::match_deref::match_deref! { match &(unaryExp.clone()) {
        Deref @ Expression::LUNARY { operator: _, exp: Deref @ Expression::LUNARY { operator: _, exp: e } } => simplify(e.clone(), false)?,
        Deref @ Expression::LUNARY { operator: op, exp: e } => {
            se = simplify(e.clone(), false)?;
            if Expression::isLiteral(se.clone()) {
                newExp = Ceval::evalLogicUnaryOp(se.clone(), op.clone())?;
            } else if !(referenceEq(&e.clone(),&se.clone())) {
                newExp = Arc::new(Expression::NFExpression::LUNARY { operator: op.clone(), exp: se.clone() });
            } else {
                newExp = unaryExp.clone();
            }
            newExp.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(unaryExp)
}

pub fn simplifyRelation(mut relationExp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut relationExp: Arc<Expression::NFExpression> = relationExp;
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut se1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut se2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut op: Arc<Operator::NFOperator>;
    let mut index: i32 = 0;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(relationExp.clone()) {
        Deref @ Expression::RELATION { exp1: __pa0, operator: __pa1, exp2: __pa2, index: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    e1 = __pa0.clone();
    op = __pa1.clone();
    e2 = __pa2.clone();
    index = __pa3.clone();
    se1 = simplify(e1.clone(), false)?;
    se2 = simplify(e2.clone(), false)?;
    if Expression::isLiteral(se1.clone()) && Expression::isLiteral(se2.clone()) {
        relationExp = Ceval::evalRelationOp(se1.clone(), op.clone(), se2.clone())?;
    } else if !(referenceEq(&e1.clone(),&se1.clone()) && referenceEq(&e2.clone(),&se2.clone())) {
        relationExp = Arc::new(Expression::NFExpression::RELATION { exp1: se1.clone(), operator: op.clone(), exp2: se2.clone(), index: index.clone() });
    }
    Ok(relationExp)
}

pub fn simplifyIf(mut ifExp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut ifExp: Arc<Expression::NFExpression> = ifExp;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut tb: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut fb: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut tb_val: bool = false;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(ifExp.clone()) {
        Deref @ Expression::IF { ty: __pa0, condition: __pa1, trueBranch: __pa2, falseBranch: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ty = __pa0.clone();
    cond = __pa1.clone();
    tb = __pa2.clone();
    fb = __pa3.clone();
    cond = simplify(cond.clone(), false)?;
    ifExp = (::match_deref::match_deref! { match &(cond.clone()) {
        Deref @ Expression::BOOLEAN { .. } => simplify(if (var_field!((*cond).value, Expression::NFExpression::BOOLEAN).clone()) {tb.clone()} else {fb.clone()}, false)?,
        _ => {
            tb = simplify(tb.clone(), false)?;
            fb = simplify(fb.clone(), false)?;
            if Expression::isEqual(tb.clone(), fb.clone())? {
                ifExp = tb.clone();
            } else if Expression::isBoolean(tb.clone()) && Expression::isBoolean(fb.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(tb.clone()) {
                    Deref @ Expression::BOOLEAN { value: __pa0 } => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                tb_val = __pa0.clone();
                ifExp = if (tb_val.clone()) {cond.clone()} else {Expression::logicNegate(cond.clone())};
            } else {
                ifExp = Arc::new(Expression::NFExpression::IF { ty: ty.clone(), condition: cond.clone(), trueBranch: tb.clone(), falseBranch: fb.clone() });
            }
            ifExp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ifExp)
}

pub fn simplifyCast(mut exp: Arc<Expression::NFExpression>, mut ty: Arc<Type::NFType>) -> Result<Arc<Expression::NFExpression>> {
    let mut castExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    castExp = (::match_deref::match_deref! { match &((ty.clone(), exp.clone())) {
        (Deref @ Type::REAL, Deref @ Expression::INTEGER { .. }) => {
            Arc::new(Expression::NFExpression::REAL { value: intReal(var_field!((*exp).value, Expression::NFExpression::INTEGER).clone()) })
        },
        (Deref @ Type::ARRAY { elementType: Deref @ Type::REAL, .. }, Deref @ Expression::ARRAY { .. }) => {
            let mut ety: Arc<Type::NFType> = Arc::new(Type::ANY);
            ety = Type::unliftArray(ty.clone())?;
            assign_variant_field!(exp => Expression::NFExpression::ARRAY;
                elements = Array::map(var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone(), Arc::new({ let __pe_b1 = ety.clone(); move |__pe_a0| simplifyCast(__pe_a0, __pe_b1.clone()) })),
                ty = Type::setArrayElementType(var_field!((*exp).ty, Expression::NFExpression::ARRAY).clone(), Type::arrayElementType(ty.clone()))
            );
            exp.clone()
        },
        _ => {
            Arc::new(Expression::NFExpression::CAST { ty: ty.clone(), exp: exp.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(castExp)
}

pub fn simplifySubscriptedExp(mut subscriptedExp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut subscriptedExp: Arc<Expression::NFExpression> = subscriptedExp;
    let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut split: bool = false;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(subscriptedExp.clone()) {
        Deref @ Expression::SUBSCRIPTED_EXP { exp: __pa0, subscripts: __pa1, ty: __pa2, split: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    e = __pa0.clone();
    subs = __pa1.clone();
    ty = __pa2.clone();
    split = __pa3.clone();
    subscriptedExp = simplify(e.clone(), false)?;
    subs = Subscript::simplifyList(subs.clone(), Type::arrayDims(Expression::typeOf(e.clone())), false)?;
    if !(split.clone()) && !(List::all(subs.clone(), (std::sync::Arc::new(fnptr!(Subscript::isLiteral, Arc<Subscript::NFSubscript>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Subscript::NFSubscript>) -> Result<bool> + 'static>))) && Type::isScalar(ty.clone()) {
        while !(subs.clone().is_empty()) && Expression::isArray(subscriptedExp.clone()) && !(Expression::isEmptyArray(subscriptedExp.clone())) && Array::allEqual(Expression::arrayElements(subscriptedExp.clone())?, (std::sync::Arc::new(Expression::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<bool> + 'static>)) {
            subs = listRest(subs.clone())?;
            subscriptedExp = Expression::arrayElements(subscriptedExp.clone())?.borrow()[(1-1) as usize].clone();
        }
        if subs.clone().is_empty() {
            return Ok(subscriptedExp);
        }
    }
    if split.clone() {
        subscriptedExp = Arc::new(Expression::NFExpression::SUBSCRIPTED_EXP { exp: subscriptedExp.clone(), subscripts: subs.clone(), ty: ty.clone(), split: split.clone() });
    } else {
        subscriptedExp = Expression::applySubscripts(subs.clone(), subscriptedExp.clone(), false)?;
    }
    Ok(subscriptedExp)
}

pub fn simplifyTupleElement(mut tupleExp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut tupleExp: Arc<Expression::NFExpression> = tupleExp;
    let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut index: i32 = 0;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(tupleExp.clone()) {
        Deref @ Expression::TUPLE_ELEMENT { tupleExp: __pa0, index: __pa1, ty: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    e = __pa0.clone();
    index = __pa1.clone();
    ty = __pa2.clone();
    e = simplify(e.clone(), false)?;
    tupleExp = Expression::tupleElement(e.clone(), ty.clone(), index.clone())?;
    Ok(tupleExp)
}

pub fn simplifyRecordElement(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut idx: i32 = 0;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::RECORD_ELEMENT { recordExp: __pa0, index: __pa1, fieldName: _, ty: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    e = __pa0.clone();
    idx = __pa1.clone();
    ty = __pa2.clone();
    e2 = simplify(e.clone(), false)?;
    if !(referenceEq(&e.clone(),&e2.clone())) {
        exp = Expression::nthRecordElement(idx.clone(), e2.clone())?;
    }
    Ok(exp)
}

pub fn combineConstantNumbers(mut r#const: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut inv_const: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut mcl: Operator::MathClassification, mut ty: Arc<Type::NFType>) -> Result<Arc<Expression::NFExpression>> {
    let mut res: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut tmp: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut result: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    res = (match mcl.clone() {
        Operator::MathClassification::ADDITION => {
            result = metamodelica::OrderedFloat(0.0_f64);
            for mut exp in &*r#const.clone() {
                let mut exp = exp.clone();
                tmp = getConstantValue(exp.clone())?;
                result = result.clone() + tmp.clone();
            }
            for mut exp in &*inv_const.clone() {
                let mut exp = exp.clone();
                tmp = getConstantValue(exp.clone())?;
                result = result.clone() - tmp.clone();
            }
            res = if (Type::isInteger(ty.clone())) {Arc::new(Expression::NFExpression::INTEGER { value: ((result.clone()).0 as i32) })} else {Arc::new(Expression::NFExpression::REAL { value: result.clone() })};
            res.clone()
        },
        Operator::MathClassification::MULTIPLICATION => {
            result = metamodelica::OrderedFloat(1.0_f64);
            for mut exp in &*r#const.clone() {
                let mut exp = exp.clone();
                tmp = getConstantValue(exp.clone())?;
                result = result.clone() * tmp.clone();
            }
            if result.clone() == metamodelica::OrderedFloat(0.0_f64) {
                if List::any(inv_const.clone(), (std::sync::Arc::new(fnptr!(Expression::isZero, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>)) {
                    res = Expression::makeNaN(ty.clone());
                } else {
                    res = Expression::makeZero(ty.clone())?;
                }
            } else {
                for mut exp in &*inv_const.clone() {
                    let mut exp = exp.clone();
                    tmp = getConstantValue(exp.clone())?;
                    result = result.clone() / tmp.clone();
                }
                res = if (Type::isInteger(ty.clone())) {Arc::new(Expression::NFExpression::INTEGER { value: ((result.clone()).0 as i32) })} else {Arc::new(Expression::NFExpression::REAL { value: result.clone() })};
            }
            res.clone()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFSimplifyExp.combineConstantNumbers")); __mm_s.push_str(&*literal!(" detected non-commutative operator in MULTARY(): [")); __mm_s.push_str(&*Operator::mathSymbol(mcl.clone())?); __mm_s.push_str(&*literal!("]\n with following arguments: ")); __mm_s.push_str(&*stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (r#const.clone()).into_iter().cloned() {
            let __x = Expression::toString(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n and following inverse arguments: ")); __mm_s.push_str(&*stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (inv_const.clone()).into_iter().cloned() {
            let __x = Expression::toString(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(", ")).clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
    });
    Ok(res)
}

fn getConstantValue(mut exp: Arc<Expression::NFExpression>) -> Result<metamodelica::Real> {
    let mut value: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    match '__try0: {
        value = unwrap_break_err!(Expression::realValue(Ceval::evalExp(exp.clone(), Ceval::noTarget().clone())?), '__try0);
        Ok::<_, anyhow::Error>((value.clone(),))
    } {
        Ok((__try0_o0,)) => {
            value = __try0_o0;
        }
        Err(_) => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFSimplifyExp.getConstantValue")); __mm_s.push_str(&*literal!(" expression is not known to be a constant number: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("try/else: outputs not set in else branch");
        }
    }
    Ok(value)
}

fn cancelTermsInMultary(mut inArguments: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut inInv_arguments: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<(Arc<metamodelica::List<Arc<Expression::NFExpression>>>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>)> {
    fn inc(mut oldValue: Option<i32>, mut step: i32) -> i32 {
        let mut value: i32 = 0;
        value = (match oldValue.clone() {
        Some(mut value) => value.clone() + step.clone(),
        _ => step.clone(),
    });
        value
    }

    let mut outArguments: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut outInv_arguments: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut counter: Arc<UnorderedMap::UnorderedMap<Arc<Expression::NFExpression>, i32>>;
    let mut arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut count: i32 = 0;
    if inArguments.clone().is_empty() || inInv_arguments.clone().is_empty() {
        outArguments = inArguments.clone();
        outInv_arguments = inInv_arguments.clone();
        return Ok((outArguments, outInv_arguments));
    }
    counter = UnorderedMap::new((std::sync::Arc::new(fnptr!(Expression::hash, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<i32> + 'static>), (std::sync::Arc::new(Expression::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<bool> + 'static>), 1);
    for mut arg in &*inArguments.clone() {
        let mut arg = arg.clone();
        UnorderedMap::addUpdate(arg.clone(), Arc::new({ let __pe_b1 = 1; move |__pe_a0| Ok(inc(__pe_a0, __pe_b1.clone())) }), counter.clone())?;
    }
    for mut arg in &*inInv_arguments.clone() {
        let mut arg = arg.clone();
        UnorderedMap::addUpdate(arg.clone(), Arc::new({ let __pe_b1 = -1; move |__pe_a0| Ok(inc(__pe_a0, __pe_b1.clone())) }), counter.clone())?;
    }
    for mut tpl in &*UnorderedMap::toList(counter.clone()) {
        let mut tpl = tpl.clone();
        (arg, count) = tpl.clone();
        if count.clone() > 0 {
            for mut i in 1..=count.clone() {
                outArguments = cons(arg.clone(), outArguments.clone());
            }
        } else if count.clone() < 0 {
            for mut i in 1..=-(count.clone()) {
                outInv_arguments = cons(arg.clone(), outInv_arguments.clone());
            }
        }
    }
    outArguments = outArguments.clone().reverse();
    outInv_arguments = outInv_arguments.clone().reverse();
    Ok((outArguments, outInv_arguments))
}

pub fn combineBinaries(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = Expression::map(exp.clone(), (std::sync::Arc::new(removeTrivialScalarProduct) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    exp = combineBinariesExp(exp.clone(), None, Arc::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(exp.clone()) }), false)?;
    Ok(exp)
}

pub fn splitMultary(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::MULTARY { .. } => {
            let mut new_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            let mut inv_args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            let mut inv_op: Arc<Operator::NFOperator>;
            let mut fixed_op: Arc<Operator::NFOperator>;
            if !(var_field!((*exp).arguments, Expression::NFExpression::MULTARY).clone().is_empty()) {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(var_field!((*exp).arguments, Expression::NFExpression::MULTARY).clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                new_exp = __pa0.clone();
                args = __pa1.clone();
                inv_args = var_field!((*exp).inv_arguments, Expression::NFExpression::MULTARY).clone();
            } else if !(var_field!((*exp).inv_arguments, Expression::NFExpression::MULTARY).clone().is_empty()) {
                if Operator::getMathClassification(var_field!((*exp).operator, Expression::NFExpression::MULTARY).clone())? == Operator::MathClassification::ADDITION.clone() {
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(var_field!((*exp).inv_arguments, Expression::NFExpression::MULTARY).clone()) {
                        Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    new_exp = __pa2.clone();
                    inv_args = __pa3.clone();
                    args = var_field!((*exp).arguments, Expression::NFExpression::MULTARY).clone();
                    new_exp = Expression::negate(new_exp.clone());
                } else {
                    new_exp = Expression::makeOne(Operator::typeOf(var_field!((*exp).operator, Expression::NFExpression::MULTARY).clone()))?;
                    args = var_field!((*exp).arguments, Expression::NFExpression::MULTARY).clone();
                    inv_args = var_field!((*exp).inv_arguments, Expression::NFExpression::MULTARY).clone();
                }
            } else {
                if Operator::getMathClassification(var_field!((*exp).operator, Expression::NFExpression::MULTARY).clone())? == Operator::MathClassification::ADDITION.clone() {
                    new_exp = Expression::makeZero(Operator::typeOf(var_field!((*exp).operator, Expression::NFExpression::MULTARY).clone()))?;
                } else {
                    new_exp = Expression::makeOne(Operator::typeOf(var_field!((*exp).operator, Expression::NFExpression::MULTARY).clone()))?;
                }
                args = var_field!((*exp).arguments, Expression::NFExpression::MULTARY).clone();
                inv_args = var_field!((*exp).inv_arguments, Expression::NFExpression::MULTARY).clone();
            }
            inv_op = Operator::invert(var_field!((*exp).operator, Expression::NFExpression::MULTARY).clone())?;
            for mut arg in &*args.clone() {
                let mut arg = arg.clone();
                fixed_op = Operator::repairBinary(var_field!((*exp).operator, Expression::NFExpression::MULTARY).clone(), Expression::typeOf(new_exp.clone()), Expression::typeOf(arg.clone()))?;
                new_exp = Arc::new(Expression::NFExpression::BINARY { exp1: new_exp.clone(), operator: fixed_op.clone(), exp2: arg.clone() });
            }
            for mut arg in &*inv_args.clone() {
                let mut arg = arg.clone();
                fixed_op = Operator::repairBinary(inv_op.clone(), Expression::typeOf(new_exp.clone()), Expression::typeOf(arg.clone()))?;
                new_exp = Arc::new(Expression::NFExpression::BINARY { exp1: new_exp.clone(), operator: fixed_op.clone(), exp2: arg.clone() });
            }
            new_exp.clone()
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn combineBinariesExp(mut exp: Arc<Expression::NFExpression>, mut optOperator: Option<Arc<Operator::NFOperator>>, mut result: Arc<Expression::NFExpression>, mut inverse: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = result;
    result = ({
        let mut final_stack: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        let mut final_inverse_stack: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        (::match_deref::match_deref! { match &((optOperator.clone(), exp.clone())) {
        (Some(op), Deref @ Expression::BINARY { .. }) if (Operator::compare(op.clone(), var_field!((*exp).operator, Expression::NFExpression::BINARY).clone()) == 0) => {
            result = combineBinariesExp(var_field!((*exp).exp1, Expression::NFExpression::BINARY).clone(), Some(op.clone()), result.clone(), inverse.clone())?;
            result = combineBinariesExp(var_field!((*exp).exp2, Expression::NFExpression::BINARY).clone(), Some(op.clone()), result.clone(), inverse.clone())?;
            result.clone()
        },
        (Some(op), Deref @ Expression::MULTARY { .. }) if (Operator::compare(op.clone(), var_field!((*exp).operator, Expression::NFExpression::MULTARY).clone()) == 0) => {
            for mut arg in &*var_field!((*exp).arguments, Expression::NFExpression::MULTARY).clone() {
                let mut arg = arg.clone();
                result = combineBinariesExp(arg.clone(), Some(var_field!((*exp).operator, Expression::NFExpression::MULTARY).clone()), result.clone(), inverse.clone())?;
            }
            for mut arg in &*var_field!((*exp).inv_arguments, Expression::NFExpression::MULTARY).clone() {
                let mut arg = arg.clone();
                result = combineBinariesExp(arg.clone(), Some(var_field!((*exp).operator, Expression::NFExpression::MULTARY).clone()), result.clone(), !(inverse.clone()))?;
            }
            result.clone()
        },
        (Some(op), Deref @ Expression::BINARY { .. }) if (Operator::isCombineable(op.clone(), var_field!((*exp).operator, Expression::NFExpression::BINARY).clone())?) => {
            result = combineBinariesExp(var_field!((*exp).exp1, Expression::NFExpression::BINARY).clone(), Some(op.clone()), result.clone(), inverse.clone())?;
            result = combineBinariesExp(var_field!((*exp).exp2, Expression::NFExpression::BINARY).clone(), Some(op.clone()), result.clone(), !(inverse.clone()))?;
            result.clone()
        },
        (Some(op), Deref @ Expression::MULTARY { .. }) if (Operator::isCombineable(op.clone(), var_field!((*exp).operator, Expression::NFExpression::MULTARY).clone())?) => {
            for mut arg in &*var_field!((*exp).arguments, Expression::NFExpression::MULTARY).clone() {
                let mut arg = arg.clone();
                result = combineBinariesExp(arg.clone(), Some(var_field!((*exp).operator, Expression::NFExpression::MULTARY).clone()), result.clone(), inverse.clone())?;
            }
            for mut arg in &*var_field!((*exp).inv_arguments, Expression::NFExpression::MULTARY).clone() {
                let mut arg = arg.clone();
                result = combineBinariesExp(arg.clone(), Some(var_field!((*exp).operator, Expression::NFExpression::MULTARY).clone()), result.clone(), !(inverse.clone()))?;
            }
            result.clone()
        },
        (_, Deref @ Expression::BINARY { .. }) if (Operator::isCommutative(var_field!((*exp).operator, Expression::NFExpression::BINARY).clone())) => {
            let mut new_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            new_exp = Arc::new(Expression::NFExpression::MULTARY { arguments: metamodelica::nil(), inv_arguments: metamodelica::nil(), operator: var_field!((*exp).operator, Expression::NFExpression::BINARY).clone() });
            new_exp = combineBinariesExp(var_field!((*exp).exp2, Expression::NFExpression::BINARY).clone(), Some(var_field!((*exp).operator, Expression::NFExpression::BINARY).clone()), new_exp.clone(), false)?;
            new_exp = combineBinariesExp(var_field!((*exp).exp1, Expression::NFExpression::BINARY).clone(), Some(var_field!((*exp).operator, Expression::NFExpression::BINARY).clone()), new_exp.clone(), false)?;
            addArgument(result.clone(), new_exp.clone(), inverse.clone())?
        },
        (_, Deref @ Expression::MULTARY { .. }) if (Operator::isCommutative(var_field!((*exp).operator, Expression::NFExpression::MULTARY).clone())) => {
            let mut new_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            new_exp = Arc::new(Expression::NFExpression::MULTARY { arguments: metamodelica::nil(), inv_arguments: metamodelica::nil(), operator: var_field!((*exp).operator, Expression::NFExpression::MULTARY).clone() });
            for mut arg in &*var_field!((*exp).arguments, Expression::NFExpression::MULTARY).clone() {
                let mut arg = arg.clone();
                new_exp = combineBinariesExp(arg.clone(), Some(var_field!((*exp).operator, Expression::NFExpression::MULTARY).clone()), new_exp.clone(), false)?;
            }
            for mut arg in &*var_field!((*exp).inv_arguments, Expression::NFExpression::MULTARY).clone() {
                let mut arg = arg.clone();
                new_exp = combineBinariesExp(arg.clone(), Some(var_field!((*exp).operator, Expression::NFExpression::MULTARY).clone()), new_exp.clone(), true)?;
            }
            addArgument(result.clone(), new_exp.clone(), inverse.clone())?
        },
        (_, Deref @ Expression::BINARY { .. }) if (Operator::isSoftCommutative(var_field!((*exp).operator, Expression::NFExpression::BINARY).clone())) => {
            let mut op: Arc<Operator::NFOperator>;
            let mut new_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            op = Operator::invert(var_field!((*exp).operator, Expression::NFExpression::BINARY).clone())?;
            new_exp = Arc::new(Expression::NFExpression::MULTARY { arguments: metamodelica::nil(), inv_arguments: metamodelica::nil(), operator: op.clone() });
            new_exp = combineBinariesExp(var_field!((*exp).exp1, Expression::NFExpression::BINARY).clone(), Some(op.clone()), new_exp.clone(), false)?;
            new_exp = combineBinariesExp(var_field!((*exp).exp2, Expression::NFExpression::BINARY).clone(), Some(op.clone()), new_exp.clone(), true)?;
            addArgument(result.clone(), new_exp.clone(), inverse.clone())?
        },
        (_, Deref @ Expression::CREF { cref: cref @ Deref @ ComponentRef::CREF { .. }, .. }) => {
            let mut cref = (*cref).clone();
            assign_variant_field!(cref => ComponentRef::NFComponentRef::CREF; subscripts = {
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut sub in (var_field!((*cref).subscripts, ComponentRef::NFComponentRef::CREF).clone()).into_iter().cloned() {
            let __x = combineBinariesSubscript(sub.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            assign_variant_field!(exp => Expression::NFExpression::CREF; cref = cref.clone());
            addArgument(result.clone(), exp.clone(), inverse.clone())?
        },
        (_, Deref @ Expression::ARRAY { .. }) => {
            if !(var_field!((*exp).literal, Expression::NFExpression::ARRAY).clone()) {
                assign_variant_field!(exp => Expression::NFExpression::ARRAY; elements = Array::map(var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone(), Arc::new({ let __pe_b1 = None; let __pe_b2 = Arc::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(exp.clone()) }); let __pe_b3 = false; move |__pe_a0| combineBinariesExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) })));
            }
            addArgument(result.clone(), exp.clone(), inverse.clone())?
        },
        (_, Deref @ Expression::RANGE { .. }) => {
            assign_variant_field!(exp => Expression::NFExpression::RANGE;
                start = combineBinariesExp(var_field!((*exp).start, Expression::NFExpression::RANGE).clone(), None, Arc::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*exp).start, Expression::NFExpression::RANGE).clone()) }), false)?,
                stop = combineBinariesExp(var_field!((*exp).stop, Expression::NFExpression::RANGE).clone(), None, Arc::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*exp).stop, Expression::NFExpression::RANGE).clone()) }), false)?
            );
            if isSome(var_field!((*exp).step, Expression::NFExpression::RANGE).clone()) {
                assign_variant_field!(exp => Expression::NFExpression::RANGE; step = Some(combineBinariesExp(Util::getOption(var_field!((*exp).step, Expression::NFExpression::RANGE).clone())?, None, Arc::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(Util::getOption(var_field!((*exp).step, Expression::NFExpression::RANGE).clone())?) }), false)?));
            }
            addArgument(result.clone(), exp.clone(), inverse.clone())?
        },
        (_, Deref @ Expression::TUPLE { .. }) => {
            assign_variant_field!(exp => Expression::NFExpression::TUPLE; elements = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut element in (var_field!((*exp).elements, Expression::NFExpression::TUPLE).clone()).into_iter().cloned() {
            let __x = combineBinariesExp(element.clone(), None, Arc::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(element.clone()) }), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            addArgument(result.clone(), exp.clone(), inverse.clone())?
        },
        (_, Deref @ Expression::RECORD { .. }) => {
            assign_variant_field!(exp => Expression::NFExpression::RECORD; elements = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut element in (var_field!((*exp).elements, Expression::NFExpression::RECORD).clone()).into_iter().cloned() {
            let __x = combineBinariesExp(element.clone(), None, Arc::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(element.clone()) }), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            addArgument(result.clone(), exp.clone(), inverse.clone())?
        },
        (_, Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } }) => {
            let mut call = (*call).clone();
            assign_variant_field!(call => Call::NFCall::TYPED_CALL; arguments = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut arg in (var_field!((*call).arguments, Call::NFCall::TYPED_CALL).clone()).into_iter().cloned() {
            let __x = combineBinariesExp(arg.clone(), None, Arc::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(arg.clone()) }), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            assign_variant_field!(exp => Expression::NFExpression::CALL; call = call.clone());
            addArgument(result.clone(), exp.clone(), inverse.clone())?
        },
        (_, Deref @ Expression::SIZE { .. }) => {
            assign_variant_field!(exp => Expression::NFExpression::SIZE; exp = combineBinariesExp(var_field!((*exp).exp, Expression::NFExpression::SIZE).clone(), None, Arc::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*exp).exp, Expression::NFExpression::SIZE).clone()) }), false)?);
            if isSome(var_field!((*exp).dimIndex, Expression::NFExpression::SIZE).clone()) {
                assign_variant_field!(exp => Expression::NFExpression::SIZE; dimIndex = Some(combineBinariesExp(Util::getOption(var_field!((*exp).dimIndex, Expression::NFExpression::SIZE).clone())?, None, Arc::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(Util::getOption(var_field!((*exp).dimIndex, Expression::NFExpression::SIZE).clone())?) }), false)?));
            }
            addArgument(result.clone(), exp.clone(), inverse.clone())?
        },
        (_, Deref @ Expression::UNARY { .. }) => {
            assign_variant_field!(exp => Expression::NFExpression::UNARY; exp = combineBinariesExp(var_field!((*exp).exp, Expression::NFExpression::UNARY).clone(), None, Arc::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*exp).exp, Expression::NFExpression::UNARY).clone()) }), false)?);
            addArgument(result.clone(), exp.clone(), inverse.clone())?
        },
        (_, Deref @ Expression::LBINARY { .. }) => {
            assign_variant_field!(exp => Expression::NFExpression::LBINARY;
                exp1 = combineBinariesExp(var_field!((*exp).exp1, Expression::NFExpression::LBINARY).clone(), None, Arc::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*exp).exp1, Expression::NFExpression::LBINARY).clone()) }), false)?,
                exp2 = combineBinariesExp(var_field!((*exp).exp2, Expression::NFExpression::LBINARY).clone(), None, Arc::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*exp).exp2, Expression::NFExpression::LBINARY).clone()) }), false)?
            );
            addArgument(result.clone(), exp.clone(), inverse.clone())?
        },
        (_, Deref @ Expression::LUNARY { .. }) => {
            assign_variant_field!(exp => Expression::NFExpression::LUNARY; exp = combineBinariesExp(var_field!((*exp).exp, Expression::NFExpression::LUNARY).clone(), None, Arc::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*exp).exp, Expression::NFExpression::LUNARY).clone()) }), false)?);
            addArgument(result.clone(), exp.clone(), inverse.clone())?
        },
        (_, Deref @ Expression::RELATION { .. }) => {
            assign_variant_field!(exp => Expression::NFExpression::RELATION;
                exp1 = combineBinariesExp(var_field!((*exp).exp1, Expression::NFExpression::RELATION).clone(), None, Arc::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*exp).exp1, Expression::NFExpression::RELATION).clone()) }), false)?,
                exp2 = combineBinariesExp(var_field!((*exp).exp2, Expression::NFExpression::RELATION).clone(), None, Arc::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*exp).exp2, Expression::NFExpression::RELATION).clone()) }), false)?
            );
            addArgument(result.clone(), exp.clone(), inverse.clone())?
        },
        (_, Deref @ Expression::IF { .. }) => {
            assign_variant_field!(exp => Expression::NFExpression::IF;
                condition = combineBinariesExp(var_field!((*exp).condition, Expression::NFExpression::IF).clone(), None, Arc::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*exp).condition, Expression::NFExpression::IF).clone()) }), false)?,
                trueBranch = combineBinariesExp(var_field!((*exp).trueBranch, Expression::NFExpression::IF).clone(), None, Arc::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*exp).trueBranch, Expression::NFExpression::IF).clone()) }), false)?,
                falseBranch = combineBinariesExp(var_field!((*exp).falseBranch, Expression::NFExpression::IF).clone(), None, Arc::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*exp).falseBranch, Expression::NFExpression::IF).clone()) }), false)?
            );
            addArgument(result.clone(), exp.clone(), inverse.clone())?
        },
        (_, Deref @ Expression::CAST { .. }) => {
            assign_variant_field!(exp => Expression::NFExpression::CAST; exp = combineBinariesExp(var_field!((*exp).exp, Expression::NFExpression::CAST).clone(), None, Arc::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*exp).exp, Expression::NFExpression::CAST).clone()) }), false)?);
            addArgument(result.clone(), exp.clone(), inverse.clone())?
        },
        (_, Deref @ Expression::BOX { .. }) => {
            assign_variant_field!(exp => Expression::NFExpression::BOX; exp = combineBinariesExp(var_field!((*exp).exp, Expression::NFExpression::BOX).clone(), None, Arc::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*exp).exp, Expression::NFExpression::BOX).clone()) }), false)?);
            addArgument(result.clone(), exp.clone(), inverse.clone())?
        },
        (_, Deref @ Expression::UNBOX { .. }) => {
            assign_variant_field!(exp => Expression::NFExpression::UNBOX; exp = combineBinariesExp(var_field!((*exp).exp, Expression::NFExpression::UNBOX).clone(), None, Arc::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*exp).exp, Expression::NFExpression::UNBOX).clone()) }), false)?);
            addArgument(result.clone(), exp.clone(), inverse.clone())?
        },
        (_, Deref @ Expression::SUBSCRIPTED_EXP { .. }) => {
            assign_variant_field!(exp => Expression::NFExpression::SUBSCRIPTED_EXP;
                exp = combineBinariesExp(var_field!((*exp).exp, Expression::NFExpression::SUBSCRIPTED_EXP).clone(), None, Arc::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*exp).exp, Expression::NFExpression::SUBSCRIPTED_EXP).clone()) }), false)?,
                subscripts = {
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut sub in (var_field!((*exp).subscripts, Expression::NFExpression::SUBSCRIPTED_EXP).clone()).into_iter().cloned() {
            let __x = combineBinariesSubscript(sub.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
            );
            addArgument(result.clone(), exp.clone(), inverse.clone())?
        },
        (_, Deref @ Expression::TUPLE_ELEMENT { .. }) => {
            assign_variant_field!(exp => Expression::NFExpression::TUPLE_ELEMENT; tupleExp = combineBinariesExp(var_field!((*exp).tupleExp, Expression::NFExpression::TUPLE_ELEMENT).clone(), None, Arc::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*exp).tupleExp, Expression::NFExpression::TUPLE_ELEMENT).clone()) }), false)?);
            addArgument(result.clone(), exp.clone(), inverse.clone())?
        },
        (_, Deref @ Expression::RECORD_ELEMENT { .. }) => {
            assign_variant_field!(exp => Expression::NFExpression::RECORD_ELEMENT; recordExp = combineBinariesExp(var_field!((*exp).recordExp, Expression::NFExpression::RECORD_ELEMENT).clone(), None, Arc::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*exp).recordExp, Expression::NFExpression::RECORD_ELEMENT).clone()) }), false)?);
            addArgument(result.clone(), exp.clone(), inverse.clone())?
        },
        (_, Deref @ Expression::MUTABLE { .. }) => {
            Mutable::update(var_field!((*exp).exp, Expression::NFExpression::MUTABLE).clone(), combineBinariesExp(Mutable::access(var_field!((*exp).exp, Expression::NFExpression::MUTABLE).clone()), None, Arc::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(Mutable::access(var_field!((*exp).exp, Expression::NFExpression::MUTABLE).clone())) }), false)?);
            addArgument(result.clone(), exp.clone(), inverse.clone())?
        },
        (_, Deref @ Expression::PARTIAL_FUNCTION_APPLICATION { .. }) => {
            assign_variant_field!(exp => Expression::NFExpression::PARTIAL_FUNCTION_APPLICATION; args = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut arg in (var_field!((*exp).args, Expression::NFExpression::PARTIAL_FUNCTION_APPLICATION).clone()).into_iter().cloned() {
            let __x = combineBinariesExp(arg.clone(), None, Arc::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(arg.clone()) }), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            addArgument(result.clone(), exp.clone(), inverse.clone())?
        },
        _ => {
            addArgument(result.clone(), exp.clone(), inverse.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
    Ok(result)
}

fn combineBinariesSubscript(mut subscript: Arc<Subscript::NFSubscript>) -> Result<Arc<Subscript::NFSubscript>> {
    let mut subscript: Arc<Subscript::NFSubscript> = subscript;
    subscript = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ Subscript::UNTYPED { .. } => {
            assign_variant_field!(subscript => Subscript::NFSubscript::UNTYPED; exp = combineBinariesExp(var_field!((*subscript).exp, Subscript::NFSubscript::UNTYPED).clone(), None, Arc::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*subscript).exp, Subscript::NFSubscript::UNTYPED).clone()) }), false)?);
            subscript.clone()
        },
        Deref @ Subscript::INDEX { .. } => {
            assign_variant_field!(subscript => Subscript::NFSubscript::INDEX; index = combineBinariesExp(var_field!((*subscript).index, Subscript::NFSubscript::INDEX).clone(), None, Arc::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*subscript).index, Subscript::NFSubscript::INDEX).clone()) }), false)?);
            subscript.clone()
        },
        Deref @ Subscript::SLICE { .. } => {
            assign_variant_field!(subscript => Subscript::NFSubscript::SLICE; slice = combineBinariesExp(var_field!((*subscript).slice, Subscript::NFSubscript::SLICE).clone(), None, Arc::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*subscript).slice, Subscript::NFSubscript::SLICE).clone()) }), false)?);
            subscript.clone()
        },
        Deref @ Subscript::EXPANDED_SLICE { .. } => {
            assign_variant_field!(subscript => Subscript::NFSubscript::EXPANDED_SLICE; indices = {
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut sub in (var_field!((*subscript).indices, Subscript::NFSubscript::EXPANDED_SLICE).clone()).into_iter().cloned() {
            let __x = combineBinariesSubscript(sub.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            subscript.clone()
        },
        _ => subscript.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(subscript)
}

fn addArgument(mut exp: Arc<Expression::NFExpression>, mut arg: Arc<Expression::NFExpression>, mut inverse: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::MULTARY { .. } if (inverse.clone()) => {
            assign_variant_field!(exp => Expression::NFExpression::MULTARY; inv_arguments = cons(arg.clone(), var_field!((*exp).inv_arguments, Expression::NFExpression::MULTARY).clone()));
            exp.clone()
        },
        Deref @ Expression::MULTARY { .. } => {
            assign_variant_field!(exp => Expression::NFExpression::MULTARY; arguments = cons(arg.clone(), var_field!((*exp).arguments, Expression::NFExpression::MULTARY).clone()));
            exp.clone()
        },
        Deref @ Expression::EMPTY { .. } => arg.clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFSimplifyExp.addArgument")); __mm_s.push_str(&*literal!(" failed to add : ")); __mm_s.push_str(&*Expression::toString(arg.clone())?); __mm_s.push_str(&*literal!(" to ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(". Only works for MULTARY()!")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn removeTrivialScalarProduct(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut exp1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut exp2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::BINARY { operator: Deref @ Operator::OPERATOR { op: Operator::Op::SCALAR_PRODUCT, ty }, .. } if (Type::sizeOf(Expression::typeOf(var_field!((*exp).exp1, Expression::NFExpression::BINARY).clone()), false)? == 1 && Type::sizeOf(Expression::typeOf(var_field!((*exp).exp2, Expression::NFExpression::BINARY).clone()), false)? == 1) => {
            subs = {
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut d in (Type::arrayDims(Expression::typeOf(var_field!((*exp).exp1, Expression::NFExpression::BINARY).clone()))).into_iter().cloned() {
            let __x = Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: 1 }) });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            exp1 = Expression::applySubscripts(subs.clone(), var_field!((*exp).exp1, Expression::NFExpression::BINARY).clone(), false)?;
            subs = {
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut d in (Type::arrayDims(Expression::typeOf(var_field!((*exp).exp2, Expression::NFExpression::BINARY).clone()))).into_iter().cloned() {
            let __x = Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: 1 }) });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            exp2 = Expression::applySubscripts(subs.clone(), var_field!((*exp).exp2, Expression::NFExpression::BINARY).clone(), false)?;
            Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: Arc::new(Operator::NFOperator { ty: ty.clone(), op: Operator::Op::MUL.clone() }), exp2: exp2.clone() })
        },
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn simplifyURIToFilename(mut arg: Arc<Expression::NFExpression>, mut call: Arc<Call::NFCall>) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    if Flags::getConfigBool(Flags::BUILDING_FMU.clone())? {
        outExp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::FMU_LOAD_RESOURCE().clone(), list![arg.clone()], Call::variability(call.clone())?, Purity::IMPURE.clone(), NFBuiltinFuncs::FMU_LOAD_RESOURCE().returnType.clone()) });
    } else {
        outExp = Arc::new(Expression::NFExpression::CALL { call: call.clone() });
    }
    Ok(outExp)
}

