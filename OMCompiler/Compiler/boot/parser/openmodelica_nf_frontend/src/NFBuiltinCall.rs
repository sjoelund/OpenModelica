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
use crate::NFClass as Class;
use crate::NFClockKind as ClockKind;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFFunction::FunctionMatchKind;
use crate::NFFunction::MatchedFunction;
use crate::NFFunction::NamedArg;
use crate::NFFunction::TypedArg;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::CachedData;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::ConnectorType;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFStructural as Structural;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFTypeCheck as TypeCheck;
use crate::NFTyping as Typing;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Gettext;
use openmodelica_util::Global;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

pub fn needSpecialHandling(mut call: Arc<Call::NFCall>) -> Result<bool> {
    let mut special: bool = false;
    let () = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::UNTYPED_CALL { .. } => {
            let __pa0 = ::match_deref::match_deref! { match &(InstNode::getFuncCache(InstNode::classScope(ComponentRef::node(var_field!((*call).r#ref, Call::NFCall::UNTYPED_CALL).clone())?))?) {
                Deref @ CachedData::FUNCTION { specialBuiltin: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            special = __pa0.clone();
            ()
        },
        Deref @ Call::TYPED_CALL { .. } => {
            special = Function::isSpecialBuiltin(var_field!((*call).r#fn, Call::NFCall::TYPED_CALL).clone())?;
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFBuiltinCall.needSpecialHandling")); __mm_s.push_str(&*literal!(" got unknown call: ")); __mm_s.push_str(&*Call::toString(call.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(special)
}

pub fn typeSpecial(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut variability: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::PURE;
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut next_context: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::UNTYPED_CALL { r#ref: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    cref = __pa0.clone();
    next_context = InstContext::set(context.clone(), InstContext::SUBEXPRESSION.clone());
    (callExp, ty, variability, purity) = (::match_deref::match_deref! { match &(ComponentRef::firstName(cref.clone(), false)?) {
        Deref @ "actualStream" => typeActualInStreamCall((literal!("actualStream")).clone(), call.clone(), next_context.clone(), info.clone())?,
        Deref @ "backSample" => typeBackSampleCall(call.clone(), next_context.clone(), info.clone())?,
        Deref @ "branch" => typeBranchCall(call.clone(), next_context.clone(), info.clone())?,
        Deref @ "cardinality" => typeCardinalityCall(call.clone(), next_context.clone(), info.clone())?,
        Deref @ "cat" => typeCatCall(call.clone(), next_context.clone(), info.clone())?,
        Deref @ "change" => typeChangeCall(call.clone(), next_context.clone(), info.clone())?,
        Deref @ "Clock" => typeClockCall(call.clone(), next_context.clone(), info.clone())?,
        Deref @ "der" => typeDerCall(call.clone(), next_context.clone(), info.clone())?,
        Deref @ "DynamicSelect" => typeDynamicSelectCall((literal!("DynamicSelect")).clone(), call.clone(), next_context.clone(), info.clone())?,
        Deref @ "edge" => typeEdgeCall(call.clone(), next_context.clone(), info.clone())?,
        Deref @ "fill" => typeFillCall(call.clone(), next_context.clone(), info.clone())?,
        Deref @ "getInstanceName" => typeGetInstanceName(call.clone(), next_context.clone(), info.clone())?,
        Deref @ "initial" => typeDiscreteCall(call.clone(), next_context.clone(), info.clone())?,
        Deref @ "inStream" => typeActualInStreamCall((literal!("inStream")).clone(), call.clone(), next_context.clone(), info.clone())?,
        Deref @ "isRoot" => typeIsRootCall(call.clone(), next_context.clone(), info.clone())?,
        Deref @ "matrix" => typeMatrixCall(call.clone(), next_context.clone(), info.clone())?,
        Deref @ "max" => typeMinMaxCall((literal!("max")).clone(), call.clone(), next_context.clone(), info.clone())?,
        Deref @ "min" => typeMinMaxCall((literal!("min")).clone(), call.clone(), next_context.clone(), info.clone())?,
        Deref @ "ndims" => typeNdimsCall(call.clone(), next_context.clone(), info.clone())?,
        Deref @ "noEvent" => typeNoEventCall(call.clone(), next_context.clone(), info.clone())?,
        Deref @ "ones" => typeZerosOnesCall((literal!("ones")).clone(), call.clone(), next_context.clone(), info.clone())?,
        Deref @ "potentialRoot" => typePotentialRootCall(call.clone(), next_context.clone(), info.clone())?,
        Deref @ "pre" => typePreCall(call.clone(), next_context.clone(), info.clone())?,
        Deref @ "promote" => typePromoteCall(call.clone(), next_context.clone(), info.clone())?,
        Deref @ "pure" => typePureCall(call.clone(), next_context.clone(), info.clone())?,
        Deref @ "rooted" => typeRootedCall(call.clone(), next_context.clone(), info.clone())?,
        Deref @ "root" => typeRootCall(call.clone(), next_context.clone(), info.clone())?,
        Deref @ "sample" => typeSampleCall(call.clone(), next_context.clone(), info.clone())?,
        Deref @ "scalar" => typeScalarCall(call.clone(), next_context.clone(), info.clone())?,
        Deref @ "shiftSample" => typeShiftSampleCall(call.clone(), next_context.clone(), info.clone())?,
        Deref @ "smooth" => typeSmoothCall(call.clone(), next_context.clone(), info.clone())?,
        Deref @ "String" => typeStringCall(call.clone(), next_context.clone(), info.clone())?,
        Deref @ "subSample" => typeSubSampleCall(call.clone(), next_context.clone(), info.clone())?,
        Deref @ "superSample" => typeSuperSampleCall(call.clone(), next_context.clone(), info.clone())?,
        Deref @ "symmetric" => typeSymmetricCall(call.clone(), next_context.clone(), info.clone())?,
        Deref @ "terminal" => typeDiscreteCall(call.clone(), next_context.clone(), info.clone())?,
        Deref @ "transpose" => typeTransposeCall(call.clone(), next_context.clone(), info.clone())?,
        Deref @ "uniqueRootIndices" => typeUniqueRootIndicesCall(call.clone(), next_context.clone(), info.clone())?,
        Deref @ "uniqueRoot" => typeUniqueRootCall(call.clone(), next_context.clone(), info.clone())?,
        Deref @ "vector" => typeVectorCall(call.clone(), next_context.clone(), info.clone())?,
        Deref @ "zeros" => typeZerosOnesCall((literal!("zeros")).clone(), call.clone(), next_context.clone(), info.clone())?,
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFBuiltinCall.typeSpecial")); __mm_s.push_str(&*literal!(" got unhandled builtin function: ")); __mm_s.push_str(&*Call::toString(call.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((callExp, ty, variability, purity))
}

pub fn makeSizeExp(mut posArgs: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut namedArgs: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>>, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut arg1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut arg2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    assertNoNamedParams((literal!("size")).clone(), namedArgs.clone(), info.clone())?;
    callExp = (::match_deref::match_deref! { match &(posArgs.clone()) {
        Deref @ metamodelica::List::Cons { head: arg1, tail: Deref @ metamodelica::List::Nil } => Arc::new(Expression::NFExpression::SIZE { exp: arg1.clone(), dimIndex: None }),
        Deref @ metamodelica::List::Cons { head: arg1, tail: Deref @ metamodelica::List::Cons { head: arg2, tail: Deref @ metamodelica::List::Nil } } => Arc::new(Expression::NFExpression::SIZE { exp: arg1.clone(), dimIndex: Some(arg2.clone()) }),
        _ => {
            Error::addSourceMessage(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("size")); __mm_s.push_str(&*List::toString(posArgs.clone(), (std::sync::Arc::new(Expression::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("(")).clone(), (literal!(", ")).clone(), (literal!(")")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone(), (literal!("size(Any[:, ...]) => Integer[:]\n  size(Any[:, ...], Integer) => Integer")).clone()], info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(callExp)
}

pub fn makeArrayExp(mut posArgs: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut namedArgs: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>>, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
    let mut arrayExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    assertNoNamedParams((literal!("array")).clone(), namedArgs.clone(), info.clone())?;
    if posArgs.clone().is_empty() {
        Error::addSourceMessage(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("array")); __mm_s.push_str(&*List::toString(posArgs.clone(), (std::sync::Arc::new(Expression::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("(")).clone(), (literal!(", ")).clone(), (literal!(")")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone(), (literal!("array(Any, Any, ...) => Any[:]")).clone()], info.clone())?;
        bail!("fail");
    }
    arrayExp = Expression::makeArray(Arc::new(crate::NFType::UNKNOWN), metamodelica::arrayFromVec(posArgs.clone().into_iter().cloned().collect()), false);
    Ok(arrayExp)
}

pub fn makeCatExp(mut n: i32, mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut tys: Arc<metamodelica::List<Arc<Type::NFType>>>, mut variability: Variability, mut purity: Purity, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut arg2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut args2: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut res: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut tys2: Arc<metamodelica::List<Arc<Type::NFType>>> = tys.clone();
    let mut tys3: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
    let mut dimsLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Dimension::NFDimension>>>>> = metamodelica::nil();
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut resTy: Arc<Type::NFType> = Arc::new(crate::NFType::UNKNOWN);
    let mut ty1: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ty2: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut resTyToMatch: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut mk: TypeCheck::MatchKind = TypeCheck::MatchKind::EXACT;
    let mut maxn: i32 = 0;
    let mut pos: i32 = 0;
    let mut sumDim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    Error::assertion((args.clone().len() as i32) == (tys.clone().len() as i32) && !(args.clone().is_empty()), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFBuiltinCall.makeCatExp")); __mm_s.push_str(&*literal!(" got wrong input sizes")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
    for mut arg in &*args.clone() {
        let mut arg = arg.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(tys2.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        ty = __pa0.clone();
        tys2 = __pa1.clone();
        dimsLst = cons(Type::arrayDims(ty.clone()), dimsLst.clone());
        if Type::isEqual(resTy.clone(), Arc::new(crate::NFType::UNKNOWN)) {
            resTy = Type::arrayElementType(ty.clone());
        } else {
            (_, _, ty1, mk) = TypeCheck::matchExpressions(Arc::new(Expression::NFExpression::INTEGER { value: 0 }), Type::arrayElementType(ty.clone()), Arc::new(Expression::NFExpression::INTEGER { value: 0 }), resTy.clone(), TypeCheck::DEFAULT_OPTIONS.clone())?;
            if TypeCheck::isCompatibleMatch(mk.clone()) {
                resTy = ty1.clone();
            }
        }
    }
    maxn = ({
        let mut __acc: Option<i32> = None;
        for mut d in (dimsLst.clone()).into_iter().cloned() {
            let __x = (d.clone().len() as i32);
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    });
    if maxn.clone() != ({
        let mut __acc: Option<i32> = None;
        for mut d in (dimsLst.clone()).into_iter().cloned() {
            let __x = (d.clone().len() as i32);
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty min reduction"))?
    }) {
        Error::addSourceMessageAndFail(Error::NF_DIFFERENT_NUM_DIM_IN_ARGUMENTS.clone(), list![stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut d in (dimsLst.clone()).into_iter().cloned() {
            let __x = ArcStr::from(::std::format!("{}", (d.clone().len() as i32)));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!(", ")).clone()), (literal!("cat")).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if n.clone() < 1 || n.clone() > maxn.clone() {
        Error::addSourceMessageAndFail(Error::NF_CAT_WRONG_DIMENSION.clone(), list![ArcStr::from(::std::format!("{}", maxn.clone())), ArcStr::from(::std::format!("{}", n.clone()))], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    tys2 = tys.clone();
    tys3 = metamodelica::nil();
    args2 = metamodelica::nil();
    pos = (args.clone().len() as i32) + 2;
    for mut arg in &*args.clone() {
        let mut arg = arg.clone();
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(tys2.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        ty = __pa2.clone();
        tys2 = __pa3.clone();
        pos = pos.clone() - 1;
        ty2 = Type::setArrayElementType(ty.clone(), resTy.clone());
        (arg2, ty1, mk) = TypeCheck::matchTypes(ty.clone(), ty2.clone(), arg.clone(), TypeCheck::ALLOW_UNKNOWN.clone())?;
        if TypeCheck::isIncompatibleMatch(mk.clone()) {
            Error::addSourceMessageAndFail(Error::ARG_TYPE_MISMATCH.clone(), list![ArcStr::from(::std::format!("{}", pos.clone())), (literal!("cat")).clone(), (literal!("arg")).clone(), (Expression::toString(arg.clone())?).clone(), (Type::toString(ty.clone())?).clone(), (Type::toString(ty2.clone())?).clone()], info.clone())?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
        args2 = cons(arg2.clone(), args2.clone());
        tys3 = cons(ty1.clone(), tys3.clone());
    }
    resTy = Arc::new(crate::NFType::UNKNOWN);
    tys2 = tys3.clone();
    for mut arg in &*args2.clone() {
        let mut arg = arg.clone();
        let (__pa4, __pa5) = ::match_deref::match_deref! { match &(tys2.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
            _ => bail!("pattern mismatch"),
        } };
        ty = __pa4.clone();
        tys2 = __pa5.clone();
        if Type::isEqual(resTy.clone(), Arc::new(crate::NFType::UNKNOWN)) {
            resTy = ty.clone();
        } else {
            (_, _, ty1, mk) = TypeCheck::matchExpressions(Arc::new(Expression::NFExpression::INTEGER { value: 0 }), ty.clone(), Arc::new(Expression::NFExpression::INTEGER { value: 0 }), resTy.clone(), TypeCheck::DEFAULT_OPTIONS.clone())?;
            if TypeCheck::isCompatibleMatch(mk.clone()) {
                resTy = ty1.clone();
            }
        }
    }
    dims = Type::arrayDims(resTy.clone());
    resTyToMatch = Arc::new(Type::NFType::ARRAY { elementType: Type::arrayElementType(resTy.clone()), dimensions: List::set(dims.clone(), n.clone(), Arc::new(crate::NFDimension::UNKNOWN))? });
    dims = ({
        let mut __acc: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
        for mut lst in (dimsLst.clone()).into_iter().cloned() {
            let __x = (lst.clone()).get(n.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    sumDim = Dimension::fromInteger(0, Prefixes::Variability::CONSTANT.clone());
    for mut d in &*dims.clone() {
        let mut d = d.clone();
        sumDim = Dimension::add(sumDim.clone(), d.clone());
    }
    resTy = Arc::new(Type::NFType::ARRAY { elementType: Type::arrayElementType(resTy.clone()), dimensions: List::set(Type::arrayDims(resTy.clone()), n.clone(), sumDim.clone())? });
    tys2 = tys3.clone();
    tys3 = metamodelica::nil();
    res = metamodelica::nil();
    pos = (args.clone().len() as i32) + 2;
    for mut arg in &*args2.clone() {
        let mut arg = arg.clone();
        let (__pa6, __pa7) = ::match_deref::match_deref! { match &(tys2.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa6, tail: __pa7 } => (__pa6.clone(), __pa7.clone()),
            _ => bail!("pattern mismatch"),
        } };
        ty = __pa6.clone();
        tys2 = __pa7.clone();
        pos = pos.clone() - 1;
        (arg2, ty1, mk) = TypeCheck::matchTypes(ty.clone(), resTyToMatch.clone(), arg.clone(), TypeCheck::ALLOW_UNKNOWN.clone())?;
        if TypeCheck::isIncompatibleMatch(mk.clone()) {
            Error::addSourceMessageAndFail(Error::ARG_TYPE_MISMATCH.clone(), list![ArcStr::from(::std::format!("{}", pos.clone())), (literal!("cat")).clone(), (literal!("arg")).clone(), (Expression::toString(arg.clone())?).clone(), (Type::toString(ty.clone())?).clone(), (Type::toString(resTyToMatch.clone())?).clone()], info.clone())?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
        res = cons(arg2.clone(), res.clone());
        tys3 = cons(ty1.clone(), tys3.clone());
    }
    ty = resTy.clone();
    callExp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::CAT().clone(), cons(Arc::new(Expression::NFExpression::INTEGER { value: n.clone() }), res.clone()), variability.clone(), purity.clone(), resTy.clone()) });
    Ok((callExp, ty))
}

fn assertNoNamedParams(mut fnName: ArcStr, mut namedArgs: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>>, mut info: SourceInfo) -> Result<()> {
    if !(namedArgs.clone().is_empty()) {
        Error::addSourceMessage(Error::NO_SUCH_INPUT_PARAMETER.clone(), list![(fnName.clone()).clone(), (Util::tuple21(listHead(namedArgs.clone())?)).clone()], info.clone())?;
        bail!("fail");
    }
    Ok(())
}

fn typeStringCall(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::PURE;
    let mut arg_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut args: Arc<metamodelica::List<Arc<TypedArg>>> = metamodelica::nil();
    let mut named_args: Arc<metamodelica::List<Arc<TypedArg>>> = metamodelica::nil();
    let mut arg: Arc<TypedArg> = Arc::new(<TypedArg as ::std::default::Default>::default());
    let mut ty_call: Arc<Call::NFCall>;
    let (__pa2, __pa0, __pa1) = ::match_deref::match_deref! { match &(Call::typeNormalCall(call.clone(), context.clone(), info.clone())?) {
        __pa2 @ Deref @ Call::ARG_TYPED_CALL { r#ref: _, positional_args: __pa0, named_args: __pa1, .. } => (__pa2.clone(), __pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    args = __pa0.clone();
    named_args = __pa1.clone();
    ty_call = __pa2.clone();
    arg = listHead(args.clone())?;
    arg_ty = Type::arrayElementType(arg.ty.clone());
    if Type::isComplex(arg_ty.clone()) {
        (callExp, outType, var, purity) = typeOverloadedStringCall(arg_ty.clone(), args.clone(), named_args.clone(), ty_call.clone(), context.clone(), info.clone())?;
    } else {
        (callExp, outType, var, purity) = typeBuiltinStringCall(ty_call.clone(), context.clone(), info.clone())?;
    }
    Ok((callExp, outType, var, purity))
}

fn typeBuiltinStringCall(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::PURE;
    let mut ty_call: Arc<Call::NFCall>;
    ty_call = Call::matchTypedNormalCall(call.clone(), context.clone(), info.clone(), true)?;
    ty = Call::typeOf(ty_call.clone());
    var = Call::variability(ty_call.clone())?;
    purity = Call::purity(ty_call.clone());
    callExp = Arc::new(Expression::NFExpression::CALL { call: ty_call.clone() });
    Ok((callExp, ty, var, purity))
}

fn typeOverloadedStringCall(mut overloadedType: Arc<Type::NFType>, mut args: Arc<metamodelica::List<Arc<TypedArg>>>, mut namedArgs: Arc<metamodelica::List<Arc<TypedArg>>>, mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::CONSTANT.clone();
    let mut purity: Purity = Purity::PURE.clone();
    let mut fn_ref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut candidates: Arc<metamodelica::List<Arc<Function::Function>>> = metamodelica::nil();
    let mut recopnode: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut matchedFunc: Arc<MatchedFunction::MatchedFunction> = Arc::new(<MatchedFunction::MatchedFunction as ::std::default::Default>::default());
    let mut matchedFunctions: Arc<metamodelica::List<Arc<MatchedFunction::MatchedFunction>>> = metamodelica::nil();
    let mut exactMatches: Arc<metamodelica::List<Arc<MatchedFunction::MatchedFunction>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(overloadedType.clone()) {
        Deref @ Type::COMPLEX { cls: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    recopnode = __pa0.clone();
    if let Ok(__iflet1) = Function::lookupFunctionSimple((literal!("'String'")).clone(), recopnode.clone(), context.clone()) {
        fn_ref = __iflet1;
    } else {
        typeBuiltinStringCall(call.clone(), context.clone(), info.clone())?;
        bail!("fail");
    }
    (fn_ref, _, _) = Function::instFunctionRef(fn_ref.clone(), context.clone(), InstNode::info(recopnode.clone())?)?;
    candidates = Function::typeRefCache(fn_ref.clone(), InstContext::FUNCTION.clone())?;
    matchedFunctions = Function::matchFunctionsSilent(candidates.clone(), args.clone(), namedArgs.clone(), context.clone(), info.clone(), true)?;
    exactMatches = MatchedFunction::getExactMatches(matchedFunctions.clone());
    if exactMatches.clone().is_empty() {
        Error::addSourceMessage(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone(), list![(Call::typedString(call.clone())?).clone(), (Function::candidateFuncListString(candidates.clone())).clone()], info.clone())?;
        bail!("fail");
    }
    if (exactMatches.clone().len() as i32) == 1 {
        let __pa2 = ::match_deref::match_deref! { match &(exactMatches.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa2, tail: _ } => __pa2.clone(),
            _ => bail!("pattern mismatch"),
        } };
        matchedFunc = __pa2.clone();
        outType = Function::returnType(matchedFunc.func.clone());
        for mut arg in &*matchedFunc.args.clone() {
            let mut arg = arg.clone();
            var = Prefixes::variabilityMax(var.clone(), arg.var.clone());
            purity = Prefixes::purityMin(purity.clone(), arg.purity.clone());
        }
        callExp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(matchedFunc.func.clone(), ({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut a in (matchedFunc.args.clone()).into_iter().cloned() {
            let __x = a.value.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), var.clone(), purity.clone(), outType.clone()) });
        return Ok((callExp.clone(), outType.clone(), var.clone(), purity.clone()));
    } else {
        Error::addSourceMessage(Error::AMBIGUOUS_MATCHING_FUNCTIONS_NFINST.clone(), list![(Call::typedString(call.clone())?).clone(), (Function::candidateFuncListString(({
        let mut __acc: Arc<metamodelica::List<Arc<Function::Function>>> = metamodelica::nil();
        for mut mfn in (matchedFunctions.clone()).into_iter().cloned() {
            let __x = mfn.func.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))).clone()], info.clone())?;
        bail!("fail");
    }
    Ok((callExp, outType, var, purity))
}

fn typeDiscreteCall(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::DISCRETE.clone();
    let mut purity: Purity = Purity::PURE;
    let mut argtycall: Arc<Call::NFCall>;
    argtycall = Call::typeMatchNormalCall(call.clone(), context.clone(), info.clone(), true)?;
    ty = Call::typeOf(argtycall.clone());
    purity = Call::purity(argtycall.clone());
    callExp = Arc::new(Expression::NFExpression::CALL { call: Call::unboxArgs(argtycall.clone()) });
    Ok((callExp, ty, var, purity))
}

fn typeNdimsCall(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(crate::NFType::INTEGER);
    let mut variability: Variability = Variability::PARAMETER.clone();
    let mut purity: Purity = Purity::PURE.clone();
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut named_args: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut arg_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::UNTYPED_CALL { named_args: __pa0, arguments: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    named_args = __pa0.clone();
    args = __pa1.clone();
    assertNoNamedParams((literal!("ndims")).clone(), named_args.clone(), info.clone())?;
    if (args.clone().len() as i32) != 1 {
        Error::addSourceMessage(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone(), list![(Call::toString(call.clone())?).clone(), (literal!("ndims(Any) => Integer")).clone()], info.clone())?;
        bail!("fail");
    }
    (_, arg_ty, _, _) = Typing::typeExp(listHead(args.clone())?, context.clone(), info.clone(), false)?;
    callExp = Arc::new(Expression::NFExpression::INTEGER { value: Type::dimensionCount(arg_ty.clone()) });
    Ok((callExp, ty, variability, purity))
}

fn typePreCall(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut variability: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::PURE;
    (callExp, ty, variability, purity) = typePreChangeCall((literal!("pre")).clone(), call.clone(), context.clone(), info.clone())?;
    Ok((callExp, ty, variability, purity))
}

fn typeChangeCall(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut variability: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::PURE;
    (callExp, ty, variability, purity) = typePreChangeCall((literal!("change")).clone(), call.clone(), context.clone(), info.clone())?;
    ty = Type::setArrayElementType(ty.clone(), Arc::new(crate::NFType::BOOLEAN));
    Ok((callExp, ty, variability, purity))
}

fn typePreChangeCall(mut name: ArcStr, mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut variability: Variability = Variability::DISCRETE.clone();
    let mut purity: Purity = Purity::PURE;
    let mut fn_ref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut named_args: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut var: Variability = Variability::CONSTANT;
    let mut r#fn: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::UNTYPED_CALL { named_args: __pa0, arguments: __pa1, r#ref: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    named_args = __pa0.clone();
    args = __pa1.clone();
    fn_ref = __pa2.clone();
    assertNoNamedParams((name.clone()).clone(), named_args.clone(), info.clone())?;
    if (args.clone().len() as i32) != 1 {
        Error::addSourceMessageAndFail(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone(), list![(Call::toString(call.clone())?).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentRef::toString(fn_ref.clone())?); __mm_s.push_str(&*literal!("(Any) => Any")); ArcStr::from(__mm_s) }).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if InstContext::inFunction(context.clone()) {
        Error::addSourceMessageAndFail(Error::EXP_INVALID_IN_FUNCTION.clone(), list![(ComponentRef::toString(fn_ref.clone())?).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    (arg, ty, var, purity) = Typing::typeExp(listHead(args.clone())?, context.clone(), info.clone(), false)?;
    if !(Expression::isCref(arg.clone())) {
        Error::addSourceMessage(Error::ARGUMENT_MUST_BE_VARIABLE.clone(), list![(literal!("First")).clone(), (ComponentRef::toString(fn_ref.clone())?).clone(), (literal!("<REMOVE ME>")).clone()], info.clone())?;
        bail!("fail");
    }
    if var.clone() == Variability::CONTINUOUS.clone() {
        Error::addSourceMessageAndFail(Error::INVALID_ARGUMENT_VARIABILITY.clone(), list![(literal!("1")).clone(), (ComponentRef::toString(fn_ref.clone())?).clone(), (Prefixes::variabilityString(Variability::DISCRETE.clone())?).clone(), (Expression::toString(arg.clone())?).clone(), (Prefixes::variabilityString(var.clone())?).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    let __pa3 = ::match_deref::match_deref! { match &(Function::typeRefCache(fn_ref.clone(), InstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Nil } => __pa3.clone(),
        _ => bail!("pattern mismatch"),
    } };
    r#fn = __pa3.clone();
    callExp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(r#fn.clone(), list![arg.clone()], var.clone(), purity.clone(), ty.clone()) });
    Ok((callExp, ty, variability, purity))
}

fn typeDerCall(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut variability: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::PURE;
    let mut fn_ref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut named_args: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut r#fn: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    let mut ety: Arc<Type::NFType> = Arc::new(Type::ANY);
    if InstContext::inFunction(context.clone()) {
        Error::addSourceMessage(Error::EXP_INVALID_IN_FUNCTION.clone(), list![(literal!("der")).clone()], info.clone())?;
        bail!("fail");
    }
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::UNTYPED_CALL { named_args: __pa0, arguments: __pa1, r#ref: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    named_args = __pa0.clone();
    args = __pa1.clone();
    fn_ref = __pa2.clone();
    assertNoNamedParams((literal!("der")).clone(), named_args.clone(), info.clone())?;
    if (args.clone().len() as i32) != 1 {
        Error::addSourceMessageAndFail(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone(), list![(Call::toString(call.clone())?).clone(), (literal!("der(Real) => Real")).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    let __pa3 = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Nil } => __pa3.clone(),
        _ => bail!("pattern mismatch"),
    } };
    arg = __pa3.clone();
    (arg, ty, variability, purity) = Typing::typeExp(arg.clone(), context.clone(), info.clone(), false)?;
    ety = Type::arrayElementType(ty.clone());
    if Type::isInteger(ety.clone()) {
        if variability.clone() < Variability::DISCRETE.clone() {
            ty = Type::setArrayElementType(ty.clone(), Arc::new(crate::NFType::REAL));
            arg = Expression::typeCast(arg.clone(), Arc::new(crate::NFType::REAL))?;
        } else {
            Error::addSourceMessageAndFail(Error::DER_OF_NONDIFFERENTIABLE_EXP.clone(), list![(Expression::toString(arg.clone())?).clone()], info.clone())?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
    } else if !(Type::isReal(ety.clone())) {
        Error::addSourceMessageAndFail(Error::ARG_TYPE_MISMATCH.clone(), list![(literal!("1")).clone(), (ComponentRef::toString(fn_ref.clone())?).clone(), (literal!("")).clone(), (Expression::toString(arg.clone())?).clone(), (Type::toString(ty.clone())?).clone(), (literal!("Real")).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if variability.clone() == Variability::DISCRETE.clone() && !(InstContext::inDiscreteScope(context.clone())) {
        Error::addSourceMessageAndFail(Error::DER_OF_NONDIFFERENTIABLE_EXP.clone(), list![(Expression::toString(arg.clone())?).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    let __pa5 = ::match_deref::match_deref! { match &(Function::typeRefCache(fn_ref.clone(), InstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Nil } => __pa5.clone(),
        _ => bail!("pattern mismatch"),
    } };
    r#fn = __pa5.clone();
    callExp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(r#fn.clone(), list![arg.clone()], variability.clone(), purity.clone(), ty.clone()) });
    Ok((callExp, ty, variability, purity))
}

fn typeEdgeCall(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut variability: Variability = Variability::DISCRETE.clone();
    let mut purity: Purity = Purity::PURE;
    let mut argtycall: Arc<Call::NFCall>;
    let mut args: Arc<metamodelica::List<Arc<TypedArg>>> = metamodelica::nil();
    let mut arg: Arc<TypedArg> = Arc::new(<TypedArg as ::std::default::Default>::default());
    let mut fn_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    if InstContext::inFunction(context.clone()) {
        Error::addSourceMessage(Error::EXP_INVALID_IN_FUNCTION.clone(), list![(literal!("edge")).clone()], info.clone())?;
        bail!("fail");
    }
    let (__pa2, __pa0, __pa1) = ::match_deref::match_deref! { match &(Call::typeNormalCall(call.clone(), context.clone(), info.clone())?) {
        __pa2 @ Deref @ Call::ARG_TYPED_CALL { r#ref: Deref @ ComponentRef::CREF { node: __pa0, .. }, positional_args: __pa1, named_args: _, .. } => (__pa2.clone(), __pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    fn_node = __pa0.clone();
    args = __pa1.clone();
    argtycall = __pa2.clone();
    argtycall = Call::matchTypedNormalCall(argtycall.clone(), context.clone(), info.clone(), true)?;
    ty = Call::typeOf(argtycall.clone());
    purity = Call::purity(argtycall.clone());
    callExp = Arc::new(Expression::NFExpression::CALL { call: Call::unboxArgs(argtycall.clone()) });
    let __pa4 = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Nil } => __pa4.clone(),
        _ => bail!("pattern mismatch"),
    } };
    arg = __pa4.clone();
    if !(Expression::isCref(arg.value.clone())) {
        Error::addSourceMessage(Error::ARGUMENT_MUST_BE_VARIABLE.clone(), list![(literal!("First")).clone(), (literal!("edge")).clone(), (literal!("<REMOVE ME>")).clone()], info.clone())?;
        bail!("fail");
    }
    Ok((callExp, ty, variability, purity))
}

fn typeMinMaxCall(mut name: ArcStr, mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    fn is_valid_type(mut ty: Arc<Type::NFType>) -> bool {
        let mut res: bool = false;
        res = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::REAL => true,
        Deref @ Type::INTEGER => true,
        Deref @ Type::BOOLEAN => true,
        Deref @ Type::ENUMERATION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        res
    }

    fn invalid_args_error(mut call: Arc<Call::NFCall>, mut name: ArcStr, mut info: SourceInfo) -> Result<()> {
        Error::addSourceMessageAndFail(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone(), list![(Call::toString(call.clone())?).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("(Real, Real) => Real\n  ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("(Integer, Integer) => Integer\n  ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("(Boolean, Boolean) => Boolean\n  ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("(enumeration(:), enumeration(:)) => enumeration(:)\n  ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("(Real[:, ...]) => Real\n  ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("(Integer[:, ...]) => Integer\n  ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("(Boolean[:, ...]) => Boolean\n  ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("(enumeration(:)[:, ...]) => enumeration(:)")); ArcStr::from(__mm_s) }).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        Ok(())
    }

    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::PURE;
    let mut fn_ref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut named_args: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut r#fn: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    let mut arg1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut arg2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty1: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ty2: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var1: Variability = Variability::CONSTANT;
    let mut var2: Variability = Variability::CONSTANT;
    let mut pur1: Purity = Purity::PURE;
    let mut pur2: Purity = Purity::PURE;
    let mut mk: TypeCheck::MatchKind = TypeCheck::MatchKind::EXACT;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::UNTYPED_CALL { named_args: __pa0, arguments: __pa1, r#ref: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    named_args = __pa0.clone();
    args = __pa1.clone();
    fn_ref = __pa2.clone();
    assertNoNamedParams((name.clone()).clone(), named_args.clone(), info.clone())?;
    (args, ty, var, purity) = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: arg1, tail: Deref @ metamodelica::List::Nil } => {
            let mut arg1 = (*arg1).clone();
            (arg1, ty1, var, purity) = Typing::typeExp(arg1.clone(), context.clone(), info.clone(), false)?;
            ty = Type::arrayElementType(ty1.clone());
            if !(Type::isArray(ty1.clone()) && is_valid_type(ty.clone())) {
                invalid_args_error(call.clone(), (name.clone()).clone(), info.clone())?;
            }
            if Type::isSingleElementArray(ty1.clone())? {
                callExp = Expression::applySubscript(Subscript::first(listHead(Type::arrayDims(ty1.clone()))?)?, arg1.clone(), metamodelica::nil(), false)?;
                return Ok((callExp.clone(), ty.clone(), var.clone(), purity.clone()));
            }
            (list![arg1.clone()], ty.clone(), var.clone(), purity.clone())
        },
        Deref @ metamodelica::List::Cons { head: arg1, tail: Deref @ metamodelica::List::Cons { head: arg2, tail: Deref @ metamodelica::List::Nil } } => {
            let mut arg1 = (*arg1).clone();
            let mut arg2 = (*arg2).clone();
            (arg1, ty1, var1, pur1) = Typing::typeExp(arg1.clone(), context.clone(), info.clone(), false)?;
            (arg2, ty2, var2, pur2) = Typing::typeExp(arg2.clone(), context.clone(), info.clone(), false)?;
            if !(is_valid_type(ty1.clone()) && is_valid_type(ty2.clone())) {
                invalid_args_error(call.clone(), (name.clone()).clone(), info.clone())?;
            }
            (arg1, arg2, ty, mk) = TypeCheck::matchExpressions(arg1.clone(), ty1.clone(), arg2.clone(), ty2.clone(), TypeCheck::DEFAULT_OPTIONS.clone())?;
            if !(TypeCheck::isValidArgumentMatch(mk.clone())) {
                invalid_args_error(call.clone(), (name.clone()).clone(), info.clone())?;
            }
            (list![arg1.clone(), arg2.clone()], ty.clone(), Prefixes::variabilityMax(var1.clone(), var2.clone()), Prefixes::purityMin(pur1.clone(), pur2.clone()))
        },
        _ => {
            invalid_args_error(call.clone(), (name.clone()).clone(), info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    r#fn = listHead(Function::typeRefCache(fn_ref.clone(), InstContext::FUNCTION.clone())?)?;
    callExp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(r#fn.clone(), args.clone(), var.clone(), purity.clone(), ty.clone()) });
    Ok((callExp, ty, var, purity))
}

fn typePromoteCall(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut variability: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::PURE;
    let mut fn_ref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut named_args: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut exp_arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut n_arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut exp_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut n_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut n_var: Variability = Variability::CONSTANT;
    let mut n: i32 = 0;
    if !(Config::languageStandardAtLeast(Config::LanguageStandard::experimental.clone())?) {
        Error::addSourceMessageAndFail(Error::EXPERIMENTAL_REQUIRED.clone(), list![(literal!("promote")).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::UNTYPED_CALL { named_args: __pa0, arguments: __pa1, r#ref: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    named_args = __pa0.clone();
    args = __pa1.clone();
    fn_ref = __pa2.clone();
    assertNoNamedParams((literal!("promote")).clone(), named_args.clone(), info.clone())?;
    if (args.clone().len() as i32) != 2 {
        Error::addSourceMessageAndFail(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone(), list![(Call::toString(call.clone())?).clone(), (literal!("promote(Any[...], Integer) => Any[...]")).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Nil } } => (__pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    exp_arg = __pa3.clone();
    n_arg = __pa4.clone();
    (exp_arg, exp_ty, variability, purity) = Typing::typeExp(exp_arg.clone(), context.clone(), info.clone(), false)?;
    (n_arg, n_ty, n_var, _) = Typing::typeExp(n_arg.clone(), context.clone(), info.clone(), false)?;
    if !(Type::isInteger(n_ty.clone())) {
        Error::addSourceMessageAndFail(Error::ARG_TYPE_MISMATCH.clone(), list![(literal!("2")).clone(), (literal!("promote")).clone(), (literal!("")).clone(), (Expression::toString(n_arg.clone())?).clone(), (Type::toString(n_ty.clone())?).clone(), (literal!("Integer")).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if n_var.clone() > Variability::CONSTANT.clone() {
        Error::addSourceMessageAndFail(Error::INVALID_ARGUMENT_VARIABILITY.clone(), list![(literal!("2")).clone(), (literal!("promote")).clone(), (Prefixes::variabilityString(Variability::CONSTANT.clone())?).clone(), (Expression::toString(n_arg.clone())?).clone(), (Prefixes::variabilityString(n_var.clone())?).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    n_arg = Ceval::evalExp(n_arg.clone(), Ceval::EvalTarget::new(info.clone(), context.clone(), None))?;
    n = Expression::integerValue(n_arg.clone())?;
    if n.clone() < Type::dimensionCount(exp_ty.clone()) {
        Error::addSourceMessageAndFail(Error::INVALID_NUMBER_OF_DIMENSIONS_FOR_PROMOTE.clone(), list![ArcStr::from(::std::format!("{}", n.clone())), ArcStr::from(::std::format!("{}", Type::dimensionCount(exp_ty.clone())))], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    (callExp, ty) = Expression::promote(exp_arg.clone(), Expression::typeOf(exp_arg.clone()), Expression::integerValue(n_arg.clone())?)?;
    Ok((callExp, ty, variability, purity))
}

fn typeSmoothCall(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut variability: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::PURE;
    let mut fn_ref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut named_args: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut arg1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut arg2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty1: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ty2: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::CONSTANT;
    let mut r#fn: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    let mut mk: TypeCheck::MatchKind = TypeCheck::MatchKind::EXACT;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::UNTYPED_CALL { named_args: __pa0, arguments: __pa1, r#ref: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    named_args = __pa0.clone();
    args = __pa1.clone();
    fn_ref = __pa2.clone();
    assertNoNamedParams((literal!("smooth")).clone(), named_args.clone(), info.clone())?;
    if (args.clone().len() as i32) != 2 {
        Error::addSourceMessageAndFail(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone(), list![(Call::toString(call.clone())?).clone(), (literal!("smooth(Integer, Any) => Any")).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Nil } } => (__pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    arg1 = __pa3.clone();
    arg2 = __pa4.clone();
    (arg1, ty1, var, _) = Typing::typeExp(arg1.clone(), context.clone(), info.clone(), false)?;
    (arg2, ty2, variability, purity) = Typing::typeExp(arg2.clone(), context.clone(), info.clone(), false)?;
    if !(Type::isInteger(ty1.clone())) {
        Error::addSourceMessageAndFail(Error::ARG_TYPE_MISMATCH.clone(), list![(literal!("1")).clone(), (ComponentRef::toString(fn_ref.clone())?).clone(), (literal!("")).clone(), (Expression::toString(arg1.clone())?).clone(), (Type::toString(ty1.clone())?).clone(), (literal!("Integer")).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if var.clone() > Variability::PARAMETER.clone() {
        Error::addSourceMessageAndFail(Error::INVALID_ARGUMENT_VARIABILITY.clone(), list![(literal!("1")).clone(), (ComponentRef::toString(fn_ref.clone())?).clone(), (Prefixes::variabilityString(Variability::PARAMETER.clone())?).clone(), (Expression::toString(arg1.clone())?).clone(), (Prefixes::variabilityString(variability.clone())?).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    (arg2, ty, mk) = TypeCheck::matchTypes(ty2.clone(), Type::setArrayElementType(ty2.clone(), Arc::new(crate::NFType::REAL)), arg2.clone(), TypeCheck::ALLOW_UNKNOWN.clone())?;
    if !(TypeCheck::isValidArgumentMatch(mk.clone())) {
        Error::addSourceMessageAndFail(Error::ARG_TYPE_MISMATCH.clone(), list![(literal!("2")).clone(), (ComponentRef::toString(fn_ref.clone())?).clone(), (literal!("")).clone(), (Expression::toString(arg2.clone())?).clone(), (Type::toString(ty2.clone())?).clone(), (literal!("Real\n  Real[:, ...]\n  Real record\n  Real record[:, ...]")).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    let __pa6 = ::match_deref::match_deref! { match &(Function::typeRefCache(fn_ref.clone(), InstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Nil } => __pa6.clone(),
        _ => bail!("pattern mismatch"),
    } };
    r#fn = __pa6.clone();
    callExp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(r#fn.clone(), list![arg1.clone(), arg2.clone()], var.clone(), purity.clone(), ty.clone()) });
    Ok((callExp, ty, variability, purity))
}

fn typeFillCall(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut variability: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::PURE;
    let mut fn_ref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut named_args: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut fill_arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::UNTYPED_CALL { named_args: __pa0, arguments: __pa1, r#ref: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    named_args = __pa0.clone();
    args = __pa1.clone();
    fn_ref = __pa2.clone();
    assertNoNamedParams((literal!("fill")).clone(), named_args.clone(), info.clone())?;
    if (args.clone().len() as i32) < 2 {
        Error::addSourceMessageAndFail(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone(), list![(Call::toString(call.clone())?).clone(), (literal!("fill(Any, Integer, ...) => Any[:, ...]")).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    fill_arg = __pa3.clone();
    args = __pa4.clone();
    (fill_arg, ty, variability, purity) = Typing::typeExp(fill_arg.clone(), context.clone(), info.clone(), false)?;
    (callExp, ty, variability, purity) = typeFillCall2(fn_ref.clone(), ty.clone(), fill_arg.clone(), variability.clone(), purity.clone(), args.clone(), context.clone(), info.clone())?;
    Ok((callExp, ty, variability, purity))
}

fn typeFillCall2(mut fnRef: Arc<ComponentRef::NFComponentRef>, mut fillType: Arc<Type::NFType>, mut fillArg: Arc<Expression::NFExpression>, mut fillVariability: Variability, mut fillPurity: Purity, mut dimensionArgs: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut variability: Variability = fillVariability.clone();
    let mut purity: Purity = fillPurity.clone();
    let mut ty_args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut arg_var: Variability = Variability::CONSTANT;
    let mut arg_pur: Purity = Purity::PURE;
    let mut arg_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut r#fn: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut index: i32 = 1;
    ty_args = list![fillArg.clone()];
    dims = metamodelica::nil();
    for mut arg in &*dimensionArgs.clone() {
        let mut arg = arg.clone();
        (arg, arg_ty, arg_var, arg_pur) = Typing::typeExp(arg.clone(), context.clone(), info.clone(), false)?;
        if !(InstContext::inAlgorithm(context.clone()) || InstContext::inFunction(context.clone())) {
            if arg_var.clone() > Variability::PARAMETER.clone() && !(InstContext::inInstanceAPI(context.clone()) || Expression::contains(arg.clone(), (std::sync::Arc::new(Expression::isResizableCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>))?) {
                Error::addSourceMessageAndFail(Error::NON_PARAMETER_EXPRESSION_DIMENSION.clone(), list![(Expression::toString(arg.clone())?).clone(), ArcStr::from(::std::format!("{}", index.clone())), (List::toString(cons(fillArg.clone(), dimensionArgs.clone()), (std::sync::Arc::new(Expression::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<ArcStr> + 'static>), (ComponentRef::toString(fnRef.clone())?).clone(), (literal!("(")).clone(), (literal!(", ")).clone(), (literal!(")")).clone(), true, 0)?).clone()], info.clone())?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            if arg_pur.clone() == Purity::PURE.clone() && !(Structural::isExpressionNotFixed(arg.clone(), false, 4)?) {
                Structural::markExp(arg.clone())?;
                arg = if (InstContext::inInstanceAPI(context.clone())) {Ceval::tryEvalExp(arg.clone(), Ceval::noTarget().clone())} else {Ceval::tryEvalExpResizable(arg.clone(), Ceval::noTarget().clone())?};
                arg_ty = Expression::typeOf(arg.clone());
            }
        }
        if !(Type::isInteger(arg_ty.clone())) {
            Error::addSourceMessageAndFail(Error::ARG_TYPE_MISMATCH.clone(), list![(intString((ty_args.clone().len() as i32) + 1)).clone(), (ComponentRef::toString(fnRef.clone())?).clone(), (literal!("")).clone(), (Expression::toString(arg.clone())?).clone(), (Type::toString(arg_ty.clone())?).clone(), (literal!("Integer")).clone()], info.clone())?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
        variability = Prefixes::variabilityMax(variability.clone(), arg_var.clone());
        purity = Prefixes::purityMin(purity.clone(), arg_pur.clone());
        ty_args = cons(arg.clone(), ty_args.clone());
        dims = cons(Dimension::fromExp(arg.clone(), arg_var.clone())?, dims.clone());
        index = index.clone() + 1;
    }
    ty_args = ty_args.clone().reverse();
    dims = dims.clone().reverse();
    let __pa0 = ::match_deref::match_deref! { match &(Function::typeRefCache(fnRef.clone(), InstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    r#fn = __pa0.clone();
    ty = Type::liftArrayLeftList(fillType.clone(), dims.clone());
    callExp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::FILL_FUNC().clone(), ty_args.clone(), variability.clone(), purity.clone(), ty.clone()) });
    Ok((callExp, ty, variability, purity))
}

fn typeZerosOnesCall(mut name: ArcStr, mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut variability: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::PURE;
    let mut fn_ref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut named_args: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut fill_arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::UNTYPED_CALL { named_args: __pa0, arguments: __pa1, r#ref: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    named_args = __pa0.clone();
    args = __pa1.clone();
    fn_ref = __pa2.clone();
    assertNoNamedParams((name.clone()).clone(), named_args.clone(), info.clone())?;
    if args.clone().is_empty() {
        Error::addSourceMessageAndFail(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone(), list![(Call::toString(call.clone())?).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentRef::toString(fn_ref.clone())?); __mm_s.push_str(&*literal!("(Integer, ...) => Integer[:, ...]")); ArcStr::from(__mm_s) }).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    fill_arg = Arc::new(Expression::NFExpression::INTEGER { value: if (name.clone() == literal!("ones")) {1} else {0} });
    (callExp, ty, variability, purity) = typeFillCall2(fn_ref.clone(), Arc::new(crate::NFType::INTEGER), fill_arg.clone(), Variability::CONSTANT.clone(), Purity::PURE.clone(), args.clone(), context.clone(), info.clone())?;
    Ok((callExp, ty, variability, purity))
}

fn typeScalarCall(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut variability: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::PURE;
    let mut fn_ref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut named_args: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut r#fn: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    let mut expanded: bool = false;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::UNTYPED_CALL { named_args: __pa0, arguments: __pa1, r#ref: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    named_args = __pa0.clone();
    args = __pa1.clone();
    fn_ref = __pa2.clone();
    assertNoNamedParams((literal!("scalar")).clone(), named_args.clone(), info.clone())?;
    if (args.clone().len() as i32) != 1 {
        Error::addSourceMessageAndFail(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone(), list![(Call::toString(call.clone())?).clone(), (literal!("scalar(Any[1, ...]) => Any")).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    (arg, ty, variability, purity) = Typing::typeExp(listHead(args.clone())?, context.clone(), info.clone(), false)?;
    for mut dim in &*Type::arrayDims(ty.clone()) {
        let mut dim = dim.clone();
        if Dimension::isKnown(dim.clone(), false) && !(Dimension::size(dim.clone(), false)? == 1) {
            Error::addSourceMessageAndFail(Error::INVALID_ARRAY_DIM_IN_SCALAR_OP.clone(), list![(Type::toString(ty.clone())?).clone()], info.clone())?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
    }
    (arg, expanded) = ExpandExp::expand(arg.clone(), false, false)?;
    ty = Type::arrayElementType(ty.clone());
    if expanded.clone() {
        args = Expression::arrayScalarElements(arg.clone());
        if (args.clone().len() as i32) != 1 {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFBuiltinCall.typeScalarCall")); __mm_s.push_str(&*literal!(" failed to expand scalar(")); __mm_s.push_str(&*Expression::toString(arg.clone())?); __mm_s.push_str(&*literal!(") correctly")); ArcStr::from(__mm_s) }).clone(), info.clone())?;
        }
        callExp = listHead(args.clone())?;
    } else {
        let __pa3 = ::match_deref::match_deref! { match &(Function::typeRefCache(fn_ref.clone(), InstContext::FUNCTION.clone())?) {
            Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Nil } => __pa3.clone(),
            _ => bail!("pattern mismatch"),
        } };
        r#fn = __pa3.clone();
        callExp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(r#fn.clone(), list![arg.clone()], variability.clone(), purity.clone(), ty.clone()) });
    }
    Ok((callExp, ty, variability, purity))
}

fn typeVectorCall(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut variability: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::PURE;
    let mut fn_ref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut named_args: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut r#fn: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    let mut vector_dim: Arc<Dimension::NFDimension> = Dimension::fromInteger(1, Prefixes::Variability::CONSTANT.clone());
    let mut dim_found: bool = false;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::UNTYPED_CALL { named_args: __pa0, arguments: __pa1, r#ref: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    named_args = __pa0.clone();
    args = __pa1.clone();
    fn_ref = __pa2.clone();
    assertNoNamedParams((literal!("vector")).clone(), named_args.clone(), info.clone())?;
    if (args.clone().len() as i32) != 1 {
        Error::addSourceMessageAndFail(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone(), list![(Call::toString(call.clone())?).clone(), (literal!("vector(Any) => Any[:]\n  vector(Any[:, ...]) => Any[:]")).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    (arg, ty, variability, purity) = Typing::typeExp(listHead(args.clone())?, context.clone(), info.clone(), false)?;
    for mut dim in &*Type::arrayDims(ty.clone()) {
        let mut dim = dim.clone();
        if !(Dimension::isKnown(dim.clone(), false)) || Dimension::size(dim.clone(), false)? > 1 {
            if dim_found.clone() {
                Error::addSourceMessageAndFail(Error::NF_VECTOR_INVALID_DIMENSIONS.clone(), list![(Type::toString(ty.clone())?).clone(), (Call::toString(call.clone())?).clone()], info.clone())?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            } else {
                vector_dim = dim.clone();
                dim_found = true;
            }
        }
    }
    if Type::isEmptyArray(ty.clone()) {
        vector_dim = Dimension::fromInteger(0, Prefixes::Variability::CONSTANT.clone());
    }
    ty = Arc::new(Type::NFType::ARRAY { elementType: Type::arrayElementType(ty.clone()), dimensions: list![vector_dim.clone()] });
    let __pa3 = ::match_deref::match_deref! { match &(Function::typeRefCache(fn_ref.clone(), InstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Nil } => __pa3.clone(),
        _ => bail!("pattern mismatch"),
    } };
    r#fn = __pa3.clone();
    callExp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(r#fn.clone(), list![arg.clone()], variability.clone(), purity.clone(), ty.clone()) });
    Ok((callExp, ty, variability, purity))
}

fn typeMatrixCall(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut variability: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::PURE;
    let mut fn_ref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut named_args: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut r#fn: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut dim1: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut dim2: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut i: i32 = 0;
    let mut ndims: i32 = 0;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::UNTYPED_CALL { named_args: __pa0, arguments: __pa1, r#ref: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    named_args = __pa0.clone();
    args = __pa1.clone();
    fn_ref = __pa2.clone();
    assertNoNamedParams((literal!("matrix")).clone(), named_args.clone(), info.clone())?;
    if (args.clone().len() as i32) != 1 {
        Error::addSourceMessageAndFail(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone(), list![(Call::toString(call.clone())?).clone(), (literal!("matrix(Any) => Any[:]\n  matrix(Any[:, ...]) => Any[:]")).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    (arg, ty, variability, purity) = Typing::typeExp(listHead(args.clone())?, context.clone(), info.clone(), false)?;
    dims = Type::arrayDims(ty.clone());
    ndims = (dims.clone().len() as i32);
    if ndims.clone() < 2 {
        (callExp, ty) = Expression::promote(arg.clone(), ty.clone(), 2)?;
    } else if ndims.clone() == 2 {
        callExp = arg.clone();
    } else {
        let (__pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(dims.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: __pa5 } } => (__pa3.clone(), __pa4.clone(), __pa5.clone()),
            _ => bail!("pattern mismatch"),
        } };
        dim1 = __pa3.clone();
        dim2 = __pa4.clone();
        dims = __pa5.clone();
        i = 3;
        for mut dim in &*dims.clone() {
            let mut dim = dim.clone();
            if Dimension::isKnown(dim.clone(), false) && Dimension::size(dim.clone(), false)? > 1 {
                Error::addSourceMessageAndFail(Error::INVALID_ARRAY_DIM_IN_CONVERSION_OP.clone(), list![ArcStr::from(::std::format!("{}", i.clone())), (literal!("matrix")).clone(), (literal!("1")).clone(), (Dimension::toString(dim.clone())?).clone()], info.clone())?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            i = i.clone() + 1;
        }
        ty = Arc::new(Type::NFType::ARRAY { elementType: Type::arrayElementType(ty.clone()), dimensions: list![dim1.clone(), dim2.clone()] });
        let __pa7 = ::match_deref::match_deref! { match &(Function::typeRefCache(fn_ref.clone(), InstContext::FUNCTION.clone())?) {
            Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Nil } => __pa7.clone(),
            _ => bail!("pattern mismatch"),
        } };
        r#fn = __pa7.clone();
        callExp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(r#fn.clone(), list![arg.clone()], variability.clone(), purity.clone(), ty.clone()) });
    }
    Ok((callExp, ty, variability, purity))
}

fn typeCatCall(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut variability: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::PURE;
    let mut fn_ref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut res: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut named_args: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut tys: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
    let mut arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut var: Variability = Variability::CONSTANT;
    let mut pur: Purity = Purity::PURE;
    let mut mk: TypeCheck::MatchKind = TypeCheck::MatchKind::EXACT;
    let mut n: i32 = 0;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::UNTYPED_CALL { named_args: __pa0, arguments: __pa1, r#ref: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    named_args = __pa0.clone();
    args = __pa1.clone();
    fn_ref = __pa2.clone();
    assertNoNamedParams((literal!("cat")).clone(), named_args.clone(), info.clone())?;
    if (args.clone().len() as i32) < 2 {
        Error::addSourceMessageAndFail(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone(), list![(Call::toString(call.clone())?).clone(), (literal!("cat(Integer, Any[:,:], ...) => Any[:]")).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    arg = __pa3.clone();
    args = __pa4.clone();
    (arg, ty, variability, purity) = Typing::typeExp(arg.clone(), context.clone(), info.clone(), false)?;
    (arg, ty, mk) = TypeCheck::matchTypes(ty.clone(), Arc::new(crate::NFType::INTEGER), arg.clone(), TypeCheck::DEFAULT_OPTIONS.clone())?;
    if variability.clone() > Variability::PARAMETER.clone() || purity.clone() != Purity::PURE.clone() {
        Error::addSourceMessageAndFail(Error::NF_CAT_FIRST_ARG_EVAL.clone(), list![(Expression::toString(arg.clone())?).clone(), (Prefixes::variabilityString(variability.clone())?).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    let __pa5 = ::match_deref::match_deref! { match &(Ceval::evalExp(arg.clone(), Ceval::EvalTarget::new(info.clone(), context.clone(), None))?) {
        Deref @ Expression::INTEGER { value: __pa5 } => __pa5.clone(),
        _ => bail!("pattern mismatch"),
    } };
    n = __pa5.clone();
    res = metamodelica::nil();
    tys = metamodelica::nil();
    for mut a in &*args.clone() {
        let mut a = a.clone();
        (arg, ty, var, pur) = Typing::typeExp(a.clone(), context.clone(), info.clone(), false)?;
        variability = Prefixes::variabilityMax(var.clone(), variability.clone());
        purity = Prefixes::purityMin(pur.clone(), purity.clone());
        res = cons(arg.clone(), res.clone());
        tys = cons(ty.clone(), tys.clone());
    }
    (callExp, ty) = makeCatExp(n.clone(), res.clone().reverse(), tys.clone().reverse(), variability.clone(), purity.clone(), info.clone())?;
    Ok((callExp, ty, variability, purity))
}

fn typeSymmetricCall(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut variability: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::PURE;
    let mut fn_ref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut named_args: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut r#fn: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::UNTYPED_CALL { named_args: __pa0, arguments: __pa1, r#ref: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    named_args = __pa0.clone();
    args = __pa1.clone();
    fn_ref = __pa2.clone();
    assertNoNamedParams((literal!("symmetric")).clone(), named_args.clone(), info.clone())?;
    if (args.clone().len() as i32) != 1 {
        Error::addSourceMessageAndFail(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone(), list![(Call::toString(call.clone())?).clone(), (literal!("symmetric(Any[n, n]) => Any[n, n]")).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    (arg, ty, variability, purity) = Typing::typeExp(listHead(args.clone())?, context.clone(), info.clone(), false)?;
    if !(Type::isSquareMatrix(ty.clone())?) {
        Error::addSourceMessageAndFail(Error::ARG_TYPE_MISMATCH.clone(), list![(literal!("1")).clone(), (ComponentRef::toString(fn_ref.clone())?).clone(), (literal!("")).clone(), (Expression::toString(arg.clone())?).clone(), (Type::toString(ty.clone())?).clone(), (literal!("Any[n, n]")).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    let __pa3 = ::match_deref::match_deref! { match &(Function::typeRefCache(fn_ref.clone(), InstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Nil } => __pa3.clone(),
        _ => bail!("pattern mismatch"),
    } };
    r#fn = __pa3.clone();
    callExp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(r#fn.clone(), list![arg.clone()], variability.clone(), purity.clone(), ty.clone()) });
    Ok((callExp, ty, variability, purity))
}

fn typeTransposeCall(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut variability: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::PURE;
    let mut fn_ref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut named_args: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut dim1: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut dim2: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut rest_dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut r#fn: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::UNTYPED_CALL { named_args: __pa0, arguments: __pa1, r#ref: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    named_args = __pa0.clone();
    args = __pa1.clone();
    fn_ref = __pa2.clone();
    assertNoNamedParams((literal!("transpose")).clone(), named_args.clone(), info.clone())?;
    if (args.clone().len() as i32) != 1 {
        Error::addSourceMessageAndFail(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone(), list![(Call::toString(call.clone())?).clone(), (literal!("transpose(Any[n, m, ...]) => Any[m, n, ...]")).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    (arg, ty, variability, purity) = Typing::typeExp(listHead(args.clone())?, context.clone(), info.clone(), false)?;
    ty = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::ARRAY { dimensions: Deref @ metamodelica::List::Cons { head: dim1, tail: Deref @ metamodelica::List::Cons { head: dim2, tail: rest_dims } }, .. } => Arc::new(Type::NFType::ARRAY { elementType: var_field!((*ty).elementType, Type::NFType::ARRAY).clone(), dimensions: cons(dim2.clone(), cons(dim1.clone(), rest_dims.clone())) }),
        _ => {
            Error::addSourceMessage(Error::ARG_TYPE_MISMATCH.clone(), list![(literal!("1")).clone(), (ComponentRef::toString(fn_ref.clone())?).clone(), (literal!("")).clone(), (Expression::toString(arg.clone())?).clone(), (Type::toString(ty.clone())?).clone(), (literal!("Any[:, :, ...]")).clone()], info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    let __pa3 = ::match_deref::match_deref! { match &(Function::typeRefCache(fn_ref.clone(), InstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Nil } => __pa3.clone(),
        _ => bail!("pattern mismatch"),
    } };
    r#fn = __pa3.clone();
    callExp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(r#fn.clone(), list![arg.clone()], variability.clone(), purity.clone(), ty.clone()) });
    Ok((callExp, ty, variability, purity))
}

fn typeCardinalityCall(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::PARAMETER.clone();
    let mut purity: Purity = Purity::IMPURE.clone();
    if !(InstContext::inCondition(context.clone()) && (InstContext::inIf(context.clone()) || InstContext::inAssert(context.clone()))) {
        Error::addSourceMessageAndFail(Error::INVALID_CARDINALITY_CONTEXT.clone(), metamodelica::nil(), info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if InstContext::inFunction(context.clone()) {
        Error::addSourceMessageAndFail(Error::EXP_INVALID_IN_FUNCTION.clone(), list![(AbsynUtil::pathString(Call::functionName(call.clone())?, (literal!(".")).clone(), true, false)?).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    (callExp, ty, _, _) = typeBuiltinCallExp(call.clone(), context.clone(), info.clone(), false)?;
    System::setUsesCardinality(true);
    Ok((callExp, ty, var, purity))
}

fn typeConnectionsArgs(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut context: i32, mut info: SourceInfo, mut fnRef: Arc<ComponentRef::NFComponentRef>) -> Result<Arc<metamodelica::List<Arc<Expression::NFExpression>>>> {
    let mut outArgs: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut index: i32 = 1;
    for mut arg in &*args.clone() {
        let mut arg = arg.clone();
        outArgs = cons((typeConnectionsArg(arg.clone(), context.clone(), info.clone(), fnRef.clone(), index.clone())?).0, outArgs.clone());
        index = index.clone() + 1;
    }
    outArgs = outArgs.clone().reverse();
    Ok(outArgs)
}

fn typeConnectionsArg(mut arg: Arc<Expression::NFExpression>, mut context: i32, mut info: SourceInfo, mut fnRef: Arc<ComponentRef::NFComponentRef>, mut index: i32) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>)> {
    let mut outArg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outType: Arc<Type::NFType> = Arc::new(Type::ANY);
    (outArg, outType, _, _) = Typing::typeExp(arg.clone(), context.clone(), info.clone(), false)?;
    checkConnectionsArgument(outArg.clone(), outType.clone(), fnRef.clone(), index.clone(), info.clone())?;
    Ok((outArg, outType))
}

fn typeBranchCall(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::PARAMETER.clone();
    let mut purity: Purity = Purity::IMPURE.clone();
    let mut fn_ref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut named_args: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut r#fn: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::UNTYPED_CALL { named_args: __pa0, arguments: __pa1, r#ref: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    named_args = __pa0.clone();
    args = __pa1.clone();
    fn_ref = __pa2.clone();
    assertNoNamedParams((literal!("Connections.branch")).clone(), named_args.clone(), info.clone())?;
    if (args.clone().len() as i32) != 2 {
        Error::addSourceMessageAndFail(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone(), list![(Call::toString(call.clone())?).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentRef::toString(fn_ref.clone())?); __mm_s.push_str(&*literal!("(Connector, Connector)")); ArcStr::from(__mm_s) }).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if InstContext::inFunction(context.clone()) {
        Error::addSourceMessageAndFail(Error::EXP_INVALID_IN_FUNCTION.clone(), list![(ComponentRef::toString(fn_ref.clone())?).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    args = typeConnectionsArgs(args.clone(), context.clone(), info.clone(), fn_ref.clone())?;
    let __pa3 = ::match_deref::match_deref! { match &(Function::typeRefCache(fn_ref.clone(), InstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Nil } => __pa3.clone(),
        _ => bail!("pattern mismatch"),
    } };
    r#fn = __pa3.clone();
    ty = Arc::new(crate::NFType::NORETCALL);
    callExp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(r#fn.clone(), args.clone(), var.clone(), purity.clone(), ty.clone()) });
    Ok((callExp, ty, var, purity))
}

fn typeIsRootCall(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::PARAMETER.clone();
    let mut purity: Purity = Purity::IMPURE.clone();
    let mut fn_ref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut named_args: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut r#fn: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::UNTYPED_CALL { named_args: __pa0, arguments: __pa1, r#ref: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    named_args = __pa0.clone();
    args = __pa1.clone();
    fn_ref = __pa2.clone();
    assertNoNamedParams((literal!("Connections.isRoot")).clone(), named_args.clone(), info.clone())?;
    if (args.clone().len() as i32) != 1 {
        Error::addSourceMessageAndFail(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone(), list![(Call::toString(call.clone())?).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentRef::toString(fn_ref.clone())?); __mm_s.push_str(&*literal!("(Connector)")); ArcStr::from(__mm_s) }).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if InstContext::inFunction(context.clone()) {
        Error::addSourceMessageAndFail(Error::EXP_INVALID_IN_FUNCTION.clone(), list![(ComponentRef::toString(fn_ref.clone())?).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    args = typeConnectionsArgs(args.clone(), context.clone(), info.clone(), fn_ref.clone())?;
    let __pa3 = ::match_deref::match_deref! { match &(Function::typeRefCache(fn_ref.clone(), InstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Nil } => __pa3.clone(),
        _ => bail!("pattern mismatch"),
    } };
    r#fn = __pa3.clone();
    ty = Arc::new(crate::NFType::BOOLEAN);
    callExp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(r#fn.clone(), args.clone(), var.clone(), purity.clone(), ty.clone()) });
    Ok((callExp, ty, var, purity))
}

fn typePotentialRootCall(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::PARAMETER.clone();
    let mut purity: Purity = Purity::IMPURE.clone();
    let mut fn_ref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut named_args: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut arg1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut arg2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut r#fn: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    let mut args_len: i32 = 0;
    let mut name: ArcStr = arcstr::literal!("");
    let mut arg_var: Variability = Variability::CONSTANT;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::UNTYPED_CALL { named_args: __pa0, arguments: __pa1, r#ref: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    named_args = __pa0.clone();
    args = __pa1.clone();
    fn_ref = __pa2.clone();
    for mut narg in &*named_args.clone() {
        let mut narg = narg.clone();
        (name, arg2) = narg.clone();
        if name.clone() == literal!("priority") {
            args = List::appendElt(arg2.clone(), args.clone());
        } else {
            Error::addSourceMessageAndFail(Error::NO_SUCH_INPUT_PARAMETER.clone(), list![(ComponentRef::toString(fn_ref.clone())?).clone(), (name.clone()).clone()], info.clone())?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
    }
    args_len = (args.clone().len() as i32);
    if args_len.clone() < 1 || args_len.clone() > 2 {
        Error::addSourceMessageAndFail(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone(), list![(Call::toString(call.clone())?).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentRef::toString(fn_ref.clone())?); __mm_s.push_str(&*literal!("(Connector, Integer = 0)")); ArcStr::from(__mm_s) }).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if InstContext::inFunction(context.clone()) {
        Error::addSourceMessageAndFail(Error::EXP_INVALID_IN_FUNCTION.clone(), list![(ComponentRef::toString(fn_ref.clone())?).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    arg1 = __pa3.clone();
    args = __pa4.clone();
    (arg1, _) = typeConnectionsArg(arg1.clone(), context.clone(), info.clone(), fn_ref.clone(), 1)?;
    if args_len.clone() == 2 {
        arg2 = listHead(args.clone())?;
        (arg2, ty, arg_var, _) = Typing::typeExp(arg2.clone(), context.clone(), info.clone(), false)?;
        if !(Type::isInteger(ty.clone())) {
            Error::addSourceMessageAndFail(Error::ARG_TYPE_MISMATCH.clone(), list![(literal!("2")).clone(), (ComponentRef::toString(fn_ref.clone())?).clone(), (literal!("")).clone(), (Expression::toString(arg2.clone())?).clone(), (Type::toString(ty.clone())?).clone(), (literal!("Integer")).clone()], info.clone())?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
        if arg_var.clone() > Variability::PARAMETER.clone() {
            Error::addSourceMessageAndFail(Error::INVALID_ARGUMENT_VARIABILITY.clone(), list![(literal!("2")).clone(), (ComponentRef::toString(fn_ref.clone())?).clone(), (Prefixes::variabilityString(Variability::PARAMETER.clone())?).clone(), (Expression::toString(arg2.clone())?).clone(), (Prefixes::variabilityString(arg_var.clone())?).clone()], info.clone())?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
        Structural::markExp(arg2.clone())?;
    } else {
        arg2 = Arc::new(Expression::NFExpression::INTEGER { value: 0 });
    }
    let __pa5 = ::match_deref::match_deref! { match &(Function::typeRefCache(fn_ref.clone(), InstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Nil } => __pa5.clone(),
        _ => bail!("pattern mismatch"),
    } };
    r#fn = __pa5.clone();
    ty = Arc::new(crate::NFType::NORETCALL);
    callExp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(r#fn.clone(), list![arg1.clone(), arg2.clone()], var.clone(), purity.clone(), ty.clone()) });
    Ok((callExp, ty, var, purity))
}

fn typeRootCall(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::PARAMETER.clone();
    let mut purity: Purity = Purity::IMPURE.clone();
    let mut fn_ref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut named_args: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut r#fn: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::UNTYPED_CALL { named_args: __pa0, arguments: __pa1, r#ref: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    named_args = __pa0.clone();
    args = __pa1.clone();
    fn_ref = __pa2.clone();
    assertNoNamedParams((literal!("Connections.root")).clone(), named_args.clone(), info.clone())?;
    if (args.clone().len() as i32) != 1 {
        Error::addSourceMessageAndFail(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone(), list![(Call::toString(call.clone())?).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentRef::toString(fn_ref.clone())?); __mm_s.push_str(&*literal!("(Connector)")); ArcStr::from(__mm_s) }).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if InstContext::inFunction(context.clone()) {
        Error::addSourceMessageAndFail(Error::EXP_INVALID_IN_FUNCTION.clone(), list![(ComponentRef::toString(fn_ref.clone())?).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    args = typeConnectionsArgs(args.clone(), context.clone(), info.clone(), fn_ref.clone())?;
    let __pa3 = ::match_deref::match_deref! { match &(Function::typeRefCache(fn_ref.clone(), InstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Nil } => __pa3.clone(),
        _ => bail!("pattern mismatch"),
    } };
    r#fn = __pa3.clone();
    ty = Arc::new(crate::NFType::NORETCALL);
    callExp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(r#fn.clone(), args.clone(), var.clone(), purity.clone(), ty.clone()) });
    Ok((callExp, ty, var, purity))
}

fn typeRootedCall(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::PARAMETER.clone();
    let mut purity: Purity = Purity::IMPURE.clone();
    let mut fn_ref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut named_args: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut r#fn: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::UNTYPED_CALL { named_args: __pa0, arguments: __pa1, r#ref: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    named_args = __pa0.clone();
    args = __pa1.clone();
    fn_ref = __pa2.clone();
    assertNoNamedParams((literal!("Connections.rooted")).clone(), named_args.clone(), info.clone())?;
    if (args.clone().len() as i32) != 1 {
        Error::addSourceMessageAndFail(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone(), list![(Call::toString(call.clone())?).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentRef::toString(fn_ref.clone())?); __mm_s.push_str(&*literal!("(Connector)")); ArcStr::from(__mm_s) }).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if InstContext::inFunction(context.clone()) {
        Error::addSourceMessageAndFail(Error::EXP_INVALID_IN_FUNCTION.clone(), list![(ComponentRef::toString(fn_ref.clone())?).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    args = typeConnectionsArgs(args.clone(), context.clone(), info.clone(), fn_ref.clone())?;
    if ComponentRef::isSimple(fn_ref.clone()) {
        Error::addSourceMessage(Error::DEPRECATED_API_CALL.clone(), list![(literal!("rooted")).clone(), (literal!("Connections.rooted")).clone()], info.clone())?;
    }
    let __pa3 = ::match_deref::match_deref! { match &(Function::typeRefCache(fn_ref.clone(), InstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Nil } => __pa3.clone(),
        _ => bail!("pattern mismatch"),
    } };
    r#fn = __pa3.clone();
    ty = Arc::new(crate::NFType::BOOLEAN);
    callExp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(r#fn.clone(), args.clone(), var.clone(), purity.clone(), ty.clone()) });
    Ok((callExp, ty, var, purity))
}

fn typeUniqueRootCall(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::PARAMETER.clone();
    let mut purity: Purity = Purity::IMPURE.clone();
    let mut fn_ref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut named_args: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut arg1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut arg2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut r#fn: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    let mut args_len: i32 = 0;
    let mut name: ArcStr = arcstr::literal!("");
    Error::addSourceMessage(Error::NON_STANDARD_OPERATOR.clone(), list![(literal!("Connections.uniqueRoot")).clone()], info.clone())?;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::UNTYPED_CALL { named_args: __pa0, arguments: __pa1, r#ref: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    named_args = __pa0.clone();
    args = __pa1.clone();
    fn_ref = __pa2.clone();
    for mut narg in &*named_args.clone() {
        let mut narg = narg.clone();
        (name, arg2) = narg.clone();
        if name.clone() == literal!("message") {
            args = List::appendElt(arg2.clone(), args.clone());
        } else {
            Error::addSourceMessageAndFail(Error::NO_SUCH_INPUT_PARAMETER.clone(), list![(ComponentRef::toString(fn_ref.clone())?).clone(), (name.clone()).clone()], info.clone())?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
    }
    args_len = (args.clone().len() as i32);
    if args_len.clone() < 1 || args_len.clone() > 2 {
        Error::addSourceMessageAndFail(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone(), list![(Call::toString(call.clone())?).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentRef::toString(fn_ref.clone())?); __mm_s.push_str(&*literal!("(Connector, String = \"\")")); ArcStr::from(__mm_s) }).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if InstContext::inFunction(context.clone()) {
        Error::addSourceMessageAndFail(Error::EXP_INVALID_IN_FUNCTION.clone(), list![(ComponentRef::toString(fn_ref.clone())?).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    arg1 = __pa3.clone();
    args = __pa4.clone();
    (arg1, _) = typeConnectionsArg(arg1.clone(), context.clone(), info.clone(), fn_ref.clone(), 1)?;
    if args_len.clone() == 2 {
        arg2 = listHead(args.clone())?;
        (arg2, ty, _, _) = Typing::typeExp(arg2.clone(), context.clone(), info.clone(), false)?;
        if !(Type::isString(ty.clone())) {
            Error::addSourceMessageAndFail(Error::ARG_TYPE_MISMATCH.clone(), list![(literal!("2")).clone(), (ComponentRef::toString(fn_ref.clone())?).clone(), (literal!("")).clone(), (Expression::toString(arg2.clone())?).clone(), (Type::toString(ty.clone())?).clone(), (literal!("String")).clone()], info.clone())?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
    } else {
        arg2 = Arc::new(Expression::NFExpression::STRING { value: (literal!("")).clone() });
    }
    let __pa5 = ::match_deref::match_deref! { match &(Function::typeRefCache(fn_ref.clone(), InstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Nil } => __pa5.clone(),
        _ => bail!("pattern mismatch"),
    } };
    r#fn = __pa5.clone();
    ty = Arc::new(crate::NFType::NORETCALL);
    callExp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(r#fn.clone(), list![arg1.clone(), arg2.clone()], var.clone(), purity.clone(), ty.clone()) });
    Ok((callExp, ty, var, purity))
}

fn typeUniqueRootIndicesCall(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::PARAMETER.clone();
    let mut purity: Purity = Purity::IMPURE.clone();
    let mut fn_ref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut named_args: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut arg1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut arg2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut arg3: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut r#fn: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    let mut args_len: i32 = 0;
    let mut name: ArcStr = arcstr::literal!("");
    let mut ty1: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ty2: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ty3: Arc<Type::NFType> = Arc::new(Type::ANY);
    Error::addSourceMessage(Error::NON_STANDARD_OPERATOR.clone(), list![(literal!("Connections.uniqueRootIndices")).clone()], info.clone())?;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::UNTYPED_CALL { named_args: __pa0, arguments: __pa1, r#ref: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    named_args = __pa0.clone();
    args = __pa1.clone();
    fn_ref = __pa2.clone();
    for mut narg in &*named_args.clone() {
        let mut narg = narg.clone();
        (name, arg3) = narg.clone();
        if name.clone() == literal!("message") {
            args = List::appendElt(arg3.clone(), args.clone());
        } else {
            Error::addSourceMessageAndFail(Error::NO_SUCH_INPUT_PARAMETER.clone(), list![(ComponentRef::toString(fn_ref.clone())?).clone(), (name.clone()).clone()], info.clone())?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
    }
    args_len = (args.clone().len() as i32);
    if args_len.clone() < 2 || args_len.clone() > 3 {
        Error::addSourceMessageAndFail(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone(), list![(Call::toString(call.clone())?).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentRef::toString(fn_ref.clone())?); __mm_s.push_str(&*literal!("(Connector, Connector, String = \"\")")); ArcStr::from(__mm_s) }).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if InstContext::inFunction(context.clone()) {
        Error::addSourceMessageAndFail(Error::EXP_INVALID_IN_FUNCTION.clone(), list![(ComponentRef::toString(fn_ref.clone())?).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    let (__pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: __pa5 } } => (__pa3.clone(), __pa4.clone(), __pa5.clone()),
        _ => bail!("pattern mismatch"),
    } };
    arg1 = __pa3.clone();
    arg2 = __pa4.clone();
    args = __pa5.clone();
    (arg1, ty1) = typeConnectionsArg(arg1.clone(), context.clone(), info.clone(), fn_ref.clone(), 1)?;
    if !(Type::isArray(ty1.clone())) {
        Error::addSourceMessageAndFail(Error::ARG_TYPE_MISMATCH.clone(), list![(literal!("1")).clone(), (ComponentRef::toString(fn_ref.clone())?).clone(), (literal!("")).clone(), (Expression::toString(arg1.clone())?).clone(), (Type::toString(ty1.clone())?).clone(), (literal!("Connector[:]")).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    (arg2, ty2) = typeConnectionsArg(arg2.clone(), context.clone(), info.clone(), fn_ref.clone(), 2)?;
    if !(Type::isArray(ty2.clone())) {
        Error::addSourceMessageAndFail(Error::ARG_TYPE_MISMATCH.clone(), list![(literal!("2")).clone(), (ComponentRef::toString(fn_ref.clone())?).clone(), (literal!("")).clone(), (Expression::toString(arg2.clone())?).clone(), (Type::toString(ty2.clone())?).clone(), (literal!("Connector[:]")).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if args_len.clone() == 3 {
        arg3 = listHead(args.clone())?;
        (arg3, ty3, _, _) = Typing::typeExp(arg3.clone(), context.clone(), info.clone(), false)?;
        if !(Type::isString(ty3.clone())) {
            Error::addSourceMessageAndFail(Error::ARG_TYPE_MISMATCH.clone(), list![(literal!("3")).clone(), (ComponentRef::toString(fn_ref.clone())?).clone(), (literal!("")).clone(), (Expression::toString(arg2.clone())?).clone(), (Type::toString(ty3.clone())?).clone(), (literal!("String")).clone()], info.clone())?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
    } else {
        arg3 = Arc::new(Expression::NFExpression::STRING { value: (literal!("")).clone() });
    }
    let __pa7 = ::match_deref::match_deref! { match &(Function::typeRefCache(fn_ref.clone(), InstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Nil } => __pa7.clone(),
        _ => bail!("pattern mismatch"),
    } };
    r#fn = __pa7.clone();
    assert!((Type::arrayDims(ty1.clone()).len() as i32) == (Type::arrayDims(ty2.clone()).len() as i32), "{}", &*(literal!("the first two parameters need to have the same size")).clone());
    ty = Arc::new(Type::NFType::ARRAY { elementType: Arc::new(crate::NFType::INTEGER), dimensions: Type::arrayDims(ty1.clone()) });
    callExp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(r#fn.clone(), list![arg1.clone(), arg2.clone(), arg3.clone()], var.clone(), purity.clone(), ty.clone()) });
    Ok((callExp, ty, var, purity))
}

fn checkConnectionsArgument(mut arg: Arc<Expression::NFExpression>, mut ty: Arc<Type::NFType>, mut fnRef: Arc<ComponentRef::NFComponentRef>, mut argIndex: i32, mut info: SourceInfo) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::CREF { .. } => {
            let mut ty2: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
            let mut valid_cref: bool = false;
            let mut isConnector: bool = false;
            (valid_cref, isConnector) = (::match_deref::match_deref! { match &(var_field!((*arg).cref, Expression::NFExpression::CREF).clone()) {
        Deref @ ComponentRef::CREF { restCref: Deref @ ComponentRef::CREF { origin: ComponentRef::Origin::CREF, ty: ty2, .. }, origin: ComponentRef::Origin::CREF, node, .. } => {
            let mut ty2 = (*ty2).clone();
            ty2 = (::match_deref::match_deref! { match &(ty2.clone()) {
        Deref @ Type::ARRAY { .. } if ((ComponentRef::subscriptsAllFlat(var_field!((*arg).cref, Expression::NFExpression::CREF).clone()).len() as i32) == (var_field!((*ty2).dimensions, Type::NFType::ARRAY).clone().len() as i32)) => var_field!((*ty2).elementType, Type::NFType::ARRAY).clone(),
        _ => ty2.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            (Class::isOverdetermined(InstNode::getClass(node.clone())?), Type::isConnector(ty2.clone()))
        },
        Deref @ ComponentRef::CREF { ty: ty2, node, .. } => {
            let mut ty2 = (*ty2).clone();
            ty2 = (::match_deref::match_deref! { match &(ty2.clone()) {
        Deref @ Type::ARRAY { .. } if ((ComponentRef::subscriptsAllFlat(var_field!((*arg).cref, Expression::NFExpression::CREF).clone()).len() as i32) == (var_field!((*ty2).dimensions, Type::NFType::ARRAY).clone().len() as i32)) => var_field!((*ty2).elementType, Type::NFType::ARRAY).clone(),
        _ => ty2.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            (Class::isOverdetermined(InstNode::getClass(node.clone())?), Type::isConnector(ty2.clone()))
        },
        _ => (false, false),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if !(valid_cref.clone() && isConnector.clone()) {
                if valid_cref.clone() {
                    Error::addSourceMessage(if (argIndex.clone() == 1) {Error::W_INVALID_ARGUMENT_TYPE_BRANCH_FIRST.clone()} else {Error::W_INVALID_ARGUMENT_TYPE_BRANCH_SECOND.clone()}, list![(ComponentRef::toString(var_field!((*arg).cref, Expression::NFExpression::CREF).clone())?).clone(), (ComponentRef::toString(fnRef.clone())?).clone()], info.clone())?;
                } else {
                    Error::addSourceMessageAndFail(if (argIndex.clone() == 1) {Error::INVALID_ARGUMENT_TYPE_BRANCH_FIRST.clone()} else {Error::INVALID_ARGUMENT_TYPE_BRANCH_SECOND.clone()}, list![(ComponentRef::toString(var_field!((*arg).cref, Expression::NFExpression::CREF).clone())?).clone(), (ComponentRef::toString(fnRef.clone())?).clone()], info.clone())?;
                    unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
                }
            }
            ()
        },
        _ => {
            Error::addSourceMessage(Error::ARG_TYPE_MISMATCH.clone(), list![ArcStr::from(::std::format!("{}", argIndex.clone())), (ComponentRef::toString(fnRef.clone())?).clone(), (literal!("")).clone(), (Expression::toString(arg.clone())?).clone(), (Type::toString(ty.clone())?).clone(), (literal!("overconstrained type/record")).clone()], info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn typeNoEventCall(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut variability: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::PURE;
    let mut fn_ref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut named_args: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut r#fn: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::UNTYPED_CALL { named_args: __pa0, arguments: __pa1, r#ref: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    named_args = __pa0.clone();
    args = __pa1.clone();
    fn_ref = __pa2.clone();
    assertNoNamedParams((literal!("noEvent")).clone(), named_args.clone(), info.clone())?;
    if (args.clone().len() as i32) != 1 {
        Error::addSourceMessageAndFail(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone(), list![(Call::toString(call.clone())?).clone(), (literal!("noEvent(Any) => Any")).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    let __pa3 = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Nil } => __pa3.clone(),
        _ => bail!("pattern mismatch"),
    } };
    arg = __pa3.clone();
    (arg, ty, variability, purity) = Typing::typeExp(arg.clone(), InstContext::set(context.clone(), InstContext::NOEVENT.clone()), info.clone(), false)?;
    let __pa5 = ::match_deref::match_deref! { match &(Function::typeRefCache(fn_ref.clone(), InstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Nil } => __pa5.clone(),
        _ => bail!("pattern mismatch"),
    } };
    r#fn = __pa5.clone();
    callExp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(r#fn.clone(), list![arg.clone()], variability.clone(), purity.clone(), ty.clone()) });
    Ok((callExp, ty, variability, purity))
}

fn typeGetInstanceName(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(crate::NFType::STRING);
    let mut var: Variability = Variability::CONSTANT.clone();
    let mut purity: Purity = Purity::PURE.clone();
    let mut scope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let __pa0 = ::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::UNTYPED_CALL { call_scope: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    scope = __pa0.clone();
    Call::typeMatchNormalCall(call.clone(), context.clone(), info.clone(), true)?;
    result = Arc::new(Expression::NFExpression::INSTANCE_NAME { scope: scope.clone() });
    Ok((result, ty, var, purity))
}

fn typeClockCall(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outType: Arc<Type::NFType> = Arc::new(crate::NFType::CLOCK);
    let mut var: Variability = Variability::PARAMETER.clone();
    let mut purity: Purity = Purity::IMPURE.clone();
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut args_count: i32 = 0;
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let __pa0 = ::match_deref::match_deref! { match &(Call::typeMatchNormalCall(call.clone(), context.clone(), info.clone(), false)?) {
        Deref @ Call::TYPED_CALL { arguments: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    args = __pa0.clone();
    args_count = (args.clone().len() as i32);
    callExp = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Nil => Arc::new(Expression::NFExpression::CLKCONST { clk: Arc::new(ClockKind::NFClockKind::INFERRED_CLOCK { idx: System::tmpTickIndex(Global::inferredClock_index.clone()) }) }),
        Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil } => Arc::new(Expression::NFExpression::CLKCONST { clk: Arc::new(ClockKind::NFClockKind::REAL_CLOCK { interval: e1.clone() }) }),
        Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } } => {
            let mut e2 = (*e2).clone();
            e2 = Ceval::evalExp(e2.clone(), Ceval::noTarget().clone())?;
            callExp = (::match_deref::match_deref! { match &(Expression::typeOf(e2.clone())) {
        Deref @ Type::INTEGER => {
            Error::assertionOrAddSourceMessage(Expression::integerValue(e2.clone())? >= 1, Error::WRONG_VALUE_OF_ARG.clone(), list![(literal!("Clock")).clone(), (literal!("resolution")).clone(), (Expression::toString(e2.clone())?).clone(), (literal!("=> 1")).clone()], info.clone())?;
            Arc::new(Expression::NFExpression::CLKCONST { clk: Arc::new(ClockKind::NFClockKind::RATIONAL_CLOCK { intervalCounter: e1.clone(), resolution: e2.clone() }) })
        },
        Deref @ Type::REAL => Arc::new(Expression::NFExpression::CLKCONST { clk: Arc::new(ClockKind::NFClockKind::EVENT_CLOCK { condition: e1.clone(), startInterval: e2.clone() }) }),
        Deref @ Type::STRING => Arc::new(Expression::NFExpression::CLKCONST { clk: Arc::new(ClockKind::NFClockKind::SOLVER_CLOCK { c: e1.clone(), solverMethod: e2.clone() }) }),
        _ => bail!("match: no arm matched"),
    } });
            callExp.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((callExp, outType, var, purity))
}

fn typeSampleCall(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::IMPURE.clone();
    let mut ty_call: Arc<Call::NFCall>;
    let mut args: Arc<metamodelica::List<Arc<TypedArg>>> = metamodelica::nil();
    let mut namedArgs: Arc<metamodelica::List<Arc<TypedArg>>> = metamodelica::nil();
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut t1: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut v1: Variability = Variability::CONSTANT;
    let mut fn_ref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut normalSample: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    let mut clockedSample: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    let mut recopnode: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Call::typeNormalCall(call.clone(), context.clone(), info.clone())?) {
        Deref @ Call::ARG_TYPED_CALL { r#ref: __pa0, positional_args: __pa1, named_args: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    fn_ref = __pa0.clone();
    args = __pa1.clone();
    namedArgs = __pa2.clone();
    recopnode = ComponentRef::node(fn_ref.clone())?;
    (fn_ref, _, _) = Function::instFunctionRef(fn_ref.clone(), context.clone(), InstNode::info(recopnode.clone())?)?;
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(Function::typeRefCache(fn_ref.clone(), InstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Nil } } => (__pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    normalSample = __pa3.clone();
    clockedSample = __pa4.clone();
    (callExp, outType, var) = (::match_deref::match_deref! { match &((args.clone(), namedArgs.clone())) {
        (Deref @ metamodelica::List::Cons { head: Deref @ TypedArg { ty: t1, value: e1, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ TypedArg { ty: Deref @ Type::INTEGER, value: e2, .. }, tail: Deref @ metamodelica::List::Nil } }, Deref @ metamodelica::List::Nil) => {
            let mut e1 = (*e1).clone();
            if Type::isInteger(t1.clone()) {
                e1 = Arc::new(Expression::NFExpression::CAST { ty: Arc::new(crate::NFType::REAL), exp: e1.clone() });
            }
            ty_call = Call::makeTypedCall(normalSample.clone(), list![e1.clone(), Arc::new(Expression::NFExpression::CAST { ty: Arc::new(crate::NFType::REAL), exp: e2.clone() })], Variability::PARAMETER.clone(), purity.clone(), Arc::new(crate::NFType::BOOLEAN));
            (Arc::new(Expression::NFExpression::CALL { call: ty_call.clone() }), Arc::new(crate::NFType::BOOLEAN), Variability::PARAMETER.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ TypedArg { ty: t1, value: e1, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ TypedArg { ty: Deref @ Type::REAL, value: e2, .. }, tail: Deref @ metamodelica::List::Nil } }, Deref @ metamodelica::List::Nil) => {
            let mut e1 = (*e1).clone();
            if Type::isInteger(t1.clone()) {
                e1 = Arc::new(Expression::NFExpression::CAST { ty: Arc::new(crate::NFType::REAL), exp: e1.clone() });
            }
            ty_call = Call::makeTypedCall(normalSample.clone(), list![e1.clone(), e2.clone()], Variability::PARAMETER.clone(), purity.clone(), Arc::new(crate::NFType::BOOLEAN));
            (Arc::new(Expression::NFExpression::CALL { call: ty_call.clone() }), Arc::new(crate::NFType::BOOLEAN), Variability::PARAMETER.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ TypedArg { ty: t1, value: e1, .. }, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: Deref @ TypedArg { ty: Deref @ Type::REAL, value: e2, name: Some(Deref @ "interval"), .. }, tail: Deref @ metamodelica::List::Nil }) => {
            let mut e1 = (*e1).clone();
            if Type::isInteger(t1.clone()) {
                e1 = Arc::new(Expression::NFExpression::CAST { ty: Arc::new(crate::NFType::REAL), exp: e1.clone() });
            }
            ty_call = Call::makeTypedCall(normalSample.clone(), list![e1.clone(), e2.clone()], Variability::PARAMETER.clone(), purity.clone(), Arc::new(crate::NFType::BOOLEAN));
            (Arc::new(Expression::NFExpression::CALL { call: ty_call.clone() }), Arc::new(crate::NFType::BOOLEAN), Variability::PARAMETER.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ TypedArg { var: v1, ty: t1, value: e1, .. }, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Nil) => {
            ty_call = Call::makeTypedCall(clockedSample.clone(), list![e1.clone(), Arc::new(Expression::NFExpression::CLKCONST { clk: Arc::new(ClockKind::NFClockKind::INFERRED_CLOCK { idx: System::tmpTickIndex(Global::inferredClock_index.clone()) }) })], v1.clone(), purity.clone(), t1.clone());
            (Arc::new(Expression::NFExpression::CALL { call: ty_call.clone() }), t1.clone(), v1.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ TypedArg { var: v1, ty: t1, value: e1, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ TypedArg { ty: Deref @ Type::CLOCK, value: e2, .. }, tail: Deref @ metamodelica::List::Nil } }, Deref @ metamodelica::List::Nil) => {
            ty_call = Call::makeTypedCall(clockedSample.clone(), list![e1.clone(), e2.clone()], v1.clone(), purity.clone(), t1.clone());
            (Arc::new(Expression::NFExpression::CALL { call: ty_call.clone() }), t1.clone(), v1.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ TypedArg { var: v1, ty: t1, value: e1, .. }, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: Deref @ TypedArg { ty: Deref @ Type::CLOCK, value: e2, name: Some(Deref @ "c"), .. }, tail: Deref @ metamodelica::List::Nil }) => {
            ty_call = Call::makeTypedCall(clockedSample.clone(), list![e1.clone(), e2.clone()], v1.clone(), purity.clone(), t1.clone());
            (Arc::new(Expression::NFExpression::CALL { call: ty_call.clone() }), t1.clone(), v1.clone())
        },
        _ => {
            Error::addSourceMessage(Error::WRONG_TYPE_OR_NO_OF_ARGS.clone(), list![(Call::toString(call.clone())?).clone(), (literal!("<NO COMPONENT>")).clone()], info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((callExp, outType, var, purity))
}

fn typeActualInStreamCall(mut name: ArcStr, mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut variability: Variability = Variability::DISCRETE.clone();
    let mut purity: Purity = Purity::IMPURE.clone();
    let mut fn_ref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut named_args: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut var: Variability = Variability::CONSTANT;
    let mut r#fn: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::UNTYPED_CALL { named_args: __pa0, arguments: __pa1, r#ref: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    named_args = __pa0.clone();
    args = __pa1.clone();
    fn_ref = __pa2.clone();
    assertNoNamedParams((name.clone()).clone(), named_args.clone(), info.clone())?;
    if (args.clone().len() as i32) != 1 {
        Error::addSourceMessageAndFail(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone(), list![(Call::toString(call.clone())?).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentRef::toString(fn_ref.clone())?); __mm_s.push_str(&*literal!("(stream variable) => Real")); ArcStr::from(__mm_s) }).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    (arg, ty, var, _) = Typing::typeExp(listHead(args.clone())?, context.clone(), info.clone(), false)?;
    (arg, _) = ExpandExp::expand(arg.clone(), false, false)?;
    let __pa3 = ::match_deref::match_deref! { match &(Function::typeRefCache(fn_ref.clone(), InstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Nil } => __pa3.clone(),
        _ => bail!("pattern mismatch"),
    } };
    r#fn = __pa3.clone();
    callExp = typeActualInStreamCall2((name.clone()).clone(), r#fn.clone(), arg.clone(), var.clone(), info.clone())?;
    Ok((callExp, ty, variability, purity))
}

fn typeActualInStreamCall2(mut name: ArcStr, mut r#fn: Arc<Function::Function>, mut arg: Arc<Expression::NFExpression>, mut var: Variability, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    callExp = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::CREF { .. } => {
            let mut arg_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
            arg_node = ComponentRef::node(var_field!((*arg).cref, Expression::NFExpression::CREF).clone())?;
            if !(InstNode::isComponent(arg_node.clone())) || !(Prefixes::ConnectorType::isStream(Component::connectorType(InstNode::component(arg_node.clone())?))) {
                Error::addSourceMessageAndFail(Error::NON_STREAM_OPERAND_IN_STREAM_OPERATOR.clone(), list![(ComponentRef::toString(var_field!((*arg).cref, Expression::NFExpression::CREF).clone())?).clone(), (name.clone()).clone()], info.clone())?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            for mut sub in &*ComponentRef::subscriptsAllFlat(var_field!((*arg).cref, Expression::NFExpression::CREF).clone()) {
                let mut sub = sub.clone();
                if Subscript::variability(sub.clone())? > Variability::PARAMETER.clone() {
                    Error::addSourceMessageAndFail(Error::CONNECTOR_NON_PARAMETER_SUBSCRIPT.clone(), list![(ComponentRef::toString(var_field!((*arg).cref, Expression::NFExpression::CREF).clone())?).clone(), (Subscript::toString(sub.clone())?).clone()], info.clone())?;
                    unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
                }
            }
            Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(r#fn.clone(), list![arg.clone()], var.clone(), Purity::IMPURE.clone(), var_field!((*arg).ty, Expression::NFExpression::CREF).clone()) })
        },
        Deref @ Expression::ARRAY { .. } => {
            assign_variant_field!(arg => Expression::NFExpression::ARRAY; elements = Array::map(var_field!((*arg).elements, Expression::NFExpression::ARRAY).clone(), Arc::new({ let __pe_b0 = (name.clone()).clone(); let __pe_b1 = r#fn.clone(); let __pe_b3 = var.clone(); let __pe_b4 = info.clone(); move |__pe_a2| typeActualInStreamCall2(__pe_b0.clone(), __pe_b1.clone(), __pe_a2, __pe_b3.clone(), __pe_b4.clone()) })));
            arg.clone()
        },
        _ => {
            Error::addSourceMessage(Error::NON_STREAM_OPERAND_IN_STREAM_OPERATOR.clone(), list![(Expression::toString(arg.clone())?).clone(), (name.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(callExp)
}

fn typeDynamicSelectCall(mut name: ArcStr, mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut variability: Variability = Variability::CONTINUOUS.clone();
    let mut purity: Purity = Purity::IMPURE.clone();
    let mut fn_ref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut named_args: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut arg1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut arg2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut var1: Variability = Variability::CONSTANT;
    let mut var2: Variability = Variability::CONSTANT;
    let mut r#fn: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    let mut ty1: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ty2: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut expStatic: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut expDynamic: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::UNTYPED_CALL { named_args: __pa0, arguments: __pa1, r#ref: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    named_args = __pa0.clone();
    args = __pa1.clone();
    fn_ref = __pa2.clone();
    assertNoNamedParams((name.clone()).clone(), named_args.clone(), info.clone())?;
    if (args.clone().len() as i32) != 2 {
        Error::addSourceMessageAndFail(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone(), list![(Call::toString(call.clone())?).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentRef::toString(fn_ref.clone())?); __mm_s.push_str(&*literal!("(static expression, dynamic expression)")); ArcStr::from(__mm_s) }).clone()], info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut arg in (args.clone()).into_iter().cloned() {
            let __x = Expression::unbox(arg.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })) {
        Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Cons { head: __pa4, tail: Deref @ metamodelica::List::Nil } } => (__pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    expStatic = __pa3.clone();
    expDynamic = __pa4.clone();
    (arg1, ty1, var1, _) = Typing::typeExp(expStatic.clone(), context.clone(), info.clone(), false)?;
    (arg1, _) = ExpandExp::expand(arg1.clone(), false, false)?;
    if let Ok((__pa6, __pa7, __pa8, _)) = Typing::typeExp(expDynamic.clone(), context.clone(), info.clone(), false) {
        arg2 = __pa6.clone();
        ty2 = __pa7.clone();
        var2 = __pa8.clone();
    } else {
        if InstContext::inInstanceAPI(context.clone()) {
            bail!("fail");
        } else {
            variability = var1.clone();
            callExp = arg1.clone();
            return Ok((callExp.clone(), ty.clone(), variability.clone(), purity.clone()));
        }
    }
    (arg2, _) = ExpandExp::expand(arg2.clone(), false, false)?;
    ty = ty1.clone();
    variability = var2.clone();
    let __pa9 = ::match_deref::match_deref! { match &(Function::typeRefCache(fn_ref.clone(), InstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::List::Cons { head: __pa9, tail: Deref @ metamodelica::List::Nil } => __pa9.clone(),
        _ => bail!("pattern mismatch"),
    } };
    r#fn = __pa9.clone();
    if Flags::isSet(Flags::NF_API_DYNAMIC_SELECT.clone())? {
        callExp = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(r#fn.clone(), list![arg1.clone(), arg2.clone()], variability.clone(), purity.clone(), ty1.clone()) });
    } else {
        variability = var1.clone();
        callExp = arg1.clone();
    }
    Ok((callExp, ty, variability, purity))
}

fn typeBackSampleCall(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::IMPURE.clone();
    let mut ty_call: Arc<Call::NFCall>;
    let mut counter: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut resolution: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let (__pa4, __pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(Call::typeMatchNormalCall(call.clone(), context.clone(), info.clone(), false)?) {
        __pa4 @ Deref @ Call::TYPED_CALL { var: __pa0, ty: __pa1, arguments: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Nil } } }, .. } => (__pa4.clone(), __pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    var = __pa0.clone();
    ty = __pa1.clone();
    counter = __pa2.clone();
    resolution = __pa3.clone();
    ty_call = __pa4.clone();
    Structural::markExp(counter.clone())?;
    Structural::markExp(resolution.clone())?;
    callExp = Arc::new(Expression::NFExpression::CALL { call: ty_call.clone() });
    Ok((callExp, ty, var, purity))
}

fn typeShiftSampleCall(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::IMPURE.clone();
    let mut ty_call: Arc<Call::NFCall>;
    let mut counter: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut resolution: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let (__pa4, __pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(Call::typeMatchNormalCall(call.clone(), context.clone(), info.clone(), false)?) {
        __pa4 @ Deref @ Call::TYPED_CALL { var: __pa0, ty: __pa1, arguments: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Nil } } }, .. } => (__pa4.clone(), __pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    var = __pa0.clone();
    ty = __pa1.clone();
    counter = __pa2.clone();
    resolution = __pa3.clone();
    ty_call = __pa4.clone();
    Structural::markExp(counter.clone())?;
    Structural::markExp(resolution.clone())?;
    callExp = Arc::new(Expression::NFExpression::CALL { call: ty_call.clone() });
    Ok((callExp, ty, var, purity))
}

fn typeSubSampleCall(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::IMPURE.clone();
    let mut ty_call: Arc<Call::NFCall>;
    let mut factor: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let (__pa3, __pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Call::typeMatchNormalCall(call.clone(), context.clone(), info.clone(), false)?) {
        __pa3 @ Deref @ Call::TYPED_CALL { var: __pa0, ty: __pa1, arguments: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil } }, .. } => (__pa3.clone(), __pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    var = __pa0.clone();
    ty = __pa1.clone();
    factor = __pa2.clone();
    ty_call = __pa3.clone();
    Structural::markExp(factor.clone())?;
    callExp = Arc::new(Expression::NFExpression::CALL { call: ty_call.clone() });
    Ok((callExp, ty, var, purity))
}

fn typeSuperSampleCall(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::IMPURE.clone();
    let mut ty_call: Arc<Call::NFCall>;
    let mut factor: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let (__pa3, __pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Call::typeMatchNormalCall(call.clone(), context.clone(), info.clone(), false)?) {
        __pa3 @ Deref @ Call::TYPED_CALL { var: __pa0, ty: __pa1, arguments: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil } }, .. } => (__pa3.clone(), __pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    var = __pa0.clone();
    ty = __pa1.clone();
    factor = __pa2.clone();
    ty_call = __pa3.clone();
    Structural::markExp(factor.clone())?;
    callExp = Arc::new(Expression::NFExpression::CALL { call: ty_call.clone() });
    Ok((callExp, ty, var, purity))
}

fn typePureCall(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::PURE.clone();
    let mut arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut c: Arc<Call::NFCall>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Call::typeMatchNormalCall(call.clone(), context.clone(), info.clone(), false)?) {
        Deref @ Call::TYPED_CALL { var: __pa0, ty: __pa1, arguments: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil }, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    var = __pa0.clone();
    ty = __pa1.clone();
    arg = __pa2.clone();
    callExp = Expression::unbox(arg.clone());
    callExp = (::match_deref::match_deref! { match &(callExp.clone()) {
        Deref @ Expression::CALL { call: c @ Deref @ Call::TYPED_CALL { .. } } => {
            let mut c = (*c).clone();
            assign_variant_field!(c => Call::NFCall::TYPED_CALL; purity = Expression::purityList(var_field!((*c).arguments, Call::NFCall::TYPED_CALL).clone(), Prefixes::Purity::PURE.clone())?);
            Arc::new(Expression::NFExpression::CALL { call: c.clone() })
        },
        _ => {
            Error::addSourceMessage(Error::FUNCTION_ARGUMENT_MUST_BE.clone(), list![(literal!("pure")).clone(), (Gettext::translateContent(Error::FUNCTION_CALL_EXPRESSION.clone())?).clone()], info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((callExp, ty, var, purity))
}

fn typeBuiltinCallExp(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo, mut vectorize: bool) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::CONSTANT;
    let mut pur: Purity = Purity::PURE;
    let mut c: Arc<Call::NFCall>;
    (c, ty, var, pur) = typeBuiltinCall(call.clone(), context.clone(), info.clone(), vectorize.clone())?;
    outExp = Arc::new(Expression::NFExpression::CALL { call: c.clone() });
    Ok((outExp, ty, var, pur))
}

fn typeBuiltinCall(mut call: Arc<Call::NFCall>, mut context: i32, mut info: SourceInfo, mut vectorize: bool) -> Result<(Arc<Call::NFCall>, Arc<Type::NFType>, Variability, Purity)> {
    let mut outCall: Arc<Call::NFCall>;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::CONSTANT;
    let mut pur: Purity = Purity::PURE;
    outCall = Call::typeMatchNormalCall(call.clone(), context.clone(), info.clone(), vectorize.clone())?;
    ty = Call::typeOf(outCall.clone());
    var = Call::variability(outCall.clone())?;
    pur = Call::purity(outCall.clone());
    Ok((outCall, ty, var, pur))
}

