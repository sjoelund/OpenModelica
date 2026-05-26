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

use crate::BaseModelica;
use crate::NFBinding as Binding;
use crate::NFBuiltinCall as BuiltinCall;
use crate::NFBuiltinFuncs;
use crate::NFCallAttributes;
use crate::NFCallParameterTree;
use crate::NFCeval as Ceval;
use crate::NFClass as Class;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEvalFunction as EvalFunction;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFFunction::FunctionMatchKind;
use crate::NFFunction::MatchedFunction;
use crate::NFFunction::NamedArg;
use crate::NFFunction::TypedArg;
use crate::NFInline as Inline;
use crate::NFInst as Inst;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::CachedData;
use crate::NFInstNode::InstNode;
use crate::NFLookup as Lookup;
use crate::NFOperator as Operator;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFRecord as Record;
use crate::NFRestriction as Restriction;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFStructural as Structural;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFTypeCheck as TypeCheck;
use crate::NFTyping as Typing;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::JSON;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

#[derive(Clone, Debug, PartialEq)]
pub enum NFCall {
    UNTYPED_CALL {
        r#ref: Arc<ComponentRef::NFComponentRef>,
        arguments: Arc<metamodelica::List<Arc<Expression::NFExpression>>>,
        named_args: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>>,
        call_scope: Arc<InstNode::InstNode>,
    },
    ARG_TYPED_CALL {
        r#ref: Arc<ComponentRef::NFComponentRef>,
        positional_args: Arc<metamodelica::List<Arc<TypedArg>>>,
        named_args: Arc<metamodelica::List<Arc<TypedArg>>>,
        call_scope: Arc<InstNode::InstNode>,
    },
    TYPED_CALL {
        r#fn: Arc<Function::Function>,
        ty: Arc<Type::NFType>,
        var: Variability,
        purity: Purity,
        arguments: Arc<metamodelica::List<Arc<Expression::NFExpression>>>,
        attributes: Arc<NFCallAttributes::NFCallAttributes>,
    },
    UNTYPED_ARRAY_CONSTRUCTOR {
        exp: Arc<Expression::NFExpression>,
        iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>>,
    },
    TYPED_ARRAY_CONSTRUCTOR {
        ty: Arc<Type::NFType>,
        var: Variability,
        purity: Purity,
        exp: Arc<Expression::NFExpression>,
        iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>>,
    },
    UNTYPED_REDUCTION {
        r#ref: Arc<ComponentRef::NFComponentRef>,
        exp: Arc<Expression::NFExpression>,
        iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>>,
    },
    TYPED_REDUCTION {
        r#fn: Arc<Function::Function>,
        ty: Arc<Type::NFType>,
        var: Variability,
        purity: Purity,
        exp: Arc<Expression::NFExpression>,
        iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>>,
        defaultExp: Option<Arc<Expression::NFExpression>>,
        foldExp: (Option<Arc<Expression::NFExpression>>, ArcStr, ArcStr),
    },
}
pub use self::NFCall::{UNTYPED_CALL,ARG_TYPED_CALL,TYPED_CALL,UNTYPED_ARRAY_CONSTRUCTOR,TYPED_ARRAY_CONSTRUCTOR,UNTYPED_REDUCTION,TYPED_REDUCTION};
pub type ParameterTree = Arc<NFCallParameterTree::Tree>;

