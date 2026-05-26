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
use crate::NFCallAttributes;
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

pub fn makeSizeExp(mut posArgs: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut namedArgs: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>>, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut arg1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut arg2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    assertNoNamedParams((literal!("size")).clone(), namedArgs.clone(), info.clone())?;
    callExp = (::match_deref::match_deref! { match &(posArgs.clone()) {
        Deref @ metamodelica::List::Cons { head: arg1, tail: Deref @ metamodelica::List::Nil } => Arc::new(Expression::NFExpression::SIZE { exp: arg1.clone(), dimIndex: None }),
        Deref @ metamodelica::List::Cons { head: arg1, tail: Deref @ metamodelica::List::Cons { head: arg2, tail: Deref @ metamodelica::List::Nil } } => Arc::new(Expression::NFExpression::SIZE { exp: arg1.clone(), dimIndex: Some(arg2.clone()) }),
        _ => {
            Error::addSourceMessage(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("size")); __mm_s.push_str(&*List::toString(posArgs.clone(), Arc::new(Expression::toString), (literal!("")).clone(), (literal!("(")).clone(), (literal!(", ")).clone(), (literal!(")")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone(), (literal!("size(Any[:, ...]) => Integer[:]\n  size(Any[:, ...], Integer) => Integer")).clone()], info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(callExp)
}

pub fn makeArrayExp(mut posArgs: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut namedArgs: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>>, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
    let mut arrayExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut fn_ref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut named_args: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    assertNoNamedParams((literal!("array")).clone(), namedArgs.clone(), info.clone())?;
    if posArgs.clone().is_empty() {
        Error::addSourceMessage(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("array")); __mm_s.push_str(&*List::toString(posArgs.clone(), Arc::new(Expression::toString), (literal!("")).clone(), (literal!("(")).clone(), (literal!(", ")).clone(), (literal!(")")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone(), (literal!("array(Any, Any, ...) => Any[:]")).clone()], info.clone())?;
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
    maxn = {
        let mut __acc: Option<i32> = None;
        for mut d in (dimsLst.clone()).into_iter().cloned() {
            let __x = (d.clone().len() as i32);
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty max reduction"))?
    };
    if maxn.clone() != {
        let mut __acc: Option<i32> = None;
        for mut d in (dimsLst.clone()).into_iter().cloned() {
            let __x = (d.clone().len() as i32);
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty min reduction"))?
    } {
        Error::addSourceMessageAndFail(Error::NF_DIFFERENT_NUM_DIM_IN_ARGUMENTS.clone(), list![stringDelimitList({
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut d in (dimsLst.clone()).into_iter().cloned() {
            let __x = ArcStr::from(::std::format!("{}", (d.clone().len() as i32)));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(", ")).clone()), (literal!("cat")).clone()], info.clone())?;
    }
    if n.clone() < 1 || n.clone() > maxn.clone() {
        Error::addSourceMessageAndFail(Error::NF_CAT_WRONG_DIMENSION.clone(), list![ArcStr::from(::std::format!("{}", maxn.clone())), ArcStr::from(::std::format!("{}", n.clone()))], info.clone())?;
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
    dims = {
        let mut __acc: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
        for mut lst in (dimsLst.clone()).into_iter().cloned() {
            let __x = (lst.clone()).get(n.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
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

fn typeActualInStreamCall2(mut name: ArcStr, mut r#fn: Arc<Function::Function>, mut arg: Arc<Expression::NFExpression>, mut var: Variability, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    callExp = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::CREF { .. } => {
            let mut arg_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
            arg_node = ComponentRef::node(var_field!((*arg).cref, Expression::NFExpression::CREF).clone())?;
            if !(InstNode::isComponent(arg_node.clone())) || !(Prefixes::ConnectorType::isStream(Component::connectorType(InstNode::component(arg_node.clone())?))) {
                Error::addSourceMessageAndFail(Error::NON_STREAM_OPERAND_IN_STREAM_OPERATOR.clone(), list![(ComponentRef::toString(var_field!((*arg).cref, Expression::NFExpression::CREF).clone())?).clone(), (name.clone()).clone()], info.clone())?;
            }
            for mut sub in &*ComponentRef::subscriptsAllFlat(var_field!((*arg).cref, Expression::NFExpression::CREF).clone()) {
                let mut sub = sub.clone();
                if Subscript::variability(sub.clone())? > Variability::PARAMETER.clone() {
                    Error::addSourceMessageAndFail(Error::CONNECTOR_NON_PARAMETER_SUBSCRIPT.clone(), list![(ComponentRef::toString(var_field!((*arg).cref, Expression::NFExpression::CREF).clone())?).clone(), (Subscript::toString(sub.clone())?).clone()], info.clone())?;
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