pub fn instantiate(mut functionName: Arc<Absyn::ComponentRef>, mut functionArgs: Arc<Absyn::FunctionArgs>, mut scope: Arc<InstNode::InstNode>, mut context: i32, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    callExp = (::match_deref::match_deref! { match &(functionArgs.clone()) {
        Deref @ Absyn::FunctionArgs::FUNCTIONARGS { .. } => instNormalCall(functionName.clone(), functionArgs.clone(), scope.clone(), context.clone(), info.clone())?,
        Deref @ Absyn::FunctionArgs::FOR_ITER_FARG { .. } => instIteratorCall(functionName.clone(), functionArgs.clone(), scope.clone(), context.clone(), info.clone())?,
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCall.instantiate")); __mm_s.push_str(&*literal!(" got unknown call type")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(callExp)
}

pub fn typeCall(mut callExp: Arc<Expression::NFExpression>, mut context: i32, mut info: SourceInfo, mut retype: bool) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::CONSTANT;
    let mut pur: Purity = Purity::PURE;
    let mut call: Arc<NFCall>;
    let mut ty_call: Arc<NFCall>;
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let __pa0 = ::match_deref::match_deref! { match &(callExp.clone()) {
        Deref @ Expression::CALL { call: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    call = __pa0.clone();
    outExp = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ UNTYPED_CALL { r#ref: cref, .. } => {
            if BuiltinCall::needSpecialHandling(call.clone())? {
                (outExp, ty, var, pur) = BuiltinCall::typeSpecial(call.clone(), context.clone(), info.clone())?;
            } else {
                checkNotPartial(cref.clone(), context.clone(), info.clone())?;
                ty_call = typeMatchNormalCall(call.clone(), context.clone(), info.clone(), true)?;
                (outExp, ty, var, pur) = typeCallExp(ty_call.clone())?;
            }
            outExp.clone()
        },
        Deref @ UNTYPED_ARRAY_CONSTRUCTOR { .. } => {
            (ty_call, ty, var, pur) = typeArrayConstructor(call.clone(), context.clone(), info.clone())?;
            Arc::new(Expression::NFExpression::CALL { call: ty_call.clone() })
        },
        Deref @ UNTYPED_REDUCTION { .. } => {
            checkNotPartial(var_field!((*call).r#ref, NFCall::UNTYPED_REDUCTION).clone(), context.clone(), info.clone())?;
            (ty_call, ty, var, pur) = typeReduction(call.clone(), context.clone(), info.clone())?;
            Arc::new(Expression::NFExpression::CALL { call: ty_call.clone() })
        },
        Deref @ TYPED_CALL { .. } if (retype.clone() && !(BuiltinCall::needSpecialHandling(call.clone())?)) => {
            ty_call = retypeCall(call.clone(), context.clone(), info.clone())?;
            (outExp, ty, var, pur) = typeCallExp(ty_call.clone())?;
            outExp.clone()
        },
        Deref @ TYPED_CALL { .. } => {
            ty = var_field!((*call).ty, NFCall::TYPED_CALL).clone();
            var = var_field!((*call).var, NFCall::TYPED_CALL).clone();
            pur = var_field!((*call).purity, NFCall::TYPED_CALL).clone();
            callExp.clone()
        },
        Deref @ TYPED_ARRAY_CONSTRUCTOR { .. } => {
            ty = var_field!((*call).ty, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone();
            var = var_field!((*call).var, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone();
            pur = var_field!((*call).purity, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone();
            callExp.clone()
        },
        Deref @ TYPED_REDUCTION { .. } => {
            ty = var_field!((*call).ty, NFCall::TYPED_REDUCTION).clone();
            var = var_field!((*call).var, NFCall::TYPED_REDUCTION).clone();
            pur = var_field!((*call).purity, NFCall::TYPED_REDUCTION).clone();
            callExp.clone()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCall.typeCall")); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*Expression::toString(callExp.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, ty, var, pur))
}

pub fn checkNotPartial(mut fnRef: Arc<ComponentRef::NFComponentRef>, mut context: i32, mut info: SourceInfo) -> Result<()> {
    if InstNode::isPartial(ComponentRef::node(fnRef.clone())?) && !(InstContext::inRelaxed(context.clone())) {
        Error::addSourceMessage(Error::PARTIAL_FUNCTION_CALL.clone(), list![(ComponentRef::toString(fnRef.clone())?).clone()], info.clone())?;
        bail!("fail");
    }
    Ok(())
}

pub fn typeCallExp(mut ty_call: Arc<NFCall>) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Variability, Purity)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::CONSTANT;
    let mut pur: Purity = Purity::PURE;
    ty = typeOf(ty_call.clone());
    var = variability(ty_call.clone())?;
    pur = purity(ty_call.clone());
    if isRecordConstructor(ty_call.clone())? {
        outExp = toRecordExpression(ty_call.clone(), ty.clone())?;
    } else {
        if Function::hasUnboxArgs(typedFunction(ty_call.clone())?) {
            outExp = Arc::new(Expression::NFExpression::CALL { call: unboxArgs(ty_call.clone()) });
        } else {
            outExp = Arc::new(Expression::NFExpression::CALL { call: ty_call.clone() });
        }
        outExp = Inline::inlineCallExp(outExp.clone(), false)?;
    }
    Ok((outExp, ty, var, pur))
}

pub fn typeNormalCall(mut call: Arc<NFCall>, mut context: i32, mut info: SourceInfo) -> Result<Arc<NFCall>> {
    let mut call: Arc<NFCall> = call;
    call = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ UNTYPED_CALL { .. } => {
            let mut fn_context: i32 = 0;
            if InstContext::inRelaxed(context.clone()) {
                fn_context = InstContext::set(InstContext::FUNCTION.clone(), InstContext::RELAXED.clone());
            } else {
                fn_context = InstContext::FUNCTION.clone();
            }
            let _ = Function::typeRefCache(var_field!((*call).r#ref, NFCall::UNTYPED_CALL).clone(), fn_context.clone())?;
            typeArgs(call.clone(), context.clone(), info.clone())?
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCall.typeNormalCall")); __mm_s.push_str(&*literal!(" got invalid function call expression")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(call)
}

pub fn makeTypedCall(mut r#fn: Arc<Function::Function>, mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut variability: Variability, mut purity: Purity, mut returnType: Arc<Type::NFType>) -> Arc<NFCall> {
    let mut call: Arc<NFCall>;
    let mut ca: Arc<NFCallAttributes::NFCallAttributes>;
    ca = Arc::new(NFCallAttributes::NFCallAttributes { tuple_: Type::isTuple(returnType.clone()), builtin: Function::isBuiltin(r#fn.clone()), isImpure: Function::isImpure(r#fn.clone()), isFunctionPointerCall: Function::isFunctionPointer(r#fn.clone()), inlineType: Function::inlineBuiltin(r#fn.clone()), tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL });
    call = Arc::new(NFCall::TYPED_CALL { r#fn: r#fn.clone(), ty: returnType.clone(), var: variability.clone(), purity: purity.clone(), arguments: args.clone(), attributes: ca.clone() });
    call
}

pub fn unboxArgs(mut call: Arc<NFCall>) -> Arc<NFCall> {
    let mut call: Arc<NFCall> = call;
    let mut c: Arc<NFCall>;
    let () = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ TYPED_CALL { .. } => {
            assign_variant_field!(call => NFCall::TYPED_CALL; arguments = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut arg in (var_field!((*call).arguments, NFCall::TYPED_CALL).clone()).into_iter().cloned() {
            let __x = Expression::unbox(arg.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        Deref @ TYPED_ARRAY_CONSTRUCTOR { exp: Deref @ Expression::CALL { call: c }, .. } => {
            assign_variant_field!(call => NFCall::TYPED_ARRAY_CONSTRUCTOR; exp = Arc::new(Expression::NFExpression::CALL { call: unboxArgs(c.clone()) }));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    call
}

pub fn typeMatchNormalCall(mut call: Arc<NFCall>, mut context: i32, mut info: SourceInfo, mut vectorize: bool) -> Result<Arc<NFCall>> {
    let mut call: Arc<NFCall> = call;
    let mut argtycall: Arc<NFCall>;
    argtycall = typeNormalCall(call.clone(), context.clone(), info.clone())?;
    call = matchTypedNormalCall(argtycall.clone(), context.clone(), info.clone(), vectorize.clone())?;
    Ok(call)
}

pub fn matchTypedNormalCall(mut call: Arc<NFCall>, mut context: i32, mut info: SourceInfo, mut vectorize: bool) -> Result<Arc<NFCall>> {
    let mut call: Arc<NFCall> = call;
    let mut func: Arc<Function::Function>;
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut typed_args: Arc<metamodelica::List<Arc<TypedArg>>> = metamodelica::nil();
    let mut matchedFunc: Arc<MatchedFunction::MatchedFunction>;
    let mut scope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut var: Variability = Variability::CONSTANT;
    let mut arg_var: Variability = Variability::CONSTANT;
    let mut pur: Purity = Purity::PURE;
    let mut arg_pur: Purity = Purity::PURE;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut arg_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let __pa0 = ::match_deref::match_deref! { match &(call.clone()) {
        Deref @ ARG_TYPED_CALL { call_scope: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    scope = __pa0.clone();
    matchedFunc = checkMatchingFunctions(call.clone(), context.clone(), info.clone(), vectorize.clone())?;
    func = matchedFunc.func.clone();
    typed_args = matchedFunc.args.clone();
    args = metamodelica::nil();
    var = Variability::CONSTANT.clone();
    pur = if (Function::isImpure(func.clone())) {Purity::IMPURE.clone()} else {Purity::PURE.clone()};
    for mut a in &*typed_args.clone() {
        let mut a = a.clone();
        let (__pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(a.clone()) {
            Deref @ TypedArg { purity: __pa1, var: __pa2, value: __pa3, .. } => (__pa1.clone(), __pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        arg_pur = __pa1.clone();
        arg_var = __pa2.clone();
        arg_exp = __pa3.clone();
        args = cons(arg_exp.clone(), args.clone());
        var = Prefixes::variabilityMax(var.clone(), arg_var.clone());
        pur = Prefixes::purityMin(pur.clone(), arg_pur.clone());
    }
    args = args.clone().reverse();
    ty = Function::returnType(func.clone());
    ty = resolvePolymorphicReturnType(func.clone(), typed_args.clone(), ty.clone())?;
    if var.clone() == Variability::PARAMETER.clone() && Function::isExternal(func.clone()) {
        var = Variability::NON_STRUCTURAL_PARAMETER.clone();
    } else if Type::isDiscrete(ty.clone()) && var.clone() == Variability::CONTINUOUS.clone() {
        var = Variability::IMPLICITLY_DISCRETE.clone();
    }
    (ty, _) = evaluateCallType(ty.clone(), func.clone(), args.clone(), 1, Arc::new(crate::NFCallParameterTree::Tree::EMPTY))?;
    call = makeTypedCall(func.clone(), args.clone(), var.clone(), pur.clone(), ty.clone());
    if MatchedFunction::isVectorized(matchedFunc.clone()) {
        call = vectorizeCall(call.clone(), matchedFunc.mk.clone(), scope.clone(), info.clone())?;
    }
    if Function::isExternal(func.clone()) {
        updateExternalRecordArgs(args.clone())?;
        updateExternalRecordArgsInType(ty.clone())?;
    }
    Ok(call)
}

pub fn retypeCall(mut call: Arc<NFCall>, mut context: i32, mut info: SourceInfo) -> Result<Arc<NFCall>> {
    let mut ty_call: Arc<NFCall>;
    let mut next_context: i32 = 0;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut arg_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut arg_var: Variability = Variability::CONSTANT;
    let mut arg_pur: Purity = Purity::PURE;
    let mut typed_args: Arc<metamodelica::List<Arc<TypedArg>>> = metamodelica::nil();
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    ty_call = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ TYPED_CALL { .. } => {
            next_context = InstContext::set(context.clone(), InstContext::SUBEXPRESSION.clone());
            for mut arg in &*var_field!((*call).arguments, NFCall::TYPED_CALL).clone().reverse() {
                let mut arg = arg.clone();
                (arg, arg_ty, arg_var, arg_pur) = Typing::typeExp(arg.clone(), next_context.clone(), info.clone(), true)?;
                typed_args = cons(Arc::new(TypedArg { name: None, value: arg.clone(), ty: arg_ty.clone(), var: arg_var.clone(), purity: arg_pur.clone() }), typed_args.clone());
                args = cons(arg.clone(), args.clone());
            }
            ty = Function::returnType(var_field!((*call).r#fn, NFCall::TYPED_CALL).clone());
            ty = resolvePolymorphicReturnType(var_field!((*call).r#fn, NFCall::TYPED_CALL).clone(), typed_args.clone(), ty.clone())?;
            (ty, _) = evaluateCallType(ty.clone(), var_field!((*call).r#fn, NFCall::TYPED_CALL).clone(), args.clone(), 1, Arc::new(crate::NFCallParameterTree::Tree::EMPTY))?;
            ty_call = makeTypedCall(var_field!((*call).r#fn, NFCall::TYPED_CALL).clone(), args.clone(), var_field!((*call).var, NFCall::TYPED_CALL).clone(), var_field!((*call).purity, NFCall::TYPED_CALL).clone(), ty.clone());
            ty_call.clone()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCall.retypeCall")); __mm_s.push_str(&*literal!(" got invalid function call expression")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ty_call)
}

pub fn typeOf(mut call: Arc<NFCall>) -> Arc<Type::NFType> {
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    ty = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ TYPED_CALL { .. } => var_field!((*call).ty, NFCall::TYPED_CALL).clone(),
        Deref @ TYPED_ARRAY_CONSTRUCTOR { .. } => var_field!((*call).ty, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(),
        Deref @ TYPED_REDUCTION { .. } => var_field!((*call).ty, NFCall::TYPED_REDUCTION).clone(),
        _ => Arc::new(crate::NFType::UNKNOWN),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ty
}

pub fn setType(mut call: Arc<NFCall>, mut ty: Arc<Type::NFType>) -> Result<Arc<NFCall>> {
    let mut call: Arc<NFCall> = call;
    call = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ TYPED_CALL { .. } => {
            assign_variant_field!(call => NFCall::TYPED_CALL; ty = ty.clone());
            call.clone()
        },
        Deref @ TYPED_ARRAY_CONSTRUCTOR { .. } => {
            assign_variant_field!(call => NFCall::TYPED_ARRAY_CONSTRUCTOR; ty = ty.clone());
            call.clone()
        },
        Deref @ TYPED_REDUCTION { .. } => {
            assign_variant_field!(call => NFCall::TYPED_REDUCTION; ty = ty.clone());
            call.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(call)
}

pub fn variability(mut call: Arc<NFCall>) -> Result<Variability> {
    let mut var: Variability = Variability::CONSTANT;
    var = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ UNTYPED_CALL { .. } => {
            let mut var_set: bool = false;
            var_set = true;
            if ComponentRef::isSimple(var_field!((*call).r#ref, NFCall::UNTYPED_CALL).clone()) {
                var = (::match_deref::match_deref! { match &(ComponentRef::firstName(var_field!((*call).r#ref, NFCall::UNTYPED_CALL).clone(), false)?) {
        Deref @ "change" => Variability::DISCRETE.clone(),
        Deref @ "edge" => Variability::DISCRETE.clone(),
        Deref @ "pre" => Variability::DISCRETE.clone(),
        Deref @ "ndims" => Variability::PARAMETER.clone(),
        Deref @ "cardinality" => Variability::PARAMETER.clone(),
        _ => {
            var_set = false;
            Variability::CONTINUOUS.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            if !(var_set.clone()) {
                var = Expression::variabilityList(var_field!((*call).arguments, NFCall::UNTYPED_CALL).clone(), Prefixes::Variability::CONSTANT.clone())?;
                for mut narg in &*var_field!((*call).named_args, NFCall::UNTYPED_CALL).clone() {
                    let mut narg = narg.clone();
                    var = Prefixes::variabilityMax(var.clone(), Expression::variability(Util::tuple22(narg.clone()))?);
                }
            }
            var.clone()
        },
        Deref @ UNTYPED_ARRAY_CONSTRUCTOR { .. } => {
            Expression::variability(var_field!((*call).exp, NFCall::UNTYPED_ARRAY_CONSTRUCTOR).clone())?
        },
        Deref @ UNTYPED_REDUCTION { .. } => {
            Expression::variability(var_field!((*call).exp, NFCall::UNTYPED_REDUCTION).clone())?
        },
        Deref @ TYPED_CALL { .. } => {
            var_field!((*call).var, NFCall::TYPED_CALL).clone()
        },
        Deref @ TYPED_ARRAY_CONSTRUCTOR { .. } => {
            var_field!((*call).var, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone()
        },
        Deref @ TYPED_REDUCTION { .. } => {
            var_field!((*call).var, NFCall::TYPED_REDUCTION).clone()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCall.variability")); __mm_s.push_str(&*literal!(" got untyped call")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(var)
}

pub fn purity(mut call: Arc<NFCall>) -> Purity {
    let mut purity: Purity = Purity::PURE;
    purity = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ TYPED_CALL { .. } => var_field!((*call).purity, NFCall::TYPED_CALL).clone(),
        Deref @ TYPED_ARRAY_CONSTRUCTOR { .. } => var_field!((*call).purity, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(),
        Deref @ TYPED_REDUCTION { .. } => var_field!((*call).purity, NFCall::TYPED_REDUCTION).clone(),
        _ => Purity::PURE.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    purity
}

pub fn compare(mut call1: Arc<NFCall>, mut call2: Arc<NFCall>) -> Result<i32> {
    let mut comp: i32 = 0;
    comp = AbsynUtil::pathCompare(functionName(call1.clone())?, functionName(call2.clone())?)?;
    if comp.clone() == 0 {
        comp = Expression::compareList(arguments(call1.clone())?, arguments(call2.clone())?);
    }
    if comp.clone() == 0 {
        comp = List::compare(iterators(call1.clone()), iterators(call2.clone()), Arc::new(compareIterator))?;
    }
    Ok(comp)
}

pub fn compareIterator(mut iter1: (Arc<InstNode::InstNode>, Arc<Expression::NFExpression>), mut iter2: (Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)) -> Result<i32> {
    let mut comp: i32 = 0;
    let mut n1: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut n2: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    (n1, e1) = iter1.clone();
    (n2, e2) = iter2.clone();
    comp = stringCompare((InstNode::name(n1.clone())?).clone(), (InstNode::name(n2.clone())?).clone());
    if comp.clone() == 0 {
        comp = Expression::compare(e1.clone(), e2.clone())?;
    }
    Ok(comp)
}

pub fn isExternal(mut call: Arc<NFCall>) -> Result<bool> {
    let mut isExternal: bool = false;
    isExternal = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ UNTYPED_CALL { .. } => Class::isExternalFunction(InstNode::getClass(ComponentRef::node(var_field!((*call).r#ref, NFCall::UNTYPED_CALL).clone())?)?)?,
        Deref @ ARG_TYPED_CALL { .. } => Class::isExternalFunction(InstNode::getClass(ComponentRef::node(var_field!((*call).r#ref, NFCall::ARG_TYPED_CALL).clone())?)?)?,
        Deref @ TYPED_CALL { .. } => Function::isExternal(var_field!((*call).r#fn, NFCall::TYPED_CALL).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isExternal)
}

pub fn isImpure(mut call: Arc<NFCall>) -> Result<bool> {
    let mut isImpure: bool = false;
    isImpure = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ UNTYPED_CALL { .. } => Function::isImpure(listHead(Function::getRefCache(var_field!((*call).r#ref, NFCall::UNTYPED_CALL).clone())?)?),
        Deref @ TYPED_CALL { purity: Prefixes::Purity::IMPURE, .. } => Function::isImpure(var_field!((*call).r#fn, NFCall::TYPED_CALL).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isImpure)
}

pub fn isRecordConstructor(mut call: Arc<NFCall>) -> Result<bool> {
    let mut isConstructor: bool = false;
    isConstructor = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ UNTYPED_CALL { .. } => SCodeUtil::isRecord(InstNode::definition(ComponentRef::node(var_field!((*call).r#ref, NFCall::UNTYPED_CALL).clone())?)?),
        Deref @ TYPED_CALL { .. } => SCodeUtil::isRecord(InstNode::definition(var_field!((*call).r#fn, NFCall::TYPED_CALL).node.clone())?),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isConstructor)
}

pub fn isExternalObjectConstructor(mut call: Arc<NFCall>) -> bool {
    let mut isConstructor: bool = false;
    isConstructor = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ TYPED_CALL { .. } => Type::isExternalObject(var_field!((*call).ty, NFCall::TYPED_CALL).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isConstructor
}

pub fn isLiteral(mut call: Arc<NFCall>) -> bool {
    fn is_literal_iter(mut iter: (Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)) -> bool {
        let mut literal: bool = Expression::isLiteral(Util::tuple22(iter.clone()));
        literal
    }

    let mut literal: bool = false;
    literal = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ TYPED_CALL { .. } => List::all(var_field!((*call).arguments, NFCall::TYPED_CALL).clone(), Arc::new(fnptr!(Expression::isLiteral, Arc<Expression::NFExpression>))),
        Deref @ TYPED_REDUCTION { .. } => Expression::isLiteral(var_field!((*call).exp, NFCall::TYPED_REDUCTION).clone()) && List::all(var_field!((*call).iters, NFCall::TYPED_REDUCTION).clone(), Arc::new(fnptr!(is_literal_iter, (Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)))),
        Deref @ TYPED_ARRAY_CONSTRUCTOR { .. } => Expression::isLiteral(var_field!((*call).exp, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone()) && List::all(var_field!((*call).iters, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), Arc::new(fnptr!(is_literal_iter, (Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)))),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    literal
}

pub fn isKnownSizeFill(mut call: Arc<NFCall>) -> Result<bool> {
    fn is_literal_iter(mut iter: (Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)) -> bool {
        let mut literal: bool = Expression::isLiteral(Util::tuple22(iter.clone()));
        literal
    }

    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ TYPED_CALL { .. } => isNamed(call.clone(), (literal!("fill")).clone())? && List::all(listRest(var_field!((*call).arguments, NFCall::TYPED_CALL).clone())?, Arc::new(fnptr!(Expression::isLiteral, Arc<Expression::NFExpression>))),
        Deref @ TYPED_ARRAY_CONSTRUCTOR { .. } => List::all(var_field!((*call).iters, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), Arc::new(fnptr!(is_literal_iter, (Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)))),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub fn isReduction(mut call: Arc<NFCall>) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ TYPED_REDUCTION { .. } => true,
        Deref @ TYPED_CALL { .. } => (::match_deref::match_deref! { match &(AbsynUtil::pathString(Function::nameConsiderBuiltin(var_field!((*call).r#fn, NFCall::TYPED_CALL).clone())?, (literal!(".")).clone(), true, false)?) {
        Deref @ "min" => true,
        Deref @ "max" => true,
        Deref @ "sum" => true,
        Deref @ "product" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn inlineType(mut call: Arc<NFCall>) -> DAE::InlineType {
    let mut inlineTy: DAE::InlineType = DAE::InlineType::AFTER_INDEX_RED_INLINE;
    inlineTy = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ TYPED_CALL { attributes: Deref @ NFCallAttributes::CALL_ATTR { inlineType: inlineTy, .. }, .. } => inlineTy.clone(),
        _ => openmodelica_frontend_types::DAE::InlineType::NO_INLINE,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    inlineTy
}

pub fn typedFunction(mut call: Arc<NFCall>) -> Result<Arc<Function::Function>> {
    let mut r#fn: Arc<Function::Function>;
    r#fn = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ TYPED_CALL { .. } => var_field!((*call).r#fn, NFCall::TYPED_CALL).clone(),
        Deref @ TYPED_ARRAY_CONSTRUCTOR { .. } => NFBuiltinFuncs::ARRAY_FUNC().clone(),
        Deref @ TYPED_REDUCTION { .. } => var_field!((*call).r#fn, NFCall::TYPED_REDUCTION).clone(),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCall.typedFunction")); __mm_s.push_str(&*literal!(" got untyped function")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(r#fn)
}

pub fn functionName(mut call: Arc<NFCall>) -> Result<Arc<Absyn::Path>> {
    let mut name: Arc<Absyn::Path>;
    name = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ UNTYPED_CALL { .. } => ComponentRef::toPath(var_field!((*call).r#ref, NFCall::UNTYPED_CALL).clone())?,
        Deref @ ARG_TYPED_CALL { .. } => ComponentRef::toPath(var_field!((*call).r#ref, NFCall::ARG_TYPED_CALL).clone())?,
        Deref @ TYPED_CALL { .. } => Function::nameConsiderBuiltin(var_field!((*call).r#fn, NFCall::TYPED_CALL).clone())?,
        Deref @ UNTYPED_ARRAY_CONSTRUCTOR { .. } => Arc::new(Absyn::Path::IDENT { name: (literal!("array")).clone() }),
        Deref @ TYPED_ARRAY_CONSTRUCTOR { .. } => Arc::new(Absyn::Path::IDENT { name: (literal!("array")).clone() }),
        Deref @ UNTYPED_REDUCTION { .. } => ComponentRef::toPath(var_field!((*call).r#ref, NFCall::UNTYPED_REDUCTION).clone())?,
        Deref @ TYPED_REDUCTION { .. } => Function::nameConsiderBuiltin(var_field!((*call).r#fn, NFCall::TYPED_REDUCTION).clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(name)
}

pub fn functionNameLast(mut call: Arc<NFCall>) -> ArcStr {
    let mut ident: ArcStr = AbsynUtil::pathLastIdent(functionName(call.clone()).unwrap()).unwrap();
    ident
}

pub fn functionNameFirst(mut call: Arc<NFCall>) -> ArcStr {
    let mut ident: ArcStr = AbsynUtil::pathFirstIdent(functionName(call.clone()).unwrap()).unwrap();
    ident
}

pub fn isNamed(mut call: Arc<NFCall>, mut name: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let mut path: Arc<Absyn::Path>;
    path = functionName(call.clone())?;
    res = (::match_deref::match_deref! { match &(path.clone()) {
        Deref @ Absyn::Path::IDENT { .. } => var_field!((*path).name, Absyn::Path::IDENT).clone() == name.clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub fn arguments(mut call: Arc<NFCall>) -> Result<Arc<metamodelica::List<Arc<Expression::NFExpression>>>> {
    let mut arguments: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    arguments = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ UNTYPED_CALL { .. } => var_field!((*call).arguments, NFCall::UNTYPED_CALL).clone(),
        Deref @ TYPED_CALL { .. } => var_field!((*call).arguments, NFCall::TYPED_CALL).clone(),
        Deref @ UNTYPED_ARRAY_CONSTRUCTOR { .. } => list![var_field!((*call).exp, NFCall::UNTYPED_ARRAY_CONSTRUCTOR).clone()],
        Deref @ TYPED_ARRAY_CONSTRUCTOR { .. } => list![var_field!((*call).exp, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone()],
        Deref @ UNTYPED_REDUCTION { .. } => list![var_field!((*call).exp, NFCall::UNTYPED_REDUCTION).clone()],
        Deref @ TYPED_REDUCTION { .. } => list![var_field!((*call).exp, NFCall::TYPED_REDUCTION).clone()],
        _ => bail!("match: no arm matched"),
    } });
    Ok(arguments)
}

pub fn setArguments(mut call: Arc<NFCall>, mut arguments: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<NFCall>> {
    let mut call: Arc<NFCall> = call;
    call = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ UNTYPED_CALL { .. } => {
            assign_variant_field!(call => NFCall::UNTYPED_CALL; arguments = arguments.clone());
            call.clone()
        },
        Deref @ TYPED_CALL { .. } => {
            assign_variant_field!(call => NFCall::TYPED_CALL; arguments = arguments.clone());
            call.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(call)
}

pub fn iterators(mut call: Arc<NFCall>) -> Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> {
    let mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    iters = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ UNTYPED_ARRAY_CONSTRUCTOR { .. } => var_field!((*call).iters, NFCall::UNTYPED_ARRAY_CONSTRUCTOR).clone(),
        Deref @ TYPED_ARRAY_CONSTRUCTOR { .. } => var_field!((*call).iters, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(),
        Deref @ UNTYPED_REDUCTION { .. } => var_field!((*call).iters, NFCall::UNTYPED_REDUCTION).clone(),
        Deref @ TYPED_REDUCTION { .. } => var_field!((*call).iters, NFCall::TYPED_REDUCTION).clone(),
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    iters
}

pub fn toRecordExpression(mut call: Arc<NFCall>, mut ty: Arc<Type::NFType>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ TYPED_CALL { .. } => EvalFunction::evaluateRecordConstructor(var_field!((*call).r#fn, NFCall::TYPED_CALL).clone(), ty.clone(), var_field!((*call).arguments, NFCall::TYPED_CALL).clone(), false)?,
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCall.toRecordExpression")); __mm_s.push_str(&*literal!(" got unknown call")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn toString(mut call: Arc<NFCall>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut name: ArcStr = arcstr::literal!("");
    let mut arg_str: ArcStr = arcstr::literal!("");
    let mut c: ArcStr = arcstr::literal!("");
    let mut argexp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut iters: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    r#str = ((::match_deref::match_deref! { match &(call.clone()) {
        Deref @ UNTYPED_CALL { .. } => {
            name = (ComponentRef::toString(var_field!((*call).r#ref, NFCall::UNTYPED_CALL).clone())?).clone();
            arg_str = stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut arg in (var_field!((*call).arguments, NFCall::UNTYPED_CALL).clone()).into_iter().cloned() {
            let __x = Expression::toString(arg.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(", ")).clone());
            { let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*arg_str.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        Deref @ ARG_TYPED_CALL { .. } => {
            name = (ComponentRef::toString(var_field!((*call).r#ref, NFCall::ARG_TYPED_CALL).clone())?).clone();
            arg_str = stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut arg in (var_field!((*call).positional_args, NFCall::ARG_TYPED_CALL).clone()).into_iter().cloned() {
            let __x = Expression::toString(arg.value.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(", ")).clone());
            for mut arg in &*var_field!((*call).named_args, NFCall::ARG_TYPED_CALL).clone() {
                let mut arg = arg.clone();
                c = (if (arg_str.clone() == literal!("")) {literal!("")} else {literal!(", ")}).clone();
                arg_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arg_str.clone()); __mm_s.push_str(&*c.clone()); __mm_s.push_str(&*Util::getOption(arg.name.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*Expression::toString(arg.value.clone())?); ArcStr::from(__mm_s) }).clone();
            }
            { let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*arg_str.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        Deref @ UNTYPED_ARRAY_CONSTRUCTOR { .. } => {
            name = (AbsynUtil::pathString(Function::nameConsiderBuiltin(NFBuiltinFuncs::ARRAY_FUNC().clone())?, (literal!(".")).clone(), true, false)?).clone();
            arg_str = (Expression::toString(var_field!((*call).exp, NFCall::UNTYPED_ARRAY_CONSTRUCTOR).clone())?).clone();
            c = stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut iter in (var_field!((*call).iters, NFCall::UNTYPED_ARRAY_CONSTRUCTOR).clone()).into_iter().cloned() {
            let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*InstNode::name(Util::tuple21(iter.clone()))?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*Expression::toString(Util::tuple22(iter.clone()))?); ArcStr::from(__mm_s) };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(", ")).clone());
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*arg_str.clone()); __mm_s.push_str(&*literal!(" for ")); __mm_s.push_str(&*c.clone()); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }
        },
        Deref @ UNTYPED_REDUCTION { .. } => {
            name = (ComponentRef::toString(var_field!((*call).r#ref, NFCall::UNTYPED_REDUCTION).clone())?).clone();
            arg_str = (Expression::toString(var_field!((*call).exp, NFCall::UNTYPED_REDUCTION).clone())?).clone();
            c = stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut iter in (var_field!((*call).iters, NFCall::UNTYPED_REDUCTION).clone()).into_iter().cloned() {
            let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*InstNode::name(Util::tuple21(iter.clone()))?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*Expression::toString(Util::tuple22(iter.clone()))?); ArcStr::from(__mm_s) };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(", ")).clone());
            { let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*arg_str.clone()); __mm_s.push_str(&*literal!(" for ")); __mm_s.push_str(&*c.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        Deref @ TYPED_CALL { .. } => {
            name = (AbsynUtil::pathString(Function::nameConsiderBuiltin(var_field!((*call).r#fn, NFCall::TYPED_CALL).clone())?, (literal!(".")).clone(), true, false)?).clone();
            arg_str = stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut arg in (var_field!((*call).arguments, NFCall::TYPED_CALL).clone()).into_iter().cloned() {
            let __x = Expression::toString(arg.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(", ")).clone());
            { let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*arg_str.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        Deref @ TYPED_ARRAY_CONSTRUCTOR { .. } => {
            name = (AbsynUtil::pathString(Function::nameConsiderBuiltin(NFBuiltinFuncs::ARRAY_FUNC().clone())?, (literal!(".")).clone(), true, false)?).clone();
            arg_str = (Expression::toString(var_field!((*call).exp, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone())?).clone();
            c = stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut iter in (var_field!((*call).iters, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone()).into_iter().cloned() {
            let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*InstNode::name(Util::tuple21(iter.clone()))?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*Expression::toString(Util::tuple22(iter.clone()))?); ArcStr::from(__mm_s) };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(", ")).clone());
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*arg_str.clone()); __mm_s.push_str(&*literal!(" for ")); __mm_s.push_str(&*c.clone()); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }
        },
        Deref @ TYPED_REDUCTION { .. } => {
            name = (AbsynUtil::pathString(Function::nameConsiderBuiltin(var_field!((*call).r#fn, NFCall::TYPED_REDUCTION).clone())?, (literal!(".")).clone(), true, false)?).clone();
            arg_str = (Expression::toString(var_field!((*call).exp, NFCall::TYPED_REDUCTION).clone())?).clone();
            c = stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut iter in (var_field!((*call).iters, NFCall::TYPED_REDUCTION).clone()).into_iter().cloned() {
            let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*InstNode::name(Util::tuple21(iter.clone()))?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*Expression::toString(Util::tuple22(iter.clone()))?); ArcStr::from(__mm_s) };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(", ")).clone());
            { let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*arg_str.clone()); __mm_s.push_str(&*literal!(" for ")); __mm_s.push_str(&*c.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(r#str)
}

pub fn toFlatString(mut call: Arc<NFCall>, mut format: BaseModelica::OutputFormat) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut name: ArcStr = arcstr::literal!("");
    let mut arg_str: ArcStr = arcstr::literal!("");
    let mut c: ArcStr = arcstr::literal!("");
    let mut argexp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut iters: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    r#str = ((::match_deref::match_deref! { match &(call.clone()) {
        Deref @ TYPED_CALL { .. } => {
            name = (AbsynUtil::pathString(Function::nameConsiderBuiltin(var_field!((*call).r#fn, NFCall::TYPED_CALL).clone())?, (literal!(".")).clone(), true, false)?).clone();
            arg_str = (toFlatStringArgs(var_field!((*call).arguments, NFCall::TYPED_CALL).clone(), (name.clone()).clone(), format.clone())?).clone();
            if (Function::isBuiltin(var_field!((*call).r#fn, NFCall::TYPED_CALL).clone())) {stringAppendList(list![(name.clone()).clone(), (literal!("(")).clone(), (arg_str.clone()).clone(), (literal!(")")).clone()])} else if (isExternalObjectConstructor(call.clone())) {stringAppendList(list![(Type::toFlatString(var_field!((*call).ty, NFCall::TYPED_CALL).clone(), format.clone())?).clone(), (literal!("(")).clone(), (arg_str.clone()).clone(), (literal!(")")).clone()])} else {stringAppendList(list![(Util::makeQuotedIdentifier((name.clone()).clone())?).clone(), (literal!("(")).clone(), (arg_str.clone()).clone(), (literal!(")")).clone()])}
        },
        Deref @ TYPED_ARRAY_CONSTRUCTOR { .. } => {
            if isVectorized(call.clone())? {
                r#str = (Expression::toFlatString(devectorizeCall(call.clone())?, format.clone())?).clone();
            } else {
                name = (AbsynUtil::pathString(Function::nameConsiderBuiltin(NFBuiltinFuncs::ARRAY_FUNC().clone())?, (literal!(".")).clone(), true, false)?).clone();
                arg_str = (Expression::toFlatString(var_field!((*call).exp, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), format.clone())?).clone();
                c = stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut iter in (var_field!((*call).iters, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone()).into_iter().cloned() {
            let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*Util::makeQuotedIdentifier((InstNode::name(Util::tuple21(iter.clone()))?).clone())?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*Expression::toFlatString(Util::tuple22(iter.clone()), format.clone())?); ArcStr::from(__mm_s) };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(", ")).clone());
                r#str = stringAppendList(list![(literal!("{")).clone(), (arg_str.clone()).clone(), (literal!(" for ")).clone(), (c.clone()).clone(), (literal!("}")).clone()]);
            }
            r#str.clone()
        },
        Deref @ TYPED_REDUCTION { .. } => {
            name = (AbsynUtil::pathString(Function::nameConsiderBuiltin(var_field!((*call).r#fn, NFCall::TYPED_REDUCTION).clone())?, (literal!(".")).clone(), true, false)?).clone();
            arg_str = (Expression::toFlatString(var_field!((*call).exp, NFCall::TYPED_REDUCTION).clone(), format.clone())?).clone();
            c = stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut iter in (var_field!((*call).iters, NFCall::TYPED_REDUCTION).clone()).into_iter().cloned() {
            let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*Util::makeQuotedIdentifier((InstNode::name(Util::tuple21(iter.clone()))?).clone())?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*Expression::toFlatString(Util::tuple22(iter.clone()), format.clone())?); ArcStr::from(__mm_s) };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(", ")).clone());
            if (Function::isBuiltin(var_field!((*call).r#fn, NFCall::TYPED_REDUCTION).clone())) {stringAppendList(list![(name.clone()).clone(), (literal!("(")).clone(), (arg_str.clone()).clone(), (literal!(" for ")).clone(), (c.clone()).clone(), (literal!(")")).clone()])} else {stringAppendList(list![(Util::makeQuotedIdentifier((name.clone()).clone())?).clone(), (literal!("(")).clone(), (arg_str.clone()).clone(), (literal!(" for ")).clone(), (c.clone()).clone(), (literal!(")")).clone()])}
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(r#str)
}

pub fn toFlatStringArgs(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut fnName: ArcStr, mut format: BaseModelica::OutputFormat) -> Result<ArcStr> {
    let mut argsString: ArcStr = arcstr::literal!("");
    let mut arg1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut arg2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut rest_args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    argsString = ((::match_deref::match_deref! { match &(fnName.clone()) {
        Deref @ "String" => (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: arg1, tail: Deref @ metamodelica::List::Cons { head: arg2, tail: Deref @ metamodelica::List::Nil } } => { let mut __mm_s = String::new(); __mm_s.push_str(&*Expression::toFlatString(arg1.clone(), format.clone())?); __mm_s.push_str(&*literal!(", format = ")); __mm_s.push_str(&*Expression::toFlatString(arg2.clone(), format.clone())?); ArcStr::from(__mm_s) },
        _ => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(args.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            arg1 = __pa0.clone();
            rest_args = __pa1.clone();
            argsString = (Expression::toFlatString(arg1.clone(), format.clone())?).clone();
            if (rest_args.clone().len() as i32) == 3 {
                let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest_args.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                arg1 = __pa2.clone();
                rest_args = __pa3.clone();
                if !(Expression::isIntegerValue(arg1.clone(), 6)) {
                    argsString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*argsString.clone()); __mm_s.push_str(&*literal!(", significantDigits = ")); __mm_s.push_str(&*Expression::toFlatString(arg1.clone(), format.clone())?); ArcStr::from(__mm_s) }).clone();
                }
            }
            let (__pa4, __pa5) = ::match_deref::match_deref! { match &(rest_args.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
                _ => bail!("pattern mismatch"),
            } };
            arg1 = __pa4.clone();
            rest_args = __pa5.clone();
            if !(Expression::isZero(arg1.clone())) {
                argsString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*argsString.clone()); __mm_s.push_str(&*literal!(", minimumLength = ")); __mm_s.push_str(&*Expression::toFlatString(arg1.clone(), format.clone())?); ArcStr::from(__mm_s) }).clone();
            }
            let (__pa6, __pa7) = ::match_deref::match_deref! { match &(rest_args.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa6, tail: __pa7 } => (__pa6.clone(), __pa7.clone()),
                _ => bail!("pattern mismatch"),
            } };
            arg1 = __pa6.clone();
            rest_args = __pa7.clone();
            if !(Expression::isTrue(arg1.clone())) {
                argsString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*argsString.clone()); __mm_s.push_str(&*literal!(", leftJustified = ")); __mm_s.push_str(&*Expression::toFlatString(arg1.clone(), format.clone())?); ArcStr::from(__mm_s) }).clone();
            }
            argsString.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }),
        _ => stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut arg in (args.clone()).into_iter().cloned() {
            let __x = Expression::toFlatString(arg.clone(), format.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(", ")).clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(argsString)
}

pub fn typedString(mut call: Arc<NFCall>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut name: ArcStr = arcstr::literal!("");
    let mut arg_str: ArcStr = arcstr::literal!("");
    let mut c: ArcStr = arcstr::literal!("");
    let mut argexp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    r#str = ((::match_deref::match_deref! { match &(call.clone()) {
        Deref @ ARG_TYPED_CALL { .. } => {
            name = (ComponentRef::toString(var_field!((*call).r#ref, NFCall::ARG_TYPED_CALL).clone())?).clone();
            arg_str = stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut arg in (var_field!((*call).positional_args, NFCall::ARG_TYPED_CALL).clone()).into_iter().cloned() {
            let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("/*")); __mm_s.push_str(&*Type::toString(arg.ty.clone())?); __mm_s.push_str(&*literal!("*/ ")); __mm_s.push_str(&*Expression::toString(arg.value.clone())?); ArcStr::from(__mm_s) };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(", ")).clone());
            for mut arg in &*var_field!((*call).named_args, NFCall::ARG_TYPED_CALL).clone() {
                let mut arg = arg.clone();
                c = (if (arg_str.clone() == literal!("")) {literal!("")} else {literal!(", ")}).clone();
                arg_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arg_str.clone()); __mm_s.push_str(&*c.clone()); __mm_s.push_str(&*Util::getOption(arg.name.clone())?); __mm_s.push_str(&*literal!(" = /*")); __mm_s.push_str(&*Type::toString(arg.ty.clone())?); __mm_s.push_str(&*literal!("*/ ")); __mm_s.push_str(&*Expression::toString(arg.value.clone())?); ArcStr::from(__mm_s) }).clone();
            }
            { let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*arg_str.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        Deref @ TYPED_CALL { .. } => {
            name = (AbsynUtil::pathString(Function::name(var_field!((*call).r#fn, NFCall::TYPED_CALL).clone()), (literal!(".")).clone(), true, false)?).clone();
            arg_str = stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut arg in (var_field!((*call).arguments, NFCall::TYPED_CALL).clone()).into_iter().cloned() {
            let __x = Expression::toStringTyped(arg.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(", ")).clone());
            { let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*arg_str.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        _ => toString(call.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(r#str)
}

pub fn toJSON(mut call: Arc<NFCall>) -> Result<Arc<JSON::JSON>> {
    pub fn iterators_json(mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>>) -> Result<Arc<JSON::JSON>> {
        let mut json: Arc<JSON::JSON> = JSON::emptyArray((iters.clone().len() as i32));
        let mut j: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
        for mut i in &*iters.clone() {
            let mut i = i.clone();
            j = JSON::emptyListObject();
            j = JSON::addPair((literal!("name")).clone(), JSON::makeString((InstNode::name(Util::tuple21(i.clone()))?).clone()), j.clone())?;
            j = JSON::addPair((literal!("range")).clone(), Expression::toJSON(Util::tuple22(i.clone()))?, j.clone())?;
            json = JSON::addElement(j.clone(), json.clone())?;
        }
        Ok(json)
    }

    let mut json: Arc<JSON::JSON> = JSON::emptyListObject();
    let mut path: Arc<Absyn::Path>;
    let () = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ TYPED_CALL { .. } => {
            path = Function::nameConsiderBuiltin(var_field!((*call).r#fn, NFCall::TYPED_CALL).clone())?;
            json = JSON::addPair((literal!("$kind")).clone(), JSON::makeString((literal!("call")).clone()), json.clone())?;
            json = JSON::addPair((literal!("name")).clone(), JSON::makeString((AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone()), json.clone())?;
            if isNamed(call.clone(), (literal!("String")).clone())? {
                json = toJSONStringArgs(var_field!((*call).arguments, NFCall::TYPED_CALL).clone(), json.clone())?;
            } else {
                json = JSON::addPair((literal!("arguments")).clone(), JSON::makeArray({
        let mut __acc: Arc<metamodelica::List<Arc<JSON::JSON>>> = metamodelica::nil();
        for mut a in (var_field!((*call).arguments, NFCall::TYPED_CALL).clone()).into_iter().cloned() {
            let __x = Expression::toJSON(a.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), json.clone())?;
            }
            ()
        },
        Deref @ TYPED_ARRAY_CONSTRUCTOR { .. } => {
            json = JSON::addPair((literal!("$kind")).clone(), JSON::makeString((literal!("iterator_call")).clone()), json.clone())?;
            json = JSON::addPair((literal!("name")).clone(), JSON::makeString((literal!("$array")).clone()), json.clone())?;
            json = JSON::addPair((literal!("exp")).clone(), Expression::toJSON(var_field!((*call).exp, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone())?, json.clone())?;
            json = JSON::addPair((literal!("iterators")).clone(), iterators_json(var_field!((*call).iters, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone())?, json.clone())?;
            ()
        },
        Deref @ TYPED_REDUCTION { .. } => {
            path = Function::nameConsiderBuiltin(var_field!((*call).r#fn, NFCall::TYPED_REDUCTION).clone())?;
            json = JSON::addPair((literal!("$kind")).clone(), JSON::makeString((literal!("iterator_call")).clone()), json.clone())?;
            json = JSON::addPair((literal!("name")).clone(), JSON::makeString((AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone()), json.clone())?;
            json = JSON::addPair((literal!("exp")).clone(), Expression::toJSON(var_field!((*call).exp, NFCall::TYPED_REDUCTION).clone())?, json.clone())?;
            json = JSON::addPair((literal!("iterators")).clone(), iterators_json(var_field!((*call).iters, NFCall::TYPED_REDUCTION).clone())?, json.clone())?;
            ()
        },
        _ => {
            json = JSON::addPair((literal!("$kind")).clone(), JSON::makeString((literal!("call")).clone()), json.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(json)
}

pub fn toJSONStringArgs(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut json: Arc<JSON::JSON>) -> Result<Arc<JSON::JSON>> {
    fn make_arg(mut name: ArcStr, mut value: Arc<Expression::NFExpression>) -> Result<Arc<JSON::JSON>> {
        let mut json: Arc<JSON::JSON> = JSON::emptyListObject();
        json = JSON::addPair((literal!("$kind")).clone(), JSON::makeString((literal!("named_arg")).clone()), json.clone())?;
        json = JSON::addPair((name.clone()).clone(), Expression::toJSON(value.clone())?, json.clone())?;
        Ok(json)
    }

    let mut json: Arc<JSON::JSON> = json;
    let mut arg_count: i32 = 0;
    let mut value: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut rest_args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut json_args: Arc<metamodelica::List<Arc<JSON::JSON>>> = metamodelica::nil();
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    value = __pa0.clone();
    rest_args = __pa1.clone();
    arg_count = (rest_args.clone().len() as i32);
    json_args = list![Expression::toJSON(value.clone())?];
    if arg_count.clone() == 1 {
        let __pa2 = ::match_deref::match_deref! { match &(rest_args.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa2, tail: _ } => __pa2.clone(),
            _ => bail!("pattern mismatch"),
        } };
        arg = __pa2.clone();
        json_args = cons(make_arg((literal!("format")).clone(), arg.clone())?, json_args.clone());
    } else {
        if arg_count.clone() == 3 {
            let (__pa3, __pa4) = ::match_deref::match_deref! { match &(rest_args.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
                _ => bail!("pattern mismatch"),
            } };
            arg = __pa3.clone();
            rest_args = __pa4.clone();
            if !(Expression::isIntegerValue(arg.clone(), 6)) {
                json_args = cons(make_arg((literal!("significantDigits")).clone(), arg.clone())?, json_args.clone());
            }
        }
        let (__pa5, __pa6) = ::match_deref::match_deref! { match &(rest_args.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa5, tail: __pa6 } => (__pa5.clone(), __pa6.clone()),
            _ => bail!("pattern mismatch"),
        } };
        arg = __pa5.clone();
        rest_args = __pa6.clone();
        if !(Expression::isZero(arg.clone())) {
            json_args = cons(make_arg((literal!("minimumLength")).clone(), arg.clone())?, json_args.clone());
        }
        let (__pa7, __pa8) = ::match_deref::match_deref! { match &(rest_args.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa7, tail: __pa8 } => (__pa7.clone(), __pa8.clone()),
            _ => bail!("pattern mismatch"),
        } };
        arg = __pa7.clone();
        rest_args = __pa8.clone();
        if !(Expression::isTrue(arg.clone())) {
            json_args = cons(make_arg((literal!("leftJustified")).clone(), arg.clone())?, json_args.clone());
        }
    }
    json = JSON::addPair((literal!("arguments")).clone(), JSON::makeList(json_args.clone().reverse()), json.clone())?;
    Ok(json)
}

pub fn toAbsyn(mut call: Arc<NFCall>) -> Result<Arc<Absyn::Exp>> {
    let mut absynCall: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    absynCall = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ UNTYPED_CALL { .. } => {
            let mut pargs: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
            let mut nargs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
            pargs = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
        for mut arg in (var_field!((*call).arguments, NFCall::UNTYPED_CALL).clone()).into_iter().cloned() {
            let __x = Expression::toAbsyn(arg.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            nargs = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
        for mut arg in (var_field!((*call).named_args, NFCall::UNTYPED_CALL).clone()).into_iter().cloned() {
            let __x = Arc::new(Absyn::NamedArg { argName: (Util::tuple21(arg.clone())).clone(), argValue: Expression::toAbsyn(Util::tuple22(arg.clone()))? });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            AbsynUtil::makeCall(ComponentRef::toAbsyn(var_field!((*call).r#ref, NFCall::UNTYPED_CALL).clone())?, pargs.clone(), nargs.clone())
        },
        Deref @ ARG_TYPED_CALL { .. } => {
            let mut pargs: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
            let mut nargs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
            pargs = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
        for mut arg in (var_field!((*call).positional_args, NFCall::ARG_TYPED_CALL).clone()).into_iter().cloned() {
            let __x = Expression::toAbsyn(arg.value.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            nargs = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
        for mut arg in (var_field!((*call).named_args, NFCall::ARG_TYPED_CALL).clone()).into_iter().cloned() {
            let __x = Arc::new(Absyn::NamedArg { argName: (Util::getOption(arg.name.clone())?).clone(), argValue: Expression::toAbsyn(arg.value.clone())? });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            AbsynUtil::makeCall(ComponentRef::toAbsyn(var_field!((*call).r#ref, NFCall::ARG_TYPED_CALL).clone())?, pargs.clone(), nargs.clone())
        },
        Deref @ TYPED_CALL { .. } => {
            let mut pargs: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
            pargs = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
        for mut arg in (var_field!((*call).arguments, NFCall::TYPED_CALL).clone()).into_iter().cloned() {
            let __x = Expression::toAbsyn(arg.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            AbsynUtil::makeCall(AbsynUtil::pathToCref(Function::name(var_field!((*call).r#fn, NFCall::TYPED_CALL).clone()))?, pargs.clone(), metamodelica::nil())
        },
        Deref @ UNTYPED_ARRAY_CONSTRUCTOR { .. } => {
            Arc::new(Absyn::Exp::CALL { function_: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("array")).clone(), subscripts: metamodelica::nil() }), functionArgs: toAbsynIterators(var_field!((*call).exp, NFCall::UNTYPED_ARRAY_CONSTRUCTOR).clone(), var_field!((*call).iters, NFCall::UNTYPED_ARRAY_CONSTRUCTOR).clone())?, typeVars: metamodelica::nil() })
        },
        Deref @ TYPED_ARRAY_CONSTRUCTOR { .. } => {
            Arc::new(Absyn::Exp::CALL { function_: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("array")).clone(), subscripts: metamodelica::nil() }), functionArgs: toAbsynIterators(var_field!((*call).exp, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), var_field!((*call).iters, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone())?, typeVars: metamodelica::nil() })
        },
        Deref @ UNTYPED_REDUCTION { .. } => {
            Arc::new(Absyn::Exp::CALL { function_: ComponentRef::toAbsyn(var_field!((*call).r#ref, NFCall::UNTYPED_REDUCTION).clone())?, functionArgs: toAbsynIterators(var_field!((*call).exp, NFCall::UNTYPED_REDUCTION).clone(), var_field!((*call).iters, NFCall::UNTYPED_REDUCTION).clone())?, typeVars: metamodelica::nil() })
        },
        Deref @ TYPED_REDUCTION { .. } => {
            Arc::new(Absyn::Exp::CALL { function_: AbsynUtil::pathToCref(Function::name(var_field!((*call).r#fn, NFCall::TYPED_REDUCTION).clone()))?, functionArgs: toAbsynIterators(var_field!((*call).exp, NFCall::TYPED_REDUCTION).clone(), var_field!((*call).iters, NFCall::TYPED_REDUCTION).clone())?, typeVars: metamodelica::nil() })
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCall.toAbsyn")); __mm_s.push_str(&*literal!(" got unknown call")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(absynCall)
}

pub fn toAbsynIterators(mut iterExp: Arc<Expression::NFExpression>, mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>>) -> Result<Arc<Absyn::FunctionArgs>> {
    let mut args: Arc<Absyn::FunctionArgs>;
    args = Arc::new(Absyn::FunctionArgs::FOR_ITER_FARG { exp: Expression::toAbsyn(iterExp.clone())?, iterType: openmodelica_ast::Absyn::ReductionIterType::COMBINE, iterators: {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ForIterator>>> = metamodelica::nil();
        for mut i in (iters.clone()).into_iter().cloned() {
            let __x = Arc::new(Absyn::ForIterator { name: (InstNode::name(Util::tuple21(i.clone()))?).clone(), guardExp: None, range: Some(Expression::toAbsyn(Util::tuple22(i.clone()))?) });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } });
    Ok(args)
}

pub fn toDAE(mut call: Arc<NFCall>) -> Result<Arc<DAE::Exp>> {
    let mut daeCall: Arc<DAE::Exp>;
    daeCall = toDAE_work(expandReduction(call.clone())?)?;
    Ok(daeCall)
}

pub fn toDAE_work(mut call: Arc<NFCall>) -> Result<Arc<DAE::Exp>> {
    let mut daeCall: Arc<DAE::Exp>;
    daeCall = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ TYPED_CALL { .. } => {
            Arc::new(DAE::Exp::CALL { path: Function::nameConsiderBuiltin(var_field!((*call).r#fn, NFCall::TYPED_CALL).clone())?, expLst: {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut e in (var_field!((*call).arguments, NFCall::TYPED_CALL).clone()).into_iter().cloned() {
            let __x = Expression::toDAE(e.clone(), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, attr: NFCallAttributes::toDAE(var_field!((*call).attributes, NFCall::TYPED_CALL).clone(), var_field!((*call).ty, NFCall::TYPED_CALL).clone())? })
        },
        Deref @ TYPED_ARRAY_CONSTRUCTOR { .. } => {
            let mut fold_id: ArcStr = arcstr::literal!("");
            let mut res_id: ArcStr = arcstr::literal!("");
            fold_id = (Util::getTempVariableIndex()).clone();
            res_id = (Util::getTempVariableIndex()).clone();
            Arc::new(DAE::Exp::REDUCTION { reductionInfo: Arc::new(DAE::ReductionInfo { path: Function::name(NFBuiltinFuncs::ARRAY_FUNC().clone()), iterType: openmodelica_ast::Absyn::ReductionIterType::COMBINE, exprType: Type::toDAE(var_field!((*call).ty, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), true)?, defaultValue: None, foldName: (fold_id.clone()).clone(), resultName: (res_id.clone()).clone(), foldExp: None }), expr: Expression::toDAE(var_field!((*call).exp, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), false)?, iterators: {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::ReductionIterator>>> = metamodelica::nil();
        for mut iter in (var_field!((*call).iters, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone()).into_iter().cloned() {
            let __x = iteratorToDAE(iter.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } })
        },
        Deref @ TYPED_REDUCTION { .. } => {
            let mut fold_id: ArcStr = arcstr::literal!("");
            let mut res_id: ArcStr = arcstr::literal!("");
            let mut fold_exp: Option<Arc<Expression::NFExpression>> = None;
            (fold_exp, fold_id, res_id) = var_field!((*call).foldExp, NFCall::TYPED_REDUCTION).clone();
            Arc::new(DAE::Exp::REDUCTION { reductionInfo: Arc::new(DAE::ReductionInfo { path: Function::name(var_field!((*call).r#fn, NFCall::TYPED_REDUCTION).clone()), iterType: openmodelica_ast::Absyn::ReductionIterType::COMBINE, exprType: Type::toDAE(var_field!((*call).ty, NFCall::TYPED_REDUCTION).clone(), true)?, defaultValue: Util::applyOption(var_field!((*call).defaultExp, NFCall::TYPED_REDUCTION).clone(), Arc::new(Expression::toDAEValue)), foldName: (fold_id.clone()).clone(), resultName: (res_id.clone()).clone(), foldExp: Util::applyOption(fold_exp.clone(), Arc::new({ let __pe_b1 = false; move |__pe_a0| Expression::toDAE(__pe_a0, __pe_b1.clone()) })) }), expr: Expression::toDAE(var_field!((*call).exp, NFCall::TYPED_REDUCTION).clone(), false)?, iterators: {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::ReductionIterator>>> = metamodelica::nil();
        for mut iter in (var_field!((*call).iters, NFCall::TYPED_REDUCTION).clone()).into_iter().cloned() {
            let __x = iteratorToDAE(iter.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } })
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCall.toDAE_work")); __mm_s.push_str(&*literal!(" got untyped call")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(daeCall)
}

pub fn expandReduction(mut call: Arc<NFCall>) -> Result<Arc<NFCall>> {
    let mut outCall: Arc<NFCall>;
    outCall = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ TYPED_ARRAY_CONSTRUCTOR { iters, .. } if ((iters.clone().len() as i32) > 1) => {
            let mut iter: (Arc<InstNode::InstNode>, Arc<Expression::NFExpression>);
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut iters = (*iters).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(iters.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            iter = __pa0.clone();
            iters = __pa1.clone();
            ty = Type::liftArrayLeftList(Expression::typeOf(var_field!((*call).exp, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone()), Type::arrayDims(Expression::typeOf(Util::tuple22(iter.clone()))));
            outCall = Arc::new(NFCall::TYPED_ARRAY_CONSTRUCTOR { ty: ty.clone(), var: var_field!((*call).var, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), purity: var_field!((*call).purity, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), exp: var_field!((*call).exp, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), iters: list![iter.clone()] });
            for mut i in &*iters.clone() {
                let mut i = i.clone();
                ty = Type::liftArrayLeftList(ty.clone(), Type::arrayDims(Expression::typeOf(Util::tuple22(i.clone()))));
                outCall = Arc::new(NFCall::TYPED_ARRAY_CONSTRUCTOR { ty: ty.clone(), var: var_field!((*call).var, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), purity: var_field!((*call).purity, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), exp: Arc::new(Expression::NFExpression::CALL { call: outCall.clone() }), iters: list![i.clone()] });
            }
            outCall.clone()
        },
        Deref @ TYPED_REDUCTION { iters, .. } if ((iters.clone().len() as i32) > 1) => {
            let mut iter: (Arc<InstNode::InstNode>, Arc<Expression::NFExpression>);
            let mut iters = (*iters).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(iters.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            iter = __pa0.clone();
            iters = __pa1.clone();
            outCall = makeTypedReduction(var_field!((*call).r#fn, NFCall::TYPED_REDUCTION).clone(), var_field!((*call).ty, NFCall::TYPED_REDUCTION).clone(), var_field!((*call).var, NFCall::TYPED_REDUCTION).clone(), var_field!((*call).purity, NFCall::TYPED_REDUCTION).clone(), var_field!((*call).exp, NFCall::TYPED_REDUCTION).clone(), list![iter.clone()], Absyn::dummyInfo.clone())?;
            for mut i in &*iters.clone() {
                let mut i = i.clone();
                outCall = makeTypedReduction(var_field!((*call).r#fn, NFCall::TYPED_REDUCTION).clone(), var_field!((*call).ty, NFCall::TYPED_REDUCTION).clone(), var_field!((*call).var, NFCall::TYPED_REDUCTION).clone(), var_field!((*call).purity, NFCall::TYPED_REDUCTION).clone(), Arc::new(Expression::NFExpression::CALL { call: outCall.clone() }), list![i.clone()], Absyn::dummyInfo.clone())?;
            }
            outCall.clone()
        },
        _ => {
            call.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outCall)
}

pub fn isVectorizeable(mut call: Arc<NFCall>) -> bool {
    let mut isVect: bool = false;
    isVect = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ TYPED_CALL { r#fn: Deref @ Function::FUNCTION { path: Deref @ Absyn::Path::IDENT { name }, .. }, .. } => {
            (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "der" => false,
        Deref @ "pre" => false,
        Deref @ "previous" => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        _ => {
            true
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isVect
}

pub fn retype(mut call: Arc<NFCall>) -> Arc<NFCall> {
    let mut call: Arc<NFCall> = call;
    let () = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ TYPED_ARRAY_CONSTRUCTOR { .. } => {
            let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
            dims = metamodelica::nil();
            for mut i in &*var_field!((*call).iters, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone().reverse() {
                let mut i = i.clone();
                dims = listAppend(Type::arrayDims(Expression::typeOf(Util::tuple22(i.clone()))), dims.clone());
            }
            assign_variant_field!(call => NFCall::TYPED_ARRAY_CONSTRUCTOR; ty = Type::liftArrayLeftList(Type::arrayElementType(var_field!((*call).ty, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone()), dims.clone()));
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    call
}

pub fn typeCast(mut callExp: Arc<Expression::NFExpression>, mut ty: Arc<Type::NFType>) -> Result<Arc<Expression::NFExpression>> {
    let mut callExp: Arc<Expression::NFExpression> = callExp;
    let mut call: Arc<NFCall>;
    let mut cast_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let __pa0 = ::match_deref::match_deref! { match &(callExp.clone()) {
        Deref @ Expression::CALL { call: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    call = __pa0.clone();
    callExp = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ TYPED_CALL { .. } if (Function::isBuiltin(var_field!((*call).r#fn, NFCall::TYPED_CALL).clone())) => {
            cast_ty = Type::setArrayElementType(var_field!((*call).ty, NFCall::TYPED_CALL).clone(), ty.clone());
            (::match_deref::match_deref! { match &(AbsynUtil::pathFirstIdent(Function::name(var_field!((*call).r#fn, NFCall::TYPED_CALL).clone()))?) {
        Deref @ "fill" => {
            assign_variant_field!(call => NFCall::TYPED_CALL;
                arguments = cons(Expression::typeCast(listHead(var_field!((*call).arguments, NFCall::TYPED_CALL).clone())?, ty.clone())?, listRest(var_field!((*call).arguments, NFCall::TYPED_CALL).clone())?),
                ty = cast_ty.clone()
            );
            Arc::new(Expression::NFExpression::CALL { call: call.clone() })
        },
        Deref @ "diagonal" => {
            assign_variant_field!(call => NFCall::TYPED_CALL;
                arguments = list![Expression::typeCast(listHead(var_field!((*call).arguments, NFCall::TYPED_CALL).clone())?, ty.clone())?],
                ty = cast_ty.clone()
            );
            Arc::new(Expression::NFExpression::CALL { call: call.clone() })
        },
        Deref @ "DynamicSelect" => {
            assign_variant_field!(call => NFCall::TYPED_CALL; arguments = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut arg in (var_field!((*call).arguments, NFCall::TYPED_CALL).clone()).into_iter().cloned() {
            let __x = Expression::typeCast(arg.clone(), ty.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            Arc::new(Expression::NFExpression::CALL { call: call.clone() })
        },
        _ => Arc::new(Expression::NFExpression::CAST { ty: cast_ty.clone(), exp: callExp.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        _ => Arc::new(Expression::NFExpression::CAST { ty: Type::setArrayElementType(typeOf(call.clone()), ty.clone()), exp: callExp.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(callExp)
}

pub fn containsExp(mut call: Arc<NFCall>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>) -> Result<bool> {
    pub type ContainsPred = fn(Arc<Expression::NFExpression>) -> Result<bool>;

    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ UNTYPED_CALL { .. } => {
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            res = Expression::listContains(var_field!((*call).arguments, NFCall::UNTYPED_CALL).clone(), func.clone())?;
            if !(res.clone()) {
                for mut arg in &*var_field!((*call).named_args, NFCall::UNTYPED_CALL).clone() {
                    let mut arg = arg.clone();
                    (_, e) = arg.clone();
                    if Expression::contains(e.clone(), func.clone())? {
                        res = true;
                        break;
                    }
                }
            }
            res.clone()
        },
        Deref @ ARG_TYPED_CALL { .. } => {
            for mut arg in &*var_field!((*call).positional_args, NFCall::ARG_TYPED_CALL).clone() {
                let mut arg = arg.clone();
                if Expression::contains(arg.value.clone(), func.clone())? {
                    res = true;
                    return Ok(res);
                }
            }
            for mut arg in &*var_field!((*call).named_args, NFCall::ARG_TYPED_CALL).clone() {
                let mut arg = arg.clone();
                if Expression::contains(arg.value.clone(), func.clone())? {
                    res = true;
                    return Ok(res);
                }
            }
            false
        },
        Deref @ TYPED_CALL { .. } => {
            Expression::listContains(var_field!((*call).arguments, NFCall::TYPED_CALL).clone(), func.clone())?
        },
        Deref @ UNTYPED_ARRAY_CONSTRUCTOR { .. } => {
            Expression::contains(var_field!((*call).exp, NFCall::UNTYPED_ARRAY_CONSTRUCTOR).clone(), func.clone())?
        },
        Deref @ TYPED_ARRAY_CONSTRUCTOR { .. } => {
            Expression::contains(var_field!((*call).exp, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), func.clone())?
        },
        Deref @ UNTYPED_REDUCTION { .. } => {
            Expression::contains(var_field!((*call).exp, NFCall::UNTYPED_REDUCTION).clone(), func.clone())?
        },
        Deref @ TYPED_REDUCTION { .. } => {
            Expression::contains(var_field!((*call).exp, NFCall::TYPED_REDUCTION).clone(), func.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(res)
}

pub fn containsExpShallow(mut call: Arc<NFCall>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>) -> Result<bool> {
    pub type ContainsPred = fn(Arc<Expression::NFExpression>) -> Result<bool>;

    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ UNTYPED_CALL { .. } => {
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            res = List::any(var_field!((*call).arguments, NFCall::UNTYPED_CALL).clone(), func.clone());
            if !(res.clone()) {
                for mut arg in &*var_field!((*call).named_args, NFCall::UNTYPED_CALL).clone() {
                    let mut arg = arg.clone();
                    (_, e) = arg.clone();
                    if func(e.clone())? {
                        res = true;
                        break;
                    }
                }
            }
            res.clone()
        },
        Deref @ ARG_TYPED_CALL { .. } => {
            for mut arg in &*var_field!((*call).positional_args, NFCall::ARG_TYPED_CALL).clone() {
                let mut arg = arg.clone();
                if func(arg.value.clone())? {
                    res = true;
                    return Ok(res);
                }
            }
            for mut arg in &*var_field!((*call).named_args, NFCall::ARG_TYPED_CALL).clone() {
                let mut arg = arg.clone();
                if func(arg.value.clone())? {
                    res = true;
                    return Ok(res);
                }
            }
            false
        },
        Deref @ TYPED_CALL { .. } => {
            List::any(var_field!((*call).arguments, NFCall::TYPED_CALL).clone(), func.clone())
        },
        Deref @ UNTYPED_ARRAY_CONSTRUCTOR { .. } => {
            func(var_field!((*call).exp, NFCall::UNTYPED_ARRAY_CONSTRUCTOR).clone())?
        },
        Deref @ TYPED_ARRAY_CONSTRUCTOR { .. } => {
            func(var_field!((*call).exp, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone())?
        },
        Deref @ UNTYPED_REDUCTION { .. } => {
            func(var_field!((*call).exp, NFCall::UNTYPED_REDUCTION).clone())?
        },
        Deref @ TYPED_REDUCTION { .. } => {
            func(var_field!((*call).exp, NFCall::TYPED_REDUCTION).clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(res)
}

pub fn applyExp(mut call: Arc<NFCall>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>) -> Result<()> {
    pub type ApplyFunc = fn(Arc<Expression::NFExpression>) -> Result<()>;

    let () = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ UNTYPED_CALL { .. } => {
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            Expression::applyList(var_field!((*call).arguments, NFCall::UNTYPED_CALL).clone(), func.clone())?;
            for mut arg in &*var_field!((*call).named_args, NFCall::UNTYPED_CALL).clone() {
                let mut arg = arg.clone();
                (_, e) = arg.clone();
                Expression::apply(e.clone(), func.clone())?;
            }
            ()
        },
        Deref @ ARG_TYPED_CALL { .. } => {
            for mut arg in &*var_field!((*call).positional_args, NFCall::ARG_TYPED_CALL).clone() {
                let mut arg = arg.clone();
                Expression::apply(arg.value.clone(), func.clone())?;
            }
            for mut arg in &*var_field!((*call).named_args, NFCall::ARG_TYPED_CALL).clone() {
                let mut arg = arg.clone();
                Expression::apply(arg.value.clone(), func.clone())?;
            }
            ()
        },
        Deref @ TYPED_CALL { .. } => {
            Expression::applyList(var_field!((*call).arguments, NFCall::TYPED_CALL).clone(), func.clone())?;
            ()
        },
        Deref @ UNTYPED_ARRAY_CONSTRUCTOR { .. } => {
            Expression::apply(var_field!((*call).exp, NFCall::UNTYPED_ARRAY_CONSTRUCTOR).clone(), func.clone())?;
            for mut i in &*var_field!((*call).iters, NFCall::UNTYPED_ARRAY_CONSTRUCTOR).clone() {
                let mut i = i.clone();
                Expression::apply(Util::tuple22(i.clone()), func.clone())?;
            }
            ()
        },
        Deref @ TYPED_ARRAY_CONSTRUCTOR { .. } => {
            Expression::apply(var_field!((*call).exp, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), func.clone())?;
            for mut i in &*var_field!((*call).iters, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone() {
                let mut i = i.clone();
                Expression::apply(Util::tuple22(i.clone()), func.clone())?;
            }
            ()
        },
        Deref @ UNTYPED_REDUCTION { .. } => {
            Expression::apply(var_field!((*call).exp, NFCall::UNTYPED_REDUCTION).clone(), func.clone())?;
            for mut i in &*var_field!((*call).iters, NFCall::UNTYPED_REDUCTION).clone() {
                let mut i = i.clone();
                Expression::apply(Util::tuple22(i.clone()), func.clone())?;
            }
            ()
        },
        Deref @ TYPED_REDUCTION { .. } => {
            Expression::apply(var_field!((*call).exp, NFCall::TYPED_REDUCTION).clone(), func.clone())?;
            for mut i in &*var_field!((*call).iters, NFCall::TYPED_REDUCTION).clone() {
                let mut i = i.clone();
                Expression::apply(Util::tuple22(i.clone()), func.clone())?;
            }
            Expression::applyOpt(var_field!((*call).defaultExp, NFCall::TYPED_REDUCTION).clone(), func.clone())?;
            Expression::applyOpt(Util::tuple31(var_field!((*call).foldExp, NFCall::TYPED_REDUCTION).clone()), func.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn applyExpShallow(mut call: Arc<NFCall>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>) -> Result<()> {
    pub type ApplyFunc = fn(Arc<Expression::NFExpression>) -> Result<()>;

    let () = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ UNTYPED_CALL { .. } => {
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            Expression::applyListShallow(var_field!((*call).arguments, NFCall::UNTYPED_CALL).clone(), func.clone());
            for mut arg in &*var_field!((*call).named_args, NFCall::UNTYPED_CALL).clone() {
                let mut arg = arg.clone();
                (_, e) = arg.clone();
                func(e.clone())?;
            }
            ()
        },
        Deref @ ARG_TYPED_CALL { .. } => {
            for mut arg in &*var_field!((*call).positional_args, NFCall::ARG_TYPED_CALL).clone() {
                let mut arg = arg.clone();
                func(arg.value.clone())?;
            }
            for mut arg in &*var_field!((*call).named_args, NFCall::ARG_TYPED_CALL).clone() {
                let mut arg = arg.clone();
                func(arg.value.clone())?;
            }
            ()
        },
        Deref @ TYPED_CALL { .. } => {
            Expression::applyListShallow(var_field!((*call).arguments, NFCall::TYPED_CALL).clone(), func.clone());
            ()
        },
        Deref @ UNTYPED_ARRAY_CONSTRUCTOR { .. } => {
            func(var_field!((*call).exp, NFCall::UNTYPED_ARRAY_CONSTRUCTOR).clone())?;
            for mut i in &*var_field!((*call).iters, NFCall::UNTYPED_ARRAY_CONSTRUCTOR).clone() {
                let mut i = i.clone();
                func(Util::tuple22(i.clone()))?;
            }
            ()
        },
        Deref @ TYPED_ARRAY_CONSTRUCTOR { .. } => {
            func(var_field!((*call).exp, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone())?;
            for mut i in &*var_field!((*call).iters, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone() {
                let mut i = i.clone();
                func(Util::tuple22(i.clone()))?;
            }
            ()
        },
        Deref @ UNTYPED_REDUCTION { .. } => {
            func(var_field!((*call).exp, NFCall::UNTYPED_REDUCTION).clone())?;
            for mut i in &*var_field!((*call).iters, NFCall::UNTYPED_REDUCTION).clone() {
                let mut i = i.clone();
                func(Util::tuple22(i.clone()))?;
            }
            ()
        },
        Deref @ TYPED_REDUCTION { .. } => {
            func(var_field!((*call).exp, NFCall::TYPED_REDUCTION).clone())?;
            for mut i in &*var_field!((*call).iters, NFCall::TYPED_REDUCTION).clone() {
                let mut i = i.clone();
                func(Util::tuple22(i.clone()))?;
            }
            Expression::applyShallowOpt(var_field!((*call).defaultExp, NFCall::TYPED_REDUCTION).clone(), func.clone())?;
            Expression::applyShallowOpt(Util::tuple31(var_field!((*call).foldExp, NFCall::TYPED_REDUCTION).clone()), func.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn foldExp<ArgT: Clone + 'static>(mut call: Arc<NFCall>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>, mut foldArg: ArgT) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone> = fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT>;

    let mut foldArg: ArgT = foldArg;
    let () = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ UNTYPED_CALL { .. } => {
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            foldArg = Expression::foldList(var_field!((*call).arguments, NFCall::UNTYPED_CALL).clone(), func.clone(), foldArg.clone())?;
            for mut arg in &*var_field!((*call).named_args, NFCall::UNTYPED_CALL).clone() {
                let mut arg = arg.clone();
                (_, e) = arg.clone();
                foldArg = Expression::fold(e.clone(), func.clone(), foldArg.clone())?;
            }
            ()
        },
        Deref @ ARG_TYPED_CALL { .. } => {
            for mut arg in &*var_field!((*call).positional_args, NFCall::ARG_TYPED_CALL).clone() {
                let mut arg = arg.clone();
                foldArg = Expression::fold(arg.value.clone(), func.clone(), foldArg.clone())?;
            }
            for mut arg in &*var_field!((*call).named_args, NFCall::ARG_TYPED_CALL).clone() {
                let mut arg = arg.clone();
                foldArg = Expression::fold(arg.value.clone(), func.clone(), foldArg.clone())?;
            }
            ()
        },
        Deref @ TYPED_CALL { .. } => {
            foldArg = Expression::foldList(var_field!((*call).arguments, NFCall::TYPED_CALL).clone(), func.clone(), foldArg.clone())?;
            ()
        },
        Deref @ UNTYPED_ARRAY_CONSTRUCTOR { .. } => {
            foldArg = Expression::fold(var_field!((*call).exp, NFCall::UNTYPED_ARRAY_CONSTRUCTOR).clone(), func.clone(), foldArg.clone())?;
            for mut i in &*var_field!((*call).iters, NFCall::UNTYPED_ARRAY_CONSTRUCTOR).clone() {
                let mut i = i.clone();
                foldArg = Expression::fold(Util::tuple22(i.clone()), func.clone(), foldArg.clone())?;
            }
            ()
        },
        Deref @ TYPED_ARRAY_CONSTRUCTOR { .. } => {
            foldArg = Expression::fold(var_field!((*call).exp, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), func.clone(), foldArg.clone())?;
            for mut i in &*var_field!((*call).iters, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone() {
                let mut i = i.clone();
                foldArg = Expression::fold(Util::tuple22(i.clone()), func.clone(), foldArg.clone())?;
            }
            ()
        },
        Deref @ UNTYPED_REDUCTION { .. } => {
            foldArg = Expression::fold(var_field!((*call).exp, NFCall::UNTYPED_REDUCTION).clone(), func.clone(), foldArg.clone())?;
            for mut i in &*var_field!((*call).iters, NFCall::UNTYPED_REDUCTION).clone() {
                let mut i = i.clone();
                foldArg = Expression::fold(Util::tuple22(i.clone()), func.clone(), foldArg.clone())?;
            }
            ()
        },
        Deref @ TYPED_REDUCTION { .. } => {
            foldArg = Expression::fold(var_field!((*call).exp, NFCall::TYPED_REDUCTION).clone(), func.clone(), foldArg.clone())?;
            for mut i in &*var_field!((*call).iters, NFCall::TYPED_REDUCTION).clone() {
                let mut i = i.clone();
                foldArg = Expression::fold(Util::tuple22(i.clone()), func.clone(), foldArg.clone())?;
            }
            foldArg = Expression::foldOpt(var_field!((*call).defaultExp, NFCall::TYPED_REDUCTION).clone(), func.clone(), foldArg.clone());
            foldArg = Expression::foldOpt(Util::tuple31(var_field!((*call).foldExp, NFCall::TYPED_REDUCTION).clone()), func.clone(), foldArg.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(foldArg)
}

pub fn mapExp(mut call: Arc<NFCall>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<NFCall>> {
    pub type MapFunc = fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>;

    let mut outCall: Arc<NFCall>;
    outCall = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ UNTYPED_CALL { .. } => {
            let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            let mut nargs: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>> = metamodelica::nil();
            let mut s: ArcStr = arcstr::literal!("");
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            args = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut arg in (var_field!((*call).arguments, NFCall::UNTYPED_CALL).clone()).into_iter().cloned() {
            let __x = Expression::map(arg.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            nargs = metamodelica::nil();
            for mut arg in &*var_field!((*call).named_args, NFCall::UNTYPED_CALL).clone() {
                let mut arg = arg.clone();
                (s, e) = arg.clone();
                e = Expression::map(e.clone(), func.clone())?;
                nargs = cons((s.clone(), e.clone()), nargs.clone());
            }
            Arc::new(NFCall::UNTYPED_CALL { r#ref: var_field!((*call).r#ref, NFCall::UNTYPED_CALL).clone(), arguments: args.clone(), named_args: nargs.clone().reverse(), call_scope: var_field!((*call).call_scope, NFCall::UNTYPED_CALL).clone() })
        },
        Deref @ ARG_TYPED_CALL { .. } => {
            let mut targs: Arc<metamodelica::List<Arc<TypedArg>>> = metamodelica::nil();
            let mut tnargs: Arc<metamodelica::List<Arc<TypedArg>>> = metamodelica::nil();
            targs = metamodelica::nil();
            tnargs = metamodelica::nil();
            for mut arg in &*var_field!((*call).positional_args, NFCall::ARG_TYPED_CALL).clone() {
                let mut arg = arg.clone();
                assign_field!(arg.value = Expression::map(arg.value.clone(), func.clone())?);
                targs = cons(arg.clone(), targs.clone());
            }
            for mut arg in &*var_field!((*call).named_args, NFCall::ARG_TYPED_CALL).clone() {
                let mut arg = arg.clone();
                assign_field!(arg.value = Expression::map(arg.value.clone(), func.clone())?);
                tnargs = cons(arg.clone(), tnargs.clone());
            }
            Arc::new(NFCall::ARG_TYPED_CALL { r#ref: var_field!((*call).r#ref, NFCall::ARG_TYPED_CALL).clone(), positional_args: targs.clone().reverse(), named_args: tnargs.clone().reverse(), call_scope: var_field!((*call).call_scope, NFCall::ARG_TYPED_CALL).clone() })
        },
        Deref @ TYPED_CALL { .. } => {
            let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            args = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut arg in (var_field!((*call).arguments, NFCall::TYPED_CALL).clone()).into_iter().cloned() {
            let __x = Expression::map(arg.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            Arc::new(NFCall::TYPED_CALL { r#fn: var_field!((*call).r#fn, NFCall::TYPED_CALL).clone(), ty: var_field!((*call).ty, NFCall::TYPED_CALL).clone(), var: var_field!((*call).var, NFCall::TYPED_CALL).clone(), purity: var_field!((*call).purity, NFCall::TYPED_CALL).clone(), arguments: args.clone(), attributes: var_field!((*call).attributes, NFCall::TYPED_CALL).clone() })
        },
        Deref @ UNTYPED_ARRAY_CONSTRUCTOR { .. } => {
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
            e = Expression::map(var_field!((*call).exp, NFCall::UNTYPED_ARRAY_CONSTRUCTOR).clone(), func.clone())?;
            iters = mapIteratorsExp(var_field!((*call).iters, NFCall::UNTYPED_ARRAY_CONSTRUCTOR).clone(), func.clone())?;
            Arc::new(NFCall::UNTYPED_ARRAY_CONSTRUCTOR { exp: e.clone(), iters: iters.clone() })
        },
        Deref @ TYPED_ARRAY_CONSTRUCTOR { .. } => {
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
            e = Expression::map(var_field!((*call).exp, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), func.clone())?;
            iters = mapIteratorsExp(var_field!((*call).iters, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), func.clone())?;
            Arc::new(NFCall::TYPED_ARRAY_CONSTRUCTOR { ty: var_field!((*call).ty, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), var: var_field!((*call).var, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), purity: var_field!((*call).purity, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), exp: e.clone(), iters: iters.clone() })
        },
        Deref @ UNTYPED_REDUCTION { .. } => {
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
            e = Expression::map(var_field!((*call).exp, NFCall::UNTYPED_REDUCTION).clone(), func.clone())?;
            iters = mapIteratorsExp(var_field!((*call).iters, NFCall::UNTYPED_REDUCTION).clone(), func.clone())?;
            Arc::new(NFCall::UNTYPED_REDUCTION { r#ref: var_field!((*call).r#ref, NFCall::UNTYPED_REDUCTION).clone(), exp: e.clone(), iters: iters.clone() })
        },
        Deref @ TYPED_REDUCTION { .. } => {
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
            let mut default_exp: Option<Arc<Expression::NFExpression>> = None;
            let mut fold_exp: (Option<Arc<Expression::NFExpression>>, ArcStr, ArcStr);
            e = Expression::map(var_field!((*call).exp, NFCall::TYPED_REDUCTION).clone(), func.clone())?;
            iters = mapIteratorsExp(var_field!((*call).iters, NFCall::TYPED_REDUCTION).clone(), func.clone())?;
            default_exp = Util::applyOption(var_field!((*call).defaultExp, NFCall::TYPED_REDUCTION).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) }));
            fold_exp = Util::applyTuple31(var_field!((*call).foldExp, NFCall::TYPED_REDUCTION).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Expression::mapOpt(__pe_a0, __pe_b1.clone()) }));
            Arc::new(NFCall::TYPED_REDUCTION { r#fn: var_field!((*call).r#fn, NFCall::TYPED_REDUCTION).clone(), ty: var_field!((*call).ty, NFCall::TYPED_REDUCTION).clone(), var: var_field!((*call).var, NFCall::TYPED_REDUCTION).clone(), purity: var_field!((*call).purity, NFCall::TYPED_REDUCTION).clone(), exp: e.clone(), iters: iters.clone(), defaultExp: default_exp.clone(), foldExp: fold_exp.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outCall)
}

pub fn mapIteratorsExp(mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>>> {
    pub type MapFunc = fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>;

    let mut outIters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut new_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    for mut i in &*iters.clone() {
        let mut i = i.clone();
        (node, exp) = i.clone();
        new_exp = Expression::map(exp.clone(), func.clone())?;
        outIters = cons(if (referenceEq(&new_exp.clone(),&exp.clone())) {i.clone()} else {(node.clone(), new_exp.clone())}, outIters.clone());
    }
    outIters = outIters.clone().reverse();
    Ok(outIters)
}

pub fn mapExpShallow(mut call: Arc<NFCall>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<NFCall>> {
    pub type MapFunc = fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>;

    let mut outCall: Arc<NFCall>;
    outCall = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ UNTYPED_CALL { .. } => {
            let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            let mut nargs: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>> = metamodelica::nil();
            let mut s: ArcStr = arcstr::literal!("");
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            args = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut arg in (var_field!((*call).arguments, NFCall::UNTYPED_CALL).clone()).into_iter().cloned() {
            let __x = func(arg.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            nargs = metamodelica::nil();
            for mut arg in &*var_field!((*call).named_args, NFCall::UNTYPED_CALL).clone() {
                let mut arg = arg.clone();
                (s, e) = arg.clone();
                e = func(e.clone())?;
                nargs = cons((s.clone(), e.clone()), nargs.clone());
            }
            Arc::new(NFCall::UNTYPED_CALL { r#ref: var_field!((*call).r#ref, NFCall::UNTYPED_CALL).clone(), arguments: args.clone(), named_args: nargs.clone().reverse(), call_scope: var_field!((*call).call_scope, NFCall::UNTYPED_CALL).clone() })
        },
        Deref @ ARG_TYPED_CALL { .. } => {
            let mut targs: Arc<metamodelica::List<Arc<TypedArg>>> = metamodelica::nil();
            let mut tnargs: Arc<metamodelica::List<Arc<TypedArg>>> = metamodelica::nil();
            targs = metamodelica::nil();
            tnargs = metamodelica::nil();
            for mut arg in &*var_field!((*call).positional_args, NFCall::ARG_TYPED_CALL).clone() {
                let mut arg = arg.clone();
                assign_field!(arg.value = func(arg.value.clone())?);
                targs = cons(arg.clone(), targs.clone());
            }
            for mut arg in &*var_field!((*call).named_args, NFCall::ARG_TYPED_CALL).clone() {
                let mut arg = arg.clone();
                assign_field!(arg.value = func(arg.value.clone())?);
                tnargs = cons(arg.clone(), tnargs.clone());
            }
            Arc::new(NFCall::ARG_TYPED_CALL { r#ref: var_field!((*call).r#ref, NFCall::ARG_TYPED_CALL).clone(), positional_args: targs.clone().reverse(), named_args: tnargs.clone().reverse(), call_scope: var_field!((*call).call_scope, NFCall::ARG_TYPED_CALL).clone() })
        },
        Deref @ TYPED_CALL { .. } => {
            let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            args = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut arg in (var_field!((*call).arguments, NFCall::TYPED_CALL).clone()).into_iter().cloned() {
            let __x = func(arg.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            Arc::new(NFCall::TYPED_CALL { r#fn: var_field!((*call).r#fn, NFCall::TYPED_CALL).clone(), ty: var_field!((*call).ty, NFCall::TYPED_CALL).clone(), var: var_field!((*call).var, NFCall::TYPED_CALL).clone(), purity: var_field!((*call).purity, NFCall::TYPED_CALL).clone(), arguments: args.clone(), attributes: var_field!((*call).attributes, NFCall::TYPED_CALL).clone() })
        },
        Deref @ UNTYPED_ARRAY_CONSTRUCTOR { .. } => {
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
            e = func(var_field!((*call).exp, NFCall::UNTYPED_ARRAY_CONSTRUCTOR).clone())?;
            iters = mapIteratorsExpShallow(var_field!((*call).iters, NFCall::UNTYPED_ARRAY_CONSTRUCTOR).clone(), func.clone());
            Arc::new(NFCall::UNTYPED_ARRAY_CONSTRUCTOR { exp: e.clone(), iters: iters.clone() })
        },
        Deref @ TYPED_ARRAY_CONSTRUCTOR { .. } => {
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
            e = func(var_field!((*call).exp, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone())?;
            iters = mapIteratorsExpShallow(var_field!((*call).iters, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), func.clone());
            Arc::new(NFCall::TYPED_ARRAY_CONSTRUCTOR { ty: var_field!((*call).ty, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), var: var_field!((*call).var, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), purity: var_field!((*call).purity, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), exp: e.clone(), iters: iters.clone() })
        },
        Deref @ UNTYPED_REDUCTION { .. } => {
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
            e = func(var_field!((*call).exp, NFCall::UNTYPED_REDUCTION).clone())?;
            iters = mapIteratorsExpShallow(var_field!((*call).iters, NFCall::UNTYPED_REDUCTION).clone(), func.clone());
            Arc::new(NFCall::UNTYPED_REDUCTION { r#ref: var_field!((*call).r#ref, NFCall::UNTYPED_REDUCTION).clone(), exp: e.clone(), iters: iters.clone() })
        },
        Deref @ TYPED_REDUCTION { .. } => {
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
            let mut default_exp: Option<Arc<Expression::NFExpression>> = None;
            let mut fold_exp: (Option<Arc<Expression::NFExpression>>, ArcStr, ArcStr);
            e = func(var_field!((*call).exp, NFCall::TYPED_REDUCTION).clone())?;
            iters = mapIteratorsExpShallow(var_field!((*call).iters, NFCall::TYPED_REDUCTION).clone(), func.clone());
            default_exp = Expression::mapShallowOpt(var_field!((*call).defaultExp, NFCall::TYPED_REDUCTION).clone(), func.clone());
            fold_exp = Util::applyTuple31(var_field!((*call).foldExp, NFCall::TYPED_REDUCTION).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Ok(Expression::mapShallowOpt(__pe_a0, __pe_b1.clone())) }));
            Arc::new(NFCall::TYPED_REDUCTION { r#fn: var_field!((*call).r#fn, NFCall::TYPED_REDUCTION).clone(), ty: var_field!((*call).ty, NFCall::TYPED_REDUCTION).clone(), var: var_field!((*call).var, NFCall::TYPED_REDUCTION).clone(), purity: var_field!((*call).purity, NFCall::TYPED_REDUCTION).clone(), exp: e.clone(), iters: iters.clone(), defaultExp: default_exp.clone(), foldExp: fold_exp.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outCall)
}

pub fn mapIteratorsExpShallow(mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> {
    pub type MapFunc = fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>;

    let mut outIters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut new_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    for mut i in &*iters.clone() {
        let mut i = i.clone();
        (node, exp) = i.clone();
        new_exp = func(exp.clone()).unwrap();
        outIters = cons(if (referenceEq(&new_exp.clone(),&exp.clone())) {i.clone()} else {(node.clone(), new_exp.clone())}, outIters.clone());
    }
    outIters = outIters.clone().reverse();
    outIters
}

pub fn mapFoldExp<ArgT: Clone + 'static>(mut call: Arc<NFCall>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<(Arc<Expression::NFExpression>, ArgT)> + 'static>, mut foldArg: ArgT) -> Result<(Arc<NFCall>, ArgT)> {
    pub type MapFunc<ArgT: Clone> = fn(Arc<Expression::NFExpression>, ArgT) -> Result<(Arc<Expression::NFExpression>, ArgT)>;

    let mut outCall: Arc<NFCall>;
    let mut foldArg: ArgT = foldArg;
    outCall = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ UNTYPED_CALL { .. } => {
            let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            let mut nargs: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>> = metamodelica::nil();
            let mut s: ArcStr = arcstr::literal!("");
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            (args, foldArg) = List::map1Fold(var_field!((*call).arguments, NFCall::UNTYPED_CALL).clone(), Arc::new(Expression::mapFold), func.clone(), foldArg.clone());
            nargs = metamodelica::nil();
            for mut arg in &*var_field!((*call).named_args, NFCall::UNTYPED_CALL).clone() {
                let mut arg = arg.clone();
                (s, e) = arg.clone();
                (e, foldArg) = Expression::mapFold(e.clone(), func.clone(), foldArg.clone())?;
                nargs = cons((s.clone(), e.clone()), nargs.clone());
            }
            Arc::new(NFCall::UNTYPED_CALL { r#ref: var_field!((*call).r#ref, NFCall::UNTYPED_CALL).clone(), arguments: args.clone(), named_args: nargs.clone().reverse(), call_scope: var_field!((*call).call_scope, NFCall::UNTYPED_CALL).clone() })
        },
        Deref @ ARG_TYPED_CALL { .. } => {
            let mut targs: Arc<metamodelica::List<Arc<TypedArg>>> = metamodelica::nil();
            let mut tnargs: Arc<metamodelica::List<Arc<TypedArg>>> = metamodelica::nil();
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            targs = metamodelica::nil();
            tnargs = metamodelica::nil();
            for mut arg in &*var_field!((*call).positional_args, NFCall::ARG_TYPED_CALL).clone() {
                let mut arg = arg.clone();
                (e, foldArg) = Expression::mapFold(arg.value.clone(), func.clone(), foldArg.clone())?;
                assign_field!(arg.value = e.clone());
                targs = cons(arg.clone(), targs.clone());
            }
            for mut arg in &*var_field!((*call).named_args, NFCall::ARG_TYPED_CALL).clone() {
                let mut arg = arg.clone();
                (e, foldArg) = Expression::mapFold(arg.value.clone(), func.clone(), foldArg.clone())?;
                assign_field!(arg.value = e.clone());
                targs = cons(arg.clone(), targs.clone());
            }
            Arc::new(NFCall::ARG_TYPED_CALL { r#ref: var_field!((*call).r#ref, NFCall::ARG_TYPED_CALL).clone(), positional_args: targs.clone().reverse(), named_args: tnargs.clone().reverse(), call_scope: var_field!((*call).call_scope, NFCall::ARG_TYPED_CALL).clone() })
        },
        Deref @ TYPED_CALL { .. } => {
            let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            (args, foldArg) = List::map1Fold(var_field!((*call).arguments, NFCall::TYPED_CALL).clone(), Arc::new(Expression::mapFold), func.clone(), foldArg.clone());
            Arc::new(NFCall::TYPED_CALL { r#fn: var_field!((*call).r#fn, NFCall::TYPED_CALL).clone(), ty: var_field!((*call).ty, NFCall::TYPED_CALL).clone(), var: var_field!((*call).var, NFCall::TYPED_CALL).clone(), purity: var_field!((*call).purity, NFCall::TYPED_CALL).clone(), arguments: args.clone(), attributes: var_field!((*call).attributes, NFCall::TYPED_CALL).clone() })
        },
        Deref @ UNTYPED_ARRAY_CONSTRUCTOR { .. } => {
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
            (e, foldArg) = Expression::mapFold(var_field!((*call).exp, NFCall::UNTYPED_ARRAY_CONSTRUCTOR).clone(), func.clone(), foldArg.clone())?;
            (iters, foldArg) = mapFoldIteratorsExp(var_field!((*call).iters, NFCall::UNTYPED_ARRAY_CONSTRUCTOR).clone(), func.clone(), foldArg.clone())?;
            Arc::new(NFCall::UNTYPED_ARRAY_CONSTRUCTOR { exp: e.clone(), iters: iters.clone() })
        },
        Deref @ TYPED_ARRAY_CONSTRUCTOR { .. } => {
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
            (e, foldArg) = Expression::mapFold(var_field!((*call).exp, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), func.clone(), foldArg.clone())?;
            (iters, foldArg) = mapFoldIteratorsExp(var_field!((*call).iters, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), func.clone(), foldArg.clone())?;
            Arc::new(NFCall::TYPED_ARRAY_CONSTRUCTOR { ty: var_field!((*call).ty, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), var: var_field!((*call).var, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), purity: var_field!((*call).purity, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), exp: e.clone(), iters: iters.clone() })
        },
        Deref @ UNTYPED_REDUCTION { .. } => {
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
            (e, foldArg) = Expression::mapFold(var_field!((*call).exp, NFCall::UNTYPED_REDUCTION).clone(), func.clone(), foldArg.clone())?;
            (iters, foldArg) = mapFoldIteratorsExp(var_field!((*call).iters, NFCall::UNTYPED_REDUCTION).clone(), func.clone(), foldArg.clone())?;
            Arc::new(NFCall::UNTYPED_REDUCTION { r#ref: var_field!((*call).r#ref, NFCall::UNTYPED_REDUCTION).clone(), exp: e.clone(), iters: iters.clone() })
        },
        Deref @ TYPED_REDUCTION { .. } => {
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
            let mut default_exp: Option<Arc<Expression::NFExpression>> = None;
            let mut fold_exp: (Option<Arc<Expression::NFExpression>>, ArcStr, ArcStr);
            let mut oe: Option<Arc<Expression::NFExpression>> = None;
            (e, foldArg) = Expression::mapFold(var_field!((*call).exp, NFCall::TYPED_REDUCTION).clone(), func.clone(), foldArg.clone())?;
            (iters, foldArg) = mapFoldIteratorsExp(var_field!((*call).iters, NFCall::TYPED_REDUCTION).clone(), func.clone(), foldArg.clone())?;
            (default_exp, foldArg) = Expression::mapFoldOpt(var_field!((*call).defaultExp, NFCall::TYPED_REDUCTION).clone(), func.clone(), foldArg.clone())?;
            oe = Util::tuple31(var_field!((*call).foldExp, NFCall::TYPED_REDUCTION).clone());
            if isSome(oe.clone()) {
                (oe, foldArg) = Expression::mapFoldOpt(oe.clone(), func.clone(), foldArg.clone())?;
                fold_exp = Util::applyTuple31(var_field!((*call).foldExp, NFCall::TYPED_REDUCTION).clone(), Arc::new({ let __pe_b1 = oe.clone(); move |__pe_a0| Ok(Util::replace(__pe_a0, __pe_b1.clone())) }));
            } else {
                fold_exp = var_field!((*call).foldExp, NFCall::TYPED_REDUCTION).clone();
            }
            Arc::new(NFCall::TYPED_REDUCTION { r#fn: var_field!((*call).r#fn, NFCall::TYPED_REDUCTION).clone(), ty: var_field!((*call).ty, NFCall::TYPED_REDUCTION).clone(), var: var_field!((*call).var, NFCall::TYPED_REDUCTION).clone(), purity: var_field!((*call).purity, NFCall::TYPED_REDUCTION).clone(), exp: e.clone(), iters: iters.clone(), defaultExp: default_exp.clone(), foldExp: fold_exp.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCall, foldArg))
}

pub fn mapFoldIteratorsExp<ArgT: Clone + 'static>(mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<(Arc<Expression::NFExpression>, ArgT)> + 'static>, mut arg: ArgT) -> Result<(Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>>, ArgT)> {
    pub type MapFunc<ArgT: Clone> = fn(Arc<Expression::NFExpression>, ArgT) -> Result<(Arc<Expression::NFExpression>, ArgT)>;

    let mut outIters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut arg: ArgT = arg;
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut new_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    for mut i in &*iters.clone() {
        let mut i = i.clone();
        (node, exp) = i.clone();
        (new_exp, arg) = Expression::mapFold(exp.clone(), func.clone(), arg.clone())?;
        outIters = cons(if (referenceEq(&new_exp.clone(),&exp.clone())) {i.clone()} else {(node.clone(), new_exp.clone())}, outIters.clone());
    }
    outIters = outIters.clone().reverse();
    Ok((outIters, arg))
}

pub fn mapFoldExpShallow<ArgT: Clone + 'static>(mut call: Arc<NFCall>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<(Arc<Expression::NFExpression>, ArgT)> + 'static>, mut foldArg: ArgT) -> Result<(Arc<NFCall>, ArgT)> {
    pub type MapFunc<ArgT: Clone> = fn(Arc<Expression::NFExpression>, ArgT) -> Result<(Arc<Expression::NFExpression>, ArgT)>;

    let mut outCall: Arc<NFCall>;
    let mut foldArg: ArgT = foldArg;
    outCall = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ UNTYPED_CALL { .. } => {
            let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            let mut nargs: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>> = metamodelica::nil();
            let mut s: ArcStr = arcstr::literal!("");
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            (args, foldArg) = List::mapFold(var_field!((*call).arguments, NFCall::UNTYPED_CALL).clone(), func.clone(), foldArg.clone());
            nargs = metamodelica::nil();
            for mut arg in &*var_field!((*call).named_args, NFCall::UNTYPED_CALL).clone() {
                let mut arg = arg.clone();
                (s, e) = arg.clone();
                (e, foldArg) = func(e.clone(), foldArg.clone())?;
                nargs = cons((s.clone(), e.clone()), nargs.clone());
            }
            Arc::new(NFCall::UNTYPED_CALL { r#ref: var_field!((*call).r#ref, NFCall::UNTYPED_CALL).clone(), arguments: args.clone(), named_args: nargs.clone().reverse(), call_scope: var_field!((*call).call_scope, NFCall::UNTYPED_CALL).clone() })
        },
        Deref @ ARG_TYPED_CALL { .. } => {
            let mut targs: Arc<metamodelica::List<Arc<TypedArg>>> = metamodelica::nil();
            let mut tnargs: Arc<metamodelica::List<Arc<TypedArg>>> = metamodelica::nil();
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            targs = metamodelica::nil();
            tnargs = metamodelica::nil();
            for mut arg in &*var_field!((*call).positional_args, NFCall::ARG_TYPED_CALL).clone() {
                let mut arg = arg.clone();
                (e, foldArg) = func(arg.value.clone(), foldArg.clone())?;
                assign_field!(arg.value = e.clone());
                targs = cons(arg.clone(), targs.clone());
            }
            for mut arg in &*var_field!((*call).named_args, NFCall::ARG_TYPED_CALL).clone() {
                let mut arg = arg.clone();
                (e, foldArg) = func(arg.value.clone(), foldArg.clone())?;
                assign_field!(arg.value = e.clone());
                targs = cons(arg.clone(), targs.clone());
            }
            Arc::new(NFCall::ARG_TYPED_CALL { r#ref: var_field!((*call).r#ref, NFCall::ARG_TYPED_CALL).clone(), positional_args: targs.clone().reverse(), named_args: tnargs.clone().reverse(), call_scope: var_field!((*call).call_scope, NFCall::ARG_TYPED_CALL).clone() })
        },
        Deref @ TYPED_CALL { .. } => {
            let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            (args, foldArg) = List::mapFold(var_field!((*call).arguments, NFCall::TYPED_CALL).clone(), func.clone(), foldArg.clone());
            Arc::new(NFCall::TYPED_CALL { r#fn: var_field!((*call).r#fn, NFCall::TYPED_CALL).clone(), ty: var_field!((*call).ty, NFCall::TYPED_CALL).clone(), var: var_field!((*call).var, NFCall::TYPED_CALL).clone(), purity: var_field!((*call).purity, NFCall::TYPED_CALL).clone(), arguments: args.clone(), attributes: var_field!((*call).attributes, NFCall::TYPED_CALL).clone() })
        },
        Deref @ UNTYPED_ARRAY_CONSTRUCTOR { .. } => {
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
            (e, foldArg) = func(var_field!((*call).exp, NFCall::UNTYPED_ARRAY_CONSTRUCTOR).clone(), foldArg.clone())?;
            (iters, _) = mapFoldIteratorsExpShallow(var_field!((*call).iters, NFCall::UNTYPED_ARRAY_CONSTRUCTOR).clone(), func.clone(), foldArg.clone());
            Arc::new(NFCall::UNTYPED_ARRAY_CONSTRUCTOR { exp: e.clone(), iters: iters.clone() })
        },
        Deref @ TYPED_ARRAY_CONSTRUCTOR { .. } => {
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
            (e, foldArg) = func(var_field!((*call).exp, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), foldArg.clone())?;
            (iters, _) = mapFoldIteratorsExpShallow(var_field!((*call).iters, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), func.clone(), foldArg.clone());
            Arc::new(NFCall::TYPED_ARRAY_CONSTRUCTOR { ty: var_field!((*call).ty, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), var: var_field!((*call).var, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), purity: var_field!((*call).purity, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), exp: e.clone(), iters: iters.clone() })
        },
        Deref @ UNTYPED_REDUCTION { .. } => {
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
            (e, foldArg) = func(var_field!((*call).exp, NFCall::UNTYPED_REDUCTION).clone(), foldArg.clone())?;
            (iters, _) = mapFoldIteratorsExpShallow(var_field!((*call).iters, NFCall::UNTYPED_REDUCTION).clone(), func.clone(), foldArg.clone());
            Arc::new(NFCall::UNTYPED_REDUCTION { r#ref: var_field!((*call).r#ref, NFCall::UNTYPED_REDUCTION).clone(), exp: e.clone(), iters: iters.clone() })
        },
        Deref @ TYPED_REDUCTION { .. } => {
            let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
            let mut default_exp: Option<Arc<Expression::NFExpression>> = None;
            let mut fold_exp: (Option<Arc<Expression::NFExpression>>, ArcStr, ArcStr);
            let mut oe: Option<Arc<Expression::NFExpression>> = None;
            (e, foldArg) = func(var_field!((*call).exp, NFCall::TYPED_REDUCTION).clone(), foldArg.clone())?;
            (iters, _) = mapFoldIteratorsExpShallow(var_field!((*call).iters, NFCall::TYPED_REDUCTION).clone(), func.clone(), foldArg.clone());
            (default_exp, foldArg) = Expression::mapFoldOptShallow(var_field!((*call).defaultExp, NFCall::TYPED_REDUCTION).clone(), func.clone(), foldArg.clone());
            oe = Util::tuple31(var_field!((*call).foldExp, NFCall::TYPED_REDUCTION).clone());
            if isSome(oe.clone()) {
                (oe, foldArg) = Expression::mapFoldOptShallow(oe.clone(), func.clone(), foldArg.clone());
                fold_exp = Util::applyTuple31(var_field!((*call).foldExp, NFCall::TYPED_REDUCTION).clone(), Arc::new({ let __pe_b1 = oe.clone(); move |__pe_a0| Ok(Util::replace(__pe_a0, __pe_b1.clone())) }));
            } else {
                fold_exp = var_field!((*call).foldExp, NFCall::TYPED_REDUCTION).clone();
            }
            Arc::new(NFCall::TYPED_REDUCTION { r#fn: var_field!((*call).r#fn, NFCall::TYPED_REDUCTION).clone(), ty: var_field!((*call).ty, NFCall::TYPED_REDUCTION).clone(), var: var_field!((*call).var, NFCall::TYPED_REDUCTION).clone(), purity: var_field!((*call).purity, NFCall::TYPED_REDUCTION).clone(), exp: e.clone(), iters: iters.clone(), defaultExp: default_exp.clone(), foldExp: fold_exp.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCall, foldArg))
}

pub fn mapFoldIteratorsExpShallow<ArgT: Clone + 'static>(mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<(Arc<Expression::NFExpression>, ArgT)> + 'static>, mut arg: ArgT) -> (Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>>, ArgT) {
    pub type MapFunc<ArgT: Clone> = fn(Arc<Expression::NFExpression>, ArgT) -> Result<(Arc<Expression::NFExpression>, ArgT)>;

    let mut outIters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut arg: ArgT = arg;
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut new_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    for mut i in &*iters.clone() {
        let mut i = i.clone();
        (node, exp) = i.clone();
        (new_exp, arg) = func(exp.clone(), arg.clone()).unwrap();
        outIters = cons(if (referenceEq(&new_exp.clone(),&exp.clone())) {i.clone()} else {(node.clone(), new_exp.clone())}, outIters.clone());
    }
    outIters = outIters.clone().reverse();
    (outIters, arg)
}

pub fn updateExternalRecordArgs(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<()> {
    for mut arg in &*args.clone() {
        let mut arg = arg.clone();
        updateExternalRecordArgsInType(Expression::typeOf(arg.clone()))?;
    }
    Ok(())
}

pub fn updateExternalRecordArgsInType(mut ty: Arc<Type::NFType>) -> Result<()> {
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut res: Arc<Restriction::NFRestriction> = Arc::new(Restriction::BLOCK);
    if Type::isRecord(ty.clone()) {
        node = Type::complexNode(ty.clone())?;
        cls = InstNode::getClass(node.clone())?;
        res = Restriction::setExternalRecord(Class::restriction(cls.clone()));
        cls = Class::setRestriction(res.clone(), cls.clone())?;
        InstNode::updateClass(cls.clone(), node.clone())?;
    }
    Ok(())
}

pub fn toArrayConstructor(mut iCall: Arc<NFCall>, mut index_ptr: Pointer::Pointer<i32>) -> Result<Arc<NFCall>> {
    let mut oCall: Arc<NFCall>;
    oCall = (::match_deref::match_deref! { match &(iCall.clone()) {
        Deref @ TYPED_CALL { .. } => {
            let mut iter_name: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
            let mut start: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut body: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut iter_range: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut step: Option<Arc<Expression::NFExpression>> = None;
            let mut rest: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            let mut iterators: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
            let mut index: i32 = 0;
            (::match_deref::match_deref! { match &(AbsynUtil::pathString(Function::nameConsiderBuiltin(var_field!((*iCall).r#fn, NFCall::TYPED_CALL).clone())?, (literal!(".")).clone(), true, false)?) {
        Deref @ "fill" => {
            index = Pointer::access(index_ptr.clone());
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(var_field!((*iCall).arguments, NFCall::TYPED_CALL).clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            body = __pa0.clone();
            rest = __pa1.clone();
            start = Arc::new(Expression::NFExpression::INTEGER { value: 1 });
            step = None;
            for mut stop in &*rest.clone().reverse() {
                let mut stop = stop.clone();
                iter_name = InstNode::newIndexedIterator(index.clone(), (literal!("f")).clone(), Absyn::dummyInfo.clone(), Arc::new(crate::NFType::INTEGER));
                iter_range = Expression::makeRange(start.clone(), step.clone(), stop.clone())?;
                iterators = cons((iter_name.clone(), iter_range.clone()), iterators.clone());
                index = index.clone() + 1;
            }
            (body, iterators) = (::match_deref::match_deref! { match &(body.clone()) {
        Deref @ Expression::CALL { call: body_call @ Deref @ TYPED_ARRAY_CONSTRUCTOR { .. } } => (var_field!((**body_call).exp, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), listAppend(iterators.clone(), var_field!((**body_call).iters, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone())),
        _ => (body.clone(), iterators.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            Pointer::update(index_ptr.clone(), index.clone());
            Arc::new(NFCall::TYPED_ARRAY_CONSTRUCTOR { ty: var_field!((*iCall).ty, NFCall::TYPED_CALL).clone(), var: var_field!((*iCall).var, NFCall::TYPED_CALL).clone(), purity: var_field!((*iCall).purity, NFCall::TYPED_CALL).clone(), exp: body.clone(), iters: iterators.clone().reverse() })
        },
        _ => iCall.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        _ => {
            let mut iterators: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
            iCall.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oCall)
}

pub fn isConnectionsOperator(mut call: Arc<NFCall>) -> Result<bool> {
    let mut isOp: bool = false;
    isOp = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ TYPED_CALL { .. } => Function::isBuiltin(var_field!((*call).r#fn, NFCall::TYPED_CALL).clone()) && AbsynUtil::pathFirstIdent(Function::name(var_field!((*call).r#fn, NFCall::TYPED_CALL).clone()))? == literal!("Connections"),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isOp)
}

pub fn isStreamOperator(mut call: Arc<NFCall>) -> bool {
    let mut isOp: bool = false;
    let mut name: ArcStr = arcstr::literal!("");
    isOp = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ TYPED_CALL { .. } if (Function::isBuiltin(var_field!((*call).r#fn, NFCall::TYPED_CALL).clone())) => {
            name = (functionNameFirst(call.clone())).clone();
            name.clone() == literal!("actualStream") || name.clone() == literal!("inStream")
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isOp
}

pub fn isCardinality(mut call: Arc<NFCall>) -> bool {
    let mut isCardinality: bool = false;
    isCardinality = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ TYPED_CALL { .. } if (Function::isBuiltin(var_field!((*call).r#fn, NFCall::TYPED_CALL).clone())) => functionNameFirst(call.clone()) == literal!("cardinality"),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isCardinality
}

fn instNormalCall(mut functionName: Arc<Absyn::ComponentRef>, mut functionArgs: Arc<Absyn::FunctionArgs>, mut scope: Arc<InstNode::InstNode>, mut context: i32, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut fn_ref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut named_args: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut name: ArcStr = arcstr::literal!("");
    name = (AbsynUtil::crefFirstIdent(functionName.clone())?).clone();
    if let Ok((__pa0, __pa1)) = instArgs(functionArgs.clone(), scope.clone(), context.clone(), info.clone()) {
        args = __pa0.clone();
        named_args = __pa1.clone();
    } else {
        if InstContext::inAnnotation(context.clone()) && !(InstContext::inInstanceAPI(context.clone())) && stringEq((name.clone()).clone(), (literal!("DynamicSelect")).clone()) {
            callExp = (::match_deref::match_deref! { match &(functionArgs.clone()) {
        Deref @ Absyn::FunctionArgs::FUNCTIONARGS { .. } => Inst::instExp(listHead(var_field!((*functionArgs).args, Absyn::FunctionArgs::FUNCTIONARGS).clone())?, scope.clone(), context.clone(), info.clone())?,
        _ => bail!("match: no arm matched"),
    } });
            return Ok(callExp);
        } else {
            bail!("fail");
        }
    }
    callExp = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "size" => BuiltinCall::makeSizeExp(args.clone(), named_args.clone(), info.clone())?,
        Deref @ "array" => BuiltinCall::makeArrayExp(args.clone(), named_args.clone(), info.clone())?,
        _ if (InstContext::inAnnotation(context.clone())) => {
            match '__try0: {
                (fn_ref, _, _) = unwrap_break_err!(Function::instFunction(functionName.clone(), InstNode::topScope(scope.clone()), context.clone(), info.clone()), '__try0);
                Ok::<_, anyhow::Error>((fn_ref.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    fn_ref = __try0_o0;
                }
                Err(_) => {
                    (fn_ref, _, _) = Function::instFunction(functionName.clone(), scope.clone(), context.clone(), info.clone())?;
                }
            }
            Arc::new(Expression::NFExpression::CALL { call: Arc::new(NFCall::UNTYPED_CALL { r#ref: fn_ref.clone(), arguments: args.clone(), named_args: named_args.clone(), call_scope: scope.clone() }) })
        },
        _ => {
            (fn_ref, _, _) = Function::instFunction(functionName.clone(), scope.clone(), context.clone(), info.clone())?;
            Arc::new(Expression::NFExpression::CALL { call: Arc::new(NFCall::UNTYPED_CALL { r#ref: fn_ref.clone(), arguments: args.clone(), named_args: named_args.clone(), call_scope: scope.clone() }) })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(callExp)
}

fn instArgs(mut args: Arc<Absyn::FunctionArgs>, mut scope: Arc<InstNode::InstNode>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<metamodelica::List<Arc<Expression::NFExpression>>>, Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>>)> {
    let mut posArgs: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut namedArgs: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    (posArgs, namedArgs) = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ Absyn::FunctionArgs::FUNCTIONARGS { .. } => {
            posArgs = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut a in (var_field!((*args).args, Absyn::FunctionArgs::FUNCTIONARGS).clone()).into_iter().cloned() {
            let __x = Inst::instExp(a.clone(), scope.clone(), context.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            namedArgs = {
        let mut __acc: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>> = metamodelica::nil();
        for mut a in (var_field!((*args).argNames, Absyn::FunctionArgs::FUNCTIONARGS).clone()).into_iter().cloned() {
            let __x = instNamedArg(a.clone(), scope.clone(), context.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            (posArgs.clone(), namedArgs.clone())
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCall.instArgs")); __mm_s.push_str(&*literal!(" got unknown function args")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((posArgs, namedArgs))
}

fn instNamedArg(mut absynArg: Arc<Absyn::NamedArg>, mut scope: Arc<InstNode::InstNode>, mut context: i32, mut info: SourceInfo) -> Result<(ArcStr, Arc<Expression::NFExpression>)> {
    let mut arg: (ArcStr, Arc<Expression::NFExpression>);
    let mut name: ArcStr = arcstr::literal!("");
    let mut exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(absynArg.clone()) {
        Deref @ Absyn::NamedArg { argValue: __pa0, argName: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    exp = __pa0.clone();
    name = __pa1.clone();
    arg = (name.clone(), Inst::instExp(exp.clone(), scope.clone(), context.clone(), info.clone())?);
    Ok(arg)
}

fn instIteratorCall(mut functionName: Arc<Absyn::ComponentRef>, mut functionArgs: Arc<Absyn::FunctionArgs>, mut scope: Arc<InstNode::InstNode>, mut context: i32, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
    let mut callExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut fn_name: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut fn_ref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut is_array: bool = false;
    fn_name = (::match_deref::match_deref! { match &(functionName.clone()) {
        Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "$array", .. } => Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("array")).clone(), subscripts: metamodelica::nil() }),
        _ => functionName.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (exp, iters) = instIteratorCallArgs(functionArgs.clone(), scope.clone(), context.clone(), info.clone())?;
    if AbsynUtil::crefFirstIdent(fn_name.clone())? == literal!("array") {
        callExp = Arc::new(Expression::NFExpression::CALL { call: Arc::new(NFCall::UNTYPED_ARRAY_CONSTRUCTOR { exp: exp.clone(), iters: iters.clone() }) });
    } else {
        (fn_ref, _, _) = Function::instFunction(fn_name.clone(), scope.clone(), context.clone(), info.clone())?;
        callExp = Arc::new(Expression::NFExpression::CALL { call: Arc::new(NFCall::UNTYPED_REDUCTION { r#ref: fn_ref.clone(), exp: exp.clone(), iters: iters.clone() }) });
    }
    Ok(callExp)
}

fn instIteratorCallArgs(mut args: Arc<Absyn::FunctionArgs>, mut scope: Arc<InstNode::InstNode>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>>)> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let _ = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ Absyn::FunctionArgs::FOR_ITER_FARG { .. } => {
            let mut for_scope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
            (for_scope, iters) = instIterators(var_field!((*args).iterators, Absyn::FunctionArgs::FOR_ITER_FARG).clone(), scope.clone(), context.clone(), info.clone())?;
            exp = Inst::instExp(var_field!((*args).exp, Absyn::FunctionArgs::FOR_ITER_FARG).clone(), for_scope.clone(), context.clone(), info.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((exp, iters))
}

fn instIterators(mut inIters: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>, mut scope: Arc<InstNode::InstNode>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<InstNode::InstNode>, Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>>)> {
    let mut outScope: Arc<InstNode::InstNode> = scope.clone();
    let mut outIters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut range: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut iter: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut range_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    for mut i in &*inIters.clone().reverse() {
        let mut i = i.clone();
        if isSome(i.range.clone()) {
            range = Inst::instExp(Util::getOption(i.range.clone())?, outScope.clone(), context.clone(), info.clone())?;
        } else {
            range = Arc::new(Expression::NFExpression::EMPTY { ty: Arc::new(crate::NFType::UNKNOWN) });
        }
        ty = (::match_deref::match_deref! { match &(range.clone()) {
        Deref @ Expression::CREF { cref: Deref @ ComponentRef::CREF { node: range_node, .. }, .. } if (InstNode::isComponent(range_node.clone())) => Arc::new(Type::NFType::COMPLEX { cls: Component::classInstance(InstNode::component(range_node.clone())?), complexTy: Arc::new(crate::NFComplexType::CLASS) }),
        _ => Arc::new(crate::NFType::UNKNOWN),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        (outScope, iter) = Inst::addIteratorToScope((i.name.clone()).clone(), outScope.clone(), info.clone(), ty.clone())?;
        outIters = cons((iter.clone(), range.clone()), outIters.clone());
    }
    Ok((outScope, outIters))
}

fn typeArrayConstructor(mut call: Arc<NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<NFCall>, Arc<Type::NFType>, Variability, Purity)> {
    let mut call: Arc<NFCall> = call;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut variability: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::PURE;
    let mut arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut range: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut iter_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut iter_var: Variability = Variability::CONSTANT;
    let mut exp_var: Variability = Variability::CONSTANT;
    let mut iter_pur: Purity = Purity::PURE;
    let mut exp_pur: Purity = Purity::PURE;
    let mut iter: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut next_context: i32 = 0;
    let mut is_structural: bool = false;
    (call, ty, variability, purity) = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ UNTYPED_ARRAY_CONSTRUCTOR { .. } => {
            variability = Variability::CONSTANT.clone();
            purity = Purity::PURE.clone();
            is_structural = !(InstContext::inFunction(context.clone()));
            next_context = InstContext::set(context.clone(), InstContext::SUBEXPRESSION.clone());
            for mut i in &*var_field!((*call).iters, NFCall::UNTYPED_ARRAY_CONSTRUCTOR).clone().reverse() {
                let mut i = i.clone();
                (iter, range) = i.clone();
                if Expression::isEmpty(range.clone()) {
                    range = Typing::deduceIterationRangeExp(Arc::new(Expression::NFExpression::CALL { call: call.clone() }), iter.clone(), info.clone())?;
                }
                (range, iter_ty, iter_var, iter_pur) = Typing::typeIterator(iter.clone(), range.clone(), next_context.clone(), is_structural.clone())?;
                if is_structural.clone() {
                    if InstContext::inRelaxed(context.clone()) {
                        range = Ceval::tryEvalExp(range.clone(), Ceval::noTarget().clone());
                    } else {
                        range = Ceval::evalExp(range.clone(), Ceval::EvalTarget::new(info.clone(), InstContext::ITERATION_RANGE.clone(), None))?;
                    }
                    iter_ty = Expression::typeOf(range.clone());
                }
                dims = List::append_reverse(Type::arrayDims(iter_ty.clone()), dims.clone());
                variability = Prefixes::variabilityMax(variability.clone(), iter_var.clone());
                purity = Prefixes::purityMin(purity.clone(), iter_pur.clone());
                iters = cons((iter.clone(), range.clone()), iters.clone());
            }
            dims = dims.clone().reverse();
            next_context = InstContext::set(next_context.clone(), InstContext::FOR.clone());
            (arg, ty, exp_var, exp_pur) = Typing::typeExp(var_field!((*call).exp, NFCall::UNTYPED_ARRAY_CONSTRUCTOR).clone(), next_context.clone(), info.clone(), false)?;
            variability = Prefixes::variabilityMax(variability.clone(), exp_var.clone());
            purity = Prefixes::purityMin(purity.clone(), exp_pur.clone());
            ty = Type::liftArrayLeftList(ty.clone(), dims.clone());
            (Arc::new(NFCall::TYPED_ARRAY_CONSTRUCTOR { ty: ty.clone(), var: variability.clone(), purity: purity.clone(), exp: arg.clone(), iters: iters.clone() }), ty.clone(), variability.clone(), purity.clone())
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCall.typeArrayConstructor")); __mm_s.push_str(&*literal!(" got invalid function call expression")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((call, ty, variability, purity))
}

fn typeReduction(mut call: Arc<NFCall>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<NFCall>, Arc<Type::NFType>, Variability, Purity)> {
    let mut call: Arc<NFCall> = call;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut variability: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::PURE;
    let mut range: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut default_exp: Option<Arc<Expression::NFExpression>> = None;
    let mut fold_exp: Option<Arc<Expression::NFExpression>> = None;
    let mut iter: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut iter_var: Variability = Variability::CONSTANT;
    let mut exp_var: Variability = Variability::CONSTANT;
    let mut iter_pur: Purity = Purity::PURE;
    let mut exp_pur: Purity = Purity::PURE;
    let mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut next_context: i32 = 0;
    let mut r#fn: Arc<Function::Function>;
    let mut fold_id: ArcStr = arcstr::literal!("");
    let mut res_id: ArcStr = arcstr::literal!("");
    let mut fold_tuple: (Option<Arc<Expression::NFExpression>>, ArcStr, ArcStr);
    (call, ty, variability, purity) = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ UNTYPED_REDUCTION { .. } => {
            variability = Variability::CONSTANT.clone();
            purity = Purity::PURE.clone();
            next_context = InstContext::set(context.clone(), InstContext::SUBEXPRESSION.clone());
            for mut i in &*var_field!((*call).iters, NFCall::UNTYPED_REDUCTION).clone().reverse() {
                let mut i = i.clone();
                (iter, range) = i.clone();
                if Expression::isEmpty(range.clone()) {
                    range = Typing::deduceIterationRangeExp(Arc::new(Expression::NFExpression::CALL { call: call.clone() }), iter.clone(), info.clone())?;
                }
                (range, _, iter_var, iter_pur) = Typing::typeIterator(iter.clone(), range.clone(), context.clone(), false)?;
                variability = Prefixes::variabilityMax(variability.clone(), iter_var.clone());
                purity = Prefixes::purityMin(purity.clone(), iter_pur.clone());
                iters = cons((iter.clone(), range.clone()), iters.clone());
            }
            next_context = InstContext::set(next_context.clone(), InstContext::FOR.clone());
            (arg, ty, exp_var, exp_pur) = Typing::typeExp(var_field!((*call).exp, NFCall::UNTYPED_REDUCTION).clone(), next_context.clone(), info.clone(), false)?;
            variability = Prefixes::variabilityMax(variability.clone(), exp_var.clone());
            purity = Prefixes::purityMin(purity.clone(), exp_pur.clone());
            let __pa0 = ::match_deref::match_deref! { match &(Function::typeRefCache(var_field!((*call).r#ref, NFCall::UNTYPED_REDUCTION).clone(), InstContext::FUNCTION.clone())?) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            r#fn = __pa0.clone();
            TypeCheck::checkReductionType(ty.clone(), Function::name(r#fn.clone()), var_field!((*call).exp, NFCall::UNTYPED_REDUCTION).clone(), info.clone())?;
            (makeTypedReduction(r#fn.clone(), ty.clone(), variability.clone(), purity.clone(), arg.clone(), iters.clone(), info.clone())?, ty.clone(), variability.clone(), purity.clone())
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCall.typeReduction")); __mm_s.push_str(&*literal!(" got invalid reduction call")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((call, ty, variability, purity))
}

pub fn makeTypedReduction(mut r#fn: Arc<Function::Function>, mut ty: Arc<Type::NFType>, mut var: Variability, mut purity: Purity, mut arg: Arc<Expression::NFExpression>, mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>>, mut info: SourceInfo) -> Result<Arc<NFCall>> {
    let mut call: Arc<NFCall>;
    let mut fold_id: ArcStr = arcstr::literal!("");
    let mut res_id: ArcStr = arcstr::literal!("");
    let mut default_exp: Option<Arc<Expression::NFExpression>> = None;
    let mut fold_exp: Option<Arc<Expression::NFExpression>> = None;
    let mut fold_tuple: (Option<Arc<Expression::NFExpression>>, ArcStr, ArcStr);
    fold_id = (Util::getTempVariableIndex()).clone();
    res_id = (Util::getTempVariableIndex()).clone();
    default_exp = reductionDefaultValue(r#fn.clone(), ty.clone())?;
    fold_exp = reductionFoldExpression(r#fn.clone(), ty.clone(), var.clone(), purity.clone(), (fold_id.clone()).clone(), (res_id.clone()).clone(), info.clone())?;
    fold_tuple = (fold_exp.clone(), fold_id.clone(), res_id.clone());
    call = Arc::new(NFCall::TYPED_REDUCTION { r#fn: r#fn.clone(), ty: ty.clone(), var: var.clone(), purity: purity.clone(), exp: arg.clone(), iters: iters.clone(), defaultExp: default_exp.clone(), foldExp: fold_tuple.clone() });
    Ok(call)
}

fn reductionDefaultValue(mut r#fn: Arc<Function::Function>, mut ty: Arc<Type::NFType>) -> Result<Option<Arc<Expression::NFExpression>>> {
    let mut defaultValue: Option<Arc<Expression::NFExpression>> = None;
    if Type::isArray(ty.clone()) {
        defaultValue = None;
    } else {
        defaultValue = (::match_deref::match_deref! { match &(AbsynUtil::pathFirstIdent(Function::name(r#fn.clone()))?) {
        Deref @ "sum" => Some(Expression::makeZero(ty.clone())?),
        Deref @ "product" => Some(Expression::makeOne(ty.clone())?),
        Deref @ "min" => Some(Expression::makeMaxValue(ty.clone())?),
        Deref @ "max" => Some(Expression::makeMinValue(ty.clone())?),
        _ => {
            Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCall.reductionDefaultValue")); __mm_s.push_str(&*literal!(" got unknown reduction name ")); __mm_s.push_str(&*AbsynUtil::pathFirstIdent(Function::name(r#fn.clone()))?); ArcStr::from(__mm_s) }).clone()], metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(defaultValue)
}

fn reductionFoldExpression(mut reductionFn: Arc<Function::Function>, mut reductionType: Arc<Type::NFType>, mut reductionVar: Variability, mut reductionPurity: Purity, mut foldId: ArcStr, mut resultId: ArcStr, mut info: SourceInfo) -> Result<Option<Arc<Expression::NFExpression>>> {
    let mut foldExp: Option<Arc<Expression::NFExpression>> = None;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut op_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut r#fn: Arc<Function::Function>;
    if Type::isComplex(reductionType.clone()) {
        foldExp = (::match_deref::match_deref! { match &(AbsynUtil::pathFirstIdent(Function::name(reductionFn.clone()))?) {
        Deref @ "sum" => {
            let __pa0 = ::match_deref::match_deref! { match &(reductionType.clone()) {
                Deref @ Type::COMPLEX { cls: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            op_node = __pa0.clone();
            (op_node, _) = Class::lookupElement((literal!("'+'")).clone(), InstNode::getClass(op_node.clone())?)?;
            Function::instFunctionNode(op_node.clone(), InstContext::NO_CONTEXT.clone(), info.clone())?;
            let __pa1 = ::match_deref::match_deref! { match &(Function::typeNodeCache(op_node.clone(), InstContext::FUNCTION.clone())?) {
                Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } => __pa1.clone(),
                _ => bail!("pattern mismatch"),
            } };
            r#fn = __pa1.clone();
            Some(Arc::new(Expression::NFExpression::CALL { call: makeTypedCall(r#fn.clone(), list![reductionFoldIterator((resultId.clone()).clone(), reductionType.clone()), reductionFoldIterator((foldId.clone()).clone(), reductionType.clone())], reductionVar.clone(), reductionPurity.clone(), r#fn.returnType.clone()) }))
        },
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    } else {
        foldExp = (::match_deref::match_deref! { match &(AbsynUtil::pathFirstIdent(Function::name(reductionFn.clone()))?) {
        Deref @ "sum" => Some(Arc::new(Expression::NFExpression::BINARY { exp1: reductionFoldIterator((resultId.clone()).clone(), reductionType.clone()), operator: Operator::makeAdd(reductionType.clone()), exp2: reductionFoldIterator((foldId.clone()).clone(), reductionType.clone()) })),
        Deref @ "product" => Some(Arc::new(Expression::NFExpression::BINARY { exp1: reductionFoldIterator((resultId.clone()).clone(), reductionType.clone()), operator: Operator::makeMul(reductionType.clone()), exp2: reductionFoldIterator((foldId.clone()).clone(), reductionType.clone()) })),
        Deref @ "$array" => None,
        Deref @ "array" => None,
        Deref @ "list" => None,
        Deref @ "listReverse" => None,
        _ => Some(Arc::new(Expression::NFExpression::CALL { call: makeTypedCall(reductionFn.clone(), list![reductionFoldIterator((foldId.clone()).clone(), reductionType.clone()), reductionFoldIterator((resultId.clone()).clone(), reductionType.clone())], reductionVar.clone(), reductionPurity.clone(), reductionType.clone()) })),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(foldExp)
}

fn reductionFoldIterator(mut name: ArcStr, mut ty: Arc<Type::NFType>) -> Arc<Expression::NFExpression> {
    let mut iterExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    iterExp = Arc::new(Expression::NFExpression::CREF { ty: ty.clone(), cref: ComponentRef::makeIterator(Arc::new(InstNode::InstNode::NAME_NODE { name: (name.clone()).clone() }), ty.clone()) });
    iterExp
}

fn typeArgs(mut call: Arc<NFCall>, mut context: i32, mut info: SourceInfo) -> Result<Arc<NFCall>> {
    let mut call: Arc<NFCall> = call;
    call = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ UNTYPED_CALL { .. } => {
            let mut arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut arg_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut arg_var: Variability = Variability::CONSTANT;
            let mut arg_pur: Purity = Purity::PURE;
            let mut typed_args: Arc<metamodelica::List<Arc<TypedArg>>> = metamodelica::nil();
            let mut typed_nargs: Arc<metamodelica::List<Arc<TypedArg>>> = metamodelica::nil();
            let mut name: ArcStr = arcstr::literal!("");
            let mut next_context: i32 = 0;
            typed_args = metamodelica::nil();
            next_context = InstContext::set(context.clone(), InstContext::SUBEXPRESSION.clone());
            for mut arg in &*var_field!((*call).arguments, NFCall::UNTYPED_CALL).clone() {
                let mut arg = arg.clone();
                (arg, arg_ty, arg_var, arg_pur) = Typing::typeExp(arg.clone(), next_context.clone(), info.clone(), false)?;
                typed_args = cons(Arc::new(TypedArg { name: None, value: arg.clone(), ty: arg_ty.clone(), var: arg_var.clone(), purity: arg_pur.clone() }), typed_args.clone());
            }
            typed_args = typed_args.clone().reverse();
            typed_nargs = metamodelica::nil();
            for mut narg in &*var_field!((*call).named_args, NFCall::UNTYPED_CALL).clone() {
                let mut narg = narg.clone();
                (name, arg) = narg.clone();
                (arg, arg_ty, arg_var, arg_pur) = Typing::typeExp(arg.clone(), next_context.clone(), info.clone(), false)?;
                typed_nargs = cons(Arc::new(TypedArg { name: Some((name.clone()).clone()), value: arg.clone(), ty: arg_ty.clone(), var: arg_var.clone(), purity: arg_pur.clone() }), typed_nargs.clone());
            }
            typed_nargs = typed_nargs.clone().reverse();
            Arc::new(NFCall::ARG_TYPED_CALL { r#ref: var_field!((*call).r#ref, NFCall::UNTYPED_CALL).clone(), positional_args: typed_args.clone(), named_args: typed_nargs.clone(), call_scope: var_field!((*call).call_scope, NFCall::UNTYPED_CALL).clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(call)
}

fn checkMatchingFunctions(mut call: Arc<NFCall>, mut context: i32, mut info: SourceInfo, mut vectorize: bool) -> Result<Arc<MatchedFunction::MatchedFunction>> {
    let mut matchedFunc: Arc<MatchedFunction::MatchedFunction>;
    let mut matchedFunctions: Arc<metamodelica::List<Arc<MatchedFunction::MatchedFunction>>> = metamodelica::nil();
    let mut exactMatches: Arc<metamodelica::List<Arc<MatchedFunction::MatchedFunction>>> = metamodelica::nil();
    let mut func: Arc<Function::Function>;
    let mut allfuncs: Arc<metamodelica::List<Arc<Function::Function>>> = metamodelica::nil();
    let mut fn_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut numerr: i32 = Error::getNumErrorMessages();
    let mut errors: Arc<metamodelica::List<i32>> = metamodelica::nil();
    ErrorExt::setCheckpoint((literal!("NFCall:checkMatchingFunctions")).clone());
    matchedFunctions = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ ARG_TYPED_CALL { r#ref: Deref @ ComponentRef::CREF { node: fn_node, .. }, .. } => {
            allfuncs = Function::getCachedFuncs(fn_node.clone())?;
            if (allfuncs.clone().len() as i32) > 1 {
                allfuncs = {
        let mut __acc: Arc<metamodelica::List<Arc<Function::Function>>> = metamodelica::nil();
        for mut r#fn in (allfuncs.clone()).into_iter().cloned() {
            if !(!(Function::isDefaultRecordConstructor(r#fn.clone()))) { continue; }
            let __x = r#fn.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            }
            Function::matchFunctions(allfuncs.clone(), var_field!((*call).positional_args, NFCall::ARG_TYPED_CALL).clone(), var_field!((*call).named_args, NFCall::ARG_TYPED_CALL).clone(), context.clone(), info.clone(), vectorize.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    if matchedFunctions.clone().is_empty() {
        if (allfuncs.clone().len() as i32) > 1 {
            ErrorExt::rollBack((literal!("NFCall:checkMatchingFunctions")).clone());
            Error::addSourceMessage(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone(), list![(typedString(call.clone())?).clone(), (Function::candidateFuncListString(allfuncs.clone())).clone()], info.clone())?;
        } else if numerr.clone() == Error::getNumErrorMessages() {
            ErrorExt::rollBack((literal!("NFCall:checkMatchingFunctions")).clone());
            Error::addSourceMessage(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone(), list![(typedString(call.clone())?).clone(), (Function::candidateFuncListString(allfuncs.clone())).clone()], info.clone())?;
        } else {
            ErrorExt::delCheckpoint((literal!("NFCall:checkMatchingFunctions")).clone());
        }
        bail!("fail");
    }
    ErrorExt::rollBack((literal!("NFCall:checkMatchingFunctions")).clone());
    if (matchedFunctions.clone().len() as i32) > 1 {
        exactMatches = MatchedFunction::getExactMatches(matchedFunctions.clone());
        if exactMatches.clone().is_empty() {
            exactMatches = MatchedFunction::getExactVectorizedMatches(matchedFunctions.clone());
        }
        if (exactMatches.clone().len() as i32) > 1 {
            Error::addSourceMessage(Error::AMBIGUOUS_MATCHING_FUNCTIONS_NFINST.clone(), list![(typedString(call.clone())?).clone(), (Function::candidateFuncListString({
        let mut __acc: Arc<metamodelica::List<Arc<Function::Function>>> = metamodelica::nil();
        for mut mfn in (matchedFunctions.clone()).into_iter().cloned() {
            let __x = mfn.func.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })).clone()], info.clone())?;
            bail!("fail");
        }
        matchedFunc = listHead(exactMatches.clone())?;
    } else {
        matchedFunc = listHead(matchedFunctions.clone())?;
    }
    if Function::isBuiltin(matchedFunc.func.clone()) {
        func = matchedFunc.func.clone();
        assign_field!(func.path = Function::nameConsiderBuiltin(func.clone())?);
        assign_field!(matchedFunc.func = func.clone());
    }
    Ok(matchedFunc)
}

fn iteratorToDAE(mut iter: (Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)) -> Result<Arc<DAE::ReductionIterator>> {
    let mut diter: Arc<DAE::ReductionIterator>;
    let mut iter_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut iter_range: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut c: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut b: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    (iter_node, iter_range) = iter.clone();
    diter = Arc::new(DAE::ReductionIterator { id: (InstNode::name(iter_node.clone())?).clone(), exp: Expression::toDAE(iter_range.clone(), false)?, guardExp: None, ty: Type::toDAE(InstNode::getType(iter_node.clone())?, true)? });
    Ok(diter)
}

fn vectorizeCall(mut base_call: Arc<NFCall>, mut mk: Arc<FunctionMatchKind::FunctionMatchKind>, mut scope: Arc<InstNode::InstNode>, mut info: SourceInfo) -> Result<Arc<NFCall>> {
    let mut vectorized_call: Arc<NFCall>;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut vect_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut iter: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut i: i32 = 0;
    let mut call_args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut sub: Arc<Subscript::NFSubscript> = Arc::new(Subscript::WHOLE);
    vectorized_call = (::match_deref::match_deref! { match &((base_call.clone(), mk.clone())) {
        (Deref @ TYPED_CALL { arguments: call_args, .. }, Deref @ FunctionMatchKind::VECTORIZED { .. }) => {
            let mut call_args = (*call_args).clone();
            iters = metamodelica::nil();
            i = 1;
            for mut dim in &*var_field!((*mk).vectDims, FunctionMatchKind::FunctionMatchKind::VECTORIZED).clone() {
                let mut dim = dim.clone();
                Error::assertion(Dimension::isKnown(dim.clone(), true), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCall.vectorizeCall")); __mm_s.push_str(&*literal!(" got unknown dimension for vectorized call")); ArcStr::from(__mm_s) }).clone(), info.clone())?;
                ty = Arc::new(Type::NFType::ARRAY { elementType: Arc::new(crate::NFType::INTEGER), dimensions: list![dim.clone()] });
                exp = Arc::new(Expression::NFExpression::RANGE { ty: ty.clone(), start: Arc::new(Expression::NFExpression::INTEGER { value: 1 }), step: None, stop: Dimension::sizeExp(dim.clone())? });
                iter = InstNode::newUniqueIterator(info.clone(), Arc::new(crate::NFType::INTEGER));
                iters = cons((iter.clone(), exp.clone()), iters.clone());
                exp = Arc::new(Expression::NFExpression::CREF { ty: Arc::new(crate::NFType::INTEGER), cref: ComponentRef::makeIterator(iter.clone(), Arc::new(crate::NFType::INTEGER)) });
                sub = Arc::new(Subscript::NFSubscript::INDEX { index: exp.clone() });
                call_args = List::mapIndices(call_args.clone(), var_field!((*mk).vectorizedArgs, FunctionMatchKind::FunctionMatchKind::VECTORIZED).clone(), Arc::new({ let __pe_b0 = sub.clone(); let __pe_b2 = metamodelica::nil(); let __pe_b3 = false; move |__pe_a1| Expression::applySubscript(__pe_b0.clone(), __pe_a1, __pe_b2.clone(), __pe_b3.clone()) }))?;
                i = i.clone() + 1;
            }
            vect_ty = Type::liftArrayLeftList(var_field!((*base_call).ty, NFCall::TYPED_CALL).clone(), var_field!((*mk).vectDims, FunctionMatchKind::FunctionMatchKind::VECTORIZED).clone());
            assign_variant_field!(base_call => NFCall::TYPED_CALL; arguments = call_args.clone());
            Arc::new(NFCall::TYPED_ARRAY_CONSTRUCTOR { ty: vect_ty.clone(), var: var_field!((*base_call).var, NFCall::TYPED_CALL).clone(), purity: var_field!((*base_call).purity, NFCall::TYPED_CALL).clone(), exp: Arc::new(Expression::NFExpression::CALL { call: base_call.clone() }), iters: iters.clone() })
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCall.vectorizeCall")); __mm_s.push_str(&*literal!(" got unknown call")); ArcStr::from(__mm_s) }).clone(), info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(vectorized_call)
}

fn isVectorized(mut call: Arc<NFCall>) -> Result<bool> {
    let mut vectorized: bool = false;
    vectorized = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ TYPED_ARRAY_CONSTRUCTOR { exp: Deref @ Expression::CALL { .. }, .. } => stringGet((InstNode::name(Util::tuple21(listHead(var_field!((*call).iters, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone())?))?).clone(),1)? == 36,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(vectorized)
}

fn devectorizeCall(mut call: Arc<NFCall>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut iter_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    let mut iter_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(call.clone()) {
        Deref @ TYPED_ARRAY_CONSTRUCTOR { iters: __pa0, exp: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    iters = __pa0.clone();
    exp = __pa1.clone();
    for mut i in &*iters.clone() {
        let mut i = i.clone();
        (iter_node, iter_exp) = i.clone();
        exp = Expression::replaceIterator(exp.clone(), iter_node.clone(), iter_exp.clone())?;
    }
    result = SimplifyExp::simplify(exp.clone(), false)?;
    Ok(result)
}

fn evaluateCallType(mut ty: Arc<Type::NFType>, mut r#fn: Arc<Function::Function>, mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut outputIndex: i32, mut ptree: ParameterTree) -> Result<(Arc<Type::NFType>, ParameterTree)> {
    let mut ty: Arc<Type::NFType> = ty;
    let mut ptree: ParameterTree = ptree;
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    let mut tys: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut binding_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut t: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut output_index: i32 = 0;
    ty = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::ARRAY { .. } => {
            (dims, ptree) = List::mapFold(var_field!((*ty).dimensions, Type::NFType::ARRAY).clone(), Arc::new({ let __pe_b1 = r#fn.clone(); let __pe_b2 = args.clone(); move |__pe_a0, __pe_a3| evaluateCallTypeDim(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }), ptree.clone());
            assign_variant_field!(ty => Type::NFType::ARRAY; dimensions = dims.clone());
            ty.clone()
        },
        Deref @ Type::TUPLE { .. } => {
            tys = metamodelica::nil();
            output_index = 1;
            for mut t in &*var_field!((*ty).types, Type::NFType::TUPLE).clone() {
                let mut t = t.clone();
                (t, ptree) = evaluateCallType(t.clone(), r#fn.clone(), args.clone(), output_index.clone(), ptree.clone())?;
                tys = cons(t.clone(), tys.clone());
                output_index = output_index.clone() + 1;
            }
            assign_variant_field!(ty => Type::NFType::TUPLE; types = tys.clone().reverse());
            ty.clone()
        },
        Deref @ Type::COMPLEX { .. } if (Type::isRecord(ty.clone()) && !(Function::isNonDefaultRecordConstructor(r#fn.clone()))) => {
            binding = Component::getBinding(InstNode::component((r#fn.outputs.clone()).get(outputIndex.clone())?)?);
            if Binding::isBound(binding.clone()) {
                binding_exp = Binding::getExp(binding.clone())?;
                ptree = buildParameterTree(r#fn.clone(), args.clone(), ptree.clone())?;
                binding_exp = Expression::map(binding_exp.clone(), Arc::new({ let __pe_b1 = ptree.clone(); move |__pe_a0| evaluateCallTypeDimExp(__pe_a0, __pe_b1.clone()) }))?;
                t = Expression::typeOf(binding_exp.clone());
            } else {
                t = ty.clone();
            }
            t.clone()
        },
        _ => ty.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((ty, ptree))
}

fn evaluateCallTypeDim(mut dim: Arc<Dimension::NFDimension>, mut r#fn: Arc<Function::Function>, mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut ptree: ParameterTree) -> Result<(Arc<Dimension::NFDimension>, ParameterTree)> {
    let mut dim: Arc<Dimension::NFDimension> = dim;
    let mut ptree: ParameterTree = ptree;
    dim = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ Dimension::EXP { .. } => {
            let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            ptree = buildParameterTree(r#fn.clone(), args.clone(), ptree.clone())?;
            exp = Expression::map(var_field!((*dim).exp, Dimension::NFDimension::EXP).clone(), Arc::new({ let __pe_b1 = ptree.clone(); move |__pe_a0| evaluateCallTypeDimExp(__pe_a0, __pe_b1.clone()) }))?;
            ErrorExt::setCheckpoint((literal!("NFCall.evaluateCallTypeDim")).clone());
            if '__try0: {
                unwrap_break_err!(Structural::markExp(exp.clone()), '__try0);
                exp = unwrap_break_err!(Ceval::evalExp(exp.clone(), Ceval::noTarget().clone()), '__try0);
                Ok::<(), anyhow::Error>(())
            }.is_err() {
            }
            ErrorExt::rollBack((literal!("NFCall.evaluateCallTypeDim")).clone());
            Dimension::fromExp(exp.clone(), Variability::CONSTANT.clone())?
        },
        _ => {
            dim.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((dim, ptree))
}

fn buildParameterTree(mut r#fn: Arc<Function::Function>, mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut ptree: ParameterTree) -> Result<ParameterTree> {
    let mut ptree: ParameterTree = ptree;
    let mut arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut rest_args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = args.clone();
    if !(NFCallParameterTree::isEmpty(ptree.clone())) {
        return Ok(ptree);
    }
    for mut i in &*r#fn.inputs.clone() {
        let mut i = i.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_args.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        arg = __pa0.clone();
        rest_args = __pa1.clone();
        ptree = NFCallParameterTree::add(ptree.clone(), (InstNode::name(i.clone())?).clone(), arg.clone(), NFCallParameterTree::addConflictDefault)?;
    }
    Ok(ptree)
}

fn evaluateCallTypeDimExp(mut exp: Arc<Expression::NFExpression>, mut ptree: ParameterTree) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut cref_parts: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut oexp: Option<Arc<Expression::NFExpression>> = None;
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { cref: Deref @ ComponentRef::CREF { .. }, .. } => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ComponentRef::toListReverse(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), true, metamodelica::nil())) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cref = __pa0.clone();
            cref_parts = __pa1.clone();
            oexp = NFCallParameterTree::getOpt(ptree.clone(), (InstNode::name(ComponentRef::node(cref.clone())?)?).clone());
            if isSome(oexp.clone()) {
                let __pa2 = ::match_deref::match_deref! { match &(oexp.clone()) {
                    Some(__pa2) => __pa2.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                outExp = __pa2.clone();
                outExp = Expression::applySubscripts(ComponentRef::getSubscripts(cref.clone()), outExp.clone(), false)?;
                for mut cr in &*cref_parts.clone() {
                    let mut cr = cr.clone();
                    outExp = Expression::recordElement((InstNode::name(ComponentRef::node(cr.clone())?)?).clone(), outExp.clone())?;
                    outExp = Expression::applySubscripts(ComponentRef::getSubscripts(cr.clone()), outExp.clone(), false)?;
                }
            } else {
                outExp = exp.clone();
            }
            outExp.clone()
        },
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

fn resolvePolymorphicReturnType(mut r#fn: Arc<Function::Function>, mut args: Arc<metamodelica::List<Arc<TypedArg>>>, mut ty: Arc<Type::NFType>) -> Result<Arc<Type::NFType>> {
    let mut outType: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut name: ArcStr = arcstr::literal!("");
    let mut input_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut arg: Arc<TypedArg>;
    let mut rest_args: Arc<metamodelica::List<Arc<TypedArg>>> = args.clone();
    outType = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::POLYMORPHIC { name } => {
            for mut i in &*r#fn.inputs.clone() {
                let mut i = i.clone();
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_args.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                arg = __pa0.clone();
                rest_args = __pa1.clone();
                input_ty = InstNode::getType(i.clone())?;
                if Type::isPolymorphicNamed(Type::arrayElementType(input_ty.clone()), (name.clone()).clone()) {
                    outType = Type::unliftArrayN(Type::dimensionCount(input_ty.clone()), arg.ty.clone())?;
                    return Ok(outType);
                }
            }
            if name.clone() == literal!("__Scalar") {
                outType = resolvePolymorphicReturnType(r#fn.clone(), args.clone(), Arc::new(Type::NFType::POLYMORPHIC { name: (literal!("__Array")).clone() }))?;
                outType = Type::arrayElementType(outType.clone());
                return Ok(outType);
            }
            bail!("fail")
        },
        Deref @ Type::ARRAY { elementType: Deref @ Type::POLYMORPHIC { .. }, .. } => {
            assign_variant_field!(ty => Type::NFType::ARRAY; elementType = resolvePolymorphicReturnType(r#fn.clone(), args.clone(), var_field!((*ty).elementType, Type::NFType::ARRAY).clone())?);
            ty.clone()
        },
        _ => ty.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outType)
}


